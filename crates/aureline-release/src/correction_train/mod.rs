//! Typed correction-train, hotfix, and backport packet model.
//!
//! Correction trains, emergency hotfixes, and supported-line backports share
//! one packet shape so release, support, and docs can read a single record
//! without translation. Each correction row carries:
//!
//! - the affected build scope (release candidate, exact-build identities, and
//!   affected release lines),
//! - fix lineage (the triage lane, source train, and hotfix/correction packet
//!   refs),
//! - backport targets (an explicit decision for every affected supported line),
//! - rollback linkage (a named release candidate or line every shipping lane
//!   can return to), and
//! - evidence refs (reruns, protected-path proofs, and adjacent sweeps).
//!
//! The model is metadata-only: it binds opaque refs and never carries raw
//! artifacts, raw logs, or credential material. Rollback targets are recorded
//! as refs that resolve to the same last-known-good candidates and lines used
//! by the release-center rollback/revocation records, rather than re-modelling
//! those records here.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for correction-train packet records.
pub const CORRECTION_TRAIN_PACKET_RECORD_KIND: &str = "correction_train_packet_record";

/// Schema version for the correction-train packet model.
pub const CORRECTION_TRAIN_PACKET_SCHEMA_VERSION: u32 = 1;

/// Shared field families every correction, hotfix, and backport row must keep
/// in scope so release, support, and docs read one vocabulary.
pub const SHARED_PACKET_FORM_TERMS: &[&str] = &[
    "correction_scope",
    "correction_risk",
    "correction_evidence",
    "target_channels",
    "triage_lane",
    "backport_decision",
    "rollback_target",
    "known_issue_update",
];

/// Support-line classes whose affected rows always require an explicit
/// (non `not_applicable`) backport decision.
pub const SUPPORTED_LINE_CLASSES: &[&str] = &["stable", "lts"];

/// Issue classes that must take the emergency hotfix lane while a claimed
/// surface remains affected.
pub const SECURITY_OR_TRUST_ISSUE_CLASSES: &[&str] = &[
    "security_policy_escape",
    "trust_boundary_or_permission_failure",
];

/// Triage lane assigned to a correction row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriageLane {
    /// Emergency lane carrying the smallest viable change set.
    Hotfix,
    /// Supported-line lane that rides a normal train but still patches lines.
    Backport,
    /// Planned correction train without an emergency or supported-line action.
    CorrectionTrainOnly,
    /// Deferred to the next planned cycle; never an emergency release.
    NextCycle,
}

impl TriageLane {
    /// Whether the lane ships on a release lane that must name a rollback target.
    pub fn ships_on_release_lane(self) -> bool {
        matches!(
            self,
            TriageLane::Hotfix | TriageLane::Backport | TriageLane::CorrectionTrainOnly
        )
    }

    /// Whether the lane is one of the two emergency/supported-line lanes that
    /// require adjacent failure-domain sweep evidence before closure.
    pub fn requires_adjacent_sweep(self) -> bool {
        matches!(self, TriageLane::Hotfix | TriageLane::Backport)
    }
}

/// Per-line backport decision drawn from the controlled vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackportDecision {
    /// Backport to this supported line.
    Yes,
    /// Do not backport; the line is unaffected, unsupported, or outside claim.
    No,
    /// Defer with a due date, owner, and non-overstated support posture.
    Defer,
    /// The line is not affected or is not a supported line for the claim.
    NotApplicable,
}

/// Top-level correction-train / hotfix / backport packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorrectionTrainPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version for this packet shape.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Correction train ref this packet governs.
    pub train_ref: String,
    /// Release candidate ref the corrections target.
    pub release_candidate_ref: String,
    /// Exact-build identities in scope for the corrections.
    #[serde(default)]
    pub exact_build_identity_refs: Vec<String>,
    /// Refs for the correction-train, hotfix, and backport templates.
    pub packet_templates: PacketTemplates,
    /// Correction, hotfix, and backport rows sharing one form.
    #[serde(default)]
    pub correction_items: Vec<CorrectionItem>,
    /// Metadata-only support projection over the correction rows.
    pub support_projection: SupportProjection,
}

