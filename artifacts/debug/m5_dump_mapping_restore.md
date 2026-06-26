# M5 dump/mapping/restore evidence

This set is the checked-in proof path for Aureline's typed M5
dump/core-file/source-map/symbol artifact strips, the shared six-state mapping-fidelity
vocabulary, and the restore-honesty records: the canonical records every debugger dump
strip, symbolicated stack header, source-map card, restored pane, support packet, and AI
context reads to show which debug artifact was opened, how trustworthy its mapping is,
which build it belongs to, and — on restore — whether the prior process/session is gone,
inspect-only, reconnect-required, or manually relaunchable. It widens the four-state
[frame-mapping](./m5_frame_variable_snapshots.md) and [symbolication](./symbolication.md)
fidelity vocabularies into one shared vocabulary read across frames, breakpoints,
variables, and dump artifacts.

The published set is
[`fixtures/debug/m5_dump_mapping_restore/canonical_set.json`](../../fixtures/debug/m5_dump_mapping_restore/canonical_set.json),
frozen against `crates/aureline-debug/src/m5_dump_mapping_restore/mod.rs` by the gate at
`crates/aureline-debug/tests/m5_dump_mapping_restore.rs`.

## Materialized artifact strips

| Strip | Kind | Entrypoint | Fidelity | Source | Pill | Precise link |
|---|---|---|---|---|---|---|
| `debug.artifact:crash_dump_exact:0001` | crash_dump | open_crash_dump | exact | local | Exact · inspect-only | yes |
| `debug.artifact:core_file_approx:0002` | core_file | open_core_file | approximate | workspace | Approximate · approx build · inspect-only | no |
| `debug.artifact:replay_exact:0003` | replay_capture | open_replay | exact | local | Exact · inspect-only | yes |
| `debug.artifact:inspect_only_symbol_only:0004` | inspect_only_session | open_inspect_only | symbol_only | local | Symbol-only · no build id · inspect-only | no |
| `debug.artifact:symbol_pdb_exact:0005` | symbol_artifact | import_symbols_or_source_map | exact | local | Exact | yes |
| `debug.artifact:symbol_dsym_mismatch:0006` | symbol_artifact | import_symbols_or_source_map | mismatched_build | local | Build mismatch | no |
| `debug.artifact:symbol_dwarf_provider:0007` | symbol_artifact | import_symbols_or_source_map | symbol_only | provider | Symbol-only · provider · no build id | no |
| `debug.artifact:source_map_mirror_stale:0008` | source_map | import_symbols_or_source_map | approximate | mirror | Approximate · mirror · approx build | no |
| `debug.artifact:source_map_imported:0009` | source_map | import_symbols_or_source_map | imported | imported | Imported · imported · approx build | no |
| `debug.artifact:source_map_unresolved:0010` | source_map | import_symbols_or_source_map | unresolved | local | Unresolved · no build id | no |

The set materializes the full six-state mapping vocabulary (exact, approximate,
symbol-only, unresolved, imported, mismatched-build), all six artifact kinds, all five
entrypoints (the four distinct session entrypoints plus the import entrypoint), all five
source classes (workspace, local, provider, mirror, imported), and the PDB/dSYM/DWARF and
JS/TS/CSS debug formats.

## Materialized restored layouts

| Restore | Reopens | Posture | Mapping | Live continuity | Exact mapping | Action |
|---|---|---|---|---|---|---|
| `debug.restore:process_gone:0001` | inspect-only session | process_gone | unresolved | no | no | none |
| `debug.restore:inspect_only_continuation:0002` | crash dump | inspect_only_continuation | exact (still verified) | no | yes | none |
| `debug.restore:reconnect_required:0003` | replay capture | reconnect_required | imported | no | no | reconnect |
| `debug.restore:manually_relaunchable:0004` | mismatched symbol artifact | manually_relaunchable | mismatched_build | no | no | relaunch |

The set materializes the full restore-posture vocabulary, proves that even a restored
layout whose exact-build mapping is still verified never implies live continuity or process
authority, and proves that a degraded restore mapping never claims an exact-build mapping.

## Proof claims

| Claim | Evidence |
|---|---|
| Any claimed M5 debug surface can show current build/artifact identity and exact-versus-degraded mapping fidelity without forcing support-only diagnostics | invariants `artifacts.build_artifact_identity_present` + `artifacts.mapping_vocabulary_complete` + `artifacts.one_canonical_mapping_pill` |
| A precise source link renders only for an exact mapping backed by an exact-build match; an imported or build-mismatched strip never renders it | invariants `artifacts.exact_link_never_hides_degraded_mapping` + `artifacts.imported_and_mismatch_stay_honest` + the `imported_and_mismatch_never_show_exact_link` test |
| Core-file, crash-dump, open-replay, and open-inspect-only entrypoints remain distinct and visible in UI, command, and export paths, separate from importing symbols/source maps | invariant `artifacts.entrypoints_distinct_and_visible` + the `session_entrypoints_are_distinct_and_inspect_only` test |
| Mirrored and imported sources disclose their provenance rather than posing as a local-trusted one | invariant `artifacts.mirrored_and_imported_sources_disclosed` |
| Restored debug layouts never imply reacquired process authority or live target continuity | invariant `restore.never_implies_live_continuity_or_authority` + the `restored_layouts_never_imply_live_authority` and `restore_claiming_live_continuity_fails_validation` tests |
| Restored layouts never imply exact-build mapping when that is no longer true | invariant `restore.exact_build_mapping_only_when_still_verified` |
| A restored layout names whether it is gone, inspect-only, reconnect-required, or manually relaunchable | invariants `restore.posture_vocabulary_complete` + `restore.required_action_named` |
| The mapping vocabulary is one shared superset read across frames, breakpoints, variables, and dump artifacts | invariant `set.shared_mapping_vocabulary_supersets_frame_fidelity` + the `shared_vocabulary_supersets_frame_fidelity` test |
| Support/export packets retain artifact fidelity, build identity, and restore posture rather than flattening them into rendered chrome | invariant `set.export_retains_artifact_and_restore_state` + the `fixture_round_trips_and_is_export_safe` test |
| Every cited proof packet and producer exists on disk | the `every_proof_packet_and_producer_exists_on_disk` freeze-gate test |

## Verification

```sh
cargo test -p aureline-debug
```
