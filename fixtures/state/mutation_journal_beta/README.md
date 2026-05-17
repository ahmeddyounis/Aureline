# Mutation-journal beta corpus

Protected fixture corpus for the mutation-journal beta. Each fixture
is one `mutation_journal_case_record` bound to:

- one `source_lane` from the closed list:
  `ai_assistant`, `interactive_refactor`, `automated_tooling`,
  `manual_save`, `migration_replay`, `restore_pipeline`,
- one `actor_class` from the closed list:
  `human_user`, `ai_agent`, `automated_tool`, `system_service`,
  `unknown_actor`,
- one `authority_class` from the closed list (mirrors the
  reactive-state authority labels):
  `workspace_vfs`, `buffer_editor`, `derived_knowledge`, `execution`,
  `policy_entitlement`, `provider_overlay`,
- one `entry_kind` (`single_file_write`, `multi_file_write`,
  `directory_rename`, `metadata_write`, `derived_artifact_write`),
- one `recovery_class` (`exact_undo`, `compensation`,
  `regeneration`, `checkpoint_restore`,
  `requires_user_resolution`),
- one `attribution_state` and one `replayability_state` cross-checked
  by the evaluator against the recovery class and actor identity,
- one primary `consumer_surface` from the closed list:
  `incident_packet`, `support_bundle`, `doctor_probe`,
  `crash_report`, `recovery_ladder`,
- one `support_export` projection declaring the metadata-safe
  baseline (no raw payload bytes, no raw private material, no
  ambient authority), and
- one closed `downgrade_label` from the mutation-journal
  vocabulary.

A failing row downgrades using the closed `downgrade_label` list; no
ad-hoc vocabulary is admitted. Open gaps are drawn from the closed
`open_gap_class` enumeration so reviewer matrix entries stay
auditable.

Boundary schema:
[`schemas/state/mutation_journal.schema.json`](../../../schemas/state/mutation_journal.schema.json).

Crate consumer:
[`crates/aureline-reactive-state/src/mutation_journal/mod.rs`](../../../crates/aureline-reactive-state/src/mutation_journal/mod.rs).

Reviewer doc:
[`docs/state/m3/mutation_journal_beta.md`](../../../docs/state/m3/mutation_journal_beta.md).

Baseline report:
[`artifacts/support/m3/mutation_journal_beta_report.md`](../../../artifacts/support/m3/mutation_journal_beta_report.md).
