//! Family qualification packet for voice-command and dictation surfaces.
//!
//! This module owns the release packet that prevents voice and dictation
//! surfaces from inheriting Stable from the command palette, AI surfaces, or
//! adjacent accessibility claims. Each row keeps mode truth, activation
//! posture, provider/privacy disclosure, transcript handling, command parity,
//! fallback state, accessibility evidence, and support/export projection as
//! explicit packet truth.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{OwnerSignoff, StableClaimLevel};

/// Supported schema version for the checked-in voice/dictation packet.
pub const VOICE_DICTATION_SURFACE_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const VOICE_DICTATION_SURFACE_QUALIFICATION_RECORD_KIND: &str =
    "voice_and_dictation_surface_qualification";

/// Repo-relative path to the checked-in packet.
pub const VOICE_DICTATION_SURFACE_QUALIFICATION_PATH: &str =
    "artifacts/release/m4/voice-and-dictation-surface-qualification.json";

/// Embedded checked-in packet JSON.
pub const VOICE_DICTATION_SURFACE_QUALIFICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m4/voice-and-dictation-surface-qualification.json"
));

/// Voice or dictation surface family covered by one row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceSurfaceKind {
    /// Voice-command overlay for invoking command graph actions.
    CommandOverlay,
    /// Dictation input lane for editor and prompt text insertion.
    DictationInput,
    /// Transcript correction strip shown before commit or command execution.
    TranscriptCorrection,
    /// Speech provider and privacy settings row.
    ProviderPrivacySettings,
    /// Unavailable, blocked, noisy, offline, or no-microphone fallback banner.
    UnavailableFallback,
    /// High-impact spoken action review sheet.
    HighImpactActionReview,
}

impl VoiceSurfaceKind {
    /// Every surface kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CommandOverlay,
        Self::DictationInput,
        Self::TranscriptCorrection,
        Self::ProviderPrivacySettings,
        Self::UnavailableFallback,
        Self::HighImpactActionReview,
    ];
}

/// Explicit speech mode rendered by the surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceMode {
    /// Speech resolves to command graph actions.
    Command,
    /// Speech inserts or edits text.
    Dictation,
    /// Surface can show either mode but must label the active mode.
    ModeSelectable,
    /// Surface is unavailable and cannot capture speech.
    Unavailable,
}

/// Current processing class disclosed before or during capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessingClass {
    /// Speech is processed locally.
    Local,
    /// Speech is routed to an approved enterprise provider.
    TrustedEnterprise,
    /// Speech is routed to a third-party provider.
    ThirdPartyProvider,
    /// Speech processing is unavailable.
    Unavailable,
}

/// Activation behavior for capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivationDefault {
    /// Hold-to-speak or push-to-talk is the default.
    PushToTalk,
    /// Tap-to-start capture with explicit stop.
    ExplicitTapToStart,
    /// Continuous listening is available only after opt-in.
    ContinuousOptIn,
    /// Wake word is available only after opt-in.
    WakeWordOptIn,
    /// Capture cannot start in this state.
    Unavailable,
}

impl ActivationDefault {
    fn allowed_for_stable_default(self) -> bool {
        matches!(
            self,
            Self::PushToTalk | Self::ExplicitTapToStart | Self::Unavailable
        )
    }
}

/// Transcript retention posture disclosed by the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptRetention {
    /// Transcript is ephemeral for the active correction flow.
    Ephemeral,
    /// Bounded correction buffer with visible delete/export controls.
    BoundedCorrectionBuffer,
    /// Metadata only; raw transcript is not retained.
    MetadataOnly,
    /// Retained only by reviewed enterprise policy.
    EnterprisePolicyRetained,
    /// No transcript is captured.
    NotCaptured,
}

impl TranscriptRetention {
    fn bounded_or_safer(self) -> bool {
        matches!(
            self,
            Self::Ephemeral
                | Self::BoundedCorrectionBuffer
                | Self::MetadataOnly
                | Self::NotCaptured
        )
    }
}

