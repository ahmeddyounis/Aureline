# Prebuild fingerprint, invalidation, and disclosure contract

This document freezes the workspace-level contract Aureline uses to
decide whether a prepared environment can be reused, why it cannot be
reused, and how that decision is disclosed to users and support. A
prebuild is an accelerator. It is not the authority for source,
policy, trust, credentials, ports, indexes, or live session state.

If this document disagrees with the PRD, Technical Architecture
Document, Technical Design Document, UI / UX Spec, or Design System
Style Guide, those source documents win and this document MUST be
updated in the same change.

## Companion Artifacts

- [`/schemas/workspace/prebuild_fingerprint.schema.json`](../../schemas/workspace/prebuild_fingerprint.schema.json)
  - boundary schema for `prebuild_fingerprint_record`,
  `prebuild_reuse_decision_record`, and
  `prebuild_disclosure_record`.
- [`/schemas/workspace/prebuild_invalidation_reason.schema.json`](../../schemas/workspace/prebuild_invalidation_reason.schema.json)
  - boundary schema for `prebuild_invalidation_reason_record` and
  `prebuild_invalidation_bundle_record`.
- [`/fixtures/workspace/prebuild_cases/`](../../fixtures/workspace/prebuild_cases/)
  - seeded YAML cases covering valid reuse, stale dependency drift,
  policy and trust drift, secret-handle revalidation, missing
  artifacts, stale indexes, local overrides, and the distinction
  between live resume, snapshot start, fresh clone, and cached
  prebuild reuse.
- [`/artifacts/entry/warm_start_chooser_contract.md`](../../artifacts/entry/warm_start_chooser_contract.md)
  and [`/schemas/entry/freshness_revalidation.schema.json`](../../schemas/entry/freshness_revalidation.schema.json)
  - lane-level disclosure contract for how entry surfaces must present
  freshness/age and revalidation truth before commit (and keep it
  exportable through open, restore, and support packets).

Related contracts this document composes with:

- [`/docs/runtime/environment_capsule_contract.md`](../runtime/environment_capsule_contract.md)
  for capsule identity, toolchain identity, redaction posture, and
  capsule drift state.
- [`/docs/runtime/execution_context_vocabulary.md`](../runtime/execution_context_vocabulary.md)
  for execution-context references, target identity, cache
  disposition, and provenance export.
- [`/docs/runtime/storage_classes_and_gc.md`](../runtime/storage_classes_and_gc.md)
  for the `prebuild_environment_cache` class and cache-manager
  pinning / low-disk rules.
- [`/docs/workspace/entry_restore_object_model.md`](./entry_restore_object_model.md)
  for `resume`, `start_from_snapshot`, and clone/open entry verbs.
- [`/docs/workspace/source_acquisition_and_bootstrap_seed.md`](./source_acquisition_and_bootstrap_seed.md)
  for source locator, checkout plan, trust-stage, and bootstrap queue
  vocabulary.
