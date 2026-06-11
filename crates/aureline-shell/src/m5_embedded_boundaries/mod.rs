//! Owner/origin, freshness, trust-boundary, system-browser-auth, no-embedded
//! high-risk-approval, return-anchor, and handoff-reason qualification audit for
//! the M5 depth embedded and provider-owned panes.
//!
//! The M5 depth lanes mint new browser-like and provider-owned surfaces —
//! embedded docs/help viewers, request/runtime viewers, live preview-route
//! panes, marketplace/account surfaces, review/provider panes, and
//! companion/browser handoff entry points. Each is easy to ship as an opaque
//! iframe that pretends to be first-party local truth, quietly becomes the
//! primary approval channel for a high-risk or scope-widening action, drops the
//! reason Aureline left a governed in-product surface, or strands the user in an
//! external browser with no way back. This module carries the stable v1 shell
//! promise forward into those lanes: every marketed M5 embedded or provider
//! surface MUST expose owner/origin and freshness chrome, MUST stay clearly
//! bounded and attributed instead of pretending to be first-party, MUST default
//! claimed-identity and provider auth to the system browser (or an equally
//! explicit native flow) rather than an embedded approval, MUST block or route
//! around high-risk embedded approvals, MUST emit a return anchor and a handoff
//! reason whenever it leaves for an external browser, vendor portal, or provider
//! console, and MUST stay support-safe so support bundles, docs/help, and
//! release packets reference the same destination descriptors shown in-product.
//!
//! The audit projects, for each registered M5 embedded surface, the canonical
//! surface descriptor against the qualification result the surface actually
//! certifies for each of the eight embedded-boundary guarantees the M5 lanes
//! must pass:
//!
//! - `owner_origin_disclosure`
//! - `freshness_disclosure`
//! - `trust_boundary_chrome`
//! - `system_browser_auth_default`
//! - `no_embedded_high_risk_approval`
//! - `return_anchor_present`
//! - `handoff_reason_preserved`
//! - `support_export_parity`
//!
//! The resulting [`M5EmbeddedBoundaryReport`] is the canonical truth object for
//! the M5 embedded-boundary lane. It is consumed by:
//!
//! - the live shell embedded/provider chrome, docs/help rails, and support
//!   inspector (so the in-product audit quotes the same per-surface findings the
//!   CLI prints);
//! - the headless inspector (`aureline_shell_m5_embedded_boundaries`), which is
//!   the only mint-from-truth path for the JSON fixtures checked in under
//!   `fixtures/ux/m5/webview-auth-handoff/`;
//! - the support-export wrapper that lets a reviewer pivot from a support case
//!   to the surface that hid its owner, became a back-door approval channel, or
//!   dropped a handoff reason;
//! - the markdown audit under
//!   `artifacts/ux/m5/embedded-boundary-audits/m5_embedded_boundaries_audit.md`
//!   (rendered from the same seed); and
//! - the cross-surface hardening matrix and release-center packets, which ingest
//!   the audit directly when qualifying or narrowing a marketed M5 embedded or
//!   provider surface whose boundary evidence is stale or red.
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every registered M5 embedded surface must declare a qualification binding
//!    for each of the eight embedded-boundary guarantees.
//! 2. Every surface must carry a canonical return anchor, a non-empty support
//!    note, a declared boundary class, at least one declared handoff target, and
//!    a flag asserting it rides the one governed embedded-boundary model; a
//!    missing anchor, missing note, missing target, or a surface that invents
//!    its own feature-local boundary rule is a blocker.
//! 3. A qualified guarantee must carry the captured evidence the guarantee
//!    requires — a destination-descriptor ref, a declared boundary class, an
//!    owner/origin disclosure, and an evidence-freshness stamp for every
//!    guarantee; a freshness disclosure for the freshness guarantee; a
//!    trust-chrome outcome for the trust guarantee; an auth-channel outcome for
//!    the system-browser guarantee; a high-risk-handling outcome for the
//!    no-embedded-approval guarantee; a return-anchor outcome for the
//!    return-anchor guarantee; a handoff-reason outcome for the handoff
//!    guarantee; and a support-parity outcome for the support guarantee. A red
//!    result (a hidden owner/origin, a hidden freshness stamp, a surface that
//!    pretends to be first-party, an embedded primary approval, a hidden
//!    high-risk embedded approval, a lost return anchor, a dropped handoff
//!    reason, or a divergent support clone) is a blocker.
//! 4. A surface that paints its own boundary chrome outside the governed model
//!    (`unqualified_local_surface`), and a marketed guarantee claimed with no
//!    captured evidence (`missing_evidence`), are blockers.
//! 5. Stale durable evidence on a marketed guarantee is a blocker, so release
//!    tooling can narrow a marketed M5 surface instead of shipping it as
//!    implicitly stable.
//! 6. At least one surface must qualify each of the eight guarantees so the
//!    audit cannot regress into a single happy-path surface.
//!
//! All identifiers, refs, and label strings are deterministic so the checked-in
//! fixtures under `fixtures/ux/m5/webview-auth-handoff/` are bit-for-bit equal to
//! the seeded report returned by [`seeded_m5_embedded_boundaries_audit`].

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

const GENERATED_AT: &str = "2026-06-11T00:00:00Z";

/// Schema version exported with every M5 embedded-boundary record.
pub const M5_EMBEDDED_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by UI, CLI, docs, and support export.
pub const M5_EMBEDDED_SHARED_CONTRACT_REF: &str = "shell:m5_embedded_boundaries:v1";

/// Stable record kind for the audit report payload.
pub const M5_EMBEDDED_REPORT_RECORD_KIND: &str = "shell_m5_embedded_boundary_report_record";

/// Stable record kind for one per-surface qualification row.
pub const M5_EMBEDDED_ROW_RECORD_KIND: &str = "shell_m5_embedded_boundary_row_record";

/// Stable record kind for the support-export wrapper.
pub const M5_EMBEDDED_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_embedded_boundary_support_export_record";

/// Stable report id quoted across surfaces.
pub const M5_EMBEDDED_REPORT_ID: &str = "shell:m5_embedded_boundaries:audit:v1";

/// Stable support-export id.
pub const M5_EMBEDDED_SUPPORT_EXPORT_ID: &str = "support-export:m5-embedded-boundaries:001";

/// Source schema ref for the canonical contract.
pub const M5_EMBEDDED_SOURCE_SCHEMA_REF: &str =
    "schemas/help/m5-destination-descriptor-diff.schema.json";

/// Markdown publication ref this audit is rendered to.
pub const M5_EMBEDDED_PUBLISHED_REPORT_REF: &str =
    "artifacts/ux/m5/embedded-boundary-audits/m5_embedded_boundaries_audit.md";

/// Companion doc publication ref.
pub const M5_EMBEDDED_PUBLISHED_DOC_REF: &str = "docs/m5/embedded-boundaries-and-auth.md";

/// One M5 depth embedded or provider-owned surface whose ownership, auth, and
/// handoff boundaries the audit qualifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmbeddedSurface {
    /// Embedded docs/help viewer.
    EmbeddedDocs,
    /// Request/runtime (data/API) viewer.
    RequestRuntimeViewer,
    /// Live preview-route pane.
    PreviewRoutePane,
    /// Marketplace / account surface.
    MarketplaceAccount,
    /// Help-center pane.
    HelpCenterPane,
    /// Review / provider-owned pane.
    ProviderReviewPane,
    /// Companion / browser handoff entry point.
    CompanionBrowserHandoff,
    /// Provider-console / vendor-portal handoff entry point.
    ProviderConsoleHandoff,
}

impl M5EmbeddedSurface {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmbeddedDocs => "embedded_docs",
            Self::RequestRuntimeViewer => "request_runtime_viewer",
            Self::PreviewRoutePane => "preview_route_pane",
            Self::MarketplaceAccount => "marketplace_account",
            Self::HelpCenterPane => "help_center_pane",
            Self::ProviderReviewPane => "provider_review_pane",
            Self::CompanionBrowserHandoff => "companion_browser_handoff",
            Self::ProviderConsoleHandoff => "provider_console_handoff",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::EmbeddedDocs => "Embedded docs viewer",
            Self::RequestRuntimeViewer => "Request/runtime viewer",
            Self::PreviewRoutePane => "Preview-route pane",
            Self::MarketplaceAccount => "Marketplace/account surface",
            Self::HelpCenterPane => "Help-center pane",
            Self::ProviderReviewPane => "Provider/review pane",
            Self::CompanionBrowserHandoff => "Companion/browser handoff",
            Self::ProviderConsoleHandoff => "Provider-console handoff",
        }
    }
}

/// Boundary class assigned to a surface.
///
/// `provider_owned` and `external_handoff` are the high-stakes classes: their
/// surfaces carry claimed-identity / provider auth or leave the governed
/// in-product surface, so the audit requires a present return-anchor outcome on
/// every qualified guarantee and a non-empty boundary-chrome set on the
/// descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BoundaryClass {
    /// Aureline-owned local content rendered in a first-party pane.
    FirstPartyLocal,
    /// Bounded embedded browser-like surface rendering external or provider
    /// content.
    EmbeddedWebview,
    /// Provider-owned pane (account, marketplace, or provider console).
    ProviderOwned,
    /// Explicit handoff to the system browser, a vendor portal, or a provider
    /// console.
    ExternalHandoff,
}

impl M5BoundaryClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyLocal => "first_party_local",
            Self::EmbeddedWebview => "embedded_webview",
            Self::ProviderOwned => "provider_owned",
            Self::ExternalHandoff => "external_handoff",
        }
    }

    /// `true` for the classes whose surface is high-stakes for the audit.
    pub const fn is_high_stakes(self) -> bool {
        matches!(self, Self::ProviderOwned | Self::ExternalHandoff)
    }
}

/// The boundary aspect a guarantee belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BoundaryAspect {
    /// Owner/origin, freshness, and trust-boundary chrome.
    Attribution,
    /// System-browser auth default and no-embedded high-risk approval.
    Auth,
    /// Return anchors and handoff reasons.
    Handoff,
    /// Support/export descriptor parity.
    Export,
}

impl M5BoundaryAspect {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Attribution => "attribution",
            Self::Auth => "auth",
            Self::Handoff => "handoff",
            Self::Export => "export",
        }
    }
}

/// One embedded-boundary guarantee a surface certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BoundaryGuarantee {
    /// The surface exposes owner/origin chrome instead of pretending to be
    /// ownerless first-party truth.
    OwnerOriginDisclosure,
    /// The surface exposes a freshness stamp for the embedded or provider
    /// content.
    FreshnessDisclosure,
    /// The surface stays clearly bounded and attributed instead of pretending to
    /// be a first-party local surface.
    TrustBoundaryChrome,
    /// Claimed identity and provider auth default to the system browser or an
    /// equally explicit native flow.
    SystemBrowserAuthDefault,
    /// High-risk or scope-widening approvals are blocked or routed out of the
    /// embedded pane.
    NoEmbeddedHighRiskApproval,
    /// A return anchor resolves the exact in-product surface to come back to.
    ReturnAnchorPresent,
    /// The handoff reason is emitted and preserved when Aureline leaves a
    /// governed surface.
    HandoffReasonPreserved,
    /// Support bundles, docs/help, and release packets reuse the same
    /// destination descriptor shown in-product.
    SupportExportParity,
}

