# Package-action, registry-auth, script-risk, and lockfile-impact contract

This document is the normative narrative seed for Aureline's
integrated package-action layer. It freezes one
`package_action_class` vocabulary (search / install / upgrade /
downgrade / remove / pin / audit / restore-from-checkpoint), one
`manifest_scope_class` vocabulary, one `registry_source_class`
vocabulary, one `registry_auth_mode_class` vocabulary, one
`script_risk_class` vocabulary, one `lockfile_impact_class`
vocabulary, one `transitive_impact_class` vocabulary, one
`rollback_posture_class` vocabulary, one `mirror_or_offline_state_class`
vocabulary, one `package_review_outcome_class` vocabulary, one
`package_action_record` shape, one `package_review_packet_record`
shape, and one `package_action_audit_event_record` shape that the
desktop shell, CLI / headless runner, AI broker package-aware tool,
extension-host package adapter, support / export reader, telemetry
sink, automation evidence projection, hosted-review reader, and the
admin / policy review surface all resolve against.

It exists so every later package surface (Cargo, npm / pnpm / yarn,
pip / uv / poetry, Go modules, Maven / Gradle, RubyGems / Bundler,
NuGet, system package adapters such as apt / dnf / brew / winget /
pacman, language-server install hooks, devcontainer feature
installers, and managed-mirror connectors) lands on one
review-aware vocabulary instead of inventing per-adapter
"installed", "blocked", "needs script", "lockfile changed",
"upgraded transitive" copy. Without this seed, each adapter would
grow a private notion of "is this risky?", a private "what will
the lockfile look like?" badge, a private rollback story, and a
private way to spell "this token came from a workspace dotfile".
Aureline cannot defend a desktop-first, CLI-equivalent,
enterprise-supportable package experience that way, and dependency
operations would degrade into ordinary low-risk metadata edits long
before later AI-driven and automation-driven package paths land.

Companion artifacts:

- [`/schemas/package/package_action.schema.json`](../../schemas/package/package_action.schema.json)
  — machine-readable boundary for `package_action_record` and the
  matched `package_action_audit_event_record`.
- [`/schemas/package/package_review_packet.schema.json`](../../schemas/package/package_review_packet.schema.json)
  — machine-readable boundary for the review packet that gathers
  lockfile-impact, transitive-impact, script-risk, registry-auth,
  mirror / offline, and rollback truth before any apply.
- [`/fixtures/package/package_action_cases/`](../../fixtures/package/package_action_cases/)
  — worked fixtures for individual-profile public-registry search,
  enterprise mirror install of a sandboxed-script package,
  managed-cloud upgrade across a major boundary with a transitive
  resolver conflict, air-gapped offline-bundle install of a native
  build, raw-secret-in-workspace denial, post-install-script
  unsandboxed denial-then-consent flow, lockfile-checkpoint rollback
  preview, and a destructive remove without checkpoint denial.

Upstream contracts this seed rides on:

- [`/docs/auth/system_browser_callback_packet.md`](../auth/system_browser_callback_packet.md)
  for the `account_free_local` / `self_hosted_org` /
  `managed_workspace` boundary the registry-auth vocabulary
  resolves under, including the local-only and
  managed-sign-in-required postures.
- [`/docs/governance/telemetry_and_support_schema_registry.md`](../governance/telemetry_and_support_schema_registry.md)
  for the consent / endpoint / retention class and support-export
  posture every `package_action_record` and review packet inherits.
- [`/docs/network/transport_governance_seed.md`](../network/transport_governance_seed.md)
  for the proxy-resolution mode, trust-store source, mirror-route
  class, offline / deny-all state, and `transport_posture` object
  every package action embeds at event time.
- [`/docs/architecture/standards_interchange_matrix.md`](../architecture/standards_interchange_matrix.md)
  for the standards posture (SemVer, SPDX / REUSE, OCI distribution,
  package-format families) the registry-source and lockfile-impact
  vocabularies cross-walk with rather than reinvent.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  for the secret-broker handle, redaction-class, and
  raw-secret-forbidden boundary every registry credential resolves
  against.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  for the approval-ticket envelope every mutation-class package
  action cites.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  for the freshness / client-scope / redaction vocabularies the
  package action and review packet re-export.
- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  for the workspace-trust state every package action and review
  packet honours.

Normative sources this seed projects from:

- `.t2/docs/Aureline_PRD.md` — the package-and-dependency
  experience and dependency-review requirements.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` — the
  package-adapter layer, the lockfile-impact computation, and the
  rollback posture.
- `.t2/docs/Aureline_Technical_Design_Document.md` — the
  per-adapter normalization and the registry-auth boundary.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — the package review
  surface, the script-risk disclosure, and the lockfile-diff
  presentation.

If this document disagrees with those sources, those sources win
and this document plus the companion schemas and fixtures update in
the same change. If this document and the schema disagree on an
enum or invariant, the schema wins and this document is updated in
the same change.

## Why this exists

A modern IDE that stays passive about package operations becomes a
launchpad for lockfile drift, supply-chain incidents, and silent
post-install scripts. Aureline integrates package operations into
the editor and the CLI, which means it MUST surface lockfile
impact, transitive impact, registry source, registry auth mode,
script / native-build risk, and rollback posture **before** apply
and MUST attribute them in support / export evidence afterwards.

This seed makes the package-facing rule explicit:

- Any package operation that changes dependency state (install,
  upgrade, downgrade, remove, pin, restore-from-checkpoint) MUST
  carry typed lockfile impact, typed transitive impact, and a
  typed rollback posture **before** apply. A surface that fires
  apply without those three values denies with
  `lockfile_impact_required_before_apply`,
  `transitive_impact_required_before_apply`, and
  `rollback_posture_required_before_apply` respectively.
- Script execution and native compilation MUST NOT be represented
  as ordinary metadata updates. Any record naming
  `lockfile_impact_class = no_lockfile_change_metadata_only` while
  `script_risk_class` is anything other than
  `no_script_no_native_build` or `declarative_metadata_only`
  denies with
  `script_or_native_build_must_not_be_represented_as_metadata_only`.
- Registry credentials MUST resolve to a secret-broker handle, a
  delegated-identity handle, a policy-injected credential, or a
  no-auth posture. Raw tokens, raw passwords, raw cookies, raw
  refresh tokens, raw client certificates, and raw `.npmrc` /
  `.pypirc` / `.cargo/credentials` token bodies never cross this
  boundary; a workspace-state row that holds a raw secret denies
  with `raw_secret_in_workspace_state_forbidden`.
- Mirror / offline state MUST be explicit. A package action minted
  while offline_or_deny_all_state is offline_grace_preserved,
  offline_air_gapped, deny_all_enforced, or
  network_disabled_by_user MUST cite a non-null
  `mirror_or_offline_state` block; otherwise it denies with
  `mirror_or_offline_state_required_when_offline_grace_or_air_gap_active`.
- Workspace trust and policy epoch MUST gate any mutation-class
  action. A row minted under
  `workspace_trust_unset_or_restricted` or under an expired policy
  epoch denies with the matching denial reason and never appears
  available on the review surface.

Live package-manager adapters, real lockfile resolvers, and
dependency-intelligence (vulnerability / license / freshness)
remain out of scope at this revision; the vocabulary and
invariants below are what those integrations will honour.

## Two questions the seed answers

Any Aureline surface claiming a package-aware behaviour MUST
answer both questions mechanically, without relying on
human-written copy:

1. **What is this action going to do to my project?** Which
   manifest, which registry source, which auth mode, which
   lockfile impact, which transitive impact, which script /
   native-build risk, which rollback posture, and which expected
   validation tasks (typecheck, build, test, license, audit) does
   this action imply?
2. **Under what trust posture is it being admitted?** Which
   workspace trust state, which policy epoch, which mirror /
   offline state, which approval ticket, which actor, which
   command, and which transport posture admitted this attempt?

Generic prose such as "installing", "updating dependencies",
"resolving", or "ready" is forbidden on these paths when a more
precise state is known.

## Frozen vocabularies

Every enum value below is reserved across this seed and its two
schema companions. Adding a value is **additive-minor** and
requires a `package_action_schema_version` /
`package_review_packet_schema_version` bump plus updates to the
schemas, this document, and a fixture exercising the new value in
the same change. Repurposing a value is **breaking** and requires
a new decision row in
`artifacts/governance/decision_index.yaml` co-signed by
`security_trust_review` and `product_scope_review`.

### `package_action_class` (8 values)

`search_for_package_metadata`,
`install_new_dependency`,
`upgrade_existing_dependency`,
`downgrade_existing_dependency`,
`remove_existing_dependency`,
`pin_existing_dependency`,
`audit_only_no_state_change`, and
`restore_lockfile_to_checkpoint`.

`search_for_package_metadata` and `audit_only_no_state_change` are
read-only; the schema enforces that they resolve to
`lockfile_impact_class = no_lockfile_change_metadata_only`,
`transitive_impact_class = no_transitive_change`, and
`rollback_posture_class = rollback_posture_not_applicable_read_only`.
The remaining six are mutation-class actions; the schema enforces
that they cite typed lockfile / transitive / rollback values.

### `manifest_scope_class` (7 values)

`workspace_root_manifest`,
`workspace_member_manifest`,
`monorepo_root_manifest`,
`application_only_dev_dependency_manifest`,
`vendored_or_offline_manifest`,
`out_of_tree_global_manifest_admin_only`, and
`manifest_scope_unknown_requires_review`.

`out_of_tree_global_manifest_admin_only` covers system-package
managers (apt / dnf / brew / winget / pacman) and language-toolchain
global installs that touch state outside the workspace; the schema
forces those to also cite a non-null
`out_of_tree_global_manifest_envelope` listing the OS-scope target
and admin actor / approval-ticket refs.

### `registry_source_class` (10 values)

`public_default_registry`,
`public_alternate_registry`,
`vendor_published_mirror`,
`customer_operated_mirror`,
`private_internal_registry`,
`managed_org_curated_registry`,
`offline_bundle_registry`,
`vendored_directory_no_registry`,
`git_or_path_dependency_no_registry`, and
`registry_source_unknown_requires_review`.

`vendored_directory_no_registry` and
`git_or_path_dependency_no_registry` resolve to
`registry_auth_mode_class = no_auth_public_registry` or
`mirror_or_offline_no_auth_required` and the schema forbids them
from carrying a `secret_broker_handle_ref` or
`delegated_identity_handle_ref`.

### `registry_auth_mode_class` (10 values)

`no_auth_public_registry`,
`secret_broker_handle_auth`,
`delegated_identity_auth`,
`policy_injected_credential_auth`,
`mirror_or_offline_no_auth_required`,
`managed_service_identity_auth`,
`mtls_client_certificate_auth`,
`device_flow_callback_auth`,
`registry_auth_unknown_requires_review`, and
`registry_auth_unsupported_blocked`.

The four secret-broker / delegated / policy-injected / managed
service identity values are the **preferred** auth modes. The
schema forbids `raw_secret_in_workspace_state` from appearing as
an auth mode; any record that supplies a
`raw_secret_in_workspace_state_observed` boolean equal to `true`
denies with `raw_secret_in_workspace_state_forbidden` and the
audit stream emits the matching denial event so a downstream
linter can demand migration to a broker handle.

### `script_risk_class` (8 values)

`no_script_no_native_build`,
`declarative_metadata_only`,
`post_install_script_runs_in_sandbox`,
`post_install_script_runs_unsandboxed_user_consent_required`,
`native_compilation_required_local_toolchain`,
`prebuilt_binary_with_runtime_loader`,
`platform_specific_binary_unverified_origin`, and
`script_risk_unknown_requires_review`.

The schema forbids the lockfile-impact-only collapse described
above:
`lockfile_impact_class = no_lockfile_change_metadata_only` paired
with `script_risk_class` outside the
`{no_script_no_native_build, declarative_metadata_only}` set
denies with
`script_or_native_build_must_not_be_represented_as_metadata_only`.
`platform_specific_binary_unverified_origin` is admissible only
when `workspace_trust_state` is `workspace_trust_trusted` and the
record cites a non-null approval ticket.

### `lockfile_impact_class` (9 values)

`no_lockfile_change_metadata_only`,
`new_lockfile_entries_added`,
`existing_lockfile_entries_updated`,
`lockfile_entries_removed`,
`lockfile_resolver_strategy_changed`,
`lockfile_format_migration`,
`lockfile_locked_unchanged_pinned_query_only`,
`lockfile_absent_will_be_created`, and
`lockfile_impact_unknown_requires_review`.

`lockfile_resolver_strategy_changed` covers cases where the
resolver itself (workspace-resolver-v2, npm `legacy-peer-deps`,
pip `--use-pep517`, Cargo `resolver = "2"`) changes between the
prior state and the proposed apply. `lockfile_format_migration`
covers a one-way file-format migration that itself becomes a
rollback obstacle.

### `transitive_impact_class` (8 values)

`no_transitive_change`,
`transitive_added_within_minor_floor`,
`transitive_added_across_major_boundary`,
`transitive_removed`,
`transitive_pinned_pattern_changed`,
`transitive_resolver_conflict_user_review_required`,
`transitive_circular_dependency_detected`, and
`transitive_impact_unknown_requires_review`.

`transitive_added_across_major_boundary` is the value the schema
forces on every record where a transitive crosses a SemVer major
boundary so the review packet cannot relabel a major bump as a
minor change.

### `rollback_posture_class` (7 values)

`rollback_via_lockfile_checkpoint`,
`rollback_via_workspace_snapshot_checkpoint`,
`rollback_requires_re_resolve_no_inline_checkpoint`,
`rollback_blocked_native_artifacts_must_be_recompiled`,
`rollback_blocked_post_install_script_was_unsandboxed`,
`rollback_unavailable_destructive_remove_with_no_checkpoint`, and
`rollback_posture_not_applicable_read_only`.

The schema forces `rollback_posture_not_applicable_read_only` on
read-only action classes and forbids it on every mutation-class
row.
`rollback_unavailable_destructive_remove_with_no_checkpoint` is
admissible only when the action class is
`remove_existing_dependency`; the review surface MUST disclose
this posture before apply, never after.

### `mirror_or_offline_state_class` (6 values)

`online_default_origin_admissible`,
`online_mirror_pinned_no_direct_origin`,
`offline_grace_window_using_warm_cache`,
`offline_air_gapped_using_offline_bundle_only`,
`network_disabled_user_setting_or_policy`, and
`mirror_or_offline_state_unknown_requires_review`.

This vocabulary cross-walks with the transport-governance seed's
`mirror_route_class` (the route the actual outbound traffic took)
and `offline_or_deny_all_state` (the global posture). The package
seed adds a per-action mirror / offline state because the packet
that the user reads MUST resolve mechanically without recomputing
the network record at render time.

### `package_review_outcome_class` (8 values)

`review_admitted_apply_proceeded`,
`review_admitted_apply_pending`,
`review_blocked_pending_user_decision`,
`review_blocked_pending_lockfile_resolution`,
`review_blocked_pending_native_build_consent`,
`review_blocked_pending_admin_policy`,
`review_blocked_pending_workspace_trust_elevation`, and
`review_dismissed_no_apply`.

A surface that admits apply without a `review_admitted_*`
outcome on the review packet denies with the matching audit-event
id and never produces a `package_action_record` with
`apply_outcome_class` outside the read-only set.

### `apply_outcome_class` (10 values)

`apply_completed_clean`,
`apply_completed_with_warnings`,
`apply_partial_failure_user_review_required`,
`apply_failed_no_state_changed`,
`apply_failed_state_changed_rollback_executed`,
`apply_failed_state_changed_rollback_unavailable`,
`apply_blocked_by_policy`,
`apply_blocked_by_workspace_trust`,
`apply_in_flight`, and
`apply_not_applicable_read_only`.

The schema forces read-only action classes to
`apply_not_applicable_read_only`, forces every terminal apply
outcome to cite `apply_completed_at`, and forces
`apply_failed_state_changed_rollback_executed` to cite a non-null
`rollback_record_ref`.

### `package_review_packet_class` (5 values)

`pre_apply_review_packet`,
`post_apply_evidence_packet`,
`rollback_preview_packet`,
`audit_only_review_packet`, and
`packet_class_unknown_requires_review`.

`pre_apply_review_packet` is the gating packet the review surface
displays before any mutation; `post_apply_evidence_packet` is the
packet support / export and admin-audit surfaces read after the
fact; `rollback_preview_packet` is the packet that materialises
the rollback posture before a restore-from-checkpoint action;
`audit_only_review_packet` is the read-only packet for
`audit_only_no_state_change` and `search_for_package_metadata`
actions.

### Audit-event id (16 values)

`package_action_admitted`,
`package_action_apply_completed`,
`package_action_apply_failed`,
`package_action_rolled_back`,
`package_action_blocked_pending_consent`,
`package_action_blocked_pending_policy`,
`package_action_blocked_pending_workspace_trust`,
`package_action_blocked_pending_lockfile_resolution`,
`package_review_packet_published`,
`package_review_packet_denied`,
`package_action_audit_denial_emitted`,
`package_action_raw_secret_observed_denial`,
`package_action_script_risk_misclassification_denial`,
`package_action_lockfile_impact_missing_denial`,
`package_action_transitive_impact_missing_denial`, and
`package_action_rollback_posture_missing_denial`.

### Denial-reason vocabulary (15 values)

`raw_secret_in_workspace_state_forbidden`,
`script_or_native_build_must_not_be_represented_as_metadata_only`,
`lockfile_impact_required_before_apply`,
`transitive_impact_required_before_apply`,
`rollback_posture_required_before_apply`,
`registry_source_must_resolve_to_typed_class`,
`registry_auth_mode_must_resolve_to_typed_class`,
`mirror_or_offline_state_required_when_offline_grace_or_air_gap_active`,
`delegated_identity_or_policy_injected_credential_preferred_over_raw_secret`,
`manifest_scope_must_match_workspace_layout`,
`workspace_trust_unset_or_restricted`,
`policy_epoch_expired_re_evaluation_required`,
`registry_unverifiable_user_review_required`,
`destructive_remove_without_checkpoint_must_disclose_rollback_unavailable`,
and
`out_of_tree_global_manifest_requires_admin_envelope`.

These denial reasons are the closed vocabulary every audit event
under `package_action_audit_denial_emitted` MUST cite. Generic
prose such as "blocked by review", "needs approval", or
"resolution failed" is forbidden on these paths.

## The package-action record

`package_action_record` is the one shape every package operation
emits. One record per attempted action (search, install, upgrade,
downgrade, remove, pin, audit, restore). Fields:

- Identity: `package_action_id`, `invocation_session_id`,
  `actor_ref`, `command_id_ref`, `surface_class` (one of
  `desktop_review_surface`, `cli_review_surface`,
  `ai_tool_review_surface`, `extension_host_review_surface`,
  `automation_run_review_surface`, `support_export_reader`,
  `admin_audit_reader`, `hosted_review_reader`).
- Classification: `package_action_class`, `manifest_scope`,
  `manifest_scope_envelope` (opaque manifest ref, opaque
  package-coordinate ref, optional out-of-tree-global envelope).
- Registry: `registry_source` (class plus opaque registry-endpoint
  ref, optional mirror-endpoint ref, optional vendored-directory
  ref).
- Auth: `registry_auth` (class, optional `secret_broker_handle_ref`,
  optional `delegated_identity_handle_ref`, optional
  `policy_injected_credential_ref`, `raw_secret_in_workspace_state_observed`
  flag for migration tracking).
- Risk: `script_risk_class`, `script_risk_envelope` (opaque
  script-descriptor ref, sandbox-policy ref, native-toolchain ref,
  optional consent-ticket ref).
- Lockfile: `lockfile_impact_class`, `lockfile_impact_envelope`
  (added / updated / removed entry counts as integer buckets, no
  raw entry bodies; opaque `prior_lockfile_snapshot_ref` and
  `proposed_lockfile_snapshot_ref`).
- Transitive: `transitive_impact_class`, `transitive_impact_envelope`
  (added / removed counts, major-boundary count, conflict count,
  no raw package coordinates).
- Validation: `expected_validation_tasks` — non-empty list drawn
  from `{typecheck, lint, build, unit_test, integration_test,
  license_audit, security_audit, dependency_audit, format_check}`
  the action expects to be re-run after apply.
- Rollback: `rollback_posture_class`, `rollback_posture_envelope`
  (opaque `lockfile_checkpoint_ref` or
  `workspace_snapshot_checkpoint_ref`, optional
  `rollback_blocker_note`).
- Mirror / offline: `mirror_or_offline_state` (class, opaque
  `mirror_endpoint_ref`, opaque `mirror_snapshot_ref`,
  `offline_since_at`).
- Authority: `workspace_trust_state`, `policy_epoch_ref`,
  `linked_approval_ticket_ref`,
  `linked_browser_handoff_packet_ref`,
  `linked_authority_ticket_ref`.
- Outcome: `apply_outcome_class`, `apply_outcome_note`,
  `apply_started_at`, `apply_completed_at`, optional
  `rollback_record_ref`.
- Linkage: `linked_package_review_packet_ref`,
  `linked_network_attribution_record_refs` (zero or more refs into
  `network_attribution_record`s the action emitted),
  `linked_run_record_ref`.
- Posture snapshot: `transport_posture_at_event`
  (the inspectable transport-posture object re-exported from the
  network seed by reference).
- Redaction: `redaction_class`, `export_safe`.
- Narrative refs.

Schema-level invariants (the schema's `allOf` gates):

1. Read-only action classes
   (`search_for_package_metadata`, `audit_only_no_state_change`)
   MUST resolve to
   `lockfile_impact_class = no_lockfile_change_metadata_only`,
   `transitive_impact_class = no_transitive_change`,
   `rollback_posture_class = rollback_posture_not_applicable_read_only`,
   and `apply_outcome_class = apply_not_applicable_read_only`.
2. Every mutation-class action MUST cite typed
   `lockfile_impact_class`, `transitive_impact_class`, and
   `rollback_posture_class` values **and** the matching envelopes
   MUST be non-null. Missing values deny with the matching denial
   reasons before apply.
3. `lockfile_impact_class = no_lockfile_change_metadata_only` paired
   with `script_risk_class` outside
   `{no_script_no_native_build, declarative_metadata_only}` denies
   with
   `script_or_native_build_must_not_be_represented_as_metadata_only`.
4. `registry_auth_mode_class = secret_broker_handle_auth` MUST
   carry a non-null `secret_broker_handle_ref`.
   `registry_auth_mode_class = delegated_identity_auth` MUST carry
   a non-null `delegated_identity_handle_ref`.
   `registry_auth_mode_class = policy_injected_credential_auth`
   MUST carry a non-null `policy_injected_credential_ref`.
   `registry_auth_mode_class = managed_service_identity_auth` MUST
   resolve only on `managed_workspace` identity mode.
5. `raw_secret_in_workspace_state_observed = true` is admissible
   only on `audit_only_no_state_change` actions and MUST emit a
   `package_action_raw_secret_observed_denial` audit event so the
   surface offers a migration path; on every other action class it
   denies with `raw_secret_in_workspace_state_forbidden`.
6. `mirror_or_offline_state_class` other than
   `online_default_origin_admissible` MUST be cited when
   `transport_posture_at_event.offline_or_deny_all_state` is
   anything other than `online_live_allowed` /
   `online_mirror_only`. A mismatch denies with
   `mirror_or_offline_state_required_when_offline_grace_or_air_gap_active`.
7. `workspace_trust_state` other than `workspace_trust_trusted`
   admits only read-only action classes and
   `restore_lockfile_to_checkpoint`; every other class denies with
   `workspace_trust_unset_or_restricted`.
8. `policy_epoch_ref` MUST be non-null when the active identity
   mode is `self_hosted_org` or `managed_workspace`; an expired
   policy epoch denies with
   `policy_epoch_expired_re_evaluation_required`.
9. Every mutation-class action MUST cite a non-null
   `linked_approval_ticket_ref` whenever the matched policy bundle
   requires an approval ticket; the schema does not encode policy
   bundles directly, but the row is rejected on the audit stream
   when the bundle requires a ticket and the row omits one.
10. `manifest_scope_class = out_of_tree_global_manifest_admin_only`
    MUST carry a non-null `out_of_tree_global_manifest_envelope`
    (admin actor, approval-ticket ref, scope locator) and is
    admissible only on `managed_workspace` or `self_hosted_org`
    identity modes.
11. `apply_outcome_class = apply_failed_state_changed_rollback_executed`
    MUST cite a non-null `rollback_record_ref` and a
    `rollback_posture_class` other than
    `rollback_unavailable_destructive_remove_with_no_checkpoint`.
12. `apply_outcome_class = apply_in_flight` is the only outcome
    that MAY leave `apply_completed_at` null; every other
    apply outcome MUST cite a non-null `apply_completed_at`.

## The package-review packet

`package_review_packet_record` is the one shape every review
surface (desktop card, CLI dependency-review summary, AI tool
review, hosted-review reader, support / export reader,
admin-audit reader) reads. One packet per action attempt or
audit cycle. Fields:

- Identity: `package_review_packet_id`,
  `bound_package_action_id_ref`, `package_review_packet_class`,
  `published_at`.
- Decision summary: `package_review_outcome_class`,
  `decision_note` (reviewable sentence), `decision_actor_ref`,
  `decision_command_id_ref`.
- Classification: `package_action_class` (mirrored from the
  bound action so the packet is self-describing).
- Risk projection: `script_risk_class`,
  `script_risk_disclosure` (reviewable sentence), optional
  `script_risk_consent_ticket_ref`.
- Lockfile projection: `lockfile_impact_class`,
  `lockfile_impact_disclosure` (reviewable sentence; raw lockfile
  bodies never appear), opaque `prior_lockfile_snapshot_ref`,
  opaque `proposed_lockfile_snapshot_ref`,
  added / updated / removed entry-count bucket fields.
- Transitive projection: `transitive_impact_class`,
  `transitive_impact_disclosure`, added / removed /
  major-boundary / conflict counts.
- Registry / auth projection: `registry_source_class`,
  `registry_auth_mode_class`, `mirror_or_offline_state_class`,
  reviewable `registry_disclosure` sentence.
- Validation: `expected_validation_tasks` (mirrored).
- Rollback projection: `rollback_posture_class`,
  `rollback_posture_disclosure`, opaque
  `rollback_checkpoint_ref` (when the rollback posture pins one).
- Authority: `workspace_trust_state`, `policy_epoch_ref`,
  `linked_approval_ticket_ref`,
  `linked_browser_handoff_packet_ref`,
  `linked_authority_ticket_ref`.
- Linkage: `linked_post_apply_evidence_packet_ref` (set on
  `pre_apply_review_packet` after apply lands a
  post-apply packet).
- Redaction: `redaction_class`, `export_safe`.

Schema-level invariants:

1. Pre-apply packets MUST resolve to a
   `package_review_outcome_class` other than
   `review_admitted_apply_proceeded`. `review_admitted_apply_proceeded`
   is admissible only on `post_apply_evidence_packet` and
   `audit_only_review_packet` (where it represents "no apply
   needed").
2. `pre_apply_review_packet` for a mutation-class action MUST cite
   typed `lockfile_impact_class`, `transitive_impact_class`,
   `script_risk_class`, and `rollback_posture_class` values; a
   missing value denies with the matching denial reason **before**
   the action enters apply.
3. `post_apply_evidence_packet` MUST cite a
   `linked_post_apply_evidence_packet_ref` of `null` (since it is
   itself the post-apply packet) and MUST cite an
   `apply_outcome_class` mirrored from the bound action.
4. `rollback_preview_packet` MUST resolve to
   `package_action_class = restore_lockfile_to_checkpoint`,
   `rollback_posture_class` in the rollback-via-checkpoint set
   (`rollback_via_lockfile_checkpoint` or
   `rollback_via_workspace_snapshot_checkpoint`), and MUST cite a
   non-null `rollback_checkpoint_ref`.
5. `audit_only_review_packet` MUST mirror a read-only action class
   and resolve to read-only lockfile / transitive / rollback /
   apply values.
6. Every packet MUST cite a `redaction_class` and an `export_safe`
   flag; raw lockfile bodies, raw manifest bodies, raw script
   bodies, raw native-toolchain command lines, raw registry URLs,
   raw registry credentials, and raw author identity strings never
   cross this boundary.

## Audit events

`package_action_audit_event_record` is the one audit-event shape
every package action and review packet emits. Fields:

- `audit_event_id` (one of the sixteen ids above).
- `bound_package_action_id_ref` and / or
  `bound_package_review_packet_id_ref`.
- `actor_ref`, `command_id_ref`,
  `linked_approval_ticket_ref`.
- `denial_reason_class` (required when `audit_event_id` is
  `package_action_audit_denial_emitted`,
  `package_action_raw_secret_observed_denial`,
  `package_action_script_risk_misclassification_denial`,
  `package_action_lockfile_impact_missing_denial`,
  `package_action_transitive_impact_missing_denial`,
  `package_action_rollback_posture_missing_denial`, or
  `package_review_packet_denied`).
- `event_note` (reviewable sentence).
- `recorded_at`.
- `redaction_class`, `export_safe`.
- `narrative_refs`.

## Desktop, CLI, AI-tool, automation, hosted-review, and admin parity

Every consumer reads this seed identically:

- Desktop and CLI emit `package_action_record` and
  `package_review_packet_record` in the same shape; a CLI bypass
  (for example "the CLI does not show transitive impact") is not
  admissible.
- AI tools that propose package actions emit a
  `pre_apply_review_packet` first; the user's confirmation produces
  the matching `package_action_record` with a non-null
  `linked_approval_ticket_ref` and a non-null
  `linked_run_record_ref` (the AI run record).
- Automation runs emit packets and records under their
  `automation_run_review_surface` surface class; the runtime
  authority ticket gates mutations.
- Hosted-review readers consume the same `post_apply_evidence_packet`
  via `linked_post_apply_evidence_packet_ref`; raw
  manifest / lockfile / token bodies never appear.
- Support / export and admin-audit readers receive the same packet
  shapes under their redaction class; the
  `signing_evidence_only` redaction class is reserved for the
  admin-audit / signing pipeline.

## Redaction and export posture

Every record and packet carries a `redaction_class` and an
`export_safe` flag. The four redaction classes
(`metadata_safe_default`, `operator_only_restricted`,
`internal_support_restricted`, `signing_evidence_only`) re-export
the support / release vocabulary by reference. Raw manifest
bodies, raw lockfile bodies, raw `package.json` /
`Cargo.toml` / `pyproject.toml` excerpts with secrets, raw
registry URLs, raw `.npmrc` / `.pypirc` /
`.cargo/credentials` token bytes, raw OAuth tokens, raw
device-flow codes, raw mTLS material, raw native-toolchain
command lines with environment variables, raw post-install
script bodies, raw vendored-directory absolute paths, and raw
author identity strings never cross this boundary.

## Out of scope at this revision

- Implementing per-adapter package managers (Cargo, npm / pnpm /
  yarn, pip / uv / poetry, Go modules, Maven / Gradle, RubyGems /
  Bundler, NuGet, system-package adapters).
- Implementing a real lockfile resolver, a real transitive-impact
  computation, or a real native-toolchain probe.
- Dependency intelligence (vulnerability / advisory / license
  / freshness scoring). Those are downstream contracts that read
  this seed's packet shape and do not redefine its vocabulary.
- A live registry-credential broker integration (the seed cites
  the secret-broker handle by ref; the broker itself is governed
  by ADR-0007).
- Full enterprise mirror provisioning, customer-managed mirror
  signing, or air-gap bundle production.

## Relationship to other seeds and ADRs

- ADR-0007 — credentials referenced via
  `secret_broker_handle_ref` are secret-broker handles under
  ADR-0007's redaction discipline. Raw secrets in workspace state
  deny with `raw_secret_in_workspace_state_forbidden`.
- ADR-0010 — mutation-class actions cite an approval-ticket ref
  on `linked_approval_ticket_ref` and (where a system browser is
  involved for device flow / OAuth) cite a browser-handoff
  packet ref on `linked_browser_handoff_packet_ref`.
- ADR-0011 — the freshness / client-scope / redaction
  vocabularies are re-exported by reference. Adding a new value
  follows the additive-minor lifecycle there.
- ADR-0018 — workspace-trust state gates every mutation-class
  action.
- Network governance seed — every package action that reaches a
  registry, mirror, or vendor host emits one or more
  `network_attribution_record` entries; the package action cites
  them on `linked_network_attribution_record_refs` and embeds the
  same `transport_posture` object.
- Telemetry-and-support schema registry — every
  `post_apply_evidence_packet` MUST be admissible under a
  registered telemetry / support row; the registry governs
  consent / endpoint / retention posture.
- Standards interchange matrix — registry sources align with the
  matrix's package-format rows; SemVer reasoning lives there.
- Capability-lifecycle contract — a future package-aware
  capability row (for example "BYO managed-mirror connector") is
  expected to consume this seed's packet shape rather than mint
  parallel vocabularies.

## Adding or changing vocabulary

Adding a value to any vocabulary is additive-minor:

1. Update the schema enum in
   `schemas/package/package_action.schema.json` and / or
   `schemas/package/package_review_packet.schema.json`.
2. Update this document.
3. Add or update a fixture under
   `fixtures/package/package_action_cases/` exercising the new
   value.
4. Bump `package_action_schema_version` /
   `package_review_packet_schema_version`.

Repurposing an existing value is breaking:

1. Open a decision row in
   `artifacts/governance/decision_index.yaml` co-signed by
   `security_trust_review` and `product_scope_review`.
2. Deprecate the old value and introduce the new value through an
   additive-minor landing.
3. Support / export rewriters and support bundles translate
   across the deprecation window.
