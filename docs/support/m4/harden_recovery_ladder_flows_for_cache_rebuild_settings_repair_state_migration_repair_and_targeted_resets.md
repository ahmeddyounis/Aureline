# Hardened recovery-ladder flows for cache rebuild, settings repair, state-migration repair, and targeted resets

This document defines the hardened recovery-ladder flow contract that promotes
four rung families from alpha to stable, typed, and exportable systems.

## What this row owns

- The [`HardenedRecoveryFlowRecord`] that binds one flow class, one recovery-
  ladder rung, a blast-radius class, a reversal class, a checkpoint class,
  preserved and impacted state classes, consent requirements, and reviewer-
  facing guidance.
- The [`HardenedRecoveryFlowEvaluator`] that validates each flow family:
  - **Cache rebuild** — blast radius is `single_disposable_state_class`,
    reversal is `regenerate` or `exact`, rung is `cache_index_repair`.
  - **Settings repair** — blast radius is `same_family_state_classes` or
    `single_disposable_state_class`, reversal is `compensating` or `exact`,
    rung is `settings_repair`, durable checkpoint is required.
  - **State-migration repair** — blast radius is `cross_family_state_classes`
    or `escalation_only`, reversal is `manual` or `compensating`, rung is
    `state_migration_repair`, durable checkpoint is required, admin consent
    is required.
  - **Targeted reset** — blast radius is `single_disposable_state_class`,
    reversal is `exact`, rung is `targeted_reset`, exactly one impacted
    state class is declared, durable checkpoint is required.
- The [`HardenedRecoveryFlowSupportPacket`] that folds validated flows into
  a metadata-only projection.
- The boundary schema at
  [`/schemas/support/harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets.schema.json`](../../schemas/support/harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets.schema.json).
- The protected fixture corpus at
  [`/fixtures/support/m4/harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets/`](../../fixtures/support/m4/harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets/).

## Acceptance and how this row meets it

- **Every flow preserves `user_authored_files`.** The evaluator refuses any
  flow that does not list `user_authored_files` in `preserved_state_classes`.
- **Every flow cites a `doctor.finding.*` ref.** The evaluator requires a
  non-empty `doctor_finding_ref` starting with `doctor.finding.`.
- **Cache rebuild limits mutation to disposable state.** The evaluator
  requires `blast_radius_class: single_disposable_state_class` and binds
  the flow to `CacheIndexRepair`.
- **Settings repair preserves settings profile state and requires a durable
  checkpoint.** The evaluator requires `durable_pre_apply` checkpoint and
  `settings_profile_state` preservation.
- **State-migration repair preserves durable indexes and migration journal.**
  The evaluator requires `durable_workspace_indexes` and
  `state_migration_journal` preservation, plus a durable checkpoint.
- **Targeted reset declares exactly one impacted state class.** The evaluator
  requires `impacted_state_classes` to have length exactly 1 and binds the
  flow to `TargetedReset`.
- **No flow declares a destructive reset.** The evaluator does not admit any
  flow class that omits required preserved state or widens blast radius
  beyond the family default.

## Failure-drill posture

The evaluator fails closed before widening any blast radius:

- A flow without `user_authored_files` preservation is refused.
- A flow with an empty `doctor_finding_ref` or `support_guidance` is refused.
- A cache-rebuild flow with a mismatched rung or invalid blast radius is refused.
- A settings-repair flow without a durable checkpoint is refused.
- A state-migration-repair flow without durable-index preservation is refused.
- A targeted-reset flow with zero or multiple impacted state classes is refused.
- A flow whose `checkpoint_class` requires a `checkpoint_ref` but omits it is refused.
- A flow whose `reversal_class` is `exact` but lacks a `checkpoint_ref` is refused.

## First consumers

- The implementation lives in
  [`crates/aureline-support/src/harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets/mod.rs`](../../../crates/aureline-support/src/harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets/mod.rs).
- The primary evaluator is `HardenedRecoveryFlowEvaluator`.

## Related contracts

- [`recovery_ladder_alpha.md`](../recovery_ladder_alpha.md) — the parent
  recovery-ladder contract.
- [`recovery_action.schema.json`](../../schemas/support/recovery_action.schema.json) —
  the closed recovery-action vocabulary.
- [`repair_transaction.schema.json`](../../schemas/support/repair_transaction.schema.json) —
  the repair-transaction contract bound to these flows.

## Out of scope for this row

- Live runtime enforcement of recovery-ladder entry/exit — those bindings
  land with the runtime-host and chrome consumers that quote this contract.
- Automatic escalation routing to support tickets — the escalation route
  refs are preserved but the upload transport is out of scope.
- Cross-tenant state-migration reconciliation; the fixture covers single-
  tenant entry and exit.
