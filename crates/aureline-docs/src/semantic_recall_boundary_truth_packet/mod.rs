//! Stable docs/code semantic-recall boundary truth packet shared by the
//! M4 stable lane and the v1.x preview lane.
//!
//! This module is the docs/search-owned contract that certifies the M4
//! stable boundary for docs and code semantic recall against the v1.x
//! preview surfaces. Every row pins a closed `recall_lane_class`,
//! `surface_track`, `locality_class`, `retrieval_epoch_state`,
//! `pack_signature_state`, `downgrade_state`, and `confidence_class`
//! plus an `embedder_identity`, `lane_participation`,
//! `chunk_or_anchor_provenance_ref`, `ranking_reason_ref`, and
//! `disclosure_ref` so the search shell, docs/help, AI context, review
//! workspace, CLI/headless inspector, retrieval inspector, support
//! export, and release proof index all read the same semantic-recall
//! boundary truth instead of reinventing it locally.
//!
//! The packet is intentionally metadata-only — it never admits raw query
//! text, raw source bodies, raw chunk text, raw vectors, provider
//! payloads, secrets, or ambient credentials. Rows whose locality is
//! `mirrored_pack` or `managed` MUST carry a verified pack signature.
//! Rows whose lane is not `lexical_only_fallback` MUST carry an
//! `embedder_identity` (`embedder_model_id`, `embedder_model_version`,
//! `tokenizer_id`, `chunking_strategy_id`, `retention_policy_id`,
//! `retrieval_epoch_label`) so managed and vector matches cannot
//! masquerade as anonymous current truth. Rows whose epoch is
//! `epoch_mismatch_invalidated` or `mixed_generation_blocked` MUST
//! also carry a non-`none` downgrade state. v1.x preview rows MUST
//! carry a recorded downgrade state and MUST NOT claim full stable
//! certainty.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`SemanticRecallBoundaryTruthPacket`].
pub const SEMANTIC_RECALL_BOUNDARY_TRUTH_PACKET_RECORD_KIND: &str =
    "semantic_recall_boundary_truth_stable_packet";

/// Stable record-kind tag for [`SemanticRecallBoundaryTruthSupportExport`].
pub const SEMANTIC_RECALL_BOUNDARY_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "semantic_recall_boundary_truth_support_export";

/// Integer schema version for the stable semantic-recall boundary truth packet.
pub const SEMANTIC_RECALL_BOUNDARY_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SEMANTIC_RECALL_BOUNDARY_TRUTH_SCHEMA_REF: &str =
    "schemas/docs/semantic_recall_boundary_truth.schema.json";

/// Repo-relative path of the reviewer doc.
pub const SEMANTIC_RECALL_BOUNDARY_TRUTH_DOC_REF: &str =
    "docs/search/m4/certify-docs-and-code-semantic-recall-boundaries.md";

/// Repo-relative path of the milestone-level doc note.
pub const SEMANTIC_RECALL_BOUNDARY_TRUTH_MILESTONE_DOC_REF: &str =
    "docs/m4/certify-docs-and-code-semantic-recall-boundaries.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const SEMANTIC_RECALL_BOUNDARY_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/search/m4/certify-docs-and-code-semantic-recall-boundaries.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const SEMANTIC_RECALL_BOUNDARY_TRUTH_FIXTURE_DIR: &str =
    "fixtures/search/m4/semantic_recall_boundary_truth_packet";

/// Repo-relative path of the checked-in stable boundary packet.
pub const SEMANTIC_RECALL_BOUNDARY_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/search/m4/semantic_recall_boundary_truth_packet.json";

/// Closed recall-lane vocabulary the packet certifies. Every required lane
/// MUST be covered by at least one row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecallLaneClass {
    /// Embedding/vector recall over the docs corpus.
    DocsSemanticRecall,
    /// Embedding/vector recall over the source-code corpus.
    CodeSemanticRecall,
    /// Hybrid fused recall mixing lexical, vector, and graph contributions.
    HybridFusedRecall,
    /// Lexical-only recall that bypasses the vector lane (fallback path).
    LexicalOnlyFallback,
}

impl RecallLaneClass {
    /// Every required recall-lane class, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::DocsSemanticRecall,
        Self::CodeSemanticRecall,
        Self::HybridFusedRecall,
        Self::LexicalOnlyFallback,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsSemanticRecall => "docs_semantic_recall",
            Self::CodeSemanticRecall => "code_semantic_recall",
            Self::HybridFusedRecall => "hybrid_fused_recall",
            Self::LexicalOnlyFallback => "lexical_only_fallback",
        }
    }

    /// True when this lane requires an embedder identity (vector-backed).
    pub const fn requires_embedder_identity(self) -> bool {
        !matches!(self, Self::LexicalOnlyFallback)
    }

    /// True when this lane requires a ranking-reason ref for fused promotions.
    pub const fn requires_ranking_reason(self) -> bool {
        matches!(self, Self::HybridFusedRecall)
    }
}

/// Closed surface-track vocabulary distinguishing the M4 stable lane from the
/// v1.x preview lane. Every certified packet MUST include rows from both
/// tracks so the boundary is documented, not implied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceTrack {
    /// Row covers a claimed M4 stable surface.
    M4Stable,
    /// Row covers an explicit v1.x preview surface that MUST NOT inherit
    /// adjacent green/stable rows.
    V1xPreview,
}

impl SurfaceTrack {
    /// Every required surface track.
    pub const REQUIRED: [Self; 2] = [Self::M4Stable, Self::V1xPreview];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::M4Stable => "m4_stable",
            Self::V1xPreview => "v1x_preview",
        }
    }
}

