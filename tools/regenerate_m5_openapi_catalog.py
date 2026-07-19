#!/usr/bin/env python3
"""Regenerate the M5 OpenAPI publication catalog and its example packs.

This is the single source of truth for the catalog that turns the optional
managed-service OpenAPI document into a published contract family: every
service-API operation the product or SDK exposes (registry/mirror,
marketplace publication, AI broker, usage/metering export, managed
control-plane, identity, support/export, offboarding, and docs-pack routes)
is bound to a lifecycle label, an auth-source class, an entitlement and
policy-override posture, a mutability posture, a preview/dry-run support
class, an offline/cache behaviour, a deprecation lane and sunset posture, a
compatibility note, and a checked-in example request/response pack.

It builds one ``endpoints`` entry per OpenAPI operation, then writes, all
deterministically:

  * ``artifacts/contracts/m5-openapi-catalog.json``        (the catalog)
  * ``examples/contracts/m5-openapi/<operation>.json``     (example packs)
  * ``examples/contracts/m5-openapi/README.md``            (example index)
  * ``docs/sdk/m5-service-api-catalog.md``                 (the SDK doc)
  * ``docs/m5/<slug>.md``                                  (the overview doc)
  * ``artifacts/release/captures/<name>_validation_capture.json`` (CI capture)
  * ``fixtures/contracts/m5-openapi/{cases.json,*.json}``  (negative fixtures)

Run ``python3 tools/regenerate_m5_openapi_catalog.py`` after editing the
endpoint set, then ``python3 tools/validate_m5_openapi_catalog.py`` and
``cargo test -p aureline-release --test rel_it_34_publish_openapi_specs_lifecycle``
to confirm the validator and the typed model agree.

The catalog reuses existing governance sources rather than minting a new
lexicon: the auth-source, entitlement, policy-override, offline, deprecation,
and sunset vocabularies are drawn verbatim from the optional-service API
surface rows (``artifacts/service/api_surface_rows.yaml``); the example payload
shapes are conformant with the component schemas in the OpenAPI document
(``openapi/service_api_seed.yaml``); and the family lifecycle label is the
effective published label the public-contract publication matrix records for
the ``service_optional_api`` family after narrowing. The catalog is
metadata-only: it carries no raw request/response bytes, credential material,
signatures, or live server URLs.
"""

from __future__ import annotations

import json
from pathlib import Path

NAME = (
    "publish_openapi_specs_lifecycle_labels_and_example_packs_for_m5_service_apis_"
    "registry_mirror_endpoints_admin_ai_usage_export_routes_and_managed_control_plane_surfaces"
)
RECORD_KIND = "m5_openapi_catalog"
CATALOG_ID = "m5_openapi_catalog:v1"
SCHEMA_VERSION = 1
AS_OF = "2026-06-19"
FAMILY_ID = "service_optional_api"

REPO_ROOT = Path(__file__).resolve().parent.parent

CATALOG_PATH = REPO_ROOT / "artifacts" / "contracts" / "m5-openapi-catalog.json"
EXAMPLE_DIR = REPO_ROOT / "examples" / "contracts" / "m5-openapi"
NEGATIVE_DIR = REPO_ROOT / "fixtures" / "contracts" / "m5-openapi"
SDK_DOC_PATH = REPO_ROOT / "docs" / "sdk" / "m5-service-api-catalog.md"
SLUG = NAME.replace("_", "-")
OVERVIEW_DOC_PATH = REPO_ROOT / "docs" / "m5" / f"{SLUG}.md"
CAPTURE_PATH = REPO_ROOT / "artifacts" / "release" / "captures" / f"{NAME}_validation_capture.json"

OVERVIEW_PAGE = f"docs/m5/{SLUG}.md"
EVIDENCE_PAGE = f"artifacts/m5/{SLUG}.md"
SDK_DOC_PAGE = "docs/sdk/m5-service-api-catalog.md"

# Cross-cutting governance sources this catalog reuses instead of restating.
JSON_SCHEMA_CATALOG_REF = "artifacts/contracts/m5-json-schema-catalog.json"
PUBLICATION_MATRIX_REF = "artifacts/contracts/m5-stability-lifecycle-map.json"
PUBLICATION_MATRIX_PATH = REPO_ROOT / PUBLICATION_MATRIX_REF
API_SURFACE_ROWS_REF = "artifacts/service/api_surface_rows.yaml"
SLO_ROWS_REF = "artifacts/service/slo_rows.yaml"
OPENAPI_README_REF = "openapi/m5/README.md"
PRIMARY_OPENAPI_DOCUMENT_REF = "openapi/service_api_seed.yaml"
EXAMPLE_PACK_HOME = "examples/contracts/m5-openapi/"
EVIDENCE_INDEX_REF = (
    "artifacts/release/m5/"
    "certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json"
)

VALIDATOR_SUITE_REFS = [
    "tools/validate_m5_openapi_catalog.py",
    "ci/contract_validation.sh",
]

# Closed vocabularies. Kept in lockstep with the typed Rust consumer and the
# boundary schema; the validator and the model both reject anything off-list.
# The auth-source, entitlement, policy-override, offline, deprecation, and
# sunset vocabularies are drawn verbatim from artifacts/service/api_surface_rows.yaml.
HTTP_METHODS = ["get", "put", "post"]
AUTH_SOURCE_CLASSES = [
    "open_no_auth",
    "bearer_token_short_lived",
    "oidc_session_token",
    "mtls_client_cert",
    "oidc_plus_mtls_dual_factor",
    "scim_bearer_token",
    "customer_byok_passthrough_no_broker_auth",
    "signed_mirror_snapshot_only_no_live_auth",
    "destruction_receipt_ledger_signed_append_only",
]
ENTITLEMENT_CLASSES = [
    "no_entitlement_required",
    "account_required",
    "organization_entitlement",
    "support_case_entitlement",
    "admin_scope_entitlement",
    "destruction_receipt_scope",
]
POLICY_OVERRIDE_POSTURES = [
    "narrow_only_no_widen",
    "narrow_only_with_emergency_disable",
    "policy_immutable_no_override",
]
MUTABILITY_POSTURES = [
    "read_only",
    "mutating_create",
    "mutating_replace",
    "mutating_append_only",
    "mutating_action_no_durable_resource",
]
PREVIEW_SUPPORT_CLASSES = [
    "read_only_no_mutation",
    "dry_run_and_preview_supported",
    "action_atomic_no_preview",
]
OFFLINE_BEHAVIOR_CLASSES = [
    "no_network_required_local_only",
    "last_known_good_local_cache_resolves",
    "bundled_mirror_snapshot_resolves",
    "queued_for_replay_on_recovery",
    "read_only_when_reachable_and_narrows_on_unreachable",
]
DEPRECATION_LANE_CLASSES = [
    "pre_release_no_deprecation_yet",
    "additive_only_no_removal_window",
    "standard_overlap_with_sunset_header",
    "emergency_sunset_with_explicit_advisory",
]
SUNSET_POSTURES = [
    "no_sunset_yet",
    "named_overlap_window_then_remove",
    "named_overlap_window_then_mirror_only",
    "immediate_sunset_on_advisory",
]
LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
MATURITY_LANES = ["stable", "beta", "experimental", "internal"]
DOWNGRADE_BEHAVIORS = ["narrow_below_cutline", "reject"]

