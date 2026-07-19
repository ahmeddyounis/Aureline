# Crate dependency rules

This document is the human-readable projection of
`artifacts/governance/package_inventory.yaml`. Production and build
dependencies must point strictly downhill. Test-only `dev-dependencies`
are excluded from the production graph but still compile in workspace tests.

The service-plane class rules live in
`artifacts/architecture/protected_path_dependency_rules.yaml`. Both files are
synchronized by `tools/governance/sync_workspace_governance.py`.

## Layering

| Layer | Crates |
|---|---|
| L0 | `aureline-activity`, `aureline-build-farm`, `aureline-build-info`, `aureline-capabilities`, `aureline-change-objects`, `aureline-chronology`, `aureline-collab`, `aureline-commands`, `aureline-config`, `aureline-content-safety`, `aureline-continuity`, `aureline-crash`, `aureline-debug`, `aureline-deps`, `aureline-docs`, `aureline-ecosystem`, `aureline-execution`, `aureline-framework`, `aureline-generated`, `aureline-governance`, `aureline-graph-proto`, `aureline-i18n`, `aureline-install`, `aureline-navigation`, `aureline-notebook`, `aureline-notices`, `aureline-reactive-state`, `aureline-records`, `aureline-rpc`, `aureline-runbooks`, `aureline-scaffold`, `aureline-service-health-feed`, `aureline-telemetry`, `aureline-templates`, `aureline-text`, `aureline-ui`, `aureline-vfs` |
| L1 | `aureline-buffer`, `aureline-design-system`, `aureline-doctor`, `aureline-graph`, `aureline-input`, `aureline-learning`, `aureline-preview`, `aureline-render` |
| L2 | `aureline-graph-ui`, `aureline-history`, `aureline-language` |
| L3 | `aureline-git`, `aureline-workspace` |
| L4 | `aureline-runtime`, `aureline-terminal` |
| L5 | `aureline-auth` |
| L6 | `aureline-api`, `aureline-data`, `aureline-infra`, `aureline-policy`, `aureline-remote` |
| L7 | `aureline-support` |
| L8 | `aureline-profiler`, `aureline-provider` |
| L9 | `aureline-companion`, `aureline-incident`, `aureline-search`, `aureline-service-health` |
| L10 | `aureline-ai`, `aureline-collections`, `aureline-extensions`, `aureline-review` |
| L11 | `aureline-settings` |
| L12 | `aureline-recovery`, `aureline-release` |
| L13 | `aureline-cli`, `aureline-editor` |
| L14 | `aureline-shell` |
| L15 | `aureline-qe` |
| LX | `aureline-bench`, `aureline-env`, `aureline-largefile-proto`, `aureline-service`, `aureline-shell-spike` |

## Exact production/build edges

