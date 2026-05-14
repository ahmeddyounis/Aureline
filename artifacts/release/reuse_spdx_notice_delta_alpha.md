<!-- SPDX-License-Identifier: Apache-2.0 -->

# Alpha REUSE/SPDX and Notice Delta

Source of truth:
[`artifacts/governance/third_party_import_manifest.yaml`](../governance/third_party_import_manifest.yaml).
This packet is the reviewer-facing delta over the manifest; release,
support, SBOM, and docs-pack consumers should read the manifest rows and
use this file only as the human review packet.

## Review State

| Field | State |
| --- | --- |
| Packet id | `alpha.repository_compliance.notice_delta` |
| Manifest ref | `artifacts/governance/third_party_import_manifest.yaml` |
| Release notice seed ref | `artifacts/governance/release_notice_seed.yaml` |
| DCO audit ref | `artifacts/governance/dco_merge_audit_alpha.md` |
| SPDX SBOM state | Placeholder only; `ci/sbom_provenance.sh` does not claim SPDX or CycloneDX conformance |
| REUSE state | New alpha compliance files carry SPDX headers; older source-bearing files remain covered by the documented sweep exception |
| Overall result | `review_required` |

## First-Party Family Coverage

These manifest rows cover checked-in first-party source, schemas,
fixtures, and release-control packets. They do not add third-party
notice text, but they keep each artifact family tied to a license and
DCO posture.

| Manifest row | Artifact family keys | License expression | SPDX/REUSE state | DCO state |
| --- | --- | --- | --- | --- |
| `alpha.import.family.binaries_source` | `binaries` | `Apache-2.0` | `covered_by_repository_default_with_delta_exceptions` | `signed_or_required_for_new_commits` |
| `alpha.import.family.symbols_source` | `symbols` | `Apache-2.0` | `covered_by_repository_default_with_delta_exceptions` | `signed_or_required_for_new_commits` |
| `alpha.import.family.docs_help_packs_source` | `docs_help_packs` | `Apache-2.0` | `covered_by_repository_default_with_delta_exceptions` | `signed_or_required_for_new_commits` |
| `alpha.import.family.schema_exports_source` | `schemas` | `Apache-2.0` | `covered_by_repository_default_with_delta_exceptions` | `signed_or_required_for_new_commits` |
| `alpha.import.family.support_exports_source` | `support_exports` | `Apache-2.0` | `covered_by_repository_default_with_delta_exceptions` | `signed_or_required_for_new_commits` |
| `alpha.import.family.release_evidence_source` | `release_evidence` | `Apache-2.0` | `covered_by_repository_default_with_delta_exceptions` | `signed_or_required_for_new_commits` |
| `alpha.import.family.update_metadata_source` | `update_metadata` | `Apache-2.0` | `covered_by_repository_default_with_delta_exceptions` | `signed_or_required_for_new_commits` |
| `alpha.import.family.supply_chain_source` | `supply_chain` | `Apache-2.0` | `provenance_only` | `signed_or_required_for_new_commits` |

## Third-Party Notice Projection

Third-party notices are projected from manifest row ids and their
`release_notice_seed.yaml` bindings. No separate notice id space is
introduced here.

| Manifest row | Source id | Notice state | Release action |
| --- | --- | --- | --- |
| `alpha.import.third_party.noto_subset` | `import.fonts.noto_subset` | Pending first import; no font bytes are checked in or shipped by this alpha seed | Emit bundled asset notice, SPDX entry, and provenance statement only after import |
| `alpha.import.third_party.docs_official_pack` | `import.docs.mirrored_official_pack` | Pending first mirror; no upstream docs pack bytes are checked in or shipped by this alpha seed | Emit docs-pack manifest attribution and provenance statement when mirrored pack is published |
| `alpha.import.build_tooling.rust_toolchain` | `dep.repo.rust_toolchain` | Build-input provenance only; not redistributed as a binary notice row | Emit build-tooling provenance and SBOM-input record only |

## Docs And Notice Delta

The docs/help family is currently first-party docs plus a reserved
mirrored-pack row. The mirrored row stays `review_required` until the
first pack mirror records upstream license terms, mirror digest,
freshness, and attribution text in the import manifest.

## Release Evidence Delta

The release-evidence family is checked-in alpha control evidence. It is
not a binary distribution notice pack. Its compliance delta is that
publication claims must remain blocked or narrowed while:

- the protected fitness packet remains evidence-stale;
- the DCO audit carries a documented historical baseline exception;
- the SPDX SBOM output remains a structural placeholder rather than a
  conformant SPDX document; and
- third-party reserved imports have not yet produced notice text.

## SBOM And Provenance Delta

`alpha.import.family.supply_chain_source` and
`alpha.import.build_tooling.rust_toolchain` keep supply-chain evidence
visible without overstating conformance. The current SBOM lane is a
placeholder that emits a structural workspace summary. Release copy may
say "SBOM placeholder" or "SPDX planned"; it must not say "SPDX SBOM
attached" until the replacement generator lands and validates.

## Explicit Exceptions

| Exception id | Scope | Reason | Owner | Exit condition |
| --- | --- | --- | --- | --- |
| `reuse.pre_existing_file_sweep_pending` | Source-bearing files that predate the current compliance lane | The repository adopted REUSE/SPDX metadata incrementally; new alpha compliance files carry SPDX headers, but older files still need the planned sweep | `@ahmeddyounis` | Official REUSE linter report is generated and all maintained source-bearing files either pass or carry a narrower file-level exception |
| `notice.reserved_import_not_yet_admitted` | `alpha.import.third_party.noto_subset`, `alpha.import.third_party.docs_official_pack` | Reserved imports have no checked-in third-party bytes yet, so notice text is pending rather than missing from a shipped payload | `@ahmeddyounis` | First imported bytes land with origin digest, upstream license text, local modification note, and generated notice output |
| `sbom.spdx_generator_not_yet_conformant` | Supply-chain evidence family | The checked-in SBOM script is a structural placeholder and explicitly does not claim SPDX or CycloneDX conformance | `@ahmeddyounis` | A real SPDX generator replaces the placeholder and a validator report is attached to release evidence |

## Review Commands

```sh
python3 ci/release/validate_import_manifest.py --repo-root .
ci/release/check_dco_signoff.sh --audit artifacts/governance/dco_merge_audit_alpha.md
```
