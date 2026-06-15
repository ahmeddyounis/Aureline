# Commercial honesty certification — human-readable rendering

Human-readable rendering of the canonical honesty-certification packet. This row
is a depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
The machine-readable truth is at
`artifacts/service/m5-commercial-honesty-certification.json`.

## Per-dimension certification

| Row | Dimension | Backing consumer | Declared claim | Effective claim | Drills |
| --- | --- | --- | --- | --- | --- |
| honesty.entitlement | entitlement_honesty | entitlement_summary | managed_full | managed_full | 4 |
| honesty.metering | metering_honesty | metering_degradation_rules | managed_full | managed_full | 3 |
| honesty.forecast | forecast_honesty | usage_forecast_views | managed_full | managed_full | 3 |
| honesty.chargeback | chargeback_honesty | chargeback_scope_views | managed_full | managed_full | 2 |
| honesty.downgrade_offboarding | downgrade_offboarding_honesty | offboarding_cards | managed_full | managed_full | 4 |
| honesty.commercial_boundary | commercial_boundary_honesty | commercial_boundary_cards | managed_full | managed_full | 2 |

The effective claim is the declared claim capped by the weakest drill cap. With
every drill `certified` and its evidence `current`, no row is narrowed; a single
`narrowed` drill or `stale` evidence narrows its row to `managed_narrowed`, and
`missing` or `downgraded` evidence drops it to `local_safe_only`. The local-safe
baseline never drops and the local core is never blocked.

## Drills exercised

| Drill | Exercised by |
| --- | --- |
| stale_meter_drill | metering, forecast |
| fail_open_local_core | metering, forecast |
| fail_closed_managed_action | metering |
| seat_loss_drill | entitlement, downgrade_offboarding |
| org_switch_drill | entitlement, downgrade_offboarding |
| grace_period_drill | entitlement, downgrade_offboarding |
| export_rights_validation | entitlement, forecast, chargeback, downgrade_offboarding, commercial_boundary |
| chargeback_scope_export_check | chargeback |
| residual_dependency_disclosure_review | commercial_boundary |

All nine drills are exercised; 18 drill results across the six rows.

## Deployment-profile coverage — never online-only

| Row | Certified in | Not offered in |
| --- | --- | --- |
| honesty.entitlement | self_hosted, enterprise_online, managed_cloud | individual_local, air_gapped |
| honesty.metering | self_hosted, enterprise_online, managed_cloud | individual_local, air_gapped |
| honesty.forecast | self_hosted, enterprise_online, managed_cloud | individual_local, air_gapped |
| honesty.chargeback | self_hosted, enterprise_online, managed_cloud | individual_local, air_gapped |
| honesty.downgrade_offboarding | self_hosted, enterprise_online, managed_cloud | individual_local, air_gapped |
| honesty.commercial_boundary | individual_local, self_hosted, enterprise_online, air_gapped, managed_cloud | (none) |

Every row partitions all five deployment profiles between certified and
not-offered, so the self-host, air-gapped, and mirror profiles are always
addressed. The commercial-boundary row certifies in every profile, including
air-gapped, where the local open core stands alone.

## Narrowing rule

| Input | Cap |
| --- | --- |
| grade certified / not_applicable | managed_full |
| grade narrowed | managed_narrowed |
| evidence current | managed_full |
| evidence stale | managed_narrowed |
| evidence missing / downgraded | local_safe_only |

Each drill cap is the weaker of its grade cap and its evidence cap; each row's
effective certified claim is the weakest of its declared claim and every drill
cap. A failed drill or stale evidence narrows the row automatically instead of
inheriting broader managed marketing language.

## Surface bindings

| Surface | Narrows on failure |
| --- | --- |
| release_center | yes |
| help_about | no |
| diagnostics | no |
| service_health | no |
| support_admin_packet | no |
| claim_public_truth_automation | yes |

Each surface projects the effective certified claim, never a stronger one. The
release center and claim/public-truth automation narrow the marketed claim when a
certification fails.

## Summary

- 6 certification rows: one per honesty dimension, each riding a real sibling
  consumer packet rather than a parallel scorecard.
- 9 certification drills, 18 drill results; every drill exercised at least once.
- Every row keeps a non-empty local-safe baseline; the local core is never blocked
  when a certification fails.
- A failed drill or stale evidence narrows the row's marketed claim automatically;
  stale or missing evidence never stays green.
- Never certified from one vendor-managed online profile alone: every row addresses
  all five profiles, and the boundary row certifies air-gapped.
- 6 surfaces, each projecting the effective claim; the release center and claim
  automation narrow on a failed certification.
