# Security intake-and-triage runbook

This document is the operational runbook the monitored SECURITY.md
contact path follows. `/SECURITY.md` pins the contract (routes,
acknowledgement targets, redaction expectations, severity references);
this document pins the day-to-day operation (who acknowledges, which
packet gets opened when, which linkage refs are populated where, and
when a packet hands off or exports).

Companion artifacts:

- [`/SECURITY.md`](../../SECURITY.md) — baseline security-contact and
  intake contract. This runbook is the operational companion.
- [`/docs/security/severity_matrix.md`](./severity_matrix.md) — severity
  vocabulary, advisory identity model, monitored-contact-path rules.
- [`/docs/security/emergency_action_model.md`](./emergency_action_model.md)
  — emergency-action, revocation, and continuity object model that
  triage packets link into.
- [`/schemas/security/private_triage_workspace_packet.schema.json`](../../schemas/security/private_triage_workspace_packet.schema.json)
  — boundary schema for the `private_triage_workspace_packet_record`
  the monitored contact path mints on intake and keeps updated through
  triage, mitigation, disclosure, and postmortem.
- [`/schemas/security/incident_workspace_packet.schema.json`](../../schemas/security/incident_workspace_packet.schema.json)
  — boundary schema for the evidence-capture companion (timeline,
  evidence-embedding, continuity).
- [`/schemas/security/advisory_record.schema.json`](../../schemas/security/advisory_record.schema.json)
  — boundary schema for advisory publication.
- [`/schemas/security/emergency_action_record.schema.json`](../../schemas/security/emergency_action_record.schema.json)
  — boundary schema for emergency-action and revocation records.
