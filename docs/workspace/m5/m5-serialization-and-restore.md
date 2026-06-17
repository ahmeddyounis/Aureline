# M5 workspace-serialization and restore-fidelity matrix

The **serialization-and-restore matrix** is the single machine-readable contract for everything
M5 is allowed to remember, export, compare, restore exactly, restore compatibly, or only reopen
as context. M5 keeps adding restorable surfaces — preview routes, notebook sessions, query
consoles, profiler captures, docs panes, incident workspaces, companion handoff packets, and
portable-state artifacts — and each remembers some slice of workspace state. This matrix replaces
the scattered, row-local restore assumptions with one controlled vocabulary.

- Typed model: `aureline-workspace` crate, module `m5_serialization_and_restore_matrix`.
- Packet: [`artifacts/workspace/m5/m5-serialization-and-restore-matrix.json`](../../../artifacts/workspace/m5/m5-serialization-and-restore-matrix.json).
- Reviewer artifact: [`artifacts/workspace/m5/m5-serialization-and-restore-matrix.md`](../../../artifacts/workspace/m5/m5-serialization-and-restore-matrix.md).
- Schema: [`schemas/workspace/m5-serialization-matrix.schema.json`](../../../schemas/workspace/m5-serialization-matrix.schema.json).
- Fixtures: [`fixtures/workspace/m5/m5-serialization-and-restore/`](../../../fixtures/workspace/m5/m5-serialization-and-restore/).
- Shiproom review packet: [`artifacts/shiproom/m5-serialization-review-packet/`](../../../artifacts/shiproom/m5-serialization-review-packet/).

## What the matrix holds

The packet carries two row families and two binding families:

1. **Artifact-class rows** — one per remembered-state artifact class. Each row classifies *what
   M5 may remember* and *how truthfully it restores*.
2. **Surface rows** — one per restorable M5 surface. Each row classifies *what a surface
   persists*, *how portable it is*, and *which restore-fidelity classes it supports*.
3. **Continuity cross-links** — one per continuity surface, binding the matrix to the canonical
   crash-recovery, browser/companion-handoff, import/export, and claim-publication packets so
   restore language stays canonical.
4. **Consumer bindings** — one per reviewer surface (shiproom, docs/help, support export) that
   must ingest this packet and narrow with it.

## The six remembered-state artifact classes

| Class | Persists | Ownership | Exportable |
| --- | --- | --- | --- |
| `workspace_authority_checkpoint` | re-resolvable record of granted authority, never the live authority | local | no |
| `window_topology_snapshot` | pane tree and monitor geometry | machine-local | no |
| `portable_state_package` | serialized portable workspace state | portable | yes |
| `restore_provenance_record` | source/producer/schema-outcome/fidelity of a restore | shared | yes |
| `placeholder_card` | slot-preserving stand-in for a surface that could not be restored | local | no |
| `compare_export_summary` | diff between two remembered states, for review | shared | yes |

Layout restore, portable-state export, and crash-recovery evidence are **kept distinct** here even
though they share artifacts: they are different ownership classes with different redaction policies
and different restore-fidelity ceilings.

## The four restore-fidelity classes

Ordered best to worst:

| Class | Meaning |
| --- | --- |
| `exact_restore` | the remembered state is restored value-for-value |
| `compatible_restore` | restored through a forward schema migration; semantics preserved |
| `layout_only` | only the pane/window layout is restored; contents reopen as context or show a placeholder |
| `manual_review` | cannot be restored automatically; surfaced for review with the slot preserved |

## The fail-closed fidelity gate

The achieved restore fidelity of a row is the **weakest ceiling** implied by five inputs:

| Input | Ceiling it imposes |
| --- | --- |
| `declared_max_fidelity` | the class's own best claim, never re-broadened |
| `schema_condition` | match → exact, forward-migratable → compatible, unmigratable → manual_review |
| `dependency_condition` | present → exact, partial-missing → layout_only, root-missing → manual_review |
| `topology_condition` | identical → exact, adapted → compatible, incompatible → layout_only |
| `evidence_freshness` | current → exact, aging → compatible, expired → layout_only, missing → manual_review |

`published_fidelity = min(declared_max, schema_ceiling, dependency_ceiling, topology_ceiling, freshness_ceiling)`.

A schema drift, a missing dependency, a changed topology, or stale evidence **narrows the restore
automatically** rather than leaving a surface claiming exact restore by inertia. The recorded
`published_fidelity`, `downgrade_reasons`, and `recovery_path` must each equal the gate's
recomputed value, so a downgrade can never be asserted or hidden by hand. An `exact_restore` row
must be genuinely clean — pristine conditions, no downgrade reason, recovery `none` — so an
inherited "remembers everything" badge can never sit over a surface that actually drifted, lost a
dependency, or went stale.

## Downgrade reasons and recovery paths

| Recovery path | When |
| --- | --- |
| `restore_compatibly` | the schema drifted but is forward-migratable |
| `relocate_dependency` | a dependency or root is missing |
| `reopen_as_context` | the topology changed; the layout is preserved |
| `refresh_evidence` | the evidence is aging, expired, or missing but state is otherwise restorable |
| `manual_review` | the restore cannot be applied automatically |
| `none` | the row restores exactly |

## A missing dependency never deletes layout

Every row declares a `missing_dependency_behavior`, and the matrix permits only
`placeholder_slot_preserved` or `reopen_as_context`. `silent_delete` exists in the vocabulary
only so the gate can reject it outright: a missing dependency holds the slot open as a placeholder
naming what to locate — it never silently removes layout.

## Portability is claimed, never assumed

- Only `portable` and `shared` state may be serialized into a portable-state package.
- An exportable row must exclude **secrets, live authority, machine-local anchors, and raw
  provider payloads**; the gate rejects an exportable row that drops any of these.
- A `machine_local` row is **never** exportable.

This is the guardrail against serializing live authority or hidden machine-local secrets just to
make restore look more complete.

## Surfaces cannot out-claim what they persist

A surface row's `max_supported_fidelity` may never exceed the strongest declared fidelity of the
artifact classes it persists, and its `portability` may never exceed the most portable class it
persists. A `portable` or `shared` surface must persist at least one exportable class. So a
surface can only claim an exact restore, or claim to be portable, when an artifact class it
actually persists can back that claim.

## Continuity cross-links and consumer bindings

The four continuity surfaces — crash recovery, browser/companion handoff, import/export, and claim
publication — each carry a cross-link to their canonical packet and reuse this matrix's vocabulary
rather than inventing surface-local restore language. The three reviewer surfaces — shiproom,
docs/help, and support export — each bind to this one packet, preserve its fidelity and ownership
labels verbatim, and narrow with it, so a row narrowed here cannot stay green downstream by
inertia.

## Export safety

The packet is metadata-only: every field is a typed state, a count, or an opaque ref. It carries
no credential bodies, raw provider payloads, live authority handles, or workspace contents, and
the support-export wrapper preserves the matrix verbatim with `raw_private_material_excluded = true`.
