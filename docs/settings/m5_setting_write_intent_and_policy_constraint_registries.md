# M5 setting-write-intent and policy-constraint registries

This lane is the write-pipeline implement lane over the frozen
[M5 settings-governance matrix](./m5_settings_resolver_contract.md) (the `write_setting` family). It turns the
*setting-write-intent* grammar (how a configuration mutation declares the scope, artifact, actor, reason,
preview class, and recovery evidence it will land) and the *policy / constraint* grammar (how a locked or denied
write explains itself) into registry resolvers that produce export-safe, honest projections, so the settings,
shell, diagnostics, admin, sync, policy, docs, CLI, and support surfaces mutate one canonical configuration
truth instead of a per-write, hand-copied path. The write intent and the policy constraint are separated in
runtime and serialized state: the preview class, target scope, target artifact, intended value, actor, change
reason, preview reference, and checkpoint / rollback recovery reference live on the write intent, while the lock
source, allowed override classes, expiry / review window, validation status, review state, docs pointer, and
last review revision live on the policy constraint, and a scoped write is never rewritten into a broader scope
or landed in an unintended artifact because a downstream writer found that path easier.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_setting_write_intent_and_policy_constraint_registries` (the authoritative
  validator).
- **Combined schema:**
  `schemas/config/m5-setting-write-intent-and-policy-constraint-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/config/m5-setting-write-intent.schema.json`](../../schemas/config/m5-setting-write-intent.schema.json)
  and
  [`schemas/governance/policy_decision_explain.schema.json`](../../schemas/governance/policy_decision_explain.schema.json)
  as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-setting-write-intent-and-policy-constraint-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/config/m5-setting-write-intent-and-policy-constraint-registries/`
  (`write_intent_beta_narrowed.json`, `policy_constraint_preview_narrowed.json`).

## Two registries

1. **Setting write intent** (`resolve_setting_write_intent_entry`) — publishes one write-intent object per
   mutation: the preview class and canonical class mode, the target scope, the target artifact, the intended
   value, the actor / route, the change reason, the preview reference, and the checkpoint / rollback recovery
   reference. A clean entry names a canonical registry token, a classified preview class, and a
   settings-governance role, covers the canonical / accessible / audit resolution forms, publishes a complete
   object, lands only in the chosen scope / artifact, and materializes the recovery evidence before a high-risk
   write applies. Otherwise it degrades honestly — a scoped write rewritten into a broader scope (or a high-risk
   write that hides its recovery evidence) degrades to
   `write_intent_rewrites_scope_or_hides_recovery_evidence`.
2. **Policy / constraint** (`resolve_policy_constraint_entry`) — keeps a locked or denied write honest. A clean
   entry names a classified lock class and provides the complete lock-source / allowed-override-classes /
   expiry-review / validation-status / review-state / docs-pointer / last-review-revision policy-constraint
   object; a record that would mask a locked value without disclosing its lock source or deny a write without
   disclosing the fallback guidance degrades to `policy_constraint_masks_lock_source_or_hides_fallback`.

## Per-entry write-intent reference

The preview class carries its canonical class mode, and the resolver publishes the full write-intent object, so
the registry — never a hand-copied per-write assumption — is the single source of truth.
`write_intent_object_is_complete` rejects an object missing any field, `write_intent_lands_in_chosen_scope`
rejects a scope rewrite or hidden recovery evidence, and `policy_constraint_stays_honest` rejects a record that
has masked its lock source or hidden its fallback.

| preview class | class mode | target scope | target artifact | intended value | change reason | recovery reference |
| --- | --- | --- | --- | --- | --- | --- |
| no-op | no_op_reversible_class | `scope.workspace` | `artifact.workspace-settings-json` | `value.true` | `reason.enable-format-on-save` | `recovery.checkpoint-and-rollback-0007` |
| low-risk | low_risk_reversible_class | `scope.user` | `artifact.user-settings-json` | `value.dark` | `reason.apply-dark-theme` | `recovery.checkpoint-and-rollback-0007` |
| high-risk | high_risk_irreversible_class | `scope.machine` | `artifact.machine-policy-json` | `value.redacted-path` | `reason.repoint-plugin-root` | `recovery.checkpoint-and-rollback-0007` |

A scope rewrite degrades to `write_intent_rewrites_scope_or_hides_recovery_evidence`, an incomplete object
degrades to `write_intent_object_incomplete`, and a masked lock degrades to
`policy_constraint_masks_lock_source_or_hides_fallback`, so a scope rewrite, an incomplete object, or a masked
lock can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **High-risk settings changes produce preview, checkpoint, and rollback evidence before apply when effective
  behavior changes materially.** Clean write-intent entries cover the canonical no-op / low-risk / material /
  high-risk / destructive preview classes and the first settings / shell / diagnostics / admin / support
  surfaces, an object-incomplete example (a missing preview / recovery reference) degrades, and no clean
  write-intent entry published an incomplete object.
- **Locked or denied writes return structured reasons and fallback guidance instead of ambiguous failure
  copy.** Clean policy-constraint entries cover the policy-locked / override-allowed / advisory lock classes
  with full resolution-form coverage while providing the complete record object, and a record that would mask a
  locked value or deny a write without disclosing its fallback degrades.
- **Tests fail when a configuration route rewrites scope or artifact ownership without explicit user or policy
  intent.** A scope-rewrite example and an unbound example degrade, a clean scope-preserving write-intent entry
  is present, and no clean entry rewrote the scope — so a Workspace or Profile write can never silently land in
  User or Machine scope because a downstream writer found that path easier.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_setting_write_intent_and_policy_constraint_registries -- support-export
cargo run -p aureline-ui --example dump_m5_setting_write_intent_and_policy_constraint_registries -- csv
cargo run -p aureline-ui --example dump_m5_setting_write_intent_and_policy_constraint_registries -- report
cargo run -p aureline-ui --example dump_m5_setting_write_intent_and_policy_constraint_registries -- write-intent-table
cargo run -p aureline-ui --example dump_m5_setting_write_intent_and_policy_constraint_registries -- fixture-write-intent-beta-narrowed
cargo run -p aureline-ui --example dump_m5_setting_write_intent_and_policy_constraint_registries -- fixture-policy-constraint-preview-narrowed
```
