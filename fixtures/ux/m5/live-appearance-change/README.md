# M5 live-appearance-change fixtures

These fixtures are the checked-in canonical truth for the M5 appearance-session
runtime audit. They conform to
[`schemas/ux/appearance-session.schema.json`](../../../../schemas/ux/appearance-session.schema.json)
and are validated by the CI gate at
[`tools/ci/m5/appearance_session_check.py`](../../../../tools/ci/m5/appearance_session_check.py).
See the companion doc
[`docs/m5/appearance-session-runtime.md`](../../../../docs/m5/appearance-session-runtime.md)
for how the live appearance session and the checkpoint-aware state machine keep
appearance changes atomic, reversible, and inspectable.

| File | Record kind | Why it is here |
| --- | --- | --- |
| `report.json` | `shell_m5_appearance_session_runtime_report_record` | The live appearance session, the checkpoint ledger, the transition ledger, the per-surface bindings, and the blocking-finding summary. |
| `support_export.json` | `shell_m5_appearance_session_runtime_support_export_record` | The support-export wrapper a reviewer pivots on; its `case_ids` quote the report id, the session ref, every checkpoint ref, every transition ref, and every surface id and descriptor revision. |
| `compact.txt` | (rendered summary) | One-line-per-row audit summary the headless inspector prints. |

The fixtures are the **only mint-from-truth output** of the headless inspector
`aureline_shell_m5_appearance_session`; regenerate them with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_session -- report > \
  fixtures/ux/m5/live-appearance-change/report.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_session -- support-export > \
  fixtures/ux/m5/live-appearance-change/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_session -- compact > \
  fixtures/ux/m5/live-appearance-change/compact.txt
```

The clean checked-in report exercises every disclosed-appearance scenario the
contract is built to keep honest:

- The live session sits in a `preview_live` state, citing the single checkpoint
  that produced it — a live appearance change that is inspectable and
  reversible.
- The transition ledger demonstrates every state-machine edge: an OS contrast
  signal applied atomically through one checkpoint, a theme preview opened and
  promoted live, an overlay preview cancelled, a partner-theme preview that
  fails its contrast preflight and auto-reverts, an imported theme rolled back
  with a disclosed surface reload, and a managed density change committed.
- Six surfaces (notebook, data/result surface, preview/browser pane, docs/help
  pane, companion surface, extension-hosted panel) ride the shared session;
  three disclose a reload-or-restart requirement, and none paints its own
  appearance outside the session model.

Every record stays clean because each change flows through one explicit
checkpoint and every restart-or-reload requirement is disclosed; the same
conditions become blockers the moment a transition skips its checkpoint, lands
on an illegal state, hides a restart requirement, or a surface paints its own
appearance outside the shared session.
