# Stabilized Repository-Topology Truth

Stable Git, search, review, blame, AI, and run/debug rows now share one
typed topology packet before they claim complete coverage or target a
root for mutation.

## Contract

The canonical packet is `repository_topology_truth_packet`, implemented
in `crates/aureline-git` and described by
`schemas/review/repository-topology.schema.json`.

It carries:

- `repository_topology_descriptor` rows for sparse/workset roots,
  partial-clone promisor roots, shallow history, submodule gitlinks,
  nested independent roots, Git LFS hydration boundaries, and
  generated/vendor exclusions.
- `surface_topology_truth_row` rows for search, Git graph, review,
  blame, code actions, AI context, run/debug, and support export.
- a redaction-safe support export that preserves topology class,
  omitted or unfetched scope, chosen action, active root, authoritative
  root, parent-child linkage, shallow boundary, and LFS pointer scope.

## Required Behavior

Rows with topology labels must use `narrowed_by_topology` or
`denied_wrong_root`; they cannot report full coverage. The labels are
closed and intentionally precise: `outside_current_slice`, `not_fetched`,
`shallow_boundary`, `submodule_not_initialized`,
`nested_repo_boundary`, `pointer_only`, `wrong_target_root`,
`policy_excluded`, `unavailable`, and `generated_or_excluded`.

Network-bearing actions are distinct and approval-aware:
`fetch_missing_objects`, `deepen_history`, `initialize_submodule`, and
`hydrate_lfs_objects` must carry `approval_required`, `approved`, or
`policy_blocked`. The packet never performs those actions implicitly.

Root identity is explicit on every row. `active_root_ref` records what
the caller selected, while `authoritative_root_ref` records the root that
owns the object or operation. Nested and submodule rows keep parent and
child identity separate.

## Evidence

The review fixture
`fixtures/review/m4/stabilize-repository-topology-truth/stable_cross_surface_topology_packet.json`
covers:

- sparse/workset search omission with a widen affordance;
- partial-clone promisor objects with fetch approval posture;
- shallow blame/history with deepen approval posture;
- parent submodule review where child initialization is required;
- nested independent repo wrong-root denial;
- Git LFS pointer-only AI context and hydrated LFS support export;
- generated/vendor run/debug exclusion; and
- support-export reconstruction without raw paths or object bytes.

## Verification

```bash
cargo test -p aureline-git --test stabilize_repository_topology_truth
```
