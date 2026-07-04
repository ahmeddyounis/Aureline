# M5 Deployment/Continuity Component Consumers

- Packet: `m5-deployment-continuity-component-consumers:stable:0001`
- As of: `2026-07-04T00:00:00Z`
- Rows: 17 across 5 consumer groups and 9 / 9 frozen families
- Families reused across groups: 5

## Rows

- **consumer:about-update:install-profile-card** — surface=about_page group=about_update family=install_profile_card authority=full label_parity=preserved handoff=none
- **consumer:about-update:deployment-summary-card** — surface=about_page group=about_update family=deployment_summary_card authority=full label_parity=preserved handoff=none
- **consumer:about-update:rollout-ring-row** — surface=update_center group=about_update family=rollout_ring_row authority=full label_parity=preserved handoff=none
- **consumer:about-update:mode-change-review-sheet** — surface=update_center group=about_update family=mode_change_review_sheet authority=full label_parity=preserved handoff=none
- **consumer:diagnostics-support:install-profile-card** — surface=diagnostics_pane group=diagnostics_support family=install_profile_card authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:diagnostics-support:control-plane-data-plane-strip** — surface=diagnostics_pane group=diagnostics_support family=control_plane_data_plane_status_strip authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:diagnostics-support:mirror-offline-artifact-row** — surface=support_bundle_flow group=diagnostics_support family=mirror_offline_artifact_row authority=export_only label_parity=disclosed_narrowed handoff=handoff_packet
- **consumer:diagnostics-support:residual-dependency-row** — surface=support_bundle_flow group=diagnostics_support family=residual_dependency_row authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:admin-offboarding:deployment-summary-card** — surface=admin_fleet_dashboard group=admin_offboarding family=deployment_summary_card authority=full label_parity=preserved handoff=none
- **consumer:admin-offboarding:rollout-ring-row** — surface=admin_fleet_dashboard group=admin_offboarding family=rollout_ring_row authority=inspect_only label_parity=disclosed_narrowed handoff=companion_app
- **consumer:admin-offboarding:install-profile-card** — surface=admin_fleet_dashboard group=admin_offboarding family=install_profile_card authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:admin-offboarding:side-by-side-import-sheet** — surface=offboarding_uninstall_flow group=admin_offboarding family=side_by_side_import_sheet authority=full label_parity=preserved handoff=none
- **consumer:browser-handoff:channel-association-review-row** — surface=handler_review_prompt group=browser_handoff family=channel_association_review_row authority=full label_parity=preserved handoff=none
- **consumer:browser-handoff:mirror-offline-artifact-row** — surface=browser_deep_link_handoff group=browser_handoff family=mirror_offline_artifact_row authority=read_only label_parity=disclosed_narrowed handoff=browser_readonly
- **consumer:docs-help-release:deployment-summary-card-docs** — surface=help_center_docs group=docs_help_release family=deployment_summary_card authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:docs-help-release:residual-dependency-row** — surface=support_export_replay group=docs_help_release family=residual_dependency_row authority=export_only label_parity=disclosed_narrowed handoff=handoff_packet
- **consumer:docs-help-release:rollout-ring-row** — surface=release_proof_surface group=docs_help_release family=rollout_ring_row authority=export_only label_parity=disclosed_narrowed handoff=handoff_packet
