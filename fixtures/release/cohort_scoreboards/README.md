# Cohort scoreboards — negative fixtures

These fixtures back the negative-path coverage for the cohort scoreboards gate
(`ci/check_cohort_scoreboards.py`). Each JSON file is a **complete** cohort
scoreboards packet that is structurally valid except for one deliberately
introduced flaw. The gate loads every case named in `cases.json` and asserts the
validator rejects it with the expected `check_id`:

- `held_on_breached_packet.json` — a signed-off row rides a packet whose
  freshness-SLO state is `breached` (expected `row.held_on_stale_packet`).
- `narrowing_row_not_narrowed.json` — a narrowing row renders a Stable effective
  label instead of dropping below the cutline (expected `row.effective_not_narrowed`).
- `claim_label_ceiling_mismatch.json` — a row records a `claim_label` wider than
  the stable claim manifest publishes for its backing claim (expected
  `ceiling.claim_label_mismatch`).

Regenerate these fixtures from the canonical packet after the packet changes:
apply the single targeted mutation to a copy of
`artifacts/release/cohort_scoreboards.json` and recompute its summary and
publication blocks so that only the targeted check fires.
