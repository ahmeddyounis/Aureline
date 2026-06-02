//! Hardened recovery-ladder flows for cache rebuild, settings repair,
//! state-migration repair, and targeted resets for the M4 stable lane.
//!
//! This module promotes four recovery-ladder rung families from alpha to
//! stable contracts. Every hardened flow is typed, truthful, narrow in
//! blast radius, versioned, and export-safe:
//!
//! - **Typed** — every flow carries a closed [`HardenedRecoveryFlowClass`],
//!   a [`HardenedRecoveryBlastRadiusClass`], a
//!   [`HardenedRecoveryReversalClass`], and a
//!   [`HardenedRecoveryCheckpointClass`] instead of free-form text.
//! - **Truthful** — the evaluator rejects flows that claim exact rollback
//!   without a durable checkpoint, or that claim no blast radius while
//!   naming impacted state classes outside the declared scope.
//! - **Narrow** — the blast-radius vocabulary is capped at
//!   single-disposable-state, same-family, cross-family, or escalation-only;
//!   no flow may silently widen to a destructive reset.
//! - **Versioned** — schema version 1 is frozen; later changes must bump
//!   the version and provide a migration note.
//! - **Export-safe** — the support packet is metadata-only by default; raw
//!   private material and ambient authority are excluded.
//!
//! The module owns four typed flow families:
//!
//! - [`HardenedRecoveryFlowClass::CacheRebuild`] — rebuilds disposable
//!   derived cache or index state from authoritative sources. Blast radius
//!   is capped at a single disposable state class; reversal is regenerate.
//! - [`HardenedRecoveryFlowClass::SettingsRepair`] — repairs settings or
//!   profile state from a captured backup or authoritative source. Blast
//!   radius is same-family; reversal is compensating or exact from checkpoint.
//! - [`HardenedRecoveryFlowClass::StateMigrationRepair`] — repairs or
//!   rolls back a failed schema migration. Blast radius is cross-family;
//!   reversal is manual or compensating; durable checkpoint is required.
//! - [`HardenedRecoveryFlowClass::TargetedReset`] — resets exactly one
//!   targeted disposable state class. Blast radius is single-object;
//!   reversal is exact from checkpoint or regenerate.
//!
//! The [`HardenedRecoveryFlowRecord`] binds one flow class, one rung ref,
//! one blast-radius declaration, one reversal class, one checkpoint class,
//! the preserved and impacted state classes, consent requirements, and a
//! reviewer-facing support-guidance string.
//!
//! The [`HardenedRecoveryFlowEvaluator`] validates that:
//!
//! - every flow preserves `user_authored_files`,
//! - every flow cites a `doctor.finding.*` ref,
//! - cache-rebuild flows limit impact to disposable state and declare
//!   regenerate reversal,
//! - settings-repair flows preserve settings-profile state, declare a
//!   durable checkpoint, and do not cross family boundaries silently,
//! - state-migration-repair flows preserve durable workspace indexes,
//!   declare a durable checkpoint, and do not mutate user-authored files,
//! - targeted-reset flows declare exactly one impacted state class, keep
//!   the blast radius at single-object, and preserve all durable state,
//! - no flow declares a destructive reset.
//!
//! The [`HardenedRecoveryFlowSupportPacket`] folds validated flows into a
//! metadata-safe projection that support-export and release-evidence
//! pipelines can consume verbatim.
//!
//! The boundary schema is at
//! `/schemas/support/harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets.schema.json`.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::recovery_ladder::RecoveryRungClass;

/// Stable record-kind tag for a hardened recovery-ladder flow record.
pub const HARDENED_RECOVERY_FLOW_RECORD_KIND: &str =
    "hardened_recovery_ladder_flow_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets_record";

/// Stable record-kind tag for the hardened recovery-flow support packet.
pub const HARDENED_RECOVERY_FLOW_SUPPORT_PACKET_RECORD_KIND: &str =
    "hardened_recovery_ladder_flow_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets_support_packet";

