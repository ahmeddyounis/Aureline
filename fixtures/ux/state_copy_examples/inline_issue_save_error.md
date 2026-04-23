# Fixture: inline issue — single-file save failure

## Scenario

The user saves a single file. The save participant (format on
save) fails mid-pipeline on this file, the content is preserved
in the recovery journal, and the row is marked with an inline
issue affordance. The shell MUST surface the failure on that
row only, preserve the last-failure reason on the row, and
expose a keyboard-reachable recovery path.

## Row bindings

- **Failure-tier row.** `tier.inline_issue` per
  [`failure_tier_matrix.yaml`](../../../artifacts/ux/failure_tier_matrix.yaml)
  § `failure_tier_rows`.
- **Empty-state intersection.** `empty:failed_last_attempt`
  when the row's derived content set (e.g., latest save output
  panel) is empty because of the failure.
- **Lifecycle row.** `lifecycle:workspace`, no controlled
  label change (workspace is still `workspace.ready`).
- **Truth class.** `user_authored_durable_truth` on the buffer
  (preserved via recovery journal);
  `runtime_observed_truth` on the save-participant status.
- **Degraded-state token.** `RetestPending` on the save status
  until the retry succeeds.

## Required axes rendered

- **Cause token.** `format_on_save_participant_failed`
  (referenced per the buffer / save contract).
- **Preserved.** Buffer contents; recovery journal; other
  files save normally; workspace posture is unchanged.
- **Narrowed.** This file's save is blocked; format-on-save
  step did not run.
- **Next-safe action hooks.** `review_archetype_match`
  (inspect the format-on-save participant),
  `review_trust_and_open` (reveal last-failure details),
  `continue_in_restricted_mode` (save without the format step
  for this file).

## Placement

- **Failure tier.** `tier.inline_issue`.
- **Delivery surface.** `inline_row_chrome` on the file's row
  in the explorer and the editor tab; `status_item` on the
  save pipeline surface.
- **Interruptibility tier.** `tier_ambient` on first failure;
  promotes to `tier_transient` if the failure repeats within a
  grouped-burst window.

## Recovery affordances (keyboard-reachable)

- **`Retry this`** — reruns save for this file.
- **`Reveal last failure`** — opens the last-failure inspector
  with the class/code ref, the participant id, and the monotonic
  timestamp.
- **`Save without participants`** — falls back to plain save,
  logging the skipped participant.
- **`Locate alternate target`** — when the failure is a target-
  path conflict, walks the user to resolution.

## Last-failure reason (mandatory visibility)

- **Keyboard-reachable reveal.** Inline affordance on the row
  named verbatim (`Reveal last failure`); reachable via
  `Tab` / `ArrowRight` through the row focus model.
- **Support-export field.** Included in the support bundle's
  `recovery_context` block. Preserves `class`/`code` per the
  error-class routing contract; preserves participant id,
  timestamp pair (monotonic + wall-clock), and the
  recovery-ladder rung history (`rung.none_required` on
  success-on-retry).
- **Export-safe description.** Redaction pass runs before bytes
  leave the product; raw external content is not included.

## Promotion triggers

- **To `tier.contextual_degraded`.** Three or more inline
  failures on distinct files within the grouped-burst window
  (dedupe via `grouped_burst_id`); OR the save participant
  subsystem reports a severity change to `degraded`.

## Recovery / support / measurement

- **Recovery-ladder rung.** `rung.none_required` on success-on-
  retry; `rung.cache_index_repair` if derived caches were
  implicated; `rung.extension_quarantine` if the failing
  participant is an extension.
- **Support-packet family.** `recovery_ladder_packet`,
  `performance_evidence_packet`.
- **Journey-trace class.** `open_edit_save`.

## Accessibility

- Screen-reader announces the failure class and the cause
  verbatim (no generic "Error").
- The reveal affordance is reachable without a pointer.
- Reduced-motion theme renders the same affordance without
  animated chrome.

## Forbidden copy and patterns on this path

- "Something went wrong"
- "Error"
- "Failed"
- "Try again" (without the typed retry route)
- Whole-file spinner while the inline failure is visible
- Toast-only rendering when the failure repeats (would trigger
  `toast_only_forbidden_for_durable_work`)

## Expected observable outcomes

- The failing row is inline-marked; other rows are unaffected.
- Last-failure reason is keyboard-reachable and preserved on
  support export.
- Repeated failures promote via the monotonic ladder (not a
  surprise modal).
- `overclaims_readiness = false` is asserted.

## Fixture fields (seed)

```yaml
__fixture__:
  name: inline_issue_save_error
  taxonomy_rows:
    - tier.inline_issue
    - empty:failed_last_attempt
    - lifecycle:workspace
  doc_section: docs/ux/state_and_recovery_taxonomy.md#7.1
running_build_identity_ref: build-identity-seed-state-inline-save-error
overclaims_readiness: false
```
