# Assist-descriptor model

One canonical, frozen, export-safe **descriptor model** for the three editor
micro-surfaces that draw inline metadata: **decorations**, **code lenses**, and
**inlay hints**. Where the [editor-assist matrix](m5-editor-assist.md) freezes the
*vocabulary* — the precedence ladder, the class catalogs, and the per-surface
degraded-state policy — this model freezes the *typed descriptor shape* every
claimed editor surface renders through, plus the deterministic resolver that
turns a descriptor and a render context into a visibility verdict with an
explicit suppression reason and accessibility truth.

Before this model, each pane was free to invent its own decoration / lens / hint
object, its own precedence handling, and its own ad hoc reason for hiding a hint.
The model stops that drift: decorations, code lenses, and inlay hints share one
[`AssistDescriptor`] shape, one resolver, and one closed set of
[`SuppressionReason`]s, so the editor shell, the headless CLI emitter, Help/About,
support export, and AI evidence surfaces consume one verdict instead of
re-deriving per-pane micro-behavior.

- Schema: [`schemas/editor/m5-assist-descriptors.schema.json`](../../schemas/editor/m5-assist-descriptors.schema.json)
- Canonical fixture: [`fixtures/editor/m5-assist-descriptors/canonical_model.json`](../../fixtures/editor/m5-assist-descriptors/canonical_model.json)
- Rust truth source: `crates/aureline-editor/src/m5_assist_descriptors`
- Headless emitter: `cargo run --bin aureline_m5_assist_descriptors`
- Freeze gate: `cargo test -p aureline-editor --test m5_assist_descriptors_replay`

The model **consumes** the frozen editor-assist matrix for its per-surface
degraded-state policy and **reuses** the assist source-label vocabulary and the
frozen hint-descriptor id prefix rather than forking new ones. It does not
redesign the editor renderer; it freezes the typed descriptor / precedence /
suppression / accessibility contract those renderers implement.

## The descriptor shape

Every decoration, code lens, and inlay hint is one [`AssistDescriptor`]:

| Field group | Fields | Why |
|---|---|---|
| Identity & class | `descriptor_id`, `family`, `class_token`, `class_label` | One id space; lens/hint ids reuse the frozen `hint:` prefix. |
| Precedence | `owning_layer`, `truth_tier`, `channel` | Inherits the frozen ladder rank; drives matrix degrade lookup. |
| Target | `anchor`, `placement` | Where the metadata is drawn relative to the text. |
| Provenance | `source` (source-label class, provider, freshness, confidence, AI/visual-distinction flags) | Keeps the source visible and drives low-confidence suppression. |
| Actionability | `actionability`, `command_ref` | Severity-bearing vs activatable vs informational. |
| Accessibility | `accessibility` (screen-reader label, non-color differentiator, keyboard path, motion class) | Never color-only, never mouse-only. |
| Suppression policy | `layout_shifting`, `density_optional`, `zoom_optional` | Drives the density / zoom / typing-budget rules. |

Decorations are editing truth (diagnostics, debug frame, conflict, review,
search, selection, breakpoints, diff). Code lenses and inlay hints are
convenience metadata. The model proves decorations never shift layout and are
never compacted, while lenses and hints always shift layout and are subject to
the suppression rules.

## The resolver

[`assist_descriptor_model`](../../crates/aureline-editor/src/m5_assist_descriptors)
resolves every descriptor against each render context in a fixed precedence
order:

1. **Surface policy** — the frozen matrix sets the base verdict for the
   descriptor's channel on the context surface (full, source-labeled fallback,
   read-only, pending partial index, suppressed in large-file, or unavailable).
2. **Editing truth stops here** — a decoration is never suppressed for a
   convenience reason. Its only reduction is the labeled large-file fallback
   (`reduced_decoration`), so reduced decorations are still drawn, never dropped.
3. **Convenience refinement** — lenses and hints are then refined, each with an
   explicit reason: `low_confidence` (speculative metadata suppressed by
   default), `density_compaction` (optional metadata at dense spacing),
   `high_zoom_horizontal_budget` (inline metadata under a narrow column budget),
   and `typing_budget` (layout-shifting metadata **held**, not dropped, while the
   user types).

Each [`ResolvedDescriptor`] carries its `visibility` verdict
(`rendered` / `downgraded` / `deferred` / `suppressed`), its `effective_degrade`,
its `suppression_reason` plus human-readable `reason_detail`, whether animations
are enabled (reduced motion forces them off), and whether it stays
keyboard-reachable. A descriptor is keyboard-reachable exactly when it is
rendered or downgraded — held and suppressed descriptors are not.

### Interaction precedence

Precedence is not only a draw order — it governs interaction. When an editing-truth
decoration and a convenience descriptor overlap on the same anchor, the
convenience descriptor **yields** ([`PrecedenceConflictCase`]): it is held with
the `outranked_by_editing_truth` reason so the diagnostic, debug line, or conflict
band underneath stays legible.

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

The model resolves the full 23-descriptor catalog under each context. Counts are
pinned in the fixture.

| Scenario | Surface | Outcome |
|---|---|---|
| `code_file_comfortable` | code file | all metadata renders; only the low-confidence AI hint is suppressed |
| `code_file_compact` | code file | compact spacing is non-lossy (only the AI hint suppressed) |
| `code_file_dense` | code file | optional inlay hints + authorship lens compacted (`density_compaction`) |
| `code_file_high_zoom` | code file | inline type hints dropped (`high_zoom_horizontal_budget`) |
| `code_file_typing` | code file | all convenience metadata deferred (`typing_budget`); decorations keep drawing |
| `code_file_reduced_motion` | code file | animations disabled across the board |
| `sql_editor_comfortable` | SQL editor | lenses / hints labeled fallback (`source_fallback`) |
| `docs_code_block_comfortable` | docs-code block | lenses / hints unavailable (`unavailable_on_surface`) |
| `partial_index` | partial-index state | semantic lenses / hints pending (`partial_index_pending`) |
| `generated_file` | generated file | reading metadata renders (apply blocking is a completion-channel concern) |
| `large_file_restricted` | large-file mode | convenience suppressed (`large_file_restricted`); decorations reduced |

## Verification

```sh
cargo run --bin aureline_m5_assist_descriptors            # JSON
cargo run --bin aureline_m5_assist_descriptors -- --lines # human-readable

cargo test -p aureline-editor --test m5_assist_descriptors_replay   # freeze gate
cargo test -p aureline-editor m5_assist_descriptors                 # unit contracts
```

## Risks and follow-ups

- **The model is a contract, not a live binding.** The resolved scenarios are the
  declared policy; wiring each live editor surface to read the descriptor model
  instead of its own ad hoc decoration/lens/hint logic is incremental follow-up.
- **Typing defers newly-resolved layout-shifting metadata.** The corpus models the
  conservative policy for a fresh batch arriving mid-typing; retaining
  already-painted hints (which do not jump) is the live editor's job and stays out
  of this static corpus.
- **Apply-blocking on generated / protected files is a completion-channel
  concern.** Decorations, lenses, and inlay hints do not mutate the buffer, so the
  model renders them for reading on those surfaces; the matrix's read-only/no-apply
  policy governs the completion / snippet / inline-AI channels instead.
