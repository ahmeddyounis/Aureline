# Embedded boundaries, system-browser auth, and handoff truth

The M5 depth lanes ship new browser-like and provider-owned surfaces — embedded
docs/help viewers, request/runtime viewers, live preview-route panes,
marketplace/account surfaces, review/provider panes, and companion/browser
handoff entry points. Each is easy to ship as an opaque iframe that pretends to
be first-party local truth, quietly becomes the primary approval channel for a
high-risk action, drops the reason Aureline left a governed in-product surface,
or strands the user in an external browser with no way back.

This lane carries the stable v1 shell promise forward into those surfaces: every
marketed M5 embedded or provider surface stays clearly bounded, auth-safe, and
attributable instead of pretending to be first-party local truth.

The canonical truth object is the **M5 embedded-boundary qualification audit**
minted by [`crate::m5_embedded_boundaries`](../../crates/aureline-shell/src/m5_embedded_boundaries/mod.rs).
The live shell embedded/provider chrome, docs/help rails, support inspector, and
the release-center hardening matrix ingest the same audit rather than cloning
status text.

## The eight boundary guarantees

Every registered surface declares a qualification binding for each of the eight
boundary guarantees, in canonical order:

1. `owner_origin_disclosure` — the surface exposes owner/origin chrome instead of
   pretending to be ownerless first-party truth.
2. `freshness_disclosure` — the surface exposes a freshness stamp for the
   embedded or provider content.
3. `trust_boundary_chrome` — the surface stays clearly bounded and attributed
   instead of pretending to be a first-party local surface.
4. `system_browser_auth_default` — claimed identity and provider auth default to
   the system browser (or an equally explicit native flow), not an embedded
   approval.
5. `no_embedded_high_risk_approval` — high-risk or scope-widening approvals are
   blocked or routed out of the embedded pane.
6. `return_anchor_present` — a return anchor resolves the exact in-product
   surface to come back to.
7. `handoff_reason_preserved` — the handoff reason is emitted and preserved in
   support/export artifacts when Aureline leaves a governed surface.
8. `support_export_parity` — support bundles, docs/help, and release packets
   reuse the same destination descriptor shown in-product.

The guarantees roll up into four aspects: `attribution`
(`owner_origin_disclosure`, `freshness_disclosure`, `trust_boundary_chrome`),
`auth` (`system_browser_auth_default`, `no_embedded_high_risk_approval`),
`handoff` (`return_anchor_present`, `handoff_reason_preserved`), and `export`
(`support_export_parity`).

## Boundary classes and high-stakes surfaces

Each surface declares a boundary class:

- `first_party_local` — Aureline-owned local content rendered in a first-party
  pane (embedded docs viewer, help-center pane).
- `embedded_webview` — a bounded embedded browser-like surface rendering external
  or provider content (request/runtime viewer, preview-route pane).
- `provider_owned` — a provider-owned pane (marketplace/account surface,
  review/provider pane).
- `external_handoff` — an explicit handoff to the system browser, a vendor
  portal, or a provider console (companion/browser handoff, provider-console
  handoff).

`provider_owned` and `external_handoff` are **high-stakes**: they carry
claimed-identity / provider auth or leave the governed in-product surface, so the
audit requires a present return-anchor outcome on every qualified guarantee and a
non-empty boundary-chrome set on the descriptor. Content surfaces that never
authenticate and expose no mutating approval narrow `system_browser_auth_default`
and `no_embedded_high_risk_approval` as `not_applicable` with a documented
reason, rather than claiming a guarantee they cannot honor.

## Blocking findings

A qualified guarantee is red (a blocker) when it hides the owner/origin
(`owner_origin_hidden`), hides freshness (`freshness_hidden`), pretends to be
first-party (`pretends_first_party`), makes the embedded pane the primary auth
approval channel (`embedded_primary_auth`), hides a high-risk approval inside the
pane (`embedded_high_risk_approval`), loses the return anchor
(`return_anchor_lost`), drops the handoff reason (`handoff_reason_dropped`), or
clones divergent support text (`support_parity_divergent`). A surface that paints
its own boundary chrome outside the governed model
(`unqualified_local_surface`), a marketed guarantee claimed with no evidence
(`missing_evidence`), and stale evidence on a marketed guarantee
(`stale_evidence_on_marketed_row`) are also blockers, so release tooling can
narrow a marketed surface instead of shipping it as implicitly stable.

## Canonical artifacts

- Schema: `schemas/help/m5-destination-descriptor-diff.schema.json`
- Report fixture: `fixtures/ux/m5/webview-auth-handoff/report.json`
- Support-export fixture: `fixtures/ux/m5/webview-auth-handoff/support_export.json`
- Compact fixture: `fixtures/ux/m5/webview-auth-handoff/compact.txt`
- Published audit: `artifacts/ux/m5/embedded-boundary-audits/m5_embedded_boundaries_audit.md`
- CI gate: `tools/ci/m5/embedded_boundaries_check.py`

## Verify

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_embedded_boundaries -- validate
cargo test -p aureline-shell --test m5_embedded_boundaries_fixtures
python3 tools/ci/m5/embedded_boundaries_check.py
```

Regenerate the checked-in fixtures and the published audit from the one
mint-from-truth path after any change to the seed:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_embedded_boundaries -- report > \
  fixtures/ux/m5/webview-auth-handoff/report.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_embedded_boundaries -- support-export > \
  fixtures/ux/m5/webview-auth-handoff/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_embedded_boundaries -- compact > \
  fixtures/ux/m5/webview-auth-handoff/compact.txt
cargo run -q -p aureline-shell --bin aureline_shell_m5_embedded_boundaries -- report-md > \
  artifacts/ux/m5/embedded-boundary-audits/m5_embedded_boundaries_audit.md
```
