# M5 AI-review scope-selector-state and rerun/outdated-freshness registries

Review-scope-selector and rerun/outdated-state implement lane over the frozen
[M5 AI-review-assist matrix][matrix] (`m5_ai_review_assist_matrix`). It makes the matrix's
`review_scope_selector` and `resolution_memory_row` object classes operable by carrying
resolved, honest projections of two registries so review, AI, provider, pending-review, and
support / export surfaces inherit one canonical model of *which diff an AI review run covered*
and *when that run went stale* — rather than hand-authored parallel prose that has to be kept
consistent. AI review scope stays explicit and truthful: a finding always names whether it came
from a selected diff, local uncommitted changes, or a hosted review object, and prior findings
are marked outdated / rerun-recommended instead of left falsely fresh once the diff drifts.

## Registry-A — review-scope-selector state

One machine-readable review-scope-selector state per AI review run, carrying:

- the analyzed review scope (`selected_diff`, `uncommitted_changes`, `pull_merge_request`,
  `base_head_range`, `staged_changes`, `saved_review_snapshot`) so a finding can never hide
  whether it came from selected lines, local uncommitted changes, or a hosted review object;
- the base / head context the run was pinned to;
- the repo-instruction / enabled-check-pack source that shaped the run;
- the current freshness and the rerun action offered within scope (no silent scope widening,
  no hidden cost or target change);
- the resolution-form coverage (canonical object, accessible summary, audit record).

A scope-selector state that cannot bind its scope to a classified review-scope kind, that is a
hand-copied per-entry assumption instead of tracing to the shared registry, or that publishes an
incomplete object degrades honestly instead of reading as a clean, current scope. The registry
reuses the matrix `m5-ai-review-scope-selector.schema.json` domain schema.

## Registry-B — rerun / outdated freshness diff

The typed freshness event that turns a prior finding stale when its analyzed scope no longer
matches the current target, keyed to which drift occurred:

- `analyzed_diff_changed` — the analyzed diff changed materially;
- `base_head_context_shifted` — the base / head context shifted under the run;
- `saved_snapshot_mismatch` — the saved review snapshot no longer matches the current target.

Each drift resolves the prior finding to `Outdated` or `Rerun recommended`
(see [`M5AiReviewAssistFindingLifecycle`][lifecycle]) instead of leaving it falsely fresh, and a
rerun preserves prior lineage while re-resolving current scope and freshness before new output
is shown. The registry reuses the matrix `m5-ai-review-resolution-memory.schema.json` domain
schema.

## Acceptance criteria proven by the resolved examples

1. Seeded diff churn or base / head drift turns prior findings into `outdated` /
   `rerun_recommended` through a typed freshness diff instead of leaving them falsely fresh; a
   freshness diff that hides the drift or runs support language ahead of its proof degrades.
2. Every resolved scope-selector state names its analyzed scope kind, so users can tell at a
   glance whether a finding came from selected lines, local uncommitted changes, or a hosted
   review object.
3. The same review check pack runs across local (`selected_diff`, `uncommitted_changes`,
   `staged_changes`) and hosted (`pull_merge_request`) scopes while the registry keeps each
   scope difference distinct in rows, packets, and history; a rerun preserves prior lineage.

Raw secret values and private endpoints never cross this boundary. The Rust validator in
`crates/aureline-ui` is the authoritative gate; the checked-in combined registries schema
(`schemas/review/m5-ai-review-scope-selector-and-rerun-state-registries.schema.json`) documents the
shape.

[matrix]: ../../crates/aureline-ui/src/m5_ai_review_assist_matrix/mod.rs
[lifecycle]: ../../crates/aureline-ui/src/m5_ai_review_assist_matrix/mod.rs
