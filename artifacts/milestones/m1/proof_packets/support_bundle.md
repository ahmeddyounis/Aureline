# Proof packet: support bundle lane

Purpose: anchor support-bundle proof captures (manifests, redaction
checks, sample exports) in one indexed location.

Reviewer entry point: [`/docs/support/support_bundle_seed.md`](../../../../docs/support/support_bundle_seed.md).

Canonical sources (non-exhaustive):

- `docs/support/support_bundle_seed.md`
- `docs/support/support_bundle_redaction_guide.md`
- `docs/support/support_export_vocabulary_seed.md`
- `docs/support/support_bundle_contract.md`
- `docs/support/support_bundle_preview_contract.md`
- `schemas/support/support_bundle_manifest.schema.json`
- `schemas/support/support_bundle_preview_item.schema.json`
- `schemas/support/support_bundle.schema.json`
- `fixtures/support/redaction_profiles/local_first_default.yaml`
- `artifacts/support/redaction_accuracy_checks.yaml`

Shell status: bounded prototype not yet wired into the running shell.

- `crates/aureline-shell/src/support_seed/`

Owning seed crate:

- `crates/aureline-support/`

Validation captures:

- `artifacts/milestones/m1/support_exports/support_bundle_seed_capture.json`

Reviewer fixtures:

- `fixtures/support/support_seed_cases/protected_walk_default_local_preview.json`
- `fixtures/support/support_seed_cases/failure_drill_secret_bearing_prohibited.json`

Refresh: re-emit fixtures and re-run the validation lane after a change
to the manifest schema, the local-first redaction profile, the
exact-build capture path, or the support-seed surface.