impl CorrectionTrainPacket {
    /// Validates packet-level and per-row invariants before a caller exposes
    /// the packet to release, support, or docs surfaces.
    pub fn validate(&self) -> Vec<CorrectionTrainViolation> {
        let mut violations = Vec::new();

        if self.record_kind != CORRECTION_TRAIN_PACKET_RECORD_KIND {
            push_violation(
                &mut violations,
                "packet.record_kind",
                &self.packet_id,
                "correction packet record_kind is not the correction-train packet kind",
            );
        }
        if self.schema_version != CORRECTION_TRAIN_PACKET_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "packet.schema_version",
                &self.packet_id,
                "correction packet schema_version must be 1",
            );
        }
        if self.exact_build_identity_refs.is_empty() {
            push_violation(
                &mut violations,
                "packet.exact_build_identity_refs_missing",
                &self.packet_id,
                "correction packet must name the affected exact-build identities",
            );
        }
        for build_ref in &self.exact_build_identity_refs {
            if !build_ref.starts_with("build-id:aureline:") {
                push_violation(
                    &mut violations,
                    "packet.exact_build_identity_ref_vocabulary",
                    build_ref,
                    "exact-build identity must use the Aureline build-id vocabulary",
                );
            }
        }

        self.packet_templates.validate(&mut violations);

        if self.correction_items.is_empty() {
            push_violation(
                &mut violations,
                "correction_items.empty",
                &self.packet_id,
                "correction packet must contain at least one correction row",
            );
        }

        let mut seen_ids = BTreeSet::new();
        for item in &self.correction_items {
            if !seen_ids.insert(item.item_id.as_str()) {
                push_violation(
                    &mut violations,
                    "correction_items.duplicate_item_id",
                    &item.item_id,
                    "correction item ids must be unique",
                );
            }
            item.validate(&mut violations);
        }

        self.support_projection
            .validate(&self.packet_id, &mut violations);

        violations
    }

    /// Set of triage lanes exercised by the correction rows.
    pub fn observed_lanes(&self) -> BTreeSet<TriageLane> {
        self.correction_items
            .iter()
            .map(|item| item.triage.lane_decision)
            .collect()
    }
}

/// Refs for the three packet templates this model formalizes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketTemplates {
    /// Correction-train template ref.
    pub correction_train_template_ref: String,
    /// Hotfix packet template ref.
    pub hotfix_packet_template_ref: String,
    /// Backport packet template ref.
    pub backport_packet_template_ref: String,
    /// Shared packet-form vocabulary the templates advertise.
    #[serde(default)]
    pub shared_packet_format_terms: Vec<String>,
}

impl PacketTemplates {
    fn validate(&self, violations: &mut Vec<CorrectionTrainViolation>) {
        let terms = self
            .shared_packet_format_terms
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for required in SHARED_PACKET_FORM_TERMS {
            if !terms.contains(required) {
                push_violation(
                    violations,
                    "packet_templates.required_terms_missing",
                    required,
                    "packet templates must advertise the shared correction form vocabulary",
                );
            }
        }
        for (field, value) in [
            (
                "correction_train_template_ref",
                &self.correction_train_template_ref,
            ),
            (
                "hotfix_packet_template_ref",
                &self.hotfix_packet_template_ref,
            ),
            (
                "backport_packet_template_ref",
                &self.backport_packet_template_ref,
            ),
        ] {
            if value.trim().is_empty() {
                push_violation(
                    violations,
                    "packet_templates.ref_empty",
                    field,
                    "every packet template ref must be a non-empty path",
                );
            }
        }
    }
}

/// One correction, hotfix, or backport row sharing the common packet form.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorrectionItem {
    /// Stable correction item id.
    pub item_id: String,
    /// Human-readable title.
    pub title: String,
    /// Issue class such as `security_policy_escape` or `non_protected_polish`.
    pub issue_class: String,
    /// Severity class such as `critical` or `low`.
    pub severity_class: String,
    /// Lifecycle state such as `triaged` or `deferred`.
    pub lifecycle_state: String,
    /// Affected claim, profile, artifact, line, channel, and rollback scope.
    pub scope: CorrectionScope,
    /// Risk posture for the correction.
    pub risk: CorrectionRisk,
    /// Evidence refs backing the correction.
    pub evidence: CorrectionEvidence,
    /// Triage lane and fix lineage refs.
    pub triage: CorrectionTriage,
    /// Per-line backport decisions.
    #[serde(default)]
    pub backport_matrix: Vec<BackportMatrixRow>,
    /// Target-channel dispositions.
    #[serde(default)]
    pub target_channel_updates: Vec<TargetChannelUpdate>,
    /// Release-truth refs updated in the same lane.
    pub release_notes: ReleaseNotesRefs,
}

