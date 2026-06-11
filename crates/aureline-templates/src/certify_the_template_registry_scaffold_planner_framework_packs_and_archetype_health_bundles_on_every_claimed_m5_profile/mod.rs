//! Certification of the template registry, scaffold planner, framework packs,
//! and archetype health bundles across every claimed M5 profile.
//!
//! This module is the canonical certification layer over the M5 template,
//! scaffold, framework-pack, and archetype-health depth lanes. Where the frozen
//! maturity matrix locks four lanes at lane granularity, this packet certifies
//! every claimed M5 *profile* that feeds those lanes — the signed template
//! registry, the generation diff-review and managed-zone recovery flow, framework
//! generator/codemod runs, framework-pack headers, the richer framework-pack lane
//! catalog, route/component/topology views, convention diagnostics, and certified
//! archetype health bundles — binding each profile to a certification verdict, the
//! evidence packet refs that back the claim, the downgrade triggers that can narrow
//! it, a rollback posture, and a per-profile proof-freshness observation.
//!
//! Each [`M5TemplateCertifiedProfile`] references its upstream packet by record
//! kind, support-export artifact, schema, and contract doc rather than embedding
//! the packet body. A [`M5TemplateCertificationCompatibilityReport`] aggregates the
//! per-profile verdicts into a single promotion verdict, and
//! [`M5TemplateCertificationPacket::apply_downgrade_automation`] narrows profiles
//! whose proof is stale, whose evidence packet failed validation, or whose upstream
//! dependency narrowed — so CI or release tooling can fail promotion or narrow the
//! claim automatically instead of shipping greener than the evidence.
//!
//! [`certify_from_current_exports`] is the first real consumer: it reads each
//! claimed profile's checked-in support export through its own producer, validates
//! it, and certifies the profile only when its evidence currently validates. Raw
//! template source bodies, raw generator output, raw provider payloads,
//! credentials, repository URLs, and secret values stay outside this boundary.
//!
//! The boundary schema is
//! [`schemas/templates/certify-the-template-registry-scaffold-planner-framework-packs-and-archetype-health-bundles-on-every-claimed-m5-profile.schema.json`](../../../../schemas/templates/certify-the-template-registry-scaffold-planner-framework-packs-and-archetype-health-bundles-on-every-claimed-m5-profile.schema.json).
//! The contract doc is
//! [`docs/frameworks/m5/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile.md`](../../../../docs/frameworks/m5/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile.md).
//! The protected fixture directory is
//! [`fixtures/templates/m5/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile/`](../../../../fixtures/templates/m5/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5TemplateCertificationPacket`].
pub const M5_TEMPLATE_CERTIFICATION_RECORD_KIND: &str =
    "certify_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile";

/// Schema version for M5 template, scaffold, framework-pack, and archetype-health certification records.
pub const M5_TEMPLATE_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_TEMPLATE_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/templates/certify-the-template-registry-scaffold-planner-framework-packs-and-archetype-health-bundles-on-every-claimed-m5-profile.schema.json";

/// Repo-relative path of the M5 template certification contract doc.
pub const M5_TEMPLATE_CERTIFICATION_DOC_REF: &str =
    "docs/frameworks/m5/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile.md";

/// Repo-relative path of the frozen maturity-matrix schema authority this certification builds on.
pub const M5_TEMPLATE_CERTIFICATION_MATRIX_SCHEMA_REF: &str =
    "schemas/templates/freeze-the-m5-template-registry-framework-pack-and-support-class-matrix.schema.json";

/// Repo-relative path of the frozen maturity-matrix contract doc this certification builds on.
pub const M5_TEMPLATE_CERTIFICATION_MATRIX_DOC_REF: &str =
    "docs/frameworks/m5/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_TEMPLATE_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/templates/m5/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile";

/// Repo-relative path of the checked support-export artifact.
pub const M5_TEMPLATE_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/templates/m5/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_TEMPLATE_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/templates/m5/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile.md";

/// One claimed M5 template, scaffold, framework-pack, or archetype-health profile certified by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TemplateProfile {
    /// Signed template registry, provenance/mirror support, and template-health rows.
    SignedTemplateRegistry,
    /// Generation diff-review, rollback or delete-generated recovery, and managed-zone honesty.
    GenerationRecovery,
    /// Framework generator and codemod runs with preview, diff, rollback, and execution-context reuse.
    FrameworkGeneratorRun,
    /// Framework-pack headers, pack-version/freshness chips, and capability/downgrade banners.
    FrameworkPackHeader,
    /// Richer framework-pack lane catalog across the notebook, infra, web-API, and mobile lanes.
    RicherFrameworkPacks,
    /// Route-explorer, component-tree, and app-topology views with authored/generated/runtime-only truth.
    AppTopologyViews,
    /// Convention diagnostics with confidence labels, suppressibility, and proving-file disclosure.
    ConventionDiagnostics,
    /// Certified archetype health-check bundles, stack diagnostics, and fix-forward guidance.
    ArchetypeHealthBundle,
}