/// Closed locality vocabulary mirroring the ARCH-RETR-010 retrieval contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalityClass {
    /// Recall ran fully on the local device against a workspace index.
    Local,
    /// Recall used a remote helper service for embedding or candidate fetch.
    RemoteHelper,
    /// Recall used a signed, versioned mirrored pack.
    MirroredPack,
    /// Recall used a managed-mode service (cloud-hosted vector index).
    Managed,
}

impl LocalityClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::RemoteHelper => "remote_helper",
            Self::MirroredPack => "mirrored_pack",
            Self::Managed => "managed",
        }
    }

    /// True when this locality requires a verified pack signature.
    pub const fn requires_pack_signature(self) -> bool {
        matches!(self, Self::MirroredPack | Self::Managed)
    }
}

/// Closed retrieval-epoch state. The packet binds ARCH-RETR-010 invalidation
/// rules: changes to embedder, tokenizer, chunker, or retention advance the
/// epoch and invalidate affected semantic caches/indexes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalEpochState {
    /// Row's recall epoch is aligned with the current managed/local generation.
    CurrentEpochAligned,
    /// Row's recall epoch is older; the cache/index was invalidated.
    EpochMismatchInvalidated,
    /// Row's candidate set mixed generations; the row MUST NOT publish on stable.
    MixedGenerationBlocked,
    /// Row's lane is lexical-only; no embedder epoch applies.
    EpochNotApplicable,
}

impl RetrievalEpochState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentEpochAligned => "current_epoch_aligned",
            Self::EpochMismatchInvalidated => "epoch_mismatch_invalidated",
            Self::MixedGenerationBlocked => "mixed_generation_blocked",
            Self::EpochNotApplicable => "epoch_not_applicable",
        }
    }

    /// True when the row must carry a recorded downgrade and disclosure.
    pub const fn requires_downgrade(self) -> bool {
        matches!(
            self,
            Self::EpochMismatchInvalidated | Self::MixedGenerationBlocked
        )
    }
}

/// Closed pack-signature state for mirrored/managed recall.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackSignatureState {
    /// Pack was signed by a verified signer and the signature was verified.
    SignedAndVerified,
    /// Pack signature did not verify; recall MUST NOT claim stable on this row.
    SignatureMismatchBlocked,
    /// Local-only recall path; no pack signature applies.
    NotApplicableLocal,
}

impl PackSignatureState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedAndVerified => "signed_and_verified",
            Self::SignatureMismatchBlocked => "signature_mismatch_blocked",
            Self::NotApplicableLocal => "not_applicable_local",
        }
    }

    /// True when the row certifies a verified, signed pack.
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::SignedAndVerified)
    }
}

/// Closed downgrade-state vocabulary for the boundary row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeState {
    /// No downgrade; row preserves canonical recall truth.
    None,
    /// Locality downgraded (e.g. managed unavailable so the row fell back to local).
    LocalityDowngradedToLocal,
    /// Embedder unavailable; lane fell back to lexical-only recall.
    EmbedderUnavailableFallbackLexical,
    /// Retrieval epoch drifted and the affected indexes were invalidated.
    EpochDriftInvalidated,
    /// Pack signature failed to verify.
    PackSignatureFailed,
    /// Some candidates were withheld from the row by policy.
    PolicyOmittedCandidates,
    /// Candidates mixed generations and the row was blocked from stable.
    MixedGenerationBlocked,
    /// Pack compatibility drifted (signed but incompatible with current build).
    CompatibilityDriftDisclosed,
}

impl DowngradeState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::LocalityDowngradedToLocal => "locality_downgraded_to_local",
            Self::EmbedderUnavailableFallbackLexical => "embedder_unavailable_fallback_lexical",
            Self::EpochDriftInvalidated => "epoch_drift_invalidated",
            Self::PackSignatureFailed => "pack_signature_failed",
            Self::PolicyOmittedCandidates => "policy_omitted_candidates",
            Self::MixedGenerationBlocked => "mixed_generation_blocked",
            Self::CompatibilityDriftDisclosed => "compatibility_drift_disclosed",
        }
    }

    /// True when the row carries a visible downgrade.
    pub const fn is_downgraded(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Closed confidence-class vocabulary attached to a boundary row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceClass {
    /// High confidence — backed by canonical or signed evidence.
    High,
    /// Medium confidence — backed by partial or warming evidence.
    Medium,
    /// Low confidence — thin evidence.
    Low,
    /// Heuristic — labeled explicitly as a guess.
    Heuristic,
}

impl ConfidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
            Self::Heuristic => "heuristic",
        }
    }
}

/// Embedder identity carried by every non-lexical recall row.
///
/// Encodes the ARCH-RETR-010 generation tuple. Any change to one of these
/// fields advances the retrieval epoch and invalidates affected indexes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbedderIdentity {
    /// Stable model identifier (e.g. `embedder.docs.v3`).
    pub embedder_model_id: String,
    /// Model version label (e.g. `2026.04.0`).
    pub embedder_model_version: String,
    /// Tokenizer identity used to chunk inputs.
    pub tokenizer_id: String,
    /// Chunking strategy identity (e.g. `paragraph.v2`).
    pub chunking_strategy_id: String,
    /// Retention-policy identity that governs cache lifetime.
    pub retention_policy_id: String,
    /// Composite retrieval-epoch label derived from the tuple above.
    pub retrieval_epoch_label: String,
}

