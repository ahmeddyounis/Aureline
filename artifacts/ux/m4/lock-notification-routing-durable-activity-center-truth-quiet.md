# Durable attention lock — release evidence

Reviewer-facing evidence packet for the lane that locks notification-envelope
routing, durable activity-center / job-row truth, quiet-hours policy,
privacy-safe OS alerts, interruptibility, and exact-target reopen on claimed
stable attention surfaces: one canonical record per durable attention class that
binds one-envelope routing, a durable job row that survives look-away /
sleep-resume / restart, coherent quiet-hours and admin suppression, a
summary-first privacy-safe OS alert, deterministic side-effect-free exact-target
reopen, distinct acknowledge/resolve/dismiss/snooze/mute transitions, badge
counts derived from durable item state, a public claim ceiling, an automatic
narrow-below-Stable verdict, recovery and route parity across the activity center
/ command palette / status bar / menus, accessibility across normal /
high-contrast / zoomed layouts, and rows that stay available without an account
or managed services.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Records / fixtures: [`/fixtures/ux/m4/lock-notification-routing-durable-activity-center-truth-quiet/`](../../../fixtures/ux/m4/lock-notification-routing-durable-activity-center-truth-quiet/)
- Schema: [`/schemas/ux/lock-notification-routing-durable-activity-center-truth-quiet.schema.json`](../../../schemas/ux/lock-notification-routing-durable-activity-center-truth-quiet.schema.json)
- Companion doc: [`/docs/ux/m4/lock-notification-routing-durable-activity-center-truth-quiet.md`](../../../docs/ux/m4/lock-notification-routing-durable-activity-center-truth-quiet.md)
- Typed source: `aureline_shell::notification_attention_stable` (`model`, `corpus`)
- Headless emitter: `aureline_shell_notification_attention_stable`
- Replay + invariant gate: `crates/aureline-shell/tests/notification_attention_stable_fixtures.rs`

## The claimed-stable matrix

| Record | Subsystem | Claim | Surface marker | Reopen kind | Quiet modes at mint |
| --- | --- | --- | --- | --- | --- |
| `indexing.json` | indexer | **stable** | stable | durable_activity_row | — |
| `restore.json` | vfs_save | **stable** | stable | placeholder_announced | — |
| `install_update_download.json` | install_update_attach | **stable** | stable | durable_activity_row | — |
| `ai_approval.json` | ai_apply | **stable** | stable | review_context | — |
| `provider_sync.json` | provider_bearing | **stable** | stable | denied_requires_revalidation | — |
| `policy_change.json` | admin_policy | **stable** | stable | canonical_object | admin_suppression |
| `remote_reconnect.json` | remote_agent | **stable** | stable | durable_activity_row | — |
| `managed_alert.json` | admin_policy | **stable** | stable | canonical_object | — |
| `classroom_presentation_overlay.json` | collaboration | beta (narrowed) | beta | canonical_object | presentation |

Coverage verdict: **8 Stable, 1 narrowed**. The narrowed row names the reason
`surface_not_yet_stable`; it is fully conformant on every pillar but its
attention surface marker is Beta, so it does not inherit Stable by adjacency.

## Acceptance evidence

- **No toast-only truth.** Every record has `interruptibility.no_toast_only_truth`
  and a present durable surface; `durable_job.is_durable()` holds across
  look-away and sleep/resume. Guarded by `no_launch_critical_row_is_toast_only`.
- **Badge / activity-center counts match queue/class truth.** Each badge is
  reconciled from a durable `NotificationAttentionState`; `count_class_truthful()`
  holds and a multi-count badge requires a present durable surface. Guarded by
  `badges_derive_from_durable_item_state`.
- **OS / companion paths are privacy-safe and exact-target-reopenable.**
  `privacy.is_privacy_safe()` holds; the admin-suppressed `policy_change` row
  carries a `policy_forbidden_on_lock_screen` payload that is never rendered, and
  reopen stays available on return. No notification surface can mutate:
  `reopen.no_side_effects_from_notification_surface` is true everywhere. Guarded
  by `os_alerts_are_privacy_safe_by_default` and
  `reopen_is_deterministic_and_side_effect_free`.
- **Quiet hours, admin suppression, and dedupe are inspectable.** Each record
  carries `quiet_hours.active_modes`, suppression preservation flags, and the
  audit-trail flag; the support export reconstructs them through stable enums
  without scraping message copy. Guarded by `quiet_hours_preserves_durable_truth`.
- **Distinct lifecycle transitions.** Acknowledge, resolve, dismiss, snooze, and
  mute are all present and distinguishable by export effect. Guarded by
  `lifecycle_verbs_are_distinct`.
- **Exact-target reopen is deterministic.** Stale or unavailable targets degrade
  to a truthful placeholder (`restore`, `provider_sync`) instead of opening a
  generic home. Guarded by `reopen_is_deterministic_and_side_effect_free`.
- **No over-claim; honest narrowing.** Every claim-ceiling assertion is bound to
  the derived evidence; the Beta overlay row narrows below the cutline with a
  named reason and surfaces the honesty marker. Guarded by
  `claim_ceiling_never_overclaims` and
  `narrowed_rows_drop_below_cutline_and_name_a_reason`.
- **Route / recovery / accessibility parity.** The same item opens from the
  activity center, command palette, status bar, and a menu command, keyboard
  first, in normal / high-contrast / zoomed layouts, with no account or managed
  services required.

## How to reproduce

```sh
# Re-emit the fixtures from the live projection.
cargo run -q -p aureline-shell \
  --bin aureline_shell_notification_attention_stable -- emit-fixtures \
  fixtures/ux/m4/lock-notification-routing-durable-activity-center-truth-quiet

# Plaintext support-export truth block.
cargo run -q -p aureline-shell \
  --bin aureline_shell_notification_attention_stable -- plaintext

# Stable corpus index.
cargo run -q -p aureline-shell \
  --bin aureline_shell_notification_attention_stable -- index

# Replay + invariant gate.
cargo test -p aureline-shell --test notification_attention_stable_fixtures
```
