# Restore prompt and deep-link entry contract

This page is the reviewer-facing entry point for Aureline's **restore prompt
contract** and **deep-link entry validator**. It describes the truth source
that resume, missing-target recovery, and reviewed deep-link entry surfaces
share — so the live shell never silently reruns a restore, never drops a
missing target from history, and never replays a deep link without a reviewed
sheet on boundary-raising actions.

The contract goal is honesty before rehydration:

- **Counts before rehydration.** The prompt names session scope, dirty-buffer
  count, restore level, safe-mode path, and clear-journal path before any
  pane is hydrated.
- **No silent rerun.** Side-effectful surfaces are skeletoned, not replayed.
  `auto_rerun_forbidden` rides on every prompt.
- **Locate / reconnect / remove / open-anyway.** Missing or stale targets
  keep their explicit recovery choices instead of being dropped from history.
- **Deep links are validated.** Origin, target class, and command class are
  inspected before a deep link can run; boundary-raising actions reopen a
  reviewed intent sheet rather than executing inline.

## Canonical truth source

The canonical builders live in:

- `crates/aureline-shell/src/restore/mod.rs` — `RestorePromptRecord`,
  `materialize_restore_prompt`, `RestorePromptChoiceKey`.
- `crates/aureline-shell/src/deeplink/mod.rs` — `DeepLinkIntent`,
  `validate_deep_link_intent`, `DeepLinkValidationOutcome`,
  `DeepLinkDenialClass`.

Both modules derive their inputs from already-shared workspace and recovery
truth (`aureline-recovery::session_restore::RestoreProposal`,
`aureline-workspace::entry_flows::EntryFlowOutcome`). Surfaces that need to
render or commit one of these flows MUST consume the same builder; pane-local
copies of restore vocabulary or deep-link admission rules are non-conforming.

## Restore prompt fields

A `RestorePromptRecord` always carries:

- `restore_class` — one of the controlled classes from
  [`crash_loop_and_restore_fidelity_contract.md`](./crash_loop_and_restore_fidelity_contract.md):
  `exact_restore`, `compatible_restore`, `layout_only`, `recovered_drafts`,
  `evidence_only`, `no_restore`.
- `session_scope` — counts for windows, tab groups, tabs.
- `dirty_buffer_count`, `missing_dependency_count`,
  `evidence_packet_count`, `recovery_packet_count`,
  `terminal_count`, `transient_task_count`.
- `prior_run_abnormal`, `auto_rerun_forbidden` (always `true`).
- `safe_mode_path` — a stable command id the surface routes the user to for
  safe-mode entry.
- `clear_journal_path` — a stable command id the surface routes the user to
  for clearing the journal (after explicit confirmation).
- `summary_line` — single-line restatement of the proposal summary.
- `notes` — passthrough notes from the proposal.
- `choices[]` — every offered route. Each choice carries `choice_key`,
  `enabled`, `forbidden_reason`, `requires_confirmation`, and a
  `journal_implication` token.

### Choice keys

Choice keys reuse the action grammar in
[`crash_loop_and_restore_fidelity_contract.md`](./crash_loop_and_restore_fidelity_contract.md):

| Choice key | Available when | Journal implication |
| --- | --- | --- |
| `restore_now` | proposal has at least one restorable layout, draft, or evidence-backed item | `restore_skeleton_only` |
| `skip_once` | a prompt is present | `no_change_for_this_launch` |
| `open_clean` | always available | `evidence_retained` |
| `compare_to_disk` | drafts present and target identity readable | `compare_only` |
| `open_journal` | journal entries exist and policy allows local inspection | `inspect_only` |
| `safe_mode` | the recovery shell is reachable | `safe_mode_entered` |
| `clear_journal` | journal entries exist | `journal_cleared_after_confirmation` |
| `open_logs` | logs are present | `inspect_only` |
| `export_evidence` | evidence packets exist | `export_only` |

`restore_now` is **never** auto-clicked. The prompt is honest: missing
dependencies, evidence-only downgrades, and side-effectful panes always
appear with their reason; `auto_rerun_forbidden` is exported with every
record.

## Deep-link validator fields

`DeepLinkIntent` carries the cross-tool subset of the platform deep-link
vocabulary that the shell admission gate inspects:

- `origin_class` — `os_shell`, `system_default_browser`, `first_party_web`,
  `trusted_companion`, `external_provider`, `collaboration_service`,
  `local_cli`, `installer_or_update_flow`, `unknown_untrusted`.
