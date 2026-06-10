# Request and database result handoff to notebook, chart, AI, and support-export surfaces

## Scope

This document describes the canonical M5 qualification packet for integrating request and database result handoff to notebook, chart, AI, and support-export surfaces in Aureline.

## Truth sources

- Implementation: `crates/aureline-api/src/integrate_request_and_database_result_handoff_to_notebook_chart_ai_and_support_export_surfaces/mod.rs`
- Schema: `schemas/data/integrate-request-and-database-result-handoff-to-notebook-chart-ai-and-support-export-surfaces.schema.json`
- Checked-in packet: `artifacts/data/m5/integrate-request-and-database-result-handoff-to-notebook-chart-ai-and-support-export-surfaces.json`
- Fixtures: `fixtures/data/m5/integrate_request_and_database_result_handoff_to_notebook_chart_ai_and_support_export_surfaces/`

## Surface claims

| Surface | Claim | Displayed | Rationale |
|---|---|---|---|
| Notebook handoff | stable | stable | Reuses the typed result-set object model, discloses truncation, preserves provenance, and requires a target ref for admitted transfers. |
| Chart handoff | stable | stable | Reuses the typed result-set object model, preserves typed columns, discloses truncation, and carries provenance chips. |
| AI handoff | stable | stable | Reuses the typed result-set object model, enforces secret boundaries, discloses truncation, and preserves provenance without leaking credentials. |
| Support export | stable | stable | Defaults to metadata-only, requires explicit consent for row data or credentials, discloses truncation, and preserves provenance. |

## Downgrade rules

- All promoted surfaces have `downgrade_if_missing: true`.
- Missing proof on a stable claim narrows the surface to `preview` instead of inheriting a generic label.

## Redaction and privacy

- Notebook, chart, and AI handoffs use the same typed result-set object model rather than ad-hoc clipboard scraping.
- All handoff paths disclose truncation state so partial results are never silently complete.
- All handoff paths preserve provenance chips for downstream traceability.
- AI handoff enforces secret boundaries; raw credentials and secret material never cross the AI handoff boundary.
- Support export defaults to metadata-only; row data and credentials require explicit user consent.
- Lossy type coercion is restricted to textual fallback formats with explicit user choice.

## Verification

Run `cargo check -p aureline-api` to verify the embedded packet deserializes and validates.