impl M5BoundaryGuarantee {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerOriginDisclosure => "owner_origin_disclosure",
            Self::FreshnessDisclosure => "freshness_disclosure",
            Self::TrustBoundaryChrome => "trust_boundary_chrome",
            Self::SystemBrowserAuthDefault => "system_browser_auth_default",
            Self::NoEmbeddedHighRiskApproval => "no_embedded_high_risk_approval",
            Self::ReturnAnchorPresent => "return_anchor_present",
            Self::HandoffReasonPreserved => "handoff_reason_preserved",
            Self::SupportExportParity => "support_export_parity",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::OwnerOriginDisclosure => "Owner/origin disclosure",
            Self::FreshnessDisclosure => "Freshness disclosure",
            Self::TrustBoundaryChrome => "Trust-boundary chrome",
            Self::SystemBrowserAuthDefault => "System-browser auth default",
            Self::NoEmbeddedHighRiskApproval => "No embedded high-risk approval",
            Self::ReturnAnchorPresent => "Return anchor present",
            Self::HandoffReasonPreserved => "Handoff reason preserved",
            Self::SupportExportParity => "Support/export parity",
        }
    }

    /// The eight embedded-boundary guarantees, in canonical order.
    pub const fn required_guarantees() -> [Self; 8] {
        [
            Self::OwnerOriginDisclosure,
            Self::FreshnessDisclosure,
            Self::TrustBoundaryChrome,
            Self::SystemBrowserAuthDefault,
            Self::NoEmbeddedHighRiskApproval,
            Self::ReturnAnchorPresent,
            Self::HandoffReasonPreserved,
            Self::SupportExportParity,
        ]
    }

    /// The aspect this guarantee belongs to.
    pub const fn canonical_aspect(self) -> M5BoundaryAspect {
        match self {
            Self::OwnerOriginDisclosure | Self::FreshnessDisclosure | Self::TrustBoundaryChrome => {
                M5BoundaryAspect::Attribution
            }
            Self::SystemBrowserAuthDefault | Self::NoEmbeddedHighRiskApproval => {
                M5BoundaryAspect::Auth
            }
            Self::ReturnAnchorPresent | Self::HandoffReasonPreserved => M5BoundaryAspect::Handoff,
            Self::SupportExportParity => M5BoundaryAspect::Export,
        }
    }

    /// `true` when a qualified binding must carry a freshness-disclosure outcome.
    pub const fn requires_freshness_disclosure(self) -> bool {
        matches!(self, Self::FreshnessDisclosure)
    }

    /// `true` when a qualified binding must carry a trust-chrome outcome.
    pub const fn requires_trust_chrome(self) -> bool {
        matches!(self, Self::TrustBoundaryChrome)
    }

    /// `true` when a qualified binding must carry an auth-channel outcome.
    pub const fn requires_auth_channel(self) -> bool {
        matches!(self, Self::SystemBrowserAuthDefault)
    }

    /// `true` when a qualified binding must carry a high-risk-handling outcome.
    pub const fn requires_high_risk_handling(self) -> bool {
        matches!(self, Self::NoEmbeddedHighRiskApproval)
    }

    /// `true` when a qualified binding must carry a return-anchor outcome.
    pub const fn requires_return_anchor(self) -> bool {
        matches!(self, Self::ReturnAnchorPresent)
    }

    /// `true` when a qualified binding must carry a handoff-reason outcome.
    pub const fn requires_handoff_reason(self) -> bool {
        matches!(self, Self::HandoffReasonPreserved)
    }

    /// `true` when a qualified binding must carry a support-parity outcome.
    pub const fn requires_support_parity(self) -> bool {
        matches!(self, Self::SupportExportParity)
    }
}

/// Qualification status a surface reports for one embedded-boundary guarantee.
///
/// Only `Qualified` rows project captured evidence and are drift/red checked.
/// `ExplicitlyNarrowed`, `NotApplicable`, `PlatformOmitted`, and
/// `DeclaredCaptureGap` rows are accepted as long as they carry a
/// `narrowing_reason`. `UnqualifiedLocalSurface` (a surface that paints its own
/// boundary chrome outside the governed model) and `MissingEvidence` are
/// blocking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BoundaryStatus {
    /// The guarantee is qualified with captured evidence.
    Qualified,
    /// The surface narrows this guarantee; a `narrowing_reason` MUST be set.
    ExplicitlyNarrowed,
    /// The guarantee does not apply to this surface; a reason MUST be set.
    NotApplicable,
    /// The guarantee is not surfaced on this client/platform; a reason MUST be
    /// set.
    PlatformOmitted,
    /// A provider-backed surface declares a known capture gap honestly; a reason
    /// MUST be set.
    DeclaredCaptureGap,
    /// The surface paints its own boundary chrome through a feature-local rule
    /// outside the governed model. Always a blocker.
    UnqualifiedLocalSurface,
    /// A marketed guarantee is claimed with no captured evidence. Always a
    /// blocker.
    MissingEvidence,
}

impl M5BoundaryStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::ExplicitlyNarrowed => "explicitly_narrowed",
            Self::NotApplicable => "not_applicable",
            Self::PlatformOmitted => "platform_omitted",
            Self::DeclaredCaptureGap => "declared_capture_gap",
            Self::UnqualifiedLocalSurface => "unqualified_local_surface",
            Self::MissingEvidence => "missing_evidence",
        }
    }

    /// `true` for statuses that require a `narrowing_reason`.
    pub const fn requires_narrowing_reason(self) -> bool {
        matches!(
            self,
            Self::ExplicitlyNarrowed
                | Self::NotApplicable
                | Self::PlatformOmitted
                | Self::DeclaredCaptureGap
        )
    }

    /// `true` for the status that projects captured evidence.
    pub const fn projects_evidence(self) -> bool {
        matches!(self, Self::Qualified)
    }
}

/// Whether the surface exposes owner/origin chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OwnerOriginDisclosure {
    /// Owner and origin are disclosed in the pane chrome.
    OwnerOriginDisclosed,
    /// Owner and origin are hidden so the pane reads as ownerless. Always a
    /// blocker.
    OwnerOriginHidden,
}

impl M5OwnerOriginDisclosure {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerOriginDisclosed => "owner_origin_disclosed",
            Self::OwnerOriginHidden => "owner_origin_hidden",
        }
    }
}

/// Whether the surface exposes a freshness stamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FreshnessDisclosure {
    /// A freshness stamp for the embedded or provider content is shown.
    FreshnessShown,
    /// The freshness of the content is hidden. Always a blocker.
    FreshnessHidden,
}

impl M5FreshnessDisclosure {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshnessShown => "freshness_shown",
            Self::FreshnessHidden => "freshness_hidden",
        }
    }
}

/// Whether the surface stays bounded and attributed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustChrome {
    /// The surface is bounded and attributed as non-first-party.
    BoundedAttributed,
    /// The surface pretends to be a first-party local surface. Always a blocker.
    PretendsFirstParty,
}

impl M5TrustChrome {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundedAttributed => "bounded_attributed",
            Self::PretendsFirstParty => "pretends_first_party",
        }
    }
}

/// Whether claimed identity and provider auth default to the system browser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthChannel {
    /// Auth defaults to the system browser or an equally explicit native flow.
    SystemBrowserDefault,
    /// The embedded pane is the primary approval channel for auth. Always a
    /// blocker.
    EmbeddedPrimaryApproval,
}

impl M5AuthChannel {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemBrowserDefault => "system_browser_default",
            Self::EmbeddedPrimaryApproval => "embedded_primary_approval",
        }
    }
}

/// Whether high-risk embedded approvals are blocked or routed out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HighRiskHandling {
    /// High-risk or scope-widening approvals are blocked or routed to a governed
    /// native flow.
    BlockedOrRouted,
    /// A high-risk approval is hidden inside the embedded pane. Always a blocker.
    EmbeddedApprovalHidden,
}

impl M5HighRiskHandling {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockedOrRouted => "blocked_or_routed",
            Self::EmbeddedApprovalHidden => "embedded_approval_hidden",
        }
    }
}

/// Whether the return anchor resolves the exact in-product surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReturnAnchorOutcome {
    /// The return anchor resolves the exact in-product surface to come back to.
    ExactReturnResolved,
    /// The return anchor fails to resolve, stranding the user. Always a blocker.
    ReturnLost,
    /// The surface never leaves a governed in-product surface on this guarantee.
    NotApplicable,
}

impl M5ReturnAnchorOutcome {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactReturnResolved => "exact_return_resolved",
            Self::ReturnLost => "return_lost",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Whether the handoff reason is preserved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffReasonOutcome {
    /// The handoff reason is emitted and preserved in support/export artifacts.
    ReasonPreserved,
    /// The handoff happens with no preserved reason. Always a blocker.
    ReasonDropped,
}

impl M5HandoffReasonOutcome {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReasonPreserved => "reason_preserved",
            Self::ReasonDropped => "reason_dropped",
        }
    }
}

/// Whether support/export surfaces reuse the same destination descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportParity {
    /// Support bundles, docs/help, and release packets reuse the same
    /// destination descriptor.
    SameDescriptorReused,
    /// Support surfaces clone divergent status text instead of reusing the
    /// descriptor. Always a blocker.
    DivergentClone,
}

impl M5SupportParity {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SameDescriptorReused => "same_descriptor_reused",
            Self::DivergentClone => "divergent_clone",
        }
    }
}

/// Freshness of the captured evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceFreshness {
    /// The evidence is current.
    Fresh,
    /// The evidence is stale. A blocker on a marketed guarantee.
    Stale,
}

impl M5EvidenceFreshness {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
        }
    }
}

/// A boundary chrome element a surface exposes on its descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BoundaryChrome {
    /// An owner badge naming the owning party.
    OwnerBadge,
    /// An origin label naming where the content comes from.
    OriginLabel,
    /// A freshness stamp for the embedded or provider content.
    FreshnessStamp,
    /// A trust-boundary frame marking the surface as non-first-party.
    TrustBoundaryFrame,
    /// A return-anchor control that comes back to the governed surface.
    ReturnAnchorControl,
    /// A handoff-reason banner explaining why Aureline left the governed
    /// surface.
    HandoffReasonBanner,
}

impl M5BoundaryChrome {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerBadge => "owner_badge",
            Self::OriginLabel => "origin_label",
            Self::FreshnessStamp => "freshness_stamp",
            Self::TrustBoundaryFrame => "trust_boundary_frame",
            Self::ReturnAnchorControl => "return_anchor_control",
            Self::HandoffReasonBanner => "handoff_reason_banner",
        }
    }
}

/// A handoff target a surface can route to.
///
/// The target set is fixed: the M5 surfaces harden the handoff destinations
/// Aureline already claims and never expand the set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffTarget {
    /// A return to a governed in-product surface.
    InProductReturn,
    /// The system browser (the default for claimed-identity / provider auth).
    SystemBrowser,
    /// A vendor portal in the system browser.
    VendorPortal,
    /// A provider console in the system browser.
    ProviderConsole,
}

