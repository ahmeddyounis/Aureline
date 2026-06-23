# Admin-plane matrix contract

This document freezes the object model behind Aureline's local admin plane: the
effective-policy view, policy diff, decision-history timeline / audit-event
explorer, locked-state explanation, retention/deletion matrix, offboarding
wizard, procurement/verification packet, and endpoint-posture card. These are
governed product contracts, not portal-only or support-console afterthoughts.

The matrix does not re-implement those surfaces. Each one already has a boundary
schema (under [`/schemas/admin/`](../../schemas/admin/) plus a few sibling
`schemas/records/`, `schemas/governance/`, `schemas/release/`, and
`schemas/storage/` schemas) and at least one producing crate. The matrix is the
single place that **names the admin-plane object families**, **freezes their
stable identifiers**, **maps each one to the proof packet that keeps it current**,
**pins one shared state vocabulary**, **defines the controlled vocabulary** the
admin plane reuses, **covers every admin path**, and **states the invariants**
every admin surface must hold — so docs, Help/About, support, and commercial
surfaces point at the same underlying objects rather than re-expressing policy,
audit, retention, or offboarding truth ad hoc.

The track invariant this lane protects: managed, self-hosted, sovereign,
mirrored, and offline-capable profiles stay **locally explainable**. A user can
see why a control is locked, what policy or mirror source is active, what data
classes exist and where they live, what can be exported or deleted now versus
later, who owns the next step, and what packet proves current posture — without a
separate vendor console.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update
in the same change.

## Companion artifacts

- [`/schemas/admin/m5-admin-plane.schema.json`](../../schemas/admin/m5-admin-plane.schema.json)
  — boundary schema for `m5_admin_plane_matrix`.
- [`/fixtures/admin/m5-admin-plane/canonical_matrix.json`](../../fixtures/admin/m5-admin-plane/canonical_matrix.json)
  — the published canonical matrix; the freeze gate asserts the in-code builder
  equals it byte-for-byte.
- [`/artifacts/admin/m5-admin-plane.md`](../../artifacts/admin/m5-admin-plane.md)
  — the human-readable companion (surface, path, state, and invariant tables).
- `crates/aureline-policy/src/m5_admin_plane/` — the builder, invariants,
  validation, and human-readable projection.
- `cargo run -p aureline-policy --example dump_m5_admin_plane` — the headless
  emitter (JSON, or `-- --lines` for the projection).

## Surface families

Each family cites the canonical boundary schema(s) it binds, the crate(s) that
already produce that truth, and the proof packet that keeps it current.

| Surface token | Family | Bound schemas | Proof packet |
| --- | --- | --- | --- |
| `effective_policy_view` | Effective policy view | effective_policy_card | `docs/admin/policy_explainability_contract.md` |
| `policy_diff` | Policy diff | effective_policy_card | `docs/admin/policy_diff_alpha.md` |
| `decision_history_timeline` | Decision-history timeline / audit explorer | audit_event_record, audit_event_filter, effective_policy_card | `docs/admin/audit_event_explorer_contract.md` |
| `locked_state_explanation` | Locked-state explanation | effective_policy_card | `docs/admin/policy_explainability_contract.md` |
| `retention_deletion_matrix` | Retention / deletion matrix | record-class-registry, records_export_delete_lifecycle, export_delete_request_summary | `docs/governance/record_class_governance.md` |
| `offboarding_wizard` | Offboarding wizard | deprovision_handoff, m5_offboarding_continuity | `docs/storage/m5_offboarding_continuity_contract.md` |
| `procurement_verification_packet` | Procurement / verification packet | offline_verification_packet, admin_audit_export | `docs/admin/admin_audit_export_contract.md` |
| `endpoint_posture_card` | Endpoint-posture card | effective_policy_card, fleet_status_row, device_rebind_event | `docs/admin/org_admin_seat_and_fleet_contract.md` |

Each surface entry additionally carries: a stable `surface_id`
(`admin_surface.<token>`), the consumers that render it, the applicable states
from the shared vocabulary, the controlled-vocabulary axes it binds, its
ownership/decision-right fields, a freshness rule, the local-safe actions that
survive an offline/mirror window, whether it captures user writes and offers
publish-later capture, and a local-explainability note.

## Controlled vocabulary

The matrix defines the named controlled vocabulary the admin plane reuses, and
each surface declares which axes it binds:

- **`policy_source_state`** — where the active value comes from and that source's
  state: local default, workspace setting, managed/mirrored policy bundle,
  remembered decision, signed offline bundle, or unknown.
- **`verification_signature_posture`** — signed-verified, signed-unverified,
  unsigned-local, expired, revoked, or unverifiable-offline.
