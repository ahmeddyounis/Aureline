# Operator-truth packet contract (target graph, route, lifecycle, approval lineage)

This document freezes the **operator-truth packet** Aureline emits for
support exports and export-safe diagnostics when a reviewer needs to
reconstruct:

- what command or action was attempted,
- what target and host boundary the action resolved against,
- what route/exposure posture the action used (and whether it changed),
- what lifecycle or connectivity constraints shaped the outcome,
- what approval and install-review lineage admitted or blocked it, and
- what capability posture was in effect at the decision moment.

The goal is **self-sufficient reconstruction without privileged internal
tooling**: a reviewer with only the exported packet can explain *what
happened* and *why the product believed it was the correct/safe thing to
do*, even when sensitive fields are redacted.

Companion artifacts:

- [`/schemas/support/operator_truth_packet.schema.json`](../../schemas/support/operator_truth_packet.schema.json)
  — machine-readable boundary for one `operator_truth_packet_record`.
- [`/fixtures/support/operator_truth_cases/`](../../fixtures/support/operator_truth_cases/)
  — worked packet fixtures covering local-only, provider-routed,
  browser-handoff continuation, wrong-target, reapproval-required,
  missing-capability, and degraded fallback scenarios.

This contract composes with (and MUST NOT mint alternatives to):

- [`/docs/runtime/target_graph_state_projection.md`](../runtime/target_graph_state_projection.md) and
  [`/schemas/runtime/target_graph_state.schema.json`](../../schemas/runtime/target_graph_state.schema.json)
  — target-graph state, host-boundary cues, wrong-target correction,
  and operator-truth projection vocabulary.
- [`/docs/runtime/origin_target_route_taxonomy.md`](../runtime/origin_target_route_taxonomy.md)
  — origin/target/route/exposure vocabulary, route-change reason codes,
  and authority linkage rules.
- [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md) and
  [`/schemas/commands/command_descriptor.schema.json`](../../schemas/commands/command_descriptor.schema.json)
  — invocation-session packet contract (enablement, preview/approval
  posture, outcome, and stable evidence refs).
- [`/docs/trust/capability_sheet_contract.md`](../trust/capability_sheet_contract.md) and
  [`/schemas/trust/capability_sheet.schema.json`](../../schemas/trust/capability_sheet.schema.json)
  — capability sheet and reduced-mode vocabulary.
- [`/docs/trust/approval_ticket_lifecycle_contract.md`](../trust/approval_ticket_lifecycle_contract.md),
  [`/schemas/trust/approval_ticket.schema.json`](../../schemas/trust/approval_ticket.schema.json),
  and
  [`/schemas/trust/approval_event.schema.json`](../../schemas/trust/approval_event.schema.json)
  — approval-ticket and export-safe approval event vocabulary.
- [`/docs/managed/managed_workspace_lifecycle_contract.md`](../managed/managed_workspace_lifecycle_contract.md) and
  [`/schemas/managed/workspace_lifecycle_state.schema.json`](../../schemas/managed/workspace_lifecycle_state.schema.json)
  — managed workspace lifecycle/degraded reason vocabulary.
- [`/docs/runtime/connectivity_and_reconciliation_contract.md`](../runtime/connectivity_and_reconciliation_contract.md) and
  [`/schemas/runtime/connectivity_state.schema.json`](../../schemas/runtime/connectivity_state.schema.json)
  — per-service-family connectivity state used by status surfaces and
  support exports.
- [`/docs/verification/install_review_packet.md`](../verification/install_review_packet.md) and
  [`/schemas/runtime/install_review_fact_grid.schema.json`](../../schemas/runtime/install_review_fact_grid.schema.json)
  — install-review fact-grid projection used by operator-truth
  composites.

Normative sources this contract projects from:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` ordering/replay
  rules and “missing span” disclosure rules for support exports.
- `.t2/docs/Aureline_Technical_Design_Document.md` transport and route
  governance rules (“route choice survives into support exports using
  the same field names and enums”).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` structured packet parity and
  export honesty rules.

If this document disagrees with those sources, those sources win and
this document plus the schema and fixtures must be updated in the same
change.

## Scope

