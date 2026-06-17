# M5 serialization qualification

The **serialization qualification** packet is the certification layer above the
[serialization-and-restore matrix](m5-serialization-and-restore.md). It turns serialization and
restore fidelity into named qualification rows — with freshness, downgrade rules, and automatic
claim narrowing — instead of implicit "restore supported" expectations attached to any restorable
UI.

- Typed model + gate: `aureline-workspace` crate, `m5_serialization_qualification`
- Packet: `artifacts/workspace/m5/m5-serialization-qualification.json`
- Reviewer artifact: `artifacts/workspace/m5/m5-serialization-qualification.md`
- Schema: `schemas/workspace/m5-serialization-qualification.schema.json`
- Fixtures: `fixtures/workspace/m5/m5-serialization-qualification/`
- Shiproom claim packet: `artifacts/shiproom/m5-serialization-claim-packet/m5_serialization_claim_packet.md`
- Qualifies matrix: `artifacts/workspace/m5/m5-serialization-and-restore-matrix.json`

## Why this packet exists

M5 keeps adding multi-pane, multi-window, restore-heavy surfaces — preview routes, notebook
sessions, query consoles, profiler captures, docs panes, incident workspaces, companion handoff
packets, and portable-state artifacts. The serialization-and-restore matrix already says *what* each
artifact class and surface may remember and restore. What was still implicit is *whether each claim
is actually qualified on the rows where Aureline claims it* — with current evidence, a downgrade
rule, and a recovery path. This packet makes that explicit:

- It publishes machine-readable qualification rows for the five serialization **families** —
  remembered-state inspection, restore fidelity, portable-state review, migration/remap handling,
  and missing-surface continuity — across claimed **profiles** and **deployment modes** (desktop,
  managed fleet, companion/browser).
- It auto-narrows docs, support packets, companion/browser handoff copy, and release-center claim
  surfaces when evidence is stale, missing, or downgraded.
- It carries the schema-jump, foreign-package, display-topology, missing-extension, and
  placeholder-continuity drills in its qualification corpus.
- It exposes evidence freshness and downgrade reasons in the shiproom claim packet.

## Restore-fidelity vocabulary (reused from the matrix)

The qualification never invents restore language. It reuses the matrix's four restore-fidelity
classes, highest to lowest:

| Class | Meaning |
| --- | --- |
| `exact_restore` | Remembered state restored value-for-value. |
| `compatible_restore` | Restored through a forward schema migration or adapted topology; semantics preserved. |
| `layout_only` | Only pane/window layout restored; contents reopen as context or a placeholder. |
| `manual_review` | Cannot be restored automatically; surfaced for review with the slot preserved. |

## The fail-closed gate

Each row declares a target fidelity (`declared_fidelity`) and records three independent inputs:

1. **Matrix claim** (`matrix_claim`) — the fidelity the serialization matrix published for the
   row's artifact class or surface. The gate can only narrow from here; it never re-broadens a
   matrix-narrowed surface.
2. **Evidence freshness** (`evidence_freshness`) — `current` → `exact_restore`, `aging` →
   `compatible_restore`, `expired` → `layout_only`, `missing` → `manual_review`. This mapping is
   identical to the matrix's own freshness condition, so a stale qualification narrows in lockstep.
3. **Drill outcomes** — the seven qualification drills (`schema_jump`, `foreign_package`,
   `display_topology`, `missing_extension`, `placeholder_continuity`, `accessibility`, `downgrade`).
   A `passed` drill backs `exact_restore`, a `narrowed` drill caps at `compatible_restore`, and a
   `failed` or `not_run` drill caps at `manual_review`. A missing required drill caps the row at
   `manual_review`, so an incompletely drilled row is never qualified by omission.

The **published fidelity** is the weakest of the declared fidelity, the matrix claim, the freshness
ceiling, and the drill ceiling. The **claim-publication** decision follows from it:

- `published` — the row qualified its declared fidelity with fresh evidence and clean drills; the
  full claim stands.
- `narrowed` — the gate lowered the fidelity below the declared maximum; the claim is auto-narrowed
  to the qualified fidelity, with a downgrade reason, a recovery path, and the stale/missing fields
  named.
- `withheld` — the row qualifies only for `manual_review`; no restore claim is published.

### Downgrade reasons and recovery paths

| Reason | Raised when | Recovery path |
| --- | --- | --- |
| `matrix_narrowed` | the matrix claim is below `exact_restore` | `adopt_matrix_narrowing` |
| `evidence_stale` | evidence is `aging`, `expired`, or `missing` | `refresh_evidence` |
| `drill_narrowed` | a drill proved only a narrower slice | `rerun_drills` |
| `drill_failed` | a drill failed or never ran | `rerun_drills` |

A row whose effective fidelity is `manual_review` takes the `withhold_claim` recovery path; a clean
published row takes `none`. The recorded reasons, path, and publication decision must equal the
recomputed gate, so a downgrade can never be asserted or hidden by hand.

## Guardrails

- **No inheritance.** Every claimed family carries at least one row, and each
  `(family, profile, deployment_mode)` row is proven independently — a profile is never marked green
  because a nearby profile passed a superficially similar restore flow.
- **No silent layout loss.** Missing-surface rows narrow to `layout_only` or `manual_review` with
  slot-preserving placeholders; the matrix's `silent_delete` behavior is never reachable from a
  qualified row.
- **No over-portability.** Portable-state-review rows reuse the matrix redaction vocabulary; the
  qualification never claims full portability where a package depends on machine-local state or an
  unsupported feature pack.
- **One source of truth.** Docs/help, support export, companion/browser handoff, release center, and
  shiproom each bind to this packet through a consumer binding that must ingest it, preserve its
  published fidelity and recovery paths, and narrow with it. A row narrowed here cannot stay
  authoritative downstream.

## Consuming the packet

Downstream surfaces render `M5SerializationQualification::export_projection()` rather than restating
each row's fidelity by hand, and support/evidence bundles carry
`M5SerializationQualification::support_export(..)`, which preserves the exact report and excludes raw
private material. The packet is metadata-only: every field is a typed state, a count, or an opaque
ref, and it carries no credential bodies, raw provider payloads, live authority handles, or
workspace contents.