impl EmbedderIdentity {
    fn is_well_formed(&self) -> bool {
        !self.embedder_model_id.trim().is_empty()
            && !self.embedder_model_version.trim().is_empty()
            && !self.tokenizer_id.trim().is_empty()
            && !self.chunking_strategy_id.trim().is_empty()
            && !self.retention_policy_id.trim().is_empty()
            && !self.retrieval_epoch_label.trim().is_empty()
    }
}

/// Pack signature carried by mirrored/managed recall rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackSignature {
    /// Pack signature state.
    pub signature_state: PackSignatureState,
    /// Repo-relative ref to the pack identity.
    pub pack_id_ref: String,
    /// Pack version label.
    pub pack_version: String,
    /// Signer identity ref.
    pub signer_id_ref: String,
    /// Verification timestamp.
    pub verified_at: String,
}

impl PackSignature {
    fn is_well_formed(&self) -> bool {
        !self.pack_id_ref.trim().is_empty()
            && !self.pack_version.trim().is_empty()
            && !self.signer_id_ref.trim().is_empty()
            && !self.verified_at.trim().is_empty()
    }
}

/// Lane participation envelope: which lanes contributed candidates and which
/// lanes were omitted by policy/locality. Captured per row so retrieval
/// inspectors can explain *why* hybrid recall promoted a result and *which*
/// lanes were omitted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneParticipation {
    /// Lanes that contributed candidates to this row.
    pub participating_lane_refs: Vec<RecallLaneClass>,
    /// Lanes that were excluded by policy, locality, or compatibility.
    #[serde(default)]
    pub omitted_lane_refs: Vec<RecallLaneClass>,
    /// True when policy-hidden omissions were disclosed on the row.
    pub policy_hidden_omissions_disclosed: bool,
    /// Repo-relative ref explaining the policy-hidden omission disclosure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_hidden_omissions_disclosure_ref: Option<String>,
}

impl LaneParticipation {
    fn participating_includes(&self, lane: RecallLaneClass) -> bool {
        self.participating_lane_refs.contains(&lane)
    }

    fn discloses_policy_omissions(&self) -> bool {
        if !self.policy_hidden_omissions_disclosed {
            return self.omitted_lane_refs.is_empty();
        }
        self.policy_hidden_omissions_disclosure_ref
            .as_ref()
            .is_some_and(|reference| !reference.trim().is_empty())
    }
}

/// One certified boundary row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecallBoundaryRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Recall lane class this row certifies.
    pub recall_lane_class: RecallLaneClass,
    /// Surface track (M4 stable vs v1.x preview).
    pub surface_track: SurfaceTrack,
    /// Locality class (local, remote helper, mirrored pack, managed).
    pub locality_class: LocalityClass,
    /// Retrieval epoch state for the row.
    pub retrieval_epoch_state: RetrievalEpochState,
    /// Embedder identity used to mint candidates (None only for lexical rows).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embedder_identity: Option<EmbedderIdentity>,
    /// Pack signature (required for mirrored/managed locality).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pack_signature: Option<PackSignature>,
    /// Lane participation envelope.
    pub lane_participation: LaneParticipation,
    /// Repo-relative ref to the chunk-or-anchor provenance for the row.
    pub chunk_or_anchor_provenance_ref: String,
    /// Repo-relative ref to the ranking-reason explanation (hybrid rows only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ranking_reason_ref: Option<String>,
    /// Workspace identity.
    pub workspace_id: String,
    /// Downgrade state for the row.
    pub downgrade_state: DowngradeState,
    /// Confidence class for the row.
    pub confidence_class: ConfidenceClass,
    /// Repo-relative ref to the in-surface disclosure shown to the user.
    pub disclosure_ref: String,
    /// Evidence refs (docs, schemas, fixtures) cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// True when raw query text is excluded from this row.
    pub raw_query_text_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// True when raw source bodies and raw vectors are excluded from this row.
    pub raw_source_bodies_excluded: bool,
    /// Capture timestamp for this row.
    pub captured_at: String,
}

/// Consumer surface that must inherit this packet's truth verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Search shell result inspector.
    SearchShell,
    /// Docs/help surface explaining recall boundaries.
    DocsHelp,
    /// AI context assembly surface.
    AiContext,
    /// Review workspace evidence lane.
    ReviewWorkspace,
    /// CLI or headless inspection surface.
    CliHeadless,
    /// In-product retrieval inspector that explains hybrid promotions.
    RetrievalInspector,
    /// Support export bundle surface.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
}

