# Artifact: Harden extension manifest, permission display, lifecycle labels, and compatibility-range truth

**Task:** Promote the extension manifest into the stable line — declare hard dependencies, optional integrations, and machine-readable permission implications; pin the compatible API and runtime ranges so a range conflict is visible before install, upgrade, or mirror promotion; resolve the effective permission set after dependency resolution so authority is never widened implicitly; and derive the stability qualification with automatic narrowing below Stable.
**Status:** Implemented
**Verification class:** Conformance / interoperability suite + Security / privacy review + Docs validation + Release evidence review

## Summary

This lane binds the manifest identity (pinned manifest schema version, publisher trust tier, lifecycle state), the declared API/runtime compatibility ranges and their resolution against the target host, the top-level declared permissions, the dependency edges (hard dependencies and optional integrations, each with a dependency class, resolution state, lifecycle/deprecation marker, machine-readable permission-implication flag, and the permissions it contributes), the effective permission resolution (declared ∪ resolved-hard-dependency-transitive, with a per-permission source diff), and the manifest lifecycle/deprecation label into one validated packet, and derives the stability qualification it may claim. A `stable` manifest claim is only allowed when the row pins the published manifest schema version, is resolution-backed, keeps its publisher trust tier out of quarantine, stays on an installable lifecycle, satisfies its API and runtime ranges, resolves every hard dependency, never widens authority implicitly, and is fully attributed. A hard dependency that contributes permissions without declaring them machine-readably widens authority implicitly (the permission is still surfaced, but the claim narrows and a manifest-review banner is raised); an unsatisfied range (`below_minimum` / `range_conflict`) or an unresolved/removed hard dependency withdraws the row. When any condition fails the visible tier is automatically narrowed below Stable (to `beta`, `preview`, or `withdrawn`) with machine-readable reasons. The checked-in packet is canonical: install review, the manifest inspector, upgrade review, mirror promotion, diagnostics, and support exports ingest it instead of inventing a generic "compatible" badge.

## What changed

- New Rust module: `crates/aureline-extensions/src/harden_extension_manifest_permission_display_lifecycle_labels_and/mod.rs` (+ `tests.rs`)
- Re-exported from `crates/aureline-extensions/src/lib.rs`
- New schema: `schemas/extensions/stable_manifest_hardening.schema.json`
- New fixtures: `fixtures/extensions/m4/harden-extension-manifest-permission-display-lifecycle-labels-and/`
  - `verified_publisher_stable_current.json` — a verified formatter that pins the published manifest version, satisfies its ranges, resolves its hard dependency, surfaces the transitive network permission machine-readably, and holds Stable.
  - `runtime_range_above_max_narrows_to_beta.json` — a linter running on a newer-than-declared runtime (`above_maximum`); it narrows to `beta` (unverified) without a hard block.
  - `catalog_asserted_narrows_to_preview.json` — a theme claiming Stable on catalog assertion alone (`catalog_asserted_only`); it narrows to `preview`.
  - `implicit_widening_narrows_to_preview.json` — a debugger whose hard dependency contributes a network permission it does not self-declare; the permission is still surfaced in the effective set, but the claim narrows to `preview` and a manifest-review banner is raised.
  - `unresolved_hard_dependency_withdrawn.json` — a database tool whose required driver cannot be resolved; the claim is `withdrawn`, a banner is raised, and the driver's permission is excluded from the effective set.
- New dump example: `crates/aureline-extensions/examples/dump_stable_manifest_hardening_records.rs`
- New docs: `docs/extensions/m4/harden-extension-manifest-permission-display-lifecycle-labels-and.md`

## Acceptance criteria

- [x] The checked-in implementation, fixtures, and proof packet are current and self-describing (schema, fixtures, and docs reference one another) rather than ad hoc notes. (`stable_manifest_hardening.schema.json`, fixtures dir, this packet)
- [x] Any surface still lacking stable qualification is automatically narrowed below Stable, with machine-readable reasons, instead of inheriting an adjacent green row. (`runtime_range_above_max_narrows_to_beta.json`, `catalog_asserted_narrows_to_preview.json`, `implicit_widening_narrows_to_preview.json`, `unresolved_hard_dependency_withdrawn.json`, `manifest_schema_version_mismatch_narrows_below_stable`)
- [x] Users and admins can inspect permissions (top-level and effective-after-resolution), compatibility range (API + runtime), activation cost class (dependency/range posture), lifecycle label, publisher provenance, and rollback/revocation state (lifecycle + quarantine) for the touched ecosystem row. (`stable_manifest_hardening_inspection`, `stable_effective_permission_resolution`, `stable_manifest_lifecycle_label`)
- [x] Manifest fixtures prove that transitive permission implications, optional integrations, and runtime-range conflicts are visible before install, upgrade, or mirror promotion. (`implicit_widening_narrows_to_preview.json`, `optional_integration_permissions_are_not_folded_into_effective`, `runtime_range_below_minimum_withdraws_the_claim`)
- [x] Stable manifest inspect/export shows top-level requested permissions and effective permissions after dependency resolution, together with dependency class and lifecycle/deprecation markers. (`declared_permissions`, `effective_permissions.diff_entries` with `source_class`, `dependencies[*].dependency_class`/`deprecation_class`, `support_export_quotes_permission_and_dependency_counts`)

## Guardrails honored

- No ambient/implicit extension privilege: a hard dependency that contributes permissions without declaring them machine-readably is flagged `implicit_widening_present`, narrows below Stable, and raises a banner — while still surfacing the permission (`implicit_widening_narrows_and_still_surfaces_the_permission`).
- No catalog-only trust: a `catalog_asserted_only` basis can never back a stable claim (`catalog_asserted_narrows_to_preview.json`, `no_catalog_only_stable_claim`).
- No hidden range conflict: a `below_minimum` / `range_conflict` API or runtime range withdraws the row and raises a banner (`runtime_range_below_minimum_withdraws_the_claim`); an `above_maximum` range narrows to `beta`.
- No unbounded/implicit authority: the effective set equals `declared ∪ transitive` and is re-derived at validation time; unresolved hard dependencies contribute nothing (`unresolved_hard_dependency_withdraws_and_excludes_its_permissions`).
- A narrower stable claim is published rather than papered over: the effective tier, downgrade flag, reasons, and banner are re-derived from the posture at validation time, so the packet cannot drift.

## How to verify

```bash
cargo test -p aureline-extensions harden_extension_manifest
cargo run -q -p aureline-extensions --example dump_stable_manifest_hardening_records -- validate
```

Materialized packets for every fixture validate against `schemas/extensions/stable_manifest_hardening.schema.json` (checked with a Draft 2020-12 validator).

## Risks / follow-ups

- The API/runtime range resolution class is supplied by the producing resolver. When a live range solver lands, the `range_resolution_class` should be derived from the declared min/max refs versus the resolved host version rather than a producer-supplied class.
- The effective-permission resolution consumes producer-supplied contributed-permission sets per dependency; a later revision should source the transitive closure directly from the dependency manifests (recursively) instead of accepting a single contributed layer.
- Trust-tier, lifecycle, capability-class, dependency-class, and stability-tier vocabularies are closed string sets shared with the manifest-baseline, permission-manifest, lifecycle-metadata, and compatibility-matrix lanes; when those crates stabilize typed enums, these should be narrowed to share them rather than re-declared as strings.
- Optional integrations are surfaced one layer deep (what enabling them would add); an enabled-integration view that re-resolves the effective set with the integration folded in is left for the install-review consumer.
