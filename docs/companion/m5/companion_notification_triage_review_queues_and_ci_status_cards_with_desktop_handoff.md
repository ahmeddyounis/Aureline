# Companion Notification Triage, Review Queues, and CI-Status Cards

This document is the human-readable contract for the read-only browser and mobile
companion triage surface: notification triage, review queues, and CI-status cards,
each with an exact desktop handoff. The machine-readable truth source is the
checked-in support export; later browser/mobile companions, the desktop companion
panel, diagnostics, support exports, and Help/About surfaces ingest it instead of
cloning status text.

- Record kind: `companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff`
- Schema: `schemas/companion/companion-notification-triage-review-queues-and-ci-status-cards-with-desktop-handoff.schema.json`
- Support export: `artifacts/companion/m5/companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff/support_export.json`
- Markdown summary: `artifacts/companion/m5/companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff.md`
- Fixtures: `fixtures/companion/m5/companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff/`
- Producer crate: `aureline-companion`

## Sections and matrix inheritance

The surface has three sections. Each one inherits its qualification and staged
rollout stage from a frozen M5 companion-matrix lane (see
`docs/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes.md`),
so the surface never claims more than the matrix qualifies.

| Section | Matrix lane | Qualification | Rollout stage |
| --- | --- | --- | --- |
| `notification_triage` | `companion_notification` | stable | general_availability |
| `review_queue` | `companion_review` | stable | general_availability |
| `ci_status_cards` | `companion_session_follow` | beta | staged_rollout |

## Read-only, narrow companion

The companion is deliberately narrow and never authors edits:

- **Notification triage** items are read-only (`read_only` is always true). The
  companion can triage (acknowledge, snooze, dismiss, escalate) but never edits.
- **Review-queue** items carry an `approve_or_defer` or `escalate_only` decision
  authority — there is no authoring authority. The desktop host stays
  authoritative for every change.
- **CI-status cards** are read-only status, with a `failing_check_count` and a
  freshness label (`live`, `cached`, `stale`).

## Exact desktop handoff

Every item carries an exact desktop handoff so a companion tap resumes the
precise host context rather than an approximate one:

- `target` — what the handoff resumes (`file_location`, `review_panel`,
  `ci_pipeline`, `incident_workspace`, `agent_session`).
- `deep_link_ref` — an opaque, resolvable handoff ref. It carries no payload body.
- `resolution` — `exact` is the qualified state; degradation narrows it to
  `approximate` or `unresolved`.
- `requires_active_host` — whether resuming needs a live desktop host session.

The handoff contract asserts the surface-wide guarantees: exact targets are
required when qualified, the companion is read-only, the desktop host stays
authoritative, and no payload bodies cross the boundary.

## Locality disclosure

The surface states three things explicitly, so it never implies a second flagship
or a hidden control plane:

- **Stays local** — notification, review, and CI source events are computed and
  owned by the local core and stay inspectable offline.
- **Staged** — companion fan-out of triage, review queues, and CI cards rolls out
  per cohort and capability gate.
- **Requires provider or admin continuity** — live delivery and exact desktop
  handoff require the companion relay and an active host session; the local core
  never depends on them to function.

## Degradation automation

`CompanionTriageSurfacePacket::apply_companion_degradation` narrows the surface
from a per-surface observation, recording the reasons in `degraded_labels` so the
state is labeled rather than hidden:

- **Relay unavailable**, **stale proof**, or a **narrowed upstream matrix lane**
  narrow every section's qualification and rollout stage one step. An unavailable
  relay additionally forces every CI card to `stale`.
- **Narrowed trust** additionally narrows the review queue.
- An **inactive host session** downgrades the resolution of every handoff that
  requires an active host to `unresolved`, so an exact handoff is never claimed
  when it can no longer resolve. This also records `handoff_target_unresolved`.

A narrowed section that reaches `withheld` rollout drops out of
`publishable_sections`, so release tooling can detect and stop shipping it.

## Proof freshness

The packet carries a proof-freshness SLO (168 hours) with automatic narrowing on
stale proof. The checked-in export, Markdown summary, and fixtures are regenerated
deterministically from the first-consumer builder via:

```text
cargo run -p aureline-companion --example dump_companion_triage_surface -- canonical
cargo run -p aureline-companion --example dump_companion_triage_surface -- markdown
cargo run -p aureline-companion --example dump_companion_triage_surface -- relay_down
cargo run -p aureline-companion --example dump_companion_triage_surface -- host_inactive
```

## Boundary

Credential bodies, raw provider payloads, and raw event bodies never cross this
boundary. Headlines, summaries, and refs are redacted metadata only, and the
packet is export-safe.
