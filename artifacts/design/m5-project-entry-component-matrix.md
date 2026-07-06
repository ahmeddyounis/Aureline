# M5 Project-Entry Component Matrix

- Packet: `m5-project-entry-components:stable:0001`
- Label: `M5 Project-Entry Component Matrix`
- Scope: Start Center quick actions, recent work, workspace switcher, restore prompts, entry chooser, entry review, destination collisions, post-entry handoff, admission checkpoints, and archetype readiness routing.
- Canonical schema: `schemas/ui/m5-project-entry-component.schema.json`
- Fixture corpus: `fixtures/ui/m5-project-entry-components/component_matrix.json`
- Release proof: `artifacts/release/m5-project-entry-component-proof/packet.json`

This matrix freezes the reusable project-entry component family for M5. It does not mint new entry verbs, target kinds, resulting modes, trust states, restore-fidelity classes, or readiness buckets. Those values are re-exported from:

- `docs/workspace/entry_restore_object_model.md`
- `docs/ux/project_entry_contract.md`
- `docs/ux/recent_work_and_restore_card_contract.md`
- `docs/ux/workspace_admission_contract.md`
- `docs/ux/restore_fidelity_classes.md`
- `schemas/workspace/entry_and_restore_result.schema.json`
- `schemas/ux/entry_chooser_row.schema.json`
- `schemas/ux/open_flow_sheet.schema.json`
- `schemas/ux/recent_work_row.schema.json`
- `schemas/workspace/admission_checkpoint.schema.json`

## Controlled Vocabularies

Every component row carries the same shared axes:

| Axis | Canonical values |
|---|---|
| Entry verbs | `open`, `clone`, `import`, `add_root`, `restore`, `resume`, `start_from_snapshot` |
| Target kinds | `local_file`, `local_folder`, `local_repo_root`, `workspace_manifest`, `workset_manifest`, `remote_repository`, `ssh_workspace`, `container_workspace`, `devcontainer_workspace`, `managed_cloud_workspace`, `portable_state_package`, `handoff_packet`, `competitor_config_root`, `template_or_prebuild_snapshot`, `review_or_work_item_deep_link`, `recovery_checkpoint` |
| Resulting modes | `single_file`, `folder`, `repo_root`, `workspace_candidate`, `workspace_with_roots`, `workset_slice`, `inspect_only`, `clone_then_review`, `clone_then_open`, `clone_then_add`, `clone_only`, `extract_then_review`, `compare_before_restore`, `apply_to_active_workspace`, `open_prebuild_with_setup_actions`, `open_prebuild_minimal`, `resume_live_session`, `restore_last_session`, `restore_from_checkpoint` |
| Trust states | `trusted`, `restricted`, `pending_evaluation` |
| Trust postures | `trust_pending_until_admission`, `trust_never_implied_by_clone`, `trust_unchanged_until_admit`, `trust_per_root_admission`, `trust_inherited_from_target`, `trust_revalidated_at_resume` |
| Restore fidelity | `exact_restore`, `compatible_restore`, `layout_only`, `recovered_drafts`, `evidence_only`, `no_restore` |
| Readiness buckets | `blocking_now`, `recommended_soon`, `optional_later` |
| Blocked reasons | `blocked_by_trust`, `blocked_by_policy`, `blocked_by_missing_prerequisite`, `blocked_by_deployment_profile` |
| Optional reasons | `optional_quality_of_life`, `optional_bundle_enhancement`, `optional_account_or_sync`, `optional_docs_or_learning` |

## Component Families

