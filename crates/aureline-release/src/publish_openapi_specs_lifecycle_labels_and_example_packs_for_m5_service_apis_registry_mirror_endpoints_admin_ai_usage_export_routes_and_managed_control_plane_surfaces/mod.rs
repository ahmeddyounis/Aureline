//! Typed M5 OpenAPI publication catalog: the canonical index that turns the
//! optional managed-service OpenAPI document into a published contract family.
//!
//! Where the public-contract publication matrix records that the
//! `service_optional_api` family must publish a full OpenAPI family before it can
//! hold a Stable contract claim, this catalog *is* that publication. Each
//! [`OpenapiEndpoint`] binds one OpenAPI operation — registry/mirror, marketplace
//! publication, identity, AI broker, collaboration relay, telemetry ingest,
//! support export, usage/metering export, managed control-plane offboarding, and
//! docs-pack routes — to:
//!
//! - its OpenAPI document, [`HttpMethod`], path, and operation id,
//! - an [`AuthSourceClass`], [`EntitlementClass`], and [`PolicyOverridePosture`]
//!   drawn verbatim from the optional-service API surface rows,
//! - a [`MutabilityPosture`] and a [`PreviewSupportClass`] declaring whether the
//!   operation mutates state and whether a preview/dry-run is supported,
//! - an [`OfflineBehaviorClass`], [`DeprecationLaneClass`], and [`SunsetPosture`],
//! - a [`LifecycleLabel`] equal to the publication matrix's effective published
//!   label for the family and a [`MaturityLane`] equal to the surface row's lane,
//! - a compatibility note for the within-major additive-minor rule, and
//! - a checked-in example request/response pack under `examples/contracts/m5-openapi/`.
//!
//! Downstream surfaces (SDK docs, help, support export, mirror packaging) resolve
//! an operation's auth posture, mutability, lifecycle label, and example pack from
//! this catalog instead of restating field semantics or reading server code. A
//! catalog whose endpoints drift from the OpenAPI document or the surface rows
//! narrows the family below the launch cutline
//! ([`DowngradeBehavior::NarrowBelowCutline`]).
//!
//! The catalog is checked in at `artifacts/contracts/m5-openapi-catalog.json` and
//! embedded here, so this typed consumer and the CI validator agree on every
//! endpoint without a cargo build in CI. The model is metadata-only: every field
//! is a typed state or an opaque repo-relative ref. It carries no raw
//! request/response bytes, credential material, signatures, or live service URLs.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Supported catalog schema version.
pub const M5_OPENAPI_CATALOG_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the catalog.
pub const M5_OPENAPI_CATALOG_RECORD_KIND: &str = "m5_openapi_catalog";

/// Stable catalog identifier.
pub const M5_OPENAPI_CATALOG_ID: &str = "m5_openapi_catalog:v1";

/// The contract family this catalog publishes.
pub const M5_OPENAPI_CATALOG_FAMILY_ID: &str = "service_optional_api";

/// Repo-relative path to the checked-in catalog.
pub const M5_OPENAPI_CATALOG_PATH: &str = "artifacts/contracts/m5-openapi-catalog.json";

/// Embedded checked-in catalog JSON.
pub const M5_OPENAPI_CATALOG_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/contracts/m5-openapi-catalog.json"
));

/// An HTTP method a published operation uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HttpMethod {
    /// A read.
    Get,
    /// A replace.
    Put,
    /// A create or action.
    Post,
}

impl HttpMethod {
    /// Every method, in declaration order.
    pub const ALL: [Self; 3] = [Self::Get, Self::Put, Self::Post];
}

/// The auth-source class a published operation requires.
///
/// Drawn verbatim from `auth_mode` in `artifacts/service/api_surface_rows.yaml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthSourceClass {
    /// No authentication.
    OpenNoAuth,
    /// A short-lived bearer token.
    BearerTokenShortLived,
    /// An OIDC session token.
    OidcSessionToken,
    /// An mTLS client certificate.
    MtlsClientCert,
    /// OIDC plus mTLS dual factor.
    OidcPlusMtlsDualFactor,
    /// A SCIM bearer token.
    ScimBearerToken,
    /// A customer BYOK pass-through with no broker auth.
    CustomerByokPassthroughNoBrokerAuth,
    /// A signed mirror snapshot with no live auth.
    SignedMirrorSnapshotOnlyNoLiveAuth,
    /// A signed append-only destruction-receipt ledger token.
    DestructionReceiptLedgerSignedAppendOnly,
}

