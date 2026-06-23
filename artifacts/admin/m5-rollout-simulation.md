# Rollout simulation — evidence companion

Human-readable companion to
[`/fixtures/admin/m5-rollout-simulation/canonical_simulation.json`](../../fixtures/admin/m5-rollout-simulation/canonical_simulation.json)
and its boundary schema
[`/schemas/admin/m5-rollout-simulation.schema.json`](../../schemas/admin/m5-rollout-simulation.schema.json).
It gives reviewers the per-profile dry-run scenarios without reading the JSON. The
contract narrative lives in
[`/docs/admin/m5-rollout-simulation.md`](../../docs/admin/m5-rollout-simulation.md),
and the frozen object model it binds back to lives in
[`/artifacts/admin/m5-admin-plane.md`](./m5-admin-plane.md).

- Bundle id: `m5-rollout-simulation:bundle:0001`
- Record kind: `m5_rollout_simulation_bundle`
- Binds matrix: `m5-admin-plane:matrix:0001`
- Profiles: 4 · Scenarios: 11 · Invariants: 17

## Profiles and managed-claim auto-narrowing

| Profile | Deployment | Sim freshness | Mirror freshness | Posture freshness | Claim state | Narrow reasons |
| --- | --- | --- | --- | --- | --- | --- |
| `managed_cloud` | managed_cloud | fresh | fresh | fresh | active_enforced | — |
| `self_hosted` | self_hosted | recent | fresh | recent | active_enforced | — |
| `sovereign_air_gapped` | sovereign_air_gapped | stale | fresh | stale | unconfirmed_stale | simulation_evidence_stale, endpoint_posture_stale |
| `mirrored_offline` | managed_cloud | recent | stale | recent | mirror_offline_last_known | mirror_freshness_stale |

The managed claim reads `active_enforced` only when simulation, mirror, and
endpoint-posture evidence are all fresh. The sovereign profile auto-narrows
because its simulation and posture evidence are stale; the mirrored profile
auto-narrows to last-known because its offline mirror is stale. No-console-required
explainability holds on the offline rows.

## Scenarios — tightening versus widening

| Profile | Change kind | Direction | Widening dims | Review | Staging | Rollback | Outcome |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `managed_cloud` | policy_import | tightening | — | single_admin_review | immediate_allowed | instant_local_revert | safe_to_promote |
| `managed_cloud` | route_egress_expansion | mixed | new_egress_class | security_compliance_review | staged_ring_required | staged_revert | promote_with_staged_rollout |
| `managed_cloud` | policy_promotion | widening | new_ai_provider | security_compliance_review | staged_ring_required | staged_revert | promote_with_staged_rollout |
| `self_hosted` | bundle_rollout | tightening | — | single_admin_review | staged_ring_required | staged_revert | safe_to_promote |
| `self_hosted` | trust_root_change | widening | trust_root_change | blocked_pending_boundary_recheck | pinned_manual_signed_only | signed_rollback_bundle | blocked_boundary_recheck |
| `self_hosted` | policy_import | widening | new_permission, registry_source_change | security_compliance_review | staged_ring_required | staged_revert | promote_with_staged_rollout |
| `sovereign_air_gapped` | policy_import | tightening | — | single_admin_review | pinned_manual_signed_only | signed_rollback_bundle | safe_to_promote |
| `sovereign_air_gapped` | trust_root_change | widening | trust_root_change | security_compliance_review | pinned_manual_signed_only | signed_rollback_bundle | hold_for_review |
| `sovereign_air_gapped` | policy_promotion | tightening | — | single_admin_review | pinned_manual_signed_only | signed_rollback_bundle | blocked_stale_evidence |
| `mirrored_offline` | mirror_source_change | widening | registry_source_change | security_compliance_review | staged_ring_required | staged_revert | hold_for_review |
| `mirrored_offline` | bundle_rollout | tightening | — | single_admin_review | immediate_allowed | instant_local_revert | safe_to_promote |

Every widening or mixed scenario clears at least dual-control review, a staged
(non-immediate) rollout, and a non-instant rollback, and names a widening
dimension. Every tightening needs at most a single admin review and names no
widening dimension; the `managed_cloud` and `mirrored_offline` tightenings are
light, immediately-applicable restrictions. The sovereign promotion is blocked
because its simulation evidence is stale; the self-hosted trust-root rotation is
blocked pending a boundary recheck.

## Rollout flows and widening dimensions covered

All six flows are exercised: `policy_import`, `policy_promotion`,
`bundle_rollout`, `mirror_source_change`, `trust_root_change`,
`route_egress_expansion`. All five widening dimensions are exercised:
`new_permission`, `new_egress_class`, `new_ai_provider`,
`registry_source_change`, `trust_root_change`.

## Invariants (all hold)

| Invariant | Statement |
| --- | --- |
| `rollout_sim.surface_states_within_matrix` | Every endpoint state shown and every claim state is one the frozen matrix admits for the endpoint-posture surface. |
| `rollout_sim.bound_surfaces_in_matrix` | Each profile binds the policy-diff and endpoint-posture surfaces; both are present, locally explainable, and typed. |
| `rollout_sim.widening_requires_stronger_review` | Widening clears dual-control+ review, a staged rollout, a non-instant rollback, and names a dimension; tightening needs at most a single admin review. |
| `rollout_sim.tightening_not_overgated` | At least one tightening is a light, immediately-applicable restriction. |
| `rollout_sim.scenarios_are_reviewable_dry_runs` | Every scenario is a dry-run naming impacted endpoints/features, review, staging, and rollback. |
| `rollout_sim.stale_scenarios_held` | A scenario with stale simulation evidence is blocked, never safe to promote. |
| `rollout_sim.boundary_recheck_consistent` | A boundary-recheck block lines up review, outcome, and pinned signed-only delivery. |
| `rollout_sim.claim_auto_narrows_on_stale` | The claim is confirmed only when all evidence is fresh, and names the stale dimension otherwise. |
| `rollout_sim.simulation_freshness_is_worst_case` | The reported simulation freshness is the stalest scenario. |
| `rollout_sim.widening_dimensions_consistent` | A scenario names a widening dimension exactly when it widens or mixes. |
| `rollout_sim.widened_features_only_on_widening` | A feature is flagged newly widened only on a widening scenario. |
| `rollout_sim.profiles_covered` | The managed-cloud, self-hosted, sovereign/air-gapped, and mirrored/offline profiles are all simulated. |
| `rollout_sim.change_kinds_covered` | Every rollout flow is simulated somewhere. |
| `rollout_sim.widening_dimensions_covered` | Every widening dimension is simulated somewhere. |
| `rollout_sim.consumer_parity` | One typed packet serves shell, CLI/headless, Help/About, support export, and release evidence identically. |
| `rollout_sim.stable_ids_unique` | Profile, scenario, and endpoint ids are unique within scope. |
| `rollout_sim.export_safe` | Every stable id is an opaque token with no URL scheme or absolute path. |

## How to regenerate / verify

```sh
# Regenerate the fixture from the in-code builder
cargo run -p aureline-policy --example dump_m5_rollout_simulation > \
  fixtures/admin/m5-rollout-simulation/canonical_simulation.json

# Freeze gate: in-code bundle must equal the checked-in fixture
cargo test -p aureline-policy --test m5_rollout_simulation

# Human-readable projection
cargo run -p aureline-policy --example dump_m5_rollout_simulation -- --lines
```