impl CorrectionItem {
    fn validate(&self, violations: &mut Vec<CorrectionTrainViolation>) {
        let lane = self.triage.lane_decision;

        // Affected-build scope: every correction row must name the affected
        // release lines it travels through.
        if self.scope.affected_release_lines.is_empty() {
            push_violation(
                violations,
                "correction_scope.affected_build_scope_missing",
                &self.item_id,
                "correction rows must name the affected release lines",
            );
        }

        // Rollback linkage: any row that ships on a release lane must name a
        // rollback target users and support can return to.
        if lane.ships_on_release_lane()
            && self
                .scope
                .rollback_target_ref
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .is_none()
        {
            push_violation(
                violations,
                "correction_scope.rollback_target_missing",
                &self.item_id,
                "correction rows that ship on a release lane must name a rollback target",
            );
        }

        // Lane policy: security and trust-boundary defects on a claimed surface
        // take the emergency hotfix lane.
        if SECURITY_OR_TRUST_ISSUE_CLASSES.contains(&self.issue_class.as_str())
            && lane != TriageLane::Hotfix
        {
            push_violation(
                violations,
                "triage.security_or_trust_requires_hotfix",
                &self.item_id,
                "security and trust-boundary defects on claimed surfaces require the hotfix lane",
            );
        }
        if self.issue_class == "data_loss_or_migration_breakage"
            && !matches!(lane, TriageLane::Hotfix | TriageLane::Backport)
        {
            push_violation(
                violations,
                "triage.data_or_migration_requires_hotfix_or_backport",
                &self.item_id,
                "data-loss and migration defects must not be train-only or next-cycle while claimed",
            );
        }
        if self.issue_class == "protected_path_regression" && lane == TriageLane::NextCycle {
            push_violation(
                violations,
                "triage.protected_path_not_next_cycle",
                &self.item_id,
                "protected-path regressions must not be deferred as next-cycle work",
            );
        }
        if self.issue_class == "non_protected_polish"
            && matches!(lane, TriageLane::Hotfix | TriageLane::Backport)
        {
            push_violation(
                violations,
                "triage.polish_not_emergency",
                &self.item_id,
                "non-protected polish must not ride hotfix or backport lanes",
            );
        }

        // Hotfix rows must point at their hotfix packet ref.
        if lane == TriageLane::Hotfix
            && self
                .triage
                .hotfix_packet_ref
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .is_none()
        {
            push_violation(
                violations,
                "triage.hotfix_packet_ref_missing",
                &self.item_id,
                "hotfix rows must point at the hotfix packet ref",
            );
        }

        // Claim narrowing must name the claim update refs it relies on.
        if self.risk.claim_narrowing_required && self.release_notes.claim_update_refs.is_empty() {
            push_violation(
                violations,
                "triage.claim_narrowing_without_claim_update",
                &self.item_id,
                "claim-narrowing corrections must name claim update refs",
            );
        }

        self.validate_backport_matrix(violations);
        self.validate_evidence(lane, violations);
    }

    fn validate_backport_matrix(&self, violations: &mut Vec<CorrectionTrainViolation>) {
        let matrix_lines = self
            .backport_matrix
            .iter()
            .map(|row| row.release_line_ref.as_str())
            .collect::<BTreeSet<_>>();
        for line in &self.scope.affected_release_lines {
            if !matrix_lines.contains(line.as_str()) {
                push_violation(
                    violations,
                    "backport_matrix.affected_line_missing",
                    &format!("{}:{}", self.item_id, line),
                    "every affected release line must appear in the backport matrix",
                );
            }
        }

        for row in &self.backport_matrix {
            let supported = SUPPORTED_LINE_CLASSES.contains(&row.support_line_class.as_str());
            let line_ref = format!("{}:{}", self.item_id, row.release_line_ref);

            if row.affected && supported && row.decision == BackportDecision::NotApplicable {
                push_violation(
                    violations,
                    "backport_matrix.affected_line_no_decision",
                    &line_ref,
                    "affected supported lines must record yes, no, or defer",
                );
            }
            if row.affected
                && supported
                && SECURITY_OR_TRUST_ISSUE_CLASSES.contains(&self.issue_class.as_str())
                && row.decision != BackportDecision::Yes
            {
                push_violation(
                    violations,
                    "backport_matrix.security_supported_line_not_yes",
                    &line_ref,
                    "security and trust hotfixes must backport to affected supported lines",
                );
            }
            if row.decision == BackportDecision::Yes {
                if row
                    .target_release_ref
                    .as_deref()
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .is_none()
                {
                    push_violation(
                        violations,
                        "backport_matrix.yes_missing_target_release",
                        &line_ref,
                        "yes backport decisions must name a target release",
                    );
                }
                if row
                    .rollback_target_ref
                    .as_deref()
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .is_none()
                {
                    push_violation(
                        violations,
                        "backport_matrix.yes_missing_rollback_target",
                        &line_ref,
                        "yes backport decisions must name a rollback target",
                    );
                }
            }
        }
    }

