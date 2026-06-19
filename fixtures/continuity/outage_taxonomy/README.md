# Control-plane-versus-data-plane outage cases

This directory stores canonical JSON fixtures emitted by
`cargo run -q -p aureline-continuity --example dump_m5_control_plane_vs_data_plane_outage_fixtures`.

Every file validates against
`schemas/continuity/control_vs_data_plane_packet.schema.json`
(`python3 tools/validate_m5_control_plane_vs_data_plane_outage_fixtures.py`).

## Files

- `page.json` — seeded stable outage-taxonomy page. It exercises one simulated
  impairment for every optional-service family (identity/policy,
  registry/updates/docs, collaboration, remote control plane, AI gateway, and
  telemetry/support) across a mix of control and data planes and a mix of
  degraded, unavailable, and recovering severities. Every packet preserves full
  local-core continuity and none flips a global "IDE down" state.
- `summary.json` — seeded page summary record
- `support_export.json` — support-export wrapper for the seeded page
- `case_ide_down_conflation_withdrawn.json` — a collaboration outage flips a
  global "IDE down" state while local-core work is still safe; the packet fails
  closed and is withdrawn
- `case_local_editing_conflated_withdrawn.json` — a remote control-plane outage
  marks local editing and save unavailable, conflating a managed-lane outage with
  a local editing failure; the packet fails closed and is withdrawn
- `case_fallback_undeclared_beta.json` — an impaired AI gateway names no narrower
  fallback and narrows to beta
- `case_operational_inconsistent_preview.json` — an operational lane still claims
  an active fallback and is held at preview
- `case_outage_evidence_stale_preview.json` — a registry/updates/docs outage
  references stale evidence and is held at preview
- `case_family_coverage_incomplete_beta.json` — the telemetry/support family is
  missing from the taxonomy and the page narrows to beta

## Regeneration

```sh
DIR=fixtures/continuity/outage_taxonomy
EX="cargo run -q -p aureline-continuity --example dump_m5_control_plane_vs_data_plane_outage_fixtures --"
$EX page > $DIR/page.json
$EX summary > $DIR/summary.json
$EX support-export > $DIR/support_export.json
$EX case-ide-down-conflation-withdrawn > $DIR/case_ide_down_conflation_withdrawn.json
$EX case-local-editing-conflated-withdrawn > $DIR/case_local_editing_conflated_withdrawn.json
$EX case-fallback-undeclared-beta > $DIR/case_fallback_undeclared_beta.json
$EX case-operational-inconsistent-preview > $DIR/case_operational_inconsistent_preview.json
$EX case-outage-evidence-stale-preview > $DIR/case_outage_evidence_stale_preview.json
$EX case-family-coverage-incomplete-beta > $DIR/case_family_coverage_incomplete_beta.json
```
