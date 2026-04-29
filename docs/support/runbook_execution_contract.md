# Runbook execution contract

This document freezes the support-facing contract for guided runbooks:
the packet a responder reviews before a run, the result record emitted
for each step, and the approval, rollback, incident-timeline, command-
result, and support-export refs that make the run reconstructable after
the incident or support case has moved on.

The operational incident workspace contract already defines the incident
object family and the action ledger. This packet sits beside it for
support and export readers: it gives runbook authors, execution
surfaces, and post-incident reviewers one portable shape for planned
steps, simulated steps, approvals, rollbacks, downgrades, and evidence
outputs.

If this document, the companion schemas, and the worked fixtures
disagree, the normative sources in `.t2/docs/` win and this document
plus its companions update in the same change.

## Companion artifacts

- [`/schemas/support/runbook_packet.schema.json`](../../schemas/support/runbook_packet.schema.json)
  -- boundary schema for `support_runbook_packet_record`. It carries
  runbook version, owner, source-document freshness, supported target
  classes, prerequisites, secret needs, approval requirements, rollback
  posture, evidence outputs, compatibility window, step contracts, and
  downgrade rules.
- [`/schemas/support/runbook_step_result.schema.json`](../../schemas/support/runbook_step_result.schema.json)
  -- boundary schema for `support_runbook_step_result_record`. It emits
  one append-only result per step attempt with state, acting surface,
  target context, approval linkage, rollback linkage, command-result
  refs, incident-timeline refs, support/export refs, and follow-up gates.
- [`/fixtures/support/runbook_cases/`](../../fixtures/support/runbook_cases/)
  -- worked pre-implementation fixtures covering packet review,
  simulation, started, skipped, blocked, retried, completed, and
  rolled-back step outcomes.

This contract composes with, and does not replace:

- [`/docs/ops/incident_workspace_contract.md`](../ops/incident_workspace_contract.md),
  [`/schemas/ops/runbook_packet.schema.json`](../../schemas/ops/runbook_packet.schema.json),
  and
  [`/schemas/ops/evidence_handoff_bundle.schema.json`](../../schemas/ops/evidence_handoff_bundle.schema.json)
  for incident workspaces, runbook/action-ledger refs, and immutable
  postmortem handoff bundles.