impl AuthSourceClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::OpenNoAuth,
        Self::BearerTokenShortLived,
        Self::OidcSessionToken,
        Self::MtlsClientCert,
        Self::OidcPlusMtlsDualFactor,
        Self::ScimBearerToken,
        Self::CustomerByokPassthroughNoBrokerAuth,
        Self::SignedMirrorSnapshotOnlyNoLiveAuth,
        Self::DestructionReceiptLedgerSignedAppendOnly,
    ];

    /// True for the open, no-auth class.
    pub fn is_open(self) -> bool {
        matches!(self, Self::OpenNoAuth)
    }
}

/// The entitlement class a published operation requires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntitlementClass {
    /// No entitlement required.
    NoEntitlementRequired,
    /// An account is required.
    AccountRequired,
    /// An organization entitlement is required.
    OrganizationEntitlement,
    /// A support-case entitlement is required.
    SupportCaseEntitlement,
    /// An admin-scope entitlement is required.
    AdminScopeEntitlement,
    /// A destruction-receipt scope is required.
    DestructionReceiptScope,
}

impl EntitlementClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoEntitlementRequired,
        Self::AccountRequired,
        Self::OrganizationEntitlement,
        Self::SupportCaseEntitlement,
        Self::AdminScopeEntitlement,
        Self::DestructionReceiptScope,
    ];
}

/// How admin policy may override a published operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyOverridePosture {
    /// Policy may only narrow, never widen.
    NarrowOnlyNoWiden,
    /// Policy may only narrow, with an emergency-disable lane.
    NarrowOnlyWithEmergencyDisable,
    /// Policy is immutable; no override.
    PolicyImmutableNoOverride,
}

impl PolicyOverridePosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::NarrowOnlyNoWiden,
        Self::NarrowOnlyWithEmergencyDisable,
        Self::PolicyImmutableNoOverride,
    ];
}

/// Whether and how a published operation mutates server state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutabilityPosture {
    /// A read with no server-side state change.
    ReadOnly,
    /// Creates or requests a new resource or job.
    MutatingCreate,
    /// Replaces an existing resource.
    MutatingReplace,
    /// Appends an immutable entry to a ledger.
    MutatingAppendOnly,
    /// An action that does not create a durable addressable resource.
    MutatingActionNoDurableResource,
}

impl MutabilityPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReadOnly,
        Self::MutatingCreate,
        Self::MutatingReplace,
        Self::MutatingAppendOnly,
        Self::MutatingActionNoDurableResource,
    ];

    /// True for the read-only posture.
    pub fn is_read_only(self) -> bool {
        matches!(self, Self::ReadOnly)
    }
}

/// Whether a published operation supports a preview / dry-run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewSupportClass {
    /// A read; there is nothing to preview.
    ReadOnlyNoMutation,
    /// A dry-run / preview is supported before applying.
    DryRunAndPreviewSupported,
    /// An atomic action with no preview.
    ActionAtomicNoPreview,
}

impl PreviewSupportClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ReadOnlyNoMutation,
        Self::DryRunAndPreviewSupported,
        Self::ActionAtomicNoPreview,
    ];
}

/// How a published operation behaves when the service is unreachable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfflineBehaviorClass {
    /// No network is required; local only.
    NoNetworkRequiredLocalOnly,
    /// A last-known-good local cache resolves.
    LastKnownGoodLocalCacheResolves,
    /// A bundled mirror snapshot resolves.
    BundledMirrorSnapshotResolves,
    /// The request queues for replay on recovery.
    QueuedForReplayOnRecovery,
    /// Read-only when reachable; narrows on unreachable.
    ReadOnlyWhenReachableAndNarrowsOnUnreachable,
}

impl OfflineBehaviorClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoNetworkRequiredLocalOnly,
        Self::LastKnownGoodLocalCacheResolves,
        Self::BundledMirrorSnapshotResolves,
        Self::QueuedForReplayOnRecovery,
        Self::ReadOnlyWhenReachableAndNarrowsOnUnreachable,
    ];
}

