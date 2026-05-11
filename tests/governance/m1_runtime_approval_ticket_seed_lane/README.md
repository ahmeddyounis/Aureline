# M1 runtime approval-ticket seed validation lane

Unattended Python lane that replays
`artifacts/runtime/m1_runtime_approval_ticket_seed.yaml` against:

- `schemas/runtime/m1_runtime_approval_ticket_seed.schema.json` —
  envelope schema (vocabularies, required coverage, named consumers);
- `schemas/runtime/approval_ticket.schema.json` — row vocabulary +
  conditional invariants;
- the reviewer-facing landing page at
  `docs/runtime/m1_authority_model.md`; and
- the upstream governance authority-ticket contract at
  `docs/governance/runtime_authority_contract.md` and the upstream
  provider-plane approval-ticket schema at
  `schemas/integration/approval_ticket.schema.json`.

The lane is the seed's first live consumer. It is invoked as:

```
python3 tests/governance/m1_runtime_approval_ticket_seed_lane/run_m1_runtime_approval_ticket_seed_lane.py --repo-root .
```

The runner writes the durable JSON capture to
`artifacts/milestones/m1/captures/runtime_approval_ticket_seed_validation_capture.json`.

Use `--force-drill
<approval_ticket_profile_id>:<drill_id>` to replay a named failure
drill on a named row's example payload; the runner exits 0 only when
the drill reproduces the row's declared `expected_check_id`.
