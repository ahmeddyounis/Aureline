# Fixtures: precedence inspectors

This directory contains fixture metadata for the `m5_precedence_inspectors` packet.

The canonical full corpus is checked in at:

`artifacts/support/m5/m5-precedence-inspector.json`

It is the one authoritative precedence-inspector registry; the typed model and fail-closed precedence
gate live in the `aureline-support` crate (`m5_precedence_inspector`).

## Coverage

- All five resolver families — `toolchain`, `setting`, `policy`, `credential`, and `route` — carry at
  least one inspector, each projecting its resolver's truth by reference (every candidate carries a
  `descriptor_ref` and every inspector a `source_of_truth_ref`) and carrying a one-step explain entry
  plus the equivalent CLI / headless object id.
- The five precedence sources (`policy_scoped`, `project_scoped`, `user_scoped`, `system_scoped`,
  `fallback_scoped`), the two value disclosures (`plain_values`, `metadata_only`), and the five
  candidate dispositions (`winner`, `overshadowed`, `unavailable`, `blocked`, `conflicting`) are each
  exercised across the candidates.
- The six resolution classes (`resolved`, `fallback`, `override`, `drift`, `conflict`, `blocked`) are
  each exercised, and the published presentation covers `transparent` (`toolchain-resolved`),
  `narrowed` (`toolchain-fallback`, `setting-workspace-over-user`, `credential-class-change`,
  `route-target-drift`, `route-conflict`), and `blocked` (`policy-lock-blocked`).
- The six downgrade reasons — `silent_fallback_eliminated`, `hidden_override`, `source_drift`,
  `unreconciled_conflict`, `policy_lock_blocked`, and `redaction_boundary` — are each exercised, the
  seven resolution paths are each exercised, and all four restart-or-reauth postures (`none`,
  `restart_required`, `reauth_required`, `reconnect_required`) are each exercised.
- The named cases are all present: hidden fallback elimination (`toolchain-fallback`), workspace-over-
  user (`setting-workspace-over-user`), policy-over-user (`policy-lock-blocked`), credential-class
  change (`credential-class-change`), and route / target drift (`route-target-drift`), plus an
  unreconciled route conflict (`route-conflict`).
- The gate is exercised in every direction: one inspector resolves cleanly and transparently with the
  winner genuinely out-precedeing its overshadowed candidate, proving the gate is not a blanket flag;
  the fallback is proven forced by an unavailable higher-precedence candidate; the override and drift
  cases keep the suppressed candidate visible; the credential case stays metadata-only with no raw
  values; the policy case blocks before use and keeps the hidden payload out; and the conflict declares
  no winner instead of silently picking one. Each inspector's `presentation`, `downgrade_reasons`,
  `resolution_path`, and `blocked_before_use` flag equal the recomputed gate, so the active-surface,
  support-center, support-export, issue-report-packet, and cli-headless surfaces ingest one registry
  and a narrowed or blocked inspector cannot read as a clean "this value won" chip.
