# Editor-assist micro-surface matrix

One canonical, frozen, export-safe **matrix** for editor assistance: decorations,
code lenses, inlay hints, completion, signature help, snippet sessions, and
hover/peek across every editor surface the product claims. Before this matrix,
each pane was free to invent its own assist-source labels, precedence order, and
degraded-state copy. The matrix stops that drift: the editor shell, the headless
CLI emitter, Help/About, support export, and AI evidence surfaces all render the
verdict it freezes instead of re-deriving per-pane micro-behavior.

The matrix is subordinate to editing truth. Diagnostics, the current debug frame,
merge conflicts, review change markers, breakpoints, search matches, and
selection occurrences outrank every convenience layer (code lenses, inlay hints,
the inline completion ghost, hover cards, parameter hints). Deterministic,
cached/lexical, snippet, and AI-backed assist sources stay distinct; constrained
files narrow or block unsafe assist classes **visibly**; hover/peek cards
preserve source, provider, freshness, and raw-versus-rendered truth; and every
offered channel stays keyboard-reachable.

- Schema: [`schemas/editor/m5-editor-assist.schema.json`](../../schemas/editor/m5-editor-assist.schema.json)
- Canonical fixture: [`fixtures/editor/m5-editor-assist/canonical_matrix.json`](../../fixtures/editor/m5-editor-assist/canonical_matrix.json)
- Rust truth source: `crates/aureline-editor/src/m5_editor_assist`
- Headless emitter: `cargo run --bin aureline_m5_editor_assist`
- Freeze gate: `cargo test -p aureline-editor --test m5_editor_assist_replay`

This model reuses the source-label vocabulary frozen by the
[assist source / completion / snippet contracts](assist_and_quickfix_beta.md) and
the constrained-file posture frozen by [large-file mode](large_file_mode.md). It
does not fork a second hint/hover vocabulary.

## What the matrix carries

### Precedence ladder

A single ordered ladder of draw layers, highest precedence first. Each layer is
tagged with a **truth tier** — `editing_truth` or `convenience_metadata` — and
whether it is suppressible under a constrained surface. Editing truth is never
suppressed and never outranked. The matrix proves
(`precedence_truth_outranks_convenience`) that every editing-truth layer ranks
above every convenience-metadata layer.

| Rank band | Layers | Tier |
| --- | --- | --- |
| 0–8 | current debug frame, error diagnostics, merge conflicts, review change markers, warning diagnostics, breakpoints, info/hint diagnostics, search matches, selection occurrences | editing truth |
| 9–13 | code lenses, inlay hints, inline completion ghost, hover cards, parameter hints | convenience metadata |

### Class catalogs

Closed, stable vocabularies, each entry carrying a stable `class_token`, a label,
and a note:

- **Decoration classes** — each maps to an editing-truth precedence layer
  (proved by `decorations_are_editing_truth`).
- **Code-lens classes** — reference/implementation counts, run/debug, test
  status, VCS authorship, AI explain, generated-source origin. AI lenses must
  carry an explicit AI label.
- **Inlay-hint classes** — parameter name, inferred type, chained-call type, enum
  value, implicit conversion, and AI-inferred (always AI-labeled).
- **Completion source kinds** — reuse the shared source-label classes
  (`deterministic_language`, `cached_fallback`, `snippet_origin`,
  `ai_inline_assist`, `project_graph`, `framework_provider`, `tool_adapter`).
- **Signature-help states** — hidden, visible (single / overloaded), stale
  pending refresh, unavailable.
- **Snippet-session states** — reuse the shared snippet lifecycle classes
  (inactive, active, exited, cancelled).
- **Hover/peek modes** — hover quick-info, pinned hover, and the peek family
  (definition, references, implementations, type definition, call hierarchy).
- **Degraded-state classes** — the closed vocabulary every surface narrows
  through (see below).

### The surface matrix

Every claimed surface binds **exactly one cell per assist channel**
(`every_surface_covers_every_channel`). A cell names the degraded-state class the
surface narrows that channel to, whether the channel stays keyboard-reachable,
and a disclosure string.

The degraded-state vocabulary:

| Degrade state | Meaning |
| --- | --- |
| `full_fidelity` | All sources available. |
| `source_labeled_fallback` | Available but limited to a labeled fallback source. |
| `read_only_no_apply` | Shown for reading; apply blocked and disclosed. |
| `suppressed_large_file` | Suppressed in large-file / restricted mode. |
| `pending_partial_index` | Labeled pending while the semantic index builds. |
| `blocked_unavailable` | Not offered on this surface. |

The per-surface policy:

| Surface | Constrained | Highlights |
| --- | --- | --- |
| `code_file` | no | Full fidelity on every channel. |
| `config_file` | no | Schema-backed; no run/reference lens, no cross-file peek. |
| `notebook_cell` | yes | Per-cell scope; cross-cell lens and peek are best-effort fallback. |
| `request_editor` | yes | Variable/header assist from the request schema; no symbol peek. |
| `sql_editor` | yes | Dialect + introspected-schema backed; degraded versus a full LSP. |
| `docs_code_block` | yes | Best-effort by language; lenses, inlay hints, and peek are unavailable. |
| `generated_file` | yes | Apply blocked (regenerate route); reading stays full. |
| `protected_file` | yes | Apply blocked (protected-path review); reading stays full. |
| `partial_index_state` | yes | Semantic channels labeled pending; snippet stays full. |
| `large_file_restricted` | yes | Convenience assist suppressed; decorations reduced, not dropped. |

### Identity & lifecycle contracts

Stable id prefixes and required lifecycle fields for each micro-surface kind:
completion sessions (`completion-session:`), hint descriptors (`hint:`),
hover/peek cards (`hover-peek:`), snippet sessions (`snippet-session:`), and
degraded assist states (`assist-degrade:`). Defined once for every consumer.

### Support / export minimums

The fields each micro-surface record must carry into a support export. Every
export minimum sets `raw_payload_excluded` — identity, source label, and degraded
state only; no credential bodies or raw provider payloads.

## Invariants

The matrix evaluates these over its own data and records the result in
`invariants[].holds`; the freeze gate re-proves every one:

1. `precedence_truth_outranks_convenience`
2. `every_surface_covers_every_channel`
3. `constrained_surfaces_narrow_visibly`
4. `apply_blocked_where_writes_route_elsewhere`
5. `large_file_suppresses_convenience_assist`
6. `partial_index_pends_semantic_channels`
7. `offered_channels_stay_keyboard_reachable`
8. `decorations_are_editing_truth`
9. `identity_contracts_cover_every_micro_surface`

## Consuming the matrix

The matrix is a single record. Editor, CLI/headless, support, and AI-evidence
consumers should read the canonical fixture (or call `editor_assist_matrix()`),
look up a surface with `surface_profile(...)`, look up a cell with
`cell(surface, channel)`, and render the degraded-state class and disclosure
directly — never re-deciding per pane. The human-readable projection
(`editor_assist_matrix_lines`) is shared by Help/About, the CLI emitter, and
support export.

## Regenerating

The fixture is generated, not hand-edited:

```sh
cargo run --bin aureline_m5_editor_assist > fixtures/editor/m5-editor-assist/canonical_matrix.json
```

If you change the matrix in code without regenerating, the freeze gate
(`m5_editor_assist_replay`) fails.
