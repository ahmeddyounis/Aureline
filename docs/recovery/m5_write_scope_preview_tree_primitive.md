# M5 write-scope-preview-tree primitive

Status: implemented (B105, task M05-895)

This is the third `implement_` lane that narrows the frozen
[M5 local-history / write-scope component matrix](./m5_local_history_write_scope_component_matrix.md)
into two reusable primitives: the **write-scope preview tree** and its **file
nodes**. It sits alongside the
[local-history-row and checkpoint-group-card primitive](./m5_local_history_row_and_checkpoint_group_card_primitive.md)
and the
[restore-preview-card and restore-granularity-selector primitive](./m5_restore_preview_card_and_restore_granularity_selector_primitive.md),
and makes every multi-file rename, refactor, replace, import, AI apply, or repair
flow preview-first: the user sees how wide the change reaches, which files are in
scope and which are held out and *why*, and who authored each change — before any
apply commits.

Truth source (checked in):

- Schema: `schemas/ui/m5-write-scope-preview-tree.schema.json`
- Support export: `artifacts/release/m5-write-scope-preview-tree-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-write-scope-preview-tree-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-write-scope-preview-tree-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-write-scope-preview-tree-primitive/`

The single mint-from-truth path is the headless emitter
`aureline_history_write_scope_preview_tree_primitive`; the in-code seed builders,
the checked support export, and the fixtures never drift.

## What the primitives implement

The matrix names the write-scope preview tree family and freezes its controlled
vocabulary (write-scope classes, managed-file caveats, mutation classes, surface
families, deployment lines, consumer surfaces, accessibility routes, qualification
classes, and downgrade triggers). This module implements two resolvers that narrow
that contract:

### `resolve_write_scope_preview_tree`

Takes one change's write-scope class, mutation class, total / included / excluded
file counts, distinct workspace-root count, generated-or-managed / out-of-workspace
/ conflict / policy-blocked signals, reviewability, apply-path readiness, and its
opaque scope label. Produces the derived **tree posture** in a fixed blocking-first
ladder:

1. `blocked_scope` — the apply path is unavailable.
2. `conflict_scope` — a pending conflict must resolve first.
3. `out_of_workspace_scope` — the change writes outside the workspace root.
4. `generated_managed_scope` — the change reaches a generated tree or managed files.
5. `broad_scope` — a multi-file, multi-root, or cross-package change.
6. `focused_scope` — a focused, in-workspace change over a small file set.

The **file-count bucket** (`empty`/`single`/`small`/`medium`/`large`/`sweeping`)
is derived from the honest *total* — including the files that will not be written —
so the blast radius is never understated by counting only the applied files. The
tree offers `inspect_tree` and `expand_all` always, `jump_to_diff` where reviewable,
`narrow_scope` where narrowable, `exclude_generated` for a generated/managed scope
that can apply, `apply_scope` where appliable, and `resolve_conflict` for a conflict
scope.

### `resolve_write_scope_file_node`

Takes one file node's change type, change actor, content class, managed-file caveat,
read-only / conflict / policy-blocked / out-of-workspace signals, whether the caller
opted the file out of the apply, diff availability, and its opaque node label.
Produces the derived **node disposition** in a fixed blocking-first ladder:

1. `policy_blocked_excluded` — policy blocks writing the file.
2. `conflict_held` — the file is held behind a pending conflict.
3. `read_only_excluded` — the file is read-only / protected.
4. `generated_excludable` — a generated / managed file, included but excludable.
5. `binary_included` — a binary file kept in scope with a binary-diff cue.
6. `included_in_scope` — an ordinary text change in scope.

The exact **exclusion reason** (`policy_blocked`, `read_only_protected`,
`conflict_pending`, `generated_opted_out`, `out_of_workspace`) is surfaced whenever
the file is out of the apply. The file always stays present in the preview and always
exposes its actor provenance — a policy-blocked, binary, metadata-only, read-only,
or generated file is never silently dropped.

## Hard invariants

Every row asserts four invariants, all of which must be `false`:

- `flattens_into_generic_file_list` — the tree never collapses distinct file states.
- `drops_ineligible_files` — ineligible files stay visible in the preview.
- `understates_write_scope` — the file-count bucket reflects the honest total.
- `hides_actor_provenance` — every file node names who authored its change.

## Consumers

One tree and node grammar is shared across all six claimed multi-file change
surfaces — `rename_preview`, `refactor_preview`, `search_replace_preview`,
`import_preview`, `ai_apply_preview`, and `repair_preview` — so the scope /
provenance / exclusion vocabulary stays identical across every claimed mutation
surface without any surface inventing a second write-scope grammar.
