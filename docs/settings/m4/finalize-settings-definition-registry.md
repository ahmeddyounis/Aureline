# Finalize Settings-Definition Registry, Effective-Configuration Inspect/Export, and Lifecycle Truth

**Doc ref:** `docs/settings/m4/finalize-settings-definition-registry.md`  
**Contract ref:** `settings:finalize_settings_definition_registry:v1`  
**Schema version:** 1

## Overview

This document defines the stable settings-definition registry certification
that makes configuration and capability lifecycle first-class product truth.
It governs:

- The canonical `SettingDefinition` registry and `SchemaRegistry` seed catalog.
- The `EffectiveSettingRecord` inspect/export parity across all surfaces.
- The experiments/Labs inventory dependency markers that narrow stable claims.
- Kill-switch visibility and preserved-data behavior.
- Offline-entitlement grace state for enterprise and managed profiles.

## Vocabulary

### Inspect surfaces

The closed `InspectSurfaceClass` vocabulary enumerates every surface that must
render the same effective-setting record:

- `desktop_settings_ui`
- `cli_headless_inspect`
- `help_about_panel`
- `diagnostics_panel`
- `support_export_packet`
- `migration_review`
- `portable_state_artifact`

### Qualification classes

- `finalized_stable` — the row is backed by complete evidence and carries no
  hidden experiment dependencies.
- `finalized_with_dependency_marker` — the row is stable-claimed but explicitly
  marks every non-stable capability dependency.
- `narrowed_unbacked`, `narrowed_hidden_experiment_dependency`,
  `narrowed_capability_below_stable`, `narrowed_kill_switch_invisible`,
  `narrowed_offline_grace_expired` — the row narrows below Stable for the
  named reason.

### Lifecycle dependency markers

Every setting that depends on a non-stable capability carries a
`LifecycleDependencyMarker` with:

- `marker_id` — stable marker identity.
- `parent_id` — the setting id.
- `required_capability_id` — the capability required.
- `required_lifecycle_state` — the required state token.
- `effect_on_parent` — `narrows_lifecycle`, `gates_capability`, or
  `declares_artifact_dependency`.
- `disclosure_summary` — copy-safe explanation.
- `fallback_path` — bounded recovery path.

## Integration points

- **Schema registry:** `crates/aureline-settings/src/schema/registry.rs`
- **Resolver:** `crates/aureline-settings/src/resolver/`
- **Experiments inventory:** `crates/aureline-settings/src/experiments/mod.rs`
- **Labs governance:** `crates/aureline-settings/src/experiments/labs_governance_beta.rs`
- **Policy offline entitlement:**
  `crates/aureline-policy/src/finalize_signed_policy_bundle_offline_entitlement_and_mirror/mod.rs`

## Verification

```sh
# Build and check
cargo check -p aureline-settings

# Run the certification page and emit fixtures
cargo run -q -p aureline-settings \
  --bin aureline_settings_finalize_settings_definition_registry -- emit-fixtures \
  fixtures/settings/m4/finalize-settings-definition-registry

# Run unit tests
cargo test -p aureline-settings finalize_settings_definition_registry
```
