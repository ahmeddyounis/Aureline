# M5 missing-surface placeholders

When an M5 workspace is restored, imported, or handed off, some panes may not be able to hydrate:
the extension, feature pack, remote target, or backing service they depend on is missing on this
machine. The layout must still read as the workspace the user left. A **missing-surface placeholder**
is that guarantee: one card per pane that could not hydrate, keeping the pane's role, its slot, its
last-known provenance, and its recovery actions visible instead of silently deleting or reshaping
layout.

- Typed model: `aureline-workspace` crate, module `m5_missing_surface_placeholders`.
- Packet: [`artifacts/workspace/m5/m5-missing-surface-placeholders.json`](../../../artifacts/workspace/m5/m5-missing-surface-placeholders.json).
- Reviewer artifact: [`artifacts/workspace/m5/m5-missing-surface-placeholders.md`](../../../artifacts/workspace/m5/m5-missing-surface-placeholders.md).
- Schema: [`schemas/workspace/m5-missing-surface-placeholders.schema.json`](../../../schemas/workspace/m5-missing-surface-placeholders.schema.json).
- Fixtures: [`fixtures/workspace/m5/m5-missing-surface-placeholders/`](../../../fixtures/workspace/m5/m5-missing-surface-placeholders/).
- Classification contract: [`docs/workspace/m5/m5-serialization-and-restore.md`](./m5-serialization-and-restore.md).
- Restore fidelity contract: [`docs/workspace/m5/m5-restore-fidelity.md`](./m5-restore-fidelity.md).

## What is missing — one closed vocabulary

A placeholder always names **what** is missing, drawn from a closed [`MissingDependencyClass`]:

- **Extension** — a required extension is missing or disabled.
- **Feature pack** — a required feature pack is not installed.
- **Remote target** — a required remote target or host is unreachable.
- **Backing service** — a required backing service is unavailable.

Each class binds to the recovery action that resolves it ([`MissingDependencyClass::primary_recovery_action`]):
an extension or feature pack is **installed**, an unreachable remote target is **reconnected**, and a
down backing service is **retried**. So a placeholder always names a concrete next step rather than a
generic "unavailable".

## What a placeholder preserves

Each [`MissingSurfacePlaceholderCard`] keeps the slot meaningful:

- the **pane role** ([`PaneRole`]) — editor, terminal, preview, notebook, query console, profiler,
  docs, incident workspace, and so on — so the slot still reads as the surface it stands in for, not
  a generic empty tab;
- the **slot** — the stable pane-tree `pane_id` and the diffable `slot_path` within the restored
  window, so the restored layout keeps its spatial meaning;
- the **last-known provenance** — the producer/version/build that wrote the pane's remembered state,
  the schema version, and the last successful attach, so the slot keeps a real history; and
- the **recovery actions** — the always-present open-details affordance plus the concrete recovery
  next step for the missing-dependency class.

## No silent layout deletion

A missing dependency never causes silent pane deletion, tab loss, or a misleading empty-state
substitution. The substitution behavior must preserve the slot ([`MissingDependencyBehavior::preserves_slot`]):
either the slot is held as a placeholder card, or the surface reopens as context with the slot
preserved. `silent_delete` exists in the vocabulary only so the gate can reject it.

## The fail-closed gate

A placeholder always describes a genuinely missing dependency, so its achieved fidelity is the
**weakest ceiling** implied by its declared fidelity and the dependency, schema, topology, and
evidence-freshness conditions ([`MissingSurfacePlaceholderCard::achieved_fidelity`]). The dependency
ceiling caps a partial-missing dependency at a slot-preserving **layout-only** restore and a
missing dependency **root** at **manual review** — so a missing surface can never publish an exact
restore, even when its schema, topology, and evidence are otherwise clean. The published fidelity
must equal the recomputed ceiling, the downgrade reasons must include the missing dependency, and the
recovery path (relocate the dependency, or manual review when the root is gone) is recomputed rather
than asserted.

## Keyboard focus and screen-reader narration

Every placeholder slot stays reachable by keyboard, and its [`PlaceholderNarration`] announces the
preserved role, the slot, the missing reason, and the recovery next step, so a screen-reader user
understands what stands in the slot and how to restore it. Every recovery affordance carries a
command id, a keyboard shortcut, a deterministic focus order, and a screen-reader label, and stays
scoped to the one pane slot.

## Same record everywhere — named classes and counts

[`PlaceholderConsumerBinding`] wires the parity surfaces — **exported diagnostics**, the **support
packet**, the **compare/export summary**, and the **companion handoff** — to this one packet. Each
attests that it carries the same record, preserves the missing-dependency-class and pane-role labels
verbatim, and names the affected-surface counts. `M5MissingSurfacePlaceholders::diagnostics_view`
produces the plain-language projection those surfaces render — per-class and per-role affected
counts — and `M5MissingSurfacePlaceholders::support_export` preserves the record for evidence bundles,
so a support packet can name which classes are missing and which pane roles they affected.

[`MissingDependencyClass`]: ../../../crates/aureline-workspace/src/m5_missing_surface_placeholders/mod.rs
[`MissingDependencyClass::primary_recovery_action`]: ../../../crates/aureline-workspace/src/m5_missing_surface_placeholders/mod.rs
[`MissingSurfacePlaceholderCard`]: ../../../crates/aureline-workspace/src/m5_missing_surface_placeholders/mod.rs
[`MissingSurfacePlaceholderCard::achieved_fidelity`]: ../../../crates/aureline-workspace/src/m5_missing_surface_placeholders/mod.rs
[`PaneRole`]: ../../../crates/aureline-workspace/src/m5_missing_surface_placeholders/mod.rs
[`PlaceholderNarration`]: ../../../crates/aureline-workspace/src/m5_missing_surface_placeholders/mod.rs
[`PlaceholderConsumerBinding`]: ../../../crates/aureline-workspace/src/m5_missing_surface_placeholders/mod.rs
