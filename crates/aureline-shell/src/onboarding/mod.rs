//! First-run onboarding alpha projection for the shell.
//!
//! This module is the first shell-owned consumer for the account-free
//! onboarding lane. It joins Start Center entry verbs, launch-bundle
//! recommendations, help-search anchors, locale fallback, learning-pack
//! posture, and portable onboarding state into one exportable record.

use std::path::Path;

use aureline_auth::{IdentityModeAlias, IdentityModeBaselinePacket, IdentityModeSurfaceRow};
use aureline_commands::alpha::{alpha_command_registry, AlphaCommandRegistryRecord};
use aureline_commands::registry::seeded_registry;
use aureline_commands::{CommandRegistry, PreflightDecisionClass};
use aureline_input::keybindings::PlatformClass;
use aureline_input::presets::{preset_binding_rows, KeymapPresetId};
use aureline_telemetry::onboarding::OnboardingEventName;
use aureline_workspace::{
    build_admission_checkpoint_route, review_non_widening_import, AdmissionCheckpointBuildRequest,
    AdmissionCheckpointRouteRecord, AdmissionClass, AdmissionSourceSurface, ArchetypeTruth,
    BlockedReasonClass, ContinueWithoutClass, DetectionConfidenceClass, DetectionOutcome,
    DetectionSignal, DetectionSignalSourceClass, DetectorState, EntryVerb, ExecutionBoundary,
    FirstUsefulEntrySource, ImportApplyRequest, ImportApplyReview, ReadinessBucket,
    ReadinessBuckets, ReadinessTask, ReadinessTaskClass, ReadinessTaskState, ResultingMode,
    SideEffectClass, SignalMaterialEffect, StateSourcePosture, SupportClaimClass, TargetKind,
    TrustReviewClass, TrustState, WideningVector,
};
use serde::{Deserialize, Serialize};

use crate::help::docs_pack::{
    current_docs_pack_manifest, DocsPackAlphaManifest, DocsPackInstallState,
    DocsPackLocaleAvailability, DocsPackLocalityState, DocsPackNode,
};
use crate::help::onboarding_help_pack::{
    current_onboarding_help_pack_alpha_manifest, OnboardingHelpPackAlphaItem,
    OnboardingHelpPackAlphaManifest, OnboardingHelpPackCommandMetadataSource,
    OnboardingHelpPackFallbackClass, OnboardingHelpPackInstallState, OnboardingHelpPackItemKind,
    OnboardingHelpPackLocaleAvailability, OnboardingHelpPackOfflinePosture,
};
use crate::import::{
    materialize_import_diff_review_packet, CompetitorConfigClassifier, ImportDiffReviewPacket,
};
use crate::learning_tour_alpha::current_learning_tour_alpha_manifest;
use crate::start_center::admission_review::{
    admission_packet_for_resolved_entry, import_form_admission_packet,
};
use crate::start_center::first_useful_work::{
    start_center_first_useful_work_surface, StartCenterFirstUsefulWorkSurface,
};
use crate::start_center::{
    build_action_rows, StartCenterPrimaryActionId, StartCenterRuntimeInputs,
};

/// Schema version for [`OnboardingAlphaSurfaceRecord`].
pub const ONBOARDING_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Default generated-at value used by deterministic fixtures and tests.
pub const ONBOARDING_ALPHA_FIXTURE_GENERATED_AT: &str = "fixture:onboarding-alpha";

/// Schema version for [`OnboardingFlow`].
pub const ONBOARDING_FLOW_SCHEMA_VERSION: u32 = 1;

/// Record-kind tag for [`OnboardingFlow`].
pub const ONBOARDING_FLOW_RECORD_KIND: &str = "onboarding_flow_record";

/// Request used to compose one first-run onboarding flow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnboardingFlowRequest {
    /// Stable flow id used by support and tests.
    pub flow_id: String,
    /// Identity-mode baseline packet rendered by the first step.
    pub identity_modes: IdentityModeBaselinePacket,
    /// Identity choice or no-account fast path selected by the user.
    pub identity_mode_choice: OnboardingIdentityModeChoice,
    /// Entry path that determines the optional import branch and landing route.
    pub entry: OnboardingFlowEntry,
}

impl OnboardingFlowRequest {
    /// Builds a request with the provided identity packet, choice, and entry.
    pub fn new(
        flow_id: impl Into<String>,
        identity_modes: IdentityModeBaselinePacket,
        identity_mode_choice: OnboardingIdentityModeChoice,
        entry: OnboardingFlowEntry,
    ) -> Self {
        Self {
            flow_id: flow_id.into(),
            identity_modes,
            identity_mode_choice,
            entry,
        }
    }
}

/// User selection at the identity-mode step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnboardingIdentityModeChoice {
    /// Continue directly to local work without persisting an identity choice.
    NoAccountFastPath,
    /// Persist an explicit identity-mode choice in portable profile state.
    ChosenMode {
        /// Selected identity mode.
        identity_mode: IdentityModeAlias,
    },
}

/// Entry branch for first-run onboarding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnboardingFlowEntry {
    /// Open local work after identity-mode review.
    OpenLocalWorkspace {
        /// Redaction-aware target selected by the user.
        target_specifier: String,
        /// Reviewed target kind.
        target_kind: TargetKind,
        /// Resulting mode admitted before the landing route.
        resulting_mode: ResultingMode,
    },
    /// Review imported profile or handoff state before landing.
    ImportProfile(OnboardingImportFlowRequest),
}

impl OnboardingFlowEntry {
    /// Returns a local folder entry with no account or import requirement.
    pub fn local_folder(target_specifier: impl Into<String>) -> Self {
        Self::OpenLocalWorkspace {
            target_specifier: target_specifier.into(),
            target_kind: TargetKind::LocalFolder,
            resulting_mode: ResultingMode::Folder,
        }
    }
}

/// Request for the optional import branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnboardingImportFlowRequest {
    /// Source path or source ref selected for import.
    pub source_path: String,
    /// Destination profile or workspace target for imported state.
    pub destination_workspace_target: String,
    /// Current owner posture used by the non-widening profile review.
    pub current_owner: StateSourcePosture,
    /// Incoming owner posture used by the non-widening profile review.
    pub incoming_owner: StateSourcePosture,
    /// Widening vectors detected before import apply.
    pub widening_vectors: Vec<WideningVector>,
    /// Whether the proposed import only narrows behavior.
    pub narrowing_only: bool,
}

impl OnboardingImportFlowRequest {
    /// Builds a non-widening local import request.
    pub fn new(
        source_path: impl Into<String>,
        destination_workspace_target: impl Into<String>,
    ) -> Self {
        Self {
            source_path: source_path.into(),
            destination_workspace_target: destination_workspace_target.into(),
            current_owner: StateSourcePosture::LocalOnly,
            incoming_owner: StateSourcePosture::Imported,
            widening_vectors: vec![WideningVector::None],
            narrowing_only: false,
        }
    }

    /// Replaces the widening vectors used by the non-widening review.
    pub fn with_widening_vectors(mut self, vectors: Vec<WideningVector>) -> Self {
        self.widening_vectors = vectors;
        self
    }

    /// Marks the import as a narrowing-only apply candidate.
    pub const fn with_narrowing_only(mut self, narrowing_only: bool) -> Self {
        self.narrowing_only = narrowing_only;
        self
    }
}

/// Composed first-run onboarding flow record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnboardingFlow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version for this flow.
    pub schema_version: u32,
    /// Stable flow id.
    pub flow_id: String,
    /// Ordered stages for identity, import, checkpoint confirmation, and landing.
    pub sequence: Vec<OnboardingFlowStage>,
    /// Identity-mode step projection.
    pub identity_mode_step: OnboardingIdentityModeStep,
    /// Optional import diff review packet.
    pub import_diff_review: Option<ImportDiffReviewPacket>,
    /// Optional non-widening import review result.
    pub import_apply_review: Option<ImportApplyReview>,
    /// Rollback-checkpoint confirmation for the import branch.
    pub rollback_checkpoint_confirmation: RollbackCheckpointConfirmation,
    /// Workspace admission checkpoint route that gates the landing.
    pub admission_checkpoint_route: AdmissionCheckpointRouteRecord,
    /// First-useful-work landing projected for Start Center.
    pub first_useful_work_landing: StartCenterFirstUsefulWorkSurface,
    /// Learning-tour step refs consumed by the flow.
    pub learning_tour_step_refs: Vec<String>,
    /// Privacy-safe onboarding telemetry event names represented by this flow.
    pub telemetry_event_names: Vec<OnboardingEventName>,
    /// Portable records the flow would persist after user confirmation.
    pub persisted_records: Vec<OnboardingFlowPersistedRecord>,
}

impl OnboardingFlow {
    /// Builds the full onboarding flow from existing shell, auth, workspace, and telemetry records.
    ///
    /// # Errors
    ///
    /// Returns [`OnboardingFlowError`] when the identity packet is invalid, the
    /// requested identity mode is missing, or the open entry is not local.
    pub fn build(request: OnboardingFlowRequest) -> Result<Self, OnboardingFlowError> {
        let identity_violations = request
            .identity_modes
            .validate()
            .into_iter()
            .map(|violation| format!("{violation:?}"))
            .collect::<Vec<_>>();
        if !identity_violations.is_empty() {
            return Err(OnboardingFlowError::InvalidIdentityModePacket {
                violations: identity_violations,
            });
        }
        if !request.identity_modes.local_core_remains_account_free() {
            return Err(OnboardingFlowError::LocalCoreNotAccountFree);
        }

        let identity_mode_step =
            identity_mode_step(&request.identity_modes, request.identity_mode_choice)?;
        let mut persisted_records = identity_persisted_records(&identity_mode_step);

        let built_entry = match request.entry {
            OnboardingFlowEntry::OpenLocalWorkspace {
                target_specifier,
                target_kind,
                resulting_mode,
            } => build_open_entry_route(target_specifier, target_kind, resulting_mode)?,
            OnboardingFlowEntry::ImportProfile(import_request) => {
                build_import_entry_route(import_request, &mut persisted_records)
            }
        };

        let rollback_checkpoint_confirmation = rollback_checkpoint_confirmation(
            built_entry.import_diff_review.as_ref(),
            built_entry.import_apply_review.as_ref(),
            &built_entry.admission_checkpoint_route,
        );
        if rollback_checkpoint_confirmation.confirmed_before_apply {
            persisted_records.push(OnboardingFlowPersistedRecord {
                record_kind: OnboardingFlowPersistedRecordKind::RollbackCheckpointConfirmation,
                record_ref: rollback_checkpoint_confirmation
                    .checkpoint_ref
                    .clone()
                    .unwrap_or_else(|| "rollback-checkpoint:missing".to_string()),
                storage_lane: OnboardingStorageLane::PortableUserProfileState,
                reason: "rollback checkpoint confirmed before import apply".to_string(),
            });
        }

        let first_useful_work_landing =
            start_center_first_useful_work_surface(&built_entry.admission_checkpoint_route);
        let sequence = flow_sequence(
            built_entry.import_diff_review.as_ref(),
            &rollback_checkpoint_confirmation,
            &built_entry.admission_checkpoint_route,
        );
        let learning_tour_step_refs =
            learning_tour_step_refs(built_entry.import_diff_review.is_some());
        let telemetry_event_names = telemetry_event_names(
            built_entry.import_diff_review.is_some(),
            rollback_checkpoint_confirmation.confirmed_before_apply,
        );

        Ok(Self {
            record_kind: ONBOARDING_FLOW_RECORD_KIND.to_string(),
            schema_version: ONBOARDING_FLOW_SCHEMA_VERSION,
            flow_id: request.flow_id,
            sequence,
            identity_mode_step,
            import_diff_review: built_entry.import_diff_review,
            import_apply_review: built_entry.import_apply_review,
            rollback_checkpoint_confirmation,
            admission_checkpoint_route: built_entry.admission_checkpoint_route,
            first_useful_work_landing,
            learning_tour_step_refs,
            telemetry_event_names,
            persisted_records,
        })
    }

