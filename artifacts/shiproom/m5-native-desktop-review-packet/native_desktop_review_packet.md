# Shiproom review packet — M5 native-desktop integration and reopen

This packet is the shiproom-facing view of the native-desktop matrix. It does
not maintain its own summary: every claim below is read from the canonical
objects, and the row table is rendered from the same seed as
`artifacts/platform/m5-native-desktop-matrix.md`.

## Canonical inputs

- Matrix: `artifacts/platform/m5-native-desktop-matrix.md`
- Report fixture: `fixtures/platform/m5_os_entry_and_reopen/report.json`
- Boundary schema: `schemas/platform/m5-native-desktop-matrix.schema.json`
- Companion doc: `docs/m5/native-desktop-integration-and-reopen.md`
- CI gate: `tools/ci/m5/native_desktop_check.py`

## Sign-off gate

Promotion holds unless all of the following are true on the current matrix:

1. Every required surface kind is present and every required control is
   satisfied by at least one surface (`report.report_clean == true`).
2. No surface carries a blocking finding — in particular none of the distinct
   failure classes (`trust_evaluation_bypassed`, `hidden_handler_takeover`,
   `wrong_target_no_recovery`, `unavailable_path_silent_loss`,
   `policy_block_unsafe`, `transient_poll_signal`,
   `privacy_unsafe_notification`).
3. No marketed surface carries stale evidence
   (`narrowable_marketed_entries` is empty).
4. The cross-links to the install-topology, embedded-boundary,
   activity-center, and auth-recovery packets are present.

A red emergency here is never silent: a wrong-target reopen, a hidden handler
takeover, or a privacy-unsafe notification each surfaces as its own finding,
and a marketed surface whose evidence is stale is narrowed rather than shipped
as implicitly stable.

## Reviewer checklist

- [ ] `python3 tools/ci/m5/native_desktop_check.py` exits clean.
- [ ] `cargo test -p aureline-shell --test m5_native_desktop_fixtures` passes.
- [ ] Each row's channel/build owner, platform scope, trust checkpoint, and the
      wrong-target / unavailable-path / policy-blocked recovery behavior read
      correctly against the install-topology and channel-ownership ledgers.
- [ ] The degraded-state vocabulary on each row matches the strings shipped in
      the user-visible surfaces.
- [ ] Evidence freshness is current on every marketed row; any stale row is
      narrowed in the affected surfaces.

## Regenerating this packet

This packet is checked in alongside the matrix it reviews. When the matrix
contract changes, regenerate the matrix and re-run the gate before re-reviewing:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop -- report-md > \
  artifacts/platform/m5-native-desktop-matrix.md
python3 tools/ci/m5/native_desktop_check.py
```
