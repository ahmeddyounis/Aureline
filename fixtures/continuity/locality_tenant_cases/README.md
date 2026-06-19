# Locality-descriptor and tenant-card cases

This directory stores canonical JSON fixtures emitted by
`cargo run -q -p aureline-continuity --example dump_m5_locality_tenant_cards_fixtures`.

Every file validates against
`schemas/continuity/locality_descriptor.schema.json`
(`python3 tools/validate_m5_locality_tenant_cards_fixtures.py`).

## Files

- `page.json` — seeded stable locality/tenant card page
- `summary.json` — seeded page summary record
- `support_export.json` — support-export wrapper for the seeded page
- `case_region_pin_unhonored_withdrawn.json` — a managed row's declared region
  pin cannot be honored, so the managed lane fails closed and the claim is
  withdrawn
- `case_region_pin_undeclared_preview.json` — a managed row does not declare a
  region pin and is held at preview
- `case_self_hosted_locality_overclaimed_withdrawn.json` — a self-hosted row
  claims a broad vendor region it does not operate and the claim is withdrawn
- `case_retention_undisclosed_beta.json` — a managed relay row hides its
  retention class and narrows to beta
- `case_tenant_boundary_unverified_preview.json` — a managed relay row cannot
  verify its tenant boundary and is held at preview
- `case_surface_projection_incomplete_beta.json` — a managed row is not projected
  onto the support-export surface and narrows to beta

## Regeneration

```sh
DIR=fixtures/continuity/locality_tenant_cases
EX="cargo run -q -p aureline-continuity --example dump_m5_locality_tenant_cards_fixtures --"
$EX page > $DIR/page.json
$EX summary > $DIR/summary.json
$EX support-export > $DIR/support_export.json
$EX case-region-pin-unhonored-withdrawn > $DIR/case_region_pin_unhonored_withdrawn.json
$EX case-region-pin-undeclared-preview > $DIR/case_region_pin_undeclared_preview.json
$EX case-self-hosted-locality-overclaimed-withdrawn > $DIR/case_self_hosted_locality_overclaimed_withdrawn.json
$EX case-retention-undisclosed-beta > $DIR/case_retention_undisclosed_beta.json
$EX case-tenant-boundary-unverified-preview > $DIR/case_tenant_boundary_unverified_preview.json
$EX case-surface-projection-incomplete-beta > $DIR/case_surface_projection_incomplete_beta.json
```
