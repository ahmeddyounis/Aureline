# Beta Reference-Workspace Report

This report is generated from the beta reference-workspace register, workspace packets, and current harness result rows. It is the source for archetype support classes, Help/About and service-health badges, release evidence, partner packets, and support exports.

## Metadata

- Report id: `reference_workspace_report:m3.beta`
- Release channel scope: `beta`
- As of: `2026-05-17`
- Generated at: `2026-05-17T19:20:17Z`
- Owner: @ahmeddyounis
- Publication gate: `blocked_widening`

## Source Artifacts

- `reference_workspace_register`: `artifacts/compat/m3/reference_workspace_register.yaml`
- `reference_workspace_rows`: `artifacts/compat/reference_workspace_rows.yaml`
- `claimed_surface_register`: `artifacts/milestones/m3/claimed_surface_register.json`
- `claim_manifest`: `artifacts/release/m3/claim_manifest.json`

## Summary

| Workspace | Reference row | Reference id | Support | Freshness | Results |
|---|---|---|---|---|---|
| C / C++ native project | `m3_reference_workspace:cpp_native` | `refws.c_cpp_native_archetype_seed` | Retest pending (`experimental` -> `retest_pending`) | `retest_pending` | pass=0, fail=0, blocked=0, not_run=6 |
| Go service or monorepo slice | `m3_reference_workspace:go_service` | `refws.go_service_archetype_seed` | Retest pending (`experimental` -> `retest_pending`) | `retest_pending` | pass=0, fail=0, blocked=0, not_run=6 |
| Java / Kotlin service | `m3_reference_workspace:jvm_service` | `refws.java_kotlin_service_archetype_seed` | Retest pending (`experimental` -> `retest_pending`) | `retest_pending` | pass=0, fail=0, blocked=0, not_run=6 |
| Python service or data app | `unmaterialized_reference_workspace:python_service_or_data_app` | `refws.python_data_app_archetype_seed` | Retest pending (`experimental` -> `retest_pending`) | `retest_pending` | pass=0, fail=0, blocked=0, not_run=0 |
| Rust workspace | `m3_reference_workspace:rust_workspace` | `refws.small_rust_self_host_slice` | Retest pending (`supported` -> `retest_pending`) | `retest_pending` | pass=0, fail=0, blocked=0, not_run=6 |
| TypeScript / JavaScript web app or service | `unmaterialized_reference_workspace:ts_web_app_or_service` | `refws.ts_web_app_archetype_seed` | Retest pending (`experimental` -> `retest_pending`) | `retest_pending` | pass=0, fail=0, blocked=0, not_run=0 |

## Workspace Rows

### C / C++ native project

- Report row: `reference_workspace_report_row:cpp_native`
- Archetype row: `archetype_row:c_or_cpp_native_project`
- Beta archetype: `beta_archetype:c_or_cpp_native_project`
- Reference workspace id: `refws.c_cpp_native_archetype_seed`
- Workspace descriptor: `fixtures/workspaces/reference/c_cpp_native_archetype_seed.json`
- Workspace packet: `fixtures/reference_workspaces/m3/cpp_native/workspace.yaml`
- Harness: `fixtures/reference_workspaces/m3/cpp_native/harness.yaml`
- Toolchain pins: `cmake 3.29.6`, `ninja 1.12.1`, `clangd 18.1.8`, `lldb 18.1.8`, `gdb 14.2`
- Platform coverage: `macos_arm64`, `macos_x86_64`, `linux_x86_64`, `windows_x86_64`
- Mode scope: `local_only`
- Support class: declared `experimental`, effective `retest_pending`
- Downgrade reasons: `not_run_workflow_results`
- Freshness: `retest_pending`, expires `2026-06-07`, stale after `2026-06-28`

| Workflow | Class | Latest result | Runner | Evidence outputs |
|---|---|---|---|---|
| `workflow.beta.cpp_native.first_open_configure` | `benchmark` | `not_run` | `benchmark_lab` | `benchmark_open_trace`, `compile_database_snapshot` |
| `workflow.beta.cpp_native.build_run` | `run` | `not_run` | `cmake_ninja_runner` | `task_event_envelope` |
| `workflow.beta.cpp_native.test` | `test` | `not_run` | `ctest_runner` | `test_tree_row`, `task_event_envelope` |
| `workflow.beta.cpp_native.debug` | `debug` | `not_run` | `lldb_or_gdb_adapter` | `debug_session_trace`, `symbol_resolution_row` |
| `workflow.beta.cpp_native.migration_probe` | `migration` | `not_run` | `migration_scorecard_probe` | `migration_compatibility_scorecard_row` |
| `workflow.beta.cpp_native.support_export` | `supportability` | `not_run` | `support_export_projection` | `redacted_support_export_row` |

