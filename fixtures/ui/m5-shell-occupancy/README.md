# M5 shell-zone occupancy fixtures

Protected, redaction-safe fixtures for the M5 shell-zone occupancy & declared-slot
routing capstone (see
[`docs/shell/m5_shell_zone_occupancy_contract.md`](../../../docs/shell/m5_shell_zone_occupancy_contract.md)).

These files are minted by the headless emitter and asserted bit-for-bit equal to the
seed by `crates/aureline-shell/tests/m5_shell_zone_occupancy_fixtures.rs`. Do not
hand-edit; regenerate with:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_occupancy --"
$BIN packet         > fixtures/ui/m5-shell-occupancy/packet.json
$BIN dashboard      > fixtures/ui/m5-shell-occupancy/dashboard.json
$BIN support-export > fixtures/ui/m5-shell-occupancy/support_export.json
$BIN compact        > fixtures/ui/m5-shell-occupancy/compact.txt
```

- `packet.json` — canonical occupancy packet (6 green, 4 yellow, 0 red).
- `dashboard.json` — the light occupancy dashboard projection.
- `support_export.json` — packet + dashboard + stable case ids.
- `compact.txt` — headless compact-line rendering.
