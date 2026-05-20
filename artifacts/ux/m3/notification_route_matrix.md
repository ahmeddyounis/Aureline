# Notification Route Matrix

This matrix summarizes how one notification envelope resolves across every
durable attention surface under each live channel context. The source of
truth is the generated corpus under
[`fixtures/ux/m3/notification_routing/`](../../../fixtures/ux/m3/notification_routing/),
minted by the headless inspector
([`crates/aureline-shell/src/bin/aureline_shell_attention_router.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_attention_router.rs)),
not from screenshots or copied toast text. Regenerate the corpus and then
re-derive this table whenever routing behavior changes.

## Surface resolution by case

Cell legend: `deliver(in-app)` = delivered to an in-product surface;
`deliver(ext)` = delivered to an external summary surface; `drop(fg)` = OS
notification dropped as redundant while foreground; `n/a(unlocked)` =
lock-screen summary not applicable; `n/a(no companion)` = no reachable
companion; `block(policy)` = suppressed by managed policy; `held` = held by
quiet hours / focus / presentation; `suppress` = suppressed by policy;
`dedupe` = coalesced repeat; `—` = surface not recommended.

| Case | Window | durable_job_row | status_item | activity_center_digest_card | contextual_banner | toast | os_notification | lock_screen_summary | companion_push |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `foreground-focused-in-app` | foreground_focused | deliver(in-app) | deliver(in-app) | — | — | deliver(in-app) | drop(fg) | — | — |
| `background-os-delivery` | background_hidden | deliver(in-app) | deliver(in-app) | — | — | deliver(in-app) | deliver(ext) | n/a(unlocked) | n/a(no companion) |
| `locked-lock-screen-summary` | locked_or_away | deliver(in-app) | deliver(in-app) | — | — | — | deliver(ext) | deliver(ext) | — |
| `quiet-hours-companion-held` | background_hidden | deliver(in-app) | — | — | — | held | held | — | held |
| `admin-suppressed-security` | background_hidden | — | deliver(in-app) | — | suppress | — | suppress | suppress | — |
| `dedupe-burst-repeat` | foreground_unfocused | dedupe | dedupe | — | — | dedupe | — | — | — |
| `companion-available-fanout` | background_hidden | deliver(in-app) | deliver(in-app) | — | — | — | deliver(ext) | — | deliver(ext) |
| `companion-policy-blocked` | background_hidden | deliver(in-app) | deliver(in-app) | — | — | — | — | — | block(policy) |
| `screen-reader-navigable` | foreground_focused | deliver(in-app) | deliver(in-app) | deliver(in-app) | — | deliver(in-app) | drop(fg) | — | — |
| `placeholder-reopen` | background_hidden | deliver(in-app) | deliver(in-app) | — | — | deliver(in-app) | — | — | — |

## Handoff and exit-gate proofs

| Case | Companion handoff | durable_truth_preserved | reopen_target_preserved | no_generic_home_reopen | emissions |
| --- | --- | --- | --- | --- | --- |
| `foreground-focused-in-app` | not_applicable | true | true | true | 1 |
| `background-os-delivery` | summary_fanout_unavailable | true | true | true | 1 |
| `locked-lock-screen-summary` | not_applicable | true | true | true | 1 |
| `quiet-hours-companion-held` | summary_fanout_held | true | true | true | 1 |
| `admin-suppressed-security` | not_applicable | true | true | true | 1 |
| `dedupe-burst-repeat` | not_applicable | true | true | true | 4 |
| `companion-available-fanout` | summary_fanout_delivered | true | true | true | 1 |
| `companion-policy-blocked` | summary_fanout_policy_blocked | true | true | true | 1 |
| `screen-reader-navigable` | not_applicable | true | true | true | 1 |
| `placeholder-reopen` | not_applicable | true | true | true | 1 |

## Coverage

- **Surfaces exercised:** `durable_job_row`, `status_item`,
  `activity_center_digest_card`, `contextual_banner`, `toast`,
  `os_notification`, `lock_screen_summary`, `companion_push`.
- **Channel-resolution classes exercised:** `delivered_in_app`,
  `delivered_external_summary`, `suppressed_foreground_redundant`,
  `lock_screen_not_applicable`, `companion_unavailable`,
  `companion_policy_blocked`, `held_by_quiet_hours_or_focus`,
  `suppressed_by_policy`, `deduped_repeat`.
- **Companion handoff classes exercised:** `not_applicable`,
  `summary_fanout_delivered`, `summary_fanout_held`,
  `summary_fanout_unavailable`, `summary_fanout_policy_blocked`.

## Results

| Rule | Result |
| --- | --- |
| One route outcome per envelope, stable across surfaces | PASS |
| Every resolved surface preserves the single reopen target | PASS |
| No outcome opens a generic home view | PASS |
| Durable truth preserved under quiet hours and admin suppression | PASS |
| OS / companion handoff stays summary-first with forbidden shortcuts | PASS |
| Held / suppressed / deduped surfaces never upgraded to delivered | PASS |
| Support export carries enums only, no raw user-facing copy | PASS |
