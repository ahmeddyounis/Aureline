# Notification route conformance (beta)

This corpus turns the governed attention router into a regression-gated proof system for durable notification routing. Every beta job or alert family that claims durable attention truth has a worked routing case that proves privacy-safe routing, exact-target reopen, quiet-hours/admin suppression, and export-safe route/outcome truth — instead of inferred behavior.

It is minted from `crates/aureline-shell/src/notification_envelope_corpus/mod.rs`, replayed by `crates/aureline-shell/tests/notification_envelope_corpus_fixtures.rs`, and composes the route-outcome contract at `docs/ux/m3/notification_envelope_beta_contract.md`. The route-outcome schema of record is `schemas/ux/notification_route_outcome.schema.json`.

## What every claimed family must prove

1. **One alert, every surface.** The same envelope resolves consistently across the durable row, status item, activity center, banner, toast, OS notification, lock-screen summary, and companion push.
2. **Live-channel narrowing, never widening.** Look-away drops the redundant OS toast, an unlocked device drops the lock-screen summary, an unreachable or policy-blocked companion drops the push — and no held/suppressed/deduped surface is ever upgraded back to delivered.
3. **Exact-target reopen.** Every surface keeps the single reopen target; a stale or missing target reopens a truthful placeholder or a revalidation requirement, never a generic home view.
4. **Durable truth survives the hold.** Quiet hours, admin suppression, presentation, and dedupe delay interruption while durable truth, badge integrity, and reopen semantics survive.
5. **Export-safe truth.** A support packet reconstructs class, route, suppression, and resolution from stable enums — never from badge text or toast copy.

## Family coverage

| Family | Cases | Surfaces resolved |
| --- | --- | --- |
| `indexing` | 3 | `durable_job_row`, `os_notification`, `status_item`, `toast` |
| `restore` | 1 | `durable_job_row`, `status_item`, `toast` |
| `install_update_download` | 1 | `durable_job_row`, `lock_screen_summary`, `os_notification`, `status_item` |
| `ai_approval` | 1 | `activity_center_digest_card`, `durable_job_row`, `os_notification`, `status_item`, `toast` |
| `provider_sync` | 2 | `companion_push`, `durable_job_row`, `os_notification`, `status_item`, `toast` |
| `policy_change` | 1 | `contextual_banner`, `lock_screen_summary`, `os_notification`, `status_item` |
| `remote_reconnect` | 1 | `companion_push`, `durable_job_row`, `lock_screen_summary`, `os_notification`, `status_item` |
| `managed_alert` | 1 | `companion_push`, `durable_job_row`, `status_item` |
| `classroom_presentation_overlay` | 1 | `companion_push`, `durable_job_row`, `os_notification`, `status_item`, `toast` |

## Channel-resolution coverage

- `delivered_in_app`
- `delivered_external_summary`
- `suppressed_foreground_redundant`
- `lock_screen_not_applicable`
- `companion_unavailable`
- `companion_policy_blocked`
- `held_by_quiet_hours_or_focus`
- `suppressed_by_policy`
- `deduped_repeat`

## Drift drills

Each drill ships an adversarial regression that the conformance lane must reject. The case records the exact reason tokens the lane produces, so a regression that lets the behavior through fails the fixture replay.

| Drill | Violation | Reason tokens |
| --- | --- | --- |
| `drill:wrong-target-reopen` | `wrong_target_reopen` | `reopen_target_divergence` |
| `drill:lock-screen-leakage` | `lock_screen_leakage` | `held_surface_upgraded_to_delivered`, `lock_screen_leak` |
| `drill:badge-inflation` | `badge_inflation` | `held_surface_upgraded_to_delivered` |
| `drill:quiet-hours-drift` | `quiet_hours_drift` | `held_surface_upgraded_to_delivered` |

## Conformance results

| Rule | Result |
| --- | --- |
| Every beta attention family covered | PASS |
| Every case conforms to the route lane | PASS |
| Reopen target preserved, no generic home | PASS |
| Drift drills all fail the lane | PASS |
| Badge never inflates | PASS |
| Overlays preserve durable semantics | PASS |
| Support export reconstructs from enums only | PASS |