/// Unavailable or fallback condition surfaced by a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceFallbackState {
    /// System or browser microphone permission is unavailable.
    NoMicrophonePermission,
    /// Enterprise policy blocks capture or provider routing.
    PolicyBlocked,
    /// Network or offline state prevents provider-backed capture.
    Offline,
    /// Provider is unavailable.
    ProviderUnavailable,
    /// Audio conditions are too noisy for confident recognition.
    NoisyEnvironment,
    /// Keyboard and command-palette fallback is shown.
    KeyboardFallback,
}

/// UI primitives required for packet-backed voice surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VoiceUiPrimitives {
    /// Mic-state pill is visible.
    pub mic_state_pill: bool,
    /// Transcript strip or correction lane is visible where capture happens.
    pub transcript_strip: bool,
    /// Command/dictation disambiguation sheet is available.
    pub command_disambiguation_sheet: bool,
    /// Provider/privacy row is visible before or during capture.
    pub provider_privacy_row: bool,
    /// Voice-unavailable banner is available for blocked states.
    pub unavailable_banner: bool,
    /// Capture/export review sheet is available before raw transcript leaves default scope.
    pub capture_export_review_sheet: bool,
}

impl VoiceUiPrimitives {
    fn complete(&self) -> bool {
        self.mic_state_pill
            && self.transcript_strip
            && self.command_disambiguation_sheet
            && self.provider_privacy_row
            && self.unavailable_banner
            && self.capture_export_review_sheet
    }
}

/// Command graph parity proven for spoken commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommandParityContract {
    /// Spoken actions resolve to stable command ids.
    pub stable_command_ids: bool,
    /// Disabled states carry the same reason as keyboard and palette paths.
    pub disabled_with_reason: bool,
    /// Preview/apply/revert posture matches command graph policy.
    pub preview_apply_revert: bool,
    /// Approval requirements are not weaker than keyboard paths.
    pub approval_requirements: bool,
    /// Undo grouping is coherent for command and dictation actions.
    pub undo_grouping: bool,
    /// Audit and support lineage uses the same command provenance model.
    pub audit_support_lineage: bool,
    /// High-impact actions require transcript confirmation or review before run.
    pub high_impact_review: bool,
}

impl CommandParityContract {
    fn complete(&self) -> bool {
        self.stable_command_ids
            && self.disabled_with_reason
            && self.preview_apply_revert
            && self.approval_requirements
            && self.undo_grouping
            && self.audit_support_lineage
            && self.high_impact_review
    }
}

/// Transcript privacy controls proven by a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TranscriptPrivacyControls {
    /// Correction buffer has an explicit bound.
    pub bounded_correction_buffer: bool,
    /// Delete action is available.
    pub delete_action: bool,
    /// Export action is explicit and reviewable.
    pub export_action: bool,
    /// Redaction applies before support or diagnostics export.
    pub redaction_before_support_export: bool,
    /// Raw transcripts are excluded from support exports by default.
    pub raw_transcripts_excluded_by_default: bool,
}

impl TranscriptPrivacyControls {
    fn complete(&self) -> bool {
        self.bounded_correction_buffer
            && self.delete_action
            && self.export_action
            && self.redaction_before_support_export
            && self.raw_transcripts_excluded_by_default
    }
}

/// Publication and support destinations that must ingest the row label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VoiceProjection {
    /// Docs and Help pages ingest the qualification label.
    pub docs_help: bool,
    /// About and service-health surfaces ingest the qualification label.
    pub about_service_health: bool,
    /// CLI/headless inspection exposes the qualification label.
    pub cli_headless_inspect: bool,
    /// Support export carries mode, provider class, and evidence refs.
    pub support_export: bool,
    /// Privacy center and diagnostics surfaces ingest retention posture.
    pub privacy_diagnostics: bool,
}

impl VoiceProjection {
    fn complete(&self) -> bool {
        self.docs_help
            && self.about_service_health
            && self.cli_headless_inspect
            && self.support_export
            && self.privacy_diagnostics
    }
}

