//! Marketplace/package install-review alpha contract and projections.
//!
//! This module consumes the existing manifest/effective-permission,
//! extension-review, provider, runtime-boundary, and install-topology records
//! to build one canonical native review sheet for the first marketplace or
//! package lane. Hosted marketplace rows and extension webviews are modeled as
//! narrower consumers of that native sheet; they may display owner/origin,
//! scope, network, service-boundary, compatibility, permission, activation
//! budget, and install-topology truth, but they may not approve an install or
//! enable action by themselves.

use serde::{Deserialize, Serialize};

use aureline_auth::CurrentDeploymentBoundaryClass;
use aureline_content_safety::{
    project_content_integrity_warnings, ContentIntegritySurfaceKind, ContentIntegrityWarningRecord,
};
use aureline_install::{
    InstallModeClass, InstallTopologyRow, InstallTopologyTruthFingerprint, TopologySurfaceClass,
};
use aureline_provider::{ActorScope, ProviderSourceClass};
use aureline_runtime::{HostBoundaryCueClass, ReachabilityState, ScopeClass};
use aureline_support::capabilities::{
    current_capability_lifecycle_registry, DependencyMarker, EffectOnParent, MarkerKind,
};

use crate::manifest_baseline::{
    DeclaredVsEffectiveDiffEntry, EffectivePermissionBaselineRecord, EffectivePermissionDiffClass,
    HostContractFamilyClass, ManifestOriginSourceClass, PublisherTrustTierClass, RedactionClass,
    SummaryFreshnessClass,
};
use crate::review_alpha::{
    ExtensionReviewAlphaPacketRecord, ReviewDecisionClass, ReviewDisclosureClass,
};

#[cfg(test)]
mod tests;

/// Record-kind tag carried on serialized [`InstallReviewAlphaPacketRecord`] payloads.
pub const INSTALL_REVIEW_ALPHA_PACKET_RECORD_KIND: &str = "install_review_alpha_packet_record";

/// Record-kind tag carried on serialized [`InstallReviewAlphaProjectionRecord`] payloads.
pub const INSTALL_REVIEW_ALPHA_PROJECTION_RECORD_KIND: &str =
    "install_review_alpha_projection_record";

/// Schema version for install-review alpha payloads.
pub const INSTALL_REVIEW_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Mutation requested by a marketplace/package review lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallReviewActionClass {
    /// Install a package that is not currently enabled or installed.
    Install,
    /// Enable an installed package for the current workspace, profile, or org scope.
    Enable,
    /// Update an installed package to a new reviewed version.
    Update,
}

/// Consumer surface asking for an install-review projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallReviewSurfaceClass {
    /// Product-owned native install-review sheet.
    NativeReviewSheet,
    /// Marketplace/package result row.
    MarketplacePackageLane,
    /// Package detail page.
    PackageDetail,
    /// CLI or headless review projection.
    CliHeadless,
    /// Support export projection.
    SupportExport,
    /// Hosted webview or provider-owned catalog lane.
    HostedWebviewLane,
}

/// Decision emitted by the install-review alpha evaluator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallReviewDecisionClass {
    /// The native review sheet may commit the mutation after review.
    AdmitAfterNativeReview,
    /// A user acknowledgement or explicit native review transition is still required.
    AwaitingUserReview,
    /// An administrator or mirror operator must act before mutation.
    AwaitingAdminReview,
    /// The mutation is refused.
    Denied,
}

/// Typed reason paired with [`InstallReviewDecisionClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallReviewDecisionReasonClass {
    /// Native review rendered all required truth and may commit.
    AdmittedAfterNativeReview,
    /// A required disclosure was not rendered.
    MissingRequiredDisclosure,
    /// Owner, origin, scope, network, or service-boundary truth is absent.
    OwnerOriginScopeNetworkBoundaryMissing,
    /// Hosted or webview content attempted to approve a native mutation.
    HostedConsumerCannotApprove,
    /// A canonical product-owned native review packet is missing.
    NativeReviewPacketRequired,
    /// Upstream extension review denied the mutation.
    UpstreamReviewDenied,
    /// Upstream extension review is still waiting on user or admin review.
    UpstreamReviewPending,
    /// Effective permission truth blocked a widening attempt.
    EffectivePermissionWideningAttempted,
    /// Compatibility evidence is missing, stale, unverified, or pending reverify.
    CompatibilityEvidenceMissing,
    /// Compatibility evidence blocks the mutation.
    CompatibilityMismatch,
    /// Compatibility is limited or bridge-backed and needs native acknowledgement.
    CompatibilityLimitedRequiresReview,
    /// The rendered compatibility label overclaims a dependency-marker-narrowed capability.
    CompatibilityLabelClaimRefused,
    /// Activation-budget evidence is missing or not strong enough for the claim.
    ActivationBudgetEvidenceMissing,
    /// Runtime budget evidence says the package is quarantined or over budget.
    ActivationBudgetBlocksMutation,
    /// Install-topology truth is not available on the install-review surface.
    InstallTopologyMissing,
}

/// Content source class for marketplace/package review lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallReviewContentSourceClass {
    /// Product-owned or foundation-owned content.
    FirstParty,
    /// Content delivered by an approved mirror or offline bundle.
    Mirrored,
    /// Community-published content.
    Community,
    /// Content scoped to a signed-in account.
    AccountOwned,
    /// Content owned by an external provider or hosted catalog.
    ProviderOwned,
}

/// Authority relationship between a consumer surface and the native review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeReviewAuthorityClass {
    /// Product-owned native sheet is canonical and can commit when admitted.
    ProductNativeReviewSheet,
    /// Hosted marketplace content is a read-only consumer of the native sheet.
    HostedMarketplaceReadOnlyConsumer,
    /// Extension webview content is a read-only consumer of the native sheet.
    ExtensionWebviewReadOnlyConsumer,
    /// Provider-hosted content is a read-only consumer of the native sheet.
    ProviderHostedReadOnlyConsumer,
}

