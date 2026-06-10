# Provider And Model Graduation, Rollout Rings, And Kill Switch

- Packet: `provider-model-graduation:stable:0001`
- Schema: `schemas/ai/ship-provider-and-model-graduation-packets-rollout-rings-and-kill-switch-or-backout-paths.schema.json`
- Support export: `artifacts/ai/m5/ship_provider_and_model_graduation_packets_rollout_rings_and_kill_switch_or_backout_paths/support_export.json`
- Fixture: `fixtures/ai/m5/ship_provider_and_model_graduation_packets_rollout_rings_and_kill_switch_or_backout_paths/`

## Coverage

The packet materializes the provider/model graduation surface into one row per
route. Every route carries its claimed qualification plus the rollout ring it is
exposed through, that ring's progress state, a provider-neutral kill switch, and
a verified backout path.

- The managed flagship route reached general availability with a Stable claim, a
  route-scoped fail-closed kill switch, and a fully-reversible backout path.
- The BYOK mid route is rolling through the broad ring at Beta with a
  provider-scoped kill switch and a checkpoint-reversible backout drilled and
  verified.
- The local small route is in canary at Preview with a route-scoped kill switch
  and a fully-reversible backout.
- The regressed route was rolled back from broad: its global kill switch has
  fired, it is `backed_out` and held (not a claimed lane), and it narrows to
  `unavailable` on stale proof.
- Proof freshness SLO is 168 hours with automatic narrowing on stale proof.

## Safety

The packet refuses to graduate a route further than its safety posture can back.
Every route's kill switch must fail closed; a claimed route must keep its kill
switch armed; a broadly-exposed claimed route must carry a verified, reversing
backout path; general availability is held to a Stable claim; and a kill-switched
or backed-out route narrows its claim instead of keeping an optimistic posture.
Every claimed route narrows rather than hides on stale proof, reusing the frozen
M5 AI workflow matrix qualification, downgrade, and rollback-posture vocabularies
so no route row may stay greener than its evidence. Raw provider endpoints,
credential bodies, raw provider payloads, exact spend values, and internal
kill-switch tokens never cross the support boundary.
