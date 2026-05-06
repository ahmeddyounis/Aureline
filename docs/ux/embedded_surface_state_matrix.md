# Embedded-surface state matrix

This document publishes the **shared embedded-surface state vocabulary**
and the **fallback matrix** that embedded docs/help panes, marketplace and
account surfaces, service dashboards, and extension-owned webviews must
use. The goal is that embedded surfaces use the same controlled state
language as the rest of Aureline and never collapse distinct trust and
availability failures into a generic “webview error”.

This matrix is normative. Where it disagrees with the PRD, TAD, TDD, or
UI/UX spec, the upstream source wins and this matrix plus its companion
schema and fixtures update in the same change.

## Companion artifacts

- [`/schemas/ux/embedded_surface_state.schema.json`](../../schemas/ux/embedded_surface_state.schema.json)
  - boundary schema for `embedded_surface_state_record`, the compact,
    export-safe state description surfaces use in docs/help, support
    export, release evidence, and parity audits.
- [`/fixtures/ux/embedded_surface_state_cases/`](../../fixtures/ux/embedded_surface_state_cases/)
  - worked examples covering every embedded-surface state and mapping the
    vocabulary onto docs/help, marketplace/account, service dashboard,
    and extension-hosted surface families.

## Upstream contracts (source of truth)

This matrix does not replace these contracts; it cross-references and
binds them into one vocabulary:

- Embedded surface boundary truth and boundary-state grammar:
  [`/docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`](../adr/0015-embedded-surface-boundary-and-auth-handoff.md)
  and
  [`/schemas/ux/embedded_surface_boundary.schema.json`](../../schemas/ux/embedded_surface_boundary.schema.json)
- Render-side boundary-card contract (owner/origin chrome, fallback posture):
  [`/docs/ux/embedded_surface_boundary_cards.md`](./embedded_surface_boundary_cards.md)
  and
  [`/schemas/ux/embedded_boundary_card.schema.json`](../../schemas/ux/embedded_boundary_card.schema.json)
- Browser handoff packet (object identity + return anchor are mandatory):
  [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
- Docs/help truth (source class + version match + freshness must be quoted):
  [`/docs/adr/0013-docs-help-service-health-truth.md`](../adr/0013-docs-help-service-health-truth.md)
- Product-wide truth and degraded-state vocabulary:
  [`/docs/governance/truth_and_degraded_state_vocabulary.md`](../governance/truth_and_degraded_state_vocabulary.md)
- View freshness and captured-scope disclosure (no false “live” claims):
  [`/docs/ux/view_freshness_contract.md`](./view_freshness_contract.md)

## Core invariants (frozen)

1. **Typed, non-collapsible states.** Certificate failures, cross-origin
   limits, policy limits, cached/mirrored snapshots, and offline snapshots
   MUST remain distinct. A generic “webview error” that hides which of
   those applies is non-conforming.
2. **No false certainty.** Cached/mirrored and offline-snapshot surfaces
   MUST remain visibly non-live in UI *and* in exported evidence. Refresh
   or “retry” never mutates a captured record into “live”; it mints a new
   record with an explicit class change.
3. **Browser handoff is a typed action.** Any “Open in browser” / external
   handoff MUST route through a `browser_handoff_packet` and MUST preserve:
   (a) current object identity, (b) scope, and (c) a return anchor back to
   the originating Aureline surface. Raw URL jumps and generic home-page
   handoffs are non-conforming.

## Embedded-surface state vocabulary (closed)

Every embedded surface resolves to exactly one `state_class` from
`embedded_surface_state_record`. Each state maps to (and must be
derivable from) the upstream embedded boundary-state grammar.

| State class | User-facing meaning | Mapped embedded boundary state | Required cues (minimum) | Required fallback posture (minimum) |
|---|---|---|---|---|
| `fresh_live` | Fresh/live embedded surface with current trust and verification intact. | `live_verified` | owner/origin/data-boundary chrome; explicit “live” state label | `browser_fallback_not_applicable` |
| `cached_mirrored` | Cached or mirrored content rendered as a **non-live snapshot** (may be stale). | `stale_snapshot` | non-live label; snapshot age; source/scope disclosure; export disclaimer | `system_browser_first` with a `browser_handoff_packet` |
| `policy_limited` | Policy denies some or all embedded capability; the host withholds or narrows the body honestly. | `policy_blocked` | policy-limited label; policy reason/route; named reduced capability | `system_browser_first` *or* `external_open_blocked_by_policy` depending on whether external-open is allowed |
| `certificate_trust_blocked` | Certificate or trust verification failed; embedded body is not shown as if trusted. | `certificate_failed` | trust-blocked label; certificate-details/inspection route; no embedded body masquerade | `external_open_blocked_by_policy` with a host-native or local-inspect target |
| `cross_origin_limited` | Cross-origin constraints reduce what can be inspected or mutated; missing capability is named explicitly. | `cross_origin_limited` | cross-origin-limited label; named missing capability; honest capability list | `system_browser_first` with a `browser_handoff_packet` |
| `offline_snapshot` | Network is unavailable; only an offline snapshot is available and must be disclosed as such. | `offline_snapshot` | offline-snapshot label; snapshot age; local continuity note; export disclaimer | `external_open_unavailable_offline` with a local-inspect/export target |

Notes:

- The upstream embedded boundary-state grammar also includes
  `external_open_only`. That state is **not** part of this vocabulary; it
  is a stronger host decision that no embedded body will render in-product.
  When `external_open_only` applies, the same handoff and no-false-certainty
  rules still apply.
- `cached_mirrored` is intentionally broader than a single freshness token.
  The surface MUST still disclose freshness (e.g. `Cached` vs `Stale`) using
  the product-wide degraded-state tokens and/or the source contract it
  quotes (docs/help freshness classes, provider fetch-time disclosure).

## Browser handoff requirements (applies to all states that offer it)

When a state offers a system-browser fallback, it MUST be a
product-owned handoff:

- Use a `browser_handoff_packet` (ADR-0010) instead of a raw URL.
- Preserve object identity and scope by carrying an `object_identity` and
  a scope ref in the packet’s provenance, not in UI-only copy.
- Preserve returnability by carrying a stable `return_anchor` that routes
  back to the originating Aureline object (not a generic landing page).
- Keep the embedded surface’s owner/origin/data-boundary chrome visible so
  the handoff remains attributable even after navigation.

## Per-surface-family mapping (minimum requirements)

The vocabulary is shared, but each embedded surface family must carry
its family-specific truth alongside the state label:

- **Docs/help panes (`embedded_docs_help`)**
  - Quote docs truth from ADR-0013 (`source_class`, `version_match_state`,
    and `freshness_class`) and keep it visible alongside `cached_mirrored`
    and `offline_snapshot` disclosures.
- **Marketplace/account (`embedded_marketplace_or_account`)**
  - Keep provider identity, acting identity, and scope disclosure visible
    in every state, including `policy_limited` and `offline_snapshot`.
- **Service dashboards (`embedded_service_dashboard`)**
  - Keep the target/service identity and freshness/staleness disclosure
    visible in every state; certificate and policy failures must surface
    their typed causes.
- **Extension-owned webviews (`extension_hosted_surface`)**
  - Keep extension owner/publisher identity visible; `cross_origin_limited`
    states must name the missing capability rather than collapsing to a
    generic “limited” badge.

Surfaces that cannot meet the minimum cue set for a state MUST downgrade
to a stronger, safer state (or suppress the embedded body) rather than
rendering a weaker ambiguous treatment.