/// Disclosure class that must remain visible before mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallReviewDisclosureClass {
    /// Owner and origin truth is rendered.
    OwnerOrigin,
    /// Current profile, org, workspace, or policy scope is rendered.
    Scope,
    /// Network reachability or policy-block state is rendered.
    NetworkState,
    /// Local, managed, mirror, provider, or hosted service boundary is rendered.
    ServiceBoundary,
    /// The native review packet and authority relationship are rendered.
    NativeCanonicalReview,
    /// Declared-vs-effective permission delta is rendered.
    PermissionDelta,
    /// Compatibility labels and evidence are rendered.
    CompatibilityLabels,
    /// Activation-budget class, trigger, and evidence are rendered.
    ActivationBudget,
    /// Install-topology row truth is rendered.
    InstallTopology,
    /// Publisher trust tier and manifest origin vocabulary are rendered.
    PublisherTrust,
}

/// Compatibility claim class mirrored from the discovery and registry schemas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityClaimClass {
    /// Package is compatible on every declared target.
    CompatibleOnAllDeclaredTargets,
    /// Package is compatible on a declared subset only.
    CompatibleOnSubsetOfDeclaredTargets,
    /// Package requires a compatibility bridge.
    CompatibilityBridgeRequired,
    /// Compatibility is unknown pending re-verification.
    CompatibilityUnknownPendingReverification,
    /// Compatibility is blocked by policy or the current target.
    IncompatibleBlockedOnPolicy,
}

/// Runtime-cost class mirrored from the discovery and runtime-budget contracts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeCostClass {
    /// Runtime cost is low and nominal.
    RuntimeCostLowNominal,
    /// Runtime cost is nominal.
    RuntimeCostNominal,
    /// Runtime cost is elevated because warm activation or idle polling breached budget.
    RuntimeCostElevatedWarmOrIdlePollingBreach,
    /// Runtime cost is unknown because evidence is pending.
    RuntimeCostUnknownPendingEvidence,
    /// Runtime cost is quarantined due to crash-loop or egress breach.
    RuntimeCostQuarantinedUnderCrashLoopOrEgressBreach,
}

/// Runtime-cost evidence class mirrored from the discovery schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeCostEvidenceClass {
    /// Activation-evidence packet is present.
    ActivationEvidencePacketPresent,
    /// Activation-evidence packet is absent until first session.
    ActivationEvidencePacketAbsentPendingFirstSession,
    /// Benchmark archive is present.
    BenchmarkArchivePresent,
    /// Benchmark archive is absent.
    BenchmarkArchiveAbsent,
    /// Evidence is self-reported only and not verified by the host.
    SelfReportedOnlyUnverified,
}

/// Bridge-state class mirrored from the discovery schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BridgeStateClass {
    /// No bridge is required on the current profile.
    NoBridgeRequiredNativeMatch,
    /// Compatibility bridge profile is required.
    BridgeRequiredCompatibilityBridgeProfile,
    /// Bridge is limited to a capability-world subset.
    BridgeRequiredCapabilityWorldSubsetOnly,
    /// Bridge is limited to a host-contract-family subset.
    BridgeRequiredHostContractFamilySubsetOnly,
    /// Bridge state is unknown pending re-verification.
    BridgeUnknownPendingReverification,
    /// Bridge is unsupported or blocked by policy.
    BridgeUnsupportedBlockedOnPolicy,
}

/// User-facing compatibility label rendered before an install or enable commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityLabel {
    /// The compatibility basis has not been resolved and must render as unknown.
    Unknown,
    /// Native capability exactly matches the current target and capability basis.
    Exact,
    /// The capability is translated through a governed bridge or adapter.
    Translated,
    /// Only part of the declared capability set is supported on this target.
    Partial,
    /// A shim emulates part of the expected behavior with known caveats.
    Shimmed,
    /// No supported path exists for the current target.
    Unsupported,
}

impl Default for CompatibilityLabel {
    fn default() -> Self {
        Self::Unknown
    }
}

impl CompatibilityLabel {
    /// Returns the serialized token for the label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Exact => "exact",
            Self::Translated => "translated",
            Self::Partial => "partial",
            Self::Shimmed => "shimmed",
            Self::Unsupported => "unsupported",
        }
    }

    /// Returns the short display label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Unknown => "Unknown",
            Self::Exact => "Exact",
            Self::Translated => "Translated",
            Self::Partial => "Partial",
            Self::Shimmed => "Shimmed",
            Self::Unsupported => "Unsupported",
        }
    }

    const fn needs_native_ack(self) -> bool {
        matches!(self, Self::Translated | Self::Partial | Self::Shimmed)
    }
}

/// Structured activation budget rendered before an install or enable commit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivationBudget {
    /// CPU ceiling or budget class for activation and background work.
    #[serde(default = "unknown_value")]
    pub cpu: String,
    /// Memory ceiling or budget class for resident extension work.
    #[serde(default = "unknown_value")]
    pub memory: String,
    /// Startup-cost ceiling for cold activation or shell startup participation.
    #[serde(default = "unknown_value")]
    pub startup_cost_ceiling: String,
    /// Feature gates that must be explicitly opted into before activation widens.
    #[serde(default = "unknown_feature_gates")]
    pub opt_in_feature_gates: Vec<String>,
}

impl Default for ActivationBudget {
    fn default() -> Self {
        Self::unknown()
    }
}

impl ActivationBudget {
    /// Returns an explicit unknown budget record.
    pub fn unknown() -> Self {
        Self {
            cpu: unknown_value(),
            memory: unknown_value(),
            startup_cost_ceiling: unknown_value(),
            opt_in_feature_gates: unknown_feature_gates(),
        }
    }

    /// Returns the feature gates, using `unknown` when the list is empty.
    pub fn opt_in_feature_gates_or_unknown(&self) -> Vec<&str> {
        if self.opt_in_feature_gates.is_empty() {
            return vec!["unknown"];
        }
        self.opt_in_feature_gates
            .iter()
            .map(String::as_str)
            .collect()
    }

    fn has_unknown_axis(&self) -> bool {
        value_unknown(&self.cpu)
            || value_unknown(&self.memory)
            || value_unknown(&self.startup_cost_ceiling)
            || self.opt_in_feature_gates.is_empty()
            || self
                .opt_in_feature_gates
                .iter()
                .any(|gate| value_unknown(gate))
    }
}

fn unknown_value() -> String {
    "unknown".to_string()
}

fn unknown_feature_gates() -> Vec<String> {
    vec![unknown_value()]
}

fn value_unknown(value: &str) -> bool {
    let value = value.trim();
    value.is_empty() || value.eq_ignore_ascii_case("unknown")
}