    /// Returns true when the route reaches a named first-useful-work landing.
    pub fn reaches_first_useful_work_landing(&self) -> bool {
        !self
            .first_useful_work_landing
            .landing_surface
            .trim()
            .is_empty()
            && self.admission_checkpoint_route.is_contract_valid()
    }

    /// Returns true when a persisted identity-choice record is present.
    pub fn has_persisted_identity_choice(&self) -> bool {
        self.persisted_records.iter().any(|record| {
            record.record_kind == OnboardingFlowPersistedRecordKind::IdentityModeChoice
        })
    }

    /// Returns compact support rows for the composed flow.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("onboarding_flow: {}", self.flow_id),
            format!(
                "identity: {} persistence={}",
                self.identity_mode_step
                    .selected_identity_mode_token
                    .as_deref()
                    .unwrap_or("no_account_fast_path"),
                self.identity_mode_step.persistence.as_str()
            ),
            format!(
                "import: {}",
                self.import_diff_review
                    .as_ref()
                    .map(|packet| packet.import_diff_preview_ref.as_str())
                    .unwrap_or("not_requested")
            ),
            format!(
                "rollback: {}",
                self.rollback_checkpoint_confirmation
                    .confirmation_state
                    .as_str()
            ),
            format!(
                "landing: {}",
                self.first_useful_work_landing.landing_surface
            ),
        ];
        lines.extend(self.admission_checkpoint_route.compact_lines());
        lines
    }
}

/// One ordered stage in the composed onboarding flow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnboardingFlowStage {
    /// Stage kind.
    pub stage_kind: OnboardingFlowStageKind,
    /// Whether the stage is required for this branch.
    pub required: bool,
    /// Stable status token for this branch.
    pub status: String,
    /// Record ref consumed or produced by this stage.
    pub record_ref: Option<String>,
}

/// Stage kind in the onboarding flow sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnboardingFlowStageKind {
    /// Identity-mode choice stage.
    IdentityModeChoice,
    /// Optional import diff review stage.
    ImportDiffReview,
    /// Rollback-checkpoint confirmation stage.
    RollbackCheckpointConfirmation,
    /// First-useful-work landing stage.
    FirstUsefulWorkLanding,
}

impl OnboardingFlowStageKind {
    /// Returns the stable token for this stage kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdentityModeChoice => "identity_mode_choice",
            Self::ImportDiffReview => "import_diff_review",
            Self::RollbackCheckpointConfirmation => "rollback_checkpoint_confirmation",
            Self::FirstUsefulWorkLanding => "first_useful_work_landing",
        }
    }
}

/// Identity-mode step projected into the flow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnboardingIdentityModeStep {
    /// Surface rows offered by the identity-mode baseline packet.
    pub available_modes: Vec<IdentityModeSurfaceRow>,
    /// Selected identity mode when the user explicitly chose one.
    pub selected_identity_mode: Option<IdentityModeAlias>,
    /// Stable selected identity-mode token.
    pub selected_identity_mode_token: Option<String>,
    /// Persistence behavior for the identity choice.
    pub persistence: IdentityChoicePersistence,
    /// Whether all offered modes preserve local core without an account.
    pub local_core_available_without_account: bool,
    /// Whether the no-account path bypassed account setup.
    pub account_prompt_bypassed: bool,
}

/// Persistence behavior for the identity-mode step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentityChoicePersistence {
    /// No identity choice is persisted for the no-account fast path.
    NotRecordedNoAccountFastPath,
    /// The explicit choice is stored in portable user/profile state.
    RecordedInPortableProfileState,
}

impl IdentityChoicePersistence {
    /// Returns the stable token for this persistence behavior.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRecordedNoAccountFastPath => "not_recorded_no_account_fast_path",
            Self::RecordedInPortableProfileState => "recorded_in_portable_profile_state",
        }
    }
}

/// Rollback-checkpoint confirmation stage for import review.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollbackCheckpointConfirmation {
    /// Confirmation state for this branch.
    pub confirmation_state: RollbackCheckpointConfirmationState,
    /// Import rollback checkpoint ref, when an import branch exists.
    pub checkpoint_ref: Option<String>,
    /// Admission checkpoint id that gated the branch.
    pub admission_checkpoint_ref: String,
    /// Import diff preview protected by the rollback checkpoint.
    pub import_diff_preview_ref: Option<String>,
    /// Whether the import checkpoint was created before apply.
    pub created_before_apply: bool,
    /// Whether the flow confirmed the checkpoint before apply.
    pub confirmed_before_apply: bool,
}

/// Confirmation state for the rollback checkpoint stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollbackCheckpointConfirmationState {
    /// No import branch exists, so no import rollback checkpoint is required.
    NotRequiredNoImport,
    /// The checkpoint exists and was confirmed before apply.
    Confirmed,
    /// Non-widening review blocked apply before checkpoint confirmation.
    BlockedByNonWideningReview,
    /// Import diff review exists but did not expose a usable checkpoint.
    MissingCheckpoint,
}

impl RollbackCheckpointConfirmationState {
    /// Returns the stable token for this confirmation state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequiredNoImport => "not_required_no_import",
            Self::Confirmed => "confirmed",
            Self::BlockedByNonWideningReview => "blocked_by_non_widening_review",
            Self::MissingCheckpoint => "missing_checkpoint",
        }
    }
}

/// Portable record written by the flow after review.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnboardingFlowPersistedRecord {
    /// Kind of record persisted.
    pub record_kind: OnboardingFlowPersistedRecordKind,
    /// Stable record ref.
    pub record_ref: String,
    /// Storage lane used for the record.
    pub storage_lane: OnboardingStorageLane,
    /// Redaction-aware reason for persisting the record.
    pub reason: String,
}

/// Persisted record kind for onboarding flow state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnboardingFlowPersistedRecordKind {
    /// Explicit identity-mode choice.
    IdentityModeChoice,
    /// Imported profile history or retained migration report.
    ImportedProfileHistory,
    /// Rollback checkpoint confirmation.
    RollbackCheckpointConfirmation,
}

impl OnboardingFlowPersistedRecordKind {
    /// Returns the stable token for this persisted record kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdentityModeChoice => "identity_mode_choice",
            Self::ImportedProfileHistory => "imported_profile_history",
            Self::RollbackCheckpointConfirmation => "rollback_checkpoint_confirmation",
        }
    }
}

/// Error returned while composing an onboarding flow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnboardingFlowError {
    /// The identity-mode baseline packet failed validation.
    InvalidIdentityModePacket {
        /// Debug-formatted validation violations.
        violations: Vec<String>,
    },
    /// Local core was not available without account creation.
    LocalCoreNotAccountFree,
    /// Requested identity mode was not present in the packet.
    MissingIdentityMode {
        /// Requested identity mode.
        identity_mode: IdentityModeAlias,
    },
    /// The no-account fast path could not find the local identity row.
    MissingAccountFreeLocalMode,
    /// The open branch used a non-local target or incompatible resulting mode.
    InvalidLocalEntry {
        /// Target kind supplied by the caller.
        target_kind: TargetKind,
        /// Resulting mode supplied by the caller.
        resulting_mode: ResultingMode,
    },
}

impl std::fmt::Display for OnboardingFlowError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidIdentityModePacket { violations } => {
                write!(
                    formatter,
                    "identity-mode packet failed validation: {}",
                    violations.join(", ")
                )
            }
            Self::LocalCoreNotAccountFree => formatter
                .write_str("identity-mode packet does not preserve account-free local core"),
            Self::MissingIdentityMode { identity_mode } => {
                write!(
                    formatter,
                    "identity mode {} is not present",
                    identity_mode.as_str()
                )
            }
            Self::MissingAccountFreeLocalMode => {
                formatter.write_str("account-free local identity mode is not present")
            }
            Self::InvalidLocalEntry {
                target_kind,
                resulting_mode,
            } => write!(
                formatter,
                "local onboarding entry cannot use target {} with resulting mode {}",
                target_kind.as_str(),
                resulting_mode.as_str()
            ),
        }
    }
}

impl std::error::Error for OnboardingFlowError {}

/// Complete first-run onboarding alpha projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingAlphaSurfaceRecord {
    /// Record discriminator for support exports and fixtures.
    pub record_kind: String,
    /// Integer schema version for this alpha projection.
    pub onboarding_alpha_schema_version: u32,
    /// Monotonic or fixture timestamp for the projection.
    pub generated_at: String,
    /// Launch context and account posture for the first-run surface.
    pub launch_context: OnboardingLaunchContext,
    /// Alpha command-registry publication consumed by this onboarding projection.
    pub alpha_command_registry_ref: String,
    /// Proof that local useful work remains available without account setup.
    pub no_account_path: NoAccountPathProof,
    /// Distinct entry verbs surfaced on the first-run wedge.
    pub entry_verbs: Vec<OnboardingEntryVerbRow>,
    /// Launch-bundle or native-path recommendation cards.
    pub recommendation_cards: Vec<OnboardingRecommendationCard>,
    /// Onboarding, migration, keymap, and contextual teaching cards.
    pub teaching_cards: Vec<OnboardingTeachingCard>,
    /// Help-search rows and pack posture visible on the alpha wedge.
    pub help_search: OnboardingHelpSearchProjection,
    /// Portable user/profile state items owned by onboarding.
    pub portable_state: OnboardingPortableStateProjection,
    /// Learning-digest handoff or truthful not-installed placeholder.
    pub learning_digest: LearningDigestProjection,
    /// Round-trip proofs from alpha command descriptors into onboarding consumers.
    pub command_descriptor_round_trips: Vec<OnboardingCommandDescriptorRoundTrip>,
}

impl OnboardingAlphaSurfaceRecord {
    /// Renders the projection as deterministic plaintext for support packets.
    pub fn render_plaintext(&self) -> String {
        let mut lines = vec![
            "Onboarding alpha".to_string(),
            format!("schema_version: {}", self.onboarding_alpha_schema_version),
            format!(
                "no_account_local_work: {}",
                self.no_account_path.local_work_available
            ),
            "entry_verbs:".to_string(),
        ];

        for row in &self.entry_verbs {
            lines.push(format!(
                "- {} => {} ({}) via {}",
                row.entry_verb_class.as_str(),
                row.command.command_id,
                row.command.anchor_source.as_str(),
                row.command.keyboard_route
            ));
        }

        lines.push("recommendations:".to_string());
        for card in &self.recommendation_cards {
            lines.push(format!(
                "- {} [{}] remembered={}",
                card.card_id,
                card.recommendation_source_class.as_str(),
                card.remembered_choice_effect.as_str()
            ));
            lines.push(format!(
                "  actions: {}",
                card.actions
                    .iter()
                    .map(|action| action.action_class.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        lines.push("help_search:".to_string());
        for item in &self.help_search.items {
            lines.push(format!(
                "- {} => {} locale {}->{} posture {} fallback {}",
                item.item_id,
                item.command.command_id,
                item.requested_locale,
                item.effective_locale,
                item.pack_install_state.as_str(),
                item.source_language_fallback_class.as_str()
            ));
        }

        lines.push("portable_state:".to_string());
        for item in &self.portable_state.items {
            lines.push(format!(
                "- {} {} {}",
                item.state_item_id,
                item.state_kind.as_str(),
                item.storage_lane.as_str()
            ));
        }

        lines.push(format!(
            "learning_digest: {} via {}",
            self.learning_digest.availability_class.as_str(),
            self.learning_digest.open_or_placeholder_command.command_id
        ));
        lines.push(format!(
            "alpha_command_registry_ref: {}",
            self.alpha_command_registry_ref
        ));
        lines.push(String::new());
        lines.join("\n")
    }
}

/// Round-trip proof from the alpha command descriptor into onboarding surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingCommandDescriptorRoundTrip {
    /// Consumer class such as Start Center, onboarding hint, or help search.
    pub consumer_class: String,
    /// Stable consumer ref inside the owning onboarding or help surface.
    pub consumer_ref: String,
    /// Stable command id projected from the alpha registry.
    pub command_id: String,
    /// Keyboard or intent route shown by the consumer.
    pub keyboard_route: String,
    /// Docs/help anchor projected from the descriptor.
    pub descriptor_anchor_ref: String,
    /// Invocation or result fixture that proves the route, when runnable.
    pub invocation_packet_ref: Option<String>,
    /// Whether preview/apply semantics are preserved by the consumer.
    pub preserves_preview_apply_semantics: bool,
    /// Disabled-reason handling mode used by the consumer.
    pub disabled_reason_mode: String,
    /// Exact reopen ref preserved for Start Center/help/support consumers.
    pub exact_reopen_ref: Option<String>,
}

/// Launch context projected onto the first-run surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingLaunchContext {
    /// Surface identifier that renders the projection.
    pub surface_id: String,
    /// Deployment profile used for local-first gating.
    pub deployment_profile: String,
    /// Active profile scope for state ownership.
    pub profile_scope: String,
    /// Account prompt class for the overall surface.
    pub account_prompt_class: AccountPromptClass,
    /// Whether the surface is allowed to show account material above local work.
    pub account_content_secondary: bool,
}

/// Proof that the first-run path keeps local work usable without sign-in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoAccountPathProof {
    /// Whether a local user can open useful work without an account.
    pub local_work_available: bool,
    /// Whether sign-in, sync, telemetry, and marketplace setup are optional.
    pub service_setup_optional: bool,
    /// Entry verbs that remain visible while no account exists.
    pub preserved_entry_verbs: Vec<EntryVerbClass>,
    /// Stable command ids that prove the local path is command-backed.
    pub local_command_ids: Vec<String>,
}

/// One distinct first-run entry verb.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingEntryVerbRow {
    /// Distinct verb class rendered to the user.
    pub entry_verb_class: EntryVerbClass,
    /// Stable Start Center action token, when the row maps to one.
    pub primary_action_token: Option<String>,
    /// Human label rendered by the entry surface.
    pub label: String,
    /// Stable command anchor and keyboard route.
    pub command: OnboardingCommandAnchor,
    /// Whether the row can proceed without account creation.
    pub no_account_allowed: bool,
    /// Whether the row requires admission, restore, or import review first.
    pub review_required_before_mutation: bool,
    /// Whether setup, trust, or package work is deliberately deferred.
    pub setup_or_trust_deferred: bool,
    /// Preflight decision class from the command registry, when registered.
    pub preflight_decision_class: Option<String>,
}

