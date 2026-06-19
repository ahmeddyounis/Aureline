# Backup/restore/failover continuity cases

This directory stores canonical JSON fixtures emitted by
`cargo run -q -p aureline-continuity --example dump_m5_backup_restore_failover_fixtures`.

Every file validates against
`schemas/continuity/backup_restore_failover_packet.schema.json`
(`python3 tools/validate_m5_backup_restore_failover_fixtures.py`).

## Files

- `page.json` — seeded stable page. It carries one packet for each managed
  continuity family (backup, failover, restore, snapshot/replication) across
  managed, self-hosted, and sovereign profiles, plus a local-core continuity
  packet. Every managed-family packet names a cadence, a current and future
  owner, typed exercised operations, a restore identity, and a partial-loss
  disclosure; the partially-exercised self-hosted restore packet discloses what
  it did not exercise. Every claimed resilience row points to a current packet.
- `summary.json` — seeded page summary record
- `registry.json` — seeded drill-packet registry record (per-claim-row coverage)
- `support_export.json` — support-export wrapper for the seeded page
- `case_generic_dr_text_withdrawn.json` — a packet rests on generic "DR tested"
  text; it fails closed and is withdrawn
- `case_sovereign_hidden_vendor_failover_withdrawn.json` — a sovereign packet
  hides a vendor-operated failover lane; it fails closed and is withdrawn
- `case_scope_not_exercised_preview.json` — a managed failover packet exercised
  nothing and is held at preview
- `case_drill_never_run_preview.json` — a managed backup drill has never been run
  and is held at preview
- `case_packet_evidence_missing_preview.json` — a claimed resilience row carries
  no packet and is held at preview
- `case_not_exercised_disclosure_missing_beta.json` — a partial drill omits what
  restored narrower than normal and narrows to beta
- `case_restore_identity_undeclared_beta.json` — a managed backup packet declares
  no restore identity and narrows to beta
- `case_drill_evidence_stale_beta.json` — a sovereign snapshot drill has aged out
  under its freshness SLO and narrows to beta

## Regeneration

```sh
DIR=fixtures/continuity/restore_identity_cases
EX="cargo run -q -p aureline-continuity --example dump_m5_backup_restore_failover_fixtures --"
$EX page > $DIR/page.json
$EX summary > $DIR/summary.json
$EX registry > $DIR/registry.json
$EX support-export > $DIR/support_export.json
$EX case-generic-dr-text-withdrawn > $DIR/case_generic_dr_text_withdrawn.json
$EX case-not-exercised-disclosure-missing-beta > $DIR/case_not_exercised_disclosure_missing_beta.json
$EX case-scope-not-exercised-preview > $DIR/case_scope_not_exercised_preview.json
$EX case-restore-identity-undeclared-beta > $DIR/case_restore_identity_undeclared_beta.json
$EX case-drill-never-run-preview > $DIR/case_drill_never_run_preview.json
$EX case-drill-evidence-stale-beta > $DIR/case_drill_evidence_stale_beta.json
$EX case-sovereign-hidden-vendor-failover-withdrawn > $DIR/case_sovereign_hidden_vendor_failover_withdrawn.json
$EX case-packet-evidence-missing-preview > $DIR/case_packet_evidence_missing_preview.json
```

The canonical evidence packets under `artifacts/m5/continuity/drill_packets/`
are regenerated from the same example (`page`, `registry`, and `support-export`).