/// Action offered by an install-review projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallReviewActionOfferClass {
    /// Approve install from the native sheet.
    ApproveInstall,
    /// Enable from the native sheet.
    EnableExtension,
    /// Open the canonical native review sheet.
    OpenNativeReviewSheet,
    /// Open the permission-delta details.
    OpenPermissionDelta,
    /// Open compatibility evidence.
    OpenCompatibilityEvidence,
    /// Open activation-budget evidence.
    OpenActivationBudgetEvidence,
    /// Open install-topology truth.
    OpenInstallTopology,
    /// Export a metadata-safe support packet.
    ExportSupportPacket,
    /// Ask an admin, mirror operator, or policy owner to review.
    ConsultAdmin,
    /// Keep the current package disabled or pinned.
    KeepDisabledOrPinned,
}

/// Owner, origin, scope, network, and boundary truth rendered by the review lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallReviewBoundaryTruth {
    /// Content source class shown on the marketplace/package lane.
    pub content_source_class: InstallReviewContentSourceClass,
    /// Manifest origin/source class from the manifest baseline.
    pub manifest_origin_source_class: ManifestOriginSourceClass,
    /// Publisher trust tier from the manifest baseline.
    pub publisher_trust_tier_class: PublisherTrustTierClass,
    /// Provider source class when content is provider- or account-owned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_source_class: Option<ProviderSourceClass>,
    /// Provider actor scope when provider-owned content participates in the decision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_actor_scope: Option<ActorScope>,
    /// Current profile scope reference.
    pub profile_scope_ref: String,
    /// Current organization or tenant scope reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_scope_ref: Option<String>,
    /// Workspace or workset scope class.
    pub workspace_scope_class: ScopeClass,
    /// Network reachability or policy-block state.
    pub network_reachability_state: ReachabilityState,
    /// Current deployment or service boundary.
    pub service_boundary_class: CurrentDeploymentBoundaryClass,
    /// Runtime/host boundary cue for the content.
    pub host_boundary_cue_class: HostBoundaryCueClass,
    /// Native review packet that owns the approval decision.
    pub canonical_native_review_ref: String,
    /// Authority relationship between this lane and native review.
    pub canonical_review_authority_class: NativeReviewAuthorityClass,
    /// Export-safe owner/origin summary rendered by UI, CLI, and support consumers.
    pub owner_origin_summary: String,
}

/// Compatibility label block rendered by marketplace/package review lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityLabelBlock {
    /// Compatibility label rendered directly on the install-review surface.
    #[serde(default)]
    pub compatibility_label: CompatibilityLabel,
    /// Compatibility class from the marketplace discovery vocabulary.
    pub compatibility_claim_class: CompatibilityClaimClass,
    /// Bridge state from the marketplace discovery vocabulary.
    pub bridge_state_class: BridgeStateClass,
    /// Freshness class for compatibility evidence.
    pub evidence_freshness_class: SummaryFreshnessClass,
    /// Evidence refs justifying the compatibility labels.
    pub evidence_refs: Vec<String>,
    /// Aureline version range the label covers.
    pub aureline_version_range: String,
    /// Platform, OS, runtime, or deployment-profile refs covered by the evidence.
    pub platform_scope_refs: Vec<String>,
    /// Unsupported or degraded reason refs.
    pub unsupported_reason_refs: Vec<String>,
}

/// Activation-budget disclosure rendered by marketplace/package review lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivationBudgetDisclosure {
    /// Structured CPU, memory, startup-cost, and feature-gate budget.
    #[serde(default)]
    pub activation_budget: ActivationBudget,
    /// Runtime-cost class from the marketplace discovery vocabulary.
    pub runtime_cost_class: RuntimeCostClass,
    /// Evidence class backing the runtime-cost claim.
    pub runtime_cost_evidence_class: RuntimeCostEvidenceClass,
    /// Budget class from `artifacts/extensions/runtime_budget_rows.yaml`.
    pub runtime_budget_class: String,
    /// Activation trigger refs such as startup, language-open, or command invocation.
    pub activation_trigger_refs: Vec<String>,
    /// Runtime-budget axis refs from `artifacts/extensions/runtime_budget_rows.yaml`.
    pub budget_axis_refs: Vec<String>,
    /// Host contract family from the manifest baseline.
    pub host_contract_family_class: HostContractFamilyClass,
    /// Restart, reload, reattach, or no-interruption implication.
    pub restart_or_reattach_implication: String,
    /// Evidence refs for activation budget and observed runtime cost.
    pub evidence_refs: Vec<String>,
}

/// Inputs supplied by a marketplace/package lane to build install-review truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallReviewAlphaInput {
    /// Stable review id.
    pub review_id: String,
    /// Requested action.
    pub action_class: InstallReviewActionClass,
    /// Extension or package subject ref.
    pub subject_ref: String,
    /// Consumer surface building the review packet.
    pub consumer_surface_class: InstallReviewSurfaceClass,
    /// True when a non-native consumer tried to approve or enable directly.
    pub consumer_attempts_native_approval: bool,
    /// Disclosure classes rendered before asking for a decision.
    pub rendered_disclosures: Vec<InstallReviewDisclosureClass>,
    /// Review event refs emitted while building the packet.
    pub review_event_refs: Vec<String>,
}