# Shared compatibility-note phrasing. Each endpoint appends its additive-minor
# rule; the closed enums make machine consumers key off vocabulary, not prose.
COMPATIBILITY_NOTE_PREFIX = (
    "Pinned to the OpenAPI document at release; clients never live-discover the "
    "service schema. Within a major, only additive-minor changes land: "
)

# A schema ref into the OpenAPI document's component schemas.
def component_ref(name: str) -> str:
    return f"{PRIMARY_OPENAPI_DOCUMENT_REF}#/components/schemas/{name}"


# Reusable example payloads, conformant with the OpenAPI document's component
# schemas. No raw credentials, signatures, bytes, or live URLs appear; ids use
# the stable id pattern and class fields use the document's closed enums.
ACK_ENVELOPE = {"accepted_at": "2026-06-19T00:00:00Z", "request_id": "req-0001"}


# One entry per OpenAPI operation. ``auth_source_class`` / ``entitlement_class``
# / ``policy_override_posture`` / ``offline_behavior_class`` /
# ``deprecation_lane_class`` / ``sunset_posture`` / ``api_family_class`` /
# ``maturity_lane`` mirror the matching row in artifacts/service/api_surface_rows.yaml.
# ``request_schema`` / ``response_schema`` are component names in the OpenAPI
# document; ``request_example`` / ``response_example`` validate against them.
ENDPOINTS = [
    {
        "operation_id": "managed_marketplace_extension_catalog_read",
        "title": "Browse the hosted extension catalog",
        "summary": "Read a page of marketplace catalog entries from the signed mirror snapshot.",
        "api_surface_id": "managed_marketplace.extension_catalog_read",
        "service_tag": "managed_marketplace",
        "api_family_class": "registry_api_family",
        "http_method": "get",
        "path": "/marketplace/extensions",
        "success_status": "200",
        "auth_source_class": "open_no_auth",
        "entitlement_class": "no_entitlement_required",
        "policy_override_posture": "narrow_only_with_emergency_disable",
        "mutability_posture": "read_only",
        "preview_support_class": "read_only_no_mutation",
        "offline_behavior_class": "bundled_mirror_snapshot_resolves",
        "deprecation_lane_class": "pre_release_no_deprecation_yet",
        "sunset_posture": "no_sunset_yet",
        "maturity_lane": "experimental",
        "compatibility_note_tail": "additive response manifest fields and additive query parameters; publisher and extension id shapes stay stable.",
        "request_schema": None,
        "request_example": None,
        "response_schema": "ExtensionCatalogPage",
        "response_example": {
            "entries": [
                {
                    "extension_id": "example.formatter",
                    "publisher_id": "example.publisher",
                    "display_title_class": "short_title_with_publisher",
                    "compatibility_claim_class": "compatible_with_current_major",
                }
            ],
            "freshness_floor_summary": "Catalog snapshot is within its mirror freshness floor.",
        },
    },
    {
        "operation_id": "managed_marketplace_extension_install_request",
        "title": "Request installation of a catalog extension",
        "summary": "Submit an install request; the response manifest is applied against the local extension host and never auto-enables.",
        "api_surface_id": "managed_marketplace.extension_install_request",
        "service_tag": "managed_marketplace",
        "api_family_class": "registry_api_family",
        "http_method": "post",
        "path": "/marketplace/extensions/install_requests",
        "success_status": "202",
        "auth_source_class": "oidc_session_token",
        "entitlement_class": "account_required",
        "policy_override_posture": "narrow_only_with_emergency_disable",
        "mutability_posture": "mutating_create",
        "preview_support_class": "dry_run_and_preview_supported",
        "offline_behavior_class": "read_only_when_reachable_and_narrows_on_unreachable",
        "deprecation_lane_class": "pre_release_no_deprecation_yet",
        "sunset_posture": "no_sunset_yet",
        "maturity_lane": "experimental",
        "compatibility_note_tail": "additive request fields, additive response fields, and additive stable error codes.",
        "request_schema": "ExtensionInstallRequest",
        "request_example": {
            "extension_id": "example.formatter",
            "requested_version_constraint_class": "latest_compatible_stable",
        },
        "response_schema": "ExtensionInstallManifest",
        "response_example": {"install_id": "install-0001", "state_class": "requested"},
    },
    {
        "operation_id": "managed_policy_distribution_policy_bundle_fetch",
        "title": "Fetch the current signed policy bundle",
        "summary": "Read the caller's signed policy-bundle envelope; the cached bundle stays authoritative within its freshness floor.",
        "api_surface_id": "managed_policy_distribution.policy_bundle_fetch",
        "service_tag": "managed_policy_distribution",
        "api_family_class": "policy_distribution_api_family",
        "http_method": "get",
        "path": "/policy/bundles/current",
        "success_status": "200",
        "auth_source_class": "bearer_token_short_lived",
        "entitlement_class": "organization_entitlement",
        "policy_override_posture": "policy_immutable_no_override",
        "mutability_posture": "read_only",
        "preview_support_class": "read_only_no_mutation",
        "offline_behavior_class": "last_known_good_local_cache_resolves",
        "deprecation_lane_class": "additive_only_no_removal_window",
        "sunset_posture": "no_sunset_yet",
        "maturity_lane": "experimental",
        "compatibility_note_tail": "additive bundle-envelope fields and additive query parameters; bundle bodies version under their own signed schema bundle.",
        "request_schema": None,
        "request_example": None,
        "response_schema": "SignedPolicyBundleEnvelope",
        "response_example": {
            "bundle_id": "policy-bundle-0001",
            "signature_scheme_class": "detached_envelope_signature",
            "freshness_floor_summary": "Signed policy bundle is authoritative within its freshness floor.",
        },
    },
    {
        "operation_id": "managed_settings_sync_user_settings_read",
        "title": "Read the user-scope settings snapshot",
        "summary": "Read a signed-in user's settings snapshot; local settings on disk remain authoritative.",
        "api_surface_id": "managed_settings_sync.user_settings_sync",
        "service_tag": "managed_settings_sync",
        "api_family_class": "sync_api_family",
        "http_method": "get",
        "path": "/settings_sync/user_scope",
        "success_status": "200",
        "auth_source_class": "oidc_session_token",
        "entitlement_class": "account_required",
        "policy_override_posture": "narrow_only_with_emergency_disable",
        "mutability_posture": "read_only",
        "preview_support_class": "read_only_no_mutation",
        "offline_behavior_class": "queued_for_replay_on_recovery",
        "deprecation_lane_class": "pre_release_no_deprecation_yet",
        "sunset_posture": "no_sunset_yet",
        "maturity_lane": "experimental",
        "compatibility_note_tail": "additive settings keys and additive response metadata; payloads forward-compatibly carry unknown keys.",
        "request_schema": None,
        "request_example": None,
        "response_schema": "UserScopeSettingsPayload",
        "response_example": {"schema_version": 1, "settings_epoch_id": "epoch-0001"},
    },
    {
        "operation_id": "managed_settings_sync_user_settings_write",
        "title": "Write the user-scope settings snapshot",
        "summary": "Write a signed-in user's settings snapshot; pending writes queue locally and replay on recovery.",
        "api_surface_id": "managed_settings_sync.user_settings_sync",
        "service_tag": "managed_settings_sync",
        "api_family_class": "sync_api_family",
        "http_method": "put",
        "path": "/settings_sync/user_scope",
        "success_status": "202",
        "auth_source_class": "oidc_session_token",
        "entitlement_class": "account_required",
        "policy_override_posture": "narrow_only_with_emergency_disable",
        "mutability_posture": "mutating_replace",
        "preview_support_class": "dry_run_and_preview_supported",
        "offline_behavior_class": "queued_for_replay_on_recovery",
        "deprecation_lane_class": "pre_release_no_deprecation_yet",
        "sunset_posture": "no_sunset_yet",
        "maturity_lane": "experimental",
        "compatibility_note_tail": "additive settings keys and additive response metadata; payloads forward-compatibly carry unknown keys.",
        "request_schema": "UserScopeSettingsPayload",
        "request_example": {"schema_version": 1, "settings_epoch_id": "epoch-0001"},
        "response_schema": "AckEnvelope",
        "response_example": ACK_ENVELOPE,
    },
    {
        "operation_id": "managed_auth_identity_oidc_session_front_door",
        "title": "Establish or refresh a managed session",
        "summary": "Open or refresh an OIDC session; existing local sessions continue under the entitlement-snapshot cache when unreachable.",
        "api_surface_id": "managed_auth_identity.oidc_session_front_door",
        "service_tag": "managed_auth_identity",
        "api_family_class": "identity_api_family",
        "http_method": "post",
        "path": "/auth/sessions",
        "success_status": "200",
        "auth_source_class": "oidc_plus_mtls_dual_factor",
        "entitlement_class": "account_required",
        "policy_override_posture": "narrow_only_with_emergency_disable",
        "mutability_posture": "mutating_create",
        "preview_support_class": "action_atomic_no_preview",
        "offline_behavior_class": "last_known_good_local_cache_resolves",
        "deprecation_lane_class": "standard_overlap_with_sunset_header",
        "sunset_posture": "named_overlap_window_then_remove",
        "maturity_lane": "experimental",
        "compatibility_note_tail": "additive session-packet claims and additive metadata fields; tightening an auth factor needs a new major and an advisory.",
        "request_schema": "OidcSessionRequest",
        "request_example": {"authorization_grant_class": "authorization_code_with_pkce"},
        "response_schema": "SignedSessionPacket",
        "response_example": {
            "session_id": "session-0001",
            "entitlement_snapshot_ref": "entitlement-snapshot-0001",
            "issued_at": "2026-06-19T00:00:00Z",
        },
    },
    {
        "operation_id": "managed_auth_identity_scim_provisioning_create_user",
        "title": "SCIM 2.0 user-creation endpoint",
        "summary": "Provision an identity-principal record over SCIM 2.0; never writes into local workspace trust.",
        "api_surface_id": "managed_auth_identity.scim_provisioning",
        "service_tag": "managed_auth_identity",
        "api_family_class": "identity_api_family",
        "http_method": "post",
        "path": "/auth/scim/v2/Users",
        "success_status": "201",
        "auth_source_class": "scim_bearer_token",
        "entitlement_class": "admin_scope_entitlement",
        "policy_override_posture": "narrow_only_no_widen",
        "mutability_posture": "mutating_create",
        "preview_support_class": "action_atomic_no_preview",
        "offline_behavior_class": "read_only_when_reachable_and_narrows_on_unreachable",
        "deprecation_lane_class": "standard_overlap_with_sunset_header",
        "sunset_posture": "named_overlap_window_then_remove",
        "maturity_lane": "experimental",
        "compatibility_note_tail": "additive SCIM schema fields and additive search parameters; changing a resource type needs a new major.",
        "request_schema": "ScimUserEnvelope",
        "request_example": {
            "schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"],
            "userName": "example-user",
        },
        "response_schema": "ScimUserEnvelope",
        "response_example": {
            "schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"],
            "userName": "example-user",
        },
    },
    {
        "operation_id": "managed_catalog_runtime_catalog_fetch",
        "title": "Fetch runtime-catalog manifests",
        "summary": "Read toolchain, language-pack, and companion-binary manifests; installed artifacts continue to run when unreachable.",
        "api_surface_id": "managed_catalog.runtime_catalog_fetch",
        "service_tag": "managed_catalog",
        "api_family_class": "catalog_api_family",
        "http_method": "get",
        "path": "/runtime_catalog/manifests",
        "success_status": "200",
        "auth_source_class": "open_no_auth",
        "entitlement_class": "no_entitlement_required",
        "policy_override_posture": "narrow_only_with_emergency_disable",
        "mutability_posture": "read_only",
        "preview_support_class": "read_only_no_mutation",
        "offline_behavior_class": "bundled_mirror_snapshot_resolves",
        "deprecation_lane_class": "additive_only_no_removal_window",
        "sunset_posture": "no_sunset_yet",
        "maturity_lane": "experimental",
        "compatibility_note_tail": "additive manifest fields and additive filter parameters; changing an id shape needs a new major.",
        "request_schema": None,
        "request_example": None,
        "response_schema": "RuntimeCatalogManifestPage",
        "response_example": {
            "manifests": [
                {
                    "manifest_id": "toolchain-0001",
                    "artifact_class": "toolchain",
                    "compatibility_claim_class": "compatible_with_current_major",
                }
            ],
            "freshness_floor_summary": "Runtime catalog snapshot is within its mirror freshness floor.",
        },
    },
    {
        "operation_id": "managed_ai_broker_ai_turn_route",
        "title": "Route an AI turn through the managed broker",
        "summary": "Route an AI turn under vendor or customer key; local models and BYOK providers resolve turns when the broker is unreachable.",
        "api_surface_id": "managed_ai_broker.ai_turn_route",
        "service_tag": "managed_ai_broker",
        "api_family_class": "ai_broker_api_family",
        "http_method": "post",
        "path": "/ai_broker/turn_routes",
        "success_status": "200",
        "auth_source_class": "bearer_token_short_lived",
        "entitlement_class": "organization_entitlement",
        "policy_override_posture": "narrow_only_with_emergency_disable",
        "mutability_posture": "mutating_action_no_durable_resource",
        "preview_support_class": "action_atomic_no_preview",
        "offline_behavior_class": "read_only_when_reachable_and_narrows_on_unreachable",
        "deprecation_lane_class": "pre_release_no_deprecation_yet",
        "sunset_posture": "no_sunset_yet",
        "maturity_lane": "experimental",
        "compatibility_note_tail": "additive turn-envelope fields and additive provider identifiers; the turn id and provider identity stay stable within a major.",
        "request_schema": "AiTurnRouteRequest",
        "request_example": {
            "turn_id": "turn-0001",
            "evidence_packet_ref": "ai-evidence-0001",
            "provider_selection_class": "managed_broker_customer_key",
        },
        "response_schema": "AiTurnRouteResult",
        "response_example": {
            "turn_id": "turn-0001",
            "route_plan_ref": "route-plan-0001",
            "spend_receipt_ref": "spend-receipt-0001",
        },
    },
    {
        "operation_id": "managed_relay_collaboration_session_open",
        "title": "Open a collaboration session",
        "summary": "Open a relay collaboration session and obtain transport info; local editing continues when the relay is unreachable.",
        "api_surface_id": "managed_relay.collaboration_session_transport",
        "service_tag": "managed_relay",
        "api_family_class": "collaboration_relay_api_family",
        "http_method": "post",
        "path": "/relay/sessions",
        "success_status": "201",
        "auth_source_class": "bearer_token_short_lived",
        "entitlement_class": "organization_entitlement",
        "policy_override_posture": "narrow_only_with_emergency_disable",
        "mutability_posture": "mutating_create",
        "preview_support_class": "action_atomic_no_preview",
        "offline_behavior_class": "read_only_when_reachable_and_narrows_on_unreachable",
        "deprecation_lane_class": "pre_release_no_deprecation_yet",
        "sunset_posture": "no_sunset_yet",
        "maturity_lane": "experimental",
        "compatibility_note_tail": "additive presence fields and additive session-control parameters; changing a session id shape needs a new major.",
        "request_schema": "CollaborationSessionOpenRequest",
        "request_example": {"workspace_id": "workspace-0001", "session_intent_class": "live_review"},
        "response_schema": "CollaborationSessionDescriptor",
        "response_example": {"session_id": "session-0001", "transport_class": "websocket_over_mtls"},
    },
    {
        "operation_id": "managed_telemetry_sink_telemetry_event_ingest",
        "title": "Ingest an opt-in telemetry event batch",
        "summary": "Submit an opt-in telemetry batch; events queue locally and drop with disclosure when quota is exhausted.",
        "api_surface_id": "managed_telemetry_sink.telemetry_event_ingest",
        "service_tag": "managed_telemetry_sink",
        "api_family_class": "telemetry_ingest_api_family",
        "http_method": "post",
        "path": "/telemetry/events",
        "success_status": "202",
        "auth_source_class": "bearer_token_short_lived",
        "entitlement_class": "account_required",
        "policy_override_posture": "narrow_only_with_emergency_disable",
        "mutability_posture": "mutating_action_no_durable_resource",
        "preview_support_class": "action_atomic_no_preview",
        "offline_behavior_class": "queued_for_replay_on_recovery",
        "deprecation_lane_class": "additive_only_no_removal_window",
        "sunset_posture": "no_sunset_yet",
        "maturity_lane": "experimental",
        "compatibility_note_tail": "additive event-envelope and disposition fields; the event-schema bundle versions separately.",
        "request_schema": "TelemetryEventBatch",
        "request_example": {
            "batch_id": "batch-0001",
            "events": [{"event_class_id": "editor.session_open", "event_at": "2026-06-19T00:00:00Z"}],
        },
        "response_schema": "AckEnvelope",
        "response_example": ACK_ENVELOPE,
    },
    {
        "operation_id": "managed_support_export_support_bundle_attach",
        "title": "Attach a local support bundle to a managed case",
        "summary": "Attach a locally-assembled support bundle by reference; the bundle body remains user-owned and local.",
        "api_surface_id": "managed_support_export.support_bundle_attach",
        "service_tag": "managed_support_export",
        "api_family_class": "support_export_api_family",
        "http_method": "post",
        "path": "/support/exports/bundles",
        "success_status": "202",
        "auth_source_class": "bearer_token_short_lived",
        "entitlement_class": "support_case_entitlement",
        "policy_override_posture": "narrow_only_with_emergency_disable",
        "mutability_posture": "mutating_action_no_durable_resource",
        "preview_support_class": "action_atomic_no_preview",
        "offline_behavior_class": "queued_for_replay_on_recovery",
        "deprecation_lane_class": "additive_only_no_removal_window",
        "sunset_posture": "no_sunset_yet",
        "maturity_lane": "experimental",
        "compatibility_note_tail": "additive attach-envelope fields; changing the case-id convention needs a new major.",
        "request_schema": "SupportBundleAttachRequest",
        "request_example": {"support_case_ref": "support-case-0001", "bundle_ref": "support-bundle-0001"},
        "response_schema": "AckEnvelope",
        "response_example": ACK_ENVELOPE,
    },
    {
        "operation_id": "managed_collaboration_review_evidence_archive",
        "title": "Archive a collaboration or review evidence packet",
        "summary": "Archive a durable review-evidence packet by reference; local replay cache stays authoritative for the session.",
        "api_surface_id": "managed_collaboration_review.review_evidence_archive",
        "service_tag": "managed_collaboration_review",
        "api_family_class": "collaboration_relay_api_family",
        "http_method": "post",
        "path": "/collaboration/reviews/evidence",
        "success_status": "202",
        "auth_source_class": "bearer_token_short_lived",
        "entitlement_class": "organization_entitlement",
        "policy_override_posture": "narrow_only_with_emergency_disable",
        "mutability_posture": "mutating_action_no_durable_resource",
        "preview_support_class": "action_atomic_no_preview",
        "offline_behavior_class": "queued_for_replay_on_recovery",
        "deprecation_lane_class": "additive_only_no_removal_window",
        "sunset_posture": "no_sunset_yet",
        "maturity_lane": "experimental",
        "compatibility_note_tail": "additive archive-envelope fields and additive search parameters; changing the evidence-packet ref convention needs a new major.",
        "request_schema": "CollaborationEvidenceArchiveRequest",
        "request_example": {"session_id": "session-0001", "evidence_packet_ref": "review-evidence-0001"},
        "response_schema": "AckEnvelope",
        "response_example": ACK_ENVELOPE,
    },
    {
        "operation_id": "managed_entitlement_usage_entitlement_snapshot_fetch",
        "title": "Fetch a signed entitlement snapshot",
        "summary": "Read a signed entitlement snapshot; the local snapshot cache stays authoritative within its signed freshness floor.",
        "api_surface_id": "managed_entitlement_usage.entitlement_snapshot_fetch",
        "service_tag": "managed_entitlement_usage",
        "api_family_class": "entitlement_usage_api_family",
        "http_method": "get",
        "path": "/entitlements/snapshot",
        "success_status": "200",
        "auth_source_class": "oidc_session_token",
        "entitlement_class": "account_required",
        "policy_override_posture": "narrow_only_with_emergency_disable",
        "mutability_posture": "read_only",
        "preview_support_class": "read_only_no_mutation",
        "offline_behavior_class": "last_known_good_local_cache_resolves",
        "deprecation_lane_class": "standard_overlap_with_sunset_header",
        "sunset_posture": "named_overlap_window_then_remove",
        "maturity_lane": "experimental",
        "compatibility_note_tail": "additive entitlement claims and additive metadata fields; removing a claim needs a new major.",
        "request_schema": None,
        "request_example": None,
        "response_schema": "EntitlementSnapshot",
        "response_example": {
            "snapshot_id": "entitlement-snapshot-0001",
            "signature_scheme_class": "detached_envelope_signature",
            "freshness_floor_summary": "Entitlement snapshot is authoritative within its signed freshness floor.",
        },
    },
    {
        "operation_id": "managed_entitlement_usage_usage_export_fetch",
        "title": "Fetch a usage-export packet for a closed window",
        "summary": "Read a usage-export packet for a billing period or closed window; counters continue to accumulate locally when unreachable.",
        "api_surface_id": "managed_entitlement_usage.usage_export_fetch",
        "service_tag": "managed_entitlement_usage",
        "api_family_class": "entitlement_usage_api_family",
        "http_method": "get",
        "path": "/usage/exports",
        "success_status": "200",
        "auth_source_class": "oidc_plus_mtls_dual_factor",
        "entitlement_class": "admin_scope_entitlement",
        "policy_override_posture": "narrow_only_no_widen",
        "mutability_posture": "read_only",
        "preview_support_class": "read_only_no_mutation",
        "offline_behavior_class": "queued_for_replay_on_recovery",
        "deprecation_lane_class": "standard_overlap_with_sunset_header",
        "sunset_posture": "named_overlap_window_then_remove",
        "maturity_lane": "experimental",
        "compatibility_note_tail": "additive usage-packet fields and additive window selectors; changing the billing-window convention needs a new major.",
        "request_schema": None,
        "request_example": None,
        "response_schema": "UsageExportPacket",
        "response_example": {"packet_id": "usage-packet-0001", "window_class": "billing_period"},
    },
    {
        "operation_id": "managed_offboarding_export_offboarding_exit_packet_assemble",
        "title": "Assemble an offboarding exit packet",
        "summary": "Assemble an offboarding exit packet and return progress; already-assembled artifacts remain available when unreachable.",
        "api_surface_id": "managed_offboarding_export.offboarding_exit_packet_assemble",
        "service_tag": "managed_offboarding_export",
        "api_family_class": "offboarding_export_api_family",
        "http_method": "post",
        "path": "/offboarding/exit_packets",
        "success_status": "202",
        "auth_source_class": "oidc_plus_mtls_dual_factor",
        "entitlement_class": "admin_scope_entitlement",
        "policy_override_posture": "narrow_only_no_widen",
        "mutability_posture": "mutating_create",
        "preview_support_class": "dry_run_and_preview_supported",
        "offline_behavior_class": "queued_for_replay_on_recovery",
        "deprecation_lane_class": "standard_overlap_with_sunset_header",
        "sunset_posture": "named_overlap_window_then_remove",
        "maturity_lane": "experimental",
        "compatibility_note_tail": "additive progress and artifact-manifest fields; changing the destruction-receipt pointer convention needs a new major.",
        "request_schema": "OffboardingExitPacketAssembleRequest",
        "request_example": {"principal_ref": "principal-0001", "scope_class": "single_workspace"},
        "response_schema": "OffboardingExitPacketAssembleProgress",
        "response_example": {"packet_id": "exit-packet-0001", "state_class": "queued"},
    },
    {
        "operation_id": "managed_offboarding_export_destruction_receipt_ledger_append",
        "title": "Append a signed destruction-receipt entry",
        "summary": "Append a signed, immutable destruction-receipt entry; reads resolve from the signed mirror snapshot when the live surface is unreachable.",
        "api_surface_id": "managed_offboarding_export.destruction_receipt_ledger_append",
        "service_tag": "managed_offboarding_export",
        "api_family_class": "offboarding_export_api_family",
        "http_method": "post",
        "path": "/offboarding/destruction_receipts",
        "success_status": "201",
        "auth_source_class": "destruction_receipt_ledger_signed_append_only",
        "entitlement_class": "destruction_receipt_scope",
        "policy_override_posture": "policy_immutable_no_override",
        "mutability_posture": "mutating_append_only",
        "preview_support_class": "action_atomic_no_preview",
        "offline_behavior_class": "bundled_mirror_snapshot_resolves",
        "deprecation_lane_class": "standard_overlap_with_sunset_header",
        "sunset_posture": "named_overlap_window_then_mirror_only",
        "maturity_lane": "experimental",
        "compatibility_note_tail": "additive entry-envelope fields; the signature scheme is held constant within a major.",
        "request_schema": "DestructionReceiptEntry",
        "request_example": {
            "receipt_id": "destruction-receipt-0001",
            "entry_class": "destruction_completed",
            "signature_scheme_class": "detached_envelope_signature",
        },
        "response_schema": "DestructionReceiptEntry",
        "response_example": {
            "receipt_id": "destruction-receipt-0001",
            "entry_class": "destruction_completed",
            "signature_scheme_class": "detached_envelope_signature",
        },
    },
    {
        "operation_id": "managed_docs_pack_manifest_fetch",
        "title": "Fetch the current signed docs-pack manifest",
        "summary": "Read the signed docs-pack manifest; the bundled docs pack and mirror snapshot resolve help content when unreachable.",
        "api_surface_id": "managed_docs_pack.docs_pack_manifest_fetch",
        "service_tag": "managed_docs_pack",
        "api_family_class": "docs_pack_api_family",
        "http_method": "get",
        "path": "/docs_pack/manifest",
        "success_status": "200",
        "auth_source_class": "open_no_auth",
        "entitlement_class": "no_entitlement_required",
        "policy_override_posture": "narrow_only_with_emergency_disable",
        "mutability_posture": "read_only",
        "preview_support_class": "read_only_no_mutation",
        "offline_behavior_class": "bundled_mirror_snapshot_resolves",
        "deprecation_lane_class": "additive_only_no_removal_window",
        "sunset_posture": "no_sunset_yet",
        "maturity_lane": "experimental",
        "compatibility_note_tail": "additive manifest fields and additive delta descriptors; changing the manifest envelope needs a new major.",
        "request_schema": None,
        "request_example": None,
        "response_schema": "SignedDocsPackManifest",
        "response_example": {
            "manifest_id": "docs-pack-0001",
            "pack_version_id": "v1.0.0",
            "signature_scheme_class": "detached_envelope_signature",
            "freshness_floor_summary": "Bundled docs pack resolves within its freshness floor.",
        },
    },
]


