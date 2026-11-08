# Silence the expected error in the robots list's tests

- **Opened:** 2026-11-08, by the agent, while checking the pinned tools
  for newer versions
  ([card](../done/2026-11-08_check-for-newer-versions-of-pinned-tools.md)).
- **Status:** noticed, not started.

## Why

The unit test `test_main_writes_nothing_on_mismatch` in
`scripts/robots-list/test_robots_list.py` runs the program on a file
with the wrong checksum, on purpose. The test passes, but the program's
error message, `error: ... does not match PINNED_SHA256`, is not held
back as the other tests do, and appears in the log of every run of the
tests, in the build and in the daily job. On 2026-11-08 it misled the
agent reading the daily job's log into reporting a possible failure.

## Known from the start

- Another test of the same file already holds back the error output,
  with `contextlib.redirect_stderr`.
- The test could also check that the message is the expected one,
  instead of only holding it back.
