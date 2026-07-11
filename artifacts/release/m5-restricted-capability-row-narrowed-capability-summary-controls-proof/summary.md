# M5 Restricted-Capability-Row and Narrowed-Capability-Summary Controls

- Packet: `m5-restricted-capability-row-narrowed-capability-summary-controls:stable:0001`
- Label: `M5 restricted-capability rows and narrowed-capability summaries with blocked action families, still-safe actions, restriction reason, and command-backed recovery paths`
- Consumer surfaces: 5
- Restriction scopes: trusted_workspace, trusted_root, restricted_workspace, mixed_root, policy_blocked, scope_unknown
- Action families: code_execution, task_automation, extension_activation, debugger_attach, workspace_settings_write, outbound_requests, file_editing, read_only_navigation
- Recovery actions: inspect_trust, reopen_restricted, continue_limited, request_approval, review_diagnostics, no_recovery_needed
- Proof freshness SLO: 720 hours (last refresh: 2026-07-11T00:00:00Z)

## Consumer surfaces

- **workspace_trust_ui**: `stable`
  - Owner: Workspace trust owner
  - Scope: The workspace-trust UI renders one restricted-capability row enumerating blocked action families, still-safe actions, and why the restriction exists for a restricted and a mixed-root workspace, plus a narrowed-capability summary keeping mixed-root restriction explicit rather than uniform
  - Row examples: 2 / summary examples: 2
- **settings_ui**: `stable`
  - Owner: Settings trust owner
  - Scope: The settings trust pane reuses the same field and recovery grammar for a policy-blocked object, names the narrowed capability a restriction removes, and degrades honestly when the narrowed capability is unnamed or the summary collapses distinct blocked families
  - Row examples: 2 / summary examples: 2
- **safe_mode_ui**: `stable`
  - Owner: Safe mode owner
  - Scope: Safe mode shows the task-blocked restricted row with its still-safe actions and a restricted summary, degrading honestly when the restriction scope cannot be resolved or a still-safe action is not named
  - Row examples: 2 / summary examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved row and summary truth, so a restriction collapsed into generic unavailable copy or a missing command-backed recovery path is visible in evidence rather than hidden
  - Row examples: 2 / summary examples: 2
- **product_ui**: `stable`
  - Owner: In-product restricted owner
  - Scope: In-product surfaces reuse the same restriction, still-safe, and command-backed recovery grammar a user sees in the workspace-trust UI, always keeping inspect-trust reachable and degrading honestly when object, source, reason, blocked families, still-safe actions, or per-root scope is unstated
  - Row examples: 7 / summary examples: 3
