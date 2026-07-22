//! Synthetic rollback drill driver for install-topology state roots.
//!
//! The driver walks only caller-provided synthetic roots. It captures a
//! pre-rollback snapshot, applies a bounded fake update to the target install
//! roots, restores those roots from the snapshot, and compares every walked
//! root against the captured state while ignoring declared post-rollback
//! evidence deltas.

#[cfg(test)]
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs::{self, File, Metadata, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::topology::{
    ChannelClass, InstallModeClass, InstallTopologyAlphaPacket, InstallTopologyRow,
    InstallTopologyValidationFinding,
};

/// Schema version for rollback-drill records.
pub const ROLLBACK_DRILL_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`RollbackDrillPreStateSnapshot`].
pub const ROLLBACK_DRILL_PRE_STATE_RECORD_KIND: &str =
    "install_topology_rollback_pre_state_snapshot";

/// Stable record-kind tag for [`RollbackDrillReport`].
pub const ROLLBACK_DRILL_REPORT_RECORD_KIND: &str = "install_topology_rollback_drill_report";

/// Schema version for update rollback-plan records.
pub const UPDATE_ROLLBACK_PLAN_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`UpdateRollbackPlan`].
pub const UPDATE_ROLLBACK_PLAN_RECORD_KIND: &str = "update_rollback_plan_record";

/// Stable record-kind tag for [`UpdateRollbackSupportExport`].
pub const UPDATE_ROLLBACK_SUPPORT_EXPORT_RECORD_KIND: &str = "update_rollback_support_export";

/// Maximum number of roots admitted by one synthetic rollback drill.
pub const ROLLBACK_DRILL_MAX_ROOTS: usize = 64;

/// Maximum number of expected-delta rows admitted by one drill.
pub const ROLLBACK_DRILL_MAX_EXPECTED_DELTAS: usize = 256;

/// Maximum number of filesystem entries captured across all synthetic roots.
pub const ROLLBACK_DRILL_MAX_ENTRIES: usize = 4_096;

/// Maximum number of entries admitted in any one synthetic directory.
pub const ROLLBACK_DRILL_MAX_DIRECTORY_ENTRIES: usize = 1_024;

/// Maximum relative-path depth below a synthetic state root.
pub const ROLLBACK_DRILL_MAX_DEPTH: usize = 64;

/// Maximum UTF-8 byte length of a captured path relative to its state root.
pub const ROLLBACK_DRILL_MAX_RELATIVE_PATH_BYTES: usize = 2_048;

/// Maximum bytes captured from one regular synthetic file.
pub const ROLLBACK_DRILL_MAX_FILE_BYTES: u64 = 1_048_576;

/// Maximum aggregate regular-file bytes captured by one snapshot.
pub const ROLLBACK_DRILL_MAX_TOTAL_FILE_BYTES: u64 = 4_194_304;

/// Maximum serialized bytes accepted for a captured pre-state document.
pub const ROLLBACK_DRILL_MAX_SNAPSHOT_BYTES: u64 = 25_165_824;

const ROLLBACK_DRILL_MAX_JSON_NODES: usize = 65_536;
const ROLLBACK_DRILL_MAX_JSON_COLLECTION_ENTRIES: usize = 8_192;
const ROLLBACK_DRILL_MAX_TEXT_BYTES: usize = 262_144;
const ROLLBACK_DRILL_MAX_ID_BYTES: usize = 512;
const ROLLBACK_DRILL_MAX_FILESYSTEM_SEGMENT_BYTES: usize = 255;
const ROLLBACK_DRILL_PRE_STATE_SUFFIX: &str = ".pre_state.json";
const ROLLBACK_DRILL_MAX_DRILL_ID_BYTES: usize =
    ROLLBACK_DRILL_MAX_FILESYSTEM_SEGMENT_BYTES - ROLLBACK_DRILL_PRE_STATE_SUFFIX.len();
const SYNTHETIC_AUTHORITY_DIRECTORY: &str = ".rollback_drill";
const SYNTHETIC_AUTHORITY_MARKER: &str = "synthetic-authority-v1";
const SYNTHETIC_AUTHORITY_MARKER_BODY: &[u8] = b"aureline.synthetic.rollback.authority.v1\n";

#[cfg(test)]
std::thread_local! {
    static FAIL_ATOMIC_WRITE_AFTER_SYNC: Cell<bool> = const { Cell::new(false) };
    static FAIL_ATOMIC_WRITE_PARENT_SYNC: Cell<bool> = const { Cell::new(false) };
    static FAIL_SYNTHETIC_UPDATE_AFTER_FIRST_WRITE: Cell<bool> = const { Cell::new(false) };
    static FAIL_RESTORE_AFTER_QUARANTINE: Cell<bool> = const { Cell::new(false) };
}

const REQUIRED_ROLLBACK_ARTIFACT_FAMILIES: &[RollbackArtifactFamilyClass] = &[
    RollbackArtifactFamilyClass::IdeBinary,
    RollbackArtifactFamilyClass::CliBinary,
    RollbackArtifactFamilyClass::RemoteAgentTarball,
    RollbackArtifactFamilyClass::UpdateMetadata,
    RollbackArtifactFamilyClass::PolicyBundle,
    RollbackArtifactFamilyClass::SchemaExport,
    RollbackArtifactFamilyClass::DocsPack,
    RollbackArtifactFamilyClass::SupportRunbookBundle,
    RollbackArtifactFamilyClass::ReleaseEvidencePacket,
];

const REQUIRED_ROLLBACK_VOCABULARY_TERMS: &[&str] = &[
    "retained_prior_artifact_set",
    "schema_rollback_hook",
    "downgrade_eligibility_state",
    "exact_build_identity_ref",
];

/// Artifact family covered by a retained rollback atom.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackArtifactFamilyClass {
    /// Desktop shell binary.
    IdeBinary,
    /// Command-line binary.
    CliBinary,
    /// Remote agent tarball or image-layer bundle.
    RemoteAgentTarball,
    /// Signed update metadata and rollback target map.
    UpdateMetadata,
    /// Policy bundle required for the release family.
    PolicyBundle,
    /// Schema export required by support and migration readers.
    SchemaExport,
    /// Docs/help pack tied to the release family.
    DocsPack,
    /// Support runbook bundle.
    SupportRunbookBundle,
    /// Release evidence packet.
    ReleaseEvidencePacket,
    /// Debug symbols or source maps.
    DebugSidecar,
    /// SBOM or attestation sidecar.
    SupplyChainProof,
}

impl RollbackArtifactFamilyClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdeBinary => "ide_binary",
            Self::CliBinary => "cli_binary",
            Self::RemoteAgentTarball => "remote_agent_tarball",
            Self::UpdateMetadata => "update_metadata",
            Self::PolicyBundle => "policy_bundle",
            Self::SchemaExport => "schema_export",
            Self::DocsPack => "docs_pack",
            Self::SupportRunbookBundle => "support_runbook_bundle",
            Self::ReleaseEvidencePacket => "release_evidence_packet",
            Self::DebugSidecar => "debug_sidecar",
            Self::SupplyChainProof => "supply_chain_proof",
        }
    }
}

/// Retention state for a prior artifact needed by rollback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetainedArtifactState {
    /// Prior artifact is retained as an exact-build artifact.
    RetainedExactBuild,
    /// Only metadata is retained; not enough for automatic rollback.
    RetainedMetadataOnly,
    /// Artifact is absent and blocks rollback.
    MissingBlocked,
    /// Artifact retention is expired and requires manual reconstruction.
    ExpiredManualReview,
}

impl RetainedArtifactState {
    /// Returns true when the retained artifact can be used by an automatic rollback.
    pub const fn is_exact_build_retained(self) -> bool {
        matches!(self, Self::RetainedExactBuild)
    }
}

/// Signature or trust state for a retained prior artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetainedArtifactVerificationState {
    /// Signature and digest were verified for the retained artifact.
    Verified,
    /// Verification ref exists but was not checked by this packet.
    PresentUnverified,
    /// Artifact was revoked and cannot be a rollback target.
    Revoked,
    /// Verification is missing and blocks automatic rollback.
    MissingBlocked,
}

impl RetainedArtifactVerificationState {
    /// Returns true when the artifact is trusted enough for automatic rollback.
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::Verified)
    }
}

/// Compatibility class for a schema rollback hook.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaRollbackCompatibilityClass {
    /// Target can read the current schema without transformation.
    BackwardReadable,
    /// Additive migration can be reversed without data loss.
    AdditiveCompatible,
    /// Repair/export path is required before the target can read the state.
    RepairRequired,
    /// Unknown compatibility requires manual review before rollback.
    UnknownManualReview,
    /// Destructive state clear would be required and is blocked.
    DestructiveBlocked,
}

impl SchemaRollbackCompatibilityClass {
    /// Returns true when the compatibility state can run without manual review.
    pub const fn automatic_allowed(self) -> bool {
        matches!(self, Self::BackwardReadable | Self::AdditiveCompatible)
    }
}

/// Flow class allowed to invoke a schema rollback hook.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackReviewedFlowClass {
    /// Interactive update center review.
    UpdateCenterReview,
    /// Headless CI or dry-run review.
    HeadlessReview,
    /// Managed fleet admin review.
    ManagedFleetReview,
    /// Support-assisted recovery review.
    SupportAssistedReview,
    /// Migration center restore or rollback review.
    MigrationCenterReview,
}

/// Runtime state of a schema rollback hook.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaRollbackHookState {
    /// Hook is available and bound to a reviewed flow.
    ReviewedFlowReady,
    /// Hook was invoked by the named reviewed checkpoint.
    InvokedThroughReviewedFlow,
    /// Hook is visible but may only be used after manual review.
    ManualReviewOnly,
    /// Hook is blocked and cannot be used for the plan.
    Blocked,
}

impl SchemaRollbackHookState {
    /// Returns true when the hook is usable in a reviewed rollback flow.
    pub const fn usable(self) -> bool {
        matches!(
            self,
            Self::ReviewedFlowReady | Self::InvokedThroughReviewedFlow | Self::ManualReviewOnly
        )
    }
}

/// Downgrade eligibility state for a rollback plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeEligibilityState {
    /// All checks passed and policy permits automatic downgrade.
    AutoEligible,
    /// Checks passed but user or admin review is required.
    EligibleWithReview,
    /// Evidence is incomplete and manual review must choose repair/export/abort.
    ManualReviewRequired,
    /// Downgrade is blocked by trust, state, policy, helper skew, or missing artifacts.
    Blocked,
    /// Target is outside the supported downgrade window.
    Unsupported,
}

impl DowngradeEligibilityState {
    /// Returns true when the rollback plan may proceed after the required review.
    pub const fn may_proceed(self) -> bool {
        matches!(
            self,
            Self::AutoEligible | Self::EligibleWithReview | Self::ManualReviewRequired
        )
    }
}

/// Upstream references a rollback plan consumes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateRollbackSourceRefs {
    /// Release artifact graph that owns current artifact relationships.
    pub artifact_graph_ref: String,
    /// Update manifest for the attempted update.
    pub update_manifest_ref: String,
    /// Update-ready review emitted before mutation.
    pub update_ready_review_ref: String,
    /// Update sequence packet that owns checkpoint ids.
    pub update_sequence_ref: String,
    /// Install diagnostics packet that owns state-root ids.
    pub install_diagnostics_ref: String,
    /// Ring rollout packet that owns prior/candidate visibility.
    pub ring_rollout_ref: String,
    /// Compatibility report containing downgrade and skew evidence.
    pub compatibility_report_ref: String,
}

/// Current or rollback-target build identity in an update rollback plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackBuildRef {
    /// Release candidate ref.
    pub release_candidate_ref: String,
    /// Exact-build identity ref.
    pub exact_build_identity_ref: String,
    /// Update manifest ref for this build.
    pub update_manifest_ref: String,
    /// Artifact bundle ref for this build.
    pub artifact_bundle_ref: String,
    /// Human-readable version label.
    pub version: String,
    /// Release channel class.
    pub channel_class: ChannelClass,
}

/// Retained prior artifact used by a rollback target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetainedPriorArtifact {
    /// Stable artifact ref.
    pub artifact_ref: String,
    /// Artifact family class.
    pub family_class: RollbackArtifactFamilyClass,
    /// Exact-build identity ref of the retained prior artifact.
    pub exact_build_identity_ref: String,
    /// Prior release candidate ref this artifact belongs to.
    pub prior_release_candidate_ref: String,
    /// Digest or content-address ref for the retained artifact.
    pub digest_ref: String,
    /// Signature or trust state for the retained artifact.
    pub verification_state: RetainedArtifactVerificationState,
    /// Retention state for the artifact bytes or metadata.
    pub retention_state: RetainedArtifactState,
    /// Retention owner or policy ref.
    pub retention_owner_ref: String,
    /// Support projection ref for this artifact.
    pub support_ref: String,
    /// True when this artifact participates in the coordinated rollback atom.
    pub rollback_atom_member: bool,
    /// Short caveat surfaced to update center and support.
    pub caveat: String,
}

/// Schema/state rollback hook admitted by the rollback plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaRollbackHook {
    /// Stable hook id.
    pub hook_id: String,
    /// Durable state-root ref the hook covers.
    pub state_root_ref: String,
    /// Schema epoch the failed update wrote or attempted to write.
    pub source_schema_epoch: String,
    /// Schema epoch the rollback target must read.
    pub target_schema_epoch: String,
    /// Compatibility class for the rollback.
    pub compatibility_class: SchemaRollbackCompatibilityClass,
    /// Flow class that is allowed to invoke this hook.
    pub reviewed_flow_class: RollbackReviewedFlowClass,
    /// Stable review or approval ref that admitted the hook.
    pub reviewed_flow_ref: String,
    /// Update sequence checkpoint that invoked or will invoke the hook.
    pub invoked_checkpoint_id: String,
    /// Hook state.
    pub hook_state: SchemaRollbackHookState,
    /// Backup snapshot ref required by the hook.
    pub backup_snapshot_ref: String,
    /// Migration journal ref required by the hook.
    pub migration_journal_ref: String,
    /// Repair transaction ref when compatibility requires repair.
    pub repair_transaction_ref: Option<String>,
    /// Reviewer-facing caveat.
    pub caveat: String,
}

/// Explicit downgrade truth carried by update, docs, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradeTruth {
    /// Downgrade eligibility state.
    pub eligibility_state: DowngradeEligibilityState,
    /// Current source build ref.
    pub source_build_ref: String,
    /// Target rollback build ref.
    pub target_build_ref: String,
    /// Migration or downgrade caveats that must be shown verbatim.
    pub migration_caveats: Vec<String>,
    /// Manual review reason classes, when review is required.
    pub manual_review_reason_classes: Vec<String>,
    /// Blocked reason classes, when rollback cannot proceed.
    pub blocked_reason_classes: Vec<String>,
    /// State roots preserved by rollback.
    pub preserved_state_root_refs: Vec<String>,
    /// State roots intentionally not restored by rollback.
    pub not_restored_state_root_refs: Vec<String>,
    /// Support-safe summary.
    pub support_summary: String,
}

/// Support projection settings embedded in the rollback plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackPlanSupportProjection {
    /// Support export projection path.
    pub support_projection_ref: String,
    /// Support bundle refs that quote this plan.
    pub support_bundle_refs: Vec<String>,
    /// Product, docs, and Help surfaces that must reuse the plan vocabulary.
    pub consuming_surface_refs: Vec<String>,
    /// Shared vocabulary terms required across surfaces.
    pub vocabulary_terms: Vec<String>,
    /// Redaction posture for the projection.
    pub redaction_class: String,
}

/// Acceptance evidence for a rollback plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackPlanAcceptance {
    /// Commands that validate the plan.
    pub validation_commands: Vec<String>,
    /// Fixture manifest ref for the plan.
    pub fixture_manifest_ref: String,
    /// Accepted evidence states.
    pub accepted_states: Vec<String>,
}

/// Governed beta update rollback plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateRollbackPlan {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Plan schema version.
    pub schema_version: u32,
    /// Stable plan id.
    pub plan_id: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Upstream source refs.
    pub source_refs: UpdateRollbackSourceRefs,
    /// Current build that failed or may fail after update.
    pub current_build: RollbackBuildRef,
    /// Prior build retained as rollback target.
    pub rollback_target: RollbackBuildRef,
    /// Prior artifacts retained for exact-build rollback.
    pub retained_prior_artifacts: Vec<RetainedPriorArtifact>,
    /// Schema/state rollback hooks admitted by reviewed flows.
    pub schema_rollback_hooks: Vec<SchemaRollbackHook>,
    /// Explicit downgrade and migration caveat truth.
    pub downgrade_truth: DowngradeTruth,
    /// Support projection contract for this plan.
    pub support_projection: RollbackPlanSupportProjection,
    /// Acceptance evidence for validators and release review.
    pub acceptance: RollbackPlanAcceptance,
}

