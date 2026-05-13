# Representation Copy/Export Review Packet

Purpose: canonical review evidence for the alpha copy/export validation lane
covering diff, review, search, and package/install review surfaces.

## Canonical Sources

- Implementation:
  `crates/aureline-content-safety/src/representation_copy_export.rs`
- CLI validator:
  `crates/aureline-content-safety/src/bin/representation_copy_export_alpha.rs`
- Protected fixture:
  `fixtures/content_safety/representation_copy_export_alpha/protected_cross_surface_copy_export.json`
- Security contract:
  `docs/security/representation_copy_export_alpha.md`
- Shared interaction contract:
  `schemas/ux/interaction_safety.schema.json`

## Acceptance Evidence

| Acceptance check | Evidence |
|---|---|
| Copy/export actions preserve raw versus rendered labels | Fixture actions carry `representation_class`, `payload_mode`, and `label_class`; validator rejects rendered/context copies without explicit label classes. |
| Unsafe or ambiguous surfaces offer inspect or reveal paths before export | Diff, review, search, and package rows all carry `inspect_or_reveal_paths`; sensitive search/package actions require preview before clipboard commit. |
| Review packet lists remaining mismatches by surface | Current validator report has `status = passed` and `violations = []`. |
| Raw/plain copy is distinct from richer/context export | Each surface has exactly one raw/plain safe default; rendered/context/export-packet actions are separate non-default actions. |
| Shared interaction-safety contract is consumed | Every action mints a `copy_export_representation_record` with schema version, surface class, target ref, representation class, redaction class, and policy context. |
| Cross-surface reconciliation does not scrape UI text | Reconciliation groups compare structured fields across diff/review and search/package rows. |

## Surface Mismatch Table

| Surface | Remaining mismatch | Notes |
|---|---|---|
| Diff | None | Plain/raw default, escaped copy, context copy, inspect/reveal, and reopen closed diff are present. |
| Review | None | Plain anchor copy, context copy, sanitized export, anchor provenance, and reopen review are present. |
| Search | None | Plain result identity default avoids private path copy; context and metadata export require preview. |
| Package/install review | None | Plain coordinate default avoids sensitive handles; export packet and metadata-only export require preview. |

## Validation Output

```json
{
  "record_kind": "representation_copy_export_validation_report",
  "schema_version": 1,
  "case_id": "case:representation-copy-export:protected-cross-surface",
  "status": "passed",
  "violations": [],
  "reconciled_groups": [
    "reconcile:diff-review-plain-copy",
    "reconcile:search-package-safe-defaults"
  ],
  "validated_surface_count": 4
}
```

## Verification

```sh
python3 -m json.tool fixtures/content_safety/representation_copy_export_alpha/protected_cross_surface_copy_export.json >/dev/null
cargo test -p aureline-content-safety --test representation_copy_export_alpha
cargo test -p aureline-content-safety representation_copy_export
cargo run -q -p aureline-content-safety --bin representation_copy_export_alpha -- fixtures/content_safety/representation_copy_export_alpha/protected_cross_surface_copy_export.json
```

## Follow-ups

- Extend live search and package surfaces to emit the same packet shape when
  those UI lanes move from fixture-backed alpha proof to runtime wiring.
- Add generated-content citation-anchor coverage when generated review
  summaries become part of the protected copy/export lane.
