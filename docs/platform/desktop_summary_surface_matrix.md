# Desktop summary-surface matrix (dock/taskbar progress, badges, recent items, and jump-list-style actions)

This document freezes Aureline’s desktop **summary surfaces** — OS-managed
surfaces outside the app window that summarize activity and provide a bounded
reopen path. The goal is to prevent platform integrations (Dock, taskbar,
jump lists, launcher recents) from drifting into platform-specific
last-writer-wins ownership, ambiguous reopen targets, or hidden mutation paths.

If this document and
[`/artifacts/platform/desktop_summary_surface_matrix.yaml`](../../artifacts/platform/desktop_summary_surface_matrix.yaml)
ever disagree, the YAML wins for tooling and this document must be updated in
the same change.

Companion contracts and artifacts:

- [`/artifacts/platform/desktop_summary_surface_matrix.yaml`](../../artifacts/platform/desktop_summary_surface_matrix.yaml)
  — machine-readable matrix and vocabulary re-exports.
- [`/docs/ux/desktop_affordance_contract.md`](../ux/desktop_affordance_contract.md)
  — OS entry and reopen invariants (no authority widening, review before
  boundary change, fallback preserves intent, handler ownership is visible).
- [`/artifacts/platform/system_affordance_route_audit.md`](../../artifacts/platform/system_affordance_route_audit.md)
  — canonical routing for `dock_taskbar_recent` and `dock_taskbar_jump_action`.
- [`/artifacts/platform/file_association_ownership_matrix.yaml`](../../artifacts/platform/file_association_ownership_matrix.yaml)
  — side-by-side and portable ownership rules for recent-item registration and
  dock/taskbar reopen, including exact-target reopen rules.
- [`/docs/ux/durable_job_envelope_contract.md`](../ux/durable_job_envelope_contract.md)
  and
  [`/artifacts/ux/badge_class_review.yaml`](../../artifacts/ux/badge_class_review.yaml)
  — durable-job/progress grammar and badge-source partition rules.
- [`/docs/ux/os_notification_and_quiet_hours_contract.md`](../ux/os_notification_and_quiet_hours_contract.md)
  and
  [`/schemas/ux/notification_suppression_record.schema.json`](../../schemas/ux/notification_suppression_record.schema.json)
  — `desktop_summary_affordance_record` contract for dock/taskbar progress and
  badge mirrors (read-only, exact-target reopen, forbidden shortcut actions).
