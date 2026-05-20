# maintenance-window & failover communication corpus

This is the release-engineering / public-proof audit lane for the marketed
managed and hybrid beta rows. It turns the maintenance, drain / read-only,
failover, and tenant / region migration *communication* model into a durable,
repeatable proof packet that release and support teams can attach to every
claimed row — one current maintenance/failover packet per row that a support
engineer can read and explain without rehydrating live control-plane state.

It composes — it does not fork — the `continuity_notice_view_record` model the
desktop shell, activity center / durable history, CLI / headless inspect,
diagnostics, and support exports all read (see
`crates/aureline-shell/src/continuity_notices/model.rs`). Each packet is a
complete `continuity_notice_view_record` validated against
`schemas/ops/continuity_notice_view.schema.json`.

It catches the quiet ways a continuity *communication* can lie:

- **Stale-as-current** — a maintenance card whose refresh aged out keeps reading
  as current operational truth.
- **Timezone ambiguity** — a window declared in a non-UTC zone renders as a naive
  local time, so an operator in another zone misreads when it happens.
- **Hidden boundary change** — a failover changed a tenant / region / endpoint
  and the recovered messaging hides it.
- **Silent replay across a changed authority** — queued intent auto-replays
  across a changed tenant / key-ownership / endpoint boundary instead of waiting
  for a boundary recheck.
- **Export drift** — a support bundle or CLI / headless projection loses the
  freshness, scope, boundary, or write-class semantics the product UI shows.

## What lives here

| File | What it is |
| ---- | ---------- |
| `timezone_mismatch_window.json` | Drill: a scheduled maintenance window in a non-UTC zone always shows its absolute UTC instant with the IANA zone and the offset in force. |
| `stale_maintenance_card_downgraded.json` | Drill: a maintenance card whose last refresh aged out downgrades, names the reason, lights the honesty marker, and carries a stale label. |
| `read_only_drain_window.json` | Drill: a drain lets existing sessions finish while new writes queue for publish-later or save as local drafts, kept separate from a hosted mutation that already landed. |
| `changed_endpoint_failover.json` | Drill: an emergency failover changed the region and endpoint identity; the change stays visible with a canonical current ref and affected publishes are held for a boundary review. |
| `queued_publish_later_preserved.json` | Drill: a read-only window preserves a queued publish-later and a local-draft intent with canonical refs, separate from a hosted approval that landed. |
| `post_window_reconciliation_changed_authority.json` | Drill: a reconciliation changed the tenant and key-ownership authority; queued intent is held for a boundary recheck and never silently replays across the new authority. |
| `corpus_matrix.json` | Enum-only matrix: one row per scenario with its claimed beta row, roll-ups, timezone, and the lane properties it proves; plus the claimed-row → packet map. |
| `export_parity_packet.json` | Per scenario, the support-bundle plaintext and CLI / headless index projections plus the semantic digest each one must preserve. |

## Scenarios

| Scenario | Notice kind | Effective freshness | Honesty | Preserved | Changed axes | Proves |
| -------- | ----------- | ------------------- | ------- | --------- | ------------ | ------ |
| `timezone_mismatch_window` | scheduled_maintenance_window | current | none | 1 | 0 | A window in `Pacific/Chatham` `+12:45` always shows its UTC instant, zone, and offset together. |
| `stale_maintenance_card_downgraded` | scheduled_maintenance_window | refresh_stale | present | 1 | 0 | A maintenance card whose refresh aged out downgrades and cannot read as current. |
| `read_only_drain_window` | drain_window | current | none | 2 | 0 | A drain finishes existing work while new writes queue or save locally, separate from a landed mutation. |
| `changed_endpoint_failover` | regional_failover | current | present | 0 | 2 | A changed region + endpoint stays visible and affected publishes are held for boundary review. |
| `queued_publish_later_preserved` | read_only_window | current | none | 2 | 0 | A queued publish-later and a local draft are preserved with canonical refs, separate from a landed approval. |
| `post_window_reconciliation_changed_authority` | post_event_reconciliation | current | present | 1 | 2 | A changed tenant / key-ownership authority holds queued intent for boundary review; it never silently replays. |

## Lane-failing invariants

The validator `ci/check_maintenance_failover_corpus.py` schema-validates every
packet against `schemas/ops/continuity_notice_view.schema.json`, rebuilds the
record from its extracted inputs with an independent port of the model and
asserts the stored record matches, then fails the lane when:

- **Stale-as-current.** A notice is `current` only when it is declared active and
  its last refresh is `fresh` / `recent`. A downgraded notice must name a
  downgrade reason, light the honesty marker, carry a stale label, and keep
  `stale_presented_as_current` false.
- **Changed boundary visible.** Every changed / unknown tenant, region,
  residency, key-ownership, or endpoint axis carries a canonical `current_ref`
  and `boundary_change_hidden` stays false — a changed endpoint can never hide
  behind recovered messaging.
- **No silent replay across a changed authority.** An auto-replay posture
  (`queued_publish_later`, `retryable_when_connected`) is never paired with a
  boundary-recheck resume trigger (`boundary_review_completed`,
  `fresh_approval_issued`); work that needs a boundary recheck is held as
  `blocked_pending_boundary_recheck` (or requires a manual rerun).
- **Queued work survives and stays separate.** Every queued publish-later and
  local-draft write carries a canonical queue / intent ref, is marked
  `intent_preserved`, and never shares an action class with a successful hosted
  mutation.
- **Timezone unambiguous.** Every schedule line carries the absolute UTC instant,
  the IANA zone, and the offset in force, so it cannot be misread as a naive
  local time.
- **Export parity.** The support-bundle plaintext preserves the full semantic
  digest and the CLI / headless index preserves the coarse digest of the product
  UI record.
- **Beta scorecard.** Every claimed managed / hybrid beta row maps to exactly one
  packet, and every packet maps back to exactly one claimed row.
- **Coverage.** The corpus proves every required drill: a downgraded stale
  notice, a changed endpoint, a changed tenant, a changed region, a drain window,
  a queued publish-later intent, a local-draft intent, a write held for boundary
  recheck, a post-window reconciliation under a changed authority, and a non-UTC
  timezone.

The matrix and parity packet are regenerated and drift-checked on every run, so a
regression in any roll-up, boundary count, posture, or export projection fails
the lane instead of shipping silently.

## Regenerate

```sh
python3 ci/check_maintenance_failover_corpus.py --write
```

This re-mints the six drill fixtures, `corpus_matrix.json`, and
`export_parity_packet.json` from the model. Run the full gate with
`scripts/ci/run_maintenance_failover_corpus.sh`.
