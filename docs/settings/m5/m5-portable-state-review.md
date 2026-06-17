# Portable-State Export/Import Review Sheets

**Doc ref:** `docs/settings/m5/m5-portable-state-review.md`  
**Contract ref:** `settings:m5_portable_state_review:v1`  
**Schema version:** 1

## Overview

This document defines the shared product truth for the *review* a user or admin
sees before an M5-owned portable-state package is exported or before its contents
are restored. The canonical record is `M5PortableStateReviewSheet` in
`crates/aureline-settings/src/m5_portable_state_review/`. The export-review sheet,
the import-review sheet, the diagnostics packet, and the support packet all
consume the same record so they explain the package the same way.

The review makes export/import safe and explainable for local backup, handoff,
and managed/offline portability: it says which selected artifact classes leave
the machine and under which data-class label, what is redacted and why, what
build produced the package, what integrity and signature evidence is attached,
and — for an import — how the package compares against current state before any
rehydration. It reuses the portability vocabulary from
[`m5_portable_state_and_restore`](./m5-portable-state-and-restore.md) (artifact
classes, exclusion reasons, fidelity labels, missing-dependency kinds) rather
than inventing surface-local language.

## Review direction

A review sheet is `export` or `import`:

- `export` — shown before a package is written. The compare-before-restore
  summary is forbidden (there is nothing to restore against yet).
- `import` — shown before a package is restored. The compare-before-restore
  summary is required.

## Data-class labels

Every selected artifact class carries one explicit data-class label, preserved
across export, import, diagnostics, and support:

- `local_only` — held on this machine only; intentionally excluded.
- `portable` — carried in full; round-trips through export/import.
- `shared` — carried in full and explicitly cleared for cross-user/fleet sharing.
- `redacted` — carried as reference or metadata only; sensitive bodies stripped.
- `machine_local` — never serialized; remains on the originating machine.

`local_only`, `redacted`, and `machine_local` require an exclusion reason
(`secret_material`, `live_authority_handle`, `machine_unique_trust_anchor`, or
`volatile_machine_state`). `portable` and `shared` cross in full and must not
declare an exclusion reason.

Each class row also carries an estimated size, a `checksum_state`
(`verified`/`present`/`mismatch`/`unavailable`), a `signature_state`
(`verified`/`present`/`untrusted`/`unsigned`/`unavailable`), and a
`visible_in_review` flag that must be true for any excluded class.

## Redaction manifest

The redaction manifest names what was stripped and how. Each entry binds an
artifact class to a `technique` — `secret_omission`, `handle_omission`,
`path_redaction`, `host_redaction`, or `reference_only` — a `reason` that must
match the class row's exclusion reason, a redacted-field count, and a detail
string. Every `redacted` class must have a matching manifest entry; entries may
also describe `machine_local` classes whose content is withheld.

## Producer/build provenance

`provenance` records the producing build's label, short commit, channel, and
dirty flag; the package and target schema versions; the platform class; and a
redacted `host_class` (`same_machine`, `managed_fleet`, `foreign_machine`, or
`unknown`). The host is recorded as a class, never as a raw hostname, so the
review can say a package came from a foreign machine without leaking a
machine-unique identity into a portable record.

## Compare-before-restore

For imports, `compare` summarizes the package against current state:

- `pane_delta` / `surface_delta` — added/removed/changed counts.
- `missing_dependency_classes` — `missing_extension`, `missing_remote_target`,
  or `unsupported_client` dependencies that would be missing on restore.
- `excluded_secret_handle_count` and `excluded_exclusion_reasons` — what stays
  behind.
- `path_redaction_count` / `host_redaction_count` — redaction magnitude.
- `fidelity_ceiling` — the weakest restore-fidelity label implied.

A comparison materially changes restore when panes or surfaces change, a
dependency class would be missing, or the fidelity ceiling is below `exact`.
Excluded secrets and redaction counts alone do not make a restore material.

## The fail-closed gate

`M5PortableStateReviewSheet::build` enforces the contract before a record can
exist:

- A `portable`/`shared` class may not declare a serialization-forbidden body
  (no secrets, live handles, or machine-unique anchors cross in full).
- A `local_only`/`redacted`/`machine_local` class must declare an exclusion
  reason and must stay visible — exclusions are never silently dropped.
- A `redacted` class must carry a matching redaction-manifest entry whose reason
  agrees with the row.
- An import review must carry a compare summary; an export review must not.
- Producer provenance fields must be present, and all four consumer surfaces
  (export, import, diagnostics, support) must consume the record.

The derived `qualification.readiness` is:

- `reviewable` — every structural pillar holds and nothing requires explicit
  review.
- `review_required` — sound, but a material restore change, an untrusted
  signature, or a schema-version mismatch (on import) must be reviewed first.
- `blocked` — a structural pillar failed (for example, a crossing class carries a
  checksum `mismatch`, or a surface hides the labels), so the package is not
  safely committable as-is.

## Corpus

The deterministic corpus (`portable_state_review_corpus`) pins six sheets:

- `export_clean_review` — export, `reviewable`.
- `import_exact_review` — import, no material change, `reviewable`.
- `redacted_export_review` — export exercising every redaction technique,
  `reviewable`.
- `lossy_import_review` — import with added/removed/changed panes and surfaces,
  `review_required`.
- `foreign_machine_import_review` — foreign-machine import with path/host
  redaction, a withheld trust anchor, a missing remote target, and an untrusted
  signature, `review_required`.
- `stale_schema_import_review` — import across a schema-version mismatch,
  `review_required`.

## Canonical paths

- Typed model: `crates/aureline-settings/src/m5_portable_state_review/`
- Schema: `schemas/settings/m5/m5-portable-state-review.schema.json`
- Fixtures: `fixtures/settings/m5/m5-portable-state-review/`
- Artifact: `artifacts/settings/m5/m5-portable-state-review.md`
- Emitter: `aureline_settings_m5_portable_state_review`

## Guardrails

The record carries typed states, redacted host classes, and opaque refs only — no
secrets, live authority tickets, machine-unique trust anchors, raw provider
payloads, raw hostnames, or workspace contents. The review is never reduced to a
single file-size or timestamp summary when the package materially changes restore
behavior.
