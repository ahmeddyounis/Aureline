# Related-object navigation — evidence companion

Human-readable companion to
[`/fixtures/navigation/related_object_navigation/canonical_links.json`](../../fixtures/navigation/related_object_navigation/canonical_links.json)
and its boundary schema
[`/schemas/navigation/related_object_navigation.schema.json`](../../schemas/navigation/related_object_navigation.schema.json).
It gives reviewers the frozen scenario and invariant tables without reading the JSON.
The contract narrative lives in
[`/docs/navigation/related_object_navigation.md`](../../docs/navigation/related_object_navigation.md).

- Set id: `related-object-navigation:set:0001`
- Record kind: `related_object_navigation_set`
- Scenarios: 6 · Invariants: 13

## Panel scenarios

| Scenario | Anchor context | Parity | Groups (source : count) | Current / Captured | Headline | Scope | Selection | Proves |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `editor_route_and_component` | editor_symbol | stable | graph:1, framework:1 | 2 / 0 | mixed | complete | no | A graph-proven component and a framework-derived route stay in separate source groups. |
| `editor_owner_doc_curated` | editor_symbol | stable | curated:2 | 0 / 2 | curated | complete | yes | A curated owner with competing candidates gates the navigating actions before a jump. |
| `generated_artifact_runtime` | generated_artifact | stable | framework:1, runtime:1 | 0 / 2 | mixed | partial | no | A generated pair and a runtime-observed test stay disclosed and captured-scope only. |
| `notebook_test_doc` | notebook_cell | partial | graph:1, curated:1 | 1 / 1 | mixed | unavailable | no | A notebook cell reuses the relation semantics and names an unavailable doc honestly. |
| `diff_hunk_unsupported` | diff_hunk | unsupported | — | 0 / 0 | empty | complete | no | A diff hunk that cannot anchor links labels unsupported parity with no fabricated links. |
| `docs_linked_component` | docs_linked_symbol | partial | framework:1, curated:1 | 1 / 1 | mixed | partial | no | A docs-linked symbol discloses a lexically matched component apart from a curated doc. |

The `editor_owner_doc_curated` panel carries an `ambiguous_candidates` downgrade reason,
a `disambiguation_required` label, one competing link ref, and a disambiguation set ref.
The `generated_artifact_runtime` panel carries a `captured_scope_ref`
(`aureline://scope/captured-trace`), `imported_snapshot`, `runtime_observed_only`,
`generated`, and `captured_scope_only` labels, and evidence refs on both links. The
`notebook_test_doc` panel names an `unavailable` doc and reports an incomplete scope. The
`diff_hunk_unsupported` panel carries an `unsupported_parity` label, a `missing_provider`
downgrade reason, a parity note, and no links. The `docs_linked_component` panel carries a
`lexical_fallback` label and a `lexical_fallback_only` downgrade reason. No panel flattens
its links into one bucket of generic buttons.

## Object kind, source class, and fallback mode

| Dimension | Vocabulary |
| --- | --- |
| Object kind | `route`, `component`, `test`, `doc`, `owner`, `generated_artifact` |
| Source class (grouping) | `graph_derived`, `framework_derived`, `curated`, `runtime_derived` (headline may also be `mixed` or `empty`) |
| Fallback mode | `primary`, `disambiguation_required`, `lexical_fallback`, `imported_snapshot`, `runtime_observed_only`, `unavailable` |

Object kind answers *what is on the other end*; source class answers *what evidence class
backed it*; fallback mode answers *how it resolves*. Only a graph-derived link reads as
proven, and every non-graph group carries attribution notes, so the four evidence classes
never collapse into one homogeneous certainty class.

## Stable actions

| Action | Token | History effect | Gated by disambiguation | Routes |
| --- | --- | --- | --- | --- |
| Go to Related | `open` | `advances_history` | yes | related panel · editor gutter · graph overlay · search panel · docs link · keyboard |
| Peek | `peek` | `preserves_current` | no | related panel · editor gutter · graph overlay · search panel · docs link · keyboard |
| Open to the Side | `split_open` | `advances_history` | yes | related panel · editor gutter · graph overlay · search panel · docs link · keyboard |
| Reveal Source | `reveal_attribution` | `preserves_current` | no | related panel · editor gutter · graph overlay · search panel · docs link · keyboard |
| Export Related Objects | `export` | `no_editor_history` | no | related panel · editor gutter · graph overlay · search panel · docs link · keyboard |

Every action lists all six routes and preserves the anchor identity, so an action behaves
identically no matter which surface invoked it. The two navigating actions are gated only
while disambiguation is pending.

## Consumer parity

Each panel projects to all seven consumer surfaces — `editor_ui`, `cli_headless`,
`ai_context`, `review_workspace`, `support_export`, `graph_overlay`, `shell_continuity` —
with source attribution, counts, fallback truth, anchor parity, and freshness/confidence
preserved, `flattens_to_generic_links: false`, and `exports_code_bodies: false`.

## Frozen invariants (all `holds: true`)

- `related_object.source_attribution_present`
- `related_object.counts_reconcile_and_partition`
- `related_object.source_classes_never_homogeneous`
- `related_object.named_source_never_generic`
- `related_object.fallback_truth_disclosed`
- `related_object.captured_scope_disclosed`
- `related_object.incomplete_scope_named`
- `related_object.disambiguation_inspectable_before_jump`
- `related_object.anchor_parity_honest`
- `related_object.actions_stable_across_routes`
- `related_object.consumers_preserve_truth`
- `related_object.corpus_covers_vocabulary`
- `related_object.replayable_support_answer`

## Release-automation binding

The freeze gate
[`crates/aureline-navigation/tests/related_object_navigation.rs`](../../crates/aureline-navigation/tests/related_object_navigation.rs)
runs under `cargo test --workspace`. It rebuilds the corpus in code and asserts it equals
this fixture, re-proves that every stored panel equals the builder's own output, that the
corpus is support-export safe, that every panel groups by source class, partitions its
counts, discloses fallback truth, names an incomplete scope, exposes competing links
before a jump, labels unsupported anchor parity honestly, and exposes the five stable
actions, and that every invariant holds — so a claimed related-object surface cannot
promote while a panel could flatten its links into generic smart links or hide its source,
fallback, scope, and parity truth.
