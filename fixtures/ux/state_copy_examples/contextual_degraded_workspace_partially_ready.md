# Fixture: contextual degraded — workspace Partially ready

## Scenario

The workspace has opened, but the managed-workspace lifecycle
reports `warming` and a language server has failed to attach on
first try. The workspace is interactive; the user can edit and
save. The shell renders the **`Partially ready`** controlled
label surface-wide with a status-item reveal for the last-
failure reason; no banner claims the workspace is fully ready.

## Row bindings

- **Failure-tier row.** `tier.contextual_degraded` per
  [`failure_tier_matrix.yaml`](../../../artifacts/ux/failure_tier_matrix.yaml)
  § `failure_tier_rows`.
- **Lifecycle row.** `lifecycle:workspace`, state
  `workspace.partially_ready`.
- **Startup-state intersection.**
  `startup_state:partial_startup` per
  [`entry_restore_truth_audit.md`](../../../docs/ux/entry_restore_truth_audit.md)
  §6.7.
- **Controlled label.** `Partially ready`.
- **Truth class.** `runtime_observed_truth` on the workspace
  status; `user_authored_durable_truth` on the buffer.
- **Degraded-state token.** `Partial` (composes with `Warming`
  while the language server is attaching).

## Required axes rendered

- **Cause token.** `missing_extension_host` +
  `language_server_unattached` (named on the status-item
  reveal; enumerated on their own rows, not collapsed into a
  generic "Something failed").
- **Preserved.** Editor interactivity; buffer edits; save
  pipeline; recovery journal; keyboard-reachable chrome;
  workspace-local search.
- **Narrowed.** `extension_host_offline`,
  `language_server_unattached`,
  `managed_workspace_not_ready`.
- **Next-safe action hooks.** `safe_mode`,
  `continue_in_restricted_mode`, `locate_missing_target`,
  `open_minimal`.

## Placement

- **Failure tier.** `tier.contextual_degraded`.
- **Delivery surfaces.** `contextual_banner` at the top of the
  workspace chrome; `status_item` pinned on the zone status bar
  with the last-failure reveal; `activity_center_digest_card`
  mirror for the warming / retry jobs.
- **Interruptibility tier.** `tier_durable`.
- **Focus.** The banner does not steal focus; the status item
  is reachable via the activity-rail focus cycle.

## Controlled label

- **Label text rendered.** "Partially ready" (reserved per
  taxonomy §8.6).
- **Supporting subtitle.** Names the warming language server
  and the missing extension host on distinct rows.

## Recovery affordances (keyboard-reachable)

- **`Repair cache / index`** — triggers
  `rung.cache_index_repair`.
- **`Retry language server attach`** — inline retry on the
  status item.
- **`Quarantine extension`** — triggers
  `rung.extension_quarantine` when the crashing participant is
  the extension.
- **`Continue in restricted mode`** — enters
  `rung.restricted_mode_fallback`.
- **`Open without restore`** — offered when a prior restore
  prompt remains pending.

## Last-failure reason (mandatory visibility)

- **Keyboard-reachable reveal.** Status-item drawer; reveal
  verb announced verbatim.
- **Support-export field.** Included on the support bundle
  (`recovery_ladder_packet`, `managed_workspace_evidence`);
  preserves the extension id, language-server id, last-attempt
  monotonic timestamp, and the recovery-rung history.
- **Export-safe description.** Redaction pass runs before
  bytes leave the product.

## Promotion triggers

- **To `tier.workflow_block`.** Consequence-bearing workflow
  (e.g., `Commit`, `Publish`, `Run task`) blocked on the
  partially-ready posture → a window-attached sheet names the
  block and routes to the appropriate rung.
- **To `tier.session_recovery`.** Repeated crash-loop on the
  extension host or the managed workspace → dialog-modal entry
  to safe mode / extension quarantine.

## Recovery / support / measurement

- **Recovery-ladder rungs.** `rung.cache_index_repair`,
  `rung.extension_quarantine`, `rung.restricted_mode_fallback`.
- **Support-packet family.** `recovery_ladder_packet`,
  `managed_workspace_evidence`, `object_issue_handoff`.
- **Journey-trace class.** `shell_open`,
  `startup_to_first_useful_chrome`.

## Accessibility

- "Partially ready" announced by assistive technology on first
  render; the narrowed-capability rows are separately
  addressable.
- The status-item drawer is keyboard-reachable without pointer
  affordances.

## Forbidden copy and patterns on this path

- "Workspace ready"
- "All extensions loaded"
- "Managed workspace is running"
- "Ready to edit"
- Undifferentiated "Something failed" banner
- Toast-only rendering (violates
  `toast_only_forbidden_for_durable_work`)

## Expected observable outcomes

- The controlled label `Partially ready` renders; all
  narrowed-capability axes are separately named.
- Last-failure reason is keyboard-reachable and preserved on
  export.
- Promotion to workflow block or session recovery only fires
  on a trigger; no silent modal appears.
- `overclaims_readiness = false` is asserted.

## Fixture fields (seed)

```yaml
__fixture__:
  name: contextual_degraded_workspace_partially_ready
  taxonomy_rows:
    - tier.contextual_degraded
    - lifecycle:workspace
  startup_state_intersection: startup_state:partial_startup
  controlled_label: "Partially ready"
  doc_section: docs/ux/state_and_recovery_taxonomy.md#7.2
running_build_identity_ref: build-identity-seed-state-ctx-partially-ready
overclaims_readiness: false
```
