//! M5 search/navigation certification — evidence-freshness and fail-closed
//! claim truth on top of the landed search/docs/graph depth lanes.
//!
//! The qualification index
//! ([`M5SearchNavigationQualificationPacket`](crate::m5_search_navigation_qualification::M5SearchNavigationQualificationPacket))
//! freezes *what* every claimed M5 search surface must prove. This certification
//! packet answers the next question — *is that proof current?* — and narrows the
//! claim automatically the moment it is not.
//!
//! Certification is organized by the four search/docs/graph depth lanes
//! ([`CertificationLaneClass`]):
//!
//! - **query-session identity** — durable query-session and stable result
//!   identity across virtualization, preview, and reopen;
//! - **ranking explainability** — user-visible ranking reasons with
//!   withheld-latency, policy-hidden, and partial-index candidates kept
//!   inspectable rather than silent;
//! - **saved-query privacy** — saved queries, scope packs, history, and deep
//!   links keep raw query text local-only by default and never widen sync or
//!   export silently; and
//! - **navigation continuity** — breadcrumb, outline, bookmark, history, and
//!   peek continuity bound to canonical anchors with visible drift/missing-target
//!   states and restore/export parity.
//!
//! Each lane cites its **own** evidence packet (its packet id, boundary schema,
//! reviewer doc, review artifact, fixture corpus, and record kind) — never an
//! adjacent lane's proof — together with that evidence's freshness
//! ([`EvidenceFreshnessClass`]) and a recheck deadline. The certification state
//! ([`CertificationStateClass`]) is then computed **fail-closed**: stale or
//! schema-superseded evidence drops the lane to `retest_pending`, a degraded
//! source lane or a broken cross-surface parity drops it to `limited`, and
//! missing evidence or a missing consumer binding drops it to `unsupported`. A
//! lane stays `certified` only with fresh evidence, a non-degraded source state,
//! and every claim surface (product, CLI/headless, docs/help, support export) in
//! parity.
//!
//! Every consumer — Help/About, docs/help, support export, and the
//! claim-publication manifest — ingests this one packet by reference and narrows
//! with it, so release and public-truth surfaces stop overclaiming search,
//! docs, or graph behavior the moment evidence freshness or parity slips. The
//! packet is metadata-safe by construction: it carries refs, states, and
//! timestamps, never raw query text, source bodies, provider payloads, or
//! credentials.

use serde::{Deserialize, Serialize};

