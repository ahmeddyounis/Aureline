# Admin-plane matrix — evidence companion

Human-readable companion to
[`/fixtures/admin/m5-admin-plane/canonical_matrix.json`](../../fixtures/admin/m5-admin-plane/canonical_matrix.json)
and its boundary schema
[`/schemas/admin/m5-admin-plane.schema.json`](../../schemas/admin/m5-admin-plane.schema.json).
It gives reviewers the frozen surface, path, state, and invariant tables without
reading the JSON. The contract narrative lives in
[`/docs/admin/m5-admin-plane.md`](../../docs/admin/m5-admin-plane.md).

- Matrix id: `m5-admin-plane:matrix:0001`
- Record kind: `m5_admin_plane_matrix`
- Surfaces: 8 · Admin paths: 6 · States: 16 · Invariants: 14

## Surface families

| Surface | Bound schemas | Scope | Default redaction | Proof packet |
| --- | --- | --- | --- | --- |
| `effective_policy_view` | effective_policy_card | managed_org | metadata_safe_default | `docs/admin/policy_explainability_contract.md` |
| `policy_diff` | effective_policy_card | managed_org | metadata_safe_default | `docs/admin/policy_diff_alpha.md` |
| `decision_history_timeline` | audit_event_record, audit_event_filter, effective_policy_card | managed_org | internal_support_restricted | `docs/admin/audit_event_explorer_contract.md` |
| `locked_state_explanation` | effective_policy_card | managed_org | metadata_safe_default | `docs/admin/policy_explainability_contract.md` |
| `retention_deletion_matrix` | record-class-registry, records_export_delete_lifecycle, export_delete_request_summary | managed_org | compliance_restricted | `docs/governance/record_class_governance.md` |
| `offboarding_wizard` | deprovision_handoff, m5_offboarding_continuity | managed_org | metadata_safe_default | `docs/storage/m5_offboarding_continuity_contract.md` |
| `procurement_verification_packet` | offline_verification_packet, admin_audit_export | shared_workspace | metadata_safe_default | `docs/admin/admin_audit_export_contract.md` |
| `endpoint_posture_card` | effective_policy_card, fleet_status_row, device_rebind_event | managed_org | metadata_safe_default | `docs/admin/org_admin_seat_and_fleet_contract.md` |

## Controlled vocabulary (each axis bound by ≥1 surface)

| Axis | Tokens |
| --- | --- |
| `policy_source_state` | local_default, workspace_setting, managed_policy_bundle, mirrored_policy_bundle, remembered_decision, signed_offline_bundle, unknown_source |
| `verification_signature_posture` | signed_verified, signed_unverified, unsigned_local, signature_expired, signature_revoked, unverifiable_offline |
| `delete_export_state` | available_now, queued_publish_later, blocked_by_hold, in_progress, completed_with_receipt, expired_window, not_applicable |
| `data_residency_class` | local_only, managed_copy, mirrored_copy, shared_workspace_copy, exported_snapshot |
| `owner_escalation` | local_user, workspace_owner, org_admin, security_owner, compliance_owner, vendor_support |

## Admin paths

| Path | Write posture | Boundary recheck | Default live vs snapshot |
| --- | --- | --- | --- |
| `local_individual` | writes_live | no | live_only |
| `managed_cloud` | writes_live | yes | snapshot_capable |
| `self_hosted` | writes_live | yes | snapshot_capable |
| `sovereign_air_gapped` | local_draft_preserved | yes | snapshot_only |
| `mirrored_offline` | publish_later_queued | yes | snapshot_only |
| `imported_snapshot` | read_only_replay | no | snapshot_only |

## Shared state vocabulary

| State token | Blocks new managed actions by default | Stale (no-silent-green) downgrade |
| --- | --- | --- |
| `active_enforced` | no | no |
| `locked_by_policy` | yes | no |
| `inherited_default` | no | no |
| `overridden_local` | no | no |
| `unconfirmed_stale` | no | yes |
| `pending_managed_sync` | no | no |
| `signature_unverified` | yes | no |
| `delete_pending` | no | no |
| `delete_blocked_by_hold` | yes | no |
| `delete_receipted` | no | no |
| `export_available_now` | no | no |
| `export_deferred` | no | no |
| `mirror_offline_last_known` | no | no |
| `boundary_changed_recheck_required` | yes | no |
| `imported_snapshot_no_live` | no | no |
| `unknown_requires_review` | no | no |

`unconfirmed_stale` is the no-silent-green downgrade: a would-be-current value
whose backing policy/audit evidence is stale, partial, or cached.

## Invariants (all hold)

| Invariant | Statement |
| --- | --- |
| `admin_plane.canonical_object_identity` | Every surface cites a canonical schema and a producing crate, so docs/help/support/commercial point at the same objects. |
| `admin_plane.proof_packet_mapped` | Every surface maps to a non-empty proof packet, so stable promotion fails when a claimed surface lacks a mapped proof row. |
| `admin_plane.no_silent_green` | Every freshness-headlined surface carries `unconfirmed_stale` and downgrades green on stale/partial/cached evidence. |
| `admin_plane.locked_state_explained` | Every surface that can show a locked control binds policy-source and owner/escalation vocabularies and declares a required ownership field. |
| `admin_plane.ownership_visible` | Every surface binds owner/escalation and declares a required ownership/decision-right field. |
| `admin_plane.delete_export_honest` | Surfaces acting on data bind the delete/export vocabulary and expose a receipt or blocked-by-hold path, never a bare deleted claim. |
| `admin_plane.data_class_located` | Surfaces that capture or export data declare managed-copy versus local-only via the data-residency vocabulary. |
| `admin_plane.verification_posture_explicit` | Surfaces that can show an unverified signature bind the verification/signature vocabulary. |
| `admin_plane.locally_explainable_offline` | Every surface keeps local-safe actions; write-bearing ones offer publish-later capture. |
| `admin_plane.controlled_vocabulary_complete` | Each of the five named controlled vocabularies is bound by at least one surface. |
| `admin_plane.stable_ids_unique` | Surface ids, path ids, and state tokens are defined once and unique. |
| `admin_plane.all_paths_covered` | All six admin paths (local, managed, self-hosted, sovereign/air-gapped, mirrored/offline, imported) are present. |
| `admin_plane.all_surfaces_present` | Every surface family is present exactly once. |
| `admin_plane.typed_not_portal_only` | Every surface is typed and locally explainable; never portal-only or console-only. |

## How to regenerate / verify

```sh
# Regenerate the fixture from the in-code builder
cargo run -p aureline-policy --example dump_m5_admin_plane > \
  fixtures/admin/m5-admin-plane/canonical_matrix.json

# Freeze gate: in-code matrix must equal the checked-in fixture
cargo test -p aureline-policy --test m5_admin_plane

# Human-readable projection
cargo run -p aureline-policy --example dump_m5_admin_plane -- --lines
```
