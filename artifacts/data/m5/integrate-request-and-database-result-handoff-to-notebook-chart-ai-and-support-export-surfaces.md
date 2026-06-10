# M5 Request and Database Result Handoff to Notebook, Chart, AI, and Support-Export Surfaces Artifact Companion

This file is the artifact-level companion document for the checked-in M5 handoff qualification packet.

- **Canonical JSON**: `artifacts/data/m5/integrate-request-and-database-result-handoff-to-notebook-chart-ai-and-support-export-surfaces.json`
- **Schema**: `schemas/data/integrate-request-and-database-result-handoff-to-notebook-chart-ai-and-support-export-surfaces.schema.json`
- **Typed consumer**: `crates/aureline-api/src/integrate_request_and_database_result_handoff_to_notebook_chart_ai_and_support_export_surfaces/mod.rs`

The packet is the single source of truth for M5 handoff depth lane qualification. All downstream surfaces ingest it directly.

## Object Coverage

| Object | Current proof |
| --- | --- |
| Notebook handoff | Two rows cover admitted dataframe-typed handoff with target-ref requirement, and policy-blocked handoff with truncation disclosure. |
| Chart handoff | Two rows cover admitted typed handoff with provenance preservation, and policy-blocked handoff with truncation disclosure. |
| AI handoff | Two rows cover admitted typed handoff with context and secret-boundary enforcement, and secret-boundary-violation blocked handoff. |
| Support export | Two rows cover metadata-safe default export with consent requirements, and internal-support restricted export with provenance preservation. |

## Surface Qualification

| Surface | Claim | Displayed | Rationale |
| --- | --- | --- | --- |
| Notebook handoff | Stable | Stable | Typed result-set object model, truncation disclosure, provenance chip, and target-ref requirement for admitted transfers. |
| Chart handoff | Stable | Stable | Typed result-set object model, typed column preservation, truncation disclosure, and provenance chip. |
| AI handoff | Stable | Stable | Typed result-set object model, secret-boundary enforcement, truncation disclosure, and provenance without credential leakage. |
| Support export | Stable | Stable | Metadata-only default, explicit consent for row data and credentials, truncation disclosure, and provenance preservation. |

## Guardrails

- Raw row bodies, raw cell values, raw credentials, raw connection-string fragments, and raw secret material do not appear in this packet.
- Notebook, chart, and AI handoffs use the canonical typed result-set object model instead of ad-hoc clipboard scraping.
- Truncation state is disclosed on every handoff and export so partial results are never silently complete.
- Provenance chips are preserved on every handoff and export for downstream traceability.
- AI handoff enforces secret boundaries; credentials and secrets never cross into AI context without explicit policy override.
- Support export defaults to metadata-only; row data and credentials require explicit user consent.
- Lossy type coercion is restricted to textual fallback formats with explicit user choice.

## Known Limits

This packet qualifies notebook, chart, AI, and support-export handoff surfaces for promoted M5 surfaces. It does not ship a live handoff runner, and imported handoff artifacts stay inspect-only unless they carry recoverable result-set lineage and provenance.