| Component family | Schema | First consumers | Required user-visible truth | Downgrade states |
|---|---|---|---|---|
| `start_center_quick_action_card` | `schemas/ui/m5-start-center-quick-action-card.schema.json` | Start Center, docs/help, CLI/headless preview | Entry verb, target kind, resulting mode, account posture, trust posture | `required_for_this_row`, `unavailable_in_this_envelope`, `pending_evaluation` |
| `recent_work_row` | `schemas/ui/m5-recent-work-row.schema.json` | Start Center recent list, Open Recent, support export | Target state, trust state, restore fidelity, write-safety badge, recovery actions | `missing_target`, `remote_unreachable`, `cached_only`, `policy_blocked` |
| `workspace_switcher_entry` | `schemas/ui/m5-workspace-switcher-entry.schema.json` | Palette switcher, menu switcher, dedicated switcher | Same recent-work truth plus cross-window consequence | `authority_expired`, `suspended_managed_workspace`, `locked_by_other_instance` |
| `restore_prompt_card` | `schemas/ui/m5-restore-prompt-card.schema.json` | Restore flow, crash recovery, Start Center restore card | Restore fidelity, session summary, dirty-buffer count, safest action, `safe_mode` / `open_without_restore` / `clear_journal` / `export_evidence` affordances | `layout_only`, `recovered_drafts`, `evidence_only` |
| `entry_chooser_row` | `schemas/ui/m5-entry-chooser-row.schema.json` | Start Center, palette, drag/drop, CLI/headless, deep link | Verb-distinct chooser row with target-kind candidates, resulting-mode candidates, last-used or recommended destination, and keyboard equivalent | `reroute_required`, `target_kind_unresolved`, `policy_limited` |
| `entry_review_sheet` | `schemas/ui/m5-entry-review-sheet.schema.json` | Open, clone, import, resume review sheets | Literal target, normalized source locator, protocol/host/auth posture, resulting mode, write scope, post-open action, side effects, trust posture, and retained-input diagnostics before writes or remote contact | `review_required`, `trust_review_required`, `write_blocked` |
| `destination_collision_sheet` | `schemas/ui/m5-destination-collision-sheet.schema.json` | Clone, import, restore destination review | Collision class, collision source, existing target identity, `Reuse` / `Add existing` / `Clone elsewhere` / `Reveal` / inspect / cancel choices, and blocking choice before materialization | `existing_non_empty_path`, `existing_workspace_file`, `existing_local_root`, `duplicate_clone_target`, `policy_blocked_destination` |
| `post_entry_handoff_card` | `schemas/ui/m5-post-entry-handoff-card.schema.json` | Post-clone, post-import, post-restore, managed resume, template/prebuild entry, single-file/folder/repo open, review-link open | Opened object, entry source class, pending setup/trust tasks, intentionally-not-done work, recommended next action, `Set up later`, `Open minimal`, follow-up state, export/share state, admission checkpoint ref, first useful work route, and same-weight plain-open path | `setup_later`, `non_durable_staging`, `open_minimal_available`, `review_before_trust`, `compare_before_restore` |
| `admission_checkpoint_card` | `schemas/ui/m5-admission-checkpoint-card.schema.json` | Shell admission, project doctor, attention inbox, CLI/headless, support export | Root identity, trust class, archetype/bundle recommendation source, blocked-vs-optional readiness tasks, ordinary editing availability, and `Continue without` / `Set up later` choices | `policy_blocked`, `missing_prerequisite`, `needs_repair`, `trust_review_required` |
| `archetype_readiness_row` | `schemas/ui/m5-archetype-readiness-row.schema.json` | Admission checkpoint, first-useful-work router, docs/help | Archetype class, confidence class, evidence source, setup location, readiness bucket, blocked/optional reason | `missing_prerequisite`, `restricted`, `mixed`, `generic` |

## First Consumer Freeze

The first consumers for this matrix are:

| Consumer | Required behavior |
|---|---|
| `desktop_shell` | Renders the cards, rows, and sheets with the canonical component family and schema refs. |
| `start_center` | Uses quick-action, recent-work, restore, and entry-chooser records; it cannot collapse entry verbs into generic start copy. |
| `workspace_switcher` | Reuses recent-work and switcher-entry truth instead of private target-state labels. |
| `entry_review` | Uses entry-review and destination-collision sheets before any write, clone, import, restore, resume, or scope widening. |
| `admission_checkpoint` | Uses admission-checkpoint and archetype-readiness rows for blocked-vs-optional setup truth. |
| `cli_headless` | Exports the same target kind, resulting mode, trust, restore, and readiness tokens as desktop. |
| `deep_link` | Resolves into entry-chooser and entry-review records; deep links do not mint private trust or target labels. |
| `docs_help` | Quotes matrix component families and schema refs, not feature-local prose. |
| `support_export` | Carries opaque ids, schema refs, component family, canonical tokens, and redaction-safe labels. |
| `release_proof` | Certifies the matrix, fixture corpus, support export, and downgrade-state coverage. |

