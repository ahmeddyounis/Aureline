// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

//! Staged save coordinator.
//!
//! The coordinator is the single entry point for committing buffer snapshots to
//! durable storage. It enforces compare-before-write, runs save participants on
//! staged content, and selects an atomic (or explicitly degraded) write lane
//! based on the VFS root capability envelope.

use std::collections::HashSet;
use std::mem;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use aureline_vfs::{
    AtomicWriteMode, ConditionalWriteOutcome, GenerationToken, ReviewedInPlaceSave,
    RootPostCommitObservation, RootWriteCondition, SaveManifest, SaveOutcome, SaveTargetToken,
    VfsRoot,
};

use super::drift_detection::{detect_external_drift, detect_root_ownership};
use super::risk::{
    summarize_staged_file_effect, FileEffectSummary, SaveParticipantClass,
    SaveParticipantEffectRecordOutcome, SaveParticipantFixSafetyClass, SaveParticipantOutputOrigin,
    SaveParticipantRiskDeclaration, SaveParticipantRiskOutcomeClass, SaveParticipantRiskReview,
    SaveParticipantRunStateClass, SourceFidelityRewriteClass,
};
use super::source_fidelity::{encode_for_save, source_fidelity_adjustments, SourceFidelityRecord};
use super::write_strategy::{select_write_strategy, WriteStrategy};

/// Maximum number of participant rows admitted in one synchronous save plan.
pub const MAX_SAVE_PARTICIPANTS: usize = 32;
/// Maximum staged-buffer size admitted to an in-process participant.
pub const MAX_PARTICIPANT_STAGED_BYTES: usize = 16 * 1024 * 1024;
/// Maximum participant deadline admitted on the protected save path.
pub const MAX_PARTICIPANT_TIMEOUT_MS: u64 = 2_000;
/// Aggregate deadline shared by participant discovery and execution for one
/// save attempt.
pub const MAX_PARTICIPANT_PLAN_TIMEOUT_MS: u64 = 2_000;

const MAX_ACTIVE_PARTICIPANT_WORKERS: usize = 4;
const MAX_REVIEW_ADMISSIONS: usize = 64;
const MAX_REVIEW_ADMISSION_BYTES: usize = 32 * 1024 * 1024;
const MAX_PARTICIPANT_ID_BYTES: usize = 160;
const MAX_REVIEW_TICKET_BYTES: usize = 256;
const MAX_VISIBLE_DISCLOSURE_BYTES: usize = 700;
const MAX_REVIEW_ADMISSION_LIFETIME: Duration = Duration::from_secs(5 * 60);
const PARTICIPANT_DESCRIPTOR_TIMEOUT: Duration = Duration::from_millis(100);
const PARTICIPANT_WAIT_SLICE: Duration = Duration::from_millis(5);
const PARTICIPANT_CANCELLATION_GRACE: Duration = Duration::from_millis(25);

static ACTIVE_PARTICIPANT_WORKERS: AtomicUsize = AtomicUsize::new(0);

/// Declared save phase for an in-process participant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SaveParticipantPhaseClass {
    /// Mutates only the staged visible-file buffer.
    FormatFix,
    /// Would update generated companions and therefore requires a supervised
    /// external-effect lane, which this coordinator does not provide.
    GeneratedArtifactUpdate,
    /// Observes the final staged buffer without mutation.
    Validation,
    /// No trustworthy phase declaration was provided.
    UnknownRequiresReview,
}

impl SaveParticipantPhaseClass {
    const fn order(self) -> u8 {
        match self {
            Self::FormatFix => 0,
            Self::GeneratedArtifactUpdate => 1,
            Self::Validation => 2,
            Self::UnknownRequiresReview => u8::MAX,
        }
    }

    /// Returns the stable phase token used by execution receipts.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FormatFix => "format_fix",
            Self::GeneratedArtifactUpdate => "generated_artifact_update",
            Self::Validation => "validation",
            Self::UnknownRequiresReview => "phase_unknown_requires_review",
        }
    }
}

/// Authority/effect boundary declared by an in-process participant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SaveParticipantEffectScopeClass {
    /// The participant receives staged bytes and may return replacement staged
    /// bytes, but declares no filesystem, process, network, or secret effects.
    StagedBufferOnly,
    /// The participant may inspect staged bytes but must return them unchanged.
    ReadOnlyStagedBuffer,
    /// External effects are required and must be routed to a supervised host.
    ExternalEffectsRequireSupervisor,
    /// The effect scope is not known.
    UnknownRequiresReview,
}

impl SaveParticipantEffectScopeClass {
    /// Returns the stable effect-scope token used by execution receipts.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StagedBufferOnly => "staged_buffer_only",
            Self::ReadOnlyStagedBuffer => "read_only_staged_buffer",
            Self::ExternalEffectsRequireSupervisor => "external_effects_require_supervisor",
            Self::UnknownRequiresReview => "effect_scope_unknown_requires_review",
        }
    }
}

/// Bounded execution declaration supplied independently of the risk record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SaveParticipantExecutionDeclaration {
    pub phase_class: SaveParticipantPhaseClass,
    pub effect_scope_class: SaveParticipantEffectScopeClass,
    pub timeout_ms: u64,
    /// Whether the implementation observes [`SaveParticipantRunControl`].
    pub cooperative_cancellation: bool,
}

impl SaveParticipantExecutionDeclaration {
    /// Declares a bounded staged-buffer mutation in `format_fix`.
    pub const fn staged_buffer_format_fix(timeout_ms: u64) -> Self {
        Self {
            phase_class: SaveParticipantPhaseClass::FormatFix,
            effect_scope_class: SaveParticipantEffectScopeClass::StagedBufferOnly,
            timeout_ms,
            cooperative_cancellation: true,
        }
    }

    /// Declares a bounded read-only validation pass.
    pub const fn read_only_validation(timeout_ms: u64) -> Self {
        Self {
            phase_class: SaveParticipantPhaseClass::Validation,
            effect_scope_class: SaveParticipantEffectScopeClass::ReadOnlyStagedBuffer,
            timeout_ms,
            cooperative_cancellation: true,
        }
    }

    /// Returns the fail-closed declaration used when an implementation does
    /// not explicitly name its phase, deadline, and effect boundary.
    pub const fn conservative_default() -> Self {
        Self {
            phase_class: SaveParticipantPhaseClass::UnknownRequiresReview,
            effect_scope_class: SaveParticipantEffectScopeClass::UnknownRequiresReview,
            timeout_ms: 0,
            cooperative_cancellation: false,
        }
    }
}

/// Attempt-scoped cancellation token for [`StagedSaveCoordinator::save_with_cancellation`].
#[derive(Debug, Clone, Default)]
pub struct SaveCancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl SaveCancellationToken {
    pub fn new() -> Self {
        Self::default()
    }

    /// Requests cancellation. A participant receives the same signal through
    /// its run-control object.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// Returns whether cancellation has been requested.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

/// Cooperative deadline/cancellation view supplied to a participant.
#[derive(Debug, Clone)]
pub struct SaveParticipantRunControl {
    attempt_cancellation: SaveCancellationToken,
    participant_cancellation: Arc<AtomicBool>,
    deadline: Instant,
    max_output_bytes: usize,
}

impl SaveParticipantRunControl {
    /// Returns true after user/policy cancellation or the participant deadline.
    pub fn is_cancelled(&self) -> bool {
        self.attempt_cancellation.is_cancelled()
            || self.participant_cancellation.load(Ordering::Acquire)
            || Instant::now() >= self.deadline
    }

    /// Returns the remaining participant budget.
    pub fn remaining(&self) -> Duration {
        self.deadline.saturating_duration_since(Instant::now())
    }

    /// Maximum output size the coordinator will accept.
    pub const fn max_output_bytes(&self) -> usize {
        self.max_output_bytes
    }
}

/// Exact runtime outcome recorded for one participant attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveParticipantExecutionOutcomeClass {
    Ran,
    HeldForReview,
    BlockedBeforeRun,
    TimedOut,
    Cancelled,
    Failed,
    EffectCeilingExceeded,
}

impl SaveParticipantExecutionOutcomeClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ran => "ran",
            Self::HeldForReview => "held_for_review",
            Self::BlockedBeforeRun => "blocked_before_run",
            Self::TimedOut => "timed_out",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
            Self::EffectCeilingExceeded => "effect_ceiling_exceeded",
        }
    }
}

