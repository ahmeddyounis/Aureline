# Proof packet: M04-183 — Finalize release-packet freshness SLOs, shiproom dashboards, and proof-index export for procurement and support

**Register**: `finalize:m4:freshness_slos_shiproom_dashboards_proof_index_export`  
**As of**: 2026-06-03  
**Decision**: HOLD

## Summary

| Metric | Value |
|--------|-------|
| Total entries | 11 |
| Entries current stable | 6 |
| Entries narrowed below cutline | 5 |
| Entries on active waiver | 1 |
| Release-blocking total | 5 |
| Release-blocking narrowed | 2 |
| Claim-publication-manifest entries | 1 |
| Reference-workspace-report entries | 1 |
| Compatibility-report entries | 1 |
| Evaluation-evidence-pack entries | 1 |
| Pilot-evidence-pack entries | 1 |
| Help/About-consumer entries | 1 |
| Support-export-consumer entries | 1 |
| Build-packet entries | 1 |
| Benchmark-packet entries | 1 |
| Shiproom-dashboard-panel entries | 1 |
| Proof-index-export-object entries | 1 |
| Packets current | 7 |
| Packets due for refresh | 1 |
| Packets breached | 2 |
| Packets missing | 1 |
| Rules firing | 3 |

## Rows

- **freshness:claim_manifest** — Claim publication manifest (`claim_publication_manifest`) → `stable` 🟢
  - Claim ref: `claim:core` (`stable`)
  - Owner: `team:release` (signed: True)

- **freshness:ref_workspace** — Reference workspace report (`reference_workspace_report`) → `stable` 🟡 (on waiver)
  - Claim ref: `claim:workspace` (`stable`)
  - Owner: `team:workspace` (signed: True)

- **freshness:compat_report** — Compatibility report (`compatibility_report`) → `beta` 🔴
  - Gap reasons: `proof_packet_freshness_breached`
  - Claim ref: `claim:compat` (`stable`)
  - Owner: `team:compat` (signed: False)

- **freshness:eval_pack** — Evaluation evidence pack (`evaluation_evidence_pack`) → `stable` 🟢
  - Claim ref: `claim:eval` (`stable`)
  - Owner: `team:eval` (signed: True)

- **freshness:pilot_pack** — Pilot evidence pack (`pilot_evidence_pack`) → `beta` 🔴
  - Gap reasons: `claim_label_narrowed`
  - Claim ref: `claim:pilot` (`beta`)
  - Owner: `team:pilot` (signed: True)

- **freshness:help_about** — Help/About consumer (`help_about_consumer`) → `stable` 🟢
  - Claim ref: `claim:help_about` (`stable`)
  - Owner: `team:docs` (signed: True)

- **freshness:support_export** — Support export consumer (`support_export_consumer`) → `preview` 🔴
  - Gap reasons: `proof_packet_missing`, `evidence_incomplete`
  - Claim ref: `claim:support` (`stable`)
  - Owner: `team:support` (signed: False)

- **freshness:build_packet** — Build packet (`build_packet`) → `stable` 🟢
  - Claim ref: `claim:build` (`stable`)
  - Owner: `team:build` (signed: True)

- **freshness:benchmark_packet** — Benchmark packet (`benchmark_packet`) → `stable` 🟢
  - Claim ref: `claim:benchmark` (`stable`)
  - Owner: `team:perf` (signed: True)

- **freshness:shiproom_panel** — Shiproom dashboard panel (`shiproom_dashboard_panel`) → `beta` 🔴
  - Gap reasons: `proof_packet_freshness_breached`, `stale_report_alarm`
  - Claim ref: `claim:shiproom` (`stable`)
  - Owner: `team:shiproom` (signed: False)

- **freshness:proof_index_export** — Proof index export object (`proof_index_export_object`) → `stable` 🟢
  - Claim ref: `claim:proof_index` (`stable`)
  - Owner: `team:release` (signed: True)

## Publication verdict

- **Gate**: `m4:stable_promotion`
- **Decision**: HOLD
- **Blocking rules**: rule:packet_freshness_breached, rule:packet_missing
- **Blocking entries**: freshness:compat_report, freshness:shiproom_panel, freshness:support_export
- **Rationale**: Compatibility report, shiproom panel, and support export are stale or missing, blocking stable promotion.
