# Run the screen reader check in a container

- **Opened:** 2026-10-30, asked by the user while the card
  [run a screen reader check on demand](../done/2026-10-30_run-screen-reader-check-on-demand.md)
  was being worked on.
- **Status:** noticed, not started.

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
