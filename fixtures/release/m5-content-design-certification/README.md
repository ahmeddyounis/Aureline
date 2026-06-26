# M5 content-design certification fixtures

Protected fixtures for the M5 content-design certification capstone. They are the
mint-from-truth output of the headless emitter and are asserted bit-for-bit equal
to the seed by
`crates/aureline-shell/tests/m5_content_design_certification_fixtures.rs` and by
`tools/ci/m5/content_design_certification_check.py`.

| File | Record |
| ---- | ------ |
| `packet.json` | The full content-design certification packet (one row per governed wording object, with derived green/yellow/red status, active waivers, and exact stale-proof causes). |
| `dashboard.json` | The light content-truth dashboard release / public-truth automation reads to auto-narrow marketed wording rows. |
| `support_export.json` | The support-export wrapper, quoting the packet, dashboard, and case ids. |
| `compact.txt` | Deterministic compact lines for headless review. |

Do not hand-edit. Regenerate with the headless emitter — see
`docs/release/m5-content-design-certification.md` → "Verify".
