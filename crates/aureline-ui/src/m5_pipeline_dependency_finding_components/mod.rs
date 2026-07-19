//! Cross-surface proof for M5 pipeline, dependency, manifest-diff, annotation,
//! and security-finding components.
//!
//! This module validates the release proof at
//! `artifacts/release/m5-pipeline-dependency-finding-proof/proof_packet.json`
//! against the typed component fixtures in this crate. It is intentionally a
//! consumer proof rather than another component: it checks that review panes,
//! package-manager/package-center views, project-health centers, companion
//! clients, support export, and release proof consume one shared vocabulary and
//! narrow action authority explicitly when they cannot offer the full desktop
//! action set.

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_annotation_rows::current_m5_annotation_row;
use crate::m5_dependency_rows::current_m5_dependency_row;
use crate::m5_manifest_diff_cards::current_m5_manifest_diff_card;
use crate::m5_pipeline_run_rows::current_m5_pipeline_run_row;
use crate::m5_security_finding_cards::current_m5_security_finding_card;

/// Stable record-kind tag carried by [`M5PipelineDependencyFindingComponentProof`].
pub const M5_PIPELINE_DEPENDENCY_FINDING_COMPONENT_PROOF_RECORD_KIND: &str =
    "m5_pipeline_dependency_finding_component_proof";

/// Schema version for the component proof.
pub const M5_PIPELINE_DEPENDENCY_FINDING_COMPONENT_PROOF_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the checked release proof packet.
pub const M5_PIPELINE_DEPENDENCY_FINDING_COMPONENT_PROOF_REF: &str =
    "artifacts/release/m5-pipeline-dependency-finding-proof/proof_packet.json";

/// Repo-relative path of the matrix the proof cites.
pub const M5_PIPELINE_DEPENDENCY_FINDING_COMPONENT_MATRIX_REF: &str =
    "artifacts/design/m5-pipeline-dependency-finding-component-matrix.md";

/// Embedded checked-in proof JSON.
pub const M5_PIPELINE_DEPENDENCY_FINDING_COMPONENT_PROOF_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5-pipeline-dependency-finding-proof/proof_packet.json"
));

/// Component family in the proof packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentFamily {
    /// Pipeline run row.
    PipelineRunRow,
    /// Annotation row.
    AnnotationRow,
    /// Dependency row.
    DependencyRow,
    /// Manifest diff card.
    ManifestDiffCard,
    /// Security finding card.
    SecurityFindingCard,
}

impl ComponentFamily {
    /// Every component family, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PipelineRunRow,
        Self::AnnotationRow,
        Self::DependencyRow,
        Self::ManifestDiffCard,
        Self::SecurityFindingCard,
    ];

    /// Stable token recorded in the proof.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PipelineRunRow => "pipeline_run_row",
            Self::AnnotationRow => "annotation_row",
            Self::DependencyRow => "dependency_row",
            Self::ManifestDiffCard => "manifest_diff_card",
            Self::SecurityFindingCard => "security_finding_card",
        }
    }

    fn required_consumers(self) -> &'static [&'static str] {
        match self {
            Self::PipelineRunRow => &[
                "review_pane",
                "pipeline_viewer",
                "project_health_center",
                "companion_client",
                "support_export",
                "release_proof",
            ],
            Self::AnnotationRow => &[
                "code_surface",
                "review_pane",
                "project_health_center",
                "companion_client",
                "support_export",
                "release_proof",
            ],
            Self::DependencyRow => &[
                "package_manager",
                "review_pane",
                "project_health_center",
                "framework_pack_health",
                "companion_client",
                "support_export",
                "release_proof",
            ],
            Self::ManifestDiffCard => &[
                "package_manager",
                "review_pane",
                "project_health_center",
                "companion_client",
                "support_export",
                "release_proof",
            ],
            Self::SecurityFindingCard => &[
                "review_pane",
                "package_manager",
                "project_health_center",
                "framework_pack_health",
                "companion_client",
                "support_export",
                "release_proof",
            ],
        }
    }

    fn fixture_ref(self) -> &'static str {
        match self {
            Self::PipelineRunRow => {
                "fixtures/ui/m5-pipeline-dependency-finding-components/pipeline_run_row.json"
            }
            Self::AnnotationRow => {
                "fixtures/ui/m5-pipeline-dependency-finding-components/annotation_row.json"
            }
            Self::DependencyRow => {
                "fixtures/ui/m5-pipeline-dependency-finding-components/dependency_row.json"
            }
            Self::ManifestDiffCard => {
                "fixtures/ui/m5-pipeline-dependency-finding-components/manifest_diff_card.json"
            }
            Self::SecurityFindingCard => {
                "fixtures/ui/m5-pipeline-dependency-finding-components/security_finding_card.json"
            }
        }
    }
}

