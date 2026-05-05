# Cross-window transfer worked-example corpus

Reviewable fixtures for the contract frozen in
[`/docs/ux/cross_window_transfer_contract.md`](../../../docs/ux/cross_window_transfer_contract.md).
Each JSON fixture conforms to
[`/schemas/ux/window_transfer_action.schema.json`](../../../schemas/ux/window_transfer_action.schema.json).

The corpus pins how pre-drop verb previews, command-backed fallbacks,
workspace/trust/host/profile continuity, secondary-window orphan
prevention, and restore/degraded fallback behave across multi-window
work.

## Fixture Rules

- Every fixture states the resolved `transfer_action_class`, the
  `transfer_phase`, and the exact preview fields visible before commit
  or denial.
- Every fixture includes a command-backed fallback, even when drag/drop
  succeeds.
- Every fixture states which workspace identity, trust, host/remote,
  profile, collaboration, and recovery cues remain visible.
- Every fixture includes an orphan-prevention posture, even when no
  critical state is at risk.
- Restore fixtures distinguish recovered layout/evidence from live
  authority and never claim hidden rerun or silent reauthorization.

## Index

| Fixture | Coverage | Expected truth |
|---|---|---|
| `move_tab_dirty_buffer_continuation.json` | move tab with dirty/recovered state | source loses tab membership; dirty authority remains visible in target |
| `move_followed_readonly_tab_breakaway.json` | move followed read-only tab | Following/read-only cues remain visible; breakaway never happens implicitly |
| `copy_editor_secondary_window_continuity.json` | copy editor to secondary window | target gets a new view identity over the same canonical object |
| `create_window_review_evidence_continuation.json` | create review window with evidence review | new window joins the same family and owns a durable continuation |
| `reopen_after_crash_secondary_compare.json` | reopen specialized compare window after crash | compare roles and evidence restore without hidden live authority |
| `degraded_missing_remote_target_placeholder.json` | degraded fallback after prerequisite disappears | missing remote/provider state becomes a placeholder with rebind action |

## Coverage Contract

The seeded fixture set keeps at least one case for each required
surface:

- move;
- copy;
- create-window;
- reopen after crash;
- degraded fallback when prerequisites disappear.
