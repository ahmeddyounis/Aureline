# Proof packet: external alpha benchmark fixture register

```yaml
as_of: 2026-05-15
freshness_date: 2026-05-15
captured_at: 2026-05-15T17:24:31Z
stale_after: P14D
source_revision: git:7ef49d38b543d94113d56e1b3aa289eea9e62c2e
trigger_revision: alpha_fixture_register_contract_set@2026-05-15
validator: ci/check_benchmark_fixture_register.py
validation_capture: artifacts/milestones/m2/captures/benchmark_fixture_register_validation_capture.json
claim_change_state: no_claim_widening
```

Entry page: `docs/benchmarks/privacy_cleared_corpus_workflow.md`
Fixture register: `artifacts/benchmarks/m2_fixture_register.yaml`
Reference workspace workflow packets: `fixtures/reference_workspaces/m2/`
Corpus manifest: `fixtures/benchmarks/corpus_manifest.yaml`
Alpha scope matrix: `artifacts/milestones/m2/alpha_wedge_matrix.yaml`
Go/no-go scoreboard: `artifacts/milestones/m2/exit_gate_scoreboard.yaml`
Validator: `ci/check_benchmark_fixture_register.py`
Latest capture: `artifacts/milestones/m2/captures/benchmark_fixture_register_validation_capture.json`

This packet anchors the alpha benchmark fixture lane. It proves that the
TypeScript / JavaScript and Python reference workspaces have owner, privacy,
provenance, repeatability, corpus, and proof-lane bindings before benchmark or
public-proof packets cite them.

## Protected Proof Path

Run:

`python3 ci/check_benchmark_fixture_register.py --repo-root . --report artifacts/milestones/m2/captures/benchmark_fixture_register_validation_capture.json`

The validator checks that:

- every claimed alpha reference workspace is registered;
- each register row names owner coverage, privacy class, provenance, corpus
  refs, repeatability notes, and intended proof lanes;
- fixture packets point back to their register rows and cover the protected
  workflows from the alpha matrix;
- corpus refs resolve through the protected corpus manifest; and
- the benchmark publication template exposes fixture-register citation fields.
