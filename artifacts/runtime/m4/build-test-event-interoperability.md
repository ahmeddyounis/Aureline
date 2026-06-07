# Build/Test Event Interoperability Review Artifact

The checked-in stable packet for this lane is:

`artifacts/runtime/m4/build_test_event_interoperability_packet.json`

It proves that one export-safe packet links:

- canonical lifecycle events across native, BSP, Bazel BEP/BES, structured
  output, and heuristic parser sources;
- source kind, confidence, raw payload refs, execution context, target id, and
  producer provenance on every envelope;
- explicit adapter capability negotiation for local, remote/helper, and
  imported-provider lanes;
- Problems, output, task/test/debug, AI, review, release, and support
  projections that preserve source kind and confidence;
- replay/export parity without exporting raw private payload bodies.

The fixture corpus includes negative cases for raw-payload reference loss,
heuristic overclaiming, capability negotiation drift, consumer confidence
projection drift, unsafe export redaction, and missing lifecycle coverage.
