# First-run onboarding: no-account local entry, setup-later, repair-safe (stable)

## Why this lane exists

The very first launch on a fresh device is where a switching user decides
whether the product respects them. Three promises have to hold at once, and each
one used to be a place the product could quietly break:

- **Start working now, no account.** Useful local work — open a folder, open a
  file, clone a repository, start a scratch file, open a sample — must be
  reachable with **no account** and **no managed service**. A sign-in wall or a
  blocking account-nag card at first run breaks the local-first promise.
- **Do the setup later.** Sign-in, workspace trust, recommended extensions,
  appearance / keymap, an AI provider, a remote / managed connection, and an
  import from another editor are all *offered* but *deferrable*. Deferring a step
  must never block first-useful-work, and it must never quietly widen trust,
  install packages, apply a workflow bundle, or skip a required checkpoint. Each
  step keeps a resume route so it can be finished later.
- **Recover safely if first-run state is damaged.** An unreadable settings
  store, a half-finished migration, a missing locale pack, or a profile written
  by a newer build must degrade *safely*: local work stays reachable, the user's
  own files are never destroyed, nothing is silently reset, and the surface
  routes to a real repair action — never a dead end.

Before this lane, a first-run surface could imply an account was required, a
deferred step could silently grant trust, a corrupt settings file could
dead-end the launch, or a "reset to defaults" could quietly delete the user's
keymap. This lane closes that gap with **one governed record** every first-run
surface reads verbatim — it does **not** fork a per-surface first-run state.

## The governed record

`first_run_onboarding_record` is minted by
`crates/aureline-shell/src/first_run_onboarding` and frozen at the boundary by
`schemas/ux/finalize-first-run-onboarding-with-no-account-local.schema.json`. The
desktop shell, command palette, menus, diagnostics, support exports, Help/About,
and docs all read this single record, so they cannot drift on the no-account
posture, the setup-later posture, repair safety, accessibility, or the landing
for the same first run.

This lane is the **stable consumer** of the upstream first-run no-account
contract [`/docs/ux/no_account_local_entry_contract.md`](../no_account_local_entry_contract.md);
it does not re-mint that vocabulary, it projects it onto a governed stable
record. The repair cues reuse the canonical support / project-doctor
conventions: a `doctor.finding.*` finding code, a
`repair_transaction:<family>.<reason>` id
(`schemas/support/repair_transaction.schema.json`), an opaque `checkpoint:*`
ref, and the `metadata_safe_default` redaction class — so support, docs, and
shiproom packets reference the same truth.

## The honesty invariants

`FirstRunOnboardingRecord::build` refuses to mint a record that would lie. Each
is a hard `BuildError`, not a warning, so a dishonest projection fails the row
instead of shipping:

- **No-account local entry.** `account_required_for_local_work` and
  `managed_services_required_for_local_work` must be false,
  `local_work_available` must be true, and at least one `entry_verb` must be
  offered.
- **Setup-later posture.** No setup step may set `blocks_first_useful_work`, and
  an outstanding (offered or deferred) step may not set `widens_trust_on_defer`,
  `installs_packages_on_defer`, `applies_workflow_bundle_on_defer`, or
  `suppresses_required_checkpoint_on_defer`. Every step carries a canonical
  `resume_route_ref`. (A *completed* step may legitimately have installed
  packages — the bounded-deferral guard applies only to outstanding steps.)
- **Repair-safe recovery.** Every repair cue must set `preserves_user_work`,
  must not set `dead_end` or `silent_reset`, must use the
  `metadata_safe_default` redaction class, and must carry a canonical
  `finding_code`, `repair_transaction_ref`, and opaque `checkpoint_ref`. A
  `healthy` record surfaces **no** cue; a `degraded` or `needs_repair` record
  surfaces **at least one**.
- **Durable, accessible truth.** `keyboard_complete`, `focus_order_defined`,
  `high_contrast_reachable`, and `zoom_reachable` must all hold; the Start Center
  entry surface must be present and every entry surface keyboard-reachable. The
  `display_copy` invariants `toast_only_truth` and `theme_only_semantics` stay
  false.
- **No-account landing.** The first-useful-work `landing` must be
  `keyboard_reachable`, not `destructive`, and not `requires_account`.

Raw paths, raw command lines, raw URLs, raw tokens, and raw user content never
cross this boundary; the record carries opaque `aureline://<class>/<id>` refs,
the canonical `doctor.finding.*` and `repair_transaction:*` ids, opaque
checkpoint refs, and short reviewable sentences only.

## The stable drill matrix

The corpus in `crates/aureline-shell/src/first_run_onboarding/corpus.rs` projects
seven first-run drills through the live builder and pins each rendered record
bit-for-bit under
[`/fixtures/ux/m4/finalize-first-run-onboarding-with-no-account-local/`](../../../fixtures/ux/m4/finalize-first-run-onboarding-with-no-account-local/):

| Scenario | Health | Landing | Deferred / repair cues | What it proves |
| --- | --- | --- | --- | --- |
| Clean first run | healthy | empty_editor | 0 / 0 | Account-free entry with every setup step offered but deferrable. |
| Setup deferred, local only | healthy | local_workspace | 4 / 0 | Deferring sign-in, trust, extensions, and remote never blocks or widens trust. |
| Setup completed with import | healthy | local_workspace | 1 / 0 | Completing an import + keymap is account-free; sign-in still defers. |
| Degraded settings store | degraded | readme | 0 / 3 | Unreadable settings/keymap/appearance fall back to safe defaults with file-preserving repairs. |
| Needs repair: partial migration | needs_repair | local_workspace | 0 / 2 | A half-finished migration keeps local work reachable and holds state for a non-destructive repair. |
| Missing locale pack | degraded | readme | 0 / 1 | A missing locale pack falls back to the base locale and offers a refresh. |
| Newer, incompatible profile | needs_repair | sample_project | 0 / 1 | A profile from a newer build is preserved, not overwritten, with a guided repair. |

The drills cover all three health classes, all three setup postures (offered,
deferred, completed), every first-run resource that can be unhealthy (settings
store, keymap profile, appearance profile, migration state, locale pack,
onboarding state, workspace profile), every entry verb, all three entry
surfaces, and the four reachable landing classes.

## How to reproduce

```sh
# Re-emit the fixtures from the in-code corpus.
cargo run -q -p aureline-shell \
  --bin aureline_shell_first_run_onboarding -- emit-fixtures \
  fixtures/ux/m4/finalize-first-run-onboarding-with-no-account-local

# Stable corpus index (scenario, health, landing, rollups).
cargo run -q -p aureline-shell --bin aureline_shell_first_run_onboarding -- index

# Plaintext support-export truth block.
cargo run -q -p aureline-shell --bin aureline_shell_first_run_onboarding -- plaintext

# Replay + invariant gate.
cargo test -p aureline-shell --test first_run_onboarding_fixtures
```

## Narrowing rule

If first-run delivery proves a narrower stable claim than the matrix above (for
example, a platform where a repair route is not yet reachable from the keyboard),
downgrade the affected scenario below Stable in this doc and re-emit the packet
instead of papering over the gap. The record, schema, fixtures, and this doc move
together in the same change.
