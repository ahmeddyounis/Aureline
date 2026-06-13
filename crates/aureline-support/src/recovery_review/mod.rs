//! Recovery-review packet joining crash-loop, scoped-reset, and quarantine truth.
//!
//! This module composes the existing M5 fault-domain, crash-store, and
//! crash-loop records into one metadata-only recovery-review packet. The packet
//! keeps crash-loop recovery choices, scoped reset / reattach comparisons,
//! quarantine and rollback review details, and bounded-blast-radius continuity
//! rows on one export-safe surface so support, docs, and shell recovery flows
//! reuse the same vocabulary.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::crash_loop_center::{
    CrashLoopEvidenceRef, CrashLoopRecoveryCenter, CrashLoopRecoveryCenterBeta,
    CrashLoopRecoverySupportPacket, CrashLoopSignal, CrashLoopTriggerClass, EvidenceDataClass,
    EvidenceEntryClass, FaultDomainClass as CrashLoopFaultDomainClass, RecentChange,
    RecentChangeClass, RecoveredArtifact, RecoveryChoiceClass, RecoveryStateClass, RedactionClass,
    RestoreClass as CrashLoopRestoreClass, SessionSensitivityClass,
    CRASH_LOOP_RECOVERY_SCHEMA_VERSION, CRASH_LOOP_SIGNAL_RECORD_KIND,
};
use crate::crash_store::{
    seeded_crash_store_viewer_packet, CrashStoreViewerPacket, CrashStoreViewerRow,
};
use crate::fault_domain_views::FaultDomainViewPacket;
use aureline_runtime::{
    seeded_host_topology_inspector, seeded_lane_filtered_event_viewer,
    seeded_reattach_review_sheet, HostLaneRecord, ReattachReviewSheet, TopologyInspectorRecord,
};

/// Stable record-kind tag carried by the recovery-review packet.
pub const RECOVERY_REVIEW_PACKET_RECORD_KIND: &str = "recovery_review_packet";

/// Stable record-kind tag for one bounded-continuity row.
pub const RECOVERY_CONTINUITY_ROW_RECORD_KIND: &str = "recovery_continuity_row";

/// Stable record-kind tag for one crash-loop review row.
pub const CRASH_LOOP_REVIEW_ROW_RECORD_KIND: &str = "crash_loop_review_row";

/// Stable record-kind tag for one scoped reset / reattach review row.
pub const SCOPED_RESET_REVIEW_ROW_RECORD_KIND: &str = "scoped_reset_review_row";

/// Stable record-kind tag for one quarantine / rollback review row.
pub const QUARANTINE_REVIEW_ROW_RECORD_KIND: &str = "quarantine_review_row";

/// Frozen schema version shared by recovery-review records.
pub const RECOVERY_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path of the recovery-review schema.
pub const RECOVERY_REVIEW_SCHEMA_REF: &str = "schemas/support/recovery_review.schema.json";

/// Repository-relative path of the recovery-review help document.
pub const RECOVERY_REVIEW_DOC_REF: &str = "docs/support/recovery-review.md";

/// Repository-relative path of the checked recovery-review artifact.
pub const RECOVERY_REVIEW_ARTIFACT_REF: &str = "artifacts/support/m5/recovery-review.md";

/// Repository-relative path of the protected recovery-review fixture directory.
pub const RECOVERY_REVIEW_FIXTURE_DIR: &str = "fixtures/support/m5/recovery_review";

/// One continuity row proving that recovery stayed inside a bounded fault domain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryContinuityRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Host lane that stayed isolated.
    pub host_lane_ref: String,
    /// Plain-language host family label.
    pub host_family_label: String,
    /// Fault-domain id governing the recovery.
    pub fault_domain_id: String,
    /// Surface tokens directly affected by the recovery.
    pub affected_surface_tokens: Vec<String>,
    /// Other surface tokens that remained available.
    pub unaffected_surface_tokens: Vec<String>,
    /// Checkpoints kept intact during bounded recovery.
    pub preserved_checkpoint_refs: Vec<String>,
    /// Result refs that remained visible as stale, partial, or rebuilding.
    pub preserved_result_refs: Vec<String>,
    /// Next safe actions exposed on the bounded surface.
    pub next_safe_action_tokens: Vec<String>,
    /// Export-safe continuity summary.
    pub continuity_summary: String,
}