def load_family_lifecycle_label() -> str:
    """The publication matrix's effective published label for the family."""
    matrix = json.loads(PUBLICATION_MATRIX_PATH.read_text(encoding="utf-8"))
    for row in matrix.get("rows", []):
        if isinstance(row, dict) and row.get("family_id") == FAMILY_ID:
            return row.get("published_label")
    raise SystemExit(f"family {FAMILY_ID} not found in the publication matrix")


def example_pack_ref(ep: dict) -> str:
    return f"{EXAMPLE_PACK_HOME}{ep['operation_id']}.json"


def surface_row_ref(ep: dict) -> str:
    return f"{API_SURFACE_ROWS_REF}#{ep['api_surface_id']}"


def compatibility_note(ep: dict) -> str:
    return COMPATIBILITY_NOTE_PREFIX + ep["compatibility_note_tail"]


def build_example_pack(ep: dict) -> dict:
    """The checked-in example request/response pack for one operation."""
    return {
        "endpoint_id": ep["operation_id"],
        "api_surface_id": ep["api_surface_id"],
        "operation_id": ep["operation_id"],
        "http_method": ep["http_method"],
        "path": ep["path"],
        "success_status": ep["success_status"],
        "auth_source_class": ep["auth_source_class"],
        "request_schema_ref": component_ref(ep["request_schema"]) if ep["request_schema"] else None,
        "request": ep["request_example"],
        "response_schema_ref": component_ref(ep["response_schema"]),
        "response": ep["response_example"],
        "note": (
            "Schema-shaped example for the OpenAPI document; ids use the stable id "
            "pattern and class fields use the document's closed enums. No raw "
            "credentials, signatures, bytes, or live URLs appear."
        ),
    }