/// Install-review packet emitted by the alpha marketplace/package lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallReviewAlphaPacketRecord {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this alpha record.
    pub install_review_alpha_schema_version: u32,
    /// Stable review id.
    pub review_id: String,
    /// Requested action.
    pub action_class: InstallReviewActionClass,
    /// Extension or package subject ref.
    pub subject_ref: String,
    /// Consumer surface that built the packet.
    pub consumer_surface_class: InstallReviewSurfaceClass,
    /// Whether the consumer attempted to approve directly.
    pub consumer_attempts_native_approval: bool,
    /// Canonical product-owned native review ref.
    pub canonical_native_review_ref: String,
    /// Upstream extension review packet ref.
    pub extension_review_alpha_ref: String,
    /// Effective-permission summary ref.
    pub effective_permission_summary_ref: String,
    /// Install-topology row ref consumed from `aureline-install`.
    pub install_topology_row_ref: String,
    /// Install-topology truth fingerprint consumed from `aureline-install`.
    pub install_topology_truth_fingerprint: InstallTopologyTruthFingerprint,
    /// Capability lifecycle rows consumed from the governance registry.
    pub capability_lifecycle_row_refs: Vec<String>,
    /// Boundary truth rendered by the review lane.
    pub boundary_truth: InstallReviewBoundaryTruth,
    /// Compatibility labels rendered by the review lane.
    pub compatibility: CompatibilityLabelBlock,
    /// Activation-budget disclosure rendered by the review lane.
    pub activation_budget: ActivationBudgetDisclosure,
    /// Permission deltas rendered by the review lane.
    pub permission_delta_entries: Vec<DeclaredVsEffectiveDiffEntry>,
    /// Number of widening attempts blocked by effective-permission truth.
    pub widening_attempted_blocked_count: u32,
    /// Review-alpha disclosure classes from the upstream packet.
    pub upstream_required_disclosures: Vec<ReviewDisclosureClass>,
    /// Disclosure classes required before marketplace/package mutation.
    pub required_disclosures: Vec<InstallReviewDisclosureClass>,
    /// Disclosure classes rendered before decision.
    pub rendered_disclosures: Vec<InstallReviewDisclosureClass>,
    /// True when this packet may commit the requested mutation.
    pub mutation_allowed: bool,
    /// Decision emitted by the install-review flow.
    pub decision_class: InstallReviewDecisionClass,
    /// Typed reason paired with the decision.
    pub decision_reason_class: InstallReviewDecisionReasonClass,
    /// Export-safe decision summary.
    pub decision_summary: String,
    /// Review event refs emitted while building the packet.
    pub review_event_refs: Vec<String>,
    /// Decision timestamp.
    pub decided_at: String,
    /// Redaction class for emitted records.
    pub redaction_class: RedactionClass,
    /// Shared content-integrity warnings emitted for package review text.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub content_integrity_warnings: Vec<ContentIntegrityWarningRecord>,
}

/// First consumer projection for native, marketplace, CLI, webview, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallReviewAlphaProjectionRecord {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this alpha record.
    pub install_review_alpha_schema_version: u32,
    /// Stable projection id.
    pub projection_id: String,
    /// Source review ref.
    pub review_ref: String,
    /// Consumer surface rendered by this projection.
    pub surface_class: InstallReviewSurfaceClass,
    /// Content source rendered on the lane.
    pub content_source_class: InstallReviewContentSourceClass,
    /// Manifest origin rendered on the lane.
    pub manifest_origin_source_class: ManifestOriginSourceClass,
    /// Publisher trust tier rendered on the lane.
    pub publisher_trust_tier_class: PublisherTrustTierClass,
    /// Current profile scope ref.
    pub profile_scope_ref: String,
    /// Current org or tenant scope ref.
    pub org_scope_ref: Option<String>,
    /// Workspace or workset scope class.
    pub workspace_scope_class: ScopeClass,
    /// Network reachability state.
    pub network_reachability_state: ReachabilityState,
    /// Service boundary class.
    pub service_boundary_class: CurrentDeploymentBoundaryClass,
    /// Host boundary cue class.
    pub host_boundary_cue_class: HostBoundaryCueClass,
    /// Authority relationship to native review.
    pub canonical_review_authority_class: NativeReviewAuthorityClass,
    /// Canonical product-owned native review ref.
    pub canonical_native_review_ref: String,
    /// Compatibility claim rendered on the lane.
    pub compatibility_claim_class: CompatibilityClaimClass,
    /// Bridge state rendered on the lane.
    pub bridge_state_class: BridgeStateClass,
    /// Compatibility label rendered on the lane.
    pub compatibility_label: CompatibilityLabel,
    /// Compatibility label token rendered on the lane.
    pub compatibility_label_token: String,
    /// Runtime-cost class rendered on the lane.
    pub runtime_cost_class: RuntimeCostClass,
    /// Runtime-cost evidence class rendered on the lane.
    pub runtime_cost_evidence_class: RuntimeCostEvidenceClass,
    /// Structured activation-budget fields rendered on the lane.
    pub activation_budget: ActivationBudget,
    /// Number of rendered permission-delta entries.
    pub permission_delta_count: usize,
    /// Number of widening attempts blocked by effective-permission truth.
    pub widening_attempted_blocked_count: u32,
    /// Install-topology row rendered on the lane.
    pub install_topology_row_ref: String,
    /// Install mode rendered on the lane.
    pub install_mode_class: InstallModeClass,
    /// Whether the requested mutation is blocked.
    pub blocked_mutation: bool,
    /// Actions made available by the consumer.
    pub offered_actions: Vec<InstallReviewActionOfferClass>,
    /// Export-safe summary for UI, CLI, and support consumers.
    pub export_safe_summary: String,
    /// Redaction class for emitted records.
    pub redaction_class: RedactionClass,
    /// Shared content-integrity warnings carried into this projection.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub content_integrity_warnings: Vec<ContentIntegrityWarningRecord>,
}

/// Typed validation finding emitted by install-review alpha validators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallReviewAlphaFinding {
    /// Stable validation check id.
    pub check_id: String,
    /// Human-readable validation message.
    pub message: String,
}

impl InstallReviewAlphaFinding {
    fn new(check_id: &str, message: impl Into<String>) -> Self {
        Self {
            check_id: check_id.to_string(),
            message: message.into(),
        }
    }
}

/// Borrowed evaluation context for [`evaluate_install_review_alpha`].
pub struct InstallReviewAlphaEvaluation<'a> {
    /// Inputs supplied by the marketplace/package lane.
    pub input: InstallReviewAlphaInput,
    /// Upstream extension-review packet.
    pub extension_review: &'a ExtensionReviewAlphaPacketRecord,
    /// Effective-permission summary consumed from the manifest baseline.
    pub effective_permission: &'a EffectivePermissionBaselineRecord,
    /// Owner, origin, scope, network, and boundary truth rendered by the lane.
    pub boundary_truth: InstallReviewBoundaryTruth,
    /// Compatibility labels rendered by the lane.
    pub compatibility: CompatibilityLabelBlock,
    /// Activation-budget disclosure rendered by the lane.
    pub activation_budget: ActivationBudgetDisclosure,
    /// Install-topology row consumed from `aureline-install`.
    pub install_topology_row: &'a InstallTopologyRow,
    /// Decision timestamp.
    pub decided_at: &'a str,
}

