//! Canonical M5 anti-abuse ranking, continuity-history, and repro-export board for
//! local-to-published, mirrored, and sideload-to-registry ecosystem flows.
//!
//! Where [`crate::m5_author_and_publish_preview`] is the publish-control gate an author
//! drives *before* release and [`crate::m5_marketplace_fact_views`] projects install-time
//! truth, this module freezes the **transparency board** that keeps a package's ranking
//! reasons, anti-abuse posture, quarantine/removal history, publisher continuity-or-loss,
//! and reproducible export packet visible *after* a family moves from a local workspace to
//! a published, mirrored, or registry-bound identity. Each [`AntiAbuseReproRow`] reuses the
//! shared [`ArtifactFamily`], [`RuntimeClass`], [`HostAbiClass`], [`WorkspaceOrigin`],
//! [`SignatureState`], and [`TrustPosture`] vocabulary so the board, the publish gate, and
//! the marketplace describe one artifact instead of a parallel synonym set.
//!
//! The board is a render-truth object, not a moderation-only console. From the observed
//! facts each row recomputes:
//!
//! - the **rendered trust posture** the surface may display — capped by the signing state,
//!   the workspace origin, *and* the registry-binding decision, so a locally-built,
//!   side-loaded, or freshly bound artifact never inherits a verified-publisher or
//!   enterprise-approved badge just because it was built on a trusted machine;
//! - the **ranking explainability** — [`RankingExplainability::TrustLed`],
//!   [`RankingExplainability::AntiAbuseLed`], or the flagged
//!   [`RankingExplainability::VanityDominated`] — so install-count, star-rating, and
//!   trending vanity metrics can be shown but never dominate the decision, and an
//!   anti-abuse demotion always leads the ranking;
//! - the **quarantine-history state** and **publisher-continuity state** folded from a
//!   sequenced [`HistoryEvent`] timeline, so a prior quarantine, a publisher transfer, or
//!   a verified-publisher loss stays disclosed on the visible surface rather than hidden
//!   in a moderation tool; and
//! - the **transparency disposition** — visible-clean, visible-with-history-disclosure, or
//!   withheld-quarantined — recomputed from those states.
//!
//! Two flow invariants make the row more than a status painter. The **repro-export packet**
//! ([`ReproExport`]) must carry the package id, content digest, host ABI, redacted logs,
//! conformance results, and manifest refs needed to reproduce a build *without* raw
//! supervisor traces or a paid service — its [`ReproExportState`] is recomputed from those
//! refs. And **local-to-published rebinding** must pass through an explicit
//! bind-published-identity review: a [`BindPublishedIdentity::BindReviewRequired`] or
//! [`BindPublishedIdentity::BoundPublishedIdentity`] row must carry its review ref, and a
//! local or side-loaded origin can never claim [`BindPublishedIdentity::BoundPublishedIdentity`]
//! without that review having moved it to a published origin first.
//!
//! [`M5AntiAbuseReproBoard::validate`] enforces every recomputation, and
//! [`M5AntiAbuseReproBoard::cross_check_matrix`] proves no row renders a stronger badge than
//! the publish-preview gate would grant the same family, so marketplace discovery,
//! authoring surfaces, diagnostics, support exports, and release packets project one trust
//! truth.
//!
//! The packet is checked in at
//! `artifacts/ecosystem/m5/m5-anti-abuse-and-repro.json` and embedded here, so this typed
//! consumer and any CI gate agree on every row without a cargo build in CI. The model is
//! metadata-only: every field is a typed state or an opaque ref. It carries no credential
//! bodies, raw provider payloads, signing secrets, raw supervisor traces, or source bodies
//! — the `source_path_ref`, `logs_ref`, and history `detail_ref`s are opaque, redacted
//! refs, never verbatim paths or log bodies.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use crate::m5_author_and_publish_preview::{
    ArtifactFamily, HostAbiClass, M5AuthorPublishMatrix, RuntimeClass, SignatureState, TrustPosture,
};
pub use crate::m5_workspace_strip::WorkspaceOrigin;

/// Supported M5 anti-abuse-and-repro board schema version.
pub const M5_ANTI_ABUSE_REPRO_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_ANTI_ABUSE_REPRO_RECORD_KIND: &str = "m5_anti_abuse_and_repro_board";

/// Repo-relative path to the checked-in packet.
pub const M5_ANTI_ABUSE_REPRO_PATH: &str = "artifacts/ecosystem/m5/m5-anti-abuse-and-repro.json";

/// Embedded checked-in packet JSON.
pub const M5_ANTI_ABUSE_REPRO_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/ecosystem/m5/m5-anti-abuse-and-repro.json"
));

/// Category a ranking-reason chip belongs to.
///
/// The category separates substantive trust and quality signals from anti-abuse
/// demotions and vanity metrics, so the board can prove vanity metrics never dominate a
/// ranking decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RankingReasonCategory {
    /// A substantive trust signal (conformance, security, publisher, maintenance).
    TrustSignal,
    /// A substantive quality signal (compatibility, documentation).
    QualitySignal,
    /// An anti-abuse demotion that lowers ranking.
    AntiAbuseDemotion,
    /// A popularity/vanity metric that must never dominate ranking.
    VanityMetric,
}

impl RankingReasonCategory {
    /// Every ranking-reason category, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::TrustSignal,
        Self::QualitySignal,
        Self::AntiAbuseDemotion,
        Self::VanityMetric,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustSignal => "trust_signal",
            Self::QualitySignal => "quality_signal",
            Self::AntiAbuseDemotion => "anti_abuse_demotion",
            Self::VanityMetric => "vanity_metric",
        }
    }
}

/// A closed ranking/anti-abuse reason chip a discovery or authoring surface shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RankingReasonChip {
    /// Conformance kit verified against the native suite.
    ConformanceVerified,
    /// A security review passed.
    SecurityReviewPassed,
    /// The publisher identity is verified.
    PublisherVerified,
    /// The package is actively maintained.
    MaintainedActively,
    /// Compatibility floor is current for supported hosts.
    CompatibilityCurrent,
    /// Documentation coverage is complete.
    DocsComplete,
    /// Anti-abuse rate-limited the package's surface.
    AntiAbuseRateLimited,
    /// Anti-abuse demoted the package's ranking.
    AntiAbuseRankingDemoted,
    /// Anti-abuse quarantined the package.
    AntiAbuseQuarantined,
    /// Install-count popularity (a vanity metric).
    InstallCountPopularity,
    /// Star-rating popularity (a vanity metric).
    StarRatingPopularity,
    /// Trending velocity (a vanity metric).
    TrendingVelocity,
}

