# Portable-State Export/Import Review Sheets

**Artifact ref:** `artifacts/settings/m5/m5-portable-state-review.md`  
**Contract ref:** `settings:m5_portable_state_review:v1`  
**Schema version:** 1  
**As of:** 2026-06-16

## Purpose

This artifact certifies that portable-state export and import are reviewable and
provenance-aware before any commit: a review sheet says which selected artifact
classes leave the machine and under which data-class label, what is redacted and
how, what build produced the package, what integrity and signature evidence is
attached, and — for an import — how the package compares against current state
before rehydration. Machine-local exclusions and redacted classes stay visible
rather than being silently dropped, and the review is never reduced to a
file-size or timestamp summary when the package materially changes restore.

## Certification Scope

The canonical record binds:

1. A review `direction` (`export` or `import`) coupled to the compare-before-
   restore summary: required for imports, forbidden for exports.
2. Per-class rows carrying a data-class label (`local_only`, `portable`,
   `shared`, `redacted`, `machine_local`), an exclusion reason where excluded, an
   estimated size, a checksum state, a signature state, a content ref, and a
   `visible_in_review` flag.
3. A redaction manifest binding each redacted (or withheld machine-local) class
   to a technique (`secret_omission`, `handle_omission`, `path_redaction`,
   `host_redaction`, `reference_only`), a matching reason, a redacted-field
   count, and a detail.
4. Producer/build provenance — build label, short commit, channel, dirty flag,
   package and target schema versions, platform, and a redacted `host_class`.
5. A compare-before-restore summary for imports — pane/surface deltas, missing
   dependency classes, excluded secrets/handles, and path/host redaction counts,
   with a fidelity ceiling.
6. Surface-parity rows proving export review, import review, diagnostics, and
   support packets render the same labels and provenance.
7. A fail-closed gate: a record cannot be built that carries secrets as
   portable/shared, hides an exclusion, omits a redaction-manifest entry for a
   redacted class, or omits an import comparison.

## Canonical Paths

- Typed model: `crates/aureline-settings/src/m5_portable_state_review/`
- Schema: `schemas/settings/m5/m5-portable-state-review.schema.json`
- Fixtures: `fixtures/settings/m5/m5-portable-state-review/`
- Companion doc: `docs/settings/m5/m5-portable-state-review.md`
- Emitter: `aureline_settings_m5_portable_state_review`

## Corpus Outcomes

| Scenario | Direction | Readiness |
| --- | --- | --- |
| `export_clean_review` | `export` | `reviewable` |
| `import_exact_review` | `import` | `reviewable` |
| `redacted_export_review` | `export` | `reviewable` |
| `lossy_import_review` | `import` | `review_required` |
| `foreign_machine_import_review` | `import` | `review_required` |
| `stale_schema_import_review` | `import` | `review_required` |

## Guardrails

The record carries typed states, redacted host classes, and opaque refs only. No
secrets, live authority tickets, machine-unique trust anchors, raw provider
payloads, raw hostnames, or workspace contents are serialized into a review sheet
or the package it describes.