- **`delete_export_state`** — available now, queued (publish later), blocked by
  hold, in progress, completed with receipt, window expired, or not applicable.
- **`data_residency_class`** — managed-copy versus local-only: local-only,
  managed copy, mirrored copy, shared-workspace copy, or exported snapshot.
- **`owner_escalation`** — local user, workspace owner, org admin, security
  owner, compliance owner, or vendor support.

## Shared state vocabulary

One vocabulary spans every surface, so a consumer can resolve any admin state by
a stable token. Each term cites the upstream schema enum it derives from.

`active_enforced`, `locked_by_policy`, `inherited_default`, `overridden_local`,
`unconfirmed_stale`, `pending_managed_sync`, `signature_unverified`,
`delete_pending`, `delete_blocked_by_hold`, `delete_receipted`,
`export_available_now`, `export_deferred`, `mirror_offline_last_known`,
`boundary_changed_recheck_required`, `imported_snapshot_no_live`,
`unknown_requires_review`.

`unconfirmed_stale` is the no-silent-green downgrade: a would-be-current value
whose backing policy/audit evidence is stale, partial, or cached.

## Admin paths

The matrix covers every deployment/connectivity path an admin surface must stay
explainable on.

| Path token | Path | Write posture | Boundary recheck | Default live vs snapshot |
| --- | --- | --- | --- | --- |
| `local_individual` | Local individual | writes_live | no | live_only |
| `managed_cloud` | Managed cloud | writes_live | yes | snapshot_capable |
| `self_hosted` | Self-hosted | writes_live | yes | snapshot_capable |
| `sovereign_air_gapped` | Sovereign / air-gapped | local_draft_preserved | yes | snapshot_only |
| `mirrored_offline` | Mirrored / offline | publish_later_queued | yes | snapshot_only |
| `imported_snapshot` | Imported snapshot | read_only_replay | no | snapshot_only |

## Invariants

The builder computes each invariant's `holds` flag from the built data, so an
inconsistent edit flips an invariant and fails the freeze gate.

- `admin_plane.canonical_object_identity` — every surface cites a canonical
  schema and a producing crate.
- `admin_plane.proof_packet_mapped` — every surface maps to a non-empty proof
  packet; this is the release-automation binding that fails stable promotion when
  a claimed surface lacks a mapped proof row.
- `admin_plane.no_silent_green` — every freshness-headlined surface carries
  `unconfirmed_stale` and a green-downgrading freshness rule.
- `admin_plane.locked_state_explained` — every surface that can show a locked
  control binds policy-source and owner/escalation vocabularies and declares a
  required ownership field.
- `admin_plane.ownership_visible` — every surface binds owner/escalation and
  declares a required ownership/decision-right field.
- `admin_plane.delete_export_honest` — surfaces that act on data bind the
  delete/export vocabulary and expose a receipt or blocked-by-hold path, never a
  bare deleted claim.
- `admin_plane.data_class_located` — surfaces that capture or export data declare
  managed-copy versus local-only via the data-residency vocabulary.
- `admin_plane.verification_posture_explicit` — surfaces that can show an
  unverified signature bind the verification/signature vocabulary.
- `admin_plane.locally_explainable_offline` — every surface keeps local-safe
  actions; write-bearing ones offer publish-later capture.
- `admin_plane.controlled_vocabulary_complete` — each of the five named
  controlled vocabularies is bound by at least one surface.
- `admin_plane.stable_ids_unique` — surface ids, path ids, and state tokens are
  unique.
- `admin_plane.all_paths_covered` — all six admin paths are present.
- `admin_plane.all_surfaces_present` — every surface family is present once.
- `admin_plane.typed_not_portal_only` — every surface is typed and locally
  explainable, never portal-only or console-only.

## Export safety

The record carries no endpoint URLs, hostnames, credentials, raw provider
payloads, or absolute paths — only opaque object refs, stable tokens, and short
reviewable sentences. `is_support_export_safe()` enforces that
`raw_payload_excluded` is true and every ref is a repo-relative object ref or
`aureline://` handle, so the matrix is safe to embed in a support export
verbatim.

## Composes with

This contract composes with (and does not replace) the per-surface contracts it
binds, notably
[`/docs/admin/policy_explainability_contract.md`](./policy_explainability_contract.md),
[`/docs/admin/policy_diff_alpha.md`](./policy_diff_alpha.md),
[`/docs/admin/audit_event_explorer_contract.md`](./audit_event_explorer_contract.md),
[`/docs/admin/deprovision_and_rebind_contract.md`](./deprovision_and_rebind_contract.md),
[`/docs/admin/admin_audit_export_contract.md`](./admin_audit_export_contract.md),
and
[`/docs/admin/org_admin_seat_and_fleet_contract.md`](./org_admin_seat_and_fleet_contract.md).