/// Stable command anchor rendered by onboarding, help, or recommendation rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingCommandAnchor {
    /// Stable command id or owned alpha command id.
    pub command_id: String,
    /// Keyboard route shown to keyboard users.
    pub keyboard_route: String,
    /// Where this command anchor is declared.
    pub anchor_source: CommandAnchorSource,
    /// Optional command registry entry id when the anchor is registry-backed.
    pub registry_entry_id: Option<String>,
    /// Optional docs/help anchor connected to the command.
    pub docs_anchor_ref: Option<String>,
}

impl OnboardingCommandAnchor {
    fn registry(command_id: impl Into<String>, registry: &CommandRegistry) -> Self {
        let command_id = command_id.into();
        let entry = registry.get(&command_id);
        Self {
            keyboard_route: keyboard_route_for(&command_id),
            registry_entry_id: entry.map(|entry| entry.registry_entry_id.clone()),
            docs_anchor_ref: entry
                .map(|entry| entry.descriptor.docs_help_anchor_ref.anchor_id.clone()),
            command_id,
            anchor_source: CommandAnchorSource::SeededCommandRegistry,
        }
    }

    fn alpha_owned(
        command_id: impl Into<String>,
        keyboard_route: impl Into<String>,
        docs_anchor_ref: impl Into<String>,
    ) -> Self {
        Self {
            command_id: command_id.into(),
            keyboard_route: keyboard_route.into(),
            anchor_source: CommandAnchorSource::OnboardingAlphaOwned,
            registry_entry_id: None,
            docs_anchor_ref: Some(docs_anchor_ref.into()),
        }
    }
}

/// Launch-bundle or native-path recommendation shown by onboarding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingRecommendationCard {
    /// Stable card id.
    pub card_id: String,
    /// Recommendation class used for support and help search.
    pub recommendation_source_class: RecommendationSourceClass,
    /// Stable bundle or native-path reference.
    pub recommendation_ref: String,
    /// Command id that opens help search for this recommendation.
    pub help_search_command: OnboardingCommandAnchor,
    /// Keymap bridge rows connected to this recommendation.
    pub keymap_bridge_refs: Vec<String>,
    /// Explicit same-weight actions available from the card.
    pub actions: Vec<RecommendationAction>,
    /// What remembering this choice is allowed to restore later.
    pub remembered_choice_effect: RememberedChoiceEffect,
    /// Whether review is still required on later opens.
    pub review_required_on_later_open: bool,
    /// Whether the card can silently install packages.
    pub can_silently_install: bool,
    /// Whether the card can silently widen workspace trust.
    pub can_silently_widen_trust: bool,
}

/// One explicit action shown on a recommendation card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationAction {
    /// Stable action class.
    pub action_class: RecommendationActionClass,
    /// Stable command anchor for the action.
    pub command: OnboardingCommandAnchor,
    /// Whether the action requires a review sheet before mutation.
    pub review_required_before_effect: bool,
}

/// Onboarding, migration, keymap, or contextual teaching card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingTeachingCard {
    /// Stable card id.
    pub card_id: String,
    /// Teaching card family.
    pub card_kind: TeachingCardKind,
    /// Primary command anchor the card teaches.
    pub command: OnboardingCommandAnchor,
    /// Stable migration or keymap bridge references.
    pub bridge_refs: Vec<String>,
    /// Docs or citation anchors backing the card.
    pub citation_refs: Vec<String>,
    /// Dismissal state item this card writes when dismissed.
    pub dismissal_state_item_ref: String,
    /// Whether the card can run a mutating action without review.
    pub hidden_mutation_allowed: bool,
}

/// Help-search and docs-pack projection visible during onboarding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpSearchProjection {
    /// Stable help-search packet id.
    pub projection_id: String,
    /// Stable command that opens help search.
    pub help_search_command: OnboardingCommandAnchor,
    /// Pack posture rows used by the help results.
    pub pack_states: Vec<OnboardingPackState>,
    /// Search result rows.
    pub items: Vec<OnboardingHelpSearchItem>,
    /// Whether support export can reconstruct what the user saw.
    pub support_export_reconstructable: bool,
}

/// One docs, glossary, or learning pack state used by onboarding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingPackState {
    /// Stable pack id.
    pub pack_id: String,
    /// Stable pack role.
    pub pack_role: PackRole,
    /// Source version or build identity for the pack.
    pub source_version_ref: String,
    /// Install state visible to the user.
    pub install_state: PackInstallState,
    /// Locale availability visible to the user.
    pub locale_availability: LocaleAvailability,
    /// Cache, mirror, or local-only posture.
    pub offline_posture: OfflinePosture,
    /// Whether pack citations can be exported.
    pub citations_exportable: bool,
}

/// One help-search result or contextual help item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpSearchItem {
    /// Stable item id.
    pub item_id: String,
    /// Stable pack id that owns the item.
    pub pack_id: String,
    /// Search surface that can render the item.
    pub surface_class: HelpSurfaceClass,
    /// Command anchor the item resolves through.
    pub command: OnboardingCommandAnchor,
    /// Requested locale.
    pub requested_locale: String,
    /// Effective locale rendered.
    pub effective_locale: String,
    /// Visible fallback posture.
    pub source_language_fallback_class: SourceLanguageFallbackClass,
    /// Pack install state repeated at the item for support export.
    pub pack_install_state: PackInstallState,
    /// Citation anchors visible from the item.
    pub citation_refs: Vec<String>,
    /// Docs-node id that supplied this onboarding item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_node_id: Option<String>,
    /// Pack revision ref preserved for support export and reopen.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_pack_revision_ref: Option<String>,
    /// Source strip ref opened from docs-node metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_strip_ref: Option<String>,
    /// Citation drawer ref opened from docs-node metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation_drawer_ref: Option<String>,
    /// Exact reopen ref preserving pack revision and locale.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_reopen_ref: Option<String>,
}

/// Portable user/profile state projected by onboarding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingPortableStateProjection {
    /// Stable state bundle id.
    pub bundle_id: String,
    /// Portable state items owned by onboarding.
    pub items: Vec<OnboardingStateItem>,
    /// Whether any item is stored in workspace-local hidden state.
    pub any_workspace_local_hidden_store: bool,
    /// Whether the bundle can be exported with a portable profile.
    pub portable_profile_exportable: bool,
}

/// One portable onboarding state item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingStateItem {
    /// Stable state item id.
    pub state_item_id: String,
    /// Distinct state kind.
    pub state_kind: OnboardingStateKind,
    /// Storage lane for the item.
    pub storage_lane: OnboardingStorageLane,
    /// Profile scope for the item.
    pub profile_scope_class: ProfileScopeClass,
    /// Reset class for the item.
    pub reset_class: OnboardingResetClass,
    /// Export class for the item.
    pub export_class: OnboardingExportClass,
    /// Short reason the item lives in this lane.
    pub portability_reason: String,
}

/// Learning digest availability for first-run onboarding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningDigestProjection {
    /// Stable digest id or placeholder id.
    pub digest_ref: String,
    /// Whether a versioned learning digest is installed.
    pub availability_class: LearningDigestAvailability,
    /// Command or placeholder action that opens the digest row.
    pub open_or_placeholder_command: OnboardingCommandAnchor,
    /// Pack id or missing-pack id connected to the digest.
    pub pack_id: String,
    /// Whether the placeholder preserves local useful work.
    pub no_account_continuity_preserved: bool,
    /// Whether command and docs anchors remain exact across reopen.
    pub exact_reopen_preserves_anchors: bool,
}

/// Account prompt class for onboarding entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountPromptClass {
    /// The row or surface has no account prompt.
    NoPrompt,
    /// The row may show a deferrable optional account prompt.
    OptionalPrompt,
}

impl AccountPromptClass {
    /// Returns the stable token for this account prompt class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPrompt => "no_prompt",
            Self::OptionalPrompt => "optional_prompt",
        }
    }
}

/// Distinct first-run entry verbs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryVerbClass {
    /// Open a local file, folder, repository, or workspace.
    Open,
    /// Clone a repository after review.
    Clone,
    /// Import a profile, settings root, archive, or handoff packet.
    Import,
    /// Restore a previous session or checkpoint.
    Restore,
    /// Open a recent local or remote work item.
    RecentWork,
}

impl EntryVerbClass {
    /// Returns the stable token for this entry verb.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Clone => "clone",
            Self::Import => "import",
            Self::Restore => "restore",
            Self::RecentWork => "recent_work",
        }
    }
}

/// Source of a stable command anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandAnchorSource {
    /// The command is present in the seeded command registry.
    SeededCommandRegistry,
    /// The command id is owned by this alpha onboarding contract.
    OnboardingAlphaOwned,
}

impl CommandAnchorSource {
    /// Returns the stable token for this anchor source.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SeededCommandRegistry => "seeded_command_registry",
            Self::OnboardingAlphaOwned => "onboarding_alpha_owned",
        }
    }
}

/// Recommendation source class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationSourceClass {
    /// Recommendation comes from a checked-in launch bundle.
    LaunchBundle,
    /// Recommendation is a native local path with no bundle install.
    NativeLocalPath,
}

impl RecommendationSourceClass {
    /// Returns the stable token for this source class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaunchBundle => "launch_bundle",
            Self::NativeLocalPath => "native_local_path",
        }
    }
}

/// Explicit recommendation action class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationActionClass {
    /// Apply the recommendation through a review path.
    Apply,
    /// Compare the recommendation before adopting it.
    Compare,
    /// Dismiss the recommendation.
    Dismiss,
    /// Open the workspace with optional setup skipped.
    OpenMinimal,
    /// Defer setup for later.
    SetUpLater,
}

impl RecommendationActionClass {
    /// Returns the stable token for this action class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Apply => "apply",
            Self::Compare => "compare",
            Self::Dismiss => "dismiss",
            Self::OpenMinimal => "open_minimal",
            Self::SetUpLater => "set_up_later",
        }
    }
}

/// Effect of remembering a recommendation choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RememberedChoiceEffect {
    /// Only the preference is restored; authority and review are not widened.
    PreferenceOnlyNoAuthorityChange,
}

