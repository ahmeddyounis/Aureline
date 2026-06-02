# Finalize Support Center and Diagnostics Center surfaces: performance inspector, language-service dashboard, index-health view, and AI evidence inspector

## Artifact summary

This artifact is the checked-in human-readable summary for the finalized Support Center and Diagnostics Center surfaces. It accompanies the boundary schema and the protected fixture corpus.

## Checked-in files

| Path | Purpose |
|------|---------|
| `schemas/support/finalize_support_center_surfaces_performance_inspector_language_service.schema.json` | Boundary schema |
| `crates/aureline-support/src/finalize_support_center_surfaces_performance_inspector_language_service/mod.rs` | Crate implementation |
| `fixtures/support/m4/finalize_support_center_surfaces_performance_inspector_language_service/` | Protected fixture corpus |
| `docs/support/m4/finalize_support_center_surfaces_performance_inspector_language_service.md` | Reviewer contract doc |

## Health-feed object model

Each `health_feed_item_record` names:
- `service_family` — one of performance, language_service, index, ai_evidence, project_doctor, support_bundle, repair_transaction, crash_evidence
- `boundary_class` — local_only, remote_managed, offline_local, cli_headless
- `affected_workflows_summary` — reviewer-facing sentence
- `last_checked_at` — ISO 8601 timestamp
- `freshness_state` — fresh, stale, unknown, partial
- `diagnostics_actions` — at least one action from the closed vocabulary
- `health_state` — healthy, degraded, unavailable, quarantined
- `local_only_continuity_note_required` — true when a partial outage requires a continuity note

## Inspector surfaces

### Performance inspector
- Publishes p50 and p95 latency budgets in milliseconds.
- Carries observed p50 and p95 latencies.
- Includes benchmark-lab trace refs and corpus metadata ref.
- Waiver hooks name thresholds that are intentionally tightened or narrowed.

### Language-service dashboard
- Carries router decision refs.
- Lists provider availability rows with health state, freshness, scope claim, and restart strikes.
- Surfaces quarantine refs when a provider is crash-loop quarantined.

### Index-health view
- Surfaces index freshness, coverage percentage, and corruption-check result.
- Names last full index timestamp.

### AI evidence inspector
- Carries evidence packet refs.
- Declares redaction class and replay posture.
- Explicitly excludes raw prompts and raw provider payloads.

## Diagnostics center record

The top-level `diagnostics_center_record` joins:
- All health-feed items
- The four inspector records
- Project Doctor finding refs
- Support-bundle preview ref
- Repair transaction refs
- Crash evidence ref
- Exact-build identity ref
- Outage scope
- Recovery-ladder hooks

## Export safety

- `raw_private_material_excluded` is always true.
- `ambient_authority_excluded` is always true.
- AI evidence inspector excludes raw prompt bodies and provider payloads.
- Support packets are metadata-only by default.

## Stable claim

This lane claims Stable when:
- All eight service families are represented in the health feed.
- All four inspector classes are present and validated.
- Partial-service outages preserve explicit healthy subsystems.
- Every health-feed item carries at least one diagnostics action.
- Export packets are redacted-by-default and exact-build.
