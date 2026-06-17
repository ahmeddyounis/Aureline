# M5 state migration and display-topology remap — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/workspace/m5/m5-state-migration-and-topology-remap.json`. The full contract lives in
`docs/workspace/m5/m5-state-migration-and-topology-remap.md`; the typed model lives in the
`aureline-workspace` crate (`m5_state_migration_and_topology_remap`).

This packet makes the three ways remembered state goes stale **first-class, reviewable events**: a
**schema migration**, an **imported-package provenance** disclosure, and a **display-topology remap**.
Each event uses the **same** exact/compatible/layout-only/manual-review vocabulary, conditions,
downgrade reasons, and recovery paths the serialization-and-restore matrix defines — reused, never
forked — so a migration or remap never hides a meaningful downgrade.

## The events (as of 2026-06-16)

| Scenario | Kind | Detail | Published fidelity | Recovery next step |
| --- | --- | --- | --- | --- |
| No-migration baseline | schema migration | schema unchanged (v3 → v3) | **exact** | none |
| Schema jump | schema migration | forward migrated (v1 → v3, 2 steps) | **compatible** | restore compatibly |
| Unmigratable schema jump | schema migration | unmigratable (v1 → v4) | **manual review** | manual review |
| Foreign-machine import | imported package | foreign machine, paths remapped | **layout-only** | relocate dependency |
| Mixed-channel import | imported package | same org, mixed channel | **compatible** | restore compatibly |
| Monitor detach / reattach | topology remap | detach + reattach + DPI, placement adapted | **compatible** | reopen as context |
| Incompatible topology remap | topology remap | geometry + snap, layout preserved | **layout-only** | reopen as context |

One event publishes exact, three compatible, two layout-only, and one manual-review — so all four
labels are exercised, and all three event kinds and both foreign origins are covered. Six events were
narrowed below the fidelity they declared.

## What the gate guarantees

- **Published fidelity is the weakest ceiling** of the declared fidelity and the schema / dependency /
  topology / evidence-freshness conditions. A schema drift, a relocated dependency, a changed
  topology, or stale evidence narrows the restore automatically.
- **The migration/remap label can never disagree with the published fidelity.** A forward migration
  reads as a schema drift, a mixed-channel import never claims a clean schema match, a remapped path
  reads as a missing dependency, and a topology remap reads as a changed topology.
- **A platform remap is a compatibility downgrade, not corruption.** A display-topology remap must
  have materially altered placement and is published as compatible or layout-only with a reopen-as-
  context step — never manual review and never a restore failure.
- **The prior artifact is never discarded.** Every event retains or archives it; `prior_artifact_discarded`
  is reject-only.
- **A missing dependency never silently deletes layout**, and a foreign import discloses its origin
  and machine-local exclusions before restore.
- **An exact event is a clean no-migration baseline** — `schema_unchanged`, pristine conditions, no
  downgrade reason, no recovery step.
- **Every event offers open-details**, and a narrowed event preserves the compare and
  recovery-next-step actions. Every affordance is keyboard-complete, screen-reader-labelled, and
  scoped to the one event.
- **The parity surfaces carry the same record.** Exported diagnostics, the support packet, the
  crash-recovery packet, and the companion handoff each bind to this packet and preserve its fidelity
  and migration/remap labels verbatim, so support can tell a deliberate platform remap apart from a
  generic restore failure.

The record is metadata only: every event excludes secrets, live authority, machine-local anchors, and
raw provider payloads.

Schema-jump, foreign-machine-import, mixed-channel-import, and monitor-detach/reattach scenarios are
exercised as fixtures under `fixtures/workspace/m5/m5-state-migration-and-topology-remap/`; the
fail-closed rejections — including a mixed-channel import claiming a clean schema match, a foreign
import hiding its machine-local exclusion, a discarded prior artifact, and a remap that did not
materially alter placement — are exercised as synthetic gate drills in the crate's
`m5_state_migration_and_topology_remap` unit tests.