/// Metadata-only effect receipt for one participant. Raw staged bytes and raw
/// provider/tool output are intentionally excluded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveParticipantEffectReceipt {
    pub participant_id: String,
    pub phase_class: SaveParticipantPhaseClass,
    pub effect_scope_class: SaveParticipantEffectScopeClass,
    pub timeout_ms: u64,
    pub outcome_class: SaveParticipantExecutionOutcomeClass,
    pub declared_file_effect_summary: FileEffectSummary,
    pub actual_file_effect_summary: Option<FileEffectSummary>,
    pub effect_ceiling_satisfied: bool,
    pub cancellation_requested: bool,
    pub summary: String,
}

/// A transformation step that can run on staged save content.
///
/// Implementations are trusted in-process code. The coordinator passes no
/// root, path, process, network, or secret capability, but Rust cannot revoke
/// ambient authority already retained inside an implementation. Participants
/// requiring those effects must declare the supervised-external scope and are
/// refused by this lane.
pub trait SaveParticipant: Send + 'static {
    /// Returns the stable id for this participant.
    fn participant_id(&self) -> &'static str;

    /// Returns the participant's risk declaration before staged mutation.
    fn risk_declaration(&self) -> SaveParticipantRiskDeclaration {
        SaveParticipantRiskDeclaration::unknown_requires_review(self.participant_id())
    }

    /// Declares phase, effect boundary, deadline, and cancellation support.
    fn execution_declaration(&self) -> SaveParticipantExecutionDeclaration {
        SaveParticipantExecutionDeclaration::conservative_default()
    }

    /// Runs the participant on the staged content and returns the resulting bytes.
    fn run(&mut self, staged: &[u8]) -> Result<Vec<u8>, String>;

    /// Runs with cooperative deadline and cancellation state. Implementations
    /// that opt into a runnable execution declaration should override this or
    /// ensure `run` itself is promptly bounded.
    fn run_with_control(
        &mut self,
        staged: &[u8],
        control: &SaveParticipantRunControl,
    ) -> Result<Vec<u8>, String> {
        if control.is_cancelled() {
            return Err("save participant cancelled before run".to_owned());
        }
        self.run(staged)
    }
}

/// Error returned when a save participant fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveParticipantError {
    pub participant_id: String,
    pub detail: String,
}

impl std::fmt::Display for SaveParticipantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "save participant failed ({id}): {detail}",
            id = self.participant_id,
            detail = self.detail
        )
    }
}

impl std::error::Error for SaveParticipantError {}

/// Error returned when a reviewed participant admission is not bounded to a
/// usable target/content/declaration/expiry tuple.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveParticipantAdmissionError {
    pub detail: String,
}

impl std::fmt::Display for SaveParticipantAdmissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "save participant admission refused: {}", self.detail)
    }
}

impl std::error::Error for SaveParticipantAdmissionError {}

struct ReviewedParticipantAdmission {
    token: SaveTargetToken,
    staged_content_binding: String,
    staged_content_bytes: usize,
    declaration: SaveParticipantRiskDeclaration,
    ticket_ref: String,
    expires_at: Instant,
}

impl std::fmt::Debug for ReviewedParticipantAdmission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReviewedParticipantAdmission")
            .field("staged_content_binding", &self.staged_content_binding)
            .field("staged_content_bytes", &self.staged_content_bytes)
            .field("has_ticket_ref", &true)
            .field("expires_at", &self.expires_at)
            .finish_non_exhaustive()
    }
}

/// Inputs to one staged save attempt.
pub struct StagedSaveRequest {
    pub token: SaveTargetToken,
    pub new_content: Vec<u8>,
    pub source_fidelity: SourceFidelityRecord,
    pub save_participant_group_id: Option<String>,
    pub checkpoint_ref: Option<String>,
    pub reviewed_in_place_admission: Option<ReviewedInPlaceSave>,
    pub committed_at: String,
}

impl std::fmt::Debug for StagedSaveRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StagedSaveRequest")
            .field("atomic_write_mode", &self.token.atomic_write_mode)
            .field("new_content_len", &self.new_content.len())
            .field(
                "has_save_participant_group",
                &self.save_participant_group_id.is_some(),
            )
            .field("has_checkpoint", &self.checkpoint_ref.is_some())
            .field(
                "has_reviewed_in_place_admission",
                &self.reviewed_in_place_admission.is_some(),
            )
            .field("has_committed_at", &!self.committed_at.is_empty())
            .finish_non_exhaustive()
    }
}

/// Result of one staged save attempt.
#[derive(Debug, Clone)]
pub struct SaveResult {
    pub packet_id: String,
    pub write_strategy: WriteStrategy,
    pub manifest: SaveManifest,
    pub source_fidelity: SourceFidelityRecord,
    /// Save-participant risk review emitted for support and review surfaces.
    pub save_participant_risk_review: SaveParticipantRiskReview,
    /// The token that should be used for the next save attempt.
    pub next_token: SaveTargetToken,
    /// Participant failure detail when `outcome == save_participant_failed`.
    pub participant_error: Option<SaveParticipantError>,
    /// One metadata-only terminal receipt per planned participant.
    pub participant_effect_receipts: Vec<SaveParticipantEffectReceipt>,
}

impl SaveResult {
    /// Returns true when the save committed durable bytes.
    pub fn committed(&self) -> bool {
        matches!(
            self.manifest.outcome,
            SaveOutcome::Committed | SaveOutcome::DegradedGuaranteeDeclared
        )
    }
}

/// Coordinates staging, compare-before-write, and capability-aware commit.
#[derive(Debug)]
pub struct StagedSaveCoordinator {
    next_packet_seq: u64,
    reviewed_participant_admissions: Vec<ReviewedParticipantAdmission>,
}

impl Default for StagedSaveCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

impl StagedSaveCoordinator {
    /// Creates a new staged save coordinator.
    pub fn new() -> Self {
        Self {
            next_packet_seq: 1,
            reviewed_participant_admissions: Vec::new(),
        }
    }

    /// Registers a single-use reviewed participant admission bound to the
    /// exact save target, initial staged content, declaration, ticket, and
    /// expiry. A raw `reviewed_ticket_ref` inside a participant declaration is
    /// never sufficient by itself.
    pub fn admit_reviewed_participant(
        &mut self,
        token: &SaveTargetToken,
        staged_content: &[u8],
        declaration: &SaveParticipantRiskDeclaration,
        ticket_ref: impl Into<String>,
        expires_at: SystemTime,
    ) -> Result<(), SaveParticipantAdmissionError> {
        let ticket_ref = ticket_ref.into();
        if !is_bounded_token(&ticket_ref, MAX_REVIEW_TICKET_BYTES) {
            return Err(SaveParticipantAdmissionError {
                detail: "review ticket ref is empty, oversized, or contains control characters"
                    .to_owned(),
            });
        }
        if declaration.reviewed_ticket_ref.as_deref() != Some(ticket_ref.as_str()) {
            return Err(SaveParticipantAdmissionError {
                detail: "review ticket ref does not match the participant declaration".to_owned(),
            });
        }
        if !is_bounded_token(&declaration.participant_id, MAX_PARTICIPANT_ID_BYTES) {
            return Err(SaveParticipantAdmissionError {
                detail: "participant id is not a bounded support-safe token".to_owned(),
            });
        }
        if staged_content.len() > MAX_PARTICIPANT_STAGED_BYTES {
            return Err(SaveParticipantAdmissionError {
                detail: "reviewed staged content exceeds the in-process participant limit"
                    .to_owned(),
            });
        }
        let lifetime = expires_at.duration_since(SystemTime::now()).map_err(|_| {
            SaveParticipantAdmissionError {
                detail: "review admission is already expired".to_owned(),
            }
        })?;
        if lifetime > MAX_REVIEW_ADMISSION_LIFETIME {
            return Err(SaveParticipantAdmissionError {
                detail: "review admission exceeds the five-minute runtime lifetime".to_owned(),
            });
        }
        let now = Instant::now();
        let monotonic_expiry = now + lifetime;

        self.reviewed_participant_admissions
            .retain(|admission| admission.expires_at > now && admission.ticket_ref != ticket_ref);
        if self.reviewed_participant_admissions.len() >= MAX_REVIEW_ADMISSIONS {
            return Err(SaveParticipantAdmissionError {
                detail: "review admission capacity is exhausted".to_owned(),
            });
        }
        let admitted_bytes = self
            .reviewed_participant_admissions
            .iter()
            .try_fold(0usize, |total, admission| {
                total.checked_add(admission.staged_content_bytes)
            })
            .and_then(|total| total.checked_add(staged_content.len()));
        if !matches!(
            admitted_bytes,
            Some(total) if total <= MAX_REVIEW_ADMISSION_BYTES
        ) {
            return Err(SaveParticipantAdmissionError {
                detail: "review admission byte capacity is exhausted".to_owned(),
            });
        }
        self.reviewed_participant_admissions
            .push(ReviewedParticipantAdmission {
                token: token.clone(),
                staged_content_binding: participant_content_binding(staged_content),
                staged_content_bytes: staged_content.len(),
                declaration: declaration.clone(),
                ticket_ref,
                expires_at: monotonic_expiry,
            });
        Ok(())
    }

