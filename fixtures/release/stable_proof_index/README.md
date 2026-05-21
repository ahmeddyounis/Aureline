# Stable proof index fixture cases

Negative fixtures for `ci/check_stable_proof_index.py`. Each `*.json` file is a
complete stable proof index that is structurally valid **except for one targeted
flaw**, paired in [`cases.json`](./cases.json) with the check id its flaw must trip.

The gate runs every case during `--check` and fails when a case marked `rejected`
validates clean or trips a different check than expected, so the
requirement-narrowing and packet-freshness-SLO rejections stay live even as the
canonical index (`artifacts/release/stable_proof_index.json`) evolves. The Rust
contract test (`crates/aureline-release/tests/stable_proof_index.rs`) also parses
each case and asserts the typed model rejects it.

| Case | Flaw | Expected check id |
|---|---|---|
| `narrowing_row_not_narrowed.json` | A narrowing-state proof row still backs a Stable label | `row.proven_not_narrowed` |
| `breached_packet_on_proven.json` | A proven proof row rides a packet whose freshness-SLO state is breached | `row.held_on_stale_packet` |

To regenerate a case after a deliberate index shape change, copy the canonical
index, reintroduce the single flaw, recompute the `summary` and `publication`
blocks, and confirm the gate still trips the expected check id.
