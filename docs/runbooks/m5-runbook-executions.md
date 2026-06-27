# Runbook execution history

A runbook execution is not a privileged side-channel. Once a runbook's
[source authority](m5-runbook-sources.md) is established and its
[executable steps](m5-runbook-steps.md) are governed, every *execution* is itself a
**stable, attributable, export-safe object**. This document is the contract for the
*execution-history* model: what each executed-step row records, how its
preview-hash / approval / audit reuse is derived mechanically, and why a runbook row
reuses Aureline's standard mutation-review machinery rather than a runbook-specific
path.

The crate `aureline-runbooks` (`m5_runbook_executions`) owns the model. The
machine-readable inventory lives at
[`artifacts/runbooks/m5-runbook-execution-history.json`](../../artifacts/runbooks/m5-runbook-execution-history.json)
(human summary:
[`artifacts/runbooks/m5-runbook-execution-history.md`](../../artifacts/runbooks/m5-runbook-execution-history.md)),
the history schema is
[`schemas/runbooks/m5-runbook-execution-history.schema.json`](../../schemas/runbooks/m5-runbook-execution-history.schema.json),
and the per-record schema is
[`schemas/runbooks/m5-runbook-execution.schema.json`](../../schemas/runbooks/m5-runbook-execution.schema.json).
The four canonical operator scenarios are checked in under
[`fixtures/runbooks/m5-operator-scenarios/`](../../fixtures/runbooks/m5-operator-scenarios).

## What an executed-step row records

Each execution record carries the
[source descriptor](m5-runbook-sources.md) whose authority it ran under, the operator
role, whether a companion drove it, and one **executed-step row** per step. Every row
records:

- the **step** that ran (its governed descriptor: class, approval scope,
  control-plane boundary, mutating flag, expected evidence);
- the **actor** accountable for running the row (`actor_ref`) — an operator role or a
  companion session;
- the **target** the row acted on or inspected (`target_ref`), empty when the step has
  no concrete target (an annotation or an approval gate);
- the **outcome** — `completed`, `skipped`, `handed_off`, `awaiting_approval`, or
  `aborted_requires_review`;
- the **preview-hash reuse** (`preview_hash`) and **approval reuse** (`approval_ref`)
  that gated the row;
- the **deviation** lineage entry for the row (`no_deviation` when clean);
- any **control-plane handoff** packet when the row pivoted out of the governed plane;
  and
- the **evidence refs** the row produced for audit.

## Mutating rows reuse the shared preview and approval

A runbook row never carries its own privileged mutate path. Preview and approval reuse
are derived *mechanically* from the step, never hand-wired:

- A **mutating** step (`mitigate`, `rollback`) reuses the shared
  command/action-envelope **preview hash** — the same preview object any other
  governed mutation produces — and the shared **approval authority** ref. A mutating
  row missing either is rejected as a hidden privileged mutate channel.
- Any **approval-bearing** step (self-approve, human, or privileged) reuses the shared
  approval authority; a **read-only** step carries no approval ref.

## Observe / verify / communicate rows carry no fake mutation

An `inspect`, `diagnose`, or `annotate` row records attributable execution and
evidence **without fake mutation semantics**: it carries no preview hash, and no
approval ref unless its scope requires one. A communication annotation, for example,
records who posted it, the note evidence, and an `awaiting_approval` outcome — but it
never pretends to have previewed or mutated anything. A row that carried a preview hash
without mutating is rejected.

## Preview, approval, and audit are derived, not hand-wired

The history carries one `RunbookExecutionRowProjection` per row, computed *from the row
alone*. Every consuming surface reads the same projection rather than re-deciding
behavior:

- **Preview** — `preview_disposition` follows the row: `read_only_preview` for
  observe/verify/communicate rows, `diff_then_confirm` for a mutating in-plane row, and
  `handoff_preview` for a boundary crossing; `reuses_shared_preview` confirms a mutating
  row reused the shared envelope.
- **Approval** — `requires_approval` and `requires_explicit_human_approval` follow the
  scope, and `reuses_shared_approval` confirms the gate routed through the shared
  approval authority.
- **Audit** — `audit_expects_evidence` and `evidence_refs` record what the row produced.
- **Attribution** — `attributable` holds only when the row names an actor and any
  deviation or handoff is itself attributable.

So a history row can always explain **what** ran, **why** (its deviation lineage),
**under which approval**, and **with which evidence outputs**.

## The same history everywhere

The history is exposed on **operator history**, **support exports**, and **incident
packets**, and `projections_for_surface` returns the same projection for each. A row's
class, actor, target, approval, preview reuse, and evidence therefore stay consistent
wherever the history is rendered or exported, and the export carries metadata and refs
only — no credential bodies or raw provider/console payloads.

This lane governs only how Aureline represents, executes, exports, and hands off
already-claimed runbook workflows. It does not invent new control planes or
external-console replacements.
