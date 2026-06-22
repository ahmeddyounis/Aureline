# Completion-row model

One canonical, frozen, export-safe **completion-row model** for the editor
families that surface completion: **code**, **config**, **notebook**, **request**,
**SQL**, **docs code-blocks**, and the **generated**, **protected**,
**partial-index**, and **large-file** states. Where the
[editor-assist matrix](m5-editor-assist.md) freezes the per-surface degraded-state
*policy* and the [assist-descriptor model](m5-assist-descriptors.md) freezes the
inline-metadata *descriptor shape*, this model freezes the **completion row** —
the single shape every claimed editor renders one suggestion through, and the
commit-honesty contract that comes with it.

Before this model, each pane was free to invent its own row: one that
distinguished a language-server result from a local-word guess by a ranking tweak
alone, another that applied an auto-import without saying so, a third that let an
AI proposal inherit the same trust weight as deterministic semantic completion.
The model stops that drift: every row carries its source kind, provider identity,
a deterministic-versus-AI assist class with a pinned trust weight, an
additional-edit / import cue, availability, and documentation truth, so the editor
shell, the headless CLI emitter, Help/About, support export, and AI evidence
surfaces consume one row instead of re-deriving per-pane behavior.

- Schema: [`schemas/editor/m5-completion-rows.schema.json`](../../schemas/editor/m5-completion-rows.schema.json)
- Canonical fixture: [`fixtures/editor/m5-completion-rows/canonical_model.json`](../../fixtures/editor/m5-completion-rows/canonical_model.json)
- Rust truth source: `crates/aureline-editor/src/m5_completion_rows`
- Headless emitter: `cargo run --bin aureline_m5_completion_rows`
- Freeze gate: `cargo test -p aureline-editor --test m5_completion_rows_replay`

The model **reuses** the assist contracts rather than forking them: each row
embeds the canonical `AssistSourceDescriptor` for provenance and projects back
into the shared `CompletionItemRecord` / `CompletionListSnapshot`, and the
deterministic `CompletionAssistClass` is the row-facing refinement of the shared
`AssistSourceLabelClass`. It does not redesign the completion engine; it freezes
the row / provenance / commit-disclosure / degraded-label contract the engine
implements.

## The row shape

Every suggestion is one [`CompletionRow`]:

| Field group | Fields | Why |
|---|---|---|
| Identity & label | `row_id`, `primary_label`, `kind_class`, `kind_icon_token` | One id space; kind drives the icon token. |
| Source & trust | `assist_class`, `trust_weight`, `source` | The deterministic-versus-AI distinction, pinned in data — not styling. |
| Commit honesty | `additional_edit_cue`, `additional_edit_summary`, `commit_disclosure_required`, `preview_required` | What accepting does beyond the current range, disclosed before commit. |
| Availability & docs | `availability`, `docs_available`, `docs_command_ref` | Deprecated / unavailable rows are marked; docs availability is explicit. |
| Accessibility | `requires_visual_distinction`, `non_color_differentiator`, `accessibility_label` | Source class is never color-only; every row is screen-reader labeled. |

## The deterministic-versus-AI distinction

[`CompletionAssistClass`] splits suggestions into eight classes, each pinned to a
[`TrustWeightClass`] so a user never infers the difference from styling alone:

| Assist class | Trust weight | Kept visually distinct |
|---|---|---|
| `deterministic_language` | `full_semantic` | no |
| `project_graph` | `full_semantic` | no |
| `framework_provider` | `full_semantic` | no |
| `tool_adapter` | `advisory` | no |
| `snippet_only` | `advisory` | yes |
| `ai_backed` | `advisory` | yes |
| `cached_fallback` | `heuristic_fallback` | yes |
| `local_word` | `heuristic_fallback` | yes |

The guardrail is provable: `ai_backed` and `local_word` rows can never reach
`full_semantic`, so AI ghost text or a local-word fallback never inherits the
trust weight of deterministic semantic completion.

## The additional-edit / import cue

[`AdditionalEditCue`] states, before commit, what accepting a row does beyond the
current insertion range:

| Cue | Meaning | Pre-commit disclosure |
|---|---|---|
| `none` | edits only the current range | — |
| `additional_edits_in_file` | adds further edits in this file | required |
| `adds_import` | adds or rewrites an import | required |
| `adds_dependency` | adds or changes a dependency | required + preview |
| `edits_config` | edits configuration elsewhere | required |
| `generated_output_effect` | changes generated output via its generator | required + preview |

A row that adds imports, edits, dependencies, config, or generated-output effects
sets `commit_disclosure_required`, and the dependency / generated-output cues also
require a preview before the broader edit applies.

## Degraded provider postures

Each [`CompletionRowSnapshot`] pins one [`CompletionProviderPosture`] and a
visible label, so a degraded path is never a silent ranking regression:

| Posture | Visible label |
|---|---|
| `full_semantic` | Full semantic completion |
| `degraded_provider` | Degraded provider — fallback results |
| `stale_partial_index` | Index still building — partial results |
| `restricted_mode` | Restricted mode — limited assist |
| `large_file_fallback` | Large-file mode — lexical fallback only |
| `offline_cached_only` | Offline — cached results only |

## Surfaces covered

`code_file`, `config_file`, `notebook_cell`, `request_editor`, `sql_editor`,
`docs_code_block`, `generated_file`, `protected_file`, `partial_index_state`, and
`large_file_restricted` — 30 rows total. The `sql_editor`, `docs_code_block`,
`protected_file`, `partial_index_state`, and `large_file_restricted` surfaces
exercise the degraded postures; `large_file_restricted` and `docs_code_block` also
carry an `unavailable` row that is inspect-only and marked.

## Honesty invariants

The model proves 15 invariants over its own data (see
[the release artifact](../../artifacts/editor/m5-completion-rows.md)), including
that AI-backed and local-word rows never carry full-semantic trust, that every
additional-edit effect discloses before commit, that generated-output and
dependency effects require preview, that every degraded posture is labeled, and
that each snapshot's rows mirror the canonical assist list one-for-one.

## What this model is not

- **Not a live binding.** The snapshots are the declared policy; wiring each live
  editor surface to render the completion-row model is incremental follow-up.
- **Not a re-design of the assist vocabulary.** Source-label classes, the
  completion acceptance contract, and the editor-surface catalog are reused; their
  own contracts remain authoritative.
- **Not M6 agent planning.** The model stays inside completion-row truth and
  commit semantics.
