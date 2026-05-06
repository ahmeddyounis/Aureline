# Pipeline trust domains, signer boundaries, and artifact-publication matrix

This document makes the release/publication trust story explicit from
build inputs to downloadable artifacts. Reviewers should be able to
trace any published artifact through build, sign, promote, mirror, and
revoke steps **without crossing an unnamed trust boundary**.

This is a pre-implementation contract: it does not stand up CI/CD,
signing infrastructure, or artifact stores. It freezes vocabulary and
matrix rows so later pipeline work extends these rows instead of
inventing parallel trust stories.

Companion artifacts:

- [`/artifacts/release/pipeline_trust_domains.yaml`](../../artifacts/release/pipeline_trust_domains.yaml)
  - machine-readable trust-domain inventory, signer-boundary inventory,
    component trust-boundary matrix, and seeded artifact-publication
    rows.
- [`/schemas/release/artifact_publication_row.schema.json`](../../schemas/release/artifact_publication_row.schema.json)
  - boundary schema for one artifact-publication row.
- [`/fixtures/release/publication_flow_examples/`](../../fixtures/release/publication_flow_examples/)
  - worked examples for each publication lane (developer preview,
    internal test, public stable, mirror-only, emergency offline).

Cross-linked artifacts already in the repository:

- [`/docs/release/build_farm_and_remote_cache_policy.md`](./build_farm_and_remote_cache_policy.md),
  [`/artifacts/release/pipeline_lane_rules.yaml`](../../artifacts/release/pipeline_lane_rules.yaml),
  and
  [`/artifacts/release/cache_trust_classes.yaml`](../../artifacts/release/cache_trust_classes.yaml)
  — lane trust domains, credential boundaries, remote-cache policy, and
  cache non-authority rules.
- [`/docs/release/release_center_object_model_contract.md`](./release_center_object_model_contract.md)
  and
  [`/schemas/release/publish_target.schema.json`](../../schemas/release/publish_target.schema.json)
  — publish-target classes, audience/visibility/mutability disclosure,
  and release-center action semantics.
- [`/docs/release/release_artifact_graph.md`](./release_artifact_graph.md),
  [`/artifacts/release/artifact_graph_rules.yaml`](../../artifacts/release/artifact_graph_rules.yaml),
  and
  [`/artifacts/release/artifact_family_map.yaml`](../../artifacts/release/artifact_family_map.yaml)
  — exact-build identity linkage, artifact graph completeness, and
  retention/revocation granularity expectations per artifact family.
- [`/docs/release/mirror_integrity_and_offline_verification_contract.md`](./mirror_integrity_and_offline_verification_contract.md)
  — mirror continuity, offline verification, and revocation propagation
  contracts.

Normative sources this document projects from:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix AO
  (pipeline trust domains, remote-cache rules, clean-room gates).
- `.t2/docs/Aureline_Technical_Design_Document.md` release-center and
  publish-target architecture sections.

If this document disagrees with those sources, those sources win and
this document plus the companion artifacts update in the same change.

## Terminology

- **Trust domain**: a named environment boundary whose code, inputs,
  and credentials are considered equivalent for the purposes of
  provenance, cache admissibility, and publication authority. Trust
  domains are stable ids.
- **Signer boundary**: the boundary where signing keys (or an equivalent
  signing capability) exist and can be invoked. Build workers MUST NOT
  embed or carry raw signing key material; signing happens across a
  named signer boundary.
- **Publisher boundary**: the boundary that mutates a publish target
  (release channel, update feed, registry listing, mirror feed, offline
  bundle, or support packet). Publisher boundaries are named because
  “built it” is not the same as “published it”.
- **Publication lane**: a distribution intent class used to interpret
  producer/signer/publisher rows. This matrix distinguishes the lanes
  used in review: developer preview, internal test, public stable,
  mirror-only, emergency offline.

## Stable trust-domain ids

Stable trust-domain ids are frozen in
[`/artifacts/release/pipeline_lane_rules.yaml`](../../artifacts/release/pipeline_lane_rules.yaml)
and described (narratively) in
[`/docs/release/build_farm_and_remote_cache_policy.md`](./build_farm_and_remote_cache_policy.md).

The current trust-domain inventory is:

- `untrusted_contributor` — developer workstation and untrusted PR work.
  No release-bearing credentials; never publishes a channel.
- `protected_engineering` — protected merges on trusted CI. May emit
  internal evidence and cache entries; does not publish public channels.
- `release_nightly` — nightly publication credentials; publishes the
  `nightly` channel only.
- `release_preview` — preview publication credentials; publishes the
  `preview` channel only.
- `release_beta` — beta publication credentials; publishes the `beta`
  channel only.
- `release_stable` — stable publication credentials; publishes the
  `stable` channel only.
- `release_lts` — LTS publication credentials; publishes the `lts`
  channel only.
- `release_hotfix` — hotfix publication credentials; publishes the
  `hotfix` channel only.
- `cleanroom_protected` — clean-room rebuild and verification lane. No
  channel publication.
- `release_mirror_or_offline` — mirror and offline distribution lane.
  Publishes mirror feeds and offline bundles only.

The stable ids above are the ids other release artifacts reference.

## Signer-boundary ids

