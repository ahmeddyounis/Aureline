# ADR 0004 — Typed internal RPC transport and schema-toolchain

- **Decision id:** D-0004 (see `artifacts/governance/decision_index.yaml#D-0004`)
- **Status:** Accepted
- **Decision date:** 2026-04-19
- **Freeze deadline:** 2026-07-15
- **Owner:** `@ahmedyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** architecture_council
- **Related requirement ids:** none

## Context

Every service boundary inside Aureline — shell ↔ supervisor, supervisor ↔
workers, workspace-authority ↔ editor, editor ↔ VFS, VFS ↔ telemetry,
renderer ↔ accessibility bridge, plus the remote-proxy seam — speaks
through the same internal transport. Whichever shape that transport
takes becomes the floor under every later deadline claim, cancellation
guarantee, trace-joining contract, and schema-evolution posture. Holding
that floor open invites each lane to negotiate its own framing,
envelope fields, error taxonomy, or version vector and then discover
late that they disagree — exactly the drift the ownership matrix's
`crates/aureline-rpc` lane exists to prevent.

The freeze matters because later work cannot land honestly on top of an
unfrozen transport: the shell / command-system lane cannot promise
"cancellable, deadline-aware dispatch" without knowing what the envelope
carries; the VFS lane cannot promise "external-change merge is
trace-joinable against the edit that triggered the reload" without a
shared trace format; the telemetry lane cannot build a protected-path
trace scope without knowing the span vocabulary; the benchmark lab
cannot instrument queue-depth and cancellation-lag hooks without canonical
hook names; and the eventual remote-agent lane cannot run a capability
negotiation if the local contract has not named one. An unfrozen
transport also keeps the build-vs-reuse posture under tension because
every lane can credibly argue for its own IDL.

This ADR closes `D-0004` (RPC transport and cross-process contract)
ahead of its `2026-07-15` freeze so shell, VFS, telemetry, and
benchmark work can start instrumenting against concrete hook names,
concrete envelopes, and one schema vocabulary rather than against a
moving target. It is scoped to the **local / internal M0 contract**
(in-process, same-host cross-process, and a single remote-proxy seam);
a full production service mesh is out of scope until a separate
decision row opens.

## Decision

Aureline freezes a single internal RPC transport, one request /
response envelope, one event-stream envelope, one schema-definition and
validation strategy, one versioning-and-capability-negotiation posture,
one set of request-metadata rules (trace, deadline, cancellation,
workspace scope, idempotency), one canonical error taxonomy, and the
protected-hot-path hooks that govern them. All are stated in terms of
contracts, hook names, and class ids rather than specific crates so
dependency refresh is a hygiene change, not a re-litigation.

### Transport and framing

- **Fabric.** A typed, bidirectional, length-prefixed binary frame
  stream carried over:
  - **In-process** channels (same-process client and service sharing
    a Rust channel end; no serialisation required for identical
    schema versions, but the same envelope types are used so a trace
    captured in-process is indistinguishable from one captured over a
    socket);
  - **Same-host IPC** over Unix-domain sockets on macOS and Linux,
    and named pipes on Windows;
  - **Remote proxy seam** over a single tunnelled byte stream
    (SSH / container exec / managed tunnel); the remote connector
    terminates the byte stream and re-exposes the same framed
    protocol on the shell side.
- **Frame shape.** Every frame is `[u32 big-endian length][envelope
  payload bytes]`. The maximum frame size is a connection-level
  capability advertised in the handshake; the default ceiling is a
  megabyte-class bound sized to carry the largest *control* envelope
  (not large data; bulk bytes use a streamed continuation envelope —
  see below).
- **Connection.** One bidirectional connection per (client, service)
  pair. Multiplexing of concurrent requests happens *inside* the
  connection via a `request_id` on every envelope; there is no
  per-request connection churn on the protected path.
- **Backpressure.** The transport exposes a bounded send queue per
  request and per subscription. On queue saturation the transport
  reports a typed `unavailable` error rather than silently dropping or
  blocking the hot path; the queue-depth and queue-age signals are
  protected-hook observables (see below).
- **Keepalive.** Connections exchange keepalive frames at an
  implementation-chosen cadence; the hook `rpc_idle_keepalive` is
  observability-only.

### Wire encoding and schema of record

- **Schema of record.** Contracts live as Rust types inside
  `crates/aureline-rpc` (envelope types, trait definitions per
  service surface, `ContractVersion` tags per method). The crate is
  the single source of truth for the on-wire shape. An external tool
  that wants to generate bindings in another language consumes the
  machine-readable export described below, not the Rust types
  directly.
- **Wire encoding.** A deterministic, schema-aware binary encoding
  applied at the envelope boundary. The encoding is **negotiated per
  connection** in the handshake; at M0 the only encoding is
  `aureline/bin/1` — a serde-backed deterministic binary form. The
  encoding is invisible to service authors; they write typed Rust
  methods and the transport encodes. `aureline/bin/1` is frozen at
  this ADR; a future encoding lands as a new row in the tradeoff
  register and is negotiated alongside it rather than replacing it
  silently.
- **Machine-readable export.** A JSON Schema document lives at
  `schemas/rpc/envelope.schema.json` and describes the cross-lane
  envelope fields (request header, response header, event header,
  error, cancellation). It is the boundary schema: external tooling
  (documentation, generated bindings for companions, audit / support
  exports, CLI / SDK glue) reads this file and stays compatible
  across encoding evolutions because the *envelope* stays stable even
  when the per-method payload shapes evolve.
- **Method manifest.** A companion JSON document lives at
  `schemas/rpc/method_manifest.schema.json` and describes the
  manifest-of-methods shape. Each service's concrete manifest
  (produced by tooling from the Rust `inventory`-class registry) is
  published under the same directory as a release artifact; the
  registry itself is Rust-owned. The manifest is the vocabulary
  used by the capability-negotiation handshake.
- **Code generation posture.** At M0 there is **no external code
  generator**. Bindings for Rust consumers come from the crate
  directly; bindings for other languages land when a specific consumer
  (CLI, companion, remote helper) requires them and open their own
  decision row. This posture is explicit so the lane does not grow an
  IDL + toolchain before a consumer needs one. The `schemas/rpc/`
  files are the contract that any future generator MUST target.
- **External compatibility seam.** JSON-RPC adapters for LSP / DAP
  and other external protocols (AD-011) live *above* this transport
  in dedicated adapter crates. They translate into typed requests
  carrying this envelope; they do not replace it. The JSON-RPC
  seam never becomes the protected-path transport inside Aureline.

### Request / response envelope

Every request carries an envelope with the following fields. Field
names are the frozen vocabulary across every service.

| Field                 | Presence | Purpose                                                                                         |
|-----------------------|----------|-------------------------------------------------------------------------------------------------|
| `envelope_schema_version` | required | Integer; current value `1`. Bumped only on breaking envelope changes.                           |
| `request_id`          | required | Unique per connection; identifies the request for cancellation and response matching.           |
| `method`              | required | `service.method` identity, resolved against the method manifest.                                |
| `contract_version`    | required | Semver string of the method contract the sender is using.                                       |
| `trace`               | required | `{ trace_id: u128, span_id: u64, parent_span_id: u64?, flags: u8 }`; W3C-tracecontext-compatible.|
| `workspace_scope`     | required | Either a workspace id string or the literal `global`; never absent.                             |
| `deadline_ns`         | required | Absolute deadline on the connection clock (see §Deadlines). `0` means "no deadline" and is only legal for unbounded subscriptions. |
| `cancellation_channel`| required | A per-request token the sender uses to issue a `Cancel` frame; echoed on all related frames.    |
| `idempotency_key`     | optional | Present when the sender requires at-most-once server-side effect; bounded length.               |
| `baggage`             | optional | Bounded key-value map; dropped at the remote-proxy seam unless explicitly whitelisted.          |
| `actor_class`         | required | Originator classification (`user`, `command`, `recipe`, `extension`, `ai`, `system`, `remote`); informs policy and audit.|
| `payload`             | required | Typed, encoding-specific method payload.                                                        |

Every response carries:

| Field                 | Presence | Purpose                                                                                         |
|-----------------------|----------|-------------------------------------------------------------------------------------------------|
| `envelope_schema_version` | required | Same integer as the request; the service MUST NOT respond with a higher version.                |
| `request_id`          | required | Echoed from the request.                                                                        |
| `trace`               | required | Echoed or descended `trace_id` / `span_id`.                                                     |
| `contract_version`    | required | The service's chosen contract version for this call (negotiated at handshake).                  |
| `result`              | required | One of `Ok(payload)`, `Err(error)`, or `Progress(chunk)` for streamed responses.                |
| `terminal`            | required | Boolean; `true` on the last response frame, `false` on progress frames.                         |
| `server_hint_ns`      | optional | Server-reported remaining work estimate; observability only.                                    |

`Progress` frames carry the same `request_id` so long-running calls are
trace-joinable and cancellable without inventing a second channel.

### Event-stream envelope

Subscriptions and streaming events share one envelope. The envelope
is the same vocabulary as the request / response envelope plus
stream-specific fields; a consumer that knows the request envelope
knows the event envelope.

| Field                 | Presence | Purpose                                                                                         |
|-----------------------|----------|-------------------------------------------------------------------------------------------------|
| `envelope_schema_version` | required | Integer; `1`.                                                                                   |
| `subscription_id`     | required | Allocated by the service at subscribe time; used to cancel.                                     |
| `sequence`            | required | Per-subscription monotonic u64; consumers gap-detect on this field.                             |
| `kind`                | required | Payload kind id (e.g. `TaskStarted`, `BufferSnapshotDelta`); resolved against the method manifest.|
| `trace`               | required | Same shape as the request envelope; carries the producer span.                                  |
| `workspace_scope`     | required | Same semantics as the request envelope.                                                         |
| `schema_version`      | required | Per-payload-kind schema version; additive-minor evolution; consumers preserve unknown fields.   |
| `producer`            | required | Service id plus instance id of the producer.                                                    |
| `idempotency_key`     | optional | Required when the producer declares at-least-once delivery; consumers MUST dedupe on it.        |
| `delivery_mode`       | required | `exactly_once`, `at_least_once`, or `best_effort`; frozen at subscribe time.                    |
| `payload`             | required | Typed, kind-specific body.                                                                      |

Event streams never ride over separate plumbing from requests; they
are multiplexed on the same connection, with the same trace, deadline
(for the subscription-level timeout), and cancellation semantics. This
keeps the benchmark lab's queue-depth / queue-age observables one
vocabulary across request and event surfaces.

### Versioning and capability negotiation

- **Wire protocol version.** The outer framing is `wire/1`. A future
  `wire/2` lands as a new decision row; `wire/1` and `wire/2`
  coexisting on a connection is not legal at M0.
- **Envelope schema version.** `envelope_schema_version = 1`. Bumped
  only on breaking envelope changes; additive envelope fields
  (optional only) do not bump the version but do surface in the
  handshake capability vector so old consumers degrade gracefully.
- **Contract (per-method) versioning.** Each method carries a semver
  `ContractVersion`. Additive-minor evolution (new optional request
  fields, new optional response fields, new enum variants placed
  behind an `unknown` fallback) is permitted inside a major. Breaking
  changes require a major bump; the old major stays available for a
  deprecation window recorded in the method manifest.
- **Capability negotiation handshake.** On every new connection the
  two sides exchange:
  - supported wire versions;
  - supported envelope schema versions;
  - supported encoding ids (`aureline/bin/1`, future additions);
  - supported method-manifest snapshot (by digest) and the set of
    methods the peer is willing to invoke / serve;
  - the peer's clock-synchronisation seed (see §Deadlines);
  - the peer's advertised bounds (max frame size, max concurrent
    inflight requests per direction, maximum subscription fan-out).
  The handshake chooses the **intersection** of supported features,
  never an optimistic superset. A missing capability fails closed
  with a typed `unavailable` error and a human-readable reason; it
  does not silently downgrade an unrelated feature.
- **Unknown-field posture.** Within a major contract version,
  receivers MUST preserve unknown fields on read and echo them on
  any round-trip that re-emits the same entity (for example, an
  event consumer that republishes to a local subscriber). Known
  breaking fields fail closed with attribution.
- **Deprecation.** A method marked deprecated in the manifest is
  still served for the documented window; removal requires a major
  bump and a new decision row.

### Request metadata: deadlines, cancellation, trace, scope, idempotency

- **Deadlines.**
  - Every request has a monotonic deadline. At connect, the handshake
    exchanges a 128-bit clock seed and per-side monotonic-clock
    reading; the transport converts absolute deadlines into per-side
    monotonic-time targets without depending on wall-clock sync.
  - Deadlines propagate to fan-out calls: a service that issues
    downstream requests on behalf of an inbound request MUST inherit
    the inbound deadline minus the estimated local-work budget; it
    MAY NOT extend the deadline beyond the inbound value.
  - On expiry, the receiver emits `DeadlineExceeded` without
    guessing the outcome of the in-flight work; whether the work
    was observable is a separate, per-method idempotency question.
  - Subscriptions use `deadline_ns = 0` to mean "unbounded"; the
    subscription's lifetime is bounded by explicit cancel or by the
    server publishing a `terminal` event.

- **Cancellation.**
  - Every request advertises a `cancellation_channel`. A `Cancel`
    frame on that channel is idempotent; repeated cancels do not
    produce multiple effects.
  - A cancelled request either returns `Cancelled` (server honoured
    the cancel) or the method's typed terminal result if the server
    could not abort; the response frame tells the truth about which
    happened, never folds them into one.
  - Cancellation on the inbound request cancels inherited child
    calls; child services observe the same cancellation channel
    transitively.
  - Deadlines act as implicit cancellations; the transport is free
    to emit `Cancel` on deadline expiry.

- **Trace IDs.**
  - `trace_id` is 128-bit, `span_id` is 64-bit, `parent_span_id` is
    64-bit or `0`, `flags` is 8-bit (`sampled`, `debug`,
    `do_not_record`). The shape is W3C-tracecontext-compatible so
    external collectors can ingest traces without a translation
    layer.
  - Every service emits one span per handled request with the
    canonical `rpc.request` attribute set (method id, contract
    version, workspace scope, actor class, error class on failure).
  - Trace IDs traverse the remote proxy seam unchanged; the
    `baggage` map is dropped unless the remote connector's policy
    explicitly whitelists keys.

- **Workspace scope.**
  - Every request declares exactly one scope: a workspace id (stable
    across renames) or the literal `global`. `global` is permitted
    for supervisor control-plane methods, telemetry enrolment, and
    the capability-negotiation handshake itself.
  - Services MUST reject requests whose scope does not match their
    own authority domain with a typed `policy` error. The VFS never
    serves a read outside the requesting workspace; the editor
    service never accepts an edit against a workspace it does not
    own; the telemetry scope handles enrolment and global control
    only.

- **Idempotency.**
  - Request-level `idempotency_key` is mandatory for any method whose
    retry is observable (external mutation, credential refresh,
    destructive command dispatch). Methods whose retry is a local
    no-op (read-only queries) MAY omit the key.
  - Event-stream `idempotency_key` is mandatory for producers that
    declare `at_least_once` delivery; consumers MUST dedupe on it
    within the documented retention window.

### Error taxonomy

Every error response carries:

```
{ class: ErrorClass, code: string, reason: string, retry: RetryHint, span_context: Option<SpanContext> }
```

`ErrorClass` is the frozen taxonomy below. `code` is a per-class
stable string (for example `vfs.path_denied`); `reason` is
human-readable; `retry` is `no`, `after_ms(u32)`, or
`reauth_required`; `span_context` lets consumers re-enter the
producer's trace.

| Class id             | Scope                                                                          | Retry posture                        |
|----------------------|--------------------------------------------------------------------------------|--------------------------------------|
| `local`              | Caller-side contract violation or input error (bad arguments, schema fail)     | `no`                                 |
| `remote`             | Peer-service logical error not caused by the caller                            | Per-code; usually `no`               |
| `policy`             | Denied by policy or workspace trust                                            | `reauth_required` where applicable   |
| `environment`        | Missing or misconfigured host environment (toolchain, capsule, OS resource)    | `no` until repair                    |
| `provider`           | External provider / integration failure (LSP, DAP, cloud, registry)            | Per-code                             |
| `deadline_exceeded`  | Deadline elapsed before a terminal response                                    | Caller-controlled                    |
| `cancelled`          | Caller-initiated cancellation honoured                                         | `no`                                 |
| `unavailable`        | Transport saturation, peer absent, handshake failure, degraded-mode refusal    | `after_ms(u32)`                      |
| `internal`           | Service bug or impossible-case assertion failure                               | `no`; opens a support packet         |

Rules:

- Every typed error MUST declare a class id and a stable code.
- A class id MAY NOT be reused across concerns. A schema failure is
  always `local` (caller-side) or `remote` (peer-side); it is never
  `internal`.
- The `local` / `remote` / `policy` / `environment` / `provider`
  split is the same split AD-011 uses for the Appendix B exit-code
  model; the ADR ratifies that vocabulary for in-process RPC so CLI
  exit codes, audit records, and support bundles all speak it.
- `deadline_exceeded` and `cancelled` are always observable terminal
  outcomes; the transport never rewrites them into `unavailable`.

### Protected-hot-path hooks

The transport exposes the following named hooks. They are the
canonical instrumentation surface for the shell spike, the supervisor
prototype, the VFS and telemetry lanes, and the benchmark lab; no lane
MAY invent alternative names for the same measurement.

| Hook id                          | Fires when                                                                                          | Protected hot-path budget |
|----------------------------------|-----------------------------------------------------------------------------------------------------|---------------------------|
| `rpc_handshake_complete`         | A new connection finishes capability negotiation                                                    | yes                       |
| `rpc_capability_intersection`    | Handshake downgrades one or more advertised capabilities                                            | no (observability only)   |
| `rpc_request_send`               | The client-side transport writes a request envelope to the framing layer                            | yes                       |
| `rpc_request_receive`            | The server-side transport hands a decoded request to the service dispatcher                         | yes                       |
| `rpc_response_dispatch`          | The service writes a terminal response envelope                                                     | yes                       |
| `rpc_progress_emit`              | The service writes a non-terminal `Progress` frame                                                  | no (observability only)   |
| `rpc_cancel_observed`            | A service dispatcher observes a `Cancel` for an inflight request                                    | yes                       |
| `rpc_deadline_expired`           | The transport fires a deadline-driven cancel on an inflight request                                 | yes                       |
| `rpc_queue_saturation`           | A connection's send queue hits its bound; next frame will be typed-rejected                         | yes                       |
| `rpc_error_classified`           | An error envelope is written with a class and code                                                   | no (observability only)   |
| `rpc_idle_keepalive`             | A keepalive frame is exchanged                                                                      | no (observability only)   |
| `event_stream_publish`           | A producer writes an event frame into a subscription                                                | yes                       |
| `event_stream_consume`           | A consumer's dispatcher hands a decoded event to the subscriber closure                             | yes                       |
| `event_stream_gap_detected`      | A consumer observes a non-contiguous `sequence` on a subscription it trusted                        | yes                       |
| `event_stream_dedupe_hit`        | An `at_least_once` consumer drops a duplicate on `idempotency_key`                                  | no (observability only)   |
| `rpc_connection_drop`            | A connection terminates (graceful or otherwise), before reconnection backoff                        | yes                       |

The benchmark lab reports every hot-path hook against its protected
budget on claimed corpora; non-hot-path hooks are observability-only
and do not gate release.

### Non-goals at this decision

Out of scope until a superseding decision row opens:

- A full production service mesh, service discovery beyond the local
  supervisor, or multi-region routing.
- Cross-process shared memory as a transport; the seam is always a
  byte stream.
- Streaming of large buffer bytes over the RPC envelope. Bulk bytes
  travel through a dedicated streamed continuation payload (a
  method-level concern) or through a separate content-addressed
  channel; the envelope is not an mmap substitute.
- Bridging third-party RPC runtimes (gRPC, Cap'n Proto, Thrift) as
  the in-repo protected-path transport. External adapters live
  above this transport; they do not replace it.
- A second wire encoding beyond `aureline/bin/1`.
- An external IDL and its code generator toolchain.
- Public-SDK stability of the internal method manifest. The method
  manifest is internal at M0; public SDK surfaces land behind a
  separate decision row.
- End-to-end encryption on the wire beyond what the carrier (SSH,
  managed tunnel, loopback) already provides.

These lines move only by opening a new decision row, not by editing
this ADR.

### Tradeoff table

The structured tradeoff rows live in
`artifacts/architecture/rpc_tradeoff_rows.yaml`. The headline summary:

| Axis                                      | Chosen stack                                                                                     | Best rejected alternative                                                | Why chosen wins                                                                                           |
|-------------------------------------------|--------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------|
| **Hot-path performance**                  | Typed binary framing + serde-backed deterministic encoding + one multiplexed connection          | JSON-RPC over HTTP as the in-repo protected-path transport               | JSON-RPC's per-request parsing cost and verbose envelope collapse the shell's cancellation/latency budget |
| **Observability and cancellation**        | Envelope carries trace, deadline, cancellation, scope, idempotency; cancel is a first-class frame | Ambient cancellation via channel close + ambient tracing via thread locals | Ambient plumbing hides what crossed the seam; the envelope is what the support bundle can honestly quote |
| **Schema discipline**                     | Rust-native contracts, with JSON Schema exports for the envelope and method manifest             | Full IDL + codegen toolchain at M0                                       | An IDL without a second-language consumer costs more than it buys; the seam is reserved for when it appears |
| **Versioning and capability negotiation** | Wire version + envelope version + per-method semver + handshake intersection                     | "Latest-wins" implicit version with schema sniffing                      | Implicit versioning silently downgrades; handshake intersection fails closed with attribution             |
| **Error honesty**                         | Frozen class taxonomy with stable codes and retry hints                                          | Single anonymous `Err(String)` type                                      | String errors cannot be routed to repair, retry, or exit codes; the taxonomy is a cross-product vocabulary |
| **Compatibility with external ecosystem** | JSON-RPC adapters above this transport for LSP / DAP / legacy tools                              | Make JSON-RPC the in-repo protected-path transport                       | AD-010 / AD-011 already separate internal RPC from compatibility RPC; fusing them re-enters the JSON-RPC cost |
| **Remote proxy cost**                     | Same envelope over a tunnelled byte stream; same trace and cancellation cross the seam           | Remote-specific envelope with translation at the seam                    | Envelope translation drifts; one vocabulary is what makes remote feel local                               |

Each row carries reopen triggers in the YAML (for example: a
benchmark-lab finding that `rpc_request_send` exceeds its budget on
the in-process path reopens the encoding row).

### Decision-example fixtures

A small corpus of decision-example fixtures lives under
`fixtures/rpc_decision_examples/`. They are short, reviewable scenarios
(in-process local call, deadline expiry, caller-initiated cancel,
event-stream gap detection, cross-process trace join, capability
negotiation with missing method, policy denial, at-least-once
idempotency dedupe, remote proxy seam, error-class routing) used by
the shell spike, the supervisor prototype, and the benchmark lab to
anchor the hook names and the envelope fields above to concrete inputs
and observable outcomes. They are not a test suite; they are the
language the ADR's hook list and error taxonomy refer to.

## Consequences

- **Frozen:** the transport (length-prefixed framed bidirectional
  byte stream over in-process channels, Unix-domain sockets / named
  pipes, and a tunnelled remote proxy seam), the envelope fields for
  request / response / event, the versioning posture, the
  capability-negotiation handshake, the deadline / cancellation /
  trace / workspace-scope / idempotency rules, the error taxonomy,
  and the protected-hot-path hook names.
- **Frozen:** the schema of record is Rust types in
  `crates/aureline-rpc`; the boundary schemas for envelope and
  method manifest live under `schemas/rpc/`; there is no external
  IDL or codegen toolchain at M0.
- **Frozen:** the JSON-RPC compatibility surface (LSP, DAP, selected
  external adapters) lives *above* this transport and never replaces
  it on the protected path.
- **Permitted:** implementations of the transport MAY refresh the
  crate dependencies that back framing, encoding, and socket plumbing
  as long as the contract, envelope, and hook names stay unchanged.
- **Permitted:** adding a new method or event kind is an additive-minor
  contract change and does not require an ADR as long as the envelope
  is unchanged and the manifest's deprecation window is respected.
- **Permitted:** the remote connector MAY whitelist specific baggage
  keys for cross-seam propagation when the policy review grants it;
  the default is to drop baggage at the seam.
- **Follow-up:** the shell / command-system lane, the VFS lane, the
  editor service, and the telemetry lane instrument every hot-path
  hook before claiming latency budgets. The benchmark lab stabilises
  traces against the hooks on claimed corpora.
- **Follow-up:** the shared subscription envelope (decision row
  `D-0005`) consumes the event-stream envelope unchanged; any
  additions are additive-minor fields and reserve their names in the
  tradeoff register here before they land.
- **Follow-up:** the eventual public-SDK surface (a separate
  decision row) will promote a subset of the method manifest with its
  own stability posture; this ADR does not make the internal manifest
  a public-compatibility surface.
- **Ratifies:** the envelope vocabulary becomes the vocabulary used
  by audit records, support bundles, trace viewers, and CLI JSON
  output that cite an in-repo RPC call. The error taxonomy is the
  same vocabulary CLI exit codes use in Appendix B.2 of the TAD.

## Alternatives considered

- **JSON-RPC as the in-repo protected-path transport.** Use
  JSON-RPC (2.0-class) for every internal call, including shell ↔
  supervisor and supervisor ↔ workers. Rejected: AD-010 / AD-011
  already separate typed binary RPC (internal) from JSON-RPC
  (external compatibility); unifying them around JSON-RPC re-enters
  the JSON parsing cost and verbose envelope on the hot path, and
  the per-call overhead is incompatible with the shell's
  cancellation and latency budgets. JSON-RPC remains the
  compatibility choice above this transport for LSP / DAP.
- **gRPC / HTTP/2 as the in-repo transport.** Use gRPC or a
  gRPC-style framing runtime as the protected-path transport.
  Rejected: the HTTP/2 framing, TLS assumption, and
  trailer-semantics buy behaviour the local / same-host case does
  not need and impose a dependency footprint the solo-maintainer
  posture cannot sustain; service-mesh conveniences gRPC brings are
  out of scope until a production service mesh row opens.
- **Cap'n Proto / FlatBuffers zero-copy IDL.** Adopt a zero-copy IDL
  framework as the schema and wire. Rejected: the IDL + code
  generator toolchain is not justified before a second-language
  consumer exists, and the zero-copy read benefit on typical RPC
  envelopes is dwarfed by framing and dispatch cost at M0. The
  `schemas/rpc/` boundary schemas reserve a clean integration point
  if a future consumer forces the choice.
- **Protobuf IDL with prost-class codegen.** Adopt Protobuf 3 as the
  schema of record with a Rust codegen step. Rejected: same
  argument as Cap'n Proto — a full IDL without a non-Rust consumer
  costs more than it buys at M0. Also: Protobuf's default
  additive-by-default evolution hides breaking changes the
  capability-negotiation handshake here makes explicit.
- **Implicit / ambient plumbing.** Pass trace context, deadlines,
  and cancellation through thread locals and channel closures rather
  than inside the envelope. Rejected: ambient plumbing hides what
  crossed the seam, collapses support-bundle attribution, and
  breaks once the call crosses the remote proxy seam.
- **Single anonymous `Err(String)` error type.** Ship a string-only
  error surface. Rejected: string errors cannot be routed to repair,
  retry, or exit codes; the taxonomy is the vocabulary the CLI exit
  codes (Appendix B.2 of the TAD) and audit records already depend
  on.
- **Let each service choose its own transport.** Permit per-service
  transport choice (JSON-RPC for some, Rust channels for others,
  HTTP for a third). Rejected: drift across services, no shared
  trace vocabulary, no single cancellation surface, and the support
  bundle cannot quote a consistent envelope. The one-transport
  posture is the floor that prevents this.
- **Defer to a later milestone.** Leave `D-0004` open and let the
  `freeze_lane` default apply on `2026-07-15`. Rejected: the
  default posture would block dependent shell / VFS / telemetry /
  benchmark work during the most expensive months of pre-
  implementation, and the external-process consumers the default
  names (remote connector, support export, CLI) would each land with
  an incompatible envelope the lane would then have to reconcile.

The `D-0004` default-if-unresolved `freeze_lane` posture would have
blocked any non-spike work that depends on a cross-process RPC
contract until an ADR lands. Accepting this ADR replaces that
freeze with the frozen transport, envelope, versioning, request
metadata, error taxonomy, and hook list above; the `freeze_lane`
default does not apply.

## Reopen triggers

Each of the following MUST open a new decision row (not an edit of
this ADR):

- a benchmark finding that `rpc_request_send`, `rpc_request_receive`,
  `rpc_response_dispatch`, `event_stream_publish`, or
  `event_stream_consume` exceeds its budget on a claimed corpus and
  the fix would require swapping the transport, the framing, or the
  encoding;
- a claimed consumer (public SDK, browser companion, third-party
  integration) forces a second wire encoding or an external IDL and
  code generator into the M0 local contract;
- a security / policy review requires an envelope-level field this
  ADR did not name (for example, a mandated cryptographic receipt);
- the remote proxy seam can no longer carry the same envelope without
  per-seam translation;
- a service needs an error class that is not `local`, `remote`,
  `policy`, `environment`, `provider`, `deadline_exceeded`,
  `cancelled`, `unavailable`, or `internal`;
- the JSON-RPC compatibility adapter tier needs to publish its own
  envelope rather than translating into this one;
- collaboration / multi-author sessions force a CRDT-style delivery
  mode that the `exactly_once` / `at_least_once` / `best_effort`
  vocabulary above cannot describe.

## Platform-specific risk notes

- **macOS.** Unix-domain sockets are the same-host transport; AppKit
  activation policies and sandboxed processes change socket-path
  reachability. The remote proxy seam rides over SSH; network
  extensions and system proxies can terminate the stream mid-call.
  `rpc_connection_drop` is the trace anchor for those events.
- **Windows.** Named pipes replace Unix-domain sockets; ACL
  differences between elevated and standard processes and OneDrive /
  Files-on-Demand paths can surface as `policy` or `environment`
  errors at connect time. Antivirus injection on the supervisor
  process can stall the handshake; the handshake timeout is a
  protected observable.
- **Linux.** Abstract-namespace socket paths differ across
  distributions; the transport MUST accept both pathname and
  abstract variants. `seccomp` profiles can deny the socket calls;
  the fallback to polling the pipe for liveness is explicit and
  surfaces as `rpc_queue_saturation` rather than a silent stall.
- **All platforms.** Virtualised networking (containers, VPNs, VMs)
  can degrade the remote proxy seam without degrading local calls.
  The transport MUST NOT collapse a remote failure into a local
  `internal` error; cross-seam failures classify as `unavailable`
  with a remote span context.

## Benchmark-measurement expectations

- Every protected-hot-path hook reports latency to the benchmark lab
  on the claimed corpora (in-process, same-host IPC, and the remote
  proxy seam at a claimed round-trip bucket) and for each encoding
  ingested by the handshake (at M0, only `aureline/bin/1`).
- The benchmark lab's reproducibility pack for RPC claims names the
  host class, the transport variant (in-process, Unix-domain socket,
  named pipe, remote proxy), the encoding id, the envelope schema
  version, the method manifest digest, and the actor class mix at
  measurement time.
- A benchmark result that crosses a protected budget on a claimed
  host is a `red` lane state; repeated `yellow` on the same hook
  forces a scope correction per the milestone-scorecard rules.

## Source anchors

- `.t2/docs/Aureline_Technical_Architecture_Document.md:409` —
  AD-010: "typed binary RPC + event streams — performance,
  observability, capability negotiation".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:410` —
  AD-011: "JSON-RPC where required, otherwise typed contracts".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:801` —
  "every cross-service request carries a trace ID, workspace scope,
  deadline, and cancellation token".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:812` —
  "typed binary RPC over local sockets/streams — shell-supervisor,
  supervisor-workers, remote proxy links".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:820` —
  "internal RPC contracts are versioned and capability-negotiated".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:822` —
  "event envelopes are typed and attributable".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:823` —
  "errors distinguish between local, remote, policy, environment,
  and provider causes".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:955` —
  "typed internal RPC — shell ↔ supervisor ↔ first-party workers —
  versioned, deadline-aware, cancellable, traceable".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:957` —
  "JSON-RPC compatibility — LSP, DAP, selected external adapters —
  wrapped by typed facades where platform semantics matter".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:961` —
  "every cross-service request should carry a trace ID, workspace
  scope, deadline, and cancellation token".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1561` —
  "Invocation schema — typed args, validation, defaults, dry-run
  preview, and idempotency hints".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:3973` —
  "task / debug / terminal lifecycle events — at-least-once
  transport is allowed only with idempotency keys".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:5080` —
  "capability negotiation chooses the intersection of supported
  features, never optimistic superset behaviour".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:6224` —
  Suggested ADRs: "exact internal typed RPC framework and schema
  tooling".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:6453` —
  Appendix D: "ADR-004 — typed RPC, event-stream, and
  capability-negotiation transport".