/// Integer schema version for hardened recovery-flow records.
pub const HARDENED_RECOVERY_FLOW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const HARDENED_RECOVERY_FLOW_SCHEMA_REF: &str =
    "schemas/support/harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const HARDENED_RECOVERY_FLOW_DOC_REF: &str =
    "docs/support/m4/harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const HARDENED_RECOVERY_FLOW_ARTIFACT_REF: &str =
    "artifacts/support/m4/harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const HARDENED_RECOVERY_FLOW_FIXTURE_DIR: &str =
    "fixtures/support/m4/harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets";

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// Closed hardened recovery-flow class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardenedRecoveryFlowClass {
    /// Rebuild disposable derived cache or index from authoritative state.
    CacheRebuild,
    /// Repair settings or profile state from backup or authoritative source.
    SettingsRepair,
    /// Repair or roll back a failed schema migration.
    StateMigrationRepair,
    /// Reset exactly one targeted disposable state class.
    TargetedReset,
}

impl HardenedRecoveryFlowClass {
    /// Every required flow class, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::CacheRebuild,
        Self::SettingsRepair,
        Self::StateMigrationRepair,
        Self::TargetedReset,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CacheRebuild => "cache_rebuild",
            Self::SettingsRepair => "settings_repair",
            Self::StateMigrationRepair => "state_migration_repair",
            Self::TargetedReset => "targeted_reset",
        }
    }

    /// Returns the default blast-radius class for this flow family.
    pub const fn default_blast_radius(self) -> HardenedRecoveryBlastRadiusClass {
        match self {
            Self::CacheRebuild => HardenedRecoveryBlastRadiusClass::SingleDisposableStateClass,
            Self::SettingsRepair => HardenedRecoveryBlastRadiusClass::SameFamilyStateClasses,
            Self::StateMigrationRepair => HardenedRecoveryBlastRadiusClass::CrossFamilyStateClasses,
            Self::TargetedReset => HardenedRecoveryBlastRadiusClass::SingleDisposableStateClass,
        }
    }

    /// Returns the default reversal class for this flow family.
    pub const fn default_reversal(self) -> HardenedRecoveryReversalClass {
        match self {
            Self::CacheRebuild => HardenedRecoveryReversalClass::Regenerate,
            Self::SettingsRepair => HardenedRecoveryReversalClass::Compensating,
            Self::StateMigrationRepair => HardenedRecoveryReversalClass::Manual,
            Self::TargetedReset => HardenedRecoveryReversalClass::Exact,
        }
    }

    /// Returns the default checkpoint class for this flow family.
    pub const fn default_checkpoint(self) -> HardenedRecoveryCheckpointClass {
        match self {
            Self::CacheRebuild => HardenedRecoveryCheckpointClass::DurablePreApply,
            Self::SettingsRepair => HardenedRecoveryCheckpointClass::DurablePreApply,
            Self::StateMigrationRepair => HardenedRecoveryCheckpointClass::DurablePreApply,
            Self::TargetedReset => HardenedRecoveryCheckpointClass::DurablePreApply,
        }
    }
}

/// Closed blast-radius class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardenedRecoveryBlastRadiusClass {
    /// Exactly one disposable state class is impacted.
    SingleDisposableStateClass,
    /// Multiple state classes within the same family (for example settings).
    SameFamilyStateClasses,
    /// State classes across different families.
    CrossFamilyStateClasses,
    /// No local apply; escalation only.
    EscalationOnly,
}

impl HardenedRecoveryBlastRadiusClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleDisposableStateClass => "single_disposable_state_class",
            Self::SameFamilyStateClasses => "same_family_state_classes",
            Self::CrossFamilyStateClasses => "cross_family_state_classes",
            Self::EscalationOnly => "escalation_only",
        }
    }
}

