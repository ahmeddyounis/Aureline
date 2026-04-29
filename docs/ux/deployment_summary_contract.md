# Deployment summary card, residual-dependency row, and mirror/offline artifact row contract

This document is the **cross-surface inspectability contract** for
Aureline's deployment summary truth. It freezes one
`deployment_summary_card_record`, one `residual_dependency_row_record`,
and one `mirror_offline_artifact_row_record` so the **deployment
class, tenant/org, region, mirror/offline state, control-plane
status, data-plane status, residual hosted dependencies, and
mirrored/offline artifact state** of a running Aureline install are
inspectable as one boundary record family rather than scattered
support notes.

The contract exists so an About panel, a diagnostics view, a support
packet, an admin-audit export, and a release-evidence excerpt all
project the same posture object without recomputing field names; so a
"self-hosted", "sovereign", "mirrored", "offline-capable", or
"managed" claim cannot be rendered with stronger independence than the
deployment actually has; and so a control-plane impairment cannot
silently collapse into generic workspace or runtime failure copy when
local-safe work and locally-mirrored artifacts remain.

The contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI/UX Spec, or design-system style guide, those sources win and this
document plus its schema and fixtures update in the same change.
Where About, diagnostics, support, admin-audit, or release-evidence
mints a parallel summary object, parallel residual-dependency view,
or parallel mirror/offline artifact list, this contract wins and the
surface is non-conforming.

## Companion artifacts

- [`/schemas/deployment/deployment_summary_card.schema.json`](../../schemas/deployment/deployment_summary_card.schema.json)
  — boundary schema for `deployment_summary_card_record`,
  `residual_dependency_row_record`, and
  `mirror_offline_artifact_row_record`.
- [`/fixtures/deployment/deployment_summary_cases/`](../../fixtures/deployment/deployment_summary_cases/)
  — worked YAML cases covering the five frozen deployment profiles,
  control-plane impairment with local-safe data plane, mirror-only
  posture with stale freshness labelling, air-gapped offline-bundle
  posture, and a managed-cloud baseline.

## Upstream contracts this contract rides on

This contract does **not** re-mint vocabulary that is already frozen
upstream; it consumes the frozen sets by name and by value:

