# Fixture: escalation surface — policy blocked requires admin

## Scenario

The user has attempted restricted-mode fallback, then a cache-
index repair, then a reconnect, and each rung has failed
because admin policy blocks the path (e.g., opening a remote
workspace on a policy-managed device with a revoked managed
workspace token). The session-recovery tier has exhausted its
product-side options; the shell surfaces the escalation via
the Support Centre with an object-issue-handoff packet linked
for the user to send to administration.

## Row bindings

- **Failure-tier row.** `tier.escalation_surface`.
- **Empty-state intersections.**
  `empty:permission_or_policy_blocked`,
  `empty:unsupported_on_this_surface`.
- **Lifecycle rows.** `lifecycle:workspace` with state
  `workspace.stopped`; `lifecycle:remote_session` with state
  `remote.policy_blocked`.
- **Startup-state intersection.**
  `startup_state:unsupported_startup` per
  [`entry_restore_truth_audit.md`](../../../docs/ux/entry_restore_truth_audit.md)
  §6.9.
- **Truth class.** `runtime_observed_truth` on the session
  status; `workspace_vcs_truth` preserved locally.
- **Degraded-state token.** `PolicyBlocked` (composes to
  `Unsupported` on surfaces the policy fully excludes).

## Required axes rendered

- **Cause token.** `policy_blocked_restore` +
  `authority_revoked_by_admin_policy`.
- **Preserved.** Local-only workspaces remain reachable; the
  recovery journal; keyboard-reachable chrome; the Support
  Centre entry.
- **Blocked.** The remote workspace; the restore; any
  workspace-level command the admin policy disables.
- **Next-safe action hooks.** `review_trust_and_open`,
  `continue_in_restricted_mode`, `set_up_later`,
  `remove_from_recents` (when the policy stripped the
  recent-work row).

## Placement

- **Failure tier.** `tier.escalation_surface`.
- **Delivery surfaces.** `support_center_surface` entry card;
  `object_handoff_surface` for the prepared handoff packet;
  optional `os_notification` when critical-safety (e.g., a
  trust revocation on a managed device) applies — always
  obeying the `lock_screen_safe_generic` / `in_product_only`
  privacy-payload classes.
- **Interruptibility tier.** `tier_critical_safety` when the
  block affects trust or policy; otherwise
  `tier_blocking_trust`.

## Recovery affordances (keyboard-reachable)

- **`Open Support Center`** — surfaces the escalation entry
  and the prepared packet (`recovery_ladder_packet`,
  `policy_audit_evidence`, `object_issue_handoff`).
- **`Send object issue handoff`** — opens the approved
  browser-handoff flow (per ADR-0010) with redaction applied.
- **`Continue locally`** — confirms
  `rung.restricted_mode_fallback` and narrows the session to
  local-only paths.
- **`Reveal last failure`** — opens the rung-history
  inspector (prior rungs, denial classes, policy source
  refs).

## Last-failure reason (mandatory visibility)

- **Keyboard-reachable reveal.** Support Centre card and the
  escalation sheet both expose the rung-history inspector;
  reveal verb announced verbatim.
- **Support-export field.** `recovery_ladder_packet`,
  `policy_audit_evidence`, `object_issue_handoff`. Preserves
  every prior rung id, outcome token, policy source ref,
  authority class, and the monotonic timestamps.
- **Export-safe description.** Redaction pass runs; raw
  credentials, raw external URLs, raw policy bundles are not
  included on lock-screen payloads; in-product surfaces may
  show the sanitized policy id per the suspicious-content
  vocabulary.

## Promotion triggers

- **From `tier.session_recovery`.** No further product-side
  rung applies; admin policy explicitly blocks each remaining
  rung.
- **Terminal.** `tier.escalation_surface` has no further tier
  on the product side; the handoff packet carries the
  escalation-to-human expectations.

## Recovery / support / measurement

- **Recovery-ladder rung.** `rung.none_required` on the
  product side; the packet carries the human-side expectation.
- **Support-packet family.** `recovery_ladder_packet`,
  `policy_audit_evidence`, `object_issue_handoff`.
- **Journey-trace class.** `shell_open`.

## Accessibility

- The Support Centre card is keyboard-reachable from the
  session-recovery resolution focus-return.
- The browser-handoff affordance obeys the focus-return rule
  (dismiss returns to the triggering row).
- Notifications at this tier are announced per the a11y
  packet's critical-safety rule; they do not leak sensitive
  content to the lock screen.

## Forbidden copy and patterns on this path

- "Error"
- "Failed"
- "Unavailable"
- Raw secret material or raw external content on any
  notification payload (violates
  `lock_screen_payload_violates_privacy_class`)
- Surprise modal that the user did not reach through the
  explicit escalation affordance

## Expected observable outcomes

- The Support Centre entry is the explicit escalation
  affordance; the handoff packet is prepared with redaction
  applied.
- Last-failure reason and rung history are preserved on
  export.
- Local-only capability remains available; the user can
  still do useful work locally.
- `overclaims_readiness = false` is asserted.

## Fixture fields (seed)

```yaml
__fixture__:
  name: escalation_surface_policy_blocked
  taxonomy_rows:
    - tier.escalation_surface
    - empty:permission_or_policy_blocked
    - empty:unsupported_on_this_surface
    - lifecycle:workspace
    - lifecycle:remote_session
  startup_state_intersection: startup_state:unsupported_startup
  doc_section: docs/ux/state_and_recovery_taxonomy.md#7.5
running_build_identity_ref: build-identity-seed-state-escalation-policy
overclaims_readiness: false
```
