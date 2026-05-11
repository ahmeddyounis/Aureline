# Proof packet: M1 docs-pack and example freshness CI gate

Purpose: anchor proof captures for the unattended M1 lane that
validates the stale-example detection pipeline against:

- the maintained source map under
  [`/artifacts/ci/m1_stale_example_source_map.yaml`](../../../artifacts/ci/m1_stale_example_source_map.yaml);
- the upstream M1 truth-source seed at
  [`/artifacts/help/m1_truth_source_examples.yaml`](../../../artifacts/help/m1_truth_source_examples.yaml)
  the protected examples pin tokens against (so a token the upstream
  seed retires fails the gate on every example that still pins it);
- the protected docs-pack page at
  [`/docs/help/docs_browser_contract.md`](../../../docs/help/docs_browser_contract.md)
  whose required source / version / freshness metadata tokens the
  gate asserts every dogfood publication keeps publishing; and
- the protected example payloads under
  [`/fixtures/help/docs_browser_cases/`](../../../fixtures/help/docs_browser_cases/)
  that the gate parses, validates against
  `schemas/ux/embedded_boundary_card.schema.json`, and re-pins to the
  controlling vocabulary seed rows.

Reviewer entry point:
[`/docs/ci/m1_docs_pack_and_example_checks.md`](../../../docs/ci/m1_docs_pack_and_example_checks.md).

## Canonical sources

- [`/artifacts/ci/m1_stale_example_source_map.yaml`](../../../artifacts/ci/m1_stale_example_source_map.yaml)
  — the source map. Adding a new protected pack or vocabulary seed is
  additive-minor and bumps `schema_version`; renaming an existing
  `source_map_id`, `pack_id`, or `vocabulary_id` is breaking.
- [`/tools/ci/check_stale_examples.py`](../../../tools/ci/check_stale_examples.py)
  — validator entry point.
- [`/ci/docs/stale_example_gate.yml`](../../../ci/docs/stale_example_gate.yml)
  — CI gate manifest (owner, severity, protected paths, commands,
  rollback lever, refresh policy).
- [`/tests/ci/m1_docs_pack_and_example_checks_lane/run_m1_docs_pack_and_example_checks_lane.py`](../../../tests/ci/m1_docs_pack_and_example_checks_lane/run_m1_docs_pack_and_example_checks_lane.py)
  — unattended validation lane that runs the gate end-to-end and
  replays the named failure drill.

## Named runtime consumers

- [`/docs/ci/m1_docs_pack_and_example_checks.md`](../../../docs/ci/m1_docs_pack_and_example_checks.md)
  — reviewer-facing landing page. Quotes the source map's protected
  pack id, required metadata tokens, controlling vocabulary seeds, and
  failure-drill replay command verbatim.
- [`/ci/docs/stale_example_gate.yml`](../../../ci/docs/stale_example_gate.yml)
  — CI gate manifest consumed by the gate runner.

## Upstream truth sources the gate projects from

- [`/docs/help/docs_browser_contract.md`](../../../docs/help/docs_browser_contract.md)
  — docs/help browser skeleton truth rows.
- [`/artifacts/help/m1_truth_source_examples.yaml`](../../../artifacts/help/m1_truth_source_examples.yaml)
  — canonical badge-vocabulary seed for `docs_help_source_class`,
  `docs_help_version_match_state`, and `docs_help_freshness_class`.
- [`/docs/help/truth_source_model.md`](../../../docs/help/truth_source_model.md)
  — reviewer landing page for that seed.

## Live runtime consumers (read-only)

- [`/artifacts/build/build_identity.json`](../../../artifacts/build/build_identity.json)
  — exact-build identity that the capture embeds for cross-artifact
  traceability.

## Validation captures

- [`/artifacts/milestones/m1/captures/docs_pack_and_example_checks_validation_capture.json`](../captures/docs_pack_and_example_checks_validation_capture.json)
  — full-pass capture written by the validator.
- [`/artifacts/milestones/m1/captures/docs_pack_and_example_checks_lane_capture.json`](../captures/docs_pack_and_example_checks_lane_capture.json)
  — aggregate lane capture (full pass + every drill replay summary).
- [`/artifacts/milestones/m1/captures/docs_pack_and_example_checks_drill_capture_docs_help_browser_skeleton.json`](../captures/docs_pack_and_example_checks_drill_capture_docs_help_browser_skeleton.json)
  — drill capture for the seeded protected pack.

## Refresh policy

Re-run the validation lane after a change to:

- the source map (`artifacts/ci/m1_stale_example_source_map.yaml`);
- the validator (`tools/ci/check_stale_examples.py`);
- the reviewer landing page
  (`docs/ci/m1_docs_pack_and_example_checks.md`);
- the gate manifest (`ci/docs/stale_example_gate.yml`);
- the upstream truth-source seed
  (`artifacts/help/m1_truth_source_examples.yaml`);
- any protected docs-pack page or example fixture listed in the
  source map.

## Closure rule

The lane stays open until the latest capture lands under the governed
proof root and reports PASS for:

- envelope sanity
  (`stale_examples.source_map_schema_version_wrong`,
  `stale_examples.source_map_id_wrong`,
  `stale_examples.overview_page_missing`,
  `stale_examples.validator_entrypoint_missing`,
  `stale_examples.ci_gate_manifest_missing`);
- vocabulary-seed resolution
  (`stale_examples.vocabulary_seed_missing`,
  `stale_examples.vocabulary_seed_unparseable`,
  `stale_examples.vocabulary_seed_rows_missing`,
  `stale_examples.vocabulary_seed_row_not_found`,
  `stale_examples.vocabulary_seed_tokens_missing`);
- docs-pack metadata presence
  (`stale_examples.docs_pack_missing`,
  `stale_examples.required_metadata_tokens_empty`,
  `stale_examples.required_metadata_tokens_missing`);
- example payload sanity
  (`stale_examples.payload_missing`,
  `stale_examples.payload_parse_failed`,
  `stale_examples.example_payload_schema_invalid`);
- vocabulary-pin freshness
  (`stale_examples.vocabulary_pin_id_missing`,
  `stale_examples.vocabulary_pin_path_missing`,
  `stale_examples.vocabulary_pin_unknown_id`,
  `stale_examples.vocabulary_pin_path_unresolved`,
  `stale_examples.vocabulary_pin_value_not_string`,
  `stale_examples.vocabulary_pin_not_in_seed`);
- and the seeded protected pack's named failure drill —

and the seeded protected pack `docs_help_browser_skeleton` is observed
in `protected_pack_ids` so the M1 dogfood publication has at least
one protected docs-pack guard before it ships.

## Failure-drill coverage

| Pack | Drill | Expected check id |
|---|---|---|
| `docs_help_browser_skeleton` | `stale_example_drill.vocabulary_pin_dropped_from_seed` | `stale_examples.vocabulary_pin_not_in_seed` |

Run the drill with:

```
python3 tools/ci/check_stale_examples.py \
    --repo-root . \
    --force-drill docs_help_browser_skeleton:stale_example_drill.vocabulary_pin_dropped_from_seed
```
