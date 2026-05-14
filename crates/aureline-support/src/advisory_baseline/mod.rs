//! Incident/advisory baseline support projection.
//!
//! This module consumes the checked-in affected-build scope artifact at
//! `/artifacts/release/affected_build_scope_example.yaml` and projects it
//! into a metadata-only support packet. The projection keeps advisory copy,
//! exact-build identity, rollback routes, known-limit refs, and support/export
//! linkage joined without duplicating the advisory schemas or support-bundle
//! manifest format.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for the affected-build scope artifact.
pub const INCIDENT_ADVISORY_SCOPE_RECORD_KIND: &str = "incident_advisory_affected_build_scope";

/// Stable record-kind tag for the support/export projection.
pub const INCIDENT_ADVISORY_SUPPORT_PACKET_RECORD_KIND: &str =
    "incident_advisory_baseline_support_packet";

/// Current schema version for the affected-build scope artifact.
pub const INCIDENT_ADVISORY_SCOPE_SCHEMA_VERSION: u32 = 1;

/// Current schema version for the support/export projection.
pub const INCIDENT_ADVISORY_SUPPORT_PACKET_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path of the checked-in affected-build scope artifact.
pub const CURRENT_AFFECTED_BUILD_SCOPE_PATH: &str =
    "artifacts/release/affected_build_scope_example.yaml";

const CURRENT_AFFECTED_BUILD_SCOPE_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/affected_build_scope_example.yaml"
));

/// Loads the checked-in alpha advisory affected-build scope artifact.
///
/// # Errors
///
/// Returns a YAML parse error when the checked-in artifact does not match
/// [`AffectedBuildScopeExample`].
pub fn current_alpha_affected_build_scope() -> Result<AffectedBuildScopeExample, serde_yaml::Error>
{
    serde_yaml::from_str(CURRENT_AFFECTED_BUILD_SCOPE_YAML)
}

/// Machine-readable affected-build scope for the advisory rehearsal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffectedBuildScopeExample {
    /// Scope artifact schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable scope id for release/support joins.
    pub scope_id: String,
    /// Review posture such as `baseline_exercised`.
    pub status: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Owning reviewer or release/support DRI.
    pub owner_dri: String,
    /// UTC timestamp when the exercise was recorded.
    pub exercised_at: String,
    /// Source contracts and evidence packets consumed by this scope.
    pub source_contract_refs: BTreeMap<String, String>,
    /// Advisory identity and affected-install linkage used by the exercise.
    pub advisory: AdvisoryExercise,
    /// Exact-build rows affected by this exercise.
    pub affected_builds: Vec<AffectedBuildRow>,
    /// Support/export boundary for the first consumer projection.
    pub support_export_contract: SupportExportContract,
    /// Acceptance proof metadata for local verification.
    pub acceptance: AcceptanceEvidence,
}