- [`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml)
  — `security_issue` row pinning private-security-channel,
  private-with-public-advisory privacy, public-on-advisory disclosure.
- [`/artifacts/governance/signing_quorum.yaml`](../../artifacts/governance/signing_quorum.yaml)
  — quorum and break-glass matrix for emergency, revocation, and
  kill-switch actions.
- [`/fixtures/security/triage_cases/`](../../fixtures/security/triage_cases/)
  — worked intake-and-triage cases across hosted, self-hosted,
  mirror-only, and offline / air-gapped distributions.

## Why publish this now

At milestone zero there is still no live on-call rota or paging
pipeline, but the repository already publishes signed artifacts in
principle, carries an advisory identity model, carries an
emergency-action / revocation object model, and carries an incident-
workspace-packet evidence-capture schema. Without an operational
intake-and-triage runbook, the monitored contact path would have no
canonical form, the first report would be absorbed ad hoc into chat
and email, and the private-triage workspace packet would be
hand-rolled. This document closes that gap by:

- naming the monitored contact routes and their owner;
- naming the packet that gets minted on intake, the vocabulary it
  carries, and the linkage rules it obeys;
- naming the handoff rules between the triage packet, the incident
  workspace packet, the advisory record, emergency-action /
  revocation records, channel-pause / freeze actions, support / export
  packets, and the postmortem record;
- naming the disclosure posture, compensating-control state, and
  break-glass state the triage packet MUST carry so a reviewer can
  reason about mitigation and public posture without re-deriving them;
- naming the redaction posture that keeps raw secret material, raw
  exploit payloads, and raw reporter identities out of every
  downstream packet.

This is a **pre-implementation** runbook. Live on-call staffing,
paging tooling, and emergency-action UI are named out of scope.

## Scope

In scope for this revision:

- The monitored intake routes (`security@aureline.dev` primary email,
  private GitHub advisory, coordinated-disclosure group, partner
  channel), their owner (`security_trust_review`), and their default
  acknowledgement posture.
- The `private_triage_workspace_packet_record` schema the monitored
  contact path mints on intake and updates through triage, mitigation,
  disclosure, and postmortem.
- Linkage rules between the private-triage packet, the incident
  workspace packet (evidence-capture companion), the advisory record,
  emergency-action / revocation records, channel pause / freeze
  actions, support / export redaction classes, and linked postmortem
  records.
- Disclosure posture, advisory-id alias reservation, compensating-
  control state, and break-glass state fields.
- Worked intake-and-triage cases across hosted, self-hosted,
  mirror-only, and offline / air-gapped distributions.

Out of scope until a superseding decision row opens:

- Live on-call rota assignments, paging infrastructure, and
  emergency-action UI behaviour beyond the object model.
- Public-advisory publication tooling. The advisory record is the
  boundary schema; the publication pipeline is a later lane.
- Coordinated-disclosure embargo scheduling with outside parties. The
  packet reserves the scope (`coordinated_disclosure_group`); party
  lists and embargo timelines are runbook content that does not belong
  in this schema.
- Automated triage or ML-based severity assignment. Severity is set by
  `security_trust_review` with co-sign per
  [`/docs/security/severity_matrix.md`](./severity_matrix.md).

## Monitored contact path

`security_md_monitored_contact` resolves into the routes below. Every
advisory record, incident workspace packet, and private-triage
workspace packet carries the opaque ref; no record is publishable
until the ref resolves.

### Route owner

- Owner lane: `security_trust_review`.
- Minimum co-sign on publication: `release_council` per severity
  matrix.
- Mailbox operator handle: `security-inbox-operator` (pseudonym; maps
  to an on-duty maintainer from `security_trust_review`).

### Routes

1. **Primary email: `security@aureline.dev`.**
   - PGP-signed encrypted mail accepted and preferred for raw
     evidence.
   - Published PGP key fingerprint is maintained as a signed file
     under `/docs/security/` (a later-lane companion; the fingerprint
     itself is not inlined here so this document does not drift when
     the key rotates).
   - The mailbox is monitored by `security-inbox-operator`.
   - Unmonitored drop-boxes, unattended shared mailboxes, and chat-only
     channels are non-conforming contact refs.

2. **Private GitHub advisory.**
   - "Report a vulnerability" on the Aureline project page.
   - Routes into the private security channel owned by
     `security_trust_review`.
   - A reporter who prefers this route does not lose priority; the
     acknowledgement clock starts when the advisory draft is opened
     and the route operator sees it.

3. **Coordinated-disclosure group intake.**
   - Used when the report is already under an upstream CNA, OSV, or
     vendor-response embargo.
   - Route packet is opened under `private_triage_workspace_scope:
     coordinated_disclosure_group`.
   - Embargo scheduling is runbook content; this runbook reserves the
     scope, not the schedule.

4. **Partner-channel intake.**
   - Used when the report is bound by a partner NDA or contractual
     channel.
   - Route packet is opened under `private_triage_workspace_scope:
     private_partner_channel`.
   - Aureline acknowledges and escalates through the partner's contact
     rather than overriding the partner agreement.

### Acknowledgement posture

Clocks start when the monitored contact path acknowledges the report
(`timeline_event.acknowledged_to_reporter` in the companion incident
workspace packet and `intake_acknowledged_at` on the private-triage
packet).

Severity-specific targets are frozen in
[`/docs/security/severity_matrix.md`](./severity_matrix.md) and
summarised in [`/SECURITY.md`](../../SECURITY.md).

An incident that never reaches the monitored contact path is a
governance failure logged against `security_trust_review`, not a reason
to stop the clock.

## The private-triage workspace packet

On intake, the monitored contact path mints one
`private_triage_workspace_packet_record` against the schema at
[`/schemas/security/private_triage_workspace_packet.schema.json`](../../schemas/security/private_triage_workspace_packet.schema.json).
The packet is the single canonical state for the triage as it moves
through acknowledgement, triage, mitigation, disclosure, and
postmortem.

### Identity

- `triage_packet_id` — opaque, stable id minted on intake. Stable for
  the life of the triage even when the packet is reclassified or
  split.
- `advisory_alias_reservation` — reserves `aureline_advisory_id`
  (required), `cve_id`, `ghsa_id`, and `additional_alias_refs` so the
  downstream advisory record and the triage packet share one identity
  graph. The reservation object is populated in place; CVE and GHSA
  alias slots are nullable until minted.

Where the repository already carries a seeded advisory record, the
packet points back at it through
`linked_advisory_refs.advisory_record_refs` and the advisory record
points back at the packet through `private_triage_workspace_ref`.

### Intake and mutation discipline

- `intake_received_at` — the timestamp the mailbox operator received
  the report. Set once; never mutated.
- `intake_acknowledged_at` — the timestamp the operator acknowledged
  the report back to the reporter. Set once; starts the severity
  clock.
- `triage_status` — evolves across the closed vocabulary
  (`triage_open` → `triage_in_progress` → one of the resolved /
  withdrawn / duplicate states). State transitions are decision-
  history events on the companion incident workspace packet, not silent
  mutations.
- `last_updated_at` — monotonic timestamp of the latest update.

### Affected-identity linkage

The packet MUST name identity in exactly the same vocabulary as the
downstream advisory record so the advisory can be published without
re-keying:

- `affected_exact_build_identity_refs` — opaque refs into
  [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json).
  Free-text "version 1.x" is non-conforming.
- `affected_artifact_refs` — opaque refs naming the artifact families
  observed (build binary, symbol bundle, docs pack, extension,
  install-profile card, signing material, runtime component,
  managed-cloud surface, third-party dependency, workspace-trust
  policy, AI-context-assembly).
- `affected_install_profile_card_refs` — opaque refs into
  [`/artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml).
