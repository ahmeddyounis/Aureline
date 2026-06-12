//! Frozen command-governance contract for feature-family command surfaces.
//!
//! This module binds the canonical descriptor-field vocabulary,
//! invocation-session field vocabulary, result-packet field vocabulary,
//! lifecycle-dependency vocabulary, downgrade rules, and per-family
//! cross-surface governance matrix into one export-safe packet. The packet is a
//! command-governance control surface rather than a second command registry:
//! descriptor/runtime contracts remain owned by the descriptor and invocation
//! modules, while this packet freezes how feature-family command surfaces prove
//! they may keep stable wording across desktop, CLI, AI, recipes, extensions,
//! and browser-companion handoff routes.
//!
//! Stable-facing rows that lose descriptor proof, invocation/result proof,
//! disabled-reason vocabulary proof, preview truth, authority parity, or fresh
//! evidence must narrow automatically. Help/About, release, and support surfaces
//! therefore consume one packet instead of widening claims with handwritten copy.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`CommandGovernanceContractPacket`].
pub const FREEZE_COMMAND_GOVERNANCE_CONTRACT_RECORD_KIND: &str =
    "command_governance_contract_packet";

/// Schema version for command-governance packets.
pub const FREEZE_COMMAND_GOVERNANCE_CONTRACT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the command-governance boundary schema.
pub const FREEZE_COMMAND_GOVERNANCE_CONTRACT_SCHEMA_REF: &str =
    "schemas/commands/freeze_command_governance_contract.schema.json";

/// Repo-relative path of the command-governance companion doc.
pub const FREEZE_COMMAND_GOVERNANCE_CONTRACT_DOC_REF: &str =
    "docs/commands/freeze_command_governance_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const FREEZE_COMMAND_GOVERNANCE_CONTRACT_FIXTURE_DIR: &str =
    "fixtures/commands/freeze_command_governance_contract";

/// Repo-relative path of the checked support export.
pub const FREEZE_COMMAND_GOVERNANCE_CONTRACT_ARTIFACT_REF: &str =
    "artifacts/commands/freeze_command_governance_contract/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const FREEZE_COMMAND_GOVERNANCE_CONTRACT_SUMMARY_REF: &str =
    "artifacts/commands/freeze_command_governance_contract/summary.md";

/// Repo-relative path of the frozen descriptor contract.
pub const FREEZE_COMMAND_GOVERNANCE_DESCRIPTOR_CONTRACT_REF: &str =
    "docs/commands/command_descriptor_contract.md";

/// Repo-relative path of the frozen invocation/result/parity contract.
pub const FREEZE_COMMAND_GOVERNANCE_INVOCATION_PARITY_CONTRACT_REF: &str =
    "docs/commands/invocation_result_and_parity_contract.md";

/// Repo-relative path of the capability lifecycle ADR.
pub const FREEZE_COMMAND_GOVERNANCE_LIFECYCLE_ADR_REF: &str =
    "docs/adr/0011-capability-lifecycle-and-dependency-markers.md";

/// Repo-relative path of the canonical disabled-reason vocabulary.
pub const FREEZE_COMMAND_GOVERNANCE_DISABLED_REASON_REF: &str =
    "docs/commands/disabled_reason_vocabulary.md";

/// Repo-relative path of the claim-publication manifest.
pub const FREEZE_COMMAND_GOVERNANCE_CLAIM_PUBLICATION_REF: &str =
    "artifacts/release/stable/claim-publication-manifest/manifest.json";

/// Repo-relative path of the capability lifecycle registry.
pub const FREEZE_COMMAND_GOVERNANCE_CAPABILITY_REGISTRY_REF: &str =
    "artifacts/governance/capability_lifecycle_registry.yaml";

/// Canonical descriptor-field class frozen by the governance contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandDescriptorFieldClass {
    /// Stable command identity and revision-bearing id.
    CommandIdentity,
    /// Alias, deprecation, and replacement metadata.
    AliasDeprecationState,
    /// Capability / risk class.
    CapabilityClass,
    /// Preview taxonomy.
    PreviewClass,
    /// Automation labels and headless posture.
    AutomationLabels,
    /// Origin, publisher, or policy provenance.
    OriginMetadata,
    /// Structured result-packet schema references.
    ResultPacketReferences,
    /// Lifecycle / support / channel / freshness posture.
    LifecycleMetadata,
}

impl CommandDescriptorFieldClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandIdentity => "command_identity",
            Self::AliasDeprecationState => "alias_deprecation_state",
            Self::CapabilityClass => "capability_class",
            Self::PreviewClass => "preview_class",
            Self::AutomationLabels => "automation_labels",
            Self::OriginMetadata => "origin_metadata",
            Self::ResultPacketReferences => "result_packet_references",
            Self::LifecycleMetadata => "lifecycle_metadata",
        }
    }

    /// Required descriptor-field coverage.
    pub const fn required_coverage() -> [Self; 8] {
        [
            Self::CommandIdentity,
            Self::AliasDeprecationState,
            Self::CapabilityClass,
            Self::PreviewClass,
            Self::AutomationLabels,
            Self::OriginMetadata,
            Self::ResultPacketReferences,
            Self::LifecycleMetadata,
        ]
    }
}

/// Canonical invocation-session field class frozen by the governance contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvocationSessionFieldClass {
    /// Surface that issued the invocation.
    IssuingSurface,
    /// Provenance for typed arguments.
    ArgumentProvenance,
    /// Context snapshot and basis ref.
    ContextSnapshot,
    /// Structured enablement decision.
    EnablementDecision,
    /// Execution-intent class.
    ExecutionIntent,
    /// Outcome block.
    Outcome,
    /// Artifact/evidence joins.
    Artifacts,
    /// Timing, latency, or cost-band disclosure.
    TimingCostBands,
}

