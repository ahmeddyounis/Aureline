# Add import/export validators and cross-surface conformance runners for M5 request/API collections, notebook parity artifacts, docs packets, trace/profile/replay, support bundles, and portable-state packages

Evidence record for the canonical M5 interchange-conformance register: the machine-readable join that ties every named M5 import/export family to its import/export validator, its cross-surface conformance runner and the real emitted artifact it exercises, the contract version and lifecycle label its consumers agree on, and the stable reason codes an interchange failure reports.

## What shipped

- A checked-in interchange-conformance register: [`/artifacts/contracts/m5-interchange-conformance.json`](../contracts/m5-interchange-conformance.json) (6 families, 42 conformance-dimension evaluations).
- The conformance report: [`/artifacts/contracts/m5-interchange-conformance.md`](../contracts/m5-interchange-conformance.md).
- The per-family import/export validators and their manifest: [`/validators/m5-interchange/`](../../validators/m5-interchange/).
- The real emitted-artifact corpus the runners exercise: [`/fixtures/contracts/m5-interchange/emitted/`](../../fixtures/contracts/m5-interchange/emitted/).
- The Help-center page: [`/docs/help/m5-interchange-conformance.md`](../../docs/help/m5-interchange-conformance.md).
- The boundary schema: [`/schemas/public/m5-contracts/m5_interchange_conformance.schema.json`](../../schemas/public/m5-contracts/m5_interchange_conformance.schema.json).
- The typed product object, its protected tests, and the in-product CLI inspect surface: `crates/aureline-release/src/add_import_export_validators_and_cross_surface_conformance_runners_for_m5_interchange_families/` and `crates/aureline-release/src/bin/aureline_release_add_import_export_validators_and_cross_surface_conformance_runners_for_m5_interchange_families.rs`.
- The single source of truth (regenerator) and the validator: [`/tools/regenerate_m5_interchange_conformance.py`](../../tools/regenerate_m5_interchange_conformance.py) and [`/tools/validate_m5_interchange_conformance.py`](../../tools/validate_m5_interchange_conformance.py).
- Negative fixtures and CI capture: [`/fixtures/contracts/m5-interchange/negative/`](../../fixtures/contracts/m5-interchange/negative/) and [`/artifacts/release/captures/add_import_export_validators_and_cross_surface_conformance_runners_for_m5_interchange_families_validation_capture.json`](../release/captures/add_import_export_validators_and_cross_surface_conformance_runners_for_m5_interchange_families_validation_capture.json).

## Families covered

`request_api_collections`, `notebook_parity_exports`, `docs_packets`, `trace_profile_replay_exports`, `support_bundles`, `portable_state_packages`.

## How it stays honest

- A catalog-linked family's `lifecycle_label` equals the published contract family's label, so the interchange claim can never run ahead of the contract; the validator asserts the agreement against the contract catalog.
- Compare-only and import-validation-only are first-class conformance classes; a family the source docs scope to inspect-only behavior is not forced to support write-back.
- Import does not silently widen trust, strip required provenance, or drop unknown fields on a round-trip; the negative fixtures prove each rejection path and the model rejects a register that claims conformance while a required trust or provenance dimension fails.
- An interchange failure reports a stable, copy-safe reason code from the closed vocabulary; every family enumerates the codes its validator can report.

## Current decision

Promotion decision: **clear**. No release-blocking M5 interchange family has a failing required conformance dimension; every named import/export family is conformant in its declared conformance class.
