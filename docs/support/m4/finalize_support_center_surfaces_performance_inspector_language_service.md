# Finalize Support Center and Diagnostics Center surfaces: performance inspector, language-service dashboard, index-health view, and AI evidence inspector

## Purpose

This document defines the stable contract for the Support Center and Diagnostics Center surfaces that consolidate per-subsystem health into one shared health-feed object model. The same model drives support-center cards, diagnostics-center summaries, and support/export packets identically across desktop and CLI/headless surfaces.

## Scope

- Performance inspector with published p50/p95 budgets, benchmark-lab traces, corpus metadata, and waiver hooks.
- Language-service dashboard with router decisions, provider availability rows, and quarantine state.
- Index-health view with freshness, coverage, and corruption-check state.
- AI evidence inspector with evidence packet refs, redaction posture, and replay lineage.
- Shared health-feed item model naming service family, boundary class, affected workflows, last-checked time, freshness state, and diagnostics actions.
- Diagnostics Center as the durable escalation surface linking Project Doctor findings, support-bundle preview, repair transactions, exact-build crash evidence, and per-subsystem inspectors.
- Partial-service outage handling that keeps unaffected subsystems explicitly healthy and preserves a visible local-only continuity note.

## Contract

The canonical boundary schema is at:

```
schemas/support/finalize_support_center_surfaces_performance_inspector_language_service.schema.json
```

The crate implementation is in:

```
crates/aureline-support/src/finalize_support_center_surfaces_performance_inspector_language_service/mod.rs
```

## Posture

- Project Doctor remains read-only by default.
- Safe mode stays bounded.
- Repairs are previewable before apply.
- Crash and support exports are exact-build and redacted-by-default.
- No surface silently mutates scope or asks users to wipe durable state.

## Integration touchpoints

- `crates/aureline-support` — owns the health-feed model, inspector records, diagnostics center record, support-center card projection, and export packet.
- `crates/aureline-doctor` — Project Doctor finding refs consumed by the diagnostics center record.
- `crates/aureline-crash` — crash evidence and incident trail refs consumed by the diagnostics center record.
- `schemas/support` — boundary schema for the diagnostics center, health feed, support cards, and export packet.

## Seeded scenarios

The protected fixture corpus under `fixtures/support/m4/finalize_support_center_surfaces_performance_inspector_language_service/` covers:

1. Nominal operation — all subsystems healthy.
2. Partial-service outage — one subsystem degraded, others explicitly healthy with continuity note.
3. Language-service quarantine — provider crash-loop with quarantine ref.
4. Index stale — coverage drop with freshness stale.
5. AI evidence redacted — raw prompts and provider payloads excluded.
6. Performance budget breach — p95 exceeds published budget with waiver hook.

## Acceptance criteria

- The health-feed object model drives support-center cards, diagnostics summaries, and export packets identically across desktop and CLI/headless surfaces.
- Partial-service outages keep unaffected subsystems explicitly healthy and preserve a visible local-only continuity note instead of flattening the whole product to a generic unavailable state.
- Blocked-user scenarios show accurate diagnosis, narrow repair, and redacted exact-build evidence without hidden resets.
- Any surface still lacking stable qualification is automatically narrowed below Stable in product copy, docs/help, and release packets.
