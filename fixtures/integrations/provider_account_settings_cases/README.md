# Provider account settings fixtures

Worked scenario bundles for the contract frozen in
[`/docs/integrations/provider_account_settings_contract.md`](../../../docs/integrations/provider_account_settings_contract.md).

Each fixture is a self-contained YAML document. The `records` array
contains records valid against
[`/schemas/integrations/provider_account_state.schema.json`](../../../schemas/integrations/provider_account_state.schema.json)
and
[`/schemas/integrations/default_target_resolution.schema.json`](../../../schemas/integrations/default_target_resolution.schema.json).

Coverage:

| Fixture                                          | Main condition                                                                                       | Required behavior                                                                                                                            |
|--------------------------------------------------|------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------|
| `user_selected_active_account.yaml`              | A user explicitly picked the issue-tracker account on this surface.                                  | `account_binding_class = user_selected_active_account`; `target_provenance_source_class = explicit_user_choice_source` wins per-context.      |
| `inherited_org_mapping_account.yaml`             | The workspace inherits its code-host account binding from an org mapping.                            | `account_binding_class = inherited_org_mapping_account`; the `repo_context` resolution chain shows the inherited org mapping winning over user-default. |
| `admin_forced_target_account.yaml`               | Org admin policy forces the surface onto a specific board and narrows permitted sync modes.          | `account_binding_class = admin_forced_account`; `default_target_resolution_class = admin_forced`; `narrowed_by_admin_policy = true`.         |
| `expired_seat_cached_read_only.yaml`             | Provider seat expired; cached read-only shadow remains.                                              | `account_binding_class = expired_seat_local_only_continuity`; `cached_offline_shadow_class = cached_read_only_after_seat_expiry`; `metadata_safe_export_only = true`. |
| `account_switch_with_queued_drafts.yaml`         | The user switched accounts on the issue-tracker surface while three publish-later drafts were queued. | `account_switch_audit_record` cites typed `scope_change_class`, `affected_workflow_classes`, `queued_draft_disclosure_class`, and `provider_authority_consequence_class`; `preserved_handoff_state_refs` is non-empty. |

Conventions:

- Records cite opaque ids only; raw URLs, raw OAuth tokens, raw
  delegated tokens, raw cookies, raw provider-private profile bodies,
  raw billing payloads, and raw export bodies never appear.
- Fixtures pin `metadata_safe_export_only = true` whenever
  `cached_offline_shadow_class` is anything other than
  `not_in_cached_shadow`, or whenever the surface is in
  `policy_blocked` or `offline_with_cached_read_state` state, so
  silent widening of export, telemetry, or AI-evidence scope is
  structurally impossible.
- `preserved_handoff_state_refs` lists publish-later queue item refs,
  browser-handoff packet refs, offline-capture control refs, or
  imported-snapshot refs the fixture's settings or audit record
  carried across the transition. Loss of provider connectivity, seat
  expiry, account switch, or policy change MUST NOT erase prepared
  handoff state.
- Each fixture is self-contained: settings record(s),
  per-target-context resolution record(s), and (where relevant) the
  richer `account_switch_audit_record` are included in the same
  bundle so a single read explains which account is in effect, why,
  what the default targets are, and what the most recent audited
  transition looked like.