impl M5HandoffTarget {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InProductReturn => "in_product_return",
            Self::SystemBrowser => "system_browser",
            Self::VendorPortal => "vendor_portal",
            Self::ProviderConsole => "provider_console",
        }
    }
}

/// Lifecycle label retained on the canonical surface descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SurfaceLifecycle {
    /// Generally available.
    Stable,
    /// Beta lane; visibility and narrowing can change.
    Beta,
    /// Deprecated; surfaces must point at the replacement.
    Deprecated,
}

impl M5SurfaceLifecycle {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Deprecated => "deprecated",
        }
    }
}

/// Canonical descriptor for one M5 embedded surface's ownership, auth, and
/// handoff contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmbeddedSurfaceDescriptor {
    /// Stable surface id (e.g. `embedded:embedded_docs`).
    pub surface_id: String,
    /// Embedded surface the descriptor belongs to.
    pub embedded_surface: M5EmbeddedSurface,
    /// Descriptor revision the audit was produced against.
    pub descriptor_revision_ref: String,
    /// Canonical primary label ref.
    pub primary_label_ref: String,
    /// Canonical return-anchor ref the surface comes back to the governed
    /// in-product surface from.
    pub return_anchor_ref: String,
    /// Support note retained on the descriptor. MUST be non-empty.
    pub support_note: String,
    /// Declared boundary class.
    pub boundary_class: M5BoundaryClass,
    /// Pinned surface lifecycle label.
    pub lifecycle_label: M5SurfaceLifecycle,
    /// Boundary chrome the surface exposes, in canonical order.
    pub boundary_chrome: Vec<M5BoundaryChrome>,
    /// Handoff targets the surface routes to. MUST be non-empty for a marketed
    /// surface.
    pub handoff_targets: Vec<M5HandoffTarget>,
    /// `true` when the surface is marketed on desktop and therefore must pass
    /// the claimed matrix or narrow accordingly.
    pub marketed_on_desktop: bool,
    /// `true` once the surface rides the one governed embedded-boundary model and
    /// does not invent a feature-local rule. MUST be `true`.
    pub routed_through_governed_boundary: bool,
}

impl M5EmbeddedSurfaceDescriptor {
    /// `true` when this surface's boundary class makes it high-stakes for the
    /// audit.
    pub const fn is_high_stakes(&self) -> bool {
        self.boundary_class.is_high_stakes()
    }
}

/// Per-guarantee binding a surface reports for one embedded-boundary guarantee.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BoundaryBinding {
    /// Guarantee this binding covers.
    pub guarantee: M5BoundaryGuarantee,
    /// Aspect projected for the guarantee. MUST equal the guarantee's canonical
    /// aspect.
    pub aspect: M5BoundaryAspect,
    /// Qualification status the surface reports.
    pub qualification_status: M5BoundaryStatus,
    /// `true` when the surface is marketed on this guarantee.
    pub marketed_on_guarantee: bool,
    /// Captured destination-descriptor ref (`None` for non-qualified rows).
    pub projected_descriptor_ref: Option<String>,
    /// Captured boundary class (`None` for non-qualified rows).
    pub projected_boundary_class: Option<M5BoundaryClass>,
    /// Captured owner/origin disclosure (`None` for non-qualified rows).
    pub projected_owner_origin: Option<M5OwnerOriginDisclosure>,
    /// Captured freshness disclosure (`None` unless the guarantee requires it).
    pub projected_freshness: Option<M5FreshnessDisclosure>,
    /// Captured trust-chrome outcome (`None` unless the guarantee requires it).
    pub projected_trust_chrome: Option<M5TrustChrome>,
    /// Captured auth-channel outcome (`None` unless the guarantee requires it).
    pub projected_auth_channel: Option<M5AuthChannel>,
    /// Captured high-risk-handling outcome (`None` unless the guarantee requires
    /// it).
    pub projected_high_risk_handling: Option<M5HighRiskHandling>,
    /// Captured return-anchor outcome (`None` unless the guarantee requires it or
    /// the surface is high-stakes).
    pub projected_return_anchor: Option<M5ReturnAnchorOutcome>,
    /// Captured handoff-reason outcome (`None` unless the guarantee requires it).
    pub projected_handoff_reason: Option<M5HandoffReasonOutcome>,
    /// Captured support-parity outcome (`None` unless the guarantee requires it).
    pub projected_support_parity: Option<M5SupportParity>,
    /// Freshness of the captured evidence (`None` for non-qualified rows).
    pub evidence_freshness: Option<M5EvidenceFreshness>,
    /// Timestamp the evidence was captured (`None` for non-qualified rows).
    pub evidence_captured_at: Option<String>,
    /// Narrowing reason set when `qualification_status` requires one.
    pub narrowing_reason: Option<String>,
    /// Reviewer-facing free-form note retained on the row.
    pub note: Option<String>,
}

/// Blocking finding class the validator emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum M5BoundaryBlockingFinding {
    /// A surface paints its own boundary chrome through a feature-local rule
    /// outside the governed model.
    UnqualifiedLocalSurface {
        /// Surface that exposes the gap.
        surface_id: String,
        /// Guarantee that exposes the gap.
        guarantee: M5BoundaryGuarantee,
    },
    /// A marketed guarantee is claimed with no captured evidence.
    MissingEvidence {
        surface_id: String,
        guarantee: M5BoundaryGuarantee,
    },
    /// A qualified guarantee is missing its captured destination descriptor.
    MissingDescriptorRef {
        surface_id: String,
        guarantee: M5BoundaryGuarantee,
    },
    /// A guarantee hides the surface's owner/origin chrome.
    OwnerOriginHidden {
        surface_id: String,
        guarantee: M5BoundaryGuarantee,
    },
    /// A guarantee hides the freshness of the embedded or provider content.
    FreshnessHidden {
        surface_id: String,
        guarantee: M5BoundaryGuarantee,
    },
    /// A guarantee lets the surface pretend to be a first-party local surface.
    PretendsFirstParty {
        surface_id: String,
        guarantee: M5BoundaryGuarantee,
    },
    /// A guarantee makes the embedded pane the primary auth approval channel.
    EmbeddedPrimaryAuth {
        surface_id: String,
        guarantee: M5BoundaryGuarantee,
    },
    /// A guarantee hides a high-risk approval inside the embedded pane.
    EmbeddedHighRiskApproval {
        surface_id: String,
        guarantee: M5BoundaryGuarantee,
    },
    /// A guarantee loses the return anchor, stranding the user.
    ReturnAnchorLost {
        surface_id: String,
        guarantee: M5BoundaryGuarantee,
    },
    /// A guarantee hands off with no preserved reason.
    HandoffReasonDropped {
        surface_id: String,
        guarantee: M5BoundaryGuarantee,
    },
    /// A guarantee clones divergent support text instead of reusing the
    /// descriptor.
    SupportParityDivergent {
        surface_id: String,
        guarantee: M5BoundaryGuarantee,
    },
    /// A marketed guarantee carries stale evidence.
    StaleEvidenceOnMarketedRow {
        surface_id: String,
        guarantee: M5BoundaryGuarantee,
    },
    /// A binding projects an aspect that disagrees with the guarantee's
    /// canonical aspect.
    AspectDrift {
        surface_id: String,
        guarantee: M5BoundaryGuarantee,
        /// Projected aspect.
        projected_aspect: M5BoundaryAspect,
    },
    /// A non-qualified row is missing the `narrowing_reason`.
    MissingNarrowingReason {
        surface_id: String,
        guarantee: M5BoundaryGuarantee,
        qualification_status: M5BoundaryStatus,
    },
    /// A qualified row is missing a captured-evidence field it requires.
    MissingProjection {
        surface_id: String,
        guarantee: M5BoundaryGuarantee,
        /// Name of the missing projection field.
        field: String,
    },
    /// The descriptor carries no canonical return anchor.
    DescriptorMissingReturnAnchor { surface_id: String },
    /// The descriptor carries no support note.
    MissingSupportNote { surface_id: String },
    /// The surface paints its own boundary chrome outside the governed model.
    SurfaceNotOnGovernedBoundary { surface_id: String },
    /// A high-stakes surface exposes no boundary chrome.
    MissingBoundaryChrome { surface_id: String },
    /// A marketed surface declares no handoff target.
    NoDeclaredHandoffTarget { surface_id: String },
}

impl M5BoundaryBlockingFinding {
    /// Stable schema token for the finding class.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::UnqualifiedLocalSurface { .. } => "unqualified_local_surface",
            Self::MissingEvidence { .. } => "missing_evidence",
            Self::MissingDescriptorRef { .. } => "missing_descriptor_ref",
            Self::OwnerOriginHidden { .. } => "owner_origin_hidden",
            Self::FreshnessHidden { .. } => "freshness_hidden",
            Self::PretendsFirstParty { .. } => "pretends_first_party",
            Self::EmbeddedPrimaryAuth { .. } => "embedded_primary_auth",
            Self::EmbeddedHighRiskApproval { .. } => "embedded_high_risk_approval",
            Self::ReturnAnchorLost { .. } => "return_anchor_lost",
            Self::HandoffReasonDropped { .. } => "handoff_reason_dropped",
            Self::SupportParityDivergent { .. } => "support_parity_divergent",
            Self::StaleEvidenceOnMarketedRow { .. } => "stale_evidence_on_marketed_row",
            Self::AspectDrift { .. } => "aspect_drift",
            Self::MissingNarrowingReason { .. } => "missing_narrowing_reason",
            Self::MissingProjection { .. } => "missing_projection",
            Self::DescriptorMissingReturnAnchor { .. } => "descriptor_missing_return_anchor",
            Self::MissingSupportNote { .. } => "missing_support_note",
            Self::SurfaceNotOnGovernedBoundary { .. } => "surface_not_on_governed_boundary",
            Self::MissingBoundaryChrome { .. } => "missing_boundary_chrome",
            Self::NoDeclaredHandoffTarget { .. } => "no_declared_handoff_target",
        }
    }

    /// Returns the surface id this finding is attached to.
    pub fn surface_id(&self) -> &str {
        match self {
            Self::UnqualifiedLocalSurface { surface_id, .. }
            | Self::MissingEvidence { surface_id, .. }
            | Self::MissingDescriptorRef { surface_id, .. }
            | Self::OwnerOriginHidden { surface_id, .. }
            | Self::FreshnessHidden { surface_id, .. }
            | Self::PretendsFirstParty { surface_id, .. }
            | Self::EmbeddedPrimaryAuth { surface_id, .. }
            | Self::EmbeddedHighRiskApproval { surface_id, .. }
            | Self::ReturnAnchorLost { surface_id, .. }
            | Self::HandoffReasonDropped { surface_id, .. }
            | Self::SupportParityDivergent { surface_id, .. }
            | Self::StaleEvidenceOnMarketedRow { surface_id, .. }
            | Self::AspectDrift { surface_id, .. }
            | Self::MissingNarrowingReason { surface_id, .. }
            | Self::MissingProjection { surface_id, .. }
            | Self::DescriptorMissingReturnAnchor { surface_id }
            | Self::MissingSupportNote { surface_id }
            | Self::SurfaceNotOnGovernedBoundary { surface_id }
            | Self::MissingBoundaryChrome { surface_id }
            | Self::NoDeclaredHandoffTarget { surface_id } => surface_id,
        }
    }

    /// Returns the guarantee this finding is attached to, when guarantee-scoped.
    pub fn guarantee(&self) -> Option<M5BoundaryGuarantee> {
        match self {
            Self::UnqualifiedLocalSurface { guarantee, .. }
            | Self::MissingEvidence { guarantee, .. }
            | Self::MissingDescriptorRef { guarantee, .. }
            | Self::OwnerOriginHidden { guarantee, .. }
            | Self::FreshnessHidden { guarantee, .. }
            | Self::PretendsFirstParty { guarantee, .. }
            | Self::EmbeddedPrimaryAuth { guarantee, .. }
            | Self::EmbeddedHighRiskApproval { guarantee, .. }
            | Self::ReturnAnchorLost { guarantee, .. }
            | Self::HandoffReasonDropped { guarantee, .. }
            | Self::SupportParityDivergent { guarantee, .. }
            | Self::StaleEvidenceOnMarketedRow { guarantee, .. }
            | Self::AspectDrift { guarantee, .. }
            | Self::MissingNarrowingReason { guarantee, .. }
            | Self::MissingProjection { guarantee, .. } => Some(*guarantee),
            Self::DescriptorMissingReturnAnchor { .. }
            | Self::MissingSupportNote { .. }
            | Self::SurfaceNotOnGovernedBoundary { .. }
            | Self::MissingBoundaryChrome { .. }
            | Self::NoDeclaredHandoffTarget { .. } => None,
        }
    }
}

