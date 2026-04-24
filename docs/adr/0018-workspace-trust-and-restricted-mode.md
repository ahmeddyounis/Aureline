# ADR 0018 — Workspace trust, restricted mode, and permission-propagation ADR seed

- **Decision id:** D-0023 (see `artifacts/governance/decision_index.yaml#D-0023`)
- **Status:** Proposed — this is an ADR seed. The trust-state vocabulary, permission-propagation matrix, trust-decision packet shape, entry-flow transitions, remembered-decision scopes, escalation cues, and audit-event id set named below reserve the fields every later surface must honour so later tasks, terminal, debug, notebook, AI, extension-activation, provider-handoff, and remote-attach lanes cannot invent their own trust-state conventions. Full freeze lands in a successor ADR once the open questions in §Open questions close.
- **Decision date:** pending
- **Freeze deadline:** 2026-09-01
- **Owner:** `@ahmeddyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** security_trust_review
- **Related requirement ids:** none

## Context

Workspace trust is the boundary between "open a folder and edit text" and
"run code the author wrote." If that boundary becomes a surface-local
convention — a warning banner in one place, a task-runner check in
another, an ad hoc guard on a notebook kernel, a separate promise in
the AI apply flow — the product accumulates a dozen partial trust
contracts whose union is weaker than any single one of them claims.
Every lane that can mutate disk state, launch a process, open a
network connection on the author's behalf, or apply AI-generated code
needs to read the **same trust posture** and observe the **same
propagation rule**.

ADR-0001 already froze the identity-mode envelope and the binary
`trusted` / `restricted` workspace-trust posture (D-0009). What it did
not freeze is:

- the **finer states** the product surfaces must distinguish beyond
  the binary (untrusted-unknown on first open, policy-degraded after
  a managed bundle narrowed the trusted posture, recovery-ladder
  restricted after a crash-loop safe-mode boot, extension-quarantine
  restricted, session-only temporary grant);
- the **permission-propagation matrix** naming, per surface, whether
  the action is allowed, read-only, degraded-with-preview-only,
  blocked-pending-trust, approval-required-per-invocation, or
  policy-denied — and how those columns shift across `trusted`,
  `restricted`, `policy_degraded`, and recovery-ladder states;
- the **entry-flow transitions** (`open_in_restricted_mode`,
  `continue_in_restricted_mode`, `open_without_restore`,
  `safe_mode_workspace_restricted`) that separate *blocked setup*
  from *optional recommendations* during open / restore;
- the **remembered-decision scope** vocabulary (session-only,
  per-workspace-per-user, per-parent-directory-per-user,
  admin-policy-scope, never-remembered) so reopen and "trust all
  workspaces under this folder" surfaces resolve through one scope
  field instead of surface-local flags;
- the **trust-decision packet** that every actor-visible grant /
  revoke / narrowing action emits, carrying actor, workspace root,
  reason class, scope, expiry, recovery cues, and audit ids;
- the **audit-event id set** admin export, support bundle, and
  governance packets read to reason about trust state without
  guessing.

The freeze matters **now**, ahead of the tasks / debugger / notebook /
terminal / AI apply / extension-activation / provider-handoff lanes
landing, because without a shared trust-state vocabulary each of those
lanes will either invent its own "is-trusted" check (the failure mode
this ADR prevents) or silently couple to the ADR-0001 binary field and
miss the finer states the product must actually distinguish. The
recovery-ladder packet seed (M00-134), the emergency-action model
(M00-108), the target-discovery taxonomy (M00-27), the record-state
and waiver-expiry seed (M00-26), and the shell interaction-safety
contract (M00-24) all assume a single trust-state vocabulary exists
to reference — this ADR closes that gap.

This ADR rides alongside:

- ADR-0001 — the identity-mode envelope and the binary trusted /
  restricted posture this ADR refines without repealing.
- ADR-0004 — trust-decision packets cross RPC as typed payloads; raw
  policy-bundle bytes, raw consent-capture bodies, and raw evidence
  bodies never do.
- ADR-0005 — trust-state views are reactive and ride the shared
  subscription envelope with authority class `derived_knowledge`;
  downstream surfaces never poll workspace trust from a side channel.
- ADR-0007 — the broker's `trust_state` field and handle-scope rules
  are inherited verbatim; a credential handle issued under `trusted`
  MUST NOT silently survive a downgrade to `restricted`.
- ADR-0008 — admin-policy narrowing is an orthogonal ceiling; trust
  grant widens only up to that ceiling.
- ADR-0009 — execution-context resolution reads `trust_state` before
  admitting repo-owned activators, tasks, debuggers, notebook
  kernels, and AI tool calls.
- ADR-0010 — provider-linked browser / device-code handoff inherits
  the trust-decision packet's actor, workspace root, and scope so
  approval tickets stay couplable to workspace trust.
- ADR-0011 — capability-lifecycle state and dependency markers do
  not silently upgrade their effective posture when trust changes;
  a marker in a restricted workspace stays `degraded_by_trust`.
- ADR-0012 — extension manifest effective-permission projection
  resolves through `trust_state` before activation, install, or
  update.
- ADR-0015 — embedded-surface boundary cards and the native-reserved
  surfaces (workspace-trust elevation, rollback, AI apply, high-risk
  approvals) stay host-native and read this ADR's trust-decision
  packet.
- ADR-0016 — the shell boundary guarantees that every trust-gated
  action routes through the command-dispatch entry point and never
  bypasses preview / approval / trust review.
- `docs/support/recovery_ladder_packet.md` (M00-134) — recovery
  actions referencing this ADR's trust states and transitions by id.
- `docs/security/emergency_action_model.md` (M00-108) — emergency
  actions that force restricted mode or trust revocation use this
  ADR's transition vocabulary.
- `docs/security/safe_preview_trust_classes.md` — the safe-preview
  trust-class vocabulary (`RawText`, `SanitizedRich`,
  `TrustedLocalActive`, `IsolatedRemoteActive`) is downstream of
  workspace trust: an `IsolatedRemoteActive` surface in a restricted
  workspace narrows further to metadata-only on denial.

Full runtime enforcement — the per-crate gate implementations, the
execution-context integration plumbing, the policy-bundle narrowing
evaluator — is **out of scope for this seed**. What is in scope is
the vocabulary, the packet shape, the matrix fields, the invariants,
and the worked examples every later implementation must observe.

## Decision

Aureline freezes, as a seed, **one trust-state vocabulary**, **one
permission-propagation matrix shape**, **one trust-decision packet**,
**one entry-flow transition set**, **one remembered-decision scope
vocabulary**, **one audit-event id set**, and **one escalation-cue
vocabulary** for workspace trust and restricted mode. Every surface
that admits, narrows, or reports a trust-gated action MUST read and
emit these fields by name; no surface may invent a parallel
"is-trusted" boolean, a parallel restricted-mode banner contract, or
a parallel policy narrowing side channel.

All rules below are stated in terms of contract, vocabulary, and event
names rather than specific crates so later implementation work is
hygiene, not re-litigation.

### Trust states (frozen)

Every workspace, at every moment, is in exactly one trust state. The
state is authoritative for every execution-or-mutation surface and is
surfaced verbatim in the shell chrome, the CLI `aureline doctor`
inspector, the support bundle, and the admin export.

| Trust state                         | Meaning                                                                                                         | Source authority                                                                                      |
|-------------------------------------|-----------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------|
| `untrusted_unknown`                 | Workspace has never been evaluated under the current user profile; restricted posture applies by default.       | Local — first open with no remembered decision, no admin pre-grant, no signed-repo allowance.         |
| `restricted`                        | Read / search / navigate / edit / save admitted; execution / mutation / activator surfaces gated.               | Local — explicit user decline, session-scoped grant expired, or default after recovery-ladder fallback.|
| `trusted`                           | Repo-defined tasks / debuggers / notebook kernels / extension activation / repo-owned activators admitted.      | Local user grant, admin pre-grant, or signed-repo allowance under managed policy.                     |
| `trusted_time_bounded`              | `trusted` posture with an explicit expiry; the workspace returns to `restricted` on expiry, focus-change timeout, or process exit depending on the scope. | Local — session or time-bounded grant.                                                                |
| `trusted_policy_degraded`           | User granted `trusted` but admin / managed / emergency policy has since narrowed the effective posture.         | Managed / self-hosted-org policy bundle or emergency-action bundle narrowing active.                  |
| `restricted_recovery_fallback`      | Boot-time or runtime recovery ladder forced restricted mode; layout restore and activators suspended.           | Recovery-ladder packet application after crash-loop safe mode, cache repair, or startup fault.        |
| `restricted_extension_quarantine`   | Restricted posture with declared extension quarantine active; install / activation gated until quarantine lifts.| Recovery-ladder extension quarantine or emergency-action extension-disable bundle.                    |
| `trust_revoked`                     | Trust was explicitly revoked (user, admin, emergency action); restricted posture is enforced even for surfaces that held a live approval ticket. | User / admin revoke, or emergency-action rotation / kill-switch.                                      |
| `trust_unavailable_identity_gate`   | The workspace's trust decision depends on an identity gate (managed IdP, SCIM binding) that is not currently reachable; restricted posture applies. | Managed identity unreachable, last-known-good identity snapshot not admissible for privileged operations. |

Rules (frozen):

1. **Restricted posture floor.** In every `restricted*` or
   `trust_revoked` state, read / search / navigate / edit / save MUST
   remain admitted. Restricted mode is **not** a mode that can block
   editing.
2. **No silent widening.** A transition from any non-`trusted` state
   to `trusted` or `trusted_time_bounded` MUST route through an
   explicit user / admin action and emit a `workspace_trust_granted`
   audit event. Admin policy narrowing MAY downgrade at any moment
   and emits `workspace_trust_policy_narrowed`.
3. **No implicit posture inheritance.** Opening a sub-folder of a
   trusted workspace does not inherit trust unless the remembered
   decision's scope explicitly covers that path (see "Remembered
   decision scope").
4. **Visible state.** The current trust state MUST be visible on the
   primary shell surface, the command palette inspector, the CLI
   `aureline doctor` output, and any support-bundle export; compact
   layouts MAY rearrange but MAY NOT hide it.
5. **Downgrade on identity / policy drift.** Loss of managed identity,
   policy-epoch roll that narrows the active bundle, or emergency-
   action application automatically transitions the state toward
   `trusted_policy_degraded`, `trust_unavailable_identity_gate`, or
   `trust_revoked` as appropriate. Downgrade is synchronous with
   the next user-visible action on the affected surface.
6. **Recovery-ladder linkage.** A transition into `restricted_recovery_fallback`
   or `restricted_extension_quarantine` MUST cite a recovery-action
   id from `schemas/support/recovery_action.schema.json`; the
   product does not invent recovery labels locally.

### Entry-flow transitions (frozen)

Open / restore / recovery surfaces resolve exactly one transition per
entry. These transitions separate **blocked setup** (restricted,
restore suspended, activators gated) from **optional recommendations**
(suggestions to grant trust, to open elsewhere, to escalate to admin).

| Transition id                        | What the user (or operator) is choosing                                                                                         | Resulting trust state                 | Layout restore | Activators / tasks / debug / notebook kernels |
|--------------------------------------|---------------------------------------------------------------------------------------------------------------------------------|---------------------------------------|:--------------:|:---------------------------------------------:|
| `initial_open_untrusted`             | First open of an unknown workspace; no explicit choice yet.                                                                      | `untrusted_unknown`                   | default        | gated pending decision                         |
| `open_in_restricted_mode`            | Explicit user choice to open restricted; layout restore permitted; activators stay gated.                                        | `restricted`                          | permitted      | gated                                          |
| `continue_in_restricted_mode`        | After a denied, expired, or revoked trust grant, continue the current session restricted.                                        | `restricted` or `trust_revoked`       | permitted      | gated                                          |
| `open_without_restore`               | Open restricted AND suspend layout restore; previously-open tasks / terminals / notebooks do not re-execute.                     | `restricted`                          | suspended      | gated                                          |
| `safe_mode_workspace_restricted`     | Recovery-ladder forced restricted mode after a crash loop or boot fault; third-party extensions disabled.                        | `restricted_recovery_fallback`        | suspended      | gated                                          |
| `extension_quarantine_restricted`    | Restricted posture with a named extension set quarantined (crash-loop bisect outcome or emergency-action bundle).                | `restricted_extension_quarantine`     | permitted      | gated (quarantined set denied)                 |
| `grant_trust_session`                | Explicit user grant for the current session only; expires on process exit, focus timeout, or explicit revoke.                    | `trusted_time_bounded`                | n/a            | admitted under ticket                          |
| `grant_trust_remembered`             | Explicit user grant with a remembered-decision scope that persists across restart for the current user profile.                   | `trusted`                             | n/a            | admitted under ticket                          |
| `grant_trust_admin_prebinding`       | Admin policy pre-binding (signed bundle or repo-signature allowance) admits the workspace without per-user prompt.               | `trusted`                             | n/a            | admitted under ticket                          |
| `revoke_trust`                       | Explicit user / admin revoke; in-flight approval tickets scoped to workspace trust fail closed.                                  | `trust_revoked`                       | n/a            | denied                                         |
| `policy_narrow_to_degraded`          | Managed / self-hosted-org policy narrowed the posture of a previously-trusted workspace.                                         | `trusted_policy_degraded`             | n/a            | selectively admitted per matrix                |
| `policy_restore_trusted`             | Managed narrowing lifted; workspace returns to `trusted`.                                                                        | `trusted`                             | n/a            | admitted                                       |
| `emergency_action_force_restricted`  | Emergency-action bundle (channel freeze, extension kill-switch, trust-root rotation) narrowed the workspace to restricted.       | `restricted` or `trust_revoked`       | permitted      | gated                                          |
| `identity_gate_unavailable`          | Managed IdP or policy source unreachable; workspace awaits identity before admitting privileged operations.                      | `trust_unavailable_identity_gate`     | permitted      | gated                                          |

Rules (frozen):

1. Every entry resolves **exactly one transition** and records it in
   the `trust_decision_packet` as `transition_id`.
2. `open_in_restricted_mode` and `continue_in_restricted_mode` are
   **never the same event**: the first is chosen at open, the
   second after a denied / revoked / expired grant.
3. `open_without_restore` is the only transition that suspends
   layout restore. Every other restricted transition preserves the
   restore path so the user's working state is not thrown away.
4. `safe_mode_workspace_restricted` and
   `extension_quarantine_restricted` MUST cite a recovery-action id
   from the recovery-ladder packet; a transition without that
   citation is rejected as malformed.
5. `grant_trust_session` MUST carry a non-null `expires_at`; a grant
   without an expiry promotes to `grant_trust_remembered` instead.
6. `emergency_action_force_restricted` MUST carry a signed
   emergency-action record reference; it MUST NOT be issued from a
   per-user local decision.

### Permission-propagation matrix (frozen shape)

Every execution-or-mutation surface names its admitted behavior across
the trust states. The matrix lives in machine-readable form at
`artifacts/security/trust_state_matrix.yaml`; this ADR freezes the
**shape, the surface family vocabulary, the authority-kind vocabulary,
and the invariants**. Admitting a new surface family or a new
authority kind is additive-minor with a schema bump plus a row;
repurposing either is breaking.

**Surface families (frozen):**

| Surface family                     | Example operations                                                                                                             |
|------------------------------------|--------------------------------------------------------------------------------------------------------------------------------|
| `workspace_open_restore`           | Open folder, reopen recent, restore last session, reopen in restricted.                                                        |
| `editor_read_write`                | Open file, edit buffer, save to disk, rename.                                                                                  |
| `search_local`                     | Text / symbol / reference search over the workspace.                                                                           |
| `local_git_read`                   | Read repository state, branches, log, diff.                                                                                    |
| `local_git_write`                  | Commit, stage, merge, push, pull, fetch, rebase.                                                                               |
| `shell_command_palette`            | Dispatch a shell command from palette / keybinding / menu.                                                                     |
| `tasks_run`                        | Run a repo-defined task (`tasks.json` or equivalent).                                                                           |
| `terminal_manual_open`             | User-initiated terminal in the workspace root.                                                                                 |
| `terminal_repo_recipe_launch`      | Repo-owned recipe / launcher / auto-start hook opens a terminal with preconfigured command.                                    |
| `debug_launch`                     | Debug session launch for a repo-defined configuration.                                                                         |
| `notebook_kernel_connect`          | Attach to a kernel declared by the workspace (repo-local kernelspec, environment activator).                                   |
| `notebook_cell_execute`            | Execute a notebook cell (any language).                                                                                        |
| `notebook_rich_output_render`      | Render sanitized rich output (HTML / JS / iframe payloads) from a notebook cell.                                               |
| `ai_context_read`                  | AI reads workspace content (files, search results) for prompt assembly.                                                        |
| `ai_apply_mutation`                | AI applies edits, creates files, or runs a workspace-mutating action.                                                          |
| `ai_tool_call_mutating`            | AI invokes a mutating tool (run task, run shell, call provider, apply patch).                                                  |
| `extension_activation`             | Extension auto-activation on workspace events / file pattern / language id.                                                    |
| `extension_install`                | Install / update a new extension inside the workspace context.                                                                 |
| `environment_activator_run`        | Repo-owned toolchain activator (direnv, venv auto-activate, `.envrc`, devcontainer lifecycle).                                 |
| `scaffold_template_run`            | Generate or run a scaffold / template that writes files or runs hooks.                                                         |
| `connected_provider_open`          | Open-in-browser, device-code handoff, OAuth flow to a connected provider.                                                      |
| `connected_provider_tool_call`     | AI or extension calls a connected-provider tool (billing-, mutation-, or egress-capable).                                      |
| `remote_attach`                    | Attach to an SSH / container / managed remote target that inherits workspace trust decisions.                                  |
| `mcp_server_launch`                | Launch a Model Context Protocol server declared by the workspace.                                                              |
| `support_bundle_export`            | Export a support bundle containing workspace state (always read-only; gated by redaction policy, not trust).                    |
| `admin_policy_read`                | Read currently effective admin policy bundle summary for display.                                                              |

**Authority kinds (frozen):**

| Authority kind                     | Meaning on the admitted surface                                                                                                |
|------------------------------------|--------------------------------------------------------------------------------------------------------------------------------|
| `allowed`                          | Admitted under the active trust state without extra gating.                                                                    |
| `read_only`                        | Admitted only in a read posture; writes / mutations / executions narrow to metadata or preview.                                |
| `degraded_preview_only`            | Admitted as a preview that MUST NOT commit; requires promoted trust or approval ticket before commit.                          |
| `blocked_pending_trust`            | Denied with a typed reason; surfaces offer the trust-grant affordance as a **recommendation**, not a blocker to editing.       |
| `blocked_pending_approval`         | Denied pending a live approval ticket (ADR-0009); trust is necessary but not sufficient.                                       |
| `approval_required_per_invocation` | Admitted only with a per-invocation approval ticket; remembered session-level trust is not sufficient.                         |
| `policy_denied`                    | Denied by admin / managed / emergency policy; surfaces cite policy source and scope.                                           |
| `quarantine_denied`                | Denied because the specific extension / kernel / recipe target is quarantined; other targets may still admit.                  |
| `not_applicable`                   | The surface has no execution or mutation meaning under this trust state.                                                       |

**Matrix invariants (frozen):**

1. Every row in `artifacts/security/trust_state_matrix.yaml` names
   exactly one `surface_family` and declares a `per_state_authority`
   block with exactly one authority entry per frozen trust state.
2. For every surface, the `restricted`, `untrusted_unknown`,
   `restricted_recovery_fallback`, `restricted_extension_quarantine`,
   `trust_revoked`, and `trust_unavailable_identity_gate` entries
   MUST NOT be `allowed` for any surface outside the
   **restricted-posture floor** (`editor_read_write`, `search_local`,
   `local_git_read`, `workspace_open_restore`, `support_bundle_export`,
   `admin_policy_read`).
3. For every surface, `trusted_policy_degraded` authority MUST be
   equal-or-narrower than `trusted`; admin policy narrows, never
   widens.
4. For every surface, `policy_denied` and `quarantine_denied`
   outcomes MUST cite a policy or quarantine record reference in
   the trust-decision packet's `source_reason_refs`.
5. Every row citing `approval_required_per_invocation` MUST name the
   approval-ticket scope (single-operation, short-window-session,
   or operation-class) so downstream surfaces know what the ticket
   admits.
6. `support_bundle_export` MUST remain at least `read_only` in every
   trust state; support / diagnostics cannot be gated by trust alone.
7. `connected_provider_open` MUST route through the browser-handoff
   packet (ADR-0010) and MUST NOT be promoted to
   `approval_required_per_invocation` without a corresponding
   approval ticket reference.
8. `ai_apply_mutation` and `ai_tool_call_mutating` are `blocked_pending_trust`
   in every non-`trusted` state by default; promotion to
   `approval_required_per_invocation` under `trusted_policy_degraded`
   requires an explicit policy-narrowing row.

### Remembered-decision scope (frozen)

Every grant / decline carries an explicit remembered-decision scope.
Surfaces resolve future opens through the declared scope; they do not
invent surface-local "trust this folder" flags.

| Remembered scope                               | Meaning                                                                                                  | Survives restart |
|------------------------------------------------|----------------------------------------------------------------------------------------------------------|:----------------:|
| `session_only`                                 | Decision applies for the current process life only.                                                      | no               |
| `per_workspace_per_user`                       | Decision applies to the exact workspace root for the current user profile.                               | yes              |
| `per_workspace_per_user_until_policy_epoch`    | `per_workspace_per_user` plus the current admin policy epoch; a policy roll invalidates the decision.    | yes (epoch-bounded) |
| `per_parent_directory_per_user`                | Decision applies to every immediate child workspace of a declared parent, for the current user profile.  | yes              |
| `admin_pre_grant_scope`                        | Decision is issued by an admin policy bundle; rescope follows the bundle's own scope fields.             | yes (bundle-bounded) |
| `signed_repo_allowance`                        | Decision rides a signed-repo allowance (repository signature continuity under managed policy).           | yes (signature-bounded) |
| `never_remembered`                             | Decision is explicit-decline for this open; restricted posture recorded, no future prompt suppression.   | yes (decline-only) |

Rules:

1. **No cross-user remember.** Trust grants are bound to the current
   user profile. Profile switch, SCIM role change, or signed-repo
   allowance revocation invalidates the decision.
2. **No cross-profile bleed.** `per_parent_directory_per_user` MUST
   resolve against canonical paths and MUST NOT follow symlinks
   outside the declared parent.
3. **Epoch-bounded admin narrowing.** `per_workspace_per_user_until_policy_epoch`
   is the only scope that automatically re-prompts on policy roll;
   `per_workspace_per_user` persists across roll but the resulting
   effective posture may downgrade to `trusted_policy_degraded`.
4. **Decline is remembered.** `never_remembered` is the shape of an
   explicit decline: the product records that the user said no and
   does not re-prompt automatically, but no future elevation is
   implied.
5. **Admin pre-grant visibility.** `admin_pre_grant_scope` and
   `signed_repo_allowance` grants MUST be visible to the user in
   the workspace-trust panel with source, signer, expiry, and
   scope summary.

### Trust-decision packet (frozen)

Every observable trust-affecting event emits a **trust-decision
packet**. The packet is the canonical cross-surface object. Admin
exports, support bundles, shell chrome, CLI explain output, audit
streams, recovery-ladder entries, and governance packets all consume
the same shape. The schema is
`schemas/security/trust_decision_packet.schema.json`.

Every packet carries:

- `packet_id` — opaque, stable, safe on RPC and in exports.
- `packet_kind` — one of `trust_decision_record`,
  `trust_audit_event_record`, or `trust_matrix_inspection_record`.
- `packet_schema_version` — integer, bumped only on breaking change.
- `workspace_root_ref` — opaque id of the workspace root; raw paths
  never appear.
- `workspace_display_scope` — reviewable sentence describing the
  root for chrome / support display only.
- `actor_class` — one of `local_user`, `local_admin`, `managed_admin`,
  `signed_repo_allowance`, `emergency_action_signer`,
  `recovery_ladder_application`, `policy_epoch_roll`,
  `identity_gate_unavailable_system`.
- `actor_identity_ref` — opaque identity ref (may be null for
  system-issued transitions).
- `identity_mode` — one of `account_free_local`, `self_hosted_org`,
  `managed_convenience` (inherited from ADR-0001).
- `prior_trust_state` / `resulting_trust_state` — both drawn from
  the trust-state vocabulary above.
- `transition_id` — drawn from the entry-flow transition set.
- `reason_class` — one of `explicit_user_grant`,
  `explicit_user_decline`, `explicit_user_revoke`, `session_expired`,
  `focus_timeout_expired`, `admin_pre_grant`,
  `signed_repo_allowance_admitted`, `admin_policy_narrowed`,
  `admin_policy_restored`, `emergency_action_applied`,
  `recovery_ladder_fallback`, `extension_quarantine_applied`,
  `identity_gate_unavailable`, `policy_epoch_roll_required_reprompt`,
  `malformed_decision_rejected`.
- `remembered_decision_scope` — drawn from the remembered-scope
  vocabulary; `session_only` for session grants; null for
  system-issued transitions that do not carry a remembered scope.
- `expires_at` — monotonic timestamp; null when the decision does
  not expire.
- `policy_context` — policy epoch, tenant scope, active bundle refs
  (same shape ADR-0005 and ADR-0008 carry).
- `source_reason_refs` — array of opaque refs to policy bundle,
  emergency-action record, recovery-action, extension-quarantine
  record, signed-repo allowance, or admin pre-grant that motivated
  the decision. Raw bundle bytes, raw signatures, raw policy
  payloads, raw evidence bodies never appear here.
- `affected_surfaces` — array of `surface_family` names the packet
  is currently narrowing, widening, or reporting on.
- `escalation_cues` — array drawn from the escalation-cue vocabulary
  below; describes the repair / review path a surface MAY present.
- `recovery_action_ref` — opaque ref into
  `schemas/support/recovery_action.schema.json` when the packet
  rides a recovery ladder; null otherwise.
- `audit_event_id` — one of the frozen audit-event ids below.
- `issued_at` — monotonic timestamp.
- `redaction_class` — drawn from ADR-0007 redaction classes; every
  packet declares it.
- `notes` — reviewable sentence; raw secret material, raw policy
  payloads, and raw filesystem paths MUST NOT appear.

### Escalation cues (frozen)

Escalation cues are the repair / review affordances a surface MAY
offer when the trust decision narrowed or denied. They are
**recommendations**, not blockers to editing; a surface that is
`blocked_pending_trust` MUST keep the editor / search / save floor
available while it presents the cue.

| Cue id                                | What the surface offers                                                                                                     |
|---------------------------------------|-----------------------------------------------------------------------------------------------------------------------------|
| `request_trust_grant`                 | Present the workspace-trust-grant affordance (with scope picker).                                                           |
| `request_trust_grant_session_only`    | Same, restricted to `session_only` scope.                                                                                   |
| `request_approval_ticket`             | Route to the per-invocation approval surface (ADR-0009).                                                                    |
| `request_admin_policy_change`         | Route to admin-policy-change request (admin-surface-only; may be a support link in non-managed deployments).                |
| `request_trust_store_unlock`          | Route to the trust-store unlock flow (ADR-0007).                                                                            |
| `request_identity_sign_in`            | Route to the identity sign-in flow; restricted posture continues until identity resolves.                                   |
| `route_to_recovery_ladder`            | Route to the recovery-ladder packet entry (M00-134).                                                                        |
| `route_to_support_bundle_export`      | Offer support-bundle export with workspace trust posture included (redacted).                                               |
| `continue_restricted_no_elevation`    | Keep working in restricted posture; no elevation path is recommended.                                                       |
| `no_recovery_available`               | State explicitly that no product-side repair exists; admin contact is the only path.                                        |

### Audit event ids (frozen)

Every observable trust-affecting action emits a structured event on
the `workspace_trust` audit stream. Events MUST NOT carry raw secret
material, raw policy-bundle bytes, raw filesystem paths, raw consent-
capture bodies, or raw evidence bodies.

| Event id                                          | Fires when                                                                                                |
|---------------------------------------------------|-----------------------------------------------------------------------------------------------------------|
| `workspace_trust_state_resolved`                  | A workspace's trust state was resolved at open or restore time.                                           |
| `workspace_trust_granted`                         | User / admin / signed-repo allowance granted `trusted` or `trusted_time_bounded`.                         |
| `workspace_trust_declined`                        | User declined to grant trust; workspace remains in `restricted` or `untrusted_unknown`.                   |
| `workspace_trust_revoked`                         | Explicit user / admin revoke, or emergency action forced revocation.                                      |
| `workspace_trust_session_grant_expired`           | A `trusted_time_bounded` grant expired on process exit, focus timeout, or explicit session close.         |
| `workspace_trust_policy_narrowed`                 | Managed / self-hosted-org policy narrowed the effective posture of a `trusted` workspace.                 |
| `workspace_trust_policy_restored`                 | Managed narrowing lifted; effective posture returned to `trusted`.                                        |
| `workspace_trust_emergency_applied`               | An emergency-action bundle narrowed the workspace to `restricted` or `trust_revoked`.                     |
| `workspace_trust_recovery_applied`                | A recovery-ladder action forced `restricted_recovery_fallback` or `restricted_extension_quarantine`.      |
| `workspace_trust_identity_gate_unavailable`       | Managed identity unreachable; workspace entered `trust_unavailable_identity_gate`.                        |
| `workspace_trust_identity_gate_restored`          | Identity reachable again; workspace exited `trust_unavailable_identity_gate`.                             |
| `workspace_trust_remembered_decision_recorded`    | A remembered-decision scope was persisted (grant or decline).                                             |
| `workspace_trust_remembered_decision_invalidated` | A remembered-decision was invalidated by policy-epoch roll, signer rotation, or user profile switch.      |
| `workspace_trust_matrix_row_denied`               | A surface request was denied by the permission-propagation matrix; packet names surface + reason.         |
| `workspace_trust_matrix_row_admitted`             | A surface request was admitted by the matrix; packet names surface + authority kind.                     |
| `workspace_trust_decision_malformed`              | A trust-decision packet failed validation; the intended transition was rejected.                         |

### Process-boundary constraints (frozen)

1. Trust-decision packets cross RPC as typed payloads only. Raw
   policy-bundle bytes, raw consent-capture bodies, raw evidence
   bodies, raw filesystem paths, and raw signing-key material MUST
   NOT cross.
2. Workspace trust state MUST NOT be re-derived in extension hosts,
   remote agents, or AI tool hosts; those surfaces read the state
   through the subscription envelope (ADR-0005) with authority class
   `derived_knowledge` and a declared freshness hint.
3. Remote-agent attach MUST carry the remote-side trust-decision
   packet and MUST NOT silently widen the local workspace's trust
   state on behalf of the remote target; conversely, remote-side
   elevation does not automatically elevate the local workspace.
4. AI tool calls that would mutate or elevate MUST read the current
   trust state on the user's host and MUST NOT cache a stale snapshot
   past the packet's `expires_at` or a subsequent state-change event.
5. Mutation-journal entries, save manifests, claim manifests, and
   support bundles carry the trust-decision packet ids and state
   names only; they do not carry raw policy payloads or raw
   remembered-decision bodies.
6. Crash dumps and core files MUST NOT inherit live trust-decision
   packets; the broker's redaction pass (ADR-0007) excludes them by
   default.

### Schema of record

Rust types in the eventual workspace-trust crate are the schema of
record. The JSON Schema export at
`schemas/security/trust_decision_packet.schema.json` is the cross-tool
boundary at this milestone (mirroring ADR-0004, ADR-0005, ADR-0006,
and ADR-0007). No external IDL or codegen toolchain is introduced at
this milestone.

### Worked examples

Worked `trust_decision_record` examples live under
`fixtures/security/restricted_mode_cases/`. Each case binds the
vocabulary frozen here to a concrete entry flow so later tasks,
debugger, notebook, AI, extension, and remote lanes can anchor their
own integration on one shared object instead of inventing parallel
ones:

- `first_open_untrusted_unknown.json` — initial open of an unknown
  workspace, restricted posture, trust-grant affordance presented as
  a recommendation.
- `open_in_restricted_mode.json` — explicit `open_in_restricted_mode`
  transition with layout restore permitted.
- `continue_in_restricted_mode_after_decline.json` — user declined
  trust; session continues restricted; matrix rows show
  `blocked_pending_trust` for tasks, debug, notebook-execute, AI
  apply.
- `open_without_restore.json` — recovery-path entry; restore
  suspended; previously-open tasks and notebooks do not re-execute.
- `safe_mode_workspace_restricted.json` — recovery-ladder-forced
  restricted fallback after a crash loop; third-party extensions
  quarantined.
- `grant_trust_session_bounded.json` — `grant_trust_session`
  transition; `trusted_time_bounded` with `expires_at`; matrix rows
  widen for the session.
- `grant_trust_remembered_workspace_scope.json` — remembered
  decision bound to the workspace root for the current user profile.
- `policy_narrow_trusted_to_degraded.json` — admin policy bundle
  narrowed a trusted workspace; matrix rows for `ai_apply_mutation`
  downgrade from `allowed` to `approval_required_per_invocation`.
- `emergency_action_force_restricted.json` — emergency-action
  bundle applied; workspace forced to `restricted`; escalation cue
  points at recovery ladder.
- `identity_gate_unavailable.json` — managed IdP unreachable;
  workspace in `trust_unavailable_identity_gate`; admin export shows
  stale-but-last-known-good posture.
- `matrix_inspection_snapshot.json` — a `trust_matrix_inspection_record`
  snapshotting the per-surface authority for a workspace under
  `trusted_policy_degraded`.

### Open questions

Promotion of this seed from Proposed to Accepted requires closing the
following. The successor ADR owns the close.

1. **Per-workspace notebook-trust ladder.** Notebooks carry their own
   trust ladder layered on top of workspace trust (cell-level
   `trusted_output`, `cleared_output`, `untrusted_output`). This seed
   names `notebook_rich_output_render` as a surface row and defers
   the ladder's internal vocabulary to the notebook-trust seed
   (M00-27 follow-on).
2. **Signed-repo allowance chain.** The signed-repo allowance scope
   requires a signer-continuity binding the signing-lane freeze
   (M00-108) will formalise. This seed reserves the scope name;
   admission rules land in the successor ADR.
3. **Remote-agent trust binding.** The exact binding between local
   workspace trust and remote-agent workspace trust (inherit, narrow,
   separate) is deferred to the remote-agent ADR. The present ADR
   locks that neither side widens the other implicitly.
4. **AI apply per-class sub-matrix.** `ai_apply_mutation` collapses
   apply-edit / create-file / run-task / call-provider into one row.
   A sub-matrix per action class is deferred to the AI evidence and
   approval ADR; this seed keeps a single conservative row.
5. **Per-invocation approval-ticket scope set.** The "approval ticket
   scopes" (single-operation, short-window-session, operation-class)
   named above require a full scope register in the execution-context
   ADR. This seed reserves the scope names and defers the scope
   lifetime rules.
6. **Extension-quarantine admission registry.** The exact set of
   extensions admitted under `restricted_extension_quarantine` is
   determined by the recovery-ladder and emergency-action lanes; this
   seed reserves the transition id and cites the recovery-action ref
   without enumerating bundles.
7. **Cross-profile migration semantics.** User-profile switch
   invalidates remembered decisions; the successor ADR closes the
   migration / export rules for organisational rollout.

### Non-goals at this decision

- Full runtime enforcement. The matrix admits only shape and
  vocabulary; per-crate gate implementations land under the tasks,
  debugger, notebook, AI, extension, and remote-agent lanes.
- Every future trust surface. The surface family vocabulary is
  additive-minor; new families land with schema bumps.
- A complete approval-ticket lifecycle. Approval tickets are
  named here; their full lifecycle lives under ADR-0009 and its
  successors.
- A complete emergency-action integration. The emergency-action
  record reference is named; the bundle's internal vocabulary lives
  under ADR-derived work seeded by M00-108.
- A complete notebook-trust ladder. Reserved; closed under M00-27's
  notebook-trust follow-on.
- A complete recovery-ladder packet. Reserved; closed under M00-134.
- A UI design for the workspace-trust panel. Layout and chrome live
  under the shell interaction-safety contract (M00-24) and the
  embedded-surface boundary (ADR-0015).

## Consequences

- **Reserved (pending successor-ADR freeze):** the trust-state
  vocabulary (`untrusted_unknown`, `restricted`, `trusted`,
  `trusted_time_bounded`, `trusted_policy_degraded`,
  `restricted_recovery_fallback`, `restricted_extension_quarantine`,
  `trust_revoked`, `trust_unavailable_identity_gate`), the
  entry-flow transition set, the surface-family vocabulary, the
  authority-kind vocabulary, the remembered-decision scope
  vocabulary, the audit-event id set, the escalation-cue set, and
  the trust-decision packet shape.
- **Reserved:** the restricted-posture floor (read / search /
  navigate / edit / save / workspace open / support bundle export /
  admin policy read remain admitted across every non-`trusted`
  state). Restricted mode is an explicit product state with
  inspectable recovery paths, not a warning banner.
- **Reserved:** trust grant MAY widen only up to the admin-policy
  ceiling; admin / managed / emergency policy narrows but never
  silently widens.
- **Reserved:** `open_in_restricted_mode`, `continue_in_restricted_mode`,
  `open_without_restore`, and later trust-escalation transitions
  separate blocked setup (restricted posture, suspended restore,
  gated activators) from optional recommendations (trust-grant
  affordance, approval-ticket route, recovery-ladder route).
- **Reserved:** the schema of record is Rust types in the eventual
  workspace-trust crate; the boundary schema lives at
  `schemas/security/trust_decision_packet.schema.json`; no external
  IDL or codegen toolchain at this milestone.
- **Permitted:** adding a new surface family, a new authority kind,
  a new entry-flow transition, a new remembered-decision scope, a
  new reason class, a new audit-event id, or a new escalation cue
  is additive-minor with a schema bump plus a matrix row;
  repurposing any existing value is breaking and requires a new
  decision row.
- **Permitted:** admin policy MAY narrow the matrix further on any
  surface. Policy MAY NOT silently widen collection beyond a
  surface's restricted-posture floor (editor / search / save /
  support bundle / admin policy read MUST remain admitted).
- **Follow-up:** the tasks, terminal, debug, notebook, AI,
  extension, connected-provider, remote-agent, recovery, and
  emergency-action lanes instrument their surface row and respect
  every frozen trust state, transition, and escalation rule before
  claiming trust guarantees.
- **Follow-up:** the notebook-trust ladder, the signed-repo
  allowance chain, the remote-agent trust binding, the AI apply
  sub-matrix, the per-invocation approval-ticket scope set, the
  extension-quarantine admission registry, and the cross-profile
  migration semantics close under named follow-on ADRs.
- **Ratifies:** ADR-0001's binary trusted / restricted posture
  remains the envelope; this ADR refines the finer states the
  product surfaces must distinguish without repealing ADR-0001.
  ADR-0005's subscription envelope carries trust-state views as
  `derived_knowledge`. ADR-0007's broker `trust_state` field and
  handle-scope rules are inherited verbatim. ADR-0009's execution
  context reads the trust state before admitting activators.
  ADR-0010's browser-handoff packet inherits the trust-decision
  packet's actor, workspace root, and scope fields. ADR-0011's
  capability-lifecycle markers stay `degraded_by_trust` in
  restricted states. ADR-0015's native-reserved surfaces
  (workspace-trust elevation, rollback, AI apply, high-risk
  approvals) remain host-native and read this ADR's packet.

## Alternatives considered

- **Keep the ADR-0001 binary trusted / restricted posture and
  nothing else.** Rejected: the binary misses the states the
  product surfaces must actually distinguish — `trusted_policy_degraded`
  after admin narrowing, `restricted_recovery_fallback` after a
  crash loop, `trust_unavailable_identity_gate` when managed
  identity is unreachable. Without a named vocabulary, each surface
  either invents one or collapses the states into the binary,
  losing fidelity the shell chrome and support bundles already
  promise.
- **Per-surface trust checks without a shared matrix.** Rejected:
  this is the failure mode the ADR exists to prevent. A tasks
  check, a terminal check, a notebook check, an AI apply check, an
  extension check, and a provider check that each implement their
  own trust logic converge on a weakest-link posture the user
  cannot reason about. One shared matrix row per surface keeps the
  contract auditable.
- **Let admin policy widen trust.** Rejected: the restricted-
  posture floor and the trust-grant-up-to-ceiling rule depend on
  admin / managed / emergency policy being narrowing-only. A widen
  path would reintroduce the very attack surface (admin override
  making the product execute repo-defined code silently) that the
  ADR closes.
- **Single remembered-decision scope.** Rejected: the product
  already promises per-folder trust, parent-directory trust for
  monorepos, signed-repo allowance, admin pre-grant, and explicit
  decline. Collapsing these into one "trusted folder" flag loses
  fidelity admin export and support bundles need.
- **Layout restore always re-runs repo-owned activators.**
  Rejected: restore is a value-add; it cannot become a silent
  bypass of workspace trust. `open_without_restore` is the
  explicit opt-out that a recovery flow uses to keep the user
  working without re-executing the author's code.
- **Inline runtime enforcement in this ADR.** Rejected: the seed
  freezes the boundary, not the implementation. Runtime
  enforcement rides the execution-context, tasks, debugger,
  notebook, AI, and extension ADRs; freezing the vocabulary first
  is what keeps those lanes comparable.
- **Fold the trust-decision packet into the emergency-action
  record or the recovery-action record.** Rejected: trust
  decisions happen at far higher frequency than emergency actions
  or recovery actions, and they happen under local user authority
  most of the time. A shared packet keeps per-surface admin-export
  cost low without forcing every grant / decline through the
  emergency / recovery machinery.
- **External IDL + generator for the trust-decision packet.**
  Rejected: same argument ADR 0004, ADR 0005, ADR 0006, and
  ADR 0007 make — an IDL without a second-language consumer costs
  more than it buys; the JSON Schema export reserves the
  integration point.

## Source anchors

- `.t2/docs/Aureline_PRD.md:1081` — "explicit 'reopen last workspace
  in restricted mode' recovery option".
- `.t2/docs/Aureline_PRD.md:1435` — "terminals in restricted
  workspaces may be available for manual use, but repo-defined
  tasks, injected launchers, and auto-run scripts remain gated by
  workspace trust".
- `.t2/docs/Aureline_PRD.md:1457` — "notebook trust should follow
  the spirit of Jupyter's model: no code or active content should
  execute merely because a notebook was opened".
- `.t2/docs/Aureline_PRD.md:1516` — "restricted mode blocks auto-
  activation of repo-owned hooks and mutating environment managers
  until trust is granted".
- `.t2/docs/Aureline_PRD.md:1678` — "repo-defined recipes that
  execute code, mutate environments, or call networked tools are
  subject to workspace trust and admin policy".
- `.t2/docs/Aureline_PRD.md:1703` — "a clear recovery ladder: safe
  mode → extension quarantine → cache reset → workspace restricted
  mode".
- `.t2/docs/Aureline_PRD.md:2860` — "10.8 Workspace trust and
  restricted mode".
- `.t2/docs/Aureline_PRD.md:2862` — "opening a workspace should not
  automatically grant permission to execute code, tasks, debuggers,
  terminals, or privileged extensions".
- `.t2/docs/Aureline_PRD.md:4603` — "RFC-004 — Workspace trust,
  restricted mode, and permission propagation".
- `.t2/docs/Aureline_PRD.md:4784` — "SEC-TRUST-001 — No code egress,
  prompt retention, or privileged execution by default in new
  workspaces".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4164` —
  "Workspace trust and restricted mode".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4200` —
  "malicious repository — restricted mode, trust gating, explicit
  approvals".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4240` —
  "Workspace trust scope — fail closed for privileged operations;
  read/search stay available".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4249` —
  "the UI must always expose current org/tenant, workspace trust
  state, policy source, and whether an operation is using local,
  delegated, or hosted credentials".
