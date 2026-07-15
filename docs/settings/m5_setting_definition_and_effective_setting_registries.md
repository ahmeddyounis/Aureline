# M5 setting-definition and effective-setting registries

This lane is the first implement lane over the frozen
[M5 settings-governance matrix](./m5_settings_resolver_contract.md). It turns the *setting-definition* grammar
(how a stable setting is declared) and the *effective-setting* grammar (how its live value is resolved from the
winning scope) into registry resolvers that produce export-safe, honest projections, so the settings, shell,
diagnostics, admin, sync, policy, docs, CLI, and support surfaces resolve one canonical configuration truth
instead of a per-setting, hand-copied reconstruction. The setting definition and the effective setting are
separated in runtime and serialized state: the declared type, stable setting ID, allowed scopes, declared
default, migration aliases, restart posture, and sensitivity class live on the setting definition, while the
resolved value or redacted summary, shadow chain of scopes that lost, lock or constraint state, validation
status, restart state, capability availability, and last-applied revision live on the effective setting, and
stable setting IDs stay non-recycled so a retired setting ID is never reused for a different meaning.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_setting_definition_and_effective_setting_registries` (the authoritative validator).
- **Combined schema:**
  `schemas/config/m5-setting-definition-and-effective-setting-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/config/m5-setting-definition.schema.json`](../../schemas/config/m5-setting-definition.schema.json)
  and
  [`schemas/config/effective_setting.schema.json`](../../schemas/config/effective_setting.schema.json)
  as its canonical domain contracts.
- **Checked proof:** `artifacts/release/m5-setting-definition-and-effective-setting-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/config/m5-setting-definition-and-effective-setting-registries/`
  (`setting_definition_beta_narrowed.json`, `effective_setting_preview_narrowed.json`).

## Two registries

1. **Setting definition** (`resolve_setting_definition_entry`) — publishes one stable setting-definition object
   per setting: the setting-definition type and canonical type mode, the stable setting ID preserved verbatim,
   the allowed scopes, the declared default, the migration aliases, the restart posture, the sensitivity class,
   and the capability dependencies. A clean entry names a canonical registry token, a classified
   setting-definition type, and a settings-governance role, covers the canonical / accessible / audit
   resolution forms, publishes a complete object, preserves the stable setting ID as a non-recycled identity,
   and discloses the sensitivity posture before a sensitive setting is surfaced. Otherwise it degrades honestly
   — a stable setting ID recycled into a different meaning (or a sensitive setting that hides its sensitivity
   posture) degrades to `setting_definition_recycles_id_or_hides_sensitivity`.
2. **Effective setting** (`resolve_effective_setting_entry`) — keeps the effective setting honest. A clean
   entry names a classified winning scope and provides the complete resolved-value / shadow-chain / lock-state /
   validation-status / restart-state / capability-availability / last-applied-revision effective-setting object;
   a record that would hide the shadow chain of scopes that lost, mask a locked value without disclosing its
   lock source, or let machine-only state masquerade as portable degrades to
   `effective_setting_hides_shadow_chain_or_masks_lock_or_machine_state`.

## Per-entry settings reference

The setting-definition type carries its canonical type mode, and the resolver publishes the full definition
object, so the registry — never a hand-copied per-setting assumption — is the single source of truth.
`setting_definition_object_is_complete` rejects an object missing any field,
`stable_setting_id_stays_non_recycled` rejects an ID recycle or a hidden sensitivity posture, and
`effective_setting_stays_honest` rejects a record that has hidden its shadow chain.

| setting-definition type | type mode | stable setting id | allowed scopes | declared default | restart posture | sensitivity class |
| --- | --- | --- | --- | --- | --- | --- |
| boolean | boolean_setting_type | `editor.format_on_save` | `scopes.machine-user-workspace` | `default.false` | `restart.none` | `sensitivity.public` |
| enum | enum_setting_type | `workbench.theme_mode` | `scopes.user-workspace` | `default.system` | `restart.none` | `sensitivity.public` |
| path | path_setting_type | `tools.plugin_root` | `scopes.machine-user` | `default.redacted-path` | `restart.on-next-launch` | `sensitivity.location-bearing` |

An ID recycle degrades to `setting_definition_recycles_id_or_hides_sensitivity`, an incomplete object degrades
to `setting_definition_object_incomplete`, and a hidden shadow chain degrades to
`effective_setting_hides_shadow_chain_or_masks_lock_or_machine_state`, so an ID recycle, an incomplete object,
or a hidden shadow chain can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Every claimed setting resolves to one stable setting-definition object with stable-setting-id /
  allowed-scopes / default / restart-posture / sensitivity fields.** Clean definition entries cover the
  canonical boolean / enum / number / path / secret-reference types and the first settings / shell /
  diagnostics / admin / support surfaces, an object-incomplete example degrades, and no clean definition entry
  published an incomplete object.
- **Stable setting IDs stay non-recycled; a retired setting ID is never reused for a different meaning.** An
  ID-recycle example and an unbound example degrade, a clean non-recycled definition entry is present, and no
  clean entry recycled the ID.
- **The suite fails when an effective setting collapses into a hidden shadow chain.** Clean effective-setting
  entries cover the machine / user / workspace scopes with full resolution-form coverage while providing the
  complete record object, and a record that would hide the shadow chain of scopes that lost or mask a locked
  value degrades — so users and support can inspect not only what is active but why another scope lost and
  whether restart is required.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_setting_definition_and_effective_setting_registries -- support-export
cargo run -p aureline-ui --example dump_m5_setting_definition_and_effective_setting_registries -- csv
cargo run -p aureline-ui --example dump_m5_setting_definition_and_effective_setting_registries -- report
cargo run -p aureline-ui --example dump_m5_setting_definition_and_effective_setting_registries -- setting-definition-table
cargo run -p aureline-ui --example dump_m5_setting_definition_and_effective_setting_registries -- fixture-setting-definition-beta-narrowed
cargo run -p aureline-ui --example dump_m5_setting_definition_and_effective_setting_registries -- fixture-effective-setting-preview-narrowed
```
