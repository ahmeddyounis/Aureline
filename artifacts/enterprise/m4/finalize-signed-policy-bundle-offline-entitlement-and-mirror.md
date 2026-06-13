# Signed Bundle, Offline Entitlement, Emergency Disable, and Trust-Root Review — Finalize Packet

- Packet: `policy:signed-policy-bundle-finalize:seeded:0001`
- Schema version: `2`
- Contract ref: `policy:signed_policy_bundle_finalize:v2`
- Qualification: `stable` (derived, not asserted)
- Upstream verifier defects: 0
- Finalize defects: 0
- Withdrawn rows: 0
- Stable rows: all 20

## Lane coverage

| Row | Import flow | Bundle kind | Delivery source | Grace posture | Privileged posture |
|---|---|---|---|---|---|
| `signed-policy-bundle-finalize:online:admin_policy_bundle` | `online` | `admin_policy_bundle` | `managed_pull` | `not_in_grace` | `admitted_with_current_verification` |
| `signed-policy-bundle-finalize:online:emergency_disable_bundle` | `online` | `emergency_disable_bundle` | `managed_pull` | `not_in_grace` | `paused_by_emergency_disable` |
| `signed-policy-bundle-finalize:mirror:trust_root_signer_update` | `mirror` | `trust_root_signer_update` | `mirror_publication` | `not_in_grace` | `admitted_with_current_verification` |
| `signed-policy-bundle-finalize:manual_import:entitlement_snapshot` | `manual_import` | `entitlement_snapshot` | `mdm_fleet_drop` | `not_in_grace` | `admitted_with_current_verification` |
| `signed-policy-bundle-finalize:air_gapped:admin_policy_bundle` | `air_gapped` | `admin_policy_bundle` | `air_gap_transfer` | `not_in_grace` | `admitted_with_current_verification` |
| `signed-policy-bundle-finalize:offline_grace:trust_root_signer_update` | `offline_grace` | `trust_root_signer_update` | `last_known_good_cache` | `grace_expired` | `denied_pending_trust_root_repair` |

All remaining rows reuse the same envelope and simulation vocabulary across the other `(flow x kind)` combinations.

## Lifecycle audit coverage

The packet embeds five export-safe lifecycle events:

1. `apply`
2. `supersede`
3. `revoke`
4. `signer_rotation_review`
5. `emergency_disable_activation`

## Evidence sources

- Upstream offline-entitlement verifier:
  `security:offline_entitlement_verifier_beta:v1`
  — `crates/aureline-auth/src/offline_entitlements/mod.rs`

## Key invariants verified

1. The embedded `OfflineEntitlementVerifierBetaPage` audits with zero defects.
2. All five required import flows and all four required bundle kinds are covered.
3. All six delivery sources (`managed_pull`, `mirror_publication`, `file_import`, `mdm_fleet_drop`, `air_gap_transfer`, `last_known_good_cache`) are present.
4. Every row carries fully inspectable epoch and envelope state: `epoch_ref`, `epoch_digest`, `trust_root_ref`, `issuer_ref`, `scope_ref`, `delivery_source`, `expiry_guidance`, and supersedes or revokes relations.
5. Offline-grace rows declare bounded grace windows and carry an explicit `staleness_label`; staleness is not disguised as a generic authentication failure.
6. Every stale row narrows new privileged operations while keeping local-safe continuity explicit.
7. Every row's pre-apply simulation packet is present and inspectable (`inspectable_before_apply: true`) with at least one `affected_surface`.
8. Emergency-disable rows declare `required_minimum_version`, making minimum-version ratchets inspectable.
9. Lifecycle audit coverage includes apply, supersede, revoke, signer rotation review, and emergency-disable activation.

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
