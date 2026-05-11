# Docs-pack and example freshness validation lane

This directory hosts the unattended validation lane for the M1
stale-example detection CI gate. The lane exercises the gate end to
end on every protected docs-pack row in
[`/artifacts/ci/m1_stale_example_source_map.yaml`](../../../artifacts/ci/m1_stale_example_source_map.yaml)
and writes a durable JSON capture under
`artifacts/milestones/m1/captures/`.

Reviewer entry point:
[`/docs/ci/m1_docs_pack_and_example_checks.md`](../../../docs/ci/m1_docs_pack_and_example_checks.md).

## Run the lane (full pass + drill replay)

```
python3 tests/ci/m1_docs_pack_and_example_checks_lane/run_m1_docs_pack_and_example_checks_lane.py --repo-root .
```

The wrapper performs two phases:

1. A clean pass over every protected docs-pack row. The validator
   writes
   `artifacts/milestones/m1/captures/docs_pack_and_example_checks_validation_capture.json`.
2. A `--force-drill` replay of every protected pack's named failure
   drill. Each drill writes its own
   `artifacts/milestones/m1/captures/docs_pack_and_example_checks_drill_capture_<pack_id>.json`.

The lane writes the aggregate capture to
`artifacts/milestones/m1/captures/docs_pack_and_example_checks_lane_capture.json`.

Exit codes:

- `0` — the full pass is clean *and* every named failure drill
  reproduces its declared `expected_check_id`.
- `1` — the full pass observed a drift, *or* at least one drill
  silently passed (meaning the gate has gone deaf to a real stale-
  example pattern).

## Replay a single drill manually

```
python3 tools/ci/check_stale_examples.py \
    --repo-root . \
    --force-drill docs_help_browser_skeleton:stale_example_drill.vocabulary_pin_dropped_from_seed
```

The runner injects a token that does not exist in the controlling
`truth_source.docs_help_source_class` seed row and MUST fail with
`stale_examples.vocabulary_pin_not_in_seed`.

## Captures

The lane and validator write to:

- `artifacts/milestones/m1/captures/docs_pack_and_example_checks_validation_capture.json`
  — durable full-pass capture (registered as `latest_capture` for the
  proof lane in
  [`/artifacts/milestones/m1/artifact_index.yaml`](../../../artifacts/milestones/m1/artifact_index.yaml)).
- `artifacts/milestones/m1/captures/docs_pack_and_example_checks_lane_capture.json`
  — durable lane capture (full pass + every drill replay summary).
- `artifacts/milestones/m1/captures/docs_pack_and_example_checks_drill_capture_<pack_id>.json`
  — one capture per protected pack's failure drill.
