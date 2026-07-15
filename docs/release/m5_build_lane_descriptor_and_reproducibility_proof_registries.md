# M5 build-lane-descriptor and reproducibility-proof registries

This lane is the first implement lane over the frozen
[M5 build-lane-trust matrix](./m5_build_lane_trust_contract.md). It turns the *build-lane-descriptor* grammar
(how a lane declares its allowed cache reads / writes, its controlled credential class, its publication rights,
and the artifact families it is expected to produce) and the *reproducibility-proof* grammar (how a release or
emergency-hotfix lane proves its inputs came from a verified cache or were re-materialized and that binaries,
packages, SBOMs, symbols, docs, and rollback metadata converge on one exact build identity) into registry
resolvers that produce export-safe, honest projections, so the build-farm, cache-service, release-center,
shiproom, provenance, diagnostics, docs, CLI, and support surfaces resolve one canonical build-lane truth
instead of a per-lane, hand-copied reconstruction. The build-lane descriptor and the reproducibility proof are
separated in runtime and serialized state: the cache posture, cache read / write scopes, controlled credential
class, publication rights, expected artifact families, hermetic-input posture, and clean-room rebuild rule live
on the descriptor, while the resolved exact build identity, verified-versus-re-materialized input-source
ledger, clean-room rebuild diff reference, sidecar-convergence state, attestation state, rollback-metadata
reference, and last rebuild revision live on the reproducibility proof, and an untrusted lane's publication
authority stays bounded so a contributor / PR lane never publishes a release artifact.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_build_lane_descriptor_and_reproducibility_proof_registries` (the authoritative
  validator).
- **Combined schema:**
  `schemas/release/m5-build-lane-descriptor-and-reproducibility-proof-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/release/m5-build-lane-descriptor.schema.json`](../../schemas/release/m5-build-lane-descriptor.schema.json)
  and
  [`schemas/release/m5-reproducibility-proof.schema.json`](../../schemas/release/m5-reproducibility-proof.schema.json)
  as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-build-lane-descriptor-and-reproducibility-proof-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/release/m5-build-lane-descriptor-and-reproducibility-proof-registries/`
  (`build_lane_descriptor_beta_narrowed.json`, `reproducibility_proof_preview_narrowed.json`).

## Two registries

1. **Build-lane descriptor** (`resolve_build_lane_descriptor_entry`) — publishes one typed build-lane-descriptor
   object per lane: the cache posture and canonical posture mode, the cache read scope, the cache write scope,
   the controlled credential class, the publication rights, the expected artifact families, the hermetic-input
   posture, and the clean-room rebuild rule. A clean entry names a canonical registry token, a classified cache
   posture, and a build-lane-trust role, covers the canonical / accessible / audit resolution forms, publishes a
   complete object, bounds its publication authority so an untrusted lane never publishes, and discloses the
   cache-trust marker before a trust-risk cache is read. Otherwise it degrades honestly — a lane whose
   publication authority is unbounded (a PR / contributor lane claiming publish rights) or a trust-risk posture
   that hides its cache-trust marker degrades to
   `descriptor_lets_untrusted_lane_publish_or_hides_cache_trust`, the structured blocker reason a
   publish-from-untrusted-lane attempt must surface.
2. **Reproducibility proof** (`resolve_reproducibility_proof_entry`) — keeps the reproducibility proof honest. A
   clean entry names a classified convergence scope and provides the complete build-identity / input-source-ledger
   / clean-room-diff / sidecar-convergence / attestation / rollback-metadata / last-rebuild-revision proof
   object; a proof that would treat a remote-cache hit as reproducibility proof, hide the verified-versus-
   re-materialized input source, or let a non-hermetic input masquerade as hermetic degrades to
   `reproducibility_proof_treats_cache_hit_as_proof_or_drifts_build_identity`.

## Per-entry build-lane reference

The cache posture carries its canonical mode, and the resolver publishes the full descriptor object, so the
registry — never a hand-copied per-lane assumption — is the single source of truth.
`build_lane_descriptor_object_is_complete` rejects an object missing any field, `untrusted_lane_cannot_publish`
rejects an unbounded publication authority or a hidden cache-trust marker, and
`reproducibility_proof_stays_honest` rejects a proof that has treated a cache hit as reproducibility proof.

| cache posture | posture mode | cache read scope | publication rights | expected artifact families |
| --- | --- | --- | --- | --- |
| hermetic no cache | hermetic_no_cache_posture | `cache.read.none` | `publication.controlled-release-publication` | `artifacts.binaries-packages-sboms-symbols-docs` |
| verified inputs only | verified_inputs_only_posture | `cache.read.verified-inputs-only` | `publication.controlled-release-publication` | `artifacts.binaries-packages-sboms` |
| shared readable untrusted | shared_readable_untrusted_posture | `cache.read.shared-readable` | `publication.withheld` | `artifacts.none-release-bearing` |

A publish-from-untrusted attempt degrades to `descriptor_lets_untrusted_lane_publish_or_hides_cache_trust`, an
incomplete object degrades to `build_lane_descriptor_object_incomplete`, and a cache hit treated as proof
degrades to `reproducibility_proof_treats_cache_hit_as_proof_or_drifts_build_identity`, so a
publish-from-untrusted attempt, an incomplete object, or a cache hit treated as proof can never turn release
evidence green.

## Acceptance criteria (proven by resolved examples)

- **Every lane exposes a typed descriptor with cache, credential, and publication boundaries.** Clean descriptor
  entries cover the canonical hermetic / verified / shared-untrusted / remote-publishing / mirror-replay cache
  postures and the first release-center / shiproom / diagnostics / provenance / support surfaces, an
  object-incomplete example degrades, and no clean descriptor entry published an incomplete object.
- **Attempting to publish from an untrusted lane fails with a structured blocker reason.** A publish-from-untrusted
  example and an unbound example degrade, a clean bounded descriptor entry is present, and no clean entry is
  unbounded or unbound.
- **Release packets can prove which lane produced each claimed artifact family.** Clean reproducibility-proof
  entries cover the verified-cache / re-materialized / hermetic-rebuild convergence scopes with full
  resolution-form coverage while providing the complete proof object — the resolved exact build identity and the
  verified-versus-re-materialized input-source ledger — and a proof that would treat a remote-cache hit as
  reproducibility proof or drift the build identity degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_build_lane_descriptor_and_reproducibility_proof_registries -- support-export
cargo run -p aureline-ui --example dump_m5_build_lane_descriptor_and_reproducibility_proof_registries -- csv
cargo run -p aureline-ui --example dump_m5_build_lane_descriptor_and_reproducibility_proof_registries -- report
cargo run -p aureline-ui --example dump_m5_build_lane_descriptor_and_reproducibility_proof_registries -- build-lane-descriptor-table
cargo run -p aureline-ui --example dump_m5_build_lane_descriptor_and_reproducibility_proof_registries -- fixture-build-lane-descriptor-beta-narrowed
cargo run -p aureline-ui --example dump_m5_build_lane_descriptor_and_reproducibility_proof_registries -- fixture-reproducibility-proof-preview-narrowed
```