- `impacted_deployment_profile_scope` — closed set drawn from the
  boundary-manifest deployment-profile vocabulary (`individual_local`,
  `self_hosted`, `enterprise_online`, `air_gapped`, `managed_cloud`).
- `affected_channel_classes` — closed set drawn from the exact-build
  identity model (`dev_local`, `nightly`, `preview`, `beta`, `stable`,
  `lts`, `hotfix`).
- `affected_publication_posture_classes` — closed set drawn from the
  install-topology plan (`online_vendor`, `offline_signed_bundle`,
  `customer_managed_mirror`, `third_party_package_index`).
- `mirror_freshness_class` — one of `up_to_date`, `stale_within_grace`,
  `stale_past_grace`, `offline_expired`, `unknown`.

This linkage is isomorphic to `affected_install_linkage` on the
advisory record. An advisory whose linkage disagrees with its triage
packet's is non-conforming at publication time.

### Current mitigation status

`current_mitigation_status` carries a closed vocabulary:

- `mitigation_not_yet_drafted` — triage is open; no mitigation has been
  proposed.
- `mitigation_drafted` — a mitigation has been drafted but not
  applied.
- `mitigation_applied_local_only` — the mitigation is applied in
  development or a staged lane but not shipped.
- `mitigation_shipped_partial` — the mitigation is shipped on a subset
  of channels or profiles; others remain affected.
- `mitigation_shipped_full` — the mitigation is shipped on every
  affected channel and profile.
- `mitigation_compensating_control_only` — no code fix at this
  revision; a compensating control (configuration change, capability
  narrowing, emergency action) is the live mitigation.
- `mitigation_not_required` — the finding is informational or
  duplicate; no mitigation is planned.

A packet at severity `security_severity.operational_emergency`,
`security_severity.critical`, or `security_severity.high` MUST NOT
linger at `mitigation_not_yet_drafted` beyond the severity-matrix
triage target; a packet that does is a decision-register observation
on `security_trust_review`.

### Compensating-control state

`compensating_control_state` names the state of any non-code
mitigation in place while a full fix is developed:

- `compensating_control_none` — no compensating control is in place.
- `compensating_control_drafted` — a compensating control is drafted
  but not applied.
- `compensating_control_applied` — a compensating control is applied
  on every affected channel and deployment profile.
- `compensating_control_partial` — a compensating control is applied
  on a subset of affected channels or profiles.
- `compensating_control_retired` — a compensating control was in place
  and has been retired because the full fix shipped.

`compensating_control_refs` carries opaque refs to the backing
artifact (settings change, capability gate narrowing, emergency-action
record, revocation record). Raw configuration bodies never appear on
this packet; the refs resolve through their owning schemas.

### Break-glass state

`break_glass_state` names whether any break-glass action has been
invoked during this triage and where the audit row is recorded:

- `break_glass_not_invoked` — no break-glass action has been invoked.
- `break_glass_invoked_pending_reconciliation` — a break-glass action
  was invoked (single-responder emergency path) and reconciliation
  with the full quorum has not yet landed.
- `break_glass_invoked_reconciled` — a break-glass action was invoked
  and the post-hoc quorum co-sign has landed on the decision trail.
- `break_glass_invoked_superseded` — the break-glass action has been
  superseded by a full-quorum emergency-action or revocation record.

`break_glass_refs` carries opaque refs to the audit row in
[`/artifacts/governance/signing_quorum.yaml`](../../artifacts/governance/signing_quorum.yaml)
and to the emergency-action / revocation record, so the break-glass
state is never implied and always inspectable.

### Disclosure posture

