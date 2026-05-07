# Cold-start trace cases

These fixtures are **review and harness seeds** for cold-start ordering.
They are not performance captures; they assert ordering, readiness cues, and
required degraded disclosures so reviewers can tell when a startup change would
violate the “shell-ready before full graph warm-up” rule.

Stage and vocabulary sources (do not invent parallel names):

- Benchmark segment ids: `artifacts/benchmarks/journey_segment_ids.yaml` (`seg.startup.*`)
- Ready cues / partiality: `schemas/recovery/hydration_phase_event.schema.json`
- Background job kinds and workload lanes: `artifacts/runtime/queue_lane_matrix.yaml`
- Hot-set shard identity: `artifacts/search/shard_rows.yaml` (`search_shard_row:quick_open.hot_set_lexical`)
- Watcher degraded disclosure: `docs/fs/path_truth_packet.md`

Each YAML file includes:

- `expected_trace`: which startup segments must exist, which are allowed to be
  in-flight at `shell_ready`, and which must not be a prerequisite.
- `interactive_threshold`: the ready cues required to claim “interactive”.
- `background_admission`: which job kinds are protected vs deferred.
- `degraded_behavior`: required disclosures when watchers, indexes, or graph
  shards are unavailable (no silent “Ready” overclaiming).

