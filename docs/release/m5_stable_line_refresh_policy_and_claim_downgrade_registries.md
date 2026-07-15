# M5 stable-line refresh-policy and claim-downgrade packet registries

This lane makes evidence refresh an ordinary release operation over the frozen
[M5 stable-line-protection matrix](./m5-stable-line-ops.md). It turns the *refresh-policy* grammar (how an
active stable line schedules a refresh cadence for each evidence surface it publishes — the certified-archetype
report, the compatibility packet, the known-limits doc, the release / help / About surface, the public-proof
surface, and the support-export packet — with an exact last-run identity, next-run identity, next-run owner,
last-success state, and freshness SLO) and the *claim-downgrade packet* grammar (the machine-readable packet
emitted when a surface misses its declared refresh window, moving the affected claim automatically to
Retest-pending, Evidence-stale, or a narrower support-language claim and naming the active downgrade reason) into
registry resolvers that produce export-safe, honest projections, so the shiproom, release-center,
executive-steering, program-governance, diagnostics, docs, CLI, support, and public-proof surfaces resolve one
canonical stable-line refresh truth instead of relying on launch-era packets to stay implicitly current. The
refresh schedule and the claim downgrade are separated in runtime and serialized state: the refresh surface,
scheduled rows, cadence window, last-run / next-run identity, next-run owner, last-success state, and freshness
posture live on the refresh policy, while the resolved line identity, missed-window ledger, affected-surface
reference, downgrade-scope state, narrowed-claim state, active downgrade reason, and last downgrade revision live
on the claim-downgrade packet, and a line's refresh posture stays preserved so support language never runs ahead
of current refresh proof.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_stable_line_refresh_policy_and_claim_downgrade_registries` (the authoritative
  validator).
- **Combined schema:**
  `schemas/program/m5-stable-line-refresh-policy-and-claim-downgrade-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-stable-line-refresh-policy.schema.json`](../../schemas/program/m5-stable-line-refresh-policy.schema.json)
  (reused from the frozen matrix) and
  [`schemas/program/m5-claim-downgrade-packet.schema.json`](../../schemas/program/m5-claim-downgrade-packet.schema.json)
  (minted by this lane) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-stable-line-refresh-policy-and-claim-downgrade-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/release/m5-stable-line-refresh-policy-and-claim-downgrade-registries/`
  (`refresh_policy_beta_narrowed.json`, `claim_downgrade_preview_narrowed.json`).

## Two registries

1. **Refresh policy** (`resolve_refresh_policy_entry`) — publishes one typed scheduled-refresh object per
   supported line surface: the refresh surface and its canonical mode, the scheduled rows, the cadence window,
   the last-run identity, the next-run identity, the next-run owner, the last-success state, and the freshness
   posture. A clean entry names a canonical registry token, a classified refresh surface, and a
   stable-line-protection role, covers the canonical / accessible / audit resolution forms, publishes a complete
   object, preserves its refresh posture before a claim widens, and keeps a public-facing surface's support
   language matched to current refresh proof. Otherwise it degrades honestly — a line widening its claim without
   a current last-success run, or a public-facing surface running its support language ahead of refresh proof,
   degrades to `refresh_policy_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof`, the structured
   blocker reason a widen-on-stale-evidence attempt must surface.
2. **Claim-downgrade packet** (`resolve_claim_downgrade_entry`) — emits the machine-readable downgrade when a
   surface misses its window. A clean entry names a classified downgrade scope (Retest-pending, Evidence-stale,
   or narrower support language) and provides the complete line-identity / missed-window-ledger /
   affected-surface / downgrade-scope / narrowed-claim / active-downgrade-reason / last-downgrade-revision packet
   object; a packet that would keep support language ahead of current proof, hide the downgrade, or let a stale
   surface masquerade as fresh degrades to
   `claim_downgrade_runs_support_ahead_of_proof_or_drops_claim_downgrade`.

## Per-entry refresh-schedule reference

The refresh surface carries its canonical mode, and the resolver publishes the full scheduled-refresh object, so
the registry — never a launch-era packet assumed to stay current — is the single source of truth.
`refresh_policy_object_is_complete` rejects an object missing any schedule field,
`line_preserves_rollback_and_diagnostics_before_widening` rejects a claim widening without a current
last-success run or public support language running ahead of refresh proof, and `claim_downgrade_stays_honest`
rejects a downgrade packet that has kept support language ahead of current proof.

A widen-on-stale-evidence attempt degrades to
`refresh_policy_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof`, an incomplete object degrades
to `refresh_policy_object_incomplete`, and a downgrade packet running support ahead of current proof degrades to
`claim_downgrade_runs_support_ahead_of_proof_or_drops_claim_downgrade`, so a widen-on-stale-evidence attempt, an
incomplete object, or a dropped downgrade can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **At least one active stable line shows scheduled refresh state for compatibility, bundle/archetype, docs,
  help/About, and support packets with exact last-run and next-run identity.** Clean refresh-policy entries cover
  the canonical certified-archetype-report / compatibility-packet / known-limits-doc / release-help-About-surface
  / public-proof-surface / support-export-packet surfaces and the first release-center / shiproom /
  executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and no clean
  refresh-policy entry published an incomplete object.
- **When a packet ages out or misses its declared refresh window, the affected claim narrows automatically and
  the downgrade reason is visible across release/help/support/public-proof consumers.** A widen-on-stale-evidence
  example and an unbound example degrade, a clean bounded refresh-policy entry is present, and no clean entry is
  unbounded or unbound.
- **Support and shiproom exports can prove that stable-line truth is current or explicitly downgraded rather than
  silently stale.** Clean claim-downgrade packet entries cover the Retest-pending / Evidence-stale /
  narrowed-support downgrade scopes with full resolution-form coverage while providing the complete packet object
  — the resolved line identity and the active downgrade reason — and a packet that would keep support language
  ahead of current proof or drop the downgrade degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_stable_line_refresh_policy_and_claim_downgrade_registries -- support-export
cargo run -p aureline-ui --example dump_m5_stable_line_refresh_policy_and_claim_downgrade_registries -- csv
cargo run -p aureline-ui --example dump_m5_stable_line_refresh_policy_and_claim_downgrade_registries -- report
cargo run -p aureline-ui --example dump_m5_stable_line_refresh_policy_and_claim_downgrade_registries -- refresh-policy-table
cargo run -p aureline-ui --example dump_m5_stable_line_refresh_policy_and_claim_downgrade_registries -- fixture-refresh-policy-beta-narrowed
cargo run -p aureline-ui --example dump_m5_stable_line_refresh_policy_and_claim_downgrade_registries -- fixture-claim-downgrade-preview-narrowed
```
