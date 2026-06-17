# M5 portable-state export/import review fixtures

Fixture corpus for the `m5_portable_state_review` record. These fixtures pin the
review shown before an M5-owned portable-state package is exported or before its
contents are restored: the per-class data-class labels, redaction manifest,
producer/build provenance, integrity and signature states, and the
compare-before-restore summary. A change to the typed model, the fail-closed
gate, or the records is caught against frozen evidence.

- Typed model: `crates/aureline-settings/src/m5_portable_state_review/`
- Schema: `schemas/settings/m5/m5-portable-state-review.schema.json`
- Companion doc: `docs/settings/m5/m5-portable-state-review.md`
- Emitter: `aureline_settings_m5_portable_state_review`

## Files

- `corpus_manifest.json` — indexes the scenarios, their direction, and the
  readiness each proves.
- `export-clean-review.json` — an export review listing every selected class
  with its label, redaction status, machine-local exclusion, estimated size,
  integrity state, and producer build; `reviewable`.
- `import-exact-review.json` — an import review of an exact, same-schema package
  whose comparison shows no material change; `reviewable`.
- `redacted-export-review.json` — an export exercising every redaction technique
  (secret omission, handle omission, path redaction, host redaction) plus a
  machine-local trust anchor held back; `reviewable`.
- `lossy-import-review.json` — an import whose comparison shows added, removed,
  and changed panes and surfaces with a recovered-drafts fidelity ceiling;
  `review_required`.
- `foreign-machine-import-review.json` — an import from a foreign machine with
  path/host redaction, a withheld machine-unique trust anchor, a missing remote
  target, and an untrusted signature; `review_required`.
- `stale-schema-import-review.json` — an import of a package written under an
  older schema; the comparison is `compatible` and the schema-version mismatch
  requires review; `review_required`.

## What the corpus proves

- **Reviews are explicit and provenance-aware.** Every sheet records producer
  build/version, schema versions, estimated size, and per-class data-class
  labels (local-only, portable, shared, redacted, machine-local) before any
  export or restore commit.
- **Exclusions stay visible.** Redacted and machine-local classes are always
  `visible_in_review`; redacted classes always carry a matching redaction-
  manifest entry naming what was stripped and how.
- **The secret boundary holds.** No class whose body is secret material, a live
  authority handle, or a machine-unique trust anchor crosses as `portable` or
  `shared`.
- **Imports compare before restore.** Every import review carries a
  compare-before-restore summary of pane/surface deltas, missing dependency
  classes, excluded secrets/handles, and path/host redaction, and the readiness
  is `review_required` whenever the comparison materially changes restore.
- **Surfaces agree.** Export review, import review, diagnostics, and support
  packets all consume the same record and show the same labels.

The fixtures carry typed states, redacted host classes, and opaque refs only —
no secrets, live authority tickets, machine-unique trust anchors, raw provider
payloads, raw hostnames, or workspace contents.
