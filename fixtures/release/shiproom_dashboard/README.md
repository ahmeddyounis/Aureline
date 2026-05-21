# Shiproom-dashboard fixture cases

Negative fixtures for `ci/check_shiproom_dashboard.py`. Each `*.json` file is a complete
shiproom dashboard that is structurally valid **except for one targeted flaw**, paired in
[`cases.json`](./cases.json) with the check id its flaw must trip.

The gate runs every case during `--check` and fails when a case marked `rejected` validates
clean or trips a different check than expected, so the panel-narrowing and
packet-freshness-SLO rejections stay live even as the canonical dashboard
(`artifacts/release/shiproom_dashboard.json`) evolves. The Rust contract test
(`crates/aureline-release/tests/shiproom_dashboard.rs`) also parses each case and asserts the
typed model rejects it.

| Case | Flaw | Expected check id |
|---|---|---|
| `narrowing_panel_not_narrowed.json` | A narrowing-state panel still renders a Stable label | `panel.displayed_not_narrowed` |
| `breached_packet_on_green.json` | A green panel rides a packet whose freshness-SLO state is breached | `panel.green_on_stale_packet` |

To regenerate a case after a deliberate dashboard shape change, copy the canonical
dashboard, reintroduce the single flaw, recompute the `summary` and `publication` blocks, and
confirm the gate still trips the expected check id.
