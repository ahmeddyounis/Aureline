# M5 AI-Review-Assist Ops Contract

Batch **B151** freeze-matrix (row **M05-1265**). This is the canonical, checked-in
contract for how Aureline prepares, scopes, publishes, stores, and reopens AI review
findings across local and provider-backed review flows. It opens the AI-review-finding-row,
review-scope-selector, publish-to-review-sheet, and resolution-memory-row batch.

The authoritative gate is the Rust validator in
`crates/aureline-ui/src/m5_ai_review_assist_matrix/`. The schemas under
`schemas/review/` document the shape; the checked-in artifacts, dashboard, and fixtures
are minted from the seed builder by the headless emitter and must never be hand-edited.

## Governed object classes

| Object class | Publish state | Canonical domain schema |
| --- | --- | --- |
| `ai_review_finding_row` | `local_draft` | `schemas/review/m5-ai-review-finding.schema.json` |
| `review_scope_selector` | `local_draft` | `schemas/review/m5-ai-review-scope-selector.schema.json` |
| `publish_to_review_sheet` | `publish_now_provider_comment` | `schemas/review/m5-ai-review-publish-sheet.schema.json` |
| `resolution_memory_row` | `export_fallback_offline` | `schemas/review/m5-ai-review-resolution-memory.schema.json` |

## Frozen vocabulary

- **Roles** (`M5AiReviewAssistRole`): `finding_classification`,
  `analyzed_scope_disclosure`, `publish_destination_disclosure`,
  `local_versus_provider_state` (the four hard-gate postures), plus
  `lifecycle_state_tracking`, `publish_export_fallback`, `resolution_memory_disclosure`.
- **Publish state** (`M5AiReviewAssistPublishState`): `local_draft`,
  `publish_now_provider_comment`, `publish_now_suggested_patch`,
  `publish_now_check_annotation`, `open_in_provider`, `export_fallback_offline`.
  `is_provider_committed()` is `true` for the four provider-writing states and `false`
  for `local_draft` and `export_fallback_offline` — this makes local-draft versus
  provider-committed state mechanically distinct (AC2).
- **Finding lifecycle** (`M5AiReviewAssistFindingLifecycle`): `open`, `dismissed`,
  `published`, `outdated`, `suppressed`, `rerun_recommended`. `is_stale_or_suppressed()`
  is `true` for `outdated` and `suppressed`, so no surface can show a stale finding as
  current (AC3).
- **Classification stages**: `finding_produced`, `scope_resolved`,
  `publish_destination_selected`, `publish_or_export_resolved`, `resolution_recorded`.
- **Consumer surfaces**: `review_detail`, `ai_review_panel`, `finding_row`,
  `review_scope_selector`, `publish_to_review_sheet`, `pending_review_tray`,
  `provider_publish_review`, `resolution_memory_ledger`, `support_export_packet`.

## Required visible state (per row)

Every row must carry a complete `required_visible_state`: `finding_label`,
`finding_class_and_severity`, `analyzed_scope`, `publish_destination`,
`local_versus_provider_state`, `lifecycle_state`, and `publish_export_fallback`.

## Hard invariants (all MUST be `false`)

1. `lets_ai_review_results_publish_or_merge_implicitly`
2. `hides_whether_output_stays_local_or_becomes_a_provider_comment_suggested_patch_or_check_annotation`
3. `keeps_stale_findings_looking_current_after_diff_or_instruction_drift`
4. `loses_local_drafts_or_evidence_when_provider_write_scope_is_missing_or_publish_fails`
5. `presents_an_ai_review_finding_without_its_analyzed_scope_publish_destination_or_lifecycle_state`

## Checked-in artifacts

- Support export: `artifacts/review/m5-ai-review-publish-packets/support_export.json`
- Matrix CSV: `artifacts/review/m5-ai-review-publish-packets/matrix.csv`
- Design report: `artifacts/review/m5-ai-review-assist-components.md`
- Health dashboard: `dashboards/m5-ai-review-assist-health.json`
- Narrowed fixtures: `fixtures/review/m5-ai-review-assist/publish_sheet_beta_narrowed.json`
  and `.../resolution_memory_preview_narrowed.json`

## Re-minting

```text
cargo run -p aureline-ui --example dump_m5_ai_review_assist_matrix -- support-export
cargo run -p aureline-ui --example dump_m5_ai_review_assist_matrix -- csv
cargo run -p aureline-ui --example dump_m5_ai_review_assist_matrix -- report
cargo run -p aureline-ui --example dump_m5_ai_review_assist_matrix -- dashboard
cargo run -p aureline-ui --example dump_m5_ai_review_assist_matrix -- fixture-publish-sheet-beta-narrowed
cargo run -p aureline-ui --example dump_m5_ai_review_assist_matrix -- fixture-resolution-memory-preview-narrowed
cargo run -p aureline-ui --example dump_m5_ai_review_assist_matrix -- validate
```

## Scope

This row is **matrix-only** (governance freeze). It defines the object, state, and field
model and maps it to concrete consumer surfaces; it does not build the consuming surfaces
or widen M5 into autonomous review approval / merge behavior. Later B151 rows implement
the finding / scope-selector / publish-sheet / resolution-memory registries, accessibility
parity, consumer adoption, and a surface-certification capstone to close B151.