    /// Runs the staged save pipeline, returning a typed [`SaveResult`] for
    /// both success and failure outcomes.
    pub fn save(
        &mut self,
        root: &mut dyn VfsRoot,
        request: StagedSaveRequest,
        participants: &mut [Box<dyn SaveParticipant>],
    ) -> SaveResult {
        self.save_with_cancellation(root, request, participants, SaveCancellationToken::new())
    }

    /// Runs the staged save pipeline with attempt-scoped cancellation.
    pub fn save_with_cancellation(
        &mut self,
        root: &mut dyn VfsRoot,
        mut request: StagedSaveRequest,
        participants: &mut [Box<dyn SaveParticipant>],
        cancellation: SaveCancellationToken,
    ) -> SaveResult {
        let packet_id = self.mint_packet_id();
        let source_fidelity = request.source_fidelity.clone();
        let participant_count = participants.len();
        // Target/root preflight runs before calling any participant code,
        // including descriptor methods. A token that does not belong to this
        // root must not cause even participant discovery to execute.
        let mut risk_review = SaveParticipantRiskReview::open(
            format!("{packet_id}:save_participant_risk"),
            packet_id.clone(),
            request.checkpoint_ref.clone(),
            Vec::new(),
        );
        let mut participant_effect_receipts = Vec::new();

        if let Err(conflict) = detect_root_ownership(root, &request.token) {
            risk_review
                .mark_external_change(format!("external_change:{}", conflict.outcome.as_str()));
            let token = request.token;
            let write_strategy = select_write_strategy(&token);
            let manifest = make_manifest(
                root,
                &token,
                request.save_participant_group_id,
                request.checkpoint_ref,
                request.committed_at,
                conflict.outcome,
                Some(conflict.detail),
            );
            return SaveResult {
                packet_id,
                write_strategy,
                manifest,
                source_fidelity,
                save_participant_risk_review: risk_review,
                next_token: token,
                participant_error: None,
                participant_effect_receipts,
            };
        }

        if let Err(conflict) = detect_external_drift(root, &request.token) {
            risk_review
                .mark_external_change(format!("external_change:{}", conflict.outcome.as_str()));
            let token = request.token;
            let write_strategy = select_write_strategy(&token);
            let manifest = make_manifest(
                root,
                &token,
                request.save_participant_group_id,
                request.checkpoint_ref,
                request.committed_at,
                conflict.outcome,
                Some(conflict.detail),
            );
            return SaveResult {
                packet_id,
                write_strategy,
                manifest,
                source_fidelity,
                save_participant_risk_review: risk_review,
                next_token: token,
                participant_error: None,
                participant_effect_receipts,
            };
        }

        if let Some((outcome, detail)) = preflight_block(&request) {
            let token = request.token;
            let write_strategy = select_write_strategy(&token);
            let manifest = make_manifest(
                root,
                &token,
                request.save_participant_group_id,
                request.checkpoint_ref,
                request.committed_at,
                outcome,
                Some(detail.clone()),
            );
            risk_review.mark_blocked_no_write(detail);
            return SaveResult {
                packet_id,
                write_strategy,
                manifest,
                source_fidelity,
                save_participant_risk_review: risk_review,
                next_token: token,
                participant_error: None,
                participant_effect_receipts,
            };
        }

        if participant_count > 0 && request.new_content.len() > MAX_PARTICIPANT_STAGED_BYTES {
            let detail = "staged content exceeds the in-process participant byte limit".to_owned();
            let token = request.token;
            let write_strategy = select_write_strategy(&token);
            let manifest = make_manifest(
                root,
                &token,
                request.save_participant_group_id,
                request.checkpoint_ref,
                request.committed_at,
                SaveOutcome::SaveParticipantFailed,
                Some(detail.clone()),
            );
            risk_review.mark_blocked_no_write(detail.clone());
            return SaveResult {
                packet_id,
                write_strategy,
                manifest,
                source_fidelity,
                save_participant_risk_review: risk_review,
                next_token: token,
                participant_error: Some(SaveParticipantError {
                    participant_id: "save_participant_plan".to_owned(),
                    detail,
                }),
                participant_effect_receipts,
            };
        }

        let participant_plan_deadline =
            Instant::now() + Duration::from_millis(MAX_PARTICIPANT_PLAN_TIMEOUT_MS);
        let (participant_plan, participant_plan_error) = plan_participants(
            participants,
            cancellation.clone(),
            participant_plan_deadline,
        );
        let declarations: Vec<_> = participant_plan
            .iter()
            .map(|entry| entry.declaration.clone())
            .collect();
        let validated_review_refs = self.matching_review_refs(
            &request.token,
            &request.new_content,
            &declarations,
            Instant::now(),
        );
        risk_review = SaveParticipantRiskReview::open_with_validated_review_refs(
            format!("{packet_id}:save_participant_risk"),
            packet_id.clone(),
            request.checkpoint_ref.clone(),
            declarations,
            &validated_review_refs,
        );
        participant_effect_receipts = participant_plan
            .iter()
            .map(ParticipantPlanEntry::planned_receipt)
            .collect::<Vec<_>>();

        if let Some(failure) = participant_plan_error {
            let err = failure.error;
            if let Some(receipt_index) = failure.receipt_index {
                update_receipt(
                    &mut participant_effect_receipts[receipt_index],
                    failure.outcome_class,
                    None,
                    false,
                    failure.cancellation_requested,
                    &err.detail,
                );
            }
            match failure.outcome_class {
                SaveParticipantExecutionOutcomeClass::TimedOut => {
                    risk_review.mark_participant_timed_out(&err.participant_id);
                }
                SaveParticipantExecutionOutcomeClass::Cancelled => {
                    risk_review.mark_participant_cancelled(&err.participant_id);
                }
                SaveParticipantExecutionOutcomeClass::Failed => {
                    risk_review.mark_participant_failed(&err.participant_id);
                }
                _ => risk_review.mark_blocked_no_write(err.to_string()),
            }
            risk_review.mark_unrun_participants_blocked();
            let token = request.token;
            let manifest = make_manifest(
                root,
                &token,
                request.save_participant_group_id,
                request.checkpoint_ref,
                request.committed_at,
                SaveOutcome::SaveParticipantFailed,
                Some(err.to_string()),
            );
            return SaveResult {
                packet_id,
                write_strategy: select_write_strategy(&token),
                manifest,
                source_fidelity: source_fidelity.clone(),
                save_participant_risk_review: risk_review,
                next_token: token,
                participant_error: Some(err),
                participant_effect_receipts,
            };
        }

        if cancellation.is_cancelled() {
            risk_review.mark_unrun_participants_blocked();
            risk_review
                .mark_blocked_no_write("save attempt was cancelled before participant execution");
            for receipt in &mut participant_effect_receipts {
                receipt.outcome_class = SaveParticipantExecutionOutcomeClass::Cancelled;
                receipt.cancellation_requested = true;
                receipt.summary =
                    "Save attempt was cancelled before participant execution.".to_owned();
            }
            let err = SaveParticipantError {
                participant_id: "save_participant_plan".to_owned(),
                detail: "save attempt cancelled before participant execution".to_owned(),
            };
            let token = request.token;
            let manifest = make_manifest(
                root,
                &token,
                request.save_participant_group_id,
                request.checkpoint_ref,
                request.committed_at,
                SaveOutcome::SaveParticipantFailed,
                Some(err.to_string()),
            );
            return SaveResult {
                packet_id,
                write_strategy: select_write_strategy(&token),
                manifest,
                source_fidelity: source_fidelity.clone(),
                save_participant_risk_review: risk_review,
                next_token: token,
                participant_error: Some(err),
                participant_effect_receipts,
            };
        }

        if risk_review.outcome_class
            == SaveParticipantRiskOutcomeClass::ReviewRequiredBeforeMutation
        {
            mark_review_receipts(&mut participant_effect_receipts, &risk_review);
            let token = request.token;
            let manifest = make_manifest(
                root,
                &token,
                request.save_participant_group_id,
                request.checkpoint_ref,
                request.committed_at,
                SaveOutcome::ReviewRequiredBeforeSave,
                Some("save participant requires review before staged mutation".to_owned()),
            );
            return SaveResult {
                packet_id,
                write_strategy: select_write_strategy(&token),
                manifest,
                source_fidelity: source_fidelity.clone(),
                save_participant_risk_review: risk_review,
                next_token: token,
                participant_error: None,
                participant_effect_receipts,
            };
        }

        let mut staged = request.new_content;
        let mut participant_error: Option<SaveParticipantError> = None;

        for (index, plan) in participant_plan.iter().enumerate() {
            let participant_id = plan.participant_id.as_str();
            if plan.declaration.reviewed_ticket_ref.is_some()
                && !self.consume_matching_review_admission(
                    &request.token,
                    &staged,
                    &plan.declaration,
                    Instant::now(),
                )
            {
                risk_review.invalidate_review_admission(participant_id);
                mark_review_receipts(&mut participant_effect_receipts, &risk_review);
                let token = request.token;
                let manifest = make_manifest(
                    root,
                    &token,
                    request.save_participant_group_id,
                    request.checkpoint_ref,
                    request.committed_at,
                    SaveOutcome::ReviewRequiredBeforeSave,
                    Some(
                        "reviewed participant admission no longer matches staged input".to_owned(),
                    ),
                );
                return SaveResult {
                    packet_id,
                    write_strategy: select_write_strategy(&token),
                    manifest,
                    source_fidelity: source_fidelity.clone(),
                    save_participant_risk_review: risk_review,
                    next_token: token,
                    participant_error: None,
                    participant_effect_receipts,
                };
            }

            let before = staged.clone();
            match run_participant_bounded(
                &mut participants[index],
                before.clone(),
                plan.execution,
                cancellation.clone(),
                participant_plan_deadline,
            ) {
                BoundedParticipantRun::Completed(Ok(next)) => {
                    if next.len() > MAX_PARTICIPANT_STAGED_BYTES {
                        let actual = summarize_staged_file_effect(&before, &next);
                        update_receipt(
                            &mut participant_effect_receipts[index],
                            SaveParticipantExecutionOutcomeClass::EffectCeilingExceeded,
                            Some(actual),
                            false,
                            false,
                            "Participant output exceeded the staged-buffer size limit.",
                        );
                        risk_review.mark_blocked_no_write(
                            "save participant output exceeded the staged-buffer size limit",
                        );
                        participant_error = Some(SaveParticipantError {
                            participant_id: participant_id.to_owned(),
                            detail: "participant output exceeds bounded staged-buffer limit"
                                .to_owned(),
                        });
                        break;
                    }
                    let actual = summarize_staged_file_effect(&before, &next);
                    match risk_review.record_actual_effect(participant_id, actual.clone()) {
                        SaveParticipantEffectRecordOutcome::Accepted => {
                            update_receipt(
                                &mut participant_effect_receipts[index],
                                SaveParticipantExecutionOutcomeClass::Ran,
                                Some(actual),
                                true,
                                false,
                                "Participant completed within its declared effect ceiling.",
                            );
                            staged = next;
                        }
                        SaveParticipantEffectRecordOutcome::CeilingExceeded { dimensions } => {
                            update_receipt(
                                &mut participant_effect_receipts[index],
                                SaveParticipantExecutionOutcomeClass::EffectCeilingExceeded,
                                Some(actual),
                                false,
                                false,
                                "Participant output exceeded its declared effect ceiling.",
                            );
                            let token = request.token;
                            let detail = format!(
                                "save participant effect ceiling exceeded: {}",
                                dimensions.join(",")
                            );
                            let manifest = make_manifest(
                                root,
                                &token,
                                request.save_participant_group_id,
                                request.checkpoint_ref,
                                request.committed_at,
                                SaveOutcome::ReviewRequiredBeforeSave,
                                Some(detail),
                            );
                            mark_remaining_receipts_blocked(
                                &mut participant_effect_receipts,
                                index + 1,
                                "Participant execution stopped after an effect-ceiling violation.",
                            );
                            return SaveResult {
                                packet_id,
                                write_strategy: select_write_strategy(&token),
                                manifest,
                                source_fidelity: source_fidelity.clone(),
                                save_participant_risk_review: risk_review,
                                next_token: token,
                                participant_error: None,
                                participant_effect_receipts,
                            };
                        }
                        SaveParticipantEffectRecordOutcome::UnknownParticipant => {
                            update_receipt(
                                &mut participant_effect_receipts[index],
                                SaveParticipantExecutionOutcomeClass::Failed,
                                Some(actual),
                                false,
                                false,
                                "Participant result did not match a declared participant id.",
                            );
                            participant_error = Some(SaveParticipantError {
                                participant_id: participant_id.to_owned(),
                                detail: "participant effect receipt has no matching declaration"
                                    .to_owned(),
                            });
                            break;
                        }
                    }
                }
                BoundedParticipantRun::Completed(Err(_private_detail)) => {
                    update_receipt(
                        &mut participant_effect_receipts[index],
                        SaveParticipantExecutionOutcomeClass::Failed,
                        None,
                        false,
                        cancellation.is_cancelled(),
                        "Participant failed before producing an accepted effect receipt.",
                    );
                    participant_error = Some(SaveParticipantError {
                        participant_id: participant_id.to_owned(),
                        detail: "participant returned an error; private detail withheld".to_owned(),
                    });
                    risk_review.mark_participant_failed(participant_id);
                    break;
                }
                BoundedParticipantRun::TimedOut => {
                    update_receipt(
                        &mut participant_effect_receipts[index],
                        SaveParticipantExecutionOutcomeClass::TimedOut,
                        None,
                        false,
                        true,
                        "Participant timed out and cancellation was requested.",
                    );
                    risk_review.mark_participant_timed_out(participant_id);
                    participant_error = Some(SaveParticipantError {
                        participant_id: participant_id.to_owned(),
                        detail: format!(
                            "participant timed out after {} ms",
                            plan.execution.timeout_ms
                        ),
                    });
                    break;
                }
                BoundedParticipantRun::Cancelled => {
                    update_receipt(
                        &mut participant_effect_receipts[index],
                        SaveParticipantExecutionOutcomeClass::Cancelled,
                        None,
                        false,
                        true,
                        "Participant was cancelled and produced no accepted effect receipt.",
                    );
                    risk_review.mark_participant_cancelled(participant_id);
                    participant_error = Some(SaveParticipantError {
                        participant_id: participant_id.to_owned(),
                        detail: "participant cancelled".to_owned(),
                    });
                    break;
                }
            }
        }

        let token = request.token;
        if let Some(err) = participant_error.clone() {
            let first_unfinished = participant_effect_receipts
                .iter()
                .position(|receipt| {
                    receipt.outcome_class == SaveParticipantExecutionOutcomeClass::BlockedBeforeRun
                })
                .unwrap_or(participant_effect_receipts.len());
            mark_remaining_receipts_blocked(
                &mut participant_effect_receipts,
                first_unfinished,
                "Participant execution stopped after a prior participant did not complete.",
            );
            risk_review.mark_unrun_participants_blocked();
            let manifest = make_manifest(
                root,
                &token,
                request.save_participant_group_id,
                request.checkpoint_ref,
                request.committed_at,
                SaveOutcome::SaveParticipantFailed,
                Some(err.to_string()),
            );
            return SaveResult {
                packet_id,
                write_strategy: select_write_strategy(&token),
                manifest,
                source_fidelity: source_fidelity.clone(),
                save_participant_risk_review: risk_review,
                next_token: token,
                participant_error: Some(err),
                participant_effect_receipts,
            };
        }

        let write_strategy = select_write_strategy(&token);
        let canonical_uri = token
            .identity
            .canonical_filesystem_object
            .canonical_uri
            .clone();

        if participant_count > 0 {
            match source_fidelity_adjustments(&source_fidelity, &staged) {
                Ok(adjustments) if !adjustments.is_empty() => {
                    risk_review.mark_source_fidelity_adjustments(adjustments);
                    let manifest = make_manifest(
                        root,
                        &token,
                        request.save_participant_group_id,
                        request.checkpoint_ref,
                        request.committed_at,
                        SaveOutcome::ReviewRequiredBeforeSave,
                        Some(
                            "save participant output would change source-fidelity posture"
                                .to_owned(),
                        ),
                    );
                    return SaveResult {
                        packet_id,
                        write_strategy,
                        manifest,
                        source_fidelity: source_fidelity.clone(),
                        save_participant_risk_review: risk_review,
                        next_token: token,
                        participant_error: None,
                        participant_effect_receipts,
                    };
                }
                Ok(_) => {}
                Err(detail) => {
                    let err = SaveParticipantError {
                        participant_id: "source_fidelity_risk_review".to_owned(),
                        detail,
                    };
                    risk_review.mark_participant_failed("source_fidelity_risk_review");
                    let manifest = make_manifest(
                        root,
                        &token,
                        request.save_participant_group_id,
                        request.checkpoint_ref,
                        request.committed_at,
                        SaveOutcome::SaveParticipantFailed,
                        Some(err.to_string()),
                    );
                    return SaveResult {
                        packet_id,
                        write_strategy,
                        manifest,
                        source_fidelity: source_fidelity.clone(),
                        save_participant_risk_review: risk_review,
                        next_token: token,
                        participant_error: Some(err),
                        participant_effect_receipts,
                    };
                }
            }
        }

        let staged = match encode_for_save(&source_fidelity, &staged) {
            Ok(bytes) => bytes,
            Err(detail) => {
                let err = SaveParticipantError {
                    participant_id: "source_fidelity_conversion".to_owned(),
                    detail,
                };
                let manifest = make_manifest(
                    root,
                    &token,
                    request.save_participant_group_id,
                    request.checkpoint_ref,
                    request.committed_at,
                    SaveOutcome::SaveParticipantFailed,
                    Some(err.to_string()),
                );
                return SaveResult {
                    packet_id,
                    write_strategy,
                    manifest,
                    source_fidelity: source_fidelity.clone(),
                    save_participant_risk_review: risk_review,
                    next_token: token,
                    participant_error: Some(err),
                    participant_effect_receipts,
                };
            }
        };

        // The final generation comparison and commit live under root authority.
        // The earlier drift check produces review metadata; this conditional
        // primitive is the correctness gate that closes the coordinator's old
        // check-then-pathname-write gap.
        let expected_generation = GenerationToken {
            kind: token.compare_before_write_generation_token.kind,
            value: token.compare_before_write_generation_token.value.clone(),
        };
        let condition = RootWriteCondition::new(
            token
                .identity
                .canonical_filesystem_object
                .strongest_identity_token
                .clone(),
            expected_generation,
            token.permission_snapshot.clone(),
            token.atomic_write_mode,
            request.reviewed_in_place_admission.take(),
        );
        let commit = root.compare_and_write(&canonical_uri, &condition, &staged);
        let (outcome, failure_detail, postcommit_observation) = match commit {
            Ok(ConditionalWriteOutcome::Committed { observation }) => {
                let outcome = match write_strategy {
                    WriteStrategy::AtomicReplace => SaveOutcome::Committed,
                    WriteStrategy::InPlaceWrite | WriteStrategy::ConditionalRemoteWrite => {
                        SaveOutcome::DegradedGuaranteeDeclared
                    }
                    WriteStrategy::Blocked => SaveOutcome::ReadOnlyOrPolicyBlocked,
                };
                (outcome, None, Some(observation))
            }
            Ok(ConditionalWriteOutcome::DurabilityUncertain {
                detail,
                observation,
            }) => (SaveOutcome::DurabilityUncertain, Some(detail), observation),
            Ok(ConditionalWriteOutcome::Conflict {
                observed_generation_token,
            }) => {
                let outcome = if token.atomic_write_mode == AtomicWriteMode::ConditionalRemoteWrite
                {
                    SaveOutcome::SaveConflict
                } else {
                    SaveOutcome::ExternalChangeDetected
                };
                let detail = observed_generation_token.map_or_else(
                    || "conditional_write_conflict: observed generation unavailable".to_owned(),
                    |observed| {
                        format!(
                            "conditional_write_conflict: pinned {} observed {}",
                            condition.expected_generation.value, observed.value
                        )
                    },
                );
                risk_review.mark_external_change(format!("external_change:{}", outcome.as_str()));
                (outcome, Some(detail), None)
            }
            Err(err) => {
                let detail = err.to_string();
                let outcome = match err {
                    aureline_vfs::roots::RootIoError::NotSupported { .. } => {
                        SaveOutcome::GeneratedOrManagedWriteBlocked
                    }
                    aureline_vfs::roots::RootIoError::IoFailure { .. } => {
                        SaveOutcome::PrecommitIoFailed
                    }
                };
                (outcome, Some(detail), None)
            }
        };

        let manifest = make_manifest_with_observation(
            root,
            &token,
            request.save_participant_group_id.clone(),
            request.checkpoint_ref.clone(),
            request.committed_at.clone(),
            outcome,
            failure_detail.clone(),
            postcommit_observation.as_ref(),
        );
        match outcome {
            SaveOutcome::DurabilityUncertain => risk_review.mark_commit_uncertain(
                failure_detail
                    .clone()
                    .unwrap_or_else(|| "save commit durability is uncertain".to_owned()),
            ),
            SaveOutcome::Committed | SaveOutcome::DegradedGuaranteeDeclared => {
                risk_review.mark_committed();
            }
            // The conditional primitive may detect drift after the earlier
            // review check. Preserve the external-change classification set
            // above instead of flattening it into a generic blocked write.
            SaveOutcome::ExternalChangeDetected | SaveOutcome::SaveConflict => {}
            _ => risk_review.mark_blocked_no_write(
                failure_detail
                    .clone()
                    .unwrap_or_else(|| "save did not commit durable bytes".to_owned()),
            ),
        }

        let next_token = postcommit_observation.as_ref().map_or_else(
            || token.clone(),
            |observation| token_from_postcommit(&token, observation, &request.committed_at),
        );

        SaveResult {
            packet_id,
            write_strategy,
            manifest,
            source_fidelity,
            save_participant_risk_review: risk_review,
            next_token,
            participant_error: None,
            participant_effect_receipts,
        }
    }

