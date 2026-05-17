# Mutation-journal beta baseline report

This artifact is the reviewer-facing baseline rendering of the
mutation-journal report produced by the
[`mutation_journal`](../../../crates/aureline-reactive-state/src/mutation_journal/mod.rs)
module from the protected corpus under
[`/fixtures/state/mutation_journal_beta/`](../../../fixtures/state/mutation_journal_beta/).
It records the source lane, actor and authority attribution, entry
shape, recovery class, attribution and replayability state,
support-export posture, downgrade label, and open-gap classes for
every grouped journal entry in the beta corpus. The report stays
metadata-safe: it never carries raw payload bytes, raw private
material, or ambient authority, and every row is drawn from the
closed mutation-journal vocabularies.

Schema: `schemas/state/mutation_journal.schema.json`
(record kind `mutation_journal_report_record`, version 1).
Reviewer doc:
[`docs/state/m3/mutation_journal_beta.md`](../../../docs/state/m3/mutation_journal_beta.md).
Corpus manifest:
[`fixtures/state/mutation_journal_beta/manifest.yaml`](../../../fixtures/state/mutation_journal_beta/manifest.yaml).

## Matrix rows

| Entry ID | Consumer surface | Source lane | Actor | Authority | Entry kind | Paths | Recovery class | Attribution | Replayability | Downgrade label | Open-gap classes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `journal:ai_bulk_paste_checkpoint_only` | `incident_packet` | `ai_assistant` | `ai_agent` | `buffer_editor` | `single_file_write` | 1 | `checkpoint_restore` | `attributed` | `requires_manual_inspection` | `degraded_to_checkpoint_restore_only` | `compensation_class_pending` |
| `journal:ai_multifile_extract_method` | `support_bundle` | `ai_assistant` | `ai_agent` | `buffer_editor` | `multi_file_write` | 3 | `exact_undo` | `attributed` | `replay_ready` | `none` | `none` |
| `journal:ai_partial_attribution_tool_use` | `recovery_ladder` | `ai_assistant` | `ai_agent` | `buffer_editor` | `multi_file_write` | 2 | `compensation` | `partially_attributed` | `requires_manual_inspection` | `yellow_partial_attribution` | `attribution_pending` |
| `journal:automated_formatter_workspace_pass` | `recovery_ladder` | `automated_tooling` | `automated_tool` | `derived_knowledge` | `multi_file_write` | 4 | `regeneration` | `attributed` | `regenerate_only` | `none` | `none` |
| `journal:interactive_refactor_rename_symbol` | `doctor_probe` | `interactive_refactor` | `human_user` | `workspace_vfs` | `multi_file_write` | 5 | `compensation` | `attributed` | `replay_with_compensation` | `none` | `none` |
| `journal:tooling_unattributed_metadata_write` | `crash_report` | `automated_tooling` | `unknown_actor` | `provider_overlay` | `metadata_write` | 1 | `requires_user_resolution` | `unattributed` | `requires_manual_inspection` | `red_blocks_beta_row` | `attribution_pending` |

## Per-source-lane summary

| Source lane | Cases | Attributed | Partially attributed | Unattributed | Downgrade required |
| --- | --- | --- | --- | --- | --- |
| `ai_assistant` | 3 | 2 | 1 | 0 | 2 |
| `interactive_refactor` | 1 | 1 | 0 | 0 | 0 |
| `automated_tooling` | 2 | 1 | 0 | 1 | 1 |

## Per-recovery-class summary

| Recovery class | Cases | Replay ready | Replay with compensation | Regenerate only | Requires manual inspection |
| --- | --- | --- | --- | --- | --- |
| `exact_undo` | 1 | 1 | 0 | 0 | 0 |
| `compensation` | 2 | 0 | 1 | 0 | 1 |
| `regeneration` | 1 | 0 | 0 | 1 | 0 |
| `checkpoint_restore` | 1 | 0 | 0 | 0 | 1 |

The three downgraded rows
(`journal:ai_bulk_paste_checkpoint_only`,
`journal:ai_partial_attribution_tool_use`,
`journal:tooling_unattributed_metadata_write`) carry closed downgrade
labels (`degraded_to_checkpoint_restore_only`,
`yellow_partial_attribution`, `red_blocks_beta_row`) and at least one
closed open-gap entry (`compensation_class_pending`,
`attribution_pending`). Every aligned row declares
`downgrade_label = none` and no open gaps. The evaluator refuses any
deviation from these contracts.

## Open gaps

- `journal:ai_bulk_paste_checkpoint_only`
  (`compensation_class_pending`): the buffer editor did not preserve
  a byte-exact pre-image, so the journal can only offer checkpoint
  restore until a finer compensation path lands.
- `journal:ai_partial_attribution_tool_use`
  (`attribution_pending`): the AI actor and the buffer-editor
  authority were attributed but the specific tool identity was lost
  before the journal fielded the entry.
- `journal:tooling_unattributed_metadata_write`
  (`attribution_pending`): the crash-recovery journal could not
  attribute the actor or a named tool to this metadata write; the
  row is blocked from automatic recovery until attribution is
  restored or the user explicitly accepts the write.

## Safety baseline

- `raw_payload_excluded = true` on every support-export projection
  and on the report.
- `raw_private_material_excluded = true` on every case and on the
  report.
- `ambient_authority_excluded = true` on every case and on the
  report.
- `destructive_resets_present = false` on every case.
- `preserves_user_authored_files = true` on every case and on every
  support-export projection.

## Out-of-scope

- Live runtime measurement of mutation throughput or latency.
- Cross-tenant ticket routing — the report is consumed locally by
  the support-export pipeline and the chrome.
- Future managed support-center case-management features that are
  not required to make beta recovery, support, and export
  trustworthy.
- Adding new downgrade labels, open-gap classes, attribution states,
  replayability states, recovery classes, source lanes, or actor
  classes without updating the schema, the Rust module, the
  reviewer doc, this report, and the protected corpus together.