/// Closed reversal class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardenedRecoveryReversalClass {
    /// Exact bit-for-bit restore from a captured checkpoint.
    Exact,
    /// Compensating semantic restore when exact restore is unavailable.
    Compensating,
    /// Regenerate disposable derived state from authoritative sources.
    Regenerate,
    /// Manual user-guided recovery path.
    Manual,
    /// Audit-only; no state changed.
    AuditOnly,
}

impl HardenedRecoveryReversalClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Compensating => "compensating",
            Self::Regenerate => "regenerate",
            Self::Manual => "manual",
            Self::AuditOnly => "audit_only",
        }
    }

    /// Returns true when this reversal class requires a checkpoint ref.
    pub const fn requires_checkpoint_ref(self) -> bool {
        matches!(self, Self::Exact)
    }
}

/// Closed checkpoint class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardenedRecoveryCheckpointClass {
    /// A durable on-disk checkpoint captured before apply.
    DurablePreApply,
    /// A session-scoped ephemeral checkpoint captured before apply.
    EphemeralPreApply,
    /// No checkpoint because the flow is observe-only or audit-only.
    NoCheckpointNeeded,
    /// Checkpoint capture was refused because the state class cannot be checkpointed.
    CheckpointRefused,
}

impl HardenedRecoveryCheckpointClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurablePreApply => "durable_pre_apply",
            Self::EphemeralPreApply => "ephemeral_pre_apply",
            Self::NoCheckpointNeeded => "no_checkpoint_needed",
            Self::CheckpointRefused => "checkpoint_refused",
        }
    }

    /// Returns true when this class requires a non-empty checkpoint ref.
    pub const fn requires_checkpoint_ref(self) -> bool {
        matches!(self, Self::DurablePreApply | Self::EphemeralPreApply)
    }
}

/// Closed preserved-state class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardenedRecoveryPreservedStateClass {
    /// User-authored files and buffers.
    UserAuthoredFiles,
    /// Selection, caret, and scroll state for open buffers.
    OpenBufferSelection,
    /// Durable index state that must not be deleted as collateral.
    DurableWorkspaceIndexes,
    /// Workspace trust state.
    WorkspaceTrustStore,
    /// Credential handles and stores.
    CredentialStore,
    /// Session restore records.
    SessionRestoreStore,
    /// Support export records and staging state.
    SupportExportStore,
    /// Settings and profile state.
    SettingsProfileState,
    /// State migration journal and checkpoint records.
    StateMigrationJournal,
}

impl HardenedRecoveryPreservedStateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserAuthoredFiles => "user_authored_files",
            Self::OpenBufferSelection => "open_buffer_selection",
            Self::DurableWorkspaceIndexes => "durable_workspace_indexes",
            Self::WorkspaceTrustStore => "workspace_trust_store",
            Self::CredentialStore => "credential_store",
            Self::SessionRestoreStore => "session_restore_store",
            Self::SupportExportStore => "support_export_store",
            Self::SettingsProfileState => "settings_profile_state",
            Self::StateMigrationJournal => "state_migration_journal",
        }
    }
}

/// Closed impacted-state class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardenedRecoveryImpactedStateClass {
    /// Disposable derived cache or index shards.
    DisposableDerivedCache,
    /// Watcher backlog or derived artifact state.
    WatcherBacklogState,
    /// Settings or profile state that will be repaired.
    SettingsProfileState,
    /// State migration journal or checkpoint that will be repaired.
    StateMigrationJournal,
    /// One targeted disposable state class.
    TargetedDisposableState,
}

impl HardenedRecoveryImpactedStateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisposableDerivedCache => "disposable_derived_cache",
            Self::WatcherBacklogState => "watcher_backlog_state",
            Self::SettingsProfileState => "settings_profile_state",
            Self::StateMigrationJournal => "state_migration_journal",
            Self::TargetedDisposableState => "targeted_disposable_state",
        }
    }
}

