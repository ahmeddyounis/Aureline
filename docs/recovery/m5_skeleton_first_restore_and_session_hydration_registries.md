# M5 skeleton-first-restore and session-hydration registries

This lane is the skeleton-first / hydrate-second implement lane over the frozen
[M5 window-restore matrix](./m5_window_restore_contract.md). It turns the *skeleton-first restore* grammar and
the *no-rerun session hydration* grammar into registry resolvers that produce export-safe, honest projections,
so the shell, recovery, diagnostics, admin, workspace, session, docs, CLI, and support surfaces resolve one
canonical restore-orchestration truth instead of a per-pane, hand-copied reconstruction. Restore is made
progressively truthful instead of all-or-nothing: the layout skeleton — window shell, stable pane-tree
structure, preserved pane roles, and placeholder set — is rebuilt first, and only then are heavy dependencies
(remote sessions, terminals, notebooks, debuggers, extension and collaboration views) lazily hydrated, so a
missing, expired, quarantined, or unsupported dependency produces a pane-role-preserving placeholder rather
than a silent layout collapse, and the deferred-hydration plan is kept distinct from the layout skeleton.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_skeleton_first_restore_and_session_hydration_registries` (the authoritative
  validator).
- **Combined schema:**
  `schemas/shell/m5-skeleton-first-restore-and-session-hydration-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/shell/m5-restore-fidelity.schema.json`](../../schemas/shell/m5-restore-fidelity.schema.json) and
  [`schemas/shell/m5-window-topology.schema.json`](../../schemas/shell/m5-window-topology.schema.json) as its
  canonical domain contracts.
- **Checked proof:** `artifacts/release/m5-skeleton-first-restore-and-session-hydration-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/ui/m5-skeleton-first-restore-and-session-hydration-registries/`
  (`placeholder_pane_continuity_beta_narrowed.json`,
  `context_only_hydration_preview_narrowed.json`).

## Two registries

1. **Skeleton-first restore** (`resolve_skeleton_restore_entry`) — rebuilds one stable restore-skeleton object
   per restore: the restore-fidelity class and canonical restore-fidelity mode, the window shell, the stable
   versioned pane-tree structure, the preserved pane roles, the placeholder set, the layout-skeleton root, and
   the distinct deferred-hydration plan. A clean entry names a canonical registry token, a classified
   restore-fidelity class, and a window-restore role, covers the canonical / accessible / audit resolution
   forms, publishes a complete object, rebuilds the skeleton before heavy hydration, and preserves pane roles
   when it defers heavy hydration. Otherwise it degrades honestly — heavy hydration that ran before the layout
   skeleton was rebuilt degrades to `hydration_preceded_skeleton`.
2. **No-rerun session hydration** (`resolve_session_hydration_entry`) — keeps lazy dependency hydration from
   rerunning session-scoped work or collapsing layout. A clean entry names a classified session-hydration
   surface and provides the preserved-pane-role / missing-dependency-class / restore-fidelity-hint disclosure
   triple; a hydration that reruns session-scoped work or reacquires broader authority, deletes layout
   structure silently on a missing dependency instead of substituting a pane-role-preserving placeholder, or
   overclaims restore fidelity on a deferred dependency degrades to
   `session_hydration_reruns_or_collapses_layout`.

## Per-restore fidelity reference

The restore-fidelity class carries its canonical restore-fidelity mode, and the resolver publishes the full
skeleton object, so the registry — never a hand-copied per-pane restore assumption — is the single source of
truth. `restore_skeleton_object_is_complete` rejects an object missing any field, `skeleton_precedes_hydration`
rejects a hydration-first restore, and `session_hydration_stays_no_rerun` rejects a hydration that reran
session-scoped work or collapsed the layout.

| restore-fidelity class | restore-fidelity mode | window shell | stable pane-tree structure | preserved pane roles | placeholder set | layout-skeleton root |
| --- | --- | --- | --- | --- | --- | --- |
| live | live_hydrated | `window-shell.main` | `pane-tree.main.v3` | `pane-roles.editor\|terminal\|preview` | `placeholders.none.0007` | `layout-skeleton.acme/warm` |
| placeholder | pane_role_placeholder | `window-shell.main` | `pane-tree.main.v4` | `pane-roles.editor\|terminal\|debugger` | `placeholders.pane.0011` | `layout-skeleton.acme/cold-start` |
| context-only | context_only | `window-shell.detached-inspector` | `pane-tree.detached.v1` | `pane-roles.notebook\|preview` | `placeholders.pane.0011` | `layout-skeleton.acme/crash-loop` |
| evidence-only | evidence_only | `window-shell.secondary` | `pane-tree.secondary.v2` | `pane-roles.remote-shell\|collab` | `placeholders.pane.0019` | `layout-skeleton.acme/remote-reconnect` |

A hydration-first restore degrades to `hydration_preceded_skeleton`, an incomplete object degrades to
`restore_skeleton_object_incomplete`, and a collapsed layout degrades to
`session_hydration_reruns_or_collapses_layout`, so a hydration-first restore, an incomplete object, or a
silent layout collapse can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Restored layouts appear as truthful skeletons before heavy services finish hydrating.** The layout
  skeleton is rebuilt before heavy hydration: a hydration-first example degrades, an unbound example degrades,
  a clean skeleton-first entry is present, and no clean entry ran hydration first.
- **Missing dependencies produce pane-role-preserving placeholders instead of silent layout collapse.** Clean
  session-hydration entries cover the terminal / debugger / preview surfaces with full resolution-form coverage
  while providing the disclosure triple, and a hydration that collapses the layout on a missing dependency
  degrades.
- **Support/export can explain which panes restored as live, placeholder, context-only, or evidence-only.**
  Clean skeleton entries cover the canonical live / placeholder / context-only / evidence-only restore-fidelity
  classes and the first shell / recovery / diagnostics / admin / support surfaces, an object-incomplete example
  degrades, and no clean skeleton entry published an incomplete object.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_skeleton_first_restore_and_session_hydration_registries -- support-export
cargo run -p aureline-ui --example dump_m5_skeleton_first_restore_and_session_hydration_registries -- csv
cargo run -p aureline-ui --example dump_m5_skeleton_first_restore_and_session_hydration_registries -- report
cargo run -p aureline-ui --example dump_m5_skeleton_first_restore_and_session_hydration_registries -- restore-fidelity-table
cargo run -p aureline-ui --example dump_m5_skeleton_first_restore_and_session_hydration_registries -- fixture-placeholder-pane-continuity-beta-narrowed
cargo run -p aureline-ui --example dump_m5_skeleton_first_restore_and_session_hydration_registries -- fixture-context-only-hydration-preview-narrowed
```
