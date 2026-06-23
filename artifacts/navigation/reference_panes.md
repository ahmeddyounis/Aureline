# References panes — evidence companion

Human-readable companion to
[`/fixtures/navigation/reference_panes/canonical_panes.json`](../../fixtures/navigation/reference_panes/canonical_panes.json)
and its boundary schema
[`/schemas/navigation/reference_panes.schema.json`](../../schemas/navigation/reference_panes.schema.json).
It gives reviewers the frozen scenario and invariant tables without reading the
JSON. The contract narrative lives in
[`/docs/navigation/reference_panes.md`](../../docs/navigation/reference_panes.md).

- Set id: `reference-panes:set:0001`
- Record kind: `reference_panes_set`
- Scenarios: 5 · Invariants: 10

## Pane scenarios

| Scenario | Groups (access kind : count) | Current / Captured | Pane evidence | Proves |
| --- | --- | --- | --- | --- |
| `semantic_read_write_call` | read:2, write:1, call:1 | 4 / 0 | `semantic` | Read, write, and call land in distinct groups; a write is never counted as a read. |
| `generated_test_external_labels` | read:1, import:1, test-only:1, generated:1 | 4 / 0 | `mixed` | Generated, test-only, and external occurrences keep their labels and counts. |
| `current_versus_captured` | read:1, call:1, import:1 | 1 / 2 | `mixed` | Current-scope occurrences are counted apart from runtime-trace and imported-snapshot ones. |
| `lexical_fallback_disclosed` | read:1, write:1 | 2 / 0 | `mixed` | Grep and syntax fallbacks carry a fallback note and a fallback evidence class. |
| `inherit_import_export_framework` | inherit:1, import:1, export:1 | 3 / 0 | `mixed` | Inherit, import, and export keep distinct groups; a framework export is named framework-derived. |

The `current_versus_captured` pane carries a `captured_scope_ref`
(`aureline://scope/captured-trace`), a `runtime_observed` and an
`imported_snapshot` label, and fallback notes for the runtime trace and imported
snapshot. The `lexical_fallback_disclosed` pane carries `lexical_fallback_only` and
`syntax_fallback_only` downgrade reasons. No pane flattens its occurrences into one
hit list.

## Access kind versus evidence class

| Dimension | Vocabulary |
| --- | --- |
| Access kind (grouping) | `read`, `write`, `call`, `inherit`, `import`, `export`, `test-only`, `generated` |
| Evidence class | `semantic`, `framework_derived`, `runtime_observed`, `imported_snapshot`, `lexical_fallback`, `syntax_fallback`, `mixed`, `unavailable` |

Access kind answers *what kind of usage* an occurrence is; evidence class answers
*how it was proven*. A runtime-observed or framework-derived reference is a normal
access-kind occurrence whose evidence is runtime or framework, not static semantic.

## Stable actions

| Action | Token | History effect | Routes |
| --- | --- | --- | --- |
| Open | `open` | `advances_history` | references pane · search panel · docs link · keyboard |
| Peek | `peek` | `preserves_current` | references pane · search panel · docs link · keyboard |
| Open to the Side | `split_open` | `advances_history` | references pane · search panel · docs link · keyboard |
| Export References | `export` | `no_editor_history` | references pane · search panel · docs link · keyboard |

Every action lists all four routes and preserves target identity, so an action
behaves identically no matter which surface invoked it.

## Consumer parity

Each pane projects to all seven consumer surfaces — `editor_ui`, `cli_headless`,
`ai_context`, `review_workspace`, `support_export`, `graph_overlay`,
`shell_continuity` — with access-kind grouping, scope counts, evidence class, and
generated/external/test labels preserved, `flattens_to_generic_hits: false`, and
`exports_code_bodies: false`.

## Frozen invariants (all `holds: true`)

- `reference_pane.access_kind_grouping_present`
- `reference_pane.scope_counts_reconcile`
- `reference_pane.evidence_class_disclosed_no_grep_as_semantic`
- `reference_pane.generated_external_test_labels_visible`
- `reference_pane.captured_scope_disclosed`
- `reference_pane.actions_stable_across_routes`
- `reference_pane.history_semantics_stable`
- `reference_pane.consumers_preserve_truth`
- `reference_pane.corpus_covers_vocabulary`
- `reference_pane.replayable_support_answer`

## Release-automation binding

The freeze gate
[`crates/aureline-navigation/tests/reference_panes.rs`](../../crates/aureline-navigation/tests/reference_panes.rs)
runs under `cargo test --workspace`. It rebuilds the corpus in code and asserts it
equals this fixture, re-proves that every stored pane equals the builder's own
output, that the corpus is support-export safe, that every pane groups by access
kind, reconciles current-versus-captured counts, discloses fallbacks, and exposes
the four stable actions, and that every invariant holds — so a claimed references
surface cannot promote while a pane could flatten a reference set into a generic
hit list or hide its scope and fallback truth.