`disclosure_posture` carries a closed vocabulary that mirrors the
advisory record's `disclosure_class` so the triage packet and the
advisory resolve into the same posture:

- `disclosure_posture_private_pending_review` — the report is still
  under private review; no public posture has been decided.
- `disclosure_posture_public_on_fix` — the report will be disclosed
  publicly when the fix ships.
- `disclosure_posture_public_on_advisory` — the report will be
  disclosed publicly when the advisory is published (default for
  `security_issue`).
- `disclosure_posture_public_immediate` — the report is already public
  (for example, an already-published upstream CVE that affects
  Aureline).
- `disclosure_posture_private_indefinite` — the report is held private
  indefinitely; a decision-history row records the governance basis.

The packet MUST carry the disclosure posture explicitly; a packet that
elides it is non-conforming.

### Redaction posture

`redaction_posture` carries the same closed set as the advisory
record's `redaction_class` (`log_safe`, `support_export_only`,
`evidence_packet_only`, `release_public`, `private_triage_only`).
`private_triage_only` is admissible only when the packet's
`private_triage_workspace_scope` is set to one of
`private_security_channel`, `private_partner_channel`,
`coordinated_disclosure_group`, `vendor_only`.

A packet whose redaction posture conflicts with its export routing
fails closed; export is blocked.

### Related advisories

`linked_advisory_refs.advisory_record_refs` carries opaque refs into
[`/schemas/security/advisory_record.schema.json`](../../schemas/security/advisory_record.schema.json).
A triage that resolves to multiple advisories (split finding) names
every advisory it feeds. Empty until at least one advisory is drafted.

### Linked postmortem records

`linked_postmortem_refs` reserves an opaque-ref slot for postmortem
records produced after the incident closes. The postmortem schema is a
later lane; the packet pre-reserves the slot so follow-up ownership
stays attached to the same identity chain.

### Linkage to the incident workspace packet

Every private-triage packet points at exactly one
`incident_workspace_packet_record` through
`linked_incident_workspace_packet_ref`. The incident workspace packet
is the evidence-capture companion (timeline, evidence-embedding,
continuity notes, handoff / export routing); the private-triage packet
is the operational state (intake time, mitigation status, break-glass
state, disclosure posture, advisory-alias reservation, compensating-
control state).

The two packets MUST agree on:

- `severity_class` (no drift between operational state and evidence
  capture);
- `private_triage_workspace_scope`;
- `monitored_contact_ref`;
- the set of `affected_exact_build_identity_refs` and the incident
  packet's `artifact_refs.exact_build_identity_refs`;
- the set of `affected_install_profile_card_refs` and the incident
  packet's `artifact_refs.install_profile_card_refs`;
- the set of `impacted_deployment_profile_scope` and the incident
  packet's `deployment_profile_scope`.

A packet pair whose fields disagree is non-conforming.

## Linkage rules

The private-triage packet sits at the centre of a small graph of
linked records. The rules below bind them so an auditor can walk the
graph without re-keying incident identity.

### Triage packet ↔ advisory record

- The triage packet's `advisory_alias_reservation.aureline_advisory_id`
  MUST equal the downstream advisory's
  `advisory_identity.aureline_advisory_id`.
- The triage packet's `disclosure_posture` MUST project 1:1 onto the
  advisory's `disclosure_class`.
- The triage packet's `affected_exact_build_identity_refs`,
  `affected_install_profile_card_refs`, `affected_channel_classes`,
  `affected_publication_posture_classes`, and `mirror_freshness_class`
  MUST equal the advisory's `affected_install_linkage` fields of the
  same names.
- The advisory's `private_triage_workspace_ref` MUST resolve to the
  triage packet's `linked_incident_workspace_packet_ref` (the
  workspace is the packet's evidence-capture companion).

### Triage packet ↔ emergency-action / revocation records

- The triage packet's `compensating_control_refs` and
  `break_glass_refs` MAY reference `emergency_action_record` ids and
  `revocation_record` ids in
  [`/schemas/security/emergency_action_record.schema.json`](../../schemas/security/emergency_action_record.schema.json).
- The emergency-action or revocation record reuses the triage
  packet's `affected_exact_build_identity_refs` and
  `affected_install_profile_card_refs`; minting a parallel id family
  on the emergency-action record is non-conforming.

### Triage packet ↔ channel pause / freeze actions

