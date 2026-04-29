# Managed-service operating-mode, service-family capacity, region or residency strip, quota or throttle banner, tenant or key detail card, and service-continuity note contract

This document freezes the visible operating-mode and service-capacity
model Aureline surfaces use so optional managed features stay legible
under quota pressure, region mismatch, tenant isolation, or key-state
changes **without implying whole-product failure**.

The goal is that any user or admin who reaches an operating-mode card,
a service-family capacity row, a region or residency strip, a quota or
throttle banner, a tenant or key detail card, or a service-continuity
note can answer four questions from the surface itself:

- **What operating mode am I in?** Local only, shared community cloud,
  enterprise SaaS, self-hosted, or sovereign — with the visible
  processing location, storage location, tenant scope, region scope,
  retention class, and key mode that mode pins.
- **What is each service family doing right now?** Per family — sync,
  registry or mirror metadata, collaboration relay, remote workspace
  control plane, AI gateway, telemetry or support ingest — what is the
  capacity state, who owns the quota, when was capacity last measured,
  which action families narrow if capacity exhausts, and what is the
  fail posture for those families?
- **What is the region, residency, tenant, and key truth?** Without
  reading procurement documents, deployment runbooks, or admin console
  pages.
- **What still works when one family narrows?** The retained local-safe
  capability set the local-first core continues to support, separated
  from the managed-only action families that pause until capacity,
  region, tenant, or key state recovers.

This contract is **vocabulary-only**: it does not implement quotas,
SLAs, or cloud routing systems.

## Companion artifacts

- [`/schemas/service/operating_mode.schema.json`](../../schemas/service/operating_mode.schema.json)
  — boundary schema for the `operating_mode_card_record`. Carries the
  mode card, the per-service-family capacity row vocabulary, the
  service-continuity note shape, the closed quota or throttle banner
  vocabulary, and the typed fail posture for each capacity state.
- [`/schemas/service/region_key_state.schema.json`](../../schemas/service/region_key_state.schema.json)
  — boundary schema for the `region_key_state_record`. Carries the
  region or residency strip, the tenant or key detail card, the
  closed key-state vocabulary, and the boundary-recheck posture that
  applies when region, tenant, or key truth is unknown or changing.
- [`/fixtures/service/capacity_and_residency_cases/`](../../fixtures/service/capacity_and_residency_cases/)
  — worked fixtures exercising the contract.

## Inherited contracts

This contract stands on top of earlier seeds and MUST NOT recast any of
them:

- [`/docs/deployment/locality_and_continuity_seed.md`](../deployment/locality_and_continuity_seed.md)
  and
  [`/artifacts/deployment/locality_matrix.yaml`](../../artifacts/deployment/locality_matrix.yaml)
  — re-exports the closed `deployment_profile`, `processing_location`,
  `storage_location`, `tenant_org_scope`, `region_scope`,
  `retention_class`, `key_mode`, control-plane service class,
  control-plane state, data-plane capability, data-plane state, and
  restore-class vocabularies. This contract reuses those vocabularies
  rather than minting parallels.
- [`/docs/service/managed_service_seed.md`](./managed_service_seed.md)
  and
  [`/artifacts/service/slo_rows.yaml`](../../artifacts/service/slo_rows.yaml)
  — freezes the closed `service_id` vocabulary, the
  `service_opt_in_posture`, `availability_slo`, `freshness_slo`, and
  `degradation_mode` vocabularies. The capacity row inherits its
  service-id binding and degradation copy from the matching SLO row.
- [`/docs/service/api_inventory_seed.md`](./api_inventory_seed.md)
  and
  [`/schemas/service/api_capability_row.schema.json`](../../schemas/service/api_capability_row.schema.json)
  — freezes the local-fallback posture every API surface row pins. A
  capacity row's `affected_action_family` set MUST be admissible
  under the corresponding API rows' fallback posture.