impl RankingReasonChip {
    /// Every ranking-reason chip, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ConformanceVerified,
        Self::SecurityReviewPassed,
        Self::PublisherVerified,
        Self::MaintainedActively,
        Self::CompatibilityCurrent,
        Self::DocsComplete,
        Self::AntiAbuseRateLimited,
        Self::AntiAbuseRankingDemoted,
        Self::AntiAbuseQuarantined,
        Self::InstallCountPopularity,
        Self::StarRatingPopularity,
        Self::TrendingVelocity,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConformanceVerified => "conformance_verified",
            Self::SecurityReviewPassed => "security_review_passed",
            Self::PublisherVerified => "publisher_verified",
            Self::MaintainedActively => "maintained_actively",
            Self::CompatibilityCurrent => "compatibility_current",
            Self::DocsComplete => "docs_complete",
            Self::AntiAbuseRateLimited => "anti_abuse_rate_limited",
            Self::AntiAbuseRankingDemoted => "anti_abuse_ranking_demoted",
            Self::AntiAbuseQuarantined => "anti_abuse_quarantined",
            Self::InstallCountPopularity => "install_count_popularity",
            Self::StarRatingPopularity => "star_rating_popularity",
            Self::TrendingVelocity => "trending_velocity",
        }
    }

    /// Canonical declaration rank used to order chips deterministically.
    pub const fn rank(self) -> u8 {
        match self {
            Self::ConformanceVerified => 0,
            Self::SecurityReviewPassed => 1,
            Self::PublisherVerified => 2,
            Self::MaintainedActively => 3,
            Self::CompatibilityCurrent => 4,
            Self::DocsComplete => 5,
            Self::AntiAbuseRateLimited => 6,
            Self::AntiAbuseRankingDemoted => 7,
            Self::AntiAbuseQuarantined => 8,
            Self::InstallCountPopularity => 9,
            Self::StarRatingPopularity => 10,
            Self::TrendingVelocity => 11,
        }
    }

    /// Category of this chip.
    pub const fn category(self) -> RankingReasonCategory {
        match self {
            Self::ConformanceVerified
            | Self::SecurityReviewPassed
            | Self::PublisherVerified
            | Self::MaintainedActively => RankingReasonCategory::TrustSignal,
            Self::CompatibilityCurrent | Self::DocsComplete => RankingReasonCategory::QualitySignal,
            Self::AntiAbuseRateLimited
            | Self::AntiAbuseRankingDemoted
            | Self::AntiAbuseQuarantined => RankingReasonCategory::AntiAbuseDemotion,
            Self::InstallCountPopularity | Self::StarRatingPopularity | Self::TrendingVelocity => {
                RankingReasonCategory::VanityMetric
            }
        }
    }

    /// Whether this chip is a substantive trust or quality signal.
    pub const fn is_substantive(self) -> bool {
        matches!(
            self.category(),
            RankingReasonCategory::TrustSignal | RankingReasonCategory::QualitySignal
        )
    }

    /// Whether this chip is an anti-abuse demotion.
    pub const fn is_anti_abuse_demotion(self) -> bool {
        matches!(self.category(), RankingReasonCategory::AntiAbuseDemotion)
    }

    /// Whether this chip is a vanity/popularity metric.
    pub const fn is_vanity_metric(self) -> bool {
        matches!(self.category(), RankingReasonCategory::VanityMetric)
    }
}

/// What is leading a package's ranking once the chips are weighed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RankingExplainability {
    /// Ranking is led by substantive trust/quality signals.
    TrustLed,
    /// Ranking is led by an anti-abuse demotion (a demotion is always surfaced first).
    AntiAbuseLed,
    /// Vanity metrics outnumber substantive signals — a flagged, non-publishable state.
    VanityDominated,
}

impl RankingExplainability {
    /// Every ranking-explainability state, in declaration order.
    pub const ALL: [Self; 3] = [Self::TrustLed, Self::AntiAbuseLed, Self::VanityDominated];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustLed => "trust_led",
            Self::AntiAbuseLed => "anti_abuse_led",
            Self::VanityDominated => "vanity_dominated",
        }
    }
}

/// A closed history-event kind in a package's continuity timeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryEventKind {
    /// The package was first published.
    Published,
    /// A new version was published.
    VersionPublished,
    /// The publisher identity was transferred.
    PublisherTransferred,
    /// A verified-publisher badge was granted.
    VerifiedPublisherGranted,
    /// A verified-publisher badge was lost.
    VerifiedPublisherLost,
    /// The package was quarantined.
    Quarantined,
    /// A quarantine was cleared.
    QuarantineCleared,
    /// The package was removed from the registry.
    Removed,
    /// A removed package was reinstated.
    Reinstated,
}

impl HistoryEventKind {
    /// Every history-event kind, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Published,
        Self::VersionPublished,
        Self::PublisherTransferred,
        Self::VerifiedPublisherGranted,
        Self::VerifiedPublisherLost,
        Self::Quarantined,
        Self::QuarantineCleared,
        Self::Removed,
        Self::Reinstated,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::VersionPublished => "version_published",
            Self::PublisherTransferred => "publisher_transferred",
            Self::VerifiedPublisherGranted => "verified_publisher_granted",
            Self::VerifiedPublisherLost => "verified_publisher_lost",
            Self::Quarantined => "quarantined",
            Self::QuarantineCleared => "quarantine_cleared",
            Self::Removed => "removed",
            Self::Reinstated => "reinstated",
        }
    }
}

/// Folded quarantine/removal history state of a package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineHistoryState {
    /// No quarantine or removal action has ever been taken.
    Clean,
    /// A prior quarantine or removal is disclosed, but the package is currently available.
    PriorActionDisclosed,
    /// The package is currently withheld (quarantined or removed).
    CurrentlyWithheld,
}

impl QuarantineHistoryState {
    /// Every quarantine-history state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::Clean,
        Self::PriorActionDisclosed,
        Self::CurrentlyWithheld,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::PriorActionDisclosed => "prior_action_disclosed",
            Self::CurrentlyWithheld => "currently_withheld",
        }
    }

    /// Whether the package is currently withheld.
    pub const fn is_currently_withheld(self) -> bool {
        matches!(self, Self::CurrentlyWithheld)
    }
}

/// Folded publisher continuity-or-loss state of a package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublisherContinuityState {
    /// The publisher identity has been continuous.
    Continuous,
    /// A publisher transfer is disclosed.
    PublisherTransferredDisclosed,
    /// A verified-publisher badge was lost and not re-granted.
    VerifiedPublisherLost,
}

impl PublisherContinuityState {
    /// Every publisher-continuity state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::Continuous,
        Self::PublisherTransferredDisclosed,
        Self::VerifiedPublisherLost,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Continuous => "continuous",
            Self::PublisherTransferredDisclosed => "publisher_transferred_disclosed",
            Self::VerifiedPublisherLost => "verified_publisher_lost",
        }
    }

    /// Whether this state must be disclosed on the visible surface.
    pub const fn is_disclosable(self) -> bool {
        matches!(
            self,
            Self::PublisherTransferredDisclosed | Self::VerifiedPublisherLost
        )
    }
}

/// Completeness of a package's reproducible export packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReproExportState {
    /// The export carries logs, conformance results, and a manifest ref.
    Complete,
    /// The export is missing one of the reproducible content refs; the gap is disclosed.
    Incomplete,
}

impl ReproExportState {
    /// Every repro-export state, in declaration order.
    pub const ALL: [Self; 2] = [Self::Complete, Self::Incomplete];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Incomplete => "incomplete",
        }
    }
}

