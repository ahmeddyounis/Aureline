# Runbook executable steps

A runbook step is not a paragraph of prose or a one-off action button. Once a
runbook's [source authority](m5-runbook-sources.md) is established, every
executable step is a **stable, typed object** that tools, previews, approval
gates, audit records, and support exports all read directly. This document is the
contract for the *executable step* model: what a step declares, how its preview,
approval, and audit behavior are derived mechanically from the object, and why no
step can mint a hidden privileged mutate channel.

The crate `aureline-runbooks` (`m5_runbook_steps`) owns the model. The
machine-readable inventory lives at
[`artifacts/runbooks/m5-runbook-step-library.json`](../../artifacts/runbooks/m5-runbook-step-library.json)
(human summary:
[`artifacts/runbooks/m5-runbook-step-library.md`](../../artifacts/runbooks/m5-runbook-step-library.md)),
and the schema is
[`schemas/runbooks/m5-runbook-step-library.schema.json`](../../schemas/runbooks/m5-runbook-step-library.schema.json).

## What an executable step declares

Each `RunbookExecutableStep` carries:

- a **stable step id** and reviewer-facing label;
- a **step class** — the shared governance taxonomy: `inspect`, `diagnose`,
  `mitigate`, `rollback`, `console_handoff`, `approval`, or `annotate`;
- a **target-selector scope** — an opaque selector ref plus its declared
  *breadth* (`no_target`, `single_target`, `scoped_set`, `environment_wide`, or
  `external_target`) and whether it crosses an environment boundary;
- an **approval scope** — `no_approval_read_only`, `scoped_self_approve`,
  `requires_human_approval`, or `requires_privileged_approval`;
- an **execution mode** — whether the step stays `view_only`, runs
  `in_product_executable`, or is `handoff_only`;
- the **control-plane boundary** it sits on (`in_app_governed`, `browser_handoff`,
  `vendor_console_handoff`, or `auth_boundary_cross`);
- a **command/action-envelope binding** — the shared command/action-envelope it
  routes through and the shared approval authority its gate uses;
- the **expected evidence outputs** it must produce for audit; and
- whether a **companion** may run it within declared scope.

## Steps bind the shared command and approval systems

An executable step never carries its own privileged mutate path. It binds the
**shared command/action-envelope and approval systems** through its
`command_binding`, so the same preview, gate, and audit apply as for any other
governed action:

- Any step that is not purely `view_only` must bind the shared envelope
  (`binds_shared_envelope` and a non-empty `action_envelope_ref`).
- An approval-bearing step must name a shared `approval_authority_ref`; a
  read-only step must not.
- The `uses_runbook_local_bypass` flag is **always false** — a runbook-local
  privileged bypass is never permitted.

## Preview, approval, and audit are derived, not hand-wired

The library carries one `StepGovernanceProjection` per step, computed *from the
step object alone*. Every consuming surface reads the same projection rather than
re-deciding behavior:

- **Preview** — the disposition follows the execution mode and mutation flag:
  `read_only_preview` for view-only/non-mutating steps, `diff_then_confirm` for a
  mutating in-product action, and `handoff_preview` for a boundary crossing.
- **Approval** — `requires_approval` and `requires_explicit_human_approval` follow
  the approval scope, and `approval_routes_through_shared_system` confirms the gate
  runs through the shared approval authority.
- **Audit** — `audit_expects_evidence` and the expected evidence outputs follow the
  step's declared evidence. A mutating or in-product step must declare at least one
  evidence output.
- **Companion** — `companion_may_execute` holds only for a permitted step within
  read-only/self-approve scope that is not a handoff; `companion_may_request`
  covers any non-prohibited step.

## No hidden privileged mutate channel

The validator rejects any step that would mint a hidden privileged mutate channel,
so the projection's `creates_hidden_mutate_channel` is always false:

- a **mutating step with no approval** is an unguarded mutate path;
- a **companion permitted outside** read-only/self-approve scope is a privilege
  escalation channel;
- a **runbook-local bypass** is a privileged path outside the shared envelope; and
- an **in-product executable step that does not bind** the shared envelope has an
  off-books mutate path.

A `view_only` step never mutates and never carries an approval gate; a
`console_handoff` step is `handoff_only` and must leave the governed plane; and a
step's `mutating` flag must match its step class.

## The same step everywhere

The library is exposed on the **desktop UI**, the **companion follow view**, and
**support exports**, and `projections_for_surface` returns the same projection for
each. A step's class, target scope, approval requirement, execution mode, and
expected evidence therefore stay consistent wherever it is previewed, executed,
followed, or exported. The companion follow view
([`fixtures/runbooks/m5-step-library/companion_follow_view.json`](../../fixtures/runbooks/m5-step-library/companion_follow_view.json))
is the same typed objects composed into the subset a companion may drive itself.

This lane governs only how Aureline represents, previews, approves, audits, and
exports already-claimed runbook steps. It does not invent new control planes or
external-console replacements.