/// One component-family row in the proof packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentFamilyProofRow {
    /// Component family.
    pub family: ComponentFamily,
    /// Schema ref.
    pub schema_ref: String,
    /// Fixture ref.
    pub fixture_ref: String,
    /// Source binding refs.
    pub source_bindings: Vec<String>,
    /// Consumer surfaces.
    pub consumer_surfaces: Vec<String>,
    /// Controlled labels covered by this proof row.
    pub controlled_label_coverage: Vec<String>,
    /// Degraded states covered by this proof row.
    pub degraded_state_coverage: Vec<String>,
    /// Export parity summary token.
    pub export_parity: String,
}

/// Copy/export invariants declared by the proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CopyExportInvariants {
    /// Required copy/export formats.
    pub formats: Vec<String>,
    /// Whether screenshot-only reconstruction is forbidden.
    pub screenshot_only_prohibited: bool,
    /// Whether controlled labels are preserved.
    pub controlled_labels_preserved: bool,
    /// Whether source refs are required.
    pub source_refs_required: bool,
}

/// Security-finding-specific invariants declared by the proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecurityFindingCardInvariants {
    /// Finding class is required.
    pub finding_class_required: bool,
    /// Package, secret, policy, and code-analysis findings share one grammar.
    pub package_secret_policy_and_code_analysis_share_one_grammar: bool,
    /// Affected scope refs are required.
    pub affected_scope_refs_required: bool,
    /// Fix availability stays separate from remediation.
    pub fix_availability_separate_from_remediation: bool,
    /// Controlled suppression display label is required.
    pub controlled_suppression_display_label_required: bool,
    /// Safest next step is required.
    pub safest_next_step_required: bool,
    /// Local validation option is required.
    pub local_validation_option_required: bool,
    /// Docs/help path is required.
    pub docs_help_path_required: bool,
    /// Audit actions are included in support export.
    pub audit_actions_included_in_support_export: bool,
}

/// Freshness contract for the checked component proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProofFreshness {
    /// Maximum age of the proof before its claims narrow.
    pub freshness_slo_hours: u32,
    /// UTC timestamp of the most recent proof refresh.
    pub last_refresh: String,
    /// Whether the checked proof is currently fresh.
    pub proof_fresh: bool,
    /// Whether stale evidence automatically narrows the claim.
    pub auto_narrow_on_stale: bool,
    /// Effect applied when the freshness check fails.
    pub stale_failure_effect: String,
}

/// One parity check shared by all certified component consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParityCheck {
    /// Stable parity-check identifier.
    pub check_id: String,
    /// Component and consumer scope covered by the check.
    pub scope: String,
    /// Condition the consumer must satisfy.
    #[serde(rename = "requires")]
    pub requirement: String,
    /// Effect applied when the parity check fails.
    pub failure_effect: String,
}

/// Certification of one claimed consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConsumerCertification {
    /// Stable consumer token.
    pub consumer: String,
    /// Claimed M5 surface.
    pub claimed_surface: String,
    /// Current certification state.
    pub claim_state: String,
    /// Component families certified on this consumer.
    pub component_families: Vec<ComponentFamily>,
    /// Freshness check used by this certification.
    pub freshness_check: String,
    /// Parity checks used by this certification.
    pub parity_check_refs: Vec<String>,
    /// Rule applied when a required check fails.
    pub narrowing_rule: String,
}