/// The registry-binding decision attached to a package's local-to-published flow.
///
/// The binding caps the [`TrustPosture`] a row may render: a still-local or
/// pending-review package renders no inherited badge, and a package only just bound to a
/// published identity caps at [`TrustPosture::RegistryBound`] until a verified-publisher
/// badge is independently granted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BindPublishedIdentity {
    /// The package is already published; no local-to-published rebind is in play.
    NotApplicablePublished,
    /// The package stays local; it is never bound to a published identity.
    StayLocal,
    /// A local-to-published rebind is pending an explicit bind-published-identity review.
    BindReviewRequired,
    /// The rebind passed review and is bound to a published registry identity.
    BoundPublishedIdentity,
}

impl BindPublishedIdentity {
    /// Every bind decision, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NotApplicablePublished,
        Self::StayLocal,
        Self::BindReviewRequired,
        Self::BoundPublishedIdentity,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicablePublished => "not_applicable_published",
            Self::StayLocal => "stay_local",
            Self::BindReviewRequired => "bind_review_required",
            Self::BoundPublishedIdentity => "bound_published_identity",
        }
    }

    /// Highest trust posture this binding lets a row render.
    ///
    /// A still-local or pending-review binding caps at [`TrustPosture::UnsignedLocalOnly`];
    /// a freshly bound identity caps at [`TrustPosture::RegistryBound`]; an
    /// already-published package leaves the cap to the signing state and origin.
    pub const fn trust_ceiling(self) -> TrustPosture {
        match self {
            Self::StayLocal | Self::BindReviewRequired => TrustPosture::UnsignedLocalOnly,
            Self::BoundPublishedIdentity => TrustPosture::RegistryBound,
            Self::NotApplicablePublished => TrustPosture::EnterpriseApproved,
        }
    }

    /// Whether this binding requires a bind-published-identity review ref.
    pub const fn requires_review_ref(self) -> bool {
        matches!(
            self,
            Self::BindReviewRequired | Self::BoundPublishedIdentity
        )
    }
}

/// The visibility disposition a discovery or authoring surface renders for a package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransparencyDisposition {
    /// Visible with no adverse history to disclose.
    VisibleClean,
    /// Visible, with a prior action, publisher transfer, or verified-publisher loss disclosed.
    VisibleWithHistoryDisclosure,
    /// Withheld because the package is currently quarantined or removed.
    WithheldQuarantined,
}

impl TransparencyDisposition {
    /// Every transparency disposition, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::VisibleClean,
        Self::VisibleWithHistoryDisclosure,
        Self::WithheldQuarantined,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VisibleClean => "visible_clean",
            Self::VisibleWithHistoryDisclosure => "visible_with_history_disclosure",
            Self::WithheldQuarantined => "withheld_quarantined",
        }
    }
}

/// One sequenced event in a package's continuity timeline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryEvent {
    /// Strictly increasing sequence index within the row's timeline.
    pub sequence: u32,
    /// Event kind.
    pub kind: HistoryEventKind,
    /// Calendar date the event occurred.
    pub at: String,
    /// Opaque, redacted ref to the action record (never a raw moderation note body).
    pub detail_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

/// A reproducible export packet for a package build.
///
/// The packet is self-contained by contract: it carries the package id, content digest,
/// host ABI, redacted logs, conformance results, and manifest refs needed to reproduce a
/// build without raw supervisor traces or a paid service.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReproExport {
    /// Stable package id the export reproduces.
    pub package_id: String,
    /// Content digest of the reproduced build.
    pub digest: String,
    /// Host ABI the export reproduces against; must equal the row's host ABI.
    pub host_abi: HostAbiClass,
    /// Opaque, redacted ref to the build logs (empty when unavailable).
    pub logs_ref: String,
    /// Opaque ref to the conformance results (empty when unavailable).
    pub conformance_results_ref: String,
    /// Opaque ref to the build manifest (empty when unavailable).
    pub manifest_ref: String,
    /// Whether the export is self-contained: no raw supervisor traces, no paid service.
    pub self_contained: bool,
    /// Completeness state; must equal the recomputed state.
    pub state: ReproExportState,
}

impl ReproExport {
    /// The completeness state recomputed from the export's content refs.
    pub fn computed_state(&self) -> ReproExportState {
        if !self.logs_ref.trim().is_empty()
            && !self.conformance_results_ref.trim().is_empty()
            && !self.manifest_ref.trim().is_empty()
        {
            ReproExportState::Complete
        } else {
            ReproExportState::Incomplete
        }
    }

    /// Whether the export carries both core identity fields.
    pub fn has_core_identity(&self) -> bool {
        !self.package_id.trim().is_empty() && !self.digest.trim().is_empty()
    }
}

