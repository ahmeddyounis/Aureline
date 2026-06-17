# M5 remembered-state objects

The **remembered-state objects** are the explicit, versioned state objects that restorable M5
surfaces resolve against, so restore fidelity stops being an emergent side effect of ad hoc
serialized blobs. The serialization-and-restore matrix classifies *what* M5 is allowed to remember;
this packet **implements the underlying objects themselves**: versioned workspace-authority
checkpoints, window-topology snapshots with stable pane-tree ids, portable profile defaults, and
machine-local hints.

- Typed model: `aureline-workspace` crate, module `m5_remembered_state_objects`.
- Packet: [`artifacts/workspace/m5/m5-remembered-state-objects.json`](../../../artifacts/workspace/m5/m5-remembered-state-objects.json).
- Reviewer artifact: [`artifacts/workspace/m5/m5-remembered-state-objects.md`](../../../artifacts/workspace/m5/m5-remembered-state-objects.md).
- Schema: [`schemas/workspace/m5-remembered-state.schema.json`](../../../schemas/workspace/m5-remembered-state.schema.json).
- Fixtures: [`fixtures/workspace/m5/m5-remembered-state-objects/`](../../../fixtures/workspace/m5/m5-remembered-state-objects/).
- Classification contract: [`docs/workspace/m5/m5-serialization-and-restore.md`](./m5-serialization-and-restore.md).

## Why separate objects

M5 keeps adding multi-pane, multi-window, restore-heavy surfaces — preview routes, notebook
sessions, query consoles, profiler captures, docs panes, incident workspaces, companion handoff
packets, and portable-state artifacts. Left to an opaque per-surface blob, every surface invents its
own serialization and its own boundary between "authority", "layout", and "this machine". This
packet replaces that with four explicit, separately versioned objects:

1. **Workspace-authority checkpoint** ([`WorkspaceAuthorityCheckpoint`]) — a re-resolvable record of
   the authority that was granted. It preserves **dirty-buffer identity** (never content), **journal
   linkage**, **trusted roots**, **active worksets**, and a **restore class**. It never serializes a
   live authority ticket: authority is stored only as a `re_resolvable_reference` that is
   re-evaluated at restore. The `live_ticket` handle class exists in the vocabulary so the gate can
   reject it.
2. **Window-topology snapshot** ([`WindowTopologySnapshot`]) — one window's pane tree, chrome, and
   the boundary scope refs that point at workspace authority, profile defaults, and machine-local
   hints **by reference**. Window-local topology never embeds authority state.
3. **Profile defaults** ([`ProfileDefaults`]) — portable defaults that seed new windows. They carry
   no machine-local anchors.
4. **Machine-local hints** ([`MachineLocalHints`]) — display geometry, monitor affinity, and an
   install anchor. This is the one object that holds machine-unique state, so the other three stay
   portable, and it is **never exportable**.

A [`RememberedStateBundle`] ties the four together purely by reference. That reference boundary is
the structural guard against flattening: a bundle whose authority ref resolves to a window snapshot
instead of a checkpoint is a flattening error.

## Stable pane ids and a versioned pane tree

The pane tree gives every pane a stable `pane_id` and carries its own schema version
(`pane_tree`, currently `1`). Stable ids make the topology serializable, diffable, and migratable:

- **Split, move/float, pin, close, placeholder substitution** are all expressed as operations that
  preserve pane ids — `split_pane`, `detach_pane`, `set_tab_pinned`, `close_pane`, and
  `substitute_placeholder`. Closing a pane collapses an emptied split or tab group so a slot never
  lingers empty, and closing the last pane is a no-op.
- **Diff** — `PaneTree::diff` reports added, removed, and retained pane ids between two trees, so
  topology change is reviewable rather than opaque.
- **Migration** — `migrate_pane_tree` stamps an older tree to the current version and reports a
  `compatible_restore`; a tree from an unreadable newer version is left untouched and reported as
  `manual_review` so the slots are preserved rather than guessed at.

## Placeholders preserve slots

When a dependency is missing, the pane keeps its `pane_id` and original role behind a
[`PlaceholderCard`]. The card names the original role and surface class, the reason, the safe
recovery actions, and a `substitution_behavior` that must preserve the slot. `silent_delete` is
reject-only: a placeholder that would drop layout is a gate failure, never a stored state.

## Fail-closed validation

`M5RememberedStateObjects::validate` returns a typed violation for each of:

- a checkpoint that serializes live authority or drops the `excludes_live_authority` attestation;
- a portable object carrying machine-local ownership or anchors;
- a machine-local object marked exportable;
- a snapshot with duplicate pane ids, a placeholder without a card, or a silent-delete placeholder;
- a bundle that flattens authority into topology, carries a dangling reference, or claims an exact
  restore over a snapshot that holds a placeholder pane;
- a schema-id registry or per-object schema version that disagrees with the build.

## Reuse, not reinvention

This packet is the canonical M5 truth source for the remembered-state objects. Crash recovery,
browser/companion handoff, import/export, and support packets resolve these objects rather than
re-deriving their own serialization. The classification of *which* objects may travel and *how
truthfully* they restore stays in the
[serialization-and-restore matrix](./m5-serialization-and-restore.md); this packet is *what those
classes are made of*.

[`WorkspaceAuthorityCheckpoint`]: ../../../crates/aureline-workspace/src/m5_remembered_state_objects/mod.rs
[`WindowTopologySnapshot`]: ../../../crates/aureline-workspace/src/m5_remembered_state_objects/mod.rs
[`ProfileDefaults`]: ../../../crates/aureline-workspace/src/m5_remembered_state_objects/mod.rs
[`MachineLocalHints`]: ../../../crates/aureline-workspace/src/m5_remembered_state_objects/mod.rs
[`RememberedStateBundle`]: ../../../crates/aureline-workspace/src/m5_remembered_state_objects/mod.rs
[`PlaceholderCard`]: ../../../crates/aureline-workspace/src/m5_remembered_state_objects/mod.rs
