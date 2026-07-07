# M5 AI Action-State-Banner and Boundary-Blocked-Banner Primitive

- Packet: `m5-ai-action-state-banner-primitive:stable:0001`
- Label: `M5 AI action-state banner and boundary-blocked-banner primitive: execution mode, action state, scope reach, placement, approval posture, operator controls, and boundary-blocked safe alternatives`
- Banner consumers: 5 (5 stable)
- Banner postures: active_within_scope, active_awaiting_approval, paused_mid_run, boundary_blocked, completed_clear, failed_needs_attention, idle_ready
- Scope reaches: single_selection, current_file, reviewed_file_set, workspace_scoped, connector_scoped, cross_workspace_scoped
- Blocked boundaries: reviewed_file_scope, connector_boundary, policy_fence, credential_boundary, network_egress_fence, cross_workspace_scope
- Safe alternatives: narrow_to_reviewed_scope, request_connector_approval, request_scoped_credential, split_into_approved_steps, stay_within_current_workspace, run_read_only_preview
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Banner consumers

- **Inline Explain/Fix**: `stable`
  - Owner: Inline explain/fix owner
  - Scope: The inline explain/fix overlay renders the shared banner so a foreground-assistant explanation reads as active-within-scope with open-plan / pause / cancel controls, while a guided-patch fix that would write beyond the reviewed file scope reads as boundary-blocked with a banner naming the reviewed-file-scope boundary and a narrow-to-reviewed-scope safe next action rather than a generic model error
  - Worked resolutions: 2
    - `streaming` in `foreground_assistant` → `active_within_scope` (reach `single_selection`, gate `auto_approved`, boundary `clear`)
    - `boundary_blocked` in `guided_patch` → `boundary_blocked` (reach `current_file`, gate `one_click_confirm`, boundary `reviewed_file_scope`)
- **Assistant Panel**: `stable`
  - Owner: Assistant-panel owner
  - Scope: The assistant panel renders the shared banner so a foreground-assistant workspace edit behind a high-friction typed confirmation reads as active-awaiting-approval, while a paused mid-run edit reads as paused with resume / cancel controls — the mode, reach, and approval visible without a secondary inspector
  - Worked resolutions: 2
    - `awaiting_approval` in `foreground_assistant` → `active_awaiting_approval` (reach `workspace_scoped`, gate `high_friction_typed`, boundary `clear`)
    - `paused` in `foreground_assistant` → `paused_mid_run` (reach `current_file`, gate `notify_only`, boundary `clear`)
- **Patch-Review Lane**: `stable`
  - Owner: Patch-review lane owner
  - Scope: The patch-review lane renders the shared banner so a review-first patch that finished reads as completed-clear, while a guided-patch tool run that a policy fence blocks reads as boundary-blocked with a banner naming the policy-fence boundary and a split-into-approved-steps safe next action rather than a generic tool failure
  - Worked resolutions: 2
    - `completed` in `review_first_placement` → `completed_clear` (reach `reviewed_file_set`, gate `one_click_confirm`, boundary `clear`)
    - `tool_running` in `guided_patch` → `boundary_blocked` (reach `reviewed_file_set`, gate `policy_blocked`, boundary `policy_fence`)
- **Branch / Worktree Agent**: `stable`
  - Owner: Branch / worktree agent owner
  - Scope: The branch/worktree agent surface renders the shared banner so a background agent tool run behind a two-person review reads as active-awaiting-approval with pause / take-over / cancel controls, while a background agent that would cross a connector boundary reads as boundary-blocked with a banner naming the connector boundary and a request-connector-approval safe next action and an explicit take-over path
  - Worked resolutions: 2
    - `tool_running` in `background_branch_agent` → `active_awaiting_approval` (reach `connector_scoped`, gate `two_person_review`, boundary `clear`)
    - `boundary_blocked` in `background_branch_agent` → `boundary_blocked` (reach `connector_scoped`, gate `one_click_confirm`, boundary `connector_boundary`)
- **CLI / Support Export**: `stable`
  - Owner: CLI / support export owner
  - Scope: The CLI / support export renders the shared banner so a headless automation run that failed reads as failed-needs-attention with open-plan / cancel controls, and an idle headless banner reads as idle-ready — the mode, reach, action state, and approval reconstructable from the support export alone
  - Worked resolutions: 2
    - `failed` in `headless_automation` → `failed_needs_attention` (reach `cross_workspace_scoped`, gate `auto_approved`, boundary `clear`)
    - `idle` in `headless_automation` → `idle_ready` (reach `single_selection`, gate `auto_approved`, boundary `clear`)