impl UpdateRollbackPlan {
    /// Validates the rollback plan.
    pub fn validate(&self) -> UpdateRollbackValidationReport {
        let mut findings = Vec::new();

        if self.record_kind != UPDATE_ROLLBACK_PLAN_RECORD_KIND {
            push_plan_finding(
                &mut findings,
                "rollback_plan.record_kind",
                &self.plan_id,
                "record_kind must be update_rollback_plan_record",
            );
        }
        if self.schema_version != UPDATE_ROLLBACK_PLAN_SCHEMA_VERSION {
            push_plan_finding(
                &mut findings,
                "rollback_plan.schema_version",
                &self.plan_id,
                "schema_version must be 1",
            );
        }
        validate_non_empty_ref(&mut findings, "plan_id", &self.plan_id, &self.plan_id);
        validate_non_empty_ref(
            &mut findings,
            "current_build.release_candidate_ref",
            &self.current_build.release_candidate_ref,
            &self.plan_id,
        );
        validate_exact_build_ref(
            &mut findings,
            "current_build.exact_build_identity_ref",
            &self.current_build.exact_build_identity_ref,
            &self.plan_id,
        );
        validate_exact_build_ref(
            &mut findings,
            "rollback_target.exact_build_identity_ref",
            &self.rollback_target.exact_build_identity_ref,
            &self.plan_id,
        );
        if self.current_build.exact_build_identity_ref
            == self.rollback_target.exact_build_identity_ref
        {
            push_plan_finding(
                &mut findings,
                "rollback_plan.same_current_and_target_exact_build",
                &self.plan_id,
                "current and rollback target exact-build refs must differ",
            );
        }
        if self.current_build.release_candidate_ref == self.rollback_target.release_candidate_ref {
            push_plan_finding(
                &mut findings,
                "rollback_plan.same_current_and_target_candidate",
                &self.plan_id,
                "current and rollback target release candidates must differ",
            );
        }

        self.validate_retained_artifacts(&mut findings);
        self.validate_schema_hooks(&mut findings);
        self.validate_downgrade_truth(&mut findings);
        self.validate_support_projection(&mut findings);

        UpdateRollbackValidationReport {
            record_kind: "update_rollback_validation_report".to_string(),
            schema_version: UPDATE_ROLLBACK_PLAN_SCHEMA_VERSION,
            plan_id: self.plan_id.clone(),
            passed: findings.is_empty(),
            coverage: self.coverage(),
            findings,
        }
    }

    /// Builds the support-export projection from the plan.
    pub fn support_export_projection(&self) -> UpdateRollbackSupportExport {
        UpdateRollbackSupportExport {
            record_kind: UPDATE_ROLLBACK_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: UPDATE_ROLLBACK_PLAN_SCHEMA_VERSION,
            plan_id: self.plan_id.clone(),
            generated_at: self.generated_at.clone(),
            source_plan_ref: "artifacts/release/m3/update_rollback/rollback_plan.json".to_string(),
            current_release_candidate_ref: self.current_build.release_candidate_ref.clone(),
            current_exact_build_identity_ref: self.current_build.exact_build_identity_ref.clone(),
            rollback_target_ref: self.rollback_target.release_candidate_ref.clone(),
            rollback_exact_build_identity_ref: self
                .rollback_target
                .exact_build_identity_ref
                .clone(),
            downgrade_eligibility_state: self.downgrade_truth.eligibility_state,
            migration_caveats: self.downgrade_truth.migration_caveats.clone(),
            retained_artifacts: self
                .retained_prior_artifacts
                .iter()
                .map(|artifact| UpdateRollbackSupportArtifactRow {
                    artifact_ref: artifact.artifact_ref.clone(),
                    family_class: artifact.family_class,
                    exact_build_identity_ref: artifact.exact_build_identity_ref.clone(),
                    retention_state: artifact.retention_state,
                    verification_state: artifact.verification_state,
                    rollback_atom_member: artifact.rollback_atom_member,
                    support_ref: artifact.support_ref.clone(),
                    caveat: artifact.caveat.clone(),
                })
                .collect(),
            schema_hooks: self
                .schema_rollback_hooks
                .iter()
                .map(|hook| UpdateRollbackSupportHookRow {
                    hook_id: hook.hook_id.clone(),
                    state_root_ref: hook.state_root_ref.clone(),
                    compatibility_class: hook.compatibility_class,
                    reviewed_flow_class: hook.reviewed_flow_class,
                    invoked_checkpoint_id: hook.invoked_checkpoint_id.clone(),
                    hook_state: hook.hook_state,
                    caveat: hook.caveat.clone(),
                })
                .collect(),
            support_bundle_refs: self.support_projection.support_bundle_refs.clone(),
            vocabulary_terms: self.support_projection.vocabulary_terms.clone(),
            redaction_class: self.support_projection.redaction_class.clone(),
        }
    }

    fn validate_retained_artifacts(&self, findings: &mut Vec<UpdateRollbackValidationFinding>) {
        if self.retained_prior_artifacts.is_empty() {
            push_plan_finding(
                findings,
                "retained_artifacts.empty",
                &self.plan_id,
                "rollback plan must retain at least one prior artifact",
            );
            return;
        }

        let mut seen_refs = BTreeSet::new();
        let mut families = BTreeSet::new();
        for artifact in &self.retained_prior_artifacts {
            validate_non_empty_ref(
                findings,
                "retained_artifacts.artifact_ref",
                &artifact.artifact_ref,
                &self.plan_id,
            );
            if !seen_refs.insert(artifact.artifact_ref.as_str()) {
                push_plan_finding(
                    findings,
                    "retained_artifacts.duplicate_artifact_ref",
                    &artifact.artifact_ref,
                    "retained artifact refs must be unique",
                );
            }
            families.insert(artifact.family_class);
            if artifact.exact_build_identity_ref != self.rollback_target.exact_build_identity_ref {
                push_plan_finding(
                    findings,
                    "retained_artifacts.exact_build_mismatch",
                    &artifact.artifact_ref,
                    "retained prior artifacts must use the rollback target exact-build ref",
                );
            }
            if artifact.prior_release_candidate_ref != self.rollback_target.release_candidate_ref {
                push_plan_finding(
                    findings,
                    "retained_artifacts.release_candidate_mismatch",
                    &artifact.artifact_ref,
                    "retained prior artifacts must belong to the rollback target candidate",
                );
            }
            if !artifact.retention_state.is_exact_build_retained() {
                push_plan_finding(
                    findings,
                    "retained_artifacts.not_exact_build_retained",
                    &artifact.artifact_ref,
                    "artifact bytes must be retained as an exact-build rollback artifact",
                );
            }
            if !artifact.verification_state.is_verified() {
                push_plan_finding(
                    findings,
                    "retained_artifacts.not_verified",
                    &artifact.artifact_ref,
                    "retained prior artifacts must have verified digest/signature state",
                );
            }
            if !artifact.rollback_atom_member {
                push_plan_finding(
                    findings,
                    "retained_artifacts.not_in_rollback_atom",
                    &artifact.artifact_ref,
                    "retained artifact must be part of the coordinated rollback atom",
                );
            }
        }

        for required in REQUIRED_ROLLBACK_ARTIFACT_FAMILIES {
            if !families.contains(required) {
                push_plan_finding(
                    findings,
                    "retained_artifacts.required_family_missing",
                    required.as_str(),
                    "rollback plan must retain every required prior artifact family",
                );
            }
        }
    }

    fn validate_schema_hooks(&self, findings: &mut Vec<UpdateRollbackValidationFinding>) {
        if self.schema_rollback_hooks.is_empty() {
            push_plan_finding(
                findings,
                "schema_hooks.empty",
                &self.plan_id,
                "rollback plan must declare schema rollback hooks",
            );
            return;
        }

        let mut seen_hooks = BTreeSet::new();
        for hook in &self.schema_rollback_hooks {
            validate_non_empty_ref(
                findings,
                "schema_hooks.hook_id",
                &hook.hook_id,
                &self.plan_id,
            );
            if !seen_hooks.insert(hook.hook_id.as_str()) {
                push_plan_finding(
                    findings,
                    "schema_hooks.duplicate_hook_id",
                    &hook.hook_id,
                    "schema rollback hook ids must be unique",
                );
            }
            validate_non_empty_ref(
                findings,
                "schema_hooks.reviewed_flow_ref",
                &hook.reviewed_flow_ref,
                &hook.hook_id,
            );
            validate_non_empty_ref(
                findings,
                "schema_hooks.backup_snapshot_ref",
                &hook.backup_snapshot_ref,
                &hook.hook_id,
            );
            validate_non_empty_ref(
                findings,
                "schema_hooks.migration_journal_ref",
                &hook.migration_journal_ref,
                &hook.hook_id,
            );
            if !hook.invoked_checkpoint_id.starts_with("checkpoint.update.") {
                push_plan_finding(
                    findings,
                    "schema_hooks.invoked_checkpoint_not_update_sequence",
                    &hook.hook_id,
                    "schema rollback hooks must bind to update sequence checkpoint ids",
                );
            }
            if !hook.hook_state.usable() {
                push_plan_finding(
                    findings,
                    "schema_hooks.blocked",
                    &hook.hook_id,
                    "blocked schema hooks cannot be part of an admitted rollback plan",
                );
            }
            if !hook.compatibility_class.automatic_allowed()
                && hook.repair_transaction_ref.is_none()
                && self.downgrade_truth.eligibility_state == DowngradeEligibilityState::AutoEligible
            {
                push_plan_finding(
                    findings,
                    "schema_hooks.repair_ref_missing",
                    &hook.hook_id,
                    "non-automatic schema compatibility must carry repair evidence or downgrade truth must require review",
                );
            }
        }
    }

    fn validate_downgrade_truth(&self, findings: &mut Vec<UpdateRollbackValidationFinding>) {
        if self.downgrade_truth.source_build_ref != self.current_build.release_candidate_ref {
            push_plan_finding(
                findings,
                "downgrade_truth.source_build_ref_mismatch",
                &self.plan_id,
                "downgrade source_build_ref must match the current release candidate",
            );
        }
        if self.downgrade_truth.target_build_ref != self.rollback_target.release_candidate_ref {
            push_plan_finding(
                findings,
                "downgrade_truth.target_build_ref_mismatch",
                &self.plan_id,
                "downgrade target_build_ref must match the rollback target candidate",
            );
        }
        if !self.downgrade_truth.eligibility_state.may_proceed() {
            push_plan_finding(
                findings,
                "downgrade_truth.not_admitted",
                &self.plan_id,
                "blocked or unsupported downgrade truth cannot back a beta rollback guarantee",
            );
        }
        if self.downgrade_truth.migration_caveats.is_empty() {
            push_plan_finding(
                findings,
                "downgrade_truth.caveats_missing",
                &self.plan_id,
                "rollback plan must expose explicit downgrade or migration caveats",
            );
        }
        if self.downgrade_truth.eligibility_state == DowngradeEligibilityState::AutoEligible
            && (!self.downgrade_truth.manual_review_reason_classes.is_empty()
                || !self.downgrade_truth.blocked_reason_classes.is_empty())
        {
            push_plan_finding(
                findings,
                "downgrade_truth.auto_with_review_or_block_reasons",
                &self.plan_id,
                "auto-eligible downgrade truth must not carry manual-review or blocked reason classes",
            );
        }
        if self.downgrade_truth.preserved_state_root_refs.is_empty() {
            push_plan_finding(
                findings,
                "downgrade_truth.preserved_state_roots_missing",
                &self.plan_id,
                "downgrade truth must name preserved state roots",
            );
        }
    }

    fn validate_support_projection(&self, findings: &mut Vec<UpdateRollbackValidationFinding>) {
        validate_non_empty_ref(
            findings,
            "support_projection.support_projection_ref",
            &self.support_projection.support_projection_ref,
            &self.plan_id,
        );
        if self.support_projection.support_bundle_refs.is_empty() {
            push_plan_finding(
                findings,
                "support_projection.support_bundle_refs_missing",
                &self.plan_id,
                "rollback plan must project into at least one support bundle ref",
            );
        }
        if self.support_projection.consuming_surface_refs.is_empty() {
            push_plan_finding(
                findings,
                "support_projection.consuming_surface_refs_missing",
                &self.plan_id,
                "rollback plan must name consuming docs/help/support surfaces",
            );
        }
        let terms = self
            .support_projection
            .vocabulary_terms
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for required in REQUIRED_ROLLBACK_VOCABULARY_TERMS {
            if !terms.contains(required) {
                push_plan_finding(
                    findings,
                    "support_projection.required_vocabulary_missing",
                    required,
                    "support projection must carry the shared rollback vocabulary",
                );
            }
        }
    }

    fn coverage(&self) -> UpdateRollbackCoverage {
        UpdateRollbackCoverage {
            retained_artifact_families: self
                .retained_prior_artifacts
                .iter()
                .map(|artifact| artifact.family_class)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
            schema_hook_count: self.schema_rollback_hooks.len(),
            downgrade_eligibility_state: self.downgrade_truth.eligibility_state,
            support_surface_count: self.support_projection.consuming_surface_refs.len(),
        }
    }
}

/// Validation report for [`UpdateRollbackPlan`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateRollbackValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Validation schema version.
    pub schema_version: u32,
    /// Plan id that was validated.
    pub plan_id: String,
    /// True when no findings were produced.
    pub passed: bool,
    /// Validation coverage summary.
    pub coverage: UpdateRollbackCoverage,
    /// Validation findings.
    pub findings: Vec<UpdateRollbackValidationFinding>,
}

/// Coverage summary for update rollback validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateRollbackCoverage {
    /// Retained artifact family classes found in the plan.
    pub retained_artifact_families: Vec<RollbackArtifactFamilyClass>,
    /// Number of schema rollback hooks.
    pub schema_hook_count: usize,
    /// Downgrade eligibility state.
    pub downgrade_eligibility_state: DowngradeEligibilityState,
    /// Count of consuming support/docs/help surfaces.
    pub support_surface_count: usize,
}

/// One validation finding for update rollback plans.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateRollbackValidationFinding {
    /// Stable check id.
    pub check_id: String,
    /// Record or ref that failed.
    pub ref_id: String,
    /// Reviewer-facing message.
    pub message: String,
}

/// Support-export projection for an update rollback plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateRollbackSupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Projection schema version.
    pub schema_version: u32,
    /// Plan id projected into support export.
    pub plan_id: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Repository-relative source plan ref.
    pub source_plan_ref: String,
    /// Current release candidate ref.
    pub current_release_candidate_ref: String,
    /// Current exact-build identity ref.
    pub current_exact_build_identity_ref: String,
    /// Rollback target release candidate ref.
    pub rollback_target_ref: String,
    /// Rollback target exact-build identity ref.
    pub rollback_exact_build_identity_ref: String,
    /// Downgrade eligibility state.
    pub downgrade_eligibility_state: DowngradeEligibilityState,
    /// Downgrade and migration caveats shown in support export.
    pub migration_caveats: Vec<String>,
    /// Retained prior artifact rows.
    pub retained_artifacts: Vec<UpdateRollbackSupportArtifactRow>,
    /// Schema rollback hook rows.
    pub schema_hooks: Vec<UpdateRollbackSupportHookRow>,
    /// Support bundle refs that quote the projection.
    pub support_bundle_refs: Vec<String>,
    /// Shared rollback vocabulary terms.
    pub vocabulary_terms: Vec<String>,
    /// Redaction posture for support export.
    pub redaction_class: String,
}

/// Support-export row for one retained prior artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateRollbackSupportArtifactRow {
    /// Stable artifact ref.
    pub artifact_ref: String,
    /// Artifact family class.
    pub family_class: RollbackArtifactFamilyClass,
    /// Exact-build identity ref.
    pub exact_build_identity_ref: String,
    /// Retention state.
    pub retention_state: RetainedArtifactState,
    /// Verification state.
    pub verification_state: RetainedArtifactVerificationState,
    /// True when this artifact is in the coordinated rollback atom.
    pub rollback_atom_member: bool,
    /// Support projection ref for this artifact.
    pub support_ref: String,
    /// Support-safe caveat.
    pub caveat: String,
}

/// Support-export row for one schema rollback hook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateRollbackSupportHookRow {
    /// Stable hook id.
    pub hook_id: String,
    /// Durable state-root ref.
    pub state_root_ref: String,
    /// Compatibility class.
    pub compatibility_class: SchemaRollbackCompatibilityClass,
    /// Reviewed flow class.
    pub reviewed_flow_class: RollbackReviewedFlowClass,
    /// Update sequence checkpoint id.
    pub invoked_checkpoint_id: String,
    /// Hook state.
    pub hook_state: SchemaRollbackHookState,
    /// Support-safe caveat.
    pub caveat: String,
}

/// Role a state root plays in the rollback drill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackDrillRootRole {
    /// Root restored from the captured pre-state snapshot.
    TargetRollback,
    /// Installed side-by-side peer root that must remain untouched.
    SideBySidePeer,
    /// Portable colocated root that must remain isolated and untouched.
    PortableStateRoot,
}

/// Expected post-rollback delta class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackDrillDeltaClass {
    /// Evidence emitted after rollback validation completes.
    PostRollbackEvidence,
    /// Local health probe output that is intentionally not restored.
    RuntimeHealthProbe,
}

/// Filesystem entry kind captured in a rollback snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackDrillEntryKind {
    /// Directory entry.
    Directory,
    /// Regular file entry.
    File,
}

/// Difference class emitted when post-state does not match pre-state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackDrillDiffKind {
    /// Entry existed before rollback but is absent afterwards.
    MissingAfterRollback,
    /// Entry did not exist before rollback but exists afterwards.
    UnexpectedAfterRollback,
    /// Entry kind changed between pre-state and post-state.
    EntryKindChanged,
    /// File contents changed between pre-state and post-state.
    ContentsChanged,
}

/// One durable state root included in a rollback drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDrillRoot {
    /// Durable state-root ref from the install-topology packet.
    pub root_ref: String,
    /// Role this root plays in the drill.
    pub role: RollbackDrillRootRole,
    /// Install-topology row that owns this root.
    pub topology_row_id: String,
    /// Channel class that owns this root.
    pub channel_class: ChannelClass,
}

/// Expected delta ignored during post-rollback comparison.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDrillExpectedDelta {
    /// Durable state-root ref containing the delta.
    pub root_ref: String,
    /// Slash-separated path relative to the durable state root.
    pub relative_path: String,
    /// Reason the delta is expected.
    pub delta_class: RollbackDrillDeltaClass,
}

/// Rollback drill plan derived from install-topology truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDrillPlan {
    /// Stable drill id.
    pub drill_id: String,
    /// Install-topology row restored by the rollback drill.
    pub target_topology_row_id: String,
    /// Durable state roots walked by the drill.
    pub roots: Vec<RollbackDrillRoot>,
    /// Post-rollback evidence paths ignored during state comparison.
    pub expected_deltas: Vec<RollbackDrillExpectedDelta>,
}

