# M5 verified-input-manifest and sidecar-completeness-manifest registries

This lane is the input-materialization and sidecar-completeness implement lane over the frozen
[M5 build-lane-trust matrix](./m5_build_lane_trust_contract.md). It turns the *verified-input-manifest* grammar
(how a lane captures the build-config digest, the materialized-input receipt, the input provenance ledger, the
verification authority it is bounded to, the artifact families it expects, the hermetic-input posture, and the
re-materialization rule) and the *sidecar-completeness-manifest* grammar (how a release or emergency-hotfix lane
proves that binaries, packages, docs packs, schemas, SBOMs, symbols, source maps, and rollback metadata are all
present and bound to one exact build identity) into registry resolvers that produce export-safe, honest
projections, so the build-farm, cache-service, release-center, shiproom, provenance, diagnostics, docs, CLI, and
support surfaces resolve one canonical exact-build truth instead of a per-lane, hand-copied reconstruction. The
verified-input manifest and the sidecar-completeness manifest are separated in runtime and serialized state: the
input source, build-config digest, materialized-input receipt, input provenance ledger, verification authority,
expected artifact families, hermetic-input posture, and re-materialization rule live on the verified-input
manifest, while the resolved exact build identity, claimed artifact families, sidecar-family ledger,
binding-identity check, missing-or-mismatched reference, attestation state, and last convergence revision live on
the sidecar-completeness manifest, and an unverified input's admission authority stays bounded so an unverified or
non-materialized input never enters a protected lane.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_verified_input_manifest_and_sidecar_completeness_registries` (the authoritative
  validator).
- **Combined schema:**
  `schemas/release/m5-verified-input-manifest-and-sidecar-completeness-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/release/m5-verified-input-manifest.schema.json`](../../schemas/release/m5-verified-input-manifest.schema.json)
  and
  [`schemas/release/m5-sidecar-completeness-manifest.schema.json`](../../schemas/release/m5-sidecar-completeness-manifest.schema.json)
  as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-verified-input-manifest-and-sidecar-completeness-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/release/m5-verified-input-manifest-and-sidecar-completeness-registries/`
  (`verified_input_beta_narrowed.json`, `sidecar_completeness_preview_narrowed.json`).

## Two registries

1. **Verified-input manifest** (`resolve_verified_input_manifest_entry`) — publishes one typed
   verified-input-manifest object per lane: the input source and canonical source mode, the build-config digest,
   the materialized-input receipt, the input provenance ledger, the verification authority, the expected artifact
   families, the hermetic-input posture, and the re-materialization rule. A clean entry names a canonical registry
   token, a classified input source, and a build-lane-trust role, covers the canonical / accessible / audit
   resolution forms, publishes a complete object, bounds its verification authority so an unverified input never
   enters a protected lane, and discloses the input-trust marker before a trust-risk input is admitted. Otherwise
   it degrades honestly — a lane whose verification authority is unbounded (an unverified or non-materialized
   input claiming protected-lane admission) or a trust-risk source that hides its input-trust marker degrades to
   `manifest_admits_unverified_input_or_hides_digest`, the structured blocker reason an admit-unverified-input
   attempt must surface.
2. **Sidecar-completeness manifest** (`resolve_sidecar_completeness_manifest_entry`) — keeps the
   sidecar-completeness manifest honest. A clean entry names a classified convergence scope and provides the
   complete build-identity / claimed-families / sidecar-ledger / binding-identity / missing-or-mismatched /
   attestation / last-convergence-revision manifest object; a manifest that would let a green build omit a claimed
   sidecar family, bind a sidecar to a different build identity, or treat a missing or mismatched sidecar as
   warning-only degrades to `sidecar_family_missing_or_mismatched_or_drifts_build_identity`.

## Per-entry manifest reference

The input source carries its canonical mode, and the resolver publishes the full manifest object, so the
registry — never a hand-copied per-lane assumption — is the single source of truth.
`verified_input_manifest_object_is_complete` rejects an object missing any field,
`unverified_input_cannot_enter_protected_lane` rejects an unbounded admission authority or a hidden input-trust
marker, and `sidecar_family_stays_converged` rejects a manifest that has let a missing or mismatched sidecar read
as a clean pass.

| input source | source mode | build-config digest | verification authority | expected artifact families |
| --- | --- | --- | --- | --- |
| rematerialized from source | rematerialized_from_source_input | `build-config.sha256.release-0007` | `verification.release-signing-scoped` | `artifacts.binaries-packages-sboms-symbols-docs` |
| verified cache input | verified_cache_input_mode | `build-config.sha256.protected-merge-0007` | `verification.controlled-scoped-to-lane` | `artifacts.binaries-packages-sboms` |
| unverified external input | unverified_external_input_mode | `build-config.sha256.contributor-pr-0007` | `verification.pr-scoped-only` | `artifacts.none-release-bearing` |

An admit-unverified attempt degrades to `manifest_admits_unverified_input_or_hides_digest`, an incomplete object
degrades to `verified_input_manifest_object_incomplete`, and a missing sidecar degrades to
`sidecar_family_missing_or_mismatched_or_drifts_build_identity`, so an admit-unverified attempt, an incomplete
object, or a missing sidecar can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Every lane exposes a typed manifest with build-config-digest, receipt, and verification boundaries.** Clean
  manifest entries cover the canonical rematerialized / verified-cache / pinned-digest / unverified-external /
  non-materialized input sources and the first release-center / shiproom / diagnostics / provenance / support
  surfaces, an object-incomplete example degrades, and no clean manifest entry published an incomplete object.
- **Attempting to admit an unverified input into a protected lane fails with a structured blocker reason.** An
  admit-unverified example and an unbound example degrade, a clean bounded manifest entry is present, and no clean
  entry is unbounded or unbound.
- **Claimed release artifacts and required sidecars converge on one exact-build descriptor.** Clean
  sidecar-completeness entries cover the binary-identity / receipt-reconciled / hermetic-rebuild convergence
  scopes with full resolution-form coverage while providing the complete manifest object — the resolved exact
  build identity and the sidecar-family ledger — and a manifest that would let a green build omit a claimed sidecar
  or drift the build identity degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_verified_input_manifest_and_sidecar_completeness_registries -- support-export
cargo run -p aureline-ui --example dump_m5_verified_input_manifest_and_sidecar_completeness_registries -- csv
cargo run -p aureline-ui --example dump_m5_verified_input_manifest_and_sidecar_completeness_registries -- report
cargo run -p aureline-ui --example dump_m5_verified_input_manifest_and_sidecar_completeness_registries -- verified-input-manifest-table
cargo run -p aureline-ui --example dump_m5_verified_input_manifest_and_sidecar_completeness_registries -- fixture-verified-input-beta-narrowed
cargo run -p aureline-ui --example dump_m5_verified_input_manifest_and_sidecar_completeness_registries -- fixture-sidecar-completeness-preview-narrowed
```
