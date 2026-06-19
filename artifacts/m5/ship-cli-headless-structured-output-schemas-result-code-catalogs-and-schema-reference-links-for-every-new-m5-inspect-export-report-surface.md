# Ship CLI/headless structured-output schemas, result-code catalogs, and schema-reference links for every new M5 inspect/export/report surface

Evidence record for the canonical M5 CLI/headless structured-output and
result-code catalog: the one index that gives every new M5 CLI/headless inspect,
export, report, and health surface a stable structured-output schema reference, a
stable result-code catalog, a lifecycle label, an explicit partial-result and
freshness vocabulary, and a UI/CLI parity proof.

## What shipped

- A checked-in catalog mapping every new M5 CLI/headless surface to its
  structured-output schema reference, result-code catalog, lifecycle label,
  partial-result and freshness vocabularies, and UI/CLI parity fixtures:
  [`/artifacts/contracts/m5-cli-output-catalog.json`](../contracts/m5-cli-output-catalog.json)
  (9 surfaces).
- The boundary schema:
  [`/schemas/public/m5-cli/m5_cli_output_catalog.schema.json`](../../schemas/public/m5-cli/m5_cli_output_catalog.schema.json).
- The per-surface CLI and UI inspect parity fixtures:
  [`/fixtures/contracts/m5-cli-json/`](../../fixtures/contracts/m5-cli-json/).
- The CLI reference doc and the narrative companion:
  [`/docs/cli/m5-structured-output-and-result-codes.md`](../../docs/cli/m5-structured-output-and-result-codes.md)
  and
  [`/docs/m5/ship-cli-headless-structured-output-schemas-result-code-catalogs-and-schema-reference-links-for-every-new-m5-inspect-export-report-surface.md`](../../docs/m5/ship-cli-headless-structured-output-schemas-result-code-catalogs-and-schema-reference-links-for-every-new-m5-inspect-export-report-surface.md).
- The typed product object plus its validator and consumer projections:
  `crates/aureline-release/src/ship_cli_headless_structured_output_schemas_result_code_catalogs_and_schema_reference_links_for_every_new_m5_inspect_export_report_surface/`.
- The single source of truth (regenerator) and the validator:
  [`/tools/regenerate_m5_cli_output_catalog.py`](../../tools/regenerate_m5_cli_output_catalog.py)
  and
  [`/tools/validate_m5_cli_output_catalog.py`](../../tools/validate_m5_cli_output_catalog.py).
- Negative fixtures and CI capture:
  [`/fixtures/contracts/m5-cli-catalog/`](../../fixtures/contracts/m5-cli-catalog/) and
  [`/artifacts/release/captures/ship_cli_headless_structured_output_schemas_result_code_catalogs_and_schema_reference_links_for_every_new_m5_inspect_export_report_surface_validation_capture.json`](../release/captures/ship_cli_headless_structured_output_schemas_result_code_catalogs_and_schema_reference_links_for_every_new_m5_inspect_export_report_surface_validation_capture.json).

## Surfaces covered

`command_inspect` (inspect), `support_bundle_export` (export),
`diagnostics_report` (report), `project_doctor_health` (health),
`restore_provenance_inspect` (inspect), `ai_evidence_export` (export),
`capability_qualification_inspect` (inspect), `repair_transaction_report`
(report), and `policy_config_inspect` (inspect) — covering command inspection,
support/export packets, qualification/claim inspection, config inspectors,
restore-provenance views, AI evidence exports, search diagnostics, and recovery
reports touched by M5. Each surface resolves its structured-output schema to a
real package in the M5 JSON Schema catalog.

## Acceptance, met

- **Every claimed M5 CLI/headless surface points to a checked-in schema reference
  and stable result-code catalog.** Each of the 9 surfaces carries a
  `structured_output_schema_ref` that resolves to a checked-in
  `schemas/public/m5-json/<family>.schema.json` package whose `$id` and family id
  agree, and a non-empty `result_code_catalog` whose every code is a member of the
  closed `exit_code_class` vocabulary.
- **Breaking or deprecated output fields produce diff/migration behavior instead
  of silent change.** The structured-output schema is resolved from the JSON
  Schema catalog and evolves under that package's additive-minor / frozen-required
  contract; the result-code, partial-result, and freshness vocabularies are
  closed and stable, and a surface that loses required evidence narrows below the
  launch cutline rather than emitting an undeclared shape.
- **At least one fixture per major M5 family proves identical
  lifecycle/degraded-state vocabulary between UI inspect surfaces and CLI/headless
  output.** Every surface ships a CLI fixture and a UI inspect fixture; both
  validate against the same resolved package schema and the validator asserts they
  carry an identical `partial_result_state`, `freshness_state`, and
  `lifecycle_label`.

## Guardrails honored

- **No human-readable CLI text as a substitute for a versioned machine-readable
  contract.** Every surface binds a real JSON Schema package and a machine-output
  envelope class; the result-code catalog keys off stable enums, not prose.
- **No UI-only fields leaking into structured output without lifecycle and support
  posture.** Every surface declares a lifecycle label (equal to the matrix
  published label), a partial-result vocabulary, and a freshness vocabulary, and
  the UI/CLI parity fixtures prove the two surfaces share the same vocabulary.

## Reuse, not restatement

The catalog reuses the established governance sources: structured-output schema
references resolve to packages published by
`/artifacts/contracts/m5-json-schema-catalog.json`; result codes and envelope
classes are members of the closed vocabularies frozen in
`/schemas/automation/cli_output_registry_entry.schema.json`; and each surface's
lifecycle label is the effective `published_label` from
`/artifacts/contracts/m5-stability-lifecycle-map.json`. The validator cross-checks
all three joins.

## Proof

Automated proof lives in
`crates/aureline-release/src/ship_cli_headless_structured_output_schemas_result_code_catalogs_and_schema_reference_links_for_every_new_m5_inspect_export_report_surface/tests.rs`
and
`crates/aureline-release/tests/ship_cli_headless_structured_output_schemas_result_code_catalogs_and_schema_reference_links_for_every_new_m5_inspect_export_report_surface.rs`:

- the checked-in catalog parses and validates with zero violations;
- every surface declares a structured-output schema reference, a non-empty
  result-code catalog with a success and an error row, a lifecycle label, and a
  parity fixture pair;
- a surface's schema reference and lifecycle label resolve for a given surface id;
- the recomputed summary agrees with the checked-in catalog;
- the model matches the frozen CI validation capture;
- negative gates reject a duplicate surface id, an off-vocabulary result code, and
  a drifted summary;
- the checked-in fixtures are rejected by the typed model.

`tools/validate_m5_cli_output_catalog.py` validates the catalog against the
boundary schema, the semantic invariants, the regenerator (no hand-edit drift),
every surface's schema-ref resolution to a JSON Schema catalog package, the
result-code and envelope vocabulary reuse against the CLI output registry, the
matrix lifecycle join, the UI/CLI parity vocabulary, every referenced path, and
the negative fixtures. It runs in the shared `ci/contract_validation.sh` lane and
as the dedicated `check_m5_cli_output_catalog` workflow.

## Reuse surfaces

`resolve_surface_schema(...)` (CLI help, docs, sample payloads),
`surface(...)` and `surfaces_for_kind(...)` (CLI/SDK inspection), and
`stable_surfaces()` / `computed_summary()` (claim manifests and dashboards). Part
of the canonical M5 evidence train; the row narrows if its catalog, boundary
schema, parity fixtures, validator, or proof drift.
