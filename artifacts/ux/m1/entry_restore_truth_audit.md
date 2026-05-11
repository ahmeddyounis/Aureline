# Entry / restore placeholder audit and truth-state copy review (reviewer entrypoint)

This page is the reviewer entrypoint for the unattended audit that
walks Aureline's protected first-run, open, import, and restore
dogfood paths and asserts every Start Center row, restore prompt,
recent-work card, and degraded-state placeholder quotes the agreed
truth vocabulary instead of hiding missing roots, stale restores,
partial recovery, or crash-recovery conditions behind ready-sounding
labels.

The audit packet — not screenshots or local notes — is what M1
review cites when checking that Aureline ships entry / restore copy
contracts and not ad hoc placeholders.

## What the audit covers

The protected walk runs over ten audited startup states, bundled into
four dogfood path classes:

- **first_run** — Start Center on a profile that has never opened
  Aureline. Verbs (Open / Clone / Import / Restore / Recent work)
  must render distinctly; no Get-started collapse; no marketplace-
  first card; no forced sign-in.
- **open** — explicit Open / Open without restore, warming, partial,
  offline, and unsupported startup states. Each names the blocked
  capability axis and the safe next action; degraded states keep
  per-row honesty (`remote_unreachable`, `authority_expired`,
  `policy_blocked_restore`) rather than an undifferentiated banner.
- **restore** — reopen with pending restore, restore failed, restore
  skipped. The advertised restore_level rides verbatim; failed or
  skipped restores never imply automated recovery completed; policy-
  skipped restores name the policy source.
- **placeholder_transition** — missing-extension zone slots, returned-
  focus placeholders, protocol-handler-reentry cards with unresolved
  target_kind. Each names its truthful cause, the resolving decision
  hook, and the blocked-capability axis.

For each case the audit verifies, by reading canonical sources:

1. The `startup_state_token` resolves to the closed vocabulary frozen
   in `artifacts/ux/startup_state_copy_review.yaml`; private state
   tokens are non-conforming.
2. The upstream seed under `fixtures/ux/entry_restore_states/`
   pins `overclaims_readiness: false`, names every required
   next-safe-action hook, blocked-capability token, recovery-ladder
   rung, support-packet family, journey class, and protected-metric
   ref the placeholder owes its user.