/// One voice/dictation qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VoiceSurfaceRow {
    /// Stable row id.
    pub surface_id: String,
    /// Human-readable title.
    pub title: String,
    /// Surface family.
    pub surface_kind: VoiceSurfaceKind,
    /// Whether the promoted build exposes this surface.
    pub promoted_build_surface: bool,
    /// Claimed lifecycle label before family qualification.
    pub claim_label: StableClaimLevel,
    /// Label rendered after qualification or narrowing.
    pub displayed_label: StableClaimLevel,
    /// Stable proof packet, absent for preview-only rows.
    #[serde(default)]
    pub qualification_packet: Option<ProofPacket>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Explicit mode rendered by the surface.
    pub mode: VoiceMode,
    /// Whether the mode label is visible or announced.
    pub explicit_mode_visible: bool,
    /// Processing class disclosed before or during capture.
    pub processing_class: ProcessingClass,
    /// Activation default.
    pub activation_default: ActivationDefault,
    /// Persistent capture/provider indicator is present for opt-in listening or remote routing.
    pub persistent_indicator: bool,
    /// Mute or stop action is visible.
    pub mute_or_stop_action: bool,
    /// Keyboard fallback is visible.
    pub keyboard_fallback: bool,
    /// Transcript retention posture.
    pub transcript_retention: TranscriptRetention,
    /// Packet-backed UI primitives.
    pub ui_primitives: VoiceUiPrimitives,
    /// Command graph parity proof.
    pub command_parity: CommandParityContract,
    /// Transcript privacy controls.
    pub transcript_privacy: TranscriptPrivacyControls,
    /// Fallback states this surface can render.
    #[serde(default)]
    pub fallback_states: Vec<VoiceFallbackState>,
    /// Publication/support projections.
    pub projection: VoiceProjection,
    /// Accessibility evidence refs.
    #[serde(default)]
    pub accessibility_refs: Vec<String>,
    /// Regression corpus refs.
    #[serde(default)]
    pub regression_refs: Vec<String>,
    /// Privacy review refs.
    #[serde(default)]
    pub privacy_review_refs: Vec<String>,
    /// Support/export packet refs.
    #[serde(default)]
    pub support_export_refs: Vec<String>,
    /// Reviewable reason this row carries its posture.
    pub rationale: String,
}

impl VoiceSurfaceRow {
    /// True when this row renders at or above the Stable cutline.
    pub fn renders_stable(&self) -> bool {
        self.displayed_label.is_at_or_above_cutline()
    }

    /// True when the row carries a captured, current proof packet.
    pub fn has_green_packet(&self) -> bool {
        self.qualification_packet.as_ref().is_some_and(|packet| {
            packet.has_capture() && packet.slo_state == FreshnessSloState::Current
        })
    }

    fn requires_indicator(&self) -> bool {
        matches!(
            self.activation_default,
            ActivationDefault::ContinuousOptIn | ActivationDefault::WakeWordOptIn
        ) || matches!(
            self.processing_class,
            ProcessingClass::TrustedEnterprise | ProcessingClass::ThirdPartyProvider
        )
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VoiceQualificationSummary {
    /// Total promoted-build rows.
    pub promoted_surface_count: usize,
    /// Rows rendering at Stable.
    pub stable_surface_count: usize,
    /// Rows narrowed below Stable.
    pub narrowed_surface_count: usize,
    /// Stable rows with green packets.
    pub green_packet_count: usize,
    /// Rows carrying preview/labs labels.
    pub preview_or_labs_count: usize,
}

/// Canonical voice/dictation qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VoiceAndDictationSurfaceQualification {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Human-readable release document.
    pub release_doc_ref: String,
    /// User-facing help projection.
    pub help_doc_ref: String,
    /// JSON Schema path.
    pub schema_ref: String,
    /// Surface rows.
    pub surfaces: Vec<VoiceSurfaceRow>,
    /// Summary counts.
    pub summary: VoiceQualificationSummary,
}

impl VoiceAndDictationSurfaceQualification {
    /// Returns rows rendered at Stable.
    pub fn stable_surfaces(&self) -> Vec<&VoiceSurfaceRow> {
        self.surfaces
            .iter()
            .filter(|surface| surface.renders_stable())
            .collect()
    }

