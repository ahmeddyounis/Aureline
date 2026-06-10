# Provider And Model Graduation Fixtures

## kill_switch_fired_backout_narrowed.json

A graduation catalogue captured after a kill-switch incident. The managed
flagship route stays claimed (Beta) but its broad ring is held with the kill
switch armed and a verified backout path. The BYOK mid route was halted by a
provider-scoped kill switch that fired after a latency regression: it is
`kill_switched`, narrowed to `unavailable`, carries no evidence refs, and its
fired switch denies new dispatch while the verified backout restored the prior
route. The local small route was backed out of canary to the internal ring and
sits `held` (not a claimed lane), narrowing to `unavailable` on stale proof.

This demonstrates that a kill switch fails closed and narrows the claim instead
of hiding the route, that a backed-out route drops out of its claimed lane rather
than keeping an optimistic posture, and that every route — halted or not — still
carries a fail-closed kill switch and a verified backout path.
