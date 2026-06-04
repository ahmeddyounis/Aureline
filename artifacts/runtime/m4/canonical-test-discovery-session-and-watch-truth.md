# Canonical Test Discovery Session And Watch Truth Artifact

The stable proof artifact is
`artifacts/runtime/m4/canonical_test_discovery_session_and_watch_truth_packet.json`.
It binds the runtime test-truth lane to
`stable-proof-index:runtime:test-discovery-session-watch-truth` and includes:

- durable discovery rows for suite/container, concrete case, parameterized
  template, parameterized invocation, notebook/interactive, and partial
  discovery scope
- append-only watch attempts grouped under `watch-series:orders`
- a release-visible quarantine record with owner, reason, expiry, and evidence
- an imported-CI parity row linked to a current discovery snapshot and a local
  rerun plan
- separate counts for live, reduced, polling, unavailable, quarantined, muted,
  and imported states

This artifact is intentionally export-safe and excludes raw runner output, raw
source, command lines, environment bodies, provider payloads, secrets, and
display-label identity.
