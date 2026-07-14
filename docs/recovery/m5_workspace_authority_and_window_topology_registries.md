# M5 workspace-authority and window-topology registries

This lane is the first implement lane over the frozen
[M5 window-restore matrix](./m5_window_restore_contract.md). It turns the *shared workspace authority* grammar
and the *window-local topology* grammar into registry resolvers that produce export-safe, honest projections,
so the shell, recovery, diagnostics, admin, workspace, session, docs, CLI, and support surfaces resolve one
canonical workspace-ownership truth instead of a per-window, hand-copied reconstruction. Workspace authority
and window topology are separated in runtime and serialized state: stable pane-tree IDs and shared
dirty-buffer / save / checkpoint state live on the shared authority, while selection and focus history stay
window-local, and profile defaults / machine-display hints are kept distinct from authoritative workspace
state so they cannot silently override restored topology.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_workspace_authority_and_window_topology_registries` (the authoritative validator).
- **Combined schema:**
  `schemas/shell/m5-workspace-authority-and-window-topology-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/shell/m5-window-topology.schema.json`](../../schemas/shell/m5-window-topology.schema.json) and
  [`schemas/shell/m5-restore-fidelity.schema.json`](../../schemas/shell/m5-restore-fidelity.schema.json) as its
  canonical domain contracts.
- **Checked proof:** `artifacts/release/m5-workspace-authority-and-window-topology-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/ui/m5-workspace-authority-and-window-topology-registries/`
  (`multi_window_shared_authority_beta_narrowed.json`,
  `auxiliary_window_topology_preview_narrowed.json`).

## Two registries

1. **Workspace authority** (`resolve_workspace_authority_entry`) — publishes one stable workspace-authority
   object per workspace: the authority scope and canonical authority mode, the windows it backs, the stable
   versioned pane-tree IDs, the shared dirty-buffer / save / checkpoint state, the authoritative workspace state
   root, and the distinct profile-defaults reference. A clean entry names a canonical registry token, a
   classified authority scope, and a window-restore role, covers the canonical / accessible / audit resolution
   forms, publishes a complete object, keeps window-local selection and focus window-local, and preserves
   window-local history when one authority backs multiple windows. Otherwise it degrades honestly — a
   window-local selection or focus that overwrites the shared authority degrades to
   `window_local_state_overwrites_shared_authority`.
2. **Window topology** (`resolve_window_topology_entry`) — keeps window-local topology distinct from the shared
   authority. A clean entry names a classified window-topology surface and provides the window-local pane-tree /
   focus-history / display-affinity disclosure triple; a topology that privately copies shared authority state
   without disclosure, merges authority and topology into one opaque blob, or lets profile defaults override
   authoritative topology degrades to `window_topology_merges_or_leaks_shared_authority`.

## Per-workspace ownership reference

The authority scope carries its canonical authority mode, and the resolver publishes the full authority object,
so the registry — never a hand-copied per-window assumption — is the single source of truth.
`workspace_authority_object_is_complete` rejects an object missing any field,
`window_local_state_stays_window_local` rejects a window-local overwrite, and `window_topology_stays_distinct`
rejects a topology that has absorbed the shared authority.

| authority scope | authority mode | backing windows | stable pane-tree IDs | shared dirty-buffer state | shared save/checkpoint state | authoritative state root |
| --- | --- | --- | --- | --- | --- | --- |
| single-window | single_window_authority | `window.main` | `pane-tree.main.v3` | `dirty-buffer.shared.0007` | `checkpoint.shared.0007` | `workspace-authority.acme/single` |
| multi-window shared | multi_window_shared_authority | `window.main`, `window.secondary` | `pane-tree.main.v4`, `pane-tree.secondary.v2` | `dirty-buffer.shared.0011` | `checkpoint.shared.0011` | `workspace-authority.acme/shared` |
| detached / auxiliary | detached_auxiliary_window_authority | `window.main`, `window.detached-inspector` | `pane-tree.main.v4`, `pane-tree.detached.v1` | `dirty-buffer.shared.0011` | `checkpoint.shared.0011` | `workspace-authority.acme/shared` |

A window-local overwrite degrades to `window_local_state_overwrites_shared_authority`, an incomplete object
degrades to `workspace_authority_object_incomplete`, and a privately-copied authority degrades to
`window_topology_merges_or_leaks_shared_authority`, so a window-local overwrite, an incomplete object, or a
leaked authority can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Every claimed workspace resolves to one stable workspace-authority object with backing-windows / pane-IDs /
  shared-state / distinct-defaults fields.** Clean authority entries cover the canonical single-window /
  multi-window / detached authority scopes and the first shell / recovery / diagnostics / admin / support
  surfaces, an object-incomplete example degrades, and no clean authority entry published an incomplete object.
- **Multiple windows share one workspace authority while preserving independent layout and focus without
  dirty-state drift.** Window-local state stays window-local: a window-local-overwrite example and an unbound
  example degrade, a clean isolated authority entry is present, and no clean entry lost window-local isolation.
- **The suite fails when window-local state overwrites shared authority or shared authority becomes private
  window state.** Clean window-topology entries cover the primary / auxiliary / diagnostics surfaces with full
  resolution-form coverage while providing the disclosure triple, and a topology that privately copies shared
  authority state degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_workspace_authority_and_window_topology_registries -- support-export
cargo run -p aureline-ui --example dump_m5_workspace_authority_and_window_topology_registries -- csv
cargo run -p aureline-ui --example dump_m5_workspace_authority_and_window_topology_registries -- report
cargo run -p aureline-ui --example dump_m5_workspace_authority_and_window_topology_registries -- workspace-ownership-table
cargo run -p aureline-ui --example dump_m5_workspace_authority_and_window_topology_registries -- fixture-multi-window-shared-authority-beta-narrowed
cargo run -p aureline-ui --example dump_m5_workspace_authority_and_window_topology_registries -- fixture-auxiliary-window-topology-preview-narrowed
```
