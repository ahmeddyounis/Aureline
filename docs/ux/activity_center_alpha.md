# Activity Center Alpha

The alpha activity center treats long-running or reviewable work as durable
rows, not as toast copy. Indexing, restore, install/update, task, and test work
share one row model with stable row and job ids, actor or subsystem identity,
phase/progress, chronology, exact reopen, actions, impact flags, and
support-export posture.

## Contract Artifacts

- `schemas/events/activity_row.schema.json` freezes the snapshot, row, and
  support-export record shapes.
- `schemas/support/git_review_event_alpha.schema.json` freezes the structured
  Git/review event family that activity rows may embed when branch, target,
  action, and exact-reopen identity matter.
- `crates/aureline-shell/src/activity_center/alpha.rs` is the first consuming
  shell implementation with in-memory and file-backed persistence.
- `crates/aureline-shell/src/activity_center/git_review.rs` is the first
  Git/review consumer and support-export projection.
- `fixtures/ux/activity_center_alpha/` carries the protected snapshot and
  support-export fixtures.
- `fixtures/review/git_activity_alpha/` carries the protected Git/review
  activity and support-export fixtures.
- `ci/check_activity_center_alpha.py` validates the checked-in contract,
  fixtures, runtime consumer, and export path.

## Row Rules

Every row must carry:

- stable `activity_row_id`, `durable_job_id`, and `canonical_event_id`;
- `job_family`, source subsystem, actor identity, and execution origin;
- state, partition, phase/progress, and chronology;
- an exact `reopen_target` that points at the durable row or authoritative
  object without replaying the original side effect;
- command-backed open-details action, plus cancel/retry/evidence/history
  actions where applicable;
- cost, policy, network, trust, provider, and recovery-impact flags;
- support/export fields when the row can leave the UI as a structured artifact.
- for Git/review rows, structured branch/head context, target identity,
  command-backed action identity, and exact reopen links.

Rows remain durable until resolved or archived. Completion, failure,
partial completion, cancellation, and supersession remain distinct states.

## Proof Path

The protected fixture shows:

- indexing and task rows in `current_work`;
- install/update and failed test rows in `needs_attention`;
- restore completion in `completed`;
- every row preserving exact reopen identity;
- install/update carrying policy/network/trust/recovery impact flags and an
  evidence link;
- test failure exposing retry as a new reviewed invocation while open-details
  remains a non-replay action;
- a support export that preserves structured row identity, family, state,
  evidence, and reopen refs without raw private material.

The Git/review protected fixture adds a local mutation row, a failed publish
row, and a review-workspace row. Each row carries the same structured
branch/target/action context that the support export includes, so support
bundles do not depend on rendered activity-center text.
