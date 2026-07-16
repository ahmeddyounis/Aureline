//! Implemented M5 post-launch supported-line truth-feed and audience-packet registries.
//!
//! The frozen [supported-line-transparency matrix][matrix] names Aureline's governed post-launch external-proof
//! objects and locks their controlled vocabulary. This module makes the supported-line truth feed operable so
//! external evaluators and support paths consume one current supported-line truth feed instead of hand-assembled
//! fragments: it bundles the already-published proof registries rather than re-synthesizing product truth by hand,
//! and turns the *truth-feed* grammar (one typed feed section per active supported line — a public-proof summary, a
//! migration-scoreboard summary, a transparency snapshot, a correction-history summary, a claim-history summary, and
//! a release-evidence link — each bound to one supported-line identity with its exact build / release-line identity
//! and its stable ID and freshness date, with public-safe correction-history and claim-history summaries separated
//! from internal-only incident / security payloads) and the *audience-packet* grammar (the export-safe packet
//! variant one canonical truth feed is projected into for a named audience — a support bundle, a procurement bundle,
//! or a partner-review bundle) into registry resolvers that produce export-safe, honest projections. Every active
//! stable or LTS-candidate line then resolves to one typed truth feed — the feed section it records, its current
//! claim, its evidence freshness, the linked supported-line-matrix / active-claim / migration-guide / release-evidence
//! refs, and its exact-build provenance, all preserved before a claim widens so a line never keeps a claim ahead of
//! current proof — and to one audience packet — the resolved line identity, the bundled truth-feed reference, the
//! public-safe-versus-internal reference, the packet-scope state, and the active packet note — that the release /
//! help, docs, support, procurement, and partner surfaces can inspect without private shiproom materials, so stale
//! external proof or an internal-only leak stays visible on the active line and a feed that cannot bind a section to
//! its exact build degrades honestly instead of leaving a stale claim to read as still green.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Generate export-safe truth feeds that bundle current public-proof ledger state, migration scoreboards,
//!   transparency snapshots, correction-train summaries, and supported-line claim history with stable IDs and
//!   freshness dates.**
//!   [`resolve_truth_feed_entry`] refuses to read as a clean, registry-bound truth feed
//!   unless it names a canonical registry token, a classified [feed section][M5TruthFeedKind], a
//!   transparency role, covers every [resolution form][M5SupportedLineTruthFeedResolutionForm] (the
//!   canonical object, the accessible summary, and the audit record), publishes every feed field (bundled
//!   evidence rows, linked supported-line-matrix / active-claim / migration-guide / release-evidence refs, current
//!   claim, evidence freshness, and owning roster), preserves its exact-build provenance before a claim
//!   widens, and keeps any feed section bound to its exact build; otherwise it degrades.
//! * **Support audience-specific packet variants for support, procurement, design-partner, or partner review while
//!   preserving one canonical data model and public-safe redaction profile.**
//!   [`line_preserves_rollback_and_diagnostics_before_widening`] rejects a
//!   truth feed whose exact-build provenance is not preserved before widening (a line resuming a wider
//!   claim on stale proof) so it degrades to
//!   [`M5TruthFeedEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof`],
//!   and a public-facing feed section whose published summary outruns current proof degrades the same
//!   way — the structured blocker reason a widen-on-stale-proof attempt must surface. Public-safe and internal-only
//!   variants share one canonical record identity so they never diverge on core supported-line facts.
//! * **Link these feeds out to compatibility reports, known limits, migration guides, and release evidence rather
//!   than duplicating or mutating their content, keeping internal-only incident / security detail out of public-safe
//!   feeds.**
//!   [`resolve_audience_packet_entry`] names a classified [packet scope][M5AudiencePacketScope]
//!   (support-bundle, procurement-bundle, or partner-review-bundle), requires the full
//!   line-identity / bundled-truth-feed / public-safe-versus-internal / packet-scope /
//!   active-note packet object, covers every resolution form, and degrades to
//!   [`M5AudiencePacketEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence`]
//!   when the packet variant would keep a claim ahead of current proof, leak internal-only detail, or let a stale
//!   feed masquerade as current, so an audience packet can never read as trustworthy when it has quietly dropped the
//!   current claim, evidence freshness, migration posture, or correction history its audience depends on.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5SupportedLineTransparencyRole`] role
//! vocabulary and the [`M5SupportedLineTransparencyConsumerSurface`] consumer-surface taxonomy — so the shiproom,
//! release-center, executive-steering, program-governance, diagnostics, docs, CLI, support, and governance
//! surfaces can never fork their own line meaning. Raw secret values and private endpoints stay outside the
//! export boundary.
//!
//! [matrix]: crate::m5_supported_line_transparency_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_supported_line_truth_feed_and_audience_packet_registries,
    seeded_m5_supported_line_truth_feed_and_audience_packet_registries_audience_packet_preview_narrowed,
    seeded_m5_supported_line_truth_feed_and_audience_packet_registries_truth_feed_beta_narrowed,
    M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_supported_line_transparency_matrix::{
    M5SupportedLineTransparencyAccessibilityRoute, M5SupportedLineTransparencyConsumerSurface,
    M5SupportedLineTransparencyDowngradeTrigger, M5SupportedLineTransparencyObject,
    M5SupportedLineTransparencyQualificationClass, M5SupportedLineTransparencyRequiredLabel,
    M5SupportedLineTransparencyRole, M5SupportedLineTransparencyWideningStage,
    M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_DOC_REF,
    M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5SupportedLineTruthFeedAudiencePacketRegistriesPacket`].
pub const M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_supported_line_truth_feed_and_audience_packet_registries";

/// Schema version for M5 line-truth_feed / line-downgrade-packet registry records.
pub const M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_SCHEMA_REF: &str =
    "schemas/program/m5-supported-line-truth-feed-and-audience-packet-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_DOC_REF: &str =
    "docs/release/m5_supported_line_truth_feed_and_audience_packet_registries.md";

/// Repo-relative path of the canonical truth-feed domain schema minted by this lane (the export-safe
/// supported-line truth feed that bundles current public-proof, migration-scoreboard, transparency, and
/// correction/claim-history summaries for one active supported line, with stable IDs and freshness dates and
/// public-safe detail separated from internal-only incident / security payloads).
pub const M5_TRUTH_FEED_DOMAIN_SCHEMA_REF: &str = "schemas/program/m5-truth-feed.schema.json";

/// Repo-relative path of the canonical audience-packet domain schema minted by this lane (the export-safe
/// packet variant — support, procurement, or partner review — derived from one canonical supported-line truth
/// feed, redacting internal-only incident / security detail while still naming the current claim, evidence
/// freshness, migration posture, and correction history for that audience).
pub const M5_AUDIENCE_PACKET_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-audience-packet.schema.json";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-supported-line-truth-feed-and-audience-packet-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-supported-line-truth-feed-and-audience-packet-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-supported-line-truth-feed-and-audience-packet-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/release/m5-supported-line-truth-feed-and-audience-packet-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// line invents a parallel surface set.
pub type M5SupportedLineTruthFeedAudiencePacketRegistriesConsumerSurface =
    M5SupportedLineTransparencyConsumerSurface;

/// One of the three resolution forms every line-truth_feed or line-downgrade-packet entry must hold across so
/// its truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or
/// written to the audit / support record. Minted by this lane because the frozen matrix names the
/// line-truth_feed and line-downgrade *domains* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportedLineTruthFeedResolutionForm {
    /// The canonical resolved line-truth_feed / line-downgrade-packet object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved line discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved line inspectable off-renderer.
    AuditRecord,
}

impl M5SupportedLineTruthFeedResolutionForm {
    /// Every resolution form, in declaration order. A clean entry must cover all three.
    pub const ALL: [Self; 3] = [
        Self::CanonicalObject,
        Self::AccessibleSummary,
        Self::AuditRecord,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalObject => "canonical_object",
            Self::AccessibleSummary => "accessible_summary",
            Self::AuditRecord => "audit_record",
        }
    }
}

/// Controlled feed section a supported-line truth feed bundles for one supported-line identity, so every active
/// line's external proof shares one typed feed rather than hand-assembled fragments re-synthesized per consumer.
/// Minted by this lane because the frozen matrix names the supported lines and their proof domains but not the
/// concrete feed sections a durable truth feed must bundle against exact build / release-line identity — a
/// public-proof summary, a migration-scoreboard summary, a transparency snapshot, a correction-history summary, a
/// claim-history summary, and a release-evidence link. Every classified feed section carries its canonical mode,
/// and the correction-history-summary and claim-history-summary sections are public-facing (they surface directly
/// in the public-safe view partners and procurement read) so their published summary must stay matched to current
/// exact-build proof before the line widens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TruthFeedKind {
    /// The public-proof summary: current public-proof-ledger state (compatibility, benchmark, support-window,
    /// known-limits, deprecation proof) joined to this line's exact-build identity (internal-only detail withheld).
    PublicProofSummary,
    /// The migration-scoreboard summary: current importer/bridge migration posture and unsupported-item categories
    /// scored for this line, linking out to migration guides rather than restating them (internal-only detail withheld).
    MigrationScoreboardSummary,
    /// The transparency snapshot: the current export-safe upstream-health / maintainer-durability posture for this
    /// line, with red-risk dependencies named and internal-only incident detail withheld.
    TransparencySnapshot,
    /// The correction-history summary (public-facing; the customer-facing correction-train summary — what changed,
    /// why, and how it was recovered — whose published summary must match current exact-build proof).
    CorrectionHistorySummary,
    /// The claim-history summary (public-facing; the customer-facing current-claim and claim-history summary, whose
    /// published summary must match current proof so procurement and partners see the claim that actually holds).
    ClaimHistorySummary,
    /// The release-evidence link: the outbound links to compatibility reports, known limits, and release evidence for
    /// this line's exact build, so the feed points at canonical evidence rather than duplicating it (internal-only
    /// detail withheld).
    ReleaseEvidenceLink,
    /// The truth-feed section is unclassified, which is disallowed.
    FeedSectionUnclassified,
}

impl M5TruthFeedKind {
    /// Every truth-feed kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::PublicProofSummary,
        Self::MigrationScoreboardSummary,
        Self::TransparencySnapshot,
        Self::CorrectionHistorySummary,
        Self::ClaimHistorySummary,
        Self::ReleaseEvidenceLink,
        Self::FeedSectionUnclassified,
    ];

    /// The six canonical truth-feed sections every claimed M5 supported line bundles for its external proof.
    pub const CANONICAL_JOURNEYS: [Self; 6] = [
        Self::PublicProofSummary,
        Self::MigrationScoreboardSummary,
        Self::TransparencySnapshot,
        Self::CorrectionHistorySummary,
        Self::ClaimHistorySummary,
        Self::ReleaseEvidenceLink,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicProofSummary => "public_proof_summary",
            Self::MigrationScoreboardSummary => "migration_scoreboard_summary",
            Self::TransparencySnapshot => "transparency_snapshot",
            Self::CorrectionHistorySummary => "correction_history_summary",
            Self::ClaimHistorySummary => "claim_history_summary",
            Self::ReleaseEvidenceLink => "release_evidence_link",
            Self::FeedSectionUnclassified => "feed_section_unclassified",
        }
    }

    /// Whether the item is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::FeedSectionUnclassified)
    }

    /// The canonical mode for this truth-feed kind.
    pub const fn canonical_report_section_mode(self) -> &'static str {
        match self {
            Self::PublicProofSummary => "public_proof_summary_mode",
            Self::MigrationScoreboardSummary => "migration_scoreboard_summary_mode",
            Self::TransparencySnapshot => "transparency_snapshot_mode",
            Self::CorrectionHistorySummary => "correction_history_summary_mode",
            Self::ClaimHistorySummary => "claim_history_summary_mode",
            Self::ReleaseEvidenceLink => "release_evidence_link_mode",
            Self::FeedSectionUnclassified => "",
        }
    }

    /// Whether this truth-feed section is public-facing and so must keep its published
    /// summary matched to current exact-build proof before the line widens.
    pub const fn is_public_facing_line(self) -> bool {
        matches!(
            self,
            Self::CorrectionHistorySummary | Self::ClaimHistorySummary
        )
    }
}

/// Controlled audience-packet scope naming which export-safe packet variant one canonical supported-line truth
/// feed is projected into for a named audience, so a partner, procurement, or support consumer reads one redacted
/// variant of the shared feed rather than a hand-assembled per-audience summary, and shares one registry rather
/// than a hand-copied per-record assumption. Minted by this lane, tracking whether the packet serves a support
/// escalation (support-bundle), a procurement check (procurement-bundle), or a design-partner / partner review
/// (partner-review-bundle). Each scope maps directly to an audience-specific packet variant the implementation
/// requirement names, so support, partner, procurement, and OSS-stewardship reviews follow the same currentness and
/// public-safe redaction rules as internal release control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AudiencePacketScope {
    /// Support-bundle: the export-safe packet variant projected for support escalations, naming the current claim,
    /// evidence freshness, migration posture, and correction history while withholding internal-only incident detail.
    SupportBundle,
    /// Procurement-bundle: the export-safe packet variant projected for procurement checks, naming the current claim,
    /// evidence freshness, migration posture, and correction history while withholding internal-only incident detail.
    ProcurementBundle,
    /// Partner-review-bundle: the export-safe packet variant projected for design-partner and partner review, naming
    /// the current claim, evidence freshness, migration posture, and correction history while withholding internal-only
    /// incident detail.
    PartnerReviewBundle,
    /// The audience-packet scope is unclassified, which is disallowed.
    AudiencePacketScopeUnclassified,
}

impl M5AudiencePacketScope {
    /// Every audience-packet scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SupportBundle,
        Self::ProcurementBundle,
        Self::PartnerReviewBundle,
        Self::AudiencePacketScopeUnclassified,
    ];

    /// The three canonical audience-packet scopes every packet variant must stay distinct across.
    pub const CANONICAL_SCOPES: [Self; 3] = [
        Self::SupportBundle,
        Self::ProcurementBundle,
        Self::PartnerReviewBundle,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportBundle => "support_bundle",
            Self::ProcurementBundle => "procurement_bundle",
            Self::PartnerReviewBundle => "partner_review_bundle",
            Self::AudiencePacketScopeUnclassified => "audience_packet_scope_unclassified",
        }
    }

    /// Whether the comparison scope is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::AudiencePacketScopeUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a line-truth_feed or
/// line-downgrade-packet token's meaning stays stable whether it appears in the release-center, shiproom,
/// executive-steering, program-governance, or a support / export form. Minted by this lane, tracking the
/// first-consumer surfaces the implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportedLineTruthFeedSurfaceContext {
    /// The release-center surface.
    ReleaseCenterSurface,
    /// The shiproom surface.
    ShiproomSurface,
    /// The executive-steering surface.
    ExecutiveSteeringSurface,
    /// The program-governance surface.
    ProgramGovernanceSurface,
    /// The support / export form surface.
    SupportOrExportForm,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5SupportedLineTruthFeedSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReleaseCenterSurface,
        Self::ShiproomSurface,
        Self::ExecutiveSteeringSurface,
        Self::ProgramGovernanceSurface,
        Self::SupportOrExportForm,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::ReleaseCenterSurface,
        Self::ShiproomSurface,
        Self::ExecutiveSteeringSurface,
        Self::ProgramGovernanceSurface,
        Self::SupportOrExportForm,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenterSurface => "release_center_surface",
            Self::ShiproomSurface => "shiproom_surface",
            Self::ExecutiveSteeringSurface => "executive_steering_surface",
            Self::ProgramGovernanceSurface => "program_governance_surface",
            Self::SupportOrExportForm => "support_or_export_form",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part a line-truth_feed or line-downgrade-packet entry must be able to show, so no
/// line journey, repo / bundle / toolchain / deployment row, known-limits packet, rollback target,
/// line-downgrade field, or registry fact is left implicit behind a hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportedLineTruthFeedAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The line journey the entry classifies (line-truth_feed entry).
    CohortArchetype,
    /// The exact repo / journey rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles the entry publishes (line-truth_feed entry).
    RepoBundleToolchainAndDeploymentRows,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The known-limits packet and rollback target the entry preserves before widening (line-truth_feed
    /// entry).
    KnownLimitsAndRollbackTarget,
    /// The line-downgrade fields (line identity, known-limits ledger, rollback target, rehearsal currency,
    /// readiness signoff, support language) the entry publishes (line-downgrade-packet entry).
    CohortEvidenceFields,
    /// The support-identity hint the entry publishes (line-downgrade-packet entry).
    SupportIdentityHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved line truth_feed or line downgrade (both entries).
    PlainLanguageMeaning,
}

impl M5SupportedLineTruthFeedAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::CohortArchetype,
        Self::RepoBundleToolchainAndDeploymentRows,
        Self::ResolutionFormCoverage,
        Self::KnownLimitsAndRollbackTarget,
        Self::CohortEvidenceFields,
        Self::SupportIdentityHint,
        Self::KeyboardRoute,
        Self::PlainLanguageMeaning,
    ];

    /// The three parts every claimed entry must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::RegistryReference];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::RegistryReference => "registry_reference",
            Self::CohortArchetype => "report_section",
            Self::RepoBundleToolchainAndDeploymentRows => {
                "repo_bundle_toolchain_and_deployment_rows"
            }
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::KnownLimitsAndRollbackTarget => "known_limits_and_rollback_target",
            Self::CohortEvidenceFields => "audience_packet_fields",
            Self::SupportIdentityHint => "support_identity_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// line truth_feed, a line-downgrade packet, or a degraded line-truth_feed / line-downgrade-packet entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportedLineTruthFeedNextAction {
    /// Expand the resolved line truth_feed's or line-downgrade packet's plain-language meaning.
    ExpandCohortMeaning,
    /// Inspect the line journey or downgrade scope the entry resolves.
    InspectArchetypeOrScope,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5SupportedLineTruthFeedNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandCohortMeaning,
        Self::InspectArchetypeOrScope,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandCohortMeaning => "expand_line_meaning",
            Self::InspectArchetypeOrScope => "inspect_journey_or_scope",
            Self::CompleteResolutionFormCoverage => "complete_resolution_form_coverage",
            Self::TraceCanonicalRegistry => "trace_canonical_registry",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportedLineTruthFeedExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The line families covered.
    CohortFamilies,
    /// The line journeys carried.
    CohortArchetypes,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The downgrade scopes carried.
    EvidenceScopes,
    /// The render / surface context.
    SurfaceContext,
    /// The line-journey modes carried.
    CohortArchetypeModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5SupportedLineTruthFeedExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::CohortFamilies,
        Self::CohortArchetypes,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::EvidenceScopes,
        Self::SurfaceContext,
        Self::CohortArchetypeModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::CohortFamilies,
        Self::CohortArchetypes,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::CohortFamilies => "line_families",
            Self::CohortArchetypes => "report_sections",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::EvidenceScopes => "comparison_scopes",
            Self::SurfaceContext => "surface_context",
            Self::CohortArchetypeModes => "report_section_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a line-truth_feed entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, widen-without-rollback, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TruthFeedEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the truth_feed means.
    DescriptorTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The line journey is unclassified (not in the resolved taxonomy).
    CohortFeedSectionUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    DescriptorNotBoundToRegistry,
    /// The resolved line-truth_feed object is incomplete: the exact repo / journey rows, bundle IDs, install
    /// topology, toolchain envelope, known limits, rollback target, or diagnostics posture is unstated.
    CohortDescriptorObjectIncomplete,
    /// The line's rollback and diagnostics posture is not preserved before widening (a line widening without
    /// a rollback target and diagnostics posture), or a public-facing line ran its support language ahead of
    /// line proof.
    DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A public-facing line did not keep its support language matched to line proof before widening.
    RollbackOrDiagnosticsNotPreservedForPublicCohort,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5TruthFeedEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::DescriptorTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::CohortFeedSectionUnclassified,
        Self::DescriptorNotBoundToRegistry,
        Self::CohortDescriptorObjectIncomplete,
        Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof,
        Self::ResolutionFormCoverageIncomplete,
        Self::RollbackOrDiagnosticsNotPreservedForPublicCohort,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DescriptorTokenUnstated => "truth_feed_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::CohortFeedSectionUnclassified => "line_feed_section_unclassified",
            Self::DescriptorNotBoundToRegistry => "truth_feed_not_bound_to_registry",
            Self::CohortDescriptorObjectIncomplete => "truth_feed_object_incomplete",
            Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof => {
                "truth_feed_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::RollbackOrDiagnosticsNotPreservedForPublicCohort => {
                "rollback_or_diagnostics_not_preserved_for_public_line"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5SupportedLineTruthFeedNextAction {
        match self {
            Self::DescriptorTokenUnstated | Self::DescriptorNotBoundToRegistry => {
                M5SupportedLineTruthFeedNextAction::TraceCanonicalRegistry
            }
            Self::CohortFeedSectionUnclassified
            | Self::CohortDescriptorObjectIncomplete
            | Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof => {
                M5SupportedLineTruthFeedNextAction::InspectArchetypeOrScope
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5SupportedLineTruthFeedNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::RollbackOrDiagnosticsNotPreservedForPublicCohort
            | Self::ProofStale => M5SupportedLineTruthFeedNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5SupportedLineTransparencyDowngradeTrigger {
        match self {
            Self::DescriptorTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::ResolutionFormCoverageIncomplete
            | Self::DescriptorNotBoundToRegistry => {
                M5SupportedLineTransparencyDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::CohortFeedSectionUnclassified | Self::CohortDescriptorObjectIncomplete => {
                M5SupportedLineTransparencyDowngradeTrigger::FreshnessWindowUnstated
            }
            Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof
            | Self::RollbackOrDiagnosticsNotPreservedForPublicCohort => {
                M5SupportedLineTransparencyDowngradeTrigger::WidenedClaimOnStalePublicProof
            }
            Self::ProofStale => M5SupportedLineTransparencyDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a line-downgrade-packet entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AudiencePacketEntryDegradeReason {
    /// The canonical registry token name is unstated.
    EvidenceTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The downgrade scope is unclassified (not in the resolved taxonomy).
    EvidenceScopeUnclassified,
    /// The line downgrade would run partner / public support language ahead of line proof, hide the line
    /// downgrade, let a known-limits gap masquerade as covered, or it dropped one of the required line-downgrade
    /// fields (line identity, known-limits ledger, rollback target, rehearsal currency, readiness signoff,
    /// support language, last widening revision).
    CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence,
    /// The canonical / accessible / audit resolution-form coverage of the downgrade is incomplete.
    EvidenceFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5AudiencePacketEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EvidenceTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::EvidenceScopeUnclassified,
        Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence,
        Self::EvidenceFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceTokenUnstated => "comparison_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::EvidenceScopeUnclassified => "comparison_audience_packet_scope_unclassified",
            Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence => {
                "audience_packet_runs_support_ahead_of_proof_or_drops_audience_packet"
            }
            Self::EvidenceFormCoverageIncomplete => "comparison_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5SupportedLineTruthFeedNextAction {
        match self {
            Self::EvidenceTokenUnstated => {
                M5SupportedLineTruthFeedNextAction::TraceCanonicalRegistry
            }
            Self::EvidenceScopeUnclassified
            | Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence => {
                M5SupportedLineTruthFeedNextAction::InspectArchetypeOrScope
            }
            Self::EvidenceFormCoverageIncomplete => {
                M5SupportedLineTruthFeedNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5SupportedLineTruthFeedNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5SupportedLineTransparencyDowngradeTrigger {
        match self {
            Self::EvidenceTokenUnstated => {
                M5SupportedLineTransparencyDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::EvidenceScopeUnclassified => {
                M5SupportedLineTransparencyDowngradeTrigger::ExportClassUnstated
            }
            Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence => {
                M5SupportedLineTransparencyDowngradeTrigger::RanSupportLanguageAheadOfPublicProof
            }
            Self::EvidenceFormCoverageIncomplete => {
                M5SupportedLineTransparencyDowngradeTrigger::ImpliedGreenWhileProofOrArchiveWasStale
            }
            Self::ProofStale => M5SupportedLineTransparencyDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_truth_feed_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TruthFeedEntryResolutionInput {
    /// Stable identity of the line-truth_feed-registry entry.
    pub entry_id: String,
    /// The stable line-binding ID this truth_feed binds to (e.g. `launch.line.public-preview`); empty means
    /// unstated.
    pub line_binding_id: String,
    /// The canonical registry token name (e.g. `line.truth_feed.claim_history_summary`); empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5SupportedLineTransparencyRole,
    /// The line journey this entry classifies.
    pub report_section: M5TruthFeedKind,
    /// The render / surface context.
    pub surface_context: M5SupportedLineTruthFeedSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5SupportedLineTruthFeedResolutionForm>,
    /// The published exact repo / journey rows; empty means unstated.
    pub exact_repo_journey_rows: String,
    /// The published bundle IDs; empty means unstated.
    pub bundle_ids: String,
    /// The published install topology; empty means unstated.
    pub install_topology: String,
    /// The published toolchain envelope; empty means unstated.
    pub toolchain_envelope: String,
    /// The published known limits; empty means unstated.
    pub known_limits: String,
    /// The published rollback target; empty means unstated.
    pub rollback_target: String,
    /// The published diagnostics posture; empty means unstated.
    pub diagnostics_posture: String,
    /// True when the behavior traces to the line-truth_feed registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the line's rollback and diagnostics posture is preserved before widening (a hard invariant
    /// when `false`).
    pub rollback_and_diagnostics_bounded: bool,
    /// True when this line's journey is public-facing.
    pub is_public_facing_line: bool,
    /// True when partner / public support language is matched to line proof before a public-facing line
    /// widens.
    pub support_language_matches_line_proof: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe line-truth_feed-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedTruthFeedEntry {
    /// Stable identity of the line-truth_feed-registry entry.
    pub entry_id: String,
    /// The stable line-binding ID this truth_feed binds to.
    pub line_binding_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the downgrade snapshot and signoff before widening.
    pub semantic_role_must_preserve_downgrade_snapshot_and_signoff_before_widening: bool,
    /// The line-journey token named by the entry.
    pub report_section: String,
    /// Whether the line journey is classified into the resolved taxonomy.
    pub report_section_is_classified: bool,
    /// The canonical mode for the entry's line journey.
    pub canonical_report_section_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published exact repo / journey rows.
    pub exact_repo_journey_rows: String,
    /// The published bundle IDs.
    pub bundle_ids: String,
    /// The published install topology.
    pub install_topology: String,
    /// The published toolchain envelope.
    pub toolchain_envelope: String,
    /// The published known limits.
    pub known_limits: String,
    /// The published rollback target.
    pub rollback_target: String,
    /// The published diagnostics posture.
    pub diagnostics_posture: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved line-truth_feed object publishes every required field.
    pub truth_feed_object_complete: bool,
    /// Whether the entry traces to the line-truth_feed registry.
    pub bound_to_registry: bool,
    /// Whether the line's rollback and diagnostics posture stays preserved before widening.
    pub rollback_and_diagnostics_bounded: bool,
    /// Whether this line's journey is public-facing.
    pub is_public_facing_line: bool,
    /// Whether partner / public support language is matched to line proof before widening.
    pub support_language_matches_line_proof: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5TruthFeedEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5SupportedLineTruthFeedNextAction,
    /// Whether the truth_feed resolves to one typed object across every claimed line (clean entry naming every
    /// fact).
    pub truth_feed_resolves_across_lines: bool,
}

impl M5ResolvedTruthFeedEntry {
    /// Whether this line-truth_feed entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_audience_packet_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AudiencePacketEntryResolutionInput {
    /// Stable identity of the line-downgrade-packet entry.
    pub entry_id: String,
    /// The stable downgrade-ref this record binds to; empty means unstated.
    pub comparison_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5SupportedLineTransparencyRole,
    /// The downgrade scope this record must resolve its line proof from.
    pub comparison_scope: M5AudiencePacketScope,
    /// The render / surface context.
    pub surface_context: M5SupportedLineTruthFeedSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5SupportedLineTruthFeedResolutionForm>,
    /// The published resolved line identity; empty means missing.
    pub resolved_line_identity: String,
    /// The published known-limits ledger; empty means missing.
    pub known_limits_ledger: String,
    /// The published rollback-target reference; empty means missing.
    pub rollback_target_reference: String,
    /// The published rehearsal-currency state; empty means missing.
    pub rehearsal_currency_state: String,
    /// The published readiness-signoff state; empty means missing.
    pub readiness_signoff_state: String,
    /// The published line-bound support-language reference; empty means missing.
    pub support_language_reference: String,
    /// The published last widening revision; empty means missing.
    pub last_widening_revision: String,
    /// True when the record keeps the line downgrade visible.
    pub keeps_audience_packet_visible: bool,
    /// True when the downgrade is truthful (never claims a clean packet over hidden line downgrade).
    pub comparison_is_truthful: bool,
    /// True when partner / public support language is present on this record.
    pub support_language_present: bool,
    /// True when the support language is bound to line proof rather than running ahead of it.
    pub support_language_bound_to_proof: bool,
    /// True when a known-limits gap is present on this record.
    pub known_limits_gap_present: bool,
    /// True when a known-limits gap is flagged rather than masquerading as covered.
    pub known_limits_gap_flagged: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe line-downgrade-packet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedAudiencePacketEntry {
    /// Stable identity of the line-downgrade-packet entry.
    pub entry_id: String,
    /// The stable downgrade-ref this record binds to.
    pub comparison_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the downgrade snapshot and signoff before widening.
    pub semantic_role_must_preserve_downgrade_snapshot_and_signoff_before_widening: bool,
    /// The downgrade-scope token named by the entry.
    pub comparison_scope: String,
    /// Whether the downgrade scope is classified into the resolved taxonomy.
    pub comparison_scope_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published resolved line identity.
    pub resolved_line_identity: String,
    /// The published known-limits ledger.
    pub known_limits_ledger: String,
    /// The published rollback-target reference.
    pub rollback_target_reference: String,
    /// The published rehearsal-currency state.
    pub rehearsal_currency_state: String,
    /// The published readiness-signoff state.
    pub readiness_signoff_state: String,
    /// The published line-bound support-language reference.
    pub support_language_reference: String,
    /// The published last widening revision.
    pub last_widening_revision: String,
    /// Whether the record keeps the line downgrade visible.
    pub keeps_audience_packet_visible: bool,
    /// Whether the downgrade is truthful.
    pub comparison_is_truthful: bool,
    /// Whether partner / public support language is present on this build.
    pub support_language_present: bool,
    /// Whether the support language is bound to line proof rather than running ahead of it.
    pub support_language_bound_to_proof: bool,
    /// Whether a known-limits gap is present on this record.
    pub known_limits_gap_present: bool,
    /// Whether a known-limits gap is flagged rather than masquerading as covered.
    pub known_limits_gap_flagged: bool,
    /// Whether the record stays honest (line downgrade visible, support language bound to proof, known-limits
    /// gap flagged).
    pub audience_packet_stays_honest: bool,
    /// Whether the entry provides the complete line-downgrade object (line identity, known-limits ledger,
    /// rollback target, rehearsal currency, readiness signoff, support language, last widening revision).
    pub provides_complete_audience_packet: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5AudiencePacketEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5SupportedLineTruthFeedNextAction,
    /// Whether the line downgrade is safe on every claimed line (clean entry naming every fact).
    pub comparison_safe_on_every_line: bool,
}

impl M5ResolvedAudiencePacketEntry {
    /// Whether this line-downgrade-packet entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5SupportedLineTruthFeedResolutionError {
    /// The line-truth_feed-entry id was empty.
    EmptyCohortDescriptorEntryId,
    /// The line-downgrade-packet-entry id was empty.
    EmptyCohortEvidencePacketEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5SupportedLineTruthFeedResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyCohortDescriptorEntryId => "empty_truth_feed_entry_id",
            Self::EmptyCohortEvidencePacketEntryId => "empty_audience_packet_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5SupportedLineTruthFeedResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 line-truth_feed / line-downgrade-packet registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5SupportedLineTruthFeedResolutionError {}

fn form_tokens(forms: &[M5SupportedLineTruthFeedResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5SupportedLineTruthFeedResolutionForm]) -> bool {
    let present: BTreeSet<M5SupportedLineTruthFeedResolutionForm> = forms.iter().copied().collect();
    M5SupportedLineTruthFeedResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved line-truth_feed object publishes every required field: classified line journey,
/// exact repo / journey rows, bundle IDs, install topology, toolchain envelope, known limits, rollback target,
/// and diagnostics posture. An unclassified journey or any empty field never resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn truth_feed_object_is_complete(
    journey: M5TruthFeedKind,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> bool {
    journey.is_classified()
        && !exact_repo_journey_rows.trim().is_empty()
        && !bundle_ids.trim().is_empty()
        && !install_topology.trim().is_empty()
        && !toolchain_envelope.trim().is_empty()
        && !known_limits.trim().is_empty()
        && !rollback_target.trim().is_empty()
        && !diagnostics_posture.trim().is_empty()
}

/// Whether the line truth_feed keeps a line from widening without preserving its rollback and diagnostics
/// posture: the journey must be classified, the rollback and diagnostics posture must be preserved before
/// widening, and a public-facing line must keep its support language matched to line proof. An unclassified
/// journey, an unpreserved rollback / diagnostics posture, or partner / public support language running ahead
/// of proof never matches.
pub fn line_preserves_rollback_and_diagnostics_before_widening(
    journey: M5TruthFeedKind,
    rollback_and_diagnostics_bounded: bool,
    is_public_facing_line: bool,
    support_language_matches_line_proof: bool,
) -> bool {
    journey.is_classified()
        && rollback_and_diagnostics_bounded
        && (!is_public_facing_line || support_language_matches_line_proof)
}

/// Whether a line-downgrade packet stays honest: the scope must be classified, the downgrade must be truthful,
/// it must keep the line downgrade visible, any partner / public support language must be bound to line proof
/// rather than running ahead of it, and any known-limits gap must be flagged rather than masquerade as covered.
pub fn audience_packet_stays_honest(
    scope: M5AudiencePacketScope,
    comparison_is_truthful: bool,
    keeps_audience_packet_visible: bool,
    support_language_present: bool,
    support_language_bound_to_proof: bool,
    known_limits_gap_present: bool,
    known_limits_gap_flagged: bool,
) -> bool {
    scope.is_classified()
        && comparison_is_truthful
        && keeps_audience_packet_visible
        && (!support_language_present || support_language_bound_to_proof)
        && (!known_limits_gap_present || known_limits_gap_flagged)
}

/// Resolves a line-truth_feed-registry entry so it stays bound to the line-truth_feed registry: the entry
/// names its canonical token, semantic role, and line journey, covers all three resolution forms, publishes
/// a complete truth_feed object (exact repo / journey rows, bundle IDs, install topology, toolchain envelope,
/// known limits, rollback target, diagnostics posture), preserves its rollback and diagnostics posture before
/// widening so a line never widens without it, and keeps a public-facing line's support language matched to
/// line proof.
pub fn resolve_truth_feed_entry(
    input: M5TruthFeedEntryResolutionInput,
) -> Result<M5ResolvedTruthFeedEntry, M5SupportedLineTruthFeedResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5SupportedLineTruthFeedResolutionError::EmptyCohortDescriptorEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.line_binding_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.exact_repo_journey_rows)
        || string_is_forbidden(&input.bundle_ids)
        || string_is_forbidden(&input.install_topology)
        || string_is_forbidden(&input.toolchain_envelope)
        || string_is_forbidden(&input.known_limits)
        || string_is_forbidden(&input.rollback_target)
        || string_is_forbidden(&input.diagnostics_posture)
    {
        return Err(M5SupportedLineTruthFeedResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = truth_feed_object_is_complete(
        input.report_section,
        &input.exact_repo_journey_rows,
        &input.bundle_ids,
        &input.install_topology,
        &input.toolchain_envelope,
        &input.known_limits,
        &input.rollback_target,
        &input.diagnostics_posture,
    );
    let preserve_ok = line_preserves_rollback_and_diagnostics_before_widening(
        input.report_section,
        input.rollback_and_diagnostics_bounded,
        input.is_public_facing_line,
        input.support_language_matches_line_proof,
    );
    let support_undisclosed =
        input.is_public_facing_line && !input.support_language_matches_line_proof;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5TruthFeedEntryDegradeReason::DescriptorTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5TruthFeedEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.report_section.is_classified() {
        Some(M5TruthFeedEntryDegradeReason::CohortFeedSectionUnclassified)
    } else if !input.bound_to_registry {
        Some(M5TruthFeedEntryDegradeReason::DescriptorNotBoundToRegistry)
    } else if !object_complete {
        Some(M5TruthFeedEntryDegradeReason::CohortDescriptorObjectIncomplete)
    } else if !preserve_ok {
        Some(M5TruthFeedEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof)
    } else if !all_forms {
        Some(M5TruthFeedEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if support_undisclosed {
        Some(M5TruthFeedEntryDegradeReason::RollbackOrDiagnosticsNotPreservedForPublicCohort)
    } else if !input.proof_fresh {
        Some(M5TruthFeedEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5SupportedLineTruthFeedNextAction::ExpandCohortMeaning,
    };

    Ok(M5ResolvedTruthFeedEntry {
        entry_id: input.entry_id,
        line_binding_id: input.line_binding_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_downgrade_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_preserve_evidence_snapshot_and_signoff_before_widening(),
        report_section: input.report_section.as_str().to_owned(),
        report_section_is_classified: input.report_section.is_classified(),
        canonical_report_section_mode: input
            .report_section
            .canonical_report_section_mode()
            .to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        exact_repo_journey_rows: input.exact_repo_journey_rows,
        bundle_ids: input.bundle_ids,
        install_topology: input.install_topology,
        toolchain_envelope: input.toolchain_envelope,
        known_limits: input.known_limits,
        rollback_target: input.rollback_target,
        diagnostics_posture: input.diagnostics_posture,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        truth_feed_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        rollback_and_diagnostics_bounded: input.rollback_and_diagnostics_bounded,
        is_public_facing_line: input.is_public_facing_line,
        support_language_matches_line_proof: input.support_language_matches_line_proof,
        degrade_reason,
        next_action,
        truth_feed_resolves_across_lines: degrade_reason.is_none(),
    })
}

/// Resolves a line-downgrade-packet entry so its downgrade stays safe: the entry names its canonical token,
/// semantic role, and downgrade scope, covers all three resolution forms, provides the complete line-identity /
/// known-limits-ledger / rollback-target / rehearsal-currency / readiness-signoff / support-language /
/// last-widening-revision line-downgrade object, and degrades honestly when the downgrade would run partner /
/// public support language ahead of line proof, hide the line downgrade, or let a known-limits gap masquerade
/// as covered.
pub fn resolve_audience_packet_entry(
    input: M5AudiencePacketEntryResolutionInput,
) -> Result<M5ResolvedAudiencePacketEntry, M5SupportedLineTruthFeedResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5SupportedLineTruthFeedResolutionError::EmptyCohortEvidencePacketEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.comparison_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.resolved_line_identity)
        || string_is_forbidden(&input.known_limits_ledger)
        || string_is_forbidden(&input.rollback_target_reference)
        || string_is_forbidden(&input.rehearsal_currency_state)
        || string_is_forbidden(&input.readiness_signoff_state)
        || string_is_forbidden(&input.support_language_reference)
        || string_is_forbidden(&input.last_widening_revision)
    {
        return Err(M5SupportedLineTruthFeedResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let record_stays_honest = audience_packet_stays_honest(
        input.comparison_scope,
        input.comparison_is_truthful,
        input.keeps_audience_packet_visible,
        input.support_language_present,
        input.support_language_bound_to_proof,
        input.known_limits_gap_present,
        input.known_limits_gap_flagged,
    );
    let provides_record = input.comparison_scope.is_classified()
        && !input.resolved_line_identity.trim().is_empty()
        && !input.known_limits_ledger.trim().is_empty()
        && !input.rollback_target_reference.trim().is_empty()
        && !input.rehearsal_currency_state.trim().is_empty()
        && !input.readiness_signoff_state.trim().is_empty()
        && !input.support_language_reference.trim().is_empty()
        && !input.last_widening_revision.trim().is_empty()
        && record_stays_honest;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5AudiencePacketEntryDegradeReason::EvidenceTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5AudiencePacketEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.comparison_scope.is_classified() {
        Some(M5AudiencePacketEntryDegradeReason::EvidenceScopeUnclassified)
    } else if !provides_record {
        Some(M5AudiencePacketEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence)
    } else if !all_forms {
        Some(M5AudiencePacketEntryDegradeReason::EvidenceFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5AudiencePacketEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5SupportedLineTruthFeedNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedAudiencePacketEntry {
        entry_id: input.entry_id,
        comparison_ref: input.comparison_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_downgrade_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_preserve_evidence_snapshot_and_signoff_before_widening(),
        comparison_scope: input.comparison_scope.as_str().to_owned(),
        comparison_scope_is_classified: input.comparison_scope.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        resolved_line_identity: input.resolved_line_identity,
        known_limits_ledger: input.known_limits_ledger,
        rollback_target_reference: input.rollback_target_reference,
        rehearsal_currency_state: input.rehearsal_currency_state,
        readiness_signoff_state: input.readiness_signoff_state,
        support_language_reference: input.support_language_reference,
        last_widening_revision: input.last_widening_revision,
        keeps_audience_packet_visible: input.keeps_audience_packet_visible,
        comparison_is_truthful: input.comparison_is_truthful,
        support_language_present: input.support_language_present,
        support_language_bound_to_proof: input.support_language_bound_to_proof,
        known_limits_gap_present: input.known_limits_gap_present,
        known_limits_gap_flagged: input.known_limits_gap_flagged,
        audience_packet_stays_honest: record_stays_honest,
        provides_complete_audience_packet: provides_record,
        degrade_reason,
        next_action,
        comparison_safe_on_every_line: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved line-truth_feed and line-downgrade-packet
/// entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineTruthFeedAudiencePacketRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5SupportedLineTruthFeedAudiencePacketRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5SupportedLineTransparencyQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Widening stages this row keeps the same truth across.
    pub widening_stages: Vec<M5SupportedLineTransparencyWideningStage>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5SupportedLineTransparencyRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5SupportedLineTransparencyAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5SupportedLineTruthFeedAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5SupportedLineTruthFeedExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5SupportedLineTransparencyDowngradeTrigger>,
    /// Resolved line-truth_feed-registry examples.
    pub truth_feed_entries: Vec<M5ResolvedTruthFeedEntry>,
    /// Resolved line-downgrade-packet examples.
    pub audience_packet_entries: Vec<M5ResolvedAudiencePacketEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the line-truth_feed and
    /// line-downgrade-packet domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never widens a line without current rollback and diagnostics downgrade. MUST be
    /// `false`.
    pub widens_a_line_without_current_rollback_and_diagnostics_downgrade: bool,
    /// Hard invariant: this row never runs partner or public support language ahead of line proof. MUST be
    /// `false`.
    pub runs_partner_or_public_support_language_ahead_of_line_proof: bool,
    /// Hard invariant: this row never hides the rollback target or diagnostics posture before widening. MUST be
    /// `false`.
    pub hides_the_rollback_target_or_diagnostics_posture_before_widening: bool,
    /// Hard invariant: this row never collapses distinct line downgrade classes into one lane. MUST be `false`.
    pub collapses_distinct_audience_packet_classes_into_one_lane: bool,
}

impl M5SupportedLineTruthFeedAudiencePacketRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5SupportedLineTruthFeedAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5SupportedLineTruthFeedAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5SupportedLineTruthFeedExportField> =
            self.export_fields.iter().copied().collect();
        M5SupportedLineTruthFeedExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.widens_a_line_without_current_rollback_and_diagnostics_downgrade
            && !self.runs_partner_or_public_support_language_ahead_of_line_proof
            && !self.hides_the_rollback_target_or_diagnostics_posture_before_widening
            && !self.collapses_distinct_audience_packet_classes_into_one_lane
    }

    /// True when a clean line-truth_feed entry preserves registry-bound truth: it traces to the registry,
    /// keeps a classified line journey, publishes a complete truth_feed object, preserves its rollback and
    /// diagnostics posture, covers all three resolution forms, and keeps a public-facing line's support
    /// language matched to proof.
    fn truth_feed_is_honest(ex: &M5ResolvedTruthFeedEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.report_section_is_classified
                && ex.truth_feed_object_complete
                && ex.rollback_and_diagnostics_bounded
                && ex.covers_all_resolution_forms
                && (!ex.is_public_facing_line || ex.support_language_matches_line_proof))
    }

    /// True when a clean line-downgrade-packet entry preserves a safe packet: it keeps a classified downgrade
    /// scope, provides the complete line-downgrade object, stays honest, and covers all three resolution forms.
    fn downgrade_is_honest(ex: &M5ResolvedAudiencePacketEntry) -> bool {
        !ex.is_clean()
            || (ex.comparison_scope_is_classified
                && ex.provides_complete_audience_packet
                && ex.audience_packet_stays_honest
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.truth_feed_entries
            .iter()
            .all(Self::truth_feed_is_honest)
            && self
                .audience_packet_entries
                .iter()
                .all(Self::downgrade_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineTruthFeedAudiencePacketRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Cohort-journey tokens (minted by this lane).
    pub report_section_kinds: Vec<String>,
    /// Evidence-scope tokens (minted by this lane).
    pub comparison_scopes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Cohort-truth_feed-entry degrade-reason tokens.
    pub truth_feed_degrade_reasons: Vec<String>,
    /// Cohort-downgrade-packet-entry degrade-reason tokens.
    pub audience_packet_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5SupportedLineTruthFeedAudiencePacketRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5SupportedLineTransparencyRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5SupportedLineTruthFeedResolutionForm::ALL, |v| v.as_str()),
            report_section_kinds: tokens(&M5TruthFeedKind::ALL, |v| v.as_str()),
            comparison_scopes: tokens(&M5AudiencePacketScope::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5SupportedLineTruthFeedSurfaceContext::ALL, |v| v.as_str()),
            truth_feed_degrade_reasons: tokens(&M5TruthFeedEntryDegradeReason::ALL, |v| v.as_str()),
            audience_packet_degrade_reasons: tokens(
                &M5AudiencePacketEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5SupportedLineTruthFeedAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5SupportedLineTruthFeedNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5SupportedLineTruthFeedExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5SupportedLineTransparencyConsumerSurface::ALL, |v| {
                v.as_str()
            }),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineTruthFeedAudiencePacketRegistriesGovernanceReview {
    /// The truth_feed registry names a canonical token, semantic role, and line journey for every entry.
    pub truth_feed_registry_names_token_role_and_journey: bool,
    /// Every claimed line resolves to one typed line-truth_feed object from the shared registry, not
    /// per-entry reconstruction.
    pub line_resolves_to_typed_truth_feed_from_shared_registry: bool,
    /// The exact repo / journey rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles are published for every resolved truth_feed.
    pub repo_bundle_toolchain_and_deployment_rows_published: bool,
    /// Cohorts cannot widen without preserving rollback and diagnostics posture before widening.
    pub lines_cannot_widen_without_rollback_and_diagnostics: bool,
    /// The line downgrade keeps the line proof visible and binds partner / public support language to it.
    pub audience_packet_keeps_proof_visible_and_binds_support_language: bool,
    /// Partner / public support language stays matched to line proof for every public-facing line.
    pub support_language_matched_to_line_proof_for_public_lines: bool,
    /// Every line-truth_feed and line-downgrade-packet entry covers the canonical / accessible / audit
    /// resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Cohort-truth_feed and line-downgrade-packet behavior stay bound to the shared registries rather than
    /// hand-copied per line.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Shiproom, release center, executive steering, and program governance read a single line source.
    pub shiproom_release_center_executive_steering_and_program_governance_read_single_source: bool,
    /// A widen-without-rollback attempt, an incomplete object, or hidden line downgrade is caught by fixtures
    /// before release downgrade turns green.
    pub truth_feed_or_downgrade_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineTruthFeedAudiencePacketRegistriesConsumerProjection {
    /// Shiproom and release center consume the shared line-truth_feed registry.
    pub shiproom_and_release_center_consume_shared_registries: bool,
    /// Executive steering and program governance consume the shared line-downgrade registry.
    pub executive_steering_and_program_governance_consume_shared_registries: bool,
    /// Diagnostics and public proof consume the shared registries.
    pub diagnostics_and_public_proof_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical line-truth_feed and line-downgrade-packet domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical line-truth_feed / line-downgrade-packet registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineTruthFeedAudiencePacketRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineTruthFeedAudiencePacketRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting line audit for the lane.
    pub line_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SupportedLineTruthFeedAudiencePacketRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SupportedLineTruthFeedAudiencePacketRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SupportedLineTruthFeedAudiencePacketRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SupportedLineTruthFeedAudiencePacketRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SupportedLineTruthFeedAudiencePacketRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SupportedLineTruthFeedAudiencePacketRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SupportedLineTruthFeedAudiencePacketRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SupportedLineTruthFeedAudiencePacketRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 line-truth_feed and line-downgrade-packet registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineTruthFeedAudiencePacketRegistriesPacket {
    /// Record kind; must equal [`M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SupportedLineTruthFeedAudiencePacketRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SupportedLineTruthFeedAudiencePacketRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SupportedLineTruthFeedAudiencePacketRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SupportedLineTruthFeedAudiencePacketRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SupportedLineTruthFeedAudiencePacketRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SupportedLineTruthFeedAudiencePacketRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SupportedLineTruthFeedAudiencePacketRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5SupportedLineTruthFeedAudiencePacketRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_RECORD_KIND
                .to_owned(),
            schema_version: M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_SCHEMA_VERSION,
            packet_id: input.packet_id,
            registries_label: input.registries_label,
            registry_rows: input.registry_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the registries-packet invariants.
    pub fn validate(&self) -> Vec<M5SupportedLineTruthFeedAudiencePacketRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_RECORD_KIND {
            violations
                .push(M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version
            != M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_SCHEMA_VERSION
        {
            violations.push(
                M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::WrongSchemaVersion,
            );
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations
                .push(M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(
                M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::VocabularySetDrift,
            );
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 line-truth_feed / line-downgrade-packet registries packet serializes"),
        ) {
            violations.push(
                M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::RawMaterialInExport,
            );
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 line-truth_feed / line-downgrade-packet registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,truth_feed_entries,audience_packet_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .truth_feed_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.audience_packet_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.truth_feed_entries.len(),
                row.audience_packet_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Supported-Line Truth-Feed and Audience-Packet Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Feed sections: {}\n",
            self.vocabulary_set.report_section_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Resolution forms: {}\n",
            self.vocabulary_set.resolution_forms.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last audit: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.registry_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Truth-feed entries: {} / audience-packet entries: {}\n",
                row.truth_feed_entries.len(),
                row.audience_packet_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry line reference table generated from the registry, so docs and shiproom runbooks
    /// render the same journey-mode / repo-rows / bundle-ids / install-topology / toolchain-envelope /
    /// rollback-target truth the resolvers produced rather than a hand-copied line table. Only clean,
    /// registry-bound line-truth_feed entries are listed.
    pub fn render_truth_feed_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| line_binding_id | journey_mode | exact_repo_journey_rows | bundle_ids | install_topology | toolchain_envelope | rollback_target |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.truth_feed_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.line_binding_id,
                    ex.canonical_report_section_mode,
                    ex.exact_repo_journey_rows,
                    ex.bundle_ids,
                    ex.install_topology,
                    ex.toolchain_envelope,
                    ex.rollback_target
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5SupportedLineTruthFeedAudiencePacketRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SupportedLineTruthFeedAudiencePacketRegistriesViolation>),
}

impl fmt::Display for M5SupportedLineTruthFeedAudiencePacketRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 line-truth_feed / line-downgrade-packet registries export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 line-truth_feed / line-downgrade-packet registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SupportedLineTruthFeedAudiencePacketRegistriesArtifactError {}

/// Validation failures emitted by [`M5SupportedLineTruthFeedAudiencePacketRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SupportedLineTruthFeedAudiencePacketRegistriesViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// The registries packet declares no rows.
    NoRegistryRows,
    /// A registry row is incomplete.
    RegistryRowIncomplete,
    /// A registry row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A registry row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A registry row does not point at both the line-truth_feed and line-downgrade-packet domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, widen-without-rollback, field-incomplete,
    /// form-incomplete, or a line-downgrade entry missing the complete downgrade object).
    DishonestExample,
    /// A registry row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Cohort-truth_feed-resolution is not proven: clean truth_feed entries do not cover the canonical line
    /// journeys or the first release-center / shiproom / executive-steering / program-governance / support
    /// surfaces, no object-incomplete example degrades, or a clean truth_feed entry published an incomplete
    /// object.
    CohortDescriptorResolutionNotProven,
    /// Rollback-and-diagnostics-preservation is not proven: no widen-without-rollback example and no unbound
    /// example degrade, no clean bounded truth_feed entry is present, or a clean truth_feed entry is unbounded
    /// or unbound.
    RollbackAndDiagnosticsPreservationNotProven,
    /// Cohort-downgrade-integrity is not proven: clean downgrade entries do not cover the canonical Retest-pending /
    /// Evidence-stale / narrowed-support scopes with full resolution-form coverage while providing the
    /// complete downgrade object, no support-ahead or form-incomplete example degrades, or a clean downgrade entry
    /// is missing the complete downgrade object.
    CohortEvidenceIntegrityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SupportedLineTruthFeedAudiencePacketRegistriesViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoRegistryRows => "no_registry_rows",
            Self::RegistryRowIncomplete => "registry_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::CohortDescriptorResolutionNotProven => "truth_feed_resolution_not_proven",
            Self::RollbackAndDiagnosticsPreservationNotProven => {
                "rollback_and_diagnostics_preservation_not_proven"
            }
            Self::CohortEvidenceIntegrityNotProven => "audience_packet_integrity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_supported_line_truth_feed_and_audience_packet_registries_export() -> Result<
    M5SupportedLineTruthFeedAudiencePacketRegistriesPacket,
    M5SupportedLineTruthFeedAudiencePacketRegistriesArtifactError,
> {
    let packet: M5SupportedLineTruthFeedAudiencePacketRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-supported-line-truth-feed-and-audience-packet-registries-proof/support_export.json"
        )
    ))
    .map_err(M5SupportedLineTruthFeedAudiencePacketRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SupportedLineTruthFeedAudiencePacketRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5SupportedLineTruthFeedAudiencePacketRegistriesPacket,
    violations: &mut Vec<M5SupportedLineTruthFeedAudiencePacketRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_SCHEMA_REF,
        M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_DOC_REF,
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_REF,
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_DOC_REF,
        M5_TRUTH_FEED_DOMAIN_SCHEMA_REF,
        M5_AUDIENCE_PACKET_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(
                M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::MissingSourceContracts,
            );
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5SupportedLineTruthFeedAudiencePacketRegistriesPacket,
    violations: &mut Vec<M5SupportedLineTruthFeedAudiencePacketRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::NoRegistryRows);
        return;
    }
    for row in &packet.registry_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.widening_stages.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(
                M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::RegistryRowIncomplete,
            );
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(
                M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::MandatoryAnatomyMissing,
            );
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_TRUTH_FEED_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_AUDIENCE_PACKET_DOMAIN_SCHEMA_REF)
        {
            violations.push(
                M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::DomainSchemaRefMissing,
            );
        }
        if row.truth_feed_entries.is_empty() || row.audience_packet_entries.is_empty() {
            violations
                .push(M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations
                .push(M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(
                M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::RowInvariantViolated,
            );
        }
    }
}