impl RememberedChoiceEffect {
    /// Returns the stable token for this remembered-choice effect.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreferenceOnlyNoAuthorityChange => "preference_only_no_authority_change",
        }
    }
}

/// Teaching card family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TeachingCardKind {
    /// Card teaching a first-run onboarding action.
    OnboardingCard,
    /// Card explaining migration/import deltas.
    MigrationHint,
    /// Card mapping imported keymap behavior.
    KeymapBridge,
    /// Card offering contextual help near a task.
    ContextualTip,
}

/// Role of an onboarding pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackRole {
    /// Baked-in first-run starter content.
    FirstRunStarterPack,
    /// Migration welcome or bridge content.
    MigrationWelcomePack,
    /// Glossary bundle content.
    GlossaryBundle,
    /// Guided learning digest content.
    GuidedContentPack,
}

/// Install state for onboarding help or docs packs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackInstallState {
    /// Pack ships inside the local product build.
    LocalOnlyStarter,
    /// Pack is cached and current.
    CachedSnapshotCurrent,
    /// Pack is referenced but not installed.
    NotInstalled,
}

impl PackInstallState {
    /// Returns the stable token for this pack install state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyStarter => "local_only_starter",
            Self::CachedSnapshotCurrent => "cached_snapshot_current",
            Self::NotInstalled => "not_installed",
        }
    }
}

/// Locale availability for a pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocaleAvailability {
    /// Requested locale has reviewed coverage.
    LocaleAvailableReviewed,
    /// Requested locale falls back to the pack primary locale.
    LocaleMissingFallbackToPrimary,
    /// Requested locale-specific pack is not installed.
    LocaleMissingNotInstalled,
}

/// Offline posture for a pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfflinePosture {
    /// Content is available from the local build.
    FullyAvailableOfflineLocalBuild,
    /// Content is available from a cached snapshot.
    CachedSnapshotOffline,
    /// Content is unavailable until installed.
    NotAvailableOffline,
}

/// Help surface class for a search item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelpSurfaceClass {
    /// Help search result.
    HelpSearch,
    /// Contextual tip result.
    ContextualTip,
    /// Source-language fallback result.
    SourceLanguageFallback,
}

/// Source-language fallback posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceLanguageFallbackClass {
    /// No fallback was needed.
    NoFallbackPrimaryLocaleOnly,
    /// Fallback is visible and command ids are preserved.
    FallbackToSourceLanguageDisclosed,
    /// The pack is missing, so a placeholder is rendered.
    FallbackBlockedPackMissing,
}

impl SourceLanguageFallbackClass {
    /// Returns the stable token for this fallback class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoFallbackPrimaryLocaleOnly => "no_fallback_primary_locale_only",
            Self::FallbackToSourceLanguageDisclosed => "fallback_to_source_language_disclosed",
            Self::FallbackBlockedPackMissing => "fallback_blocked_pack_missing",
        }
    }
}

/// Distinct portable onboarding state kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingStateKind {
    /// A hint or tip was dismissed.
    DismissedHint,
    /// A task was completed.
    CompletedTask,
    /// Setup was deferred.
    DeferredSetup,
    /// A protected recovery recommendation is retained.
    ProtectedRecoveryRecommendation,
    /// Imported-profile history is retained.
    ImportedProfileHistory,
}

impl OnboardingStateKind {
    /// Returns the stable token for this state kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DismissedHint => "dismissed_hint",
            Self::CompletedTask => "completed_task",
            Self::DeferredSetup => "deferred_setup",
            Self::ProtectedRecoveryRecommendation => "protected_recovery_recommendation",
            Self::ImportedProfileHistory => "imported_profile_history",
        }
    }
}

/// Storage lane for onboarding state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingStorageLane {
    /// State lives in portable user/profile state.
    PortableUserProfileState,
}

impl OnboardingStorageLane {
    /// Returns the stable token for this storage lane.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PortableUserProfileState => "portable_user_profile_state",
        }
    }
}

/// Profile scope for onboarding state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileScopeClass {
    /// State is scoped to the current profile.
    PerProfile,
}

/// Reset class for onboarding state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingResetClass {
    /// State can reset per profile.
    ResettablePerProfile,
}

/// Export class for onboarding state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingExportClass {
    /// State is exported in a portable profile package.
    InPortableProfilePackage,
    /// State is exported in a redacted support bundle.
    InSupportBundleRedacted,
}

/// Availability for the first-run learning digest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningDigestAvailability {
    /// No guided learning pack is installed, and the placeholder says so.
    NotInstalledPlaceholder,
}

impl LearningDigestAvailability {
    /// Returns the stable token for this availability value.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotInstalledPlaceholder => "not_installed_placeholder",
        }
    }
}

struct BuiltOnboardingEntry {
    admission_checkpoint_route: AdmissionCheckpointRouteRecord,
    import_diff_review: Option<ImportDiffReviewPacket>,
    import_apply_review: Option<ImportApplyReview>,
}

fn identity_mode_step(
    packet: &IdentityModeBaselinePacket,
    choice: OnboardingIdentityModeChoice,
) -> Result<OnboardingIdentityModeStep, OnboardingFlowError> {
    let available_modes = packet.surface_rows();
    match choice {
        OnboardingIdentityModeChoice::NoAccountFastPath => {
            if !packet
                .identity_mode_rows
                .iter()
                .any(|row| row.identity_mode == IdentityModeAlias::AccountFreeLocal)
            {
                return Err(OnboardingFlowError::MissingAccountFreeLocalMode);
            }
            Ok(OnboardingIdentityModeStep {
                available_modes,
                selected_identity_mode: None,
                selected_identity_mode_token: None,
                persistence: IdentityChoicePersistence::NotRecordedNoAccountFastPath,
                local_core_available_without_account: packet.local_core_remains_account_free(),
                account_prompt_bypassed: true,
            })
        }
        OnboardingIdentityModeChoice::ChosenMode { identity_mode } => {
            if !packet
                .identity_mode_rows
                .iter()
                .any(|row| row.identity_mode == identity_mode)
            {
                return Err(OnboardingFlowError::MissingIdentityMode { identity_mode });
            }
            Ok(OnboardingIdentityModeStep {
                available_modes,
                selected_identity_mode: Some(identity_mode),
                selected_identity_mode_token: Some(identity_mode.as_str().to_string()),
                persistence: IdentityChoicePersistence::RecordedInPortableProfileState,
                local_core_available_without_account: packet.local_core_remains_account_free(),
                account_prompt_bypassed: identity_mode == IdentityModeAlias::AccountFreeLocal,
            })
        }
    }
}

fn identity_persisted_records(
    step: &OnboardingIdentityModeStep,
) -> Vec<OnboardingFlowPersistedRecord> {
    if step.persistence != IdentityChoicePersistence::RecordedInPortableProfileState {
        return Vec::new();
    }
    let Some(token) = step.selected_identity_mode_token.as_ref() else {
        return Vec::new();
    };
    vec![OnboardingFlowPersistedRecord {
        record_kind: OnboardingFlowPersistedRecordKind::IdentityModeChoice,
        record_ref: format!("profile:onboarding.identity_mode_choice:{token}"),
        storage_lane: OnboardingStorageLane::PortableUserProfileState,
        reason: "explicit identity mode selected by the user".to_string(),
    }]
}

fn build_open_entry_route(
    target_specifier: String,
    target_kind: TargetKind,
    resulting_mode: ResultingMode,
) -> Result<BuiltOnboardingEntry, OnboardingFlowError> {
    if !matches!(
        (target_kind, resulting_mode),
        (TargetKind::LocalFolder, ResultingMode::Folder)
            | (TargetKind::LocalRepoRoot, ResultingMode::RepoRoot)
            | (
                TargetKind::WorkspaceManifest,
                ResultingMode::WorkspaceWithRoots
            )
    ) {
        return Err(OnboardingFlowError::InvalidLocalEntry {
            target_kind,
            resulting_mode,
        });
    }
    let admission = admission_packet_for_resolved_entry(
        AdmissionSourceSurface::StartCenter,
        EntryVerb::Open,
        target_kind,
        resulting_mode,
        target_specifier,
        None,
    );
    let archetype = ArchetypeTruth::new(
        DetectionOutcome::ProbableArchetype,
        DetectionConfidenceClass::HighProbable,
        SupportClaimClass::SupportedScoped,
        DetectorState::ReadyEnough,
        vec![DetectionSignal::new(
            "signal:onboarding.local_entry.selected",
            DetectionSignalSourceClass::FilesystemLayout,
            vec![SignalMaterialEffect::RouteSelection],
            "Local entry target was selected from first-run onboarding.",
        )],
    )
    .with_detected_fact_refs(vec!["fact:onboarding.local_entry.selected".to_string()]);
    let admission_checkpoint_route = build_admission_checkpoint_route(
        AdmissionCheckpointBuildRequest::new(
            admission,
            "entry.action.onboarding.open_local_work",
            FirstUsefulEntrySource::FolderOrRepoOpen,
            archetype,
        )
        .with_trust(
            TrustState::PendingEvaluation,
            TrustReviewClass::TrustReviewPending,
        ),
    );

    Ok(BuiltOnboardingEntry {
        admission_checkpoint_route,
        import_diff_review: None,
        import_apply_review: None,
    })
}

fn build_import_entry_route(
    request: OnboardingImportFlowRequest,
    persisted_records: &mut Vec<OnboardingFlowPersistedRecord>,
) -> BuiltOnboardingEntry {
    let import_review = CompetitorConfigClassifier::new().build_review(
        Path::new(&request.source_path),
        request.destination_workspace_target.clone(),
    );
    let import_diff_review = materialize_import_diff_review_packet(&import_review);
    let import_apply_review = review_non_widening_import(&ImportApplyRequest {
        source_artifact_ref: import_diff_review.import_plan_ref.clone(),
        target_scope: request.destination_workspace_target.clone(),
        current_owner: request.current_owner,
        incoming_owner: request.incoming_owner,
        widening_vectors: request.widening_vectors,
        narrowing_only: request.narrowing_only,
    });
    persisted_records.push(OnboardingFlowPersistedRecord {
        record_kind: OnboardingFlowPersistedRecordKind::ImportedProfileHistory,
        record_ref: import_diff_review
            .retained_migration_report
            .migration_report_id
            .clone(),
        storage_lane: OnboardingStorageLane::PortableUserProfileState,
        reason: "retained import diff review and migration report".to_string(),
    });

    let admission = import_form_admission_packet(
        import_diff_review.source_path.clone(),
        import_diff_review.destination_workspace_target.clone(),
    );
    let (archetype, readiness, admission_class, ordinary_editing_available) =
        import_route_truth(&import_diff_review, &import_apply_review);
    let admission_checkpoint_route = build_admission_checkpoint_route(
        AdmissionCheckpointBuildRequest::new(
            admission,
            "entry.action.onboarding.import_profile",
            FirstUsefulEntrySource::ImportedStateOrHandoffPacket,
            archetype,
        )
        .with_readiness(readiness)
        .with_continue_without(ContinueWithoutClass::CompareBeforeRestore)
        .with_admission_class(admission_class)
        .with_availability(true, ordinary_editing_available),
    );

    BuiltOnboardingEntry {
        admission_checkpoint_route,
        import_diff_review: Some(import_diff_review),
        import_apply_review: Some(import_apply_review),
    }
}

