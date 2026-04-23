# Policy-bundle, offline entitlement-snapshot, grace-state, and admin-audit packet seed

This document freezes the shared object vocabulary Aureline uses for
signed policy bundles, offline-tolerant entitlement snapshots, grace
and expiry transitions, and admin-audit / admin-export packets. It
exists so the first identity lane lands on one object model instead of
each surface inventing its own "is the org online?" badge, its own
"what features still work offline?" clause, its own "why did this
narrow?" prose, and its own audit row shape.

Companion artifacts:

- [`/schemas/identity/policy_bundle.schema.json`](../../schemas/identity/policy_bundle.schema.json)
  — machine-readable boundary for `policy_bundle_record`.
- [`/schemas/identity/entitlement_snapshot.schema.json`](../../schemas/identity/entitlement_snapshot.schema.json)
  — machine-readable boundary for `entitlement_snapshot_record`.
- [`/schemas/identity/admin_audit_packet.schema.json`](../../schemas/identity/admin_audit_packet.schema.json)
  — machine-readable boundary for `admin_audit_packet_record`.
- [`/fixtures/identity/grace_state_cases/`](../../fixtures/identity/grace_state_cases/)
  — worked examples for account-free local inspect-only policy, managed
  policy refreshed live, stale-past-grace policy refused on managed
  widening, entitlement grace-state, entitlement grace-expired narrowed
  to local-safe, revocation-event admin audit, and org-switch admin
  audit.
