# M5 desktop-profile certification fixtures

Protected, redaction-safe fixtures for the M5 desktop-profile certification capstone (see
[`docs/shell/m5_desktop_profile_certification_contract.md`](../../../docs/shell/m5_desktop_profile_certification_contract.md)).

These files are minted by the headless emitter and asserted bit-for-bit equal to the seed by
`crates/aureline-shell/tests/m5_desktop_profile_certification_fixtures.rs`. Do not hand-edit;
regenerate with:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_desktop_profile_certification --"
$BIN packet         > fixtures/ui/m5-desktop-profile-certification/packet.json
$BIN dashboard      > fixtures/ui/m5-desktop-profile-certification/dashboard.json
$BIN support-export > fixtures/ui/m5-desktop-profile-certification/support_export.json
$BIN compact        > fixtures/ui/m5-desktop-profile-certification/compact.txt
```

- `packet.json` — canonical certification packet (3 green, 3 yellow, 0 red).
- `dashboard.json` — the light certification dashboard projection.
- `support_export.json` — packet + dashboard + stable case ids.
- `compact.txt` — headless compact-line rendering.
