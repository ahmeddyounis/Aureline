# M5 Problems / Output / Execution-Evidence Waiver and Downgrade Log

- Packet: `m5-problems-output-evidence-certification:stable:0001`
- Generated from: `artifacts/tooling/m5-problems-output-evidence-certification/support_export.json`
- Evidence freshness SLO: 168 hours (last refresh: 2026-06-21T00:00:00Z)

No manual waivers are granted: a profile sits below its claim only by automatic narrowing when current, reopenable proof cannot back it.

## Auto-downgraded profiles (1)

- **notebook_output**: claim `qualified` -> effective `retest_pending`
  - Trigger: `stale_evidence`
  - Held at retest_pending below the qualified claim: stale/superseded handling proof aged out; reopen-to-origin stays available until re-verified
  - Uncurrent dimensions: stale_superseded_handling
