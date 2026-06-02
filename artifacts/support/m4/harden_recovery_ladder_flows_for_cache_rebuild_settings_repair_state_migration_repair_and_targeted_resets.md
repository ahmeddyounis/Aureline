# Hardened recovery-ladder flows for cache rebuild, settings repair, state-migration repair, and targeted resets — Artifact

## Status

**Stable** — hardened M4 recovery-ladder flow contract with typed blast
radius, reversal, checkpoint, preserved-state, and impacted-state
vocabularies.

## Checked-in outputs

| Output | Path |
|--------|------|
| Implementation | `crates/aureline-support/src/harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets/mod.rs` |
| Boundary schema | `schemas/support/harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets.schema.json` |
| Reviewer doc | `docs/support/m4/harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets.md` |
| Fixture corpus | `fixtures/support/m4/harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets/` |

## What is hardened

Four recovery-ladder rung families are promoted to stable contracts:

1. **Cache rebuild** — regenerates disposable derived cache or index from
   authoritative state. Blast radius is `single_disposable_state_class`;
   reversal is `regenerate` or `exact`; checkpoint is `durable_pre_apply`.
2. **Settings repair** — repairs settings/profile state from backup.
   Blast radius is `same_family_state_classes`; reversal is `compensating`
   or `exact`; checkpoint is `durable_pre_apply`.
3. **State-migration repair** — repairs or rolls back a failed schema
   migration. Blast radius is `cross_family_state_classes`; reversal is
   `manual` or `compensating`; checkpoint is `durable_pre_apply`; admin
   consent is required.
4. **Targeted reset** — resets exactly one targeted disposable state class.
   Blast radius is `single_disposable_state_class`; reversal is `exact`;
   checkpoint is `durable_pre_apply`; exactly one impacted state class is
   declared.

## Seeded support scenarios

The fixture corpus covers four flow families:

- `cache_rebuild` — `hardened_recovery_flow:cache_rebuild.alpha_v1`
- `settings_repair` — `hardened_recovery_flow:settings_repair.alpha_v1`
- `state_migration_repair` — `hardened_recovery_flow:state_migration_repair.alpha_v1`
- `targeted_reset` — `hardened_recovery_flow:targeted_reset.alpha_v1`

Each fixture includes:
- one hardened flow record,
- a `doctor.finding.*` ref,
- preserved and impacted state classes,
- blast-radius, reversal, and checkpoint declarations,
- evidence refs and support guidance.

## Verification

Run the protected tests:

```bash
cargo test -p aureline-support --test harden_recovery_ladder_flows
```

## Risks and follow-ups

- Live runtime enforcement of recovery-ladder entry/exit is out of scope
  and lands with the chrome/runtime consumers.
- Automatic escalation routing to support tickets is not covered; only
  typed escalation refs are preserved.
- Cross-tenant state-migration reconciliation remains unsupported.
