//! Frozen template-registry, scaffold-planner, framework-pack, and support-class maturity matrix.
//!
//! This module locks the canonical depth qualification for the four template
//! and framework lanes that deepen switching and setup quality — a signed
//! template registry with verified provenance, a scaffold planner that previews
//! its write impact before any change, framework packs that keep
//! authored-versus-generated-versus-runtime-only truth and support class
//! explicit, and archetype health bundles that partition blockers from warnings
//! and optimizations — into one export-safe packet. Each
//! [`TemplateFrameworkLaneRow`] binds a lane to its qualification class, support
//! class, generation-truth class, evidence requirements, downgrade triggers,
//! rollback posture, source contracts, and consumer-surface parity.
//!
//! The matrix is the single source of truth for whether these lanes may ship as
//! Stable, Beta, Preview, or must narrow further. It references the upstream
//! template-registry, scaffold-run, framework-pack, and archetype-health
//! contracts by id rather than embedding their content. Template source bodies,
//! raw generator output, provider payloads, credentials, and secret values never
//! cross this boundary; the packet carries only metadata, qualification truth,
//! and contract references.
//!
//! [`TemplateFrameworkMatrixPacket::apply_downgrade_automation`] narrows lanes
//! whose proof is stale, whose evidence is invalid, or whose upstream dependency
//! narrowed, and blocks promotion automatically when a Stable lane loses its
//! evidence — so CI or release tooling can narrow a stale or underqualified lane
//! instead of shipping greener than the evidence.
//!
//! The boundary schema is
//! [`schemas/templates/freeze-the-m5-template-registry-framework-pack-and-support-class-matrix.schema.json`](../../../../schemas/templates/freeze-the-m5-template-registry-framework-pack-and-support-class-matrix.schema.json).
//! The contract doc is
//! [`docs/frameworks/m5/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix.md`](../../../../docs/frameworks/m5/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix.md).
//! The protected fixture directory is
//! [`fixtures/templates/m5/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix/`](../../../../fixtures/templates/m5/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`TemplateFrameworkMatrixPacket`].
pub const TEMPLATE_FRAMEWORK_MATRIX_RECORD_KIND: &str =
    "freeze_template_registry_framework_pack_and_support_class_matrix";

/// Schema version for template-registry, framework-pack, and support-class matrix records.
pub const TEMPLATE_FRAMEWORK_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const TEMPLATE_FRAMEWORK_MATRIX_SCHEMA_REF: &str =
    "schemas/templates/freeze-the-m5-template-registry-framework-pack-and-support-class-matrix.schema.json";

/// Repo-relative path of the matrix contract doc.
pub const TEMPLATE_FRAMEWORK_MATRIX_DOC_REF: &str =
    "docs/frameworks/m5/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix.md";

/// Repo-relative path of the signed template-registry entry contract.
pub const TEMPLATE_FRAMEWORK_MATRIX_REGISTRY_CONTRACT_REF: &str =
    "schemas/templates/template_registry_entry.schema.json";

/// Repo-relative path of the scaffold-run (planner) contract.
pub const TEMPLATE_FRAMEWORK_MATRIX_SCAFFOLD_CONTRACT_REF: &str =
    "schemas/templates/scaffold_run_alpha.schema.json";

/// Repo-relative path of the framework-pack descriptor contract.
pub const TEMPLATE_FRAMEWORK_MATRIX_FRAMEWORK_PACK_CONTRACT_REF: &str =
    "schemas/language/framework_pack_descriptor.schema.json";

/// Repo-relative path of the archetype-health-bundle contract.
pub const TEMPLATE_FRAMEWORK_MATRIX_ARCHETYPE_HEALTH_CONTRACT_REF: &str =
    "schemas/governance/seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const TEMPLATE_FRAMEWORK_MATRIX_FIXTURE_DIR: &str =
    "fixtures/templates/m5/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix";