- [`/docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md),
  [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md),
  [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md),
  [`/docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`](../adr/0015-embedded-surface-boundary-and-auth-handoff.md)
  — upstream identity, secret-broker, browser-handoff, and
  embedded-surface contracts this seed rides on.
- [`/schemas/security/emergency_action_record.schema.json`](../../schemas/security/emergency_action_record.schema.json)
  — upstream emergency-action and revocation object model. Policy
  bundle and entitlement snapshot revocations ride on the same
  revocation_record identity.
- [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  — the `entitlement_usage_export_packet` record class this seed's
  export-safe projections will be consumed by.

Normative sources this seed projects from:

- `.t2/docs/Aureline_PRD.md` local-first posture, account-free default,
  three identity modes, and the "managed outage narrows managed claims
  only" clause.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` identity,
  policy distribution, and offline-continuity appendices.
- `.t2/docs/Aureline_Technical_Design_Document.md` identity-mode,
  entitlement-snapshot, and admin-audit sections.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` identity, entitlement, and
  admin-console boundary guidance.

If this document disagrees with those sources, those sources win and
this document plus the schema updates in the same change.

## Why this exists

Local-first posture is only real if policy and entitlement are
distributed as signed, inspectable objects. Without one governed seed,
every later lane can silently turn every code path into a hosted-
console-required experience:

- a "managed capability" badge would claim live entitlement while the
  managed service is unreachable;
- a stale policy snapshot would silently grant a new managed privilege
  that the fresh bundle would have narrowed;
- an expired entitlement would hard-block local editing rather than
  narrow to the local-safe defaults documented by the snapshot;
- a grace-state banner would fork per surface, with each surface using
  its own copy and its own recovery path vocabulary;
- an admin-audit row would fragment into surface-local schemas with
  ambiguous tenant scope and unprincipled policy-epoch metadata;
- an enterprise narrowing decision would require a hosted admin
  console as the only source of truth, breaking self-hosted and air-
  gapped deployments; and
- an entitlement usage export would be describable only from billing-
  system prose instead of the signed snapshot the user held.

This seed closes those gaps before the first real OIDC, SCIM, policy-
authoring console, or billing integration lands. Live providers
remain out of scope at this revision; the vocabulary and invariants
below are what those integrations will honour.

## Three records, one identity graph

| Record                            | Purpose                                                                                                                                                                                                                                                    |
|-----------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `policy_bundle_record`            | Versioned, signed policy bundle with source, signer, scope selector, per-capability narrowing rules, distribution freshness, last-known-good linkage, revocation refs, downgrade-safety posture, local-only-lane posture, and explainability envelope.     |
| `entitlement_snapshot_record`     | Plan / feature / seat / quota snapshot with grace and expiry state, source / signer / last refresh, offline tolerance, per-feature local-safe-vs-managed-only posture, linked policy bundles, revocation refs, and explainability envelope.                |
| `admin_audit_packet_record`       | Typed admin-audit / admin-export packet carrying event class, decision reason, tenant scope, actor / target refs, policy + entitlement epoch, linked policy-bundle and snapshot refs, signer identity, distribution status, and explainability envelope.   |

All three records deliberately share:

- the identity-mode vocabulary (`account_free_local`, `self_hosted_org`,
  `managed_workspace`) re-exported from ADR-0001;
- the deployment-profile class (`individual_local`, `self_hosted`,
  `enterprise_online`, `managed_cloud`, `air_gapped`);
- the workspace-trust posture (`trusted`, `restricted`);
- the signer-identity / signer-continuity envelope;
- the distribution-status vocabulary for live / mirror / file-import /
  offline paths (re-exported from the emergency-action contract so a
  bundle, a snapshot, and an audit row carrying the same distribution
  id are one object, not three);
- the explainability envelope naming local-versus-vendor-console fields
  and acknowledged-gap waivers;
- the redaction class so support / admin / release surfaces share one
  redaction discipline;
- the export-safe label and opaque-ref discipline (raw SKUs, raw
  tenant names, raw user emails, raw directory attribute values, raw
  rule text, raw signatures, raw billing rows, and raw device
  fingerprints never appear on this boundary).

## Policy-bundle row anatomy

The policy bundle is the signed, inspectable object a self-hosted or
managed deployment distributes to Aureline clients. Each bundle carries
the axes below; reviewers change one axis at a time and diffs are
mechanical.

### Identity axes

- **`policy_bundle_id`** — stable opaque id preserved across `bundle_version`
  rolls within the same signer chain. A new id is minted only under
  cross-signed or broken continuity.
- **`bundle_version`** — `<bundle_name>@<semver>` identifier. Admin exports,
  support packets, and release evidence cite the exact version.
- **`policy_epoch`** — `policy_epoch.<lane>.<windowed>.<id>` monotonic epoch
  label. Epoch equality and "newer than" ordering are the only
  operations consumers perform.

### Provenance axes

- **`policy_source_class`** — closed enum: `customer_self_hosted_origin`,
  `vendor_managed_origin`, `signed_mirror_origin`, `manual_file_import_origin`,
  `air_gapped_transfer_origin`, `runtime_preload_origin`.
- **`signer_identity`** — signer authority class, opaque authority ref,
  continuity state, previous signer refs, continuity-statement ref,
  last-verified timestamp, signing-scope note.

Signer continuity is intentionally distinct from distribution
freshness. A mirrored snapshot may carry continuity proof while still
being past its freshness grace, and a fresh refresh may still fail
continuity review.

### Scope and narrowing axes

- **`scope_selector`** — one of `tenant_or_org_scoped`,
  `deployment_profile_scoped`, `install_profile_scoped`,
  `workspace_scoped`, `global_tenant_inherited`, `local_advisory_scope_only`.
  `local_advisory_scope_only` is the only admissible selector in
  `account_free_local` identity mode: the bundle is inspectable but it
  carries no authority over managed-gated features.
- **`narrowing_rules`** — per-capability rules. Each rule names
  `capability_class` (`settings_policy`, `workspace_trust_narrowing_policy`,
  `ai_capability_gating_policy`, `extension_publisher_policy`,
  `credential_store_policy`, `network_egress_policy`,
  `update_channel_policy`, `support_export_policy`, `sync_scope_policy`,
  `feature_flag_gating_policy`, `admin_console_explainability_policy`),
  `direction` (`narrows_only`, `widens_only_within_self_hosted_ceiling`,
  `informational_only`), whether downgrade reverses the narrowing,
  and a reviewable note.

The direction vocabulary is what keeps a `managed_convenience` layer
honest: it MAY narrow, and it MAY widen only within the ceiling the
`self_hosted_org` bundle already grants. A `widens_only` direction
that crosses the self-hosted ceiling is non-conforming at the record
level.

### Distribution axes

Every bundle carries one or more `distribution_status_record` rows so
an imported snapshot never pretends to be live-connected:

- **`source_class`** — `authoritative_origin`, `customer_managed_mirror`,
  `approved_private_mirror`, `manual_file_import`,
  `offline_transfer_snapshot`, `local_cache_projection`, `runtime_preload`.
- **`path_class`** — `managed_push`, `managed_pull`, `mirror_sync`,
  `file_import`, `offline_transfer`, `local_cache_projection`,
  `runtime_preload`.
- **`freshness_class`** — `authoritative_live`, `mirrored_current`,
  `mirrored_stale_within_grace`, `mirrored_stale_past_grace`,
  `manual_snapshot_current`, `manual_snapshot_stale`,
  `offline_snapshot_unexpired`, `offline_snapshot_expired`, `unknown`.
- **`validation_state`** — `verified_current`,
  `verified_but_stale_within_grace`, `verified_but_stale_past_grace`,
  `verification_failed`, `pending_review`, `cached_without_revalidation`.

A row with `freshness_class = authoritative_live` MUST carry
`validation_state = verified_current` or `pending_review`; the
combination "authoritative_live + verification_failed" is non-
conforming.

### Last-known-good linkage

The `last_known_good_linkage` envelope names the bundle the client
falls back to when a refresh fails: its ref, its version, its epoch,
the timestamp it was last verified, and a reviewable `fallback_posture_note`.
ADR-0001 pins that expired bundles degrade to the safe defaults
documented by the bundle itself rather than a hard lockout, unless
the bundle was explicitly configured to fail closed.

### Revocation axes

- **`revocation_refs`** — zero or more refs into
  `schemas/security/emergency_action_record.schema.json#revocation_record`.
  A bundle is considered revoked when any entry is present; the most
  recent by `revoked_at` is the one consumers render.
- **`revocation_reason_class`** — `signer_compromise_rotation`,
  `policy_authoring_mistake`, `tenant_wind_down`,
  `superseded_by_successor_bundle`, `cross_signed_transition_completed`,
  `emergency_policy_narrowing`, `mirror_integrity_failure`.

### Downgrade-safety and local-only-lane posture

- **`downgrade_safety_posture`** — `fail_safe_local_continuity`,
  `fail_safe_self_hosted_continuity`, `configured_fail_closed_restricted`,
  `configured_fail_closed_emergency_only`,
  `posture_unresolved_review_required`. A revoked bundle MUST declare
  a posture that does not introduce new managed narrowing on stale
  clients; `configured_fail_closed_*` values are admissible only when
  the bundle explicitly opts in.
- **`local_only_lane_posture`** — `local_only_safe_default`,
  `local_only_requires_explicit_opt_in`,
  `local_only_not_available_requires_managed`,
  `local_only_inspectable_only`, `local_only_ambiguous_requires_review`.
  Anything other than `local_only_safe_default` or
  `local_only_inspectable_only` requires a justification note so a
  reviewer can spot coercive or ambiguous defaults on the local-only
  lane.

### Explainability envelope

Every bundle names whether it is `locally_explainable_from_bundle`,
`locally_explainable_with_signed_evidence`,
`vendor_console_required_for_full_detail`, or
`vendor_console_only_acknowledged_gap`. The last value requires at
least one waiver ref into `artifacts/governance/exception_packet`; a
gap without a waiver is non-conforming at the record level.

## Entitlement-snapshot row anatomy

The entitlement snapshot is the signed, offline-tolerant object
describing plan, features, seat, quota, grace, and expiry. Snapshots
are not authorisation tickets; the policy bundle remains the authority
for narrowing decisions. The snapshot is what the desktop reads so
it can render feature availability honestly when the control plane is
stale, narrowed, or unreachable.

### Identity axes

- **`snapshot_id`** — stable opaque id for the snapshot lineage.
- **`entitlement_epoch`** — `entitlement_epoch.<lane>.<windowed>.<id>`.
  A snapshot whose epoch is older than the cached epoch is refused for
  any feature tagged `managed_only_requires_fresh_refresh`.

### Plan and feature axes

- **`plan_class`** — `account_free_local_plan`, `self_hosted_community_plan`,
  `self_hosted_enterprise_plan`, `managed_individual_plan`,
  `managed_team_plan`, `managed_business_plan`, `managed_enterprise_plan`,
  `evaluation_trial_plan`, `education_plan`, `non_profit_plan`.
  `account_free_local_plan` binds to `local_derived_account_free` source
  and carries no tenant ref.
- **`feature_rows[]`** — per-feature row with `feature_ref`, export-safe
  label, `capability_scope` (`desktop_core`, `local_ai_byok`,
  `workspace_tasks_trusted`, `extensions_catalog_view`,
  `extensions_install`, `managed_settings_sync`,
  `managed_marketplace_publish`, `managed_ai_quota`,
  `hosted_admin_console`, `hosted_fleet_admin`, `hosted_audit_stream`,
  `self_hosted_policy_authoring`, `self_hosted_scim_provisioning`),
  `availability_class`, and the two axes that keep the desktop honest:
  - `local_safe_vs_managed_only` — `local_safe_always`,
    `local_safe_when_trusted_workspace`, `local_safe_when_byok_configured`,
    `managed_only_when_signed_in`, `managed_only_requires_fresh_refresh`,
    `managed_only_requires_admin_console`.
  - `managed_narrowing_on_stale_snapshot` —
    `remains_available_local_continuity`, `narrows_to_inspect_only`,
    `narrows_to_read_only`, `pauses_with_visible_recovery`,
    `blocks_with_visible_repair_hook`.

A feature flagged `managed_only_requires_fresh_refresh` on a snapshot
whose `last_refresh.freshness_class` is `stale_past_grace` or
`offline_snapshot_expired` MUST mark itself
`unavailable_pending_entitlement_refresh` or `unavailable_revoked`.
That is the invariant that blocks "new managed privilege from stale
snapshots".

### Seat and quota axes

- **`seat_binding`** — `seat_state` from
  `not_applicable_account_free`, `seat_active`,
  `seat_active_pending_first_sign_in`, `seat_suspended_grace`,
  `seat_revoked`, `seat_deprovisioned_local_preserved`,
  `seat_unknown_refresh_required`, plus actor-subject ref, group
  refs, and a reviewable note. `seat_deprovisioned_local_preserved` is
  the ADR-0001 durable guarantee that managed deprovision preserves
  local workspace, local Git, local tasks, and BYOK AI capability.
- **`quota_rows[]`** — `quota_ref`, label, capability scope,
  `limit_class` (`unlimited_in_plan`, `fixed_monthly_limit`,
  `fixed_daily_limit`, `sliding_window_limit`, `concurrency_limit`,
  `pooled_org_limit`, `evaluation_trial_limit`,
  `managed_only_not_applicable_locally`), `usage_state` (`unused`,
  `within_soft_budget`, `within_hard_limit`, `approaching_limit`,
  `limit_exhausted`, `unknown_refresh_required`), and nullable
  `remaining_count` / `limit_count` / `window_resets_at`. A quota row
  in `unknown_refresh_required` MUST NOT expose counts derived from a
  stale snapshot.

### Grace, expiry, and offline tolerance

- **`grace_state`** — `reason_class` (`not_in_grace`,
  `managed_service_unreachable`, `policy_bundle_refresh_failed`,
  `billing_settlement_pending`, `seat_transition_pending`,
  `entitlement_signer_rotation_pending`, `air_gapped_offline_window`),
  started / expires timestamps, posture note, and
  `on_expiry_posture` from `narrow_to_local_safe_defaults`,
  `narrow_to_inspect_only_managed_features`,
  `pause_managed_only_features_with_visible_recovery`,
  `configured_fail_closed_restricted`. Desktop-core rows MUST expire
  into `narrow_to_local_safe_defaults` or
  `narrow_to_inspect_only_managed_features`.
- **`expiry_state`** — `effective_at`, `expires_at`,
  `review_deadline_at`, expiry note. `expires_at` is null only for
  `account_free_local_plan` and `self_hosted_community_plan`.
- **`offline_tolerance`** — `unlimited_offline_local_safe`,
  `bounded_offline_until_expiry`, `bounded_offline_signed_window`,
  `bounded_offline_air_gapped_window`,
  `refresh_required_before_new_managed_use`,
  `offline_not_permitted_managed_only_feature`.

### Source, signer, and last-refresh

- **`source_class`** — `customer_self_hosted_entitlement_origin`,
  `vendor_managed_entitlement_origin`, `signed_mirror_entitlement`,
  `manual_file_import_entitlement`, `air_gapped_transfer_entitlement`,
  `build_preload_entitlement`, `local_derived_account_free`.
- **`signer_identity`** — mirrors the policy bundle shape.
- **`last_refresh`** — `freshness_class` (`authoritative_live`,
  `refreshed_within_grace`, `stale_within_grace`, `stale_past_grace`,
  `offline_snapshot_unexpired`, `offline_snapshot_expired`,
  `never_refreshed_seed`, `not_applicable_account_free`),
  last-refreshed timestamp, refresh-attempt timestamp, next-refresh
  deadline, source class, reviewable note.

### Policy-bundle and revocation linkage

- **`policy_bundle_links[]`** — opaque refs into the bundles that
  authorise the snapshot's narrowing rules. Empty is admissible only
  for `local_derived_account_free` snapshots.
- **`revocation_refs[]`** — refs into
  `emergency_action_record#revocation_record`. A snapshot with any
  revocation ref MUST NOT claim `authoritative_live` freshness.

### Explainability envelope

Same shape as the policy bundle's envelope: `locally_explainable_from_snapshot`,
`locally_explainable_with_signed_evidence`,
`vendor_console_required_for_full_detail`,
`vendor_console_only_acknowledged_gap`. The gap posture requires at
least one waiver ref.

## Admin-audit packet row anatomy

The admin-audit packet is the typed row the admin audit stream, the
admin export surfaces, the support exporter, boundary-manifest tooling,
and release evidence read for identity-lane events. One packet per
event; fields are export-safe and reusable by each surface without
renaming.

### Event taxonomy

- **`audit_event_class`** — `policy_bundle_change`,
  `entitlement_lifecycle_transition`, `seat_action`, `device_action`,
  `revocation_event`, `org_switch`, `identity_mode_transition`,
  `admin_console_explainability_assertion`.
- **`decision_reason_class`** — one of 33 typed reasons (see the
  schema for the full enum). Every packet carries exactly one
  decision reason; the field is mandatory so release evidence and
  admin exports never fork on free-text prose. The enum spans policy
  authoring updates and rollbacks, signer rotations and compromise
  response, mirror integrity failures, tenant wind-downs and
  restructures, seat assign / reassign / revoke / pause / restore,
  device register / pause / revoke / forget, plan changes by admin or
  billing, quota reset / grant / revoke, entitlement refresh
  completed / declined-stale / revoked, policy-bundle revoked, org
  switch user-initiated / admin-initiated / degraded-into-local,
  identity-mode transitions in both directions, and the two
  explainability-assertion rows.

### Scope, actor, target axes

- **`tenant_scope`** — tenant-or-org ref, export-safe label,
  deployment-profile scope, install-profile-card refs, scope note.
  Tenant ref is null for `account_free_local` identity-mode packets.
- **`actor_ref`** — `actor_class` from `anonymous_local_user`,
  `authenticated_end_user`, `authenticated_local_admin`,
  `authenticated_org_admin`, `service_account_automation`,
  `vendor_managed_automation`, `signed_policy_bundle_source`,
  `signed_entitlement_source`, `system_scheduled_task`. Raw subject
  / session ids never appear.
- **`target_refs[]`** — at least one target, each with
  `target_kind` from `policy_bundle`, `entitlement_snapshot`,
  `seat_binding`, `device_record`, `tenant_or_org`, `workspace`,
  `install_profile_card`, `credential_store_lock_state`,
  `account_boundary`, `capability_scope`. The schema keeps each
  event class bound to the target kinds it MUST name (policy-bundle
  changes name a `policy_bundle`, entitlement lifecycle transitions
  name an `entitlement_snapshot`, device or seat actions name a
  `device_record` or `seat_binding`, org-switch and identity-mode
  packets name a `tenant_or_org` or `account_boundary`).

### Policy and entitlement epoch

- **`policy_epoch`** — opaque policy epoch at the time of the event.
- **`entitlement_epoch`** — opaque entitlement epoch at the time of the
  event.
- **`policy_bundle_links[]`** and **`entitlement_snapshot_links[]`** —
  refs into the policy-bundle and entitlement-snapshot rows the packet
  cites. Required non-empty for `policy_bundle_change` and
  `entitlement_lifecycle_transition` respectively.

### Before / after state

`before_after_state` carries export-safe before and after labels plus
a reviewable transition note. Raw before / after rule bodies, raw
directory attribute diffs, and raw session payload diffs MUST NOT
appear; only the reviewable labels and note.

### Distribution, signer, and explainability

- **`distribution_statuses[]`** — at least one, same shape as the
  bundle and the snapshot.
- **`signer_identity`** — same envelope as the bundle and the
  snapshot.
- **`explainability_envelope`** — same class and waiver-ref shape.
  `vendor_console_only_acknowledged_gap` requires at least one waiver
  ref; this is how the seed closes "enterprise claim requires hosted
  console as only source of truth".
- **`history_links`** — supersedes / superseded-by / related packet
  refs, plus support-packet, release-evidence-packet, and
  mutation-journal refs so a later reviewer can trace an admin
  decision across surfaces.

### Invariant: entitlement_refresh_declined_stale_snapshot

A packet whose `decision_reason_class` is
`entitlement_refresh_declined_stale_snapshot` MUST NOT claim
`authoritative_live` distribution; every distribution row carries a
stale or offline class. That is the invariant that blocks
"refresh-decline decisions masquerading as live refreshes" on the
admin audit stream.

## What the seed guarantees

Re-stating the acceptance claims from the task against the schema
invariants above:

- **Local-core use remains inspectable and non-blocking in stale or
  missing org context where promised.** `account_free_local`
  identity mode is pinned to `local_advisory_scope_only` policy bundle
  scope and `local_derived_account_free` entitlement source; every
  feature tagged `local_safe_always` / `local_safe_when_trusted_workspace`
  / `local_safe_when_byok_configured` keeps rendering under stale or
  missing org context. Policy-bundle expiry defaults to
  `fail_safe_local_continuity`; grace-state expiry defaults to
  `narrow_to_local_safe_defaults` for desktop-core rows.
- **New managed privilege cannot appear from stale snapshots or
  missing policy refresh.** The entitlement-snapshot schema blocks
  any feature tagged `managed_only_requires_fresh_refresh` from
  appearing `available_*` when `last_refresh.freshness_class` is
  `stale_past_grace` or `offline_snapshot_expired`. Quota rows in
  `unknown_refresh_required` are forbidden from exposing counts
  derived from a stale snapshot. Distribution rows with
  `authoritative_live` freshness must have `verified_current` or
  `pending_review` validation.
- **Packet fields are export-safe and reusable by support, admin,
  and boundary-manifest work.** Every field is an opaque ref, a
  typed enum value, an export-safe label, or a reviewable sentence;
  raw tokens, raw URLs, raw rule text, raw tenant names, raw user
  emails, raw directory attribute values, and raw signatures are
  forbidden on the boundary. History links carry support-packet,
  release-evidence-packet, and mutation-journal refs so a later
  surface resolves one admin event across multiple exports.
- **A policy or entitlement narrowing decision can be explained from
  the packet without privileged console-only state.**
  `local_vs_vendor_console_explainability_class` is frozen on every
  record; `vendor_console_only_acknowledged_gap` is admissible only
  with at least one waiver ref. `locally_inspectable_fields` is a
  required array; the reviewer can verify from the packet which
  fields a local CLI, a support export, or a self-hosted admin
  desktop can render without calling the vendor console.

## Out of scope at this revision

- Live OIDC, SCIM, or provider implementation. These schemas are
  what those integrations will honour, not the integrations
  themselves.
- Billing or line-item semantics. Raw SKUs, raw invoice ids, and raw
  billing ledger rows never cross this boundary.
- Policy-authoring console UI. Policy bundles are produced by the
  self-hosted or vendor-managed origin; the bundle shape is what
  this seed freezes.
- Credential-store secret handle classes (ADR-0007), browser-
  handoff / approval-ticket classes (ADR-0010), and embedded-surface
  boundary (ADR-0015). Those remain authoritative for their lanes;
  this seed cites them by opaque ref.
- Signing infrastructure implementation. Continuity statements and
  transparency-log anchors are cited by opaque ref; their production
  flow is an implementation follow-up.

## Change discipline

Adding a new enum value (a new `policy_source_class`, a new
`audit_event_class`, a new `decision_reason_class`, a new
`feature_availability_class`, a new `grace_reason_class`, and so on)
is **additive-minor**: consumers MUST forward-compat unknown values,
schema versions on the three records MUST bump, and the registry
entry in `docs/governance/telemetry_and_support_schema_registry.md`
is updated alongside.

Repurposing or removing an enum value is **breaking**: it requires a
new decision row co-signed by `security_trust_review`,
`product_scope_review`, and (for admin audit events and entitlement
snapshots) `compatibility_ecosystem_review`, and a superseding ADR.

The three records are the cross-tool boundary. The eventual
Rust-crate types remain the schema of record for in-process code;
this file and the schemas are what every cross-process consumer
reads.
