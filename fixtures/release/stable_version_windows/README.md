# Stable version-window freeze fixture cases

Negative fixtures for `ci/check_stable_version_windows.py`. Each `*.json` file is a
complete stable version-window freeze that is structurally valid **except for one
targeted flaw**, paired in [`cases.json`](./cases.json) with the check id its flaw must
trip.

The gate runs every case during `--check` and fails when a case marked `rejected`
validates clean or trips a different check than expected, so the surface-narrowing and
packet-freshness-SLO rejections stay live even as the canonical freeze
(`artifacts/release/stable_version_windows.json`) evolves. The Rust contract test
(`crates/aureline-release/tests/stable_version_windows.rs`) also parses each case and
asserts the typed model rejects it.

| Case | Flaw | Expected check id |
|---|---|---|
| `narrowing_window_not_narrowed.json` | A narrowing-state window row still freezes a Stable label | `row.frozen_not_narrowed` |
| `breached_packet_on_frozen.json` | A frozen window row rides a packet whose freshness-SLO state is breached | `row.held_on_stale_packet` |

To regenerate a case after a deliberate freeze shape change, copy the canonical freeze,
reintroduce the single flaw, recompute the `summary` and `publication` blocks, and
confirm the gate still trips the expected check id.
