# Finalize issue and work-item linkage with branch

**Scope:** Bind work-item detail surfaces, status-transition sheets, offline-handoff continuity, branch/review links, and publish-later continuity into a single coherent work-item linkage finalization packet.

**Status:** Stable review lane — implemented in `crates/aureline-review`.

## Goal

Bind review-workspace beta packets, review-stabilization packets, work-item detail surfaces, status-transition sheets, offline-handoff continuities, branch-to-work-item links, review-to-work-item links, publish-later continuities, command-graph operations, and support/export envelopes into a single coherent linkage finalization view. Every surface discloses its write mode, every transition sheet previews provider side effects and preserves local draft on failure, offline handoffs survive restart/reconnect/export-import, and branch/review links are previewable before publish.

## Design principles

1. **Governed work-item detail surfaces with write-mode disclosure** — Every detail surface carries an explicit `write_mode_disclosure_class` so users always know whether they are viewing read-only, comment/link, full-edit, offline-capture-only, or policy-blocked state.
2. **Status-transition sheets preview provider side effects** — Transition sheets enumerate `previewed_side_effects` before any provider mutation occurs and guarantee `local_draft_preserved_on_failure`.
3. **Offline-handoff packets are first-class** — Offline-handoff continuities explicitly declare `survives_restart`, `survives_reconnect`, and `survives_export_import` so handoff state never silently disappears.
4. **Branch and review links previewable before publish** — Both `BranchWorkItemLinkRecord` and `ReviewWorkItemLinkRecord` expose `previewable_before_publish` so users can inspect linkage before it crosses the provider boundary.
5. **Publish-later continuity cites queue items** — Publish-later continuities reference a `publish_later_queue_item_record_id_ref` and disclose `queue_state` and `drain_state` so deferred publishes are observable and retryable.
6. **Redaction-safe support export** — Raw URLs and raw provider payloads are explicitly forbidden from crossing the support boundary.

## Record kinds

| Record kind | Purpose |
|---|---|
| `work_item_linkage_finalization_packet` | Top-level packet consumed by review surfaces and support exports. |
| `work_item_linkage_finalization_record` | Core linkage finalization binding workspace, stabilization, and finalization state. |
| `work_item_detail_surface_record` | Governed work-item detail surface with write-mode disclosure and freshness. |
| `status_transition_sheet_record` | Status-transition sheet with previewed side effects and local-draft preservation. |
| `offline_handoff_continuity_record` | Offline-handoff continuity with restart/reconnect/export-import survival. |
| `branch_work_item_link_record` | Branch-to-work-item link with previewability before publish. |
| `review_work_item_link_record` | Review-to-work-item link with previewability before publish. |
| `publish_later_continuity_record` | Publish-later continuity with queue item ref and survival flags. |
| `work_item_linkage_command_record` | Command-graph operations (preview, confirm, save draft, queue publish, export handoff, refresh, invalidate). |
| `work_item_linkage_support_export_packet` | Redaction-safe export with reopen context. |
| `work_item_linkage_inspection_record` | Compact boolean projection for CLI and inspector surfaces. |

## Closed vocabularies

### Finalization states
- `finalized_current`, `finalized_stale_provider_overlay`, `finalized_partial_work_item_scope`, `finalized_diverged_requires_review`, `finalized_offline_handoff_only`

### Write-mode disclosure classes
- `read_only`, `comment_or_link`, `full_edit`, `offline_capture_only`, `policy_blocked`

### Work-item linkage command classes
- `preview_linkage`, `confirm_status_transition`, `save_local_draft`, `queue_publish_later`, `export_offline_handoff`, `refresh_provider_overlay`, `invalidate_linkage`

### Consumer surfaces
- `review_workspace_inspector`, `review_landing_strip`, `cli_headless_entry`, `support_export`, `audit_lane`, `browser_companion`, `offline_handoff`

## Key invariants

- `finalized_current` state requires at least one detail surface and all detail surfaces must disclose their write mode.
- Every transition sheet must reference a known detail surface by `detail_surface_id`.
- Offline-handoff continuities must survive at least restart and reconnect.
- Branch and review links must be previewable before publish when the finalization state is `finalized_current`.
- Publish-later continuities must cite a valid queue item and survive restart.
- Support/export records keep every `raw_*_export_allowed` flag false.
