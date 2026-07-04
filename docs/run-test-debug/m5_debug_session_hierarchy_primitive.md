# M5 Debug-Session-Hierarchy Primitive

Status: stable (M05-824, batch B96)

The reusable execution-lifecycle component matrix
([`m5-execution-lifecycle-component-matrix.schema.json`](../../schemas/ui/m5-execution-lifecycle-component-matrix.schema.json),
frozen in M05-820) *freezes* the run/attempt/input-request/artifact-publish/rerun/debug
component families as a governed contract. This primitive *narrows* the three **debug**
families of that matrix — `debug_session_header`, `thread_process_tree`, and
`dump_crash_artifact_card` — into one working resolver with a real, tested implementation.
It closes the B96 execution-lifecycle lane opened by the run/attempt-header primitive
([`m5_run_attempt_header_primitive.md`](m5_run_attempt_header_primitive.md), M05-821), the
input-request / artifact-publish primitive
([`m5_input_request_artifact_publish_primitive.md`](m5_input_request_artifact_publish_primitive.md),
M05-822), and the rerun-comparison-sheet primitive
([`m5_rerun_comparison_sheet_primitive.md`](m5_rerun_comparison_sheet_primitive.md),
M05-823).

A single bounded **debug session** — one launched, attached, core, replay, or inspect-only
session — projects onto five surfaces that share one session identity and one target
identity:

- a **debug session header** (`M5ResolvedDebugSessionHeader`),
- a set of **thread / process tree rows** (`M5ResolvedDebugTreeRow`),
- a set of **dump / crash artifact cards** (`M5ResolvedDumpCrashCard`),
- a **CLI / headless line** (`M5ResolvedDebugCliLine`), and
- a **support-export projection** (`M5ResolvedDebugExport`).

The resolver is
`resolve_debug_hierarchy(&M5DebugHierarchyInput) -> Result<M5ResolvedDebugHierarchy, M5DebugHierarchyError>`
in
[`crates/aureline-runtime/src/implement_the_m5_debug_session_header_thread_process_tree_and_dump_crash_artifact_card_primitive`](../../crates/aureline-runtime/src/implement_the_m5_debug_session_header_thread_process_tree_and_dump_crash_artifact_card_primitive).
The boundary schema is
[`schemas/ui/m5-debug-session-hierarchy.schema.json`](../../schemas/ui/m5-debug-session-hierarchy.schema.json).

The surface family enum is reused verbatim from the run/attempt-header primitive
(`M5RunAttemptSurfaceFamily`, ten execution surfaces: task, test, request, notebook, AI,
publish, preview, history, support, companion) so later M5 rows cannot invent a parallel
debug-surface vocabulary. The session mode (`M5DebugSessionMode`), symbolication state
(`M5SymbolicationState`), retention class (`M5RetentionClass`), truth class, locality, and
downgrade-trigger vocabularies are reused from the frozen matrix.

## Live attached control versus captured analysis (AC2)

The control posture is **derived purely from the session mode**, never supplied
independently, so the same mode always reads as the same posture:

| Session mode | Control posture (`M5DebugControlPosture`) |
| --- | --- |
| `launch`, `attach` | `live_attached_control` |
| `core`, `replay` | `captured_analysis` |
| `inspect_only` | `inspect_only_view` |

Honesty is enforced against the truth class: a `live_attached_control` session must be
`live` truth and its adapter (`M5DebugAdapterState`) must be able to carry live control
(`connected` / `restored`); a captured or inspect-only session must **never** be `live`
truth. A stop reason of `crash_capture` is rejected for a live-control session, and a
`running` stop reason is rejected for a captured session. A **dump card never offers a
live-control action** (`continue_execution` / `pause_execution` / `detach_session`), and a
captured / inspect-only tree row is likewise forbidden from offering one — so captured
evidence can never masquerade as live control.

## Thread / process hierarchy, never flattened (AC1)

Each tree node (`M5DebugTreeNodeInput`) carries a `node_kind` (`process` / `thread`), an
optional `parent_ref`, a `thread_count`, a `run_state` (`M5ThreadRunState`), and an
`is_selected` flag. The resolver rejects an empty tree, a duplicate node, a dangling
parent, a tree with no root, a running process claiming zero threads, more than one
selected thread, and a `selected_thread_ref` that is not a thread in the tree or disagrees
with the selected node. It computes each node's `depth` from its parent chain so the
hierarchy is preserved rather than collapsed into a flat list. The debug hierarchy stays
understandable even when the session is **restored**, **degraded**, or **inspect-only** —
the export carries one node summary per tree row, and every projection carries the identity
and control posture.

## Dump provenance and symbolication, preserved (AC3)

Each dump card (`M5DumpCardInput`) names its `dump_ref`, its `producing_run_ref` (lineage),
its `artifact_kind` (`M5DumpArtifactKind`), its `symbolication` state, its
`capture_time_label`, and both a `build_provenance_label` and a `symbol_provenance_label`.
The resolver rejects a dump with an empty ref, broken lineage, missing build / symbol
provenance, a missing capture time, a duplicate ref, or a live-control action. A dump card
is always `captured_truth` and never implies live control.

## Redaction

Raw process memory, register bytes, dump payloads, symbol blobs, credentials, provider
cursors, and URLs never cross this boundary. The resolver rejects obviously forbidden
material in any ref or label, and the packet's `validate()` re-scans the serialized export
for forbidden material.

## Checked-in proof

The seeded packet (`seeded_m5_debug_hierarchy_packet`) binds all ten execution surfaces to
the shared contract with one worked, self-consistent case each, covering every session
mode, every control posture, and every symbolication state. The checked-in support export
([`artifacts/release/m5-debug-session-hierarchy-primitive-proof/support_export.json`](../../artifacts/release/m5-debug-session-hierarchy-primitive-proof/support_export.json))
is regenerated by the `dump_m5_debug_session_hierarchy_primitive` example and is the
`include_str!` canonical the tests assert byte-for-byte against the in-crate builder.
