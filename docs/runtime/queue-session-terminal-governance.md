# Queue Session Terminal Governance

This document freezes the canonical runtime vocabulary for background-work
identity, restore continuity, and terminal-boundary semantics across notebook,
data, pipeline, preview, profiler, docs-recall, sync/offboarding, companion,
incident, and infrastructure surfaces.

The Rust contract is
[`crates/aureline-runtime/src/queue_session_terminal_governance/`](../../crates/aureline-runtime/src/queue_session_terminal_governance/),
the boundary schema is
[`schemas/runtime/queue-session-terminal-governance.schema.json`](../../schemas/runtime/queue-session-terminal-governance.schema.json),
and the reviewer matrix is
[`artifacts/runtime/queue-session-terminal-governance.md`](../../artifacts/runtime/queue-session-terminal-governance.md).

## Stable Vocabulary

- Queue identity: `queue_lane_class`, `collapse_key_class`,
  `budget_domain_class`, `checkpoint_policy_class`, `retry_class`,
  `cancellation_class`, and an opaque `queue_identity_ref`.
- Restore continuity: `restore_fidelity_class`,
  `no_hidden_rerun_class`, and an opaque `restore_anchor_ref`.
- Terminal boundary: `terminal_boundary_class`,
  `clipboard_posture_class`, and an opaque `boundary_ref`.
- Downgrade posture: `known_limit_class`, `downgrade_rule_class`,
  `support_class`, and `disclosure_ref` whenever a row narrows below stable.

## Stable Rules

- A stable workload claim must publish one queue identity row, one restore
  continuity row, and one downgrade rule row.
- Workloads that cross a runtime boundary must also publish one terminal
  boundary row; generic desktop continuity is not enough.
- Protected interaction stays reserved. Background-heavy workloads may not
  borrow the foreground budget implicitly.
- Restore preserves structure, evidence, transcripts, and honest rerun
  affordances, but never silently replays commands or reacquires authority.
- Clipboard and paste posture must preserve the active boundary class and
  policy outcome rather than relying on local heuristics or generic copy.
- Support export, release evidence, and docs/help reuse the same runtime
  vocabulary; they do not paraphrase boundary or restore semantics locally.

## Downgrade Rules

- Stale or missing queue identity proof narrows the workload row.
- Stale restore fidelity or missing no-hidden-rerun proof narrows the workload
  row.
- Stale terminal-boundary or clipboard proof narrows the workload row.
- Missing checkpoint proof or exhausted retry budgets narrow the workload row.
- Missing evidence blocks stable publication.