/// Promotion gate declared by the proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionGate {
    /// Schemas are checked in.
    pub schemas_checked_in: bool,
    /// Fixtures are checked in.
    pub fixtures_checked_in: bool,
    /// Matrix is checked in.
    pub matrix_checked_in: bool,
    /// Release proof is checked in.
    pub release_proof_checked_in: bool,
    /// Support export is checked in.
    pub support_export_checked_in: bool,
    /// Consumer certifications are checked in.
    pub consumer_certifications_checked_in: bool,
    /// Freshness and parity failures narrow affected claims.
    pub freshness_and_parity_checks_narrow_claims: bool,
    /// First consumers can reference one baseline.
    pub first_consumers_can_reference_one_baseline: bool,
}

/// Typed release proof for the component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PipelineDependencyFindingComponentProof {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Proof id.
    pub proof_id: String,
    /// Proof status.
    pub status: String,
    /// Matrix ref.
    pub matrix_ref: String,
    /// UTC date this proof is current as of.
    pub as_of: String,
    /// Freshness contract for the proof.
    pub proof_freshness: ProofFreshness,
    /// Shared parity checks for claimed consumers.
    pub parity_checks: Vec<ParityCheck>,
    /// Per-consumer certifications.
    pub consumer_certifications: Vec<ConsumerCertification>,
    /// Component family proof rows.
    pub component_families: Vec<ComponentFamilyProofRow>,
    /// Suppression vocabulary.
    pub suppression_vocabulary: Vec<String>,
    /// Copy/export invariants.
    pub copy_export_invariants: CopyExportInvariants,
    /// Security-finding invariants.
    pub security_finding_card_invariants: SecurityFindingCardInvariants,
    /// Promotion gate.
    pub promotion_gate: PromotionGate,
}

/// A normalized consumer projection row across component families.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentConsumerProjection {
    /// Component family.
    pub family: ComponentFamily,
    /// Consumer surface.
    pub consumer_surface: String,
    /// Primary row/card id.
    pub component_id: String,
    /// State or status token.
    pub state: String,
    /// Freshness token.
    pub freshness_state: String,
    /// Action or authority token.
    pub action_or_authority: String,
    /// Limited-action note; `not_applicable` means no narrowing occurred.
    pub limited_action_note: String,
}

impl M5PipelineDependencyFindingComponentProof {
    /// Returns the proof row for a family.
    #[must_use]
    pub fn family_row(&self, family: ComponentFamily) -> Option<&ComponentFamilyProofRow> {
        self.component_families
            .iter()
            .find(|row| row.family == family)
    }

