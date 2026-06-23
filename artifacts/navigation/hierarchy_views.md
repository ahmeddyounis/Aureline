# Hierarchy views — evidence companion

Human-readable companion to
[`/fixtures/navigation/hierarchy_views/canonical_views.json`](../../fixtures/navigation/hierarchy_views/canonical_views.json)
and its boundary schema
[`/schemas/navigation/hierarchy_views.schema.json`](../../schemas/navigation/hierarchy_views.schema.json).
It gives reviewers the frozen scenario and invariant tables without reading the JSON.
The contract narrative lives in
[`/docs/navigation/hierarchy_views.md`](../../docs/navigation/hierarchy_views.md).

- Set id: `hierarchy-views:set:0001`
- Record kind: `hierarchy_views_set`
- Scenarios: 5 · Invariants: 12

## View scenarios

| Scenario | View kind | Direction | Tiers (legend : count) | Current / Captured | View scope | Ambiguity | Proves |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `call_direct_and_transitive` | call | incoming | direct:2, transitive:1 | 3 / 0 | complete | unambiguous | Direct callers land in the direct tier; the multi-hop caller lands in the transitive tier. |
| `call_runtime_and_inferred` | call | outgoing | direct:1, inferred:1, runtime-observed:1 | 2 / 1 | partial | unambiguous | Runtime-observed and framework-inferred callees stay in their own tiers; the untraced remainder is a named gap. |
| `type_incomplete_scope` | type | incoming | direct:1, transitive:1 | 2 / 0 | unavailable | unambiguous | The unindexed external crate is named as a missing scope, so the partial hierarchy never reads as complete. |
| `override_ambiguous_roots` | override | bidirectional | direct:2 | 2 / 0 | complete | ambiguous-needs-selection | Two competing base roots and a disambiguation set are exposed and the navigating actions are gated before a jump. |
| `ownership_inferred_imported` | ownership | incoming | inferred:2 | 1 / 1 | stale | unambiguous | Framework-inferred and imported ownership edges stay disclosed with a captured scope ref, labels, notes, and a stale gap. |

The `call_runtime_and_inferred` view carries a `captured_scope_ref`
(`aureline://scope/captured-trace`), a `runtime_observed` and a `framework_derived`
label, evidence refs on the runtime and framework edges, and a named
`aureline://scope/untraced-paths` gap. The `override_ambiguous_roots` view carries an
`ambiguous_candidates` downgrade reason, a `competing_roots` label, two competing root
refs, and a disambiguation set ref. The `ownership_inferred_imported` view carries an
`imported_snapshot`, `framework_derived`, `generated`, and `stale_evidence` label and a
named `aureline://scope/archived-branch` gap. No view flattens its edges into one
opaque tree.

## Legend versus edge kind

| Dimension | Vocabulary |
| --- | --- |
| View kind | `call`, `type`, `override`, `ownership` |
| Edge legend (grouping) | `direct`, `transitive`, `inferred`, `runtime_observed` (headline may also be `mixed` or `empty`) |

View kind answers *which hierarchy* this is; legend answers *how each edge was
reached*. Proof class wins over depth, so a deep runtime edge is never relabeled as
transitive structure, and an inferred or runtime edge never enters the direct tier.

## Stable actions

| Action | Token | History effect | Gated by ambiguity | Routes |
| --- | --- | --- | --- | --- |
| Go to Node | `open` | `advances_history` | yes | hierarchy view · graph overlay · search panel · docs link · keyboard |
| Peek | `peek` | `preserves_current` | no | hierarchy view · graph overlay · search panel · docs link · keyboard |
| Open to the Side | `split_open` | `advances_history` | yes | hierarchy view · graph overlay · search panel · docs link · keyboard |
| Expand Subtree | `expand` | `preserves_current` | no | hierarchy view · graph overlay · search panel · docs link · keyboard |
| Export Hierarchy | `export` | `no_editor_history` | no | hierarchy view · graph overlay · search panel · docs link · keyboard |

Every action lists all five routes and preserves target identity, so an action
behaves identically no matter which surface invoked it. The two navigating actions are
gated only while the root is ambiguous.

## Consumer parity

Each view projects to all seven consumer surfaces — `editor_ui`, `cli_headless`,
`ai_context`, `review_workspace`, `support_export`, `graph_overlay`,
`shell_continuity` — with legend grouping, edge counts, scope completeness,
freshness/confidence, and ambiguity state preserved, `flattens_to_single_tree: false`,
and `exports_code_bodies: false`.

## Frozen invariants (all `holds: true`)

- `hierarchy_view.legend_grouping_present`
- `hierarchy_view.legend_counts_reconcile`
- `hierarchy_view.direct_distinguished_from_derived`
- `hierarchy_view.inferred_and_runtime_disclosed`
- `hierarchy_view.missing_scope_named`
- `hierarchy_view.captured_scope_disclosed`
- `hierarchy_view.ambiguity_inspectable_before_jump`
- `hierarchy_view.actions_stable_across_routes`
- `hierarchy_view.consumers_preserve_truth`
- `hierarchy_view.edges_match_view_kind`
- `hierarchy_view.corpus_covers_vocabulary`
- `hierarchy_view.replayable_support_answer`

## Release-automation binding

The freeze gate
[`crates/aureline-navigation/tests/hierarchy_views.rs`](../../crates/aureline-navigation/tests/hierarchy_views.rs)
runs under `cargo test --workspace`. It rebuilds the corpus in code and asserts it
equals this fixture, re-proves that every stored view equals the builder's own output,
that the corpus is support-export safe, that every view groups by legend, partitions
its counts, names missing scope, discloses inferred/runtime edges, exposes competing
roots before a jump, and exposes the five stable actions, and that every invariant
holds — so a claimed hierarchy surface cannot promote while a view could flatten a
hierarchy into one opaque tree or hide its legend, scope, and ambiguity truth.
