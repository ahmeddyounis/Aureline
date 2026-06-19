# Continuity freshness-SLO stale-evidence cases

This directory stores canonical JSON fixtures emitted by
`cargo run -q -p aureline-continuity --example dump_m5_continuity_freshness_slo_fixtures`.

Every file validates against
`schemas/continuity/continuity_freshness_slo_dashboard.schema.json`
(`python3 tools/validate_m5_continuity_freshness_slo_fixtures.py`) and the
freshness recompute / shiproom gate
(`python3 tools/check_m5_continuity_freshness.py`).

The same dashboard packet is checked in as the canonical artifact
`artifacts/m5/continuity/freshness_slo_dashboard.json` (a copy of `page.json`).

## Files

- `page.json` — seeded clean dashboard; every claimed continuity row is within
  its freshness SLO, the promotion verdict is `proceed`, and nothing narrows
- `summary.json` — seeded dashboard summary record
- `support_export.json` — support-export wrapper for the seeded dashboard
- `case_managed_backup_breached_hold.json` — a managed backup packet breached its
  freshness SLO; the row narrows to beta and promotion holds
- `case_relay_packet_missing_hold.json` — a managed relay row has no captured
  continuity packet; the row narrows to preview and promotion holds
- `case_owner_signoff_missing_beta.json` — a self-hosted restore row lacks a
  current drill-owner sign-off; the row narrows and promotion holds
- `case_no_rerun_path_beta.json` — a sovereign snapshot row has no rerun path to
  refresh its evidence; the row narrows and promotion holds
- `case_local_core_stays_green.json` — a managed row goes stale (promotion holds),
  but the local-core continuity lane stays within SLO and never narrows or blocks

## Regeneration

```sh
DIR=fixtures/continuity/stale_evidence_cases
EX="cargo run -q -p aureline-continuity --example dump_m5_continuity_freshness_slo_fixtures --"
$EX page > $DIR/page.json
$EX summary > $DIR/summary.json
$EX support-export > $DIR/support_export.json
$EX case-managed-backup-breached-hold > $DIR/case_managed_backup_breached_hold.json
$EX case-relay-packet-missing-hold > $DIR/case_relay_packet_missing_hold.json
$EX case-owner-signoff-missing-beta > $DIR/case_owner_signoff_missing_beta.json
$EX case-no-rerun-path-beta > $DIR/case_no_rerun_path_beta.json
$EX case-local-core-stays-green > $DIR/case_local_core_stays_green.json
cp $DIR/page.json artifacts/m5/continuity/freshness_slo_dashboard.json
```

Or, equivalently, run the rerun tool which regenerates the dashboard from the
example and recomputes every packet's freshness state against the `as_of` clock:

```sh
python3 tools/continuity/run_drill_packets.py --regenerate
```
