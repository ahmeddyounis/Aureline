# Stable Execution Evidence Bundle

This contract makes task events, problem rows, output channels, output chunks,
and execution evidence one exportable object family.

The stable truth source is `ExecutionEvidenceBundle` in `aureline-runtime`.
Problems, editor decorations, timeline rows, output tabs, CLI/headless output,
review, AI explanations, release packets, and support exports must preserve the
same object refs instead of scraping rendered pane text.

## Required Objects

- `StableTaskEventEnvelope` carries event ID, event kind, execution context,
  run/session ref, adapter/provider source, timestamp or sequence, parent and
  correlation refs, confidence, fallback state, and raw payload ref.
- `StableProblemRecord` carries severity, file/span or structured location,
  source tool, originating task event or static-analysis ref, confidence,
  quick-fix refs, raw-output/provider backlinks, freshness, editor decoration,
  timeline, and rerun refs.
- `StableOutputChannelDescriptor` carries canonical channel name, source
  subsystem, execution context, run/session ref, source kind, trust state,
  retention class, freshness, parser confidence, backlinks, and append-friendly
  searchable chunks.
- `StableExecutionEvidenceObject` links events, problems, output chunks,
  artifacts, mapping refs, freshness, baseline/comparison refs, qualifiers, and
  reopen refs.
- `ExecutionEvidenceBundle` packages all objects with a redaction profile,
  reopen lineage, source contracts, consumer projections, promotion state, and
  validation findings.

## Stability Rules

- Heuristic parser records must use explicit confidence and preserve raw-output
  backlinks.
- Imported provider evidence must remain distinct from current local or remote
  runtime truth.
- Output channels must use canonical names and append-friendly searchable chunks.
- Evidence objects and bundles must preserve reopen lineage for runs, channels,
  provider annotations, or artifacts when available.
- Consumer projections must preserve object refs, source kind, confidence,
  freshness, reopen lineage, and JSON export.

## Canonical Assets

- Schema: `schemas/runtime/execution-evidence-bundle.schema.json`
- Packet: `artifacts/runtime/m4/stabilize_problem_records_output_channels_and_execution_evidence_truth_packet.json`
- Fixtures: `fixtures/runtime/m4/stabilize_problem_records_output_channels_and_execution_evidence/`

