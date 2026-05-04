# Provider-object and traceability-link fixtures

Worked cases for the contract frozen in
[`/docs/work_items/provider_object_and_traceability_contract.md`](../../../docs/work_items/provider_object_and_traceability_contract.md).

Each fixture is a self-contained YAML document carrying one record
that is schema-valid against one of the two boundary schemas:

- [`/schemas/work_items/provider_object.schema.json`](../../../schemas/work_items/provider_object.schema.json)
- [`/schemas/work_items/traceability_link.schema.json`](../../../schemas/work_items/traceability_link.schema.json)

Every fixture carries only opaque workspace / branch / revision /
provider-host / provider-tenant / provider-issue / actor /
account-mapping / callback-envelope / queue-item / browser-handoff
packet / change-object / patch-stack / review-workspace / review-pack
/ review-evaluation-result / support-bundle / incident-workspace /
freshness-floor / policy-epoch / execution-context handles plus
monotonic placeholder timestamps and redaction-aware reviewable
labels (no raw provider URLs, no raw provider issue bodies, no raw
comment bodies, no raw label values that disclose customer / tenant
identity, no raw delegated tokens, no raw branch / commit URLs, no
raw author identity strings, no raw absolute paths, no raw
notification payloads).

## Coverage matrix

| Fixture                                                              | Record                                                | Authority / origin lane                                                                                          | Acceptance bullet(s) covered                                                                                                       |
|----------------------------------------------------------------------|-------------------------------------------------------|------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------|
| `provider_authoritative_synced_projection.yaml`                      | `provider_object_record`                              | `provider_authoritative_synced` / `provider_callback_envelope_validated` / `live_authoritative_fresh`            | Per-field truth fully provider-authoritative; users can tell the projection is provider truth.                                     |
| `provider_issue_linked_to_branch.yaml`                               | `traceability_link_record`                            | `work_item_to_branch_or_worktree` / `provider_authoritative_link_validated` / `live_authoritative_fresh`         | Provider issue linked to branch through a stable typed link; bidirectional pair validated by callback envelope.                    |
| `provider_issue_linked_to_review_workspace.yaml`                     | `traceability_link_record`                            | `work_item_to_review_workspace` / `provider_overlay_with_local_overlay_synced` / `derived_from_review_workspace_link` | Provider issue linked to review workspace through a stable typed link; both sides agree.                                           |
| `stale_cached_provider_object_projection.yaml`                       | `provider_object_record`                              | `provider_authoritative_stale_local_continues` / `degraded_beyond_grace_local_continues`                          | Stale cached issue continues honestly past the freshness floor; surface MUST NOT relabel as live-authoritative-fresh.              |
| `policy_blocked_cached_read_only_shadow_projection.yaml`             | `provider_object_record`                              | `cached_read_only_shadow_inspect_only` / `cached_read_only_shadow_no_refresh`                                    | Policy-blocked provider mutation rendered as cached read-only shadow; locally_proposed_overlay rows forbidden.                     |
| `policy_blocked_provider_mutation_revoked_link.yaml`                 | `traceability_link_record`                            | `link_revoked_by_policy` / `revoked_by_policy` / typed escape hatch via browser-handoff                          | Policy-blocked provider mutation rendered as a revoked-by-policy traceability link with a typed reason and escape hatch.           |
| `conflicting_local_vs_provider_field_state.yaml`                     | `provider_object_record`                              | `mapping_conflict_unresolved` / `mapping_conflict_unresolved_local_vs_provider`                                  | Conflicting local-vs-provider field state surfaced explicitly; both values are rendered side by side per conflicting field row.   |
| `mapping_pending_user_resolution_projection.yaml`                    | `provider_object_record`                              | `mapping_pending_user_resolution` / `account_mapping_binding_pending`                                            | Pending-account-mapping resolution surfaced honestly; surface offers reselect-account as the next safe action.                     |
| `imported_handoff_evidence_only_projection.yaml`                     | `provider_object_record`                              | `imported_evidence_only_no_live_provider_path` / `imported_from_support_export_no_live_provider_path`            | Imported-evidence-only projection with no live provider path; bound support bundle is the only routable carrier.                   |
| `work_item_to_other_work_item_blocking_link.yaml`                    | `traceability_link_record`                            | `work_item_to_other_work_item` / `source_blocks_target`                                                          | Cross-issue blocking link with typed blocking_relationship_class.                                                                  |
| `work_item_to_validation_evidence_link.yaml`                         | `traceability_link_record`                            | `work_item_to_validation_evidence` / `local_authoritative_link_no_provider_overlay`                              | Locally-authoritative link to a review evaluation result carrying validation evidence.                                             |
| `work_item_to_incident_workspace_imported_link.yaml`                 | `traceability_link_record`                            | `work_item_to_incident_workspace_packet` / `imported_handoff_evidence_only_link`                                 | Imported-evidence-only incident workspace link; bound packet is the only routable carrier.                                         |
| `superseded_link_after_account_remap.yaml`                           | `traceability_link_record`                            | `superseded` / `account_mapping_binding_minted_link`                                                              | Superseded traceability link after an account remap; preserved for audit through superseded_by_traceability_link_record_id_ref.   |
| `silent_promotion_to_provider_authoritative_synced_denied.yaml`      | `provider_object_audit_event_record`                  | denial: `silent_promotion_to_provider_authoritative_synced_forbidden`                                            | Silent promotion of a stale projection to provider-authoritative-synced is denied with a typed reason.                             |
| `silent_promotion_to_provider_authoritative_link_denied.yaml`        | `traceability_link_audit_event_record`                | denial: `silent_promotion_to_provider_authoritative_link_validated_forbidden`                                    | Silent promotion of a pending link to provider-authoritative-link-validated is denied with a typed reason.                         |

