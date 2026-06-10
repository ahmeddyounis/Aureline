# Recorded-Macro Promotion And Headless-Safe Result Fixtures

## headless_blocked_fail_closed.json

A user-automation catalogue captured after a recorded deploy macro failed
promotion because its capture was tainted.

The format-and-organize-imports macro stays at Stable: it was recorded from a user
session and promoted into a recipe behind a one-time promotion prompt, its
reversible edit is pre-authorized to run headless under policy and previews in a
diff first, and its headless result completed every step safely with a
content-addressed result audited to the run-record timeline.

The deploy-trigger macro demonstrates narrowing and headless fail-closed behavior:
its capture is tainted, so its promotion state is
`promotion_blocked_tainted_capture` and it claims `held` rather than any public
lane. Its irreversible external publish previews a diff, is denied by policy, and
is held back `headless_blocked_fail_closed` so it never runs unattended; the
headless result is `blocked_fail_closed` with one blocked step, and every
downgrade rule narrows to `unavailable`, so a provider outage keeps it out of
every claimed lane.

This demonstrates that a blocked promotion drops its public claim instead of
hiding behind a Stable, Beta, or Preview label, that an irreversible publish never
runs unattended headless, that a step needing interactive confirmation fails
closed rather than executing silently, and that the downgrade rules narrow rather
than hide the automation.