/// One per-surface embedded-boundary qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmbeddedBoundaryRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Canonical descriptor for the surface.
    pub descriptor: M5EmbeddedSurfaceDescriptor,
    /// Guarantee-by-guarantee qualification bindings, in canonical order.
    pub bindings: Vec<M5BoundaryBinding>,
    /// Blocking findings emitted against this row.
    pub blocking_findings: Vec<M5BoundaryBlockingFinding>,
    /// `true` when the surface's boundary class classifies it as high-stakes.
    pub high_stakes: bool,
    /// `true` when the surface is marketed on desktop.
    pub marketed: bool,
}

/// Per-class blocking-finding count summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BoundaryFindingSummary {
    /// Total blocking findings across the audit.
    pub total_blocking_findings: usize,
    /// Number of `unqualified_local_surface` findings.
    pub unqualified_local_surface: usize,
    /// Number of `missing_evidence` findings.
    pub missing_evidence: usize,
    /// Number of `missing_descriptor_ref` findings.
    pub missing_descriptor_ref: usize,
    /// Number of `owner_origin_hidden` findings.
    pub owner_origin_hidden: usize,
    /// Number of `freshness_hidden` findings.
    pub freshness_hidden: usize,
    /// Number of `pretends_first_party` findings.
    pub pretends_first_party: usize,
    /// Number of `embedded_primary_auth` findings.
    pub embedded_primary_auth: usize,
    /// Number of `embedded_high_risk_approval` findings.
    pub embedded_high_risk_approval: usize,
    /// Number of `return_anchor_lost` findings.
    pub return_anchor_lost: usize,
    /// Number of `handoff_reason_dropped` findings.
    pub handoff_reason_dropped: usize,
    /// Number of `support_parity_divergent` findings.
    pub support_parity_divergent: usize,
    /// Number of `stale_evidence_on_marketed_row` findings.
    pub stale_evidence_on_marketed_row: usize,
    /// Number of `aspect_drift` findings.
    pub aspect_drift: usize,
    /// Number of `missing_narrowing_reason` findings.
    pub missing_narrowing_reason: usize,
    /// Number of `missing_projection` findings.
    pub missing_projection: usize,
    /// Number of `descriptor_missing_return_anchor` findings.
    pub descriptor_missing_return_anchor: usize,
    /// Number of `missing_support_note` findings.
    pub missing_support_note: usize,
    /// Number of `surface_not_on_governed_boundary` findings.
    pub surface_not_on_governed_boundary: usize,
    /// Number of `missing_boundary_chrome` findings.
    pub missing_boundary_chrome: usize,
    /// Number of `no_declared_handoff_target` findings.
    pub no_declared_handoff_target: usize,
}

impl M5BoundaryFindingSummary {
    fn empty() -> Self {
        Self {
            total_blocking_findings: 0,
            unqualified_local_surface: 0,
            missing_evidence: 0,
            missing_descriptor_ref: 0,
            owner_origin_hidden: 0,
            freshness_hidden: 0,
            pretends_first_party: 0,
            embedded_primary_auth: 0,
            embedded_high_risk_approval: 0,
            return_anchor_lost: 0,
            handoff_reason_dropped: 0,
            support_parity_divergent: 0,
            stale_evidence_on_marketed_row: 0,
            aspect_drift: 0,
            missing_narrowing_reason: 0,
            missing_projection: 0,
            descriptor_missing_return_anchor: 0,
            missing_support_note: 0,
            surface_not_on_governed_boundary: 0,
            missing_boundary_chrome: 0,
            no_declared_handoff_target: 0,
        }
    }

    fn record(&mut self, finding: &M5BoundaryBlockingFinding) {
        self.total_blocking_findings += 1;
        match finding {
            M5BoundaryBlockingFinding::UnqualifiedLocalSurface { .. } => {
                self.unqualified_local_surface += 1
            }
            M5BoundaryBlockingFinding::MissingEvidence { .. } => self.missing_evidence += 1,
            M5BoundaryBlockingFinding::MissingDescriptorRef { .. } => {
                self.missing_descriptor_ref += 1
            }
            M5BoundaryBlockingFinding::OwnerOriginHidden { .. } => self.owner_origin_hidden += 1,
            M5BoundaryBlockingFinding::FreshnessHidden { .. } => self.freshness_hidden += 1,
            M5BoundaryBlockingFinding::PretendsFirstParty { .. } => self.pretends_first_party += 1,
            M5BoundaryBlockingFinding::EmbeddedPrimaryAuth { .. } => {
                self.embedded_primary_auth += 1
            }
            M5BoundaryBlockingFinding::EmbeddedHighRiskApproval { .. } => {
                self.embedded_high_risk_approval += 1
            }
            M5BoundaryBlockingFinding::ReturnAnchorLost { .. } => self.return_anchor_lost += 1,
            M5BoundaryBlockingFinding::HandoffReasonDropped { .. } => {
                self.handoff_reason_dropped += 1
            }
            M5BoundaryBlockingFinding::SupportParityDivergent { .. } => {
                self.support_parity_divergent += 1
            }
            M5BoundaryBlockingFinding::StaleEvidenceOnMarketedRow { .. } => {
                self.stale_evidence_on_marketed_row += 1
            }
            M5BoundaryBlockingFinding::AspectDrift { .. } => self.aspect_drift += 1,
            M5BoundaryBlockingFinding::MissingNarrowingReason { .. } => {
                self.missing_narrowing_reason += 1
            }
            M5BoundaryBlockingFinding::MissingProjection { .. } => self.missing_projection += 1,
            M5BoundaryBlockingFinding::DescriptorMissingReturnAnchor { .. } => {
                self.descriptor_missing_return_anchor += 1
            }
            M5BoundaryBlockingFinding::MissingSupportNote { .. } => self.missing_support_note += 1,
            M5BoundaryBlockingFinding::SurfaceNotOnGovernedBoundary { .. } => {
                self.surface_not_on_governed_boundary += 1
            }
            M5BoundaryBlockingFinding::MissingBoundaryChrome { .. } => {
                self.missing_boundary_chrome += 1
            }
            M5BoundaryBlockingFinding::NoDeclaredHandoffTarget { .. } => {
                self.no_declared_handoff_target += 1
            }
        }
    }
}

/// Per-guarantee coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BoundaryCoverageSummary {
    /// Guarantee this summary covers.
    pub guarantee: M5BoundaryGuarantee,
    /// Number of `qualified` rows on this guarantee.
    pub qualified_rows: usize,
    /// Number of `explicitly_narrowed` rows on this guarantee.
    pub explicitly_narrowed_rows: usize,
    /// Number of `not_applicable` rows on this guarantee.
    pub not_applicable_rows: usize,
    /// Number of `platform_omitted` rows on this guarantee.
    pub platform_omitted_rows: usize,
    /// Number of `declared_capture_gap` rows on this guarantee.
    pub declared_capture_gap_rows: usize,
    /// Number of `unqualified_local_surface` rows on this guarantee.
    pub unqualified_local_surface_rows: usize,
    /// Number of `missing_evidence` rows on this guarantee.
    pub missing_evidence_rows: usize,
}

impl M5BoundaryCoverageSummary {
    fn narrowed_rows(&self) -> usize {
        self.explicitly_narrowed_rows
            + self.not_applicable_rows
            + self.platform_omitted_rows
            + self.declared_capture_gap_rows
    }
}

/// A single return-anchor index entry the audit publishes so the embedded
/// chrome, docs, and release surfaces can return each surface by its anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReturnAnchorEntry {
    /// Embedded surface the anchor belongs to.
    pub embedded_surface: M5EmbeddedSurface,
    /// Surface id the anchor returns.
    pub surface_id: String,
    /// Canonical return-anchor ref.
    pub return_anchor_ref: String,
}

/// One marketed guarantee release tooling should narrow because its evidence is
/// stale or red.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NarrowableRow {
    /// Surface id that must narrow.
    pub surface_id: String,
    /// Guarantee that must narrow.
    pub guarantee: M5BoundaryGuarantee,
    /// Stable reason the row is narrowable.
    pub reason: String,
}

/// M5 embedded-boundary ownership, auth, and handoff qualification audit report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmbeddedBoundaryReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable report id quoted across surfaces.
    pub report_id: String,
    /// Source schema ref for the canonical contract.
    pub source_schema_ref: String,
    /// Required boundary guarantees, in canonical order.
    pub required_guarantees: Vec<M5BoundaryGuarantee>,
    /// Per-surface qualification rows, sorted by `descriptor.surface_id`.
    pub rows: Vec<M5EmbeddedBoundaryRow>,
    /// Per-guarantee coverage summary, in canonical order.
    pub guarantee_coverage: Vec<M5BoundaryCoverageSummary>,
    /// Per-class blocking-finding summary.
    pub findings_summary: M5BoundaryFindingSummary,
    /// Canonical return-anchor index, sorted by surface id.
    pub return_anchor_index: Vec<M5ReturnAnchorEntry>,
    /// Number of registered M5 surfaces present.
    pub registered_surface_count: usize,
    /// Number of high-stakes surfaces present.
    pub high_stakes_surface_count: usize,
    /// Number of surfaces marketed on desktop.
    pub marketed_surface_count: usize,
    /// Total boundary guarantees checked.
    pub boundary_guarantees_checked: usize,
    /// Marketed rows release tooling should narrow because their evidence is
    /// stale or red.
    pub narrowable_marketed_rows: Vec<M5NarrowableRow>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Markdown publication ref this audit is rendered to.
    pub published_report_ref: String,
    /// Companion doc publication ref.
    pub published_doc_ref: String,
    /// Docs/help refs the audit can be reopened from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the audit can be reopened from.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the audit was generated.
    pub generated_at: String,
}

