# Locality, tenancy, key-mode, and local-core continuity seed

This document is the narrative seed for Aureline's shared locality /
tenancy / key-mode matrix, its typed control-plane versus data-plane
degrade model, and the local-core continuity packet every managed,
enterprise, mirror, or local-first claim resolves against. It exists
so later deployment rows — release planning, support bundles,
boundary-manifest rows, and DR drills — cannot hide locality or
resilience debt behind generic "service degraded" copy.

Companion artifacts:

- [`/artifacts/deployment/locality_matrix.yaml`](../../artifacts/deployment/locality_matrix.yaml)
  — machine-readable locality matrix with frozen vocabularies, per-
  profile locality rows, degrade-model mappings, and the seeded
  continuity-case index.
- [`/schemas/deployment/local_core_continuity_packet.schema.json`](../../schemas/deployment/local_core_continuity_packet.schema.json)
  — boundary schema for the `local_core_continuity_packet_record`
  consumed by support, boundary, and drill review surfaces.
- [`/fixtures/deployment/continuity_cases/`](../../fixtures/deployment/continuity_cases/)
  — concrete continuity packet fixtures the matrix points at.
- [`/docs/deployment/drill_catalog_seed.md`](./drill_catalog_seed.md)
  and
  [`/artifacts/support/deployment_drill_catalog_seed.yaml`](../../artifacts/support/deployment_drill_catalog_seed.yaml)
  — shared continuity and impairment drill catalog seed. Drill rows
  resolve locality posture against this seed.
- [`/docs/product/boundary_manifest_strawman.md`](../product/boundary_manifest_strawman.md)
  and
  [`/schemas/product/boundary_manifest.schema.json`](../../schemas/product/boundary_manifest.schema.json)
  — boundary-manifest rows whose `local_core_continuity` and
  `absence_narrows_to` clauses cite per-profile retained-local-safe
  and blocked-managed-only claims from this seed.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  and
  [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json)
  — support bundles consume this seed's locality, tenant, region, and
  key-mode vocabulary directly instead of re-minting one.
