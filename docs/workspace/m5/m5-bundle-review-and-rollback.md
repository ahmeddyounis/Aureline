# M5 bundle review and rollback

This document is the contract for the `m5_bundle_review_and_rollback_packet`. The
canonical packet is checked in at
`artifacts/workspace/m5/m5-bundle-review-and-rollback.json`, validated by
`schemas/workspace/m5-bundle-review-and-rollback.schema.json`, and backed by the
typed model in the `aureline-workspace` crate (`m5_bundle_review_and_rollback`).

It builds directly on the workflow-bundle manifests
(`m5_workflow_bundle_manifests`): a manifest declares what a bundle *is*, and a
bundle-review record declares what installing, updating, removing, or drift-checking
it would *do* to the current workspace, reviewably and reversibly.

## What the packet governs

Every claimed M5 stack — notebook, data/API, profiler, framework-pack, docs,
companion, sync-handoff, and opened local folder — gets one diff-and-checkpoint
[`BundleReviewRecord`] per lifecycle action it is reviewed for. The four actions
share **one** model:

- `install` — adopt a bundle into a workspace.
- `update` — move an installed bundle to a newer revision.
- `remove` — uninstall a bundle.
- `drift_review` — compare current local state against the bundle without mutating.

`install`, `update`, and `remove` mutate local state and therefore carry a one-step
rollback checkpoint; `drift_review` is read-only.

## The diff preview is one shape across every action and component category

Every review carries a `component_diffs` list. Each [`ComponentDiffEntry`] is a
diffable reference — never an opaque blob — spanning the same closed component
vocabulary the manifest uses: `extension`, `profile_preset`, `layout_preset`,
`settings_preset`, `task_recipe`, `launch_recipe`, `debug_recipe`, `docs_pack`,
`tour_pack`, `template_ref`, `scaffold_ref`, and `migration_mapping`. Each entry
records:

- **How it differs** — a `diff_action` of `added`, `removed`, `modified`, or
  `unchanged`.
- **Who owns the local state** — an `ownership` of `bundle_owned`,
  `locally_overridden`, `adopted`, `removable`, `blocked_by_policy`, or
  `blocked_by_lifecycle`.
- **What was decided** — a `resolution` of `keep_local`, `adopt_bundle`, `rebase`,
  `compare`, `remove_bundle_owned`, or `not_applicable`.
- **What capability it depends on** — a `lifecycle_stage`, with any non-stable stage
  forcing `requires_review` and disclosure (see below).
- **A diffable preview ref** — `diff_preview_ref`, an opaque reference into the
  rendered preview, plus an optional `local_override_ref` when local state diverges.

The same object feeds Start Center, bundle-detail pages, CLI/headless install,
diagnostics, support export, and docs/help (`consumer_surfaces`) so the desktop UI,
the CLI, and a support export always explain the same drift and rollback state.

## Created-versus-adopted assets are explicit, so removal is safe

The `ownership` class is the guardrail that keeps `remove` and `rebase` honest:

- `bundle_owned` and `removable` assets are created and owned by the bundle and may
  be removed with it.
- `locally_overridden` and `adopted` assets carry durable user state and are
  **user-protected**: a review may never resolve them to `remove_bundle_owned`, and
  a remove operation's drift record must preserve them
  ([`BundleReviewRecord::removal_preserves_user_assets`]).
- `blocked_by_policy` and `blocked_by_lifecycle` assets cannot be pulled in: a
  blocked asset may only be compared or kept local, never adopted or rebased, and a
  blocked component must carry a matching `policy_lifecycle_warnings` entry
  ([`BundleReviewRecord::blocked_components_warned`]).

This is what lets a bundle be updated or removed without silently resetting
unrelated user state or erasing locally authored artifacts under the banner of
cleanup.

## Resolutions stay distinct verbs

`keep_local`, `adopt_bundle`, `rebase`, `compare`, `remove_bundle_owned`, and
`not_applicable` are separate choices that never collapse into one another. The
safety rules ([`ComponentDiffEntry::resolution_safe`]):

- An `unchanged` component takes no action (`not_applicable`).
- A user-protected asset paired with `remove_bundle_owned` is rejected.
- A blocked asset paired with `adopt_bundle` or `rebase` is rejected.
- `remove_bundle_owned` only applies to `bundle_owned` or `removable` assets.
- A non-blocked component that differs must record a real decision.

## Non-stable dependencies are declared, never hidden

A review may depend on a Preview, Labs, policy-gated, mirror-only, or
bounded-platform capability — but it must say so. Each non-stable component is
`requires_review`, and the distinct non-stable stages roll up into the review's
`dependency_markers`; a review that depends on a non-stable capability must set
`discloses_non_stable_dependencies` ([`BundleReviewRecord::disclosure_consistent`]).

## One-step rollback before any mutation commits

Every mutating operation carries a [`RollbackCheckpoint`] minted **before** the
mutation commits: a non-empty `checkpoint_ref`, `one_step` and `reversible` true, and
`captured_before_mutation` true ([`RollbackCheckpoint::supports_one_step_rollback`]).
Drift review carries at least a non-empty baseline `checkpoint_ref`. This is what
lets an install, update, or remove be undone in a single step.

## Certification never out-ranks the target

A review presents as certified only when its `certification_target` is `certified`
([`BundleReviewRecord::presents_as_certified`]). Anything weaker, anything depending
on a non-stable capability, anything with a review-gated component, any remove, and
any blocking warning must carry a `caveat`
([`BundleReviewRecord::caveats_required`]).

## Validation

`M5BundleReviewAndRollbackPacket::validate()` returns every violation. It enforces:

- The schema version, record kind, and every closed vocabulary array.
- Unique review ids; every M5 wedge and every operation covered.
- Per review: guardrails held (diffable/mirrorable/export-safe, never opaque),
  disclosure consistent, a one-step rollback checkpoint on mutating ops, removal that
  preserves user assets, blocked components warned, provenance present, every diff
  entry consistent, a caveat when required, and no certification overreach.
- The `summary` equals the recomputed summary.

## What this packet is not

It carries no credential bodies, raw provider payloads, raw local paths, or bundle
binary contents. Every field is a typed state, a count, or an opaque ref. Opaque
binary bundle state is forbidden (`opaque_binary_state` is always false).
