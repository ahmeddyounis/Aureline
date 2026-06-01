# Publish stable SDK/deprecation policy, manifest version windows, and ecosystem migration guidance

**Status:** Stable SDK / deprecation policy lane — implemented in `crates/aureline-extensions`.

## Goal

Make the **SDK / API deprecation policy**, the **manifest version windows**, and the **ecosystem migration guidance** of an extension inspectable on the stable ecosystem line. Every claimed stable row carries one canonical, checked-in SDK-policy truth: the **deprecation policy** (deprecation stage, last-supported version window, replacement package or API, whether pinning to a last-known-good version is allowed, and the named affected dependency edges), the **deprecation propagation** (whether the deprecation actually flows into install-time warnings, marketplace cards, dependency-resolution output, the migration docs, and a compatibility shim), the **manifest version window** (min / max / published / row manifest version), the **ecosystem migration guidance** (the `exact` / `translated` / `partial` / `shimmed` / `unsupported` outcome generated from the real imported artifact, the shim availability, and a preserved rollback checkpoint / diagnostics when a mapping is not exact), the permission posture, compatibility, and install / activation-cost / revocation / mirror posture. The **stability qualification** that truth is allowed to claim is derived, not asserted: when the evidence can no longer back a `stable` SDK-policy claim, the visible tier is **automatically narrowed below Stable** (`beta`, `preview`, or `withdrawn`) with machine-readable reasons. The SDK migration console, the deprecation-policy view, dependency-resolution output, install review, the marketplace card, the extension detail view, diagnostics, support exports, the CLI inspector, and release packets ingest this packet instead of leaving a deprecation living only in release notes.

## Design principles

1. **Deprecations are never release-note-only** — A deprecated / sunsetting / removed SDK surface must name a replacement package or API, a last-supported version window, and the affected dependency edges, and must flow into install-time warnings, marketplace cards, dependency-resolution output, and the migration docs. A missing replacement or window narrows to `preview`; an unnamed edge list or an incomplete propagation into install / card / dependency-resolution narrows to `beta`; a missing migration doc narrows to `preview`.
2. **Pinning to a last-known-good version is a stated policy** — The deprecation record always carries the `pin_allowed` / `pin_discouraged` / `pin_blocked` posture, so an admin or author can see whether downgrading / pinning remains allowed.
3. **The SDK lifecycle is visible** — A `removed` SDK withdraws the row; a `sunset`-window SDK narrows to `beta` and raises a review banner. A `deprecated` SDK with a complete, propagated policy and a supportable migration is a valid stable outcome — the *policy* is stable even though the API is deprecated.
4. **Manifest versions sit inside a published window** — The manifest-version-window record carries the min / max / published / row manifest version. A row manifest version outside the supported window narrows to `preview` and raises a review banner. The window bounds are cross-checked at validation time (`min <= published <= max`, `min <= max`).
5. **Migration outcomes come from the real imported artifact** — The migration outcome is one of `exact` / `translated` / `partial` / `shimmed` / `unsupported`. An `unsupported` outcome withdraws the row; a `partial` outcome narrows to `beta`. A non-exact mapping (`partial` / `shimmed` / `unsupported`) must preserve a rollback checkpoint and diagnostics — a missing checkpoint narrows to `preview`. A `shimmed` outcome must carry an available compatibility shim (cross-checked at validation time).
6. **No ambient extension privilege** — The permission posture carries declared / effective / policy-cap refs plus a `widened` flag. A permission set widened beyond the declared manifest or the policy cap withdraws the row (`allows_ambient_extension_privilege == false`).
7. **No unbounded activation cost** — An `unbounded` cost withdraws the row (`allows_unbounded_activation_cost == false`).
8. **No catalog-only trust** — A `stable` tier must be `evidence_backed`; a `catalog_asserted_only` basis narrows below Stable (`allows_catalog_only_trust == false`).
9. **Revocation, mirrorability, and install scope are visible** — A quarantined / revoked revocation posture withdraws; an advisory posture or a not-mirrorable row narrows to `beta`; an undisclosed install scope narrows to `preview`.
10. **No drift** — The effective tier, downgrade verdict, narrowing reasons, and banner are re-derived from the posture at validation time, so a stored packet cannot drift from its truth.

## Record kinds

