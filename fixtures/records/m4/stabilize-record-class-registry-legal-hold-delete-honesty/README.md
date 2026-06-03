# Fixtures — record-class registry: hold/delete honesty and chronology

These fixtures document the stable output of the record-class registry stabilization
for hold/delete honesty states, timezone-aware chronology, and collaboration record
classes.

## Files

| File | Purpose |
|---|---|
| `summary.json` | Flat summary counts: row count, vocabulary counts, scope vocabulary. |
| `page.json` | Full record with outcome vocabulary, new registry rows, and defects (empty). |
| `support_export.json` | Example export bundle manifest, destruction receipt, hold evaluations, and chronology export illustrating outcome vocabulary in action. |
| `defects.json` | Empty array — no defects in this stable lane. |

## Outcome vocabulary (10 tokens)

`requested`, `queued`, `blocked_by_hold`, `completed`, `policy_retained`,
`outside_platform_scope`, `manual_local_capture_required`, `partial`,
`not_found`, `omitted_by_redaction`

## Key properties illustrated

- **Hold fail-closed**: both `active` and `unknown_indeterminate` set `blocks_destructive_action: true`.
- **Hold no new rights**: `grants_new_read_rights` and `grants_new_export_rights` are always `false`.
- **Redaction honesty**: `omitted_by_redaction` is distinct from `not_found`; transcript content is omitted via `full_content_omit` profile.
- **Local-only honesty**: `local_only_artifact_note` is set when local artifacts are excluded from the bundle.
- **Destruction receipt durable**: receipt carries separate ref buckets and `hash_checksum_manifest`.
- **Chronology timezone-aware**: entries carry both `timestamp_utc` and `source_timezone_label`.

## Truth source

- Rust types: `crates/aureline-records/src/stabilize_record_class_registry_legal_hold_delete_honesty/mod.rs`
- Registry: `artifacts/governance/record_class_registry_alpha.yaml`
- Schema: `schemas/records/record-class-registry.schema.json`
