# Evidence — certify schema publication, WIT/OpenAPI packaging, validator coverage, and compatibility truth on every claimed M5 public artifact family

Proof packet for the M5 public-contract certification closeout. It lists the checked-in artifacts, the upstream truth they join, and the automated and operator-facing proof that exercises them.

- Current as of: `2026-06-19`
- Certification decision: **HOLD**

## Checked-in artifacts

- Certification register: `artifacts/certification/m5-public-contract-certification.json`
- Certification report: `artifacts/certification/m5-public-contract-certification.md`
- Shiproom dashboard: `shiproom/m5-public-contract-certification-dashboard.md`
- Help-center page: `docs/help/m5-public-contract-certification.md`
- JSON Schema: `schemas/public/m5-contracts/m5_public_contract_certification.schema.json`
- Negative fixtures: `fixtures/contracts/m5-public-contract-certification/`
- CI capture: `artifacts/release/captures/certify_schema_publication_wit_openapi_packaging_validator_coverage_and_compatibility_truth_on_every_claimed_m5_public_artifact_family_validation_capture.json`

## Upstream contract truth joined

- Contract-health register: `artifacts/release/m5-contract-health.json`
- Publication matrix: `artifacts/contracts/m5-stability-lifecycle-map.json`
- Contract catalog: `artifacts/contracts/m5-contract-catalog.json`
- JSON Schema catalog: `artifacts/contracts/m5-json-schema-catalog.json`
- OpenAPI catalog: `artifacts/contracts/m5-openapi-catalog.json`
- WIT publication: `artifacts/contracts/m5-wit-contract-publication.json`
- Reader/writer compatibility suite: `artifacts/contracts/m5-reader-writer-compat-suite.json`
- Interchange-conformance register: `artifacts/contracts/m5-interchange-conformance.json`
- Canonical M5 evidence index: `artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`

## Proof

- Schema + semantic + drift + cross-source validator: `tools/validate_m5_public_contract_certification.py`
- Typed Rust consumer + tests: `crates/aureline-release/src/certify_schema_publication_wit_openapi_packaging_validator_coverage_and_compatibility_truth_on_every_claimed_m5_public_artifact_family/`
- In-product inspect surface: `crates/aureline-release/src/bin/aureline_release_certify_schema_publication_wit_openapi_packaging_validator_coverage_and_compatibility_truth_on_every_claimed_m5_public_artifact_family.rs`
- CI gate: `.github/workflows/check_m5_public_contract_certification.yml`

## Result

15 of 16 claimed families certify their public contract claim; 0 narrow and 1 withhold. 1 family/families certify below the marketed claim, demonstrating the automatic narrowing the closeout requires.
