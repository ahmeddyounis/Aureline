# Workset Portability Alpha

Portable worksets and sparse slices are durable scope artifacts, not private UI
filters. The alpha contract keeps one vocabulary across search, graph, AI,
refactor, support, export, and headless consumers.

## Canonical Records

| Record | Source | Purpose |
|---|---|---|
| Workset artifact | `schemas/workspace/workset_artifact.schema.json` | Stable workset/slice identity, include/exclude patterns, root taxonomy, sparse/full mode, source class, readiness, portability, scope chips, consumer bindings, and scope-widen diffs. |
| Workset switcher | `schemas/workspace/workset_switcher.schema.json` | Read-only switcher rows and active-scope banners projected from saved artifacts. |
| Scope-diff review | `schemas/workspace/scope_diff_review.schema.json` | Explicit review sheet before widening or narrowing the active scope. |
| Cross-repo result group | `schemas/workspace/cross_repo_result_group.schema.json` | Root-preserving result grouping with outside-current-scope markers. |
| Cross-repo jump event | `schemas/workspace/cross_repo_jump_event.schema.json` | Jump, peek, and back-navigation continuity without dropping workset or root identity. |
| Bundle index | `schemas/workspace/workset_alpha.schema.json` | Single alpha-family citation that references the canonical records above. |

## Runtime Path

`crates/aureline-workspace/src/worksets` owns the durable workset/slice artifact,
scope chip, non-UI consumer binding, and scope-widen diff model.

`crates/aureline-shell/src/workset_switcher` is the first shell consumer. It
projects:

- `WorksetSwitcherRecord` rows with workset name, repo/folder counts,
  source class, readiness, active state, hidden-result summary, and open/manage
  actions.
- `ScopeBannerRecord` with the active boundary, partial/warming/policy/widened
  state, hidden-result disclosure, and widen/reset actions.
- `ScopeDiffReviewRecord` with added or removed roots/modules, expected index
  and runtime cost, remote/cache source notes, support/export impact, trust and
  policy posture, and confirm/cancel/remember-choice actions.

## Reopen Truth

Saved worksets project `WorksetScopeConsumerBinding` records for local UI,
remote UI, headless, support export, navigation, and refactor consumers. Exact
reopen keeps `reopen_state = exact`; degraded reopen keeps the same stable
scope identity and emits an explicit reason such as `missing_root`,
`rebinding_required`, `remote_unavailable`, `managed_provider_unavailable`, or
`policy_limited`.

## Proof Fixtures

The protected fixture index is
`fixtures/workspace/workset_switcher_alpha/protected_workset_portability.yaml`.
It points at the saved artifact, switcher, warming/policy banners,
scope-diff reviews, cross-repo result group, and cross-repo peek event used by
the Rust tests.

Run:

```sh
cargo test -p aureline-shell --test workset_switcher_alpha
```
