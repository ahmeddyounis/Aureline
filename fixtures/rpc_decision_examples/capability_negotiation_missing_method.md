# Fixture: capability negotiation with missing method

## Scenario

A newer caller advertises a method (for example,
`editor.reflow_with_shape_cache`) that the older service in the
current workspace does not serve. Capability negotiation on the
connection's handshake must discover the mismatch and fail the
call closed rather than silently downgrade.

## Envelope fields exercised

- Handshake exchanges: supported wire versions, envelope schema
  versions, encoding ids, method-manifest snapshot digest, and the
  set of methods each side is willing to invoke / serve.
- The intersection is chosen for the connection; no advertised
  capability is assumed from a superset.
- The call to the missing method returns
  `result = Err(error_payload)` with
  `error_payload.class = unavailable` and a human-readable `reason`
  that attributes the gap to the method-manifest intersection.
- `retry` is `after_ms(u32)` with a bounded delay (transport may
  retry against a refreshed handshake).

## Hooks exercised

- `rpc_handshake_complete` — fires on both sides at connection setup.
- `rpc_capability_intersection` — fires because the advertised and
  intersected capability sets differ.
- `rpc_error_classified` — class `unavailable` recorded on the
  subsequent call.

## Expected observable outcomes

- The error is `unavailable`, NOT `internal`: a missing capability is
  a legitimate and attributable outcome, not an assertion failure.
- The handshake never chooses a superset: if both sides advertise the
  same method but with disjoint contract-major versions, the method
  is reported as unavailable for this connection.
- The consumer surface (CLI exit code, Appendix B.2 taxonomy, support
  bundle) speaks the same `unavailable` vocabulary as the envelope.

## ADR sections motivating this fixture

- Versioning and capability negotiation — handshake intersection.
- Error taxonomy — `unavailable` class.
- Protected-hot-path hooks — `rpc_handshake_complete`,
  `rpc_capability_intersection`.