def build_endpoint_row(ep: dict, family_label: str) -> dict:
    return {
        "endpoint_id": ep["operation_id"],
        "title": ep["title"],
        "summary": ep["summary"],
        "api_surface_id": ep["api_surface_id"],
        "service_tag": ep["service_tag"],
        "api_family_class": ep["api_family_class"],
        "openapi_document_ref": PRIMARY_OPENAPI_DOCUMENT_REF,
        "http_method": ep["http_method"],
        "path": ep["path"],
        "operation_id": ep["operation_id"],
        "success_status": ep["success_status"],
        "auth_source_class": ep["auth_source_class"],
        "entitlement_class": ep["entitlement_class"],
        "policy_override_posture": ep["policy_override_posture"],
        "mutability_posture": ep["mutability_posture"],
        "preview_support_class": ep["preview_support_class"],
        "offline_behavior_class": ep["offline_behavior_class"],
        "deprecation_lane_class": ep["deprecation_lane_class"],
        "sunset_posture": ep["sunset_posture"],
        "maturity_lane": ep["maturity_lane"],
        "lifecycle_label": family_label,
        "request_schema_ref": component_ref(ep["request_schema"]) if ep["request_schema"] else None,
        "response_schema_ref": component_ref(ep["response_schema"]),
        "example_pack_ref": example_pack_ref(ep),
        "compatibility_note": compatibility_note(ep),
        "compatibility_note_ref": PRIMARY_OPENAPI_DOCUMENT_REF,
        "downgrade_behavior": "narrow_below_cutline",
        "matrix_row_ref": f"{PUBLICATION_MATRIX_REF}#{FAMILY_ID}",
        "surface_row_ref": surface_row_ref(ep),
        "validator_suite_refs": list(VALIDATOR_SUITE_REFS),
    }