    fn matching_review_refs(
        &mut self,
        token: &SaveTargetToken,
        staged_content: &[u8],
        declarations: &[SaveParticipantRiskDeclaration],
        now: Instant,
    ) -> Vec<(String, String)> {
        self.reviewed_participant_admissions
            .retain(|admission| admission.expires_at > now);
        let staged_content_binding = participant_content_binding(staged_content);
        declarations
            .iter()
            .filter_map(|declaration| {
                let ticket = declaration.reviewed_ticket_ref.as_ref()?;
                self.reviewed_participant_admissions
                    .iter()
                    .any(|admission| {
                        admission.expires_at > now
                            && &admission.token == token
                            && admission.staged_content_binding == staged_content_binding
                            && &admission.declaration == declaration
                            && &admission.ticket_ref == ticket
                    })
                    .then(|| (declaration.participant_id.clone(), ticket.clone()))
            })
            .collect()
    }

    fn consume_matching_review_admission(
        &mut self,
        token: &SaveTargetToken,
        staged_content: &[u8],
        declaration: &SaveParticipantRiskDeclaration,
        now: Instant,
    ) -> bool {
        let Some(ticket) = declaration.reviewed_ticket_ref.as_ref() else {
            return true;
        };
        let staged_content_binding = participant_content_binding(staged_content);
        let Some(index) = self
            .reviewed_participant_admissions
            .iter()
            .position(|admission| {
                admission.expires_at > now
                    && &admission.token == token
                    && admission.staged_content_binding == staged_content_binding
                    && &admission.declaration == declaration
                    && &admission.ticket_ref == ticket
            })
        else {
            return false;
        };
        self.reviewed_participant_admissions.swap_remove(index);
        true
    }

