//! Imported / offline evidence lineage propagation across downstream consumers, so non-live data can never
//! masquerade as current route, provider, or service truth once it flows into a companion card, a browser /
//! export handoff, a support packet, or an AI explanation / evidence consumer.
//!
//! This module is the B149 cross-surface lineage-propagation lane over the five non-live-evidence object classes
//! frozen in [`crate::m5_historical_reference_matrix`]. Where the archive-viewer lane
//! ([`crate::m5_archived_snapshot_viewer_and_analysis_only_banner_consumers`]) proves how a preserved snapshot is
//! *shown* as non-live, the state lane
//! ([`crate::m5_archived_object_expiry_removal_state_and_metadata_fallback`]) keeps it honest after its retention
//! window closes, and the live-target-handoff lane
//! ([`crate::m5_live_target_handoff_packet_and_route_validation`]) makes reopening a current object a validated
//! pivot, this lane carries the imported / offline evidence *descriptor and its "Showing imported or offline
//! evidence" label* into the first downstream consumers that can ingest archived data — proving they render the
//! same non-live vocabulary and lineage fields as the primary archive viewer, never rank / narrate / summarize a
//! historical packet as current live service truth, and always join the lineage back to its source snapshot
//! descriptor and any live-target handoff packet so provenance and safe exits survive the hop.
//!
//! The three honesty axes mirror the row acceptance criteria.
//!
//! 1. **At least one companion / export surface and one AI / support consumer display imported / offline evidence
//!    with the same non-live vocabulary and lineage fields as the primary archive viewer.** Every binding carries
//!    the controlled [`NonLiveEvidenceGrammar`] — the historical-role, snapshot-label, capture-time, provenance,
//!    mutation-blocked-posture, and "Showing imported or offline evidence" words — identical for one profile
//!    across every surface, and a [`LineageDescriptor`] joining the packet back to its source snapshot descriptor.
//! 2. **Historical packets cannot silently appear as current route, health, or provider state in claimed M5
//!    downstream consumers.** The consumer action set ([`LineageConsumerAction`]) is closed and analysis-only —
//!    there is no rank / narrate-as-current affordance — the non-live boundary is always called out, and the
//!    `ranked_or_narrated_as_current_live_service_truth` and
//!    `presents_imported_offline_as_current_route_or_provider_state` guardrails must both be `false`.
//! 3. **The first imported / offline evidence flows remain export-safe and do not leak live secrets or stale
//!    authority through lineage metadata.** Each descriptor names its live-target handoff packet or metadata-only
//!    exit by controlled id rather than embedding a live route or secret, the export is scrubbed for forbidden
//!    boundary material, and the `leaks_live_secret_or_stale_authority_through_lineage` guardrail must be `false`.
//!
//! Every binding names the accessibility routes ([`M5HistoricalReferenceAccessibilityRoute`]) through which the
//! non-live boundary, its provenance, and its lineage join can be discovered without pointer-only chrome;
//! keyboard focus and screen-reader announcement are mandatory. The historical side stays visibly non-live and
//! mutation blocked throughout, and the non-live grammar is identical across every surface that renders the same
//! profile.
//!
//! The boundary schema is
//! [`schemas/program/m5-imported-offline-evidence-lineage-propagation.schema.json`](../../../../schemas/program/m5-imported-offline-evidence-lineage-propagation.schema.json).
//! The contract doc is
//! [`docs/support/m5_imported_offline_evidence_lineage_propagation.md`](../../../../docs/support/m5_imported_offline_evidence_lineage_propagation.md).
//! The protected fixture directory is
//! [`fixtures/recovery/m5-imported-offline-lineage/`](../../../../fixtures/recovery/m5-imported-offline-lineage/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_imported_offline_lineage,
    seeded_m5_imported_offline_lineage_imported_offline_narrowed,
    seeded_m5_imported_offline_lineage_metadata_only_narrowed,
};

use crate::m5_historical_reference_matrix::{
    M5HistoricalReferenceAccessibilityRoute, M5HistoricalReferenceConsumerSurface,
    M5HistoricalReferenceObject, M5HistoricalReferenceRole, M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF,
    M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ImportedOfflineLineagePacket`].
pub const M5_IMPORTED_OFFLINE_LINEAGE_RECORD_KIND: &str = "m5_imported_offline_lineage_registry";

/// Schema version for imported / offline lineage records.
pub const M5_IMPORTED_OFFLINE_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_IMPORTED_OFFLINE_LINEAGE_PACKET_ID: &str = "m5-imported-offline-lineage:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_IMPORTED_OFFLINE_LINEAGE_SCHEMA_REF: &str =
    "schemas/program/m5-imported-offline-evidence-lineage-propagation.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_IMPORTED_OFFLINE_LINEAGE_DOC_REF: &str =
    "docs/support/m5_imported_offline_evidence_lineage_propagation.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_IMPORTED_OFFLINE_LINEAGE_ARTIFACT_REF: &str =
    "artifacts/support/m5-imported-offline-lineage/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_IMPORTED_OFFLINE_LINEAGE_CSV_REF: &str =
    "artifacts/support/m5-imported-offline-lineage/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_IMPORTED_OFFLINE_LINEAGE_REPORT_REF: &str =
    "artifacts/support/m5-imported-offline-lineage/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_IMPORTED_OFFLINE_LINEAGE_FIXTURE_DIR: &str =
    "fixtures/recovery/m5-imported-offline-lineage";

/// The canonical non-live label word every propagated imported / offline evidence descriptor carries.
pub const M5_IMPORTED_OFFLINE_LABEL: &str = "Showing imported or offline evidence";

/// Proof-freshness SLO in hours for this lane.
pub const M5_IMPORTED_OFFLINE_LINEAGE_PROOF_SLO_HOURS: u32 = 720;

/// Mutation-blocked-posture sentinel words a non-live grammar may never fall back to; an imported / offline
/// evidence descriptor whose historical role must be present before surfacing as non-live evidence must always
/// keep a real mutation-blocked posture rather than implying the object is editable, live, writable, or current.
const MUTATION_BLOCKED_POSTURE_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "editable",
    "live_object",
    "writable",
    "current_object",
];

/// Whether a consumer surface is an export / support path that must map an object class back to its canonical
/// contract by id.
pub const fn consumer_must_reference_canonical(
    consumer: M5HistoricalReferenceConsumerSurface,
) -> bool {
    matches!(
        consumer,
        M5HistoricalReferenceConsumerSurface::Support
            | M5HistoricalReferenceConsumerSurface::CliExport
    )
}

/// Whether `token` is a member of the frozen [`M5HistoricalReferenceRole`] vocabulary.
pub fn is_known_historical_reference_role_token(token: &str) -> bool {
    historical_reference_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5HistoricalReferenceRole`], if it is one.
