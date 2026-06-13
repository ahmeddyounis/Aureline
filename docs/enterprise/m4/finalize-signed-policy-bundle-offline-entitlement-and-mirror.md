# Finalize signed administrative bundles, offline entitlement grace, emergency-disable ratchets, and trust-root rotation review

This lane makes signed administrative policy, offline-entitlement, emergency-disable,
and trust-root rotation artifacts visible and verifiable enough that product,
security review, support export, and release packets can all explain: where the
bundle came from, who issued and signed it, what scope it binds, which prior
bundle or signer it supersedes or revokes, what its last-known-good revision
is, whether a grace window is active, what capability consequences follow when
grace or trust freshness is lost, and what the pre-apply simulation showed
before the bundle was applied. The runtime owner is
`aureline_policy::finalize_signed_policy_bundle_offline_entitlement_and_mirror`.

The packet does **not** re-derive raw policy rule text, raw entitlement
payloads, raw emergency payloads, raw trust-root bytes, or raw identity truth.
The upstream `aureline_auth::offline_entitlements` verifier beta audit remains
canonical for its own slice. This packet re-exports those qualification tokens
verbatim and adds the finalize invariants needed for one evidence packet across
managed, mirrored, fleet, and air-gapped paths.

## Contract

For the stable claim to hold, all of the following conditions must be verified
simultaneously:

1. **Upstream verifier clean** — `aureline_auth::offline_entitlements::audit_offline_entitlement_verifier_beta_rows` returns zero defects for the embedded verifier page.
2. **All five import flows covered** — at least one row exists for each of: `online`, `mirror`, `manual_import`, `air_gapped`, and `offline_grace`.
3. **All four bundle kinds covered** — `admin_policy_bundle`, `entitlement_snapshot`, `emergency_disable_bundle`, and `trust_root_signer_update`.
4. **All delivery sources covered** — `managed_pull`, `mirror_publication`, `file_import`, `mdm_fleet_drop`, `air_gap_transfer`, and `last_known_good_cache`.
5. **Epoch and envelope states inspectable** — every row carries non-empty `epoch_ref`, `epoch_digest`, `trust_root_ref`, `last_successful_validation_time`, `bundle_ref`, `issuer_ref`, `scope_ref`, `delivery_source_token`, and `expiry_guidance_token`.
6. **Relations visible** — every row carries at least one inspectable `supersedes_ref` or `revokes_ref`.
7. **Grace windows declared** — every stale row (in-grace or grace-expired) carries a non-empty `grace_window_end` and a human-readable `staleness_label`; staleness is never surfaced as a generic authentication failure.
8. **Stale rows pause new privileged operations** — stale rows narrow fresh privileged actions while keeping local-safe continuity explicit.
9. **Simulation packets present before apply** — every row's `PolicyBundleSimulationPacket` carries at least one `affected_surface`, proving the pre-apply inspection ran and was exported; `inspectable_before_apply` must be `true`.
10. **Widening rows require approval** — when a row has `widens_managed_claims: true`, its simulation packet must carry a non-empty `approval_owner_ref` and a non-empty `expiry_posture_token`.
11. **Emergency-disable minimum version declared** — emergency-disable rows carry a non-empty `required_minimum_version`.
12. **Lifecycle coverage present** — the packet carries export-safe lifecycle events for `apply`, `supersede`, `revoke`, `signer_rotation_review`, and `emergency_disable_activation`.
13. **Local-core continuity explicit** — all rows carry `local_core_continuity_explicit: true` so the local-editing floor cannot be silently removed by a managed capability change.

## Required behavior

`validate_finalize_signed_policy_bundle_page` rejects a page when its `defects` list is non-empty.

`audit_finalize_signed_policy_bundle_page` runs the combined check and returns a
typed `Vec<FinalizeSignedPolicyBundleDefect>`. Each defect carries a closed
`narrow_reason_token` and an export-safe `note`. The absence of defects is the
stable claim.

Two conditions force `Withdrawn` immediately and cannot be overridden:

- A `RawPrivateMaterialExposed` defect in the upstream offline-entitlement
  verifier page (narrow reason: `raw_private_material_exposed`). The function
  returns immediately with this single defect and skips all other checks.
- A stale bundle row with an empty `staleness_label` (narrow reason:
  `staleness_disguised_as_auth_failure`). Entitlement staleness must never be
  disguised as a generic authentication failure; when the staleness label is
  missing the packet cannot make a claimable qualification.

A missing required import flow, bundle kind, delivery source, or lifecycle
class narrows to `Preview` rather than `Beta` because the coverage gap prevents
any verifiable claim for that slice.

## Import flows