Frozen by this contract:

- A single packet record (`operator_truth_packet_record`) that captures
  the *export-safe truth* needed to reconstruct target, route/exposure,
  lifecycle/connectivity constraints, approval lineage, install-review
  facts, and capability posture for one attempted action.
- Explicit omission and redaction markers so missing data is never
  mistaken for empty data.
- Stable linkage rules back to canonical command descriptors,
  invocation-session packets, target-graph state records, and governed
  capability and approval artifacts (no parallel truth system).

Out of scope:

- Any support backend, ticket submission, or case management tooling.
- Raw logs, raw command lines, raw environment bodies, raw secret bytes,
  raw provider payloads, and raw content bodies.

## Why this exists

Without one shared packet:

- support exports would mix per-surface “where did this run?” copy,
  losing the canonical route tokens required for later audit;
- wrong-target, stale/partial target graphs, and approval withdrawal
  would collapse into generic “failed” outcomes without reconstructable
  lineage; and
- redaction would silently delete key fields, making “missing” look like
  “false”.

The operator-truth packet closes that gap with one export-safe record
whose fields are either present, or explicitly marked as omitted with a
typed reason.

## Packet anatomy

Every `operator_truth_packet_record` carries these major blocks:

| Block | Job |
|---|---|
| `packet_header` | stable packet identity, evidence id, ownership, freshness, and artifact links |
| `command_linkage` | stable linkage to command + invocation-session + execution-context identities |
| `route_truth` | origin/target/route/exposure tokens, route-change reason, and authority linkage refs |
| `target_graph_state` | intended vs resolved target, host-boundary cues, confidence, wrong-target correction, and belief basis |
| `invocation_session_packet` | enablement/preview/approval posture and outcome at the decision moment |
| `lifecycle_context` | managed-workspace lifecycle state and per-family connectivity snapshot (or explicit omission markers) |
| `approval_lineage` | approval ticket + export-safe approval events (or explicit omission markers) |
| `install_review_facts` | install-review fact-grid projection (or explicit omission marker) |
| `capability_state` | capability sheet snapshot (or explicit omission marker) |
| `redaction_context` | redaction profile/class plus withheld/omitted marker refs so gaps are explicit |

## Redaction and omission rules

The packet is export-safe by construction. When a block cannot be
included (policy, user choice, platform limitation, expired source),
the packet MUST carry an explicit `omitted_block_marker` instead of
dropping the field or substituting an empty object/array.

Rules:

1. **No silent nulls for omitted blocks.** `null` is admissible only
   when the underlying contract declares `null` as a meaningful value.
2. **Omission reason is always typed.** Every omitted marker carries an
   `omission_reason_class` aligned with the support-bundle omission
   vocabulary.
3. **Missing span honesty.** If the packet reconstructs a sequence of
   events (approval, handoff, route change) but some span is missing,
   the packet MUST include an omitted marker that makes the gap
   reviewable.

## Linkage rules (no parallel truth)

The operator-truth packet is a *projection* that must stay joinable to
canonical objects:

- `command_linkage.command_id` and `.invocation_session_id` MUST match
  the embedded `invocation_session_packet` (when present).
- `route_truth.invocation_session_ref` and `.execution_context_ref`
  MUST match `command_linkage` (when non-null).
- `target_graph_state.graph_summary.execution_context_id_ref` (when
  present) MUST match `command_linkage.execution_context_id`.
- `route_truth.authority_linkage_ref.*` refs MUST match the embedded
  approval ticket / approval events (when present) or be explicitly
  omitted with a marker (never silently absent).

## Required example scenarios

The fixture corpus under
[`/fixtures/support/operator_truth_cases/`](../../fixtures/support/operator_truth_cases/)
includes at least:

- local-only action (no managed/provider boundary);
- provider-routed action (provider-visible route/exposure tokens);
- browser-handoff continuation (authority linkage via browser-handoff);
- wrong-target attempt (wrong-target correction explicit);
- reapproval-required mutation (approval posture and reason explicit);
- missing capability (typed disabled reason + repair hook);
- degraded local fallback after service impairment (connectivity state
  and route-change reason remain explicit).