- [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
  and [`/docs/governance/deployment_profile_truth.md`](../governance/deployment_profile_truth.md)
  — `deployment_profile_id` vocabulary
  (`individual_local`, `self_hosted`, `enterprise_online`,
  `air_gapped`, `managed_cloud`),
  `product_facing_label_vocabulary`,
  `tenant_org_scope_class_vocabulary`,
  `region_scope_class_vocabulary`,
  `retention_class_vocabulary`,
  `key_mode_class_vocabulary`, public-claim allowlist, and
  prohibited-implied-claim list. Summary cards re-export these field
  names byte-for-byte.
- [`/artifacts/governance/residual_dependencies.yaml`](../../artifacts/governance/residual_dependencies.yaml)
  — `dependency_class_vocabulary`, `posture_class_vocabulary`,
  `absence_impact_class_vocabulary`, and
  `continuity_fallback_class_vocabulary`. Residual-dependency rows
  cite ledger rows by `ledger_row_ref` and never paraphrase posture.
- [`/schemas/deployment/local_core_continuity_packet.schema.json`](../../schemas/deployment/local_core_continuity_packet.schema.json)
  and [`/docs/deployment/locality_and_continuity_seed.md`](../deployment/locality_and_continuity_seed.md)
  — `control_plane_service_class`, `control_plane_service_state_class`,
  `data_plane_capability_class`,
  `data_plane_capability_state_class`,
  `mirror_freshness_class`, `offline_import_source_class`, and
  `restore_class`. Summary cards re-export these field names verbatim
  and link the continuity packet that resolved against the same
  posture.
- [`/docs/ux/transport_and_environment_status_contract.md`](./transport_and_environment_status_contract.md)
  and [`/schemas/ux/transport_posture.schema.json`](../../schemas/ux/transport_posture.schema.json)
  — `offline_or_deny_all_state` vocabulary the
  `mirror_offline_state_class` field re-exports verbatim, plus the
  modal-prohibition rule for ambient explainer surfaces.
- [`/docs/ux/control_data_plane_status_contract.md`](./control_data_plane_status_contract.md)
  and [`/schemas/ops/outage_notice.schema.json`](../../schemas/ops/outage_notice.schema.json)
  — plane-separation rule and the local-continuity vocabulary the
  card cites when a control-plane outage is in flight.
- [`/docs/governance/deployment_profile_truth.md`](../governance/deployment_profile_truth.md)
  — change-trigger rules consumers re-evaluate when a card's
  posture changes between renders.

## Who reads this contract

- **About panel, diagnostics view, support-packet builder,
  admin-audit-export emitter, release-evidence excerpt builder, and
  CLI `--explain` text formatter** — to read **one** summary record
  family instead of recomputing fields.
- **Project doctor / attention inbox, status-bar deployment cell,
  workspace switcher, managed-cloud admin surface, and self-hosted
  operator console** — to render the same residual-dependency rows
  and mirror/offline artifact rows the support packet exports, with
  the same opaque refs.
- **Reviewers (release, security, accessibility, claim-manifest)** —
  to verify that `self_hosted`, `sovereign`, `air_gapped`,
  `mirrored`, and `managed_cloud` claims rendered against this
  record family cannot imply more independence than the underlying
  posture supports, that control-plane impairment cannot collapse
  into generic workspace/runtime failure, and that mirror/offline
  artifact state remains exportable to About, diagnostics, and
  support packets without widening redaction.

## Two questions the contract answers

Any Aureline surface claiming to expose current deployment truth MUST
answer both questions mechanically, without per-surface copy:

1. **What is the deployment posture right now?** Which deployment
   profile is in force? Which tenant/org scope, region scope,
   retention class, and key mode applies? Which mirror/offline
   posture is in force? What is the worst surviving control-plane
   service state, and what is the worst surviving data-plane
   capability state, separately? Which residual hosted or public
   dependencies remain in scope, and what does each of those
   dependent features narrow to when the dependency is unreachable?
2. **What is the trust state of every mirrored or offline artifact
   the deployment relies on?** For updates, extensions, docs packs,
   policy bundles, and models: who signed it, what is the current
   digest verification state, how fresh is the cached artifact, what
   is the offline-cache posture, and which scoped, inspect-only
   actions (verify, open-manifest) does the surface offer?

Generic prose like "everything is fine", "running on cloud",
"offline-ready", "always available offline", or "self-hosted means no
dependencies" is forbidden when a more precise posture and a more
precise dependency row are knowable. The schema enforces typed
vocabulary and typed sentences; surfaces render those values.

## 1. Scope

This contract freezes:

- One **deployment summary card record** (§3) emitted whenever an
  About, diagnostics, support, admin-audit, or release-evidence
  surface needs to project the deployment posture. The card carries:
  the `deployment_profile`, `product_facing_label_class`,
  `tenant_org_scope_class`, `region_scope_class`, `retention_class`,
  `key_mode_class`, `mirror_offline_state_class`, the
  `control_plane_state_summary` (worst surviving state plus the
  impaired service-class list), the `data_plane_state_summary`
  (worst surviving capability state plus the impaired and available
  capability-class lists), the `residual_dependency_row_refs`, the
  `mirror_offline_artifact_row_refs`, the `open_details_action`, the
  `prohibited_implied_claim_classes` the card is denied from
  surfacing, and the `freshness_posture_ref` resolved against the
  underlying continuity packet.
- The **residual-dependency row record** (§4) for every dependency
  whose posture is `required`, `optional`, `cached`, or `mirrored`
  on the rendered deployment profile. Forbidden and
  not-applicable-structural rows are surfaced separately so a
  reviewer can see what the profile excludes by construction. Each
  row names the `dependency_class`, the `posture_class`, the
  dependent feature, the `unreachable_impact_class`, the
  `continuity_fallback_class`, and the back-pointer
  `ledger_row_ref` into
  [`/artifacts/governance/residual_dependencies.yaml`](../../artifacts/governance/residual_dependencies.yaml).
- The **mirror/offline artifact row record** (§5) for the five
  artifact families this contract enumerates: `updates`,
  `extensions`, `docs_pack`, `policy_bundle`, `models`. Each row
  names the `signer_state_class` and opaque
  `signer_fingerprint_ref`, the `digest_state_class` and opaque
  `digest_ref`, the `mirror_freshness_class`, the
  `offline_cache_posture_class`, the `mirror_source_class`, the
  inspect-only `verify_action` and `open_manifest_action`, and the
  evidence refs into the continuity packet, mirror snapshot, or
  offline bundle that backs the row.
- The **cross-record invariants** (§6) so claims rendered against
  this record family cannot imply stronger independence than the
  underlying posture supports, control-plane impairment never
  collapses into workspace or runtime failure copy, and the
  mirror/offline artifact rows export verbatim into About,
  diagnostics, and support packets.

## 2. Out of scope

- The actual managed-service backend, signed-policy distribution
  pipeline, mirror server, offline-bundle build pipeline, model
  registry, extension marketplace, docs-pack publisher, and update
  channel. This contract pins the inspectable record family; it does
  not implement any of those services.
- Final user-facing copy and microcopy. This contract pins the
  closed sets the copy resolves against; the design-system style
  guide and shell-interaction-safety contract own the strings.
- Telemetry wire format, opaque-ref minting algorithm, and the
  diagnostics-bundle envelope. The schema registry rows for the
  About / diagnostics / support / admin-audit / release-evidence
  packet families consume this record family separately.
- The full claim-manifest evaluation engine. Claim rows continue to
  cite `deployment_profile_id` directly; this contract surfaces the
  posture every claim row resolves against without re-implementing
  claim evaluation.

## 3. The deployment summary card record

A deployment summary card is one structured projection of the current
deployment truth. The shell, About panel, diagnostics view, support
packet, admin-audit packet, and release-evidence excerpt each render
the same record without changing field names.

### 3.1 Required fields

- `record_kind = deployment_summary_card_record`.
- `deployment_summary_schema_version = 1`.
- `card_id` — opaque, stable, safe to log and export.
- `emitted_at` — RFC 3339 UTC timestamp from a monotonic clock.
- `deployment_profile` — re-export of
  `deployment_profile_id` from
  [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml).
  Closed set: `individual_local`, `self_hosted`,
  `enterprise_online`, `air_gapped`, `managed_cloud`.
- `product_facing_label_class` — re-export of
  `product_facing_label_vocabulary`. Closed set:
  `desktop_local_first`, `self_hosted_sovereign`,
  `hybrid_remote_attach`, `air_gapped_mirror_only`,
  `browser_companion_handoff_default_home`.
- `tenant_org_scope_class` — re-export from the locality posture.
- `region_scope_class` — re-export from the locality posture.
- `retention_class` — re-export from the locality posture.
- `key_mode_class` — re-export from the locality posture.
- `mirror_offline_state_class` — closed set (§3.3).
- `control_plane_state_summary` — see §3.4.
- `data_plane_state_summary` — see §3.5.
- `residual_dependency_row_refs[]` — opaque ref array into emitted
  `residual_dependency_row_record` rows. Empty only for the
  `individual_local` profile when no optional-cached dependency is
  in scope.
- `mirror_offline_artifact_row_refs[]` — opaque ref array into
  emitted `mirror_offline_artifact_row_record` rows. Required
  non-empty for the `air_gapped` profile and for any deployment
  whose `mirror_offline_state_class` is `online_mirror_only` or
  `offline_air_gapped`.
- `open_details_action` — see §3.6.
- `prohibited_implied_claim_classes[]` — closed set (§3.7) the card
  is **denied** from surfacing on this profile. Surfaces that render
  the card MUST NOT mint copy that asserts any class on this list.
- `freshness_posture_ref` — opaque ref into the underlying
  continuity packet's `freshness_posture`.
- `consumer_surfaces[]` — closed set of surfaces consuming the card
  (§3.8).
- `redaction_class` — one of `metadata_safe_default`,
  `operator_only_restricted`, `internal_support_restricted`,
  `signing_evidence_only`.
- `export_safe` — boolean. Required `true` when the card is exported
  through About, diagnostics, support, or admin-audit. The schema
  enforces that exported cards never widen redaction relative to the
  underlying continuity packet.

Optional fields: `linked_continuity_packet_ref`,
`linked_drill_refs[]`, `linked_outage_notice_refs[]`,
`linked_transport_posture_ref`, `notes`.

### 3.2 Cross-field locality posture rules

For `self_hosted`, `enterprise_online`, and `managed_cloud`
deployment profiles, the schema enforces that
`tenant_org_scope_class`, `region_scope_class`, and `key_mode_class`
are **actionable values** (not `not_applicable`). Silent
not-applicable values on these profiles mean the card is not
reviewable.

For `air_gapped`, `mirror_offline_state_class` MUST resolve to
`offline_air_gapped`; an `air_gapped` card that resolves to any
`online_*` state is non-conforming.

For `individual_local`, `tenant_org_scope_class` MUST be
`single_user_local` and `region_scope_class` MUST be
`not_applicable`.

### 3.3 `mirror_offline_state_class` (closed)

Re-export of `offline_or_deny_all_state` from the transport posture
contract, extended with `not_applicable` for completeness.

- `online_live_allowed` — direct egress permitted; live fetches are
  reachable.
- `online_mirror_only` — egress is mirrored; live origin fetch is
  not allowed and the mirror is the only compatibility surface.
- `offline_grace_preserved` — current network is unreachable but
  the policy/entitlement grace window has not expired.
- `offline_air_gapped` — public-internet egress is forbidden and
  the deployment runs against signed offline bundles only.
- `deny_all_enforced` — admin policy denies egress entirely.
- `network_disabled_by_user` — the user has explicitly disabled
  network access.
- `network_degraded_heuristic` — heuristic degradation; egress is
  technically possible but is being treated as unreliable.
- `not_applicable` — the rendered surface does not depend on a
  live or mirrored egress path (used only for
  `individual_local`).

### 3.4 `control_plane_state_summary`

The control-plane summary distils the per-service control-plane
state into a single inspectable cell **without** collapsing
service-level detail. Required fields:

- `worst_state_class` — one of `healthy`, `stale_cache`,
  `unavailable`, `mirror_only`, `boundary_recheck_required`,
  `not_applicable`. Re-export of
  `control_plane_service_state_class`. The worst state across all
  in-scope service classes.
- `summary_label` — short reviewable sentence in product terms.
  Generic "service degraded" copy is non-conforming.
- `impaired_service_classes[]` — array of
  `control_plane_service_class` values that are not in `healthy`
  state. Empty only when `worst_state_class = healthy`.
- `healthy_service_classes[]` — array of
  `control_plane_service_class` values reporting `healthy` state.
- `service_state_refs[]` — opaque refs into the per-service
  control-plane state rows on the underlying continuity packet.

### 3.5 `data_plane_state_summary`

The data-plane summary distils the per-capability data-plane state
into a single inspectable cell. Required fields:

- `worst_state_class` — one of `available_local_safe`,
  `available_mirror_backed`, `reduced_read_only`,
  `blocked_pending_reconnect`,
  `blocked_pending_boundary_recheck`, `blocked_by_policy`,
  `not_applicable`. Re-export of
  `data_plane_capability_state_class`. The worst state across all
  in-scope capability classes.
- `summary_label` — short reviewable sentence in product terms.
- `impaired_capability_classes[]` — array of
  `data_plane_capability_class` values not in
  `available_local_safe`.
- `available_local_safe_capability_classes[]` — array of
  `data_plane_capability_class` values reporting
  `available_local_safe`. The schema enforces this list is
  non-empty whenever the underlying continuity packet retains any
  local-safe capability — a control-plane outage cannot collapse
  this list to empty.
- `capability_state_refs[]` — opaque refs into the per-capability
  data-plane state rows on the underlying continuity packet.

### 3.6 `open_details_action`

Every card MUST carry exactly one open-details action. Required
fields:

- `action_id` — opaque id.
- `label` — short reviewable label (e.g. "Open deployment details").
- `target_route_ref` — opaque ref to the detail route.
- `scope_class` — `scope_local_only`. The open-details action
  cannot reach beyond the local device.
- `authority_class` — `user_local_authority`.
- `consent_class` — `no_consent_required_safe_default`.
- `side_effects` — exactly `["no_side_effect_inspect_only"]`.
- `preserves_evidence_context` — `true`.
- `revalidation_on_open` — one of `none_already_fresh`,
  `snapshot_open_read_only`. Generic refresh-on-open is non-
  conforming because the card already serializes the resolved
  posture.
- `modal_prohibited` — `true`. Cards never raise a modal from the
  open-details action; the action navigates or expands inline.

### 3.7 `prohibited_implied_claim_class` (closed)

Closed set of independence-flavoured claims a surface MAY NOT mint
when rendering the card on a posture that does not support them.
Each card cites the classes that are **denied** for its current
posture so a reviewer can confirm the claim guardrail without
reading per-profile prose.

- `implied_air_gapped_when_egress_allowed` — the surface MAY NOT
  imply air-gapped status when `mirror_offline_state_class` permits
  any form of online egress.
- `implied_sovereign_when_vendor_managed` — the surface MAY NOT
  imply sovereignty when `key_mode_class = vendor_managed` or any
  required residual dependency is vendor-hosted.
- `implied_self_hosted_when_managed_cloud` — the surface MAY NOT
  imply self-hosted operation when the profile is `managed_cloud`
  or `enterprise_online` with vendor control plane in scope.
- `implied_no_residual_dependency_when_required_present` — the
  surface MAY NOT imply zero residual dependency when any
  `residual_dependency_row_record` carries
  `posture_class = required` and
  `vendor_or_public_dependence = true`.
- `implied_offline_parity_when_mirror_only` — the surface MAY NOT
  imply offline parity (works the same offline as online) when
  `mirror_offline_state_class = online_mirror_only` or any mirror
  artifact row carries a non-`mirror_fresh_within_window` freshness
  class.
- `implied_managed_independence_when_local_dependent` — the
  surface MAY NOT imply that a managed-cloud deployment is
  independent of local artifacts when an offline cache or local
  policy bundle is required for the rendered claim.
- `implied_always_fresh_when_bounded_or_unbounded_stale` — the
  surface MAY NOT imply always-fresh artifacts when the
  freshness posture resolves to `bounded_stale` or
  `unbounded_stale`.

### 3.8 `consumer_surface_class` (closed)

Closed set of surfaces consuming the card.

- `about_panel` — desktop About panel and CLI `--about` text.
- `diagnostics_view` — desktop diagnostics view and CLI
  `--diagnose` text.
- `support_packet_export` — support-bundle export.
- `admin_audit_export` — admin-audit export.
- `release_evidence_excerpt` — release-evidence packet section.
- `status_bar_cell` — status-bar deployment cell projection.
- `companion_surface` — browser-companion summary projection
  (only valid when the profile's
  `companion_surface_posture_class` permits a companion render).
- `cli_text_formatter` — CLI text formatter render.

A `companion_surface` consumer on a profile whose
`companion_surface_posture_class` is
`companion_handoff_explicitly_disallowed` is non-conforming and
schema-denied.

## 4. The residual-dependency row record

The residual-dependency row is one structured projection of one
ledger row resolved against the rendered deployment profile. The
shell, About panel, diagnostics view, support packet, admin-audit
packet, and release-evidence excerpt each render the same row
without changing field names.

### 4.1 Required fields

- `record_kind = residual_dependency_row_record`.
- `residual_dependency_row_schema_version = 1`.
- `row_id` — opaque, stable.
- `card_id_ref` — opaque ref into the parent
  `deployment_summary_card_record`.
- `dependency_class` — re-export of
  `dependency_class_vocabulary`. Closed set: `sign_in`,
  `package_registry`, `remote_mirror`, `remote_agent`,
  `symbol_service`, `ai_provider`, `policy_bundle`, `docs_pack`,
  `browser_handoff`, `companion_notification_channel`,
  `hosted_control_plane_reachability`.
- `posture_class` — re-export of `posture_class_vocabulary`.
  Closed set: `required`, `optional`, `cached`, `mirrored`,
  `forbidden`, `not_applicable_structural`.
- `vendor_or_public_dependence` — boolean. Required `true` when
  the dependency resolves against a vendor-hosted, public-internet,
  or vendor-published mirror surface; required `false` when the
  dependency resolves entirely on customer-operated infrastructure
  (customer mirror, customer IdP, customer policy distribution),
  is `forbidden`, or is `not_applicable_structural`.
- `dependent_feature_label` — short reviewable sentence in product
  terms naming what feature depends on the dependency. Generic
  "all features" copy is non-conforming.
- `dependent_feature_refs[]` — opaque refs into the boundary
  manifest, claim manifest, qualification matrix, or capability
  registry rows that name the dependent features.
- `unreachable_impact_class` — re-export of
  `absence_impact_class_vocabulary`. Closed set:
  `no_impact_capability_not_claimed_for_profile`,
  `narrows_to_local_core_capabilities`,
  `narrows_to_mirror_backed_read_only`,
  `narrows_to_cached_last_known_good`,
  `narrows_to_review_only_boundary_recheck`,
  `blocked_pending_reconnect`,
  `blocked_pending_boundary_recheck`,
  `blocked_pending_mirror_refresh`, `blocked_by_policy`,
  `fail_closed_forbidden_in_profile`.
- `unreachable_impact_label` — short reviewable sentence in
  product terms describing what happens to the dependent feature
  when the dependency is unreachable.
- `continuity_fallback_class` — re-export of
  `continuity_fallback_class_vocabulary`. Closed set:
  `continue_local_no_restore`, `replay_cached_snapshot`,
  `mirror_snapshot_import`, `resume_after_reconnect`,
  `manual_reconcile_after_boundary_change`,
  `fail_closed_no_fallback`, `not_applicable_structural`.
- `ledger_row_ref` — opaque ref into the matching row in
  [`/artifacts/governance/residual_dependencies.yaml`](../../artifacts/governance/residual_dependencies.yaml).
- `freshness_label` — short reviewable sentence describing how
  fresh the dependency's last-known-good state is. Required
  whenever `posture_class` is `cached` or `mirrored`.
- `redaction_class` — one of the four standard classes.
- `export_safe` — boolean.

Optional fields: `linked_outage_notice_ref`,
`linked_continuity_packet_ref`, `evidence_links[]`, `notes`.

### 4.2 Posture / impact pairing rules

The schema enforces these pairings between `posture_class`,
`vendor_or_public_dependence`, and `unreachable_impact_class`:

- `posture_class = forbidden` rows MUST set
  `vendor_or_public_dependence = false` and
  `unreachable_impact_class = fail_closed_forbidden_in_profile`.
  Forbidden dependencies are not residual; they are excluded by
  construction.
- `posture_class = not_applicable_structural` rows MUST set
  `vendor_or_public_dependence = false` and
  `unreachable_impact_class = no_impact_capability_not_claimed_for_profile`.
- `posture_class = required` rows on `self_hosted`,
  `enterprise_online`, or `managed_cloud` profiles MUST set
  `vendor_or_public_dependence = true` whenever the dependency
  class is `ai_provider`, `browser_handoff`,
  `companion_notification_channel`, or
  `hosted_control_plane_reachability` (vendor-bound by ledger);
  the schema rejects a managed-cloud claim that names a vendor
  dependency as customer-operated.
- `posture_class = cached` rows MUST carry a non-null
  `freshness_label` and an `unreachable_impact_class` of
  `narrows_to_cached_last_known_good` or
  `blocked_pending_reconnect`.
- `posture_class = mirrored` rows MUST carry a non-null
  `freshness_label`, an `unreachable_impact_class` of
  `narrows_to_mirror_backed_read_only` or
  `blocked_pending_mirror_refresh`, and a
  `continuity_fallback_class` of `mirror_snapshot_import`.

These pairings keep "still vendor-hosted or public-service
dependence" inspectable. A surface that paraphrases posture as
prose without citing a ledger row and one of the typed impact
classes is non-conforming.

## 5. The mirror/offline artifact row record

The mirror/offline artifact row is one structured projection of one
mirrored or offline artifact family. The shell, About panel,
diagnostics view, support packet, admin-audit packet, and
release-evidence excerpt each render the same row without changing
field names.

### 5.1 Required fields

- `record_kind = mirror_offline_artifact_row_record`.
- `mirror_offline_artifact_row_schema_version = 1`.
- `row_id` — opaque, stable.
- `card_id_ref` — opaque ref into the parent
  `deployment_summary_card_record`.
- `artifact_class` — closed set (§5.2).
- `artifact_label` — short reviewable label in product terms.
- `signer_state_class` — closed set (§5.3).
- `signer_fingerprint_ref` — opaque fingerprint of the signing
  trust root. Required non-null whenever `signer_state_class` is
  `signed_trust_root_pinned`, `signed_offline_trust_root`, or
  `signed_org_ca_pinned`.
- `digest_state_class` — closed set (§5.4).
- `digest_ref` — opaque digest. Required non-null whenever
  `digest_state_class` is `digest_verified` or `digest_pending`.
- `mirror_freshness_class` — re-export of
  `mirror_freshness_class` from the local-core continuity packet.
  Closed set: `mirror_fresh_within_window`,
  `mirror_within_extended_window`, `mirror_past_extended_window`,
  `mirror_freshness_unknown`, `not_applicable`.
- `last_refresh_at` — RFC 3339 UTC timestamp from a monotonic
  clock. Nullable; required non-null whenever
  `mirror_freshness_class` is anything other than
  `mirror_freshness_unknown` or `not_applicable`.
- `offline_cache_posture_class` — closed set (§5.5).
- `mirror_source_class` — re-export of `mirror_source_class` from
  the local-core continuity packet. Closed set:
  `customer_operated_mirror`,
  `vendor_published_mirror_for_customer`,
  `offline_bundle_derived_mirror`, `not_applicable`.
- `verify_action` — see §5.6.
- `open_manifest_action` — see §5.6.
- `evidence_links[]` — opaque refs into the underlying mirror
  snapshot, offline bundle, signed manifest, support-packet
  excerpt, claim-manifest entry, or compatibility-report entry.
- `redaction_class` — one of the four standard classes.
- `export_safe` — boolean.

Optional fields: `linked_outage_notice_ref`,
`linked_continuity_packet_ref`,
`linked_offline_import_source_class`, `notes`.

### 5.2 `artifact_class` (closed)

Closed set of artifact families. Adding a value is additive-minor.

- `updates` — application updates and patch bundles.
- `extensions` — extension packages and their signed manifests.
- `docs_pack` — docs and help packs (offline help / docs index).
- `policy_bundle` — signed policy bundles and their epochs.
- `models` — signed model bundles for local inference.

### 5.3 `signer_state_class` (closed)

- `signed_trust_root_pinned` — signature verifies against a
  pinned vendor or upstream trust root.
- `signed_offline_trust_root` — signature verifies against the
  air-gapped customer-managed offline trust root.
- `signed_org_ca_pinned` — signature verifies against the
  enterprise / org CA bundle.
- `unsigned` — artifact carries no signature. Schema-denied for
  the `air_gapped` profile.
- `signer_unknown` — signer state cannot be resolved at this
  render. The schema requires the row to surface a verify action
  whose `revalidation_on_open` is `blocked_until_fresh` or
  `requery_before_batch`.
- `not_applicable` — the artifact class does not apply to the
  rendered profile (for example, models on a profile that does
  not claim local inference).

### 5.4 `digest_state_class` (closed)

- `digest_verified` — digest checksum matches the signed manifest.
- `digest_pending` — digest verification is in flight.
- `digest_mismatch` — digest does not match. Schema-denies the
  artifact from being labelled as ready; surfaces MUST cite the
  mismatch through the verify action.
- `digest_unknown` — digest state cannot be resolved.
- `not_applicable` — the artifact class does not apply.

### 5.5 `offline_cache_posture_class` (closed)

- `offline_bundle_present` — a signed offline bundle is present
  and resolves the artifact.
- `mirror_snapshot_present` — a mirror snapshot is present and
  resolves the artifact.
- `no_cache_required` — the rendered profile does not require
  the artifact to be cached locally (e.g. `managed_cloud` with
  vendor-served extensions).
- `cache_missing_blocked` — the rendered profile requires the
  artifact to be cached locally and the cache is missing. The
  parent card MUST surface the matching residual-dependency row
  with `unreachable_impact_class = blocked_pending_mirror_refresh`.
- `not_applicable` — the artifact class does not apply.

### 5.6 `verify_action` and `open_manifest_action`

Both actions are inspect-only and follow the repair-action-card
shape from
[`/docs/ux/transport_and_environment_status_contract.md`](./transport_and_environment_status_contract.md).
Each carries:

- `action_id` — opaque id.
- `label` — short reviewable label.
- `scope_class` — `scope_local_only`. Verify and open-manifest
  actions cannot reach beyond the local device.
- `authority_class` — `user_local_authority`.
- `consent_class` — `no_consent_required_safe_default`.
- `side_effects` — exactly `["no_side_effect_inspect_only"]`.
- `preserves_evidence_context` — `true`.
- `modal_prohibited` — `true` for `open_manifest_action`. The
  `verify_action` MAY raise a step-up modal only when
  `signer_state_class = signer_unknown` or
  `digest_state_class = digest_pending`; the schema enforces that
  every other path is non-modal.
- `evidence_links[]` — opaque refs into the records the action
  cites.

A `digest_state_class = digest_mismatch` row's `verify_action`
MUST set `revalidation_on_open = blocked_until_fresh` so the
action does not silently pass a known mismatch.

## 6. Cross-record invariants

The schema enforces these invariants mechanically. A surface that
violates any of them is non-conforming.

1. **Locality posture matches the deployment profile.** For
   `self_hosted`, `enterprise_online`, and `managed_cloud` cards,
   `tenant_org_scope_class`, `region_scope_class`, and
   `key_mode_class` cannot be `not_applicable`. For
   `individual_local`, `tenant_org_scope_class` is
   `single_user_local` and `region_scope_class` is
   `not_applicable`. For `air_gapped`,
   `mirror_offline_state_class` is `offline_air_gapped`.
2. **Plane separation never collapses.** If
   `control_plane_state_summary.worst_state_class` is anything
   other than `healthy` and `not_applicable`, the
   `data_plane_state_summary` MUST keep
   `available_local_safe_capability_classes[]` non-empty whenever
   the underlying continuity packet retains any local-safe
   capability. A control-plane outage cannot zero out the local
   data-plane summary; surfaces MUST render the impaired
   control-plane service classes separately from the data-plane
   summary, and their `summary_label` strings MUST NOT collapse
   into a generic "service degraded" or "everything is broken"
   sentence.
3. **Workspace and runtime impairment is named separately.** A
   data-plane state of `blocked_pending_reconnect`,
   `blocked_pending_boundary_recheck`, or
   `blocked_by_policy` cited in `data_plane_state_summary` MUST
   list at least one capability class in
   `impaired_capability_classes[]`; the card cannot imply that the
   whole runtime is down when only a managed-only capability is
   blocked.
4. **No prohibited implied claim is assertable.** A card whose
   `prohibited_implied_claim_classes[]` is non-empty MUST refuse
   to render any consumer surface that asserts copy on the listed
   classes. Surfaces resolve their copy against the card's allowed
   claim set, never against an unguarded posture.
5. **Vendor-bound dependencies cannot be customer-operated.** A
   `residual_dependency_row_record` whose
   `dependency_class` is `ai_provider`, `browser_handoff`,
   `companion_notification_channel`, or
   `hosted_control_plane_reachability`, whose `posture_class` is
   `required`, and whose parent card's `deployment_profile` is
   `enterprise_online` or `managed_cloud` MUST set
   `vendor_or_public_dependence = true`.
6. **Air-gapped profiles forbid online states and unsigned
   artifacts.** A card with `deployment_profile = air_gapped`
   MUST set `mirror_offline_state_class = offline_air_gapped`,
   MUST emit at least one `mirror_offline_artifact_row_record`
   for the `policy_bundle` and `docs_pack` artifact classes
   (which the air-gapped profile claims as mirrored), and MUST
   reject any `mirror_offline_artifact_row_record` whose
   `signer_state_class = unsigned`. A card whose
   `companion_surface` consumer is included on this profile is
   schema-denied because the air-gapped profile's
   `companion_surface_posture_class` is
   `companion_handoff_explicitly_disallowed`.
7. **Mirror-only artifacts require a freshness label.** A
   `mirror_offline_artifact_row_record` whose
   `mirror_freshness_class` is `mirror_within_extended_window`,
   `mirror_past_extended_window`, or `mirror_freshness_unknown`
   MUST include a non-empty `last_refresh_at` (where applicable)
   and the parent card MUST include
   `implied_offline_parity_when_mirror_only` in
   `prohibited_implied_claim_classes[]`.
8. **Inspect-only actions stay inspect-only.** The
   `open_details_action`, `verify_action`, and
   `open_manifest_action` MUST declare exactly
   `["no_side_effect_inspect_only"]`. Stacking another effect on
   any of those actions is non-conforming.
9. **Export discipline preserves redaction.** A card or row
   exported under `export_safe = true` MUST keep its
   `redaction_class` ≤ the underlying continuity packet's
   redaction class. Exports that widen redaction are non-
   conforming.
10. **One card, one continuity packet, one transport posture.**
    A card that cites a `linked_continuity_packet_ref` MUST keep
    its `mirror_offline_state_class`, control-plane summary, and
    data-plane summary value-equal to the cited packet. A card
    that cites a `linked_transport_posture_ref` MUST keep its
    `mirror_offline_state_class` value-equal to the transport
    posture's `offline_or_deny_all_state`.

## 7. Independence-claim guardrails

The acceptance criterion *self-hosted, sovereign, mirrored,
offline-capable, and managed claims can be rendered without implying
stronger independence than the current deployment actually has*
resolves through the closed
`prohibited_implied_claim_class` vocabulary in §3.7 plus the per-
profile pairing rules below.

- A `self_hosted` card whose `key_mode_class = vendor_managed` is
  schema-denied: self-hosted does not imply vendor-managed keys.
- A card claiming `air_gapped` MUST forbid every
  `vendor_or_public_dependence = true` row from carrying
  `posture_class = required`. Required vendor dependencies on
  air-gapped are non-conforming.
- A card claiming `mirrored` (any profile with
  `mirror_offline_state_class = online_mirror_only`) MUST list
  `implied_offline_parity_when_mirror_only` in
  `prohibited_implied_claim_classes[]`.
- A card claiming `offline-capable` (any profile whose
  `mirror_offline_state_class` is
  `offline_grace_preserved` or `offline_air_gapped`) MUST list
  `implied_always_fresh_when_bounded_or_unbounded_stale` in
  `prohibited_implied_claim_classes[]` whenever the
  `freshness_posture_ref` resolves to a non-`fresh` staleness
  class.
- A card claiming `managed_cloud` MUST list
  `implied_self_hosted_when_managed_cloud` and
  `implied_managed_independence_when_local_dependent` in
  `prohibited_implied_claim_classes[]`.

These rules are schema-enforced. A fixture that asserts a
prohibited implied claim against the chosen profile is non-
conforming.

## 8. Plane-separation rules

The acceptance criterion *summary cards distinguish control-plane
impairment from workspace/runtime impairment* resolves through §3.4,
§3.5, and §6.2-§6.3. In particular:

- A control-plane impairment (any
  `control_plane_service_state_class` value other than `healthy`
  or `not_applicable`) MUST be named through the impaired-service-
  class list and its `summary_label`, and MUST NOT cause the
  data-plane summary to flatten into a generic outage.
- A workspace or runtime impairment (any
  `data_plane_capability_state_class` value other than
  `available_local_safe`) MUST be named through the impaired-
  capability-class list and the worst-state class. The
  `available_local_safe_capability_classes[]` MUST list every
  capability that remains local-safe; a control-plane outage
  cannot zero out this list.
- The `summary_label` strings on the two summary objects are
  rendered separately. UI surfaces MAY NOT compose them into one
  sentence that reads as a single failure mode.

## 9. Export and reuse posture

The acceptance criterion *residual dependencies and mirror/offline
artifact state are exportable and reusable in About, diagnostics,
and support packets* resolves through:

- The `consumer_surfaces[]` field on the card naming exactly the
  surfaces consuming the record. The card record is exported
  byte-for-byte under the declared `redaction_class`; About,
  diagnostics, support-packet, admin-audit, and release-evidence
  consumers all read the same record family.
- The `residual_dependency_row_record` rows and
  `mirror_offline_artifact_row_record` rows are addressable from
  the card by opaque ref. Each row is independently exportable
  under its own `redaction_class`, so a support packet can
  include the rows without the card or the card without rows when
  that is the right scope.
- A card exported under `export_safe = true` preserves field
  names exactly. Renaming, paraphrasing, or splitting fields on
  export is non-conforming. The schema registry rows for the
  About, diagnostics, support, admin-audit, and release-evidence
  packet families consume this record family without minting a
  parallel deployment-summary shape.

## 10. Adding or changing vocabulary

Adding a value to any vocabulary in this contract is **additive-
minor** and requires:

1. Updating the schema enum in
   `schemas/deployment/deployment_summary_card.schema.json`.
2. Updating this document.
3. Adding or updating a fixture under
   `fixtures/deployment/deployment_summary_cases/` exercising the
   new value.
4. Bumping the corresponding `*_schema_version` integer.

Repurposing an existing value is **breaking** and requires:

1. A new decision row in
   `artifacts/governance/decision_index.yaml` co-signed by
   `security_trust_review` and `product_scope_review`.
2. Deprecation of the old value, addition of the new value through
   an additive-minor landing, and a translation pass on About,
   diagnostics, support, admin-audit, and release-evidence
   consumers across the deprecation window.

Vocabularies that re-export from upstream seeds (deployment
profile, locality posture, residual-dependency ledger, control-
plane / data-plane state, mirror freshness, transport posture)
follow the upstream change rules; this contract follows in the
same change.

## 11. Out of scope at this revision

- Final About / diagnostics / support / admin-audit / release-
  evidence layout, animation, and accessibility wiring. The
  contract pins the record family; the rendering surfaces own
  their own component contracts.
- Pixel-perfect summary card layout and the cross-platform widget
  toolkit.
- Localization-ready string tables; the contract carries
  reviewable English sentences and the localization layer is
  consumed separately.
- Live mirror server, signed-policy distribution pipeline,
  offline-bundle build pipeline, model registry, extension
  marketplace, docs-pack publisher, and update channel
  implementations.