    /// Returns all validation violations for this proof and its checked-in fixtures.
    #[must_use]
    pub fn validate(&self) -> Vec<ComponentProofViolation> {
        use ComponentProofViolation as V;

        let mut violations = Vec::new();
        if self.record_kind != M5_PIPELINE_DEPENDENCY_FINDING_COMPONENT_PROOF_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_PIPELINE_DEPENDENCY_FINDING_COMPONENT_PROOF_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.matrix_ref != M5_PIPELINE_DEPENDENCY_FINDING_COMPONENT_MATRIX_REF {
            violations.push(V::MatrixRefMismatch);
        }
        if any_blank([&self.proof_id, &self.status, &self.as_of]) {
            violations.push(V::MissingProofIdentity);
        }

        if self.proof_freshness.freshness_slo_hours == 0
            || self.proof_freshness.last_refresh.trim().is_empty()
            || !self.proof_freshness.proof_fresh
            || !self.proof_freshness.auto_narrow_on_stale
            || self.proof_freshness.stale_failure_effect != "narrow_claim"
        {
            violations.push(V::ProofFreshnessInvalid);
        }

        let required_parity_checks: BTreeSet<&str> = [
            "required_field_parity",
            "controlled_label_parity",
            "action_vocabulary_parity",
            "degraded_state_parity",
            "suppression_visibility",
            "copy_export_parity",
        ]
        .into_iter()
        .collect();
        let mut parity_check_ids = BTreeSet::new();
        let parity_checks_valid = self.parity_checks.iter().all(|check| {
            parity_check_ids.insert(check.check_id.as_str())
                && !check.scope.trim().is_empty()
                && !check.requirement.trim().is_empty()
                && check.failure_effect == "narrow_claim"
        });
        if !parity_checks_valid || parity_check_ids != required_parity_checks {
            violations.push(V::ParityChecksIncomplete);
        }

        let mut certification_consumers = BTreeSet::new();
        let certifications_valid = self.consumer_certifications.iter().all(|certification| {
            let expected = match certification.consumer.as_str() {
                "review_pane" => Some(("m5_review_surface", ComponentFamily::ALL.as_slice())),
                "package_manager" => Some((
                    "m5_package_surface",
                    &[
                        ComponentFamily::DependencyRow,
                        ComponentFamily::ManifestDiffCard,
                        ComponentFamily::SecurityFindingCard,
                    ][..],
                )),
                "project_health_center" => {
                    Some(("m5_health_surface", ComponentFamily::ALL.as_slice()))
                }
                "companion_client" => Some((
                    "m5_narrow_companion_surface",
                    ComponentFamily::ALL.as_slice(),
                )),
                _ => None,
            };
            let Some((claimed_surface, expected_families)) = expected else {
                return false;
            };
            let actual_families: BTreeSet<ComponentFamily> =
                certification.component_families.iter().copied().collect();
            let expected_families: BTreeSet<ComponentFamily> =
                expected_families.iter().copied().collect();
            let actual_checks: BTreeSet<&str> = certification
                .parity_check_refs
                .iter()
                .map(String::as_str)
                .collect();
            certification_consumers.insert(certification.consumer.as_str())
                && certification.claimed_surface == claimed_surface
                && certification.claim_state == "passed"
                && actual_families == expected_families
                && certification.freshness_check == "proof_freshness.proof_fresh"
                && actual_checks == required_parity_checks
                && !certification.narrowing_rule.trim().is_empty()
        });
        let required_consumers: BTreeSet<&str> = [
            "review_pane",
            "package_manager",
            "project_health_center",
            "companion_client",
        ]
        .into_iter()
        .collect();
        if !certifications_valid || certification_consumers != required_consumers {
            violations.push(V::ConsumerCertificationsIncomplete);
        }

        let mut seen = BTreeSet::new();
        for row in &self.component_families {
            if !seen.insert(row.family) {
                violations.push(V::DuplicateFamily(row.family));
            }
            if row.fixture_ref != row.family.fixture_ref() {
                violations.push(V::FixtureRefMismatch(row.family));
            }
            if row.source_bindings.is_empty()
                || row.controlled_label_coverage.is_empty()
                || row.degraded_state_coverage.is_empty()
                || row.export_parity.trim().is_empty()
            {
                violations.push(V::FamilyCoverageMissing(row.family));
            }
            if !has_all(
                &row.consumer_surfaces,
                row.family.required_consumers().iter().copied(),
            ) {
                violations.push(V::ConsumerSurfaceMissing(row.family));
            }
        }
        for family in ComponentFamily::ALL {
            if !seen.contains(&family) {
                violations.push(V::FamilyMissing(family));
            }
        }

        if self.suppression_vocabulary
            != vec![
                "unsuppressed",
                "suppressed_until_review",
                "suppressed_by_policy",
                "exception_expired",
            ]
        {
            violations.push(V::SuppressionVocabularyMismatch);
        }
        if !has_all(
            &self.copy_export_invariants.formats,
            ["text", "json", "markdown"],
        ) || !self.copy_export_invariants.screenshot_only_prohibited
            || !self.copy_export_invariants.controlled_labels_preserved
            || !self.copy_export_invariants.source_refs_required
        {
            violations.push(V::CopyExportInvariantMissing);
        }
        if !self.security_finding_card_invariants.finding_class_required
            || !self
                .security_finding_card_invariants
                .package_secret_policy_and_code_analysis_share_one_grammar
            || !self
                .security_finding_card_invariants
                .affected_scope_refs_required
            || !self
                .security_finding_card_invariants
                .fix_availability_separate_from_remediation
            || !self
                .security_finding_card_invariants
                .controlled_suppression_display_label_required
            || !self
                .security_finding_card_invariants
                .safest_next_step_required
            || !self
                .security_finding_card_invariants
                .local_validation_option_required
            || !self
                .security_finding_card_invariants
                .docs_help_path_required
            || !self
                .security_finding_card_invariants
                .audit_actions_included_in_support_export
        {
            violations.push(V::SecurityFindingInvariantMissing);
        }
        if !self.promotion_gate.schemas_checked_in
            || !self.promotion_gate.fixtures_checked_in
            || !self.promotion_gate.matrix_checked_in
            || !self.promotion_gate.release_proof_checked_in
            || !self.promotion_gate.support_export_checked_in
            || !self.promotion_gate.consumer_certifications_checked_in
            || !self
                .promotion_gate
                .freshness_and_parity_checks_narrow_claims
            || !self
                .promotion_gate
                .first_consumers_can_reference_one_baseline
        {
            violations.push(V::PromotionGateIncomplete);
        }

        if current_m5_pipeline_run_row().is_err() {
            violations.push(V::FixtureValidationFailed(ComponentFamily::PipelineRunRow));
        }
        if current_m5_annotation_row().is_err() {
            violations.push(V::FixtureValidationFailed(ComponentFamily::AnnotationRow));
        }
        if current_m5_dependency_row().is_err() {
            violations.push(V::FixtureValidationFailed(ComponentFamily::DependencyRow));
        }
        if current_m5_manifest_diff_card().is_err() {
            violations.push(V::FixtureValidationFailed(
                ComponentFamily::ManifestDiffCard,
            ));
        }
        if current_m5_security_finding_card().is_err() {
            violations.push(V::FixtureValidationFailed(
                ComponentFamily::SecurityFindingCard,
            ));
        }

        match self.consumer_projections() {
            Ok(projections) => validate_projection_parity(&projections, &mut violations),
            Err(family) => violations.push(V::FixtureValidationFailed(family)),
        }

        violations
    }