fn import_route_truth(
    packet: &ImportDiffReviewPacket,
    import_apply_review: &ImportApplyReview,
) -> (ArchetypeTruth, ReadinessBuckets, AdmissionClass, bool) {
    if import_apply_review.allowed {
        let archetype = ArchetypeTruth::new(
            DetectionOutcome::ProbableArchetype,
            DetectionConfidenceClass::HighProbable,
            SupportClaimClass::SupportedScoped,
            DetectorState::ReadyEnough,
            vec![DetectionSignal::new(
                packet.import_diff_preview_ref.clone(),
                DetectionSignalSourceClass::ImportPacket,
                vec![
                    SignalMaterialEffect::RouteSelection,
                    SignalMaterialEffect::Readiness,
                ],
                "Import diff review and retained migration report are ready before apply.",
            )],
        )
        .with_recommendation_refs(vec![packet
            .retained_migration_report
            .migration_report_id
            .clone()]);
        let readiness = ReadinessBuckets::new().with_task(ReadinessTask::new(
            "task:onboarding.import.compare_before_apply",
            ReadinessTaskClass::ImportedStateCompare,
            ReadinessBucket::RecommendedSoon,
            ReadinessTaskState::Pending,
            ExecutionBoundary::NoExecution,
            vec![SideEffectClass::NoSideEffect],
            "Import diff review is available and protected by a rollback checkpoint.",
        ));
        (archetype, readiness, AdmissionClass::Admitted, true)
    } else {
        let archetype = ArchetypeTruth::new(
            DetectionOutcome::RestrictedOrPolicyBlocked,
            DetectionConfidenceClass::RestrictedByPolicy,
            SupportClaimClass::ClaimBlockedByPolicy,
            DetectorState::Blocked,
            vec![DetectionSignal::new(
                packet.import_diff_preview_ref.clone(),
                DetectionSignalSourceClass::ImportPacket,
                vec![
                    SignalMaterialEffect::Policy,
                    SignalMaterialEffect::Readiness,
                    SignalMaterialEffect::RouteSelection,
                ],
                "Import apply is blocked by non-widening profile review.",
            )],
        )
        .with_policy_block_refs(import_apply_review.reasons.clone());
        let readiness = ReadinessBuckets::new().with_task(
            ReadinessTask::new(
                "task:onboarding.import.non_widening_review",
                ReadinessTaskClass::ImportedStateCompare,
                ReadinessBucket::BlockingNow,
                ReadinessTaskState::BlockedByPolicy,
                ExecutionBoundary::NoExecution,
                vec![SideEffectClass::NoSideEffect],
                "Import apply waits for non-widening review repair.",
            )
            .with_blocked_reason(BlockedReasonClass::BlockedByPolicy),
        );
        (archetype, readiness, AdmissionClass::PolicyBlocked, false)
    }
}

fn rollback_checkpoint_confirmation(
    packet: Option<&ImportDiffReviewPacket>,
    review: Option<&ImportApplyReview>,
    route: &AdmissionCheckpointRouteRecord,
) -> RollbackCheckpointConfirmation {
    let admission_checkpoint_ref = route.checkpoint.admission_checkpoint_id.clone();
    let Some(packet) = packet else {
        return RollbackCheckpointConfirmation {
            confirmation_state: RollbackCheckpointConfirmationState::NotRequiredNoImport,
            checkpoint_ref: None,
            admission_checkpoint_ref,
            import_diff_preview_ref: None,
            created_before_apply: false,
            confirmed_before_apply: false,
        };
    };
    if review.is_some_and(|review| !review.allowed) {
        return RollbackCheckpointConfirmation {
            confirmation_state: RollbackCheckpointConfirmationState::BlockedByNonWideningReview,
            checkpoint_ref: Some(packet.rollback_checkpoint.checkpoint_ref.clone()),
            admission_checkpoint_ref,
            import_diff_preview_ref: Some(packet.import_diff_preview_ref.clone()),
            created_before_apply: packet.rollback_checkpoint.created_before_apply,
            confirmed_before_apply: false,
        };
    }
    let checkpoint_ready = packet.rollback_checkpoint.clear_pre_apply_checkpoint()
        && packet.every_row_uses_one_checkpoint();
    RollbackCheckpointConfirmation {
        confirmation_state: if checkpoint_ready {
            RollbackCheckpointConfirmationState::Confirmed
        } else {
            RollbackCheckpointConfirmationState::MissingCheckpoint
        },
        checkpoint_ref: Some(packet.rollback_checkpoint.checkpoint_ref.clone()),
        admission_checkpoint_ref,
        import_diff_preview_ref: Some(packet.import_diff_preview_ref.clone()),
        created_before_apply: packet.rollback_checkpoint.created_before_apply,
        confirmed_before_apply: checkpoint_ready,
    }
}

fn flow_sequence(
    import_diff_review: Option<&ImportDiffReviewPacket>,
    rollback: &RollbackCheckpointConfirmation,
    route: &AdmissionCheckpointRouteRecord,
) -> Vec<OnboardingFlowStage> {
    vec![
        OnboardingFlowStage {
            stage_kind: OnboardingFlowStageKind::IdentityModeChoice,
            required: true,
            status: "completed".to_string(),
            record_ref: Some("identity-mode-baseline:packet".to_string()),
        },
        OnboardingFlowStage {
            stage_kind: OnboardingFlowStageKind::ImportDiffReview,
            required: import_diff_review.is_some(),
            status: import_diff_review
                .map(|packet| packet.apply_gate_class.clone())
                .unwrap_or_else(|| "skipped_no_import".to_string()),
            record_ref: import_diff_review.map(|packet| packet.import_diff_preview_ref.clone()),
        },
        OnboardingFlowStage {
            stage_kind: OnboardingFlowStageKind::RollbackCheckpointConfirmation,
            required: import_diff_review.is_some(),
            status: rollback.confirmation_state.as_str().to_string(),
            record_ref: rollback.checkpoint_ref.clone(),
        },
        OnboardingFlowStage {
            stage_kind: OnboardingFlowStageKind::FirstUsefulWorkLanding,
            required: true,
            status: route
                .first_useful_route
                .route_reason_class
                .as_str()
                .to_string(),
            record_ref: Some(route.route_record_id.clone()),
        },
    ]
}

fn learning_tour_step_refs(import_requested: bool) -> Vec<String> {
    let mut refs = vec!["step:aureline.entry.open-folder"];
    if import_requested {
        refs.push("step:aureline.import.preview-before-apply");
    }
    let Ok(manifest) = current_learning_tour_alpha_manifest() else {
        return Vec::new();
    };
    refs.into_iter()
        .filter(|step_ref| manifest.step(step_ref).is_some())
        .map(str::to_string)
        .collect()
}

fn telemetry_event_names(
    import_requested: bool,
    rollback_confirmed: bool,
) -> Vec<OnboardingEventName> {
    let mut events = vec![
        OnboardingEventName::FirstRunReached,
        OnboardingEventName::FirstRunAdmitted,
    ];
    if import_requested {
        events.push(OnboardingEventName::MigrationDryRunProduced);
        if rollback_confirmed {
            events.push(OnboardingEventName::MigrationRollbackCheckpointWritten);
        }
        events.push(OnboardingEventName::MigrationOutcomeRecorded);
    }
    events.push(OnboardingEventName::FirstUsefulNavigationReached);
    events
}

/// Builds the first-run onboarding alpha projection for shell and support export.
pub fn build_onboarding_alpha_surface(
    generated_at: impl Into<String>,
) -> OnboardingAlphaSurfaceRecord {
    let registry = seeded_registry();
    let alpha_registry = alpha_command_registry();
    let docs_manifest = current_docs_pack_manifest().ok();
    let onboarding_help_manifest = current_onboarding_help_pack_alpha_manifest().ok();
    OnboardingAlphaSurfaceRecord {
        record_kind: "onboarding_alpha_surface_record".to_string(),
        onboarding_alpha_schema_version: ONBOARDING_ALPHA_SCHEMA_VERSION,
        generated_at: generated_at.into(),
        launch_context: OnboardingLaunchContext {
            surface_id: "surface:onboarding_alpha:first_run_start_center".to_string(),
            deployment_profile: "individual_local".to_string(),
            profile_scope: "profile:default_local".to_string(),
            account_prompt_class: AccountPromptClass::NoPrompt,
            account_content_secondary: true,
        },
        alpha_command_registry_ref: alpha_registry.registry_id.clone(),
        no_account_path: no_account_path(registry),
        entry_verbs: entry_verb_rows(registry),
        recommendation_cards: recommendation_cards(registry),
        teaching_cards: teaching_cards(registry),
        help_search: help_search_projection(
            registry,
            docs_manifest.as_ref(),
            onboarding_help_manifest.as_ref(),
        ),
        portable_state: portable_state_projection(),
        learning_digest: learning_digest_projection(docs_manifest.as_ref()),
        command_descriptor_round_trips: alpha_command_descriptor_round_trips(alpha_registry),
    }
}

/// Serializes the onboarding alpha projection to a JSON export file.
///
/// # Errors
///
/// Returns an error when parent directory creation, JSON serialization, or file
/// writing fails.
pub fn write_onboarding_alpha_export(
    path: impl AsRef<Path>,
    generated_at: impl Into<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let record = build_onboarding_alpha_surface(generated_at);
    let payload = serde_json::to_string_pretty(&record)?;
    std::fs::write(path, format!("{payload}\n"))?;
    Ok(())
}

fn no_account_path(registry: &CommandRegistry) -> NoAccountPathProof {
    let local_command_ids = [
        "cmd:workspace.open_folder",
        "cmd:workspace.clone_repository",
        "cmd:workspace.import_profile",
        "cmd:workspace.restore_from_checkpoint",
        "cmd:quick_open.toggle",
    ]
    .into_iter()
    .filter(|command_id| {
        registry.get(command_id).is_some() || *command_id == "cmd:quick_open.toggle"
    })
    .map(str::to_string)
    .collect();

    NoAccountPathProof {
        local_work_available: true,
        service_setup_optional: true,
        preserved_entry_verbs: vec![
            EntryVerbClass::Open,
            EntryVerbClass::Clone,
            EntryVerbClass::Import,
            EntryVerbClass::Restore,
            EntryVerbClass::RecentWork,
        ],
        local_command_ids,
    }
}

fn alpha_command_descriptor_round_trips(
    alpha_registry: &AlphaCommandRegistryRecord,
) -> Vec<OnboardingCommandDescriptorRoundTrip> {
    alpha_registry
        .claimed_commands
        .iter()
        .flat_map(|claim| claim.discoverability_record.consumer_refs.iter())
        .filter(|consumer| {
            matches!(
                consumer.consumer_class.as_str(),
                "start_center_card"
                    | "onboarding_hint"
                    | "keymap_bridge"
                    | "help_search_result"
                    | "migration_guidance"
            )
        })
        .map(|consumer| OnboardingCommandDescriptorRoundTrip {
            consumer_class: consumer.consumer_class.clone(),
            consumer_ref: consumer.consumer_ref.clone(),
            command_id: consumer.command_id.clone(),
            keyboard_route: consumer.keyboard_route.clone(),
            descriptor_anchor_ref: consumer.descriptor_anchor_ref.clone(),
            invocation_packet_ref: consumer.invocation_packet_ref.clone(),
            preserves_preview_apply_semantics: consumer.preserves_preview_apply_semantics,
            disabled_reason_mode: consumer.disabled_reason_mode.clone(),
            exact_reopen_ref: consumer.exact_reopen_ref.clone(),
        })
        .collect()
}

fn entry_verb_rows(registry: &CommandRegistry) -> Vec<OnboardingEntryVerbRow> {
    let runtime = StartCenterRuntimeInputs {
        client_scope: "desktop_product",
        workspace_trust_state: "trusted",
        execution_context_available: true,
        provider_linked: None,
        credential_available: None,
        policy_disabled: false,
        policy_blocked_in_context: false,
        labs_enabled: false,
    };
    let mut rows = build_action_rows(registry, runtime)
        .into_iter()
        .map(|row| {
            let command_id = row.command_id.to_string();
            OnboardingEntryVerbRow {
                entry_verb_class: entry_verb_for_action(row.action_id),
                primary_action_token: Some(row.action_id.token().to_string()),
                label: row.title.to_string(),
                command: OnboardingCommandAnchor::registry(command_id, registry),
                no_account_allowed: true,
                review_required_before_mutation: matches!(
                    row.action_id,
                    StartCenterPrimaryActionId::CloneRepository
                        | StartCenterPrimaryActionId::RestoreLastSession
                        | StartCenterPrimaryActionId::ImportFrom
                ),
                setup_or_trust_deferred: matches!(
                    row.action_id,
                    StartCenterPrimaryActionId::CloneRepository
                        | StartCenterPrimaryActionId::ImportFrom
                ),
                preflight_decision_class: row
                    .preflight
                    .map(|decision| preflight_decision_class_token(decision.decision_class)),
            }
        })
        .collect::<Vec<_>>();

    rows.push(OnboardingEntryVerbRow {
        entry_verb_class: EntryVerbClass::RecentWork,
        primary_action_token: None,
        label: "Recent work".to_string(),
        command: OnboardingCommandAnchor::registry("cmd:quick_open.toggle", registry),
        no_account_allowed: true,
        review_required_before_mutation: true,
        setup_or_trust_deferred: true,
        preflight_decision_class: Some("allowed".to_string()),
    });
    rows
}

