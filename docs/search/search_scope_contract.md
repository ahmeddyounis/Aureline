# Search and quick-open scope contract

The workset-aware search scope is the canonical resolver every search and
quick-open surface reads when it has to answer "what does the active
workset/slice include and what does the user see on the chip?". This
document is the reviewer-facing entry point for the runtime path that
backs the surface.

It composes with — and does **not** replace — the surrounding contracts:

- [`docs/ux/scope_truth_chip.md`](../ux/scope_truth_chip.md) for the
  chrome chip vocabulary and chip actions;
- [`docs/workspace/workset_artifact_contract.md`](../workspace/workset_artifact_contract.md)
  for the persisted workset artifact shape;
- [`search_readiness_vocabulary.md`](./search_readiness_vocabulary.md) for
  the readiness, freshness, partial-truth, and source-class tokens that
  flow through the lexical lane;
- [`quick_open_contract.md`](./quick_open_contract.md) for the renderer-
  facing quick-open row anatomy.

If those contracts already define a vocabulary axis, this document and
the runtime path quote that axis instead of minting a synonym.

## Canonical types

The scope resolver lives at
`crates/aureline-search/src/scope/`. The runtime types are:

| Type | Owns |
|---|---|
| [`WorkspaceSearchScope`] | Workset/slice identity (`workset_id`, `workset_name`), scope class, root refs, include/exclude patterns, chip label, presentation state, partial-index note, and the partial-scope flag. |
| [`ScopePatternRecord`] | One include / exclude pattern entry projected from the canonical workspace `PatternEntry`. |
| [`ScopeFilterOutcome`] | Partition of one workspace file list into in-scope / out-of-scope buckets, with `all_workspace_count` and `in_scope_count` for chip count disclosure. |
| [`WorkspaceSearchScopeMetadata`] | Serializable metadata snapshot attached to search-shell and quick-open exports. |

The resolver consumes a `WorksetArtifactRecord` directly through
`WorkspaceSearchScope::from_workset_artifact`, and offers
`for_full_workspace`, `for_current_repo`, and `for_workset_stub`
constructors for the cases where no full artifact is in play. Surfaces
MUST go through these constructors — they MUST NOT mint a parallel chip
label, partition, or pattern projection.

## Honesty contract

1. **The chip carries the same scope vocabulary the workset switcher
   uses.** Surfaces project the chip label through
   [`WorkspaceSearchScope::chip_label`]; the result mirrors the workspace
   crate's `chip_label_family` plus the workset name (`Selected workset ·
   Hot path`, `Sparse slice · Frontend slice`, ...).

2. **Switching worksets re-runs the filter.** A workset switch on the
   search shell or quick-open session re-projects the scope and re-runs
   the filter over the lexical file list. The chrome surface guarantees
   that a row from the previous workset's pattern set never bleeds into
   the new chip — the failure drill at
   `fixtures/search/scope_cases/workset_switch_failure_drill.json`
   captures this invariant.

3. **The `partial_scope` flag is true whenever the active scope is
   narrower than the workspace OR the active scope's chip presentation
   state reads as partial.** The chrome SHOULD render a `Partial` cue
   alongside this chip when the flag is true. It MUST NOT collapse the
   flag into a generic "loading" badge.

4. **Excludes always win over includes.** Within one workset's pattern
   set, a path that matches any exclude pattern is out-of-scope even if
   it matches an include. When no include patterns are present, every
   path that survives the exclude check is in scope.

5. **Snapshots carry the scope metadata.** The search shell's
   `WorkspaceSearchSurfaceCard` and the quick-open `QuickOpenSnapshot`
   embed the canonical [`WorkspaceSearchScopeMetadata`] so an exported or
   replayed session preserves the chip label, pattern fingerprint, and
   partial-scope flag that produced the visible row set.

## Pattern vocabulary

| Token | Meaning |
|---|---|
| `include` | Row must match at least one include pattern (after passing every exclude check) to be in scope. |
| `exclude` | Row matching this pattern is out of scope, even when an include matches the same row. |

Glob syntax is intentionally narrow:

- `**` matches any path (including the empty path) across `/` boundaries;
- `*` matches any run of non-`/` characters within one segment;
- everything else is a literal segment.

The vocabulary is enough for the M1 workset/slice surfaces. A richer
matcher (anchors, character classes, brace expansion) belongs in a
future scope-pattern crate, not here.

## Presentation states

| Token | When to surface it |
|---|---|
| `active_narrow_safe` | Scope is narrowed but readiness is ready and member truth is loaded. The chip stays "safe to trust" but still shows the narrowed scope label. |
| `active_partial` | Scope reads as partial: workset members are warming / cached / manifest-known, the partial-index note is set, or readiness is below ready. |
| `active_policy_limited` | Scope is a policy-limited view (admin-narrowed). |
| `active_widened` | Scope was just widened; chrome may surface a transient "widened" cue. |

Surfaces MUST quote these tokens directly — they MUST NOT collapse
`active_partial` and `active_policy_limited` into a single "warning"
badge.

## Failure drill

> Switch worksets during search/open and confirm the scope-truth chip
> updates before any result row can be mistaken for repo-wide truth.

The drill runs through:

1. Open a multi-root or sparse-scope workspace. The chip on the search
   shell and on quick open reads the active workset's family + name.
2. Type a query that returns rows under workset A's pattern set.
3. Switch to workset B (different include pattern).
4. The chip updates to workset B's family + name in the same frame the
   filter is reapplied. Workset A's rows are dropped from the visible
   set; the snapshot's `scope_metadata` field carries workset B's
   `chip_label`, pattern fingerprint, and `partial_scope` flag.

Fixture coverage for the drill lives at
`fixtures/search/scope_cases/workset_switch_failure_drill.json`. The
single-scope cases under the same directory pin the chip vocabulary and
partition behavior for the supporting scope shapes.

## Edit guidance

- Keep new pattern semantics and chip labels under
  `crates/aureline-search/src/scope/`. Do NOT mint parallel chip families
  in the search shell or quick-open layers.
- When adding a fixture, mirror the existing schema (`record_kind`,
  `schema_version`, `scope_kind`, `expect`). Update
  `fixtures/search/scope_cases/README.md` so reviewers can find the new
  case quickly.
- When adding a new presentation state or pattern vocabulary token,
  update this document, the `WorkspaceSearchScope` projection, and the
  fixtures together so reviewers can spot drift in a single PR.

[`WorkspaceSearchScope`]: ../../crates/aureline-search/src/scope/projection.rs
[`ScopePatternRecord`]: ../../crates/aureline-search/src/scope/filter.rs
[`ScopeFilterOutcome`]: ../../crates/aureline-search/src/scope/filter.rs
[`WorkspaceSearchScopeMetadata`]: ../../crates/aureline-search/src/scope/projection.rs
[`WorkspaceSearchScope::chip_label`]: ../../crates/aureline-search/src/scope/projection.rs