/// One crash-loop review row joining fault-domain and crash-store truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashLoopReviewRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable review id.
    pub review_id: String,
    /// Source crash-loop center id.
    pub center_ref: String,
    /// Host lane that failed.
    pub host_lane_ref: String,
    /// Plain-language host family label.
    pub host_family_label: String,
    /// Stable fault-domain id shown to users and support.
    pub fault_domain_id: String,
    /// Stable fault-domain token shown to users and support.
    pub fault_domain_token: String,
    /// Surface tokens directly affected by the crash loop.
    pub failing_surface_tokens: Vec<String>,
    /// Exact build identity ref carried into crash review.
    pub exact_build_identity_ref: String,
    /// Visible build id.
    pub build_id: String,
    /// Visible crash id.
    pub crash_id: String,
    /// Session or target ref carried into the review.
    pub session_ref: String,
    /// Last attempted reopen mode.
    pub last_reopen_mode_token: String,
    /// Checkpoints preserved across the failure.
    pub preserved_checkpoint_refs: Vec<String>,
    /// Support-safe evidence refs preserved by the review.
    pub evidence_refs: Vec<String>,
    /// Safe-mode command path.
    pub safe_mode_command_id: String,
    /// Open-without-restore command path.
    pub open_without_restore_command_id: String,
    /// Targeted disable command paths.
    pub disable_recent_change_command_ids: Vec<String>,
    /// Recent change refs targeted by disable actions.
    pub disable_recent_change_refs: Vec<String>,
    /// Open-logs command path.
    pub open_logs_command_id: String,
    /// Export path used by the crash review.
    pub export_command_id: String,
    /// Whether the review blocks hidden rerun.
    pub no_hidden_rerun: bool,
}

/// One scoped reset / supervised reattach review row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopedResetReviewRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable review id.
    pub review_id: String,
    /// Previous host lane ref.
    pub previous_host_lane_ref: String,
    /// Current host lane ref.
    pub current_host_lane_ref: String,
    /// Previous host family label.
    pub previous_host_family_label: String,
    /// Current host family label.
    pub current_host_family_label: String,
    /// Previous host fingerprint token.
    pub previous_host_fingerprint_token: String,
    /// Current host fingerprint token.
    pub current_host_fingerprint_token: String,
    /// Session or target ref carried into the review.
    pub session_ref: String,
    /// Surface tokens still anchored around the failed lane.
    pub surrounding_surface_tokens: Vec<String>,
    /// Checkpoints preserved across the reset.
    pub preserved_checkpoint_refs: Vec<String>,
    /// Preserved state refs carried into the review.
    pub preserved_state_refs: Vec<String>,
    /// Lost state refs called out before continuing.
    pub lost_state_refs: Vec<String>,
    /// Replay-risk token.
    pub replay_risk_token: String,
    /// Rerun-requirement token.
    pub rerun_requirement_token: String,
    /// Review-decision token.
    pub decision_token: String,
    /// Whether approval or policy drift is present.
    pub approval_or_policy_drift_present: bool,
    /// Whether auth drift is present.
    pub auth_drift_present: bool,
    /// Safe actions exposed on the scoped reset surface.
    pub scoped_reset_action_tokens: Vec<String>,
    /// Whether the review blocks hidden rerun.
    pub no_hidden_rerun: bool,
}

/// One quarantine review row covering repeated failure or restart-budget abuse.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuarantineReviewRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable review id.
    pub review_id: String,
    /// Host lane under review.
    pub host_lane_ref: String,
    /// Plain-language host family label.
    pub host_family_label: String,
    /// Fault-domain id shown to users and support.
    pub fault_domain_id: String,
    /// Current restart-budget or lifecycle state token.
    pub current_state_token: String,
    /// Trigger that caused the review.
    pub trigger_summary: String,
    /// Scope summary naming what is narrowed and what stayed preserved.
    pub scope_summary: String,
    /// Evidence refs that justify the review.
    pub evidence_refs: Vec<String>,
    /// Action tokens exposed for controlled recovery.
    pub recovery_action_tokens: Vec<String>,
    /// Rollback candidate or last-known-good ref.
    pub rollback_candidate_ref: String,
    /// Support/export path preserved for follow-up.
    pub support_export_ref: String,
    /// Candidate action shown in the review.
    pub candidate_action_label: String,
    /// Risk note shown before confirmation.
    pub risk_note: String,
    /// Confirmation action ref.
    pub confirm_action_ref: String,
    /// Preserved checkpoints that keep recovery local.
    pub preserved_checkpoint_refs: Vec<String>,
    /// Surface tokens that remain visible while the lane is narrowed.
    pub surrounding_surface_tokens: Vec<String>,
}

/// Recovery-review packet joining crash-loop, reattach, quarantine, and continuity truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryReviewPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// RFC 3339 UTC generation time.
    pub generated_at: String,
    /// Help document consumed by support and QA reviewers.
    pub doc_ref: String,
    /// Boundary schema consumed by support/export review.
    pub schema_ref: String,
    /// Checked artifact ref.
    pub artifact_ref: String,
    /// Source topology packet ref.
    pub topology_packet_ref: String,
    /// Source crash-store packet ref.
    pub crash_store_packet_ref: String,
    /// Canonical fault-domain tokens reused across surfaces.
    pub protected_fault_domain_tokens: Vec<String>,
    /// Bounded continuity rows.
    pub continuity_rows: Vec<RecoveryContinuityRow>,
    /// Crash-loop review rows.
    pub crash_loop_reviews: Vec<CrashLoopReviewRow>,
    /// Scoped reset / reattach review rows.
    pub scoped_reset_reviews: Vec<ScopedResetReviewRow>,
    /// Quarantine / rollback review rows.
    pub quarantine_reviews: Vec<QuarantineReviewRow>,
    /// Metadata-safe export summary.
    pub export_safe_summary: String,
}