- [`/docs/auth/managed_auth_and_session_continuity_contract.md`](../auth/managed_auth_and_session_continuity_contract.md)
  — managed identity remains additive capability. The tenant or key
  detail card composes against the managed session state record; it
  does not replace it.
- [`/docs/policy/admin_policy_and_bundle_cache_contract.md`](../policy/admin_policy_and_bundle_cache_contract.md)
  — policy bundles narrow what an operating mode admits. Capacity
  rows that block an action family by policy point at a policy
  decision rather than reusing capacity vocabulary.

Normative sources:

- `.t2/docs/Aureline_PRD.md` §5.24, §5.53, §5.57, and Appendix AN.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §9.6, §9.7,
  and §9.8.
- `.t2/docs/Aureline_Technical_Design_Document.md` §11.4.2 and §11.4.3.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §18.31, §18.42, and
  Appendix BL.

If this document disagrees with those sources, those sources win and
this document plus its companion schemas update in the same change. If
this document and the companion schemas disagree, this document wins
and the schemas update in the same change.

## Why this exists

Without a frozen operating-mode and capacity vocabulary, a managed
feature reaches for one of four failure modes when it narrows:

1. **Whole-product failure copy.** A blocked AI gateway request
   surfaces as "Aureline is unavailable" even though local edit, save,
   search, Git, and BYOK AI remain reachable.
2. **Region or residency surprise.** A user who never reviewed the
   active region discovers it only when an export is rejected — by
   then, an unbounded action has already been attempted.
3. **Quota copy that hides who owns the quota.** "Quota exceeded"
   without a quota owner, a measurement window, or an affected action
   family forces the user to open a procurement ticket to recover
   intent.
4. **Key-state mismatch that fails open or fails too closed.** A
   changed customer-managed key blocks every managed action when in
   reality only one family (for example, AI evidence retention) is
   bounded by that key, and the rest can continue under the existing
   posture; or the surface fails open and a managed write proceeds
   under a stale key.

This contract closes those gaps by freezing two record families:

- **`operating_mode_card_record`** — the user- and admin-visible card
  that pins one operating mode, the per-service-family capacity rows,
  the quota or throttle banner state, and the service-continuity note.
- **`region_key_state_record`** — the user- and admin-visible record
  that pins the region or residency strip, the tenant or key detail
  card, and the boundary-recheck posture.

Both records compose with the locality continuity packet, the SLO
rows, the API surface rows, and the deletion-job record; none replace
them.

## Scope

Frozen at this revision:

- one `operating_mode_card_record` carrying card id, the closed
  `operating_mode_class` (`local_only`, `shared_community_cloud`,
  `enterprise_saas`, `self_hosted`, `sovereign`), the locality
  posture re-exported from the locality seed, the per-service-family
  capacity row list (sync, registry or mirror metadata, collaboration
  relay, remote workspace control plane, AI gateway, telemetry or
  support ingest), an optional quota or throttle banner, an optional
  tenant or key detail summary, the service-continuity note, the
  retained local-safe capability list, the blocked managed-only
  capability list, the freshness posture for the card itself, and the
  display-copy invariants that forbid whole-product-failure language;
- one `region_key_state_record` carrying record id, region or
  residency strip vocabulary, tenant or key detail card vocabulary,
  the closed `key_state_class` (`bound_and_current`,
  `bound_pending_rotation`, `rotation_in_progress`,
  `rotation_completed_recheck_required`, `revoked_recheck_required`,
  `mismatch_recheck_required`, `customer_managed_unreachable`,
  `not_applicable`), the closed `affected_action_family_class` set
  (sync, marketplace publish, marketplace install, collaboration
  publish, remote attach, remote execute, AI inference, AI evidence
  retention, telemetry export, support export, offboarding export,
  policy distribution, identity refresh), and the
  `boundary_recheck_required` flag with linked recheck refs;
