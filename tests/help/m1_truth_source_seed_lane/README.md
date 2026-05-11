# M1 docs/help/About/service-health truth-source seed validation lane

This directory hosts the unattended Python runner that validates the
canonical badge-vocabulary seed at
[`/artifacts/help/m1_truth_source_examples.yaml`](../../../artifacts/help/m1_truth_source_examples.yaml)
against
[`/schemas/help/provenance_badge_vocabulary.schema.json`](../../../schemas/help/provenance_badge_vocabulary.schema.json).

The lane is part of the public-truth seed proof set under
[`/docs/help/truth_source_model.md`](../../../docs/help/truth_source_model.md).

## Run the lane (full pass)

```
python3 tests/help/m1_truth_source_seed_lane/run_m1_truth_source_seed_lane.py --repo-root .
```

Exit codes:

- `0` — every row passes, no errors.
- `1` — one or more rows failed schema, vocabulary, surface-coverage,
  honesty-fallback, seed-placeholder, named-consumer, or example-payload
  invariants.
- `2` — a forced drill (`--force-drill`) did **not** reproduce the
  named `expected_check_id`.

## Replay a named failure drill

```
python3 tests/help/m1_truth_source_seed_lane/run_m1_truth_source_seed_lane.py \
    --repo-root . \
    --force-drill <row_id>:<drill_id>
```

The runner applies the row's declared `forced_input`, re-validates the
single targeted row, and exits 0 only when the row's
`expected_check_id` is reproduced. Unforced rows are still validated
in full so a passing forced-drill run cannot hide drift on a sibling.

The drills the seed currently carries:

| Row | Drill | Expected check id |
|---|---|---|
| `truth_source.docs_help_source_class` | `truth_source_drill.honesty_fallback_token_dropped` | `truth_source.honesty_fallback_token_missing` |
| `truth_source.docs_help_version_match_state` | `truth_source_drill.version_match_token_count_drifted` | `truth_source.vocabulary_token_count_mismatch` |
| `truth_source.docs_help_freshness_class` | `truth_source_drill.freshness_degraded_pair_widened` | `truth_source.degraded_state_token_widened` |
| `truth_source.client_scope_badge_family` | `truth_source_drill.required_consuming_surface_help_pane_dropped` | `truth_source.required_consuming_surface_missing` |
| `truth_source.install_mode_class` | `truth_source_drill.install_mode_unknown_token_dropped` | `truth_source.honesty_fallback_token_not_in_vocabulary` |
| `truth_source.provenance_row_state` | `truth_source_drill.provenance_seed_placeholder_dropped` | `truth_source.seed_placeholder_role_widened` |
| `truth_source.service_health_state` | `truth_source_drill.service_health_seed_placeholder_dropped` | `truth_source.seed_placeholder_token_required` |

## Capture

The runner writes a durable JSON capture to
`artifacts/milestones/m1/captures/truth_source_seed_validation_capture.json`
on every run. The capture is registered as the lane's `latest_capture`
in
[`/artifacts/milestones/m1/artifact_index.yaml`](../../../artifacts/milestones/m1/artifact_index.yaml).