/// Repo-relative path of the checked support-export artifact.
pub const TEMPLATE_FRAMEWORK_MATRIX_ARTIFACT_REF: &str =
    "artifacts/templates/m5/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const TEMPLATE_FRAMEWORK_MATRIX_SUMMARY_REF: &str =
    "artifacts/templates/m5/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix.md";

/// One of the four template and framework lanes governed by this matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateFrameworkLane {
    /// Signed template registry with verified publisher provenance and signatures.
    SignedTemplateRegistry,
    /// Scaffold planner with reviewable diff preview and visible rollback boundary before write.
    ScaffoldPlanner,
    /// Framework packs with explicit authored/generated/runtime-only truth and support class.
    FrameworkPack,
    /// Archetype health bundles partitioning blockers, warnings, and optimizations.
    ArchetypeHealthBundle,
}

impl TemplateFrameworkLane {
    /// Every lane, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SignedTemplateRegistry,
        Self::ScaffoldPlanner,
        Self::FrameworkPack,
        Self::ArchetypeHealthBundle,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedTemplateRegistry => "signed_template_registry",
            Self::ScaffoldPlanner => "scaffold_planner",
            Self::FrameworkPack => "framework_pack",
            Self::ArchetypeHealthBundle => "archetype_health_bundle",
        }
    }
}

/// Qualification class for a template or framework lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateFrameworkQualificationClass {
    /// Lane qualifies for the Stable claim.
    Stable,
    /// Lane is narrowed to Beta.
    Beta,
    /// Lane is narrowed to Preview.
    Preview,
    /// Lane is experimental and not claimed.
    Experimental,
    /// Lane is unavailable on this build.
    Unavailable,
    /// Lane is held pending upstream resolution.
    Held,
}

impl TemplateFrameworkQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the lane may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Support class communicated alongside every lane — the support-class matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateFrameworkSupportClass {
    /// First-party, officially supported by the IDE vendor.
    OfficiallySupported,
    /// Managed by the workspace's own team.
    TeamManaged,
    /// Community-maintained with best-effort support.
    CommunitySupported,
    /// Experimental; behavior and support may change without notice.
    Experimental,
    /// Explicitly unsupported.
    Unsupported,
    /// Narrowed below its normal support class by an active downgrade.
    NarrowedBelowStable,
}

impl TemplateFrameworkSupportClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfficiallySupported => "officially_supported",
            Self::TeamManaged => "team_managed",
            Self::CommunitySupported => "community_supported",
            Self::Experimental => "experimental",
            Self::Unsupported => "unsupported",
            Self::NarrowedBelowStable => "narrowed_below_stable",
        }
    }
}

/// Authored-versus-generated-versus-runtime-only truth for a lane's outputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateFrameworkGenerationTruth {
    /// Hand-authored content that the user owns outright.
    Authored,
    /// Generator-produced content carrying lineage for update or rebase.
    Generated,
    /// Behavior provided by the runtime/pack at run time, not written to disk.
    RuntimeOnly,
    /// A mix of authored and generated content, separated in diff preview.
    MixedAuthoredGenerated,
}

impl TemplateFrameworkGenerationTruth {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authored => "authored",
            Self::Generated => "generated",
            Self::RuntimeOnly => "runtime_only",
            Self::MixedAuthoredGenerated => "mixed_authored_generated",
        }
    }
}

/// Evidence requirement level for a lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateFrameworkEvidenceRequirement {
    /// At least one evidence packet is required.
    Required,
    /// Evidence is recommended but not blocking.
    Recommended,
    /// Evidence is optional.
    Optional,
    /// Not applicable for this lane's current qualification.
    NotApplicable,
}