### Go service or monorepo slice

- Report row: `reference_workspace_report_row:go_service`
- Archetype row: `archetype_row:go_service_or_monorepo_slice`
- Beta archetype: `beta_archetype:go_service_or_monorepo_slice`
- Reference workspace id: `refws.go_service_archetype_seed`
- Workspace descriptor: `fixtures/workspaces/reference/go_service_archetype_seed.json`
- Workspace packet: `fixtures/reference_workspaces/m3/go_service/workspace.yaml`
- Harness: `fixtures/reference_workspaces/m3/go_service/harness.yaml`
- Toolchain pins: `go 1.22.5`, `delve 1.23.1`
- Platform coverage: `macos_arm64`, `macos_x86_64`, `linux_x86_64`, `windows_x86_64`
- Mode scope: `local_only`
- Support class: declared `experimental`, effective `retest_pending`
- Downgrade reasons: `not_run_workflow_results`
- Freshness: `retest_pending`, expires `2026-06-07`, stale after `2026-06-28`

| Workflow | Class | Latest result | Runner | Evidence outputs |
|---|---|---|---|---|
| `workflow.beta.go_service.first_open_module_graph` | `benchmark` | `not_run` | `benchmark_lab` | `benchmark_open_trace`, `module_graph_snapshot` |
| `workflow.beta.go_service.run` | `run` | `not_run` | `go_runner` | `task_event_envelope` |
| `workflow.beta.go_service.test` | `test` | `not_run` | `go_test_runner` | `test_tree_row`, `task_event_envelope` |
| `workflow.beta.go_service.debug` | `debug` | `not_run` | `delve_debug_adapter` | `debug_session_trace_or_capability_loss_row` |
| `workflow.beta.go_service.migration_probe` | `migration` | `not_run` | `migration_scorecard_probe` | `migration_compatibility_scorecard_row` |
| `workflow.beta.go_service.support_export` | `supportability` | `not_run` | `support_export_projection` | `redacted_support_export_row` |

### Java / Kotlin service

- Report row: `reference_workspace_report_row:jvm_service`
- Archetype row: `archetype_row:java_or_kotlin_service`
- Beta archetype: `beta_archetype:java_or_kotlin_service`
- Reference workspace id: `refws.java_kotlin_service_archetype_seed`
- Workspace descriptor: `fixtures/workspaces/reference/java_kotlin_service_archetype_seed.json`
- Workspace packet: `fixtures/reference_workspaces/m3/jvm_service/workspace.yaml`
- Harness: `fixtures/reference_workspaces/m3/jvm_service/harness.yaml`
- Toolchain pins: `jdk 21.0.7`, `gradle 8.10.2`, `kotlin 2.0.21`, `junit_jupiter 5.10.3`
- Platform coverage: `macos_arm64`, `macos_x86_64`, `linux_x86_64`, `windows_x86_64`
- Mode scope: `local_only`
- Support class: declared `experimental`, effective `retest_pending`
- Downgrade reasons: `not_run_workflow_results`
- Freshness: `retest_pending`, expires `2026-06-07`, stale after `2026-06-28`

| Workflow | Class | Latest result | Runner | Evidence outputs |
|---|---|---|---|---|
| `workflow.beta.jvm_service.first_open_index` | `benchmark` | `not_run` | `benchmark_lab` | `benchmark_open_trace`, `target_graph_snapshot`, `support_export_summary_row` |
| `workflow.beta.jvm_service.run` | `run` | `not_run` | `bsp_or_gradle_runner` | `task_event_envelope` |
| `workflow.beta.jvm_service.test` | `test` | `not_run` | `junit_structured_ingest` | `test_tree_row`, `task_event_envelope` |
| `workflow.beta.jvm_service.debug` | `debug` | `not_run` | `debug_adapter_or_external_handoff` | `debug_session_trace_or_handoff` |
| `workflow.beta.jvm_service.migration_probe` | `migration` | `not_run` | `migration_scorecard_probe` | `migration_compatibility_scorecard_row` |
| `workflow.beta.jvm_service.support_export` | `supportability` | `not_run` | `support_export_projection` | `redacted_support_export_row` |

### Python service or data app