pub fn historical_reference_role_from_token(token: &str) -> Option<M5HistoricalReferenceRole> {
    M5HistoricalReferenceRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// The disposition of a propagated imported / offline evidence lineage descriptor at a downstream consumer.
///
/// The disposition governs the discoverable action set, parity, content availability, and lineage join — never
/// the non-live grammar: an imported / offline descriptor always carries the same historical-role, snapshot-label,
/// capture-time, provenance, mutation-blocked-posture, and "Showing imported or offline evidence" words, and
/// discloses its non-live boundary through an explicit lineage descriptor plus a live-target-handoff or
/// metadata-only exit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceLineageDisposition {
    /// The propagated lineage joins back to a validated live-target handoff packet, so an explicit
    /// open-current-live-object action is offered without widening authority.
    LiveTargetJoinable,
    /// The propagated lineage is imported / offline evidence with no live counterpart; its non-live boundary is
    /// called out and no open-current-live-object action is offered.
    ImportedOfflineOnly,
    /// Only the lineage metadata is retained; the content is unavailable, so the descriptor renders metadata and
    /// its lineage join rather than a blank pane and offers a metadata-only exit.
    MetadataOnlyExit,
    /// The lineage was carried into a browser / export handoff, redacted and secret-free; it stays non-live and
    /// offers a metadata-only exit rather than a live route.
    ExportedRedactedLineage,
}

impl EvidenceLineageDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LiveTargetJoinable,
        Self::ImportedOfflineOnly,
        Self::MetadataOnlyExit,
        Self::ExportedRedactedLineage,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTargetJoinable => "live_target_joinable",
            Self::ImportedOfflineOnly => "imported_offline_only",
            Self::MetadataOnlyExit => "metadata_only_exit",
            Self::ExportedRedactedLineage => "exported_redacted_lineage",
        }
    }

    /// A stable, human-facing default label for the disposition.
    pub const fn stable_label(self) -> &'static str {
        match self {
            Self::LiveTargetJoinable => "Imported / offline evidence (live target joinable)",
            Self::ImportedOfflineOnly => "Imported / offline evidence only (no live counterpart)",
            Self::MetadataOnlyExit => "Imported / offline evidence (metadata-only exit)",
            Self::ExportedRedactedLineage => "Imported / offline evidence (exported, redacted)",
        }
    }

    /// Whether this disposition joins back to a live-target handoff packet.
    pub const fn is_live_target_joinable(self) -> bool {
        matches!(self, Self::LiveTargetJoinable)
    }
}

/// The action a downstream lineage consumer may expose.
///
/// The set is deliberately closed and analysis-only: there is no rank / narrate / summarize-as-current, apply,
/// sync, or restore action. `OpenCurrentLiveObject` appears only when the lineage joins back to a validated
/// live-target handoff packet, so a downstream consumer can never present imported / offline evidence as if it
/// were current live service truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineageConsumerAction {
    /// Inspect the propagated lineage descriptor / metadata only.
    InspectLineage,
    /// Export the lineage descriptor record.
    ExportLineage,
    /// Open the current live object — only when the lineage joins back to a validated live-target handoff packet.
    OpenCurrentLiveObject,
}

impl LineageConsumerAction {
    /// The analysis-only base action set present on every downstream lineage consumer.
    pub const BASE: [Self; 2] = [Self::InspectLineage, Self::ExportLineage];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectLineage => "inspect_lineage",
            Self::ExportLineage => "export_lineage",
            Self::OpenCurrentLiveObject => "open_current_live_object",
        }
    }
}

/// The next action a lineage descriptor offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineageNextAction {
    /// Open the current live object through the joined, validated live-target handoff.
    OpenCurrentLiveObjectThroughValidatedHandoff,
    /// Inspect the lineage metadata only when no live counterpart remains.
    InspectLineageMetadataOnly,
}

impl LineageNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenCurrentLiveObjectThroughValidatedHandoff => {
                "open_current_live_object_through_validated_handoff"
            }
            Self::InspectLineageMetadataOnly => "inspect_lineage_metadata_only",
        }
    }
}

/// Whether a binding joins a live target or discloses a non-live boundary only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineageParity {
    /// The lineage joins a validated live-target handoff and shows an open-current-live-object action.
    LiveTargetLineageJoined,
    /// The non-live boundary is explicitly disclosed with a metadata-only exit.
    NonLiveBoundaryDisclosed,
}

impl LineageParity {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTargetLineageJoined => "live_target_lineage_joined",
            Self::NonLiveBoundaryDisclosed => "non_live_boundary_disclosed",
        }
    }
}

/// Downgrade trigger that can narrow this lineage-propagation lane below its claimed parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportedOfflineLineageDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Non-live grammar drifted between surfaces for the same profile.
    GrammarDriftDetected,
    /// A consumer dropped its mutation-blocked posture and began to imply the evidence is live.
    MutationBlockedPostureDropped,
    /// A consumer ranked, narrated, or summarized a historical packet as current live service truth.
    RankedOrNarratedAsCurrentLiveServiceTruth,
    /// A consumer presented imported / offline evidence as current route or provider state.
    PresentsImportedOfflineAsCurrentRouteOrProviderState,
    /// A consumer reopened a live target without validating identity, trust, route, and authority.
    ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
    /// Lineage metadata leaked a live secret or stale authority.
    LeaksLiveSecretOrStaleAuthorityThroughLineage,
    /// An export dropped the non-live vocabulary the product UI uses.
    DropsNonLiveVocabularyInExport,
    /// A lineage descriptor was not joined back to its source snapshot descriptor.
    LineageUnjoinedToSourceDescriptor,
    /// An accessibility route for the non-live boundary, provenance, or lineage join was dropped.
    AccessibilityRouteDropped,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream historical-reference contract narrowed.
    UpstreamHistoricalReferenceNarrowed,
}

