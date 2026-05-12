# Proof packet: release-center / provenance seed lane

Purpose: anchor proof captures for the release-center / provenance
skeleton that links the running build's exact-build identity to the
support-bundle preview.

Reviewer entry point: [`/docs/release/release_center_seed.md`](../../../../docs/release/release_center_seed.md).

Canonical sources (non-exhaustive):

- `docs/release/release_center_seed.md`
- `docs/release/release_center_object_model_contract.md`
- `docs/release/release_center_provenance_linkage.md`
- `schemas/release/release_provenance_crosswalk.schema.json`
- `schemas/release/release_center_object.schema.json`
- `artifacts/release/release_support_crosswalk.yaml`

Shell status: bounded prototype not yet wired into the running shell.

- `crates/aureline-shell/src/release_center/`

Upstream dependencies:

- `crates/aureline-build-info/` — exact-build identity through runtime.
- `crates/aureline-support/` — support-bundle manifest the linkage row
  reads.
- `crates/aureline-shell/src/help_about/` — shared install-mode and
  provenance row vocabulary.
- `crates/aureline-shell/src/support_seed/` — bounded prototype not yet wired into the running shell; the linkage row consumes this surface.

Validation captures:

- `artifacts/milestones/m1/captures/release_center_seed_validation_capture.json`

Reviewer fixtures:

- `fixtures/release/release_center_cases/protected_walk_running_build_linked.json`
- `fixtures/release/release_center_cases/failure_drill_missing_provenance_chain.json`

Refresh: re-run the validation lane after a change to the build-info
contract, the support-bundle manifest schema, the Help/About
install-mode or provenance vocabulary, or the release-center surface
projection.
