# Task-event envelope, adapter map, and raw-payload retention contract

This packet freezes one shared task-event envelope, one adapter-map
register, and one raw-payload retention posture before the build,
test, debug, notebook, and AI-tool orchestration paths fragment into
incompatible event vocabularies across UI, CLI, support, replay,
benchmark, and automation surfaces. It names the canonical envelope
record every producer (native task runner, Build Server Protocol
client, Bazel Build Event Protocol reader, structured tool-output
ingest, heuristic parser, debugger adapter, notebook kernel, AI
broker, user-reported manual entry, support-bundle replay) emits and
every consumer reads, the adapter register every ingest surface
resolves through, and the raw-payload retention block every
first-class consumer trusts instead of re-scraping tool output.

If this packet, the
[`task_event_envelope.schema.json`](../../schemas/tooling/task_event_envelope.schema.json)
boundary, the adapter register at
[`/artifacts/tooling/adapter_map.yaml`](../../artifacts/tooling/adapter_map.yaml),
and the replay corpus under
[`/fixtures/tooling/task_event_replay/`](../../fixtures/tooling/task_event_replay/)
disagree, the machine-readable schema and the frozen execution-context
vocabulary in
[`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
win for tooling and this packet must update in the same change.

Companion artifacts:

- [`/schemas/tooling/task_event_envelope.schema.json`](../../schemas/tooling/task_event_envelope.schema.json)
  — boundary schema for the `task_event_envelope_record` and the
  `task_event_replay_bundle_record`. Re-exports the frozen
  execution-context vocabulary (scope_class) from
  `schemas/runtime/execution_context.schema.json` rather than minting
  new target, toolchain, or scope classes.
- [`/artifacts/tooling/adapter_map.yaml`](../../artifacts/tooling/adapter_map.yaml)
  — canonical adapter register binding every source kind (native
  task runner, BSP, BEP, structured tool output, heuristic parser,
  unsupported-source preservation) to an adapter id, a capability
  set, a confidence ceiling, a normalization-pass ordering, an
  unsupported-source posture, and a raw-payload retention posture.
- [`/fixtures/tooling/task_event_replay/`](../../fixtures/tooling/task_event_replay/)
  — replay corpus covering a native cargo test-case outcome, a BSP
  build-target lifecycle, a Bazel BEP artifact-publication event, a
  JUnit structured test suite, a heuristic-parser diagnostic, an
  unsupported-source opaque preservation event, and a support-export
  replay bundle.
- [`/artifacts/governance/stable_surface_inventory.yaml#tooling.task_event_envelope`](../../artifacts/governance/stable_surface_inventory.yaml)
  — surface-contract packet row. The task-event envelope is an
  experimental surface governed through the same inventory row as
  command descriptors, settings, and portable profiles.
- [`/artifacts/governance/schema_families.yaml#family_id:tooling`](../../artifacts/governance/schema_families.yaml)
  — schema-family registration for `schemas/tooling/`.
- [`/docs/execution/context_inspector_packet.md`](../execution/context_inspector_packet.md)
  — execution-context snapshot and provenance-diff packet; every
  task-event envelope cites an `execution_context_id` that resolves
  to the snapshot contract there.
- [`/docs/automation/cli_surface_contract.md`](../automation/cli_surface_contract.md)
  — CLI / headless JSON-surface contract; stable CLI output that
  reports task, test, or build events consumes envelopes through
  this packet rather than scraping human stdout.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md`
  — build, test, debug, notebook, and support-export orchestration
  posture; "raw payload retained so support can reparse" treated as
  an in-product contract rather than an afterthought.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — task / test / debug event orchestration, adapter boundary,
  confidence posture, and execution-context re-export rules this
  packet projects.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — task-runner, BSP client, Bazel BEP reader, structured-output
  ingest, heuristic-parser, and debugger-adapter architecture the
  adapter map maps onto.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — task panes, test-result views, diagnostics panels, and build /
  run progress surfaces that MUST consume envelopes rather than
  scraping tool output.
- `.t2/docs/Aureline_Milestones_Document.md`
  — shared task-event envelope named as a release-blocking posture
  during the foundations phase.

If this document disagrees with those sources, those sources win and
this packet plus the companion artifacts update in the same change.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: tooling_packet
packet_id: tooling.task_event_envelope.seed
evidence_id: evidence.tooling.task_event_envelope.packet
title: Task-event envelope, adapter map, and raw-payload retention contract
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  claim_row_refs:
    - packet_row:task_event_envelope.shape
    - packet_row:task_event_envelope.source_kind_honesty
    - packet_row:task_event_envelope.confidence_honesty
    - packet_row:task_event_envelope.raw_payload_retention
    - packet_row:task_event_envelope.adapter_capability_negotiation
    - packet_row:task_event_envelope.unsupported_source_posture
    - packet_row:task_event_envelope.execution_context_linkage
    - packet_row:task_event_envelope.replay_bundle_contract
    - packet_row:task_event_envelope.cross_surface_parity
  covered_lanes:
    - release_evidence
    - support_export
    - governance_packets
    - docs_public_truth
    - automation_and_cli
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-04-24T00:00:00Z
  stale_after: P30D
  freshness_class: warm_cached
  source_revision: task_event_envelope_seed@1
  trigger_revision: task_event_envelope_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen execution-context, authority-class,
    redaction-default, and record-class-registry vocabularies. No
    live native task runner, BSP client, BEP reader, or heuristic
    parser is wired to this packet yet. Claims are structural: every
    envelope, adapter row, and replay fixture in the artifact set
    reuses the existing frozen tokens rather than minting new per-
    surface language.
artifact_links:
  supporting_evidence_ids:
    - evidence.tooling.task_event_envelope_schema
    - evidence.tooling.task_event_adapter_map_seed
    - evidence.tooling.task_event_replay_corpus
    - evidence.execution.context_snapshot_schema
  fixture_refs:
    - fixtures/tooling/task_event_replay/
  source_anchor_refs:
    - schemas/tooling/task_event_envelope.schema.json
    - artifacts/tooling/adapter_map.yaml
    - schemas/runtime/execution_context.schema.json
    - schemas/execution/context_snapshot.schema.json
    - artifacts/governance/stable_surface_inventory.yaml
    - artifacts/governance/schema_families.yaml
    - artifacts/governance/control_artifact_index.yaml
```

## Summary

This seed packet freezes:

- one `task_event_envelope_record` shape every task, test, build,
  diagnostic, artifact-publication, log-line, progress-tick,
  resource-usage, and debugger producer emits when it crosses the
  UI, CLI, support-export, replay, benchmark, AI-broker, or
  automation boundary;
- one `task_event_replay_bundle_record` shape every support bundle,
  diagnostics view, and automation consumer reads when it retains a
  set of envelopes for later reparse or review without flattening
  events into summary strings;
- one adapter-map register binding every native, protocol, and
  heuristic adapter to a stable `adapter_id`, a capability set, an
  adapter-confidence ceiling, an ordered normalization-pass list, an
  unsupported-source posture, and a default raw-payload retention
  posture;
- one raw-payload retention block that records retention class,
  opaque retention id, reported bytes, content hash, media class,
  redaction class, and truncation note so raw bodies stay retained
  by reference rather than inlined;
- one execution-context linkage rule: every envelope cites an
  `execution_context_id` that resolves to
  `schemas/runtime/execution_context.schema.json` and optionally an
  `execution_context_snapshot_ref` resolving to
  `schemas/execution/context_snapshot.schema.json`, so task, test,
  and debug-prep flows do not re-mint toolchain, target, or scope
  vocabulary;
- one seed replay corpus covering native-runner, BSP, BEP,
  structured-output, heuristic-parser, unsupported-source, and
  support-export replay-bundle cases.

It does not claim a live native runner, BSP client, BEP reader, or
adapter plugin is wired up. It claims only that one inspectable
task-event model exists in one reviewable form and reuses the frozen
runtime, execution-context, and record-class vocabularies already
landed in this repository.

## Claim coverage

| Packet row | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|
| `packet_row:task_event_envelope.shape` | `seed_only` | `internal` | `evidence.tooling.task_event_envelope_schema` | Freezes one machine-readable envelope record every producer emits; `allOf` gates tie `payload_kind` to the correct typed-payload shape. |
| `packet_row:task_event_envelope.source_kind_honesty` | `seed_only` | `internal` | `evidence.tooling.task_event_adapter_map_seed` | Freezes the closed `source_kind` vocabulary; heuristic, structured, and authoritative sources never hide behind each other. |
| `packet_row:task_event_envelope.confidence_honesty` | `seed_only` | `internal` | `evidence.tooling.task_event_envelope_schema` | `authoritative_from_source` is reserved for first-party sources; heuristic parsers MUST NOT claim it; structured ingest MAY at most claim `structured_parse_match`; adapter confidence ceilings are re-enforced in the schema. |
| `packet_row:task_event_envelope.raw_payload_retention` | `seed_only` | `internal` | `evidence.tooling.task_event_envelope_schema` | Freezes a retention block with opaque `raw_payload_ref`, reported bytes, content hash, media class, redaction class, and truncation note; raw bytes never cross this boundary. |
| `packet_row:task_event_envelope.adapter_capability_negotiation` | `seed_only` | `internal` | `evidence.tooling.task_event_adapter_map_seed` | Adapter map freezes capabilities per source and per adapter so consumers treat missing events as `not_observed_by_adapter` rather than `not_happened_in_task`. |
| `packet_row:task_event_envelope.unsupported_source_posture` | `seed_only` | `internal` | `evidence.tooling.task_event_replay_corpus` | Five unsupported-source postures enumerated; `preserve_opaque_with_provenance` keeps a reviewable row even when no adapter understands the source. |
| `packet_row:task_event_envelope.execution_context_linkage` | `seed_only` | `internal` | `evidence.execution.context_snapshot_schema` | Every envelope cites one `execution_context_id`; task, test, and debug-prep flows share the execution-context record rather than re-mint it. |
| `packet_row:task_event_envelope.replay_bundle_contract` | `seed_only` | `internal` | `evidence.tooling.task_event_replay_corpus` | Replay bundles carry trace-id sets, context-id sets, retention counts, and a redaction class so support export / benchmark / automation read one contract rather than bespoke views. |
| `packet_row:task_event_envelope.cross_surface_parity` | `seed_only` | `internal` | `evidence.tooling.task_event_envelope_schema` | UI task panes, CLI stable JSON, support bundles, replay harness, benchmark reports, and AI broker all point at the same envelope record rather than parallel event families. |

## What this seed freezes

### Envelope record

Every task-event envelope carries these required fields (see
[`task_event_envelope.schema.json`](../../schemas/tooling/task_event_envelope.schema.json)
for the authoritative boundary):

- `record_kind` (`task_event_envelope_record`)
- `task_event_envelope_schema_version`
- `event_id` — opaque unique-within-one-context id
- `trace_id` — opaque correlation id shared across envelopes for the
  same task / build / test run
- `parent_event_id` — optional, for nested lifecycle
- `captured_at` — when the producer observed the event
- `ingested_at` — when the adapter emitted the envelope
- `event_kind` — one of the closed event-kind vocabulary (see below)
- `payload_kind` — one of the closed payload-kind vocabulary
- `source_kind` — one of `native_task_runner`,
  `build_server_protocol_client`, `bazel_build_event_protocol`,
  `structured_tool_output_ingest`, `heuristic_parser`,
  `debugger_adapter_emitted`, `notebook_kernel_emitted`,
  `ai_emitted_event`, `user_reported_manual`,
  `replayed_from_support_bundle`
- `confidence_class` — `authoritative_from_source`,
  `structured_parse_match`, `heuristic_best_effort`,
  `degraded_partial`, or `unknown`
- `workspace_or_target_identity` — workspace id, target ref,
  optional build-target id, optional `scope_class` pass-through
- `execution_context_id` — points at the execution-context record
- `raw_payload_retention` — retention class, opaque retention ref,
  reported bytes, content hash, media class, redaction class,
  truncation note
- `provenance` — producer tool, producer version, adapter id,
  adapter capabilities asserted, adapter confidence ceiling, ingest
  host class, ordered normalization-pass list, unsupported-source
  posture, optional bridge-adapter refs
- `typed_payload` — concrete shape matches `payload_kind`

Optional fields: `execution_context_snapshot_ref`,
`approval_ticket_ref`, `notes`.

### Event-kind vocabulary

The schema's `event_kind` is a closed enum that names every build,
test, diagnostic, artifact-publication, log, progress, resource, and
debugger event class the envelope can carry. The full list is in
[`task_event_envelope.schema.json#/$defs/event_kind`](../../schemas/tooling/task_event_envelope.schema.json);
the short form:

- task lifecycle: `task_started`, `task_progress_tick`,
  `task_completed`, `task_cancelled`, `task_failed`,
  `task_retry_scheduled`;
- test lifecycle: `test_suite_started`, `test_case_started`,
  `test_case_completed`, `test_case_skipped`, `test_case_flaky`,
  `test_suite_completed`;
- build lifecycle: `compile_unit_started`, `compile_unit_completed`,
  `build_target_started`, `build_target_completed`,
  `build_target_cached`;
- artifact lifecycle: `artifact_published`, `artifact_withdrawn`;
- supporting: `diagnostic_reported`, `log_line_emitted`,
  `resource_usage_sample`, `debugger_breakpoint_hit`,
  `debugger_session_ended`;
- honest gap marker: `unsupported_source_placeholder` —
  reserved for envelopes that preserve a source whose adapter has
  not yet been seeded. MUST carry `confidence_class = unknown` and
  `payload_kind = opaque_preservation_only`.

Adding a new kind is additive-minor; repurposing an existing kind is
breaking.

### Confidence posture

A consumer that reads an envelope learns the confidence posture of
the emission from three fields acting together:

1. `source_kind` — who produced it;
2. `confidence_class` — the envelope-level confidence;
3. `provenance.adapter_confidence_ceiling` — the maximum confidence
   the adapter is allowed to assert on any envelope it emits.

The schema enforces three invariants:

- `confidence_class = authoritative_from_source` is reserved for
  `native_task_runner`, `build_server_protocol_client`,
  `bazel_build_event_protocol`, `debugger_adapter_emitted`, and
  `notebook_kernel_emitted` source kinds.
- `source_kind = heuristic_parser` MUST set `confidence_class` to
  one of `heuristic_best_effort`, `degraded_partial`, or `unknown`.
- `source_kind = structured_tool_output_ingest` MAY claim
  `structured_parse_match` but MUST NOT claim
  `authoritative_from_source`.

Adapters downgrade confidence on truncation, missing fields, version
drift, or policy-limited decode; consumers read
`confidence_class` rather than infer confidence from the source
kind alone.

### Raw-payload retention

Raw stdout, raw stderr, raw compiler output, raw BSP payloads, raw
BEP JSONL, raw JUnit XML, raw SARIF, raw TAP, and raw heuristic text
never cross the envelope boundary. Every envelope records a
retention block:

- `retention_class` — one of `not_retained_ephemeral`,
  `retained_local_support_bundle_only`,
  `retained_local_with_replay_opt_in`,
  `retained_managed_with_redaction`,
  `retained_managed_with_broadened_capture`;
- `raw_payload_ref` — opaque retention id. MUST be non-null on every
  `retained_*` class; MUST be null on `not_retained_ephemeral`;
- `raw_payload_bytes_reported` — byte count at capture;
- `raw_payload_hash` — hash-algorithm-prefixed digest;
- `raw_payload_media_class` — one of the closed media-class
  vocabulary (JSONL, JSON document, XML document, TAP stream, SARIF
  document, cargo JSON message stream, BEP stream, BSP JSON-RPC
  stream, ANSI-colored stream, binary blob, mixed text and binary,
  plain UTF-8 text, or `not_applicable`);
- `redaction_class` — `metadata_and_hashes_only` (default),
  `structured_fields_with_path_redaction`,
  `structured_fields_with_broadened_capture`, or `none`;
- `truncation_note` — short sentence naming why a truncation
  happened.

Envelopes whose `retention_class` is
`retained_managed_with_broadened_capture` or whose `redaction_class`
is `structured_fields_with_broadened_capture` or `none` MUST carry
an `approval_ticket_ref`. Raw bytes remain retained by the producer;
support, replay, and benchmark surfaces resolve the
`raw_payload_ref` through the retention surface and never expect
bytes inline.

### Execution-context linkage

Every envelope cites one `execution_context_id` that resolves to the
canonical record frozen in
[`execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json).
Consumers that need to reason about toolchain identity, target
identity, environment capsule, scope, trust state, identity mode,
policy epoch, or activator decision read the execution-context
record rather than pulling duplicate fields out of the envelope.
Envelope `workspace_or_target_identity.scope_class` is a pass-through
of the execution-context `scope_class`; no new scope vocabulary is
minted here. When a consumer wants to compare two envelopes across
runs it reads the optional `execution_context_snapshot_ref`
pointing at the snapshot contract frozen in
[`context_snapshot.schema.json`](../../schemas/execution/context_snapshot.schema.json).

### Adapter capability negotiation

The adapter map at
[`artifacts/tooling/adapter_map.yaml`](../../artifacts/tooling/adapter_map.yaml)
is the canonical register; every adapter row names:

- `adapter_id` (stable across tool versions);
- `source_kind`;
- `producer_tool` and `producer_tool_version_policy`;
- `ingest_host_class`
  (`in_process_native`, `in_process_adapter_plugin`,
  `out_of_process_sidecar`, `remote_agent`,
  `support_bundle_replay`, `automation_replay_harness`);
- `adapter_confidence_ceiling`;
- `unsupported_source_posture`;
- `raw_payload_retention_posture` (the retention class the adapter
  defaults to; individual envelopes MAY narrow further);
- `capabilities` (subset of the closed capability vocabulary);
- `payload_kinds_supported`;
- `normalization_passes` (ordered; first is `raw_capture_only`, last
  is `adapter_confidence_downgrade` or
  `adapter_confidence_preserved`);
- `fixture_refs` (at least one fixture under
  `fixtures/tooling/task_event_replay/`).

A consumer that asks an adapter for a capability the adapter does
not offer MUST treat the absent events as
`not_observed_by_adapter`, never `not_happened_in_task`. When two
adapters observe the same source under
`coexist_side_by_side_with_native`, they emit envelopes sharing one
`trace_id` with distinct `event_id`s so support and replay can
compare the authoritative emission against the bridged one.

### Unsupported-source posture

Five postures are enumerated:

- `reject_with_explanation` — adapter declines to emit; the
  opaque-preservation adapter (`aureline.unsupported.opaque_preservation`)
  MAY pick up the slack.
- `accept_as_heuristic_only` — adapter emits as
  `heuristic_parser` / `heuristic_best_effort` regardless of how
  structured the raw source appeared.
- `accept_with_degraded_confidence` — adapter preserves the
  `source_kind` but marks `confidence_class = degraded_partial`.
- `coexist_side_by_side_with_native` — adapter emits alongside a
  first-party native adapter under one `trace_id`.
- `preserve_opaque_with_provenance` — adapter emits an
  `unsupported_source_placeholder` envelope carrying only
  provenance and `raw_payload_retention`;
  `confidence_class = unknown`, `payload_kind = opaque_preservation_only`.

The opaque-preservation posture exists so unsupported sources stay
reviewable. Automation consumers detect the gap through the
placeholder rather than silently treating the source as empty.

### Replay bundle

A `task_event_replay_bundle_record` names one set of envelopes
emitted during one task, build, test, or debug run. It carries:

- `bundle_id`;
- `captured_at`;
- `trace_ids` (non-empty);
- `execution_context_ids` (non-empty);
- `envelope_count`;
- `adapter_map_ref` (points at the adapter-map row-set the bundle
  was emitted under);
- `redaction_class` and optional `approval_ticket_ref`;
- `raw_payload_retention_summary` (counts of retained-local,
  retained-managed, and not-retained envelopes).

Support bundles, diagnostics views, benchmark harnesses, replay
probes, and automation consumers cite a bundle and iterate
envelopes through it rather than flattening to summary strings.

## Rules (normative)

1. Every build, test, diagnostic, or artifact-publication event that
   crosses a UI, CLI, support-export, replay, benchmark, AI-broker,
   or automation boundary MUST ride a `task_event_envelope_record`.
   Parallel event families on any consumer surface are
   non-conforming.
2. Every envelope MUST cite an `execution_context_id`, a
   `source_kind`, a `confidence_class`, and a `raw_payload_retention`
   block. Silent omission of any of these is non-conforming.
3. Raw stdout, raw stderr, raw payload bytes, raw path bytes, raw
   secret values, and raw env bodies MUST NOT appear inline in any
   envelope. Consumers that want raw bytes resolve
   `raw_payload_ref` through the producer's retention surface under
   the envelope's `retention_class`.
4. `confidence_class = authoritative_from_source` MUST be reserved
   for `native_task_runner`, `build_server_protocol_client`,
   `bazel_build_event_protocol`, `debugger_adapter_emitted`, and
   `notebook_kernel_emitted` source kinds.
5. `source_kind = heuristic_parser` MUST NOT claim
   `authoritative_from_source` or `structured_parse_match`.
6. `source_kind = structured_tool_output_ingest` MAY claim
   `structured_parse_match` but MUST NOT claim
   `authoritative_from_source`.
7. `retention_class` values other than `not_retained_ephemeral` MUST
   carry a non-null `raw_payload_ref`. `not_retained_ephemeral` MUST
   carry a null `raw_payload_ref` and a null `raw_payload_hash`.
8. `retention_class = retained_managed_with_broadened_capture` and
   `redaction_class` in `{structured_fields_with_broadened_capture,
   none}` MUST carry a non-null envelope `approval_ticket_ref`.
9. `event_kind = unsupported_source_placeholder` MUST carry
   `payload_kind = opaque_preservation_only` and
   `confidence_class = unknown`.
10. `provenance.normalization_passes` MUST start with
    `raw_capture_only` and MUST end with
    `adapter_confidence_downgrade` or `adapter_confidence_preserved`.
11. Every adapter row in
    [`artifacts/tooling/adapter_map.yaml`](../../artifacts/tooling/adapter_map.yaml)
    MUST cite at least one fixture under
    `fixtures/tooling/task_event_replay/`.
12. Milestone slugs (for example `M0`, `M00`, `M00-190`, `WP-01`)
    MUST NOT appear in any envelope, adapter row, fixture, or
    registered id.

## Cross-surface parity

The same envelope contract is consumed by:

- the desktop task pane, test explorer, diagnostics panel, build /
  run progress surfaces, and debug-prep views;
- the CLI / headless stable JSON surface
  (`docs/automation/cli_surface_contract.md`) when it emits task,
  test, build, or diagnostic output;
- the support-bundle packet family for diagnostic triage and replay;
- the benchmark harness when it reports build / test runtime and
  resource-usage samples;
- the AI broker when it consults build / test events as context;
- third-party automation consumers that read the CLI / headless
  JSON surface or the support-bundle export.

No surface invents its own task / test / diagnostic / artifact event
shape. Surfaces that want structure not yet covered by the envelope
add an additive-minor `payload_kind` (bumping
`task_event_envelope_schema_version`) rather than minting a parallel
family.

## Out of scope (for this packet)

- Live native task-runner, BSP client, BEP reader, structured-output
  adapter, heuristic-parser adapter, notebook-kernel adapter, or
  debugger-adapter implementation wiring.
- Raw-payload retention engine implementation (the retention
  `raw_payload_ref` is reserved here; the storage surface lands with
  the support-bundle packet family).
- Per-UI surface layouts for task panes, test explorers,
  diagnostics panels, or debugger views.
- Full AI-broker context-assembly wiring; this packet reserves the
  envelope shape AI context assembly will consume.
- Migration from any prior task / test / build output scraper
  (there is no prior production implementation to migrate from).
