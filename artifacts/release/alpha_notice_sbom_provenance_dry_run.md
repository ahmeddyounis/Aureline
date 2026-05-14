<!-- SPDX-License-Identifier: Apache-2.0 -->

# Alpha Notice, SBOM, and Provenance Dry Run

This packet records what the alpha publication dry run can and cannot claim
about notices, SBOM output, and provenance. The canonical machine-readable
state lives in
[`alpha_publication_manifest.yaml`](alpha_publication_manifest.yaml).

## Source Chain

| Surface | Source of truth | Dry-run state |
| --- | --- | --- |
| Third-party notice projection | `artifacts/governance/release_notice_seed.yaml` and `artifacts/governance/third_party_import_manifest.yaml` | Reviewable, but reserved imports have pending notice text until real bytes are admitted. |
| Human notice delta | `artifacts/release/reuse_spdx_notice_delta_alpha.md` | Review required; names explicit exceptions and blocks stronger publication copy. |
| SBOM placeholder | `ci/sbom_provenance.sh` | Emits a structural workspace summary. It is not SPDX or CycloneDX. |
| Provenance seed | `artifacts/release/provenance_capture_seed.json` | Records clean-room provenance capture shape and known limitations. |
| Trust-domain context | `artifacts/release/pipeline_trust_domains.yaml` | Names release and mirror/offline trust boundaries for review. |

## Mirror and Offline Behavior

| Family | What remains verifiable without vendor reachability | What degrades |
| --- | --- | --- |
| Notices | The notice seed, import manifest rows, and notice delta packet remain locally readable. | Reserved imports still require future notice text; mirror/offline review must show the pending state. |
| SBOM | The placeholder script and generated structural format are identifiable. | Standards conformance is unavailable and must not be implied. |
| Provenance | The provenance capture seed names inputs, output refs, and known limitations. | Final signing, in-toto or SLSA-style attestations, and trust-root receipts are absent. |
| Advisory/revocation metadata | The manifest declares snapshot freshness limits. | Live advisory and revocation truth becomes stale after the declared limits and blocks stronger claims. |

## Blocked Claims

The dry run explicitly blocks these statements:

- "SPDX SBOM attached"
- "CycloneDX SBOM attached"
- "Signed attestation available"
- "All third-party notice text complete"
- "Live advisory or revocation state verified while offline"
- "Broader alpha publication ready"

Acceptable wording is narrower: "SBOM placeholder present", "provenance seed
present", "notice delta review required", and "offline snapshot freshness
declared".

## Acceptance Hooks

The publication validator checks that:

- notices and SBOM/provenance are covered in the same manifest as binaries,
  docs/help packs, policy bundles, symbols, and support schemas;
- mirror-only, deny-all, and offline-verification postures carry receipts for
  both notices and SBOM/provenance;
- reserved notice imports and placeholder SBOM state remain blockers; and
- vendor-unreachable behavior degrades to explicit snapshot states.

## Verification

```sh
python3 ci/release/check_alpha_publication_dry_run.py --repo-root . --validate-only
```
