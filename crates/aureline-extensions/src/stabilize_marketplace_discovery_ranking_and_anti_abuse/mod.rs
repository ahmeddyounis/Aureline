//! Stabilize marketplace discovery ranking, anti-abuse, verified-publisher tiers,
//! quarantine, revocation, and enterprise curation truth for the stable extension
//! ecosystem lane.
//!
//! Marketplace discovery is a trust surface. This module keeps the ranking reasons,
//! verified-publisher posture, anti-abuse controls, quarantine / revocation state,
//! enterprise curation flow, transparency actions, and support-export projection in
//! one typed packet so public catalogs, mirrored catalogs, enterprise-approved
//! registries, offline bundles, and support surfaces cannot invent local status text.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version for every stable marketplace discovery packet.
pub const MARKETPLACE_DISCOVERY_SCHEMA_VERSION: u32 = 1;

/// The published stable discovery model version.
pub const MARKETPLACE_DISCOVERY_PUBLISHED_VERSION: u32 = 1;

/// Canonical schema path the packet cites.
pub const MARKETPLACE_DISCOVERY_SCHEMA_REF: &str =
    "schemas/extensions/marketplace-ranking-and-anti-abuse.schema.json";

/// Record-kind tag for [`MarketplaceDiscoveryPacket`].
pub const MARKETPLACE_DISCOVERY_PACKET_RECORD_KIND: &str = "marketplace_discovery_packet";

/// Record-kind tag for [`MarketplaceDiscoverySupportExport`].
pub const MARKETPLACE_DISCOVERY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "marketplace_discovery_support_export";

/// Required ranking signal classes for launch-grade discovery.
pub const REQUIRED_RANKING_SIGNAL_CLASSES: &[&str] = &[
    "query_relevance",
    "category_fit",
    "current_version_compatibility",
    "runtime_health",
    "maintenance_freshness",
    "verified_or_official_status",
    "rollback_uninstall_quality",
    "docs_security_posture",
];

/// Closed ranking-signal vocabulary.
pub const RANKING_SIGNAL_CLASSES: &[&str] = REQUIRED_RANKING_SIGNAL_CLASSES;

/// Required anti-abuse controls for launch-grade discovery.
pub const REQUIRED_ABUSE_CONTROL_CLASSES: &[&str] = &[
    "publisher_identity_verification",
    "namespace_reservation",
    "typosquat_detection",
    "look_alike_detection",
    "review_install_fraud_detection",
    "suspicious_package_quarantine",
    "malware_static_policy_scanning",
    "rapid_revocation",
];

/// Closed anti-abuse control vocabulary.
pub const ABUSE_CONTROL_CLASSES: &[&str] = REQUIRED_ABUSE_CONTROL_CLASSES;

/// Closed publisher/status tier vocabulary shared by public and mirrored lanes.
pub const PUBLISHER_TIER_CLASSES: &[&str] = &[
    "official_pack",
    "verified_publisher",
    "enterprise_approved",
    "community",
    "under_review",
];

/// Closed discovery posture vocabulary.
pub const DISCOVERY_POSTURE_CLASSES: &[&str] = &[
    "prominent",
    "ordinary",
    "narrowed",
    "under_review",
    "quarantined",
    "revoked",
];

/// Closed anti-abuse state vocabulary.
pub const ABUSE_STATE_CLASSES: &[&str] = &[
    "passed",
    "under_review",
    "suspicious",
    "quarantined",
    "blocked",
    "revoked",
];

/// Closed quarantine and revocation status vocabulary.
pub const QUARANTINE_REVOCATION_CLASSES: &[&str] = &[
    "none",
    "review_hold",
    "quarantined",
    "revoked",
    "emergency_disabled",
];

/// Closed registry lane vocabulary.
pub const REGISTRY_LANE_CLASSES: &[&str] = &[
    "public_registry",
    "quarantine_holding",
    "approved_mirror",
    "private_registry",
    "offline_bundle",
];

/// Closed transparency event vocabulary.
pub const TRANSPARENCY_EVENT_CLASSES: &[&str] = &[
    "removal",
    "appeal",
    "emergency_disable",
    "verified_publisher_action",
    "quarantine",
    "revocation",
];

/// Closed stability-tier vocabulary.
pub const STABILITY_TIERS: &[&str] = &["stable", "beta", "preview", "withdrawn"];