impl AffectedBuildScopeExample {
    /// Validates the scope artifact against the baseline release/support rules.
    pub fn validate(&self) -> Vec<AdvisoryBaselineViolation> {
        let mut violations = Vec::new();

        if self.schema_version != INCIDENT_ADVISORY_SCOPE_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "scope.schema_version",
                &self.scope_id,
                "affected-build scope schema_version must be 1",
            );
        }
        if self.record_kind != INCIDENT_ADVISORY_SCOPE_RECORD_KIND {
            push_violation(
                &mut violations,
                "scope.record_kind",
                &self.scope_id,
                "affected-build scope record_kind is not supported",
            );
        }
        if self.advisory.advisory_id.trim().is_empty()
            || !self.advisory.advisory_id.starts_with("AURELINE-ADV-")
        {
            push_violation(
                &mut violations,
                "advisory.id",
                &self.scope_id,
                "advisory_id must be a stable Aureline advisory id",
            );
        }
        if self.affected_builds.is_empty() {
            push_violation(
                &mut violations,
                "affected_builds.empty",
                &self.scope_id,
                "at least one affected build row is required",
            );
        }

        for (key, value) in &self.source_contract_refs {
            if key.trim().is_empty() || value.trim().is_empty() {
                push_violation(
                    &mut violations,
                    "source_contract_refs.empty",
                    &self.scope_id,
                    "source contract refs must have non-empty keys and values",
                );
            }
        }

        let scoped_build_refs = self
            .affected_builds
            .iter()
            .map(|row| row.exact_build_identity_ref.as_str())
            .collect::<BTreeSet<_>>();
        let advisory_build_refs = self
            .advisory
            .affected_install_linkage
            .exact_build_identity_refs
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if scoped_build_refs != advisory_build_refs {
            push_violation(
                &mut violations,
                "affected_builds.linkage_mismatch",
                &self.scope_id,
                "affected build refs must match advisory affected-install linkage refs",
            );
        }

        for row in &self.affected_builds {
            row.validate(&mut violations);
        }

        if !self.support_export_contract.raw_private_material_excluded {
            push_violation(
                &mut violations,
                "support_export.raw_private_material_excluded",
                &self.support_export_contract.support_packet_id,
                "support projection must exclude raw private material",
            );
        }
        if self.support_export_contract.redaction_class != "metadata_safe_default" {
            push_violation(
                &mut violations,
                "support_export.redaction_class",
                &self.support_export_contract.support_packet_id,
                "support projection must use metadata_safe_default redaction",
            );
        }

        violations
    }

    /// Projects the scope into a metadata-only support packet.
    pub fn support_projection(
        &self,
        packet_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> AdvisoryBaselineSupportProjection {
        let rows = self
            .affected_builds
            .iter()
            .map(AdvisoryBaselineSupportRow::from_affected_build)
            .collect::<Vec<_>>();

        let exact_build_identity_refs = rows
            .iter()
            .map(|row| row.exact_build_identity_ref.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let rollback_route_refs = rows
            .iter()
            .map(|row| row.rollback_target_ref.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let known_limit_refs = rows
            .iter()
            .flat_map(|row| row.known_limit_refs.iter().cloned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let support_packet_refs = rows
            .iter()
            .flat_map(|row| row.support_packet_refs.iter().cloned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();

        AdvisoryBaselineSupportProjection {
            schema_version: INCIDENT_ADVISORY_SUPPORT_PACKET_SCHEMA_VERSION,
            record_kind: INCIDENT_ADVISORY_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            packet_id: packet_id.into(),
            generated_at: generated_at.into(),
            scope_id: self.scope_id.clone(),
            advisory_id: self.advisory.advisory_id.clone(),
            severity_class: self.advisory.severity_class.clone(),
            redaction_class: self.support_export_contract.redaction_class.clone(),
            data_class: self.support_export_contract.data_class.clone(),
            raw_private_material_excluded: self
                .support_export_contract
                .raw_private_material_excluded,
            exact_build_identity_refs,
            rollback_route_refs,
            known_limit_refs,
            support_packet_refs,
            rows,
        }
    }
}

/// Advisory identity, copy-safe IDs, and affected-install linkage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvisoryExercise {
    /// Stable Aureline advisory id used as the join key.
    pub advisory_id: String,
    /// Security severity vocabulary token.
    pub severity_class: String,
    /// Compact surface severity vocabulary token.
    pub surface_severity_class: String,
    /// Disclosure class from the advisory schemas.
    pub disclosure_class: String,
    /// Action-state token rendered by advisory surfaces.
    pub action_state: String,
    /// Opaque advisory record ref projected by consumers.
    pub advisory_record_ref: String,
    /// Repository-relative template ref used by future advisory drafts.
    pub advisory_template_ref: String,
    /// Redaction-safe summary of the advisory exercise.
    pub summary: String,
    /// Copy-safe advisory aliases visible in the exercise.
    pub copy_safe_ids: Vec<CopySafeAdvisoryId>,
    /// Affected-install linkage matching the advisory schema vocabulary.
    pub affected_install_linkage: AffectedInstallLinkage,
}

/// One copy-safe advisory ID row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopySafeAdvisoryId {
    /// ID kind such as `aureline_advisory_id`, `cve_id`, or `ghsa_id`.
    pub id_kind: String,
    /// Exact ID value or sentinel value when no public alias exists.
    pub id_value: String,
    /// Copy availability state.
    pub copy_state: String,
    /// Visibility boundary for this ID.
    pub visibility_class: String,
    /// Source record ref for this ID row.
    pub source_ref: String,
}

/// Affected-install linkage shared by advisory and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffectedInstallLinkage {
    /// Install-profile card refs named by the advisory.
    pub install_profile_card_refs: Vec<String>,
    /// Exact-build identity refs named by the advisory.
    pub exact_build_identity_refs: Vec<String>,
    /// Channel classes in scope.
    pub channel_classes: Vec<String>,
    /// Publication postures in scope.
    pub publication_posture_classes: Vec<String>,
    /// Mirror/offline freshness state.
    pub mirror_freshness_class: String,
    /// Offline expiration timestamp when applicable.
    pub offline_expiration_at: Option<String>,
    /// Copy-safe note describing what remains available locally.
    pub local_continuity_note: String,
}

/// One affected exact-build row in the advisory exercise.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffectedBuildRow {
    /// Stable row id for the affected build.
    pub build_scope_id: String,
    /// Exact-build identity ref for the affected build.
    pub exact_build_identity_ref: String,
    /// Source artifact that proves or names the exact-build identity.
    pub exact_build_identity_source_ref: String,
    /// Release channel class for this affected row.
    pub channel_class: String,
    /// Install-profile card refs associated with the affected row.
    pub install_profile_card_refs: Vec<String>,
    /// How this row matches the current install or packet.
    pub installed_match_state: String,
    /// Current mitigation state and support packet refs.
    pub mitigation: MitigationState,
    /// Rollback or publication-hold route for this row.
    pub rollback_route: RollbackRoute,
    /// Known limits that narrow the advisory claim.
    pub known_limits: KnownLimitTruth,
}