- the closed `capacity_state_class` vocabulary
  (`healthy`, `at_warning_threshold`, `at_throttle_threshold`,
  `quota_exhausted`, `region_mismatch_blocked`,
  `tenant_isolation_blocked`, `key_state_blocked`,
  `boundary_recheck_required`, `unknown_pending_recheck`);
- the closed `quota_owner_class` vocabulary
  (`account_free_local`, `community_cloud_shared_pool`,
  `enterprise_tenant`, `self_hosted_customer_operated`,
  `sovereign_customer_operated`, `byok_customer_operated`,
  `not_applicable`);
- the closed `fail_posture_class` vocabulary
  (`fail_closed_managed_only`, `fail_open_local_safe`,
  `fail_open_local_safe_with_label`, `boundary_recheck_required`,
  `not_applicable`);
- the rule that capacity or key-state failures fail closed only for
  the managed action that cannot be bounded safely, and fail open for
  documented local-safe workflows where the product contract allows
  it;
- the display-copy invariants that forbid whole-product-failure
  language, generic "service unavailable" copy, and silent fail-open
  of a managed action under unknown region, tenant, or key state.

Out of scope at this revision (named explicitly so reviewers know what
is *not* being decided here):

- implementing quotas, throttling, accounting ledgers, billing, SLAs,
  or cloud routing systems. The records freeze the *visible
  vocabulary*; orchestration lives elsewhere;
- live wiring against any specific cloud provider, region, residency
  certification, or key management service;
- final user-facing copy, status-strip integration, notification
  routing, animation, or iconography. Surfaces compose against the
  records frozen here;
- replacement of the unified status strip in
  `docs/ux/control_data_plane_status_contract.md` or the locality
  continuity packet in
  `schemas/deployment/local_core_continuity_packet.schema.json`. Those
  remain the cross-surface continuity render records; this contract
  freezes the *capacity, region, tenant, and key truth* that the
  operating-mode card surfaces.

## The four questions every record answers

Any Aureline surface that renders an operating-mode card, a service-
family capacity row, a region or residency strip, a quota or throttle
banner, a tenant or key detail card, or a service-continuity note MUST
answer these questions mechanically against the records here:

1. **Which operating mode is in force?** `operating_mode_class`,
   `locality_posture`, and `service_continuity_note` answer this. The
   mode pins what processing location, storage location, tenant scope,
   region scope, retention class, and key mode the user sees.
2. **What is each service family doing?** `service_family_capacity`
   answers this. Each family carries `service_family_class`,
   `linked_service_ids`, `capacity_state`, `quota_owner`,
   `measurement_time`, `affected_action_families`, `fail_posture`,
   and `degradation_mode`.
3. **What is the region, tenant, and key truth?**
   `region_residency_strip`, `tenant_detail`, and `key_state` on the
   `region_key_state_record` answer this. Managed and enterprise
   modes MUST pin a non-`not_applicable` value for region, tenant,
   and key.
4. **What still works locally?** `retained_local_safe_capabilities`
   and `blocked_managed_only_capabilities` answer this. The card
   cannot let "what still works" be inferred from the absence of a
   blocked action.

Generic copy such as "Aureline is unavailable", "service is offline",
"quota exceeded", "try again", or "please contact support" is not
admissible on these paths when a typed state is available from the
records.

## Operating modes and required disclosure

An `operating_mode_card_record` carries exactly one
`operating_mode_class`. The mode pins the locality posture the card
surfaces; managed and enterprise modes MUST NOT collapse region,
tenant, or key into `not_applicable`.

