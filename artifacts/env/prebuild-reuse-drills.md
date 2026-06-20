# Prebuild-reuse failure / recovery drills

These drills exercise the prebuild-snapshot reuse engine through each named drift
class — source drift, policy drift, platform drift, extension-lock drift, and
partial artifact loss — from an injected failure through the warm-start downgrade
and back to recovery. Each drill is computed from the same
`evaluate_prebuild_reuse` engine the cases and fixtures use, so the drills cannot
disagree with the certification. The machine-readable form is the `drills` array
of [`artifacts/env/prebuild-fingerprint-packet.json`](prebuild-fingerprint-packet.json).

Every drill runs six phases: **inject** (the drift is introduced under a snapshot
that still loads warm), **observe** (the recorded fingerprint is compared
key-by-key against current truth), **narrow** (the engine narrows or rejects
reuse), **refresh** (the affected layer is rebuilt and re-fingerprinted),
**recover** (the fingerprint matches again), and **verify** (the recovered
outcome matches the engine for a fully current snapshot).

## Drill summary

| Drill | Injected drift | Degraded outcome | Reuse blocked | Recovers to |
| --- | --- | --- | --- | --- |
| `drill.prebuild.source_drift` | source-tree edit | cold | yes | warm |
| `drill.prebuild.policy_drift` | policy-epoch advance | invalidated | yes | warm |
| `drill.prebuild.platform_drift` | platform / arch change | invalidated | yes | warm |
| `drill.prebuild.extension_lock_drift` | extension-lock change | partially warm | no | warm |
| `drill.prebuild.partial_artifact_loss` | lost search index | partially warm | no | warm |

## Source drift

A source-tree edit lands under a pinned snapshot. The `source_tree_identity` key
(identity class) drifts, so the engine rejects the snapshot for a **cold** build
rather than serving a stale tree's tools and indexes as current truth. Reuse is
blocked; once the snapshot is rebuilt and re-fingerprinted against the current
tree, reuse returns to warm.

## Policy drift

The policy epoch advances while the snapshot stays pinned. The `policy_epoch` key
(invalidating class) drifts, so the snapshot is **invalidated** and evicted — it
must not carry capabilities the current policy may disallow. Reuse is blocked
until a fresh snapshot is built under the current epoch.

## Platform drift

The platform / architecture changes under the snapshot. The `platform_arch` key
(invalidating class) drifts, so the binary-incompatible snapshot is
**invalidated** and evicted rather than reused on the wrong host. Reuse is blocked
until a snapshot is built for the current platform.

## Extension-lock drift

The resolved extension lock changes. The `extension_lock_digest` key (layered
class) drifts, so the engine narrows reuse to **partially warm**: the base,
toolchain, and dependency layers stay warm while the extension layer is rebuilt.
Reuse continues partially, with `language_intel` degraded until the extension
layer is rebuilt.

## Partial artifact loss

The cached search-index artifact is lost while every fingerprint key still
matches. Losing a non-critical layer narrows reuse to **partially warm**: only
the index is rebuilt, and `search_index` (and the `language_intel` that depends
on it) are gated until it returns. Reuse continues partially.

## What the drills prove

- Source, policy, and platform drift block warm reuse — a fast-loading snapshot
  never outruns them.
- Layered drift and partial artifact loss narrow reuse to partial rather than
  rejecting it, rebuilding only the affected layer.
- Every drill recovers to warm once the snapshot is rebuilt and re-fingerprinted,
  and the recovered outcome matches the engine for a fully current snapshot.
