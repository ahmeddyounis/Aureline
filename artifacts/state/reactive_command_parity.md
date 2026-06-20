# Reactive command parity packet

The canonical reactive-command-parity packet is implemented in
[`crates/aureline-reactive-state/src/reactive_command_parity/mod.rs`](../../crates/aureline-reactive-state/src/reactive_command_parity/mod.rs)
and serialized to
[`artifacts/state/reactive_command_parity.json`](./reactive_command_parity.json).

It is the checked-in truth source for:

- per-surface publication gate, optimistic posture, preserved lineage, and
  divergence resolution in
  [`docs/state/reactive_command_parity.md`](../../docs/state/reactive_command_parity.md)
- the request-to-publish-or-divergence drills in
  [`artifacts/state/reactive_command_parity_drills.md`](./reactive_command_parity_drills.md)
- metadata-safe support export in
  [`crates/aureline-support/src/reactive_command_parity/mod.rs`](../../crates/aureline-support/src/reactive_command_parity/mod.rs)
- fixture replay in
  [`crates/aureline-reactive-state/tests/reactive_command_parity.rs`](../../crates/aureline-reactive-state/tests/reactive_command_parity.rs)
- support-export replay in
  [`crates/aureline-support/tests/reactive_command_parity_support_export.rs`](../../crates/aureline-support/tests/reactive_command_parity_support_export.rs)

## Frozen evidence

The packet proves:

- every claimed mutating surface publishes user-visible state only after the
  command graph commits, the mutation journal commits, and the reactive graph
  republishes — never from a private optimistic cache
- each surface declares whether its optimistic path is never present, removed, or
  quarantined behind the publication gate, so a local prediction can never
  outvote the canonical command, approval, or journal outcome
- every flow preserves actor, scope, command, and checkpoint lineage so
  diagnostics and support packets can correlate the published state with what
  produced it
- known divergence cases resolve to an explicit degraded or waiting state instead
  of a hidden cache win
- one drill per surface walks request → pending → command commit → journal commit
  → publish, or request → command commit → diverge for the honest-divergence path

## Parity flows

| Flow | Surface | Mutation | Optimistic posture | Pre-publish state | Divergence resolution |
| --- | --- | --- | --- | --- | --- |
| `ai_apply_edit` | AI apply | `apply_edit` | `optimistic_quarantined` | `pending` | `revert_to_canonical` |
| `review_approve_action` | review action | `approve_action` | `never_optimistic` | `waiting_state` | `hold_and_wait` |
| `scaffold_update_artifact` | scaffold update | `scaffold_artifact` | `optimistic_removed` | `pending` | `revert_to_canonical` |
| `provider_config_mutation` | provider mutation | `provider_config_change` | `optimistic_removed` | `pending` | `degrade_surface` |
| `notebook_execute_cell` | notebook result | `execute_cell` | `optimistic_quarantined` | `pending` | `hold_and_wait` |
| `support_repair_state` | support repair | `repair_state` | `never_optimistic` | `waiting_state` | `degrade_surface` |

Every row carries `publishes_after_command_commit = true`,
`publishes_after_journal_commit = true`, `publishes_via_reactive_graph = true`,
`claims_success_before_publish = false`, and `support_correlatable = true`.

## Fixture corpus

The fixture corpus under
[`fixtures/state/reactive_command_parity/`](../../fixtures/state/reactive_command_parity/)
pins one scenario per flow. Each fixture binds the expected optimistic posture,
divergence resolution, pre-publish visibility, and the
`claims_success_before_publish = false` guardrail back to its flow so drift
between the packet and the fixtures fails CI.

## Export posture

Every support-export row produced from this packet keeps:

- `raw_payload_excluded = true`
- `ambient_authority_excluded = true`
- explicit `optimistic_posture`, `divergence_resolution`, `preserved_lineage`,
  and the publication-gate flags
- support-safe summaries for both `publication_summary` and `parity_rationale`
