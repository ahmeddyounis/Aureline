# Artifact: Publish stable SDK/deprecation policy, manifest version windows, and ecosystem migration guidance

**Task:** Publish the stable SDK / deprecation policy, the manifest version windows, and the ecosystem migration guidance for a claimed stable extension row — binding the SDK deprecation policy (stage, last-supported window, replacement package / API, pin allowance, named affected dependency edges), the deprecation propagation (install-time warnings, marketplace cards, dependency-resolution output, migration docs, compatibility shim), the manifest version window, the ecosystem migration guidance (exact / translated / partial / shimmed / unsupported outcome, shim availability, preserved rollback checkpoint), the permission posture, compatibility, and install / activation-cost / revocation / mirror posture into one validated packet, and derive the stability qualification with automatic narrowing below Stable.

**Status:** Implemented
**Verification class:** Conformance / interoperability suite + Security / privacy review + Docs validation + Release evidence review

## Summary

This lane binds, for a stable SDK-policy row, the identity (SDK-policy descriptor ref, row identity, package identity, SDK channel id, SDK-policy version, the pinned published profile version, the publisher namespace, the pinned policy-evidence ref, publisher trust tier, lifecycle state), the **SDK deprecation policy** (deprecation stage `active` / `deprecated` / `sunset` / `removed`, last-supported version, replacement kind `replacement_package` / `replacement_api` / `superseded_no_replacement` / `none`, the replacement ref, the pin policy `pin_allowed` / `pin_discouraged` / `pin_blocked`, the support-window ref, and the named affected dependency edges), the **deprecation propagation** (whether the deprecation surfaces in install-time warnings, marketplace cards, dependency-resolution output, the migration docs, and a compatibility shim), the **manifest version window** (min / max / published / row manifest versions), the **ecosystem migration guidance** (outcome `exact` / `translated` / `partial` / `shimmed` / `unsupported`, the migration-doc ref, the shim availability `no_shim_needed` / `shim_available` / `shim_unavailable`, a preserved-rollback-checkpoint flag, and a diagnostics ref), the permission posture (declared / effective / policy-cap refs, widened flag), the compatibility label, and the install posture (install scope, activation cost class, revocation posture, mirrorability, rollback) into one validated packet, and derives the stability qualification it may claim. A `stable` SDK-policy claim is only allowed when the row pins the published SDK-policy profile version, is evidence-backed, keeps its trust tier out of quarantine, stays runnable, keeps its SDK out of the sunset window and out of removal, names a replacement and a last-supported window and the affected dependency edges and propagates the deprecation into the install warning / marketplace card / dependency resolution and the migration docs whenever the SDK is deprecated, keeps the row manifest version inside the supported window, surfaces an `exact` / `translated` / `shimmed` migration with a preserved rollback checkpoint when a mapping is not exact, never widens permissions beyond the declared manifest or the policy cap, keeps its activation cost bounded, keeps verified non-parity-limited compatibility, discloses its install scope, keeps a clean revocation posture, stays mirrorable, and is fully attributed.

When any condition fails the visible tier is automatically narrowed below Stable with machine-readable reasons. A removed SDK, an unsupported migration, a permission widened beyond the declared manifest or policy cap, an unbounded activation cost, a non-runnable lifecycle, an unsupported compatibility, or a quarantined / revoked revocation posture withdraws the row; a missing replacement or last-supported window, a missing migration doc, an out-of-window manifest version, a missing rollback checkpoint, an unpublished profile version, a catalog-asserted basis, a quarantined trust tier, an unverified compatibility, or an undisclosed install scope narrows to `preview`; an SDK in the sunset window, unnamed dependency edges, an incomplete deprecation propagation, a partial migration, a parity-limited compatibility, an advisory revocation posture, or a not-mirrorable row narrows to `beta`. The manifest-window bounds and the migration / shim consistency are cross-checked so a record cannot be internally inconsistent. The checked-in packet is canonical: the SDK migration console, the deprecation-policy view, dependency-resolution output, install review, the marketplace card, the extension detail view, diagnostics, support exports, the CLI inspector, and release packets ingest it instead of leaving a deprecation living only in release notes.

