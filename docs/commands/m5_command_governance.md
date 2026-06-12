## M5 command governance

This contract defines the export-safe packet that proves preview, disabled-reason, and approval parity for the M5 depth-surface commands across:

- desktop
- CLI
- AI
- recipes
- extensions
- browser / companion handoff

The packet is emitted by [`aureline_commands::m5_command_governance`] and is seeded from the canonical merged command registry in `artifacts/commands/command_registry_seed.yaml` plus `artifacts/commands/m5_command_registry_seed.yaml`.

### Required invariants

- Every M5 command has a row for all six required routes.
- High-risk or approval-gated M5 commands declare `preview_gate_metadata` in the registry entry.
- Every route preserves the descriptor-owned preview class, approval posture, and no-bypass contract.
- Denial and approval packets preserve `actor_ref`, `target_ref`, `trust_epoch_ref`, and `rollout_state_ref`.
- Copy-safe command introspection stays descriptor-backed: `Copy command ID`, `Copy CLI form`, `Add to recipe`, and `Why not automatable?` are projected from the same command truth.
- Every exported denial, preview, and approval packet is support-safe.

### Generated artifacts

- Packet fixture: `fixtures/commands/m5_command_governance/packet.json`
- Support export: `artifacts/commands/m5_command_governance/support_export.json`
- Summary: `artifacts/commands/m5_command_governance/summary.md`
- Schema: `schemas/commands/m5_command_governance.schema.json`

### Regeneration

```bash
cargo run -q -p aureline-commands --bin aureline_commands -- m5-command-governance json
cargo run -q -p aureline-commands --bin aureline_commands -- m5-command-governance support-export
cargo run -q -p aureline-commands --bin aureline_commands -- m5-command-governance summary
```
