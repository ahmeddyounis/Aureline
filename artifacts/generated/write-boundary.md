# Generated-artifact write-boundary proof packet

The canonical write-boundary packet is implemented in
[`crates/aureline-generated/src/write_boundary/mod.rs`](../../crates/aureline-generated/src/write_boundary/mod.rs)
and serialized to
[`artifacts/generated/write-boundary-packet.json`](./write-boundary-packet.json).

It is the checked-in truth source for:

- the reviewer contract in
  [`docs/generated/write-boundary-review.md`](../../docs/generated/write-boundary-review.md)
- the boundary schema at
  [`schemas/generated/write-boundary.schema.json`](../../schemas/generated/write-boundary.schema.json)
- fixture replay in
  [`crates/aureline-generated/tests/write_boundary.rs`](../../crates/aureline-generated/tests/write_boundary.rs)
- the fixture corpus under
  [`fixtures/generated/write-boundary/`](../../fixtures/generated/write-boundary/)

## What the packet models

For each generated-artifact class and boundary state, the packet carries one
case — the artifact subject and the decision the engine reaches when a direct
edit is attempted. A direct edit is **admitted** only when the artifact is its
own canonical source and in sync; otherwise it is **held** for a reviewed
override or **blocked** in favor of regeneration.

## Cases

| Case | Class | Boundary state | Effective gate | Outcome |
| --- | --- | --- | --- | --- |
| `write-boundary.scaffolded_project.in_sync` | scaffolded_project | `in_sync` | `direct_edit_allowed` | `direct_edit_admitted` |
| `write-boundary.ai_assisted_edit.in_sync` | ai_assisted_edit | `in_sync` | `direct_edit_allowed` | `direct_edit_admitted` |
| `write-boundary.notebook_output.in_sync` | notebook_output | `in_sync` | `regenerate_only` | `blocked_regenerate_first` |
| `write-boundary.preview_derivative.in_sync` | preview_derivative | `in_sync` | `regenerate_only` | `blocked_regenerate_first` |
| `write-boundary.support_packet.in_sync` | support_packet | `in_sync` | `regenerate_only` | `blocked_regenerate_first` |
| `write-boundary.request_artifact.in_sync` | request_artifact | `in_sync` | `reviewed_override_required` | `blocked_pending_review` |
| `write-boundary.framework_codegen.in_sync` | framework_codegen | `in_sync` | `reviewed_override_required` | `blocked_pending_review` |
| `write-boundary.framework_codegen.override_admitted` | framework_codegen | `in_sync` | `reviewed_override_required` | `override_admitted_with_divergence` |
| `write-boundary.request_artifact.drift_detected` | request_artifact | `drift_detected` | `reviewed_override_required` | `blocked_pending_review` |
| `write-boundary.notebook_output.source_missing` | notebook_output | `source_missing` | `regenerate_only` | `blocked_regenerate_first` |
| `write-boundary.preview_derivative.generator_unavailable` | preview_derivative | `generator_unavailable` | `regenerate_only` | `blocked_regenerate_first` |
| `write-boundary.framework_codegen.regeneration_blocked_by_policy` | framework_codegen | `regeneration_blocked_by_policy` | `regenerate_only` | `blocked_regenerate_first` |

## The frozen guardrails

- A non-authoritative generated artifact is never mutated silently: it is
  admitted only when it is its own canonical source and in sync, otherwise held
  for a reviewed override or blocked in favor of regeneration.
- Every block carries its reason as `why_blocked_tokens` and a `guidance_line`;
  a block is never a generic save failure.
- The single override case leaves a durable diverged-from-generator state with
  a recovery path — regenerate to discard, or reconcile into the canonical
  source — and an override never forces a write past a `regenerate_only` block.
- Every decision carries a three-way compare over canonical source, current
  artifact, and regenerated candidate, with provenance preserved on every leg
  even when a leg cannot be produced.
- The source-missing, generator-unavailable, and policy-blocked cases surface
  the precondition to restore before regeneration rather than a dead-end block.

## Surface bindings

Every binding ingests the same packet id (`generated.write_boundary.v1`):

- `file_tree_save_gate` — `crates/aureline-vfs/src/save_conflict_suite/mod.rs`
- `review_override_sheet` — `crates/aureline-review/src/change_inspector/mod.rs`
- `diverged_state_lineage` — `crates/aureline-workspace/src/mutation_and_generated_artifact_lineage/mod.rs`
- `ai_context` — `crates/aureline-ai/src/context_inspector/mod.rs`
- `support_export` — `crates/aureline-support/src/generated_lineage/mod.rs`