- [`/artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
  — control-artifact routing for this seed and its owning lane.

Normative sources:

- `.t2/docs/Aureline_PRD.md` §5.24, §5.53, §5.57, and Appendix AN.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §9.6, §9.7,
  and §9.8.
- `.t2/docs/Aureline_Technical_Design_Document.md` §11.4.2 and
  §11.4.3.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §18.31, §18.42, and
  Appendix BL.

If this document disagrees with those sources, those sources win and
this document plus the YAML matrix update in the same change. If this
document and the YAML matrix disagree, this document wins and the
YAML updates in the same change.

## Scope

Frozen at this revision:

- one locality-row shape shared by support bundles, boundary-manifest
  planning, drill-catalog review, and DR drill artefacts;
- a closed deployment-profile vocabulary (`individual_local`,
  `self_hosted`, `enterprise_online`, `air_gapped`, `managed_cloud`)
  pulled forward from the boundary-manifest strawman and the
  deployment drill catalog seed;
- closed processing-location, storage-location, tenant/org-scope,
  region-scope, retention-class, and key-mode vocabularies so every
  managed, enterprise, and self-hosted row carries an actionable
  posture rather than a silent blank;
- a closed control-plane service vocabulary (sync, registry, relay,
  AI broker, auth/identity, policy, docs-pack, catalog, telemetry
  sink) and a typed state vocabulary (`healthy`, `stale_cache`,
  `unavailable`, `mirror_only`, `boundary_recheck_required`,
  `not_applicable`);
- a closed data-plane capability vocabulary (local editing, save,
  search, Git, tasks, docs inspection, export, diagnostics) and a
  typed state vocabulary (`available_local_safe`,
  `available_mirror_backed`, `reduced_read_only`,
  `blocked_pending_reconnect`, `blocked_pending_boundary_recheck`,
  `blocked_by_policy`, `not_applicable`);
- a closed restore-class vocabulary re-exported from the deployment
  drill catalog seed;
- a freshness posture with `fresh`, `bounded_stale`, and
  `unbounded_stale` classes carrying freshness-floor refs or
  stale rationale;
- mirror-continuity and air-gap-continuity field sets the schema
  requires whenever any control-plane service is mirror-only or the
  deployment profile is `air_gapped`;
- seeded continuity-case fixtures covering individual-local baseline,
  self-hosted stale-policy cached-local-safe, air-gapped mirror-only
  docs, managed-cloud relay disconnect, and enterprise failover with
  explicit boundary recheck.

Out of scope at this revision:

- live multi-region failover or DR implementation;
- a managed-service operating runbook, production runbook pages, or
  the wire protocol of any managed control plane;
- release-evidence claim manifests or support-bundle bodies (those
  compose over this seed).

## The two questions the seed answers

Any Aureline surface claiming managed, enterprise, mirror, or
local-first behavior MUST answer both questions mechanically, without
relying on human-written copy:

1. **Where does the row sit?** Which deployment profile, processing
   location, storage location, tenant/org scope, region scope,
   retention class, and key mode is in force?
2. **What remains true when a service impairs?** Which control-plane
   service class is impaired, which data-plane capabilities remain
   available (local-safe or mirror-backed), which managed-only
   capabilities are blocked, what freshness posture holds, and which
   restore class collapses the row back to a product-term next step?

Generic copy such as "service unavailable", "loading", or "try again"
is forbidden on these paths when a more precise state is known.

## Locality matrix

Every deployment profile row in the YAML matrix carries:

- a stable `deployment_profile` value;
- a `locality_posture` block pinning `processing_location_class`,
  `storage_location_class`, `tenant_org_scope_class`,
  `region_scope_class`, `retention_class`, and `key_mode_class`;
- a `control_plane_service_posture` list with one default state per
  service class in the frozen vocabulary;
- a `data_plane_capability_posture` list with one default state per
  capability class in the frozen vocabulary;
- a `default_restore_class` the profile's outages collapse back to;
- a `mirror_or_air_gap_applies` flag so mirror and offline bundle
  truth is never silently implied;
- one or more `per_profile_drill_refs` into the deployment drill
  catalog seed.

### Required locality truth

| Deployment profile | Processing location | Storage location | Tenant/org scope | Region scope | Retention class | Key mode |
|---|---|---|---|---|---|---|
| `individual_local`  | `on_device_only`                    | `device_local_disk`            | `single_user_local`                | `not_applicable`          | `no_retention_beyond_local_disk`        | `os_store` |
| `self_hosted`       | `customer_control_plane`            | `customer_control_plane_storage` | `customer_tenant`               | `customer_region_pinned`  | `customer_retention_window`             | `customer_managed` |
| `enterprise_online` | `customer_or_vendor_control_plane`  | `customer_control_plane_storage` | `customer_tenant`               | `customer_region_pinned`  | `vendor_retention_window_with_customer_policy` | `customer_managed` |
| `air_gapped`        | `on_device_only`                    | `mirror_or_offline_bundle`     | `customer_tenant`                  | `customer_region_pinned`  | `customer_retention_window`             | `offline_trust_root` |
| `managed_cloud`     | `vendor_control_plane`              | `vendor_control_plane_storage` | `customer_tenant`                  | `customer_region_pinned`  | `vendor_retention_window_default`       | `vendor_managed` |

**Managed, enterprise, and self-hosted rows MUST NOT set
`tenant_org_scope`, `region_scope`, or `key_mode` to
`not_applicable`.** The schema enforces this on every continuity
packet; the matrix enforces it on every profile row.

## Typed degrade model

The matrix binds control-plane impairment to data-plane continuity:

- **Control-plane service vocabulary.** Every service class is named
  explicitly: `sync_service`, `registry_service`, `relay_service`,
  `ai_broker_service`, `auth_identity_service`, `policy_service`,
  `docs_pack_service`, `catalog_service`, `telemetry_sink_service`.
  No control-plane outage MAY be described as a generic "service
  degraded" condition on a surface that consumes this seed.
- **Control-plane state vocabulary.** `healthy`, `stale_cache`,
  `unavailable`, `mirror_only`, `boundary_recheck_required`, and
  `not_applicable`. `mirror_only` requires populated
  mirror-continuity fields; `boundary_recheck_required` forces a
  `manual_reconcile_after_boundary_change` restore class.
- **Data-plane capability vocabulary.** `local_editing`,
  `local_save`, `local_search`, `local_git`, `local_tasks`,
  `local_docs_inspect`, `local_export`, `local_diagnostics`. Packets
  MUST state each class explicitly; silent omission is forbidden.
- **Data-plane state vocabulary.** `available_local_safe`,
  `available_mirror_backed`, `reduced_read_only`,
  `blocked_pending_reconnect`, `blocked_pending_boundary_recheck`,
  `blocked_by_policy`, `not_applicable`.

The `degrade_model.mappings` block in the YAML pins the minimum
data-plane baseline every non-healthy control-plane state MUST
preserve. For example, `stale_cache` and `unavailable` states MUST
keep `local_editing`, `local_save`, and `local_export` at
`available_local_safe` unless the packet names an explicit
narrowing.

## Local-core continuity packet

The packet schema freezes a `local_core_continuity_packet_record`
shape every continuity case resolves into. Required fields:

- `packet_id`, `deployment_profile`, optional `scenario_summary`;
- `locality_posture` carrying the six required locality values
  above;
- `control_plane_state_by_service_class` — at least one entry, one
  per service class in play;
- `data_plane_state_by_capability_class` — at least one entry, one
  per capability class in play;
- `retained_local_safe_capabilities` — product-term sentences of
  what remains available;
- `blocked_managed_only_capabilities` — product-term sentences of
  what is intentionally blocked (empty only when no managed form
  exists);
- `freshness_posture` — `fresh`, `bounded_stale` (with
  `freshness_floor_ref`), or `unbounded_stale` (with
  `staleness_rationale`);
- `restore_class` — one of the five restore classes;
- `mirror_continuity_fields` — required when any service is
  `mirror_only` or the profile is `air_gapped`;
- `air_gap_continuity_fields` — required when the profile is
  `air_gapped`;
- `linked_drill_refs`, `linked_boundary_rows`, `evidence_outputs`,
  and `narrative_refs`.

### Mirror-only and air-gap continuity fields

Mirror-continuity fields:

- `mirror_snapshot_id` (opaque, raw URLs and raw paths never appear);
- `mirror_freshness_class` (`mirror_fresh_within_window`,
  `mirror_within_extended_window`, `mirror_past_extended_window`,
  `mirror_freshness_unknown`, `not_applicable`);
- `signed_bundle_trust_root_fingerprint` (opaque fingerprint);
- optional `last_mirror_refresh_at`;
- optional `mirror_source_class`.

Air-gap-continuity fields:

- `offline_bundle_id`;
- `offline_bundle_trust_root_fingerprint`;
- `last_import_at`;
- `import_source_class` (`customer_sneakernet_bundle`,
  `customer_signed_registry_export`, `vendor_signed_offline_bundle`,
  `certified_archetype_seed`, `not_applicable`);
- optional `disallows_public_internet_egress`.

## Consumption rules

- **Support bundles** (`schemas/support/support_bundle.schema.json`)
  reuse deployment-profile, region-scope, tenant-scope, and key-mode
  vocabulary directly. Bundles cite `local_core_continuity_packet_record`
  packets rather than minting private locality fields.
- **Boundary-manifest rows** map `local_core_continuity` and
  `absence_narrows_to` clauses to per-profile retained-local-safe
  and blocked-managed-only claims in this seed whenever optional
  services, mirrors, or cached policy/auth truth are in play.
- **Deployment drill catalog** rows resolve locality posture,
  control-plane state, and data-plane state against this seed; the
  drill catalog's `evidence_output_vocabulary`, `restore_class_vocabulary`,
  and `deployment_profile_vocabulary` are re-exported from the same
  source.
- **DR drill artifacts** cite a `local_core_continuity_packet_record`
  per exercised posture rather than free-text locality claims.

## Coverage guards

Tooling SHOULD verify that every vocabulary value is exercised by at
least one seeded continuity case or per-profile drill ref:

- every `deployment_profile` has at least one continuity case;
- every `control_plane_service_state_class` other than
  `not_applicable` is exercised on at least one fixture;
- every `data_plane_capability_state_class` other than
  `not_applicable` is exercised on at least one fixture;
- every `restore_class` is exercised on at least one fixture;
- every `staleness_class` is exercised on at least one fixture.

These guards are advisory at this revision; a later milestone may
promote them to required CI checks.

## Future extension rules

- Adding a new deployment profile, control-plane service class,
  data-plane capability class, restore class, or staleness class is
  **additive-minor**: bump `schema_version` in the matrix and schema
  and update the narrative seed in the same change.
- Repurposing an existing vocabulary value is **breaking**. Open a
  decision row in
  `artifacts/governance/decision_index.yaml` before landing the
  change; do not silently shift existing semantics.
- Claim-widening for managed, enterprise, sovereign, or air-gapped
  language should add or refresh a continuity-case fixture here
  before the public claim widens elsewhere.
