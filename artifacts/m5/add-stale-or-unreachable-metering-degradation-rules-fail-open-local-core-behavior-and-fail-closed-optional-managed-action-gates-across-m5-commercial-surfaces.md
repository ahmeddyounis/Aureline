# Metering degradation rules — human-readable rendering

Human-readable rendering of the canonical metering-degradation rule set. This row is a
depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
The machine-readable truth is at `artifacts/service/m5-metering-degradation-rules.json`.

## Per-lane fail posture and disposition

| Service family | Lane | Fail posture | Disposition | Gates an optional action |
| --- | --- | --- | --- | --- |
| ai_gateway_family | managed_lane.ai_gateway | fail_open_local_safe_with_label | fail_open_managed_labeled | no |
| sync_family | managed_lane.settings_sync | fail_open_local_safe | fail_open_local_safe_path | no |
| collaboration_relay_family | managed_lane.companion_relay | fail_closed_managed_only | fail_closed_optional_action_gated | yes, one action |
| registry_or_mirror_metadata_family | managed_lane.registry_mirror | fail_open_local_safe | fail_open_local_safe_path | no |
| telemetry_or_support_ingest_family | managed_lane.support_ingest | fail_open_local_safe_with_label | fail_open_managed_labeled | no |
| remote_workspace_control_plane_family | managed_lane.managed_workspace | fail_closed_managed_only | fail_closed_optional_action_gated | yes, one action |

The disposition is recomputed from the lane's fail posture, so fail-open and fail-closed
behavior matches the frozen control-plane matrix. Every rule keeps a non-empty local-safe
promise and never collapses the local core to local-safe-only.

## Per-trigger disclosure

| Trigger | Value disclosure | Freshness | Related managed state | Retry action |
| --- | --- | --- | --- | --- |
| metering_stale | labeled_stale_bound_to_unit_as_of_scope | freshness_stale | meter_stale | Re-check the meter now. |
| service_unreachable | suppressed_no_managed_number | freshness_unknown | (none) | Reconnect to the metering service. |
| rating_path_unavailable | suppressed_no_managed_number | freshness_unknown | (none) | Retry the rating path. |

A stale number is labeled and bound to its unit, as-of time, and scope owner; an unreachable
number is suppressed. Every rule carries a last-contact as-of time, so no spend or quota
number ever crosses the boundary bare.

## A degradation is not an account error

The degradation trigger is metering posture, distinct from the managed-state vocabulary.
Every rule sets `not_an_account_error` and stays distinct from the four account-loss states,
so a stale or unreachable meter never collapses a seat loss, an org switch, a grace window,
and a sign-in failure into one generic error.

| Distinct-from account states (every rule) |
| --- |
| seat_removed, org_switched, grace_period, reauth_required |

## The gate is one optional action

Only the two fail-closed lanes gate, and each gates exactly one named action with a blocking
reason; local work continues throughout.

| Lane | Gated optional action (illustrative, stale trigger) | Blocking reason |
| --- | --- | --- |
| managed_lane.companion_relay | Joining a new live companion-follow or relay session while relay minutes cannot be bounded. | Spend cannot be bounded because the meter is stale, so this one action waits. |
| managed_lane.managed_workspace | Attaching or running a new remote workspace while workspace-hour spend cannot be bounded. | Spend cannot be bounded because the meter is stale, so this one action waits. |

## Surface bindings

| Surface | Binds rules |
| --- | --- |
| diagnostics | all 18 rules |
| account_surface | all 18 rules |
| help_about | the three meter-stale rules for ai_gateway, settings_sync, registry_mirror |
| support_admin_packet | the 6 fail-closed (gated) rules |
| claim_public_truth_automation | all 18 rules |

## Summary

- 18 degradation rules, one per service family × degradation trigger; an exhaustive 6 × 3
  matrix.
- 12 fail-open rules (no gate) and 6 fail-closed rules (one optional action gated each),
  matching the frozen control-plane fail posture.
- Every rule keeps a non-empty local-safe promise and never collapses the local core to
  local-safe-only.
- A stale number is labeled and bound to unit, as-of time, and scope owner; an unreachable
  number is suppressed; every rule carries a last-contact as-of time.
- Every rule stays distinct from a seat loss, an org switch, a grace window, and a sign-in
  failure, and narrows the marketed claim to managed-narrowed.
- 5 surfaces, each projecting the effective claim, never a stronger one.
