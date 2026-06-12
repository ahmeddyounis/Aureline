## M5 command governance

This contract defines the export-safe packet that proves command-result, preview, disabled-reason, approval, activity-center, and export parity for the M5 depth-surface commands across:

- desktop
- CLI
- AI
- recipes
- extensions
- browser / companion handoff

The packet is emitted by [`aureline_commands::m5_command_governance`] and is seeded from the canonical merged command registry in `artifacts/commands/command_registry_seed.yaml` plus `artifacts/commands/m5_command_registry_seed.yaml`. Durable activity joins reuse the checked M5 activity-object audit in `fixtures/ux/m5/activity-center/report.json` so command-result rows point at the same exact-target reopen anchors and support-export identity that the shell activity center already publishes.

Rollout and flag truth is not synthesized locally. Each command row now quotes
the shared M5 rollout inventory in
`artifacts/commands/m5_rollout_inventory/packet.json`, which carries owner,
ring, cohort, expiry, promotion posture, kill-switch paths, and the stable
surface disclosure refs that Help/About, diagnostics, settings inspectors,
support exports, and docs/release consumers must preserve.

Each command row now also quotes the command-origin and lifecycle contract that M5 help, palette, tooltip, inspector, CLI/help, and support/export surfaces need to share:

- origin disclosure: `source_display_label`, `origin_class`, `source_ref`, `publisher_ref`, runtime provenance, bundle/pack refs, and bridge/native posture
- lifecycle disclosure: `lifecycle_state`, support class, release channel, freshness class, stability label, visible migration/experiment state, and rollout ref
- rollout governance: capability id, owner, rollout ring, cohort, review/expiry date, declared/effective state, promotion posture, kill-switch paths, affected capability ids, and stable-facing disclosure refs
- alias lifecycle: alias state, canonical resolution, replacement posture, new-binding eligibility, and migration note refs
- copy-safe inspection actions: copy command ID, copy CLI skeleton, copy recipe-step template, inspect origin, inspect lifecycle, inspect capability class, and inspect why-not-automatable detail refs
- route provenance: origin scope, client-scope label, authority-boundary ref, and explicit browser/companion handoff packet refs when the route narrows to desktop handoff

### Required invariants

- Every M5 command has a row for all six required routes.
- High-risk or approval-gated M5 commands declare `preview_gate_metadata` in the registry entry.
- Every route preserves the descriptor-owned preview class, approval posture, and no-bypass contract.
- Every command row carries one canonical result-packet governance record with the shared invocation schema, result schema, outcome vocabulary, and export posture.
- Every command row carries one origin disclosure record, one lifecycle disclosure record, and one capability-class inspect ref so command source and maturity remain copy-safe and exportable.
- Stable aliases keep lifecycle state visible, and deprecated/retired aliases require explicit replacement posture instead of silently drifting.
- Canonical outcomes cover `success`, `partial_success`, `cancelled`, `superseded`, `denied`, `degraded`, and `failed`.
- Long-running or exact-target-reopen rows join the M5 activity-object contract and preserve the shell-owned reopen anchor instead of inventing a surface-local history object.
- Commands that claim rollback or checkpoint posture preserve those refs in the result contract and keep the release/support joins stable.
- Denial and approval packets preserve `actor_ref`, `target_ref`, `trust_epoch_ref`, and `rollout_state_ref`.
- Disabled-by-policy command families narrow route posture through the shared rollout row rather than by surface-local flags.
- Copy-safe command introspection stays descriptor-backed: `Copy command ID`, `Copy CLI form`, `Copy recipe step`, `Inspect origin`, `Inspect lifecycle`, `Inspect capability class`, and `Why not automatable?` are projected from the same command truth.
- Browser / companion rows that cannot run directly preserve a typed handoff packet ref instead of implying local execution authority.
- Copy-safe summaries, raw packet export refs, support-export case refs, and release-evidence refs stay export-safe and reusable.

### Generated artifacts

- Packet fixture: `fixtures/commands/m5_command_governance/packet.json`
- Support export: `artifacts/commands/m5_command_governance/support_export.json`
- Summary: `artifacts/commands/m5_command_governance/summary.md`
- Shared rollout source: `artifacts/commands/m5_rollout_inventory/packet.json`
- Schema: `schemas/commands/m5_command_governance.schema.json`

### Regeneration

```bash
cargo run -q -p aureline-commands --bin aureline_commands -- m5-command-governance json
cargo run -q -p aureline-commands --bin aureline_commands -- m5-command-governance support-export
cargo run -q -p aureline-commands --bin aureline_commands -- m5-command-governance summary
```