impl ConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 8] = [
        Self::SearchShell,
        Self::DocsHelp,
        Self::AiContext,
        Self::ReviewWorkspace,
        Self::CliHeadless,
        Self::RetrievalInspector,
        Self::SupportExport,
        Self::ReleaseProofIndex,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchShell => "search_shell",
            Self::DocsHelp => "docs_help",
            Self::AiContext => "ai_context",
            Self::ReviewWorkspace => "review_workspace",
            Self::CliHeadless => "cli_headless",
            Self::RetrievalInspector => "retrieval_inspector",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
        }
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecallBoundaryConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Packet id consumed by the projection.
    pub semantic_recall_boundary_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the surface preserves the recall-lane vocabulary.
    pub preserves_lane_vocabulary: bool,
    /// True when the surface preserves the surface-track vocabulary.
    pub preserves_surface_track_vocabulary: bool,
    /// True when the surface preserves the locality vocabulary.
    pub preserves_locality_vocabulary: bool,
    /// True when the surface preserves the retrieval-epoch vocabulary.
    pub preserves_epoch_state_vocabulary: bool,
    /// True when the surface preserves the pack-signature vocabulary.
    pub preserves_pack_signature_vocabulary: bool,
    /// True when the surface preserves the downgrade-state vocabulary.
    pub preserves_downgrade_vocabulary: bool,
    /// True when the surface exposes embedder identity on managed/vector rows.
    pub exposes_embedder_identity: bool,
    /// True when the surface exposes participating-and-omitted lane refs.
    pub exposes_participating_and_omitted_lanes: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl SemanticRecallBoundaryConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.semantic_recall_boundary_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_surface_track_vocabulary
            && self.preserves_locality_vocabulary
            && self.preserves_epoch_state_vocabulary
            && self.preserves_pack_signature_vocabulary
            && self.preserves_downgrade_vocabulary
            && self.exposes_embedder_identity
            && self.exposes_participating_and_omitted_lanes
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Closed promotion state for [`SemanticRecallBoundaryTruthPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionState {
    /// Packet certifies a stable M4 claim for every declared row.
    Stable,
    /// Packet must remain narrowed below stable until a recorded gap closes.
    NarrowedBelowStable,
    /// Packet has a blocker finding and cannot publish on stable surfaces.
    BlocksStable,
}

impl PromotionState {
    /// Stable token used in fixtures, schemas, and support exports.
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
pub enum FindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the row below stable.
    Warning,
    /// Blocker that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required recall-lane row is missing from the packet.
    MissingLaneCoverage,
    /// Required surface-track row is missing from the packet.
    MissingSurfaceTrackCoverage,
    /// Row that requires an embedder identity dropped or malformed it.
    MissingEmbedderIdentity,
    /// Row dropped its chunk-or-anchor provenance ref.
    MissingChunkOrAnchorProvenance,
    /// Hybrid recall row dropped its ranking-reason ref.
    MissingRankingReason,
    /// Row dropped its disclosure ref.
    MissingDisclosureRef,
    /// Row's evidence-refs array is empty.
    MissingEvidenceRefs,
    /// Mirrored/managed row dropped its pack signature.
    MissingPackSignature,
    /// Pack signature failed verification (not signed_and_verified).
    PackSignatureNotVerified,
    /// Row's epoch is mismatch/mixed-generation but no downgrade is recorded.
    EpochMismatchPresentedAsCurrent,
    /// Row admits raw query text past the boundary.
    RawQueryTextPresent,
    /// Row admits raw source bodies or raw vectors past the boundary.
    RawSourceBodiesPresent,
    /// Row admits secrets past the boundary.
    SecretsPresent,
    /// Row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// Row has omitted lanes but did not disclose policy-hidden omissions.
    PolicyOmissionsUndisclosed,
    /// Row is marked v1.x preview but claims no downgrade (must narrow below stable).
    PreviewRowClaimsM4StableCertainty,
    /// Row is marked m4_stable but its embedder identity is absent.
    UnlabeledManagedOrVectorMatch,
    /// A required consumer projection is missing.
    MissingConsumerProjection,
    /// A consumer projection drops part of the closed vocabulary.
    ConsumerProjectionDrift,
    /// A consumer projection collapses the recall-lane vocabulary.
    LaneVocabularyCollapsed,
    /// A consumer projection collapses the surface-track vocabulary.
    SurfaceTrackVocabularyCollapsed,
    /// A consumer projection collapses the locality vocabulary.
    LocalityVocabularyCollapsed,
    /// A consumer projection collapses the retrieval-epoch vocabulary.
    EpochStateVocabularyCollapsed,
    /// A consumer projection collapses the pack-signature vocabulary.
    PackSignatureVocabularyCollapsed,
    /// A consumer projection collapses the downgrade-state vocabulary.
    DowngradeVocabularyCollapsed,
    /// A consumer projection drops embedder identity exposure.
    EmbedderIdentityDropped,
    /// A consumer projection drops the participating/omitted lane exposure.
    LaneParticipationDropped,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl FindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingLaneCoverage => "missing_lane_coverage",
            Self::MissingSurfaceTrackCoverage => "missing_surface_track_coverage",
            Self::MissingEmbedderIdentity => "missing_embedder_identity",
            Self::MissingChunkOrAnchorProvenance => "missing_chunk_or_anchor_provenance",
            Self::MissingRankingReason => "missing_ranking_reason",
            Self::MissingDisclosureRef => "missing_disclosure_ref",
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::MissingPackSignature => "missing_pack_signature",
            Self::PackSignatureNotVerified => "pack_signature_not_verified",
            Self::EpochMismatchPresentedAsCurrent => "epoch_mismatch_presented_as_current",
            Self::RawQueryTextPresent => "raw_query_text_present",
            Self::RawSourceBodiesPresent => "raw_source_bodies_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::PolicyOmissionsUndisclosed => "policy_omissions_undisclosed",
            Self::PreviewRowClaimsM4StableCertainty => "preview_row_claims_m4_stable_certainty",
            Self::UnlabeledManagedOrVectorMatch => "unlabeled_managed_or_vector_match",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::SurfaceTrackVocabularyCollapsed => "surface_track_vocabulary_collapsed",
            Self::LocalityVocabularyCollapsed => "locality_vocabulary_collapsed",
            Self::EpochStateVocabularyCollapsed => "epoch_state_vocabulary_collapsed",
            Self::PackSignatureVocabularyCollapsed => "pack_signature_vocabulary_collapsed",
            Self::DowngradeVocabularyCollapsed => "downgrade_vocabulary_collapsed",
            Self::EmbedderIdentityDropped => "embedder_identity_dropped",
            Self::LaneParticipationDropped => "lane_participation_dropped",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    /// Closed finding kind.
    pub finding_kind: FindingKind,
    /// Finding severity.
    pub severity: FindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl ValidationFinding {
    fn new(
        finding_kind: FindingKind,
        severity: FindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`SemanticRecallBoundaryTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecallBoundaryTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Packet capture timestamp.
    pub generated_at: String,
    /// Recall-lane classes the packet covers.
    #[serde(default)]
    pub covered_lane_classes: Vec<RecallLaneClass>,
    /// Surface tracks the packet covers.
    #[serde(default)]
    pub covered_surface_tracks: Vec<SurfaceTrack>,
    /// Truth rows.
    #[serde(default)]
    pub rows: Vec<SemanticRecallBoundaryRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<SemanticRecallBoundaryConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Docs/search-owned packet for the M4 stable semantic-recall boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecallBoundaryTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Packet capture timestamp.
    pub generated_at: String,
    /// Recall-lane classes the packet covers.
    #[serde(default)]
    pub covered_lane_classes: Vec<RecallLaneClass>,
    /// Surface tracks the packet covers.
    #[serde(default)]
    pub covered_surface_tracks: Vec<SurfaceTrack>,
    /// Truth rows.
    #[serde(default)]
    pub rows: Vec<SemanticRecallBoundaryRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<SemanticRecallBoundaryConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl SemanticRecallBoundaryTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: SemanticRecallBoundaryTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: SEMANTIC_RECALL_BOUNDARY_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: SEMANTIC_RECALL_BOUNDARY_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_lane_classes: input.covered_lane_classes,
            covered_surface_tracks: input.covered_surface_tracks,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: PromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against the stable semantic-recall boundary invariants.
    pub fn validate(&self) -> Vec<ValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: ConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique recall-lane tokens observed across rows.
    pub fn lane_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.recall_lane_class);
        }
        set.into_iter().map(RecallLaneClass::as_str).collect()
    }

    /// Returns the unique surface-track tokens observed across rows.
    pub fn surface_track_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.surface_track);
        }
        set.into_iter().map(SurfaceTrack::as_str).collect()
    }

    /// Returns the unique locality tokens observed across rows.
    pub fn locality_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.locality_class);
        }
        set.into_iter().map(LocalityClass::as_str).collect()
    }

    /// Returns the unique retrieval-epoch tokens observed across rows.
    pub fn epoch_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.retrieval_epoch_state);
        }
        set.into_iter().map(RetrievalEpochState::as_str).collect()
    }

    /// Returns the unique pack-signature tokens observed across rows.
    pub fn pack_signature_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            if let Some(signature) = row.pack_signature.as_ref() {
                set.insert(signature.signature_state);
            }
        }
        set.into_iter().map(PackSignatureState::as_str).collect()
    }

    /// Returns the unique downgrade-state tokens observed across rows.
    pub fn downgrade_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.downgrade_state);
        }
        set.into_iter().map(DowngradeState::as_str).collect()
    }

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> SemanticRecallBoundaryTruthSupportExport {
        SemanticRecallBoundaryTruthSupportExport {
            record_kind: SEMANTIC_RECALL_BOUNDARY_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SEMANTIC_RECALL_BOUNDARY_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            semantic_recall_boundary_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            semantic_recall_boundary_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != SEMANTIC_RECALL_BOUNDARY_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "semantic-recall boundary packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != SEMANTIC_RECALL_BOUNDARY_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "semantic-recall boundary packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }

        for required in RecallLaneClass::REQUIRED {
            let in_coverage = self.covered_lane_classes.contains(&required);
            let in_rows = self
                .rows
                .iter()
                .any(|row| row.recall_lane_class == required);
            if !in_coverage || !in_rows {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingLaneCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "no row covers required recall lane class {}",
                        required.as_str()
                    ),
                ));
            }
        }

        for required in SurfaceTrack::REQUIRED {
            let in_coverage = self.covered_surface_tracks.contains(&required);
            let in_rows = self.rows.iter().any(|row| row.surface_track == required);
            if !in_coverage || !in_rows {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingSurfaceTrackCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers required surface track {}", required.as_str()),
                ));
            }
        }

        for row in &self.rows {
            if row.row_id.trim().is_empty()
                || row.workspace_id.trim().is_empty()
                || row.captured_at.trim().is_empty()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingIdentity,
                    FindingSeverity::Blocker,
                    format!("row {} drops a required identity field", row.row_id),
                ));
            }
            if row.disclosure_ref.trim().is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!("row {} drops its disclosure ref", row.row_id),
                ));
            }
            if row.evidence_refs.is_empty()
                || row
                    .evidence_refs
                    .iter()
                    .any(|reference| reference.trim().is_empty())
            {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingEvidenceRefs,
                    FindingSeverity::Blocker,
                    format!("row {} drops evidence refs", row.row_id),
                ));
            }
            if row.chunk_or_anchor_provenance_ref.trim().is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingChunkOrAnchorProvenance,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} drops its chunk-or-anchor provenance ref",
                        row.row_id
                    ),
                ));
            }

            if row.recall_lane_class.requires_embedder_identity() {
                let embedder_ok = row
                    .embedder_identity
                    .as_ref()
                    .is_some_and(EmbedderIdentity::is_well_formed);
                if !embedder_ok {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingEmbedderIdentity,
                        FindingSeverity::Blocker,
                        format!("row {} drops or malforms its embedder identity", row.row_id),
                    ));
                }
                if !row
                    .lane_participation
                    .participating_includes(row.recall_lane_class)
                {
                    findings.push(ValidationFinding::new(
                        FindingKind::UnlabeledManagedOrVectorMatch,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} lane {} is not in its participating-lane refs",
                            row.row_id,
                            row.recall_lane_class.as_str()
                        ),
                    ));
                }
            }

            if row.recall_lane_class.requires_ranking_reason() {
                let reason_ok = row
                    .ranking_reason_ref
                    .as_ref()
                    .is_some_and(|reference| !reference.trim().is_empty());
                if !reason_ok {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingRankingReason,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} is hybrid recall but missing ranking-reason ref",
                            row.row_id
                        ),
                    ));
                }
            }

            if row.locality_class.requires_pack_signature() {
                match row.pack_signature.as_ref() {
                    None => {
                        findings.push(ValidationFinding::new(
                            FindingKind::MissingPackSignature,
                            FindingSeverity::Blocker,
                            format!(
                                "row {} locality {} requires a pack signature",
                                row.row_id,
                                row.locality_class.as_str()
                            ),
                        ));
                    }
                    Some(signature) => {
                        if !signature.is_well_formed() {
                            findings.push(ValidationFinding::new(
                                FindingKind::MissingPackSignature,
                                FindingSeverity::Blocker,
                                format!("row {} pack signature is malformed", row.row_id),
                            ));
                        }
                        if !signature.signature_state.is_verified() {
                            findings.push(ValidationFinding::new(
                                FindingKind::PackSignatureNotVerified,
                                FindingSeverity::Blocker,
                                format!(
                                    "row {} pack signature state is {}",
                                    row.row_id,
                                    signature.signature_state.as_str()
                                ),
                            ));
                        }
                    }
                }
            }

            if row.retrieval_epoch_state.requires_downgrade()
                && !row.downgrade_state.is_downgraded()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::EpochMismatchPresentedAsCurrent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} epoch is {} but downgrade state is {}",
                        row.row_id,
                        row.retrieval_epoch_state.as_str(),
                        row.downgrade_state.as_str()
                    ),
                ));
            }

            if !row.lane_participation.discloses_policy_omissions() {
                findings.push(ValidationFinding::new(
                    FindingKind::PolicyOmissionsUndisclosed,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} omits lanes without disclosing policy-hidden omissions",
                        row.row_id
                    ),
                ));
            }

            if row.surface_track == SurfaceTrack::V1xPreview && !row.downgrade_state.is_downgraded()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::PreviewRowClaimsM4StableCertainty,
                    FindingSeverity::Blocker,
                    format!("row {} is v1x_preview but claims no downgrade", row.row_id),
                ));
            }

            if !row.raw_query_text_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::RawQueryTextPresent,
                    FindingSeverity::Blocker,
                    format!("row {} admits raw query text", row.row_id),
                ));
            }
            if !row.raw_source_bodies_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::RawSourceBodiesPresent,
                    FindingSeverity::Blocker,
                    format!("row {} admits raw source bodies or raw vectors", row.row_id),
                ));
            }
            if !row.secrets_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::SecretsPresent,
                    FindingSeverity::Blocker,
                    format!("row {} admits secrets", row.row_id),
                ));
            }
            if !row.ambient_authority_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::AmbientAuthorityPresent,
                    FindingSeverity::Blocker,
                    format!("row {} admits ambient authority/credentials", row.row_id),
                ));
            }
        }

        for required_surface in ConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingConsumerProjection,
                    FindingSeverity::Blocker,
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
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerProjectionDrift,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve semantic-recall boundary truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_lane_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::LaneVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the recall-lane vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_surface_track_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::SurfaceTrackVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the surface-track vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_locality_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::LocalityVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the locality vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_epoch_state_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::EpochStateVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the retrieval-epoch vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_pack_signature_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::PackSignatureVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the pack-signature vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_downgrade_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::DowngradeVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} drops the downgrade-state vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.exposes_embedder_identity {
                findings.push(ValidationFinding::new(
                    FindingKind::EmbedderIdentityDropped,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} drops embedder identity exposure",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.exposes_participating_and_omitted_lanes {
                findings.push(ValidationFinding::new(
                    FindingKind::LaneParticipationDropped,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} drops participating/omitted lane exposure",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion
                .retain(|finding| finding.finding_kind != FindingKind::PromotionStateMismatch);
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(ValidationFinding::new(
                    FindingKind::PromotionStateMismatch,
                    FindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn promotion_state_for_findings(findings: &[ValidationFinding]) -> PromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Blocker)
    {
        PromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Warning)
    {
        PromotionState::NarrowedBelowStable
    } else {
        PromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecallBoundaryTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub semantic_recall_boundary_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub semantic_recall_boundary_packet: SemanticRecallBoundaryTruthPacket,
}

impl SemanticRecallBoundaryTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == SEMANTIC_RECALL_BOUNDARY_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == SEMANTIC_RECALL_BOUNDARY_TRUTH_SCHEMA_VERSION
            && self.semantic_recall_boundary_packet_id_ref
                == self.semantic_recall_boundary_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.semantic_recall_boundary_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable packet.
#[derive(Debug)]
pub enum SemanticRecallBoundaryTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for SemanticRecallBoundaryTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "semantic-recall boundary truth packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "semantic-recall boundary truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for SemanticRecallBoundaryTruthArtifactError {}

/// Returns the checked-in stable semantic-recall boundary truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_semantic_recall_boundary_truth_packet(
) -> Result<SemanticRecallBoundaryTruthPacket, SemanticRecallBoundaryTruthArtifactError> {
    let packet: SemanticRecallBoundaryTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/search/m4/semantic_recall_boundary_truth_packet.json"
    )))
    .map_err(SemanticRecallBoundaryTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(SemanticRecallBoundaryTruthArtifactError::Validation(
            findings,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_embedder_identity() -> EmbedderIdentity {
        EmbedderIdentity {
            embedder_model_id: "embedder.docs.v3".to_owned(),
            embedder_model_version: "2026.04.0".to_owned(),
            tokenizer_id: "tokenizer.docs.v2".to_owned(),
            chunking_strategy_id: "chunker.paragraph.v2".to_owned(),
            retention_policy_id: "retention.workspace.v5".to_owned(),
            retrieval_epoch_label:
                "embedder=v3+tokenizer=v2+chunker=paragraph.v2+retention=workspace.v5".to_owned(),
        }
    }

    fn sample_pack_signature() -> PackSignature {
        PackSignature {
            signature_state: PackSignatureState::SignedAndVerified,
            pack_id_ref: "pack:docs:mirrored:v3".to_owned(),
            pack_version: "2026.04.0".to_owned(),
            signer_id_ref: "signer:aureline:packs".to_owned(),
            verified_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn sample_lane_participation(lane: RecallLaneClass) -> LaneParticipation {
        LaneParticipation {
            participating_lane_refs: vec![lane],
            omitted_lane_refs: Vec::new(),
            policy_hidden_omissions_disclosed: false,
            policy_hidden_omissions_disclosure_ref: None,
        }
    }

    fn sample_row(
        lane: RecallLaneClass,
        track: SurfaceTrack,
        locality: LocalityClass,
    ) -> SemanticRecallBoundaryRow {
        let preview = matches!(track, SurfaceTrack::V1xPreview);
        SemanticRecallBoundaryRow {
            row_id: format!("row:{}:{}", lane.as_str(), track.as_str()),
            recall_lane_class: lane,
            surface_track: track,
            locality_class: locality,
            retrieval_epoch_state: if lane.requires_embedder_identity() {
                RetrievalEpochState::CurrentEpochAligned
            } else {
                RetrievalEpochState::EpochNotApplicable
            },
            embedder_identity: if lane.requires_embedder_identity() {
                Some(sample_embedder_identity())
            } else {
                None
            },
            pack_signature: if locality.requires_pack_signature() {
                Some(sample_pack_signature())
            } else {
                None
            },
            lane_participation: sample_lane_participation(lane),
            chunk_or_anchor_provenance_ref: format!("provenance:{}:baseline", lane.as_str()),
            ranking_reason_ref: if lane.requires_ranking_reason() {
                Some(format!("ranking_reason:{}:baseline", lane.as_str()))
            } else {
                None
            },
            workspace_id: "workspace:fixture".to_owned(),
            downgrade_state: if preview {
                DowngradeState::CompatibilityDriftDisclosed
            } else {
                DowngradeState::None
            },
            confidence_class: if preview {
                ConfidenceClass::Medium
            } else {
                ConfidenceClass::High
            },
            disclosure_ref: SEMANTIC_RECALL_BOUNDARY_TRUTH_DOC_REF.to_owned(),
            evidence_refs: vec![SEMANTIC_RECALL_BOUNDARY_TRUTH_DOC_REF.to_owned()],
            raw_query_text_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            raw_source_bodies_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn sample_projection(surface: ConsumerSurface) -> SemanticRecallBoundaryConsumerProjection {
        SemanticRecallBoundaryConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            semantic_recall_boundary_packet_id_ref: "packet:m4:semantic_recall_boundary:baseline"
                .to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_surface_track_vocabulary: true,
            preserves_locality_vocabulary: true,
            preserves_epoch_state_vocabulary: true,
            preserves_pack_signature_vocabulary: true,
            preserves_downgrade_vocabulary: true,
            exposes_embedder_identity: true,
            exposes_participating_and_omitted_lanes: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn baseline_input() -> SemanticRecallBoundaryTruthPacketInput {
        SemanticRecallBoundaryTruthPacketInput {
            packet_id: "packet:m4:semantic_recall_boundary:baseline".to_owned(),
            workflow_or_surface_id: "workflow.search_docs.semantic_recall_boundary.baseline"
                .to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_lane_classes: RecallLaneClass::REQUIRED.to_vec(),
            covered_surface_tracks: SurfaceTrack::REQUIRED.to_vec(),
            rows: vec![
                sample_row(
                    RecallLaneClass::DocsSemanticRecall,
                    SurfaceTrack::M4Stable,
                    LocalityClass::Local,
                ),
                sample_row(
                    RecallLaneClass::CodeSemanticRecall,
                    SurfaceTrack::M4Stable,
                    LocalityClass::MirroredPack,
                ),
                sample_row(
                    RecallLaneClass::HybridFusedRecall,
                    SurfaceTrack::M4Stable,
                    LocalityClass::Managed,
                ),
                sample_row(
                    RecallLaneClass::LexicalOnlyFallback,
                    SurfaceTrack::M4Stable,
                    LocalityClass::Local,
                ),
                sample_row(
                    RecallLaneClass::DocsSemanticRecall,
                    SurfaceTrack::V1xPreview,
                    LocalityClass::RemoteHelper,
                ),
            ],
            consumer_projections: ConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(sample_projection)
                .collect(),
            source_contract_refs: vec![SEMANTIC_RECALL_BOUNDARY_TRUTH_DOC_REF.to_owned()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(
            RecallLaneClass::DocsSemanticRecall.as_str(),
            "docs_semantic_recall"
        );
        assert_eq!(SurfaceTrack::M4Stable.as_str(), "m4_stable");
        assert_eq!(SurfaceTrack::V1xPreview.as_str(), "v1x_preview");
        assert_eq!(LocalityClass::MirroredPack.as_str(), "mirrored_pack");
        assert_eq!(
            RetrievalEpochState::MixedGenerationBlocked.as_str(),
            "mixed_generation_blocked"
        );
        assert_eq!(
            PackSignatureState::SignedAndVerified.as_str(),
            "signed_and_verified"
        );
        assert_eq!(
            DowngradeState::EpochDriftInvalidated.as_str(),
            "epoch_drift_invalidated"
        );
        assert_eq!(
            FindingKind::UnlabeledManagedOrVectorMatch.as_str(),
            "unlabeled_managed_or_vector_match"
        );
        assert_eq!(PromotionState::BlocksStable.as_str(), "blocks_stable");
    }

    #[test]
    fn baseline_input_materializes_stable() {
        let packet = SemanticRecallBoundaryTruthPacket::materialize(baseline_input());
        assert_eq!(packet.promotion_state, PromotionState::Stable);
        assert!(packet.validation_findings.is_empty());
        assert!(packet.is_stable());
    }

    #[test]
    fn raw_query_text_blocks_stable() {
        let mut input = baseline_input();
        input.rows[0].raw_query_text_excluded = false;
        let packet = SemanticRecallBoundaryTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawQueryTextPresent));
    }

    #[test]
    fn missing_embedder_identity_blocks_stable() {
        let mut input = baseline_input();
        input.rows[0].embedder_identity = None;
        let packet = SemanticRecallBoundaryTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| { finding.finding_kind == FindingKind::MissingEmbedderIdentity }));
    }

    #[test]
    fn unsigned_mirrored_pack_blocks_stable() {
        let mut input = baseline_input();
        let mirrored = input
            .rows
            .iter_mut()
            .find(|row| row.locality_class == LocalityClass::MirroredPack)
            .expect("mirrored pack row");
        mirrored.pack_signature = None;
        let packet = SemanticRecallBoundaryTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingPackSignature));
    }

    #[test]
    fn mixed_generation_presented_as_current_blocks_stable() {
        let mut input = baseline_input();
        let docs_row = input
            .rows
            .iter_mut()
            .find(|row| {
                row.recall_lane_class == RecallLaneClass::DocsSemanticRecall
                    && row.surface_track == SurfaceTrack::M4Stable
            })
            .expect("docs stable row");
        docs_row.retrieval_epoch_state = RetrievalEpochState::MixedGenerationBlocked;
        docs_row.downgrade_state = DowngradeState::None;
        let packet = SemanticRecallBoundaryTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::EpochMismatchPresentedAsCurrent
        }));
    }

    #[test]
    fn preview_row_without_downgrade_blocks_stable() {
        let mut input = baseline_input();
        let preview_row = input
            .rows
            .iter_mut()
            .find(|row| row.surface_track == SurfaceTrack::V1xPreview)
            .expect("preview row");
        preview_row.downgrade_state = DowngradeState::None;
        let packet = SemanticRecallBoundaryTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::PreviewRowClaimsM4StableCertainty
        }));
    }

    #[test]
    fn policy_omissions_undisclosed_blocks_stable() {
        let mut input = baseline_input();
        let row = &mut input.rows[2];
        row.lane_participation.omitted_lane_refs = vec![RecallLaneClass::LexicalOnlyFallback];
        row.lane_participation.policy_hidden_omissions_disclosed = false;
        row.lane_participation
            .policy_hidden_omissions_disclosure_ref = None;
        let packet = SemanticRecallBoundaryTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| { finding.finding_kind == FindingKind::PolicyOmissionsUndisclosed }));
    }

    #[test]
    fn missing_lane_coverage_blocks_stable() {
        let mut input = baseline_input();
        input
            .rows
            .retain(|row| row.recall_lane_class != RecallLaneClass::LexicalOnlyFallback);
        input
            .covered_lane_classes
            .retain(|lane| *lane != RecallLaneClass::LexicalOnlyFallback);
        let packet = SemanticRecallBoundaryTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingLaneCoverage));
    }

    #[test]
    fn missing_consumer_projection_blocks_stable() {
        let mut input = baseline_input();
        input.consumer_projections = vec![sample_projection(ConsumerSurface::SearchShell)];
        let packet = SemanticRecallBoundaryTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn projection_dropping_embedder_identity_blocks_stable() {
        let mut input = baseline_input();
        input.consumer_projections = ConsumerSurface::REQUIRED
            .iter()
            .copied()
            .map(|surface| {
                let mut projection = sample_projection(surface);
                if surface == ConsumerSurface::RetrievalInspector {
                    projection.exposes_embedder_identity = false;
                }
                projection
            })
            .collect();
        let packet = SemanticRecallBoundaryTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| { finding.finding_kind == FindingKind::EmbedderIdentityDropped }));
    }

    #[test]
    fn support_export_is_export_safe_when_packet_is_stable() {
        let packet = SemanticRecallBoundaryTruthPacket::materialize(baseline_input());
        let export = packet.support_export("export:test", "2026-05-26T12:00:10Z");
        assert!(export.is_export_safe());
    }
}