use crate::m5_search_navigation_qualification::M5_SEARCH_NAVIGATION_QUALIFICATION_SCHEMA_REF;
use crate::navigation_continuity::{
    NAVIGATION_CONTINUITY_BINDING_ARTIFACT_REF, NAVIGATION_CONTINUITY_BINDING_DOC_REF,
    NAVIGATION_CONTINUITY_BINDING_FIXTURE_DIR, NAVIGATION_CONTINUITY_BINDING_PACKET_ID,
    NAVIGATION_CONTINUITY_BINDING_PACKET_RECORD_KIND, NAVIGATION_CONTINUITY_BINDING_SCHEMA_REF,
};
use crate::query_session::SEARCH_QUERY_SESSION_SCHEMA_VERSION;
use crate::query_session_first_consumers::{
    QUERY_SESSION_FIRST_CONSUMERS_ARTIFACT_REF, QUERY_SESSION_FIRST_CONSUMERS_DOC_REF,
    QUERY_SESSION_FIRST_CONSUMERS_FIXTURE_DIR, QUERY_SESSION_FIRST_CONSUMERS_PACKET_ID,
    QUERY_SESSION_FIRST_CONSUMERS_PACKET_RECORD_KIND, QUERY_SESSION_FIRST_CONSUMERS_SCHEMA_REF,
};
use crate::ranking_explainability::{
    RANKING_EXPLAINABILITY_ARTIFACT_REF, RANKING_EXPLAINABILITY_DOC_REF,
    RANKING_EXPLAINABILITY_FIXTURE_DIR, RANKING_EXPLAINABILITY_PACKET_ID,
    RANKING_EXPLAINABILITY_PACKET_RECORD_KIND, RANKING_EXPLAINABILITY_SCHEMA_REF,
};
use crate::saved_query_governance::{
    SAVED_QUERY_GOVERNANCE_ARTIFACT_REF, SAVED_QUERY_GOVERNANCE_DOC_REF,
    SAVED_QUERY_GOVERNANCE_FIXTURE_DIR, SAVED_QUERY_GOVERNANCE_PACKET_ID,
    SAVED_QUERY_GOVERNANCE_PACKET_RECORD_KIND, SAVED_QUERY_GOVERNANCE_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5SearchNavigationCertificationPacket`].
pub const M5_SEARCH_NAVIGATION_CERTIFICATION_PACKET_RECORD_KIND: &str =
    "m5_search_navigation_certification_packet";

/// Frozen schema version for the M5 search/navigation certification packet.
pub const M5_SEARCH_NAVIGATION_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path of the boundary schema.
pub const M5_SEARCH_NAVIGATION_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/search/m5-search-navigation-certification.schema.json";

/// Repository-relative path of the reviewer-facing contract document.
pub const M5_SEARCH_NAVIGATION_CERTIFICATION_DOC_REF: &str =
    "docs/search/m5-search-navigation-certification.md";

/// Repository-relative path of the checked review artifact.
pub const M5_SEARCH_NAVIGATION_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/search/m5/m5-search-navigation-certification.md";

/// Repository-relative path of the protected fixture directory.
pub const M5_SEARCH_NAVIGATION_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/search/m5/m5-search-navigation-certification";

/// Stable packet identifier reused by every consumer binding.
pub const M5_SEARCH_NAVIGATION_CERTIFICATION_PACKET_ID: &str =
    "search.m5.search_navigation_certification.v1";

/// Stable record-kind tag carried by
/// [`M5SearchNavigationCertificationSupportExport`].
pub const M5_SEARCH_NAVIGATION_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_search_navigation_certification_support_export";

/// Default recheck window, in days, after which lane evidence is treated as
/// stale and the lane fails closed to `retest_pending`.
pub const CERTIFICATION_RECHECK_WINDOW_DAYS: u32 = 30;

const QUALIFICATION_SCHEMA_REF: &str = M5_SEARCH_NAVIGATION_QUALIFICATION_SCHEMA_REF;

// Checked consumer surfaces that must ingest the certification index verbatim.
const HELP_ABOUT_CONSUMER_REF: &str = "docs/search/result_identity_and_ranking.md";
const DOCS_HELP_CONSUMER_REF: &str = "docs/search/m5-search-navigation-qualification.md";
const SUPPORT_EXPORT_CONSUMER_REF: &str = "schemas/search/support_export_parity_truth.schema.json";
const CLAIM_PUBLICATION_CONSUMER_REF: &str =
    "artifacts/release/stable/claim-publication-manifest/manifest.json";

const REQUIRED_PROJECTION_FIELDS: &[&str] = &[
    "certification_row_id",
    "lane",
    "certification_state",
    "evidence_packet_id",
    "evidence_freshness",
    "recheck_by",
    "source_state_token",
    "stale_proof_tokens",
    "downgrade_rule_ids",
];

/// One claimed M5 search/docs/graph depth lane the certification covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationLaneClass {
    /// Durable query-session and stable result identity.
    QuerySessionIdentity,
    /// User-visible ranking explainability.
    RankingExplainability,
    /// Saved-query, scope-pack, history, and deep-link privacy posture.
    SavedQueryPrivacy,
    /// Navigation-continuity truth across breadcrumb, outline, bookmark,
    /// history, and peek anchors.
    NavigationContinuity,
}

impl CertificationLaneClass {
    /// All certified depth lanes in canonical order.
    pub const ALL: [Self; 4] = [
        Self::QuerySessionIdentity,
        Self::RankingExplainability,
        Self::SavedQueryPrivacy,
        Self::NavigationContinuity,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuerySessionIdentity => "query_session_identity",
            Self::RankingExplainability => "ranking_explainability",
            Self::SavedQueryPrivacy => "saved_query_privacy",
            Self::NavigationContinuity => "navigation_continuity",
        }
    }

    /// Returns a review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::QuerySessionIdentity => "Query-session identity",
            Self::RankingExplainability => "Ranking explainability",
            Self::SavedQueryPrivacy => "Saved-query privacy",
            Self::NavigationContinuity => "Navigation continuity",
        }
    }

    /// The claim the lane certifies, in user-facing terms.
    pub const fn certified_claim(self) -> &'static str {
        match self {
            Self::QuerySessionIdentity => "Every claimed search, docs, and graph surface mints and reuses the one shared query session and stable result identity across virtualization, preview, and reopen.",
            Self::RankingExplainability => "Every ranked result carries an inspectable ranking-reason explanation, and withheld-latency, policy-hidden, and partial-index candidates stay visible rather than silent.",
            Self::SavedQueryPrivacy => "Saved queries, scope packs, history, and signed deep links keep raw query text local-only by default and never widen sync or export silently.",
            Self::NavigationContinuity => "Breadcrumb, outline, bookmark, history, and peek continuity bind to canonical anchors with visible drift and missing-target states and restore/export parity.",
        }
    }

    /// Returns the canonical evidence refs that back the lane's own proof.
    fn evidence_refs(self) -> LaneEvidenceRefs {
        match self {
            Self::QuerySessionIdentity => LaneEvidenceRefs {
                packet_id: QUERY_SESSION_FIRST_CONSUMERS_PACKET_ID,
                schema_ref: QUERY_SESSION_FIRST_CONSUMERS_SCHEMA_REF,
                doc_ref: QUERY_SESSION_FIRST_CONSUMERS_DOC_REF,
                artifact_ref: QUERY_SESSION_FIRST_CONSUMERS_ARTIFACT_REF,
                fixture_dir: QUERY_SESSION_FIRST_CONSUMERS_FIXTURE_DIR,
                record_kind: QUERY_SESSION_FIRST_CONSUMERS_PACKET_RECORD_KIND,
            },
            Self::RankingExplainability => LaneEvidenceRefs {
                packet_id: RANKING_EXPLAINABILITY_PACKET_ID,
                schema_ref: RANKING_EXPLAINABILITY_SCHEMA_REF,
                doc_ref: RANKING_EXPLAINABILITY_DOC_REF,
                artifact_ref: RANKING_EXPLAINABILITY_ARTIFACT_REF,
                fixture_dir: RANKING_EXPLAINABILITY_FIXTURE_DIR,
                record_kind: RANKING_EXPLAINABILITY_PACKET_RECORD_KIND,
            },
            Self::SavedQueryPrivacy => LaneEvidenceRefs {
                packet_id: SAVED_QUERY_GOVERNANCE_PACKET_ID,
                schema_ref: SAVED_QUERY_GOVERNANCE_SCHEMA_REF,
                doc_ref: SAVED_QUERY_GOVERNANCE_DOC_REF,
                artifact_ref: SAVED_QUERY_GOVERNANCE_ARTIFACT_REF,
                fixture_dir: SAVED_QUERY_GOVERNANCE_FIXTURE_DIR,
                record_kind: SAVED_QUERY_GOVERNANCE_PACKET_RECORD_KIND,
            },
            Self::NavigationContinuity => LaneEvidenceRefs {
                packet_id: NAVIGATION_CONTINUITY_BINDING_PACKET_ID,
                schema_ref: NAVIGATION_CONTINUITY_BINDING_SCHEMA_REF,
                doc_ref: NAVIGATION_CONTINUITY_BINDING_DOC_REF,
                artifact_ref: NAVIGATION_CONTINUITY_BINDING_ARTIFACT_REF,
                fixture_dir: NAVIGATION_CONTINUITY_BINDING_FIXTURE_DIR,
                record_kind: NAVIGATION_CONTINUITY_BINDING_PACKET_RECORD_KIND,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct LaneEvidenceRefs {
    packet_id: &'static str,
    schema_ref: &'static str,
    doc_ref: &'static str,
    artifact_ref: &'static str,
    fixture_dir: &'static str,
    record_kind: &'static str,
}

/// Freshness of a lane's checked-in evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshnessClass {
    /// Evidence was regenerated within the recheck window and validates against
    /// the lane's current schema version.
    Fresh,
    /// Evidence is older than the recheck window and must be re-certified.
    Stale,
    /// Evidence is absent or unreadable.
    Missing,
    /// Evidence validates an older schema version than the lane now publishes.
    Superseded,
}

impl EvidenceFreshnessClass {
    /// All freshness classes in canonical order.
    pub const ALL: [Self; 4] = [Self::Fresh, Self::Stale, Self::Missing, Self::Superseded];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
            Self::Missing => "missing",
            Self::Superseded => "superseded",
        }
    }

    /// Returns a review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Fresh => "Fresh",
            Self::Stale => "Stale",
            Self::Missing => "Missing",
            Self::Superseded => "Superseded",
        }
    }

    /// True when the freshness class requires a re-test before the lane may
    /// publish a green claim.
    pub const fn requires_retest(self) -> bool {
        !matches!(self, Self::Fresh)
    }
}

/// Closed, fail-closed certification-state vocabulary published per lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationStateClass {
    /// Fresh evidence proves the lane claim on every surface in parity.
    Certified,
    /// Evidence is stale or schema-superseded; the claim is held pending a
    /// re-test.
    RetestPending,
    /// The lane proves only a narrowed scope — a degraded source lane or a
    /// broken cross-surface parity — and may not imply whole-claim certainty.
    Limited,
    /// Evidence is missing, or a consumer binding is broken; the broad claim is
    /// not currently sustainable.
    Unsupported,
}

impl CertificationStateClass {
    /// All certification states in canonical (best-to-worst) order.
    pub const ALL: [Self; 4] = [
        Self::Certified,
        Self::RetestPending,
        Self::Limited,
        Self::Unsupported,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::RetestPending => "retest_pending",
            Self::Limited => "limited",
            Self::Unsupported => "unsupported",
        }
    }

    /// Returns a review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Certified => "Certified",
            Self::RetestPending => "Retest pending",
            Self::Limited => "Limited",
            Self::Unsupported => "Unsupported",
        }
    }

    /// Fail-closed severity; a higher value is a stricter narrowing and wins
    /// when more than one downgrade condition fires on a lane.
    const fn severity(self) -> u8 {
        match self {
            Self::Certified => 0,
            Self::Limited => 1,
            Self::RetestPending => 2,
            Self::Unsupported => 3,
        }
    }

    /// True when the state is a published green claim.
    pub const fn is_certified(self) -> bool {
        matches!(self, Self::Certified)
    }
}

/// Trigger that fails a lane closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDowngradeTriggerClass {
    /// Lane evidence is older than the recheck window.
    EvidenceStale,
    /// Lane evidence validates an older schema version than the lane publishes.
    SchemaVersionDrift,
    /// The source lane packet is itself in a degraded (narrowed) state.
    DegradedSourceState,
    /// One claim surface stopped projecting the lane's certified truth.
    SurfaceParityBreak,
    /// Lane evidence is absent or unreadable.
    EvidenceMissing,
    /// A downstream consumer stopped ingesting the certification by reference.
    ConsumerBindingMissing,
}

impl CertificationDowngradeTriggerClass {
    /// All downgrade triggers in canonical order.
    pub const ALL: [Self; 6] = [
        Self::EvidenceStale,
        Self::SchemaVersionDrift,
        Self::DegradedSourceState,
        Self::SurfaceParityBreak,
        Self::EvidenceMissing,
        Self::ConsumerBindingMissing,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceStale => "evidence_stale",
            Self::SchemaVersionDrift => "schema_version_drift",
            Self::DegradedSourceState => "degraded_source_state",
            Self::SurfaceParityBreak => "surface_parity_break",
            Self::EvidenceMissing => "evidence_missing",
            Self::ConsumerBindingMissing => "consumer_binding_missing",
        }
    }

    /// Stable rule id minted for the trigger.
    fn rule_id(self) -> &'static str {
        match self {
            Self::EvidenceStale => "evidence_stale_requires_retest",
            Self::SchemaVersionDrift => "schema_version_drift_requires_retest",
            Self::DegradedSourceState => "degraded_source_state_narrows_to_limited",
            Self::SurfaceParityBreak => "surface_parity_break_narrows_to_limited",
            Self::EvidenceMissing => "evidence_missing_blocks_claim",
            Self::ConsumerBindingMissing => "consumer_binding_missing_blocks_claim",
        }
    }

    /// Stale-proof token a fired rule stamps onto the lane row.
    fn stale_proof_token(self) -> &'static str {
        match self {
            Self::EvidenceStale => "evidence_stale_past_recheck_window",
            Self::SchemaVersionDrift => "evidence_schema_superseded",
            Self::DegradedSourceState => "source_lane_degraded",
            Self::SurfaceParityBreak => "surface_parity_break",
            Self::EvidenceMissing => "evidence_missing",
            Self::ConsumerBindingMissing => "consumer_binding_missing",
        }
    }

    /// State the trigger fails the lane closed to.
    fn downgraded_state(self) -> CertificationStateClass {
        match self {
            Self::EvidenceStale | Self::SchemaVersionDrift => {
                CertificationStateClass::RetestPending
            }
            Self::DegradedSourceState | Self::SurfaceParityBreak => {
                CertificationStateClass::Limited
            }
            Self::EvidenceMissing | Self::ConsumerBindingMissing => {
                CertificationStateClass::Unsupported
            }
        }
    }
}

/// One claim surface whose certification parity is audited.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationSurfaceClass {
    /// The product (desktop) search/navigation surface.
    ProductSurface,
    /// CLI / headless search output.
    CliHeadless,
    /// Docs/help search and discoverability copy.
    DocsHelp,
    /// Support-export and handoff surfaces.
    SupportExport,
}

impl CertificationSurfaceClass {
    /// All audited claim surfaces in canonical order.
    pub const ALL: [Self; 4] = [
        Self::ProductSurface,
        Self::CliHeadless,
        Self::DocsHelp,
        Self::SupportExport,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductSurface => "product_surface",
            Self::CliHeadless => "cli_headless",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
        }
    }

    /// Returns a review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ProductSurface => "Product surface",
            Self::CliHeadless => "CLI / headless",
            Self::DocsHelp => "Docs/help",
            Self::SupportExport => "Support export",
        }
    }
}

/// Stable consumer surface that ingests the certification result verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationConsumerClass {
    /// The Help/About evidence-freshness surface.
    HelpAbout,
    /// Docs/help search and discoverability surfaces.
    DocsHelp,
    /// Support-export and handoff packets.
    SupportExport,
    /// Release claim-publication / public-truth manifest.
    ClaimPublicationManifest,
}

impl CertificationConsumerClass {
    /// All consumer surfaces in canonical order.
    pub const ALL: [Self; 4] = [
        Self::HelpAbout,
        Self::DocsHelp,
        Self::SupportExport,
        Self::ClaimPublicationManifest,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelpAbout => "help_about",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
            Self::ClaimPublicationManifest => "claim_publication_manifest",
        }
    }
}

/// One row of the closed evidence-freshness vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessStateRow {
    /// Freshness class.
    pub freshness_class: EvidenceFreshnessClass,
    /// Stable token.
    pub token: String,
    /// Human-readable label.
    pub label: String,
    /// True when the class forces a re-test before a green claim.
    pub requires_retest: bool,
    /// Review-safe summary.
    pub summary: String,
}

/// Recheck cadence and freshness vocabulary that govern the certification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessPolicy {
    /// Recheck window, in days, after which evidence is treated as stale.
    pub recheck_window_days: u32,
    /// Closed evidence-freshness vocabulary.
    pub freshness_states: Vec<FreshnessStateRow>,
    /// Review-safe summary of the cadence.
    pub summary: String,
}

/// One per-lane certification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneCertificationRow {
    /// Stable row identifier.
    pub certification_row_id: String,
    /// Depth lane certified by the row.
    pub lane: CertificationLaneClass,
    /// Human-readable lane label.
    pub lane_label: String,
    /// The claim the lane certifies.
    pub certified_claim: String,
    /// Owning lane evidence packet id.
    pub evidence_packet_id: String,
    /// Owning lane boundary schema ref.
    pub evidence_schema_ref: String,
    /// Owning lane reviewer doc ref.
    pub evidence_doc_ref: String,
    /// Owning lane review artifact ref.
    pub evidence_artifact_ref: String,
    /// Owning lane fixture corpus dir.
    pub evidence_fixture_dir: String,
    /// Owning lane record kind.
    pub evidence_record_kind: String,
    /// Freshness of the lane's evidence.
    pub evidence_freshness: EvidenceFreshnessClass,
    /// RFC 3339 UTC time the evidence was last generated.
    pub evidence_generated_at: String,
    /// RFC 3339 UTC deadline by which the evidence must be re-certified.
    pub recheck_by: String,
    /// The source lane packet's own published state token.
    pub source_state_token: String,
    /// True when the source lane is in a degraded (narrowed) state.
    pub source_state_is_degraded: bool,
    /// Published certification state for the lane.
    pub certification_state: CertificationStateClass,
    /// Active stale-proof tokens explaining a narrowed row.
    pub stale_proof_tokens: Vec<String>,
    /// Active downgrade-rule ids explaining the published state.
    pub downgrade_rule_ids: Vec<String>,
    /// Review-safe summary.
    pub summary: String,
}

/// One audited claim surface inside a [`CertificationParityAudit`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceParityRow {
    /// Claim surface.
    pub surface: CertificationSurfaceClass,
    /// Human-readable surface label.
    pub surface_label: String,
    /// True when the surface projects the lane's published certification state.
    pub in_parity: bool,
    /// The certification-state token the surface projects.
    pub projected_state_token: String,
    /// Review-safe note.
    pub note: String,
}

/// Cross-surface parity audit for one lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationParityAudit {
    /// Depth lane audited.
    pub lane: CertificationLaneClass,
    /// Per-surface parity rows.
    pub surface_parity: Vec<SurfaceParityRow>,
    /// True when every audited surface projects the lane's published state.
    pub all_in_parity: bool,
    /// Review-safe summary.
    pub summary: String,
}

/// One automatic downgrade rule published by the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationDowngradeRuleRow {
    /// Stable rule identifier.
    pub rule_id: String,
    /// Trigger that fires the rule.
    pub trigger_class: CertificationDowngradeTriggerClass,
    /// Source certification state before the downgrade.
    pub source_state: CertificationStateClass,
    /// Resulting certification state after the downgrade.
    pub downgraded_state: CertificationStateClass,
    /// User-visible effect of the downgrade.
    pub required_effect: String,
    /// Reviewable rationale for the downgrade.
    pub rationale: String,
    /// Supporting evidence or contract refs used to inspect the rule.
    pub evidence_refs: Vec<String>,
}

/// One consumer-surface binding proving the same certification is reused.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationConsumerBinding {
    /// Consumer surface that ingests the certification.
    pub consumer: CertificationConsumerClass,
    /// Checked consumer or contract ref.
    pub consumer_ref: String,
    /// Packet identifier the consumer ingests verbatim.
    pub ingested_packet_id: String,
    /// Number of lane rows the consumer exposes by reference.
    pub lane_row_count: usize,
    /// Fields the consumer must preserve verbatim from the packet.
    pub required_verbatim_fields: Vec<String>,
    /// True when the consumer narrows immediately on stale evidence or a
    /// non-certified row.
    pub narrow_on_stale_evidence: bool,
    /// True when limited / retest / unsupported states stay labeled explicitly.
    pub explicit_limited_state_labels_required: bool,
    /// Review-safe summary of the binding contract.
    pub summary: String,
}

/// One validation error returned by
/// [`M5SearchNavigationCertificationPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SearchNavigationCertificationViolation {
    /// Field or collection path that failed validation.
    pub path: String,
    /// Reviewable explanation of the failure.
    pub message: String,
}

/// Canonical M5 search/navigation certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SearchNavigationCertificationPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Frozen schema version.
    pub schema_version: u32,
    /// Stable packet identifier.
    pub packet_id: String,
    /// RFC 3339 UTC generation time.
    pub generated_at: String,
    /// Reviewer-facing contract document ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Checked review artifact ref.
    pub artifact_ref: String,
    /// Authoritative spec sections quoted by the packet.
    pub source_spec_refs: Vec<String>,
    /// Existing lane contracts this certification composes.
    pub supporting_contract_refs: Vec<String>,
    /// Depth lanes covered by the packet.
    pub certified_lanes: Vec<CertificationLaneClass>,
    /// Claim surfaces whose parity is audited.
    pub parity_surfaces: Vec<CertificationSurfaceClass>,
    /// Recheck cadence and freshness vocabulary.
    pub freshness_policy: FreshnessPolicy,
    /// Per-lane certification rows.
    pub lane_rows: Vec<LaneCertificationRow>,
    /// Per-lane cross-surface parity audits.
    pub parity_audits: Vec<CertificationParityAudit>,
    /// Automatic downgrade rules used by the packet.
    pub downgrade_rules: Vec<CertificationDowngradeRuleRow>,
    /// Consumer-surface bindings proving one certification index is reused.
    pub consumer_bindings: Vec<CertificationConsumerBinding>,
    /// Metadata-safe summary safe for support and release surfaces.
    pub export_safe_summary: String,
}

