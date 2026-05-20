# Maintenance & failover continuity-notice audit report

Reviewer-facing summary of the continuity-notice audit lane. Regenerate the
machine-readable findings with:

```sh
scripts/ci/run_continuity_notices.sh \
  --report-json artifacts/ops/m3/maintenance_failover_truth_corpus_report.json
```

- **View record:** `continuity_notice_view_record`
- **Schema:** `schemas/ops/continuity_notice_view.schema.json`
- **Corpus (code):** `crates/aureline-shell/src/continuity_notices/corpus.rs`
- **Corpus (fixtures):** `fixtures/ops/m3/maintenance_and_failover_notices/`
- **Replay test:** `crates/aureline-shell/tests/continuity_notices_fixtures.rs`
- **Validator:** `ci/check_continuity_notices.py`
- **Contract:** `docs/ops/m3/maintenance_failover_truth.md`

## What the lane proves

1. **No silent current.** A notice reads as `current` only while it is declared
   active and its last refresh is fresh / recent; superseded, completed,
   imported, and refresh-aged-out notices downgrade, name a reason, light the
   honesty marker, and carry a stale label.
2. **Queued work survives and stays separate.** Queued publish-later and
   local-draft writes carry a canonical queue / intent ref and are kept in a
   separate list from successful hosted mutations.
3. **Boundary preserved after recovery.** A changed / unknown tenant, region,
   residency, key-ownership, or endpoint axis carries a canonical current ref
   and stays visible even on a completed notice.
4. **No generic-degraded collapse.** Every notice names its category and keeps
   the display-copy no-lie invariants false.

## Drill index

| Scenario | Category | Effective freshness | Honesty | Preserved | Changed axes | Boundary unresolved |
| -------- | -------- | ------------------- | ------- | --------- | ------------ | ------------------- |
| `scheduled_maintenance_window` | maintenance | current | none | 1 | 0 | false |
| `read_only_window_publish_later` | drain | current | none | 2 | 0 | false |
| `drain_before_failover` | drain | current | present | 1 | 0 | true |
| `scheduled_export_freeze` | maintenance | current | none | 0 | 0 | false |
| `regional_failover_changed_boundary` | failover | current | present | 0 | 2 | true |
| `tenant_migration_new_region` | tenant_migration | current | present | 1 | 3 | true |
| `control_plane_failover` | failover | current | present | 0 | 0 | true |
| `region_migration_reconciling` | tenant_migration | current | present | 1 | 2 | true |
| `post_event_reconciliation_completed` | maintenance | completed_historical | present | 0 | 1 | false |
| `superseded_notice_downgraded` | maintenance | superseded_stale | present | 0 | 0 | false |
| `imported_offline_history` | maintenance | imported_historical | present | 0 | 0 | false |
| `stale_refresh_active_downgraded` | drain | refresh_stale | present | 1 | 0 | false |

## Coverage

The corpus exercises every notice kind (9), category (4), effective freshness
(5), write-continuity posture (8), downgrade reason (4), and boundary-axis state
(4).
