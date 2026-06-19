# Ship CLI/headless structured-output schemas, result-code catalogs, and schema-reference links for every new M5 inspect/export/report surface

This is the narrative companion to the canonical **M5 CLI/headless
structured-output and result-code catalog**. The machine-readable catalog is
authoritative; if the two disagree, the catalog wins and this document must be
updated in the same change.

- Catalog (source of truth): `artifacts/contracts/m5-cli-output-catalog.json`
- Boundary schema: `schemas/public/m5-cli/m5_cli_output_catalog.schema.json`
- Parity fixtures (CLI + UI inspect, one pair per surface):
  `fixtures/contracts/m5-cli-json/*.{cli,ui}.json`
- CLI reference doc: `docs/cli/m5-structured-output-and-result-codes.md`
- Validator: `tools/validate_m5_cli_output_catalog.py`
- Regenerator: `tools/regenerate_m5_cli_output_catalog.py`
- Typed consumer + protected tests: `aureline-release`
  (`ship_cli_headless_structured_output_schemas_result_code_catalogs_and_schema_reference_links_for_every_new_m5_inspect_export_report_surface`)
- Evidence/proof packet:
  `artifacts/m5/ship-cli-headless-structured-output-schemas-result-code-catalogs-and-schema-reference-links-for-every-new-m5-inspect-export-report-surface.md`

## What the catalog is for

The canonical M5 JSON Schema catalog publishes the durable artifact *families*.
This catalog gives every new M5 **CLI/headless surface** that emits structured
output — inspect, export, report, and health — a stable, checked-in contract:

- a **structured-output schema reference** resolved from the JSON Schema catalog
  (`schemas/public/m5-json/<family>.schema.json`), so a surface points at a real
  versioned schema rather than restating field semantics;
- a **result-code catalog** of stable enums, each with a pinned numeric code and
  a partial-result flag;
- a **lifecycle label** equal to the publication matrix's effective published
  label for the family; and
- explicit **partial-result** and **freshness** vocabularies so machine output is
  safe for automation.

The catalog is the source CLI help, docs, sample payloads, and support bundles
resolve a surface's schema reference and result-code catalog from, instead of
restating them per surface.

## What each surface row publishes

Every surface row records:

- **Surface identity** — a stable `surface_id`, the `surface_kind`
  (`inspect`/`export`/`report`/`health`), and the `command_id` it reuses from the
  command descriptor so CLI and desktop resolve one command.
- **Structured-output schema** — the `structured_output_schema_ref` and
  `structured_output_schema_id` resolved from the JSON Schema catalog package, and
  the `output_envelope_class` a machine consumer binds against.
- **Result-code catalog** — a non-empty list of result-code rows, each drawn from
  the closed `exit_code_class` vocabulary frozen in
  `schemas/automation/cli_output_registry_entry.schema.json`, with a pinned
  numeric code and a `partial_result` flag. `success` and
  `success_no_action_taken` are always numeric `0`.
- **Lifecycle label** — `lts`/`stable`/`beta`/`preview`/`withdrawn`, equal to the
  publication matrix `published_label`, plus the matching
  `machine_output_stability_class`.
- **Partial-result and freshness vocabularies** — the closed
  `partial_result_states` (`complete`/`partial`/`degraded`/`unavailable`/
  `stale_retest_needed`) and `freshness_states` (`fresh`/`stale`/`retest_needed`/
  `unknown`) the surface can emit.
- **UI/CLI parity** — the `ui_inspect_surface`, the `parity_match_mode`, and a CLI
  fixture and a UI inspect fixture proving the lifecycle/degraded-state vocabulary
  is identical on both surfaces.
- **Downgrade behavior** — a surface missing its schema reference, result-code
  catalog, lifecycle label, or parity fixture narrows below the launch cutline
  rather than emitting an undeclared shape.

## Result codes, partial results, and staleness

The result-code catalog reuses the closed `exit_code_class` vocabulary rather than
minting a new one, so downstream tooling keys off one stable enum across CLI,
desktop, and support surfaces. A surface that cannot fully resolve emits
`partial_success_with_warnings` with a `partial_result_state` of `partial` or
`degraded`; a surface whose inputs are stale emits a `freshness_state` of `stale`
or `retest_needed` so automation never mistakes a stale cache for a fresh result.
The validator enforces the coupling: a surface that declares a partial/degraded
state must publish the partial-result carrier code, and vice versa, and every
surface must be able to report `stale_retest_needed`.

## UI/CLI parity

Each surface ships a CLI fixture and a UI inspect fixture under
`fixtures/contracts/m5-cli-json/`. Both validate against the same resolved JSON
Schema package, and the validator proves they carry an identical
`partial_result_state`, `freshness_state`, and `lifecycle_label`. This is the
mechanical proof of the acceptance criterion that the lifecycle/degraded-state
vocabulary matches field-for-field between the UI inspect surface and the
CLI/headless output.

## Offline and mirror use

The catalog, its boundary schema, the parity fixtures, and the validator bundle
into offline/mirror artifact sets and validate without runtime service access
(`offline_bundle.requires_runtime_service` is `false`).

## How downstream surfaces consume it

- **CLI help, docs, and sample payloads** resolve a surface's schema reference and
  result-code catalog from the catalog and quote one identity per surface.
- **Support/export bundles** fold the surface's `surface_id`, schema reference, and
  result-code catalog in so a support consumer keys off stable enums.
- **Claim-publication automation** narrows any marketed/support-class family whose
  required structured-output schema, result-code catalog, parity fixture, or
  validator evidence is missing, stale, or downgraded.

## Maintenance

Edit the surface set in `tools/regenerate_m5_cli_output_catalog.py`, then run:

```bash
python3 tools/regenerate_m5_cli_output_catalog.py
python3 tools/validate_m5_cli_output_catalog.py
cargo test -p aureline-release --test \
  ship_cli_headless_structured_output_schemas_result_code_catalogs_and_schema_reference_links_for_every_new_m5_inspect_export_report_surface
```

The regenerator is the single source of truth for the catalog, the parity
fixtures, the CLI doc, the CI validation capture, and the negative fixtures. The
validator and the typed Rust consumer both recompute the derived state, so a
hand-edited catalog or fixture fails CI.