- A channel pause / freeze action (emergency-action of kind
  `channel_freeze`, `update_pause`, `kill_switch`) MUST be cited from
  the triage packet through `break_glass_refs` or
  `compensating_control_refs` while the action is in force.
- The triage packet MUST NOT assert `mitigation_shipped_full` while a
  channel freeze or update pause is still in force; those states are
  mutually exclusive. A packet that makes both assertions is
  non-conforming.

### Triage packet ↔ support / export redaction

- The triage packet's `redaction_posture` MUST narrow (never widen)
  the redaction class inherited by any support-bundle or release-
  evidence export derived from the companion incident workspace
  packet.
- `export_routing` on the companion incident workspace packet
  (`no_export`, `export_embedded_in_advisory`,
  `export_by_reference_from_advisory`, `export_to_support_bundle`,
  `export_to_release_evidence_packet`) is the routing authority; the
  triage packet's `redaction_posture` is the redaction authority.
  `export_embedded_in_advisory` is admissible only when the triage
  packet's `redaction_posture` is `evidence_packet_only` or
  `private_triage_only`.

### Triage packet ↔ postmortem records

- `linked_postmortem_refs` is populated once the incident closes
  (`triage_resolved_to_advisory`, `triage_resolved_without_advisory`,
  `triage_withdrawn`, `triage_closed_duplicate`).
- The postmortem schema is a later lane; today's postmortem refs are
  opaque placeholders. The slot is reserved so postmortem ownership
  does not have to mint a parallel identity system.

## Operational walkthrough

A typical triage flows through the following stages. Every stage
records a decision-history event on the companion incident workspace
packet; the private-triage packet tracks operational state only.

1. **Report received.** `security-inbox-operator` observes the
   report on one of the monitored routes. `intake_received_at` set on
   the packet. Incident workspace packet timeline entry
   `report_received` added.
2. **Acknowledged to reporter.** Operator replies with an
   acknowledgement and a reserved `aureline_advisory_id`.
   `intake_acknowledged_at` set. Incident workspace packet timeline
   entry `acknowledged_to_reporter` added. Severity clock starts.
3. **Triage opened.** Packet's `triage_status` moves to
   `triage_in_progress`. Incident workspace packet timeline entry
   `triage_opened` added. Private-triage workspace scope pinned.
4. **Severity set.** Reviewer sets severity class per the severity
   matrix. Incident workspace packet timeline entry `severity_assigned`
   added. Decision-history row on the downstream advisory captures the
   forum co-sign.
5. **Reproduction.** Reviewer reproduces (or fails to reproduce)
   against the affected exact-build identity. Incident workspace packet
   timeline entries `reproduction_confirmed` or `reproduction_failed`
   added.
6. **Mitigation drafted / applied / shipped.** Packet's
   `current_mitigation_status` advances. Compensating-control state
   updated if a compensating control is applied before the code fix.
   Incident workspace packet timeline entries recorded.
7. **Emergency action invoked (if needed).** Break-glass state set
   and `break_glass_refs` / `compensating_control_refs` populated with
   the emergency-action or revocation id. Post-hoc quorum
   reconciliation lands before the packet can assert
   `break_glass_invoked_reconciled`.
8. **Advisory drafted and reviewed.**
   `advisory_alias_reservation.aureline_advisory_id` resolves to a
   concrete advisory draft. CVE and GHSA alias slots populated as
   assigned. Decision-history rows captured.
9. **Public advisory published.** `disclosure_posture` pinned (usually
   `disclosure_posture_public_on_advisory`). Incident workspace packet
   timeline entry `advisory_published` added.
10. **Postmortem.** `linked_postmortem_refs` populated. Packet's
    `triage_status` moves to `triage_resolved_to_advisory`.

Not every triage runs every stage. A duplicate closes at step 3; a
withdrawn report closes at any step with a reporter-originated
withdrawal event; a no-fix informational closes at
`triage_resolved_without_advisory`.

## Denial-reason vocabulary

The schema MUST reject packets that are operationally incoherent.
Reserved denial reasons (enforced by schema `allOf` gates and by
downstream validators):

- `unresolved_monitored_contact_ref` — `monitored_contact_ref` does
  not resolve to `security_md_monitored_contact` or to a named
  partner / coordinated-disclosure contact ref.
- `missing_advisory_alias_reservation` — no `aureline_advisory_id`
  has been reserved on a triage that has advanced past intake.
