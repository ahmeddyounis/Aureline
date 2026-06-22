# Assist-descriptor model

## Release evidence

This artifact documents the one canonical, frozen, export-safe decoration /
code-lens / inlay-hint descriptor model produced by
`crates/aureline-editor/src/m5_assist_descriptors/`. It materializes a single
typed [`AssistDescriptor`] shape for all three micro-surface families and a
deterministic resolver that turns a descriptor and a render context into a
visibility verdict with an explicit suppression reason and keyboard /
screen-reader / non-color accessibility truth. It consumes the frozen
[editor-assist matrix](m5-editor-assist.md) for per-surface degraded-state policy
so editor, CLI/headless, support-export, and AI-evidence consumers render one
verdict rather than inventing per-pane micro-behavior.

## Record family

| Record | Kind | Schema | Version |
|---|---|---|---|
| `AssistDescriptorModel` | `m5_assist_descriptor_model` | `schemas/editor/m5-assist-descriptors.schema.json` | 1 |

- Model id: `m5-assist-descriptors:model:0001`
- As of: `2026-06-22T00:00:00Z`
- Coverage: 23 descriptors (10 decorations, 7 code lenses, 6 inlay hints) resolved across 11 scenarios and 3 precedence-conflict cases
- Overall: all 15 invariants hold

## Honesty invariants (all must pass)

1. `descriptor_catalog_covers_every_class` — exactly one descriptor per decoration, code-lens, and inlay-hint class.
2. `editing_truth_never_convenience_suppressed` — no decoration is suppressed or deferred for a convenience reason in any scenario.
3. `convenience_outranked_by_truth_when_rendered` — every rendered convenience descriptor ranks below every rendered editing-truth descriptor.
4. `non_rendered_resolutions_carry_reason` — every downgraded, deferred, or suppressed resolution carries an explicit reason and detail.
5. `rendered_resolutions_have_no_suppression_reason` — full renders carry the not-suppressed reason.
6. `actionable_or_severity_decorations_fully_accessible` — every actionable or severity-bearing decoration declares a keyboard path, screen-reader label, and non-color differentiator.
7. `every_descriptor_has_non_color_and_screen_reader` — every descriptor is non-color-differentiable and screen-reader-labeled.
8. `ai_descriptors_carry_ai_label` — AI-sourced descriptors are labeled AI inline assist and kept visually distinct.
9. `reduced_motion_disables_animation` — the reduced-motion scenario enables no animation.
10. `large_file_suppresses_convenience_keeps_decorations` — large-file mode suppresses every convenience descriptor and keeps every decoration.
11. `low_confidence_convenience_suppressed` — low-confidence convenience metadata is suppressed by default on a full-fidelity code file.
12. `typing_defers_layout_shifting_convenience` — no layout-shifting convenience descriptor renders while typing.
13. `keyboard_reachable_iff_offered` — keyboard-reachable exactly when rendered or downgraded.
14. `lens_and_hint_ids_reuse_frozen_prefix` — lens / hint ids reuse the frozen hint-descriptor id prefix.
15. `precedence_conflicts_resolve_to_editing_truth` — every overlap is won by the editing-truth descriptor.

## Scenario coverage

Generated and pinned in `fixtures/editor/m5-assist-descriptors/canonical_model.json`.

| Scenario | Rendered | Downgraded | Deferred | Suppressed | Reason exercised |
|---|---|---|---|---|---|
| code_file_comfortable | 22 | 0 | 0 | 1 | low_confidence |
| code_file_compact | 22 | 0 | 0 | 1 | (compact is non-lossy) |
| code_file_dense | 18 | 0 | 0 | 5 | density_compaction |
| code_file_high_zoom | 19 | 0 | 0 | 4 | high_zoom_horizontal_budget |
| code_file_typing | 10 | 0 | 12 | 1 | typing_budget |
| code_file_reduced_motion | 22 | 0 | 0 | 1 | (animations disabled) |
| sql_editor_comfortable | 10 | 12 | 0 | 1 | source_fallback |
| docs_code_block_comfortable | 10 | 0 | 0 | 13 | unavailable_on_surface |
| partial_index | 10 | 12 | 0 | 1 | partial_index_pending |
| generated_file | 22 | 0 | 0 | 1 | (reading metadata renders) |
| large_file_restricted | 0 | 10 | 0 | 13 | large_file_restricted / reduced_decoration |

The three precedence-conflict cases exercise `outranked_by_editing_truth`:
diagnostic underline over an inferred-type inlay hint, the current debug line over
a reference-count lens, and a merge-conflict region over a parameter-name hint.

## Verification

Emit the canonical model:

```sh
cargo run --bin aureline_m5_assist_descriptors
cargo run --bin aureline_m5_assist_descriptors -- --lines
```

Run the freeze gate (rebuilds the model and asserts it equals the fixture):

```sh
cargo test -p aureline-editor --test m5_assist_descriptors_replay
```

Run the unit contract suite:

```sh
cargo test -p aureline-editor m5_assist_descriptors
```

## Risks and follow-ups

- **The model is a contract, not a live binding.** The resolved scenarios are the
  declared policy; wiring each live editor surface (notebook, request/SQL,
  docs-code, generated, protected) to read the descriptor model instead of its own
  ad hoc decoration/lens/hint logic is incremental follow-up.
- **Typing defers newly-resolved layout-shifting metadata.** The corpus models the
  conservative policy for a fresh batch arriving mid-typing; retaining
  already-painted hints (which do not jump) is the live editor's job.
- **Source-label, snippet-state, and matrix vocabularies are reused, not
  re-proved here.** The model references the assist source-label classes, the
  frozen hint-descriptor id prefix, and the editor-assist matrix degrade policy;
  their own contracts remain the source of truth for those vocabularies.
