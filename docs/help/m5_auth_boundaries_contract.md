# M5 Auth-Boundary: Browser / Device-Code Handoff Cards and Webview Origin Bars

This document is the contract for the M5 auth-boundary lane: the canonical source
for how Aureline tells the truth when authentication, a device code, or provider
content crosses out of native desktop chrome into the system browser or an
embedded webview. Help/About, voice, and admin surfaces ingest the two checked-in
sets rather than minting parallel handoff dialogs or origin bars, so a user can
always distinguish native product chrome from browser/provider-owned content
before granting trust or entering credentials.

- Record kinds: `m5_browser_handoff_card_set`, `m5_webview_origin_bar_set`
- Schemas:
  [`schemas/help/m5-browser-handoff-card.schema.json`](../../schemas/help/m5-browser-handoff-card.schema.json),
  [`schemas/help/m5-webview-origin-bar.schema.json`](../../schemas/help/m5-webview-origin-bar.schema.json)
- Canonical support exports:
  [`artifacts/help/m5-auth-boundary-proof/browser_handoff_cards.json`](../../artifacts/help/m5-auth-boundary-proof/browser_handoff_cards.json),
  [`artifacts/help/m5-auth-boundary-proof/webview_origin_bars.json`](../../artifacts/help/m5-auth-boundary-proof/webview_origin_bars.json)
- Governance summary: [`artifacts/help/m5-auth-boundary-governance.md`](../../artifacts/help/m5-auth-boundary-governance.md)
- Matrix CSVs:
  [`artifacts/help/m5-auth-boundary-browser-cards.csv`](../../artifacts/help/m5-auth-boundary-browser-cards.csv),
  [`artifacts/help/m5-auth-boundary-webview-bars.csv`](../../artifacts/help/m5-auth-boundary-webview-bars.csv)
- Fixtures: [`fixtures/help/auth-boundary/`](../../fixtures/help/auth-boundary/)
- Producers: `aureline_shell::m5_auth_boundaries::current_stable_m5_browser_handoff_card_set`,
  `aureline_shell::m5_auth_boundaries::current_stable_m5_webview_origin_bar_set`
- Headless emitter: `aureline_shell_m5_auth_boundaries`

## Browser / device-code handoff cards

One card is named per handoff kind. Each card declares the provider/domain, the
reason for the handoff, the data-exit boundary plus a reviewable note, the
fallback state if the handoff is blocked, a local-continuity note, the
device-code and expiry disclosure where relevant, and a return anchor.

| Handoff kind | Reason | Data exit | Device code |
| --- | --- | --- | --- |
| `system_browser_auth` | Authenticate with provider | `vendor_or_third_party_outbound` | — |
| `device_code_auth` | Authorize device code | `no_payload_leaves_product` | Code + expiry disclosed |
| `provider_content_view` | View provider content | `external_public_browse` | — |
| `vendor_outbound_link` | Open vendor resource | `vendor_or_third_party_outbound` | — |

## Webview origin bars

One bar is named per owner class. Each bar discloses the extension/provider/origin
that owns the content, the permission state, an open-in-browser action, and the
capability limits relative to native trusted chrome.

| Owner class | Origin disclosure | Permission |
| --- | --- | --- |
| `extension_owned` | Named extension origin | Scoped permissions granted |
| `provider_owned` | Named provider origin | No elevated permissions |
| `first_party_embedded` | First-party origin | No elevated permissions |
| `unknown_untrusted` | Undisclosed origin (blocked) | Permission denied |

## Controlled vocabularies

- **Handoff kind** — `system_browser_auth`, `device_code_auth`,
  `provider_content_view`, `vendor_outbound_link`.
- **Handoff reason** — `authenticate_with_provider`, `authorize_device_code`,
  `view_provider_content`, `open_vendor_resource`.
- **Fallback state** — `local_continuity_preserved`, `retry_handoff_in_app`,
  `manual_code_entry`, `copy_link_for_manual_open`.
- **Expiry disclosure** — `expires_with_countdown`, `expires_at_disclosed_time`,
  `no_expiry_applicable`.
- **Owner class** — `extension_owned`, `provider_owned`, `first_party_embedded`,
  `unknown_untrusted`.
- **Origin disclosure** — `named_extension_origin`, `named_provider_origin`,
  `first_party_origin`, `undisclosed_origin_blocked`.
- **Permission state** — `no_elevated_permissions`, `scoped_permissions_granted`,
  `permission_request_pending`, `permission_denied`.
- **Capability limit** — `not_native_trust_chrome`, `cannot_verify_updates`,
  `cannot_grant_device_permission`, `cannot_display_product_security`,
  `cannot_enter_product_credentials`.
- **Data-exit boundary** — reused verbatim from the About/help/community
  destination vocabulary in `aureline_shell::public_truth`.

## Invariants

The producers enforce, and the schemas mirror, the following:

- **Native chrome is distinguishable.** A browser handoff card sets
  `opens_outside_native_chrome = true` and `impersonates_native_chrome = false`
  and labels provider-owned content. A webview origin bar sets
  `labeled_as_embedded = true` and `impersonates_native_chrome = false`.
- **Handoffs preserve target identity, expiry, and return-path truth.** The
  reason must match the handoff kind, the data-exit boundary must be honest for
  the kind, a device-code handoff (and only a device-code handoff) carries a
  device-code disclosure whose expiry is real and whose code is shown in-app and
  never transmitted, and every card carries a local-continuity note and a return
  anchor.
- **Embedded surfaces disclose capability limits.** Every bar discloses at least
  the `not_native_trust_chrome` limit, and the set covers the full
  capability-limit vocabulary, so embedded content never pretends parity with
  native trusted surfaces.
- **Embedded surfaces never impersonate native trust messaging.** Every bar holds
  `may_show_update_verification`, `may_show_device_permission_prompt`, and
  `may_show_product_security_messaging` at `false`, and an unknown/untrusted
  origin is disclosed as blocked with a non-elevated or denied permission state.

Raw URLs, raw email addresses, raw local paths, raw usernames, raw hostnames,
tokens, and raw secret material never cross this boundary; the records carry
opaque refs and bounded reviewable sentences only.

## Versioning

Adding a new handoff kind, owner class, fallback state, expiry disclosure,
permission state, or capability-limit class is additive-minor and bumps the
relevant schema version. Repurposing an existing value is breaking and requires a
new decision row. This lane does not invent new auth protocols; it hardens
boundary honesty around the current M5 browser/auth/device-code flows.
