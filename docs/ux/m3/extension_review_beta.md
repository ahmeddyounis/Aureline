# Extension Mutation Review Beta

This page describes the shell-owned review surface implemented in
`crates/aureline-shell/src/extensions/review/`.

The surface joins the existing extension contracts into one review packet
before an extension install, update, disable, or rollback can mutate installed
state. It does not define a marketplace or installer. It is the product-owned
review layer that reads manifest, permission, registry, runtime-budget, and
rollback truth from the lower-level contracts.

## Required Review Truth

Every mutation review renders:

| Concern | Source |
|---|---|
| Publisher identity and continuity | `ExtensionReviewAlphaPacketRecord` plus `PublisherContinuityAlphaRecord` |
| Source lane | `ManifestOriginSourceClass` and `InstallReviewContentSourceClass` |
| Permission delta | `DeclaredVsEffectiveDiffEntry` and, for updates, `PermissionManifestDeltaRecord` |
| Compatibility and activation budget | `InstallReviewAlphaPacketRecord` |
| Rollback implication | `ExtensionMutationStatePlan` |
| Installed/cache/revoked remainder | `ExtensionMutationStatePlan` |

Install and update reviews require compatibility range, activation triggers,
runtime budget class, activation evidence, publisher identity, permission
delta, source lane, native authority, and rollback implications before the
native sheet can commit.

Disable and rollback reviews require user-owned state preservation and must
state what remains installed, cached, and revoked. A rollback review also
requires both a rollback checkpoint and a last-known-good version.

## Source-Lane Vocabulary

The shell review uses the same record shape for:

| Lane | `ManifestOriginSourceClass` |
|---|---|
| Primary registry | `public_registry` |
| Private registry | `private_registry` |
| Approved mirror | `mirror` |
| Offline bundle | `offline_bundle` |
| Manual import / local archive | `vendored_local` |

The source lane changes provenance and freshness labels only. It does not
change permission-delta, compatibility, activation-budget, rollback, or
state-preservation vocabulary.

## Decisions

| Decision | Meaning |
|---|---|
| `ready_for_native_mutation` | Product-owned native review can commit. |
| `awaiting_user_review` | User acknowledgement or permission re-consent is still required. |
| `awaiting_admin_review` | Admin, mirror operator, or policy owner must act first. |
| `denied` | Mutation is refused. |

The validator refuses opaque publisher identity, unknown source lanes, missing
permission deltas, missing compatibility or activation evidence, permission
widening without re-consent, missing rollback truth, and disable/rollback
plans that do not preserve user-owned state.

## Fixtures

Fixtures live under `fixtures/extensions/m3/install_update_review/`:

| Fixture | Covers |
|---|---|
| `install_primary_registry_ready.json` | Primary registry install with permission delta, compatibility, activation budget, publisher identity, and fresh-install rollback posture. |
| `update_mirror_permission_widening_requires_review.json` | Mirror update with a version-to-version permission widening that requires re-consent. |
| `disable_manual_import_preserves_state.json` | Manual import disable that keeps user state and explains installed/cache/revoked remainder. |
| `rollback_offline_bundle_ready.json` | Offline bundle rollback to last-known-good with checkpoint and retained audit state. |

Verify with:

```bash
cargo test -p aureline-shell extensions::review
```
