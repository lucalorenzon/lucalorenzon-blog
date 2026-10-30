#!/usr/bin/env bash
# Runs inside the container: lets Orca read /site/index.html in Firefox and
# leaves Orca's debug file and version numbers in /out. It does not judge the
# reading: scripts/screen-reader/check.py (on the host) does.
set -u

ORCA_WAIT=${ORCA_WAIT:-8}
# The reading is over when sound has played and then stopped for this long.
# This is read from the recorded silence (record.py), not from the null
# sink's state: speech-dispatcher keeps its stream open, so the sink stays
# RUNNING and never goes idle. Orca can stay silent for several seconds
# between two items, so keep it well above that gap.
QUIET_SECONDS=${QUIET_SECONDS:-10}
MAX_SECONDS=${MAX_SECONDS:-90}
OUT=/out
SINK=null_out

step() { printf '[%s] %s\n' "$(date +%H:%M:%S)" "$*"; }

# Orca 51 quits on SIGTERM and flushes its debug file then. SIGABRT is the
# last resort: Python prints every thread's stack (faulthandler) to
# orca-stderr.log, to see where it is stuck.
quit_orca() {
  local pid=$1
  if gdbus call --session --dest org.gnome.Orca.Service --object-path /org/gnome/Orca/Service \
       --method org.gnome.Orca.Service.Quit >/dev/null 2>&1; then
    step "quit orca (D-Bus quit sent)"
  else
    step "quit orca (no D-Bus service; SIGTERM)"
    kill -TERM "$pid" 2>/dev/null
  fi
  for _ in $(seq 10); do kill -0 "$pid" 2>/dev/null || break; sleep 1; done
  if kill -0 "$pid" 2>/dev/null; then
    step "orca still alive; SIGTERM"
    kill -TERM "$pid" 2>/dev/null
    for _ in $(seq 5); do kill -0 "$pid" 2>/dev/null || break; sleep 1; done
  fi
  if kill -0 "$pid" 2>/dev/null; then
    step "orca still alive; SIGABRT (stack dump)"
    kill -ABRT "$pid" 2>/dev/null
    sleep 3
  else
    step "orca quit"
  fi
}

write_versions() {
  {
    printf 'Orca\t%s\n' "$(dpkg-query -W -f='${Version}' orca)"
    printf 'Firefox\t%s\n' "$(dpkg-query -W -f='${Version}' firefox-esr)"
    printf 'speech-dispatcher\t%s\n' "$(dpkg-query -W -f='${Version}' speech-dispatcher)"
  } >"$OUT/versions.txt"
}

inner() {
  export GTK_A11Y=atspi NO_AT_BRIDGE=0 GNOME_ACCESSIBILITY=1 MOZ_ENABLE_WAYLAND=0

  step "start accessibility bus launcher"
  /usr/libexec/at-spi-bus-launcher --launch-immediately &
  bus_pid=$!
  sleep 2

  step "turn accessibility on"
  gsettings set org.gnome.desktop.interface toolkit-accessibility true || step "gsettings failed (ignored)"
  for prop in IsEnabled ScreenReaderEnabled; do
    gdbus call --session --dest org.a11y.Bus --object-path /org/a11y/bus \
      --method org.freedesktop.DBus.Properties.Set org.a11y.Status "$prop" "<true>" >/dev/null 2>&1 \
      || step "could not set $prop"
  done

  openbox >"$OUT/openbox.log" 2>&1 &
  wm_pid=$!

  # Without a sound device speech-dispatcher hangs, and Orca with it:
  # give it a null sink that accepts the sound and drops it.
  step "start pulseaudio with a null sink"
  pulseaudio --daemonize=yes --exit-idle-time=-1 --log-target=file:"$OUT/pulseaudio.log" \
    || step "pulseaudio failed to start"
  pactl load-module module-null-sink sink_name=$SINK >/dev/null \
    && pactl set-default-sink $SINK && step "null sink is the default" \
    || step "null sink setup failed"

  # Started before Orca, so the file holds everything Orca says. It ends by
  # itself when the reading is over (or at MAX_SECONDS).
  python3 /usr/local/bin/record.py --out "$OUT/reading.wav" \
    --quiet "$QUIET_SECONDS" --max "$MAX_SECONDS" &
  rec_pid=$!

  # Firefox turns accessibility on only if a screen reader is running when
  # it starts, hence Orca first.
  step "start orca"
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
user_pref("browser.translations.automaticallyPopup", false);
user_pref("browser.translations.enable", false);
JS
  firefox-esr --new-instance --no-remote --profile "$profile" file:///site/index.html \
    >/dev/null 2>"$OUT/firefox-stderr.log" &
  ff_pid=$!

  step "focus firefox window"
  if timeout 30 xdotool search --sync --onlyvisible --class firefox windowactivate; then
    step "focus ok"
  else
    step "focus failed or timed out"
  fi

  wait "$rec_pid"

  quit_orca "$orca_pid"

  step "stop firefox, openbox, bus launcher"
  kill "$ff_pid" "$wm_pid" "$bus_pid" 2>/dev/null
  sleep 2

  mkdir -p "$OUT/speechd"
  cp -r "$HOME"/.cache/speech-dispatcher/log/* "$OUT/speechd/" 2>/dev/null || step "no speech-dispatcher log"
  step "inner done"
}

if [ "${1:-}" = inner ]; then
  inner
  exit 0
fi

# The host runs the container as its own uid, which has no home and no
# passwd entry here: give it both, in a folder anybody can write to.
export HOME=/tmp/home USER=reader LOGNAME=reader
mkdir -p "$HOME" || exit 1
# dbus-daemon refuses to start for a uid with no passwd entry. nss_wrapper
# shows the programs a passwd and group file of our own, without making the
# image's /etc/passwd writable.
if ! id -un >/dev/null 2>&1; then
  printf 'reader:x:%s:%s:reader:%s:/bin/bash\n' "$(id -u)" "$(id -g)" "$HOME" >/tmp/passwd
  printf 'reader:x:%s:\n' "$(id -g)" >/tmp/group
  export LD_PRELOAD=libnss_wrapper.so NSS_WRAPPER_PASSWD=/tmp/passwd NSS_WRAPPER_GROUP=/tmp/group
fi
# The accessibility bus and speech-dispatcher put their sockets here.
export XDG_RUNTIME_DIR=/tmp/runtime
mkdir -p -m 700 "$XDG_RUNTIME_DIR" || exit 1

write_versions || exit 1

step "start Xvfb"
Xvfb :99 -screen 0 1280x800x24 >"$OUT/xvfb.log" 2>&1 &
xvfb_pid=$!
export DISPLAY=:99
for _ in $(seq 20); do [ -S /tmp/.X11-unix/X99 ] && break; sleep 0.5; done
if [ -S /tmp/.X11-unix/X99 ]; then step "X ready"; else step "X socket not found"; fi

step "start inner under dbus-run-session"
dbus-run-session -- "$0" inner

kill "$xvfb_pid" 2>/dev/null
exit 0