    fn mint_packet_id(&mut self) -> String {
        let seq = self.next_packet_seq;
        self.next_packet_seq = self.next_packet_seq.saturating_add(1);
        let stamp = monotonic_stamp();
        format!("save_packet:{stamp}:{seq}")
    }
}

#[derive(Debug, Clone)]
struct ParticipantPlanEntry {
    participant_id: String,
    declaration: SaveParticipantRiskDeclaration,
    execution: SaveParticipantExecutionDeclaration,
}

#[derive(Debug)]
struct ParticipantPlanFailure {
    error: SaveParticipantError,
    receipt_index: Option<usize>,
    outcome_class: SaveParticipantExecutionOutcomeClass,
    cancellation_requested: bool,
}

impl ParticipantPlanEntry {
    fn planned_receipt(&self) -> SaveParticipantEffectReceipt {
        SaveParticipantEffectReceipt {
            participant_id: self.participant_id.clone(),
            phase_class: self.execution.phase_class,
            effect_scope_class: self.execution.effect_scope_class,
            timeout_ms: self.execution.timeout_ms,
            outcome_class: SaveParticipantExecutionOutcomeClass::BlockedBeforeRun,
            declared_file_effect_summary: self.declaration.declared_file_effect_summary.clone(),
            actual_file_effect_summary: None,
            effect_ceiling_satisfied: false,
            cancellation_requested: false,
            summary: "Participant did not run before the save attempt stopped.".to_owned(),
        }
    }
}

