# M5 portable-state export/import and restore fixtures

Fixture corpus for the `m5_portable_state_and_restore` record. These fixtures pin
the seeded package-class table, restore-provenance cards, schema-migration
labels, and missing-dependency placeholders so a change to the typed model, the
fail-closed gate, or the records is caught against frozen evidence.

- Typed model: `crates/aureline-settings/src/m5_portable_state_and_restore/`
- Schema: `schemas/settings/m5/m5-portable-state-and-restore.schema.json`
- Companion doc: `docs/settings/m5/m5-portable-state-and-restore.md`
- Emitter: `aureline_settings_m5_portable_state_and_restore`

## Files

- `corpus_manifest.json` — indexes the scenarios and what each proves (claim
  class and fidelity ceiling).
- `exact-local-restore.json` — same-machine, same-schema restore; every card is
  `exact` and the package qualifies for an exact-restore claim.
- `cross-version-compatible.json` — an older-schema package restores under a
  compatible migration and is labeled `compatible`, not `exact`.
- `stale-package-drill.json` — a stale package restores `layout_only` with an
  unmappable-values sidecar.
- `lossy-mapping-drill.json` — a cross-platform import recovers affected content
  as editable drafts (`recovered_drafts`).
- `redacted-export-drill.json` — a redacted export restores `evidence_only`.
- `unsupported-client-drill.json` — `missing_extension`, `missing_remote_target`,
  and `unsupported_client` placeholders stay visible in the restored layout.

## What the corpus proves

- **Portability is explicit.** Every package classifies all six M5 artifact
  classes as portable, redacted, restore-only, or machine-local, with exclusion
  reasons for redacted and local classes.
- **Restore fidelity is honest.** No card claims `exact` while carrying a missing
  dependency or across a schema-version mismatch; the package publishes the
  weakest label across its cards as its fidelity ceiling.
- **Missing dependencies stay visible.** Every placeholder is `visible_in_layout`
  and never `silently_dropped`.
- **Restores are reversible.** Overwriting restores are previewable and carry a
  rollback checkpoint.
- **Surfaces agree.** Desktop, CLI, support export, help/docs, and admin docs all
  consume the same record.

The fixtures carry typed states and opaque refs only — no secrets, live authority
tickets, machine-unique trust anchors, raw provider payloads, or workspace
contents.
