# Maintenance-window & failover communication corpus report

Reviewer-facing summary of the maintenance-window and failover communication
audit lane — the release-engineering / public-proof packet that proves the
claimed managed and hybrid beta rows keep maintenance, drain, failover, and
tenant-migration communication honest under real service change. Regenerate the
machine-readable findings with:

```sh
scripts/ci/run_maintenance_failover_corpus.sh \
  --report-json artifacts/ops/m3/maintenance_failover_corpus_report.json
```

- **View record:** `continuity_notice_view_record`
- **Schema:** `schemas/ops/continuity_notice_view.schema.json`
- **Corpus:** `fixtures/ops/m3/maintenance_failover_corpus/`
- **Matrix:** `fixtures/ops/m3/maintenance_failover_corpus/corpus_matrix.json`
- **Parity packet:** `fixtures/ops/m3/maintenance_failover_corpus/export_parity_packet.json`
- **Validator:** `ci/check_maintenance_failover_corpus.py`
- **Script:** `scripts/ci/run_maintenance_failover_corpus.sh`
- **Contract:** `docs/ops/m3/maintenance_failover_truth.md`

## What the lane proves

1. **No stale-as-current.** A notice reads as `current` only while it is declared
   active and its last refresh is fresh / recent; an aged-out maintenance card
   downgrades, names a reason, lights the honesty marker, and carries a stale
   label.
2. **Timezone is unambiguous.** Every scheduled window shows its absolute UTC
   instant alongside the IANA zone and the offset in force, so it cannot be
   misread as a naive local time.
3. **Changed boundary stays visible.** A changed tenant / region / endpoint /
   key-ownership axis carries a canonical current ref and is never hidden behind
   recovered messaging.
4. **No silent replay across a changed authority.** Queued intent that would
   cross a changed authority boundary is held for a boundary recheck — an
   auto-replay posture is never paired with a boundary-recheck resume trigger.
5. **Queued work survives and stays separate.** Queued publish-later and
   local-draft writes carry a canonical queue / intent ref and stay separate from
   successful hosted mutations.
6. **Export parity.** Support-bundle plaintext and the CLI / headless index
   preserve the same semantics as the product UI record, so support can explain
   outcomes without rehydrating live control-plane state.

## Drill index

| Scenario | Claimed beta row | Notice kind | Effective freshness | Honesty | Preserved | Changed axes | Boundary unresolved |
| -------- | ---------------- | ----------- | ------------------- | ------- | --------- | ------------ | ------------------- |
| `timezone_mismatch_window` | `beta.row.managed_cloud.eu_review_collab` | scheduled_maintenance_window | current | none | 1 | 0 | false |
| `stale_maintenance_card_downgraded` | `beta.row.managed_cloud.registry_maintenance` | scheduled_maintenance_window | refresh_stale | present | 1 | 0 | false |
| `read_only_drain_window` | `beta.row.managed_cloud.merge_queue_drain` | drain_window | current | none | 2 | 0 | false |
| `changed_endpoint_failover` | `beta.row.managed_cloud.regional_failover` | regional_failover | current | present | 0 | 2 | true |
| `queued_publish_later_preserved` | `beta.row.hybrid.read_only_publish_later` | read_only_window | current | none | 2 | 0 | false |
| `post_window_reconciliation_changed_authority` | `beta.row.managed_cloud.tenant_reconciliation` | post_event_reconciliation | current | present | 1 | 2 | true |

## Beta scorecard

Every claimed managed / hybrid beta row maps to exactly one current
maintenance/failover packet, and every packet maps back to exactly one claimed
row (6 rows, 6 packets). The matrix records the claimed-row → packet map; the
parity packet carries a self-contained support bundle per row so a support team
can explain the maintenance/failover outcome from the exported packet alone.

## Coverage

The corpus proves every required communication drill: a downgraded stale
maintenance card, a timezone-mismatch window (non-UTC offset), a read-only /
drain window, a changed endpoint, a changed tenant, a changed region, a queued
publish-later intent, a local-draft intent, a write held for a boundary recheck,
and a post-window reconciliation under a changed tenant / key-ownership
authority. The enum-only matrix and the export-parity packet are regenerated and
drift-checked on every run.
