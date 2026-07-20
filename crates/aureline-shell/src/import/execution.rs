// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

//! Fail-closed first-run import execution for conservative profile settings.
//!
//! This adapter intentionally owns a much narrower surface than source IDEs
//! expose. It reads only bounded, declarative settings files; turns every
//! admitted, unsupported, and authority-bearing item into a review row; and
//! writes only immutable revisions of a dedicated imported profile. A retained
//! checkpoint makes an interrupted apply resumable, while create-new revision
//! publication fences concurrent or resumed stale writers without crash-stale
//! lock files. Tasks, launch state, extensions, secrets, paths, trust, network,
//! credentials, subprocesses, and automation authority never cross this
//! boundary. Durable state becomes user-effective only after shell bootstrap
//! explicitly installs its validated `ImportedProfileDefault` resolver overlay.

use std::collections::BTreeMap;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

use aureline_settings::resolver::ScopeOverlay;
use aureline_settings::schema::{SchemaRegistry, SettingScope, SettingValue};

use super::{CompetitorConfigClassification, ImportReviewDecisionClass, ImportReviewRecord};

/// Schema version of the executable preview and dedicated profile state.
pub const IMPORT_EXECUTION_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for executable import previews.
pub const IMPORT_EXECUTION_PREVIEW_RECORD_KIND: &str = "first_run_import_execution_preview_record";

/// Stable record kind for dedicated imported-profile state.
pub const IMPORTED_PROFILE_STATE_RECORD_KIND: &str = "imported_profile_state_record";

/// Stable record kind for durable pre-apply checkpoints.
pub const IMPORT_EXECUTION_CHECKPOINT_RECORD_KIND: &str =
    "first_run_import_execution_checkpoint_record";

const MAX_SOURCE_FILE_BYTES: u64 = 64 * 1024;
const MAX_DURABLE_FILE_BYTES: u64 = 1024 * 1024;
const MAX_HISTORY_ROWS: usize = 1024;
const ABSENT_STATE_DIGEST: &str = "state:absent";

/// A schema-safe setting value admitted into imported-profile state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "value_kind", content = "value", rename_all = "snake_case")]
pub enum ImportSettingValue {
    /// Boolean preference.
    Boolean(bool),
    /// Bounded integer preference.
    Integer(i64),
    /// Bounded, non-control text preference.
    Text(String),
}

/// Review decision for one parsed source item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportExecutionRowDecision {
    /// The reviewed row may mutate the dedicated imported profile.
    Admitted,
    /// The target already carries this exact value.
    NoChange,
    /// A different source owns the target value, so explicit conflict review is required.
    ManualReview,
    /// The source concept has no supported mapping in this conservative adapter.
    Unsupported,
    /// The source concept could widen authority and is categorically excluded.
    BlockedAuthority,
}

impl ImportExecutionRowDecision {
    /// Stable serialized token for compact shell status lines.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::NoChange => "no_change",
            Self::ManualReview => "manual_review",
            Self::Unsupported => "unsupported",
            Self::BlockedAuthority => "blocked_authority",
        }
    }
}

/// One exact body-derived row shown before import apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportExecutionRow {
    /// Stable row identity derived from source and target refs.
    pub row_ref: String,
    /// Redaction-safe source file and key ref. Sensitive key names are hashed.
    pub source_setting_ref: String,
    /// Target setting id, absent for excluded rows.
    pub target_setting_id: Option<String>,
    /// Existing value in the dedicated target profile.
    pub before_value: Option<ImportSettingValue>,
    /// Parsed value proposed by the source adapter.
    pub after_value: Option<ImportSettingValue>,
    /// Apply decision for this row.
    pub decision: ImportExecutionRowDecision,
    /// Stable reason code suitable for logs and support projections.
    pub reason_code: String,
}

/// Apply gate computed from body-derived rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportPreviewApplyGate {
    /// At least one admitted mutation can be checkpointed and applied.
    AllowedCheckpointRequired,
    /// Every supported row already matches the target.
    NoChanges,
    /// No supported setting can be applied.
    BlockedNoSupportedSettings,
}

impl ImportPreviewApplyGate {
    /// Stable serialized token for UI and command results.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllowedCheckpointRequired => "allowed_checkpoint_required",
            Self::NoChanges => "no_changes",
            Self::BlockedNoSupportedSettings => "blocked_no_supported_settings",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LivePreviewAuthority {
    canonical_source_root: PathBuf,
    profile_key: String,
    source_snapshot_digest: String,
    target_state_digest: String,
    plan_digest: String,
    generated_at: String,
}

/// Serializable review packet. Deserialization deliberately strips apply authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutableImportPreview {
    /// Stable record kind.
    pub record_kind: String,
    /// Record schema version.
    pub schema_version: u32,
    /// Stable preview ref derived from exact source and target state.
    pub preview_ref: String,
    /// Lightweight review record that opened this execution preview.
    pub import_review_ref: String,
    /// Redaction-safe, content-addressed source-root ref.
    pub source_root_ref: String,
    /// Supported source ecosystem.
    pub source_classification: CompetitorConfigClassification,
    /// Dedicated imported-profile ref; never interpreted as a filesystem path.
    pub target_profile_ref: String,
    /// Policy/trust epoch used for planning.
    pub policy_epoch: String,
    /// Exact parsed rows, including unsupported and blocked concepts.
    pub rows: Vec<ImportExecutionRow>,
    /// Apply posture computed from the rows.
    pub apply_gate: ImportPreviewApplyGate,
    /// Digest of the bounded source files and excluded-path presence map.
    pub source_snapshot_digest: String,
    /// Digest of target state at preview time, or `state:absent`.
    pub target_state_digest: String,
    /// Digest binding source, target, policy epoch, and exact rows.
    pub plan_digest: String,
    /// Live UTC timestamp captured when the preview was built.
    pub generated_at: String,
    /// In-process authority is never serialized or accepted from exported packets.
    #[serde(skip, default)]
    live_authority: Option<LivePreviewAuthority>,
}

impl ExecutableImportPreview {
    /// Number of rows that will mutate imported-profile state.
    pub fn admitted_mutation_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.decision == ImportExecutionRowDecision::Admitted)
            .count()
    }

    /// Number of rows excluded because they could widen authority.
    pub fn blocked_authority_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.decision == ImportExecutionRowDecision::BlockedAuthority)
            .count()
    }

    /// True only for an in-process preview with live revalidation evidence.
    pub fn carries_live_apply_authority(&self) -> bool {
        self.live_authority.is_some()
    }
}

/// Durable owner and provenance for one imported setting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportedProfileSettingRecord {
    /// Imported value.
    pub value: ImportSettingValue,
    /// Source ecosystem that owns this imported baseline.
    pub source_classification: CompetitorConfigClassification,
    /// Redaction-safe source setting ref.
    pub source_setting_ref: String,
    /// Live UTC timestamp of the last admitted import.
    pub imported_at: String,
}

/// Durable import or rollback history action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportedProfileHistoryAction {
    /// Reviewed import apply.
    Applied,
    /// Restore through a retained checkpoint.
    RolledBack,
}

/// One privacy-safe durable imported-profile history row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportedProfileHistoryEntry {
    /// Action represented by the row.
    pub action: ImportedProfileHistoryAction,
    /// Digest of the caller-provided idempotency token; the token is never persisted.
    pub idempotency_token_digest: String,
    /// Exact executable preview that admitted the apply.
    pub preview_ref: String,
    /// Lightweight source/destination review that opened the preview.
    pub import_review_ref: String,
    /// Redaction-safe content-addressed source-root ref.
    pub source_root_ref: String,
    /// Exact bounded source snapshot consumed by apply.
    pub source_snapshot_digest: String,
    /// Policy/trust epoch revalidated before apply.
    pub policy_epoch: String,
    /// Plan digest for apply, or original plan digest for rollback.
    pub plan_digest: String,
    /// Checkpoint protecting or restoring this action.
    pub checkpoint_ref: String,
    /// Effective-settings digest after the action.
    pub result_settings_digest: String,
    /// Target setting ids changed by this action; no values are duplicated here.
    pub changed_setting_ids: Vec<String>,
    /// Live UTC action timestamp.
    pub occurred_at: String,
}

/// Dedicated durable state written by this importer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportedProfileState {
    /// Stable record kind.
    pub record_kind: String,
    /// Record schema version.
    pub schema_version: u32,
    /// Content-addressed imported-profile ref.
    pub profile_ref: String,
    /// Monotonic local revision.
    pub revision: u64,
    /// Imported settings keyed by canonical Aureline setting id.
    pub settings: BTreeMap<String, ImportedProfileSettingRecord>,
    /// Durable apply and rollback history retained after first run.
    pub history: Vec<ImportedProfileHistoryEntry>,
    /// Live UTC timestamp of the latest mutation.
    pub updated_at: String,
}

impl ImportedProfileState {
    /// Projects this durable state into the real resolver's imported-profile layer.
    ///
    /// Projection is read-only: the caller must explicitly install the returned
    /// overlay on the live resolver with `EffectiveSettingsResolver::set_overlay`.
    /// Every setting is revalidated against the supplied canonical registry, so
    /// an unknown id, disallowed scope, or incompatible value fails closed.
    pub fn to_resolver_overlay(
        &self,
        registry: &SchemaRegistry,
    ) -> Result<ScopeOverlay, ImportExecutionError> {
        let profile_key = self.profile_ref.strip_prefix("imported-profile:").ok_or(
            ImportExecutionError::ResolverProjectionUnavailable {
                setting_id: "profile".to_owned(),
                reason_code: "profile_ref_invalid",
            },
        )?;
        validate_profile_key(profile_key)?;
        validate_durable_state(self, &self.profile_ref)?;

        let mut overlay = ScopeOverlay::new(
            SettingScope::ImportedProfileDefault,
            "Reviewed imported profile",
        );
        for (setting_id, record) in &self.settings {
            let definition = registry.definition(setting_id).ok_or_else(|| {
                ImportExecutionError::ResolverProjectionUnavailable {
                    setting_id: setting_id.clone(),
                    reason_code: "setting_not_registered",
                }
            })?;
            if !definition.allows_scope(SettingScope::ImportedProfileDefault) {
                return Err(ImportExecutionError::ResolverProjectionUnavailable {
                    setting_id: setting_id.clone(),
                    reason_code: "imported_profile_scope_not_allowed",
                });
            }
            let value = resolver_value(&record.value);
            definition.validate_value(&value).map_err(|_| {
                ImportExecutionError::ResolverProjectionUnavailable {
                    setting_id: setting_id.clone(),
                    reason_code: "setting_value_invalid",
                }
            })?;
            overlay.set_value(setting_id.clone(), value);
        }
        Ok(overlay)
    }
}

/// Whether an apply changed state or replayed an idempotent result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportApplyDisposition {
    /// A checkpoint was created and admitted settings were written.
    Applied,
    /// The exact token and plan had already produced the current effective state.
    AlreadyApplied,
    /// The reviewed source values already matched the target.
    NoChanges,
}

/// Result of a reviewed import apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportApplyOutcome {
    /// Apply disposition.
    pub disposition: ImportApplyDisposition,
    /// Target imported-profile ref.
    pub target_profile_ref: String,
    /// Reviewed plan digest.
    pub plan_digest: String,
    /// Retained checkpoint ref, absent for a pure no-op.
    pub checkpoint_ref: Option<String>,
    /// Effective-settings digest after the operation.
    pub result_settings_digest: String,
    /// Durable state revision after the operation.
    pub revision: u64,
    /// Live UTC outcome timestamp.
    pub completed_at: String,
}

/// Input to one-step rollback.
#[derive(Debug, Clone, Copy)]
pub struct ImportRollbackRequest<'a> {
    /// Checkpoint ref returned by apply.
    pub checkpoint_ref: &'a str,
    /// Idempotency token for this rollback request.
    pub idempotency_token: &'a str,
}

/// Result of restoring a retained import checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportRollbackOutcome {
    /// True when this call performed the restore; false on idempotent replay.
    pub restored_now: bool,
    /// Restored checkpoint ref.
    pub checkpoint_ref: String,
    /// Target imported-profile ref.
    pub target_profile_ref: String,
    /// Effective-settings digest after restore.
    pub result_settings_digest: String,
    /// Durable state revision after restore.
    pub revision: u64,
    /// Live UTC outcome timestamp.
    pub completed_at: String,
}