impl M5TemplateProfile {
    /// Every claimed profile, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SignedTemplateRegistry,
        Self::GenerationRecovery,
        Self::FrameworkGeneratorRun,
        Self::FrameworkPackHeader,
        Self::RicherFrameworkPacks,
        Self::AppTopologyViews,
        Self::ConventionDiagnostics,
        Self::ArchetypeHealthBundle,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedTemplateRegistry => "signed_template_registry",
            Self::GenerationRecovery => "generation_recovery",
            Self::FrameworkGeneratorRun => "framework_generator_run",
            Self::FrameworkPackHeader => "framework_pack_header",
            Self::RicherFrameworkPacks => "richer_framework_packs",
            Self::AppTopologyViews => "app_topology_views",
            Self::ConventionDiagnostics => "convention_diagnostics",
            Self::ArchetypeHealthBundle => "archetype_health_bundle",
        }
    }
}

/// Template, scaffold, framework-pack, or archetype-health lane a certified profile belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TemplateCertificationLane {
    /// Signed template-registry lane.
    TemplateRegistry,
    /// Scaffold-planner and generation lane.
    ScaffoldPlanner,
    /// Framework-pack lane.
    FrameworkPack,
    /// Archetype-health lane.
    ArchetypeHealth,
    /// Cross-cutting control row (e.g. the maturity matrix itself).
    CrossCutting,
}

impl M5TemplateCertificationLane {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TemplateRegistry => "template_registry",
            Self::ScaffoldPlanner => "scaffold_planner",
            Self::FrameworkPack => "framework_pack",
            Self::ArchetypeHealth => "archetype_health",
            Self::CrossCutting => "cross_cutting",
        }
    }
}

/// Qualification class claimed by a profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TemplateCertificationQualificationClass {
    /// Profile claims the Stable maturity.
    Stable,
    /// Profile claims the Beta maturity.
    Beta,
    /// Profile claims the Preview maturity.
    Preview,
    /// Profile is experimental and not claimed.
    Experimental,
    /// Profile is unavailable on this build.
    Unavailable,
    /// Profile is held pending upstream resolution.
    Held,
}

impl M5TemplateCertificationQualificationClass {
    /// Stable token recorded in the certification.
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
}

/// Certification verdict earned by a profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TemplateCertificationVerdict {
    /// Profile is certified at its claimed qualification with current, valid evidence.
    Certified,
    /// Profile is certified, but narrowed below its claimed qualification.
    NarrowedCertified,
    /// Profile is blocked from promotion until its evidence or dependency recovers.
    Blocked,
    /// Profile could not be certified (missing or invalid evidence).
    NotCertified,
}

impl M5TemplateCertificationVerdict {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::NarrowedCertified => "narrowed_certified",
            Self::Blocked => "blocked",
            Self::NotCertified => "not_certified",
        }
    }

    /// Whether the verdict still permits a public claim (possibly narrowed).
    pub const fn is_publishable(self) -> bool {
        matches!(self, Self::Certified | Self::NarrowedCertified)
    }
}

/// Downgrade trigger that can narrow a certified profile.
///
/// The vocabulary mirrors the frozen template/framework maturity matrix this
/// certification builds on, plus `evidence_packet_invalid` for the certification
/// mechanic itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TemplateCertificationDowngradeTrigger {
    /// Proof packet has gone stale relative to its freshness SLO.
    ProofStale,
    /// Evidence packet failed validation or is missing.
    EvidencePacketInvalid,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// A registry signature could not be verified.
    SignatureUnverified,
    /// A pinned template revision is unavailable on this build or mirror.
    TemplateRevisionUnavailable,
    /// The scaffold planner's diff preview is unavailable before a write.
    ScaffoldPreviewUnavailable,
    /// The support class narrowed below its claimed class.
    SupportClassNarrowed,
    /// Heuristic or bridge behavior risks being presented as exact first-party truth.
    HeuristicPresentedAsExact,
    /// Archetype-health proof has gone stale.
    ArchetypeHealthStale,
    /// Authored/generated/runtime-only lineage truth is missing.
    LineageTruthMissing,
    /// Scope expanded beyond the qualified template/scaffold/framework boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency profile narrowed.
    UpstreamDependencyNarrowed,
}

impl M5TemplateCertificationDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ProofStale,
        Self::EvidencePacketInvalid,
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

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::EvidencePacketInvalid => "evidence_packet_invalid",
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

