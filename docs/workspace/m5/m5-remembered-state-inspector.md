# M5 remembered-state inspector

The **remembered-state inspector** is the one inspectable surface where a user or operator can see
what the current windows and workspace will remember, when each class was last written, how
truthfully it restores, whether it is portable, and what can be compared, exported, or cleared —
**without reading logs or raw JSON**. It is the user-facing projection of the serialization-and-restore
matrix (which classifies *what* M5 may remember) and the remembered-state objects (which *implement*
the underlying state).

- Typed model: `aureline-workspace` crate, module `m5_remembered_state_inspector`.
- Packet: [`artifacts/workspace/m5/m5-remembered-state-inspector.json`](../../../artifacts/workspace/m5/m5-remembered-state-inspector.json).
- Reviewer artifact: [`artifacts/workspace/m5/m5-remembered-state-inspector.md`](../../../artifacts/workspace/m5/m5-remembered-state-inspector.md).
- Schema: [`schemas/workspace/m5-remembered-state-inspector.schema.json`](../../../schemas/workspace/m5-remembered-state-inspector.schema.json).
- Fixtures: [`fixtures/workspace/m5/m5-remembered-state-inspector/`](../../../fixtures/workspace/m5/m5-remembered-state-inspector/).
- Classification contract: [`docs/workspace/m5/m5-serialization-and-restore.md`](./m5-serialization-and-restore.md).
- Underlying objects: [`docs/workspace/m5/m5-remembered-state-objects.md`](./m5-remembered-state-objects.md).

## One row per remembered-state class

Each [`InspectorRow`] inspects one remembered-state artifact class relevant to the current
windows/workspace and exposes, in plain language:

- the **artifact class** and a human-readable **title**;
- the **last-write time** and the **schema version** of the underlying object;
- **producer/build provenance** — which component and build wrote it;
- a **portable / shared / local / machine-local** ownership label and whether it is **exportable**;
- the **restore fidelity** it claims; and
- **what it will remember** and **what it will not** — so the meaning is legible without opening a
  raw file.

The artifact-class, ownership, and restore-fidelity vocabularies are **reused** from the
serialization-and-restore matrix, never redefined, so remembered-state meaning cannot fork by
surface. The inspector's ownership and fidelity labels for each class match the matrix row for that
class exactly.

## Inspect, export, compare, clear — bounded and accessible

Each row carries [`ActionAffordance`]s for inspect, export, compare, and clear. Every affordance:

- is bounded to the **selected class only** (`selected_state_class_only`) and **excludes unrelated
  content and caches**, so no action can silently widen its blast radius;
- carries a **command id**, a **keyboard shortcut**, a deterministic **focus order**, and a
  **screen-reader label**, so every flow is keyboard-complete and screen-reader-safe.

The actions differ in what they do:

- **Inspect** and **compare** are read-only. Every row offers inspect, so a class's meaning is never
  hidden behind a debug flag or a raw dump.
- **Export** is offered **exactly when the class is exportable**. Portable and shared state is
  exportable; local-only and machine-local state stays visible but is **never** exported.
- **Clear** is the only action that removes remembered state. It is bounded to the selected class and
  **requires confirmation**, so it never looks like a destructive global reset and never removes
  user-owned content outside the class.

## Fail-closed validation

`M5RememberedStateInspector::validate` returns a typed violation for each of:

- a row whose `exportable` flag disagrees with its ownership;
- a non-exportable class that offers export, or an exportable class that omits it;
- a row that hides its meaning by omitting the inspect action;
- an affordance missing a command id, keyboard shortcut, or screen-reader label, or two affordances
  sharing a focus order;
- a clear (or any action) modeled as a `global_reset` or touching unrelated content/caches, or a
  clear that is unconfirmed;
- a required reuse surface with no preserving consumer binding, or a binding that drops a label;
- a closed-vocabulary array or summary roll-up that disagrees with the build.

`global_reset` exists in the boundary vocabulary only so the gate can reject it; a persisted clear is
always `selected_state_class_only`.

## Reuse, not reinvention

[`ConsumerBinding`] wires the four reuse surfaces — **diagnostics**, **crash recovery**,
**browser/companion handoff**, and **support export** — to this one packet, each attesting that it
reuses the inspector vocabulary and preserves the ownership and fidelity labels verbatim. Those
surfaces render the inspector's labels rather than inventing their own, so remembered-state meaning
stays canonical across the product. `M5RememberedStateInspector::inspect_view` produces the
plain-language projection those surfaces render.

[`InspectorRow`]: ../../../crates/aureline-workspace/src/m5_remembered_state_inspector/mod.rs
[`ActionAffordance`]: ../../../crates/aureline-workspace/src/m5_remembered_state_inspector/mod.rs
[`ConsumerBinding`]: ../../../crates/aureline-workspace/src/m5_remembered_state_inspector/mod.rs
