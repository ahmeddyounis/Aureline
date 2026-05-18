# Refactor Preview And Rollback Corpus

This corpus backs the beta launch-language refactor evidence lane. Rows cover
rename, extract, move, import-update, and cross-file signature-change previews
across warm semantic, partial-index, cached, and remote-assisted conditions.

Each fixture contains one `language_refactor_preview_record` plus the matching
`language_refactor_validation_result_record`. Fixtures carry only opaque refs,
counts, closed vocabulary, and export-safe summaries; raw source, raw diffs,
paths, provider logs, and secrets are intentionally excluded.