/// Rollback posture for a certified profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TemplateCertificationRollbackPosture {
    /// Read-only profile that never mutates workspace, repository, or registry state.
    ReadOnlyNoMutation,
    /// Scaffold planner previews its file and directory impact before any write.
    ScaffoldPreviewBeforeWrite,
    /// Generated work can be rolled back or deleted-generated without touching authored source.
    GenerationRollbackOrDeleteGenerated,
    /// Generated content lives in a managed zone the producer can regenerate.
    ManagedZoneRegenerate,
    /// Evidence is preserved but no automatic revert exists.
    EvidencePreservedNoRevert,
    /// Not applicable for the profile's current verdict.
    NotApplicable,
}

impl M5TemplateCertificationRollbackPosture {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyNoMutation => "read_only_no_mutation",
            Self::ScaffoldPreviewBeforeWrite => "scaffold_preview_before_write",
            Self::GenerationRollbackOrDeleteGenerated => "generation_rollback_or_delete_generated",
            Self::ManagedZoneRegenerate => "managed_zone_regenerate",
            Self::EvidencePreservedNoRevert => "evidence_preserved_no_revert",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Per-profile proof-freshness observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TemplateCertificationProfileFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the profile's last proof refresh.
    pub last_proof_refresh: String,
    /// True when the profile's proof is currently within its freshness SLO.
    pub proof_fresh: bool,
}

/// One certified claimed M5 profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TemplateCertifiedProfile {
    /// Claimed M5 profile.
    pub profile: M5TemplateProfile,
    /// Lane the profile belongs to.
    pub lane: M5TemplateCertificationLane,
    /// Qualification class claimed by the profile.
    pub claimed_qualification: M5TemplateCertificationQualificationClass,
    /// Certification verdict.
    pub verdict: M5TemplateCertificationVerdict,
    /// Upstream packet record kind backing the profile.
    pub upstream_record_kind: String,
    /// Support-export artifact ref backing the profile.
    pub evidence_artifact_ref: String,
    /// Schema ref backing the profile.
    pub evidence_schema_ref: String,
    /// Contract doc ref backing the profile.
    pub evidence_doc_ref: String,
    /// Downgrade triggers that can narrow the profile.
    pub downgrade_triggers: Vec<M5TemplateCertificationDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5TemplateCertificationRollbackPosture,
    /// Per-profile proof freshness.
    pub proof_freshness: M5TemplateCertificationProfileFreshness,
}

/// Aggregate compatibility report across all certified profiles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TemplateCertificationCompatibilityReport {
    /// Total certified profiles in the packet.
    pub total_profiles: u32,
    /// Count of fully certified profiles.
    pub certified_count: u32,
    /// Count of narrowed-but-certified profiles.
    pub narrowed_count: u32,
    /// Count of blocked profiles.
    pub blocked_count: u32,
    /// Count of profiles that could not be certified.
    pub not_certified_count: u32,
    /// True when every profile is publishable (certified or narrowed).
    pub all_profiles_publishable: bool,
    /// Human-readable promotion note.
    pub promotion_note: String,
}

