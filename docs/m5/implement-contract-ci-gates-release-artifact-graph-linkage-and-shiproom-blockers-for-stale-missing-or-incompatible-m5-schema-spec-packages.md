# Implement contract CI gates, release-artifact-graph linkage, and shiproom blockers for stale, missing, or incompatible M5 schema/spec packages

This is the narrative companion to the canonical **M5 contract-health register**. The machine-readable register is authoritative; if the two disagree, the register wins and this document must be updated in the same change.

- Register (source of truth): `artifacts/release/m5-contract-health.json`
- CI gates: `ci/contracts/m5-contract-gates/` (manifest `ci/contracts/m5-contract-gates/manifest.json`)
- Shiproom blocker dashboard: `shiproom/m5-contract-blocker-dashboard.md`
- Help-center page: `docs/help/m5-contract-health.md`
- Boundary schema: `schemas/public/m5-contracts/m5_contract_health.schema.json`
- Validator: `tools/validate_m5_contract_health.py`
- Regenerator: `tools/regenerate_m5_contract_health.py`
- Typed consumer + protected tests: `aureline-release` (`implement_contract_ci_gates_release_artifact_graph_linkage_and_shiproom_blockers_for_stale_missing_or_incompatible_m5_schema_spec_packages`)
- Evidence/proof packet: `artifacts/m5/implement-contract-ci-gates-release-artifact-graph-linkage-and-shiproom-blockers-for-stale-missing-or-incompatible-m5-schema-spec-packages.md`

## What the register is for

The public-contract publication matrix records *whether* each M5 artifact family has published its contract forms, and the contract catalog is the consuming index that joins each family to its lifecycle label and sample gallery. This register is the *enforcement* layer on top of both: per family it evaluates one CI gate per contract-package class, binds the family to the build identity and artifact-graph node that proves the contract set it shipped, and emits a shiproom blocker decision.

It reuses the matrix's gap-reason and remediation vocabulary and the release-candidate freshness states rather than inventing a new red/yellow contract-health vocabulary, and the mirror/offline publishability of a family follows the same gate outputs so sovereign and self-hosted trains are not second-class citizens.

## What shipped

- A checked-in contract-health register over all 16 published M5 contract families (8 release-blocking), each bound to its CI gates, its build-identity and artifact-graph linkage, and a shiproom blocker decision.
- The five CI gates (80 per-family evaluations) under `ci/contracts/m5-contract-gates/`, plus a gate manifest that carries the promotion decision.
- The shiproom blocker dashboard, the Help-center page, the boundary schema, validator, regenerator, and a typed Rust consumer with an in-product CLI inspect surface.

## Current decision

The contract-health promotion decision is **hold**.

Held by: `task_event_envelope` (failing gate kinds: `compatibility_report`). The matrix narrows these families below the launch cutline, and this register holds promotion on the same signal.

## In-product inspect surface

The typed consumer ships a headless inspect bin that prints the register, a per-family inspect view, the shiproom blocker projection, and the gate manifest, with no live service:

```sh
cargo run -q -p aureline-release --bin aureline_release_implement_contract_ci_gates_release_artifact_graph_linkage_and_shiproom_blockers_for_stale_missing_or_incompatible_m5_schema_spec_packages -- inspect task_event_envelope
```
