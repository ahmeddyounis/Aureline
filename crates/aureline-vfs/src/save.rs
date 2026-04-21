//! Save-target issuance and conflict-aware save stub.
//!
//! Implements the ADR 0006 save pipeline contract at prototype
//! fidelity:
//!
//! 1. **Open** produces a [`SaveTargetToken`] carrying layers 1-4
//!    of the identity model, the root's capability flags, the
//!    resolved [`AtomicWriteMode`], the generation token observed
//!    at open, and the permission snapshot. A surface that cannot
//!    produce a token MUST NOT offer a save affordance.
//! 2. **Stage** pairs the token with a buffer snapshot in a
//!    [`SaveRequest`].
//! 3. **Compare-before-write** re-reads the canonical object's
//!    strongest generation token and compares it to the token on
//!    the [`SaveTargetToken`]. A mismatch yields
//!    [`SaveOutcome::ExternalChangeDetected`].
//! 4. **Commit** writes through the synthetic root under the
//!    selected mode and records a [`SaveManifest`].
//! 5. **Block** policies (read-only, policy-constrained,
//!    review-required) fail closed with typed outcomes before
//!    any bytes move.
//!
//! The stub does NOT run save participants, does NOT write to
//! disk, and does NOT wire a durable journal. Those land with
//! the production save pipeline; this prototype proves the
//! conflict / degrade / block contract with the same vocabulary
//! the production pipeline will report.

use crate::capabilities::{AtomicWriteMode, CapabilityFlags};
use crate::hooks::HookCounters;
use crate::identity::{CanonicalFilesystemObject, IdentityRecord, IdentityToken};
use crate::synthetic::SyntheticRoot;

/// Kinds a generation token can take. Superset of the
/// strongest-identity-token kinds plus the mtime / remote-revision
/// forms roots use when they cannot provide a generation counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenerationTokenKind {
    FileIdGeneration,
    DeviceInodeGeneration,
    WindowsObjectId,
    ProviderObjectIdRevision,
    InodeMtimeSize,
    RemoteRevisionToken,
    ContentHash,
}

impl GenerationTokenKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FileIdGeneration => "file_id_generation",
            Self::DeviceInodeGeneration => "device_inode_generation",
            Self::WindowsObjectId => "windows_object_id",
            Self::ProviderObjectIdRevision => "provider_object_id_revision",
            Self::InodeMtimeSize => "inode_mtime_size",
            Self::RemoteRevisionToken => "remote_revision_token",
            Self::ContentHash => "content_hash",
        }
    }
}

/// Opaque generation token. Compared by `(kind, value)` equality.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerationToken {
    pub kind: GenerationTokenKind,
    pub value: String,
}

impl GenerationToken {
    pub fn from_identity(token: &IdentityToken) -> Self {
        Self {
            kind: match token.kind {
                crate::capabilities::StrongestIdentityTokenKind::FileIdGeneration => {
                    GenerationTokenKind::FileIdGeneration
                }
                crate::capabilities::StrongestIdentityTokenKind::DeviceInodeGeneration => {
                    GenerationTokenKind::DeviceInodeGeneration
                }
                crate::capabilities::StrongestIdentityTokenKind::WindowsObjectId => {
                    GenerationTokenKind::WindowsObjectId
                }
                crate::capabilities::StrongestIdentityTokenKind::ProviderObjectIdRevision => {
                    GenerationTokenKind::ProviderObjectIdRevision
                }
                crate::capabilities::StrongestIdentityTokenKind::LogicalDocumentIdSourceRefs => {
                    // Virtual / generated document roots have no
                    // generation counter; fall back to a content
                    // hash so compare-before-write still has
                    // something to compare.
                    GenerationTokenKind::ContentHash
                }
                crate::capabilities::StrongestIdentityTokenKind::ContentHashOnly => {
                    GenerationTokenKind::ContentHash
                }
            },
            value: token.value.clone(),
        }
    }
}

/// Compare-before-write generation token, with the producer-local
/// monotonic timestamp at which it was observed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompareBeforeWriteGenerationToken {
    pub kind: GenerationTokenKind,
    pub value: String,
    pub observed_at: String,
}

/// Permission snapshot recorded on a save-target token at open.
/// The production pipeline consults this when it decides whether
/// an in-place write is legal without elevating permissions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionSnapshot {
    pub writable: bool,
    pub mode: String,
    pub owner: Option<String>,
    pub group: Option<String>,
    pub acl_summary: Option<String>,
}

impl PermissionSnapshot {
    pub fn writable_default() -> Self {
        Self {
            writable: true,
            mode: "0644".to_owned(),
            owner: Some("example".to_owned()),
            group: Some("staff".to_owned()),
            acl_summary: None,
        }
    }

