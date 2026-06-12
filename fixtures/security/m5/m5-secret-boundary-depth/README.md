# M5 Secret Boundary Depth Fixtures

This fixture set mirrors the canonical packet in `artifacts/security/m5/m5-secret-boundary-depth.json`.

- `canonical_packet.json` is the checked packet replay used by `aureline-auth` tests.
- `support_export.json` is the metadata-only support export derived from the same packet.

The fixture boundary is redaction-safe:

- raw secret values are excluded
- raw handle ids are excluded
- row ids, default modes, export posture, and repair-owner tokens are preserved