impl TemplateFrameworkEvidenceRequirement {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Recommended => "recommended",
            Self::Optional => "optional",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Downgrade trigger that can narrow a lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateFrameworkDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// A template registry signature failed verification or expired.
    SignatureUnverified,
    /// The signed template revision is unavailable offline.
    TemplateRevisionUnavailable,
    /// A scaffold plan would write before review or export.
    ScaffoldPreviewUnavailable,
    /// A framework pack's support class narrowed below stable.
    SupportClassNarrowed,
    /// A framework pack presents heuristic or bridge behavior without current support-class cues.
    HeuristicPresentedAsExact,
    /// An archetype health bundle's checks went stale or unchecked.
    ArchetypeHealthStale,
    /// Generated-project lineage truth is missing, so update/rebase is unsafe.
    LineageTruthMissing,
    /// Scope expanded beyond the qualified template/framework boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl TemplateFrameworkDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::SignatureUnverified,
        Self::TemplateRevisionUnavailable,
        Self::ScaffoldPreviewUnavailable,
        Self::SupportClassNarrowed,
        Self::HeuristicPresentedAsExact,
        Self::ArchetypeHealthStale,
        Self::LineageTruthMissing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::SignatureUnverified => "signature_unverified",
            Self::TemplateRevisionUnavailable => "template_revision_unavailable",
            Self::ScaffoldPreviewUnavailable => "scaffold_preview_unavailable",
            Self::SupportClassNarrowed => "support_class_narrowed",
            Self::HeuristicPresentedAsExact => "heuristic_presented_as_exact",
            Self::ArchetypeHealthStale => "archetype_health_stale",
            Self::LineageTruthMissing => "lineage_truth_missing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Rollback posture for a lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateFrameworkRollbackPosture {
    /// Read-only lane that never mutates workspace, repository, or remote state.
    ReadOnlyNoMutation,
    /// Writes are gated behind a reviewable plan and a workspace checkpoint.
    PreviewedWriteWithCheckpoint,
    /// Generated outputs can be reverted by deleting them with no lingering scope.
    DeleteGeneratedOutputs,
    /// Update or rebase uses three-way lineage truth rather than blind regeneration.
    ThreeWayLineageUpdate,
    /// Evidence is preserved but no automatic revert exists.
    EvidencePreservedNoRevert,
    /// Not applicable for the lane's current qualification.
    NotApplicable,
}

impl TemplateFrameworkRollbackPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyNoMutation => "read_only_no_mutation",
            Self::PreviewedWriteWithCheckpoint => "previewed_write_with_checkpoint",
            Self::DeleteGeneratedOutputs => "delete_generated_outputs",
            Self::ThreeWayLineageUpdate => "three_way_lineage_update",
            Self::EvidencePreservedNoRevert => "evidence_preserved_no_revert",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Consumer surface that must project a lane's qualification truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateFrameworkConsumerSurface {
    /// Template/starter gallery.
    Gallery,
    /// Scaffold diff preview before write.
    DiffPreview,
    /// Scaffold run surface.
    RunSurface,
    /// Recovery / rollback surface.
    Recovery,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl TemplateFrameworkConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Gallery,
        Self::DiffPreview,
        Self::RunSurface,
        Self::Recovery,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gallery => "gallery",
            Self::DiffPreview => "diff_preview",
            Self::RunSurface => "run_surface",
            Self::Recovery => "recovery",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// One row in the template-registry, framework-pack, and support-class matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateFrameworkLaneRow {
    /// Template or framework lane.
    pub lane: TemplateFrameworkLane,
    /// Qualification class earned by this lane.
    pub qualification: TemplateFrameworkQualificationClass,
    /// Support class communicated for this lane.
    pub support_class: TemplateFrameworkSupportClass,
    /// Authored/generated/runtime-only truth for this lane's outputs.
    pub generation_truth: TemplateFrameworkGenerationTruth,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Evidence requirement level.
    pub evidence_requirement: TemplateFrameworkEvidenceRequirement,
    /// Required evidence packet refs for this qualification.
    pub required_evidence_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<TemplateFrameworkDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: TemplateFrameworkRollbackPosture,
    /// Source contract refs consumed by this lane.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this lane.
    pub consumer_surfaces: Vec<TemplateFrameworkConsumerSurface>,
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateFrameworkMatrixTrustReview {
    /// Template source provenance stays inspectable from gallery through recovery.
    pub template_source_provenance_inspectable: bool,
    /// Generator version and framework-pack version stay inspectable on every surface.
    pub generator_and_pack_versions_inspectable: bool,
    /// Signed template registry signatures are verified before a template is offered.
    pub signed_registry_signatures_verified: bool,
    /// Scaffold plans are previewable or exportable before any write.
    pub scaffold_diff_preview_before_write: bool,
    /// Rollback boundary is visible before generation starts.
    pub rollback_boundary_visible: bool,
    /// Authored, generated, and runtime-only content stay explicitly separated.
    pub authored_generated_runtime_truth_explicit: bool,
    /// Support class and downgrade cues stay current on every framework pack.
    pub support_class_and_downgrade_cues_current: bool,
    /// Heuristic or bridge behavior is never presented as exact first-party truth.
    pub heuristic_never_presented_as_exact: bool,
    /// Archetype health bundles partition blockers, warnings, and optimizations.
    pub archetype_health_partitioned: bool,
    /// No credential bodies or raw provider payloads cross the export boundary.
    pub no_credential_bodies_in_export: bool,
    /// Downgrade narrows the claim rather than hiding the lane.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified lanes automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateFrameworkMatrixConsumerProjection {
    /// Gallery shows template source class and support class.
    pub gallery_shows_source_and_support_class: bool,
    /// Diff preview shows authored-versus-generated truth before write.
    pub diff_preview_shows_authored_generated_truth: bool,
    /// Run surface shows generator version and framework-pack version.
    pub run_surface_shows_pack_and_generator_versions: bool,
    /// Recovery surface shows rollback boundary and generated-project lineage.
    pub recovery_shows_rollback_and_lineage: bool,
    /// CLI / headless shows qualification truth.
    pub cli_headless_shows_qualification: bool,
    /// Support export shows qualification truth.
    pub support_export_shows_qualification: bool,
    /// Diagnostics shows qualification truth.
    pub diagnostics_shows_qualification: bool,
    /// Help / About shows qualification truth.
    pub help_about_shows_qualification: bool,
    /// Preview / Labs lanes are visibly labeled when not covered by this packet.
    pub preview_labs_label_for_unqualified_lanes: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateFrameworkMatrixProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Per-lane observation fed to [`TemplateFrameworkMatrixPacket::apply_downgrade_automation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateFrameworkLaneObservation {
    /// Lane the observation applies to.
    pub lane: TemplateFrameworkLane,
    /// True when the lane's checked-in evidence currently validates.
    pub evidence_valid: bool,
    /// True when the lane's proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when an upstream dependency of the lane narrowed.
    pub upstream_narrowed: bool,
}

/// Constructor input for [`TemplateFrameworkMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateFrameworkMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Lane rows.
    pub lane_rows: Vec<TemplateFrameworkLaneRow>,
    /// Trust review block.
    pub trust_review: TemplateFrameworkMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: TemplateFrameworkMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: TemplateFrameworkMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen template-registry, framework-pack, and support-class matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateFrameworkMatrixPacket {
    /// Record kind; must equal [`TEMPLATE_FRAMEWORK_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`TEMPLATE_FRAMEWORK_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Lane rows.
    pub lane_rows: Vec<TemplateFrameworkLaneRow>,
    /// Trust review block.
    pub trust_review: TemplateFrameworkMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: TemplateFrameworkMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: TemplateFrameworkMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl TemplateFrameworkMatrixPacket {
    /// Builds a template-registry, framework-pack, and support-class matrix packet from stable-lane input.
    pub fn new(input: TemplateFrameworkMatrixPacketInput) -> Self {
        Self {
            record_kind: TEMPLATE_FRAMEWORK_MATRIX_RECORD_KIND.to_owned(),
            schema_version: TEMPLATE_FRAMEWORK_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            lane_rows: input.lane_rows,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Narrows lanes whose evidence is invalid, whose proof is stale, or whose
    /// upstream dependency narrowed.
    ///
    /// Invalid evidence blocks the lane (qualification `held`, support class
    /// `narrowed_below_stable`); stale proof or a narrowed upstream narrows a
    /// Stable lane to Beta and its support class to `narrowed_below_stable`.
    /// Observations for lanes not present in the packet are ignored; lanes
    /// without an observation are left unchanged.
    pub fn apply_downgrade_automation(
        &mut self,
        observations: &[TemplateFrameworkLaneObservation],
    ) {
        for row in &mut self.lane_rows {
            let Some(observation) = observations.iter().find(|obs| obs.lane == row.lane) else {
                continue;
            };
            if !observation.evidence_valid {
                row.qualification = TemplateFrameworkQualificationClass::Held;
                row.support_class = TemplateFrameworkSupportClass::NarrowedBelowStable;
                row.evidence_requirement = TemplateFrameworkEvidenceRequirement::NotApplicable;
                row.required_evidence_packet_refs.clear();
                row.rollback_posture = TemplateFrameworkRollbackPosture::NotApplicable;
            } else if (!observation.proof_fresh || observation.upstream_narrowed)
                && row.qualification == TemplateFrameworkQualificationClass::Stable
            {
                row.qualification = TemplateFrameworkQualificationClass::Beta;
                row.support_class = TemplateFrameworkSupportClass::NarrowedBelowStable;
            }
        }
    }

    /// Validates the template-registry, framework-pack, and support-class matrix invariants.
    pub fn validate(&self) -> Vec<TemplateFrameworkMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != TEMPLATE_FRAMEWORK_MATRIX_RECORD_KIND {
            violations.push(TemplateFrameworkMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != TEMPLATE_FRAMEWORK_MATRIX_SCHEMA_VERSION {
            violations.push(TemplateFrameworkMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(TemplateFrameworkMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_lane_rows(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("template/framework matrix packet serializes"),
        ) {
            violations.push(TemplateFrameworkMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("template/framework matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, gallery, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_lanes = self
            .lane_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# Template Registry, Framework-Pack, and Support-Class Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Lanes: {} ({} stable)\n",
            self.lane_rows.len(),
            stable_lanes
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Lanes\n\n");
        for row in &self.lane_rows {
            out.push_str(&format!(
                "- **{}**: `{}` ({})\n",
                row.lane.as_str(),
                row.qualification.as_str(),
                row.support_class.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Generation truth: {}\n",
                row.generation_truth.as_str()
            ));
            out.push_str(&format!(
                "  - Evidence: {} ({} refs)\n",
                row.evidence_requirement.as_str(),
                row.required_evidence_packet_refs.len()
            ));
            out.push_str(&format!(
                "  - Rollback: {}\n",
                row.rollback_posture.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in template/framework matrix export.
#[derive(Debug)]
pub enum TemplateFrameworkMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<TemplateFrameworkMatrixViolation>),
}

impl fmt::Display for TemplateFrameworkMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "template/framework matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "template/framework matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for TemplateFrameworkMatrixArtifactError {}

/// Validation failures emitted by [`TemplateFrameworkMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemplateFrameworkMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required lane is missing from the matrix.
    RequiredLaneMissing,
    /// A lane row is incomplete.
    LaneRowIncomplete,
    /// A lane claiming Stable is missing required evidence packet refs.
    StableLaneMissingEvidence,
    /// A lane has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A lane has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl TemplateFrameworkMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredLaneMissing => "required_lane_missing",
            Self::LaneRowIncomplete => "lane_row_incomplete",
            Self::StableLaneMissingEvidence => "stable_lane_missing_evidence",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable template/framework matrix export.
///
/// This is the first real consumer of the frozen matrix: a gallery, diagnostics,
/// Help/About, or support-export surface calls it to ingest the canonical packet
/// rather than cloning status text.
///
/// # Errors
///
/// Returns [`TemplateFrameworkMatrixArtifactError`] when the checked-in support
/// export fails to parse or fails validation.
pub fn current_stable_template_framework_matrix_export(
) -> Result<TemplateFrameworkMatrixPacket, TemplateFrameworkMatrixArtifactError> {
    let packet: TemplateFrameworkMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/templates/m5/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix/support_export.json"
    )))
    .map_err(TemplateFrameworkMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(TemplateFrameworkMatrixArtifactError::Validation(violations))
    }
}

/// Canonical trust review block with every invariant satisfied.
pub fn canonical_trust_review() -> TemplateFrameworkMatrixTrustReview {
    TemplateFrameworkMatrixTrustReview {
        template_source_provenance_inspectable: true,
        generator_and_pack_versions_inspectable: true,
        signed_registry_signatures_verified: true,
        scaffold_diff_preview_before_write: true,
        rollback_boundary_visible: true,
        authored_generated_runtime_truth_explicit: true,
        support_class_and_downgrade_cues_current: true,
        heuristic_never_presented_as_exact: true,
        archetype_health_partitioned: true,
        no_credential_bodies_in_export: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

/// Canonical consumer projection block with every surface projecting qualification truth.
pub fn canonical_consumer_projection() -> TemplateFrameworkMatrixConsumerProjection {
    TemplateFrameworkMatrixConsumerProjection {
        gallery_shows_source_and_support_class: true,
        diff_preview_shows_authored_generated_truth: true,
        run_surface_shows_pack_and_generator_versions: true,
        recovery_shows_rollback_and_lineage: true,
        cli_headless_shows_qualification: true,
        support_export_shows_qualification: true,
        diagnostics_shows_qualification: true,
        help_about_shows_qualification: true,
        preview_labs_label_for_unqualified_lanes: true,
    }
}

/// Canonical source contract refs that every matrix export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        TEMPLATE_FRAMEWORK_MATRIX_SCHEMA_REF.to_owned(),
        TEMPLATE_FRAMEWORK_MATRIX_DOC_REF.to_owned(),
        TEMPLATE_FRAMEWORK_MATRIX_REGISTRY_CONTRACT_REF.to_owned(),
        TEMPLATE_FRAMEWORK_MATRIX_SCAFFOLD_CONTRACT_REF.to_owned(),
        TEMPLATE_FRAMEWORK_MATRIX_FRAMEWORK_PACK_CONTRACT_REF.to_owned(),
        TEMPLATE_FRAMEWORK_MATRIX_ARCHETYPE_HEALTH_CONTRACT_REF.to_owned(),
    ]
}

/// Builds the canonical frozen template/framework matrix from stable-lane truth.
///
/// The four lanes mirror the checked-in support export: a Stable signed template
/// registry, a Stable scaffold planner, a Beta framework-pack lane (community
/// support, support-class and downgrade cues required), and a Stable archetype
/// health-bundle lane.
pub fn frozen_template_framework_matrix(
    packet_id: String,
    matrix_label: String,
    minted_at: String,
    proof_freshness: TemplateFrameworkMatrixProofFreshness,
) -> TemplateFrameworkMatrixPacket {
    TemplateFrameworkMatrixPacket::new(TemplateFrameworkMatrixPacketInput {
        packet_id,
        matrix_label,
        lane_rows: canonical_lane_rows(),
        trust_review: canonical_trust_review(),
        consumer_projection: canonical_consumer_projection(),
        proof_freshness,
        source_contract_refs: canonical_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at,
    })
}

/// Canonical lane rows that match the checked-in support export.
pub fn canonical_lane_rows() -> Vec<TemplateFrameworkLaneRow> {
    use TemplateFrameworkConsumerSurface as Surface;
    use TemplateFrameworkDowngradeTrigger as Trigger;

    vec![
        TemplateFrameworkLaneRow {
            lane: TemplateFrameworkLane::SignedTemplateRegistry,
            qualification: TemplateFrameworkQualificationClass::Stable,
            support_class: TemplateFrameworkSupportClass::OfficiallySupported,
            generation_truth: TemplateFrameworkGenerationTruth::Authored,
            scope_summary: "Signed template registry whose publisher provenance and signatures are verified before a template is offered; template source class and generator version stay inspectable from gallery through recovery".to_owned(),
            evidence_requirement: TemplateFrameworkEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:template-registry-signature-verification:m5".to_owned(),
                "evidence:template-source-provenance:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::SignatureUnverified,
                Trigger::TemplateRevisionUnavailable,
                Trigger::PolicyBlocked,
            ],
            rollback_posture: TemplateFrameworkRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                TEMPLATE_FRAMEWORK_MATRIX_REGISTRY_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                Surface::Gallery,
                Surface::CliHeadless,
                Surface::SupportExport,
                Surface::Diagnostics,
            ],
        },
        TemplateFrameworkLaneRow {
            lane: TemplateFrameworkLane::ScaffoldPlanner,
            qualification: TemplateFrameworkQualificationClass::Stable,
            support_class: TemplateFrameworkSupportClass::OfficiallySupported,
            generation_truth: TemplateFrameworkGenerationTruth::MixedAuthoredGenerated,
            scope_summary: "Scaffold planner whose file and directory impact is reviewable or exportable before any write, with a visible rollback boundary and a create-empty alternative offered at equal weight".to_owned(),
            evidence_requirement: TemplateFrameworkEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:scaffold-plan-preview-before-write:m5".to_owned(),
                "evidence:scaffold-rollback-boundary:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ScaffoldPreviewUnavailable,
                Trigger::LineageTruthMissing,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: TemplateFrameworkRollbackPosture::PreviewedWriteWithCheckpoint,
            source_contract_refs: vec![
                TEMPLATE_FRAMEWORK_MATRIX_SCAFFOLD_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                Surface::DiffPreview,
                Surface::RunSurface,
                Surface::Recovery,
                Surface::SupportExport,
            ],
        },
        TemplateFrameworkLaneRow {
            lane: TemplateFrameworkLane::FrameworkPack,
            qualification: TemplateFrameworkQualificationClass::Beta,
            support_class: TemplateFrameworkSupportClass::CommunitySupported,
            generation_truth: TemplateFrameworkGenerationTruth::RuntimeOnly,
            scope_summary: "Framework packs that keep authored, generated, and runtime-only behavior explicitly separated and never present heuristic or bridge behavior as exact first-party truth without current support-class and downgrade cues".to_owned(),
            evidence_requirement: TemplateFrameworkEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:framework-pack-support-class-cues:m5".to_owned(),
                "evidence:framework-pack-generation-truth:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::SupportClassNarrowed,
                Trigger::HeuristicPresentedAsExact,
                Trigger::ScopeExpansionUnqualified,
            ],
            rollback_posture: TemplateFrameworkRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                TEMPLATE_FRAMEWORK_MATRIX_FRAMEWORK_PACK_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                Surface::Gallery,
                Surface::RunSurface,
                Surface::SupportExport,
                Surface::HelpAbout,
            ],
        },
        TemplateFrameworkLaneRow {
            lane: TemplateFrameworkLane::ArchetypeHealthBundle,
            qualification: TemplateFrameworkQualificationClass::Stable,
            support_class: TemplateFrameworkSupportClass::OfficiallySupported,
            generation_truth: TemplateFrameworkGenerationTruth::Generated,
            scope_summary: "Archetype health bundles that partition blockers, warnings, and optimizations and preserve live, cached, policy-evaluated, and unchecked freshness state instead of collapsing to one pass/fail bit".to_owned(),
            evidence_requirement: TemplateFrameworkEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:archetype-health-partition:m5".to_owned(),
                "evidence:archetype-health-freshness:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ArchetypeHealthStale,
                Trigger::PolicyBlocked,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: TemplateFrameworkRollbackPosture::ThreeWayLineageUpdate,
            source_contract_refs: vec![
                TEMPLATE_FRAMEWORK_MATRIX_ARCHETYPE_HEALTH_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                Surface::RunSurface,
                Surface::Recovery,
                Surface::SupportExport,
                Surface::Diagnostics,
            ],
        },
    ]
}

fn validate_source_contracts(
    packet: &TemplateFrameworkMatrixPacket,
    violations: &mut Vec<TemplateFrameworkMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        TEMPLATE_FRAMEWORK_MATRIX_SCHEMA_REF,
        TEMPLATE_FRAMEWORK_MATRIX_DOC_REF,
        TEMPLATE_FRAMEWORK_MATRIX_REGISTRY_CONTRACT_REF,
        TEMPLATE_FRAMEWORK_MATRIX_SCAFFOLD_CONTRACT_REF,
        TEMPLATE_FRAMEWORK_MATRIX_FRAMEWORK_PACK_CONTRACT_REF,
        TEMPLATE_FRAMEWORK_MATRIX_ARCHETYPE_HEALTH_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(TemplateFrameworkMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_lane_rows(
    packet: &TemplateFrameworkMatrixPacket,
    violations: &mut Vec<TemplateFrameworkMatrixViolation>,
) {
    let present: BTreeSet<TemplateFrameworkLane> =
        packet.lane_rows.iter().map(|row| row.lane).collect();
    for required in TemplateFrameworkLane::ALL {
        if !present.contains(&required) {
            violations.push(TemplateFrameworkMatrixViolation::RequiredLaneMissing);
            return;
        }
    }

    for row in &packet.lane_rows {
        if row.scope_summary.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(TemplateFrameworkMatrixViolation::LaneRowIncomplete);
        }
        if row.qualification.is_stable() && row.required_evidence_packet_refs.is_empty() {
            violations.push(TemplateFrameworkMatrixViolation::StableLaneMissingEvidence);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(TemplateFrameworkMatrixViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(TemplateFrameworkMatrixViolation::ConsumerSurfacesMissing);
        }
    }
}

fn validate_trust_review(
    packet: &TemplateFrameworkMatrixPacket,
    violations: &mut Vec<TemplateFrameworkMatrixViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.template_source_provenance_inspectable,
        review.generator_and_pack_versions_inspectable,
        review.signed_registry_signatures_verified,
        review.scaffold_diff_preview_before_write,
        review.rollback_boundary_visible,
        review.authored_generated_runtime_truth_explicit,
        review.support_class_and_downgrade_cues_current,
        review.heuristic_never_presented_as_exact,
        review.archetype_health_partitioned,
        review.no_credential_bodies_in_export,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(TemplateFrameworkMatrixViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &TemplateFrameworkMatrixPacket,
    violations: &mut Vec<TemplateFrameworkMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.gallery_shows_source_and_support_class,
        projection.diff_preview_shows_authored_generated_truth,
        projection.run_surface_shows_pack_and_generator_versions,
        projection.recovery_shows_rollback_and_lineage,
        projection.cli_headless_shows_qualification,
        projection.support_export_shows_qualification,
        projection.diagnostics_shows_qualification,
        projection.help_about_shows_qualification,
        projection.preview_labs_label_for_unqualified_lanes,
    ] {
        if !ok {
            violations.push(TemplateFrameworkMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &TemplateFrameworkMatrixPacket,
    violations: &mut Vec<TemplateFrameworkMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(TemplateFrameworkMatrixViolation::ProofFreshnessIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
