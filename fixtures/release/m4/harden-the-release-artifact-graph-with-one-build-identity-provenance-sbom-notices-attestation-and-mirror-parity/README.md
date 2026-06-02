# Release artifact graph — fixture cases

Each JSON file in this directory is a negative case: a mutated copy of the
checked-in artifact that the typed model must reject.

| File | Violation exercised |
|---|---|
| `narrowing_row_not_narrowed.json` | A narrowed-unbacked row with `effective_label` still matching `claim_label` |
| `held_with_active_gap.json` | A current row carrying an active gap reason |
| `missing_rule_for_reason.json` | A rule set that drops the rule for `waiver_expired` |
| `stale_packet_not_narrowed.json` | A narrowed-stale row whose `effective_label` still matches `claim_label` |
| `missing_family_kind.json` | A row set that drops the `mirror_parity` family kind |

The `cases.json` manifest lists every case so the integration test can iterate
over them without hard-coding file names.