- Report row: `reference_workspace_report_row:python_service_or_data_app`
- Archetype row: `archetype_row:python_service_or_data_app`
- Beta archetype: `beta_archetype:python_service_or_data_app`
- Reference workspace id: `refws.python_data_app_archetype_seed`
- Workspace descriptor: `fixtures/workspaces/reference/python_data_app_archetype_seed.json`
- Platform coverage: `macos_arm64`, `macos_x86_64`, `linux_x86_64`, `windows_x86_64`
- Mode scope: `local_only`, `local_plus_devcontainer_or_container`
- Support class: declared `experimental`, effective `retest_pending`
- Downgrade reasons: `missing_workspace_packet`, `missing_beta_harness`, `no_current_workflow_results`
- Freshness: `retest_pending`, expires `2026-06-07`, stale after `2026-06-28`
- Workflow results: no current beta harness row is materialized.

### Rust workspace

- Report row: `reference_workspace_report_row:rust_workspace`
- Archetype row: `archetype_row:rust_workspace`
- Beta archetype: `beta_archetype:rust_workspace`
- Reference workspace id: `refws.small_rust_self_host_slice`
- Workspace descriptor: `fixtures/workspaces/reference/small_rust_self_host_slice.json`
- Workspace packet: `fixtures/reference_workspaces/m3/rust_workspace/workspace.yaml`
- Harness: `fixtures/reference_workspaces/m3/rust_workspace/harness.yaml`
- Toolchain pins: `rustc 1.84.0`, `cargo 1.84.0`, `rustfmt 1.84.0`, `clippy 1.84.0`
- Platform coverage: `macos_arm64`, `macos_x86_64`, `linux_x86_64`, `windows_x86_64`
- Mode scope: `local_only`
- Support class: declared `supported`, effective `retest_pending`
- Downgrade reasons: `not_run_workflow_results`
- Freshness: `retest_pending`, expires `2026-06-07`, stale after `2026-06-28`

| Workflow | Class | Latest result | Runner | Evidence outputs |
|---|---|---|---|---|
| `workflow.beta.rust_workspace.first_open_metadata` | `benchmark` | `not_run` | `benchmark_lab` | `benchmark_open_trace`, `target_graph_snapshot` |
| `workflow.beta.rust_workspace.build` | `run` | `not_run` | `cargo_runner` | `task_event_envelope` |
| `workflow.beta.rust_workspace.test` | `test` | `not_run` | `cargo_runner` | `test_tree_row`, `task_event_envelope` |
| `workflow.beta.rust_workspace.debug` | `debug` | `not_run` | `cargo_debug_adapter` | `debug_chronology_capture` |
| `workflow.beta.rust_workspace.migration_probe` | `migration` | `not_run` | `migration_scorecard_probe` | `migration_compatibility_scorecard_row` |
| `workflow.beta.rust_workspace.support_export` | `supportability` | `not_run` | `support_export_projection` | `redacted_support_export_row` |

### TypeScript / JavaScript web app or service

- Report row: `reference_workspace_report_row:ts_web_app_or_service`
- Archetype row: `archetype_row:ts_web_app_or_service`
- Beta archetype: `beta_archetype:ts_web_app_or_service`
- Reference workspace id: `refws.ts_web_app_archetype_seed`
- Workspace descriptor: `fixtures/workspaces/reference/ts_web_app_archetype_seed.json`
- Platform coverage: `macos_arm64`, `macos_x86_64`, `linux_x86_64`, `windows_x86_64`
- Mode scope: `local_only`, `local_plus_one_remote_mode`
- Support class: declared `experimental`, effective `retest_pending`
- Downgrade reasons: `missing_workspace_packet`, `missing_beta_harness`, `no_current_workflow_results`
- Freshness: `retest_pending`, expires `2026-06-07`, stale after `2026-06-28`
- Workflow results: no current beta harness row is materialized.

## Claim Integration

Claim publication checks compare each beta archetype row in `artifacts/release/m3/claim_manifest.json` against this report. A claim row fails publication if its effective support class is greener than the matching reference-workspace row. Badge projections in `artifacts/compat/m3/reference_workspace_badges.json` carry the same support and freshness labels for docs, Help/About, service health, release packets, partner packets, and support exports.

## How To Refresh

```sh
python3 ci/check_m3_reference_workspace_report.py --repo-root .
```

Use `--check` in CI to fail when the checked-in report, docs copy, badge projection, or validation capture would drift.

The artifact copy lives at `artifacts/compat/m3/reference_workspace_report.md`.
