# Published Supportability Runbooks, Field Playbooks, and Incident/Advisory Packet Integration for the Stable Line

## Summary

This artifact documents the M4 stable-line publication of supportability runbooks,
field playbooks, and incident/advisory packet integration. The checked-in catalog,
schema, fixtures, and crate consumer establish a typed, truthful, narrow-blast-radius
contract for blocked-user recovery and supportable field operations.

## Schema

- [`schemas/support/publish_supportability_runbooks_field_playbooks_and_incident_advisory.schema.json`](../../schemas/support/publish_supportability_runbooks_field_playbooks_and_incident_advisory.schema.json)

## Crate consumer

- [`crates/aureline-support/src/publish_supportability_runbooks_field_playbooks_and_incident_advisory/mod.rs`](../../crates/aureline-support/src/publish_supportability_runbooks_field_playbooks_and_incident_advisory/mod.rs)

## Fixture corpus

- [`fixtures/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory/`](../../fixtures/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory/)

## Runbook source classes

| Source class | Authoritative posture | Export right | Notes |
|---|---|---|---|
| `repo_local` | `authoritative` | `metadata_safe_export` | Versioned runbook in workspace/repo; claims execution when current |
| `reviewed_docs_pack` | `authoritative` | `redacted_export` | Quarterly-reviewed docs pack with signer and freshness |
| `managed_catalog` | `managed_admin` | `redacted_export` | Enterprise catalog with IT-admin delegation |
| `browser_only_vendor_docs` | `reference_only` | `reference_only` | Browser-only vendor docs; never claims execution |

## Step envelope contract

Every step carries:
- Stable `step_id` and `ordinal`
- Closed `step_class`: `observe`, `verify`, `mitigate`, `rollback`, `communicate`
- `target_selector_scope` with explicit handoff metadata for external scopes
- `approval_requirement` with ticket ref for mutating steps
- `expected_evidence_outputs` with redaction state
- `deviation_note_required` flag for mutating steps

## Deviation notes

Durable [`DeviationNote`](../../crates/aureline-support/src/publish_supportability_runbooks_field_playbooks_and_incident_advisory/mod.rs) records survive export and investigation with:
- Actor ref, reason class, summary, evidence refs
- Departed step id and runbook packet id
- UTC creation timestamp

## Incident/advisory packets

[`IncidentAdvisoryPacket`](../../crates/aureline-support/src/publish_supportability_runbooks_field_playbooks_and_incident_advisory/mod.rs) joins:
- Incident header and environment scope
- Field-playbook packet ref
- Action ledger entry refs
- Deviation notes
- Console handoff refs
- Privacy baseline with `raw_private_material_excluded` and `ambient_authority_excluded`

## Fixtures covered

| Fixture | Source class | Key scenario |
|---|---|---|
| `repo_local_crash_recovery_runbook.yaml` | `repo_local` | Crash-loop recovery with safe-mode entry/exit |
| `reviewed_docs_pack_safe_mode_entry.yaml` | `reviewed_docs_pack` | Safe-mode entry across local and enterprise targets |
| `managed_catalog_enterprise_policy_repair.yaml` | `managed_catalog` | Enterprise policy repair with two-person approval |
| `browser_only_vendor_docs_handoff.yaml` | `browser_only_vendor_docs` | Explicit browser/console handoff with reference-only posture |
| `incident_workspace_with_deviation.yaml` | `repo_local` | Incident workspace with multiple durable deviation notes |
| `advisory_packet_known_limit.yaml` | `reviewed_docs_pack` | Advisory packet with overclaim blockers and rollback routes |
| `field_playbook_extension_bisect.yaml` | `repo_local` | Extension bisect with quarantine, restore, and export |

## Guardrails

- Project Doctor remains read-only by default.
- Safe-mode entry is bounded and previewable.
- Crash/support exports carry exact-build identity and are redacted-by-default.
- Browser-only vendor docs never claim execution authority.
- Mutating steps require explicit approval tickets or waiver.
- Deviation notes are first-class metadata, not log-only text.
