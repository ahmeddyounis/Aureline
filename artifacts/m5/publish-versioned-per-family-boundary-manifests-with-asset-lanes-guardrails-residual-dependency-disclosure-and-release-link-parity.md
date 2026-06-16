# M5 evidence pointer — versioned, per-family boundary manifests with release-link parity

Evidence pointer for the versioned, per-family boundary-manifest register that
publishes, per claimed M5 family and per manifest version, which capabilities stay
open/local, which may be productized, the guardrails preserving the claim, the
residual proprietary/hosted dependencies the family still rests on, and the release
train each manifest is linked to. It is the publication layer above the open/local
boundary and upstream-durability matrix and is governed by the canonical M5 evidence
index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Register: `artifacts/governance/m5-versioned-boundary-manifests.json`
- Schema: `schemas/governance/m5-versioned-boundary-manifests.schema.json`
- Reviewer contract: `docs/m5/publish_versioned_per_family_boundary_manifests_with_asset_lanes_guardrails_residual_dependency_disclosure_and_release_link_parity.md`
- Validation capture: `artifacts/governance/captures/m5-versioned-boundary-manifests_validation_capture.json`
- Fixture corpus: `fixtures/governance/m5-versioned-boundary-manifests/`
- Owning crate module: `crates/aureline-governance/src/m5_versioned_boundary_manifests/`
- Regenerator: `python3 tools/regenerate_m5_versioned_boundary_manifests.py`

## Executable proof

Inline unit coverage lives in
`crates/aureline-governance/src/m5_versioned_boundary_manifests/tests.rs`. The
protected gate is
`crates/aureline-governance/tests/m5_versioned_boundary_manifests.rs`
(run by `.github/workflows/check_m5_versioned_boundary_manifests.yml`). It loads the
embedded register, proves it validates cleanly, proves every M5 family has exactly
one versioned manifest, proves every manifest declares all five guardrails and at
least one per-asset-lane entry (no vague open-core copy), proves residual
proprietary/hosted dependencies are recorded and that any undisclosed dependency
narrows its manifest, proves no manifest publishes a label greener than its release
evidence, cross-checks the typed model against the frozen validation capture
(summary, release-link parity, and publication verdict), proves a manifest-layer
failure on a still-stable family holds promotion while inherited (below-cutline) and
waived narrowings stay gated upstream, and proves the negative fixtures
(over-claim, undisclosed dependency, narrowed-above-cutline, proceed-while-a-rule-fires)
are rejected by the model.

## Narrowing and downgrade

Each manifest narrows on the specific axis that thins out — parity break, release-link
gap, undisclosed dependency, unsatisfied guardrail, or stale proof — and drops its
effective label below the launch cutline. Help/About, service-health, docs, support,
and evaluation surfaces consume `reuse_projection()` so a narrowed manifest downgrades
every surface from one source of truth.
