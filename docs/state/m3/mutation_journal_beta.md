# Mutation-journal beta

This reviewer doc is the contract for the mutation-journal beta. The
beta promotes the mutation journal from anecdotal write-trail logging
to a typed projection that groups multi-file or tool-driven writes,
records a closed compensation class per group, and preserves the
replay-safe audit fields support and incident packets need to explain
what changed without re-reading the diff.

Each `mutation_journal_case_record` binds one grouped journal entry
to:

- one `source_lane` from the closed list:
  `ai_assistant`, `interactive_refactor`, `automated_tooling`,
  `manual_save`, `migration_replay`, `restore_pipeline`;
- one `actor_class` from the closed list:
  `human_user`, `ai_agent`, `automated_tool`, `system_service`,
  `unknown_actor`;
- one `authority_class` from the closed list (mirrors the reactive
  state authority labels):
  `workspace_vfs`, `buffer_editor`, `derived_knowledge`,
  `execution`, `policy_entitlement`, `provider_overlay`;
- one `entry_kind` from the closed list:
  `single_file_write`, `multi_file_write`, `directory_rename`,
  `metadata_write`, `derived_artifact_write`;
- one `group_size` plus an `affected_paths` list of repo-relative
  paths so reviewers and support can audit which files moved under
  this entry;
- one `recovery_class` from the closed list:
  `exact_undo`, `compensation`, `regeneration`,
  `checkpoint_restore`, `requires_user_resolution`;
- one `attribution_state` (`attributed`, `partially_attributed`,
  `unattributed`) the evaluator cross-checks against the declared
  actor;
- one `replayability_state` (`replay_ready`,
  `replay_with_compensation`, `regenerate_only`,
  `requires_manual_inspection`) the evaluator cross-checks against
  the recovery class and attribution state;
- one `consumer_surface` (`incident_packet`, `support_bundle`,
  `doctor_probe`, `crash_report`, `recovery_ladder`) recording the
  primary surface that quotes this entry;
- one `support_export` projection declaring the metadata-safe
  baseline (no raw payload bytes, no raw private material, no
  ambient authority) so the closed-vocabulary audit fields make it
  into support and incident packets without re-reading the diff; and
- one `downgrade_label` from the closed mutation-journal vocabulary
  (`none`, `red_blocks_beta_row`, `yellow_partial_attribution`,
  `yellow_recovery_class_unknown`,
  `degraded_to_checkpoint_restore_only`,
  `stale_corpus_blocks_release_candidate`).

Implementation:
[`crates/aureline-reactive-state/src/mutation_journal/mod.rs`](../../../crates/aureline-reactive-state/src/mutation_journal/mod.rs).
Boundary schema:
[`schemas/state/mutation_journal.schema.json`](../../../schemas/state/mutation_journal.schema.json).
Protected fixture corpus:
[`fixtures/state/mutation_journal_beta/`](../../../fixtures/state/mutation_journal_beta/).
Baseline report:
[`artifacts/support/m3/mutation_journal_beta_report.md`](../../../artifacts/support/m3/mutation_journal_beta_report.md).
Integration drill:
[`crates/aureline-reactive-state/tests/mutation_journal_beta.rs`](../../../crates/aureline-reactive-state/tests/mutation_journal_beta.rs).
First non-state consumer:
[`crates/aureline-support/src/mutation_journal/mod.rs`](../../../crates/aureline-support/src/mutation_journal/mod.rs).

## Why this lane exists

Earlier milestones recorded write-trail evidence ad hoc: individual
file writes were logged, but multi-file refactors, AI tool calls, and
automated tooling passes left a confusing trail of unrelated entries.
Support packets had to reconstruct what happened from raw diffs, and
recovery surfaces could not tell whether an entry was exact-undoable
or only restorable through a checkpoint. The mutation-journal beta
closes those gaps by:

1. grouping every material multi-file or tool-driven write under one
   entry,
2. attributing the group to actor, authority, and source lane,
3. recording one closed compensation class per group so reviewers and
   recovery surfaces know how the entry can be undone, and
4. pinning the audit fields support and incident packets quote so
   they can explain what changed without reconstructing it from raw
   diffs alone.

The evaluator then refuses claims like `replayability_state =
replay_ready` when attribution is missing or when the recovery class
is `checkpoint_restore`; reviewers see the gap in the matrix instead
of having to re-derive it from prose.

