# M5 restore-provenance fixtures

Scenario fixtures for restore-provenance cards. Each file is a single
[`RestoreProvenanceCard`](../../../../crates/aureline-workspace/src/m5_restore_provenance/mod.rs)
that the crate's unit tests deserialize and assert against; the canonical full packet lives at
`artifacts/workspace/m5/m5-restore-provenance.json` and is exercised by the embedded-packet tests
and the fail-closed gate drills.

| Fixture | Proves |
| --- | --- |
| `exact_desktop_restore.json` | A clean desktop restore from an automatic checkpoint publishes **exact** continuity — pristine schema/dependency/topology/evidence, no downgrade reason, no recovery step. |
| `compatible_import.json` | A portable-state import that *claimed* exact but was forward-migrated and adapted to a different display is narrowed by the gate to **compatible**, with a compatible-restore next step. |
| `manual_review_crash_recovery.json` | A crash-recovery snapshot whose schema cannot be migrated is held for **manual review** with the layout slot preserved — never auto-applied, never silently deleted. |
| `layout_only_companion_handoff.json` | A browser/companion re-entry is capped at a **layout-only** contextual reopen by its source ceiling, so it can never imply a full restore. |

The fidelity labels (`exact_restore`, `compatible_restore`, `layout_only`, `manual_review`), the
artifact-class labels, and the redaction-exclusion labels are reused from the serialization-and-restore
matrix vocabulary rather than redefined, so restore meaning cannot fork between desktop restore,
import, crash recovery, support replay, and companion/browser re-entry.

The fail-closed rejections — overstated fidelity, a handoff implying a full restore, a silent layout
delete, a missing redaction exclusion, a dropped open-details/compare/recovery action, an
inaccessible or unscoped affordance, a downgrade-reason or recovery-path mismatch, an exact card that
is not clean, and a missing or drifted consumer binding — are exercised as synthetic gate drills in
the crate's `m5_restore_provenance` unit tests rather than as checked-in invalid fixtures.
