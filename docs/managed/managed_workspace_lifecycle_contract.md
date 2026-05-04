# Managed-Workspace Lifecycle, Retry/Offboarding, and Local-Only Continuation Contract

This contract freezes the managed-workspace lifecycle, the
suspend/resume/expiry/rebuild/offboarding consequences, and the
local-only continuation surfaces that survive managed-service loss.
It exists so future shell, notebook, AI, review, and support/export
surfaces share one reviewable lifecycle object instead of inventing
per-vendor "loading" / "paused" / "expired" copy or pretending a
remote environment is opaque to the user.

The contract is normative for product surfaces that render managed-
workspace lifecycle phase, persistence posture, retry posture,
offboarding posture, or local-only continuation. Where this document
disagrees with the source product and architecture specs, the source
specs win and this document must be updated in the same change.
Where a downstream surface invents a conflicting label, this document
wins and that surface is non-conforming.

Companion artifacts:

- [`/schemas/managed/workspace_lifecycle_state.schema.json`](../../schemas/managed/workspace_lifecycle_state.schema.json)
  — boundary schema for one
  `managed_workspace_lifecycle_state_record`, the object every
  surface that quotes managed-workspace state reads before claiming
  reachability, persistence, expiry, rebuild, offboarding, or
  local-only continuation.
- [`/fixtures/managed/workspace_lifecycle_cases/`](../../fixtures/managed/workspace_lifecycle_cases/)
  — worked cases covering first attach, suspend/resume, expired
  workspace, rebuild review, and local-only continuation after a
  managed-service outage.

Inherited contracts:

- [`/docs/verification/target_and_host_boundary_packet.md`](../verification/target_and_host_boundary_packet.md)
  freezes the reviewer-facing managed-workspace lifecycle labels
  (`warming`, `ready`, `degraded`, `paused`, `suspended`, `expired`,
  `local_only_continuation`) and the wrong-target / reapproval
  vocabulary every target-truth record reuses. This contract narrows
  the lifecycle into the 12-phase machine-readable view; it does not
  replace the reviewer-label projection.
- [`/artifacts/runtime/managed_workspace_lifecycle.yaml`](../../artifacts/runtime/managed_workspace_lifecycle.yaml)
  is the canonical lifecycle-state matrix, transition-reason
  vocabulary, activation-budget slice vocabulary, and audit-event
  vocabulary this contract reuses without re-deriving.
- [`/docs/service/managed_service_seed.md`](../service/managed_service_seed.md)
  provides managed-service SLO, degradation, retention, deletion,
  and local-core non-dependence vocabulary. This contract narrows
  managed-workspace lifecycle truth; it does not replace the service
  row.
- [`/docs/admin/org_admin_seat_and_fleet_contract.md`](../admin/org_admin_seat_and_fleet_contract.md)
  defines seat-lifecycle consequences. Seat suspension or
  deprovisioning may force a workspace into `expired` or `closed`
  with a typed reason; this contract names the resulting lifecycle
  view, not the seat policy.
- [`/docs/integrations/provider_account_mapping_and_offline_capture_contract.md`](../integrations/provider_account_mapping_and_offline_capture_contract.md)
  defines offline-capture controls for connected providers. When the
  managed workspace is unavailable, offline-capture surfaces may stay
  available on the local host; this contract names the local-only
  continuation surface set.
- [`/docs/managed/metering_and_usage_export_contract.md`](metering_and_usage_export_contract.md)
  freezes managed metering and usage-export explainability. A
  managed-workspace lifecycle state may be quoted alongside a quota
  state; the two records remain independent.

Normative source anchors:

- `.t2/docs/Aureline_PRD.md` — managed-workspace lifecycle, suspend
  and resume, expiry, rebuild, offboarding, local-core
  non-dependence.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  managed-workspace control plane, capsule and volume drift
  recovery, snapshot / hibernation budgets, retirement drain
  window, kill-switch quarantine.