impl M5SearchNavigationCertificationPacket {
    /// Validates lane coverage, own-proof evidence binding, fail-closed state
    /// derivation, parity audits, downgrade automation, and shared-consumer
    /// bindings. An empty result means the certification is fully covered and
    /// metadata-safe.
    pub fn validate(&self) -> Vec<M5SearchNavigationCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SEARCH_NAVIGATION_CERTIFICATION_PACKET_RECORD_KIND {
            push(&mut violations, "record_kind", "unexpected record_kind");
        }
        if self.schema_version != M5_SEARCH_NAVIGATION_CERTIFICATION_SCHEMA_VERSION {
            push(
                &mut violations,
                "schema_version",
                "unexpected schema_version",
            );
        }
        if self.packet_id != M5_SEARCH_NAVIGATION_CERTIFICATION_PACKET_ID {
            push(&mut violations, "packet_id", "unexpected packet_id");
        }
        if self.generated_at.trim().is_empty() {
            push(&mut violations, "generated_at", "generated_at is required");
        }
        if self.doc_ref != M5_SEARCH_NAVIGATION_CERTIFICATION_DOC_REF {
            push(
                &mut violations,
                "doc_ref",
                "packet must quote the canonical reviewer doc",
            );
        }
        if self.schema_ref != M5_SEARCH_NAVIGATION_CERTIFICATION_SCHEMA_REF {
            push(
                &mut violations,
                "schema_ref",
                "packet must quote the canonical schema ref",
            );
        }
        if self.artifact_ref != M5_SEARCH_NAVIGATION_CERTIFICATION_ARTIFACT_REF {
            push(
                &mut violations,
                "artifact_ref",
                "packet must quote the checked review artifact ref",
            );
        }
        if self.source_spec_refs.is_empty() {
            push(
                &mut violations,
                "source_spec_refs",
                "packet must quote at least one authoritative spec ref",
            );
        }
        if self.supporting_contract_refs.is_empty() {
            push(
                &mut violations,
                "supporting_contract_refs",
                "packet must cite the composed lane contracts",
            );
        }

