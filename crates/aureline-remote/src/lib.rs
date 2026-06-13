//! Governed route objects, exposure-review sheets, revocation truth, and
//! transport governance for port-forward, tunnel, preview-route, exposed-
//! service, and egress-classification rows.
//!
//! This crate owns the typed truth model that replaces convenience-toggle
//! exposure with a route object whose source service/process, host/workspace
//! identity, port/path handles, exposure class, audience, auth/TLS posture,
//! expiry, last-access posture, and revocation behavior are all reviewable
//! before and after the route becomes reachable.
//!
//! The principal models include:
//!
//! - [`route_governance::RouteObject`] — the stable route-truth row consumed
//!   by UI, audits, issue reports, and support exports. Mirrors the boundary
//!   schema at `/schemas/remote/route_object.schema.json`.
//! - [`route_governance::ExposureReview`] — the typed review sheet a widen,
//!   share, copy, or open step MUST surface before a route widens its
//!   audience. Mirrors the boundary schema at
//!   `/schemas/remote/exposure_review.schema.json`.
//! - [`stabilize_transport_governance_and_egress_classification_across_update::TransportGovernancePage`]
//!   — the stable proof packet that makes egress routing for every named lane
//!   (update, marketplace, AI, docs, provider, remote, mirror/offline)
//!   inspectable through one typed vocabulary instead of subsystem-specific
//!   status strings.
//! - [`finalize_qualification_rows_for_desktop_local_remote_helper::QualificationMatrixPage`]
//!   — the stable qualification-matrix proof packet covering desktop-local,
//!   remote/helper, provider-linked, state/schema, and accessibility surfaces
//!   across all deployment profiles (local OSS, self-hosted, managed, and
//!   air-gapped).
//! - [`managed_workspace_lifecycle::ManagedWorkspaceLifecyclePage`] — the
//!   stable lifecycle proof packet that makes managed-workspace provision,
//!   warm, ready, suspend, resume, reconnect, rebuild, recreate, expiry, and
//!   local-safe continuation a reviewed concept whose continuity, persistence,
//!   provenance, recovery, and caveat truth stays consistent across desktop,
//!   preview, companion, incident, and support/export surfaces. Mirrors the
//!   boundary schema at
//!   `/schemas/remote/managed_workspace_lifecycle.schema.json`.
//! - [`networked_surface_transport_matrix::NetworkedSurfaceTransportMatrixPage`]
//!   — the stable matrix proof packet that freezes the origin scope, endpoint
//!   class, egress class, route choice, auth posture, trust material, denial
//!   vocabulary, and mirror/offline behavior for every newer network-capable
//!   surface (AI gateways, docs/browser fetchers, request/API clients,
//!   database/cloud connectors, registry reads, companion handoffs, provider
//!   mutation, sync/offboarding, and remote preview routes) through one shared
//!   vocabulary. Mirrors the boundary schema at
//!   `/schemas/network/networked_surface_transport_matrix.schema.json`.
//! - [`networked_surface_transport_decision::TransportDecisionLogPage`] — the
//!   runtime decision layer that pairs with the frozen matrix: for every
//!   network-capable action it emits one inspectable
//!   [`networked_surface_transport_decision::TransportDecision`] resolving a
//!   [`networked_surface_transport_decision::TransportPolicySnapshot`] against
//!   an [`networked_surface_transport_decision::EndpointDescriptor`] and a
//!   typed [`networked_surface_transport_decision::TransportOutcomeClass`]
//!   before any side effects leave the boundary. Mirrors the boundary schema at
//!   `/schemas/network/networked_surface_transport_decision.schema.json`.
//! - [`networked_surface_proxy_resolution::ProxyResolutionGovernancePage`] —
//!   the proxy-resolution layer that makes the proxy-resolution step a
//!   first-class governed object: for every network-capable surface it freezes
//!   the ordered candidate chain resolution walked (PAC → manual → environment
//!   → system → declared direct), the winning tier, and a typed
//!   [`networked_surface_proxy_resolution::ProxyResolutionDenialClass`]
//!   (`deny_proxy_resolution`) when no tier may be honored, so the precedence is
//!   inspectable and no helper, client, or extension can ship a private proxy
//!   stack, a direct CA override, or a silent direct-connect fallback. Mirrors
//!   the boundary schema at
//!   `/schemas/network/networked_surface_proxy_resolution.schema.json`.
//! - [`networked_surface_transport_trust::TransportTrustPage`] — the
//!   transport-trust layer that makes the trust inputs and host proof behind
//!   every network-capable surface a first-class governed object: for each
//!   covered surface it freezes the trust-store source, the organization CA
//!   bundle / pin-set review state, the SSH/TLS host-proof state and history
//!   depth, the client-certificate binding posture, and the trust-root freshness
//!   and rotation cue, emitting a typed
//!   [`networked_surface_transport_trust::TrustDenialClass`] (`deny_trust`) or a
//!   typed host-proof state when a trust input is missing or stale, so no helper,
//!   client, or extension can ship a direct CA override or silently downgrade
//!   trust. Mirrors the boundary schema at
//!   `/schemas/network/networked_surface_transport_trust.schema.json`.
//! - [`networked_surface_transport_explainability::TransportExplainabilityPage`]
//!   — the explainability layer that projects the decision stream into
//!   current-posture inspectors, a recent network-event ledger filterable by
//!   endpoint class, origin scope, and allow/deny outcome, and per-action
//!   explain sheets rendered through one stable field catalog so product,
//!   CLI/headless, diagnostics, and support exports quote identical decision
//!   codes and field names. Mirrors the boundary schema at
//!   `/schemas/network/networked_surface_transport_explainability.schema.json`.
//!
//! All records reuse closed-vocabulary tokens so the surface a user sees in
//! the UI is identical to the tokens logs, audits, and exports quote.
//!
//! No raw URLs, hostnames, IPs, ports, paths, query strings, cookies,
//! headers, or token bytes ever appear on any record. Only opaque handles
//! and closed-vocabulary tokens cross the boundary.