/// Evaluate the canonical install-review alpha packet.
///
/// The evaluator fails closed when required disclosures are absent, a hosted
/// surface tries to approve natively, compatibility evidence is missing, or
/// effective-permission truth records a blocked widening attempt.
pub fn evaluate_install_review_alpha(
    evaluation: InstallReviewAlphaEvaluation<'_>,
) -> InstallReviewAlphaPacketRecord {
    let InstallReviewAlphaEvaluation {
        input,
        extension_review,
        effective_permission,
        boundary_truth,
        compatibility,
        activation_budget,
        install_topology_row,
        decided_at,
    } = evaluation;

    let required_disclosures = required_disclosures_for(input.action_class);
    let missing_disclosure =
        first_missing_disclosure(&required_disclosures, &input.rendered_disclosures);
    let content_integrity_warnings = project_install_review_alpha_content_integrity(
        &input.review_id,
        &input.subject_ref,
        &install_review_alpha_content_for_warnings(
            &input.subject_ref,
            &boundary_truth.owner_origin_summary,
        ),
    );

    let (decision_class, decision_reason_class, decision_summary) = if missing_disclosure.is_some()
    {
        (
            InstallReviewDecisionClass::Denied,
            InstallReviewDecisionReasonClass::MissingRequiredDisclosure,
            "Denied: the consumer did not render every required install-review disclosure."
                .to_string(),
        )
    } else if let Some(summary) = boundary_truth_missing(&boundary_truth) {
        (
            InstallReviewDecisionClass::Denied,
            InstallReviewDecisionReasonClass::OwnerOriginScopeNetworkBoundaryMissing,
            summary,
        )
    } else if input.consumer_attempts_native_approval
        && input.consumer_surface_class != InstallReviewSurfaceClass::NativeReviewSheet
    {
        (
            InstallReviewDecisionClass::Denied,
            InstallReviewDecisionReasonClass::HostedConsumerCannotApprove,
            "Denied: hosted or webview consumers may open the native review sheet but may not approve the mutation.".to_string(),
        )
    } else if boundary_truth.canonical_native_review_ref.trim().is_empty() {
        (
            InstallReviewDecisionClass::Denied,
            InstallReviewDecisionReasonClass::NativeReviewPacketRequired,
            "Denied: install review requires a canonical product-owned native review packet."
                .to_string(),
        )
    } else if !install_topology_row
        .surface_claims
        .contains(&TopologySurfaceClass::InstallReview)
    {
        (
            InstallReviewDecisionClass::Denied,
            InstallReviewDecisionReasonClass::InstallTopologyMissing,
            "Denied: install-topology truth is not claimed for the install-review surface."
                .to_string(),
        )
    } else if matches!(extension_review.decision_class, ReviewDecisionClass::Denied) {
        (
            InstallReviewDecisionClass::Denied,
            InstallReviewDecisionReasonClass::UpstreamReviewDenied,
            "Denied: upstream extension review packet denied the mutation.".to_string(),
        )
    } else if matches!(
        extension_review.decision_class,
        ReviewDecisionClass::AwaitingAdminReview
    ) {
        (
            InstallReviewDecisionClass::AwaitingAdminReview,
            InstallReviewDecisionReasonClass::UpstreamReviewPending,
            "Awaiting admin review: upstream extension review has not admitted the mutation."
                .to_string(),
        )
    } else if matches!(
        extension_review.decision_class,
        ReviewDecisionClass::AwaitingUserReview
    ) {
        (
            InstallReviewDecisionClass::AwaitingUserReview,
            InstallReviewDecisionReasonClass::UpstreamReviewPending,
            "Awaiting user review: upstream extension review has not admitted the mutation."
                .to_string(),
        )
    } else if effective_permission.widening_attempted_blocked_count > 0
        || effective_permission
            .declared_vs_effective_diff
            .iter()
            .any(|entry| {
                matches!(
                    entry.diff_class,
                    EffectivePermissionDiffClass::WideningAttemptedBlocked
                )
            })
    {
        (
            InstallReviewDecisionClass::Denied,
            InstallReviewDecisionReasonClass::EffectivePermissionWideningAttempted,
            "Denied: effective-permission truth blocked an authority-widening attempt.".to_string(),
        )
    } else if compatibility_evidence_missing(&compatibility) {
        (
            InstallReviewDecisionClass::Denied,
            InstallReviewDecisionReasonClass::CompatibilityEvidenceMissing,
            "Denied: compatibility label is missing, stale, unverified, or pending reverify."
                .to_string(),
        )
    } else if matches!(
        compatibility.compatibility_claim_class,
        CompatibilityClaimClass::IncompatibleBlockedOnPolicy
    ) || matches!(
        compatibility.bridge_state_class,
        BridgeStateClass::BridgeUnsupportedBlockedOnPolicy
    ) || matches!(
        compatibility.compatibility_label,
        CompatibilityLabel::Unsupported
    ) {
        (
            InstallReviewDecisionClass::Denied,
            InstallReviewDecisionReasonClass::CompatibilityMismatch,
            "Denied: compatibility evidence blocks this package on the current target.".to_string(),
        )
    } else if let Some(summary) = exact_label_conflicts_with_partial_marker(
        compatibility.compatibility_label,
        &extension_review.capability_lifecycle_row_refs,
    ) {
        (
            InstallReviewDecisionClass::Denied,
            InstallReviewDecisionReasonClass::CompatibilityLabelClaimRefused,
            summary,
        )
    } else if activation_budget_blocks(&activation_budget) {
        (
            InstallReviewDecisionClass::Denied,
            InstallReviewDecisionReasonClass::ActivationBudgetBlocksMutation,
            "Denied: runtime-budget evidence says the package is quarantined or over budget."
                .to_string(),
        )
    } else if activation_budget_evidence_missing(&activation_budget) {
        (
            InstallReviewDecisionClass::AwaitingUserReview,
            InstallReviewDecisionReasonClass::ActivationBudgetEvidenceMissing,
            "Awaiting review: activation-budget evidence is not strong enough for the rendered runtime-cost claim.".to_string(),
        )
    } else if compatibility_requires_native_ack(&compatibility)
        && input.consumer_surface_class != InstallReviewSurfaceClass::NativeReviewSheet
    {
        (
            InstallReviewDecisionClass::AwaitingUserReview,
            InstallReviewDecisionReasonClass::CompatibilityLimitedRequiresReview,
            "Awaiting native review: compatibility is limited, bridge-backed, or target-scoped."
                .to_string(),
        )
    } else if input.consumer_surface_class != InstallReviewSurfaceClass::NativeReviewSheet {
        (
            InstallReviewDecisionClass::AwaitingUserReview,
            InstallReviewDecisionReasonClass::NativeReviewPacketRequired,
            "Awaiting native review: this consumer is a narrower read-only lane.".to_string(),
        )
    } else {
        (
            InstallReviewDecisionClass::AdmitAfterNativeReview,
            InstallReviewDecisionReasonClass::AdmittedAfterNativeReview,
            "Admitted after native review: owner/origin, scope, network, service boundary, permission delta, compatibility, activation budget, and install topology were rendered.".to_string(),
        )
    };

    let mutation_allowed = decision_class == InstallReviewDecisionClass::AdmitAfterNativeReview
        && input.consumer_surface_class == InstallReviewSurfaceClass::NativeReviewSheet;

    InstallReviewAlphaPacketRecord {
        record_kind: INSTALL_REVIEW_ALPHA_PACKET_RECORD_KIND.to_string(),
        install_review_alpha_schema_version: INSTALL_REVIEW_ALPHA_SCHEMA_VERSION,
        review_id: input.review_id,
        action_class: input.action_class,
        subject_ref: input.subject_ref,
        consumer_surface_class: input.consumer_surface_class,
        consumer_attempts_native_approval: input.consumer_attempts_native_approval,
        canonical_native_review_ref: boundary_truth.canonical_native_review_ref.clone(),
        extension_review_alpha_ref: extension_review.review_id.clone(),
        effective_permission_summary_ref: effective_permission.manifest_baseline_ref.clone(),
        install_topology_row_ref: install_topology_row.topology_row_id.clone(),
        install_topology_truth_fingerprint: install_topology_row.truth_fingerprint(),
        capability_lifecycle_row_refs: extension_review.capability_lifecycle_row_refs.clone(),
        boundary_truth,
        compatibility,
        activation_budget,
        permission_delta_entries: effective_permission.declared_vs_effective_diff.clone(),
        widening_attempted_blocked_count: effective_permission.widening_attempted_blocked_count,
        upstream_required_disclosures: extension_review.required_disclosures.clone(),
        required_disclosures,
        rendered_disclosures: input.rendered_disclosures,
        mutation_allowed,
        decision_class,
        decision_reason_class,
        decision_summary,
        review_event_refs: input.review_event_refs,
        decided_at: decided_at.to_string(),
        redaction_class: RedactionClass::MetadataSafeDefault,
        content_integrity_warnings,
    }
}