impl RecoveryReviewPacket {
    /// Builds a recovery-review packet from existing runtime and support records.
    pub fn from_components(
        packet_id: impl Into<String>,
        generated_at: impl Into<String>,
        inspector: &TopologyInspectorRecord,
        topology_packet: &FaultDomainViewPacket,
        crash_store_packet: &CrashStoreViewerPacket,
        crash_center: &CrashLoopRecoveryCenter,
        crash_support_packet: &CrashLoopRecoverySupportPacket,
    ) -> Self {
        let continuity_rows = build_continuity_rows(inspector, topology_packet);
        let crash_loop_reviews = vec![build_crash_loop_review_row(
            inspector,
            topology_packet,
            crash_store_packet,
            crash_center,
            crash_support_packet,
        )];
        let scoped_reset_reviews = topology_packet
            .reattach_reviews
            .iter()
            .map(|review| build_scoped_reset_review_row(inspector, topology_packet, review))
            .collect::<Vec<_>>();
        let quarantine_reviews =
            build_quarantine_review_rows(inspector, topology_packet, crash_store_packet);
        let protected_fault_domain_tokens = inspector
            .lanes
            .iter()
            .map(|lane| lane.fault_domain_token.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        Self {
            record_kind: RECOVERY_REVIEW_PACKET_RECORD_KIND.to_owned(),
            schema_version: RECOVERY_REVIEW_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            generated_at: generated_at.into(),
            doc_ref: RECOVERY_REVIEW_DOC_REF.to_owned(),
            schema_ref: RECOVERY_REVIEW_SCHEMA_REF.to_owned(),
            artifact_ref: RECOVERY_REVIEW_ARTIFACT_REF.to_owned(),
            topology_packet_ref: topology_packet.packet_id.clone(),
            crash_store_packet_ref: crash_store_packet.packet_id.clone(),
            protected_fault_domain_tokens,
            continuity_rows,
            crash_loop_reviews,
            scoped_reset_reviews,
            quarantine_reviews,
            export_safe_summary:
                "Recovery review stays metadata-only while preserving crash-loop, scoped reset, quarantine, and bounded continuity truth."
                    .to_owned(),
        }
    }

    /// Validates the packet against the bounded recovery contract.
    pub fn validate(&self) -> Vec<RecoveryReviewViolation> {
        let mut violations = Vec::new();
        if self.record_kind != RECOVERY_REVIEW_PACKET_RECORD_KIND {
            push_recovery_review_violation(
                &mut violations,
                "record_kind",
                &self.packet_id,
                "record_kind must be recovery_review_packet",
            );
        }
        if self.schema_version != RECOVERY_REVIEW_SCHEMA_VERSION {
            push_recovery_review_violation(
                &mut violations,
                "schema_version",
                &self.packet_id,
                "schema_version must match the recovery-review contract",
            );
        }
        if self.doc_ref != RECOVERY_REVIEW_DOC_REF {
            push_recovery_review_violation(
                &mut violations,
                "doc_ref",
                &self.packet_id,
                "packet must quote the canonical recovery-review doc",
            );
        }
        if self.schema_ref != RECOVERY_REVIEW_SCHEMA_REF {
            push_recovery_review_violation(
                &mut violations,
                "schema_ref",
                &self.packet_id,
                "packet must quote the canonical recovery-review schema",
            );
        }
        if self.artifact_ref != RECOVERY_REVIEW_ARTIFACT_REF {
            push_recovery_review_violation(
                &mut violations,
                "artifact_ref",
                &self.packet_id,
                "packet must quote the checked recovery-review artifact",
            );
        }
        if self.continuity_rows.is_empty() {
            push_recovery_review_violation(
                &mut violations,
                "continuity_rows",
                &self.packet_id,
                "bounded continuity rows are required",
            );
        }
        if self.crash_loop_reviews.is_empty() {
            push_recovery_review_violation(
                &mut violations,
                "crash_loop_reviews",
                &self.packet_id,
                "at least one crash-loop review is required",
            );
        }
        if self.scoped_reset_reviews.is_empty() {
            push_recovery_review_violation(
                &mut violations,
                "scoped_reset_reviews",
                &self.packet_id,
                "at least one scoped reset review is required",
            );
        }
        if self.quarantine_reviews.is_empty() {
            push_recovery_review_violation(
                &mut violations,
                "quarantine_reviews",
                &self.packet_id,
                "at least one quarantine review is required",
            );
        }

        for row in &self.continuity_rows {
            if row.preserved_checkpoint_refs.is_empty() {
                push_recovery_review_violation(
                    &mut violations,
                    "continuity_rows.preserved_checkpoint_refs",
                    &row.row_id,
                    "continuity rows must preserve checkpoints",
                );
            }
            if row.affected_surface_tokens.is_empty() || row.unaffected_surface_tokens.is_empty() {
                push_recovery_review_violation(
                    &mut violations,
                    "continuity_rows.surface_tokens",
                    &row.row_id,
                    "continuity rows must name both affected and unaffected surfaces",
                );
            }
            if row.next_safe_action_tokens.is_empty() {
                push_recovery_review_violation(
                    &mut violations,
                    "continuity_rows.next_safe_action_tokens",
                    &row.row_id,
                    "continuity rows must preserve bounded next-safe actions",
                );
            }
        }

        for row in &self.crash_loop_reviews {
            if row.build_id.trim().is_empty()
                || row.crash_id.trim().is_empty()
                || row.session_ref.trim().is_empty()
            {
                push_recovery_review_violation(
                    &mut violations,
                    "crash_loop_reviews.identity",
                    &row.review_id,
                    "crash-loop reviews must keep exact build, crash, and session identity visible",
                );
            }
            if row.safe_mode_command_id.trim().is_empty()
                || row.open_without_restore_command_id.trim().is_empty()
                || row.open_logs_command_id.trim().is_empty()
                || row.export_command_id.trim().is_empty()
            {
                push_recovery_review_violation(
                    &mut violations,
                    "crash_loop_reviews.command_ids",
                    &row.review_id,
                    "crash-loop reviews must preserve bounded recovery commands",
                );
            }
            if !row.no_hidden_rerun {
                push_recovery_review_violation(
                    &mut violations,
                    "crash_loop_reviews.no_hidden_rerun",
                    &row.review_id,
                    "crash-loop reviews must block hidden rerun",
                );
            }
        }

        for row in &self.scoped_reset_reviews {
            if row.preserved_state_refs.is_empty() || row.lost_state_refs.is_empty() {
                push_recovery_review_violation(
                    &mut violations,
                    "scoped_reset_reviews.state_refs",
                    &row.review_id,
                    "scoped reset reviews must compare preserved and lost state",
                );
            }
            if row.replay_risk_token.trim().is_empty()
                || row.rerun_requirement_token.trim().is_empty()
                || row.decision_token.trim().is_empty()
            {
                push_recovery_review_violation(
                    &mut violations,
                    "scoped_reset_reviews.review_tokens",
                    &row.review_id,
                    "scoped reset reviews must preserve replay, rerun, and decision tokens",
                );
            }
            if !row.no_hidden_rerun {
                push_recovery_review_violation(
                    &mut violations,
                    "scoped_reset_reviews.no_hidden_rerun",
                    &row.review_id,
                    "scoped reset reviews must block hidden rerun",
                );
            }
        }

        for row in &self.quarantine_reviews {
            if row.evidence_refs.is_empty() {
                push_recovery_review_violation(
                    &mut violations,
                    "quarantine_reviews.evidence_refs",
                    &row.review_id,
                    "quarantine reviews must cite evidence",
                );
            }
            if row.rollback_candidate_ref.trim().is_empty()
                || row.support_export_ref.trim().is_empty()
                || row.confirm_action_ref.trim().is_empty()
            {
                push_recovery_review_violation(
                    &mut violations,
                    "quarantine_reviews.recovery_paths",
                    &row.review_id,
                    "quarantine reviews must preserve rollback, support/export, and confirm paths",
                );
            }
        }

        violations
    }

    /// Renders a concise support-safe plaintext summary.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Recovery review packet\n");
        out.push_str(&format!("Packet: {}\n", self.packet_id));
        out.push_str(&format!("Generated: {}\n", self.generated_at));
        out.push_str(&format!(
            "Continuity rows: {}\n",
            self.continuity_rows.len()
        ));
        out.push_str(&format!(
            "Crash-loop reviews: {}\n",
            self.crash_loop_reviews.len()
        ));
        out.push_str(&format!(
            "Scoped reset reviews: {}\n",
            self.scoped_reset_reviews.len()
        ));
        out.push_str(&format!(
            "Quarantine reviews: {}\n",
            self.quarantine_reviews.len()
        ));
        for row in &self.crash_loop_reviews {
            out.push_str(&format!(
                "\nCrash loop: {} build={} session={}\n",
                row.host_lane_ref, row.build_id, row.session_ref
            ));
            out.push_str(&format!(
                "  fault domain: {} ({})\n",
                row.fault_domain_id, row.fault_domain_token
            ));
            out.push_str(&format!("  reopen mode: {}\n", row.last_reopen_mode_token));
            out.push_str(&format!(
                "  commands: {}, {}, {}, {}\n",
                row.safe_mode_command_id,
                row.open_without_restore_command_id,
                row.open_logs_command_id,
                row.export_command_id
            ));
        }
        for row in &self.scoped_reset_reviews {
            out.push_str(&format!(
                "\nScoped reset: {} -> {}\n",
                row.previous_host_lane_ref, row.current_host_lane_ref
            ));
            out.push_str(&format!(
                "  replay={} rerun={} decision={}\n",
                row.replay_risk_token, row.rerun_requirement_token, row.decision_token
            ));
        }
        for row in &self.quarantine_reviews {
            out.push_str(&format!(
                "\nQuarantine review: {} state={}\n",
                row.host_lane_ref, row.current_state_token
            ));
            out.push_str(&format!("  trigger: {}\n", row.trigger_summary));
            out.push_str(&format!(
                "  rollback: {} support={}\n",
                row.rollback_candidate_ref, row.support_export_ref
            ));
        }
        out
    }
}

