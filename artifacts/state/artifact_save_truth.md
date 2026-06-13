# Artifact save truth

This report freezes metadata-sensitive save cues, fallback disclosure, and no-silent-stomp guards for generated, exported, draft, and remote artifact families.

## Summary

- packet_id: state.artifact_save_truth.v1
- row_count: 11
- fixture_count: 11
- preferred_atomic_row_count: 2
- regenerate_or_export_row_count: 6
- draft_stage_row_count: 2
- compare_only_blocked_row_count: 1
- logical_target_indicator_row_count: 11
- rebase_or_abort_mutator_count: 16

## Rows

| Row | Preferred path | Alternates | Indicators | Disclosure | Rebase/abort mutators |
| --- | --- | --- | --- | --- | --- |
| `notebook_document` | `atomic_replace` | `conditional_remote_write, in_place_write` | `encoding, newline_mode, bom_or_final_newline, logical_target_ambiguity` | `atomic_with_disclosed_fallback` | `format_on_save, organize_imports, refactor_apply, ai_apply` |
| `notebook_output_artifact` | `regenerate_from_source` | `save_as_copy` | `generated_state_boundary, logical_target_ambiguity` | `regenerate_or_export_disclosed` | `generated_output_write` |
| `request_workspace_document` | `atomic_replace` | `conditional_remote_write, in_place_write` | `encoding, newline_mode, bom_or_final_newline, execute_bit_or_permissions, logical_target_ambiguity` | `atomic_with_disclosed_fallback` | `format_on_save, organize_imports, refactor_apply, ai_apply` |
| `request_response_snapshot` | `save_as_copy` | `` | `generated_state_boundary, logical_target_ambiguity` | `regenerate_or_export_disclosed` | `` |
| `database_export_artifact` | `regenerate_from_source` | `save_as_copy` | `generated_state_boundary, logical_target_ambiguity` | `regenerate_or_export_disclosed` | `generated_output_write` |
| `profiler_trace_artifact` | `save_as_copy` | `` | `encoding, generated_state_boundary, logical_target_ambiguity` | `regenerate_or_export_disclosed` | `` |
| `preview_output_artifact` | `regenerate_from_source` | `save_as_copy` | `generated_state_boundary, logical_target_ambiguity` | `regenerate_or_export_disclosed` | `generated_output_write` |
| `sync_packet_artifact` | `stage_local_draft` | `save_as_copy` | `encoding, newline_mode, bom_or_final_newline, generated_state_boundary, logical_target_ambiguity` | `draft_stage_disclosed` | `format_on_save, ai_apply, generated_output_write` |
| `provider_local_draft` | `stage_local_draft` | `save_as_copy` | `encoding, newline_mode, bom_or_final_newline, logical_target_ambiguity` | `draft_stage_disclosed` | `format_on_save, ai_apply` |
| `infrastructure_overlay_document` | `compare_only_blocked` | `save_as_copy` | `generated_state_boundary, logical_target_ambiguity, execute_bit_or_permissions` | `compare_only_blocked` | `` |
| `imported_archive_capture` | `save_as_copy` | `` | `encoding, generated_state_boundary, logical_target_ambiguity` | `regenerate_or_export_disclosed` | `` |

## Fixture coverage

- `notebook_document`: A notebook document saved on a local root keeps encoding, newline, BOM/final-newline, and logical-target cues visible while format-on-save must rebase or abort on any external drift. Evidence: `metadata_preservation_disclosure, mid_flight_drift_rebase_required`.
- `notebook_output_artifact`: A notebook output refresh routes through regeneration, not exact file save, and the generated-output writer must rebase or abort if the backing result changed. Evidence: `export_or_regenerate_not_exact_save, mid_flight_drift_rebase_required`.
- `request_workspace_document`: A remote request-workspace script falls back from atomic replace to conditional remote write while execute-bit retention stays explicit and AI apply must rebase or abort. Evidence: `execute_bit_retention, mid_flight_drift_rebase_required`.
- `request_response_snapshot`: A provider-backed response snapshot only exports by save-as copy after compare-before-save review; it never impersonates an exact editable file. Evidence: `export_or_regenerate_not_exact_save`.
- `database_export_artifact`: A database export refresh is regenerate-from-source rather than in-place overwrite, and the export writer must rebase or abort if the query basis changed. Evidence: `export_or_regenerate_not_exact_save, mid_flight_drift_rebase_required`.
- `profiler_trace_artifact`: An imported profiler trace surfaces lossy-decode risk before any text conversion and only leaves the archive as an exported copy. Evidence: `lossy_decode_risk, export_or_regenerate_not_exact_save`.
- `preview_output_artifact`: A container-backed preview output refresh uses regeneration, keeps generated-state cues visible, and aborts instead of stomping newer output lineage. Evidence: `export_or_regenerate_not_exact_save, mid_flight_drift_rebase_required`.
- `sync_packet_artifact`: An offline sync packet stages a local draft, keeps compare-before-save visible before replay, and requires generated-output refresh to rebase or abort on reconnect drift. Evidence: `metadata_preservation_disclosure, mid_flight_drift_rebase_required`.
- `provider_local_draft`: A provider-linked local draft keeps logical-target ambiguity visible, stages locally first, and makes AI apply rebase or abort before any publish-later replay. Evidence: `logical_target_ambiguity_disclosure, mid_flight_drift_rebase_required`.
- `infrastructure_overlay_document`: A provider-backed infrastructure overlay blocks ordinary save, keeps logical-target ambiguity visible, and routes the user through compare-first review or alternate-target export only. Evidence: `logical_target_ambiguity_disclosure`.
- `imported_archive_capture`: An imported archive capture exposes lossy-decode risk and only allows save-as copy after compare-before-save review; it never claims exact source-file semantics. Evidence: `lossy_decode_risk, export_or_regenerate_not_exact_save`.
