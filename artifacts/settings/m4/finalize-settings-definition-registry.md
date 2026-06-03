# Finalize Settings-Definition Registry, Effective-Configuration Inspect/Export, and Lifecycle Truth

**Artifact ref:** `artifacts/settings/m4/finalize-settings-definition-registry.md`  
**Contract ref:** `settings:finalize_settings_definition_registry:v1`  
**Schema version:** 1  
**As of:** 2026-06-03

## Purpose

This artifact certifies that the stable settings-definition registry,
effective-configuration inspect/export parity, experiments/Labs dependency
markers, kill-switch visibility, and offline-entitlement grace state are
first-class product truth across all claimed surfaces.

## Certification scope

The certification page (`FinalizeSettingsDefinitionRegistryPage`) binds, for
every setting in the seed catalog:

1. **Schema registry completeness** — `setting_id`, value type, default, allowed
   scopes, migration aliases, restart posture, sensitivity, and capability
   dependencies.
2. **Effective-setting inspect parity** — the same `EffectiveSettingRecord`
   vocabulary feeds desktop UI, CLI/headless inspect, Help/About, diagnostics,
   support export, migration review, and portable-state artifacts.
3. **Lifecycle dependency markers** — every non-stable capability dependency
   carries a visible `LifecycleDependencyMarker` with `required_capability_id`,
   `required_lifecycle_state`, and `effect_on_parent`.
4. **Kill-switch visibility** — disabled rows expose the winning disable-source
   class, reason, preserved-data scope, and fallback path.
5. **Offline-entitlement grace** — enterprise/managed rows expose
   last-known-good policy source, grace expiry, and blocked-capability truth.

## Acceptance criteria

- Every visible stable setting resolves through one effective-setting record and
  can explain what value won, why it won, whether it is locked, what restart or
  live-apply posture applies, and which lifecycle dependency markers narrow its
  claim.
- Experiments/Labs affecting claimed surfaces are visible, exportable, and
  bounded by owner, cohort, expiry, rollout ring, and kill-switch metadata.
- Stable claims do not silently depend on hidden experiment state.
- Kill-switch drills preserve durable user data, emit export-safe explanation,
  and downgrade affected surfaces truthfully.
- Hidden experiment dependencies on marketed stable rows are zero.
- Offline-entitlement grace or expiry paths are green on claimed enterprise
  profiles.

## Canonical paths

- **Doc:** `docs/settings/m4/finalize-settings-definition-registry.md`
- **Fixture:** `fixtures/settings/m4/finalize-settings-definition-registry/`
- **Source:** `crates/aureline-settings/src/finalize_settings_definition_registry/mod.rs`
- **Bin:** `cargo run -q -p aureline-settings --bin aureline_settings_finalize_settings_definition_registry -- page`
