# Final go/no-go rehearsal — negative fixtures

These fixtures back the negative-path coverage for the final go/no-go rehearsal gate
(`ci/check_go_no_go_rehearsal.py`). Each JSON file is a **complete** go/no-go rehearsal
that is structurally valid except for one deliberately introduced flaw. The gate loads
every case named in `cases.json` and asserts the validator rejects it with the expected
`check_id`:

- `go_on_breached_packet.json` — a Go stage rides a rehearsal packet whose freshness-SLO
  state is `breached` (expected `row.go_on_stale_packet`).
- `narrowing_row_not_narrowed.json` — a No-Go stage renders a Stable effective label
  instead of dropping below the cutline (expected `row.effective_not_narrowed`).
- `claim_label_ceiling_mismatch.json` — a stage records a `claim_label` wider than the
  stable claim manifest publishes for its backing claim (expected
  `ceiling.claim_label_mismatch`).

Regenerate these fixtures from the canonical rehearsal after the rehearsal changes: apply
the single targeted mutation to a copy of `artifacts/release/go_no_go_rehearsal.json` and
recompute its summary and publication blocks so that only the targeted check fires.
