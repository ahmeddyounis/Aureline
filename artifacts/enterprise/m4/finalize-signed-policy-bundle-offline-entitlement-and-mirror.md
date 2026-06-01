# Signed Policy-Bundle, Offline-Entitlement Grace, and Mirror/Manual-Import — Finalize Packet

- Packet: `policy:signed-policy-bundle-finalize:seeded:0001`
- Schema version: `1`
- Contract ref: `policy:signed_policy_bundle_finalize:v1`
- Qualification: `stable` (derived, not asserted)
- Upstream verifier defects: 0
- Finalize defects: 0
- Withdrawn rows: 0
- Stable rows: all 10

## Lane coverage

| Row | Import flow | Bundle kind | Grace posture | Epoch inspectable | Simulation packet |
|---|---|---|---|---|---|
| `signed-policy-bundle-finalize:online:policy_bundle` | `online` | `policy_bundle` | `not_in_grace` | ✓ | ✓ |
| `signed-policy-bundle-finalize:online:entitlement_snapshot` | `online` | `entitlement_snapshot` | `not_in_grace` | ✓ | ✓ |
| `signed-policy-bundle-finalize:mirror:policy_bundle` | `mirror` | `policy_bundle` | `not_in_grace` | ✓ | ✓ |
| `signed-policy-bundle-finalize:mirror:entitlement_snapshot` | `mirror` | `entitlement_snapshot` | `not_in_grace` | ✓ | ✓ |
| `signed-policy-bundle-finalize:manual_import:policy_bundle` | `manual_import` | `policy_bundle` | `not_in_grace` | ✓ | ✓ |
| `signed-policy-bundle-finalize:manual_import:entitlement_snapshot` | `manual_import` | `entitlement_snapshot` | `not_in_grace` | ✓ | ✓ |
| `signed-policy-bundle-finalize:air_gapped:policy_bundle` | `air_gapped` | `policy_bundle` | `not_in_grace` | ✓ | ✓ |
| `signed-policy-bundle-finalize:air_gapped:entitlement_snapshot` | `air_gapped` | `entitlement_snapshot` | `not_in_grace` | ✓ | ✓ |
| `signed-policy-bundle-finalize:offline_grace:policy_bundle` | `offline_grace` | `policy_bundle` | `in_grace` | ✓ | ✓ |
| `signed-policy-bundle-finalize:offline_grace:entitlement_snapshot` | `offline_grace` | `entitlement_snapshot` | `in_grace` | ✓ | ✓ |

## Evidence sources

- Upstream offline-entitlement verifier:
  `security:offline_entitlement_verifier_beta:v1`
  — `crates/aureline-auth/src/offline_entitlements/mod.rs`

## Key invariants verified

1. The embedded `OfflineEntitlementVerifierBetaPage` audits with zero defects.
2. All five required import flows (`online`, `mirror`, `manual_import`, `air_gapped`, `offline_grace`) have rows for both bundle kinds.
3. Every row carries fully inspectable epoch state (`epoch_ref`, `epoch_digest`, `trust_root_ref`, `last_successful_validation_time`) preserving the same inspect/export vocabulary across all import paths.
4. Offline-grace rows declare bounded grace windows and carry an explicit `staleness_label`; staleness is not disguised as a generic authentication failure.
5. Every row's pre-apply simulation packet is present and inspectable (`inspectable_before_apply: true`) with at least one `affected_surface`.
6. No seeded row widens managed claims; `widens_managed_claims: false` on all rows.
7. Every simulation packet carries a non-empty `expiry_posture_token`.
8. Every row carries `local_core_continuity_explicit: true`.

## Hard guardrails — withdrawal conditions

Two conditions force `Withdrawn` immediately and cannot be overridden:

- A `RawPrivateMaterialExposed` defect in the upstream offline-entitlement
  verifier page (narrow reason: `raw_private_material_exposed`).
- A stale bundle row with an empty `staleness_label`
  (narrow reason: `staleness_disguised_as_auth_failure`).

## Canonical paths

- Doc: `docs/enterprise/m4/finalize-signed-policy-bundle-offline-entitlement-and-mirror.md`
- Runtime owner: `aureline_policy::finalize_signed_policy_bundle_offline_entitlement_and_mirror`
- Fixtures: `fixtures/enterprise/m4/finalize-signed-policy-bundle-offline-entitlement-and-mirror/`
- Schema: `schemas/enterprise/finalize-signed-policy-bundle-offline-entitlement-and-mirror.schema.json`
