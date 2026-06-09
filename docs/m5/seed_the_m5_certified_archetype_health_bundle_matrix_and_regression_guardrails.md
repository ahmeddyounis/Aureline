# M5 Certified-Archetype Health-Bundle Matrix and Regression Guardrails

## Purpose

This document describes the M5 certified-archetype health-bundle matrix and regression guardrails that govern how certified archetypes maintain their health status and how regressions trigger automatic narrowing of claims.

## Artifact

The canonical matrix is checked in at:

```
artifacts/release/m5/seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails.json
```

It is embedded in the `aureline-release` crate so that CI gates, support exports, and docs surfaces all consume the same record without requiring a cargo build in CI.

## Model

Each row in the matrix represents one certified archetype and carries:

- **Health bundle** — a collection of health indicators with states (green, yellow, red, missing)
- **Bundle state** — the overall state earned by the archetype (healthy, degraded, regressed, missing, on_waiver)
- **Gap reasons** — active reasons that may trigger regression guardrails
- **Published label** — the effective lifecycle label after narrowing

## Archetypes

The matrix covers all M5 depth-lane archetypes:

| Archetype | Release-blocking | Default bundle state |
|-----------|------------------|----------------------|
| Notebook | No | Regressed |
| DataRich | No | Regressed |
| AiAdjacent | Yes | On waiver |
| Framework | Yes | Healthy |
| Review | No | Degraded |
| Companion | Yes | Regressed |
| ManagedDepth | No | Missing |

## Regression guardrails

Guardrail rules fire when gap reasons are present on rows whose claim is at or above the stable cutline. Blocking rules prevent publication. Example guardrails:

- `health_indicator_red` → RecaptureBenchmarks (blocks publication)
- `health_bundle_missing` → RefreshHealthBundle (blocks publication)
- `benchmark_regression_detected` → RecaptureBenchmarks (blocks publication)
- `owner_signoff_missing` → RequestOwnerSignoff (blocks publication)

## Downgrade behavior

A row whose bundle state forces narrowing must publish a label below the cutline. A row that holds its label must have:

- owner sign-off
- no active gap reasons
- a proof packet within its freshness SLO

## Integration touchpoints

- `crates/aureline-release` — typed consumer and CI gate
- `crates/aureline-help` / `crates/aureline-cli` — support-export and inspection surfaces
- `docs/m5` / `docs/help` — canonical documentation
- `artifacts/release` / `artifacts/compat` / `artifacts/benchmarks` — downstream evidence
- `fixtures/release` / `schemas/governance` — validation and schema governance
