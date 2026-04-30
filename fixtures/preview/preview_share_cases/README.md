# Preview share-sheet, share-link, revoke, and expiry cases

Worked fixtures for the preview share-surface contract frozen in
[`/docs/preview/preview_share_contract.md`](../../../docs/preview/preview_share_contract.md).
Every fixture conforms to one of the two surface schemas:

- [`/schemas/preview/preview_share_sheet.schema.json`](../../../schemas/preview/preview_share_sheet.schema.json)
- [`/schemas/preview/preview_share_link.schema.json`](../../../schemas/preview/preview_share_link.schema.json)

The fixtures collectively exercise:

- a workspace-local-only anchor with no auth, no expiry, and no revoke
  path;
- an organization-only signed-in share routed to the managed
  organization endpoint with step-up auth and workspace-admin revoke;
- a temporary-external share with explicit-timestamp expiry,
  step-up auth, and user-self-revoke;
- a one-time-external share with one-time-token auth and
  one_time_use_expiry;
- a share sheet refused by share policy (policy_blocked_no_share_audience)
  with a typed mint_blocked_reason_class and a refusal disposition;
- a previously-active temporary-external share now revoked by the user
  with a typed terminal explanation and a regenerate path;
- a previously-active one-time-external share now expired (single-use
  consumed) with terminal_explanation_one_time_use_consumed;
- a previously-active organization-only share whose bound preview
  runtime restarted, transitioning to terminal_unavailable_runtime_restart
  with regenerate_admitted_after_runtime_restart;
- a previously-active temporary-external share whose bound device
  target changed, transitioning to terminal_unavailable_target_change
  with regenerate_admitted_after_target_rebind;
- a previously-active organization-only share bound to a captured
  replay snapshot whose mock-data set changed, transitioning to
  terminal_unavailable_stale_capture;
- a share-sheet record for the local-only anchor (sheet_admitted_minted_link
  disposition);
- a share-sheet record opened to regenerate from a revoked temporary-
  external link (sheet_admitted_regenerate_from_revoked).

## Intended usage

- **Schema conformance.** Every fixture MUST validate against the
  schema referenced in its `# yaml-language-server` header.
- **Vocabulary parity.** Fixtures reuse the share-mode, audience,
  destination, auth, expiry, revoke-path, export-posture,
  resolved-state, runtime-lineage, continuity, lifecycle,
  regenerate-path, terminal-explanation, revoke-actor, and
  revoke-reason vocabularies frozen by the contract.
- **Composition.** Fixtures cite the underlying preview-snapshot,
  preview-runtime-strip, hot-reload-state, and device-target
  descriptor records by opaque ref only; raw URLs, raw absolute paths,
  raw IP addresses, raw hostnames, raw bearer tokens, raw session
  cookies, raw expiring credentials, raw rendered bytes, raw stack
  frames, and raw mock-data bodies never appear.

## Fixtures

- [`share_link_local_only_anchor.yaml`](./share_link_local_only_anchor.yaml)
  — workspace-local-only anchor link, no auth / no expiry / no revoke,
  resolves to live runtime.
- [`share_link_organization_only_signed_in.yaml`](./share_link_organization_only_signed_in.yaml)
  — organization-only signed-in share, step-up auth, workspace-admin
  revoke, session-lifetime expiry, resolves to live runtime.
- [`share_link_temporary_external_expiring.yaml`](./share_link_temporary_external_expiring.yaml)
  — temporary external share, step-up auth, user self-revoke, explicit
  24-hour expiry.
- [`share_link_one_time_external.yaml`](./share_link_one_time_external.yaml)
  — one-time external share, one-time-token auth, one_time_use_expiry.
- [`share_link_revoked_with_regenerate.yaml`](./share_link_revoked_with_regenerate.yaml)
  — temporary external share revoked by the user; terminal_unavailable_revoked
  with regenerate_admitted_user_self.
- [`share_link_expired_with_regenerate.yaml`](./share_link_expired_with_regenerate.yaml)
  — one-time external share consumed; expired with
  terminal_explanation_one_time_use_consumed and a regenerate-through-approval
  path.
- [`share_link_runtime_restarted_terminal.yaml`](./share_link_runtime_restarted_terminal.yaml)
  — organization-only share whose runtime restarted; terminal_unavailable_runtime_restart.
- [`share_link_target_changed_terminal.yaml`](./share_link_target_changed_terminal.yaml)
  — temporary external share whose target changed; terminal_unavailable_target_change.
- [`share_link_stale_capture_terminal.yaml`](./share_link_stale_capture_terminal.yaml)
  — organization-only share bound to a captured replay whose data set
  changed; terminal_unavailable_stale_capture.
- [`share_sheet_policy_blocked_share.yaml`](./share_sheet_policy_blocked_share.yaml)
  — share-sheet refused by share policy; mint_admissible = false with
  blocked_share_policy_narrowed.
- [`share_sheet_local_only_open_proposal.yaml`](./share_sheet_local_only_open_proposal.yaml)
  — share-sheet for the local-only anchor proposal;
  sheet_admitted_minted_link.
- [`share_sheet_regenerate_after_revoke.yaml`](./share_sheet_regenerate_after_revoke.yaml)
  — share-sheet opened to regenerate from a revoked temporary-external
  link; sheet_admitted_regenerate_from_revoked with the predecessor
  link cited by ref.

## Related artifacts

- [`/schemas/preview/preview_snapshot.schema.json`](../../../schemas/preview/preview_snapshot.schema.json)
  — cross-surface preview-snapshot record share records project from.
- [`/schemas/preview/preview_runtime_strip.schema.json`](../../../schemas/preview/preview_runtime_strip.schema.json),
  [`/schemas/preview/hot_reload_state.schema.json`](../../../schemas/preview/hot_reload_state.schema.json),
  [`/schemas/preview/device_target_descriptor.schema.json`](../../../schemas/preview/device_target_descriptor.schema.json)
  — surface contracts share records cite by opaque ref only.
- [`/fixtures/preview/preview_runtime_surface_cases/`](../preview_runtime_surface_cases/)
  — preview-runtime strip / picker / hot-reload fixtures.
- [`/fixtures/preview/source_mapping_cases/`](../source_mapping_cases/)
  — preview-snapshot fixtures.
