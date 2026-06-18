# M5 appearance-session runtime audit

Generated from the seeded runtime in
[`crate::appearance_session`](../../../../crates/aureline-shell/src/appearance_session/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_session -- report-md > \
  artifacts/ux/m5/appearance-session-checkpoints/m5_appearance_session_runtime_audit.md
```

- Report id: `shell:m5_appearance_session:runtime:v1`
- Source schema ref: `schemas/ux/appearance-session.schema.json`
- Canonical record schema: `schemas/ux/appearance_checkpoint.schema.json`
- Live session: `appearance-session:primary`
- Checkpoints: `6`
- Transitions: `7`
- Registered surfaces: `6`
- Marketed surfaces: `5`
- Surfaces needing reload/restart: `3`
- Live change demonstrated: `true`
- Blocking findings: `0`
- Status: **clean**
- Generated at: `2026-06-17T00:00:00Z`

## Live appearance session

| Axis | Value | Source |
| ---- | ----- | ------ |
| Theme package | `theme-pkg:aureline-default` | `theme-rev:aureline-default:1.4.0` |
| Resolved theme | `dark_reference` | `follow_system` |
| Contrast | `contrast_standard` | — |
| Accent | `system_accent` | — |
| Density | `standard` | — |
| Text scale | `100%` | `system` |
| Reduced motion | `motion_standard` | `os_signal` |
| Preview state | `preview_live` | checkpoint `appearance-checkpoint:preview-light` |

## Checkpoint ledger

| Checkpoint | Class | Scope | Atomicity | Apply | Reversible |
| ---------- | ----- | ----- | --------- | ----- | ---------- |
| `appearance-checkpoint:import-dusk` | `appearance_import_checkpoint` | `workspace_appearance` | `surface_reload_from_single_checkpoint` | `committed` | `true` |
| `appearance-checkpoint:os-contrast` | `appearance_os_signal_checkpoint` | `global_appearance` | `single_checkpoint_atomic` | `committed` | `true` |
| `appearance-checkpoint:overlay-accent` | `appearance_overlay_checkpoint` | `profile_appearance` | `single_checkpoint_atomic` | `reverted` | `true` |
| `appearance-checkpoint:partner-preview-failed` | `appearance_preview_checkpoint` | `preview_only` | `single_checkpoint_atomic` | `preflight_failed` | `true` |
| `appearance-checkpoint:policy-density` | `appearance_policy_checkpoint` | `global_appearance` | `single_checkpoint_atomic` | `committed` | `true` |
| `appearance-checkpoint:preview-light` | `appearance_preview_checkpoint` | `global_appearance` | `single_checkpoint_atomic` | `preview_live` | `true` |

## State-machine transitions

| Seq | Op | Trigger | From | To | Checkpoint | Restart/Reload |
| --: | -- | ------- | ---- | -- | ---------- | -------------- |
| 1 | `os_signal_applied` | `os_signal` | `not_previewing` | `preview_committed` | `appearance-checkpoint:os-contrast` | `false` |
| 2 | `open_preview` | `user_action` | `not_previewing` | `preview_pending_validation` | `appearance-checkpoint:preview-light` | `false` |
| 3 | `preflight_passed` | `user_action` | `preview_pending_validation` | `preview_live` | `appearance-checkpoint:preview-light` | `false` |
| 4 | `cancel_preview` | `user_action` | `preview_live` | `not_previewing` | `appearance-checkpoint:overlay-accent` | `false` |
| 5 | `validation_failed` | `user_action` | `preview_pending_validation` | `preview_failed_reverted` | `appearance-checkpoint:partner-preview-failed` | `false` |
| 6 | `revert_committed` | `sync_import` | `preview_committed` | `rollback_applied` | `appearance-checkpoint:import-dusk` | `true` |
| 7 | `commit_preview` | `managed_policy` | `preview_live` | `preview_committed` | `appearance-checkpoint:policy-density` | `false` |

## Per-surface bindings

| Surface | Family | Capability | Reload/Restart disclosed | Marketed |
| ------- | ------ | ---------- | ------------------------ | -------- |
| `surface:companion.sidecar` | `companion_surface` | `requires_app_restart` | `true` | `true` |
| `surface:data.result-grid` | `data_result_surface` | `applies_live` | `false` | `true` |
| `surface:docs.help-pane` | `docs_help_pane` | `applies_live` | `false` | `true` |
| `surface:extension.dusk-panel` | `extension_hosted_surface` | `requires_surface_reload` | `true` | `false` |
| `surface:notebook.cell-chrome` | `notebook` | `applies_live` | `false` | `true` |
| `surface:preview.browser-pane` | `preview_browser_pane` | `requires_surface_reload` | `true` | `true` |

## Findings summary

| Scope | Count |
| ----- | ----: |
| `session` | 0 |
| `checkpoint` | 0 |
| `transition` | 0 |
| `surface` | 0 |
| `total` | 0 |

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_session -- validate
cargo test -p aureline-shell --test m5_appearance_session_fixtures
python3 tools/ci/m5/appearance_session_check.py
```
