# Signature-help and snippet-session model

## Release evidence

This artifact documents the one canonical, frozen, export-safe signature-help and
snippet-session model produced by
`crates/aureline-editor/src/m5_signature_snippet/`. It binds a signature-help
**card** and a snippet-session **strip** for every claimed editor family into one
governed typing-loop contract, each derived from — and proven to mirror — the
canonical `SignatureHelpRecord` / `SnippetSessionRecord` shared assist packets.
Editor, CLI/headless, support-export, and AI-evidence consumers render this model
rather than inventing per-pane signature / snippet behavior.

The model is the input-meaning honesty lane for mid-typing assistance: it makes
every surface truthful about **what the next keystroke will mean** (active overload
and parameter; placeholder count, active index, and exit path), **how composition
behaves** (IME and multi-cursor coherence or an explicit narrow-to-one-target),
and **what accepting actually does** (the side-effect cue and whether a preview is
required).

## Record family

| Record | Kind | Schema | Version |
|---|---|---|---|
| `SignatureSnippetModel` | `m5_signature_snippet_model` | `schemas/editor/m5-signature-snippet.schema.json` | 1 |
| `SignatureSnippetSnapshot` | `m5_signature_snippet_snapshot` | (nested) | 1 |
| `SignatureCard` | `m5_signature_card` | (nested) | 1 |
| `SnippetStrip` | `m5_snippet_strip` | (nested) | 1 |

- Model id: `m5-signature-snippet:model:0001`
- As of: `2026-06-22T00:00:00Z`
- Coverage: 9 signature cards and 9 snippet strips across 10 claimed editor families
- Overall: all 19 invariants hold

## Reused canonical packets

The model does not fork the assist contracts. Each card and strip **embeds** the
canonical `AssistSourceDescriptor` (provider id, support, freshness, locality,
scope, degraded state) for provenance and **derives** the canonical
`SignatureHelpRecord` / `SnippetSessionRecord` from the same data, so the surface
binding and the shared session model cannot drift (the `cards_mirror_canonical_record`
and `strips_mirror_canonical_record` invariants). The snippet strip reuses the
`SnippetSessionRecord` Tab / IME / cursor semantics — including `captures_tab`,
`is_keyboard_and_ime_safe`, and the `SnippetSessionController` traversal — rather
than re-implementing placeholder traversal.

## Honesty invariants (all must pass)

1. `every_surface_has_card_or_strip` — each claimed editor family resolves at least a card or strip.
2. `signature_never_obscures_active_line` — no signature card overlaps the active editor line.
3. `visible_signature_is_typing_loop_safe` — every visible card stays non-blocking and IME-safe.
4. `visible_signature_exposes_active_parameter` — every visible card exposes its active parameter within the count.
5. `overloaded_signature_exposes_active_overload` — every overloaded card exposes its active overload within the count.
6. `stale_signature_discloses_limited_cue` — every stale card discloses a limited / refresh-pending cue.
7. `snippet_never_hijacks_tab_invisibly` — every Tab-capturing strip keeps a visible strip and discloses the capture.
8. `active_snippet_exposes_exit_path` — every active strip exposes a visible exit path and a coherent active placeholder index.
9. `ime_multicursor_coherent_or_degraded` — every strip stays coherent or degrades to one disclosed composition target.
10. `accept_side_effects_disclose_before_commit` — every accept beyond the target range discloses before commit with a summary or preview.
11. `generated_and_dependency_effects_require_preview` — generated-output and dependency effects require preview.
12. `blocked_items_carry_reason_and_disclose` — every snapshot with a blocked card or strip flags disclosure.
13. `degraded_surfaces_label_and_disclose` — every non-full-fidelity surface carries a visible label and flags disclosure.
14. `every_card_and_strip_source_labeled` — every card and strip carries a non-empty source label.
15. `every_card_and_strip_keyboard_reachable` — every card and strip is keyboard reachable.
16. `every_card_and_strip_screen_reader_meaningful` — every card and strip carries a non-empty screen-reader label.
17. `cards_mirror_canonical_record` — each card's canonical record mirrors its overload, parameter, placement, and source.
18. `strips_mirror_canonical_record` — each strip's canonical record mirrors its state, placeholder, posture, and source.
19. `first_consumers_prove_shared_model` — the notebook, request, SQL, and docs-code surfaces each resolve a card or strip.

## Surface coverage

Generated and pinned in `fixtures/editor/m5-signature-snippet/canonical_model.json`.

| Surface | Posture | Signature card | Snippet strip | Snippet IME | Accept side effect |
|---|---|---|---|---|---|
| code_file | full_fidelity | visible_overloaded (2/3, param 2/3) | active, captures Tab | no_composition | adds_import |
| config_file | full_fidelity | — | active | no_composition | adds_dependency (preview) |
| notebook_cell | full_fidelity | visible_single (param 1/2) | active | composition_primary_caret_only | edits_target_range_only |
| request_editor | full_fidelity | visible_single (param 1/2) | active | no_composition | edits_target_range_only |
| sql_editor | source_labeled_fallback | visible_single, provider_unavailable | active | no_composition | edits_target_range_only |
| docs_code_block | source_labeled_fallback | stale_pending_refresh | active | no_composition | edits_target_range_only |
| generated_file | read_only_no_apply | visible_single, restricted_read_only | active, restricted_read_only | no_composition | adds_generated_scaffolding (preview) |
| protected_file | read_only_no_apply | visible_single, restricted_read_only | active, restricted_read_only | no_composition | adds_config_edit (preview) |
| partial_index_state | pending_partial_index | stale_pending_refresh | active | no_composition | edits_target_range_only |
| large_file_restricted | suppressed_large_file | unavailable | — | — | — |

The **notebook_cell** strip is the worked proof of the explicit IME degrade path:
under active composition with three carets it narrows to one primary caret, sets
`composition_disclosure_required`, and announces the narrowing in its
screen-reader label. The **code_file** strip is the worked proof of disclosed Tab
capture (it owns Tab while active and keeps a visible strip). Degraded surfaces
(`source_labeled_fallback`, `read_only_no_apply`, `pending_partial_index`,
`suppressed_large_file`) each carry a visible label and a named block reason.

## Verification

Emit the canonical model:

```sh
cargo run --bin aureline_m5_signature_snippet
cargo run --bin aureline_m5_signature_snippet -- --lines
```

Run the freeze gate (rebuilds the model and asserts it equals the fixture):

```sh
cargo test -p aureline-editor --test m5_signature_snippet_replay
```

Run the unit contract suite:

```sh
cargo test -p aureline-editor m5_signature_snippet
```

## Risks and follow-ups

- **The model is a contract, not a live binding.** The resolved snapshots are the
  declared policy; wiring each live editor surface (notebook, request/SQL,
  docs-code, generated, protected) to render the card / strip is incremental
  follow-up.
- **Postures are illustrative for the corpus.** Each surface pins one
  representative posture and one representative card / strip; the live router and
  session manager decide the posture and traversal per keystroke from the same
  provider arbitration and `SnippetSessionController` this model reuses.
- **IME `composition_blocked` is documented, not exercised.** The corpus proves the
  coherent and narrow-to-primary-caret paths; the fully-blocked path is in the
  catalog for downstream consumers but not surfaced in a snapshot here.
- **Assist source-label, signature/snippet record, and surface vocabularies are
  reused, not re-proved here.** Their own contracts remain the source of truth.
