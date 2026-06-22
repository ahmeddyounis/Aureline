# Advanced-editing micro-surface model

## Release evidence

This artifact documents the one canonical, frozen, export-safe advanced-editing
micro-surface model produced by `crates/aureline-editor/src/m5_advanced_editing/`.
It binds the orientation-aid surfaces that sit *around* the editing task —
selection-summary strips, fold-state risk markers, and minimap / overview-ruler
aids — into one governed contract for every claimed advanced editor. Editor,
CLI/headless, support-export, and AI-evidence consumers render this model rather
than inventing per-pane selection / fold / minimap behavior.

The model is the advanced-editing honesty lane: it makes every surface truthful
about **what the next edit will do** (selection semantics: exact for all
selections, normalized / expanded, primary-caret-only, or blocked, plus an
explanation for any operation that cannot apply to every caret), **what a folded
region hides** (diagnostics, conflicts, trust / policy warnings advertised with a
non-colour marker and a reveal route, never falsely appearing clean), and **how
overview aids relate to the main editor** (optional accelerators pinned to the same
marker-semantics source, never the sole carrier of critical state, degrading
honestly in constrained profiles). Every micro-surface stays keyboard-complete,
non-colour-only, and density / zoom / motion aware.

## Record family

| Record | Kind | Schema | Version |
|---|---|---|---|
| `AdvancedEditingModel` | `m5_advanced_editing_model` | `schemas/editor/m5-advanced-editing.schema.json` | 1 |
| `AdvancedEditorSnapshot` | `m5_advanced_editing_snapshot` | (nested) | 1 |
| `SelectionSummaryStrip` | `m5_selection_summary_strip` | (nested) | 1 |
| `FoldRiskSummary` | `m5_fold_risk_summary` | (nested) | 1 |
| `OverviewAidParity` | `m5_overview_aid_parity` | (nested) | 1 |

- Model id: `m5-advanced-editing:model:0001`
- As of: `2026-06-22T00:00:00Z`
- Coverage: 10 advanced editor surfaces, one snapshot each
- Overall: all 17 invariants hold

## Reused canonical packets

The model does not fork the editor's existing vocabulary. It **reuses** the
`EditorSurfaceClass` and `AssistDegradeClass` catalogs from the editor-assist
matrix, the `HiddenStateCounts`, `OverviewAidKind`, and `OrientationAidAvailability`
records from the carried-forward orientation foundations, and the `DensityTier`,
`ZoomTier`, and `MotionClass` render tiers from the assist-descriptor model. Notebook
and request editors resolve the **same** `SelectionSummaryStrip` / `FoldRiskSummary`
/ `OverviewAidParity` record kinds rather than bolting on their own selection / fold
/ minimap semantics (the `notebook_and_request_reuse_shared_vocabulary` invariant).
Every overview aid pins the shared `marker-semantics:main-editor:diagnostics-conflict-trust`
source, so an aid cannot diverge into a second hidden truth model.

## Honesty invariants (all must pass)

1. `every_surface_resolves_a_snapshot` — each claimed advanced editor surface resolves exactly one snapshot.
2. `selection_semantics_always_disclosed` — every strip names its semantics class and discloses any non-exact semantics.
3. `multi_caret_strips_show_count_and_primary` — every multi-cursor / column strip shows a caret count, a primary caret, and an undo grouping.
4. `unsupported_operations_explained` — every narrowed / blocked selection explains an unsupported operation with a reason and a fallback route.
5. `folds_advertise_hidden_critical_state` — every fold hiding diagnostics / conflicts / trust warnings advertises it with a marker and a reveal route.
6. `fold_risk_class_matches_counts` — every fold-risk class is derived correctly from its hidden-state counts.
7. `every_fold_keyboard_toggleable` — every fold summary stays keyboard-toggleable, labeled, and non-colour-marked.
8. `overview_aids_not_sole_carrier` — every minimap / overview aid is an optional accelerator, never the sole carrier of critical state, and names replacement routes.
9. `overview_aids_aligned_with_main_editor` — every overview aid pins the shared main-editor marker-semantics source.
10. `degraded_aids_have_alternate_path` — every reduced / disabled aid carries a visible message and an alternate path.
11. `severity_state_non_color_only` — every strip, fold, and aid carries a non-colour differentiator for severity / actionability.
12. `render_awareness_preserves_critical_state` — every density / zoom / motion policy preserves critical state and non-colour differentiation.
13. `snapshots_preserve_critical_state_in_profile` — every snapshot preserves critical state under its captured profile.
14. `every_surface_screen_reader_meaningful` — every strip, fold, and aid carries a non-empty screen-reader label.
15. `degraded_surfaces_label_and_disclose` — every non-full-fidelity surface carries a visible label and flags disclosure.
16. `every_surface_has_strip_and_overview_aids` — every snapshot resolves a selection strip and at least one overview aid.
17. `notebook_and_request_reuse_shared_vocabulary` — notebook and request editors reuse the shared record kinds, not forked semantics.

