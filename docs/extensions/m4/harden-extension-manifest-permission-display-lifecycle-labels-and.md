# Harden extension manifest, permission display, lifecycle labels, and compatibility-range truth

**Status:** Stable extension-manifest lane — implemented in `crates/aureline-extensions`.

## Goal

Promote the extension manifest into the **stable line**. Every claimed stable ecosystem row carries one canonical, checked-in manifest truth: the pinned manifest schema version, the publisher trust tier and lifecycle/deprecation label, the declared (top-level) permissions, the hard dependencies and optional integrations, the compatible API and runtime ranges, and the **effective** permission set after dependency resolution. The **stability qualification** that truth is allowed to claim is derived, not asserted: when the resolver can no longer back a `stable` manifest claim, the visible tier is **automatically narrowed below Stable** (`beta`, `preview`, or `withdrawn`) with machine-readable reasons. Install review, the manifest inspector, upgrade review, mirror promotion, diagnostics, and support exports ingest this packet instead of inventing a generic "compatible" badge.

## Design principles

1. **Declared dependency graph** — Every dependency carries a `dependency_class` (`hard_dependency` / `optional_integration`), a `resolution_state_class`, a lifecycle/deprecation marker, a machine-readable permission-implication flag, and the permissions it contributes. No "needs some other things" prose.
2. **Compatibility ranges resolved before install** — The declared API and runtime ranges each carry a `range_resolution_class` (`satisfied` / `below_minimum` / `above_maximum` / `range_conflict`) against the target host, so a range conflict is visible **before** install, upgrade, or mirror promotion.
3. **Effective permissions after dependency resolution** — The effective set is `declared ∪ resolved-hard-dependency-transitive`. Each effective permission carries a `source_class` (`top_level_declared` / `transitive_hard_dependency` / `both`) and the dependencies that contributed it. Optional-integration permissions are surfaced **separately** (what enabling them would add) and never silently folded in.
4. **No implicit authority widening** — A hard dependency that contributes permissions while `permission_implications_machine_readable == false` widens authority implicitly; the permission is still surfaced, but the claim narrows below Stable and a manifest-review banner is raised. The effective set is re-derived from the declared set and the resolved hard dependencies at validation time, so a stored packet can never hide a transitive permission.
5. **No catalog-only stable claim** — A `stable` manifest tier must be `resolution_backed`; a `catalog_asserted_only` basis narrows below Stable.
6. **Lifecycle and deprecation labels stay inspectable** — The manifest's own lifecycle state and deprecation marker (plus support-window and replacement refs) and each dependency's lifecycle/deprecation markers are carried on every surface.
7. **Downgraded-manifest banner** — A range conflict, an unresolved/removed hard dependency, an implicit-widening event, a quarantined trust tier, or a removal-scheduled dependency raises a banner that names the reason a reviewer must see before install.

## Record kinds

| Record kind | Purpose |
|---|---|
| `stable_manifest_hardening_packet` | Top-level packet consumed by install review, the manifest inspector, upgrade review, mirror promotion, diagnostics, support export, docs/help, and release packets. |
| `stable_manifest_hardening_identity` | Manifest baseline ref, pinned manifest schema version, source package, publisher trust tier, lifecycle state. |
| `stable_manifest_compatibility_range` | Declared API/runtime ranges and their resolution against the target host. |
| `stable_manifest_dependency_edge` | Per-dependency class, resolution state, lifecycle/deprecation markers, machine-readable flag, and contributed permissions. |
| `stable_effective_permission_resolution` | Declared / transitive / effective permission refs, per-permission diff, optional-integration permissions, and the implicit-widening flag. |
| `stable_manifest_lifecycle_label` | Manifest lifecycle state, deprecation marker, support-window and replacement refs. |
| `stable_manifest_hardening_qualification_claim` | Claimed tier, effective tier after the posture is applied, support claim, narrowing reasons. |
| `stable_downgraded_manifest_banner` | Whether a manifest-review banner must display and why. |
| `stable_manifest_hardening_inspection` | Compact boolean/count projection for CLI and inspector surfaces. |
| `stable_manifest_hardening_support_export` | Metadata-safe support/partner export row. |

