# Update the screen reader check for an English site

- **Opened:** 2026-11-01, while the card
  [put the curriculum on the home page](../done/2026-11-01_put-curriculum-on-home-page.md)
  was being closed.
- **Status:** noticed, not started.

## Why

The site moved from Italian to English, and its page now shows the
curriculum instead of "Sito in costruzione". The on-demand screen reader
check (`scripts/screen-reader/`) still compares what Orca says with the
reference transcript of the Italian page, so its next run will report a
difference. Some of its comments, its unit tests and its log sample
also describe an Italian page.

The card
[give the screen reader container an Italian voice](give-the-screen-reader-container-an-italian-voice.md)
rested on visitors being mostly Italian; with an English site that
premise no longer holds.

## Questions

- Run the check (it needs Docker, run by the user) and accept the new
  transcript as the reference, if it reads the page as intended.
- Which comments, tests and samples in `scripts/screen-reader/` still
  describe the Italian page, and which of them should change?
- Is the Italian voice card closed as superseded, or does part of it
  still apply?