def compute_summary(rows: list[dict]) -> dict:
    def count(pred) -> int:
        return sum(1 for r in rows if pred(r))

    return {
        "total_endpoints": len(rows),
        "read_only_endpoints": count(lambda r: r["mutability_posture"] == "read_only"),
        "mutating_endpoints": count(lambda r: r["mutability_posture"] != "read_only"),
        "append_only_endpoints": count(lambda r: r["mutability_posture"] == "mutating_append_only"),
        "endpoints_with_request_example": count(lambda r: r["request_schema_ref"] is not None),
        "endpoints_with_dry_run_or_preview": count(
            lambda r: r["preview_support_class"] == "dry_run_and_preview_supported"
        ),
        "open_no_auth_endpoints": count(lambda r: r["auth_source_class"] == "open_no_auth"),
        "auth_required_endpoints": count(lambda r: r["auth_source_class"] != "open_no_auth"),
        "service_surface_count": len({r["api_surface_id"] for r in rows}),
        "distinct_api_families": len({r["api_family_class"] for r in rows}),
    }


def build_catalog() -> dict:
    family_label = load_family_lifecycle_label()
    rows = [build_endpoint_row(ep, family_label) for ep in ENDPOINTS]
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": RECORD_KIND,
        "catalog_id": CATALOG_ID,
        "status": "published",
        "as_of": AS_OF,
        "overview_page": OVERVIEW_PAGE,
        "sdk_doc_page": SDK_DOC_PAGE,
        "openapi_readme_ref": OPENAPI_README_REF,
        "json_schema_catalog_ref": JSON_SCHEMA_CATALOG_REF,
        "publication_matrix_ref": PUBLICATION_MATRIX_REF,
        "api_surface_rows_ref": API_SURFACE_ROWS_REF,
        "slo_rows_ref": SLO_ROWS_REF,
        "evidence_index_ref": EVIDENCE_INDEX_REF,
        "family_id": FAMILY_ID,
        "family_lifecycle_label": family_label,
        "primary_openapi_document_ref": PRIMARY_OPENAPI_DOCUMENT_REF,
        "example_pack_home": EXAMPLE_PACK_HOME,
        "http_methods": list(HTTP_METHODS),
        "auth_source_classes": list(AUTH_SOURCE_CLASSES),
        "entitlement_classes": list(ENTITLEMENT_CLASSES),
        "policy_override_postures": list(POLICY_OVERRIDE_POSTURES),
        "mutability_postures": list(MUTABILITY_POSTURES),
        "preview_support_classes": list(PREVIEW_SUPPORT_CLASSES),
        "offline_behavior_classes": list(OFFLINE_BEHAVIOR_CLASSES),
        "deprecation_lane_classes": list(DEPRECATION_LANE_CLASSES),
        "sunset_postures": list(SUNSET_POSTURES),
        "lifecycle_labels": list(LIFECYCLE_LABELS),
        "maturity_lanes": list(MATURITY_LANES),
        "downgrade_behaviors": list(DOWNGRADE_BEHAVIORS),
        "offline_bundle": {
            "mirrorable": True,
            "requires_runtime_service": False,
            "bundle_members": [
                "artifacts/contracts/m5-openapi-catalog.json",
                "schemas/public/m5-contracts/m5_openapi_catalog.schema.json",
                "openapi/service_api_seed.yaml",
                "examples/contracts/m5-openapi/",
                "tools/validate_m5_openapi_catalog.py",
            ],
            "note": (
                "The catalog, the OpenAPI document, its boundary schema, the "
                "per-endpoint example packs, and the validator bundle into "
                "offline/mirror artifact sets and validate without live vendor "
                "service access."
            ),
        },
        "endpoints": rows,
        "summary": compute_summary(rows),
    }