## Surface coverage

Generated and pinned in `fixtures/editor/m5-advanced-editing/canonical_model.json`.

| Surface | Posture | Sel. mode | Semantics | Folds (risk) | Minimap | Overview ruler | Profile (density/zoom/motion) |
|---|---|---|---|---|---|---|---|
| code_file | full_fidelity | multi_cursor | exact_all_selections | hidden_critical; clean | available | available | comfortable/standard/animated_reducible |
| config_file | full_fidelity | column_block | exact_all_selections | hidden_informational | available | available | compact/standard/static |
| notebook_cell | full_fidelity | multi_cursor | normalized_expanded | hidden_critical | reduced | available | comfortable/standard/static |
| request_editor | full_fidelity | multi_cursor | primary_caret_only | clean | disabled_by_setting | reduced | compact/standard/static |
| sql_editor | full_fidelity | column_block | exact_all_selections | hidden_critical | available | available | comfortable/standard/static |
| docs_code_block | full_fidelity | single_caret | normalized_expanded | hidden_informational | disabled_by_setting | reduced | comfortable/standard/static |
| generated_file | read_only_no_apply | multi_cursor | blocked | hidden_critical | available | available | comfortable/standard/static |
| protected_file | read_only_no_apply | multi_cursor | blocked | hidden_critical | available | available | comfortable/standard/static |
| partial_index_state | pending_partial_index | multi_cursor | exact_all_selections | hidden_critical | reduced | reduced | comfortable/high/static |
| large_file_restricted | suppressed_large_file | single_caret | primary_caret_only | — (folding suppressed) | disabled_large_file | disabled_large_file | comfortable/standard/static |

The **config_file** and **sql_editor** strips are the worked proof of a
`column_block` edit with a column-edit preview that applies exactly to every row;
the **notebook_cell** strip of a multi-cursor edit normalized to the cell with a
disclosed cross-cell limit; the **request_editor** strip of a `primary_caret_only`
edit because one caret lands in a read-only resolved section; the **generated_file**
and **protected_file** strips of a `blocked` edit whose fallback routes open the
generator source or request approval; the **large_file_restricted** strip of
multi-cursor suppression in restricted mode. The **code_file**,
**notebook_cell**, **sql_editor**, **generated_file**, **protected_file**, and
**partial_index_state** folds prove hidden critical state (diagnostics, conflicts,
trust warnings) advertised with a reveal route; the **large_file_restricted**
surface proves folding suppression. Degraded postures (`read_only_no_apply`,
`pending_partial_index`, `suppressed_large_file`) each carry a visible label and
flag disclosure, and every reduced / disabled overview aid names an alternate path.

## Verification

Emit the canonical model:

```sh
cargo run --bin aureline_m5_advanced_editing
cargo run --bin aureline_m5_advanced_editing -- --lines
```

Run the freeze gate (rebuilds the model and asserts it equals the fixture):

```sh
cargo test -p aureline-editor --test m5_advanced_editing_replay
```

Run the unit contract suite:

```sh
cargo test -p aureline-editor m5_advanced_editing
```

## Risks and follow-ups

- **The model is a contract, not a live binding.** The resolved snapshots are the
  declared policy; wiring each live editor (notebook, request/SQL, docs-code,
  generated, protected, large-file) to render the selection strip, fold summaries,
  and overview aids is incremental follow-up.
- **Postures are illustrative for the corpus.** Each surface pins one representative
  strip, fold set, and aid set; the live editor decides the selection mode,
  semantics, fold risk, and aid availability per session from the same sources this
  model reuses.
- **Marker semantics are referenced by id, not re-proved here.** The shared
  diagnostics / conflict / trust marker source remains the source of truth; this
  model carries its ref and proves aids align to it.
- **Surface, degrade, hidden-state, overview-aid, and render-tier vocabularies are
  reused, not re-proved here.** Their own contracts remain authoritative.
