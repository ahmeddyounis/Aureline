# M5 live-appearance OS-change corpus fixtures

These fixtures are the checked-in canonical truth for the M5 live-appearance
change & evidence-linkage report. They conform to
[`schemas/ux/m5-live-appearance-evidence.schema.json`](../../../../schemas/ux/m5-live-appearance-evidence.schema.json)
and are validated by the CI gate at
[`tools/ci/m5/live_appearance_evidence_check.py`](../../../../tools/ci/m5/live_appearance_evidence_check.py).
See the companion doc
[`docs/m5/live-appearance-and-evidence-linkage.md`](../../../../docs/m5/live-appearance-and-evidence-linkage.md)
for how the release/evidence center, support/export wrapper, extension-inspection
surface, and sync/import flows consume the same report object.

| File | Record kind | Why it is here |
| ---- | ----------- | -------------- |
| `report.json` | `shell_m5_live_appearance_evidence_report_record` | Live OS theme/contrast/accent/text-scale/reduce-motion changes across macOS, Windows, and Linux, each binding one platform-lab capture to the exact build, theme package, appearance session, and checkpoint that produced it, with apply posture, golden match, and trust/severity/lifecycle/focus cue outcomes, plus the cross-platform axis coverage and surface coverage summaries. |
| `support_export.json` | `shell_m5_live_appearance_evidence_support_export_record` | The support-export wrapper; `case_ids` quotes the report id, the exact-build ref, and every row id, appearance-session ref, checkpoint ref, theme-package ref, screenshot ref, and golden ref. |
| `compact.txt` | (rendered summary) | One-line report header, per-axis platform coverage, and a per-change summary for quick CI/log inspection. |

These fixtures are bit-for-bit equal to the output of
`seeded_live_appearance_evidence_report` in
[`crates/aureline-shell/src/live_appearance_evidence/mod.rs`](../../../../crates/aureline-shell/src/live_appearance_evidence/mod.rs),
enforced by the integration test
`crates/aureline-shell/tests/m5_live_appearance_evidence_fixtures.rs`. Regenerate
them with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- report > \
  fixtures/ux/m5/os-appearance-contrast-accent/report.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- support-export > \
  fixtures/ux/m5/os-appearance-contrast-accent/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- compact > \
  fixtures/ux/m5/os-appearance-contrast-accent/compact.txt
cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- markdown > \
  artifacts/ux/m5/live-appearance-platform-labs/m5_live_appearance_evidence.md
```

The corpus exercises the full posture and honesty spectrum the contract is built
to keep legible:

- live theme flip, contrast increase, accent change, and text-scale change that
  apply live through the appearance-session model;
- a Windows forced-colors change that needs a disclosed embedded preview reload;
- a Linux display-scale change that needs a disclosed app restart, whose
  post-restart capture is still attributed to the same session checkpoint; and
- a portable build that honestly omits forced-colors because the platform signal
  is unavailable, rather than faking a capture.
