# Scope-truth chip example artifacts

These artifacts are short, reviewable examples of the chip
contract frozen in
[`/docs/workspace/scope_truth_packet.md`](../../../docs/workspace/scope_truth_packet.md)
and validated by the schema at
[`/schemas/workspace/workset_artifact.schema.json`](../../../schemas/workspace/workset_artifact.schema.json).

Every example is one `scope_truth_chip_record` that references
a workset fixture in
[`/fixtures/workspace/workset_examples/`](../../../fixtures/workspace/workset_examples/)
by `workset_id`. Surfaces render the chip from the record; they
never derive a parallel scope label.

## What each example shows

| Example | Surface class | Underlying workset fixture | Key chip state |
|---|---|---|---|
| [`open_flow_trust_card_current_repo.json`](./open_flow_trust_card_current_repo.json) | `open_flow_trust_card` | `current_repo_fallback.json` | `active_narrow_safe`, single root, no hidden content |
| [`search_scope_banner_selected_workset.json`](./search_scope_banner_selected_workset.json) | `scope_banner` | `selected_workset_multi_root.json` | `active_partial`, `outside_scope_roots` count = 4, widen actions offered |
| [`cross_repo_result_group_outside_current_scope.json`](./cross_repo_result_group_outside_current_scope.json) | `cross_repo_result_group` | `selected_workset_multi_root.json` | `outside_current_scope`, marker visible, 12 results from payments-infra |
| [`support_packet_header_policy_limited.json`](./support_packet_header_policy_limited.json) | `support_packet_header` | `policy_limited_admin_hidden.json` | `active_policy_limited`, `policy_hidden` count = 5, list not visible |
| [`ai_context_inspector_sparse_slice.json`](./ai_context_inspector_sparse_slice.json) | `ai_context_inspector` | `sparse_slice_pattern_narrowed.json` | `active_partial`, `partial_index` count = 17412 |
| [`refactor_scope_footer_widened.json`](./refactor_scope_footer_widened.json) | `refactor_scope_footer` | `selected_workset_multi_root.json` (+ `scope_widen_diff_selected_to_full.json`) | `active_widened`, `open_scope_diff` offered |
| [`export_scope_footer_full_workspace.json`](./export_scope_footer_full_workspace.json) | `export_scope_footer` | `full_workspace_multi_root.json` | `active_narrow_safe`, `export_workset_artifact` offered |

## Scope rules

- Every example carries `workset_artifact_schema_version: 1` and
  validates against the shared workset-artifact schema.
- Chips reference a `workset_id` from the fixture directory; they
  do not inline the artifact. Consumers resolve the artifact to
  read patterns, member refs, or portability detail.
- Chip labels resolve from the closed label family
  (`Current repo`, `Selected workset`, `Sparse slice`,
  `Full workspace`, `Policy-limited view`, `Outside current
  scope`) and embed the workset name where relevant.
- Raw policy bodies, credential material, and the exact
  hidden-member list of an admin-policy narrowing never appear.
  Counts and class labels cross the boundary; raw lists do not.
- Opaque ids and timestamps are chosen for review clarity rather
  than to mirror a real machine.

## Coverage contract

This example set MUST keep at least one chip for each of the
seven surface classes this packet covers at seed time
(`open_flow_trust_card`, `scope_banner`, `cross_repo_result_group`,
`support_packet_header`, `ai_context_inspector`,
`refactor_scope_footer`, `export_scope_footer`) and at least one
chip exercising each of the five chip presentation states
(`active_narrow_safe`, `active_partial`, `active_policy_limited`,
`active_widened`, `outside_current_scope`). Adding a chip for a
new surface class or a new `count_class` is welcome; removing a
surface class this directory already covers is a breaking change.