#![doc(html_root_url = "https://docs.rs/aureline-remote/0.0.0")]

pub mod finalize_qualification_rows_for_desktop_local_remote_helper;
pub mod harden_the_connected_provider_registry_capability_matrix_and;
pub mod managed_workspace_lifecycle;
pub mod networked_surface_proxy_resolution;
pub mod networked_surface_transport_decision;
pub mod networked_surface_transport_explainability;
pub mod networked_surface_transport_matrix;
pub mod networked_surface_transport_trust;
pub mod route_governance;
pub mod stabilize_transport_governance_and_egress_classification_across_update;

pub use finalize_qualification_rows_for_desktop_local_remote_helper::{
    audit_qualification_matrix_page, seeded_qualification_matrix_page,
    seeded_qualification_snapshot, validate_qualification_matrix_page, AccessibilityFeatureClass,
    DependencyClass as QualificationDependencyClass, DeploymentProfileClass, FailureDowngradeClass,
    MatrixSurfaceClass, NarrowReasonClass, QualificationMatrixDefect, QualificationMatrixPage,
    QualificationMatrixRow, QualificationMatrixSummary, QualificationMatrixSupportExport,
    QualificationRecord, QualificationSnapshot, QualificationTierClass,
    QUALIFICATION_MATRIX_ARTIFACT_REF, QUALIFICATION_MATRIX_DEFECT_RECORD_KIND,
    QUALIFICATION_MATRIX_DOC_REF, QUALIFICATION_MATRIX_PAGE_RECORD_KIND,
    QUALIFICATION_MATRIX_ROW_RECORD_KIND, QUALIFICATION_MATRIX_SCHEMA_VERSION,
    QUALIFICATION_MATRIX_SHARED_CONTRACT_REF, QUALIFICATION_MATRIX_SUMMARY_RECORD_KIND,
    QUALIFICATION_MATRIX_SUPPORT_EXPORT_RECORD_KIND, QUALIFICATION_RECORD_KIND,
    REQUIRED_ACCESSIBILITY_FEATURES, REQUIRED_ROW_COUNT, REQUIRED_SURFACE_PROFILE_PAIRS,
};