    fn validate_evidence(&self, lane: TriageLane, violations: &mut Vec<CorrectionTrainViolation>) {
        if lane.ships_on_release_lane()
            && !matches!(
                self.evidence.freshness_state.as_str(),
                "current" | "waived_current"
            )
        {
            push_violation(
                violations,
                "evidence.not_current_for_claimed_correction",
                &self.item_id,
                "release-lane corrections require current or explicitly waived evidence",
            );
        }
        if lane.requires_adjacent_sweep() && self.evidence.adjacent_sweep_refs.is_empty() {
            push_violation(
                violations,
                "evidence.adjacent_sweep_missing",
                &self.item_id,
                "hotfix and backport rows must include adjacent failure-domain sweep refs",
            );
        }
    }
}

/// Affected claim, profile, artifact, line, channel, and rollback scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorrectionScope {
    /// Affected claim refs.
    #[serde(default)]
    pub affected_claim_refs: Vec<String>,
    /// Affected profile refs.
    #[serde(default)]
    pub affected_profile_refs: Vec<String>,
    /// Affected artifact refs.
    #[serde(default)]
    pub affected_artifact_refs: Vec<String>,
    /// Affected release lines.
    #[serde(default)]
    pub affected_release_lines: Vec<String>,
    /// Target channel refs.
    #[serde(default)]
    pub target_channel_refs: Vec<String>,
    /// Named rollback target users and support can return to.
    #[serde(default)]
    pub rollback_target_ref: Option<String>,
    /// Compatibility or interface-diff refs.
    #[serde(default)]
    pub compatibility_refs: Vec<String>,
}

/// Risk posture for a correction row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorrectionRisk {
    /// Risk level such as `critical` or `low`.
    pub risk_level: String,
    /// User-data risk summary.
    pub user_data_risk: String,
    /// Security or trust risk summary.
    pub security_or_trust_risk: String,
    /// Migration or schema risk summary.
    pub migration_or_schema_risk: String,
    /// Blast-radius summary.
    pub blast_radius: String,
    /// Workaround state such as `safe_workaround_available`.
    pub workaround_state: String,
    /// Whether a public claim must narrow for this correction.
    #[serde(default)]
    pub claim_narrowing_required: bool,
    /// Free-text risk summary.
    pub risk_summary: String,
}

/// Evidence refs backing a correction row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorrectionEvidence {
    /// Evidence refs.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Protected-path refs.
    #[serde(default)]
    pub protected_path_refs: Vec<String>,
    /// Rerun refs.
    #[serde(default)]
    pub rerun_refs: Vec<String>,
    /// Adjacent failure-domain sweep refs.
    #[serde(default)]
    pub adjacent_sweep_refs: Vec<String>,
    /// Support packet refs.
    #[serde(default)]
    pub support_packet_refs: Vec<String>,
    /// Evidence freshness state such as `current`.
    pub freshness_state: String,
    /// Latest rerun timestamp.
    #[serde(default)]
    pub last_rerun_at: Option<String>,
}

/// Triage lane and fix-lineage refs for a correction row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorrectionTriage {
    /// Assigned triage lane.
    pub lane_decision: TriageLane,
    /// Decision state such as `triaged` or `deferred`.
    pub decision_state: String,
    /// Rationale for the lane decision.
    pub rationale: String,
    /// Decision owner.
    pub decision_owner: String,
    /// Decision timestamp.
    #[serde(default)]
    pub decided_at: Option<String>,
    /// Target train ref this row rides.
    #[serde(default)]
    pub target_train_ref: Option<String>,
    /// Hotfix packet ref when the lane is `hotfix`.
    #[serde(default)]
    pub hotfix_packet_ref: Option<String>,
    /// Correction packet ref for this row.
    #[serde(default)]
    pub correction_packet_ref: Option<String>,
    /// Backport matrix ref for this row.
    #[serde(default)]
    pub backport_matrix_ref: Option<String>,
}

