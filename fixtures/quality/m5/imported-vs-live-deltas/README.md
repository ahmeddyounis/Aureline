# M5 Imported-versus-Live Delta Fixtures

Conformance fixtures for the M5 diagnostic-quality snapshot and
imported-versus-live delta contract.

## quality_parity_set.json

The full `DiagnosticQualityParityPacket`: four diagnostic-quality snapshots (a
live language-service snapshot, a live runtime/test snapshot, an imported scanner
snapshot held read-only, and a stale nightly-CI import) and three
imported-versus-live delta packets (an imported scan versus a live rerun, a stale
CI scan versus a current local rerun, and a runtime lane versus a static lane).

Each snapshot names the active quality-profile ref and fingerprint, the
rule-pack/tool versions in force, the recent collection ids the findings were
drawn from, the suppression/baseline refs and release-visible debt count, the
imported scanner session refs, and the last save-participant outcomes. Each delta
packet keeps its two sides' distinct imported-versus-live origin and freshness and
states a compatibility verdict with explicit notes:

- the imported-versus-live delta is **compatible only with local confirmation** —
  the imported side stays a static snapshot until a live rerun confirms it;
- the CI-versus-local delta is **blocked by a rule-pack mismatch** with named
  compatibility notes, so a stale CI scan cannot impersonate a current local
  result;
- the runtime-versus-static delta is **exactly comparable** across two live lanes
  and carries no caveat;
- the nightly-CI import snapshot is **stale** against the current rule-pack epoch,
  so it auto-downgrades from `beta` to `held` with a `stale_governance_state`
  trigger and a precise degraded label.

This fixture validates against
[`schemas/quality/diagnostic-quality-parity.schema.json`](../../../../schemas/quality/diagnostic-quality-parity.schema.json)
and is byte-identical to the checked support export at
[`artifacts/m5/diagnostics/diagnostic-quality-parity-proof/support_export.json`](../../../../artifacts/m5/diagnostics/diagnostic-quality-parity-proof/support_export.json).

## quality_snapshot.example.json

A single `diagnostic_quality_snapshot` (the imported-scanner snapshot, with
imported scanner session refs and suppression/baseline state) that validates
against
[`schemas/quality/diagnostic-quality-snapshot.schema.json`](../../../../schemas/quality/diagnostic-quality-snapshot.schema.json).

## delta_packet.example.json

A single `diagnostic_delta_packet` (the imported-versus-live comparison, with two
distinct-origin sides and a named compatibility note) that validates against
[`schemas/quality/diagnostic-delta-packet.schema.json`](../../../../schemas/quality/diagnostic-delta-packet.schema.json).
