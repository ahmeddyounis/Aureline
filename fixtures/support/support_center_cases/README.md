# Support Center route fixtures

Seed corpus for the Support Center information architecture and
route-to-evidence contract frozen in
[`docs/support/support_center_information_architecture.md`](../../../docs/support/support_center_information_architecture.md).

Every case in this directory projects onto one
`support_center_route_record` shape from
[`schemas/support/support_center_capability_card.schema.json`](../../../schemas/support/support_center_capability_card.schema.json)
and resolves to one capability-card row in
[`artifacts/support/support_center_routes.yaml`](../../../artifacts/support/support_center_routes.yaml).
Cases are reviewable contracts over the route shape, not full live
routes. They preserve stable symptom-surface class, primary module
class, default first action, deployment-context parity rows, and
evidence-id classes so a user does not restate their case after
handoff and a parity gap between local-only, managed, self-hosted,
mirrored, and offline deployments stays visible rather than implicit.

The five required deployment contexts (closed at this revision):

| Case file | `deployment_context_class` | Symptom surface | Primary module |
|---|---|---|---|
| `local_only_error_surface_to_project_doctor.yaml` | `local_only` | `error_surface` | `project_doctor` |
| `managed_policy_denial_to_project_doctor.yaml` | `managed` | `policy_denial_surface` | `project_doctor` |
| `self_hosted_transport_failure_to_field_diagnostics.yaml` | `self_hosted` | `transport_failure_surface` | `field_diagnostics` |
| `mirrored_update_failure_to_advisory_history.yaml` | `mirrored` | `update_failure_surface` | `advisory_or_incident_history` |
| `offline_crash_loop_to_crash_triage.yaml` | `offline` | `crash_loop_surface` | `crash_triage` |

Each case enforces:

- `no_upload_first_invariant.local_first_path_named: true` and
  `no_upload_first_invariant.upload_required_for_first_action: false`;
- exactly one `deployment_context_parity_row` per
  `deployment_context_class` (`local_only`, `managed`,
  `self_hosted`, `mirrored`, `offline`);
- a non-empty `evidence_preserved_for_escalation.preserved_evidence_id_classes`
  list so the route's evidence rides forward into any later
  escalation packet without re-asking the user.

Adding a new fixture for an existing deployment context (for
example, a second managed case under a different symptom surface)
is additive-minor. Repurposing a `deployment_context_class` token
or a `symptom_surface_class` token is breaking and requires a new
decision row in `artifacts/governance/decision_index.yaml`.
