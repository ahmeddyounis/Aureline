# Fixture: contextual degraded — remote session Read-only degraded

## Scenario

The user is connected to a remote workspace over SSH. The
authority (bearer token) has expired; writes that would be
authoritative to the remote MUST NOT commit. Local reads
continue; the shell renders the **`Read-only degraded`**
controlled label on the remote-session status item and blocks
writes before they commit.

## Row bindings

- **Failure-tier row.** `tier.contextual_degraded`.
- **Lifecycle row.** `lifecycle:remote_session`, state
  `remote.read_only_degraded`.
- **Startup-state intersection.**
  `startup_state:offline_startup` (when the remote became
  unreachable mid-session) per
  [`entry_restore_truth_audit.md`](../../../docs/ux/entry_restore_truth_audit.md)
  §6.8.
- **Controlled label.** `Read-only degraded`.
- **Truth class.** `runtime_observed_truth` on the remote-
  session status; `workspace_vcs_truth` on the remote
  working tree (now read-only).
- **Degraded-state token.** `Cached` on the rows served from
  the last successful read; composes with `Offline` if the
  remote is fully unreachable.

## Required axes rendered

- **Cause token.** `authority_expired_for_remote` (reauth
  required) or `remote_unreachable` (reconnect required) —
  named on the status-item reveal.
- **Preserved.** Read of the last-successful remote state;
  local-only rows remain fully writable; recovery journal
  continues; keyboard-reachable chrome.
- **Narrowed.** Writes to the remote working tree, the remote
  index, and the remote VCS refs are blocked before commit.
  The shell refuses mutations and names the block (no silent
  no-op writes).
- **Next-safe action hooks.** `reauth_required`,
  `reconnect_required`, `continue_in_restricted_mode`,
  `open_without_restore`.

## Placement

- **Failure tier.** `tier.contextual_degraded`.
- **Delivery surfaces.** `contextual_banner` at the workspace
  root with the controlled label; `status_item` pinned on the
  remote-session indicator; `durable_job_row` for the
  reconnect / reauth job.
- **Interruptibility tier.** `tier_durable`.

## Controlled label

- **Label text rendered.** "Read-only degraded" (reserved per
  taxonomy §8.6).
- **Supporting subtitle.** Names exactly what is blocked
  ("Commits to *[remote-ref]* paused — reauthenticate to
  resume.").

## Recovery affordances (keyboard-reachable)

- **`Reauthenticate`** — triggers the reauth flow
  (`next_step_decision_hook = reauth_required`).
- **`Reconnect`** — triggers reconnect
  (`next_step_decision_hook = reconnect_required`).
- **`Continue locally`** — confirms local-only edits and
  names what will not sync
  (`rung.restricted_mode_fallback`).
- **`Reveal last failure`** — opens the reauth/reconnect
  evidence (authority class, expiry time by ref).

## Last-failure reason (mandatory visibility)

- **Keyboard-reachable reveal.** Remote-session status-item
  drawer; reveal verb announced verbatim.
- **Support-export field.** `auth_evidence_packet`,
  `recovery_ladder_packet`. Preserves the authority class,
  the expiry timestamp pair, and the recovery-rung history.
  No raw credential material crosses the boundary (per the
  secret-broker ADR).
- **Export-safe description.** Redaction pass runs; raw token
  material is not included on any support export.

## Promotion triggers

- **To `tier.workflow_block`.** A user action that requires
  remote writes (e.g., `Commit`, `Push`, `Run remote task`) —
  a window-attached sheet explains the block and offers the
  reauth / reconnect / continue-locally actions.
- **To `tier.session_recovery`.** Policy explicitly disables
  remote access → `rung.restricted_mode_fallback` becomes the
  durable posture for the session.

## Recovery / support / measurement

- **Recovery-ladder rungs.** `rung.restricted_mode_fallback`,
  `rung.open_without_restore`.
- **Support-packet family.** `auth_evidence_packet`,
  `recovery_ladder_packet`.
- **Journey-trace class.** `shell_open`.

## Accessibility

- "Read-only degraded" announced by assistive technology;
  the blocked-writes list is separately addressable.
- Reauth and reconnect affordances are keyboard-reachable
  and honour the IME / focus-return rules in the a11y
  packet template.

## Forbidden copy and patterns on this path

- "Offline" as a bare word
- "All remotes connected"
- "Online"
- "Error" / "Failed"
- Silent no-op writes to the remote (denies with
  `read_only_degraded_silent_noop`)
- Remote-reachable rows rendering the same label as
  unreachable rows (every remote row names its own posture)

## Expected observable outcomes

- The controlled label `Read-only degraded` renders; writes
  are refused **before** commit with a typed denial.
- Last-failure reason (authority expiry, reconnect failure)
  is keyboard-reachable and preserved on export.
- `overclaims_readiness = false` is asserted.

## Fixture fields (seed)

```yaml
__fixture__:
  name: contextual_degraded_remote_read_only
  taxonomy_rows:
    - tier.contextual_degraded
    - lifecycle:remote_session
  startup_state_intersection: startup_state:offline_startup
  controlled_label: "Read-only degraded"
  doc_section: docs/ux/state_and_recovery_taxonomy.md#7.2
running_build_identity_ref: build-identity-seed-state-ctx-remote-ro
overclaims_readiness: false
```
