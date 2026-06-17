# Fixtures: M5 workspace-serialization and restore-fidelity matrix

This directory contains fixture metadata for the `m5_serialization_and_restore_matrix` packet.

The canonical full corpus is checked in at:

`artifacts/workspace/m5/m5-serialization-and-restore-matrix.json`

and validated against:

`schemas/workspace/m5-serialization-matrix.schema.json`

## Coverage

- `workspace_authority_checkpoint`, `window_topology_snapshot`, `portable_state_package`,
  `restore_provenance_record`, `placeholder_card`, and `compare_export_summary` are the only
  remembered-state artifact classes, and each carries exactly one row — no class inherits a
  restore fidelity from an adjacent one.
- Every restore-fidelity class is exercised by a published row: `exact_restore`
  (`workspace_authority_checkpoint`), `compatible_restore` (`window_topology_snapshot`,
  `portable_state_package`, `restore_provenance_record`), `layout_only` (`placeholder_card`), and
  `manual_review` (`compare_export_summary`). This proves the gate narrows as well as certifies.
- Every ownership class is exercised: `local` (`workspace_authority_checkpoint`,
  `placeholder_card`), `machine_local` (`window_topology_snapshot`), `portable`
  (`portable_state_package`), and `shared` (`restore_provenance_record`,
  `compare_export_summary`).
- Every downgrade reason is exercised — `topology_changed`, `schema_drift`, `evidence_stale`, and
  `dependency_missing` — and every recovery path — `none`, `reopen_as_context`,
  `restore_compatibly`, `refresh_evidence`, `relocate_dependency`, and `manual_review`.
- The fidelity gate is exercised in every direction: each row's `published_fidelity`,
  `downgrade_reasons`, and `recovery_path` equal the recomputed gate, the one exact row is
  pristine (no downgrade reason, recovery `none`), and the four narrowed rows each name a caveat,
  the stale/missing field, and a recovery path.
- The portability guardrail is exercised: the three exportable rows
  (`portable_state_package`, `restore_provenance_record`, `compare_export_summary`) exclude
  secrets, live authority, machine-local anchors, and raw provider payloads; the machine-local
  `window_topology_snapshot` is never exportable.
- The missing-dependency guardrail is exercised: every row's `missing_dependency_behavior` is
  `placeholder_slot_preserved` or `reopen_as_context`; `silent_delete` never appears, and
  `placeholder_card` is the worked example of a missing dependency that preserves the slot rather
  than deleting layout.
- All eight restorable surfaces — `preview_route`, `notebook_session`, `query_console`,
  `profiler_capture`, `docs_pane`, `incident_workspace`, `companion_handoff_packet`, and
  `portable_state_artifact` — each carry exactly one row, and none claims a restore fidelity or a
  portability its persisted artifact classes cannot back.
- The four continuity surfaces (`crash_recovery`, `browser_companion_handoff`, `import_export`,
  `claim_publication`) are each cross-linked, and the three reviewer surfaces (`shiproom`,
  `docs_help`, `support_export`) each bind to this one packet and narrow with it.

The condition variants the canonical six rows do not exhibit —
`dependency_root_missing`, `topology_incompatible`, and `evidence_freshness: missing` — and the
fail-closed rejections (`silent_delete`, a machine-local export, an overstated fidelity, a dropped
redaction exclusion) are exercised as synthetic gate drills in the crate's
`m5_serialization_and_restore_matrix` unit tests.