- `.t2/docs/Aureline_Technical_Design_Document.md` — execution-
  context provenance, managed-workspace lifecycle, remote-agent
  attach, compatibility skew rules.

## Scope

Frozen at this revision:

- the 12-phase reviewer-facing `lifecycle_phase_class`:
  `discovery`, `preflight`, `allocating`, `booting`, `attaching`,
  `sync_warming`, `ready`, `suspended`, `degraded`, `expired`,
  `rebuild`, `closed`;
- the projection from `lifecycle_phase_class` onto the canonical
  `managed_workspace_lifecycle_state` vocabulary in
  [`managed_workspace_lifecycle.yaml`](../../artifacts/runtime/managed_workspace_lifecycle.yaml);
- the `persistence_posture` matrix declaring what persists and what
  is lost across each phase for the seven resource classes
  (`workspace_files`, `in_progress_processes`, `terminal_sessions`,
  `forwarded_ports`, `notebook_kernels`, `credentials_and_tokens`,
  `evidence_artifacts`);
- the `continuation_posture` block naming when the editor stays in
  `workspace_attached`, `workspace_unavailable_local_only_continuation`,
  `offboarding_in_progress`, or `workspace_closed_local_artifacts_only`;
- the typed expiry, rebuild, closed, and degraded blocks naming
  why a phase was entered;
- the local-only-continuation admissible-surface set;
- the offboarding posture for files, evidence, credentials, and
  support bundles across access-end windows.

Out of scope:

- a managed-workspace control plane, provisioner, or runtime;
- raw container images, raw volumes, raw kernel handles, raw URLs,
  raw user emails, raw tenant names, or raw provider account
  identifiers;
- billing or invoicing for workspace minutes (handled by the
  metering contract);
- collaboration session replication semantics.

## Core Rule

A managed workspace is not a black box. It has a reviewable phase, a
typed transition reason, an explicit per-resource persistence posture,
a typed retry posture, and a typed offboarding posture. Every surface
that renders managed-workspace state must read a
`managed_workspace_lifecycle_state_record` and must surface what
survives, what is lost, and what continuation is admissible — without
inventing per-vendor labels.

Local-core workflows remain non-blocking. Loss of the managed
workspace narrows the specific managed action whose work cannot be
performed remotely; it must not block opening, editing, saving,
searching, local Git, local tasks, direct local/BYOK AI, or already-
authorized local automation.

## Lifecycle Phase Vocabulary

The contract freezes 12 phases. Surfaces MAY NOT mint additional
phases or collapse two distinct phases into one chip.

| `lifecycle_phase_class` | Reviewer meaning | Underlying taxonomy state | Reachability |
|---|---|---|---|
| `discovery` | Workspace is referenced but no managed instance exists yet. | `undeclared` | `unreachable` |
| `preflight` | Capsule, policy, and capacity preflight checks are running before allocation. | `undeclared` | `unreachable` |
| `allocating` | Control plane is allocating compute, volumes, and network. | `provisioning` | `warming` |
| `booting` | Compute is up; OS / container is booting. | `provisioning` | `warming` |
| `attaching` | Editor is attaching session, identity, and adapter to the booted instance. | `provisioning` or `warming` | `warming` |
| `sync_warming` | Capsule and prebuild warmers are running; activators applied. | `warming` | `warming` |
| `ready` | Instance is reachable, capsule in sync, activators applied, normal launch target. | `ready` | `reachable` |
| `suspended` | Instance auto-paused or user-paused; filesystem preserved, compute released. | `idle_suspended` or `snapshot_paused` | `unreachable` |
| `degraded` | Control plane detected drift / partition / partial dependency loss; recovery in flight or required. | `recovering` (may overlay `ready` while recovering) | `degraded` or `unreachable` |
| `expired` | Long-idle hibernation, retirement, session-ticket expiry, or access-end window elapsed. | `hibernated`, `retiring`, `retired`, or a `ready` instance whose session ticket has expired | `unreachable` or `unreachable_pending_reauth` |
| `rebuild` | A successor image, unrecoverable drift, admin-requested migration, or user-requested rebuild requires re-allocation; review is admissible before commit. | `provisioning` (from a prior `retiring` / `quarantined` / `recovering`) | `warming` |
| `closed` | Instance retired or deprovisioned; managed binding is gone. Local artifacts and exports remain on the local host. | `retired` or `quarantined` | `unreachable` |

