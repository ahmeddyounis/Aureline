# Route Exposure And Handoff Beta

This document is the UX-facing contract for the beta route/exposure matrix.
The authoritative record is
[`artifacts/routes/m3/route_exposure_matrix.json`](../../../artifacts/routes/m3/route_exposure_matrix.json);
the release packet is
[`artifacts/release/m3/route_exposure_matrix.md`](../../../artifacts/release/m3/route_exposure_matrix.md);
the support packet is
[`artifacts/support/m3/provider_and_browser_handoff_audit.md`](../../../artifacts/support/m3/provider_and_browser_handoff_audit.md).

## Surface rule

Help/About, service health, diagnostics, docs/help, and support exports must
render the same row vocabulary:

- origin: `local_desktop`, `remote_helper`, `managed_workspace`,
  `browser_companion`, `provider_linked_context`,
  `embedded_docs_help_webview`, `headless_cli`;
- target: the closed route-truth `action_target_class`;
- route: the closed route-truth `action_route_class`;
- exposure: the closed route-truth `action_exposure_class`;
- approval reuse: `approval_reuse_class` plus explicit
  `reapproval_trigger_classes`;
- browser/system handoff: `browser_handoff_class` plus an opaque packet ref
  when a browser or companion boundary is crossed.

Do not replace these with surface-local labels such as "external link",
"open web", "maybe remote", or "safe enough". The row chip can use friendlier
copy, but its details, support export, and diagnostics must quote the token.

## UI behavior

| UX area | Required behavior |
| --- | --- |
| Help/About | Show route/exposure truth as a release-truth evidence family. Rows with high-risk handoff exposure must link to the support-safe row id. |
| Service health | Show a degraded or blocking state when a row has `exposure_unknown_requires_review`, missing parity, stale provider truth, or a missing handoff packet. |
| Diagnostics | Include row id, origin class, target class, route class, exposure class, approval reuse class, and handoff class. Exclude raw URLs, tokens, callback bodies, and provider payloads. |
| Docs/help | Explain the same row vocabulary and route users to the release/support packets for current evidence. |
| Support export | Preserve the same row ids and tokens that were visible in UI; do not reconstruct them from rendered text. |

## Claimed beta rows

The UX surfaces must be able to inspect these rows without opening raw logs:

- `route-exposure:provider-comment-browser-handoff`
- `route-exposure:provider-merge-mirror-step-up`
- `route-exposure:release-publish-later-offline`
- `route-exposure:managed-ci-check-copy-export`
- `route-exposure:tunnel-inspect-preview`
- `route-exposure:notebook-desktop-handoff`
- `route-exposure:voice-command-desktop-review`
- `route-exposure:browser-companion-native-depth`
- `route-exposure:preview-canvas-source-depth`
- `route-exposure:embedded-docs-open-browser`
- `route-exposure:auth-callback-system-browser-return`
- `route-exposure:provider-webhook-return-deny`

## Copy guidance

Use plain row summaries for compact UI, but keep the token available in
details and export:

| Token | Suggested UI summary |
| --- | --- |
| `provider_visible_mutation` | Provider-visible mutation |
| `publicly_visible_publish` | Public publish path |
| `browser_session_visible` | Browser-session visible |
| `third_party_callback_visible` | Third-party callback visible |
| `tunnel_exposed_public` | Tunnel-exposed route |
| `force_reapproval_on_boundary_change` | Reapprove when target, trust, policy, host, or privacy changes |
| `not_reusable_provider_side_effect` | Reapprove each provider side effect |
| `browser_companion_to_desktop` | Desktop handoff required for native-depth work |
| `provider_callback_return` | Callback return validated against handoff packet |

## Failure states

The UX must block or downgrade beta claims when the validator reports:

- a high-risk row with `exposure_unknown_requires_review`;
- a high-risk row with `uncategorized_high_risk_gap = true`;
- a missing route row for a claimed handoff route from
  `artifacts/milestones/m3/claimed_surface_register.json`;
- a missing provider route row from
  `artifacts/security/m3/route_resolution_panels/baseline_support_export.json`;
- a browser/system handoff row without an opaque handoff packet ref;
- a row that lacks parity for `help_about`, `service_health`,
  `diagnostics`, `support_export`, or `docs_help`.

Run:

```sh
python3 ci/check_m3_route_exposure_matrix.py --repo-root . --check
```