| Mode | Meaning | Required locality posture |
| --- | --- | --- |
| `local_only` | The desktop core operates with no managed-service prerequisites. Optional services may be reachable but none of them is required. | `processing_location: on_device_only`, `storage_location: device_local_disk`, `tenant_org_scope: single_user_local`, `region_scope: not_applicable`, `key_mode: os_store`. |
| `shared_community_cloud` | A shared community-operated cloud is in scope; the user opted into a community-operated control plane. | `processing_location: vendor_control_plane`, `storage_location: vendor_control_plane_storage`, `tenant_org_scope: shared_multi_tenant`, `region_scope` non-`not_applicable`, `key_mode` non-`not_applicable`. |
| `enterprise_saas` | A vendor-operated control plane runs the customer's tenant under contracted enterprise SaaS terms. | `processing_location: vendor_control_plane` or `customer_or_vendor_control_plane`, `tenant_org_scope: customer_tenant`, `region_scope` non-`not_applicable`, `key_mode` non-`not_applicable`. |
| `self_hosted` | The customer operates the control plane and storage in their own environment. There is no vendor fallback. | `processing_location: customer_control_plane`, `storage_location: customer_control_plane_storage`, `tenant_org_scope: customer_tenant`, `region_scope: customer_region_pinned`, `key_mode: customer_managed`. |
| `sovereign` | The customer operates a sovereign control plane in a regulated jurisdiction with offline trust roots and regulated egress. | `processing_location: customer_control_plane` or `on_device_only`, `storage_location: customer_control_plane_storage` or `mirror_or_offline_bundle`, `tenant_org_scope: customer_tenant`, `region_scope: customer_region_pinned`, `key_mode: offline_trust_root` or `customer_managed`. |

The mode card MUST NOT carry a "default" mode. A surface that cannot
resolve the active mode renders `unknown_pending_recheck` on every
service-family capacity row and pins `boundary_recheck_required`
until a recheck completes; the card itself cannot be issued without
an `operating_mode_class`.

## Service families and capacity rows

A `service_family_capacity` row carries exactly one
`service_family_class` from the closed vocabulary:

| Family | Meaning | Linked service ids (typical) |
| --- | --- | --- |
| `sync_family` | Settings, project, and workspace state synchronization. | `managed_settings_sync` |
| `registry_or_mirror_metadata_family` | Marketplace, catalog, and docs-pack registry / mirror metadata. | `managed_marketplace`, `managed_catalog`, `managed_docs_pack` |
| `collaboration_relay_family` | Hosted review, collaboration archive, and presence relay. | `managed_collaboration_review`, `managed_relay` |
| `remote_workspace_control_plane_family` | Remote attach, remote execute, and remote workspace control plane. | `managed_relay` |
| `ai_gateway_family` | Managed AI broker / inference gateway. | `managed_ai_broker` |
| `telemetry_or_support_ingest_family` | Telemetry sink, support export, entitlement-usage, and offboarding-export ingest. | `managed_telemetry_sink`, `managed_support_export`, `managed_entitlement_usage`, `managed_offboarding_export` |

Each capacity row carries:

- `service_family_class` — one of the above.
- `linked_service_ids` — non-empty subset of the SLO row vocabulary
  the family resolves through.
- `capacity_state` — one value from the closed
  `capacity_state_class` vocabulary.
- `quota_owner` — one value from the closed `quota_owner_class`
  vocabulary, naming who owns the quota the family is measured
  against. `not_applicable` is reserved for families with no
  metered quota at this revision (typically `local_only` mode).
- `measurement_time` — RFC 3339 UTC timestamp at which the capacity
  state was last measured. A row with `unknown_pending_recheck` MAY
  carry a null measurement time but MUST resolve before the card is
  treated as live.
- `affected_action_families` — closed subset of the action-family
  vocabulary (sync, marketplace publish, marketplace install,
  collaboration publish, remote attach, remote execute, AI
  inference, AI evidence retention, telemetry export, support
  export, offboarding export, policy distribution, identity
  refresh) naming the action families that pause when this row is
  not `healthy`.
- `fail_posture` — one value from the closed `fail_posture_class`
  vocabulary. Capacity or key-state failures fail closed only for
  the managed action family that cannot be bounded safely; rows
  whose `capacity_state` is non-healthy MUST pin a fail posture
  consistent with the affected action families' API-surface
  fallback posture.