pub use managed_workspace_lifecycle::{
    audit_lifecycle_page, seeded_lifecycle_snapshot, seeded_managed_workspace_lifecycle_page,
    validate_lifecycle_page, CaveatClass, ContinuityClass, ExpiryClass, LifecycleDefect,
    LifecycleDispositionClass, LifecycleMatrixRow, LifecycleRecord, LifecycleSnapshot,
    LifecycleStateClass, LifecycleSummary, LifecycleSupportExport, ManagedWorkspaceLifecyclePage,
    NarrowReasonClass as ManagedWorkspaceLifecycleNarrowReasonClass, PersistenceClass,
    ProvenanceClass, RecoveryOptionClass, SurfaceClass as ManagedWorkspaceSurfaceClass,
    TransitionReasonClass, MANAGED_WORKSPACE_LIFECYCLE_ARTIFACT_REF,
    MANAGED_WORKSPACE_LIFECYCLE_DEFECT_RECORD_KIND, MANAGED_WORKSPACE_LIFECYCLE_DOC_REF,
    MANAGED_WORKSPACE_LIFECYCLE_PAGE_RECORD_KIND, MANAGED_WORKSPACE_LIFECYCLE_RECORD_KIND,
    MANAGED_WORKSPACE_LIFECYCLE_ROW_RECORD_KIND, MANAGED_WORKSPACE_LIFECYCLE_SCHEMA_VERSION,
    MANAGED_WORKSPACE_LIFECYCLE_SHARED_CONTRACT_REF,
    MANAGED_WORKSPACE_LIFECYCLE_SUMMARY_RECORD_KIND,
    MANAGED_WORKSPACE_LIFECYCLE_SUPPORT_EXPORT_RECORD_KIND, REQUIRED_LIFECYCLE_STATES,
    REQUIRED_RECORD_COUNT as MANAGED_WORKSPACE_LIFECYCLE_REQUIRED_RECORD_COUNT,
    REQUIRED_SURFACES as MANAGED_WORKSPACE_LIFECYCLE_REQUIRED_SURFACES,
};

pub use route_governance::{
    AudienceBlock, AudienceClass, AuthBlock, AuthSourceClass, ControlledExposureLabel,
    CookieSessionClass, CopyDisclosureClass, CopyShareBlock, CrossOriginClass,
    DataSensitivityClass, DowngradeBlock, DowngradeState, EndpointHandlesBlock, ExposureLabelClass,
    ExposureReview, ExposureReviewFinding, HostClass, HostIdentityBlock, LastAccessBlock,
    LastAccessClass, LifecycleState, LingeringLocalPreviewClass, ProposedTransition, ProtocolClass,
    ReachabilityLocalClass, ReachabilityPublicClass, ReopenClass, ReviewOutcomeClass,
    RevocationBlock, RevocationSummary, RevokePostureClass, RouteKind, RouteObject,
    RouteObjectFinding, SourceBlock, StaleSharedLinkState, TeardownState, TlsPostureClass,
    ViewerStateClass, EXPOSURE_REVIEW_RECORD_KIND, EXPOSURE_REVIEW_SCHEMA_VERSION,
    ROUTE_OBJECT_RECORD_KIND, ROUTE_OBJECT_SCHEMA_VERSION,
};

