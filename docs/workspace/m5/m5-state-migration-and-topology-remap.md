# M5 state migration and display-topology remap

Remembered state becomes untrustworthy in three specific ways the generic restore success/failure
signal hides: the **schema changes**, a package comes **from another machine or org**, or the
**display topology shifts** enough to change where windows land. This lane makes each of those an
explicit, reviewable **migration/remap event** with downgrade truth, so a restore never quietly
changes layout or silently drops a meaningful migration or remap decision.

It extends the restore-provenance lane rather than inventing new restore language: every event reuses
the **same** exact/compatible/layout-only/manual-review fidelity classes, the same schema /
dependency / topology / evidence-freshness conditions, the same downgrade reasons, and the same
recovery paths the serialization-and-restore matrix defines.

- Typed model: `aureline-workspace` crate, module `m5_state_migration_and_topology_remap`.
- Packet: [`artifacts/workspace/m5/m5-state-migration-and-topology-remap.json`](../../../artifacts/workspace/m5/m5-state-migration-and-topology-remap.json).
- Reviewer artifact: [`artifacts/workspace/m5/m5-state-migration-and-topology-remap.md`](../../../artifacts/workspace/m5/m5-state-migration-and-topology-remap.md).
- Schema: [`schemas/workspace/m5-state-migration-and-topology-remap.schema.json`](../../../schemas/workspace/m5-state-migration-and-topology-remap.schema.json).
- Fixtures: [`fixtures/workspace/m5/m5-state-migration-and-topology-remap/`](../../../fixtures/workspace/m5/m5-state-migration-and-topology-remap/).
- Restore-fidelity contract: [`docs/workspace/m5/m5-restore-fidelity.md`](./m5-restore-fidelity.md).
- Classification contract: [`docs/workspace/m5/m5-serialization-and-restore.md`](./m5-serialization-and-restore.md).

## The three event kinds

Each [`MigrationRemapEvent`] records exactly one kind, with the matching detail block:

- **Schema migration** — the stored schema changed. The detail records the **result class**
  (`schema_unchanged`, `forward_migrated`, or `unmigratable`), the schema-version jump
  (`from`/`to`), and how many forward steps ran. `forward_migrated` is the compatible-downgrade case;
  `unmigratable` is held for manual review.
- **Imported-package provenance** — a package came from elsewhere. The detail records the **origin**
  (`same_machine`, `same_org_different_machine`, `foreign_machine`, `foreign_org`), the **channel
  match** (`same_channel` / `mixed_channel`), the **path-handling posture**
  (`paths_portable_relative`, `paths_remapped_to_local_roots`, `paths_require_review`), the
  producer/version/build provenance, whether **machine-local anchors were excluded**, and whether the
  origin was **disclosed before restore**.
- **Display-topology remap** — the monitors, DPI, or fullscreen/snap state shifted. The detail records
  the **triggers** (`monitor_geometry`, `dpi_scale`, `fullscreen_snap_state`, `monitor_detached`,
  `monitor_reattached`), whether placement was **materially altered**, and the **resolution**
  (`placement_adapted_to_available_displays` → compatible, `layout_preserved_contents_reopened` →
  layout-only).

## The fail-closed gate

The published fidelity is the **weakest ceiling** implied by the declared resulting fidelity and the
schema, dependency, topology, and evidence-freshness conditions
([`MigrationRemapEvent::achieved_fidelity`]) — identical to the restore-provenance gate. On top of
that, each event-kind detail is cross-checked so the migration/remap label and the published fidelity
can never disagree:

- a **forward migration must read as a schema drift** (and `unmigratable` as manual review), so the
  result class and the schema condition always agree;
- a **mixed-channel import can never claim a clean schema match** — a different producing channel
  caps it at a compatible restore at best;
- a **remapped or unresolved path must read as a missing dependency**, so a foreign package's
  relocated paths narrow the restore rather than claiming exact continuity;
- a **display-topology remap must read as a changed topology** and must have materially altered
  placement — a remap is a deliberate compatibility downgrade, **never** layout corruption and never
  a manual-review failure.

## Guardrails

- **The prior artifact is never discarded.** Every event records `prior_artifact_retained` or
  `prior_artifact_archived`; `prior_artifact_discarded` is reject-only, so a migration or import never
  silently throws away the old remembered state.
- **A missing dependency never silently deletes layout.** The slot is preserved as a placeholder or
  reopened as context; `silent_delete` is reject-only.
- **The manual-review path is never hidden.** When a schema cannot be migrated, the event is held for
  manual review with the prior artifact retained rather than auto-applied.
- **A foreign import discloses before it surprises.** A package from another machine or org must
  exclude machine-local anchors and disclose its origin before the restore is applied.
- **An exact event is a clean no-migration baseline** — a `schema_unchanged` migration with pristine
  conditions, no downgrade reason, and no recovery step — so a downgrade is never presented as exact.

## Open-details, compare, recovery — preserved when it matters

Every event offers a read-only **open-details** action so the migration/remap decision is never
hidden. Wherever the fidelity was narrowed, the event also preserves a **compare** action (review the
migrated/remapped state against the preserved prior artifact) and a **recovery-next-step** action (the
concrete path that would restore more). Every affordance carries a command id, a keyboard shortcut, a
deterministic focus order, and a screen-reader label, and stays scoped to the one event.

## Same record everywhere — support can tell a remap from a failure

[`MigrationRemapConsumerBinding`] wires the parity surfaces — **exported diagnostics**, the **support
packet**, the **crash-recovery packet**, and the **companion handoff** — to this one packet. Each
attests that it carries the same fidelity and migration/remap labels verbatim and narrows with it, so
support can distinguish a deliberate platform remap from a generic restore failure rather than reading
a weaker surface-local summary. `M5StateMigrationAndTopologyRemap::event_view` produces the
plain-language projection those surfaces render, and
`M5StateMigrationAndTopologyRemap::support_export` preserves the record for evidence bundles.

The record is metadata only: every event excludes secrets, live authority, machine-local anchors, and
raw provider payloads.

[`MigrationRemapEvent`]: ../../../crates/aureline-workspace/src/m5_state_migration_and_topology_remap/mod.rs
[`MigrationRemapEvent::achieved_fidelity`]: ../../../crates/aureline-workspace/src/m5_state_migration_and_topology_remap/mod.rs
[`MigrationRemapConsumerBinding`]: ../../../crates/aureline-workspace/src/m5_state_migration_and_topology_remap/mod.rs