fn plan_participants(
    participants: &mut [Box<dyn SaveParticipant>],
    cancellation: SaveCancellationToken,
    plan_deadline: Instant,
) -> (Vec<ParticipantPlanEntry>, Option<ParticipantPlanFailure>) {
    if participants.len() > MAX_SAVE_PARTICIPANTS {
        return (
            Vec::new(),
            Some(ParticipantPlanFailure {
                error: SaveParticipantError {
                    participant_id: "save_participant_plan".to_owned(),
                    detail: format!(
                        "participant count {} exceeds limit {}",
                        participants.len(),
                        MAX_SAVE_PARTICIPANTS
                    ),
                },
                receipt_index: None,
                outcome_class: SaveParticipantExecutionOutcomeClass::BlockedBeforeRun,
                cancellation_requested: false,
            }),
        );
    }

    let mut plan = Vec::with_capacity(participants.len());
    let mut first_error = None;
    let mut participant_ids = HashSet::with_capacity(participants.len());
    let mut previous_phase_order = 0;

    for index in 0..participants.len() {
        let descriptor = match inspect_participant_bounded(
            &mut participants[index],
            cancellation.clone(),
            plan_deadline,
        ) {
            BoundedParticipantInspection::Completed(descriptor) => descriptor,
            BoundedParticipantInspection::TimedOut => {
                let participant_id = format!("participant_slot:{index}");
                let declaration =
                    SaveParticipantRiskDeclaration::unknown_requires_review(&participant_id);
                plan.push(ParticipantPlanEntry {
                    participant_id: participant_id.clone(),
                    declaration,
                    execution: SaveParticipantExecutionDeclaration::conservative_default(),
                });
                first_error = Some(ParticipantPlanFailure {
                    error: SaveParticipantError {
                        participant_id,
                        detail: "participant descriptor timed out before execution".to_owned(),
                    },
                    receipt_index: Some(index),
                    outcome_class: SaveParticipantExecutionOutcomeClass::TimedOut,
                    cancellation_requested: true,
                });
                append_uninspected_participant_slots(&mut plan, index + 1, participants.len());
                break;
            }
            BoundedParticipantInspection::Cancelled => {
                let participant_id = format!("participant_slot:{index}");
                let declaration =
                    SaveParticipantRiskDeclaration::unknown_requires_review(&participant_id);
                plan.push(ParticipantPlanEntry {
                    participant_id: participant_id.clone(),
                    declaration,
                    execution: SaveParticipantExecutionDeclaration::conservative_default(),
                });
                first_error = Some(ParticipantPlanFailure {
                    error: SaveParticipantError {
                        participant_id,
                        detail: "save cancelled during participant preflight".to_owned(),
                    },
                    receipt_index: Some(index),
                    outcome_class: SaveParticipantExecutionOutcomeClass::Cancelled,
                    cancellation_requested: true,
                });
                append_uninspected_participant_slots(&mut plan, index + 1, participants.len());
                break;
            }
            BoundedParticipantInspection::Failed(detail) => {
                let participant_id = format!("participant_slot:{index}");
                let declaration =
                    SaveParticipantRiskDeclaration::unknown_requires_review(&participant_id);
                plan.push(ParticipantPlanEntry {
                    participant_id: participant_id.clone(),
                    declaration,
                    execution: SaveParticipantExecutionDeclaration::conservative_default(),
                });
                first_error = Some(ParticipantPlanFailure {
                    error: SaveParticipantError {
                        participant_id,
                        detail,
                    },
                    receipt_index: Some(index),
                    outcome_class: SaveParticipantExecutionOutcomeClass::Failed,
                    cancellation_requested: false,
                });
                append_uninspected_participant_slots(&mut plan, index + 1, participants.len());
                break;
            }
        };
        let participant_id = descriptor.participant_id;
        let declaration = descriptor.declaration;
        let execution = descriptor.execution;
        let mut error_detail = None;

        if !is_bounded_token(&participant_id, MAX_PARTICIPANT_ID_BYTES) {
            error_detail = Some(
                "participant id is empty, oversized, or contains control characters".to_owned(),
            );
        } else if declaration.participant_id != participant_id {
            error_detail = Some(format!(
                "risk declaration id {} does not match runtime participant id {}",
                declaration.participant_id, participant_id
            ));
        } else if !participant_ids.insert(participant_id.clone()) {
            error_detail = Some("participant id is duplicated in the save plan".to_owned());
        } else if let Err(detail) = validate_risk_declaration(&declaration) {
            error_detail = Some(detail);
        } else if let Err(detail) = validate_execution_declaration(&declaration, execution) {
            error_detail = Some(detail);
        } else if index > 0 && execution.phase_class.order() < previous_phase_order {
            error_detail = Some(
                "participant phase order moved backwards; coordinator refuses implicit reordering"
                    .to_owned(),
            );
        }

        previous_phase_order = execution.phase_class.order();
        if first_error.is_none() {
            if let Some(detail) = error_detail {
                first_error = Some(ParticipantPlanFailure {
                    error: SaveParticipantError {
                        participant_id: participant_id.clone(),
                        detail,
                    },
                    receipt_index: Some(index),
                    outcome_class: SaveParticipantExecutionOutcomeClass::BlockedBeforeRun,
                    cancellation_requested: false,
                });
            }
        }
        plan.push(ParticipantPlanEntry {
            participant_id,
            declaration,
            execution,
        });
    }

    (plan, first_error)
}

fn append_uninspected_participant_slots(
    plan: &mut Vec<ParticipantPlanEntry>,
    from: usize,
    participant_count: usize,
) {
    for index in from..participant_count {
        let participant_id = format!("participant_slot:{index}");
        plan.push(ParticipantPlanEntry {
            declaration: SaveParticipantRiskDeclaration::unknown_requires_review(&participant_id),
            participant_id,
            execution: SaveParticipantExecutionDeclaration::conservative_default(),
        });
    }
}

fn validate_risk_declaration(declaration: &SaveParticipantRiskDeclaration) -> Result<(), String> {
    if !is_bounded_token(&declaration.participant_id, MAX_PARTICIPANT_ID_BYTES) {
        return Err("risk declaration participant id is not a bounded token".to_owned());
    }
    if !is_bounded_text(
        &declaration.visible_disclosure,
        MAX_VISIBLE_DISCLOSURE_BYTES,
    ) {
        return Err(
            "participant disclosure is empty, oversized, or contains control bytes".to_owned(),
        );
    }
    if declaration.review_trigger_classes.is_empty()
        || declaration.review_trigger_classes.len() > 16
        || declaration
            .review_trigger_classes
            .iter()
            .enumerate()
            .any(|(index, trigger)| declaration.review_trigger_classes[..index].contains(trigger))
    {
        return Err("participant review triggers must be nonempty, bounded, and unique".to_owned());
    }
    if let Some(ticket) = declaration.reviewed_ticket_ref.as_deref() {
        if !is_bounded_token(ticket, MAX_REVIEW_TICKET_BYTES) {
            return Err("participant review ticket ref is not a bounded token".to_owned());
        }
    }
    if declaration.declared_file_effect_summary.changed_bytes > MAX_PARTICIPANT_STAGED_BYTES as u64
    {
        return Err("declared changed-byte ceiling exceeds the participant lane limit".to_owned());
    }
    if declaration.fix_safety_class == SaveParticipantFixSafetyClass::SafeLocalTextEdit {
        let effect = &declaration.declared_file_effect_summary;
        if effect.files_touched > 1
            || effect.files_created > 0
            || effect.files_deleted > 0
            || effect.whole_file_rewrite
            || effect.generated_artifacts_touched > 0
            || effect.protected_paths_touched > 0
            || effect.may_touch_outside_visible_file
        {
            return Err(
                "safe_local_text_edit declaration contains effects outside one staged file"
                    .to_owned(),
            );
        }
    }
    Ok(())
}

