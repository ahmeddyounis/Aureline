# M5 companion degraded-state continuity controls

Status: implemented. This contract governs the **degraded states** of every reusable companion
component frozen in the [M5 companion component matrix](m5_companion_component_matrix.md) — the
notification row, the mobile review card, the CI-status card, the session-follow tile, the
incident-snapshot card, and the desktop-handoff sheet — across the claimed notification and
handoff surfaces. It closes the acceptance-criteria gap that remains once the network, the auth
posture, a publish policy, or the object itself is no longer there: a user must be able to tell,
before invoking an action, whether they are looking at live, cached, offline, or blocked
companion data, and no row or card may route blindly into a broken or over-privileged path
without an explanatory state and a desktop fallback.

- Crate module:
  `crates/aureline-companion/src/ship_cached_offline_auth_blocked_and_policy_blocked_companion_states_with_summary_first_object_continuity_safe_triage_verbs_and_no_blind_tap_routing`
- Boundary schema:
  [`schemas/ui/m5-companion-degraded-state-continuity-controls.schema.json`](../../schemas/ui/m5-companion-degraded-state-continuity-controls.schema.json)
- Checked support export:
  `artifacts/release/m5-companion-degraded-state-continuity-proof/support_export.json`
  (plus `matrix.csv` and `summary.md`)
- Scenario fixtures:
  `fixtures/ui/m5-companion-degraded-state-continuity-controls/`
- Headless emitter:
  `cargo run -p aureline-companion --example dump_companion_degraded_state_continuity_controls -- <support-export|report|csv|fixture-*|validate>`

## Reused frozen vocabulary

This lane never invents a parallel companion grammar. It reuses the frozen matrix enums
verbatim: component family, object kind, client scope, freshness, handoff target, degraded
reason, required labels, surface family, deployment line, consumer surface, accessibility route,
and downgrade triggers. It mints new vocabulary only for what the matrix left implicit about
degraded states: the controlled availability state, the derived data-trust class, the derived
next-safe-action, and the keyboard-complete safe triage verbs.

## Degraded surface

Each surface names one governed component family, the object it references (`object_kind`,
`object_label`), a **preserved object summary** (`object_summary_note` — the last-known summary
shown even when full detail cannot be fetched), its **exact object landing reference**
(`object_landing_ref` — the one stable object `Open` lands on, never a generic activity page),
its **stable object identity** (`stable_object_ref`), its client scope, its freshness, and its
governed **availability state**.

Its **data-trust class** and its **next-safe-action** are both *derived* from the availability
state by `resolve_availability`, never asserted:

| availability state | data-trust class | live? | needs desktop fallback? | next-safe-action |
| --- | --- | --- | --- | --- |
| `live` | `live_trusted` | yes | no | `proceed_in_companion` |
| `cached` | `cached_reduced` | no | no | `refresh_for_latest` |
| `offline` | `offline_stale` | no | yes | `retry_when_online` |
| `auth_blocked` | `blocked` | no | yes | `reauth_on_desktop` |
| `policy_blocked` | `blocked` | no | yes | `open_on_desktop_read_only` |
| `loading` | `loading` | no | yes | `wait_for_load` |
| `deleted_object` | `gone` | no | no (stops routing) | `view_cached_summary_only` |

A cached, offline, or stale surface can therefore never read as live. Any surface that is not
live carries an explicit **state explanation** (`state_explanation_note`), and every surface
carries an explicit **next-safe-action copy** (`next_safe_action_note`), so a degraded state and
what to do about it are always visible before an action.

A surface whose primary path is **broken** (offline, loading) or **over-privileged** (auth-blocked,
policy-blocked — a publish path that is no longer allowed from the companion) carries an explicit
**desktop-fallback note** (`desktop_fallback_note`) and offers a resolvable desktop handoff (a
`handoff_to_desktop` verb plus a `handoff_target` other than `no_handoff`), so a tap never
silently fails or over-reaches. A surface whose object was **deleted** (`deleted_object`)
preserves its last-known summary and **stops routing**: its `handoff_target` is `no_handoff` and
it offers no `handoff_to_desktop` verb, so it never opens a target that no longer exists.

## Hard invariants (per surface, all `false`)

- `masks_scope_or_freshness`
- `hides_capability_boundary`
- `invents_alternate_state_label`
- `implies_desktop_action_is_companion_safe`
- `routes_to_generic_activity_page`
- `routes_blindly_into_broken_or_overprivileged_path`

## Coverage

The canonical packet carries seven degraded surfaces covering all seven availability states and
all six component families across the notification and handoff surfaces. Raw object payloads, log
bodies, secret values, and private endpoints never cross this boundary.
