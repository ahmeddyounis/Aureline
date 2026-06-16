# Open/local-boundary and upstream-durability matrix — evidence pointer

Evidence-index entry for the governance matrix that freezes the open-versus-paid boundary, repository compliance, third-party imports, and maintainer/signer durability across the claimed ecosystem and release lanes. The matrix maps every asset lane to its open/local boundary posture, the compliance/control expectations it must satisfy, the emergency signing/registry/security authority that owns it, and the continuity rules that keep it from depending on one irreplaceable human — and narrows the lane automatically when any of that goes missing, stale, or downgraded.

It is reused by release packets, docs/boundary manifests, repository-compliance scans, and shiproom gates rather than re-stated per surface, and a durability-layer failure on a still-stable lane holds publication while inherited and waived narrowings stay gated upstream.

## Canonical record and proof

- **Matrix JSON**: `artifacts/governance/m5-boundary-and-upstream-durability.json`
- **Schema**: `schemas/governance/m5-boundary-and-upstream-durability.schema.json`
- **Fixtures**: `fixtures/governance/m5-boundary-and-upstream-durability/`
- **Validation capture**: `artifacts/governance/captures/m5-boundary-and-upstream-durability_validation_capture.json`
- **Typed consumer**: `crates/aureline-governance/src/m5_boundary_and_upstream_durability/mod.rs`
- **Protected tests**: `crates/aureline-governance/tests/m5_boundary_and_upstream_durability.rs`
- **Regenerator**: `tools/regenerate_m5_boundary_and_upstream_durability.py`
- **Companion doc**: `docs/m5/freeze_the_m5_open_boundary_repository_compliance_third_party_import_and_maintainer_signer_durability_matrix.md`
- **CI gate**: `.github/workflows/check_m5_boundary_and_upstream_durability.yml`

## Upstream source registers

- `artifacts/release/open_paid_boundary_audit.json`
- `artifacts/governance/signing_quorum.yaml`
- `artifacts/governance/third_party_import_manifest.yaml`
- `artifacts/governance/upstream_health_scorecard.yaml`
- `docs/governance/maintainer_coverage_policy.md`