        for required in CertificationLaneClass::ALL {
            if !self.certified_lanes.contains(&required) {
                push(
                    &mut violations,
                    "certified_lanes",
                    &format!("missing certified lane {}", required.as_str()),
                );
            }
        }
        for required in CertificationSurfaceClass::ALL {
            if !self.parity_surfaces.contains(&required) {
                push(
                    &mut violations,
                    "parity_surfaces",
                    &format!("missing parity surface {}", required.as_str()),
                );
            }
        }

        self.validate_freshness_policy(&mut violations);

        for lane in CertificationLaneClass::ALL {
            if !self.lane_rows.iter().any(|row| row.lane == lane) {
                push(
                    &mut violations,
                    "lane_rows",
                    &format!("missing lane row for {}", lane.as_str()),
                );
            }
        }

        let rule_ids: Vec<&str> = self
            .downgrade_rules
            .iter()
            .map(|rule| rule.rule_id.as_str())
            .collect();
        for row in &self.lane_rows {
            self.validate_row(&mut violations, row, &rule_ids);
        }

        self.validate_parity_audits(&mut violations);

        for required in CertificationDowngradeTriggerClass::ALL {
            let Some(rule) = self
                .downgrade_rules
                .iter()
                .find(|rule| rule.trigger_class == required)
            else {
                push(
                    &mut violations,
                    "downgrade_rules",
                    &format!("missing downgrade trigger {}", required.as_str()),
                );
                continue;
            };
            if rule.rule_id != required.rule_id() {
                push(
                    &mut violations,
                    &format!("downgrade_rules.{}", rule.rule_id),
                    "rule id must match its trigger",
                );
            }
            if rule.downgraded_state != required.downgraded_state() {
                push(
                    &mut violations,
                    &format!("downgrade_rules.{}", rule.rule_id),
                    "downgraded_state must match the fail-closed mapping",
                );
            }
            if rule.source_state != CertificationStateClass::Certified {
                push(
                    &mut violations,
                    &format!("downgrade_rules.{}", rule.rule_id),
                    "downgrade rule must narrow from the certified state",
                );
            }
            if rule.evidence_refs.is_empty() {
                push(
                    &mut violations,
                    &format!("downgrade_rules.{}", rule.rule_id),
                    "downgrade rule must cite at least one evidence ref",
                );
            }
        }

        self.validate_consumer_bindings(&mut violations);

