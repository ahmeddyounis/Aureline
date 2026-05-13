# Settings Inspector Alpha

This page describes the schema-backed effective-settings inspector
implemented in `aureline-settings`.

## Contract

The inspector record is `EffectiveSettingInspectionRecord` in
`crates/aureline-settings/src/inspector/`. It is built from the
canonical `SettingDefinition` plus the resolver's `EffectiveValue`.
UI rows, CLI inspection, help deep links, write previews, and support
exports consume this record through projections instead of re-reading
raw overlays or copying settings into a private surface model.

Each inspected setting exposes:

- schema definition fields: stable `setting_id`, declared type,
  allowed scopes, default preview, migration aliases, restart posture,
  sensitivity class, redaction class, capability dependencies, help
  doc ref, and evidence refs;
- effective state: winning value or redacted summary, winning scope,
  source label, shadow chain, lock state, lock reason, validation
  status, restart state, capability availability, policy explanation,
  and last-applied revision;
- write-preview state: selected target scope, exact destination
  artifact ref, actor class, reason class, preview class, checkpoint
  ref, approval ref, restart posture, and rollback-ready change
  summary before apply.

Policy ceilings remain visible in the shadow chain. A locked or
constrained setting includes `policy_lock_explanation`, naming the
policy source scope, source label, stable source ref, and reason.

## CLI Consumer

The first surfaced consumer is the settings crate CLI:

```sh
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- inspect security.ai.egress_policy
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- preview-write
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- support-export
```

The `inspect` command emits a CLI projection and the canonical
effective-setting record. `preview-write` emits a scope-explicit write
preview without mutating the live resolver. `support-export` embeds the
same effective-setting records consumed by the CLI projection.

## Fixtures

Protected fixtures live in
`fixtures/settings/inspector_alpha/`:

- `policy_locked_effective_record.json`
- `scope_explicit_write_preview.json`
- `support_export_shared_contract.json`

The integration test
`crates/aureline-settings/tests/inspector_alpha_fixtures.rs` replays
those fixtures through the Rust types and checks that the policy lock,
restart state, capability state, scope-explicit destination, checkpoint
and approval posture, and shared support-export source record remain
intact.

## Verification

```sh
cargo test -p aureline-settings
```
