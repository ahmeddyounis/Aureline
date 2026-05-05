# Signed postmortem and compensating-control contract

This document freezes the pre-implementation contract for **signed
postmortem records** and **compensating-control rows**.

The goal is to keep critical incidents from ending at mitigation by
anchoring one durable, machine-checkable post-incident artifact family
that records:

- incident identity and cross-artifact linkage (incident, advisory,
  disable bundle, emergency action, revocation, fixed release/build);
- timeline and blast radius (affected builds, install profiles, and
  deployment scope);
- mitigation path (temporary controls and shipped fixes); and
- named follow-up owners with deadlines.

Companion artifacts:

- [`/schemas/security/postmortem_record.schema.json`](../../schemas/security/postmortem_record.schema.json)
  - machine boundary for `postmortem_record`.
- [`/schemas/security/compensating_control_row.schema.json`](../../schemas/security/compensating_control_row.schema.json)
  - machine boundary for `compensating_control_row_record`.
- [`/fixtures/security/postmortem_cases/`](../../fixtures/security/postmortem_cases/)
  - worked examples covering disable-bundle mitigation, fixed-release
    mitigation, and long-lived compensating controls awaiting full
    remediation.

Normative source alignment:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` — high-severity
  incidents require signed advisories and postmortems that identify
  timeline, blast radius, affected versions, compensating controls, and
  follow-up owners.
- `.t2/docs/Aureline_Technical_Design_Document.md` §7.11.13 — disclosure
  and postmortem references must be linkable without replacing the
  mitigation truth.
- [`/docs/security/intake_and_triage.md`](./intake_and_triage.md) and
  [`/schemas/security/private_triage_workspace_packet.schema.json`](../../schemas/security/private_triage_workspace_packet.schema.json)
  - the private-triage packet owns the identity chain and reserves
    `linked_postmortem_refs` so follow-up ownership does not mint a
    parallel id system.
- [`/docs/ops/incident_workspace_contract.md`](../ops/incident_workspace_contract.md) and
  [`/schemas/ops/evidence_handoff_bundle.schema.json`](../../schemas/ops/evidence_handoff_bundle.schema.json)
  - immutable evidence-handoff bundles are the durable snapshot a
    postmortem reader uses for reconstruction without embedding raw
    evidence bodies.
- [`/docs/security/advisory_surface_contract.md`](./advisory_surface_contract.md)
  - disclosure links can reference `post_incident_review` by stable id.

If this contract disagrees with `.t2/docs/` sources, the `.t2/docs/`
source wins and this document, schemas, and fixtures update together.

## Scope

Frozen at this revision:

- `postmortem_record`
  - signed post-incident record with incident identity, linked advisory
    identities, timeline, blast radius, affected and fixed build refs,
    mitigation steps, compensating-control rows, and follow-up owner
    obligations.
- `compensating_control_row_record`
  - durable row describing one temporary control, its residual risk, its
    explicit review horizon, optional expiry, optional customer action,
    and explicit claim/capability narrowing while active.
- Visibility projection rules across three summary lanes:
  - **public** (`public_summary`)
  - **export-safe** (`export_safe_summary`)
  - **private** (`private_summary`)

Out of scope:

- writing public incident reports;
- staffing incident response operations;
- embedding raw evidence bodies, raw configuration bodies, raw secrets,
  raw URLs/hostnames, absolute paths, or raw signing material.

## Signed postmortem record

A `postmortem_record` is the signed object that freezes post-incident
truth as a first-class product artifact.

It carries:

- **Identity**: `postmortem_id`, `record_kind`, and schema version.
- **Incident identity chain**:
  - `source_incident_workspace_id_ref` (ops lane),
  - `linked_private_triage_packet_ref`,
  - `linked_incident_workspace_packet_ref`, and
  - `linked_evidence_handoff_bundle_refs[]` (immutable snapshot refs).
- **Linked security artifacts**:
  - `linked_advisory_record_refs[]` and `linked_advisory_identities[]`,
  - `linked_disable_bundle_refs[]`,
  - `linked_emergency_action_refs[]`, and
  - `linked_revocation_refs[]`.
- **Affected versions and blast radius**:
  - `affected_install_linkage` (install-profile cards, exact-build refs,
    channels, mirror posture),
  - `blast_radius` (affected builds + scope) with explicit
    `blast_radius_class`.
- **Timeline**: `timeline[]` entries (append-only, reviewable notes).
- **Mitigation path**: `mitigation_path[]` steps with typed supporting
  refs (fixed build identity, disable bundle id, emergency action id).
- **Compensating controls**: `compensating_control_rows[]` describing
  temporary controls that were applied while remediation progressed.
- **Follow-up obligations**: `follow_up_obligations[]` rows with
  machine-readable `owner_ref`, `owner_forum`, `due_at`, and `status`.
- **Visibility projections**:
  - `public_summary` for public surfaces when `visibility_class` is
    `public`,
  - `export_safe_summary` for support/admin/export surfaces, and
  - `private_summary` for private-triage readers only.
- **Signature metadata**: `record_signature_state` describing signing
  class and verification state without embedding raw signature bytes.

Non-negotiable rules:

1. **Durable linkage, no re-keying.** Postmortems MUST carry the stable
   incident id and link refs so a reader can walk from triage → advisory
   → emergency artifacts → fixed builds without inventing parallel ids.
2. **Append-only timeline and mitigation.** Corrections land as new
   timeline or mitigation-path rows (or a new postmortem record that
   supersedes the old one in a future lane). Prior entries are not
   rewritten in place.
3. **Export-safe summaries are explicit.** Support, admin packets, and
   release evidence MUST use `export_safe_summary` rather than scraping
   private notes.

## Compensating-control row record

A `compensating_control_row_record` is the durable object that prevents
temporary mitigations from turning into silent permanent “fixes”.

It MUST carry:

- **Control identity**: `control_row_id` plus schema version and kind.
- **Control type**: `control_type` describing which control family is in
  force (settings change, capability gate narrowing, disable bundle,
  emergency action, etc.).
- **Why it is temporary**: `temporary_basis_note`.
- **Residual risk**: `residual_risk_note`.
- **Review horizon**: `must_review_by` (non-null, always).
- **Optional expiry**: `expires_at` (required when `control_state` is
  `expired`).
- **Customer action if any**: `customer_action` with explicit
  `action_required` and typed note.
- **Narrowing truth**: at least one of:
  - `narrowed_capabilities[]`, or
  - `narrowed_claim_manifest_row_refs[]`.

Non-negotiable rules:

1. **No silent permanence.** Every active compensating control MUST
   carry a `must_review_by` horizon. Consumers MUST treat rows past
   their review horizon as requiring review; they MUST NOT be rendered
   as evergreen “fixed”.
2. **No fix-by-implication.** A compensating control MUST state residual
   risk and MUST name what is still narrowed. Surfaces MUST NOT imply
   full remediation from the existence of a control row.

## Visibility and summary rules

Postmortem records support three summary lanes:

- **Public**: `public_summary` is admissible only when
  `visibility_class` is `public`. This summary is safe for release notes
  and public advisory surfaces.
- **Export-safe**: `export_safe_summary` is always present and is the
  only summary admissible in support bundles, admin handoffs, and
  export previews.
- **Private**: `private_summary` is for private-triage readers only and
  MUST NOT be embedded verbatim into public exports.

Surfaces MUST NOT attempt to “upgrade” visibility by copying private
summary fields into public channels.

## Worked examples

Worked examples live in:
[`/fixtures/security/postmortem_cases/`](../../fixtures/security/postmortem_cases/).

They cover:

- a critical advisory mitigated via an emergency disable bundle;
- a high-severity incident resolved by a fixed release/build; and
- a long-lived compensating control awaiting full remediation with a
  review horizon that prevents silent permanence.