/// One anti-abuse/continuity/repro transparency row for a marketed M5 artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AntiAbuseReproRow {
    /// Stable row id.
    pub row_id: String,
    /// Marketed M5 artifact family this row governs.
    pub artifact_family: ArtifactFamily,
    /// Human-readable package identity.
    pub package_identity: String,
    /// Opaque, redacted ref to the package source path.
    pub source_path_ref: String,
    /// Runtime class of the package.
    pub runtime_class: RuntimeClass,
    /// Host/ABI execution locus.
    pub host_abi: HostAbiClass,
    /// Workspace origin.
    pub origin: WorkspaceOrigin,
    /// Signing/provenance state.
    pub signature_state: SignatureState,
    /// Trust posture the author requests, before the board caps it.
    pub declared_trust_posture: TrustPosture,
    /// Trust posture actually rendered after the board caps it.
    ///
    /// Must equal [`AntiAbuseReproRow::effective_rendered_trust`].
    pub rendered_trust_posture: TrustPosture,
    /// Ranking/anti-abuse reason chips, in canonical order.
    pub ranking_reasons: Vec<RankingReasonChip>,
    /// Ranking explainability; must equal the recomputed value.
    pub ranking_explainability: RankingExplainability,
    /// Sequenced continuity timeline.
    #[serde(default)]
    pub history_events: Vec<HistoryEvent>,
    /// Quarantine/removal history state; must equal the recomputed value.
    pub quarantine_history_state: QuarantineHistoryState,
    /// Publisher continuity-or-loss state; must equal the recomputed value.
    pub publisher_continuity_state: PublisherContinuityState,
    /// Reproducible export packet.
    pub repro_export: ReproExport,
    /// Local-to-published binding decision.
    pub bind_decision: BindPublishedIdentity,
    /// Opaque ref to the bind-published-identity review record (empty when not required).
    pub bind_review_ref: String,
    /// Transparency disposition; must equal the recomputed value.
    pub transparency_disposition: TransparencyDisposition,
    /// Ref binding this row into marketplace discovery.
    pub marketplace_ref: String,
    /// Ref binding this row into the authoring surface.
    pub authoring_ref: String,
    /// Ref to the family's publish-preview record.
    pub publish_preview_ref: String,
    /// Ref binding this row into diagnostics, support, and release surfaces.
    pub support_export_ref: String,
    /// Additional source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl AntiAbuseReproRow {
    /// The trust posture the board lets this row render.
    ///
    /// Lowers the declared posture to the weakest of the signing-state, origin, and
    /// binding ceilings, so a locally-built, side-loaded, or freshly bound artifact can
    /// never inherit a verified-publisher or enterprise-approved badge.
    pub fn effective_rendered_trust(&self) -> TrustPosture {
        self.declared_trust_posture
            .min(self.signature_state.trust_ceiling())
            .min(self.origin.trust_ceiling())
            .min(self.bind_decision.trust_ceiling())
    }

    /// The ranking-reason chips in canonical order, deduplicated.
    pub fn canonical_ranking_reasons(&self) -> Vec<RankingReasonChip> {
        let set: BTreeSet<RankingReasonChip> = self.ranking_reasons.iter().copied().collect();
        let mut reasons: Vec<RankingReasonChip> = set.into_iter().collect();
        reasons.sort_by_key(|chip| chip.rank());
        reasons
    }

    /// The ranking explainability recomputed from the chips.
    pub fn computed_ranking_explainability(&self) -> RankingExplainability {
        let substantive = self
            .ranking_reasons
            .iter()
            .filter(|c| c.is_substantive())
            .count();
        let vanity = self
            .ranking_reasons
            .iter()
            .filter(|c| c.is_vanity_metric())
            .count();
        if self
            .ranking_reasons
            .iter()
            .any(|c| c.is_anti_abuse_demotion())
        {
            RankingExplainability::AntiAbuseLed
        } else if vanity > substantive {
            RankingExplainability::VanityDominated
        } else {
            RankingExplainability::TrustLed
        }
    }

    /// The quarantine/removal history state folded from the timeline.
    pub fn computed_quarantine_history_state(&self) -> QuarantineHistoryState {
        let mut quarantined = false;
        let mut removed = false;
        let mut any_action = false;
        for event in self.events_in_sequence() {
            match event.kind {
                HistoryEventKind::Quarantined => {
                    quarantined = true;
                    any_action = true;
                }
                HistoryEventKind::QuarantineCleared => quarantined = false,
                HistoryEventKind::Removed => {
                    removed = true;
                    any_action = true;
                }
                HistoryEventKind::Reinstated => {
                    removed = false;
                    quarantined = false;
                }
                _ => {}
            }
        }
        if quarantined || removed {
            QuarantineHistoryState::CurrentlyWithheld
        } else if any_action {
            QuarantineHistoryState::PriorActionDisclosed
        } else {
            QuarantineHistoryState::Clean
        }
    }

    /// The publisher continuity-or-loss state folded from the timeline.
    pub fn computed_publisher_continuity_state(&self) -> PublisherContinuityState {
        let mut verified = false;
        let mut lost_ever = false;
        let mut transferred_ever = false;
        for event in self.events_in_sequence() {
            match event.kind {
                HistoryEventKind::VerifiedPublisherGranted => verified = true,
                HistoryEventKind::VerifiedPublisherLost => {
                    verified = false;
                    lost_ever = true;
                }
                HistoryEventKind::PublisherTransferred => transferred_ever = true,
                _ => {}
            }
        }
        if lost_ever && !verified {
            PublisherContinuityState::VerifiedPublisherLost
        } else if transferred_ever {
            PublisherContinuityState::PublisherTransferredDisclosed
        } else {
            PublisherContinuityState::Continuous
        }
    }

    /// The transparency disposition recomputed from the history states.
    pub fn computed_transparency_disposition(&self) -> TransparencyDisposition {
        if self
            .computed_quarantine_history_state()
            .is_currently_withheld()
        {
            TransparencyDisposition::WithheldQuarantined
        } else if self.computed_quarantine_history_state()
            == QuarantineHistoryState::PriorActionDisclosed
            || self.computed_publisher_continuity_state().is_disclosable()
        {
            TransparencyDisposition::VisibleWithHistoryDisclosure
        } else {
            TransparencyDisposition::VisibleClean
        }
    }

    /// The history events sorted by their sequence index.
    pub fn events_in_sequence(&self) -> Vec<&HistoryEvent> {
        let mut events: Vec<&HistoryEvent> = self.history_events.iter().collect();
        events.sort_by_key(|event| event.sequence);
        events
    }

    /// Whether the package is currently withheld (quarantined or removed).
    pub fn is_withheld(&self) -> bool {
        self.computed_transparency_disposition() == TransparencyDisposition::WithheldQuarantined
    }

    /// Whether this row renders as a local-only artifact.
    pub fn is_local_only(&self) -> bool {
        self.rendered_trust_posture == TrustPosture::UnsignedLocalOnly
    }

    /// Whether the row carries its own non-empty cross-surface refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.marketplace_ref.trim().is_empty()
            && !self.authoring_ref.trim().is_empty()
            && !self.publish_preview_ref.trim().is_empty()
            && !self.support_export_ref.trim().is_empty()
    }

    /// Whether every stored derived value agrees with the recomputed board decision.
    pub fn row_consistent(&self) -> bool {
        self.rendered_trust_posture == self.effective_rendered_trust()
            && self.ranking_explainability == self.computed_ranking_explainability()
            && self.quarantine_history_state == self.computed_quarantine_history_state()
            && self.publisher_continuity_state == self.computed_publisher_continuity_state()
            && self.transparency_disposition == self.computed_transparency_disposition()
            && self.repro_export.state == self.repro_export.computed_state()
            && self.ranking_reasons == self.canonical_ranking_reasons()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5AntiAbuseReproSummary {
    /// Total transparency rows.
    pub total_rows: usize,
    /// Number of marketed families claimed.
    pub family_count: usize,
    /// Rows visible with no adverse history.
    pub visible_clean_rows: usize,
    /// Rows visible with a history disclosure.
    pub visible_with_disclosure_rows: usize,
    /// Rows withheld as currently quarantined or removed.
    pub withheld_quarantined_rows: usize,
    /// Rows disclosing a prior quarantine or removal action.
    pub prior_action_rows: usize,
    /// Rows disclosing a publisher transfer.
    pub publisher_transferred_rows: usize,
    /// Rows disclosing a verified-publisher loss.
    pub verified_publisher_lost_rows: usize,
    /// Rows whose ranking is anti-abuse-led.
    pub anti_abuse_led_rows: usize,
    /// Rows whose ranking is trust-led.
    pub trust_led_rows: usize,
    /// Rows carrying a complete reproducible export.
    pub complete_repro_export_rows: usize,
    /// Rows carrying an incomplete reproducible export.
    pub incomplete_repro_export_rows: usize,
    /// Rows whose local-to-published rebind is pending review.
    pub bind_review_required_rows: usize,
    /// Rows bound to a published identity.
    pub bound_published_identity_rows: usize,
    /// Rows rendered as local-only (no inherited trust badge).
    pub local_only_rendered_rows: usize,
    /// Rows rendered with a verified-publisher or enterprise-approved badge.
    pub verified_or_enterprise_rendered_rows: usize,
}

/// A redaction-safe export row projected from a transparency row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AntiAbuseReproExportRow {
    /// Row id.
    pub row_id: String,
    /// Artifact-family token.
    pub artifact_family: String,
    /// Origin token.
    pub origin: String,
    /// Signing-state token.
    pub signature_state: String,
    /// Rendered trust-posture token.
    pub rendered_trust_posture: String,
    /// Ranking-explainability token.
    pub ranking_explainability: String,
    /// Ranking-reason chip tokens, in canonical order.
    pub ranking_reason_chips: Vec<String>,
    /// Quarantine-history-state token.
    pub quarantine_history_state: String,
    /// Publisher-continuity-state token.
    pub publisher_continuity_state: String,
    /// Repro-export-state token.
    pub repro_export_state: String,
    /// Bind-decision token.
    pub bind_decision: String,
    /// Transparency-disposition token.
    pub transparency_disposition: String,
    /// Whether the row is rendered as local-only.
    pub local_only: bool,
    /// Whether the row is currently withheld.
    pub withheld: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AntiAbuseReproExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<M5AntiAbuseReproExportRow>,
    /// Whether every row's stored decision agrees with the board.
    pub all_rows_consistent: bool,
    /// Rows visible (clean or with disclosure).
    pub visible_count: usize,
    /// Rows withheld.
    pub withheld_count: usize,
    /// Rows rendered as local-only.
    pub local_only_count: usize,
}