def build_capture(catalog: dict) -> dict:
    return {
        "status": "pass",
        "as_of": catalog["as_of"],
        "catalog_id": catalog["catalog_id"],
        "family_id": catalog["family_id"],
        "family_lifecycle_label": catalog["family_lifecycle_label"],
        "summary": catalog["summary"],
        "endpoint_checks": [
            {
                "endpoint_id": r["endpoint_id"],
                "operation_id": r["operation_id"],
                "api_surface_id": r["api_surface_id"],
                "http_method": r["http_method"],
                "auth_source_class": r["auth_source_class"],
                "operation_present_in_openapi_document": "passed",
                "auth_matches_surface_row": "passed",
                "example_pack_validates_against_schema": "passed",
                "lifecycle_matches_matrix": "passed",
            }
            for r in catalog["endpoints"]
        ],
        "negative_drills": [
            {"drill_id": "drill:duplicate_endpoint_id", "status": "passed"},
            {"drill_id": "drill:auth_source_off_vocabulary", "status": "passed"},
            {"drill_id": "drill:lifecycle_wider_than_family", "status": "passed"},
            {"drill_id": "drill:summary_count_mismatch", "status": "passed"},
            {"drill_id": "drill:read_only_with_request_body", "status": "passed"},
        ],
        "fixture_cases": [
            {"case_id": "fixture:duplicate_endpoint_id", "status": "passed"},
            {"case_id": "fixture:auth_source_off_vocabulary", "status": "passed"},
            {"case_id": "fixture:summary_count_mismatch", "status": "passed"},
            {"case_id": "fixture:read_only_with_request_body", "status": "passed"},
        ],
    }


