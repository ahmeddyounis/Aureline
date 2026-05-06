# VS Code migration/import for TS/JS users (reference scenario)

## Covers acceptance rows

- `ts_js_acceptance_row:migration_import_vscode_expectations`

## Binding

- Launch bundle: `launch_bundle:typescript_web_app.seed`
- Archetype row: `archetype_row:ts_web_app_or_service`
- Migration source row: `migration_source:vs_code_code_oss`

## Scenario goal

Prove that a TS/JS user switching from VS Code can:

- preview an import plan,
- review an explicit diff (including unsupported-field reporting),
- apply the import without silent trust widening, and
- rollback to a checkpoint.

Scope is limited to the marketed VS Code migration lane. It does not claim
extension-runtime parity.

## Required truth and disclosures

- Migration scope and quality bars are governed by:
  - `docs/migration/source_ecosystem_coverage_matrix.md`
  - `artifacts/migration/source_ecosystem_rows.yaml`
- First-run import diff + rollback contract is governed by:
  - `docs/migration/first_run_import_diff_and_rollback_contract.md`

## Evidence hooks

- Source-profile examples exist under:
  - `fixtures/migration/source_profile_examples/`

## Known-limit expectations

- Any narrowed import surface (for example “launch configs import only for
  a subset of debug adapters”) requires a known-limit note before certified
  bundle copy can mention the migration lane without caveats.