/// Fail-closed import execution error. Messages never contain raw paths or values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportExecutionError {
    /// A request field is missing, too long, or contains unsafe control data.
    InvalidRequest { reason_code: &'static str },
    /// The selected source family is not executable by this adapter.
    UnsupportedSource,
    /// The source root or one of its expected entries is unavailable.
    SourceUnavailable { source_item_ref: String },
    /// A symlink, escape, or unexpected filesystem object was observed.
    UnsafeSourceLayout { source_item_ref: String },
    /// A bounded source or durable file exceeded its limit.
    InputTooLarge { source_item_ref: String },
    /// A source body was invalid or ambiguous.
    MalformedInput {
        source_item_ref: String,
        reason_code: &'static str,
    },
    /// Durable imported-profile state is unavailable or invalid.
    DurableStateUnavailable { reason_code: &'static str },
    /// Durable state cannot be projected into the canonical settings resolver.
    ResolverProjectionUnavailable {
        setting_id: String,
        reason_code: &'static str,
    },
    /// Exported/deserialized preview packets do not carry apply authority.
    PreviewAuthorityMissing,
    /// Source bytes, parsed rows, or target state changed since preview.
    PreviewStale { reason_code: &'static str },
    /// Current policy/trust epoch no longer matches the reviewed preview.
    PolicyEpochChanged,
    /// Preview contains no admitted state change.
    ApplyNotAllowed,
    /// The idempotency token was previously used for a different effective result.
    IdempotencyConflict,
    /// Another process won the immutable next-revision compare-and-publish fence.
    ConcurrentMutation,
    /// The requested checkpoint does not exist or is invalid.
    CheckpointUnavailable,
    /// Target settings changed after apply, so rollback cannot discard them.
    RollbackConflict,
}

impl fmt::Display for ImportExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidRequest { .. } => "invalid import execution request",
            Self::UnsupportedSource => "unsupported import source",
            Self::SourceUnavailable { .. } => "import source is unavailable",
            Self::UnsafeSourceLayout { .. } => "unsafe import source layout",
            Self::InputTooLarge { .. } => "import input exceeds the bounded size limit",
            Self::MalformedInput { .. } => "import input is malformed or ambiguous",
            Self::DurableStateUnavailable { .. } => "imported-profile state is unavailable",
            Self::ResolverProjectionUnavailable { .. } => {
                "imported-profile state is incompatible with the settings resolver"
            }
            Self::PreviewAuthorityMissing => "preview does not carry live apply authority",
            Self::PreviewStale { .. } => "preview evidence is stale; refresh before apply",
            Self::PolicyEpochChanged => "policy or trust epoch changed; refresh before apply",
            Self::ApplyNotAllowed => "preview has no admitted import changes",
            Self::IdempotencyConflict => "idempotency token conflicts with durable history",
            Self::ConcurrentMutation => "another imported-profile mutation committed first",
            Self::CheckpointUnavailable => "import rollback checkpoint is unavailable",
            Self::RollbackConflict => "target changed after import; rollback needs review",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for ImportExecutionError {}

/// Local durable store for imported-profile settings and checkpoints.
#[derive(Debug, Clone)]
pub struct ImportedProfileStore {
    state_root: PathBuf,
}

impl ImportedProfileStore {
    /// Creates a store below the caller-owned application state root.
    pub fn new(state_root: impl Into<PathBuf>) -> Self {
        Self {
            state_root: state_root.into(),
        }
    }

    /// Builds an exact body-derived preview without writing durable state.
    pub fn preview(
        &self,
        review: &ImportReviewRecord,
        policy_epoch: &str,
    ) -> Result<ExecutableImportPreview, ImportExecutionError> {
        validate_policy_epoch(policy_epoch)?;
        validate_import_review_envelope(review)?;
        validate_request_label(
            &review.destination_workspace_target,
            "destination_target_invalid",
        )?;
        if review.decision_class != ImportReviewDecisionClass::ApplyAfterPreview {
            return Err(ImportExecutionError::UnsupportedSource);
        }

        let secured = secure_source_root(Path::new(&review.source_path))?;
        if secured.classification != review.classification {
            return Err(ImportExecutionError::PreviewStale {
                reason_code: "source_classification_changed",
            });
        }
        let profile_key = profile_key(&review.destination_workspace_target);
        let target_profile_ref = format!("imported-profile:{profile_key}");
        let current = self.load_profile_by_key(&profile_key)?;
        let target_state_digest = state_digest(current.as_ref())?;
        let source = collect_source_snapshot(&secured.canonical_root, secured.classification)?;
        let rows = materialize_rows(&source, current.as_ref());
        let apply_gate = apply_gate_for_rows(&rows);
        let source_root_ref = source_root_ref(&secured.canonical_root);
        let plan_digest = compute_plan_digest(
            &review.import_review_id,
            &source_root_ref,
            secured.classification,
            &source.snapshot_digest,
            &target_profile_ref,
            policy_epoch,
            &target_state_digest,
            &rows,
            apply_gate,
        )?;
        let preview_ref = format!("import-execution-preview:{}", digest_suffix(&plan_digest));

        let generated_at = utc_timestamp_now();
        Ok(ExecutableImportPreview {
            record_kind: IMPORT_EXECUTION_PREVIEW_RECORD_KIND.to_owned(),
            schema_version: IMPORT_EXECUTION_SCHEMA_VERSION,
            preview_ref,
            import_review_ref: review.import_review_id.clone(),
            source_root_ref,
            source_classification: secured.classification,
            target_profile_ref,
            policy_epoch: policy_epoch.to_owned(),
            rows,
            apply_gate,
            source_snapshot_digest: source.snapshot_digest.clone(),
            target_state_digest: target_state_digest.clone(),
            plan_digest: plan_digest.clone(),
            generated_at: generated_at.clone(),
            live_authority: Some(LivePreviewAuthority {
                canonical_source_root: secured.canonical_root,
                profile_key,
                source_snapshot_digest: source.snapshot_digest,
                target_state_digest,
                plan_digest,
                generated_at,
            }),
        })
    }

    /// Applies exactly the admitted rows after revalidating source, target, and policy.
    pub fn apply(
        &self,
        preview: &ExecutableImportPreview,
        idempotency_token: &str,
        current_policy_epoch: &str,
    ) -> Result<ImportApplyOutcome, ImportExecutionError> {
        validate_idempotency_token(idempotency_token)?;
        validate_policy_epoch(current_policy_epoch)?;
        let authority = preview
            .live_authority
            .as_ref()
            .ok_or(ImportExecutionError::PreviewAuthorityMissing)?;
        validate_preview_shape(preview, authority)?;
        if preview.policy_epoch != current_policy_epoch {
            return Err(ImportExecutionError::PolicyEpochChanged);
        }
        if preview.apply_gate == ImportPreviewApplyGate::BlockedNoSupportedSettings {
            return Err(ImportExecutionError::ApplyNotAllowed);
        }

        let current = self.load_profile_by_key(&authority.profile_key)?;
        let token_digest = aureline_history::body_object_id(idempotency_token.as_bytes());
        if let Some(state) = current.as_ref() {
            if let Some(entry) = state
                .history
                .iter()
                .find(|entry| entry.idempotency_token_digest == token_digest)
            {
                if entry.action == ImportedProfileHistoryAction::Applied
                    && entry.plan_digest == preview.plan_digest
                    && entry.result_settings_digest == settings_digest(&state.settings)?
                {
                    return Ok(ImportApplyOutcome {
                        disposition: ImportApplyDisposition::AlreadyApplied,
                        target_profile_ref: state.profile_ref.clone(),
                        plan_digest: preview.plan_digest.clone(),
                        checkpoint_ref: Some(entry.checkpoint_ref.clone()),
                        result_settings_digest: entry.result_settings_digest.clone(),
                        revision: state.revision,
                        completed_at: utc_timestamp_now(),
                    });
                }
                return Err(ImportExecutionError::IdempotencyConflict);
            }
        }

        if state_digest(current.as_ref())? != authority.target_state_digest {
            return Err(ImportExecutionError::PreviewStale {
                reason_code: "target_state_changed",
            });
        }
        let source = collect_source_snapshot(
            &authority.canonical_source_root,
            preview.source_classification,
        )?;
        if source.snapshot_digest != authority.source_snapshot_digest {
            return Err(ImportExecutionError::PreviewStale {
                reason_code: "source_state_changed",
            });
        }
        let live_rows = materialize_rows(&source, current.as_ref());
        let live_apply_gate = apply_gate_for_rows(&live_rows);
        let live_plan_digest = compute_plan_digest(
            &preview.import_review_ref,
            &source_root_ref(&authority.canonical_source_root),
            preview.source_classification,
            &source.snapshot_digest,
            &preview.target_profile_ref,
            current_policy_epoch,
            &authority.target_state_digest,
            &live_rows,
            live_apply_gate,
        )?;
        if live_plan_digest != preview.plan_digest
            || live_rows != preview.rows
            || live_apply_gate != preview.apply_gate
        {
            return Err(ImportExecutionError::PreviewStale {
                reason_code: "parsed_plan_changed",
            });
        }

        if preview.apply_gate == ImportPreviewApplyGate::NoChanges {
            let (revision, result_settings_digest) = match current.as_ref() {
                Some(state) => (state.revision, settings_digest(&state.settings)?),
                None => (0, settings_digest(&BTreeMap::new())?),
            };
            return Ok(ImportApplyOutcome {
                disposition: ImportApplyDisposition::NoChanges,
                target_profile_ref: preview.target_profile_ref.clone(),
                plan_digest: preview.plan_digest.clone(),
                checkpoint_ref: None,
                result_settings_digest,
                revision,
                completed_at: utc_timestamp_now(),
            });
        }
        if preview.apply_gate != ImportPreviewApplyGate::AllowedCheckpointRequired {
            return Err(ImportExecutionError::ApplyNotAllowed);
        }

        let changed_rows: Vec<&ImportExecutionRow> = preview
            .rows
            .iter()
            .filter(|row| row.decision == ImportExecutionRowDecision::Admitted)
            .collect();
        if changed_rows.is_empty() {
            return Err(ImportExecutionError::ApplyNotAllowed);
        }
        let now = utc_timestamp_now();
        let mut next = current.clone().unwrap_or_else(|| ImportedProfileState {
            record_kind: IMPORTED_PROFILE_STATE_RECORD_KIND.to_owned(),
            schema_version: IMPORT_EXECUTION_SCHEMA_VERSION,
            profile_ref: preview.target_profile_ref.clone(),
            revision: 0,
            settings: BTreeMap::new(),
            history: Vec::new(),
            updated_at: now.clone(),
        });
        validate_durable_state(&next, &preview.target_profile_ref)?;
        if next.history.len() >= MAX_HISTORY_ROWS {
            return Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "history_capacity_reached",
            });
        }
        let mut changed_setting_ids = Vec::with_capacity(changed_rows.len());
        for row in changed_rows {
            let target_setting_id =
                row.target_setting_id
                    .as_ref()
                    .ok_or(ImportExecutionError::PreviewStale {
                        reason_code: "admitted_row_missing_target",
                    })?;
            let value = row
                .after_value
                .clone()
                .ok_or(ImportExecutionError::PreviewStale {
                    reason_code: "admitted_row_missing_value",
                })?;
            next.settings.insert(
                target_setting_id.clone(),
                ImportedProfileSettingRecord {
                    value,
                    source_classification: preview.source_classification,
                    source_setting_ref: row.source_setting_ref.clone(),
                    imported_at: now.clone(),
                },
            );
            changed_setting_ids.push(target_setting_id.clone());
        }
        changed_setting_ids.sort();
        changed_setting_ids.dedup();
        let result_settings_digest = settings_digest(&next.settings)?;
        let checkpoint_ref =
            checkpoint_ref(&authority.profile_key, &preview.plan_digest, &token_digest);
        let checkpoint = ImportExecutionCheckpoint {
            record_kind: IMPORT_EXECUTION_CHECKPOINT_RECORD_KIND.to_owned(),
            schema_version: IMPORT_EXECUTION_SCHEMA_VERSION,
            checkpoint_ref: checkpoint_ref.clone(),
            profile_key: authority.profile_key.clone(),
            target_profile_ref: preview.target_profile_ref.clone(),
            preview_ref: preview.preview_ref.clone(),
            import_review_ref: preview.import_review_ref.clone(),
            source_root_ref: preview.source_root_ref.clone(),
            policy_epoch: preview.policy_epoch.clone(),
            idempotency_token_digest: token_digest.clone(),
            plan_digest: preview.plan_digest.clone(),
            source_snapshot_digest: preview.source_snapshot_digest.clone(),
            prior_state_digest: authority.target_state_digest.clone(),
            expected_applied_settings_digest: result_settings_digest.clone(),
            prior_state: current.clone(),
            created_at: now.clone(),
        };
        self.write_checkpoint(&checkpoint)?;

        next.revision =
            next.revision
                .checked_add(1)
                .ok_or(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "revision_overflow",
                })?;
        next.updated_at = now.clone();
        next.history.push(ImportedProfileHistoryEntry {
            action: ImportedProfileHistoryAction::Applied,
            idempotency_token_digest: token_digest.clone(),
            preview_ref: preview.preview_ref.clone(),
            import_review_ref: preview.import_review_ref.clone(),
            source_root_ref: preview.source_root_ref.clone(),
            source_snapshot_digest: preview.source_snapshot_digest.clone(),
            policy_epoch: preview.policy_epoch.clone(),
            plan_digest: preview.plan_digest.clone(),
            checkpoint_ref: checkpoint_ref.clone(),
            result_settings_digest: result_settings_digest.clone(),
            changed_setting_ids,
            occurred_at: now.clone(),
        });
        if let Err(error) = self.write_profile_by_key(&authority.profile_key, &next) {
            if error == ImportExecutionError::ConcurrentMutation {
                if let Some(winner) = self.load_profile_by_key(&authority.profile_key)? {
                    if let Some(entry) = winner.history.iter().find(|entry| {
                        entry.idempotency_token_digest == token_digest
                            && entry.action == ImportedProfileHistoryAction::Applied
                            && entry.plan_digest == preview.plan_digest
                    }) {
                        if entry.result_settings_digest == settings_digest(&winner.settings)? {
                            return Ok(ImportApplyOutcome {
                                disposition: ImportApplyDisposition::AlreadyApplied,
                                target_profile_ref: winner.profile_ref.clone(),
                                plan_digest: preview.plan_digest.clone(),
                                checkpoint_ref: Some(entry.checkpoint_ref.clone()),
                                result_settings_digest: entry.result_settings_digest.clone(),
                                revision: winner.revision,
                                completed_at: utc_timestamp_now(),
                            });
                        }
                    }
                }
            }
            return Err(error);
        }

        Ok(ImportApplyOutcome {
            disposition: ImportApplyDisposition::Applied,
            target_profile_ref: preview.target_profile_ref.clone(),
            plan_digest: preview.plan_digest.clone(),
            checkpoint_ref: Some(checkpoint_ref),
            result_settings_digest,
            revision: next.revision,
            completed_at: now,
        })
    }

    /// Restores one exact checkpoint unless newer effective settings landed.
    pub fn rollback(
        &self,
        request: ImportRollbackRequest<'_>,
    ) -> Result<ImportRollbackOutcome, ImportExecutionError> {
        validate_idempotency_token(request.idempotency_token)?;
        let checkpoint = self.read_checkpoint(request.checkpoint_ref)?;
        let mut current = self
            .load_profile_by_key(&checkpoint.profile_key)?
            .ok_or(ImportExecutionError::CheckpointUnavailable)?;
        validate_durable_state(&current, &checkpoint.target_profile_ref)?;
        let token_digest = aureline_history::body_object_id(request.idempotency_token.as_bytes());

        if let Some(entry) = current
            .history
            .iter()
            .find(|entry| entry.idempotency_token_digest == token_digest)
        {
            if entry.action == ImportedProfileHistoryAction::RolledBack
                && entry.checkpoint_ref == checkpoint.checkpoint_ref
                && entry.result_settings_digest == settings_digest(&current.settings)?
            {
                return Ok(ImportRollbackOutcome {
                    restored_now: false,
                    checkpoint_ref: checkpoint.checkpoint_ref.clone(),
                    target_profile_ref: current.profile_ref.clone(),
                    result_settings_digest: entry.result_settings_digest.clone(),
                    revision: current.revision,
                    completed_at: utc_timestamp_now(),
                });
            }
            return Err(ImportExecutionError::IdempotencyConflict);
        }

        let latest = current
            .history
            .last()
            .ok_or(ImportExecutionError::RollbackConflict)?;
        if latest.action != ImportedProfileHistoryAction::Applied
            || latest.checkpoint_ref != checkpoint.checkpoint_ref
            || settings_digest(&current.settings)? != checkpoint.expected_applied_settings_digest
        {
            return Err(ImportExecutionError::RollbackConflict);
        }
        if current.history.len() >= MAX_HISTORY_ROWS {
            return Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "history_capacity_reached",
            });
        }

        let before_settings = current.settings.clone();
        current.settings = checkpoint
            .prior_state
            .as_ref()
            .map(|state| state.settings.clone())
            .unwrap_or_default();
        let mut changed_setting_ids: Vec<String> = before_settings
            .keys()
            .chain(current.settings.keys())
            .cloned()
            .collect();
        changed_setting_ids.sort();
        changed_setting_ids.dedup();
        changed_setting_ids.retain(|setting_id| {
            before_settings.get(setting_id) != current.settings.get(setting_id)
        });
        let result_settings_digest = settings_digest(&current.settings)?;
        let now = utc_timestamp_now();
        current.revision = current.revision.checked_add(1).ok_or(
            ImportExecutionError::DurableStateUnavailable {
                reason_code: "revision_overflow",
            },
        )?;
        current.updated_at = now.clone();
        current.history.push(ImportedProfileHistoryEntry {
            action: ImportedProfileHistoryAction::RolledBack,
            idempotency_token_digest: token_digest.clone(),
            preview_ref: checkpoint.preview_ref.clone(),
            import_review_ref: checkpoint.import_review_ref.clone(),
            source_root_ref: checkpoint.source_root_ref.clone(),
            source_snapshot_digest: checkpoint.source_snapshot_digest.clone(),
            policy_epoch: checkpoint.policy_epoch.clone(),
            plan_digest: checkpoint.plan_digest.clone(),
            checkpoint_ref: checkpoint.checkpoint_ref.clone(),
            result_settings_digest: result_settings_digest.clone(),
            changed_setting_ids,
            occurred_at: now.clone(),
        });
        if let Err(error) = self.write_profile_by_key(&checkpoint.profile_key, &current) {
            if error == ImportExecutionError::ConcurrentMutation {
                if let Some(winner) = self.load_profile_by_key(&checkpoint.profile_key)? {
                    if let Some(entry) = winner.history.iter().find(|entry| {
                        entry.idempotency_token_digest == token_digest
                            && entry.action == ImportedProfileHistoryAction::RolledBack
                            && entry.checkpoint_ref == checkpoint.checkpoint_ref
                    }) {
                        if entry.result_settings_digest == settings_digest(&winner.settings)? {
                            return Ok(ImportRollbackOutcome {
                                restored_now: false,
                                checkpoint_ref: checkpoint.checkpoint_ref,
                                target_profile_ref: winner.profile_ref.clone(),
                                result_settings_digest: entry.result_settings_digest.clone(),
                                revision: winner.revision,
                                completed_at: utc_timestamp_now(),
                            });
                        }
                    }
                }
            }
            return Err(error);
        }

        Ok(ImportRollbackOutcome {
            restored_now: true,
            checkpoint_ref: checkpoint.checkpoint_ref,
            target_profile_ref: current.profile_ref,
            result_settings_digest,
            revision: current.revision,
            completed_at: now,
        })
    }

    /// Loads the dedicated imported profile for a destination target.
    pub fn load_profile_for_target(
        &self,
        destination_workspace_target: &str,
    ) -> Result<Option<ImportedProfileState>, ImportExecutionError> {
        validate_request_label(destination_workspace_target, "destination_target_invalid")?;
        self.load_profile_by_key(&profile_key(destination_workspace_target))
    }

    /// Loads and validates the resolver overlay for a destination target.
    ///
    /// This does not mutate a resolver or activate a profile. Shell bootstrap
    /// owns that explicit wiring step and must pass the same canonical registry
    /// used by its live [`aureline_settings::resolver::EffectiveSettingsResolver`].
    pub fn load_resolver_overlay_for_target(
        &self,
        destination_workspace_target: &str,
        registry: &SchemaRegistry,
    ) -> Result<Option<ScopeOverlay>, ImportExecutionError> {
        self.load_profile_for_target(destination_workspace_target)?
            .map(|state| state.to_resolver_overlay(registry))
            .transpose()
    }

    fn load_profile_by_key(
        &self,
        profile_key: &str,
    ) -> Result<Option<ImportedProfileState>, ImportExecutionError> {
        validate_profile_key(profile_key)?;
        let directory = self.profile_revision_directory(profile_key);
        let metadata = match std::fs::symlink_metadata(&directory) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_) => {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "profile_revision_directory_metadata_failed",
                })
            }
        };
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            return Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "profile_revision_directory_type_unsafe",
            });
        }
        ensure_existing_child_chain_has_no_symlink(&self.state_root, &directory.join("entry"))?;

        let entries = std::fs::read_dir(&directory).map_err(|_| {
            ImportExecutionError::DurableStateUnavailable {
                reason_code: "profile_revision_directory_read_failed",
            }
        })?;
        let mut committed_count = 0usize;
        let mut latest: Option<(u64, PathBuf)> = None;
        for entry in entries {
            let entry = entry.map_err(|_| ImportExecutionError::DurableStateUnavailable {
                reason_code: "profile_revision_entry_read_failed",
            })?;
            let entry_path = entry.path();
            let entry_metadata = std::fs::symlink_metadata(&entry_path).map_err(|_| {
                ImportExecutionError::DurableStateUnavailable {
                    reason_code: "profile_revision_entry_metadata_failed",
                }
            })?;
            if entry_metadata.file_type().is_symlink() || !entry_metadata.is_file() {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "profile_revision_entry_type_unsafe",
                });
            }
            let Some(filename) = entry.file_name().to_str().map(str::to_owned) else {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "profile_revision_filename_invalid",
                });
            };
            if filename.starts_with('.') && filename.contains(".tmp.") {
                continue;
            }
            let Some(revision_text) = filename.strip_suffix(".json") else {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "profile_revision_filename_invalid",
                });
            };
            if revision_text.len() != 20 || !revision_text.bytes().all(|byte| byte.is_ascii_digit())
            {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "profile_revision_filename_invalid",
                });
            }
            let revision = revision_text.parse::<u64>().map_err(|_| {
                ImportExecutionError::DurableStateUnavailable {
                    reason_code: "profile_revision_filename_invalid",
                }
            })?;
            if revision == 0 {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "profile_revision_filename_invalid",
                });
            }
            committed_count += 1;
            if committed_count > MAX_HISTORY_ROWS {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "profile_revision_capacity_exceeded",
                });
            }
            if latest
                .as_ref()
                .map_or(true, |(latest_revision, _)| revision > *latest_revision)
            {
                latest = Some((revision, entry_path));
            }
        }
        let Some((expected_revision, path)) = latest else {
            return Ok(None);
        };
        let bytes = read_optional_durable_file(&self.state_root, &path)?.ok_or(
            ImportExecutionError::DurableStateUnavailable {
                reason_code: "profile_revision_disappeared",
            },
        )?;
        let state: ImportedProfileState = serde_json::from_slice(&bytes).map_err(|_| {
            ImportExecutionError::DurableStateUnavailable {
                reason_code: "profile_state_malformed",
            }
        })?;
        validate_durable_state(&state, &format!("imported-profile:{profile_key}"))?;
        if state.revision != expected_revision || committed_count != state.revision as usize {
            return Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "profile_revision_lineage_invalid",
            });
        }
        Ok(Some(state))
    }

    fn write_profile_by_key(
        &self,
        profile_key: &str,
        state: &ImportedProfileState,
    ) -> Result<(), ImportExecutionError> {
        validate_profile_key(profile_key)?;
        validate_durable_state(state, &format!("imported-profile:{profile_key}"))?;
        if state.revision == 0 {
            return Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "profile_revision_zero_not_committable",
            });
        }
        let path = self.profile_revision_path(profile_key, state.revision);
        match write_new_json(&self.state_root, &path, state) {
            Ok(()) => Ok(()),
            Err(error) => match std::fs::symlink_metadata(&path) {
                Ok(metadata) if metadata.is_file() && !metadata.file_type().is_symlink() => {
                    Err(ImportExecutionError::ConcurrentMutation)
                }
                _ => Err(error),
            },
        }
    }

    fn write_checkpoint(
        &self,
        checkpoint: &ImportExecutionCheckpoint,
    ) -> Result<(), ImportExecutionError> {
        validate_checkpoint(checkpoint)?;
        let path = self.checkpoint_path(&checkpoint.profile_key, &checkpoint.checkpoint_ref)?;
        if let Some(bytes) = read_optional_durable_file(&self.state_root, &path)? {
            let existing: ImportExecutionCheckpoint =
                serde_json::from_slice(&bytes).map_err(|_| {
                    ImportExecutionError::DurableStateUnavailable {
                        reason_code: "checkpoint_malformed",
                    }
                })?;
            validate_checkpoint(&existing)?;
            if checkpoints_equivalent(&existing, checkpoint) {
                return Ok(());
            }
            return Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "checkpoint_identity_collision",
            });
        }
        match write_new_json(&self.state_root, &path, checkpoint) {
            Ok(()) => Ok(()),
            Err(error) => {
                let Some(bytes) = read_optional_durable_file(&self.state_root, &path)? else {
                    return Err(error);
                };
                let existing: ImportExecutionCheckpoint =
                    serde_json::from_slice(&bytes).map_err(|_| {
                        ImportExecutionError::DurableStateUnavailable {
                            reason_code: "checkpoint_malformed",
                        }
                    })?;
                validate_checkpoint(&existing)?;
                if checkpoints_equivalent(&existing, checkpoint) {
                    Ok(())
                } else {
                    Err(error)
                }
            }
        }
    }

    fn read_checkpoint(
        &self,
        checkpoint_ref: &str,
    ) -> Result<ImportExecutionCheckpoint, ImportExecutionError> {
        let (profile_key, _) = parse_checkpoint_ref(checkpoint_ref)?;
        let path = self.checkpoint_path(profile_key, checkpoint_ref)?;
        let bytes = read_optional_durable_file(&self.state_root, &path)?
            .ok_or(ImportExecutionError::CheckpointUnavailable)?;
        let checkpoint: ImportExecutionCheckpoint = serde_json::from_slice(&bytes)
            .map_err(|_| ImportExecutionError::CheckpointUnavailable)?;
        validate_checkpoint(&checkpoint)?;
        if checkpoint.checkpoint_ref != checkpoint_ref {
            return Err(ImportExecutionError::CheckpointUnavailable);
        }
        Ok(checkpoint)
    }

    fn profile_revision_directory(&self, profile_key: &str) -> PathBuf {
        self.state_root
            .join("imported_profiles")
            .join(profile_key)
            .join("revisions")
    }

    fn profile_revision_path(&self, profile_key: &str, revision: u64) -> PathBuf {
        self.profile_revision_directory(profile_key)
            .join(format!("{revision:020}.json"))
    }

    fn checkpoint_path(
        &self,
        profile_key: &str,
        checkpoint_ref: &str,
    ) -> Result<PathBuf, ImportExecutionError> {
        validate_profile_key(profile_key)?;
        let (_, checkpoint_key) = parse_checkpoint_ref(checkpoint_ref)?;
        Ok(self
            .state_root
            .join("imported_profiles")
            .join(profile_key)
            .join("checkpoints")
            .join(format!("{checkpoint_key}.json")))
    }
}

