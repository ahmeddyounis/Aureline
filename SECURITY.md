# Security policy

This file is the baseline security-contact and intake contract for
Aureline. It is the file every advisory record, incident workspace
packet, private-triage workspace packet, emergency-action record,
support-bundle export, and release-evidence packet resolves
`security_md_monitored_contact` against. A report that never lands on a
path below is not on the monitored contact path, and downstream
acknowledge / triage / mitigation / publication clocks do not start.

This is a **pre-implementation** policy. No live on-call rota, paging
system, or emergency-action automation runs against these routes at
this revision. The routes, acknowledgement targets, redaction
expectations, and referenced schemas are normative.

## Scope

- **In scope.** Security issues affecting the Aureline source tree,
  published artifacts (IDE binary, CLI binary, SDK library, debug
  symbols / source-map bundles, docs packs, extensions, install
  bundles, offline / air-gapped bundles, managed-cloud surfaces),
  published schemas, trust-store / signing-material posture, secret-
  broker / redaction posture, capability / workspace-trust posture,
  AI-context-assembly fences, and the governance artifacts published
  under [`artifacts/governance/`](./artifacts/governance/).
- **Out of scope at this revision.** Live on-call rotation
  assignments, paging infrastructure, public-advisory publication on
  day one, coordinated-disclosure embargo scheduling with outside
  parties (the packet model reserves the scope; the runbook is
  `docs/security/intake_and_triage.md`), and third-party service
  issues (file those with the upstream provider; if the upstream flaw
  propagates through an Aureline signed artifact, open a report here
  as well).
- **Also out of scope.** Generic bug reports, feature requests, and
  product feedback. Those belong on the normal issue routes named in
  [`artifacts/governance/issue_routing.yaml`](./artifacts/governance/issue_routing.yaml).

## Reporting a vulnerability

Please report suspected security issues **privately** first. The
monitored contact path is the private security channel defined by the
`security_issue` row in
[`artifacts/governance/issue_routing.yaml`](./artifacts/governance/issue_routing.yaml)
(`default_route_class: private_security_channel`,
`privacy_class: private_with_public_advisory`,
`disclosure_class: public_on_advisory`,
`owning_forum_placeholder: security_trust_review`).

Supported intake routes:

| Route kind                            | Address / target                                                                                                            | Expected use                                                                                                                     |
|---------------------------------------|-----------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------|
| Private email to maintainers          | `security@aureline.dev` (PGP-signed encrypted mail accepted; published key fingerprint below)                                | Default private route for unembargoed and coordinated reports. Use for anything that names a signed artifact, trust root, or secret. |
| Private GitHub advisory               | GitHub "Report a vulnerability" on the Aureline project page (routes into the private security channel)                      | Alternative for reporters who prefer GitHub workflow. Do not file a public issue first; escalate to email if the advisory draft tool is unavailable. |
| Coordinated-disclosure group intake   | Existing coordinated-disclosure or vendor-response group the reporter already participates in (CNA, OSV, upstream CNA, etc.) | Use when an upstream flaw propagates through Aureline or when the reporter has an existing embargo with another vendor.          |
| Partner-channel intake                | Partner's existing private channel under a signed partner agreement                                                          | Use when the report is bound by a partner NDA or contractual channel. Aureline acknowledges and escalates through the partner's contact. |

The PGP key fingerprint, full key material, the exact
acknowledge-by-local-clock offset, and the mailbox operator handle are
maintained in
[`docs/security/intake_and_triage.md`](./docs/security/intake_and_triage.md)
so this file does not drift when the key rotates.

**Please do not** open a public GitHub issue, public pull request,
public discussion post, public chat message, social-media post, blog
post, or conference talk about a suspected security issue before the
monitored contact path has acknowledged the report and a coordinated
disclosure window has been agreed. Public disclosure before
coordination is the single most common way recoverable vulnerabilities
become unrecoverable incidents.

## What to include in a report

A report that lets us triage in one round — not three — contains:

- A short, reviewable description of the issue. No raw secret
  material, no raw exploit payloads, no raw binary bytes embedded in
  the body. Placeholders of the shape `<redacted: <secret_class>>` or
  `<exploit_payload_reference>` are preferred; raw bytes may be sent
  as separately-encrypted attachments under the PGP key above.
