# Fixtures: M5 bundle review and rollback

This directory contains fixture metadata for the
`m5_bundle_review_and_rollback_packet`.

The canonical full corpus is checked in at:

`artifacts/workspace/m5/m5-bundle-review-and-rollback.json`

and is validated by `schemas/workspace/m5-bundle-review-and-rollback.schema.json` and
the typed model in the `aureline-workspace` crate (`m5_bundle_review_and_rollback`).

## Coverage

- **Every wedge, one review.** Each of the eight M5 stacks — `notebook_workspace`,
  `data_and_api_workspace`, `profiler_workspace`, `framework_pack_workspace`,
  `docs_workspace`, `companion_workspace`, `sync_handoff_workspace`, and
  `local_folder_workspace` — has one bundle review (`covers_every_wedge`).
- **Every operation, exercised.** `install`, `update`, `remove`, and `drift_review`
  each appear twice (`covers_every_operation`). The six mutating reviews carry a
  one-step rollback checkpoint minted before the mutation commits; the two
  drift-review reviews are read-only.
- **Every diff action and resolution.** The component diffs exercise `added`,
  `removed`, `modified`, and `unchanged`, resolved by `keep_local`, `adopt_bundle`,
  `rebase`, `compare`, `remove_bundle_owned`, and `not_applicable`.
- **Every ownership class.** `bundle_owned`, `locally_overridden`, `adopted`,
  `removable`, `blocked_by_policy`, and `blocked_by_lifecycle` are all classified.
  The remove reviews keep an `adopted` profile and a `locally_overridden` layout while
  deleting only bundle-owned assets, proving removal never erases user state.
- **Every lifecycle stage.** `stable` everywhere plus one disclosed non-stable
  dependency each of `policy_gated` (data/API), `bounded_platform` (profiler),
  `preview` (framework-pack), `mirror_only` (companion), and `labs` (sync-handoff).
  Each non-stable component is review-gated and rolled into the review's dependency
  markers; the policy-gated and Labs components are additionally blocked and carry a
  blocking warning.
- **Every certification target.** `certified` (notebook, framework-pack),
  `managed_approved` (data/API), `community_reviewed` (profiler, docs, local folder),
  `imported_pending_review` (companion), and `local_draft` (sync-handoff). Only the
  certified reviews present as certified; every weaker review carries a caveat.
- **Every drift state.** `in_sync` (profiler), `local_ahead` (companion, local
  folder), `bundle_ahead` (notebook, framework-pack, docs), `diverged` (data/API), and
  `unknown` (sync-handoff).

## Negative cases

Negative cases are asserted in the crate's unit tests rather than checked-in malformed
fixtures: a user-protected asset resolved to `remove_bundle_owned`, a blocked asset
resolved to `adopt_bundle`, an unchanged component taking an action, a summary that
drifts from the recomputed summary, an undisclosed non-stable dependency, and a
mutating review missing its one-step rollback checkpoint are all rejected by
`ComponentDiffEntry::resolution_safe` and `M5BundleReviewAndRollbackPacket::validate`.
