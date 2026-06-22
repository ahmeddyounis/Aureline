# Signature-help and snippet-session model

One canonical, frozen, export-safe model that binds **signature-help cards** and
**snippet-session strips** into a single, low-latency typing-loop contract across
the editor families: **code**, **config**, **notebook**, **request**, **SQL**,
**docs code-blocks**, and the **generated**, **protected**, **partial-index**, and
**large-file** states. Where the [completion-row model](m5-completion-rows.md)
freezes the shared *suggestion row* and the
[editor-assist matrix](m5-editor-assist.md) freezes the per-surface degraded-state
*policy*, this model freezes the two protected mid-typing surfaces that materially
change what the next keystroke means while the user is composing.

Before this model, signature help and snippet sessions were scattered across
provider-specific panes: one pane let snippet mode silently hijack Tab, another
let a stale signature card sit over the active line with no limited cue, a third
dropped a multi-cursor snippet to one composition target with no explanation. The
model folds both into one governed session model so that every surface carries its
source, its active overload / parameter, its placeholder count and exit path, its
IME / multi-cursor posture, and its no-hidden-side-effects truth.

- Schema: [`schemas/editor/m5-signature-snippet.schema.json`](../../schemas/editor/m5-signature-snippet.schema.json)
- Canonical fixture: [`fixtures/editor/m5-signature-snippet/canonical_model.json`](../../fixtures/editor/m5-signature-snippet/canonical_model.json)
- Rust truth source: `crates/aureline-editor/src/m5_signature_snippet`
- Headless emitter: `cargo run --bin aureline_m5_signature_snippet`
- Freeze gate: `cargo test -p aureline-editor --test m5_signature_snippet_replay`

The model **reuses** the assist contracts rather than forking them: each card and
strip embeds the canonical `AssistSourceDescriptor` for provenance and derives the
canonical `SignatureHelpRecord` / `SnippetSessionRecord` (and its
`SnippetSessionController` traversal semantics) from the same data. It folds
snippet and signature help into one session model instead of scattering them
across provider-specific panes; it does not redesign a snippet-authoring DSL.

## The signature card

Every `SignatureCard` always exposes the truth that changes input meaning:

| Field group | Fields | Why |
|---|---|---|
| Identity & state | `card_id`, `state_class` | Hidden / single / overloaded / stale / unavailable. |
| Active position | `active_signature_index`, `signature_count`, `active_parameter_index`, `parameter_count` | The active overload and active parameter are always visible. |
| Placement | `placement_class`, `non_blocking`, `ime_composition_safe`, `obscures_active_line` | The card stays subordinate, IME-safe, and never overlaps the active line. |
| Stale honesty | `stale_disclosed`, `blocked_reason`, `non_color_differentiator` | A stale card shows a limited / refresh-pending cue, not silent stale text. |
| Accept honesty | `accept_side_effect`, `side_effect_summary`, `commit_disclosure_required`, `preview_required` | A signature-derived accept discloses any extra edit before commit. |
| Accessibility | `keyboard_reachable`, `accessibility_label` | Keyboard-reachable, screen-reader meaningful. |
| Provenance | `source`, `canonical_record` | The canonical `SignatureHelpRecord` is derived from the same data. |

## The snippet strip

Every `SnippetStrip` always exposes its placeholder traversal and exit path:

| Field group | Fields | Why |
|---|---|---|
| Identity & state | `strip_id`, `state_class` | Inactive / active / exited / cancelled. |
| Placeholders | `active_placeholder_index`, `placeholder_count` | Placeholder count and active index are always visible. |
| Tab & exit | `tab_behavior_class`, `visible_strip_required`, `tab_capture_disclosed`, `exit_path` | Tab capture is disclosed, never silent; the exit path is always visible. |
| IME & multi-cursor | `ime_posture_class`, `cursor_posture_class`, `selection_count`, `multi_cursor_compatible`, `primary_caret_ref`, `composition_disclosure_required` | Coherent for the whole selection set, or degraded to one disclosed target. |
| Accept honesty | `accept_side_effect`, `side_effect_summary`, `commit_disclosure_required`, `preview_required` | Imports / scaffolding / dependencies / config edits disclose before commit. |
| Accessibility | `keyboard_reachable`, `accessibility_label` | Keyboard-reachable, screen-reader meaningful. |
| Provenance | `source`, `canonical_record` | The canonical `SnippetSessionRecord` is derived from the same data. |

