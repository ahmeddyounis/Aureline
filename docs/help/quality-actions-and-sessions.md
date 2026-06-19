# Quality Actions and Quality Sessions

When Aureline formats your code, organizes imports, applies a quick fix, fixes
every occurrence of a rule, runs a lint autofix, or updates a suppression or
baseline, it does **not** silently mutate your work and report a one-off,
tool-specific status string. Every mutating quality action is first described as a
typed **quality-action proposal** — with an explicit scope, safety class, preview
requirement, checkpoint, and rollback boundary — and that proposal is serialized
inside a typed **quality session** that records what triggered it, which effective
quality profile governed it, when it ran, how it ended, what was validated, and how
to undo it.

This page describes the M5 quality-session ledger, which proves that the
format-on-type, format-on-save, manual quick-fix and fix-all, headless lint
autofix, review-apply baseline/suppression, and imported-scan comparison routes all
flow through that one contract, using one shared result vocabulary rather than a
per-provider status string per surface.

- Record kind: `m5_quality_session_ledger`
- Packet type: `QualitySessionLedgerPacket`
  (`crates/aureline-runtime/src/m5_quality_action_proposals_and_sessions/`)
- Proposal schema: [`schemas/quality/quality_action_proposal.schema.json`](../../schemas/quality/quality_action_proposal.schema.json)
- Session schema: [`schemas/quality/quality_session.schema.json`](../../schemas/quality/quality_session.schema.json)
- Ledger schema: [`schemas/quality/quality-session-ledger.schema.json`](../../schemas/quality/quality-session-ledger.schema.json)
- Checked support export: [`artifacts/m5/diagnostics/quality-session-proof/support_export.json`](../../artifacts/m5/diagnostics/quality-session-proof/support_export.json)
- Summary artifact: [`artifacts/m5/diagnostics/quality-session-proof/support_export.md`](../../artifacts/m5/diagnostics/quality-session-proof/support_export.md)
- Fixtures: [`fixtures/quality/m5/quality-actions-and-sessions/`](../../fixtures/quality/m5/quality-actions-and-sessions/)
- Loader: `aureline_runtime::m5_quality_action_proposals_and_sessions::current_m5_quality_session_ledger_export`
- Conformance dump: `cargo run -p aureline-runtime --example dump_m5_quality_session_ledger`

## The quality-action proposal

A proposal is built before any mutation runs. Beyond the action itself it carries:

| Field | Meaning |
| --- | --- |
| Action class | `format_range`, `format_document`, `organize_imports`, `quick_fix_single`, `fix_all_rule`, `lint_autofix_batch`, `suppression_proposal`, `baseline_update`, `scanner_read_only`, or `validation_recheck`. |
| Safety class | How risky the change is: `trivia_safe`, `local_syntax_safe`, `semantic_local`, `cross_file_semantic`, `generated_or_protected`, or `unknown_or_unstable`. |
| Mutation scope | How much it touches, from `no_mutation_read_only` up to `multi_file_workspace`, `generated_family`, and `protected_or_policy_scoped`. |
| Preview requirement | Whether apply must route through preview or typed review first. |
| Apply posture | Whether apply is allowed, preview-gated, or blocked pending user review, policy, or trust. |
| Checkpoint & rollback boundary | The checkpoint that anchors the change and how far an undo reaches — `current_buffer_undo`, `single_file_checkpoint`, `grouped_workspace_checkpoint`, `policy_audit_only`, or `manual_recovery_required`. |
| Validation refs | The checks (build, type-check, lint recheck, policy check) run after apply. |

The safety class, scope, and policy posture **derive** the preview requirement,
apply posture, and rollback boundary, so a riskier change cannot quietly claim an
easier path. A read-only action (`scanner_read_only`, `validation_recheck`) carries
no mutation and `no_mutation` rollback.

## The quality session

A session binds one trigger to the proposals it considered:

| Trigger | Path |
| --- | --- |
| `on_type` | As-you-edit fast pass. |
| `on_save` | On-save participant pipeline. |
| `manual_command` | A manual desktop command. |
| `cli_headless` | A CLI or headless run. |
| `review` | A review packet or batch preview (review-apply). |
| `import_comparison` | Comparing an imported scan or replay against a later local revision. |

Every session reports a single typed **outcome** — `applied`, `preview_required`,
`skipped`, `timed_out`, `rebase_required`, `blocked_by_policy`, `failed`, or
`reverted` — drawn from the same vocabulary regardless of which path triggered it.
An on-type format, an on-save organize-imports, a headless lint batch, and a
review-apply baseline update all speak that one vocabulary, so the editor,
Problems, the review surface, the CLI, and a support export never disagree about
what ran or why.

## Four guarantees

1. **Every mutating action is a typed proposal inside a typed session.** A mutation
   is never a bare side effect — it is a proposal with an explicit scope, safety
   class, preview requirement, checkpoint, and rollback boundary, serialized inside
   a session.
2. **One result vocabulary across every path.** On-type, on-save, manual, headless,
   review-apply, and import-comparison sessions all report through the same typed
   outcome and class tokens, never a divergent per-provider status string.
3. **Generated, lockfile, manifest, and protected paths reuse the same lifecycle.**
   A change to a generated family, a lockfile, a manifest, or a protected path
   cannot claim a weaker mutation bar because it "looks like formatting": it must
   require preview-first or block apply, and it carries a real rollback boundary.
4. **Rollback notes, validation refs, and safety classes stay inspectable.** Every
   required surface — the save-participant UI, Problems, review, CLI, and support
   export — receives a projection that exposes the outcome, the safety classes in
   play, the rollback-boundary note, and the validation refs.

## What the validator refuses

`QualitySessionLedgerPacket::validate` rejects a ledger that omits a required
trigger path or action class, lets a serialized result token diverge from its typed
class, grants a generated or protected mutation a weaker bar (auto-apply), drops a
mutating proposal's rollback boundary, lets an import-comparison session mutate,
hides the proposal or session truth from a required surface, or serializes a lossy
or raw-content-bearing support export. Raw patches, raw source text, raw tool
arguments, raw provider payloads, credentials, and raw logs never cross this
boundary.
