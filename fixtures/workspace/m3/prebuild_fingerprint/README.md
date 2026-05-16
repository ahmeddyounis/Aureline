# Prebuild fingerprint alpha fixtures

Fixtures in this directory anchor the alpha record contract documented in
[`/docs/workspace/m3/prebuild_fingerprint_alpha.md`](../../../../docs/workspace/m3/prebuild_fingerprint_alpha.md).
Each JSON file validates against
[`/schemas/workspace/prebuild_fingerprint.schema.json`](../../../../schemas/workspace/prebuild_fingerprint.schema.json),
projects through `aureline_workspace::prebuilds::project_prebuild_fingerprint_alpha`,
and renders deterministic rows through
[`/crates/aureline-shell/src/start_center/prebuild_fingerprints/mod.rs`](../../../../crates/aureline-shell/src/start_center/prebuild_fingerprints/mod.rs).

| Fixture | Record kind | Demonstrates |
|---|---|---|
| [`valid_cached_prebuild_fingerprint.json`](./valid_cached_prebuild_fingerprint.json) | `prebuild_fingerprint_record` | Full fingerprint coverage, cache classes, redaction posture, residue exclusions. |
| [`reuse_allowed_decision.json`](./reuse_allowed_decision.json) | `prebuild_reuse_decision_record` | Reuse allowed only when no invalidation or revalidation remains. |
| [`stale_snapshot_resume_denied_decision.json`](./stale_snapshot_resume_denied_decision.json) | `prebuild_reuse_decision_record` | A stale snapshot cannot masquerade as live resume. |
| [`local_override_rebuild_disclosure.json`](./local_override_rebuild_disclosure.json) | `prebuild_disclosure_record` | Local overrides are disclosed and require rebuild before trust. |
| [`fresh_clone_disclosure.json`](./fresh_clone_disclosure.json) | `prebuild_disclosure_record` | Fresh clone stays distinct from cached prebuild authority. |

Raw secrets, raw credential bodies, raw environment values, raw command
lines, machine-unique trust anchors, and uncommitted workspace edits never
appear in these fixtures.
