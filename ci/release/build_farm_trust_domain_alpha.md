# Alpha build-farm trust domain and evidence plumbing baseline

This baseline makes alpha release evidence inspectable before production
publication automation exists. The source of truth is
`artifacts/release/alpha_artifact_graph.yaml`; this document explains the
trust boundaries and the collector at `ci/release/collect_alpha_evidence.sh`
turns that graph into a redaction-safe evidence packet.

The seed is deliberately narrow. It does not claim publishable package bytes,
live channel mutation, signing completion, or stable support. Rows whose bytes
are not yet produced by protected release infrastructure are labeled as
metadata seeds in the graph.

## Trust Domains

| Domain | Components | May Produce | Must Not Do |
|---|---|---|---|
| `untrusted_contributor` | developer workstations, untrusted PR runners | local build and test outputs | write release-bearing caches, hold release credentials, or publish channels |
| `protected_engineering` | protected merge CI and review validators | internal proof packets, support/export projections, schema checks | publish public channels or sign release artifacts |
| `release_preview` | preview release runner and package builder | preview candidate artifact graph rows, digest sets, docs/schema/support bundles, dry-run packets | use stable credentials or promote stale evidence as passing |
| `cleanroom_protected` | clean-room rebuild runner | independent rebuild proof, input manifests, reproducibility summaries | publish channels or depend only on warmed release caches |
| `release_mirror_or_offline` | mirror and offline bundle lanes | mirror receipts and offline continuity metadata | re-sign origin artifacts as if the mirror were the origin |
| `release_hotfix` | emergency revocation or hotfix lane | revocation, rollback, or emergency metadata with quorum refs | widen ordinary preview scope without normal release evidence |

These domain names come from `artifacts/release/pipeline_trust_domains.yaml`.
The alpha graph only narrows them to the preview seed so later clean-room
rehearsal can replay the boundaries without inventing a second vocabulary.

## Boundary Rules

- Source materialization uses pinned commit/tree evidence, lock/toolchain
  digests, and reviewed feature inputs from graph refs.
- Release-bearing caches do not accept writes from untrusted fork or local
  preview lanes.
- Build workers do not hold raw signing keys. Signing and attestation stay in
  the hardened signing boundary once real package bytes exist.
- Docs packs, schemas, support packets, update metadata, provenance inputs, and
  rollback metadata are part of the release graph, not follow-up notes.
- Rollback targets name the coordinated artifact family, not a single package.
- Missing or stale evidence stays visible as `stale_blocking`,
  `seed_not_signed`, or `metadata_seed_pending_real_package_bytes`; consumers
  must not convert those states into a green release claim.

## Evidence Collection

Run the collector from the repository root:

```sh
ci/release/collect_alpha_evidence.sh --repo-root .
```

The collector reads `artifacts/release/alpha_artifact_graph.yaml`, resolves the
candidate, publish target, bundle, artifact nodes, trust-domain refs, and
source paths, then writes `target/release-evidence/alpha_evidence_packet.json`.

For a deterministic review artifact:

```sh
ci/release/collect_alpha_evidence.sh \
  --repo-root . \
  --generated-at 2026-05-14T07:30:00Z \
  --out /tmp/aureline-alpha-evidence.json
```

The packet includes:

- candidate version;
- publish-target class;
- computed SHA-256 digest set for the graph source refs;
- rollout ring;
- auth-source class;
- rollback target;
- exact-build identity ref;
- trust-domain refs; and
- missing or stale evidence states carried by the graph.

## Clean-Room Rehearsal Inputs

The clean-room lane should consume the same graph fields:

- `build_roots[]` for source materialization and cache posture;
- `provenance_inputs[]` for source, toolchain, protected fitness, and advisory
  support evidence;
- `artifact_families.*[]` for artifact node coverage;
- `release_center_objects.*_descriptors[]` for candidate, target, timeline,
  rollback, and bundle reconstruction.

If a clean-room run cannot resolve any source ref from the graph, it must fail
closed and report the missing ref rather than substitute a local directory or
version label.