Rules:

1. A surface MAY NOT render `ready` unless the underlying taxonomy
   state is `ready` and the reachability is `reachable`.
2. A surface MAY NOT render `suspended` over `recovering`, `warming`,
   or `expired`; suspended projects only onto `idle_suspended` or
   `snapshot_paused`.
3. A surface MAY NOT render `degraded` on `paused`, `suspended`, or
   `expired`. Degraded MAY overlay `ready` only when the underlying
   taxonomy state is `recovering`.
4. A surface MAY NOT render `expired` without a typed
   `expiry_reason_class`.
5. A surface MAY NOT render `rebuild` without a typed
   `rebuild_reason_class` and a `prior_workspace_instance_ref`.
6. A surface MAY NOT render `closed` without a typed
   `closed_reason_class`.

## Persistence Posture Matrix

Every record carries a `persistence_posture` object naming, for each
of the seven resource classes, what survives the current phase. The
matrix is the contract for what users may rely on; it is not a
rendering hint.

### Resource classes (frozen)

| `resource_class` | Meaning |
|---|---|
| `workspace_files` | The mounted workspace filesystem, including user edits and committed history. |
| `in_progress_processes` | User-launched build, test, run, or long-running tool processes. |
| `terminal_sessions` | Open terminal panes, shell history, and pty state. |
| `forwarded_ports` | Port-forward tunnels and remote-port-bound previews. |
| `notebook_kernels` | Notebook kernel processes and per-kernel in-memory state. |
| `credentials_and_tokens` | Session tickets, managed-control-plane tokens, BYOK provider tokens, and approval tickets. |
| `evidence_artifacts` | Mutation-journal entries, route-truth packets, support bundles, and exported evidence on the workspace volume. |

### `persistence_class` (frozen)

| `persistence_class` | Meaning |
|---|---|
| `persisted_durable` | Survives the current phase and all admissible next phases on the same managed binding. |
| `persisted_snapshot` | Captured in a snapshot or hibernation image; restored on resume from snapshot. |
| `persisted_workspace_only` | Survives within the current managed-workspace volume; lost if the workspace is rebuilt, retired, or closed. |
| `preserved_locally_only` | Held on the local host; survives managed-workspace loss because it never lived inside the workspace. |
| `suspended_resumable` | Process or session is paused and may be resumed if and only if the next phase is `ready` via the resume path. |
| `ephemeral_lost_on_transition` | Destroyed by the current phase transition; not recoverable. |
| `regenerated_on_resume` | Recreated by the editor after the next admissible resume; not the same handle as before. |
| `never_persisted_security_boundary` | Forbidden to persist by the security boundary; treated as lost on every transition by design. |
| `policy_redacted` | Data exists but its content is redacted by policy; presence is disclosed, content is not. |
| `not_applicable` | Resource does not apply in this phase. |

### Required posture per phase (minimum required behavior)

