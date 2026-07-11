# M5 embedded-origin-bar and embedded-state-panel controls

The second implement lane over the frozen [M5 embedded-boundary component matrix](m5_embedded_boundary_components_contract.md). It turns the two contributed-webview embedded-boundary components — the extension-owned **embedded origin bar** and the **embedded-state panel** — into resolvers that produce export-safe, honest projections, so a contributed webview becomes an explicit, bounded product object naming *whose* content it is and *why* it is in-product or handed off, instead of an anonymous web pane.

- Controls packet schema: `schemas/ui/m5-embedded-origin-bar-state-panel-controls.schema.json`
- Support export: `artifacts/release/m5-embedded-origin-bar-state-panel-controls-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-embedded-origin-bar-state-panel-controls-proof/matrix.csv`
- Summary: `artifacts/release/m5-embedded-origin-bar-state-panel-controls-proof/summary.md`
- Narrowed fixtures: `fixtures/ui/m5-embedded-origin-bar-state-panel-controls/`
- Resolver + validator: `crates/aureline-shell` (module `implement_the_m5_embedded_origin_bar_and_embedded_state_panel_...`)

## Reused, not re-minted

The lane binds directly to the frozen embedded-boundary and auth-boundary object model so it can never fork its own owner, origin, permission, capability, or fallback wording:

- **Boundary disposition** reuses the single controlled `M5EmbeddedBoundaryDisposition` vocabulary from the matrix (live_first_party_local, live_first_party_hosted, live_provider_owned, stale_snapshot, offline_snapshot, provider_blocked, browser_handoff_only, capability_limited, not_evaluated).
- **Owner / origin** reuses `WebviewOwnerClass` (extension-owned, provider-owned, first-party-embedded, unknown/untrusted); **origin disclosure** reuses `OriginDisclosureClass`; **permission state** reuses `WebviewPermissionState`.
- **Capability limits** reuse `CapabilityLimitClass`, **browser fallback** reuses `BrowserHandoffKind`, and **freshness** reuses the matrix `M5EmbeddedFreshnessState`.

## Embedded origin bar resolver

`resolve_embedded_origin_bar` degrades first rather than ever letting an anonymous or masquerading bar read as a clean pass:

| Condition | Degrade reason |
| --- | --- |
| Owner / origin undisclosed, blocked, or untrusted | `owner_or_origin_unstated` |
| Extension surface hides its publisher / extension name | `publisher_or_extension_unstated` |
| Capability limits unstated | `capability_limits_unstated` |
| Imitates native permission / trust / update / confirmation UI | `imitates_native_permission_ui` |
| Proof stale | `proof_stale` |

A clean bar names its owner/origin, publisher, permission state, and capability limits, and offers a reload action and an open-in-browser path — the **AC1** guarantee that no claimed M5 contributed webview appears without visible owner/origin chrome and capability-limit disclosure. A stale or offline snapshot never reads as fresh first-party local truth.

## Embedded-state panel resolver

`resolve_embedded_state_panel` explains stale, offline, policy-blocked, certificate-denied, and cross-origin-limited states with the same severity and support-boundary vocabulary as first-party Aureline surfaces:

| Condition | Degrade reason |
| --- | --- |
| State class unstated | `state_class_unstated` |
| State not explained | `state_not_explained` |
| Severity / support-boundary vocabulary forked | `support_boundary_or_severity_unstated` |
| Non-live state rendered as fresh first-party truth | `blocked_shown_as_fresh` |
| Imitates native permission / trust / update / confirmation UI | `imitates_native_permission_ui` |
| Proof stale | `proof_stale` |

The `imitates_native_permission_ui` degrade — carried by both resolvers — is the **AC2** guarantee: embedded contributed surfaces never imitate native permission, trust, update, or irreversible confirmation UI. The `blocked_shown_as_fresh` degrade keeps a stale, offline, policy-blocked, certificate-denied, or cross-origin-limited state from ever reading as fresh first-party truth.

## Guardrails

Every controls row asserts (and the validator enforces) that it never:

- masquerades as native permission or irreversible approval UI;
- hides owner/origin or the browser handoff behind menus only;
- renders a stale, offline, or provider-blocked pane as fresh first-party local truth;
- embeds a high-risk approval without a native step-up.

Acceptance criteria are proven by the resolved examples carried in the packet, not merely asserted by governance flags. Raw secret values and private endpoints never cross the export boundary.