/// The deprecation lane a published operation is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeprecationLaneClass {
    /// Pre-release; no deprecation yet.
    PreReleaseNoDeprecationYet,
    /// Additive only; no removal window.
    AdditiveOnlyNoRemovalWindow,
    /// Standard overlap with a Sunset header.
    StandardOverlapWithSunsetHeader,
    /// Emergency sunset with an explicit advisory.
    EmergencySunsetWithExplicitAdvisory,
}

impl DeprecationLaneClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PreReleaseNoDeprecationYet,
        Self::AdditiveOnlyNoRemovalWindow,
        Self::StandardOverlapWithSunsetHeader,
        Self::EmergencySunsetWithExplicitAdvisory,
    ];
}

/// The sunset posture a published operation declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SunsetPosture {
    /// No sunset yet.
    NoSunsetYet,
    /// A named overlap window then removal.
    NamedOverlapWindowThenRemove,
    /// A named overlap window then mirror-only.
    NamedOverlapWindowThenMirrorOnly,
    /// Immediate sunset on advisory.
    ImmediateSunsetOnAdvisory,
}

impl SunsetPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NoSunsetYet,
        Self::NamedOverlapWindowThenRemove,
        Self::NamedOverlapWindowThenMirrorOnly,
        Self::ImmediateSunsetOnAdvisory,
    ];
}

/// The lifecycle/stability label the family publishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleLabel {
    /// Long-term-stable.
    Lts,
    /// Stable.
    Stable,
    /// Beta.
    Beta,
    /// Preview.
    Preview,
    /// Withdrawn.
    Withdrawn,
}

impl LifecycleLabel {
    /// Every label, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Lts,
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Withdrawn,
    ];

    /// True when this label is at or above the stable cutline.
    pub fn holds_stable(self) -> bool {
        matches!(self, Self::Lts | Self::Stable)
    }
}

/// The maturity lane the surface row records for an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaturityLane {
    /// Stable.
    Stable,
    /// Beta.
    Beta,
    /// Experimental.
    Experimental,
    /// Internal.
    Internal,
}

impl MaturityLane {
    /// Every lane, in declaration order.
    pub const ALL: [Self; 4] = [Self::Stable, Self::Beta, Self::Experimental, Self::Internal];
}

/// What happens to the family when required publication evidence is lost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeBehavior {
    /// The family narrows below the launch cutline.
    NarrowBelowCutline,
    /// The artifact is rejected.
    Reject,
}

impl DowngradeBehavior {
    /// Every behavior, in declaration order.
    pub const ALL: [Self; 2] = [Self::NarrowBelowCutline, Self::Reject];
}

/// One published OpenAPI operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenapiEndpoint {
    /// Stable endpoint identifier (the operation id).
    pub endpoint_id: String,
    /// Human-readable title.
    pub title: String,
    /// One-line summary.
    pub summary: String,
    /// The surface row this operation belongs to.
    pub api_surface_id: String,
    /// The OpenAPI tag / service id.
    pub service_tag: String,
    /// The api-family class.
    pub api_family_class: String,
    /// Repo-relative ref to the OpenAPI document.
    pub openapi_document_ref: String,
    /// The HTTP method.
    pub http_method: HttpMethod,
    /// The HTTP path.
    pub path: String,
    /// The OpenAPI operation id.
    pub operation_id: String,
    /// The success status code.
    pub success_status: String,
    /// The auth-source class.
    pub auth_source_class: AuthSourceClass,
    /// The entitlement class.
    pub entitlement_class: EntitlementClass,
    /// The policy-override posture.
    pub policy_override_posture: PolicyOverridePosture,
    /// The mutability posture.
    pub mutability_posture: MutabilityPosture,
    /// The preview/dry-run support class.
    pub preview_support_class: PreviewSupportClass,
    /// The offline/cache behaviour.
    pub offline_behavior_class: OfflineBehaviorClass,
    /// The deprecation lane.
    pub deprecation_lane_class: DeprecationLaneClass,
    /// The sunset posture.
    pub sunset_posture: SunsetPosture,
    /// The surface maturity lane.
    pub maturity_lane: MaturityLane,
    /// The family lifecycle label this operation publishes under.
    pub lifecycle_label: LifecycleLabel,
    /// Ref to the request body's component schema, if any.
    pub request_schema_ref: Option<String>,
    /// Ref to the response body's component schema.
    pub response_schema_ref: String,
    /// Ref to the checked-in example request/response pack.
    pub example_pack_ref: String,
    /// Human-readable compatibility note.
    pub compatibility_note: String,
    /// Ref to the doc that carries the compatibility note.
    pub compatibility_note_ref: String,
    /// What happens when required publication evidence is lost.
    pub downgrade_behavior: DowngradeBehavior,
    /// Ref to the publication-matrix row.
    pub matrix_row_ref: String,
    /// Ref to the surface row.
    pub surface_row_ref: String,
    /// Refs to the validators that gate this endpoint.
    pub validator_suite_refs: Vec<String>,
}