impl ImportedOfflineLineageDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::GrammarDriftDetected,
        Self::MutationBlockedPostureDropped,
        Self::RankedOrNarratedAsCurrentLiveServiceTruth,
        Self::PresentsImportedOfflineAsCurrentRouteOrProviderState,
        Self::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
        Self::LeaksLiveSecretOrStaleAuthorityThroughLineage,
        Self::DropsNonLiveVocabularyInExport,
        Self::LineageUnjoinedToSourceDescriptor,
        Self::AccessibilityRouteDropped,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamHistoricalReferenceNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::GrammarDriftDetected => "grammar_drift_detected",
            Self::MutationBlockedPostureDropped => "mutation_blocked_posture_dropped",
            Self::RankedOrNarratedAsCurrentLiveServiceTruth => {
                "ranked_or_narrated_as_current_live_service_truth"
            }
            Self::PresentsImportedOfflineAsCurrentRouteOrProviderState => {
                "presents_imported_offline_as_current_route_or_provider_state"
            }
            Self::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority => {
                "reopens_live_target_without_validating_identity_trust_route_and_authority"
            }
            Self::LeaksLiveSecretOrStaleAuthorityThroughLineage => {
                "leaks_live_secret_or_stale_authority_through_lineage"
            }
            Self::DropsNonLiveVocabularyInExport => "drops_non_live_vocabulary_in_export",
            Self::LineageUnjoinedToSourceDescriptor => "lineage_unjoined_to_source_descriptor",
            Self::AccessibilityRouteDropped => "accessibility_route_dropped",
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamHistoricalReferenceNarrowed => "upstream_historical_reference_narrowed",
        }
    }
}

/// The controlled non-live grammar an imported / offline evidence profile presents.
///
/// These six words describe the non-live (historical) side of the propagated evidence and must be identical
/// across every downstream consumer that shows the same profile. The historical-role word must be a frozen
/// [`M5HistoricalReferenceRole`] token; the rest are controlled words the lineage carries, including the shared
/// "Showing imported or offline evidence" label that keeps the vocabulary identical to the primary archive viewer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NonLiveEvidenceGrammar {
    /// Historical-role word (must be a frozen [`M5HistoricalReferenceRole`] token).
    pub historical_role_word: String,
    /// The captured-evidence / archived-snapshot label word.
    pub snapshot_label_word: String,
    /// The capture-time word the lineage is attributed to.
    pub capture_time_word: String,
    /// The provenance / capture-context word the lineage is attributed to.
    pub provenance_word: String,
    /// The mutation-blocked-posture word (read-only, non-authoritative-for-mutation).
    pub mutation_blocked_posture_word: String,
    /// The shared "Showing imported or offline evidence" label word.
    pub imported_offline_label_word: String,
}

impl NonLiveEvidenceGrammar {
    /// Whether every grammar word is present.
    pub fn all_present(&self) -> bool {
        !self.historical_role_word.trim().is_empty()
            && !self.snapshot_label_word.trim().is_empty()
            && !self.capture_time_word.trim().is_empty()
            && !self.provenance_word.trim().is_empty()
            && !self.mutation_blocked_posture_word.trim().is_empty()
            && !self.imported_offline_label_word.trim().is_empty()
    }

    /// Whether the historical-role word is a member of the frozen role vocabulary.
    pub fn historical_role_word_in_vocabulary(&self) -> bool {
        is_known_historical_reference_role_token(self.historical_role_word.trim())
    }

    /// Whether the capture-time and provenance words that keep the evidence from dead-linking are both present.
    pub fn capture_context_present(&self) -> bool {
        !self.capture_time_word.trim().is_empty() && !self.provenance_word.trim().is_empty()
    }

    /// Whether the imported / offline label matches the canonical shared non-live vocabulary word.
    pub fn imported_offline_label_is_canonical(&self) -> bool {
        self.imported_offline_label_word.trim() == M5_IMPORTED_OFFLINE_LABEL
    }

    /// Whether the profile honours the mutation-blocked rule: a historical-side role that must be present before
    /// the object may be surfaced as non-live evidence must pair it with a real mutation-blocked posture word and
    /// never collapse to an editable / live / writable / current-object sentinel.
    pub fn mutation_blocked_posture_satisfied(&self) -> bool {
        match historical_reference_role_from_token(self.historical_role_word.trim()) {
            Some(role) if role.must_be_present_before_surfacing_as_non_live_evidence() => {
                let posture = self.mutation_blocked_posture_word.trim().to_lowercase();
                !posture.is_empty()
                    && !MUTATION_BLOCKED_POSTURE_ABSENT_SENTINELS.contains(&posture.as_str())
            }
            _ => true,
        }
    }
}

/// The join that keeps a propagated lineage attributable to its capture context: the source capture-context ref,
/// the producer / build ref, and the provenance-lineage ref the evidence was captured under.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageJoin {
    /// Stable id / ref of the source capture context.
    pub source_capture_context_ref: String,
    /// Stable id / ref of the producer / build that captured the evidence.
    pub producer_build_ref: String,
    /// Stable id / ref of the provenance lineage chain.
    pub provenance_lineage_ref: String,
}

impl LineageJoin {
    /// Whether every join ref is present, so the lineage is fully attributable.
    pub fn all_present(&self) -> bool {
        !self.source_capture_context_ref.trim().is_empty()
            && !self.producer_build_ref.trim().is_empty()
            && !self.provenance_lineage_ref.trim().is_empty()
    }
}

/// The explicit lineage descriptor every propagated imported / offline evidence binding carries.
///
/// It joins the packet back to its source snapshot / archived descriptor and, when the lineage is joinable, its
/// live-target handoff packet — otherwise a metadata-only exit — so downstream consumers preserve provenance and
/// a safe exit. It names those joins by controlled id rather than embedding a live route, secret, or authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageDescriptor {
    /// The source snapshot / archived descriptor this lineage joins back to (never omitted).
    pub source_snapshot_descriptor_ref: String,
    /// The controlled capture-context join.
    pub lineage_join: LineageJoin,
    /// The live-target handoff packet ref, present only when the lineage joins a live target.
    pub live_target_handoff_ref: Option<String>,
    /// The metadata-only exit ref, present when no live counterpart remains.
    pub metadata_only_exit_ref: Option<String>,
    /// The explicit non-live boundary note ("Showing imported or offline evidence, not current live truth").
    pub non_live_boundary_note: String,
    /// The next action offered.
    pub next_action: LineageNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a lineage binding must carry, derived from its disposition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineageRenderDisclosure {
    /// The parity state the disposition requires.
    pub parity_state: LineageParity,
    /// Whether the binding offers an open-current-live-object action.
    pub offers_open_live_target: bool,
    /// Whether the lineage descriptor must carry a live-target handoff ref.
    pub requires_live_target_handoff_ref: bool,
    /// Whether the lineage descriptor must carry a metadata-only exit ref.
    pub requires_metadata_only_exit_ref: bool,
    /// The next action the lineage descriptor must offer.
    pub next_action: LineageNextAction,
    /// Whether the propagated evidence's content is available in this disposition.
    pub expects_content_available: bool,
}

