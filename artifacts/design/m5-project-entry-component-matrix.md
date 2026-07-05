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
| `entry_chooser_row` | `schemas/ui/m5-entry-chooser-row.schema.json` | Start Center, palette, drag/drop, CLI/headless, deep link | Verb-distinct chooser row with target and resulting-mode candidates | `reroute_required`, `target_kind_unresolved`, `policy_limited` |
| `entry_review_sheet` | `schemas/ui/m5-entry-review-sheet.schema.json` | Open, clone, import, resume review sheets | Target, resulting mode, destination disposition, side effects, trust posture before writes | `review_required`, `trust_review_required`, `write_blocked` |
| `destination_collision_sheet` | `schemas/ui/m5-destination-collision-sheet.schema.json` | Clone, import, restore destination review | Collision class, safe actions, blocking choice before materialization | `existing_non_empty_path`, `existing_workspace_file`, `policy_blocked_destination` |
| `post_entry_handoff_card` | `schemas/ui/m5-post-entry-handoff-card.schema.json` | Post-clone, post-import, post-restore, managed resume | Admission checkpoint ref and first useful work route | `setup_later`, `review_before_trust`, `compare_before_restore` |
| `admission_checkpoint_card` | `schemas/ui/m5-admission-checkpoint-card.schema.json` | Shell admission, project doctor, attention inbox, CLI/headless, support export | Admission class, blocked-vs-optional buckets, ordinary editing availability | `policy_blocked`, `missing_prerequisite`, `needs_repair` |
| `archetype_readiness_row` | `schemas/ui/m5-archetype-readiness-row.schema.json` | Admission checkpoint, first-useful-work router, docs/help | Archetype class, setup location, readiness bucket, blocked/optional reason | `missing_prerequisite`, `restricted`, `mixed` |

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