#[derive(Debug)]
struct SecuredSourceRoot {
    canonical_root: PathBuf,
    classification: CompetitorConfigClassification,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SourceSnapshot {
    classification: CompetitorConfigClassification,
    candidates: Vec<SourceSettingCandidate>,
    excluded_rows: Vec<SourceExcludedRow>,
    file_digests: Vec<SourceFileDigest>,
    snapshot_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SourceSettingCandidate {
    source_setting_ref: String,
    target_setting_id: String,
    value: ImportSettingValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SourceExcludedRow {
    source_setting_ref: String,
    decision: ImportExecutionRowDecision,
    reason_code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SourceFileDigest {
    source_item_ref: String,
    body_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ImportExecutionCheckpoint {
    record_kind: String,
    schema_version: u32,
    checkpoint_ref: String,
    profile_key: String,
    target_profile_ref: String,
    preview_ref: String,
    import_review_ref: String,
    source_root_ref: String,
    policy_epoch: String,
    idempotency_token_digest: String,
    plan_digest: String,
    source_snapshot_digest: String,
    prior_state_digest: String,
    expected_applied_settings_digest: String,
    prior_state: Option<ImportedProfileState>,
    created_at: String,
}

fn secure_source_root(source_root: &Path) -> Result<SecuredSourceRoot, ImportExecutionError> {
    let metadata = std::fs::symlink_metadata(source_root).map_err(|_| {
        ImportExecutionError::SourceUnavailable {
            source_item_ref: "source_root".to_owned(),
        }
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(ImportExecutionError::UnsafeSourceLayout {
            source_item_ref: "source_root".to_owned(),
        });
    }
    let canonical_root = std::fs::canonicalize(source_root).map_err(|_| {
        ImportExecutionError::SourceUnavailable {
            source_item_ref: "source_root".to_owned(),
        }
    })?;
    let has_vscode = secure_marker_directory(&canonical_root, ".vscode")?;
    let has_idea = secure_marker_directory(&canonical_root, ".idea")?;
    let classification = match (has_vscode, has_idea) {
        (true, false) => CompetitorConfigClassification::VSCodeWorkspaceRoot,
        (false, true) => CompetitorConfigClassification::JetBrainsIdeaRoot,
        (false, false) => return Err(ImportExecutionError::UnsupportedSource),
        (true, true) => {
            return Err(ImportExecutionError::MalformedInput {
                source_item_ref: "source_root".to_owned(),
                reason_code: "ambiguous_source_markers",
            })
        }
    };
    Ok(SecuredSourceRoot {
        canonical_root,
        classification,
    })
}

fn secure_marker_directory(
    canonical_root: &Path,
    marker: &str,
) -> Result<bool, ImportExecutionError> {
    let path = canonical_root.join(marker);
    let metadata = match std::fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(_) => {
            return Err(ImportExecutionError::SourceUnavailable {
                source_item_ref: marker.to_owned(),
            })
        }
    };
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(ImportExecutionError::UnsafeSourceLayout {
            source_item_ref: marker.to_owned(),
        });
    }
    let canonical =
        std::fs::canonicalize(&path).map_err(|_| ImportExecutionError::SourceUnavailable {
            source_item_ref: marker.to_owned(),
        })?;
    if !canonical.starts_with(canonical_root) {
        return Err(ImportExecutionError::UnsafeSourceLayout {
            source_item_ref: marker.to_owned(),
        });
    }
    Ok(true)
}

fn collect_source_snapshot(
    canonical_root: &Path,
    classification: CompetitorConfigClassification,
) -> Result<SourceSnapshot, ImportExecutionError> {
    let mut candidates = Vec::new();
    let mut excluded_rows = Vec::new();
    let mut file_digests = Vec::new();
    match classification {
        CompetitorConfigClassification::VSCodeWorkspaceRoot => collect_vscode_source(
            canonical_root,
            &mut candidates,
            &mut excluded_rows,
            &mut file_digests,
        )?,
        CompetitorConfigClassification::JetBrainsIdeaRoot => collect_jetbrains_source(
            canonical_root,
            &mut candidates,
            &mut excluded_rows,
            &mut file_digests,
        )?,
        CompetitorConfigClassification::UnknownConfigRoot => {
            return Err(ImportExecutionError::UnsupportedSource)
        }
    }
    normalize_candidates(&mut candidates)?;
    validate_candidates_against_resolver(&candidates)?;
    candidates.sort_by(|left, right| {
        left.target_setting_id
            .cmp(&right.target_setting_id)
            .then_with(|| left.source_setting_ref.cmp(&right.source_setting_ref))
    });
    excluded_rows.sort_by(|left, right| {
        left.source_setting_ref
            .cmp(&right.source_setting_ref)
            .then_with(|| left.reason_code.cmp(&right.reason_code))
    });
    file_digests.sort_by(|left, right| left.source_item_ref.cmp(&right.source_item_ref));
    let snapshot_digest =
        digest_serializable(&(classification, &candidates, &excluded_rows, &file_digests))?;
    Ok(SourceSnapshot {
        classification,
        candidates,
        excluded_rows,
        file_digests,
        snapshot_digest,
    })
}

fn collect_vscode_source(
    canonical_root: &Path,
    candidates: &mut Vec<SourceSettingCandidate>,
    excluded_rows: &mut Vec<SourceExcludedRow>,
    file_digests: &mut Vec<SourceFileDigest>,
) -> Result<(), ImportExecutionError> {
    const SETTINGS: &str = ".vscode/settings.json";
    if let Some(bytes) = secure_read_source_file(canonical_root, SETTINGS)? {
        file_digests.push(SourceFileDigest {
            source_item_ref: SETTINGS.to_owned(),
            body_digest: aureline_history::body_object_id(&bytes),
        });
        let object = parse_strict_json_object(&bytes, SETTINGS)?;
        for (key, value) in object {
            match map_vscode_setting(&key, &value, SETTINGS)? {
                Some(candidate) => candidates.push(candidate),
                None => excluded_rows.push(excluded_source_setting(SETTINGS, &key)),
            }
        }
    }
    for (relative, decision, reason_code) in [
        (
            ".vscode/keybindings.json",
            ImportExecutionRowDecision::BlockedAuthority,
            "command_binding_import_excluded",
        ),
        (
            ".vscode/tasks.json",
            ImportExecutionRowDecision::BlockedAuthority,
            "task_execution_import_excluded",
        ),
        (
            ".vscode/launch.json",
            ImportExecutionRowDecision::BlockedAuthority,
            "launch_execution_import_excluded",
        ),
        (
            ".vscode/extensions.json",
            ImportExecutionRowDecision::BlockedAuthority,
            "extension_authority_import_excluded",
        ),
        (
            ".vscode/snippets",
            ImportExecutionRowDecision::Unsupported,
            "snippet_import_not_in_conservative_lane",
        ),
    ] {
        if secure_source_entry_exists(canonical_root, relative)? {
            excluded_rows.push(SourceExcludedRow {
                source_setting_ref: relative.to_owned(),
                decision,
                reason_code: reason_code.to_owned(),
            });
        }
    }
    Ok(())
}

fn collect_jetbrains_source(
    canonical_root: &Path,
    candidates: &mut Vec<SourceSettingCandidate>,
    excluded_rows: &mut Vec<SourceExcludedRow>,
    file_digests: &mut Vec<SourceFileDigest>,
) -> Result<(), ImportExecutionError> {
    for relative in [
        ".idea/editor.xml",
        ".idea/codeStyles/Project.xml",
        ".idea/codeStyles/codeStyleConfig.xml",
    ] {
        let Some(bytes) = secure_read_source_file(canonical_root, relative)? else {
            continue;
        };
        file_digests.push(SourceFileDigest {
            source_item_ref: relative.to_owned(),
            body_digest: aureline_history::body_object_id(&bytes),
        });
        for (name, value) in parse_jetbrains_options(&bytes, relative)? {
            match map_jetbrains_setting(&name, &value, relative)? {
                Some(candidate) => candidates.push(candidate),
                None => excluded_rows.push(excluded_source_setting(relative, &name)),
            }
        }
    }
    for (relative, decision, reason_code) in [
        (
            ".idea/workspace.xml",
            ImportExecutionRowDecision::BlockedAuthority,
            "workspace_execution_and_private_state_excluded",
        ),
        (
            ".idea/runConfigurations",
            ImportExecutionRowDecision::BlockedAuthority,
            "run_configuration_import_excluded",
        ),
        (
            ".idea/modules.xml",
            ImportExecutionRowDecision::Unsupported,
            "machine_project_path_import_excluded",
        ),
        (
            ".idea/misc.xml",
            ImportExecutionRowDecision::Unsupported,
            "sdk_and_project_metadata_import_excluded",
        ),
        (
            ".idea/inspectionProfiles",
            ImportExecutionRowDecision::Unsupported,
            "inspection_profile_import_not_in_conservative_lane",
        ),
    ] {
        if secure_source_entry_exists(canonical_root, relative)? {
            excluded_rows.push(SourceExcludedRow {
                source_setting_ref: relative.to_owned(),
                decision,
                reason_code: reason_code.to_owned(),
            });
        }
    }
    Ok(())
}

fn map_vscode_setting(
    key: &str,
    value: &serde_json::Value,
    source_file: &str,
) -> Result<Option<SourceSettingCandidate>, ImportExecutionError> {
    let source_setting_ref = format!("{source_file}#{key}");
    let malformed = |reason_code: &'static str| ImportExecutionError::MalformedInput {
        source_item_ref: safe_source_key_ref(source_file, key),
        reason_code,
    };
    let mapped = match key {
        "editor.tabSize" => SourceSettingCandidate {
            source_setting_ref,
            target_setting_id: "editor.tab_size".to_owned(),
            value: ImportSettingValue::Integer(
                json_bounded_integer(value, 1, 16)
                    .ok_or_else(|| malformed("editor_tab_size_invalid"))?,
            ),
        },
        "editor.formatOnSave" => SourceSettingCandidate {
            source_setting_ref,
            target_setting_id: "editor.format_on_save".to_owned(),
            value: ImportSettingValue::Boolean(
                value
                    .as_bool()
                    .ok_or_else(|| malformed("editor_format_on_save_invalid"))?,
            ),
        },
        "workbench.colorTheme" => {
            let theme_name =
                json_bounded_text(value, 128).ok_or_else(|| malformed("theme_name_invalid"))?;
            let Some(theme_mode) = vscode_theme_mode(&theme_name) else {
                return Ok(None);
            };
            SourceSettingCandidate {
                source_setting_ref,
                target_setting_id: "ui.theme".to_owned(),
                value: ImportSettingValue::Text(theme_mode.to_owned()),
            }
        }
        "workbench.reduceMotion" => {
            let source_mode =
                json_bounded_text(value, 16).ok_or_else(|| malformed("reduce_motion_invalid"))?;
            let Some(motion_mode) = vscode_motion_mode(&source_mode) else {
                return Ok(None);
            };
            SourceSettingCandidate {
                source_setting_ref,
                target_setting_id: "ui.motion".to_owned(),
                value: ImportSettingValue::Text(motion_mode.to_owned()),
            }
        }
        _ => return Ok(None),
    };
    Ok(Some(mapped))
}

fn map_jetbrains_setting(
    name: &str,
    value: &str,
    source_file: &str,
) -> Result<Option<SourceSettingCandidate>, ImportExecutionError> {
    let source_setting_ref = format!("{source_file}#option:{name}");
    let malformed = |reason_code: &'static str| ImportExecutionError::MalformedInput {
        source_item_ref: safe_source_key_ref(source_file, name),
        reason_code,
    };
    let integer = |minimum, maximum, reason_code| {
        parse_bounded_integer(value, minimum, maximum).ok_or_else(|| malformed(reason_code))
    };
    let mapped = match name {
        "TAB_SIZE" => SourceSettingCandidate {
            source_setting_ref,
            target_setting_id: "editor.tab_size".to_owned(),
            value: ImportSettingValue::Integer(integer(1, 16, "tab_size_invalid")?),
        },
        _ => return Ok(None),
    };
    Ok(Some(mapped))
}

fn normalize_candidates(
    candidates: &mut Vec<SourceSettingCandidate>,
) -> Result<(), ImportExecutionError> {
    candidates.sort_by(|left, right| {
        left.target_setting_id
            .cmp(&right.target_setting_id)
            .then_with(|| left.source_setting_ref.cmp(&right.source_setting_ref))
    });
    let mut normalized: Vec<SourceSettingCandidate> = Vec::with_capacity(candidates.len());
    for candidate in candidates.drain(..) {
        if let Some(previous) = normalized
            .iter()
            .find(|previous| previous.target_setting_id == candidate.target_setting_id)
        {
            if previous.value == candidate.value {
                continue;
            }
            return Err(ImportExecutionError::MalformedInput {
                source_item_ref: candidate.source_setting_ref,
                reason_code: "ambiguous_target_mapping",
            });
        }
        normalized.push(candidate);
    }
    *candidates = normalized;
    Ok(())
}

fn validate_candidates_against_resolver(
    candidates: &[SourceSettingCandidate],
) -> Result<(), ImportExecutionError> {
    let registry = SchemaRegistry::with_seed_catalog();
    for candidate in candidates {
        let definition = registry
            .definition(&candidate.target_setting_id)
            .ok_or_else(|| ImportExecutionError::ResolverProjectionUnavailable {
                setting_id: candidate.target_setting_id.clone(),
                reason_code: "import_mapping_target_not_registered",
            })?;
        if !definition.allows_scope(SettingScope::ImportedProfileDefault) {
            return Err(ImportExecutionError::ResolverProjectionUnavailable {
                setting_id: candidate.target_setting_id.clone(),
                reason_code: "import_mapping_scope_not_allowed",
            });
        }
        definition
            .validate_value(&resolver_value(&candidate.value))
            .map_err(|_| ImportExecutionError::ResolverProjectionUnavailable {
                setting_id: candidate.target_setting_id.clone(),
                reason_code: "import_mapping_value_invalid",
            })?;
    }
    Ok(())
}

fn materialize_rows(
    source: &SourceSnapshot,
    current: Option<&ImportedProfileState>,
) -> Vec<ImportExecutionRow> {
    let mut rows = Vec::with_capacity(source.candidates.len() + source.excluded_rows.len());
    for candidate in &source.candidates {
        let existing = current.and_then(|state| state.settings.get(&candidate.target_setting_id));
        let (decision, reason_code) = match existing {
            None => (
                ImportExecutionRowDecision::Admitted,
                "supported_profile_setting",
            ),
            Some(existing) if existing.value == candidate.value => (
                ImportExecutionRowDecision::NoChange,
                "target_already_matches",
            ),
            Some(existing)
                if existing.source_classification == source.classification
                    && existing.source_setting_ref == candidate.source_setting_ref =>
            {
                (
                    ImportExecutionRowDecision::Admitted,
                    "same_source_reviewed_update",
                )
            }
            Some(_) => (
                ImportExecutionRowDecision::ManualReview,
                "different_source_owns_target",
            ),
        };
        rows.push(ImportExecutionRow {
            row_ref: row_ref(
                &candidate.source_setting_ref,
                Some(&candidate.target_setting_id),
            ),
            source_setting_ref: candidate.source_setting_ref.clone(),
            target_setting_id: Some(candidate.target_setting_id.clone()),
            before_value: existing.map(|record| record.value.clone()),
            after_value: Some(candidate.value.clone()),
            decision,
            reason_code: reason_code.to_owned(),
        });
    }
    for excluded in &source.excluded_rows {
        rows.push(ImportExecutionRow {
            row_ref: row_ref(&excluded.source_setting_ref, None),
            source_setting_ref: excluded.source_setting_ref.clone(),
            target_setting_id: None,
            before_value: None,
            after_value: None,
            decision: excluded.decision,
            reason_code: excluded.reason_code.clone(),
        });
    }
    rows.sort_by(|left, right| left.row_ref.cmp(&right.row_ref));
    rows
}

fn excluded_source_setting(source_file: &str, source_key: &str) -> SourceExcludedRow {
    let authority_bearing = is_authority_or_sensitive_key(source_key);
    SourceExcludedRow {
        source_setting_ref: safe_source_key_ref(source_file, source_key),
        decision: if authority_bearing {
            ImportExecutionRowDecision::BlockedAuthority
        } else {
            ImportExecutionRowDecision::Unsupported
        },
        reason_code: if authority_bearing {
            "authority_or_sensitive_setting_excluded".to_owned()
        } else {
            "unsupported_setting_visible".to_owned()
        },
    }
}

fn safe_source_key_ref(source_file: &str, source_key: &str) -> String {
    if is_authority_or_sensitive_key(source_key)
        || source_key.len() > 160
        || source_key.chars().any(char::is_control)
    {
        format!(
            "{source_file}#redacted-key:{}",
            digest_suffix(&aureline_history::body_object_id(source_key.as_bytes()))
        )
    } else {
        format!("{source_file}#{source_key}")
    }
}

fn is_authority_or_sensitive_key(key: &str) -> bool {
    let normalized = key.to_ascii_lowercase();
    [
        "password",
        "passwd",
        "secret",
        "token",
        "credential",
        "apikey",
        "api_key",
        "auth",
        "oauth",
        "terminal",
        "shell",
        "command",
        "executable",
        "task",
        "launch",
        "debug",
        "extension",
        "plugin",
        "workspace.trust",
        "security",
        "proxy",
        "network",
        "http.",
        "https.",
        "egress",
        "entitlement",
        "copilot",
        "chat.",
        "ai.",
        "autosave",
        "auto_save",
        "destructive",
    ]
    .iter()
    .any(|needle| normalized.contains(needle))
}

fn row_ref(source_setting_ref: &str, target_setting_id: Option<&str>) -> String {
    let identity = format!(
        "{source_setting_ref}\n{}",
        target_setting_id.unwrap_or("excluded")
    );
    format!(
        "import-execution-row:{}",
        digest_suffix(&aureline_history::body_object_id(identity.as_bytes()))
    )
}

fn secure_source_entry_exists(
    canonical_root: &Path,
    relative: &str,
) -> Result<bool, ImportExecutionError> {
    let Some(path) = secure_source_entry_path(canonical_root, relative)? else {
        return Ok(false);
    };
    let metadata =
        std::fs::symlink_metadata(&path).map_err(|_| ImportExecutionError::SourceUnavailable {
            source_item_ref: relative.to_owned(),
        })?;
    if metadata.file_type().is_symlink() {
        return Err(ImportExecutionError::UnsafeSourceLayout {
            source_item_ref: relative.to_owned(),
        });
    }
    let canonical =
        std::fs::canonicalize(&path).map_err(|_| ImportExecutionError::SourceUnavailable {
            source_item_ref: relative.to_owned(),
        })?;
    if !canonical.starts_with(canonical_root) {
        return Err(ImportExecutionError::UnsafeSourceLayout {
            source_item_ref: relative.to_owned(),
        });
    }
    Ok(true)
}

fn secure_read_source_file(
    canonical_root: &Path,
    relative: &str,
) -> Result<Option<Vec<u8>>, ImportExecutionError> {
    let Some(path) = secure_source_entry_path(canonical_root, relative)? else {
        return Ok(None);
    };
    let before =
        std::fs::symlink_metadata(&path).map_err(|_| ImportExecutionError::SourceUnavailable {
            source_item_ref: relative.to_owned(),
        })?;
    if before.file_type().is_symlink() || !before.is_file() {
        return Err(ImportExecutionError::UnsafeSourceLayout {
            source_item_ref: relative.to_owned(),
        });
    }
    if before.len() > MAX_SOURCE_FILE_BYTES {
        return Err(ImportExecutionError::InputTooLarge {
            source_item_ref: relative.to_owned(),
        });
    }
    let canonical =
        std::fs::canonicalize(&path).map_err(|_| ImportExecutionError::SourceUnavailable {
            source_item_ref: relative.to_owned(),
        })?;
    if !canonical.starts_with(canonical_root) {
        return Err(ImportExecutionError::UnsafeSourceLayout {
            source_item_ref: relative.to_owned(),
        });
    }
    let mut file = File::open(&canonical).map_err(|_| ImportExecutionError::SourceUnavailable {
        source_item_ref: relative.to_owned(),
    })?;
    let opened = file
        .metadata()
        .map_err(|_| ImportExecutionError::SourceUnavailable {
            source_item_ref: relative.to_owned(),
        })?;
    if !opened.is_file() || opened.len() > MAX_SOURCE_FILE_BYTES {
        return Err(ImportExecutionError::InputTooLarge {
            source_item_ref: relative.to_owned(),
        });
    }
    let mut bytes = Vec::with_capacity(opened.len() as usize);
    file.by_ref()
        .take(MAX_SOURCE_FILE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| ImportExecutionError::SourceUnavailable {
            source_item_ref: relative.to_owned(),
        })?;
    if bytes.len() as u64 > MAX_SOURCE_FILE_BYTES {
        return Err(ImportExecutionError::InputTooLarge {
            source_item_ref: relative.to_owned(),
        });
    }
    let after =
        std::fs::symlink_metadata(&path).map_err(|_| ImportExecutionError::PreviewStale {
            reason_code: "source_entry_changed_during_read",
        })?;
    let after_canonical =
        std::fs::canonicalize(&path).map_err(|_| ImportExecutionError::PreviewStale {
            reason_code: "source_entry_changed_during_read",
        })?;
    if after.file_type().is_symlink()
        || !after.is_file()
        || after_canonical != canonical
        || after.len() != opened.len()
    {
        return Err(ImportExecutionError::PreviewStale {
            reason_code: "source_entry_changed_during_read",
        });
    }
    Ok(Some(bytes))
}

fn secure_source_entry_path(
    canonical_root: &Path,
    relative: &str,
) -> Result<Option<PathBuf>, ImportExecutionError> {
    let relative_path = Path::new(relative);
    if relative_path.is_absolute()
        || relative_path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(ImportExecutionError::UnsafeSourceLayout {
            source_item_ref: "invalid_relative_source_ref".to_owned(),
        });
    }
    let mut current = canonical_root.to_path_buf();
    let components: Vec<_> = relative_path.components().collect();
    for (index, component) in components.iter().enumerate() {
        let Component::Normal(name) = component else {
            return Err(ImportExecutionError::UnsafeSourceLayout {
                source_item_ref: relative.to_owned(),
            });
        };
        current.push(name);
        let metadata = match std::fs::symlink_metadata(&current) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_) => {
                return Err(ImportExecutionError::SourceUnavailable {
                    source_item_ref: relative.to_owned(),
                })
            }
        };
        if metadata.file_type().is_symlink() {
            return Err(ImportExecutionError::UnsafeSourceLayout {
                source_item_ref: relative.to_owned(),
            });
        }
        if index + 1 < components.len() && !metadata.is_dir() {
            return Err(ImportExecutionError::UnsafeSourceLayout {
                source_item_ref: relative.to_owned(),
            });
        }
    }
    Ok(Some(current))
}

