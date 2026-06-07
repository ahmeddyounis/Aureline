# Organization Admin Truth Fixtures

Generated from
`aureline_policy::stabilize_organization_admin_provisioning_and_seat_lifecycle_truth`.

## Files

- `page.json` — Seeded stable proof packet.
- `overview.json` — Organization or tenant overview card.
- `providers.json` — OIDC, SCIM, signed-file, and manual provider cards.
- `seats.json` — User or seat lifecycle rows.
- `impacts.json` — Seat transfer, suspension, downgrade, org switch, and
  deprovision impact previews.
- `rollout_rings.json` — Rollout-ring audit rows.
- `summary.json` — Aggregate qualification summary.
- `defects.json` — Empty defect list for the seeded page.
- `support_export.json` — Support/export projection.
- `drill_local_safety_withdrawn.json` — Withdrawal drill for missing local
  safety or export/offboarding guarantee.
- `drill_missing_provisioning_preview.json` — Preview drill for missing
  provisioning class coverage.
- `drill_generic_failure_beta.json` — Beta drill for generic failure kind copy.

## Regenerate

```sh
cargo run -q -p aureline-policy --example dump_organization_admin_truth_fixtures -- page > fixtures/enterprise/m4/stabilize-organization-admin-provisioning-and-seat-lifecycle-truth/page.json
cargo run -q -p aureline-policy --example dump_organization_admin_truth_fixtures -- support-export > fixtures/enterprise/m4/stabilize-organization-admin-provisioning-and-seat-lifecycle-truth/support_export.json
```
