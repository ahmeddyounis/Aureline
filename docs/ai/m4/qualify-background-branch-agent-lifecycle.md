# Background Branch-Agent Lifecycle Qualification

This is the M4 stable qualification contract for any exposed background
branch-agent or long-running AI automation lane.

The canonical record is
[`schemas/ai/background-branch-agent-run.schema.json`](../../../schemas/ai/background-branch-agent-run.schema.json).
It composes with the frozen base lifecycle contract in
[`docs/ai/background_branch_agent_lifecycle.md`](../background_branch_agent_lifecycle.md)
and does not replace that contract's branch-agent session or review packet
vocabulary.

## Stable Run Object

Every governed branch-agent run carries one stable `stable_run_id` across:

- launch review sheet
- active run rows
- checkpoint rows
- drift and re-review drills
- operator takeover row
- completion review row
- cleanup rows
- support export and evidence packet refs

The run object records plan version, initiator, branch/worktree identity refs,
base commit ref, requested goal, current state, current execution locus, pending
approval refs, evidence refs, cancellation posture, and cleanup posture.

Execution locus is never collapsed into one generic success state. Local-only
current-worktree assist, isolated side worktree, side branch, ephemeral
workspace, and managed remote workspace remain separate values.

## Launch Review

Before execution begins, the launch review sheet must disclose:

- requested goal
- base branch or commit ref
- target branch/worktree or change-object identity
- tool and connector classes
- approval gates
- estimated cost/risk band
- admitted secret or credential scope
- stop conditions

Approval gates expire with the reviewed scope. The agent cannot reuse stale
approvals after drift.

## Checkpoints And Re-Review

Active rows and checkpoint rows expose elapsed time, milestone, environment
assumptions, pending approvals, and operator actions.

The stable milestone vocabulary is `planning`, `collecting_context`, `editing`,
`validating`, `awaiting_approval`, `blocked`, `failed`,
`ready_for_review`, `re_review_required`, `cancelled`, and
`operator_takeover`.

The drift drill set must cover:

- base branch advance
- provider/model route drift
- policy epoch change
- trust narrowing
- boundary expansion

Each drift case blocks further writes, preserves already-produced diff/logs,
checkpoints, and cited evidence, and requires fresh review or manual takeover.

## Takeover, Completion, And Cleanup

Cancellation must state whether compute stopped immediately, is finishing
safely, or continues only until a safe checkpoint. Cancellation and failure keep
review artifacts.

Manual takeover preserves branch identity, checkpoint lineage, tool-call
history, validation receipts, and pending-write disclosure.

Completion review must summarize diff, validation, evidence packet, compare to
base, cleanup options, and follow-up commands. Background branch agents cannot
self-merge or directly push protected destinations on the stable path.

Cleanup requires a preview before destructive branch/worktree removal and must
preserve evidence refs, checkpoint refs, and support-export refs for the
retention window.
