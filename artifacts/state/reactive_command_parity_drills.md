# Reactive command parity drills

These drills walk each mutating surface from the user's request through
the canonical publication path — command commit, journal commit, reactive
publish — or, where the canonical outcome diverges, into an honest degraded
or waiting state. They are generated from the canonical packet in
[`crates/aureline-reactive-state/src/reactive_command_parity/mod.rs`](../../crates/aureline-reactive-state/src/reactive_command_parity/mod.rs)
and replayed by
[`crates/aureline-reactive-state/tests/reactive_command_parity.rs`](../../crates/aureline-reactive-state/tests/reactive_command_parity.rs).

Every drill asserts two properties: no surface claims published truth before
the reactive graph publishes, and the published or degraded state stays
correlatable with the command and mutation-journal lineage that produced it.
A divergence drill ends on the honest posture it can reach — `degraded_state`
or `waiting_state` — never an optimistic cache win.

## AI apply becomes truth only after command and journal commit

- **Drill id**: `drill.reactive_command_parity.ai_apply_publishes_after_commit`
- **Surface**: `ai_apply`
- **Exercised flow**: `ai_apply_edit`
- **Asserts no optimistic truth before publish**: `true`
- **Asserts lineage correlatable**: `true`
- **Final posture**: stage `reactive_published`, visibility `published_truth`

| Phase | Publication stage | State visibility | Step |
| --- | --- | --- | --- |
| `request` | `action_requested` | `pending` | The user requests a scoped apply; the edit set shows as a quarantined pending preview. |
| `pending` | `action_requested` | `pending` | The preview is gated; no buffer or tree node claims the edit as current truth. |
| `command_commit` | `command_committed` | `pending` | The apply command commits in the command graph. |
| `journal_commit` | `journal_committed` | `pending` | The mutation journal records the edit with actor, scope, command, and checkpoint lineage. |
| `publish` | `reactive_published` | `published_truth` | The reactive graph republishes the edited buffers and tree as current truth. |
| `verify` | `reactive_published` | `published_truth` | Diagnostics correlate the published edit with its command and journal lineage. |

The preview never claimed truth before publish; the edit became current only after the journal committed.

## Review action holds in waiting when the canonical outcome diverges

- **Drill id**: `drill.reactive_command_parity.review_action_holds_on_divergence`
- **Surface**: `review_action`
- **Exercised flow**: `review_approve_action`
- **Asserts no optimistic truth before publish**: `true`
- **Asserts lineage correlatable**: `true`
- **Final posture**: stage `diverged`, visibility `waiting_state`

| Phase | Publication stage | State visibility | Step |
| --- | --- | --- | --- |
| `request` | `action_requested` | `waiting_state` | A reviewer requests approve; the workspace shows a waiting state, never an optimistic approved flip. |
| `pending` | `action_requested` | `waiting_state` | Approve stays in waiting while the canonical path runs. |
| `command_commit` | `command_committed` | `waiting_state` | The approve command is accepted, but the merge base moved underneath it. |
| `diverge` | `diverged` | `waiting_state` | The canonical merge-queue outcome diverges from the request; the workspace keeps waiting instead of claiming approval. |
| `verify` | `diverged` | `waiting_state` | The waiting state stays visible with its command lineage rather than taking a hidden cache win. |

The divergence resolved to an explicit waiting state; the workspace never showed an approval the canonical path did not publish.

## Scaffold update reflects only the canonically published tree

- **Drill id**: `drill.reactive_command_parity.scaffold_update_publishes_canonical_tree`
- **Surface**: `scaffold_update`
- **Exercised flow**: `scaffold_update_artifact`
- **Asserts no optimistic truth before publish**: `true`
- **Asserts lineage correlatable**: `true`
- **Final posture**: stage `reactive_published`, visibility `published_truth`

