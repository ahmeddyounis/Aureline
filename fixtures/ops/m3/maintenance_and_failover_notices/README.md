# maintenance & failover continuity-notice drill corpus

These fixtures back the M3 continuity-communication lane: scheduled maintenance,
read-only / drain windows, regional and control-plane failover, and tenant /
region migration. They are read by every surface that displays a continuity
notice — desktop shell, the activity center / durable history, CLI / headless
inspect, diagnostics, and support exports — so a single regression in the
no-silent-current downgrade rule, the queued-publish-later preservation, the
boundary-change derivation, or the display-copy invariants fails the corpus
instead of shipping silently.

Each fixture is a complete `continuity_notice_view_record` validated against
`schemas/ops/continuity_notice_view.schema.json`. The view composes — it does
not fork — the upstream `maintenance_notice_record`,
`tenant_migration_event_record`, `failover_banner_record`, and
`local_safe_baseline_record` boundary records frozen in
`docs/ops/maintenance_migration_failover_contract.md` and
`docs/ops/failover_continuity_banner_contract.md`. The fixtures are minted by the
`aureline_shell_continuity_notices_corpus` emitter from the in-code corpus at
`crates/aureline-shell/src/continuity_notices/corpus.rs`, and the fixture-replay
test at `crates/aureline-shell/tests/continuity_notices_fixtures.rs` asserts the
disk content matches the in-code projection bit-for-bit.

Regenerate after any change to the corpus or model:

```sh
cargo run -q -p aureline-shell \
  --bin aureline_shell_continuity_notices_corpus -- emit-fixtures \
  fixtures/ops/m3/maintenance_and_failover_notices
```

## Drills

| Scenario id | Category | Effective freshness | Honesty | Preserved intents | Changed axes | What it proves |
| ----------- | -------- | ------------------- | ------- | ----------------- | ------------ | -------------- |
| `scheduled_maintenance_window` | maintenance | `current` | none | 1 | 0 | A planned window declares its exact time, timezone, and offset; review comments queue for publish-later and a settings sync is retryable. |
| `read_only_window_publish_later` | drain | `current` | none | 2 | 0 | A read-only window preserves a queued publish-later and a local-draft intent, kept visibly separate from a hosted mutation that already landed. |
| `drain_before_failover` | drain | `current` | present | 1 | 0 | A drain lets existing sessions finish while new writes queue; the region is unknown-recheck, so the boundary change is unresolved and postponing is safer. |
| `scheduled_export_freeze` | maintenance | `current` | none | 0 | 0 | A support upload has no safe retry across the freeze (export now is safer); a telemetry upload stays retryable. |
| `regional_failover_changed_boundary` | failover | `current` | present | 0 | 2 | An emergency failover changed the region and endpoint; publishes wait for a boundary recheck and an in-flight write needs a manual rerun. |
| `tenant_migration_new_region` | tenant_migration | `current` | present | 1 | 3 | A tenant migration changed tenant, region, and residency; a publish is held as a local draft until the new boundary is reviewed. |
| `control_plane_failover` | failover | `current` | present | 0 | 0 | A control-plane failover left key ownership unknown-recheck; remote sessions block pending reconnect, an AI prompt is retryable, refresh is recent. |
| `region_migration_reconciling` | tenant_migration | `current` | present | 1 | 2 | A region migration is reconciling with a queued replay; region and residency changed. |
| `post_event_reconciliation_completed` | maintenance | `completed_historical` | present | 0 | 1 | A completed reconciliation keeps the changed tenant identity visible and cannot read as current. |
| `superseded_notice_downgraded` | maintenance | `superseded_stale` | present | 0 | 0 | A superseded notice downgrades even with a fresh refresh and points at its replacement. |
| `imported_offline_history` | maintenance | `imported_historical` | present | 0 | 0 | An air-gapped imported notice has no live refresh and is labeled imported history. |
| `stale_refresh_active_downgraded` | drain | `refresh_stale` | present | 1 | 0 | A still-active read-only window whose last refresh aged out downgrades to stale and names why. |

## Invariants the replay test enforces

- **No silent current.** `effective_freshness == current` only when the declared
  lifecycle freshness is `active_current` and the last refresh is `fresh` or
  `recent` relative to `as_of`. A superseded, completed, imported, or
  refresh-aged-out notice downgrades, names the `downgrade_reasons`, lights the
  honesty marker, and carries a non-null stale label — it never reads as live.
- **Queued work survives and stays separate.** Every queued publish-later and
  local-draft write carries a canonical queue / intent ref and is marked
  `intent_preserved`; successful hosted mutations live in a separate list so
  survived queued work is never collapsed into work that landed.
- **Boundary preserved after recovery.** A changed or unknown-recheck tenant /
  region / residency / key-ownership / endpoint axis carries a canonical
  `current_ref` and stays visible even on a completed / recovered notice;
  `boundary_change_unresolved` lights the honesty marker until reviewed.
- **No generic-degraded collapse.** Every notice names its category
  (maintenance, drain, failover, or tenant-migration), and the display copy
  invariants (`all_work_broken_implied`, `incident_language_for_planned_used`,
  `generic_degraded_banner_used`, `queued_and_succeeded_collapsed`,
  `stale_presented_as_current`, `boundary_change_hidden`) all stay false.
- **Cross-surface agreement.** Desktop, the activity center / durable history,
  CLI / headless inspect, diagnostics, and support exports replay the same
  record, so they agree on the window, scope, write classes, boundary, and
  freshness for the same notice.
