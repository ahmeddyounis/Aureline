# Explain-plan freshness notes, engine-version context, and plan-comparison flows

## Scope

This document describes the canonical M5 qualification packet for explain-plan freshness notes, engine-version context, and plan-comparison flows in Aureline.

## Truth sources

- Implementation: `crates/aureline-api/src/implement_explain_plan_freshness_notes_engine_version_context_and_plan_comparison_flows/mod.rs`
- Schema: `schemas/data/implement-explain-plan-freshness-notes-engine-version-context-and-plan-comparison-flows.schema.json`
- Checked-in packet: `artifacts/data/m5/implement-explain-plan-freshness-notes-engine-version-context-and-plan-comparison-flows.json`
- Fixtures: `fixtures/data/m5/implement_explain_plan_freshness_notes_engine_version_context_and_plan_comparison_flows/`

## Surface claims

| Surface | Claim | Displayed | Rationale |
|---|---|---|---|
| Explain-plan freshness note | stable | stable | Shows engine version, plan mode, freshness state, and stale labeling. |
| Engine-version context | stable | stable | Shows engine family, version ref, mismatch visibility, and plan/comparison visibility. |
| Plan-comparison flow | stable | stable | Shows comparison basis, diff visibility, rollback recommendation, and downgrade on mismatch. |

## Downgrade rules

- All promoted surfaces have `downgrade_if_missing: true`.
- Missing proof on a stable claim narrows the surface to `preview` instead of inheriting a generic label.
- Plan-comparison flows enforce `downgrade_on_mismatch` so divergent or inconclusive comparisons do not masquerade as authoritative.

## Redaction and privacy

- Explain-plan surfaces do not expose raw plan payloads, raw hostnames, or raw connection strings.
- Engine-version context uses opaque refs rather than exposing full version strings or build identifiers.
- Stale imported plans are visibly labeled and do not masquerade as live truth.
- Plan-comparison flows disclose the comparison basis, diff visibility, and rollback recommendation before any downstream action.

## Verification

Run `cargo check -p aureline-api` to verify the embedded packet deserializes and validates.
