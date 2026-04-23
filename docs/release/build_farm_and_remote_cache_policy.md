# Build farm trust-domain, remote-cache non-dependence, and release-lane provenance policy

This document freezes Aureline's build-farm trust-domain rules,
remote-cache non-dependence rules, and release-lane provenance policy.
It exists so fast CI paths cannot silently become hidden authorities for
release evidence or reproducibility, and so every promoted release
remains explainable from clean-room rebuild inputs and exact-build
identities even when remote-cache fast paths are bypassed,
invalidated, quarantined, or simply unavailable.

The narrative rules live here; the machine-readable companions live in
[`/artifacts/release/pipeline_lane_rules.yaml`](../../artifacts/release/pipeline_lane_rules.yaml)
and
[`/artifacts/release/cache_trust_classes.yaml`](../../artifacts/release/cache_trust_classes.yaml).

This policy does not stand up a build farm. It governs how a build farm
behaves whenever one is brought online so that later release-engineering
work composes on top of these rules instead of inventing a parallel
trust story per pipeline.

Companion artifacts:

- [`/artifacts/release/pipeline_lane_rules.yaml`](../../artifacts/release/pipeline_lane_rules.yaml)
  — machine-readable lane-by-lane rules covering allowed cache posture,
  credential boundary, publishing rights, branch/channel mapping,
  emergency hotfix posture, and re-materialized input requirements.
- [`/artifacts/release/cache_trust_classes.yaml`](../../artifacts/release/cache_trust_classes.yaml)
  — machine-readable cache-class taxonomy covering verified vs.
  untrusted classes, write/read policy, content-addressed storage and
  OCI-compatible mirror posture, cache-poisoning release blockers, and
  cache-comparability-loss handling.
- [`/docs/adr/0017-release-posture-artifact-families-and-promotion-gates.md`](../adr/0017-release-posture-artifact-families-and-promotion-gates.md)
  — governing release-posture ADR for channels, rollback atom,
  same-change-set release bundles, waiver/late-proof policy, and
  stable-facing promotion vetoes that this policy plugs into.
- [`/docs/release/release_artifact_graph.md`](./release_artifact_graph.md)
  and
  [`/artifacts/release/artifact_graph_rules.yaml`](../../artifacts/release/artifact_graph_rules.yaml)
  — release-artifact graph completeness rules. Caches never become a
  node family; they only accelerate the materialization of node-bearing
  inputs.
- [`/artifacts/release/artifact_family_map.yaml`](../../artifacts/release/artifact_family_map.yaml)
  and
  [`/artifacts/release/promotion_gate_map.yaml`](../../artifacts/release/promotion_gate_map.yaml)
  — artifact-family postures and promotion-gate maps that decide which
  lane outputs are publishable and which fail closed.