impl RollbackDrillPlan {
    /// Builds a portable plus side-by-side rollback drill plan from topology truth.
    ///
    /// # Errors
    ///
    /// Returns [`RollbackDrillError`] when the topology packet does not
    /// validate, the requested rows are missing, or the selected rows do not
    /// model a rollback-capable side-by-side target with an isolated portable
    /// state root.
    pub fn portable_side_by_side(
        topology: &InstallTopologyAlphaPacket,
        target_topology_row_id: &str,
        portable_topology_row_id: &str,
    ) -> Result<Self, RollbackDrillError> {
        let validation = topology.validate();
        if !validation.passed {
            return Err(RollbackDrillError::TopologyPacketInvalid {
                findings: validation.findings,
            });
        }

        let target = topology.row_by_id(target_topology_row_id).ok_or_else(|| {
            RollbackDrillError::MissingTopologyRow {
                topology_row_id: target_topology_row_id.to_string(),
            }
        })?;
        if !target.is_side_by_side() {
            return Err(RollbackDrillError::InvalidDrillPlan {
                detail: format!(
                    "target row {} does not claim side-by-side behavior",
                    target.topology_row_id
                ),
            });
        }
        if !target.rollback_posture.rollback_available {
            return Err(RollbackDrillError::InvalidDrillPlan {
                detail: format!(
                    "target row {} does not claim rollback availability",
                    target.topology_row_id
                ),
            });
        }
        let paired_channel =
            target
                .paired_channel_class
                .ok_or_else(|| RollbackDrillError::InvalidDrillPlan {
                    detail: format!(
                        "target row {} is missing paired channel truth",
                        target.topology_row_id
                    ),
                })?;
        let peer = find_side_by_side_peer(topology, target, paired_channel)?;

        let portable = topology
            .row_by_id(portable_topology_row_id)
            .ok_or_else(|| RollbackDrillError::MissingTopologyRow {
                topology_row_id: portable_topology_row_id.to_string(),
            })?;
        if portable.install_mode_class != InstallModeClass::Portable {
            return Err(RollbackDrillError::InvalidDrillPlan {
                detail: format!(
                    "portable row {} is not an install-mode portable row",
                    portable.topology_row_id
                ),
            });
        }
        if !portable
            .durable_state_root_refs
            .iter()
            .any(|root| root.contains("portable_colocated_root"))
        {
            return Err(RollbackDrillError::InvalidDrillPlan {
                detail: format!(
                    "portable row {} does not expose a portable colocated root",
                    portable.topology_row_id
                ),
            });
        }

        let mut roots = Vec::new();
        extend_roots(&mut roots, target, RollbackDrillRootRole::TargetRollback);
        extend_roots(&mut roots, peer, RollbackDrillRootRole::SideBySidePeer);
        extend_roots(
            &mut roots,
            portable,
            RollbackDrillRootRole::PortableStateRoot,
        );
        reject_duplicate_root_roles(&roots)?;

        let evidence_root = roots
            .iter()
            .filter(|root| root.role == RollbackDrillRootRole::TargetRollback)
            .find(|root| root.root_ref.contains("recovery_root"))
            .or_else(|| {
                roots
                    .iter()
                    .find(|root| root.role == RollbackDrillRootRole::TargetRollback)
            })
            .map(|root| root.root_ref.clone())
            .ok_or_else(|| RollbackDrillError::InvalidDrillPlan {
                detail: "drill plan has no target rollback roots".to_string(),
            })?;

        Ok(Self {
            drill_id: format!(
                "install.rollback.drill.{}",
                sanitize_id(target_topology_row_id)
            ),
            target_topology_row_id: target.topology_row_id.clone(),
            roots,
            expected_deltas: vec![RollbackDrillExpectedDelta {
                root_ref: evidence_root,
                relative_path: "rollback-evidence/post-rollback.json".to_string(),
                delta_class: RollbackDrillDeltaClass::PostRollbackEvidence,
            }],
        })
    }

    /// Returns target root refs restored by the drill.
    pub fn target_root_refs(&self) -> Vec<&str> {
        self.roots
            .iter()
            .filter(|root| root.role == RollbackDrillRootRole::TargetRollback)
            .map(|root| root.root_ref.as_str())
            .collect()
    }
}

/// One captured filesystem entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDrillEntry {
    /// Durable state-root ref containing the entry.
    pub root_ref: String,
    /// Slash-separated path relative to the durable state root.
    pub relative_path: String,
    /// Captured entry kind.
    pub entry_kind: RollbackDrillEntryKind,
    /// File bytes for regular files.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contents: Vec<u8>,
}

/// Pre-rollback state snapshot used to restore target roots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDrillPreStateSnapshot {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Snapshot schema version.
    pub schema_version: u32,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Drill plan id that produced the snapshot.
    pub drill_id: String,
    /// Install-topology row restored by the snapshot.
    pub target_topology_row_id: String,
    /// Durable state roots included in the snapshot.
    pub roots: Vec<RollbackDrillRoot>,
    /// Captured entries under all walked roots.
    pub entries: Vec<RollbackDrillEntry>,
    /// Integrity digest over plan identity, roots, and entries.
    pub entry_digest: String,
    /// Redaction-safe capture timestamp.
    pub captured_at: String,
}

/// One post-rollback state difference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDrillDiff {
    /// Durable state-root ref containing the difference.
    pub root_ref: String,
    /// Slash-separated path relative to the durable state root.
    pub relative_path: String,
    /// Difference class.
    pub diff_kind: RollbackDrillDiffKind,
}

/// Filesystem path for one synthetic state root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDrillRootPath {
    /// Durable state-root ref.
    pub root_ref: String,
    /// Synthetic path for the root.
    pub path: PathBuf,
}

/// Synthetic filesystem layout materialized for a drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDrillLayout {
    /// Root paths created for the drill.
    pub roots: Vec<RollbackDrillRootPath>,
}

/// Rollback drill result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDrillReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Report schema version.
    pub schema_version: u32,
    /// Drill plan id.
    pub drill_id: String,
    /// Snapshot id used for rollback.
    pub pre_state_snapshot_id: String,
    /// True when pre-state contained at least one captured entry.
    pub pre_state_captured: bool,
    /// True when target roots matched their pre-state after rollback.
    pub target_rolled_back: bool,
    /// Number of entries in the pre-state snapshot.
    pub pre_state_entry_count: usize,
    /// Number of entries in the post-rollback snapshot.
    pub post_state_entry_count: usize,
    /// Number of declared expected deltas ignored during comparison.
    pub expected_delta_count: usize,
    /// Durable state-root refs compared by the drill.
    pub compared_root_refs: Vec<String>,
    /// Differences found after rollback.
    pub diffs: Vec<RollbackDrillDiff>,
}

/// Errors returned while running the rollback drill.
#[derive(Debug, PartialEq, Eq)]
pub enum RollbackDrillError {
    /// The install-topology packet failed validation.
    TopologyPacketInvalid {
        /// Validation findings from the topology packet.
        findings: Vec<InstallTopologyValidationFinding>,
    },
    /// A requested install-topology row was not present.
    MissingTopologyRow {
        /// Missing topology row id.
        topology_row_id: String,
    },
    /// The selected rows cannot form a rollback drill.
    InvalidDrillPlan {
        /// Redaction-safe failure detail.
        detail: String,
    },
    /// A state-root ref cannot be mapped into the synthetic tree.
    UnsafeStateRoot {
        /// Unsafe state-root ref.
        root_ref: String,
        /// Redaction-safe failure detail.
        detail: String,
    },
    /// A planned state root was missing from the synthetic tree.
    MissingStateRoot {
        /// Durable state-root ref.
        root_ref: String,
        /// Redacted logical path class. Host paths are never carried here.
        path: PathBuf,
    },
    /// Filesystem I/O failed while reading or writing the synthetic tree.
    Io {
        /// Redacted logical path class. Host paths are never carried here.
        path: PathBuf,
        /// Redaction-safe I/O error class.
        detail: String,
    },
    /// Snapshot serialization failed.
    Serialization {
        /// Redaction-safe serialization error detail.
        detail: String,
    },
    /// The captured pre-state snapshot is unreadable or fails integrity checks.
    CorruptedPreStateSnapshot {
        /// Redacted logical path class. Host paths are never carried here.
        path: PathBuf,
        /// Redaction-safe failure detail.
        detail: String,
    },
    /// A bounded synthetic input exceeded a declared resource limit.
    ResourceLimitExceeded {
        /// Redaction-safe resource class.
        resource: &'static str,
        /// Admitted maximum for the resource.
        limit: u64,
    },
    /// Restore completed but one or more bounded backup or staging artifacts
    /// could not be removed safely and were retained for explicit cleanup.
    RecoverableCleanupPending {
        /// Number of backup or staging artifacts still present.
        retained_backup_count: usize,
    },
    /// A restore transaction could not re-establish its pre-mutation layout.
    /// Existing and staged directories are retained rather than deleted.
    RestoreRecoveryRequired {
        /// Number of backup or staging artifacts retained for recovery.
        retained_backup_count: usize,
    },
    /// Pre-state did not capture any entries.
    PreStateNotCaptured {
        /// Drill plan id.
        drill_id: String,
    },
    /// The synthetic update did not alter the target roots before rollback.
    SyntheticUpdateDidNotTouchTarget {
        /// Drill plan id.
        drill_id: String,
    },
    /// Post-state did not match the captured pre-state after rollback.
    TargetNotRolledBack {
        /// Differences found after rollback.
        diffs: Vec<RollbackDrillDiff>,
    },
}

impl fmt::Display for RollbackDrillError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TopologyPacketInvalid { findings } => {
                write!(f, "install topology packet is invalid: {}", findings.len())
            }
            Self::MissingTopologyRow { topology_row_id } => {
                let _ = topology_row_id;
                write!(f, "missing install topology row")
            }
            Self::InvalidDrillPlan { detail } => {
                let _ = detail;
                write!(f, "invalid rollback drill plan")
            }
            Self::UnsafeStateRoot { root_ref, detail } => {
                let _ = root_ref;
                write!(f, "unsafe rollback drill state root: {detail}")
            }
            Self::MissingStateRoot { root_ref, path } => {
                let _ = (root_ref, path);
                write!(f, "rollback drill state root is missing")
            }
            Self::Io { path, detail } => {
                let _ = path;
                write!(f, "rollback drill I/O failed: {detail}")
            }
            Self::Serialization { detail } => {
                write!(f, "rollback drill serialization failed: {detail}")
            }
            Self::CorruptedPreStateSnapshot { path, detail } => {
                let _ = path;
                write!(
                    f,
                    "rollback drill pre-state snapshot is corrupted: {detail}"
                )
            }
            Self::ResourceLimitExceeded { resource, limit } => {
                write!(f, "rollback drill {resource} exceeds limit {limit}")
            }
            Self::RecoverableCleanupPending {
                retained_backup_count,
            } => write!(
                f,
                "rollback drill restored state but retained {retained_backup_count} bounded backup or staging artifacts"
            ),
            Self::RestoreRecoveryRequired {
                retained_backup_count,
            } => write!(
                f,
                "rollback drill restore needs recovery; {retained_backup_count} backup or staging artifacts were retained"
            ),
            Self::PreStateNotCaptured { drill_id } => {
                let _ = drill_id;
                write!(f, "rollback drill captured no pre-state")
            }
            Self::SyntheticUpdateDidNotTouchTarget { drill_id } => {
                let _ = drill_id;
                write!(
                    f,
                    "rollback drill synthetic update did not touch target roots"
                )
            }
            Self::TargetNotRolledBack { diffs } => {
                write!(
                    f,
                    "rollback drill target did not roll back: {}",
                    diffs.len()
                )
            }
        }
    }
}

impl std::error::Error for RollbackDrillError {}

/// Filesystem-backed driver for synthetic rollback drills.
#[derive(Debug, Clone)]
pub struct RollbackDrillDriver {
    synthetic_tree_root: PathBuf,
    authority: Arc<OnceLock<SyntheticAuthority>>,
}

#[derive(Debug, Clone)]
struct SyntheticAuthority {
    canonical_root: PathBuf,
    root_identity: ObjectIdentity,
}

/// Stable object identity on Unix. Windows fields are only a bounded-change
/// observation used by read/create checks; they never authorize destructive
/// restore or pathname cleanup because `std` exposes no stable file id there.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ObjectIdentity {
    #[cfg(unix)]
    device: u64,
    #[cfg(unix)]
    inode: u64,
    #[cfg(unix)]
    mode: u32,
    #[cfg(windows)]
    creation_time: u64,
    #[cfg(windows)]
    file_attributes: u32,
    #[cfg(not(any(unix, windows)))]
    is_directory: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FileReadStamp {
    object: ObjectIdentity,
    len: u64,
    modified: Option<SystemTime>,
    #[cfg(unix)]
    change_time_seconds: i64,
    #[cfg(unix)]
    change_time_nanos: i64,
    #[cfg(windows)]
    last_write_time: u64,
}

#[derive(Debug, Default)]
struct CaptureBudget {
    entries: usize,
    total_file_bytes: u64,
}

#[derive(Debug)]
struct SyntheticUpdateFailure {
    error: RollbackDrillError,
    mutation_started: bool,
}

/// An armed temporary-file handle. Until the write is durably installed and
/// verified, every error path and unwind truncates and syncs the open inode.
/// Pathname cleanup is attempted only when the parent still has its pinned
/// identity; Windows and unknown platforms retain a zero-byte file rather than
/// authorize deletion from metadata that is not a true file identity.
#[derive(Debug)]
struct PendingFile {
    file: File,
    current_path: PathBuf,
    parent: PathBuf,
    parent_identity: ObjectIdentity,
    armed: bool,
}

impl PendingFile {
    fn new(
        file: File,
        current_path: PathBuf,
        parent: PathBuf,
        parent_identity: ObjectIdentity,
    ) -> Self {
        Self {
            file,
            current_path,
            parent,
            parent_identity,
            armed: true,
        }
    }

    fn install(
        &mut self,
        authority: &SyntheticAuthority,
        target: &Path,
    ) -> Result<(), RollbackDrillError> {
        install_temporary_file(authority, &self.current_path, target, &self.parent)?;
        self.current_path = target.to_path_buf();
        Ok(())
    }

    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for PendingFile {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        let _ = self.file.set_len(0);
        let _ = self.file.sync_all();
        if !destructive_path_cleanup_supported() {
            return;
        }
        let parent_matches = fs::symlink_metadata(&self.parent)
            .ok()
            .filter(|metadata| !metadata_is_redirect(metadata) && metadata.is_dir())
            .map(|metadata| object_identity(&metadata) == self.parent_identity)
            .unwrap_or(false);
        if !parent_matches {
            return;
        }
        let handle_identity = self
            .file
            .metadata()
            .ok()
            .map(|metadata| object_identity(&metadata));
        let path_matches = fs::symlink_metadata(&self.current_path)
            .ok()
            .filter(|metadata| !metadata_is_redirect(metadata) && metadata.is_file())
            .map(|metadata| Some(object_identity(&metadata)) == handle_identity)
            .unwrap_or(false);
        if path_matches {
            let _ = fs::remove_file(&self.current_path);
            let _ = sync_directory(&self.parent);
        }
    }
}

impl RollbackDrillDriver {
    /// Creates a driver rooted at a synthetic filesystem tree.
    pub fn new(synthetic_tree_root: impl AsRef<Path>) -> Self {
        Self {
            synthetic_tree_root: synthetic_tree_root.as_ref().to_path_buf(),
            authority: Arc::new(OnceLock::new()),
        }
    }

    /// Returns the synthetic root directory used by this driver.
    pub fn synthetic_tree_root(&self) -> &Path {
        &self.synthetic_tree_root
    }

    /// Returns the path for a durable state-root ref under the synthetic tree.
    ///
    /// # Errors
    ///
    /// Returns [`RollbackDrillError::UnsafeStateRoot`] when the ref cannot be
    /// represented as a single synthetic path segment.
    pub fn state_root_path(&self, root_ref: &str) -> Result<PathBuf, RollbackDrillError> {
        let authority = self.authority()?;
        let state_roots = secure_state_roots_path(authority, false)?;
        let path = state_roots.join(safe_root_segment(root_ref)?);
        verify_path_if_present(authority, &path, ExpectedPathKind::Directory)?;
        Ok(path)
    }

    /// Returns the pre-state snapshot path for `drill_id`.
    pub fn pre_state_snapshot_path(&self, drill_id: &str) -> PathBuf {
        self.authority
            .get()
            .map(|authority| authority.canonical_root.as_path())
            .unwrap_or(self.synthetic_tree_root.as_path())
            .join(SYNTHETIC_AUTHORITY_DIRECTORY)
            .join(format!(
                "{}{}",
                sanitize_id(drill_id),
                ROLLBACK_DRILL_PRE_STATE_SUFFIX
            ))
    }

    /// Creates a deterministic synthetic state tree for the plan.
    ///
    /// # Errors
    ///
    /// Returns [`RollbackDrillError`] if root refs are unsafe or the synthetic
    /// tree cannot be written.
    pub fn seed_synthetic_state_tree(
        &self,
        plan: &RollbackDrillPlan,
    ) -> Result<RollbackDrillLayout, RollbackDrillError> {
        validate_drill_plan_bounds(plan)?;
        let authority = self.authority()?;
        let state_roots = secure_state_roots_path(authority, true)?;
        let mut roots = Vec::new();
        for root in &plan.roots {
            let path = state_roots.join(safe_root_segment(&root.root_ref)?);
            create_secure_directory(authority, &path)?;
            atomic_write_bounded(
                authority,
                &path.join("state-root.json"),
                synthetic_state_root_body(root).as_bytes(),
            )?;
            atomic_write_bounded(
                authority,
                &path.join("settings").join("profile.json"),
                format!(
                    "{{\"root_ref\":\"{}\",\"channel\":\"{:?}\",\"role\":\"{:?}\"}}\n",
                    root.root_ref, root.channel_class, root.role
                )
                .as_bytes(),
            )?;
            atomic_write_bounded(
                authority,
                &path.join("build").join("current.txt"),
                format!("previous-build:{}\n", root.topology_row_id).as_bytes(),
            )?;
            atomic_write_bounded(
                authority,
                &path.join("support").join("export-index.json"),
                format!(
                    "{{\"support_ref\":\"support.install.rollback.{}\"}}\n",
                    root.root_ref
                )
                .as_bytes(),
            )?;
            roots.push(RollbackDrillRootPath {
                root_ref: root.root_ref.clone(),
                path,
            });
        }
        Ok(RollbackDrillLayout { roots })
    }