/// Project an install-review packet into one consumer surface.
///
/// Non-native projections never receive approval actions. They can open the
/// canonical native sheet and evidence panels only.
pub fn project_install_review_alpha_surface(
    packet: &InstallReviewAlphaPacketRecord,
    surface_class: InstallReviewSurfaceClass,
) -> InstallReviewAlphaProjectionRecord {
    let mut offered_actions = vec![
        InstallReviewActionOfferClass::OpenNativeReviewSheet,
        InstallReviewActionOfferClass::OpenPermissionDelta,
        InstallReviewActionOfferClass::OpenCompatibilityEvidence,
        InstallReviewActionOfferClass::OpenActivationBudgetEvidence,
        InstallReviewActionOfferClass::OpenInstallTopology,
        InstallReviewActionOfferClass::ExportSupportPacket,
    ];

    if surface_class == InstallReviewSurfaceClass::NativeReviewSheet && packet.mutation_allowed {
        match packet.action_class {
            InstallReviewActionClass::Install | InstallReviewActionClass::Update => {
                offered_actions.insert(0, InstallReviewActionOfferClass::ApproveInstall);
            }
            InstallReviewActionClass::Enable => {
                offered_actions.insert(0, InstallReviewActionOfferClass::EnableExtension);
            }
        }
    }

    match packet.decision_class {
        InstallReviewDecisionClass::AwaitingAdminReview | InstallReviewDecisionClass::Denied => {
            offered_actions.push(InstallReviewActionOfferClass::ConsultAdmin);
            offered_actions.push(InstallReviewActionOfferClass::KeepDisabledOrPinned);
        }
        InstallReviewDecisionClass::AdmitAfterNativeReview
        | InstallReviewDecisionClass::AwaitingUserReview => {}
    }

    InstallReviewAlphaProjectionRecord {
        record_kind: INSTALL_REVIEW_ALPHA_PROJECTION_RECORD_KIND.to_string(),
        install_review_alpha_schema_version: INSTALL_REVIEW_ALPHA_SCHEMA_VERSION,
        projection_id: format!("install_review_projection:{}:{surface_class:?}", packet.review_id),
        review_ref: packet.review_id.clone(),
        surface_class,
        content_source_class: packet.boundary_truth.content_source_class,
        manifest_origin_source_class: packet.boundary_truth.manifest_origin_source_class,
        publisher_trust_tier_class: packet.boundary_truth.publisher_trust_tier_class,
        profile_scope_ref: packet.boundary_truth.profile_scope_ref.clone(),
        org_scope_ref: packet.boundary_truth.org_scope_ref.clone(),
        workspace_scope_class: packet.boundary_truth.workspace_scope_class,
        network_reachability_state: packet.boundary_truth.network_reachability_state,
        service_boundary_class: packet.boundary_truth.service_boundary_class,
        host_boundary_cue_class: packet.boundary_truth.host_boundary_cue_class,
        canonical_review_authority_class: packet.boundary_truth.canonical_review_authority_class,
        canonical_native_review_ref: packet.canonical_native_review_ref.clone(),
        compatibility_claim_class: packet.compatibility.compatibility_claim_class,
        bridge_state_class: packet.compatibility.bridge_state_class,
        compatibility_label: packet.compatibility.compatibility_label,
        compatibility_label_token: packet.compatibility.compatibility_label.as_str().to_string(),
        runtime_cost_class: packet.activation_budget.runtime_cost_class,
        runtime_cost_evidence_class: packet.activation_budget.runtime_cost_evidence_class,
        activation_budget: packet.activation_budget.activation_budget.clone(),
        permission_delta_count: packet.permission_delta_entries.len(),
        widening_attempted_blocked_count: packet.widening_attempted_blocked_count,
        install_topology_row_ref: packet.install_topology_row_ref.clone(),
        install_mode_class: packet
            .install_topology_truth_fingerprint
            .install_mode_class,
        blocked_mutation: !packet.mutation_allowed,
        offered_actions,
        export_safe_summary: format!(
            "{} Content source: {:?}. Scope: {} / {:?}. Network: {:?}. Boundary: {:?}. Compatibility: {:?} ({:?}). Runtime cost: {:?}. Activation budget: cpu={} memory={} startup_cost_ceiling={} opt_in_feature_gates={}. Install row: {}.",
            packet.decision_summary,
            packet.boundary_truth.content_source_class,
            packet.boundary_truth.profile_scope_ref,
            packet.boundary_truth.org_scope_ref,
            packet.boundary_truth.network_reachability_state,
            packet.boundary_truth.service_boundary_class,
            packet.compatibility.compatibility_claim_class,
            packet.compatibility.compatibility_label,
            packet.activation_budget.runtime_cost_class,
            packet.activation_budget.activation_budget.cpu.as_str(),
            packet.activation_budget.activation_budget.memory.as_str(),
            packet
                .activation_budget
                .activation_budget
                .startup_cost_ceiling
                .as_str(),
            packet
                .activation_budget
                .activation_budget
                .opt_in_feature_gates_or_unknown()
                .join(","),
            packet.install_topology_row_ref
        ),
        redaction_class: RedactionClass::MetadataSafeDefault,
        content_integrity_warnings: packet.content_integrity_warnings.clone(),
    }
}