impl OpenapiEndpoint {
    /// True when this operation reads without changing server state.
    pub fn is_read_only(&self) -> bool {
        self.mutability_posture.is_read_only()
    }

    /// True when this operation carries a request body.
    pub fn has_request_body(&self) -> bool {
        self.request_schema_ref.is_some()
    }

    /// True when this operation supports a preview / dry-run.
    pub fn supports_dry_run(&self) -> bool {
        self.preview_support_class == PreviewSupportClass::DryRunAndPreviewSupported
    }
}

/// The offline/mirror bundling declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OfflineBundle {
    /// True when the family bundles into mirror artifact sets.
    pub mirrorable: bool,
    /// True when validation requires runtime service access.
    pub requires_runtime_service: bool,
    /// Bundle members (catalog, schema, OpenAPI document, examples, validator).
    pub bundle_members: Vec<String>,
    /// Human-readable note.
    pub note: String,
}

/// Summary counts over the endpoint set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5OpenapiCatalogSummary {
    /// Total endpoints.
    pub total_endpoints: usize,
    /// Read-only endpoints.
    pub read_only_endpoints: usize,
    /// Mutating endpoints.
    pub mutating_endpoints: usize,
    /// Append-only endpoints.
    pub append_only_endpoints: usize,
    /// Endpoints with a request example.
    pub endpoints_with_request_example: usize,
    /// Endpoints with a dry-run / preview.
    pub endpoints_with_dry_run_or_preview: usize,
    /// Open, no-auth endpoints.
    pub open_no_auth_endpoints: usize,
    /// Endpoints requiring authentication.
    pub auth_required_endpoints: usize,
    /// Distinct surface rows covered.
    pub service_surface_count: usize,
    /// Distinct api families covered.
    pub distinct_api_families: usize,
}

/// A structural validation violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5OpenapiCatalogViolation {
    /// Stable check id.
    pub check_id: String,
    /// Human-readable detail.
    pub detail: String,
}

/// The typed M5 OpenAPI publication catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5OpenapiCatalog {
    /// Catalog schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable catalog identifier.
    pub catalog_id: String,
    /// Lifecycle status of this catalog artifact.
    pub status: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// SDK reference doc.
    pub sdk_doc_page: String,
    /// Ref to the OpenAPI publication README.
    pub openapi_readme_ref: String,
    /// Ref to the JSON Schema catalog.
    pub json_schema_catalog_ref: String,
    /// Ref to the public-contract publication matrix.
    pub publication_matrix_ref: String,
    /// Ref to the optional-service API surface rows.
    pub api_surface_rows_ref: String,
    /// Ref to the service SLO rows.
    pub slo_rows_ref: String,
    /// Ref to the canonical M5 evidence index.
    pub evidence_index_ref: String,
    /// The contract family this catalog publishes.
    pub family_id: String,
    /// The effective published label for the family.
    pub family_lifecycle_label: LifecycleLabel,
    /// Ref to the primary OpenAPI document.
    pub primary_openapi_document_ref: String,
    /// Home directory for the example packs.
    pub example_pack_home: String,
    /// Closed HTTP-method vocabulary.
    pub http_methods: Vec<HttpMethod>,
    /// Closed auth-source vocabulary.
    pub auth_source_classes: Vec<AuthSourceClass>,
    /// Closed entitlement vocabulary.
    pub entitlement_classes: Vec<EntitlementClass>,
    /// Closed policy-override vocabulary.
    pub policy_override_postures: Vec<PolicyOverridePosture>,
    /// Closed mutability vocabulary.
    pub mutability_postures: Vec<MutabilityPosture>,
    /// Closed preview-support vocabulary.
    pub preview_support_classes: Vec<PreviewSupportClass>,
    /// Closed offline-behaviour vocabulary.
    pub offline_behavior_classes: Vec<OfflineBehaviorClass>,
    /// Closed deprecation-lane vocabulary.
    pub deprecation_lane_classes: Vec<DeprecationLaneClass>,
    /// Closed sunset-posture vocabulary.
    pub sunset_postures: Vec<SunsetPosture>,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<LifecycleLabel>,
    /// Closed maturity-lane vocabulary.
    pub maturity_lanes: Vec<MaturityLane>,
    /// Closed downgrade-behavior vocabulary.
    pub downgrade_behaviors: Vec<DowngradeBehavior>,
    /// The offline/mirror bundling declaration.
    pub offline_bundle: OfflineBundle,
    /// The published endpoints.
    pub endpoints: Vec<OpenapiEndpoint>,
    /// Summary counts.
    pub summary: M5OpenapiCatalogSummary,
}

