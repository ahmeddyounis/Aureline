//! M5 certification packet for docs, browser, semantic-recall, and
//! codebase-understanding surfaces, narrowing any underqualified row.
//!
//! Where the frozen recall matrix locks four depth lanes, this module
//! *certifies* every landed docs-and-code-understanding surface against that
//! matrix and against each surface's own checked-in evidence. Each
//! [`CertifiedSurfaceRow`] binds one shipped surface to the schema and support
//! export it certifies, the qualification class it earned, a certification
//! verdict, its evidence packet refs, downgrade triggers, and an explicit
//! "not greener than the matrix" flag.
//!
//! The packet carries three machine-readable companions that release, support,
//! and diagnostics tooling ingest directly instead of cloning status text:
//!
//! - a [`CertificationCompatibilityReport`] proving every certified row stays
//!   compatible with — and no greener than — the frozen matrix,
//! - a [`CertificationDowngradeRule`] set encoding how a stale, policy-blocked,
//!   or underqualified surface is automatically narrowed before publication,
//! - a [`CertificationProofFreshness`] block driving auto-narrow on stale proof.
//!
//! The certification packet is canonical for claimed M5 support in this lane:
//! no surface may stay greener than this packet. It references upstream
//! schemas, support exports, and contracts by id rather than embedding their
//! content. Raw document bodies, raw source files, raw query text, raw provider
//! payloads, credentials, and live vendor-doc snapshots stay outside the
//! support boundary.
//!
//! The boundary schema is
//! [`schemas/docs/certify-docs-browser-semantic-recall-and-codebase-understanding-rows-and-narrow-any-underqualified-surface.schema.json`](../../../../schemas/docs/certify-docs-browser-semantic-recall-and-codebase-understanding-rows-and-narrow-any-underqualified-surface.schema.json).
//! The contract doc is
//! [`docs/docs/m5/certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface.md`](../../../../docs/docs/m5/certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface/`](../../../../fixtures/docs/m5/certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    DOCS_AUTHORING_REVIEW_ARTIFACT_REF, DOCS_AUTHORING_REVIEW_SCHEMA_REF,
    DOCS_PACK_RECALL_ARTIFACT_REF, DOCS_PACK_RECALL_SCHEMA_REF, DOCS_SEARCH_LINK_ARTIFACT_REF,
    DOCS_SEARCH_LINK_SCHEMA_REF, LIGHT_REMOTE_EDIT_ARTIFACT_REF, LIGHT_REMOTE_EDIT_SCHEMA_REF,
    M5_DOCS_RECALL_MATRIX_ARTIFACT_REF, M5_DOCS_RECALL_MATRIX_SCHEMA_REF,
    M5_DOCS_RECALL_MATRIX_SCHEMA_VERSION, RETRIEVAL_DEBUG_ARTIFACT_REF, RETRIEVAL_DEBUG_SCHEMA_REF,
    SAVED_QUERY_PRIVACY_ARTIFACT_REF, SAVED_QUERY_PRIVACY_SCHEMA_REF, SCOPED_BROWSER_ARTIFACT_REF,
    SCOPED_BROWSER_SCHEMA_REF, SEMANTIC_RECALL_LEDGER_ARTIFACT_REF,
    SEMANTIC_RECALL_LEDGER_SCHEMA_REF, UNDERSTANDING_CARDS_ARTIFACT_REF,
    UNDERSTANDING_CARDS_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`CertificationPacket`].
pub const CERTIFICATION_RECORD_KIND: &str =
    "certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface";

/// Schema version for certification records.
pub const CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const CERTIFICATION_SCHEMA_REF: &str =
    "schemas/docs/certify-docs-browser-semantic-recall-and-codebase-understanding-rows-and-narrow-any-underqualified-surface.schema.json";

/// Repo-relative path of the certification contract doc.
pub const CERTIFICATION_DOC_REF: &str =
    "docs/docs/m5/certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface.md";

/// Repo-relative path of the protected fixture directory.
pub const CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/docs/m5/certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface";

/// Repo-relative path of the checked support-export artifact.
pub const CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/docs/m5/certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/docs/m5/certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface.md";

