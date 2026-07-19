# Add import/export validators and cross-surface conformance runners for M5 request/API collections, notebook parity artifacts, docs packets, trace/profile/replay, support bundles, and portable-state packages

This is the narrative companion to the canonical **M5 interchange-conformance register**. The machine-readable register is authoritative; if the two disagree, the register wins and this document must be updated in the same change.

- Register (source of truth): `artifacts/contracts/m5-interchange-conformance.json`
- Conformance report: `artifacts/contracts/m5-interchange-conformance.md`
- Validators: `validators/m5-interchange/` (manifest `validators/m5-interchange/manifest.json`)
- Emitted-artifact corpus: `fixtures/contracts/m5-interchange/emitted/`
- Help-center page: `docs/help/m5-interchange-conformance.md`
- Boundary schema: `schemas/public/m5-contracts/m5_interchange_conformance.schema.json`
- Validator: `tools/validate_m5_interchange_conformance.py`
- Regenerator: `tools/regenerate_m5_interchange_conformance.py`
- Typed consumer + protected tests: `aureline-release` (`add_import_export_validators_and_cross_surface_conformance_runners_for_m5_interchange_families`)
- Evidence/proof packet: `artifacts/m5/add-import-export-validators-and-cross-surface-conformance-runners-for-m5-request-api-collections-notebook-parity-artifacts-docs-packets-trace-profile-replay-support-bundles-and-portable-state-packages.md`

## What the register is for

M5 ships many versioned import/export families. This register is the conformance layer that proves each high-value family survives real product use across the desktop, CLI/headless, and support/export surfaces. Per family it binds the import/export validator that guards it, the cross-surface conformance runner that exercises a real emitted artifact, the contract version and lifecycle label the consumers must agree on, the degraded-state vocabulary they share, and the stable, copy-safe reason codes an interchange failure reports.

## What shipped

- A checked-in interchange-conformance register over all 6 named M5 interchange families (2 release-blocking, 2 linked to a published contract family that supplies their lifecycle label).
- A per-family import/export validator descriptor and a real emitted artifact for each family (42 conformance-dimension evaluations in all), under `validators/m5-interchange/` and `fixtures/contracts/m5-interchange/emitted/`.
- The conformance report, the Help-center page, the boundary schema, the validator, the regenerator, a typed Rust consumer with an in-product CLI inspect surface, and negative fixtures that prove each rejection path.

## Current decision

The interchange-conformance promotion decision is **clear**.

## In-product inspect surface

The typed consumer ships a headless inspect bin that prints the register, a per-family inspect view, the support/export projection, and the validator manifest, with no live service:

```sh
cargo run -q -p aureline-release --bin aureline_release_add_import_export_validators_cross -- inspect support_bundles
```