/// One validation issue emitted by [`RecoveryReviewPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryReviewViolation {
    /// Field or path that failed validation.
    pub path: String,
    /// Subject record reference.
    pub subject_ref: String,
    /// Export-safe validation summary.
    pub summary: String,
}

fn push_recovery_review_violation(
    violations: &mut Vec<RecoveryReviewViolation>,
    path: impl Into<String>,
    subject_ref: impl Into<String>,
    summary: impl Into<String>,
) {
    violations.push(RecoveryReviewViolation {
        path: path.into(),
        subject_ref: subject_ref.into(),
        summary: summary.into(),
    });
}

fn build_continuity_rows(
    inspector: &TopologyInspectorRecord,
    topology_packet: &FaultDomainViewPacket,
) -> Vec<RecoveryContinuityRow> {
    let all_surfaces = topology_packet
        .topology_results
        .iter()
        .map(|result| result.surface_token.clone())
        .collect::<BTreeSet<_>>();

    topology_packet
        .rows
        .iter()
        .filter(|row| !row.partial_truth_result_refs.is_empty() || row.restart_strike_count > 0)
        .map(|row| {
            let restart_card = topology_packet
                .restart_cards
                .iter()
                .find(|card| card.host_lane_ref == row.host_lane_ref)
                .expect("restart card matches row");
            let unaffected_surface_tokens = all_surfaces
                .iter()
                .filter(|surface| !row.surface_tokens.contains(*surface))
                .cloned()
                .collect::<Vec<_>>();
            let continuity_summary = format!(
                "{} stayed local to {} while surrounding layout and checkpoints remained intact.",
                row.health_label, row.host_family_label
            );
            let lane = inspector
                .lane(&row.host_lane_ref)
                .expect("continuity row lane exists");
            RecoveryContinuityRow {
                record_kind: RECOVERY_CONTINUITY_ROW_RECORD_KIND.to_owned(),
                schema_version: RECOVERY_REVIEW_SCHEMA_VERSION,
                row_id: format!("recovery-continuity:{}", row.host_lane_ref),
                host_lane_ref: row.host_lane_ref.clone(),
                host_family_label: row.host_family_label.clone(),
                fault_domain_id: row.fault_domain_id.clone(),
                affected_surface_tokens: row.surface_tokens.clone(),
                unaffected_surface_tokens,
                preserved_checkpoint_refs: lane.preserved_checkpoint_refs.clone(),
                preserved_result_refs: row.partial_truth_result_refs.clone(),
                next_safe_action_tokens: restart_card.next_safe_action_tokens.clone(),
                continuity_summary,
            }
        })
        .collect()
}

