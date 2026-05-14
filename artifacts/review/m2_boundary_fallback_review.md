# Boundary Fallback Review

Review packet:
[`fixtures/auth/boundary_fallback_alpha/baseline_packet.json`](../../fixtures/auth/boundary_fallback_alpha/baseline_packet.json)

Docs:
[`docs/identity/boundary_fallback_alpha.md`](../../docs/identity/boundary_fallback_alpha.md)

## Source Contracts

- Claimed identity defaults:
  `fixtures/auth/system_browser_alpha/managed_claim_prefers_system_browser.json`
- Embedded boundary chrome:
  seeded `EmbeddedBoundaryAlphaSnapshot`
- Native callback and protocol handoff:
  seeded `NativeBoundaryHandoffPacket`
- Approval-ticket actor and reapproval truth:
  `fixtures/security/approval_ticket_alpha/baseline_packet.json`

## Acceptance Evidence

- Claimed managed identity defaults to system-browser auth and retains
  device-code plus stay-local alternatives.
- Docs/help, extension webview, marketplace/account, and auth-handoff rows all
  carry owner/origin, provider/domain, profile or org scope, network state,
  browser fallback, and theme/zoom/focus continuity.
- High-risk approval, destructive confirmation, and trust elevation remain
  product-owned native surfaces.
- Callback replay and trust-store-change rows fail closed, preserve exact
  origin explanation, and route to native reapproval or truthful placeholder
  recovery.
- Actor-class rows distinguish human account, installation grant, and
  delegated credential without a generic signed-in collapse.

## Verification

```sh
cargo test -p aureline-shell --test boundary_fallback_alpha --no-fail-fast
cargo test -p aureline-shell embedded::boundary_fallback_alpha --no-fail-fast
```

## Residual Risk

This is an alpha contract and fixture-backed consumer. It validates the boundary
state model before live browser launch, callback listener, or provider adapter
code exists.
