# Save-Review Sheet

The save-review sheet is a protected review surface shown when Aureline refuses
to write the current buffer to disk because the pinned save target cannot be
proven safe to overwrite (for example, external drift, identity ambiguity, or a
review-required root policy gate).

This sheet exists to keep conflict resolution explicit and attributable: users
must be able to see what changed, which target would be affected, and which
destructive choice (if any) they are admitting.

## Contract sources

- `docs/ux/editor_external_change_contract.md` is the cross-surface contract for
  external changes, stale disclosure, and the review-choice matrix.
- `artifacts/fs/save_review_choice_matrix.yaml` freezes the choice keys and
  forbidden-reason vocabulary shared across save-conflict and external-change
  review surfaces.

## When it opens

The save-review sheet is opened from the real staged save coordinator when a
save attempt returns a review-required outcome such as:

- `external_change_detected`
- `save_conflict`
- `wrong_target_prevented`
- `watcher_uncertainty`
- `review_required_before_save`
- `review_required_before_rename`

## Minimum required content

The sheet MUST make the following facts visible without relying on hover-only
UI:

- **Target identity**: the presentation URI and canonical URI of the pinned save
  target.
- **Compare-before-write evidence**: the pinned generation token (at open/pin)
  versus the currently observed strongest generation token.
- **Diff availability and summary**: whether a text diff is available; when it
  is, a short summary and a small preview of changed lines.
- **Choice set**: the review-choice keys and whether each is enabled, including
  a forbidden reason when disabled.

## Choice keys

The choice vocabulary is closed and shared with the external-change contract:

- `compare`: review the external state and pin the basis for subsequent choices.
- `overwrite`: explicitly admit writing the current buffer over the observed
  external bytes (destructive).
- `merge`: resolve using a merge strategy when available (may degrade).
- `reload`: adopt the external bytes into the buffer (may degrade).
- `retry`: refresh the observed external state without committing bytes.
- `save_as`: export the buffer to a distinct target (may degrade).
- `cancel`: close the sheet without mutating durable bytes.

## Records and logging

The shell projects the sheet from a machine-readable record and writes the
record into `.logs/review_sheets/` for later support/export and recovery
provenance work.

Reference implementation:

- `crates/aureline-shell/src/save_review/mod.rs`
- `crates/aureline-shell/src/bootstrap/native_shell.rs`

