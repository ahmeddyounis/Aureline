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

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs::{File, Metadata, OpenOptions};
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

/// Private protected-body kind referenced by digest-bound local metadata.
const IMPORT_EXECUTION_CHECKPOINT_BODY_RECORD_KIND: &str =
    "first_run_import_execution_checkpoint_body_record";

/// Private digest-bound metadata for the protected checkpoint body.
///
/// This adapter does not mint the public
/// `first_run_import_rollback_checkpoint_record`: it does not own a durable
/// migration-session, migration-restore, comparison, or compatibility-report
/// record and therefore cannot truthfully reference those artifacts. A higher
/// orchestration layer may project the public record only when those companion
/// records actually exist.
const IMPORT_EXECUTION_CHECKPOINT_METADATA_RECORD_KIND: &str =
    "first_run_import_execution_checkpoint_metadata_record";

const MAX_SOURCE_FILE_BYTES: u64 = 64 * 1024;
const MAX_DURABLE_FILE_BYTES: u64 = 1024 * 1024;
const MAX_HISTORY_ROWS: usize = 1024;
const MAX_IMPORT_PREVIEW_ROWS: usize = 4096;
const MAX_IMPORTED_PROFILE_SETTINGS: usize = 4096;
const MAX_IMPORTED_PROFILE_STATE_ENTRIES: usize = 4096;
const ABSENT_STATE_DIGEST: &str = "state:absent";

/// A schema-safe setting value admitted into imported-profile state.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "value_kind", content = "value", rename_all = "snake_case")]
pub enum ImportSettingValue {
    /// Boolean preference.
    Boolean(bool),
    /// Bounded integer preference.
    Integer(i64),
    /// Bounded, non-control text preference.
    Text(String),
}

impl fmt::Debug for ImportSettingValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Imported settings are user-authored content. Keep `Debug` useful for
        // shape assertions without making log/debug formatting an accidental
        // value-export path.
        formatter.write_str(match self {
            Self::Boolean(_) => "Boolean([redacted])",
            Self::Integer(_) => "Integer([redacted])",
            Self::Text(_) => "Text([redacted])",
        })
    }
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
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
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

impl fmt::Debug for ExecutableImportPreview {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExecutableImportPreview")
            .field("record_kind", &self.record_kind)
            .field("schema_version", &self.schema_version)
            .field("preview_ref", &self.preview_ref)
            .field("import_review_ref", &self.import_review_ref)
            .field("source_root_ref", &self.source_root_ref)
            .field("source_classification", &self.source_classification)
            .field("target_profile_ref", &self.target_profile_ref)
            .field("row_count", &self.rows.len())
            .field("admitted_mutation_count", &self.admitted_mutation_count())
            .field("blocked_authority_count", &self.blocked_authority_count())
            .field("apply_gate", &self.apply_gate)
            .field("source_snapshot_digest", &self.source_snapshot_digest)
            .field("target_state_digest", &self.target_state_digest)
            .field("plan_digest", &self.plan_digest)
            .field("generated_at", &self.generated_at)
            .field("has_live_authority", &self.live_authority.is_some())
            .finish()
    }
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
#[derive(Clone, Copy)]
pub struct ImportRollbackRequest<'a> {
    /// Checkpoint ref returned by apply.
    pub checkpoint_ref: &'a str,
    /// Idempotency token for this rollback request.
    pub idempotency_token: &'a str,
}

impl fmt::Debug for ImportRollbackRequest<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let checkpoint_ref = if parse_checkpoint_ref(self.checkpoint_ref).is_ok() {
            self.checkpoint_ref
        } else {
            "[redacted invalid checkpoint ref]"
        };
        formatter
            .debug_struct("ImportRollbackRequest")
            .field("checkpoint_ref", &checkpoint_ref)
            .field("idempotency_token", &"[redacted]")
            .finish()
    }
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
///
/// User-effective imported-profile revisions and recovery checkpoints
/// intentionally use different caller-owned roots. Production callers bind
/// `imported_profile_state_root` to `$AURELINE_CONFIG/profiles/imported` and
/// `checkpoint_history_root` below `$AURELINE_STATE/history`; combining them
/// would misclassify durable user truth as disposable/local application state.
///
/// Imported profile state is a local-only activation/history record. It is not
/// a `*.aureprofile.json` portable artifact; export must project through the
/// separately governed `portable_profile_artifact_record` flow.
#[derive(Clone)]
pub struct ImportedProfileStore {
    imported_profile_state_root: PathBuf,
    checkpoint_history_root: PathBuf,
    requires_distinct_roots: bool,
}

impl fmt::Debug for ImportedProfileStore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImportedProfileStore")
            .field("imported_profile_state_root", &"[redacted path]")
            .field("checkpoint_history_root", &"[redacted path]")
            .field(
                "uses_distinct_roots",
                &(self.imported_profile_state_root != self.checkpoint_history_root),
            )
            .field("requires_distinct_roots", &self.requires_distinct_roots)
            .finish()
    }
}

impl ImportedProfileStore {
    /// Creates a same-root harness used only by focused unit tests. Production
    /// construction cannot bypass the documented config/state separation.
    #[cfg(test)]
    fn new(root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        Self {
            imported_profile_state_root: root.clone(),
            checkpoint_history_root: root,
            requires_distinct_roots: false,
        }
    }

    /// Creates a store with distinct imported-state and checkpoint roots.
    pub fn with_roots(
        imported_profile_state_root: impl Into<PathBuf>,
        checkpoint_history_root: impl Into<PathBuf>,
    ) -> Self {
        Self {
            imported_profile_state_root: imported_profile_state_root.into(),
            checkpoint_history_root: checkpoint_history_root.into(),
            requires_distinct_roots: true,
        }
    }