pub use harden_the_connected_provider_registry_capability_matrix_and::{
    audit_provider_registry_page, seeded_provider_descriptor_snapshot,
    seeded_provider_registry_page, validate_provider_registry_page, ActorIdentityClass,
    CallbackPathClass, DependencyClass as ProviderRegistryDependencyClass, MutationPostureClass,
    ObjectKindClass, ObjectSupportEntry, ProviderDescriptorRecord, ProviderDescriptorSnapshot,
    ProviderFamilyClass, ProviderRegistryDefect, ProviderRegistryNarrowReasonClass,
    ProviderRegistryPage, ProviderRegistryQualificationClass, ProviderRegistryRow,
    ProviderRegistrySummary, ProviderRegistrySupportExport, PublishModeClass,
    SnapshotFreshnessClass, PROVIDER_DESCRIPTOR_RECORD_KIND, PROVIDER_REGISTRY_ARTIFACT_REF,
    PROVIDER_REGISTRY_DEFECT_RECORD_KIND, PROVIDER_REGISTRY_DOC_REF,
    PROVIDER_REGISTRY_PAGE_RECORD_KIND, PROVIDER_REGISTRY_ROW_RECORD_KIND,
    PROVIDER_REGISTRY_SCHEMA_VERSION, PROVIDER_REGISTRY_SHARED_CONTRACT_REF,
    PROVIDER_REGISTRY_SUMMARY_RECORD_KIND, PROVIDER_REGISTRY_SUPPORT_EXPORT_RECORD_KIND,
    REQUIRED_DESCRIPTOR_PAIRS,
};

pub use stabilize_transport_governance_and_egress_classification_across_update::{
    audit_transport_governance_page, seeded_transport_governance_page,
    seeded_transport_policy_snapshot, validate_transport_governance_page, ControlPlaneStatusClass,
    DataPlaneStatusClass, DependencyClass, EgressDecisionClass, EgressLaneClass, EgressRouteClass,
    OfflinePostureClass, TransportGovernanceDefect, TransportGovernanceNarrowReasonClass,
    TransportGovernancePage, TransportGovernanceQualificationClass, TransportGovernanceRow,
    TransportGovernanceSummary, TransportGovernanceSupportExport, TransportPolicyRecord,
    TransportPolicySnapshot, REQUIRED_EGRESS_LANES, TRANSPORT_GOVERNANCE_ARTIFACT_REF,
    TRANSPORT_GOVERNANCE_DEFECT_RECORD_KIND, TRANSPORT_GOVERNANCE_DOC_REF,
    TRANSPORT_GOVERNANCE_PAGE_RECORD_KIND, TRANSPORT_GOVERNANCE_ROW_RECORD_KIND,
    TRANSPORT_GOVERNANCE_SCHEMA_VERSION, TRANSPORT_GOVERNANCE_SHARED_CONTRACT_REF,
    TRANSPORT_GOVERNANCE_SUMMARY_RECORD_KIND, TRANSPORT_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND,
    TRANSPORT_POLICY_RECORD_KIND,
};

pub use networked_surface_transport_matrix::{
    audit_networked_surface_matrix_page, seeded_networked_surface_matrix_page,
    seeded_networked_surface_matrix_snapshot, validate_networked_surface_matrix_page,
    AuthPostureClass, DenialReasonClass, EgressClass, EndpointClass, MatrixNarrowReasonClass,
    MatrixQualificationClass, MirrorOfflineBehaviorClass, NetworkedSurfaceMatrixDefect,
    NetworkedSurfaceMatrixRow, NetworkedSurfaceMatrixSnapshot, NetworkedSurfaceMatrixSummary,
    NetworkedSurfaceMatrixSupportExport, NetworkedSurfaceRecord,
    NetworkedSurfaceTransportMatrixPage, OriginScopeClass, ProofFreshnessClass, RouteChoiceClass,
    SurfaceClass as NetworkedSurfaceClass, TrustMaterialClass,
    NETWORKED_SURFACE_MATRIX_ARTIFACT_REF, NETWORKED_SURFACE_MATRIX_DEFECT_RECORD_KIND,
    NETWORKED_SURFACE_MATRIX_DOC_REF, NETWORKED_SURFACE_MATRIX_EVIDENCE_INDEX_REF,
    NETWORKED_SURFACE_MATRIX_PAGE_RECORD_KIND, NETWORKED_SURFACE_MATRIX_ROW_RECORD_KIND,
    NETWORKED_SURFACE_MATRIX_SCHEMA_VERSION, NETWORKED_SURFACE_MATRIX_SHARED_CONTRACT_REF,
    NETWORKED_SURFACE_MATRIX_SUMMARY_RECORD_KIND,
    NETWORKED_SURFACE_MATRIX_SUPPORT_EXPORT_RECORD_KIND,
    NETWORKED_SURFACE_MATRIX_SURFACE_RECORD_KIND, REQUIRED_SURFACES as REQUIRED_NETWORKED_SURFACES,
};

