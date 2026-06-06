# Stabilize Extension Dependency Resolution And Publisher Continuity

**Status:** Stable dependency-resolution and publisher-continuity lane implemented in `crates/aureline-extensions`.

## Goal

This lane makes extension install truth reproducible across public-registry, approved-mirror, and enterprise-curated installs. A stable row now carries one packet for dependency declarations, deterministic resolver output, effective permissions after hard-dependency resolution, re-consent on authority widening, publisher-continuity workflow state, yanks/revocations/last-known-good pins, and SDK/API deprecation propagation.

The packet is canonical for UI install/update review, mirror review, rollback review, CLI/headless output, migration docs, claim packets, support exports, and enterprise admin exports. Consumers should ingest the checked-in packet instead of inferring risk from the top-level manifest or from a catalog badge.

## Stable Rules

- Hard dependencies, optional integrations, API ranges, runtime ranges, lockfile refs, and install-export refs are explicit.
- Effective permissions are derived from `declared permissions union resolved hard-dependency permissions`; optional-integration permissions are surfaced separately.
- Re-consent is required when dependency resolution expands the effective permission set, even if the top-level manifest did not change.
- Publisher continuity workflows cover key rotation, ownership transfer, namespace disputes, maintainer removal, orphan adoption, and approved-mirror succession.
- High-trust auto-update resumes only after delay/cooldown, audit trail, user/admin notification, transfer-history preservation, and package-identity continuity are proven.
- Yanks, revocations, last-known-good pins, and policy-allowed hold/downgrade behavior remain explicit in public, mirror, rollback, and offline-support flows.
- SDK/API deprecations must flow into resolver output, install warnings, migration docs, and compatibility shims where feasible.
- Claim packets and migration docs narrow automatically when dependency, continuity, revocation, or deprecation proof is stale or incomplete.

## Record Kinds

| Record | Purpose |
|---|---|
| `extension_dependency_resolution_packet` | Top-level packet for stable dependency resolution and continuity truth. |
| `ExtensionDependencyResolutionInput` | Authoring input used by fixtures and generators. |
| `DependencyResolution` | Deterministic resolver output, hard dependencies, optional integrations, ranges, lockfile, and install export. |
| `EffectivePermissionResolution` | Derived declared/inherited/effective/optional/prior/expanded permissions plus re-consent state. |
| `PublisherContinuityInput` | Continuity workflow, state, delay, audit, notifications, gate, and identity preservation. |
| `RevocationPinInput` | Yank/revocation propagation, last-known-good pin, policy hold/downgrade, and rollback export. |
| `DeprecationPropagationInput` | SDK/API deprecation state and propagation into resolution, warnings, docs, shims, and claim proof. |
| `DependencyResolutionQualificationClaim` | Claimed/effective tier, support claim, and machine-readable narrowing reasons. |
| `DependencyResolutionInspection` | Compact UI/CLI/support projection. |
| `extension_dependency_resolution_support_export` | Metadata-safe support and enterprise export. |

## Canonical Fixtures

Fixtures live under `fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/`:

- `public_install_stable.json`
- `mirrored_update_permission_widening_reconsent_stable.json`
- `enterprise_curated_install_stable.json`
- `rollback_last_known_good_stable.json`
- `key_rotation_cooldown_narrows_to_beta.json`
- `ownership_transfer_pending_notification_narrows_to_preview.json`
- `namespace_dispute_withdrawn.json`
- `maintainer_removal_pending_review_narrows_to_preview.json`
- `orphan_adoption_pending_review_narrows_to_preview.json`
- `approved_mirror_succession_stable.json`

## How To Verify

```bash
cargo test -p aureline-extensions stabilize_extension_dependency_resolution
cargo run -q -p aureline-extensions --example dump_extension_dependency_resolution_records -- validate
```

Schema: `schemas/extensions/extension-dependency-resolution.schema.json`.
