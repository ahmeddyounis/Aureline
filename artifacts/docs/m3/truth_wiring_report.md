# Beta truth wiring report

This report joins the current claim manifest to the current compatibility report for docs/help truth surfaces. It is read-only: degraded or stale proof narrows the surface state instead of widening copy.

## Inputs

- Claim manifest: `artifacts/release/m3/claim_manifest.json` (`claim_manifest:m3.beta`, rev 1, state `draft`)
- Compatibility report: `artifacts/compat/m3/compatibility_report.json` (`compat_report:m3.beta`, rev 1, state `draft`)
- As of: `2026-05-15`
- Generated at: `2026-05-15T20:39:50Z`

## Surface Bindings

| Surface | State | Claim rows | Compatibility rows | Missing compatibility rows | Freshness | Honesty |
|---|---|---:|---:|---|---|---|
| Docs browser | `degraded` | 4 | 6 | _(none)_ | `current` | present |
| Migration center | `degraded` | 1 | 3 | _(none)_ | `current` | present |
| Help/About | `degraded` | 10 | 11 | _(none)_ | `current` | present |
| Service health | `degraded` | 10 | 11 | _(none)_ | `current` | present |

## Surface Details

### Docs browser

- Surface ref: `crates/aureline-shell/src/docs_browser/`
- Required channel: `docs_site`
- Contract state: `degraded`
- Claim families: `docs_freshness`
- Claim rows: `m3_claim_row:canonical.docs.freshness_truth`, `m3_claim_row:beta_surface.debug_test_task_model`, `m3_claim_row:beta_surface.support_export_diagnostics`, `m3_claim_row:beta_surface.importer_and_migration`
- Compatibility rows: `compat_row:command_plane.command_descriptor_schema`, `compat_row:deployment_profiles.boundary_manifest_truth`, `compat_row:release_identity.exact_build_propagation`, `compat_row:remote.attach_envelope_and_drift`, `compat_row:state.profile_layout_schema`, `compat_row:tooling.task_event_envelope`

### Migration center

- Surface ref: `crates/aureline-shell/src/migration_center/mod.rs`
- Required channel: `migration_notes`
- Contract state: `degraded`
- Claim families: `docs_freshness`
- Claim rows: `m3_claim_row:beta_surface.importer_and_migration`
- Compatibility rows: `compat_row:deployment_profiles.boundary_manifest_truth`, `compat_row:release_identity.exact_build_propagation`, `compat_row:state.profile_layout_schema`

### Help/About

- Surface ref: `crates/aureline-shell/src/about/mod.rs`
- Required channel: `help_about`
- Contract state: `degraded`
- Claim families: `boundary_truth`, `docs_freshness`, `exact_build_identity`, `version_skew_truth`
- Claim rows: `m3_claim_row:canonical.boundary.open_local_vs_managed`, `m3_claim_row:canonical.build.exact_build_identity`, `m3_claim_row:canonical.docs.freshness_truth`, `m3_claim_row:beta_surface.extension_runtime`, `m3_claim_row:beta_surface.debug_test_task_model`, `m3_claim_row:beta_surface.packaging_update_rollback`, `m3_claim_row:beta_surface.policy_proxy_transport`, `m3_claim_row:beta_surface.support_export_diagnostics`, `m3_claim_row:beta_surface.importer_and_migration`, `m3_claim_row:beta_archetype.python_service_or_data_app`
- Compatibility rows: `compat_row:command_plane.command_descriptor_schema`, `compat_row:deployment_profiles.boundary_manifest_truth`, `compat_row:desktop.platform_conformance_profiles`, `compat_row:desktop_benchmark_lab.exact_build_identity`, `compat_row:extension_host.sdk_wit_permission_window`, `compat_row:launcher.local_helper_contracts`, `compat_row:provider.service_api_and_browser_handoff`, `compat_row:release_identity.exact_build_propagation`, `compat_row:remote.attach_envelope_and_drift`, `compat_row:state.profile_layout_schema`, `compat_row:tooling.task_event_envelope`

### Service health

- Surface ref: `crates/aureline-shell/src/service_health/mod.rs`
- Required channel: `service_health`
- Contract state: `degraded`
- Claim families: `boundary_truth`, `docs_freshness`, `exact_build_identity`, `version_skew_truth`
- Claim rows: `m3_claim_row:canonical.boundary.open_local_vs_managed`, `m3_claim_row:canonical.build.exact_build_identity`, `m3_claim_row:canonical.docs.freshness_truth`, `m3_claim_row:beta_surface.extension_runtime`, `m3_claim_row:beta_surface.debug_test_task_model`, `m3_claim_row:beta_surface.packaging_update_rollback`, `m3_claim_row:beta_surface.policy_proxy_transport`, `m3_claim_row:beta_surface.support_export_diagnostics`, `m3_claim_row:beta_surface.importer_and_migration`, `m3_claim_row:beta_archetype.python_service_or_data_app`
- Compatibility rows: `compat_row:command_plane.command_descriptor_schema`, `compat_row:deployment_profiles.boundary_manifest_truth`, `compat_row:desktop.platform_conformance_profiles`, `compat_row:desktop_benchmark_lab.exact_build_identity`, `compat_row:extension_host.sdk_wit_permission_window`, `compat_row:launcher.local_helper_contracts`, `compat_row:provider.service_api_and_browser_handoff`, `compat_row:release_identity.exact_build_propagation`, `compat_row:remote.attach_envelope_and_drift`, `compat_row:state.profile_layout_schema`, `compat_row:tooling.task_event_envelope`

## Community Handoff

| Issue | Route | Trust | Source | Context preserved | Template |
|---|---|---|---|---|---|
| `docs_truth_mismatch` | `public_issue_tracker` | `official_public` | `docs_browser` | yes | `issue-template:docs-truth-mismatch` |
| `migration_compatibility_regression` | `public_issue_tracker` | `official_public` | `migration_center` | yes | `issue-template:migration-compatibility-regression` |
| `design_proposal` | `public_rfc_forum` | `community` | `help_about` | yes | `issue-template:public-rfc-proposal` |
| `security_sensitive` | `private_security_channel` | `official_authenticated` | `help_about` | yes | `issue-template:private-security-intake` |
| `private_workspace_support` | `private_support_channel` | `official_authenticated` | `service_health` | yes | `issue-template:private-support-intake` |

## Findings

_All checked surface bindings resolve their claim rows and compatibility rows, and all seeded handoff routes preserve object and issue context._

## Refresh

Run `cargo test -p aureline-shell --lib docs_browser::truth_wiring` after refreshing the claim manifest or compatibility report. The checked-in report should be regenerated from `TruthWiringReport::render_markdown()` when either input changes.
