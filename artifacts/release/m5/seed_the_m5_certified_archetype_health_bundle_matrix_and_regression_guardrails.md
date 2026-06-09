# M5 Certified-Archetype Health-Bundle Matrix and Regression Guardrails

This artifact seeds the canonical health-bundle matrix for every M5 certified archetype and the regression guardrails that narrow a certified claim when health indicators regress.

## Checked-in artifact

- `artifacts/release/m5/seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails.json`

## Schema

- `schemas/governance/seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails.schema.json`

## Typed consumer

- `crates/aureline-release/src/seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails/mod.rs`

## Overview

The matrix binds each M5 certified archetype to a health bundle composed of required health indicators. Each row carries:

- a `HealthBundle` with indicator states (green, yellow, red, missing)
- a `bundle_state` (healthy, degraded, regressed, missing, on_waiver)
- active `gap_reasons` that trigger regression guardrails
- a `published_label` after narrowing

The `RegressionGuardrailRule` set defines closed conditions that gate publication. When a blocking rule fires, the publication decision is `hold`.

## Archetypes covered

- Notebook
- DataRich
- AiAdjacent
- Framework
- Review
- Companion
- ManagedDepth

## Health indicator kinds

- UnitTestPassRate
- IntegrationTestPassRate
- BenchmarkRegression
- CompatibilitySurfaceCoverage
- DependencyFreshness
- SecurityScanClean
- AccessibilitySignoffCoverage

## Guardrail actions

- HoldPublication
- NarrowLabel
- RefreshHealthBundle
- RequestOwnerSignoff
- RecaptureBenchmarks
- ReRunSecurityScan