/// Narrowing reasons that can be emitted by the discovery packet.
pub const DISCOVERY_NARROWING_REASONS: &[&str] = &[
    "discovery_version_not_published",
    "ranking_signal_missing",
    "ranking_reason_not_explainable",
    "ranking_uses_vanity_metric",
    "stale_compatibility",
    "crash_resource_regression",
    "maintenance_stale",
    "high_rollback_or_uninstall_rate",
    "docs_security_gap",
    "publisher_status_not_mechanical",
    "publisher_identity_unverified",
    "publisher_under_review",
    "anti_abuse_control_missing",
    "abuse_reason_not_visible",
    "lookalike_or_typosquat_under_review",
    "fraud_review_active",
    "suspicious_package_quarantined",
    "malware_policy_block",
    "revocation_active",
    "enterprise_curation_identity_gap",
    "surface_visibility_gap",
    "transparency_not_exportable",
];

/// Reasons that withdraw the discovery claim outright.
pub const WITHDRAWN_REASONS: &[&str] = &[
    "suspicious_package_quarantined",
    "malware_policy_block",
    "revocation_active",
    "enterprise_curation_identity_gap",
];

/// Reasons that narrow the discovery claim to preview.
pub const PREVIEW_REASONS: &[&str] = &[
    "discovery_version_not_published",
    "ranking_signal_missing",
    "ranking_reason_not_explainable",
    "ranking_uses_vanity_metric",
    "publisher_status_not_mechanical",
    "publisher_identity_unverified",
    "publisher_under_review",
    "anti_abuse_control_missing",
    "abuse_reason_not_visible",
    "lookalike_or_typosquat_under_review",
    "fraud_review_active",
    "surface_visibility_gap",
    "transparency_not_exportable",
];

/// Reasons that narrow the discovery claim to beta.
pub const BETA_REASONS: &[&str] = &[
    "stale_compatibility",
    "crash_resource_regression",
    "maintenance_stale",
    "high_rollback_or_uninstall_rate",
    "docs_security_gap",
];

/// Input used to build a [`MarketplaceDiscoveryPacket`].
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MarketplaceDiscoveryInput {
    /// Stable package identity, shared by public, mirrored, and support surfaces.
    pub identity: MarketplacePackageIdentity,
    /// Publisher and verified-status posture sourced from the registry/status model.
    pub publisher: PublisherStatus,
    /// Typed ranking signals used before any raw popularity or review volume.
    pub ranking_signals: Vec<RankingSignal>,
    /// Anti-abuse controls and their visible states.
    pub anti_abuse_controls: Vec<AntiAbuseControl>,
    /// Quarantine or revocation status currently attached to the package.
    pub quarantine_revocation: QuarantineRevocationStatus,
    /// Public, mirrored, private, and offline promotion / curation flow rows.
    pub curation_paths: Vec<EnterpriseCurationPath>,
    /// Marketplace and export surfaces that must render the same truth.
    pub surface_truth: SurfaceTruth,
    /// Transparency-ready moderation and verified-publisher action events.
    pub transparency_events: Vec<TransparencyEvent>,
    /// Tier requested by the producer before discovery posture is evaluated.
    pub claimed_tier: String,
}

/// Package identity that stays stable across registry lanes.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MarketplacePackageIdentity {
    /// Record kind for the identity row.
    pub record_kind: String,
    /// Schema version for the identity row.
    pub schema_version: u32,
    /// Catalog descriptor ref that produced this discovery row.
    pub catalog_descriptor_ref: String,
    /// Package id shown in marketplace and install surfaces.
    pub package_id: String,
    /// Package version evaluated by the discovery packet.
    pub package_version: String,
    /// Digest or immutable artifact ref for mirror/offline continuity.
    pub artifact_ref: String,
    /// Current discovery-model version pinned by this row.
    pub discovery_version: u32,
}

/// Publisher status derived from one registry/status model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PublisherStatus {
    /// Record kind for the publisher status row.
    pub record_kind: String,
    /// Publisher identity displayed to users and admins.
    pub publisher_id: String,
    /// Publisher tier such as `verified_publisher` or `official_pack`.
    pub tier_class: String,
    /// Whether identity verification has passed for tiered publishers.
    pub identity_verified: bool,
    /// Whether this row is mechanically sourced rather than copied badge text.
    pub mechanically_sourced: bool,
    /// Registry/status model ref used to derive the tier.
    pub status_model_ref: String,
    /// Namespace reservation or continuity ref for look-alike protection.
    pub namespace_ref: String,
}