/// One certified docs-and-code-understanding surface in the B7 batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertifiedSurfaceLane {
    /// Mirror-aware docs-pack recall with source/version/freshness chips.
    DocsPackRecall,
    /// Docs and code semantic recall with query-session ledger and provenance.
    SemanticRecall,
    /// Cited codebase-understanding cards (topology, ownership, explainers).
    CodebaseUnderstandingCards,
    /// Retrieval-debug inspector with exact/imported/heuristic labeling.
    RetrievalDebug,
    /// Scoped docs/review browser surfaces with handoff reason and return path.
    ScopedBrowserSurface,
    /// Browser-lite light remote-edit surfaces with narrow scope.
    LightRemoteEdit,
    /// Saved-query privacy controls and support-export-safe search history.
    SavedQueryPrivacy,
    /// Docs authoring suggestions and stale-link / stale-example review.
    DocsAuthoringReview,
    /// Docs search with symbol-linked reference cards and code-anchor deep links.
    DocsSearchLink,
    /// The frozen M5 docs and code-recall matrix this packet certifies against.
    RecallMatrix,
}

impl CertifiedSurfaceLane {
    /// Every certified surface, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::DocsPackRecall,
        Self::SemanticRecall,
        Self::CodebaseUnderstandingCards,
        Self::RetrievalDebug,
        Self::ScopedBrowserSurface,
        Self::LightRemoteEdit,
        Self::SavedQueryPrivacy,
        Self::DocsAuthoringReview,
        Self::DocsSearchLink,
        Self::RecallMatrix,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsPackRecall => "docs_pack_recall",
            Self::SemanticRecall => "semantic_recall",
            Self::CodebaseUnderstandingCards => "codebase_understanding_cards",
            Self::RetrievalDebug => "retrieval_debug",
            Self::ScopedBrowserSurface => "scoped_browser_surface",
            Self::LightRemoteEdit => "light_remote_edit",
            Self::SavedQueryPrivacy => "saved_query_privacy",
            Self::DocsAuthoringReview => "docs_authoring_review",
            Self::DocsSearchLink => "docs_search_link",
            Self::RecallMatrix => "recall_matrix",
        }
    }

    /// Canonical schema ref for the certified surface.
    pub const fn schema_ref(self) -> &'static str {
        match self {
            Self::DocsPackRecall => DOCS_PACK_RECALL_SCHEMA_REF,
            Self::SemanticRecall => SEMANTIC_RECALL_LEDGER_SCHEMA_REF,
            Self::CodebaseUnderstandingCards => UNDERSTANDING_CARDS_SCHEMA_REF,
            Self::RetrievalDebug => RETRIEVAL_DEBUG_SCHEMA_REF,
            Self::ScopedBrowserSurface => SCOPED_BROWSER_SCHEMA_REF,
            Self::LightRemoteEdit => LIGHT_REMOTE_EDIT_SCHEMA_REF,
            Self::SavedQueryPrivacy => SAVED_QUERY_PRIVACY_SCHEMA_REF,
            Self::DocsAuthoringReview => DOCS_AUTHORING_REVIEW_SCHEMA_REF,
            Self::DocsSearchLink => DOCS_SEARCH_LINK_SCHEMA_REF,
            Self::RecallMatrix => M5_DOCS_RECALL_MATRIX_SCHEMA_REF,
        }
    }

    /// Canonical support-export ref for the certified surface.
    pub const fn artifact_ref(self) -> &'static str {
        match self {
            Self::DocsPackRecall => DOCS_PACK_RECALL_ARTIFACT_REF,
            Self::SemanticRecall => SEMANTIC_RECALL_LEDGER_ARTIFACT_REF,
            Self::CodebaseUnderstandingCards => UNDERSTANDING_CARDS_ARTIFACT_REF,
            Self::RetrievalDebug => RETRIEVAL_DEBUG_ARTIFACT_REF,
            Self::ScopedBrowserSurface => SCOPED_BROWSER_ARTIFACT_REF,
            Self::LightRemoteEdit => LIGHT_REMOTE_EDIT_ARTIFACT_REF,
            Self::SavedQueryPrivacy => SAVED_QUERY_PRIVACY_ARTIFACT_REF,
            Self::DocsAuthoringReview => DOCS_AUTHORING_REVIEW_ARTIFACT_REF,
            Self::DocsSearchLink => DOCS_SEARCH_LINK_ARTIFACT_REF,
            Self::RecallMatrix => M5_DOCS_RECALL_MATRIX_ARTIFACT_REF,
        }
    }
}

/// Qualification class a surface is certified at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationQualificationClass {
    /// Certified for the Stable claim.
    Stable,
    /// Certified at Beta.
    Beta,
    /// Certified at Preview.
    Preview,
    /// Experimental; not claimed.
    Experimental,
    /// Unavailable on this build.
    Unavailable,
    /// Held pending upstream resolution.
    Held,
}

