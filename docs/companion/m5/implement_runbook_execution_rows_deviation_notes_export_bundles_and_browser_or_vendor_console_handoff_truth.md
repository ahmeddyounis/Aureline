# Runbook Execution Rows, Deviation Notes, Export Bundles, and Browser or Vendor-Console Handoff

This document is the human-readable contract for the runbook *execution* phase of
an incident: the per-step **execution rows** that record each runbook step as it
runs, the first-class **deviation notes** that record every departure from the
runbook, the **export bundles** that package an incident's transcript and evidence
for sharing, and the **browser or vendor-console handoff** that resumes an item in
an external surface. It builds on the static runbook packets defined by the incident
workspace surface. The machine-readable truth source is the checked-in support
export; later incident workspace, desktop companion panel, browser companion,
diagnostics, support export, and Help/About surfaces ingest it instead of cloning
status text.

- Record kind: `implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth`
- Schema: `schemas/companion/implement-runbook-execution-rows-deviation-notes-export-bundles-and-browser-or-vendor-console-handoff-truth.schema.json`
- Support export: `artifacts/companion/m5/implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth/support_export.json`
- Markdown summary: `artifacts/companion/m5/implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth.md`
- Fixtures: `fixtures/companion/m5/implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth/`
- Producer crate: `aureline-companion`

## Sections and matrix inheritance

The packet has four sections. Each one inherits its qualification and staged
rollout stage from the single frozen M5 companion-matrix `incident_workspace` lane
(see
`docs/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes.md`),
so the section never claims more than the matrix qualifies.

| Section | Matrix lane | Scope | Qualification | Rollout stage |
| --- | --- | --- | --- | --- |
| `execution_row` | `incident_workspace` | `read_only` | stable | general_availability |
| `deviation_note` | `incident_workspace` | `read_only` | stable | general_availability |
| `export_bundle` | `incident_workspace` | `read_only` | beta | staged_rollout |
| `external_handoff` | `incident_workspace` | `read_only` | preview | early_access |

## Read-mostly and host-authoritative

The surface is read-mostly: every section is read-only and the desktop host stays
authoritative for any change.

- **Execution rows** record each runbook step (`runbook_step_state`) and its
  `outcome` (`in_flight`, `succeeded`, `failed`, `deviated`, `rolled_back`). A row
  never applies an automated step from the surface: every row carries
  `requires_host_approval = true`, and an `automated_with_approval` step still
  relays each action to the host for explicit approval. There is no unbounded
  workspace write.
- **Deviation notes** record every departure from the runbook (`step_skipped`,
  `step_reordered`, `manual_override`, `parameter_changed`, `runbook_aborted`) with
  a `significance` of `minor`, `notable`, or `major`. A deviation is a first-class
  fact, never silently dropped.
- **Export bundles** package the incident for sharing (`incident_evidence_bundle`,
  `runbook_execution_transcript`, `deviation_log`, `full_incident_archive`). A
  bundle is `ready`, `building`, `partial`, or `failed`; a not-ready bundle is
  recorded as incomplete and labeled, never claimed complete. Every bundle is
  redaction-checked (`redaction_checked = true`).
- **External handoff** resumes an item in a browser or vendor console
  (`browser_companion_tab`, `vendor_incident_console`, `vendor_status_page`). It
  always discloses that it `requires_provider_continuity` and always keeps an exact
  local desktop handoff as the `local_fallback_available` path.

The `scope_contract` block asserts these guarantees for the whole packet, and the
validator rejects any section item carrying a write scope, any execution row that
does not require host approval, any export bundle that is not redaction-checked, and
any external handoff that lacks provider-continuity disclosure or a local fallback.

## Attribution: provable or honestly narrowed