/// One typed ranking signal and its user-visible reason chip.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct RankingSignal {
    /// Ranking signal class.
    pub signal_class: String,
    /// Signed score delta contributed by this signal.
    pub score_delta: i16,
    /// Concise reason chip shown on result rows or details.
    pub reason_chip: String,
    /// Evidence ref that backs the signal.
    pub evidence_ref: String,
    /// Whether this signal used raw install count or review volume as primary input.
    pub raw_install_count_primary: bool,
}

/// One anti-abuse control and its visible moderation state.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct AntiAbuseControl {
    /// Anti-abuse control class.
    pub control_class: String,
    /// Current state for this control.
    pub state_class: String,
    /// User-visible reason text or label for suspicious / blocked states.
    pub user_visible_reason: String,
    /// Admin-visible reason or review reference.
    pub admin_visible_reason: String,
    /// Evidence ref for moderation, fraud, scanner, or revocation review.
    pub evidence_ref: String,
    /// Whether this control projects safely into support export.
    pub support_exportable: bool,
    /// Whether mirrored/offline consumers retain continuity for this control.
    pub mirror_safe_continuity: bool,
}

/// Quarantine and revocation truth for the affected package.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct QuarantineRevocationStatus {
    /// Current quarantine or revocation class.
    pub status_class: String,
    /// Reason class shown to users/admins when the package is narrowed or blocked.
    pub reason_class: String,
    /// Installed impact summary, such as updates paused or load denied.
    pub installed_impact: String,
    /// Last-known-good package ref when pinning remains allowed.
    pub last_known_good_ref: String,
    /// Whether mirrored/offline catalogs preserve this status.
    pub mirror_visible: bool,
}

/// Enterprise curation or mirror promotion path.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct EnterpriseCurationPath {
    /// Source registry lane.
    pub source_lane: String,
    /// Target registry lane.
    pub target_lane: String,
    /// Whether the package identity is preserved across the path.
    pub identity_preserved: bool,
    /// Whether provenance and artifact refs are preserved across the path.
    pub provenance_preserved: bool,
    /// Whether support-class truth is preserved across the path.
    pub support_class_preserved: bool,
    /// Promotion or approval evidence ref.
    pub approval_ref: String,
}

/// Surface alignment flags for all stable discovery consumers.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct SurfaceTruth {
    /// Whether marketplace cards show ranking and abuse reasons.
    pub marketplace_cards_explicit: bool,
    /// Whether search results show ranking and abuse reasons.
    pub search_results_explicit: bool,
    /// Whether detail views show ranking and abuse reasons.
    pub details_views_explicit: bool,
    /// Whether admin review shows ranking and abuse reasons.
    pub admin_review_explicit: bool,
    /// Whether mirrored catalogs show ranking and abuse reasons.
    pub mirrored_catalogs_explicit: bool,
    /// Whether support exports show ranking and abuse reasons.
    pub support_export_explicit: bool,
}

/// Transparency-ready moderation or publisher-status event.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct TransparencyEvent {
    /// Transparency event class.
    pub event_class: String,
    /// Actor or authority that made the decision.
    pub actor_ref: String,
    /// Decision timestamp in RFC 3339 form.
    pub decision_timestamp: String,
    /// Exportable notice or transparency ref.
    pub notice_ref: String,
    /// Whether the event can be included in support/admin export.
    pub exportable: bool,
    /// Whether mirrored/offline catalogs preserve the event.
    pub mirror_visible: bool,
}

/// Derived aggregate summary for discovery ranking and abuse posture.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MarketplaceDiscoveryInspection {
    /// Record kind for the inspection row.
    pub record_kind: String,
    /// Total signed ranking score from typed ranking signals.
    pub ranking_score: i16,
    /// Count of ranking signals present in the packet.
    pub ranking_signal_count: usize,
    /// Count of required ranking signal classes missing from the packet.
    pub missing_ranking_signal_count: usize,
    /// Whether every ranking reason is explainable without private score internals.
    pub ranking_explainable: bool,
    /// Whether raw install count or review volume was used as the primary ranking input.
    pub vanity_metric_used: bool,
    /// Count of required anti-abuse controls present in the packet.
    pub anti_abuse_control_count: usize,
    /// Count of anti-abuse controls in review or suspicious state.
    pub review_control_count: usize,
    /// Count of anti-abuse controls in quarantined, blocked, or revoked state.
    pub blocking_control_count: usize,
    /// Whether all required surface truth rows are explicit.
    pub surfaces_aligned: bool,
    /// Whether enterprise curation paths preserve identity and provenance.
    pub curation_identity_preserved: bool,
    /// Whether all transparency events are attributable and exportable.
    pub transparency_exportable: bool,
}