## Closed vocabularies

### Dependency classes
`hard_dependency`, `optional_integration`

### Dependency resolution states
`resolved`, `unresolved_missing`, `version_conflict`, `optional_absent`

### Deprecation markers
`active`, `deprecated`, `removal_scheduled`, `removed`

### Range resolution
`satisfied`, `below_minimum`, `above_maximum`, `range_conflict` (only `satisfied` may keep a stable claim)

### Permission capability classes
`filesystem_read`, `filesystem_write`, `network_access`, `process_exec`, `secret_access`, `ui_contribution`, `telemetry`, `workspace_state`, `passive_metadata`

### Permission source
`top_level_declared`, `transitive_hard_dependency`, `both`

### Stability tiers
`stable`, `beta`, `preview`, `withdrawn` (only `stable` is a stable manifest claim)

### Narrowing reasons
`manifest_schema_version_mismatch`, `catalog_only_trust_not_resolution_backed`, `trust_tier_quarantined`, `lifecycle_not_installable`, `runtime_range_below_min_unsupported`, `runtime_range_above_max_unverified`, `api_range_below_min_unsupported`, `api_range_above_max_unverified`, `unresolved_hard_dependency`, `dependency_version_conflict`, `dependency_removed`, `dependency_deprecated`, `dependency_removal_scheduled`, `transitive_permission_not_machine_readable`, `effective_permission_diff_inconsistent`, `attribution_incomplete`

## Key invariants

- A `stable` effective tier requires `manifest_schema_version == published`, `claim_basis_class == resolution_backed`, a non-quarantined trust tier, an installable lifecycle, satisfied API and runtime ranges, every hard dependency resolved (and not removed or version-conflicting), no implicit authority widening, and complete attribution.
- `effective_permission_refs == declared ∪ transitive`, with one diff entry per effective permission. The resolution is re-derived from the declared set and the resolved hard dependencies at validation time, so the packet cannot drift from its evidence.
- An unresolved hard dependency contributes nothing to the effective set; its absence is a narrowing reason and a banner condition.
- The effective tier, downgrade flag, narrowing reasons, and the downgraded-manifest banner are re-derived from the posture at validation time.
- `allows_implicit_authority_widening`, `allows_catalog_only_trust`, `allows_hidden_range_conflict`, and `allows_unsurfaced_transitive_permission` are forced `false` and validated.

## File locations

| Artifact | Path |
|---|---|
| Implementation | `crates/aureline-extensions/src/harden_extension_manifest_permission_display_lifecycle_labels_and/mod.rs` |
| Schema | `schemas/extensions/stable_manifest_hardening.schema.json` |
| Fixtures | `fixtures/extensions/m4/harden-extension-manifest-permission-display-lifecycle-labels-and/` |
| Tests | `crates/aureline-extensions/src/harden_extension_manifest_permission_display_lifecycle_labels_and/tests.rs` |
| Dump example | `crates/aureline-extensions/examples/dump_stable_manifest_hardening_records.rs` |
| Proof packet | `artifacts/extensions/m4/harden-extension-manifest-permission-display-lifecycle-labels-and.md` |

## Integration with existing lanes

- Sits above the manifest baseline (`crates/aureline-extensions/src/manifest_baseline/`): that module owns the first inspectable manifest record and the single-extension declared-vs-effective permission diff; this module owns the **dependency-resolved** effective-permission truth and its stability qualification. The `identity.manifest_baseline_ref` points back at the baseline (`manifest_baseline:` prefix).
- Reuses the same trust-tier, lifecycle, capability-class, and stability-tier shapes carried by the permission-manifest, lifecycle-metadata, compatibility-matrix, and marketplace fact-grid lanes, so install review and support surfaces share one manifest vocabulary.

## Verification

```bash
cargo test -p aureline-extensions harden_extension_manifest
cargo run -q -p aureline-extensions --example dump_stable_manifest_hardening_records -- validate
```

Materialized packets for every fixture validate against `schemas/extensions/stable_manifest_hardening.schema.json` (checked with a Draft 2020-12 validator).