    /// Captures and writes the pre-state snapshot for a plan.
    ///
    /// # Errors
    ///
    /// Returns [`RollbackDrillError`] when a planned root is missing,
    /// unreadable, unsafe, or captures no entries.
    pub fn capture_pre_state(
        &self,
        plan: &RollbackDrillPlan,
    ) -> Result<RollbackDrillPreStateSnapshot, RollbackDrillError> {
        validate_drill_plan_bounds(plan)?;
        let authority = self.authority()?;
        let mut snapshot = self.capture_snapshot(plan)?;
        if snapshot.entries.is_empty() {
            return Err(RollbackDrillError::PreStateNotCaptured {
                drill_id: plan.drill_id.clone(),
            });
        }
        snapshot.entry_digest = digest_snapshot(&snapshot);
        let path = self.pre_state_snapshot_path(&plan.drill_id);
        let json = serde_json::to_vec_pretty(&snapshot).map_err(|err| {
            RollbackDrillError::Serialization {
                detail: serialization_error_class(&err),
            }
        })?;
        if json.len() as u64 > ROLLBACK_DRILL_MAX_SNAPSHOT_BYTES {
            return corrupted(&path, "snapshot exceeds the serialized byte limit");
        }
        atomic_write_bounded_with_limit(
            authority,
            &path,
            &json,
            ROLLBACK_DRILL_MAX_SNAPSHOT_BYTES,
        )?;
        Ok(snapshot)
    }

    /// Runs the full rollback drill after capturing pre-state.
    ///
    /// # Errors
    ///
    /// Returns [`RollbackDrillError`] when snapshot capture, synthetic update,
    /// rollback, or post-state comparison fails.
    pub fn run(&self, plan: &RollbackDrillPlan) -> Result<RollbackDrillReport, RollbackDrillError> {
        self.capture_pre_state(plan)?;
        self.run_from_captured_pre_state(plan)
    }

    /// Runs the rollback drill using an already captured pre-state snapshot.
    ///
    /// # Errors
    ///
    /// Returns [`RollbackDrillError::CorruptedPreStateSnapshot`] when the
    /// snapshot cannot be parsed or fails its integrity digest.
    pub fn run_from_captured_pre_state(
        &self,
        plan: &RollbackDrillPlan,
    ) -> Result<RollbackDrillReport, RollbackDrillError> {
        validate_drill_plan_bounds(plan)?;
        self.authority()?;
        let snapshot = self.load_pre_state_snapshot(plan)?;
        if let Err(failure) = self.apply_synthetic_update(plan) {
            if failure.mutation_started {
                return self.fail_after_restoring_mutation(plan, &snapshot, failure.error);
            }
            return Err(failure.error);
        }
        let mutated_snapshot = match self.capture_snapshot(plan) {
            Ok(snapshot) => snapshot,
            Err(error) => {
                return self.fail_after_restoring_mutation(plan, &snapshot, error);
            }
        };
        if !target_changed(&snapshot, &mutated_snapshot, plan) {
            let error = RollbackDrillError::SyntheticUpdateDidNotTouchTarget {
                drill_id: plan.drill_id.clone(),
            };
            return self.fail_after_restoring_mutation(plan, &snapshot, error);
        }

        self.restore_target_roots(plan, &snapshot)?;
        self.write_expected_delta_evidence(plan)?;

        let mut post_snapshot = self.capture_snapshot(plan)?;
        post_snapshot.entry_digest = digest_snapshot(&post_snapshot);
        let diffs = compare_snapshots(&snapshot, &post_snapshot, &plan.expected_deltas);
        if !diffs.is_empty() {
            return Err(RollbackDrillError::TargetNotRolledBack { diffs });
        }

        Ok(RollbackDrillReport {
            record_kind: ROLLBACK_DRILL_REPORT_RECORD_KIND.to_string(),
            schema_version: ROLLBACK_DRILL_SCHEMA_VERSION,
            drill_id: plan.drill_id.clone(),
            pre_state_snapshot_id: snapshot.snapshot_id,
            pre_state_captured: true,
            target_rolled_back: true,
            pre_state_entry_count: snapshot.entries.len(),
            post_state_entry_count: post_snapshot.entries.len(),
            expected_delta_count: plan.expected_deltas.len(),
            compared_root_refs: plan
                .roots
                .iter()
                .map(|root| root.root_ref.clone())
                .collect(),
            diffs: Vec::new(),
        })
    }

    fn capture_snapshot(
        &self,
        plan: &RollbackDrillPlan,
    ) -> Result<RollbackDrillPreStateSnapshot, RollbackDrillError> {
        validate_drill_plan_bounds(plan)?;
        let authority = self.authority()?;
        let state_roots = secure_state_roots_path(authority, false)?;
        let mut entries = Vec::new();
        let mut budget = CaptureBudget::default();
        for root in &plan.roots {
            let root_path = state_roots.join(safe_root_segment(&root.root_ref)?);
            let metadata = match fs::symlink_metadata(&root_path) {
                Ok(metadata) => metadata,
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                    return Err(RollbackDrillError::MissingStateRoot {
                        root_ref: safe_error_ref(&root.root_ref),
                        path: redacted_error_path(),
                    });
                }
                Err(err) => return Err(io_error(&root_path, err)),
            };
            if metadata_is_redirect(&metadata) || !metadata.is_dir() {
                return unsafe_state_root("planned state root is not a regular directory");
            }
            verify_path_if_present(authority, &root_path, ExpectedPathKind::Directory)?;
            if !root_path.starts_with(&state_roots) {
                return Err(RollbackDrillError::MissingStateRoot {
                    root_ref: safe_error_ref(&root.root_ref),
                    path: redacted_error_path(),
                });
            }
            walk_root(
                authority,
                &root.root_ref,
                &root_path,
                &root_path,
                0,
                &mut entries,
                &mut budget,
            )?;
        }
        entries.sort_by(|left, right| {
            left.root_ref
                .cmp(&right.root_ref)
                .then_with(|| left.relative_path.cmp(&right.relative_path))
        });

        Ok(RollbackDrillPreStateSnapshot {
            record_kind: ROLLBACK_DRILL_PRE_STATE_RECORD_KIND.to_string(),
            schema_version: ROLLBACK_DRILL_SCHEMA_VERSION,
            snapshot_id: format!("snapshot:rollback-drill:{}", now_nanos()),
            drill_id: plan.drill_id.clone(),
            target_topology_row_id: plan.target_topology_row_id.clone(),
            roots: plan.roots.clone(),
            entries,
            entry_digest: String::new(),
            captured_at: format!("unix-nanos:{}", now_nanos()),
        })
    }

    fn load_pre_state_snapshot(
        &self,
        plan: &RollbackDrillPlan,
    ) -> Result<RollbackDrillPreStateSnapshot, RollbackDrillError> {
        let authority = self.authority()?;
        let path = self.pre_state_snapshot_path(&plan.drill_id);
        let bytes = read_regular_file_bounded(authority, &path, ROLLBACK_DRILL_MAX_SNAPSHOT_BYTES)?;
        let value: serde_json::Value = serde_json::from_slice(&bytes)
            .map_err(|_| corrupted_error("snapshot JSON is malformed"))?;
        validate_json_shape(&value)?;
        let snapshot: RollbackDrillPreStateSnapshot = serde_json::from_value(value)
            .map_err(|_| corrupted_error("snapshot fields are malformed"))?;
        validate_snapshot(&path, plan, &snapshot)?;
        Ok(snapshot)
    }

    fn apply_synthetic_update(
        &self,
        plan: &RollbackDrillPlan,
    ) -> Result<(), SyntheticUpdateFailure> {
        let mut mutation_started = false;
        let authority = self.authority().map_err(|error| SyntheticUpdateFailure {
            error,
            mutation_started,
        })?;
        for root in plan
            .roots
            .iter()
            .filter(|root| root.role == RollbackDrillRootRole::TargetRollback)
        {
            let root_path =
                self.state_root_path(&root.root_ref)
                    .map_err(|error| SyntheticUpdateFailure {
                        error,
                        mutation_started,
                    })?;
            verify_path_if_present(authority, &root_path, ExpectedPathKind::Directory).map_err(
                |error| SyntheticUpdateFailure {
                    error,
                    mutation_started,
                },
            )?;
            atomic_write_bounded_tracking_mutation(
                authority,
                &root_path.join("build").join("current.txt"),
                format!("candidate-build:{}\n", root.topology_row_id).as_bytes(),
                &mut mutation_started,
            )
            .map_err(|error| SyntheticUpdateFailure {
                error,
                mutation_started,
            })?;
            if take_synthetic_update_failure_after_first_write() {
                return Err(SyntheticUpdateFailure {
                    error: RollbackDrillError::Io {
                        path: redacted_error_path(),
                        detail: "injected partial synthetic update failure".to_string(),
                    },
                    mutation_started,
                });
            }
            atomic_write_bounded_tracking_mutation(
                authority,
                &root_path
                    .join("update-staging")
                    .join("candidate-marker.json"),
                format!(
                    "{{\"target\":\"{}\",\"root_ref\":\"{}\",\"synthetic\":true}}\n",
                    root.topology_row_id, root.root_ref
                )
                .as_bytes(),
                &mut mutation_started,
            )
            .map_err(|error| SyntheticUpdateFailure {
                error,
                mutation_started,
            })?;
        }
        Ok(())
    }

    fn fail_after_restoring_mutation<T>(
        &self,
        plan: &RollbackDrillPlan,
        snapshot: &RollbackDrillPreStateSnapshot,
        mutation_error: RollbackDrillError,
    ) -> Result<T, RollbackDrillError> {
        match self.restore_target_roots(plan, snapshot) {
            Ok(()) => Err(mutation_error),
            Err(
                restore_error @ (RollbackDrillError::RecoverableCleanupPending { .. }
                | RollbackDrillError::RestoreRecoveryRequired { .. }),
            ) => Err(restore_error),
            Err(_) => Err(RollbackDrillError::RestoreRecoveryRequired {
                retained_backup_count: 0,
            }),
        }
    }

    fn restore_target_roots(
        &self,
        plan: &RollbackDrillPlan,
        snapshot: &RollbackDrillPreStateSnapshot,
    ) -> Result<(), RollbackDrillError> {
        let authority =
            self.authority()
                .map_err(|_| RollbackDrillError::RestoreRecoveryRequired {
                    retained_backup_count: 0,
                })?;
        match restore_target_roots_transactionally(authority, plan, snapshot) {
            Ok(()) => Ok(()),
            Err(
                error @ (RollbackDrillError::RecoverableCleanupPending { .. }
                | RollbackDrillError::RestoreRecoveryRequired { .. }),
            ) => Err(error),
            Err(_) => Err(RollbackDrillError::RestoreRecoveryRequired {
                retained_backup_count: 0,
            }),
        }
    }

    fn write_expected_delta_evidence(
        &self,
        plan: &RollbackDrillPlan,
    ) -> Result<(), RollbackDrillError> {
        let authority = self.authority()?;
        for delta in &plan.expected_deltas {
            if delta.delta_class != RollbackDrillDeltaClass::PostRollbackEvidence {
                continue;
            }
            let root_path = self.state_root_path(&delta.root_ref)?;
            let path = join_relative(&root_path, &delta.relative_path)?;
            atomic_write_bounded(
                authority,
                &path,
                format!(
                    "{{\"drill_id\":\"{}\",\"delta_class\":\"post_rollback_evidence\"}}\n",
                    plan.drill_id
                )
                .as_bytes(),
            )?;
        }
        Ok(())
    }

    fn authority(&self) -> Result<&SyntheticAuthority, RollbackDrillError> {
        if self.authority.get().is_none() {
            let candidate = initialize_synthetic_authority(&self.synthetic_tree_root)?;
            let _ = self.authority.set(candidate);
        }
        let authority = self.authority.get().ok_or_else(|| RollbackDrillError::Io {
            path: redacted_error_path(),
            detail: "synthetic authority initialization failed".to_string(),
        })?;
        revalidate_synthetic_authority(&self.synthetic_tree_root, authority)?;
        Ok(authority)
    }
}

#[derive(Debug, Clone, Copy)]
enum ExpectedPathKind {
    Directory,
    RegularFile,
}

fn initialize_synthetic_authority(
    requested_root: &Path,
) -> Result<SyntheticAuthority, RollbackDrillError> {
    if !requested_root.is_absolute() {
        return unsafe_state_root("synthetic authority must be an absolute path");
    }
    let requested_metadata =
        fs::symlink_metadata(requested_root).map_err(|err| io_error(requested_root, err))?;
    if metadata_is_redirect(&requested_metadata) || !requested_metadata.is_dir() {
        return unsafe_state_root("synthetic authority must be a regular directory");
    }
    let canonical_root =
        fs::canonicalize(requested_root).map_err(|err| io_error(requested_root, err))?;
    let canonical_metadata =
        fs::symlink_metadata(&canonical_root).map_err(|err| io_error(&canonical_root, err))?;
    if metadata_is_redirect(&canonical_metadata) || !canonical_metadata.is_dir() {
        return unsafe_state_root("synthetic authority did not resolve to a regular directory");
    }
    if directory_is_group_or_world_writable(&canonical_metadata) {
        return unsafe_state_root("synthetic authority permissions are too broad");
    }

    let root_identity = object_identity(&canonical_metadata);
    let marker_directory = canonical_root.join(SYNTHETIC_AUTHORITY_DIRECTORY);
    let marker = marker_directory.join(SYNTHETIC_AUTHORITY_MARKER);
    let mut entry_count = 0usize;
    let mut marker_directory_present = false;
    for entry in fs::read_dir(&canonical_root).map_err(|err| io_error(&canonical_root, err))? {
        let entry = entry.map_err(|err| io_error(&canonical_root, err))?;
        entry_count = entry_count.saturating_add(1);
        if entry_count > ROLLBACK_DRILL_MAX_DIRECTORY_ENTRIES {
            return resource_limit(
                "synthetic authority entries",
                ROLLBACK_DRILL_MAX_DIRECTORY_ENTRIES,
            );
        }
        marker_directory_present = marker_directory_present
            || entry.file_name().as_os_str() == std::ffi::OsStr::new(SYNTHETIC_AUTHORITY_DIRECTORY);
    }

    if entry_count == 0 {
        create_one_directory(&marker_directory).map_err(|err| io_error(&marker_directory, err))?;
        let mut marker_file = private_create_new(&marker)?;
        marker_file
            .write_all(SYNTHETIC_AUTHORITY_MARKER_BODY)
            .map_err(|err| io_error(&marker, err))?;
        marker_file
            .sync_all()
            .map_err(|err| io_error(&marker, err))?;
        sync_directory(&marker_directory)?;
        sync_directory(&canonical_root)?;
    } else if !marker_directory_present {
        return unsafe_state_root(
            "non-empty synthetic authority is missing the explicit authority marker",
        );
    }

    validate_authority_marker(&canonical_root)?;
    let after =
        fs::symlink_metadata(&canonical_root).map_err(|err| io_error(&canonical_root, err))?;
    if metadata_is_redirect(&after) || !after.is_dir() || object_identity(&after) != root_identity {
        return unsafe_state_root("synthetic authority changed during initialization");
    }

    Ok(SyntheticAuthority {
        canonical_root,
        root_identity,
    })
}

fn revalidate_synthetic_authority(
    requested_root: &Path,
    authority: &SyntheticAuthority,
) -> Result<(), RollbackDrillError> {
    let requested_metadata =
        fs::symlink_metadata(requested_root).map_err(|err| io_error(requested_root, err))?;
    if metadata_is_redirect(&requested_metadata) || !requested_metadata.is_dir() {
        return unsafe_state_root("synthetic authority became a redirect or non-directory");
    }
    let canonical =
        fs::canonicalize(requested_root).map_err(|err| io_error(requested_root, err))?;
    if canonical != authority.canonical_root {
        return unsafe_state_root("synthetic authority resolved outside its pinned directory");
    }
    revalidate_canonical_authority(authority)
}

fn revalidate_canonical_authority(
    authority: &SyntheticAuthority,
) -> Result<(), RollbackDrillError> {
    let metadata = fs::symlink_metadata(&authority.canonical_root)
        .map_err(|err| io_error(&authority.canonical_root, err))?;
    if metadata_is_redirect(&metadata)
        || !metadata.is_dir()
        || directory_is_group_or_world_writable(&metadata)
        || object_identity(&metadata) != authority.root_identity
    {
        return unsafe_state_root("synthetic authority identity changed");
    }
    validate_authority_marker(&authority.canonical_root)
}

