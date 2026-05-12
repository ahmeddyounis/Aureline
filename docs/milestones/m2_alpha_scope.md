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
- Latest validation capture: `artifacts/milestones/m2/captures/alpha_scope_validation_capture.json`
- Validator: `ci/check_alpha_scope.py`

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

## Update Rules

1. Update `artifacts/milestones/m2/alpha_wedge_matrix.yaml` first.
2. Add or update the matching row in
   `artifacts/milestones/m2/exit_gate_scoreboard.yaml`.
3. Update `artifacts/milestones/m2/dependency_graph.yaml` when a source,
   fixture, consumer, or proof root changes.
4. Update the proof packet and known-limits section when the user-visible claim
   surface changes.
5. Run the validator and refresh the capture when the artifact set changes.