/// The typed M5 anti-abuse-and-repro board packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5AntiAbuseReproBoard {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Marketed families the packet claims; one row per family.
    pub artifact_families: Vec<ArtifactFamily>,
    /// Closed runtime-class vocabulary.
    pub runtime_classes: Vec<RuntimeClass>,
    /// Closed host/ABI vocabulary.
    pub host_abi_classes: Vec<HostAbiClass>,
    /// Closed workspace-origin vocabulary.
    pub workspace_origins: Vec<WorkspaceOrigin>,
    /// Closed signing-state vocabulary.
    pub signature_states: Vec<SignatureState>,
    /// Closed trust-posture vocabulary.
    pub trust_postures: Vec<TrustPosture>,
    /// Closed ranking-reason-category vocabulary.
    pub ranking_reason_categories: Vec<RankingReasonCategory>,
    /// Closed ranking-reason-chip vocabulary.
    pub ranking_reason_chips: Vec<RankingReasonChip>,
    /// Closed ranking-explainability vocabulary.
    pub ranking_explainability_states: Vec<RankingExplainability>,
    /// Closed history-event-kind vocabulary.
    pub history_event_kinds: Vec<HistoryEventKind>,
    /// Closed quarantine-history-state vocabulary.
    pub quarantine_history_states: Vec<QuarantineHistoryState>,
    /// Closed publisher-continuity-state vocabulary.
    pub publisher_continuity_states: Vec<PublisherContinuityState>,
    /// Closed repro-export-state vocabulary.
    pub repro_export_states: Vec<ReproExportState>,
    /// Closed bind-decision vocabulary.
    pub bind_decisions: Vec<BindPublishedIdentity>,
    /// Closed transparency-disposition vocabulary.
    pub transparency_dispositions: Vec<TransparencyDisposition>,
    /// Transparency rows, one per marketed family.
    #[serde(default)]
    pub rows: Vec<AntiAbuseReproRow>,
    /// Summary counts.
    pub summary: M5AntiAbuseReproSummary,
}

impl M5AntiAbuseReproBoard {
    /// Returns the row for a marketed family.
    pub fn row(&self, family: ArtifactFamily) -> Option<&AntiAbuseReproRow> {
        self.rows.iter().find(|r| r.artifact_family == family)
    }

    /// Rows rendered as local-only.
    pub fn local_only_rows(&self) -> impl Iterator<Item = &AntiAbuseReproRow> {
        self.rows.iter().filter(|r| r.is_local_only())
    }

    /// Rows currently withheld.
    pub fn withheld_rows(&self) -> impl Iterator<Item = &AntiAbuseReproRow> {
        self.rows.iter().filter(|r| r.is_withheld())
    }

    /// Rows visible (clean or with disclosure).
    pub fn visible_rows(&self) -> impl Iterator<Item = &AntiAbuseReproRow> {
        self.rows.iter().filter(|r| !r.is_withheld())
    }

    /// Whether every row's stored decision agrees with the recomputed board.
    pub fn all_rows_consistent(&self) -> bool {
        self.rows.iter().all(|r| r.row_consistent())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5AntiAbuseReproSummary {
        let count_disposition = |disposition: TransparencyDisposition| {
            self.rows
                .iter()
                .filter(|r| r.transparency_disposition == disposition)
                .count()
        };
        M5AntiAbuseReproSummary {
            total_rows: self.rows.len(),
            family_count: self.artifact_families.len(),
            visible_clean_rows: count_disposition(TransparencyDisposition::VisibleClean),
            visible_with_disclosure_rows: count_disposition(
                TransparencyDisposition::VisibleWithHistoryDisclosure,
            ),
            withheld_quarantined_rows: count_disposition(
                TransparencyDisposition::WithheldQuarantined,
            ),
            prior_action_rows: self
                .rows
                .iter()
                .filter(|r| {
                    r.quarantine_history_state == QuarantineHistoryState::PriorActionDisclosed
                })
                .count(),
            publisher_transferred_rows: self
                .rows
                .iter()
                .filter(|r| {
                    r.publisher_continuity_state
                        == PublisherContinuityState::PublisherTransferredDisclosed
                })
                .count(),
            verified_publisher_lost_rows: self
                .rows
                .iter()
                .filter(|r| {
                    r.publisher_continuity_state == PublisherContinuityState::VerifiedPublisherLost
                })
                .count(),
            anti_abuse_led_rows: self
                .rows
                .iter()
                .filter(|r| r.ranking_explainability == RankingExplainability::AntiAbuseLed)
                .count(),
            trust_led_rows: self
                .rows
                .iter()
                .filter(|r| r.ranking_explainability == RankingExplainability::TrustLed)
                .count(),
            complete_repro_export_rows: self
                .rows
                .iter()
                .filter(|r| r.repro_export.state == ReproExportState::Complete)
                .count(),
            incomplete_repro_export_rows: self
                .rows
                .iter()
                .filter(|r| r.repro_export.state == ReproExportState::Incomplete)
                .count(),
            bind_review_required_rows: self
                .rows
                .iter()
                .filter(|r| r.bind_decision == BindPublishedIdentity::BindReviewRequired)
                .count(),
            bound_published_identity_rows: self
                .rows
                .iter()
                .filter(|r| r.bind_decision == BindPublishedIdentity::BoundPublishedIdentity)
                .count(),
            local_only_rendered_rows: self.local_only_rows().count(),
            verified_or_enterprise_rendered_rows: self
                .rows
                .iter()
                .filter(|r| r.rendered_trust_posture.is_trusted_badge())
                .count(),
        }
    }

    /// Produces an export projection that downstream surfaces — marketplace discovery,
    /// authoring surfaces, diagnostics, support, and release surfaces — render instead of
    /// restating anti-abuse, history, or repro-export status text by hand.
    pub fn export_projection(&self) -> M5AntiAbuseReproExportProjection {
        let rows = self
            .rows
            .iter()
            .map(|r| M5AntiAbuseReproExportRow {
                row_id: r.row_id.clone(),
                artifact_family: r.artifact_family.as_str().to_owned(),
                origin: r.origin.as_str().to_owned(),
                signature_state: r.signature_state.as_str().to_owned(),
                rendered_trust_posture: r.rendered_trust_posture.as_str().to_owned(),
                ranking_explainability: r.ranking_explainability.as_str().to_owned(),
                ranking_reason_chips: r
                    .ranking_reasons
                    .iter()
                    .map(|chip| chip.as_str().to_owned())
                    .collect(),
                quarantine_history_state: r.quarantine_history_state.as_str().to_owned(),
                publisher_continuity_state: r.publisher_continuity_state.as_str().to_owned(),
                repro_export_state: r.repro_export.state.as_str().to_owned(),
                bind_decision: r.bind_decision.as_str().to_owned(),
                transparency_disposition: r.transparency_disposition.as_str().to_owned(),
                local_only: r.is_local_only(),
                withheld: r.is_withheld(),
                summary: format!(
                    "{}: ranking {}, history {}, publisher {}, repro {}, bind {}, disposition {}; rendered {}{}",
                    r.artifact_family.as_str(),
                    r.ranking_explainability.as_str(),
                    r.quarantine_history_state.as_str(),
                    r.publisher_continuity_state.as_str(),
                    r.repro_export.state.as_str(),
                    r.bind_decision.as_str(),
                    r.transparency_disposition.as_str(),
                    r.rendered_trust_posture.as_str(),
                    if r.is_local_only() { " (local-only)" } else { "" },
                ),
            })
            .collect();
        M5AntiAbuseReproExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_rows_consistent: self.all_rows_consistent(),
            visible_count: self.visible_rows().count(),
            withheld_count: self.withheld_rows().count(),
            local_only_count: self.local_only_rows().count(),
        }
    }

