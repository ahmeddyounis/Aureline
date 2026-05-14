# Boundary Fallback Alpha Fixtures

These fixtures validate system-browser auth, embedded-boundary chrome,
native approval ownership, callback return review, and open-in-browser
fallbacks across identity and provider lanes.

The baseline packet quotes the first source contracts instead of copying their
state model:

- `fixtures/auth/system_browser_alpha/managed_claim_prefers_system_browser.json`
- `fixtures/security/approval_ticket_alpha/baseline_packet.json`
- seeded embedded boundary chrome from `aureline-shell`
- seeded native handoff review rows from `aureline-shell`

Raw callback payloads, provider URLs with tokens, cookies, device codes, and
secret material are intentionally absent. Rows use opaque refs so support and
review packets can reconstruct origin, target, actor class, and fallback path
without exposing sensitive data.