fn validate_authority_marker(canonical_root: &Path) -> Result<(), RollbackDrillError> {
    #[cfg(unix)]
    let root_metadata =
        fs::symlink_metadata(canonical_root).map_err(|err| io_error(canonical_root, err))?;
    let marker_directory = canonical_root.join(SYNTHETIC_AUTHORITY_DIRECTORY);
    let directory_metadata =
        fs::symlink_metadata(&marker_directory).map_err(|err| io_error(&marker_directory, err))?;
    if metadata_is_redirect(&directory_metadata) || !directory_metadata.is_dir() {
        return unsafe_state_root("synthetic authority marker directory is unsafe");
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if directory_metadata.dev() != root_metadata.dev() {
            return unsafe_state_root("synthetic authority marker crosses a filesystem boundary");
        }
    }
    let marker_directory_canonical =
        fs::canonicalize(&marker_directory).map_err(|err| io_error(&marker_directory, err))?;
    if marker_directory_canonical != marker_directory {
        return unsafe_state_root("synthetic authority marker directory escaped containment");
    }

    let marker = marker_directory.join(SYNTHETIC_AUTHORITY_MARKER);
    let marker_metadata = fs::symlink_metadata(&marker).map_err(|err| io_error(&marker, err))?;
    if metadata_is_redirect(&marker_metadata) || !marker_metadata.is_file() {
        return unsafe_state_root("synthetic authority marker is unsafe");
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if marker_metadata.dev() != root_metadata.dev() {
            return unsafe_state_root("synthetic authority marker crosses a filesystem boundary");
        }
    }
    if marker_metadata.len() != SYNTHETIC_AUTHORITY_MARKER_BODY.len() as u64 {
        return unsafe_state_root("synthetic authority marker is invalid");
    }
    let mut file = File::open(&marker).map_err(|err| io_error(&marker, err))?;
    let handle_metadata = file.metadata().map_err(|err| io_error(&marker, err))?;
    if object_identity(&handle_metadata) != object_identity(&marker_metadata) {
        return unsafe_state_root("synthetic authority marker identity changed");
    }
    let mut bytes = Vec::with_capacity(SYNTHETIC_AUTHORITY_MARKER_BODY.len());
    Read::by_ref(&mut file)
        .take(SYNTHETIC_AUTHORITY_MARKER_BODY.len() as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|err| io_error(&marker, err))?;
    if bytes != SYNTHETIC_AUTHORITY_MARKER_BODY {
        return unsafe_state_root("synthetic authority marker is invalid");
    }
    let after = fs::symlink_metadata(&marker).map_err(|err| io_error(&marker, err))?;
    if metadata_is_redirect(&after) || file_read_stamp(&after) != file_read_stamp(&handle_metadata)
    {
        return unsafe_state_root("synthetic authority marker changed while reading");
    }
    Ok(())
}

fn secure_state_roots_path(
    authority: &SyntheticAuthority,
    create: bool,
) -> Result<PathBuf, RollbackDrillError> {
    revalidate_canonical_authority(authority)?;
    let state_roots = authority.canonical_root.join("state-roots");
    match fs::symlink_metadata(&state_roots) {
        Ok(metadata) => {
            if metadata_is_redirect(&metadata) || !metadata.is_dir() {
                return unsafe_state_root("synthetic state-roots authority is unsafe");
            }
            let canonical =
                fs::canonicalize(&state_roots).map_err(|err| io_error(&state_roots, err))?;
            if canonical != state_roots {
                return unsafe_state_root("synthetic state-roots authority escaped containment");
            }
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound && create => {
            let root_before = directory_identity(&authority.canonical_root)?;
            create_one_directory(&state_roots).map_err(|err| io_error(&state_roots, err))?;
            if directory_identity(&authority.canonical_root)? != root_before {
                return unsafe_state_root("synthetic authority changed while creating state roots");
            }
            verify_path_if_present(authority, &state_roots, ExpectedPathKind::Directory)?;
            sync_directory(&authority.canonical_root)?;
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => return Err(io_error(&state_roots, err)),
    }
    if fs::symlink_metadata(&state_roots).is_ok() {
        verify_path_if_present(authority, &state_roots, ExpectedPathKind::Directory)?;
    }
    Ok(state_roots)
}

fn verify_path_if_present(
    authority: &SyntheticAuthority,
    path: &Path,
    expected: ExpectedPathKind,
) -> Result<(), RollbackDrillError> {
    ensure_lexically_contained(authority, path)?;
    let relative = path
        .strip_prefix(&authority.canonical_root)
        .map_err(|_| unsafe_state_root_error("path escaped the synthetic authority"))?;
    let mut current = authority.canonical_root.clone();
    let components: Vec<_> = relative.components().collect();
    for (index, component) in components.iter().enumerate() {
        let std::path::Component::Normal(segment) = component else {
            return unsafe_state_root("path contains an unsafe component");
        };
        current.push(segment);
        let metadata = match fs::symlink_metadata(&current) {
            Ok(metadata) => metadata,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(err) => return Err(io_error(&current, err)),
        };
        if metadata_is_redirect(&metadata) {
            return unsafe_state_root("path contains a redirect component");
        }
        if !metadata_is_on_authority_filesystem(authority, &metadata) {
            return unsafe_state_root("path crosses the synthetic authority filesystem");
        }
        if metadata.is_dir() && directory_is_group_or_world_writable(&metadata) {
            return unsafe_state_root("path contains a broadly writable directory");
        }
        let is_final = index + 1 == components.len();
        if !is_final && !metadata.is_dir() {
            return unsafe_state_root("path ancestor is not a directory");
        }
        if is_final {
            let kind_matches = match expected {
                ExpectedPathKind::Directory => metadata.is_dir(),
                ExpectedPathKind::RegularFile => metadata.is_file(),
            };
            if !kind_matches {
                return unsafe_state_root("path has an unexpected filesystem kind");
            }
        }
        let canonical = fs::canonicalize(&current).map_err(|err| io_error(&current, err))?;
        if canonical != current || !canonical.starts_with(&authority.canonical_root) {
            return unsafe_state_root("path resolved outside the synthetic authority");
        }
    }
    Ok(())
}

fn ensure_lexically_contained(
    authority: &SyntheticAuthority,
    path: &Path,
) -> Result<(), RollbackDrillError> {
    if !path.is_absolute() || !path.starts_with(&authority.canonical_root) {
        return unsafe_state_root("path is outside the synthetic authority");
    }
    let relative = path
        .strip_prefix(&authority.canonical_root)
        .map_err(|_| unsafe_state_root_error("path is outside the synthetic authority"))?;
    if relative
        .components()
        .any(|component| !matches!(component, std::path::Component::Normal(_)))
    {
        return unsafe_state_root("path contains an unsafe component");
    }
    Ok(())
}

fn create_secure_directory(
    authority: &SyntheticAuthority,
    path: &Path,
) -> Result<(), RollbackDrillError> {
    ensure_lexically_contained(authority, path)?;
    let relative = path
        .strip_prefix(&authority.canonical_root)
        .map_err(|_| unsafe_state_root_error("directory escaped the synthetic authority"))?;
    let mut current = authority.canonical_root.clone();
    for component in relative.components() {
        let std::path::Component::Normal(segment) = component else {
            return unsafe_state_root("directory contains an unsafe component");
        };
        let parent = current.clone();
        current.push(segment);
        match fs::symlink_metadata(&current) {
            Ok(metadata) => {
                if metadata_is_redirect(&metadata) || !metadata.is_dir() {
                    return unsafe_state_root("directory component is unsafe");
                }
                if !metadata_is_on_authority_filesystem(authority, &metadata) {
                    return unsafe_state_root(
                        "directory component crosses the synthetic authority filesystem",
                    );
                }
                let canonical =
                    fs::canonicalize(&current).map_err(|err| io_error(&current, err))?;
                if canonical != current {
                    return unsafe_state_root("directory component escaped containment");
                }
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                let parent_before = directory_identity(&parent)?;
                create_one_directory(&current).map_err(|err| io_error(&current, err))?;
                if directory_identity(&parent)? != parent_before {
                    return unsafe_state_root("directory parent changed during creation");
                }
                verify_path_if_present(authority, &current, ExpectedPathKind::Directory)?;
                sync_directory(&parent)?;
            }
            Err(err) => return Err(io_error(&current, err)),
        }
    }
    Ok(())
}

#[cfg(unix)]
fn create_one_directory(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::DirBuilderExt;

    let mut builder = fs::DirBuilder::new();
    builder.mode(0o700);
    builder.create(path)
}

#[cfg(not(unix))]
fn create_one_directory(path: &Path) -> std::io::Result<()> {
    fs::DirBuilder::new().create(path)
}

fn private_create_new(path: &Path) -> Result<File, RollbackDrillError> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options.open(path).map_err(|err| io_error(path, err))
}

fn directory_identity(path: &Path) -> Result<ObjectIdentity, RollbackDrillError> {
    let metadata = fs::symlink_metadata(path).map_err(|err| io_error(path, err))?;
    if metadata_is_redirect(&metadata) || !metadata.is_dir() {
        return Err(unsafe_state_root_error("directory identity is unsafe"));
    }
    Ok(object_identity(&metadata))
}

fn object_identity(metadata: &Metadata) -> ObjectIdentity {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        ObjectIdentity {
            device: metadata.dev(),
            inode: metadata.ino(),
            mode: metadata.mode(),
        }
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        ObjectIdentity {
            creation_time: metadata.creation_time(),
            file_attributes: metadata.file_attributes(),
        }
    }
    #[cfg(not(any(unix, windows)))]
    {
        ObjectIdentity {
            is_directory: metadata.is_dir(),
        }
    }
}

fn file_read_stamp(metadata: &Metadata) -> FileReadStamp {
    #[cfg(unix)]
    use std::os::unix::fs::MetadataExt;
    #[cfg(windows)]
    use std::os::windows::fs::MetadataExt;

    FileReadStamp {
        object: object_identity(metadata),
        len: metadata.len(),
        modified: metadata.modified().ok(),
        #[cfg(unix)]
        change_time_seconds: metadata.ctime(),
        #[cfg(unix)]
        change_time_nanos: metadata.ctime_nsec(),
        #[cfg(windows)]
        last_write_time: metadata.last_write_time(),
    }
}

fn metadata_is_redirect(metadata: &Metadata) -> bool {
    if metadata.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        windows_file_attributes_include_reparse_point(metadata.file_attributes())
    }
    #[cfg(not(windows))]
    {
        false
    }
}

fn directory_is_group_or_world_writable(metadata: &Metadata) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        metadata.mode() & 0o022 != 0
    }
    #[cfg(not(unix))]
    {
        let _ = metadata;
        false
    }
}

fn metadata_is_on_authority_filesystem(
    authority: &SyntheticAuthority,
    metadata: &Metadata,
) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        metadata.dev() == authority.root_identity.device
    }
    #[cfg(not(unix))]
    {
        let _ = (authority, metadata);
        true
    }
}

#[cfg(any(test, windows))]
const fn windows_file_attributes_include_reparse_point(attributes: u32) -> bool {
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
    attributes & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

const fn destructive_path_cleanup_supported() -> bool {
    cfg!(unix)
}

#[cfg(test)]
fn take_atomic_write_failure_after_sync() -> bool {
    FAIL_ATOMIC_WRITE_AFTER_SYNC.with(|failpoint| failpoint.replace(false))
}

#[cfg(not(test))]
const fn take_atomic_write_failure_after_sync() -> bool {
    false
}

#[cfg(test)]
fn take_atomic_write_parent_sync_failure() -> bool {
    FAIL_ATOMIC_WRITE_PARENT_SYNC.with(|failpoint| failpoint.replace(false))
}

#[cfg(not(test))]
const fn take_atomic_write_parent_sync_failure() -> bool {
    false
}

#[cfg(test)]
fn take_synthetic_update_failure_after_first_write() -> bool {
    FAIL_SYNTHETIC_UPDATE_AFTER_FIRST_WRITE.with(|failpoint| failpoint.replace(false))
}

#[cfg(not(test))]
const fn take_synthetic_update_failure_after_first_write() -> bool {
    false
}

#[cfg(test)]
fn take_restore_failure_after_quarantine() -> bool {
    FAIL_RESTORE_AFTER_QUARANTINE.with(|failpoint| failpoint.replace(false))
}

#[cfg(not(test))]
const fn take_restore_failure_after_quarantine() -> bool {
    false
}

fn sync_directory(path: &Path) -> Result<(), RollbackDrillError> {
    #[cfg(unix)]
    {
        File::open(path)
            .and_then(|directory| directory.sync_all())
            .map_err(|err| io_error(path, err))?;
    }
    #[cfg(not(unix))]
    {
        let _ = path;
    }
    Ok(())
}

fn find_side_by_side_peer<'a>(
    topology: &'a InstallTopologyAlphaPacket,
    target: &InstallTopologyRow,
    paired_channel: ChannelClass,
) -> Result<&'a InstallTopologyRow, RollbackDrillError> {
    topology
        .rows
        .iter()
        .find(|row| {
            row.channel_class == paired_channel
                && row.platform_class == target.platform_class
                && row.is_side_by_side()
                && row.paired_channel_class == Some(target.channel_class)
        })
        .ok_or_else(|| RollbackDrillError::InvalidDrillPlan {
            detail: format!(
                "no side-by-side peer found for target {} and channel {:?}",
                target.topology_row_id, paired_channel
            ),
        })
}

fn extend_roots(
    roots: &mut Vec<RollbackDrillRoot>,
    row: &InstallTopologyRow,
    role: RollbackDrillRootRole,
) {
    roots.extend(
        row.durable_state_root_refs
            .iter()
            .map(|root_ref| RollbackDrillRoot {
                root_ref: root_ref.clone(),
                role,
                topology_row_id: row.topology_row_id.clone(),
                channel_class: row.channel_class,
            }),
    );
}

fn reject_duplicate_root_roles(roots: &[RollbackDrillRoot]) -> Result<(), RollbackDrillError> {
    let mut seen = BTreeMap::<&str, RollbackDrillRootRole>::new();
    for root in roots {
        if let Some(existing) = seen.insert(&root.root_ref, root.role) {
            return Err(RollbackDrillError::InvalidDrillPlan {
                detail: format!(
                    "root {} appears in both {:?} and {:?} roles",
                    root.root_ref, existing, root.role
                ),
            });
        }
    }
    Ok(())
}

fn synthetic_state_root_body(root: &RollbackDrillRoot) -> String {
    format!(
        "{{\"root_ref\":\"{}\",\"topology_row_id\":\"{}\",\"channel_class\":\"{:?}\",\"role\":\"{:?}\"}}\n",
        root.root_ref, root.topology_row_id, root.channel_class, root.role
    )
}

fn atomic_write_bounded(
    authority: &SyntheticAuthority,
    path: &Path,
    contents: &[u8],
) -> Result<(), RollbackDrillError> {
    atomic_write_bounded_with_limit_tracking(
        authority,
        path,
        contents,
        ROLLBACK_DRILL_MAX_FILE_BYTES,
        None,
    )
}

fn atomic_write_bounded_tracking_mutation(
    authority: &SyntheticAuthority,
    path: &Path,
    contents: &[u8],
    mutation_started: &mut bool,
) -> Result<(), RollbackDrillError> {
    atomic_write_bounded_with_limit_tracking(
        authority,
        path,
        contents,
        ROLLBACK_DRILL_MAX_FILE_BYTES,
        Some(mutation_started),
    )
}

fn atomic_write_bounded_with_limit(
    authority: &SyntheticAuthority,
    path: &Path,
    contents: &[u8],
    limit: u64,
) -> Result<(), RollbackDrillError> {
    atomic_write_bounded_with_limit_tracking(authority, path, contents, limit, None)
}

fn atomic_write_bounded_with_limit_tracking(
    authority: &SyntheticAuthority,
    path: &Path,
    contents: &[u8],
    limit: u64,
    mutation_started: Option<&mut bool>,
) -> Result<(), RollbackDrillError> {
    if contents.len() as u64 > limit {
        return resource_limit("write bytes", limit as usize);
    }
    ensure_lexically_contained(authority, path)?;
    let parent = path
        .parent()
        .ok_or_else(|| unsafe_state_root_error("write target has no parent"))?;
    create_secure_directory(authority, parent)?;
    verify_path_if_present(authority, parent, ExpectedPathKind::Directory)?;
    let parent_identity = directory_identity(parent)?;
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if metadata_is_redirect(&metadata) || !metadata.is_file() {
                return unsafe_state_root("write target is not a regular file");
            }
            verify_path_if_present(authority, path, ExpectedPathKind::RegularFile)?;
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => return Err(io_error(path, err)),
    }

    let (temporary_path, temporary_file) = create_temporary_file(parent)?;
    let mut pending = PendingFile::new(
        temporary_file,
        temporary_path,
        parent.to_path_buf(),
        parent_identity.clone(),
    );
    pending
        .file
        .write_all(contents)
        .map_err(|err| io_error(&pending.current_path, err))?;
    pending
        .file
        .sync_all()
        .map_err(|err| io_error(&pending.current_path, err))?;
    if take_atomic_write_failure_after_sync() {
        return Err(RollbackDrillError::Io {
            path: redacted_error_path(),
            detail: "injected post-write failure".to_string(),
        });
    }
    if directory_identity(parent)? != parent_identity {
        return unsafe_state_root("write parent changed before install");
    }
    verify_path_if_present(authority, parent, ExpectedPathKind::Directory)?;
    pending.install(authority, path)?;
    if let Some(mutation_started) = mutation_started {
        *mutation_started = true;
    }
    verify_path_if_present(authority, path, ExpectedPathKind::RegularFile)?;
    let installed = fs::symlink_metadata(path).map_err(|err| io_error(path, err))?;
    let installed_handle = pending.file.metadata().map_err(|err| io_error(path, err))?;
    if installed.len() != contents.len() as u64
        || object_identity(&installed) != object_identity(&installed_handle)
    {
        return unsafe_state_root("installed file did not match the bounded pending write");
    }
    if directory_identity(parent)? != parent_identity {
        return unsafe_state_root("write parent changed during install");
    }
    pending.disarm();
    if take_atomic_write_parent_sync_failure() {
        return Err(RollbackDrillError::Io {
            path: redacted_error_path(),
            detail: "injected parent-directory sync failure".to_string(),
        });
    }
    sync_directory(parent)?;
    Ok(())
}

fn create_temporary_file(parent: &Path) -> Result<(PathBuf, File), RollbackDrillError> {
    for attempt in 0..32u32 {
        let path = parent.join(format!(
            ".aureline-rollback-write-{}-{attempt}.tmp",
            now_nanos()
        ));
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        match options.open(&path) {
            Ok(file) => return Ok((path, file)),
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(err) => return Err(io_error(&path, err)),
        }
    }
    Err(RollbackDrillError::Io {
        path: redacted_error_path(),
        detail: "temporary file namespace is exhausted".to_string(),
    })
}

