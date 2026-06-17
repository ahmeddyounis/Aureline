# M5 workspace-serialization and restore-fidelity matrix — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/workspace/m5/m5-serialization-and-restore-matrix.json`. The full contract and gate
semantics live in `docs/workspace/m5/m5-serialization-and-restore.md`; the typed model lives in
the `aureline-workspace` crate (`m5_serialization_and_restore_matrix`).

This matrix is the canonical M5 truth source for serialization and restore fidelity: crash
recovery, browser/mobile handoff, import/export, and support packets reuse it rather than inventing
surface-local restore language.

## Remembered-state artifact classes (as of 2026-06-16)

| Artifact class | Owner | Ownership | Export | Declared → Published | Recovery | Missing-dep behavior |
| --- | --- | --- | --- | --- | --- | --- |
| `workspace_authority_checkpoint` | workspace-continuity | local | no | exact → **exact_restore** | none | placeholder_slot_preserved |
| `window_topology_snapshot` | shell-layout | machine-local | no | exact → **compatible_restore** | reopen_as_context | placeholder_slot_preserved |
| `portable_state_package` | data-portability | portable | yes | exact → **compatible_restore** | restore_compatibly | reopen_as_context |
| `restore_provenance_record` | restore-truth | shared | yes | exact → **compatible_restore** | refresh_evidence | reopen_as_context |
| `placeholder_card` | restore-continuity | local | no | layout_only → **layout_only** | relocate_dependency | placeholder_slot_preserved |
| `compare_export_summary` | portability-review | shared | yes | compatible → **manual_review** | manual_review | reopen_as_context |

One class restores exactly (`workspace_authority_checkpoint`), proving the gate is not a blanket
downgrade; the other five narrow automatically on a changed topology, a forward-migratable schema,
aging evidence, a missing dependency, or an unmigratable-schema-plus-expired-evidence pair. Every
published fidelity equals the gate's recomputed ceiling and never exceeds the class's declared
maximum.

## How each class narrows

- `workspace_authority_checkpoint` — schema match, dependencies present, identical topology,
  current evidence: restored exactly, and the live authority is never serialized.
- `window_topology_snapshot` — the display topology changed, so the pane tree adapts to the
  current monitors rather than claiming identical coordinates; it is machine-local and never
  exported.
- `portable_state_package` — imported into a newer build, so state is forward-migrated; it
  excludes secrets, live authority, and machine-local anchors before it travels.
- `restore_provenance_record` — provenance evidence is aging, so the trusted fidelity narrows to
  compatible and asks for a refresh.
- `placeholder_card` — a referenced surface is missing; the slot is preserved as a placeholder
  naming what to locate. This is the proof that a missing dependency never deletes layout.
- `compare_export_summary` — the stored schema cannot be migrated and the diff evidence is
  expired, so the summary drops to manual review rather than implying a safe restore.

## Restorable surfaces

| Surface | Persists | Max fidelity | Portability | Continuity |
| --- | --- | --- | --- | --- |
| `preview_route` | topology snapshot, placeholder | layout_only | local | crash_recovery |
| `notebook_session` | authority checkpoint, provenance | exact_restore | local | crash_recovery, import_export |
| `query_console` | authority checkpoint, topology snapshot | compatible_restore | local | crash_recovery |
| `profiler_capture` | portable package, compare summary | compatible_restore | shared | import_export, claim_publication |
| `docs_pane` | topology snapshot, placeholder | layout_only | machine-local | crash_recovery |
| `incident_workspace` | authority checkpoint, provenance, placeholder | compatible_restore | local | crash_recovery, claim_publication |
| `companion_handoff_packet` | portable package, provenance | compatible_restore | portable | browser_companion_handoff, import_export |
| `portable_state_artifact` | portable package, compare summary, provenance | compatible_restore | portable | import_export, browser_companion_handoff, claim_publication |

No surface claims a restore fidelity or a portability its persisted artifact classes cannot back.

## Cross-links

The matrix cross-links to the four continuity surfaces — `crash_recovery`,
`browser_companion_handoff`, `import_export`, and `claim_publication` — each reusing this matrix's
vocabulary. The reviewer surfaces `shiproom`, `docs_help`, and `support_export` each bind to this
one packet and narrow with it: a class narrowed here cannot stay green on a shiproom row, a docs
badge, or a support export.

## Guardrail

Layout restore, portable-state export, and crash-recovery evidence are not treated as equivalent
just because they share artifacts: each artifact class carries its own ownership, redaction
policy, and restore-fidelity ceiling. No live authority, secret, or machine-local state is
serialized or implied portable to make restore look more complete. The packet is metadata-only and
carries no credential bodies, raw provider payloads, or workspace contents.
