# Retention/deletion — evidence companion

Human-readable companion to
[`/fixtures/admin/m5-retention-deletion/canonical_retention.json`](../../fixtures/admin/m5-retention-deletion/canonical_retention.json)
and its boundary schema
[`/schemas/admin/m5-retention-deletion.schema.json`](../../schemas/admin/m5-retention-deletion.schema.json).
It gives reviewers the rendered per-profile retention/deletion matrices without
reading the JSON. The contract narrative lives in
[`/docs/admin/m5-retention-deletion.md`](../../docs/admin/m5-retention-deletion.md),
and the frozen object model it binds back to lives in
[`/artifacts/admin/m5-admin-plane.md`](./m5-admin-plane.md).

- Bundle id: `m5-retention-deletion:bundle:0001`
- Record kind: `m5_retention_deletion_bundle`
- Binds matrix: `m5-admin-plane:matrix:0001`
- Profiles: 4 · Rows: 13 · Invariants: 17

## Profiles and coverage

| Profile | Deployment | Coverage state | Completeness | Locally inspectable | Console-independent |
| --- | --- | --- | --- | --- | --- |
| `managed_cloud` | managed_cloud | active_enforced | complete | yes | yes |
| `self_hosted` | self_hosted | active_enforced | complete | yes | yes |
| `sovereign_air_gapped` | sovereign_air_gapped | unconfirmed_stale | partial_imported | yes | yes |
| `mirrored_offline` | managed_cloud | unconfirmed_stale | partial_offline | yes | yes |

Every profile keeps a locally inspectable matrix with no vendor console. The
sovereign profile's imported registry view and the mirrored profile's offline
view are labeled with a non-complete completeness class and an unconfirmed
coverage state rather than presented as a confirmed-complete matrix.

## Retention/deletion rows (data class · location · retention · outcome · state · linkage)

| Profile | Record family | Data class | Location | Retention | Outcome | State | Linkage |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `managed_cloud` | `durable_workspace_state` | user_owned | local_only | user_controlled | immediate | export_available_now | — |
| `managed_cloud` | `collaboration_session_record` | workspace_owned | managed_copy | fixed_window | deferred | delete_pending | privacy_request_case |
| `managed_cloud` | `ai_retained_evidence_packet` | tenant_owned | managed_copy | regulatory_hold | blocked | delete_blocked_by_hold | legal_hold |
| `managed_cloud` | `support_export_packet` | user_owned | exported_snapshot | fixed_window | immediate | delete_receipted | destruction_receipt |
| `self_hosted` | `operational_audit_record` | tenant_owned | managed_copy | regulatory_hold | blocked | delete_blocked_by_hold | legal_hold |
| `self_hosted` | `portable_state_package` | user_owned | local_only | user_controlled | immediate | delete_receipted | destruction_receipt |
| `self_hosted` | `collaboration_review_evidence` | workspace_owned | managed_copy | fixed_window | deferred | delete_pending | partial_delete_reason |
| `sovereign_air_gapped` | `ai_retained_evidence_packet` | tenant_owned | local_only | regulatory_hold | blocked | delete_blocked_by_hold | legal_hold |
| `sovereign_air_gapped` | `imported_audit_snapshot` | imported | local_only | mirror_last_synced | immediate | unconfirmed_stale | — |
| `sovereign_air_gapped` | `derived_offline_index` | derived_cache | local_only | ephemeral_regenerable | immediate | active_enforced | — |
| `mirrored_offline` | `managed_copy_index_entry` | tenant_owned | mirrored_copy | mirror_last_synced | deferred | export_deferred | partial_delete_reason |
| `mirrored_offline` | `sync_mirror_ledger` | derived_cache | mirrored_copy | mirror_last_synced | immediate | unconfirmed_stale | — |
| `mirrored_offline` | `offboarding_exit_packet` | user_owned | local_only | user_controlled | immediate | export_available_now | — |