    /// Builds normalized consumer projections from the five checked-in fixtures.
    ///
    /// # Errors
    ///
    /// Returns the family whose fixture could not load or validate.
    pub fn consumer_projections(
        &self,
    ) -> Result<Vec<ComponentConsumerProjection>, ComponentFamily> {
        let pipeline =
            current_m5_pipeline_run_row().map_err(|_| ComponentFamily::PipelineRunRow)?;
        let annotation = current_m5_annotation_row().map_err(|_| ComponentFamily::AnnotationRow)?;
        let dependency = current_m5_dependency_row().map_err(|_| ComponentFamily::DependencyRow)?;
        let manifest =
            current_m5_manifest_diff_card().map_err(|_| ComponentFamily::ManifestDiffCard)?;
        let security =
            current_m5_security_finding_card().map_err(|_| ComponentFamily::SecurityFindingCard)?;

        let mut projections = Vec::new();
        for surface in &pipeline.consumer_surfaces {
            let projection = pipeline
                .projection_for(surface)
                .expect("surface came from pipeline row");
            projections.push(ComponentConsumerProjection {
                family: ComponentFamily::PipelineRunRow,
                consumer_surface: surface.clone(),
                component_id: projection.row_id,
                state: projection.normalized_status,
                freshness_state: projection.freshness_state,
                action_or_authority: projection.authority_label,
                limited_action_note: projection.limited_action_note,
            });
        }
        for surface in &annotation.consumer_surfaces {
            let projection = annotation
                .projection_for(surface)
                .expect("surface came from annotation row");
            projections.push(ComponentConsumerProjection {
                family: ComponentFamily::AnnotationRow,
                consumer_surface: surface.clone(),
                component_id: projection.row_id,
                state: projection.suppression_state,
                freshness_state: projection.freshness_state,
                action_or_authority: projection.open_details_action_id,
                limited_action_note: projection.stale_handoff_reason,
            });
        }
        for surface in &dependency.consumer_surfaces {
            let projection = dependency
                .projection_for(surface)
                .expect("surface came from dependency row");
            projections.push(ComponentConsumerProjection {
                family: ComponentFamily::DependencyRow,
                consumer_surface: surface.clone(),
                component_id: projection.row_id,
                state: projection.update_state.clone(),
                freshness_state: projection.freshness_state,
                action_or_authority: projection.update_state,
                limited_action_note: projection.degraded_state,
            });
        }
        for surface in &manifest.consumer_surfaces {
            let projection = manifest
                .projection_for(surface)
                .expect("surface came from manifest diff card");
            projections.push(ComponentConsumerProjection {
                family: ComponentFamily::ManifestDiffCard,
                consumer_surface: surface.clone(),
                component_id: projection.card_id,
                state: projection.degraded_state.clone(),
                freshness_state: projection.freshness_state,
                action_or_authority: if surface == "companion_client" {
                    "inspect_only".to_owned()
                } else {
                    projection.write_authority.clone()
                },
                limited_action_note: if surface == "companion_client" {
                    format!(
                        "companion_client_narrows_{}_to_inspect_only",
                        projection.write_authority
                    )
                } else {
                    projection.limited_action_note
                },
            });
        }
        for surface in &security.consumer_surfaces {
            let projection = security
                .projection_for(surface)
                .expect("surface came from security finding card");
            projections.push(ComponentConsumerProjection {
                family: ComponentFamily::SecurityFindingCard,
                consumer_surface: surface.clone(),
                component_id: projection.card_id,
                state: projection.suppression_state,
                freshness_state: projection.freshness_state,
                action_or_authority: projection.safest_next_step,
                limited_action_note: projection.fix_availability_state,
            });
        }

        Ok(projections)
    }
}

