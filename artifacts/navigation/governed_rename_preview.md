# Governed rename preview — evidence companion

Human-readable companion to
[`/fixtures/navigation/governed_rename_preview/canonical_previews.json`](../../fixtures/navigation/governed_rename_preview/canonical_previews.json)
and its boundary schema
[`/schemas/navigation/governed_rename_preview.schema.json`](../../schemas/navigation/governed_rename_preview.schema.json).
It gives reviewers the frozen scenario and invariant tables without reading the JSON.
The contract narrative lives in
[`/docs/navigation/governed_rename_preview.md`](../../docs/navigation/governed_rename_preview.md).

- Set id: `rename-preview:set:0001`
- Record kind: `rename_preview_governance_set`
- Scenarios: 6 · Invariants: 12

## Preview scenarios

| Scenario | Groups (kind : count) | Will change / Held | Current / Captured | Apply posture | Proves |
| --- | --- | --- | --- | --- | --- |
| `clean_editable` | editable:3 | 3 / 0 | 3 / 0 | `ready_for_apply_after_preview` | Every candidate is editable, yet a blind apply is still blocked. |
| `blocked_generated_readonly` | editable:1, blocked:2, generated:1, read-only:1 | 1 / 4 | 5 / 0 | `blocked_pending_policy_or_protected_review` | Blocked, generated, and read-only candidates are held with visible reasons. |
| `conflict_shadowing` | editable:1, conflict:2 | 1 / 2 | 3 / 0 | `blocked_pending_scope_review` | Shadowing and alias conflicts are held with their conflict notes. |
| `stale_unresolved_refresh` | editable:1, partial-scope:2 | 1 / 2 | 1 / 2 | `blocked_pending_refresh` | Stale scope and an unresolved anchor demand a refresh; counts split current vs captured. |
| `fallback_sparse_visible` | editable:1, partial-scope:1 | 1 / 1 | 2 / 0 | `ready_for_apply_after_preview` | A lexical/grep + sparse candidate is held visibly while the editable set still applies. |
| `inspect_only_nothing_editable` | generated:1, read-only:2 | 0 / 3 | 1 / 2 | `inspect_only_unavailable` | When nothing is editable the rename is inspect-only, not silently empty. |

Every preview's apply gate has `inspect_before_mutate_required: true`,
`blind_apply_blocked: true`, `omitted_candidates_remain_visible: true`,
`redacted_candidates_remain_visible: true`, and a bound
`undo_checkpoint_ref` (`aureline://undo/rename/...`). `apply_allowed_after_preview` is
true only for the two `ready_for_apply_after_preview` scenarios.

## Group kind versus evidence class

| Dimension | Vocabulary |
| --- | --- |
| Group kind (precedence) | `editable`, `blocked_for_review`, `conflict`, `generated_boundary`, `read_only_or_external`, `partial_scope_omitted` |
| Omission reason | `blocked_pending_review`, `policy_limited`, `conflict_pending_resolution`, `generated_boundary`, `read_only_or_protected`, `external_dependency`, `out_of_scope_sparse`, `partially_loaded`, `stale_scope`, `unresolved_anchor` |
| Evidence class | `semantic`, `framework_derived`, `runtime_observed`, `imported_snapshot`, `lexical_fallback`, `syntax_fallback`, `mixed`, `unavailable` |

Group kind answers *whether and why* a candidate will be changed; evidence class
answers *how it was proven*. A candidate matched only by a grep fallback is held in the
partial-scope group with `lexical_fallback` evidence, a fallback note, and a downgrade
reason — never renamed as if it were semantic.

## Apply postures and preconditions

| Posture | Token | Apply allowed after preview |
| --- | --- | --- |
| Ready after preview | `ready_for_apply_after_preview` | yes |
| Policy / protected review | `blocked_pending_policy_or_protected_review` | no |
| Scope review | `blocked_pending_scope_review` | no |
| Refresh | `blocked_pending_refresh` | no |
| Inspect-only | `inspect_only_unavailable` | no |

Preconditions exercised across the corpus: `review_blocked_candidates`,
`resolve_conflicts`, `acknowledge_generated_boundary`, `acknowledge_read_only_omission`,
`acknowledge_partial_scope`, `refresh_stale_scope`, `widen_sparse_scope`.

## Consumer parity

Each preview projects to all seven consumer surfaces — `editor_ui`, `cli_headless`,
`ai_context`, `review_workspace`, `support_export`, `graph_overlay`,
`shell_continuity` — with candidate grouping, change-versus-held counts, omission
reasons, conflict notes, the apply gate, and the undo checkpoint preserved,
`omitted_candidates_remain_visible: true`, `flattens_to_single_apply_action: false`,
and `exports_code_bodies: false`.

## Frozen invariants (all `holds: true`)

- `rename_preview.candidate_grouping_disjoint`
- `rename_preview.counts_reconcile`
- `rename_preview.omissions_visible`
- `rename_preview.evidence_class_disclosed_no_grep_as_semantic`
- `rename_preview.conflict_notes_preserved`
- `rename_preview.inspect_before_mutate_enforced`
- `rename_preview.apply_posture_matches_groups`
- `rename_preview.partial_scope_truth`
- `rename_preview.preview_set_consistent`
- `rename_preview.consumers_preserve_truth`
- `rename_preview.corpus_covers_vocabulary`
- `rename_preview.replayable_support_answer`

## Release-automation binding

The freeze gate
[`crates/aureline-navigation/tests/rename_preview.rs`](../../crates/aureline-navigation/tests/rename_preview.rs)
runs under `cargo test --workspace`. It rebuilds the corpus in code and asserts it
equals this fixture, re-proves that every stored preview equals the builder's own
output, that the corpus is support-export safe, that every preview groups candidates
disjointly, reconciles change-versus-held counts, keeps omitted candidates visible,
enforces the inspect-before-mutate apply gate, and that every invariant holds — so a
claimed rename surface cannot promote while a broad rename could flatten into one apply
action or drop its blocked, conflicting, generated, read-only, or partial-scope
candidates.