pub use networked_surface_transport_decision::{
    audit_transport_decision_page, seeded_transport_decision_page,
    seeded_transport_decision_snapshot, validate_transport_decision_page,
    DecisionNarrowReasonClass, DecisionQualificationClass, EndpointDescriptor,
    ProxyResolutionSourceClass, TransportDecision, TransportDecisionDefect,
    TransportDecisionLogPage, TransportDecisionRow, TransportDecisionSnapshot,
    TransportDecisionSummary, TransportDecisionSupportExport, TransportOutcomeClass,
    TransportPolicySnapshot as NetworkedSurfaceTransportPolicySnapshot,
    TRANSPORT_DECISION_ARTIFACT_REF, TRANSPORT_DECISION_DEFECT_RECORD_KIND,
    TRANSPORT_DECISION_DOC_REF, TRANSPORT_DECISION_ENDPOINT_RECORD_KIND,
    TRANSPORT_DECISION_EVIDENCE_INDEX_REF, TRANSPORT_DECISION_PAGE_RECORD_KIND,
    TRANSPORT_DECISION_POLICY_SNAPSHOT_RECORD_KIND, TRANSPORT_DECISION_RECORD_KIND,
    TRANSPORT_DECISION_ROW_RECORD_KIND, TRANSPORT_DECISION_SCHEMA_VERSION,
    TRANSPORT_DECISION_SHARED_CONTRACT_REF, TRANSPORT_DECISION_SUMMARY_RECORD_KIND,
    TRANSPORT_DECISION_SUPPORT_EXPORT_RECORD_KIND,
};

pub use networked_surface_proxy_resolution::{
    audit_proxy_resolution_page, seeded_proxy_resolution_page, seeded_proxy_resolution_snapshot,
    validate_proxy_resolution_page, ProxyCandidate, ProxyNarrowReasonClass,
    ProxyQualificationClass, ProxyResolutionDefect, ProxyResolutionDenialClass,
    ProxyResolutionGovernancePage, ProxyResolutionOutcomeClass, ProxyResolutionRecord,
    ProxyResolutionRow, ProxyResolutionSnapshot, ProxyResolutionSummary,
    ProxyResolutionSupportExport, ProxyResolutionTierClass, PROXY_RESOLUTION_ARTIFACT_REF,
    PROXY_RESOLUTION_CANDIDATE_RECORD_KIND, PROXY_RESOLUTION_DEFECT_RECORD_KIND,
    PROXY_RESOLUTION_DOC_REF, PROXY_RESOLUTION_EVIDENCE_INDEX_REF,
    PROXY_RESOLUTION_PAGE_RECORD_KIND, PROXY_RESOLUTION_RECORD_KIND,
    PROXY_RESOLUTION_ROW_RECORD_KIND, PROXY_RESOLUTION_SCHEMA_VERSION,
    PROXY_RESOLUTION_SHARED_CONTRACT_REF, PROXY_RESOLUTION_SUMMARY_RECORD_KIND,
    PROXY_RESOLUTION_SUPPORT_EXPORT_RECORD_KIND,
};

