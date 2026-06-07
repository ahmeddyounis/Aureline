# Build/Test Event Interoperability Fixtures

These fixtures exercise the stable `BuildTestEventInteroperabilityPacket`.

- `baseline_stable.json` validates the canonical source-kind, lifecycle,
  capability negotiation, consumer projection, and replay/export packet.
- The `*_blocks_stable.json` cases cover raw-payload loss, heuristic
  confidence overclaiming, missing capability negotiation, consumer projection
  drift, unsafe raw export posture, and missing canonical lifecycle coverage.
