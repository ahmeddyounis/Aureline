# Long-Running Agent Budget Fixtures

## wall_clock_ceiling_stop.json

A budget catalogue captured after a wall-clock ceiling incident on a
long-running agent.

The migration agent's wall-clock ceiling reached its limit: the ceiling is
`ceiling_reached` under `hard_stop_on_reach`, its stop outcome is `hard_stopped`,
and the run state agrees that the run is `stopped_at_ceiling`. Its spend ledger
accumulated strictly from low to medium across the sweep and transform phases,
the metered band discloses the BYOK charge owner, and the run keeps its Beta claim
because stopping cleanly at a configured ceiling is the feature working — not a
quota failure.

The indexing agent is still `running` at Stable while its token ceiling is in its
`warning_raised` state and its managed entitlement quota is in `warning` with the
warning surfaced. No ceiling has been reached, so no stop has fired; the warnings
are surfaced before any hard limit.

This demonstrates that every run configures the wall-clock, token, and tool-call
ceilings and bounds them, that a reached bounding ceiling actually stops the run
with the run state agreeing, that the spend ledger only accumulates and discloses
who is charged, and that ceiling and quota warnings are surfaced before the hard
limit.
