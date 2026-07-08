# M5 structured-artifact review component fixtures

Protected fixtures for row M05-964 (batch B114). Each file is a full, valid
`M5ArtifactComponentMatrixPacket` that exercises a narrowed structured-artifact
review scenario. They validate under
`schemas/ui/m5-structured-artifact-review-component-matrix.schema.json` and are
loaded by the module's `checked_narrowed_fixtures_validate` test.

- `rendered_compare_viewer_render_untrusted.json` — the rendered compare viewer
  narrows its fidelity vocabulary to `render_untrusted` + `raw_fallback`,
  proving an untrusted render is labeled and offered an export-safe fallback
  rather than opened as trusted.
- `media_metadata_rail_metadata_unavailable.json` — the media-metadata rail
  narrows to `raw_fallback` + `redacted_or_withheld`, proving missing or
  redacted media metadata stays explicit instead of blank.

Regenerate with:

```
GEN_STRUCTURED_ARTIFACT_REVIEW_ARTIFACTS=1 \
  cargo test -p aureline-review \
  freeze_the_m5_structured_artifact_review_component_matrix::tests::generate_artifacts
```
