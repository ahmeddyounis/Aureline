# Certified reference workspaces — fixture cases

Each JSON file in this directory is a negative case: a mutated copy of the
checked-in artifact that the typed model must reject.

| File | Violation exercised |
|---|---|
| `narrowing_row_not_narrowed.json` | A narrowed-stale matrix row with `effective_certified: true` |
| `held_with_active_downgrade.json` | A certified matrix row carrying an active downgrade reason |
| `published_wider_than_claimed.json` | A matrix row whose `effective_certified` is true while `claimed_certified` is false |
| `stale_report_not_narrowed.json` | An expired report whose `effective_state` is still `current` |
| `missing_rule_for_reason.json` | A downgrade rule set that drops the rule for `waiver_expired` |

The `cases.json` manifest lists every case so the integration test can iterate
over them without hard-coding file names.
