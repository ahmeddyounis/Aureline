# aureline-rpc

## Purpose
Transport layer for the supervisor and service fabric: framing, request/response
surface, deadline/cancellation plumbing, and trace-context propagation for
every cross-process call.

## Protected-path status
Protected. Reliability, deadline honesty, and trace-context integrity sit on
this crate.

## Allowed dependencies
- May depend on `aureline-telemetry` for trace propagation.
- Must not depend on `aureline-render`, `aureline-buffer`, `aureline-vfs`,
  `aureline-text`, or `aureline-shell-spike`.

## Canonical owner path
`crates/aureline-rpc/`

## Work packages
- WP-13 (Release engineering and governance)

## Contract anchor
- ADR: [`docs/adr/0004-rpc-transport-and-schema-toolchain.md`](../../docs/adr/0004-rpc-transport-and-schema-toolchain.md).
- Envelope boundary schema: [`schemas/rpc/envelope.schema.json`](../../schemas/rpc/envelope.schema.json).
- Method-manifest boundary schema:
  [`schemas/rpc/method_manifest.schema.json`](../../schemas/rpc/method_manifest.schema.json).
- Decision-example fixtures:
  [`fixtures/rpc_decision_examples/`](../../fixtures/rpc_decision_examples/).
- Tradeoff register:
  [`artifacts/architecture/rpc_tradeoff_rows.yaml`](../../artifacts/architecture/rpc_tradeoff_rows.yaml).

The Rust types in this crate are the schema of record; the JSON Schema files
above are the cross-tool boundary, derived from the types and reviewed in
lockstep with them.

## What the prototype proves

The initial code in `src/` freezes the envelope vocabulary, the frozen error
taxonomy, the trace-context shape, the method-manifest shape, and the
protected-hot-path hook ids against the ADR. A small `InProcessTransport`
demonstrates the end-to-end contract — handshake-driven capability
intersection, unary request/response, deadline enforcement, first-class
cancel frames, scope/actor-class policy enforcement, idempotency-required
methods, and event-stream publish/consume with gap detection and
at-least-once dedupe — all against the same types that the length-prefixed
byte-stream transport will ship with. The framing, socket plumbing, and
length-prefixed byte-stream transport land in a follow-up; this prototype
proves the contract compiles and enforces the rules.
