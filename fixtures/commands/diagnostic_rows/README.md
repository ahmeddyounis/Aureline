# Command-diagnostic row fixtures

Worked examples for
[`/docs/ux/command_diagnostics_contract.md`](../../../docs/ux/command_diagnostics_contract.md)
and
[`/schemas/commands/diagnostic_projection.schema.json`](../../../schemas/commands/diagnostic_projection.schema.json).

Each fixture file carries exactly one of the three record kinds
the schema defines (`command_diagnostic_row_record`,
`protected_entry_badge_record`, `remediation_projection_record`).
Rows that reference a remediation or a protected-entry badge
point at a sibling fixture by opaque id; the id naming convention
is `diag-row:…`, `diag-badge:…`, and `diag-remediation:…`.

Raw hostnames, raw URLs, raw account handles, raw project
identifiers, raw command bodies, raw argument values, raw
prompt text, and raw credential material never appear in a
fixture; every field that resolves to a user-visible string is
carried as an opaque label ref that the docs / localisation
pipeline resolves.

## Cases

| File | Case | Acceptance note |
|------|------|-----------------|
| `diagnostic_row_local_only_enabled.json` | Baseline enabled local-only command | Renders across palette / menu / keybinding help / CLI help / AI tool for the same projection key; no badge. |
| `diagnostic_row_policy_blocked.json` + `remediation_policy_blocked.json` + `badge_provider_bearing_push.json` | Policy-blocked externally-visible mutation | `policy_blocked_in_context`; remediation is admin-only, non-dismissible; badge shows provider-bearing target. |
| `diagnostic_row_wrong_target.json` + `remediation_wrong_target.json` + `badge_managed_workspace_control.json` | Wrong-target managed workspace control | Command invoked on an individual workspace; badge shows managed-control-plane target; remediation routes to managed-fleet admin. |
| `diagnostic_row_provider_limited.json` + `remediation_provider_unlinked.json` + `badge_provider_bearing_publish.json` | Provider-limited publish | `required_provider_unlinked`; remediation repair hook is `request_provider_link`; badge names `installation_or_app_grant` origin. |
| `diagnostic_row_stale_context.json` + `remediation_stale_context.json` | Stale-context re-preview | `basis_snapshot_drifted`; remediation is `actionable_by_user` with follow-up deep link `prefill_command_invocation_preview`. |
| `diagnostic_row_missing_capability.json` + `remediation_missing_capability.json` + `badge_credential_broker_step_up.json` | Missing-capability credential broker step-up | `required_credential_missing`; badge is a credential-broker-scope target with `step_up_authentication_required` approval. |
| `diagnostic_row_reapproval_required.json` + `remediation_reapproval_required.json` + `badge_remote_attach.json` | Reapproval-required remote attach on reconnect | `approval_denial_no_approval_path` on a `restored_session_reconnect_route`; badge shows remote-agent target, delegated-user-token origin. |
| `diagnostic_row_terminal_paste_enabled.json` + `badge_terminal_paste.json` | Protected-entry terminal multiline paste | Enabled row with protected-entry badge; target is a local device shell, route is `editor_keybinding_route`. |
| `badge_tasks_entry_launch.json` | Protected-entry task launcher entry | Badge for a tasks entry point targeting a local task runner session; route is `palette_invocation_route`. |
| `badge_debug_entry_launch.json` | Protected-entry debug launcher entry | Badge for a debug entry point targeting a debug adapter session; route is `palette_invocation_route`. |
| `diagnostic_row_browser_handoff_enabled.json` + `badge_browser_handoff.json` | Protected-entry browser handoff | Enabled row with protected-entry badge; target is `browser_handoff_external_site`, origin is `delegated_user_token`. |

## Parity expectations

Every diagnostic row's `parity_surface_set` enumerates the
rendering surfaces that MUST render the row identically for the
same (command_id, command_revision_ref, policy_epoch,
trust_state, issuing_surface, ui_slot_class, execution_context_id)
tuple. A parity audit reading these fixtures verifies that the
fields listed in
[`/docs/ux/command_diagnostics_contract.md`](../../../docs/ux/command_diagnostics_contract.md)
under "Parity rules" match field-for-field across every
surface in the set.

Every protected-entry badge's `applicable_surface_classes`
enumerates the rendering surfaces that MUST render the badge
identically for the same (command_id, command_revision_ref,
target_identity_ref, origin_identity_ref) tuple.