        violations
    }

    fn validate_freshness_policy(
        &self,
        violations: &mut Vec<M5SearchNavigationCertificationViolation>,
    ) {
        if self.freshness_policy.recheck_window_days == 0 {
            push(
                violations,
                "freshness_policy.recheck_window_days",
                "recheck window must be positive",
            );
        }
        for required in EvidenceFreshnessClass::ALL {
            let Some(row) = self
                .freshness_policy
                .freshness_states
                .iter()
                .find(|row| row.freshness_class == required)
            else {
                push(
                    violations,
                    "freshness_policy.freshness_states",
                    &format!("missing freshness state {}", required.as_str()),
                );
                continue;
            };
            let base = format!("freshness_policy.freshness_states.{}", row.token);
            if row.token != required.as_str() {
                push(violations, &base, "token must match the freshness class");
            }
            if row.label != required.label() {
                push(violations, &base, "label must match the freshness class");
            }
            if row.requires_retest != required.requires_retest() {
                push(
                    violations,
                    &base,
                    "requires_retest must match the freshness class",
                );
            }
        }
    }

    fn validate_row(
        &self,
        violations: &mut Vec<M5SearchNavigationCertificationViolation>,
        row: &LaneCertificationRow,
        rule_ids: &[&str],
    ) {
        let base = format!("lane_rows.{}", row.certification_row_id);
        if row.lane_label != row.lane.label() {
            push(
                violations,
                &format!("{base}.lane_label"),
                "lane_label must match the canonical lane label",
            );
        }
        if row.certified_claim != row.lane.certified_claim() {
            push(
                violations,
                &format!("{base}.certified_claim"),
                "certified_claim must match the canonical lane claim",
            );
        }
        if row.summary.trim().is_empty() {
            push(
                violations,
                &format!("{base}.summary"),
                "summary may not be empty",
            );
        }
        if row.recheck_by.trim().is_empty() {
            push(
                violations,
                &format!("{base}.recheck_by"),
                "recheck_by is required",
            );
        }

        // Own-proof guard: a lane may not borrow an adjacent lane's evidence.
        let refs = row.lane.evidence_refs();
        if row.evidence_packet_id != refs.packet_id {
            push(
                violations,
                &format!("{base}.evidence_packet_id"),
                "lane must cite its own evidence packet id",
            );
        }
        if row.evidence_schema_ref != refs.schema_ref {
            push(
                violations,
                &format!("{base}.evidence_schema_ref"),
                "lane must cite its own evidence schema",
            );
        }
        if row.evidence_doc_ref != refs.doc_ref {
            push(
                violations,
                &format!("{base}.evidence_doc_ref"),
                "lane must cite its own evidence doc",
            );
        }
        if row.evidence_artifact_ref != refs.artifact_ref {
            push(
                violations,
                &format!("{base}.evidence_artifact_ref"),
                "lane must cite its own evidence artifact",
            );
        }
        if row.evidence_fixture_dir != refs.fixture_dir {
            push(
                violations,
                &format!("{base}.evidence_fixture_dir"),
                "lane must cite its own evidence fixture corpus",
            );
        }
        if row.evidence_record_kind != refs.record_kind {
            push(
                violations,
                &format!("{base}.evidence_record_kind"),
                "lane must cite its own evidence record kind",
            );
        }

        // Fail-closed derivation: the published state must be exactly the
        // narrowing implied by freshness, source degradation, and parity.
        let parity_break = self
            .parity_audits
            .iter()
            .find(|audit| audit.lane == row.lane)
            .map(|audit| !audit.all_in_parity)
            .unwrap_or(false);
        let derived = derive_certification_state(LaneConditions {
            freshness: row.evidence_freshness,
            source_degraded: row.source_state_is_degraded,
            parity_break,
        });
        if row.certification_state != derived.state {
            push(
                violations,
                &format!("{base}.certification_state"),
                "certification_state must equal the fail-closed derivation",
            );
        }

        // A certified row must be fresh, undegraded, in parity, and clean.
        if row.certification_state.is_certified() {
            if row.evidence_freshness != EvidenceFreshnessClass::Fresh {
                push(
                    violations,
                    &format!("{base}.certification_state"),
                    "a certified row requires fresh evidence",
                );
            }
            if row.source_state_is_degraded {
                push(
                    violations,
                    &format!("{base}.certification_state"),
                    "a certified row may not sit on a degraded source lane",
                );
            }
            if !row.stale_proof_tokens.is_empty() {
                push(
                    violations,
                    &format!("{base}.stale_proof_tokens"),
                    "a certified row may not carry stale-proof tokens",
                );
            }
            if !row.downgrade_rule_ids.is_empty() {
                push(
                    violations,
                    &format!("{base}.downgrade_rule_ids"),
                    "a certified row may not cite downgrade rules",
                );
            }
        } else {
            if row.downgrade_rule_ids.is_empty() {
                push(
                    violations,
                    &format!("{base}.downgrade_rule_ids"),
                    "a non-certified row must cite downgrade rules",
                );
            }
            if row.stale_proof_tokens.is_empty() {
                push(
                    violations,
                    &format!("{base}.stale_proof_tokens"),
                    "a non-certified row must carry stale-proof tokens",
                );
            }
        }

        // Derived tokens and rule ids must be exactly what fired, no more.
        if row.stale_proof_tokens != derived.tokens {
            push(
                violations,
                &format!("{base}.stale_proof_tokens"),
                "stale-proof tokens must match the fired downgrade triggers",
            );
        }
        if row.downgrade_rule_ids != derived.rule_ids {
            push(
                violations,
                &format!("{base}.downgrade_rule_ids"),
                "downgrade rule ids must match the fired downgrade triggers",
            );
        }
        for rule_id in &row.downgrade_rule_ids {
            if !rule_ids.contains(&rule_id.as_str()) {
                push(
                    violations,
                    &format!("{base}.downgrade_rule_ids"),
                    &format!("row cites unknown downgrade rule {rule_id}"),
                );
            }
        }
    }

    fn validate_parity_audits(
        &self,
        violations: &mut Vec<M5SearchNavigationCertificationViolation>,
    ) {
        for lane in CertificationLaneClass::ALL {
            let Some(audit) = self.parity_audits.iter().find(|audit| audit.lane == lane) else {
                push(
                    violations,
                    "parity_audits",
                    &format!("missing parity audit for {}", lane.as_str()),
                );
                continue;
            };
            let base = format!("parity_audits.{}", lane.as_str());
            for surface in CertificationSurfaceClass::ALL {
                if !audit
                    .surface_parity
                    .iter()
                    .any(|row| row.surface == surface)
                {
                    push(
                        violations,
                        &base,
                        &format!("missing surface parity row {}", surface.as_str()),
                    );
                }
            }
            let computed_all = audit.surface_parity.iter().all(|row| row.in_parity);
            if audit.all_in_parity != computed_all {
                push(
                    violations,
                    &format!("{base}.all_in_parity"),
                    "all_in_parity must equal the conjunction of surface parity rows",
                );
            }
            // The lane row's published state and each in-parity surface's
            // projected token must agree, so a surface can never quietly
            // overclaim a greener state than the lane proves.
            if let Some(row) = self.lane_rows.iter().find(|row| row.lane == lane) {
                let published = row.certification_state.as_str();
                for parity_row in &audit.surface_parity {
                    if parity_row.surface_label != parity_row.surface.label() {
                        push(
                            violations,
                            &format!("{base}.{}", parity_row.surface.as_str()),
                            "surface_label must match the canonical surface label",
                        );
                    }
                    if parity_row.in_parity && parity_row.projected_state_token != published {
                        push(
                            violations,
                            &format!("{base}.{}", parity_row.surface.as_str()),
                            "an in-parity surface must project the lane's published state",
                        );
                    }
                    if !parity_row.in_parity && parity_row.projected_state_token == published {
                        push(
                            violations,
                            &format!("{base}.{}", parity_row.surface.as_str()),
                            "an out-of-parity surface must project a different state",
                        );
                    }
                }
            }
        }
    }

    fn validate_consumer_bindings(
        &self,
        violations: &mut Vec<M5SearchNavigationCertificationViolation>,
    ) {
        for required in CertificationConsumerClass::ALL {
            let Some(binding) = self
                .consumer_bindings
                .iter()
                .find(|binding| binding.consumer == required)
            else {
                push(
                    violations,
                    "consumer_bindings",
                    &format!("missing consumer binding {}", required.as_str()),
                );
                continue;
            };
            let base = format!("consumer_bindings.{}", binding.consumer.as_str());
            if binding.ingested_packet_id != self.packet_id {
                push(
                    violations,
                    &base,
                    "consumer binding must ingest the canonical packet id",
                );
            }
            if binding.lane_row_count != self.lane_rows.len() {
                push(
                    violations,
                    &base,
                    "consumer binding row count must match lane rows",
                );
            }
            if !binding.narrow_on_stale_evidence {
                push(
                    violations,
                    &base,
                    "consumer binding must narrow on stale evidence",
                );
            }
            for field in REQUIRED_PROJECTION_FIELDS {
                if !binding
                    .required_verbatim_fields
                    .iter()
                    .any(|item| item == field)
                {
                    push(
                        violations,
                        &base,
                        &format!("consumer binding must preserve {field}"),
                    );
                }
            }
        }
    }

    /// Returns true when the packet remains metadata-safe by construction.
    pub fn is_export_safe(&self) -> bool {
        self.export_safe_summary.contains("metadata-safe")
            && self
                .consumer_bindings
                .iter()
                .all(|binding| binding.narrow_on_stale_evidence)
    }

    /// Returns the number of lane rows in each published certification state.
    pub fn state_counts(&self) -> CertificationStateCounts {
        let mut counts = CertificationStateCounts::default();
        for row in &self.lane_rows {
            match row.certification_state {
                CertificationStateClass::Certified => counts.certified += 1,
                CertificationStateClass::RetestPending => counts.retest_pending += 1,
                CertificationStateClass::Limited => counts.limited += 1,
                CertificationStateClass::Unsupported => counts.unsupported += 1,
            }
        }
        counts
    }

    /// True when every lane row is fully certified on fresh evidence.
    pub fn is_fully_certified(&self) -> bool {
        self.lane_rows
            .iter()
            .all(|row| row.certification_state.is_certified())
    }

    /// Builds a support export that wraps the exact product packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5SearchNavigationCertificationSupportExport {
        M5SearchNavigationCertificationSupportExport {
            record_kind: M5_SEARCH_NAVIGATION_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_SEARCH_NAVIGATION_CERTIFICATION_SCHEMA_VERSION,
            export_id: export_id.into(),
            certification_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            certification_packet: self.clone(),
        }
    }
}