impl InvocationSessionFieldClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IssuingSurface => "issuing_surface",
            Self::ArgumentProvenance => "argument_provenance",
            Self::ContextSnapshot => "context_snapshot",
            Self::EnablementDecision => "enablement_decision",
            Self::ExecutionIntent => "execution_intent",
            Self::Outcome => "outcome",
            Self::Artifacts => "artifacts",
            Self::TimingCostBands => "timing_cost_bands",
        }
    }

    /// Required invocation-session field coverage.
    pub const fn required_coverage() -> [Self; 8] {
        [
            Self::IssuingSurface,
            Self::ArgumentProvenance,
            Self::ContextSnapshot,
            Self::EnablementDecision,
            Self::ExecutionIntent,
            Self::Outcome,
            Self::Artifacts,
            Self::TimingCostBands,
        ]
    }
}

/// Canonical result-packet field class frozen by the governance contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultPacketFieldClass {
    /// Canonical command identity echoed into the result packet.
    CanonicalCommandIdentity,
    /// Outcome, warning, and error codes.
    OutcomeCodes,
    /// Created or affected artifact refs.
    ArtifactRefs,
    /// Rollback posture and handle.
    RollbackHandle,
    /// Checkpoint refs.
    CheckpointRefs,
    /// Notification/activity joins.
    NotificationActivityRefs,
    /// Evidence refs.
    EvidenceRefs,
    /// Export / redaction posture.
    ExportPosture,
}

impl ResultPacketFieldClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalCommandIdentity => "canonical_command_identity",
            Self::OutcomeCodes => "outcome_codes",
            Self::ArtifactRefs => "artifact_refs",
            Self::RollbackHandle => "rollback_handle",
            Self::CheckpointRefs => "checkpoint_refs",
            Self::NotificationActivityRefs => "notification_activity_refs",
            Self::EvidenceRefs => "evidence_refs",
            Self::ExportPosture => "export_posture",
        }
    }

    /// Required result-packet field coverage.
    pub const fn required_coverage() -> [Self; 8] {
        [
            Self::CanonicalCommandIdentity,
            Self::OutcomeCodes,
            Self::ArtifactRefs,
            Self::RollbackHandle,
            Self::CheckpointRefs,
            Self::NotificationActivityRefs,
            Self::EvidenceRefs,
            Self::ExportPosture,
        ]
    }
}

/// Lifecycle-dependency marker class used when a stable-facing command surface
/// still relies on a narrower dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleDependencyClass {
    /// The dependency remains in Labs.
    LabsDependency,
    /// The dependency remains in Preview.
    PreviewDependency,
    /// The dependency remains in Beta.
    BetaDependency,
    /// The dependency is policy-gated or kill-switched.
    PolicyGatedDependency,
    /// The dependency is ownerless, stale, or otherwise underqualified.
    UnderqualifiedDependency,
}

impl LifecycleDependencyClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LabsDependency => "labs_dependency",
            Self::PreviewDependency => "preview_dependency",
            Self::BetaDependency => "beta_dependency",
            Self::PolicyGatedDependency => "policy_gated_dependency",
            Self::UnderqualifiedDependency => "underqualified_dependency",
        }
    }

    /// Required lifecycle-dependency vocabulary coverage.
    pub const fn required_coverage() -> [Self; 5] {
        [
            Self::LabsDependency,
            Self::PreviewDependency,
            Self::BetaDependency,
            Self::PolicyGatedDependency,
            Self::UnderqualifiedDependency,
        ]
    }
}

/// Feature family covered by the command-governance packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeatureFamilyClass {
    /// Notebook commands and notebook-adjacent actions.
    Notebook,
    /// Data/API exploration and data grid actions.
    DataApi,
    /// Profiling and regression-analysis actions.
    Profiler,
    /// Docs, search, and browser handoff actions.
    DocsBrowser,
    /// Pipeline, task, and job actions.
    Pipeline,
    /// Framework-pack and scaffold pack actions.
    FrameworkPack,
    /// Companion and handoff actions.
    Companion,
    /// Sync and device-registry actions.
    Sync,
    /// Incident, advisory, and response actions.
    Incident,
    /// Infrastructure and managed-control actions.
    Infrastructure,
}

impl FeatureFamilyClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::DataApi => "data_api",
            Self::Profiler => "profiler",
            Self::DocsBrowser => "docs_browser",
            Self::Pipeline => "pipeline",
            Self::FrameworkPack => "framework_pack",
            Self::Companion => "companion",
            Self::Sync => "sync",
            Self::Incident => "incident",
            Self::Infrastructure => "infrastructure",
        }
    }

    /// Required feature-family coverage.
    pub const fn required_coverage() -> [Self; 10] {
        [
            Self::Notebook,
            Self::DataApi,
            Self::Profiler,
            Self::DocsBrowser,
            Self::Pipeline,
            Self::FrameworkPack,
            Self::Companion,
            Self::Sync,
            Self::Incident,
            Self::Infrastructure,
        ]
    }
}

/// Surface that consumes the canonical command-governance truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandGovernanceSurfaceClass {
    /// Desktop shell command routes.
    Desktop,
    /// CLI or headless routes.
    Cli,
    /// AI tool or assistant routes.
    Ai,
    /// Recipe or declarative automation routes.
    Recipe,
    /// Extension-contributed routes.
    Extension,
    /// Browser or companion handoff routes.
    BrowserCompanion,
}

impl CommandGovernanceSurfaceClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Cli => "cli",
            Self::Ai => "ai",
            Self::Recipe => "recipe",
            Self::Extension => "extension",
            Self::BrowserCompanion => "browser_companion",
        }
    }

    /// Required surface coverage per feature family.
    pub const fn required_coverage() -> [Self; 6] {
        [
            Self::Desktop,
            Self::Cli,
            Self::Ai,
            Self::Recipe,
            Self::Extension,
            Self::BrowserCompanion,
        ]
    }
}

/// Consumer surface that must never widen claim copy beyond the current packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationConsumerSurfaceClass {
    /// Help or About surfaces.
    HelpAbout,
    /// Release-center or release-note surfaces.
    Release,
    /// Support export or diagnostics export surfaces.
    Support,
    /// Documentation or command-reference surfaces.
    Docs,
}

impl PublicationConsumerSurfaceClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelpAbout => "help_about",
            Self::Release => "release",
            Self::Support => "support",
            Self::Docs => "docs",
        }
    }

    /// Required publication-consumer coverage for downgrade rules.
    pub const fn required_coverage() -> [Self; 4] {
        [Self::HelpAbout, Self::Release, Self::Support, Self::Docs]
    }
}

/// Effective claim the packet allows a surface to publish.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceClaimClass {
    /// Stable wording may publish.
    Stable,
    /// Narrowed to beta wording.
    Beta,
    /// Narrowed to preview wording.
    Preview,
    /// Narrowed to an explicit policy-blocked state.
    PolicyBlocked,
    /// No claim-bearing wording may publish.
    Unsupported,
}

impl GovernanceClaimClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::PolicyBlocked => "policy_blocked",
            Self::Unsupported => "unsupported",
        }
    }

    /// Whether the claim is stable.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Freshness state for evidence backing a feature-family claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshnessClass {
    /// Evidence is current.
    Current,
    /// Evidence is current but due soon.
    DueForRefresh,
    /// Evidence is stale.
    Stale,
    /// Evidence is missing.
    Missing,
}

impl EvidenceFreshnessClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::DueForRefresh => "due_for_refresh",
            Self::Stale => "stale",
            Self::Missing => "missing",
        }
    }

    /// Whether this freshness state forces a narrowing.
    pub const fn forces_narrowing(self) -> bool {
        matches!(self, Self::Stale | Self::Missing)
    }
}

/// Trigger that forces claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeTriggerClass {
    /// The descriptor contract is missing or stale.
    DescriptorContractMissing,
    /// The invocation-session contract is missing or stale.
    InvocationSessionContractMissing,
    /// The result-packet contract is missing or stale.
    ResultPacketContractMissing,
    /// A lifecycle dependency remains below the stable claim.
    LifecycleDependencyPresent,
    /// Backing evidence is stale.
    StaleEvidence,
    /// Backing evidence is missing.
    MissingEvidence,
    /// Cross-surface authority parity broke.
    AuthorityParityBroken,
    /// Preview or approval semantics drifted from the canonical command.
    PreviewSemanticsStale,
    /// Disabled-reason vocabulary parity is stale or missing.
    DisabledReasonVocabularyMissing,
}

impl DowngradeTriggerClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DescriptorContractMissing => "descriptor_contract_missing",
            Self::InvocationSessionContractMissing => "invocation_session_contract_missing",
            Self::ResultPacketContractMissing => "result_packet_contract_missing",
            Self::LifecycleDependencyPresent => "lifecycle_dependency_present",
            Self::StaleEvidence => "stale_evidence",
            Self::MissingEvidence => "missing_evidence",
            Self::AuthorityParityBroken => "authority_parity_broken",
            Self::PreviewSemanticsStale => "preview_semantics_stale",
            Self::DisabledReasonVocabularyMissing => "disabled_reason_vocabulary_missing",
        }
    }

    /// Required downgrade-trigger coverage.
    pub const fn required_coverage() -> [Self; 9] {
        [
            Self::DescriptorContractMissing,
            Self::InvocationSessionContractMissing,
            Self::ResultPacketContractMissing,
            Self::LifecycleDependencyPresent,
            Self::StaleEvidence,
            Self::MissingEvidence,
            Self::AuthorityParityBroken,
            Self::PreviewSemanticsStale,
            Self::DisabledReasonVocabularyMissing,
        ]
    }
}

/// Canonical refs the governance packet binds together.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceContractRefs {
    /// Canonical descriptor contract ref.
    pub descriptor_contract_ref: String,
    /// Canonical invocation/result/parity contract ref.
    pub invocation_parity_contract_ref: String,
    /// Canonical lifecycle ADR ref.
    pub lifecycle_adr_ref: String,
    /// Canonical disabled-reason vocabulary ref.
    pub disabled_reason_vocabulary_ref: String,
    /// Canonical descriptor schema ref.
    pub descriptor_schema_ref: String,
    /// Canonical invocation-session schema ref.
    pub invocation_session_schema_ref: String,
    /// Canonical result-packet schema ref.
    pub result_packet_schema_ref: String,
    /// Canonical claim-publication manifest ref.
    pub claim_publication_manifest_ref: String,
    /// Canonical capability lifecycle registry ref.
    pub capability_lifecycle_registry_ref: String,
}

impl GovernanceContractRefs {
    /// Returns the canonical governance refs.
    pub fn canonical() -> Self {
        Self {
            descriptor_contract_ref: FREEZE_COMMAND_GOVERNANCE_DESCRIPTOR_CONTRACT_REF.to_owned(),
            invocation_parity_contract_ref:
                FREEZE_COMMAND_GOVERNANCE_INVOCATION_PARITY_CONTRACT_REF.to_owned(),
            lifecycle_adr_ref: FREEZE_COMMAND_GOVERNANCE_LIFECYCLE_ADR_REF.to_owned(),
            disabled_reason_vocabulary_ref: FREEZE_COMMAND_GOVERNANCE_DISABLED_REASON_REF
                .to_owned(),
            descriptor_schema_ref: "schemas/commands/command_descriptor.schema.json".to_owned(),
            invocation_session_schema_ref:
                "schemas/commands/command_invocation_session.schema.json".to_owned(),
            result_packet_schema_ref: "schemas/commands/command_result_packet.schema.json"
                .to_owned(),
            claim_publication_manifest_ref: FREEZE_COMMAND_GOVERNANCE_CLAIM_PUBLICATION_REF
                .to_owned(),
            capability_lifecycle_registry_ref: FREEZE_COMMAND_GOVERNANCE_CAPABILITY_REGISTRY_REF
                .to_owned(),
        }
    }

    fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// One descriptor-field row frozen by the contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorFieldRow {
    /// Field class being frozen.
    pub field_class: CommandDescriptorFieldClass,
    /// Human-readable label for reviewers and support.
    pub field_label: String,
    /// Pointer into the descriptor contract or schema.
    pub source_pointer: String,
    /// Whether the field is exported on public command surfaces.
    pub exported: bool,
}

/// One invocation-session field row frozen by the contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvocationSessionFieldRow {
    /// Field class being frozen.
    pub field_class: InvocationSessionFieldClass,
    /// Human-readable label for reviewers and support.
    pub field_label: String,
    /// Pointer into the invocation/result contract or schema.
    pub source_pointer: String,
    /// Whether the field is required on every invocation surface.
    pub required_on_every_surface: bool,
}

/// One result-packet field row frozen by the contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultPacketFieldRow {
    /// Field class being frozen.
    pub field_class: ResultPacketFieldClass,
    /// Human-readable label for reviewers and support.
    pub field_label: String,
    /// Pointer into the result contract or schema.
    pub source_pointer: String,
    /// Whether the field is required on every result packet.
    pub required_on_every_result: bool,
}

/// One lifecycle-dependency vocabulary row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleDependencyVocabularyRow {
    /// Marker class.
    pub dependency_class: LifecycleDependencyClass,
    /// Disclosure label shared by UI, docs, release, and support surfaces.
    pub disclosure_label: String,
    /// Pointer into the lifecycle ADR or registry vocabulary.
    pub source_pointer: String,
    /// Whether this dependency forces a stable-facing claim to narrow.
    pub narrows_stable_claim: bool,
}

/// One downgrade rule consumers apply when a trigger fires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradeRuleRow {
    /// Trigger this rule covers.
    pub trigger: DowngradeTriggerClass,
    /// Claim class a consumer must narrow to.
    pub narrowed_to: GovernanceClaimClass,
    /// Surfaces that must honor the rule.
    pub consumer_surfaces: Vec<PublicationConsumerSurfaceClass>,
    /// Whether stable copy is forbidden once the trigger fires.
    pub stable_copy_forbidden: bool,
    /// Reviewer-facing rule summary.
    pub summary: String,
}

/// One live lifecycle dependency attached to a feature-family row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleDependencyRow {
    /// Opaque marker ref.
    pub marker_ref: String,
    /// Marker class.
    pub dependency_class: LifecycleDependencyClass,
    /// Capability or experiment ref the family depends on.
    pub dependency_ref: String,
    /// Owner or owning packet ref for the dependency.
    pub owner_ref: String,
    /// Disclosure ref surfaced to users and support.
    pub disclosure_ref: String,
    /// Whether this dependency forces stable wording to narrow.
    pub narrowing_required: bool,
}

/// One per-surface governance row for a feature family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceGovernanceRow {
    /// Surface class this row covers.
    pub surface_class: CommandGovernanceSurfaceClass,
    /// True when the surrounding product plan would otherwise claim Stable.
    pub claimed_stable: bool,
    /// Effective claim this surface may publish today.
    pub effective_claim: GovernanceClaimClass,
    /// Whether stable-facing copy is still allowed to publish.
    pub published_stable_copy_allowed: bool,
    /// True when the surface reads the current descriptor contract.
    pub descriptor_contract_current: bool,
    /// True when the surface reads the current invocation-session contract.
    pub invocation_session_contract_current: bool,
    /// True when the surface reads the current result-packet contract.
    pub result_packet_contract_current: bool,
    /// True when dependency markers are disclosed on this surface.
    pub lifecycle_dependency_disclosed: bool,
    /// Marker refs attached to this surface row.
    pub dependency_marker_refs: Vec<String>,
    /// True when authority class cannot widen on this surface.
    pub authority_parity_preserved: bool,
    /// True when preview semantics match the canonical command.
    pub preview_parity_preserved: bool,
    /// True when approval semantics match the canonical command.
    pub approval_parity_preserved: bool,
    /// True when disabled-reason vocabulary matches the canonical command.
    pub disabled_reason_parity_preserved: bool,
    /// True when route/origin truth is disclosed.
    pub route_truth_disclosed: bool,
    /// Evidence freshness as rendered on this surface.
    pub evidence_freshness: EvidenceFreshnessClass,
    /// Downgrade reasons applied to this surface row.
    pub downgrade_reasons: Vec<DowngradeTriggerClass>,
}

impl SurfaceGovernanceRow {
    fn stable_ready(&self) -> bool {
        self.effective_claim.is_stable()
            && self.published_stable_copy_allowed
            && self.descriptor_contract_current
            && self.invocation_session_contract_current
            && self.result_packet_contract_current
            && self.lifecycle_dependency_disclosed
            && self.dependency_marker_refs.is_empty()
            && self.authority_parity_preserved
            && self.preview_parity_preserved
            && self.approval_parity_preserved
            && self.disabled_reason_parity_preserved
            && self.route_truth_disclosed
            && !self.evidence_freshness.forces_narrowing()
            && self.downgrade_reasons.is_empty()
    }
}

/// Governance row for one feature family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureFamilyGovernanceRow {
    /// Feature family this row covers.
    pub family_class: FeatureFamilyClass,
    /// Display label for docs and support.
    pub display_label: String,
    /// True when the surrounding launch plan would otherwise claim Stable.
    pub claimed_stable: bool,
    /// Effective claim this family may publish today.
    pub effective_claim: GovernanceClaimClass,
    /// Whether stable-facing copy is still allowed to publish.
    pub published_stable_copy_allowed: bool,
    /// Current descriptor-proof ref for this family.
    pub descriptor_proof_ref: String,
    /// Whether the descriptor proof is current.
    pub descriptor_contract_current: bool,
    /// Current invocation-session proof ref for this family.
    pub invocation_session_proof_ref: String,
    /// Whether the invocation-session proof is current.
    pub invocation_session_contract_current: bool,
    /// Current result-packet proof ref for this family.
    pub result_packet_proof_ref: String,
    /// Whether the result-packet proof is current.
    pub result_packet_contract_current: bool,
    /// Whether the disabled-reason vocabulary proof is current.
    pub disabled_reason_vocabulary_current: bool,
    /// Whether preview / approval posture proof is current.
    pub preview_semantics_current: bool,
    /// Rollout or qualification owner ref.
    pub rollout_owner_ref: String,
    /// Primary evidence packet ref for this family.
    pub evidence_packet_ref: String,
    /// Evidence freshness backing the family row.
    pub evidence_freshness: EvidenceFreshnessClass,
    /// Lifecycle dependencies attached to this family.
    pub lifecycle_dependencies: Vec<LifecycleDependencyRow>,
    /// Downgrade reasons applied at the family level.
    pub downgrade_reasons: Vec<DowngradeTriggerClass>,
    /// Per-surface governance rows.
    pub surface_rows: Vec<SurfaceGovernanceRow>,
}

