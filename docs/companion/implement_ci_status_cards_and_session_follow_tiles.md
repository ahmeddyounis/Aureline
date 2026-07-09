# M5 CI-status cards and session-follow tiles

Status: implemented. This contract implements the next two components frozen in the
[M5 companion component matrix](m5_companion_component_matrix.md) — the `ci_status_card`
and the `session_follow_tile` — as one export-safe controls packet with two co-equal
control vectors. It keeps the companion honest about live versus stale context: a user
never has to infer which run or commit a CI card refers to, whether a rerun is mobile-safe,
whether a followed session is live enough to join, or what exactly will open on desktop.

- Crate module:
  `crates/aureline-companion/src/implement_ci_status_cards_and_session_follow_tiles_with_provider_source_run_or_session_identity_stale_state_labeling_and_follow_or_handoff_continuity`
- Boundary schema:
  [`schemas/ui/m5-ci-status-card-session-follow-tile-controls.schema.json`](../../schemas/ui/m5-ci-status-card-session-follow-tile-controls.schema.json)
- Checked support export:
  `artifacts/release/m5-ci-status-card-session-follow-tile-proof/support_export.json`
  (plus `matrix.csv` and `summary.md`)
- Scenario fixtures:
  `fixtures/ui/m5-ci-status-card-session-follow-tile-controls/`
- Headless emitter:
  `cargo run -p aureline-companion --example dump_ci_status_card_session_follow_tile_controls -- <support-export|report|csv|fixture-*|validate>`

## Reused frozen vocabulary

This lane never invents a parallel companion grammar. It reuses the frozen matrix enums
verbatim: object kind, client scope, freshness, CI status, session-follow state, handoff
target, degraded reason, required labels, surface family, deployment line, consumer
surface, accessibility route, and downgrade triggers. It mints new vocabulary only for what
the matrix left implicit about these two controls.

## CI-status card

Each CI-status card names its **provider/source class** (`provider_class`,
`provider_label`), its **stable run and commit identity** (`run_ref`, `commit_ref`), the
object it references (`object_kind`, `object_label`), its **exact object landing reference**
(`object_landing_ref` — the one stable object `Open` lands on, never a generic activity
page), its repo/workspace client scope, its **failure count**, and its freshness.

Its **result class** is *derived* from the frozen CI status by `resolve_ci_result`, never
asserted:

| CI status | result class | live result? |
| --- | --- | --- |
| `passed` | `green` | yes (failure count must be 0) |
| `failed` | `red` | yes (failure count must be ≥ 1) |
| `running` / `queued` | `in_flight` | yes (in-flight note required) |
| `canceled` | `canceled` | yes |
| `stale` | `stale_unknown` | no (stale note required) |

A stale CI status can therefore never read as a live pass or fail. Each card offers a
keyboard-complete `Open` verb, and a `rerun` or `handoff_to_desktop` verb names one exact
desktop-handoff target — a card whose `handoff_target` is `no_handoff` offers neither the
`rerun` nor the `handoff_to_desktop` verb, so no desktop-only action is offered as if
companion-safe and no verb points at a target it cannot resolve.

## Session-follow tile

Each session-follow tile names its **presenter and session identity** (`presenter_ref`,
`session_ref`), the object it references, its exact object landing reference, its scope, and
its freshness.

Its **joinability class** is *derived* from the frozen session-follow state by
`resolve_session_joinability`, never asserted:

| follow state | joinability | live? | joinable? |
| --- | --- | --- | --- |
| `live_following` | `live_joinable` | yes | yes |
| `paused_follow` | `paused_resumable` | no | yes |
| `diverged_from_host` / `read_only_mirror` | `stale_read_only` | no | no (stale note required) |
| `host_inactive` / `follow_ended` | `not_joinable` | no | no (not-joinable note required) |

A diverged, stale, host-inactive, or ended session therefore degrades to an explicit
read-only or not-joinable state instead of an ambiguous empty card, and a tile that is not
joinable never offers the `follow` or `resume_follow` verb — so no user is offered an
ambiguous join into an expired or narrowed session.

## Hard invariants (per control, all `false`)

- `masks_scope_or_freshness`
- `hides_capability_boundary`
- `invents_alternate_state_label`
- `implies_desktop_action_is_companion_safe`
- `routes_to_generic_activity_page`

## Coverage

The canonical packet carries six CI-status cards covering all five result classes and all
six CI statuses, and six session-follow tiles covering all four joinability classes and all
six session-follow states. Raw log bodies, build output, secret values, and private
endpoints never cross this boundary.
