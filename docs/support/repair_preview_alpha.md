# Repair Preview Alpha

This alpha path makes guided repair inspectable without granting it a
destructive apply engine. The support crate consumes the seed cases in
[`fixtures/support/repair_cases/`](../../fixtures/support/repair_cases/)
and the protected manifest in
[`fixtures/support/repair_preview_alpha/`](../../fixtures/support/repair_preview_alpha/)
to emit:

- `repair_transaction_record` from the checked-in seed case;
- `repair_preview_record` with impacted state, preserved state, lost
  capabilities, checkpoint proposal, reversal class, and confirmation
  requirement;
- `repair_outcome_record` with actor/source lineage and initiating
  diagnosis refs;
- `repair_mutation_journal_entry` for local-history and support joins;
- `repair_support_packet` metadata rows for export-safe review.

The implementation lives in
[`crates/aureline-support/src/repair`](../../crates/aureline-support/src/repair)
and is covered by the protected test
[`crates/aureline-support/tests/repair_transaction_preview_alpha.rs`](../../crates/aureline-support/tests/repair_transaction_preview_alpha.rs).

## Preview Truth

Every preview shows the blast radius before apply:

- `impacted_change_rows` list state classes the repair may mutate;
- `preserved_assertion_rows` list state classes the repair must not
  mutate, including `user_authored_files`;
- `lost_capability_classes` list temporary capability narrowing;
- `checkpoint_proposal` states whether a durable checkpoint exists or
  why no checkpoint is present;
- `claimed_reversal_class` states whether reversal is `exact`,
  `compensating`, `regenerate`, `manual`, or `audit_only`.

Non-exact repair paths are not authorized by an ordinary review click.
`compensating` and `manual` previews carry
`confirmation_class = strong_confirmation_required` and remain
`dry_run_complete_pending_review` until the stronger acknowledgement is
attached. Escalation-only repairs stay `audit_only` and never claim a
checkpoint or local apply path.

## Journaling

Outcomes carry `journal_lineage`, and every outcome receives a matching
`repair_mutation_journal_entry`. The journal binds the repair
transaction, preview, outcome, actor lineage, source lineage, checkpoint
refs, reversal class, mutation scope, and initiating Doctor findings.
Support bundles and local history can therefore explain what the product
believed, who approved the reviewed surface, what changed, and what
remained untouched.

## Verification

Run:

```bash
cargo test -p aureline-support --test repair_transaction_preview_alpha
```

The test covers the seed corpus, checkpoint visibility, reversal-class
truth, strong confirmation for non-exact repairs, pre-apply forbidden
boundary refusal, outcome journaling, and metadata-safe support export.
