# M5 Companion Component Accessibility & Auto-Narrowing (M05-1002)

This lane is the **accessibility / keyboard / screen-reader / share-export parity and honest
auto-narrowing capstone** over the frozen M5 companion component matrix
(`schemas/ui/m5-companion-component-matrix.schema.json`, M05-996). Where the freeze matrix defines
the reusable notification row, mobile review card, CI-status card, session-follow tile,
incident-snapshot card, and desktop-handoff sheet primitives, and the M05-997 through M05-1001
implementation / degraded-state / consumer lanes resolve their per-surface truth, this lane
certifies — per component family — that companion claims stay **keyboard-complete,
assistive-tech-reachable, share/export-safe, and self-narrowing** across desktop, assistive,
headless, and export paths.

- Module: `crates/aureline-companion/src/implement_keyboard_screen_reader_share_export_parity_and_automatic_narrowing_when_object_freshness_companion_authority_tenant_scope_or_handoff_validity_is_stale_limited_or_revoked_across_claimed_m5_companion_components`
- Schema: `schemas/ui/m5-companion-component-accessibility-parity.schema.json`
- Support export: `artifacts/release/m5-companion-component-accessibility-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-companion-component-accessibility-proof/matrix.csv`
- Report: `artifacts/release/m5-companion-component-accessibility-proof.md`
- Fixtures: `fixtures/ui/m5-companion-component-accessibility-parity/`

## What it guarantees

1. **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
   screen-reader-reachable, and CLI/headless-reachable path into the same canonical object
   identity, workspace/repo client scope, freshness class, companion-versus-desktop capability
   boundary, severity, and exact desktop-handoff target the rich component shows — never a
   hover-only chip that strands assistive-tech or headless users. Hierarchy-heavy families (the
   incident-snapshot card's nested service / run / severity / status lineage) additionally bind
   their tree to a flat list / textual path.
2. **Share / export parity.** The support / notification-export / issue-report share reconstructs
   each component's meaning from typed tokens and opaque refs without a screenshot, preserving the
   same canonical IDs, client scopes, freshness classes, capability boundaries, handoff targets,
   and narrowing reasons shown in-product — and never a raw payload body.
3. **Honest auto-narrowing.** When object freshness is stale, companion authority is limited,
   tenant scope has narrowed, or handoff validity is revoked, the component's companion claim
   auto-narrows from `live_companion_safe` / `cached_continuity_safe` to a
   `stale_freshness_projection` / `limited_authority_projection` / `narrowed_tenant_projection` /
   `revoked_handoff_projection`, discloses the narrowing with a precise trigger and binding
   dimension, and preserves the canonical identity / scope / freshness / handoff lineage. A
   component with every dimension intact must not carry a spurious narrowing, and a **stale,
   limited, or revoked state never masquerades as live-companion-safe** — it never silently implies
   live data or allowed companion mutation.
4. **Cross-surface disclosure.** The same narrowed state surfaces in the notification-triage,
   review-queue, CI-status, session-follow, incident-awareness, desktop-handoff, status-bar,
   general product UI, and support / export share so product, docs, and release publication stay
   aligned on companion-boundary downgrade behavior — no generic companion wording is allowed to
   conceal object identity, client scope, freshness, companion-versus-desktop capability boundary,
   or exact desktop-handoff target.

## Claim ladder

| Claim | Rank | Meaning |
| --- | --- | --- |
| `live_companion_safe` | 5 | Fresh data, in-authority companion capability, in-scope tenant, resolvable handoff; trustworthy and actionable now. |
| `cached_continuity_safe` | 4 | Self-sufficient cached / summary-first projection (a stable labeled snapshot); not itself a live, in-authority surface. |
| `stale_freshness_projection` | 3 | Object freshness has gone stale; must not read as live until refreshed. |
| `limited_authority_projection` | 2 | Companion authority is limited (desktop-required / read-only); not a fully companion-safe action surface until widened on desktop. |
| `narrowed_tenant_projection` | 1 | Tenant / client scope narrowed from what was granted; cannot present as in-scope until reconciled. |
| `revoked_handoff_projection` | 0 | Handoff target revoked or unresolvable; cannot claim it will open the intended object on desktop. |

## Condition → ceiling → trigger

| Condition state | Permitted ceiling | Frozen trigger |
| --- | --- | --- |
| `live_in_scope` | `live_companion_safe` | — (baseline) |
| `freshness_stale` | `stale_freshness_projection` | `freshness_hidden` |
| `authority_limited` | `limited_authority_projection` | `capability_boundary_unstated` |
| `tenant_scope_narrowed` | `narrowed_tenant_projection` | `client_scope_unstated` |
| `handoff_revoked` | `revoked_handoff_projection` | `handoff_target_unresolved` |

`freshness_stale`, `authority_limited`, and `handoff_revoked` are the states that would silently
imply live data or allowed companion mutation: a row modeling any of them must never let its
effective claim assert `live_companion_safe`.

## Coverage

Six rows over six frozen families (one per family): **2 green / 4 yellow / 0 red**. Every claim
dimension, every condition state, every claim tier, and all nine consumer surfaces are exercised
across the packet.

## Regenerating artifacts

The support export, CSV, report, and fixtures are generated from the single in-code seed and are
byte-checked by the test suite. Regenerate with:

```
GEN_COMPANION_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-companion --lib generate_artifacts
```