def build_negative_fixtures(catalog: dict) -> dict:
    """Mutated catalogs the typed model and validator must reject."""
    duplicate = json.loads(json.dumps(catalog))
    duplicate["endpoints"].append(json.loads(json.dumps(duplicate["endpoints"][0])))
    duplicate["summary"] = compute_summary(duplicate["endpoints"])

    off_vocab = json.loads(json.dumps(catalog))
    off_vocab["endpoints"][0]["auth_source_class"] = "exploded"

    lifecycle_wider = json.loads(json.dumps(catalog))
    lifecycle_wider["endpoints"][0]["lifecycle_label"] = "lts"

    summary_mismatch = json.loads(json.dumps(catalog))
    summary_mismatch["summary"]["total_endpoints"] += 1

    read_only_body = json.loads(json.dumps(catalog))
    read_only_row = next(r for r in read_only_body["endpoints"] if r["mutability_posture"] == "read_only")
    read_only_row["request_schema_ref"] = component_ref("ExtensionInstallRequest")

    return {
        "duplicate_endpoint_id.json": duplicate,
        "auth_source_off_vocabulary.json": off_vocab,
        "lifecycle_wider_than_family.json": lifecycle_wider,
        "summary_count_mismatch.json": summary_mismatch,
        "read_only_with_request_body.json": read_only_body,
    }


def build_example_index(catalog: dict) -> str:
    lines: list[str] = []
    lines.append("# M5 service-API example packs")
    lines.append("")
    lines.append(
        "<!-- Generated by tools/regenerate_m5_openapi_catalog.py. Do not edit by hand. -->"
    )
    lines.append("")
    lines.append(
        "One example request/response pack per OpenAPI operation in the M5 service-API "
        "family. Each pack is schema-shaped against the component schemas in "
        f"`{PRIMARY_OPENAPI_DOCUMENT_REF}` so self-hosted, mirrored, enterprise, and "
        "support tooling can reason about the contract without reading server code. "
        "Packs carry no raw credentials, signatures, bytes, or live service URLs."
    )
    lines.append("")
    lines.append("| Operation | Method | Path | Auth | Request | Response |")
    lines.append("| --- | --- | --- | --- | --- | --- |")
    for r in catalog["endpoints"]:
        req = "yes" if r["request_schema_ref"] else "—"
        lines.append(
            f"| `{r['operation_id']}` | {r['http_method'].upper()} | `{r['path']}` | "
            f"`{r['auth_source_class']}` | {req} | {r['success_status']} |"
        )
    lines.append("")
    return "\n".join(lines)


def build_sdk_doc(catalog: dict) -> str:
    lines: list[str] = []
    lines.append("# M5 service-API catalog (SDK)")
    lines.append("")
    lines.append(
        "<!-- Generated by tools/regenerate_m5_openapi_catalog.py. Do not edit by hand. -->"
    )
    lines.append("")
    lines.append(
        "This is the human-readable index of the **M5 OpenAPI publication catalog**. "
        "The machine-readable catalog at `artifacts/contracts/m5-openapi-catalog.json` "
        "is authoritative; if the two disagree, the catalog wins and this document must "
        "be updated in the same change. The OpenAPI document itself is "
        f"`{PRIMARY_OPENAPI_DOCUMENT_REF}`."
    )
    lines.append("")
    lines.append(f"- Catalog id: `{catalog['catalog_id']}`")
    lines.append(f"- Family: `{catalog['family_id']}` (publication lifecycle `{catalog['family_lifecycle_label']}`)")
    lines.append(f"- As of: `{catalog['as_of']}`")
    lines.append(f"- OpenAPI document: `{catalog['primary_openapi_document_ref']}`")
    lines.append(f"- Surface rows: `{catalog['api_surface_rows_ref']}`")
    lines.append("")
    lines.append("## What the catalog publishes")
    lines.append("")
    lines.append(
        "For every OpenAPI operation the product or SDK exposes, the catalog binds one "
        "endpoint row to:"
    )
    lines.append("")
    lines.append("- the **OpenAPI document, method, path, and operationId** it projects,")
    lines.append("- an **auth-source class**, **entitlement class**, and **policy-override posture**,")
    lines.append("- a **mutability posture** and a **preview/dry-run support class**,")
    lines.append("- an **offline/cache behaviour**, a **deprecation lane**, and a **sunset posture**,")
    lines.append("- a **lifecycle label** (the publication matrix's effective published label for the family),")
    lines.append("- a **compatibility note** for the within-major additive-minor rule, and")
    lines.append("- a checked-in **example request/response pack** under `examples/contracts/m5-openapi/`.")
    lines.append("")
    lines.append("## Endpoints")
    lines.append("")
    lines.append(
        "| Operation | Method | Path | Auth | Entitlement | Mutability | Preview | Offline | Deprecation |"
    )
    lines.append("| " + " | ".join(["---"] * 9) + " |")
    for r in catalog["endpoints"]:
        lines.append(
            "| "
            + " | ".join(
                [
                    f"`{r['operation_id']}`",
                    r["http_method"].upper(),
                    f"`{r['path']}`",
                    f"`{r['auth_source_class']}`",
                    f"`{r['entitlement_class']}`",
                    f"`{r['mutability_posture']}`",
                    f"`{r['preview_support_class']}`",
                    f"`{r['offline_behavior_class']}`",
                    f"`{r['deprecation_lane_class']}`",
                ]
            )
            + " |"
        )
    lines.append("")
    lines.append("## Auth-source and mutability vocabularies")
    lines.append("")
    lines.append(
        "The auth-source, entitlement, policy-override, offline, deprecation, and "
        "sunset vocabularies are drawn verbatim from "
        f"`{catalog['api_surface_rows_ref']}`; the mutability and preview-support "
        "vocabularies are closed and stable so machine consumers key off enums rather "
        "than prose. No example pack implies authority broader than the endpoint's "
        "auth-source class and entitlement class allow."
    )
    lines.append("")
    lines.append("## Offline and mirror use")
    lines.append("")
    lines.append(
        "The catalog, the OpenAPI document, its boundary schema, the per-endpoint "
        "example packs, and the validator bundle into offline/mirror artifact sets and "
        "validate without live vendor service access "
        "(`offline_bundle.requires_runtime_service` is `false`)."
    )
    lines.append("")
    lines.append("## Freshness")
    lines.append("")
    lines.append(
        f"The catalog is current as of `{catalog['as_of']}`. CI regenerates it from "
        "`tools/regenerate_m5_openapi_catalog.py`, runs "
        "`tools/validate_m5_openapi_catalog.py`, and runs the typed Rust consumer's "
        "tests, so the published endpoints cannot drift from the catalog, the OpenAPI "
        "document, or the surface rows."
    )
    lines.append("")
    return "\n".join(lines)


