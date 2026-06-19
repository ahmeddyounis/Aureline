# Key-mode and storage-posture cases

This directory stores canonical JSON fixtures emitted by
`cargo run -q -p aureline-continuity --example dump_m5_key_mode_storage_posture_fixtures`.

Every file validates against
`schemas/continuity/key_mode_descriptor.schema.json`
(`python3 tools/validate_m5_key_mode_storage_posture_fixtures.py`).

## Files

- `page.json` — seeded stable key-mode/storage-posture page (exercises a real
  customer-managed-key lane and a real offline-trust-root lane)
- `summary.json` — seeded page summary record
- `support_export.json` — support-export wrapper for the seeded page
- `case_customer_key_unavailable_withdrawn.json` — a customer-managed key is
  unavailable, so the managed lane fails closed and the claim is withdrawn while
  local-core continuity is preserved
- `case_trust_root_mismatch_withdrawn.json` — the running offline trust root does
  not match the declared one, so the managed lane fails closed
- `case_key_material_lost_withdrawn.json` — durable key material is lost, so the
  managed lane fails closed
- `case_store_locked_preview.json` — the local store is locked on a managed-lane
  row and is held at preview as a typed degraded state
- `case_encryption_opaque_beta.json` — a row claims "encrypted" without naming
  the protecting key mode and narrows to beta
- `case_profile_key_mode_mismatch_preview.json` — a self-hosted row leans on
  vendor-managed keys and is held at preview

## Regeneration

```sh
DIR=fixtures/continuity/key_mode_failure_cases
EX="cargo run -q -p aureline-continuity --example dump_m5_key_mode_storage_posture_fixtures --"
$EX page > $DIR/page.json
$EX summary > $DIR/summary.json
$EX support-export > $DIR/support_export.json
$EX case-customer-key-unavailable-withdrawn > $DIR/case_customer_key_unavailable_withdrawn.json
$EX case-trust-root-mismatch-withdrawn > $DIR/case_trust_root_mismatch_withdrawn.json
$EX case-key-material-lost-withdrawn > $DIR/case_key_material_lost_withdrawn.json
$EX case-store-locked-preview > $DIR/case_store_locked_preview.json
$EX case-encryption-opaque-beta > $DIR/case_encryption_opaque_beta.json
$EX case-profile-key-mode-mismatch-preview > $DIR/case_profile_key_mode_mismatch_preview.json
```