impl CertificationQualificationClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Greenness rank; a higher rank is a stronger public claim.
    ///
    /// Used to enforce that no certified row is greener than the frozen matrix.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Stable => 5,
            Self::Beta => 4,
            Self::Preview => 3,
            Self::Experimental => 2,
            Self::Held => 1,
            Self::Unavailable => 0,
        }
    }

    /// Whether the class carries a publicly claimable promotion (Stable or Beta).
    pub const fn is_promoted(self) -> bool {
        matches!(self, Self::Stable | Self::Beta)
    }
}

/// Certification verdict recorded for a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationVerdict {
    /// Surface is certified at its claimed qualification with current evidence.
    Certified,
    /// Surface was narrowed to a lower qualification to match its evidence.
    NarrowedToQualified,
    /// Surface is held pending evidence or upstream graduation.
    HeldPendingEvidence,
    /// Surface is blocked from promotion because it is underqualified.
    BlockedUnderqualified,
}

impl CertificationVerdict {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::NarrowedToQualified => "narrowed_to_qualified",
            Self::HeldPendingEvidence => "held_pending_evidence",
            Self::BlockedUnderqualified => "blocked_underqualified",
        }
    }

    /// Whether the verdict allows the surface to keep a promoted public claim.
    pub const fn permits_promotion(self) -> bool {
        matches!(self, Self::Certified | Self::NarrowedToQualified)
    }
}

/// Downgrade trigger that can narrow a surface below its certified claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Pinned, signed mirror is offline or unavailable.
    MirrorOffline,
    /// Source version no longer matches the indexed/pinned version.
    SourceVersionMismatch,
    /// Freshness window for the recall corpus expired.
    FreshnessExpired,
    /// Workspace trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified docs/browser boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency surface narrowed.
    UpstreamDependencyNarrowed,
    /// Certified surface drifted greener than the frozen matrix.
    GreenerThanMatrix,
}

impl CertificationDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::MirrorOffline,
        Self::SourceVersionMismatch,
        Self::FreshnessExpired,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
        Self::GreenerThanMatrix,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::MirrorOffline => "mirror_offline",
            Self::SourceVersionMismatch => "source_version_mismatch",
            Self::FreshnessExpired => "freshness_expired",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
            Self::GreenerThanMatrix => "greener_than_matrix",
        }
    }
}

/// Automatic narrowing action a downgrade rule applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDowngradeAction {
    /// Narrow the surface to Beta.
    NarrowToBeta,
    /// Narrow the surface to Preview.
    NarrowToPreview,
    /// Hold the surface pending evidence.
    Hold,
    /// Block promotion of the surface.
    BlockPromotion,
}

impl CertificationDowngradeAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NarrowToBeta => "narrow_to_beta",
            Self::NarrowToPreview => "narrow_to_preview",
            Self::Hold => "hold",
            Self::BlockPromotion => "block_promotion",
        }
    }
}

/// Consumer surface that must project this certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationConsumerSurface {
    /// Release / promotion gate tooling.
    ReleaseGate,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
    /// Docs M5 evidence index.
    EvidenceIndex,
}

impl CertificationConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReleaseGate,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
        Self::EvidenceIndex,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseGate => "release_gate",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
            Self::EvidenceIndex => "evidence_index",
        }
    }
}

/// One certified surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedSurfaceRow {
    /// Certified docs-and-code-understanding surface.
    pub lane: CertifiedSurfaceLane,
    /// Qualification class the surface is certified at.
    pub qualification: CertificationQualificationClass,
    /// Certification verdict.
    pub verdict: CertificationVerdict,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Canonical schema ref for the certified surface.
    pub schema_ref: String,
    /// Canonical support-export ref for the certified surface.
    pub artifact_ref: String,
    /// Evidence packet refs backing this certification.
    pub evidence_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this surface.
    pub downgrade_triggers: Vec<CertificationDowngradeTrigger>,
    /// True when the certified claim is not greener than the frozen matrix.
    pub not_greener_than_matrix: bool,
    /// Open-raw / open-source escape preserved on every derived result.
    pub open_raw_open_source_escape_preserved: bool,
}

impl CertifiedSurfaceRow {
    /// Whether this row carries a promoted, promotion-permitting certification.
    pub fn is_promoted_and_certified(&self) -> bool {
        self.qualification.is_promoted() && self.verdict.permits_promotion()
    }
}