## No-hidden-side-effects truth

`AcceptSideEffectClass` states, before commit, what accepting a snippet or a
signature-derived completion does beyond the target range:

| Cue | Meaning | Pre-commit disclosure |
|---|---|---|
| `edits_target_range_only` | edits only the snippet / signature target | — |
| `adds_import` | adds or rewrites an import | required |
| `adds_generated_scaffolding` | writes generated scaffolding via its generator | required + preview |
| `adds_dependency` | adds or changes a dependency | required + preview |
| `adds_config_edit` | edits configuration elsewhere | required |

## IME and multi-cursor coherence

A snippet session either stays coherent for the whole selection set or degrades
*explicitly* to one composition target:

- `no_composition` / `composition_active_pass_through` — the session stays coherent
  for every caret.
- `composition_primary_caret_only` — multi-cursor composition narrows to one
  primary caret (`primary_caret_ref`), sets `composition_disclosure_required`, and
  announces the narrowing in the strip's screen-reader label. The notebook surface
  proves this path.
- `composition_blocked` — composition cannot continue and traversal pauses with a
  cue (documented; not exercised in the corpus).

## Blocked / degraded reasons

`AssistBlockReason` names why a surface is not full fidelity, so a degraded path is
never a silent regression: `large_file_suppressed`, `partial_index_pending`,
`restricted_read_only` (apply routes through a generator or staged review),
`provider_unavailable`, and `stale_awaiting_refresh`.

## Surfaces covered

`code_file`, `config_file`, `notebook_cell`, `request_editor`, `sql_editor`,
`docs_code_block`, `generated_file`, `protected_file`, `partial_index_state`, and
`large_file_restricted` — 9 signature cards and 9 snippet strips. The
**notebook**, **request**, **SQL**, and **docs-code** surfaces prove the shared
session model directly (the `first_consumers_prove_shared_model` invariant), rather
than shipping isolated implementations. The `code_file` snippet exercises the
multi-cursor Tab-capture path with an import side effect; the `notebook_cell`
snippet exercises the IME narrow-to-primary-caret path; `generated_file` and
`config_file` exercise the preview-required side effects; `docs_code_block` and
`partial_index_state` exercise the stale signature cue; `large_file_restricted`
suppresses both surfaces with a named reason.

## Honesty invariants

The model proves 19 invariants over its own data (see
[the release artifact](../../artifacts/editor/m5-signature-snippet.md)), including
that no card obscures the active line, that every visible card exposes its active
parameter (and overloaded cards their active overload), that stale cards disclose a
limited cue, that snippet mode never hijacks Tab invisibly, that every active strip
exposes a visible exit path, that IME / multi-cursor stays coherent or degrades to
one disclosed target, that every accept side effect discloses before commit, and
that each card and strip mirrors its canonical session record.

## What this model is not

- **Not a live binding.** The snapshots are the declared policy; wiring each live
  editor surface to render the signature card / snippet strip is incremental
  follow-up.
- **Not a snippet-authoring DSL redesign.** The model stays inside session
  semantics, visibility, and the first claimed consumers; placeholder grammars and
  cross-language snippet packs are out of scope.
- **Not a re-design of the assist vocabulary.** Source-label classes, the
  signature-help / snippet-session records, the IME / cursor postures, and the
  editor-surface catalog are reused; their own contracts remain authoritative.