- [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  and
  [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  — exact-build identity model that every release-bearing lane resolves
  through. Cache content is identified by the digest of the inputs that
  produced it, not by the cache key alone.
- [`/docs/build/cleanroom_rebuild_lane.md`](../build/cleanroom_rebuild_lane.md)
  and
  [`/ci/cleanroom_rebuild.sh`](../../ci/cleanroom_rebuild.sh)
  — clean-room rebuild lane that this policy treats as the protected
  reproducibility floor. Release proof falls back to clean-room rebuild
  when remote-cache fast paths are bypassed or invalidated.
- [`/docs/build/reproducible_build_baseline.md`](../build/reproducible_build_baseline.md)
  — pinned toolchain, lockfiles, and build-identity contract every lane
  composes with.
- [`/docs/governance/provenance_and_compliance_baseline.md`](../governance/provenance_and_compliance_baseline.md)
  — provenance and compliance baseline that this policy extends with
  cache-class and lane-class rules.
- [`/docs/release/install_topology_plan.md`](./install_topology_plan.md)
  and
  [`/artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml)
  — install topology and channel mapping consumed by the lane/channel
  binding rules below.
- [`/docs/security/severity_matrix.md`](../security/severity_matrix.md)
  and
  [`/docs/security/emergency_action_model.md`](../security/emergency_action_model.md)
  — advisory, revocation, and emergency-action object model that
  hotfix-lane and cache-poisoning rules cite.

Normative sources this policy projects from:

- `.t2/docs/Aureline_PRD.md` §16 release governance, §10.18 supply chain
  and offline distribution rules, and the protected-channel evidence
  bars in §7.32 (clean-room rebuild, exact-build supportability,
  remote-cache non-dependence, mirror/offline parity, user-visible
  provenance).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §25.8 "Build
  farm, CI/CD, and provenance architecture" and Appendix AO
  "Build, CI/CD, Provenance, and Artifact-Publication Matrix".
- `.t2/docs/Aureline_Technical_Design_Document.md` §11.2.2 "Build farm,
  remote cache, and clean-room rebuild rules", §11.2.9 "Release-center
  publication, provenance, and rollback rules", and Appendix N
  "Release artifact graph and promotion evidence".
- `.t2/docs/Aureline_Milestones_Document.md` §3.36 build reproducibility
  and remote-cache discipline governance and §7.32 reproducible build,
  exact-build symbolication, and publication-parity bars.

If this document disagrees with those sources, those sources win and
this document plus the machine-readable companions update in the same
change.

## Why publish this now

The repository already froze:

- the exact-build identity model and the artifact-family map,
- the release-artifact graph and bundle-completeness rules,
- the clean-room rebuild lane, its required inputs, and its capture
  outputs,
- the release-posture ADR, the promotion-gate map, and the shiproom
  review order, and
- the emergency-action and revocation object model.

What it had not yet frozen was a single rule for how engineering CI,
mainline merges, nightly/preview/stable/LTS releases, and emergency
hotfixes interact with build farms and remote caches. Without that
rule:

- a contributor PR pipeline could quietly hand its cached outputs to a
  release lane;
- a release lane could declare success because a remote-cache hit
  shortened the build, even though the inputs that produced the cached
  bytes were no longer reachable, signed, or reviewable;
- a cache-poisoning event could invalidate a fast path and silently
  block release proof, or worse, ship the poisoned artifact under the
  same channel manifest;
- a mirror or offline promotion lane could depend on opaque cache state
  that an air-gapped reviewer could not reproduce; and
- an emergency hotfix could borrow trust from a warm cache instead of
  re-materializing inputs under hardened controls.

This policy closes that gap. It does not require building a build farm
to be enforceable: it states what such a farm must do whenever it is
brought online, and it states what protected-channel proof must look
like in the meantime.

## Invariants

The following invariants apply to every lane in this policy:

- **Caches accelerate; caches do not authorize.** No remote cache,
  shared mirror, or fast-path layer is admissible as the authoritative
  source of release evidence. Authority lives in the inputs (commit,
  toolchain image, lockfiles, environment manifest), the exact-build
  identity, and the clean-room rebuild lane. A cache hit may shorten a
  build; it never replaces the proof that the build is reproducible.
- **Trust domains are explicit.** Every lane declares its trust domain,
  its credential boundary, its allowed cache classes, and its
  publishing rights. Cross-domain promotion is blocked unless the
  receiving domain re-materializes the inputs under its own controls.
- **Release proof survives cache bypass.** If a release lane is forced
  to ignore every remote cache it normally consumes, the lane MUST
  still be able to produce the same exact-build identity set and pass
  the protected promotion gates. If it cannot, the cache had become a
  hidden authority and the lane fails closed.
- **Cache content is content-addressed, not name-addressed.** Cached
  outputs carry the digest of the inputs that produced them. Cache
  keys are an indexing convenience; they are not an identity. A cache
  that cannot produce the input digest for a hit is treated as
  untrusted by definition.
- **Cross-lane writes are forbidden by default.** A lane may only write
  to caches inside its own trust domain. Cross-domain promotion of
  cached bytes requires re-verification against the receiving domain's
  rules, recorded in the lane's evidence output.
- **Mirror and offline lanes inherit, not invent.** A mirror, air-gap,
  or offline-promotion lane may not depend on opaque cache state. It
  must materialize its inputs from content-addressed sources or from a
  signed, reviewable mirror manifest that itself resolves through the
  same exact-build identity model.
- **Cache-comparability loss is a release blocker on protected lanes.**
  If a release-bearing cache cannot prove that two consumers see the
  same content (mirror desynchronization, integrity-check failure,
  signer-continuity break, untracked pruning), the lane fails closed
  on RC/stable/LTS/hotfix candidates and falls back to the clean-room
  rebuild path.

## Lane vocabulary

The closed set of pipeline lanes governed by this policy is:

- `contributor_pr_lane` — pull requests opened from any source,
  including untrusted forks.
- `protected_merge_lane` — merges into protected branches that feed
  the nightly/preview promotion graph.
- `nightly_release_lane` — recurring nightly publication on the
  `nightly` channel.
- `preview_release_lane` — `preview` channel publication.
- `beta_release_lane` — `beta` channel publication.
- `stable_release_lane` — `stable` channel publication, including
  `rc_or_stable_candidate` review state.
- `lts_release_lane` — `lts` channel publication and backport
  promotion.
- `emergency_hotfix_lane` — `hotfix` channel publication for
  correction releases against an existing stable or LTS line.
- `cleanroom_rebuild_lane` — protected reproducibility lane that
  composes from
  [`/ci/cleanroom_rebuild.sh`](../../ci/cleanroom_rebuild.sh).
- `mirror_or_offline_promotion_lane` — mirror, air-gap, and
  manual-import promotion paths covered by the emergency-transport
  flow in
  [`/artifacts/release/promotion_gate_map.yaml`](../../artifacts/release/promotion_gate_map.yaml).

`rc_candidate` is a review state over an existing channel-bound build
set per ADR-0017. It does not appear as a separate lane id here; an RC
review uses the same lane that produced the underlying build.

## Trust-domain and lane-class rules

Each lane declares one trust domain, one credential boundary, one
allowed cache posture, one publishing-rights class, and one
branch/channel mapping. The narrative rules per lane follow; the
machine-readable rows live in
[`/artifacts/release/pipeline_lane_rules.yaml`](../../artifacts/release/pipeline_lane_rules.yaml).

### Contributor / PR lane

- **Trust domain:** `untrusted_contributor`. Forks are presumed
  untrusted regardless of contributor reputation.
- **Credential boundary:** no signing keys, no release-bearing
  registry credentials, no protected-cache write tokens. Read access
  is limited to the `pr_local_cache` class.
- **Allowed cache posture:** read from `verified_baseline_cache` for
  reproducible toolchain seeding only; write only to `pr_local_cache`.
  Outputs of this lane MUST NOT be promoted into
  `protected_merge_cache` or `release_lane_cache`.
- **Publishing rights:** none. The lane may produce ephemeral
  preview-only artifacts inside the PR, but no channel manifest, no
  registry mutation, and no advisory-bearing publication.
- **Branch/channel mapping:** none. The lane is bound to the PR ref;
  no channel id is admissible.
- **Re-materialized input requirement:** not applicable; this lane is
  not release-bearing.
- **Failure posture:** failure of this lane blocks PR merge through
  the merge-block path declared by
  [`/artifacts/release/qualification_schedule.yaml`](../../artifacts/release/qualification_schedule.yaml)
  but never blocks an unrelated release lane.

### Protected merge lane

- **Trust domain:** `protected_engineering`. Only commits that have
  cleared the contributor lane's protected-merge gate enter here.
- **Credential boundary:** mainline CI credentials. No release signing
  keys, no LTS-only credentials, and no emergency-hotfix credentials.
- **Allowed cache posture:** read/write `protected_merge_cache`; read
  `verified_baseline_cache`. May NOT consume `pr_local_cache`. Cached
  outputs from this lane may seed nightly and preview release lanes
  only after content-addressed re-verification.
- **Publishing rights:** internal artifacts only. The lane may publish
  internal evidence packets that are not channel-bound.
- **Branch/channel mapping:** protected merge refs only; no channel id
  is admissible at this stage.
- **Re-materialized input requirement:** every protected-merge build
  records its input digest set so downstream lanes can re-materialize
  inputs without relying on the cache.
- **Failure posture:** failure invalidates the cache row and opens
  regression triage per
  [`/artifacts/governance/evidence_rerun_triggers.yaml`](../../artifacts/governance/evidence_rerun_triggers.yaml);
  it does not silently skip downstream lanes.

### Nightly release lane

- **Trust domain:** `release_nightly`.
- **Credential boundary:** nightly publishing credentials only. No
  stable signing keys, no LTS credentials, no hotfix credentials.
- **Allowed cache posture:** read `verified_baseline_cache`,
  `protected_merge_cache`, and `release_lane_cache`; write only
  `release_lane_cache`. Cached inputs from `protected_merge_cache`
  MUST be content-addressed and re-verified against the recorded input
  digest before consumption.
- **Publishing rights:** `nightly` channel only. May produce a
  publishable nightly release packet per
  [`/docs/release/release_evidence_packet_template.md`](./release_evidence_packet_template.md).
- **Branch/channel mapping:** `nightly`.
- **Re-materialized input requirement:** the lane MUST record its
  input manifest, exact-build identity set, and a periodic clean-room
  rebuild reference (cadence governed by
  [`/artifacts/release/qualification_schedule.yaml`](../../artifacts/release/qualification_schedule.yaml))
  so the channel can be reconstructed without cache hits.
- **Failure posture:** stale or missing required proof holds the
  channel for refresh per
  [`/artifacts/release/promotion_gate_map.yaml`](../../artifacts/release/promotion_gate_map.yaml)
  `stale_evidence_policy.preview_candidate`.

### Preview release lane

- Same trust-domain, credential, and cache posture rules as the
  nightly lane, narrowed to `release_preview` and the `preview`
  channel.
- **Re-materialized input requirement:** every preview promotion MUST
  cite a current clean-room rebuild reference for the coordinated
  release family per ADR-0017 and the
  `gate.candidate_envelope` / `gate.docs_command_route_and_reproducible_release`
  rows of the promotion gate map. A preview promotion that depends on
  a cache hit without a re-materializable input set fails closed.

### Beta release lane

- **Trust domain:** `release_beta`.
- **Credential boundary:** beta publishing credentials. No stable or
  LTS signing keys.
- **Allowed cache posture:** as preview, narrowed by the additional
  beta-stage gate refs in the promotion-gate map.
- **Re-materialized input requirement:** beta promotion requires the
  clean-room rebuild lane to have produced a current proof for the
  coordinated release family. Cached toolchain images, dependency
  artifacts, and intermediate build outputs are admissible only when
  they resolve through content-addressed inputs whose digests appear
  in the lane's recorded input manifest.

### Stable release lane

- **Trust domain:** `release_stable`. The strictest credential
  boundary; signing happens in a hardened signing domain per Appendix
  AO and the provenance baseline.
- **Allowed cache posture:** read `verified_baseline_cache` and
  `release_lane_cache`; MUST NOT read from `protected_merge_cache`
  unless the cached entry is re-verified by content-addressed digest
  AND the entry has a recorded provenance row. MUST NOT consume
  `pr_local_cache` under any condition.
- **Publishing rights:** `stable` channel and `rc_or_stable_candidate`
  review-state movement only.
- **Branch/channel mapping:** `stable`.
- **Re-materialized input requirement:** stable promotion requires a
  current clean-room rebuild proof that does not depend on any
  release-lane or protected-merge cache. The clean-room lane
  re-materializes inputs from pinned commits, lockfiles, and reviewed
  toolchain references; cache hits during the clean-room run are
  acceptable only when the cache content is content-addressed and the
  recovered bytes match the input digest.
- **Failure posture:** any stale or missing required proof produces
  `no_go` per the stable-stage rule of
  [`/artifacts/release/promotion_gate_map.yaml`](../../artifacts/release/promotion_gate_map.yaml).

### LTS release lane

- Same trust-domain, credential, and cache-posture rules as the
  stable lane, narrowed to `release_lts` and the `lts` channel.
- **Re-materialized input requirement:** LTS promotion additionally
  requires that the backport scope and supportability evidence resolve
  through the clean-room rebuild lane for both the LTS branch and any
  shared upstream ancestor cited in the release packet.
- **Cache freshness:** LTS lanes MUST NOT depend on caches whose
  retention is shorter than the LTS support window. If a cache class
  cannot cover the support window, the lane re-materializes inputs
  from the source bundle or reproducibility pack.

### Emergency hotfix lane

- **Trust domain:** `release_hotfix`. Inherits the stable-lane
  hardening and adds extra human approvals per the signing-quorum
  policy.
- **Credential boundary:** hotfix-only credentials. No reuse of
  general engineering credentials. Break-glass actions are recorded as
  emergency-action records per
  [`/docs/security/emergency_action_model.md`](../security/emergency_action_model.md).
- **Allowed cache posture:** minimized dependency surface. The lane
  MUST re-materialize all inputs from pinned content-addressed
  sources. Cached intermediate outputs are admissible only from
  `release_lane_cache` and only after content-addressed re-
  verification AND a fresh signing-quorum step.
- **Publishing rights:** `hotfix` channel only, with named rollback
  target and current response packet.
- **Re-materialized input requirement:** mandatory. Every hotfix
  carries a clean-room rebuild proof and a re-materialized input
  manifest in the same release-evidence packet refresh.
- **Failure posture:** any cache poisoning, signer-continuity break,
  or cache-comparability loss on a hotfix path triggers
  `gate.candidate_envelope` and
  `gate.public_interface_provenance_and_durability` blocks
  immediately; the response routes to the
  `mirror_only_response` flow if hosted publication itself is
  compromised.

### Clean-room rebuild lane

- **Trust domain:** `cleanroom_protected`. The lane is intentionally
  the simplest trust domain in the system: it accepts only pinned
  commits and pinned lockfiles, runs from
  [`/ci/cleanroom_rebuild.sh`](../../ci/cleanroom_rebuild.sh), and
  emits the manifests and provenance summary documented in
  [`/docs/build/cleanroom_rebuild_lane.md`](../build/cleanroom_rebuild_lane.md).
- **Credential boundary:** none beyond read access to the pinned
  inputs and the runner's ephemeral state. The lane never holds
  release signing keys.
- **Allowed cache posture:** `verified_baseline_cache` only, used
  exclusively for toolchain mirroring. The clean-room lane does not
  consume `protected_merge_cache`, `release_lane_cache`,
  `pr_local_cache`, or any opaque CI-built cache.
- **Publishing rights:** internal control artifacts only (build
  identity, input manifest, digest manifest, provenance capture).
  Outputs feed the release-evidence packet by stable ref.
- **Re-materialized input requirement:** definitionally satisfied:
  this lane is the re-materialization reference.

### Mirror or offline promotion lane

- **Trust domain:** `release_mirror_or_offline`. Bound to the
  emergency-transport flow declared in
  [`/artifacts/release/promotion_gate_map.yaml`](../../artifacts/release/promotion_gate_map.yaml)
  and the manual-import / mirror-import receipt rules in the
  emergency-action model.
- **Credential boundary:** mirror-publication credentials only. The
  mirror lane never holds upstream stable signing keys; it republishes
  signed content under the mirror manifest's signer continuity model.
- **Allowed cache posture:** the lane MUST NOT depend on opaque cache
  state. Mirror inputs are content-addressed against the upstream
  release packet and verified by signer continuity per
  [`/docs/security/emergency_action_model.md`](../security/emergency_action_model.md).
  Local mirror caches are admissible only as `verified_mirror_cache`
  with a published mirror manifest.
- **Publishing rights:** mirror or air-gap publication paths only;
  the lane never invents a new channel id.
- **Re-materialized input requirement:** mandatory. The mirror lane
  MUST be able to demonstrate that an air-gapped reviewer can
  reconstruct the published bytes from the mirror manifest, the
  upstream exact-build identity set, and the signed reproducibility
  pack. If reconstruction depends on opaque mirror-local cache state,
  the mirror is not authoritative and the lane fails closed.

## Cache trust classes

The closed set of cache classes used by the lanes above is:

- `verified_baseline_cache` — pinned toolchain bytes mirrored under a
  reviewed signing root. Reproducible by digest from upstream
  publication. Admissible on every lane.
- `protected_merge_cache` — outputs produced by the protected merge
  lane. Content-addressed by input digest. Admissible only inside the
  protected engineering trust domain; release lanes consume it only
  after content-addressed re-verification.
- `release_lane_cache` — outputs produced by a specific release lane.
  Bound to that release lane's trust domain. Cross-lane consumption
  requires re-verification.
- `pr_local_cache` — ephemeral, untrusted outputs scoped to a single
  PR run. Never admissible outside the contributor lane.
- `verified_mirror_cache` — mirror-side cache that resolves through a
  signed mirror manifest. Admissible on the mirror or offline
  promotion lane only.
- `developer_local_cache` — developer machine cache, declared for
  completeness. Never admissible on a release lane and never recorded
  inside a release-evidence packet beyond a cache-identity ref in a
  support bundle.

The cache class taxonomy, write/read policy, content-addressed-storage
rules, OCI-compatible mirror posture, cache-poisoning release blockers,
and cache-comparability-loss handling are frozen in
[`/artifacts/release/cache_trust_classes.yaml`](../../artifacts/release/cache_trust_classes.yaml).

## Remote-cache non-dependence rules

The following rules apply to every release-bearing lane regardless of
the cache class involved.

1. **Cache hits are never inputs to the exact-build identity.** The
   identity is computed from the source commit, lockfiles, toolchain
   image identity, target tuple, and other governed axes per
   [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md).
   Whether a particular intermediate output came from a cache hit or
   was rebuilt locally is a build-log detail, not an identity field.
2. **Cache hits never satisfy promotion gates.** A
   `gate.candidate_envelope`, `gate.public_interface_provenance_and_durability`,
   or `gate.docs_command_route_and_reproducible_release` row is
   satisfied by reproducible inputs and current clean-room rebuild
   proof, not by a cache hit count.
3. **Cache hits never substitute for a reviewable input manifest.**
   Every release-bearing lane records its input manifest and
   digest manifest. The manifest must be reconstructable when every
   cache is bypassed.
4. **Bypassing every remote cache MUST NOT change the published
   identity.** A scheduled cache-bypass rehearsal is part of the
   rehearsal calendar in
   [`/artifacts/release/qualification_schedule.yaml`](../../artifacts/release/qualification_schedule.yaml)
   for stable-facing lanes. A drift between cached and bypassed
   builds is a release blocker, not a curiosity.
5. **Cache-comparability loss falls back to the clean-room lane.** If
   two consumers of the same cache class cannot prove they see the
   same bytes (mirror desync, integrity check failure, signer
   continuity break, untracked pruning), release lanes treat the
   cache as untrusted, mark the affected entries as
   `cache_comparability_lost`, and fall back to the clean-room
   rebuild path before any further channel movement.
6. **Cache poisoning is a release blocker.** Confirmed or suspected
   poisoning of any release-bearing cache class moves the affected
   release packet to `blocked` per the release-evidence packet
   template, opens an `emergency_action_record` per the
   emergency-action model, and triggers the
   `signed_update_metadata_or_signing_root_compromised` escalation in
   `advisory_and_revocation_scope_policy` of
   [`/artifacts/release/promotion_gate_map.yaml`](../../artifacts/release/promotion_gate_map.yaml)
   when signing roots are implicated.
7. **No silent cross-domain cache promotion.** A cache entry produced
   under one trust domain may not be promoted into another trust
   domain without re-verification. Re-verification means computing the
   input digest and comparing it to the recorded input digest of the
   destination lane; a name match is insufficient.
8. **Mirror lanes do not depend on opaque cache state.** Mirror or
   offline promotion lanes consume bytes from a signed mirror
   manifest that itself resolves through the upstream exact-build
   identity. A mirror that cannot be reconstructed from its manifest
   plus the clean-room rebuild lane is not authoritative.

## Verified vs. untrusted cache classes

| Cache class | Verified posture | Authoritative for release evidence? | Notes |
|---|---|---|---|
| `verified_baseline_cache` | verified by upstream signing root and content-addressed mirror manifest | no — accelerates only | mirrors the pinned toolchain inputs declared by the reproducible build baseline |
| `protected_merge_cache` | content-addressed by input digest; written under protected engineering credentials | no — internal acceleration only | release lanes re-verify against the recorded input digest before consumption |
| `release_lane_cache` | content-addressed; written under the lane's release credentials | no — accelerates publishing within the lane | other release lanes re-verify before reuse; never authoritative across lanes |
| `pr_local_cache` | untrusted by definition | never | scoped to one PR run; cannot leak into protected lanes |
| `verified_mirror_cache` | content-addressed by upstream digest plus signed mirror manifest | no — accelerates only | mirror or offline promotion lanes consume by digest, not by name |
| `developer_local_cache` | local to one developer machine | never | only recorded by stable cache-identity ref inside support bundles |

In every row, the canonical authority remains the inputs (commit,
lockfiles, toolchain image identity), the exact-build identity, and
the clean-room rebuild proof. The cache class column says only how
quickly the lane may reach those authorities.

## Re-materialized input requirements per release lane

A release lane is "re-materializable" when it can produce the same
exact-build identity set and the same artifact digest manifest from
the recorded inputs without relying on any single cache layer. The
required minimum proof per release stage is:

| Release stage | Required re-materialization proof |
|---|---|
| `nightly` | recorded input manifest plus a periodic clean-room rebuild reference; cache bypass rehearsal cadence per `qualification_schedule.yaml` |
| `preview` | nightly proof plus a clean-room rebuild reference current for the coordinated release family at promotion time |
| `beta` | preview proof plus a clean-room rebuild proof tied to the same artifact-family map row that the candidate ships |
| `stable` (RC or stable) | beta proof plus a clean-room rebuild proof produced under the protected reproducibility lane and bound to the same exact-build identity set the candidate publishes |
| `lts` | stable proof plus mirror/offline parity proof and a re-materialization plan that survives the LTS support window |
| `hotfix` | mandatory clean-room rebuild proof and re-materialized input manifest in the same packet refresh; minimized cache surface |

These rows compose with the promotion-gate map: the gate map decides
which release postures are required at each stage; this policy decides
which inputs and which proof of re-materialization are required to
satisfy those gates.

## Linkage with content-addressed artifacts and OCI-compatible mirrors

Aureline distinguishes three layers and refuses to collapse them:

1. **Content-addressed artifacts.** Release-bearing artifacts (binaries,
   debug sidecars, docs packs, schemas, SBOMs, attestations, source
   bundles, reproducibility packs) are addressed by digest, never by
   mutable name alone. Channel manifests carry the digest set, and
   downstream consumers verify by digest before use. The exact-build
   identity model is the canonical source for which digests belong to
   which artifact family.
2. **OCI-compatible mirrors where appropriate.** Mirror and air-gap
   distribution may use OCI-compatible registries to host the same
   content-addressed artifacts. An OCI mirror is a transport optimization
   for content-addressed bytes; it is not a separate authority. A mirror
   that loses signer continuity, fails integrity checks, or cannot
   resolve a published digest is treated as untrusted under the
   `verified_mirror_cache` rules above and the emergency-action model.
3. **Clean-room rebuild inputs.** The clean-room lane re-materializes
   the inputs (pinned commit, lockfiles, toolchain image references,
   environment manifest) and emits the artifact digest manifest, the
   build identity, and the provenance capture summary. The clean-room
   lane's outputs are how a release packet proves that the
   content-addressed artifacts could be produced from reviewable
   inputs without a particular cache layer being present.

The relationship is: **content-addressed artifacts** are the publishable
nouns; **OCI-compatible mirrors** are one allowed transport for those
nouns; **clean-room rebuild inputs** are the proof that those nouns
exist because a reviewer could rebuild them, not because a cache
remembered them. None of the three replaces the other, and none makes
remote caches authoritative.

## Release-lane provenance policy

Every release-bearing lane emits a provenance capture composed of:

- the recorded input manifest (commit, lockfiles, toolchain image
  references, environment manifest, mirror endpoints used);
- the exact-build identity set produced by the lane;
- the artifact digest manifest;
- the cache classes consumed and, for each consumed entry, the
  recorded input digest used to verify the entry against the lane's
  trust domain;
- the clean-room rebuild reference used to satisfy the
  re-materialization requirement for the lane's release stage;
- any cache-poisoning, cache-comparability-loss, or signer-continuity
  events the lane encountered, with refs to the corresponding
  emergency-action records;
- the signing-quorum action ids used per
  [`/artifacts/governance/signing_quorum.yaml`](../../artifacts/governance/signing_quorum.yaml);
  and
- the lane's publishing-rights class and channel binding.

The capture composes with the
[`provenance_capture_seed.json`](../../artifacts/release/provenance_capture_seed.json)
shape used by the clean-room lane today; release-grade captures extend
that shape rather than minting a parallel record.

## What happens when cache comparability is lost

When a release-bearing cache class cannot prove byte-equivalence
between its consumers, or when a signer-continuity break or integrity
check failure invalidates a cache row, lanes proceed as follows:

1. The affected release packet moves to `blocked` per
   [`/docs/release/release_evidence_packet_template.md`](./release_evidence_packet_template.md).
2. The consuming lane drops the affected cache class from its allowed
   posture for the affected entry and marks it `cache_comparability_lost`
   in its provenance capture.
3. The lane falls back to the clean-room rebuild path. If the
   clean-room lane reproduces the same exact-build identity set, the
   release packet may resume with the cache class explicitly excluded
   for the affected entries.
4. If the failure implicates signed update metadata or any signing
   root, the
   `signed_update_metadata_or_signing_root_compromised` escalation in
   `advisory_and_revocation_scope_policy` widens the response to
   `channel_manifest_or_signing_root` and the affected channel pauses
   per the emergency-action model.
5. Mirror or offline promotion lanes that discover comparability loss
   in `verified_mirror_cache` route the response to
   `mirror_only_response` per the emergency-transport flow in
   [`/artifacts/release/promotion_gate_map.yaml`](../../artifacts/release/promotion_gate_map.yaml).

## Reuse surfaces

This policy is intentionally reusable by the following downstream
lanes without restating its rules:

- **Clean-room rebuild work.** Lanes that extend the clean-room lane
  cite this policy for their cache posture and re-materialization
  requirements.
- **Release-artifact work.** Lanes that publish release artifacts cite
  this policy for trust-domain, credential boundary, and publishing
  rights, and they cite the artifact-graph rules for
  bundle-completeness.
- **Channel and versioning work.** Channel-management work cites this
  policy for the lane/channel binding rules and the cache-bypass
  rehearsal cadence.
- **Emergency-action work.** Hotfix, advisory, revocation, and
  emergency-disable lanes cite this policy for the hotfix-lane and
  cache-poisoning rules and route through the emergency-action model
  for the durable record.
- **Mirror and offline promotion work.** Mirror, air-gap, and
  manual-import lanes cite this policy for the
  `mirror_or_offline_promotion_lane` rules and the mirror-cache trust
  class.

## Current repository posture

This repository is still pre-implementation and does not yet operate a
build farm with multiple trust domains, multiple release credentials,
or a hosted remote cache. The current posture is:

- the clean-room rebuild lane is the protected reproducibility floor;
- CI runs the contributor and protected-merge equivalents through the
  GitHub Actions workflows in
  [`/.github/workflows/`](../../.github/workflows/) without yet
  carrying release credentials;
- the release-evidence packet template, promotion-gate map, and
  artifact-family map declare the gates this policy fills; and
- the machine-readable companions in
  [`/artifacts/release/pipeline_lane_rules.yaml`](../../artifacts/release/pipeline_lane_rules.yaml)
  and
  [`/artifacts/release/cache_trust_classes.yaml`](../../artifacts/release/cache_trust_classes.yaml)
  are seeded so later release-engineering work extends rows rather
  than minting parallel rules.

That is acceptable at this stage as long as the policy stays honest:
no current lane claims more trust than the clean-room rebuild lane can
back, and no current lane treats a cache hit as authoritative for
release evidence.
