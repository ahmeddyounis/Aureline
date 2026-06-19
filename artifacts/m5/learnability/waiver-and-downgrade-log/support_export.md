# M5 Learnability Waiver and Downgrade Log

- Packet: `m5-learnability-certification:stable:0001`
- Generated from: `artifacts/m5/learnability/certification-report/support_export.json`
- Evidence freshness SLO: 168 hours (last refresh: 2026-06-19T00:00:00Z)

No manual waivers are granted: a learnability row sits below its claim only by automatic narrowing when current, reopenable proof cannot back it.

## Auto-downgraded rows (1)

- **learn-cert:profiler_trace:stale-offline-mirror:0001** (profiler_trace): claim `certified` -> effective `uncertified`
  - Trigger: `offline_mirror_continuity_lost`
  - Offline/mirror docs-pack aged outside its freshness window; held uncertified until a fresh mirror re-backs continuity
  - Uncurrent dimensions: offline_mirror
