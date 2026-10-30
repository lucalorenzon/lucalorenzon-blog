# Run the screen reader check in a container

- **Opened:** 2026-10-30, asked by the user while the card
  [run a screen reader check on demand](../done/2026-10-30_run-screen-reader-check-on-demand.md)
  was being worked on.
- **Status:** being worked on, since 2026-10-30.

## Why

The on-demand screen reader check works only on the user's machine, and
is fragile there. It depends on GNOME on Wayland, Orca 50 and its D-Bus
remote control, the Firefox snap (with its profile in the snap's own
folder), a click to give the window the focus, the user's default voice
(English, which ends up in the transcript as the language of Orca's own
words), and the format of Orca's debug file. On another machine the
transcript would differ with the same page, so the comparison means
nothing there.

The user wants the check to work on any computer. Not urgent: the local
command stays until this card replaces it.

## Idea to try

A Docker image with a virtual screen, the accessibility bus, Orca,
Firefox and a fixed synthetic voice (for example espeak-ng through
Speech Dispatcher). The transcript would be the same on any machine with
Docker, and might even be compared in the build on GitHub. The audio
would be written to a file, so the user can listen to it anywhere and
judge the pronunciation.

## Questions

- Whether Orca runs without a real desktop session: a timeboxed trial,
  as for the local command, before any design.
- Whether the audio can be written to a file the user plays afterwards,
  and how the user's answers are then recorded.
- Whether the transcript comparison can run in the build, and what that
  costs.
- What happens to the local command: replaced, or kept for checks with
  the user's own voice and setup.

## Trial: does Orca read the page in a container? Yes

Agreed with the user on 2026-10-30: one hour at most, and the trial
succeeds if Orca reads the site's page inside the container. Voice,
audio file and the build comparison stay out of it: they are the other
questions above. Done in three runs and about 17 minutes (08:37 to
08:47, paused by the user, then 16:32 to 16:39).

Set-up, chosen for the trial only: Debian trixie, where Firefox is an
ordinary package (on Ubuntu it is a snap, which does not work in a
container); a virtual screen (Xvfb) instead of GNOME; a session bus with
the accessibility bus; the openbox window manager, so Firefox gets the
focus without a click; PulseAudio with a null sink; Orca started before
Firefox, as in the local command. The site is built on the host and
mounted read-only. Versions: Orca 48.1, Firefox 153.6.0esr,
speech-dispatcher 0.12.0. The files are in the appendix below.

### Runs

1. **Orca announced the page, then went silent.** It said the window
   title and "Page has 3 landmarks, 1 heading.", nothing more, and did
   not quit on SIGTERM.
2. **Diagnosis: Orca was stuck on the voice.** With Python's
   faulthandler on, SIGABRT showed Orca's main thread waiting for
   speech-dispatcher to answer, and speech-dispatcher itself waiting:
   the container has no sound device. Rejected explanation: that Orca
   went on reading and only the end of its debug file was lost.
3. **With a null audio sink, Orca read the page whole.** PulseAudio
   started before Orca, with a null sink as default output, which takes
   the sound and drops it.

### Result

The reading has the same structure as the reference taken on the
user's desktop (`docs/accessibility/screen-reader-transcript.txt`):
the landmark summary, banner, the logo, image, "Home Page", heading 1,
leaving banner, main content, "Sito in costruzione", leaving main
content, information, "© Luca Lorenzon 2026". Orca asks for an Italian
voice for the page's Italian text. The differences come from Orca 48
against Orca 50 on the desktop:

- no "Finished loading" announcement before the reading;
- the logo is sent to the voice as "Lvk@73r", not "Lvk @ 73r";
- the debug file has another format (`Speaking '...' as string`, not
  SSML), so the local command's parser does not read it as it is.

Also observed: Orca 48 does not quit on SIGTERM even when idle, and has
no D-Bus service to quit it. This time the debug file held the whole
reading anyway, but a reliable way to stop Orca and flush its file is
still to be found.

