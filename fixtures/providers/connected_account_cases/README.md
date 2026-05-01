# Connected-account, acting-identity-badge, and effective-scope fixtures

Worked cases for the contract frozen in
[`/docs/providers/connected_account_registry_contract.md`](../../../docs/providers/connected_account_registry_contract.md).

Each fixture is a self-contained YAML document bundling the records a
single scenario would emit. Every record is schema-valid against one
of the boundary schemas:

- [`/schemas/providers/connected_account_record.schema.json`](../../../schemas/providers/connected_account_record.schema.json)
  (registry link / grant / credential / identity records, the
  acting-identity badge record, and the account-invalidation event
  record).
- [`/schemas/providers/effective_scope_resolution.schema.json`](../../../schemas/providers/effective_scope_resolution.schema.json)
  (the provider-scope-resolution result, the least-privilege
  alternative, and the effective-scope invalidation event).

The owning `connected_provider_record` (the registry-row anchor every
fixture references through `connected_provider_record_id`) is frozen
in [`/schemas/integration/browser_handoff_packet.schema.json`](../../../schemas/integration/browser_handoff_packet.schema.json)
and is exercised in
[`/fixtures/providers/provider_mode_cases/`](../provider_mode_cases/);
the fixtures here intentionally leave that record out of the bundle
so each case is focused on the registry / badge / scope-resolution
shape under test.

The `__fixture__` header on every file names the scenario, the actor
class(es), the badge label class(es), and the decision class(es) the
case exercises. The `records` array carries the concrete records.

Coverage across the seeded scenarios:

| Scenario file                                                  | Actor class(es)                          | Badge label class(es)                                 | Decision class / event class                                      |
|----------------------------------------------------------------|------------------------------------------|--------------------------------------------------------|--------------------------------------------------------------------|
| `human_account_link_allowed_review_publish.yaml`               | `human_account`                          | `you_label`                                            | `provider_scope_resolution_result_record` → `allowed`              |
| `installation_grant_allowed_ci_check.yaml`                     | `installation_or_app_grant`              | `install_label`                                        | `provider_scope_resolution_result_record` → `allowed`              |
| `delegated_credential_browser_only_admin_surface.yaml`         | `delegated_user_token`                   | `delegated_label`                                      | `provider_scope_resolution_result_record` → `browser_only`         |
| `project_scoped_grant_local_draft_only_offline.yaml`           | `project_scoped_grant`                   | `project_scoped_grant_label`                           | `provider_scope_resolution_result_record` → `local_draft_only`     |
| `policy_injected_service_identity_release_publish.yaml`        | `policy_injected_service_identity`       | `policy_injected_service_label`                        | `provider_scope_resolution_result_record` → `allowed`              |
| `unknown_actor_class_denied_repair_required.yaml`              | `unknown_actor_class`                    | `unknown_actor_repair_label`                           | `provider_scope_resolution_result_record` → `denied`               |
| `account_invalidation_org_membership_loss.yaml`                | `installation_or_app_grant`              | (registry only)                                        | `account_invalidation_event_record` → `org_membership_loss`        |
| `effective_scope_invalidation_policy_epoch_rolled.yaml`        | `policy_injected_service_identity`       | (resolution only)                                      | `effective_scope_invalidation_event_record` → `policy_epoch_rolled`|

Vocabulary coverage across the bundle:

- `provider_actor_class`: `human_account`, `installation_or_app_grant`,
  `delegated_user_token`, `project_scoped_grant`,
  `policy_injected_service_identity`, `unknown_actor_class`.
- `label_class`: `you_label`, `install_label`, `bot_label`,
  `delegated_label`, `project_scoped_grant_label`,
  `policy_injected_service_label`, `unknown_actor_repair_label`.
- `decision_class`: `allowed`, `denied`, `browser_only`,
  `local_draft_only`.
- `invalidation_cause_class` and `invalidation_trigger_class`:
  exercised through the two invalidation fixtures
  (`org_membership_loss` and `policy_epoch_rolled`); the remaining
  causes / triggers are listed in the contract and bound to the same
  schema enforcement rules.

Adding a case is additive-minor. Repurposing a `case_id` is breaking
and requires a new decision row.