/// Closed redaction class vocabulary for evidence refs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardenedRecoveryRedactionClass {
    /// Metadata-safe default redaction.
    MetadataSafeDefault,
    /// Opt-in support evidence.
    OptInOnly,
    /// Prohibited from support or release projections.
    Prohibited,
}

impl HardenedRecoveryRedactionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::OptInOnly => "opt_in_only",
            Self::Prohibited => "prohibited",
        }
    }
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// One hardened recovery-ladder flow record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenedRecoveryFlowRecord {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable flow identifier.
    pub flow_id: String,
    /// Flow class.
    pub flow_class: HardenedRecoveryFlowClass,
    /// Recovery-ladder rung class bound to this flow.
    pub rung_class: RecoveryRungClass,
    /// Blast-radius class.
    pub blast_radius_class: HardenedRecoveryBlastRadiusClass,
    /// Reversal class.
    pub reversal_class: HardenedRecoveryReversalClass,
    /// Checkpoint class.
    pub checkpoint_class: HardenedRecoveryCheckpointClass,
    /// Checkpoint ref when available.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub checkpoint_ref: Option<String>,
    /// Project Doctor finding that justified the flow.
    pub doctor_finding_ref: String,
    /// Preserved state classes.
    pub preserved_state_classes: Vec<HardenedRecoveryPreservedStateClass>,
    /// Impacted state classes.
    pub impacted_state_classes: Vec<HardenedRecoveryImpactedStateClass>,
    /// Whether active user consent is required.
    pub requires_user_consent: bool,
    /// Whether active admin consent is required.
    pub requires_admin_consent: bool,
    /// User-facing support guidance string.
    pub support_guidance: String,
    /// Reviewer-facing flow summary.
    pub flow_summary: String,
    /// Evidence refs cited by the flow.
    pub evidence_refs: Vec<String>,
    /// Redaction class for the support packet.
    pub redaction_class: HardenedRecoveryRedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
}

/// One hardened recovery-flow support packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenedRecoveryFlowSupportPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Flow refs included in this packet.
    pub flow_refs: Vec<String>,
    /// Whether every included flow passed evaluation.
    pub all_flows_valid: bool,
    /// Redaction class.
    pub redaction_class: HardenedRecoveryRedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
}

// ---------------------------------------------------------------------------
// Violations and errors
// ---------------------------------------------------------------------------

/// One evaluation violation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenedRecoveryFlowViolation {
    /// Stable check identifier.
    pub check_id: String,
    /// Subject ref that failed the check.
    pub subject_ref: String,
    /// Reviewer-facing message.
    pub message: String,
}

/// Evaluation error carrying one or more violations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardenedRecoveryFlowError {
    /// Violations that caused the error.
    pub violations: Vec<HardenedRecoveryFlowViolation>,
}

impl fmt::Display for HardenedRecoveryFlowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "hardened recovery flow evaluation failed with {} violation(s): ",
            self.violations.len()
        )?;
        for v in &self.violations {
            write!(f, "[{}: {}] ", v.check_id, v.message)?;
        }
        Ok(())
    }
}

impl Error for HardenedRecoveryFlowError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// ---------------------------------------------------------------------------
// Load helpers
// ---------------------------------------------------------------------------

/// Loads a hardened recovery-ladder flow record from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`HardenedRecoveryFlowRecord`].
pub fn load_hardened_recovery_flow(
    yaml: &str,
) -> Result<HardenedRecoveryFlowRecord, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

// ---------------------------------------------------------------------------
// Evaluator
// ---------------------------------------------------------------------------

/// Evaluates hardened recovery-ladder flow records.
///
/// The evaluator is stateless and cheap to construct.
#[derive(Debug, Clone, Default)]
pub struct HardenedRecoveryFlowEvaluator;

impl HardenedRecoveryFlowEvaluator {
    /// Returns a new evaluator.
    pub fn new() -> Self {
        Self
    }

