# Generated-artifact mutation-guardrails proof packet

The canonical mutation-guardrails packet is implemented in
[`crates/aureline-generated/src/mutation_guardrails/mod.rs`](../../crates/aureline-generated/src/mutation_guardrails/mod.rs)
and serialized to
[`artifacts/generated/mutation-guardrails-packet.json`](./mutation-guardrails-packet.json).

It is the checked-in truth source for:

- the reviewer contract in
  [`docs/generated/mutation-guardrails.md`](../../docs/generated/mutation-guardrails.md)
- the guardrail schema at
  [`schemas/generated/mutation-guardrails.schema.json`](../../schemas/generated/mutation-guardrails.schema.json)
- fixture replay in
  [`crates/aureline-generated/tests/mutation_guardrails.rs`](../../crates/aureline-generated/tests/mutation_guardrails.rs)
- the fixture corpus under
  [`fixtures/generated/mutation-guardrails/`](../../fixtures/generated/mutation-guardrails/)

## What the packet models

For each automated mutation route — AI apply, refactor, quick fix, automation —
and each guardrail outcome, the packet carries one case: the mutation attempt
and the decision the single engine reaches. The engine reuses the
[write-boundary](./write-boundary.md) decision for the underlying boundary
classification and layers the automated-route requirements on top: a
cross-boundary mutation is admitted only with a complete safety envelope
(preview, reviewed side-effect summary, regeneration awareness, and a rollback
class) **and** a recorded reviewed override.

## Cases

| Case | Route | Mutation class | Outcome | Unmet |
| --- | --- | --- | --- | --- |
| `mutation-guardrails.ai_apply.ai_assisted_edit.admitted_direct` | ai_apply | semantic_tooling | `admitted_direct` | — |
| `mutation-guardrails.refactor.scaffolded_project.admitted_direct` | refactor | semantic_tooling | `admitted_direct` | — |
| `mutation-guardrails.refactor.framework_codegen.admitted_with_preview_and_override` | refactor | semantic_tooling | `admitted_with_preview_and_override` | — |
| `mutation-guardrails.ai_apply.framework_codegen.blocked_missing_preview` | ai_apply | semantic_tooling | `blocked_pending_review` | `preview` |
| `mutation-guardrails.automation.framework_codegen.blocked_undeclared_side_effects` | automation | generated_state | `blocked_pending_review` | `side_effect_summary` |
| `mutation-guardrails.ai_apply.framework_codegen.blocked_regeneration_not_acknowledged` | ai_apply | semantic_tooling | `blocked_pending_review` | `regeneration_awareness` |
| `mutation-guardrails.automation.framework_codegen.blocked_no_rollback_class` | automation | generated_state | `blocked_pending_review` | `rollback_class` |
| `mutation-guardrails.quick_fix.request_artifact.blocked_pending_review` | quick_fix | semantic_tooling | `blocked_pending_review` | — |
| `mutation-guardrails.automation.notebook_output.blocked_regenerate_first` | automation | generated_state | `blocked_regenerate_first` | — |
| `mutation-guardrails.quick_fix.preview_derivative.blocked_regenerate_first` | quick_fix | generated_state | `blocked_regenerate_first` | — |
| `mutation-guardrails.refactor.framework_codegen.blocked_regeneration_policy` | refactor | semantic_tooling | `blocked_regenerate_first` | — |
| `mutation-guardrails.ai_apply.missing_boundary_data.blocked` | ai_apply | semantic_tooling | `blocked_missing_boundary_data` | — |

## The frozen guardrails

- No AI apply, refactor, quick fix, or automation path can silently mutate a
  non-authoritative generated artifact as if it were ordinary user-authored
  source: it is admitted directly only when the artifact is its own canonical
  source and in sync.
- An automated route that targets an artifact with no canonical-source boundary
  data is blocked outright — it cannot be classified, treated as ordinary
  source, or admitted through an override.
- A cross-boundary mutation is admitted only with all four safety requirements —
  preview, reviewed side-effect summary, regeneration awareness, and a rollback
  class — and a recorded reviewed override; any unmet requirement holds the
  mutation and names what is missing.
- An undeclared networked install, tool download, secret use, or broad write is
  never run silently; an audit-only reversal class is never accepted as
  rollback-safe.
- Every decision records actor lineage and mutation class against the shared
  mutation-journal contract and reuses the write-boundary decision and the
  regeneration side-effect / rollback vocabulary, so no route gets a hidden
  mutation path and support / export can explain which route crossed the
  boundary and under what posture.

## Surface bindings

Every binding ingests the same packet id (`generated.mutation_guardrails.v1`):

- `ai_apply_gate` — `crates/aureline-ai/src/harden_ai_scoped_apply/mod.rs`
- `refactor_transaction` — `crates/aureline-review/src/stabilize_worktree_patch_stack_and_explicit_change_object/mod.rs`
- `automation_runner` — `crates/aureline-workspace/src/mutation_and_generated_artifact_lineage/mod.rs`
- `mutation_journal` — `crates/aureline-workspace/src/mutation_journal/mod.rs`
- `support_export` — `crates/aureline-support/src/generated_lineage/mod.rs`
