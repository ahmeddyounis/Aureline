# M5 missing-surface-placeholder fixtures

Scenario fixtures for missing-surface placeholders. Each file is a single
[`MissingSurfacePlaceholderCard`](../../../../crates/aureline-workspace/src/m5_missing_surface_placeholders/mod.rs)
that the crate's unit tests deserialize and assert against; the canonical full packet lives at
`artifacts/workspace/m5/m5-missing-surface-placeholders.json` and is exercised by the embedded-packet
tests and the fail-closed gate drills.

| Fixture | Proves |
| --- | --- |
| `extension_layout_only.json` | A preview pane whose renderer **extension** is missing keeps its role and slot and publishes **layout-only**, with an install-dependency next step. |
| `feature_pack_layout_only.json` | A notebook whose **feature pack** is missing is capped at **layout-only** by the dependency ceiling even though its schema would forward-migrate. |
| `remote_reopen_as_context.json` | A query console whose **remote target** is unreachable **reopens as context** with its slot preserved, offering both reconnect and reopen-as-context. |
| `service_manual_review.json` | An incident workspace whose **backing service** root is missing is held for **manual review** with the slot preserved — never auto-applied, never silently deleted. |

The fidelity labels (`exact_restore`, `compatible_restore`, `layout_only`, `manual_review`), the
dependency/schema/topology/freshness conditions, the redaction-exclusion labels, the
missing-dependency behaviors, and the downgrade/recovery vocabularies are reused from the
serialization-and-restore matrix, and the re-entry-surface labels are reused from the
restore-provenance packet, so a placeholder means the same thing across desktop restore, import,
crash recovery, support replay, and companion/browser re-entry.

The fail-closed rejections — a silent layout delete, an exact restore published for a missing
surface, an overstated fidelity, a placeholder with no missing dependency, erased provenance, a
missing redaction exclusion, a dropped open-details/recovery/reopen action, an inaccessible
narration, an unscoped or duplicate-focus affordance, a downgrade-reason or recovery-path mismatch,
a duplicated pane slot, and a missing or drifted consumer binding — are exercised as synthetic gate
drills in the crate's `m5_missing_surface_placeholders` unit tests rather than as checked-in invalid
fixtures.
