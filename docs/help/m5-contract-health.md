# M5 contract health and release gates

This Help-center page explains how Aureline keeps its **published M5 contract packages** honest at release time. Every published contract family — its JSON Schema, WIT world, or OpenAPI spec, plus its example corpus, validator suite, and compatibility report — is checked by a CI gate, tied to the exact build identity it shipped, and surfaced on the shiproom blocker dashboard.

## What the gates guarantee

- A missing, stale, downgraded, or incompatible contract package blocks the same release and claim-publication paths as missing evidence or a stale qualification row.
- One build identity proves the contract set the candidate shipped: each family resolves to a release packet, an artifact-graph node, and a package version.
- Shiproom reads a machine-readable contract-health summary instead of an ad hoc spreadsheet check.

## The gates

| Gate | What it checks |
| --- | --- |
| `schema_spec_package` | Schema/spec contract package published and fresh |
| `example_corpus` | Example payload corpus published |
| `validator_coverage` | Validator suite wired |
| `compatibility_report` | Compatibility / migration report fresh |
| `release_packet_linkage` | Release packet linked to the artifact graph and build identity |

## Where to look

- Contract-health register (source of truth): `artifacts/release/m5-contract-health.json`
- CI gates: `ci/contracts/m5-contract-gates/`
- Shiproom blocker dashboard: `shiproom/m5-contract-blocker-dashboard.md`
- Publication matrix and contract catalog: `artifacts/contracts/m5-stability-lifecycle-map.json`, `artifacts/contracts/m5-contract-catalog.json`

## Freshness

The register is current as of `2026-06-19`. CI regenerates it from the publication matrix and contract catalog via `tools/regenerate_m5_contract_health.py`, runs `tools/validate_m5_contract_health.py`, and runs the typed Rust consumer's tests, so the register, gates, dashboard, and docs cannot drift from the upstream contract truth.
