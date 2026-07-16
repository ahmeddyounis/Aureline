//! Shared archived-snapshot viewers and analysis-only banners that render preserved M5 historical
//! evidence across the shell / archive-viewer, help / docs, support, review / incident, runbook-archive,
//! release-center, companion / export, program-governance, and CLI / export surfaces at **one canonical
//! non-live vocabulary and action-rule set**.
//!
//! This module is the B149 archive-consumer lane over the five non-live-evidence object classes frozen in
//! [`crate::m5_historical_reference_matrix`] and made machine-readable by the historical-snapshot-descriptor
//! implement lane ([`crate::m5_historical_snapshot_descriptor_and_change_diff_registries`]). Where those
//! lanes describe *what* is preserved, this lane proves *how it is shown*: every archive-bearing surface
//! frames a preserved snapshot with the same archive/state banner and fact grid — snapshot label, capture
//! time, provenance, analysis-only posture, and the exact action set allowed on archived evidence — before a
//! user can mistake it for the current editable object.
//!
//! It binds each preserved-evidence profile to the concrete consumer surfaces that render it and proves — by
//! fixtures, not screenshots — that the same profile presents the same banner-role, snapshot-label,
//! capture-time, provenance, analysis-only-posture, and allowed-action-set grammar wherever it appears, and
//! that the discoverable open-current-live-object action only appears where the live target still exists.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **One vocabulary / no drift.** For a given preserved-evidence profile every consumer surface — a
//!    support bundle viewer, a retirement snapshot page, and a review / incident evidence reopen flow among
//!    them — must present identical [`ArchiveViewerBannerGrammar`]: the same banner-role word, the same
//!    snapshot-label word, the same capture-time word, the same provenance word, the same
//!    analysis-only-posture word, and the same allowed-action-set word. The banner-role word must be a token
//!    from the frozen [`M5HistoricalReferenceRole`] vocabulary, so no surface rewrites `snapshot_labeling`,
//!    `capture_time_attribution`, `provenance_attribution`, or `mutation_blocked_posture` in its own words.
//! 2. **Analysis-only, never write-capable-as-live.** An archived view exposes inspect, compare, and
//!    export-evidence actions, and an open-current-live-object action *only* where the live target still
//!    exists (the [`ArchiveViewPosture::LiveTargetOpenable`] posture); ordinary mutation affordances are
//!    disabled by construction (no write action can even be represented). No binding may present a
//!    write-capable control as if the current object were open live, reopen a live target without validating
//!    identity / trust / route / authority, dead-link an expired or removed artifact instead of showing
//!    metadata, leave non-live evidence unjoined to its capture context, or let archived / imported evidence
//!    look live, writable, or current by omission.
//! 3. **Screen-reader and keyboard discoverable.** Every binding names the accessibility routes
//!    ([`M5HistoricalReferenceAccessibilityRoute`]) through which the archived / non-live state, its
//!    provenance, and the open-live-target action can be discovered without pointer-only chrome; keyboard
//!    focus and screen-reader announcement are mandatory.
//!
//! Narrowing is disclosed, never hidden: a metadata-only exit, an imported / offline-only view, or an
//! exported, export-safe view carries an explicit [`ArchiveNarrowNote`] naming the reason, the preserved
//! grammar, and the next action, so a surface may narrow *which* actions remain without ever rewording the
//! underlying banner grammar or quietly implying the object is still live.
//!
//! The packet references upstream historical-reference contracts by id rather than embedding their content.
//! Raw secret values, credentials, and private endpoints stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/program/m5-archived-snapshot-viewer-consumers.schema.json`](../../../../schemas/program/m5-archived-snapshot-viewer-consumers.schema.json).
//! The contract doc is
//! [`docs/support/m5_archived_snapshot_viewer_consumers.md`](../../../../docs/support/m5_archived_snapshot_viewer_consumers.md).
//! The protected fixture directory is
//! [`fixtures/recovery/m5-archived-snapshot-viewer-consumers/`](../../../../fixtures/recovery/m5-archived-snapshot-viewer-consumers/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_archived_snapshot_viewer_consumers,
    seeded_m5_archived_snapshot_viewer_consumers_imported_offline_narrowed,
    seeded_m5_archived_snapshot_viewer_consumers_metadata_only_narrowed,
};