## Workspace Switcher Entry Truth

`workspace_switcher_entry` rows must preserve exact object identity and active
window posture. A conforming row includes the canonical object identity ref and
identity source, open-window state, selected profile ref, selected keymap ref,
local/remote/managed/imported/cached badges, dirty-session flag, dirty-buffer
count, restore badges, and the close/reopen/move action set. `locked_by_other_instance`
must render as an open-in-other-window case with `transfer_window` available;
remote or managed rows must show the remote/profile boundary before activation.

The close/reopen/move action set is part of the component contract, not local UI
chrome. It must include `close_window`, `reopen_previous_workspace`, and
`move_to_new_window`; rows add `open_in_new_window`, `transfer_window`,
`reconnect`, or `reauthorize` according to target state.

## Restore Prompt Card Truth

`restore_prompt_card` rows must project the same object identity and restore
vocabulary used by Start Center, crash recovery, manual switchers, support
diagnostics, and exports. A conforming card includes a redaction-safe session
summary, dirty-buffer count, canonical restore class label, partial/unsafe
reason tokens, safest next action, and visible affordances for `safe_mode`,
`open_without_restore`, `clear_journal`, and `export_evidence`.

The canonical labels are exactly `Exact restore`, `Compatible restore`,
`Layout only`, `Recovered drafts`, `Evidence only`, and `No restore`. Docs/help,
support exports, and release proof must quote those labels from this matrix and
must not substitute feature-local wording.

## Entry Chooser Row Truth

`entry_chooser_row` rows must preserve the literal entry verb selected by the
surface that invoked them. Start Center, command palette, drag-and-drop,
deep-link, and CLI/headless projections all carry the same `entry_verb_candidate`,
`target_kind_candidates`, `resulting_mode_candidates`,
`last_used_or_recommended_destination`, and `keyboard_equivalent` fields. A row
for `clone` cannot be rewritten as `open` because a local copy exists, an
`import` row cannot become `restore` because a packet includes restore metadata,
and `restore` remains the session-recovery verb rather than a generic start path.

## Entry Review Sheet Truth

`entry_review_sheet` rows are the confirmation boundary for `open`, `clone`,
`import`, and `resume`. A conforming sheet includes the redaction-safe literal
target label, normalized source locator, source-locator kind, protocol class,
host class, auth posture, resulting mode, write scope, post-open action,
destination disposition, side-effect truth, and retained-input diagnostics.

The side-effect truth is explicit before execution: repository hooks, dependency
restore, and trust widening are either disclosed or blocked as hidden work.
`no_hidden_hook_or_trust_widening_truth` must be true on every review sheet.
Failed attempts preserve typed retained-input refs, redacted error context,
repair actions, and retry posture so users can fix the attempt without
re-entering the target, destination, auth posture, or resulting-mode choices.

## Destination Collision Sheet Truth

`destination_collision_sheet` rows distinguish why the collision exists before
any clone, import, restore, or scope-widening write lands. The
`collision_source_class` values are `existing_local_root`,
`prior_workspace_state`, `duplicate_clone_target`, `existing_local_path`, and
`policy_blocked_destination`. A conforming row carries an opaque existing-target
identity ref, a redaction-safe existing-target label, `blocks_until_choice =
true`, and `overwrite_or_retry_copy_forbidden = true`.

The safe action set uses one vocabulary across local, remote, import, template,
prebuild, and restore flows: `reuse_existing`,
`add_existing_to_workspace`, `clone_elsewhere`,
`reveal_in_filesystem`, `inspect_only`, and `cancel_no_change`. A collision
sheet must never collapse those into generic overwrite, retry, or start copy.

## Post-Entry Handoff Card Truth

`post_entry_handoff_card` rows state what Aureline opened or staged and what it
intentionally did not do yet. A conforming card includes `opened_object_ref`,
`opened_object_label`, pending setup/trust task labels, intentionally-not-done
labels, `recommended_next_action`, `set_up_later_available = true`,
`open_minimal_available = true`, `follow_up_state_class`, and
`export_or_share_state`.