/// Stability claim after marketplace discovery posture is evaluated.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MarketplaceDiscoveryClaim {
    /// Record kind for the claim row.
    pub record_kind: String,
    /// Claimed tier before automatic narrowing.
    pub claimed_tier: String,
    /// Effective tier after ranking, abuse, quarantine, and curation posture.
    pub effective_tier: String,
    /// Support claim class for consuming surfaces.
    pub support_claim_class: String,
    /// Whether the claimed tier was narrowed.
    pub narrowed: bool,
    /// Machine-readable narrowing reasons.
    pub narrowing_reasons: Vec<String>,
}

/// Top-level stable marketplace discovery packet.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MarketplaceDiscoveryPacket {
    /// Record kind for the packet.
    pub record_kind: String,
    /// Schema version for the packet.
    pub schema_version: u32,
    /// Canonical schema ref for the packet.
    pub schema_ref: String,
    /// Stable package identity.
    pub identity: MarketplacePackageIdentity,
    /// Publisher status.
    pub publisher: PublisherStatus,
    /// Ranking signals.
    pub ranking_signals: Vec<RankingSignal>,
    /// Anti-abuse controls.
    pub anti_abuse_controls: Vec<AntiAbuseControl>,
    /// Quarantine and revocation status.
    pub quarantine_revocation: QuarantineRevocationStatus,
    /// Enterprise curation paths.
    pub curation_paths: Vec<EnterpriseCurationPath>,
    /// Surface truth alignment.
    pub surface_truth: SurfaceTruth,
    /// Transparency events.
    pub transparency_events: Vec<TransparencyEvent>,
    /// Derived inspection row.
    pub inspection: MarketplaceDiscoveryInspection,
    /// Derived stability claim.
    pub claim: MarketplaceDiscoveryClaim,
    /// Derived discovery posture visible in result rows.
    pub discovery_posture_class: String,
}

/// Metadata-safe support/export projection of [`MarketplaceDiscoveryPacket`].
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MarketplaceDiscoverySupportExport {
    /// Record kind for the support/export row.
    pub record_kind: String,
    /// Schema version for the support/export row.
    pub schema_version: u32,
    /// Package id.
    pub package_id: String,
    /// Package version.
    pub package_version: String,
    /// Publisher id.
    pub publisher_id: String,
    /// Publisher tier.
    pub publisher_tier_class: String,
    /// Discovery posture.
    pub discovery_posture_class: String,
    /// Effective stability tier.
    pub effective_tier: String,
    /// Narrowing reasons safe for support export.
    pub narrowing_reasons: Vec<String>,
    /// Ranking signal count.
    pub ranking_signal_count: usize,
    /// Anti-abuse control count.
    pub anti_abuse_control_count: usize,
    /// Current quarantine/revocation status.
    pub quarantine_revocation_class: String,
    /// Whether mirror/offline catalogs preserve abuse and revocation truth.
    pub mirror_safe: bool,
    /// Whether the package is blocked from stable discovery.
    pub blocks_stable_discovery: bool,
}

impl MarketplaceDiscoveryPacket {
    /// Builds a discovery packet from producer input and derives its effective posture.
    ///
    /// # Errors
    ///
    /// Returns an error when the input uses a value outside a closed vocabulary.
    pub fn from_input(input: MarketplaceDiscoveryInput) -> Result<Self, MarketplaceDiscoveryError> {
        validate_input_vocabularies(&input)?;

        let inspection = inspect(&input);
        let mut reasons = derive_narrowing_reasons(&input, &inspection);
        reasons.sort();
        reasons.dedup();

        let effective_tier = derive_effective_tier(&input.claimed_tier, &reasons);
        let discovery_posture_class =
            derive_discovery_posture(&input, &inspection, &effective_tier);
        let claim = MarketplaceDiscoveryClaim {
            record_kind: "marketplace_discovery_claim".to_string(),
            claimed_tier: input.claimed_tier.clone(),
            support_claim_class: support_claim_class(&effective_tier).to_string(),
            narrowed: effective_tier != input.claimed_tier,
            effective_tier,
            narrowing_reasons: reasons,
        };

        Ok(Self {
            record_kind: MARKETPLACE_DISCOVERY_PACKET_RECORD_KIND.to_string(),
            schema_version: MARKETPLACE_DISCOVERY_SCHEMA_VERSION,
            schema_ref: MARKETPLACE_DISCOVERY_SCHEMA_REF.to_string(),
            identity: input.identity,
            publisher: input.publisher,
            ranking_signals: input.ranking_signals,
            anti_abuse_controls: input.anti_abuse_controls,
            quarantine_revocation: input.quarantine_revocation,
            curation_paths: input.curation_paths,
            surface_truth: input.surface_truth,
            transparency_events: input.transparency_events,
            inspection,
            claim,
            discovery_posture_class,
        })
    }