impl FeatureFamilyGovernanceRow {
    fn stable_ready(&self) -> bool {
        self.effective_claim.is_stable()
            && self.published_stable_copy_allowed
            && self.descriptor_contract_current
            && self.invocation_session_contract_current
            && self.result_packet_contract_current
            && self.disabled_reason_vocabulary_current
            && self.preview_semantics_current
            && !self.evidence_freshness.forces_narrowing()
            && self.lifecycle_dependencies.is_empty()
            && self.downgrade_reasons.is_empty()
            && self
                .surface_rows
                .iter()
                .all(SurfaceGovernanceRow::stable_ready)
    }
}

/// Exportable lineage block for the checked packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceEvidenceExport {
    /// Evidence id shown in-product and in exports.
    pub evidence_id: String,
    /// JSON export ref.
    pub json_export_ref: String,
    /// Markdown summary ref.
    pub markdown_summary_ref: String,
    /// Help/About projection ref.
    pub help_projection_ref: String,
    /// Release projection ref.
    pub release_projection_ref: String,
    /// Support projection ref.
    pub support_projection_ref: String,
}

/// Constructor input for [`CommandGovernanceContractPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandGovernanceContractPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Display label for docs and support.
    pub display_label: String,
    /// Policy epoch ref the packet was evaluated under.
    pub policy_epoch_ref: String,
    /// Canonical refs the packet binds together.
    pub contract_refs: GovernanceContractRefs,
    /// Frozen descriptor-field rows.
    pub descriptor_fields: Vec<DescriptorFieldRow>,
    /// Frozen invocation-session field rows.
    pub invocation_session_fields: Vec<InvocationSessionFieldRow>,
    /// Frozen result-packet field rows.
    pub result_packet_fields: Vec<ResultPacketFieldRow>,
    /// Frozen lifecycle-dependency vocabulary rows.
    pub lifecycle_dependency_vocabulary: Vec<LifecycleDependencyVocabularyRow>,
    /// Frozen downgrade rules.
    pub downgrade_rules: Vec<DowngradeRuleRow>,
    /// Per-family governance rows.
    pub feature_family_rows: Vec<FeatureFamilyGovernanceRow>,
    /// Evidence lineage/export block.
    pub evidence_export: GovernanceEvidenceExport,
    /// Source contracts consumed by the packet.
    pub source_contract_refs: Vec<String>,
    /// Redaction class token for the packet.
    pub redaction_class_token: String,
    /// Mint time for the packet.
    pub minted_at: String,
}

/// Export-safe command-governance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandGovernanceContractPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Display label for docs and support.
    pub display_label: String,
    /// Policy epoch ref the packet was evaluated under.
    pub policy_epoch_ref: String,
    /// Canonical refs the packet binds together.
    pub contract_refs: GovernanceContractRefs,
    /// Frozen descriptor-field rows.
    pub descriptor_fields: Vec<DescriptorFieldRow>,
    /// Frozen invocation-session field rows.
    pub invocation_session_fields: Vec<InvocationSessionFieldRow>,
    /// Frozen result-packet field rows.
    pub result_packet_fields: Vec<ResultPacketFieldRow>,
    /// Frozen lifecycle-dependency vocabulary rows.
    pub lifecycle_dependency_vocabulary: Vec<LifecycleDependencyVocabularyRow>,
    /// Frozen downgrade rules.
    pub downgrade_rules: Vec<DowngradeRuleRow>,
    /// Per-family governance rows.
    pub feature_family_rows: Vec<FeatureFamilyGovernanceRow>,
    /// Evidence lineage/export block.
    pub evidence_export: GovernanceEvidenceExport,
    /// Source contracts consumed by the packet.
    pub source_contract_refs: Vec<String>,
    /// Redaction class token for the packet.
    pub redaction_class_token: String,
    /// Mint time for the packet.
    pub minted_at: String,
}