| Phase | `workspace_files` | `in_progress_processes` | `terminal_sessions` | `forwarded_ports` | `notebook_kernels` | `credentials_and_tokens` | `evidence_artifacts` |
|---|---|---|---|---|---|---|---|
| `discovery` | `not_applicable` | `not_applicable` | `not_applicable` | `not_applicable` | `not_applicable` | `not_applicable` | `preserved_locally_only` |
| `preflight` | `not_applicable` | `not_applicable` | `not_applicable` | `not_applicable` | `not_applicable` | `persisted_durable` (preflight ticket) | `preserved_locally_only` |
| `allocating` | `not_applicable` | `not_applicable` | `not_applicable` | `not_applicable` | `not_applicable` | `persisted_durable` | `preserved_locally_only` |
| `booting` | `persisted_workspace_only` | `not_applicable` | `not_applicable` | `not_applicable` | `not_applicable` | `persisted_durable` | `persisted_workspace_only` |
| `attaching` | `persisted_workspace_only` | `not_applicable` | `not_applicable` | `not_applicable` | `not_applicable` | `persisted_durable` | `persisted_workspace_only` |
| `sync_warming` | `persisted_workspace_only` | `not_applicable` | `not_applicable` | `not_applicable` | `not_applicable` | `persisted_durable` | `persisted_workspace_only` |
| `ready` | `persisted_workspace_only` or `persisted_durable` | `persisted_workspace_only` | `persisted_workspace_only` | `persisted_workspace_only` | `persisted_workspace_only` | `persisted_durable` | `persisted_workspace_only` or `persisted_durable` |
| `suspended` | `persisted_workspace_only` or `persisted_snapshot` | `suspended_resumable` or `ephemeral_lost_on_transition` | `suspended_resumable` or `ephemeral_lost_on_transition` | `ephemeral_lost_on_transition` | `regenerated_on_resume` | `persisted_durable` (session ticket may need refresh) | `persisted_workspace_only` |
| `degraded` | `persisted_workspace_only` | `suspended_resumable` or `ephemeral_lost_on_transition` | `suspended_resumable` or `ephemeral_lost_on_transition` | `ephemeral_lost_on_transition` | `regenerated_on_resume` | `persisted_durable` | `persisted_workspace_only` |
| `expired` | `persisted_snapshot` or `persisted_workspace_only` | `ephemeral_lost_on_transition` | `ephemeral_lost_on_transition` | `ephemeral_lost_on_transition` | `ephemeral_lost_on_transition` | `never_persisted_security_boundary` (session ticket) or `policy_redacted` | `persisted_workspace_only` |
| `rebuild` | `persisted_durable` (committed history) and `ephemeral_lost_on_transition` (uncommitted scratch) | `ephemeral_lost_on_transition` | `ephemeral_lost_on_transition` | `ephemeral_lost_on_transition` | `ephemeral_lost_on_transition` | `persisted_durable` (managed-control-plane token) | `persisted_durable` (committed evidence) and `ephemeral_lost_on_transition` (uncommitted) |
| `closed` | `preserved_locally_only` (already-cloned/exported) | `ephemeral_lost_on_transition` | `ephemeral_lost_on_transition` | `ephemeral_lost_on_transition` | `ephemeral_lost_on_transition` | `never_persisted_security_boundary` | `preserved_locally_only` (already-exported) |

Rule: a surface MAY NOT claim a stronger persistence class than the
matrix admits for the current phase. A surface MAY claim a weaker
class (e.g., `persisted_workspace_only` instead of `persisted_durable`)
when the underlying provider does not back the stronger guarantee.

Rule: `credentials_and_tokens` rendering MUST disclose redaction. A
record MAY NOT render a credential body; it quotes a token ref and
its `credential_redaction_class`.

Rule: `evidence_artifacts` rendering MUST disclose
`evidence_export_posture_class` so a support / review surface can
tell whether the evidence is reachable from the local host or only
from the managed workspace.

## Continuation Posture

Every record carries a `continuation_posture` block. The block names
which continuation the editor is in.

### `posture_class` (frozen)

| Token | Meaning |
|---|---|
| `workspace_attached` | The editor is attached to the managed workspace and managed actions are admissible. |
| `workspace_unavailable_local_only_continuation` | The managed workspace is unreachable, suspended, expired, or policy-blocked and the editor is continuing locally with a narrowed scope. |
| `offboarding_in_progress` | The managed binding is closing; offboarding exports are admissible. |
| `workspace_closed_local_artifacts_only` | The managed binding is gone; only local artifacts and already-exported bundles remain. |

