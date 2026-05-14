# Recovery-ladder alpha

This page is the support-facing contract for the alpha recovery ladder.
The implementation lives in
[`crates/aureline-support/src/recovery_ladder/mod.rs`](../../crates/aureline-support/src/recovery_ladder/mod.rs)
and consumes the protected fixture corpus under
[`fixtures/recovery/recovery_ladder_alpha/`](../../fixtures/recovery/recovery_ladder_alpha/).

The alpha ladder covers four bounded rungs:

| Rung | What changes | What is preserved | Return path |
|---|---|---|---|
| Safe mode | Disables extension activation, restore replay, remote auto-reattach, and heavy optional services. | User-authored files, open-buffer position, durable indexes, trust, credentials, restore records, and support export state. | Explicit exit from safe mode after the crash-loop finding is reviewed. |
| Runtime or extension quarantine | Isolates one runtime or extension lane and stops automatic restart/readmission. | User-authored files, open-buffer position, durable indexes, trust, credentials, and support export state. | Explicit clear or re-enable action after owner, reason, expiry, and evidence are reviewed. |
| Open without restore | Disables restore replay for this entry and keeps the restore store read-only. | User-authored files, restore records, trust, credentials, durable indexes, and support export state. | Reopen with restore enabled after the unsafe replay reason is reviewed. |
| Cache/index repair | Disposes only declared disposable cache/index shards and schedules rebuild from authoritative state. | User-authored files, open buffers, trust, credentials, restore records, and support export state. | Rebuild cache/index and return to the rebuilt-index-ready posture. |

## Quarantine floor

Every runtime or extension quarantine must carry:

- quarantined lane ref;
- owner ref;
- reason class;
- expiry timestamp;
- release visibility;
- Project Doctor finding ref;
- restore conditions;
- explicit clear action;
- explicit re-enable action;
- redaction-safe evidence refs.

Expired or anonymous quarantines are validation failures. The support and
release projections include only metadata-safe evidence refs and exclude
raw private content, raw logs, command lines, credentials, and ambient
authority.

## Protected fixtures

The protected corpus includes:

- `safe_mode_crash_loop.yaml`;
- `extension_lane_quarantine.yaml`;
- `runtime_lane_quarantine.yaml`;
- `open_without_restore.yaml`;
- `cache_index_repair.yaml`.

The tests in
[`crates/aureline-support/tests/recovery_ladder_alpha.rs`](../../crates/aureline-support/tests/recovery_ladder_alpha.rs)
assert that these cases enter the expected rung, preserve user-owned
state, stop hidden restart cycling after the strike budget is exceeded,
and project support/release rows that explain quarantine owner, reason,
expiry, visibility, Doctor finding, restore conditions, and evidence.

## Surface contract

The first consuming surface is the support/release projection in the
support crate:

- `RecoveryLadderAlpha::evaluate` emits a decision record.
- `RecoveryLadderAlpha::support_packet` emits metadata-safe support rows.
- `RecoveryLadderAlpha::release_packet` emits metadata-safe release rows.

This is not a destructive reset flow. Cache/index repair is limited to
declared disposable state, open-without-restore preserves the restore
store, and quarantine does not remove the extension/runtime lane or
silently re-enable it on restart.
