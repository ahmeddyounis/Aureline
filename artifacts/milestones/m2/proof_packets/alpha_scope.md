# Proof packet: external alpha scope matrix

Entry page: `docs/milestones/m2_alpha_scope.md`
Canonical matrix: `artifacts/milestones/m2/alpha_wedge_matrix.yaml`
Go/no-go scoreboard: `artifacts/milestones/m2/exit_gate_scoreboard.yaml`
Dependency graph: `artifacts/milestones/m2/dependency_graph.yaml`
Validator: `ci/check_alpha_scope.py`

This packet anchors the alpha scope freeze so later proof lanes have one row to
cite for launch wedges, deployment claims, known limits, and go/no-go state.

## Known Limits

The alpha claim is limited to the TypeScript / JavaScript and Python wedges
listed in the matrix. Managed-cloud daily-driver parity, browser/mobile
companion parity, full notebook parity, and additional language wedges require
a scope review or waiver plus updates to the matrix, scoreboard, dependency
graph, and this packet.

## Protected Proof Path

Run:

`python3 ci/check_alpha_scope.py --repo-root . --report artifacts/milestones/m2/captures/alpha_scope_validation_capture.json`

The validator checks that:

- TS/JS and Python alpha wedge rows exist;
- every claimed workflow and deployment row resolves to a scoreboard row;
- every scoreboard row names proof-packet refs and a go/no-go state;
- protected fixture refs exist; and
- held or excluded rows cannot become alpha claims without scope review.