#[derive(Debug)]
struct StrictJsonValue(serde_json::Value);

impl<'de> Deserialize<'de> for StrictJsonValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StrictJsonVisitor)
    }
}

struct StrictJsonVisitor;

impl<'de> Visitor<'de> for StrictJsonVisitor {
    type Value = StrictJsonValue;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("valid JSON without duplicate object keys")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(serde_json::Value::Bool(value)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(serde_json::Value::Number(value.into())))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(serde_json::Value::Number(value.into())))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        serde_json::Number::from_f64(value)
            .map(serde_json::Value::Number)
            .map(StrictJsonValue)
            .ok_or_else(|| E::custom("non-finite JSON number"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_string(value.to_owned())
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(serde_json::Value::String(value)))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(serde_json::Value::Null))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(serde_json::Value::Null))
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element::<StrictJsonValue>()? {
            values.push(value.0);
        }
        Ok(StrictJsonValue(serde_json::Value::Array(values)))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = serde_json::Map::new();
        while let Some((key, value)) = map.next_entry::<String, StrictJsonValue>()? {
            if values.insert(key, value.0).is_some() {
                return Err(de::Error::custom("duplicate JSON object key"));
            }
        }
        Ok(StrictJsonValue(serde_json::Value::Object(values)))
    }
}