impl M5TemplateCertificationCompatibilityReport {
    /// Recomputes the compatibility report from a profile set.
    pub fn from_profiles(profiles: &[M5TemplateCertifiedProfile]) -> Self {
        let mut certified = 0u32;
        let mut narrowed = 0u32;
        let mut blocked = 0u32;
        let mut not_certified = 0u32;
        for profile in profiles {
            match profile.verdict {
                M5TemplateCertificationVerdict::Certified => certified += 1,
                M5TemplateCertificationVerdict::NarrowedCertified => narrowed += 1,
                M5TemplateCertificationVerdict::Blocked => blocked += 1,
                M5TemplateCertificationVerdict::NotCertified => not_certified += 1,
            }
        }
        let all_publishable = blocked == 0 && not_certified == 0;
        let promotion_note = if all_publishable {
            "all claimed M5 template, scaffold, framework-pack, and archetype-health profiles are publishable"
                .to_owned()
        } else {
            format!(
                "{} profile(s) blocked and {} profile(s) uncertified; promotion narrows",
                blocked, not_certified
            )
        };
        Self {
            total_profiles: profiles.len() as u32,
            certified_count: certified,
            narrowed_count: narrowed,
            blocked_count: blocked,
            not_certified_count: not_certified,
            all_profiles_publishable: all_publishable,
            promotion_note,
        }
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TemplateCertificationTrustReview {
    /// Template source provenance and mirror lineage stay inspectable before a template is offered.
    pub template_source_provenance_inspectable: bool,
    /// Generator and pack versions stay inspectable from gallery through diff, run, and recovery.
    pub generator_and_pack_versions_inspectable: bool,
    /// Signed registry signatures are verified before a template is offered.
    pub signed_registry_signatures_verified: bool,
    /// The scaffold planner previews its file and directory impact before any write.
    pub scaffold_diff_preview_before_write: bool,
    /// The rollback boundary is visible before any generated write.
    pub rollback_boundary_visible: bool,
    /// Authored, generated, and runtime-only truth stay explicit and separated.
    pub authored_generated_runtime_truth_explicit: bool,
    /// Support-class and downgrade cues stay current.
    pub support_class_and_downgrade_cues_current: bool,
    /// Framework packs never present heuristic or bridge behavior as exact first-party truth.
    pub heuristic_never_presented_as_exact: bool,
    /// Archetype health partitions blockers from warnings and optimizations.
    pub archetype_health_partitioned: bool,
    /// No credential bodies or raw provider payloads cross the export boundary.
    pub no_credential_bodies_in_export: bool,
    /// Downgrade narrows the claim rather than hiding the profile.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified profiles automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TemplateCertificationConsumerProjection {
    /// Template / starter gallery shows certification verdict and provenance.
    pub gallery_shows_certification: bool,
    /// Framework-pack header shows certification verdict and freshness.
    pub pack_header_shows_certification: bool,
    /// Scaffold run surface shows certification verdict and preview/rollback state.
    pub scaffold_run_shows_certification: bool,
    /// Generation diff-review surface shows certification verdict.
    pub diff_review_shows_certification: bool,
    /// CLI / headless shows certification truth.
    pub cli_headless_shows_certification: bool,
    /// Support export shows certification truth.
    pub support_export_shows_certification: bool,
    /// Diagnostics shows certification truth.
    pub diagnostics_shows_certification: bool,
    /// Help / About shows certification truth.
    pub help_about_shows_certification: bool,
    /// Preview / Labs profiles are visibly labeled when not certified Stable.
    pub preview_labs_label_for_unqualified_profiles: bool,
}

/// Packet-level proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TemplateCertificationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last certification refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the certification.
    pub auto_narrow_on_stale: bool,
}

/// Per-profile observation fed to [`M5TemplateCertificationPacket::apply_downgrade_automation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TemplateCertificationProfileObservation {
    /// Profile the observation applies to.
    pub profile: M5TemplateProfile,
    /// True when the profile's checked-in evidence currently validates.
    pub evidence_valid: bool,
    /// True when the profile's proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when an upstream dependency of the profile narrowed.
    pub upstream_narrowed: bool,
}