- `degradation_mode` — re-export of the SLO row's degradation mode
  vocabulary so capacity copy and SLO copy resolve to the same
  state. Required when the row is non-`healthy` and the linked
  service has a degradation mode.
- `recovery_cue` — short product-term sentence naming the next
  user-visible step.
- `notes` — optional short reviewable note.

A capacity row MUST NOT mark a row `healthy` while its measurement
time is older than the freshness floor on the linked SLO row; surfaces
that cannot honor the freshness floor MUST route through
`unknown_pending_recheck` instead of backdating health.

## Quota or throttle banner

The optional `quota_throttle_banner` surfaces when at least one
service-family capacity row resolves to `at_warning_threshold`,
`at_throttle_threshold`, or `quota_exhausted`. The banner carries:

- `banner_state` — one of `at_warning_threshold`,
  `at_throttle_threshold`, `quota_exhausted`,
  `boundary_recheck_required`, `unknown_pending_recheck`.
- `quota_owner` — the same closed vocabulary the capacity row uses.
- `measurement_time` — RFC 3339 UTC timestamp.
- `measurement_window_summary` — a short reviewable sentence that
  names the metering window without leaking raw billing values
  ("daily inference budget", "monthly support-export quota").
- `affected_action_families` — non-empty subset of the action-family
  vocabulary.
- `fail_posture` — closed vocabulary value.
- `recovery_cue` — short product-term sentence.

`quota_exhausted` and `at_throttle_threshold` are not default-quiet
states. The banner MUST be rendered with a non-null `recovery_cue`
and the affected capacity rows' `fail_posture` MUST resolve to
`fail_closed_managed_only` for action families that cannot be
bounded safely under the exhausted quota.

## Region or residency strip

The `region_residency_strip` block on the `region_key_state_record`
pins the user-visible region and residency truth:

- `region_scope` — re-export of the locality seed's `region_scope`
  vocabulary. Managed and enterprise modes MUST set a
  non-`not_applicable` value.
- `region_ref` — opaque region ref. Raw cloud-region identifiers are
  not admissible.
- `residency_scope_class` — one of `customer_region_pinned`,
  `regulated_jurisdiction`, `cross_region_audited_egress`,
  `boundary_recheck_required`, or `not_applicable`.
- `residency_summary` — short reviewable sentence describing what
  the residency posture admits and forbids.
- `boundary_recheck_required` — boolean. `true` whenever
  `region_scope` is `boundary_recheck_required` or the residency
  scope is `boundary_recheck_required`.
- `linked_recheck_refs` — opaque refs to recheck packets, route
  reviews, or boundary manifests that govern the recheck.

A managed or enterprise card whose region scope is
`boundary_recheck_required` MUST set every affected service family's
`fail_posture` to `boundary_recheck_required` and MUST NOT resume
managed routes until the recheck completes.

## Tenant or key detail card

The `tenant_detail` and `key_state` blocks on the
`region_key_state_record` pin the visible tenant and key truth:

- `tenant_org_scope` — re-export of the locality vocabulary.
- `tenant_ref` — opaque tenant ref. Raw tenant names are not
  admissible.
- `tenant_summary` — short reviewable sentence.
- `key_mode` — re-export of the locality `key_mode` vocabulary.
- `key_state_class` — one of `bound_and_current`,
  `bound_pending_rotation`, `rotation_in_progress`,
  `rotation_completed_recheck_required`, `revoked_recheck_required`,
  `mismatch_recheck_required`, `customer_managed_unreachable`,
  `not_applicable`.
- `key_ref` — opaque key ref. Raw key bytes, raw fingerprints, and
  raw certificate bodies are not admissible; opaque fingerprints in
  the locality continuity packet's `opaque_fingerprint` form are
  acceptable.
- `key_state_summary` — short reviewable sentence.