/// Resolves the render disclosures a lineage binding must carry from its disposition.
///
/// A live-target-joinable lineage renders the base action set plus an open-current-live-object action, carries a
/// live-target handoff ref, and keeps its content available. A non-live-boundary disposition narrows the actions,
/// carries a metadata-only exit ref, and — when its content is unavailable — still renders capture time,
/// provenance, and its non-live boundary instead of a dead link. All keep the non-live grammar and join back to a
/// source snapshot descriptor.
pub const fn resolve_lineage_disposition_disclosure(
    disposition: EvidenceLineageDisposition,
) -> LineageRenderDisclosure {
    match disposition {
        EvidenceLineageDisposition::LiveTargetJoinable => LineageRenderDisclosure {
            parity_state: LineageParity::LiveTargetLineageJoined,
            offers_open_live_target: true,
            requires_live_target_handoff_ref: true,
            requires_metadata_only_exit_ref: false,
            next_action: LineageNextAction::OpenCurrentLiveObjectThroughValidatedHandoff,
            expects_content_available: true,
        },
        EvidenceLineageDisposition::ImportedOfflineOnly => LineageRenderDisclosure {
            parity_state: LineageParity::NonLiveBoundaryDisclosed,
            offers_open_live_target: false,
            requires_live_target_handoff_ref: false,
            requires_metadata_only_exit_ref: true,
            next_action: LineageNextAction::InspectLineageMetadataOnly,
            expects_content_available: true,
        },
        EvidenceLineageDisposition::MetadataOnlyExit => LineageRenderDisclosure {
            parity_state: LineageParity::NonLiveBoundaryDisclosed,
            offers_open_live_target: false,
            requires_live_target_handoff_ref: false,
            requires_metadata_only_exit_ref: true,
            next_action: LineageNextAction::InspectLineageMetadataOnly,
            expects_content_available: false,
        },
        EvidenceLineageDisposition::ExportedRedactedLineage => LineageRenderDisclosure {
            parity_state: LineageParity::NonLiveBoundaryDisclosed,
            offers_open_live_target: false,
            requires_live_target_handoff_ref: false,
            requires_metadata_only_exit_ref: true,
            next_action: LineageNextAction::InspectLineageMetadataOnly,
            expects_content_available: true,
        },
    }
}

