# Continuity drill-freshness report

Human-readable companion to the canonical certified-row registry at
`artifacts/m5/continuity/certification/certified_rows.json`. It summarizes the
per-row backup/restore/failover drill and continuity-proof freshness posture that
the certification verdict depends on. The authoritative freshness SLO dashboard
lives at `artifacts/m5/continuity/freshness_slo_dashboard.json`; this report
records how that freshness feeds the certification lane.

## Why drill freshness gates certification

No claimed row may keep a `stable` certification once its latest drill packet or
continuity summary is out of its freshness SLO or lacks restore-identity /
partial-loss disclosure. The certification report carries two drill-bearing
dimensions per scope row:

- `backup_restore_failover` — is there a **current** backup/restore/failover
  drill backing the claim, with restore identity and partial-loss disclosed?
- `drill_freshness_slo` — is the backing continuity proof packet **within its
  freshness SLO** (mirrors the freshness-SLO dashboard verdict)?

When either is `stale`/`partial` the row narrows to `beta`; when either is
`missing` the row narrows to `preview`. The local-core lane is never held to
managed drill freshness.

## Per-row drill and freshness posture (as of 2026-06-19)

| Row | Drill evidence | Drill state | Freshness state | Restore identity / partial loss |
| --- | --- | --- | --- | --- |
| Managed cloud workspace sync and backup | `drill:managed-cloud:backup:2026-06-01` | current | current | same-identity restore; bounded recent-window loss |
| Managed relay and collaboration failover | `drill:managed-relay:failover:2026-05-20` | current | due-soon (within SLO) | same-identity failover; queued-action replay |
| Customer self-hosted restore and rebuild | `drill:self-hosted:restore:2026-05-01` | current | current | reissued-identity restore; bounded recent-window loss |
| Sovereign air-gapped snapshot and replication | `drill:sovereign:snapshot:2026-05-15` | current | current | new-install rebind; cache-only loss |
| Local desktop core continuity (local-core lane) | `freshness:local-core:autosave:current` | n/a | current | local autosave/Git; no partial loss |

Each certification-scope row references a **distinct** backup/restore/failover
drill: no single reference-environment drill stands in for more than one claimed
profile.

## Stale-evidence behaviour

The fixtures under `fixtures/continuity/certification_cases/` exercise the
narrowing paths the certification gate enforces:

- `case_backup_drill_stale_narrows.json` — a stale backup drill narrows the
  managed row to `beta`.
- `case_freshness_breached_narrows.json` — a breached freshness SLO narrows the
  managed relay row.
- `case_restore_identity_missing_narrows.json` — missing restore-identity
  disclosure narrows the self-hosted row to `preview`.
- `case_mirror_offline_missing_narrows.json` — a missing offline-continuity
  packet narrows the sovereign air-gapped row to `preview`.
- `case_profile_mismatch_withdrawn.json` — locality contradicting the sovereign
  profile withdraws the claim.
- `case_local_core_stays_certified.json` — a managed row goes missing, yet the
  local-core lane stays certified.
