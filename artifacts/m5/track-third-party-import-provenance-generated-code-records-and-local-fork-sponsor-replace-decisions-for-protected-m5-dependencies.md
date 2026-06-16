# M5 evidence pointer — import-provenance and local-fork review truth

Evidence pointer for the import-provenance and local-fork review register that publishes,
per protected-path import used by an M5 family, the import provenance (origin attribution,
SPDX license, upstream pin), the update ownership, the divergence profile, the
sponsor/fork/replace decision required for long-lived forks and single-source imports, and
the generated-code generator identity and regeneration path. It is the import-truth layer
above the open/local boundary and upstream-durability matrix and is governed by the
canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Register: `artifacts/governance/m5-import-provenance-and-fork-review.json`
- Schema: `schemas/governance/m5-import-provenance-and-fork-review.schema.json`
- Reviewer contract: `docs/m5/track_third_party_import_provenance_generated_code_records_and_local_fork_sponsor_replace_decisions_for_protected_m5_dependencies.md`
- Validation capture: `artifacts/governance/captures/m5-import-provenance-and-fork-review_validation_capture.json`
- Fixture corpus: `fixtures/governance/m5-import-provenance-and-fork-review/`
- Owning crate module: `crates/aureline-governance/src/m5_import_provenance_and_fork_review/`
- Regenerator: `python3 tools/regenerate_m5_import_provenance_and_fork_review.py`

## Executable proof

Inline unit coverage lives in
`crates/aureline-governance/src/m5_import_provenance_and_fork_review/tests.rs`. The
protected gate is `crates/aureline-governance/tests/m5_import_provenance_and_fork_review.rs`
(run by `.github/workflows/check_m5_import_provenance_and_fork_review.yml`). It loads the
embedded register, proves it validates cleanly, proves every import kind is exercised and
every narrowing reason is watched by a rule, proves every record declares all six control
dimensions, proves the provenance, ownership, divergence, and generator gaps are recorded
as first-class truth, proves an ownerless or generator-free import still narrows and that
the scan and the surface agree on every record (no clean import card masks a gap),
cross-checks the typed model against the frozen validation capture (summary,
manifest/surface parity, and promotion verdict), proves an import-layer failure on a
still-stable subject holds promotion while inherited (below-cutline) and waived narrowings
stay gated upstream, and proves the negative fixtures (hidden ownership gap, clean surface
over a gapped scan, narrowed-above-cutline, proceed-while-a-rule-fires) are rejected by the
model.

## Narrowing and downgrade

Each record narrows on the specific axis that thins out — a provenance gap, an ownership
gap, a divergence/decision gap, a generator gap, or stale proof — and drops its effective
label below the launch cutline. Help/About, service-health, release-center, support, and
architecture-board surfaces consume `reuse_projection()` so a narrowed record downgrades
every surface from one source of truth.