- Affected build, channel, install lane, and deployment profile where
  known. Use published values from
  [`schemas/build/exact_build_identity.schema.json`](./schemas/build/exact_build_identity.schema.json)
  (build id, channel class), from
  [`artifacts/release/install_topology_matrix.yaml`](./artifacts/release/install_topology_matrix.yaml)
  (install-profile card), and from the deployment-profile vocabulary
  (`individual_local`, `self_hosted`, `enterprise_online`,
  `air_gapped`, `managed_cloud`). Free-text "version 1.x" is accepted
  but will be resolved to an exact build during triage.
- Reproduction steps, minimum configuration, and observed effect. A
  reviewer-rewritable prose form is preferred; link or attach raw
  traces by reference or under redaction rather than embedding.
- The posture you believe the finding falls into: severity class
  (from
  [`docs/security/severity_matrix.md`](./docs/security/severity_matrix.md)),
  subject kind (from
  [`schemas/security/advisory_record.schema.json`](./schemas/security/advisory_record.schema.json)),
  and whether an emergency-action lane (channel freeze, kill switch,
  trust-root rotation, revocation, disable bundle) may be needed.
- Any existing CVE, GHSA, vendor-response, or coordinated-disclosure
  id already assigned. Aureline will reserve its own advisory id
  (`AURELINE-ADV-YYYY-NNNN+`) and treat CVE / GHSA / upstream ids as
  aliases under
  [`schemas/security/advisory_record.schema.json#/$defs/advisory_identity`](./schemas/security/advisory_record.schema.json).

A report that is incomplete is still worth sending. We will ask only
what is needed and will acknowledge regardless.

## Safe disclosure instructions

- **Encrypt raw bytes.** Send raw binary bytes, raw traces, raw
  memory dumps, raw credentials, or raw signing material under the
  published PGP key only. Never paste raw secrets into email bodies,
  issue bodies, chat messages, or screenshots.
- **Do not publish proofs of concept before fix or mitigation.** A
  public proof of concept before a fix or compensating control is
  published widens the blast radius and may move the report's
  severity class upward.
- **Do not test against infrastructure you do not own.** Reports
  based on testing against the hosted / managed-cloud surface, a
  partner's managed deployment, or another user's installation
  require prior authorization and a signed testing agreement.
  Automated scanning of the hosted surface without authorization is
  not in scope for this policy and may be treated as a separate
  matter.
- **Preserve evidence.** Keep the original report and any supporting
  evidence until the advisory is published or the report is closed,
  even if you disclose the finding to another party under a
  coordinated-disclosure agreement.
- **Contact us if you lose the channel.** If the primary address
  bounces or a PGP key rotation is mid-flight, use one of the
  alternative routes above. A report lost in transit still does not
  start the clock; tell us about the loss.

## Acknowledgement targets

Acknowledgement, triage, and fix clocks per severity class are frozen
in
[`docs/security/severity_matrix.md`](./docs/security/severity_matrix.md).
The clocks start when the monitored contact path acknowledges the
report.

| Severity class                             | Acknowledge target                   | Triage target                                | Fix / mitigation target                                                   |
|--------------------------------------------|--------------------------------------|-----------------------------------------------|---------------------------------------------------------------------------|
| `security_severity.operational_emergency`  | Immediate; no business-window delay  | Within 24 hours of monitored acknowledgement  | Immediate containment, channel freeze / pause, or trust-root action       |
| `security_severity.critical`               | Within 24 hours                      | Within 72 hours                               | Within 7 days or a compensating control                                   |
| `security_severity.high`                   | Within 24 hours                      | Within 5 days                                 | Within 30 days                                                            |
| `security_severity.medium`                 | Within 3 business days               | Within 10 days                                | Within 90 days                                                            |
| `security_severity.low`                    | Within 5 business days               | Within 20 days                                | Next appropriate release                                                  |

Clocks stop only when the advisory is published, the report is
withdrawn on the reporter's request, or the finding is closed as a
duplicate. Extensions and severity reclassifications land as decision-
history rows on the advisory record; we do not backdate.

## Channel-specific expectations for security issues

- **No public issue or discussion.** A suspected security issue filed
  on a public issue board or public discussion board is closed
  and re-routed by a maintainer; the reporter is asked to resend on a
  private route. Re-routing does not count as publication and does
  not re-start the clock.
- **No chat-only reports.** A suspected security issue raised in a
  chat channel (community or maintainer) is asked to be resent on the
  private route. A maintainer who receives a chat-only report is
  responsible for creating a monitored-contact intake; reporters do
  not lose credit.
