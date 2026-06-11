# Notification privacy, quiet-hours, badges, and dedupe for the M5 depth lanes

The M5 depth lanes mint new notification sources. This page is the human-facing
companion to the notification-route qualification audit minted by the
`aureline-shell` `m5_notification_routes` module. The audit is the canonical
truth object; later dashboards, docs, release-center views, and support exports
should ingest it instead of cloning its status text.

## The notification contract

Aureline's stable shell already promises that notifications never turn the
attention model into toast spam, privacy leakage, or badge ambiguity: every
alert flows through one governed notification envelope, carries a declared
privacy class, keeps lock-screen and companion copy summary-first, honours
quiet-hours and admin suppression without erasing the durable object, coalesces
repeated failures by root cause, derives badge counts from durable item state,
and reopens the exact authoritative object. The M5 depth lanes carry that
contract forward to nine new notification sources:

- **Notebook run** (`notebook_run`) — a notebook execution outcome.
- **Data/API run** (`data_api_run`) — a request or query run outcome.
- **Pipeline action** (`pipeline_action`) — a pipeline rerun/cancel outcome.
- **Profiler capture** (`profiler_capture`) — a profiler-capture completion.
- **Preview route** (`preview_route`) — a live preview-route state change.
- **Companion summary** (`companion_summary`) — a companion summary fanout.
- **Incident packet** (`incident_packet`) — an incident-packet generation outcome.
- **Sync state change** (`sync_state_change`) — a workspace sync state change.
- **Offboarding job** (`offboarding_job`) — an export-and-wipe job outcome.

No new source invents a feature-local notification rule that bypasses
quiet-hours, admin suppression, or exact-target reopen semantics. Each source
rides the one governed router and its envelope, privacy-class, and badge model.

## Notification privacy classes

Every source declares a privacy class so the router knows how much detail may
surface on a lock screen, native OS notification, or companion summary:

| Privacy class | What it means |
| ------------- | ------------- |
| `summary_safe` | Carries no workspace, code, or secret detail. |
| `workspace_sensitive` | May reference workspace content by reference only. |
| `security_critical` | Concerns credentials, approvals, or high-risk action. |
| `managed_sensitive` | Governed by admin policy and managed-depth rules. |

`security_critical` and `managed_sensitive` are the high-stakes classes: their
notifications always carry an exact-target reopen affordance and a non-empty
suppression-control set. If a notification cannot be shown safely on a lock
screen or companion surface, it degrades to a bounded summary or open-app
affordance instead of leaking detail.

## The nine notification guarantees

For each registered source the audit projects the canonical source descriptor
against the qualification result the source certifies for each of the nine
notification guarantees:

| Guarantee | What it proves |
| --------- | -------------- |
| `privacy_classification` | The source declares a privacy class instead of defaulting to clear. |
| `lock_screen_privacy` | Lock-screen / OS-notification copy is summary-first and leaks no workspace, code, or secret detail. |
| `payload_minimization` | Notification packets and support exports carry stable class and outcome enums, not secret-bearing payloads, by default. |
| `quiet_hours_policy` | Quiet-hours suppression changes fanout without erasing the durable object, reopen target, or audit trail. |
| `admin_suppression` | Admin suppression is honoured without erasing the durable object or audit trail. |
| `root_cause_dedupe` | Repeated failures or retries from one root cause coalesce instead of flooding the user with semantically identical alerts. |
| `badge_semantics` | Badge counts and companion summaries are derived from durable item state and stay correct after retries and partial delivery. |
| `exact_target_reopen` | Notifications and badges reopen the exact authoritative object rather than re-issuing a side effect from the notification surface. |
| `companion_fanout_honesty` | Companion fanout refers to the same object and labels stale or failed delivery honestly. |

A qualified guarantee carries the notification-envelope ref, the declared
privacy class, the lock-screen disclosure, and an evidence-freshness stamp the
audit requires — plus the payload-minimization, quiet-hours, admin-suppression,
dedupe, badge, reopen, or fanout outcome its guarantee needs. A red result (a
lock-screen leak, a secret-bearing payload, a bypassed quiet-hours window, an
overridden admin suppression, a duplicate flood, a raw-event badge counter, a
lost reopen target, or a silent fanout failure) is a blocker. A source that
emits through a feature-local rule outside the governed router, a marketed
guarantee claimed with no evidence, and stale evidence on a marketed guarantee
are all blockers, so release tooling can narrow a marketed source instead of
shipping it as implicitly stable.

## Quiet-hours, admin suppression, and badges

Quiet-hours and admin suppression may change fanout — fewer toasts, no native OS
alert during a quiet window, no companion push under an admin policy — but they
never erase the durable activity object, its exact-target reopen, or its audit
trail. Badge counts are reconciled from durable item state rather than raw event
fanout, so a source that retries five times or delivers partially does not paint
a drifting counter; the badge and any companion summary still refer back to the
real authoritative object.

## Channel set

The M5 sources harden the channels Aureline already claims — desktop toasts,
native OS notifications, durable activity-center rows, and companion summaries —
and never expand the channel set. Each marketed source declares the subset of
those channels it fans out to.

## Support and companion safety

Notification packets and support exports capture stable class and outcome enums
without leaking secret-bearing payloads by default. The support-export wrapper
lets a reviewer pivot from a support case to the source and descriptor revision
that leaked, flooded, or bypassed suppression, and stale or failed companion
fanout is labelled honestly rather than hidden behind a silent success.

## Canonical artifacts and verification

- Schema: `schemas/ux/m5-notification-envelope-diff.schema.json`
- Audit fixture: `fixtures/ux/m5/notification-dedupe/report.json`
- Support-export fixture: `fixtures/ux/m5/notification-dedupe/support_export.json`
- Published audit: `artifacts/ux/m5/quiet-hours-and-privacy/m5_notification_routes_audit.md`
- CI gate: `tools/ci/m5/notification_routes_check.py`

The headless inspector is the only mint-from-truth path for the fixtures and the
published audit. Regenerate and verify with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_notification_routes -- report > \
  fixtures/ux/m5/notification-dedupe/report.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_notification_routes -- support-export > \
  fixtures/ux/m5/notification-dedupe/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_notification_routes -- compact > \
  fixtures/ux/m5/notification-dedupe/compact.txt
cargo run -q -p aureline-shell --bin aureline_shell_m5_notification_routes -- report-md > \
  artifacts/ux/m5/quiet-hours-and-privacy/m5_notification_routes_audit.md
cargo test -p aureline-shell --test m5_notification_routes_fixtures
python3 tools/ci/m5/notification_routes_check.py
```