| Crate | Class | May depend on |
|---|---|---|
| `aureline-activity` | `shell_ui` | — |
| `aureline-ai` | `ai_control_plane` | `aureline-commands`, `aureline-content-safety`, `aureline-docs`, `aureline-git`, `aureline-graph`, `aureline-history`, `aureline-navigation`, `aureline-runtime`, `aureline-search` |
| `aureline-api` | `remote_helper` | `aureline-auth` |
| `aureline-auth` | `ai_control_plane` | `aureline-runtime`, `aureline-workspace` |
| `aureline-bench` | `off_cone` | `aureline-buffer`, `aureline-text` |
| `aureline-buffer` | `text_buffer` | `aureline-content-safety` |
| `aureline-build-farm` | `support_diagnostics` | — |
| `aureline-build-info` | `support_diagnostics` | — |
| `aureline-capabilities` | `shell_ui` | — |
| `aureline-change-objects` | `support_diagnostics` | — |
| `aureline-chronology` | `support_diagnostics` | — |
| `aureline-cli` | `shell_ui` | `aureline-i18n`, `aureline-release` |
| `aureline-collab` | `remote_helper` | — |
| `aureline-collections` | `index_search` | `aureline-search` |
| `aureline-commands` | `shell_ui` | — |
| `aureline-companion` | `remote_helper` | `aureline-auth`, `aureline-provider` |
| `aureline-config` | `shell_ui` | — |
| `aureline-content-safety` | `text_buffer` | — |
| `aureline-continuity` | `support_diagnostics` | — |
| `aureline-crash` | `support_diagnostics` | — |
| `aureline-data` | `ai_control_plane` | `aureline-auth` |
| `aureline-debug` | `task_execution` | — |
| `aureline-deps` | `support_diagnostics` | — |
| `aureline-design-system` | `shell_ui` | `aureline-ui` |
| `aureline-docs` | `index_search` | — |
| `aureline-doctor` | `support_diagnostics` | `aureline-i18n` |
| `aureline-ecosystem` | `updater_release` | — |
| `aureline-editor` | `shell_ui` | `aureline-buffer`, `aureline-history`, `aureline-language`, `aureline-recovery`, `aureline-render`, `aureline-text`, `aureline-ui`, `aureline-vfs`, `aureline-workspace` |
| `aureline-env` | `off_cone` | — |
| `aureline-execution` | `task_execution` | — |
| `aureline-extensions` | `updater_release` | `aureline-auth`, `aureline-content-safety`, `aureline-i18n`, `aureline-install`, `aureline-provider`, `aureline-runtime`, `aureline-search`, `aureline-support` |
| `aureline-framework` | `support_diagnostics` | — |
| `aureline-generated` | `support_diagnostics` | — |
| `aureline-git` | `task_execution` | `aureline-history`, `aureline-vfs` |
| `aureline-governance` | `support_diagnostics` | — |
| `aureline-graph` | `index_search` | `aureline-docs`, `aureline-graph-proto`, `aureline-navigation` |
| `aureline-graph-proto` | `index_search` | — |
| `aureline-graph-ui` | `index_search` | `aureline-graph`, `aureline-graph-proto` |
| `aureline-history` | `text_buffer` | `aureline-buffer`, `aureline-records`, `aureline-vfs` |
| `aureline-i18n` | `support_diagnostics` | — |
| `aureline-incident` | `support_diagnostics` | `aureline-crash`, `aureline-provider`, `aureline-support` |
| `aureline-infra` | `ai_control_plane` | `aureline-auth` |
| `aureline-input` | `shell_ui` | `aureline-commands` |
| `aureline-install` | `updater_release` | — |
| `aureline-language` | `index_search` | `aureline-content-safety`, `aureline-graph`, `aureline-navigation` |
| `aureline-largefile-proto` | `off_cone` | — |
| `aureline-learning` | `shell_ui` | `aureline-commands` |
| `aureline-navigation` | `index_search` | — |
| `aureline-notebook` | `task_execution` | — |
| `aureline-notices` | `support_diagnostics` | — |
| `aureline-policy` | `ai_control_plane` | `aureline-auth` |
| `aureline-preview` | `shell_ui` | `aureline-content-safety` |
| `aureline-profiler` | `support_diagnostics` | `aureline-build-info`, `aureline-runtime`, `aureline-support` |
| `aureline-provider` | `ai_control_plane` | `aureline-auth`, `aureline-support` |
| `aureline-qe` | `support_diagnostics` | `aureline-commands`, `aureline-docs`, `aureline-git`, `aureline-shell`, `aureline-workspace` |
| `aureline-reactive-state` | `index_search` | — |
| `aureline-records` | `support_diagnostics` | — |
| `aureline-recovery` | `support_diagnostics` | `aureline-history`, `aureline-settings`, `aureline-support`, `aureline-workspace` |
| `aureline-release` | `updater_release` | `aureline-settings` |
| `aureline-remote` | `remote_helper` | `aureline-auth`, `aureline-execution` |
| `aureline-render` | `renderer` | `aureline-text` |
| `aureline-review` | `task_execution` | `aureline-content-safety`, `aureline-git`, `aureline-graph`, `aureline-navigation`, `aureline-provider`, `aureline-search` |
| `aureline-rpc` | `remote_helper` | — |
| `aureline-runbooks` | `support_diagnostics` | — |
| `aureline-runtime` | `task_execution` | `aureline-language`, `aureline-rpc`, `aureline-workspace` |
| `aureline-scaffold` | `updater_release` | — |
| `aureline-search` | `index_search` | `aureline-docs`, `aureline-git`, `aureline-graph`, `aureline-language`, `aureline-navigation`, `aureline-provider`, `aureline-reactive-state`, `aureline-vfs`, `aureline-workspace` |
| `aureline-service` | `off_cone` | — |
| `aureline-service-health` | `support_diagnostics` | `aureline-continuity`, `aureline-provider`, `aureline-service-health-feed` |
| `aureline-service-health-feed` | `support_diagnostics` | — |
| `aureline-settings` | `shell_ui` | `aureline-design-system`, `aureline-extensions`, `aureline-i18n`, `aureline-ui` |
| `aureline-shell` | `shell_ui` | `aureline-ai`, `aureline-auth`, `aureline-buffer`, `aureline-build-info`, `aureline-change-objects`, `aureline-commands`, `aureline-content-safety`, `aureline-docs`, `aureline-editor`, `aureline-extensions`, `aureline-git`, `aureline-graph`, `aureline-graph-proto`, `aureline-history`, `aureline-i18n`, `aureline-input`, `aureline-install`, `aureline-language`, `aureline-navigation`, `aureline-policy`, `aureline-preview`, `aureline-provider`, `aureline-reactive-state`, `aureline-recovery`, `aureline-release`, `aureline-render`, `aureline-review`, `aureline-runtime`, `aureline-search`, `aureline-service-health-feed`, `aureline-settings`, `aureline-support`, `aureline-telemetry`, `aureline-terminal`, `aureline-text`, `aureline-ui`, `aureline-vfs`, `aureline-workspace` |
| `aureline-shell-spike` | `off_cone` | `aureline-build-info`, `aureline-text` |
| `aureline-support` | `support_diagnostics` | `aureline-build-farm`, `aureline-build-info`, `aureline-change-objects`, `aureline-crash`, `aureline-doctor`, `aureline-generated`, `aureline-graph`, `aureline-history`, `aureline-i18n`, `aureline-language`, `aureline-notices`, `aureline-policy`, `aureline-reactive-state`, `aureline-records`, `aureline-runtime`, `aureline-service-health-feed`, `aureline-vfs`, `aureline-workspace` |
| `aureline-telemetry` | `support_diagnostics` | — |
| `aureline-templates` | `updater_release` | — |
| `aureline-terminal` | `task_execution` | `aureline-workspace` |
| `aureline-text` | `text_buffer` | — |
| `aureline-ui` | `shell_ui` | — |
| `aureline-vfs` | `vfs_watchers` | — |
| `aureline-workspace` | `shell_ui` | `aureline-history`, `aureline-vfs` |

## Rules

- New crates and production/build edges must update the Cargo manifests,
  package inventory, ownership matrix, protected dependency rules, and
  topology docs in the same change.
- Production-facing crates must not depend on `LX` crates. `LX` is reserved
  for disposable spikes, benchmarks, prototypes, and isolated metadata models.
- Cycles in the production/build graph are forbidden. Test-only integration
  topology may use `dev-dependencies`; those edges do not authorize runtime
  or library coupling.
- A dependency-class change is an architecture change and requires the
  applicable decision and protected-path review artifacts.
- Run `python3 tools/governance/sync_workspace_governance.py` to check drift,
  and use `--write` only after the new classification or edge is reviewed.
