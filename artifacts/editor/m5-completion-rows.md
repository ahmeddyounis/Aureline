# Completion-row model

## Release evidence

This artifact documents the one canonical, frozen, export-safe completion-row
model produced by `crates/aureline-editor/src/m5_completion_rows/`. It
materializes a single source-labeled, commit-honest `CompletionRow` for every
claimed editor family and resolves one `CompletionRowSnapshot` per family, each
pinning a visible provider posture and deriving the canonical shared
[`CompletionListSnapshot`](../../crates/aureline-editor/src/assist/) from the same
rows. Editor, CLI/headless, support-export, and AI-evidence consumers render this
model rather than inventing per-pane completion-row behavior.

The model is the commit-honesty lane for completion: it makes every row truthful
about **where the suggestion came from** (source kind and provider identity),
**what happens on accept** (the additional-edit / import cue and whether a
preview is required), and **how trustworthy or degraded the current path is** (a
pinned trust weight and a visible per-surface provider posture).

## Record family

| Record | Kind | Schema | Version |
|---|---|---|---|
| `CompletionRowModel` | `m5_completion_row_model` | `schemas/editor/m5-completion-rows.schema.json` | 1 |
| `CompletionRowSnapshot` | `m5_completion_row_snapshot` | (nested) | 1 |
| `CompletionRow` | `m5_completion_row` | (nested) | 1 |

- Model id: `m5-completion-rows:model:0001`
- As of: `2026-06-22T00:00:00Z`
- Coverage: 30 rows across 10 claimed editor families
- Overall: all 15 invariants hold

## Reused canonical packets

The row does not fork the assist contracts. Each row **embeds** the canonical
`AssistSourceDescriptor` (provider id, support, freshness, locality, scope,
degraded state) for provenance and projects back into the shared
`CompletionItemRecord` / `CompletionListSnapshot` via `to_canonical_item`, so the
row presentation and the shared assist packet cannot drift. The deterministic
`CompletionAssistClass` is the row-facing refinement of the shared
`AssistSourceLabelClass`, additionally splitting a lexical local-word guess out of
the broader cached-fallback class.

## Honesty invariants (all must pass)

1. `every_surface_family_has_rows` — each claimed editor family resolves at least one row.
2. `ai_never_full_semantic_trust` — no AI-backed row carries full-semantic trust weight.
3. `local_word_never_full_semantic_trust` — no local-word row carries full-semantic trust weight.
4. `trust_weight_tracks_assist_class` — every row's trust weight equals the weight pinned to its assist class.
5. `additional_edit_rows_disclose_before_commit` — every acceptable row with an additional-edit cue discloses it before commit.
6. `generated_and_dependency_effects_require_preview` — rows that change generated output or a dependency require preview.
7. `degraded_postures_label_fallback` — every degraded provider posture carries a visible fallback label and flags disclosure.
8. `deterministic_and_ai_are_distinct` — at least one surface exposes both a deterministic and an AI-backed row with differing class and trust weight.
9. `every_row_carries_source_label` — every row carries a non-empty source label so provider/source is visible.
10. `rows_match_canonical_snapshot` — each snapshot's canonical assist list has one item per row with the same source and id.
11. `non_available_rows_are_marked` — every deprecated or unavailable row carries a non-color marker and visual distinction.
12. `distinct_classes_have_non_color_differentiator` — every row requiring visual distinction carries a non-empty non-color differentiator.
13. `disclosure_implies_summary_or_preview` — every row that must disclose before commit carries a summary or requires preview.
14. `unavailable_rows_are_inspect_only` — every unavailable row resolves to an inspect-only acceptance posture.
15. `assist_class_catalog_complete` — the deterministic / cached / local-word / snippet / AI core classes are each exercised.

## Trust-weight pinning (the deterministic-versus-AI guardrail)

| Assist class | Trust weight | Visually distinct |
|---|---|---|
| `deterministic_language` | `full_semantic` | no |
| `project_graph` | `full_semantic` | no |
| `framework_provider` | `full_semantic` | no |
| `tool_adapter` | `advisory` | no |
| `snippet_only` | `advisory` | yes |
| `ai_backed` | `advisory` | yes |
| `cached_fallback` | `heuristic_fallback` | yes |
| `local_word` | `heuristic_fallback` | yes |

AI-backed and local-word rows can never reach `full_semantic`, so a fallback or
ghost-text suggestion never inherits the visual / trust weight of deterministic
semantic completion.

## Surface coverage

Generated and pinned in `fixtures/editor/m5-completion-rows/canonical_model.json`.

| Surface | Provider posture | Rows | Deterministic | AI | Fallback | Disclose | Preview | Unavailable |
|---|---|---|---|---|---|---|---|---|
| code_file | full_semantic | 6 | 3 | 1 | 1 | 1 | 0 | 0 |
| config_file | full_semantic | 3 | 0 (schema) | 0 | 1 | 1 | 0 | 0 |
| notebook_cell | full_semantic | 3 | 1 | 1 | 1 | 1 | 0 | 0 |
| request_editor | full_semantic | 3 | 0 (schema/env) | 0 | 1 | 0 | 0 | 0 |
| sql_editor | degraded_provider | 3 | 1 | 0 | 2 | 0 | 0 | 0 |
| docs_code_block | restricted_mode | 3 | 1 | 0 | 1 | 0 | 0 | 1 |
| generated_file | full_semantic | 2 | 1 | 0 | 1 | 1 | 1 | 0 |
| protected_file | restricted_mode | 2 | 1 | 0 | 1 | 2 | 2 | 0 |
| partial_index_state | stale_partial_index | 2 | 1 | 0 | 1 | 0 | 0 | 0 |
| large_file_restricted | large_file_fallback | 3 | 1 | 0 | 1 | 0 | 0 | 1 |

Degraded postures (`degraded_provider`, `stale_partial_index`, `restricted_mode`,
`large_file_fallback`) each carry a visible label so a fallback list never appears
as a silent ranking regression. The `code_file` type-import row, the
`config_file` value row, the `notebook_cell` import row, the `generated_file`
symbol row, and both `protected_file` rows exercise the
additional-edit/import/config/generated-output/staged-review disclosure paths.

## Verification

Emit the canonical model:

```sh
cargo run --bin aureline_m5_completion_rows
cargo run --bin aureline_m5_completion_rows -- --lines
```

Run the freeze gate (rebuilds the model and asserts it equals the fixture):

```sh
cargo test -p aureline-editor --test m5_completion_rows_replay
```

Run the unit contract suite:

```sh
cargo test -p aureline-editor m5_completion_rows
```

## Risks and follow-ups

- **The model is a contract, not a live binding.** The resolved snapshots are the
  declared policy; wiring each live editor surface (notebook, request/SQL,
  docs-code, generated, protected) to render the completion-row model instead of
  its own ad hoc list is incremental follow-up.
- **Provider postures are illustrative for the corpus.** Each surface pins one
  representative posture; the live router decides the posture per request from the
  same provider arbitration this model already reuses.
- **Assist source-label, side-effect, and surface vocabularies are reused, not
  re-proved here.** The row references the assist source-label classes, the shared
  completion acceptance contract, and the editor-surface catalog; their own
  contracts remain the source of truth for those vocabularies.