- [`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md)
  and
  [`/schemas/workspace/entry_and_restore_result.schema.json`](../../schemas/workspace/entry_and_restore_result.schema.json)
  — `recent_work_entry_record` fields used by Start Center, workspace switcher,
  OS jump lists, and CLI recents.
- [`/schemas/platform/deep_link_intent.schema.json`](../../schemas/platform/deep_link_intent.schema.json)
  — `deep_link_intent_record` fields used to carry handler ownership, target
  availability/freshness, and replay/fallback posture for OS entry paths.
- [`/artifacts/platform/claimed_desktop_profiles.yaml`](../../artifacts/platform/claimed_desktop_profiles.yaml)
  and
  [`/docs/platform/desktop_platform_conformance_matrix.md`](./desktop_platform_conformance_matrix.md)
  — the claimed desktop profiles this matrix applies to.
- [`/fixtures/platform/desktop_summary_surface_cases/`](../../fixtures/platform/desktop_summary_surface_cases/)
  — reviewer-side worked cases binding platform surfaces to the upstream
  contracts above.

Normative source anchors projected here:

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` “Native desktop integration, deep-link,
  and system-affordance contract” and the “Notifications / badges” and
  “Recent work” sections.
- `.t2/docs/Aureline_Technical_Design_Document.md` “Notification, attention, and
  activity-center architecture” (including “Badge / companion summary”).

## 1. Summary surfaces in scope

This matrix covers four OS-facing summary surface classes:

1. **Progress mirror** (`dock_taskbar_progress`)
   - A read-only mirror of a durable job’s phase/progress.
   - Modeled as `desktop_summary_affordance_record` with
     `affordance_class = dock_taskbar_progress`.
2. **Badge mirror** (`dock_taskbar_badge`)
   - A read-only mirror of badge-class counts derived from durable objects.
   - Modeled as `desktop_summary_affordance_record` with
     `affordance_class = dock_taskbar_badge`.
3. **Recent-item reopen** (`dock_taskbar_recent`)
   - An OS recent entry that reopens a specific workspace/recent-work target.
   - Modeled as `deep_link_intent_record` with
     `source_surface_class = dock_taskbar_recent` and a
     `recent_work_entry_record` identity as the target anchor.
4. **Jump-list-style action** (`dock_taskbar_jump_action`)
   - An OS “quick action” or jump-list task that reopens a bounded in-product
     surface (e.g., Start Center) and never completes a mutation.
   - Modeled as `deep_link_intent_record` with
     `source_surface_class = dock_taskbar_jump_action`.

The platform-specific primitives differ (Dock tile vs taskbar button vs launcher
integration), but the **contract is platform-neutral** and must not fork by OS.

## 2. Cross-surface invariants (non-negotiable)

### 2.1 Summary surfaces are mirrors, not truth

- **Badges and progress indicators are never authoritative state.**
  They are mirrors derived from durable objects (durable jobs, attention items,
  activity rows) and are always traceable back to a canonical object and route.
- A summary surface that maintains a private counter, progress loop, or “last
  message wins” state is non-conforming.

### 2.2 Exact-target reopen or fail closed

Activation from a summary surface MUST:

- resolve to the canonical object advertised by the surface, through a canonical
  command ID (route audit), **or**
- degrade explicitly (placeholder, locate/reconnect, denial) while preserving
  the original intent and disclosing why exact open is unavailable.

Opening a generic empty shell, silently opening “some workspace”, or silently
changing target kind is non-conforming.

### 2.3 No authority widening from OS surfaces

Summary surfaces may not widen:

- trust posture (trusted ↔ restricted),
- policy authority,
- profile/tenant scope,
- collaboration presence/role, or
- remote authority,

without routing through an in-product review surface that discloses the boundary
change.

### 2.4 Ownership is visible; last-writer-wins is forbidden

Side-by-side channels and portable installs must not fight over:

- recent-item registration,
- jump-list tasks,
- dock/taskbar reopen targets, or
- badge/progress ownership.

Every summary surface entry path must disclose `owning_channel_ref` and
`owner_build_ref` (handler ownership), and any ownership change must be
reviewable before it takes effect on claim-bearing rows.

Portable installs must not write machine-global shell state.

### 2.5 Summary-only surfaces are inspect-only

Jump-list actions, dock-menu actions, and summary-surface shortcuts must not
directly complete:

- destructive writes,
- privileged step-up actions,
- policy overrides,
- trust changes,
- provider grant changes, or
- cross-workspace mutations.

When a surface needs to lead to one of those outcomes, it must route to the
in-product review/approval surface instead of attempting the mutation from the
OS.

## 3. Recent-item object fields (required)

An OS recent entry (or jump-list “recent destination”) is a projection of one
`recent_work_entry_record` plus the OS entry boundary record.

Every recent-item summary surface MUST preserve:

- **Object identity**
  - `recent_work_id` (stable opaque id),
  - `target_kind` and one of:
    - `filesystem_identity_ref` (local targets),
    - `remote_target_descriptor_ref` (remote targets), or
    - `artifact_descriptor_ref` (handoff/template/import targets).
- **Channel/build owner**
  - `handler_ownership.owning_channel_ref` and `handler_ownership.owner_build_ref`
    on the OS-entry `deep_link_intent_record`.
- **Stale/unavailable marker**
  - `recent_work_entry_record.target_state` (user-facing state chip), and
  - `deep_link_intent_record.target_identity.availability_class` +
    `freshness_class` (OS-entry resolver truth).
- **Trust and restore posture**
  - `recent_work_entry_record.trust_state` and `restore_availability`.
- **Exact-target reopen rule**
  - activation resolves through `cmd:start_center.open_recent` (or an equivalent
    canonical open command) and must not substitute a different target kind or a
    different trust posture.
- **Forbidden action classes**
  - mutating/privileged classes are forbidden from summary-only surfaces; recent
    entries are open/reopen only and must route through in-product review when
    the open would widen authority or when handler ownership changes.

## 4. Platform surface matrix (conceptual)

This section is a narrative projection of the machine-readable matrix. Tooling
and reviewers should treat the YAML as the authoritative roster of rows and
surface dispositions.

### 4.1 macOS (Dock tile + Dock menu)

Summary surfaces:

- Progress mirror: supported on the Dock tile (`dock_taskbar_progress`).
- Badge mirror: supported on the Dock tile (`dock_taskbar_badge`).
- Recent-item reopen: supported via Dock recent entries
  (`dock_taskbar_recent`).
- Jump-list-style actions: supported via Dock menu quick actions
  (`dock_taskbar_jump_action`), limited to safe reopen/inspect actions.

Known parity gaps to disclose:

- Dock menus and recent items are OS-managed surfaces; stale entries can persist
  across restarts. Entries must carry availability/freshness and fail closed
  rather than silently reopening the wrong target.

### 4.2 Windows (taskbar button + jump list)

Summary surfaces:

- Progress mirror: supported on the taskbar button (`dock_taskbar_progress`),
  including indeterminate and error/paused presentations where applicable.
- Badge mirror: supported as a taskbar “badge-style” indicator
  (`dock_taskbar_badge`), noting that the platform primitive may be an overlay
  indicator rather than a numeric bubble in some packaging modes.
- Recent-item reopen: supported via jump-list recent destinations
  (`dock_taskbar_recent`).
- Jump-list-style actions: supported via jump-list tasks
  (`dock_taskbar_jump_action`), limited to safe reopen/inspect actions.

Known parity gaps to disclose:

- Jump-list tasks and recent destinations can outlive the running process; the
  last-known-good list may be shown while the app is not running. Entries must
  therefore be safe, stable, and route through the canonical review/open path
  when stale or ownership has changed.

### 4.3 Linux (claimed GNOME profiles; launcher + notification equivalents)

Summary surfaces:

- Recent-item reopen: supported only where the claimed profile’s launcher /
  desktop stack provides a stable recent-entry mechanism; otherwise treated as
  best-effort with explicit disclosure when unavailable.
- Progress and badge mirrors: treated as best-effort and **not claim-bearing**
  on the GNOME rows unless the named profile row explicitly calls out a stable
  primitive. When unavailable, Aureline must rely on in-product surfaces and OS
  notifications without implying dock/taskbar parity.

Known parity gaps to disclose:

- Desktop-environment differences can remove dock badges, dock progress, and
  jump-list equivalents entirely. The product must not imply parity; surfaces
  must disclose when an affordance is unavailable or degraded on the current
  claimed profile.

## 5. Change control

Adding or widening a desktop summary surface requires updating, in the same
change:

- the machine-readable matrix in
  [`/artifacts/platform/desktop_summary_surface_matrix.yaml`](../../artifacts/platform/desktop_summary_surface_matrix.yaml),
- the reviewer fixtures under
  [`/fixtures/platform/desktop_summary_surface_cases/`](../../fixtures/platform/desktop_summary_surface_cases/),
- and any affected upstream contract references (route audit, durable-job and
  badge contracts, notification suppression, and recent-work object model).

Repurposing an existing row (changing its meaning) is breaking and requires an
explicit governance decision row in the normal decision index path.
