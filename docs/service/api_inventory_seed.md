# Optional-service API inventory, OpenAPI seed, and local-vs-service dependency rules

This document is the narrative seed for Aureline's optional managed-
service HTTP API surfaces. It freezes four things so that future
service surfaces stay contract-first and never silently become
prerequisites for local desktop value:

1. **One inventory of optional service-API families** (registry,
   policy, sync, fleet, diagnostics, managed-control-plane surfaces)
   with owner, auth mode, data class, and local-fallback posture.
2. **One machine-readable OpenAPI seed** showing versioning,
   deprecation lane, and offline / cache behavior expectations so
   mock, client-generation, and documentation work can attach to one
   source before any backend exists.
3. **Dependency rules** stating that local desktop operation MUST NOT
   depend on live service-schema discovery and that service APIs
   share the same compatibility inventory as every other machine
   surface.
4. **Cross-references** back into the managed-service SLO, retention,
   deletion-semantics, stable-surface inventory, schema-family,
   policy-flag-schema-stack, and standards-matrix registers so that
   nothing in this seed lives in isolation.

## Companion artifacts

- [`/artifacts/service/api_surface_rows.yaml`](../../artifacts/service/api_surface_rows.yaml)
  — per-service-HTTP capability rows binding each optional API
  surface to one `managed_service_slo_row`, one or more
  `managed_service_retention_row` rows, a frozen auth mode, a
  data-class set, a local-fallback posture, a versioning rule, a
  deprecation lane, and an offline / cache posture.
- [`/schemas/service/api_capability_row.schema.json`](../../schemas/service/api_capability_row.schema.json)
  — boundary schema for the row shape in `api_surface_rows.yaml`.
  Draft 2020-12; `additionalProperties: false`; every closed
  vocabulary (`auth_mode`, `entitlement_class`,
  `policy_override_posture`, `data_class`, `local_fallback_posture`,
  `versioning_rule_class`, `url_version_segment_policy`,
  `unknown_field_policy`, `deprecation_lane_class`,
  `sunset_posture`, `client_signal_header`,
  `offline_behavior_class`, `client_cache_policy_class`,
  `last_known_good_posture`, `schema_discovery_posture`) is pinned
  here and re-exported in the YAML row file. Adding a value is
  additive-minor; repurposing is breaking.
- [`/openapi/service_api_seed.yaml`](../../openapi/service_api_seed.yaml)
  — OpenAPI 3.1 seed (forward-compatible with 3.2 as soon as the
  standards-matrix row ratifies it) describing the paths, request
  and response shapes, security schemes, deprecation header
  expectations, and per-operation `x-aureline-offline-and-cache`
  posture for each row. Not a live service description; a seed.

## Inherited contracts

This seed stands on top of earlier contracts and MUST NOT recast any
of them:

- [`/docs/service/managed_service_seed.md`](./managed_service_seed.md)
  and
  [`/artifacts/service/slo_rows.yaml`](../../artifacts/service/slo_rows.yaml)
  — freezes the closed `service_id_vocabulary` (thirteen services),
  the closed `service_opt_in_posture`, `availability_slo`,
  `freshness_slo`, `degradation_mode`, and identity / deployment
  profile vocabularies. Every row in this seed inherits its
  availability, freshness, degradation, and recovery-cue contract
  from the matching SLO row.
- [`/artifacts/service/retention_rows.yaml`](../../artifacts/service/retention_rows.yaml)
  — every data class touched by an API surface MUST have a matching
  retention row naming managed-copy kind, retention window, export
  posture, legal-hold eligibility, customer-exit posture, and
  deletion semantics.
- [`/docs/governance/contract_packet_template.md`](../governance/contract_packet_template.md)
  and
  [`/artifacts/governance/stable_surface_inventory.yaml`](../../artifacts/governance/stable_surface_inventory.yaml)
  — every API surface row is published as a
  `surface_contract_packet` row in the stable-surface inventory so
  compat, docs, migration, deprecation, release, and support work
  share one surface id instead of minting parallel ids.
- [`/docs/governance/policy_flag_schema_stack.md`](../governance/policy_flag_schema_stack.md)
  §"HTTP service API surface — OpenAPI 3.2+"
  and the `standard.openapi_3_2` row in
  [`/artifacts/governance/standards_matrix.yaml`](../../artifacts/governance/standards_matrix.yaml)
  — ratifies OpenAPI 3.2+ as the HTTP service API description
  format. This seed publishes in the 3.1 subset that is a strict
  subset of 3.2 so later bumps are mechanical.
