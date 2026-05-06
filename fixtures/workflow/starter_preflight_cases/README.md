# Starter preflight action-envelope fixtures

Seed corpus for the contract frozen in
[`/docs/workflow/starter_side_effect_envelope.md`](../../../docs/workflow/starter_side_effect_envelope.md)
and the schema at
[`/schemas/workflow/starter_preflight_action.schema.json`](../../../schemas/workflow/starter_preflight_action.schema.json).

Each file is a single YAML `starter_preflight_action_record` that:

- expresses starter behavior as a diff: `plain_open_baseline` + `starter_added_delta`;
- partitions actions into `actions_run_now[]` and `actions_deferred[]`;
- uses only the closed `starter_preflight_action_class` taxonomy;
- lists at least one same-weight `bypass_path_id`;
- includes a `recovery_path_sentence` that makes the escape hatch explicit.

The case-index manifest at [`./manifest.yaml`](./manifest.yaml) maps each
fixture to the contract sections it exercises.

