# Mutation-journal and generated-artifact-lineage fixtures

These fixtures are short, reviewable scenarios that anchor the
vocabulary frozen in
[`/docs/workspace/mutation_lineage_model.md`](../../../docs/workspace/mutation_lineage_model.md)
and validated by the schemas at
[`/schemas/workspace/mutation_journal.schema.json`](../../../schemas/workspace/mutation_journal.schema.json)
and
[`/schemas/workspace/generated_artifact_lineage.schema.json`](../../../schemas/workspace/generated_artifact_lineage.schema.json).

Each fixture names the originator class it covers, the actor / source
/ undo / reversal classes it exercises, the checkpoint kinds it links,
and the worked-example section of the lineage document it motivates.

**Scope rules**

- Fixtures validate against one of the two mutation-journal or
  generated-artifact-lineage schemas; they do not encode wire bytes,
  ADR-0005 subscription envelopes, or ADR-0004 RPC envelopes.
- A new fixture MUST exercise at least one frozen actor class, one
  source class, one reversal class, or one generation class, and MUST
  cite the lineage-model section that motivates it.
- Monotonic timestamps and stable IDs are opaque; they are chosen to
  read well rather than to reflect any real clock or system state.
- Filesystem-identity records reuse the vocabulary frozen in
  [`/schemas/filesystem/save_target_token.schema.json`](../../../schemas/filesystem/save_target_token.schema.json);
  no identity fields are redefined here.

**Index**

| Fixture                                                            | Originator class            | Record kind                 | Key classes exercised                                                                                 | Doc section |
|--------------------------------------------------------------------|-----------------------------|-----------------------------|-------------------------------------------------------------------------------------------------------|-------------|
| [`typing_single_keystroke.json`](./typing_single_keystroke.json)   | `typing`                    | `mutation_journal_entry`    | `user_keystroke` / `human_local` / `text_edit` / `exact_undo`                                         | §4.1        |
| [`format_on_save_group.json`](./format_on_save_group.json)         | `format_on_save`            | `mutation_group_record`     | `save_participant` + `formatter_run` / `save_participant_group` / `compensating_undo`                 | §4.2        |
| [`decode_recovery_repair.json`](./decode_recovery_repair.json)     | `repair`                    | `mutation_journal_entry`    | `decode_recovery` / `decode_recovery_change` / `restore_from_checkpoint`                              | §4.3        |
| [`ai_patch_applied.json`](./ai_patch_applied.json)                 | `ai_patch_proposal`         | `mutation_group_record`     | `ai_apply` / `ai_hosted_provider` / `machine_generated_change` / `compensating_undo` + AI evidence    | §4.4        |
| [`build_output_mutation.json`](./build_output_mutation.json)       | `build_output`              | `mutation_journal_entry`    | `build_runner` / `disposable_derived` / `regenerate_or_recompute` (pairs with `build_output_lineage`) | §4.5        |
| [`build_output_lineage.json`](./build_output_lineage.json)         | `build_output`              | `generated_artifact_lineage_record` | `build_output` / `block_direct_edit` / `never_writable` / `rebuild_command`                   | §4.5        |
| [`preview_regeneration.json`](./preview_regeneration.json)         | `generated_preview_artifact`| `mutation_group_record`     | `preview_runtime` / `disposable_derived` / `regenerate_or_recompute` (pairs with `preview_snapshot_lineage`) | §4.6  |
| [`preview_snapshot_lineage.json`](./preview_snapshot_lineage.json) | `generated_preview_artifact`| `generated_artifact_lineage_record` | `preview_render_snapshot` / `inspect_only` / `never_writable` / `preview_runtime_refresh`     | §4.6        |