/// Constructor input for [`M5TemplateCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TemplateCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified profiles.
    pub certified_profiles: Vec<M5TemplateCertifiedProfile>,
    /// Compatibility report.
    pub compatibility_report: M5TemplateCertificationCompatibilityReport,
    /// Trust review block.
    pub trust_review: M5TemplateCertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5TemplateCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TemplateCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 template, scaffold, framework-pack, and archetype-health certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TemplateCertificationPacket {
    /// Record kind; must equal [`M5_TEMPLATE_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_TEMPLATE_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified profiles.
    pub certified_profiles: Vec<M5TemplateCertifiedProfile>,
    /// Compatibility report.
    pub compatibility_report: M5TemplateCertificationCompatibilityReport,
    /// Trust review block.
    pub trust_review: M5TemplateCertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5TemplateCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TemplateCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5TemplateCertificationPacket {
    /// Builds an M5 template certification packet from stable-lane input.
    pub fn new(input: M5TemplateCertificationPacketInput) -> Self {
        Self {
            record_kind: M5_TEMPLATE_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_TEMPLATE_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            certification_label: input.certification_label,
            certified_profiles: input.certified_profiles,
            compatibility_report: input.compatibility_report,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Narrows profiles whose evidence is invalid, whose proof is stale, or whose
    /// upstream dependency narrowed, then recomputes the compatibility report.
    ///
    /// This is the downgrade automation: invalid evidence blocks the profile,
    /// stale proof or a narrowed upstream narrows it, and the per-profile
    /// freshness flag is updated. Observations for profiles not present in the
    /// packet are ignored; profiles without an observation are left unchanged.
    pub fn apply_downgrade_automation(
        &mut self,
        observations: &[M5TemplateCertificationProfileObservation],
    ) {
        for profile in &mut self.certified_profiles {
            let Some(observation) = observations
                .iter()
                .find(|obs| obs.profile == profile.profile)
            else {
                continue;
            };
            profile.proof_freshness.proof_fresh = observation.proof_fresh;
            if !observation.evidence_valid {
                profile.verdict = M5TemplateCertificationVerdict::Blocked;
            } else if (!observation.proof_fresh || observation.upstream_narrowed)
                && profile.verdict == M5TemplateCertificationVerdict::Certified
            {
                profile.verdict = M5TemplateCertificationVerdict::NarrowedCertified;
            }
        }
        self.compatibility_report =
            M5TemplateCertificationCompatibilityReport::from_profiles(&self.certified_profiles);
    }

    /// Validates the M5 template certification invariants.
    pub fn validate(&self) -> Vec<M5TemplateCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_TEMPLATE_CERTIFICATION_RECORD_KIND {
            violations.push(M5TemplateCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_TEMPLATE_CERTIFICATION_SCHEMA_VERSION {
            violations.push(M5TemplateCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5TemplateCertificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_profiles(self, &mut violations);
        validate_compatibility_report(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 template certification packet serializes"),
        ) {
            violations.push(M5TemplateCertificationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 template certification packet serializes")
    }

    /// Profiles currently publishable (certified or narrowed).
    pub fn publishable_profiles(&self) -> impl Iterator<Item = &M5TemplateCertifiedProfile> {
        self.certified_profiles
            .iter()
            .filter(|profile| profile.verdict.is_publishable())
    }

    /// Deterministic Markdown summary for support, gallery, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Template Registry, Scaffold Planner, Framework Packs, and Archetype Health Certification\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.certification_label));
        out.push_str(&format!(
            "- Profiles: {} ({} certified, {} narrowed, {} blocked, {} uncertified)\n",
            self.compatibility_report.total_profiles,
            self.compatibility_report.certified_count,
            self.compatibility_report.narrowed_count,
            self.compatibility_report.blocked_count,
            self.compatibility_report.not_certified_count,
        ));
        out.push_str(&format!(
            "- All profiles publishable: {}\n",
            self.compatibility_report.all_profiles_publishable
        ));
        out.push_str(&format!(
            "- Promotion: {}\n",
            self.compatibility_report.promotion_note
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Profiles\n\n");
        for profile in &self.certified_profiles {
            out.push_str(&format!(
                "- **{}** ({}): `{}` (claimed `{}`)\n",
                profile.profile.as_str(),
                profile.lane.as_str(),
                profile.verdict.as_str(),
                profile.claimed_qualification.as_str(),
            ));
            out.push_str(&format!(
                "  - Evidence: `{}`\n",
                profile.evidence_artifact_ref
            ));
            out.push_str(&format!(
                "  - Proof fresh: {} (last refresh: {})\n",
                profile.proof_freshness.proof_fresh, profile.proof_freshness.last_proof_refresh
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 template certification export.
#[derive(Debug)]
pub enum M5TemplateCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5TemplateCertificationViolation>),
}

impl fmt::Display for M5TemplateCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 template certification export parse failed: {error}"
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
                    "m5 template certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5TemplateCertificationArtifactError {}

/// Validation failures emitted by [`M5TemplateCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5TemplateCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required claimed profile is missing from the certification.
    RequiredProfileMissing,
    /// A certified profile is incomplete.
    ProfileIncomplete,
    /// A publishable profile is missing evidence refs.
    PublishableProfileMissingEvidence,
    /// A profile has no downgrade triggers.
    DowngradeTriggersMissing,
    /// The compatibility report does not agree with the profile verdicts.
    CompatibilityReportMismatch,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5TemplateCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredProfileMissing => "required_profile_missing",
            Self::ProfileIncomplete => "profile_incomplete",
            Self::PublishableProfileMissingEvidence => "publishable_profile_missing_evidence",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::CompatibilityReportMismatch => "compatibility_report_mismatch",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in M5 template certification export.
///
/// This is the canonical reader: a gallery, pack header, scaffold run,
/// diagnostics, or support-export surface calls it to ingest the certification
/// packet rather than cloning status text.
///
/// # Errors
///
/// Returns [`M5TemplateCertificationArtifactError`] when the checked-in support
/// export fails to parse or fails validation.
pub fn current_m5_template_certification_export(
) -> Result<M5TemplateCertificationPacket, M5TemplateCertificationArtifactError> {
    let packet: M5TemplateCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/templates/m5/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile/support_export.json"
    )))
    .map_err(M5TemplateCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5TemplateCertificationArtifactError::Validation(violations))
    }
}

/// First-consumer certification: reads every claimed profile's checked-in support
/// export through its own producer and certifies a profile only when its evidence
/// currently validates.
///
/// A profile whose upstream export fails to parse or validate is recorded as
/// [`M5TemplateCertificationVerdict::Blocked`], so a stale or underqualified
/// profile narrows the certification instead of leaving it greener than the
/// evidence. The returned packet's compatibility report and proof-freshness
/// metadata are minted from `minted_at` and `proof_freshness`.
pub fn certify_from_current_exports(
    packet_id: String,
    certification_label: String,
    minted_at: String,
    proof_freshness: M5TemplateCertificationProofFreshness,
) -> M5TemplateCertificationPacket {
    let profiles = M5TemplateProfile::ALL
        .into_iter()
        .map(|profile| {
            let descriptor = profile_descriptor(profile);
            let evidence_valid = descriptor.evidence_valid();
            let verdict = if !evidence_valid {
                M5TemplateCertificationVerdict::Blocked
            } else {
                descriptor.default_verdict
            };
            M5TemplateCertifiedProfile {
                profile,
                lane: descriptor.lane,
                claimed_qualification: descriptor.claimed_qualification,
                verdict,
                upstream_record_kind: descriptor.upstream_record_kind.to_owned(),
                evidence_artifact_ref: descriptor.evidence_artifact_ref.to_owned(),
                evidence_schema_ref: descriptor.evidence_schema_ref.to_owned(),
                evidence_doc_ref: descriptor.evidence_doc_ref.to_owned(),
                downgrade_triggers: descriptor.downgrade_triggers.clone(),
                rollback_posture: descriptor.rollback_posture,
                proof_freshness: M5TemplateCertificationProfileFreshness {
                    proof_freshness_slo_hours: proof_freshness.proof_freshness_slo_hours,
                    last_proof_refresh: proof_freshness.last_proof_refresh.clone(),
                    proof_fresh: true,
                },
            }
        })
        .collect::<Vec<_>>();

    let compatibility_report = M5TemplateCertificationCompatibilityReport::from_profiles(&profiles);

    M5TemplateCertificationPacket::new(M5TemplateCertificationPacketInput {
        packet_id,
        certification_label,
        certified_profiles: profiles,
        compatibility_report,
        trust_review: canonical_trust_review(),
        consumer_projection: canonical_consumer_projection(),
        proof_freshness,
        source_contract_refs: canonical_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at,
    })
}

/// Canonical trust review block with every invariant satisfied.
pub fn canonical_trust_review() -> M5TemplateCertificationTrustReview {
    M5TemplateCertificationTrustReview {
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

/// Canonical consumer projection block with every surface projecting certification truth.
pub fn canonical_consumer_projection() -> M5TemplateCertificationConsumerProjection {
    M5TemplateCertificationConsumerProjection {
        gallery_shows_certification: true,
        pack_header_shows_certification: true,
        scaffold_run_shows_certification: true,
        diff_review_shows_certification: true,
        cli_headless_shows_certification: true,
        support_export_shows_certification: true,
        diagnostics_shows_certification: true,
        help_about_shows_certification: true,
        preview_labs_label_for_unqualified_profiles: true,
    }
}

/// Canonical source contract refs that every certification export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        M5_TEMPLATE_CERTIFICATION_SCHEMA_REF.to_owned(),
        M5_TEMPLATE_CERTIFICATION_DOC_REF.to_owned(),
        M5_TEMPLATE_CERTIFICATION_MATRIX_SCHEMA_REF.to_owned(),
        M5_TEMPLATE_CERTIFICATION_MATRIX_DOC_REF.to_owned(),
    ]
}

/// Static descriptor binding a claimed profile to its lane, claim, and evidence refs.
struct ProfileDescriptor {
    lane: M5TemplateCertificationLane,
    claimed_qualification: M5TemplateCertificationQualificationClass,
    default_verdict: M5TemplateCertificationVerdict,
    upstream_record_kind: &'static str,
    evidence_artifact_ref: &'static str,
    evidence_schema_ref: &'static str,
    evidence_doc_ref: &'static str,
    downgrade_triggers: Vec<M5TemplateCertificationDowngradeTrigger>,
    rollback_posture: M5TemplateCertificationRollbackPosture,
    evidence_probe: fn() -> bool,
}

impl ProfileDescriptor {
    fn evidence_valid(&self) -> bool {
        (self.evidence_probe)()
    }
}

fn profile_descriptor(profile: M5TemplateProfile) -> ProfileDescriptor {
    use M5TemplateCertificationDowngradeTrigger as Trigger;
    use M5TemplateCertificationLane as Lane;
    use M5TemplateCertificationQualificationClass as Qual;
    use M5TemplateCertificationRollbackPosture as Rollback;
    use M5TemplateCertificationVerdict as Verdict;

    match profile {
        M5TemplateProfile::SignedTemplateRegistry => ProfileDescriptor {
            lane: Lane::TemplateRegistry,
            claimed_qualification: Qual::Stable,
            default_verdict: Verdict::Certified,
            upstream_record_kind:
                crate::implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows::SIGNED_TEMPLATE_REGISTRY_RECORD_KIND,
            evidence_artifact_ref:
                crate::implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows::SIGNED_TEMPLATE_REGISTRY_ARTIFACT_REF,
            evidence_schema_ref:
                crate::implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows::SIGNED_TEMPLATE_REGISTRY_SCHEMA_REF,
            evidence_doc_ref:
                crate::implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows::SIGNED_TEMPLATE_REGISTRY_DOC_REF,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::SignatureUnverified,
                Trigger::TemplateRevisionUnavailable,
            ],
            rollback_posture: Rollback::ReadOnlyNoMutation,
            evidence_probe: || crate::implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows::current_signed_template_registry_export().is_ok(),
        },
        M5TemplateProfile::GenerationRecovery => ProfileDescriptor {
            lane: Lane::ScaffoldPlanner,
            claimed_qualification: Qual::Stable,
            default_verdict: Verdict::Certified,
            upstream_record_kind:
                crate::add_generation_diff_review_rollback_or_delete_generated_recovery_and_managed_zone_honesty::GENERATION_RECOVERY_RECORD_KIND,
            evidence_artifact_ref:
                crate::add_generation_diff_review_rollback_or_delete_generated_recovery_and_managed_zone_honesty::GENERATION_RECOVERY_ARTIFACT_REF,
            evidence_schema_ref:
                crate::add_generation_diff_review_rollback_or_delete_generated_recovery_and_managed_zone_honesty::GENERATION_RECOVERY_SCHEMA_REF,
            evidence_doc_ref:
                crate::add_generation_diff_review_rollback_or_delete_generated_recovery_and_managed_zone_honesty::GENERATION_RECOVERY_DOC_REF,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::ScaffoldPreviewUnavailable,
                Trigger::LineageTruthMissing,
            ],
            rollback_posture: Rollback::GenerationRollbackOrDeleteGenerated,
            evidence_probe: || crate::add_generation_diff_review_rollback_or_delete_generated_recovery_and_managed_zone_honesty::current_generation_recovery_export().is_ok(),
        },
        M5TemplateProfile::FrameworkGeneratorRun => ProfileDescriptor {
            lane: Lane::ScaffoldPlanner,
            claimed_qualification: Qual::Stable,
            default_verdict: Verdict::Certified,
            upstream_record_kind:
                crate::implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse::GENERATOR_RUN_RECORD_KIND,
            evidence_artifact_ref:
                crate::implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse::GENERATOR_RUN_ARTIFACT_REF,
            evidence_schema_ref:
                crate::implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse::GENERATOR_RUN_SCHEMA_REF,
            evidence_doc_ref:
                crate::implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse::GENERATOR_RUN_DOC_REF,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::ScaffoldPreviewUnavailable,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::GenerationRollbackOrDeleteGenerated,
            evidence_probe: || crate::implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse::current_generator_run_export().is_ok(),
        },
        M5TemplateProfile::FrameworkPackHeader => ProfileDescriptor {
            lane: Lane::FrameworkPack,
            claimed_qualification: Qual::Stable,
            default_verdict: Verdict::Certified,
            upstream_record_kind:
                crate::implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners::FRAMEWORK_PACK_RECORD_KIND,
            evidence_artifact_ref:
                crate::implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners::FRAMEWORK_PACK_ARTIFACT_REF,
            evidence_schema_ref:
                crate::implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners::FRAMEWORK_PACK_SCHEMA_REF,
            evidence_doc_ref:
                crate::implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners::FRAMEWORK_PACK_DOC_REF,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::SupportClassNarrowed,
                Trigger::PolicyBlocked,
            ],
            rollback_posture: Rollback::ReadOnlyNoMutation,
            evidence_probe: || crate::implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners::current_framework_pack_export().is_ok(),
        },
        M5TemplateProfile::RicherFrameworkPacks => ProfileDescriptor {
            lane: Lane::FrameworkPack,
            claimed_qualification: Qual::Beta,
            default_verdict: Verdict::NarrowedCertified,
            upstream_record_kind:
                crate::add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter::RICHER_FRAMEWORK_PACK_RECORD_KIND,
            evidence_artifact_ref:
                crate::add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter::RICHER_FRAMEWORK_PACK_ARTIFACT_REF,
            evidence_schema_ref:
                crate::add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter::RICHER_FRAMEWORK_PACK_SCHEMA_REF,
            evidence_doc_ref:
                crate::add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter::RICHER_FRAMEWORK_PACK_DOC_REF,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::SupportClassNarrowed,
                Trigger::HeuristicPresentedAsExact,
            ],
            rollback_posture: Rollback::ReadOnlyNoMutation,
            evidence_probe: || crate::add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter::current_richer_framework_pack_export().is_ok(),
        },
        M5TemplateProfile::AppTopologyViews => ProfileDescriptor {
            lane: Lane::FrameworkPack,
            claimed_qualification: Qual::Stable,
            default_verdict: Verdict::Certified,
            upstream_record_kind:
                crate::ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth::APP_TOPOLOGY_RECORD_KIND,
            evidence_artifact_ref:
                crate::ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth::APP_TOPOLOGY_ARTIFACT_REF,
            evidence_schema_ref:
                crate::ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth::APP_TOPOLOGY_SCHEMA_REF,
            evidence_doc_ref:
                crate::ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth::APP_TOPOLOGY_DOC_REF,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::LineageTruthMissing,
                Trigger::ScopeExpansionUnqualified,
            ],
            rollback_posture: Rollback::ReadOnlyNoMutation,
            evidence_probe: || crate::ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth::current_app_topology_export().is_ok(),
        },
        M5TemplateProfile::ConventionDiagnostics => ProfileDescriptor {
            lane: Lane::FrameworkPack,
            claimed_qualification: Qual::Beta,
            default_verdict: Verdict::NarrowedCertified,
            upstream_record_kind:
                crate::add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure::CONVENTION_DIAGNOSTIC_RECORD_KIND,
            evidence_artifact_ref:
                crate::add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure::CONVENTION_DIAGNOSTIC_ARTIFACT_REF,
            evidence_schema_ref:
                crate::add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure::CONVENTION_DIAGNOSTIC_SCHEMA_REF,
            evidence_doc_ref:
                crate::add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure::CONVENTION_DIAGNOSTIC_DOC_REF,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::HeuristicPresentedAsExact,
                Trigger::SupportClassNarrowed,
            ],
            rollback_posture: Rollback::ReadOnlyNoMutation,
            evidence_probe: || crate::add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure::current_convention_diagnostic_export().is_ok(),
        },
        M5TemplateProfile::ArchetypeHealthBundle => ProfileDescriptor {
            lane: Lane::ArchetypeHealth,
            claimed_qualification: Qual::Stable,
            default_verdict: Verdict::Certified,
            upstream_record_kind:
                crate::ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance::ARCHETYPE_HEALTH_RECORD_KIND,
            evidence_artifact_ref:
                crate::ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance::ARCHETYPE_HEALTH_ARTIFACT_REF,
            evidence_schema_ref:
                crate::ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance::ARCHETYPE_HEALTH_SCHEMA_REF,
            evidence_doc_ref:
                crate::ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance::ARCHETYPE_HEALTH_DOC_REF,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::ArchetypeHealthStale,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::ManagedZoneRegenerate,
            evidence_probe: || crate::ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance::current_archetype_health_export().is_ok(),
        },
    }
}

fn validate_source_contracts(
    packet: &M5TemplateCertificationPacket,
    violations: &mut Vec<M5TemplateCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_TEMPLATE_CERTIFICATION_SCHEMA_REF,
        M5_TEMPLATE_CERTIFICATION_DOC_REF,
        M5_TEMPLATE_CERTIFICATION_MATRIX_SCHEMA_REF,
        M5_TEMPLATE_CERTIFICATION_MATRIX_DOC_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5TemplateCertificationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_profiles(
    packet: &M5TemplateCertificationPacket,
    violations: &mut Vec<M5TemplateCertificationViolation>,
) {
    let present: BTreeSet<M5TemplateProfile> = packet
        .certified_profiles
        .iter()
        .map(|profile| profile.profile)
        .collect();
    for required in M5TemplateProfile::ALL {
        if !present.contains(&required) {
            violations.push(M5TemplateCertificationViolation::RequiredProfileMissing);
            return;
        }
    }

    for profile in &packet.certified_profiles {
        if profile.upstream_record_kind.trim().is_empty()
            || profile.evidence_artifact_ref.trim().is_empty()
            || profile.evidence_schema_ref.trim().is_empty()
            || profile.evidence_doc_ref.trim().is_empty()
            || profile.proof_freshness.last_proof_refresh.trim().is_empty()
            || profile.proof_freshness.proof_freshness_slo_hours == 0
        {
            violations.push(M5TemplateCertificationViolation::ProfileIncomplete);
        }
        if profile.verdict.is_publishable() && profile.evidence_artifact_ref.trim().is_empty() {
            violations.push(M5TemplateCertificationViolation::PublishableProfileMissingEvidence);
        }
        if profile.downgrade_triggers.is_empty() {
            violations.push(M5TemplateCertificationViolation::DowngradeTriggersMissing);
        }
    }
}

fn validate_compatibility_report(
    packet: &M5TemplateCertificationPacket,
    violations: &mut Vec<M5TemplateCertificationViolation>,
) {
    let recomputed =
        M5TemplateCertificationCompatibilityReport::from_profiles(&packet.certified_profiles);
    if recomputed != packet.compatibility_report {
        violations.push(M5TemplateCertificationViolation::CompatibilityReportMismatch);
    }
}

fn validate_trust_review(
    packet: &M5TemplateCertificationPacket,
    violations: &mut Vec<M5TemplateCertificationViolation>,
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
            violations.push(M5TemplateCertificationViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5TemplateCertificationPacket,
    violations: &mut Vec<M5TemplateCertificationViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.gallery_shows_certification,
        projection.pack_header_shows_certification,
        projection.scaffold_run_shows_certification,
        projection.diff_review_shows_certification,
        projection.cli_headless_shows_certification,
        projection.support_export_shows_certification,
        projection.diagnostics_shows_certification,
        projection.help_about_shows_certification,
        projection.preview_labs_label_for_unqualified_profiles,
    ] {
        if !ok {
            violations.push(M5TemplateCertificationViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5TemplateCertificationPacket,
    violations: &mut Vec<M5TemplateCertificationViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5TemplateCertificationViolation::ProofFreshnessIncomplete);
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
