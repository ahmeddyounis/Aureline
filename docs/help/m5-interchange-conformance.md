# M5 import/export interchange conformance

This Help-center page explains how Aureline proves that its **import/export families** survive real product use. Request/API collections, notebook paired/parity exports, docs packets, trace/profile/replay captures, support bundles, and portable-state packages each carry a published contract version and lifecycle label, an import/export validator, and a cross-surface conformance runner that exercises a real exported artifact across the desktop, CLI/headless, and support/export surfaces.

## What conformance guarantees

- A family does not claim interchange support on a local export alone: it must prove an import/validation path and more than one consumer path.
- Import does not silently widen trust, strip required provenance, or break the round-trip rules the rest of the contract lane promises.
- Where a family is scoped to compare-only or inspect-only behavior, that is a valid conformance class — write-back is never forced.
- An interchange failure reports a stable, copy-safe reason code and diagnostic instead of a raw parser exception or a generic corruption message.

## The families

| Family | Conformance class | Lifecycle label |
| --- | --- | --- |
| Request/API collection import & export | `round_trip_write_back` | beta |
| Notebook paired/parity export & compare | `compare_only` | beta |
| Docs suggestion/validation/evidence packet import | `import_validation_only` | beta |
| Trace/profile/replay export & round-trip | `round_trip_write_back` | beta |
| Support bundle export & import validation | `import_validation_only` | stable |
| Portable-state package export & round-trip | `round_trip_write_back` | beta |

## Where to look

- Interchange-conformance register (source of truth): `artifacts/contracts/m5-interchange-conformance.json`
- Conformance report: `artifacts/contracts/m5-interchange-conformance.md`
- Validators: `validators/m5-interchange/`
- Emitted-artifact corpus: `fixtures/contracts/m5-interchange/emitted/`
- Contract catalog and publication matrix: `artifacts/contracts/m5-contract-catalog.json`, `artifacts/contracts/m5-stability-lifecycle-map.json`

## Freshness

The register is current as of `2026-06-19`. CI regenerates it from the contract catalog via `tools/regenerate_m5_interchange_conformance.py`, runs `tools/validate_m5_interchange_conformance.py`, and runs the typed Rust consumer's tests, so the register, validators, report, and docs cannot drift from the upstream contract truth.