/// One lineage binding: a preserved-object class propagated in one disposition to one downstream consumer surface
/// for one imported / offline evidence profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedOfflineLineageBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable imported / offline evidence profile id (shared across surfaces that show the same profile).
    pub lineage_profile_id: String,
    /// Human-readable imported / offline evidence profile identity.
    pub lineage_profile_label: String,
    /// Which preserved-object class this binding carries.
    pub object_class: M5HistoricalReferenceObject,
    /// Which downstream consumer surface renders it.
    pub consumer: M5HistoricalReferenceConsumerSurface,
    /// The disposition of this propagated lineage.
    pub disposition: EvidenceLineageDisposition,
    /// A stable, human-facing disposition label.
    pub disposition_label: String,
    /// The controlled non-live grammar presented (identical across surfaces for one profile).
    pub non_live_grammar: NonLiveEvidenceGrammar,
    /// Whether the propagated evidence's content is available.
    pub content_available: bool,
    /// Whether a live-target lineage is joined or a non-live boundary is disclosed.
    pub parity_state: LineageParity,
    /// The discoverable action set allowed on this downstream lineage consumer.
    pub allowed_actions: Vec<LineageConsumerAction>,
    /// The accessibility routes through which the non-live boundary, provenance, and lineage join can be
    /// discovered without pointer-only chrome.
    pub accessibility_routes: Vec<M5HistoricalReferenceAccessibilityRoute>,
    /// The explicit lineage descriptor; required and complete on every propagated binding.
    pub lineage_descriptor: LineageDescriptor,
    /// The historical side stays mutation blocked. MUST be `true`.
    pub non_live_boundary_explicitly_called_out: bool,
    /// Guardrail: this consumer ranked, narrated, or summarized the packet as current live service truth. MUST be
    /// `false`.
    pub ranked_or_narrated_as_current_live_service_truth: bool,
    /// Guardrail: this consumer presents imported / offline evidence as current route or provider state. MUST be
    /// `false`.
    pub presents_imported_offline_as_current_route_or_provider_state: bool,
    /// Guardrail: this consumer reopens a live target without validating identity, trust, route, and authority.
    /// MUST be `false`.
    pub reopens_live_target_without_validating_identity_trust_route_and_authority: bool,
    /// Guardrail: this consumer leaks a live secret or stale authority through lineage metadata. MUST be `false`.
    pub leaks_live_secret_or_stale_authority_through_lineage: bool,
    /// Guardrail: this consumer drops the non-live vocabulary in its export. MUST be `false`.
    pub drops_non_live_vocabulary_in_export: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl ImportedOfflineLineageBinding {
    /// Disclosures this binding must carry, derived from its disposition.
    pub const fn disclosure(&self) -> LineageRenderDisclosure {
        resolve_lineage_disposition_disclosure(self.disposition)
    }

    /// Whether this binding joins back to a live target.
    pub const fn is_live_target_joinable(&self) -> bool {
        self.disposition.is_live_target_joinable()
    }

    /// Whether every guardrail row-invariant holds (non-live boundary called out, all guardrails false).
    pub const fn guardrails_hold(&self) -> bool {
        self.non_live_boundary_explicitly_called_out
            && !self.ranked_or_narrated_as_current_live_service_truth
            && !self.presents_imported_offline_as_current_route_or_provider_state
            && !self.reopens_live_target_without_validating_identity_trust_route_and_authority
            && !self.leaks_live_secret_or_stale_authority_through_lineage
            && !self.drops_non_live_vocabulary_in_export
    }

    /// Whether the analysis-only base action set is present.
    pub fn has_base_actions(&self) -> bool {
        LineageConsumerAction::BASE
            .iter()
            .all(|action| self.allowed_actions.contains(action))
    }

    /// Whether no rank / narrate / apply / sync affordance leaked in (structurally guaranteed by the closed action
    /// enum, but checked so the invariant is explicit).
    pub fn action_set_is_closed(&self) -> bool {
        self.allowed_actions.iter().all(|action| {
            matches!(
                action,
                LineageConsumerAction::InspectLineage
                    | LineageConsumerAction::ExportLineage
                    | LineageConsumerAction::OpenCurrentLiveObject
            )
        })
    }

    /// Whether the open-current-live-object action is present exactly when the disposition offers it.
    pub fn open_live_action_matches_disposition(&self) -> bool {
        let offered = self.disclosure().offers_open_live_target;
        let present = self
            .allowed_actions
            .contains(&LineageConsumerAction::OpenCurrentLiveObject);
        offered == present
    }

    /// Whether the content-available flag matches what the disposition expects.
    pub fn content_presence_matches_disposition(&self) -> bool {
        self.content_available == self.disclosure().expects_content_available
    }

    /// Whether, when the content is unavailable, the binding still renders capture time, provenance, and a
    /// non-live boundary note instead of degrading to a dead link.
    pub fn renders_metadata_instead_of_dead_link(&self) -> bool {
        if self.content_available {
            return true;
        }
        self.non_live_grammar.capture_context_present()
            && !self
                .lineage_descriptor
                .non_live_boundary_note
                .trim()
                .is_empty()
    }

    /// Whether keyboard focus and screen-reader announcement are both discoverable.
    pub fn accessibility_state_discoverable(&self) -> bool {
        self.accessibility_routes
            .contains(&M5HistoricalReferenceAccessibilityRoute::KeyboardFocusable)
            && self
                .accessibility_routes
                .contains(&M5HistoricalReferenceAccessibilityRoute::ScreenReaderAnnounced)
    }

    /// Whether this binding points at the canonical per-domain schema and the matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let domain_ref = self.object_class.canonical_domain_schema_ref();
        self.source_contract_refs
            .iter()
            .any(|reference| reference == domain_ref)
            && self
                .source_contract_refs
                .iter()
                .any(|reference| reference == M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedOfflineLineageTrustReview {
    /// Object-class reuse is proven by fixtures rather than inferred from screenshots.
    pub object_class_reuse_proven_by_fixtures: bool,
    /// The same profile presents the same non-live grammar across surfaces.
    pub same_profile_same_non_live_grammar_across_surfaces: bool,
    /// Every historical-role word is a frozen role token.
    pub historical_role_words_stay_in_frozen_vocabulary: bool,
    /// The shared imported / offline label matches the canonical non-live vocabulary word.
    pub imported_offline_label_matches_primary_archive_viewer: bool,
    /// A non-live side's mutation-blocked posture never masquerades as a live, writable, or current object.
    pub mutation_blocked_posture_never_masquerades_as_live: bool,
    /// Every propagated binding joins its lineage back to a source snapshot descriptor.
    pub every_binding_joins_lineage_back_to_source_descriptor: bool,
    /// A companion / export surface and a support / AI consumer show imported / offline evidence with the same
    /// vocabulary and lineage fields as the primary archive viewer.
    pub companion_and_support_consumers_share_non_live_vocabulary: bool,
    /// Metadata, provenance, and non-live boundary render instead of a generic dead link when content is gone.
    pub metadata_provenance_and_boundary_render_instead_of_dead_link: bool,
    /// A historical packet is never ranked, narrated, or summarized as current live service truth.
    pub historical_packet_never_narrated_as_current_live_truth: bool,
    /// Imported / offline evidence is never presented as current route or provider state.
    pub imported_offline_never_presented_as_current_route_or_provider: bool,
    /// Lineage metadata never leaks a live secret or stale authority.
    pub lineage_metadata_never_leaks_secret_or_stale_authority: bool,
    /// An open-current-live-object action is offered only through a validated live-target handoff.
    pub open_live_offered_only_through_validated_handoff: bool,
    /// Stable disposition labels are used across surfaces.
    pub stable_disposition_labels_used_across_surfaces: bool,
    /// Accessibility routes for the non-live boundary, provenance, and lineage join are present.
    pub accessibility_routes_present_for_boundary_provenance_and_join: bool,
    /// Disposition disclosure spans joinable, imported-offline-only, metadata-only, and exported-redacted.
    pub disposition_disclosed_across_all_lineage_dispositions: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the object class.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl ImportedOfflineLineageTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.object_class_reuse_proven_by_fixtures
            && self.same_profile_same_non_live_grammar_across_surfaces
            && self.historical_role_words_stay_in_frozen_vocabulary
            && self.imported_offline_label_matches_primary_archive_viewer
            && self.mutation_blocked_posture_never_masquerades_as_live
            && self.every_binding_joins_lineage_back_to_source_descriptor
            && self.companion_and_support_consumers_share_non_live_vocabulary
            && self.metadata_provenance_and_boundary_render_instead_of_dead_link
            && self.historical_packet_never_narrated_as_current_live_truth
            && self.imported_offline_never_presented_as_current_route_or_provider
            && self.lineage_metadata_never_leaks_secret_or_stale_authority
            && self.open_live_offered_only_through_validated_handoff
            && self.stable_disposition_labels_used_across_surfaces
            && self.accessibility_routes_present_for_boundary_provenance_and_join
            && self.disposition_disclosed_across_all_lineage_dispositions
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedOfflineLineageProjection {
    /// The shell / archive-viewer surface consumes the shared lineage packet.
    pub shell_consumes_lineage: bool,
    /// The help / docs surface consumes the shared lineage packet.
    pub help_docs_consumes_lineage: bool,
    /// The support / AI evidence consumer consumes the shared lineage packet.
    pub support_consumes_lineage: bool,
    /// The review / incident surface consumes the shared lineage packet.
    pub review_incident_consumes_lineage: bool,
    /// The runbook-archive surface consumes the shared lineage packet.
    pub runbook_archive_consumes_lineage: bool,
    /// The release-center retirement snapshot page consumes the shared lineage packet.
    pub release_center_consumes_lineage: bool,
    /// The companion / export card consumes the shared lineage packet.
    pub companion_export_consumes_lineage: bool,
    /// The program-governance review consumes the shared lineage packet.
    pub program_governance_consumes_lineage: bool,
    /// The CLI / export path consumes the shared lineage packet.
    pub cli_export_consumes_lineage: bool,
    /// Every object class is stated by two or more consumers.
    pub every_object_class_stated_by_two_or_more_consumers: bool,
    /// Non-live grammar is identical for the same profile.
    pub non_live_grammar_identical_for_same_profile: bool,
    /// The non-live boundary is disclosed rather than hidden.
    pub non_live_boundary_disclosed_not_hidden: bool,
    /// Export maps a lineage row back to one historical-reference object class.
    pub lineage_maps_back_to_one_historical_reference_object: bool,
}

impl ImportedOfflineLineageProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.shell_consumes_lineage
            && self.help_docs_consumes_lineage
            && self.support_consumes_lineage
            && self.review_incident_consumes_lineage
            && self.runbook_archive_consumes_lineage
            && self.release_center_consumes_lineage
            && self.companion_export_consumes_lineage
            && self.program_governance_consumes_lineage
            && self.cli_export_consumes_lineage
            && self.every_object_class_stated_by_two_or_more_consumers
            && self.non_live_grammar_identical_for_same_profile
            && self.non_live_boundary_disclosed_not_hidden
            && self.lineage_maps_back_to_one_historical_reference_object
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedOfflineLineageProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5ImportedOfflineLineagePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ImportedOfflineLineagePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Lineage bindings.
    pub lineage_bindings: Vec<ImportedOfflineLineageBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ImportedOfflineLineageDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5HistoricalReferenceConsumerSurface>,
    /// Trust review block.
    pub trust_review: ImportedOfflineLineageTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ImportedOfflineLineageProjection,
    /// Proof freshness block.
    pub proof_freshness: ImportedOfflineLineageProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe imported / offline lineage packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ImportedOfflineLineagePacket {
    /// Record kind; must equal [`M5_IMPORTED_OFFLINE_LINEAGE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_IMPORTED_OFFLINE_LINEAGE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Lineage bindings.
    pub lineage_bindings: Vec<ImportedOfflineLineageBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ImportedOfflineLineageDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5HistoricalReferenceConsumerSurface>,
    /// Trust review block.
    pub trust_review: ImportedOfflineLineageTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ImportedOfflineLineageProjection,
    /// Proof freshness block.
    pub proof_freshness: ImportedOfflineLineageProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ImportedOfflineLineagePacket {
    /// Builds an imported / offline lineage packet from stable-lane input.
    pub fn new(input: M5ImportedOfflineLineagePacketInput) -> Self {
        Self {
            record_kind: M5_IMPORTED_OFFLINE_LINEAGE_RECORD_KIND.to_owned(),
            schema_version: M5_IMPORTED_OFFLINE_LINEAGE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            lineage_bindings: input.lineage_bindings,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the imported / offline lineage invariants.
    pub fn validate(&self) -> Vec<M5ImportedOfflineLineageViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_IMPORTED_OFFLINE_LINEAGE_RECORD_KIND {
            violations.push(M5ImportedOfflineLineageViolation::WrongRecordKind);
        }
        if self.schema_version != M5_IMPORTED_OFFLINE_LINEAGE_SCHEMA_VERSION {
            violations.push(M5ImportedOfflineLineageViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ImportedOfflineLineageViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5ImportedOfflineLineageViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5ImportedOfflineLineageViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5ImportedOfflineLineageViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(M5ImportedOfflineLineageViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5ImportedOfflineLineageViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("lineage packet serializes"),
        ) {
            violations.push(M5ImportedOfflineLineageViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("lineage packet serializes")
    }

    /// Deterministic matrix CSV, one row per lineage binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "object_class,consumer,disposition,content_available,next_action,parity_state,disposition_label\n",
        );
        for binding in &self.lineage_bindings {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                binding.object_class.as_str(),
                binding.consumer.as_str(),
                binding.disposition.as_str(),
                binding.content_available,
                binding.lineage_descriptor.next_action.as_str(),
                binding.parity_state.as_str(),
                binding.disposition_label.replace(',', ";"),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let joined = self
            .lineage_bindings
            .iter()
            .filter(|binding| binding.is_live_target_joinable())
            .count();

        let mut out = String::new();
        out.push_str("# Imported / Offline Evidence Lineage: One Vocabulary Across Consumers\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Lineage bindings: {} ({} live-target joinable)\n",
            self.lineage_bindings.len(),
            joined
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Lineage bindings\n\n");
        for binding in &self.lineage_bindings {
            out.push_str(&format!(
                "- **{}** [`{}`]: object `{}` on `{}`, disposition `{}`, content-available `{}`, role `{}`\n",
                binding.lineage_profile_label,
                binding.binding_id,
                binding.object_class.as_str(),
                binding.consumer.as_str(),
                binding.disposition.as_str(),
                binding.content_available,
                binding.non_live_grammar.historical_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in imported / offline lineage export.
#[derive(Debug)]
pub enum M5ImportedOfflineLineageArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ImportedOfflineLineageViolation>),
}

impl fmt::Display for M5ImportedOfflineLineageArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "lineage export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "lineage export failed validation: {tokens}")
            }
        }
    }
}

impl Error for M5ImportedOfflineLineageArtifactError {}

/// Validation failures emitted by [`M5ImportedOfflineLineagePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ImportedOfflineLineageViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No lineage bindings are present.
    LineageBindingsMissing,
    /// A lineage binding is incomplete.
    BindingIncomplete,
    /// A binding's non-live grammar values are incomplete.
    GrammarFacetIncomplete,
    /// A binding's historical-role word is not a frozen role token.
    HistoricalRoleWordOutsideVocabulary,
    /// A binding's imported / offline label is not the canonical shared vocabulary word.
    ImportedOfflineLabelNotCanonical,
    /// A binding's gate-role dropped its mutation-blocked posture.
    MutationBlockedPostureMissingForGateRole,
    /// A binding's parity state does not match its disposition.
    ParityStateMismatch,
    /// A binding's content-available flag does not match its disposition.
    ContentPresenceMismatch,
    /// Two surfaces show the same profile with different non-live grammar.
    GrammarDriftAcrossSurfaces,
    /// A shared object class is not stated by at least two distinct consumers.
    ObjectClassReuseUnproven,
    /// A support / export binding does not point at the canonical contracts.
    SupportExportReferenceMissing,
    /// A binding is missing a stable disposition label.
    DispositionLabelMissing,
    /// A binding is missing its source snapshot descriptor join.
    SourceDescriptorJoinMissing,
    /// A binding's lineage join is incomplete.
    LineageJoinIncomplete,
    /// A binding's live-target handoff ref presence does not match its disposition.
    LiveTargetHandoffRefMismatch,
    /// A binding's metadata-only exit ref presence does not match its disposition.
    MetadataOnlyExitRefMismatch,
    /// A binding's lineage next action does not match the required next action.
    LineageNextActionMismatch,
    /// A binding is missing its non-live boundary note.
    NonLiveBoundaryNoteMissing,
    /// A binding is missing its next-action copy.
    LineageNextActionLabelMissing,
    /// A binding is missing the analysis-only base action set.
    BaseActionsMissing,
    /// A binding's action set is not the closed lineage action set.
    ActionSetNotClosed,
    /// A binding's open-current-live-object action does not match its disposition.
    OpenLiveActionDispositionMismatch,
    /// A binding whose content is gone degrades to a generic dead link.
    MetadataFallbackMissing,
    /// A binding cannot discover its non-live boundary via keyboard focus and screen-reader announcement.
    AccessibilityStateUndiscoverable,
    /// A binding's non-live side is not mutation blocked / boundary not called out.
    NonLiveBoundaryNotCalledOut,
    /// A binding was ranked, narrated, or summarized as current live service truth.
    RankedOrNarratedAsCurrentLiveServiceTruth,
    /// A binding presents imported / offline evidence as current route or provider state.
    PresentsImportedOfflineAsCurrentRouteOrProviderState,
    /// A binding reopens a live target without validating identity, trust, route, and authority.
    ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
    /// A binding leaks a live secret or stale authority through lineage metadata.
    LeaksLiveSecretOrStaleAuthorityThroughLineage,
    /// A binding drops the non-live vocabulary in its export.
    DropsNonLiveVocabularyInExport,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared object class appears among the bindings.
    ObjectClassCoverageMissing,
    /// Not every disposition appears among the bindings.
    DispositionCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ImportedOfflineLineageViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::LineageBindingsMissing => "lineage_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::GrammarFacetIncomplete => "grammar_facet_incomplete",
            Self::HistoricalRoleWordOutsideVocabulary => "historical_role_word_outside_vocabulary",
            Self::ImportedOfflineLabelNotCanonical => "imported_offline_label_not_canonical",
            Self::MutationBlockedPostureMissingForGateRole => {
                "mutation_blocked_posture_missing_for_gate_role"
            }
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::ContentPresenceMismatch => "content_presence_mismatch",
            Self::GrammarDriftAcrossSurfaces => "grammar_drift_across_surfaces",
            Self::ObjectClassReuseUnproven => "object_class_reuse_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::DispositionLabelMissing => "disposition_label_missing",
            Self::SourceDescriptorJoinMissing => "source_descriptor_join_missing",
            Self::LineageJoinIncomplete => "lineage_join_incomplete",
            Self::LiveTargetHandoffRefMismatch => "live_target_handoff_ref_mismatch",
            Self::MetadataOnlyExitRefMismatch => "metadata_only_exit_ref_mismatch",
            Self::LineageNextActionMismatch => "lineage_next_action_mismatch",
            Self::NonLiveBoundaryNoteMissing => "non_live_boundary_note_missing",
            Self::LineageNextActionLabelMissing => "lineage_next_action_label_missing",
            Self::BaseActionsMissing => "base_actions_missing",
            Self::ActionSetNotClosed => "action_set_not_closed",
            Self::OpenLiveActionDispositionMismatch => "open_live_action_disposition_mismatch",
            Self::MetadataFallbackMissing => "metadata_fallback_missing",
            Self::AccessibilityStateUndiscoverable => "accessibility_state_undiscoverable",
            Self::NonLiveBoundaryNotCalledOut => "non_live_boundary_not_called_out",
            Self::RankedOrNarratedAsCurrentLiveServiceTruth => {
                "ranked_or_narrated_as_current_live_service_truth"
            }
            Self::PresentsImportedOfflineAsCurrentRouteOrProviderState => {
                "presents_imported_offline_as_current_route_or_provider_state"
            }
            Self::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority => {
                "reopens_live_target_without_validating_identity_trust_route_and_authority"
            }
            Self::LeaksLiveSecretOrStaleAuthorityThroughLineage => {
                "leaks_live_secret_or_stale_authority_through_lineage"
            }
            Self::DropsNonLiveVocabularyInExport => "drops_non_live_vocabulary_in_export",
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::ObjectClassCoverageMissing => "object_class_coverage_missing",
            Self::DispositionCoverageMissing => "disposition_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable imported / offline lineage export.
pub fn current_stable_m5_imported_offline_lineage_export(
) -> Result<M5ImportedOfflineLineagePacket, M5ImportedOfflineLineageArtifactError> {
    let packet: M5ImportedOfflineLineagePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/support/m5-imported-offline-lineage/support_export.json"
    )))
    .map_err(M5ImportedOfflineLineageArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ImportedOfflineLineageArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ImportedOfflineLineagePacket,
    violations: &mut Vec<M5ImportedOfflineLineageViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_IMPORTED_OFFLINE_LINEAGE_SCHEMA_REF,
        M5_IMPORTED_OFFLINE_LINEAGE_DOC_REF,
        M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF,
        M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF,
    ];
    // The five object classes map to three canonical domain schemas; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object_class in M5HistoricalReferenceObject::ALL {
        domains.insert(object_class.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5ImportedOfflineLineageViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5ImportedOfflineLineagePacket,
    violations: &mut Vec<M5ImportedOfflineLineageViolation>,
) {
    if packet.lineage_bindings.is_empty() {
        violations.push(M5ImportedOfflineLineageViolation::LineageBindingsMissing);
        return;
    }

    // One vocabulary: the non-live grammar must be identical for every binding that renders the same
    // imported / offline evidence profile.
    let mut profile_grammar: BTreeMap<&str, &NonLiveEvidenceGrammar> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each object class must be stated by at least two distinct consumers.
    let mut object_consumers: BTreeMap<
        M5HistoricalReferenceObject,
        BTreeSet<M5HistoricalReferenceConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5HistoricalReferenceConsumerSurface> = BTreeSet::new();
    let mut seen_objects: BTreeSet<M5HistoricalReferenceObject> = BTreeSet::new();
    let mut seen_dispositions: BTreeSet<EvidenceLineageDisposition> = BTreeSet::new();

    for binding in &packet.lineage_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.lineage_profile_id.trim().is_empty()
            || binding.lineage_profile_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5ImportedOfflineLineageViolation::BindingIncomplete);
        }
        if binding.disposition_label.trim().is_empty() {
            violations.push(M5ImportedOfflineLineageViolation::DispositionLabelMissing);
        }
        if !binding.non_live_grammar.all_present() {
            violations.push(M5ImportedOfflineLineageViolation::GrammarFacetIncomplete);
        }
        if !binding
            .non_live_grammar
            .historical_role_word_in_vocabulary()
        {
            violations.push(M5ImportedOfflineLineageViolation::HistoricalRoleWordOutsideVocabulary);
        }
        if !binding
            .non_live_grammar
            .imported_offline_label_is_canonical()
        {
            violations.push(M5ImportedOfflineLineageViolation::ImportedOfflineLabelNotCanonical);
        }
        if !binding
            .non_live_grammar
            .mutation_blocked_posture_satisfied()
        {
            violations
                .push(M5ImportedOfflineLineageViolation::MutationBlockedPostureMissingForGateRole);
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5ImportedOfflineLineageViolation::ParityStateMismatch);
        }
        if !binding.content_presence_matches_disposition() {
            violations.push(M5ImportedOfflineLineageViolation::ContentPresenceMismatch);
        }

        // Lineage descriptor: always present, always joined back to a source snapshot descriptor.
        let descriptor = &binding.lineage_descriptor;
        if descriptor.source_snapshot_descriptor_ref.trim().is_empty() {
            violations.push(M5ImportedOfflineLineageViolation::SourceDescriptorJoinMissing);
        }
        if !descriptor.lineage_join.all_present() {
            violations.push(M5ImportedOfflineLineageViolation::LineageJoinIncomplete);
        }
        if descriptor.live_target_handoff_ref.is_some()
            != disclosure.requires_live_target_handoff_ref
        {
            violations.push(M5ImportedOfflineLineageViolation::LiveTargetHandoffRefMismatch);
        }
        if descriptor.metadata_only_exit_ref.is_some() != disclosure.requires_metadata_only_exit_ref
        {
            violations.push(M5ImportedOfflineLineageViolation::MetadataOnlyExitRefMismatch);
        }
        if descriptor.next_action != disclosure.next_action {
            violations.push(M5ImportedOfflineLineageViolation::LineageNextActionMismatch);
        }
        if descriptor.non_live_boundary_note.trim().is_empty() {
            violations.push(M5ImportedOfflineLineageViolation::NonLiveBoundaryNoteMissing);
        }
        if descriptor.next_action_label.trim().is_empty() {
            violations.push(M5ImportedOfflineLineageViolation::LineageNextActionLabelMissing);
        }

        // Action rules.
        if !binding.has_base_actions() {
            violations.push(M5ImportedOfflineLineageViolation::BaseActionsMissing);
        }
        if !binding.action_set_is_closed() {
            violations.push(M5ImportedOfflineLineageViolation::ActionSetNotClosed);
        }
        if !binding.open_live_action_matches_disposition() {
            violations.push(M5ImportedOfflineLineageViolation::OpenLiveActionDispositionMismatch);
        }

        // AC2 / dead-link: never degrade to a generic dead link when metadata / provenance / boundary can be shown.
        if !binding.renders_metadata_instead_of_dead_link() {
            violations.push(M5ImportedOfflineLineageViolation::MetadataFallbackMissing);
        }

        // Accessibility discovery.
        if !binding.accessibility_state_discoverable() {
            violations.push(M5ImportedOfflineLineageViolation::AccessibilityStateUndiscoverable);
        }

        // Guardrail row-invariants.
        if !binding.non_live_boundary_explicitly_called_out {
            violations.push(M5ImportedOfflineLineageViolation::NonLiveBoundaryNotCalledOut);
        }
        if binding.ranked_or_narrated_as_current_live_service_truth {
            violations
                .push(M5ImportedOfflineLineageViolation::RankedOrNarratedAsCurrentLiveServiceTruth);
        }
        if binding.presents_imported_offline_as_current_route_or_provider_state {
            violations.push(
                M5ImportedOfflineLineageViolation::PresentsImportedOfflineAsCurrentRouteOrProviderState,
            );
        }
        if binding.reopens_live_target_without_validating_identity_trust_route_and_authority {
            violations.push(
                M5ImportedOfflineLineageViolation::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
            );
        }
        if binding.leaks_live_secret_or_stale_authority_through_lineage {
            violations.push(
                M5ImportedOfflineLineageViolation::LeaksLiveSecretOrStaleAuthorityThroughLineage,
            );
        }
        if binding.drops_non_live_vocabulary_in_export {
            violations.push(M5ImportedOfflineLineageViolation::DropsNonLiveVocabularyInExport);
        }

        // Support / export consumers must map an object class back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations.push(M5ImportedOfflineLineageViolation::SupportExportReferenceMissing);
        }

        // Grammar-drift accumulation.
        match profile_grammar.get(binding.lineage_profile_id.as_str()) {
            None => {
                profile_grammar.insert(
                    binding.lineage_profile_id.as_str(),
                    &binding.non_live_grammar,
                );
            }
            Some(existing) => {
                if **existing != binding.non_live_grammar && !drift_reported {
                    violations.push(M5ImportedOfflineLineageViolation::GrammarDriftAcrossSurfaces);
                    drift_reported = true;
                }
            }
        }

        object_consumers
            .entry(binding.object_class)
            .or_default()
            .insert(binding.consumer);
        seen_consumers.insert(binding.consumer);
        seen_objects.insert(binding.object_class);
        seen_dispositions.insert(binding.disposition);
    }

    // Coverage: every consumer surface, object class, and disposition must appear.
    for consumer in M5HistoricalReferenceConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5ImportedOfflineLineageViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for object_class in M5HistoricalReferenceObject::ALL {
        if !seen_objects.contains(&object_class) {
            violations.push(M5ImportedOfflineLineageViolation::ObjectClassCoverageMissing);
            break;
        }
    }
    for disposition in EvidenceLineageDisposition::ALL {
        if !seen_dispositions.contains(&disposition) {
            violations.push(M5ImportedOfflineLineageViolation::DispositionCoverageMissing);
            break;
        }
    }

    // Reuse: every present object class must be stated by two or more distinct consumers.
    for consumers in object_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5ImportedOfflineLineageViolation::ObjectClassReuseUnproven);
            break;
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