| Record kind | Purpose |
|---|---|
| `stable_sdk_deprecation_policy_packet` | Top-level packet consumed by the SDK migration console, the deprecation-policy view, dependency-resolution output, install review, the marketplace card, the extension detail view, diagnostics, support export, docs/help, release packets, and the CLI inspector. |
| `stable_sdk_policy_identity` | SDK-policy descriptor ref, row identity, package identity, SDK channel id, SDK-policy version, pinned published profile version, publisher namespace, policy-evidence ref, publisher trust tier, lifecycle state. |
| `stable_sdk_deprecation_policy` | Deprecation stage, last-supported version, replacement kind + ref, pin policy, support-window ref, affected dependency edge count + ref. |
| `stable_deprecation_propagation` | Whether the deprecation flows into install warnings, marketplace cards, dependency-resolution output, migration docs, and a compatibility shim. |
| `stable_manifest_version_window` | Min / max / published / row manifest version. |
| `stable_ecosystem_migration_guidance` | Migration outcome, migration-doc ref, shim availability, preserved-rollback-checkpoint flag, diagnostics ref. |
| `stable_sdk_policy_permission_posture` | Declared / effective / policy-cap permission refs, no-widening flag, re-consent flag. |
| `stable_sdk_policy_compatibility` | Compatibility label, scorecard ref, verified flag. |
| `stable_sdk_policy_install_posture` | Install scope + disclosure, activation cost class, revocation posture, mirrorability, rollback support. |
| `stable_sdk_policy_qualification_claim` | Claimed tier, effective tier after the posture is applied, support claim, narrowing reasons. |
| `stable_sdk_policy_downgraded_banner` | Whether a review banner must display and why. |
| `stable_sdk_deprecation_policy_inspection` | Compact projection (deprecation / window / migration posture) for CLI and dashboard surfaces. |
| `stable_sdk_deprecation_policy_support_export` | Metadata-safe support / partner / mirror export row that preserves the deprecation / manifest-window / migration posture so a reviewer can see why a row is or is not a stable SDK-policy claim. |

## Narrowing buckets

| Tier | Triggered by (examples) |
|---|---|
| **withdrawn** | non-runnable lifecycle, removed SDK stage, unsupported migration, permission widened beyond the declared manifest / policy cap, unbounded activation cost, unsupported compatibility, quarantined/revoked revocation posture |
| **preview** | unpublished profile version, catalog-asserted basis, quarantined trust tier, missing replacement, missing last-supported window, missing migration docs, manifest version out of window, missing rollback checkpoint, unverified compatibility, undisclosed install scope, incomplete attribution |
| **beta** | SDK in sunset window, unnamed dependency edges, incomplete deprecation propagation, partial migration, parity-limited compatibility, advisory revocation posture, not mirrorable |

## Canonical fixtures

Under `fixtures/extensions/m4/publish-stable-sdk-deprecation-policy-manifest-version-windows/`:

- `current_sdk_active_policy_stable.json` — current SDK, manifest in window, exact migration. Holds **Stable**.
- `deprecated_sdk_with_shim_stable.json` — a deprecated SDK surface with a named replacement, a last-supported window, named affected edges, full propagation, and a shimmed migration with a preserved rollback checkpoint; still **Stable** (the deprecation *policy* is complete).
- `sunset_window_narrows_to_beta.json` — the SDK is in the sunset window; narrows to `beta` with a review banner.
- `partial_migration_narrows_to_beta.json` — the migration outcome is only partial; narrows to `beta`.
- `missing_replacement_narrows_to_preview.json` — a deprecated SDK surface with no named replacement; narrows to `preview`.
- `manifest_version_out_of_window_narrows_to_preview.json` — the row's manifest version is above the supported window; narrows to `preview` with a review banner.
- `catalog_asserted_basis_narrows_to_preview.json` — a stable claim asserted only by the catalog with no evidence bundle; narrows to `preview`.
- `unsupported_migration_withdrawn.json` — the imported artifact cannot be mapped onto the current SDK; the row is `withdrawn` with a banner.
- `widened_permission_withdrawn.json` — effective permissions widened beyond the declared manifest and policy cap; the row is `withdrawn` with a banner.

## How to verify

```bash
cargo test -p aureline-extensions publish_stable_sdk
cargo run -q -p aureline-extensions --example dump_stable_sdk_deprecation_policy_records -- validate
```

Materialized packets for every fixture validate against
[`schemas/extensions/stable_sdk_deprecation_policy.schema.json`](../../../schemas/extensions/stable_sdk_deprecation_policy.schema.json)
(checked with a Draft 2020-12 validator).