fn build_crash_loop_review_row(
    inspector: &TopologyInspectorRecord,
    topology_packet: &FaultDomainViewPacket,
    crash_store_packet: &CrashStoreViewerPacket,
    crash_center: &CrashLoopRecoveryCenter,
    crash_support_packet: &CrashLoopRecoverySupportPacket,
) -> CrashLoopReviewRow {
    let lane = inspector
        .lane("lane:notebook-kernel")
        .expect("seeded notebook lane exists");
    let view_row = topology_packet
        .rows
        .iter()
        .find(|row| row.host_lane_ref == lane.lane_id)
        .expect("seeded notebook view row exists");
    let crash_row = crash_store_packet
        .rows
        .iter()
        .find(|row| row.host_family_id == "notebook_kernel_host")
        .expect("seeded notebook crash row exists");

    let disable_rows = crash_support_packet
        .choice_rows
        .iter()
        .filter(|row| {
            matches!(
                row.choice_class,
                RecoveryChoiceClass::DisableRecentlyChangedExtension
                    | RecoveryChoiceClass::DisableRecentlyChangedProfileOrLayout
            )
        })
        .collect::<Vec<_>>();

    CrashLoopReviewRow {
        record_kind: CRASH_LOOP_REVIEW_ROW_RECORD_KIND.to_owned(),
        schema_version: RECOVERY_REVIEW_SCHEMA_VERSION,
        review_id: format!("crash-loop-review:{}", lane.lane_id),
        center_ref: crash_center.center_id.clone(),
        host_lane_ref: lane.lane_id.clone(),
        host_family_label: lane.family_label.clone(),
        fault_domain_id: lane.fault_domain_id.clone(),
        fault_domain_token: lane.fault_domain_token.clone(),
        failing_surface_tokens: view_row.surface_tokens.clone(),
        exact_build_identity_ref: crash_row.primary_exact_build_identity_ref.clone(),
        build_id: crash_row.build_id.clone(),
        crash_id: crash_row.crash_id.clone(),
        session_ref: lane
            .target_ref
            .clone()
            .unwrap_or_else(|| crash_row.session_type_id.clone()),
        last_reopen_mode_token: crash_center.last_reopen_mode.as_str().to_owned(),
        preserved_checkpoint_refs: lane.preserved_checkpoint_refs.clone(),
        evidence_refs: crash_support_packet.evidence_refs.clone(),
        safe_mode_command_id: crash_center
            .choice(RecoveryChoiceClass::EnterSafeMode)
            .expect("safe mode choice exists")
            .command
            .command_id
            .clone(),
        open_without_restore_command_id: crash_center
            .choice(RecoveryChoiceClass::OpenWithoutRestore)
            .expect("open without restore choice exists")
            .command
            .command_id
            .clone(),
        disable_recent_change_command_ids: disable_rows
            .iter()
            .map(|row| row.command_id.clone())
            .collect(),
        disable_recent_change_refs: disable_rows
            .iter()
            .filter_map(|row| row.targets_recent_change_ref.clone())
            .collect(),
        open_logs_command_id: crash_center
            .choice(RecoveryChoiceClass::OpenLogs)
            .expect("open logs choice exists")
            .command
            .command_id
            .clone(),
        export_command_id: crash_center
            .choice(RecoveryChoiceClass::ExportCrashManifest)
            .expect("export choice exists")
            .command
            .command_id
            .clone(),
        no_hidden_rerun: true,
    }
}

