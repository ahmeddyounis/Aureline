# M5 remembered-state objects — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/workspace/m5/m5-remembered-state-objects.json`. The full contract lives in
`docs/workspace/m5/m5-remembered-state-objects.md`; the typed model lives in the
`aureline-workspace` crate (`m5_remembered_state_objects`).

This packet **implements** the remembered-state objects the serialization-and-restore matrix only
classifies. Restorable M5 surfaces resolve against these explicit, versioned objects instead of an
implicit layout blob.

## The four state objects (as of 2026-06-16)

| Object | Schema version | Ownership | Exportable | Keeps |
| --- | --- | --- | --- | --- |
| `WorkspaceAuthorityCheckpoint` | 1 | local | no | dirty-buffer identity, journal links, trusted roots, active worksets, restore class |
| `WindowTopologySnapshot` | 1 | machine-local topology by ref | no | pane tree (stable ids), window chrome, boundary scope refs |
| `ProfileDefaults` | 1 | portable | yes | default density, window role, default inspectors |
| `MachineLocalHints` | 1 | machine-local | **never** | display-topology hash, monitor affinity, install anchor |

The four objects are kept **separate**, never flattened into one payload. A
`RememberedStateBundle` wires them together purely by reference, which is what keeps authority,
topology, profile, and machine-local state from collapsing back into one convenience blob.

## Worked example

The checked-in packet carries one bundle (`bundle:primary-session`) that resolves:

- `ckpt:primary-session` — authority captured as a **re-resolvable reference**
  (`re_resolvable_reference`), never a live ticket; two dirty buffers (one with a draft-journal
  link), the mutation and draft journals, two trusted roots, and two worksets.
- `snap:primary-window` — a pane tree of an editor split beside a docs/preview tab group. Pane ids
  (`pane:editor-main`, `pane:docs`, `pane:preview`) are stable and unique; the preview pane has
  degraded to a slot-preserving placeholder for a non-reentrant live surface.
- `profile:default` — portable defaults that carry no machine-local anchors.
- `machine:this-host` — the one machine-local object, holding display geometry and an install
  anchor; never exportable.

Because the snapshot carries a placeholder pane, the bundle publishes `compatible_restore`, not
`exact_restore` — a bundle that claimed an exact restore over a placeholder is rejected.

## Stable pane ids and a versioned pane tree

Every pane carries a stable `pane_id` that survives split, move/float, pin, close-sibling, and
placeholder substitution, so the topology can be **diffed** (`PaneTree::diff`), **migrated**
(`migrate_pane_tree`), and **tested**. The pane tree is versioned (`pane_tree` schema version `1`);
an older tree forward-migrates to a compatible restore, and an unreadable newer tree is held for
manual review with its slots preserved rather than guessed at.

## Guardrails the gate enforces

- A checkpoint never serializes a live authority ticket; `live_ticket` exists only so the gate can
  reject it, and `excludes_live_authority` must attest the exclusion.
- A portable object (profile defaults) never carries machine-local ownership or anchors.
- Machine-local hints are never exportable.
- A missing dependency never silently deletes a slot: a degraded pane keeps its `pane_id` and
  original role behind a placeholder whose `silent_delete` behavior is reject-only.
- A bundle's authority ref must resolve to a checkpoint, never a window snapshot — authority and
  topology stay distinct objects.

Schema-evolution, partial-hydrate, and missing-dependency-substitution scenarios are exercised as
fixtures under `fixtures/workspace/m5/m5-remembered-state-objects/`; the fail-closed rejections are
exercised as synthetic gate drills in the crate's `m5_remembered_state_objects` unit tests.