impl CommandGovernanceContractPacket {
    /// Builds a packet from canonical rows.
    pub fn new(input: CommandGovernanceContractPacketInput) -> Self {
        Self {
            record_kind: FREEZE_COMMAND_GOVERNANCE_CONTRACT_RECORD_KIND.to_owned(),
            schema_version: FREEZE_COMMAND_GOVERNANCE_CONTRACT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            display_label: input.display_label,
            policy_epoch_ref: input.policy_epoch_ref,
            contract_refs: input.contract_refs,
            descriptor_fields: input.descriptor_fields,
            invocation_session_fields: input.invocation_session_fields,
            result_packet_fields: input.result_packet_fields,
            lifecycle_dependency_vocabulary: input.lifecycle_dependency_vocabulary,
            downgrade_rules: input.downgrade_rules,
            feature_family_rows: input.feature_family_rows,
            evidence_export: input.evidence_export,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the packet's invariants.
    pub fn validate(&self) -> Vec<CommandGovernanceContractViolation> {
        let mut violations = Vec::new();
        if self.record_kind != FREEZE_COMMAND_GOVERNANCE_CONTRACT_RECORD_KIND {
            violations.push(CommandGovernanceContractViolation::WrongRecordKind);
        }
        if self.schema_version != FREEZE_COMMAND_GOVERNANCE_CONTRACT_SCHEMA_VERSION {
            violations.push(CommandGovernanceContractViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(CommandGovernanceContractViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_contract_refs(self, &mut violations);
        validate_descriptor_fields(self, &mut violations);
        validate_invocation_session_fields(self, &mut violations);
        validate_result_packet_fields(self, &mut violations);
        validate_lifecycle_dependency_vocabulary(self, &mut violations);
        validate_downgrade_rules(self, &mut violations);
        validate_feature_family_rows(self, &mut violations);
        validate_evidence_export(self, &mut violations);
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("command governance packet serializes"),
        ) {
            violations.push(CommandGovernanceContractViolation::RawMaterialInExport);
        }
        violations
    }

    /// Returns deterministic export-safe JSON.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("command governance packet serializes")
    }

    /// Renders a compact Markdown summary for review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed_families = self
            .feature_family_rows
            .iter()
            .filter(|row| !row.effective_claim.is_stable())
            .count();
        let narrowed_surfaces = self
            .feature_family_rows
            .iter()
            .flat_map(|row| &row.surface_rows)
            .filter(|row| !row.effective_claim.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# Command Governance Contract\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Feature families: {} ({} narrowed below Stable)\n",
            self.feature_family_rows.len(),
            narrowed_families
        ));
        out.push_str(&format!(
            "- Surface rows: {} ({} narrowed below Stable)\n",
            self.feature_family_rows
                .iter()
                .map(|row| row.surface_rows.len())
                .sum::<usize>(),
            narrowed_surfaces
        ));
        out.push_str(&format!(
            "- Descriptor fields: {}\n",
            self.descriptor_fields.len()
        ));
        out.push_str(&format!(
            "- Invocation-session fields: {}\n",
            self.invocation_session_fields.len()
        ));
        out.push_str(&format!(
            "- Result-packet fields: {}\n",
            self.result_packet_fields.len()
        ));
        out.push_str(&format!(
            "- Lifecycle dependency classes: {}\n",
            self.lifecycle_dependency_vocabulary.len()
        ));
        out.push_str(&format!(
            "- Downgrade rules: {}\n",
            self.downgrade_rules.len()
        ));
        out.push_str(&format!(
            "- Evidence id: `{}`\n",
            self.evidence_export.evidence_id
        ));
        out
    }
}

/// Errors emitted when reading the checked support export.
#[derive(Debug)]
pub enum CommandGovernanceContractArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CommandGovernanceContractViolation>),
}

impl fmt::Display for CommandGovernanceContractArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "command governance export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "command governance export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for CommandGovernanceContractArtifactError {}

/// Validation failures emitted by [`CommandGovernanceContractPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandGovernanceContractViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// Canonical contract refs drifted.
    ContractRefsNotCanonical,
    /// Descriptor-field coverage is incomplete or malformed.
    DescriptorFieldCoverageMissing,
    /// Invocation-session field coverage is incomplete or malformed.
    InvocationSessionFieldCoverageMissing,
    /// Result-packet field coverage is incomplete or malformed.
    ResultPacketFieldCoverageMissing,
    /// Lifecycle-dependency vocabulary coverage is incomplete or malformed.
    LifecycleDependencyCoverageMissing,
    /// Downgrade-rule coverage is incomplete or malformed.
    DowngradeRuleCoverageMissing,
    /// Feature-family coverage is incomplete.
    FeatureFamilyCoverageMissing,
    /// A feature-family row is missing required refs or proof fields.
    FeatureFamilyRefsMissing,
    /// A feature-family row is missing required surface coverage.
    SurfaceCoverageMissing,
    /// A surface row references an unknown dependency marker.
    SurfaceDependencyReferenceMissing,
    /// A stable-facing row failed to auto-narrow when proof or freshness dropped.
    StableClaimNotAutoNarrowed,
    /// A narrowed row still permits stable copy.
    StableCopyFlagMismatch,
    /// A dependency marker is present but not disclosed.
    LifecycleDependencyUndisclosed,
    /// A supposedly stable row broke command-authority or preview parity.
    SurfaceAuthorityParityBroken,
    /// Evidence export refs are missing.
    EvidenceExportRefsMissing,
    /// The packet carries raw material outside the export boundary.
    RawMaterialInExport,
}