    pub fn read_only_default() -> Self {
        Self {
            writable: false,
            mode: "0444".to_owned(),
            owner: Some("root".to_owned()),
            group: Some("staff".to_owned()),
            acl_summary: Some("read-only overlay".to_owned()),
        }
    }
}

/// Layer 5 of the filesystem-identity model. A surface that
/// cannot produce one MUST NOT offer a save affordance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveTargetToken {
    pub identity: IdentityRecord,
    pub capability_flags: CapabilityFlags,
    pub atomic_write_mode: AtomicWriteMode,
    pub compare_before_write_generation_token: CompareBeforeWriteGenerationToken,
    pub permission_snapshot: PermissionSnapshot,
    pub review_required_before_save: bool,
    pub review_required_before_rename: bool,
}

/// Reasons [`open_save_target`] can fail before a token is issued.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenError {
    UnknownPresentation(String),
    MissingGenerationToken(String),
}

impl std::fmt::Display for OpenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownPresentation(uri) => write!(f, "unknown presentation uri: {uri}"),
            Self::MissingGenerationToken(uri) => {
                write!(f, "canonical object missing generation token: {uri}")
            }
        }
    }
}

impl std::error::Error for OpenError {}

/// Open a presentation URI and issue a save-target token against
/// the synthetic root. Increments `vfs_canonicalize` on success
/// (layers 1-4 resolved) and `vfs_alias_converge` when the
/// presentation opens as an alias of the canonical object.
pub fn open_save_target(
    root: &SyntheticRoot,
    presentation_uri: &str,
    observed_at: impl Into<String>,
    counters: &mut HookCounters,
) -> Result<SaveTargetToken, OpenError> {
    let observed_at = observed_at.into();
    let identity = root
        .identity_record(presentation_uri)
        .map_err(|_| OpenError::UnknownPresentation(presentation_uri.to_owned()))?;
    counters.vfs_canonicalize += 1;

    // Alias convergence fires when the opened presentation URI
    // differs from the canonical URI, or when the canonical
    // object already knows about any aliases.
    let presentation_differs_from_canonical =
        identity.presentation_path.uri != identity.canonical_filesystem_object.canonical_uri;
    let object_has_known_aliases = !identity.alias_set.aliases.is_empty();
    if presentation_differs_from_canonical || object_has_known_aliases {
        counters.vfs_alias_converge += 1;
    }

    let strongest = root
        .read_strongest_token(&identity.canonical_filesystem_object.canonical_uri)
        .ok_or_else(|| {
            OpenError::MissingGenerationToken(
                identity.canonical_filesystem_object.canonical_uri.clone(),
            )
        })?;

    let compare_before_write = {
        let gen_token = GenerationToken::from_identity(&strongest);
        CompareBeforeWriteGenerationToken {
            kind: gen_token.kind,
            value: gen_token.value,
            observed_at,
        }
    };

    let permission_snapshot = root
        .permission_snapshot(&identity.canonical_filesystem_object.canonical_uri)
        .unwrap_or_else(PermissionSnapshot::writable_default);

    let envelope = root.envelope();
    let atomic_write_mode = envelope.select_save_mode();

    Ok(SaveTargetToken {
        identity,
        capability_flags: envelope.capability_flags.clone(),
        atomic_write_mode,
        compare_before_write_generation_token: compare_before_write,
        permission_snapshot,
        review_required_before_save: envelope.capability_flags.review_required_before_save,
        review_required_before_rename: envelope.capability_flags.review_required_before_rename,
    })
}

/// Inputs the editor / AI apply / CLI stages into the save
/// pipeline. The prototype models the new bytes + an optional
/// save-participant-group identifier (ADR 0003).
#[derive(Debug, Clone)]
pub struct SaveRequest {
    pub token: SaveTargetToken,
    pub new_content: Vec<u8>,
    pub save_participant_group_id: Option<String>,
    pub checkpoint_ref: Option<String>,
    pub committed_at: String,
    /// If `Some`, a save participant raises this error before the
    /// pipeline reaches compare-before-write. Used by the
    /// participant-failure scenario to reach the
    /// `SaveParticipantFailed` outcome.
    pub participant_failure: Option<String>,
}

/// Frozen save-outcome vocabulary. Mirrors the ADR failure-case
/// taxonomy; `Committed` is the only success state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SaveOutcome {
    Committed,
    ExternalChangeDetected,
    WatcherUncertainty,
    SaveConflict,
    WrongTargetPrevented,
    SaveParticipantFailed,
    DegradedGuaranteeDeclared,
    GeneratedOrManagedWriteBlocked,
    ReadOnlyOrPolicyBlocked,
    ReviewRequiredBeforeSave,
    ReviewRequiredBeforeRename,
}