impl M5OpenapiCatalog {
    /// Returns the endpoint registered for `endpoint_id`.
    pub fn endpoint(&self, endpoint_id: &str) -> Option<&OpenapiEndpoint> {
        self.endpoints.iter().find(|e| e.endpoint_id == endpoint_id)
    }

    /// Endpoints belonging to a surface row.
    pub fn endpoints_for_surface(&self, api_surface_id: &str) -> Vec<&OpenapiEndpoint> {
        self.endpoints
            .iter()
            .filter(|e| e.api_surface_id == api_surface_id)
            .collect()
    }

    /// The read-only endpoints, in catalog order.
    pub fn read_only_endpoints(&self) -> Vec<&OpenapiEndpoint> {
        self.endpoints.iter().filter(|e| e.is_read_only()).collect()
    }

    /// True when the family publishes at or above the stable cutline.
    pub fn publishes_stable(&self) -> bool {
        self.family_lifecycle_label.holds_stable()
    }

    /// Recomputes the summary block from the endpoints.
    pub fn computed_summary(&self) -> M5OpenapiCatalogSummary {
        let count =
            |f: &dyn Fn(&OpenapiEndpoint) -> bool| self.endpoints.iter().filter(|e| f(e)).count();
        let surfaces: BTreeSet<&str> = self
            .endpoints
            .iter()
            .map(|e| e.api_surface_id.as_str())
            .collect();
        let families: BTreeSet<&str> = self
            .endpoints
            .iter()
            .map(|e| e.api_family_class.as_str())
            .collect();
        M5OpenapiCatalogSummary {
            total_endpoints: self.endpoints.len(),
            read_only_endpoints: count(&|e| e.is_read_only()),
            mutating_endpoints: count(&|e| !e.is_read_only()),
            append_only_endpoints: count(&|e| {
                e.mutability_posture == MutabilityPosture::MutatingAppendOnly
            }),
            endpoints_with_request_example: count(&|e| e.has_request_body()),
            endpoints_with_dry_run_or_preview: count(&|e| e.supports_dry_run()),
            open_no_auth_endpoints: count(&|e| e.auth_source_class.is_open()),
            auth_required_endpoints: count(&|e| !e.auth_source_class.is_open()),
            service_surface_count: surfaces.len(),
            distinct_api_families: families.len(),
        }
    }

