# Published Supportability Runbooks, Field Playbooks, and Incident/Advisory Packet Integration for the Stable Line

## Overview

This document defines the M4 stable-line contract for supportability runbooks,
field playbooks, and incident/advisory packet integration. It is the
reviewer-facing companion to the boundary schema and the crate consumer.

## Scope

- Runbook source classes with explicit authoritative posture
- Executable step envelopes with stable IDs and bounded authority
- Durable deviation notes for operator departures from declared sequences
- Field-playbook packets for stable-line operations
- Incident/advisory integration packets joinable to support bundles

## Source class model

Runbook sources are modeled as one of four classes, each with explicit
`authoritative_posture`, `signer_or_source_ref`, `docs_freshness_state`,
`approver_policy_summary`, and `export_right`:

1. **Repo-local / workspace-local** (`repo_local`)  
   Authoritative when current. Signed by CI/release signer. Exportable as
   metadata-safe reference.

2. **Reviewed docs pack** (`reviewed_docs_pack`)  
   Authoritative within its compatibility window. Signed by quarterly docs
   review. Exportable with redaction profile applied.

3. **Managed catalog** (`managed_catalog`)  
   Authority delegated to managed administrator. Signed by enterprise IT
   catalog signer. Exportable with redaction.

4. **Browser-only vendor documentation** (`browser_only_vendor_docs`)  
   Reference-only. Never claims execution authority. Exportable by reference
   only.

## Step envelope contract

Each executable step is an envelope with:

- `step_id`: opaque stable id
- `ordinal`: contiguous integer starting at 0
- `step_class`: one of `observe`, `verify`, `mitigate`, `rollback`, `communicate`
- `title` and `intent_summary`: reviewer-facing, redaction-safe
- `target_selector_scope`: one of `local_workspace`, `runtime_target`,
  `environment_scope`, `service_resource`, `browser_console_external`,
  `unresolved_requires_review`
- `target_identity_ref`: optional resolved target identity
- `approval_requirement`: one of `no_approval_required`,
  `runtime_approval_ticket`, `two_person_approval`, `policy_grant`,
  `break_glass`, `approval_forbidden`
- `approval_ticket_ref`: required for mutating steps unless waived
- `expected_evidence_outputs`: array of evidence output contracts with class,
  completion requirement, and redaction state
- `handoff_metadata`: required when target scope is external; carries
  `handoff_kind`, `target_console_uri_class`, `reason_class`, and authority flags
- `deviation_note_required`: true for mutating steps
- `rollback_step_ref`: optional rollback step linkage

## Deviation notes

Whenever an operator departs from the declared step sequence, a durable
`DeviationNote` is emitted with:

- `deviation_note_id`: stable id
- `actor_ref`: opaque actor identity
- `reason_class`: closed-vocabulary reason token
- `summary`: reviewer-facing explanation
- `evidence_refs`: supporting evidence references
- `departed_step_id`: the step that was departed from
- `runbook_packet_id`: the packet in which the deviation occurred
- `created_at`: UTC timestamp

Deviation notes are first-class metadata. They survive export, join to incident
packets, and are never reduced to log-only text.

## Field-playbook packet

A `FieldPlaybookPacket` collects:

- Stable `packet_id`, `runbook_id`, and `packet_version`
- `owner` block with owner, escalation owner, backup owner, and review cadence
- `source_document` block with source class, authoritative posture, freshness,
  signer, approver policy, and export right
- `compatibility_window` with version floor/ceiling, validity period, and expiry
  behavior
- `supported_target_scopes` array
- `steps` array of step envelopes
- `redaction_class`, `raw_private_material_excluded`, `ambient_authority_excluded`
- `authored_at` and `last_reviewed_at` timestamps

## Incident/advisory packet

An `IncidentAdvisoryPacket` joins a field-playbook packet to incident context:

- `incident_id` and optional `advisory_id`
- `field_playbook_ref`: stable reference to the playbook packet
- `action_ledger_entry_refs`: stable references to action ledger entries
- `deviation_notes`: durable deviation notes for this incident
- `console_handoff_refs`: references to console handoff metadata
- Privacy baseline flags

## Validation rules

The catalog validator enforces:

1. Schema version must be `1`.
2. Record kinds must match declared constants.
3. Raw private material and ambient authority must be excluded.
4. All four runbook source classes must be represented in the catalog.
5. Step ordinals must be contiguous starting from `0`.
6. Mutating steps (`mitigate`, `rollback`) must carry `approval_ticket_ref`
   or explicitly declare `no_approval_required` / `approval_forbidden`.
7. Steps with external target scopes must carry `handoff_metadata`.
8. `browser_only_vendor_docs` sources must have `reference_only` authoritative
   posture.
9. Incident/advisory packets must exclude raw private material and ambient
   authority.
10. Deviation notes must carry the correct `record_kind`.

## References

- Boundary schema: `schemas/support/publish_supportability_runbooks_field_playbooks_and_incident_advisory.schema.json`
- Crate consumer: `crates/aureline-support/src/publish_supportability_runbooks_field_playbooks_and_incident_advisory/mod.rs`
- Fixture corpus: `fixtures/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory/`
- Artifact: `artifacts/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory.md`