A `key_state_class` other than `bound_and_current` or
`not_applicable` MUST list at least one `affected_action_family` on
the record. The action family set MUST be the *minimum* set of
managed action families that cannot be bounded safely under the new
key state. Rows that conflate "any key change" with "all managed
work blocked" are a regression; reviewers MUST reject any record
that lists every action family without a per-family rationale.

## Affected action family vocabulary

The closed `affected_action_family_class` vocabulary is shared by
the capacity row, the quota throttle banner, and the region or key
state record:

- `sync_action_family`
- `marketplace_publish_action_family`
- `marketplace_install_action_family`
- `collaboration_publish_action_family`
- `remote_attach_action_family`
- `remote_execute_action_family`
- `ai_inference_action_family`
- `ai_evidence_retention_action_family`
- `telemetry_export_action_family`
- `support_export_action_family`
- `offboarding_export_action_family`
- `policy_distribution_action_family`
- `identity_refresh_action_family`

Each row that names an action family pins exactly one
`fail_posture`. A row that pauses `ai_inference_action_family`
because of `quota_exhausted` does not implicitly pause
`ai_evidence_retention_action_family`; rows that affect both MUST
list both explicitly.

## Service-continuity note

Every operating-mode card carries a `service_continuity_note` block:

- `local_core_status_class` — one of `local_core_unaffected`,
  `meaningful_safe_subset_available`, `local_only_available`,
  `no_safe_local_subset`, `unknown_requires_review`. The default for
  managed and enterprise modes whose linked services are non-healthy
  is `meaningful_safe_subset_available`.
- `retained_local_safe_capabilities` — non-empty list of short
  reviewable sentences naming what the local-first core continues to
  support.
- `blocked_managed_only_capabilities` — list of short reviewable
  sentences naming the managed-only paths paused by the active
  capacity / region / tenant / key state.
- `linked_continuity_packet_ref` — optional pointer into a
  `local_core_continuity_packet_record` carrying the deeper
  continuity truth the card surfaces.

`no_safe_local_subset` is reserved for postures where genuinely no
local-safe subset remains (for example, a corrupted local profile
that itself prevents desktop-core operation). It is not admissible
when the desktop core can still edit, save, search, run local Git,
or export.

## Fail posture rules

Capacity, region, tenant, and key-state changes resolve through one
of five `fail_posture_class` values:

- `fail_closed_managed_only` — the managed action family is paused
  because it cannot be bounded safely under the new state. Local
  edit, save, undo, search, Git, BYOK AI, and local export remain
  available where the API-surface fallback posture admits them.
- `fail_open_local_safe` — the action family resolves locally
  without the managed surface; the local-first contract for the
  family explicitly admits local-only operation.
- `fail_open_local_safe_with_label` — the action family resolves
  locally with a stale-label or boundary-aware indicator that the
  managed copy is paused.
- `boundary_recheck_required` — privileged replay, managed routes,
  and managed writes pause until a recheck packet completes. The
  surface MUST NOT silently fail open under this posture.
- `not_applicable` — the family has no managed form in the active
  operating mode (typical for `local_only`).

A row's fail posture MUST be consistent with the API surface row's
`local_fallback_posture`. Reviewers MUST reject a capacity row that
selects `fail_open_local_safe` for an action family whose API row
declares `managed_only_no_local_fallback_narrows_managed_claim_only`.

## Display-copy invariants

Every record's `display_copy` block MUST keep four invariants false:

- `whole_product_failure_implied` — a capacity, region, tenant, or
  key state cannot imply that the whole product is broken when the
  local-first core remains available.
- `generic_unavailable_copy_used` — capacity copy cannot reuse
  generic "service unavailable" copy when a typed
  `capacity_state_class` is available.
- `silent_fail_open_under_unknown_state` — a managed write or
  managed route MUST NOT fail open while region, tenant, or key
  state is `unknown_pending_recheck`,
  `boundary_recheck_required`, `revoked_recheck_required`,
  `mismatch_recheck_required`, or `customer_managed_unreachable`.
