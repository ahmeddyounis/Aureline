# Python Hover, Navigation, References, and Rename Preview Alpha

This document records the first bounded Python assistance path in
`aureline-language`.

## Owned Runtime Surface

The runtime lives under `crates/aureline-language/src/python/` and exposes
`PythonLaunchWedge`. The wedge consumes the shared `LspRouter` and emits four
inspectable records:

- `python_hover_record`
- `semantic_result_ref_record` for definition targets
- `python_reference_set_record`
- `rename_preview_record`

Every record includes provider identity, freshness, scope, router decision id,
policy context, redaction posture, export-safe summaries, and the selected
Python interpreter/environment context. Raw source bodies and provider logs are
not embedded in these records.

## Scope, Interpreter, And Fallback Rules

- Ready Python language-service results render as compatibility-layer answers
  with `authoritative_live` freshness when the host matches the selected
  interpreter execution context.
- A language-service host for a different interpreter or environment is ignored
  for semantic authority and the result falls back to file-local syntax/text
  truth.
- Missing, ambiguous, drifted, or unavailable interpreter selection blocks
  semantic-provider admission and is preserved in every emitted provider
  snapshot.
- Partial worksets and sparse scopes remain visible through scope descriptors,
  omitted-scope refs, and non-authoritative rename preview posture.
- Generated stubs and read-only rename candidates stay counted and warned
  instead of disappearing from the preview.
- Rename preview never applies edits directly; it only reports candidate,
  skipped, generated, protected, and affected-file counts plus checkpoint or
  rollback posture.

## Protected Proof Path

The protected fixture is
`fixtures/language/python_nav_alpha/wedge_cases.json`. It covers:

- a ready Python language-service lane bound to a selected virtual environment;
- a sparse active workset with omitted roots and generated candidates;
- an unavailable provider path that falls back to file-local syntax/text truth;
- a ready host bound to a different interpreter, which must not answer the
  selected environment; and
- unresolved interpreter selection, which exposes degraded fallback state.

Run:

```sh
cargo test -p aureline-language --test python_nav_alpha
python3 -m json.tool fixtures/language/python_nav_alpha/wedge_cases.json >/dev/null
```
