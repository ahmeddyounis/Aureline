# Provider and browser-handoff audit packet

This support packet is the support-safe projection of the beta
route/exposure matrix at
[`artifacts/routes/m3/route_exposure_matrix.json`](../../routes/m3/route_exposure_matrix.json).
It carries opaque refs, closed vocabulary tokens, and redaction posture only.
Raw URLs, raw provider payloads, raw callback bodies, raw tokens, raw
absolute paths, and raw user identifiers are not exportable from this packet.

## Support-export parity contract

Every row in this packet preserves the same fields Help/About, service
health, diagnostics, docs/help, and release evidence render:

- `origin.origin_class`
- `target.target_class`
- `route.action_route_class`
- `route.route_change_reason_code`
- `exposure.action_exposure_class`
- `exposure.privacy_consequence_class`
- `approval.approval_reuse_class`
- `approval.reapproval_trigger_classes`
- `handoff.browser_handoff_class`
- `support_export.consumer_surfaces`
- `promotion_guard.high_risk`

The support export row is metadata-only and must never carry raw browser
destinations or credential material.

## Audited rows

| Row | Support item | Provider row parity | Browser/system handoff | Export posture |
| --- | --- | --- | --- | --- |
| `route-exposure:provider-comment-browser-handoff` | `support.item.route_exposure.provider_comment_browser_handoff` | `route-resolution-beta:row:connected:human-dev:comment` | `browser-handoff-packet:pr:1234:comment:connected` | metadata only |
| `route-exposure:provider-merge-mirror-step-up` | `support.item.route_exposure.provider_merge_mirror_step_up` | `route-resolution-beta:row:mirror_only:reviewer:merge` | `browser-handoff-packet:pr:1234:merge:mirror_only` | metadata only |
| `route-exposure:release-publish-later-offline` | `support.item.route_exposure.release_publish_later_offline` | `route-resolution-beta:row:offline:release-signer:release-publish` | publish-later queue, no browser | metadata only |
| `route-exposure:managed-ci-check-copy-export` | `support.item.route_exposure.managed_ci_check_copy_export` | `route-resolution-beta:row:enterprise_managed:managed-bot:check-run` | no browser | metadata only |
| `route-exposure:tunnel-inspect-preview` | `support.item.route_exposure.tunnel_inspect_preview` | `route-resolution-beta:row:connected:tunnel:inspect` | `browser-handoff-packet:preview:tunnel:payments` fallback | metadata only |
| `route-exposure:notebook-desktop-handoff` | `support.item.route_exposure.notebook_desktop_handoff` | claimed handoff route | `handoff-packet:notebook-first-data-workflow` | metadata only |
| `route-exposure:voice-command-desktop-review` | `support.item.route_exposure.voice_command_desktop_review` | claimed handoff route | native desktop review sheet | metadata only |
| `route-exposure:browser-companion-native-depth` | `support.item.route_exposure.browser_companion_native_depth` | claimed handoff route | `handoff-packet:browser-companion-native-depth` | metadata only |
| `route-exposure:preview-canvas-source-depth` | `support.item.route_exposure.preview_canvas_source_depth` | claimed handoff route | `handoff-packet:preview-canvas-source-depth` | metadata only |
| `route-exposure:embedded-docs-open-browser` | `support.item.route_exposure.embedded_docs_open_browser` | embedded-boundary audit | `embedded-boundary-handoff:docs-help` | metadata only |
| `route-exposure:auth-callback-system-browser-return` | `support.item.route_exposure.auth_callback_system_browser_return` | auth handoff schema | `browser-handoff-packet:auth:return` | metadata only |
| `route-exposure:provider-webhook-return-deny` | `support.item.route_exposure.provider_webhook_return_deny` | provider callback schema | `browser-handoff-packet:webhook:return` | metadata only |

## Reapproval audit

Support triage should treat these triggers as first-class row data, not
derived prose:

| Trigger class | Support interpretation |
| --- | --- |
| `target_identity_changed` | The route may point at a different workspace, provider object, environment, callback target, or tunnel endpoint. Reapproval is required before mutation. |
| `trust_posture_changed` | Workspace trust or client posture changed. Reapproval is required for protected actions. |
| `policy_epoch_changed` | Local, org, transport, or provider policy changed. Reapproval is required where the route can mutate or expose data. |
| `host_or_domain_changed` | Browser, provider, mirror, callback, or tunnel host changed. Reapproval is required before handoff. |
| `privacy_consequence_changed` | Exposure class or privacy consequence widened. Reapproval is required before continuing. |
| `provider_scope_changed` | Acting identity, installation grant, delegated credential, or provider scope changed. Reapproval is required for provider actions. |
| `approval_expired` | The approval ticket or handoff packet expired. Reapproval is required before mutation or callback drain. |
| `freshness_floor_unmet` | Provider, mirror, route, or snapshot freshness no longer satisfies the row. Green claims are held closed. |
| `route_exposure_widened` | The route changed from local/narrow exposure to browser, provider, tunnel, public, or cross-boundary exposure. Reapproval is required. |

## Drift behavior

The validator fails the packet when a high-risk provider/browser/embedded row
is unknown, uncategorized, or missing support parity. The checked-in capture
is
[`artifacts/release/m3/captures/route_exposure_matrix_validation_capture.json`](../../release/m3/captures/route_exposure_matrix_validation_capture.json).