#[cfg(unix)]
fn install_temporary_file(
    authority: &SyntheticAuthority,
    temporary_path: &Path,
    target: &Path,
    parent: &Path,
) -> Result<(), RollbackDrillError> {
    verify_path_if_present(authority, parent, ExpectedPathKind::Directory)?;
    fs::rename(temporary_path, target).map_err(|err| io_error(target, err))
}

#[cfg(not(unix))]
fn install_temporary_file(
    authority: &SyntheticAuthority,
    temporary_path: &Path,
    target: &Path,
    parent: &Path,
) -> Result<(), RollbackDrillError> {
    verify_path_if_present(authority, parent, ExpectedPathKind::Directory)?;
    let target_exists = match fs::symlink_metadata(target) {
        Ok(metadata) => {
            if metadata_is_redirect(&metadata) || !metadata.is_file() {
                return unsafe_state_root("write target is unsafe");
            }
            true
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => false,
        Err(err) => return Err(io_error(target, err)),
    };
    if target_exists {
        return unsafe_state_root(
            "atomic replacement of an existing file is unavailable on this platform",
        );
    }
    fs::rename(temporary_path, target).map_err(|err| io_error(target, err))
}

fn unique_absent_sibling(parent: &Path, prefix: &str) -> Result<PathBuf, RollbackDrillError> {
    for attempt in 0..32u32 {
        let path = parent.join(format!("{prefix}-{}-{attempt}", now_nanos()));
        match fs::symlink_metadata(&path) {
            Ok(_) => continue,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(path),
            Err(err) => return Err(io_error(&path, err)),
        }
    }
    Err(RollbackDrillError::Io {
        path: redacted_error_path(),
        detail: "temporary directory namespace is exhausted".to_string(),
    })
}

fn read_regular_file_bounded(
    authority: &SyntheticAuthority,
    path: &Path,
    max_bytes: u64,
) -> Result<Vec<u8>, RollbackDrillError> {
    verify_path_if_present(authority, path, ExpectedPathKind::RegularFile)?;
    let before = fs::symlink_metadata(path).map_err(|err| io_error(path, err))?;
    if metadata_is_redirect(&before) || !before.is_file() {
        return unsafe_state_root("bounded input is not a regular file");
    }
    if before.len() > max_bytes {
        return resource_limit_u64("input file bytes", max_bytes);
    }
    let before_stamp = file_read_stamp(&before);
    let mut file = File::open(path).map_err(|err| io_error(path, err))?;
    let opened = file.metadata().map_err(|err| io_error(path, err))?;
    if !opened.is_file() || object_identity(&opened) != object_identity(&before) {
        return unsafe_state_root("bounded input identity changed before read");
    }
    let mut bytes = Vec::with_capacity((before.len() as usize).min(64 * 1024));
    Read::by_ref(&mut file)
        .take(max_bytes.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|err| io_error(path, err))?;
    if bytes.len() as u64 > max_bytes {
        return resource_limit_u64("input file bytes", max_bytes);
    }
    let handle_after = file.metadata().map_err(|err| io_error(path, err))?;
    let path_after = fs::symlink_metadata(path).map_err(|err| io_error(path, err))?;
    if metadata_is_redirect(&path_after)
        || file_read_stamp(&handle_after) != before_stamp
        || file_read_stamp(&path_after) != before_stamp
    {
        return unsafe_state_root("bounded input changed while reading");
    }
    Ok(bytes)
}

fn walk_root(
    authority: &SyntheticAuthority,
    root_ref: &str,
    root_path: &Path,
    current_path: &Path,
    depth: usize,
    entries: &mut Vec<RollbackDrillEntry>,
    budget: &mut CaptureBudget,
) -> Result<(), RollbackDrillError> {
    if depth > ROLLBACK_DRILL_MAX_DEPTH {
        return resource_limit("filesystem depth", ROLLBACK_DRILL_MAX_DEPTH);
    }
    let metadata = fs::symlink_metadata(current_path).map_err(|err| io_error(current_path, err))?;
    if metadata_is_redirect(&metadata) {
        return unsafe_state_root("synthetic tree contains a redirect entry");
    }
    if !metadata.is_dir() && !metadata.is_file() {
        return unsafe_state_root("synthetic tree contains an unsupported entry kind");
    }
    let expected_kind = if metadata.is_dir() {
        ExpectedPathKind::Directory
    } else {
        ExpectedPathKind::RegularFile
    };
    verify_path_if_present(authority, current_path, expected_kind)?;

    if current_path != root_path {
        budget.entries = budget.entries.saturating_add(1);
        if budget.entries > ROLLBACK_DRILL_MAX_ENTRIES {
            return resource_limit("filesystem entries", ROLLBACK_DRILL_MAX_ENTRIES);
        }
        let relative_path = relative_path(root_path, current_path)?;
        if relative_path.len() > ROLLBACK_DRILL_MAX_RELATIVE_PATH_BYTES {
            return resource_limit(
                "relative path bytes",
                ROLLBACK_DRILL_MAX_RELATIVE_PATH_BYTES,
            );
        }
        if metadata.is_dir() {
            entries.push(RollbackDrillEntry {
                root_ref: root_ref.to_string(),
                relative_path,
                entry_kind: RollbackDrillEntryKind::Directory,
                contents: Vec::new(),
            });
        } else {
            let contents =
                read_regular_file_bounded(authority, current_path, ROLLBACK_DRILL_MAX_FILE_BYTES)?;
            budget.total_file_bytes = budget
                .total_file_bytes
                .saturating_add(contents.len() as u64);
            if budget.total_file_bytes > ROLLBACK_DRILL_MAX_TOTAL_FILE_BYTES {
                return resource_limit_u64(
                    "aggregate file bytes",
                    ROLLBACK_DRILL_MAX_TOTAL_FILE_BYTES,
                );
            }
            entries.push(RollbackDrillEntry {
                root_ref: root_ref.to_string(),
                relative_path,
                entry_kind: RollbackDrillEntryKind::File,
                contents,
            });
        }
    }

    if metadata.is_dir() {
        let before = file_read_stamp(&metadata);
        let mut children = Vec::new();
        for child in fs::read_dir(current_path).map_err(|err| io_error(current_path, err))? {
            let child = child.map_err(|err| io_error(current_path, err))?;
            if children.len() >= ROLLBACK_DRILL_MAX_DIRECTORY_ENTRIES {
                return resource_limit("directory entries", ROLLBACK_DRILL_MAX_DIRECTORY_ENTRIES);
            }
            let name = child.file_name();
            let Some(name) = name.to_str() else {
                return unsafe_state_root("synthetic entry name is not valid UTF-8");
            };
            if name.is_empty()
                || name.len() > ROLLBACK_DRILL_MAX_FILESYSTEM_SEGMENT_BYTES
                || name.chars().any(char::is_control)
            {
                return unsafe_state_root("synthetic entry name is unsafe");
            }
            children.push(child.path());
        }
        let after =
            fs::symlink_metadata(current_path).map_err(|err| io_error(current_path, err))?;
        if metadata_is_redirect(&after) || file_read_stamp(&after) != before {
            return unsafe_state_root("synthetic directory changed while enumerating");
        }
        children.sort();
        for child in children {
            walk_root(
                authority,
                root_ref,
                root_path,
                &child,
                depth.saturating_add(1),
                entries,
                budget,
            )?;
        }
        let final_metadata =
            fs::symlink_metadata(current_path).map_err(|err| io_error(current_path, err))?;
        if metadata_is_redirect(&final_metadata) || file_read_stamp(&final_metadata) != before {
            return unsafe_state_root("synthetic directory identity changed during traversal");
        }
    }
    Ok(())
}

#[derive(Debug)]
struct StagedRootRestore {
    target: PathBuf,
    stage: PathBuf,
    backup: PathBuf,
    target_identity: ObjectIdentity,
    stage_identity: ObjectIdentity,
}

fn restore_target_roots_transactionally(
    authority: &SyntheticAuthority,
    plan: &RollbackDrillPlan,
    snapshot: &RollbackDrillPreStateSnapshot,
) -> Result<(), RollbackDrillError> {
    if !destructive_path_cleanup_supported() {
        return Err(RollbackDrillError::RestoreRecoveryRequired {
            retained_backup_count: 0,
        });
    }
    validate_snapshot(Path::new("<snapshot>"), plan, snapshot)?;
    let target_roots: BTreeSet<&str> = plan.target_root_refs().into_iter().collect();
    let state_roots = secure_state_roots_path(authority, false).map_err(|_| {
        RollbackDrillError::RestoreRecoveryRequired {
            retained_backup_count: 0,
        }
    })?;
    let state_roots_identity = directory_identity(&state_roots).map_err(|_| {
        RollbackDrillError::RestoreRecoveryRequired {
            retained_backup_count: 0,
        }
    })?;
    let mut staged = Vec::new();

    for (index, root_ref) in target_roots.iter().enumerate() {
        let observed_state_roots = match directory_identity(&state_roots) {
            Ok(identity) => identity,
            Err(_) => {
                return Err(abort_restore_staging(authority, &staged, None));
            }
        };
        if observed_state_roots != state_roots_identity {
            return Err(abort_restore_staging(authority, &staged, None));
        }
        let target_setup: Result<(PathBuf, ObjectIdentity), RollbackDrillError> = (|| {
            let target = state_roots.join(safe_root_segment(root_ref)?);
            verify_path_if_present(authority, &target, ExpectedPathKind::Directory)?;
            let target_identity = directory_identity(&target)?;
            Ok((target, target_identity))
        })();
        let (target, target_identity) = match target_setup {
            Ok(target_setup) => target_setup,
            Err(_) => {
                return Err(abort_restore_staging(authority, &staged, None));
            }
        };
        let stage = match create_unique_staging_directory(
            authority,
            &state_roots,
            ".aureline-rollback-stage",
            index,
        ) {
            Ok(stage) => stage,
            Err(_) => {
                return Err(abort_restore_staging(authority, &staged, None));
            }
        };
        if materialize_snapshot_root(authority, &stage, root_ref, snapshot).is_err() {
            return Err(abort_restore_staging(authority, &staged, Some(&stage)));
        }
        let stage_identity = match directory_identity(&stage) {
            Ok(identity) => identity,
            Err(_) => {
                return Err(abort_restore_staging(authority, &staged, Some(&stage)));
            }
        };
        let backup = match unique_absent_sibling(&state_roots, ".aureline-rollback-backup") {
            Ok(backup) => backup,
            Err(_) => {
                return Err(abort_restore_staging(authority, &staged, Some(&stage)));
            }
        };
        staged.push(StagedRootRestore {
            target,
            stage,
            backup,
            target_identity,
            stage_identity,
        });
    }

    let mut installed = 0usize;
    for (index, item) in staged.iter().enumerate() {
        if let Err(install_error) =
            install_staged_root(authority, &state_roots, &state_roots_identity, item)
        {
            let current_recovery_failed = matches!(
                &install_error,
                RollbackDrillError::RestoreRecoveryRequired { .. }
            );
            let prior_recovery = rollback_installed_roots(authority, &staged[..installed]);
            let stage_cleanup = if current_recovery_failed {
                cleanup_staged_roots(authority, &staged[index.saturating_add(1)..])
            } else {
                cleanup_staged_roots(authority, &staged[index..])
            };
            if current_recovery_failed || prior_recovery.is_err() || stage_cleanup.is_err() {
                return Err(restore_recovery_required(&staged));
            }
            return Err(install_error);
        }
        installed = installed.saturating_add(1);
    }

    for item in &staged {
        if remove_tree_bounded(authority, &item.backup).is_err() {
            return Err(RollbackDrillError::RecoverableCleanupPending {
                retained_backup_count: retained_backup_count(&staged),
            });
        }
    }
    if sync_directory(&state_roots).is_err() {
        return Err(restore_recovery_required(&staged));
    }
    Ok(())
}

fn install_staged_root(
    authority: &SyntheticAuthority,
    state_roots: &Path,
    state_roots_identity: &ObjectIdentity,
    item: &StagedRootRestore,
) -> Result<(), RollbackDrillError> {
    revalidate_canonical_authority(authority)?;
    if directory_identity(state_roots)? != *state_roots_identity
        || directory_identity(&item.target)? != item.target_identity
        || directory_identity(&item.stage)? != item.stage_identity
    {
        return unsafe_state_root("restore authority changed before atomic install");
    }
    match fs::symlink_metadata(&item.backup) {
        Ok(_) => return unsafe_state_root("restore backup destination already exists"),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => return Err(io_error(&item.backup, err)),
    }

    fs::rename(&item.target, &item.backup).map_err(|err| io_error(&item.target, err))?;
    if take_restore_failure_after_quarantine() {
        return if recover_current_root(item, false).is_ok() {
            Err(RollbackDrillError::Io {
                path: redacted_error_path(),
                detail: "injected restore failure".to_string(),
            })
        } else {
            Err(RollbackDrillError::RestoreRecoveryRequired {
                retained_backup_count: retained_restore_artifact_count(item),
            })
        };
    }
    let quarantined_identity = directory_identity(&item.backup);
    if !matches!(quarantined_identity, Ok(ref identity) if identity == &item.target_identity) {
        let recovered = recover_current_root(item, false);
        return if recovered.is_ok() {
            unsafe_state_root("restore target identity changed during quarantine")
        } else {
            Err(RollbackDrillError::RestoreRecoveryRequired {
                retained_backup_count: retained_restore_artifact_count(item),
            })
        };
    }

    if let Err(error) = fs::rename(&item.stage, &item.target) {
        let recovered = recover_current_root(item, false);
        return if recovered.is_ok() {
            Err(io_error(&item.target, error))
        } else {
            Err(RollbackDrillError::RestoreRecoveryRequired {
                retained_backup_count: retained_restore_artifact_count(item),
            })
        };
    }
    match directory_identity(&item.target) {
        Ok(identity) if identity == item.stage_identity => Ok(()),
        _ => {
            if recover_current_root(item, true).is_ok() {
                unsafe_state_root("restored target identity changed during install")
            } else {
                Err(RollbackDrillError::RestoreRecoveryRequired {
                    retained_backup_count: retained_restore_artifact_count(item),
                })
            }
        }
    }
}

fn recover_current_root(
    item: &StagedRootRestore,
    stage_was_installed: bool,
) -> Result<(), RollbackDrillError> {
    if stage_was_installed {
        match fs::symlink_metadata(&item.stage) {
            Ok(_) => return unsafe_state_root("restore staging path was unexpectedly occupied"),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => return Err(io_error(&item.stage, err)),
        }
        fs::rename(&item.target, &item.stage).map_err(|err| io_error(&item.target, err))?;
    }
    if let Err(error) = fs::rename(&item.backup, &item.target) {
        if stage_was_installed {
            let _ = fs::rename(&item.stage, &item.target);
        }
        return Err(io_error(&item.target, error));
    }
    if directory_identity(&item.target)? != item.target_identity {
        return unsafe_state_root("recovered target identity does not match the quarantined root");
    }
    Ok(())
}

fn create_unique_staging_directory(
    authority: &SyntheticAuthority,
    parent: &Path,
    prefix: &str,
    index: usize,
) -> Result<PathBuf, RollbackDrillError> {
    verify_path_if_present(authority, parent, ExpectedPathKind::Directory)?;
    for attempt in 0..32u32 {
        let path = parent.join(format!("{prefix}-{}-{index}-{attempt}", now_nanos()));
        match create_one_directory(&path) {
            Ok(()) => {
                verify_path_if_present(authority, &path, ExpectedPathKind::Directory)?;
                return Ok(path);
            }
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(err) => return Err(io_error(&path, err)),
        }
    }
    Err(RollbackDrillError::Io {
        path: redacted_error_path(),
        detail: "restore staging namespace is exhausted".to_string(),
    })
}

fn materialize_snapshot_root(
    authority: &SyntheticAuthority,
    stage: &Path,
    root_ref: &str,
    snapshot: &RollbackDrillPreStateSnapshot,
) -> Result<(), RollbackDrillError> {
    for entry in snapshot
        .entries
        .iter()
        .filter(|entry| entry.root_ref == root_ref)
    {
        let path = join_relative(stage, &entry.relative_path)?;
        match entry.entry_kind {
            RollbackDrillEntryKind::Directory => create_secure_directory(authority, &path)?,
            RollbackDrillEntryKind::File => {
                atomic_write_bounded(authority, &path, &entry.contents)?
            }
        }
    }
    Ok(())
}

fn rollback_installed_roots(
    authority: &SyntheticAuthority,
    installed: &[StagedRootRestore],
) -> Result<(), RollbackDrillError> {
    for item in installed.iter().rev() {
        recover_current_root(item, true)?;
        remove_tree_bounded(authority, &item.stage)?;
    }
    Ok(())
}

fn cleanup_staged_roots(
    authority: &SyntheticAuthority,
    staged: &[StagedRootRestore],
) -> Result<(), RollbackDrillError> {
    for item in staged {
        remove_tree_bounded(authority, &item.stage)?;
    }
    Ok(())
}

fn abort_restore_staging(
    authority: &SyntheticAuthority,
    staged: &[StagedRootRestore],
    current_stage: Option<&Path>,
) -> RollbackDrillError {
    if let Some(stage) = current_stage {
        let _ = remove_tree_bounded(authority, stage);
    }
    let _ = cleanup_staged_roots(authority, staged);
    let retained_current = current_stage.map_or(0, retained_artifact_path_count);
    RollbackDrillError::RestoreRecoveryRequired {
        retained_backup_count: retained_cleanup_count(staged).saturating_add(retained_current),
    }
}

fn restore_recovery_required(staged: &[StagedRootRestore]) -> RollbackDrillError {
    RollbackDrillError::RestoreRecoveryRequired {
        retained_backup_count: retained_cleanup_count(staged),
    }
}

fn retained_artifact_path_count(path: &Path) -> usize {
    usize::from(fs::symlink_metadata(path).is_ok())
}

fn retained_restore_artifact_count(item: &StagedRootRestore) -> usize {
    retained_artifact_path_count(&item.backup)
        .saturating_add(retained_artifact_path_count(&item.stage))
}

fn retained_backup_count(staged: &[StagedRootRestore]) -> usize {
    staged
        .iter()
        .map(|item| retained_artifact_path_count(&item.backup))
        .sum()
}

fn retained_cleanup_count(staged: &[StagedRootRestore]) -> usize {
    staged.iter().map(retained_restore_artifact_count).sum()
}

fn remove_tree_bounded(
    authority: &SyntheticAuthority,
    path: &Path,
) -> Result<(), RollbackDrillError> {
    let mut removed = 0usize;
    remove_tree_bounded_inner(authority, path, 0, &mut removed)
}

fn remove_tree_bounded_inner(
    authority: &SyntheticAuthority,
    path: &Path,
    depth: usize,
    removed: &mut usize,
) -> Result<(), RollbackDrillError> {
    if depth > ROLLBACK_DRILL_MAX_DEPTH {
        return resource_limit("cleanup depth", ROLLBACK_DRILL_MAX_DEPTH);
    }
    ensure_lexically_contained(authority, path)?;
    let parent = path
        .parent()
        .ok_or_else(|| unsafe_state_root_error("cleanup target has no parent"))?;
    verify_path_if_present(authority, parent, ExpectedPathKind::Directory)?;
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(err) => return Err(io_error(path, err)),
    };
    if !metadata_is_on_authority_filesystem(authority, &metadata) {
        return unsafe_state_root("cleanup target crosses the synthetic authority filesystem");
    }
    *removed = removed.saturating_add(1);
    if *removed > ROLLBACK_DRILL_MAX_ENTRIES.saturating_add(ROLLBACK_DRILL_MAX_ROOTS) {
        return resource_limit(
            "cleanup entries",
            ROLLBACK_DRILL_MAX_ENTRIES.saturating_add(ROLLBACK_DRILL_MAX_ROOTS),
        );
    }
    if metadata_is_redirect(&metadata) {
        return if metadata.is_dir() {
            fs::remove_dir(path).map_err(|err| io_error(path, err))
        } else {
            fs::remove_file(path).map_err(|err| io_error(path, err))
        };
    }
    if metadata.is_file() {
        return fs::remove_file(path).map_err(|err| io_error(path, err));
    }
    if !metadata.is_dir() {
        return unsafe_state_root("cleanup encountered an unsupported entry kind");
    }

    let identity = object_identity(&metadata);
    let mut children = Vec::new();
    for child in fs::read_dir(path).map_err(|err| io_error(path, err))? {
        let child = child.map_err(|err| io_error(path, err))?;
        if children.len() >= ROLLBACK_DRILL_MAX_DIRECTORY_ENTRIES {
            return resource_limit(
                "cleanup directory entries",
                ROLLBACK_DRILL_MAX_DIRECTORY_ENTRIES,
            );
        }
        children.push(child.path());
    }
    children.sort();
    for child in children {
        verify_path_if_present(authority, path, ExpectedPathKind::Directory)?;
        remove_tree_bounded_inner(authority, &child, depth.saturating_add(1), removed)?;
    }
    let after = fs::symlink_metadata(path).map_err(|err| io_error(path, err))?;
    if metadata_is_redirect(&after) || !after.is_dir() || object_identity(&after) != identity {
        return unsafe_state_root("cleanup directory identity changed");
    }
    verify_path_if_present(authority, parent, ExpectedPathKind::Directory)?;
    fs::remove_dir(path).map_err(|err| io_error(path, err))
}

