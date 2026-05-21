# Beta Rollback-Drill Fixtures

These frozen fixtures back the beta install rollback drill consumed by
`crates/aureline-install/tests/rollback_drill_beta.rs`. The drill matures the
alpha synthetic state-root rollback into a beta-level rehearsal that ties the
synthetic restore to the governed beta rollback plan
(`artifacts/release/m3/update_rollback/rollback_plan.json`) and the
release-center rollback/revocation record model
(`crates/aureline-release` `RollbackOrRevocationRecord`).

The drill exercises:

- a planned rollback to a prior known-good build (the plan's rollback target);
- post-rollback durable state-root integrity verification (the restored target
  matches its captured pre-state while peer and portable roots stay untouched);
- exact-build install diagnostics after rollback resolving to the rollback
  target build identity instead of the superseded candidate; and
- an honest failure when the prior build is unavailable or unverifiable: a
  revoked rollback target, a missing/unverifiable retained prior artifact, and a
  missing captured prior-build state are all rejected rather than reported as a
  successful rollback.

Files:

- `release_center_rollback_record.json` — healthy release-center rollback record
  whose last-known-good ref is the plan's rollback target exact-build, with a
  consistent artifact graph and a linked rollback manifest.
- `release_center_rollback_record_missing_prior_build.json` — honest-failure
  record where the would-be rollback target build is revoked, so it can no
  longer back an exact-build rollback.
- `post_rollback_exact_build_diagnostics.json` — exact-build install diagnostics
  observed after rollback, reusing `ExactBuildInstallIdentity`, all resolving to
  the rollback target build identity.

Run:

```bash
cargo test -p aureline-install --test rollback_drill_beta
python3 ci/check_beta_rollback_drill.py --repo-root . --check
```
