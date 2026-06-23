# M5 chronology, replay, timeline-bookmark, and notebook-debug-parity evidence

This set is the checked-in proof path for Aureline's typed M5 chronology-capability
descriptors, replay sessions, timeline bookmarks, notebook-kernel capability descriptors,
cell-frame links, and restart/reconnect consequence records: the canonical records every
live-debug, replay, notebook, profiler, AI, and support surface reads to know what
time-travel and notebook-debug a backend supports, what a replay reconstructed and from
which capture, where a timeline bookmark is pinned, and what a restart or reconnect
preserved, lost, invalidated, or left stale. It materializes the chronology-capability,
replay-session, and notebook-debug-parity families named by the
[M5 debug-contracts matrix](./m5_debug_contracts.md).

The published set is
[`fixtures/debug/m5_chronology_replay_parity/canonical_set.json`](../../fixtures/debug/m5_chronology_replay_parity/canonical_set.json),
frozen against `crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs` by the gate at
`crates/aureline-debug/tests/m5_chronology_replay_parity.rs`.

## Materialized chronology capabilities

| Descriptor | Backend | Support | Timeline | Time-travel | Notebook parity |
|---|---|---|---|---|---|
| `debug.chronology:local_native_supported:0001` | local_native | supported | recorded_complete | yes | mirrored |
| `debug.chronology:remote_helper_partial:0002` | remote_helper | limited | recorded_partial | yes (subset) | divergent |
| `debug.chronology:container_recording:0003` | container | supported | recording | yes | mirrored |
| `debug.chronology:browser_unavailable:0004` | browser_runtime | unavailable | unavailable | no | not_applicable |
| `debug.chronology:managed_policy_blocked:0005` | managed_runtime | policy_blocked | live_no_recording | no | unsupported |
| `debug.chronology:remote_helper_expired:0006` | remote_helper | unavailable | expired | no | unsupported |

The descriptors materialize the full support-class vocabulary (supported, limited,
unavailable, policy_blocked), most timeline states, and prove the guardrail: the browser,
managed, and expired backends back zero verbs and inherit no time-travel from the supported
neighbors.

## Materialized replay sessions, bookmarks, kernels, and links

| Record | Detail |
|---|---|
| `debug.replay:local_native_active:0001` | supported, inspect-only, replay_active, full replay verbs, reacquire consequence |
| `debug.replay:container_mismatched:0002` | limited, inspect-only, mismatched capture, no replay verbs until re-recorded |
| `debug.bookmark:user_request_entry:0001` | user_set, bound to the local capture, survives export/restore |
| `debug.bookmark:auto_db_commit:0002` | auto_event, same capture identity |
| `debug.bookmark:error_unhandled:0003` | error_stop, same capture identity |
| `notebook.kernel:python_local_supported:0001` | supported, full debug verbs, breakpoints preserved on restart |
| `notebook.kernel:python_remote_limited:0002` | limited, subset of verbs, fresh session on restart |
| `notebook.kernel:managed_policy_blocked:0003` | policy_blocked, no verbs, transport-lost reconnect consequence |
| `notebook.cell_frame:exact_current:0001` | exact + supported → renders exact link |
| `notebook.cell_frame:approximate_nearest:0002` | approximate → never exact |
| `notebook.cell_frame:stale_after_restart:0003` | stale → never exact |
| `notebook.cell_frame:unmapped_blocked:0004` | unmapped on a policy-blocked kernel → never exact |

## Materialized restart/reconnect consequences

Each record itemizes all five subjects (variables, queued cells, debug state, breakpoints,
transient outputs) with one disposition each — never a flattened banner.

| Consequence | Trigger | Sample dispositions |
|---|---|---|
| `debug.consequence:session_restart:0001` | session_restart | breakpoints preserved; variables/debug-state/outputs lost |
| `debug.consequence:debug_reconnect:0002` | reconnect | variables/breakpoints preserved; debug-state invalidated; queued/outputs stale |
| `debug.consequence:kernel_restart_preserved:0003` | kernel_restart | breakpoints preserved; the rest lost |
| `debug.consequence:kernel_restart_reset:0004` | kernel_restart | everything lost (fresh session) |
| `debug.consequence:kernel_transport_lost:0005` | transport_lost_reconnect | breakpoints preserved; debug-state/queued invalidated; variables/outputs stale |
| `debug.consequence:replay_reacquire:0006` | replay_reacquire | variables/debug-state/breakpoints preserved; outputs stale; no live queue |

The set materializes the full disposition vocabulary (preserved, lost, invalidated, stale)
and the full trigger vocabulary across notebook, debug, and replay sessions.

## Proof claims

| Claim | Evidence |
|---|---|
| Replay, notebook, and live-debug surfaces reuse one chronology/support-class vocabulary and one export model | invariants `capability.support_class_vocabulary_complete` + `capability.one_shared_support_vocabulary` + the `one_support_vocabulary_is_shared_across_surfaces` test |
| An unsupported runtime never inherits a neighboring backend's replay or notebook-debug claim | invariant `capability.no_inherited_claims_across_backends` + the `unsupported_backends_inherit_no_claims` test |
| Time-travel verbs are backed only when a recorded/replayable timeline supports them | invariant `capability.time_travel_verbs_backed_only_when_replayable` + the `time_travel_verbs_are_backed_only_when_replayable` test |
| Replay sessions are inspect-only and bound to the capture they reconstruct | invariant `replay.inspect_only_and_capture_bound` + the `making_a_replay_session_mutable_fails_validation` test |
| Timeline bookmarks remain bound to one capture/session/target identity and survive support export and restore review | invariant `bookmark.bound_to_one_capture_and_survives_export` + the `orphaning_a_bookmark_from_its_capture_fails_validation` test |
| Restart/reconnect surfaces explain — per subject — what variables, queued cells, debug state, breakpoints, and transient outputs were preserved versus lost | invariants `consequence.itemized_never_flattened` + `consequence.required_subjects_complete` + the `flattening_a_restart_consequence_fails_validation` test |
| Restart consequences are not flattened into a generic reconnect banner | invariant `consequence.itemized_never_flattened` + the `every_restart_consequence_itemizes_the_five_subjects` test |
| Restart/reconnect consequences exist for notebook, debug, and replay sessions | invariant `consequence.covers_notebook_debug_and_replay` + the `restart_consequences_cover_notebook_debug_and_replay` test |
| A remapped or degraded frame-to-cell link is never drawn exact | invariant `link.exact_only_when_exact_and_supported` + the `drawing_a_stale_link_exact_fails_validation` test |
| Support/export packets retain chronology, replay, and parity state rather than flattening them into rendered chrome | invariant `set.export_retains_capability_state` + the `fixture_round_trips_and_is_export_safe` test |
| Every cited proof packet and producer exists on disk | the `every_proof_packet_and_producer_exists_on_disk` freeze-gate test |

## Verification

```sh
cargo test -p aureline-debug
```
