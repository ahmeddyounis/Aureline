# Native collaboration presence → OS badge/notification parity (and quiet-hours translation)

Collaboration presence, follow state, presenter stance, recording posture, and
shared-control grants are **truth cues**. When those cues are projected into
OS-facing surfaces (app-icon badges, OS notifications, lock-screen summaries,
and status-item mirrors), they MUST remain aligned with the canonical in-product
records that own the truth.

This document is normative. Where it disagrees with `.t2/docs/` (especially the
UI/UX spec’s notification routing and OS-conventions sections), the upstream
spec wins and this document plus its matrix and cases MUST be updated in the
same change.

Machine-readable companions:

- [`/artifacts/ux/presence_surface_matrix.yaml`](../../artifacts/ux/presence_surface_matrix.yaml)
  — mapping rows that bind collaboration presence + authority signals to OS
  surfaces, quiet-hours translation, and privacy payload classes.
- [`/fixtures/ux/presence_badge_notification_cases/`](../../fixtures/ux/presence_badge_notification_cases/)
  — parity cases covering live-only sessions, recording-on sessions, follow-only
  sessions, shared debug grants, stale-badge cleanup, quiet-hours deferral, and
  cross-device reopen/handoff.

This audit composes with (and does not replace):

- [`/docs/ux/desktop_affordance_contract.md`](./desktop_affordance_contract.md)
  — global rule: OS notifications/badges are projections of durable attention;
  collaboration presence notifications must name freshness + authority limits.
- [`/docs/ux/notification_delivery_contract.md`](./notification_delivery_contract.md) and
  [`/schemas/ux/event_lineage.schema.json`](../../schemas/ux/event_lineage.schema.json)
  — frozen event classes (incl. `collaboration_or_human_request`), dedupe rules,
  badge classes, and required carry-over fields for OS delivery.
- [`/docs/ux/os_notification_and_quiet_hours_contract.md`](./os_notification_and_quiet_hours_contract.md) and
  [`/schemas/ux/notification_suppression_record.schema.json`](../../schemas/ux/notification_suppression_record.schema.json)
  — suppression audit, quiet-hours holds/releases, privacy payload rules, and
  no-bypass shortcut constraints for OS surfaces.
- Collaboration truth owners:
  - [`/docs/collaboration/session_authority_contract.md`](../collaboration/session_authority_contract.md)
    + [`/schemas/collaboration/session_state.schema.json`](../../schemas/collaboration/session_state.schema.json)
  - [`/docs/collaboration/shared_control_contract.md`](../collaboration/shared_control_contract.md)
    + [`/schemas/collaboration/control_grant.schema.json`](../../schemas/collaboration/control_grant.schema.json)
  - [`/docs/collaboration/recording_retention_delete_contract.md`](../collaboration/recording_retention_delete_contract.md)
    + [`/schemas/collaboration/recorded_artifact_row.schema.json`](../../schemas/collaboration/recorded_artifact_row.schema.json)
  - Follow/presenter view-authority (explicitly not control):
    [`/schemas/collaboration/follow_and_presenter_state.schema.json`](../../schemas/collaboration/follow_and_presenter_state.schema.json)

Upstream `.t2/docs/` anchors that govern this parity:

- `.t2/docs/Aureline_UI_UX_Spec_Document.md#6.7` (platform conventions; OS notifications/badges; quiet-hours)
- `.t2/docs/Aureline_UI_UX_Spec_Document.md#9.3` (notification routing; collaboration events; suppress-to-digest)
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md#12.7` (native platform behavior; notifications/badges)

## Scope

This parity audit governs how the following collaboration-adjacent facts are
allowed to project into OS-facing surfaces:

1. **Session presence & freshness** (live, degraded, ended; cached vs stale).
2. **Follow mode & presenter stance** (view authority only).
3. **Recording posture** (explicit enablement; retention/visibility scope).
4. **Shared-control grants** (terminal/debug lanes; explicit, revocable grants).
5. **Unread attention & durable activity rows** (mentions, session requests,
   review requests) that may increment app-icon badges.

Out of scope: implementing a collaboration relay, platform notification
adapters, OS-specific iconography, or per-OS delivery APIs. This audit freezes
**semantics**, **field sources**, **quiet-hours translation**, and **privacy
posture** so future platform adapters cannot invent surface-local truth.

## Canonical source-of-truth records (no surface-local state)

OS-facing surfaces MUST project from the same canonical records that in-product
surfaces already use:

- **Presence truth**: `collaboration_session_record` and its transitions/downgrades
  (`schemas/collaboration/session_state.schema.json`).
- **Follow/presenter truth**: `follow_target_record` and `presenter_state_record`
  (`schemas/collaboration/follow_and_presenter_state.schema.json`).
