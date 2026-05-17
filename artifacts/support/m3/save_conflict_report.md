# Save-conflict regression-suite baseline report

This artifact is the reviewer-facing baseline rendering of the
regression-suite report produced by the
[`save_conflict_suite`](../../../crates/aureline-vfs/src/save_conflict_suite/mod.rs)
module from the protected corpus under
[`/fixtures/recovery/m3/save_conflict_suite/`](../../../fixtures/recovery/m3/save_conflict_suite/).
It records the exact expected behavior, downgrade label, and open gap
class set for every claimed (`scenario_class`, `platform_row_class`)
tuple. Per-platform summaries show how many rows pass cleanly, how many
require a closed downgrade label, and how many are blocked until a fix
lands. The report stays metadata-safe: it never carries raw private
material or ambient authority, and every row is drawn from the closed
suite vocabularies.

Schema: `schemas/recovery/save_conflict_suite_beta.schema.json`
(record kind `save_conflict_suite_report_record`, version 1).
Matrix doc: [`docs/state/m3/save_conflict_beta_matrix.md`](../../../docs/state/m3/save_conflict_beta_matrix.md).
Corpus manifest:
[`fixtures/recovery/m3/save_conflict_suite/manifest.yaml`](../../../fixtures/recovery/m3/save_conflict_suite/manifest.yaml).

## Matrix rows

| Case ID | Scenario | Platform | Anchor difficulty | Compare outcome | Expected outcome | Downgrade label | Open-gap classes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `case:save_conflict_suite:alias_drift:linux_desktop` | `alias_drift` | `linux_desktop` | `case_only_drift` | `unchanged` | `downgrade_required` | `yellow_partial_coverage` | `platform_implementation_pending` |
| `case:save_conflict_suite:alias_drift:macos_desktop` | `alias_drift` | `macos_desktop` | `case_only_drift` | `unchanged` | `pass` | `none` | `none` |
| `case:save_conflict_suite:alias_drift:windows_desktop` | `alias_drift` | `windows_desktop` | `case_only_drift` | `unchanged` | `pass` | `none` | `none` |
| `case:save_conflict_suite:difficult_save_path:linux_desktop` | `difficult_save_path` | `linux_desktop` | `bind_mount_overlay` | `external_change_detected` | `pass` | `none` | `none` |
| `case:save_conflict_suite:difficult_save_path:macos_desktop` | `difficult_save_path` | `macos_desktop` | `bind_mount_overlay` | `external_change_detected` | `downgrade_required` | `degraded_to_safe_path_only` | `platform_implementation_pending` |
| `case:save_conflict_suite:difficult_save_path:windows_desktop` | `difficult_save_path` | `windows_desktop` | `bind_mount_overlay` | `external_change_detected` | `downgrade_required` | `degraded_to_safe_path_only` | `platform_implementation_pending` |
| `case:save_conflict_suite:external_change:linux_desktop` | `external_change` | `linux_desktop` | `unicode_normalization` | `external_change_detected` | `pass` | `none` | `none` |
| `case:save_conflict_suite:external_change:macos_desktop` | `external_change` | `macos_desktop` | `unicode_normalization` | `external_change_detected` | `pass` | `none` | `none` |
| `case:save_conflict_suite:external_change:windows_desktop` | `external_change` | `windows_desktop` | `unicode_normalization` | `external_change_detected` | `downgrade_required` | `yellow_platform_skew` | `platform_implementation_pending` |
| `case:save_conflict_suite:permission_loss:linux_desktop` | `permission_loss` | `linux_desktop` | `symlink_alias` | `wrong_target_prevented` | `pass` | `none` | `none` |
| `case:save_conflict_suite:permission_loss:macos_desktop` | `permission_loss` | `macos_desktop` | `symlink_alias` | `wrong_target_prevented` | `pass` | `none` | `none` |
| `case:save_conflict_suite:permission_loss:windows_desktop` | `permission_loss` | `windows_desktop` | `symlink_alias` | `wrong_target_prevented` | `downgrade_required` | `degraded_to_safe_path_only` | `platform_implementation_pending` |
| `case:save_conflict_suite:save_conflict:linux_desktop` | `save_conflict` | `linux_desktop` | `symlink_alias` | `save_conflict` | `pass` | `none` | `none` |
| `case:save_conflict_suite:save_conflict:macos_desktop` | `save_conflict` | `macos_desktop` | `symlink_alias` | `save_conflict` | `pass` | `none` | `none` |
| `case:save_conflict_suite:save_conflict:windows_desktop` | `save_conflict` | `windows_desktop` | `symlink_alias` | `save_conflict` | `downgrade_required` | `yellow_platform_skew` | `platform_implementation_pending` |

## Per-platform summary

| Platform | Cases | Pass | Downgrade required | Blocked until fix | Open gaps |
| --- | --- | --- | --- | --- | --- |
| `linux_desktop` | 5 | 4 | 1 | 0 | 1 |
| `macos_desktop` | 5 | 4 | 1 | 0 | 1 |
| `windows_desktop` | 5 | 1 | 4 | 0 | 4 |

All declared open gaps belong to the `platform_implementation_pending`
class; no row is currently blocked until a fix lands. Every row that
declares an open gap also declares a closed downgrade label, and every
row that downgrades records at least one open-gap entry — both
contracts are checked by
[`SaveConflictSuiteEvaluator::validate_corpus`](../../../crates/aureline-vfs/src/save_conflict_suite/mod.rs)
and the integration drill at
[`crates/aureline-vfs/tests/save_conflict_suite.rs`](../../../crates/aureline-vfs/tests/save_conflict_suite.rs).

## Open gaps per platform

- `linux_desktop` / `alias_drift` (`yellow_partial_coverage`):
  the case-only variant alias is only observable on a casefold-enabled
  mount or a fuse insensitive-preserving layer; stock ext4 reports the
  drift via the `divergent_unknown_alias` fall-through.
- `macos_desktop` / `difficult_save_path` (`degraded_to_safe_path_only`):
  macOS does not expose POSIX bind-mounts; firmlink / nullfs equivalents
  need a dedicated `alias_kind` chain before the row can graduate from
  `degraded_to_safe_path_only`.
- `windows_desktop` / `external_change` (`yellow_platform_skew`):
  NTFS does not auto-normalize NFD/NFC; the alias resolution lands via
  the case-only / object-id path rather than the
  `unicode_normalization_variant` inspector chain.
- `windows_desktop` / `save_conflict` (`yellow_platform_skew`):
  Windows symlink creation requires developer mode or admin; the suite
  reads through pre-created symlinks only.
- `windows_desktop` / `permission_loss` (`degraded_to_safe_path_only`):
  Windows `policy_constrained` semantics (deny ACEs, mandatory integrity
  labels) surface through a distinct primitive than POSIX bits; platform
  parity reporting is pending.
- `windows_desktop` / `difficult_save_path` (`degraded_to_safe_path_only`):
  Windows surfaces overlay aliases as junction reparse points; the
  suite degrades to safe-path until a junction-aware
  `bind_mount_alias` projection lands.

## Safety baseline

- `raw_private_material_excluded = true` on every row and the report.
- `ambient_authority_excluded = true` on every row and the report.
- `destructive_resets_present = false` on every row.
- `preserves_user_authored_files = true` on every row.

## Out-of-scope

- Live runtime measurement of save-pipeline latency or throughput.
- Cross-tenant or hosted ticket routing — the report is consumed
  locally by the support-export pipeline and the chrome.
- Adding new downgrade labels or open-gap classes without updating the
  schema, the Rust module, the matrix doc, this report, and the
  protected corpus together.
