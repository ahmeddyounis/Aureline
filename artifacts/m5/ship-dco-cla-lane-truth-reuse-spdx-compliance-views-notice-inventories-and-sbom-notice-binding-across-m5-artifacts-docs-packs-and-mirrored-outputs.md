# M5 evidence pointer — repository-compliance and notice-binding truth

Evidence pointer for the repository-compliance and notice-binding register that
publishes, per claimed M5 artifact family, docs pack, and mirrored output, the DCO/CLA
contribution-provenance lane truth, the REUSE/SPDX file-level licensing coverage, the
third-party notice-inventory state, and the SBOM/notice binding. It is the
compliance-truth layer above the open/local boundary and upstream-durability matrix and is
governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Register: `artifacts/governance/m5-compliance-and-notice-binding.json`
- Schema: `schemas/governance/m5-compliance-and-notice-binding.schema.json`
- Reviewer contract: `docs/m5/ship_dco_cla_lane_truth_reuse_spdx_compliance_views_notice_inventories_and_sbom_notice_binding_across_m5_artifacts_docs_packs_and_mirrored_outputs.md`
- Validation capture: `artifacts/governance/captures/m5-compliance-and-notice-binding_validation_capture.json`
- Fixture corpus: `fixtures/governance/m5-compliance-and-notice-binding/`
- Owning crate module: `crates/aureline-governance/src/m5_compliance_and_notice_binding/`
- Regenerator: `python3 tools/regenerate_m5_compliance_and_notice_binding.py`

## Executable proof

Inline unit coverage lives in
`crates/aureline-governance/src/m5_compliance_and_notice_binding/tests.rs`. The protected
gate is `crates/aureline-governance/tests/m5_compliance_and_notice_binding.rs` (run by
`.github/workflows/check_m5_compliance_and_notice_binding.yml`). It loads the embedded
register, proves it validates cleanly, proves every M5 family has an artifact-family record
and that docs packs and mirrored outputs are covered alongside them, proves every record
declares all six control dimensions, proves the DCO/CLA and REUSE/SPDX gaps are recorded as
first-class truth, proves a present, bound SBOM still narrows on a notice gap and that the
scan and the surface agree on every record (no green badge masks a gap), cross-checks the
typed model against the frozen validation capture (summary, scan/surface parity, and
promotion verdict), proves a compliance-layer failure on a still-stable subject holds
promotion while inherited (below-cutline) and waived narrowings stay gated upstream, and
proves the negative fixtures (hidden licensing gap, clean surface over a gapped scan,
narrowed-above-cutline, proceed-while-a-rule-fires) are rejected by the model.

## Narrowing and downgrade

Each record narrows on the specific axis that thins out — a provenance gap, a licensing
gap, a notice gap, an SBOM/binding gap, a stale mirror, or stale proof — and drops its
effective label below the launch cutline. Help/About, service-health, release-center,
support, and evaluation surfaces consume `reuse_projection()` so a narrowed record
downgrades every surface from one source of truth.
