# Generated-artifact regeneration plan

This document describes the *regeneration plan* for generated artifacts: the
typed, reviewable object a regenerate action resolves to **before** it runs. The
canonical packet is implemented in
[`crates/aureline-generated/src/regeneration_plan/mod.rs`](../../crates/aureline-generated/src/regeneration_plan/mod.rs)
and serialized to
[`artifacts/generated/regeneration-plan-packet.json`](../../artifacts/generated/regeneration-plan-packet.json).

The sibling [`generated-artifact governance`](./m5-generated-governance.md)
matrix certifies generated-artifact truth one row per *class*, the
[`generated-artifact descriptor`](./generated-artifact-descriptor.md) models the
per-*artifact* identity object the surfaces render, and the
[`write boundary`](./write-boundary-review.md) lane models what happens when a
user attempts a direct *edit*. This lane models the other half of the writable
boundary — what happens when a user asks to **regenerate** a derived artifact.

## Why this exists

A regenerate button looks safe: press it, get fresh bytes. But a regeneration
can fail for some targets and not others, read stale inputs, be forbidden by
policy, or quietly perform a networked install, a tool download, secret use, or
a broad filesystem write. Without one typed plan, each surface can guess
differently about whether a regeneration was complete, current, reversible, or
safe — and a degraded result can masquerade as success. This lane makes the
plan a first-class object: every regenerate action carries a visible
**side-effect boundary** and **rollback boundary** before execution, and its
outcome is **labeled precisely** so a partial, blocked, stale, or policy-limited
regeneration never reads as a clean rebuild.

## The five readiness states

Every plan names one [`PlanReadiness`] — the headline outcome:

| State | Meaning |
| --- | --- |
| `ready` | Every target will regenerate; no stale inputs, every side effect declared and reviewed. |
| `ready_stale_inputs` | Every target can regenerate, but at least one input is stale, so the result may not reflect the latest source. |
| `partial` | Some targets will regenerate and at least one cannot; the plan applies only partially. |
| `policy_limited` | No target runs and the sole obstruction is a policy block or an undeclared side effect awaiting review. |
| `blocked` | No target runs because required source, generator, or runtime is missing. |

## The planning engine

One engine — `plan_regeneration` — folds a [`RegenerationRequest`] into a
single [`RegenerationPlan`]. Each [`RegenerationTarget`] is planned
independently into a [`TargetPlan`] with one of the [`TargetOutcome`] values,
and the per-target outcomes fold into the plan-level readiness:

| Target outcomes | Plan readiness |
| --- | --- |
| all run, none stale | `ready` |
| all run, some stale | `ready_stale_inputs` |
| some run, some not | `partial` |
| none run, only policy / undeclared side effect | `policy_limited` |
| none run, missing source / generator / runtime | `blocked` |

Per target, the precedence is: a hard block (missing source, generator, or
runtime) outranks a policy block, which outranks a disclosure hold; staleness is
only a flag on an otherwise-runnable target. Every reason is still carried in
the target's `why_blocked_tokens`, even when it is not the deciding one.

## Blocked, partial, stale — but never silent

Every plan that is not fully `ready` carries:

- **`why_blocked_tokens`** — stable tokens naming each input that blocked or
  held a target (e.g. `source_missing`, `generator_unavailable`,
  `runtime_unavailable`, `regeneration_blocked_by_policy`,
  `undeclared_side_effect_network_install`). Empty only when the plan is fully
  ready.
- **`guidance_line`** — a user-visible line that states the readiness in words.
  A partial regeneration says it is *not* complete; a policy-limited or blocked
  plan says it is *not* a regeneration.
- **`recovery`** — the recovery path: regenerate the ready targets, refresh the
  stale inputs, restore the missing source, restore the generator, provision
  the runtime, declare and review the side effect, or resolve the policy.

## No silent side effects

Every plan carries an aggregate [`SideEffectBoundary`] over the side effects its
targets would perform. The four sensitive [`SideEffectClass`] values —
`network_install`, `tool_download`, `secret_access`, and
`broad_filesystem_write` — may not run unless they are declared and reviewed. An
[`SideEffectDisclosure::Undeclared`] sensitive side effect holds its target
([`TargetOutcome::HeldForDisclosure`]) instead of running, so a regeneration can
never hide a networked install, a tool download, secret use, or a broad write.

## An honest rollback boundary

Every plan carries a [`RollbackBoundary`] — the reversible checkpoint that
bounds it — and a computed [`RollbackCoverage`]. Coverage is derived from the
side effects: a regeneration whose writes stay inside the workspace checkpoint
is `fully_reversible`, but one that performs a global install, a tool download,
or a broad write escapes the checkpoint and is reported as
`partially_reversible`. A regenerate action therefore never implies a clean undo
it cannot deliver.

## One plan for every surface

Real consumers bind to the packet:

- `regenerate_plan_sheet` — `crates/aureline-vfs/src/save_conflict_suite/mod.rs`
- `help_regeneration_guide` — `crates/aureline-shell/src/help/mod.rs`
- `support_export` — `crates/aureline-support/src/generated_lineage/mod.rs`
- `release_evidence` — `crates/aureline-release/src/harden_docs_help_about_and_service_health_truth/mod.rs`
- `ai_context` — `crates/aureline-ai/src/context_inspector/mod.rs`

The support export and release evidence preserve the plan packet — its copy
line, why-blocked tokens, side-effect boundary, and recovery path, with no raw
bytes, diffs, or credentials — so regeneration behavior is inspectable after the
fact.

## Regeneration

The proof packet and fixtures are projections of the seeded packet:

```bash
cargo run -q -p aureline-generated --example dump_regeneration_plan -- packet \
  | python3 -c 'import json,sys; print(json.dumps(json.load(sys.stdin), indent=2, sort_keys=True))' \
  > artifacts/generated/regeneration-plan-packet.json
```

The fixture corpus under
[`fixtures/generated/regeneration-plan/`](../../fixtures/generated/regeneration-plan/)
is generated the same way from the `fixtures` mode and split one file per
fixture. The replay gate in
[`crates/aureline-generated/tests/regeneration_plan.rs`](../../crates/aureline-generated/tests/regeneration_plan.rs)
fails CI if the artifact or fixtures drift from the seeded packet.