impl M5EmbeddedBoundaryReport {
    /// Returns `true` when every required guarantee is qualified by at least one
    /// surface.
    pub fn every_required_guarantee_qualified(&self) -> bool {
        for guarantee in M5BoundaryGuarantee::required_guarantees() {
            let any_qualified = self.rows.iter().any(|surface| {
                surface.bindings.iter().any(|binding| {
                    binding.guarantee == guarantee
                        && binding.qualification_status == M5BoundaryStatus::Qualified
                })
            });
            if !any_qualified {
                return false;
            }
        }
        true
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "audit: surfaces={}, high_stakes={}, marketed={}, guarantees={}, blocking={}, clean={}",
            self.registered_surface_count,
            self.high_stakes_surface_count,
            self.marketed_surface_count,
            self.boundary_guarantees_checked,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        for coverage in &self.guarantee_coverage {
            lines.push(format!(
                "{}: qualified={}, narrowed={}, unqualified={}, missing_evidence={}",
                coverage.guarantee.display_label(),
                coverage.qualified_rows,
                coverage.narrowed_rows(),
                coverage.unqualified_local_surface_rows,
                coverage.missing_evidence_rows,
            ));
        }
        for surface in &self.rows {
            for finding in &surface.blocking_findings {
                lines.push(format!(
                    "blocker: {} -- {} -- {}",
                    finding.class_token(),
                    finding.surface_id(),
                    finding
                        .guarantee()
                        .map(M5BoundaryGuarantee::as_str)
                        .unwrap_or("surface"),
                ));
            }
        }
        for narrowable in &self.narrowable_marketed_rows {
            lines.push(format!(
                "narrowable: {} -- {} -- {}",
                narrowable.surface_id,
                narrowable.guarantee.as_str(),
                narrowable.reason,
            ));
        }
        lines
    }