- [`/artifacts/governance/schema_families.yaml`](../../artifacts/governance/schema_families.yaml)
  — the `service` family row is the machine home for both the
  `deletion_job_record` and the `api_capability_row` schemas. The
  OpenAPI seed is co-located under `/openapi/` and named in the
  service family's notes.
- [`/artifacts/compat/qualification_matrix_seed.yaml`](../../artifacts/compat/qualification_matrix_seed.yaml)
  — every API surface row's `compat_binding.compatibility_window_source_ref`
  resolves to a row in the qualification matrix so service APIs
  share the same compatibility inventory as WIT worlds, event
  envelopes, JSON Schema artifacts, and record-class rows.

Normative sources:

- `.t2/docs/Aureline_PRD.md` §5.24, §5.53, §5.57, and Appendix AN.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §9.6 and
  §9.7.
- `.t2/docs/Aureline_Technical_Design_Document.md` §11.4.2 and
  §11.4.3.

If this document disagrees with those sources, those sources win and
this document plus the YAML and OpenAPI seeds update in the same
change. If the narrative and the YAML rows disagree, the narrative
wins and the YAML updates in the same change. If the YAML rows and
the OpenAPI seed disagree, the YAML rows win and the OpenAPI seed
updates in the same change.

## Scope

Frozen at this revision:

- one API surface row per optional service-HTTP capability (a family
  of endpoints hosted on one managed-service surface), binding it to
  an owner, an `auth_mode`, an `entitlement_class`, a
  `policy_override_posture`, a `data_class` set, a
  `local_fallback_posture`, a `versioning_rule_class` plus a current
  major-version tag, a `deprecation_lane_class` plus a
  `sunset_posture` and a typed `client_signal_header` set, an
  `offline_behavior_class` plus a `client_cache_policy_class`, a
  `last_known_good_posture`, a `schema_discovery_posture`, and a
  `freshness_floor_summary`;
- the row shape frozen at `schemas/service/api_capability_row.schema.json`
  (Draft 2020-12, `aureline.dev` `$id`,
  `additionalProperties: false`), with `allOf` gates that forbid
  `open_no_auth` when the row touches
  `credential_material_reference`, require a non-null
  `openapi_document_ref` when the row is at `beta` or `stable`
  maturity, and forbid `live_discovery_against_service` as a
  `schema_discovery_posture`;
- the OpenAPI seed at `openapi/service_api_seed.yaml`, published
  early enough that mock, code-generation, SDK-stub, and
  documentation work can attach to one source;
- a closed `api_family_class` vocabulary covering the spec's named
  families (`registry_api_family`,
  `policy_distribution_api_family`, `sync_api_family`,
  `fleet_api_family`, `diagnostics_api_family`,
  `managed_control_plane_api_family`) plus the families the managed
  services in scope today require (`identity_api_family`,
  `catalog_api_family`, `ai_broker_api_family`,
  `collaboration_relay_api_family`, `telemetry_ingest_api_family`,
  `support_export_api_family`, `entitlement_usage_api_family`,
  `offboarding_export_api_family`, `docs_pack_api_family`);
- an explicit local-core non-dependence clause covering first-run,
  startup, and the forbidden-coupling list.

Out of scope at this revision:

- running a live HTTP server or implementing any of these endpoints;
- shipping an auth server, token introspection service, or
  SCIM backend;
- shipping a client SDK, a mock server binary, or a code generator;
- wire-protocol choices below the HTTP level, traffic-shaping, or
  tenancy routing algorithms;
- pricing, entitlement ledger algorithms, or legal-hold process
  mechanics (those compose over this seed and the retention rows).

## The four questions this seed answers

Any future Aureline surface that declares an optional service HTTP
API MUST answer these four questions mechanically against the rows
here:

1. **Who owns it, who may call it, and with what credentials?**
   `ownership.owner_dri`, `ownership.owning_lane`, `auth.auth_mode`,
   `auth.entitlement_class`, and `auth.policy_override_posture`
   answer this. No surface ships with `open_no_auth` at
   `maturity_lane: stable` or `beta` if it touches
   `credential_material_reference`.
2. **What data classes cross the boundary, and what is the local
   fallback when the surface is unreachable?** `data_classes` and
   `local_fallback_posture` answer this. `local_core_blocking` is
   pinned `false` on every row.