impl SaveOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Committed => "committed",
            Self::ExternalChangeDetected => "external_change_detected",
            Self::WatcherUncertainty => "watcher_uncertainty",
            Self::SaveConflict => "save_conflict",
            Self::WrongTargetPrevented => "wrong_target_prevented",
            Self::SaveParticipantFailed => "save_participant_failed",
            Self::DegradedGuaranteeDeclared => "degraded_guarantee_declared",
            Self::GeneratedOrManagedWriteBlocked => "generated_or_managed_write_blocked",
            Self::ReadOnlyOrPolicyBlocked => "read_only_or_policy_blocked",
            Self::ReviewRequiredBeforeSave => "review_required_before_save",
            Self::ReviewRequiredBeforeRename => "review_required_before_rename",
        }
    }
}

/// Save manifest recorded on every outcome (committed or failed).
/// The mutation journal and support-bundle exporter quote this
/// record verbatim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveManifest {
    pub presentation_path: crate::identity::PresentationPath,
    pub canonical_filesystem_object: CanonicalFilesystemObject,
    pub generation_token: GenerationToken,
    pub capability_mode: AtomicWriteMode,
    pub save_participant_group_id: Option<String>,
    pub checkpoint_ref: Option<String>,
    pub committed_at: String,
    pub outcome: SaveOutcome,
    pub failure_detail: Option<String>,
}

/// Reviewable save-plan record emitted per scenario. Bundles the
/// identity record, the save-target token, and the save manifest
/// the pipeline produced so a single artifact explains
/// "opened path vs. actual write target vs. rewrite class vs.
/// degraded / unsupported conditions".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavePlan {
    pub label: String,
    pub scenario_summary: String,
    pub identity_record: IdentityRecord,
    pub save_target_token: SaveTargetToken,
    pub save_manifest: SaveManifest,
    /// Watcher-health frames captured while the scenario ran.
    pub watcher_frames: Vec<crate::watcher::WatcherHealthFrame>,
    /// Free-text notes the scenario records for reviewers: which
    /// hook fired for which reason, what the degraded guarantee
    /// is, why the save was blocked, etc.
    pub reviewer_notes: Vec<String>,
}