## What changed

- New Rust module: `crates/aureline-extensions/src/publish_stable_sdk_deprecation_policy_manifest_version_windows/mod.rs` (+ `tests.rs`)
- Re-exported from `crates/aureline-extensions/src/lib.rs`
- New schema: `schemas/extensions/stable_sdk_deprecation_policy.schema.json`
- New fixtures: `fixtures/extensions/m4/publish-stable-sdk-deprecation-policy-manifest-version-windows/`
  - `current_sdk_active_policy_stable.json` — current SDK, manifest in window, exact migration; holds Stable.
  - `deprecated_sdk_with_shim_stable.json` — deprecated SDK surface with a named replacement, last-supported window, named edges, full propagation, and a shimmed migration with a preserved rollback checkpoint; still Stable.
  - `sunset_window_narrows_to_beta.json` — the SDK is in the sunset window; narrows to `beta` with a banner.
  - `partial_migration_narrows_to_beta.json` — the migration outcome is partial; narrows to `beta`.
  - `missing_replacement_narrows_to_preview.json` — a deprecated SDK surface with no named replacement; narrows to `preview`.
  - `manifest_version_out_of_window_narrows_to_preview.json` — the row manifest version is above the supported window; narrows to `preview` with a banner.
  - `catalog_asserted_basis_narrows_to_preview.json` — a stable claim asserted only by the catalog; narrows to `preview`.
  - `unsupported_migration_withdrawn.json` — the imported artifact cannot be mapped onto the current SDK; `withdrawn` with a banner.
  - `widened_permission_withdrawn.json` — effective permissions widened beyond the declared manifest and policy cap; `withdrawn` with a banner.
- New dump example: `crates/aureline-extensions/examples/dump_stable_sdk_deprecation_policy_records.rs`
- New docs: `docs/extensions/m4/publish-stable-sdk-deprecation-policy-manifest-version-windows.md`

## Acceptance criteria

- [x] The checked-in implementation, fixtures, and proof packet are current and self-describing (schema, fixtures, and docs reference one another) rather than ad hoc notes. (`stable_sdk_deprecation_policy.schema.json`, fixtures dir, this packet)
- [x] Any surface still lacking stable qualification is automatically narrowed below Stable, with machine-readable reasons, instead of inheriting an adjacent green row. (`sunset_window_narrows_to_beta.json`, `partial_migration_narrows_to_beta.json`, `missing_replacement_narrows_to_preview.json`, `manifest_version_out_of_window_narrows_to_preview.json`, `catalog_asserted_basis_narrows_to_preview.json`, `unsupported_migration_withdrawn.json`, `widened_permission_withdrawn.json`)
- [x] Users and admins can inspect permissions (permission posture, declared / effective / policy-cap refs), compatibility range (compatibility label + scorecard ref), activation cost (`activation_cost_class`), lifecycle label, publisher provenance (trust tier + SDK-policy ref + publisher namespace), and rollback / revocation state (revocation posture + rollback support + preserved migration rollback checkpoint) for the touched ecosystem row. (`stable_sdk_deprecation_policy_inspection`, `stable_sdk_policy_install_posture`, `stable_sdk_policy_permission_posture`, `stable_sdk_policy_compatibility`, `stable_sdk_policy_identity`)
- [x] Conformance fixtures, activation-budget instrumentation, and publisher continuity packets make the ecosystem claims supportable and mirrorable on the M4 line. (`activation_cost_class`, `mirrorability_class`, `policy_evidence_ref`, all nine fixtures, the metadata-safe support export)
- [x] One stable manifest / permission / lifecycle / compatibility vocabulary is shared across install review, runtime, mirror / manual import, disable / rollback, and revocation paths. (trust-tier, lifecycle, compatibility-label, install-scope, revocation-posture, mirrorability, stability-tier, and claim-basis vocabularies are the same closed string sets shared with the catalog-truth, lifecycle-flow, bridge-certification, mirror-import, performance-budget, and policy-pack-governance stable lanes.)

