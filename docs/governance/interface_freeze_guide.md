# Interface-freeze guide

Use the interface-freeze matrix when a task, ADR, packet, or planning
note would otherwise restate a contract that is already frozen.

Canonical source:

- [`/artifacts/governance/interface_freeze_matrix.yaml`](../../artifacts/governance/interface_freeze_matrix.yaml)

Overview:

- [`./interface_freeze_matrix.md`](./interface_freeze_matrix.md)

## How to cite a row

1. Find the row id in `interface_freeze_matrix.yaml`.
2. Check its `freeze_status`.
3. Cite the row id and the canonical refs instead of copying the
   contract prose into the new document.
4. Describe only the delta your change needs.

Preferred citation format in prose:

`Cites interface-freeze row <row_id>; implementation reuses the frozen contract and changes only <delta>.`

Preferred citation format in YAML or packet notes:

`interface_freeze_rows: [<row_id>]`

## What each status means for downstream work

| Status | Downstream rule |
|---|---|
| `frozen` | Reuse the row. Do not rewrite the contract in task prose or code comments. |
| `provisional` | Reuse only the frozen subset named in `current_state`; keep the missing scope visible in the task. |
| `blocked` | Stop broadening implementation on that surface. Open or land the missing contract first. |

## Examples

Use `buffer_strategy_and_source_fidelity` when an editor task depends on
piece-tree semantics, undo grouping, decode recovery, or large-file
fallback. The task should not restate those rules.

Use `design_token_and_component_state_vocabulary` when a shell surface
needs state names such as `pending`, `policy_blocked`, or
`reconnecting`. The task should cite the row and name only the new
surface using the existing vocabulary.

Use `review_change_stack_and_hosted_merge_policy` as a stop sign. If a
task wants hosted review, change-stack, or merge-policy behavior, the
task should say the row is `blocked` rather than infer policy from Git
or mutation-lineage seeds.

## Required follow-through

- If you change a frozen contract, update the owning contract doc first,
  then update the matrix row in the same change.
- If a provisional row becomes frozen or blocked, update the row's
  `current_state`, `blocking_downstream_tasks`, and `next_review_date`
  together.
- If a new surface becomes a real dependency for implementation
  broadening, add a row instead of hiding the dependency in review
  notes or a handoff file.