fn validate_projection_parity(
    projections: &[ComponentConsumerProjection],
    violations: &mut Vec<ComponentProofViolation>,
) {
    let mut by_family: BTreeMap<ComponentFamily, Vec<&ComponentConsumerProjection>> =
        BTreeMap::new();
    for projection in projections {
        by_family
            .entry(projection.family)
            .or_default()
            .push(projection);
        if any_blank([
            &projection.consumer_surface,
            &projection.component_id,
            &projection.state,
            &projection.freshness_state,
            &projection.action_or_authority,
            &projection.limited_action_note,
        ]) {
            violations.push(ComponentProofViolation::ProjectionFieldMissing(
                projection.family,
            ));
        }
    }

    for family in ComponentFamily::ALL {
        let Some(rows) = by_family.get(&family) else {
            violations.push(ComponentProofViolation::FamilyMissing(family));
            continue;
        };
        let surfaces: Vec<String> = rows
            .iter()
            .map(|projection| projection.consumer_surface.clone())
            .collect();
        if !has_all(&surfaces, family.required_consumers().iter().copied()) {
            violations.push(ComponentProofViolation::ConsumerSurfaceMissing(family));
        }
        let ids: BTreeSet<&str> = rows
            .iter()
            .map(|projection| projection.component_id.as_str())
            .collect();
        if ids.len() != 1 {
            violations.push(ComponentProofViolation::ProjectionIdentityDrift(family));
        }
    }

    for projection in projections
        .iter()
        .filter(|projection| projection.consumer_surface == "companion_client")
    {
        match projection.family {
            ComponentFamily::PipelineRunRow
                if projection.action_or_authority != "allowed"
                    && projection.limited_action_note == "not_applicable" =>
            {
                violations.push(ComponentProofViolation::LimitedActionNarrowingMissing(
                    projection.family,
                ));
            }
            ComponentFamily::DependencyRow
                if projection.action_or_authority != "available"
                    && projection.limited_action_note == "none" =>
            {
                violations.push(ComponentProofViolation::LimitedActionNarrowingMissing(
                    projection.family,
                ));
            }
            ComponentFamily::ManifestDiffCard
                if projection.action_or_authority != "inspect_only"
                    || !projection.limited_action_note.contains("narrows") =>
            {
                violations.push(ComponentProofViolation::LimitedActionNarrowingMissing(
                    projection.family,
                ));
            }
            ComponentFamily::SecurityFindingCard
                if projection.limited_action_note.trim().is_empty() =>
            {
                violations.push(ComponentProofViolation::LimitedActionNarrowingMissing(
                    projection.family,
                ));
            }
            _ => {}
        }
    }
}