| Phase | Publication stage | State visibility | Step |
| --- | --- | --- | --- |
| `request` | `action_requested` | `pending` | A scaffold update is requested; the explorer shows a pending in-flight cue. |
| `pending` | `action_requested` | `pending` | With the old optimistic write removed, no file appears before the command commits. |
| `command_commit` | `command_committed` | `pending` | The scaffold command commits in the command graph. |
| `journal_commit` | `journal_committed` | `pending` | The mutation journal records the written files with scope and checkpoint lineage. |
| `publish` | `reactive_published` | `published_truth` | The reactive tree republishes the new files as current truth. |
| `verify` | `reactive_published` | `published_truth` | The explorer shows exactly the files the journal recorded; no optimistic node remains. |

No file was shown before the journal committed; the explorer matched the canonical tree.

## Provider mutation degrades when the provider rejects the change

- **Drill id**: `drill.reactive_command_parity.provider_mutation_degrades_on_reject`
- **Surface**: `provider_mutation`
- **Exercised flow**: `provider_config_mutation`
- **Asserts no optimistic truth before publish**: `true`
- **Asserts lineage correlatable**: `true`
- **Final posture**: stage `diverged`, visibility `degraded_state`

| Phase | Publication stage | State visibility | Step |
| --- | --- | --- | --- |
| `request` | `action_requested` | `pending` | A provider config change is requested; the surface shows pending, not an applied config. |
| `pending` | `action_requested` | `pending` | No optimistic config value is shown while the command runs. |
| `command_commit` | `command_committed` | `pending` | The command commits, but the provider rejects the change. |
| `diverge` | `diverged` | `degraded_state` | The canonical outcome diverges; the provider surface degrades to an explicit failed-change state. |
| `verify` | `diverged` | `degraded_state` | The degraded state is support-correlatable with the command and journal lineage; no stale config remains. |

A rejected change degraded explicitly; the surface never claimed a configuration the provider refused.

## Notebook cell result becomes truth only after the journal commits

- **Drill id**: `drill.reactive_command_parity.notebook_result_publishes_after_commit`
- **Surface**: `notebook_result_mutation`
- **Exercised flow**: `notebook_execute_cell`
- **Asserts no optimistic truth before publish**: `true`
- **Asserts lineage correlatable**: `true`
- **Final posture**: stage `reactive_published`, visibility `published_truth`

| Phase | Publication stage | State visibility | Step |
| --- | --- | --- | --- |
| `request` | `action_requested` | `pending` | A cell run is requested; the cell shows a quarantined running cue. |
| `pending` | `action_requested` | `pending` | The running cue is gated; the prior output is not replaced with an optimistic result. |
| `command_commit` | `command_committed` | `pending` | The execution command commits in the command graph. |
| `journal_commit` | `journal_committed` | `pending` | The mutation journal records the execution result with actor, scope, and checkpoint lineage. |
| `publish` | `reactive_published` | `published_truth` | The reactive graph republishes the cell output as current truth. |
| `verify` | `reactive_published` | `published_truth` | Diagnostics correlate the published output with its command and journal lineage. |

The running cue never stood in for the result; the output became current only after the journal committed.

## Support repair reports recovery only after the journal commits

- **Drill id**: `drill.reactive_command_parity.support_repair_publishes_after_commit`
- **Surface**: `support_repair`
- **Exercised flow**: `support_repair_state`
- **Asserts no optimistic truth before publish**: `true`
- **Asserts lineage correlatable**: `true`
- **Final posture**: stage `reactive_published`, visibility `published_truth`

| Phase | Publication stage | State visibility | Step |
| --- | --- | --- | --- |
| `request` | `action_requested` | `waiting_state` | A support repair is requested; the surface shows waiting, never an optimistic repaired state. |
| `pending` | `action_requested` | `waiting_state` | The repair stays in waiting while the canonical path runs. |
| `command_commit` | `command_committed` | `waiting_state` | The repair command commits in the command graph. |
| `journal_commit` | `journal_committed` | `waiting_state` | The mutation journal records the repair with actor, scope, command, and checkpoint lineage. |
| `publish` | `reactive_published` | `published_truth` | The reactive graph republishes the repaired state as current truth. |
| `verify` | `reactive_published` | `published_truth` | The support packet correlates the published recovery with its command and journal lineage. |

The repair was reported only after the journal committed; waiting held until the recovery published.
