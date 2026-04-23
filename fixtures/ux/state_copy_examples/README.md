# State copy examples

Seed corpus for the taxonomy frozen in
[`/docs/ux/state_and_recovery_taxonomy.md`](../../../docs/ux/state_and_recovery_taxonomy.md)
and the machine-readable matrix in
[`/artifacts/ux/failure_tier_matrix.yaml`](../../../artifacts/ux/failure_tier_matrix.yaml).

Each file is a reviewer-facing worked example. A fixture is a
**seed**: it pins the state-row token, the failure-tier placement
(when applicable), the controlled-label, the preserved / narrowed
capability split, the minimum recovery affordances, the
keyboard-reachable last-failure affordance, and the support /
measurement hooks that later docs, support, diagnostics, and
release-evidence surfaces import by id.

Every fixture:

- Resolves every axis to vocabulary already frozen in the
  taxonomy or in the re-exported upstream contracts
  (attention/activity taxonomy, recovery-ladder packet,
  entry/restore object model, truth-and-degraded-state
  vocabulary, a11y packet).
- Cites at least one `next_step_decision_hook`, one
  recovery-ladder rung id (or `rung.none_required`), one
  support-packet family, one failure-tier id (when applicable),
  and one fixture-level `overclaims_readiness = false` assertion.
- Asserts the last-failure reason is keyboard-reachable and
  preserved on support export where a last-failure reason
  exists.
- Carries no raw absolute paths, raw URLs, raw credential
  material, raw prompt text, or raw logs. Every identity is an
  opaque ref; every timestamp is a monotonic placeholder.
- Honours the forbidden-generic-copy rule on protected paths —
  "Working…", "Something went wrong", "Error", "Failed",
  "Try again", "Unavailable" are quoted only in the
  forbidden-copy section, never in the rendered copy.

## Cases

| Fixture | Primary state-row class | Failure tier | Controlled label |
| --- | --- | --- | --- |
| [`empty_first_run.md`](./empty_first_run.md) | `empty:no_work_started` | — | — |
| [`empty_filter_narrowed.md`](./empty_filter_narrowed.md) | `empty:filter_narrowed_to_zero` | — | — |
| [`loading_warming_index.md`](./loading_warming_index.md) | `loading:progressive_partial_results`, `loading:top_of_pane_progress_indicator`, `loading:skeleton_row`, `loading:inline_placeholder` | — | `Partially ready` |
| [`inline_issue_save_error.md`](./inline_issue_save_error.md) | `lifecycle:workspace`, `empty:failed_last_attempt` | `tier.inline_issue` | — |
| [`contextual_degraded_workspace_partially_ready.md`](./contextual_degraded_workspace_partially_ready.md) | `lifecycle:workspace` | `tier.contextual_degraded` | `Partially ready` |
| [`contextual_degraded_extension_quarantined.md`](./contextual_degraded_extension_quarantined.md) | `lifecycle:extension` | `tier.contextual_degraded` | `Degraded` |
| [`contextual_degraded_remote_read_only.md`](./contextual_degraded_remote_read_only.md) | `lifecycle:remote_session` | `tier.contextual_degraded` | `Read-only degraded` |
| [`workflow_block_ai_apply_blocked.md`](./workflow_block_ai_apply_blocked.md) | `lifecycle:ai_action` | `tier.workflow_block` | — |
| [`session_recovery_crash_loop_safe_mode.md`](./session_recovery_crash_loop_safe_mode.md) | `lifecycle:workspace`, `lifecycle:update_rollback` | `tier.session_recovery` | — |
| [`escalation_surface_policy_blocked.md`](./escalation_surface_policy_blocked.md) | `empty:permission_or_policy_blocked`, `empty:unsupported_on_this_surface` | `tier.escalation_surface` | — |

## Schema references

- Taxonomy doc:
  [`/docs/ux/state_and_recovery_taxonomy.md`](../../../docs/ux/state_and_recovery_taxonomy.md).
- Failure-tier matrix:
  [`/artifacts/ux/failure_tier_matrix.yaml`](../../../artifacts/ux/failure_tier_matrix.yaml).
- Attention / activity taxonomy:
  [`/docs/ux/attention_activity_taxonomy.md`](../../../docs/ux/attention_activity_taxonomy.md).
- Recovery-ladder packet:
  [`/docs/support/recovery_ladder_packet.md`](../../../docs/support/recovery_ladder_packet.md).
- Truth and degraded-state vocabulary:
  [`/docs/governance/truth_and_degraded_state_vocabulary.md`](../../../docs/governance/truth_and_degraded_state_vocabulary.md).
- Entry / restore truth audit:
  [`/docs/ux/entry_restore_truth_audit.md`](../../../docs/ux/entry_restore_truth_audit.md).

## Build identity

Every fixture reserves `running_build_identity_ref` as an opaque
seed id. A later lane resolves it against the build-identity
record without renaming the field.
