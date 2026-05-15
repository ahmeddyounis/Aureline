# External alpha scope-freeze proof packet

```yaml
packet_id: review_packet:alpha.scope_freeze.2026-05-15
artifact_index_ref: artifacts/milestones/m2/artifact_index.yaml
scoreboard_row_refs:
  - scoreboard_row:alpha_scope.scope_change_control
owner_dri: "@ahmeddyounis"
reviewer_refs:
  - "@alpha-review"
freshness_date: 2026-05-15
captured_at: 2026-05-15T09:05:23Z
stale_after: P14D
source_revision: git:74110a768890b0bf89a505e2f3e900dae168bbae
trigger_revision: alpha_scope_contract_set@2026-05-15
exact_build_identity_ref: artifacts/build/build_identity.json
channel_context: preview
deployment_context:
  - individual_local
claim_change_state: no_claim_widening
same_change_truth_refs:
  docs_ref: docs/milestones/m2_alpha_scope.md
  migration_ref: docs/migration/source_ecosystem_coverage_matrix.md
  help_truth_ref: docs/docs/help_about_service_health_routes.md
  known_limits_ref: artifacts/feedback/external_alpha_known_limits.md
  support_export_ref: docs/support/support_bundle_contract.md
```

This packet promotes the scope-change-control row to green by proving that the
external alpha claim surface is frozen and that late additions route through
the existing scope review or waiver discipline. It documents the current rule
set from `artifacts/milestones/m2/alpha_wedge_matrix.yaml` without changing the
policy text.

## Canonical Artifacts

- Scoreboard row: `scoreboard_row:alpha_scope.scope_change_control`
- Scope matrix: `artifacts/milestones/m2/alpha_wedge_matrix.yaml`
- Go/no-go scoreboard: `artifacts/milestones/m2/exit_gate_scoreboard.yaml`
- Dependency graph: `artifacts/milestones/m2/dependency_graph.yaml`
- Known-limits packet: `artifacts/milestones/m2/proof_packets/alpha_scope.md#known-limits`
- Latest capture: `artifacts/milestones/m2/captures/alpha_scope_validation_capture.json`
- Validator: `ci/check_alpha_scope.py`
- Exact build identity: `artifacts/build/build_identity.json`

## Required Outcome

`accept_current`

The row is green because the matrix is frozen, TS/JS and Python remain the only
claimed external alpha wedges, held and out-of-scope rows require explicit
scope review before widening, and the validator passes against the matrix,
scoreboard, and dependency graph.

The current late-addition policy is
`explicit_scope_review_or_waiver`. The default decision for an unlisted wedge
is `no_alpha_claim`.

## Late-Addition Rule Set

Any new alpha wedge, deployment row, or public-facing claim widening must carry
the matrix change-control refs below before it can be treated as an alpha
claim:

- `artifacts/milestones/m2/alpha_wedge_matrix.yaml`
- `artifacts/milestones/m2/exit_gate_scoreboard.yaml`
- `artifacts/milestones/m2/dependency_graph.yaml`
- `artifacts/milestones/m2/proof_packets/alpha_scope.md#known-limits`

Waivers must enumerate every field required by
`change_control.waiver_fields_required`:

- `requested_change`
- `affected_claim_surface`
- `rollback_or_narrowing_path`
- `fresh_evidence_required`
- `docs_support_migration_impact`
- `owner`
- `expiry`

## Evidence Rows

| Evidence | Value |
|---|---|
| Owning packet | `artifacts/milestones/m2/proof_packets/scope_freeze.md` |
| Latest capture | `artifacts/milestones/m2/captures/alpha_scope_validation_capture.json` |
| Validator command | `python3 ci/check_alpha_scope.py --repo-root . --report artifacts/milestones/m2/captures/alpha_scope_validation_capture.json` |
| Freshness rule | `docs_claim_truth_proof`, `stale_after: P14D` |

## Protected Proof Path

Run the protected scope check and refresh the checked-in capture:

```sh
python3 ci/check_alpha_scope.py --repo-root . --report artifacts/milestones/m2/captures/alpha_scope_validation_capture.json
```

The validator checks that:

- the alpha matrix is in the `frozen` scope state;
- the required scope-change refs are present in the matrix change-control row;
- TS/JS and Python are the only claimed launch wedges;
- every claimed workflow, deployment row, and protected fixture resolves to a
  known scoreboard row;
- every scoreboard row carries a proof-packet ref and go/no-go state; and
- held or excluded rows cannot become alpha claims without scope review,
  proof-packet, scoreboard, and known-limit updates.

## Same-Change-Set Checklist

- Owning proof packet added.
- Validator capture refreshed.
- Scoreboard row moved to `green` and remains `scope_go`.
- Scoreboard row cites the matrix, scoreboard, dependency graph, owning packet,
  and latest capture.
- Docs, migration notes, Help/About truth, known limits, and support-export
  truth are unchanged because this is `no_claim_widening`.