3. The matching `startup_state_copy_review_row` enumerates at least
   one forbidden label (the explicit "Workspace ready" / "Session
   restored" overclaim regressions reviewers spot at copy time) and
   shares the same hook / token / recovery / measurement linkage.
4. Each `dogfood_copy_snapshot` — the rendered Start Center row,
   restore prompt, recent-work card, or placeholder card that the
   dogfood shell paints for this state — does not contain any of the
   case's forbidden label fragments in the inspected fields. A
   misleading row is caught here, not in late design review.

## Protected walk (run unattended)

```bash
python3 tests/ux/entry_restore_copy_snapshots/run_entry_restore_copy_audit.py \
  --repo-root .
```

The runner emits a durable JSON capture at
`artifacts/milestones/m1/captures/entry_restore_truth_audit_validation_capture.json`
and exits non-zero on any regression. The capture records, per case,
the observed source-fixture summary, the copy-review row's hook /
forbidden-label set, and every dogfood snapshot scanned (path,
inspected fields, matches) so reviewers can see what the audit
actually saw — not just a pass / fail line.

## Failure drills (prove the lane fails loudly)

Every case under `fixtures/ux/entry_restore_placeholder_cases/`
declares one named failure drill with a forced input that mutates
the case state (drops a required hook, drops a blocked-capability
token, drops a recovery-ladder rung, drops a support-packet family,
forces `overclaims_readiness`, or injects a forbidden label into a
snapshot field). The runner asserts the audit reports the expected
`check_id`:

| Drill                                                  | Forced input                                                          | Expected check                                                |
| ------------------------------------------------------ | --------------------------------------------------------------------- | ------------------------------------------------------------- |
| `first_run_inject_workspace_ready_label`               | inject "Workspace ready" fragment into the first-run subtitle         | `entry_restore_copy_audit.forbidden_label.matched`            |
| `reopen_drop_open_without_restore_hook`                | drop `open_without_restore` next-safe action                          | `entry_restore_copy_audit.next_safe_action_hook.missing`      |
| `restore_failed_force_overclaim`                       | force `overclaims_readiness = true`                                   | `entry_restore_copy_audit.overclaim.detected`                 |
| `restore_skipped_drop_policy_audit_evidence`           | drop `policy_audit_evidence` support-packet family                    | `entry_restore_copy_audit.recovery_or_support_ref.missing`    |
| `open_without_restore_drop_index_block`                | drop `index_not_authoritative` blocked-capability token               | `entry_restore_copy_audit.blocked_capability_token.missing`   |
| `warming_startup_drop_semantic_lookups_pending`        | drop `semantic_lookups_pending` blocked-capability token              | `entry_restore_copy_audit.blocked_capability_token.missing`   |
| `partial_startup_drop_extension_quarantine_rung`       | drop `rung.extension_quarantine` recovery-ladder rung                 | `entry_restore_copy_audit.recovery_or_support_ref.missing`    |
| `offline_startup_drop_reauth_required_hook`            | drop `reauth_required` next-safe action                               | `entry_restore_copy_audit.next_safe_action_hook.missing`      |
| `unsupported_startup_drop_safe_mode_hook`              | drop `safe_mode` next-safe action                                     | `entry_restore_copy_audit.next_safe_action_hook.missing`      |
| `empty_state_drop_locate_missing_target_hook`          | drop `locate_missing_target` next-safe action                         | `entry_restore_copy_audit.next_safe_action_hook.missing`      |

To replay one:

```bash
python3 tests/ux/entry_restore_copy_snapshots/run_entry_restore_copy_audit.py \
  --repo-root . \
  --force-drill first_run_inject_workspace_ready_label
```

A drill that fails to surface its expected check is itself a failure
mode — the runner records it as
`entry_restore_copy_audit.failure_drill.expected_finding_missing`.

## Required coverage

The audit refuses to pass unless the case corpus collectively covers:

- dogfood path classes `first_run`, `open`, `restore`,
  `placeholder_transition`;
- entry verbs `open`, `clone`, `import`, `restore`.

## Dogfood snapshots scanned

| State                                          | Rendered surface fixture(s)                                                                                                        |
| ---------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `startup_state:first_run`                      | `start_center_first_run_no_account.json`, `first_run_start_center.json`                                                             |
| `startup_state:reopen_with_pending_restore`    | `start_center_restore_card_compatible.json`                                                                                         |
| `startup_state:restore_failed`                 | `start_center_restore_card_evidence_only.json`, `partial_restore_card_remote_dirty.json`                                            |
| `startup_state:restore_skipped`                | `start_center_unsupported_envelope.json`                                                                                            |
| `startup_state:open_without_restore`           | `restore_prompt_no_restore_first_launch.json`                                                                                       |
| `startup_state:warming_startup`                | `indexing_in_progress_search.json`                                                                                                  |
| `startup_state:partial_startup`                | `partial_restore_card_remote_dirty.json`                                                                                            |
| `startup_state:offline_startup`                | `start_center_offline_managed.json`, `remote_workspace_disconnected.json`                                                           |
| `startup_state:unsupported_startup`            | `start_center_unsupported_envelope.json`                                                                                            |
| `startup_state:empty_state_or_placeholder_transition` | `start_center_recent_work_missing_target.json`                                                                                |

## Storage / index

- Cases: `fixtures/ux/entry_restore_placeholder_cases/`
- Runner: `tests/ux/entry_restore_copy_snapshots/run_entry_restore_copy_audit.py`
- Capture: `artifacts/milestones/m1/captures/entry_restore_truth_audit_validation_capture.json`
- Proof packet: `artifacts/milestones/m1/proof_packets/entry_restore_truth_audit.md`
- Index: `artifacts/milestones/m1/artifact_index.yaml#entry_restore_truth_audit`

## Relationship to adjacent lanes

This audit is **complementary** to the existing M1 lanes; it does
not replace them:

- `token_motion_audit` — proves the protected shell surfaces still
  consume shared tokens, component states, and reduced-motion
  presets. This audit instead checks the *copy* — the placeholder
  vocabulary the user sees — across entry / restore states.
- `crash_restore` — proves the restore-fidelity drill matrix
  classifies restores honestly. This audit confirms the rendered
  copy (Start Center row, restore prompt, evidence-only card) names
  the resulting restore class verbatim.
- `start_center` (proof packet) — proves the Start Center surface
  pipes through the canonical entry / restore record. This audit
  pins the rendered labels and the closed forbidden-label set on
  the audited startup states.