- [`/docs/commands/invocation_result_and_parity_contract.md`](../commands/invocation_result_and_parity_contract.md),
  [`/schemas/commands/result_packet.schema.json`](../../schemas/commands/result_packet.schema.json),
  and
  [`/schemas/commands/rollback_handle.schema.json`](../../schemas/commands/rollback_handle.schema.json)
  for command outcomes, checkpoint refs, and rollback handles.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  and
  [`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
  for approval-ticket issuance, spend, revocation, and browser handoff.
- [`/schemas/runtime/target_context.schema.json`](../../schemas/runtime/target_context.schema.json)
  and
  [`/schemas/runtime/live_action_envelope.schema.json`](../../schemas/runtime/live_action_envelope.schema.json)
  for live target identity, drift, and action-safety review.
- [`/docs/support/support_bundle_contract.md`](./support_bundle_contract.md),
  [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json),
  and
  [`/schemas/support/support_packet_index.schema.json`](../../schemas/support/support_packet_index.schema.json)
  for support/export packet inclusion and reconstruction.

## Normative sources projected here

- `.t2/docs/Aureline_Technical_Architecture_Document.md` section 19.7
  and Appendix CN: incident workspaces are read-mostly by default,
  runbook steps record actor, timestamp, outcome, evidence, and
  deviation notes, and exports preserve runbook/action-ledger joins.
- `.t2/docs/Aureline_Technical_Design_Document.md` section 7.6.11.7,
  Appendix V, and Appendix DJ: observe, verify, mitigate, rollback, and
  communicate steps have distinct approval, evidence, and rollback
  obligations.
- `.t2/docs/Aureline_PRD.md` operational supportability, diagnostics,
  incident, approval, and rollback requirements.

## Scope

Frozen at this revision:

- one `support_runbook_packet_record` shape for reviewable runbook
  guidance before a run or simulation;
- one `support_runbook_step_result_record` shape for every step attempt;
- closed vocabularies for step classes, execution states, acting
  surfaces, target-context states, approval states, rollback states,
  downgrade triggers, and follow-up gates;
- linkage rules from runbook steps to command result packets, approval
  tickets, rollback handles, incident timeline entries, support/export
  packets, and operational action-ledger entries; and
- downgrade rules for stale docs, missing prerequisites, changed target
  context, revoked approvals, and partial completion discovered mid-run.

Out of scope:

- a runbook editor, incident-room UI, live action runner, hosted support
  upload, vendor-console automation, or approval-ticket issuer;
- raw command lines, raw provider URLs, raw provider payloads, raw logs,
  raw terminal transcripts, raw approval-ticket bodies, raw operator
  identities, or raw secrets; and
- changing the operational incident-workspace action ledger. This packet
  cites that ledger by stable refs.

## Runbook packet

A `support_runbook_packet_record` is the reviewable packet a surface
presents before it offers simulation or execution. It is not authority by
itself. Authority comes from the current policy epoch, target context,
workspace trust state, approval ticket, and acting surface.

Every packet records:

| Block | Required meaning |
|---|---|
| `owner` | DRI, escalation owner, review cadence, and optional backup owner |
| `source_document` | source ref, revision, provenance class, and freshness state |
| `compatibility_window` | build/schema window and behavior after the window expires |
| `supported_target_classes` | target classes the packet may run or simulate against |
| `prerequisites` | typed gates that must be satisfied, skipped, simulated, or block |
| `secret_needs` | broker-handle-only secret needs and behavior when unavailable |
| `default_approval_requirements` | per-action approval posture and evidence requirement |
| `default_rollback_posture` | whether rollback is exact, checkpointed, compensating, manual, or audit-only |
| `evidence_output_contract` | evidence rows expected for each step state |
| `step_contracts` | ordered step grammar with approval, rollback, target, and evidence refs |
| `linkage_contract` | required joins to command results, timelines, exports, approvals, and rollbacks |
| `downgrade_rules` | fail-closed behavior when preconditions drift |

Runbook text never grants permission. A mutating or production-tagged
step remains blocked until the packet's step contract, current target
context, policy epoch, and approval requirement all agree.

## Step result states

Each attempt emits exactly one `support_runbook_step_result_record`.
Corrections and retries append new records and point at the prior result;
they do not rewrite earlier records.

| State | Meaning | Required reconstruction refs |
|---|---|---|
| `simulated` | dry run, preview, or imported replay only | step ref, acting surface, target context, evidence refs |
| `started` | live step accepted and currently running | approval state, target fingerprint, incident timeline ref |
| `skipped` | optional or policy-disallowed step intentionally skipped | skip reason, actor, follow-up gate when needed |
| `blocked` | step did not run because a required gate failed | downgrade trigger, blocked gate, follow-up owner |
| `retried` | a later attempt supersedes or repeats a typed failure | retry reason, prior result ref, retry budget |
| `completed` | step finished and produced required result/evidence refs | command result refs or evidence refs, timeline/export refs |
| `rolled_back` | partial or completed effect was restored or compensated | rollback handle/ref, resulting state evidence, timeline/export refs |

`started` is intentionally non-terminal. It must be followed by a later
terminal record, provider callback, support export row, or follow-up gate
that explains why the run remains open.

## Approval and rollback linkage

Approval and rollback are explicit per step; they are not inherited from
the surrounding incident or support case.

Rules:

- read-only observe and verify steps may use
  `approval_state = not_required`, but they still record the acting
  surface and target context;
- simulation does not mint authority. A later live attempt must
  re-resolve target context and approval against the current policy
  epoch;
- protected-target mitigation, external communication, browser/vendor
  console handoff, rollback execution, and break-glass actions cite an
  approval-ticket, policy-grant, break-glass, or handoff ref according
  to their step contract;
- rollback handles are evidence, not ambient authority. Spending a
  handle reruns trust, policy, target-context, credential, preview, and
  approval checks;
- rollback execution uses the same or stricter authority as the forward
  action when policy requires it; and
- command result packets, operational action-ledger entries, incident
  timeline entries, and support/export rows all remain separate refs so
  a postmortem can reconstruct what was proposed, approved, run,
  skipped, retried, completed, or rolled back without reading raw
  command payloads.

## Downgrade rules

The packet declares downgrade rules so surfaces fail consistently:

| Trigger | Default downgrade |
|---|---|
| `stale_runbook_documentation` | block mutation or downgrade to simulation until source docs refresh |
| `missing_prerequisite` | block the affected step or entire runbook according to prerequisite severity |
| `changed_target_context` | require target re-resolution and reapproval before live execution |
| `revoked_approval` | block live execution and append an approval follow-up gate |
| `partial_completion_mid_run` | require rollback, compensation, or an explicit deviation note before continuing |
| `policy_epoch_expired` | re-evaluate policy and approval from the current epoch |
| `secret_projection_unavailable` | block secret-bearing steps; never fall back to raw secret capture |
| `rollback_posture_unavailable` | block mutation or require audit-only disclosure before reviewer acceptance |
| `evidence_sink_unavailable` | block completion claims until evidence can be captured or an omission is recorded |
| `acting_surface_not_authoritative` | hand off to desktop/CLI or another governed surface before mutation |

Downgrades are visible result states, not hidden UI disabled states. The
blocked, skipped, retried, or rolled-back result carries the trigger,
downgrade state, follow-up gate, and owner.

## Export reconstruction

A support export or postmortem reconstructs a run from these joins:

1. open the support/export packet and follow `runbook_packet_ref`;
2. read ordered `step_contracts` from the packet;
3. gather all `support_runbook_step_result_record` rows for the packet;
4. follow `command_result_packet_refs` and operational
   `action_ledger_entry_refs` for executed steps;
5. follow approval-ticket and rollback-handle refs for mutating and
   rollback steps;
6. follow incident-timeline refs for chronology; and
7. inspect follow-up gates for blocked, skipped, retried, or open
   started steps.

If any required join is missing, the export remains usable but records a
typed gap (`missing_required_blocked`, `redacted_by_policy`, or
`unavailable`) instead of implying success.

## Versioning

The support schemas use integer schema versions. Adding optional fields
or enum values is additive-minor. Repurposing an existing enum value,
loosening a mutating-step approval rule, allowing raw payloads through
these boundaries, or treating a runbook packet as authority is breaking
and requires a new review row.
