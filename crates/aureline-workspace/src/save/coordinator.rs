//! Staged save coordinator.
//!
//! The coordinator is the single entry point for committing buffer snapshots to
//! durable storage. It enforces compare-before-write, runs save participants on
//! staged content, and selects an atomic (or explicitly degraded) write lane
//! based on the VFS root capability envelope.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use aureline_vfs::save::open_save_target;
use aureline_vfs::{
    AtomicWriteMode, GenerationToken, GenerationTokenKind, HookCounters, SaveManifest, SaveOutcome,
    SaveTargetToken, VfsRoot, VfsUri,
};

use super::drift_detection::detect_external_drift;
use super::risk::{
    summarize_staged_file_effect, SaveParticipantRiskDeclaration, SaveParticipantRiskOutcomeClass,
    SaveParticipantRiskReview,
};
use super::source_fidelity::{encode_for_save, source_fidelity_adjustments, SourceFidelityRecord};
use super::write_strategy::{select_write_strategy, WriteStrategy};

/// A transformation step that can run on staged save content.
pub trait SaveParticipant {
    /// Returns the stable id for this participant.
    fn participant_id(&self) -> &'static str;

    /// Returns the participant's risk declaration before staged mutation.
    fn risk_declaration(&self) -> SaveParticipantRiskDeclaration {
        SaveParticipantRiskDeclaration::safe_local(self.participant_id())
    }

    /// Runs the participant on the staged content and returns the resulting bytes.
    fn run(&mut self, staged: &[u8]) -> Result<Vec<u8>, String>;
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

/// Inputs to one staged save attempt.
#[derive(Debug, Clone)]
pub struct StagedSaveRequest {
    pub token: SaveTargetToken,
    pub new_content: Vec<u8>,
    pub source_fidelity: SourceFidelityRecord,
    pub save_participant_group_id: Option<String>,
    pub checkpoint_ref: Option<String>,
    pub committed_at: String,
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
#[derive(Debug, Default, Clone)]
pub struct StagedSaveCoordinator {
    next_packet_seq: u64,
}

impl StagedSaveCoordinator {
    /// Creates a new staged save coordinator.
    pub fn new() -> Self {
        Self { next_packet_seq: 1 }
    }

    /// Runs the staged save pipeline, returning a typed [`SaveResult`] for
    /// both success and failure outcomes.
    pub fn save(
        &mut self,
        root: &mut dyn VfsRoot,
        request: StagedSaveRequest,
        participants: &mut [Box<dyn SaveParticipant>],
    ) -> SaveResult {
        let packet_id = self.mint_packet_id();
        let source_fidelity = request.source_fidelity.clone();
        let mut staged = request.new_content;
        let participant_count = participants.len();
        let declarations: Vec<_> = participants
            .iter()
            .map(|participant| participant.risk_declaration())
            .collect();
        let mut risk_review = SaveParticipantRiskReview::open(
            format!("{packet_id}:save_participant_risk"),
            packet_id.clone(),
            request.checkpoint_ref.clone(),
            declarations,
        );

        if risk_review.outcome_class
            == SaveParticipantRiskOutcomeClass::ReviewRequiredBeforeMutation
        {
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
            };
        }

        let mut participant_error: Option<SaveParticipantError> = None;

        for participant in participants.iter_mut() {
            let participant_id = participant.participant_id();
            let before = staged.clone();
            match participant.run(&staged) {
                Ok(next) => {
                    let actual = summarize_staged_file_effect(&before, &next);
                    risk_review.record_actual_effect(participant_id, actual);
                    staged = next;
                    if risk_review.outcome_class
                        == SaveParticipantRiskOutcomeClass::ReviewRequiredBeforeCommit
                    {
                        let token = request.token;
                        let manifest = make_manifest(
                            root,
                            &token,
                            request.save_participant_group_id,
                            request.checkpoint_ref,
                            request.committed_at,
                            SaveOutcome::ReviewRequiredBeforeSave,
                            Some(
                                "save participant output widened to review-required write"
                                    .to_owned(),
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
                        };
                    }
                }
                Err(detail) => {
                    participant_error = Some(SaveParticipantError {
                        participant_id: participant_id.to_owned(),
                        detail,
                    });
                    risk_review.mark_participant_failed(participant_id);
                    break;
                }
            }
        }

        let token = request.token;
        if let Some(err) = participant_error.clone() {
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
            };
        }

        let write_strategy = select_write_strategy(&token);
        let canonical_uri = token
            .identity
            .canonical_filesystem_object
            .canonical_uri
            .clone();

