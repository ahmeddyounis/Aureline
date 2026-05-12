# Alpha Scope Matrix and Scoreboard

This page is the reviewer-facing entry point for the external alpha scope
freeze. The canonical truth sources are the YAML artifacts below; downstream
proof packets, dashboards, docs, support exports, and issue templates should
cite those rows instead of copying scope text.

## Canonical Artifacts

- Wedge matrix: `artifacts/milestones/m2/alpha_wedge_matrix.yaml`
- Go/no-go scoreboard: `artifacts/milestones/m2/exit_gate_scoreboard.yaml`
- Dependency graph: `artifacts/milestones/m2/dependency_graph.yaml`
- Proof packet: `artifacts/milestones/m2/proof_packets/alpha_scope.md`
- Design-partner guide: `docs/alpha/design_partner_guide.md`
- Design-partner intake packet: `artifacts/milestones/m2/design_partner_intake_packet.md`
- Design-partner task pack: `artifacts/milestones/m2/design_partner_task_pack.md`
- Design-partner feedback taxonomy: `artifacts/feedback/design_partner_feedback_taxonomy.yaml`
- External alpha known limits: `artifacts/feedback/external_alpha_known_limits.md`
- Benchmark fixture register: `artifacts/benchmarks/m2_fixture_register.yaml`
- Privacy-cleared corpus workflow: `docs/benchmarks/privacy_cleared_corpus_workflow.md`
- Benchmark fixture proof packet: `artifacts/milestones/m2/proof_packets/benchmark_fixture_register.md`
- TypeScript / JavaScript launch bundle manifest: `artifacts/bundles/tsjs_launch_bundle_alpha.yaml`
- Python launch bundle manifest: `artifacts/bundles/python_launch_bundle_alpha.yaml`
- Archetype seed rows: `artifacts/certification/m2_archetype_seed_rows.yaml`
- Launch bundle proof packet: `artifacts/milestones/m2/proof_packets/launch_bundles_and_archetypes.md`
- Launch bundle validator: `ci/check_alpha_launch_bundles.py`
- Latest launch bundle capture: `artifacts/milestones/m2/captures/launch_bundle_validation_capture.json`
- Proof artifact index: `artifacts/milestones/m2/artifact_index.yaml`
- Review packet template: `docs/review/m2_review_packet_template.md`
- Same-change truth workflow: `docs/governance/m2_truth_update_workflow.md`
- Proof artifact index packet: `artifacts/milestones/m2/proof_packets/artifact_index.md`
- Proof artifact index validator: `ci/check_alpha_proof_artifact_index.py`
- Latest proof artifact index capture: `artifacts/milestones/m2/captures/artifact_index_validation_capture.json`
- Latest validation capture: `artifacts/milestones/m2/captures/alpha_scope_validation_capture.json`
- Latest benchmark fixture capture: `artifacts/milestones/m2/captures/benchmark_fixture_register_validation_capture.json`
- Validator: `ci/check_alpha_scope.py`
- Design-partner validator: `ci/check_design_partner_alpha.py`
- Benchmark fixture validator: `ci/check_benchmark_fixture_register.py`

## Definition of Green

The scope freeze is green when:

- the matrix names TypeScript / JavaScript and Python as the only claimed
  external alpha wedges;
- every claimed workflow and deployment row points to one scoreboard row;
- local and helper-backed deployment claims are explicit;
- out-of-scope and held rows require scope review or waiver before widening;
- known limits and supportability rows are present; and
- the validator passes.

Feature proof rows may still be `not_yet_measured` or
`blocked_until_packet_current`; that is intentional. This artifact freezes the
claim surface and proof routing before the feature lanes turn those rows green.

## Claimed Alpha Wedges

| Wedge | Claimed workflows | Deployment claim |
|---|---|---|
| TypeScript / JavaScript web app or service | open to first useful navigation result, rename preview, targeted test/debug loop, Git/review basics | local desktop and helper-backed local services |
| Python service or data app | interpreter and pytest workflow, debug/refactor basics, notebook handoff disclosure, Git/review basics | local desktop and helper-backed Python environment or devcontainer |

## Explicitly Not Claimed

Managed-cloud daily-driver parity, browser/mobile companion parity, full
notebook parity, and additional language wedges are not alpha claims. Adding
one requires updating the matrix, scoreboard, dependency graph, known-limits
packet, and protected fixture path in the same change.

## How to Validate

Run:

`python3 ci/check_alpha_scope.py --repo-root .`

Optional machine-readable report:

`python3 ci/check_alpha_scope.py --repo-root . --report target/alpha-scope/report.json`

External alpha design-partner packet validation:

`python3 ci/check_design_partner_alpha.py --repo-root . --report target/design-partner-alpha/report.json`

External alpha benchmark fixture-register validation:

`python3 ci/check_benchmark_fixture_register.py --repo-root . --report target/benchmark-fixture-register/report.json`

External alpha proof artifact index validation:

`python3 ci/check_alpha_proof_artifact_index.py --repo-root . --report target/alpha-proof-artifact-index/report.json`

External alpha launch bundle validation:

`python3 ci/check_alpha_launch_bundles.py --repo-root . --report target/alpha-launch-bundles/report.json --render-gallery`

## Update Rules

1. Update `artifacts/milestones/m2/alpha_wedge_matrix.yaml` first.
2. Add or update the matching row in
   `artifacts/milestones/m2/exit_gate_scoreboard.yaml`.
3. Update `artifacts/milestones/m2/dependency_graph.yaml` when a source,
   fixture, consumer, or proof root changes.
4. Register proof evidence in `artifacts/milestones/m2/artifact_index.yaml`
   with owner, freshness, exact-build identity, and same-change truth refs.
5. Update the proof packet and known-limits section when the user-visible claim
   surface changes.
6. Run the validator and refresh the capture when the artifact set changes.