- `quota_owner_omitted` — a quota or throttle banner cannot render
  without naming the `quota_owner`.

The schema enforces these invariants as `const: false` so a fixture
or record that flips them is invalid by construction.

## Self-hosted with no vendor fallback

The `self_hosted` mode pins a contract that the existing
`account_free_local` and managed-convenience modes do not: the
customer operates the control plane and there is **no** vendor
fallback for any service family. Capacity rows in this mode resolve
through customer-operated quotas and customer-operated key
material; rows that reach `quota_exhausted`, `key_state_blocked`,
or `customer_managed_unreachable` MUST set `fail_posture` to
`fail_closed_managed_only` or `boundary_recheck_required`. A
record that lists a vendor recovery cue in `self_hosted` mode is a
regression.

The `sovereign` mode extends `self_hosted` with regulated egress
and offline trust roots; rows MUST resolve through the locality
seed's air-gap continuity fields when an offline bundle is in
play, and a recovery cue that implies vendor egress is a
regression.

## Local-core non-dependence

Every record in this contract preserves the local-core
non-dependence clause from the managed-service seed and the
locality seed. A surface that renders an operating-mode card MUST
NOT block the desktop core's first-run or startup paths waiting
for a card to materialize; surfaces that lack a card resolve to
the `local_only` posture by default until a managed control plane
is reached.

## Worked fixtures

See [`/fixtures/service/capacity_and_residency_cases/`](../../fixtures/service/capacity_and_residency_cases/)
for fixtures exercising the contract:

- `local_only_mode.yaml` — a `local_only` operating-mode card with
  no managed services in scope and a `not_applicable` quota owner
  on every family.
- `enterprise_saas_with_region_strip.yaml` — an `enterprise_saas`
  card with the region or residency strip pinning a customer-region
  and customer-tenant scope and a healthy capacity posture.
- `self_hosted_no_vendor_fallback.yaml` — a `self_hosted` card with
  customer-operated capacity for every family and an explicit "no
  vendor fallback" continuity note.
- `ai_quota_exhaustion.yaml` — an `enterprise_saas` card whose AI
  gateway family has reached `quota_exhausted` for the
  `ai_inference_action_family`, with a `fail_closed_managed_only`
  fail posture, the quota throttle banner naming the enterprise
  tenant as `quota_owner`, and BYOK plus local-AI continuity in the
  service-continuity note.
- `key_state_mismatch_one_family.yaml` — an `enterprise_saas`
  card paired with a `region_key_state_record` whose
  `key_state_class` is `mismatch_recheck_required` for the
  `ai_evidence_retention_action_family` only, while every other
  managed action family remains healthy. The fixture demonstrates
  that key-state changes do not collapse into whole-product
  failure copy.

## Evolution rules

- Adding a new `operating_mode_class`, `service_family_class`,
  `capacity_state_class`, `quota_owner_class`,
  `affected_action_family_class`, `fail_posture_class`, or
  `key_state_class` value is additive-minor and bumps the
  `schema_version` on the affected schema. Repurposing an existing
  value is breaking and requires a new decision row in
  `artifacts/governance/decision_index.yaml`.
- Promoting a service family from "no metered quota" to a metered
  capacity row requires a paired update to the linked SLO row's
  `degradation_modes` and the linked API surface row's
  `local_fallback_posture` so the fail posture remains consistent
  across the three surfaces.
- New region or key-state vocabulary lands a paired decision row
  before the schema and fixture changes ship; a region or
  residency value that is admissible only in a specific operating
  mode MUST be enforced by a schema-level `if/then` gate so the
  vocabulary cannot escape the mode it was scoped for.
- This document, the two JSON Schemas, and the worked fixtures stay
  in sync by review. Tooling MAY reject PRs that introduce a
  vocabulary value, mode, family, or action family in only one of
  the surfaces.