## Truthful escape hatches

Every degraded fixture exposes at least one truthful escape hatch
named through the typed vocabularies (see contract section 3):

- **Open externally** — `linked_browser_handoff_packet_ref` on the
  policy-blocked traceability link and on the mapping-conflict
  block.
- **Export packet** — `linked_support_bundle_record_id_ref` on
  the imported-evidence projection;
  `linked_incident_workspace_packet_record_id_ref` on the
  imported-evidence incident-workspace link.
- **View sync diagnostics** — both record families carry an
  audit-event vocabulary (`provider_object_*`,
  `traceability_link_*`) that lets queue-review, support-export,
  and admin-reconciliation surfaces render typed sync diagnostics
  rather than generic "something failed" copy.
- **Resolve mapping conflict** — typed `next_safe_action_class`
  on the conflicting projection (`resolve_mapping_conflict`) and
  on the mapping-conflict block (`resolve_mapping_conflict`).
- **Reselect account** — `reselect_account` next safe action on
  the mapping-pending projection.

## Cross-record lineage

The fixtures form a small lineage:

- `work_items:provider_object:01` is the provider-authoritative-synced
  projection backing `work_items:detail:01`;
  `work_items:traceability_link:01` is its issue-to-branch link;
  `work_items:traceability_link:02` is its issue-to-review-workspace
  link; `work_items:traceability_link:04` is its outbound blocking
  link to `work_items:detail:04`;
  `work_items:traceability_link:05` is its link to a review
  evaluation result carrying validation evidence;
  `work_items:traceability_link:07` is its superseded
  review-pack link.
- `work_items:provider_object:02` is the stale cached projection
  backing `work_items:detail:02`.
- `work_items:provider_object:03` is the policy-blocked cached
  read-only shadow projection backing `work_items:detail:03`;
  `work_items:traceability_link:03` is its revoked-by-policy
  publish-later queue-item link.
- `work_items:provider_object:04` is the mapping-conflict-unresolved
  projection backing `work_items:detail:04` (cross-referenced from
  the blocking link above).
- `work_items:provider_object:05` is the mapping-pending-
  user-resolution projection (no detail row admitted yet).
- `work_items:provider_object:06` is the imported-evidence-only
  projection backing `work_items:detail:06`;
  `work_items:traceability_link:06` is its imported-evidence
  incident-workspace link.