/// Row counts by published certification state.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CertificationStateCounts {
    /// Lanes that remain fully certified.
    pub certified: usize,
    /// Lanes held pending a re-test.
    pub retest_pending: usize,
    /// Lanes narrowed to a limited claim.
    pub limited: usize,
    /// Lanes whose broad claim is unsupported.
    pub unsupported: usize,
}

/// Metadata-safe support export wrapping the exact certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SearchNavigationCertificationSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Certification packet id preserved by the export.
    pub certification_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials / authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub certification_packet: M5SearchNavigationCertificationPacket,
}

#[derive(Debug, Clone, Copy)]
struct LaneConditions {
    freshness: EvidenceFreshnessClass,
    source_degraded: bool,
    parity_break: bool,
}

struct DerivedState {
    state: CertificationStateClass,
    tokens: Vec<String>,
    rule_ids: Vec<String>,
}

/// Computes the fail-closed certification state for one lane. Every applicable
/// trigger contributes its token and rule id, and the strictest narrowing wins.
fn derive_certification_state(conditions: LaneConditions) -> DerivedState {
    use CertificationDowngradeTriggerClass as Trigger;

    let mut fired: Vec<Trigger> = Vec::new();
    match conditions.freshness {
        EvidenceFreshnessClass::Fresh => {}
        EvidenceFreshnessClass::Stale => fired.push(Trigger::EvidenceStale),
        EvidenceFreshnessClass::Superseded => fired.push(Trigger::SchemaVersionDrift),
        EvidenceFreshnessClass::Missing => fired.push(Trigger::EvidenceMissing),
    }
    if conditions.source_degraded {
        fired.push(Trigger::DegradedSourceState);
    }
    if conditions.parity_break {
        fired.push(Trigger::SurfaceParityBreak);
    }

    let state = fired
        .iter()
        .map(|trigger| trigger.downgraded_state())
        .max_by_key(|state| state.severity())
        .unwrap_or(CertificationStateClass::Certified);

    DerivedState {
        state,
        tokens: fired
            .iter()
            .map(|trigger| trigger.stale_proof_token().to_owned())
            .collect(),
        rule_ids: fired
            .iter()
            .map(|trigger| trigger.rule_id().to_owned())
            .collect(),
    }
}

/// Returns the canonical seeded certification packet with every lane certified
/// on fresh evidence.
pub fn seeded_m5_search_navigation_certification_packet() -> M5SearchNavigationCertificationPacket {
    build_packet(CertificationVariant::Canonical)
}

/// Returns a seeded packet where the navigation-continuity lane's evidence is
/// stale, so that lane fails closed to `retest_pending` while the others stay
/// certified.
pub fn seeded_retest_pending_m5_search_navigation_certification_packet(
) -> M5SearchNavigationCertificationPacket {
    build_packet(CertificationVariant::RetestPending)
}

/// Returns a seeded packet where the ranking-explainability lane's source is
/// degraded and one surface drops parity, so that lane fails closed to
/// `limited` while the others stay certified.
pub fn seeded_limited_m5_search_navigation_certification_packet(
) -> M5SearchNavigationCertificationPacket {
    build_packet(CertificationVariant::Limited)
}

/// Returns a seeded packet where the saved-query-privacy lane's evidence is
/// missing, so that lane fails closed to `unsupported` while the others stay
/// certified.
pub fn seeded_unsupported_m5_search_navigation_certification_packet(
) -> M5SearchNavigationCertificationPacket {
    build_packet(CertificationVariant::Unsupported)
}

#[derive(Debug, Clone, Copy)]
enum CertificationVariant {
    Canonical,
    RetestPending,
    Limited,
    Unsupported,
}

impl CertificationVariant {
    /// Returns the freshness, source-degradation, and parity-break conditions
    /// for a lane under this variant.
    fn conditions(self, lane: CertificationLaneClass) -> LaneConditions {
        let fresh = LaneConditions {
            freshness: EvidenceFreshnessClass::Fresh,
            source_degraded: false,
            parity_break: false,
        };
        match (self, lane) {
            (Self::RetestPending, CertificationLaneClass::NavigationContinuity) => LaneConditions {
                freshness: EvidenceFreshnessClass::Stale,
                source_degraded: false,
                parity_break: false,
            },
            (Self::Limited, CertificationLaneClass::RankingExplainability) => LaneConditions {
                freshness: EvidenceFreshnessClass::Fresh,
                source_degraded: true,
                parity_break: true,
            },
            (Self::Unsupported, CertificationLaneClass::SavedQueryPrivacy) => LaneConditions {
                freshness: EvidenceFreshnessClass::Missing,
                source_degraded: false,
                parity_break: false,
            },
            _ => fresh,
        }
    }
}