### `local_only_continuation_reason_class` (frozen)

Used only when
`posture_class = workspace_unavailable_local_only_continuation`:

- `managed_control_plane_unreachable`
- `remote_agent_attach_unreachable`
- `browser_handoff_return_unavailable`
- `route_dependency_unreachable`
- `session_ticket_expired`
- `workspace_suspended_user_continuing_locally`
- `workspace_expired_user_continuing_locally`
- `workspace_rebuild_in_progress`
- `policy_blocked_managed_action`
- `user_requested_local_fallback`
- `admin_requested_local_fallback`

### `local_only_admissible_surface_class` (frozen)

These surfaces MUST stay safe and available whenever the posture is
`workspace_unavailable_local_only_continuation`. A surface that
blocks any of these on managed-workspace loss is non-conforming.

| Token | Meaning |
|---|---|
| `file_open_and_edit` | Open and edit local files. |
| `file_save_to_local_disk` | Save edits to the local disk. |
| `local_search` | Search the local working tree. |
| `local_git` | Local Git read/write, commits, branches, diffs. |
| `local_tasks` | Already-authorized local tasks (build, test, run) that do not require the managed workspace. |
| `local_byok_ai` | Direct local or BYOK AI routes whose budget policy admits the current quota state. |
| `local_authorized_automation` | Already-authorized automation that does not cross the managed boundary. |
| `support_bundle_export_local_only` | Export a support bundle from local evidence only. |
| `offboarding_export_local_only` | Export already-prepared offboarding artifacts that live on the local host. |
| `already_exported_local_artifacts` | Read and re-export artifacts that have already crossed onto the local host. |

### `managed_action_blocking_class` (frozen)

| Token | Meaning |
|---|---|
| `managed_actions_admissible` | Workspace is attached and managed actions are admissible. |
| `managed_actions_narrowed_recovery_in_flight` | Workspace is recovering; only narrowed managed actions are admissible. |
| `managed_actions_blocked_workspace_unavailable` | Managed actions blocked until workspace is reachable. |
| `managed_actions_blocked_pending_reauth` | Managed actions blocked until session-ticket / approval reauth completes. |
| `managed_actions_blocked_policy_suppressed` | Managed actions blocked by policy; suppression must render. |
| `managed_actions_blocked_expired` | Managed actions blocked because the workspace expired. |
| `managed_actions_blocked_rebuild_review_required` | Managed actions blocked until the rebuild review completes. |
| `managed_actions_closed` | Managed binding is gone; managed actions are no longer offered. |

Rules:

1. A record whose phase is `ready` MUST set `posture_class` to
   `workspace_attached` and `managed_action_blocking_class` to
   `managed_actions_admissible` or
   `managed_actions_narrowed_recovery_in_flight`.
2. A record whose phase is `expired`, `rebuild`, or `closed` MAY NOT
   set `posture_class = workspace_attached`.
3. A record whose `posture_class` is
   `workspace_unavailable_local_only_continuation` MUST carry a
   non-null `local_only_continuation_reason_class` and a
   non-empty `local_only_admissible_surfaces` list.
4. A record whose `posture_class` is
   `workspace_closed_local_artifacts_only` MUST set
   `managed_action_blocking_class = managed_actions_closed`.

## Retry Posture

Resume, rebuild, reconnect, and local-only continuation are distinct
outcomes. A surface MAY NOT collapse them into a generic "try again"
button. The contract names every typed retry outcome.

### `retry_outcome_class` (frozen)

