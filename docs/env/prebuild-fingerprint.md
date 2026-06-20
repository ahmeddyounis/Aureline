# Prebuild-snapshot compatibility fingerprints

This document describes the prebuild-snapshot compatibility fingerprint, its
invalidation rules, and the cold-versus-partially-warm downgrade truth for
claimed M5 warm-start paths. The canonical implementation is
[`crates/aureline-env/src/prebuilds/mod.rs`](../../crates/aureline-env/src/prebuilds/mod.rs);
the packet, corpus, and expected decisions are checked in under
[`artifacts/env/prebuild-fingerprint-packet.json`](../../artifacts/env/prebuild-fingerprint-packet.json)
and [`fixtures/env/prebuilds/`](../../fixtures/env/prebuilds/), and the
failure / recovery drills are written up in
[`artifacts/env/prebuild-reuse-drills.md`](../../artifacts/env/prebuild-reuse-drills.md).

It builds on the typed environment capsule
([`docs/env/environment-capsule.md`](environment-capsule.md)) and the
environment-capsule governance matrix
([`docs/env/m5-env-governance.md`](m5-env-governance.md)). The governance lane
proves a prebuild fingerprint *exists* and stays fresh; this lane makes that
fingerprint *operational* — it decides, for a concrete snapshot, whether a warm
start may reuse it, and says why.

## Why this exists

A prebuild snapshot is an accelerator: it can load in milliseconds. But speed is
never proof of compatibility. A snapshot that loads fast but was built from a
different source tree, under an older policy epoch, or for a different platform
would present stale tools and indexes as current truth. This lane makes that
impossible: before a warm start reuses a snapshot, the snapshot's recorded
fingerprint must still match current truth, and any mismatch downgrades the start
visibly instead of winning by loading quickly.

## The six fingerprint keys

A `PrebuildFingerprint` is keyed on the six inputs that actually decide
compatibility, one per `FingerprintKey`:

| Key | Class | Drift outcome | Absent outcome |
| --- | --- | --- | --- |
| `source_tree_identity` | identity | cold | cold |
| `capsule_hash` | identity | cold | cold |
| `platform_arch` | invalidating | invalidated | invalidated |
| `policy_epoch` | invalidating | invalidated | invalidated |
| `extension_lock_digest` | layered | partially warm | cold |
| `toolchain_digest` | layered | partially warm | cold |

Each key carries a `CompatibilityClass` that fixes its downgrade floor:

- **Invalidating** (platform/arch, policy epoch) — a drift invalidates the
  snapshot outright. A snapshot built for a different platform is
  binary-incompatible; one built under an older policy epoch may carry
  capabilities the current policy disallows. Neither may be partially reused; the
  snapshot is discarded and evicted.
- **Identity** (source tree, capsule hash) — a drift means the snapshot is for
  different content. Reuse is rejected and the environment is rebuilt cold (a
  benign cache miss, not an eviction).
- **Layered** (extension lock, toolchain) — a drift affects only a layer. The
  unaffected layers stay warm while the affected layer is rebuilt, so the start
  is partially warm. A *missing* layered key cannot prove the layer, so it drops
  to a cold rebuild rather than a partial one.

The combined `fingerprint` digest is folded over the keyed digests in canonical
order, so it is a deterministic, reproducible function of the keys.

## Artifact integrity

Beyond the fingerprint keys, a snapshot carries the integrity of its cached
artifact layers (`base_image`, `toolchain`, `dependencies`, `extensions`,
`search_index`). Losing a **non-critical** layer narrows the start to partially
warm and rebuilds just that layer; losing a **critical** layer (the base image or
toolchain) forces a cold build, because nothing can be rebuilt over it.

## The four starts

`evaluate_prebuild_reuse` compares a snapshot's recorded fingerprint against the
current expected fingerprint, folds in artifact integrity, and returns one
explicit `StartOutcome`:

- **warm** — every key matches and every critical artifact is intact; the whole
  snapshot is reused.
- **partially warm** — only part is reused; an affected layer is rebuilt.
- **cold** — no reuse is trustworthy for current content; rebuilt cold (the
  snapshot is a benign cache miss).
- **invalidated** — the snapshot is binary- or trust-incompatible and is
  discarded; rebuilt and evicted.

The coldest contribution among the six keys and the artifact layers always wins,
so an invalidating drift can never be masked by a layer that still matches. Each
outcome maps onto the same governance `WarmStartPosture` the environment-capsule
lane reads (`warm` → `warm_full_reuse`, `partially warm` → `warm_partial_reuse`,
`cold`/`invalidated` → `cold_build`), so the prebuild lane narrows in lockstep
instead of forking a parallel warm-start model.

## Narrowing or disabling actions

A downgraded start must not present stale tools or indexes as current truth, so
the decision carries a per-action `CapsuleActionClass` gate (`build_run`,
`search_index`, `language_intel`, `services`). A warm start leaves every action
available; a cold or invalidated start disables every snapshot-served action
until the rebuild; a partially warm start keeps actions whose backing keys and
layers are intact available and **degrades** the rest. For example, an
extension-lock drift degrades `language_intel` while `build_run` stays available,
and a lost search index gates `search_index` and `language_intel` while build and
services keep running.

## Explainability and metadata-first export

The `PrebuildDecision` is the single explainability object desktop, headless, and
support surfaces all read (`desktop_prebuild_decision`,
`headless_prebuild_decision`, `support_prebuild_decision`). It carries the
outcome, the dominant `PrebuildReason`, the reason tokens, the per-key and
per-artifact evaluation, the gated actions, and a review-safe headline — and
nothing else. Every field is an id, digest, version, enum, or review-safe prose;
`export_prebuild_decision` projects a metadata-first `PrebuildExport` (always
`metadata_only`) so support and release surfaces can distinguish a warm,
partially warm, cold, or invalidated start without secrets, raw bodies, or
provider payloads.

## Guardrails

- A prebuild snapshot is reused only when every fingerprint key matches current
  truth and its critical artifacts are intact; speed is never proof of
  compatibility.
- Source, policy, and platform drift can never be silently outrun: each rejects
  or invalidates the snapshot rather than serving it.
- The engine only narrows; it never promotes a snapshot above the outcome its
  evidence supports, and the invalidation rules are derived from the
  compatibility class so they cannot drift from the engine.
