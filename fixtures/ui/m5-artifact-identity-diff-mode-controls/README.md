# M5 artifact identity bar & diff-mode switcher fixtures

Protected fixtures for row M05-965 (batch B114). Each file is a full, valid
`ArtifactReviewControlsPacket` that exercises a narrowed structured-artifact
review scenario. They validate under
`schemas/ui/m5-artifact-identity-diff-mode-controls.schema.json` and are loaded
by the module's `checked_narrowed_fixtures_validate` test.

- `schema_unrecognized_raw_fallback.json` — an authored notebook whose parser
  drops to `schema_unrecognized`. The identity bar stops claiming a writable
  target, keeps a raw/export-safe fallback note, and drops to a compare-only
  rollback posture; the diff-mode switcher turns off the structured lens (with a
  reason) and makes the raw text fallback the active lens.
- `generated_regenerate_only.json` — keeps the authored, generated, and imported
  artifacts to spotlight the generated API client's regenerate-only truth: it
  names its generated-from relation, points at `openapi/spec.yaml` as the source
  of truth, and carries a `regenerate_only_no_manual_edit` posture.

Regenerate with:

```
GEN_ARTIFACT_IDENTITY_DIFF_MODE_ARTIFACTS=1 \
  cargo test -p aureline-review \
  implement_artifact_identity_bars_and_diff_mode_switchers_with_artifact_class_canonical_source_parser_schema_and_compare_only_truth::tests::generate_artifacts
```