fn parse_strict_json_object(
    bytes: &[u8],
    source_item_ref: &str,
) -> Result<BTreeMap<String, serde_json::Value>, ImportExecutionError> {
    let parsed: StrictJsonValue =
        serde_json::from_slice(bytes).map_err(|_| ImportExecutionError::MalformedInput {
            source_item_ref: source_item_ref.to_owned(),
            reason_code: "strict_json_required",
        })?;
    let serde_json::Value::Object(values) = parsed.0 else {
        return Err(ImportExecutionError::MalformedInput {
            source_item_ref: source_item_ref.to_owned(),
            reason_code: "settings_root_must_be_object",
        });
    };
    Ok(values.into_iter().collect())
}

fn parse_jetbrains_options(
    bytes: &[u8],
    source_item_ref: &str,
) -> Result<BTreeMap<String, String>, ImportExecutionError> {
    let text = std::str::from_utf8(bytes).map_err(|_| ImportExecutionError::MalformedInput {
        source_item_ref: source_item_ref.to_owned(),
        reason_code: "xml_must_be_utf8",
    })?;
    let malformed = |reason_code: &'static str| ImportExecutionError::MalformedInput {
        source_item_ref: source_item_ref.to_owned(),
        reason_code,
    };
    let mut index = 0usize;
    let mut stack: Vec<String> = Vec::new();
    let mut root_seen = false;
    let mut options = BTreeMap::new();
    while index < text.len() {
        let Some(relative_open) = text[index..].find('<') else {
            if !text[index..].trim().is_empty() {
                return Err(malformed("xml_text_content_not_supported"));
            }
            break;
        };
        let open = index + relative_open;
        if !text[index..open].trim().is_empty() {
            return Err(malformed("xml_text_content_not_supported"));
        }
        if text[open..].starts_with("<!--") {
            let end = text[open + 4..]
                .find("-->")
                .map(|offset| open + 4 + offset + 3)
                .ok_or_else(|| malformed("xml_comment_unterminated"))?;
            index = end;
            continue;
        }
        if text[open..].starts_with("<?") {
            let end = text[open + 2..]
                .find("?>")
                .map(|offset| open + 2 + offset + 2)
                .ok_or_else(|| malformed("xml_declaration_unterminated"))?;
            let instruction = text[open + 2..end - 2].trim();
            let xml_declaration = instruction
                .strip_prefix("xml")
                .is_some_and(|rest| rest.chars().next().map_or(true, char::is_whitespace));
            if root_seen || !xml_declaration {
                return Err(malformed("xml_processing_instruction_forbidden"));
            }
            index = end;
            continue;
        }
        if text[open..].starts_with("<!") {
            return Err(malformed("xml_declaration_forbidden"));
        }
        let close =
            find_tag_end(text, open + 1).ok_or_else(|| malformed("xml_tag_unterminated"))?;
        let mut body = text[open + 1..close].trim();
        if let Some(remainder) = body.strip_prefix('/') {
            let name = remainder.trim();
            if !valid_xml_name(name) {
                return Err(malformed("xml_close_tag_invalid"));
            }
            let opened = stack
                .pop()
                .ok_or_else(|| malformed("xml_close_without_open"))?;
            if opened != name {
                return Err(malformed("xml_tag_mismatch"));
            }
            index = close + 1;
            continue;
        }
        let self_closing = body.ends_with('/');
        if self_closing {
            body = body[..body.len() - 1].trim_end();
        }
        let (name, attributes) =
            parse_xml_start_tag(body).map_err(|reason_code| malformed(reason_code))?;
        if stack.is_empty() {
            if root_seen || !matches!(name.as_str(), "project" | "component") {
                return Err(malformed("xml_root_not_supported"));
            }
            root_seen = true;
        }
        if name == "option" {
            let option_name = attributes
                .get("name")
                .ok_or_else(|| malformed("xml_option_name_missing"))?;
            let option_value = attributes
                .get("value")
                .ok_or_else(|| malformed("xml_option_value_missing"))?;
            if option_name.is_empty() || option_name.len() > 160 {
                return Err(malformed("xml_option_name_invalid"));
            }
            if option_value.len() > 512 || option_value.chars().any(char::is_control) {
                return Err(malformed("xml_option_value_invalid"));
            }
            if let Some(previous) = options.insert(option_name.clone(), option_value.clone()) {
                if previous != *option_value {
                    return Err(malformed("xml_option_ambiguous"));
                }
            }
        }
        if !self_closing {
            stack.push(name);
        }
        index = close + 1;
    }
    if !root_seen || !stack.is_empty() {
        return Err(malformed("xml_document_incomplete"));
    }
    Ok(options)
}

fn find_tag_end(text: &str, start: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut quote = None;
    let mut index = start;
    while index < bytes.len() {
        match bytes[index] {
            b'\'' | b'"' if quote.is_none() => quote = Some(bytes[index]),
            byte if quote == Some(byte) => quote = None,
            b'>' if quote.is_none() => return Some(index),
            _ => {}
        }
        index += 1;
    }
    None
}

fn parse_xml_start_tag(body: &str) -> Result<(String, BTreeMap<String, String>), &'static str> {
    let bytes = body.as_bytes();
    let mut index = 0usize;
    skip_ascii_whitespace(bytes, &mut index);
    let name = parse_xml_name(body, bytes, &mut index).ok_or("xml_tag_name_invalid")?;
    let mut attributes = BTreeMap::new();
    loop {
        skip_ascii_whitespace(bytes, &mut index);
        if index == bytes.len() {
            break;
        }
        let attribute_name =
            parse_xml_name(body, bytes, &mut index).ok_or("xml_attribute_name_invalid")?;
        skip_ascii_whitespace(bytes, &mut index);
        if bytes.get(index) != Some(&b'=') {
            return Err("xml_attribute_equals_missing");
        }
        index += 1;
        skip_ascii_whitespace(bytes, &mut index);
        let quote = *bytes.get(index).ok_or("xml_attribute_quote_missing")?;
        if !matches!(quote, b'\'' | b'"') {
            return Err("xml_attribute_quote_missing");
        }
        index += 1;
        let value_start = index;
        while bytes.get(index).is_some_and(|byte| *byte != quote) {
            index += 1;
        }
        if index >= bytes.len() {
            return Err("xml_attribute_unterminated");
        }
        let decoded = decode_xml_entities(&body[value_start..index])?;
        index += 1;
        if attributes.insert(attribute_name, decoded).is_some() {
            return Err("xml_duplicate_attribute");
        }
    }
    Ok((name, attributes))
}

fn parse_xml_name(body: &str, bytes: &[u8], index: &mut usize) -> Option<String> {
    let start = *index;
    while bytes.get(*index).is_some_and(|byte| {
        byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b':' | b'.')
    }) {
        *index += 1;
    }
    if *index == start {
        None
    } else {
        Some(body[start..*index].to_owned())
    }
}

fn valid_xml_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b':' | b'.'))
}

fn skip_ascii_whitespace(bytes: &[u8], index: &mut usize) {
    while bytes.get(*index).is_some_and(u8::is_ascii_whitespace) {
        *index += 1;
    }
}

fn decode_xml_entities(value: &str) -> Result<String, &'static str> {
    let mut decoded = String::with_capacity(value.len());
    let mut remaining = value;
    while let Some(offset) = remaining.find('&') {
        decoded.push_str(&remaining[..offset]);
        let after = &remaining[offset + 1..];
        let end = after.find(';').ok_or("xml_entity_unterminated")?;
        let entity = &after[..end];
        match entity {
            "amp" => decoded.push('&'),
            "lt" => decoded.push('<'),
            "gt" => decoded.push('>'),
            "quot" => decoded.push('"'),
            "apos" => decoded.push('\''),
            _ => return Err("xml_entity_not_supported"),
        }
        remaining = &after[end + 1..];
    }
    decoded.push_str(remaining);
    Ok(decoded)
}

fn json_bounded_integer(value: &serde_json::Value, minimum: i64, maximum: i64) -> Option<i64> {
    value
        .as_i64()
        .filter(|value| (minimum..=maximum).contains(value))
}

fn resolver_value(value: &ImportSettingValue) -> SettingValue {
    match value {
        ImportSettingValue::Boolean(value) => SettingValue::Boolean(*value),
        ImportSettingValue::Integer(value) => SettingValue::Integer(*value),
        ImportSettingValue::Text(value) => SettingValue::String(value.clone()),
    }
}

fn json_bounded_text(value: &serde_json::Value, maximum_length: usize) -> Option<String> {
    value
        .as_str()
        .filter(|value| {
            !value.is_empty()
                && value.len() <= maximum_length
                && !value.chars().any(char::is_control)
        })
        .map(str::to_owned)
}

fn parse_bounded_integer(value: &str, minimum: i64, maximum: i64) -> Option<i64> {
    value
        .parse::<i64>()
        .ok()
        .filter(|value| (minimum..=maximum).contains(value))
}

fn vscode_theme_mode(theme_name: &str) -> Option<&'static str> {
    match theme_name {
        "Dark+ (default dark)"
        | "Dark+"
        | "Default Dark Modern"
        | "Default Dark+"
        | "Visual Studio Dark" => Some("dark"),
        "Light+ (default light)"
        | "Light+"
        | "Default Light Modern"
        | "Default Light+"
        | "Quiet Light"
        | "Visual Studio Light" => Some("light"),
        _ => None,
    }
}

fn vscode_motion_mode(source_mode: &str) -> Option<&'static str> {
    match source_mode {
        "on" => Some("reduced"),
        "off" => Some("full"),
        _ => None,
    }
}

fn apply_gate_for_rows(rows: &[ImportExecutionRow]) -> ImportPreviewApplyGate {
    if rows
        .iter()
        .any(|row| row.decision == ImportExecutionRowDecision::Admitted)
    {
        ImportPreviewApplyGate::AllowedCheckpointRequired
    } else if rows
        .iter()
        .any(|row| row.decision == ImportExecutionRowDecision::NoChange)
    {
        ImportPreviewApplyGate::NoChanges
    } else {
        ImportPreviewApplyGate::BlockedNoSupportedSettings
    }
}

fn source_root_ref(canonical_source_root: &Path) -> String {
    format!(
        "source-root:{}",
        digest_suffix(&aureline_history::body_object_id(
            canonical_source_root.as_os_str().as_encoded_bytes()
        ))
    )
}

#[allow(clippy::too_many_arguments)]
fn compute_plan_digest(
    import_review_ref: &str,
    source_root_ref: &str,
    source_classification: CompetitorConfigClassification,
    source_snapshot_digest: &str,
    target_profile_ref: &str,
    policy_epoch: &str,
    target_state_digest: &str,
    rows: &[ImportExecutionRow],
    apply_gate: ImportPreviewApplyGate,
) -> Result<String, ImportExecutionError> {
    digest_serializable(&(
        IMPORT_EXECUTION_PREVIEW_RECORD_KIND,
        IMPORT_EXECUTION_SCHEMA_VERSION,
        import_review_ref,
        source_root_ref,
        source_classification,
        source_snapshot_digest,
        target_profile_ref,
        policy_epoch,
        target_state_digest,
        rows,
        apply_gate,
    ))
}

fn validate_preview_shape(
    preview: &ExecutableImportPreview,
    authority: &LivePreviewAuthority,
) -> Result<(), ImportExecutionError> {
    let rebound_plan_digest = compute_plan_digest(
        &preview.import_review_ref,
        &preview.source_root_ref,
        preview.source_classification,
        &preview.source_snapshot_digest,
        &preview.target_profile_ref,
        &preview.policy_epoch,
        &preview.target_state_digest,
        &preview.rows,
        preview.apply_gate,
    )?;
    if preview.record_kind != IMPORT_EXECUTION_PREVIEW_RECORD_KIND
        || preview.schema_version != IMPORT_EXECUTION_SCHEMA_VERSION
        || preview.plan_digest != authority.plan_digest
        || rebound_plan_digest != authority.plan_digest
        || preview.source_snapshot_digest != authority.source_snapshot_digest
        || preview.target_state_digest != authority.target_state_digest
        || preview.target_profile_ref != format!("imported-profile:{}", authority.profile_key)
        || preview.source_root_ref != source_root_ref(&authority.canonical_source_root)
        || preview.preview_ref
            != format!(
                "import-execution-preview:{}",
                digest_suffix(&authority.plan_digest)
            )
        || preview.generated_at != authority.generated_at
    {
        return Err(ImportExecutionError::PreviewStale {
            reason_code: "preview_packet_modified",
        });
    }
    Ok(())
}

fn validate_request_label(
    value: &str,
    reason_code: &'static str,
) -> Result<(), ImportExecutionError> {
    if value.trim().is_empty() || value.len() > 256 || value.chars().any(char::is_control) {
        return Err(ImportExecutionError::InvalidRequest { reason_code });
    }
    Ok(())
}

fn validate_import_review_envelope(
    review: &ImportReviewRecord,
) -> Result<(), ImportExecutionError> {
    if review.record_kind != "import_review_record"
        || review.schema_version != 1
        || !valid_import_review_ref(&review.import_review_id)
    {
        return Err(ImportExecutionError::InvalidRequest {
            reason_code: "import_review_envelope_invalid",
        });
    }
    Ok(())
}

fn valid_import_review_ref(value: &str) -> bool {
    value
        .strip_prefix("import-review-")
        .is_some_and(|suffix| valid_lower_hex(suffix, 16))
}

fn valid_object_digest(value: &str) -> bool {
    value
        .strip_prefix("obj:blake3:")
        .is_some_and(|suffix| valid_lower_hex(suffix, 64))
}

fn valid_digest_ref(value: &str, prefix: &str) -> bool {
    value
        .strip_prefix(prefix)
        .is_some_and(|suffix| valid_lower_hex(suffix, 64))
}

fn valid_lower_hex(value: &str, expected_length: usize) -> bool {
    value.len() == expected_length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn validate_policy_epoch(policy_epoch: &str) -> Result<(), ImportExecutionError> {
    if policy_epoch.is_empty()
        || policy_epoch.len() > 128
        || !policy_epoch
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-'))
    {
        return Err(ImportExecutionError::InvalidRequest {
            reason_code: "policy_epoch_invalid",
        });
    }
    Ok(())
}

fn validate_idempotency_token(token: &str) -> Result<(), ImportExecutionError> {
    if token.is_empty()
        || token.len() > 128
        || !token
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-'))
    {
        return Err(ImportExecutionError::InvalidRequest {
            reason_code: "idempotency_token_invalid",
        });
    }
    Ok(())
}

fn profile_key(destination_workspace_target: &str) -> String {
    digest_suffix(&aureline_history::body_object_id(
        destination_workspace_target.as_bytes(),
    ))
    .to_owned()
}

fn validate_profile_key(profile_key: &str) -> Result<(), ImportExecutionError> {
    if profile_key.len() != 64
        || !profile_key
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(ImportExecutionError::DurableStateUnavailable {
            reason_code: "profile_key_invalid",
        });
    }
    Ok(())
}

