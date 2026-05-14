<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- dco-baseline-cutoff: f055fda88c9a737cf978ff50e2b0a4ad26c21618 -->

# Alpha DCO Merge Audit

This packet records the current Developer Certificate of Origin 1.1
posture for protected alpha artifact families. It is intentionally
bounded: commit-trailer enforcement is required for new merge commits,
while unsigned repository history that predates the CI gate is named as
an explicit baseline exception.

## Canonical Rules

| Field | Value |
| --- | --- |
| Contributor rule | Every new merge commit carries a `Signed-off-by:` trailer for Developer Certificate of Origin 1.1 |
| CI entrypoint | `ci/release/check_dco_signoff.sh` |
| Import manifest | `artifacts/governance/third_party_import_manifest.yaml` |
| Historical exception | `baseline.repository_history_before_dco_ci` |
| Baseline cutoff | `f055fda88c9a737cf978ff50e2b0a4ad26c21618` |
| Future merge behavior | Unsigned commits after the cutoff fail the DCO check unless a narrower exception is added here and reviewed |

## Protected Family Audit

| Anchor | Artifact family key | DCO posture | Exception ref |
| --- | --- | --- | --- |
| binaries | `binaries` | `signed_or_required_for_new_commits` | `baseline.repository_history_before_dco_ci` for pre-gate history only |
| symbols | `symbols` | `signed_or_required_for_new_commits` | `baseline.repository_history_before_dco_ci` for pre-gate history only |
| docs_help_packs | `docs_help_packs` | `signed_or_required_for_new_commits` | `baseline.repository_history_before_dco_ci` for pre-gate history only |
| schemas | `schemas` | `signed_or_required_for_new_commits` | `baseline.repository_history_before_dco_ci` for pre-gate history only |
| support_exports | `support_exports` | `signed_or_required_for_new_commits` | `baseline.repository_history_before_dco_ci` for pre-gate history only |
| release_evidence | `release_evidence` | `signed_or_required_for_new_commits` | `baseline.repository_history_before_dco_ci` for pre-gate history only |
| update_metadata | `update_metadata` | `signed_or_required_for_new_commits` | `baseline.repository_history_before_dco_ci` for pre-gate history only |
| supply_chain | `supply_chain` | `signed_or_required_for_new_commits` | `baseline.repository_history_before_dco_ci` for pre-gate history only |

## Third-Party Reserved Imports

| Anchor | Manifest row | DCO posture | Reason |
| --- | --- | --- | --- |
| third-party-reserved-imports | `alpha.import.third_party.noto_subset` | `not_applicable_external_reserved_import` | The row reserves a future third-party import; no upstream bytes are checked in by the current alpha seed |
| third-party-reserved-imports | `alpha.import.third_party.docs_official_pack` | `not_applicable_external_reserved_import` | The row reserves a future mirrored docs pack; no upstream bytes are checked in by the current alpha seed |
| third-party-build-inputs | `alpha.import.build_tooling.rust_toolchain` | `not_applicable_external_reserved_import` | The toolchain is observed as a host/build input and is not redistributed by the current alpha seed |

## Documented Exceptions

| Exception id | Scope | Reason | Owner | CI behavior | Exit condition |
| --- | --- | --- | --- | --- | --- |
| `baseline.repository_history_before_dco_ci` | Commits reachable from cutoff `f055fda88c9a737cf978ff50e2b0a4ad26c21618` | The repository had no executable DCO gate when the historical seed commits were created | `@ahmeddyounis` | `ci/release/check_dco_signoff.sh --audit artifacts/governance/dco_merge_audit_alpha.md` treats those commits as documented exceptions and still fails unsigned commits after the cutoff | A history rewrite or signed baseline import replaces the exception, or the project formally accepts this baseline as permanent historical evidence |

## CI Usage

```sh
# Check the current review range using the environment or upstream branch.
ci/release/check_dco_signoff.sh --audit artifacts/governance/dco_merge_audit_alpha.md

# Check an explicit range.
ci/release/check_dco_signoff.sh \
  --audit artifacts/governance/dco_merge_audit_alpha.md \
  --range origin/main..HEAD
```