pub use networked_surface_transport_trust::{
    audit_transport_trust_page, seeded_transport_trust_page, seeded_transport_trust_snapshot,
    validate_transport_trust_page, CaBundleDescriptor, CaBundleReviewClass, ClientCertDescriptor,
    ClientCertPostureClass, HostProofDescriptor, HostProofStateClass, RotationCueClass,
    TransportTrustPage, TrustDefect, TrustDenialClass, TrustEvaluationRecord,
    TrustNarrowReasonClass, TrustOutcomeClass, TrustQualificationClass, TrustRootDescriptor,
    TrustRootFreshnessClass, TrustRow, TrustSnapshot, TrustStoreSourceClass, TrustSummary,
    TrustSupportExport, TRANSPORT_TRUST_ARTIFACT_REF, TRANSPORT_TRUST_CA_BUNDLE_RECORD_KIND,
    TRANSPORT_TRUST_CLIENT_CERT_RECORD_KIND, TRANSPORT_TRUST_DEFECT_RECORD_KIND,
    TRANSPORT_TRUST_DOC_REF, TRANSPORT_TRUST_EVALUATION_RECORD_KIND,
    TRANSPORT_TRUST_EVIDENCE_INDEX_REF, TRANSPORT_TRUST_HOST_PROOF_RECORD_KIND,
    TRANSPORT_TRUST_PAGE_RECORD_KIND, TRANSPORT_TRUST_ROOT_RECORD_KIND,
    TRANSPORT_TRUST_ROW_RECORD_KIND, TRANSPORT_TRUST_SCHEMA_VERSION,
    TRANSPORT_TRUST_SHARED_CONTRACT_REF, TRANSPORT_TRUST_SUMMARY_RECORD_KIND,
    TRANSPORT_TRUST_SUPPORT_EXPORT_RECORD_KIND,
};

pub use networked_surface_transport_explainability::{
    audit_transport_explainability_page, seeded_transport_explainability_page,
    seeded_transport_explainability_snapshot, validate_transport_explainability_page,
    ActionExplainSheet, EventDispositionClass, ExplainField, ExplainNarrowReasonClass,
    ExplainQualificationClass, NetworkEventLedger, NetworkEventLedgerEntry,
    TransportExplainabilityDefect, TransportExplainabilityPage, TransportExplainabilityRow,
    TransportExplainabilitySummary, TransportExplainabilitySupportExport,
    TransportPostureInspector, EXPLAIN_FIELD_NAMES, TRANSPORT_EXPLAINABILITY_ARTIFACT_REF,
    TRANSPORT_EXPLAINABILITY_DEFECT_RECORD_KIND, TRANSPORT_EXPLAINABILITY_DOC_REF,
    TRANSPORT_EXPLAINABILITY_EVENT_RECORD_KIND, TRANSPORT_EXPLAINABILITY_EVIDENCE_INDEX_REF,
    TRANSPORT_EXPLAINABILITY_EXPLAIN_SHEET_RECORD_KIND,
    TRANSPORT_EXPLAINABILITY_LEDGER_RECORD_KIND, TRANSPORT_EXPLAINABILITY_PAGE_RECORD_KIND,
    TRANSPORT_EXPLAINABILITY_POSTURE_RECORD_KIND, TRANSPORT_EXPLAINABILITY_ROW_RECORD_KIND,
    TRANSPORT_EXPLAINABILITY_SCHEMA_VERSION, TRANSPORT_EXPLAINABILITY_SHARED_CONTRACT_REF,
    TRANSPORT_EXPLAINABILITY_SUMMARY_RECORD_KIND,
    TRANSPORT_EXPLAINABILITY_SUPPORT_EXPORT_RECORD_KIND,
};