fn checkpoint_ref(profile_key: &str, plan_digest: &str, token_digest: &str) -> String {
    let identity = format!("{profile_key}\n{plan_digest}\n{token_digest}");
    let checkpoint_key = digest_suffix(&aureline_history::body_object_id(identity.as_bytes()));
    format!("import-checkpoint:{profile_key}:{checkpoint_key}")
}

fn parse_checkpoint_ref(checkpoint_ref: &str) -> Result<(&str, &str), ImportExecutionError> {
    let mut parts = checkpoint_ref.split(':');
    let valid = parts.next() == Some("import-checkpoint");
    let profile_key = parts.next().unwrap_or_default();
    let checkpoint_key = parts.next().unwrap_or_default();
    if !valid
        || parts.next().is_some()
        || validate_profile_key(profile_key).is_err()
        || validate_profile_key(checkpoint_key).is_err()
    {
        return Err(ImportExecutionError::CheckpointUnavailable);
    }
    Ok((profile_key, checkpoint_key))
}

fn validate_durable_state(
    state: &ImportedProfileState,
    expected_profile_ref: &str,
) -> Result<(), ImportExecutionError> {
    let profile_key = state
        .profile_ref
        .strip_prefix("imported-profile:")
        .unwrap_or_default();
    if state.record_kind != IMPORTED_PROFILE_STATE_RECORD_KIND
        || state.schema_version != IMPORT_EXECUTION_SCHEMA_VERSION
        || state.profile_ref != expected_profile_ref
        || state.history.len() > MAX_HISTORY_ROWS
        || state.revision != state.history.len() as u64
        || validate_profile_key(profile_key).is_err()
        || state.updated_at.is_empty()
        || state.updated_at.len() > 64
    {
        return Err(ImportExecutionError::DurableStateUnavailable {
            reason_code: "profile_state_contract_invalid",
        });
    }
    for (setting_id, record) in &state.settings {
        if !valid_setting_id(setting_id)
            || record.source_setting_ref.is_empty()
            || record.source_setting_ref.len() > 320
            || record.source_setting_ref.chars().any(char::is_control)
            || record.imported_at.is_empty()
            || record.imported_at.len() > 64
            || matches!(&record.value, ImportSettingValue::Text(value) if value.len() > 128 || value.chars().any(char::is_control))
        {
            return Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "profile_setting_contract_invalid",
            });
        }
    }
    for entry in &state.history {
        let checkpoint_profile_key = parse_checkpoint_ref(&entry.checkpoint_ref)
            .map(|(profile_key, _)| profile_key)
            .unwrap_or_default();
        if !valid_object_digest(&entry.idempotency_token_digest)
            || !valid_digest_ref(&entry.preview_ref, "import-execution-preview:")
            || !valid_import_review_ref(&entry.import_review_ref)
            || !valid_digest_ref(&entry.source_root_ref, "source-root:")
            || !valid_object_digest(&entry.source_snapshot_digest)
            || validate_policy_epoch(&entry.policy_epoch).is_err()
            || !valid_object_digest(&entry.plan_digest)
            || checkpoint_profile_key != profile_key
            || !valid_object_digest(&entry.result_settings_digest)
            || entry.occurred_at.is_empty()
            || entry.occurred_at.len() > 64
            || entry
                .changed_setting_ids
                .iter()
                .any(|id| !valid_setting_id(id))
            || entry
                .changed_setting_ids
                .windows(2)
                .any(|pair| pair[0] >= pair[1])
        {
            return Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "profile_history_contract_invalid",
            });
        }
    }
    if let Some(latest) = state.history.last() {
        if settings_digest(&state.settings)? != latest.result_settings_digest {
            return Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "profile_history_result_digest_mismatch",
            });
        }
    } else if !state.settings.is_empty() {
        return Err(ImportExecutionError::DurableStateUnavailable {
            reason_code: "profile_settings_without_history",
        });
    }
    Ok(())
}

fn valid_setting_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        })
}

fn validate_checkpoint(checkpoint: &ImportExecutionCheckpoint) -> Result<(), ImportExecutionError> {
    let (profile_key, _) = parse_checkpoint_ref(&checkpoint.checkpoint_ref)?;
    if checkpoint.record_kind != IMPORT_EXECUTION_CHECKPOINT_RECORD_KIND
        || checkpoint.schema_version != IMPORT_EXECUTION_SCHEMA_VERSION
        || checkpoint.profile_key != profile_key
        || checkpoint.target_profile_ref != format!("imported-profile:{profile_key}")
        || !valid_digest_ref(&checkpoint.preview_ref, "import-execution-preview:")
        || !valid_import_review_ref(&checkpoint.import_review_ref)
        || !valid_digest_ref(&checkpoint.source_root_ref, "source-root:")
        || validate_policy_epoch(&checkpoint.policy_epoch).is_err()
        || !valid_object_digest(&checkpoint.idempotency_token_digest)
        || !valid_object_digest(&checkpoint.plan_digest)
        || !valid_object_digest(&checkpoint.source_snapshot_digest)
        || !(checkpoint.prior_state_digest == ABSENT_STATE_DIGEST
            || valid_object_digest(&checkpoint.prior_state_digest))
        || !valid_object_digest(&checkpoint.expected_applied_settings_digest)
        || checkpoint.created_at.is_empty()
        || checkpoint.created_at.len() > 64
        || checkpoint.checkpoint_ref
            != checkpoint_ref(
                profile_key,
                &checkpoint.plan_digest,
                &checkpoint.idempotency_token_digest,
            )
    {
        return Err(ImportExecutionError::CheckpointUnavailable);
    }
    if state_digest(checkpoint.prior_state.as_ref())? != checkpoint.prior_state_digest {
        return Err(ImportExecutionError::CheckpointUnavailable);
    }
    if let Some(prior) = checkpoint.prior_state.as_ref() {
        validate_durable_state(prior, &checkpoint.target_profile_ref)?;
    }
    Ok(())
}

fn checkpoints_equivalent(
    left: &ImportExecutionCheckpoint,
    right: &ImportExecutionCheckpoint,
) -> bool {
    left.record_kind == right.record_kind
        && left.schema_version == right.schema_version
        && left.checkpoint_ref == right.checkpoint_ref
        && left.profile_key == right.profile_key
        && left.target_profile_ref == right.target_profile_ref
        && left.preview_ref == right.preview_ref
        && left.import_review_ref == right.import_review_ref
        && left.source_root_ref == right.source_root_ref
        && left.policy_epoch == right.policy_epoch
        && left.idempotency_token_digest == right.idempotency_token_digest
        && left.plan_digest == right.plan_digest
        && left.source_snapshot_digest == right.source_snapshot_digest
        && left.prior_state_digest == right.prior_state_digest
        && left.expected_applied_settings_digest == right.expected_applied_settings_digest
        && left.prior_state == right.prior_state
}

fn settings_digest(
    settings: &BTreeMap<String, ImportedProfileSettingRecord>,
) -> Result<String, ImportExecutionError> {
    let effective_rows: Vec<_> = settings
        .iter()
        .map(|(setting_id, record)| {
            (
                setting_id,
                &record.value,
                record.source_classification,
                &record.source_setting_ref,
            )
        })
        .collect();
    digest_serializable(&effective_rows)
}

fn state_digest(state: Option<&ImportedProfileState>) -> Result<String, ImportExecutionError> {
    match state {
        Some(state) => digest_serializable(state),
        None => Ok(ABSENT_STATE_DIGEST.to_owned()),
    }
}

fn digest_serializable<T: Serialize + ?Sized>(value: &T) -> Result<String, ImportExecutionError> {
    let bytes =
        serde_json::to_vec(value).map_err(|_| ImportExecutionError::DurableStateUnavailable {
            reason_code: "canonical_serialization_failed",
        })?;
    Ok(aureline_history::body_object_id(&bytes))
}

fn digest_suffix(digest: &str) -> &str {
    digest.strip_prefix("obj:blake3:").unwrap_or(digest)
}

fn read_optional_durable_file(
    state_root: &Path,
    path: &Path,
) -> Result<Option<Vec<u8>>, ImportExecutionError> {
    if state_root.as_os_str().is_empty() || !path.starts_with(state_root) {
        return Err(ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_path_outside_state_root",
        });
    }
    let metadata = match std::fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => {
            return Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_file_metadata_failed",
            })
        }
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_file_type_unsafe",
        });
    }
    if metadata.len() > MAX_DURABLE_FILE_BYTES {
        return Err(ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_file_too_large",
        });
    }
    ensure_existing_child_chain_has_no_symlink(state_root, path)?;
    let mut file = File::open(path).map_err(|_| ImportExecutionError::DurableStateUnavailable {
        reason_code: "durable_file_open_failed",
    })?;
    let opened = file
        .metadata()
        .map_err(|_| ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_file_metadata_failed",
        })?;
    if !opened.is_file() || opened.len() > MAX_DURABLE_FILE_BYTES {
        return Err(ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_file_too_large",
        });
    }
    let mut bytes = Vec::with_capacity(opened.len() as usize);
    file.by_ref()
        .take(MAX_DURABLE_FILE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_file_read_failed",
        })?;
    if bytes.len() as u64 > MAX_DURABLE_FILE_BYTES {
        return Err(ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_file_too_large",
        });
    }
    let after = std::fs::symlink_metadata(path).map_err(|_| {
        ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_file_changed_during_read",
        }
    })?;
    if after.file_type().is_symlink() || !after.is_file() || after.len() != opened.len() {
        return Err(ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_file_changed_during_read",
        });
    }
    Ok(Some(bytes))
}

fn write_new_json<T: Serialize>(
    state_root: &Path,
    path: &Path,
    value: &T,
) -> Result<(), ImportExecutionError> {
    let mut bytes = serde_json::to_vec_pretty(value).map_err(|_| {
        ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_serialization_failed",
        }
    })?;
    bytes.push(b'\n');
    if bytes.len() as u64 > MAX_DURABLE_FILE_BYTES {
        return Err(ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_file_too_large",
        });
    }
    let parent = prepare_secure_parent(state_root, path)?;
    reject_symlink_target(path)?;
    let (temporary_path, mut temporary) = create_temporary_file(&parent, path)?;
    let result = (|| {
        temporary
            .write_all(&bytes)
            .map_err(|_| ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_temp_write_failed",
            })?;
        temporary
            .sync_all()
            .map_err(|_| ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_temp_sync_failed",
            })?;
        drop(temporary);
        reject_symlink_target(path)?;
        std::fs::hard_link(&temporary_path, path).map_err(|_| {
            ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_create_new_failed",
            }
        })?;
        std::fs::remove_file(&temporary_path).map_err(|_| {
            ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_temp_cleanup_failed",
            }
        })?;
        sync_directory(&parent)
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temporary_path);
    }
    result
}

fn prepare_secure_parent(state_root: &Path, path: &Path) -> Result<PathBuf, ImportExecutionError> {
    if state_root.as_os_str().is_empty() || !path.starts_with(state_root) {
        return Err(ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_path_outside_state_root",
        });
    }
    ensure_directory_not_symlink(state_root)?;
    let parent = path
        .parent()
        .ok_or(ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_parent_missing",
        })?;
    let relative_parent = parent.strip_prefix(state_root).map_err(|_| {
        ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_path_outside_state_root",
        }
    })?;
    let mut current = state_root.to_path_buf();
    for component in relative_parent.components() {
        let Component::Normal(component) = component else {
            return Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_path_component_invalid",
            });
        };
        current.push(component);
        ensure_directory_not_symlink(&current)?;
    }
    Ok(parent.to_path_buf())
}

fn ensure_directory_not_symlink(path: &Path) -> Result<(), ImportExecutionError> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.is_dir() {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "durable_directory_type_unsafe",
                });
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            match std::fs::create_dir(path) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(_) => {
                    return Err(ImportExecutionError::DurableStateUnavailable {
                        reason_code: "durable_directory_create_failed",
                    })
                }
            }
            let metadata = std::fs::symlink_metadata(path).map_err(|_| {
                ImportExecutionError::DurableStateUnavailable {
                    reason_code: "durable_directory_metadata_failed",
                }
            })?;
            if metadata.file_type().is_symlink() || !metadata.is_dir() {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "durable_directory_type_unsafe",
                });
            }
        }
        Err(_) => {
            return Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_directory_metadata_failed",
            })
        }
    }
    Ok(())
}

fn ensure_existing_child_chain_has_no_symlink(
    state_root: &Path,
    path: &Path,
) -> Result<(), ImportExecutionError> {
    let root_metadata = std::fs::symlink_metadata(state_root).map_err(|_| {
        ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_parent_metadata_failed",
        }
    })?;
    if root_metadata.file_type().is_symlink() || !root_metadata.is_dir() {
        return Err(ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_parent_type_unsafe",
        });
    }
    let relative_parent = path
        .parent()
        .and_then(|parent| parent.strip_prefix(state_root).ok())
        .ok_or(ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_path_outside_state_root",
        })?;
    let mut current = state_root.to_path_buf();
    for component in relative_parent.components() {
        let Component::Normal(component) = component else {
            return Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_path_component_invalid",
            });
        };
        current.push(component);
        let metadata = match std::fs::symlink_metadata(&current) {
            Ok(metadata) => metadata,
            Err(_) => {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "durable_parent_metadata_failed",
                })
            }
        };
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            return Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_parent_type_unsafe",
            });
        }
    }
    Ok(())
}

fn reject_symlink_target(path: &Path) -> Result<(), ImportExecutionError> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
            Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_target_type_unsafe",
            })
        }
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(_) => Err(ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_target_metadata_failed",
        }),
    }
}

fn create_temporary_file(
    parent: &Path,
    destination: &Path,
) -> Result<(PathBuf, File), ImportExecutionError> {
    let filename = destination
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("profile.json");
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    for attempt in 0..32u32 {
        let temporary_path = parent.join(format!(
            ".{filename}.tmp.{}.{}.{}",
            std::process::id(),
            nonce,
            attempt
        ));
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        configure_private_file_mode(&mut options);
        match options.open(&temporary_path) {
            Ok(file) => return Ok((temporary_path, file)),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(_) => {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "durable_temp_create_failed",
                })
            }
        }
    }
    Err(ImportExecutionError::DurableStateUnavailable {
        reason_code: "durable_temp_collision_limit",
    })
}

#[cfg(unix)]
fn configure_private_file_mode(options: &mut OpenOptions) {
    use std::os::unix::fs::OpenOptionsExt;
    options.mode(0o600);
}

#[cfg(not(unix))]
fn configure_private_file_mode(_options: &mut OpenOptions) {}

fn sync_directory(path: &Path) -> Result<(), ImportExecutionError> {
    #[cfg(unix)]
    {
        File::open(path)
            .and_then(|directory| directory.sync_all())
            .map_err(|_| ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_directory_sync_failed",
            })?;
    }
    #[cfg(not(unix))]
    {
        let _ = path;
    }
    Ok(())
}

pub(super) fn utc_timestamp_now() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    utc_timestamp_from_seconds(seconds)
}

