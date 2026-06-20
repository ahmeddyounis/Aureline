# Generated-artifact mutation guardrails

This document describes the *mutation guardrails* that govern the automated
routes — AI apply, refactor, quick fix, and automation — that can reach a
generated artifact. The canonical packet is implemented in
[`crates/aureline-generated/src/mutation_guardrails/mod.rs`](../../crates/aureline-generated/src/mutation_guardrails/mod.rs)
and serialized to
[`artifacts/generated/mutation-guardrails-packet.json`](../../artifacts/generated/mutation-guardrails-packet.json).

The sibling [`write boundary`](./write-boundary-review.md) lane decides what
happens when a *user* attempts a direct edit to a generated file. This lane
reuses that decision and layers the *automated-route* requirements on top: an
AI apply or an automation pass can reach the same generated artifacts, and
without one typed decision it could mutate a derived file as if it were
ordinary user-authored source — silently, with no preview, no side-effect
summary, no regeneration awareness, and no rollback class.

## Why this exists

A generated file looks like any other file on disk, so an AI patch apply, a
refactor transaction, a quick fix, or an automation pass can write to it the
same way it writes to a hand-authored source file. This lane makes that a
first-class decision: the automated mutation is **admitted directly** (the
target is its own canonical source and in sync), **admitted across the
boundary** (with a complete safety envelope and a recorded reviewed override),
**held for review**, **blocked in favor of regeneration**, or **blocked because
the target carries no canonical-source boundary data at all** — always with a
visible reason, a recorded actor lineage, and a support summary.

## The four automated routes

| Route | Journal actor class |
| --- | --- |
| `ai_apply` | `ai_apply` |
| `refactor` | `refactor_engine` |
| `quick_fix` | `code_action` |
| `automation` | `build_runner` |

A direct human edit is covered by the sibling write-boundary lane.

## The decision engine

One engine — `decide_mutation_guardrail` — folds the mutation attempt into a
single [`MutationGuardrailDecision`]. It reuses `decide_write_boundary` for the
underlying boundary classification and layers the route requirements on top:

| Condition | Guardrail outcome |
| --- | --- |
| No canonical-source boundary data | `blocked_missing_boundary_data` |
| Boundary admits a direct edit (own canonical source, in sync) | `admitted_direct` |
| Boundary is regenerate-only | `blocked_regenerate_first` |
| Boundary admits a recorded override **and** the envelope is complete | `admitted_with_preview_and_override` |
| Boundary holds for review, or the envelope is incomplete | `blocked_pending_review` |

The guardrail is strictly *stricter* than the write boundary alone: even when
the write boundary would admit a recorded override, the mutation is held unless
the route also carried a complete safety envelope.

## The safety envelope

Any allowed cross-boundary mutation must satisfy all four
[`SafetyRequirement`]s; an unmet requirement holds the mutation and is named in
the `why_blocked_tokens`:

| Requirement | Met when | Unmet token |
| --- | --- | --- |
| `preview` | a preview of the change is supplied | `missing_preview` |
| `side_effect_summary` | the side effects are declared and reviewed | `undeclared_side_effects` |
| `regeneration_awareness` | the route acknowledges the artifact is regenerated | `regeneration_not_acknowledged` |
| `rollback_class` | a reversible (non-audit-only) reversal class is declared | `no_rollback_class` |

The side-effect classes and the rollback coverage are the same ones the
[regeneration plan](./regeneration-plan.md) lane defines: a networked install,
a tool download, or a broad write escapes the workspace checkpoint, so the
rollback is only `partially_reversible`. An undeclared sensitive side effect is
never run silently.

## No silent mutation, and no hidden mutation path

- An automated route never mutates a *non-authoritative* generated artifact as
  if it were ordinary source: `admitted_direct` is reached only when the write
  boundary admits a direct edit (the artifact is its own canonical source and
  in sync).
- A target with no canonical-source boundary data is blocked outright — it
  cannot be classified, treated as ordinary source, or admitted through an
  override.
- Every decision records [`ActorLineage`] — the route, the journal actor class,
  the source class, the actor reference, the mutation class, and the reversal
  class — and reuses the write-boundary decision and the regeneration
  side-effect / rollback vocabulary, so no route gets a generator-specific
  hidden mutation path and support and audit packets can explain exactly which
  route crossed the canonical boundary and under what posture.

## One decision for every surface

Real consumers bind to the packet:

- `ai_apply_gate` — `crates/aureline-ai/src/harden_ai_scoped_apply/mod.rs`
- `refactor_transaction` — `crates/aureline-review/src/stabilize_worktree_patch_stack_and_explicit_change_object/mod.rs`
- `automation_runner` — `crates/aureline-workspace/src/mutation_and_generated_artifact_lineage/mod.rs`
- `mutation_journal` — `crates/aureline-workspace/src/mutation_journal/mod.rs`
- `support_export` — `crates/aureline-support/src/generated_lineage/mod.rs`

## Regeneration

The proof packet and fixtures are projections of the seeded packet:

```bash
cargo run -q -p aureline-generated --example dump_mutation_guardrails -- packet \
  | python3 -c 'import json,sys; print(json.dumps(json.load(sys.stdin), indent=2, sort_keys=True))' \
  > artifacts/generated/mutation-guardrails-packet.json
```

The fixture corpus under
[`fixtures/generated/mutation-guardrails/`](../../fixtures/generated/mutation-guardrails/)
is generated the same way from the `fixtures` mode and split one file per
fixture. The replay gate in
[`crates/aureline-generated/tests/mutation_guardrails.rs`](../../crates/aureline-generated/tests/mutation_guardrails.rs)
fails CI if the artifact or fixtures drift from the seeded packet.
