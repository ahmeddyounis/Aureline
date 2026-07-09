# M5 notification rows and mobile review cards

Status: implemented. This contract implements the first two components frozen in the
[M5 companion component matrix](m5_companion_component_matrix.md) — the
`notification_row` and the `mobile_review_card` — as one export-safe controls packet
with two co-equal control vectors. It makes the *first glance* at a companion event or
review item trustworthy: a user never has to infer what object a tap opens, whether a
card is fresh enough to trust, how urgent it is, or whether companion execution is
sufficient before acting.

- Crate module:
  `crates/aureline-companion/src/implement_notification_rows_and_mobile_review_cards_with_object_identity_client_scope_freshness_severity_unread_and_desktop_handoff_truth`
- Boundary schema:
  [`schemas/ui/m5-notification-row-mobile-review-card-controls.schema.json`](../../schemas/ui/m5-notification-row-mobile-review-card-controls.schema.json)
- Checked support export:
  `artifacts/release/m5-notification-row-mobile-review-card-proof/support_export.json`
  (plus `matrix.csv` and `summary.md`)
- Scenario fixtures:
  `fixtures/ui/m5-notification-row-mobile-review-card-controls/`
- Headless emitter:
  `cargo run -p aureline-companion --example dump_notification_row_mobile_review_card_controls -- <support-export|report|csv|fixture-*|validate>`

## Reused frozen vocabulary

This lane never invents a parallel companion grammar. It reuses the frozen matrix
enums verbatim: object kind, client scope, freshness, disposition, severity, review
kind, notification category, handoff target, degraded reason, required labels, surface
family, deployment line, consumer surface, accessibility route, and downgrade
triggers. It mints new vocabulary only for what the matrix left implicit about these
two controls.

## Notification row

Each notification row names the object it references (`object_kind`, `object_label`),
its **exact object landing reference** (`object_landing_ref` — the one stable object
`Open` lands on, never a generic activity page), its repo/workspace client scope, its
severity and category, and its unread state.

Its **delivery class** is *derived* from the freshness class by
`resolve_notification_delivery`, never asserted:

| freshness | delivery class | live? |
| --- | --- | --- |
| `live` | `live` | yes |
| `cached` | `cached` | no (cached note required) |
| `stale` / `offline_held` / `expired_snapshot` | `stale` | no (stale note required) |
| `unknown_freshness` | `unknown` | no (unknown note required) |

A stale, offline-held, or expired notification can therefore never read as live. Each
row offers a keyboard-complete `Open` verb, and when scope must widen it names one
exact desktop-handoff target — a row whose `handoff_target` is `no_handoff` never
offers the `handoff_to_desktop` verb, so no verb points at a target it cannot resolve.

## Mobile review card

Each mobile review card names its review kind, the object it references, its exact
object landing reference, its scope, its freshness, and its companion-versus-desktop
capability boundary.

Its **capability class** is *derived* from the frozen disposition vocabulary by
`resolve_review_capability`, never asserted:

| disposition | capability class | companion execution sufficient? |
| --- | --- | --- |
| `comment_capable` | `comment_capable` | yes |
| `review_only` / `cached` / `stale` | `review_only` | yes (view only) |
| `desktop_required` / `handoff_ready` | `desktop_required` | no (desktop-required note required) |
| `policy_blocked` | `policy_blocked` | no (policy-blocked note required) |

A desktop-required or policy-blocked review can therefore never read as
companion-completable, so a user can tell whether companion execution is sufficient
before tapping `Comment` or `Approve`.

## Hard invariants (per control, all `false`)

- `masks_scope_or_freshness`
- `hides_capability_boundary`
- `invents_alternate_state_label`
- `implies_desktop_action_is_companion_safe`
- `routes_to_generic_activity_page`

## Coverage

The canonical packet carries six notification rows covering all four delivery classes
and all six severities, and six mobile review cards covering all four capability
classes and all six review kinds. Raw file bodies, diff hunks, secret values, and
private endpoints never cross this boundary.