- `.t2/docs/Aureline_Technical_Design_Document.md:4245` — "Aureline
  adopts a restricted-mode model".
- `.t2/docs/Aureline_Technical_Design_Document.md:4256` — "Blocked
  or gated by default: tasks, debuggers, terminals where policy
  requires gating, package-install helpers, privileged extensions,
  AI apply flows, repo-owned activators, network-enabled privileged
  operations".
- `.t2/docs/Aureline_Technical_Design_Document.md:5073` — "may never
  silently widen workspace trust".
- `.t2/docs/Aureline_Technical_Design_Document.md:9685` — "notebook
  open never autoexecutes stored output, widgets, or active
  content, and workspace trust, notebook trust, kernel availability,
  and output trust remain visibly distinct".

## Linked artifacts

- Decision register row:
  `artifacts/governance/decision_index.yaml#D-0023`.
- RFC: none at this milestone (RFC-004 in the backlog is the
  eventual successor).
- Trust-state permission-propagation matrix (machine form):
  `artifacts/security/trust_state_matrix.yaml`.
- Trust-decision packet schema (boundary form):
  `schemas/security/trust_decision_packet.schema.json`.
- Worked restricted-mode cases:
  `fixtures/security/restricted_mode_cases/`.
- Identity-mode envelope this ADR refines:
  `docs/adr/0001-identity-modes.md`.
