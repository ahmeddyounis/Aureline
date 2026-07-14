# M5 staged-trust and post-open bootstrap-queue registries

This lane is the staged-trust + post-open-queue implement lane over the frozen
[M5 repository-bootstrap matrix](./m5_repository_bootstrap_contract.md). It turns the *staged-trust* grammar
(how Aureline browses the tree, manifests, and docs and computes safe metadata before any repo-owned hook,
task, extension recommendation, package restore, submodule init, LFS hydrate, or generator install can run) and
the *post-open bootstrap-queue* grammar (typed, attributable work objects that run repo-owned code, hydrate
network-backed content, mutate the reviewed checkout, or merely recommend) into registry resolvers that produce
export-safe, honest projections, so the acquisition, git, trust, diagnostics, docs, CLI, and support surfaces
resolve one canonical staging and queue truth instead of a per-entry, hand-copied reconstruction. The staged
trust and the post-open queue are separated in runtime and serialized state: the browse-scope reference, the
computed-metadata reference, the deferred repo-owned action set, the trust-prompt policy, the explicit-approval
reference, and the staged-trust provenance live on the staged trust, while the queue-item kind, the execution
site, the trust consequence, the network consequence, the approval requirement, and the attribution reference
live on the post-open queue, and no protected queue item may auto-execute during acquisition so repository open
stays useful before any repo-owned action runs.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_staged_trust_and_post_open_queue_registries` (the authoritative validator).
- **Combined schema:**
  `schemas/workspaces/m5-staged-trust-and-post-open-queue-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/workspaces/m5-checkout-plan.schema.json`](../../schemas/workspaces/m5-checkout-plan.schema.json)
  (checkout topology and submodule / LFS hydration reviewed before mutation) and
  [`schemas/workspaces/m5-bootstrap-evidence.schema.json`](../../schemas/workspaces/m5-bootstrap-evidence.schema.json)
  (staged-trust and post-open-queue evidence) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-staged-trust-and-post-open-queue-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:**
  `fixtures/workspaces/m5-staged-trust-and-post-open-queue-registries/`
  (`deferred_hydrate_beta_narrowed.json`, `trust_prompt_preview_narrowed.json`).

## Two registries

1. **Staged trust** (`resolve_staged_trust_entry`) — publishes one stable staged-trust object per acquisition
   path: the trust-stage kind and canonical trust mode, the browse-scope reference, the computed-metadata
   reference, the deferred repo-owned action set, the trust-prompt policy, the explicit-approval reference, and
   the staged-trust provenance. A clean entry names a canonical registry token, a classified trust stage, and a
   repository-bootstrap role, covers the canonical / accessible / audit resolution forms, publishes a complete
   object, keeps the stage browse-safe before it widens trust, and records an explicit approval before any
   trust-widening stage. Otherwise it degrades honestly — a stage that would run a repo-owned action or widen
   trust before browse-safe metadata is computed and an explicit approval is recorded degrades to
   `staged_trust_runs_repo_owned_action_implicitly_or_widens_trust_early`.
2. **Post-open queue** (`resolve_post_open_queue_entry`) — keeps the post-open bootstrap queue safe. A clean
   entry names a classified queue class and provides the complete queue-item-kind / execution-site /
   trust-consequence / network-consequence / approval-requirement / attribution queue object; a protected item
   that would auto-execute during acquisition, run ungated without an explicit approval or policy, or hide what
   it would run and where degrades to `post_open_queue_item_executes_implicitly_or_hides_consequence`.

## Per-item post-open-queue reference

The queue class carries whether it is protected, and the resolver publishes the full queue object, so the
registry — never a hand-copied per-entry assumption — is the single source of truth.
`staged_trust_object_is_complete` rejects an object missing any field, `staged_trust_stays_browse_safe` rejects a
stage that widens trust before browse-safe metadata is computed, and `post_open_queue_item_holds_for_approval`
rejects an item that auto-executes during acquisition or runs a protected step ungated.

| queue class | queue item kind | execution site | trust consequence | network consequence | approval requirement |
| --- | --- | --- | --- | --- | --- |
| runs_repo_owned_code | `queue-item.repo-hook-or-task` | `site.worktree` | `consequence.widens-trust-runs-code` | `consequence.offline` | `approval.explicit-required` |
| hydrates_network_backed_content | `queue-item.submodule-init-or-lfs-hydrate` | `site.network` | `consequence.widens-trust` | `consequence.hydrates-network` | `approval.explicit-required` |
| mutates_reviewed_checkout | `queue-item.index-warmup-or-docs-import` | `site.git-dir` | `consequence.mutates-checkout` | `consequence.offline` | `approval.policy-allowed` |
| inert_recommendation | `queue-item.bundle-recommendation-or-trust-prompt` | `site.presentation-only` | `consequence.no-trust-change` | `consequence.offline` | `approval.none-inert` |

A protected item that auto-executes during acquisition degrades to
`post_open_queue_item_executes_implicitly_or_hides_consequence`, an incomplete staged-trust object degrades to
`staged_trust_object_incomplete`, and a trust widened before browse-safe metadata is computed degrades to
`staged_trust_runs_repo_owned_action_implicitly_or_widens_trust_early`, so an implicitly-executing hook, an
incomplete object, or an early trust widening can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Repository open remains useful before repo-owned actions run, and no protected queue item executes
  implicitly during acquisition.** Clean staging entries cover the canonical browse-tree-and-manifests /
  compute-safe-metadata / review-deferred-repo-actions / run-repo-owned-action-after-approval /
  hydrate-network-content-after-approval stages and the first shell / entry / diagnostics / admin / support
  surfaces, an object-incomplete example degrades, an implicit-repo-action example degrades, and no clean
  staging entry widened trust early or published an incomplete object.
- **Bootstrap queue rows identify exactly what would run, where it would run, and what trust or network
  consequence it carries.** Clean queue entries cover the runs-code / hydrates-network / mutates-checkout /
  inert-recommendation classes with full resolution-form coverage while providing the complete queue object.
- **Tests fail when a repository hook/task/extension or hydration step runs merely because a path was opened or
  cloned.** A queue item that auto-executes during acquisition degrades, a form-incomplete example degrades, and
  no clean queue entry auto-executes or is missing the complete queue object.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_staged_trust_and_post_open_queue_registries -- support-export
cargo run -p aureline-ui --example dump_m5_staged_trust_and_post_open_queue_registries -- csv
cargo run -p aureline-ui --example dump_m5_staged_trust_and_post_open_queue_registries -- report
cargo run -p aureline-ui --example dump_m5_staged_trust_and_post_open_queue_registries -- post-open-queue-table
cargo run -p aureline-ui --example dump_m5_staged_trust_and_post_open_queue_registries -- fixture-deferred-hydrate-beta-narrowed
cargo run -p aureline-ui --example dump_m5_staged_trust_and_post_open_queue_registries -- fixture-trust-prompt-preview-narrowed
```