| Flow token | Description |
| --- | --- |
| `online` | Bundle fetched from the live signed origin. |
| `mirror` | Bundle served from a declared signed mirror. |
| `manual_import` | Bundle imported as a signed file by an admin action. |
| `air_gapped` | Bundle transferred via an air-gapped signed media exchange. |
| `offline_grace` | Bundle used under a declared offline-grace window; last-known-good authority is extended while the primary path is unavailable. |

All five flows must be covered for a stable claim. All five carry the same
epoch inspection vocabulary (`epoch_ref`, `epoch_digest`, `trust_root_ref`,
`last_successful_validation_time`) so mirror and manual-import rows can be
compared directly with online rows by support and audit tooling.

## Bundle kinds

| Kind token | Description |
| --- | --- |
| `admin_policy_bundle` | Signed policy bundle authorising managed narrowing rules. |
| `entitlement_snapshot` | Signed entitlement snapshot describing plan, seat, and quota state. |
| `emergency_disable_bundle` | Signed emergency-disable ratchet that may narrow or freeze affected capabilities immediately. |
| `trust_root_signer_update` | Signed trust-root or signer-update review object with continuity metadata. |

The seeded page covers all four kinds for every import flow (20 rows total).

## Delivery sources

| Delivery token | Description |
| --- | --- |
| `managed_pull` | Live managed pull from the signed origin. |
| `mirror_publication` | Signed mirror publication or mirror-sync distribution. |
| `file_import` | Local operator file import. |
| `mdm_fleet_drop` | MDM or fleet-managed artifact drop. |
| `air_gap_transfer` | Offline or air-gapped transfer path. |
| `last_known_good_cache` | Last-known-good cache selection under stale or degraded conditions. |

## Pre-apply simulation packet

Every row carries a `PolicyBundleSimulationPacket` before the apply is permitted.
The packet surfaces:

- `changed_feature_areas` — feature areas whose effective policy would change.
- `previous_values_summary` / `simulated_values_summary` — export-safe before/after maps.
- `affected_surfaces` — surfaces affected by the apply (must be non-empty to prove the inspection ran).
- `degraded_mode_consequences` — consequences if the apply runs without full verification.
- `offline_or_stale_policy_notes` — grace-specific or stale-path notes.
- `approval_owner_ref` — required when `widens_managed_claims: true`.
- `expiry_posture_token` — required on every row.

## Offline-grace state

Grace-window and staleness state is carried in `OfflineGraceState`:

- `grace_posture` — `not_in_grace`, `in_grace`, or `grace_expired`.
- `grace_window_start` / `grace_window_end` — bounds on the grace extension.
- `last_known_good_revision` — opaque ref to the last-verified bundle revision.
- `staleness_label` — explicit human-readable label (must be non-empty when stale).
- `blocked_capability_consequences` — export-safe labels for capabilities blocked when grace expires.

## Lifecycle audit events

Every packet carries `BundleLifecycleAuditEvent` rows for the required audit
flows:

- `apply`
- `supersede`
- `revoke`
- `signer_rotation_review`
- `emergency_disable_activation`

Each event quotes bundle kind, bundle ref, scope, delivery source, actor,
supersedes or revokes refs, resulting privileged-operation posture, and whether
local-safe continuity remained preserved.

## Boundary

The following material stays outside this packet's support boundary:

- Raw policy bundle bodies or raw rule text.
- Raw entitlement payloads.
- Raw identities, raw hostnames, raw file paths, raw extension ids.
- Raw credentials or secret material.
- Raw exception justification text.

Every exported field carries either a closed-vocabulary token, a plain-language
label, an opaque ref, a count, or a schema-version integer.

## Truth source

The seeded proof packet is `seeded_finalize_signed_policy_bundle_page()` in
[`/crates/aureline-policy/src/finalize_signed_policy_bundle_offline_entitlement_and_mirror/mod.rs`](../../../crates/aureline-policy/src/finalize_signed_policy_bundle_offline_entitlement_and_mirror/mod.rs).

That function is the single inspectable record for this lane. Dashboards,
Help/About surfaces, and support exports should ingest it rather than cloning
status text or maintaining parallel bundle-signed checks.

## Canonical paths

- Runtime owner: `aureline_policy::finalize_signed_policy_bundle_offline_entitlement_and_mirror`
- Artifact: `artifacts/enterprise/m4/finalize-signed-policy-bundle-offline-entitlement-and-mirror.md`
- Fixtures: `fixtures/enterprise/m4/finalize-signed-policy-bundle-offline-entitlement-and-mirror/`
- Schema: `schemas/enterprise/finalize-signed-policy-bundle-offline-entitlement-and-mirror.schema.json`