/// Projects shared content-integrity warnings for install-review alpha text.
pub fn project_install_review_alpha_content_integrity(
    case_id: &str,
    subject_ref: &str,
    review_text: &str,
) -> Vec<ContentIntegrityWarningRecord> {
    project_content_integrity_warnings(
        case_id,
        ContentIntegritySurfaceKind::Package,
        subject_ref,
        review_text,
    )
}

fn install_review_alpha_content_for_warnings(
    subject_ref: &str,
    owner_origin_summary: &str,
) -> String {
    format!("{subject_ref}\n{owner_origin_summary}")
}

/// Validate structural invariants for an install-review alpha packet.
pub fn validate_install_review_alpha_packet(
    packet: &InstallReviewAlphaPacketRecord,
) -> Vec<InstallReviewAlphaFinding> {
    let mut findings = Vec::new();

    if packet.record_kind != INSTALL_REVIEW_ALPHA_PACKET_RECORD_KIND {
        findings.push(InstallReviewAlphaFinding::new(
            "install_review_alpha.packet.record_kind_wrong",
            format!(
                "record_kind must be '{INSTALL_REVIEW_ALPHA_PACKET_RECORD_KIND}'; got {:?}",
                packet.record_kind
            ),
        ));
    }
    if packet.install_review_alpha_schema_version != INSTALL_REVIEW_ALPHA_SCHEMA_VERSION {
        findings.push(InstallReviewAlphaFinding::new(
            "install_review_alpha.packet.schema_version_wrong",
            format!(
                "install_review_alpha_schema_version must be {INSTALL_REVIEW_ALPHA_SCHEMA_VERSION}; got {}",
                packet.install_review_alpha_schema_version
            ),
        ));
    }
    if !packet.review_id.starts_with("install_review_alpha:") {
        findings.push(InstallReviewAlphaFinding::new(
            "install_review_alpha.packet.id_unprefixed",
            "review_id must start with 'install_review_alpha:'",
        ));
    }
    if let Some(missing) =
        first_missing_disclosure(&packet.required_disclosures, &packet.rendered_disclosures)
    {
        findings.push(InstallReviewAlphaFinding::new(
            "install_review_alpha.packet.required_disclosure_missing",
            format!("required disclosure '{missing:?}' was not rendered"),
        ));
    }
    if let Some(message) = boundary_truth_missing(&packet.boundary_truth) {
        findings.push(InstallReviewAlphaFinding::new(
            "install_review_alpha.packet.boundary_truth_missing",
            message,
        ));
    }
    if compatibility_evidence_missing(&packet.compatibility)
        && packet.decision_reason_class
            != InstallReviewDecisionReasonClass::CompatibilityEvidenceMissing
    {
        findings.push(InstallReviewAlphaFinding::new(
            "install_review_alpha.packet.compatibility_missing_not_blocked",
            "missing compatibility evidence must block install or enable mutation",
        ));
    }
    if let Some(message) = exact_label_conflicts_with_partial_marker(
        packet.compatibility.compatibility_label,
        &packet.capability_lifecycle_row_refs,
    ) {
        findings.push(InstallReviewAlphaFinding::new(
            "install_review_alpha.packet.exact_compatibility_label_refused",
            message,
        ));
    }
    if packet.widening_attempted_blocked_count > 0
        && packet.decision_reason_class
            != InstallReviewDecisionReasonClass::EffectivePermissionWideningAttempted
    {
        findings.push(InstallReviewAlphaFinding::new(
            "install_review_alpha.packet.widening_not_blocked",
            "permission widening attempts must deny the review packet",
        ));
    }
    if packet.consumer_surface_class != InstallReviewSurfaceClass::NativeReviewSheet
        && packet.mutation_allowed
    {
        findings.push(InstallReviewAlphaFinding::new(
            "install_review_alpha.packet.hosted_consumer_mutation_allowed",
            "hosted, webview, CLI, support, and marketplace lanes cannot commit native review mutations",
        ));
    }
    if packet.consumer_attempts_native_approval
        && packet.consumer_surface_class != InstallReviewSurfaceClass::NativeReviewSheet
        && packet.decision_reason_class
            != InstallReviewDecisionReasonClass::HostedConsumerCannotApprove
    {
        findings.push(InstallReviewAlphaFinding::new(
            "install_review_alpha.packet.hosted_parity_claim_not_denied",
            "hosted or webview native-approval parity claims must be denied",
        ));
    }
    if packet.install_topology_row_ref.trim().is_empty() {
        findings.push(InstallReviewAlphaFinding::new(
            "install_review_alpha.packet.install_topology_ref_missing",
            "install review must cite an install-topology row",
        ));
    }

    findings
}

