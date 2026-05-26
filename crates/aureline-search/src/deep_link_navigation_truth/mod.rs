//! Stable deep-link remap and navigation-continuity truth packets for moved
//! files, renamed symbols, and workspace changes.
//!
//! This module is the search-owned contract for the M4 stable lane that ships
//! a single export-safe truth packet per deep-link/bookmark/history reopen
//! event. The packet binds the deep-link remap decision and the
//! workspace-side navigation-continuity record to one shared identity space
//! so the search shell, navigation history, docs/help, AI context inspector,
//! CLI/headless inspector, support export, and the release proof index all
//! read the same packet instead of reconstructing remap truth from raw paths.
//!
//! The packet is intentionally metadata-only — it carries no raw query text,
//! raw source bodies, secrets, ambient credentials, or destination contents —
//! and preserves the closed vocabularies that v24 surfaces depend on:
//!
//! - [`DeepLinkRemapPacket`] is the search-side decision record. It pins the
//!   old/new target identity, drift state, scope/workset, confidence class,
//!   evidence families, recovery actions, and destination-visibility rows for
//!   each peek/preview/split/open/back surface.
//! - [`NavigationContinuityRecord`] is the workspace-side projection that
//!   anchors the durable bookmark or history artifact to the same remap
//!   decision and recovery vocabulary.
//! - The packet glues those records together, derives a closed promotion
//!   state from validation findings, and refuses to certify when any
//!   consumer projection drops the drift state, recovery vocabulary, or
//!   destination visibility.

use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_workspace::{
    NavigationArtifactKind, NavigationContinuityError, NavigationContinuityRecord,
    NavigationContinuityState, NavigationOriginClass, NavigationRecoveryAction,
    NavigationSurfaceClass,
};

use crate::remap::{
    DeepLinkDriftState, DeepLinkRemapOutcome, DeepLinkRemapPacket, RemapConfidenceClass,
    RemapEvidenceClass, RemapTargetKind,
};

/// Stable record-kind tag for [`DeepLinkNavigationTruthPacket`].
pub const DEEP_LINK_NAVIGATION_TRUTH_PACKET_RECORD_KIND: &str =
    "deep_link_navigation_truth_stable_packet";

/// Stable record-kind tag for [`DeepLinkNavigationTruthSupportExport`].
pub const DEEP_LINK_NAVIGATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "deep_link_navigation_truth_support_export";

/// Integer schema version for the stable deep-link/navigation-continuity packet.
pub const DEEP_LINK_NAVIGATION_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DEEP_LINK_NAVIGATION_TRUTH_SCHEMA_REF: &str =
    "schemas/search/deep_link_navigation_truth_packet.schema.json";

/// Repo-relative path of the reviewer doc.
pub const DEEP_LINK_NAVIGATION_TRUTH_DOC_REF: &str =
    "docs/search/m4/deep-link-remap-and-navigation-continuity.md";

/// Repo-relative path of the human-readable artifact narrative.
pub const DEEP_LINK_NAVIGATION_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/search/m4/deep-link-remap-and-navigation-continuity.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const DEEP_LINK_NAVIGATION_TRUTH_FIXTURE_DIR: &str =
    "fixtures/search/m4/deep_link_navigation_truth_packet";

/// Repo-relative path of the checked-in stable truth packet.
pub const DEEP_LINK_NAVIGATION_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/search/m4/deep_link_navigation_truth_packet.json";

/// Closed promotion state for [`DeepLinkNavigationTruthPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkNavigationTruthPromotionState {
    /// Packet certifies a stable claim.
    Stable,
    /// Packet must remain narrowed below stable until a recorded gap closes.
    NarrowedBelowStable,
    /// Packet has a blocker finding and cannot publish on stable surfaces.
    BlocksStable,
}

