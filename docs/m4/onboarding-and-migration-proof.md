# Onboarding and migration proof — companion documentation

This document is the companion contract for the onboarding and migration proof
index. It explains what the proof index proves, how effective claims are derived,
which consumer surfaces must ingest it, and how to maintain proof truth without
drifting into stale or contradictory claims.

Canonical machine source (do not clone status text from this document — ingest
the JSON):

- Index: [`/artifacts/ux/m4/onboarding-and-migration-proof-index.json`](../../artifacts/ux/m4/onboarding-and-migration-proof-index.json)
- Scoreboard: [`/artifacts/ux/m4/switching-row-scoreboard.md`](../../artifacts/ux/m4/switching-row-scoreboard.md)
- Upstream marketed rows: [`/docs/migration/m3/marketed_switching_rows.md`](../migration/m3/marketed_switching_rows.md)
- First-run onboarding evidence: [`/artifacts/ux/m4/finalize-first-run-onboarding-with-no-account-local.md`](../../artifacts/ux/m4/finalize-first-run-onboarding-with-no-account-local.md)
- Migration center evidence: [`/artifacts/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported.md`](../../artifacts/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported.md)
- Warm continuity evidence: [`/artifacts/ux/m4/harden_shell_startup_warm_restore_and_first_useful.md`](../../artifacts/ux/m4/harden_shell_startup_warm_restore_and_first_useful.md)
- Start center evidence: [`/artifacts/ux/m4/stabilize-the-start-center-recent-work-list-workspace.md`](../../artifacts/ux/m4/stabilize-the-start-center-recent-work-list-workspace.md)
- Archetype preflight evidence: [`/artifacts/ux/m4/stabilize-workspace-archetype-detection-readiness-preflight.md`](../../artifacts/ux/m4/stabilize-workspace-archetype-detection-readiness-preflight.md)

## What the proof index proves

The proof index binds the first-run onboarding, migration center, warm continuity,
Start Center, and archetype preflight evidence packets into one derived proof record
per marketed switching row. For each row it answers:

1. **No-account local entry.** Can a user reach first-useful-work without an account
   or managed service on this row? Every row's no-account dimension cites a fixture
   from the first-run onboarding or migration center corpus and requires the fixture
   to pin `account_required_for_local_work: false`,
   `managed_services_required_for_local_work: false`, and `local_work_available:
   true`.

2. **Setup-later continuity.** If a user defers sign-in, trust, extensions, or a
   remote connection, does first-useful-work still complete? Every applicable row's
   setup-later dimension cites a deferred-setup fixture and requires no setup step to
   set `blocks_first_useful_work: true` and no deferred step to widen trust, install
   packages, apply a workflow bundle, or suppress a required checkpoint.

3. **Import diff/rollback posture.** For rows that involve a source-ecosystem import,
   is the diff reviewed before apply, is rollback available, and is rollback verified
   with a live per-ecosystem apply session? Rows where rollback is projected from an
   adjacent flow rather than verified are narrowed to beta with
   `rollback_evidence_incomplete`.

4. **Unsupported-gap taxonomy.** For rows that involve a source-ecosystem import, is
   the Exact/Translated/Partial/Shimmed/Unsupported taxonomy visible before the user
   commits? Every applicable gap must carry `visible_before_apply: true`; a gap that
   is silent until after apply is a narrowing defect.

5. **First-useful-work success.** Does the row reach a typed first-useful-work
   landing that is keyboard-reachable, non-destructive, and account-free? Rows whose
   first-useful-work routing is in retest-pending carry
   `narrowed_retest_pending` and stay beta until the archetype qualification report
   exits retest-pending.

6. **Consumer-surface parity.** Are Start Center, migration center, Help/About,
   release notes, docs packs, and support playbooks consuming the same canonical
   packet? The proof index tracks wiring state per surface. Wiring-pending is honest
   for a pre-implementation repository and does not narrow the core proof claims, but
   each surface owner is expected to ingest the canonical source rather than
   maintaining bespoke status text.

## Dimension applicability

Not every dimension applies to every row family:

| Dimension | Entry rows | Archetype rows | Migration source rows |
|-----------|------------|----------------|----------------------|
| No-account local entry | ✓ | ✓ | ✓ |
| Setup-later continuity | ✓ | ✓ | ✓ |
| Import diff/rollback | only `entry.import` | — | ✓ |
| Unsupported-gap taxonomy | only `entry.import` | — | ✓ |
| First-useful-work success | ✓ | ✓ | ✓ |
| Consumer-surface parity | ✓ | ✓ | ✓ |

## Derivation rules

An `effective_claim` is derived from the most conservative applicable dimension:

1. If any applicable dimension has `proof_state: narrowed_incomplete_rollback` →
   the row's effective claim is at most `beta`.
2. If any applicable dimension has `proof_state: narrowed_retest_pending` →
   the row's effective claim is at most `beta`.
