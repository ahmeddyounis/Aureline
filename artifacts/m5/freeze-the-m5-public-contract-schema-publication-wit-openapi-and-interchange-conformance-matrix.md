# Freeze the M5 public-contract, schema-publication, WIT/OpenAPI, and interchange-conformance matrix

Evidence record for the canonical M5 public-contract publication matrix: the one
inventory that names every M5 artifact family the source docs treat as a
published contract and freezes which contract forms it must publish before it can
hold a Stable contract claim.

## What shipped

- A checked-in matrix mapping every M5 public-contract family to a contract form,
  stability lane, reader/writer posture, packaging need, per-form publication
  requirement, validator suite, and release-packet linkage:
  [`/artifacts/contracts/m5-stability-lifecycle-map.json`](../contracts/m5-stability-lifecycle-map.json)
  (16 families).
- Flat and human-readable projections derived from the same source:
  [`/artifacts/contracts/m5-public-contract-inventory.csv`](../contracts/m5-public-contract-inventory.csv)
  and
  [`/artifacts/contracts/m5-public-contract-matrix.md`](../contracts/m5-public-contract-matrix.md).
- The boundary schema:
  [`/schemas/public/m5-contracts/m5_public_contract_matrix.schema.json`](../../schemas/public/m5-contracts/m5_public_contract_matrix.schema.json).
- The contract and narrative companion:
  [`/docs/m5/freeze-the-m5-public-contract-schema-publication-wit-openapi-and-interchange-conformance-matrix.md`](../../docs/m5/freeze-the-m5-public-contract-schema-publication-wit-openapi-and-interchange-conformance-matrix.md).
- WIT and OpenAPI publication indexes:
  [`/wit/m5-contracts/README.md`](../../wit/m5-contracts/README.md) and
  [`/openapi/m5/README.md`](../../openapi/m5/README.md).
- A worked example row and corpus index:
  [`/examples/contracts/m5/`](../../examples/contracts/m5/).
- The canonical product object plus its validator and consumer projections:
  `crates/aureline-release/src/freeze_the_m5_public_contract_schema_publication_wit_openapi_and_interchange_conformance_matrix/`.
- The single source of truth (regenerator) and the validator:
  [`/tools/regenerate_m5_public_contract_matrix.py`](../../tools/regenerate_m5_public_contract_matrix.py)
  and
  [`/tools/validate_m5_public_contract_matrix.py`](../../tools/validate_m5_public_contract_matrix.py).
- Negative fixtures and CI capture:
  [`/fixtures/contracts/m5/`](../../fixtures/contracts/m5/) and
  [`/artifacts/release/captures/freeze_the_m5_public_contract_schema_publication_wit_openapi_and_interchange_conformance_matrix_validation_capture.json`](../release/captures/freeze_the_m5_public_contract_schema_publication_wit_openapi_and_interchange_conformance_matrix_validation_capture.json).

## Families covered

`command_descriptors`, `cli_headless_structured_output`, `task_event_envelope`,
`execution_context_provenance`, `diagnostic_records`, `project_doctor_findings`,
`repair_transactions`, `support_bundles_and_handoff`,
`appearance_sessions_and_theme_assets`, `teaching_tour_and_learning_packets`,
`policy_bundles`, `capability_records`,
`notification_and_chronology_primitives`, `replay_and_trace_evidence`,
`extension_host_wit_world`, and `service_optional_api` — spanning the
JSON-Schema-backed, record-registry, event-envelope, CLI-structured-output,
asset-package, teaching-content, WIT-world-package, and OpenAPI-family contract
forms.

## Reuse, not restatement

The matrix reuses, rather than restates, the established governance vocabularies:
the contract-form/category/maturity-lane lexicon from
`/artifacts/governance/compatibility_surfaces.yaml`, the lifecycle labels and
launch cutline from the stable claim matrix, the qualification rows from
`/artifacts/compat/qualification_matrix_seed.yaml`, the contract-family rows from
`/artifacts/contracts/contract_families.yaml`, and the claim manifest at
`/artifacts/release/stable_claim_manifest.json`. It mints no new contract-status
lexicon.

## Auto-narrowing (worked examples)

The current matrix holds promotion because two release-blocking families put
forward at the cutline are missing required publication evidence:

- `task_event_envelope` narrows to Beta — no migration/deprecation notes are
  published yet (`migration_notes_unpublished`).
- `service_optional_api` narrows to Beta — its OpenAPI family is still a seed
  (`openapi_spec_unpublished`).

A family is narrowed automatically the moment any required contract form,
validator suite, migration note, or release linkage is missing; it never inherits
an adjacent published family's claim.

## Proof

Automated proof lives in
`crates/aureline-release/src/freeze_the_m5_public_contract_schema_publication_wit_openapi_and_interchange_conformance_matrix/tests.rs`
and
`crates/aureline-release/tests/freeze_the_m5_public_contract_schema_publication_wit_openapi_and_interchange_conformance_matrix.rs`:

- the checked-in matrix parses and validates with zero violations;
- WIT and OpenAPI contract forms are inventoried, and every gap reason has a stop
  rule;
- the recomputed summary and promotion verdict agree with the checked-in matrix;
- the model matches the frozen CI validation capture;
- negative gates reject a published family with an unpublished requirement, a
  narrowed family that drops its gap reasons, a required form marked
  not-applicable, a row published wider than its claim, a promotion that proceeds
  while a rule fires, and a duplicate family id;
- the three checked-in fixtures are rejected by the typed model.

`tools/validate_m5_public_contract_matrix.py` validates the JSON against the
schema, the standalone row example against the row definition, the semantic
invariants (derived gap reasons, narrowing, promotion, summary), the CSV/Markdown
projections (no hand-edit drift), every referenced path, the contract-family
registry anchors, and the negative fixtures. It runs in the shared
`ci/contract_validation.sh` lane and as the dedicated
`check_m5_public_contract_matrix` workflow.

## Reuse surfaces

`support_export_projection()` (Help/About, SDK/docs, support export),
`computed_promotion_decision()` / `computed_blocking_family_ids()` (claim
manifests and shiproom dashboards), and `rows_for_form(...)` /
`release_blocking_rows()` (release-center inspection). Part of the canonical M5
evidence train; the row narrows if its artifact, schema, validator, or proof
drift.