    /// Renders the markdown audit artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 embedded-boundary owner/origin, auth, and handoff qualification audit\n",
        );
        out.push('\n');
        out.push_str(
            "Generated from the seeded audit in\n\
             [`crate::m5_embedded_boundaries`](../../../../crates/aureline-shell/src/m5_embedded_boundaries/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_embedded_boundaries -- report-md > \\\n  artifacts/ux/m5/embedded-boundary-audits/m5_embedded_boundaries_audit.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Registered M5 surfaces: `{}`\n",
            self.registered_surface_count
        ));
        out.push_str(&format!(
            "- High-stakes surfaces: `{}`\n",
            self.high_stakes_surface_count
        ));
        out.push_str(&format!(
            "- Marketed surfaces: `{}`\n",
            self.marketed_surface_count
        ));
        out.push_str(&format!(
            "- Boundary guarantees checked: `{}`\n",
            self.boundary_guarantees_checked
        ));
        out.push_str(&format!(
            "- Blocking findings: `{}`\n",
            self.findings_summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Narrowable marketed rows: `{}`\n",
            self.narrowable_marketed_rows.len()
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Per-guarantee coverage\n\n");
        out.push_str(
            "| Boundary guarantee | Qualified | Narrowed | Unqualified | Missing evidence |\n\
             | ------------------ | --------: | -------: | ----------: | ---------------: |\n",
        );
        for coverage in &self.guarantee_coverage {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                coverage.guarantee.display_label(),
                coverage.qualified_rows,
                coverage.narrowed_rows(),
                coverage.unqualified_local_surface_rows,
                coverage.missing_evidence_rows,
            ));
        }
        out.push('\n');

        out.push_str("## Findings summary\n\n");
        out.push_str("| Class | Count |\n| ----- | ----: |\n");
        out.push_str(&format!(
            "| `unqualified_local_surface` | {} |\n",
            self.findings_summary.unqualified_local_surface
        ));
        out.push_str(&format!(
            "| `missing_evidence` | {} |\n",
            self.findings_summary.missing_evidence
        ));
        out.push_str(&format!(
            "| `missing_descriptor_ref` | {} |\n",
            self.findings_summary.missing_descriptor_ref
        ));
        out.push_str(&format!(
            "| `owner_origin_hidden` | {} |\n",
            self.findings_summary.owner_origin_hidden
        ));
        out.push_str(&format!(
            "| `freshness_hidden` | {} |\n",
            self.findings_summary.freshness_hidden
        ));
        out.push_str(&format!(
            "| `pretends_first_party` | {} |\n",
            self.findings_summary.pretends_first_party
        ));
        out.push_str(&format!(
            "| `embedded_primary_auth` | {} |\n",
            self.findings_summary.embedded_primary_auth
        ));
        out.push_str(&format!(
            "| `embedded_high_risk_approval` | {} |\n",
            self.findings_summary.embedded_high_risk_approval
        ));
        out.push_str(&format!(
            "| `return_anchor_lost` | {} |\n",
            self.findings_summary.return_anchor_lost
        ));
        out.push_str(&format!(
            "| `handoff_reason_dropped` | {} |\n",
            self.findings_summary.handoff_reason_dropped
        ));
        out.push_str(&format!(
            "| `support_parity_divergent` | {} |\n",
            self.findings_summary.support_parity_divergent
        ));
        out.push_str(&format!(
            "| `stale_evidence_on_marketed_row` | {} |\n",
            self.findings_summary.stale_evidence_on_marketed_row
        ));
        out.push_str(&format!(
            "| `aspect_drift` | {} |\n",
            self.findings_summary.aspect_drift
        ));
        out.push_str(&format!(
            "| `missing_narrowing_reason` | {} |\n",
            self.findings_summary.missing_narrowing_reason
        ));
        out.push_str(&format!(
            "| `missing_projection` | {} |\n",
            self.findings_summary.missing_projection
        ));
        out.push_str(&format!(
            "| `descriptor_missing_return_anchor` | {} |\n",
            self.findings_summary.descriptor_missing_return_anchor
        ));
        out.push_str(&format!(
            "| `missing_support_note` | {} |\n",
            self.findings_summary.missing_support_note
        ));
        out.push_str(&format!(
            "| `surface_not_on_governed_boundary` | {} |\n",
            self.findings_summary.surface_not_on_governed_boundary
        ));
        out.push_str(&format!(
            "| `missing_boundary_chrome` | {} |\n",
            self.findings_summary.missing_boundary_chrome
        ));
        out.push_str(&format!(
            "| `no_declared_handoff_target` | {} |\n\n",
            self.findings_summary.no_declared_handoff_target
        ));

        out.push_str("## Return anchor index\n\n");
        out.push_str(
            "| Embedded surface | Surface id | Return anchor |\n| ---------------- | ---------- | ------------- |\n",
        );
        for entry in &self.return_anchor_index {
            out.push_str(&format!(
                "| {} | `{}` | `{}` |\n",
                entry.embedded_surface.display_label(),
                entry.surface_id,
                entry.return_anchor_ref,
            ));
        }
        out.push('\n');

        out.push_str("## Per-surface rows\n\n");
        for surface in &self.rows {
            out.push_str(&format!(
                "### `{}` ({}, {}, {})\n\n",
                surface.descriptor.surface_id,
                surface.descriptor.embedded_surface.as_str(),
                surface.descriptor.boundary_class.as_str(),
                surface.descriptor.lifecycle_label.as_str()
            ));
            out.push_str(&format!(
                "- Descriptor revision: `{}`\n",
                surface.descriptor.descriptor_revision_ref
            ));
            out.push_str(&format!(
                "- Boundary class: `{}`\n",
                surface.descriptor.boundary_class.as_str()
            ));
            out.push_str(&format!(
                "- Return anchor: `{}`\n",
                surface.descriptor.return_anchor_ref
            ));
            out.push_str(&format!(
                "- Boundary chrome: {}\n",
                if surface.descriptor.boundary_chrome.is_empty() {
                    "none".to_owned()
                } else {
                    surface
                        .descriptor
                        .boundary_chrome
                        .iter()
                        .map(|chrome| format!("`{}`", chrome.as_str()))
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            ));
            out.push_str(&format!(
                "- Handoff targets: {}\n",
                if surface.descriptor.handoff_targets.is_empty() {
                    "none".to_owned()
                } else {
                    surface
                        .descriptor
                        .handoff_targets
                        .iter()
                        .map(|target| format!("`{}`", target.as_str()))
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            ));
            out.push_str(&format!(
                "- Marketed on desktop: `{}`\n",
                if surface.marketed { "yes" } else { "no" }
            ));
            out.push_str(&format!(
                "- High-stakes: `{}`\n\n",
                if surface.high_stakes { "yes" } else { "no" }
            ));

            out.push_str(
                "| Boundary guarantee | Status | Owner/origin | Freshness | Trust | Auth | High-risk | Return | Handoff | Support | Freshness ev. | Narrowing reason |\n\
                 | ------------------ | ------ | ------------ | --------- | ----- | ---- | --------- | ------ | ------- | ------- | ------------- | ---------------- |\n",
            );
            for binding in &surface.bindings {
                let owner_origin = binding
                    .projected_owner_origin
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let freshness = binding
                    .projected_freshness
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let trust = binding
                    .projected_trust_chrome
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let auth = binding
                    .projected_auth_channel
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let high_risk = binding
                    .projected_high_risk_handling
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let return_anchor = binding
                    .projected_return_anchor
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let handoff = binding
                    .projected_handoff_reason
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let support = binding
                    .projected_support_parity
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let freshness_ev = binding
                    .evidence_freshness
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let narrowing = binding.narrowing_reason.as_deref().unwrap_or("-");
                out.push_str(&format!(
                    "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                    binding.guarantee.display_label(),
                    binding.qualification_status.as_str(),
                    owner_origin,
                    freshness,
                    trust,
                    auth,
                    high_risk,
                    return_anchor,
                    handoff,
                    support,
                    freshness_ev,
                    narrowing,
                ));
            }
            out.push('\n');

            if surface.blocking_findings.is_empty() {
                out.push_str("Findings: none.\n\n");
            } else {
                out.push_str("Findings:\n\n");
                for finding in &surface.blocking_findings {
                    out.push_str(&format!(
                        "- `{}` on `{}`\n",
                        finding.class_token(),
                        finding
                            .guarantee()
                            .map(M5BoundaryGuarantee::as_str)
                            .unwrap_or("surface"),
                    ));
                }
                out.push('\n');
            }
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_embedded_boundaries -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_embedded_boundaries_fixtures\n");
        out.push_str("python3 tools/ci/m5/embedded_boundaries_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the M5 embedded-boundary audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmbeddedBoundarySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Audit report quoted in full.
    pub report: M5EmbeddedBoundaryReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl M5EmbeddedBoundarySupportExport {
    /// Builds the support-export wrapper for an audit report.
    pub fn from_report(
        support_export_id: impl Into<String>,
        report: M5EmbeddedBoundaryReport,
    ) -> Self {
        let mut case_ids = vec![report.report_id.clone()];
        for surface in &report.rows {
            case_ids.push(surface.descriptor.surface_id.clone());
            case_ids.push(surface.descriptor.descriptor_revision_ref.clone());
        }
        Self {
            record_kind: M5_EMBEDDED_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_EMBEDDED_SCHEMA_VERSION,
            shared_contract_ref: M5_EMBEDDED_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Computes the per-surface blocking findings from a descriptor and its
/// guarantee bindings.
fn compute_surface_findings(
    descriptor: &M5EmbeddedSurfaceDescriptor,
    bindings: &[M5BoundaryBinding],
    high_stakes: bool,
) -> Vec<M5BoundaryBlockingFinding> {
    let mut findings = Vec::new();

    // Descriptor-level (surface-scoped) findings.
    if descriptor.return_anchor_ref.trim().is_empty() {
        findings.push(M5BoundaryBlockingFinding::DescriptorMissingReturnAnchor {
            surface_id: descriptor.surface_id.clone(),
        });
    }
    if descriptor.support_note.trim().is_empty() {
        findings.push(M5BoundaryBlockingFinding::MissingSupportNote {
            surface_id: descriptor.surface_id.clone(),
        });
    }
    if !descriptor.routed_through_governed_boundary {
        findings.push(M5BoundaryBlockingFinding::SurfaceNotOnGovernedBoundary {
            surface_id: descriptor.surface_id.clone(),
        });
    }
    if high_stakes && descriptor.boundary_chrome.is_empty() {
        findings.push(M5BoundaryBlockingFinding::MissingBoundaryChrome {
            surface_id: descriptor.surface_id.clone(),
        });
    }
    if descriptor.marketed_on_desktop && descriptor.handoff_targets.is_empty() {
        findings.push(M5BoundaryBlockingFinding::NoDeclaredHandoffTarget {
            surface_id: descriptor.surface_id.clone(),
        });
    }

    for binding in bindings {
        let guarantee = binding.guarantee;
        let surface_id = descriptor.surface_id.clone();

        // A binding's aspect must match its guarantee's canonical aspect.
        if binding.aspect != guarantee.canonical_aspect() {
            findings.push(M5BoundaryBlockingFinding::AspectDrift {
                surface_id: surface_id.clone(),
                guarantee,
                projected_aspect: binding.aspect,
            });
        }

        match binding.qualification_status {
            M5BoundaryStatus::UnqualifiedLocalSurface => {
                findings.push(M5BoundaryBlockingFinding::UnqualifiedLocalSurface {
                    surface_id: surface_id.clone(),
                    guarantee,
                });
            }
            M5BoundaryStatus::MissingEvidence => {
                findings.push(M5BoundaryBlockingFinding::MissingEvidence {
                    surface_id: surface_id.clone(),
                    guarantee,
                });
            }
            M5BoundaryStatus::Qualified => {
                compute_qualified_findings(binding, high_stakes, &surface_id, &mut findings);
            }
            status if status.requires_narrowing_reason() => {
                let reason_ok = binding
                    .narrowing_reason
                    .as_deref()
                    .map(str::trim)
                    .map(str::is_empty)
                    == Some(false);
                if !reason_ok {
                    findings.push(M5BoundaryBlockingFinding::MissingNarrowingReason {
                        surface_id: surface_id.clone(),
                        guarantee,
                        qualification_status: status,
                    });
                }
            }
            _ => {}
        }
    }
    findings
}

/// Computes the blocking findings for one qualified embedded-boundary binding.
fn compute_qualified_findings(
    binding: &M5BoundaryBinding,
    high_stakes: bool,
    surface_id: &str,
    findings: &mut Vec<M5BoundaryBlockingFinding>,
) {
    let guarantee = binding.guarantee;

    // Required captured-evidence projections (universal for qualified rows).
    if binding.projected_descriptor_ref.is_none() {
        findings.push(M5BoundaryBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            guarantee,
            field: "projected_descriptor_ref".to_owned(),
        });
    }
    if binding.projected_boundary_class.is_none() {
        findings.push(M5BoundaryBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            guarantee,
            field: "projected_boundary_class".to_owned(),
        });
    }
    if binding.projected_owner_origin.is_none() {
        findings.push(M5BoundaryBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            guarantee,
            field: "projected_owner_origin".to_owned(),
        });
    }
    if binding.evidence_freshness.is_none() {
        findings.push(M5BoundaryBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            guarantee,
            field: "evidence_freshness".to_owned(),
        });
    }

    // Guarantee-specific required projections.
    if guarantee.requires_freshness_disclosure() && binding.projected_freshness.is_none() {
        findings.push(M5BoundaryBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            guarantee,
            field: "projected_freshness".to_owned(),
        });
    }
    if guarantee.requires_trust_chrome() && binding.projected_trust_chrome.is_none() {
        findings.push(M5BoundaryBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            guarantee,
            field: "projected_trust_chrome".to_owned(),
        });
    }
    if guarantee.requires_auth_channel() && binding.projected_auth_channel.is_none() {
        findings.push(M5BoundaryBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            guarantee,
            field: "projected_auth_channel".to_owned(),
        });
    }
    if guarantee.requires_high_risk_handling() && binding.projected_high_risk_handling.is_none() {
        findings.push(M5BoundaryBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            guarantee,
            field: "projected_high_risk_handling".to_owned(),
        });
    }
    if guarantee.requires_return_anchor() && binding.projected_return_anchor.is_none() {
        findings.push(M5BoundaryBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            guarantee,
            field: "projected_return_anchor".to_owned(),
        });
    }
    if guarantee.requires_handoff_reason() && binding.projected_handoff_reason.is_none() {
        findings.push(M5BoundaryBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            guarantee,
            field: "projected_handoff_reason".to_owned(),
        });
    }
    if guarantee.requires_support_parity() && binding.projected_support_parity.is_none() {
        findings.push(M5BoundaryBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            guarantee,
            field: "projected_support_parity".to_owned(),
        });
    }
    if high_stakes && binding.projected_return_anchor.is_none() {
        findings.push(M5BoundaryBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            guarantee,
            field: "projected_return_anchor".to_owned(),
        });
    }

    // Red captured results.
    if binding.projected_descriptor_ref.is_none() {
        findings.push(M5BoundaryBlockingFinding::MissingDescriptorRef {
            surface_id: surface_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_owner_origin == Some(M5OwnerOriginDisclosure::OwnerOriginHidden) {
        findings.push(M5BoundaryBlockingFinding::OwnerOriginHidden {
            surface_id: surface_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_freshness == Some(M5FreshnessDisclosure::FreshnessHidden) {
        findings.push(M5BoundaryBlockingFinding::FreshnessHidden {
            surface_id: surface_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_trust_chrome == Some(M5TrustChrome::PretendsFirstParty) {
        findings.push(M5BoundaryBlockingFinding::PretendsFirstParty {
            surface_id: surface_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_auth_channel == Some(M5AuthChannel::EmbeddedPrimaryApproval) {
        findings.push(M5BoundaryBlockingFinding::EmbeddedPrimaryAuth {
            surface_id: surface_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_high_risk_handling == Some(M5HighRiskHandling::EmbeddedApprovalHidden) {
        findings.push(M5BoundaryBlockingFinding::EmbeddedHighRiskApproval {
            surface_id: surface_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_return_anchor == Some(M5ReturnAnchorOutcome::ReturnLost) {
        findings.push(M5BoundaryBlockingFinding::ReturnAnchorLost {
            surface_id: surface_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_handoff_reason == Some(M5HandoffReasonOutcome::ReasonDropped) {
        findings.push(M5BoundaryBlockingFinding::HandoffReasonDropped {
            surface_id: surface_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_support_parity == Some(M5SupportParity::DivergentClone) {
        findings.push(M5BoundaryBlockingFinding::SupportParityDivergent {
            surface_id: surface_id.to_owned(),
            guarantee,
        });
    }
    if binding.marketed_on_guarantee
        && binding.evidence_freshness == Some(M5EvidenceFreshness::Stale)
    {
        findings.push(M5BoundaryBlockingFinding::StaleEvidenceOnMarketedRow {
            surface_id: surface_id.to_owned(),
            guarantee,
        });
    }
}

/// Computes the per-guarantee coverage and per-class finding summary.
fn summarize_report(
    surfaces: &[M5EmbeddedBoundaryRow],
) -> (Vec<M5BoundaryCoverageSummary>, M5BoundaryFindingSummary) {
    let mut coverage: Vec<M5BoundaryCoverageSummary> = M5BoundaryGuarantee::required_guarantees()
        .iter()
        .map(|guarantee| M5BoundaryCoverageSummary {
            guarantee: *guarantee,
            qualified_rows: 0,
            explicitly_narrowed_rows: 0,
            not_applicable_rows: 0,
            platform_omitted_rows: 0,
            declared_capture_gap_rows: 0,
            unqualified_local_surface_rows: 0,
            missing_evidence_rows: 0,
        })
        .collect();
    let mut summary = M5BoundaryFindingSummary::empty();

    for surface in surfaces {
        for binding in &surface.bindings {
            if let Some(coverage_row) = coverage
                .iter_mut()
                .find(|row| row.guarantee == binding.guarantee)
            {
                match binding.qualification_status {
                    M5BoundaryStatus::Qualified => coverage_row.qualified_rows += 1,
                    M5BoundaryStatus::ExplicitlyNarrowed => {
                        coverage_row.explicitly_narrowed_rows += 1
                    }
                    M5BoundaryStatus::NotApplicable => coverage_row.not_applicable_rows += 1,
                    M5BoundaryStatus::PlatformOmitted => coverage_row.platform_omitted_rows += 1,
                    M5BoundaryStatus::DeclaredCaptureGap => {
                        coverage_row.declared_capture_gap_rows += 1
                    }
                    M5BoundaryStatus::UnqualifiedLocalSurface => {
                        coverage_row.unqualified_local_surface_rows += 1
                    }
                    M5BoundaryStatus::MissingEvidence => coverage_row.missing_evidence_rows += 1,
                }
            }
        }
        for finding in &surface.blocking_findings {
            summary.record(finding);
        }
    }

    (coverage, summary)
}

/// Computes the marketed rows release tooling should narrow because their
/// evidence is stale or red.
fn compute_narrowable_rows(surfaces: &[M5EmbeddedBoundaryRow]) -> Vec<M5NarrowableRow> {
    let mut narrowable = Vec::new();
    for surface in surfaces {
        if !surface.marketed {
            continue;
        }
        for finding in &surface.blocking_findings {
            if let Some(guarantee) = finding.guarantee() {
                narrowable.push(M5NarrowableRow {
                    surface_id: surface.descriptor.surface_id.clone(),
                    guarantee,
                    reason: format!("blocking_finding:{}", finding.class_token()),
                });
            }
        }
    }
    narrowable
}

/// Builds an [`M5EmbeddedBoundaryRow`] from a descriptor and its guarantee
/// bindings, computing the per-surface blocking findings.
pub fn build_m5_embedded_boundary_row(
    descriptor: M5EmbeddedSurfaceDescriptor,
    bindings: Vec<M5BoundaryBinding>,
) -> M5EmbeddedBoundaryRow {
    let high_stakes = descriptor.is_high_stakes();
    let marketed = descriptor.marketed_on_desktop;
    let blocking_findings = compute_surface_findings(&descriptor, &bindings, high_stakes);

    M5EmbeddedBoundaryRow {
        record_kind: M5_EMBEDDED_ROW_RECORD_KIND.to_owned(),
        schema_version: M5_EMBEDDED_SCHEMA_VERSION,
        shared_contract_ref: M5_EMBEDDED_SHARED_CONTRACT_REF.to_owned(),
        descriptor,
        bindings,
        blocking_findings,
        high_stakes,
        marketed,
    }
}

/// Builds a full [`M5EmbeddedBoundaryReport`] from per-surface rows.
pub fn build_m5_embedded_boundaries_audit(
    surfaces: Vec<M5EmbeddedBoundaryRow>,
) -> M5EmbeddedBoundaryReport {
    let mut surfaces = surfaces;
    surfaces.sort_by(|left, right| left.descriptor.surface_id.cmp(&right.descriptor.surface_id));

    let registered_surface_count = surfaces.len();
    let high_stakes_surface_count = surfaces.iter().filter(|row| row.high_stakes).count();
    let marketed_surface_count = surfaces.iter().filter(|row| row.marketed).count();
    let boundary_guarantees_checked = surfaces.iter().map(|row| row.bindings.len()).sum::<usize>();

    let (guarantee_coverage, findings_summary) = summarize_report(&surfaces);
    let narrowable_marketed_rows = compute_narrowable_rows(&surfaces);
    let report_clean = findings_summary.total_blocking_findings == 0;

    let mut return_anchor_index: Vec<M5ReturnAnchorEntry> = surfaces
        .iter()
        .map(|surface| M5ReturnAnchorEntry {
            embedded_surface: surface.descriptor.embedded_surface,
            surface_id: surface.descriptor.surface_id.clone(),
            return_anchor_ref: surface.descriptor.return_anchor_ref.clone(),
        })
        .collect();
    return_anchor_index.sort_by(|left, right| left.surface_id.cmp(&right.surface_id));

    M5EmbeddedBoundaryReport {
        record_kind: M5_EMBEDDED_REPORT_RECORD_KIND.to_owned(),
        schema_version: M5_EMBEDDED_SCHEMA_VERSION,
        shared_contract_ref: M5_EMBEDDED_SHARED_CONTRACT_REF.to_owned(),
        report_id: M5_EMBEDDED_REPORT_ID.to_owned(),
        source_schema_ref: M5_EMBEDDED_SOURCE_SCHEMA_REF.to_owned(),
        required_guarantees: M5BoundaryGuarantee::required_guarantees().to_vec(),
        rows: surfaces,
        guarantee_coverage,
        findings_summary,
        return_anchor_index,
        registered_surface_count,
        high_stakes_surface_count,
        marketed_surface_count,
        boundary_guarantees_checked,
        narrowable_marketed_rows,
        report_clean,
        published_report_ref: M5_EMBEDDED_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: M5_EMBEDDED_PUBLISHED_DOC_REF.to_owned(),
        docs_help_refs: vec![
            M5_EMBEDDED_PUBLISHED_DOC_REF.to_owned(),
            "docs/m5/first_useful_work.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-embedded-boundaries".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_m5_embedded_boundaries`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum M5EmbeddedValidationError {
    /// The audit has no registered surfaces.
    NoRegisteredSurfaces,
    /// A required boundary guarantee has no qualified surface.
    RequiredGuaranteeNotQualified { guarantee: String },
    /// A surface is missing a required guarantee from its binding set.
    MissingRequiredGuarantee {
        surface_id: String,
        guarantee: String,
    },
    /// A blocking finding remains on the surface.
    BlockingFindingPresent {
        surface_id: String,
        guarantee: String,
        class: String,
    },
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
    /// A surface's descriptor revision ref is empty.
    MissingDescriptorRevisionRef { surface_id: String },
}

/// Validates an audit report against the M5 embedded-boundary acceptance
/// invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_m5_embedded_boundaries(
    report: &M5EmbeddedBoundaryReport,
) -> Result<(), Vec<M5EmbeddedValidationError>> {
    let mut errors = Vec::new();

    if report.rows.is_empty() {
        errors.push(M5EmbeddedValidationError::NoRegisteredSurfaces);
    }

    for guarantee in M5BoundaryGuarantee::required_guarantees() {
        let any_qualified = report.rows.iter().any(|surface| {
            surface.bindings.iter().any(|binding| {
                binding.guarantee == guarantee
                    && binding.qualification_status == M5BoundaryStatus::Qualified
            })
        });
        if !any_qualified {
            errors.push(M5EmbeddedValidationError::RequiredGuaranteeNotQualified {
                guarantee: guarantee.as_str().to_owned(),
            });
        }
    }

    for surface in &report.rows {
        for guarantee in M5BoundaryGuarantee::required_guarantees() {
            if !surface
                .bindings
                .iter()
                .any(|binding| binding.guarantee == guarantee)
            {
                errors.push(M5EmbeddedValidationError::MissingRequiredGuarantee {
                    surface_id: surface.descriptor.surface_id.clone(),
                    guarantee: guarantee.as_str().to_owned(),
                });
            }
        }
        if surface.descriptor.descriptor_revision_ref.trim().is_empty() {
            errors.push(M5EmbeddedValidationError::MissingDescriptorRevisionRef {
                surface_id: surface.descriptor.surface_id.clone(),
            });
        }
        for finding in &surface.blocking_findings {
            errors.push(M5EmbeddedValidationError::BlockingFindingPresent {
                surface_id: finding.surface_id().to_owned(),
                guarantee: finding
                    .guarantee()
                    .map(|guarantee| guarantee.as_str().to_owned())
                    .unwrap_or_else(|| "surface".to_owned()),
                class: finding.class_token().to_owned(),
            });
        }
    }

    if report.published_report_ref.trim().is_empty() {
        errors.push(M5EmbeddedValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(M5EmbeddedValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Seed row used by [`seeded_m5_embedded_boundaries_audit`].
struct SurfaceSeed {
    surface_id: &'static str,
    embedded_surface: M5EmbeddedSurface,
    descriptor_revision_ref: &'static str,
    primary_label_ref: &'static str,
    return_anchor_ref: &'static str,
    support_note: &'static str,
    boundary_class: M5BoundaryClass,
    lifecycle_label: M5SurfaceLifecycle,
    boundary_chrome: &'static [M5BoundaryChrome],
    handoff_targets: &'static [M5HandoffTarget],
    return_anchor_outcome: M5ReturnAnchorOutcome,
    bindings: &'static [BindingSeed],
}

struct BindingSeed {
    guarantee: M5BoundaryGuarantee,
    qualification_status: M5BoundaryStatus,
    narrowing_reason: Option<&'static str>,
    note: Option<&'static str>,
}

/// Helper: a qualified guarantee with captured evidence.
const fn qualified(guarantee: M5BoundaryGuarantee) -> BindingSeed {
    BindingSeed {
        guarantee,
        qualification_status: M5BoundaryStatus::Qualified,
        narrowing_reason: None,
        note: None,
    }
}

/// Helper: an honestly-declared capture gap with a documented reason.
const fn declared_capture_gap(guarantee: M5BoundaryGuarantee, reason: &'static str) -> BindingSeed {
    BindingSeed {
        guarantee,
        qualification_status: M5BoundaryStatus::DeclaredCaptureGap,
        narrowing_reason: Some(reason),
        note: None,
    }
}

/// Helper: a not-applicable guarantee with a documented reason.
const fn not_applicable(guarantee: M5BoundaryGuarantee, reason: &'static str) -> BindingSeed {
    BindingSeed {
        guarantee,
        qualification_status: M5BoundaryStatus::NotApplicable,
        narrowing_reason: Some(reason),
        note: None,
    }
}

use M5BoundaryChrome::{
    FreshnessStamp, HandoffReasonBanner, OriginLabel, OwnerBadge, ReturnAnchorControl,
    TrustBoundaryFrame,
};
use M5BoundaryGuarantee::{
    FreshnessDisclosure, HandoffReasonPreserved, NoEmbeddedHighRiskApproval, OwnerOriginDisclosure,
    ReturnAnchorPresent, SupportExportParity, SystemBrowserAuthDefault, TrustBoundaryChrome,
};
use M5HandoffTarget::{InProductReturn, ProviderConsole, SystemBrowser, VendorPortal};

const FULL_CHROME: &[M5BoundaryChrome] = &[
    OwnerBadge,
    OriginLabel,
    FreshnessStamp,
    TrustBoundaryFrame,
    ReturnAnchorControl,
    HandoffReasonBanner,
];

const LOCAL_CHROME: &[M5BoundaryChrome] =
    &[OwnerBadge, OriginLabel, FreshnessStamp, ReturnAnchorControl];

const LOCAL_TARGETS: &[M5HandoffTarget] = &[InProductReturn, SystemBrowser];

const PROVIDER_TARGETS: &[M5HandoffTarget] = &[InProductReturn, SystemBrowser, VendorPortal];

const CONSOLE_TARGETS: &[M5HandoffTarget] = &[
    InProductReturn,
    SystemBrowser,
    ProviderConsole,
    VendorPortal,
];

/// Binding set for an auth-capable surface: all eight guarantees qualified.
const FULL_BINDINGS: &[BindingSeed] = &[
    qualified(OwnerOriginDisclosure),
    qualified(FreshnessDisclosure),
    qualified(TrustBoundaryChrome),
    qualified(SystemBrowserAuthDefault),
    qualified(NoEmbeddedHighRiskApproval),
    qualified(ReturnAnchorPresent),
    qualified(HandoffReasonPreserved),
    qualified(SupportExportParity),
];

/// Binding set for a content surface that never authenticates and exposes no
/// mutating approval: the two auth guarantees are not applicable.
const CONTENT_BINDINGS: &[BindingSeed] = &[
    qualified(OwnerOriginDisclosure),
    qualified(FreshnessDisclosure),
    qualified(TrustBoundaryChrome),
    not_applicable(
        SystemBrowserAuthDefault,
        "this_surface_renders_content_only_and_never_authenticates_so_there_is_no_auth_channel_to_default",
    ),
    not_applicable(
        NoEmbeddedHighRiskApproval,
        "this_surface_exposes_no_mutating_approval_so_there_is_no_high_risk_embedded_channel_to_block",
    ),
    qualified(ReturnAnchorPresent),
    qualified(HandoffReasonPreserved),
    qualified(SupportExportParity),
];

/// Binding set for the provider-console handoff: the freshness of the external
/// console is declared at handoff rather than continuously polled.
const CONSOLE_BINDINGS: &[BindingSeed] = &[
    qualified(OwnerOriginDisclosure),
    declared_capture_gap(
        FreshnessDisclosure,
        "the_provider_console_is_external_so_its_freshness_is_declared_at_handoff_not_continuously_polled",
    ),
    qualified(TrustBoundaryChrome),
    qualified(SystemBrowserAuthDefault),
    qualified(NoEmbeddedHighRiskApproval),
    qualified(ReturnAnchorPresent),
    qualified(HandoffReasonPreserved),
    qualified(SupportExportParity),
];

const SURFACE_SEEDS: &[SurfaceSeed] = &[
    // Embedded docs viewer. First-party-local; renders Aureline help content.
    SurfaceSeed {
        surface_id: "embedded:embedded_docs",
        embedded_surface: M5EmbeddedSurface::EmbeddedDocs,
        descriptor_revision_ref: "embedded-rev:embedded_docs:2026.06.01-01",
        primary_label_ref: "label:embedded.embedded_docs:primary",
        return_anchor_ref: "embedded:return:embedded_docs",
        support_note: "The embedded docs viewer discloses owner and origin, stamps content freshness, stays bounded as non-first-party, and returns to the exact in-product doc anchor; it renders local help and never authenticates.",
        boundary_class: M5BoundaryClass::FirstPartyLocal,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        boundary_chrome: LOCAL_CHROME,
        handoff_targets: LOCAL_TARGETS,
        return_anchor_outcome: M5ReturnAnchorOutcome::ExactReturnResolved,
        bindings: CONTENT_BINDINGS,
    },
    // Request/runtime viewer. Embedded-webview; renders external request/response.
    SurfaceSeed {
        surface_id: "embedded:request_runtime_viewer",
        embedded_surface: M5EmbeddedSurface::RequestRuntimeViewer,
        descriptor_revision_ref: "embedded-rev:request_runtime_viewer:2026.06.01-01",
        primary_label_ref: "label:embedded.request_runtime_viewer:primary",
        return_anchor_ref: "embedded:return:request_runtime_viewer",
        support_note: "The request/runtime viewer is a bounded embedded webview: it discloses owner and origin, stamps response freshness, never pretends to be first-party, and returns to the exact request it was serving.",
        boundary_class: M5BoundaryClass::EmbeddedWebview,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        boundary_chrome: FULL_CHROME,
        handoff_targets: LOCAL_TARGETS,
        return_anchor_outcome: M5ReturnAnchorOutcome::ExactReturnResolved,
        bindings: CONTENT_BINDINGS,
    },
    // Live preview-route pane. Embedded-webview; renders the served route.
    SurfaceSeed {
        surface_id: "embedded:preview_route_pane",
        embedded_surface: M5EmbeddedSurface::PreviewRoutePane,
        descriptor_revision_ref: "embedded-rev:preview_route_pane:2026.06.01-01",
        primary_label_ref: "label:embedded.preview_route_pane:primary",
        return_anchor_ref: "embedded:return:preview_route_pane",
        support_note: "The live preview-route pane is a bounded embedded webview: it discloses owner and origin, stamps the served-route freshness, stays attributed as non-first-party, and returns to the exact route and scope it was serving.",
        boundary_class: M5BoundaryClass::EmbeddedWebview,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        boundary_chrome: FULL_CHROME,
        handoff_targets: LOCAL_TARGETS,
        return_anchor_outcome: M5ReturnAnchorOutcome::ExactReturnResolved,
        bindings: CONTENT_BINDINGS,
    },
    // Help-center pane. First-party-local; renders Aureline help surfaces.
    SurfaceSeed {
        surface_id: "embedded:help_center_pane",
        embedded_surface: M5EmbeddedSurface::HelpCenterPane,
        descriptor_revision_ref: "embedded-rev:help_center_pane:2026.06.01-01",
        primary_label_ref: "label:embedded.help_center_pane:primary",
        return_anchor_ref: "embedded:return:help_center_pane",
        support_note: "The help-center pane discloses owner and origin, stamps content freshness, stays bounded as non-first-party, and returns to the exact help anchor; it renders local help and never authenticates.",
        boundary_class: M5BoundaryClass::FirstPartyLocal,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        boundary_chrome: LOCAL_CHROME,
        handoff_targets: LOCAL_TARGETS,
        return_anchor_outcome: M5ReturnAnchorOutcome::ExactReturnResolved,
        bindings: CONTENT_BINDINGS,
    },
    // Marketplace/account surface. Provider-owned; high-stakes (claimed identity).
    SurfaceSeed {
        surface_id: "embedded:marketplace_account",
        embedded_surface: M5EmbeddedSurface::MarketplaceAccount,
        descriptor_revision_ref: "embedded-rev:marketplace_account:2026.06.01-01",
        primary_label_ref: "label:embedded.marketplace_account:primary",
        return_anchor_ref: "embedded:return:marketplace_account",
        support_note: "The marketplace/account surface is provider-owned and high-stakes: it discloses owner and origin, defaults claimed-identity and provider auth to the system browser, blocks high-risk embedded approvals, and returns to the exact account anchor with a preserved handoff reason.",
        boundary_class: M5BoundaryClass::ProviderOwned,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        boundary_chrome: FULL_CHROME,
        handoff_targets: PROVIDER_TARGETS,
        return_anchor_outcome: M5ReturnAnchorOutcome::ExactReturnResolved,
        bindings: FULL_BINDINGS,
    },
    // Review/provider pane. Provider-owned; high-stakes (provider mutation).
    SurfaceSeed {
        surface_id: "embedded:provider_review_pane",
        embedded_surface: M5EmbeddedSurface::ProviderReviewPane,
        descriptor_revision_ref: "embedded-rev:provider_review_pane:2026.06.01-01",
        primary_label_ref: "label:embedded.provider_review_pane:primary",
        return_anchor_ref: "embedded:return:provider_review_pane",
        support_note: "The review/provider pane is provider-owned and high-stakes: it discloses owner and origin, routes high-risk and scope-widening approvals out of the embedded pane to a governed native flow, and returns to the exact review anchor with a preserved handoff reason.",
        boundary_class: M5BoundaryClass::ProviderOwned,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        boundary_chrome: FULL_CHROME,
        handoff_targets: PROVIDER_TARGETS,
        return_anchor_outcome: M5ReturnAnchorOutcome::ExactReturnResolved,
        bindings: FULL_BINDINGS,
    },
    // Companion/browser handoff. External-handoff; high-stakes (leaves product).
    SurfaceSeed {
        surface_id: "embedded:companion_browser_handoff",
        embedded_surface: M5EmbeddedSurface::CompanionBrowserHandoff,
        descriptor_revision_ref: "embedded-rev:companion_browser_handoff:2026.06.01-01",
        primary_label_ref: "label:embedded.companion_browser_handoff:primary",
        return_anchor_ref: "embedded:return:companion_browser_handoff",
        support_note: "The companion/browser handoff is an explicit external handoff and high-stakes: it discloses where it sends the user, defaults auth to the system browser, emits and preserves a handoff reason, and returns to the exact in-product surface.",
        boundary_class: M5BoundaryClass::ExternalHandoff,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        boundary_chrome: FULL_CHROME,
        handoff_targets: LOCAL_TARGETS,
        return_anchor_outcome: M5ReturnAnchorOutcome::ExactReturnResolved,
        bindings: FULL_BINDINGS,
    },
    // Provider-console handoff. External-handoff; high-stakes (vendor portal).
    SurfaceSeed {
        surface_id: "embedded:provider_console_handoff",
        embedded_surface: M5EmbeddedSurface::ProviderConsoleHandoff,
        descriptor_revision_ref: "embedded-rev:provider_console_handoff:2026.06.01-01",
        primary_label_ref: "label:embedded.provider_console_handoff:primary",
        return_anchor_ref: "embedded:return:provider_console_handoff",
        support_note: "The provider-console handoff is an explicit external handoff and high-stakes: it discloses the destination owner, defaults auth to the system browser, emits and preserves a handoff reason, returns to the exact in-product surface, and declares its external-freshness gap honestly.",
        boundary_class: M5BoundaryClass::ExternalHandoff,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        boundary_chrome: FULL_CHROME,
        handoff_targets: CONSOLE_TARGETS,
        return_anchor_outcome: M5ReturnAnchorOutcome::ExactReturnResolved,
        bindings: CONSOLE_BINDINGS,
    },
];

fn build_binding_from_seed(seed: &SurfaceSeed, binding_seed: &BindingSeed) -> M5BoundaryBinding {
    let guarantee = binding_seed.guarantee;
    let qualified = binding_seed.qualification_status.projects_evidence();
    let high_stakes = seed.boundary_class.is_high_stakes();
    let marketed_on_guarantee = !matches!(
        binding_seed.qualification_status,
        M5BoundaryStatus::NotApplicable | M5BoundaryStatus::PlatformOmitted
    );

    M5BoundaryBinding {
        guarantee,
        aspect: guarantee.canonical_aspect(),
        qualification_status: binding_seed.qualification_status,
        marketed_on_guarantee,
        projected_descriptor_ref: qualified.then(|| {
            format!(
                "destination-descriptor:{}:{}",
                seed.surface_id,
                guarantee.as_str()
            )
        }),
        projected_boundary_class: qualified.then_some(seed.boundary_class),
        projected_owner_origin: qualified.then_some(M5OwnerOriginDisclosure::OwnerOriginDisclosed),
        projected_freshness: (qualified && guarantee.requires_freshness_disclosure())
            .then_some(M5FreshnessDisclosure::FreshnessShown),
        projected_trust_chrome: (qualified && guarantee.requires_trust_chrome())
            .then_some(M5TrustChrome::BoundedAttributed),
        projected_auth_channel: (qualified && guarantee.requires_auth_channel())
            .then_some(M5AuthChannel::SystemBrowserDefault),
        projected_high_risk_handling: (qualified && guarantee.requires_high_risk_handling())
            .then_some(M5HighRiskHandling::BlockedOrRouted),
        projected_return_anchor: (qualified && (guarantee.requires_return_anchor() || high_stakes))
            .then_some(seed.return_anchor_outcome),
        projected_handoff_reason: (qualified && guarantee.requires_handoff_reason())
            .then_some(M5HandoffReasonOutcome::ReasonPreserved),
        projected_support_parity: (qualified && guarantee.requires_support_parity())
            .then_some(M5SupportParity::SameDescriptorReused),
        evidence_freshness: qualified.then_some(M5EvidenceFreshness::Fresh),
        evidence_captured_at: qualified.then(|| GENERATED_AT.to_owned()),
        narrowing_reason: binding_seed.narrowing_reason.map(str::to_owned),
        note: binding_seed.note.map(str::to_owned),
    }
}

fn build_surface_from_seed(seed: &SurfaceSeed) -> M5EmbeddedBoundaryRow {
    let descriptor = M5EmbeddedSurfaceDescriptor {
        surface_id: seed.surface_id.to_owned(),
        embedded_surface: seed.embedded_surface,
        descriptor_revision_ref: seed.descriptor_revision_ref.to_owned(),
        primary_label_ref: seed.primary_label_ref.to_owned(),
        return_anchor_ref: seed.return_anchor_ref.to_owned(),
        support_note: seed.support_note.to_owned(),
        boundary_class: seed.boundary_class,
        lifecycle_label: seed.lifecycle_label,
        boundary_chrome: seed.boundary_chrome.to_vec(),
        handoff_targets: seed.handoff_targets.to_vec(),
        marketed_on_desktop: true,
        routed_through_governed_boundary: true,
    };
    let bindings: Vec<M5BoundaryBinding> = seed
        .bindings
        .iter()
        .map(|binding_seed| build_binding_from_seed(seed, binding_seed))
        .collect();
    build_m5_embedded_boundary_row(descriptor, bindings)
}

/// Seeded audit builder used by the headless inspector and the integration
/// test. The seed mirrors the JSON fixtures checked in under
/// `fixtures/ux/m5/webview-auth-handoff/`.
pub fn seeded_m5_embedded_boundaries_audit() -> M5EmbeddedBoundaryReport {
    let surfaces = SURFACE_SEEDS.iter().map(build_surface_from_seed).collect();
    build_m5_embedded_boundaries_audit(surfaces)
}