- `advisory_alias_identity_drift` — the triage packet's reserved
  `aureline_advisory_id` disagrees with the advisory record it
  points at.
- `mitigation_and_channel_freeze_conflict` —
  `mitigation_shipped_full` while a cited emergency action is still
  in force.
- `break_glass_without_audit_ref` — `break_glass_state` is not
  `break_glass_not_invoked` but `break_glass_refs` is empty.
- `compensating_control_state_without_ref` —
  `compensating_control_state` is not `compensating_control_none` but
  `compensating_control_refs` is empty.
- `disclosure_posture_unset_post_intake` — the triage has advanced
  past acknowledgement but `disclosure_posture` is unset.
- `disclosure_posture_advisory_mismatch` — the triage packet's
  `disclosure_posture` does not project onto the linked advisory's
  `disclosure_class`.
- `redaction_posture_widens_on_export` — the triage packet's
  `redaction_posture` is narrower than the class applied to a
  derived support-bundle or release-evidence export.
- `private_triage_only_outside_scope` — `redaction_posture:
  private_triage_only` set outside
  `private_security_channel` / `private_partner_channel` /
  `coordinated_disclosure_group` / `vendor_only`.
- `high_severity_without_exact_build_identity_refs` —
  `security_severity.operational_emergency`,
  `security_severity.critical`, or `security_severity.high` packet
  with empty `affected_exact_build_identity_refs`.
- `low_severity_with_emergency_hooks` — `security_severity.low`
  packet with non-empty `break_glass_refs`, `compensating_control_refs`
  resolving to an emergency-action or revocation record, or
  `linked_emergency_action_refs`.
- `deployment_profile_scope_empty_on_consequential_severity` —
  `security_severity.operational_emergency`,
  `security_severity.critical`, or `security_severity.high` packet
  with empty `impacted_deployment_profile_scope`.
- `incident_workspace_packet_field_drift` — any of the mirrored
  fields (`severity_class`, `private_triage_workspace_scope`,
  `monitored_contact_ref`, the refs listed in "Linkage to the incident
  workspace packet") disagrees with the companion incident workspace
  packet.
- `postmortem_ref_before_resolution` — `linked_postmortem_refs`
  non-empty while `triage_status` is still `triage_open` or
  `triage_in_progress`.
- `triage_status_unresolved` — `triage_status` not set to any value
  from the closed vocabulary.
- `coordinated_disclosure_handoff_without_scope` —
  coordinated-disclosure handoff intent recorded without
  `private_triage_workspace_scope: coordinated_disclosure_group`.
- `partner_handoff_without_scope` — partner-channel handoff intent
  recorded without `private_triage_workspace_scope:
  private_partner_channel`.

## Forbidden user-facing phrases

Drift scans MUST reject the following phrases on any surface derived
from the triage packet (release note, About / Help notice,
support-bundle export, release-evidence packet); the monitored contact
path is private by default and a surface that implies otherwise is a
policy drift, not a copy choice:

- "contact us on Slack"
- "DM a maintainer"
- "public issue tracker"
- "post on the forum"
- "tag us on social"
- "file publicly"
- "we'll handle it informally"

These phrases widen the contact path beyond the monitored routes and
are non-conforming.

## Change control

- Adding a new `triage_status` value, `current_mitigation_status`
  value, `compensating_control_state` value, `break_glass_state`
  value, `disclosure_posture` value, or denial reason is
  additive-minor and requires a schema version bump on
  `private_triage_workspace_packet_record`.
- Repurposing an existing vocabulary value is breaking and requires a
  new decision row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  co-signed by `security_trust_review` and `release_council`.
- Adding a new monitored intake route or redirecting an existing one
  requires a decision-history row; the change lands in
  [`/SECURITY.md`](../../SECURITY.md) and this runbook in the same
  change.

## Next-milestone expectations

- Seed the PGP key fingerprint file under `/docs/security/` and
  link it from `/SECURITY.md` and this runbook.
- Open a signing-quorum row specific to the
  `break_glass_invoked_pending_reconciliation` reconciliation clock
  once the release-cadence decision closes.
- Land the postmortem-record schema so
  `linked_postmortem_refs` stops being the only remaining reserved
  slot on the private-triage packet.
- Open the coordinated-disclosure runbook referenced by the
  `coordinated_disclosure_group` scope value. The packet reserves
  the scope; the party lists and embargo timelines are support /
  security authority.
