# Finalize Experiments/Labs Inventory, Kill-Switch Visibility, and Release-Claim Alignment

**Doc ref:** `docs/release/m4/finalize-experiments-labs-inventory.md`  
**Contract ref:** `release:finalize_experiments_labs_inventory:v1`  
**Schema version:** 1

## Overview

This document defines the stable experiments/Labs inventory certification that
makes capability lifecycle first-class product truth across release, support,
and audit surfaces. It governs:

- The canonical `ExperimentsInventory` rows consumed by settings, diagnostics,
  and support export.
- Kill-switch visibility: every disabled row exposes owner, source, reason,
  preserved data, and fallback path.
- Dependency markers carried by saved artifacts, bundles, sync packets, and
  migration exports.
- Release-claim alignment so a row whose proof packet ages out narrows
  automatically.

## Vocabulary

### Inventory surfaces

The closed `InventorySurfaceClass` vocabulary enumerates every surface that must
render experiment rows:

- `settings_ui`
- `cli_headless`
- `help_about`
- `diagnostics_and_support`
- `release_center`
- `migration_and_portable`

### Qualification classes

- `finalized_stable` — the row is backed by complete evidence.
- `finalized_on_waiver` — the row is stable-claimed under an active waiver.
- `visible_bounded` — the row is below Stable but visible and bounded.
- `narrowed_unbacked`, `narrowed_hidden_dependency`,
  `narrowed_metadata_missing`, `narrowed_effective_lifecycle_mismatch`,
  `narrowed_offline_grace_expired` — the row narrows for the named reason.

### Kill-switch visibility rows

Every disabled row carries a `KillSwitchVisibilityRow` with:

- `source_class` — `emergency_security_response`, `admin_policy_ceiling`, etc.
- `source_ref` — stable source reference.
- `reason` — copy-safe disable explanation.
- `preserve_user_data` — true when durable data is retained.
- `preserved_data_scope` — scope of preserved data.
- `fallback_path` — bounded recovery path.

## Integration points

- **Experiments inventory:** `crates/aureline-settings/src/experiments/mod.rs`
- **Labs governance:** `crates/aureline-settings/src/experiments/labs_governance_beta.rs`
- **Stable claim manifest:** `crates/aureline-release/src/stable_claim_manifest/mod.rs`
- **Settings registry:** `crates/aureline-settings/src/finalize_settings_definition_registry/mod.rs`

## Verification

```sh
# Build and check
cargo check -p aureline-release

# Run the certification page and emit fixtures
cargo run -q -p aureline-release \
  --bin aureline_release_finalize_experiments_labs_inventory -- emit-fixtures \
  fixtures/release/m4/finalize-experiments-labs-inventory

# Run unit tests
cargo test -p aureline-release finalize_experiments_labs_inventory
```