fn validate_governance_review(
    packet: &M5SupportedLineTruthFeedAudiencePacketRegistriesPacket,
    violations: &mut Vec<M5SupportedLineTruthFeedAudiencePacketRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.truth_feed_registry_names_token_role_and_journey,
        review.line_resolves_to_typed_truth_feed_from_shared_registry,
        review.repo_bundle_toolchain_and_deployment_rows_published,
        review.lines_cannot_widen_without_rollback_and_diagnostics,
        review.audience_packet_keeps_proof_visible_and_binds_support_language,
        review.support_language_matched_to_line_proof_for_public_lines,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.shiproom_release_center_executive_steering_and_program_governance_read_single_source,
        review.truth_feed_or_downgrade_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SupportedLineTruthFeedAudiencePacketRegistriesPacket,
    violations: &mut Vec<M5SupportedLineTruthFeedAudiencePacketRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shiproom_and_release_center_consume_shared_registries,
        projection.executive_steering_and_program_governance_consume_shared_registries,
        projection.diagnostics_and_public_proof_consume_shared_registries,
        projection.docs_help_and_cli_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(
                M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SupportedLineTruthFeedAudiencePacketRegistriesPacket,
    violations: &mut Vec<M5SupportedLineTruthFeedAudiencePacketRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(
            M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::ProofFreshnessIncomplete,
        );
    }
}