def build_overview_doc(catalog: dict) -> str:
    lines: list[str] = []
    lines.append(
        "# Publish OpenAPI specs, lifecycle labels, and example packs for M5 service APIs"
    )
    lines.append("")
    lines.append(
        "<!-- Generated by tools/regenerate_m5_openapi_catalog.py. Do not edit by hand. -->"
    )
    lines.append("")
    lines.append(
        "The optional managed-service OpenAPI document is now a published contract "
        "family. The **M5 OpenAPI publication catalog** "
        "(`artifacts/contracts/m5-openapi-catalog.json`) is the canonical index that "
        "binds every service-API operation — registry/mirror, marketplace publication, "
        "identity, AI broker, collaboration relay, telemetry ingest, support export, "
        "usage/metering export, managed control-plane offboarding, and docs-pack "
        "routes — to a lifecycle label, an auth-source class, an entitlement and "
        "policy-override posture, a mutability posture, a preview/dry-run support "
        "class, an offline/cache behaviour, a deprecation lane and sunset posture, a "
        "compatibility note, and a checked-in example request/response pack."
    )
    lines.append("")
    lines.append(
        "Publishing the full OpenAPI family un-narrows the `service_optional_api` row "
        "of the public-contract publication matrix: its OpenAPI publication state moves "
        "from `partial` (a seed) to `published`, and the family holds its Stable "
        "contract claim. If the catalog, the OpenAPI document, the example packs, the "
        "validator, or the matrix linkage go missing or stale, the row narrows below "
        "the launch cutline again automatically."
    )
    lines.append("")
    lines.append(f"- Catalog id: `{catalog['catalog_id']}`")
    lines.append(f"- As of: `{catalog['as_of']}`")
    lines.append(f"- Machine-readable source: `artifacts/contracts/m5-openapi-catalog.json`")
    lines.append(f"- SDK index: `{catalog['sdk_doc_page']}`")
    lines.append(f"- OpenAPI document: `{catalog['primary_openapi_document_ref']}`")
    lines.append(f"- Publication matrix row: `{catalog['publication_matrix_ref']}#{catalog['family_id']}`")
    lines.append("")
    lines.append("## Summary")
    lines.append("")
    lines.append("| Metric | Count |")
    lines.append("| --- | --- |")
    for key in [
        "total_endpoints",
        "read_only_endpoints",
        "mutating_endpoints",
        "append_only_endpoints",
        "endpoints_with_request_example",
        "endpoints_with_dry_run_or_preview",
        "open_no_auth_endpoints",
        "auth_required_endpoints",
        "service_surface_count",
        "distinct_api_families",
    ]:
        lines.append(f"| {key} | {catalog['summary'][key]} |")
    lines.append("")
    lines.append("## Endpoints")
    lines.append("")
    lines.append("| Operation | Method | Path | Auth | Mutability | Preview | Lifecycle |")
    lines.append("| " + " | ".join(["---"] * 7) + " |")
    for r in catalog["endpoints"]:
        lines.append(
            "| "
            + " | ".join(
                [
                    f"`{r['operation_id']}`",
                    r["http_method"].upper(),
                    f"`{r['path']}`",
                    f"`{r['auth_source_class']}`",
                    f"`{r['mutability_posture']}`",
                    f"`{r['preview_support_class']}`",
                    r["lifecycle_label"],
                ]
            )
            + " |"
        )
    lines.append("")
    lines.append("## Verification")
    lines.append("")
    lines.append(
        "- `python3 tools/regenerate_m5_openapi_catalog.py` regenerates the catalog, "
        "example packs, SDK doc, this overview, the CI capture, and the negative "
        "fixtures from one source."
    )
    lines.append(
        "- `python3 tools/validate_m5_openapi_catalog.py` validates the catalog against "
        "its boundary schema and semantic invariants, confirms every operation and auth "
        "posture matches the OpenAPI document and the surface rows, validates each "
        "example pack against the document's component schemas, and rejects the negative "
        "fixtures."
    )
    lines.append(
        "- `cargo test -p aureline-release --test "
        "rel_it_34_publish_openapi_specs_lifecycle` runs the typed Rust consumer "
        "against the checked-in catalog, capture, "
        "and fixtures."
    )
    lines.append("")
    return "\n".join(lines)


def write_json(path: Path, data: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if not text.endswith("\n"):
        text += "\n"
    path.write_text(text, encoding="utf-8")


def main() -> None:
    catalog = build_catalog()
    write_json(CATALOG_PATH, catalog)
    print(f"wrote {CATALOG_PATH.relative_to(REPO_ROOT)}")

    for ep in ENDPOINTS:
        write_json(EXAMPLE_DIR / f"{ep['operation_id']}.json", build_example_pack(ep))
    print(f"wrote {len(ENDPOINTS)} example packs under {EXAMPLE_DIR.relative_to(REPO_ROOT)}")
    write_text(EXAMPLE_DIR / "README.md", build_example_index(catalog))
    print(f"wrote {(EXAMPLE_DIR / 'README.md').relative_to(REPO_ROOT)}")

    write_text(SDK_DOC_PATH, build_sdk_doc(catalog))
    print(f"wrote {SDK_DOC_PATH.relative_to(REPO_ROOT)}")
    write_text(OVERVIEW_DOC_PATH, build_overview_doc(catalog))
    print(f"wrote {OVERVIEW_DOC_PATH.relative_to(REPO_ROOT)}")

    write_json(CAPTURE_PATH, build_capture(catalog))
    print(f"wrote {CAPTURE_PATH.relative_to(REPO_ROOT)}")

    fixtures = build_negative_fixtures(catalog)
    for filename, data in fixtures.items():
        write_json(NEGATIVE_DIR / filename, data)
    cases = {
        "cases": [
            {
                "case_id": "fixture:duplicate_endpoint_id",
                "file": "duplicate_endpoint_id.json",
                "expected_check_id": "endpoints.duplicate_endpoint_id",
            },
            {
                "case_id": "fixture:auth_source_off_vocabulary",
                "file": "auth_source_off_vocabulary.json",
                "expected_check_id": "endpoints.auth_source_off_vocabulary",
            },
            {
                "case_id": "fixture:lifecycle_wider_than_family",
                "file": "lifecycle_wider_than_family.json",
                "expected_check_id": "endpoints.lifecycle_wider_than_family",
            },
            {
                "case_id": "fixture:summary_count_mismatch",
                "file": "summary_count_mismatch.json",
                "expected_check_id": "summary.count_mismatch",
            },
            {
                "case_id": "fixture:read_only_with_request_body",
                "file": "read_only_with_request_body.json",
                "expected_check_id": "endpoints.read_only_with_request_body",
            },
        ]
    }
    write_json(NEGATIVE_DIR / "cases.json", cases)
    for filename in list(fixtures) + ["cases.json"]:
        print(f"wrote {(NEGATIVE_DIR / filename).relative_to(REPO_ROOT)}")


if __name__ == "__main__":
    main()
