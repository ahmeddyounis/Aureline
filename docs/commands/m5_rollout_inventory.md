## M5 rollout inventory

This contract defines the canonical M5 rollout inventory packet for the
optional depth-surface command families. It is the shared rollout source of
truth for:

- command-governance parity rows
- capability-state truth projections
- Help / About rollout summaries
- diagnostics and support exports
- settings-inspector and docs/release disclosure refs

Each row binds:

- `command_id` and `capability_id`
- `owner_ref`
- `rollout_ring` and `cohort`
- `review_or_expiry_date`
- declared and effective lifecycle state
- `promotion_state`
- `rollout_state_ref`
- explicit kill-switch paths
- affected capability-family ids
- stable-facing disclosure refs for settings, help/About, diagnostics,
  support-export, and docs/release consumers

### Required invariants

- Every claimed optional M5 command family has exactly one rollout row.
- Every row has an owner, ring, cohort, review/expiry date, and at least one
  kill-switch path.
- Stable claims may only remain allowed when the effective state is `stable`.
- Rows may not depend on hidden rollout state; `no_hidden_flag_rule_satisfied`
  must remain true.
- Stable-facing consumers must preserve the same narrowing truth instead of
  flattening beta, preview, Labs, policy-blocked, or retest-pending posture.

### Generated artifacts

- Packet artifact: `artifacts/commands/m5_rollout_inventory/packet.json`
- Packet fixture: `fixtures/commands/m5_rollout_inventory/packet.json`
- Support export: `artifacts/commands/m5_rollout_inventory/support_export.json`
- Summary: `artifacts/commands/m5_rollout_inventory/summary.md`
- Schema: `schemas/commands/m5_rollout_inventory.schema.json`

### Regeneration

```bash
cargo run -q -p aureline-commands --bin aureline_commands -- m5-rollout-inventory json
cargo run -q -p aureline-commands --bin aureline_commands -- m5-rollout-inventory support-export
cargo run -q -p aureline-commands --bin aureline_commands -- m5-rollout-inventory summary
```