| Token | When admissible | Required disclosure |
|---|---|---|
| `resume_admissible` | Phase is `suspended`; underlying state is `idle_suspended` or `snapshot_paused`. | Resume is offered; persistence posture for processes/terminals/kernels is rendered before commit. |
| `reconnect_admissible` | Phase is `degraded`; underlying state is `recovering`. | Reconnect is offered; recovery reason and admissible managed actions are rendered. |
| `reauth_admissible` | Phase is `expired` and `expiry_reason_class = session_ticket_expired`. | Reauth is offered; user must complete reauth before managed actions resume. |
| `rebuild_review_required` | Phase is `expired` (`hibernation_window_elapsed`, `successor_image_available`, `policy_epoch_rolled`) or phase is `rebuild`. | Rebuild review is required; user MUST acknowledge what is lost vs. preserved before commit. |
| `local_only_continuation_admissible` | Workspace is unavailable but the editor admits local-only continuation. | Local-only admissible surfaces are rendered; managed actions are blocked with a typed reason. |
| `offboarding_admissible` | Posture is `offboarding_in_progress`. | Offboarding export is offered with the access-end window. |
| `closed_no_action` | Phase is `closed`. | No managed action is offered; only local artifacts and already-exported bundles remain. |

Rules:

1. A record MUST carry one or more `retry_outcome_class` values
   admissible from the current phase.
2. `rebuild_review_required` MAY NOT be combined with
   `resume_admissible`. Rebuild and resume are distinct outcomes
   because rebuild discards `in_progress_processes`,
   `terminal_sessions`, and uncommitted `workspace_files` scratch.
3. `reauth_admissible` MAY combine with `local_only_continuation_admissible`
   when the editor stays usable locally during reauth.
4. `closed_no_action` is exclusive; no other outcome is admissible.

## Expiry, Rebuild, Closed, and Degraded Blocks

The schema requires a typed block on every non-`workspace_attached`
phase so the user can tell why the phase was entered.

### `expiry_reason_class` (frozen)

Required when phase is `expired`:

- `session_ticket_expired`
- `hibernation_window_elapsed`
- `retirement_drain_window_completed`
- `policy_epoch_rolled`
- `kill_switch_tripped`
- `successor_image_available`
- `access_end_window_expired`

### `rebuild_reason_class` (frozen)

Required when phase is `rebuild`:

- `successor_image_available`
- `capsule_drift_unrecoverable`
- `volume_drift_unrecoverable`
- `admin_requested_migration`
- `policy_epoch_rolled`
- `kill_switch_recovery`
- `user_requested_rebuild`

### `closed_reason_class` (frozen)

Required when phase is `closed`:

- `user_requested_retire`
- `admin_requested_quarantine`
- `retirement_drain_window_completed`
- `access_end_window_expired`
- `offboarding_completed`
- `workspace_deleted_by_admin`
- `seat_revoked`

### `degraded_reason_class` (frozen)

Required when phase is `degraded`:

- `control_plane_failure`
- `capsule_drift_detected`
- `volume_drift_detected`
- `network_partition_detected`
- `partial_route_dependency_unreachable`
- `session_ticket_expiring_soon`

## Offboarding Posture

Every record carries an `offboarding_posture` block declaring what
remains accessible across the access-end window for the four
offboarding-relevant resource classes.

### `offboarding_persistence_class` (frozen)

| Token | Meaning |
|---|---|
| `exported_before_close` | Already exported off the managed surface. |
| `export_available_during_access_end_window` | Export available without a support ticket while the access-end window is open. |
| `preserved_locally_after_close` | Lives on the local host; survives the close. |
| `discarded_at_close` | Discarded at the close transition by design. |
| `policy_held_post_close` | Held by policy after close; subject to retention rules. |
| `not_applicable` | Resource not applicable for this offboarding view. |

Rules:

1. A record whose phase is `closed` MUST carry an
   `offboarding_persistence_class` for `workspace_files`,
   `evidence_artifacts`, `credentials_and_tokens`, and
   `support_bundles`. Hidden / unspecified offboarding posture is
   non-conforming.