    /// Builds an exact body-derived preview without writing durable state.
    pub fn preview(
        &self,
        review: &ImportReviewRecord,
        policy_epoch: &str,
    ) -> Result<ExecutableImportPreview, ImportExecutionError> {
        self.validate_root_shapes()?;
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
        self.validate_root_shapes()?;
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
                    self.confirm_profile_revision_durable(&authority.profile_key, state)?;
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
        let checkpoint = ImportExecutionCheckpointBody {
            record_kind: IMPORT_EXECUTION_CHECKPOINT_BODY_RECORD_KIND.to_owned(),
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
                            self.confirm_profile_revision_durable(&authority.profile_key, &winner)?;
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
        self.validate_root_shapes()?;
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
                self.confirm_profile_revision_durable(&checkpoint.profile_key, &current)?;
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
                            self.confirm_profile_revision_durable(
                                &checkpoint.profile_key,
                                &winner,
                            )?;
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
        self.validate_root_shapes()?;
        validate_request_label(destination_workspace_target, "destination_target_invalid")?;
        self.load_profile_by_key(&profile_key(destination_workspace_target))
    }

    fn validate_root_shapes(&self) -> Result<(), ImportExecutionError> {
        for root in [
            &self.imported_profile_state_root,
            &self.checkpoint_history_root,
        ] {
            if !root.is_absolute()
                || !root
                    .components()
                    .any(|component| matches!(component, Component::Normal(_)))
                || root
                    .components()
                    .any(|component| matches!(component, Component::CurDir | Component::ParentDir))
            {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "durable_store_root_invalid",
                });
            }
        }
        if self.requires_distinct_roots
            && (self.imported_profile_state_root == self.checkpoint_history_root
                || self
                    .imported_profile_state_root
                    .starts_with(&self.checkpoint_history_root)
                || self
                    .checkpoint_history_root
                    .starts_with(&self.imported_profile_state_root))
        {
            return Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_store_roots_overlap",
            });
        }
        Ok(())
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
        let metadata = match std::fs::symlink_metadata(&self.imported_profile_state_root) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_) => {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "profile_revision_directory_metadata_failed",
                })
            }
        };
        if metadata_is_path_redirect(&metadata) || !metadata.is_dir() {
            return Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "profile_revision_directory_type_unsafe",
            });
        }
        ensure_existing_child_chain_has_no_symlink(
            &self.imported_profile_state_root,
            &self.imported_profile_state_root.join("entry"),
        )?;

        let entries = std::fs::read_dir(&self.imported_profile_state_root).map_err(|_| {
            ImportExecutionError::DurableStateUnavailable {
                reason_code: "profile_revision_directory_read_failed",
            }
        })?;
        let filename_prefix = format!("imported-{profile_key}-");
        let mut committed_count = 0usize;
        let mut inspected_count = 0usize;
        let mut latest: Option<(u64, PathBuf)> = None;
        for entry in entries {
            inspected_count += 1;
            if inspected_count > MAX_IMPORTED_PROFILE_STATE_ENTRIES {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "profile_revision_capacity_exceeded",
                });
            }
            let entry = entry.map_err(|_| ImportExecutionError::DurableStateUnavailable {
                reason_code: "profile_revision_entry_read_failed",
            })?;
            let Some(filename) = entry.file_name().to_str().map(str::to_owned) else {
                continue;
            };
            if filename.starts_with('.') && filename.contains(".tmp.") {
                continue;
            }
            let Some(revision_text) = filename
                .strip_prefix(&filename_prefix)
                .and_then(|value| value.strip_suffix(".profile-state.json"))
            else {
                // Bounded legacy residue and future explicitly unrelated rows
                // are not inputs to the imported-profile resolver layer.
                continue;
            };
            let entry_path = entry.path();
            let entry_metadata = std::fs::symlink_metadata(&entry_path).map_err(|_| {
                ImportExecutionError::DurableStateUnavailable {
                    reason_code: "profile_revision_entry_metadata_failed",
                }
            })?;
            if metadata_is_path_redirect(&entry_metadata) || !entry_metadata.is_file() {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "profile_revision_entry_type_unsafe",
                });
            }
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
        let bytes = read_optional_durable_file(&self.imported_profile_state_root, &path)?.ok_or(
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
        match write_new_json(&self.imported_profile_state_root, &path, state) {
            Ok(WriteNewJsonOutcome::Durable) => Ok(()),
            Ok(WriteNewJsonOutcome::CommitStateUncertain) => Err(durable_commit_state_uncertain()),
            Err(error) => match std::fs::symlink_metadata(&path) {
                Ok(metadata) if metadata.is_file() && !metadata_is_path_redirect(&metadata) => {
                    Err(ImportExecutionError::ConcurrentMutation)
                }
                _ => Err(error),
            },
        }
    }

    fn write_checkpoint(
        &self,
        checkpoint: &ImportExecutionCheckpointBody,
    ) -> Result<(), ImportExecutionError> {
        validate_checkpoint(checkpoint)?;
        let body_path =
            self.checkpoint_body_path(&checkpoint.profile_key, &checkpoint.checkpoint_ref)?;
        let canonical_checkpoint = if let Some(bytes) =
            read_optional_durable_file(&self.checkpoint_history_root, &body_path)?
        {
            let existing: ImportExecutionCheckpointBody =
                serde_json::from_slice(&bytes).map_err(|_| {
                    ImportExecutionError::DurableStateUnavailable {
                        reason_code: "checkpoint_body_malformed",
                    }
                })?;
            validate_checkpoint(&existing)?;
            if checkpoints_equivalent(&existing, checkpoint) {
                // A retry may arrive after the private body committed but
                // before its metadata projection did. The body owns the one
                // canonical checkpoint timestamp; never regenerate metadata
                // from the retry's later wall-clock capture.
                self.confirm_checkpoint_body_durable(&body_path, &existing)?;
                existing
            } else {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "checkpoint_identity_collision",
                });
            }
        } else {
            match write_new_json(&self.checkpoint_history_root, &body_path, checkpoint) {
                Ok(WriteNewJsonOutcome::Durable) => checkpoint.clone(),
                Ok(WriteNewJsonOutcome::CommitStateUncertain) => {
                    return Err(durable_commit_state_uncertain())
                }
                Err(error) => {
                    let Some(bytes) =
                        read_optional_durable_file(&self.checkpoint_history_root, &body_path)?
                    else {
                        return Err(error);
                    };
                    let existing: ImportExecutionCheckpointBody = serde_json::from_slice(&bytes)
                        .map_err(|_| ImportExecutionError::DurableStateUnavailable {
                            reason_code: "checkpoint_body_malformed",
                        })?;
                    validate_checkpoint(&existing)?;
                    if checkpoints_equivalent(&existing, checkpoint) {
                        self.confirm_checkpoint_body_durable(&body_path, &existing)?;
                        existing
                    } else {
                        return Err(error);
                    }
                }
            }
        };

        let checkpoint_metadata = checkpoint_metadata_record(&canonical_checkpoint)?;
        let metadata_path =
            self.checkpoint_path(&checkpoint.profile_key, &checkpoint.checkpoint_ref)?;
        if let Some(bytes) =
            read_optional_durable_file(&self.checkpoint_history_root, &metadata_path)?
        {
            let existing: ImportExecutionCheckpointMetadata = serde_json::from_slice(&bytes)
                .map_err(|_| ImportExecutionError::DurableStateUnavailable {
                    reason_code: "checkpoint_metadata_malformed",
                })?;
            if existing != checkpoint_metadata {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "checkpoint_identity_collision",
                });
            }
            return self.confirm_checkpoint_metadata_durable(&metadata_path, &checkpoint_metadata);
        }
        match write_new_json(
            &self.checkpoint_history_root,
            &metadata_path,
            &checkpoint_metadata,
        ) {
            Ok(WriteNewJsonOutcome::Durable) => Ok(()),
            Ok(WriteNewJsonOutcome::CommitStateUncertain) => Err(durable_commit_state_uncertain()),
            Err(error) => {
                let Some(bytes) =
                    read_optional_durable_file(&self.checkpoint_history_root, &metadata_path)?
                else {
                    return Err(error);
                };
                let existing: ImportExecutionCheckpointMetadata = serde_json::from_slice(&bytes)
                    .map_err(|_| ImportExecutionError::DurableStateUnavailable {
                        reason_code: "checkpoint_metadata_malformed",
                    })?;
                if existing == checkpoint_metadata {
                    self.confirm_checkpoint_metadata_durable(&metadata_path, &checkpoint_metadata)
                } else {
                    Err(error)
                }
            }
        }
    }

    fn confirm_profile_revision_durable(
        &self,
        profile_key: &str,
        expected: &ImportedProfileState,
    ) -> Result<(), ImportExecutionError> {
        sync_directory_revalidating_identity(&self.imported_profile_state_root)?;
        match self.load_profile_by_key(profile_key)? {
            Some(observed) if observed == *expected => Ok(()),
            _ => Err(ImportExecutionError::ConcurrentMutation),
        }
    }

    fn confirm_checkpoint_body_durable(
        &self,
        path: &Path,
        expected: &ImportExecutionCheckpointBody,
    ) -> Result<(), ImportExecutionError> {
        let parent = path
            .parent()
            .ok_or(ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_parent_missing",
            })?;
        sync_directory_revalidating_identity(parent)?;
        let bytes = read_optional_durable_file(&self.checkpoint_history_root, path)?.ok_or(
            ImportExecutionError::DurableStateUnavailable {
                reason_code: "checkpoint_disappeared_after_sync",
            },
        )?;
        let observed: ImportExecutionCheckpointBody =
            serde_json::from_slice(&bytes).map_err(|_| {
                ImportExecutionError::DurableStateUnavailable {
                    reason_code: "checkpoint_body_malformed",
                }
            })?;
        validate_checkpoint(&observed)?;
        if checkpoints_equivalent(&observed, expected) {
            Ok(())
        } else {
            Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "checkpoint_identity_collision",
            })
        }
    }

    fn confirm_checkpoint_metadata_durable(
        &self,
        path: &Path,
        expected: &ImportExecutionCheckpointMetadata,
    ) -> Result<(), ImportExecutionError> {
        let parent = path
            .parent()
            .ok_or(ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_parent_missing",
            })?;
        sync_directory_revalidating_identity(parent)?;
        let bytes = read_optional_durable_file(&self.checkpoint_history_root, path)?.ok_or(
            ImportExecutionError::DurableStateUnavailable {
                reason_code: "checkpoint_disappeared_after_sync",
            },
        )?;
        let observed: ImportExecutionCheckpointMetadata =
            serde_json::from_slice(&bytes).map_err(|_| {
                ImportExecutionError::DurableStateUnavailable {
                    reason_code: "checkpoint_metadata_malformed",
                }
            })?;
        if observed == *expected {
            Ok(())
        } else {
            Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "checkpoint_identity_collision",
            })
        }
    }

    fn read_checkpoint(
        &self,
        checkpoint_ref: &str,
    ) -> Result<ImportExecutionCheckpointBody, ImportExecutionError> {
        let (profile_key, _) = parse_checkpoint_ref(checkpoint_ref)?;
        let metadata_path = self.checkpoint_path(profile_key, checkpoint_ref)?;
        let metadata_bytes =
            read_optional_durable_file(&self.checkpoint_history_root, &metadata_path)?
                .ok_or(ImportExecutionError::CheckpointUnavailable)?;
        let metadata: ImportExecutionCheckpointMetadata =
            serde_json::from_slice(&metadata_bytes)
                .map_err(|_| ImportExecutionError::CheckpointUnavailable)?;
        let body_path = self.checkpoint_body_path(profile_key, checkpoint_ref)?;
        let body_bytes = read_optional_durable_file(&self.checkpoint_history_root, &body_path)?
            .ok_or(ImportExecutionError::CheckpointUnavailable)?;
        let checkpoint: ImportExecutionCheckpointBody = serde_json::from_slice(&body_bytes)
            .map_err(|_| ImportExecutionError::CheckpointUnavailable)?;
        validate_checkpoint(&checkpoint)?;
        if checkpoint.checkpoint_ref != checkpoint_ref
            || metadata != checkpoint_metadata_record(&checkpoint)?
        {
            return Err(ImportExecutionError::CheckpointUnavailable);
        }
        Ok(checkpoint)
    }

    fn profile_revision_path(&self, profile_key: &str, revision: u64) -> PathBuf {
        self.imported_profile_state_root.join(format!(
            "imported-{profile_key}-{revision:020}.profile-state.json"
        ))
    }

    fn checkpoint_path(
        &self,
        profile_key: &str,
        checkpoint_ref: &str,
    ) -> Result<PathBuf, ImportExecutionError> {
        validate_profile_key(profile_key)?;
        let (_, checkpoint_key) = parse_checkpoint_ref(checkpoint_ref)?;
        Ok(self
            .checkpoint_history_root
            .join("import_checkpoints")
            .join(profile_key)
            .join(format!("{checkpoint_key}.json")))
    }

    fn checkpoint_body_path(
        &self,
        profile_key: &str,
        checkpoint_ref: &str,
    ) -> Result<PathBuf, ImportExecutionError> {
        validate_profile_key(profile_key)?;
        let (_, checkpoint_key) = parse_checkpoint_ref(checkpoint_ref)?;
        Ok(self
            .checkpoint_history_root
            .join("import_checkpoint_bodies")
            .join(profile_key)
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
struct ImportExecutionCheckpointBody {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ImportExecutionCheckpointMetadata {
    schema_version: u32,
    record_kind: String,
    checkpoint_ref: String,
    protected_body_ref: String,
    protected_body_digest: String,
    target_profile_ref: String,
    plan_digest: String,
    source_snapshot_digest: String,
    prior_state_digest: String,
    expected_applied_settings_digest: String,
    idempotency_token_digest: String,
    created_at: String,
}

fn secure_source_root(source_root: &Path) -> Result<SecuredSourceRoot, ImportExecutionError> {
    let metadata = std::fs::symlink_metadata(source_root).map_err(|_| {
        ImportExecutionError::SourceUnavailable {
            source_item_ref: "source_root".to_owned(),
        }
    })?;
    if metadata_is_path_redirect(&metadata) || !metadata.is_dir() {
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
    if metadata_is_path_redirect(&metadata) || !metadata.is_dir() {
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
    if candidates.len().saturating_add(excluded_rows.len()) > MAX_IMPORT_PREVIEW_ROWS {
        return Err(ImportExecutionError::InputTooLarge {
            source_item_ref: "source_settings".to_owned(),
        });
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
                None => {
                    excluded_rows.push(excluded_source_setting(relative, &format!("option:{name}")))
                }
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
    if is_authority_or_sensitive_key(source_key) || !source_key_is_support_safe(source_key) {
        format!(
            "{source_file}#redacted-key:{}",
            digest_suffix(&aureline_history::body_object_id(source_key.as_bytes()))
        )
    } else {
        format!("{source_file}#{source_key}")
    }
}

fn source_key_is_support_safe(source_key: &str) -> bool {
    !source_key.is_empty()
        && source_key.len() <= 160
        && source_key
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

fn source_fragment_is_valid(source_key: &str) -> bool {
    source_key_is_support_safe(source_key)
        || source_key
            .strip_prefix("option:")
            .is_some_and(source_key_is_support_safe)
        || source_key
            .strip_prefix("redacted-key:")
            .is_some_and(|digest| valid_lower_hex(digest, 64))
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
    if metadata_is_path_redirect(&metadata) {
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
    if metadata_is_path_redirect(&before) || !before.is_file() {
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
    if !opened.is_file()
        || DurableFileIdentity::from_metadata(&opened)
            != DurableFileIdentity::from_metadata(&before)
    {
        return Err(ImportExecutionError::PreviewStale {
            reason_code: "source_entry_changed_during_read",
        });
    }
    if opened.len() > MAX_SOURCE_FILE_BYTES {
        return Err(ImportExecutionError::InputTooLarge {
            source_item_ref: relative.to_owned(),
        });
    }
    let mut bytes = Vec::with_capacity(opened.len() as usize);
    Read::by_ref(&mut file)
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
    if metadata_is_path_redirect(&after)
        || !after.is_file()
        || after_canonical != canonical
        || DurableFileIdentity::from_metadata(&after) != DurableFileIdentity::from_metadata(&opened)
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
        if metadata_is_path_redirect(&metadata) {
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
        || review.source_path.is_empty()
        || review.source_path.len() > 4096
        || review.source_path.chars().any(char::is_control)
        || review.import_review_id
            != super::review_id_for(
                &review.source_path,
                &review.destination_workspace_target,
                review.classification,
            )
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
    let checkpoint_object_id = aureline_history::body_object_id(identity.as_bytes());
    let checkpoint_key = digest_suffix(&checkpoint_object_id);
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
        || state.settings.len() > MAX_IMPORTED_PROFILE_SETTINGS
        || state.history.len() > MAX_HISTORY_ROWS
        || state.revision != state.history.len() as u64
        || validate_profile_key(profile_key).is_err()
        || !valid_utc_timestamp(&state.updated_at)
    {
        return Err(ImportExecutionError::DurableStateUnavailable {
            reason_code: "profile_state_contract_invalid",
        });
    }
    for (setting_id, record) in &state.settings {
        if !valid_setting_id(setting_id)
            || !valid_source_setting_ref(&record.source_setting_ref)
            || !valid_utc_timestamp(&record.imported_at)
            || matches!(&record.value, ImportSettingValue::Text(value) if value.len() > 128 || value.chars().any(char::is_control))
        {
            return Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "profile_setting_contract_invalid",
            });
        }
    }
    let mut seen_idempotency_digests = BTreeSet::new();
    for (index, entry) in state.history.iter().enumerate() {
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
            || !valid_utc_timestamp(&entry.occurred_at)
            || entry
                .changed_setting_ids
                .iter()
                .any(|id| !valid_setting_id(id))
            || entry.changed_setting_ids.len() > MAX_IMPORTED_PROFILE_SETTINGS
            || entry
                .changed_setting_ids
                .windows(2)
                .any(|pair| pair[0] >= pair[1])
            || !seen_idempotency_digests.insert(entry.idempotency_token_digest.as_str())
        {
            return Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "profile_history_contract_invalid",
            });
        }
        if entry.action == ImportedProfileHistoryAction::RolledBack {
            let previous_apply = index
                .checked_sub(1)
                .and_then(|previous| state.history.get(previous));
            if match previous_apply {
                Some(previous) => {
                    previous.action != ImportedProfileHistoryAction::Applied
                        || entry.preview_ref != previous.preview_ref
                        || entry.import_review_ref != previous.import_review_ref
                        || entry.source_root_ref != previous.source_root_ref
                        || entry.source_snapshot_digest != previous.source_snapshot_digest
                        || entry.policy_epoch != previous.policy_epoch
                        || entry.plan_digest != previous.plan_digest
                        || entry.checkpoint_ref != previous.checkpoint_ref
                        || entry.changed_setting_ids != previous.changed_setting_ids
                }
                None => true,
            } {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "profile_history_rollback_lineage_invalid",
                });
            }
        }
    }
    if let Some(latest) = state.history.last() {
        if state.updated_at != latest.occurred_at
            || settings_digest(&state.settings)? != latest.result_settings_digest
        {
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

fn valid_source_setting_ref(value: &str) -> bool {
    if value.is_empty()
        || value.len() > 320
        || value.chars().any(char::is_control)
        || value.chars().any(char::is_whitespace)
        || value.starts_with('/')
        || value.starts_with('\\')
        || value.starts_with('~')
        || value.contains("://")
        || value.contains("../")
        || value.contains("..\\")
    {
        return false;
    }
    let (source_file, source_key) = value
        .split_once('#')
        .map_or((value, None), |(file, key)| (file, Some(key)));
    if source_file.is_empty()
        || source_file.contains('#')
        || !source_file
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
        || source_file
            .split('/')
            .any(|component| component.is_empty() || matches!(component, "." | ".."))
    {
        return false;
    }
    source_key
        .map(|key| !key.contains('#') && source_fragment_is_valid(key))
        .unwrap_or(true)
}

fn valid_utc_timestamp(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 20
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
    {
        return false;
    }
    let Some(year) = timestamp_number(bytes, 0, 4) else {
        return false;
    };
    let Some(month) = timestamp_number(bytes, 5, 7) else {
        return false;
    };
    let Some(day) = timestamp_number(bytes, 8, 10) else {
        return false;
    };
    let Some(hour) = timestamp_number(bytes, 11, 13) else {
        return false;
    };
    let Some(minute) = timestamp_number(bytes, 14, 16) else {
        return false;
    };
    let Some(second) = timestamp_number(bytes, 17, 19) else {
        return false;
    };
    let leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap_year => 29,
        2 => 28,
        _ => return false,
    };
    year >= 1970 && day >= 1 && day <= days_in_month && hour <= 23 && minute <= 59 && second <= 59
}

fn timestamp_number(bytes: &[u8], start: usize, end: usize) -> Option<u32> {
    bytes
        .get(start..end)?
        .iter()
        .try_fold(0_u32, |value, byte| {
            byte.is_ascii_digit()
                .then_some(value * 10 + u32::from(*byte - b'0'))
        })
}

fn checkpoint_metadata_record(
    checkpoint: &ImportExecutionCheckpointBody,
) -> Result<ImportExecutionCheckpointMetadata, ImportExecutionError> {
    let (profile_key, checkpoint_key) = parse_checkpoint_ref(&checkpoint.checkpoint_ref)?;
    if profile_key != checkpoint.profile_key {
        return Err(ImportExecutionError::CheckpointUnavailable);
    }

    Ok(ImportExecutionCheckpointMetadata {
        schema_version: IMPORT_EXECUTION_SCHEMA_VERSION,
        record_kind: IMPORT_EXECUTION_CHECKPOINT_METADATA_RECORD_KIND.to_owned(),
        checkpoint_ref: checkpoint.checkpoint_ref.clone(),
        protected_body_ref: format!("import-checkpoint-body:{profile_key}:{checkpoint_key}"),
        protected_body_digest: digest_serializable(checkpoint)?,
        target_profile_ref: checkpoint.target_profile_ref.clone(),
        plan_digest: checkpoint.plan_digest.clone(),
        source_snapshot_digest: checkpoint.source_snapshot_digest.clone(),
        prior_state_digest: checkpoint.prior_state_digest.clone(),
        expected_applied_settings_digest: checkpoint.expected_applied_settings_digest.clone(),
        idempotency_token_digest: checkpoint.idempotency_token_digest.clone(),
        created_at: checkpoint.created_at.clone(),
    })
}

fn validate_checkpoint(
    checkpoint: &ImportExecutionCheckpointBody,
) -> Result<(), ImportExecutionError> {
    let (profile_key, _) = parse_checkpoint_ref(&checkpoint.checkpoint_ref)?;
    if checkpoint.record_kind != IMPORT_EXECUTION_CHECKPOINT_BODY_RECORD_KIND
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
        || !valid_utc_timestamp(&checkpoint.created_at)
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
    left: &ImportExecutionCheckpointBody,
    right: &ImportExecutionCheckpointBody,
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct DurableFileIdentity {
    length: u64,
    modified: Option<SystemTime>,
    created: Option<SystemTime>,
    #[cfg(unix)]
    device: u64,
    #[cfg(unix)]
    inode: u64,
    #[cfg(unix)]
    change_time_seconds: i64,
    #[cfg(unix)]
    change_time_nanoseconds: i64,
}

impl DurableFileIdentity {
    fn from_metadata(metadata: &Metadata) -> Self {
        #[cfg(unix)]
        use std::os::unix::fs::MetadataExt;

        Self {
            length: metadata.len(),
            modified: metadata.modified().ok(),
            created: metadata.created().ok(),
            #[cfg(unix)]
            device: metadata.dev(),
            #[cfg(unix)]
            inode: metadata.ino(),
            #[cfg(unix)]
            change_time_seconds: metadata.ctime(),
            #[cfg(unix)]
            change_time_nanoseconds: metadata.ctime_nsec(),
        }
    }

    fn same_file_object(&self, other: &Self) -> bool {
        #[cfg(unix)]
        {
            self.device == other.device && self.inode == other.inode && self.length == other.length
        }
        #[cfg(not(unix))]
        {
            self.length == other.length
                && self.created == other.created
                && self.modified == other.modified
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DurableDirectoryIdentity {
    created: Option<SystemTime>,
    #[cfg(unix)]
    device: u64,
    #[cfg(unix)]
    inode: u64,
}

impl DurableDirectoryIdentity {
    fn from_metadata(metadata: &Metadata) -> Self {
        #[cfg(unix)]
        use std::os::unix::fs::MetadataExt;

        Self {
            created: metadata.created().ok(),
            #[cfg(unix)]
            device: metadata.dev(),
            #[cfg(unix)]
            inode: metadata.ino(),
        }
    }
}

fn direct_directory_identity(
    path: &Path,
) -> Result<DurableDirectoryIdentity, ImportExecutionError> {
    let metadata = std::fs::symlink_metadata(path).map_err(|_| {
        ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_directory_metadata_failed",
        }
    })?;
    if metadata_is_path_redirect(&metadata) || !metadata.is_dir() {
        return Err(ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_directory_type_unsafe",
        });
    }
    Ok(DurableDirectoryIdentity::from_metadata(&metadata))
}

fn durable_file_changed_during_read() -> ImportExecutionError {
    ImportExecutionError::DurableStateUnavailable {
        reason_code: "durable_file_changed_during_read",
    }
}

fn read_optional_durable_file(
    state_root: &Path,
    path: &Path,
) -> Result<Option<Vec<u8>>, ImportExecutionError> {
    read_optional_durable_file_with_post_read_hook(state_root, path, |_| {})
}

// Stable `std` APIs do not expose dirfd-relative open/install on every target.
// These checks pin and re-observe the named parent and file identities around
// each operation. They detect observed swaps (including same-length Unix
// replacements through dev+ino+ctime), while a swap-and-restore entirely
// inside a final name-operation window remains an explicit platform limit.
fn read_optional_durable_file_with_post_read_hook<F>(
    state_root: &Path,
    path: &Path,
    post_read_hook: F,
) -> Result<Option<Vec<u8>>, ImportExecutionError>
where
    F: FnOnce(&Path),
{
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
    if metadata_is_path_redirect(&metadata) || !metadata.is_file() {
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
    let parent = path
        .parent()
        .ok_or(ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_parent_missing",
        })?;
    let parent_identity = direct_directory_identity(parent)?;
    let before_identity = DurableFileIdentity::from_metadata(&metadata);
    let mut file = File::open(path).map_err(|_| ImportExecutionError::DurableStateUnavailable {
        reason_code: "durable_file_open_failed",
    })?;
    let opened = file
        .metadata()
        .map_err(|_| ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_file_metadata_failed",
        })?;
    if !opened.is_file() || DurableFileIdentity::from_metadata(&opened) != before_identity {
        return Err(durable_file_changed_during_read());
    }
    if opened.len() > MAX_DURABLE_FILE_BYTES {
        return Err(ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_file_too_large",
        });
    }
    let mut bytes = Vec::with_capacity(opened.len() as usize);
    Read::by_ref(&mut file)
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
    post_read_hook(path);
    ensure_existing_child_chain_has_no_symlink(state_root, path)
        .map_err(|_| durable_file_changed_during_read())?;
    let after = std::fs::symlink_metadata(path).map_err(|_| durable_file_changed_during_read())?;
    let parent_after =
        direct_directory_identity(parent).map_err(|_| durable_file_changed_during_read())?;
    if metadata_is_path_redirect(&after)
        || !after.is_file()
        || DurableFileIdentity::from_metadata(&after) != DurableFileIdentity::from_metadata(&opened)
        || parent_after != parent_identity
    {
        return Err(durable_file_changed_during_read());
    }
    Ok(Some(bytes))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WriteNewJsonOutcome {
    Durable,
    CommitStateUncertain,
}

fn durable_commit_state_uncertain() -> ImportExecutionError {
    ImportExecutionError::DurableStateUnavailable {
        reason_code: "durable_commit_state_uncertain",
    }
}

fn write_new_json<T: Serialize>(
    state_root: &Path,
    path: &Path,
    value: &T,
) -> Result<WriteNewJsonOutcome, ImportExecutionError> {
    write_new_json_with_preinstall_hook(state_root, path, value, || {})
}

/// Owns a staged file until its publication has been fully validated.
///
/// The drop path is deliberately privacy-biased before publication: an
/// ordinary pre-commit error scrubs the handle explicitly before pathname
/// cleanup, while an unwinding pre-install hook still truncates through the
/// already-open handle. Once the create-new hard link succeeds, the handle is
/// disarmed immediately because truncating it would also corrupt the installed
/// artifact. Post-commit failures are reported as an uncertain commit instead.
struct PendingSensitiveFile {
    file: Option<File>,
}

impl PendingSensitiveFile {
    fn new(file: File) -> Self {
        Self { file: Some(file) }
    }

    fn file_mut(&mut self) -> Result<&mut File, ImportExecutionError> {
        self.file
            .as_mut()
            .ok_or(ImportExecutionError::ConcurrentMutation)
    }

    fn scrub_and_close(&mut self) {
        if let Some(file) = self.file.take() {
            let _ = file.set_len(0);
            let _ = file.sync_all();
        }
    }

    fn close_without_scrub(&mut self) {
        drop(self.file.take());
    }
}

impl Drop for PendingSensitiveFile {
    fn drop(&mut self) {
        self.scrub_and_close();
    }
}

fn write_new_json_with_preinstall_hook<T, F>(
    state_root: &Path,
    path: &Path,
    value: &T,
    preinstall_hook: F,
) -> Result<WriteNewJsonOutcome, ImportExecutionError>
where
    T: Serialize,
    F: FnOnce(),
{
    write_new_json_with_hooks(state_root, path, value, preinstall_hook, || Ok(()))
}

fn write_new_json_with_hooks<T, F, G>(
    state_root: &Path,
    path: &Path,
    value: &T,
    preinstall_hook: F,
    postinstall_hook: G,
) -> Result<WriteNewJsonOutcome, ImportExecutionError>
where
    T: Serialize,
    F: FnOnce(),
    G: FnOnce() -> Result<(), ImportExecutionError>,
{
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
    let parent_identity = direct_directory_identity(&parent)?;
    reject_symlink_target(path)?;
    let (temporary_path, temporary) = create_temporary_file(&parent, path)?;
    let mut pending_temporary = PendingSensitiveFile::new(temporary);
    let precommit_result = (|| {
        verify_write_parent_identity(state_root, path, &parent, &parent_identity)?;
        let temporary = pending_temporary.file_mut()?;
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
        let temporary_identity =
            DurableFileIdentity::from_metadata(&temporary.metadata().map_err(|_| {
                ImportExecutionError::DurableStateUnavailable {
                    reason_code: "durable_temp_sync_failed",
                }
            })?);
        verify_direct_file_identity(&temporary_path, &temporary_identity)?;
        verify_write_parent_identity(state_root, path, &parent, &parent_identity)?;
        preinstall_hook();
        verify_write_parent_identity(state_root, path, &parent, &parent_identity)?;
        verify_direct_file_identity(&temporary_path, &temporary_identity)?;
        reject_symlink_target(path)?;
        verify_write_parent_identity(state_root, path, &parent, &parent_identity)?;
        verify_direct_file_identity(&temporary_path, &temporary_identity)?;
        std::fs::hard_link(&temporary_path, path).map_err(|_| {
            ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_create_new_failed",
            }
        })?;
        Ok(temporary_identity)
    })();
    let temporary_identity = match precommit_result {
        Ok(identity) => identity,
        Err(error) => {
            pending_temporary.scrub_and_close();
            if write_parent_identity_matches(state_root, path, &parent, &parent_identity) {
                let _ = std::fs::remove_file(&temporary_path);
            }
            return Err(error);
        }
    };

    // The destination and temporary pathname now name the same synchronized
    // file object. Disarm before any fallible validation or injected hook: a
    // scrub through this handle would truncate the committed destination too.
    pending_temporary.close_without_scrub();
    if postinstall_hook().is_err() {
        return Ok(WriteNewJsonOutcome::CommitStateUncertain);
    }

    let postcommit_result = (|| {
        verify_write_parent_identity(state_root, path, &parent, &parent_identity)?;
        verify_direct_file_object(path, &temporary_identity)?;
        std::fs::remove_file(&temporary_path).map_err(|_| {
            ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_temp_cleanup_failed",
            }
        })?;
        verify_write_parent_identity(state_root, path, &parent, &parent_identity)?;
        verify_direct_file_object(path, &temporary_identity)?;
        sync_directory(&parent)?;
        verify_write_parent_identity(state_root, path, &parent, &parent_identity)?;
        verify_direct_file_object(path, &temporary_identity)
    })();
    if postcommit_result.is_err() {
        return Ok(WriteNewJsonOutcome::CommitStateUncertain);
    }
    Ok(WriteNewJsonOutcome::Durable)
}

fn verify_write_parent_identity(
    state_root: &Path,
    path: &Path,
    parent: &Path,
    expected: &DurableDirectoryIdentity,
) -> Result<(), ImportExecutionError> {
    ensure_existing_child_chain_has_no_symlink(state_root, path)
        .map_err(|_| ImportExecutionError::ConcurrentMutation)?;
    if direct_directory_identity(parent).map_err(|_| ImportExecutionError::ConcurrentMutation)?
        != *expected
    {
        return Err(ImportExecutionError::ConcurrentMutation);
    }
    Ok(())
}

fn write_parent_identity_matches(
    state_root: &Path,
    path: &Path,
    parent: &Path,
    expected: &DurableDirectoryIdentity,
) -> bool {
    ensure_existing_child_chain_has_no_symlink(state_root, path).is_ok()
        && direct_directory_identity(parent).is_ok_and(|observed| observed == *expected)
}

fn verify_direct_file_identity(
    path: &Path,
    expected: &DurableFileIdentity,
) -> Result<(), ImportExecutionError> {
    let metadata =
        std::fs::symlink_metadata(path).map_err(|_| ImportExecutionError::ConcurrentMutation)?;
    if metadata_is_path_redirect(&metadata)
        || !metadata.is_file()
        || DurableFileIdentity::from_metadata(&metadata) != *expected
    {
        return Err(ImportExecutionError::ConcurrentMutation);
    }
    Ok(())
}

fn verify_direct_file_object(
    path: &Path,
    expected: &DurableFileIdentity,
) -> Result<(), ImportExecutionError> {
    let metadata =
        std::fs::symlink_metadata(path).map_err(|_| ImportExecutionError::ConcurrentMutation)?;
    let observed = DurableFileIdentity::from_metadata(&metadata);
    if metadata_is_path_redirect(&metadata)
        || !metadata.is_file()
        || !observed.same_file_object(expected)
    {
        return Err(ImportExecutionError::ConcurrentMutation);
    }
    Ok(())
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
    ensure_directory_chain_without_untrusted_redirects(path, true)
}

fn ensure_directory_chain_without_untrusted_redirects(
    path: &Path,
    create_missing: bool,
) -> Result<(), ImportExecutionError> {
    if path.as_os_str().is_empty() {
        return Err(ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_directory_create_failed",
        });
    }

    let mut directory = if path.is_relative() {
        PathBuf::from(".")
    } else {
        PathBuf::new()
    };
    // A relative path is already below the process working directory, so no
    // child redirect gets the platform-root exception.
    let mut normal_component_depth = usize::from(path.is_relative());
    for component in path.components() {
        match component {
            Component::Prefix(prefix) => directory.push(prefix.as_os_str()),
            Component::RootDir => directory.push(component.as_os_str()),
            Component::CurDir => continue,
            Component::ParentDir => {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "durable_directory_type_unsafe",
                })
            }
            Component::Normal(segment) => directory.push(segment),
        }
        if !matches!(component, Component::Normal(_)) {
            continue;
        }
        match std::fs::symlink_metadata(&directory) {
            Ok(metadata) => {
                if metadata_is_path_redirect(&metadata) {
                    if !allow_trusted_platform_root_alias(
                        &directory,
                        &metadata,
                        normal_component_depth,
                    ) {
                        return Err(ImportExecutionError::DurableStateUnavailable {
                            reason_code: "durable_directory_type_unsafe",
                        });
                    }
                    let followed = std::fs::metadata(&directory).map_err(|_| {
                        ImportExecutionError::DurableStateUnavailable {
                            reason_code: "durable_directory_metadata_failed",
                        }
                    })?;
                    if !followed.is_dir() {
                        return Err(ImportExecutionError::DurableStateUnavailable {
                            reason_code: "durable_directory_type_unsafe",
                        });
                    }
                } else if !metadata.is_dir() {
                    return Err(ImportExecutionError::DurableStateUnavailable {
                        reason_code: "durable_directory_type_unsafe",
                    });
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound && create_missing => {
                let parent =
                    directory
                        .parent()
                        .ok_or(ImportExecutionError::DurableStateUnavailable {
                            reason_code: "durable_directory_create_failed",
                        })?;
                let parent_identity = followed_directory_identity(parent)?;
                match create_private_directory(&directory) {
                    Ok(()) => {}
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                    Err(_) => {
                        return Err(ImportExecutionError::DurableStateUnavailable {
                            reason_code: "durable_directory_create_failed",
                        })
                    }
                }
                let metadata = std::fs::symlink_metadata(&directory).map_err(|_| {
                    ImportExecutionError::DurableStateUnavailable {
                        reason_code: "durable_directory_metadata_failed",
                    }
                })?;
                if metadata_is_path_redirect(&metadata) || !metadata.is_dir() {
                    return Err(ImportExecutionError::DurableStateUnavailable {
                        reason_code: "durable_directory_type_unsafe",
                    });
                }
                if followed_directory_identity(parent)? != parent_identity {
                    return Err(ImportExecutionError::ConcurrentMutation);
                }
                sync_directory_uninjected(parent)?;
                if followed_directory_identity(parent)? != parent_identity {
                    return Err(ImportExecutionError::ConcurrentMutation);
                }
                let metadata = std::fs::symlink_metadata(&directory).map_err(|_| {
                    ImportExecutionError::DurableStateUnavailable {
                        reason_code: "durable_directory_metadata_failed",
                    }
                })?;
                if metadata_is_path_redirect(&metadata) || !metadata.is_dir() {
                    return Err(ImportExecutionError::ConcurrentMutation);
                }
            }
            Err(_) => {
                return Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "durable_directory_metadata_failed",
                })
            }
        }
        normal_component_depth = normal_component_depth.saturating_add(1);
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn allow_trusted_platform_root_alias(
    path: &Path,
    metadata: &Metadata,
    normal_component_depth: usize,
) -> bool {
    use std::os::unix::fs::MetadataExt;

    // macOS spells the temporary hierarchy through `/var`, whose exact
    // platform-owned target is `/private/var`. No other alias is admitted.
    if path != Path::new("/var")
        || normal_component_depth != 0
        || !metadata.file_type().is_symlink()
        || metadata.uid() != 0
    {
        return false;
    }
    let Some(parent) = path.parent() else {
        return false;
    };
    let Ok(parent_metadata) = std::fs::symlink_metadata(parent) else {
        return false;
    };
    parent_metadata.is_dir()
        && parent_metadata.uid() == 0
        && parent_metadata.mode() & 0o022 == 0
        && std::fs::canonicalize(path).is_ok_and(|target| target == Path::new("/private/var"))
}

#[cfg(not(target_os = "macos"))]
fn allow_trusted_platform_root_alias(
    _path: &Path,
    _metadata: &Metadata,
    _normal_component_depth: usize,
) -> bool {
    false
}

fn metadata_is_path_redirect(metadata: &Metadata) -> bool {
    metadata.file_type().is_symlink() || metadata_is_platform_redirect(metadata)
}

fn followed_directory_identity(
    path: &Path,
) -> Result<DurableDirectoryIdentity, ImportExecutionError> {
    let metadata =
        std::fs::metadata(path).map_err(|_| ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_directory_metadata_failed",
        })?;
    if !metadata.is_dir() {
        return Err(ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_directory_type_unsafe",
        });
    }
    Ok(DurableDirectoryIdentity::from_metadata(&metadata))
}

#[cfg(unix)]
fn create_private_directory(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::DirBuilderExt;

    let mut builder = std::fs::DirBuilder::new();
    builder.mode(0o700);
    builder.create(path)
}

#[cfg(not(unix))]
fn create_private_directory(path: &Path) -> std::io::Result<()> {
    std::fs::DirBuilder::new().create(path)
}

#[cfg(windows)]
fn metadata_is_platform_redirect(metadata: &Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;

    windows_file_attributes_include_reparse_point(metadata.file_attributes())
}

#[cfg(windows)]
fn windows_file_attributes_include_reparse_point(attributes: u32) -> bool {
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
    attributes & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

#[cfg(not(windows))]
fn metadata_is_platform_redirect(_metadata: &Metadata) -> bool {
    false
}

fn ensure_existing_child_chain_has_no_symlink(
    state_root: &Path,
    path: &Path,
) -> Result<(), ImportExecutionError> {
    ensure_directory_chain_without_untrusted_redirects(state_root, false)?;
    let root_metadata = std::fs::symlink_metadata(state_root).map_err(|_| {
        ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_parent_metadata_failed",
        }
    })?;
    if metadata_is_path_redirect(&root_metadata) || !root_metadata.is_dir() {
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
        if metadata_is_path_redirect(&metadata) || !metadata.is_dir() {
            return Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_parent_type_unsafe",
            });
        }
    }
    Ok(())
}

