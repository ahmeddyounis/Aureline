# M5 lifecycle transition-safety fixtures

Protected, redaction-safe fixtures for the M5 lifecycle transition-safety capstone (see
[`docs/lifecycle/m5_lifecycle_transition_safety_contract.md`](../../../docs/lifecycle/m5_lifecycle_transition_safety_contract.md)).

These files are minted by the headless emitter and asserted bit-for-bit equal to the seed by
`crates/aureline-shell/tests/m5_lifecycle_transition_safety_fixtures.rs`. Do not hand-edit;
regenerate with:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_transition_safety --"
$BIN packet         > fixtures/state/m5-lifecycle-transition-safety/packet.json
$BIN dashboard      > fixtures/state/m5-lifecycle-transition-safety/dashboard.json
$BIN support-export > fixtures/state/m5-lifecycle-transition-safety/support_export.json
$BIN compact        > fixtures/state/m5-lifecycle-transition-safety/compact.txt
```

- `packet.json` — canonical certification packet (9 green, 4 yellow, 0 red).
- `dashboard.json` — the light certification dashboard projection.
- `support_export.json` — packet + dashboard + stable case ids.
- `compact.txt` — headless compact-line rendering.