fn build_packet(variant: CertificationVariant) -> M5SearchNavigationCertificationPacket {
    let lane_rows: Vec<LaneCertificationRow> = CertificationLaneClass::ALL
        .into_iter()
        .map(|lane| seed_row(lane, variant))
        .collect();
    let parity_audits: Vec<CertificationParityAudit> = CertificationLaneClass::ALL
        .into_iter()
        .map(|lane| seed_parity_audit(lane, variant))
        .collect();
    let row_count = lane_rows.len();

    M5SearchNavigationCertificationPacket {
        record_kind: M5_SEARCH_NAVIGATION_CERTIFICATION_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_SEARCH_NAVIGATION_CERTIFICATION_SCHEMA_VERSION,
        packet_id: M5_SEARCH_NAVIGATION_CERTIFICATION_PACKET_ID.to_owned(),
        generated_at: "2026-06-17T00:00:00Z".to_owned(),
        doc_ref: M5_SEARCH_NAVIGATION_CERTIFICATION_DOC_REF.to_owned(),
        schema_ref: M5_SEARCH_NAVIGATION_CERTIFICATION_SCHEMA_REF.to_owned(),
        artifact_ref: M5_SEARCH_NAVIGATION_CERTIFICATION_ARTIFACT_REF.to_owned(),
        source_spec_refs: vec![
            ".t2/docs/Aureline_Milestones_Document.md".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md".to_owned(),
            ".t2/docs/Aureline_UI_UX_Spec_Document.md".to_owned(),
            ".t2/docs/Aureline_UX_Design_System_Style_Guide.md".to_owned(),
            ".t2/docs/Aureline_PRD.md".to_owned(),
        ],
        supporting_contract_refs: vec![
            QUALIFICATION_SCHEMA_REF.to_owned(),
            QUERY_SESSION_FIRST_CONSUMERS_SCHEMA_REF.to_owned(),
            RANKING_EXPLAINABILITY_SCHEMA_REF.to_owned(),
            SAVED_QUERY_GOVERNANCE_SCHEMA_REF.to_owned(),
            NAVIGATION_CONTINUITY_BINDING_SCHEMA_REF.to_owned(),
        ],
        certified_lanes: CertificationLaneClass::ALL.to_vec(),
        parity_surfaces: CertificationSurfaceClass::ALL.to_vec(),
        freshness_policy: seeded_freshness_policy(),
        lane_rows,
        parity_audits,
        downgrade_rules: seeded_downgrade_rules(),
        consumer_bindings: seeded_consumer_bindings(row_count),
        export_safe_summary:
            "This metadata-safe certification index proves the four M5 search/docs/graph depth lanes — query-session identity, ranking explainability, saved-query privacy, and navigation continuity — carry current evidence: each lane cites its own evidence packet, schema, doc, artifact, fixture corpus, and record kind together with that evidence's freshness and recheck deadline, and the certification state is derived fail-closed so stale or schema-superseded evidence drops the lane to retest_pending, a degraded source lane or a broken cross-surface parity drops it to limited, and missing evidence or a missing consumer binding drops it to unsupported. Help/About, docs/help, support export, and the claim-publication manifest ingest this one index by reference and narrow with it, and no raw query text, source bodies, provider payloads, or secrets cross the boundary."
                .to_owned(),
    }
}

fn seed_row(lane: CertificationLaneClass, variant: CertificationVariant) -> LaneCertificationRow {
    let refs = lane.evidence_refs();
    let conditions = variant.conditions(lane);
    let derived = derive_certification_state(conditions);

    let (evidence_generated_at, recheck_by) = match conditions.freshness {
        // Stale evidence sits well before the recheck deadline that has passed.
        EvidenceFreshnessClass::Stale => ("2026-03-20T00:00:00Z", "2026-04-19T00:00:00Z"),
        // Missing evidence has no current generation; the deadline is in the past.
        EvidenceFreshnessClass::Missing => ("", "2026-05-17T00:00:00Z"),
        // Fresh / superseded evidence is inside the current recheck window.
        _ => ("2026-06-10T00:00:00Z", "2026-07-10T00:00:00Z"),
    };

    let source_state_token = if conditions.source_degraded {
        "scope_limited"
    } else {
        "qualified"
    };

    let summary = match derived.state {
        CertificationStateClass::Certified => format!(
            "{} is certified on fresh evidence ({}); the lane's own proof validates and every claim surface projects the same certified truth.",
            lane.label(),
            refs.packet_id
        ),
        CertificationStateClass::RetestPending => format!(
            "{} fails closed to retest pending because its evidence ({}) is past the {}-day recheck window; the lane holds its claim until the evidence is regenerated.",
            lane.label(),
            refs.packet_id,
            CERTIFICATION_RECHECK_WINDOW_DAYS
        ),
        CertificationStateClass::Limited => format!(
            "{} fails closed to limited because its source lane is degraded and a claim surface dropped parity; the lane keeps only a narrowed claim and the overclaiming surface is held back.",
            lane.label()
        ),
        CertificationStateClass::Unsupported => format!(
            "{} fails closed to unsupported because its evidence ({}) is missing; the broad claim cannot be sustained until the evidence is restored.",
            lane.label(),
            refs.packet_id
        ),
    };

    LaneCertificationRow {
        certification_row_id: format!("m5_search_navigation_certification:{}", lane.as_str()),
        lane,
        lane_label: lane.label().to_owned(),
        certified_claim: lane.certified_claim().to_owned(),
        evidence_packet_id: refs.packet_id.to_owned(),
        evidence_schema_ref: refs.schema_ref.to_owned(),
        evidence_doc_ref: refs.doc_ref.to_owned(),
        evidence_artifact_ref: refs.artifact_ref.to_owned(),
        evidence_fixture_dir: refs.fixture_dir.to_owned(),
        evidence_record_kind: refs.record_kind.to_owned(),
        evidence_freshness: conditions.freshness,
        evidence_generated_at: evidence_generated_at.to_owned(),
        recheck_by: recheck_by.to_owned(),
        source_state_token: source_state_token.to_owned(),
        source_state_is_degraded: conditions.source_degraded,
        certification_state: derived.state,
        stale_proof_tokens: derived.tokens,
        downgrade_rule_ids: derived.rule_ids,
        summary,
    }
}

fn seed_parity_audit(
    lane: CertificationLaneClass,
    variant: CertificationVariant,
) -> CertificationParityAudit {
    let conditions = variant.conditions(lane);
    let derived = derive_certification_state(conditions);
    let published = derived.state.as_str().to_owned();

    let surface_parity: Vec<SurfaceParityRow> = CertificationSurfaceClass::ALL
        .into_iter()
        .map(|surface| {
            // A parity break is modeled as the support-export surface still
            // projecting the greener certified state the lane no longer holds.
            let out_of_parity =
                conditions.parity_break && surface == CertificationSurfaceClass::SupportExport;
            if out_of_parity {
                SurfaceParityRow {
                    surface,
                    surface_label: surface.label().to_owned(),
                    in_parity: false,
                    projected_state_token: CertificationStateClass::Certified.as_str().to_owned(),
                    note: format!(
                        "{} still projects a certified claim the lane no longer proves; the certification holds the broad claim back until parity is restored.",
                        surface.label()
                    ),
                }
            } else {
                SurfaceParityRow {
                    surface,
                    surface_label: surface.label().to_owned(),
                    in_parity: true,
                    projected_state_token: published.clone(),
                    note: format!(
                        "{} projects the lane's published {} state.",
                        surface.label(),
                        published
                    ),
                }
            }
        })
        .collect();

    let all_in_parity = surface_parity.iter().all(|row| row.in_parity);

    CertificationParityAudit {
        lane,
        surface_parity,
        all_in_parity,
        summary: format!(
            "{} parity audit: {} of {} claim surfaces project the lane's {} state.",
            lane.label(),
            CertificationSurfaceClass::ALL.len() - usize::from(!all_in_parity),
            CertificationSurfaceClass::ALL.len(),
            published
        ),
    }
}