fn entry_verb_for_action(action_id: StartCenterPrimaryActionId) -> EntryVerbClass {
    match action_id {
        StartCenterPrimaryActionId::OpenFolder | StartCenterPrimaryActionId::OpenWorkspace => {
            EntryVerbClass::Open
        }
        StartCenterPrimaryActionId::CloneRepository => EntryVerbClass::Clone,
        StartCenterPrimaryActionId::RestoreLastSession => EntryVerbClass::Restore,
        StartCenterPrimaryActionId::ImportFrom => EntryVerbClass::Import,
    }
}

fn recommendation_cards(registry: &CommandRegistry) -> Vec<OnboardingRecommendationCard> {
    vec![OnboardingRecommendationCard {
        card_id: "onboarding.recommendation.launch_bundle.typescript_web".to_string(),
        recommendation_source_class: RecommendationSourceClass::LaunchBundle,
        recommendation_ref: "launch_bundle:typescript_web_app.seed".to_string(),
        help_search_command: OnboardingCommandAnchor::alpha_owned(
            "cmd:help.search",
            "Cmd/Ctrl+Shift+H",
            "docs:anchor:onboarding_alpha:help_search",
        ),
        keymap_bridge_refs: vec![
            "keymap_bridge:vs_code:command_palette.open".to_string(),
            "keymap_bridge:vs_code:workspace.open_folder".to_string(),
        ],
        actions: vec![
            RecommendationAction {
                action_class: RecommendationActionClass::Apply,
                command: OnboardingCommandAnchor::alpha_owned(
                    "cmd:onboarding.apply_recommendation",
                    "Enter after review",
                    "docs:anchor:onboarding_alpha:apply_recommendation",
                ),
                review_required_before_effect: true,
            },
            RecommendationAction {
                action_class: RecommendationActionClass::Compare,
                command: OnboardingCommandAnchor::alpha_owned(
                    "cmd:onboarding.compare_recommendation",
                    "Space",
                    "docs:anchor:onboarding_alpha:compare_recommendation",
                ),
                review_required_before_effect: false,
            },
            RecommendationAction {
                action_class: RecommendationActionClass::Dismiss,
                command: OnboardingCommandAnchor::alpha_owned(
                    "cmd:onboarding.dismiss_recommendation",
                    "Delete",
                    "docs:anchor:onboarding_alpha:dismiss_recommendation",
                ),
                review_required_before_effect: false,
            },
            RecommendationAction {
                action_class: RecommendationActionClass::OpenMinimal,
                command: OnboardingCommandAnchor::registry("cmd:workspace.open_folder", registry),
                review_required_before_effect: false,
            },
            RecommendationAction {
                action_class: RecommendationActionClass::SetUpLater,
                command: OnboardingCommandAnchor::alpha_owned(
                    "cmd:onboarding.set_up_later",
                    "Cmd/Ctrl+L",
                    "docs:anchor:onboarding_alpha:set_up_later",
                ),
                review_required_before_effect: false,
            },
        ],
        remembered_choice_effect: RememberedChoiceEffect::PreferenceOnlyNoAuthorityChange,
        review_required_on_later_open: true,
        can_silently_install: false,
        can_silently_widen_trust: false,
    }]
}

fn teaching_cards(registry: &CommandRegistry) -> Vec<OnboardingTeachingCard> {
    vec![
        OnboardingTeachingCard {
            card_id: "onboarding.card.open_local_folder".to_string(),
            card_kind: TeachingCardKind::OnboardingCard,
            command: OnboardingCommandAnchor::registry("cmd:workspace.open_folder", registry),
            bridge_refs: Vec::new(),
            citation_refs: vec!["docs:anchor:onboarding_alpha:open_folder".to_string()],
            dismissal_state_item_ref: "state_item.onboarding.dismissed.open_local_folder"
                .to_string(),
            hidden_mutation_allowed: false,
        },
        OnboardingTeachingCard {
            card_id: "onboarding.card.import_keymap_bridge".to_string(),
            card_kind: TeachingCardKind::KeymapBridge,
            command: OnboardingCommandAnchor::registry("cmd:command_palette.open", registry),
            bridge_refs: vec!["keymap_bridge:vs_code:command_palette.open".to_string()],
            citation_refs: vec!["docs:anchor:onboarding_alpha:keymap_bridge".to_string()],
            dismissal_state_item_ref: "state_item.onboarding.dismissed.keymap_bridge".to_string(),
            hidden_mutation_allowed: false,
        },
        OnboardingTeachingCard {
            card_id: "onboarding.card.migration_profile_history".to_string(),
            card_kind: TeachingCardKind::MigrationHint,
            command: OnboardingCommandAnchor::registry("cmd:workspace.import_profile", registry),
            bridge_refs: vec!["migration_bridge:vscode_profile:shortcut_delta".to_string()],
            citation_refs: vec!["docs:anchor:onboarding_alpha:import_profile".to_string()],
            dismissal_state_item_ref: "state_item.onboarding.dismissed.import_profile".to_string(),
            hidden_mutation_allowed: false,
        },
        OnboardingTeachingCard {
            card_id: "onboarding.card.contextual_tip.restore".to_string(),
            card_kind: TeachingCardKind::ContextualTip,
            command: OnboardingCommandAnchor::registry(
                "cmd:workspace.restore_from_checkpoint",
                registry,
            ),
            bridge_refs: Vec::new(),
            citation_refs: vec!["docs:anchor:onboarding_alpha:restore_truth".to_string()],
            dismissal_state_item_ref: "state_item.onboarding.dismissed.restore_tip".to_string(),
            hidden_mutation_allowed: false,
        },
    ]
}

fn help_search_projection(
    registry: &CommandRegistry,
    docs_manifest: Option<&DocsPackAlphaManifest>,
    onboarding_help_manifest: Option<&OnboardingHelpPackAlphaManifest>,
) -> OnboardingHelpSearchProjection {
    if let Some(projection) = onboarding_help_pack_projection(registry, onboarding_help_manifest) {
        return projection;
    }

    if let Some(projection) = manifest_help_search_projection(registry, docs_manifest) {
        return projection;
    }

    OnboardingHelpSearchProjection {
        projection_id: "discoverability:onboarding_alpha:first_run".to_string(),
        help_search_command: OnboardingCommandAnchor::alpha_owned(
            "cmd:help.search",
            "Cmd/Ctrl+Shift+H",
            "docs:anchor:onboarding_alpha:help_search",
        ),
        pack_states: vec![
            OnboardingPackState {
                pack_id: "pack:onboarding_alpha:local_starter".to_string(),
                pack_role: PackRole::FirstRunStarterPack,
                source_version_ref: "build:local:onboarding-alpha:1".to_string(),
                install_state: PackInstallState::LocalOnlyStarter,
                locale_availability: LocaleAvailability::LocaleAvailableReviewed,
                offline_posture: OfflinePosture::FullyAvailableOfflineLocalBuild,
                citations_exportable: true,
            },
            OnboardingPackState {
                pack_id: "pack:onboarding_alpha:migration_cached".to_string(),
                pack_role: PackRole::MigrationWelcomePack,
                source_version_ref: "docs-pack:migration-alpha:2026-05-13".to_string(),
                install_state: PackInstallState::CachedSnapshotCurrent,
                locale_availability: LocaleAvailability::LocaleMissingFallbackToPrimary,
                offline_posture: OfflinePosture::CachedSnapshotOffline,
                citations_exportable: true,
            },
            OnboardingPackState {
                pack_id: "pack:onboarding_alpha:learning_digest".to_string(),
                pack_role: PackRole::GuidedContentPack,
                source_version_ref: "docs-pack:learning-digest:not-installed".to_string(),
                install_state: PackInstallState::NotInstalled,
                locale_availability: LocaleAvailability::LocaleMissingNotInstalled,
                offline_posture: OfflinePosture::NotAvailableOffline,
                citations_exportable: false,
            },
        ],
        items: vec![
            OnboardingHelpSearchItem {
                item_id: "help:onboarding_alpha:open_folder".to_string(),
                pack_id: "pack:onboarding_alpha:local_starter".to_string(),
                surface_class: HelpSurfaceClass::HelpSearch,
                command: OnboardingCommandAnchor::registry("cmd:workspace.open_folder", registry),
                requested_locale: "en-US".to_string(),
                effective_locale: "en-US".to_string(),
                source_language_fallback_class:
                    SourceLanguageFallbackClass::NoFallbackPrimaryLocaleOnly,
                pack_install_state: PackInstallState::LocalOnlyStarter,
                citation_refs: vec!["docs:anchor:onboarding_alpha:open_folder".to_string()],
                docs_node_id: None,
                source_pack_revision_ref: None,
                source_strip_ref: None,
                citation_drawer_ref: None,
                exact_reopen_ref: None,
            },
            OnboardingHelpSearchItem {
                item_id: "help:onboarding_alpha:keymap_source_language_fallback".to_string(),
                pack_id: "pack:onboarding_alpha:migration_cached".to_string(),
                surface_class: HelpSurfaceClass::SourceLanguageFallback,
                command: OnboardingCommandAnchor::registry("cmd:command_palette.open", registry),
                requested_locale: "es-MX".to_string(),
                effective_locale: "en-US".to_string(),
                source_language_fallback_class:
                    SourceLanguageFallbackClass::FallbackToSourceLanguageDisclosed,
                pack_install_state: PackInstallState::CachedSnapshotCurrent,
                citation_refs: vec![
                    "docs:anchor:onboarding_alpha:keymap_bridge".to_string(),
                    "docs:anchor:onboarding_alpha:source_language_fallback".to_string(),
                ],
                docs_node_id: None,
                source_pack_revision_ref: None,
                source_strip_ref: None,
                citation_drawer_ref: None,
                exact_reopen_ref: None,
            },
            OnboardingHelpSearchItem {
                item_id: "help:onboarding_alpha:learning_digest_not_installed".to_string(),
                pack_id: "pack:onboarding_alpha:learning_digest".to_string(),
                surface_class: HelpSurfaceClass::ContextualTip,
                command: OnboardingCommandAnchor::alpha_owned(
                    "cmd:help.search",
                    "Cmd/Ctrl+Shift+H",
                    "docs:anchor:onboarding_alpha:learning_digest_not_installed",
                ),
                requested_locale: "en-US".to_string(),
                effective_locale: "en-US".to_string(),
                source_language_fallback_class:
                    SourceLanguageFallbackClass::FallbackBlockedPackMissing,
                pack_install_state: PackInstallState::NotInstalled,
                citation_refs: Vec::new(),
                docs_node_id: None,
                source_pack_revision_ref: None,
                source_strip_ref: None,
                citation_drawer_ref: None,
                exact_reopen_ref: None,
            },
        ],
        support_export_reconstructable: true,
    }
}

fn onboarding_help_pack_projection(
    registry: &CommandRegistry,
    manifest: Option<&OnboardingHelpPackAlphaManifest>,
) -> Option<OnboardingHelpSearchProjection> {
    let manifest = manifest?;
    let local_item = manifest.item("ohp:item:first_run.open_folder")?;
    let fallback_item = manifest.item("ohp:item:keymap_bridge.command_palette")?;
    let glossary_item = manifest.item("ohp:item:glossary.workset")?;
    let missing_item = manifest.item("ohp:item:learning_digest.not_installed")?;

    let pack_states = [local_item, fallback_item, glossary_item, missing_item]
        .into_iter()
        .map(|item| onboarding_pack_state_from_help_pack_item(manifest, item))
        .collect::<Option<Vec<_>>>()?;

    Some(OnboardingHelpSearchProjection {
        projection_id: "discoverability:onboarding_help_pack_alpha:first_run".to_string(),
        help_search_command: OnboardingCommandAnchor::alpha_owned(
            "cmd:help.search",
            "Cmd/Ctrl+Shift+H",
            "docs:anchor:onboarding_alpha:help_search",
        ),
        pack_states,
        items: vec![
            onboarding_help_item_from_help_pack_item(
                local_item,
                registry,
                HelpSurfaceClass::HelpSearch,
            ),
            onboarding_help_item_from_help_pack_item(
                fallback_item,
                registry,
                HelpSurfaceClass::SourceLanguageFallback,
            ),
            onboarding_help_item_from_help_pack_item(
                glossary_item,
                registry,
                HelpSurfaceClass::HelpSearch,
            ),
            onboarding_help_item_from_help_pack_item(
                missing_item,
                registry,
                HelpSurfaceClass::ContextualTip,
            ),
        ],
        support_export_reconstructable: true,
    })
}