- [`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md)
  for card-level disclosure, equal-weight bypass paths, and lane
  separation in the template / prebuild / resume-live surfaces.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  for secret-broker handle and raw-secret redaction rules.

## Scope

Frozen at this revision:

- the fingerprint dimensions that make a prebuild reusable:
  source revision, environment capsule, toolchain identity, platform
  and host class, policy and feature state, cache class, freshness,
  and redaction posture;
- the invalidation reasons and outcomes for dependency drift,
  environment mismatch, policy / feature changes, trust changes,
  secret-handle changes, credential revalidation, port rebinds, stale
  indexes, missing artifacts, freshness expiry, and false warm-start
  or resume claims;
- the disclosure record every Start Center, prebuild picker,
  resume-live card, CLI / headless entry flow, support export, and
  Project Doctor finding reads;
- the portable-artifact exclusion rule that prebuild manifests and
  exports carry metadata, hashes, and opaque refs only, never raw
  secret material or host-specific residue.

Out of scope:

- managed fleet scheduling, cloud prebuild production, OCI layer
  distribution, remote cache services, and runtime materializer
  implementation;
- final user-facing copy. This contract freezes the record fields and
  closed vocabularies the copy resolves against.

## Record Family

### `prebuild_fingerprint_record`

One record describes one prepared environment candidate. It MUST carry:

- `source_identity`: repository / root refs, commit or tree identity,
  branch intent when present, submodule / LFS / sparse posture, and
  dependency lock digests used to build the prebuild.
- `environment_identity`: environment capsule id/hash/schema,
  workspace template ref, base image or host identity, platform/arch,
  host class, service topology ref, mount model ref, and materializer
  version.
- `toolchain_identity`: critical toolchain digests, extension lock
  digest, package-manager lock digest, and known unsupported gaps.
- `policy_feature_identity`: workspace trust state, policy epoch,
  policy bundle ref, feature flag digest, entitlement snapshot ref,
  sandbox profile, and egress posture.
- `cache_artifacts`: cache classes included (`toolchain_layer`,
  `dependency_store`, `index_shard`, `artifact_mirror`,
  `extension_package`, `docs_pack`, `service_image_layer`) and their
  digest refs.
- `redaction_and_portability`: allowed export fields, support-bundle
  posture, excluded residue classes, and whether a broadened capture
  was explicitly approved.
- `freshness`: creation time, last validation time, maximum reuse age,
  age class, producer class, and signer / mirror posture.

The record MUST NOT include raw credentials, raw environment values,
raw terminal history, raw machine-specific socket paths, raw local
absolute paths outside an approved filesystem-identity record, or raw
uncommitted source bytes.

### `prebuild_invalidation_reason_record`

One record names a single reason a fingerprint cannot be accepted as a
full reuse hit. Reasons are typed by category and effect. They never
encode private prose as the only explanation.

Required dimensions:

- `reason_class`
- `reason_category`
- `detected_at`
- `comparison_refs`
- `reuse_effect`
- `required_revalidation`
- `user_disclosure_class`
- `support_summary`

The companion bundle record groups one or more reasons against one
candidate fingerprint and resolves the final reuse outcome.

### `prebuild_reuse_decision_record`

One record describes the decision made for an entry attempt. It binds:

- requested path: `resume_live_workspace`, `start_from_snapshot`,
  `clone_fresh`, or `reuse_cached_prebuild`;
- candidate fingerprint ref and current fingerprint ref;
- source materialization class: `live_materialized_workspace`,
  `prebuilt_snapshot`, `stale_prebuild_snapshot`,
  `fresh_clone_materialization`, or `local_override_materialization`;
- reuse outcome: `reuse_allowed`, `reuse_after_revalidation`,
  `partial_warm_reuse_only`, `rebuild_required`,
  `clone_fresh_required`, or `resume_live_denied`;
- invalidation bundle refs;
- required revalidation set for credentials, ports, indexes, policy,
  trust, feature flags, source, environment, or host;
- disclosure record ref.

A stale prebuild or snapshot MUST NOT produce a successful
`resume_live_workspace` outcome. If a surface was asked to resume but
only a snapshot or prebuild is available, the decision keeps
`requested_path = resume_live_workspace` for auditability, sets
`reuse_outcome = resume_live_denied`, and offers an alternative lane.

### `prebuild_disclosure_record`

One record is the user / support readable projection of the decision.
It MUST say:

- whether the workspace is live, prebuilt, stale, partially warm,
  rebuilt, cloned fresh, or locally overridden;
- why the prebuild is valid, partially valid, or invalid;
- freshness age and maximum accepted age;
- host class and platform/arch;
- required revalidation before trust: credentials, ports, indexes,
  policy, trust, feature flags, source, environment, host;
- which local override, if any, changed the fingerprint;
- whether rebuild, fresh clone, or inspect-only continuation is
  required;
- which artifact classes were excluded from portable exports and
  support packets.

## Fingerprint Dimensions

| Dimension | Must Match For Full Reuse | Downgrade When It Does Not Match |
|---|---|---|
| Source | repo/root identity, commit or tree, dependency locks, sparse/submodule/LFS posture | dependency or source drift invalidation |
| Environment | capsule hash, template ref, base image or host identity, platform/arch, host class | environment mismatch or host mismatch |
| Toolchain | critical toolchain digests, package manager, extension lock, known unsupported gaps | toolchain mismatch or partial warm reuse |
| Policy / Trust | policy epoch, policy bundle ref, trust state, sandbox profile, egress posture | policy/trust revalidation or rebuild |
| Feature State | feature flag digest, entitlement snapshot, runtime capability envelope | feature mismatch or capability denial |
| Credentials | handle set digest, projection mode, trust-store epoch, expiry posture | credential revalidation required |
| Ports / Routes | declared port set, route dependency class, collision key, exposure policy | port or route revalidation required |
| Indexes | index schema, graph/docs epoch, embedder/model ids where relevant | stale index label or index rebuild |
| Cache Bodies | digest refs for included cache classes and mirror/signature posture | missing artifact or partial warm reuse |
| Redaction | export field policy and excluded residue set | support export blocked until labeled |

## Reuse Rules

1. **Prebuilds are not live sessions.** `resume_live_workspace`
   requires a live runtime identity, current attach authority, current
   policy / entitlement state, and current credential posture. A
   prebuild snapshot can be an alternative lane, never evidence of live
   continuity.
2. **Full reuse requires a matching fingerprint.** Source identity,
   capsule hash, host/platform class, critical toolchain digests,
   policy epoch, trust state, feature flag digest, and extension lock
   digest all match the accepted window.
3. **Partial warm reuse is explicit.** Cache bodies may be reused for
   cold materialization only when the decision says which classes are
   valid and which classes rebuild.
4. **Policy and trust fail closed.** Policy epoch changes, trust
   downgrades, entitlement expiry, or sandbox-profile changes require
   revalidation before any mutating, networked, credentialed, or
   trusted execution path is enabled.
5. **Credentials are handles, not portable state.** Secret-handle set
   changes, trust-store changes, expiry, or lock state never silently
   degrade to raw secrets or stale credentials.
6. **Indexes can be warm but stale.** Stale graph/search/docs indexes
   may accelerate discovery only with stale labels and must rebuild
   before current-truth claims or mutating actions depend on them.
7. **Missing artifacts do not become success.** Missing image layers,
   dependency caches, index shards, extension packages, docs packs, or
   provenance manifests force partial warm reuse or rebuild.
8. **Local overrides are part of the decision.** A user override,
   machine override, branch override, policy override, or environment
   variable override that changes effective execution is disclosed and
   included in the current fingerprint comparison.
9. **Support packets preserve the explanation.** A support packet
   carries fingerprint refs, invalidation bundle refs, disclosure
   records, and redaction summaries sufficient to explain the decision
   without a live managed control plane.

## Path Separation

| Path | Authority Required | Prebuild Role | Required Disclosure |
|---|---|---|---|
| `resume_live_workspace` | live runtime identity, attach authority, current credentials, current policy and trust | none; prebuild may only appear as an alternative lane | live session identity, freshness, host class, required revalidation |
| `start_from_snapshot` | snapshot/prebuild artifact identity and trust review | source materialization path | snapshot age, producer, excluded residue, setup/rebuild requirements |
| `clone_fresh` | source locator and checkout plan | none unless optional cache is reused after clone | source freshness, credential posture, bootstrap queue |
| `reuse_cached_prebuild` | matching fingerprint and accepted policy window | acceleration only | matching dimensions, age, host class, cache classes, any revalidation |

## Export And Residue Rules

Portable prebuild artifacts and default support exports include:

- opaque refs, digests, schema versions, producer/signature posture,
  cache class labels, freshness age classes, and redaction summaries;
- counts of excluded secret or residue classes;
- invalidation reasons and revalidation requirements.

They exclude by contract:

- raw secret material, raw credential bodies, raw bearer tokens,
  private keys, certificates, and session cookies;
- raw environment values and raw command lines unless an explicit
  broadened capture is approved elsewhere;
- OS keychain / enterprise-vault contents;
- SSH agent sockets, Unix sockets, local named pipes, port bindings,
  process ids, cgroup / namespace ids, local UID/GID values, and
  machine-unique trust anchors;
- terminal history, clipboard contents, dirty-buffer recovery
  journals, uncommitted user edits, and local-only logs;
- host absolute paths outside approved filesystem-identity records;
- cache tempdirs and tool state that cannot be replayed from declared
  artifact digests.

## Fixtures

The fixture set under
[`/fixtures/workspace/prebuild_cases/`](../../fixtures/workspace/prebuild_cases/)
anchors these invariants:

- valid cached prebuild reuse with all matching dimensions;
- dependency lock drift requiring rebuild;
- policy and trust drift requiring revalidation;
- secret-handle change requiring credential revalidation;
- stale index / missing artifact producing partial warm reuse;
- stale snapshot denied as a live resume path;
- local override disclosed before rebuild;
- fresh clone path with optional prebuild caches excluded from the
  authority chain.