fn compare_snapshots(
    pre: &RollbackDrillPreStateSnapshot,
    post: &RollbackDrillPreStateSnapshot,
    expected_deltas: &[RollbackDrillExpectedDelta],
) -> Vec<RollbackDrillDiff> {
    let pre_map = normalized_entries(pre, expected_deltas);
    let post_map = normalized_entries(post, expected_deltas);
    let keys: BTreeSet<_> = pre_map.keys().chain(post_map.keys()).cloned().collect();
    let mut diffs = Vec::new();
    for key in keys {
        match (pre_map.get(&key), post_map.get(&key)) {
            (Some(_), None) => diffs.push(diff(key, RollbackDrillDiffKind::MissingAfterRollback)),
            (None, Some(_)) => {
                diffs.push(diff(key, RollbackDrillDiffKind::UnexpectedAfterRollback))
            }
            (Some(left), Some(right)) if left.entry_kind != right.entry_kind => {
                diffs.push(diff(key, RollbackDrillDiffKind::EntryKindChanged));
            }
            (Some(left), Some(right))
                if left.entry_kind == RollbackDrillEntryKind::File
                    && left.contents != right.contents =>
            {
                diffs.push(diff(key, RollbackDrillDiffKind::ContentsChanged));
            }
            _ => {}
        }
    }
    diffs
}

fn normalized_entries<'a>(
    snapshot: &'a RollbackDrillPreStateSnapshot,
    expected_deltas: &[RollbackDrillExpectedDelta],
) -> BTreeMap<(String, String), &'a RollbackDrillEntry> {
    snapshot
        .entries
        .iter()
        .filter(|entry| !is_expected_delta(&entry.root_ref, &entry.relative_path, expected_deltas))
        .map(|entry| ((entry.root_ref.clone(), entry.relative_path.clone()), entry))
        .collect()
}

fn diff(key: (String, String), diff_kind: RollbackDrillDiffKind) -> RollbackDrillDiff {
    RollbackDrillDiff {
        root_ref: key.0,
        relative_path: key.1,
        diff_kind,
    }
}

fn target_changed(
    pre: &RollbackDrillPreStateSnapshot,
    post: &RollbackDrillPreStateSnapshot,
    plan: &RollbackDrillPlan,
) -> bool {
    let target_roots: BTreeSet<String> = plan
        .roots
        .iter()
        .filter(|root| root.role == RollbackDrillRootRole::TargetRollback)
        .map(|root| root.root_ref.clone())
        .collect();
    let pre_target = target_entries(pre, &target_roots);
    let post_target = target_entries(post, &target_roots);
    pre_target != post_target
}

fn target_entries(
    snapshot: &RollbackDrillPreStateSnapshot,
    roots: &BTreeSet<String>,
) -> BTreeMap<(String, String), RollbackDrillEntry> {
    snapshot
        .entries
        .iter()
        .filter(|entry| roots.contains(&entry.root_ref))
        .map(|entry| {
            (
                (entry.root_ref.clone(), entry.relative_path.clone()),
                entry.clone(),
            )
        })
        .collect()
}

fn is_expected_delta(
    root_ref: &str,
    relative_path: &str,
    expected_deltas: &[RollbackDrillExpectedDelta],
) -> bool {
    expected_deltas
        .iter()
        .filter(|delta| delta.root_ref == root_ref)
        .any(|delta| {
            relative_path == delta.relative_path
                || delta
                    .relative_path
                    .strip_prefix(relative_path)
                    .is_some_and(|suffix| suffix.starts_with('/'))
        })
}

fn validate_snapshot(
    path: &Path,
    plan: &RollbackDrillPlan,
    snapshot: &RollbackDrillPreStateSnapshot,
) -> Result<(), RollbackDrillError> {
    if snapshot.record_kind != ROLLBACK_DRILL_PRE_STATE_RECORD_KIND {
        return corrupted(path, "snapshot record_kind is unsupported");
    }
    if snapshot.schema_version != ROLLBACK_DRILL_SCHEMA_VERSION {
        return corrupted(path, "snapshot schema_version is unsupported");
    }
    if snapshot.drill_id != plan.drill_id {
        return corrupted(path, "snapshot drill_id does not match the active plan");
    }
    if snapshot.target_topology_row_id != plan.target_topology_row_id {
        return corrupted(
            path,
            "snapshot target_topology_row_id does not match the active plan",
        );
    }
    if snapshot.roots != plan.roots {
        return corrupted(path, "snapshot root set does not match the active plan");
    }
    if snapshot.snapshot_id.len() > ROLLBACK_DRILL_MAX_ID_BYTES
        || snapshot.captured_at.len() > ROLLBACK_DRILL_MAX_ID_BYTES
    {
        return corrupted(path, "snapshot identity fields exceed their byte limit");
    }
    if snapshot.entries.len() > ROLLBACK_DRILL_MAX_ENTRIES {
        return resource_limit("snapshot entries", ROLLBACK_DRILL_MAX_ENTRIES);
    }
    let admitted_roots: BTreeSet<&str> = plan
        .roots
        .iter()
        .map(|root| root.root_ref.as_str())
        .collect();
    let mut entry_kinds = BTreeMap::new();
    let mut total_file_bytes = 0u64;
    for entry in &snapshot.entries {
        if !admitted_roots.contains(entry.root_ref.as_str()) {
            return corrupted(path, "snapshot entry references an unplanned state root");
        }
        validate_relative_path(&entry.relative_path)
            .map_err(|_| corrupted_error("snapshot entry path is unsafe"))?;
        let depth = entry.relative_path.split('/').count();
        if depth > ROLLBACK_DRILL_MAX_DEPTH {
            return resource_limit("snapshot entry depth", ROLLBACK_DRILL_MAX_DEPTH);
        }
        if entry.relative_path.len() > ROLLBACK_DRILL_MAX_RELATIVE_PATH_BYTES {
            return resource_limit(
                "snapshot relative path bytes",
                ROLLBACK_DRILL_MAX_RELATIVE_PATH_BYTES,
            );
        }
        match entry.entry_kind {
            RollbackDrillEntryKind::Directory if !entry.contents.is_empty() => {
                return corrupted(path, "snapshot directory entry carries file contents");
            }
            RollbackDrillEntryKind::Directory => {}
            RollbackDrillEntryKind::File => {
                if entry.contents.len() as u64 > ROLLBACK_DRILL_MAX_FILE_BYTES {
                    return resource_limit_u64(
                        "snapshot file bytes",
                        ROLLBACK_DRILL_MAX_FILE_BYTES,
                    );
                }
                total_file_bytes = total_file_bytes.saturating_add(entry.contents.len() as u64);
                if total_file_bytes > ROLLBACK_DRILL_MAX_TOTAL_FILE_BYTES {
                    return resource_limit_u64(
                        "snapshot aggregate file bytes",
                        ROLLBACK_DRILL_MAX_TOTAL_FILE_BYTES,
                    );
                }
            }
        }
        let key = (entry.root_ref.as_str(), entry.relative_path.as_str());
        if entry_kinds.insert(key, entry.entry_kind).is_some() {
            return corrupted(path, "snapshot contains duplicate entry paths");
        }
    }
    for (root_ref, relative_path) in entry_kinds.keys() {
        let mut ancestor = String::new();
        let components: Vec<_> = relative_path.split('/').collect();
        for component in components.iter().take(components.len().saturating_sub(1)) {
            if !ancestor.is_empty() {
                ancestor.push('/');
            }
            ancestor.push_str(component);
            if entry_kinds.get(&(*root_ref, ancestor.as_str()))
                == Some(&RollbackDrillEntryKind::File)
            {
                return corrupted(path, "snapshot places an entry below a regular file");
            }
        }
    }
    let expected_digest = digest_snapshot(snapshot);
    if snapshot.entry_digest != expected_digest {
        return corrupted(
            path,
            "snapshot entry digest does not match captured contents",
        );
    }
    if snapshot.entries.is_empty() {
        return Err(RollbackDrillError::PreStateNotCaptured {
            drill_id: plan.drill_id.clone(),
        });
    }
    Ok(())
}

fn corrupted<T>(path: &Path, detail: impl Into<String>) -> Result<T, RollbackDrillError> {
    let _ = path;
    Err(RollbackDrillError::CorruptedPreStateSnapshot {
        path: redacted_error_path(),
        detail: detail.into(),
    })
}

fn digest_snapshot(snapshot: &RollbackDrillPreStateSnapshot) -> String {
    let mut hasher = Fnv1a64::default();
    hasher.update(snapshot.drill_id.as_bytes());
    hasher.update(snapshot.target_topology_row_id.as_bytes());
    for root in &snapshot.roots {
        hasher.update(root.root_ref.as_bytes());
        hasher.update(format!("{:?}", root.role).as_bytes());
        hasher.update(root.topology_row_id.as_bytes());
        hasher.update(format!("{:?}", root.channel_class).as_bytes());
    }
    for entry in &snapshot.entries {
        hasher.update(entry.root_ref.as_bytes());
        hasher.update(entry.relative_path.as_bytes());
        hasher.update(format!("{:?}", entry.entry_kind).as_bytes());
        hasher.update(&entry.contents);
    }
    format!("fnv1a64:{:016x}", hasher.finish())
}

#[derive(Debug, Clone, Copy)]
struct Fnv1a64(u64);

impl Default for Fnv1a64 {
    fn default() -> Self {
        Self(0xcbf29ce484222325)
    }
}

impl Fnv1a64 {
    fn update(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(0x100000001b3);
        }
        self.0 ^= 0xff;
        self.0 = self.0.wrapping_mul(0x100000001b3);
    }

    const fn finish(self) -> u64 {
        self.0
    }
}

fn validate_drill_plan_bounds(plan: &RollbackDrillPlan) -> Result<(), RollbackDrillError> {
    if !is_safe_identifier(&plan.drill_id)
        || plan.drill_id.len() > ROLLBACK_DRILL_MAX_DRILL_ID_BYTES
        || !is_safe_identifier(&plan.target_topology_row_id)
    {
        return Err(RollbackDrillError::InvalidDrillPlan {
            detail: "drill identifiers are empty, unsafe, or oversized".to_string(),
        });
    }
    if plan.roots.is_empty() || plan.roots.len() > ROLLBACK_DRILL_MAX_ROOTS {
        return resource_limit("planned state roots", ROLLBACK_DRILL_MAX_ROOTS);
    }
    if plan.expected_deltas.len() > ROLLBACK_DRILL_MAX_EXPECTED_DELTAS {
        return resource_limit("expected delta rows", ROLLBACK_DRILL_MAX_EXPECTED_DELTAS);
    }

    let mut roots = BTreeSet::new();
    let mut has_target = false;
    for root in &plan.roots {
        safe_root_segment(&root.root_ref)?;
        if !is_safe_identifier(&root.topology_row_id) {
            return Err(RollbackDrillError::InvalidDrillPlan {
                detail: "root topology identifier is unsafe or oversized".to_string(),
            });
        }
        if !roots.insert(root.root_ref.as_str()) {
            return Err(RollbackDrillError::InvalidDrillPlan {
                detail: "the same state root appears more than once".to_string(),
            });
        }
        has_target = has_target || root.role == RollbackDrillRootRole::TargetRollback;
    }
    if !has_target {
        return Err(RollbackDrillError::InvalidDrillPlan {
            detail: "drill plan has no target rollback roots".to_string(),
        });
    }
    let target_roots: BTreeSet<&str> = plan
        .roots
        .iter()
        .filter(|root| root.role == RollbackDrillRootRole::TargetRollback)
        .map(|root| root.root_ref.as_str())
        .collect();
    let mut deltas = BTreeSet::new();
    for delta in &plan.expected_deltas {
        if !roots.contains(delta.root_ref.as_str())
            || !target_roots.contains(delta.root_ref.as_str())
        {
            return Err(RollbackDrillError::InvalidDrillPlan {
                detail: "expected delta must reference a target rollback root".to_string(),
            });
        }
        validate_relative_path(&delta.relative_path).map_err(|_| {
            RollbackDrillError::InvalidDrillPlan {
                detail: "expected delta path is unsafe or oversized".to_string(),
            }
        })?;
        if !deltas.insert((delta.root_ref.as_str(), delta.relative_path.as_str())) {
            return Err(RollbackDrillError::InvalidDrillPlan {
                detail: "expected delta rows must be unique".to_string(),
            });
        }
    }
    Ok(())
}

fn is_safe_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= ROLLBACK_DRILL_MAX_ID_BYTES
        && !value.ends_with('.')
        && !is_windows_reserved_segment(value)
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '.' || ch == '_' || ch == '-')
}

