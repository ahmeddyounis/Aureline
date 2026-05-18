# Route/exposure matrix beta packet

This packet publishes the beta route/exposure matrix used by release
evidence, Help/About, service health, diagnostics, docs/help, and support
exports. The machine-readable source is
[`artifacts/routes/m3/route_exposure_matrix.json`](../../routes/m3/route_exposure_matrix.json)
and its schema is
[`schemas/routes/exposure_matrix.schema.json`](../../../schemas/routes/exposure_matrix.schema.json).

The matrix is intentionally above the older route-origin and
provider-route packets. It does not replace them; it binds their rows to one
controlled vocabulary for origin, target, route, exposure, approval reuse,
reapproval triggers, privacy consequence, and browser/system handoff.

## Row vocabulary

| Field family | Controlled tokens |
| --- | --- |
| Origin class | `local_desktop`, `remote_helper`, `managed_workspace`, `browser_companion`, `provider_linked_context`, `embedded_docs_help_webview`, `headless_cli` |
| Target class | Reuses the route-truth target vocabulary from `schemas/remote/forwarded_endpoint.schema.json`: `local_host_target`, `remote_agent_target`, `managed_workspace_target`, `connected_provider_target`, `embedded_webview_target`, `system_browser_target`, `native_os_callback_target`, `publish_release_target`, `tunnel_exposed_target`, and related closed target classes. |
| Route class | Reuses the route-truth route vocabulary: `approval_gated_route`, `browser_handoff_route`, `embedded_webview_route`, `native_callback_route`, `provider_webhook_return_route`, `publish_pipeline_route`, `tunnel_exposed_route`, `managed_control_plane_route`, `mirror_or_private_registry_route`, and related closed route classes. |
| Exposure class | Reuses `action_exposure_class`: `no_side_effect_local_read`, `local_only_mutation`, `workspace_visible_mutation`, `provider_visible_mutation`, `publicly_visible_publish`, `cross_tenant_visible`, `third_party_callback_visible`, `browser_session_visible`, `tunnel_exposed_public`, `exposure_unknown_requires_review`. |
| Approval reuse | `not_required_read_only`, `reuse_same_target_policy_privacy`, `reuse_until_expiry_same_host`, `force_reapproval_on_boundary_change`, `force_reapproval_every_launch`, `not_reusable_provider_side_effect`. |
| Browser/system handoff | `no_browser_handoff`, `system_browser_required`, `system_browser_fallback_available`, `embedded_webview_boundary`, `browser_companion_to_desktop`, `provider_callback_return`, `blocked_by_policy`. |

## Release matrix rows

| Row | Origin | Target | Route | Exposure | Approval reuse | Handoff |
| --- | --- | --- | --- | --- | --- | --- |
| `route-exposure:provider-comment-browser-handoff` | `local_desktop` | `connected_provider_target` | `browser_handoff_route` | `provider_visible_mutation` | `not_reusable_provider_side_effect` | `system_browser_required` |
| `route-exposure:provider-merge-mirror-step-up` | `local_desktop` | `connected_provider_target` | `mirror_or_private_registry_route` | `provider_visible_mutation` | `force_reapproval_on_boundary_change` | `system_browser_required` |
| `route-exposure:release-publish-later-offline` | `headless_cli` | `publish_release_target` | `publish_pipeline_route` | `publicly_visible_publish` | `force_reapproval_on_boundary_change` | `no_browser_handoff` |
| `route-exposure:managed-ci-check-copy-export` | `managed_workspace` | `managed_workspace_target` | `managed_control_plane_route` | `workspace_visible_mutation` | `reuse_same_target_policy_privacy` | `no_browser_handoff` |
| `route-exposure:tunnel-inspect-preview` | `remote_helper` | `tunnel_exposed_target` | `tunnel_exposed_route` | `tunnel_exposed_public` | `reuse_until_expiry_same_host` | `system_browser_fallback_available` |
| `route-exposure:notebook-desktop-handoff` | `browser_companion` | `notebook_kernel_local_target` | `browser_handoff_route` | `browser_session_visible` | `force_reapproval_on_boundary_change` | `browser_companion_to_desktop` |
| `route-exposure:voice-command-desktop-review` | `local_desktop` | `local_host_target` | `approval_gated_route` | `workspace_visible_mutation` | `force_reapproval_every_launch` | `no_browser_handoff` |
| `route-exposure:browser-companion-native-depth` | `browser_companion` | `local_host_target` | `browser_handoff_route` | `browser_session_visible` | `force_reapproval_on_boundary_change` | `browser_companion_to_desktop` |
| `route-exposure:preview-canvas-source-depth` | `embedded_docs_help_webview` | `embedded_webview_target` | `embedded_webview_route` | `browser_session_visible` | `force_reapproval_on_boundary_change` | `embedded_webview_boundary` |
| `route-exposure:embedded-docs-open-browser` | `embedded_docs_help_webview` | `system_browser_target` | `browser_handoff_route` | `browser_session_visible` | `reuse_same_target_policy_privacy` | `system_browser_required` |
| `route-exposure:auth-callback-system-browser-return` | `provider_linked_context` | `native_os_callback_target` | `native_callback_route` | `third_party_callback_visible` | `force_reapproval_on_boundary_change` | `provider_callback_return` |
| `route-exposure:provider-webhook-return-deny` | `provider_linked_context` | `connected_provider_target` | `provider_webhook_return_route` | `third_party_callback_visible` | `force_reapproval_on_boundary_change` | `provider_callback_return` |

## Promotion guard

Every row above is high risk for beta promotion. A row blocks promotion when:

- `action_exposure_class` is `exposure_unknown_requires_review`;
- `promotion_guard.uncategorized_high_risk_gap` is true;
- a row lacks support-export parity on `help_about`, `service_health`,
  `diagnostics`, `support_export`, and `docs_help`;
- a claimed browser/deep-link handoff route from
  `artifacts/milestones/m3/claimed_surface_register.json` is missing from
  `route_refs`;
- a provider route row from
  `artifacts/security/m3/route_resolution_panels/baseline_support_export.json`
  is missing from `provider_route_resolution_row_refs`.

The drift gate is
[`ci/check_m3_route_exposure_matrix.py`](../../../ci/check_m3_route_exposure_matrix.py).
It writes
[`artifacts/release/m3/captures/route_exposure_matrix_validation_capture.json`](captures/route_exposure_matrix_validation_capture.json)
so release evidence can quote the same pass/fail state without re-parsing
the Markdown packet.
