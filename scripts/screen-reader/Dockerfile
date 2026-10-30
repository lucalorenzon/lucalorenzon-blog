# Image for the on-demand screen reader check: Orca and Firefox ESR on a
# virtual screen, with no real sound card. Card:
# docs/wip/run-screen-reader-check-in-a-container.md
FROM debian:trixie-slim

# Orca 51 is only in trixie-backports (trixie itself has 48), and the exact
# version is pinned so that a reading can be compared over time. Backports
# are used only for what is asked for with -t. Orca 51 needs speech-dispatcher
# 0.12.1 from there, and its espeak-ng module must come from there too (the
# trixie one wants exactly 0.12.0). Firefox ESR is not pinned: Debian drops
# old ESR versions, so a pin would break the build.
RUN echo 'deb http://deb.debian.org/debian trixie-backports main' \
      > /etc/apt/sources.list.d/backports.list \
 && apt-get update \
 && apt-get install -y --no-install-recommends -t trixie-backports \
      orca=51.1-1~bpo13+1 speech-dispatcher speech-dispatcher-espeak-ng \
 && apt-get install -y --no-install-recommends \
      firefox-esr xvfb xauth dbus dbus-x11 at-spi2-core espeak-ng \
      pulseaudio pulseaudio-utils \
      openbox xdotool procps libglib2.0-bin fonts-dejavu-core ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Gives the host's uid a passwd entry at run time (see entrypoint.sh).
RUN apt-get update \
 && apt-get install -y --no-install-recommends libnss-wrapper \
 && rm -rf /var/lib/apt/lists/*

# Xvfb cannot create this folder when it runs as a normal user.
RUN mkdir -p -m 1777 /tmp/.X11-unix

COPY entrypoint.sh /usr/local/bin/entrypoint.sh
# Records the null sink and decides when the reading is over (python3 is
# already in the image: Debian's tools depend on it).
COPY record.py /usr/local/bin/record.py
RUN chmod 755 /usr/local/bin/entrypoint.sh /usr/local/bin/record.py

# No user is created here: the host runs the container with
# --user <uid>:<gid> so that /out is writable by the host user, whatever its
# uid. The entrypoint gives that user a home and a runtime folder.
# /site (read-only) and /out are mounted by the host program.
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
