# Package-action and package-review packet worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/package/package_action_contract.md`](../../../docs/package/package_action_contract.md)
and the schemas at
[`/schemas/package/package_action.schema.json`](../../../schemas/package/package_action.schema.json)
and
[`/schemas/package/package_review_packet.schema.json`](../../../schemas/package/package_review_packet.schema.json).

Every file is a single JSON document carrying a `__fixture__`
prelude summarising the scenario, the contract sections it
exercises, the linked schemas, and the acceptance bullets it
backs. The runtime payload conforms to one of three shapes:

- `package_action_record` — one integrated package operation
  (search, install, upgrade, downgrade, remove, pin, audit,
  restore-from-checkpoint).
- `package_review_packet_record` — one pre-apply, post-apply,
  rollback-preview, or audit-only review packet that gates apply
  on lockfile / transitive / script-risk / rollback / mirror-or-
  offline truth.
- `package_action_audit_event_record` — one audit-stream entry
  (admit / apply / deny / rollback / consent step) on the
  `package_action` audit stream.

No fixture embeds raw manifest bodies, raw lockfile bodies, raw
post-install script bodies, raw native-toolchain command lines,
raw registry URLs, raw OAuth tokens, raw `.npmrc` / `.pypirc` /
`.cargo/credentials` token bytes, raw mTLS material, raw
vendored-directory absolute paths, or raw author identity strings.
Every such field is an opaque ref, an integer-bucket count, a
typed enum value, or a reviewable sentence.

## Cases

- [`individual_local_search_for_metadata.json`](./individual_local_search_for_metadata.json)
  — Read-only `search_for_package_metadata` action on an
  individual-local profile against a public default registry. No
  registry credential, no lockfile change, no transitive change,
  no rollback applicable. Validates that the schema admits
  read-only flows under `no_auth_public_registry` and that
  `apply_outcome_class` resolves to `apply_not_applicable_read_only`.
- [`self_hosted_org_install_secret_broker.json`](./self_hosted_org_install_secret_broker.json)
  — Mutation `install_new_dependency` on a self-hosted-org profile
  with `secret_broker_handle_auth` against a public alternate
  registry. Sandboxed post-install script; lockfile gains new
  entries; transitive set adds two within-minor packages;
  rollback via lockfile checkpoint; apply completes clean.
  Acceptance bullets 1, 2, 3.
- [`enterprise_online_install_policy_injected_native_build.json`](./enterprise_online_install_policy_injected_native_build.json)
  — Mutation `install_new_dependency` on an enterprise-online
  managed-workspace profile with `policy_injected_credential_auth`
  against a `customer_operated_mirror`. Native compilation
  required against the local toolchain; one transitive crosses a
  major boundary; rollback blocked because native artifacts must
  be recompiled; mirror is pinned (no direct origin admissible).
  Acceptance bullets 1, 2, 3.
- [`managed_cloud_upgrade_review_blocked_resolver_conflict.json`](./managed_cloud_upgrade_review_blocked_resolver_conflict.json)
  — Pre-apply `package_review_packet_record` for an
  `upgrade_existing_dependency` on managed-cloud against a
  `managed_org_curated_registry` under `delegated_identity_auth`.
  Lockfile would update entries; one transitive crosses a major
  boundary; one transitive surfaces a resolver conflict. The
  packet resolves to `review_blocked_pending_lockfile_resolution`
  so apply does not proceed. Acceptance bullets 1, 2, 3.
- [`air_gapped_install_offline_bundle.json`](./air_gapped_install_offline_bundle.json)
  — Mutation `install_new_dependency` on an air-gapped profile
  using an `offline_bundle_registry` and
  `mirror_or_offline_no_auth_required`. Native compilation
  required; rollback blocked because native artifacts must be
  recompiled; `mirror_or_offline_state_class` =
  `offline_air_gapped_using_offline_bundle_only`; transport
  posture `offline_air_gapped` is honoured. Acceptance bullets 1,
  2, 3.
- [`audit_only_raw_secret_observed.json`](./audit_only_raw_secret_observed.json)
  — `audit_only_no_state_change` action that observes a raw
  registry credential persisted in workspace state. The audit
  records `raw_secret_in_workspace_state_observed = true` (the
  only action class on which that flag is admissible) so a
  downstream linter can demand migration. Acceptance bullet 2.
- [`audit_raw_secret_denial_event.json`](./audit_raw_secret_denial_event.json)
  — `package_action_audit_event_record` carrying
  `package_action_raw_secret_observed_denial` with
  `denial_reason_class = raw_secret_in_workspace_state_forbidden`.
  Bound to the audit-only scan above. Acceptance bullet 2.
- [`unsandboxed_script_consent_review.json`](./unsandboxed_script_consent_review.json)
  — Pre-apply `package_review_packet_record` for an install whose
  post-install script demands an unsandboxed environment. The
  packet resolves to `review_blocked_pending_native_build_consent`
  and discloses
  `rollback_blocked_post_install_script_was_unsandboxed` so the
  user must explicitly admit a consent ticket before apply.
  Acceptance bullet 3.
- [`rollback_preview_lockfile_checkpoint.json`](./rollback_preview_lockfile_checkpoint.json)
  — `rollback_preview_packet` materialising a
  `restore_lockfile_to_checkpoint` plan. Pins the lockfile
  checkpoint, resolves to `review_admitted_apply_pending`, and
  carries `rollback_via_lockfile_checkpoint`. Acceptance bullet 1.
- [`destructive_remove_no_checkpoint_review.json`](./destructive_remove_no_checkpoint_review.json)
  — Pre-apply `package_review_packet_record` for a destructive
  `remove_existing_dependency` with no recoverable checkpoint.
  `rollback_posture_class` =
  `rollback_unavailable_destructive_remove_with_no_checkpoint`,
  outcome = `review_blocked_pending_user_decision`. Acceptance
  bullet 1.
- [`destructive_remove_no_checkpoint_denial_event.json`](./destructive_remove_no_checkpoint_denial_event.json)
  — `package_action_audit_event_record` carrying
  `package_action_audit_denial_emitted` with
  `denial_reason_class = destructive_remove_without_checkpoint_must_disclose_rollback_unavailable`
  for a downstream surface that attempted to apply the destructive
  remove without disclosure.
