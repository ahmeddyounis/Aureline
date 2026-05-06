# Browser/web surface cases

These YAML files are worked, cross-surface examples for the scoped
browser and web surface capability matrix:

- `docs/web/scoped_browser_surface_matrix.md`
- `artifacts/web/scoped_browser_capabilities.yaml`

They are intentionally **illustrative** (not schema-pinned). They focus
on:

- which delivery mode is admissible (embedded webview vs system browser
  handoff vs scoped companion),
- what write scope is allowed (if any),
- how local-core fallback preserves usefulness when web surfaces are
  unavailable or policy-blocked, and
- what disclosure rows are mandatory (owner/origin chrome, handoff
  sheet, route truth, preserved-local-work).

## Files

- `docs_viewing_embedded_offline_snapshot.yaml`
- `auth_handoff_system_browser_device_code_fallback.yaml`
- `preview_share_embedded_then_system_browser_handoff.yaml`
- `incident_ci_review_embedded_artifact_then_provider_handoff.yaml`
- `admin_flow_portal_blocked_local_policy_inspection.yaml`
- `companion_notification_snapshot_and_handoff.yaml`
- `extension_embedded_surface_policy_blocked_external_open_only.yaml`

