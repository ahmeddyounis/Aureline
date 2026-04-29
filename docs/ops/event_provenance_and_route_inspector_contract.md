# Capability boundary, transport route, and event-provenance inspector contract

This document freezes the inspectable object model Aureline uses to
answer four questions for any high-impact action or evidence record:

1. **Where did it run?** (capability boundary)
2. **How did it travel?** (transport route and hop timeline)
3. **Which authority covered it?** (approval ticket or remembered decision)
4. **Which stable IDs prove it happened?** (event / run / session / approval ids)

The goal is to keep boundary truth, route truth, drift truth, and
provenance truth consumable from **one object family** across:
desktop UI, CLI / headless summaries, support / export packets, remote
handoffs, browser-mediated flows, and offline/imported evidence readers.

If this document, the companion schemas, and the worked fixtures
disagree, the normative sources in `.t2/docs/` win and this document
plus its companions update in the same change.

## Companion artifacts

- [`/docs/runtime/origin_target_route_taxonomy.md`](../runtime/origin_target_route_taxonomy.md)
  — canonical `action_route_truth_record` the router, CLI, and support
  exports reference for origin / target / route / exposure / authority
  truth.
- [`/schemas/ops/route_timeline.schema.json`](../../schemas/ops/route_timeline.schema.json)
  — boundary schema for `route_timeline_record`, including boundary
  summary, hop timeline, route-drift banner, and export/redaction
  preview.
- [`/schemas/ops/event_provenance_row.schema.json`](../../schemas/ops/event_provenance_row.schema.json)
  — boundary schema for `event_provenance_row_record`, the stable ID row
  used across logs, diagnostics, artifacts, and audits.
- [`/fixtures/ops/route_drift_cases/`](../../fixtures/ops/route_drift_cases/)
  — worked fixtures covering local-only, remote-agent, external-provider,
  proxy drift, imported evidence, and reconnect replay with drift.

This contract composes with (and does not replace):

- [`/docs/network/transport_governance_seed.md`](../network/transport_governance_seed.md)
  and
  [`/schemas/network/network_attribution_record.schema.json`](../../schemas/network/network_attribution_record.schema.json)
  — proxy / mirror / certificate / egress posture and per-attempt network
  attribution referenced by hop timelines when a hop is network-shaped.