fn build_scoped_reset_review_row(
    inspector: &TopologyInspectorRecord,
    topology_packet: &FaultDomainViewPacket,
    review: &ReattachReviewSheet,
) -> ScopedResetReviewRow {
    let current_lane = inspector
        .lane(&review.current_host_lane_ref)
        .expect("reattach current lane exists");
    let view_row = topology_packet
        .rows
        .iter()
        .find(|row| row.host_lane_ref == review.current_host_lane_ref)
        .expect("reattach current row exists");
    let restart_card = topology_packet
        .restart_cards
        .iter()
        .find(|card| card.host_lane_ref == review.current_host_lane_ref)
        .expect("reattach restart card exists");

    ScopedResetReviewRow {
        record_kind: SCOPED_RESET_REVIEW_ROW_RECORD_KIND.to_owned(),
        schema_version: RECOVERY_REVIEW_SCHEMA_VERSION,
        review_id: review.review_id.clone(),
        previous_host_lane_ref: review.previous_host_lane_ref.clone(),
        current_host_lane_ref: review.current_host_lane_ref.clone(),
        previous_host_family_label: review.previous_host_family_label.clone(),
        current_host_family_label: review.current_host_family_label.clone(),
        previous_host_fingerprint_token: review.previous_host_fingerprint_token.clone(),
        current_host_fingerprint_token: review.current_host_fingerprint_token.clone(),
        session_ref: current_lane
            .target_ref
            .clone()
            .unwrap_or_else(|| review.current_host_lane_ref.clone()),
        surrounding_surface_tokens: view_row.surface_tokens.clone(),
        preserved_checkpoint_refs: current_lane.preserved_checkpoint_refs.clone(),
        preserved_state_refs: review.preserved_state_refs.clone(),
        lost_state_refs: review.lost_state_refs.clone(),
        replay_risk_token: review.replay_risk_token.clone(),
        rerun_requirement_token: review.rerun_requirement_token.clone(),
        decision_token: review.decision_token.clone(),
        approval_or_policy_drift_present: review.policy_drift_present,
        auth_drift_present: review.auth_drift_present,
        scoped_reset_action_tokens: restart_card.next_safe_action_tokens.clone(),
        no_hidden_rerun: true,
    }
}

