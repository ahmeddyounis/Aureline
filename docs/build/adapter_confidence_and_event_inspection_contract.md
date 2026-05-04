# Build-adapter confidence ladder, parser-fallback reason card, and raw-event inspector contract

This packet freezes one shared confidence ladder, one shared
parser-fallback reason card, and one shared raw-event inspector record
before run, test, build, debug, output pane, problems pane, AI action
sheet, CI overlay, support-bundle export, and review pack paths
fragment into incompatible "trusted / partial / heuristic" labels and
incompatible raw-log inspectors. It names the seven confidence states
every consumer reads when answering "how trustworthy is this build
result?", the reason-card every consumer renders when the active state
is `parser_fallback` or `stale_result`, and the inspector record every
consumer opens when a user asks "show me the raw events behind this
result" without first scraping a free-form log.

If this packet, the
[`adapter_confidence_state.schema.json`](../../schemas/build/adapter_confidence_state.schema.json),
the
[`raw_event_inspector.schema.json`](../../schemas/build/raw_event_inspector.schema.json),
and the fixture corpus under
[`/fixtures/build/adapter_confidence_cases/`](../../fixtures/build/adapter_confidence_cases/)
disagree, the machine-readable schemas plus the frozen build-adapter,
target-graph, target-descriptor, run / attempt, task-event-envelope,
and execution-context vocabularies in
[`/schemas/tooling/adapter_descriptor.schema.json`](../../schemas/tooling/adapter_descriptor.schema.json),
[`/schemas/tooling/target_graph_snapshot.schema.json`](../../schemas/tooling/target_graph_snapshot.schema.json),
[`/schemas/tooling/target_descriptor.schema.json`](../../schemas/tooling/target_descriptor.schema.json),
[`/schemas/tooling/task_event_envelope.schema.json`](../../schemas/tooling/task_event_envelope.schema.json),
[`/schemas/execution/run.schema.json`](../../schemas/execution/run.schema.json),
[`/schemas/execution/attempt.schema.json`](../../schemas/execution/attempt.schema.json),
and
[`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
win and this packet plus the companion artifacts update in the same
change.

Companion artifacts:

- [`/schemas/build/adapter_confidence_state.schema.json`](../../schemas/build/adapter_confidence_state.schema.json)
  — boundary schema for `build_adapter_confidence_state_record` and
  `parser_fallback_reason_card_record`. Re-exports `confidence_class`
  from `schemas/tooling/task_event_envelope.schema.json` and the
  scope / target / freshness vocabularies from
  `schemas/tooling/target_graph_snapshot.schema.json`,
  `schemas/tooling/target_descriptor.schema.json`, and
  `schemas/runtime/execution_context.schema.json`.
- [`/schemas/build/raw_event_inspector.schema.json`](../../schemas/build/raw_event_inspector.schema.json)
  — boundary schema for `raw_event_inspector_record`. Cites the
  matching `build_adapter_confidence_state_record` and
  `parser_fallback_reason_card_record` so the confidence ladder, the
  reason card, and the inspector all share one originating run.
- [`/fixtures/build/adapter_confidence_cases/`](../../fixtures/build/adapter_confidence_cases/)
  — fixture corpus covering native-adapter success, parser fallback
  with partial target graph, imported CI log replay, and unknown-tool
  degraded inspection.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md`
  — run / test / build / debug picker, output pane, problems pane,
  AI action sheet, CI overlay, and support-bundle export posture;
  "build results must explain why they are trusted, partial, or
  heuristic" treated as an in-product contract.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — build-adapter boundary, parser-fallback handling, raw-event
  retention, and target-identity provenance.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — native runner, BSP client, BEP reader, structured importer,
  heuristic inferrer, parser-fallback, imported-result, and
  opaque-preservation architecture this packet projects.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — pickers, output pane headers, problems pane chips, AI action
  sheets, CI overlays, support-bundle exporters, and review packs
  that MUST consume the typed states rather than mint per-surface
  "trusted / partial / heuristic" labels.

If this document disagrees with those sources, those sources win and
this packet plus the companion artifacts update in the same change.

## Why freeze this now

Build intelligence becomes a credibility risk the moment a
parser-fallback row renders the same way as a native-adapter row, the
moment a stale baseline renders the same way as a fresh build, or the
moment a user has to scrape free-form log lines to know which fields
were observed and which were guessed. The execution model already
freezes "what ran" through `run_record` / `attempt_record` and the
adapter / target-graph contract already freezes "which adapter owns
this graph"; what is missing is a typed disclosure of "this result
was produced by a free-form parser, recognized N fields, guessed M
fields, and is missing K pieces of evidence". Without a frozen
contract, surfaces will mint per-feature labels — output pane writes
"approximate", picker writes "best effort", AI action sheet writes
"low confidence", CI overlay writes "(parsed)", and support exports
strip the disclosure entirely — and a reviewer will not be able to
tell across surfaces whether two rows describe the same loss.

## Confidence ladder (seven states)

The ladder is closed; consumers MUST NOT mint surface-local states.

| State                 | Confidence ceiling          | Evidence required                                                                                  | Renders reason card |
| --------------------- | --------------------------- | -------------------------------------------------------------------------------------------------- | ------------------- |
| `native_adapter`      | `authoritative_from_source` | At least one of native runner streamed events, BSP session, or BEP stream                          | No                  |
| `declared_adapter`    | `structured_parse_match`    | Declared structured importer / BSP / BEP / structured tool output stream                           | No                  |
| `inferred_adapter`    | `heuristic_best_effort`     | Manifest / lockfile observation or filename / directory convention match                           | No                  |
| `parser_fallback`     | `heuristic_best_effort`     | Free-form log line parse; native / BSP / BEP evidence sources MUST NOT appear                      | Yes (required)      |
| `imported_result`     | `structured_parse_match`    | Managed CI artifact import, support-bundle replay, provider API snapshot, review / release packet  | No                  |
| `stale_result`        | `degraded_partial`          | Prior trusted row with a typed stale reason (toolchain rev, lockfile, manifest, source change ...) | Yes (required)      |
| `unavailable_result`  | `unknown`                   | `no_evidence_observed` plus a typed unavailable reason                                              | No                  |

The state-to-ceiling and state-to-evidence-source binding is enforced
by `allOf` invariants on
[`/schemas/build/adapter_confidence_state.schema.json`](../../schemas/build/adapter_confidence_state.schema.json):

- `native_adapter` MUST cap at `authoritative_from_source`.
- `declared_adapter` MUST cap at `structured_parse_match`.
- `inferred_adapter` and `parser_fallback` MUST cap at
  `heuristic_best_effort`.
- `imported_result` MUST cite an `import_source_class` other than
  `not_imported` and MUST set freshness to `imported_read_only` or
  `support_bundle_replay_only`.
- `stale_result` MUST cite a non-`not_stale` `stale_reason_class` and
  MUST cap freshness at `stale_pending_refresh`.
- `unavailable_result` MUST cite `no_evidence_observed` and a
  non-`not_unavailable` `unavailable_reason_class`.
- `parser_fallback` MUST cite `free_form_log_line_parse` and MUST NOT
  cite `native_runner_streamed_events`,
  `build_server_protocol_session`, or
  `bazel_build_event_protocol_stream`.

The last invariant is the spec's "no fixture allows a parser fallback
result to masquerade as native adapter certainty" rule, encoded
directly in the schema.

## Parser-fallback reason card

Whenever the active state is `parser_fallback` or `stale_result`, the
record MUST cite a `parser_fallback_reason_card_record` that discloses:

1. **What was recognized** — a non-empty list of typed
   `recognized_artifact_class` entries (e.g.
   `compile_diagnostic`, `test_case_outcome`, `produced_artifact_path`)
   so a reviewer can read the picker chip and tell what the row knows.
2. **What was guessed** — a typed list of
   `guessed_artifact_class` entries (e.g.
   `build_target_kind_inferred_from_extension`,
   `test_outcome_inferred_from_keyword`) so a reviewer can decide
   whether to trust the row or escalate.
3. **What evidence was missing** — a non-empty list of
   `missing_evidence_class` entries (e.g.
   `no_structured_event_stream_observed`, `no_exit_code_observed`,
   `no_target_descriptor_attached`) so a reviewer can tell what
   would have to be inspected to fill the gap.
4. **Which support artifact can be opened** — a non-empty list of
   `openable_support_artifact_entry` rows. At least one entry MUST
   bind `open_action_class` =
   `open_raw_event_inspector_pane` so the raw-event inspector is
   always reachable; other entries MAY offer
   `open_originating_run_pane`, `open_target_graph_view`,
   `open_support_bundle_export`, etc.

The reason card is the contract behind the spec's "users do not need
to inspect raw logs unless they choose to" rule. Pickers, output pane
headers, problems pane chips, AI action sheets, and support exports
all render the reason card identically; raw-log inspection is a
choice, not a default.

## Raw-event inspector

The `raw_event_inspector_record` is the typed view a user opens when
they choose to inspect raw evidence. It does not embed raw bytes
inline; it cites:

- **Source-log segments** grouped by `source_log_class`
  (stdout / stderr / combined / structured streams, BSP messages,
  BEP stream, JUnit / TAP / SARIF artifacts, `cargo --message-format=json`,
  `go build -json`, MSBuild diagnostic logs, managed CI artifact logs,
  provider API payload captures, support-bundle replay captures,
  AI-emitted event captures, and the explicit
  `no_source_log_observed` segment for the unavailable path).
- **Parsed-event segments** grouped by `parsed_event_class` with
  per-class refs back to `task_event_envelope_record` ids.
- **Discarded-line segments** grouped by
  `discarded_line_reason_class` so silent discards are impossible
  (the `no_lines_discarded` reason MUST be cited explicitly when
  nothing was dropped).
- **Timing phases** keyed to `timing_phase_class` so per-phase
  duration is never re-derived from raw timestamps.
- **Target identity** keyed to `target_identity_authority_class` and
  `publishing_graph_id_ref` / `target_id_ref` so two surfaces
  resolve the same `(graph_id, target_id)` pair.
- **Provenance** back to `originating_run_ref`,
  `originating_attempt_ref`,
  `build_adapter_confidence_state_record_ref`, and (when present)
  `parser_fallback_reason_card_record_ref` so the inspector pane,
  the confidence chip, and the reason card all share one lineage.

`originating_run_ref` is the join key. The matching
`build_adapter_confidence_state_record` MUST cite the same
`originating_run_ref`; the `parser_fallback_reason_card_record` (when
present) MUST cite both. This is the spec's "raw-event inspection and
confidence ladders share one lineage model back to run and target
identity" rule.

## Lineage model

```
                run_record (schemas/execution/run.schema.json)
                         |
                         | originating_run_ref
                         |
            +------------+------------+
            |                         |
            v                         v
 build_adapter_confidence_state    raw_event_inspector
   _record (this schema)             _record (this schema)
            |                         |
            | parser_fallback_reason  | parser_fallback_reason_card_
            |   _card_ref             |   record_ref
            v                         v
       parser_fallback_reason_card_record (this schema)
                         |
                         | linked_build_adapter_confidence_state_
                         |   record_id_ref
                         v
        (back to confidence-state record above)
```

All three records resolve through the same `originating_run_ref` and
the same `target_id_ref` / `publishing_graph_id_ref`. Every fixture
in `/fixtures/build/adapter_confidence_cases/` exercises the join
path so a regression that breaks lineage is caught at fixture time.

## Surface contract

Every consumer surface MUST render confidence and reason data through
these three records:

- **Run / test / build / debug picker** renders a confidence chip
  pinned to `build_adapter_confidence_state` and a reason chip when
  the state is `parser_fallback` or `stale_result`.
- **Output pane header** renders the confidence chip plus a "what was
  recognized / guessed / missing" badge linking the reason card.
- **Problems pane chip** renders the confidence chip per build target
  row.
- **AI action sheet** disables structured-build tool calls when the
  state is `parser_fallback`, `stale_result`, or `unavailable_result`,
  citing the reason card.
- **CI overlay** renders the same chip set as the picker, including
  imported-result freshness (`imported_read_only` /
  `support_bundle_replay_only`).
- **Support-bundle exporter** preserves all three records and the
  evidence counts mirror across exporter and inspector so a review
  pack does not lose disclosure.
- **Review pack** renders a typed "explain this build result" packet
  citing the reason card and the inspector ref, never a raw-log
  pointer.

## Fixture corpus

[`/fixtures/build/adapter_confidence_cases/`](../../fixtures/build/adapter_confidence_cases/)
covers the four scenarios named in the spec:

- `native_adapter_success.yaml` — first-party native runner streamed
  events; ceiling `authoritative_from_source`; no reason card; the
  inspector cites a structured-event stream and zero discarded lines.
- `parser_fallback_partial_target_graph.yaml` — free-form log line
  parse over an unknown JVM build tool; ceiling
  `heuristic_best_effort`; reason card discloses two recognized
  fields, three guessed fields, three missing-evidence entries, and
  one openable raw-event inspector action.
- `imported_ci_log_replay.yaml` — managed CI artifact import;
  ceiling `structured_parse_match`; no reason card; freshness
  `imported_read_only`; the inspector groups events by imported
  outcome class and cites `imported_replay_no_phase` for timing.
- `unknown_tool_degraded_inspection.yaml` — `unavailable_result`
  with `no_adapter_attached`; ceiling `unknown`; no reason card;
  the inspector cites `no_source_log_observed` and a single
  `unknown_phase` timing entry so a "no result" row still carries
  typed counts.

Every fixture is a multi-document YAML file and validates under
JSON Schema Draft 2020-12 against the boundary schemas above.
