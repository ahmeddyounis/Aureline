# Save-participant fixtures

These fixtures anchor the record families defined by
[`/docs/execution/save_participant_and_fix_safety_contract.md`](../../../docs/execution/save_participant_and_fix_safety_contract.md)
and the schemas:

- [`/schemas/execution/save_participant_plan.schema.json`](../../../schemas/execution/save_participant_plan.schema.json)
- [`/schemas/execution/fix_safety_class.schema.json`](../../../schemas/execution/fix_safety_class.schema.json)

They are pre-implementation examples. IDs are opaque and chosen for
readability; they are not planning identifiers.

| Fixture | Schema | Record kind | Scenario |
|---|---|---|---|
| [`safe_format_on_save.yaml`](./safe_format_on_save.yaml) | `save_participant_plan` | `save_participant_plan_record` | Deterministic one-file formatter, validation, compare-before-write, durable write, and post-save refresh. |
| [`generated_companion_update.yaml`](./generated_companion_update.yaml) | `save_participant_plan` | `save_participant_plan_record` | Canonical source save triggers a generated companion update that requires lineage and preview posture. |
| [`blocked_whole_file_rewrite.yaml`](./blocked_whole_file_rewrite.yaml) | `save_participant_plan` | `save_participant_plan_record` | Whole-file formatter rewrite is disclosed and blocked pending review. |
| [`external_change_mid_save.yaml`](./external_change_mid_save.yaml) | `save_participant_plan` | `save_participant_plan_record` | Compare-before-write detects a newer external revision after participants ran. |
| [`preview_required_multi_file_fix.yaml`](./preview_required_multi_file_fix.yaml) | `save_participant_plan` | `save_participant_plan_record` | A lint fix may touch multiple workspace files and therefore opens batch preview. |