impl DeepLinkNavigationTruthPromotionState {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity for one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkNavigationTruthFindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for the packet validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkNavigationTruthFindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// A row has no embedded remap packet or it fails validation.
    InvalidRemapPacket,
    /// A row has no embedded continuity record or it fails validation.
    InvalidContinuityRecord,
    /// A row's continuity record does not cite the embedded remap packet.
    ContinuityRemapPacketMismatch,
    /// A row's continuity record recovery actions disagree with the remap packet.
    ContinuityActionDrift,
    /// A row's destination-visibility rows disagree between the remap and continuity records.
    DestinationVisibilityDrift,
    /// A row crosses a visible boundary but lacks destination identity on one or more required surfaces.
    DestinationVisibilityDropped,
    /// A row drops the drift state vocabulary that consumers depend on.
    DriftStateDropped,
    /// A row drops the recovery-action vocabulary that consumers depend on.
    RecoveryActionVocabularyDropped,
    /// A row drops the evidence vocabulary that consumers depend on.
    EvidenceClassDropped,
    /// A row admits raw query text, raw source bodies, secrets, or destination contents.
    RawBoundaryMaterialPresent,
    /// Packet `covered_outcomes` drops an outcome carried by a row.
    OutcomeCoverageDropped,
    /// Packet `covered_drift_states` drops a drift state carried by a row.
    DriftStateCoverageDropped,
    /// Packet `covered_outcomes` declares an outcome no row carries.
    OutcomeCoverageOverDeclared,
    /// Packet `covered_drift_states` declares a drift state no row carries.
    DriftStateCoverageOverDeclared,
    /// A required consumer projection is missing.
    MissingConsumerProjection,
    /// A consumer projection drops or reminted truth (packet id, remap, continuity, or destination).
    ConsumerProjectionDrift,
    /// A consumer projection drops the drift-state vocabulary.
    ProjectionDriftStateDropped,
    /// A consumer projection drops the recovery-action vocabulary.
    ProjectionRecoveryActionDropped,
    /// A consumer projection drops the confidence/evidence vocabulary.
    ProjectionConfidenceEvidenceDropped,
    /// A consumer projection drops the destination-visibility rows.
    ProjectionDestinationVisibilityDropped,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl DeepLinkNavigationTruthFindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::InvalidRemapPacket => "invalid_remap_packet",
            Self::InvalidContinuityRecord => "invalid_continuity_record",
            Self::ContinuityRemapPacketMismatch => "continuity_remap_packet_mismatch",
            Self::ContinuityActionDrift => "continuity_action_drift",
            Self::DestinationVisibilityDrift => "destination_visibility_drift",
            Self::DestinationVisibilityDropped => "destination_visibility_dropped",
            Self::DriftStateDropped => "drift_state_dropped",
            Self::RecoveryActionVocabularyDropped => "recovery_action_vocabulary_dropped",
            Self::EvidenceClassDropped => "evidence_class_dropped",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::OutcomeCoverageDropped => "outcome_coverage_dropped",
            Self::DriftStateCoverageDropped => "drift_state_coverage_dropped",
            Self::OutcomeCoverageOverDeclared => "outcome_coverage_over_declared",
            Self::DriftStateCoverageOverDeclared => "drift_state_coverage_over_declared",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::ProjectionDriftStateDropped => "projection_drift_state_dropped",
            Self::ProjectionRecoveryActionDropped => "projection_recovery_action_dropped",
            Self::ProjectionConfidenceEvidenceDropped => "projection_confidence_evidence_dropped",
            Self::ProjectionDestinationVisibilityDropped => {
                "projection_destination_visibility_dropped"
            }
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the packet's truth verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkNavigationTruthConsumerSurface {
    /// Search shell quick-open, results, bookmark, and history surfaces.
    SearchShell,
    /// Workspace navigation history / back-forward chrome.
    NavigationHistory,
    /// Docs/help surface explaining remap and continuity decisions.
    DocsHelp,
    /// AI context inspector / picker.
    AiContextInspector,
    /// CLI or headless inspection surface.
    CliHeadless,
    /// Support export bundle.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
}

impl DeepLinkNavigationTruthConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 7] = [
        Self::SearchShell,
        Self::NavigationHistory,
        Self::DocsHelp,
        Self::AiContextInspector,
        Self::CliHeadless,
        Self::SupportExport,
        Self::ReleaseProofIndex,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchShell => "search_shell",
            Self::NavigationHistory => "navigation_history",
            Self::DocsHelp => "docs_help",
            Self::AiContextInspector => "ai_context_inspector",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
        }
    }
}

