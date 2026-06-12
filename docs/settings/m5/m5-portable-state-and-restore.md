# Portable-State Export/Import and Restore Provenance

**Doc ref:** `docs/settings/m5/m5-portable-state-and-restore.md`  
**Contract ref:** `settings:m5_portable_state_and_restore:v1`  
**Schema version:** 1

## Overview

This document defines the shared product truth for the portability of M5-owned
artifacts. The canonical record is `M5PortableStateRestoreCertification` in
`crates/aureline-settings/src/m5_portable_state_and_restore/`. Desktop settings
export/import, CLI/headless inspect, support export, help/docs, and admin-docs
surfaces consume the same record before any export, import, restore, or reopen
of an M5 package.

The record makes portability honest: it explains what each M5 artifact class is
(portable, redacted, restore-only, or machine-local), what fidelity a restored
card actually carries (a schema-migration label), and which live dependency,
extension, or remote target is missing and therefore shown as a visible
placeholder rather than silently dropped.

## Portable artifact classes

Every package classifies all six M5-owned artifact classes:

- `selected_settings`
- `profiles`
- `manifests`
- `bundle_selections`
- `docs_packs`
- `evidence_references`

Each class carries a disposition:

- `portable` — carried in full; the class round-trips through export/import.
- `redacted` — carried as reference or metadata only; sensitive bodies are
  stripped. Requires an exclusion reason.
- `restore_only` — re-derivable on import (for example, bundle selections are
  re-resolved; binaries stay machine-local).
- `machine_local` — never serialized; remains on the originating machine.
  Requires an exclusion reason.

Exclusion reasons name why a class is redacted or held local: `secret_material`,
`live_authority_handle`, `machine_unique_trust_anchor`, or
`volatile_machine_state`. The first three forbid raw serialization entirely.

## Restore-provenance cards and schema-migration labels

Every restore card carries an honest migration label, ordered by fidelity:

- `exact` — same schema, all dependencies present; restores exactly.
- `compatible` — a compatible schema migration was applied.
- `layout_only` — only the structure restores; some values could not map.
- `recovered_drafts` — content is recovered as editable drafts, not authority.
- `evidence_only` — only evidence/reference pointers are restored.

The package's `effective_fidelity_ceiling` is the weakest label across its cards.

## Missing-dependency placeholders

When a dependency is unavailable on import or reopen, the affected surface is
kept as a visible placeholder rather than dropped. Placeholder kinds:

- `missing_extension`
- `missing_remote_target`
- `unsupported_client`

Each placeholder records the affected artifact class, a placeholder ref rendered
in the restored layout, `visible_in_layout` / `silently_dropped` flags, and a
recovery hint.

## The fail-closed gate

`M5PortableStateRestoreCertification::build` enforces the contract before a
record can exist:

- Every required artifact class must be classified exactly once.
- A class carried `portable` may not declare a serialization-forbidden exclusion
  reason (no secrets, live handles, or machine-unique anchors leave as portable).
- A card may not claim `exact` fidelity while carrying a missing dependency or
  across a source/target schema-version mismatch.
- Overwriting restores must be previewable and rollback-checkpointed.

The derived `fidelity_qualification.claim_class` is `exact_restore` only when
every structural pillar holds and the ceiling is `exact`; `degraded_restore`
when the restore is sound but below exact fidelity; and `unsupported` when a
structural pillar fails.

## Corpus

The deterministic corpus (`portable_state_restore_corpus`) pins one exact
baseline and five degraded restores, including four downgrade drills:

- `exact_local_restore` — exact, qualifies.
- `cross_version_compatible` — compatible migration.
- `stale_package_drill` — stale package, layout-only with sidecar.
- `lossy_mapping_drill` — cross-platform loss recovered as drafts.
- `redacted_export_drill` — evidence-only restore.
- `unsupported_client_drill` — missing-extension, missing-remote-target, and
  unsupported-client placeholders stay visible.

## Canonical paths

- Typed model: `crates/aureline-settings/src/m5_portable_state_and_restore/`
- Schema: `schemas/settings/m5/m5-portable-state-and-restore.schema.json`
- Fixtures: `fixtures/settings/m5/m5-portable-state-and-restore/`
- Artifact: `artifacts/settings/m5/m5-portable-state-and-restore.md`
- Emitter: `aureline_settings_m5_portable_state_and_restore`

## Guardrails

The record carries typed states and opaque refs only — no secrets, live
authority tickets, machine-unique trust anchors, raw provider payloads, or
workspace contents.