- [`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
  and
  [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  — approval-ticket identity, scope, expiry, and issuer constraints.
- [`/schemas/governance/waiver_expiry.schema.json`](../../schemas/governance/waiver_expiry.schema.json)
  — remembered decisions (expiry, renewal policy, reprompt behavior) that
  gate “do not ask again” flows.
- [`/schemas/execution/run.schema.json`](../../schemas/execution/run.schema.json)
  and
  [`/schemas/commands/command_descriptor.schema.json`](../../schemas/commands/command_descriptor.schema.json)
  — run ids, attempt ids, and invocation-session ids that event
  provenance rows cross-link.
- [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  — execution-context id and trust/policy epoch anchors referenced by the
  policy context on route/provenance packets.

## Normative sources projected here

This contract is a projection of (and must remain consistent with):

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §18.43 “Capability boundary,
  transport route, and event-provenance inspector”, including its
  boundary-class table and drift/replay rules.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Appendix DZ templates for the
  boundary summary card, hop timeline row, event provenance row, route
  drift banner, and export/redaction preview.

## Scope

Frozen at this revision:

- **Boundary summary card** fields for one action or evidence view:
  boundary class, actor/source label, target label, data-class labels,
  approval source summary, and export-safe summary.
- **Route / hop timeline** rows: ordered hops with drift markers and
  optional linkage to a network-attribution record for proxy/mirror/cert
  context.
- **Approval ticket or remembered-decision linkage**: stable refs to the
  authority objects that admitted the route (or admitted reuse).
- **Event provenance rows**: stable event ids and the join keys needed to
  cross-link logs, runs, sessions, approvals, artifacts, and exports.
- **Route drift banner**: changed facts and the required replay /
  reapproval posture when endpoint, tenant, region, proxy, certificate,
  mirror, or policy facts drift.
- **Export/redaction preview**: explicit included route facts and
  explicitly excluded sensitive classes so exports stay useful without
  surprise leakage.

Out of scope at this revision:

- full distributed tracing (span graphs, per-hop latency histograms, or
  backend event pipelines);
- a live UI implementation of the inspector surfaces; and
- any provider-specific raw payload capture (raw URLs, raw hostnames, raw
  tokens, raw request/response bodies).

## The object family

The inspector is built from two core records:

- `route_timeline_record` — one export-safe packet that backs the boundary
  summary card, hop timeline, drift banner, and export preview.
- `event_provenance_row_record` — one stable id row (often attached to the
  `route_timeline_record`) that carries the join keys for durable history,
  diagnostics, support packets, and machine-readable exports.

The inspector **does not** mint parallel authority or route-truth
vocabularies. Instead:

- origin/target/route/exposure/authority tokens come from
  `action_route_truth_record` (see `/docs/runtime/origin_target_route_taxonomy.md`);
- approval truth comes from `approval_ticket_record` or
  `remembered_decision_record` referenced by id; and
- proxy/mirror/certificate posture comes from `network_attribution_record`
  referenced by id when needed.

## Boundary classes and required visible language

The `boundary_class` token is the stable classifier; the UI/CLI label is
the required visible language. Surfaces MUST NOT invent alternative
phrasing for these labels when rendering the same `boundary_class`.

| Boundary class | Meaning | Required visible language |
|---|---|---|
| Local only | Work remained on the current device and local trusted services | `No external route used` |
| Local + isolated extension/runtime | Work crossed into a sandboxed local host | `Ran in isolated local host` |
| Local + remote workspace agent | Work crossed to a remote filesystem/runtime under current workspace authority | `Ran on remote workspace` |
| Local + managed service/control plane | Work relied on an Aureline-managed or enterprise-managed service lane | `Used managed service` |
| Local + external provider | Work crossed to a third-party API, registry, mirror, or AI provider | `Sent to external provider` |
| Browser/webview-mediated | Work moved through an embedded or linked browser surface | `Browser-mediated route` |
| Imported/offline evidence only | Current surface is showing historical or imported data, not a live route | `Showing imported or offline evidence` |

## Route drift, approval renewal, and replay rules

The drift banner exists to prevent silent replay across changed facts.
If a drift condition is known, the system MUST surface it and MUST adopt
the required replay posture; “best effort” replay is forbidden.

| Situation | Required behavior | Forbidden behavior |
|---|---|---|
| Endpoint, tenant, or region changed since planning | Require renewed review and show changed route facts before replay/continue | Silent replay to new target under cached approval |
| Proxy, mirror, or certificate posture changed | Mark route drifted, preserve last-known-good evidence, offer inspect/approve/retry actions | Hide drift behind generic network failure/success |
| Approval expired or narrowed | Restate capability boundary and request fresh decision with current scope | Imply remembered approval still covers widened action |
| Offline or imported evidence reopened | Label as non-live evidence and preserve original route summary where known | Present historical route facts as current live state |
| Queued action resumes after reconnect | Revalidate route, policy, and target drift before replay | Replay queued high-impact actions across changed boundaries |

Additional rules (frozen):

- remembered decisions apply to declared capability and data classes, not
  vague trust. If route class, target, data sensitivity, tenant, or
  policy epoch changes materially, the decision MUST be reconsidered.
- route drift MUST appear consistently across task history, AI evidence,
  provider actions, support exports, and remote/browser handoffs.
- event ids, run ids, session ids, and approval ids MUST remain
  cross-linkable across durable history and exports so investigators do
  not reconstruct truth from screenshots.
- offline/self-hosted/air-gapped profiles MUST make “no external route
  used” a first-class truth, not the absence of a provider badge.