fn build_quarantine_review_rows(
    inspector: &TopologyInspectorRecord,
    topology_packet: &FaultDomainViewPacket,
    crash_store_packet: &CrashStoreViewerPacket,
) -> Vec<QuarantineReviewRow> {
    topology_packet
        .rows
        .iter()
        .filter(|row| {
            matches!(
                row.restart_budget_state_token.as_str(),
                "quarantined" | "budget_exhausted" | "budget_warning"
            )
        })
        .map(|row| {
            let lane = inspector
                .lane(&row.host_lane_ref)
                .expect("quarantine lane exists");
            let crash_row = matching_crash_store_row(crash_store_packet, lane);
            let evidence_refs = quarantine_evidence_refs(row, crash_row);
            let rollback_candidate_ref = crash_row
                .map(|entry| entry.primary_exact_build_identity_ref.clone())
                .unwrap_or_else(|| format!("rollback-candidate:{}", row.host_lane_ref));
            let support_export_ref = crash_row
                .map(|entry| entry.support_export_review_ref.clone())
                .unwrap_or_else(|| format!("support-export:{}", row.host_lane_ref));
            let candidate_action_label = quarantine_candidate_action_label(row.restart_budget_state_token.as_str()).to_owned();
            let risk_note = format!(
                "Affected capabilities stay narrowed to {:?}; surrounding layout and checkpoints remain preserved.",
                row.affected_capability_tokens
            );
            let scope_summary = format!(
                "{} narrows {} while keeping {:?} visible by checkpoint or partial-truth label.",
                row.restart_budget_state_label, row.host_family_label, row.surface_tokens
            );
            QuarantineReviewRow {
                record_kind: QUARANTINE_REVIEW_ROW_RECORD_KIND.to_owned(),
                schema_version: RECOVERY_REVIEW_SCHEMA_VERSION,
                review_id: format!("quarantine-review:{}", row.host_lane_ref),
                host_lane_ref: row.host_lane_ref.clone(),
                host_family_label: row.host_family_label.clone(),
                fault_domain_id: row.fault_domain_id.clone(),
                current_state_token: row.restart_budget_state_token.clone(),
                trigger_summary: row.quarantine_trigger_summary.clone(),
                scope_summary,
                evidence_refs,
                recovery_action_tokens: topology_packet
                    .restart_cards
                    .iter()
                    .find(|card| card.host_lane_ref == row.host_lane_ref)
                    .map(|card| card.next_safe_action_tokens.clone())
                    .unwrap_or_default(),
                rollback_candidate_ref,
                support_export_ref,
                candidate_action_label,
                risk_note,
                confirm_action_ref: format!("action:recovery-review:{}:confirm", row.host_lane_ref),
                preserved_checkpoint_refs: row.preserved_checkpoint_refs.clone(),
                surrounding_surface_tokens: row.surface_tokens.clone(),
            }
        })
        .collect()
}

fn quarantine_candidate_action_label(state_token: &str) -> &'static str {
    match state_token {
        "quarantined" => "Rollback or re-enable once after review",
        "budget_exhausted" => "Restart isolated or stay on local fallback",
        "budget_warning" => "Narrow now before the next strike forces quarantine",
        _ => "Review bounded recovery",
    }
}

fn quarantine_evidence_refs(
    row: &crate::fault_domain_views::FaultDomainViewRow,
    crash_row: Option<&CrashStoreViewerRow>,
) -> Vec<String> {
    let mut refs = row.partial_truth_result_refs.clone();
    refs.extend(row.lane_event_ids.clone());
    if let Some(banner) = &row.crash_banner_ref {
        refs.push(banner.clone());
    }
    if let Some(entry) = crash_row {
        refs.push(entry.crash_envelope_ref.clone());
        refs.push(entry.restart_lineage_ref.clone());
    }
    refs.sort();
    refs.dedup();
    refs
}

fn matching_crash_store_row<'a>(
    crash_store_packet: &'a CrashStoreViewerPacket,
    lane: &HostLaneRecord,
) -> Option<&'a CrashStoreViewerRow> {
    let host_family_id = match lane.lane_id.as_str() {
        "lane:notebook-kernel" => Some("notebook_kernel_host"),
        "lane:preview-dev-server" => Some("preview_dev_server_host"),
        "lane:provider-run" => Some("provider_run_session_host"),
        "lane:profiler-replay" => Some("profiler_replay_session_host"),
        "lane:pipeline-viewer" => Some("pipeline_viewer_host"),
        "lane:data-api-connector" => Some("query_runtime_host"),
        _ => None,
    };
    host_family_id.and_then(|family| {
        crash_store_packet
            .rows
            .iter()
            .find(|row| row.host_family_id == family)
    })
}

