# Editor-assist micro-surface matrix

## Release evidence

This artifact documents the one canonical, frozen, export-safe editor-assist
micro-surface matrix produced by `crates/aureline-editor/src/m5_editor_assist/`.
It freezes precedence, class catalogs, the surface × channel degraded-state
matrix, identity/lifecycle contracts, and support-export minimums so editor,
CLI/headless, support-export, and AI-evidence consumers render one verdict rather
than inventing per-pane micro-behavior.

## Record family

| Record | Kind | Schema | Version |
|---|---|---|---|
| `EditorAssistMatrix` | `m5_editor_assist_matrix` | `schemas/editor/m5-editor-assist.schema.json` | 1 |

- Matrix id: `m5-editor-assist:matrix:0001`
- As of: `2026-06-22T00:00:00Z`
- Coverage: 10 surfaces × 9 channels, 14-layer precedence ladder
- Overall: all 9 invariants hold

## Honesty invariants (all must pass)

1. `precedence_truth_outranks_convenience` — every editing-truth layer outranks every convenience-metadata layer.
2. `every_surface_covers_every_channel` — each surface binds exactly one cell per assist channel.
3. `constrained_surfaces_narrow_visibly` — every constrained surface narrows or blocks at least one channel through the shared degraded-state vocabulary.
4. `apply_blocked_where_writes_route_elsewhere` — generated and protected surfaces never expose full-fidelity apply on completion, snippet, or inline-AI channels.
5. `large_file_suppresses_convenience_assist` — large-file / restricted mode suppresses every convenience channel and keeps only reduced decorations.
6. `partial_index_pends_semantic_channels` — partial-index state labels every semantic channel pending.
7. `offered_channels_stay_keyboard_reachable` — every offered cell stays keyboard-reachable; only blocked cells are not.
8. `decorations_are_editing_truth` — every decoration class is owned by an editing-truth precedence layer.
9. `identity_contracts_cover_every_micro_surface` — every micro-surface kind has an id prefix and required lifecycle fields.

## Surface coverage

Generated and pinned in `fixtures/editor/m5-editor-assist/canonical_matrix.json`.

| Surface | Constrained | Narrowed channels |
|---|---|---|
| code_file | no | none (full fidelity) |
| config_file | no | code_lens, peek (source-labeled fallback) |
| notebook_cell | yes | code_lens, peek (source-labeled fallback) |
| request_editor | yes | completion/signature/code_lens/inlay/hover (fallback), peek (blocked) |
| sql_editor | yes | completion/signature/code_lens/inlay/hover/peek (fallback) |
| docs_code_block | yes | completion/signature/hover (fallback); code_lens/inlay/peek (blocked) |
| generated_file | yes | completion/snippet/inline-AI (read-only, no apply) |
| protected_file | yes | completion/snippet/inline-AI (read-only, no apply) |
| partial_index_state | yes | all semantic channels (pending); inline-AI (fallback) |
| large_file_restricted | yes | all convenience channels (suppressed); decorations (reduced) |

## Verification

Emit the canonical matrix:

```sh
cargo run --bin aureline_m5_editor_assist
cargo run --bin aureline_m5_editor_assist -- --lines
```

Run the freeze gate (rebuilds the matrix and asserts it equals the fixture):

```sh
cargo test -p aureline-editor --test m5_editor_assist_replay
```

Run the unit contract suite:

```sh
cargo test -p aureline-editor m5_editor_assist
```

## Risks and follow-ups

- **The matrix is a contract, not a live binding.** The per-surface degraded
  states are the declared policy; wiring each live editor surface (notebook,
  request/SQL, docs-code, generated, protected) to read the matrix instead of its
  own ad hoc logic is incremental follow-up as those surfaces mature.
- **Source-label and snippet-state vocabularies are reused, not re-proved here.**
  The matrix references the assist source-label and snippet lifecycle classes;
  their own contracts remain the source of truth for those vocabularies.
- **IME / accessibility behavior is represented as keyboard-reachability and
  reduced-decoration labels.** Full IME composition behavior for snippet sessions
  remains owned by the assist snippet model; this matrix records that offered
  channels stay keyboard-reachable and that large-file decorations are labeled
  reduced rather than silently dropped.
