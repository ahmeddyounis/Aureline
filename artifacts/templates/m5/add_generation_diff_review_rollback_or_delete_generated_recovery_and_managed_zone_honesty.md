# Generation Diff Review, Rollback/Delete-Generated Recovery, and Managed-Zone Honesty

- Packet: `generation-recovery:stable:0001`
- Label: `Generation Diff Review, Rollback/Delete-Generated Recovery, and Managed-Zone Honesty`
- Rows: 4 (2 admitted for recovery)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-07T00:00:00Z)

## Rows

- **template:first_party.rust.cli_tool:01** `0.4.2`: generated_only / officially_supported
  - Scope: Freshly generated first-party Rust CLI starter; the generation diff is previewed before any write and a pre-generation checkpoint exists so the whole generated tree can be rolled back without touching authored files
  - Diff review: preview_ready (overwrite guard: no_overwrite_needed)
  - Recovery: rollback_to_checkpoint (authored protected: true, admitted: true)
- **template:community.node.backend_service:02** `2.1.0`: mixed_authored_and_generated / community_supported
  - Scope: Generated Node backend whose authored source and generated scaffolding live in distinct zones; deleting the generated zone leaves authored files intact, and any overwrite needs a three-way review against the previewed diff
  - Diff review: preview_required_before_write (overwrite guard: overwrite_requires_three_way_review)
  - Recovery: delete_generated_only (authored protected: true, admitted: true)
- **template:community.python.web_framework_pack:04** `0.9.0`: generated_then_user_edited / bridge_behavior
  - Scope: Framework-pack scaffold that bridges some files through heuristic mapping rather than exact first-party generation; the bridge behavior and its known issues stay labeled, recovery quarantines the hand-edited generated zone, and the row is held for review instead of being offered as exact truth
  - Diff review: preview_required_before_write (overwrite guard: overwrite_requires_user_selected_files)
  - Recovery: quarantine_generated (authored protected: true, admitted: false)
- **template:imported.unknown_origin:00** `0.0.0`: zone_unknown_review_required / support_unknown
  - Scope: Imported project with no resolvable scaffold lineage; the generation diff cannot be computed, overwrite and recovery are blocked, and the row is labeled lineage-unknown rather than hidden or silently overwritten
  - Diff review: diff_unavailable_review_required (overwrite guard: overwrite_blocked_lineage_unknown)
  - Recovery: recovery_blocked_lineage_unknown (authored protected: true, admitted: false)