3. **What is the versioning and deprecation lane?** `versioning`
   and `deprecation_lane` answer this. Every row names a
   `current_major_version_tag`, a `url_version_segment_policy`, an
   `unknown_field_policy`, and a typed `client_signal_header` set
   clients honor for deprecation and sunset.
4. **How does the client behave offline, and how does it discover
   the schema?** `offline_and_cache_posture` answers this.
   `schema_discovery_posture` MUST be one of
   `embedded_openapi_pinned_to_release`,
   `bundled_openapi_pinned_to_signed_mirror`, or
   `bundled_openapi_with_additive_minor_tolerance`. The fourth
   enum value, `live_discovery_against_service`, is explicitly
   forbidden by the schema.

Generic copy such as "service is offline", "version unsupported",
"please update", or "contact support" is not admissible on these
paths when a typed state is available from the rows and the
OpenAPI seed.

## Local-core non-dependence

The desktop core's first-run and startup commitments from the
managed-service seed extend to every HTTP API described here.

### First-run

On first launch with no account, no network, and no prior profile:

- the desktop shell MUST NOT call any path in
  `openapi/service_api_seed.yaml`;
- no API row may be required to resolve local settings, local
  workspace trust, local Git, local search, or the local editor;
- any built-in behavior that would normally consult a managed
  surface MUST degrade to the corresponding
  `local_fallback_posture` named on the row.

### Startup

On every subsequent launch, regardless of network or control-plane
reachability:

- startup MUST NOT block waiting for any API surface row at
  `service_opt_in_posture: always_off_unless_opted_in` or
  `off_until_account_created` (from the SLO row);
- startup MUST NOT perform a live schema-discovery request against
  any hosted service; the client pins to the embedded or mirrored
  OpenAPI document at release time;
- when a surface is reachable but in a degraded mode, the client
  renders the row's typed degradation mode and recovery cue
  (`Aureline-API-Degradation-Mode` and `Aureline-API-Recovery-Cue`
  response headers), not a loading spinner or a generic error.

### Forbidden couplings

Reviewers MUST reject any change that:

- adds a row whose `local_core_blocking` is `true`;
- adds a row whose `schema_discovery_posture` is
  `live_discovery_against_service`;
- adds a row whose `offline_behavior_class` silently collapses to
  "service unreachable means app unusable" without naming a
  `local_fallback_posture` other than
  `managed_only_no_local_fallback_narrows_managed_claim_only`;
- introduces a path that is not also represented as an
  `api_surface_id` row in `api_surface_rows.yaml`;
- introduces a row whose `data_classes` include
  `credential_material_reference` with `auth.auth_mode: open_no_auth`;
- routes degradation copy through a spinner or a generic error on a
  managed surface when a typed degradation mode from the SLO row is
  available.

## Row shape

Each row in `artifacts/service/api_surface_rows.yaml` is a
`managed_service_api_capability_row`. Required fields:

- `schema_version`, `row_kind` — row-shape versioning and
  discriminator.
- `api_surface_id` — stable `<service_id>.<capability_id>` id.
  Compat, docs, migration, deprecation, release, and support work
  cite this id verbatim.
- `api_surface_title` and `summary` — short product-term
  description. No milestone slugs.
- `service_id` — one value from the SLO
  `service_id_vocabulary`. Every surface is hosted by exactly one
  managed service.
- `api_family_class` — one value from the closed
  `api_family_class_vocabulary`.
- `maturity_lane` — `experimental` or `internal` at seed time.
  Promotion to `beta` or `stable` requires an explicit decision row
  and a non-null `openapi_document_ref`.
- `ownership` — owner DRI, owning lane, backup owner or waiver.
- `auth` — auth mode, entitlement class, policy override posture.
- `data_classes` — non-empty set drawn from the closed
  `data_class_vocabulary`.
- `local_fallback_posture` — what happens when the surface is
  unreachable.
- `local_core_blocking` — pinned `false` by the schema.
- `versioning` — rule class, current major-version tag, URL
  version-segment policy, unknown-field policy, and short
  summaries of the additive-minor and breaking rules.
- `deprecation_lane` — lane class, sunset posture, client-signal
  headers, notice-window summary.
- `offline_and_cache_posture` — offline behavior class, client
  cache policy class, last-known-good posture, schema-discovery
  posture, freshness-floor summary.
