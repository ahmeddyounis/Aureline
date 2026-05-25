# Open/paid boundary audit — negative fixtures

These fixtures back the negative-path coverage for the open/paid boundary audit gate
(`ci/check_open_paid_boundary_audit.py`). Each JSON file is a **complete** open/paid
boundary audit that is structurally valid except for one deliberately introduced flaw.
The gate loads every case named in `cases.json` and asserts the validator rejects it
with the expected `check_id`:

- `held_on_breached_packet.json` — an attested row rides an attestation packet whose
  freshness-SLO state is `breached` (expected `row.held_on_stale_packet`).
- `narrowing_row_not_narrowed.json` — a narrowing row renders a Stable effective label
  instead of dropping below the cutline (expected `row.effective_not_narrowed`).
- `claim_label_ceiling_mismatch.json` — a row records a `claim_label` wider than the
  stable claim manifest publishes for its backing claim (expected
  `ceiling.claim_label_mismatch`).

Regenerate these fixtures from the canonical audit after the audit changes: apply the
single targeted mutation to a copy of
`artifacts/release/open_paid_boundary_audit.json` and recompute its summary and
publication blocks so that only the targeted check fires.