The follow-up state vocabulary is shared by deferred setup, inspect-only or
non-durable staging, safe reuse/add/clone-elsewhere, and plain editing:
`setup_deferred_durable`, `non_durable_staging`,
`safe_reuse_available`, `safe_add_existing_available`,
`safe_clone_elsewhere_available`, and `open_minimal_available`. These states
keep the later setup path recoverable while allowing ordinary editing or
inspect-only mode immediately.

## Admission Checkpoint Card Truth

`admission_checkpoint_card` rows recommend a wedge or setup path without
pretending certainty or monopolizing plain editing. A conforming card includes
the `root_identity_ref` and redaction-safe `root_identity_label`, the
`admission_class`, the trust class, the archetype-or-bundle
`recommendation_source`, an explicit `readiness_bucket_summary`, and a
`readiness_tasks` list whose per-task `readiness_bucket` values reconcile with
that summary. Every `blocking_now` task names a `blocked_reason_class` and every
`optional_later` task names an `optional_reason_class`, so blocking-now versus
recommended-soon versus optional-later work stays explicit and reviewable.

The `recommendation_source` vocabulary is
`certified_archetype_detection`, `probable_archetype_detection`,
`workflow_bundle_manifest`, `template_or_prebuild_manifest`, `policy_profile`,
and `no_recommendation`. A card cannot auto-install packs or hide uncertainty:
`continue_without_available` and `set_up_later_available` are always true, and
`checkpoint_actions` always includes `continue_without` and `set_up_later`.

## Archetype Readiness Row Truth

`archetype_readiness_row` rows state a detected archetype outcome with honest
confidence and evidence. The `detected_archetype_class` outcomes are
`certified` (certified match), `probable` (probable match), `mixed`
(mixed/ambiguous), `generic` (unknown/generic), `restricted`
(restricted/policy-blocked), and `missing_prerequisite`
(missing-toolchain/remote-prerequisite). Each row carries a `confidence_class`
(`high`, `medium`, `low`, `none`) and an `evidence_source_class`
(`repository_manifest_detected`, `lockfile_detected`,
`config_or_toolchain_file_detected`, `workflow_bundle_manifest`,
`policy_profile`, `remote_prerequisite_probe`, `no_local_evidence`).

`restricted` and `missing_prerequisite` rows sit in `blocking_now` and name a
`blocked_reason_class`; `generic` rows cannot overclaim confidence above `low`.

## First-Useful-Work Routing Truth

`post_entry_handoff_card` rows route first-useful-work differently for each
entry source while preserving a same-weight plain-open path. The
`entry_source_class` values are `single_file_open`, `folder_or_repo_open`,
`repo_clone`, `restore`, `review_link_open`, and `imported_handoff_packet`
(plus `template_or_prebuild_open` for prebuild entry). Plain opens
(`single_file_open`, `folder_or_repo_open`) route to `ordinary_editing`; clone,
restore, review-link, and import sources route to `review_before_trust`,
`compare_before_restore`, or `inspect_import` as appropriate.

Routing stays attributable to entry source and evidence rather than one
universal welcome tab: `plain_open_same_weight` and `open_minimal_available` are
always true, and the routes across sources never collapse into a single path.

## Evidence Expectations

Every component row must provide:

- A schema ref for the component family.
- At least one evidence ref in `fixtures/ui/m5-project-entry-components/component_matrix.json`.
- Consumer-surface coverage for desktop and at least one non-desktop projection (`cli_headless`, `deep_link`, `support_export`, `docs_help`, or `release_proof`).
- Canonical target-kind, trust, restore-fidelity, resulting-mode, and readiness fields, even when the component is rendered read-only or export-only.
- A copy contract proving the component uses canonical labels and forbids generic `Get started` wording.

## Non-Conforming Drift

The following changes must update this matrix and the fixture/proof packet in the same change:

- Adding or renaming an entry component family.
- Adding a target kind, resulting mode, trust state, restore-fidelity class, readiness bucket, blocked reason, or optional reason.
- Rendering `Open`, `Clone`, `Import`, `Add root`, `Restore`, `Resume`, or `Start from snapshot` through a generic entry row.
- Shipping a support export, CLI/headless output, deep-link review, or docs/help example with private target, restore, trust, or setup urgency wording.