    /// Validates derived invariants that prevent local surface drift.
    ///
    /// # Errors
    ///
    /// Returns every invariant violation found in the packet.
    pub fn validate(&self) -> Result<(), Vec<MarketplaceDiscoveryValidationError>> {
        let mut errors = Vec::new();

        if self.record_kind != MARKETPLACE_DISCOVERY_PACKET_RECORD_KIND {
            errors.push(MarketplaceDiscoveryValidationError::new(
                "record_kind",
                "unexpected marketplace discovery packet record kind",
            ));
        }
        if self.schema_version != MARKETPLACE_DISCOVERY_SCHEMA_VERSION {
            errors.push(MarketplaceDiscoveryValidationError::new(
                "schema_version",
                "unexpected marketplace discovery schema version",
            ));
        }
        if !STABILITY_TIERS.contains(&self.claim.claimed_tier.as_str()) {
            errors.push(MarketplaceDiscoveryValidationError::new(
                "claim.claimed_tier",
                "claimed tier is outside the stable discovery vocabulary",
            ));
        }
        if !STABILITY_TIERS.contains(&self.claim.effective_tier.as_str()) {
            errors.push(MarketplaceDiscoveryValidationError::new(
                "claim.effective_tier",
                "effective tier is outside the stable discovery vocabulary",
            ));
        }
        if !DISCOVERY_POSTURE_CLASSES.contains(&self.discovery_posture_class.as_str()) {
            errors.push(MarketplaceDiscoveryValidationError::new(
                "discovery_posture_class",
                "discovery posture is outside the stable discovery vocabulary",
            ));
        }

        let ranking_score: i16 = self.ranking_signals.iter().map(|s| s.score_delta).sum();
        if ranking_score != self.inspection.ranking_score {
            errors.push(MarketplaceDiscoveryValidationError::new(
                "inspection.ranking_score",
                "ranking score must be derived from typed ranking signals",
            ));
        }

        let missing = missing_required_classes(
            REQUIRED_RANKING_SIGNAL_CLASSES,
            self.ranking_signals.iter().map(|s| s.signal_class.as_str()),
        )
        .len();
        if missing != self.inspection.missing_ranking_signal_count {
            errors.push(MarketplaceDiscoveryValidationError::new(
                "inspection.missing_ranking_signal_count",
                "missing ranking signal count must be derived",
            ));
        }

        if self.claim.effective_tier == "stable" {
            if !self.claim.narrowing_reasons.is_empty() {
                errors.push(MarketplaceDiscoveryValidationError::new(
                    "claim.narrowing_reasons",
                    "stable discovery claims must not retain narrowing reasons",
                ));
            }
            if self.discovery_posture_class != "prominent"
                && self.discovery_posture_class != "ordinary"
            {
                errors.push(MarketplaceDiscoveryValidationError::new(
                    "discovery_posture_class",
                    "stable discovery claims must not render under review, quarantined, or revoked",
                ));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// Projects a serialized discovery packet into its compact support/export row.
///
/// # Errors
///
/// Returns a deserialization error if `payload` is not a discovery packet.
pub fn project_marketplace_discovery(
    payload: &str,
) -> Result<MarketplaceDiscoverySupportExport, MarketplaceDiscoveryError> {
    let packet: MarketplaceDiscoveryPacket = serde_json::from_str(payload)?;
    Ok(project_marketplace_discovery_support_export(&packet))
}

/// Projects a discovery packet into a metadata-safe support/export row.
pub fn project_marketplace_discovery_support_export(
    packet: &MarketplaceDiscoveryPacket,
) -> MarketplaceDiscoverySupportExport {
    let controls_mirror_safe = packet
        .anti_abuse_controls
        .iter()
        .all(|control| control.mirror_safe_continuity);
    MarketplaceDiscoverySupportExport {
        record_kind: MARKETPLACE_DISCOVERY_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: MARKETPLACE_DISCOVERY_SCHEMA_VERSION,
        package_id: packet.identity.package_id.clone(),
        package_version: packet.identity.package_version.clone(),
        publisher_id: packet.publisher.publisher_id.clone(),
        publisher_tier_class: packet.publisher.tier_class.clone(),
        discovery_posture_class: packet.discovery_posture_class.clone(),
        effective_tier: packet.claim.effective_tier.clone(),
        narrowing_reasons: packet.claim.narrowing_reasons.clone(),
        ranking_signal_count: packet.inspection.ranking_signal_count,
        anti_abuse_control_count: packet.inspection.anti_abuse_control_count,
        quarantine_revocation_class: packet.quarantine_revocation.status_class.clone(),
        mirror_safe: controls_mirror_safe
            && packet.quarantine_revocation.mirror_visible
            && packet
                .transparency_events
                .iter()
                .all(|event| event.mirror_visible),
        blocks_stable_discovery: packet.claim.effective_tier == "withdrawn",
    }
}

fn inspect(input: &MarketplaceDiscoveryInput) -> MarketplaceDiscoveryInspection {
    let missing_ranking_signal_count = missing_required_classes(
        REQUIRED_RANKING_SIGNAL_CLASSES,
        input
            .ranking_signals
            .iter()
            .map(|s| s.signal_class.as_str()),
    )
    .len();
    let anti_abuse_control_count = input
        .anti_abuse_controls
        .iter()
        .map(|control| control.control_class.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let review_control_count = input
        .anti_abuse_controls
        .iter()
        .filter(|control| {
            control.state_class == "under_review" || control.state_class == "suspicious"
        })
        .count();
    let blocking_control_count = input
        .anti_abuse_controls
        .iter()
        .filter(|control| {
            control.state_class == "quarantined"
                || control.state_class == "blocked"
                || control.state_class == "revoked"
        })
        .count();

    MarketplaceDiscoveryInspection {
        record_kind: "marketplace_discovery_inspection".to_string(),
        ranking_score: input
            .ranking_signals
            .iter()
            .map(|signal| signal.score_delta)
            .sum(),
        ranking_signal_count: input.ranking_signals.len(),
        missing_ranking_signal_count,
        ranking_explainable: input
            .ranking_signals
            .iter()
            .all(|signal| !signal.reason_chip.is_empty() && !signal.evidence_ref.is_empty()),
        vanity_metric_used: input
            .ranking_signals
            .iter()
            .any(|signal| signal.raw_install_count_primary),
        anti_abuse_control_count,
        review_control_count,
        blocking_control_count,
        surfaces_aligned: input.surface_truth.all_explicit(),
        curation_identity_preserved: input.curation_paths.iter().all(|path| {
            path.identity_preserved
                && path.provenance_preserved
                && path.support_class_preserved
                && !path.approval_ref.is_empty()
        }),
        transparency_exportable: input.transparency_events.iter().all(|event| {
            !event.actor_ref.is_empty()
                && !event.notice_ref.is_empty()
                && event.exportable
                && event.mirror_visible
        }),
    }
}

fn derive_narrowing_reasons(
    input: &MarketplaceDiscoveryInput,
    inspection: &MarketplaceDiscoveryInspection,
) -> Vec<String> {
    let mut reasons = Vec::new();

    if input.identity.discovery_version != MARKETPLACE_DISCOVERY_PUBLISHED_VERSION {
        reasons.push("discovery_version_not_published");
    }
    if inspection.missing_ranking_signal_count > 0 {
        reasons.push("ranking_signal_missing");
    }
    if !inspection.ranking_explainable {
        reasons.push("ranking_reason_not_explainable");
    }
    if inspection.vanity_metric_used {
        reasons.push("ranking_uses_vanity_metric");
    }
    for signal in &input.ranking_signals {
        match signal.signal_class.as_str() {
            "current_version_compatibility" if signal.score_delta <= -20 => {
                reasons.push("stale_compatibility")
            }
            "runtime_health" if signal.score_delta <= -20 => {
                reasons.push("crash_resource_regression")
            }
            "maintenance_freshness" if signal.score_delta <= -20 => {
                reasons.push("maintenance_stale")
            }
            "rollback_uninstall_quality" if signal.score_delta <= -20 => {
                reasons.push("high_rollback_or_uninstall_rate")
            }
            "docs_security_posture" if signal.score_delta <= -20 => {
                reasons.push("docs_security_gap")
            }
            _ => {}
        }
    }

    if !input.publisher.mechanically_sourced || input.publisher.status_model_ref.is_empty() {
        reasons.push("publisher_status_not_mechanical");
    }
    if tier_requires_identity(&input.publisher.tier_class) && !input.publisher.identity_verified {
        reasons.push("publisher_identity_unverified");
    }
    if input.publisher.tier_class == "under_review" {
        reasons.push("publisher_under_review");
    }

    let missing_controls = missing_required_classes(
        REQUIRED_ABUSE_CONTROL_CLASSES,
        input
            .anti_abuse_controls
            .iter()
            .map(|control| control.control_class.as_str()),
    );
    if !missing_controls.is_empty() {
        reasons.push("anti_abuse_control_missing");
    }
    if input.anti_abuse_controls.iter().any(|control| {
        control.user_visible_reason.is_empty()
            || control.admin_visible_reason.is_empty()
            || !control.support_exportable
            || !control.mirror_safe_continuity
    }) {
        reasons.push("abuse_reason_not_visible");
    }
    for control in &input.anti_abuse_controls {
        if (control.control_class == "typosquat_detection"
            || control.control_class == "look_alike_detection")
            && (control.state_class == "under_review" || control.state_class == "suspicious")
        {
            reasons.push("lookalike_or_typosquat_under_review");
        }
        if control.control_class == "review_install_fraud_detection"
            && (control.state_class == "under_review" || control.state_class == "suspicious")
        {
            reasons.push("fraud_review_active");
        }
        if control.state_class == "quarantined" {
            reasons.push("suspicious_package_quarantined");
        }
        if control.control_class == "malware_static_policy_scanning"
            && (control.state_class == "blocked" || control.state_class == "revoked")
        {
            reasons.push("malware_policy_block");
        }
        if control.control_class == "rapid_revocation" && control.state_class == "revoked" {
            reasons.push("revocation_active");
        }
    }

    if input.quarantine_revocation.status_class == "quarantined" {
        reasons.push("suspicious_package_quarantined");
    }
    if input.quarantine_revocation.status_class == "revoked"
        || input.quarantine_revocation.status_class == "emergency_disabled"
    {
        reasons.push("revocation_active");
    }
    if !input.quarantine_revocation.mirror_visible {
        reasons.push("abuse_reason_not_visible");
    }
    if !inspection.curation_identity_preserved {
        reasons.push("enterprise_curation_identity_gap");
    }
    if !inspection.surfaces_aligned {
        reasons.push("surface_visibility_gap");
    }
    if !inspection.transparency_exportable {
        reasons.push("transparency_not_exportable");
    }

    reasons.into_iter().map(str::to_string).collect()
}

fn derive_effective_tier(claimed_tier: &str, reasons: &[String]) -> String {
    if reasons
        .iter()
        .any(|reason| WITHDRAWN_REASONS.contains(&reason.as_str()))
    {
        return "withdrawn".to_string();
    }
    if reasons
        .iter()
        .any(|reason| PREVIEW_REASONS.contains(&reason.as_str()))
    {
        return "preview".to_string();
    }
    if reasons
        .iter()
        .any(|reason| BETA_REASONS.contains(&reason.as_str()))
    {
        return "beta".to_string();
    }
    claimed_tier.to_string()
}

fn derive_discovery_posture(
    input: &MarketplaceDiscoveryInput,
    inspection: &MarketplaceDiscoveryInspection,
    effective_tier: &str,
) -> String {
    if input.quarantine_revocation.status_class == "revoked"
        || input.quarantine_revocation.status_class == "emergency_disabled"
    {
        return "revoked".to_string();
    }
    if input.quarantine_revocation.status_class == "quarantined"
        || inspection.blocking_control_count > 0
    {
        return "quarantined".to_string();
    }
    if effective_tier == "preview" {
        return "under_review".to_string();
    }
    if effective_tier == "beta" {
        return "narrowed".to_string();
    }
    if inspection.ranking_score >= 60
        && (input.publisher.tier_class == "official_pack"
            || input.publisher.tier_class == "verified_publisher"
            || input.publisher.tier_class == "enterprise_approved")
    {
        "prominent".to_string()
    } else {
        "ordinary".to_string()
    }
}

fn support_claim_class(effective_tier: &str) -> &'static str {
    match effective_tier {
        "stable" => "stable_marketplace_discovery_claim",
        "beta" => "beta_marketplace_discovery_narrowed",
        "preview" => "preview_marketplace_discovery_review",
        _ => "withdrawn_marketplace_discovery_blocked",
    }
}

fn missing_required_classes<'a, 'b, I>(required: &'b [&'b str], present: I) -> Vec<&'b str>
where
    I: IntoIterator<Item = &'a str>,
{
    let present = present.into_iter().collect::<BTreeSet<_>>();
    required
        .iter()
        .copied()
        .filter(|class| !present.contains(class))
        .collect()
}

fn tier_requires_identity(tier: &str) -> bool {
    tier == "official_pack" || tier == "verified_publisher" || tier == "enterprise_approved"
}

fn validate_input_vocabularies(
    input: &MarketplaceDiscoveryInput,
) -> Result<(), MarketplaceDiscoveryError> {
    validate_member("claimed_tier", &input.claimed_tier, STABILITY_TIERS)?;
    validate_member(
        "publisher.tier_class",
        &input.publisher.tier_class,
        PUBLISHER_TIER_CLASSES,
    )?;
    validate_member(
        "quarantine_revocation.status_class",
        &input.quarantine_revocation.status_class,
        QUARANTINE_REVOCATION_CLASSES,
    )?;
    for signal in &input.ranking_signals {
        validate_member(
            "ranking_signals.signal_class",
            &signal.signal_class,
            RANKING_SIGNAL_CLASSES,
        )?;
    }
    for control in &input.anti_abuse_controls {
        validate_member(
            "anti_abuse_controls.control_class",
            &control.control_class,
            ABUSE_CONTROL_CLASSES,
        )?;
        validate_member(
            "anti_abuse_controls.state_class",
            &control.state_class,
            ABUSE_STATE_CLASSES,
        )?;
    }
    for path in &input.curation_paths {
        validate_member(
            "curation_paths.source_lane",
            &path.source_lane,
            REGISTRY_LANE_CLASSES,
        )?;
        validate_member(
            "curation_paths.target_lane",
            &path.target_lane,
            REGISTRY_LANE_CLASSES,
        )?;
    }
    for event in &input.transparency_events {
        validate_member(
            "transparency_events.event_class",
            &event.event_class,
            TRANSPARENCY_EVENT_CLASSES,
        )?;
    }
    Ok(())
}

fn validate_member(
    field: &'static str,
    value: &str,
    allowed: &[&str],
) -> Result<(), MarketplaceDiscoveryError> {
    if allowed.contains(&value) {
        Ok(())
    } else {
        Err(MarketplaceDiscoveryError::InvalidVocabulary {
            field,
            value: value.to_string(),
        })
    }
}

impl SurfaceTruth {
    fn all_explicit(&self) -> bool {
        self.marketplace_cards_explicit
            && self.search_results_explicit
            && self.details_views_explicit
            && self.admin_review_explicit
            && self.mirrored_catalogs_explicit
            && self.support_export_explicit
    }
}

/// Error returned while building or projecting a marketplace discovery packet.
#[derive(Debug)]
pub enum MarketplaceDiscoveryError {
    /// A closed-vocabulary field carried an unsupported value.
    InvalidVocabulary {
        /// Field that failed validation.
        field: &'static str,
        /// Unsupported field value.
        value: String,
    },
    /// JSON serialization or deserialization failed.
    Json(serde_json::Error),
}

impl fmt::Display for MarketplaceDiscoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidVocabulary { field, value } => {
                write!(f, "{field} has unsupported value {value:?}")
            }
            Self::Json(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for MarketplaceDiscoveryError {}

impl From<serde_json::Error> for MarketplaceDiscoveryError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

/// Validation error returned when a packet's derived fields drift.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarketplaceDiscoveryValidationError {
    /// Field that failed validation.
    pub field: &'static str,
    /// Human-readable validation message.
    pub message: &'static str,
}

impl MarketplaceDiscoveryValidationError {
    fn new(field: &'static str, message: &'static str) -> Self {
        Self { field, message }
    }
}
