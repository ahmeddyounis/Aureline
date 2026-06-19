# M5 quality-action and quality-session fixtures

`quality_session_ledger.json` is the protected fixture corpus for the M5
quality-session ledger (`QualitySessionLedgerPacket`). It is byte-identical to the
checked support export at
[`artifacts/m5/diagnostics/quality-session-proof/support_export.json`](../../../../artifacts/m5/diagnostics/quality-session-proof/support_export.json)
and validates against
[`schemas/quality/quality-session-ledger.schema.json`](../../../../schemas/quality/quality-session-ledger.schema.json),
whose embedded sessions and proposals resolve to
[`schemas/quality/quality_session.schema.json`](../../../../schemas/quality/quality_session.schema.json)
and
[`schemas/quality/quality_action_proposal.schema.json`](../../../../schemas/quality/quality_action_proposal.schema.json).

The fixture exercises every required trigger path — on-type, on-save, manual,
headless, review-apply, and import-comparison — every action class — format-range,
format-document, organize-imports, quick-fix, fix-all-rule, lint-autofix-batch,
suppression, baseline-update, scanner-read-only, and validation-recheck — every
safety class, and a representative spread of outcomes, and proves that:

- every mutating quality action is a typed proposal serialized inside a typed
  quality session, with an explicit scope, safety class, preview requirement,
  checkpoint ref, and rollback boundary;
- on-type, on-save, manual, headless, review-apply, and import-comparison sessions
  all report through one typed result vocabulary rather than divergent
  per-provider status text;
- generated, lockfile, manifest, and protected paths reuse the same
  preview/apply/validate/revert lifecycle instead of a weaker bar; and
- the support export preserves each session's ordered proposal trail rather than a
  lossy display-only row.

## Regenerate

```bash
cargo run -p aureline-runtime --example dump_m5_quality_session_ledger > \
  fixtures/quality/m5/quality-actions-and-sessions/quality_session_ledger.json
```

The in-crate builder, the checked artifact, and this fixture are kept in lockstep
by the unit tests in
`crates/aureline-runtime/src/m5_quality_action_proposals_and_sessions/tests.rs`.