fn seeded_notebook_crash_signal(crash_row: &CrashStoreViewerRow) -> CrashLoopSignal {
    CrashLoopSignal {
        schema_version: CRASH_LOOP_RECOVERY_SCHEMA_VERSION,
        record_kind: CRASH_LOOP_SIGNAL_RECORD_KIND.to_owned(),
        signal_id: "crash_loop_center.notebook_kernel_repeated_failure".to_owned(),
        captured_at: "2026-05-18T12:06:00Z".to_owned(),
        trigger_class: CrashLoopTriggerClass::RuntimeHostRestartBudgetExceeded,
        strike_count: 2,
        strike_budget: 2,
        hidden_restart_attempts: 2,
        crash_id: crash_row.crash_id.clone(),
        build_id: crash_row.build_id.clone(),
        crash_envelope_ref: crash_row.crash_envelope_ref.clone(),
        crash_manifest_ref: format!("crash-manifest:{}", crash_row.crash_id),
        restore_class: CrashLoopRestoreClass::CompatibleRestore,
        suspected_fault_domain: CrashLoopFaultDomainClass::LanguageRuntimeHost,
        fault_domain_ref: crash_row.restart_lineage_ref.clone(),
        last_reopen_mode: crate::crash_loop_center::ReopenModeClass::FullRestore,
        doctor_finding_ref: "doctor.finding.runtime.notebook_kernel_crash_loop".to_owned(),
        session_sensitivity_class: SessionSensitivityClass::LocalMutating,
        recent_changes: vec![
            RecentChange {
                change_id: "change.extension.notebook-lsp.update".to_owned(),
                change_class: RecentChangeClass::ExtensionUpdated,
                subject_ref: "extension:notebook-lsp".to_owned(),
                display_label: "Notebook LSP extension".to_owned(),
                observed_at: "2026-05-18T11:58:00Z".to_owned(),
                reversible: true,
            },
            RecentChange {
                change_id: "change.layout.notebook-runtime-panel".to_owned(),
                change_class: RecentChangeClass::LayoutChanged,
                subject_ref: "layout:notebook-runtime-panel".to_owned(),
                display_label: "Notebook runtime panel layout".to_owned(),
                observed_at: "2026-05-18T11:59:00Z".to_owned(),
                reversible: true,
            },
        ],
        recovered_artifacts: vec![
            RecoveredArtifact {
                artifact_id: "recovered-draft:notebook-cell-8".to_owned(),
                entry_class: EvidenceEntryClass::RecoveredDraft,
                summary: "Recovered unsaved cell draft for notebook analysis.ipynb".to_owned(),
                preserves: vec![
                    RecoveryStateClass::UserAuthoredFiles,
                    RecoveryStateClass::RecoveredDrafts,
                ],
            },
            RecoveredArtifact {
                artifact_id: "checkpoint:notebook-output-snapshot".to_owned(),
                entry_class: EvidenceEntryClass::RollbackableState,
                summary: "Checkpointed notebook output snapshot remains available for comparison."
                    .to_owned(),
                preserves: vec![
                    RecoveryStateClass::CheckpointHistory,
                    RecoveryStateClass::SessionRestoreStore,
                ],
            },
        ],
        evidence: vec![
            CrashLoopEvidenceRef {
                evidence_ref: crash_row.crash_envelope_ref.clone(),
                evidence_kind: "crash_envelope".to_owned(),
                data_class: EvidenceDataClass::Metadata,
                redaction_class: RedactionClass::MetadataSafeDefault,
                summary: "Crash envelope kept local with exact crash and build identity."
                    .to_owned(),
            },
            CrashLoopEvidenceRef {
                evidence_ref: crash_row.restart_lineage_ref.clone(),
                evidence_kind: "restart_lineage".to_owned(),
                data_class: EvidenceDataClass::EnvironmentAdjacent,
                redaction_class: RedactionClass::MetadataSafeDefault,
                summary: "Restart lineage preserves the last good checkpoint and strike history."
                    .to_owned(),
            },
        ],
    }
}

/// Builds the canonical seeded recovery-review packet.
pub fn seeded_recovery_review_packet() -> RecoveryReviewPacket {
    let inspector = seeded_host_topology_inspector();
    let reattach_review = seeded_reattach_review_sheet();
    let event_viewer = seeded_lane_filtered_event_viewer();
    let topology_packet = crate::fault_domain_views::FaultDomainViewPacket::from_topology(
        "fault-domain-view:recovery-review-seed",
        "2026-05-18T12:05:00Z",
        &inspector,
        vec![reattach_review],
        event_viewer,
    );
    let crash_store_packet = seeded_crash_store_viewer_packet();
    let crash_row = crash_store_packet
        .rows
        .iter()
        .find(|row| row.host_family_id == "notebook_kernel_host")
        .expect("seeded notebook crash row exists");
    let signal = seeded_notebook_crash_signal(crash_row);
    let evaluator = CrashLoopRecoveryCenterBeta::new();
    let crash_center = evaluator
        .evaluate(&signal)
        .expect("seeded notebook signal must validate");
    let crash_support_packet =
        evaluator.support_packet("support:recovery-review:notebook-kernel", &crash_center);

    RecoveryReviewPacket::from_components(
        "recovery-review:seed",
        "2026-05-18T12:08:00Z",
        &inspector,
        &topology_packet,
        &crash_store_packet,
        &crash_center,
        &crash_support_packet,
    )
}
