# M5 frame-mapping and variable/watch snapshot evidence

This set is the checked-in proof path for Aureline's typed M5 frame mappings and
variable/watch snapshots: the canonical records every debugger frame stack,
variables/watch pane, notebook variable explorer, replay inspector, and exported crash
reads to show which source a stack frame maps to, how trustworthy that mapping is, and
whether a value is a live read, a captured snapshot, a stale last-known value,
unavailable, or redacted. It materializes the frame-mapping and variable/watch-snapshot
families named by the [M5 debug-contracts matrix](./m5_debug_contracts.md).

The published set is
[`fixtures/debug/m5_frame_variable_snapshots/canonical_set.json`](../../fixtures/debug/m5_frame_variable_snapshots/canonical_set.json),
frozen against `crates/aureline-debug/src/m5_frame_variable_snapshots/mod.rs` by the gate
at `crates/aureline-debug/tests/m5_frame_variable_snapshots.rs`.

## Materialized frames

| Frame | Fidelity | Thread #idx | Pill | Precise link | Async boundary |
|---|---|---|---|---|---|
| `debug.frame:main_exact_current:0001` | exact | main #0 | Exact | yes | no |
| `debug.frame:main_approx_heuristic:0002` | approximate | main #1 | Approximate · approx build | no | no |
| `debug.frame:main_sourcemap_selected:0003` | approximate | main #2 | Approximate · source-map | no | no |
| `debug.frame:main_symbol_only_mismatch:0004` | symbol_only | main #3 | Symbol-only · build mismatch | no | no |
| `debug.frame:main_unmapped_async:0005` | unmapped | main #4 | Unmapped · no build id · async boundary | no | yes |
| `debug.frame:main_exact_runtime_gap:0006` | exact | main #5 | Exact · async boundary | yes | yes |
| `debug.frame:worker_exact_current:0007` | exact | worker #0 | Exact | yes | no |

The set materializes the full fidelity vocabulary (exact, approximate, symbol-only,
unmapped), every mapping provenance, every build-match outcome, every continuity class,
the current frame on two distinct threads, and a selected frame tracked distinctly from
the current frame.

## Materialized snapshots

| Snapshot | Kind | Scope | Disclosure | Surface / note |
|---|---|---|---|---|
| `debug.snapshot:local_live:0001` | variable | local | Live | live read |
| `debug.snapshot:arg_captured_replay:0002` | variable | argument | Captured | replay capture, truncated |
| `debug.snapshot:closure_stale:0003` | variable | closure | Stale | last-known value |
| `debug.snapshot:local_optimized_out:0004` | variable | local | Unavailable | reason `optimized_out` |
| `debug.snapshot:local_live_lazy:0005` | variable | local | Live | lazy handle, truncated |
| `debug.snapshot:global_secret_redacted:0006` | variable | global | Redacted | secret class, body withheld |
| `debug.snapshot:watch_live_notebook:0007` | watch | watch_expression | Live | notebook explorer |
| `debug.snapshot:watch_eval_error_notebook:0008` | watch | watch_expression | Unavailable | notebook, `evaluation_error` |

The set materializes the full disclosure vocabulary (live, captured, stale, unavailable,
redacted), both entry kinds (variable, watch), a notebook-context and a replay-context
snapshot, a lazy-loadable value, a truncated value, multiple unavailable reasons, and a
secret redaction.

## Proof claims

| Claim | Evidence |
|---|---|
| Variables, watches, and variable explorers always say whether they are live reads, captured snapshots, stale last-known state, unavailable, or redacted | invariants `snapshots.disclosure_vocabulary_complete` + `snapshots.one_canonical_disclosure_pill` + `snapshots.live_authority_only_when_truly_live` |
| Disconnected, captured, replayed, or stale state is never presented as current live program truth | invariant `snapshots.live_authority_only_when_truly_live` + the `value_implies_live_authority_only_when_truly_live` test |
| An unavailable value names its reason and withholds its body; a redacted value withholds its body | invariants `snapshots.unavailable_names_reason_and_withholds_body` + `snapshots.redacted_withholds_value_body` + the `unavailable_and_redacted_values_withhold_their_bodies` freeze-gate test |
| Frame stacks preserve current-frame identity and mapping quality without flattening exact, approximate, symbol-only, and unresolved states into one generic location link | invariants `frames.fidelity_vocabulary_complete` + `frames.exact_link_never_hides_approximate_symbol_only_unmapped_or_mismatch` + `frames.preserve_current_frame_identity_per_thread` |
| A lost mapping degrades to an explicit unmapped frame rather than a guessed location | invariant `frames.lost_mapping_degrades_to_explicit_unmapped` + the `lost_mapping_degrades_to_explicit_unmapped` test |
| A source-map mapping is never flattened into a direct exact link | invariant `frames.source_map_provenance_always_disclosed` |
| Build/artifact identity and an async/runtime boundary stay visible in frame stacks and exported evidence | invariants `frames.build_artifact_identity_preserved` + `frames.async_boundary_stays_visible` |
| Notebook variable explorers and replay/debug inspectors reuse the same snapshot vocabulary instead of inventing notebook-only or replay-only truth | invariants `snapshots.variables_and_watches_share_one_vocabulary` + `snapshots.notebook_and_replay_reuse_snapshot_vocabulary` |
| Support/export packets retain frame fidelity and value disclosure state rather than flattening them into rendered chrome | invariant `set.export_retains_frame_and_value_state` + the `fixture_round_trips_and_is_export_safe` test |
| Every cited proof packet and producer exists on disk | the `every_proof_packet_and_producer_exists_on_disk` freeze-gate test |

## Verification

```sh
cargo test -p aureline-debug
```
