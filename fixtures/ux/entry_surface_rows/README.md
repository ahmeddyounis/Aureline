# First-run no-account local-work, service-opt-in boundary, and onboarding portability fixtures

Seed corpus for the contract frozen in
[`/docs/ux/no_account_local_entry_contract.md`](../../../docs/ux/no_account_local_entry_contract.md)
and the schema at
[`/schemas/ux/onboarding_portability_state.schema.json`](../../../schemas/ux/onboarding_portability_state.schema.json).

Each file is a single JSON document validating against one of the
four record kinds in the schema
(`entry_surface_row_record`,
`account_prompt_record`,
`onboarding_portability_state_record`,
`onboarding_portability_manifest_record`).

Every fixture:

- Resolves every axis to vocabulary either re-exported from the
  Start Center contract, the entry-restore object model, the
  entry-restore truth audit, the onboarding measurement plan,
  and the deployment-profile register, or introduced by this
  contract in §3.
- Pins the `account_prompt_class`,
  `account_prompt_timing_class`, `boundary_crossing_class`,
  `state_portability_class`, `reset_class`, and `export_class`
  values this contract owns.
- Carries no raw absolute paths, raw URLs, raw credential
  material, or raw secrets. Every id is an opaque ref; every
  timestamp is a monotonic placeholder.
- Names the contract sections it exercises under
  `__fixture__.contract_sections`.

## Cases

| Fixture | Record kind | Scenario axis | Contract anchor |
| --- | --- | --- | --- |
| [`first_run_no_account_open_folder.json`](./first_run_no_account_open_folder.json) | `entry_surface_row_record` | First-run `Open folder` row under `individual_local`; `no_prompt`, `no_boundary_crossed`, equal-weight floor. | §3.2, §3.4, §4, §11.1 |
| [`service_opt_in_telemetry_optional.json`](./service_opt_in_telemetry_optional.json) | `entry_surface_row_record` (+ `account_prompt_record` linkage) | Optional telemetry opt-in on first-run; declinable, stays in local-only lane. | §3.2, §3.3, §5, §11.2 |
| [`service_opt_in_telemetry_optional_prompt.json`](./service_opt_in_telemetry_optional_prompt.json) | `account_prompt_record` | Prompt record paired with the telemetry row; `optional_prompt`, `shown_at_first_run_declinable`, `stays_in_local_only_lane`. | §5, §11.2 |
| [`managed_cloud_resume_reauth_required.json`](./managed_cloud_resume_reauth_required.json) | `entry_surface_row_record` | Managed-cloud resume after authority lapse; `required_prompt`, `reaches_remote_resource`, aborts with local alternative offered. | §3.2, §3.4, §11.3 |
| [`managed_cloud_resume_reauth_prompt.json`](./managed_cloud_resume_reauth_prompt.json) | `account_prompt_record` | Prompt record paired with the managed-cloud resume row; `required_prompt`, `shown_at_entry_declinable`, `reconnect_required` / `reauth_required`. | §5, §11.3 |
| [`policy_forced_managed_fleet_sso.json`](./policy_forced_managed_fleet_sso.json) | `account_prompt_record` | Managed-fleet SSO prompt; `policy_forced_prompt`, `forced_by_policy`, `managed_fleet_sso`, cites active policy bundle. | §3.2, §5.1, §11.4 |
| [`air_gapped_marketplace_unavailable.json`](./air_gapped_marketplace_unavailable.json) | `account_prompt_record` | Marketplace-browse card on `air_gapped_mirror_only`; `unavailable_prompt`, `unavailable_in_envelope`, narrowing advertised. | §3.2, §5.1, §11.5 |
| [`privacy_reduced_open_and_clone_preserved.json`](./privacy_reduced_open_and_clone_preserved.json) | `entry_surface_row_record` | `hide_all_except_open_and_clone` privacy-reduction mode; `Open folder` still `no_prompt`, equal-weight floor preserved. | §4 rule 3–4, §7.2.3, §11.6 |
| [`onboarding_state_tour_progress_portable.json`](./onboarding_state_tour_progress_portable.json) | `onboarding_portability_state_record` | Tour progress rides the portable profile package; `portable_profile_state`, `resettable_per_profile`. | §3.5, §6.3, §11.7 |
| [`onboarding_state_recent_work_metadata_device_scoped.json`](./onboarding_state_recent_work_metadata_device_scoped.json) | `onboarding_portability_state_record` | Recent-work metadata is device-local, never exported; `device_scoped`, `not_exported_machine_local`. | §6.3, §7.2.9, §11.8 |
| [`onboarding_state_imported_profile_history_portable.json`](./onboarding_state_imported_profile_history_portable.json) | `onboarding_portability_state_record` | Imported-profile history rides the portable profile package redacted; moves with the user across devices. | §6.3, §11.9 |
| [`onboarding_portability_manifest.json`](./onboarding_portability_manifest.json) | `onboarding_portability_manifest_record` | Top-level manifest declaring all twelve const-true invariants and binding the state items plus entry-surface rows. | §7, §11.10 |

## Schema references

- Onboarding portability / no-account schema:
  [`/schemas/ux/onboarding_portability_state.schema.json`](../../../schemas/ux/onboarding_portability_state.schema.json).
- Start Center / workspace-switcher schema (upstream):
  [`/schemas/ux/start_center_surface.schema.json`](../../../schemas/ux/start_center_surface.schema.json).

## Companion corpora

- Start Center / workspace-switcher / restore-card worked fixtures:
  [`/fixtures/ux/start_center_rows/`](../start_center_rows/).
- `startup_state` tokens and per-state placeholder truth rows:
  [`/fixtures/ux/entry_restore_states/`](../entry_restore_states/).
- No-account switching scoreboard seed (row ids this corpus cites):
  [`/artifacts/product/no_account_switching_scoreboard_seed.yaml`](../../../artifacts/product/no_account_switching_scoreboard_seed.yaml).
