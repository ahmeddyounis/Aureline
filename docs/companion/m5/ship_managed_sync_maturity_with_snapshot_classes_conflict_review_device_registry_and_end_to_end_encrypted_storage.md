# Managed Sync Maturity: Snapshot Classes, Conflict Review, Device Registry, and End-to-End Encrypted Storage

This document is the human-readable contract for managed sync maturity: the
**snapshot classes** that record which classes of local state participate in managed
sync and in which direction, the **conflict review** queue that records every sync
conflict with no silent server authority, the **device registry** that records the
devices participating in sync and their trust state, and the **encrypted storage**
posture that records the customer-managed-key, end-to-end-encryption, and
region-residency claim for each managed artifact scope. The machine-readable truth
source is the checked-in support export; later desktop companion panel, CLI/headless,
diagnostics, support export, and Help/About surfaces ingest it instead of cloning
status text.

- Record kind: `ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage`
- Schema: `schemas/companion/ship-managed-sync-maturity-with-snapshot-classes-conflict-review-device-registry-and-end-to-end-encrypted-storage.schema.json`
- Support export: `artifacts/companion/m5/ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage/support_export.json`
- Markdown summary: `artifacts/companion/m5/ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage.md`
- Fixtures: `fixtures/companion/m5/ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage/`
- Producer crate: `aureline-companion`

## Sections and matrix inheritance

The packet has four sections. The snapshot-class, conflict-review, and
device-registry sections inherit their qualification and staged rollout stage from
the frozen M5 companion-matrix `managed_sync` lane; the encrypted-storage section
inherits from the `residency_encryption` lane (see
`docs/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes.md`),
so a section never claims more than the matrix qualifies.

| Section | Matrix lane | Scope | Qualification | Rollout stage |
| --- | --- | --- | --- | --- |
| `snapshot_class` | `managed_sync` | `read_only` | beta | staged_rollout |
| `conflict_review` | `managed_sync` | `read_only` | beta | staged_rollout |
| `device_registry` | `managed_sync` | `read_only` | beta | staged_rollout |
| `encrypted_storage` | `residency_encryption` | `read_only` | preview | early_access |

## Inspectable, with no silent server authority

The surface is read-only and the local core stays authoritative.

- **Snapshot classes** record which class of state syncs (`settings`, `profile`,
  `device_registry`, `workspace_layout`, `extension_state`) and its `direction`
  (`local_to_managed`, `managed_to_local`, `bidirectional`, `local_only`). Every row
  asserts `local_authoritative = true`: the local core is the source of truth, and
  every record carries a `reconciliation` state (`reconciled`,
  `diverged_pending_review`, `unreconcilable`) so a synced record always reconciles
  back to the local core or narrows honestly.
- **Conflict review** records every sync conflict (`concurrent_edit`,
  `deleted_remotely`, `schema_mismatch`, `clock_skew`, `device_revoked`) with a
  `resolution_state` (`pending_review`, `resolved_keep_local`, `resolved_keep_remote`,
  `resolved_merged`, `deferred`). There is deliberately no "resolved by server"
  state: every conflict sets `requires_user_review = true`, so the server never wins
  silently.
- **Device registry** records each device's `trust_state` (`trusted`,
  `pending_approval`, `revoked`, `unknown`) and flags the current device with
  `this_device`. Revocation is honored; an unapproved device is `pending_approval`.
- **Encrypted storage** records, per managed artifact scope
  (`managed_snapshot_store`, `sync_transport`, `conflict_history`,
  `device_registry_store`), the `encryption_posture`, the `key_custody` model
  (`customer_managed_key`, `provider_managed_key`, `local_only_no_key_escrow`), and an
  opaque `residency_region_ref`.

The `scope_contract` block asserts these guarantees for the whole packet, and the
validator rejects any section item carrying a write scope, any snapshot class that
does not record the local core as authoritative, and any conflict that does not
require user review.

## Provable where claimed

Customer-managed-key and end-to-end-encryption claims stay provable. An
encrypted-storage row's `encryption_posture` is one of:

- `end_to_end_encrypted_verified` or `encrypted_at_rest_verified` — a verified claim;
  it MUST set `claim_verified = true`, and the validator rejects a verified posture
  whose claim is not verified.
- `claimed_unverified` — encryption is claimed but the claim could not be verified; it
  MUST set `proof_label_shown = true`, so the surface labels it as unverified rather
  than showing it as proven.
- `not_encrypted` — stated honestly.

The `inspectability_contract` block asserts that every synced record is reconcilable,
that conflicts are reviewed rather than auto-resolved, that the device registry is
inspectable, that encryption claims are provable or labeled, that region residency is
disclosed, and that no claim is made without backing evidence.

## Stale-state honesty

Every item carries a `freshness` state — `live`, `cached`, `stale`, or `unknown`. Any
item whose freshness is `stale` or `unknown` MUST set `stale_label_shown = true`; the
validator rejects an unlabeled stale/unknown item. The `stale_state_honesty` block
asserts stale and unknown items are labeled, stale is never shown as live, and a
freshness floor is enforced before an item is shown.

## Exact desktop handoff

Every item carries an exact `desktop_handoff` resolving to a precise host location
(here, the managed-sync review panel). The handoff carries an opaque deep-link ref —
never a payload body — and records whether an active host session is required to
resume it.

## Downgrade-aware: narrows, never hides

`apply_managed_sync_degradation` narrows from a per-observation signal, and records
the reasons in `degraded_labels` rather than hiding the section:

| Signal | Effect |
| --- | --- |
| Sync provider unavailable | Narrows every section one step; forces every live/cached item to `stale` and labels it (`sync_provider_unavailable`, `freshness_downgraded_to_stale`) |
| Proof stale | Labels `proof_stale`; narrows every section one step |
| Upstream matrix lane narrowed | Labels `upstream_matrix_narrowed`; narrows every section one step |
| Sync uninspectable | Narrows every snapshot, conflict, and device record to `unreconcilable` and narrows the three managed-sync sections (`sync_inspection_unavailable`, `reconciliation_downgraded`) |
| Residency/encryption unverified | Downgrades every verified encryption claim to `claimed_unverified`, labels it, and narrows the encrypted-storage section (`residency_or_encryption_unverified`, `encryption_claim_downgraded`) |
| Device trust narrowed | Narrows trusted devices to `pending_approval` and narrows the conflict-review and device-registry sections (`device_trust_narrowed`) |
| Admin continuity unavailable | Narrows the device-registry and encrypted-storage sections, since managed-tenant device approval and residency depend on it (`admin_continuity_unavailable`) |
| Host session inactive | Downgrades every host-dependent desktop handoff to `unresolved` (`host_session_inactive`, `handoff_target_unresolved`) |

Degradation narrows the claim; it never corrupts the packet, which still validates
after any single or combined observation.

## Locality

- **Stays local:** the local core is the authoritative source of truth; every
  snapshot class, conflict record, device entry, and the exact desktop handoff for
  each item stay inspectable and reconcilable offline.
- **Staged:** managed sync of additional snapshot classes, customer-managed-key
  custody, and region pinning roll out per cohort and managed tenant.
- **Requires provider/admin continuity:** server-side sync, conflict relay, and
  device approval require the sync provider and, for managed tenants, admin
  continuity; end-to-end-encryption and region-residency guarantees require the
  managed key authority and are claimed only when verifiable. The local core never
  depends on them to function.

## Boundary safety

The packet is export-safe metadata only. It carries redacted summaries and opaque
refs — never credential bodies, raw key material, raw provider payloads, or raw sync
record contents. The validator runs a forbidden-material heuristic over the
serialized export.

## Regenerating

The checked-in support export, Markdown summary, and fixtures are regenerated
deterministically from the first-consumer builder:

```text
cargo run -p aureline-companion --example dump_managed_sync_maturity_surface -- canonical > artifacts/companion/m5/ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage/support_export.json
cargo run -p aureline-companion --example dump_managed_sync_maturity_surface -- markdown  > artifacts/companion/m5/ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage.md
```