    /// Validates a single hardened recovery-ladder flow record.
    ///
    /// # Errors
    ///
    /// Returns [`HardenedRecoveryFlowError`] when any invariant is violated.
    pub fn validate_flow(
        &self,
        flow: &HardenedRecoveryFlowRecord,
    ) -> Result<(), HardenedRecoveryFlowError> {
        let violations = evaluate_flow(flow);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(HardenedRecoveryFlowError { violations })
        }
    }

    /// Builds a metadata-safe support packet from a validated flow record.
    ///
    /// The packet is metadata-only: it cites ids, refs, and closed-vocabulary
    /// tokens, never raw payloads, credentials, paths, or ambient authority.
    ///
    /// # Errors
    ///
    /// Returns [`HardenedRecoveryFlowError`] when the flow does not pass
    /// validation.
    pub fn support_packet(
        &self,
        flow: &HardenedRecoveryFlowRecord,
    ) -> Result<HardenedRecoveryFlowSupportPacket, HardenedRecoveryFlowError> {
        self.validate_flow(flow)?;
        Ok(HardenedRecoveryFlowSupportPacket {
            schema_version: HARDENED_RECOVERY_FLOW_SCHEMA_VERSION,
            record_kind: HARDENED_RECOVERY_FLOW_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            packet_id: format!("support_packet:{}", flow.flow_id),
            flow_refs: vec![flow.flow_id.clone()],
            all_flows_valid: true,
            redaction_class: flow.redaction_class,
            captured_at: flow.captured_at.clone(),
        })
    }

    /// Builds a support packet from a slice of flow records.
    ///
    /// # Errors
    ///
    /// Returns [`HardenedRecoveryFlowError`] when any flow does not pass
    /// validation.
    pub fn support_packet_from_flows(
        &self,
        flows: &[HardenedRecoveryFlowRecord],
    ) -> Result<HardenedRecoveryFlowSupportPacket, HardenedRecoveryFlowError> {
        let mut flow_refs = Vec::with_capacity(flows.len());
        for flow in flows {
            self.validate_flow(flow)?;
            flow_refs.push(flow.flow_id.clone());
        }
        let captured_at = flows
            .first()
            .map(|f| f.captured_at.clone())
            .unwrap_or_default();
        Ok(HardenedRecoveryFlowSupportPacket {
            schema_version: HARDENED_RECOVERY_FLOW_SCHEMA_VERSION,
            record_kind: HARDENED_RECOVERY_FLOW_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            packet_id: format!("support_packet:hardened_recovery_flows:{}", captured_at),
            flow_refs,
            all_flows_valid: true,
            redaction_class: HardenedRecoveryRedactionClass::MetadataSafeDefault,
            captured_at,
        })
    }
}

