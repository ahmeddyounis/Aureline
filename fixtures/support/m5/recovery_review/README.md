# Recovery Review Fixtures

This directory holds the protected fixture corpus for the metadata-only
`recovery_review_packet`.

- `packet.json` keeps one continuity row, one crash-loop review row, one scoped
  reset review row, and one quarantine review row so schema and packet
  validation stay reproducible without depending on runtime builders.
