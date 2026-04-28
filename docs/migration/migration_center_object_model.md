# Migration center object model

This document freezes the durable object model every migration-center,
first-run import, post-import review, issue handoff, and support-export
surface uses when it names **which source tool/version was imported**,
**which domains were selected**, **which restore checkpoint protects the
apply**, **which importer outcomes are authoritative**, **which shortcut
changes deserve their own digest**, and **which compatibility / report /
support refs carry the same migration truth across surfaces**.

Migration cannot stay trapped in wizard-local state. The migration
session and importer outcome packet defined here make switching
reviewable, exportable, and supportable even when the original UI is no
longer open.

Companion artifacts:

- [`/schemas/migration/migration_session.schema.json`](../../schemas/migration/migration_session.schema.json)
  — machine-readable schema for `migration_session_record`,
  `migration_shortcut_digest_record`, and `migration_restore_record`.
- [`/schemas/migration/importer_outcome.schema.json`](../../schemas/migration/importer_outcome.schema.json)
  — machine-readable schema for `importer_outcome_row_record` and
  `importer_outcome_packet_record`.
- [`/docs/migration/first_run_import_diff_and_rollback_contract.md`](./first_run_import_diff_and_rollback_contract.md)
  — first-run dry-run import plan, preview diff, apply, rollback, and
  imported-profile history contract.
- [`/schemas/migration/import_plan.schema.json`](../../schemas/migration/import_plan.schema.json),
  [`/schemas/migration/import_diff_preview.schema.json`](../../schemas/migration/import_diff_preview.schema.json),
  and [`/schemas/migration/import_rollback_checkpoint.schema.json`](../../schemas/migration/import_rollback_checkpoint.schema.json)
  — machine-readable first-run import plan, preview, and rollback
  checkpoint packet schemas.
- [`/fixtures/migration/import_preview_cases/`](../../fixtures/migration/import_preview_cases/)
  — worked cases for first-run full preview, partial import, skip path,
  rollback, and imported-profile history linkage.
- [`/docs/migration/source_ecosystem_coverage_matrix.md`](./source_ecosystem_coverage_matrix.md)
  — governed source ecosystem coverage matrix for the marketed migration
  lanes and their non-claimed source policy.
- [`/artifacts/migration/source_ecosystem_rows.yaml`](../../artifacts/migration/source_ecosystem_rows.yaml)
  — canonical source ecosystem row ids, import targets, caveats, owner
  allocation, proof burden, and downstream evidence refs.
- [`/artifacts/migration/quality_bar_rubric.yaml`](../../artifacts/migration/quality_bar_rubric.yaml)
  — canonical quality-bar vocabulary for migration lane claims.
- [`/fixtures/migration/source_profile_examples/`](../../fixtures/migration/source_profile_examples/)
  — source profile examples for the governed source ecosystem rows.
- [`/docs/migration/compatibility_scorecard_contract.md`](./compatibility_scorecard_contract.md),
  [`/schemas/migration/compatibility_scorecard.schema.json`](../../schemas/migration/compatibility_scorecard.schema.json),
  and [`/artifacts/migration/top_imported_workflow_rows.yaml`](../../artifacts/migration/top_imported_workflow_rows.yaml)
  — imported-extension, imported-workflow, and workflow-bundle
  compatibility scorecards that migration sessions and importer outcomes
  cite when a row is native, bridge-backed, partial, community-only,
  blocked, deprecated, or replaced.
- [`/schemas/commands/keybinding_resolver.schema.json`](../../schemas/commands/keybinding_resolver.schema.json)
  — keybinding-resolution, import-bridge fidelity, leader-overlay, and
  high-frequency shortcut-diff contract reused by the shortcut digest.
