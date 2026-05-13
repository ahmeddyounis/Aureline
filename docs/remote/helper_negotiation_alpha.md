# Helper Capability Negotiation Alpha Contract

This contract freezes the alpha helper/agent capability envelope used by
remote attach, helper-backed execution rows, support exports, and the
mixed-version drift harness. It is a data contract and proof lane, not a
remote platform implementation.

Companion artifacts:

- [`/schemas/remote/helper_capabilities_alpha.schema.json`](../../schemas/remote/helper_capabilities_alpha.schema.json)
  defines the `helper_capability_envelope_record`.
- [`/fixtures/remote/mixed_version_drift_alpha/`](../../fixtures/remote/mixed_version_drift_alpha/)
  contains supported, limited, retry-required, and unsupported-skew cases.
- [`/ci/check_helper_capabilities_alpha.py`](../../ci/check_helper_capabilities_alpha.py)
  validates the schema, joins each case to the canonical compatibility and
  skew-window sources, and renders the support/export-safe projection.

The contract composes with:

- [`/schemas/compat/mixed_version_envelope.schema.json`](../../schemas/compat/mixed_version_envelope.schema.json)
  for the generic distributed-boundary version envelope.
- [`/schemas/release/helper_version_negotiation.schema.json`](../../schemas/release/helper_version_negotiation.schema.json)
  for release/update preflight packets.
- [`/schemas/runtime/remote_agent_hello.schema.json`](../../schemas/runtime/remote_agent_hello.schema.json)
  for remote-agent hello, heartbeat, reconnect, and target-witness records.
- [`/schemas/remote/attach_session.schema.json`](../../schemas/remote/attach_session.schema.json)
  for attach lifecycle and route truth.
- [`/schemas/runtime/prebuild_descriptor_alpha.schema.json`](../../schemas/runtime/prebuild_descriptor_alpha.schema.json)
  and [`/artifacts/templates/warm_start_descriptor_seed.yaml`](../../artifacts/templates/warm_start_descriptor_seed.yaml)
  for warmed, stale, or resume-capable environment metadata.

## Scope

Frozen here:

- the helper capability envelope exchanged between a client and helper or
  remote agent;
- the bounded result labels: `supported`, `limited`, `retry_required`, and
  `unsupported_skew`;
- the decision classes that decide whether attach is allowed, narrowed to
  review/file/inspect-only, retried after a probe, blocked, or held for
  upgrade/repin;
- the drift summary that records safe local continuation, cancelled mutation
  refs, preserved read-only refs, and repair actions;
- the support/export-safe projection consumed by the harness.

Out of scope:

- starting a real remote agent, helper binary, tunnel broker, task runner, or
  managed workspace;
- carrying raw endpoints, hostnames, paths, ports, credentials, helper process
  arguments, transport frames, logs, or support bundle bodies;
- redefining compatibility rows, skew windows, remote-agent hello state, attach
  lifecycle state, or prebuild freshness vocabulary.

## Core Invariants

1. **Compatibility truth is imported, not restated.**
   Each envelope cites `compat_row:remote.attach_envelope_and_drift`,
   `skew_window:desktop_cli_and_remote_agent.declared_adjacent_window`, and a
   concrete `skew_case:*` from the version-skew register.
2. **Negotiation chooses an intersection.**
   `negotiated_capabilities` must be a subset of both the client and helper
   advertised sets. Missing requested capabilities must appear in
   `dropped_capabilities` with a typed reason.
3. **Reduced labels disable mutation.**
   `limited`, `retry_required`, and `unsupported_skew` never carry remote
   mutation authority. Mutating capability classes must be dropped until the
   envelope returns to `supported`.
4. **Retry-required is not degraded support.**
   Untested same-major or probe-required pairings stay `retry_required` until a
   probe or reattach record moves them to supported, limited, or unsupported.
5. **Unsupported skew fails closed.**
   Unknown required features, protocol-floor mismatches outside the published
   window, or unsupported skew-register cases use `unsupported_skew` and name
   upgrade/repin or local-only continuation.
6. **Support projection is metadata-only.**
   The harness renders envelope id, versions, skew case, label, decision,
   posture, safe continuation, repair actions, and dropped capabilities. It
   never emits raw target, endpoint, secret, or payload material.

## Harness

Run the normal validation:

```sh
python3 ci/check_helper_capabilities_alpha.py --repo-root .
```

Render the support/export-safe projection:

```sh
python3 ci/check_helper_capabilities_alpha.py --repo-root . --render-negotiation-projection
```

Exercise a failure drill:

```sh
python3 ci/check_helper_capabilities_alpha.py --repo-root . \
  --force-drill helper_capability_case:remote_agent.unsupported_required_feature:rewrite_unsupported_skew_to_supported
```

The forced drill exits successfully only when the harness reproduces the named
check id, proving the lane fails loudly if unsupported skew is relabeled as
supported.