impl AffectedBuildRow {
    fn validate(&self, violations: &mut Vec<AdvisoryBaselineViolation>) {
        if !self
            .exact_build_identity_ref
            .starts_with("build-id:aureline:")
        {
            push_violation(
                violations,
                "affected_build.exact_build_identity_ref",
                &self.build_scope_id,
                "affected build must name an Aureline exact-build identity",
            );
        }
        if self.install_profile_card_refs.is_empty() {
            push_violation(
                violations,
                "affected_build.install_profile_card_refs",
                &self.build_scope_id,
                "affected build must name at least one install-profile card ref",
            );
        }
        if self.mitigation.current_mitigation_summary.trim().is_empty()
            || self.mitigation.support_packet_refs.is_empty()
        {
            push_violation(
                violations,
                "affected_build.mitigation",
                &self.build_scope_id,
                "affected build must carry mitigation summary and support packet refs",
            );
        }
        if self.rollback_route.rollback_target_ref.trim().is_empty()
            || self.rollback_route.checkpoint_ref.trim().is_empty()
            || self.rollback_route.preserved_state_classes.is_empty()
        {
            push_violation(
                violations,
                "affected_build.rollback_route",
                &self.build_scope_id,
                "affected build must carry rollback target, checkpoint, and preserved state classes",
            );
        }
        if self.known_limits.summary.trim().is_empty()
            || self.known_limits.known_limit_refs.is_empty()
            || self.known_limits.overclaim_blockers.is_empty()
        {
            push_violation(
                violations,
                "affected_build.known_limits",
                &self.build_scope_id,
                "affected build must carry known-limit refs and overclaim blockers",
            );
        }
    }
}

/// Current mitigation state for one affected build.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MitigationState {
    /// Mitigation state token from the affected-install vocabulary.
    pub mitigation_state: String,
    /// Copy-safe summary of the current mitigation.
    pub current_mitigation_summary: String,
    /// Copy-safe local continuity note.
    pub local_continuity_note: String,
    /// Support packet refs that can reconstruct this mitigation.
    pub support_packet_refs: Vec<String>,
}

/// Rollback or publication-hold route for one affected build.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackRoute {
    /// Route class such as `rollback_or_repin`.
    pub route_class: String,
    /// Opaque rollback target ref.
    pub rollback_target_ref: String,
    /// Repository-relative update rollback sequence ref.
    pub rollback_sequence_ref: String,
    /// Checkpoint ref required before the route can proceed.
    pub checkpoint_ref: String,
    /// Reversal class shown to the user or reviewer.
    pub reversal_class: String,
    /// Copy-safe rollback summary.
    pub user_visible_summary: String,
    /// State classes the route must preserve.
    pub preserved_state_classes: Vec<String>,
}

/// Known-limit truth that narrows one affected-build claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnownLimitTruth {
    /// Truth state such as `known_limited` or `evidence_stale`.
    pub truth_state: String,
    /// Copy-safe summary of the known limit.
    pub summary: String,
    /// Evidence refs backing the known-limit statement.
    pub known_limit_refs: Vec<String>,
    /// Overclaim blockers that prevent broader health or compatibility claims.
    pub overclaim_blockers: Vec<String>,
}

/// Support/export boundary declared by the affected-build scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportContract {
    /// Stable support packet id.
    pub support_packet_id: String,
    /// Redaction class for the support projection.
    pub redaction_class: String,
    /// Diagnostic data class for the support projection.
    pub data_class: String,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Support packets linked by this projection.
    pub linked_support_packet_refs: Vec<String>,
    /// Export surface refs that can consume this projection.
    pub export_surface_refs: Vec<String>,
    /// Copy-safe export summary.
    pub export_summary: String,
}