fn utc_timestamp_from_seconds(seconds: u64) -> String {
    let days = (seconds / 86_400).min(i64::MAX as u64) as i64;
    let seconds_of_day = seconds % 86_400;
    let (year, month, day) = civil_date_from_unix_days(days);
    let hour = seconds_of_day / 3_600;
    let minute = (seconds_of_day % 3_600) / 60;
    let second = seconds_of_day % 60;
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

fn civil_date_from_unix_days(days_since_epoch: i64) -> (i64, u64, u64) {
    let days = days_since_epoch + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    (year, month as u64, day as u64)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::import::CompetitorConfigClassifier;

    const POLICY_EPOCH: &str = "policy-epoch:test:1";

    struct TestLayout {
        _temp: tempfile::TempDir,
        source_root: PathBuf,
        state_root: PathBuf,
    }

    impl TestLayout {
        fn vscode(settings: &str) -> Self {
            let temp = tempfile::tempdir().expect("tempdir");
            let source_root = temp.path().join("source");
            fs::create_dir_all(source_root.join(".vscode")).expect("vscode marker");
            fs::write(source_root.join(".vscode/settings.json"), settings)
                .expect("settings fixture");
            let state_root = temp.path().join("state");
            Self {
                _temp: temp,
                source_root,
                state_root,
            }
        }

        fn jetbrains(project_xml: &str) -> Self {
            let temp = tempfile::tempdir().expect("tempdir");
            let source_root = temp.path().join("source");
            fs::create_dir_all(source_root.join(".idea/codeStyles")).expect("idea marker");
            fs::write(
                source_root.join(".idea/codeStyles/Project.xml"),
                project_xml,
            )
            .expect("project style fixture");
            let state_root = temp.path().join("state");
            Self {
                _temp: temp,
                source_root,
                state_root,
            }
        }

        fn review(&self, target: &str) -> ImportReviewRecord {
            CompetitorConfigClassifier::new().build_review(&self.source_root, target)
        }

        fn store(&self) -> ImportedProfileStore {
            ImportedProfileStore::new(&self.state_root)
        }
    }

    fn setting_row<'a>(
        preview: &'a ExecutableImportPreview,
        target: &str,
    ) -> &'a ImportExecutionRow {
        preview
            .rows
            .iter()
            .find(|row| row.target_setting_id.as_deref() == Some(target))
            .unwrap_or_else(|| panic!("missing row for {target}"))
    }

    #[test]
    fn vscode_preview_is_body_derived_and_redacts_sensitive_keys_and_values() {
        let layout = TestLayout::vscode(
            r#"{
                "editor.tabSize": 2,
                "editor.insertSpaces": true,
                "editor.formatOnSave": true,
                "workbench.colorTheme": "Quiet Light",
                "unknown.harmless": {"nested": true},
                "service.apiToken": "never-persist-this-secret"
            }"#,
        );
        fs::write(
            layout.source_root.join(".vscode/tasks.json"),
            r#"{"tasks":[{"command":"never-run"}]}"#,
        )
        .expect("tasks fixture");
        fs::write(
            layout.source_root.join(".vscode/extensions.json"),
            r#"{"recommendations":["unsafe.extension"]}"#,
        )
        .expect("extension fixture");

        let preview = layout
            .store()
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("preview");

        assert_eq!(preview.admitted_mutation_count(), 3);
        assert_eq!(
            setting_row(&preview, "editor.tab_size").after_value,
            Some(ImportSettingValue::Integer(2))
        );
        assert!(preview.blocked_authority_count() >= 3);
        assert!(preview.rows.iter().any(|row| {
            row.source_setting_ref == ".vscode/settings.json#editor.insertSpaces"
                && row.decision == ImportExecutionRowDecision::Unsupported
        }));
        assert!(preview.rows.iter().any(|row| {
            row.source_setting_ref
                .starts_with(".vscode/settings.json#redacted-key:")
                && row.decision == ImportExecutionRowDecision::BlockedAuthority
        }));
        let exported = serde_json::to_string(&preview).expect("preview export");
        assert!(!exported.contains("never-persist-this-secret"));
        assert!(!exported.contains("service.apiToken"));
        assert!(!exported.contains(&layout.source_root.display().to_string()));
        assert_eq!(
            preview.apply_gate,
            ImportPreviewApplyGate::AllowedCheckpointRequired
        );
    }

    #[test]
    fn apply_writes_only_dedicated_profile_with_pre_apply_checkpoint() {
        let layout = TestLayout::vscode(
            r#"{"editor.tabSize":2,"editor.formatOnSave":true,"files.autoSave":"afterDelay"}"#,
        );
        let store = layout.store();
        let review = layout.review("profile:default");
        let preview = store.preview(&review, POLICY_EPOCH).expect("preview");
        let outcome = store
            .apply(&preview, "apply-token-001", POLICY_EPOCH)
            .expect("apply");

        assert_eq!(outcome.disposition, ImportApplyDisposition::Applied);
        let checkpoint_ref = outcome.checkpoint_ref.as_deref().expect("checkpoint ref");
        let state = store
            .load_profile_for_target("profile:default")
            .expect("load")
            .expect("profile state");
        assert_eq!(state.revision, 1);
        assert_eq!(state.settings.len(), 2);
        assert_eq!(
            state.settings["editor.tab_size"].value,
            ImportSettingValue::Integer(2)
        );
        assert_eq!(
            state.settings["editor.format_on_save"].value,
            ImportSettingValue::Boolean(true)
        );
        assert!(!state.settings.keys().any(|key| key.contains("auto")));
        assert_eq!(state.history[0].checkpoint_ref, checkpoint_ref);
        assert!(!state.history[0]
            .idempotency_token_digest
            .contains("apply-token-001"));
        let (profile_key, checkpoint_key) =
            parse_checkpoint_ref(checkpoint_ref).expect("checkpoint parses");
        assert!(layout
            .state_root
            .join("imported_profiles")
            .join(profile_key)
            .join("checkpoints")
            .join(format!("{checkpoint_key}.json"))
            .is_file());
    }

    #[test]
    fn applied_profile_projects_through_the_real_effective_settings_resolver() {
        use aureline_settings::resolver::EffectiveSettingsResolver;

        let layout = TestLayout::vscode(
            r#"{
                "editor.tabSize": 2,
                "editor.formatOnSave": true,
                "editor.insertSpaces": false,
                "workbench.colorTheme": "Quiet Light",
                "workbench.reduceMotion": "on"
            }"#,
        );
        let store = layout.store();
        let preview = store
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("preview");
        assert_eq!(preview.admitted_mutation_count(), 4);
        store
            .apply(&preview, "resolver-projection", POLICY_EPOCH)
            .expect("apply");

        let registry = SchemaRegistry::with_seed_catalog();
        let overlay = store
            .load_resolver_overlay_for_target("profile:default", &registry)
            .expect("project overlay")
            .expect("imported overlay");
        assert_eq!(overlay.scope, SettingScope::ImportedProfileDefault);
        assert_eq!(overlay.values.len(), 4);
        assert!(!overlay.values.contains_key("editor.insert_spaces"));

        let mut resolver = EffectiveSettingsResolver::new(registry);
        resolver.set_overlay(overlay).expect("install overlay");
        let tab_size = resolver
            .resolve("editor.tab_size")
            .expect("resolve tab size");
        assert_eq!(tab_size.value, SettingValue::Integer(2));
        assert_eq!(tab_size.winning_scope, SettingScope::ImportedProfileDefault);
        assert_eq!(
            resolver
                .resolve("editor.format_on_save")
                .expect("resolve format on save")
                .value,
            SettingValue::Boolean(true)
        );
        assert_eq!(
            resolver.resolve("ui.theme").expect("resolve theme").value,
            SettingValue::String("light".to_owned())
        );
        assert_eq!(
            resolver.resolve("ui.motion").expect("resolve motion").value,
            SettingValue::String("reduced".to_owned())
        );
    }

    #[test]
    fn resolver_projection_fails_closed_on_unknown_out_of_range_and_invalid_enum_rows() {
        let layout =
            TestLayout::vscode(r#"{"editor.tabSize":2,"workbench.colorTheme":"Quiet Light"}"#);
        let store = layout.store();
        let preview = store
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("preview");
        store
            .apply(&preview, "projection-validation", POLICY_EPOCH)
            .expect("apply");
        let state = store
            .load_profile_for_target("profile:default")
            .expect("load")
            .expect("profile state");
        let registry = SchemaRegistry::with_seed_catalog();

        let mut unknown = state.clone();
        unknown.settings.insert(
            "editor.insert_spaces".to_owned(),
            ImportedProfileSettingRecord {
                value: ImportSettingValue::Boolean(true),
                source_classification: CompetitorConfigClassification::VSCodeWorkspaceRoot,
                source_setting_ref: ".vscode/settings.json#editor.insertSpaces".to_owned(),
                imported_at: utc_timestamp_now(),
            },
        );
        let unknown_digest = settings_digest(&unknown.settings).expect("settings digest");
        unknown
            .history
            .last_mut()
            .expect("history")
            .result_settings_digest = unknown_digest;
        assert!(matches!(
            unknown.to_resolver_overlay(&registry),
            Err(ImportExecutionError::ResolverProjectionUnavailable {
                reason_code: "setting_not_registered",
                ..
            })
        ));

        let mut out_of_range = state.clone();
        out_of_range.settings["editor.tab_size"].value = ImportSettingValue::Integer(99);
        let out_of_range_digest = settings_digest(&out_of_range.settings).expect("settings digest");
        out_of_range
            .history
            .last_mut()
            .expect("history")
            .result_settings_digest = out_of_range_digest;
        assert!(matches!(
            out_of_range.to_resolver_overlay(&registry),
            Err(ImportExecutionError::ResolverProjectionUnavailable {
                reason_code: "setting_value_invalid",
                ..
            })
        ));

        let mut invalid_enum = state;
        invalid_enum.settings["ui.theme"].value =
            ImportSettingValue::Text("extension-theme-name".to_owned());
        let invalid_enum_digest = settings_digest(&invalid_enum.settings).expect("settings digest");
        invalid_enum
            .history
            .last_mut()
            .expect("history")
            .result_settings_digest = invalid_enum_digest;
        assert!(matches!(
            invalid_enum.to_resolver_overlay(&registry),
            Err(ImportExecutionError::ResolverProjectionUnavailable {
                reason_code: "setting_value_invalid",
                ..
            })
        ));
    }

    #[test]
    fn apply_is_idempotent_for_same_token_plan_and_effective_state() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let store = layout.store();
        let preview = store
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("preview");
        let first = store
            .apply(&preview, "same-token", POLICY_EPOCH)
            .expect("first apply");
        let second = store
            .apply(&preview, "same-token", POLICY_EPOCH)
            .expect("idempotent apply");
        assert_eq!(first.disposition, ImportApplyDisposition::Applied);
        assert_eq!(second.disposition, ImportApplyDisposition::AlreadyApplied);
        assert_eq!(first.checkpoint_ref, second.checkpoint_ref);
        assert_eq!(second.revision, 1);
        assert_eq!(
            store
                .load_profile_for_target("profile:default")
                .expect("load")
                .expect("state")
                .history
                .len(),
            1
        );

        let no_change = store
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("no-change preview");
        assert_eq!(no_change.apply_gate, ImportPreviewApplyGate::NoChanges);
        let no_change_outcome = store
            .apply(&no_change, "new-noop-token", POLICY_EPOCH)
            .expect("no-change apply");
        assert_eq!(
            no_change_outcome.disposition,
            ImportApplyDisposition::NoChanges
        );
        assert_eq!(no_change_outcome.checkpoint_ref, None);
    }

    #[test]
    fn serialized_preview_cannot_be_reused_as_apply_authority() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let store = layout.store();
        let preview = store
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("preview");
        let exported = serde_json::to_vec(&preview).expect("serialize");
        let restored: ExecutableImportPreview =
            serde_json::from_slice(&exported).expect("deserialize");
        assert!(!restored.carries_live_apply_authority());
        assert_eq!(
            store.apply(&restored, "replayed-token", POLICY_EPOCH),
            Err(ImportExecutionError::PreviewAuthorityMissing)
        );
    }

    #[test]
    fn every_apply_relevant_preview_field_is_bound_to_live_authority() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let store = layout.store();
        let preview = store
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("preview");

        let mut tampered_packets = Vec::new();

        let mut tampered = preview.clone();
        tampered.source_root_ref.push_str("-tampered");
        tampered_packets.push(tampered);

        let mut tampered = preview.clone();
        tampered.rows[0].reason_code.push_str("-tampered");
        tampered_packets.push(tampered);

        let mut tampered = preview.clone();
        tampered.apply_gate = ImportPreviewApplyGate::NoChanges;
        tampered_packets.push(tampered);

        let mut tampered = preview.clone();
        tampered.preview_ref.push_str("-tampered");
        tampered_packets.push(tampered);

        let mut tampered = preview;
        tampered.generated_at = "1970-01-01T00:00:00Z".to_owned();
        tampered_packets.push(tampered);

        for (index, tampered) in tampered_packets.iter().enumerate() {
            assert_eq!(
                store.apply(tampered, &format!("tamper-token-{index}"), POLICY_EPOCH),
                Err(ImportExecutionError::PreviewStale {
                    reason_code: "preview_packet_modified"
                })
            );
        }
        assert!(!layout.state_root.exists());
    }

    #[test]
    fn concurrent_stale_writers_cannot_both_commit() {
        use std::sync::{Arc, Barrier};

        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let store = layout.store();
        let review = layout.review("profile:default");
        let previews = [
            store.preview(&review, POLICY_EPOCH).expect("first preview"),
            store
                .preview(&review, POLICY_EPOCH)
                .expect("second preview"),
        ];
        let barrier = Arc::new(Barrier::new(3));
        let handles: Vec<_> = previews
            .into_iter()
            .enumerate()
            .map(|(index, preview)| {
                let store = store.clone();
                let barrier = Arc::clone(&barrier);
                std::thread::spawn(move || {
                    barrier.wait();
                    store.apply(&preview, &format!("concurrent-token-{index}"), POLICY_EPOCH)
                })
            })
            .collect();
        barrier.wait();
        let results: Vec<_> = handles
            .into_iter()
            .map(|handle| handle.join().expect("apply worker"))
            .collect();

        assert_eq!(
            results
                .iter()
                .filter(|result| {
                    matches!(
                        result,
                        Ok(ImportApplyOutcome {
                            disposition: ImportApplyDisposition::Applied,
                            ..
                        })
                    )
                })
                .count(),
            1
        );
        assert_eq!(
            results
                .iter()
                .filter(|result| {
                    matches!(
                        result,
                        Err(ImportExecutionError::ConcurrentMutation)
                            | Err(ImportExecutionError::PreviewStale {
                                reason_code: "target_state_changed"
                            })
                    )
                })
                .count(),
            1
        );
        let state = store
            .load_profile_for_target("profile:default")
            .expect("load")
            .expect("profile state");
        assert_eq!(state.revision, 1);
        assert_eq!(state.history.len(), 1);
    }

    #[test]
    fn checkpoint_only_crash_residue_is_resumable_without_a_stale_lock() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let store = layout.store();
        let preview = store
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("preview");
        let authority = preview.live_authority.as_ref().expect("live authority");
        let idempotency_token = "resume-after-checkpoint";
        let token_digest = aureline_history::body_object_id(idempotency_token.as_bytes());
        let checkpoint_ref =
            checkpoint_ref(&authority.profile_key, &preview.plan_digest, &token_digest);
        let mut projected_settings = BTreeMap::new();
        for row in preview
            .rows
            .iter()
            .filter(|row| row.decision == ImportExecutionRowDecision::Admitted)
        {
            projected_settings.insert(
                row.target_setting_id.clone().expect("target setting"),
                ImportedProfileSettingRecord {
                    value: row.after_value.clone().expect("setting value"),
                    source_classification: preview.source_classification,
                    source_setting_ref: row.source_setting_ref.clone(),
                    imported_at: preview.generated_at.clone(),
                },
            );
        }
        let checkpoint = ImportExecutionCheckpoint {
            record_kind: IMPORT_EXECUTION_CHECKPOINT_RECORD_KIND.to_owned(),
            schema_version: IMPORT_EXECUTION_SCHEMA_VERSION,
            checkpoint_ref: checkpoint_ref.clone(),
            profile_key: authority.profile_key.clone(),
            target_profile_ref: preview.target_profile_ref.clone(),
            preview_ref: preview.preview_ref.clone(),
            import_review_ref: preview.import_review_ref.clone(),
            source_root_ref: preview.source_root_ref.clone(),
            policy_epoch: preview.policy_epoch.clone(),
            idempotency_token_digest: token_digest,
            plan_digest: preview.plan_digest.clone(),
            source_snapshot_digest: preview.source_snapshot_digest.clone(),
            prior_state_digest: authority.target_state_digest.clone(),
            expected_applied_settings_digest: settings_digest(&projected_settings)
                .expect("settings digest"),
            prior_state: None,
            created_at: utc_timestamp_now(),
        };
        store
            .write_checkpoint(&checkpoint)
            .expect("checkpoint-only crash marker");

        let profile_directory = layout
            .state_root
            .join("imported_profiles")
            .join(&authority.profile_key);
        let revisions = profile_directory.join("revisions");
        fs::create_dir(&revisions).expect("revision directory");
        fs::write(
            revisions.join(".00000000000000000001.json.tmp.crashed"),
            b"partial",
        )
        .expect("crashed temporary revision");
        fs::write(
            profile_directory.join("mutation.lock"),
            b"legacy crash residue",
        )
        .expect("legacy lock residue");

        let outcome = store
            .apply(&preview, idempotency_token, POLICY_EPOCH)
            .expect("resume apply");
        assert_eq!(outcome.disposition, ImportApplyDisposition::Applied);
        assert_eq!(
            outcome.checkpoint_ref.as_deref(),
            Some(checkpoint_ref.as_str())
        );
        let state = store
            .load_profile_for_target("profile:default")
            .expect("load")
            .expect("profile state");
        assert_eq!(state.revision, 1);
        assert_eq!(
            state.settings["editor.tab_size"].value,
            ImportSettingValue::Integer(2)
        );
    }

    #[test]
    fn immutable_revision_fence_rejects_a_resumed_stale_owner() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let store = layout.store();
        let preview = store
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("preview");
        store
            .apply(&preview, "winning-owner", POLICY_EPOCH)
            .expect("winning apply");

        let profile_key = profile_key("profile:default");
        let mut stale_candidate = store
            .load_profile_by_key(&profile_key)
            .expect("load")
            .expect("profile state");
        stale_candidate
            .settings
            .get_mut("editor.tab_size")
            .expect("tab size")
            .value = ImportSettingValue::Integer(16);
        let stale_digest = settings_digest(&stale_candidate.settings).expect("settings digest");
        stale_candidate
            .history
            .last_mut()
            .expect("history")
            .result_settings_digest = stale_digest;
        assert_eq!(
            store.write_profile_by_key(&profile_key, &stale_candidate),
            Err(ImportExecutionError::ConcurrentMutation)
        );
        let durable = store
            .load_profile_by_key(&profile_key)
            .expect("reload")
            .expect("profile state");
        assert_eq!(
            durable.settings["editor.tab_size"].value,
            ImportSettingValue::Integer(2)
        );
        assert_eq!(durable.revision, 1);
    }

    #[test]
    fn apply_rejects_source_policy_and_target_drift() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let store = layout.store();
        let review = layout.review("profile:default");
        let source_preview = store
            .preview(&review, POLICY_EPOCH)
            .expect("source preview");
        fs::write(
            layout.source_root.join(".vscode/settings.json"),
            r#"{"editor.tabSize":4}"#,
        )
        .expect("source change");
        assert!(matches!(
            store.apply(&source_preview, "source-drift", POLICY_EPOCH),
            Err(ImportExecutionError::PreviewStale {
                reason_code: "source_state_changed"
            })
        ));

        let current_preview = store
            .preview(&review, POLICY_EPOCH)
            .expect("current preview");
        assert_eq!(
            store.apply(&current_preview, "policy-drift", "policy-epoch:test:2"),
            Err(ImportExecutionError::PolicyEpochChanged)
        );

        let stale_target_preview = store
            .preview(&review, POLICY_EPOCH)
            .expect("target preview");
        let winning_preview = store
            .preview(&review, POLICY_EPOCH)
            .expect("winning preview");
        store
            .apply(&winning_preview, "winning-apply", POLICY_EPOCH)
            .expect("winning apply");
        assert!(matches!(
            store.apply(&stale_target_preview, "stale-target", POLICY_EPOCH),
            Err(ImportExecutionError::PreviewStale {
                reason_code: "target_state_changed"
            })
        ));
    }

    #[test]
    fn rollback_restores_prior_settings_and_is_idempotent() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let store = layout.store();
        let review = layout.review("profile:default");
        let initial = store
            .preview(&review, POLICY_EPOCH)
            .expect("initial preview");
        store
            .apply(&initial, "initial-apply", POLICY_EPOCH)
            .expect("initial apply");

        fs::write(
            layout.source_root.join(".vscode/settings.json"),
            r#"{"editor.tabSize":4}"#,
        )
        .expect("updated source");
        let update = store
            .preview(&review, POLICY_EPOCH)
            .expect("update preview");
        assert_eq!(
            setting_row(&update, "editor.tab_size").before_value,
            Some(ImportSettingValue::Integer(2))
        );
        let applied = store
            .apply(&update, "update-apply", POLICY_EPOCH)
            .expect("update apply");
        let checkpoint_ref = applied.checkpoint_ref.as_deref().expect("checkpoint");

        let request = ImportRollbackRequest {
            checkpoint_ref,
            idempotency_token: "rollback-update",
        };
        let first = store.rollback(request).expect("rollback");
        let second = store.rollback(request).expect("idempotent rollback");
        assert!(first.restored_now);
        assert!(!second.restored_now);
        let state = store
            .load_profile_for_target("profile:default")
            .expect("load")
            .expect("state");
        assert_eq!(
            state.settings["editor.tab_size"].value,
            ImportSettingValue::Integer(2)
        );
        assert_eq!(state.history.len(), 3);
        assert_eq!(
            state.history.last().expect("history").action,
            ImportedProfileHistoryAction::RolledBack
        );
    }

    #[test]
    fn rollback_refuses_to_remove_a_newer_import() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let store = layout.store();
        let review = layout.review("profile:default");
        let first = store.preview(&review, POLICY_EPOCH).expect("first preview");
        let first_outcome = store
            .apply(&first, "first-apply", POLICY_EPOCH)
            .expect("first apply");
        fs::write(
            layout.source_root.join(".vscode/settings.json"),
            r#"{"editor.tabSize":4}"#,
        )
        .expect("new source");
        let second = store
            .preview(&review, POLICY_EPOCH)
            .expect("second preview");
        store
            .apply(&second, "second-apply", POLICY_EPOCH)
            .expect("second apply");

        assert_eq!(
            store.rollback(ImportRollbackRequest {
                checkpoint_ref: first_outcome.checkpoint_ref.as_deref().expect("checkpoint"),
                idempotency_token: "rollback-old",
            }),
            Err(ImportExecutionError::RollbackConflict)
        );
    }

    #[test]
    fn different_source_owner_requires_manual_review_and_token_reuse_conflicts() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let store = layout.store();
        let vscode_review = layout.review("profile:default");
        let first = store
            .preview(&vscode_review, POLICY_EPOCH)
            .expect("vscode preview");
        store
            .apply(&first, "owned-token", POLICY_EPOCH)
            .expect("vscode apply");

        fs::write(
            layout.source_root.join(".vscode/settings.json"),
            r#"{"editor.tabSize":4}"#,
        )
        .expect("updated source");
        let vscode_update = store
            .preview(&vscode_review, POLICY_EPOCH)
            .expect("vscode update preview");
        assert_eq!(
            store.apply(&vscode_update, "owned-token", POLICY_EPOCH),
            Err(ImportExecutionError::IdempotencyConflict)
        );

        fs::remove_dir_all(layout.source_root.join(".vscode")).expect("remove vscode marker");
        fs::create_dir_all(layout.source_root.join(".idea/codeStyles")).expect("idea code styles");
        fs::write(
            layout.source_root.join(".idea/codeStyles/Project.xml"),
            r#"<component><option name="TAB_SIZE" value="4" /></component>"#,
        )
        .expect("jetbrains setting");
        let jetbrains_review = layout.review("profile:default");
        let jetbrains = store
            .preview(&jetbrains_review, POLICY_EPOCH)
            .expect("jetbrains preview");
        assert_eq!(
            setting_row(&jetbrains, "editor.tab_size").decision,
            ImportExecutionRowDecision::ManualReview
        );
        assert_eq!(
            jetbrains.apply_gate,
            ImportPreviewApplyGate::BlockedNoSupportedSettings
        );
        assert_eq!(
            store.apply(&jetbrains, "jetbrains-apply", POLICY_EPOCH),
            Err(ImportExecutionError::ApplyNotAllowed)
        );
        assert_eq!(
            store
                .load_profile_for_target("profile:default")
                .expect("load")
                .expect("state")
                .settings["editor.tab_size"]
                .value,
            ImportSettingValue::Integer(2)
        );
    }

    #[test]
    fn malformed_duplicate_and_oversized_json_fail_closed() {
        let duplicate = TestLayout::vscode(r#"{"editor.tabSize":2,"editor.tabSize":4}"#);
        assert!(matches!(
            duplicate
                .store()
                .preview(&duplicate.review("profile:default"), POLICY_EPOCH),
            Err(ImportExecutionError::MalformedInput {
                reason_code: "strict_json_required",
                ..
            })
        ));

        let malformed = TestLayout::vscode(r#"{"editor.tabSize":2,}"#);
        assert!(matches!(
            malformed
                .store()
                .preview(&malformed.review("profile:default"), POLICY_EPOCH),
            Err(ImportExecutionError::MalformedInput { .. })
        ));

        let oversized = TestLayout::vscode("{}");
        fs::write(
            oversized.source_root.join(".vscode/settings.json"),
            vec![b' '; MAX_SOURCE_FILE_BYTES as usize + 1],
        )
        .expect("oversized fixture");
        assert!(matches!(
            oversized
                .store()
                .preview(&oversized.review("profile:default"), POLICY_EPOCH),
            Err(ImportExecutionError::InputTooLarge { .. })
        ));
    }

    #[test]
    fn jetbrains_adapter_parses_only_whitelisted_options() {
        let layout = TestLayout::jetbrains(
            r#"<?xml version="1.0" encoding="UTF-8"?>
               <component name="ProjectCodeStyleConfiguration">
                 <code_scheme name="Project" version="173">
                   <option name="TAB_SIZE" value="4" />
                   <option name="USE_TAB_CHARACTER" value="false" />
                   <option name="RIGHT_MARGIN" value="100" />
                   <option name="UNKNOWN_COSMETIC_OPTION" value="ignored" />
                   <option name="REMOTE_AUTH_TOKEN" value="never-persist" />
                 </code_scheme>
               </component>"#,
        );
        fs::write(
            layout.source_root.join(".idea/workspace.xml"),
            "<not-even-well-formed and-private",
        )
        .expect("opaque excluded workspace state");
        let preview = layout
            .store()
            .preview(&layout.review("profile:jetbrains"), POLICY_EPOCH)
            .expect("jetbrains preview");
        assert_eq!(preview.admitted_mutation_count(), 1);
        assert_eq!(
            setting_row(&preview, "editor.tab_size").after_value,
            Some(ImportSettingValue::Integer(4))
        );
        assert!(preview.rows.iter().any(|row| {
            row.source_setting_ref
                .ends_with("#option:USE_TAB_CHARACTER")
                && row.decision == ImportExecutionRowDecision::Unsupported
        }));
        assert!(preview.rows.iter().any(|row| {
            row.source_setting_ref == ".idea/workspace.xml"
                && row.decision == ImportExecutionRowDecision::BlockedAuthority
        }));
        let exported = serde_json::to_string(&preview).expect("serialize");
        assert!(!exported.contains("never-persist"));
        assert!(!exported.contains("REMOTE_AUTH_TOKEN"));
    }

    #[test]
    fn jetbrains_dtd_malformed_and_ambiguous_options_fail_closed() {
        let dtd = TestLayout::jetbrains(
            r#"<!DOCTYPE project [<!ENTITY xxe SYSTEM "file:///etc/passwd">]>
               <project><option name="TAB_SIZE" value="4" /></project>"#,
        );
        assert!(matches!(
            dtd.store()
                .preview(&dtd.review("profile:default"), POLICY_EPOCH),
            Err(ImportExecutionError::MalformedInput {
                reason_code: "xml_declaration_forbidden",
                ..
            })
        ));

        let ambiguous = TestLayout::jetbrains(
            r#"<component>
                 <option name="TAB_SIZE" value="2" />
                 <option name="TAB_SIZE" value="4" />
               </component>"#,
        );
        assert!(matches!(
            ambiguous
                .store()
                .preview(&ambiguous.review("profile:default"), POLICY_EPOCH),
            Err(ImportExecutionError::MalformedInput {
                reason_code: "xml_option_ambiguous",
                ..
            })
        ));
    }

    #[test]
    fn excluded_only_source_has_no_apply_authority() {
        let layout = TestLayout::vscode("{}");
        fs::write(
            layout.source_root.join(".vscode/launch.json"),
            r#"{"configurations":[]}"#,
        )
        .expect("launch fixture");
        let store = layout.store();
        let preview = store
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("preview");
        assert_eq!(
            preview.apply_gate,
            ImportPreviewApplyGate::BlockedNoSupportedSettings
        );
        assert_eq!(
            store.apply(&preview, "blocked-apply", POLICY_EPOCH),
            Err(ImportExecutionError::ApplyNotAllowed)
        );
        assert!(!layout.state_root.exists());
    }

    #[test]
    fn destination_labels_are_never_used_as_store_paths() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let target = "../../../../outside/profile";
        let store = layout.store();
        let preview = store
            .preview(&layout.review(target), POLICY_EPOCH)
            .expect("preview");
        store
            .apply(&preview, "path-safe-apply", POLICY_EPOCH)
            .expect("apply");
        let key = profile_key(target);
        assert!(layout
            .state_root
            .join("imported_profiles")
            .join(key)
            .join("revisions")
            .join("00000000000000000001.json")
            .is_file());
        assert!(!layout._temp.path().join("outside").exists());
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_source_root_marker_and_settings_are_rejected() {
        use std::os::unix::fs::symlink;

        let root_link = tempfile::tempdir().expect("tempdir");
        let real = root_link.path().join("real");
        fs::create_dir_all(real.join(".vscode")).expect("real marker");
        fs::write(real.join(".vscode/settings.json"), "{}").expect("real settings");
        let linked = root_link.path().join("linked");
        symlink(&real, &linked).expect("root symlink");
        let linked_review = CompetitorConfigClassifier::new().build_review(&linked, "profile:x");
        assert!(matches!(
            ImportedProfileStore::new(root_link.path().join("state"))
                .preview(&linked_review, POLICY_EPOCH),
            Err(ImportExecutionError::UnsafeSourceLayout { .. })
        ));

        let settings_link = TestLayout::vscode("{}");
        let outside = settings_link._temp.path().join("outside.json");
        fs::write(&outside, r#"{"editor.tabSize":8}"#).expect("outside settings");
        fs::remove_file(settings_link.source_root.join(".vscode/settings.json"))
            .expect("remove fixture settings");
        symlink(
            &outside,
            settings_link.source_root.join(".vscode/settings.json"),
        )
        .expect("settings symlink");
        assert!(matches!(
            settings_link
                .store()
                .preview(&settings_link.review("profile:default"), POLICY_EPOCH),
            Err(ImportExecutionError::UnsafeSourceLayout { .. })
        ));
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_durable_state_root_is_rejected_before_write() {
        use std::os::unix::fs::symlink;

        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let real_state = layout._temp.path().join("real-state");
        fs::create_dir(&real_state).expect("real state");
        symlink(&real_state, &layout.state_root).expect("state root symlink");
        let store = layout.store();
        let preview = store
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("preview remains read only");
        assert!(matches!(
            store.apply(&preview, "unsafe-state-root", POLICY_EPOCH),
            Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_directory_type_unsafe"
            })
        ));
        assert!(fs::read_dir(real_state)
            .expect("read real state")
            .next()
            .is_none());
    }

    #[test]
    fn live_timestamp_formatter_uses_real_utc_calendar_shape() {
        assert_eq!(utc_timestamp_from_seconds(0), "1970-01-01T00:00:00Z");
        assert_eq!(utc_timestamp_from_seconds(86_400), "1970-01-02T00:00:00Z");
        let live = utc_timestamp_now();
        assert_eq!(live.len(), 20);
        assert!(live.ends_with('Z'));
        assert_ne!(live, "2026-05-13T00:00:00Z");
    }
}
