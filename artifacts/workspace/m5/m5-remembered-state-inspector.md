# M5 remembered-state inspector — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/workspace/m5/m5-remembered-state-inspector.json`. The full contract lives in
`docs/workspace/m5/m5-remembered-state-inspector.md`; the typed model lives in the
`aureline-workspace` crate (`m5_remembered_state_inspector`).

This packet is the **one inspectable place** where users and operators see what the current windows
and workspace will remember, what is portable, and what can be cleared — without reading logs or raw
JSON. It **projects**, rather than redefines, the serialization-and-restore matrix and the
remembered-state objects: ownership and restore-fidelity labels are reused from the matrix row for
each class.

## The inspected classes (as of 2026-06-16)

| Class | Visibility | Exportable | Restores | Actions |
| --- | --- | --- | --- | --- |
| Workspace authority checkpoint | local | no | exact | inspect, compare, clear |
| Window topology snapshot | machine-local | no | compatible | inspect, compare, clear |
| Portable state package | portable | **yes** | compatible | inspect, compare, export, clear |
| Restore provenance record | shared | **yes** | compatible | inspect, compare, export, clear |
| Placeholder cards | local | no | layout-only | inspect, compare, clear |
| Compare / export summaries | shared | **yes** | manual-review | inspect, compare, export, clear |

Three classes are exportable (portable or shared); three are local-only or machine-local and stay
visible but are never exported. Every class is inspectable, comparable, and clearable.

## What each action guarantees

- **Inspect / compare** — read-only. Every row offers inspect, so no class's meaning is hidden behind
  a debug flag or a raw dump.
- **Export** — offered exactly when the class is exportable. The export excludes secrets, live
  authority, and machine-local anchors, so a portable package never carries machine-unique state.
- **Clear** — bounded to the one selected class and **confirmed**. It never removes user-owned content
  outside the class and never looks like a destructive global reset. The `global_reset` boundary is
  reject-only — present in the vocabulary so the gate can refuse it.

Every affordance carries a command id, a keyboard shortcut, a deterministic focus order, and a
screen-reader label, so the compare/export/clear flows are keyboard-complete and screen-reader-safe.

## Guardrails the gate enforces

- A row's `exportable` flag must follow its ownership: portable/shared is exportable, local/machine-local
  is not.
- A non-exportable class never offers export; an exportable class always does.
- Every row offers inspect; no class's meaning is hidden.
- Every action stays `selected_state_class_only` and excludes unrelated content and caches; a clear is
  always confirmed.
- The four reuse surfaces — diagnostics, crash recovery, browser/companion handoff, and support export
  — each bind to this packet and preserve its ownership and fidelity labels verbatim.

Local-only/no-export, portable/exportable, and bounded-clear scenarios are exercised as fixtures under
`fixtures/workspace/m5/m5-remembered-state-inspector/`; the fail-closed rejections are exercised as
synthetic gate drills in the crate's `m5_remembered_state_inspector` unit tests.
