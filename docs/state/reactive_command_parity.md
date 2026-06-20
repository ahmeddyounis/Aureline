# Command and mutation-journal publication parity

This document describes the publication-parity contract for mutating product
surfaces. The canonical packet is implemented in
[`crates/aureline-reactive-state/src/reactive_command_parity/mod.rs`](../../crates/aureline-reactive-state/src/reactive_command_parity/mod.rs)
and serialized to
[`artifacts/state/reactive_command_parity.json`](../../artifacts/state/reactive_command_parity.json).

It builds on the subscription envelope and invalidation semantics frozen in
[`docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`](../adr/0005-subscription-envelope-and-invalidation-semantics.md),
the boundary schema at
[`schemas/runtime/subscription_envelope.schema.json`](../../schemas/runtime/subscription_envelope.schema.json),
the mutation-journal contract at
[`schemas/state/mutation_journal.schema.json`](../../schemas/state/mutation_journal.schema.json),
and the command descriptor at
[`schemas/commands/command_descriptor.schema.json`](../../schemas/commands/command_descriptor.schema.json).

## Why this exists

The product spans the shell, search, graph, review, docs, AI, preview, support,
and companion-adjacent surfaces, and several of those surfaces *mutate* state — AI
apply, review actions, scaffold/update, provider mutation, notebook/result
mutation, and support repair. The loophole this packet closes is the surface
that **looks correct locally because it optimistically updated itself even
though the canonical command and mutation journal say otherwise**. Without one
publication-parity contract each mutating surface could keep a private
optimistic cache that diverges from the command, approval, or journal outcome.

## The contract

A parity flow is keyed by a mutating surface and the kind of mutation it
performs. Every flow publishes user-visible state changes through the one
canonical path and never from a private cache:

- **Mutating surface** — `ai_apply`, `review_action`, `scaffold_update`,
  `provider_mutation`, `notebook_result_mutation`, `support_repair`.
- **Mutation kind** — `apply_edit`, `approve_action`, `scaffold_artifact`,
  `provider_config_change`, `execute_cell`, `repair_state`.
- **Optimistic posture** — `never_optimistic`, `optimistic_removed`,
  `optimistic_quarantined`. A removed or quarantined path shows an explicit
  `pending` cue; a never-optimistic surface shows an explicit `waiting_state`.
  None ever shows `published_truth` before publication.
- **Publication stage** — `action_requested`, `command_committed`,
  `journal_committed`, `reactive_published`, `diverged`. User-visible truth is
  only ever shown at `reactive_published`.
- **State visibility** — `pending`, `published_truth`, `degraded_state`,
  `waiting_state`.
- **Divergence resolution** — `degrade_surface`, `hold_and_wait`,
  `revert_to_canonical`.

Each flow also declares the lineage it preserves (`preserved_lineage`), a
support-safe `publication_summary`, and a `parity_rationale`.

## Invariants

1. User-visible state on a mutating surface is published only after the command
   graph commits, the mutation journal commits, and the reactive graph
   republishes. `publishes_after_command_commit`,
   `publishes_after_journal_commit`, and `publishes_via_reactive_graph` are all
   true; `claims_success_before_publish` is always false.
2. No mutating surface keeps a private optimistic cache that can outvote the
   canonical command, approval, or journal outcome. Optimistic paths are never
   offered as truth, removed, or quarantined behind the publication gate.
3. Every published state preserves actor, scope, command, and checkpoint lineage
   so diagnostics and support packets can reconstruct what the user saw and when.
4. Known divergence cases convert to an explicit `degraded_state` or
   `waiting_state` instead of a hidden cache win.
5. Each mutating surface publishes through the one canonical reactive path
   instead of inventing a private epoch or stale-state language.

The optimistic posture and the pre-publish visibility must agree so the
in-flight state is legible: `never_optimistic` ⇒ `waiting_state`,
`optimistic_removed` ⇒ `pending`, `optimistic_quarantined` ⇒ `pending`.

## Drills

The packet drills each mutating surface from the user's request through publish
or honest divergence — see
[`artifacts/state/reactive_command_parity_drills.md`](../../artifacts/state/reactive_command_parity_drills.md):

- **AI apply** marks the edit current only after the apply command and journal
  commit; the inline preview is a gated prediction, never standalone truth.
- **Review action** holds in an explicit waiting state when the canonical
  merge-queue outcome diverges, instead of flipping to an optimistic approval.
- **Scaffold update** shows files only after the journal records them; the old
  optimistic tree write was removed.
- **Provider mutation** degrades to an explicit failed-change state when the
  provider rejects a change rather than keeping a stale optimistic config.
- **Notebook result** publishes the cell output only after the journal commits;
  the running cue never stands in for the result.
- **Support repair** reports recovery only after the journal commits; a failed
  repair degrades rather than reporting a cache win.

## Consumers

The canonical packet is mirrored by the metadata-safe support export in
[`crates/aureline-support/src/reactive_command_parity/mod.rs`](../../crates/aureline-support/src/reactive_command_parity/mod.rs)
so support and diagnostics surfaces quote the same optimistic posture,
divergence resolution, and preserved lineage that the state packet freezes.
Product surfaces (`crates/aureline-ai`, `crates/aureline-review`,
`crates/aureline-scaffold`, `crates/aureline-provider`, `crates/aureline-notebook`,
`crates/aureline-support`) are listed per flow as `consumer_refs` and should
ingest this packet rather than inventing local optimistic-state wording.
