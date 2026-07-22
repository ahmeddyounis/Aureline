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

## Synthetic filesystem authority and limits

The filesystem-backed driver is deliberately narrower than an installer. Its
root must be an existing absolute directory that is either empty or already
contains the driver-created
`.rollback_drill/synthetic-authority-v1` marker. A populated, unmarked directory
is rejected, so a caller cannot accidentally point the drill at a real profile
or workspace tree. The driver pins the canonical root and revalidates it, the
`state-roots` directory, every target root, and every existing path component.
Symlinks and Windows reparse points are never followed, and Unix filesystem /
mount boundaries below the pinned authority are rejected.

Capture and restore admit at most 64 planned roots, 256 expected deltas, 4,096
entries total, 1,024 entries in one directory, 64 relative-path components,
2,048 UTF-8 bytes per relative path, 1 MiB per regular file, 4 MiB of aggregate
file content, and 24 MiB for the serialized snapshot document. JSON depth,
node, collection, key, and scalar limits are checked before typed projection.
Filesystem root refs are capped at the 255-byte portable segment ceiling, and a
drill ID is capped at 240 bytes so its generated `.pre_state.json` filename also
fits that ceiling. General non-path record IDs retain their separate 512-byte
bound.
Special files, non-UTF-8 names, unstable reads, redirected ancestors, oversized
files, directory explosions, and excessive nesting fail closed. Errors carry
logical path classes and I/O categories rather than host paths, parser payloads,
or OS error text.

On Unix, restore materializes bounded sibling staging trees, quarantines each
target with a same-parent rename, installs the staged tree, revalidates object
identity after every rename, and rolls already-installed roots back if a later
root fails. Quarantine cleanup is bounded and redirect-aware; if cleanup cannot
finish safely, the driver reports `RecoverableCleanupPending` and retains the
quarantine instead of broadening deletion. Reported retained-artifact counts
are the backup and staging paths that still exist, including both paths when a
failed transaction retained both. If a synthetic update fails after its first
successful write, or mutation verification itself fails, the driver first
attempts the same transactional restore; inability to complete it is reported
as `RestoreRecoveryRequired` rather than returning only the initiating error.

Pending atomic writes remain armed until the installed path and open handle are
proven to identify the same bounded file. Before that point, errors and unwinds
truncate and sync the open inode, even if its parent directory was moved. Once
replacement succeeds the predecessor is no longer recoverable, so the guard is
disarmed before syncing the parent directory; a parent-sync failure reports an
error without deleting or truncating the successfully installed replacement.

The standard library does not expose a stable Windows file identity adequate
for authorizing destructive rename/delete races. Consequently the synthetic
driver detects Windows reparse points but fails closed before replacing an
existing file or performing destructive restore/cleanup on Windows. This is an
explicit drill limitation, not a claim that Windows installer rollback is
implemented; enabling it requires a reviewed stable-file-ID mechanism and
cross-platform adversarial coverage.

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
