# Portable-State Export/Import and Restore Provenance

**Artifact ref:** `artifacts/settings/m5/m5-portable-state-and-restore.md`  
**Contract ref:** `settings:m5_portable_state_and_restore:v1`  
**Schema version:** 1  
**As of:** 2026-06-12

## Purpose

This artifact certifies that portable-state export/import and restore claims for
M5-owned artifacts are explicit, honestly labeled, and dependency-aware: a
package says what is portable, redacted, machine-local, or restore-only; a
restore card never claims exact fidelity it cannot back; and a missing extension,
remote target, or unsupported client stays visible as a placeholder instead of
silently dropping the affected surface.

## Certification Scope

The canonical record binds:

1. Six M5 artifact classes — `selected_settings`, `profiles`, `manifests`,
   `bundle_selections`, `docs_packs`, and `evidence_references` — each with a
   portability disposition and an exclusion reason where redacted or local.
2. Restore-provenance cards carrying schema-migration labels (`exact`,
   `compatible`, `layout_only`, `recovered_drafts`, `evidence_only`), source and
   target schema versions, an integrity ref, a rollback checkpoint, and an
   optional unmappable-values sidecar.
3. Missing-dependency placeholders (`missing_extension`, `missing_remote_target`,
   `unsupported_client`) that remain visible in restored layouts and packets.
4. Surface-parity rows proving desktop, CLI, support export, help/docs, and
   admin docs render the same package and provenance truth.
5. A fail-closed fidelity gate: a record cannot be built that carries secrets as
   portable, claims exact restore with a missing dependency, or claims exact
   restore across a schema-version mismatch.

## Canonical Paths

- Typed model: `crates/aureline-settings/src/m5_portable_state_and_restore/`
- Schema: `schemas/settings/m5/m5-portable-state-and-restore.schema.json`
- Fixtures: `fixtures/settings/m5/m5-portable-state-and-restore/`
- Companion doc: `docs/settings/m5/m5-portable-state-and-restore.md`
- Emitter: `aureline_settings_m5_portable_state_and_restore`

## Corpus Outcomes

| Scenario | Claim | Fidelity ceiling |
| --- | --- | --- |
| `exact_local_restore` | `exact_restore` | `exact` |
| `cross_version_compatible` | `degraded_restore` | `compatible` |
| `stale_package_drill` | `degraded_restore` | `layout_only` |
| `lossy_mapping_drill` | `degraded_restore` | `recovered_drafts` |
| `redacted_export_drill` | `degraded_restore` | `evidence_only` |
| `unsupported_client_drill` | `degraded_restore` | `layout_only` |

## Guardrails

The record carries typed states and opaque refs only. No secrets, live authority
tickets, machine-unique trust anchors, raw provider payloads, or workspace
contents are serialized into portable-state packages.