fn validate_execution_declaration(
    risk: &SaveParticipantRiskDeclaration,
    execution: SaveParticipantExecutionDeclaration,
) -> Result<(), String> {
    if execution.timeout_ms == 0 || execution.timeout_ms > MAX_PARTICIPANT_TIMEOUT_MS {
        return Err(format!(
            "participant timeout must be within 1..={MAX_PARTICIPANT_TIMEOUT_MS} ms"
        ));
    }
    if !execution.cooperative_cancellation {
        return Err("participant does not declare cooperative cancellation support".to_owned());
    }

    match (execution.phase_class, execution.effect_scope_class) {
        (
            SaveParticipantPhaseClass::FormatFix,
            SaveParticipantEffectScopeClass::StagedBufferOnly,
        ) => {
            if matches!(
                risk.participant_class,
                SaveParticipantClass::ValidationAfterApply | SaveParticipantClass::ScannerReadOnly
            ) || risk.output_origin_class == SaveParticipantOutputOrigin::ReadOnlyValidation
            {
                return Err("read-only participant was declared in a mutating phase".to_owned());
            }
            let effect = &risk.declared_file_effect_summary;
            if effect.files_touched > 1
                || effect.files_created > 0
                || effect.files_deleted > 0
                || effect.generated_artifacts_touched > 0
                || effect.protected_paths_touched > 0
                || effect.may_touch_outside_visible_file
            {
                return Err(
                    "in-process staged-buffer participant declared external file effects"
                        .to_owned(),
                );
            }
        }
        (
            SaveParticipantPhaseClass::Validation,
            SaveParticipantEffectScopeClass::ReadOnlyStagedBuffer,
        ) => {
            if !matches!(
                risk.participant_class,
                SaveParticipantClass::ValidationAfterApply | SaveParticipantClass::ScannerReadOnly
            ) || risk.output_origin_class != SaveParticipantOutputOrigin::ReadOnlyValidation
                || risk.source_fidelity_rewrite_class != SourceFidelityRewriteClass::NoWriteNeeded
                || risk.declared_file_effect_summary != FileEffectSummary::no_write()
            {
                return Err(
                    "validation phase requires a read-only class, origin, and no-write effect"
                        .to_owned(),
                );
            }
        }
        (
            SaveParticipantPhaseClass::GeneratedArtifactUpdate,
            SaveParticipantEffectScopeClass::ExternalEffectsRequireSupervisor,
        ) => {
            return Err(
                "generated or external effects require a supervised participant host".to_owned(),
            );
        }
        _ => {
            return Err(
                "participant phase/effect declaration is unknown or unsupported by this lane"
                    .to_owned(),
            );
        }
    }
    Ok(())
}

fn is_bounded_token(value: &str, max_bytes: usize) -> bool {
    !value.is_empty() && value.len() <= max_bytes && !value.chars().any(char::is_control)
}

fn participant_content_binding(value: &[u8]) -> String {
    aureline_history::body_object_id(value)
}

fn is_bounded_text(value: &str, max_bytes: usize) -> bool {
    !value.is_empty()
        && value.len() <= max_bytes
        && !value
            .chars()
            .any(|character| character.is_control() && !matches!(character, '\n' | '\t'))
}

fn mark_review_receipts(
    receipts: &mut [SaveParticipantEffectReceipt],
    risk_review: &SaveParticipantRiskReview,
) {
    for (receipt, entry) in receipts
        .iter_mut()
        .zip(risk_review.participant_entries.iter())
    {
        if entry.run_state_class == SaveParticipantRunStateClass::HeldForReview {
            receipt.outcome_class = SaveParticipantExecutionOutcomeClass::HeldForReview;
            receipt.summary = "Participant was held for review before staged mutation.".to_owned();
        } else if entry.run_state_class == SaveParticipantRunStateClass::BlockedBeforeRun {
            receipt.outcome_class = SaveParticipantExecutionOutcomeClass::BlockedBeforeRun;
            receipt.summary = "Participant was blocked before staged mutation.".to_owned();
        }
    }
}

fn update_receipt(
    receipt: &mut SaveParticipantEffectReceipt,
    outcome_class: SaveParticipantExecutionOutcomeClass,
    actual_file_effect_summary: Option<FileEffectSummary>,
    effect_ceiling_satisfied: bool,
    cancellation_requested: bool,
    summary: &str,
) {
    receipt.outcome_class = outcome_class;
    receipt.actual_file_effect_summary = actual_file_effect_summary;
    receipt.effect_ceiling_satisfied = effect_ceiling_satisfied;
    receipt.cancellation_requested = cancellation_requested;
    receipt.summary = summary.to_owned();
}

fn mark_remaining_receipts_blocked(
    receipts: &mut [SaveParticipantEffectReceipt],
    from: usize,
    summary: &str,
) {
    for receipt in receipts.iter_mut().skip(from) {
        if receipt.outcome_class == SaveParticipantExecutionOutcomeClass::BlockedBeforeRun {
            receipt.summary = summary.to_owned();
        }
    }
}

struct ParticipantDescriptor {
    participant_id: String,
    declaration: SaveParticipantRiskDeclaration,
    execution: SaveParticipantExecutionDeclaration,
}

enum BoundedParticipantInspection {
    Completed(ParticipantDescriptor),
    TimedOut,
    Cancelled,
    Failed(String),
}

struct ParticipantInspectionCompletion {
    participant: Box<dyn SaveParticipant>,
    result: Result<ParticipantDescriptor, String>,
}

fn inspect_participant_bounded(
    participant_slot: &mut Box<dyn SaveParticipant>,
    attempt_cancellation: SaveCancellationToken,
    plan_deadline: Instant,
) -> BoundedParticipantInspection {
    if attempt_cancellation.is_cancelled() {
        return BoundedParticipantInspection::Cancelled;
    }
    if Instant::now() >= plan_deadline {
        return BoundedParticipantInspection::TimedOut;
    }
    let Some(worker_slot) = ParticipantWorkerSlot::acquire() else {
        return BoundedParticipantInspection::Failed(
            "bounded save-participant worker capacity is exhausted".to_owned(),
        );
    };
    let participant = mem::replace(participant_slot, Box::new(UnavailableParticipant));
    let deadline = (Instant::now() + PARTICIPANT_DESCRIPTOR_TIMEOUT).min(plan_deadline);
    let (sender, receiver) = mpsc::sync_channel(1);
    let spawned = std::thread::Builder::new()
        .name("aureline-save-participant-preflight".to_owned())
        .spawn(move || {
            let _worker_slot = worker_slot;
            let participant = participant;
            let result = catch_unwind(AssertUnwindSafe(|| ParticipantDescriptor {
                participant_id: participant.participant_id().to_owned(),
                declaration: participant.risk_declaration(),
                execution: participant.execution_declaration(),
            }))
            .map_err(|_| "save participant panicked during preflight".to_owned());
            let _ = sender.send(ParticipantInspectionCompletion {
                participant,
                result,
            });
        });
    if spawned.is_err() {
        return BoundedParticipantInspection::Failed(
            "could not start bounded save-participant preflight worker".to_owned(),
        );
    }

    loop {
        if attempt_cancellation.is_cancelled() {
            reclaim_inspected_participant(participant_slot, &receiver);
            return BoundedParticipantInspection::Cancelled;
        }
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            reclaim_inspected_participant(participant_slot, &receiver);
            return BoundedParticipantInspection::TimedOut;
        }
        match receiver.recv_timeout(remaining.min(PARTICIPANT_WAIT_SLICE)) {
            Ok(completion) => {
                *participant_slot = completion.participant;
                if attempt_cancellation.is_cancelled() {
                    return BoundedParticipantInspection::Cancelled;
                }
                if Instant::now() >= deadline {
                    return BoundedParticipantInspection::TimedOut;
                }
                return match completion.result {
                    Ok(descriptor) => BoundedParticipantInspection::Completed(descriptor),
                    Err(detail) => BoundedParticipantInspection::Failed(detail),
                };
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                return BoundedParticipantInspection::Failed(
                    "save-participant preflight worker ended without a result".to_owned(),
                );
            }
        }
    }
}

fn reclaim_inspected_participant(
    participant_slot: &mut Box<dyn SaveParticipant>,
    receiver: &mpsc::Receiver<ParticipantInspectionCompletion>,
) {
    if let Ok(completion) = receiver.recv_timeout(PARTICIPANT_CANCELLATION_GRACE) {
        *participant_slot = completion.participant;
    }
}

enum BoundedParticipantRun {
    Completed(Result<Vec<u8>, String>),
    TimedOut,
    Cancelled,
}

struct ParticipantWorkerCompletion {
    participant: Box<dyn SaveParticipant>,
    result: Result<Vec<u8>, String>,
}

struct ParticipantWorkerSlot;

impl ParticipantWorkerSlot {
    fn acquire() -> Option<Self> {
        let mut active = ACTIVE_PARTICIPANT_WORKERS.load(Ordering::Acquire);
        loop {
            if active >= MAX_ACTIVE_PARTICIPANT_WORKERS {
                return None;
            }
            match ACTIVE_PARTICIPANT_WORKERS.compare_exchange_weak(
                active,
                active + 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return Some(Self),
                Err(observed) => active = observed,
            }
        }
    }
}

impl Drop for ParticipantWorkerSlot {
    fn drop(&mut self) {
        ACTIVE_PARTICIPANT_WORKERS.fetch_sub(1, Ordering::AcqRel);
    }
}

struct UnavailableParticipant;

impl SaveParticipant for UnavailableParticipant {
    fn participant_id(&self) -> &'static str {
        "internal:participant_unavailable"
    }

    fn run(&mut self, _staged: &[u8]) -> Result<Vec<u8>, String> {
        Err("participant is unavailable after bounded execution stopped".to_owned())
    }
}

