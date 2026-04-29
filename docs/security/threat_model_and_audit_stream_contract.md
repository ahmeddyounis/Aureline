# Threat-model, audit-stream, and evidence-window contract

This document freezes the shared security vocabulary that advisories,
incidents, approval tickets, support exports, admin exports, and
postmortems use when they cite threats and audit evidence. It binds
threat classes to concrete audit streams and evidence windows so later
implementation work does not scatter the same facts across unrelated
packets.

Companion artifacts:

- [`/artifacts/security/threat_classes.yaml`](../../artifacts/security/threat_classes.yaml)
  - machine-readable threat catalog and minimum audit-stream map.
- [`/schemas/security/audit_stream_record.schema.json`](../../schemas/security/audit_stream_record.schema.json)
  - strict boundary for `audit_stream_record`.
- [`/schemas/security/evidence_window.schema.json`](../../schemas/security/evidence_window.schema.json)
  - strict boundary for `evidence_window_record`.
- [`/fixtures/security/audit_stream_cases/`](../../fixtures/security/audit_stream_cases/)
  - worked records covering required stream families and evidence
  windows.
- [`/docs/security/severity_matrix.md`](./severity_matrix.md),
  [`/docs/security/advisory_surface_contract.md`](./advisory_surface_contract.md),
  and [`/docs/security/emergency_action_model.md`](./emergency_action_model.md)
  - advisory, incident, severity, emergency-action, and revocation
  records that cite this vocabulary by stable refs.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  - support/export packet rules this contract composes with.
- [`/docs/collaboration/shared_control_contract.md`](../collaboration/shared_control_contract.md)
  - collaboration control-grant and revocation vocabulary.
- [`/docs/policy/admin_policy_and_bundle_cache_contract.md`](../policy/admin_policy_and_bundle_cache_contract.md)
  - policy-source and signed bundle-cache vocabulary.

Normative source alignment:

- `.t2/docs/Aureline_PRD.md` sections on collaboration auditability,
  managed identity, tenant separation, data portability, support
  exports, and security response.
