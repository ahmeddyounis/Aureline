# Activity center (beta)

The beta activity center promotes the durable activity center to a
first-class beta surface that owns long-running jobs, failures,
retries, acknowledgements, and object reopen paths across the claimed
beta workflows. It replaces toast-only truth with durable rows that
agree with their status-strip badges and support-export rows on item
class, status, and resolution.

The projection lives in
[`crates/aureline-shell/src/activity_center/beta.rs`](../../../crates/aureline-shell/src/activity_center/beta.rs).
It does not re-derive the per-row lifecycle truth — that still comes
from the routed notification path under
[`crate::notifications`](../../../crates/aureline-shell/src/notifications/)
and the alpha row model under
[`crate::activity_center::alpha`](../../../crates/aureline-shell/src/activity_center/alpha.rs).
The beta page projects the acceptance promises a daily-beta reviewer
needs to inspect on every claimed row.

## Contract surface

The beta projection ships five record kinds, all under the shared
contract ref `shell:activity_center_beta:v1`:

- `shell_activity_center_beta_row_record` — durable activity row. Each
  row carries a stable `row_id`, `durable_job_id`, `canonical_event_id`,
  job family, source subsystem, severity, privacy, lifecycle state,
  resolution class, activity partition, the authoritative reopen
  class, the exact reopen identity (or a typed placeholder / denial
  reason), a typed `retry_posture`, a `toast_independence` block, and
  stable command ids for open-details and retry.
- `shell_activity_center_beta_badge_record` — row-aligned status-strip
  badge. The badge mirror echoes the row's `job_family`, `state_class`,
  `resolution_class`, and `activity_partition`. The validator rejects
  any drift between the badge and the row.
- `shell_activity_center_beta_page_record` — page record that
  aggregates the rows, the badge mirror, and a summary banner with
  reopen-class counts, long-running counts, durable-retry counts,
  attention-counting badge counts, and the set of job families
  present.
- `shell_activity_center_beta_support_export_row_record` — per-row
  export row that quotes the row's identity, family, state,
  resolution, reopen class, and exact reopen identity. Raw private
  material is excluded by construction.
- `shell_activity_center_beta_support_export_record` — support-export
  wrapper. Embeds the page, every per-row export row, and every
  `case_id` in stable page order so support reviewers can pivot from
  a row to the page without separate query plumbing.

## Acceptance posture

The beta projection delivers the four M3 activity-center acceptance
gates:

- **Authoritative reopen.** Every row promises one of three reopen
  classes: `exact_durable_object` (with a required
  `exact_target_identity_ref`), `truthful_placeholder` (with a
  required `placeholder_reason_label`), or `denied_and_explained`
  (with a required `denial_reason_label`). The validator
  (`validate_activity_center_beta_page`) rejects an exact reopen
  without an identity ref, a placeholder without a reason, and a
  denial without an explanation. A generic home fallback is not
  expressible.
- **Exact object routing.** When the reopen class is
  `exact_durable_object`, the row carries the canonical object
  identity that the open-details command targets. The badge mirror
  echoes the same row id so the row, the badge, and the support
  export pivot to the same case.
- **Badge / support-export parity.** Badges, rows, and support-export
  rows agree on `job_family`, `state_class`, `resolution_class`,
  `activity_partition`, `reopen_class`, and
  `exact_reopen_identity_ref`. The validators
  (`validate_activity_center_beta_page` and
  `validate_activity_center_beta_support_export`) reject any drift.
- **Toast independence.** Every row sets
  `recoverable_without_toast=true` and
  `reopenable_after_toast_expiry=true`; long-running or retryable
  rows must expose a durable affordance — either
  `durable_acknowledge_available=true` or a non-`not_applicable`
  retry posture. Durable retry, when offered, must be bound to a
  typed `retry_command_id` and not to a toast button. The validator
  rejects a long-running row that admits a toast-only recovery path.

The page must additionally cover the six claimed beta job families
(`indexing`, `restore`, `install_update`, `task_run`, `test_run`,
`git_review`); a coverage gap is treated as an acceptance failure.

## Headless inspector

The beta projection is exercised through the
`aureline_shell_activity_center` binary. The bin is the only
mint-from-truth path for the JSON checked in under
`fixtures/ux/m3/activity_center/`, so live shell records, CLI rows,
and support-export rows cannot drift.

```sh
cargo run -q -p aureline-shell --bin aureline_shell_activity_center -- page
cargo run -q -p aureline-shell --bin aureline_shell_activity_center -- rows
cargo run -q -p aureline-shell --bin aureline_shell_activity_center -- badges
cargo run -q -p aureline-shell --bin aureline_shell_activity_center -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_activity_center -- validate
```

`validate` exits non-zero (status `3`) if any acceptance invariant
fails, on either the page or the support-export wrapper; it is wired
so CI can fail closed on a regression in any record kind.

## Fixtures

Reviewable fixtures live under
[`fixtures/ux/m3/activity_center/`](../../../fixtures/ux/m3/activity_center/):

- `rows.json` — indexing-running, restore-completed, install/update
  partially-completed-with-retry, queued task, failed test with
  durable retry, denied git/publish with managed-policy explanation,
  and a restore-archived placeholder row.
- `badges.json` — row-aligned badge mirror for the seven rows.
- `page.json` — full beta page with the aggregate summary banner.
- `support_export.json` — support-export wrapper that quotes the
  page plus every per-row export row and case id in stable order.

## Verification

```sh
cargo test -p aureline-shell --test activity_center_beta_fixtures
cargo test -p aureline-shell --lib activity_center::beta
cargo run -q -p aureline-shell --bin aureline_shell_activity_center -- validate
```

The fixture test in
[`crates/aureline-shell/tests/activity_center_beta_fixtures.rs`](../../../crates/aureline-shell/tests/activity_center_beta_fixtures.rs)
replays every JSON fixture through the Rust types, asserts the
contract invariants, and asserts that the checked-in `page.json` is
bit-for-bit equal to the page returned by the seeded builder.
Regenerating with the headless bin is the only mint-from-truth path.

## Related contracts

- [Activity center alpha](../activity_center_alpha.md) — per-row
  lifecycle, partition, action-grammar, impact-flags, and
  Git/review-event row vocabulary the beta projection builds on.
- [Notification envelope contract](../notification_envelope_contract.md)
  — typed envelope vocabulary that mints the lifecycle observations
  consumed by the alpha row model.
- [Durable job envelope contract](../durable_job_envelope_contract.md)
  — durable job vocabulary the row, badge, and support export agree
  on.