fn reject_symlink_target(path: &Path) -> Result<(), ImportExecutionError> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata_is_path_redirect(&metadata) || !metadata.is_file() => {
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
    #[cfg(test)]
    if import_directory_sync_failpoint_should_fail() {
        return Err(ImportExecutionError::DurableStateUnavailable {
            reason_code: "injected_durable_directory_sync_failure",
        });
    }
    sync_directory_uninjected(path)
}

fn sync_directory_uninjected(path: &Path) -> Result<(), ImportExecutionError> {
    #[cfg(unix)]
    File::open(path)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| ImportExecutionError::DurableStateUnavailable {
            reason_code: "durable_directory_sync_failed",
        })?;
    #[cfg(not(unix))]
    let _ = path;
    Ok(())
}

#[cfg(test)]
thread_local! {
    static IMPORT_DIRECTORY_SYNC_FAIL_ON_CALL: std::cell::Cell<usize> = const {
        std::cell::Cell::new(0)
    };
}

#[cfg(test)]
fn set_import_directory_sync_fail_on_call(call: usize) {
    IMPORT_DIRECTORY_SYNC_FAIL_ON_CALL.with(|remaining| remaining.set(call));
}

#[cfg(test)]
fn import_directory_sync_failpoint_should_fail() -> bool {
    IMPORT_DIRECTORY_SYNC_FAIL_ON_CALL.with(|remaining| match remaining.get() {
        0 => false,
        1 => {
            remaining.set(0);
            true
        }
        count => {
            remaining.set(count - 1);
            false
        }
    })
}

