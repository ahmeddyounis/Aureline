# Provider qualification case fixtures

Worked qualification cases for the packet frozen in
[`/docs/providers/provider_qualification_packet.md`](../../../docs/providers/provider_qualification_packet.md),
the support matrix in
[`/artifacts/providers/provider_support_matrix.yaml`](../../../artifacts/providers/provider_support_matrix.yaml),
and the parity-and-packet drill suite in
[`/artifacts/qa/provider_handoff_parity_suite.yaml`](../../../artifacts/qa/provider_handoff_parity_suite.yaml).

Each fixture is a self-contained YAML document that pins a single
matrix row or qualification drill to a reviewable scenario. The
fixtures are higher-level qualification evidence than the
schema-bound record fixtures elsewhere in
`/fixtures/providers/`; they cite those record fixtures by relative
path rather than re-emitting the full record bodies.

The `__fixture__` header on every file names the scenario, the
matrix row id (when the case qualifies a matrix row), the drill id
(when the case qualifies a parity-suite drill), and the closed
vocabulary members the case exercises.

Coverage families:

| Family | Files |
|---|---|
| Matrix rows | `review_or_code_host__pull_request.yaml`, `issue_or_planning_tracker__issue_or_work_item.yaml`, `ci_or_check_provider__check_run.yaml`, `docs_or_portal_provider__docs_page.yaml`, `artifact_registry__package_version.yaml`, `release_publisher__release_artifact.yaml`, `ai_provider__other.yaml`, `managed_admin__admin_surface.yaml` |
| Acting-identity drills | `acting_identity_human_account_qualification.yaml`, `acting_identity_installation_or_app_grant_qualification.yaml`, `acting_identity_delegated_user_token_qualification.yaml`, `acting_identity_project_scoped_grant_qualification.yaml`, `acting_identity_policy_injected_service_qualification.yaml` |
| Browser-handoff parity drills | `browser_handoff_parity_review_anchor_round_trip.yaml`, `browser_handoff_parity_object_link_anchor_round_trip.yaml`, `browser_handoff_parity_browser_unavailable_degraded_alternative.yaml` |
| Packet-field drills | `revoked_grant_packet_field_qualification.yaml`, `delayed_delivery_packet_field_qualification.yaml`, `host_mismatch_packet_field_qualification.yaml`, `partial_import_packet_field_qualification.yaml`, `mirror_or_self_host_routing_packet_field_qualification.yaml` |

Adding a case is additive-minor. Repurposing a `case_id` is
breaking and requires a new decision row.