- `compat_binding` — the surface-contract packet row ref, the
  stable-surface inventory file ref, and the compatibility-window
  source row ref.
- `linked_record_class_ids`, `linked_slo_row_ref`, and
  `linked_retention_row_refs` — bind the row into the managed-
  service seed and the record-class registry.
- `openapi_document_ref` — pointer to
  `openapi/service_api_seed.yaml` (or a successor per-family
  document). Non-null at `beta` or `stable` maturity.
- `notes` — optional short operator-reviewable note.

## Dependency rules

- **Local-core independence.** Local desktop operation MUST NOT
  depend on reaching any path in `openapi/service_api_seed.yaml`.
  Clients MUST NOT perform live schema discovery against a hosted
  service; the OpenAPI document is pinned at release time or
  resolved through a signed mirror snapshot.
- **Shared compatibility inventory.** Every API surface row is
  published as a `surface_contract_packet` in
  `artifacts/governance/stable_surface_inventory.yaml`. Compat,
  docs, migration, deprecation, release, and support work cite the
  same surface id that other machine surfaces (WIT worlds, event
  envelopes, JSON Schema artifacts, record-class rows) already use.
- **Shared schema stack.** The OpenAPI document lives in the
  `service` schema family and cites
  `artifacts/governance/standards_matrix.yaml#standard.openapi_3_2`.
  JSON Schema component bodies inherit the cross-family Draft
  2020-12 pin and the per-family `additionalProperties: false`
  default.
- **Shared retention and deletion contract.** Every data class
  crossing an API boundary is pinned through
  `artifacts/service/retention_rows.yaml` and, when destructive,
  through `schemas/service/deletion_job_record.schema.json`. API
  rows do not mint parallel retention or deletion vocabularies.
- **No raw body leakage.** Raw request bytes, raw response bytes,
  raw user-supplied text, raw URLs, raw path bytes, and raw
  credential material never appear as literal examples in the
  OpenAPI seed, the row file, or this document. Schemas describe
  the shapes; fixtures, if needed, live under `/fixtures/`.

## Deprecation lane

Every surface declares one of four lane classes:

- `pre_release_no_deprecation_yet` — used while the surface is at
  `maturity_lane: experimental` or `internal`.
- `additive_only_no_removal_window` — the surface only adds
  fields; nothing is removed. The client-signal header set omits
  `Deprecation` and `Sunset` until a removal lane is chosen.
- `standard_overlap_with_sunset_header` — the default for
  stable-facing surfaces. Default overlap window is twelve months
  unless the row's `notice_window_summary` names a narrower one.
- `emergency_sunset_with_explicit_advisory` — reserved for
  security or compliance advisories that short-circuit the normal
  overlap. Requires an inspectable advisory ref.

Client-signal headers in scope:

- `Deprecation` — RFC 8594 Deprecation header.
- `Sunset` — RFC 8594 Sunset header.
- `Aureline-API-Version` — current major-version tag.
- `Aureline-API-Deprecation-Advisory` — advisory ref.
- `Aureline-API-Freshness-Floor` — machine-readable freshness
  floor so stale cache is labeled rather than silently served.
- `Aureline-API-Degradation-Mode` — the SLO row's degradation mode
  (`degraded_slow`, `degraded_stale_cache`,
  `unavailable_retry_later`, `unavailable_quota_exhausted`,
  `unavailable_entitlement_expired`, `unavailable_policy_blocked`,
  `mirror_only`, `boundary_recheck_required`).
- `Aureline-API-Recovery-Cue` — the per-mode recovery cue.

## Offline and cache posture

Every surface declares one offline behavior class, one cache
policy class, one last-known-good posture, one schema-discovery
posture, and a short freshness-floor summary. The schema forbids
`live_discovery_against_service` so clients do not accidentally
take a live runtime dependency on a hosted service schema
endpoint.

Canonical combinations used in this seed:

| api_family_class | offline_behavior_class | client_cache_policy_class | schema_discovery_posture |
| --- | --- | --- | --- |
| registry_api_family | bundled_mirror_snapshot_resolves | cache_bounded_by_service_freshness_floor | embedded_openapi_pinned_to_release |
| policy_distribution_api_family | last_known_good_local_cache_resolves | cache_authoritative_within_signed_freshness_floor | embedded_openapi_pinned_to_release |
| sync_api_family | queued_for_replay_on_recovery | cache_authoritative_within_signed_freshness_floor | embedded_openapi_pinned_to_release |
| identity_api_family | last_known_good_local_cache_resolves | cache_authoritative_within_signed_freshness_floor | embedded_openapi_pinned_to_release |
| catalog_api_family | bundled_mirror_snapshot_resolves | cache_bounded_by_service_freshness_floor | embedded_openapi_pinned_to_release |
| ai_broker_api_family | read_only_when_reachable_and_narrows_on_unreachable | no_cache_ephemeral_session_only | embedded_openapi_pinned_to_release |
| collaboration_relay_api_family | read_only_when_reachable_and_narrows_on_unreachable or queued_for_replay_on_recovery | no_cache_ephemeral_session_only | embedded_openapi_pinned_to_release |
| telemetry_ingest_api_family | queued_for_replay_on_recovery | no_cache_ephemeral_session_only | embedded_openapi_pinned_to_release |
| support_export_api_family | queued_for_replay_on_recovery | no_cache_ephemeral_session_only | embedded_openapi_pinned_to_release |
| entitlement_usage_api_family | last_known_good_local_cache_resolves or queued_for_replay_on_recovery | cache_authoritative_within_signed_freshness_floor or no_cache_ephemeral_session_only | embedded_openapi_pinned_to_release |
| offboarding_export_api_family | queued_for_replay_on_recovery or bundled_mirror_snapshot_resolves | cache_authoritative_within_signed_freshness_floor or no_cache_ephemeral_session_only | embedded_openapi_pinned_to_release or bundled_openapi_pinned_to_signed_mirror |
| docs_pack_api_family | bundled_mirror_snapshot_resolves | cache_bounded_by_service_freshness_floor | embedded_openapi_pinned_to_release |

## Rendering contract

Product surfaces that render a managed API's state consume the
frozen rows here. Specifically:

- availability, degradation, and recovery-cue copy renders from
  the `Aureline-API-Degradation-Mode` and
  `Aureline-API-Recovery-Cue` response headers and the matching
  SLO row, not from free text in error paths;
- deprecation and sunset copy renders from the `Deprecation`,
  `Sunset`, and `Aureline-API-Deprecation-Advisory` headers plus
  the row's `deprecation_lane` fields, not from ad-hoc banners;
- freshness labels render from
  `Aureline-API-Freshness-Floor` plus the row's
  `offline_and_cache_posture.freshness_floor_summary`, so stale
  cache is labeled rather than surfaced as live;
- retention, legal-hold, and delete copy continues to render
  through `retention_rows.yaml`, `record_class_registry.yaml`,
  and `deletion_job_record.schema.json`. API rows never mint
  parallel copy surfaces.

## Evolution rules

- Adding a new `api_surface_id`, `api_family_class`, `auth_mode`,
  `entitlement_class`, `policy_override_posture`, `data_class`,
  `local_fallback_posture`, `versioning_rule_class`,
  `url_version_segment_policy`, `unknown_field_policy`,
  `deprecation_lane_class`, `sunset_posture`,
  `client_signal_header`, `offline_behavior_class`,
  `client_cache_policy_class`, `last_known_good_posture`, or
  `schema_discovery_posture` value is additive-minor; bump the
  `schema_version` on the affected file. Repurposing an existing
  value is breaking and requires a new decision row in
  `artifacts/governance/decision_index.yaml`.
- Every new API surface lands an `api_surface_rows.yaml` row in
  the same change as the OpenAPI addition. The OpenAPI seed MUST
  NOT carry a path without a matching
  `x-aureline-api-surface-id`.
- Promoting a row from `experimental` to `beta` or `stable`
  requires a decision row, a non-null `openapi_document_ref`, a
  paired update to
  `artifacts/governance/stable_surface_inventory.yaml`, and a
  paired row (or refresh) in the qualification matrix.
- Introducing a new `api_family_class` follows the schema-family
  rules in `schema_families.yaml` (the family row MUST name the
  OpenAPI document and cite `standard.openapi_3_2`). The service
  family row already names `openapi/service_api_seed.yaml`; per-
  family splits remain additive-minor unless a row is retired.
- This document, the row schema, the YAML row file, and the
  OpenAPI seed stay in sync by review. Tooling MAY reject PRs that
  introduce a surface, vocabulary value, or path in only one of
  the four surfaces.
