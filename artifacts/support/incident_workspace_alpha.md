# Incident Workspace Alpha Packet

This packet is the checked-in support artifact for the alpha incident
workspace/runbook lane. It points reviewers to the implementation,
fixture, and proof path that make incident triage inspectable without a
hosted support service.

## Canonical implementation

- Runtime crate: `crates/aureline-incident`
- Protected fixture: `fixtures/support/incident_workspace_alpha/provider_unavailable_missing_span.yaml`
- Existing contracts consumed:
  - `schemas/support/support_bundle_manifest.schema.json`
  - `schemas/support/runbook_packet.schema.json`
  - `schemas/support/runbook_step_result.schema.json`
  - `schemas/ops/incident_workspace.schema.json`
  - `schemas/ops/evidence_handoff_bundle.schema.json`
  - `docs/support/runbook_execution_contract.md`
  - `docs/ops/incident_workspace_contract.md`

The crate does not define a parallel support bundle shape. It projects
incident workspace rows through `aureline_support::bundle::SupportBundlePreviewBuilder`,
so exact-build identity, local-first redaction controls, omission
markers, and reopen-without-network behavior are inherited from the
support-bundle alpha contract.

## Packet shape

`IncidentWorkspacePacket` carries:

| Block | Purpose |
|---|---|
| `build_identity` | Exact-build refs copied from `aureline-support` capture |
| `provider_lane_state` | Whether hosted/provider lanes are available, degraded, unavailable, or not configured |
| `local_continuity_state` | Whether local diagnosis and export remain available |
| `evidence_attachments` | Logs, crash refs, task history, and support bundle refs, by reference or redacted summary |
| `missing_spans` | Explicit gaps for trace windows, symbolication reports, provider callbacks, and bundle manifest joins |
| `span_coverage` | Captured-versus-missing rollup that disables complete-coverage claims when required spans are absent |
| `runbook_packets` | Support runbook packet summaries with source freshness, rollback posture, policy refs, and exact-build refs |
| `support_bundle_links` | Reopenable local support preview and manifest refs |

## Acceptance proof

The protected test `crates/aureline-incident/tests/incident_workspace_alpha.rs`
demonstrates:

1. Local incident workspaces attach a redacted log slice, crash incident
   trail, task history, and support bundle id.
2. Missing symbolication and unavailable trace windows become typed
   `MissingSpan` rows, and `span_coverage.complete_coverage_claimed`
   remains `false`.
3. The support runbook fixture at
   `fixtures/support/runbook_cases/packet_live_mitigation_with_rollback.yaml`
   parses into an incident runbook summary with exact-build refs.
4. The redacted export preview carries support-bundle redaction controls,
   exact-build metadata, action reconstruction context, and an honesty
   marker for missing spans.
5. Provider unavailability does not block local review or local preview
   reopen.

## Review command

```sh
cargo test -p aureline-incident
```

## Boundaries

- Raw logs, command lines, provider payloads, terminal transcripts,
  approval-ticket bodies, stack bodies, and secret material do not cross
  the packet boundary.
- Hosted/provider unavailability is recorded as a state, not treated as
  a reason to discard local evidence.
- Mutating runbook execution is not implemented here. Runbook packets
  remain reviewable metadata and continue to depend on the current
  policy epoch, target context, approval ticket, and rollback posture.

