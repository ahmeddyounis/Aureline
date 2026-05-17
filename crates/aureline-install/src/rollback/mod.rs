//! Synthetic rollback drill driver for install-topology state roots.
//!
//! The driver walks only caller-provided synthetic roots. It captures a
//! pre-rollback snapshot, applies a bounded fake update to the target install
//! roots, restores those roots from the snapshot, and compares every walked
//! root against the captured state while ignoring declared post-rollback
//! evidence deltas.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};
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
        /// Expected synthetic path.
        path: PathBuf,
    },
    /// Filesystem I/O failed while reading or writing the synthetic tree.
    Io {
        /// Path involved in the I/O operation.
        path: PathBuf,
        /// Redaction-safe I/O error detail.
        detail: String,
    },
    /// Snapshot serialization failed.
    Serialization {
        /// Redaction-safe serialization error detail.
        detail: String,
    },
    /// The captured pre-state snapshot is unreadable or fails integrity checks.
    CorruptedPreStateSnapshot {
        /// Snapshot path that failed.
        path: PathBuf,
        /// Redaction-safe failure detail.
        detail: String,
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
                write!(f, "missing install topology row: {topology_row_id}")
            }
            Self::InvalidDrillPlan { detail } => write!(f, "invalid rollback drill plan: {detail}"),
            Self::UnsafeStateRoot { root_ref, detail } => {
                write!(f, "unsafe rollback drill state root {root_ref}: {detail}")
            }
            Self::MissingStateRoot { root_ref, path } => {
                write!(
                    f,
                    "rollback drill state root {root_ref} is missing at {}",
                    path.display()
                )
            }
            Self::Io { path, detail } => write!(
                f,
                "rollback drill I/O failed at {}: {detail}",
                path.display()
            ),
            Self::Serialization { detail } => {
                write!(f, "rollback drill serialization failed: {detail}")
            }
            Self::CorruptedPreStateSnapshot { path, detail } => write!(
                f,
                "rollback drill pre-state snapshot is corrupted at {}: {detail}",
                path.display()
            ),
            Self::PreStateNotCaptured { drill_id } => {
                write!(f, "rollback drill {drill_id} captured no pre-state")
            }
            Self::SyntheticUpdateDidNotTouchTarget { drill_id } => write!(
                f,
                "rollback drill {drill_id} synthetic update did not touch target roots"
            ),
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
}

