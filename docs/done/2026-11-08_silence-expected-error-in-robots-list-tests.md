# Silence the expected error in the robots list's tests

- **Opened:** 2026-11-08, by the agent, while checking the pinned tools
  for newer versions
  ([card](2026-11-08_check-for-newer-versions-of-pinned-tools.md)).
- **Status:** done, on 2026-11-08.

## Why

The unit test `test_main_writes_nothing_on_mismatch` in
`scripts/robots-list/test_robots_list.py` runs the program on a file
with the wrong checksum, on purpose. The test passed, but the program's
error message, `error: ... does not match PINNED_SHA256`, was not held
back, and appeared in the log of every run of the tests, in the build
and in the daily job. On 2026-11-08 it misled the agent reading the
daily job's log into reporting a possible failure.

## Answers

### How is the message kept out of the log?

The test captures the error output, as another test of the same file
already did, and checks that it holds the expected message. So the
test now checks the message, instead of only hiding it.

### Do other tests do the same?

Yes: three tests of the content dates step
(`scripts/content-dates/test_content_dates.py`), on a folder that is
not a git repository, printed an error and a warning. They are fixed
the same way, each checking its own message. The tests of the version
check and of the screen reader check were already quiet.

## Results

Commit `c0ea24f`. Before, two of the four Python test suites printed
messages among their results; after, all four print only their dots,
the number of tests and `OK` (robots list 33, version check 45, content
dates 11, screen reader 37). Only tests changed, no program code.
