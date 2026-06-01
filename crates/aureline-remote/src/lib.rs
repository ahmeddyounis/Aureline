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
//! Three principal models live here:
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
//!
//! All records reuse closed-vocabulary tokens so the surface a user sees in
//! the UI is identical to the tokens logs, audits, and exports quote.
//!
//! No raw URLs, hostnames, IPs, ports, paths, query strings, cookies,
//! headers, or token bytes ever appear on any record. Only opaque handles
//! and closed-vocabulary tokens cross the boundary.

#![doc(html_root_url = "https://docs.rs/aureline-remote/0.0.0")]

pub mod route_governance;
pub mod stabilize_transport_governance_and_egress_classification_across_update;

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

pub use stabilize_transport_governance_and_egress_classification_across_update::{
    audit_transport_governance_page, seeded_transport_governance_page,
    seeded_transport_policy_snapshot, validate_transport_governance_page,
    ControlPlaneStatusClass, DataPlaneStatusClass, DependencyClass, EgressDecisionClass,
    EgressLaneClass, EgressRouteClass, OfflinePostureClass, TransportGovernanceDefect,
    TransportGovernancePage, TransportGovernanceNarrowReasonClass,
    TransportGovernanceQualificationClass, TransportGovernanceRow, TransportGovernanceSummary,
    TransportGovernanceSupportExport, TransportPolicyRecord, TransportPolicySnapshot,
    REQUIRED_EGRESS_LANES, TRANSPORT_GOVERNANCE_ARTIFACT_REF, TRANSPORT_GOVERNANCE_DOC_REF,
    TRANSPORT_GOVERNANCE_DEFECT_RECORD_KIND, TRANSPORT_GOVERNANCE_PAGE_RECORD_KIND,
    TRANSPORT_GOVERNANCE_ROW_RECORD_KIND, TRANSPORT_GOVERNANCE_SCHEMA_VERSION,
    TRANSPORT_GOVERNANCE_SHARED_CONTRACT_REF, TRANSPORT_GOVERNANCE_SUMMARY_RECORD_KIND,
    TRANSPORT_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND, TRANSPORT_POLICY_RECORD_KIND,
};
