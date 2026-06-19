# Implement canonical JSON Schema packages, explicit version fields, and stability labels for M5 durable artifacts

Evidence record for the canonical M5 JSON Schema catalog: the one index that
publishes a checked-in JSON Schema package — with an explicit in-band version
field, a lifecycle/stability label, a field-level compatibility contract, an
example payload, and a round-trip fixture — for every durable M5 artifact family
the publication matrix puts forward as a JSON-Schema-backed contract.

## What shipped

- A checked-in catalog mapping every durable M5 artifact family to its JSON
  Schema package, version field(s), lifecycle label, field contract, example, and
  round-trip fixture:
  [`/artifacts/contracts/m5-json-schema-catalog.json`](../contracts/m5-json-schema-catalog.json)
  (15 packages).
- The canonical JSON Schema packages, one per family:
  [`/schemas/public/m5-json/`](../../schemas/public/m5-json/).
- Version-stamped example payloads and round-trip fixtures:
  [`/examples/contracts/m5/json/`](../../examples/contracts/m5/json/) and
  [`/fixtures/contracts/m5-json-roundtrip/`](../../fixtures/contracts/m5-json-roundtrip/).
- The boundary schema:
  [`/schemas/public/m5-contracts/m5_json_schema_catalog.schema.json`](../../schemas/public/m5-contracts/m5_json_schema_catalog.schema.json).
- The SDK catalog and the narrative companion:
  [`/docs/sdk/m5-json-schema-catalog.md`](../../docs/sdk/m5-json-schema-catalog.md)
  and
  [`/docs/m5/implement-canonical-json-schema-packages-explicit-version-fields-and-stability-labels-for-newly-stable-or-beta-m5-durable-artifacts.md`](../../docs/m5/implement-canonical-json-schema-packages-explicit-version-fields-and-stability-labels-for-newly-stable-or-beta-m5-durable-artifacts.md).
- The typed product object plus its validator and consumer projections:
  `crates/aureline-release/src/implement_canonical_json_schema_packages_explicit_version_fields_and_stability_labels_for_newly_stable_or_beta_m5_durable_artifacts/`.
- The single source of truth (regenerator) and the validator:
  [`/tools/regenerate_m5_json_schema_catalog.py`](../../tools/regenerate_m5_json_schema_catalog.py)
  and
  [`/tools/validate_m5_json_schema_catalog.py`](../../tools/validate_m5_json_schema_catalog.py).
- Negative fixtures and CI capture:
  [`/fixtures/contracts/m5-json-catalog/`](../../fixtures/contracts/m5-json-catalog/) and
  [`/artifacts/release/captures/implement_canonical_json_schema_packages_explicit_version_fields_and_stability_labels_for_newly_stable_or_beta_m5_durable_artifacts_validation_capture.json`](../release/captures/implement_canonical_json_schema_packages_explicit_version_fields_and_stability_labels_for_newly_stable_or_beta_m5_durable_artifacts_validation_capture.json).

## Families covered

`command_descriptors`, `cli_headless_structured_output`, `task_event_envelope`,
`execution_context_provenance`, `diagnostic_records`, `project_doctor_findings`,
`repair_transactions`, `support_bundles_and_handoff`,
`appearance_sessions_and_theme_assets`, `teaching_tour_and_learning_packets`,
`policy_bundles`, `capability_records`,
`notification_and_chronology_primitives`, `replay_and_trace_evidence`, and
`service_optional_api` — every matrix family put forward as a JSON-Schema-backed
contract. The WIT-only `extension_host_wit_world` family is intentionally absent;
its contract form is a WIT world, not a JSON Schema.

## Acceptance, met

- **Every claimed stable or beta durable family has a checked-in JSON Schema
  package, version field, lifecycle label, and compatibility note.** The 15
  packages each carry a `schema_id`, a required in-band version field, a
  lifecycle label equal to the matrix `published_label`, and a compatibility note
  plus doc ref.
- **Export/import, support, and docs/help resolve the same schema identifier and
  lifecycle label.** Each package's `schema_id` and `lifecycle_label` are carried
  both in the catalog row and in the package schema's `$id` and
  `x-aureline-contract.lifecycle_label`; the validator asserts they agree and
  that the label equals the matrix `published_label`.
- **Schema examples and validators pass for a real M5 artifact.** Each package's
  example payload uses the family's real envelope fields (record-kind tag, version
  field, primary identity) and validates against the package schema; the typed
  Rust consumer in `aureline-release` parses and validates the catalog.

## Guardrails honored

- **No example-only JSON without a schema and version field.** Every example and
  round-trip fixture validates against a checked-in package schema and carries the
  declared version field.
- **No stripping of unknown fields for round-trip families.** Every package sets
  `additionalProperties: true`; the validator proves an undeclared field in each
  round-trip fixture survives validation and a parse/serialize round-trip.

## Reuse, not restatement

The catalog reuses the established governance sources rather than minting a new
lexicon: the durable families and version fields come from
`/artifacts/contracts/contract_families.yaml`, and each package's lifecycle label
is the effective `published_label` from
`/artifacts/contracts/m5-stability-lifecycle-map.json`. The validator
cross-checks both joins.

## Proof

Automated proof lives in
`crates/aureline-release/src/implement_canonical_json_schema_packages_explicit_version_fields_and_stability_labels_for_newly_stable_or_beta_m5_durable_artifacts/tests.rs`
and
`crates/aureline-release/tests/implement_canonical_json_schema_packages_explicit_version_fields_and_stability_labels_for_newly_stable_or_beta_m5_durable_artifacts.rs`:

- the checked-in catalog parses and validates with zero violations;
- every package declares a version field, a lifecycle label, a schema package,
  and a compatibility note, and preserves unknown fields;
- a schema identifier and lifecycle label resolve for a given family;
- the recomputed summary agrees with the checked-in catalog;
- the model matches the frozen CI validation capture;
- negative gates reject a duplicate package id, an unknown lifecycle label, and a
  drifted summary;
- the checked-in fixtures are rejected by the typed model.

`tools/validate_m5_json_schema_catalog.py` validates the catalog against the
schema, the semantic invariants, the regenerator (no hand-edit drift), every
package schema as a valid Draft 2020-12 schema, every example and round-trip
fixture against its package schema, unknown-field preservation, the matrix
lifecycle-label and registry version-field joins, every referenced path, and the
negative fixtures. It runs in the shared `ci/contract_validation.sh` lane and as
the dedicated `check_m5_json_schema_catalog` workflow.

## Reuse surfaces

`resolve_schema_label(...)` (export/import, support export, docs/help),
`package(...)` and `packages_for_label(...)` (CLI/SDK inspection), and
`stable_packages()` / `computed_summary()` (claim manifests and dashboards). Part
of the canonical M5 evidence train; the row narrows if its catalog, package
schemas, validator, or proof drift.