fn validate_release_posture(
    packet: &M5SupportedLineTruthFeedAudiencePacketRegistriesPacket,
    violations: &mut Vec<M5SupportedLineTruthFeedAudiencePacketRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.line_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(
            M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::ReleasePostureIncomplete,
        );
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5SupportedLineTruthFeedAudiencePacketRegistriesPacket,
    violations: &mut Vec<M5SupportedLineTruthFeedAudiencePacketRegistriesViolation>,
) {
    let truth_feeds = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.truth_feed_entries.iter())
    };
    let downgrade = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.audience_packet_entries.iter())
    };

    // AC1: every active line can be inspected by exact rows, bundles, toolchains, and deployment profiles.
    // Clean truth_feed entries cover the canonical line journeys and the first release-center / shiproom /
    // executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and no
    // clean truth_feed entry published an incomplete object.
    let clean_journeys: BTreeSet<String> = truth_feeds()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.report_section.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = truth_feeds()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let journeys_covered = M5TruthFeedKind::CANONICAL_JOURNEYS
        .iter()
        .all(|k| clean_journeys.contains(k.as_str()));
    let first_surfaces_covered = M5SupportedLineTruthFeedSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = truth_feeds().any(|ex| {
        ex.degrade_reason == Some(M5TruthFeedEntryDegradeReason::CohortDescriptorObjectIncomplete)
    });
    let no_clean_incomplete =
        !truth_feeds().any(|ex| ex.is_clean() && !ex.truth_feed_object_complete);
    if !(journeys_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::CohortDescriptorResolutionNotProven,
        );
    }

    // AC2: line packets preserve rollback and diagnostics posture before widening. A widen-without-rollback
    // example degrades, an unbound example degrades, at least one clean bounded truth_feed entry is present, and
    // no clean truth_feed entry is unbounded or unbound.
    let widen_fold_degrades = truth_feeds().any(|ex| {
        ex.degrade_reason
            == Some(
                M5TruthFeedEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof,
            )
    });
    let unbound_degrades = truth_feeds().any(|ex| {
        ex.degrade_reason == Some(M5TruthFeedEntryDegradeReason::DescriptorNotBoundToRegistry)
    });
    let bounded_clean_truth_feed =
        truth_feeds().any(|ex| ex.is_clean() && ex.rollback_and_diagnostics_bounded);
    let no_clean_unbound = !truth_feeds().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_unbounded =
        !truth_feeds().any(|ex| ex.is_clean() && !ex.rollback_and_diagnostics_bounded);
    if !(widen_fold_degrades
        && unbound_degrades
        && bounded_clean_truth_feed
        && no_clean_unbound
        && no_clean_unbounded)
    {
        violations.push(
            M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::RollbackAndDiagnosticsPreservationNotProven,
        );
    }

    // AC3: claim publication can prove which line downgrade backs each launch-bearing lane. Clean downgrade
    // entries cover every canonical Retest-pending / Evidence-stale / narrowed-support scope with full
    // resolution-form coverage while providing the complete downgrade object, a support-ahead example degrades, a
    // form-incomplete example degrades, and no clean downgrade entry is missing the complete object.
    let clean_comparison_scopes: BTreeSet<String> = downgrade()
        .filter(|ex| {
            ex.is_clean()
                && ex.comparison_scope_is_classified
                && ex.provides_complete_audience_packet
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.comparison_scope.clone())
        .collect();
    let comparison_scopes_covered = M5AudiencePacketScope::CANONICAL_SCOPES
        .iter()
        .all(|m| clean_comparison_scopes.contains(m.as_str()));
    let support_ahead_degrades = downgrade().any(|ex| {
        ex.degrade_reason
            == Some(
                M5AudiencePacketEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence,
            )
    });
    let form_incomplete_degrades = downgrade().any(|ex| {
        ex.degrade_reason
            == Some(M5AudiencePacketEntryDegradeReason::EvidenceFormCoverageIncomplete)
    });
    let no_clean_missing_downgrade =
        !downgrade().any(|ex| ex.is_clean() && !ex.provides_complete_audience_packet);
    if !(comparison_scopes_covered
        && support_ahead_degrades
        && form_incomplete_degrades
        && no_clean_missing_downgrade)
    {
        violations.push(
            M5SupportedLineTruthFeedAudiencePacketRegistriesViolation::CohortEvidenceIntegrityNotProven,
        );
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The launch-bearing lines this lane implements, for downstream reference: the line-truth_feed registry
/// covers the core-team canary, design-partner preview, extension-author, public preview, and certified-journey
/// lines the frozen matrix froze, and the line-downgrade-packet registry binds the downgrade that backs each.
pub const IMPLEMENTED_LINES: [M5SupportedLineTransparencyObject; 5] =
    M5SupportedLineTransparencyObject::ALL;
