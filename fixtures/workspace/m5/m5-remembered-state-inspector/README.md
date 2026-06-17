# M5 remembered-state inspector fixtures

Scenario fixtures for the remembered-state inspector. Each file is a single
[`InspectorRow`](../../../../crates/aureline-workspace/src/m5_remembered_state_inspector/mod.rs)
that the crate's unit tests deserialize and assert against; the canonical full packet lives at
`artifacts/workspace/m5/m5-remembered-state-inspector.json` and is exercised by the embedded-packet
tests and the fail-closed gate drills.

| Fixture | Proves |
| --- | --- |
| `local_only_no_export.json` | A machine-local class is visible, inspectable, comparable, and clearable, but offers **no export** — local-only state never leaves the machine. |
| `portable_exportable.json` | A portable class is exportable and offers a bounded export action scoped to the selected class. |
| `bounded_clear.json` | A clear action is **confirmed** and **scoped to the selected class only**, excluding unrelated content and caches — it never looks like a destructive global reset. |

The labels (`ownership`, `published_fidelity`) are reused from the serialization-and-restore matrix
vocabulary rather than redefined, so the inspector cannot fork remembered-state meaning. The
fail-closed rejections — exportability mismatch, non-exportable offering export, missing inspect
action, global-reset clear, inaccessible affordance, duplicate focus order, missing/ drifted
consumer binding, and summary mismatch — are exercised as synthetic gate drills in the crate's
`m5_remembered_state_inspector` unit tests rather than as checked-in invalid fixtures.