- `target_class` — `local_file`, `local_folder`, `workspace_root`,
  `recent_work_entry`, `review_thread`, `work_item`, `managed_workspace`,
  `command_target`, `unknown_target`.
- `command_class` — `inspect_only`, `reveal_only`, `open_existing_context`,
  `create_or_add_context`, `join_presence`, `resume_session`, `auth_return`,
  `retry_or_reconnect`, `acknowledge_notification`,
  `mutating_command_request`, `privileged_authority_widening`.
- `intent_id`, `route_label` (opaque), `replay_consumed` (bool).

`validate_deep_link_intent` returns one of:

- `Admitted { reviewed_sheet_required: bool, fallback }` — the shell may
  reopen the entry-flow sheet (when `reviewed_sheet_required` is `true`) or
  proceed with a non-mutating route directly.
- `Denied { denial_class, summary }` — the shell offers locate / reconnect /
  remove / open-anyway recovery routes instead of executing the intent.

### Denial classes

- `origin_unverified` — `unknown_untrusted` origin.
- `target_unresolved` — `unknown_target` target class.
- `replay_consumed` — `replay_consumed` is `true`.
- `boundary_raising_without_review` — combinations the validator refuses
  even after admission (e.g. `external_provider` + `mutating_command_request`
  without an admission ticket).

### Reviewed-sheet rules

A reviewed sheet is **required** when any of the following hold:

- `command_class` is `mutating_command_request` or
  `privileged_authority_widening`.
- `target_class` is `managed_workspace`, `review_thread`, `work_item`, or
  `command_target` (any boundary-raising target).
- `origin_class` is one of `external_provider`, `collaboration_service`,
  `installer_or_update_flow` (any non-local origin).

Otherwise the route is `Admitted { reviewed_sheet_required: false }` and the
shell may dispatch through the same entry-flow resolver Start Center uses —
never through a hidden shortcut.

## Live shell consumer

The first live consumer is the startup restore-proposal pathway in:

- `crates/aureline-shell/src/bootstrap/native_shell.rs`

On startup, after `RestoreProposal::build` runs the shell:

1. Materializes a `RestorePromptRecord` from the proposal.
2. Writes `.logs/recovery/restore_prompt_latest.json` for protected-row
   evidence capture.
3. Notes the prompt summary line through the command runtime so the live
   shell, status surface, and diagnostics packet share one truth.

Deep-link validation is exercised by the same module when an
`EntryFlowOutcome` arrives via a non-Start-Center surface (deep link,
default-browser callback). `validate_deep_link_intent` runs before the
existing entry-flow sheet overlay opens; on denial the overlay still opens
with the missing-target/recovery choice set so user intent is preserved
without silent execution.

## Fixtures and proof set

The minimal fixture set lives at:

- `fixtures/ux/restore_and_deeplink_cases/`

It includes:

- `restore_prompt_recovered_drafts.json` — drafts present after abnormal
  termination; safe-mode and clear-journal paths visible.
- `restore_prompt_no_restore_first_launch.json` — empty proposal; no
  prompt-driving counts but `auto_rerun_forbidden` and choices stay honest.
- `deeplink_admitted_workspace_open.json` — system-default-browser open of a
  local workspace root; admitted, no review required.
- `deeplink_review_required_managed_resume.json` — managed-workspace resume
  via deep link; admitted but reviewed-sheet required.
- `deeplink_denied_unknown_origin.json` — `unknown_untrusted` origin; denied
  with `origin_unverified`.

All fixtures round-trip through the canonical builders in unit tests.

## Failure drill

Trigger restore with a missing target or stale deep link:

- Restore: launch with `prior_run_abnormal=true` and a corrupt snapshot — the
  prompt downgrades to `evidence_only`, surfaces `clear_journal` and
  `safe_mode` paths, and never auto-restores.
- Deep link: feed a `DeepLinkIntent` with `unknown_target` or
  `replay_consumed=true` — validation returns `Denied`, and the shell offers
  the entry-recovery choice set instead of opening the wrong path.

## Out of scope

- Marketplace, account, or sign-in detours. Restore truth and deep-link
  validation are local-first.
- Remote-first managed-workspace resume beyond the validator's
  `reviewed_sheet_required` decision.
- Final visual layout for the prompt or deep-link sheet beyond the required
  fields and choice grammar.
