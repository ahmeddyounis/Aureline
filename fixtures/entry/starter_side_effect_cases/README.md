# Entry starter side-effect diff fixtures

This directory is the seed corpus for the **starter action diff** contract:

- Contract (entry surfaces): `artifacts/entry/environment_starter_summary_contract.md`
- Schema: `schemas/entry/starter_action_diff.schema.json`

Each `*.yaml` file is one exported `starter_preflight_action_record` that:

- expresses starter behavior as a diff: `plain_open_baseline` + `starter_added_delta`;
- partitions actions into `actions_run_now[]` and `actions_deferred[]`;
- uses only the closed `starter_preflight_action_class` taxonomy (including `port_exposure`);
- lists at least one same-weight `bypass_path_id`;
- keeps bypass lanes present under offline/policy-narrowed scenarios (captured in fixture scenarios).

## Index

| Fixture | Lane focus | Scenario focus |
| --- | --- | --- |
| `open_folder_plain_open.yaml` | plain open | same-weight open folder without starter |
| `open_workspace_plain_open.yaml` | plain open | same-weight open workspace without starter |
| `starter_scaffold_local_files_and_dependency_restore.yaml` | starter diff | file generation + dependency restore; deferred extensions/tasks |
| `starter_managed_remote_with_deferred_reapproval.yaml` | starter diff | remote provisioning now; trust/secret deferred; continue-without-starter present |
| `starter_devcontainer_port_exposure_previewed.yaml` | starter diff | port exposure/forwarding disclosed as an explicit action row |
| `starter_policy_narrowed_bypass_still_present.yaml` | starter diff | bypass remains same-weight under policy-gated deferred actions |

