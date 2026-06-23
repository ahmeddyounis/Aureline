# Relation-aware continuity — evidence companion

Human-readable companion to
[`/fixtures/navigation/relation_continuity/canonical_continuity.json`](../../fixtures/navigation/relation_continuity/canonical_continuity.json)
and its boundary schema
[`/schemas/navigation/relation_continuity.schema.json`](../../schemas/navigation/relation_continuity.schema.json).
It gives reviewers the frozen scenario and invariant tables without reading the JSON.
The contract narrative lives in
[`/docs/navigation/relation_continuity.md`](../../docs/navigation/relation_continuity.md).

- Set id: `relation-continuity:set:0001`
- Record kind: `relation_continuity_set`
- Scenarios: 5 · Invariants: 9

## Continuity scenarios

| Scenario | Entries | Current / Captured | Disclose-before-jump | Drift states | Rename evidence |
| --- | --- | --- | --- | --- | --- |
| `bound_peek_reveal_split` | 3 | 3 / 0 | 0 | bound | 1 |
| `remapped_history` | 3 | 0 / 3 | 3 | bound, remapped | 0 |
| `drifted_missing_scope` | 3 | 0 / 3 | 3 | drifted, missing_target, scope_unavailable | 0 |
| `fallback_runtime_framework` | 3 | 0 / 3 | 3 | bound, remapped | 0 |
| `archived_and_ambiguous` | 2 | 0 / 2 | 2 | drifted, archived | 1 |

Each entry preserves its relation kind, origin surface, and a return anchor that restores
the origin selection and viewport (`restores_selection` / `restores_viewport` are always
true). `auto_open_allowed` is true only for the three bound, live, semantic entries in
`bound_peek_reveal_split`; every other entry — remapped, drifted, missing, scope-
unavailable, archived, or resting on a lexical/runtime/framework/imported evidence class —
discloses its state and offers recovery before any jump.

## What stays visible per drift state

- **Remapped** (`remapped_history`, `fallback_runtime_framework`): keeps the relation
  kind, cites stable `remap_evidence_refs` with `used_nearby_fallback: false`, retains a
  current target, and offers `open_remapped_target` — never a silent jump.
- **Drifted / missing / scope-unavailable** (`drifted_missing_scope`): carries no current
  target, keeps a visible `drift_reason` and recovery choices, and the drifted entry
  routes its ambiguity through a `disambiguation_set_ref`.
- **Archived** (`archived_and_ambiguous`): retained as metadata-only replay evidence with
  `keep_archived_reference` recovery.

## Evidence honesty

The `fallback_runtime_framework` scenario proves a `lexical_fallback` call, a
`runtime_observed` reference, and a `framework_derived` route binding each name their
evidence class, carry a fallback note and a downgrade reason, and stay captured-scope only
— so a grep or runtime match never auto-opens or reads as semantic. The
`remapped_history` scenario carries an `imported_snapshot` recent location that is
captured-scope only.

## Rename-preview evidence

`bound_peek_reveal_split` carries a `ready_for_apply_after_preview` rename preview
(4 changed / 0 held, bound); `archived_and_ambiguous` carries a
`blocked_pending_scope_review` rename preview (0 changed / 6 held, drifted) with a
disambiguation path. Both keep a `definition` root relation and a replay-safe id
(`aureline://replay/rename/...`).

## Frozen invariants (all hold)

| Invariant | Guarantees |
| --- | --- |
| `entry_preserves_relation_context` | Relation kind, origin surface, return anchor, and captured/current snapshots are preserved; a remap never relabels the relation kind. |
| `current_vs_captured_separated` | Counts reconcile by kind, by drift, and current + captured == total; current-scope only for bound live semantic entries. |
| `no_silent_jump` | Non-bound entries never auto-open, keep a drift reason and recovery; remaps cite stable evidence; drifted/missing/unavailable/archived carry no current target. |
| `drift_states_visible_with_disambiguation` | Drift state is labeled, and selection ambiguity carries a disambiguation set ref and a choose-from-disambiguation recovery. |
| `fallback_class_honest` | Fallback entries carry a note and downgrade reason and never auto-open; runtime/framework/imported entries stay captured-scope. |
| `replay_ids_stable` | Every entry and rename-evidence row carries a replay-safe id derived from its stable id, and the packet is replay-safe. |
| `rename_evidence_preserved` | Rename evidence keeps a definition root, posture, ambiguity, drift, return anchor, counts, and replay id, and discloses drift/ambiguity. |
| `consumers_preserve_truth` | Every required consumer surface preserves relation kind, origin, return anchor, current/captured, drift, fallback, ambiguity, and replay ids without retargeting or exporting code bodies. |
| `corpus_covers_vocabulary` | The corpus exercises every entry kind, every drift state, the semantic/framework/runtime/imported/lexical evidence answers, and rename evidence. |

## Release-automation binding

The freeze gate
[`/crates/aureline-navigation/tests/relation_continuity.rs`](../../crates/aureline-navigation/tests/relation_continuity.rs)
rebuilds the corpus in code and asserts it equals this fixture after a serialize
round-trip, re-proves every stored packet equals the builder's output, and re-checks the
guarantees above. Regenerate the fixture after any builder change with:

```sh
cargo run -p aureline-navigation --example dump_relation_continuity \
  > fixtures/navigation/relation_continuity/canonical_continuity.json
```