## v20 requirements honored

- [x] SDK/API deprecations flow into install-time warnings, marketplace cards, dependency-resolution output, migration docs, and compatibility shims (the `stable_deprecation_propagation` record), not only release notes — an incomplete propagation narrows the claim.
- [x] Deprecation packets name the affected dependency edges (`affected_dependency_edge_count` + `dependency_edges_ref`), the last-supported window (`last_supported_version` + `support_window_ref`), the replacement package or API (`replacement_kind_class` + `replacement_ref`), and whether pinning to a last-known-good version remains allowed by policy (`pin_policy_class`).
- [x] Exact / translated / partial / shimmed / unsupported outcome labels are generated from the imported artifact (`migration_outcome_class`) and a rollback checkpoint and diagnostics are preserved when a mapping is not exact (`rollback_checkpoint_preserved`, `diagnostics_ref`).
- [x] Ecosystem migration fixtures prove that deprecated APIs and packages surface the same sunset, replacement, and pin/downgrade truth in marketplace, CLI/headless, docs, and support export — the same packet drives the inspection projection (CLI/headless), the support export (support/mirror), and the consumer-surface bindings (marketplace card, docs/help). (`deprecated_sdk_with_shim_stable.json`, `sunset_window_narrows_to_beta.json`)

## Guardrails honored

- No ambient extension privilege: a permission set widened beyond the declared manifest or policy cap withdraws the row (`widened_permission_withdraws_the_row`); `allows_ambient_extension_privilege` is pinned false.
- No catalog-only trust: a `catalog_asserted_only` basis can never back a stable claim (`catalog_asserted_basis_cannot_back_stable`, `no_catalog_only_stable_claim`); `allows_catalog_only_trust` is pinned false.
- No unbounded activation cost: an `unbounded` activation-cost class withdraws the row (`unbounded_activation_cost_withdraws_the_row`); `allows_unbounded_activation_cost` is pinned false.
- No widened public scope from this row alone: the packet only narrows; it never promotes a row to a wider claim than the posture supports.
- A narrower stable claim is published rather than papered over: the effective tier, downgrade flag, reasons, and banner are re-derived from the posture at validation time, and the manifest-window bounds and migration / shim consistency are cross-checked, so the packet cannot drift.

## How to verify

```bash
cargo test -p aureline-extensions publish_stable_sdk
cargo run -q -p aureline-extensions --example dump_stable_sdk_deprecation_policy_records -- validate
```

Materialized packets for every fixture validate against `schemas/extensions/stable_sdk_deprecation_policy.schema.json` (checked with a Draft 2020-12 validator).

## Risks / follow-ups

- The SDK deprecation policy is modeled for one SDK surface per row; a later revision should carry a per-affected-edge change list (edge id + deprecation stage + replacement) so the dependency-resolution surface can render individual edge deltas rather than an aggregate edge count.
- The migration outcome is a producer-supplied closed string here; when the import/migration engine emits a typed mapping report for the imported artifact, this should ingest that report directly rather than re-declaring a bare outcome class.
- The activation cost is a producer-supplied closed string; when the stable performance-budget lane's measured cost is available for a row, this should ingest that measurement instead of re-declaring a bare cost class.
- The manifest version window is carried as plain integers; when the stable manifest-hardening lane exposes a typed manifest-version range, this should share it rather than re-declare min / max integers.
- Trust-tier, lifecycle, compatibility-label, install-scope, revocation-posture, mirrorability, stability-tier, and claim-basis vocabularies are closed string sets shared with the other stable extension lanes; when those crates stabilize typed enums, these should be narrowed to share them rather than re-declared as strings.
- The compatibility-shim availability is a single class; a later revision should bind the shim's own published version and conformance state rather than a bare availability flag.
