# Operational incident workspace worked cases

These fixtures are short, reviewable operational-incident scenarios
that anchor the contract frozen in
[`/docs/ops/incident_workspace_contract.md`](../../../docs/ops/incident_workspace_contract.md)
and validated by:

- [`/schemas/ops/incident_workspace.schema.json`](../../../schemas/ops/incident_workspace.schema.json)
- [`/schemas/ops/runbook_packet.schema.json`](../../../schemas/ops/runbook_packet.schema.json)
- [`/schemas/ops/evidence_handoff_bundle.schema.json`](../../../schemas/ops/evidence_handoff_bundle.schema.json)

Each fixture names the operational severity, lifecycle state, alert
source, mutating-action admission posture, and handoff destination the
case carries, so a reviewer can read the matrix in one pass.

## Coverage matrix

| Case | Severity | Lifecycle | Alert source | Mutating posture | Handoff |
|------|----------|-----------|--------------|------------------|---------|
| [`read_mostly_diagnostic_only.yaml`](./read_mostly_diagnostic_only.yaml) | sev3 | open_triage_in_progress | paging_provider_alert | read_mostly_default_no_mutation_admitted | none |
| [`runbook_mutating_under_runtime_approval.yaml`](./runbook_mutating_under_runtime_approval.yaml) | sev2 | mitigating_in_progress | internal_alert_route | mutating_admitted_under_runtime_approval_ticket | none |
| [`runbook_mutating_under_break_glass.yaml`](./runbook_mutating_under_break_glass.yaml) | sev1 | mitigating_in_progress | audit_or_security_alert | mutating_admitted_under_break_glass_event | none |
| [`browser_handoff_attributable_exit.yaml`](./browser_handoff_attributable_exit.yaml) | sev2 | mitigating_in_progress | paging_provider_alert | mutating_admitted_under_browser_handoff | system_browser |
| [`postmortem_evidence_handoff_bundle.yaml`](./postmortem_evidence_handoff_bundle.yaml) | sev2 | resolved_postmortem_pending | paging_provider_alert | read_mostly_default_no_mutation_admitted | postmortem |
| [`last_synced_offline_replay.yaml`](./last_synced_offline_replay.yaml) | n/a (replay) | imported_replay_no_live_lifecycle | imported_alert_no_live_source | read_mostly_default_no_mutation_admitted | imported_replay |

## Scope rules

- Fixtures validate against their named schema; they do not encode
  wire bytes, ADR-0005 subscription envelopes, or ADR-0004 RPC
  envelopes.
- Raw provider URLs, raw provider payloads, raw stdout / stderr,
  raw stack frames, raw absolute paths, raw command lines, raw
  response bodies, raw alert payloads, raw operator identity strings,
  raw approval-ticket bodies, raw browser-cookie material, and raw
  secret material MUST NOT appear; placeholders of the shape
  `<redacted: <secret_class>>` or opaque handles stand in.
- Incident, runbook, action-ledger, evidence-handoff bundle,
  exact-build identity, branch, commit, code-pointer, alert,
  approval-ticket, break-glass, browser-handoff, console-handoff,
  publish-later, provider-callback, rollback-checkpoint, support-
  bundle, security-incident-packet, advisory, emergency-action, and
  forensic-packet refs are opaque.
- At this milestone there is still no live operational runtime,
  paging-provider integration, observability collector, or admin
  console wired up. These fixtures remain pre-implementation
  governance artifacts.