/// Run the conflict-aware save stub. Mutates the root only on a
/// successful commit (or on a degraded `in_place_write` commit).
///
/// Increments the matching hook counters. Never panics — every
/// failure is a typed [`SaveOutcome`].
pub fn attempt_save(
    root: &mut SyntheticRoot,
    request: SaveRequest,
    counters: &mut HookCounters,
) -> SaveManifest {
    counters.vfs_save_stage += 1;
    let token = &request.token;
    let canonical_uri = token.identity.canonical_filesystem_object.canonical_uri.clone();
    let presentation_path = token.identity.presentation_path.clone();

    // Policy / read-only / review gates short-circuit before any
    // participant runs. Read_only_or_policy_blocked and the two
    // review_required outcomes are fail-closed states named in
    // the ADR's failure-case taxonomy.
    if token.capability_flags.read_only || token.capability_flags.policy_constrained {
        counters.vfs_save_blocked += 1;
        counters.vfs_save_manifest_record += 1;
        return make_manifest(
            presentation_path,
            root,
            &canonical_uri,
            token,
            request.save_participant_group_id,
            request.checkpoint_ref,
            request.committed_at,
            SaveOutcome::ReadOnlyOrPolicyBlocked,
            Some("root advertises read_only or policy_constrained; save_mode is blocked".to_owned()),
        );
    }
    if token.review_required_before_save {
        counters.vfs_save_blocked += 1;
        counters.vfs_save_manifest_record += 1;
        return make_manifest(
            presentation_path,
            root,
            &canonical_uri,
            token,
            request.save_participant_group_id,
            request.checkpoint_ref,
            request.committed_at,
            SaveOutcome::ReviewRequiredBeforeSave,
            Some("root advertises review_required_before_save; pipeline halts at stage 3".to_owned()),
        );
    }
    if token.atomic_write_mode == AtomicWriteMode::Blocked {
        counters.vfs_save_blocked += 1;
        counters.vfs_save_manifest_record += 1;
        return make_manifest(
            presentation_path,
            root,
            &canonical_uri,
            token,
            request.save_participant_group_id,
            request.checkpoint_ref,
            request.committed_at,
            SaveOutcome::ReadOnlyOrPolicyBlocked,
            Some("save_target_token pinned atomic_write_mode = blocked".to_owned()),
        );
    }

    // Run one save participant; fail closed on participant error.
    counters.vfs_save_participant_run += 1;
    if let Some(failure_detail) = request.participant_failure {
        counters.vfs_save_participant_failed += 1;
        counters.vfs_save_manifest_record += 1;
        return make_manifest(
            presentation_path,
            root,
            &canonical_uri,
            token,
            request.save_participant_group_id,
            request.checkpoint_ref,
            request.committed_at,
            SaveOutcome::SaveParticipantFailed,
            Some(failure_detail),
        );
    }

    // Compare-before-write against a fresh read of the canonical
    // object. The pinned token is what the editor captured at
    // open; the current token is what is on disk right now.
    counters.vfs_save_compare_before_write += 1;
    let current_strongest = match root.read_strongest_token(&canonical_uri) {
        Some(t) => t,
        None => {
            // Canonical object disappeared since open.
            counters.vfs_save_conflict += 1;
            counters.vfs_save_manifest_record += 1;
            return make_manifest(
                presentation_path,
                root,
                &canonical_uri,
                token,
                request.save_participant_group_id,
                request.checkpoint_ref,
                request.committed_at,
                SaveOutcome::WrongTargetPrevented,
                Some("canonical object missing at save time".to_owned()),
            );
        }
    };
    let pinned = &token.compare_before_write_generation_token;
    let current_generation = GenerationToken::from_identity(&current_strongest);
    if pinned.value != current_generation.value || pinned.kind != current_generation.kind {
        counters.vfs_external_change_detected += 1;
        counters.vfs_save_conflict += 1;
        counters.vfs_save_manifest_record += 1;
        // The save_conflict outcome is used by roots that rely on
        // conditional remote writes; external_change_detected by
        // local roots that compare generation counters.
        let outcome = if token.atomic_write_mode == AtomicWriteMode::ConditionalRemoteWrite {
            SaveOutcome::SaveConflict
        } else {
            SaveOutcome::ExternalChangeDetected
        };
        let detail = format!(
            "generation_token_mismatch: pinned {pinned_value} observed {observed_value}",
            pinned_value = pinned.value,
            observed_value = current_generation.value,
        );
        return make_manifest(
            presentation_path,
            root,
            &canonical_uri,
            token,
            request.save_participant_group_id,
            request.checkpoint_ref,
            request.committed_at,
            outcome,
            Some(detail),
        );
    }

    // Commit — mode-specific counter fires. Atomic replace is the
    // only path without a degraded banner; in-place writes and
    // conditional remote writes each emit
    // vfs_degraded_guarantee_declared.
    match token.atomic_write_mode {
        AtomicWriteMode::AtomicReplace => {
            counters.vfs_save_atomic_commit += 1;
        }
        AtomicWriteMode::InPlaceWrite => {
            counters.vfs_save_in_place_commit += 1;
            counters.vfs_degraded_guarantee_declared += 1;
        }
        AtomicWriteMode::ConditionalRemoteWrite => {
            counters.vfs_save_remote_conditional_commit += 1;
            counters.vfs_degraded_guarantee_declared += 1;
        }
        AtomicWriteMode::Blocked => unreachable!("blocked already handled above"),
    }
    root.apply_commit(&canonical_uri, request.new_content);
    counters.vfs_save_manifest_record += 1;

    make_manifest(
        presentation_path,
        root,
        &canonical_uri,
        token,
        request.save_participant_group_id,
        request.checkpoint_ref,
        request.committed_at,
        SaveOutcome::Committed,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
fn make_manifest(
    presentation_path: crate::identity::PresentationPath,
    root: &SyntheticRoot,
    canonical_uri: &str,
    token: &SaveTargetToken,
    save_participant_group_id: Option<String>,
    checkpoint_ref: Option<String>,
    committed_at: String,
    outcome: SaveOutcome,
    failure_detail: Option<String>,
) -> SaveManifest {
    let strongest = root.read_strongest_token(canonical_uri).unwrap_or_else(|| IdentityToken {
        kind: token
            .identity
            .canonical_filesystem_object
            .strongest_identity_token
            .kind,
        value: token
            .identity
            .canonical_filesystem_object
            .strongest_identity_token
            .value
            .clone(),
    });
    let canonical_object = CanonicalFilesystemObject {
        canonical_uri: canonical_uri.to_owned(),
        normalization_form: token
            .identity
            .canonical_filesystem_object
            .normalization_form,
        strongest_identity_token: strongest.clone(),
        fallback_identity_tokens: root.fallback_tokens(canonical_uri),
    };
    SaveManifest {
        presentation_path,
        canonical_filesystem_object: canonical_object,
        generation_token: GenerationToken::from_identity(&strongest),
        capability_mode: token.atomic_write_mode,
        save_participant_group_id,
        checkpoint_ref,
        committed_at,
        outcome,
        failure_detail,
    }
}
