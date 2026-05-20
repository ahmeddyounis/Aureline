# High-authority entry interstitial — UX / security / a11y review

This artifact is the reviewer-facing projection of the high-authority entry
interstitial contract. It records how each cross-boundary entry path is
disclosed before execution, and it asserts the four honesty invariants on a row
set drawn from the canonical fixtures so UX, security/privacy, accessibility,
and support reviewers read the same truth.

- Runtime model: [`crates/aureline-shell/src/entry_interstitials/`](../../../crates/aureline-shell/src/entry_interstitials/)
- Boundary schema: [`schemas/workspace/entry_interstitial.schema.json`](../../../schemas/workspace/entry_interstitial.schema.json)
- Fixtures: [`fixtures/workspace/m3/high_authority_entry/`](../../../fixtures/workspace/m3/high_authority_entry/)
- Beta contract: [`docs/workspace/m3/high_authority_entry_beta.md`](../../../docs/workspace/m3/high_authority_entry_beta.md)

## Review rows

Each row is one canonical fixture. "Confirm command" is the
`canonical_command_ref`; the confirm action binds to exactly this id.

| Fixture | Kind | Source | Boundaries crossed | Target truth | Confirm command | Confirm enabled |
| --- | --- | --- | --- | --- | --- | --- |
| `protocol_deep_link_managed_open.json` | Deep link | System default browser | authority_widening, target_boundary, tenant_boundary, policy_boundary | exact_available | `cmd:workspace.restore_from_checkpoint` | Yes |
| `auth_callback_return.json` | Auth return | Auth provider callback | authority_widening | exact_available | `cmd:auth.complete_system_browser_return` | Yes |
| `collaboration_join.json` | Collaboration join | Collaboration service | authority_widening, target_boundary, tenant_boundary, remote_boundary | exact_available | `cmd:collaboration.join_session` | Yes |
| `remote_target_open_unreachable.json` | Remote open | System default browser | authority_widening, target_boundary, remote_boundary | remote_unreachable | `cmd:workspace.open_remote_target` | No (target unreachable) |
| `managed_resume.json` | Managed resume | Managed admin surface | authority_widening, target_boundary, tenant_boundary, policy_boundary | exact_available | `cmd:workspace.restore_from_checkpoint` | Yes |
| `notification_reopen_exact.json` | Notification reopen | OS notification | target_boundary, tenant_boundary, policy_boundary | exact_available | `cmd:notification.open_review` | Yes |
| `plain_local_open_fast_path.json` | Deep link (fast path) | OS file association | none | exact_available | `cmd:workspace.open_file` | n/a (no prompt) |

## Acceptance-criteria assertions

**No claimed beta high-authority entry path can execute silently from an OS
surface or browser callback.** Every interstitial row carries
`silent_execution_forbidden: true` and a three-action confirm/reject/defer set.
The only no-prompt row is `plain_local_open_fast_path.json`, which crosses no
boundary by construction.

**Users can tell why Aureline is asking and what happens on reject or defer.**
`confirm_explanation` names the source and the exact boundaries crossed;
`reject_outcome_label` states "Nothing opens and nothing changes";
`defer_outcome_label` states the request returns to the Start Center preserved,
not run.

**Notification reopen, deep-link open, and auth callback return preserve
exact-target truth or explain the fallback.** `reopens_generic_home` is `false`
on every row. When the target is exact (`notification_reopen_exact.json`,
`auth_callback_return.json`, the managed/deep-link opens) the exact object is
resolved. When it is not (`remote_target_open_unreachable.json`) an announced
`target_placeholder` preserves the intent with bounded fallbacks
(`reconnect_required`, `retry_later`, `return_to_start_center`) and the confirm
action is disabled with the typed reason `remote_unreachable`.

**Interstitial actions reuse the canonical command graph and cannot widen
authority.** On every row, `actions[confirm].command_id == canonical_command_ref`
and `authority_not_widened: true`. Reject binds to `cmd:workspace.entry.drop`
(no change); defer binds to `cmd:start_center.open_recent`.

## Accessibility notes

- Behavior binds to `action_key` + `command_id`, never to a label, so the
  confirm/reject/defer triad is keyboard- and screen-reader-addressable
  independent of visual order.
- Disabled confirm carries a typed `outcome_label` ("Cannot run yet: target is
  …") rather than a silent or ambiguous control, so assistive tech announces
  *why* the primary action is unavailable.
- Every placeholder is `announced: true`; a moved/missing/unreachable target is
  read out as a labeled state with named recovery actions, never an empty pane.

## Security / privacy notes

- The record and its support packet are redaction-safe: identities are opaque
  `object_identity_ref`s and all human text is producer-scrubbed. No raw URLs,
  paths, callback bodies, or credentials cross the boundary.
- A non-local origin is always disclosed via `source_class`; it can never bypass
  review by virtue of originating outside the shell.
- Support export carries the typed packet
  (`entry_interstitial_support_packet_record`) so route/origin incidents are
  reconstructable without transient UI scraping.
