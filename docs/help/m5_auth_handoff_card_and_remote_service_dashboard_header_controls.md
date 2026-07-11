# M5 auth handoff-card and remote/service dashboard-header controls

The fourth implement lane over the frozen [M5 embedded-boundary component matrix](m5_embedded_boundary_components_contract.md). It turns the two auth / service-boundary embedded-boundary components — the **auth handoff card** and the **remote/service dashboard header** — into resolvers that produce export-safe, honest projections, so an auth or service-dashboard boundary becomes reviewable *without* turning embedded panes into security theater: the user can tell *whose* provider they are signing in with, *which* local state survives the handoff, *whose* service the dashboard names, and *whether* the flow may still perform a high-risk approval in embedded chrome (it may not).

- Controls packet schema: `schemas/ui/m5-auth-handoff-card-remote-service-dashboard-header-controls.schema.json`
- Support export: `artifacts/release/m5-auth-handoff-card-remote-service-dashboard-header-controls-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-auth-handoff-card-remote-service-dashboard-header-controls-proof/matrix.csv`
- Summary: `artifacts/release/m5-auth-handoff-card-remote-service-dashboard-header-controls-proof/summary.md`
- Narrowed fixtures: `fixtures/ui/m5-auth-handoff-card-remote-service-dashboard-header-controls/`
- Resolver + validator: `crates/aureline-shell` (module `implement_the_m5_auth_handoff_card_and_remote_service_dashboard_header_...`)

## Reused, not re-minted

The lane binds directly to the frozen embedded-boundary and auth-boundary object model so it can never fork its own provider, reason, fallback, expiry, or freshness wording:

- **Boundary disposition** reuses the single controlled `M5EmbeddedBoundaryDisposition` vocabulary from the matrix (live_first_party_local, live_first_party_hosted, live_provider_owned, stale_snapshot, offline_snapshot, provider_blocked, browser_handoff_only, capability_limited, not_evaluated).
- **Owner / origin** reuses `WebviewOwnerClass`; **freshness** reuses the matrix `M5EmbeddedFreshnessState`.
- **Browser handoff kind** reuses `BrowserHandoffKind`, **handoff reason** reuses `HandoffReasonClass`, **fallback state** reuses `FallbackStateClass`, and **device-code expiry** reuses `ExpiryDisclosureClass`.
- **Handoff posture** is minted here as `M5AuthHandoffPosture` (embedded_sign_in_checkpoint, system_browser_handoff, passkey_handoff, device_code_handoff, provider_content_handoff) — the axis that distinguishes an embedded checkpoint from a browser or passkey handoff.

## Auth handoff card resolver

`resolve_auth_handoff_card` degrades first rather than ever letting a security-theater card read as a clean pass:

| Condition | Degrade reason |
| --- | --- |
| Provider or domain unstated | `provider_or_domain_unstated` |
| Reason for handoff unexplained | `reason_for_handoff_unstated` |
| Local-safe continuity note missing | `local_continuity_unstated` |
| Fallback state unstated | `fallback_state_unstated` |
| Device-code posture omits code / expiry disclosure | `device_code_or_expiry_unstated` |
| Embedded surface imitates native permission / approval chrome | `imitates_native_approval_ui` |
| High-risk approval embedded without a native step-up | `high_risk_approval_embedded_without_step_up` |
| Proof stale | `proof_stale` |

A clean card names its posture, provider/domain, reason, fallback state, local-continuity note, and — under a device-code posture — its code or expiry disclosure. This is the **AC1** guarantee: users can distinguish embedded sign-in checkpoints from system-browser or passkey handoff and know which local state remains intact while the handoff completes. An embedded checkpoint carries a `capability_limited` disposition; every external handoff carries `browser_handoff_only`.

## Remote/service dashboard header resolver

`resolve_remote_service_dashboard_header` keeps the target/service identity, ownership boundary, freshness, and local recovery explicit:

| Condition | Degrade reason |
| --- | --- |
| Target / service identity unstated | `service_identity_unstated` |
| Ownership boundary (owner/origin) undisclosed | `ownership_boundary_unstated` |
| Freshness / offline state hidden | `freshness_or_offline_unstated` |
| Dashboard substitutes for primary local recovery controls | `substitutes_for_local_recovery` |
| Export / open-console action unavailable | `export_or_console_action_unavailable` |
| High-risk approval allowed inside embedded chrome | `high_risk_approval_in_embedded_chrome` |
| Proof stale | `proof_stale` |

The `substitutes_for_local_recovery`, `freshness_or_offline_unstated`, and `ownership_boundary_unstated` degrades are the **AC2** guarantee: a remote or service dashboard never substitutes for the primary local recovery controls or hides its freshness and ownership boundaries. An offline dashboard reads as `offline_snapshot`, never as fresh first-party truth.

## Guardrails

Every controls row asserts (and the validator enforces) that it never:

- masquerades as native permission or irreversible approval UI;
- hides owner/origin or the browser handoff behind menus only;
- renders a stale, offline, or provider-blocked pane as fresh first-party local truth;
- embeds a high-risk approval without a native step-up.

Acceptance criteria are proven by the resolved examples carried in the packet, not merely asserted by governance flags. Raw secret values and private endpoints never cross the export boundary.
