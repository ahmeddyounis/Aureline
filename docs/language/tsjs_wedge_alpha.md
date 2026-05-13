# TS/JS Hover, Navigation, References, and Rename Preview Alpha

This document records the first bounded TypeScript/JavaScript assistance path in
`aureline-language`.

## Owned Runtime Surface

The runtime lives under `crates/aureline-language/src/tsjs/` and exposes
`TsJsLaunchWedge`. The wedge consumes the shared `LspRouter` and emits four
inspectable records:

- `tsjs_hover_record`
- `semantic_result_ref_record` for definition targets
- `tsjs_reference_set_record`
- `rename_preview_record`

Every record includes provider identity, freshness, scope, router decision id,
policy context, redaction posture, and export-safe summaries. Raw source bodies
and provider logs are not embedded in these records.

## Scope And Fallback Rules

- Ready TypeScript language-service results render as compatibility-layer
  answers with `authoritative_live` freshness.
- Partial worksets and sparse scopes remain visible through scope descriptors,
  omitted-scope refs, and non-authoritative rename preview posture.
- Generated or read-only rename candidates stay counted and warned instead of
  disappearing from the preview.
- When the language provider is unavailable, hover, definition, references, and
  rename preview fall back only to file-local syntax/text truth and keep a
  degraded disclosure flag.
- Rename preview never applies edits directly; it only reports candidate,
  skipped, generated, protected, and affected-file counts plus checkpoint or
  rollback posture.

## Protected Proof Path

The protected fixture is
`fixtures/language/tsjs_nav_alpha/wedge_cases.json`. It covers:

- a ready TypeScript language-service lane;
- a sparse active workset with omitted roots and generated candidates; and
- an unavailable provider path that falls back to file-local syntax/text truth.

Run:

```sh
cargo test -p aureline-language --test tsjs_nav_alpha
python3 -m json.tool fixtures/language/tsjs_nav_alpha/wedge_cases.json >/dev/null
```