impl RollbackDrillDriver {
    /// Creates a driver rooted at a synthetic filesystem tree.
    pub fn new(synthetic_tree_root: impl AsRef<Path>) -> Self {
        Self {
            synthetic_tree_root: synthetic_tree_root.as_ref().to_path_buf(),
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
        Ok(self
            .synthetic_tree_root
            .join("state-roots")
            .join(safe_root_segment(root_ref)?))
    }

    /// Returns the pre-state snapshot path for `drill_id`.
    pub fn pre_state_snapshot_path(&self, drill_id: &str) -> PathBuf {
        self.synthetic_tree_root
            .join(".rollback_drill")
            .join(format!("{}.pre_state.json", sanitize_id(drill_id)))
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
        let mut roots = Vec::new();
        for root in &plan.roots {
            let path = self.state_root_path(&root.root_ref)?;
            create_dir_all(&path).map_err(|err| io_error(&path, err))?;
            write_bytes(
                &path.join("state-root.json"),
                synthetic_state_root_body(root).as_bytes(),
            )?;
            write_bytes(
                &path.join("settings").join("profile.json"),
                format!(
                    "{{\"root_ref\":\"{}\",\"channel\":\"{:?}\",\"role\":\"{:?}\"}}\n",
                    root.root_ref, root.channel_class, root.role
                )
                .as_bytes(),
            )?;
            write_bytes(
                &path.join("build").join("current.txt"),
                format!("previous-build:{}\n", root.topology_row_id).as_bytes(),
            )?;
            write_bytes(
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
                detail: err.to_string(),
            }
        })?;
        write_bytes(&path, &json)?;
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
        let snapshot = self.load_pre_state_snapshot(plan)?;
        self.apply_synthetic_update(plan)?;
        let mutated_snapshot = self.capture_snapshot(plan)?;
        if !target_changed(&snapshot, &mutated_snapshot, plan) {
            return Err(RollbackDrillError::SyntheticUpdateDidNotTouchTarget {
                drill_id: plan.drill_id.clone(),
            });
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
        let mut entries = Vec::new();
        for root in &plan.roots {
            let root_path = self.state_root_path(&root.root_ref)?;
            if !root_path.exists() {
                return Err(RollbackDrillError::MissingStateRoot {
                    root_ref: root.root_ref.clone(),
                    path: root_path,
                });
            }
            walk_root(&root.root_ref, &root_path, &root_path, &mut entries)?;
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
        let path = self.pre_state_snapshot_path(&plan.drill_id);
        let bytes = fs::read(&path).map_err(|err| io_error(&path, err))?;
        let snapshot: RollbackDrillPreStateSnapshot =
            serde_json::from_slice(&bytes).map_err(|err| {
                RollbackDrillError::CorruptedPreStateSnapshot {
                    path: path.clone(),
                    detail: err.to_string(),
                }
            })?;
        validate_snapshot(&path, plan, &snapshot)?;
        Ok(snapshot)
    }

    fn apply_synthetic_update(&self, plan: &RollbackDrillPlan) -> Result<(), RollbackDrillError> {
        for root in plan
            .roots
            .iter()
            .filter(|root| root.role == RollbackDrillRootRole::TargetRollback)
        {
            let root_path = self.state_root_path(&root.root_ref)?;
            write_bytes(
                &root_path.join("build").join("current.txt"),
                format!("candidate-build:{}\n", root.topology_row_id).as_bytes(),
            )?;
            write_bytes(
                &root_path
                    .join("update-staging")
                    .join("candidate-marker.json"),
                format!(
                    "{{\"target\":\"{}\",\"root_ref\":\"{}\",\"synthetic\":true}}\n",
                    root.topology_row_id, root.root_ref
                )
                .as_bytes(),
            )?;
        }
        Ok(())
    }

    fn restore_target_roots(
        &self,
        plan: &RollbackDrillPlan,
        snapshot: &RollbackDrillPreStateSnapshot,
    ) -> Result<(), RollbackDrillError> {
        let target_roots: BTreeSet<&str> = plan.target_root_refs().into_iter().collect();
        for root_ref in &target_roots {
            let root_path = self.state_root_path(root_ref)?;
            clear_directory_contents(&root_path)?;
        }

        for entry in snapshot
            .entries
            .iter()
            .filter(|entry| target_roots.contains(entry.root_ref.as_str()))
        {
            let root_path = self.state_root_path(&entry.root_ref)?;
            let path = join_relative(&root_path, &entry.relative_path)?;
            match entry.entry_kind {
                RollbackDrillEntryKind::Directory => {
                    create_dir_all(&path).map_err(|err| io_error(&path, err))?;
                }
                RollbackDrillEntryKind::File => {
                    write_bytes(&path, &entry.contents)?;
                }
            }
        }
        Ok(())
    }

    fn write_expected_delta_evidence(
        &self,
        plan: &RollbackDrillPlan,
    ) -> Result<(), RollbackDrillError> {
        for delta in &plan.expected_deltas {
            if delta.delta_class != RollbackDrillDeltaClass::PostRollbackEvidence {
                continue;
            }
            let root_path = self.state_root_path(&delta.root_ref)?;
            let path = join_relative(&root_path, &delta.relative_path)?;
            write_bytes(
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

fn walk_root(
    root_ref: &str,
    root_path: &Path,
    current_path: &Path,
    entries: &mut Vec<RollbackDrillEntry>,
) -> Result<(), RollbackDrillError> {
    let metadata = fs::symlink_metadata(current_path).map_err(|err| io_error(current_path, err))?;
    if metadata.file_type().is_symlink() {
        return Err(RollbackDrillError::UnsafeStateRoot {
            root_ref: root_ref.to_string(),
            detail: format!("symlink entry is not followed: {}", current_path.display()),
        });
    }

    if current_path != root_path {
        let relative_path = relative_path(root_path, current_path)?;
        if metadata.is_dir() {
            entries.push(RollbackDrillEntry {
                root_ref: root_ref.to_string(),
                relative_path,
                entry_kind: RollbackDrillEntryKind::Directory,
                contents: Vec::new(),
            });
        } else if metadata.is_file() {
            let contents = fs::read(current_path).map_err(|err| io_error(current_path, err))?;
            entries.push(RollbackDrillEntry {
                root_ref: root_ref.to_string(),
                relative_path,
                entry_kind: RollbackDrillEntryKind::File,
                contents,
            });
        }
    }

    if metadata.is_dir() {
        let mut children = fs::read_dir(current_path)
            .map_err(|err| io_error(current_path, err))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| io_error(current_path, err))?;
        children.sort_by_key(|entry| entry.path());
        for child in children {
            walk_root(root_ref, root_path, &child.path(), entries)?;
        }
    }
    Ok(())
}

fn clear_directory_contents(path: &Path) -> Result<(), RollbackDrillError> {
    if !path.exists() {
        create_dir_all(path).map_err(|err| io_error(path, err))?;
        return Ok(());
    }
    let mut children = fs::read_dir(path)
        .map_err(|err| io_error(path, err))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| io_error(path, err))?;
    children.sort_by_key(|entry| entry.path());
    children.reverse();
    for child in children {
        let child_path = child.path();
        let metadata =
            fs::symlink_metadata(&child_path).map_err(|err| io_error(&child_path, err))?;
        if metadata.file_type().is_symlink() || metadata.is_file() {
            fs::remove_file(&child_path).map_err(|err| io_error(&child_path, err))?;
        } else if metadata.is_dir() {
            clear_directory_contents(&child_path)?;
            fs::remove_dir(&child_path).map_err(|err| io_error(&child_path, err))?;
        }
    }
    Ok(())
}

fn write_bytes(path: &Path, contents: &[u8]) -> Result<(), RollbackDrillError> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent).map_err(|err| io_error(parent, err))?;
    }
    fs::write(path, contents).map_err(|err| io_error(path, err))
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
                || relative_path
                    .strip_prefix(delta.relative_path.as_str())
                    .is_some_and(|suffix| suffix.starts_with('/'))
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
    Err(RollbackDrillError::CorruptedPreStateSnapshot {
        path: path.to_path_buf(),
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

fn safe_root_segment(root_ref: &str) -> Result<String, RollbackDrillError> {
    if root_ref.is_empty() {
        return Err(RollbackDrillError::UnsafeStateRoot {
            root_ref: root_ref.to_string(),
            detail: "root ref must not be empty".to_string(),
        });
    }
    if root_ref == "." || root_ref == ".." || root_ref.contains('/') || root_ref.contains('\\') {
        return Err(RollbackDrillError::UnsafeStateRoot {
            root_ref: root_ref.to_string(),
            detail: "root ref must be a single path segment".to_string(),
        });
    }
    if !root_ref
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '.' || ch == '_' || ch == '-')
    {
        return Err(RollbackDrillError::UnsafeStateRoot {
            root_ref: root_ref.to_string(),
            detail: "root ref contains unsupported characters".to_string(),
        });
    }
    Ok(root_ref.to_string())
}

fn join_relative(root_path: &Path, relative_path: &str) -> Result<PathBuf, RollbackDrillError> {
    if relative_path.is_empty()
        || relative_path.starts_with('/')
        || relative_path.contains('\\')
        || relative_path
            .split('/')
            .any(|component| component.is_empty() || component == "." || component == "..")
    {
        return Err(RollbackDrillError::UnsafeStateRoot {
            root_ref: root_path.display().to_string(),
            detail: format!("relative path is unsafe: {relative_path}"),
        });
    }
    Ok(root_path.join(relative_path))
}

fn relative_path(root_path: &Path, current_path: &Path) -> Result<String, RollbackDrillError> {
    let relative = current_path.strip_prefix(root_path).map_err(|err| {
        RollbackDrillError::UnsafeStateRoot {
            root_ref: root_path.display().to_string(),
            detail: err.to_string(),
        }
    })?;
    Ok(relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/"))
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
    RollbackDrillError::Io {
        path: path.to_path_buf(),
        detail: err.to_string(),
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