    /// Cross-checks the rows against the publish-preview gate.
    ///
    /// Proves no row renders a *stronger* trust badge than the publish-preview gate would
    /// grant the same family, so a local-to-published or mirrored transparency row can
    /// never widen the rendered trust above the publish gate.
    pub fn cross_check_matrix(
        &self,
        matrix: &M5AuthorPublishMatrix,
    ) -> Vec<M5AntiAbuseReproViolation> {
        let mut violations = Vec::new();
        for row in &self.rows {
            match matrix.family(row.artifact_family) {
                None => violations.push(M5AntiAbuseReproViolation::MissingMatrixRow {
                    row_id: row.row_id.clone(),
                    family: row.artifact_family.as_str(),
                }),
                Some(gate) => {
                    if row.rendered_trust_posture.rank() > gate.published_trust_posture.rank() {
                        violations.push(M5AntiAbuseReproViolation::RowExceedsPublishGate {
                            row_id: row.row_id.clone(),
                            rendered: row.rendered_trust_posture.as_str(),
                            published: gate.published_trust_posture.as_str(),
                        });
                    }
                }
            }
        }
        violations
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5AntiAbuseReproViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<ArtifactFamily> = self.artifact_families.iter().copied().collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !seen_ids.insert(row.row_id.clone()) {
                violations.push(M5AntiAbuseReproViolation::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }
            if !seen_families.insert(row.artifact_family) {
                violations.push(M5AntiAbuseReproViolation::DuplicateFamilyRow {
                    family: row.artifact_family.as_str(),
                });
            }
            if !claimed.contains(&row.artifact_family) {
                violations.push(M5AntiAbuseReproViolation::UnclaimedFamilyRow {
                    row_id: row.row_id.clone(),
                    family: row.artifact_family.as_str(),
                });
            }
            self.validate_row(row, &mut violations);
        }

        // Every claimed family must carry its own row, so anti-abuse, history, or
        // repro-export truth never falls off the board by losing a row.
        for &family in &self.artifact_families {
            if !seen_families.contains(&family) {
                violations.push(M5AntiAbuseReproViolation::MissingFamilyRow {
                    family: family.as_str(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5AntiAbuseReproViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5AntiAbuseReproViolation>) {
        if self.schema_version != M5_ANTI_ABUSE_REPRO_SCHEMA_VERSION {
            violations.push(M5AntiAbuseReproViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_ANTI_ABUSE_REPRO_RECORD_KIND {
            violations.push(M5AntiAbuseReproViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5AntiAbuseReproViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "artifact_families",
                self.artifact_families == ArtifactFamily::ALL.to_vec(),
            ),
            (
                "runtime_classes",
                self.runtime_classes == RuntimeClass::ALL.to_vec(),
            ),
            (
                "host_abi_classes",
                self.host_abi_classes == HostAbiClass::ALL.to_vec(),
            ),
            (
                "workspace_origins",
                self.workspace_origins == WorkspaceOrigin::ALL.to_vec(),
            ),
            (
                "signature_states",
                self.signature_states == SignatureState::ALL.to_vec(),
            ),
            (
                "trust_postures",
                self.trust_postures == TrustPosture::ALL.to_vec(),
            ),
            (
                "ranking_reason_categories",
                self.ranking_reason_categories == RankingReasonCategory::ALL.to_vec(),
            ),
            (
                "ranking_reason_chips",
                self.ranking_reason_chips == RankingReasonChip::ALL.to_vec(),
            ),
            (
                "ranking_explainability_states",
                self.ranking_explainability_states == RankingExplainability::ALL.to_vec(),
            ),
            (
                "history_event_kinds",
                self.history_event_kinds == HistoryEventKind::ALL.to_vec(),
            ),
            (
                "quarantine_history_states",
                self.quarantine_history_states == QuarantineHistoryState::ALL.to_vec(),
            ),
            (
                "publisher_continuity_states",
                self.publisher_continuity_states == PublisherContinuityState::ALL.to_vec(),
            ),
            (
                "repro_export_states",
                self.repro_export_states == ReproExportState::ALL.to_vec(),
            ),
            (
                "bind_decisions",
                self.bind_decisions == BindPublishedIdentity::ALL.to_vec(),
            ),
            (
                "transparency_dispositions",
                self.transparency_dispositions == TransparencyDisposition::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5AntiAbuseReproViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_row(
        &self,
        row: &AntiAbuseReproRow,
        violations: &mut Vec<M5AntiAbuseReproViolation>,
    ) {
        for (field, value) in [
            ("row_id", &row.row_id),
            ("package_identity", &row.package_identity),
            ("source_path_ref", &row.source_path_ref),
            ("marketplace_ref", &row.marketplace_ref),
            ("authoring_ref", &row.authoring_ref),
            ("publish_preview_ref", &row.publish_preview_ref),
            ("support_export_ref", &row.support_export_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5AntiAbuseReproViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: field,
                });
            }
        }

        // Each row must explain its ranking with at least one chip, in canonical order.
        if row.ranking_reasons.is_empty() {
            violations.push(M5AntiAbuseReproViolation::EmptyRankingReasons {
                row_id: row.row_id.clone(),
            });
        }
        if row.ranking_reasons != row.canonical_ranking_reasons() {
            violations.push(M5AntiAbuseReproViolation::RankingReasonsNotCanonical {
                row_id: row.row_id.clone(),
            });
        }

        // Vanity metrics must never dominate a ranking decision.
        let computed_explainability = row.computed_ranking_explainability();
        if computed_explainability == RankingExplainability::VanityDominated {
            violations.push(M5AntiAbuseReproViolation::VanityMetricsDominateRanking {
                row_id: row.row_id.clone(),
            });
        }
        if row.ranking_explainability != computed_explainability {
            violations.push(M5AntiAbuseReproViolation::RankingExplainabilityMismatch {
                row_id: row.row_id.clone(),
                declared: row.ranking_explainability.as_str(),
                computed: computed_explainability.as_str(),
            });
        }

        // History timeline must have strictly increasing sequence indices, and each
        // event must carry its date, detail ref, and note.
        let mut last_seq: Option<u32> = None;
        for event in row.events_in_sequence() {
            if let Some(prev) = last_seq {
                if event.sequence <= prev {
                    violations.push(M5AntiAbuseReproViolation::HistorySequenceNotMonotonic {
                        row_id: row.row_id.clone(),
                    });
                }
            }
            last_seq = Some(event.sequence);
            if event.at.trim().is_empty()
                || event.detail_ref.trim().is_empty()
                || event.note.trim().is_empty()
            {
                violations.push(M5AntiAbuseReproViolation::EmptyHistoryEventField {
                    row_id: row.row_id.clone(),
                    sequence: event.sequence,
                });
            }
        }

        // Quarantine, publisher continuity, and transparency disposition must all match
        // the recomputed fold, so a verified-publisher loss or quarantine can never be
        // hidden by recording a clean state by hand.
        let computed_quarantine = row.computed_quarantine_history_state();
        if row.quarantine_history_state != computed_quarantine {
            violations.push(M5AntiAbuseReproViolation::QuarantineHistoryStateMismatch {
                row_id: row.row_id.clone(),
                declared: row.quarantine_history_state.as_str(),
                computed: computed_quarantine.as_str(),
            });
        }
        let computed_continuity = row.computed_publisher_continuity_state();
        if row.publisher_continuity_state != computed_continuity {
            violations.push(
                M5AntiAbuseReproViolation::PublisherContinuityStateMismatch {
                    row_id: row.row_id.clone(),
                    declared: row.publisher_continuity_state.as_str(),
                    computed: computed_continuity.as_str(),
                },
            );
        }
        let computed_disposition = row.computed_transparency_disposition();
        if row.transparency_disposition != computed_disposition {
            violations.push(M5AntiAbuseReproViolation::TransparencyDispositionMismatch {
                row_id: row.row_id.clone(),
                declared: row.transparency_disposition.as_str(),
                computed: computed_disposition.as_str(),
            });
        }

        // A current quarantine must be reflected in the ranking chips, so anti-abuse
        // action is never hidden in a moderation-only tool.
        let has_quarantine_chip = row
            .ranking_reasons
            .contains(&RankingReasonChip::AntiAbuseQuarantined);
        if has_quarantine_chip != computed_quarantine.is_currently_withheld() {
            violations.push(M5AntiAbuseReproViolation::QuarantineNotReflectedInRanking {
                row_id: row.row_id.clone(),
            });
        }

        // The repro-export packet must carry its core identity, be self-contained, and
        // record the state recomputed from its content refs.
        if !row.repro_export.has_core_identity() {
            violations.push(M5AntiAbuseReproViolation::ReproExportMissingCoreIdentity {
                row_id: row.row_id.clone(),
            });
        }
        if !row.repro_export.self_contained {
            violations.push(M5AntiAbuseReproViolation::ReproExportNotSelfContained {
                row_id: row.row_id.clone(),
            });
        }
        if row.repro_export.host_abi != row.host_abi {
            violations.push(M5AntiAbuseReproViolation::ReproExportHostAbiMismatch {
                row_id: row.row_id.clone(),
            });
        }
        let computed_repro = row.repro_export.computed_state();
        if row.repro_export.state != computed_repro {
            violations.push(M5AntiAbuseReproViolation::ReproExportStateMismatch {
                row_id: row.row_id.clone(),
                declared: row.repro_export.state.as_str(),
                computed: computed_repro.as_str(),
            });
        }

        // The rendered trust posture must equal the board's recomputed posture, so a row
        // can never render a stronger badge than its signing, origin, and binding allow.
        let effective = row.effective_rendered_trust();
        if row.rendered_trust_posture != effective {
            violations.push(M5AntiAbuseReproViolation::OverstatedTrustPosture {
                row_id: row.row_id.clone(),
                rendered: row.rendered_trust_posture.as_str(),
                computed: effective.as_str(),
            });
        }

        // Non-inheritance: a locally authored, side-loaded, unsigned/revoked, or
        // not-yet-bound row must render local-only and may never inherit a trusted badge
        // just because it was built on a trusted machine.
        let locally_capped = row.signature_state.is_local_or_untrusted()
            || row.origin.caps_to_local_only()
            || matches!(
                row.bind_decision,
                BindPublishedIdentity::StayLocal | BindPublishedIdentity::BindReviewRequired
            );
        if locally_capped && row.rendered_trust_posture != TrustPosture::UnsignedLocalOnly {
            violations.push(M5AntiAbuseReproViolation::LocalArtifactInheritedTrust {
                row_id: row.row_id.clone(),
                rendered: row.rendered_trust_posture.as_str(),
            });
        }

        // Local-to-published rebinding must pass through an explicit review: a bound
        // identity may never appear on a still-local or side-loaded origin, and a row
        // pending or completing the bind must carry its review ref.
        if row.origin.is_local_authored()
            && row.bind_decision == BindPublishedIdentity::BoundPublishedIdentity
        {
            violations.push(M5AntiAbuseReproViolation::SilentPublishedIdentityBind {
                row_id: row.row_id.clone(),
                origin: row.origin.as_str(),
            });
        }
        if row.bind_decision.requires_review_ref() && row.bind_review_ref.trim().is_empty() {
            violations.push(M5AntiAbuseReproViolation::MissingBindReviewRef {
                row_id: row.row_id.clone(),
                bind_decision: row.bind_decision.as_str(),
            });
        }
    }
}

/// A validation violation for the M5 anti-abuse-and-repro packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AntiAbuseReproViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// A marketed family carries more than one row.
    DuplicateFamilyRow {
        /// Family token.
        family: &'static str,
    },
    /// A claimed marketed family has no row.
    MissingFamilyRow {
        /// Family token.
        family: &'static str,
    },
    /// A row covers a family the packet does not claim.
    UnclaimedFamilyRow {
        /// Row id.
        row_id: String,
        /// Family token.
        family: &'static str,
    },
    /// A row carries no ranking-reason chips.
    EmptyRankingReasons {
        /// Row id.
        row_id: String,
    },
    /// A row's ranking-reason chips are not in canonical, deduplicated order.
    RankingReasonsNotCanonical {
        /// Row id.
        row_id: String,
    },
    /// A row's ranking is dominated by vanity metrics.
    VanityMetricsDominateRanking {
        /// Row id.
        row_id: String,
    },
    /// A row's stored ranking explainability disagrees with the recomputed value.
    RankingExplainabilityMismatch {
        /// Row id.
        row_id: String,
        /// Declared explainability token.
        declared: &'static str,
        /// Computed explainability token.
        computed: &'static str,
    },
    /// A row's history-event sequence indices are not strictly increasing.
    HistorySequenceNotMonotonic {
        /// Row id.
        row_id: String,
    },
    /// A history event is missing its date, detail ref, or note.
    EmptyHistoryEventField {
        /// Row id.
        row_id: String,
        /// Event sequence index.
        sequence: u32,
    },
    /// A row's quarantine-history state disagrees with the recomputed fold.
    QuarantineHistoryStateMismatch {
        /// Row id.
        row_id: String,
        /// Declared state token.
        declared: &'static str,
        /// Computed state token.
        computed: &'static str,
    },
    /// A row's publisher-continuity state disagrees with the recomputed fold.
    PublisherContinuityStateMismatch {
        /// Row id.
        row_id: String,
        /// Declared state token.
        declared: &'static str,
        /// Computed state token.
        computed: &'static str,
    },
    /// A row's transparency disposition disagrees with the recomputed value.
    TransparencyDispositionMismatch {
        /// Row id.
        row_id: String,
        /// Declared disposition token.
        declared: &'static str,
        /// Computed disposition token.
        computed: &'static str,
    },
    /// A current quarantine is not reflected in the ranking chips, or vice versa.
    QuarantineNotReflectedInRanking {
        /// Row id.
        row_id: String,
    },
    /// A repro-export packet is missing its package id or digest.
    ReproExportMissingCoreIdentity {
        /// Row id.
        row_id: String,
    },
    /// A repro-export packet is not self-contained.
    ReproExportNotSelfContained {
        /// Row id.
        row_id: String,
    },
    /// A repro-export packet's host ABI disagrees with the row's host ABI.
    ReproExportHostAbiMismatch {
        /// Row id.
        row_id: String,
    },
    /// A repro-export packet's state disagrees with the recomputed completeness.
    ReproExportStateMismatch {
        /// Row id.
        row_id: String,
        /// Declared state token.
        declared: &'static str,
        /// Computed state token.
        computed: &'static str,
    },
    /// A row renders a trust posture beyond what its signing/origin/binding supports.
    OverstatedTrustPosture {
        /// Row id.
        row_id: String,
        /// Rendered trust-posture token.
        rendered: &'static str,
        /// Computed effective trust-posture token.
        computed: &'static str,
    },
    /// A locally authored or side-loaded artifact inherited a trusted publisher badge.
    LocalArtifactInheritedTrust {
        /// Row id.
        row_id: String,
        /// Rendered trust-posture token.
        rendered: &'static str,
    },
    /// A local or side-loaded origin claims a bound published identity without review.
    SilentPublishedIdentityBind {
        /// Row id.
        row_id: String,
        /// Origin token.
        origin: &'static str,
    },
    /// A row pending or completing a bind is missing its review ref.
    MissingBindReviewRef {
        /// Row id.
        row_id: String,
        /// Bind-decision token.
        bind_decision: &'static str,
    },
    /// A row references a family the publish-preview gate does not carry.
    MissingMatrixRow {
        /// Row id.
        row_id: String,
        /// Family token.
        family: &'static str,
    },
    /// A row renders a stronger badge than the publish-preview gate would grant.
    RowExceedsPublishGate {
        /// Row id.
        row_id: String,
        /// Rendered trust-posture token.
        rendered: &'static str,
        /// Gate's published trust-posture token.
        published: &'static str,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5AntiAbuseReproViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateRowId { row_id } => {
                write!(f, "duplicate row id {row_id}")
            }
            Self::DuplicateFamilyRow { family } => {
                write!(f, "duplicate row for family {family}")
            }
            Self::MissingFamilyRow { family } => {
                write!(f, "missing row for claimed family {family}")
            }
            Self::UnclaimedFamilyRow { row_id, family } => {
                write!(f, "row {row_id} covers unclaimed family {family}")
            }
            Self::EmptyRankingReasons { row_id } => {
                write!(f, "row {row_id} carries no ranking-reason chips")
            }
            Self::RankingReasonsNotCanonical { row_id } => {
                write!(
                    f,
                    "row {row_id} ranking-reason chips are not in canonical, deduplicated order"
                )
            }
            Self::VanityMetricsDominateRanking { row_id } => {
                write!(f, "row {row_id} ranking is dominated by vanity metrics")
            }
            Self::RankingExplainabilityMismatch {
                row_id,
                declared,
                computed,
            } => {
                write!(
                    f,
                    "row {row_id} records ranking explainability {declared} but the board computes {computed}"
                )
            }
            Self::HistorySequenceNotMonotonic { row_id } => {
                write!(
                    f,
                    "row {row_id} history-event sequence indices are not strictly increasing"
                )
            }
            Self::EmptyHistoryEventField { row_id, sequence } => {
                write!(
                    f,
                    "row {row_id} history event {sequence} is missing its date, detail ref, or note"
                )
            }
            Self::QuarantineHistoryStateMismatch {
                row_id,
                declared,
                computed,
            } => {
                write!(
                    f,
                    "row {row_id} records quarantine-history state {declared} but the board computes {computed}"
                )
            }
            Self::PublisherContinuityStateMismatch {
                row_id,
                declared,
                computed,
            } => {
                write!(
                    f,
                    "row {row_id} records publisher-continuity state {declared} but the board computes {computed}"
                )
            }
            Self::TransparencyDispositionMismatch {
                row_id,
                declared,
                computed,
            } => {
                write!(
                    f,
                    "row {row_id} records transparency disposition {declared} but the board computes {computed}"
                )
            }
            Self::QuarantineNotReflectedInRanking { row_id } => {
                write!(
                    f,
                    "row {row_id} quarantine state and ranking chips disagree on anti-abuse visibility"
                )
            }
            Self::ReproExportMissingCoreIdentity { row_id } => {
                write!(
                    f,
                    "row {row_id} repro export is missing its package id or digest"
                )
            }
            Self::ReproExportNotSelfContained { row_id } => {
                write!(
                    f,
                    "row {row_id} repro export is not self-contained (raw supervisor traces or paid service)"
                )
            }
            Self::ReproExportHostAbiMismatch { row_id } => {
                write!(
                    f,
                    "row {row_id} repro export host ABI disagrees with the row"
                )
            }
            Self::ReproExportStateMismatch {
                row_id,
                declared,
                computed,
            } => {
                write!(
                    f,
                    "row {row_id} records repro-export state {declared} but the board computes {computed}"
                )
            }
            Self::OverstatedTrustPosture {
                row_id,
                rendered,
                computed,
            } => {
                write!(
                    f,
                    "row {row_id} renders trust posture {rendered} but the board computes {computed}"
                )
            }
            Self::LocalArtifactInheritedTrust { row_id, rendered } => {
                write!(
                    f,
                    "row {row_id} renders {rendered} but a locally authored or side-loaded artifact must render unsigned_local_only"
                )
            }
            Self::SilentPublishedIdentityBind { row_id, origin } => {
                write!(
                    f,
                    "row {row_id} is {origin} but claims a bound published identity; the bind must pass an explicit review first"
                )
            }
            Self::MissingBindReviewRef {
                row_id,
                bind_decision,
            } => {
                write!(
                    f,
                    "row {row_id} bind decision {bind_decision} is missing its bind-published-identity review ref"
                )
            }
            Self::MissingMatrixRow { row_id, family } => {
                write!(
                    f,
                    "row {row_id} references family {family} which the publish-preview gate does not carry"
                )
            }
            Self::RowExceedsPublishGate {
                row_id,
                rendered,
                published,
            } => {
                write!(
                    f,
                    "row {row_id} renders {rendered} but the publish-preview gate grants only {published}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for M5AntiAbuseReproViolation {}

/// Loads the embedded M5 anti-abuse-and-repro board packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5AntiAbuseReproBoard`].
pub fn current_m5_anti_abuse_repro_board() -> Result<M5AntiAbuseReproBoard, serde_json::Error> {
    serde_json::from_str(M5_ANTI_ABUSE_REPRO_JSON)
}

#[cfg(test)]
mod tests;
