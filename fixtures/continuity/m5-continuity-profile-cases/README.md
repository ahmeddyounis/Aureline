# Continuity-claim matrix profile cases

This directory stores canonical JSON fixtures emitted by
`cargo run -q -p aureline-continuity --example dump_m5_continuity_claim_matrix_fixtures`.

Every file validates against
`schemas/continuity/m5-continuity-claim-row.schema.json`
(`python3 tools/validate_m5_continuity_claim_matrix_fixtures.py`).

## Files

- `page.json` — seeded stable continuity-claim matrix packet
- `summary.json` — seeded matrix summary record
- `support_export.json` — support-export wrapper for the seeded packet
- `drill_managed_restore_drill_stale_beta.json` — a managed backup row's drill
  evidence is stale and the claim narrows to beta
- `drill_drill_never_run_preview.json` — a managed row's continuity drill was
  never run and the claim is held at preview
- `drill_sovereign_hidden_vendor_failover_withdrawn.json` — a sovereign row
  hides a vendor-operated failover lane and the claim is withdrawn
- `drill_locality_undisclosed_beta.json` — a managed relay row hides processing
  locality and narrows to beta
- `drill_local_only_overclaimed_preview.json` — a local-only row names a managed
  backup family without a managed dependency and is held at preview
- `drill_partial_loss_undisclosed_beta.json` — a self-hosted restore row hides
  partial-loss behavior and narrows to beta

## Regeneration

```sh
DIR=fixtures/continuity/m5-continuity-profile-cases
EX="cargo run -q -p aureline-continuity --example dump_m5_continuity_claim_matrix_fixtures --"
$EX page > $DIR/page.json
$EX summary > $DIR/summary.json
$EX support-export > $DIR/support_export.json
$EX drill-managed-restore-drill-stale-beta > $DIR/drill_managed_restore_drill_stale_beta.json
$EX drill-drill-never-run-preview > $DIR/drill_drill_never_run_preview.json
$EX drill-sovereign-hidden-vendor-failover-withdrawn > $DIR/drill_sovereign_hidden_vendor_failover_withdrawn.json
$EX drill-locality-undisclosed-beta > $DIR/drill_locality_undisclosed_beta.json
$EX drill-local-only-overclaimed-preview > $DIR/drill_local_only_overclaimed_preview.json
$EX drill-partial-loss-undisclosed-beta > $DIR/drill_partial_loss_undisclosed_beta.json
```
