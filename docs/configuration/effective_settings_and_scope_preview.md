# Effective Settings And Scope Preview

The settings authority is implemented in `crates/aureline-settings`. It owns the schema registry, the effective-value resolver, shadow-chain inspection, and scope-explicit write preview path consumed by desktop settings rows, the headless inspector, sync review, policy explainers, and support exports.

## Runtime Objects

- `SettingDefinition` declares the stable `setting_id`, value type, default, allowed scopes, aliases, migrations, sensitivity class, restart posture, preview class, redaction class, and capability dependencies.
- `EffectiveValue` is the in-process resolver result with the winning value, winning scope, source label, lock state, restart posture, and shadow chain.
- `EffectiveSettingRecord` is the serialized cross-surface record. It adds schema tokens, write-intent posture, capability state, control-stack trace, and last-write provenance for UI, CLI, sync, policy, docs/help, and support consumers.
- `SettingWritePreviewRecord` is the scope-explicit mutation preview. It names the selected scope, exact target artifact, actor, reason, checkpoint, approval ticket, restart posture, and any denial reason before apply.

## Shared Surface Commands

Use the headless inspector to reproduce the governed paths:

```bash
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- effective-record security.ai.egress_policy
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- inspect security.ai.egress_policy
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- preview-write
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- ui-beta-page
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- sync-beta-review
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- support-export
```

The `effective-record` command emits the canonical resolver record. The other commands project the same resolver state into CLI inspection, desktop UI rows, sync review, and support export shapes.

## Fixture Case

The fixture directory `fixtures/config/effective_settings_shadow_chain/` captures a policy-locked AI egress setting:

- built-in default contributes `disabled`;
- user-global contributes `any_hosted_provider`;
- admin policy pins `approved_hosted_providers_only`;
- the user-global row remains visible as `capped`;
- the policy row is the winner and sets `policy_locked`;
- the write preview targets `settings://scope/workspace/security.ai.egress_policy` and reports no broader-scope fan-out.

## Conformance Notes

Writes must not spill into a broader scope than the selected target. Policy and capability locks must remain visible in the same shadow chain rendered by UI, CLI, sync review, and support export. Restart posture and rollback/approval requirements come from the definition row and are carried by the resolver output rather than being inferred by individual surfaces.
