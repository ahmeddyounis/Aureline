# Remote Preview Route Lifecycle, Expiry, Target Identity, and Trust Disclosure Fixtures

These fixtures are valid, export-safe packets that exercise the lifecycle,
expiry, target-identity, and preview/runtime trust narrowing behavior the
canonical support export keeps green. Each one keeps the trust-review and
consumer-projection invariants satisfied and proof freshness valid — the
difference is which states are narrowed and why.

## expired_route_blocked.json

A preview route that has passed its expiry, so the route is `expired`,
`blocked_route_expired`, and auto-revoked, and carries an explicit attention
reason. Demonstrates that every route stays time-bounded and that an expired
route is blocked and auto-revoked rather than silently served.

## unbounded_route_blocked.json

A provider-owned route the contract cannot make time-bounded, so the expiry state
is `no_expiry_unbounded`, the route is `blocked_no_expiry_not_time_bounded`, the
phase, host, trust, and egress are all `unknown_*_provider_owned`, and the route
carries explicit attention reasons. Demonstrates that an unbounded route is never
served, that an unknown host/trust/egress is disclosed rather than assumed, and
that every route stays attributable.
