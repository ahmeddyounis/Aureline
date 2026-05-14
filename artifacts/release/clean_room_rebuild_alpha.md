<!-- SPDX-License-Identifier: Apache-2.0 -->

# Alpha Clean-Room Rebuild Dry Run

This packet records the alpha clean-room rebuild rehearsal that the
publication manifest consumes. It is scoped to exact-build evidence and
artifact-family comparability, not to public release publication.

Source of truth:
[`artifacts/release/alpha_publication_manifest.yaml`](alpha_publication_manifest.yaml).

## Rebuild Entry Points

| Purpose | Ref | Dry-run posture |
| --- | --- | --- |
| Clean-room rebuild command | `ci/cleanroom_rebuild.sh --out-dir target/cleanroom-rebuild` | Requires a clean checkout and emits build identity, digest, SBOM stub, provenance summary, input manifest, and capture summary. |
| Offline rebuild variant | `ci/cleanroom_rebuild.sh --offline --out-dir target/cleanroom-rebuild` | Exercises pinned inputs without public network fallback where local caches can satisfy the build. |
| Evidence collector | `ci/release/collect_alpha_evidence.sh --repo-root . --validate-only` | Validates the alpha artifact graph and reconstructs candidate, target, digest set, rollout, auth source, and rollback target. |
| Publication dry-run validator | `python3 ci/release/check_alpha_publication_dry_run.py --repo-root . --validate-only` | Validates the one-manifest publication packet without mutating any channel or mirror state. |

The existing clean-room lane already emits `provenance_capture.json` shaped by
[`artifacts/release/provenance_capture_seed.json`](provenance_capture_seed.json).
This alpha packet reuses that contract instead of defining a second rebuild
format.

## Comparable Family Result

| Family | Comparison basis | Result | Known differences |
| --- | --- | --- | --- |
| Binaries | Alpha artifact graph nodes plus checked-in digest material | Comparable with blockers | Real signed package bytes are absent; shell binary evidence is seeded from support fixtures and command source. |
| Docs/help packs | Checked-in docs pack source refs and digest material | Comparable | Mirrored official docs pack bytes are reserved but not admitted. |
| Policy bundles | Policy pack schema plus emergency-disable fixture | Comparable outside graph | Policy pack is mirror/offline-ready but not yet a node in the alpha artifact graph. |
| Symbols | Exact symbolication fixture and crash linkage packet | Comparable with blockers | Real debug sidecar archive bytes are absent. |
| Support schemas | Support bundle schema, support packet index, and support linkage packets | Comparable | Hosted support intake is outside this dry run. |
| Notices | Notice delta packet, release notice seed, and import manifest | Comparable with blockers | Reserved imports have pending notice text until real bytes are imported. |
| SBOM/provenance | Provenance capture seed and SBOM/provenance stub script | Comparable as placeholder only | The SBOM lane does not claim SPDX or CycloneDX conformance and attestation signing is absent. |

## Acceptance Evidence

The dry run is acceptable only as a controlled alpha rehearsal when these
statements remain true:

- The rebuild result is tied to the same exact-build identity as
  `artifacts/release/alpha_artifact_graph.yaml`.
- Known differences are explicit in the manifest blockers and are not hidden
  behind a green publication state.
- The mirror/offline rehearsal can validate the same artifact family set from
  the manifest without live vendor reachability.
- Live service health, advisory freshness, and revocation freshness degrade to
  declared snapshot states under mirror/offline postures.

## Current Blockers

| Blocker | Impact |
| --- | --- |
| `blocker.publish.package_bytes_missing` | Blocks broader alpha publication because signed package artifacts are not present. |
| `blocker.fitness.evidence_stale` | Blocks broader publication until protected fitness evidence is refreshed. |
| `blocker.policy.not_in_alpha_artifact_graph` | Keeps policy bundle coverage explicit but non-graph-backed. |
| `blocker.sbom.placeholder_only` | Prevents any standards-conformant SBOM claim. |
| `blocker.attestation.signing_absent` | Prevents a signed attestation or final provenance claim. |

## Verification

```sh
python3 ci/release/check_alpha_publication_dry_run.py --repo-root . --validate-only
ci/release/collect_alpha_evidence.sh --repo-root . --validate-only
```