2. A managed customer MUST be able to obtain the promised offboarding
   export during the access-end window without a support ticket
   (per the metering contract). If the export is unavailable,
   `offboarding_persistence_class = export_available_during_access_end_window`
   MAY NOT be claimed for that resource.
3. After access ends, local-core workflows and already-exported
   local files remain user-controlled. Managed aggregate retention
   follows the record-class and retention-row policies.

## UI And Support Rules

### Editor and shell surfaces

- The lifecycle phase chip MUST quote the record by id; surfaces
  MAY NOT re-derive the phase from ambient signals.
- The persistence posture MUST be discoverable before the user
  commits to a transition (resume, rebuild, close).
- Local-only continuation surfaces MUST stay reachable when the
  posture is `workspace_unavailable_local_only_continuation`.

### AI surfaces

- AI route selection MUST quote the lifecycle record alongside the
  quota state. AI routes that require the managed workspace MUST
  honor `managed_action_blocking_class`.
- Local/BYOK AI routes MUST stay admissible during
  `workspace_unavailable_local_only_continuation` if the route's
  budget policy admits the current quota state.

### Notebook and kernel surfaces

- Notebook kernels MUST render `regenerated_on_resume` truthfully;
  surfaces MAY NOT pretend kernel handles survive suspension.
- A notebook surface running inside a managed workspace MUST
  preserve the notebook-kernel boundary cue alongside the managed-
  workspace boundary cue (per the verification packet).

### Review and support surfaces

- Support packets quote `workspace_lifecycle_record_id`, not free-
  text "workspace down" copy.
- Support copy MUST disclose the phase, persistence posture, and
  retry outcomes the user is being offered.
- Support MUST NOT claim a workspace is "available" when the record
  says `unreachable`.

### Offboarding and export paths

- Offboarding packets MUST quote the lifecycle record alongside the
  metering record. The two records remain independent.
- Closed workspaces MUST render `closed_reason_class`. A closed
  workspace MAY NOT render as "available later" without a typed
  rebuild path.

## Forbidden Patterns

The following are non-conforming:

- rendering `ready` over a `recovering`, `warming`, `idle_suspended`,
  or `expired` underlying state;
- rendering a generic "loading" or "paused" chip when the contract
  admits a typed phase;
- claiming `persisted_durable` for `in_progress_processes`,
  `terminal_sessions`, `forwarded_ports`, or `notebook_kernels` —
  these are never durable across phase transitions;
- claiming `credentials_and_tokens` are persisted in plain text
  inside the workspace volume;
- collapsing `resume`, `rebuild`, `reconnect`, and `local-only
  continuation` into one "try again" button;
- blocking local edit / save / search / Git / tasks when the managed
  workspace is unavailable;
- omitting `expiry_reason_class`, `rebuild_reason_class`,
  `closed_reason_class`, or `degraded_reason_class` on the matching
  phase;
- rendering an offboarding export path that requires a support
  ticket during the access-end window;
- using raw user emails, raw tenant names, raw workspace volume ids,
  raw provider URLs, or raw container image digests in the
  lifecycle payload.

## Evolution Rules

- Adding a new `lifecycle_phase_class`, `persistence_class`,
  `retry_outcome_class`, `expiry_reason_class`, `rebuild_reason_class`,
  `closed_reason_class`, `degraded_reason_class`,
  `local_only_continuation_reason_class`,
  `local_only_admissible_surface_class`,
  `managed_action_blocking_class`, or
  `offboarding_persistence_class` is additive-minor and requires a
  schema_version bump and at least one fixture.
- Repurposing an existing class is breaking and requires a new
  governance decision row plus a migration note for support / export
  consumers.
- Any new managed-workspace surface must cite this contract, the
  schema, and at least one fixture before it may claim lifecycle,
  persistence, retry, offboarding, or local-only continuation
  behavior.
- The lifecycle record is independent of metering, target-truth, and
  seat-lifecycle records; surfaces that quote multiple records MUST
  preserve each record id and not collapse them into a single chip.
