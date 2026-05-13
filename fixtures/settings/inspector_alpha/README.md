# Settings inspector alpha fixtures

These fixtures prove that settings inspection, write preview, and
support export consume the same schema-backed effective-setting record.

| Fixture | Purpose |
|---|---|
| [`policy_locked_effective_record.json`](./policy_locked_effective_record.json) | Shows winning value, source chain, policy-lock explanation, restart state, capability state, and last-applied revision for a locked high-risk setting. |
| [`scope_explicit_write_preview.json`](./scope_explicit_write_preview.json) | Shows a write preview that keeps the selected workspace scope, names checkpoint and approval refs, and produces a rollback-ready change summary before apply. |
| [`support_export_shared_contract.json`](./support_export_shared_contract.json) | Shows support export embedding the same effective-setting inspection record instead of a copy-only shadow model. |

Regenerate representative payloads with:

```sh
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- inspect security.ai.egress_policy
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- preview-write
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- support-export
```
