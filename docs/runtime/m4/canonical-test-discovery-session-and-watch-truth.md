# Canonical Test Discovery Session And Watch Truth

This contract is the stable test-truth packet for launch-stable test lanes. It
is the packet that test explorer rows, inline results, CLI/headless output,
support exports, and release evidence read when they need to know what was
discovered, what actually ran, what changed across watch/retry/import flows, and
which scope is muted, quarantined, reduced, polling, unavailable, or imported.

## Boundary

- Rust model:
  `crates/aureline-debug/src/canonical_test_discovery_session_and_watch_truth/mod.rs`
- Schema:
  `schemas/runtime/canonical-test-discovery-session-and-watch-truth.schema.json`
- Checked-in packet:
  `artifacts/runtime/m4/canonical_test_discovery_session_and_watch_truth_packet.json`
- Fixture corpus:
  `fixtures/testing/m4/canonical-test-discovery-session-and-watch-truth/`

The packet is metadata-only. It carries durable IDs, source anchors, target and
environment refs, governed artifact refs, and export-safe summaries. It does not
carry raw runner output, raw source bodies, stack traces, command lines, process
environment bodies, secrets, provider payloads, or display labels as durable
identity.

## Required Objects

Stable packets must include discovery records for:

- suite/container rows
- concrete cases
- parameterized templates
- parameterized invocations
- notebook or interactive test rows
- partial-discovery rows with explicit omitted-scope reasons

Stable packets must also include:

- session plans with discovery snapshot, exact selection, target, environment,
  retry policy, and watch policy refs
- append-only attempt records with one-based per-session attempt indexes
- stability verdicts with evidence attempts and evidence windows
- governed mute/quarantine records with owner, reason, expiry, evidence refs,
  and release visibility
- triage packets for failing or governed scope
- watch-state records grouped by session series and attempt refs
- imported-CI parity summaries that remain read-only and link provider evidence
  to current discovery snapshots and local rerun plans
- state counts for live, reduced, polling, unavailable, quarantined, muted, and
  imported rows

## Readiness Rules

- Display labels are never durable test IDs.
- Partial discovery must name omitted scope and why it is omitted.
- Watch mode is represented as a session series or grouped attempts, not a
  mutable anonymous status row.
- Quarantine and mute are distinct governance actions. Quarantine requires
  owner, reason, expiry, release visibility, and evidence.
- Expired quarantines must remain release-visible and block readiness until
  cleared or renewed.
- Imported CI sessions are read-only evidence. They may link to local rerun
  plans and parity attempts, but they cannot impersonate current local runs.
- Release and support packets must count live, reduced, polling, unavailable,
  quarantined, muted, and imported states separately.

## Verification

```sh
cargo fmt -p aureline-debug
cargo test -p aureline-debug canonical_test_discovery_session_and_watch_truth
```
