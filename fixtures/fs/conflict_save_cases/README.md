# Conflict-save corpus (filesystem fixtures)

This directory seeds a reviewer-facing corpus of **conflict-aware save**
scenarios. Each YAML file is one `conflict_save_case` focused on:

- staging and fidelity validation,
- compare-before-write mismatch/uncertainty handling,
- explicit preferred vs degraded write lane disclosure,
- the review-choice set (compare/overwrite/merge/cancel/reload/retry/save_as),
- and the checkpoint/journal expectations that keep recovery safe.

Companion artifacts:

- `artifacts/fs/conflict_save_sequence.md`
- `artifacts/fs/save_review_choice_matrix.yaml`

## Schema (lightweight, reviewer-facing)

Each fixture is one YAML document with:

- `schema_version: 1`
- `case_id`, `case_family: conflict_save_case`
- `title`, `summary`
- `scenario_axes` (buffer dirtiness, external observation kind, uncertainty)
- `save_pipeline` (staging, fidelity, compare-before-write, preferred vs degraded lane)
- `review` (offered choices, forbidden reasons, diff/checkpoint expectations)
- `checkpoint_and_journal_expectations` (what must be captured so recovery is attributable)

The choice vocabulary is frozen by `artifacts/fs/save_review_choice_matrix.yaml`.

## Index

| Case id | Focus |
|---|---|
| `corpus.fs.conflict_save.external_rewrite_dirty_buffer` | External rewrite during edit; save blocked pending review. |
| `corpus.fs.conflict_save.move_or_rename_during_edit` | External move/rename; wrong-target prevention and alias convergence. |
| `corpus.fs.conflict_save.alias_path_ambiguity` | Alias ambiguity; uncertainty must remain visible and block overwrite. |
| `corpus.fs.conflict_save.watcher_uncertainty` | Watcher uncertainty; retry/refresh cannot silently relabel as current. |
| `corpus.fs.conflict_save.remote_root_degradation` | Remote-root degradation/disconnect; no blind overwrite. |
| `corpus.fs.conflict_save.post_resolution_journal_checkpoint` | Post-resolution checkpoint capture and attribution. |