Incident execution stays attributable. Execution rows and deviation notes carry an
`attribution` of `attributed`, `partially_attributed`, or `unattributed`. When
attribution to evidence and build identity is lost, the affected rows and notes
narrow to `unattributed` rather than claiming a provenance they can no longer prove.
The `attribution_contract` block asserts that execution rows and deviation notes are
attributed or narrowed, that every deviation is recorded as a first-class note, that
export bundles track the provenance of what they package, and that no provenance is
claimed without backing evidence.

## Stale-state honesty

Every item carries a `freshness` state — `live`, `cached`, `stale`, or `unknown`.
Stale-state honesty means a degraded item is never re-shown as live:

- Any item whose freshness is `stale` or `unknown` MUST set `stale_label_shown =
  true`; the validator rejects an unlabeled stale/unknown item.
- Any export bundle that is not `ready` MUST set `incomplete_label_shown = true`;
  the validator rejects an unlabeled incomplete bundle.
- The `stale_state_honesty` block asserts stale and unknown items are labeled, stale
  is never shown as live, and a freshness floor is enforced before an item is shown.

## Exact desktop handoff and local-first external handoff

Every item carries an exact `desktop_handoff` resolving to a precise host location
(here, the incident workspace). The handoff carries an opaque deep-link ref — never
a payload body — and records whether an active host session is required to resume it.

The external-handoff items additionally carry an `external` handoff into a browser
or vendor console. This is never the only path: the same item always keeps its exact
desktop handoff as the local-first fallback, so the local core never depends on the
external surface to function. The external handoff always sets
`requires_provider_continuity = true` and `local_fallback_available = true`.

## Downgrade-aware: narrows, never hides

`apply_runbook_execution_degradation` narrows from a per-observation signal, and
records the reasons in `degraded_labels` rather than hiding the section:

| Signal | Effect |
| --- | --- |
| Relay unavailable | Narrows every section one step; forces every live/cached item to `stale` and labels it (`relay_unavailable`, `freshness_downgraded_to_stale`) |
| Proof stale | Labels `proof_stale`; narrows every section one step |
| Upstream matrix lane narrowed | Labels `upstream_matrix_narrowed`; narrows every section one step |
| Host session inactive | Downgrades every host-dependent desktop handoff to `unresolved`; narrows the execution-row section, since an approved action can no longer relay (`host_session_inactive`, `handoff_target_unresolved`) |
| Trust narrowed | Narrows the execution-row and external-handoff sections (`trust_narrowed`) |
| Incident attribution lost | Marks every execution row and deviation note `unattributed` and narrows the execution-row and deviation-note sections (`incident_attribution_lost`) |
| Export incomplete | Narrows every ready bundle to `partial`, labels it, and narrows the export-bundle section (`export_bundle_incomplete`) |
| External unreachable | Marks every external handoff `unresolved` while the local desktop fallback stays exact, and narrows the external-handoff section (`external_handoff_unavailable`) |

Degradation narrows the claim; it never corrupts the packet, which still validates
after any single or combined observation.

## Locality

- **Stays local:** runbook execution rows, deviation notes, and the exact desktop
  handoff for every item are owned by the local core and stay inspectable offline.
- **Staged:** export-bundle building and browser/vendor-console handoff roll out per
  cohort and capability gate.
- **Requires provider/admin continuity:** resuming an item in a browser or vendor
  console, and relaying a host-approved execution action, require provider
  continuity and an active host session; the local core and its desktop handoff
  never depend on them to function.

## Boundary safety

The packet is export-safe metadata only. It carries redacted summaries and opaque
refs — never credential bodies, raw provider payloads, or raw execution, note,
bundle, or vendor-console bodies. The validator runs a forbidden-material heuristic
over the serialized export.

## Regenerating

The checked-in support export, Markdown summary, and fixtures are regenerated
deterministically from the first-consumer builder:

```text
cargo run -p aureline-companion --example dump_runbook_execution_surface -- canonical > artifacts/companion/m5/implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth/support_export.json
cargo run -p aureline-companion --example dump_runbook_execution_surface -- markdown  > artifacts/companion/m5/implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth.md
```
