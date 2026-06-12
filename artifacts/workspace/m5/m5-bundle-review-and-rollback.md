# M5 bundle review and rollback — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/workspace/m5/m5-bundle-review-and-rollback.json`. The full contract lives
in `docs/workspace/m5/m5-bundle-review-and-rollback.md`; the typed model lives in the
`aureline-workspace` crate (`m5_bundle_review_and_rollback`).

This artifact freezes one **diff-and-checkpoint review** per claimed M5 stack for the
install, update, remove, and drift-review lifecycle of a workflow bundle. Each review
shows a per-component diff, classifies created-versus-adopted assets, discloses
lifecycle-sensitive dependencies, and mints a one-step rollback checkpoint before any
mutation commits.

## Review roll-up (as of 2026-06-11)

| Review | Wedge | Bundle | Operation | Target | Drift | Markers | 1-step rollback |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `review-notebook-install` | notebook | `m5-wbm:notebook` | install | certified | bundle_ahead | — | yes |
| `review-data-and-api-update` | data/API | `m5-wbm:data-and-api` | update | managed_approved | diverged | policy_gated | yes |
| `review-profiler-drift` | profiler | `m5-wbm:profiler` | drift_review | community_reviewed | in_sync | bounded_platform | n/a |
| `review-framework-pack-update` | framework-pack | `m5-wbm:framework-pack` | update | certified | bundle_ahead | preview | yes |
| `review-docs-install` | docs | `m5-wbm:docs` | install | community_reviewed | bundle_ahead | — | yes |
| `review-companion-remove` | companion | `m5-wbm:companion` | remove | imported_pending_review | local_ahead | mirror_only | yes |
| `review-sync-handoff-drift` | sync-handoff | `m5-wbm:sync-handoff` | drift_review | local_draft | unknown | labs | n/a |
| `review-local-folder-remove` | local folder | `m5-wbm:local-folder` | remove | community_reviewed | local_ahead | — | yes |

## What the reviews prove

- **One diff-and-checkpoint model.** Install, update, remove, and drift review share
  one operation vocabulary and one component-diff shape spanning every content
  category — extensions, presets, recipes, docs/tour packs, template/scaffold refs,
  and migration mappings.
- **Created-versus-adopted is explicit.** Every diff is classified `bundle_owned`,
  `locally_overridden`, `adopted`, `removable`, `blocked_by_policy`, or
  `blocked_by_lifecycle`. The two remove reviews keep an `adopted` profile and a
  `locally_overridden` layout while deleting only bundle-owned assets.
- **Resolutions stay distinct and safe.** A user-protected asset is never resolved to
  `remove_bundle_owned`; a blocked asset is never adopted or rebased, only compared
  or kept local.
- **Non-stable dependencies are disclosed.** Five reviews each disclose one
  lifecycle-sensitive marker — `policy_gated`, `bounded_platform`, `preview`,
  `mirror_only`, and `labs` — and review-gate the depending component.
- **One-step rollback before mutation.** Every mutating review (both installs, both
  updates, both removes) carries a checkpoint minted before the mutation commits.
- **No certification overreach.** Only the two `certified` reviews present as
  certified; every weaker, non-stable, review-gated, remove, or blocking-warning
  review carries a caveat.

## Consuming surfaces

Start Center, bundle-detail pages, CLI/headless install, diagnostics, support export,
and docs/help all ingest this one packet rather than re-deriving drift or rollback
state per surface, so the desktop UI, the CLI, and a support export always explain the
same thing.