/// Compatibility report binding the certification to the frozen matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationCompatibilityReport {
    /// Ref of the frozen matrix support export this packet certifies against.
    pub matrix_artifact_ref: String,
    /// Ref of the frozen matrix schema.
    pub matrix_schema_ref: String,
    /// Matrix schema version this certification is compatible with.
    pub matrix_schema_version: u32,
    /// Every certified surface is present in the packet.
    pub all_surfaces_present: bool,
    /// No certified surface is greener than the frozen matrix.
    pub no_surface_greener_than_matrix: bool,
    /// Every certified surface references a checked-in schema and support export.
    pub every_surface_has_schema_and_artifact: bool,
    /// Downgrade rules are auto-enforced in release/support tooling.
    pub downgrade_rules_auto_enforced: bool,
}

/// One machine-readable downgrade rule consumed by release/support tooling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationDowngradeRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Trigger that fires the rule.
    pub trigger: CertificationDowngradeTrigger,
    /// Narrowing action the rule applies.
    pub action: CertificationDowngradeAction,
    /// Surfaces the rule applies to.
    pub applies_to: Vec<CertifiedSurfaceLane>,
    /// True when the rule is enforced automatically rather than by hand.
    pub auto_enforced: bool,
    /// Human-readable rationale.
    pub rationale: String,
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationTrustReview {
    /// Docs recall stays mirror-aware (pinned/signed mirror outranks live docs).
    pub docs_recall_mirror_aware: bool,
    /// Codebase explainers stay cited with an explicit source class.
    pub explainers_cited_with_source_class: bool,
    /// Recall and explainer results preserve their confidence class.
    pub confidence_class_preserved: bool,
    /// Open-raw / open-source escapes are preserved on every derived result.
    pub open_raw_open_source_escape_preserved: bool,
    /// Ranking reasons are explicit on every recall result.
    pub ranking_reasons_explicit: bool,
    /// The retrieval-debug inspector stays available for every recall result.
    pub retrieval_debug_available: bool,
    /// Browser surfaces stay narrow, attributable, and return-path safe.
    pub browser_surfaces_narrow_and_return_path_safe: bool,
    /// No source, mirror, pack, heuristic, or handoff looks more authoritative than proven.
    pub no_source_looks_more_authoritative_than_proven: bool,
    /// No certified surface stays greener than this canonical packet.
    pub no_surface_greener_than_packet: bool,
    /// Downgrade narrows the claim rather than hiding the surface.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationConsumerProjection {
    /// Release gate ingests the certification packet rather than cloning text.
    pub release_gate_consumes_packet: bool,
    /// CLI / headless shows certification truth.
    pub cli_headless_shows_certification: bool,
    /// Support export shows certification truth.
    pub support_export_shows_certification: bool,
    /// Diagnostics shows certification truth.
    pub diagnostics_shows_certification: bool,
    /// Help / About shows certification truth.
    pub help_about_shows_certification: bool,
    /// The M5 evidence index references this packet.
    pub evidence_index_references_packet: bool,
    /// Narrowed, held, or blocked surfaces are visibly labeled, not hidden.
    pub narrowed_surfaces_labeled_not_hidden: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the certification.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`CertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified surface rows.
    pub surface_rows: Vec<CertifiedSurfaceRow>,
    /// Compatibility report.
    pub compatibility_report: CertificationCompatibilityReport,
    /// Downgrade rules.
    pub downgrade_rules: Vec<CertificationDowngradeRule>,
    /// Trust review block.
    pub trust_review: CertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: CertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationPacket {
    /// Record kind; must equal [`CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified surface rows.
    pub surface_rows: Vec<CertifiedSurfaceRow>,
    /// Compatibility report.
    pub compatibility_report: CertificationCompatibilityReport,
    /// Downgrade rules.
    pub downgrade_rules: Vec<CertificationDowngradeRule>,
    /// Trust review block.
    pub trust_review: CertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: CertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl CertificationPacket {
    /// Builds a certification packet from stable-lane input.
    pub fn new(input: CertificationPacketInput) -> Self {
        Self {
            record_kind: CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            certification_label: input.certification_label,
            surface_rows: input.surface_rows,
            compatibility_report: input.compatibility_report,
            downgrade_rules: input.downgrade_rules,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surfaces whose certification narrows or holds the claim.
    ///
    /// Release and support tooling use this to render the narrowed surfaces
    /// rather than hiding them.
    pub fn narrowed_surfaces(&self) -> Vec<CertifiedSurfaceLane> {
        self.surface_rows
            .iter()
            .filter(|row| {
                !matches!(row.verdict, CertificationVerdict::Certified)
                    || !row.qualification.is_promoted()
            })
            .map(|row| row.lane)
            .collect()
    }

    /// Surfaces blocked from promotion.
    ///
    /// A non-empty result means promotion must fail until the surface is
    /// re-certified or narrowed.
    pub fn promotion_blockers(&self) -> Vec<CertifiedSurfaceLane> {
        self.surface_rows
            .iter()
            .filter(|row| !row.verdict.permits_promotion())
            .map(|row| row.lane)
            .collect()
    }

    /// Validates the certification invariants.
    pub fn validate(&self) -> Vec<CertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != CERTIFICATION_RECORD_KIND {
            violations.push(CertificationViolation::WrongRecordKind);
        }
        if self.schema_version != CERTIFICATION_SCHEMA_VERSION {
            violations.push(CertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(CertificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_compatibility_report(self, &mut violations);
        validate_downgrade_rules(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(CertificationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("certification packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let certified = self
            .surface_rows
            .iter()
            .filter(|row| matches!(row.verdict, CertificationVerdict::Certified))
            .count();
        let mut out = String::new();
        out.push_str("# M5 Docs and Code-Understanding Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.certification_label));
        out.push_str(&format!(
            "- Surfaces: {} ({} certified, {} narrowed/held/blocked)\n",
            self.surface_rows.len(),
            certified,
            self.narrowed_surfaces().len()
        ));
        out.push_str(&format!(
            "- Downgrade rules: {} (auto-enforced)\n",
            self.downgrade_rules.len()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "- **{}**: `{}` / `{}`\n",
                row.lane.as_str(),
                row.qualification.as_str(),
                row.verdict.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!("  - Schema: `{}`\n", row.schema_ref));
        }
        let blockers = self.promotion_blockers();
        if !blockers.is_empty() {
            out.push_str("\n## Promotion blockers\n\n");
            for lane in blockers {
                out.push_str(&format!("- `{}`\n", lane.as_str()));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum CertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CertificationViolation>),
}

impl fmt::Display for CertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "certification export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for CertificationArtifactError {}

/// Validation failures emitted by [`CertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required surface is missing from the packet.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row references the wrong canonical schema or artifact.
    SurfaceRefMismatch,
    /// A certified-and-promoted surface is missing evidence packet refs.
    CertifiedSurfaceMissingEvidence,
    /// A surface has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface claims to be no greener than the matrix but is not flagged so.
    SurfaceGreenerThanMatrix,
    /// A promotion-permitting verdict carries a non-promoted qualification, or
    /// a blocking verdict carries a promoted qualification.
    VerdictQualificationMismatch,
    /// Compatibility report does not satisfy required invariants.
    CompatibilityReportIncomplete,
    /// Downgrade rules are missing or not auto-enforced.
    DowngradeRulesIncomplete,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl CertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::SurfaceRefMismatch => "surface_ref_mismatch",
            Self::CertifiedSurfaceMissingEvidence => "certified_surface_missing_evidence",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::SurfaceGreenerThanMatrix => "surface_greener_than_matrix",
            Self::VerdictQualificationMismatch => "verdict_qualification_mismatch",
            Self::CompatibilityReportIncomplete => "compatibility_report_incomplete",
            Self::DowngradeRulesIncomplete => "downgrade_rules_incomplete",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable certification export.
pub fn current_stable_certification_export(
) -> Result<CertificationPacket, CertificationArtifactError> {
    let packet: CertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface/support_export.json"
    )))
    .map_err(CertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CertificationArtifactError::Validation(violations))
    }
}

/// Seeded stable certification input for emitters, the artifact, and tests.
pub fn seeded_stable_certification_input() -> CertificationPacketInput {
    CertificationPacketInput {
        packet_id: "m5-docs-certification:stable:0001".to_owned(),
        certification_label: "M5 Docs and Code-Understanding Certification".to_owned(),
        surface_rows: seeded_surface_rows(),
        compatibility_report: seeded_compatibility_report(),
        downgrade_rules: seeded_downgrade_rules(),
        trust_review: seeded_trust_review(),
        consumer_projection: seeded_consumer_projection(),
        proof_freshness: CertificationProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: "2026-06-09T00:00:00Z".to_owned(),
            auto_narrow_on_stale: true,
        },
        source_contract_refs: seeded_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-09T00:00:00Z".to_owned(),
    }
}

fn seeded_source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        CERTIFICATION_SCHEMA_REF.to_owned(),
        CERTIFICATION_DOC_REF.to_owned(),
        M5_DOCS_RECALL_MATRIX_ARTIFACT_REF.to_owned(),
        M5_DOCS_RECALL_MATRIX_SCHEMA_REF.to_owned(),
    ];
    for lane in CertifiedSurfaceLane::ALL {
        refs.push(lane.schema_ref().to_owned());
    }
    refs
}

fn evidence_ref(lane: CertifiedSurfaceLane) -> String {
    format!("evidence:{}:m5", lane.as_str().replace('_', "-"))
}

fn certified_row(
    lane: CertifiedSurfaceLane,
    qualification: CertificationQualificationClass,
    scope_summary: &str,
    triggers: Vec<CertificationDowngradeTrigger>,
) -> CertifiedSurfaceRow {
    CertifiedSurfaceRow {
        lane,
        qualification,
        verdict: CertificationVerdict::Certified,
        scope_summary: scope_summary.to_owned(),
        schema_ref: lane.schema_ref().to_owned(),
        artifact_ref: lane.artifact_ref().to_owned(),
        evidence_packet_refs: vec![evidence_ref(lane)],
        downgrade_triggers: triggers,
        not_greener_than_matrix: true,
        open_raw_open_source_escape_preserved: true,
    }
}

fn seeded_surface_rows() -> Vec<CertifiedSurfaceRow> {
    use CertificationDowngradeTrigger as T;
    use CertificationQualificationClass as Q;
    use CertifiedSurfaceLane as L;
    vec![
        certified_row(
            L::DocsPackRecall,
            Q::Stable,
            "Mirror-aware docs-pack recall with source/version/freshness chips and stale-example findings; pinned signed mirrors outrank live vendor docs",
            vec![T::ProofStale, T::MirrorOffline, T::SourceVersionMismatch, T::FreshnessExpired],
        ),
        certified_row(
            L::SemanticRecall,
            Q::Stable,
            "Docs and code semantic recall with a query-session ledger, explicit ranking reasons, and a cited provenance export",
            vec![T::ProofStale, T::SourceVersionMismatch, T::FreshnessExpired, T::UpstreamDependencyNarrowed],
        ),
        certified_row(
            L::CodebaseUnderstandingCards,
            Q::Stable,
            "Cited topology, ownership, and codebase-explainer cards that preserve source class and confidence with open-raw/open-source escapes",
            vec![T::ProofStale, T::SourceVersionMismatch, T::TrustNarrowing, T::UpstreamDependencyNarrowed],
        ),
        certified_row(
            L::RetrievalDebug,
            Q::Stable,
            "Retrieval-debug inspector with exact/imported/heuristic labeling and explicit ranking reasons for every docs/code recall result",
            vec![T::ProofStale, T::PolicyBlocked, T::UpstreamDependencyNarrowed],
        ),
        certified_row(
            L::ScopedBrowserSurface,
            Q::Beta,
            "Narrow, attributable docs/review browser surfaces with explicit handoff reason, return-path safety, and trust-class disclosure",
            vec![T::ProofStale, T::PolicyBlocked, T::TrustNarrowing, T::ScopeExpansionUnqualified],
        ),
        certified_row(
            L::LightRemoteEdit,
            Q::Beta,
            "Browser-lite light remote-edit surfaces with narrow scope, stale-state honesty, and no hidden authority expansion",
            vec![T::ProofStale, T::PolicyBlocked, T::TrustNarrowing, T::ScopeExpansionUnqualified],
        ),
        certified_row(
            L::SavedQueryPrivacy,
            Q::Stable,
            "Saved-query privacy controls with local-versus-shared retention truth and support-export-safe search history",
            vec![T::ProofStale, T::PolicyBlocked, T::TrustNarrowing],
        ),
        certified_row(
            L::DocsAuthoringReview,
            Q::Beta,
            "Docs authoring suggestions with stale-link/stale-example review verdicts, apply-posture truth, and open-raw/open-source escapes",
            vec![T::ProofStale, T::SourceVersionMismatch, T::FreshnessExpired, T::TrustNarrowing],
        ),
        certified_row(
            L::DocsSearchLink,
            Q::Stable,
            "Docs search with symbol-linked reference cards and code-anchor-preserving deep links",
            vec![T::ProofStale, T::SourceVersionMismatch, T::UpstreamDependencyNarrowed],
        ),
        certified_row(
            L::RecallMatrix,
            Q::Stable,
            "The frozen M5 docs and code-recall matrix this certification binds against; no certified surface stays greener than the matrix",
            vec![T::ProofStale, T::GreenerThanMatrix, T::UpstreamDependencyNarrowed],
        ),
    ]
}

fn seeded_compatibility_report() -> CertificationCompatibilityReport {
    CertificationCompatibilityReport {
        matrix_artifact_ref: M5_DOCS_RECALL_MATRIX_ARTIFACT_REF.to_owned(),
        matrix_schema_ref: M5_DOCS_RECALL_MATRIX_SCHEMA_REF.to_owned(),
        matrix_schema_version: M5_DOCS_RECALL_MATRIX_SCHEMA_VERSION,
        all_surfaces_present: true,
        no_surface_greener_than_matrix: true,
        every_surface_has_schema_and_artifact: true,
        downgrade_rules_auto_enforced: true,
    }
}

fn seeded_downgrade_rules() -> Vec<CertificationDowngradeRule> {
    use CertificationDowngradeAction as A;
    use CertificationDowngradeTrigger as T;
    vec![
        CertificationDowngradeRule {
            rule_id: "downgrade:proof_stale:hold".to_owned(),
            trigger: T::ProofStale,
            action: A::Hold,
            applies_to: CertifiedSurfaceLane::ALL.to_vec(),
            auto_enforced: true,
            rationale: "When proof exceeds the freshness SLO, every certified surface is held until re-proven rather than shipping stale evidence.".to_owned(),
        },
        CertificationDowngradeRule {
            rule_id: "downgrade:mirror_offline:narrow_recall".to_owned(),
            trigger: T::MirrorOffline,
            action: A::NarrowToBeta,
            applies_to: vec![
                CertifiedSurfaceLane::DocsPackRecall,
                CertifiedSurfaceLane::SemanticRecall,
            ],
            auto_enforced: true,
            rationale: "A pinned, signed mirror going offline narrows recall to Beta with explicit offline/freshness labels instead of silently serving live vendor docs.".to_owned(),
        },
        CertificationDowngradeRule {
            rule_id: "downgrade:scope_expansion:block_browser".to_owned(),
            trigger: T::ScopeExpansionUnqualified,
            action: A::BlockPromotion,
            applies_to: vec![
                CertifiedSurfaceLane::ScopedBrowserSurface,
                CertifiedSurfaceLane::LightRemoteEdit,
            ],
            auto_enforced: true,
            rationale: "Any browser scope expansion beyond the qualified docs/review/light-edit boundary blocks promotion until re-qualified.".to_owned(),
        },
        CertificationDowngradeRule {
            rule_id: "downgrade:policy_blocked:hold".to_owned(),
            trigger: T::PolicyBlocked,
            action: A::Hold,
            applies_to: CertifiedSurfaceLane::ALL.to_vec(),
            auto_enforced: true,
            rationale: "A policy or legal block holds the affected surface until the block clears, with the held state shown rather than hidden.".to_owned(),
        },
        CertificationDowngradeRule {
            rule_id: "downgrade:greener_than_matrix:block".to_owned(),
            trigger: T::GreenerThanMatrix,
            action: A::BlockPromotion,
            applies_to: CertifiedSurfaceLane::ALL.to_vec(),
            auto_enforced: true,
            rationale: "A certified surface drifting greener than the frozen matrix blocks promotion; this packet is canonical and no surface may stay greener than it.".to_owned(),
        },
        CertificationDowngradeRule {
            rule_id: "downgrade:upstream_narrowed:narrow_dependents".to_owned(),
            trigger: T::UpstreamDependencyNarrowed,
            action: A::NarrowToPreview,
            applies_to: vec![
                CertifiedSurfaceLane::CodebaseUnderstandingCards,
                CertifiedSurfaceLane::RetrievalDebug,
                CertifiedSurfaceLane::DocsSearchLink,
            ],
            auto_enforced: true,
            rationale: "When an upstream recall or graph dependency narrows, dependent explainer/debug surfaces narrow to Preview rather than overstating their depth.".to_owned(),
        },
    ]
}

fn seeded_trust_review() -> CertificationTrustReview {
    CertificationTrustReview {
        docs_recall_mirror_aware: true,
        explainers_cited_with_source_class: true,
        confidence_class_preserved: true,
        open_raw_open_source_escape_preserved: true,
        ranking_reasons_explicit: true,
        retrieval_debug_available: true,
        browser_surfaces_narrow_and_return_path_safe: true,
        no_source_looks_more_authoritative_than_proven: true,
        no_surface_greener_than_packet: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn seeded_consumer_projection() -> CertificationConsumerProjection {
    CertificationConsumerProjection {
        release_gate_consumes_packet: true,
        cli_headless_shows_certification: true,
        support_export_shows_certification: true,
        diagnostics_shows_certification: true,
        help_about_shows_certification: true,
        evidence_index_references_packet: true,
        narrowed_surfaces_labeled_not_hidden: true,
    }
}

fn validate_source_contracts(
    packet: &CertificationPacket,
    violations: &mut Vec<CertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required = vec![
        CERTIFICATION_SCHEMA_REF,
        CERTIFICATION_DOC_REF,
        M5_DOCS_RECALL_MATRIX_ARTIFACT_REF,
        M5_DOCS_RECALL_MATRIX_SCHEMA_REF,
    ];
    for lane in CertifiedSurfaceLane::ALL {
        required.push(lane.schema_ref());
    }
    for needed in required {
        if !refs.contains(needed) {
            violations.push(CertificationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_surface_rows(
    packet: &CertificationPacket,
    violations: &mut Vec<CertificationViolation>,
) {
    let present: BTreeSet<CertifiedSurfaceLane> =
        packet.surface_rows.iter().map(|row| row.lane).collect();
    for required in CertifiedSurfaceLane::ALL {
        if !present.contains(&required) {
            violations.push(CertificationViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.scope_summary.trim().is_empty()
            || row.schema_ref.trim().is_empty()
            || row.artifact_ref.trim().is_empty()
            || !row.open_raw_open_source_escape_preserved
        {
            violations.push(CertificationViolation::SurfaceRowIncomplete);
        }
        if row.schema_ref != row.lane.schema_ref() || row.artifact_ref != row.lane.artifact_ref() {
            violations.push(CertificationViolation::SurfaceRefMismatch);
        }
        if row.is_promoted_and_certified() && row.evidence_packet_refs.is_empty() {
            violations.push(CertificationViolation::CertifiedSurfaceMissingEvidence);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(CertificationViolation::DowngradeTriggersMissing);
        }
        if !row.not_greener_than_matrix {
            violations.push(CertificationViolation::SurfaceGreenerThanMatrix);
        }
        if row.verdict.permits_promotion() != row.qualification.is_promoted() {
            violations.push(CertificationViolation::VerdictQualificationMismatch);
        }
    }
}

fn validate_compatibility_report(
    packet: &CertificationPacket,
    violations: &mut Vec<CertificationViolation>,
) {
    let report = &packet.compatibility_report;
    let refs_ok = report.matrix_artifact_ref == M5_DOCS_RECALL_MATRIX_ARTIFACT_REF
        && report.matrix_schema_ref == M5_DOCS_RECALL_MATRIX_SCHEMA_REF
        && report.matrix_schema_version == M5_DOCS_RECALL_MATRIX_SCHEMA_VERSION;
    let flags_ok = report.all_surfaces_present
        && report.no_surface_greener_than_matrix
        && report.every_surface_has_schema_and_artifact
        && report.downgrade_rules_auto_enforced;
    if !refs_ok || !flags_ok {
        violations.push(CertificationViolation::CompatibilityReportIncomplete);
    }
}

fn validate_downgrade_rules(
    packet: &CertificationPacket,
    violations: &mut Vec<CertificationViolation>,
) {
    if packet.downgrade_rules.is_empty() {
        violations.push(CertificationViolation::DowngradeRulesIncomplete);
        return;
    }
    for rule in &packet.downgrade_rules {
        if rule.rule_id.trim().is_empty()
            || rule.rationale.trim().is_empty()
            || rule.applies_to.is_empty()
            || !rule.auto_enforced
        {
            violations.push(CertificationViolation::DowngradeRulesIncomplete);
            return;
        }
    }
}

fn validate_trust_review(
    packet: &CertificationPacket,
    violations: &mut Vec<CertificationViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.docs_recall_mirror_aware,
        review.explainers_cited_with_source_class,
        review.confidence_class_preserved,
        review.open_raw_open_source_escape_preserved,
        review.ranking_reasons_explicit,
        review.retrieval_debug_available,
        review.browser_surfaces_narrow_and_return_path_safe,
        review.no_source_looks_more_authoritative_than_proven,
        review.no_surface_greener_than_packet,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(CertificationViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &CertificationPacket,
    violations: &mut Vec<CertificationViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.release_gate_consumes_packet,
        projection.cli_headless_shows_certification,
        projection.support_export_shows_certification,
        projection.diagnostics_shows_certification,
        projection.help_about_shows_certification,
        projection.evidence_index_references_packet,
        projection.narrowed_surfaces_labeled_not_hidden,
    ] {
        if !ok {
            violations.push(CertificationViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &CertificationPacket,
    violations: &mut Vec<CertificationViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(CertificationViolation::ProofFreshnessIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