fn onboarding_pack_state_from_help_pack_item(
    manifest: &OnboardingHelpPackAlphaManifest,
    item: &OnboardingHelpPackAlphaItem,
) -> Option<OnboardingPackState> {
    let pack = manifest.pack(&item.pack_id)?;
    Some(OnboardingPackState {
        pack_id: pack.pack_id.clone(),
        pack_role: match item.item_kind {
            OnboardingHelpPackItemKind::StartCenterCard => PackRole::FirstRunStarterPack,
            OnboardingHelpPackItemKind::KeymapBridge
            | OnboardingHelpPackItemKind::MigrationGuidance
            | OnboardingHelpPackItemKind::ContextualHint => PackRole::MigrationWelcomePack,
            OnboardingHelpPackItemKind::GlossaryPackItem => PackRole::GlossaryBundle,
            OnboardingHelpPackItemKind::FutureGuidedLearningRef => PackRole::GuidedContentPack,
        },
        source_version_ref: pack.pack_revision_ref.clone(),
        install_state: pack_install_state_from_help_pack(pack.install_state),
        locale_availability: locale_availability_from_help_pack_item(item),
        offline_posture: offline_posture_from_help_pack(pack.offline_posture),
        citations_exportable: item.citation.availability
            == crate::help::onboarding_help_pack::OnboardingHelpPackCitationAvailability::Available
            && !item.citation.citation_refs.is_empty(),
    })
}

fn onboarding_help_item_from_help_pack_item(
    item: &OnboardingHelpPackAlphaItem,
    registry: &CommandRegistry,
    surface_class: HelpSurfaceClass,
) -> OnboardingHelpSearchItem {
    OnboardingHelpSearchItem {
        item_id: item.item_id.clone(),
        pack_id: item.pack_id.clone(),
        surface_class,
        command: command_anchor_from_help_pack_item(item, registry),
        requested_locale: item.locale.requested_locale.clone(),
        effective_locale: item.locale.effective_locale.clone(),
        source_language_fallback_class: source_language_fallback_from_help_pack_item(item),
        pack_install_state: pack_install_state_from_help_pack_item(item),
        citation_refs: item.citation.citation_refs.clone(),
        docs_node_id: Some(item.docs_node_id.clone()),
        source_pack_revision_ref: Some(item.pack_revision_ref.clone()),
        source_strip_ref: item.citation.source_strip_ref.clone(),
        citation_drawer_ref: item.citation.citation_drawer_ref.clone(),
        exact_reopen_ref: Some(item.exact_reopen_ref.clone()),
    }
}

fn command_anchor_from_help_pack_item(
    item: &OnboardingHelpPackAlphaItem,
    registry: &CommandRegistry,
) -> OnboardingCommandAnchor {
    let entry = registry.get(&item.command_hint.command_id);
    OnboardingCommandAnchor {
        command_id: item.command_hint.command_id.clone(),
        keyboard_route: item.command_hint.keyboard_route.clone(),
        anchor_source: match item.command_hint.metadata_source {
            OnboardingHelpPackCommandMetadataSource::CommandRegistry if entry.is_some() => {
                CommandAnchorSource::SeededCommandRegistry
            }
            _ => CommandAnchorSource::OnboardingAlphaOwned,
        },
        registry_entry_id: entry.map(|entry| entry.registry_entry_id.clone()),
        docs_anchor_ref: Some(item.command_hint.help_anchor_id.clone()),
    }
}

fn pack_install_state_from_help_pack(
    install_state: OnboardingHelpPackInstallState,
) -> PackInstallState {
    match install_state {
        OnboardingHelpPackInstallState::LocalOnlyStarter => PackInstallState::LocalOnlyStarter,
        OnboardingHelpPackInstallState::CachedSnapshotCurrent
        | OnboardingHelpPackInstallState::CachedSnapshotStale
        | OnboardingHelpPackInstallState::MirrorOnlyVerified => {
            PackInstallState::CachedSnapshotCurrent
        }
        OnboardingHelpPackInstallState::NotInstalled => PackInstallState::NotInstalled,
    }
}

fn pack_install_state_from_help_pack_item(item: &OnboardingHelpPackAlphaItem) -> PackInstallState {
    match item.content_render_state {
        crate::help::onboarding_help_pack::OnboardingHelpPackRenderState::BlockedNotInstalled => {
            PackInstallState::NotInstalled
        }
        _ => match item.pack_id.as_str() {
            "pack:onboarding-help:first-run-alpha" | "pack:onboarding-help:glossary-alpha" => {
                PackInstallState::LocalOnlyStarter
            }
            _ => PackInstallState::CachedSnapshotCurrent,
        },
    }
}

fn locale_availability_from_help_pack_item(
    item: &OnboardingHelpPackAlphaItem,
) -> LocaleAvailability {
    match item.locale.locale_availability {
        OnboardingHelpPackLocaleAvailability::LocaleAvailableReviewed => {
            LocaleAvailability::LocaleAvailableReviewed
        }
        OnboardingHelpPackLocaleAvailability::LocaleAvailableStaleCopy
        | OnboardingHelpPackLocaleAvailability::LocaleMissingFallbackToPrimary => {
            LocaleAvailability::LocaleMissingFallbackToPrimary
        }
        OnboardingHelpPackLocaleAvailability::LocaleMissingNotInstalled => {
            LocaleAvailability::LocaleMissingNotInstalled
        }
    }
}

fn offline_posture_from_help_pack(
    offline_posture: OnboardingHelpPackOfflinePosture,
) -> OfflinePosture {
    match offline_posture {
        OnboardingHelpPackOfflinePosture::FullyAvailableOfflineLocalBuild => {
            OfflinePosture::FullyAvailableOfflineLocalBuild
        }
        OnboardingHelpPackOfflinePosture::CachedSnapshotOffline
        | OnboardingHelpPackOfflinePosture::MirrorVerifiedOffline => {
            OfflinePosture::CachedSnapshotOffline
        }
        OnboardingHelpPackOfflinePosture::NotAvailableOffline => {
            OfflinePosture::NotAvailableOffline
        }
    }
}

fn source_language_fallback_from_help_pack_item(
    item: &OnboardingHelpPackAlphaItem,
) -> SourceLanguageFallbackClass {
    match item.source_language_fallback.fallback_class {
        OnboardingHelpPackFallbackClass::NoFallbackPrimaryLocaleOnly => {
            SourceLanguageFallbackClass::NoFallbackPrimaryLocaleOnly
        }
        OnboardingHelpPackFallbackClass::FallbackToSourceLanguageDisclosed => {
            SourceLanguageFallbackClass::FallbackToSourceLanguageDisclosed
        }
        OnboardingHelpPackFallbackClass::FallbackBlockedPackMissing => {
            SourceLanguageFallbackClass::FallbackBlockedPackMissing
        }
    }
}

fn manifest_help_search_projection(
    registry: &CommandRegistry,
    docs_manifest: Option<&DocsPackAlphaManifest>,
) -> Option<OnboardingHelpSearchProjection> {
    let manifest = docs_manifest?;
    let local_node = manifest.node("docs-node:project-entry.open-folder")?;
    let fallback_node = manifest.node("docs-node:onboarding.keymap-bridge")?;
    let missing_node = manifest.node("docs-node:onboarding.deep-dive.not-installed")?;

    Some(OnboardingHelpSearchProjection {
        projection_id: "discoverability:onboarding_alpha:first_run".to_string(),
        help_search_command: OnboardingCommandAnchor::alpha_owned(
            "cmd:help.search",
            "Cmd/Ctrl+Shift+H",
            "docs:anchor:onboarding_alpha:help_search",
        ),
        pack_states: vec![
            onboarding_pack_state_from_node(local_node, PackRole::FirstRunStarterPack),
            onboarding_pack_state_from_node(fallback_node, PackRole::MigrationWelcomePack),
            onboarding_pack_state_from_node(missing_node, PackRole::GuidedContentPack),
        ],
        items: vec![
            onboarding_help_item_from_node(
                local_node,
                registry,
                "help:onboarding_alpha:open_folder",
                HelpSurfaceClass::HelpSearch,
            ),
            onboarding_help_item_from_node(
                fallback_node,
                registry,
                "help:onboarding_alpha:keymap_source_language_fallback",
                HelpSurfaceClass::SourceLanguageFallback,
            ),
            onboarding_help_item_from_node(
                missing_node,
                registry,
                "help:onboarding_alpha:learning_digest_not_installed",
                HelpSurfaceClass::ContextualTip,
            ),
        ],
        support_export_reconstructable: true,
    })
}

fn onboarding_pack_state_from_node(
    node: &DocsPackNode,
    pack_role: PackRole,
) -> OnboardingPackState {
    OnboardingPackState {
        pack_id: node.source_pack_ref.clone(),
        pack_role,
        source_version_ref: node.source_pack_revision_ref.clone(),
        install_state: pack_install_state_from_docs_node(node),
        locale_availability: locale_availability_from_docs_node(node),
        offline_posture: offline_posture_from_docs_node(node),
        citations_exportable: node.has_citation_anchor(),
    }
}

fn onboarding_help_item_from_node(
    node: &DocsPackNode,
    registry: &CommandRegistry,
    item_id: &str,
    surface_class: HelpSurfaceClass,
) -> OnboardingHelpSearchItem {
    OnboardingHelpSearchItem {
        item_id: item_id.to_string(),
        pack_id: node.source_pack_ref.clone(),
        surface_class,
        command: command_anchor_from_docs_node(node, registry),
        requested_locale: node.requested_locale.clone(),
        effective_locale: node.effective_locale.clone(),
        source_language_fallback_class: source_language_fallback_from_docs_node(node),
        pack_install_state: pack_install_state_from_docs_node(node),
        citation_refs: node.citation_anchor_refs.clone(),
        docs_node_id: Some(node.doc_node_id.clone()),
        source_pack_revision_ref: Some(node.source_pack_revision_ref.clone()),
        source_strip_ref: Some(node.source_strip_ref.clone()),
        citation_drawer_ref: Some(node.citation_drawer_ref.clone()),
        exact_reopen_ref: Some(node.exact_reopen_ref.clone()),
    }
}

fn command_anchor_from_docs_node(
    node: &DocsPackNode,
    registry: &CommandRegistry,
) -> OnboardingCommandAnchor {
    let command_id = node.command_id.as_deref().unwrap_or("cmd:help.search");
    if registry.get(command_id).is_some() {
        return OnboardingCommandAnchor::registry(command_id, registry);
    }

    OnboardingCommandAnchor::alpha_owned(
        command_id,
        "Cmd/Ctrl+Shift+H",
        node.help_anchor_id
            .clone()
            .unwrap_or_else(|| format!("docs:anchor:{}", node.doc_node_id)),
    )
}

fn pack_install_state_from_docs_node(node: &DocsPackNode) -> PackInstallState {
    match (node.locality_state, node.install_state) {
        (DocsPackLocalityState::NotInstalled, _) | (_, DocsPackInstallState::NotInstalled) => {
            PackInstallState::NotInstalled
        }
        (DocsPackLocalityState::WarmLocalCache, _)
        | (DocsPackLocalityState::MirrorOnly, _)
        | (_, DocsPackInstallState::CachedOnly)
        | (_, DocsPackInstallState::MirrorOnlyVerified) => PackInstallState::CachedSnapshotCurrent,
        _ => PackInstallState::LocalOnlyStarter,
    }
}

