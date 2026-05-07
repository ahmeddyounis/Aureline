# Presence, OS badge, and OS notification parity cases

Worked parity audit cases for collaboration presence and authority projections
defined in:

- `/docs/ux/native_presence_notification_parity.md`
- `/artifacts/ux/presence_surface_matrix.yaml`

These cases exist to prevent OS-facing surfaces (status item, OS notifications,
lock-screen summaries, and app-icon badges) from drifting away from the
canonical collaboration and durable-attention records.

## Case shape

Each YAML document:

- references one or more `parity_rows` in `/artifacts/ux/presence_surface_matrix.yaml`;
- binds the case to one or more upstream fixtures (collaboration session/join
  flows, durable attention, or system-affordance cases) rather than embedding
  surface-local copy; and
- includes reviewer checks that assert the “presence ≠ control ≠ recording”
  invariants and the quiet-hours / privacy / reopen behavior.

## Cases

- `live_only_session.yaml`
  — live view-first session; presence projection is status-item only.
- `recorded_session.yaml`
  — recording posture explicitly admitted; lock-screen payload stays generic.
- `follow_only_session.yaml`
  — follow mode shown as view authority only; no implicit control.
- `shared_debug_moment.yaml`
  — debug-lane control grant + revocation; OS shortcuts never mutate.
- `stale_badge_after_session_end.yaml`
  — presence/badge cleanup after session end; no stuck “active” indicators.
- `quiet_hours_deferral.yaml`
  — quiet-hours hold + typed digest release; no replay spam.
- `cross_device_reopen_or_handoff.yaml`
  — click-through reopens exact canonical target (or truthful placeholder).