        // Root capability gates fail closed before any target resolution or bytes move.
        if token.capability_flags.read_only || token.capability_flags.policy_constrained {
            let manifest = make_manifest(
                root,
                &token,
                request.save_participant_group_id,
                request.checkpoint_ref,
                request.committed_at,
                SaveOutcome::ReadOnlyOrPolicyBlocked,
                Some(
                    "root advertises read_only or policy_constrained; save_mode is blocked"
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
            };
        }

        if token.review_required_before_save {
            let manifest = make_manifest(
                root,
                &token,
                request.save_participant_group_id,
                request.checkpoint_ref,
                request.committed_at,
                SaveOutcome::ReviewRequiredBeforeSave,
                Some(
                    "root advertises review_required_before_save; pipeline halts before commit"
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
            };
        }

        if token.atomic_write_mode == AtomicWriteMode::Blocked {
            let manifest = make_manifest(
                root,
                &token,
                request.save_participant_group_id,
                request.checkpoint_ref,
                request.committed_at,
                SaveOutcome::ReadOnlyOrPolicyBlocked,
                Some("save_target_token pinned atomic_write_mode = blocked".to_owned()),
            );
            return SaveResult {
                packet_id,
                write_strategy,
                manifest,
                source_fidelity: source_fidelity.clone(),
                save_participant_risk_review: risk_review,
                next_token: token,
                participant_error: None,
            };
        }

        if token.atomic_write_mode == AtomicWriteMode::AtomicReplace
            && token.review_required_before_rename
        {
            let manifest = make_manifest(
                root,
                &token,
                request.save_participant_group_id,
                request.checkpoint_ref,
                request.committed_at,
                SaveOutcome::ReviewRequiredBeforeRename,
                Some(
                    "root advertises review_required_before_rename; atomic_replace requires rename"
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
            };
        }

        // File-level permission gate: even when a root is writable in
        // aggregate, a specific target may not be writable.
        if !token.permission_snapshot.writable {
            let manifest = make_manifest(
                root,
                &token,
                request.save_participant_group_id,
                request.checkpoint_ref,
                request.committed_at,
                SaveOutcome::ReadOnlyOrPolicyBlocked,
                Some("permission_snapshot marks the target as not writable".to_owned()),
            );
            return SaveResult {
                packet_id,
                write_strategy,
                manifest,
                source_fidelity: source_fidelity.clone(),
                save_participant_risk_review: risk_review,
                next_token: token,
                participant_error: None,
            };
        }

        if let Err(conflict) = detect_external_drift(root, &token) {
            risk_review
                .mark_external_change(format!("external_change:{}", conflict.outcome.as_str()));
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
                source_fidelity: source_fidelity.clone(),
                save_participant_risk_review: risk_review,
                next_token: token,
                participant_error: None,
            };
        }

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
                };
            }
        };

        // Commit under the selected write strategy.
        let mut failure_detail: Option<String> = None;
        let commit_ok = match write_strategy {
            WriteStrategy::AtomicReplace => commit_atomic_replace(root, &canonical_uri, &staged)
                .map_err(|err| err.to_string())
                .map(|_| ()),
            WriteStrategy::InPlaceWrite => commit_in_place(root, &canonical_uri, &staged)
                .map_err(|err| err.to_string())
                .map(|_| ()),
            WriteStrategy::ConditionalRemoteWrite => root
                .write_bytes(&canonical_uri, staged.clone())
                .map_err(|err| err.to_string()),
            WriteStrategy::Blocked => {
                Err("save target is blocked by capability or policy".to_owned())
            }
        };

        let outcome = match commit_ok {
            Ok(()) => match write_strategy {
                WriteStrategy::AtomicReplace => SaveOutcome::Committed,
                WriteStrategy::InPlaceWrite | WriteStrategy::ConditionalRemoteWrite => {
                    SaveOutcome::DegradedGuaranteeDeclared
                }
                WriteStrategy::Blocked => SaveOutcome::ReadOnlyOrPolicyBlocked,
            },
            Err(detail) => {
                failure_detail = Some(detail);
                SaveOutcome::ReadOnlyOrPolicyBlocked
            }
        };

        let manifest = make_manifest(
            root,
            &token,
            request.save_participant_group_id.clone(),
            request.checkpoint_ref.clone(),
            request.committed_at.clone(),
            outcome,
            failure_detail.clone(),
        );
        if matches!(
            outcome,
            SaveOutcome::Committed | SaveOutcome::DegradedGuaranteeDeclared
        ) {
            risk_review.mark_committed();
        } else {
            risk_review.mark_blocked_no_write(
                failure_detail
                    .clone()
                    .unwrap_or_else(|| "save did not commit durable bytes".to_owned()),
            );
        }

        // After commit, refresh the token so the next save attempts compare
        // against the new on-disk generation token and updated identity.
        let next_token = if matches!(
            outcome,
            SaveOutcome::Committed | SaveOutcome::DegradedGuaranteeDeclared
        ) {
            refresh_token_after_commit(root, &token.identity.presentation_path.uri)
                .unwrap_or(token.clone())
        } else {
            token.clone()
        };