/// One validation finding emitted by the packet validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeepLinkNavigationTruthValidationFinding {
    /// Closed finding kind.
    pub finding_kind: DeepLinkNavigationTruthFindingKind,
    /// Finding severity.
    pub severity: DeepLinkNavigationTruthFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl DeepLinkNavigationTruthValidationFinding {
    fn new(
        finding_kind: DeepLinkNavigationTruthFindingKind,
        severity: DeepLinkNavigationTruthFindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// One row binding a search remap decision to a workspace continuity record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeepLinkNavigationTruthRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Durable artifact whose continuity is being preserved.
    pub artifact_class: NavigationArtifactKind,
    /// Surface or workflow that produced the reopen attempt.
    pub origin_class: NavigationOriginClass,
    /// Embedded search-owned remap packet.
    pub remap_packet: DeepLinkRemapPacket,
    /// Embedded workspace-side navigation continuity record.
    pub continuity_record: NavigationContinuityRecord,
    /// Short display title safe to surface alongside the row.
    pub display_title: String,
    /// True when raw query text, raw bodies, secrets, and destination contents are excluded.
    pub raw_boundary_material_excluded: bool,
    /// Capture timestamp for this row.
    pub captured_at: String,
}

impl DeepLinkNavigationTruthRow {
    /// Closed outcome the row resolved to (from the remap packet).
    pub const fn outcome_class(&self) -> DeepLinkRemapOutcome {
        self.remap_packet.outcome_class
    }

    /// Closed drift state the row resolved to (from the remap packet).
    pub const fn drift_state(&self) -> DeepLinkDriftState {
        self.remap_packet.deep_link_drift_state
    }

    /// Closed continuity outcome (from the continuity record).
    pub const fn continuity_state(&self) -> NavigationContinuityState {
        self.continuity_record.continuity_state
    }

    /// Closed confidence class (from the remap packet).
    pub const fn confidence_class(&self) -> RemapConfidenceClass {
        self.remap_packet.confidence.confidence_class
    }

    fn recovery_actions_match(&self) -> bool {
        let packet: Vec<NavigationRecoveryAction> = self.remap_packet.recovery_actions.clone();
        let continuity: Vec<NavigationRecoveryAction> =
            self.continuity_record.recovery_actions.clone();
        packet == continuity
    }

    fn destination_visibility_matches(&self) -> bool {
        let packet = &self.remap_packet.destination_visibility;
        let continuity = &self.continuity_record.destination_visibility;
        if packet.len() != continuity.len() {
            return false;
        }
        packet.iter().all(|row| {
            continuity.iter().any(|other| {
                other.surface_class == row.surface_class
                    && other.target_root_id_ref == row.target_root_id_ref
                    && other.target_root_label == row.target_root_label
                    && other.destination_repo_visible == row.destination_repo_visible
            })
        })
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeepLinkNavigationTruthConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: DeepLinkNavigationTruthConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Truth packet id consumed by the projection.
    pub export_packet_id_ref: String,
    /// Render timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id verbatim.
    pub preserves_same_packet: bool,
    /// True when the surface preserves the remap decision (old/new target,
    /// outcome, drift state, scope identity) verbatim.
    pub preserves_remap_truth: bool,
    /// True when the surface preserves the workspace continuity record's
    /// state, artifact id, placeholder ref, and failure reason verbatim.
    pub preserves_continuity_truth: bool,
    /// True when the surface preserves the destination-visibility rows
    /// verbatim for peek, preview, split, open, and back paths.
    pub preserves_destination_visibility: bool,
    /// True when the surface preserves the drift-state vocabulary verbatim.
    pub preserves_drift_state_vocabulary: bool,
    /// True when the surface preserves the closed recovery-action vocabulary verbatim.
    pub preserves_recovery_action_vocabulary: bool,
    /// True when the surface preserves the confidence class + evidence families verbatim.
    pub preserves_confidence_and_evidence: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl DeepLinkNavigationTruthConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.export_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_remap_truth
            && self.preserves_continuity_truth
            && self.preserves_destination_visibility
            && self.preserves_drift_state_vocabulary
            && self.preserves_recovery_action_vocabulary
            && self.preserves_confidence_and_evidence
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`DeepLinkNavigationTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeepLinkNavigationTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Query session that produced the packet.
    pub query_session_id_ref: String,
    /// Capture timestamp for the packet as a whole.
    pub generated_at: String,
    /// Closed outcomes covered by this packet (must match row outcomes exactly).
    #[serde(default)]
    pub covered_outcomes: Vec<DeepLinkRemapOutcome>,
    /// Closed drift states covered by this packet (must match row drift states exactly).
    #[serde(default)]
    pub covered_drift_states: Vec<DeepLinkDriftState>,
    /// Rows joining a remap packet and continuity record.
    #[serde(default)]
    pub rows: Vec<DeepLinkNavigationTruthRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<DeepLinkNavigationTruthConsumerProjection>,
    /// Source contract refs (docs / schema / fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Search-owned packet for deep-link remap and navigation-continuity truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeepLinkNavigationTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Query session that produced the packet.
    pub query_session_id_ref: String,
    /// Packet capture timestamp.
    pub generated_at: String,
    /// Closed outcomes covered by this packet.
    #[serde(default)]
    pub covered_outcomes: Vec<DeepLinkRemapOutcome>,
    /// Closed drift states covered by this packet.
    #[serde(default)]
    pub covered_drift_states: Vec<DeepLinkDriftState>,
    /// Rows joining a remap packet and continuity record.
    #[serde(default)]
    pub rows: Vec<DeepLinkNavigationTruthRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<DeepLinkNavigationTruthConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: DeepLinkNavigationTruthPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<DeepLinkNavigationTruthValidationFinding>,
}

impl DeepLinkNavigationTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: DeepLinkNavigationTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: DEEP_LINK_NAVIGATION_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: DEEP_LINK_NAVIGATION_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            query_session_id_ref: input.query_session_id_ref,
            generated_at: input.generated_at,
            covered_outcomes: input.covered_outcomes,
            covered_drift_states: input.covered_drift_states,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: DeepLinkNavigationTruthPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against stable invariants.
    pub fn validate(&self) -> Vec<DeepLinkNavigationTruthValidationFinding> {
        self.derived_findings(true)
    }

    /// True when no blocker finding fires.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == DeepLinkNavigationTruthFindingSeverity::Blocker)
    }

    /// True when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: DeepLinkNavigationTruthConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique outcome tokens carried across rows, sorted.
    pub fn outcome_tokens(&self) -> Vec<&'static str> {
        let mut set = HashSet::new();
        for row in &self.rows {
            set.insert(row.outcome_class());
        }
        let mut tokens: Vec<&'static str> =
            set.into_iter().map(DeepLinkRemapOutcome::as_str).collect();
        tokens.sort_unstable();
        tokens
    }

    /// Returns the unique drift-state tokens carried across rows, sorted.
    pub fn drift_state_tokens(&self) -> Vec<&'static str> {
        let mut set = HashSet::new();
        for row in &self.rows {
            set.insert(row.drift_state());
        }
        let mut tokens: Vec<&'static str> =
            set.into_iter().map(DeepLinkDriftState::as_str).collect();
        tokens.sort_unstable();
        tokens
    }

    /// Returns the unique confidence-class tokens carried across rows, sorted.
    pub fn confidence_tokens(&self) -> Vec<&'static str> {
        let mut set = HashSet::new();
        for row in &self.rows {
            set.insert(row.confidence_class());
        }
        let mut tokens: Vec<&'static str> = set
            .into_iter()
            .map(RemapConfidenceClass::as_str)
            .collect();
        tokens.sort_unstable();
        tokens
    }

    /// Returns the unique evidence-class tokens carried across rows, sorted.
    pub fn evidence_tokens(&self) -> Vec<&'static str> {
        let mut set = HashSet::new();
        for row in &self.rows {
            for class in &row.remap_packet.confidence.evidence_classes {
                set.insert(*class);
            }
        }
        let mut tokens: Vec<&'static str> =
            set.into_iter().map(RemapEvidenceClass::as_str).collect();
        tokens.sort_unstable();
        tokens
    }

    /// Returns the unique recovery-action tokens carried across rows, sorted.
    pub fn recovery_action_tokens(&self) -> Vec<&'static str> {
        let mut set = HashSet::new();
        for row in &self.rows {
            for action in &row.remap_packet.recovery_actions {
                set.insert(*action);
            }
        }
        let mut tokens: Vec<&'static str> = set
            .into_iter()
            .map(NavigationRecoveryAction::as_str)
            .collect();
        tokens.sort_unstable();
        tokens
    }

    /// Returns the unique target-kind tokens carried across rows, sorted.
    pub fn target_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = HashSet::new();
        for row in &self.rows {
            set.insert(row.remap_packet.old_target.target_kind);
            if let Some(new_target) = row.remap_packet.new_target.as_ref() {
                set.insert(new_target.target_kind);
            }
        }
        let mut tokens: Vec<&'static str> =
            set.into_iter().map(RemapTargetKind::as_str).collect();
        tokens.sort_unstable();
        tokens
    }

    /// Builds a support export wrapping the exact product packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> DeepLinkNavigationTruthSupportExport {
        DeepLinkNavigationTruthSupportExport {
            record_kind: DEEP_LINK_NAVIGATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DEEP_LINK_NAVIGATION_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            export_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            export_packet: self.clone(),
        }
    }

    fn derived_findings(
        &self,
        include_record_fields: bool,
    ) -> Vec<DeepLinkNavigationTruthValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != DEEP_LINK_NAVIGATION_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(DeepLinkNavigationTruthValidationFinding::new(
                DeepLinkNavigationTruthFindingKind::WrongRecordKind,
                DeepLinkNavigationTruthFindingSeverity::Blocker,
                "deep-link/navigation truth packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != DEEP_LINK_NAVIGATION_TRUTH_SCHEMA_VERSION
        {
            findings.push(DeepLinkNavigationTruthValidationFinding::new(
                DeepLinkNavigationTruthFindingKind::WrongSchemaVersion,
                DeepLinkNavigationTruthFindingSeverity::Blocker,
                "deep-link/navigation truth packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.query_session_id_ref.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(DeepLinkNavigationTruthValidationFinding::new(
                DeepLinkNavigationTruthFindingKind::MissingIdentity,
                DeepLinkNavigationTruthFindingSeverity::Blocker,
                "packet, workflow, session, and timestamp refs are required",
            ));
        }

        if self.rows.is_empty() {
            findings.push(DeepLinkNavigationTruthValidationFinding::new(
                DeepLinkNavigationTruthFindingKind::MissingIdentity,
                DeepLinkNavigationTruthFindingSeverity::Blocker,
                "packet must include at least one row",
            ));
        }

        let mut row_outcomes: HashSet<DeepLinkRemapOutcome> = HashSet::new();
        let mut row_drift_states: HashSet<DeepLinkDriftState> = HashSet::new();

        for row in &self.rows {
            if row.row_id.trim().is_empty()
                || row.display_title.trim().is_empty()
                || row.captured_at.trim().is_empty()
            {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::MissingIdentity,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "row {} identity, display title, or capture timestamp is empty",
                        row.row_id
                    ),
                ));
            }
            if !row.raw_boundary_material_excluded {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::RawBoundaryMaterialPresent,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "row {} admits raw query text, source bodies, secrets, or destination contents",
                        row.row_id
                    ),
                ));
            }

            if let Err(err) = row.remap_packet.validate() {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::InvalidRemapPacket,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!("row {} embedded remap packet invalid: {err}", row.row_id),
                ));
            }
            if let Err(err) = row.continuity_record.validate() {
                let kind = match err {
                    NavigationContinuityError::MissingDestinationVisibility(_)
                    | NavigationContinuityError::IncompleteDestinationVisibility(_)
                    | NavigationContinuityError::DestinationRepoNotVisible(_) => {
                        DeepLinkNavigationTruthFindingKind::DestinationVisibilityDropped
                    }
                    _ => DeepLinkNavigationTruthFindingKind::InvalidContinuityRecord,
                };
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    kind,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "row {} embedded continuity record invalid: {err}",
                        row.row_id
                    ),
                ));
            }

            if row.continuity_record.remap_packet_id_ref.as_deref()
                != Some(row.remap_packet.remap_packet_id.as_str())
            {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::ContinuityRemapPacketMismatch,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "row {} continuity record does not cite its embedded remap packet",
                        row.row_id
                    ),
                ));
            }

            if !row.recovery_actions_match() {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::ContinuityActionDrift,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "row {} continuity recovery actions diverge from remap packet",
                        row.row_id
                    ),
                ));
            }

            if !row.destination_visibility_matches() {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::DestinationVisibilityDrift,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "row {} destination visibility rows differ between remap and continuity",
                        row.row_id
                    ),
                ));
            }

            if row.remap_packet.scope.crosses_visible_boundary()
                && row.remap_packet.destination_visibility.is_empty()
            {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::DestinationVisibilityDropped,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "row {} crosses a visible boundary but has no destination visibility rows",
                        row.row_id
                    ),
                ));
            }

            if row.remap_packet.recovery_actions.is_empty()
                && !matches!(row.outcome_class(), DeepLinkRemapOutcome::FailedExplicitReason)
            {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::RecoveryActionVocabularyDropped,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "row {} drops the recovery-action vocabulary for outcome {}",
                        row.row_id,
                        row.outcome_class().as_str()
                    ),
                ));
            }

            if matches!(
                row.outcome_class(),
                DeepLinkRemapOutcome::Remapped | DeepLinkRemapOutcome::ResolvedExact
            ) && row.remap_packet.confidence.evidence_classes.is_empty()
            {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::EvidenceClassDropped,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "row {} resolves to {} but carries no evidence classes",
                        row.row_id,
                        row.outcome_class().as_str()
                    ),
                ));
            }

            if matches!(
                row.drift_state(),
                DeepLinkDriftState::Unresolvable | DeepLinkDriftState::ResolvedExact
            ) && row.outcome_class() == DeepLinkRemapOutcome::ResolvedExact
                && row.drift_state() != DeepLinkDriftState::ResolvedExact
            {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::DriftStateDropped,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "row {} drops the drift state for outcome {}",
                        row.row_id,
                        row.outcome_class().as_str()
                    ),
                ));
            }

            row_outcomes.insert(row.outcome_class());
            row_drift_states.insert(row.drift_state());
        }

        let covered_outcomes: HashSet<DeepLinkRemapOutcome> =
            self.covered_outcomes.iter().copied().collect();
        for outcome in &row_outcomes {
            if !covered_outcomes.contains(outcome) {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::OutcomeCoverageDropped,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "row carries outcome {} but packet covered_outcomes drops it",
                        outcome.as_str()
                    ),
                ));
            }
        }
        for outcome in &covered_outcomes {
            if !row_outcomes.contains(outcome) {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::OutcomeCoverageOverDeclared,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "packet declares outcome {} in coverage but no row carries it",
                        outcome.as_str()
                    ),
                ));
            }
        }

        let covered_drift_states: HashSet<DeepLinkDriftState> =
            self.covered_drift_states.iter().copied().collect();
        for drift in &row_drift_states {
            if !covered_drift_states.contains(drift) {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::DriftStateCoverageDropped,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "row carries drift state {} but packet covered_drift_states drops it",
                        drift.as_str()
                    ),
                ));
            }
        }
        for drift in &covered_drift_states {
            if !row_drift_states.contains(drift) {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::DriftStateCoverageOverDeclared,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "packet declares drift state {} in coverage but no row carries it",
                        drift.as_str()
                    ),
                ));
            }
        }

        for required_surface in DeepLinkNavigationTruthConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::MissingConsumerProjection,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "packet {} is missing a preserved {} projection",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
        for projection in &self.consumer_projections {
            if !projection.preserves_truth_for(&self.packet_id) {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::ConsumerProjectionDrift,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve deep-link/navigation truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_drift_state_vocabulary {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::ProjectionDriftStateDropped,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the drift-state vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_recovery_action_vocabulary {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::ProjectionRecoveryActionDropped,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the recovery-action vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_confidence_and_evidence {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::ProjectionConfidenceEvidenceDropped,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the confidence/evidence vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_destination_visibility {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::ProjectionDestinationVisibilityDropped,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    format!(
                        "projection {} drops destination visibility",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion.retain(|finding| {
                finding.finding_kind != DeepLinkNavigationTruthFindingKind::PromotionStateMismatch
            });
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(DeepLinkNavigationTruthValidationFinding::new(
                    DeepLinkNavigationTruthFindingKind::PromotionStateMismatch,
                    DeepLinkNavigationTruthFindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn promotion_state_for_findings(
    findings: &[DeepLinkNavigationTruthValidationFinding],
) -> DeepLinkNavigationTruthPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == DeepLinkNavigationTruthFindingSeverity::Blocker)
    {
        DeepLinkNavigationTruthPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == DeepLinkNavigationTruthFindingSeverity::Warning)
    {
        DeepLinkNavigationTruthPromotionState::NarrowedBelowStable
    } else {
        DeepLinkNavigationTruthPromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeepLinkNavigationTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub export_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub export_packet: DeepLinkNavigationTruthPacket,
}

impl DeepLinkNavigationTruthSupportExport {
    /// True when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == DEEP_LINK_NAVIGATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == DEEP_LINK_NAVIGATION_TRUTH_SCHEMA_VERSION
            && self.export_packet_id_ref == self.export_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.export_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable packet.
#[derive(Debug)]
pub enum DeepLinkNavigationTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<DeepLinkNavigationTruthValidationFinding>),
}

impl fmt::Display for DeepLinkNavigationTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "deep-link/navigation truth packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "deep-link/navigation truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for DeepLinkNavigationTruthArtifactError {}

/// Returns the checked-in stable truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_deep_link_navigation_truth_packet(
) -> Result<DeepLinkNavigationTruthPacket, DeepLinkNavigationTruthArtifactError> {
    let packet: DeepLinkNavigationTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/search/m4/deep_link_navigation_truth_packet.json"
    )))
    .map_err(DeepLinkNavigationTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(DeepLinkNavigationTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use aureline_workspace::{
        NavigationContinuityRecordKind, NavigationDestinationVisibility, NavigationScopeIdentity,
    };
    use aureline_workspace::{ScopeClass as WorkspaceScopeClass, ScopeMode as WorkspaceScopeMode};

    use crate::remap::{
        DeepLinkRemapRecordKind, RemapConfidence, RemapScopePacket, RemapTarget,
        DEEP_LINK_REMAP_PACKET_SCHEMA_VERSION,
    };

    fn sample_remap_packet() -> DeepLinkRemapPacket {
        DeepLinkRemapPacket {
            record_kind: DeepLinkRemapRecordKind::DeepLinkRemapPacketRecord,
            deep_link_remap_packet_schema_version: DEEP_LINK_REMAP_PACKET_SCHEMA_VERSION,
            remap_packet_id: "search:remap_packet:sample".to_owned(),
            deep_link_id_ref: "search:deep_link:sample".to_owned(),
            deep_link_drift_state: DeepLinkDriftState::ResolvedRemapped,
            outcome_class: DeepLinkRemapOutcome::Remapped,
            old_target: RemapTarget {
                target_kind: RemapTargetKind::WorkspaceFile,
                target_ref: "workspace:file:sample:old".to_owned(),
                workspace_id_ref: "workspace:app".to_owned(),
                root_id_ref: Some("root:app".to_owned()),
                root_label: Some("app".to_owned()),
                stable_result_key: None,
                path_identity_ref: None,
                symbol_anchor_ref: None,
                graph_node_ref: None,
                revision_ref: None,
            },
            new_target: Some(RemapTarget {
                target_kind: RemapTargetKind::WorkspaceFile,
                target_ref: "workspace:file:sample:new".to_owned(),
                workspace_id_ref: "workspace:app".to_owned(),
                root_id_ref: Some("root:app".to_owned()),
                root_label: Some("app".to_owned()),
                stable_result_key: None,
                path_identity_ref: None,
                symbol_anchor_ref: None,
                graph_node_ref: None,
                revision_ref: None,
            }),
            scope: RemapScopePacket {
                originating_workspace_id_ref: "workspace:app".to_owned(),
                active_workspace_id_ref: "workspace:app".to_owned(),
                stable_scope_id_ref: "scope:app:current_repo".to_owned(),
                scope_class: WorkspaceScopeClass::CurrentRepo,
                scope_mode: WorkspaceScopeMode::Full,
                workset_id_ref: None,
                workset_name: None,
                active_root_refs: vec!["root:app".to_owned()],
                destination_workspace_id_ref: Some("workspace:app".to_owned()),
                destination_root_id_ref: Some("root:app".to_owned()),
                destination_root_label: Some("app".to_owned()),
                destination_in_active_scope: true,
            },
            confidence: RemapConfidence {
                confidence_class: RemapConfidenceClass::ExactIdentity,
                confidence_score: Some(99),
                evidence_classes: vec![RemapEvidenceClass::FilesystemIdentity],
            },
            remap_chain_refs: vec!["search:deep_link:old".to_owned()],
            destination_visibility: Vec::new(),
            recovery_actions: vec![NavigationRecoveryAction::OpenRemappedTarget],
            failure_reason: None,
            emitted_at: "2026-05-13T06:40:00Z".to_owned(),
        }
    }

    fn sample_continuity_record() -> NavigationContinuityRecord {
        NavigationContinuityRecord {
            record_kind: NavigationContinuityRecordKind::NavigationContinuityRecord,
            navigation_continuity_schema_version: 1,
            continuity_id: "navigation:continuity:sample".to_owned(),
            artifact_kind: NavigationArtifactKind::Bookmark,
            artifact_id_ref: "bookmark:sample".to_owned(),
            deep_link_id_ref: Some("search:deep_link:sample".to_owned()),
            remap_packet_id_ref: Some("search:remap_packet:sample".to_owned()),
            continuity_state: NavigationContinuityState::Remapped,
            origin_class: NavigationOriginClass::BookmarkRestore,
            scope_identity: NavigationScopeIdentity {
                workspace_id_ref: "workspace:app".to_owned(),
                stable_scope_id_ref: "scope:app:current_repo".to_owned(),
                scope_class: WorkspaceScopeClass::CurrentRepo,
                scope_mode: WorkspaceScopeMode::Full,
                workset_id_ref: None,
                workset_name: None,
                active_root_refs: vec!["root:app".to_owned()],
                target_workspace_id_ref: Some("workspace:app".to_owned()),
                target_root_id_ref: Some("root:app".to_owned()),
                target_root_label: Some("app".to_owned()),
            },
            destination_visibility: Vec::new(),
            recovery_actions: vec![NavigationRecoveryAction::OpenRemappedTarget],
            failure_reason: None,
            placeholder_ref: None,
            emitted_at: "2026-05-13T06:40:00Z".to_owned(),
        }
    }

    fn sample_row() -> DeepLinkNavigationTruthRow {
        DeepLinkNavigationTruthRow {
            row_id: "row:sample:remapped".to_owned(),
            artifact_class: NavigationArtifactKind::Bookmark,
            origin_class: NavigationOriginClass::BookmarkRestore,
            remap_packet: sample_remap_packet(),
            continuity_record: sample_continuity_record(),
            display_title: "Sample bookmark".to_owned(),
            raw_boundary_material_excluded: true,
            captured_at: "2026-05-13T06:40:00Z".to_owned(),
        }
    }

    fn sample_projection(
        surface: DeepLinkNavigationTruthConsumerSurface,
        packet_id: &str,
    ) -> DeepLinkNavigationTruthConsumerProjection {
        DeepLinkNavigationTruthConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            export_packet_id_ref: packet_id.to_owned(),
            rendered_at: "2026-05-13T06:40:05Z".to_owned(),
            preserves_same_packet: true,
            preserves_remap_truth: true,
            preserves_continuity_truth: true,
            preserves_destination_visibility: true,
            preserves_drift_state_vocabulary: true,
            preserves_recovery_action_vocabulary: true,
            preserves_confidence_and_evidence: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn baseline_input(packet_id: &str) -> DeepLinkNavigationTruthPacketInput {
        DeepLinkNavigationTruthPacketInput {
            packet_id: packet_id.to_owned(),
            workflow_or_surface_id: "workflow.search.deep_link_navigation.baseline".to_owned(),
            query_session_id_ref: "search:session:m4:deep_link_nav".to_owned(),
            generated_at: "2026-05-13T06:40:00Z".to_owned(),
            covered_outcomes: vec![DeepLinkRemapOutcome::Remapped],
            covered_drift_states: vec![DeepLinkDriftState::ResolvedRemapped],
            rows: vec![sample_row()],
            consumer_projections: DeepLinkNavigationTruthConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(|surface| sample_projection(surface, packet_id))
                .collect(),
            source_contract_refs: vec![DEEP_LINK_NAVIGATION_TRUTH_DOC_REF.to_owned()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(
            DeepLinkNavigationTruthConsumerSurface::SearchShell.as_str(),
            "search_shell"
        );
        assert_eq!(
            DeepLinkNavigationTruthConsumerSurface::NavigationHistory.as_str(),
            "navigation_history"
        );
        assert_eq!(
            DeepLinkNavigationTruthFindingKind::ContinuityRemapPacketMismatch.as_str(),
            "continuity_remap_packet_mismatch"
        );
        assert_eq!(
            DeepLinkNavigationTruthFindingKind::DestinationVisibilityDropped.as_str(),
            "destination_visibility_dropped"
        );
        assert_eq!(
            DeepLinkNavigationTruthPromotionState::Stable.as_str(),
            "stable"
        );
    }

    #[test]
    fn baseline_packet_certifies_stable() {
        let packet = DeepLinkNavigationTruthPacket::materialize(baseline_input(
            "packet:m4:deep_link_navigation:baseline",
        ));
        assert_eq!(
            packet.promotion_state,
            DeepLinkNavigationTruthPromotionState::Stable
        );
        assert!(packet.validation_findings.is_empty());
        assert_eq!(packet.outcome_tokens(), vec!["remapped"]);
        assert_eq!(packet.drift_state_tokens(), vec!["resolved_remapped"]);
    }

    #[test]
    fn continuity_dropping_remap_packet_id_blocks_stable() {
        let mut input = baseline_input("packet:m4:deep_link_navigation:continuity_mismatch");
        if let Some(row) = input.rows.first_mut() {
            row.continuity_record.remap_packet_id_ref = Some("search:remap_packet:other".to_owned());
        }
        let packet = DeepLinkNavigationTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            DeepLinkNavigationTruthPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| finding
            .finding_kind
            == DeepLinkNavigationTruthFindingKind::ContinuityRemapPacketMismatch));
    }

    #[test]
    fn missing_consumer_projection_blocks_stable() {
        let mut input = baseline_input("packet:m4:deep_link_navigation:missing_projection");
        input.consumer_projections.pop();
        let packet = DeepLinkNavigationTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            DeepLinkNavigationTruthPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| finding
            .finding_kind
            == DeepLinkNavigationTruthFindingKind::MissingConsumerProjection));
    }

    #[test]
    fn projection_dropping_drift_state_blocks_stable() {
        let packet_id = "packet:m4:deep_link_navigation:drift_dropped";
        let mut input = baseline_input(packet_id);
        if let Some(projection) = input.consumer_projections.iter_mut().find(|projection| {
            projection.consumer_surface == DeepLinkNavigationTruthConsumerSurface::SupportExport
        }) {
            projection.preserves_drift_state_vocabulary = false;
        }
        let packet = DeepLinkNavigationTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            DeepLinkNavigationTruthPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| finding
            .finding_kind
            == DeepLinkNavigationTruthFindingKind::ProjectionDriftStateDropped));
    }

    #[test]
    fn over_declaring_outcome_coverage_blocks_stable() {
        let mut input = baseline_input("packet:m4:deep_link_navigation:outcome_over");
        input
            .covered_outcomes
            .push(DeepLinkRemapOutcome::FailedExplicitReason);
        let packet = DeepLinkNavigationTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            DeepLinkNavigationTruthPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| finding
            .finding_kind
            == DeepLinkNavigationTruthFindingKind::OutcomeCoverageOverDeclared));
    }

    #[test]
    fn destination_drift_blocks_stable() {
        let mut input = baseline_input("packet:m4:deep_link_navigation:dest_drift");
        if let Some(row) = input.rows.first_mut() {
            row.continuity_record.destination_visibility.push(
                NavigationDestinationVisibility {
                    surface_class: NavigationSurfaceClass::Peek,
                    target_root_id_ref: "root:other".to_owned(),
                    target_root_label: "other".to_owned(),
                    destination_repo_visible: true,
                },
            );
        }
        let packet = DeepLinkNavigationTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            DeepLinkNavigationTruthPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| finding
            .finding_kind
            == DeepLinkNavigationTruthFindingKind::DestinationVisibilityDrift));
    }
}
