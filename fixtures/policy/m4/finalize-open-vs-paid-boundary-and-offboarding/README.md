# Finalize open-vs-paid boundary and offboarding fixtures

This directory contains machine-generated and hand-written fixtures for the
`aureline_policy::finalize_open_vs_paid_boundary_and_offboarding` lane.

## JSON fixtures (generated)

Run the example to regenerate:

```sh
cargo run -q -p aureline-policy --example dump_finalize_open_vs_paid_boundary_and_offboarding_fixtures -- page > page.json
cargo run -q -p aureline-policy --example dump_finalize_open_vs_paid_boundary_and_offboarding_fixtures -- rows > rows.json
cargo run -q -p aureline-policy --example dump_finalize_open_vs_paid_boundary_and_offboarding_fixtures -- defects > defects.json
cargo run -q -p aureline-policy --example dump_finalize_open_vs_paid_boundary_and_offboarding_fixtures -- summary > summary.json
cargo run -q -p aureline-policy --example dump_finalize_open_vs_paid_boundary_and_offboarding_fixtures -- support-export > support_export.json
cargo run -q -p aureline-policy --example dump_finalize_open_vs_paid_boundary_and_offboarding_fixtures -- usage-export-packets > usage_export_packets.json
```

## Drill fixtures

- `drill_local_core_managed_withdrawn.json` — local-core capability classified as managed (withdrawn)
- `drill_missing_offboarding_beta.json` — managed capability missing offboarding disclosure (beta)
- `drill_stale_export_schema_beta.json` — stale usage-export schema version (beta)

## YAML input fixtures

- `all_capabilities_stable_input.yaml` — clean stable input with all 15 capabilities
- `local_core_managed_withdrawn_input.yaml` — hard guardrail violation
- `missing_offboarding_beta_input.yaml` — missing offboarding disclosure
- `stale_export_schema_beta_input.yaml` — stale schema version
