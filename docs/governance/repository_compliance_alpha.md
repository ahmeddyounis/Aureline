<!-- SPDX-License-Identifier: Apache-2.0 -->

# Alpha Repository Compliance Contract

This page is the reviewer entry point for the alpha repository-compliance
lane. The machine-readable source of truth is
[`artifacts/governance/third_party_import_manifest.yaml`](../../artifacts/governance/third_party_import_manifest.yaml).

## Canonical Artifacts

| Artifact | Role |
| --- | --- |
| [`third_party_import_manifest.yaml`](../../artifacts/governance/third_party_import_manifest.yaml) | Maps every alpha artifact family to origin, license, upstream version, local modification posture, update owner, DCO state, and notice/SPDX state |
| [`critical_upstream_health_register.yaml`](../../artifacts/governance/critical_upstream_health_register.yaml) | Names red-risk upstreams with owner, health status, and fork/replace/escalate trigger |
| [`reuse_spdx_notice_delta_alpha.md`](../../artifacts/release/reuse_spdx_notice_delta_alpha.md) | Human review packet for REUSE/SPDX gaps, notice projection, and SBOM/provenance limits |
| [`dco_merge_audit_alpha.md`](../../artifacts/governance/dco_merge_audit_alpha.md) | DCO 1.1 merge-audit packet and historical baseline exception |
| [`validate_import_manifest.py`](../../ci/release/validate_import_manifest.py) | CI/release validator that reads the alpha artifact graph and the compliance artifacts together |
| [`check_dco_signoff.sh`](../../ci/release/check_dco_signoff.sh) | Commit-trailer checker for new merge commits |

## Review Rule

Release, docs, support, and publication evidence must consume the manifest
rows instead of copying status text. If an artifact family is added to the
alpha artifact graph, the same change must add or update a manifest row,
notice delta entry, and any critical-upstream health row needed to keep the
claim honest.

## Commands

```sh
python3 ci/release/validate_import_manifest.py --repo-root .
ci/release/check_dco_signoff.sh --audit artifacts/governance/dco_merge_audit_alpha.md
```