fn evaluate_flow(flow: &HardenedRecoveryFlowRecord) -> Vec<HardenedRecoveryFlowViolation> {
    let mut violations = Vec::new();

    if flow.schema_version != HARDENED_RECOVERY_FLOW_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "hardened_recovery_flow.schema_version_mismatch",
            &flow.flow_id,
            format!(
                "schema version must be {} ",
                HARDENED_RECOVERY_FLOW_SCHEMA_VERSION
            ),
        );
    }
    if flow.record_kind != HARDENED_RECOVERY_FLOW_RECORD_KIND {
        push_violation(
            &mut violations,
            "hardened_recovery_flow.record_kind_mismatch",
            &flow.flow_id,
            "record_kind must match the hardened recovery flow record kind",
        );
    }
    if flow.flow_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "hardened_recovery_flow.flow_id_empty",
            &flow.flow_id,
            "flow_id must be non-empty",
        );
    }
    if !flow.doctor_finding_ref.starts_with("doctor.finding.") {
        push_violation(
            &mut violations,
            "hardened_recovery_flow.doctor_finding_ref_missing",
            &flow.flow_id,
            "doctor_finding_ref must start with doctor.finding.",
        );
    }
    if flow.support_guidance.trim().is_empty() {
        push_violation(
            &mut violations,
            "hardened_recovery_flow.support_guidance_empty",
            &flow.flow_id,
            "support_guidance must be non-empty",
        );
    }
    if flow.flow_summary.trim().is_empty() {
        push_violation(
            &mut violations,
            "hardened_recovery_flow.flow_summary_empty",
            &flow.flow_id,
            "flow_summary must be non-empty",
        );
    }
    if flow.evidence_refs.is_empty() {
        push_violation(
            &mut violations,
            "hardened_recovery_flow.evidence_refs_empty",
            &flow.flow_id,
            "evidence_refs must contain at least one ref",
        );
    }

    let preserved: BTreeSet<HardenedRecoveryPreservedStateClass> =
        flow.preserved_state_classes.iter().copied().collect();

    if !preserved.contains(&HardenedRecoveryPreservedStateClass::UserAuthoredFiles) {
        push_violation(
            &mut violations,
            "hardened_recovery_flow.user_authored_files_not_preserved",
            &flow.flow_id,
            "every flow must preserve user_authored_files",
        );
    }

    // Family-specific checks
    match flow.flow_class {
        HardenedRecoveryFlowClass::CacheRebuild => {
            if flow.rung_class != RecoveryRungClass::CacheIndexRepair {
                push_violation(
                    &mut violations,
                    "hardened_recovery_flow.cache_rebuild_rung_mismatch",
                    &flow.flow_id,
                    "cache_rebuild must bind to CacheIndexRepair rung",
                );
            }
            if !matches!(
                flow.blast_radius_class,
                HardenedRecoveryBlastRadiusClass::SingleDisposableStateClass
            ) {
                push_violation(
                    &mut violations,
                    "hardened_recovery_flow.cache_rebuild_blast_radius_invalid",
                    &flow.flow_id,
                    "cache_rebuild blast radius must be single_disposable_state_class",
                );
            }
            if !matches!(
                flow.reversal_class,
                HardenedRecoveryReversalClass::Regenerate | HardenedRecoveryReversalClass::Exact
            ) {
                push_violation(
                    &mut violations,
                    "hardened_recovery_flow.cache_rebuild_reversal_invalid",
                    &flow.flow_id,
                    "cache_rebuild reversal must be regenerate or exact",
                );
            }
        }
        HardenedRecoveryFlowClass::SettingsRepair => {
            if flow.rung_class != RecoveryRungClass::SettingsRepair {
                push_violation(
                    &mut violations,
                    "hardened_recovery_flow.settings_repair_rung_mismatch",
                    &flow.flow_id,
                    "settings_repair must bind to SettingsRepair rung",
                );
            }
            if !matches!(
                flow.blast_radius_class,
                HardenedRecoveryBlastRadiusClass::SameFamilyStateClasses
                    | HardenedRecoveryBlastRadiusClass::SingleDisposableStateClass
            ) {
                push_violation(
                    &mut violations,
                    "hardened_recovery_flow.settings_repair_blast_radius_invalid",
                    &flow.flow_id,
                    "settings_repair blast radius must be same_family_state_classes or single_disposable_state_class",
                );
            }
            if !preserved.contains(&HardenedRecoveryPreservedStateClass::SettingsProfileState) {
                push_violation(
                    &mut violations,
                    "hardened_recovery_flow.settings_profile_not_preserved",
                    &flow.flow_id,
                    "settings_repair must preserve settings_profile_state",
                );
            }
            if flow.checkpoint_class != HardenedRecoveryCheckpointClass::DurablePreApply {
                push_violation(
                    &mut violations,
                    "hardened_recovery_flow.settings_repair_checkpoint_invalid",
                    &flow.flow_id,
                    "settings_repair requires a durable_pre_apply checkpoint",
                );
            }
        }
        HardenedRecoveryFlowClass::StateMigrationRepair => {
            if flow.rung_class != RecoveryRungClass::StateMigrationRepair {
                push_violation(
                    &mut violations,
                    "hardened_recovery_flow.state_migration_repair_rung_mismatch",
                    &flow.flow_id,
                    "state_migration_repair must bind to StateMigrationRepair rung",
                );
            }
            if !matches!(
                flow.blast_radius_class,
                HardenedRecoveryBlastRadiusClass::CrossFamilyStateClasses
                    | HardenedRecoveryBlastRadiusClass::EscalationOnly
            ) {
                push_violation(
                    &mut violations,
                    "hardened_recovery_flow.state_migration_repair_blast_radius_invalid",
                    &flow.flow_id,
                    "state_migration_repair blast radius must be cross_family_state_classes or escalation_only",
                );
            }
            if !preserved.contains(&HardenedRecoveryPreservedStateClass::DurableWorkspaceIndexes) {
                push_violation(
                    &mut violations,
                    "hardened_recovery_flow.durable_indexes_not_preserved",
                    &flow.flow_id,
                    "state_migration_repair must preserve durable_workspace_indexes",
                );
            }
            if !preserved.contains(&HardenedRecoveryPreservedStateClass::StateMigrationJournal) {
                push_violation(
                    &mut violations,
                    "hardened_recovery_flow.migration_journal_not_preserved",
                    &flow.flow_id,
                    "state_migration_repair must preserve state_migration_journal",
                );
            }
            if flow.checkpoint_class != HardenedRecoveryCheckpointClass::DurablePreApply {
                push_violation(
                    &mut violations,
                    "hardened_recovery_flow.state_migration_repair_checkpoint_invalid",
                    &flow.flow_id,
                    "state_migration_repair requires a durable_pre_apply checkpoint",
                );
            }
        }
        HardenedRecoveryFlowClass::TargetedReset => {
            if flow.rung_class != RecoveryRungClass::TargetedReset {
                push_violation(
                    &mut violations,
                    "hardened_recovery_flow.targeted_reset_rung_mismatch",
                    &flow.flow_id,
                    "targeted_reset must bind to TargetedReset rung",
                );
            }
            if flow.blast_radius_class != HardenedRecoveryBlastRadiusClass::SingleDisposableStateClass
            {
                push_violation(
                    &mut violations,
                    "hardened_recovery_flow.targeted_reset_blast_radius_invalid",
                    &flow.flow_id,
                    "targeted_reset blast radius must be single_disposable_state_class",
                );
            }
            if flow.impacted_state_classes.len() != 1 {
                push_violation(
                    &mut violations,
                    "hardened_recovery_flow.targeted_reset_impacted_state_count",
                    &flow.flow_id,
                    "targeted_reset must declare exactly one impacted_state_class",
                );
            }
            if flow.checkpoint_class != HardenedRecoveryCheckpointClass::DurablePreApply {
                push_violation(
                    &mut violations,
                    "hardened_recovery_flow.targeted_reset_checkpoint_invalid",
                    &flow.flow_id,
                    "targeted_reset requires a durable_pre_apply checkpoint",
                );
            }
        }
    }

    // Checkpoint / reversal consistency
    if flow.checkpoint_class.requires_checkpoint_ref() && flow.checkpoint_ref.is_none() {
        push_violation(
            &mut violations,
            "hardened_recovery_flow.checkpoint_ref_missing",
            &flow.flow_id,
            "checkpoint_class requires a non-empty checkpoint_ref",
        );
    }
    if flow.reversal_class.requires_checkpoint_ref() && flow.checkpoint_ref.is_none() {
        push_violation(
            &mut violations,
            "hardened_recovery_flow.reversal_requires_checkpoint_ref",
            &flow.flow_id,
            "reversal_class exact requires a checkpoint_ref",
        );
    }

    violations
}

fn push_violation(
    violations: &mut Vec<HardenedRecoveryFlowViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(HardenedRecoveryFlowViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}
