# M5 Adaptive-Efficiency Component Consumers

- Packet: `m5-efficiency-component-consumers:stable:0001`
- As of: `2026-07-10T00:00:00Z`
- Rows: 19 across 5 consumer classes and 8 / 8 frozen families
- Controls lanes adopted: 4 / 4
- Work dispositions preserved: 9 / 9
- Families reused across classes: 7

## Rows

- **consumer:shell-status:power-state** — surface=shell_status_bar class=shell_status_activity family=power_state_indicator lane=power_throttle authority=full label_parity=preserved handoff=none
- **consumer:activity-center:background-row** — surface=activity_center class=shell_status_activity family=background_work_row lane=background_work authority=full label_parity=preserved handoff=none
- **consumer:activity-center:override-sheet** — surface=activity_center class=shell_status_activity family=per_workspace_override_sheet lane=override_policy authority=full label_parity=preserved handoff=none
- **consumer:activity-center:resume-card** — surface=activity_center class=shell_status_activity family=resume_summary_card lane=resume_continuity authority=full label_parity=preserved handoff=none
- **consumer:background-tray:background-banner** — surface=background_work_tray class=shell_status_activity family=background_work_banner lane=background_work authority=full label_parity=preserved handoff=none
- **consumer:notebook:throttled-row** — surface=notebook_canvas class=work_content_surface family=throttled_subsystem_row lane=power_throttle authority=read_only label_parity=disclosed_narrowed handoff=desktop_shell
- **consumer:preview:background-row** — surface=preview_pane class=work_content_surface family=background_work_row lane=background_work authority=read_only label_parity=disclosed_narrowed handoff=desktop_shell
- **consumer:pipeline:throttled-row** — surface=pipeline_runner class=work_content_surface family=throttled_subsystem_row lane=power_throttle authority=inspect_only label_parity=disclosed_narrowed handoff=desktop_shell
- **consumer:graph:stale-note** — surface=graph_explorer class=work_content_surface family=stale_result_continuity_note lane=resume_continuity authority=read_only label_parity=disclosed_narrowed handoff=desktop_shell
- **consumer:docs-browser:power-state** — surface=docs_browser_handoff class=docs_browser_companion family=power_state_indicator lane=power_throttle authority=read_only label_parity=disclosed_narrowed handoff=browser_readonly
- **consumer:docs-browser:override-note** — surface=docs_browser_handoff class=docs_browser_companion family=override_policy_note_row lane=override_policy authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:companion:background-banner** — surface=companion_adjacent class=docs_browser_companion family=background_work_banner lane=background_work authority=inspect_only label_parity=disclosed_narrowed handoff=desktop_shell
- **consumer:incident:power-state** — surface=incident_console class=incident_diagnostics family=power_state_indicator lane=power_throttle authority=read_only label_parity=disclosed_narrowed handoff=desktop_shell
- **consumer:diagnostics:override-note** — surface=diagnostics_panel class=incident_diagnostics family=override_policy_note_row lane=override_policy authority=read_only label_parity=disclosed_narrowed handoff=desktop_shell
- **consumer:diagnostics:resume-card** — surface=diagnostics_panel class=incident_diagnostics family=resume_summary_card lane=resume_continuity authority=read_only label_parity=disclosed_narrowed handoff=desktop_shell
- **consumer:support-export:override-sheet** — surface=support_export_replay class=support_export_help family=per_workspace_override_sheet lane=override_policy authority=export_only label_parity=disclosed_narrowed handoff=support_packet
- **consumer:support-export:stale-note** — surface=support_export_replay class=support_export_help family=stale_result_continuity_note lane=resume_continuity authority=export_only label_parity=disclosed_narrowed handoff=support_packet
- **consumer:help-about:power-state** — surface=help_about_reference class=support_export_help family=power_state_indicator lane=power_throttle authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:help-about:resume-card** — surface=help_about_reference class=support_export_help family=resume_summary_card lane=resume_continuity authority=read_only label_parity=disclosed_narrowed handoff=none
