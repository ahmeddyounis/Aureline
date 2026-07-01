# M5 lifecycle-vocabulary parity fixtures

Protected, redaction-safe fixtures for the M5 lifecycle-vocabulary parity capstone (see
[`docs/lifecycle/m5_lifecycle_vocabulary_parity_contract.md`](../../../docs/lifecycle/m5_lifecycle_vocabulary_parity_contract.md)).

These files are minted by the headless emitter and asserted bit-for-bit equal to the seed by
`crates/aureline-shell/tests/m5_lifecycle_vocabulary_parity_fixtures.rs`. Do not hand-edit;
regenerate with:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_vocabulary_parity --"
$BIN packet         > fixtures/state/m5-lifecycle-vocabulary-parity/packet.json
$BIN dashboard      > fixtures/state/m5-lifecycle-vocabulary-parity/dashboard.json
$BIN support-export > fixtures/state/m5-lifecycle-vocabulary-parity/support_export.json
$BIN compact        > fixtures/state/m5-lifecycle-vocabulary-parity/compact.txt
```

- `packet.json` — canonical parity packet (11 green, 4 yellow, 0 red).
- `dashboard.json` — the light parity dashboard projection.
- `support_export.json` — packet + dashboard + stable case ids.
- `compact.txt` — headless compact-line rendering.
