# Task-event replay fixtures

These fixtures are short, reviewable scenarios that anchor the shared
task-event envelope vocabulary frozen in
[`/schemas/tooling/task_event_envelope.schema.json`](../../../schemas/tooling/task_event_envelope.schema.json),
the adapter register frozen in
[`/artifacts/tooling/adapter_map.yaml`](../../../artifacts/tooling/adapter_map.yaml),
and the contract seed in
[`/docs/tooling/task_event_contract_seed.md`](../../../docs/tooling/task_event_contract_seed.md).

Each fixture is one `task_event_envelope_record` or
`task_event_replay_bundle_record` rendered as a worked scenario. The
set exists so reviewers can compare a first-party authoritative
emission, a structured-parse emission, a heuristic emission, and an
unsupported-source opaque preservation emission side by side — and so
support-bundle, diagnostics-view, benchmark, and automation consumers
can anchor their replay contracts to one shared record shape.

## Scope rules

- Fixtures validate against the shared envelope schema. They carry
  `task_event_envelope_schema_version: 1`.
- Fixtures MUST NOT encode raw stdout bytes, raw stderr bytes, raw
  compiler output, raw BSP payloads, raw BEP JSONL, raw JUnit XML,
  raw SARIF bodies, raw env bodies, raw command lines, raw paths,
  raw URLs, or raw secret values. Only class labels, frozen tokens,
  opaque ids, hashes, counts, and short reviewer-facing summary
  sentences are admissible.
- Opaque ids and monotonic timestamps are chosen for review clarity
  rather than to mirror a real machine.
- `summary` and `notes` strings are short reviewer-facing sentences.
  They name tokens and counts; they do not paste tool output.
- Milestone slugs (for example `M0`, `M00`, `M00-190`, `WP-01`) MUST
  NOT appear in any fixture field.

## Index

| Fixture | Source kind | Confidence | Key coverage |
|---|---|---|---|
| [`native_cargo_test_case_outcome.json`](./native_cargo_test_case_outcome.json) | `native_task_runner` | `authoritative_from_source` | test-case outcome; retained_local_with_replay_opt_in; in-process native adapter |
| [`bsp_build_target_lifecycle.json`](./bsp_build_target_lifecycle.json) | `build_server_protocol_client` | `authoritative_from_source` | build-target completion; out-of-process sidecar adapter; local cache hit |
| [`bazel_bep_artifact_publication.json`](./bazel_bep_artifact_publication.json) | `bazel_build_event_protocol` | `authoritative_from_source` | artifact publication; BEP ingest; retained_local_with_replay_opt_in |
| [`junit_structured_test_suite.json`](./junit_structured_test_suite.json) | `structured_tool_output_ingest` | `structured_parse_match` | test-suite completion; JUnit XML ingest reports structured_parse_match (never authoritative) |
| [`heuristic_parser_diagnostic.json`](./heuristic_parser_diagnostic.json) | `heuristic_parser` | `heuristic_best_effort` | diagnostic from line-oriented parser; secret_pattern_scan + path_redaction_pass applied; adapter_confidence_downgrade at the end |
| [`unsupported_source_preserved.json`](./unsupported_source_preserved.json) | `heuristic_parser` / `preserve_opaque_with_provenance` | `unknown` | unsupported_source_placeholder + opaque_preservation_only; review gap kept reviewable |
| [`replay_bundle_support_export.json`](./replay_bundle_support_export.json) | n/a (bundle record) | n/a | replay-bundle record indexing the six envelope fixtures above for support-export |

## Coverage contract

The fixture set MUST keep:

- at least one envelope whose `source_kind` is `native_task_runner`
  and whose `confidence_class` is `authoritative_from_source`;
- at least one envelope whose `source_kind` is
  `build_server_protocol_client` and whose adapter runs
  `out_of_process_sidecar`;
- at least one envelope whose `source_kind` is
  `bazel_build_event_protocol` and whose `payload_kind` is
  `artifact_publication`;
- at least one envelope whose `source_kind` is
  `structured_tool_output_ingest` and whose `confidence_class` is
  `structured_parse_match` (never `authoritative_from_source`);
- at least one envelope whose `source_kind` is `heuristic_parser`
  and whose `confidence_class` is at most `heuristic_best_effort`
  (with `adapter_confidence_downgrade` in normalization_passes);
- at least one envelope whose `event_kind` is
  `unsupported_source_placeholder`, `payload_kind` is
  `opaque_preservation_only`, and `confidence_class` is `unknown`;
- at least one `task_event_replay_bundle_record` indexing at least
  two distinct adapters and citing the adapter-map row-set ref.

Removing a layer of coverage is a breaking change.
