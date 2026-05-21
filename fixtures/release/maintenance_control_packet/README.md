# Maintenance-control packet fixture cases

Negative fixtures for `ci/check_maintenance_control_packet.py`. Each `*.json` file is a
complete maintenance-control packet that is structurally valid **except for one targeted
flaw**, paired in [`cases.json`](./cases.json) with the check id its flaw must trip.

The gate runs every case during `--check` and fails when a case marked `rejected`
validates clean or trips a different check than expected, so the lane-narrowing and
control-packet-freshness-SLO rejections stay live even as the canonical packet
(`artifacts/release/maintenance_control_packet.json`) evolves. The Rust contract test
(`crates/aureline-release/tests/maintenance_control_packet.rs`) also parses each case and
asserts the typed model rejects it.

| Case | Flaw | Expected check id |
|---|---|---|
| `narrowing_lane_not_narrowed.json` | A narrowing-state control row still governs a Stable label | `row.controlled_not_narrowed` |
| `breached_packet_on_governed.json` | A governed control row rides a packet whose freshness-SLO state is breached | `row.held_on_stale_packet` |

To regenerate a case after a deliberate packet shape change, copy the canonical packet,
reintroduce the single flaw, recompute the `summary` and `publication` blocks, and
confirm the gate still trips the expected check id.
