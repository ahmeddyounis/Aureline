# M5 AI action-state-banner and boundary-blocked-banner primitive contract

Task: **M05-877** — Implement AI action-state banners and boundary-blocked banners with
mode / scope / placement / approval and fallback truth across the claimed M5 inline,
panel, review, and background-agent surfaces.

This lane narrows the `ai_action_state_banner` family from the frozen
[AI-execution/replay component matrix](./freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix.md)
(M05-876) into one reusable primitive: a resolver plus a parity matrix. A user can tell
— from the banner and its boundary-blocked state alone — what AI execution mode is
active, how far it can reach, where it is placed, what approval applies, and what
operator controls exist, before mistaking an explanation for a mutation or hitting a
generic model or tool error.

## Primitive

- Resolver: `resolve_action_state_banner(&M5AiBannerResolutionInput) -> Result<M5ResolvedActionStateBanner, M5AiBannerResolutionError>`.
- Parity matrix packet: `M5AiActionStateBannerPrimitivePacket`, one row per claimed
  banner consumer.

### Derived banner posture ladder (blocking-first)

1. A crossed boundary (`blocked_boundary` present, a `boundary_blocked` action state, or
   a `policy_blocked` approval gate) → **boundary_blocked** — and the banner carries a
   self-contained `M5AiBoundaryBlockedBanner` naming the exact boundary and the next
   safe alternative.
2. `failed` action state → **failed_needs_attention**.
3. `paused` action state → **paused_mid_run**.
4. `awaiting_approval` action state, or a `high_friction_typed` / `two_person_review`
   gate → **active_awaiting_approval**.
5. `composing` / `streaming` / `tool_running` → **active_within_scope**.
6. `completed` → **completed_clear**.
7. otherwise (`idle`) → **idle_ready**.

Placement and approval are carried explicitly and are never inferred from the execution
mode. A blocked request always produces a named boundary and a narrower safe action,
never a generic `model error` or `tool failed`.

### Blocked boundary → safe alternative

| Blocked boundary | Safe alternative |
| --- | --- |
| `reviewed_file_scope` | `narrow_to_reviewed_scope` |
| `connector_boundary` | `request_connector_approval` |
| `policy_fence` | `split_into_approved_steps` |
| `credential_boundary` | `request_scoped_credential` |
| `network_egress_fence` | `run_read_only_preview` |
| `cross_workspace_scope` | `stay_within_current_workspace` |

### Resolver errors

`empty_banner_label`, `empty_scope_repr`, `empty_operator_controls` (a user is never left
without a control), `boundary_blocked_without_boundary` (a blocked action state or a
policy-blocked gate must name its boundary), and `forbidden_banner_material`.

## Claimed consumer surfaces

`inline_explain_fix`, `assistant_panel`, `patch_review`, `branch_worktree_agent`, and
`cli_support_export`. Every row reuses the shared banner anatomy, the same postures /
reaches / boundaries / safe alternatives / operator controls, the same mandatory export
fields, and a non-visual accessibility route, so the mode/scope/approval vocabulary stays
identical across inline, panel, review, and agent surfaces.

## Hard invariants (per row, all must be false)

- `masks_execution_mode_or_reach`
- `shows_boundary_crossing_as_allowed`
- `emits_generic_model_or_tool_error`
- `hides_operator_controls_or_takeover`

## Acceptance-criterion lints

- `posture_coverage_unproven` — at least one worked resolution proves an active banner
  and at least one proves a boundary-blocked banner.
- `mode_and_reach_explicit_unproven` — at least one active resolution shows its mode, its
  reach, and an immediate steering control (open plan, pause, or cancel).
- `boundary_blocked_self_contained_unproven` — at least one boundary-blocked resolution
  names its boundary, its safe alternative, and a non-empty headline.

## Reused vocabulary (frozen in M05-876)

`M5AiExecutionMode`, `M5AiActionState`, `M5AiApprovalGate`, `M5AiSurfaceFamily`,
`M5AiDeploymentLine`, `M5AiConsumerSurface`, `M5AiAccessibilityRoute`,
`M5AiQualificationClass`, and `M5AiExecutionDowngradeTrigger`.

## Artifacts

- Boundary schema: `schemas/ai/m5-ai-action-state-banner.schema.json`.
- Support export (canonical): `artifacts/ai/m5/implement_ai_action_state_banners_and_boundary_blocked_banners_across_claimed_m5_inline_panel_review_agent_surfaces/support_export.json`.
- Matrix CSV and Markdown report alongside the support export.
- Narrowed fixtures under `fixtures/ai/m5/implement_ai_action_state_banners_and_boundary_blocked_banners_across_claimed_m5_inline_panel_review_agent_surfaces/`.
- Headless emitter: `cargo run -p aureline-ai --bin aureline_ai_action_state_banner_primitive -- <support-export|report|csv|validate|fixture-...>`.
