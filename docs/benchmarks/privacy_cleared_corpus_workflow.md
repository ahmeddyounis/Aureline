# Privacy-Cleared Corpus Workflow

This page is the reviewer-facing workflow for admitting external alpha
reference workspaces and benchmark corpus additions. The machine-readable
register is
[`artifacts/benchmarks/m2_fixture_register.yaml`](../../artifacts/benchmarks/m2_fixture_register.yaml);
packets and dashboards should cite register rows from that file rather than
copying workspace descriptor paths.

## Canonical Artifacts

- Fixture register: `artifacts/benchmarks/m2_fixture_register.yaml`
- Reference workspace workflow packets: `fixtures/reference_workspaces/m2/`
- Existing workspace descriptors: `fixtures/workspaces/reference/`
- Protected benchmark corpus manifest: `fixtures/benchmarks/corpus_manifest.yaml`
- Corpus governance policy: `docs/benchmarks/corpus_governance.md`
- Design-partner intake checklist:
  `artifacts/program/design_partner_intake_checklist.yaml`
- Publication rehearsal checklist:
  `artifacts/bench/publication_rehearsal_checklist.yaml`
- Benchmark publication template:
  `docs/benchmarks/benchmark_publication_pack_template.md`
- Validator: `ci/check_benchmark_fixture_register.py`
- Latest capture:
  `artifacts/milestones/m2/captures/benchmark_fixture_register_validation_capture.json`

## Admission Rule

No new corpus, fixture-repository, or reference-workspace addition is admissible
for alpha benchmark or public-proof packets until it has all of the following:

- provenance: source class, source revision, origin, lineage summary, and
  license posture;
- privacy review: privacy class, clearance decision, reviewer, scan/redaction
  refs, and raw-private-byte omission note;
- repeatability: resolution mode, deterministic inputs, host/toolchain posture,
  and repeatability notes;
- owner coverage: selection, evidence, publication, privacy reviewer, and backup
  owner or waiver;
- proof binding: scoreboard rows, workflow bundle, corpus refs, and packet
  citation fields.

Missing privacy review defaults to `exclude_until_replaced`. Missing
repeatability notes leaves the addition proposed-only and blocks packet use.

## Workflow

1. Bind the candidate to an existing workflow bundle and archetype row.
2. Record source lineage before inspecting or transforming bytes.
3. Run privacy, license, retention, and access review using the shared
   design-partner intake checklist.
4. Prepare or update a register row with corpus refs, proof lanes, owner
   coverage, privacy decision, and repeatability notes.
5. Add or update the reference-workspace workflow packet under
   `fixtures/reference_workspaces/m2/`.
6. Run the validator and refresh the capture before any benchmark packet cites
   the new row.

## Packet Citation

Benchmark publication packets should cite:

- `fixture_register_row` from `artifacts/benchmarks/m2_fixture_register.yaml`;
- corpus ids from the row's `corpus_refs`;
- privacy decision and raw-private-byte omission note; and
- repeatability notes plus any out-of-band materialization step.

Raw partner repository names, private paths, raw trace bodies, and support
packet text remain outside public packets unless the privacy workflow explicitly
admits a sanitized artifact.

## Validation

Run:

`python3 ci/check_benchmark_fixture_register.py --repo-root .`

Optional capture refresh:

`python3 ci/check_benchmark_fixture_register.py --repo-root . --report artifacts/milestones/m2/captures/benchmark_fixture_register_validation_capture.json`

