# Continuity certification cases

This directory stores canonical JSON fixtures emitted by
`cargo run -q -p aureline-continuity --example dump_m5_continuity_certification_fixtures`.

Every file validates against
`schemas/continuity/continuity_certification_report.schema.json`
(`python3 tools/validate_m5_continuity_certification_fixtures.py`) and the
certification recompute / narrowing gate
(`python3 tools/check_m5_continuity_certification.py`).

The same report packet is checked in as the canonical certified-row registry
`artifacts/m5/continuity/certification/certified_rows.json` (a copy of
`page.json`), beside its support-export wrapper.

## Files

- `page.json` — seeded fully-certified report; every claimed managed,
  self-hosted, and sovereign row is certified at `stable` and nothing narrows
- `summary.json` — seeded report summary record
- `support_export.json` — support-export wrapper for the seeded report
- `case_backup_drill_stale_narrows.json` — a stale backup/restore/failover drill
  narrows the managed cloud row to `beta`
- `case_freshness_breached_narrows.json` — a breached continuity-proof freshness
  SLO narrows the managed relay row to `beta`
- `case_restore_identity_missing_narrows.json` — missing restore-identity /
  partial-loss disclosure narrows the self-hosted row to `preview`
- `case_mirror_offline_missing_narrows.json` — a missing mirror/offline
  continuity packet narrows the sovereign air-gapped row to `preview`
- `case_profile_mismatch_withdrawn.json` — locality contradicting the sovereign
  profile withdraws the claim
- `case_local_core_stays_certified.json` — a managed row narrows, but the
  local-core continuity lane stays certified and never narrows or withdraws

## Regeneration

```sh
DIR=fixtures/continuity/certification_cases
ART=artifacts/m5/continuity/certification
run() { cargo run -q -p aureline-continuity --example dump_m5_continuity_certification_fixtures -- "$1"; }
run page > $DIR/page.json
run summary > $DIR/summary.json
run support-export > $DIR/support_export.json
run case-backup-drill-stale-narrows > $DIR/case_backup_drill_stale_narrows.json
run case-restore-identity-missing-narrows > $DIR/case_restore_identity_missing_narrows.json
run case-freshness-breached-narrows > $DIR/case_freshness_breached_narrows.json
run case-mirror-offline-missing-narrows > $DIR/case_mirror_offline_missing_narrows.json
run case-profile-mismatch-withdrawn > $DIR/case_profile_mismatch_withdrawn.json
run case-local-core-stays-certified > $DIR/case_local_core_stays_certified.json
run page > $ART/certified_rows.json
run support-export > $ART/certification_support_export.json
```