- `.t2/docs/Aureline_Technical_Design_Document.md:867` —
  "Internal IPC — typed binary RPC + event streams — performance,
  structured tracing, and versioned contracts".
- `.t2/docs/Aureline_Technical_Design_Document.md:955` —
  "typed internal RPC — versioned, deadline-aware, cancellable,
  traceable".
- `.t2/docs/Aureline_Technical_Design_Document.md:961` —
  "every cross-service request should carry a trace ID, workspace
  scope, deadline, and cancellation token".
- `.t2/docs/Aureline_PRD.md:1819` — "cross-service interactions
  MUST occur through typed RPC, typed command dispatch, or
  append-only event streams".
- `.t2/docs/Aureline_PRD.md:1820` — "every cross-service request
  MUST carry a trace identifier, workspace scope, cancellation
  token, and deadline budget".
- `.t2/docs/Aureline_PRD.md:1821` — "services that may run locally
  or remotely MUST expose the same logical contract and capability
  negotiation".

## Linked artifacts

- Decision register row: `artifacts/governance/decision_index.yaml#D-0004`
- RFC: none.
- Tradeoff register (machine form):
  `artifacts/architecture/rpc_tradeoff_rows.yaml`.
- Envelope schema (machine form):
  `schemas/rpc/envelope.schema.json`.
- Method-manifest schema (machine form):
  `schemas/rpc/method_manifest.schema.json`.
- Decision-example fixtures:
  `fixtures/rpc_decision_examples/`.
- Prototype crate implementing the end-to-end example:
  `crates/aureline-rpc/`.
- Affected lanes: `crates/aureline-rpc`, `crates/aureline-telemetry`,
  `crates/aureline-vfs`, `crates/aureline-buffer`,
  `crates/aureline-shell-spike`,
  `artifacts/governance/ownership_matrix.yaml#scorecard_lane_index:benchmark_lab`,
  `artifacts/governance/ownership_matrix.yaml#scorecard_lane_index:shell_command_system`.

## Supersession history

First acceptance. No supersession.