3. If any applicable dimension has `proof_state: narrowed_stale_evidence` →
   the row's effective claim is at most `beta`.
4. If any applicable dimension has `proof_state: narrowed_packet_missing` →
   the row's effective claim is at most `preview`.
5. A row family with a `beta` label ceiling (archetype rows) cannot hold a stable
   effective claim regardless of individual dimension proof states.
6. A row that passes all applicable dimensions at stable holds effective claim
   `stable`.

The derivation is re-run whenever a dimension's source packet or fixture is
refreshed. A row cannot drift from its evidence.

## Narrowing the claim — never widening it

When delivery proves a narrower stable claim than the current index:

1. Update the affected dimension(s) in the proof index with the honest proof state
   and narrowing reason.
2. Re-derive the row's `effective_claim` and `narrowing_reasons`.
3. Update the scoreboard and this companion doc in the same change set.
4. Do not adjust the upstream marketed-rows doc to inherit the prior green row; the
   marketed-rows doc cites the scoreboard for current posture.

No row may inherit a green effective claim from an adjacent row with stronger
evidence. Narrowing is permanent until the specific gap is resolved.

## Stale evidence handling

Each dimension carries a `freshness_slo_days` and a `freshness_state`. When
`freshness_state` transitions from `current` to `stale`:

- The dimension's `proof_state` transitions to `narrowed_stale_evidence`.
- The row's `effective_claim` is re-derived; if it was stable, it narrows to beta.
- Consumer surfaces that ingest the proof index will reflect the narrowed claim
  automatically without requiring a manual update.

The canonical approach is: land a refreshed source packet (fixture, schema, emitter),
verify the dimension still proves at the target claim level, update the `captured_at`
timestamp, and let the derivation restore the claim.

## Consumer surfaces

The following consumer surfaces must ingest the proof index rather than cloning
status text:

### Start Center

The Start Center entry surfaces (`start_center_packet`) carry `target_kind_label`
vocabulary from the same corpus the proof index references. Once wired, the Start
Center should render the `effective_claim` and `caveats` from the proof index for
the row it is presenting, rather than hand-authoring a claim string.

### Migration center

The migration center (`migration_center_packet`) carries the per-ecosystem
diff/rollback/taxonomy records. Once wired, the migration center pivot and its
claim label should be derived from the proof index row for the active ecosystem
rather than a bespoke status field.

### Help/About

Help/About should ingest the row's `effective_claim`, `narrowing_reasons`, and
`caveats` from the proof index and project them through the
`HelpAboutReleaseTruthCard` vocabulary. It must not fabricate a claim string that
is wider than the proof index row's `effective_claim`.

### Release notes

Release notes for a stable promotion event should enumerate the proof index rows
and their `effective_claim` values rather than asserting broad switching claims.
Rows narrowed to beta must appear as beta in the release notes; no aggregation may
hide a narrowed sub-row.

### Docs packs

Migration guides and onboarding docs must cite the proof index row's
`effective_claim` and `caveats` as the canonical switching truth for the row they
describe. The docs pack reference is this companion document.

### Support playbooks

Support playbooks should cite the proof index row's `caveats` and the canonical
source packet's `support_export_ref` or `support_export_lines` projection when
preparing diagnostics or filing incidents related to a switching row.

## Keyboard completeness and accessibility invariants

The proof index inherits the accessibility invariants from the upstream evidence
packets:

- Every entry verb, setup-step resume route, and first-useful-work landing must be
  keyboard-reachable (inherited from the first-run onboarding packet).
- Every migration center recovery route and taxonomy gap must be keyboard-reachable
  (inherited from the migration center packet).
- High-contrast and zoomed layout parity must hold on every surface that presents
  proof-index rows (inherited from the Start Center, migration center, and archetype
  preflight packets).
- First-run truth must not be toast-only or theme-only: the `display_copy` block of
  every first-run record keeps `toast_only_truth` and `theme_only_semantics` false.

## Rollback guidance

For rows where `import_diff_rollback` is applicable:

- The canonical rollback contract is [`/docs/migration/first_run_import_diff_and_rollback_contract.md`](../migration/first_run_import_diff_and_rollback_contract.md).
- The migration center evidence packet describes per-ecosystem rollback posture at
  [`/artifacts/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported.md`](../../artifacts/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported.md).
- A row where rollback is projected from an adjacent flow (`narrowed_incomplete_rollback`) must not advertise rollback as verified for the affected ecosystem.

## Honest posture

This is the honest posture for a pre-implementation repository. Five of seventeen
rows qualify stable (four entry rows plus VS Code / Code-OSS migration); twelve are
narrowed to beta with named reasons. Consumer wiring is universally pending.

No switching claim may be marketed beyond the effective claim in the proof index for
the row being promoted. If a switching path still requires caveats, the caveats are
published and the row is narrowed rather than hidden behind generic onboarding copy.