- **Private by default, public on advisory.** The monitored contact
  path is private. Public advisory publication follows the
  `public_on_advisory` disclosure class in
  [`artifacts/governance/issue_routing.yaml`](./artifacts/governance/issue_routing.yaml),
  not reporter-controlled timing, unless a coordinated-disclosure
  group has agreed otherwise in writing.
- **Evidence stays redacted.** Raw secret material, raw binary bytes,
  raw signing material, raw reporter identities, and raw exploit
  payloads never cross the boundary schemas
  ([`advisory_record.schema.json`](./schemas/security/advisory_record.schema.json),
  [`incident_workspace_packet.schema.json`](./schemas/security/incident_workspace_packet.schema.json),
  [`private_triage_workspace_packet.schema.json`](./schemas/security/private_triage_workspace_packet.schema.json)).
  Evidence items carry typed embedding state
  (`omitted` / `embedded` / `redacted` / `by_reference`) and a
  redaction-pass class inherited from ADR-0007.
- **Reporter recognition is opt-in.** A reporter who wishes to be
  credited in the public advisory MUST say so explicitly on the
  intake; absent consent, the advisory credits the report as
  coordinated intake with no reporter identity. Pseudonymous credit
  is supported.

## What to expect after reporting

- An acknowledgement from the monitored contact path within the
  severity-appropriate window above.
- A reserved Aureline advisory id (`AURELINE-ADV-YYYY-NNNN+`)
  communicated back on acknowledgement. CVE and GHSA aliases are
  minted as they become available and are carried as alias slots on
  the same advisory record, not as parallel ids.
- A private-triage workspace packet (see
  [`schemas/security/private_triage_workspace_packet.schema.json`](./schemas/security/private_triage_workspace_packet.schema.json))
  opened under the appropriate workspace scope
  (`private_security_channel`, `private_partner_channel`,
  `coordinated_disclosure_group`, or `vendor_only`).
- A continuity note describing what still works locally, under
  management, or offline if the affected install lane must be
  disabled before a fix is available, so admins on self-hosted,
  mirror-only, and air-gapped deployments can plan the interim.
- Fix or mitigation landing on the target above, with a public
  advisory, release note, and About / Help notice at the severity-
  appropriate level. Single-forum close on high, critical, or
  operational-emergency reports is not permitted; release-council
  co-sign is required.

## Companion artifacts

- [`docs/security/intake_and_triage.md`](./docs/security/intake_and_triage.md)
  — the intake-and-triage runbook this file pins to. Carries PGP
  key material, mailbox operator handles, and the step-by-step
  operational playbook the monitored contact path follows.
- [`docs/security/severity_matrix.md`](./docs/security/severity_matrix.md)
  — frozen severity vocabulary, advisory identity model, and
  monitored-contact-path obligations.
- [`docs/security/emergency_action_model.md`](./docs/security/emergency_action_model.md)
  — emergency-action, revocation, and continuity object model the
  triage packet can link into.
- [`schemas/security/advisory_record.schema.json`](./schemas/security/advisory_record.schema.json)
  — boundary schema for advisory publication.
- [`schemas/security/incident_workspace_packet.schema.json`](./schemas/security/incident_workspace_packet.schema.json)
  — boundary schema for the triage workspace's evidence-capture
  companion (timeline, evidence-embedding, continuity).
- [`schemas/security/private_triage_workspace_packet.schema.json`](./schemas/security/private_triage_workspace_packet.schema.json)
  — boundary schema for the private-triage workspace intake packet
  (intake time, affected build / artifact / deployment identity,
  mitigation status, break-glass state, disclosure posture, advisory-
  id aliases, compensating controls, linked postmortem refs,
  redaction posture).
- [`schemas/security/emergency_action_record.schema.json`](./schemas/security/emergency_action_record.schema.json)
  — boundary schema for emergency-action and revocation records.
- [`fixtures/security/triage_cases/`](./fixtures/security/triage_cases/)
  — worked intake-and-triage cases across hosted, self-hosted,
  mirror-only, and offline / air-gapped distributions.

## Policy change control

Changes to this file are security-trust / release-council authority.
Adding a new intake route, redefining an acknowledgement target,
changing the monitored mailbox, or narrowing the coordinated-
disclosure posture requires a decision-history row in
[`artifacts/governance/decision_index.yaml`](./artifacts/governance/decision_index.yaml).
Field-level wording edits that do not change routes, clocks, or
schema refs land as ordinary documentation changes.
