# Incident Workspace Headers, Evidence Timelines, Resource Slices, and Runbook Packets

This document is the human-readable contract for the incident workspace content:
the **header** card that identifies an incident, the ordered **evidence timeline**
(including first-class missing spans), the read-only **resource slices** attributed
to the incident, and the **runbook packets** that guide mitigation. The
machine-readable truth source is the checked-in support export; later incident
workspace, desktop companion panel, diagnostics, support export, and Help/About
surfaces ingest it instead of cloning status text.

- Record kind: `add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets`
- Schema: `schemas/companion/add-incident-workspace-headers-evidence-timelines-resource-slices-and-runbook-packets.schema.json`
- Support export: `artifacts/companion/m5/add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets/support_export.json`
- Markdown summary: `artifacts/companion/m5/add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets.md`
- Fixtures: `fixtures/companion/m5/add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets/`
- Producer crate: `aureline-companion`

## Sections and matrix inheritance

The packet has four sections. Each one inherits its qualification and staged
rollout stage from the single frozen M5 companion-matrix `incident_workspace` lane
(see
`docs/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes.md`),
so the section never claims more than the matrix qualifies.

| Section | Matrix lane | Scope | Qualification | Rollout stage |
| --- | --- | --- | --- | --- |
| `header` | `incident_workspace` | `read_only` | stable | general_availability |
| `evidence_timeline` | `incident_workspace` | `read_only` | stable | general_availability |
| `resource_slice` | `incident_workspace` | `read_only` | beta | staged_rollout |
| `runbook_packet` | `incident_workspace` | `read_only` | preview | early_access |

## Read-mostly and host-authoritative

The incident workspace is read-mostly: every section is read-only and the desktop
host stays authoritative for any change.

- **Header** identifies the incident — severity, lifecycle status, attribution,
  build identity, and originating evidence — and hands off into the workspace.
- **Evidence timeline** is an ordered list of evidence spans (`crash_trail`,
  `log_window`, `metric_series`, `user_report`, `diagnostic_bundle`,
  `build_artifact`). A span is `present`, `partial`, or `missing`; a missing or
  partial span is recorded as a first-class gap and labeled, never silently
  dropped.
- **Resource slices** are bounded read-only views of resource state attributed to
  the incident (`cpu_profile`, `memory_snapshot`, `log_slice`, `thread_dump`,
  `network_trace`, `disk_io`). Each slice declares the bounded window it captured.
- **Runbook packets** guide mitigation. A runbook never executes an automated step
  from the workspace: every packet carries `requires_host_approval = true`, and an
  `automated_with_approval` packet still relays each action to the host for
  explicit approval. There is no unbounded workspace write.

The `scope_contract` block asserts these guarantees for the whole packet, and the
validator rejects any section item carrying a write scope and any runbook item that
does not require host approval.

## Attribution: provable or honestly narrowed

Incident packets stay attributable. Headers and evidence spans carry an
`attribution` of `attributed`, `partially_attributed`, or `unattributed`. When
attribution to evidence and build identity is lost, the affected headers and
evidence spans narrow to `unattributed` rather than claiming a provenance they can
no longer prove. The `attribution_contract` block asserts that headers and evidence
spans are attributed or narrowed, that missing spans are recorded as first-class
facts, and that no provenance is claimed without backing evidence.

## Stale-state honesty

Every item carries a `freshness` state — `live`, `cached`, `stale`, or `unknown`.
Stale-state honesty means a degraded item is never re-shown as live:

- Any item whose freshness is `stale` or `unknown` MUST set `stale_label_shown =
  true`; the validator rejects an unlabeled stale/unknown item.
- Any evidence span that is `partial` or `missing` MUST set `gap_label_shown =
  true`; the validator rejects an unlabeled gap.
- The `stale_state_honesty` block asserts stale and unknown items are labeled,
  stale is never shown as live, and a freshness floor is enforced before an item is
  shown.

## Exact desktop handoff

Every item carries an exact [`desktop_handoff`] resolving to a precise host
location (here, the incident workspace). The handoff carries an opaque deep-link
ref — never a payload body — and records whether an active host session is required
to resume it.

## Downgrade-aware: narrows, never hides

`apply_incident_workspace_degradation` narrows from a per-observation signal, and
records the reasons in `degraded_labels` rather than hiding the section:

| Signal | Effect |
| --- | --- |
| Relay unavailable | Narrows every section one step; forces every live/cached item to `stale` and labels it (`relay_unavailable`, `freshness_downgraded_to_stale`) |
| Proof stale | Labels `proof_stale`; narrows every section one step |
| Upstream matrix lane narrowed | Labels `upstream_matrix_narrowed`; narrows every section one step |
| Host session inactive | Downgrades every host-dependent handoff to `unresolved`; narrows the runbook section, since an approved action can no longer relay (`host_session_inactive`, `handoff_target_unresolved`) |
| Trust narrowed | Narrows the runbook section (`trust_narrowed`) |
| Incident attribution lost | Marks every header and evidence span `unattributed` and narrows the header and evidence-timeline sections (`incident_attribution_lost`) |
| Evidence incomplete | Narrows every present evidence span to `partial`, labels the gap, and narrows the evidence-timeline section (`evidence_incomplete`) |

Degradation narrows the claim; it never corrupts the packet, which still validates
after any single or combined observation.

## Locality

- **Stays local:** incident headers, evidence spans, resource slices, and runbook
  packets are owned by the local core and stay inspectable offline.
- **Staged:** resource-slice capture and runbook automation roll out per cohort and
  capability gate.
- **Requires provider/admin continuity:** exact handoff into a live host, and
  relaying a host-approved runbook action, require the companion relay and an active
  host session; the local core never depends on them to function.

## Boundary safety

The packet is export-safe metadata only. It carries redacted summaries and opaque
refs — never credential bodies, raw provider payloads, or raw incident, evidence,
resource, or runbook bodies. The validator runs a forbidden-material heuristic over
the serialized export.

## Regenerating

The checked-in support export, Markdown summary, and fixtures are regenerated
deterministically from the first-consumer builder:

```text
cargo run -p aureline-companion --example dump_incident_workspace_surface -- canonical > artifacts/companion/m5/add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets/support_export.json
cargo run -p aureline-companion --example dump_incident_workspace_surface -- markdown  > artifacts/companion/m5/add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets.md
```