/// Acceptance proof metadata embedded in the scope artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcceptanceEvidence {
    /// Command that validates this baseline.
    pub validator_command: String,
    /// Rust consumer module path.
    pub rust_consumer: String,
    /// Proof packet and template refs.
    pub proof_refs: Vec<String>,
    /// Protected fixture refs consumed by this baseline.
    pub protected_fixture_refs: Vec<String>,
    /// Acceptance states exercised by the proof path.
    pub accepted_states: Vec<String>,
}

/// Metadata-only support/export projection for the advisory baseline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvisoryBaselineSupportProjection {
    /// Support projection schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable support packet id.
    pub packet_id: String,
    /// UTC timestamp when this projection was generated.
    pub generated_at: String,
    /// Source affected-build scope id.
    pub scope_id: String,
    /// Advisory id projected by this packet.
    pub advisory_id: String,
    /// Severity class projected by this packet.
    pub severity_class: String,
    /// Redaction class for the packet.
    pub redaction_class: String,
    /// Diagnostic data class for the packet.
    pub data_class: String,
    /// Whether raw private material is excluded from the packet.
    pub raw_private_material_excluded: bool,
    /// Exact-build refs covered by the packet.
    pub exact_build_identity_refs: Vec<String>,
    /// Rollback route refs covered by the packet.
    pub rollback_route_refs: Vec<String>,
    /// Known-limit refs covered by the packet.
    pub known_limit_refs: Vec<String>,
    /// Linked support packet refs.
    pub support_packet_refs: Vec<String>,
    /// Per-build support rows.
    pub rows: Vec<AdvisoryBaselineSupportRow>,
}

impl AdvisoryBaselineSupportProjection {
    /// Returns true when the projection is safe for metadata-only export.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.redaction_class == "metadata_safe_default"
            && self.data_class == "metadata_only"
            && self
                .rows
                .iter()
                .all(AdvisoryBaselineSupportRow::is_export_safe)
    }
}

/// One affected-build row in the support/export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvisoryBaselineSupportRow {
    /// Stable affected-build row id.
    pub build_scope_id: String,
    /// Exact-build identity ref for the row.
    pub exact_build_identity_ref: String,
    /// Release channel class for the row.
    pub channel_class: String,
    /// Current mitigation state for the row.
    pub mitigation_state: String,
    /// Rollback target ref for the row.
    pub rollback_target_ref: String,
    /// Known-limit refs for the row.
    pub known_limit_refs: Vec<String>,
    /// Support packet refs for the row.
    pub support_packet_refs: Vec<String>,
    /// Export-safe row summary.
    pub export_safe_summary: String,
}

impl AdvisoryBaselineSupportRow {
    fn from_affected_build(row: &AffectedBuildRow) -> Self {
        Self {
            build_scope_id: row.build_scope_id.clone(),
            exact_build_identity_ref: row.exact_build_identity_ref.clone(),
            channel_class: row.channel_class.clone(),
            mitigation_state: row.mitigation.mitigation_state.clone(),
            rollback_target_ref: row.rollback_route.rollback_target_ref.clone(),
            known_limit_refs: row.known_limits.known_limit_refs.clone(),
            support_packet_refs: row.mitigation.support_packet_refs.clone(),
            export_safe_summary: format!(
                "{}; rollback: {}; known limits: {}",
                row.mitigation.current_mitigation_summary.trim(),
                row.rollback_route.user_visible_summary.trim(),
                row.known_limits.summary.trim()
            ),
        }
    }

    /// Returns true when this row has all metadata needed for support export.
    pub fn is_export_safe(&self) -> bool {
        self.exact_build_identity_ref
            .starts_with("build-id:aureline:")
            && !self.rollback_target_ref.trim().is_empty()
            && !self.known_limit_refs.is_empty()
            && !self.support_packet_refs.is_empty()
            && !self.export_safe_summary.trim().is_empty()
    }
}

/// One validation issue found in the advisory baseline artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvisoryBaselineViolation {
    /// Stable validation check id.
    pub check_id: String,
    /// Artifact or row ref associated with the violation.
    pub ref_id: String,
    /// Copy-safe validation message.
    pub message: String,
}

fn push_violation(
    violations: &mut Vec<AdvisoryBaselineViolation>,
    check_id: &str,
    ref_id: &str,
    message: &str,
) {
    violations.push(AdvisoryBaselineViolation {
        check_id: check_id.to_owned(),
        ref_id: ref_id.to_owned(),
        message: message.to_owned(),
    });
}
