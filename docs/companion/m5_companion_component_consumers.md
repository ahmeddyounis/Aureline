# M5 companion component consumers

**Status:** Stable (adoption lane over the frozen M5 companion component matrix)

This lane proves the six reusable M5 companion components are adopted consistently across every
claimed M5 companion surface, so the same object-identity, client-scope, freshness,
capability-boundary, severity, and handoff-target language survives outside any single
feature-local card. It is the closing consumer lane of batch B118, sitting on top of:

- the frozen matrix
  (`crate::freeze_the_m5_companion_component_matrix`, schema
  `schemas/ui/m5-companion-component-matrix.schema.json`), and
- the three sibling implement lanes that narrow the six families into working primitives:
  - notification row + mobile review card →
    `schemas/ui/m5-notification-row-mobile-review-card-controls.schema.json`
  - CI-status card + session-follow tile →
    `schemas/ui/m5-ci-status-card-session-follow-tile-controls.schema.json`
  - incident-snapshot card + desktop-handoff sheet →
    `schemas/ui/m5-incident-snapshot-card-desktop-handoff-sheet-controls.schema.json`

## Consumers

Ten claimed M5 companion consumers each adopt the shared components and point at the canonical
component schemas instead of re-wording facts in local prose:

| Consumer | Role |
| --- | --- |
| `inbox` | Notification Inbox |
| `review` | Review Queue |
| `ci` | CI Status |
| `session_follow` | Session Follow |
| `incident` | Incident Awareness |
| `advisory` | Advisory Center |
| `help` | Help / Docs |
| `support` | Support / Export Desk |
| `handoff` | Desktop Handoff |
| `export` | Export Packet |

The `help`, `support`, and `export` consumers are held to a stronger check: every family they
adopt must reference the canonical component schema, so a help, support, or export surface can
never drift from the product truth.

## Shared descriptor vocabulary

Every binding keeps all six descriptors explicit — the track invariant for this lane:

`object_identity`, `client_scope`, `freshness`, `capability_boundary`, `severity`,
`handoff_target`.

## Parity-health, narrowing, and live-safety honesty

A consumer renders a component under one parity-health mode. Full parity preserves the descriptor
vocabulary with no banner. Any weakened mode auto-narrows the claim and always discloses a
self-contained banner naming the exact reason, the preserved descriptors, and the recovery action
— never a generic "degraded" note:

| Parity-health mode | Narrowing reason | Recovery action | Stale / desktop / policy? |
| --- | --- | --- | --- |
| `full_parity` | — | — | no |
| `cached_narrowed` | `showing_cached_value` | `refresh_for_live_value` | no |
| `stale_narrowed` | `stale_beyond_window` | `reopen_or_wait_for_refresh` | yes |
| `desktop_required_narrowed` | `desktop_required_action` | `open_on_desktop_to_complete` | yes |
| `policy_blocked_narrowed` | `policy_blocked_on_companion` | `request_policy_grant_or_use_desktop` | yes |

A cached value narrows only the freshness — the last-known reading is still trustworthy — so it is
not counted against the live-safety-honesty invariant. A binding that reflects a stale card, a
desktop-required action, or a policy-blocked path always narrows and never asserts that the
component is live and companion-safe, so a stale card never reads as live and a desktop-required
action is never implied companion-safe.

## Guardrails (enforced by `validate`)

- Every one of the six component families is adopted by at least two distinct consumers — proof
  that they are reusable components, not one activity feed plus isolated feature-local cards.
- At least one worked binding proves a narrowed rendering with a self-contained banner, and at
  least one proves a full-parity rendering with no banner.
- At least one worked binding reflects a stale, desktop-required, or policy-blocked component and
  never asserts live-and-companion-safe; any such binding that claims live-and-companion-safe
  fails validation.
- Generic companion wording never conceals object identity, client scope, freshness, or the
  companion-versus-desktop capability boundary.

## Artifacts

Minted only by
`cargo run -p aureline-companion --example dump_companion_component_consumers`:

- `artifacts/release/m5-companion-component-consumer-proof/support_export.json`
- `artifacts/release/m5-companion-component-consumer-proof/matrix.csv`
- `artifacts/release/m5-companion-component-consumer-proof/report.md`
- `fixtures/ui/m5-companion-component-consumers/advisory_beta_narrowed.json`
- `fixtures/ui/m5-companion-component-consumers/handoff_preview_narrowed.json`

The checked-in support export and fixtures are validated against the seed builder by the inline
tests, so the in-code matrix and the on-disk artifacts can never drift.
