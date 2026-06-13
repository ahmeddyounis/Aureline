# M5 Suspicious-Content, Safe-Preview, and Copy/Export Matrix Fixtures

These fixtures are valid, export-safe matrix packets that exercise the downgrade
behavior the canonical support export keeps green. Each one keeps every artifact
family present, trust-review and consumer-projection invariants satisfied, and
proof freshness valid — the difference is which families are narrowed and why.

## marketplace_install_held.json

The marketplace install/update family is held pending upstream publisher-identity
proof graduation. Held families do not require evidence packets; active payloads
are blocked, preview is blocked, and strong-decision strict identity rendering is
preserved while held. Every other family remains at its canonical qualification.

## notebook_active_content_blocked_narrowed.json

The notebook rich-output family is narrowed to Preview while active-content
isolation hardens. Active output is blocked pending review, the trust-class ladder
ends at `blocked`, and only raw and sanitized representations are offered.
Demonstrates the `active_content_untrusted` downgrade trigger narrowing a family
claim rather than hiding it.
