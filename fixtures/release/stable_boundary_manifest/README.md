# Stable boundary manifest fixture cases

Negative fixtures for `ci/check_stable_boundary_manifest.py`. Each `*.json` file is
a complete stable boundary manifest that is structurally valid **except for one
targeted flaw**, paired in [`cases.json`](./cases.json) with the check id its flaw
must trip.

The gate runs every case during `--check` and fails when a case marked `rejected`
validates clean or trips a different check than expected, so the per-value-line
narrowing and packet-freshness-SLO rejections stay live even as the canonical
manifest (`artifacts/release/stable_boundary_manifest.json`) evolves. The Rust
contract test (`crates/aureline-release/tests/stable_boundary_manifest.rs`) also
parses each case and asserts the typed model rejects it.

| Case | Flaw | Expected check id |
|---|---|---|
| `narrowing_row_not_narrowed.json` | A narrowing-state value-line row still publishes a Stable label | `row.published_not_narrowed` |
| `breached_packet_on_published.json` | A published value-line row rides a packet whose freshness-SLO state is breached | `row.held_on_stale_packet` |

To regenerate a case after a deliberate manifest shape change, copy the canonical
manifest, reintroduce the single flaw, recompute the `summary` and `publication`
blocks, and confirm the gate still trips the expected check id.
