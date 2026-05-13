# Prebuild Descriptor Alpha Seed

This document defines the alpha seed lane for prebuild metadata and warm-start
descriptors. The seed lets launch-bundle, project-entry, Start Center, and
support review surfaces describe cached, warmed, stale, and resume-capable
runtime state without claiming that a full prebuild service is productized.

## Companion Artifacts

- [`/schemas/runtime/prebuild_descriptor_alpha.schema.json`](../../schemas/runtime/prebuild_descriptor_alpha.schema.json)
  defines the alpha seed schema for `prebuild_descriptor_alpha_record` and
  `warm_start_descriptor_seed_manifest`.
- [`/artifacts/templates/warm_start_descriptor_seed.yaml`](../../artifacts/templates/warm_start_descriptor_seed.yaml)
  is the first seed manifest. It binds TypeScript web, Python devcontainer, and
  managed resume metadata rows to environment capsule and launch-bundle refs.
- [`/crates/aureline-shell/src/start_center/mod.rs`](../../crates/aureline-shell/src/start_center/mod.rs)
  is the first consumer. Start Center projects descriptor rows and refuses raw
  secrets, raw command lines, or unreviewable warm-start claims.
- [`/ci/check_prebuild_descriptor_alpha.py`](../../ci/check_prebuild_descriptor_alpha.py)
  validates the schema, seed artifact, launch-bundle backrefs, protected fixture
  ids, docs, and shell consumer.
- [`/fixtures/runtime/prebuild_descriptor_alpha/manifest.json`](../../fixtures/runtime/prebuild_descriptor_alpha/manifest.json)
  fixes the protected descriptor ids and acceptance states.

The descriptor seed consumes the environment-capsule alpha contract in
[`/schemas/runtime/environment_capsule_alpha.schema.json`](../../schemas/runtime/environment_capsule_alpha.schema.json)
and the execution-context alpha vocabulary in
[`/schemas/runtime/execution_context_alpha.schema.json`](../../schemas/runtime/execution_context_alpha.schema.json).

## Contract

Each descriptor names:

- source identity: source class, producer class, source ref, digest, and source
  revision ref;
- freshness: freshness state, observation time, age class, allowed age window,
  and evidence ref;
- target: target class, boundary class, target identity, capsule location, and
  requested source artifact;
- compatibility fingerprint: capsule ref/hash, source digest set, policy epoch,
  platform/arch, extension lock digest, and critical toolchain digests;
- warm-start posture: warm state, prebuild reuse state, cache disposition,
  warmed artifact refs, invalidation reason when present, resume capability,
  fallback action, materializer claim, and live-runtime claim;
- safety: trust state, identity mode, policy epoch, no raw secrets, no raw
  command lines, revalidation requirement, review requirement, and support
  export class.

The seed intentionally carries `metadata_only_no_materializer_claim` on every
row. A row can say that metadata exists for a warm start, a stale snapshot, or a
managed resume candidate, but it cannot imply that Aureline has already attached
to or recreated that runtime.

## Guardrails

- Warm-start descriptors are accelerators and review inputs, not authorities.
  Source revision, target class, and freshness remain visible on every row.
- Stale descriptors require an invalidation reason and drift marker before they
  can appear in launch-bundle or project-entry review.
- Resume-capable descriptors require explicit reauth or revalidation posture;
  cached metadata must not widen authority or fake live availability.
- Launch bundles reference the same descriptor ids under `prebuild_descriptor_refs`
  so bundle review and Start Center do not invent different warmed/live labels.
- Support exports preserve metadata and evidence refs without embedding secrets,
  raw environment bodies, or raw command lines.

## Verification

Run:

```sh
python3 ci/check_prebuild_descriptor_alpha.py --repo-root . --render-warm-start-gallery
cargo test -p aureline-shell start_center::tests::warm_start_descriptor
```