fn required_disclosures_for(
    _action_class: InstallReviewActionClass,
) -> Vec<InstallReviewDisclosureClass> {
    vec![
        InstallReviewDisclosureClass::OwnerOrigin,
        InstallReviewDisclosureClass::Scope,
        InstallReviewDisclosureClass::NetworkState,
        InstallReviewDisclosureClass::ServiceBoundary,
        InstallReviewDisclosureClass::NativeCanonicalReview,
        InstallReviewDisclosureClass::PermissionDelta,
        InstallReviewDisclosureClass::CompatibilityLabels,
        InstallReviewDisclosureClass::ActivationBudget,
        InstallReviewDisclosureClass::InstallTopology,
        InstallReviewDisclosureClass::PublisherTrust,
    ]
}

fn first_missing_disclosure(
    required: &[InstallReviewDisclosureClass],
    rendered: &[InstallReviewDisclosureClass],
) -> Option<InstallReviewDisclosureClass> {
    required
        .iter()
        .find(|required| !rendered.contains(required))
        .copied()
}

fn boundary_truth_missing(boundary_truth: &InstallReviewBoundaryTruth) -> Option<String> {
    if boundary_truth.profile_scope_ref.trim().is_empty() {
        return Some("Denied: current profile scope is missing from install review.".to_string());
    }
    if boundary_truth.owner_origin_summary.trim().is_empty() {
        return Some("Denied: owner/origin summary is missing from install review.".to_string());
    }
    if boundary_truth.canonical_native_review_ref.trim().is_empty() {
        return Some(
            "Denied: canonical native review ref is missing from install review.".to_string(),
        );
    }
    if matches!(
        boundary_truth.content_source_class,
        InstallReviewContentSourceClass::ProviderOwned
    ) && (boundary_truth.provider_source_class.is_none()
        || boundary_truth.provider_actor_scope.is_none())
    {
        return Some(
            "Denied: provider-owned content must disclose provider source and actor scope."
                .to_string(),
        );
    }
    if matches!(
        boundary_truth.content_source_class,
        InstallReviewContentSourceClass::AccountOwned
    ) && boundary_truth.org_scope_ref.is_none()
    {
        return Some(
            "Denied: account-owned content must disclose current org or tenant scope.".to_string(),
        );
    }
    None
}

fn compatibility_evidence_missing(compatibility: &CompatibilityLabelBlock) -> bool {
    compatibility.evidence_refs.is_empty()
        || compatibility.platform_scope_refs.is_empty()
        || matches!(
            compatibility.compatibility_label,
            CompatibilityLabel::Unknown
        )
        || matches!(
            compatibility.evidence_freshness_class,
            SummaryFreshnessClass::Stale | SummaryFreshnessClass::Unverified
        )
        || matches!(
            compatibility.compatibility_claim_class,
            CompatibilityClaimClass::CompatibilityUnknownPendingReverification
        )
        || matches!(
            compatibility.bridge_state_class,
            BridgeStateClass::BridgeUnknownPendingReverification
        )
}

fn compatibility_requires_native_ack(compatibility: &CompatibilityLabelBlock) -> bool {
    compatibility.compatibility_label.needs_native_ack()
        || matches!(
            compatibility.compatibility_claim_class,
            CompatibilityClaimClass::CompatibleOnSubsetOfDeclaredTargets
                | CompatibilityClaimClass::CompatibilityBridgeRequired
        )
        || matches!(
            compatibility.bridge_state_class,
            BridgeStateClass::BridgeRequiredCompatibilityBridgeProfile
                | BridgeStateClass::BridgeRequiredCapabilityWorldSubsetOnly
                | BridgeStateClass::BridgeRequiredHostContractFamilySubsetOnly
        )
}

fn activation_budget_blocks(activation_budget: &ActivationBudgetDisclosure) -> bool {
    matches!(
        activation_budget.runtime_cost_class,
        RuntimeCostClass::RuntimeCostQuarantinedUnderCrashLoopOrEgressBreach
    )
}

fn activation_budget_evidence_missing(activation_budget: &ActivationBudgetDisclosure) -> bool {
    activation_budget.evidence_refs.is_empty()
        || activation_budget.budget_axis_refs.is_empty()
        || activation_budget.activation_trigger_refs.is_empty()
        || activation_budget.activation_budget.has_unknown_axis()
        || matches!(
            activation_budget.runtime_cost_class,
            RuntimeCostClass::RuntimeCostUnknownPendingEvidence
        )
        || matches!(
            activation_budget.runtime_cost_evidence_class,
            RuntimeCostEvidenceClass::ActivationEvidencePacketAbsentPendingFirstSession
                | RuntimeCostEvidenceClass::BenchmarkArchiveAbsent
                | RuntimeCostEvidenceClass::SelfReportedOnlyUnverified
        )
}

fn exact_label_conflicts_with_partial_marker(
    compatibility_label: CompatibilityLabel,
    capability_lifecycle_row_refs: &[String],
) -> Option<String> {
    if compatibility_label != CompatibilityLabel::Exact || capability_lifecycle_row_refs.is_empty()
    {
        return None;
    }
    let registry = current_capability_lifecycle_registry().ok()?;
    for row_ref in capability_lifecycle_row_refs {
        let Some(row) = registry.row_by_id(row_ref) else {
            continue;
        };
        if let Some(marker) = registry
            .markers_for_row(row)
            .into_iter()
            .find(|marker| marker_narrows_exact_compatibility(marker))
        {
            return Some(format!(
                "Denied: exact compatibility label conflicts with dependency marker {} on {}.",
                marker.marker_id(),
                row_ref
            ));
        }
    }
    None
}

fn marker_narrows_exact_compatibility(marker: &DependencyMarker) -> bool {
    matches!(
        marker.effect_on_parent(),
        EffectOnParent::NarrowsEffectiveLifecycleState
            | EffectOnParent::NarrowsEffectiveSupportClass
            | EffectOnParent::NarrowsEffectiveReleaseChannel
            | EffectOnParent::NarrowsEffectiveFreshnessClass
            | EffectOnParent::NarrowsEffectiveClientScope
            | EffectOnParent::GatesEntireCapability
    ) || matches!(
        marker.marker_kind(),
        MarkerKind::NonStableCapabilityDependency
            | MarkerKind::DisabledByPolicyDependency
            | MarkerKind::ProviderLinkedDependency
            | MarkerKind::ClientScopeRestrictedDependency
            | MarkerKind::FreshnessFloorDependency
            | MarkerKind::KillSwitchDependency
            | MarkerKind::ManagedOnlyDependency
    )
}
