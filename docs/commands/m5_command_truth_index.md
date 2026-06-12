## M5 command truth index

This contract defines the downstream-facing evidence index that binds the M5
command-governance packet, capability-state truth packet, and rollout inventory
into one canonical source for:

- Help / About lifecycle and rollout truth
- release-center claim rows
- support exports and escalation packets
- public-truth and docs publication rows

The runtime owner is [`aureline_commands::m5_command_truth_index`]. It does not
invent a second maturity ladder. Instead it reuses:

- `artifacts/commands/m5_command_governance/support_export.json`
- `artifacts/commands/m5_capability_state_truth/support_export.json`
- `artifacts/commands/m5_rollout_inventory/packet.json`

and projects one typed row per claimed M5 command family. Each row carries:

- the effective lifecycle state and downstream truth posture
- parity completeness for authority and result-packet reuse
- explicit help/About, release-center, support-export, and public-truth refs
- evidence refs that support, release, and docs consumers join directly

### Required invariants

- Every claimed M5 command family has exactly one row in the index.
- Every row covers `help_about`, `release_center`, `support_export`, and
  `public_truth`.
- Stable wording is allowed only when the effective lifecycle state is
  `Stable`, authority parity is complete, result-packet reuse is complete,
  lifecycle truth remains visible, and no findings are present.
- `DisabledByPolicy` and `RetestPending` rows remain visible with typed
  narrowing reasons instead of disappearing or inheriting green support copy.
- Lifecycle-below-stable rows stay explicit as narrowed instead of inheriting
  stable wording from adjacent packets.
- Downstream consumers read projection refs and evidence refs from this index
  rather than re-deriving them from only one upstream packet.

### Generated artifacts

- Packet artifact: `artifacts/commands/m5_command_truth_index/packet.json`
- Packet fixture: `fixtures/commands/m5_command_truth_index/packet.json`
- Support export: `artifacts/commands/m5_command_truth_index/support_export.json`
- Summary: `artifacts/commands/m5_command_truth_index/summary.md`
- Schema: `schemas/commands/m5_command_truth_index.schema.json`

### Regeneration

```bash
cargo run -q -p aureline-commands --bin aureline_commands -- m5-command-truth-index json
cargo run -q -p aureline-commands --bin aureline_commands -- m5-command-truth-index support-export
cargo run -q -p aureline-commands --bin aureline_commands -- m5-command-truth-index summary
```