    /// Returns rows narrowed below Stable.
    pub fn narrowed_surfaces(&self) -> Vec<&VoiceSurfaceRow> {
        self.surfaces
            .iter()
            .filter(|surface| !surface.renders_stable())
            .collect()
    }

    /// Recomputes summary counts from row state.
    pub fn computed_summary(&self) -> VoiceQualificationSummary {
        let promoted: Vec<&VoiceSurfaceRow> = self
            .surfaces
            .iter()
            .filter(|surface| surface.promoted_build_surface)
            .collect();
        VoiceQualificationSummary {
            promoted_surface_count: promoted.len(),
            stable_surface_count: promoted
                .iter()
                .filter(|surface| surface.renders_stable())
                .count(),
            narrowed_surface_count: promoted
                .iter()
                .filter(|surface| !surface.renders_stable())
                .count(),
            green_packet_count: promoted
                .iter()
                .filter(|surface| surface.renders_stable() && surface.has_green_packet())
                .count(),
            preview_or_labs_count: promoted
                .iter()
                .filter(|surface| !surface.renders_stable())
                .count(),
        }
    }

    /// Validates structural invariants that do not depend on wall-clock arithmetic.
    pub fn validate(&self) -> Vec<VoiceQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != VOICE_DICTATION_SURFACE_QUALIFICATION_SCHEMA_VERSION {
            violations.push(VoiceQualificationViolation::SchemaVersion {
                expected: VOICE_DICTATION_SURFACE_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != VOICE_DICTATION_SURFACE_QUALIFICATION_RECORD_KIND {
            violations.push(VoiceQualificationViolation::RecordKind {
                expected: VOICE_DICTATION_SURFACE_QUALIFICATION_RECORD_KIND.to_string(),
                actual: self.record_kind.clone(),
            });
        }

        let mut ids = BTreeSet::new();
        for surface in &self.surfaces {
            if !ids.insert(surface.surface_id.clone()) {
                violations.push(VoiceQualificationViolation::DuplicateSurfaceId {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if surface.displayed_label.rank() > surface.claim_label.rank() {
                violations.push(VoiceQualificationViolation::DisplayedWiderThanClaim {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if surface.promoted_build_surface
                && surface.renders_stable()
                && !surface.has_green_packet()
            {
                violations.push(
                    VoiceQualificationViolation::StableSurfaceWithoutGreenPacket {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.renders_stable() && !surface.owner_signoff.signed_off {
                violations.push(
                    VoiceQualificationViolation::StableSurfaceMissingOwnerSignoff {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.renders_stable()
                && (!surface.explicit_mode_visible
                    || !surface.mute_or_stop_action
                    || !surface.keyboard_fallback)
            {
                violations.push(VoiceQualificationViolation::MissingExplicitModeOrFallback {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if surface.renders_stable() && !surface.activation_default.allowed_for_stable_default()
            {
                violations.push(VoiceQualificationViolation::UnsafeActivationDefault {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if surface.renders_stable()
                && surface.requires_indicator()
                && !surface.persistent_indicator
            {
                violations.push(VoiceQualificationViolation::MissingPersistentIndicator {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if surface.renders_stable()
                && (!surface.transcript_retention.bounded_or_safer()
                    || !surface.transcript_privacy.complete())
            {
                violations.push(VoiceQualificationViolation::IncompleteTranscriptPrivacy {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if surface.renders_stable() && !surface.ui_primitives.complete() {
                violations.push(VoiceQualificationViolation::IncompleteUiPrimitives {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if surface.renders_stable() && !surface.command_parity.complete() {
                violations.push(VoiceQualificationViolation::IncompleteCommandParity {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if surface.renders_stable() && surface.fallback_states.is_empty() {
                violations.push(VoiceQualificationViolation::MissingFallbackStates {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if surface.renders_stable() && !surface.projection.complete() {
                violations.push(VoiceQualificationViolation::IncompleteProjection {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if surface.renders_stable()
                && (surface.accessibility_refs.is_empty()
                    || surface.regression_refs.is_empty()
                    || surface.privacy_review_refs.is_empty()
                    || surface.support_export_refs.is_empty())
            {
                violations.push(VoiceQualificationViolation::MissingEvidenceRefs {
                    surface_id: surface.surface_id.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(VoiceQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in voice/dictation qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_voice_and_dictation_surface_qualification(
) -> Result<VoiceAndDictationSurfaceQualification, serde_json::Error> {
    serde_json::from_str(VOICE_DICTATION_SURFACE_QUALIFICATION_JSON)
}

/// Validation failure for the voice/dictation qualification packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VoiceQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// Surface ids must be unique.
    DuplicateSurfaceId { surface_id: String },
    /// Displayed lifecycle label is wider than the row claim.
    DisplayedWiderThanClaim { surface_id: String },
    /// A Stable promoted surface lacks a current captured proof packet.
    StableSurfaceWithoutGreenPacket { surface_id: String },
    /// A Stable promoted surface lacks owner sign-off.
    StableSurfaceMissingOwnerSignoff { surface_id: String },
    /// Mode truth, stop action, or keyboard fallback is missing.
    MissingExplicitModeOrFallback { surface_id: String },
    /// Continuous or wake-word behavior is the default.
    UnsafeActivationDefault { surface_id: String },
    /// Persistent indicator is missing for opt-in listening or remote routing.
    MissingPersistentIndicator { surface_id: String },
    /// Transcript retention or privacy controls are incomplete.
    IncompleteTranscriptPrivacy { surface_id: String },
    /// Required voice UI primitives are incomplete.
    IncompleteUiPrimitives { surface_id: String },
    /// Command graph parity is incomplete.
    IncompleteCommandParity { surface_id: String },
    /// Unavailable or fallback states are missing.
    MissingFallbackStates { surface_id: String },
    /// Docs, Help, About, CLI, support, or privacy projection is incomplete.
    IncompleteProjection { surface_id: String },
    /// Accessibility, regression, privacy, or support evidence is missing.
    MissingEvidenceRefs { surface_id: String },
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for VoiceQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema_version expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record_kind expected {expected}, got {actual}")
            }
            Self::DuplicateSurfaceId { surface_id } => write!(f, "{surface_id} is duplicated"),
            Self::DisplayedWiderThanClaim { surface_id } => {
                write!(f, "{surface_id} displays wider than its claim")
            }
            Self::StableSurfaceWithoutGreenPacket { surface_id } => {
                write!(f, "{surface_id} is Stable without a green packet")
            }
            Self::StableSurfaceMissingOwnerSignoff { surface_id } => {
                write!(f, "{surface_id} is Stable without owner sign-off")
            }
            Self::MissingExplicitModeOrFallback { surface_id } => {
                write!(f, "{surface_id} lacks explicit mode truth or fallback")
            }
            Self::UnsafeActivationDefault { surface_id } => {
                write!(f, "{surface_id} has an unsafe activation default")
            }
            Self::MissingPersistentIndicator { surface_id } => {
                write!(
                    f,
                    "{surface_id} lacks persistent capture/provider indicator"
                )
            }
            Self::IncompleteTranscriptPrivacy { surface_id } => {
                write!(f, "{surface_id} lacks transcript privacy controls")
            }
            Self::IncompleteUiPrimitives { surface_id } => {
                write!(f, "{surface_id} lacks voice UI primitives")
            }
            Self::IncompleteCommandParity { surface_id } => {
                write!(f, "{surface_id} lacks command parity proof")
            }
            Self::MissingFallbackStates { surface_id } => {
                write!(f, "{surface_id} lacks fallback states")
            }
            Self::IncompleteProjection { surface_id } => {
                write!(f, "{surface_id} lacks full projection coverage")
            }
            Self::MissingEvidenceRefs { surface_id } => {
                write!(f, "{surface_id} lacks evidence refs")
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for VoiceQualificationViolation {}
