# Boundary Fallback Alpha

This page is the reviewer entry point for validating system-browser auth,
embedded-boundary chrome, callback origin truth, provider actor truth, native
approval ownership, and open-in-browser fallbacks across identity and provider
lanes.

## Runtime Contract

The first shell consumer is
[`BoundaryFallbackAlphaPacket`](../../crates/aureline-shell/src/embedded/boundary_fallback_alpha.rs).
It quotes the existing source contracts instead of cloning their authority
state:

- [`SystemBrowserAlphaPacket`](../../crates/aureline-auth/src/system_browser/mod.rs)
  for claimed identity rows that default to the system browser while exposing
  device-code and stay-local alternatives.
- [`EmbeddedBoundaryAlphaSnapshot`](../../crates/aureline-shell/src/embedded/boundary_alpha.rs)
  for docs/help, extension webview, and marketplace/account owner/origin
  chrome.
- [`NativeBoundaryHandoffPacket`](../../crates/aureline-shell/src/deeplink/native_handoff.rs)
  for browser callback and protocol handoff origin, target, channel, build, and
  recovery review.
- [`ApprovalTicketAlphaPacket`](../../crates/aureline-provider/src/approval_tickets/mod.rs)
  for high-risk provider/helper mutation actor, target, expiry, drift, and
  native reapproval truth.

## Protected Fixture

The protected packet lives at
[`fixtures/auth/boundary_fallback_alpha/baseline_packet.json`](../../fixtures/auth/boundary_fallback_alpha/baseline_packet.json).
It covers:

- claimed managed identity defaulting to system-browser auth;
- provider-linked mutation fallback and product-owned approval ticket refs;
- product docs/help panes, extension-owned webviews, and marketplace/account
  content with owner/origin, browser fallback, and theme/zoom/focus continuity;
- a bounded embedded auth exception with provider/domain visibility and a
  device-code fallback;
- callback replay plus trust-store-change examples that fail closed and route
  to native reapproval or truthful placeholder recovery;
- human account, installation grant, and delegated credential actor rows.

## Validation Rules

Claimed identity/provider/auth rows must default to system-browser auth unless
they name a bounded embedded exception with provider/domain visibility,
lower-trust cues, and a browser/device-code/placeholder fallback.

Embedded surfaces may request native review, but high-risk approvals,
destructive confirmations, and trust elevation remain product-owned native
surfaces. Open-in-browser fallback must preserve object identity or show a
truthful placeholder instead of silently widening authority.

Callback and deep-link returns must disclose source/origin, requested action,
target scope, channel/build owner, and confirm/reject behavior. Stale,
replayed, expired, sleep/wake, and trust-store-change returns must fail closed
and require native review when authority or target state changed while Aureline
was out of focus.

## Verification

```sh
cargo test -p aureline-shell --test boundary_fallback_alpha --no-fail-fast
cargo test -p aureline-shell embedded::boundary_fallback_alpha --no-fail-fast
```
