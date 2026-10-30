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

## Trial: does Orca read the page in a container? (in progress)

Agreed with the user on 2026-10-30: one hour at most, and the trial
succeeds if Orca reads the site's page inside the container (its debug
file holds the reading). Voice, audio file and the build comparison stay
out of it: they are the other questions above.

Time used so far: 10 minutes (08:37 to 08:47), paused by the user.

Set-up, chosen for the trial only: Debian trixie, where Firefox is an
ordinary package (on Ubuntu it is a snap, which does not work in a
container); a virtual screen (Xvfb) instead of GNOME; a session bus with
the accessibility bus; the openbox window manager, so Firefox gets the
focus without a click; Orca started before Firefox, as in the local
command. The site is built on the host and mounted read-only. The files
are in the appendix below.

### First run

Orca 48.1, Firefox 153.6.0esr, speech-dispatcher 0.12.0. Orca saw
Firefox and said the window title ("Luca Lorenzon - Home Page — Mozilla
Firefox frame") and "Page has 3 landmarks, 1 heading.", then nothing:
no "Finished loading", no reading of the content. The debug file stops
right after the landmark summary, and Orca did not quit on SIGTERM.

Two explanations, not told apart yet: Orca got stuck, perhaps waiting on
the voice, as the container has no sound device; or Orca went on reading
and the end of its debug file, written in blocks, was lost when the
process was killed. Orca 48 also writes its utterances as plain strings
(`Speaking '...' as string`), not as SSML like Orca 50, so the local
command's parser would not read this file as it is.

### Next run (prepared, not run yet)

The script lists the processes before quitting (is speech-dispatcher
running?) and sends Orca SIGABRT with Python's faulthandler on, so Orca
prints where each thread is: idle means the file was cut short, stuck
means something blocks it.

How to run it: build the image from a folder holding the two files
below, then run it with the built site mounted at `/site` (read-only)
and an empty folder writable by uid 1000 at `/out`:

```
sudo docker build -t sr-trial . 2>&1 | tee build.log
sudo docker run --rm -v <repository>/target/site:/site:ro -v "$PWD/out":/out sr-trial 2>&1 | tee run.log
```

where `<repository>` is the path of this repository. The user runs
Docker; the agent has no access to it.

## Appendix: trial files

These are throwaway trial files, kept here so they survive the session.

`Dockerfile`:

````dockerfile
FROM debian:trixie-slim

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
      orca firefox-esr xvfb xauth dbus dbus-x11 at-spi2-core \
      speech-dispatcher speech-dispatcher-espeak-ng espeak-ng \
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

  # Diagnosis run: dump Orca's stacks to orca-stderr.log, which tells
  # whether it is idle (then the debug file was only cut short) or stuck.
  step "SIGABRT to orca (stack dump)"
  kill -ABRT "$orca_pid" 2>/dev/null
  for _ in $(seq 5); do kill -0 "$orca_pid" 2>/dev/null || break; sleep 1; done
  if kill -0 "$orca_pid" 2>/dev/null; then step "orca still alive; SIGKILL"; kill -KILL "$orca_pid"; fi

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