        SaveResult {
            packet_id,
            write_strategy,
            manifest,
            source_fidelity,
            save_participant_risk_review: risk_review,
            next_token,
            participant_error: None,
        }
    }

    fn mint_packet_id(&mut self) -> String {
        let seq = self.next_packet_seq;
        self.next_packet_seq = self.next_packet_seq.saturating_add(1);
        let stamp = monotonic_stamp();
        format!("save_packet:{stamp}:{seq}")
    }
}

fn refresh_token_after_commit(
    root: &dyn VfsRoot,
    presentation_uri: &VfsUri,
) -> Result<SaveTargetToken, String> {
    let mut counters = HookCounters::default();
    open_save_target(root, presentation_uri, monotonic_stamp(), &mut counters)
        .map_err(|e| e.to_string())
}

fn commit_atomic_replace(
    root: &mut dyn VfsRoot,
    canonical_uri: &VfsUri,
    new_content: &[u8],
) -> Result<(), CommitError> {
    let Some(path) = canonical_uri.file_path() else {
        return root
            .write_bytes(canonical_uri, new_content.to_vec())
            .map_err(CommitError::from_root);
    };
    atomic_replace_on_disk(&path, new_content).map_err(|err| CommitError::io(path, err))
}

fn commit_in_place(
    root: &mut dyn VfsRoot,
    canonical_uri: &VfsUri,
    new_content: &[u8],
) -> Result<(), CommitError> {
    let Some(path) = canonical_uri.file_path() else {
        return root
            .write_bytes(canonical_uri, new_content.to_vec())
            .map_err(CommitError::from_root);
    };
    in_place_write_on_disk(&path, new_content).map_err(|err| CommitError::io(path, err))
}

#[derive(Debug, Clone)]
struct CommitError {
    path: Option<PathBuf>,
    detail: String,
}

impl CommitError {
    fn io(path: PathBuf, err: std::io::Error) -> Self {
        Self {
            path: Some(path),
            detail: err.to_string(),
        }
    }

    fn from_root(err: aureline_vfs::RootIoError) -> Self {
        Self {
            path: None,
            detail: err.to_string(),
        }
    }
}

impl std::fmt::Display for CommitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path) = &self.path {
            write!(
                f,
                "commit failed for {path:?}: {detail}",
                detail = self.detail
            )
        } else {
            write!(f, "commit failed: {detail}", detail = self.detail)
        }
    }
}

impl std::error::Error for CommitError {}

fn atomic_replace_on_disk(path: &Path, new_content: &[u8]) -> std::io::Result<()> {
    let parent_dir = path.parent().unwrap_or_else(|| Path::new("."));
    let temp_path =
        allocate_sibling_temp_path(parent_dir, path.file_name().and_then(|n| n.to_str()));

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp_path)?;
    file.write_all(new_content)?;

    if let Ok(meta) = std::fs::metadata(path) {
        let _ = std::fs::set_permissions(&temp_path, meta.permissions());
    }

    file.sync_all()?;
    drop(file);

    std::fs::rename(&temp_path, path)?;
    let _ = sync_parent_dir(parent_dir);
    Ok(())
}

fn in_place_write_on_disk(path: &Path, new_content: &[u8]) -> std::io::Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)?;
    file.write_all(new_content)?;
    file.sync_all()?;
    Ok(())
}

fn allocate_sibling_temp_path(parent_dir: &Path, hint: Option<&str>) -> PathBuf {
    let pid = std::process::id();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let hint = hint.unwrap_or("untitled");
    for attempt in 0..128u32 {
        let candidate = parent_dir.join(format!(".aureline-save.{hint}.{pid}.{now}.{attempt}.tmp"));
        if !candidate.exists() {
            return candidate;
        }
    }
    parent_dir.join(format!(".aureline-save.{hint}.{pid}.{now}.overflow.tmp"))
}

fn sync_parent_dir(parent_dir: &Path) -> std::io::Result<()> {
    // Best-effort durability barrier for the directory entry. Not all
    // platforms allow opening directories as files.
    let file = std::fs::File::open(parent_dir)?;
    file.sync_all()
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
    let canonical_uri = &token.identity.canonical_filesystem_object.canonical_uri;
    let strongest = root
        .read_strongest_identity_token(canonical_uri)
        .unwrap_or_else(|_| {
            token
                .identity
                .canonical_filesystem_object
                .strongest_identity_token
                .clone()
        });
    let fallback = root
        .read_fallback_identity_tokens(canonical_uri)
        .unwrap_or_else(|_| {
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
    let generation_token = root
        .read_generation_token(canonical_uri)
        .unwrap_or(GenerationToken {
            kind: GenerationTokenKind::ContentHash,
            value: "missing".to_owned(),
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