- [`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md)
  — entry-surface model for `project_entry_action_record`,
  `restore_prompt_record`, and the earlier `migration_result_record`.
- [`/docs/state/migration_and_restore_playbook.md`](../state/migration_and_restore_playbook.md)
  — shared restore-provenance and downgrade playbook reused by
  migration restore records.
- [`/schemas/release/compatibility_row.schema.json`](../../schemas/release/compatibility_row.schema.json)
  — compatibility-row contract reused through `compatibility_row_refs`.
- [`/schemas/support/support_packet_index.schema.json`](../../schemas/support/support_packet_index.schema.json)
  — support packet family index that now reserves migration-session and
  importer-outcome reference fields.

The two migration schemas carry embedded examples so reviewers can read
worked packets without chasing a separate fixture directory.

This contract is normative. Where it disagrees with the PRD, TAD, TDD,
or UI/UX spec, those documents win and this document plus the companion
schemas MUST be updated in the same change. Where a migration surface
invents a different outcome term, checkpoint story, or export field,
this document wins and the surface is non-conforming.

## Why this exists in addition to the entry/restore model

[`migration_result_record`](../workspace/entry_restore_object_model.md)
answers the entry-surface question: what happened when the user chose
`Import` from Start Center, first run, or another entry route?

The migration-center model answers a different question: what is the
durable migration truth after that entry route exists?

That distinction matters because the same migration may later be:

- reviewed again in a migration-center history view,
- reopened from a restore checkpoint,
- exported to support,
- attached to a docs/help or issue-template flow, or
- compared against a compatibility report or bundle recommendation.

Those later surfaces need stable ids and stable packets, not UI-local
ephemera.

## Scope

- Freeze one `migration_session_record` covering source tool/version,
  target profile/workspace, selected domains, actor, compatibility
  linkage, restore linkage, and machine-readable report/export fields.
- Freeze one `importer_outcome_row_record` whose primary outcome
  vocabulary is closed: `imported`, `mapped`, `skipped`,
  `manual_review`, `bridge_required`, `unsupported`.
- Freeze one `importer_outcome_packet_record` that groups outcome rows
  and reserves typed slots for parity scores, equivalence-map rows,
  post-import validation refs, recommended-bundle handoff, and
  migration-report/export linkage.
- Freeze scorecard refs as the compatibility bridge between importer
  outcomes and public claims for imported extensions, imported workflows,
  and workflow-bundle handoff rows.
- Freeze one `migration_shortcut_digest_record` so high-frequency
  shortcut deltas do not disappear inside a general settings diff.
- Freeze one `migration_restore_record` so restore checkpoints remain
  inspectable after apply, partial apply, downgrade, or support export.

## Out of scope

- The full migration executor, importer UI, or shipping compatibility
  bridge implementation.
- Final copy or badge wording; this document freezes machine vocabulary,
  not product microcopy.
- The full docs/help page model, support packet body, or compatibility
  report renderer. Those systems are linked here by stable refs rather
  than redefined.

## 1. Migration session

Every durable migration flow emits exactly one
`migration_session_record`. The session exists before apply, survives
partial apply or restore, and remains the canonical id quoted by docs,
support, and issue-template flows.

Minimum fields:

| Field | Meaning |
|---|---|
| `source_descriptor` | source tool family, display name, version, and staged artifact refs |
| `target_descriptor` | target profile/workspace identity plus apply scope |
| `selected_domains` | which domains the user actually chose |
| `session_state` | session lifecycle from `source_selected` through `applied`, `restored`, or `failed` |
| `actor` | who initiated or resumed the session and from which surface |
| `compatibility_report_ref` | machine-readable compatibility-report ref for the session |
| `compatibility_row_refs` | stable compatibility-row ids the session depends on |
| `restore_record_ref` | restore-checkpoint linkage once apply begins |
| `outcome_packet_ref` | grouped importer outcomes once apply completes or is partially applied |
| `migration_report_ref`, `issue_template_refs`, `export_refs`, `support_packet_refs` | machine-readable handoff/export linkage |

Rules:

1. `compatibility_report_ref` and `compatibility_row_refs` are not
   optional storytelling sugar. They are the compatibility linkage seed
   that keeps migration truth attached to the same report/row model used
   elsewhere in the repository.
2. A session MAY exist before apply with only preview refs. It MUST gain
   `restore_record_ref` before `session_state` reaches `applying`.
3. A session in `applied`, `partially_applied`, or `restored` state MUST
   also carry `outcome_packet_ref`.
4. Later preview and validation packets attach by ref; the session does
   not collapse into one monolithic wizard blob.

## 2. Importer outcome rows

Every imported or evaluated object gets exactly one
`importer_outcome_row_record`. The primary outcome vocabulary is closed:

| Outcome | Meaning |
|---|---|
| `imported` | the destination can carry the source object without a semantic narrowing beyond documented canonicalization |
| `mapped` | Aureline found a semantic or capability-equivalent destination object |
| `skipped` | the source object was intentionally left unchanged or left at destination truth |
| `manual_review` | a human must decide before Aureline may claim the migration complete for this object |
| `bridge_required` | the object depends on an explicit compatibility bridge; native parity is not claimed |
| `unsupported` | the destination does not support the source concept and Aureline must say so plainly |

Required row fields:

- `source_object_ref`
- `target_object_ref` when an imported or mapped destination exists
- `outcome_state`
- `reason_class`
- `confidence_class`
- `docs_help_refs`

Rules:

1. The vocabulary is closed. A surface that renders `approximate`,
   `translated`, or `best effort` instead of one of the six controlled
   states is non-conforming at this layer.
2. `bridge_required` is an end state, not a hidden implementation note.
   The row MUST carry a `bridge_requirement`.
3. `unsupported` is an end state, not a folded subtype of `skipped`.
4. `docs_help_refs` are mandatory so review, docs, and support can point
   back to the same explanatory material.

## 3. Importer outcome packet

The packet groups outcome rows under one stable id. It is the packet
docs, support exports, post-import review, and issue templates should
quote when they need the whole migration outcome instead of one row.

The packet reserves:

- `parity_score_refs` for category-specific parity scores;
- `equivalence_map_row_refs` for machine-readable source-to-target
  mapping rows;
- `post_import_validation_refs` for validator outcomes after apply;
- `recommended_bundle_ref` for honest bundle handoff; and
- `migration_report_ref`, `issue_template_refs`, `export_refs`, and
  `support_packet_refs` for downstream linkage.

Rules:

1. `outcome_summary` always carries all six outcome counters. Even when
   the value is zero, `bridge_required` and `unsupported` remain visible.
2. If any row is `bridge_required` or `unsupported`, the packet MUST
   carry `export_refs` and `support_packet_refs`. That prevents those
   states from disappearing during export or support handoff.
3. Parity scores stay category-specific. A strong shortcut or theme
   score may not hide a weak extension or launch/debug result.

## 4. Shortcut digest

Shortcut migrations are special because preserving muscle memory is not
the same thing as preserving raw configuration text.

`migration_shortcut_digest_record` therefore keeps four durable digest
buckets plus one dedicated post-import shortcut-diff row set:

- `imported_command_rows`
- `remapped_gesture_rows`
- `unresolved_conflict_rows`
- `high_frequency_diff_rows`
- `muscle_memory_risk_notes`

Rules:

1. High-frequency shortcut changes remain separately addressable from the
   rest of the migration diff.
2. `high_frequency_diff_rows`, when present, quote the frozen
   import-bridge fidelity class and resulting resolver layer from
   [`/schemas/commands/keybinding_resolver.schema.json`](../../schemas/commands/keybinding_resolver.schema.json)
   rather than flattening every shortcut change into "imported" or
   "remapped".
3. Conflicted or unsupported shortcuts remain visible even when the
   broader migration succeeded.
4. The digest is exportable on its own because shortcut regressions are
   a common support and workflow-adoption issue.

## 5. Restore record

Every migration apply path that mutates durable truth creates one
`migration_restore_record`.

Minimum fields:

- `checkpoint_ref`
- `created_at`
- `scope`
- `availability_state`
- `restore_action_hints`
- `cleanup_state`

Rules:

1. No apply occurs before a restore checkpoint exists.
2. Restore records survive partial apply and downgrade; they are not
   success-only receipts.
3. The record may point to shared restore provenance via
   `restore_provenance_ref`, but it does not collapse into the
   restore-provenance record because migration also needs checkpoint
   scope, availability, and cleanup truth.

## 6. Compatibility, docs, and support linkage

The migration-center seed integrates with existing repository contracts
instead of minting one more private linkage system.

| Need | Stable field(s) |
|---|---|
| compatibility linkage | `compatibility_report_ref`, `compatibility_row_refs` |
| docs/help linkage | `docs_help_refs` |
| issue-template linkage | `issue_template_refs` |
| machine-readable export linkage | `export_refs`, `migration_report_ref` |
| support handoff linkage | `support_packet_refs` |
| existing entry-surface linkage | `entry_action_ref`, `entry_restore_result_ref` |

Rules:

1. The migration session is the top-level durable id.
2. The outcome packet is the grouped outcome artifact.
3. Shortcut digest and restore record remain first-class linked
   companions, not nested footnotes.
4. Support exports and docs should point to the same ids/refs the local
   migration center uses.

## 7. Operational rules

1. Migration stays diff-first and reviewable. The durable packet model
   exists to keep that reviewability alive after the original UI closes.
2. The model supports preview, apply, post-import validation, restore,
   support export, and issue handoff without treating migration as a
   wizard-only concept.
3. Bridge-required and unsupported states remain explicit everywhere.
   Aureline may help the user continue, but it may not imply native
   parity where only a bridge or manual workaround exists.
