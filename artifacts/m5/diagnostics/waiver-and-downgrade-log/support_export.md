# M5 Diagnostic-Truth Waiver and Downgrade Log

- Packet: `m5-diagnostic-truth-certification:stable:0001`
- Generated from: `artifacts/m5/diagnostics/certification-report/support_export.json`
- Evidence freshness SLO: 168 hours (last refresh: 2026-06-19T00:00:00Z)

No manual waivers are granted: a diagnostic row sits below its claim only by automatic narrowing when current, reopenable proof cannot back it.

## Auto-downgraded rows (1)

- **diag-cert:framework:stale-collection:0001** (framework_row): claim `certified` -> effective `uncertified`
  - Trigger: `stale_dimension_proof`
  - Collection snapshot aged outside its freshness window; held uncertified until a fresh enumeration re-backs the claim
  - Uncurrent dimensions: collection_snapshot