- **Shared-control truth**: `control_grant_record` / `control_grant_revocation_record`
  (`schemas/collaboration/control_grant.schema.json`).
- **Recording/retention truth**: `collaboration_recorded_artifact_row_record`
  (`schemas/collaboration/recorded_artifact_row.schema.json`).
- **OS notification/badge truth**: durable attention lineage and suppression
  records (`schemas/ux/activity_event_envelope.schema.json`,
  `schemas/ux/event_lineage.schema.json`,
  `schemas/ux/notification_suppression_record.schema.json`).

If an OS surface cannot resolve the canonical record(s), it MUST degrade to a
truthful placeholder (with freshness + recovery actions) rather than minting a
surface-local “best effort” state.

## Core invariants (what must stay visibly true)

1. **Presence never implies control.**
   - Follow/presenter/presence cues are view authority only.
   - Shared terminal/debug control is admitted only by `control_grant_record`
     rows and is never inferred from presence, follow, or presenter state.
2. **Presence never implies recording.**
   - Recording/transcript/replay posture is admitted only by recorded-artifact
     rows + session policy manifests; an OS projection must not imply capture
     unless that posture is explicitly admitted.
3. **Badges are counts over durable objects, not “presence.”**
   - App-icon badge increments are always per-class counts that resolve to
     durable attention items (e.g. `mentions`, `session_requests`, `needs_review`)
     and clear only when those durable items clear.
4. **Quiet-hours and suppression are cross-surface, auditable, and reversible.**
   - If a collaboration event is suppressed (quiet hours / DND / focus /
     presentation / privacy mode / admin suppression / user mute), the durable
     record and suppression record still exist and can release as a typed digest
     on mode exit.
5. **Lock-screen payloads are narrower than in-product payloads.**
   - Lock-screen summaries default to `lock_screen_safe_generic` unless policy
     explicitly allows scoped identity; click-through still reopens the exact
     canonical target via event lineage.

## Projection rules by OS surface

### Status-item / tray / menu-bar presence indicator

The collaboration status item is the primary native “presence” projection.

Rules:

- It renders only **session + follow/presenter + authority limits**:
  - session state + freshness (live / degraded / stale / ended), and
  - follow/presenter stance as view authority only, and
  - explicit control-grant/recording cues **only when those records exist**.
- It MUST NOT:
  - imply write authority, shared control, or recording based on presence alone;
  - outlive the session record (no “stuck active” presence after session end);
  - claim live participation when freshness is `warm_cached`, `stale`, or
    `unverified`.

### OS notifications

Collaboration-origin notifications are permitted only when the event is a
collaboration/human-request durable attention lineage (see the
`collaboration_or_human_request` routing row).

Rules:

- The notification payload is a projection of an `activity_event_envelope`
  lineage: it carries a canonical event id and reopens to the canonical object
  (or a truthful placeholder) rather than opening a generic home screen.
- Notification copy uses reviewable, privacy-safe labels derived from the owning
  canonical records; OS adapters do not invent parallel prose.
- Notification actions are bounded to reopen/inspect/acknowledge; any
  consequence-bearing or authority-widening action routes through an in-product
  review/approval surface.

### App-icon badges (counts)

App-icon badges are **counts**, not presence indicators.

Rules:

- Badge counts derive from durable attention rows and the frozen badge classes
  (see `artifacts/ux/badge_class_review.yaml`).
- Presence-only changes do not increment an app-icon badge.
- A badge that persists after its backing durable items clear is non-conforming.

### Lock-screen summaries

Lock-screen payloads are narrower and default to generic summaries.

Rules:

- Default privacy payload class for collaboration presence or session events is
  `lock_screen_safe_generic` unless policy explicitly allows scoped identity.
- Even when lock-screen payload is generic, click-through resolves through the
  canonical event lineage to the exact object (or a truthful placeholder).

## Parity cases

The fixture corpus under `fixtures/ux/presence_badge_notification_cases/`
exercises:

- live-only session presence projection (no recording, no control);
- session with explicit recording admitted;
- follow-only (view authority) session posture without implying control;
- shared debug control grant moment (explicit grant + revocation; no inference);
- stale badge/presence cleanup after session end;
- quiet-hours suppression + release as digest on mode exit; and
- cross-device reopen/handoff semantics for collaboration presence notifications.

## Acceptance checklist

A surface set conforms when:

1. OS badges/notifications/status mirrors remain projections of canonical
   collaboration + durable-attention records (no surface-local state).
2. Presence projection never implies control, recording, or write authority
   beyond what the canonical records admit.
3. Quiet-hours, DND, privacy mode, and admin suppression are handled via the
   same suppression vocabulary and produce auditable held/release behavior.
4. Lock-screen payloads degrade to privacy-safe summaries by default while
   preserving exact reopen via canonical lineage.

