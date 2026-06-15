# M5 Release-Candidate, Publish-Target, Artifact-Bundle, and Exact-Build Publication Matrix Artifact Companion

This file is the artifact-level companion document for the checked-in M5 publication matrix.

- **Canonical JSON**: `artifacts/release/m5/freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix.json`
- **Schema**: `schemas/governance/freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix.schema.json`
- **Typed consumer**: `crates/aureline-release/src/freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix/mod.rs`
- **Overview**: `docs/m5/freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix.md`
- **Protected fixtures**: `fixtures/release/m5/freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix/`

The matrix is the single source of truth that maps every new M5 artifact family to its release candidate scope, publish target class, required evidence refs, exact-build identity fields, rollback/revocation posture, and mirror/offline publication expectations. Release-center surfaces, headless publication flows, and support/export packets ingest it directly, and claim-narrowing logic narrows any family whose exact-build linkage or evidence thins out.
