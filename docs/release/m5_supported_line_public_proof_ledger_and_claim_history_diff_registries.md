# M5 public-proof-ledger and claim-history-diff registries

This lane makes durable external proof operable over the frozen
[M5 supported-line-transparency matrix](./m5-supported-line-transparency-ops.md). It publishes one line-by-line
public-proof ledger per active stable or LTS-candidate line so external claims inherit current rather than tribal
truth, and emits a typed claim-history diff whenever the proof backing a line changes — instead of letting a stale
compatibility report, benchmark packet, support-window statement, known-limits set, or deprecation asset drift into
an implicit docs mismatch. It records the *public-proof-ledger* grammar (the ledger published per active supported
line — one typed section per joined proof source: a compatibility report, a benchmark / evidence packet, a
support-window statement, a known-limits set, a deprecation report, and a successor report — each bound to one
supported-line identity with its freshness state, last-versus-current diff, and the exact evidence-packet refs
currently backing its public claims) and the *claim-history-diff* grammar (the typed diff event naming whether a
proof source changed freshness or moved from current to retest-pending, narrowed the scope it backs, or changed the
release-line identity it is associated with, naming the active diff reason) into registry resolvers that produce
export-safe, honest projections, so release / help, About, docs, support, and procurement surfaces resolve one
canonical, freshness-checked truth instead of re-synthesizing product truth by hand. The ledger and the diff are
separated in runtime and serialized state: the joined proof source, affected rows, linked evidence-packet refs,
rollback target, and proof posture live on the public-proof-ledger entry, while the resolved line identity, affected
proof-source reference, previous-versus-current claim-state reference, diff-scope state, and active diff reason live
on the claim-history-diff entry, and a line's rollback posture stays preserved so onboarding / migration / support
language never runs ahead of current public proof.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_supported_line_public_proof_ledger_and_claim_history_diff_registries` (the
  authoritative validator).
- **Combined schema:**
  `schemas/program/m5-supported-line-public-proof-ledger-and-claim-history-diff-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-public-proof-freshness-ledger.schema.json`](../../schemas/program/m5-public-proof-freshness-ledger.schema.json)
  (reused from the frozen matrix — the public-proof freshness ledger each joined proof source is recorded against)
  and
  [`schemas/program/m5-claim-history-diff.schema.json`](../../schemas/program/m5-claim-history-diff.schema.json)
  (minted by this lane) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-supported-line-public-proof-ledger-and-claim-history-diff-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`). This checked-in proof is the first line-by-line public-proof
  ledger — it demonstrates one durable external-proof loop end to end for at least one active supported line.
- **Narrowed fixtures:**
  `fixtures/release/m5-supported-line-public-proof-ledger-and-claim-history-diff-registries/`
  (`public_proof_ledger_beta_narrowed.json`, `claim_history_diff_preview_narrowed.json`).

## Two registries

1. **Public-proof ledger** (`resolve_public_proof_ledger_entry`) — publishes one typed ledger section per joined
   proof source, per active supported line: the proof source and its canonical mode, the affected line rows, the
   linked compatibility / benchmark / support-window / known-limits / deprecation evidence-packet refs, the freshness
   state, the rollback / reversibility target, and the owning roster. A clean entry names a canonical registry token,
   a classified proof source, and a transparency role, covers the canonical / accessible / audit resolution forms,
   publishes a complete object, preserves its rollback posture before a claim widens, and keeps a public-facing
   section's compatibility / known-issues / support claim matched to current public proof. Otherwise it degrades
   honestly — a line widening its claim on stale proof, or a public-facing section running its language ahead of
   current proof, degrades to
   `public_proof_ledger_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof`, the structured blocker
   reason a widen-on-stale-proof attempt must surface.
2. **Claim-history diff** (`resolve_claim_history_diff_entry`) — turns a proof change into a typed diff event rather
   than an implicit docs mismatch. A clean entry names a classified diff scope (freshness-change, scope-narrowing, or
   release-line-reassociation) and provides the complete line-identity / affected-proof-source /
   previous-versus-current-claim-state / diff-scope / active-reason diff object; a diff that would keep support
   language ahead of current proof, hide the diff, or let a gap masquerade as covered degrades to
   `claim_history_diff_runs_support_ahead_of_proof_or_drops_claim_history_diff`.

## Per-entry ledger reference

The joined proof source carries its canonical mode, and the resolver publishes the full ledger object, so the
registry — never a report merely assumed to still be current — is the single source of truth.
`public_proof_ledger_object_is_complete` rejects an object missing any ledger field,
`line_preserves_rollback_and_diagnostics_before_widening` rejects a claim widening on stale proof or language running
ahead of current proof, and `claim_history_diff_stays_honest` rejects a diff that has kept support language ahead of
current proof.

## Acceptance criteria (proven by resolved examples)

- **Every active stable or LTS-candidate line exposes one machine-readable public-proof ledger with freshness state,
  last/current diff, and the exact evidence packet refs currently backing public claims.** Clean public-proof-ledger
  entries cover the canonical compatibility-report / benchmark-packet / support-window-statement / known-limits-set /
  deprecation-report / successor-report proof sources and the first release-center / shiproom / executive-steering /
  program-governance / support surfaces, an object-incomplete example degrades, and no clean ledger entry published
  an incomplete object.
- **A stale or mismatched compatibility / benchmark / known-limits / deprecation asset becomes a typed diff event
  rather than an implicit docs mismatch.** A widen-on-stale-proof example and an unbound example degrade, a clean
  ledger entry is present, and no clean entry is unbounded or unbound.
- **At least one docs / help / support / public-proof consumer renders the ledger directly and shows current-versus-
  previous claim-state history for a supported line.** Clean claim-history-diff entries cover the freshness-change /
  scope-narrowing / release-line-reassociation diff scopes with full resolution-form coverage while providing the
  complete diff object — the resolved line identity and the active diff reason — and a diff that would keep support
  language ahead of current proof or drop the diff degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_supported_line_public_proof_ledger_and_claim_history_diff_registries -- support-export
cargo run -p aureline-ui --example dump_m5_supported_line_public_proof_ledger_and_claim_history_diff_registries -- csv
cargo run -p aureline-ui --example dump_m5_supported_line_public_proof_ledger_and_claim_history_diff_registries -- report
cargo run -p aureline-ui --example dump_m5_supported_line_public_proof_ledger_and_claim_history_diff_registries -- public-proof-ledger-table
cargo run -p aureline-ui --example dump_m5_supported_line_public_proof_ledger_and_claim_history_diff_registries -- fixture-public-proof-ledger-beta-narrowed
cargo run -p aureline-ui --example dump_m5_supported_line_public_proof_ledger_and_claim_history_diff_registries -- fixture-claim-history-diff-preview-narrowed
```