fn locale_availability_from_docs_node(node: &DocsPackNode) -> LocaleAvailability {
    match node.locale_availability {
        DocsPackLocaleAvailability::RequestedLocaleAuthoritative => {
            LocaleAvailability::LocaleAvailableReviewed
        }
        DocsPackLocaleAvailability::RequestedLocalePartial
        | DocsPackLocaleAvailability::RequestedLocaleMissingFallbackToPrimary => {
            LocaleAvailability::LocaleMissingFallbackToPrimary
        }
        DocsPackLocaleAvailability::RequestedLocaleNotInstalled => {
            LocaleAvailability::LocaleMissingNotInstalled
        }
    }
}

fn offline_posture_from_docs_node(node: &DocsPackNode) -> OfflinePosture {
    match (node.locality_state, node.install_state) {
        (DocsPackLocalityState::NotInstalled, _) | (_, DocsPackInstallState::NotInstalled) => {
            OfflinePosture::NotAvailableOffline
        }
        (DocsPackLocalityState::WarmLocalCache, _)
        | (DocsPackLocalityState::MirrorOnly, _)
        | (_, DocsPackInstallState::CachedOnly)
        | (_, DocsPackInstallState::MirrorOnlyVerified) => OfflinePosture::CachedSnapshotOffline,
        _ => OfflinePosture::FullyAvailableOfflineLocalBuild,
    }
}

fn source_language_fallback_from_docs_node(node: &DocsPackNode) -> SourceLanguageFallbackClass {
    match node.locale_availability {
        DocsPackLocaleAvailability::RequestedLocaleAuthoritative => {
            SourceLanguageFallbackClass::NoFallbackPrimaryLocaleOnly
        }
        DocsPackLocaleAvailability::RequestedLocalePartial
        | DocsPackLocaleAvailability::RequestedLocaleMissingFallbackToPrimary => {
            SourceLanguageFallbackClass::FallbackToSourceLanguageDisclosed
        }
        DocsPackLocaleAvailability::RequestedLocaleNotInstalled => {
            SourceLanguageFallbackClass::FallbackBlockedPackMissing
        }
    }
}

fn portable_state_projection() -> OnboardingPortableStateProjection {
    OnboardingPortableStateProjection {
        bundle_id: "onboarding_state_bundle:alpha_profile".to_string(),
        items: vec![
            state_item(
                "state_item.onboarding.dismissed.open_local_folder",
                OnboardingStateKind::DismissedHint,
                "Dismissal follows the portable profile so the same hint does not reappear as hidden workspace state.",
            ),
            state_item(
                "state_item.onboarding.completed.open_folder",
                OnboardingStateKind::CompletedTask,
                "First useful local open completion is profile-owned and exportable.",
            ),
            state_item(
                "state_item.onboarding.deferred.bundle_setup",
                OnboardingStateKind::DeferredSetup,
                "Deferred setup restores only the user preference and not authority.",
            ),
            state_item(
                "state_item.onboarding.recovery.import_rollback",
                OnboardingStateKind::ProtectedRecoveryRecommendation,
                "Recovery recommendations remain inspectable after import or restore review.",
            ),
            state_item(
                "state_item.onboarding.imported_profile.vscode",
                OnboardingStateKind::ImportedProfileHistory,
                "Imported profile history is portable profile state with rollback evidence refs.",
            ),
        ],
        any_workspace_local_hidden_store: false,
        portable_profile_exportable: true,
    }
}

fn state_item(
    state_item_id: impl Into<String>,
    state_kind: OnboardingStateKind,
    portability_reason: impl Into<String>,
) -> OnboardingStateItem {
    OnboardingStateItem {
        state_item_id: state_item_id.into(),
        state_kind,
        storage_lane: OnboardingStorageLane::PortableUserProfileState,
        profile_scope_class: ProfileScopeClass::PerProfile,
        reset_class: OnboardingResetClass::ResettablePerProfile,
        export_class: if state_kind == OnboardingStateKind::ProtectedRecoveryRecommendation {
            OnboardingExportClass::InSupportBundleRedacted
        } else {
            OnboardingExportClass::InPortableProfilePackage
        },
        portability_reason: portability_reason.into(),
    }
}

fn learning_digest_projection(
    docs_manifest: Option<&DocsPackAlphaManifest>,
) -> LearningDigestProjection {
    let missing_node = docs_manifest
        .and_then(|manifest| manifest.node("docs-node:onboarding.deep-dive.not-installed"));
    LearningDigestProjection {
        digest_ref: "learning_digest:onboarding_alpha:not_installed".to_string(),
        availability_class: LearningDigestAvailability::NotInstalledPlaceholder,
        open_or_placeholder_command: missing_node
            .map(|node| {
                OnboardingCommandAnchor::alpha_owned(
                    node.command_id.as_deref().unwrap_or("cmd:help.search"),
                    "Cmd/Ctrl+Shift+H",
                    node.help_anchor_id.clone().unwrap_or_else(|| {
                        "docs:anchor:onboarding_alpha:learning_digest_not_installed".to_string()
                    }),
                )
            })
            .unwrap_or_else(|| {
                OnboardingCommandAnchor::alpha_owned(
                    "cmd:help.search",
                    "Cmd/Ctrl+Shift+H",
                    "docs:anchor:onboarding_alpha:learning_digest_not_installed",
                )
            }),
        pack_id: missing_node
            .map(|node| node.source_pack_ref.clone())
            .unwrap_or_else(|| "pack:onboarding_alpha:learning_digest".to_string()),
        no_account_continuity_preserved: true,
        exact_reopen_preserves_anchors: true,
    }
}

fn keyboard_route_for(command_id: &str) -> String {
    if let Ok(rows) = preset_binding_rows(KeymapPresetId::VsCode, PlatformClass::Macos) {
        if let Some(row) = rows.iter().find(|row| row.command_id == command_id) {
            return row.literal_sequence.clone();
        }
    }
    "Command Palette search".to_string()
}

fn preflight_decision_class_token(decision_class: PreflightDecisionClass) -> String {
    match decision_class {
        PreflightDecisionClass::Allowed => "allowed",
        PreflightDecisionClass::BlockedByPolicy => "blocked_by_policy",
        PreflightDecisionClass::DisabledWithReason => "disabled_with_reason",
        PreflightDecisionClass::PreviewRequired => "preview_required",
        PreflightDecisionClass::ApprovalRequired => "approval_required",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_run_entry_verbs_are_distinct_and_no_account() {
        let surface = build_onboarding_alpha_surface(ONBOARDING_ALPHA_FIXTURE_GENERATED_AT);
        let verbs = surface
            .entry_verbs
            .iter()
            .map(|row| row.entry_verb_class)
            .collect::<Vec<_>>();
        assert!(verbs.contains(&EntryVerbClass::Open));
        assert!(verbs.contains(&EntryVerbClass::Clone));
        assert!(verbs.contains(&EntryVerbClass::Import));
        assert!(verbs.contains(&EntryVerbClass::Restore));
        assert!(verbs.contains(&EntryVerbClass::RecentWork));
        assert!(surface.no_account_path.local_work_available);
        assert!(surface.entry_verbs.iter().all(|row| row.no_account_allowed));
    }

    #[test]
    fn recommendation_actions_do_not_restore_hidden_authority() {
        let surface = build_onboarding_alpha_surface(ONBOARDING_ALPHA_FIXTURE_GENERATED_AT);
        let card = surface
            .recommendation_cards
            .iter()
            .find(|card| card.card_id == "onboarding.recommendation.launch_bundle.typescript_web")
            .expect("launch recommendation card");
        let actions = card
            .actions
            .iter()
            .map(|action| action.action_class)
            .collect::<Vec<_>>();
        assert_eq!(
            actions,
            vec![
                RecommendationActionClass::Apply,
                RecommendationActionClass::Compare,
                RecommendationActionClass::Dismiss,
                RecommendationActionClass::OpenMinimal,
                RecommendationActionClass::SetUpLater,
            ]
        );
        assert_eq!(
            card.remembered_choice_effect,
            RememberedChoiceEffect::PreferenceOnlyNoAuthorityChange
        );
        assert!(card.review_required_on_later_open);
        assert!(!card.can_silently_install);
        assert!(!card.can_silently_widen_trust);
    }

    #[test]
    fn help_search_preserves_locale_fallback_and_pack_posture() {
        let surface = build_onboarding_alpha_surface(ONBOARDING_ALPHA_FIXTURE_GENERATED_AT);
        assert!(surface
            .help_search
            .pack_states
            .iter()
            .any(|pack| pack.install_state == PackInstallState::LocalOnlyStarter));
        assert!(surface
            .help_search
            .pack_states
            .iter()
            .any(|pack| pack.install_state == PackInstallState::CachedSnapshotCurrent));
        assert!(surface
            .help_search
            .pack_states
            .iter()
            .any(|pack| pack.install_state == PackInstallState::NotInstalled));
        let fallback = surface
            .help_search
            .items
            .iter()
            .find(|item| {
                item.source_language_fallback_class
                    == SourceLanguageFallbackClass::FallbackToSourceLanguageDisclosed
            })
            .expect("source-language fallback item");
        assert_eq!(fallback.requested_locale, "es-MX");
        assert_eq!(fallback.effective_locale, "en-US");
        assert!(!fallback.citation_refs.is_empty());
        assert!(fallback.command.command_id.starts_with("cmd:"));
    }

    #[test]
    fn portable_state_keeps_distinct_progress_and_recovery_meaning() {
        let surface = build_onboarding_alpha_surface(ONBOARDING_ALPHA_FIXTURE_GENERATED_AT);
        let kinds = surface
            .portable_state
            .items
            .iter()
            .map(|item| item.state_kind)
            .collect::<Vec<_>>();
        assert!(kinds.contains(&OnboardingStateKind::DismissedHint));
        assert!(kinds.contains(&OnboardingStateKind::CompletedTask));
        assert!(kinds.contains(&OnboardingStateKind::DeferredSetup));
        assert!(kinds.contains(&OnboardingStateKind::ProtectedRecoveryRecommendation));
        assert!(kinds.contains(&OnboardingStateKind::ImportedProfileHistory));
        assert!(surface
            .portable_state
            .items
            .iter()
            .all(|item| item.storage_lane == OnboardingStorageLane::PortableUserProfileState));
        assert!(!surface.portable_state.any_workspace_local_hidden_store);
    }

    #[test]
    fn learning_digest_placeholder_is_truthful_and_local() {
        let surface = build_onboarding_alpha_surface(ONBOARDING_ALPHA_FIXTURE_GENERATED_AT);
        assert_eq!(
            surface.learning_digest.availability_class,
            LearningDigestAvailability::NotInstalledPlaceholder
        );
        assert!(surface.learning_digest.no_account_continuity_preserved);
        assert!(surface.learning_digest.exact_reopen_preserves_anchors);
        assert_eq!(
            surface
                .learning_digest
                .open_or_placeholder_command
                .command_id,
            "cmd:help.search"
        );
    }

    #[test]
    fn alpha_command_registry_round_trips_cover_onboarding_consumers() {
        use std::collections::BTreeSet;

        let surface = build_onboarding_alpha_surface(ONBOARDING_ALPHA_FIXTURE_GENERATED_AT);
        assert_eq!(
            surface.alpha_command_registry_ref,
            "command-registry:alpha:launch-wedge:01"
        );
        let consumer_classes = surface
            .command_descriptor_round_trips
            .iter()
            .map(|row| row.consumer_class.as_str())
            .collect::<BTreeSet<_>>();
        for required in [
            "start_center_card",
            "onboarding_hint",
            "keymap_bridge",
            "help_search_result",
            "migration_guidance",
        ] {
            assert!(consumer_classes.contains(required), "missing {required}");
        }
        for row in &surface.command_descriptor_round_trips {
            assert!(row.command_id.starts_with("cmd:"));
            assert!(row.preserves_preview_apply_semantics);
            assert_eq!(
                row.disabled_reason_mode,
                "typed_reason_required_when_unavailable"
            );
            assert!(row.exact_reopen_ref.is_some());
        }
    }
}
