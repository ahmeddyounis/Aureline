# Routing Policy Fixtures

## quota_exhausted_fallback_to_local.json

A routing-policy catalogue captured after a per-session quota incident. The
composer surface's BYOK vendor quota crossed its warning into the grace window,
so the policy resolved down to its on-device fallback hop: the BYOK hop is
`available_not_selected`, the local hop is `selected` and matches the resolved
`local` mode, and the chain still ends in a reachable non-AI terminal fallback.
The surface narrowed from Stable to Beta rather than keeping an optimistic claim.

The background-agent surface had its per-session vendor quota fully exhausted and
no on-device model available: both AI hops are `exhausted_skipped`, the surface
dropped out of every claimed lane to `held`, carries no evidence refs, and
narrows to `unavailable` on the provider-unavailable trigger — yet the chain
still terminates in a reachable non-AI command path.

This demonstrates that a fallback chain stays strictly ordered and always ends in
a non-AI terminal fallback, that an exhausted quota or per-session budget narrows
the claim instead of hiding behind a Stable label, and that the selected hop's
mode always matches the surface's resolved mode.