All five data classes (user-owned, workspace-owned, tenant-owned, imported,
derived-cache) and all three delete outcomes (immediate, deferred, blocked) are
exercised, as are all four linkage classes (destruction receipt, privacy-request
case, legal hold, partial-delete reason). The three stale rows
(`imported_audit_snapshot`, `managed_copy_index_entry`, `sync_mirror_ledger`) are
shown under non-confirmed states (`unconfirmed_stale`, `export_deferred`), never
as confirmed-green. The two receipted rows carry a destruction receipt; the three
hold-blocked rows each name their hold and escalate to a compliance or security
owner.

## Non-immediate deletes explain their remainder

| Row | What remains | Where | Expected completion | Next-step owner |
| --- | --- | --- | --- | --- |
| `…managed_cloud.0002` | managed copy of the session record | managed_copy | within 30 days of the privacy request completing | compliance_owner |
| `…managed_cloud.0003` | full evidence packet | managed_copy | when the legal hold is released | compliance_owner |
| `…self_hosted.0001` | operational audit history | managed_copy | when the security owner releases the hold | security_owner |
| `…self_hosted.0003` | derived references in the review search index | managed_copy | on the next nightly reindex | security_owner |
| `…sovereign.0001` | sealed evidence packet | local_only | when the offline hold seal is lifted | compliance_owner |
| `…mirrored.0001` | upstream managed copy | managed_copy | when the mirror reconnects to the control plane | org_admin |

Every immediate delete carries no remainder; every deferred or blocked delete
names what remains, where, when, and who controls the next step.

## Export parity and propagation

Each matrix offers both a `machine_readable_json` summary export and a
`plain_language_handoff` packet, and every row carries both a machine summary and
a plain-language sentence. Every matrix names propagation into the support export,
the offboarding flow, a compliance packet, and the Help/About public-truth
surface, so the states reach those surfaces unchanged.

## Invariants (all hold)

| Invariant | Statement |
| --- | --- |
| `retention_deletion.surface_states_within_matrix` | Every rendered state is one the frozen matrix admits for the retention/deletion surface. |
| `retention_deletion.retention_route_outcome_complete` | Every row names its retention class, export/delete routes, outcome, state, owner, and governing schema; row ids are unique. |
| `retention_deletion.data_classes_distinguished` | Every data class appears across the bundle. |
| `retention_deletion.non_immediate_explains_remainder` | Every deferred or blocked delete explains what/where/when/who; immediate deletes carry no remainder. |
| `retention_deletion.deletion_linkage_distinct` | Every non-immediate delete links to a specific receipt/case/hold/partial-reason and every linkage class appears. |
| `retention_deletion.delete_export_honest` | Receipted deletes carry a receipt and hold-blocked deletes name their hold. |
| `retention_deletion.location_explicit` | Local-only and hosted locations are both exercised and labeled distinctly. |
| `retention_deletion.export_parity` | Every row carries both export representations and every matrix offers both export forms. |
| `retention_deletion.propagation_complete` | Every matrix names propagation into support export, offboarding, compliance packet, and Help/About public truth. |
| `retention_deletion.no_silent_green` | Stale evidence never sits under a confirmed active/export-available/receipted state. |
| `retention_deletion.ownership_visible` | Every blocked delete escalates to a governance owner other than the local user. |
| `retention_deletion.locally_inspectable_offline` | Every profile keeps a locally inspectable, vendor-console-independent matrix. |
| `retention_deletion.coverage_labeled` | A partial registry view is labeled, never implied complete. |
| `retention_deletion.consumer_parity` | One typed packet serves every consumer the matrix declares for this surface identically. |
| `retention_deletion.profiles_covered` | The managed-cloud, self-hosted, sovereign/air-gapped, and mirrored/offline profiles are all rendered. |
| `retention_deletion.outcomes_all_present` | Immediate, deferred, and blocked outcomes all appear. |
| `retention_deletion.export_safe` | Every stable id is an opaque token and every governing schema is a repo-relative ref. |

## How to regenerate / verify

```sh
# Regenerate the fixture from the in-code builder
cargo run -p aureline-policy --example dump_m5_retention_deletion > \
  fixtures/admin/m5-retention-deletion/canonical_retention.json

# Freeze gate: in-code bundle must equal the checked-in fixture
cargo test -p aureline-policy --test m5_retention_deletion

# Human-readable projection
cargo run -p aureline-policy --example dump_m5_retention_deletion -- --lines
```