impl CommandGovernanceContractViolation {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ContractRefsNotCanonical => "contract_refs_not_canonical",
            Self::DescriptorFieldCoverageMissing => "descriptor_field_coverage_missing",
            Self::InvocationSessionFieldCoverageMissing => {
                "invocation_session_field_coverage_missing"
            }
            Self::ResultPacketFieldCoverageMissing => "result_packet_field_coverage_missing",
            Self::LifecycleDependencyCoverageMissing => "lifecycle_dependency_coverage_missing",
            Self::DowngradeRuleCoverageMissing => "downgrade_rule_coverage_missing",
            Self::FeatureFamilyCoverageMissing => "feature_family_coverage_missing",
            Self::FeatureFamilyRefsMissing => "feature_family_refs_missing",
            Self::SurfaceCoverageMissing => "surface_coverage_missing",
            Self::SurfaceDependencyReferenceMissing => "surface_dependency_reference_missing",
            Self::StableClaimNotAutoNarrowed => "stable_claim_not_auto_narrowed",
            Self::StableCopyFlagMismatch => "stable_copy_flag_mismatch",
            Self::LifecycleDependencyUndisclosed => "lifecycle_dependency_undisclosed",
            Self::SurfaceAuthorityParityBroken => "surface_authority_parity_broken",
            Self::EvidenceExportRefsMissing => "evidence_export_refs_missing",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Returns the checked support export.
pub fn current_frozen_command_governance_contract_export(
) -> Result<CommandGovernanceContractPacket, CommandGovernanceContractArtifactError> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/commands/freeze_command_governance_contract/support_export.json"
    );
    let json = std::fs::read_to_string(path).map_err(|error| {
        CommandGovernanceContractArtifactError::SupportExport(serde_json::Error::io(error))
    })?;
    let packet: CommandGovernanceContractPacket = serde_json::from_str(&json)
        .map_err(CommandGovernanceContractArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CommandGovernanceContractArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &CommandGovernanceContractPacket,
    violations: &mut Vec<CommandGovernanceContractViolation>,
) {
    for required in [
        FREEZE_COMMAND_GOVERNANCE_CONTRACT_DOC_REF,
        FREEZE_COMMAND_GOVERNANCE_CONTRACT_SCHEMA_REF,
        FREEZE_COMMAND_GOVERNANCE_DESCRIPTOR_CONTRACT_REF,
        FREEZE_COMMAND_GOVERNANCE_INVOCATION_PARITY_CONTRACT_REF,
        FREEZE_COMMAND_GOVERNANCE_LIFECYCLE_ADR_REF,
        FREEZE_COMMAND_GOVERNANCE_DISABLED_REASON_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(CommandGovernanceContractViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_contract_refs(
    packet: &CommandGovernanceContractPacket,
    violations: &mut Vec<CommandGovernanceContractViolation>,
) {
    if !packet.contract_refs.matches_canonical() {
        violations.push(CommandGovernanceContractViolation::ContractRefsNotCanonical);
    }
}

fn validate_descriptor_fields(
    packet: &CommandGovernanceContractPacket,
    violations: &mut Vec<CommandGovernanceContractViolation>,
) {
    for required in CommandDescriptorFieldClass::required_coverage() {
        if !packet
            .descriptor_fields
            .iter()
            .any(|row| row.field_class == required)
        {
            violations.push(CommandGovernanceContractViolation::DescriptorFieldCoverageMissing);
            break;
        }
    }
    for row in &packet.descriptor_fields {
        if row.field_label.trim().is_empty()
            || row.source_pointer.trim().is_empty()
            || !row.exported
        {
            violations.push(CommandGovernanceContractViolation::DescriptorFieldCoverageMissing);
            break;
        }
    }
}

fn validate_invocation_session_fields(
    packet: &CommandGovernanceContractPacket,
    violations: &mut Vec<CommandGovernanceContractViolation>,
) {
    for required in InvocationSessionFieldClass::required_coverage() {
        if !packet
            .invocation_session_fields
            .iter()
            .any(|row| row.field_class == required)
        {
            violations
                .push(CommandGovernanceContractViolation::InvocationSessionFieldCoverageMissing);
            break;
        }
    }
    for row in &packet.invocation_session_fields {
        if row.field_label.trim().is_empty()
            || row.source_pointer.trim().is_empty()
            || !row.required_on_every_surface
        {
            violations
                .push(CommandGovernanceContractViolation::InvocationSessionFieldCoverageMissing);
            break;
        }
    }
}

fn validate_result_packet_fields(
    packet: &CommandGovernanceContractPacket,
    violations: &mut Vec<CommandGovernanceContractViolation>,
) {
    for required in ResultPacketFieldClass::required_coverage() {
        if !packet
            .result_packet_fields
            .iter()
            .any(|row| row.field_class == required)
        {
            violations.push(CommandGovernanceContractViolation::ResultPacketFieldCoverageMissing);
            break;
        }
    }
    for row in &packet.result_packet_fields {
        if row.field_label.trim().is_empty()
            || row.source_pointer.trim().is_empty()
            || !row.required_on_every_result
        {
            violations.push(CommandGovernanceContractViolation::ResultPacketFieldCoverageMissing);
            break;
        }
    }
}

fn validate_lifecycle_dependency_vocabulary(
    packet: &CommandGovernanceContractPacket,
    violations: &mut Vec<CommandGovernanceContractViolation>,
) {
    for required in LifecycleDependencyClass::required_coverage() {
        if !packet
            .lifecycle_dependency_vocabulary
            .iter()
            .any(|row| row.dependency_class == required)
        {
            violations.push(CommandGovernanceContractViolation::LifecycleDependencyCoverageMissing);
            break;
        }
    }
    for row in &packet.lifecycle_dependency_vocabulary {
        if row.disclosure_label.trim().is_empty() || row.source_pointer.trim().is_empty() {
            violations.push(CommandGovernanceContractViolation::LifecycleDependencyCoverageMissing);
            break;
        }
        if !row.narrows_stable_claim {
            violations.push(CommandGovernanceContractViolation::LifecycleDependencyCoverageMissing);
            break;
        }
    }
}

fn validate_downgrade_rules(
    packet: &CommandGovernanceContractPacket,
    violations: &mut Vec<CommandGovernanceContractViolation>,
) {
    for required in DowngradeTriggerClass::required_coverage() {
        if !packet
            .downgrade_rules
            .iter()
            .any(|row| row.trigger == required)
        {
            violations.push(CommandGovernanceContractViolation::DowngradeRuleCoverageMissing);
            break;
        }
    }
    for row in &packet.downgrade_rules {
        if row.summary.trim().is_empty() || !row.stable_copy_forbidden {
            violations.push(CommandGovernanceContractViolation::DowngradeRuleCoverageMissing);
            break;
        }
        for required in PublicationConsumerSurfaceClass::required_coverage() {
            if !row
                .consumer_surfaces
                .iter()
                .any(|surface| *surface == required)
            {
                violations.push(CommandGovernanceContractViolation::DowngradeRuleCoverageMissing);
                return;
            }
        }
        if row.narrowed_to.is_stable() {
            violations.push(CommandGovernanceContractViolation::DowngradeRuleCoverageMissing);
            break;
        }
    }
}

fn validate_feature_family_rows(
    packet: &CommandGovernanceContractPacket,
    violations: &mut Vec<CommandGovernanceContractViolation>,
) {
    for required in FeatureFamilyClass::required_coverage() {
        if !packet
            .feature_family_rows
            .iter()
            .any(|row| row.family_class == required)
        {
            violations.push(CommandGovernanceContractViolation::FeatureFamilyCoverageMissing);
            break;
        }
    }

    for row in &packet.feature_family_rows {
        validate_feature_family_row(packet, row, violations);
        if !violations.is_empty() {
            return;
        }
    }
}

fn validate_feature_family_row(
    packet: &CommandGovernanceContractPacket,
    row: &FeatureFamilyGovernanceRow,
    violations: &mut Vec<CommandGovernanceContractViolation>,
) {
    if row.display_label.trim().is_empty()
        || row.descriptor_proof_ref.trim().is_empty()
        || row.invocation_session_proof_ref.trim().is_empty()
        || row.result_packet_proof_ref.trim().is_empty()
        || row.rollout_owner_ref.trim().is_empty()
        || row.evidence_packet_ref.trim().is_empty()
    {
        violations.push(CommandGovernanceContractViolation::FeatureFamilyRefsMissing);
        return;
    }

    for dependency in &row.lifecycle_dependencies {
        if dependency.marker_ref.trim().is_empty()
            || dependency.dependency_ref.trim().is_empty()
            || dependency.owner_ref.trim().is_empty()
            || dependency.disclosure_ref.trim().is_empty()
        {
            violations.push(CommandGovernanceContractViolation::FeatureFamilyRefsMissing);
            return;
        }
        if !packet
            .lifecycle_dependency_vocabulary
            .iter()
            .any(|vocab| vocab.dependency_class == dependency.dependency_class)
        {
            violations.push(CommandGovernanceContractViolation::LifecycleDependencyCoverageMissing);
            return;
        }
    }

    for required in CommandGovernanceSurfaceClass::required_coverage() {
        if !row
            .surface_rows
            .iter()
            .any(|surface| surface.surface_class == required)
        {
            violations.push(CommandGovernanceContractViolation::SurfaceCoverageMissing);
            return;
        }
    }

    if row.published_stable_copy_allowed != row.effective_claim.is_stable() {
        violations.push(CommandGovernanceContractViolation::StableCopyFlagMismatch);
        return;
    }

    let requires_narrowing = !row.descriptor_contract_current
        || !row.invocation_session_contract_current
        || !row.result_packet_contract_current
        || !row.disabled_reason_vocabulary_current
        || !row.preview_semantics_current
        || row.evidence_freshness.forces_narrowing()
        || row
            .lifecycle_dependencies
            .iter()
            .any(|dependency| dependency.narrowing_required);

    if requires_narrowing && row.effective_claim.is_stable() {
        violations.push(CommandGovernanceContractViolation::StableClaimNotAutoNarrowed);
        return;
    }
    if row.effective_claim.is_stable() && !row.stable_ready() {
        violations.push(CommandGovernanceContractViolation::SurfaceAuthorityParityBroken);
        return;
    }

    for surface in &row.surface_rows {
        validate_surface_row(row, surface, violations);
        if !violations.is_empty() {
            return;
        }
    }
}

fn validate_surface_row(
    family_row: &FeatureFamilyGovernanceRow,
    surface: &SurfaceGovernanceRow,
    violations: &mut Vec<CommandGovernanceContractViolation>,
) {
    if surface.published_stable_copy_allowed != surface.effective_claim.is_stable() {
        violations.push(CommandGovernanceContractViolation::StableCopyFlagMismatch);
        return;
    }

    for marker_ref in &surface.dependency_marker_refs {
        if !family_row
            .lifecycle_dependencies
            .iter()
            .any(|dependency| dependency.marker_ref == *marker_ref)
        {
            violations.push(CommandGovernanceContractViolation::SurfaceDependencyReferenceMissing);
            return;
        }
    }

    if !surface.dependency_marker_refs.is_empty() && !surface.lifecycle_dependency_disclosed {
        violations.push(CommandGovernanceContractViolation::LifecycleDependencyUndisclosed);
        return;
    }

    let requires_narrowing = !surface.descriptor_contract_current
        || !surface.invocation_session_contract_current
        || !surface.result_packet_contract_current
        || !surface.authority_parity_preserved
        || !surface.preview_parity_preserved
        || !surface.approval_parity_preserved
        || !surface.disabled_reason_parity_preserved
        || !surface.route_truth_disclosed
        || surface.evidence_freshness.forces_narrowing()
        || !surface.dependency_marker_refs.is_empty();

    if requires_narrowing && surface.effective_claim.is_stable() {
        violations.push(CommandGovernanceContractViolation::StableClaimNotAutoNarrowed);
        return;
    }

    if !surface.authority_parity_preserved
        || !surface.preview_parity_preserved
        || !surface.approval_parity_preserved
        || !surface.disabled_reason_parity_preserved
        || !surface.route_truth_disclosed
    {
        if surface.effective_claim.is_stable() {
            violations.push(CommandGovernanceContractViolation::SurfaceAuthorityParityBroken);
            return;
        }
        if !surface.claimed_stable && surface.downgrade_reasons.is_empty() {
            violations.push(CommandGovernanceContractViolation::SurfaceAuthorityParityBroken);
            return;
        }
    }

    if surface.effective_claim.is_stable() && !surface.stable_ready() {
        violations.push(CommandGovernanceContractViolation::SurfaceAuthorityParityBroken);
    }
}

fn validate_evidence_export(
    packet: &CommandGovernanceContractPacket,
    violations: &mut Vec<CommandGovernanceContractViolation>,
) {
    let export = &packet.evidence_export;
    if export.evidence_id.trim().is_empty()
        || export.json_export_ref.trim().is_empty()
        || export.markdown_summary_ref.trim().is_empty()
        || export.help_projection_ref.trim().is_empty()
        || export.release_projection_ref.trim().is_empty()
        || export.support_projection_ref.trim().is_empty()
    {
        violations.push(CommandGovernanceContractViolation::EvidenceExportRefsMissing);
    }
}

fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_material(text),
        serde_json::Value::Array(values) => values.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

fn contains_forbidden_material(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("bearer ")
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("oauth_token")
        || lower.contains("private_key")
        || lower.contains("signing_key")
        || lower.contains("raw_prompt")
        || lower.contains("raw_body")
        || lower.contains("billing-account")
}

#[cfg(test)]
mod tests;
