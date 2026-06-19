# M5 Quality-Session Proof

`support_export.json` is the checked support export of the M5 quality-session
ledger (`QualitySessionLedgerPacket`). It is the canonical artifact downstream
editor, Problems, review, CLI, and support surfaces ingest through
`aureline_runtime::m5_quality_action_proposals_and_sessions::current_m5_quality_session_ledger_export`
instead of forking per-surface mutation status.

The ledger proves that **every mutating quality action** on the claimed M5
surfaces is represented as a **typed proposal** (`QualityActionProposal`) and
serialized inside a **typed quality session** (`QualitySession`), and that the
on-type, on-save, manual, headless, review-apply, and imported-scan comparison
paths all report through **one result vocabulary** rather than divergent
per-provider status text.

It carries eight sessions that span every required trigger path, every action
class, every safety class, and a representative spread of outcomes:

- **On-type / notebook** — a trivia-safe `format_range` is auto-applied, yet still
  recorded as a typed proposal inside a session (`applied`).
- **On-save / framework** — `format_document` plus `organize_imports` route
  through preview-first (`preview_required`).
- **Manual / request + data** — a local `quick_fix_single` and a cross-file
  `fix_all_rule` require a preview before apply (`preview_required`).
- **Headless / package** — a localized, syntax-safe `lint_autofix_batch` is
  auto-applied in headless mode using the same vocabulary as the interactive
  paths (`applied`).
- **Review-apply / governance** — a `suppression_proposal` and a `baseline_update`
  are policy-bearing and held `blocked_by_policy`.
- **Import comparison / scanner** — `scanner_read_only` and `validation_recheck`
  compare an imported snapshot against the local revision and stay read-only;
  never a local apply (`applied`, zero mutating proposals).
- **Generated / protected** — a regenerated artifact family including a lockfile
  and manifest reuses the **same preview/apply/validate/revert lifecycle** rather
  than a weaker bar because it "looks like formatting" (`preview_required`).
- **Unknown / unstable** — a provider-ambiguous quick fix is held for user review,
  not silently applied (`failed`).

Each session names its trigger, effective profile, execution context, started and
ended times, outcome, validation refs, and rollback refs; each proposal carries an
explicit scope, safety class, preview requirement, checkpoint ref, and rollback
boundary. The editor (save-participant UI), Problems, review, CLI, and support
surfaces each receive a projection that exposes the outcome, the safety classes in
play, the rollback-boundary note, and the validation refs, and the support export
preserves each session's ordered proposal trail rather than a lossy display-only
row.

`support_export.md` is the deterministic Markdown summary of the same packet.

## Regenerate

```bash
cargo run -p aureline-runtime --example dump_m5_quality_session_ledger > \
  artifacts/m5/diagnostics/quality-session-proof/support_export.json
cargo run -p aureline-runtime --example dump_m5_quality_session_ledger summary > \
  artifacts/m5/diagnostics/quality-session-proof/support_export.md
cp artifacts/m5/diagnostics/quality-session-proof/support_export.json \
  fixtures/quality/m5/quality-actions-and-sessions/quality_session_ledger.json
```

The artifact validates against
[`schemas/quality/quality-session-ledger.schema.json`](../../../../schemas/quality/quality-session-ledger.schema.json)
(whose `sessions` resolve to
[`schemas/quality/quality_session.schema.json`](../../../../schemas/quality/quality_session.schema.json)
and
[`schemas/quality/quality_action_proposal.schema.json`](../../../../schemas/quality/quality_action_proposal.schema.json))
and is byte-identical to the protected fixture at
[`fixtures/quality/m5/quality-actions-and-sessions/quality_session_ledger.json`](../../../../fixtures/quality/m5/quality-actions-and-sessions/quality_session_ledger.json).