## Which reference, which Orca

### Which reading is the reference? The container's (decided by the user)

The reference is the reading produced in the container, no longer the
one taken on the user's desktop. Rejected: making the container match
the desktop exactly (Orca 50.3), which would tie the image to the
user's own system and its updates.

Open consequence: the local command compares the desktop reading with
`docs/accessibility/screen-reader-transcript.txt`; once the reference
comes from the container, the two readings differ by Orca version
alone. What happens to the local command is still the open question
above.

### Which Orca? 51.1 from trixie-backports (decided by the user)

Debian has no Orca 50 for trixie: trixie has 48.1, trixie-backports
(an official Debian repository) has 51.1, as do testing and unstable
(checked on 2026-10-30 in the Debian archive). The user chose 51.1 from
the backports, so the image stays on Debian stable. Rejected: Ubuntu
for the exact 50 (Firefox there is a snap; it would need Mozilla's own
repository); building Orca from source (most work).

Orca 51 pulls speech-dispatcher 0.12.1 and at-spi2-core 2.62 from the
backports; the espeak-ng module of speech-dispatcher must then come
from the backports too, or apt refuses the mix (the first build
failed on that).

### Run with Orca 51.1

- Orca read the page whole and **quit cleanly on SIGTERM** ("Screen
  reader off."), which solves Orca 48's quit problem.
- The debug file is back to Orca 50's format (SSML), so the local
  command's parser should read it.
- Still no "Finished loading" announcement before the reading.
- **The logo is read as "code start", "code end", without its name**,
  where Orca 50 says "Lvk @ 73r", "image". Recorded by the user as a
  card in Now:
  [logo name not read by Orca 51](../backlog/logo-name-not-read-by-orca-51.md).
- Orca spends about seven seconds looking for a braille display that is
  not there before it goes on reading.
- Harmless warnings: the `ScreenReaderEnabled` property no longer
  exists on the accessibility bus 2.62; Firefox, built for trixie's
  libraries, warns about a caret event of the backported ones.

## Appendix: trial files

These are throwaway trial files, kept here so they survive the session.
How to run them: build the image from a folder holding the two files,
then run it with the built site mounted at `/site` (read-only) and an
empty folder writable by uid 1000 at `/out`, where `<repository>` is
the path of this repository. The user runs Docker; the agent has no
access to it.

```
sudo docker build -t sr-trial . 2>&1 | tee build.log
sudo docker run --rm -v <repository>/target/site:/site:ro -v "$PWD/out":/out sr-trial 2>&1 | tee run.log
```

`Dockerfile`:

````dockerfile
FROM debian:trixie-slim

# Orca comes from trixie-backports (51.x; trixie itself has 48.1).
# Backports are used only for the packages asked for with -t. Orca 51
# pulls speech-dispatcher 0.12.1 from there, so its espeak-ng module must
# come from there too (the trixie one wants exactly 0.12.0).
RUN echo 'deb http://deb.debian.org/debian trixie-backports main' \
      > /etc/apt/sources.list.d/backports.list \
 && apt-get update \
 && apt-get install -y --no-install-recommends -t trixie-backports \
      orca speech-dispatcher speech-dispatcher-espeak-ng \
 && apt-get install -y --no-install-recommends \
      firefox-esr xvfb xauth dbus dbus-x11 at-spi2-core espeak-ng \
      pulseaudio pulseaudio-utils \
      openbox xdotool procps libglib2.0-bin fonts-dejavu-core ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Same uid as the host user, so /out is writable.
RUN groupadd --gid 1001 reader \
 && useradd --uid 1000 --gid 1001 --create-home --home-dir /home/reader --shell /bin/bash reader

COPY run-trial.sh /usr/local/bin/run-trial.sh
RUN chmod 755 /usr/local/bin/run-trial.sh

USER reader
ENV HOME=/home/reader
# Site mounted read-only at /site, output dir at /out.
ENTRYPOINT ["/usr/local/bin/run-trial.sh"]
````

`run-trial.sh`:

````bash
#!/usr/bin/env bash
# Trial: can Orca read a static page in Firefox inside a container?
# Run with no argument: starts Xvfb, then re-invokes itself as "inner"
# under dbus-run-session.
set -u

WM=${WM:-1}
FOCUS=${FOCUS:-1}
ORCA_WAIT=${ORCA_WAIT:-8}
READ_SECONDS=${READ_SECONDS:-45}
OUT=/out

step() { printf '[%s] %s\n' "$(date +%H:%M:%S)" "$*"; }

inner() {
  export GTK_A11Y=atspi NO_AT_BRIDGE=0 GNOME_ACCESSIBILITY=1 MOZ_ENABLE_WAYLAND=0

  step "start accessibility bus launcher"
  launcher=/usr/libexec/at-spi-bus-launcher
  if [ ! -x "$launcher" ]; then
    step "$launcher missing; searching"
    found=$(find /usr/lib* -path '*at-spi2-core*' -name 'at-spi-bus-launcher' 2>/dev/null | head -1)
    step "found: ${found:-nothing}"
    launcher=${found:-}
  fi
  [ -n "$launcher" ] && "$launcher" --launch-immediately &
  bus_pid=$!
  sleep 2

  step "turn accessibility on"
  gsettings set org.gnome.desktop.interface toolkit-accessibility true || step "gsettings failed (ignored)"
  for prop in IsEnabled ScreenReaderEnabled; do
    gdbus call --session --dest org.a11y.Bus --object-path /org/a11y/bus \
      --method org.freedesktop.DBus.Properties.Set org.a11y.Status "$prop" "<true>" 2>&1
    printf '%s = ' "$prop"
    gdbus call --session --dest org.a11y.Bus --object-path /org/a11y/bus \
      --method org.freedesktop.DBus.Properties.Get org.a11y.Status "$prop" 2>&1
  done

  wm_pid=""
  if [ "$WM" = 1 ]; then
    step "start openbox"
    openbox >/out/openbox.log 2>&1 &
    wm_pid=$!
  fi

  # Without a sound device speech-dispatcher hangs, and Orca with it:
  # give it a null sink that accepts the sound and drops it.
  step "start pulseaudio with a null sink"
  pulseaudio --daemonize=yes --exit-idle-time=-1 --log-target=file:"$OUT/pulseaudio.log" \
    || step "pulseaudio failed to start"
  pactl load-module module-null-sink sink_name=null_out >/dev/null \
    && pactl set-default-sink null_out && step "null sink is the default" \
    || step "null sink setup failed"

  step "start orca (before Firefox)"
  # faulthandler: on SIGABRT, Python prints every thread's stack to stderr.
  python3 -X faulthandler /usr/bin/orca --debug-file "$OUT/orca-debug.log" >"$OUT/orca-stderr.log" 2>&1 &
  orca_pid=$!
  sleep "$ORCA_WAIT"
  if kill -0 "$orca_pid" 2>/dev/null; then step "orca alive (pid $orca_pid)"; else step "orca NOT alive"; fi

  step "start firefox"
  profile=$(mktemp -d)
  cat >"$profile/user.js" <<'JS'
user_pref("accessibility.force_disabled", 0);
user_pref("browser.shell.checkDefaultBrowser", false);
user_pref("browser.aboutwelcome.enabled", false);
user_pref("startup.homepage_welcome_url", "");
user_pref("startup.homepage_welcome_url.additional", "");
user_pref("browser.startup.homepage_override.mstone", "ignore");
user_pref("datareporting.policy.dataSubmissionEnabled", false);
user_pref("datareporting.healthreport.uploadEnabled", false);
user_pref("datareporting.policy.firstRunURL", "");
user_pref("toolkit.telemetry.enabled", false);
user_pref("toolkit.telemetry.reportingpolicy.firstRun", false);
user_pref("browser.tabs.warnOnClose", false);
JS
  firefox-esr --new-instance --no-remote --profile "$profile" file:///site/index.html \
    >/dev/null 2>"$OUT/firefox-stderr.log" &
  ff_pid=$!

  if [ "$FOCUS" = 1 ]; then
    step "focus firefox window"
    if timeout 30 xdotool search --sync --onlyvisible --class firefox windowactivate; then
      step "focus ok"
    else
      step "focus failed or timed out"
    fi
  fi

  step "wait ${READ_SECONDS}s for Orca to read"
  sleep "$READ_SECONDS"

  step "processes before quitting"
  ps -eo pid,ppid,stat,wchan:24,etime,time,args | grep -vE ' ps | grep ' | cut -c1-200

  # A clean quit flushes the debug file. If Orca does not quit, SIGABRT
  # makes it print where it is stuck into orca-stderr.log.
  if gdbus call --session --dest org.gnome.Orca.Service --object-path /org/gnome/Orca/Service \
       --method org.gnome.Orca.Service.Quit >/dev/null 2>&1; then
    step "quit orca (D-Bus quit sent)"
  else
    step "quit orca (no D-Bus service; SIGTERM)"
    kill -TERM "$orca_pid" 2>/dev/null
  fi
  for _ in $(seq 10); do kill -0 "$orca_pid" 2>/dev/null || break; sleep 1; done
  if kill -0 "$orca_pid" 2>/dev/null; then
    step "orca still alive; SIGABRT (stack dump)"
    kill -ABRT "$orca_pid" 2>/dev/null
    sleep 3
  else
    step "orca quit"
  fi

  step "stop firefox, openbox, bus launcher"
  kill "$ff_pid" 2>/dev/null
  [ -n "$wm_pid" ] && kill "$wm_pid" 2>/dev/null
  kill "$bus_pid" 2>/dev/null
  sleep 2

  mkdir -p "$OUT/speechd"
  cp -r "$HOME"/.cache/speech-dispatcher/log/* "$OUT/speechd/" 2>/dev/null || step "no speech-dispatcher log"
  step "inner done"
}

if [ "${1:-}" = inner ]; then
  inner
  exit 0
fi

step "versions"
orca --version 2>&1
firefox-esr --version 2>&1
speech-dispatcher --version 2>&1 | head -1

step "start Xvfb"
Xvfb :99 -screen 0 1280x800x24 >"$OUT/xvfb.log" 2>&1 &
xvfb_pid=$!
export DISPLAY=:99
for _ in $(seq 20); do [ -S /tmp/.X11-unix/X99 ] && break; sleep 0.5; done
[ -S /tmp/.X11-unix/X99 ] && step "X ready" || step "X socket not found"

# The accessibility bus and speech-dispatcher put their sockets here.
export XDG_RUNTIME_DIR=/tmp/runtime-reader
mkdir -p -m 700 "$XDG_RUNTIME_DIR"

step "start inner under dbus-run-session"
dbus-run-session -- "$0" inner

step "summary"
LOG=$OUT/orca-debug.log
echo "orca-debug.log size: $(stat -c %s "$LOG" 2>/dev/null || echo missing) bytes"
echo "Speaking lines: $(grep -c 'SPEECH DISPATCHER: Speaking' "$LOG" 2>/dev/null)"
echo "First 30 utterances:"
grep 'SPEECH DISPATCHER: Speaking' "$LOG" 2>/dev/null | head -30 | sed -E 's/<[^>]*>//g'
if grep 'SPEECH DISPATCHER: Speaking' "$LOG" 2>/dev/null | grep -q 'Finished loading'; then
  found=1; echo "Finished loading: SPOKEN"
else
  found=0; echo "Finished loading: NOT spoken"
fi
kill "$xvfb_pid" 2>/dev/null
[ "$found" = 1 ]
````
