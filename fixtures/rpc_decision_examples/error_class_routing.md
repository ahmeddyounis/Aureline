# Fixture: error-class routing

## Scenario

A batch of representative failures — a schema validation failure on
the caller side, a provider-integration hiccup from an LSP adapter, a
cancelled long-running search, an unknown-method rejection, and an
assertion failure in a service — all flow through the transport. Each
must be classified into exactly one frozen class with a stable code,
and each class must route consistently through the CLI exit-code
model and the support-bundle summary.

## Envelope fields exercised

- `error_payload.class` is one of the nine frozen ids; no class id is
  reused across concerns.
- `error_payload.code` is dotted-lowercase, per-class stable (for
  example, `vfs.path_denied`, `lsp.provider_timeout`,
  `rpc.handshake_mismatch`).
- `error_payload.retry` is `no`, `after_ms(u32)`, or
  `reauth_required`; each class has a documented default posture.
- `error_payload.span_context` lets the consumer re-enter the
  producer's trace.

## Hooks exercised

- `rpc_error_classified` — fires once per error response; the
  emitted attributes (class, code) are what the observability stream
  counts.

## Expected observable outcomes

- A schema failure is ALWAYS `local` (caller-side) or `remote`
  (peer-side); it is never `internal`.
- `deadline_exceeded` and `cancelled` are NEVER rewritten into
  `unavailable`.
- The same class vocabulary flows to:
  - CLI exit codes (Appendix B.2 mapping).
  - Audit records.
  - Support bundles.
- A new `code` can be added within an existing `class` without an
  ADR; a new `class` requires a new decision row.

## ADR sections motivating this fixture

- Error taxonomy — frozen class ids and code shape.
- Tradeoff table, **Error honesty** — class+code is the cross-product
  vocabulary.
- Protected-hot-path hooks — `rpc_error_classified` (observability
  only, not a budget holder).
