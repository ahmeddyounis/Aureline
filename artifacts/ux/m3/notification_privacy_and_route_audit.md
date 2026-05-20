# Notification privacy and route audit

Generated from the seeded corpus in `crates/aureline-shell/src/notification_envelope_corpus/mod.rs`. Every route outcome is minted from truth by the governed attention router, never copied from a screenshot or toast text.

Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- audit-md > \
  artifacts/ux/m3/notification_privacy_and_route_audit.md
```

- Packet id: `shell:notification_envelope_corpus:packet:001`
- Shared contract ref: `shell:notification_envelope_corpus:v1`
- Route-outcome schema: `schemas/ux/notification_route_outcome.schema.json`
- Generated at: `2026-05-20T00:00:00Z`
- Cases: 12 across 9 beta attention families
- All families present: yes · all cases conform: yes

## Surface resolution by case

Cells carry the `channel_resolution_class` for each surface; `—` means the surface was not recommended.

| Case | Family | Window | durable_job_row | status_item | activity_center_digest_card | contextual_banner | toast | os_notification | lock_screen_summary | companion_push |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `case:indexing-foreground-focused` | `indexing` | `foreground_focused` | delivered_in_app | delivered_in_app | — | — | delivered_in_app | suppressed_foreground_redundant | — | — |
| `case:indexing-presenting-overlay` | `indexing` | `foreground_unfocused` | delivered_in_app | delivered_in_app | — | — | suppressed_by_policy | suppressed_by_policy | — | — |
| `case:restore-placeholder-reopen` | `restore` | `background_hidden` | delivered_in_app | delivered_in_app | — | — | delivered_in_app | — | — | — |
| `case:install-update-download-locked` | `install_update_download` | `locked_or_away` | delivered_in_app | delivered_in_app | — | — | — | delivered_external_summary | delivered_external_summary | — |
| `case:ai-approval-screen-reader` | `ai_approval` | `foreground_focused` | delivered_in_app | delivered_in_app | delivered_in_app | — | delivered_in_app | suppressed_foreground_redundant | — | — |
| `case:provider-sync-revalidation` | `provider_sync` | `background_hidden` | delivered_in_app | delivered_in_app | — | — | delivered_in_app | delivered_external_summary | — | companion_unavailable |
| `case:policy-change-admin-suppressed` | `policy_change` | `background_hidden` | — | delivered_in_app | — | suppressed_by_policy | — | suppressed_by_policy | suppressed_by_policy | — |
| `case:remote-reconnect-companion-fanout` | `remote_reconnect` | `background_hidden` | delivered_in_app | delivered_in_app | — | — | — | delivered_external_summary | lock_screen_not_applicable | delivered_external_summary |
| `case:managed-alert-companion-blocked` | `managed_alert` | `background_hidden` | delivered_in_app | delivered_in_app | — | — | — | — | — | companion_policy_blocked |
| `case:classroom-presentation-overlay` | `classroom_presentation_overlay` | `foreground_unfocused` | delivered_in_app | delivered_in_app | — | — | suppressed_by_policy | suppressed_by_policy | — | suppressed_by_policy |
| `case:indexing-dedupe-burst` | `indexing` | `foreground_unfocused` | deduped_repeat | deduped_repeat | — | — | deduped_repeat | — | — | — |
| `case:provider-sync-quiet-hours-held` | `provider_sync` | `background_hidden` | delivered_in_app | — | — | — | held_by_quiet_hours_or_focus | held_by_quiet_hours_or_focus | — | held_by_quiet_hours_or_focus |

## Reopen, durable truth, and handoff proofs

| Case | Reopen proof | Companion handoff | durable_truth_preserved | reopen_target_preserved | no_generic_home_reopen | emissions |
| --- | --- | --- | --- | --- | --- | --- |
| `case:indexing-foreground-focused` | `exact_target` | `not_applicable` | yes | yes | yes | 1 |
| `case:indexing-presenting-overlay` | `exact_target` | `not_applicable` | yes | yes | yes | 1 |
| `case:restore-placeholder-reopen` | `truthful_placeholder` | `not_applicable` | yes | yes | yes | 1 |
| `case:install-update-download-locked` | `exact_target` | `not_applicable` | yes | yes | yes | 1 |
| `case:ai-approval-screen-reader` | `exact_target` | `not_applicable` | yes | yes | yes | 1 |
| `case:provider-sync-revalidation` | `denied_requires_revalidation` | `summary_fanout_unavailable` | yes | yes | yes | 1 |
| `case:policy-change-admin-suppressed` | `exact_target` | `not_applicable` | yes | yes | yes | 1 |
| `case:remote-reconnect-companion-fanout` | `exact_target` | `summary_fanout_delivered` | yes | yes | yes | 1 |
| `case:managed-alert-companion-blocked` | `exact_target` | `summary_fanout_policy_blocked` | yes | yes | yes | 1 |
| `case:classroom-presentation-overlay` | `exact_target` | `summary_fanout_policy_blocked` | yes | yes | yes | 1 |
| `case:indexing-dedupe-burst` | `exact_target` | `not_applicable` | yes | yes | yes | 4 |
| `case:provider-sync-quiet-hours-held` | `exact_target` | `summary_fanout_held` | yes | yes | yes | 1 |

## Drift drills (regression gate)

Each drill takes a conforming outcome, constructs the named regression, and shows the conformance lane rejects it. The diff is the actionable artifact.

| Drill | Violation | Diff field | Conforming | Regressed | Lane rejects | Reason tokens |
| --- | --- | --- | --- | --- | --- | --- |
| `drill:wrong-target-reopen` | `wrong_target_reopen` | `resolved_surface_routes[].reopen_target_ref` | `ux:reopen:indexing:shard:01` | `ux:reopen:generic-home` | yes | `reopen_target_divergence` |
| `drill:lock-screen-leakage` | `lock_screen_leakage` | `lock_screen_summary.resolved_receipt_state` | `suppressed_policy` | `delivered` | yes | `held_surface_upgraded_to_delivered`, `lock_screen_leak` |
| `drill:badge-inflation` | `badge_inflation` | `durable_badge_count` | `durable_count=1` | `raw_emissions=4` | yes | `held_surface_upgraded_to_delivered` |
| `drill:quiet-hours-drift` | `quiet_hours_drift` | `toast.resolved_receipt_state` | `held_quiet_hours` | `delivered` | yes | `held_surface_upgraded_to_delivered` |

## Badge integrity

| Probe | Family | Raw emissions | Durable count | OS badge | Lock-screen | Inflation prevented |
| --- | --- | --- | --- | --- | --- | --- |
| `probe:indexing-retry-storm` | `indexing` | 4 | 1 | yes | yes | yes |
| `probe:managed-quiet-hours-badge` | `managed_alert` | 3 | 1 | no | no | yes |

## Overlay parity

Presentation, follow, focus, and screen-reader postures dim audience-visible surfaces but never change the claimed durable rows or the reopen target.

| Overlay | Durable rows match baseline | Reopen matches baseline | Audience visible (baseline → overlay) |
| --- | --- | --- | --- |
| `presenting` | yes | yes | yes → no |
| `following_presenter` | yes | yes | yes → no |
| `focus_mode` | yes | yes | yes → no |
| `screen_reader_active` | yes | yes | yes → yes |

## Results

| Rule | Result |
| --- | --- |
| Every beta attention family has a worked routing case | PASS |
| Every resolved surface preserves the single reopen target | PASS |
| No outcome opens a generic home view | PASS |
| Durable truth preserved under hold, suppression, and dedupe | PASS |
| Wrong-target, lock-screen leak, badge inflation, quiet-hours drift all fail the lane | PASS |
| Badge never inflates under a retry storm | PASS |
| Presentation/follow/focus/screen-reader keep durable semantics | PASS |