fn sync_directory_revalidating_identity(path: &Path) -> Result<(), ImportExecutionError> {
    let expected = direct_directory_identity(path)?;
    sync_directory(path).map_err(|_| durable_commit_state_uncertain())?;
    let observed =
        direct_directory_identity(path).map_err(|_| ImportExecutionError::ConcurrentMutation)?;
    if observed != expected {
        return Err(ImportExecutionError::ConcurrentMutation);
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

        let debug = format!("{preview:?}");
        assert!(!debug.contains("never-persist-this-secret"));
        assert!(!debug.contains("service.apiToken"));
        assert!(!debug.contains(&layout.source_root.display().to_string()));
        assert!(debug.contains("has_live_authority"));
    }

    #[test]
    fn rollback_request_debug_redacts_tokens_and_invalid_checkpoint_input() {
        let token = "customer-secret-idempotency-token";
        let request = ImportRollbackRequest {
            checkpoint_ref: "/Users/alice/Private/checkpoint.json",
            idempotency_token: token,
        };
        let debug = format!("{request:?}");
        assert!(!debug.contains(token));
        assert!(!debug.contains("/Users/alice"));
        assert!(debug.contains("[redacted]"));
        assert!(debug.contains("[redacted invalid checkpoint ref]"));
    }

    #[test]
    fn preview_rejects_source_row_count_above_contract_bound() {
        let mut settings = String::from("{");
        for index in 0..=MAX_IMPORT_PREVIEW_ROWS {
            if index > 0 {
                settings.push(',');
            }
            settings.push_str(&format!("\"{index:x}\":0"));
        }
        settings.push('}');
        assert!(settings.len() as u64 <= MAX_SOURCE_FILE_BYTES);

        let layout = TestLayout::vscode(&settings);
        assert_eq!(
            layout
                .store()
                .preview(&layout.review("profile:default"), POLICY_EPOCH),
            Err(ImportExecutionError::InputTooLarge {
                source_item_ref: "source_settings".to_owned(),
            })
        );
    }

    #[test]
    fn source_setting_refs_hash_path_like_or_noncanonical_user_keys() {
        let layout = TestLayout::vscode(
            r#"{
                "../../customer/private": true,
                "https://example.invalid/customer": false,
                "C:customer-private": false,
                "unknown.harmless": true
            }"#,
        );
        let preview = layout
            .store()
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("preview");
        let exported = serde_json::to_string(&preview).expect("preview export");
        assert!(!exported.contains("../../customer/private"));
        assert!(!exported.contains("https://example.invalid/customer"));
        assert!(!exported.contains("C:customer-private"));
        assert!(exported.contains("unknown.harmless"));
        assert!(preview.rows.iter().all(|row| {
            row.source_setting_ref
                .split_once('#')
                .map(|(_, key)| source_fragment_is_valid(key))
                .unwrap_or(true)
        }));

        assert!(valid_source_setting_ref(
            ".idea/codeStyles/Project.xml#option:TAB_SIZE"
        ));
        for unsafe_ref in [
            "/Users/alice/settings.json#editor.tabSize",
            "../private/settings.json#editor.tabSize",
            ".vscode/settings.json#https://example.invalid/private",
            ".vscode/settings.json#key with spaces",
        ] {
            assert!(
                !valid_source_setting_ref(unsafe_ref),
                "accepted {unsafe_ref:?}"
            );
        }
    }

    #[test]
    fn review_envelope_binds_source_destination_and_classification() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let store = layout.store();
        let review = layout.review("profile:default");

        let mut changed_destination = review.clone();
        changed_destination.destination_workspace_target = "profile:other".to_owned();
        let mut changed_source = review.clone();
        changed_source.source_path = layout
            ._temp
            .path()
            .join("different-source")
            .display()
            .to_string();
        let mut changed_classification = review;
        changed_classification.classification = CompetitorConfigClassification::JetBrainsIdeaRoot;

        for changed in [changed_destination, changed_source, changed_classification] {
            assert_eq!(
                store.preview(&changed, POLICY_EPOCH),
                Err(ImportExecutionError::InvalidRequest {
                    reason_code: "import_review_envelope_invalid",
                })
            );
        }
        assert!(!layout.state_root.exists());
    }

    #[test]
    fn imported_setting_debug_never_exports_user_values_or_store_paths() {
        let text = ImportSettingValue::Text("private-profile-value".to_owned());
        assert_eq!(format!("{text:?}"), "Text([redacted])");
        let integer = ImportSettingValue::Integer(42);
        assert_eq!(format!("{integer:?}"), "Integer([redacted])");

        let store = ImportedProfileStore::with_roots(
            "/private/config/customer-name",
            "/private/state/customer-name",
        );
        let debug = format!("{store:?}");
        assert!(!debug.contains("customer-name"));
        assert!(!debug.contains("/private/"));
        assert!(debug.contains("uses_distinct_roots: true"));
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
            .join("import_checkpoints")
            .join(profile_key)
            .join(format!("{checkpoint_key}.json"))
            .is_file());
        assert!(layout
            .state_root
            .join("import_checkpoint_bodies")
            .join(profile_key)
            .join(format!("{checkpoint_key}.json"))
            .is_file());
        assert!(layout
            .state_root
            .join(format!(
                "imported-{profile_key}-00000000000000000001.profile-state.json"
            ))
            .is_file());
    }

    #[test]
    fn durable_history_rejects_rollback_without_preceding_apply_lineage() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let store = layout.store();
        let preview = store
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("preview");
        store
            .apply(&preview, "history-lineage-apply", POLICY_EPOCH)
            .expect("apply");
        let mut state = store
            .load_profile_for_target("profile:default")
            .expect("load")
            .expect("state");
        state.history[0].action = ImportedProfileHistoryAction::RolledBack;

        assert_eq!(
            validate_durable_state(&state, &state.profile_ref),
            Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "profile_history_rollback_lineage_invalid",
            })
        );
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
        out_of_range
            .settings
            .get_mut("editor.tab_size")
            .expect("tab size setting")
            .value = ImportSettingValue::Integer(99);
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
        invalid_enum
            .settings
            .get_mut("ui.theme")
            .expect("theme setting")
            .value = ImportSettingValue::Text("extension-theme-name".to_owned());
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
        let checkpoint = ImportExecutionCheckpointBody {
            record_kind: IMPORT_EXECUTION_CHECKPOINT_BODY_RECORD_KIND.to_owned(),
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
            created_at: "2001-02-03T04:05:06Z".to_owned(),
        };
        store
            .write_checkpoint(&checkpoint)
            .expect("checkpoint-only crash marker");
        let metadata_path = store
            .checkpoint_path(&authority.profile_key, &checkpoint_ref)
            .expect("checkpoint metadata path");
        fs::remove_file(&metadata_path).expect("simulate crash before metadata publication");

        let profile_filename = format!(
            "imported-{}-00000000000000000001.profile-state.json",
            authority.profile_key
        );
        fs::write(
            layout
                .state_root
                .join(format!(".{profile_filename}.tmp.crashed")),
            b"partial",
        )
        .expect("crashed temporary revision");
        fs::write(
            layout.state_root.join("mutation.lock"),
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
        let metadata: ImportExecutionCheckpointMetadata = serde_json::from_slice(
            &fs::read(metadata_path).expect("retry published checkpoint metadata"),
        )
        .expect("checkpoint metadata");
        assert_eq!(metadata.created_at, checkpoint.created_at);
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
            .join(format!(
                "imported-{key}-00000000000000000001.profile-state.json"
            ))
            .is_file());
        assert!(!layout._temp.path().join("outside").exists());
    }

    #[test]
    fn split_roots_keep_profile_truth_out_of_local_history() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let imported_profile_state = layout
            ._temp
            .path()
            .join("config")
            .join("profiles")
            .join("imported");
        let checkpoint_history = layout._temp.path().join("state").join("history");
        let store = ImportedProfileStore::with_roots(&imported_profile_state, &checkpoint_history);
        let preview = store
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("preview");
        let outcome = store
            .apply(&preview, "split-root-apply", POLICY_EPOCH)
            .expect("apply");
        let (profile_key, checkpoint_key) =
            parse_checkpoint_ref(outcome.checkpoint_ref.as_deref().expect("checkpoint ref"))
                .expect("checkpoint parses");

        let imported_state_path = imported_profile_state.join(format!(
            "imported-{profile_key}-00000000000000000001.profile-state.json"
        ));
        assert!(imported_state_path.is_file());
        let durable_json: serde_json::Value = serde_json::from_slice(
            &fs::read(&imported_state_path).expect("read imported profile state"),
        )
        .expect("parse imported profile state");
        assert_eq!(
            durable_json["record_kind"],
            IMPORTED_PROFILE_STATE_RECORD_KIND
        );
        assert_eq!(
            durable_json["schema_version"],
            IMPORT_EXECUTION_SCHEMA_VERSION
        );
        let checkpoint_path = checkpoint_history
            .join("import_checkpoints")
            .join(profile_key)
            .join(format!("{checkpoint_key}.json"));
        assert!(checkpoint_path.is_file());
        let checkpoint_json: serde_json::Value =
            serde_json::from_slice(&fs::read(checkpoint_path).expect("read import checkpoint"))
                .expect("parse import checkpoint");
        assert_eq!(
            checkpoint_json["record_kind"],
            IMPORT_EXECUTION_CHECKPOINT_METADATA_RECORD_KIND
        );
        assert_eq!(
            checkpoint_json["schema_version"],
            IMPORT_EXECUTION_SCHEMA_VERSION
        );
        assert!(checkpoint_json.get("protected_body_ref").is_some());
        assert!(checkpoint_json.get("protected_body_digest").is_some());
        assert!(checkpoint_json.get("migration_session_ref").is_none());
        assert!(checkpoint_json
            .get("migration_restore_record_ref")
            .is_none());
        assert!(checkpoint_json.get("compatibility_report_ref").is_none());
        assert!(checkpoint_json
            .get("rollback_checkpoint_outcome_class")
            .is_none());
        assert!(checkpoint_json.get("prior_state").is_none());
        let checkpoint_body_path = checkpoint_history
            .join("import_checkpoint_bodies")
            .join(profile_key)
            .join(format!("{checkpoint_key}.json"));
        let checkpoint_body_json: serde_json::Value = serde_json::from_slice(
            &fs::read(checkpoint_body_path).expect("read protected checkpoint body"),
        )
        .expect("parse protected checkpoint body");
        assert_eq!(
            checkpoint_body_json["record_kind"],
            IMPORT_EXECUTION_CHECKPOINT_BODY_RECORD_KIND
        );
        let checkpoint_body: ImportExecutionCheckpointBody =
            serde_json::from_value(checkpoint_body_json).expect("typed checkpoint body");
        assert_eq!(
            checkpoint_json["protected_body_digest"],
            digest_serializable(&checkpoint_body).expect("checkpoint body digest")
        );
        assert!(!checkpoint_history.join("profiles").exists());
        assert!(!imported_profile_state.join("import_checkpoints").exists());
    }

    #[test]
    fn relative_durable_store_roots_fail_before_source_or_state_io() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let store = ImportedProfileStore::with_roots(
            PathBuf::from("relative-config/profiles/imported"),
            PathBuf::from("relative-state/history/imports"),
        );

        assert_eq!(
            store.preview(&layout.review("profile:default"), POLICY_EPOCH),
            Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_store_root_invalid",
            })
        );
        assert!(!Path::new("relative-config").exists());
        assert!(!Path::new("relative-state").exists());
    }

    #[test]
    fn production_store_roots_must_be_physically_separate_families() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let shared = layout._temp.path().join("config");
        let overlapping_stores = [
            ImportedProfileStore::with_roots(&shared, &shared),
            ImportedProfileStore::with_roots(&shared, shared.join("history")),
            ImportedProfileStore::with_roots(shared.join("profiles"), &shared),
        ];

        for store in overlapping_stores {
            assert_eq!(
                store.preview(&layout.review("profile:default"), POLICY_EPOCH),
                Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "durable_store_roots_overlap",
                })
            );
        }
        assert!(!shared.exists());
    }

    #[cfg(unix)]
    #[test]
    fn filesystem_root_is_not_a_durable_import_store() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let store = ImportedProfileStore::with_roots(
            Path::new("/"),
            layout._temp.path().join("state").join("history"),
        );

        assert_eq!(
            store.preview(&layout.review("profile:default"), POLICY_EPOCH),
            Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_store_root_invalid",
            })
        );
    }

    #[test]
    fn portable_profile_suffix_is_never_activated_as_imported_profile_state() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let imported_profile_state = layout
            ._temp
            .path()
            .join("config")
            .join("profiles")
            .join("imported");
        fs::create_dir_all(&imported_profile_state).expect("imported state root");
        let key = profile_key("profile:default");
        fs::write(
            imported_profile_state.join(format!(
                "imported-{key}-00000000000000000001.aureprofile.json"
            )),
            br#"{"record_kind":"imported_profile_state_record","schema_version":1}"#,
        )
        .expect("legacy/misclassified portable-suffix row");

        let store = ImportedProfileStore::with_roots(
            &imported_profile_state,
            layout._temp.path().join("state").join("history"),
        );
        assert_eq!(
            store
                .load_profile_for_target("profile:default")
                .expect("conforming load"),
            None
        );
    }

    #[test]
    fn misplaced_legacy_state_is_not_implicitly_activated() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let misplaced_state = layout._temp.path().join("state").join("imports");
        let legacy_store = ImportedProfileStore::new(&misplaced_state);
        let preview = legacy_store
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("preview");
        legacy_store
            .apply(&preview, "misplaced-state-apply", POLICY_EPOCH)
            .expect("legacy-shaped apply");

        let configured_profiles = layout
            ._temp
            .path()
            .join("config")
            .join("profiles")
            .join("imported");
        let conforming = ImportedProfileStore::with_roots(
            configured_profiles,
            layout._temp.path().join("state").join("history"),
        );
        assert_eq!(
            conforming
                .load_profile_for_target("profile:default")
                .expect("conforming load"),
            None
        );
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
        assert!(matches!(
            store.preview(&layout.review("profile:default"), POLICY_EPOCH),
            Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "profile_revision_directory_type_unsafe"
            })
        ));
        assert!(fs::read_dir(real_state)
            .expect("read real state")
            .next()
            .is_none());
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn trusted_macos_var_alias_can_publish_and_reload_durable_state() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let canonical_temp = layout
            ._temp
            .path()
            .canonicalize()
            .expect("canonical tempdir");
        let suffix = canonical_temp
            .strip_prefix("/private/var")
            .expect("macOS tempdir below /private/var");
        let alias_root = Path::new("/var").join(suffix);
        let imported_profiles = alias_root.join("config/profiles/imported");
        let checkpoint_history = alias_root.join("state/history/imports");
        let store = ImportedProfileStore::with_roots(imported_profiles, checkpoint_history);
        let preview = store
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("preview");

        store
            .apply(&preview, "macos-platform-root-alias", POLICY_EPOCH)
            .expect("trusted platform alias apply");

        assert!(store
            .load_profile_for_target("profile:default")
            .expect("reload through alias")
            .is_some());
    }

    #[cfg(windows)]
    #[test]
    fn windows_reparse_attribute_is_classified_as_redirect() {
        assert!(windows_file_attributes_include_reparse_point(0x400));
        assert!(windows_file_attributes_include_reparse_point(0x400 | 0x10));
        assert!(!windows_file_attributes_include_reparse_point(0x10));
    }

    #[cfg(unix)]
    #[test]
    fn durable_read_rejects_same_length_file_replacement() {
        let temp = tempfile::tempdir().expect("tempdir");
        let state_root = temp.path().join("state");
        fs::create_dir(&state_root).expect("state root");
        let durable_path = state_root.join("profile-state.json");
        fs::write(&durable_path, b"original").expect("durable file");
        let replacement = state_root.join("replacement.json");
        fs::write(&replacement, b"replaced").expect("same-length replacement");

        let result =
            read_optional_durable_file_with_post_read_hook(&state_root, &durable_path, |_| {
                fs::rename(&replacement, &durable_path).expect("replace durable path");
            });

        assert_eq!(result, Err(durable_file_changed_during_read()));
    }

    #[cfg(unix)]
    #[test]
    fn durable_publish_rejects_parent_replacement_before_install() {
        let temp = tempfile::tempdir().expect("tempdir");
        let state_root = temp.path().join("state");
        let durable_path = state_root.join("profile-state.json");
        let moved_original = temp.path().join("moved-original-state");
        let outside_directory = temp.path().join("outside");
        fs::create_dir(&outside_directory).expect("outside directory");
        let outside_sentinel = outside_directory.join("sentinel.txt");
        fs::write(&outside_sentinel, b"retain-me\n").expect("outside sentinel");

        let result = write_new_json_with_preinstall_hook(
            &state_root,
            &durable_path,
            &serde_json::json!({"record_kind": "identity_swap_test"}),
            || {
                fs::rename(&state_root, &moved_original).expect("move pinned parent");
                fs::create_dir(&state_root).expect("replacement parent");
            },
        );

        assert_eq!(result, Err(ImportExecutionError::ConcurrentMutation));
        assert!(!durable_path.exists());
        assert!(fs::read_dir(&state_root)
            .expect("replacement parent remains readable")
            .next()
            .is_none());
        let moved_entries = fs::read_dir(&moved_original)
            .expect("moved parent remains readable")
            .map(|entry| entry.expect("moved parent entry").path())
            .collect::<Vec<_>>();
        assert_eq!(moved_entries.len(), 1);
        let staged_path = &moved_entries[0];
        assert!(staged_path
            .file_name()
            .expect("staged filename")
            .to_string_lossy()
            .contains(".tmp."));
        let staged_bytes = fs::read(staged_path).expect("scrubbed staged file");
        assert!(staged_bytes.is_empty());
        assert_eq!(
            fs::metadata(staged_path)
                .expect("scrubbed staged metadata")
                .len(),
            0
        );
        assert!(!String::from_utf8_lossy(&staged_bytes).contains("identity_swap_test"));
        assert_eq!(
            fs::read(&outside_sentinel).expect("outside sentinel remains readable"),
            b"retain-me\n"
        );
    }

    #[test]
    fn durable_publish_scrubs_staged_payload_when_install_fails() {
        let temp = tempfile::tempdir().expect("tempdir");
        let state_root = temp.path().join("state");
        fs::create_dir(&state_root).expect("state root");
        let durable_path = state_root.join("profile-state.json");
        fs::write(&durable_path, b"preexisting\n").expect("preexisting destination");

        let result = write_new_json(
            &state_root,
            &durable_path,
            &serde_json::json!({"record_kind": "install_failure_private_payload"}),
        );

        assert_eq!(
            result,
            Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_create_new_failed",
            })
        );
        assert_eq!(
            fs::read(&durable_path).expect("preexisting destination remains readable"),
            b"preexisting\n"
        );
        let remaining_entries = fs::read_dir(&state_root)
            .expect("state root remains readable")
            .map(|entry| entry.expect("state root entry").path())
            .collect::<Vec<_>>();
        assert_eq!(remaining_entries, vec![durable_path]);
    }

    #[test]
    fn durable_publish_preserves_installed_payload_when_commit_state_is_uncertain() {
        let temp = tempfile::tempdir().expect("tempdir");
        let state_root = temp.path().join("state");
        let durable_path = state_root.join("profile-state.json");
        let value = serde_json::json!({
            "record_kind": "postinstall_uncertainty_private_payload"
        });

        let result = write_new_json_with_hooks(
            &state_root,
            &durable_path,
            &value,
            || {},
            || {
                Err(ImportExecutionError::DurableStateUnavailable {
                    reason_code: "injected_postinstall_failure",
                })
            },
        );

        assert_eq!(result, Ok(WriteNewJsonOutcome::CommitStateUncertain));
        let installed = fs::read(&durable_path).expect("installed payload remains readable");
        assert!(!installed.is_empty());
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&installed).expect("installed JSON"),
            value
        );
        let staged_paths = fs::read_dir(&state_root)
            .expect("state root remains readable")
            .map(|entry| entry.expect("state root entry").path())
            .filter(|path| path != &durable_path)
            .collect::<Vec<_>>();
        assert_eq!(staged_paths.len(), 1);
        assert_eq!(
            fs::read(&staged_paths[0]).expect("uncertain staged link remains readable"),
            installed
        );
    }

    #[test]
    fn durable_publish_preserves_destination_when_directory_sync_fails() {
        let temp = tempfile::tempdir().expect("tempdir");
        let state_root = temp.path().join("state");
        let durable_path = state_root.join("profile-state.json");
        let value = serde_json::json!({"record_kind": "directory_sync_uncertainty"});
        set_import_directory_sync_fail_on_call(1);

        let result = write_new_json(&state_root, &durable_path, &value);

        assert_eq!(result, Ok(WriteNewJsonOutcome::CommitStateUncertain));
        let installed = fs::read(&durable_path).expect("installed payload remains readable");
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&installed).expect("installed JSON"),
            value
        );
        let entries = fs::read_dir(&state_root)
            .expect("state root remains readable")
            .map(|entry| entry.expect("state root entry").path())
            .collect::<Vec<_>>();
        assert_eq!(entries, vec![durable_path]);
    }

    #[test]
    fn checkpoint_commit_uncertainty_fails_apply_then_reconciles_on_retry() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let store = layout.store();
        let preview = store
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("preview");
        set_import_directory_sync_fail_on_call(1);

        assert_eq!(
            store.apply(&preview, "checkpoint-sync-retry", POLICY_EPOCH),
            Err(durable_commit_state_uncertain())
        );
        let retried = store
            .apply(&preview, "checkpoint-sync-retry", POLICY_EPOCH)
            .expect("retry reconciles checkpoint then applies profile");
        assert_eq!(retried.disposition, ImportApplyDisposition::Applied);
    }

    #[test]
    fn rollback_metadata_uncertainty_reconciles_without_rewriting_private_body() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let store = layout.store();
        let preview = store
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("preview");
        set_import_directory_sync_fail_on_call(2);

        assert_eq!(
            store.apply(&preview, "metadata-sync-retry", POLICY_EPOCH),
            Err(durable_commit_state_uncertain())
        );
        let retried = store
            .apply(&preview, "metadata-sync-retry", POLICY_EPOCH)
            .expect("retry reconciles body and private checkpoint metadata");
        assert_eq!(retried.disposition, ImportApplyDisposition::Applied);
    }

    #[test]
    fn profile_commit_uncertainty_fails_apply_then_reconciles_as_idempotent() {
        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let store = layout.store();
        let preview = store
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("preview");
        set_import_directory_sync_fail_on_call(3);

        assert_eq!(
            store.apply(&preview, "profile-sync-retry", POLICY_EPOCH),
            Err(durable_commit_state_uncertain())
        );
        let retried = store
            .apply(&preview, "profile-sync-retry", POLICY_EPOCH)
            .expect("retry revalidates the installed profile revision");
        assert_eq!(retried.disposition, ImportApplyDisposition::AlreadyApplied);
    }

    #[cfg(panic = "unwind")]
    #[test]
    fn durable_publish_scrubs_staged_payload_when_hook_unwinds() {
        let temp = tempfile::tempdir().expect("tempdir");
        let state_root = temp.path().join("state");
        let durable_path = state_root.join("profile-state.json");

        let panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = write_new_json_with_preinstall_hook(
                &state_root,
                &durable_path,
                &serde_json::json!({"record_kind": "unwind_private_payload"}),
                || panic!("injected preinstall unwind"),
            );
        }));

        assert!(panic_result.is_err());
        assert!(!durable_path.exists());
        let staged_entries = fs::read_dir(&state_root)
            .expect("state root remains readable")
            .map(|entry| entry.expect("state root entry").path())
            .collect::<Vec<_>>();
        assert_eq!(staged_entries.len(), 1);
        assert!(staged_entries[0]
            .file_name()
            .expect("staged filename")
            .to_string_lossy()
            .contains(".tmp."));
        let staged_bytes = fs::read(&staged_entries[0]).expect("scrubbed staged file");
        assert!(staged_bytes.is_empty());
        assert!(!String::from_utf8_lossy(&staged_bytes).contains("unwind_private_payload"));
    }

    #[cfg(panic = "unwind")]
    #[test]
    fn durable_publish_does_not_scrub_destination_when_postinstall_hook_unwinds() {
        let temp = tempfile::tempdir().expect("tempdir");
        let state_root = temp.path().join("state");
        let durable_path = state_root.join("profile-state.json");

        let panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = write_new_json_with_hooks(
                &state_root,
                &durable_path,
                &serde_json::json!({"record_kind": "postinstall_unwind_private_payload"}),
                || {},
                || panic!("injected postinstall unwind"),
            );
        }));

        assert!(panic_result.is_err());
        let installed = fs::read(&durable_path).expect("installed payload remains readable");
        assert!(!installed.is_empty());
        assert!(String::from_utf8_lossy(&installed).contains("postinstall_unwind_private_payload"));
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_durable_root_ancestor_is_rejected_before_write() {
        use std::os::unix::fs::symlink;

        let layout = TestLayout::vscode(r#"{"editor.tabSize":2}"#);
        let real_config = layout._temp.path().join("real-config");
        fs::create_dir(&real_config).expect("real config");
        let config_link = layout._temp.path().join("config-link");
        symlink(&real_config, &config_link).expect("config ancestor symlink");
        let store = ImportedProfileStore::with_roots(
            config_link.join("profiles").join("imported"),
            layout._temp.path().join("state").join("history"),
        );
        let preview = store
            .preview(&layout.review("profile:default"), POLICY_EPOCH)
            .expect("preview remains read only");

        assert!(matches!(
            store.apply(&preview, "unsafe-config-ancestor", POLICY_EPOCH),
            Err(ImportExecutionError::DurableStateUnavailable {
                reason_code: "durable_directory_type_unsafe"
            })
        ));
        assert!(fs::read_dir(real_config)
            .expect("read real config")
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
        assert!(valid_utc_timestamp("2024-02-29T23:59:59Z"));
        for invalid in [
            "2023-02-29T00:00:00Z",
            "2024-13-01T00:00:00Z",
            "2024-01-00T00:00:00Z",
            "2024-01-01T24:00:00Z",
            "2024-01-01T00:60:00Z",
            "2024-01-01T00:00:60Z",
            "2024-01-01T00:00:00+00:00",
            "not-a-timestamp",
        ] {
            assert!(!valid_utc_timestamp(invalid), "accepted {invalid}");
        }
    }
}