fn run_participant_bounded(
    participant_slot: &mut Box<dyn SaveParticipant>,
    staged: Vec<u8>,
    execution: SaveParticipantExecutionDeclaration,
    attempt_cancellation: SaveCancellationToken,
    plan_deadline: Instant,
) -> BoundedParticipantRun {
    if attempt_cancellation.is_cancelled() {
        return BoundedParticipantRun::Cancelled;
    }
    if Instant::now() >= plan_deadline {
        return BoundedParticipantRun::TimedOut;
    }
    let Some(worker_slot) = ParticipantWorkerSlot::acquire() else {
        return BoundedParticipantRun::Completed(Err(
            "bounded save-participant worker capacity is exhausted".to_owned(),
        ));
    };

    let mut participant = mem::replace(participant_slot, Box::new(UnavailableParticipant));
    let participant_cancellation = Arc::new(AtomicBool::new(false));
    let deadline =
        (Instant::now() + Duration::from_millis(execution.timeout_ms)).min(plan_deadline);
    let control = SaveParticipantRunControl {
        attempt_cancellation: attempt_cancellation.clone(),
        participant_cancellation: participant_cancellation.clone(),
        deadline,
        max_output_bytes: MAX_PARTICIPANT_STAGED_BYTES,
    };
    let (sender, receiver) = mpsc::sync_channel(1);
    let spawned = std::thread::Builder::new()
        .name("aureline-save-participant".to_owned())
        .spawn(move || {
            let _worker_slot = worker_slot;
            let result = catch_unwind(AssertUnwindSafe(|| {
                participant.run_with_control(&staged, &control)
            }))
            .unwrap_or_else(|_| Err("save participant panicked".to_owned()));
            let _ = sender.send(ParticipantWorkerCompletion {
                participant,
                result,
            });
        });
    if spawned.is_err() {
        return BoundedParticipantRun::Completed(Err(
            "could not start bounded save-participant worker".to_owned(),
        ));
    }

    loop {
        if attempt_cancellation.is_cancelled() {
            participant_cancellation.store(true, Ordering::Release);
            reclaim_after_cancellation(participant_slot, &receiver);
            return BoundedParticipantRun::Cancelled;
        }
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            participant_cancellation.store(true, Ordering::Release);
            reclaim_after_cancellation(participant_slot, &receiver);
            return BoundedParticipantRun::TimedOut;
        }
        let wait = remaining.min(PARTICIPANT_WAIT_SLICE);
        match receiver.recv_timeout(wait) {
            Ok(completion) => {
                *participant_slot = completion.participant;
                if attempt_cancellation.is_cancelled() {
                    return BoundedParticipantRun::Cancelled;
                }
                if Instant::now() >= deadline {
                    return BoundedParticipantRun::TimedOut;
                }
                return BoundedParticipantRun::Completed(completion.result);
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                return BoundedParticipantRun::Completed(Err(
                    "save-participant worker ended without a result".to_owned(),
                ));
            }
        }
    }
}

fn reclaim_after_cancellation(
    participant_slot: &mut Box<dyn SaveParticipant>,
    receiver: &mpsc::Receiver<ParticipantWorkerCompletion>,
) {
    if let Ok(completion) = receiver.recv_timeout(PARTICIPANT_CANCELLATION_GRACE) {
        *participant_slot = completion.participant;
    }
}

fn preflight_block(request: &StagedSaveRequest) -> Option<(SaveOutcome, String)> {
    let token = &request.token;
    if token.capability_flags.read_only
        || token.capability_flags.policy_constrained
        || !token.permission_snapshot.writable
    {
        return Some((
            SaveOutcome::ReadOnlyOrPolicyBlocked,
            "root policy or pinned permission snapshot does not admit writes".to_owned(),
        ));
    }
    if token.atomic_write_mode == AtomicWriteMode::Blocked {
        return Some((
            SaveOutcome::GeneratedOrManagedWriteBlocked,
            "save target token pins a blocked mutation mode".to_owned(),
        ));
    }
    if token.atomic_write_mode == AtomicWriteMode::AtomicReplace
        && (!token.capability_flags.supports_atomic_replace || token.review_required_before_rename)
    {
        let outcome = if token.review_required_before_rename {
            SaveOutcome::ReviewRequiredBeforeRename
        } else {
            SaveOutcome::GeneratedOrManagedWriteBlocked
        };
        return Some((
            outcome,
            "conditional atomic replacement is unavailable or requires rename review".to_owned(),
        ));
    }
    if token.atomic_write_mode == AtomicWriteMode::ConditionalRemoteWrite
        && !token.capability_flags.supports_conditional_remote_write
    {
        return Some((
            SaveOutcome::GeneratedOrManagedWriteBlocked,
            "conditional remote write is not supported by the root".to_owned(),
        ));
    }
    if token.atomic_write_mode == AtomicWriteMode::InPlaceWrite {
        let admission_checkpoint = request
            .reviewed_in_place_admission
            .as_ref()
            .map(|admission| admission.checkpoint_ref());
        if !token.capability_flags.supports_in_place_write
            || admission_checkpoint.is_none()
            || admission_checkpoint != request.checkpoint_ref.as_deref()
        {
            return Some((
                SaveOutcome::ReviewRequiredBeforeSave,
                "in-place save requires an exact root-owned preimage checkpoint and target/content-bound review admission"
                    .to_owned(),
            ));
        }
    } else if token.review_required_before_save {
        return Some((
            SaveOutcome::ReviewRequiredBeforeSave,
            "root requires review before save".to_owned(),
        ));
    }
    None
}

fn monotonic_stamp() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("mono:{nanos}")
}

#[allow(clippy::too_many_arguments)]
fn make_manifest(
    root: &dyn VfsRoot,
    token: &SaveTargetToken,
    save_participant_group_id: Option<String>,
    checkpoint_ref: Option<String>,
    committed_at: String,
    outcome: SaveOutcome,
    failure_detail: Option<String>,
) -> SaveManifest {
    make_manifest_with_observation(
        root,
        token,
        save_participant_group_id,
        checkpoint_ref,
        committed_at,
        outcome,
        failure_detail,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
fn make_manifest_with_observation(
    _root: &dyn VfsRoot,
    token: &SaveTargetToken,
    save_participant_group_id: Option<String>,
    checkpoint_ref: Option<String>,
    committed_at: String,
    outcome: SaveOutcome,
    failure_detail: Option<String>,
    postcommit: Option<&RootPostCommitObservation>,
) -> SaveManifest {
    let canonical_uri = &token.identity.canonical_filesystem_object.canonical_uri;
    let strongest = postcommit
        .map(|observation| observation.strongest_identity_token.clone())
        .unwrap_or_else(|| {
            token
                .identity
                .canonical_filesystem_object
                .strongest_identity_token
                .clone()
        });
    let fallback = postcommit
        .map(|observation| observation.fallback_identity_tokens.clone())
        .unwrap_or_else(|| {
            token
                .identity
                .canonical_filesystem_object
                .fallback_identity_tokens
                .clone()
        });
    let canonical_object = aureline_vfs::CanonicalFilesystemObject {
        canonical_uri: canonical_uri.clone(),
        normalization_form: token
            .identity
            .canonical_filesystem_object
            .normalization_form,
        strongest_identity_token: strongest.clone(),
        fallback_identity_tokens: fallback,
    };
    let generation_token = postcommit
        .map(|observation| observation.generation_token.clone())
        .unwrap_or_else(|| GenerationToken {
            kind: token.compare_before_write_generation_token.kind,
            value: token.compare_before_write_generation_token.value.clone(),
        });
    SaveManifest {
        presentation_path: token.identity.presentation_path.clone(),
        canonical_filesystem_object: canonical_object,
        generation_token,
        capability_mode: token.atomic_write_mode,
        save_participant_group_id,
        checkpoint_ref,
        committed_at,
        outcome,
        failure_detail,
    }
}

fn token_from_postcommit(
    token: &SaveTargetToken,
    observation: &RootPostCommitObservation,
    observed_at: &str,
) -> SaveTargetToken {
    let mut next = token.clone();
    next.identity
        .canonical_filesystem_object
        .strongest_identity_token = observation.strongest_identity_token.clone();
    next.identity
        .canonical_filesystem_object
        .fallback_identity_tokens = observation.fallback_identity_tokens.clone();
    next.compare_before_write_generation_token.kind = observation.generation_token.kind;
    next.compare_before_write_generation_token.value = observation.generation_token.value.clone();
    next.compare_before_write_generation_token.observed_at = observed_at.to_owned();
    next.permission_snapshot = observation.permission_snapshot.clone();
    next
}
