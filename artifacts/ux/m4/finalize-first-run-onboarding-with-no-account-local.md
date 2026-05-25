# First-run onboarding (no-account, setup-later, repair-safe) — release evidence

Reviewer-facing evidence packet for the lane that makes the first launch
replacement-grade on claimed stable desktop rows: account-free local entry, a
setup-later posture that never blocks first-useful-work or silently widens trust,
repair-safe recovery cues that preserve the user's files and carry an export-safe
chain of custody, durable and keyboard-reachable first-run truth, and an
account-free first-useful-work landing.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Records / fixtures: [`/fixtures/ux/m4/finalize-first-run-onboarding-with-no-account-local/`](../../../fixtures/ux/m4/finalize-first-run-onboarding-with-no-account-local/)
- Schema: [`/schemas/ux/finalize-first-run-onboarding-with-no-account-local.schema.json`](../../../schemas/ux/finalize-first-run-onboarding-with-no-account-local.schema.json)
- Companion doc: [`/docs/ux/m4/finalize-first-run-onboarding-with-no-account-local.md`](../../../docs/ux/m4/finalize-first-run-onboarding-with-no-account-local.md)
- Upstream contract: [`/docs/ux/no_account_local_entry_contract.md`](../../../docs/ux/no_account_local_entry_contract.md)
- Repair-transaction contract: [`/schemas/support/repair_transaction.schema.json`](../../../schemas/support/repair_transaction.schema.json)
- Typed source: `aureline_shell::first_run_onboarding` (`model`, `corpus`)
- Headless emitter: `aureline_shell_first_run_onboarding`
- Replay + invariant gate: `crates/aureline-shell/tests/first_run_onboarding_fixtures.rs`

## What this packet proves

1. **First-run local entry is account-free on every claimed stable row.** Each
   record's `entry` pins `account_required_for_local_work` and
   `managed_services_required_for_local_work` false, `local_work_available` true,
   and a non-empty `entry_verbs` list; the builder rejects any record that claims
   local work needs an account or a managed service, and the gate replays it for
   all seven scenarios.

2. **Setup is offered but deferrable, and deferral is bounded.** No setup step
   sets `blocks_first_useful_work`, and an outstanding (offered / deferred) step
   never widens trust, installs packages, applies a workflow bundle, or
   suppresses a checkpoint. Every step carries a canonical `resume_route_ref`.
   The `setup_deferred_local_only` drill defers sign-in, trust, extensions, and a
   remote connection without blocking local work; `setup_completed_with_import`
   shows a completed import + keymap with sign-in still deferring cleanly.

3. **Damaged first-run state degrades safely with a real repair route.** Every
   `repair_cue` sets `preserves_user_work`, never sets `dead_end` or
   `silent_reset`, routes through `metadata_safe_default`, and carries an
   export-safe chain of custody — a `doctor.finding.*` finding code, a
   `repair_transaction:<family>.<reason>` id, and an opaque `checkpoint:*` ref —
   plus a keyboard-reachable `repair_route_ref`. A `healthy` record carries no
   cue; a `degraded` or `needs_repair` record carries at least one. The drills
   cover an unreadable settings/keymap/appearance store, a partial migration, a
   missing locale pack, and a newer incompatible profile.

4. **First-run truth is durable, accessible, and never toast- or theme-only.**
   Each `accessibility` block pins `keyboard_complete`, `focus_order_defined`,
   `high_contrast_reachable`, and `zoom_reachable`; the Start Center entry surface
   is always present and every entry surface (Start Center, command palette,
   menu) is keyboard-reachable to the same routes. The derived `display_copy`
   keeps `toast_only_truth` and `theme_only_semantics` false.

5. **The first-useful-work landing is keyboard-reachable, non-destructive, and
   account-free.** Each `landing` selects one of the typed classes, stays
   `keyboard_reachable`, and pins `destructive` and `requires_account` false. The
   `support_export_lines` projection surfaces the no-account claim, the health,
   and each repair cue's finding and transaction ids for diagnostics / support
   export.

## Scenario rollups (pinned)

| Fixture | Scenario | Health | Landing | Deferred / repair cues | Honesty marker |
| --- | --- | --- | --- | --- | --- |
| `clean_first_run.json` | clean_first_run | healthy | empty_editor | 0 / 0 | false |
| `setup_deferred_local_only.json` | setup_deferred_local_only | healthy | local_workspace | 4 / 0 | true |
| `setup_completed_with_import.json` | setup_completed_with_import | healthy | local_workspace | 1 / 0 | true |
| `degraded_settings_store.json` | degraded_settings_store | degraded | readme | 0 / 3 | true |
| `needs_repair_partial_migration.json` | needs_repair_partial_migration | needs_repair | local_workspace | 0 / 2 | true |
| `missing_locale_pack.json` | missing_locale_pack | degraded | readme | 0 / 1 | true |
| `newer_profile_incompatible.json` | newer_profile_incompatible | needs_repair | sample_project | 0 / 1 | true |

## Coverage

The replay gate `corpus_covers_required_cases_and_vocabularies` asserts the
corpus exercises every scenario class, all three health classes, all six entry
verbs, all seven setup steps, all three setup postures, all seven first-run
resources, all three entry surfaces, and the four reachable landing classes — so
a new vocabulary value cannot land without a drill.

## Reproduce

```sh
cargo run -q -p aureline-shell --bin aureline_shell_first_run_onboarding -- index
cargo run -q -p aureline-shell --bin aureline_shell_first_run_onboarding -- plaintext
cargo test -p aureline-shell --test first_run_onboarding_fixtures
cargo test -p aureline-shell --lib first_run_onboarding
```

## Narrowing rule

This packet is canonical for the stable line in this lane. If delivery proves a
narrower stable claim than the rollups above, downgrade the affected scenario in
the companion doc and re-emit this packet rather than inheriting an adjacent
green row. The record, schema, fixtures, doc, and this packet move together in
the same change.