/// Error returned when the checked-in component proof fails to load.
#[derive(Debug)]
pub enum ComponentProofArtifactError {
    /// The proof could not be parsed.
    Fixture(serde_json::Error),
    /// The parsed proof failed validation.
    Validation(Vec<ComponentProofViolation>),
}

impl fmt::Display for ComponentProofArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fixture(err) => write!(f, "component proof parse error: {err}"),
            Self::Validation(violations) => {
                write!(f, "component proof failed validation: {violations:?}")
            }
        }
    }
}

impl Error for ComponentProofArtifactError {}

/// A validation invariant the component proof can violate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentProofViolation {
    /// The record-kind tag is wrong.
    WrongRecordKind,
    /// The schema version is wrong.
    WrongSchemaVersion,
    /// Proof identity is incomplete.
    MissingProofIdentity,
    /// Matrix ref is not canonical.
    MatrixRefMismatch,
    /// Proof freshness does not fail closed by narrowing the claim.
    ProofFreshnessInvalid,
    /// Required parity checks are missing, duplicated, or malformed.
    ParityChecksIncomplete,
    /// Required consumer certifications are missing, duplicated, or malformed.
    ConsumerCertificationsIncomplete,
    /// A component family is missing.
    FamilyMissing(ComponentFamily),
    /// A component family is duplicated.
    DuplicateFamily(ComponentFamily),
    /// Fixture ref does not match the canonical fixture.
    FixtureRefMismatch(ComponentFamily),
    /// Source, label, degraded-state, or export coverage is missing.
    FamilyCoverageMissing(ComponentFamily),
    /// Required consumer surface is missing.
    ConsumerSurfaceMissing(ComponentFamily),
    /// Suppression vocabulary is not canonical.
    SuppressionVocabularyMismatch,
    /// Copy/export invariants are incomplete.
    CopyExportInvariantMissing,
    /// Security-finding invariants are incomplete.
    SecurityFindingInvariantMissing,
    /// Promotion gate is incomplete.
    PromotionGateIncomplete,
    /// A typed component fixture failed validation.
    FixtureValidationFailed(ComponentFamily),
    /// Projection field is missing.
    ProjectionFieldMissing(ComponentFamily),
    /// Projections for one family do not preserve a stable id.
    ProjectionIdentityDrift(ComponentFamily),
    /// A reduced-capability consumer did not expose an explicit narrowed action.
    LimitedActionNarrowingMissing(ComponentFamily),
}

/// Loads and validates the checked-in component proof.
///
/// # Errors
///
/// Returns an error if the checked-in proof cannot be parsed or fails
/// validation.
pub fn current_m5_pipeline_dependency_finding_component_proof(
) -> Result<M5PipelineDependencyFindingComponentProof, ComponentProofArtifactError> {
    let proof: M5PipelineDependencyFindingComponentProof =
        serde_json::from_str(M5_PIPELINE_DEPENDENCY_FINDING_COMPONENT_PROOF_JSON)
            .map_err(ComponentProofArtifactError::Fixture)?;
    let violations = proof.validate();
    if violations.is_empty() {
        Ok(proof)
    } else {
        Err(ComponentProofArtifactError::Validation(violations))
    }
}

fn any_blank<'a>(values: impl IntoIterator<Item = &'a String>) -> bool {
    values.into_iter().any(|value| value.trim().is_empty())
}

fn has_all<'a>(actual: &[String], required: impl IntoIterator<Item = &'a str>) -> bool {
    let actual: BTreeSet<&str> = actual.iter().map(String::as_str).collect();
    required.into_iter().all(|value| actual.contains(value))
}