## Required coverage

The corpus seeds at least one case per required source lane
(`ai_assistant`, `interactive_refactor`, `automated_tooling`), at
least one case per recovery class (`exact_undo`, `compensation`,
`regeneration`, `checkpoint_restore`), at least one case per consumer
surface (`incident_packet`, `support_bundle`, `doctor_probe`,
`crash_report`, `recovery_ladder`), and at least one
`partially_attributed` and one `unattributed` case so the attribution
contract is exercised by fixtures and not anecdotes.

| Required class | Seeded by |
| --- | --- |
| `ai_assistant` | `ai_multifile_extract_method_case.yaml`, `ai_bulk_paste_checkpoint_restore_case.yaml`, `ai_partial_attribution_case.yaml` |
| `interactive_refactor` | `interactive_refactor_rename_case.yaml` |
| `automated_tooling` | `automated_formatter_pass_case.yaml`, `tooling_unattributed_case.yaml` |
| `exact_undo` | `ai_multifile_extract_method_case.yaml` |
| `compensation` | `interactive_refactor_rename_case.yaml`, `ai_partial_attribution_case.yaml` |
| `regeneration` | `automated_formatter_pass_case.yaml` |
| `checkpoint_restore` | `ai_bulk_paste_checkpoint_restore_case.yaml` |

## What the evaluator refuses

- `replayability_state = replay_ready` without
  `attribution_state = attributed`, or with `recovery_class` in
  `{checkpoint_restore, requires_user_resolution}`.
- `replayability_state = replay_with_compensation` without
  `recovery_class = compensation` or without
  `attribution_state = attributed`.
- `replayability_state = regenerate_only` without
  `recovery_class = regeneration`.
- `replayability_state = requires_manual_inspection` without either
  `recovery_class` in
  `{checkpoint_restore, requires_user_resolution}` or
  `attribution_state` in `{partially_attributed, unattributed}`.
- `attribution_state = attributed` with `actor_class =
  unknown_actor`.
- `attribution_state = unattributed` with a known `actor_class`.
- `downgrade_label = none` on a row whose attribution, recovery, or
  replayability fields do not form a clean triple (attributed +
  non-checkpoint/non-user-resolution recovery + non-manual
  replayability).
- `downgrade_label = yellow_partial_attribution` without
  `attribution_state = partially_attributed`.
- `downgrade_label = red_blocks_beta_row` without
  `attribution_state = unattributed`.
- `downgrade_label = degraded_to_checkpoint_restore_only` without
  `recovery_class = checkpoint_restore`.
- `downgrade_label = yellow_recovery_class_unknown` without
  `recovery_class = requires_user_resolution`.
- Aligned rows that carry any non-none `downgrade_label` or
  non-none `open_gap_class`.
- Downgraded rows that drop the closed downgrade label or fail to
  record at least one closed `open_gap`.
- Support-export projections that drop any of `entry_id`,
  `source_lane`, `actor_class`, `authority_class`, `recovery_class`,
  `replayability_state`, or `affected_paths`, or admit raw payload,
  raw private material, or ambient authority.
- Cases that drop preservation of user-authored files or declare
  `destructive_resets_present = true`.
- Corpora missing any required source lane, recovery class,
  consumer surface, or attribution-gap row.

## What this lane does NOT own

- Persisting the raw byte-level diff. The journal preserves audit
  truth in metadata-safe form; the underlying diff bytes remain owned
  by the buffer-editor / vfs producers under their own retention
  policies.
- Live transport for the subscription envelope — that contract lives
  in `aureline-reactive-state`'s alpha modules and ADR
  [`0005`](../../adr/0005-subscription-envelope-and-invalidation-semantics.md).
- New recovery classes or source lanes. Extending either lands as a
  coordinated schema, Rust module, fixture, and reviewer-doc patch.

## Out of scope

- Live runtime measurement of mutation throughput or latency.
- Cross-tenant ticket routing — the report is consumed locally by
  the support-export pipeline and the chrome.
- Future managed support-center case-management features that are
  not required to make beta recovery, support, and export
  trustworthy.
- Adding new downgrade labels, open-gap classes, attribution states,
  replayability states, recovery classes, source lanes, or actor
  classes without updating the schema, the Rust module, this
  reviewer doc, the baseline report, and the protected corpus
  together.
