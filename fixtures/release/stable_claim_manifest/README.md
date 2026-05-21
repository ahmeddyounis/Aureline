# Stable claim manifest fixture cases

Negative fixtures for `ci/check_stable_claim_manifest.py`. Each `*.json` file is a
complete stable claim manifest that is structurally valid **except for one
targeted flaw**, paired in [`cases.json`](./cases.json) with the check id its
flaw must trip.

The gate runs every case during `--check` and fails when a case marked
`rejected` validates clean or trips a different check than expected, so the
lifecycle-label narrowing and packet-freshness-SLO rejections stay live even as
the canonical manifest (`artifacts/release/stable_claim_manifest.json`) evolves.
The Rust contract test (`crates/aureline-release/tests/stable_claim_manifest.rs`)
also parses each case and asserts the typed model rejects it.

| Case | Flaw | Expected check id |
|---|---|---|
| `narrowing_entry_not_narrowed.json` | A narrowing-state entry still publishes its claimed lifecycle label | `entry.published_not_narrowed` |
| `breached_packet_on_published.json` | A published entry rides a packet whose freshness-SLO state is breached | `entry.held_on_stale_packet` |

To regenerate a case after a deliberate manifest shape change, copy the canonical
manifest, reintroduce the single flaw, recompute the `summary` and `publication`
blocks, and confirm the gate still trips the expected check id.