fn seeded_freshness_policy() -> FreshnessPolicy {
    FreshnessPolicy {
        recheck_window_days: CERTIFICATION_RECHECK_WINDOW_DAYS,
        freshness_states: EvidenceFreshnessClass::ALL
            .into_iter()
            .map(|class| FreshnessStateRow {
                freshness_class: class,
                token: class.as_str().to_owned(),
                label: class.label().to_owned(),
                requires_retest: class.requires_retest(),
                summary: match class {
                    EvidenceFreshnessClass::Fresh => "Evidence was regenerated within the recheck window and validates against the lane's current schema.".to_owned(),
                    EvidenceFreshnessClass::Stale => "Evidence is older than the recheck window; the lane fails closed to retest pending until it is regenerated.".to_owned(),
                    EvidenceFreshnessClass::Missing => "Evidence is absent or unreadable; the lane fails closed to unsupported.".to_owned(),
                    EvidenceFreshnessClass::Superseded => "Evidence validates an older schema version than the lane now publishes; the lane fails closed to retest pending.".to_owned(),
                },
            })
            .collect(),
        summary: format!(
            "Lane evidence must be regenerated and re-certified within {} days; once it falls outside the window the lane fails closed automatically.",
            CERTIFICATION_RECHECK_WINDOW_DAYS
        ),
    }
}

fn seeded_downgrade_rules() -> Vec<CertificationDowngradeRuleRow> {
    use CertificationDowngradeTriggerClass as Trigger;
    let rule =
        |trigger: Trigger, required_effect: &str, rationale: &str, evidence_refs: Vec<String>| {
            CertificationDowngradeRuleRow {
                rule_id: trigger.rule_id().to_owned(),
                trigger_class: trigger,
                source_state: CertificationStateClass::Certified,
                downgraded_state: trigger.downgraded_state(),
                required_effect: required_effect.to_owned(),
                rationale: rationale.to_owned(),
                evidence_refs,
            }
        };
    vec![
        rule(
            Trigger::EvidenceStale,
            "When a lane's evidence is older than the recheck window, the lane fails closed to retest pending; the claim is held until the evidence is regenerated and re-certified.",
            "Certification is only as current as its newest evidence; a stale packet cannot keep a green claim.",
            vec![
                M5_SEARCH_NAVIGATION_CERTIFICATION_DOC_REF.to_owned(),
                M5_SEARCH_NAVIGATION_CERTIFICATION_ARTIFACT_REF.to_owned(),
            ],
        ),
        rule(
            Trigger::SchemaVersionDrift,
            "When a lane's evidence validates an older schema version than the lane now publishes, the lane fails closed to retest pending until the evidence is regenerated against the current schema.",
            "A schema bump can change the meaning of the proof; evidence on the old shape is no longer authoritative.",
            vec![
                M5_SEARCH_NAVIGATION_CERTIFICATION_DOC_REF.to_owned(),
                QUALIFICATION_SCHEMA_REF.to_owned(),
            ],
        ),
        rule(
            Trigger::DegradedSourceState,
            "When the source lane packet is itself in a degraded (narrowed) state, the lane fails closed to limited and may not imply whole-claim certainty.",
            "The certification cannot be greener than the lane it certifies; a degraded source narrows the certified claim.",
            vec![
                M5_SEARCH_NAVIGATION_CERTIFICATION_DOC_REF.to_owned(),
                QUALIFICATION_SCHEMA_REF.to_owned(),
            ],
        ),
        rule(
            Trigger::SurfaceParityBreak,
            "When one claim surface (product, CLI/headless, docs/help, or support export) stops projecting the lane's certified truth, the lane fails closed to limited and the overclaiming surface is held back until parity is restored.",
            "A surface that overclaims a greener state than the lane proves breaks the one-index promise; release and public-truth surfaces must narrow with the lane.",
            vec![
                M5_SEARCH_NAVIGATION_CERTIFICATION_DOC_REF.to_owned(),
                SUPPORT_EXPORT_CONSUMER_REF.to_owned(),
            ],
        ),
        rule(
            Trigger::EvidenceMissing,
            "When a lane's evidence is absent or unreadable, the lane fails closed to unsupported; the broad claim cannot be sustained until the evidence is restored.",
            "An unverifiable claim is not a claim; missing evidence is the strictest fail-closed state.",
            vec![
                M5_SEARCH_NAVIGATION_CERTIFICATION_DOC_REF.to_owned(),
                M5_SEARCH_NAVIGATION_CERTIFICATION_ARTIFACT_REF.to_owned(),
            ],
        ),
        rule(
            Trigger::ConsumerBindingMissing,
            "If Help/About, docs/help, support export, or the claim-publication manifest stops ingesting this certification by reference, the broad claim fails closed to unsupported until parity is restored.",
            "The certification only governs claim truth while every consumer ingests it; a broken binding lets a surface drift.",
            vec![
                M5_SEARCH_NAVIGATION_CERTIFICATION_DOC_REF.to_owned(),
                CLAIM_PUBLICATION_CONSUMER_REF.to_owned(),
            ],
        ),
    ]
}

fn seeded_consumer_bindings(row_count: usize) -> Vec<CertificationConsumerBinding> {
    let verbatim_fields: Vec<String> = REQUIRED_PROJECTION_FIELDS
        .iter()
        .map(|field| (*field).to_owned())
        .collect();
    let binding = |consumer: CertificationConsumerClass, consumer_ref: &str, summary: &str| {
        CertificationConsumerBinding {
            consumer,
            consumer_ref: consumer_ref.to_owned(),
            ingested_packet_id: M5_SEARCH_NAVIGATION_CERTIFICATION_PACKET_ID.to_owned(),
            lane_row_count: row_count,
            required_verbatim_fields: verbatim_fields.clone(),
            narrow_on_stale_evidence: true,
            explicit_limited_state_labels_required: true,
            summary: summary.to_owned(),
        }
    };
    vec![
        binding(
            CertificationConsumerClass::HelpAbout,
            HELP_ABOUT_CONSUMER_REF,
            "Help/About surfaces the per-lane certification state and recheck deadline verbatim so users see when search/docs/graph evidence was last certified and when it must be re-tested.",
        ),
        binding(
            CertificationConsumerClass::DocsHelp,
            DOCS_HELP_CONSUMER_REF,
            "Docs/help describes the certified lanes and their freshness by reference, narrowing the prose automatically when a lane drops to retest pending, limited, or unsupported.",
        ),
        binding(
            CertificationConsumerClass::SupportExport,
            SUPPORT_EXPORT_CONSUMER_REF,
            "Support-export packets attach the same lane row ids, certification states, freshness, and downgrade tokens instead of minting a parallel badge, and stay metadata-only.",
        ),
        binding(
            CertificationConsumerClass::ClaimPublicationManifest,
            CLAIM_PUBLICATION_CONSUMER_REF,
            "The claim-publication manifest consumes this index so a stale, limited, or unsupported lane cannot keep a broader search/docs/graph release claim green.",
        ),
    ]
}

fn push(violations: &mut Vec<M5SearchNavigationCertificationViolation>, path: &str, message: &str) {
    violations.push(M5SearchNavigationCertificationViolation {
        path: path.to_owned(),
        message: message.to_owned(),
    });
}

// The frozen schema version must track the upstream query-session schema this
// certification composes; a bump there should be a deliberate, reviewed change
// here too.
const _: () = assert!(
    M5_SEARCH_NAVIGATION_CERTIFICATION_SCHEMA_VERSION == SEARCH_QUERY_SESSION_SCHEMA_VERSION
);

#[cfg(test)]
mod tests;