Signer boundaries are frozen as stable ids because they define where a
trust story changes from “I built bytes” to “I asserted bytes”.

The current signer-boundary inventory is:

- `none` — no signature or attestation is produced; only local bytes
  exist.
- `developer_local_signing_only` — developer or workstation signing (if
  used) is explicitly not authoritative for public/support-bearing
  artifacts.
- `hardened_signing_service` — isolated signing and attestation service.
  Release lanes submit digest sets; keys never reside on general build
  workers.
- `mirror_receipt_signer` — signer boundary for mirror continuity
  receipts and manual-import receipts. This boundary MUST NOT re-sign
  origin artifacts as if it were the origin.
- `third_party_publisher` — external publisher signer boundary (partner,
  community, or third-party registry). First-party verification consumes
  these signatures but does not treat them as release-lane authority.

## Publication-lane classes

The publication lanes used by this matrix are:

- `developer_preview` — developer workstation builds and untrusted PR
  previews.
- `internal_test` — internal validation and recurring lanes that are not
  support-bearing public truth.
- `public_stable` — support-bearing public channels (stable/LTS) and
  their coupled artifact graphs.
- `mirror_only` — mirror feeds and customer-managed mirror movement
  without advancing public channels.
- `emergency_offline` — emergency offline distribution and containment
  actions whose receipts are auditable, attributable, and bounded.

## Component trust-boundary matrix

The matrix below names the trust domains and signer boundaries for the
pipeline components reviewers need to reason about.

| Component | Trust domain id | Signer boundary id | What it may do | What it must not do |
|---|---|---|---|---|
| Developer workstation build | `untrusted_contributor` | `developer_local_signing_only` or `none` | local builds, local packaging, local tests | publish channels, write release caches, claim support-bearing provenance |
| CI runner (untrusted PR) | `untrusted_contributor` | `none` | build/test with no release credentials | write protected caches, sign, publish channels |
| CI runner (protected merge) | `protected_engineering` | `none` | write protected caches, emit internal evidence | publish channels, sign, hold stable/LTS creds |
| CI runner (release lanes) | `release_*` | `none` | build/package under lane credentials; request signing by digest | hold raw signing keys; publish outside its lane |
| Package builder (release lanes) | `release_*` | `none` | assemble installables, packs, and manifests under lane credentials; request signing by digest | hold raw signing keys; publish outside its lane; bypass artifact-graph validation |
| Remote caches | *varies by cache class* | `none` | accelerate builds via content-addressed entries | be authoritative for release truth |
| Signing & attestation service | *invoked by release lanes* | `hardened_signing_service` | produce signatures/attestations over digest sets | accept arbitrary bytes; expose raw private keys to workers |
| Release center (publication control plane) | `release_*` | *depends on action* | promote/rollback/revoke/yank/repin under audited actions | invent publish targets or trust vocabulary not present in contracts |
| Mirror / offline relays | `release_mirror_or_offline` | `mirror_receipt_signer` | mirror movement and offline bundle assembly with continuity receipts | re-sign origin artifacts as if origin; widen stale/revoked artifacts |
| Public download surfaces | *consumed, not trusted* | `none` | serve bytes and metadata | create provenance; act as signing authority |

The remote-cache row is intentionally “varies by cache class”: cache
admissibility is controlled by
[`/artifacts/release/cache_trust_classes.yaml`](../../artifacts/release/cache_trust_classes.yaml),
not by a single blanket “cache is trusted” claim.

## Artifact-publication rows (narrative rules)

Artifact-publication rows are the join point between:

- trust domains (`pipeline_lane_rules.yaml`);
- signer boundaries (this document and its companion artifact);
- artifact families (the artifact family map and artifact graph rules);
- publish targets (publish-target contract); and
- mirror/offline continuity and revocation propagation contracts.

The machine-readable row shape is frozen in
[`/schemas/release/artifact_publication_row.schema.json`](../../schemas/release/artifact_publication_row.schema.json).

### Rules for unsigned and developer-only artifacts

1. **Developer-only artifacts may exist, but they are not promotable.**
   A developer workstation build may be signed with a developer key, but
   that signature never becomes a first-party release signature.
2. **Unsigned artifacts must not enter support-bearing lanes.** Public
   stable/LTS publication rows require `hardened_signing_service` (or an
   explicitly recorded exception) for every runnable payload and for the
   coupled artifact graph nodes that describe it (SBOM, attestation,
   update metadata, docs/schema packs where applicable).
3. **Mirrors may not rewrite trust.** Mirror operators may sign mirror
   continuity receipts and offline verification packets, but they MUST
   NOT repackage and re-sign an artifact such that it could be confused
   for the origin.
4. **Promotion crosses trust domains only by explicit steps.** A build
   or cache entry produced under one trust domain is not promoted into
   another trust domain by name alone; it requires content-addressed
   re-verification and must be recorded as such in the destination lane.

### Seeded publication rows

The canonical seeded row set lives in
[`/artifacts/release/pipeline_trust_domains.yaml`](../../artifacts/release/pipeline_trust_domains.yaml).
Worked lane examples live under
[`/fixtures/release/publication_flow_examples/`](../../fixtures/release/publication_flow_examples/).