fn is_windows_reserved_segment(value: &str) -> bool {
    let stem = value.split('.').next().unwrap_or(value);
    let upper = stem.to_ascii_uppercase();
    matches!(upper.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || upper
            .strip_prefix("COM")
            .or_else(|| upper.strip_prefix("LPT"))
            .is_some_and(|suffix| suffix.len() == 1 && matches!(suffix.as_bytes()[0], b'1'..=b'9'))
}

fn validate_json_shape(value: &serde_json::Value) -> Result<(), RollbackDrillError> {
    let mut nodes = 0usize;
    let mut stack = vec![(value, 0usize, false)];
    while let Some((value, depth, is_byte_array)) = stack.pop() {
        if is_byte_array {
            let serde_json::Value::Array(items) = value else {
                return Err(corrupted_error("snapshot file contents are malformed"));
            };
            if items.len() as u64 > ROLLBACK_DRILL_MAX_FILE_BYTES {
                return resource_limit_u64(
                    "snapshot JSON file-content entries",
                    ROLLBACK_DRILL_MAX_FILE_BYTES,
                );
            }
            if items
                .iter()
                .any(|item| !matches!(item.as_u64(), Some(byte) if byte <= u8::MAX as u64))
            {
                return Err(corrupted_error(
                    "snapshot file contents contain a non-byte value",
                ));
            }
            continue;
        }
        nodes = nodes.saturating_add(1);
        if nodes > ROLLBACK_DRILL_MAX_JSON_NODES {
            return resource_limit("snapshot JSON nodes", ROLLBACK_DRILL_MAX_JSON_NODES);
        }
        if depth > ROLLBACK_DRILL_MAX_DEPTH {
            return resource_limit("snapshot JSON depth", ROLLBACK_DRILL_MAX_DEPTH);
        }
        match value {
            serde_json::Value::String(text) => {
                if text.len() > ROLLBACK_DRILL_MAX_TEXT_BYTES {
                    return resource_limit(
                        "snapshot JSON string bytes",
                        ROLLBACK_DRILL_MAX_TEXT_BYTES,
                    );
                }
            }
            serde_json::Value::Array(items) => {
                if items.len() > ROLLBACK_DRILL_MAX_JSON_COLLECTION_ENTRIES {
                    return resource_limit(
                        "snapshot JSON array entries",
                        ROLLBACK_DRILL_MAX_JSON_COLLECTION_ENTRIES,
                    );
                }
                stack.extend(
                    items
                        .iter()
                        .map(|item| (item, depth.saturating_add(1), false)),
                );
            }
            serde_json::Value::Object(object) => {
                if object.len() > ROLLBACK_DRILL_MAX_JSON_COLLECTION_ENTRIES {
                    return resource_limit(
                        "snapshot JSON object entries",
                        ROLLBACK_DRILL_MAX_JSON_COLLECTION_ENTRIES,
                    );
                }
                for (key, item) in object {
                    if key.len() > ROLLBACK_DRILL_MAX_ID_BYTES {
                        return resource_limit(
                            "snapshot JSON key bytes",
                            ROLLBACK_DRILL_MAX_ID_BYTES,
                        );
                    }
                    stack.push((item, depth.saturating_add(1), key == "contents"));
                }
            }
            serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::Number(_) => {
            }
        }
    }
    Ok(())
}

fn safe_error_ref(value: &str) -> String {
    if is_safe_identifier(value) {
        value.to_string()
    } else {
        "<invalid-state-root-ref>".to_string()
    }
}

fn redacted_error_path() -> PathBuf {
    PathBuf::from("<synthetic-rollback-tree>")
}

fn unsafe_state_root<T>(detail: &'static str) -> Result<T, RollbackDrillError> {
    Err(unsafe_state_root_error(detail))
}

fn unsafe_state_root_error(detail: &'static str) -> RollbackDrillError {
    RollbackDrillError::UnsafeStateRoot {
        root_ref: "<synthetic-state-root>".to_string(),
        detail: detail.to_string(),
    }
}

fn corrupted_error(detail: &'static str) -> RollbackDrillError {
    RollbackDrillError::CorruptedPreStateSnapshot {
        path: redacted_error_path(),
        detail: detail.to_string(),
    }
}

fn resource_limit<T>(resource: &'static str, limit: usize) -> Result<T, RollbackDrillError> {
    resource_limit_u64(resource, limit as u64)
}

fn resource_limit_u64<T>(resource: &'static str, limit: u64) -> Result<T, RollbackDrillError> {
    Err(RollbackDrillError::ResourceLimitExceeded { resource, limit })
}

fn serialization_error_class(_error: &serde_json::Error) -> String {
    "JSON encoding failed".to_string()
}

fn io_error_class(kind: std::io::ErrorKind) -> &'static str {
    match kind {
        std::io::ErrorKind::NotFound => "not found",
        std::io::ErrorKind::PermissionDenied => "permission denied",
        std::io::ErrorKind::AlreadyExists => "already exists",
        std::io::ErrorKind::InvalidInput => "invalid input",
        std::io::ErrorKind::InvalidData => "invalid data",
        std::io::ErrorKind::UnexpectedEof => "unexpected end of input",
        std::io::ErrorKind::WriteZero => "short write",
        std::io::ErrorKind::Interrupted => "interrupted",
        std::io::ErrorKind::OutOfMemory => "out of memory",
        _ => "filesystem operation failed",
    }
}

fn safe_root_segment(root_ref: &str) -> Result<String, RollbackDrillError> {
    if root_ref.is_empty() || root_ref.len() > ROLLBACK_DRILL_MAX_FILESYSTEM_SEGMENT_BYTES {
        return unsafe_state_root("root ref is empty or exceeds its byte limit");
    }
    if root_ref == "." || root_ref == ".." || root_ref.contains('/') || root_ref.contains('\\') {
        return Err(RollbackDrillError::UnsafeStateRoot {
            root_ref: safe_error_ref(root_ref),
            detail: "root ref must be a single path segment".to_string(),
        });
    }
    if !root_ref
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '.' || ch == '_' || ch == '-')
    {
        return Err(RollbackDrillError::UnsafeStateRoot {
            root_ref: safe_error_ref(root_ref),
            detail: "root ref contains unsupported characters".to_string(),
        });
    }
    if is_windows_reserved_segment(root_ref) || root_ref.ends_with('.') {
        return unsafe_state_root("root ref is reserved by a supported platform");
    }
    Ok(root_ref.to_string())
}

fn join_relative(root_path: &Path, relative_path: &str) -> Result<PathBuf, RollbackDrillError> {
    let _ = root_path;
    validate_relative_path(relative_path)?;
    Ok(root_path.join(relative_path))
}

fn relative_path(root_path: &Path, current_path: &Path) -> Result<String, RollbackDrillError> {
    let relative = current_path
        .strip_prefix(root_path)
        .map_err(|_| unsafe_state_root_error("walked path escaped its state root"))?;
    let mut components = Vec::new();
    for component in relative.components() {
        let std::path::Component::Normal(component) = component else {
            return unsafe_state_root("walked path contains an unsafe component");
        };
        let component = component
            .to_str()
            .ok_or_else(|| unsafe_state_root_error("walked path is not valid UTF-8"))?;
        components.push(component);
    }
    let relative = components.join("/");
    validate_relative_path(&relative)?;
    Ok(relative)
}

fn validate_relative_path(relative_path: &str) -> Result<(), RollbackDrillError> {
    if relative_path.is_empty()
        || relative_path.len() > ROLLBACK_DRILL_MAX_RELATIVE_PATH_BYTES
        || relative_path.starts_with('/')
        || relative_path.contains('\\')
        || relative_path.split('/').count() > ROLLBACK_DRILL_MAX_DEPTH
        || relative_path.split('/').any(|component| {
            component.is_empty()
                || component == "."
                || component == ".."
                || component.len() > ROLLBACK_DRILL_MAX_FILESYSTEM_SEGMENT_BYTES
                || component.ends_with('.')
                || is_windows_reserved_segment(component)
                || component.chars().any(char::is_control)
        })
    {
        return unsafe_state_root("relative path is unsafe or exceeds its bounds");
    }
    Ok(())
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '.' || ch == '_' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn io_error(path: &Path, err: std::io::Error) -> RollbackDrillError {
    let _ = path;
    RollbackDrillError::Io {
        path: redacted_error_path(),
        detail: io_error_class(err.kind()).to_string(),
    }
}

fn now_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}

fn push_plan_finding(
    findings: &mut Vec<UpdateRollbackValidationFinding>,
    check_id: &str,
    ref_id: &str,
    message: &str,
) {
    findings.push(UpdateRollbackValidationFinding {
        check_id: check_id.to_string(),
        ref_id: ref_id.to_string(),
        message: message.to_string(),
    });
}

fn validate_non_empty_ref(
    findings: &mut Vec<UpdateRollbackValidationFinding>,
    check_id: &str,
    value: &str,
    owner_ref: &str,
) {
    if value.trim().is_empty() {
        push_plan_finding(findings, check_id, owner_ref, "reference must not be empty");
    }
}

fn validate_exact_build_ref(
    findings: &mut Vec<UpdateRollbackValidationFinding>,
    check_id: &str,
    value: &str,
    owner_ref: &str,
) {
    validate_non_empty_ref(findings, check_id, value, owner_ref);
    if !value.starts_with("build-id:aureline:") {
        push_plan_finding(
            findings,
            check_id,
            owner_ref,
            "exact-build identity refs must use the build-id:aureline namespace",
        );
    }
}

#[cfg(test)]
mod rollback_hardening_tests {
    use super::*;

    fn topology_fixture() -> InstallTopologyAlphaPacket {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/install/topology_alpha/install_topology_alpha_packet.json");
        let bytes = fs::read(path).expect("read topology fixture");
        serde_json::from_slice(&bytes).expect("parse topology fixture")
    }

    fn drill_plan() -> RollbackDrillPlan {
        RollbackDrillPlan::portable_side_by_side(
            &topology_fixture(),
            "install.topology.windows.preview.side_by_side",
            "install.topology.windows.portable.stable",
        )
        .expect("build rollback drill plan")
    }

    #[test]
    fn pending_write_guard_scrubs_bytes_after_injected_post_write_failure() {
        let authority_root = tempfile::tempdir().expect("authority tempdir");
        let outside = tempfile::tempdir().expect("outside tempdir");
        let outside_sentinel = outside.path().join("outside-sentinel.txt");
        fs::write(&outside_sentinel, b"outside-safe").expect("write outside sentinel");
        let driver = RollbackDrillDriver::new(authority_root.path());
        let authority = driver.authority().expect("initialize authority");
        let target = authority
            .canonical_root
            .join(SYNTHETIC_AUTHORITY_DIRECTORY)
            .join("pending-sensitive-state.json");

        FAIL_ATOMIC_WRITE_AFTER_SYNC.with(|failpoint| failpoint.set(true));
        let error = atomic_write_bounded(authority, &target, b"private-synthetic-state")
            .expect_err("injected post-write failure must fail");
        assert!(matches!(error, RollbackDrillError::Io { .. }));
        assert!(!target.exists(), "failed target must not remain installed");

        for entry in
            fs::read_dir(target.parent().expect("target parent")).expect("read authority directory")
        {
            let entry = entry.expect("authority entry");
            if entry.file_name().as_os_str() == std::ffi::OsStr::new(SYNTHETIC_AUTHORITY_MARKER) {
                continue;
            }
            let metadata = fs::symlink_metadata(entry.path()).expect("entry metadata");
            if metadata.is_file() {
                assert_eq!(metadata.len(), 0, "failed pending file must be scrubbed");
            }
        }
        assert_eq!(
            fs::read(&outside_sentinel).expect("read outside sentinel"),
            b"outside-safe"
        );
    }

    #[cfg(unix)]
    #[test]
    fn installed_write_remains_after_parent_sync_failure() {
        let authority_root = tempfile::tempdir().expect("authority tempdir");
        let driver = RollbackDrillDriver::new(authority_root.path());
        let authority = driver.authority().expect("initialize authority");
        let target = authority
            .canonical_root
            .join(SYNTHETIC_AUTHORITY_DIRECTORY)
            .join("durable-install-state.json");
        atomic_write_bounded(authority, &target, b"prior-state").expect("write prior state");

        FAIL_ATOMIC_WRITE_PARENT_SYNC.with(|failpoint| failpoint.set(true));
        let error = atomic_write_bounded(authority, &target, b"installed-state")
            .expect_err("injected parent sync failure must fail honestly");
        assert!(matches!(error, RollbackDrillError::Io { .. }));
        assert_eq!(
            fs::read(&target).expect("read installed target"),
            b"installed-state",
            "an installed replacement cannot be safely removed after its predecessor is gone"
        );
    }

    #[cfg(unix)]
    #[test]
    fn pending_write_guard_scrubs_moved_parent_inode_without_touching_replacement() {
        let tempdir = tempfile::tempdir().expect("tempdir");
        let parent = tempdir.path().join("pending-parent");
        let moved_parent = tempdir.path().join("moved-pending-parent");
        fs::create_dir(&parent).expect("create pending parent");
        let pending_path = parent.join("private-write.tmp");
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&pending_path)
            .expect("create pending file");
        file.write_all(b"private-synthetic-state")
            .expect("write pending bytes");
        file.sync_all().expect("sync pending bytes");
        let parent_identity = directory_identity(&parent).expect("pending parent identity");
        let pending = PendingFile::new(file, pending_path.clone(), parent.clone(), parent_identity);

        fs::rename(&parent, &moved_parent).expect("move original parent");
        fs::create_dir(&parent).expect("create replacement parent");
        let replacement_sentinel = parent.join("replacement-sentinel.txt");
        fs::write(&replacement_sentinel, b"replacement-safe").expect("write replacement sentinel");
        let outside = tempfile::tempdir().expect("outside tempdir");
        let outside_sentinel = outside.path().join("outside-sentinel.txt");
        fs::write(&outside_sentinel, b"outside-safe").expect("write outside sentinel");

        drop(pending);

        let moved_pending = moved_parent.join("private-write.tmp");
        assert_eq!(
            fs::read(&moved_pending).expect("read moved pending inode"),
            b"",
            "the open inode must be scrubbed even after its parent moves"
        );
        assert_eq!(
            fs::read(&replacement_sentinel).expect("read replacement sentinel"),
            b"replacement-safe"
        );
        assert_eq!(
            fs::read(&outside_sentinel).expect("read outside sentinel"),
            b"outside-safe"
        );
        assert!(
            !parent.join("private-write.tmp").exists(),
            "cleanup must not create or remove a path in the replacement parent"
        );
    }

    #[cfg(unix)]
    #[test]
    fn partial_synthetic_update_failure_restores_captured_pre_state() {
        let tempdir = tempfile::tempdir().expect("tempdir");
        let driver = RollbackDrillDriver::new(tempdir.path());
        let plan = drill_plan();
        driver
            .seed_synthetic_state_tree(&plan)
            .expect("seed synthetic tree");
        let pre_state = driver.capture_pre_state(&plan).expect("capture pre-state");

        FAIL_SYNTHETIC_UPDATE_AFTER_FIRST_WRITE.with(|failpoint| failpoint.set(true));
        let error = driver
            .run_from_captured_pre_state(&plan)
            .expect_err("injected partial update must fail after recovery");
        assert!(matches!(error, RollbackDrillError::Io { .. }));

        let post_state = driver
            .capture_snapshot(&plan)
            .expect("capture recovered state");
        assert!(
            compare_snapshots(&pre_state, &post_state, &[]).is_empty(),
            "every partial mutation must be restored before the update error is returned"
        );
        let state_roots = driver
            .authority()
            .expect("authority")
            .canonical_root
            .join("state-roots");
        for entry in fs::read_dir(state_roots).expect("read state roots") {
            let name = entry
                .expect("state-root entry")
                .file_name()
                .to_string_lossy()
                .into_owned();
            assert!(!name.starts_with(".aureline-rollback-stage"));
            assert!(!name.starts_with(".aureline-rollback-backup"));
        }
    }

    #[cfg(unix)]
    #[test]
    fn committed_first_write_sync_failure_restores_captured_pre_state() {
        let tempdir = tempfile::tempdir().expect("tempdir");
        let driver = RollbackDrillDriver::new(tempdir.path());
        let plan = drill_plan();
        driver
            .seed_synthetic_state_tree(&plan)
            .expect("seed synthetic tree");
        let pre_state = driver.capture_pre_state(&plan).expect("capture pre-state");

        FAIL_ATOMIC_WRITE_PARENT_SYNC.with(|failpoint| failpoint.set(true));
        let error = driver
            .run_from_captured_pre_state(&plan)
            .expect_err("committed first-write sync failure must fail after recovery");
        assert!(matches!(error, RollbackDrillError::Io { .. }));

        let post_state = driver
            .capture_snapshot(&plan)
            .expect("capture recovered state");
        assert!(
            compare_snapshots(&pre_state, &post_state, &[]).is_empty(),
            "a write installed before parent sync fails must still trigger exact pre-state restore"
        );
    }

    #[cfg(unix)]
    #[test]
    fn injected_post_quarantine_failure_recovers_transaction_input_layout() {
        let tempdir = tempfile::tempdir().expect("tempdir");
        let driver = RollbackDrillDriver::new(tempdir.path());
        let plan = drill_plan();
        driver
            .seed_synthetic_state_tree(&plan)
            .expect("seed synthetic tree");

        FAIL_RESTORE_AFTER_QUARANTINE.with(|failpoint| failpoint.set(true));
        let error = driver
            .run(&plan)
            .expect_err("injected restore failure must fail honestly");
        assert!(matches!(
            error,
            RollbackDrillError::RestoreRecoveryRequired {
                retained_backup_count: 0
            }
        ));

        let state_roots = driver
            .authority()
            .expect("authority")
            .canonical_root
            .join("state-roots");
        for entry in fs::read_dir(&state_roots).expect("read state roots") {
            let name = entry
                .expect("state-root entry")
                .file_name()
                .to_string_lossy()
                .into_owned();
            assert!(!name.starts_with(".aureline-rollback-stage"));
            assert!(!name.starts_with(".aureline-rollback-backup"));
        }
        for root_ref in plan.target_root_refs() {
            let root = driver.state_root_path(root_ref).expect("target state root");
            assert!(root.is_dir(), "target state root must remain available");
            let current_build = fs::read_to_string(root.join("build").join("current.txt"))
                .expect("read recovered transaction-input build marker");
            assert!(
                current_build.starts_with("candidate-build:"),
                "the failed restore must recover its updated transaction input"
            );
        }
    }

    #[test]
    fn windows_reparse_attribute_is_never_treated_as_a_regular_entry() {
        assert!(windows_file_attributes_include_reparse_point(0x400));
        assert!(windows_file_attributes_include_reparse_point(0x410));
        assert!(!windows_file_attributes_include_reparse_point(0x10));
    }

    #[test]
    fn plan_rejects_identifiers_that_cannot_fit_generated_path_segments() {
        let mut oversized_root = drill_plan();
        oversized_root.roots[0].root_ref =
            "r".repeat(ROLLBACK_DRILL_MAX_FILESYSTEM_SEGMENT_BYTES + 1);
        assert!(matches!(
            validate_drill_plan_bounds(&oversized_root),
            Err(RollbackDrillError::UnsafeStateRoot { .. })
        ));

        let mut oversized_drill = drill_plan();
        oversized_drill.drill_id = "d".repeat(ROLLBACK_DRILL_MAX_DRILL_ID_BYTES + 1);
        assert!(matches!(
            validate_drill_plan_bounds(&oversized_drill),
            Err(RollbackDrillError::InvalidDrillPlan { .. })
        ));
    }
}
