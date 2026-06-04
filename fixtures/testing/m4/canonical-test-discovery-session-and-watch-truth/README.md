# Canonical Test Discovery Session And Watch Truth Fixtures

This corpus exercises the stable packet defined by
`schemas/runtime/canonical-test-discovery-session-and-watch-truth.schema.json`
and `crates/aureline-debug::canonical_test_discovery_session_and_watch_truth`.

| Fixture | Purpose |
|---|---|
| `baseline_stable.json` | Covers suite/container, concrete case, parameterized template, parameterized invocation, notebook/interactive, partial discovery, append-only watch attempts, visible quarantine governance, and imported-CI parity linkage. |

The fixture intentionally uses durable refs and digest-like anchors rather than
raw display labels, raw runner output, raw source paths, stack traces, command
lines, environment bodies, or provider payloads.