    /// Validates the catalog's structural invariants.
    ///
    /// Mirrors the CI validator's semantic invariants. The checked-in catalog
    /// returns no violations; each negative fixture returns at least one.
    pub fn validate(&self) -> Vec<M5OpenapiCatalogViolation> {
        let mut out = Vec::new();
        let mut push = |check_id: &str, detail: String| {
            out.push(M5OpenapiCatalogViolation {
                check_id: check_id.to_string(),
                detail,
            })
        };

        if self.schema_version != M5_OPENAPI_CATALOG_SCHEMA_VERSION {
            push(
                "catalog.schema_version",
                format!("unexpected schema_version {}", self.schema_version),
            );
        }
        if self.record_kind != M5_OPENAPI_CATALOG_RECORD_KIND {
            push(
                "catalog.record_kind",
                format!("unexpected record_kind {}", self.record_kind),
            );
        }
        if self.catalog_id != M5_OPENAPI_CATALOG_ID {
            push(
                "catalog.catalog_id",
                format!("unexpected catalog_id {}", self.catalog_id),
            );
        }
        if self.family_id != M5_OPENAPI_CATALOG_FAMILY_ID {
            push(
                "catalog.family_id",
                format!("unexpected family_id {}", self.family_id),
            );
        }

        if self.http_methods != HttpMethod::ALL {
            push(
                "vocab.http_methods",
                "http_methods off the canonical list".into(),
            );
        }
        if self.auth_source_classes != AuthSourceClass::ALL {
            push(
                "vocab.auth_source_classes",
                "auth_source_classes off the canonical list".into(),
            );
        }
        if self.entitlement_classes != EntitlementClass::ALL {
            push(
                "vocab.entitlement_classes",
                "entitlement_classes off the canonical list".into(),
            );
        }
        if self.policy_override_postures != PolicyOverridePosture::ALL {
            push(
                "vocab.policy_override_postures",
                "policy_override_postures off the canonical list".into(),
            );
        }
        if self.mutability_postures != MutabilityPosture::ALL {
            push(
                "vocab.mutability_postures",
                "mutability_postures off the canonical list".into(),
            );
        }
        if self.preview_support_classes != PreviewSupportClass::ALL {
            push(
                "vocab.preview_support_classes",
                "preview_support_classes off the canonical list".into(),
            );
        }
        if self.offline_behavior_classes != OfflineBehaviorClass::ALL {
            push(
                "vocab.offline_behavior_classes",
                "offline_behavior_classes off the canonical list".into(),
            );
        }
        if self.deprecation_lane_classes != DeprecationLaneClass::ALL {
            push(
                "vocab.deprecation_lane_classes",
                "deprecation_lane_classes off the canonical list".into(),
            );
        }
        if self.sunset_postures != SunsetPosture::ALL {
            push(
                "vocab.sunset_postures",
                "sunset_postures off the canonical list".into(),
            );
        }
        if self.lifecycle_labels != LifecycleLabel::ALL {
            push(
                "vocab.lifecycle_labels",
                "lifecycle_labels off the canonical list".into(),
            );
        }
        if self.maturity_lanes != MaturityLane::ALL {
            push(
                "vocab.maturity_lanes",
                "maturity_lanes off the canonical list".into(),
            );
        }
        if self.downgrade_behaviors != DowngradeBehavior::ALL {
            push(
                "vocab.downgrade_behaviors",
                "downgrade_behaviors off the canonical list".into(),
            );
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for ep in &self.endpoints {
            let eid = ep.endpoint_id.as_str();
            if !seen.insert(eid) {
                push(
                    "endpoints.duplicate_endpoint_id",
                    format!("duplicate endpoint_id {eid}"),
                );
            }
            if ep.lifecycle_label != self.family_lifecycle_label {
                push(
                    "endpoints.lifecycle_wider_than_family",
                    format!("{eid}: lifecycle_label disagrees with the family label"),
                );
            }
            if ep.is_read_only() {
                if ep.has_request_body() {
                    push(
                        "endpoints.read_only_with_request_body",
                        format!("{eid}: read_only endpoint carries a request body"),
                    );
                }
                if ep.preview_support_class != PreviewSupportClass::ReadOnlyNoMutation {
                    push(
                        "endpoints.read_only_preview_class",
                        format!("{eid}: read_only endpoint must use read_only_no_mutation preview"),
                    );
                }
            } else if ep.preview_support_class == PreviewSupportClass::ReadOnlyNoMutation {
                push(
                    "endpoints.mutating_preview_class",
                    format!("{eid}: mutating endpoint must not use read_only_no_mutation preview"),
                );
            }
            if ep.response_schema_ref.is_empty() {
                push(
                    "endpoints.missing_response_schema",
                    format!("{eid}: missing response_schema_ref"),
                );
            }
            if ep.example_pack_ref.is_empty() {
                push(
                    "endpoints.missing_example_pack",
                    format!("{eid}: missing example_pack_ref"),
                );
            }
            if ep.validator_suite_refs.is_empty() {
                push(
                    "endpoints.missing_validator_suite",
                    format!("{eid}: missing validator_suite_refs"),
                );
            }
        }

        if self.summary != self.computed_summary() {
            push(
                "summary.count_mismatch",
                "summary counts disagree with the endpoints".into(),
            );
        }

        out
    }
}

/// Parses the embedded checked-in catalog into the typed model.
pub fn current_m5_openapi_catalog() -> Result<M5OpenapiCatalog, serde_json::Error> {
    serde_json::from_str(M5_OPENAPI_CATALOG_JSON)
}

#[cfg(test)]
mod tests;
