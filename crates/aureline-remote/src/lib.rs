//! Governed route objects, exposure-review sheets, and revocation truth for
//! port-forward, tunnel, preview-route, and exposed-service rows.
//!
//! This crate owns the typed truth model that replaces convenience-toggle
//! exposure with a route object whose source service/process, host/workspace
//! identity, port/path handles, exposure class, audience, auth/TLS posture,
//! expiry, last-access posture, and revocation behavior are all reviewable
//! before and after the route becomes reachable.
//!
//! Two records live here:
//!
//! - [`route_governance::RouteObject`] — the stable route-truth row consumed
//!   by UI, audits, issue reports, and support exports. Mirrors the boundary
//!   schema at `/schemas/remote/route_object.schema.json`.
//! - [`route_governance::ExposureReview`] — the typed review sheet a widen,
//!   share, copy, or open step MUST surface before a route widens its
//!   audience. Mirrors the boundary schema at
//!   `/schemas/remote/exposure_review.schema.json`.
//!
//! Both records reuse a single closed `ControlledExposureLabel` vocabulary so
//! the surface a user sees ("Local only", "Same device / LAN", "Authenticated
//! org route", "Signed preview link", "Public route") is identical to the
//! token logs, audits, and exports quote.
//!
//! No raw URLs, hostnames, IPs, ports, paths, query strings, cookies,
//! headers, or token bytes ever appear on either record. Only opaque handles
//! and closed-vocabulary tokens cross the boundary.

#![doc(html_root_url = "https://docs.rs/aureline-remote/0.0.0")]

pub mod route_governance;

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