/// One backport-matrix row: a decision for a single affected line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackportMatrixRow {
    /// Release line ref.
    pub release_line_ref: String,
    /// Support-line class such as `stable`, `lts`, or `beta`.
    pub support_line_class: String,
    /// Channel class for the line.
    pub channel_class: String,
    /// Whether the line is affected.
    #[serde(default)]
    pub affected: bool,
    /// Backport decision for the line.
    pub decision: BackportDecision,
    /// Rationale for the decision.
    #[serde(default)]
    pub rationale: Option<String>,
    /// Decision owner.
    #[serde(default)]
    pub decision_owner: Option<String>,
    /// Decision due date.
    #[serde(default)]
    pub decision_due_at: Option<String>,
    /// Target release ref for a `yes` decision.
    #[serde(default)]
    pub target_release_ref: Option<String>,
    /// Rollback target ref for a `yes` decision.
    #[serde(default)]
    pub rollback_target_ref: Option<String>,
    /// Known-issue ref updated in the same lane.
    #[serde(default)]
    pub known_issue_ref: Option<String>,
    /// Support-note ref updated in the same lane.
    #[serde(default)]
    pub support_note_ref: Option<String>,
    /// Docs/help update ref.
    #[serde(default)]
    pub docs_update_ref: Option<String>,
}

/// One target-channel disposition for a correction row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetChannelUpdate {
    /// Channel ref receiving the disposition.
    pub channel_ref: String,
    /// Channel class.
    pub channel_class: String,
    /// Disposition such as `ship_hotfix`, `ship_backport`, or `next_cycle`.
    pub disposition: String,
    /// Exact-build identity ref shipped on the channel, when known.
    #[serde(default)]
    pub exact_build_identity_ref: Option<String>,
    /// Rollback target ref for the channel.
    #[serde(default)]
    pub rollback_target_ref: Option<String>,
    /// Known-issue ref updated for the channel.
    #[serde(default)]
    pub known_issue_ref: Option<String>,
    /// Docs/help update ref for the channel.
    #[serde(default)]
    pub docs_update_ref: Option<String>,
    /// Support-note ref for the channel.
    #[serde(default)]
    pub support_note_ref: Option<String>,
}

/// Release-truth refs updated in the same lane as the correction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseNotesRefs {
    /// Known-issue statement ref.
    pub known_issue_statement_ref: String,
    /// Claim update refs.
    #[serde(default)]
    pub claim_update_refs: Vec<String>,
    /// Support-comms refs.
    #[serde(default)]
    pub support_comms_refs: Vec<String>,
}

/// Metadata-only support projection over the correction rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportProjection {
    /// Generated support projection ref.
    pub support_projection_ref: String,
    /// Support bundle refs.
    #[serde(default)]
    pub support_bundle_refs: Vec<String>,
    /// Surfaces allowed to quote this projection.
    #[serde(default)]
    pub consuming_surface_refs: Vec<String>,
    /// Shared correction vocabulary the projection carries.
    #[serde(default)]
    pub vocabulary_terms: Vec<String>,
    /// Redaction class applied to every row.
    pub redaction_class: String,
}

impl SupportProjection {
    fn validate(&self, packet_id: &str, violations: &mut Vec<CorrectionTrainViolation>) {
        let terms = self
            .vocabulary_terms
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for required in SHARED_PACKET_FORM_TERMS {
            if !terms.contains(required) {
                push_violation(
                    violations,
                    "support_projection.required_vocabulary_missing",
                    packet_id,
                    "support projection is missing required correction vocabulary",
                );
            }
        }
        if self.redaction_class.trim().is_empty() {
            push_violation(
                violations,
                "support_projection.redaction_class_empty",
                packet_id,
                "support projection must declare a redaction class",
            );
        }
    }
}

/// Validation failure emitted while checking a correction-train packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorrectionTrainViolation {
    /// Stable check id.
    pub check_id: String,
    /// Redaction-safe ref associated with the violation.
    pub ref_id: String,
    /// Redaction-safe validation message.
    pub message: String,
}

fn push_violation(
    violations: &mut Vec<CorrectionTrainViolation>,
    check_id: &str,
    ref_id: &str,
    message: &str,
) {
    violations.push(CorrectionTrainViolation {
        check_id: check_id.to_owned(),
        ref_id: ref_id.to_owned(),
        message: message.to_owned(),
    });
}