- `.t2/docs/Aureline_Technical_Design_Document.md` Appendix X
  threat matrix, Appendix CV/CW retention and export honesty, and
  Appendix DI evidence-window vocabulary.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` security,
  policy, supportability, remote, collaboration, and managed-service
  boundary passages.

If this contract disagrees with those source documents, the source
documents win and this file, schemas, fixtures, and catalog update
together.

## Why this exists

Security reviews need a common language before implementation creates
separate logs for extensions, policy, AI, collaboration, remote
sessions, incidents, and support. Without one contract:

- an advisory can claim "recent evidence" without naming the actual
  current, reviewable, or exportable window;
- a support export can show a blank field that a reader interprets as
  "never recorded" even though the field was redacted or expired;
- a collaboration grant can be audited as ordinary presence even though
  it raised control authority;
- a managed-tenant boundary issue can be treated as a generic support
  event instead of a key or isolation failure; and
- emergency action review can lose the link between actor, policy
  source, approval ticket, evidence, redaction, and export posture.

This contract closes those gaps by freezing three pieces:

1. A threat-class catalog.
2. A strict audit-stream record.
3. An evidence-window record and omission semantics.

## Scope

Frozen here:

- ten threat classes:
  `credential_theft`, `supply_chain_compromise`,
  `unauthorized_mutation`, `data_exfiltration`,
  `trust_boundary_confusion`, `provider_spoofing`,
  `stale_evidence_misread`, `unsafe_emergency_action`,
  `hostile_collaborator_session_grant_abuse`, and
  `tenant_isolation_key_management_failure`;
- audit-stream records that always link actor, route, object, target
  context, decision, policy source, approval-ticket posture, threat
  refs, evidence refs, evidence-window ref, redaction class, export
  posture, and field-disposition rows;
- evidence-window states for `current`, `reviewable`, `exportable`,
  `expired`, `redacted`, and `legally_retained`;
- explicit absence, redaction, expiry, legal-hold, policy-withheld,
  unavailable-source, and outside-platform-scope rules; and
- minimum stream families for extension lifecycle, policy-bundle
  changes, workspace-trust changes, AI tool and apply actions,
  collaboration elevated-control grants and revocations, and remote
  join and leave events.

Out of scope:

- SIEM ingestion, detection engines, or alert routing;
- final incident-response tooling or advisory publication automation;
- raw telemetry, raw terminal bytes, raw provider payloads, raw code
  content, raw secrets, raw tenant names, or raw human identifiers; and
- a replacement for existing advisory, emergency-action, support,
  policy, or collaboration contracts.

## Threat classes

Threat classes live in
[`threat_classes.yaml`](../../artifacts/security/threat_classes.yaml).
Every advisory claim, incident packet, support export, and audit
stream that asserts a security threat cites one or more of those ids.

| Threat class | Primary concern | Minimum audit coverage |
|---|---|---|
| `credential_theft` | Secret, token, credential-handle, signing, or delegated-grant exposure | secret use, AI tool calls, extension lifecycle, support export |
| `supply_chain_compromise` | Compromised build, extension, dependency, registry, mirror, policy bundle, or update | extension lifecycle, policy-bundle change, advisory or emergency linkage |
| `unauthorized_mutation` | State mutation without the required trust, policy, approval, actor, or preview posture | workspace trust, policy bundle, AI apply, collaboration grant |
| `data_exfiltration` | Data leaving a local, tenant, provider, remote, or support boundary without admitted redaction/export posture | AI tool, support export, remote session, collaboration archive |
| `trust_boundary_confusion` | Actor or surface treating local, remote, browser, provider, managed, or tenant boundaries as interchangeable | workspace trust, remote join/leave, AI tool |
| `provider_spoofing` | Provider, callback, handoff, link, registry, mirror, or app grant not matching expected host, tenant, actor, or target | provider handoff, remote join, policy change, incident linkage |
| `stale_evidence_misread` | Cached, partial, expired, redacted, imported, or offline evidence treated as live and complete | support export, incident/advisory linkage, policy bundle, remote leave |
| `unsafe_emergency_action` | Freeze, kill switch, revocation, trust-root, or emergency policy action missing quorum, expiry, signer continuity, or reconciliation | emergency action, policy bundle, advisory linkage |
| `hostile_collaborator_session_grant_abuse` | Collaboration role, presence, presenter, or shared-control grant widening or replaying authority | collaboration grant/revoke, remote join/leave |
| `tenant_isolation_key_management_failure` | Managed data, audit stream, cache, key, policy, AI memory, or admin action crossing tenant, region, key, or offboarding boundaries | policy change, remote join/leave, support export |

Rules:

1. A threat claim MUST cite at least one threat-class id. Free-form
   labels such as "security issue" or "suspicious" are not enough.
2. If a finding touches managed tenant state, key state, managed audit
   storage, AI memory, or customer-managed keys, it MUST consider
   `tenant_isolation_key_management_failure` even when another threat
   class is primary.
3. If a finding touches collaboration elevated control, presenter
   handoff, shared terminal/debug/runbook/kernel authority, or grant
   replay, it MUST consider
   `hostile_collaborator_session_grant_abuse`.
4. A support export or advisory may summarize threat evidence, but it
   must preserve the cited threat ids and evidence-window refs.

## Audit-stream record

Every audit-stream row validates as `audit_stream_record`. The record
is strict (`additionalProperties: false`) so security and support
exports cannot carry surface-local fields that other reviewers cannot
parse.

Required groups:

| Group | Purpose |
|---|---|
| `actor` | Who acted, under which actor class, and whether the display identity was redacted |
| `route` | Which local, remote, managed, provider, browser, extension, collaboration, CLI, or offline path carried the action |
| `object` | The subject being changed, inspected, denied, exported, granted, or revoked |
| `target_context` | The local, remote, provider, tenant, managed workspace, release channel, policy bundle, or collaboration target context |
| `decision` | The allow/deny/apply/revoke/quarantine/export/no-op decision and reason code |
| `policy_source` | The source layer, policy ref, policy epoch, decision ref, freshness, and explain ref |
| `approval_ticket` | Whether approval was not required, valid, spent, missing, expired, revoked, or redacted |
| `threat_refs` | Threat-class ids from the catalog |
| `evidence_refs` | Opaque evidence refs, their evidence windows, embedding state, and collection state |
| `redaction` | Redaction class, profile ref, and redacted field paths |
| `export` | Export posture, destination class, and manifest ref |
| `field_dispositions` | One row per omitted or present field that matters to export honesty |

### Stream families

Minimum stream families are frozen in the catalog and admitted by the
schema:

- `extension_lifecycle`
- `policy_bundle_change`
- `workspace_trust_change`
- `ai_tool_action`
- `ai_apply_action`
- `collaboration_control_grant`
- `collaboration_control_revocation`
- `remote_session_join`
- `remote_session_leave`
- `advisory_incident_linkage`
- `support_export`
- `emergency_action`

The first nine are the minimum implementation-bearing streams for
security review. The final three are join points so advisory,
incident, support, and emergency packets can cite the same vocabulary
without minting parallel audit rows.

## Evidence windows

Every audit stream cites an `evidence_window_ref`; the referenced
record validates as `evidence_window_record`. Evidence windows replace
phrases like "recent logs," "current capture," "available evidence,"
or "retained for review" with explicit state rows.

| Window state | Meaning | Export effect |
|---|---|---|
| `current` | The event or signal is inside its accepted freshness window | may embed or cite live-current evidence |
| `reviewable` | A reviewer may inspect the row or refs for security, support, advisory, or postmortem work | may export metadata or references |
| `exportable` | The row may leave its source boundary under the declared destination and redaction profile | may embed when the profile admits it |
| `expired` | The ordinary retention or export window has elapsed | omitted fields use `omitted_by_expiry` |
| `redacted` | Data exists but has been withheld or transformed under a redaction profile | export redacted summaries or metadata |
| `legally_retained` | A legal, incident, support, or administrative hold preserves metadata or refs beyond ordinary expiry | export metadata-only or reference-only rows |

Rules:

1. A record can carry multiple active state rows. For example, a row
   can be `reviewable`, `redacted`, and `legally_retained`.
2. `current` is not the same as `reviewable`; an expired live window
   may still be reviewable under a postmortem hold.
3. `exportable` is destination-specific. A record can be exportable to
   an admin export while blocked from a public advisory.
4. `redacted` means data exists or existed and is intentionally
   withheld; it is not absence.
5. `legally_retained` is metadata-visible even when the retained body
   is not embedded in a support export.

## Absence, redaction, and expiry

Exported audit data must remain honest about what is present, withheld,
expired, held, unavailable, or outside the product's scope. Therefore:

- A blank field never means "never recorded" unless a
  `field_disposition` row says `not_recorded_by_design`.
- A redacted field uses `omitted_by_redaction` and names the redaction
  class or profile.
- An expired field uses `omitted_by_expiry` and names the evidence
  window that expired.
- A legally held field uses `omitted_by_legal_hold` or cites a
  `legally_retained` evidence-window state.
- A policy-withheld field uses `omitted_by_policy` and cites the
  policy source.
- An unreachable backend or missing imported packet uses
  `unavailable_source`, not `not_recorded_by_design`.
- Material the product never possessed uses `outside_platform_scope`.

The audit-stream schema requires `absence_summary` and at least one
field-disposition row. Support and admin exports may summarize these
rows, but they must not delete them.

## Minimum stream semantics

### Extension Lifecycle

Extension lifecycle audit streams cover install, update, enable,
disable, quarantine, removal, activation denial, permission changes,
and marketplace or mirror source changes. Minimum evidence includes
extension id, version/provenance ref, source/mirror posture, policy
decision ref, actor, target context, and evidence window.

Required threat coverage includes `supply_chain_compromise`,
`credential_theft`, and `unauthorized_mutation`.

### Policy-Bundle Changes

Policy-bundle audit streams cover receipt, validation, application,
rejection, expiry, last-known-good fallback, emergency disable, and
managed override changes. Minimum evidence includes policy bundle ref,
policy epoch, source layer, signature or verification ref, decision
ref, affected target context, and evidence window.

Required threat coverage includes `unauthorized_mutation`,
`trust_boundary_confusion`, `unsafe_emergency_action`, and
`tenant_isolation_key_management_failure`.

### Workspace-Trust Changes

Workspace-trust audit streams cover trust grants, revocations,
policy narrowing, expiry, restricted opens, emergency force-restricted
actions, and identity-gate unavailable states. Minimum evidence
includes actor, trust decision ref, workspace or target context,
policy source, approval-ticket posture, and evidence window.

Required threat coverage includes `trust_boundary_confusion`,
`unauthorized_mutation`, and `stale_evidence_misread`.

### AI Tool And Apply Actions

AI tool streams cover tool-call request, denial, execution, result
redaction, provider route, and context-export posture. AI apply streams
cover preview creation, approval, apply, rejection, revert, and
checkpoint linkage. Minimum evidence includes actor, AI/provider route,
tool or diff ref, target context, policy decision, approval-ticket
posture when required, redaction class, export posture, and evidence
window.

Required threat coverage includes `credential_theft`,
`data_exfiltration`, `provider_spoofing`,
`trust_boundary_confusion`, `unauthorized_mutation`, and
`stale_evidence_misread` depending on the action.

### Collaboration Elevated Control

Collaboration streams distinguish presence, follow, presenter state,
and elevated control. A shared-control grant is not inferred from a
role badge, presenter handoff, or presence row. Minimum evidence
includes session ref, grant or revocation ref, grantee actor ref,
authority lane, target context, approval-ticket posture when required,
recording/retention posture, and evidence window.

Required threat coverage includes
`hostile_collaborator_session_grant_abuse`,
`unauthorized_mutation`, and `trust_boundary_confusion`.

### Remote Join And Leave

Remote join and leave streams cover admitted joins, denied joins,
leave observation, revocation, timeout, reconnection, and target
identity drift. Minimum evidence includes actor, remote session ref,
route ref, target context, tenant or region refs where applicable,
policy source, decision, and evidence window.

Required threat coverage includes `trust_boundary_confusion`,
`provider_spoofing`, `tenant_isolation_key_management_failure`, and
`hostile_collaborator_session_grant_abuse` where collaboration is
involved.

## Review and change discipline

Adding a new threat class, stream family, field-disposition value,
evidence-window state, actor class, route class, object kind, target
kind, decision outcome, redaction class, or export posture is
additive-minor and requires the catalog, schema, fixtures, and
governance index to update together. Repurposing an existing value is
breaking and requires security and release review.

Rows are superseded, not rewritten in place. Exporters preserve the
prior row and add a correcting record so incidents, advisories,
support packets, and postmortems can reconstruct the actual sequence
of decisions.