- Transport boundary trust-decision packets cross as typed payloads:
  `docs/adr/0004-rpc-transport-and-schema-toolchain.md`.
- Reactive-truth contract trust-state views subscribe through:
  `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`.
- Secret broker whose `trust_state` and handle-scope rules are
  inherited:
  `docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`.
- Settings resolver whose admin-policy narrowing is the ceiling
  trust grants widen up to:
  `docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`.
- Execution-context resolver that reads trust state before admitting
  activators:
  `docs/adr/0009-execution-context-and-scope.md`.
- Connected-provider browser-handoff packet whose actor / workspace
  / scope fields this packet mirrors:
  `docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`.
- Capability-lifecycle markers that stay `degraded_by_trust` in
  restricted states:
  `docs/adr/0011-capability-lifecycle-and-dependency-markers.md`.
- Extension manifest / effective-permission / policy-pack seed
  whose permission projection reads `trust_state`:
  `docs/adr/0012-extension-manifest-permission-publisher-policy.md`.
- Embedded-surface boundary whose native-reserved
  workspace-trust-elevation surface consumes this packet:
  `docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`.
- Shell interaction-safety contract whose preview / approval rules
  this ADR inherits:
  `docs/ux/shell_interaction_safety_contract.md`.
- Safe-preview trust classes that downgrade further under
  restricted workspaces:
  `docs/security/safe_preview_trust_classes.md`.
- Emergency-action model whose `emergency_action_force_restricted`
  transition references are carried in `source_reason_refs`:
  `docs/security/emergency_action_model.md`.
- Recovery-ladder packet whose `recovery_action_ref` is required
  for `restricted_recovery_fallback` and
  `restricted_extension_quarantine` transitions:
  `docs/support/recovery_ladder_packet.md`.
- Affected lanes: `governance_lane:security_trust_review`,
  `governance_lane:docs_public_truth`,
  `governance_lane:support_export`,
  `governance_lane:governance_packets`.

## Supersession history

First acceptance. No supersession.
