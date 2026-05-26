# Semantic-recall boundary truth packet — fixture corpus

This directory holds the fixture corpus that the `aureline-docs`
integration test consumes to certify the stable docs/code
semantic-recall boundary truth packet.

Each fixture is a JSON file with the following top-level shape:

```jsonc
{
  "record_kind": "semantic_recall_boundary_truth_stable_case",
  "schema_version": 1,
  "case_name": "<stable_token>",
  "scenario": "<one paragraph describing what this case proves>",
  "input": { /* SemanticRecallBoundaryTruthPacketInput */ },
  "expect": { /* assertions */ }
}
```

The covered cases are:

- `baseline_stable.json` — every required recall-lane class is
  certified on the `m4_stable` track, an explicit `v1x_preview` row
  carries a recorded downgrade, mirrored/managed locality rows
  carry a verified pack signature, hybrid recall carries a
  ranking-reason ref, policy-hidden omissions are disclosed, and
  every required consumer surface preserves the packet verbatim.

- `raw_query_text_leak_blocks_stable.json` — a baseline row drops
  the `raw_query_text_excluded` flag. The validator MUST raise
  `raw_query_text_present` and the packet MUST narrow to
  `blocks_stable`.

- `unsigned_mirrored_pack_blocks_stable.json` — a mirrored-pack
  row drops its pack signature. The validator MUST raise
  `missing_pack_signature`.

- `mixed_generation_recall_blocks_stable.json` — a stable row
  declares `retrieval_epoch_state = mixed_generation_blocked`
  while keeping `downgrade_state = none`. The validator MUST
  raise `epoch_mismatch_presented_as_current` so mixed-generation
  recall cannot masquerade as current truth.

- `policy_omissions_undisclosed_blocks_stable.json` — a hybrid
  row records omitted lanes but does not disclose the
  policy-hidden omissions. The validator MUST raise
  `policy_omissions_undisclosed`.