use crate::m5_historical_reference_matrix::{
    M5HistoricalReferenceAccessibilityRoute, M5HistoricalReferenceConsumerSurface,
    M5HistoricalReferenceObject, M5HistoricalReferenceRole, M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF,
    M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ArchivedSnapshotViewerConsumersPacket`].
pub const M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_RECORD_KIND: &str =
    "m5_archived_snapshot_viewer_analysis_only_banner_consumer_registry";

/// Schema version for archived-snapshot-viewer consumer records.
pub const M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_PACKET_ID: &str =
    "m5-archived-snapshot-viewer-consumers:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_SCHEMA_REF: &str =
    "schemas/program/m5-archived-snapshot-viewer-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_DOC_REF: &str =
    "docs/support/m5_archived_snapshot_viewer_consumers.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/support/m5-archived-snapshot-viewer-consumers/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_CSV_REF: &str =
    "artifacts/support/m5-archived-snapshot-viewer-consumers/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_REPORT_REF: &str =
    "artifacts/support/m5-archived-snapshot-viewer-consumers/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/recovery/m5-archived-snapshot-viewer-consumers";

/// Proof-freshness SLO in hours for this lane.
pub const M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_PROOF_SLO_HOURS: u32 = 720;

/// Analysis-only-posture sentinel words a preserved-evidence banner may never fall back to; a banner whose
/// role must be present before surfacing as non-live evidence must always keep a real analysis-only posture
/// rather than implying the object is editable, live, writable, or the current object.
const ANALYSIS_ONLY_POSTURE_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "editable",
    "live_object",
    "writable",
    "current_object",
];

/// Whether a consumer surface is an export / support path that must map an object class back to its
/// canonical contract by id.
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
///
/// This is the "one vocabulary" gate: a banner's role word must be a controlled role token rather than a
/// per-surface synonym.
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

/// The action posture an archived-snapshot viewer renders for one binding.
///
/// The posture governs the discoverable action set and narrowing disclosure, never the banner grammar: a
/// narrowed posture still carries the same banner-role, snapshot-label, capture-time, provenance,
/// analysis-only-posture, and allowed-action-set words, and discloses the narrowing through an explicit note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveViewPosture {
    /// The live target still exists; the viewer offers a validated open-current-live-object action.
    LiveTargetOpenable,
    /// The live target is gone or removed; the viewer offers a metadata-only inspection exit instead of a
    /// dead link, with no open-live-object action.
    MetadataOnlyExit,
    /// Imported / offline evidence only; the viewer warns that the data is not current live route, service,
    /// or workspace truth, with no open-live-object action.
    ImportedOfflineOnly,
    /// An exported, export-safe-redacted archive view.
    ExportedRedacted,
}

impl ArchiveViewPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LiveTargetOpenable,
        Self::MetadataOnlyExit,
        Self::ImportedOfflineOnly,
        Self::ExportedRedacted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTargetOpenable => "live_target_openable",
            Self::MetadataOnlyExit => "metadata_only_exit",
            Self::ImportedOfflineOnly => "imported_offline_only",
            Self::ExportedRedacted => "exported_redacted",
        }
    }

    /// Whether this posture narrows below the full live-target-openable disclosure.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::LiveTargetOpenable)
    }
}

/// A discoverable action a preserved-evidence viewer may expose.
///
/// The set is deliberately closed and analysis-only: there is no editable / write action variant, so an
/// archived view can never present a write-capable control as if the current object were open live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveAction {
    /// Inspect the preserved snapshot's fields metadata-only.
    Inspect,
    /// Compare the preserved snapshot against another preserved snapshot.
    Compare,
    /// Export the preserved evidence packet.
    ExportEvidence,
    /// Open the current live object — only when the live target still exists and is validated.
    OpenCurrentLiveObject,
}

impl ArchiveAction {
    /// The analysis-only base action set present on every archived view.
    pub const ANALYSIS_ONLY_BASE: [Self; 3] = [Self::Inspect, Self::Compare, Self::ExportEvidence];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inspect => "inspect",
            Self::Compare => "compare",
            Self::ExportEvidence => "export_evidence",
            Self::OpenCurrentLiveObject => "open_current_live_object",
        }
    }
}

/// Why an archived-snapshot viewer narrowed its action set below a live-target-openable view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveNarrowReason {
    /// The live target was removed; only a metadata-only exit remains.
    LiveTargetRemovedMetadataOnly,
    /// The evidence is imported / offline only and never pointed at a live target.
    ImportedOfflineDisclosed,
    /// An exported view redacted its surrounding detail export-safe.
    ExportRedactionNarrowed,
}

impl ArchiveNarrowReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTargetRemovedMetadataOnly => "live_target_removed_metadata_only",
            Self::ImportedOfflineDisclosed => "imported_offline_disclosed",
            Self::ExportRedactionNarrowed => "export_redaction_narrowed",
        }
    }
}

/// The next action a narrow note offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveNarrowNextAction {
    /// Open the metadata-only inspection exit for a removed live target.
    OpenMetadataOnlyExit,
    /// Open the import / offline source backing the evidence.
    OpenImportSource,
    /// Open the full detail behind the redacted export.
    OpenFullDetail,
}

impl ArchiveNarrowNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenMetadataOnlyExit => "open_metadata_only_exit",
            Self::OpenImportSource => "open_import_source",
            Self::OpenFullDetail => "open_full_detail",
        }
    }
}

/// Whether a binding preserves the full live-target-openable view or discloses a narrowed posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveViewerParityState {
    /// The banner grammar and full action set are preserved and shown.
    FacetsPreserved,
    /// The banner grammar is preserved and a narrowed action set is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl ArchiveViewerParityState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetsPreserved => "facets_preserved",
            Self::FacetsDisclosedNarrowed => "facets_disclosed_narrowed",
        }
    }
}

/// Downgrade trigger that can narrow this consumer lane below its claimed parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchivedSnapshotViewerConsumersDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Banner grammar drifted between surfaces for the same profile.
    ArchiveBannerGrammarDriftDetected,
    /// A banner dropped its analysis-only posture and began to imply the object is live or writable.
    AnalysisOnlyPostureDropped,
    /// A surface presented a write-capable control as if the current object were open live.
    PresentsWriteCapableControlAsIfCurrentObjectOpenLive,
    /// A surface reopened a live target without validating identity, trust, route, and authority.
    ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
    /// A surface dead-linked an expired or removed artifact instead of showing metadata.
    DeadLinksExpiredOrRemovedArtifactInsteadOfShowingMetadata,
    /// A surface left non-live evidence unjoined to its capture context.
    LeavesNonLiveEvidenceUnjoinedToCaptureContext,
    /// A surface let archived or imported evidence look live, writable, or current by omission.
    LetsArchivedOrImportedEvidenceLookLiveWritableOrCurrentByOmission,
    /// An accessibility route for the non-live state, provenance, or open-live-target action was dropped.
    AccessibilityRouteDropped,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream historical-reference contract narrowed.
    UpstreamHistoricalReferenceNarrowed,
}

impl ArchivedSnapshotViewerConsumersDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ArchiveBannerGrammarDriftDetected,
        Self::AnalysisOnlyPostureDropped,
        Self::PresentsWriteCapableControlAsIfCurrentObjectOpenLive,
        Self::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
        Self::DeadLinksExpiredOrRemovedArtifactInsteadOfShowingMetadata,
        Self::LeavesNonLiveEvidenceUnjoinedToCaptureContext,
        Self::LetsArchivedOrImportedEvidenceLookLiveWritableOrCurrentByOmission,
        Self::AccessibilityRouteDropped,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamHistoricalReferenceNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ArchiveBannerGrammarDriftDetected => "archive_banner_grammar_drift_detected",
            Self::AnalysisOnlyPostureDropped => "analysis_only_posture_dropped",
            Self::PresentsWriteCapableControlAsIfCurrentObjectOpenLive => {
                "presents_write_capable_control_as_if_current_object_open_live"
            }
            Self::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority => {
                "reopens_live_target_without_validating_identity_trust_route_and_authority"
            }
            Self::DeadLinksExpiredOrRemovedArtifactInsteadOfShowingMetadata => {
                "dead_links_expired_or_removed_artifact_instead_of_showing_metadata"
            }
            Self::LeavesNonLiveEvidenceUnjoinedToCaptureContext => {
                "leaves_non_live_evidence_unjoined_to_capture_context"
            }
            Self::LetsArchivedOrImportedEvidenceLookLiveWritableOrCurrentByOmission => {
                "lets_archived_or_imported_evidence_look_live_writable_or_current_by_omission"
            }
            Self::AccessibilityRouteDropped => "accessibility_route_dropped",
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamHistoricalReferenceNarrowed => "upstream_historical_reference_narrowed",
        }
    }
}

/// The controlled banner grammar a preserved-evidence profile presents.
///
/// These six words must be identical across every consumer surface that shows the same profile. The
/// banner-role word must be a frozen role token; the rest are controlled words the profile's banner carries.
/// A surface may narrow which actions remain, but it may never reword any of these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveViewerBannerGrammar {
    /// Banner-role word (must be a frozen [`M5HistoricalReferenceRole`] token).
    pub banner_role_word: String,
    /// The captured-evidence / archived-snapshot label word.
    pub snapshot_label_word: String,
    /// The capture-time word the evidence is attributed to.
    pub capture_time_word: String,
    /// The provenance / capture-context word the evidence is attributed to.
    pub provenance_word: String,
    /// The analysis-only-posture word (read-only, non-authoritative-for-mutation).
    pub analysis_only_posture_word: String,
    /// The allowed-action-set word describing the archived view's action rules.
    pub allowed_action_set_word: String,
}

impl ArchiveViewerBannerGrammar {
    /// Whether every grammar word is present.
    pub fn all_present(&self) -> bool {
        !self.banner_role_word.trim().is_empty()
            && !self.snapshot_label_word.trim().is_empty()
            && !self.capture_time_word.trim().is_empty()
            && !self.provenance_word.trim().is_empty()
            && !self.analysis_only_posture_word.trim().is_empty()
            && !self.allowed_action_set_word.trim().is_empty()
    }

    /// Whether the banner-role word is a member of the frozen role vocabulary.
    pub fn banner_role_word_in_vocabulary(&self) -> bool {
        is_known_historical_reference_role_token(self.banner_role_word.trim())
    }

    /// Whether the profile honours the analysis-only rule: a banner whose role must be present before the
    /// object may be surfaced as non-live evidence must pair it with a real analysis-only posture word and
    /// never collapse to an editable / live / writable / current-object sentinel.
    pub fn analysis_only_posture_satisfied(&self) -> bool {
        match historical_reference_role_from_token(self.banner_role_word.trim()) {
            Some(role) if role.must_be_present_before_surfacing_as_non_live_evidence() => {
                let posture = self.analysis_only_posture_word.trim().to_lowercase();
                !posture.is_empty()
                    && !ANALYSIS_ONLY_POSTURE_ABSENT_SENTINELS.contains(&posture.as_str())
            }
            _ => true,
        }
    }
}

/// The explicit note a narrowed posture shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveNarrowNote {
    /// Why the posture narrowed.
    pub reason: ArchiveNarrowReason,
    /// Note naming the preserved grammar (never omitted).
    pub preserved_grammar_note: String,
    /// The next action offered.
    pub next_action: ArchiveNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchiveViewRenderDisclosure {
    /// The parity state the posture requires.
    pub parity_state: ArchiveViewerParityState,
    /// The narrow reason the posture requires, if any.
    pub narrow_reason: Option<ArchiveNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<ArchiveNarrowNextAction>,
    /// Whether the binding must carry an explicit narrow note.
    pub needs_narrow_note: bool,
    /// Whether the binding must carry an explicit import / offline note.
    pub needs_import_offline_note: bool,
    /// Whether the binding must carry an explicit export-safe-detail note.
    pub needs_export_detail_note: bool,
    /// Whether the binding offers a validated open-current-live-object action.
    pub offers_open_live_target: bool,
}

/// Resolves the render disclosures a consumer binding must carry from its posture.
///
/// The live-target-openable posture renders the full analysis-only action set plus a validated
/// open-current-live-object action. A metadata-only exit, an imported / offline-only view, and an exported
/// view each narrow the action set and disclose the narrowing through an explicit note — but all three keep
/// every banner grammar word.
pub const fn resolve_archive_view_render_disclosure(
    posture: ArchiveViewPosture,
) -> ArchiveViewRenderDisclosure {
    match posture {
        ArchiveViewPosture::LiveTargetOpenable => ArchiveViewRenderDisclosure {
            parity_state: ArchiveViewerParityState::FacetsPreserved,
            narrow_reason: None,
            narrow_next_action: None,
            needs_narrow_note: false,
            needs_import_offline_note: false,
            needs_export_detail_note: false,
            offers_open_live_target: true,
        },
        ArchiveViewPosture::MetadataOnlyExit => ArchiveViewRenderDisclosure {
            parity_state: ArchiveViewerParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(ArchiveNarrowReason::LiveTargetRemovedMetadataOnly),
            narrow_next_action: Some(ArchiveNarrowNextAction::OpenMetadataOnlyExit),
            needs_narrow_note: true,
            needs_import_offline_note: false,
            needs_export_detail_note: false,
            offers_open_live_target: false,
        },
        ArchiveViewPosture::ImportedOfflineOnly => ArchiveViewRenderDisclosure {
            parity_state: ArchiveViewerParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(ArchiveNarrowReason::ImportedOfflineDisclosed),
            narrow_next_action: Some(ArchiveNarrowNextAction::OpenImportSource),
            needs_narrow_note: true,
            needs_import_offline_note: true,
            needs_export_detail_note: false,
            offers_open_live_target: false,
        },
        ArchiveViewPosture::ExportedRedacted => ArchiveViewRenderDisclosure {
            parity_state: ArchiveViewerParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(ArchiveNarrowReason::ExportRedactionNarrowed),
            narrow_next_action: Some(ArchiveNarrowNextAction::OpenFullDetail),
            needs_narrow_note: true,
            needs_import_offline_note: false,
            needs_export_detail_note: true,
            offers_open_live_target: false,
        },
    }
}

/// One consumer binding: a preserved-evidence object class rendered on one consumer surface in one posture
/// for one preserved-evidence profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveViewerConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable preserved-evidence-profile id (shared across surfaces that show the same profile).
    pub evidence_profile_id: String,
    /// Human-readable preserved-evidence-profile identity.
    pub evidence_profile_label: String,
    /// Which preserved-evidence object class this binding renders.
    pub object_class: M5HistoricalReferenceObject,
    /// Which consumer surface renders it.
    pub consumer: M5HistoricalReferenceConsumerSurface,
    /// Which action posture this surface renders.
    pub posture: ArchiveViewPosture,
    /// The controlled banner grammar presented (identical across surfaces for one profile).
    pub banner_grammar: ArchiveViewerBannerGrammar,
    /// Whether grammar is preserved in full or a narrowing is disclosed.
    pub parity_state: ArchiveViewerParityState,
    /// The discoverable action set allowed on this archived view.
    pub allowed_actions: Vec<ArchiveAction>,
    /// The accessibility routes through which the non-live state, provenance, and open-live-target action
    /// can be discovered without pointer-only chrome.
    pub accessibility_routes: Vec<M5HistoricalReferenceAccessibilityRoute>,
    /// The explicit narrow note; required and complete when the posture narrows.
    pub narrow_note: Option<ArchiveNarrowNote>,
    /// Import / offline note; required and non-empty when the disclosure demands it.
    pub import_offline_note: String,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Guardrail: this surface presents a write-capable control as if the current object were open live.
    /// MUST be `false`.
    pub presents_write_capable_control_as_if_current_object_open_live: bool,
    /// Guardrail: this surface reopens a live target without validating identity, trust, route, and
    /// authority. MUST be `false`.
    pub reopens_live_target_without_validating_identity_trust_route_and_authority: bool,
    /// Guardrail: this surface dead-links an expired or removed artifact instead of showing metadata. MUST
    /// be `false`.
    pub dead_links_expired_or_removed_artifact_instead_of_showing_metadata: bool,
    /// Guardrail: this surface leaves non-live evidence unjoined to its capture context. MUST be `false`.
    pub leaves_non_live_evidence_unjoined_to_capture_context: bool,
    /// Guardrail: this surface lets archived or imported evidence look live, writable, or current by
    /// omission. MUST be `false`.
    pub lets_archived_or_imported_evidence_look_live_writable_or_current_by_omission: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl ArchiveViewerConsumerBinding {
    /// Disclosures this binding must carry, derived from its posture.
    pub const fn disclosure(&self) -> ArchiveViewRenderDisclosure {
        resolve_archive_view_render_disclosure(self.posture)
    }

    /// Whether this binding renders below the full live-target-openable view.
    pub const fn is_narrowed(&self) -> bool {
        self.posture.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.presents_write_capable_control_as_if_current_object_open_live
            && !self.reopens_live_target_without_validating_identity_trust_route_and_authority
            && !self.dead_links_expired_or_removed_artifact_instead_of_showing_metadata
            && !self.leaves_non_live_evidence_unjoined_to_capture_context
            && !self.lets_archived_or_imported_evidence_look_live_writable_or_current_by_omission
    }

    /// Whether the analysis-only base action set is present.
    pub fn has_analysis_only_base_actions(&self) -> bool {
        ArchiveAction::ANALYSIS_ONLY_BASE
            .iter()
            .all(|action| self.allowed_actions.contains(action))
    }

    /// Whether the open-current-live-object action is present exactly when the posture offers it.
    pub fn open_live_action_matches_posture(&self) -> bool {
        let offered = self.disclosure().offers_open_live_target;
        let present = self
            .allowed_actions
            .contains(&ArchiveAction::OpenCurrentLiveObject);
        offered == present
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
pub struct ArchivedSnapshotViewerConsumersTrustReview {
    /// Object-class reuse is proven by fixtures rather than inferred from screenshots.
    pub object_class_reuse_proven_by_fixtures: bool,
    /// The same profile presents the same banner grammar across surfaces.
    pub same_profile_same_banner_across_surfaces: bool,
    /// Every banner-role word is a frozen role token.
    pub banner_role_words_stay_in_frozen_vocabulary: bool,
    /// A banner's analysis-only posture never masquerades as a live, writable, or current object.
    pub analysis_only_posture_never_masquerades_as_live: bool,
    /// A write-capable control is never shown as if the current object were open live.
    pub write_controls_never_shown_as_current_object_open_live: bool,
    /// An open-live-target action always validates identity, trust, route, and authority first.
    pub open_live_target_always_validates_identity_trust_route_authority: bool,
    /// Expired or removed artifacts show metadata instead of a dead link.
    pub expired_or_removed_artifacts_show_metadata_not_dead_links: bool,
    /// Non-live evidence is always joined to its capture context.
    pub non_live_evidence_always_joined_to_capture_context: bool,
    /// Archived or imported evidence never looks live, writable, or current by omission.
    pub archived_evidence_never_looks_live_by_omission: bool,
    /// Accessibility routes for the non-live state, provenance, and open-live-target action are present.
    pub accessibility_routes_present_for_state_provenance_and_open_live_target: bool,
    /// Narrowing is disclosed across live-target, metadata-only, imported / offline, and exported postures.
    pub narrowing_disclosed_across_postures: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the object class.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl ArchivedSnapshotViewerConsumersTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.object_class_reuse_proven_by_fixtures
            && self.same_profile_same_banner_across_surfaces
            && self.banner_role_words_stay_in_frozen_vocabulary
            && self.analysis_only_posture_never_masquerades_as_live
            && self.write_controls_never_shown_as_current_object_open_live
            && self.open_live_target_always_validates_identity_trust_route_authority
            && self.expired_or_removed_artifacts_show_metadata_not_dead_links
            && self.non_live_evidence_always_joined_to_capture_context
            && self.archived_evidence_never_looks_live_by_omission
            && self.accessibility_routes_present_for_state_provenance_and_open_live_target
            && self.narrowing_disclosed_across_postures
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchivedSnapshotViewerConsumersProjection {
    /// The shell / archive-viewer surface consumes the shared archive banner.
    pub shell_consumes_archive_banner: bool,
    /// The help / docs surface consumes the shared archive banner.
    pub help_docs_consumes_archive_banner: bool,
    /// The support bundle viewer consumes the shared archive banner.
    pub support_consumes_archive_banner: bool,
    /// The review / incident surface consumes the shared archive banner.
    pub review_incident_consumes_archive_banner: bool,
    /// The runbook-archive surface consumes the shared archive banner.
    pub runbook_archive_consumes_archive_banner: bool,
    /// The release-center retirement snapshot page consumes the shared archive banner.
    pub release_center_consumes_archive_banner: bool,
    /// The companion / export path consumes the shared archive banner.
    pub companion_export_consumes_archive_banner: bool,
    /// The program-governance review consumes the shared archive banner.
    pub program_governance_consumes_archive_banner: bool,
    /// The CLI / export path consumes the shared archive banner.
    pub cli_export_consumes_archive_banner: bool,
    /// Every object class is adopted by two or more consumers.
    pub every_object_class_adopted_by_two_or_more_consumers: bool,
    /// Banner grammar is identical for the same profile.
    pub banner_grammar_identical_for_same_profile: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps an object class back to one historical-reference object class.
    pub export_maps_back_to_one_historical_reference_object: bool,
}

impl ArchivedSnapshotViewerConsumersProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.shell_consumes_archive_banner
            && self.help_docs_consumes_archive_banner
            && self.support_consumes_archive_banner
            && self.review_incident_consumes_archive_banner
            && self.runbook_archive_consumes_archive_banner
            && self.release_center_consumes_archive_banner
            && self.companion_export_consumes_archive_banner
            && self.program_governance_consumes_archive_banner
            && self.cli_export_consumes_archive_banner
            && self.every_object_class_adopted_by_two_or_more_consumers
            && self.banner_grammar_identical_for_same_profile
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_historical_reference_object
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchivedSnapshotViewerConsumersProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5ArchivedSnapshotViewerConsumersPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ArchivedSnapshotViewerConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<ArchiveViewerConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ArchivedSnapshotViewerConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5HistoricalReferenceConsumerSurface>,
    /// Trust review block.
    pub trust_review: ArchivedSnapshotViewerConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ArchivedSnapshotViewerConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: ArchivedSnapshotViewerConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe archived-snapshot-viewer consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ArchivedSnapshotViewerConsumersPacket {
    /// Record kind; must equal [`M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<ArchiveViewerConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ArchivedSnapshotViewerConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5HistoricalReferenceConsumerSurface>,
    /// Trust review block.
    pub trust_review: ArchivedSnapshotViewerConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ArchivedSnapshotViewerConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: ArchivedSnapshotViewerConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ArchivedSnapshotViewerConsumersPacket {
    /// Builds an archived-snapshot-viewer consumer packet from stable-lane input.
    pub fn new(input: M5ArchivedSnapshotViewerConsumersPacketInput) -> Self {
        Self {
            record_kind: M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            consumer_bindings: input.consumer_bindings,
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

    /// Validates the archived-snapshot-viewer consumer invariants.
    pub fn validate(&self) -> Vec<M5ArchivedSnapshotViewerConsumersViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_RECORD_KIND {
            violations.push(M5ArchivedSnapshotViewerConsumersViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_SCHEMA_VERSION {
            violations.push(M5ArchivedSnapshotViewerConsumersViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ArchivedSnapshotViewerConsumersViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5ArchivedSnapshotViewerConsumersViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5ArchivedSnapshotViewerConsumersViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5ArchivedSnapshotViewerConsumersViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(M5ArchivedSnapshotViewerConsumersViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5ArchivedSnapshotViewerConsumersViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("archived-snapshot-viewer consumer packet serializes"),
        ) {
            violations
                .push(M5ArchivedSnapshotViewerConsumersViolation::RawBoundaryMaterialInExport);
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
            .expect("archived-snapshot-viewer consumer packet serializes")
    }

    /// Deterministic matrix CSV, one row per consumer binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from("object_class,consumer,posture,banner_role_word,parity_state\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                binding.object_class.as_str(),
                binding.consumer.as_str(),
                binding.posture.as_str(),
                binding.banner_grammar.banner_role_word,
                binding.parity_state.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .consumer_bindings
            .iter()
            .filter(|binding| binding.is_narrowed())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Archived-Snapshot Viewers & Analysis-Only Banners: One Vocabulary Across Surfaces\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Consumer bindings: {} ({} narrowed)\n",
            self.consumer_bindings.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Consumer bindings\n\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "- **{}** [`{}`]: object `{}` on `{}`, posture `{}`, role `{}`\n",
                binding.evidence_profile_label,
                binding.binding_id,
                binding.object_class.as_str(),
                binding.consumer.as_str(),
                binding.posture.as_str(),
                binding.banner_grammar.banner_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in archived-snapshot-viewer consumer export.
#[derive(Debug)]
pub enum M5ArchivedSnapshotViewerConsumersArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ArchivedSnapshotViewerConsumersViolation>),
}

impl fmt::Display for M5ArchivedSnapshotViewerConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "archived-snapshot-viewer consumer export parse failed: {error}"
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
                    "archived-snapshot-viewer consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ArchivedSnapshotViewerConsumersArtifactError {}

/// Validation failures emitted by [`M5ArchivedSnapshotViewerConsumersPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ArchivedSnapshotViewerConsumersViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No consumer bindings are present.
    ConsumerBindingsMissing,
    /// A consumer binding is incomplete.
    BindingIncomplete,
    /// A binding's banner grammar values are incomplete.
    GrammarFacetIncomplete,
    /// A binding's banner-role word is not a frozen role token.
    BannerRoleWordOutsideVocabulary,
    /// A binding's gate-role dropped its analysis-only posture.
    AnalysisOnlyPostureMissingForGateRole,
    /// A binding's parity state does not match its posture.
    ParityStateMismatch,
    /// Two surfaces show the same profile with different banner grammar.
    ArchiveBannerGrammarDriftAcrossSurfaces,
    /// A shared object class is not adopted by at least two distinct consumers.
    ObjectClassReuseUnproven,
    /// A support / export binding does not point at the canonical contracts.
    SupportExportReferenceMissing,
    /// A narrowed binding is missing its explicit narrow note.
    NarrowNoteMissing,
    /// A narrow note's reason does not match the required narrow reason.
    NarrowReasonMismatch,
    /// A narrow note's next action does not match the required next action.
    NarrowNextActionMismatch,
    /// A narrow note is missing its preserved-grammar note.
    NarrowNotePreservedGrammarMissing,
    /// A narrow note is missing its next-action copy.
    NarrowNextActionLabelMissing,
    /// A live-target-openable binding carries a narrow note it must not.
    UnexpectedNarrowNote,
    /// A binding that needs an explicit import / offline note is missing it.
    ImportOfflineNoteMissing,
    /// A binding that needs an explicit export-detail note is missing it.
    ExportDetailNoteMissing,
    /// A binding is missing the analysis-only base action set.
    AnalysisOnlyBaseActionsMissing,
    /// A binding's open-current-live-object action does not match its posture.
    OpenLiveActionPostureMismatch,
    /// A binding cannot discover its non-live state via keyboard focus and screen-reader announcement.
    AccessibilityStateUndiscoverable,
    /// A binding presents a write-capable control as if the current object were open live.
    PresentsWriteCapableControlAsIfCurrentObjectOpenLive,
    /// A binding reopens a live target without validating identity, trust, route, and authority.
    ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
    /// A binding dead-links an expired or removed artifact instead of showing metadata.
    DeadLinksExpiredOrRemovedArtifactInsteadOfShowingMetadata,
    /// A binding leaves non-live evidence unjoined to its capture context.
    LeavesNonLiveEvidenceUnjoinedToCaptureContext,
    /// A binding lets archived or imported evidence look live, writable, or current by omission.
    LetsArchivedOrImportedEvidenceLookLiveWritableOrCurrentByOmission,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared object class appears among the bindings.
    ObjectClassCoverageMissing,
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

impl M5ArchivedSnapshotViewerConsumersViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ConsumerBindingsMissing => "consumer_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::GrammarFacetIncomplete => "grammar_facet_incomplete",
            Self::BannerRoleWordOutsideVocabulary => "banner_role_word_outside_vocabulary",
            Self::AnalysisOnlyPostureMissingForGateRole => {
                "analysis_only_posture_missing_for_gate_role"
            }
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::ArchiveBannerGrammarDriftAcrossSurfaces => {
                "archive_banner_grammar_drift_across_surfaces"
            }
            Self::ObjectClassReuseUnproven => "object_class_reuse_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::NarrowNoteMissing => "narrow_note_missing",
            Self::NarrowReasonMismatch => "narrow_reason_mismatch",
            Self::NarrowNextActionMismatch => "narrow_next_action_mismatch",
            Self::NarrowNotePreservedGrammarMissing => "narrow_note_preserved_grammar_missing",
            Self::NarrowNextActionLabelMissing => "narrow_next_action_label_missing",
            Self::UnexpectedNarrowNote => "unexpected_narrow_note",
            Self::ImportOfflineNoteMissing => "import_offline_note_missing",
            Self::ExportDetailNoteMissing => "export_detail_note_missing",
            Self::AnalysisOnlyBaseActionsMissing => "analysis_only_base_actions_missing",
            Self::OpenLiveActionPostureMismatch => "open_live_action_posture_mismatch",
            Self::AccessibilityStateUndiscoverable => "accessibility_state_undiscoverable",
            Self::PresentsWriteCapableControlAsIfCurrentObjectOpenLive => {
                "presents_write_capable_control_as_if_current_object_open_live"
            }
            Self::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority => {
                "reopens_live_target_without_validating_identity_trust_route_and_authority"
            }
            Self::DeadLinksExpiredOrRemovedArtifactInsteadOfShowingMetadata => {
                "dead_links_expired_or_removed_artifact_instead_of_showing_metadata"
            }
            Self::LeavesNonLiveEvidenceUnjoinedToCaptureContext => {
                "leaves_non_live_evidence_unjoined_to_capture_context"
            }
            Self::LetsArchivedOrImportedEvidenceLookLiveWritableOrCurrentByOmission => {
                "lets_archived_or_imported_evidence_look_live_writable_or_current_by_omission"
            }
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::ObjectClassCoverageMissing => "object_class_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable archived-snapshot-viewer consumer export.
pub fn current_stable_m5_archived_snapshot_viewer_consumers_export(
) -> Result<M5ArchivedSnapshotViewerConsumersPacket, M5ArchivedSnapshotViewerConsumersArtifactError>
{
    let packet: M5ArchivedSnapshotViewerConsumersPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/support/m5-archived-snapshot-viewer-consumers/support_export.json"
        )))
        .map_err(M5ArchivedSnapshotViewerConsumersArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ArchivedSnapshotViewerConsumersArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ArchivedSnapshotViewerConsumersPacket,
    violations: &mut Vec<M5ArchivedSnapshotViewerConsumersViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_SCHEMA_REF,
        M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_DOC_REF,
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
            violations.push(M5ArchivedSnapshotViewerConsumersViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5ArchivedSnapshotViewerConsumersPacket,
    violations: &mut Vec<M5ArchivedSnapshotViewerConsumersViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(M5ArchivedSnapshotViewerConsumersViolation::ConsumerBindingsMissing);
        return;
    }

    // One vocabulary: the banner grammar must be identical for every binding that renders the same
    // preserved-evidence profile.
    let mut profile_grammar: BTreeMap<&str, &ArchiveViewerBannerGrammar> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each object class must be adopted by at least two distinct consumers.
    let mut object_consumers: BTreeMap<
        M5HistoricalReferenceObject,
        BTreeSet<M5HistoricalReferenceConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5HistoricalReferenceConsumerSurface> = BTreeSet::new();
    let mut seen_objects: BTreeSet<M5HistoricalReferenceObject> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.evidence_profile_id.trim().is_empty()
            || binding.evidence_profile_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5ArchivedSnapshotViewerConsumersViolation::BindingIncomplete);
        }
        if !binding.banner_grammar.all_present() {
            violations.push(M5ArchivedSnapshotViewerConsumersViolation::GrammarFacetIncomplete);
        }
        if !binding.banner_grammar.banner_role_word_in_vocabulary() {
            violations
                .push(M5ArchivedSnapshotViewerConsumersViolation::BannerRoleWordOutsideVocabulary);
        }
        if !binding.banner_grammar.analysis_only_posture_satisfied() {
            violations.push(
                M5ArchivedSnapshotViewerConsumersViolation::AnalysisOnlyPostureMissingForGateRole,
            );
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5ArchivedSnapshotViewerConsumersViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations.push(M5ArchivedSnapshotViewerConsumersViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations
                            .push(M5ArchivedSnapshotViewerConsumersViolation::NarrowReasonMismatch);
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations.push(
                            M5ArchivedSnapshotViewerConsumersViolation::NarrowNextActionMismatch,
                        );
                    }
                    if note.preserved_grammar_note.trim().is_empty() {
                        violations.push(
                            M5ArchivedSnapshotViewerConsumersViolation::NarrowNotePreservedGrammarMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5ArchivedSnapshotViewerConsumersViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5ArchivedSnapshotViewerConsumersViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_import_offline_note && binding.import_offline_note.trim().is_empty() {
            violations.push(M5ArchivedSnapshotViewerConsumersViolation::ImportOfflineNoteMissing);
        }
        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations.push(M5ArchivedSnapshotViewerConsumersViolation::ExportDetailNoteMissing);
        }

        // Action rules.
        if !binding.has_analysis_only_base_actions() {
            violations
                .push(M5ArchivedSnapshotViewerConsumersViolation::AnalysisOnlyBaseActionsMissing);
        }
        if !binding.open_live_action_matches_posture() {
            violations
                .push(M5ArchivedSnapshotViewerConsumersViolation::OpenLiveActionPostureMismatch);
        }

        // Accessibility discovery.
        if !binding.accessibility_state_discoverable() {
            violations
                .push(M5ArchivedSnapshotViewerConsumersViolation::AccessibilityStateUndiscoverable);
        }

        // Guardrail row-invariants (each must be false).
        if binding.presents_write_capable_control_as_if_current_object_open_live {
            violations.push(
                M5ArchivedSnapshotViewerConsumersViolation::PresentsWriteCapableControlAsIfCurrentObjectOpenLive,
            );
        }
        if binding.reopens_live_target_without_validating_identity_trust_route_and_authority {
            violations.push(
                M5ArchivedSnapshotViewerConsumersViolation::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
            );
        }
        if binding.dead_links_expired_or_removed_artifact_instead_of_showing_metadata {
            violations.push(
                M5ArchivedSnapshotViewerConsumersViolation::DeadLinksExpiredOrRemovedArtifactInsteadOfShowingMetadata,
            );
        }
        if binding.leaves_non_live_evidence_unjoined_to_capture_context {
            violations.push(
                M5ArchivedSnapshotViewerConsumersViolation::LeavesNonLiveEvidenceUnjoinedToCaptureContext,
            );
        }
        if binding.lets_archived_or_imported_evidence_look_live_writable_or_current_by_omission {
            violations.push(
                M5ArchivedSnapshotViewerConsumersViolation::LetsArchivedOrImportedEvidenceLookLiveWritableOrCurrentByOmission,
            );
        }

        // Support / export consumers must map an object class back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations
                .push(M5ArchivedSnapshotViewerConsumersViolation::SupportExportReferenceMissing);
        }

        // Grammar-drift accumulation.
        match profile_grammar.get(binding.evidence_profile_id.as_str()) {
            None => {
                profile_grammar.insert(
                    binding.evidence_profile_id.as_str(),
                    &binding.banner_grammar,
                );
            }
            Some(existing) => {
                if **existing != binding.banner_grammar && !drift_reported {
                    violations.push(
                        M5ArchivedSnapshotViewerConsumersViolation::ArchiveBannerGrammarDriftAcrossSurfaces,
                    );
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
    }

    // Coverage: every consumer surface and every object class must appear.
    for consumer in M5HistoricalReferenceConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5ArchivedSnapshotViewerConsumersViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for object_class in M5HistoricalReferenceObject::ALL {
        if !seen_objects.contains(&object_class) {
            violations.push(M5ArchivedSnapshotViewerConsumersViolation::ObjectClassCoverageMissing);
            break;
        }
    }

    // Reuse: every present object class must be adopted by two or more distinct consumers.
    for consumers in object_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5ArchivedSnapshotViewerConsumersViolation::ObjectClassReuseUnproven);
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
