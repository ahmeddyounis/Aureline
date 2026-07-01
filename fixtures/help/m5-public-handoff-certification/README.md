# M5 public-handoff & capture-boundary certification fixtures

Protected fixtures for the M5 public-handoff certification capstone (`M05-723`).
They are the mint-from-truth output of the headless emitter
`aureline_shell_m5_public_handoff_certification` and are asserted bit-for-bit equal
to the seed by
`crates/aureline-shell/tests/m5_public_handoff_certification_fixtures.rs`.

| File | Record |
| ---- | ------ |
| `packet.json` | The full public-handoff certification packet (one row per governed object, with derived green/yellow/red status, active waivers, and exact stale-proof causes). |
| `dashboard.json` | The light boundary-truth dashboard release / public-truth automation reads to auto-narrow claimed surfaces. |
| `support_export.json` | The support-export wrapper, quoting the packet, dashboard, and case ids. |
| `compact.txt` | Deterministic compact lines for headless review. |

Do not hand-edit. Regenerate with the headless emitter — see
`docs/help/m5_public_handoff_certification_contract.md` → "Verify".
