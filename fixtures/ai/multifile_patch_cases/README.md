# AI multi-file patch review worked-example corpus

This directory holds worked examples for the protected AI multi-file patch
review/apply sequence frozen in:

- `artifacts/ai/multifile_patch_review_sequence.md`
- `schemas/ai/patch_review_summary.schema.json`

Each `.yaml` file is a multi-document YAML stream. The first document is a
`__fixture__` prelude describing the scenario and the exercised vocabulary
axes. The remaining documents are individual records:

- `patch_review_summary_record`
- `patch_validation_summary_record`
- `patch_review_approval_record`
- `patch_apply_audit_record`
- `patch_review_audit_event_record` (when used)

No fixture embeds raw patch bodies, raw diff text, raw prompt text, raw file
bodies, raw terminal/log bodies, raw URLs, raw absolute paths, or raw credential
material. Diff and patch content is referenced only by opaque artifact refs and
content-addressed digests.

