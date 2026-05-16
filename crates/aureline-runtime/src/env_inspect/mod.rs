//! Environment-inspect contract.
//!
//! This module owns the canonical `env-inspect` snapshot that answers the
//! "why this target, runtime, toolchain, and policy decision?" question for
//! a freshly resolved [`ExecutionContext`]. The projection is the single
//! contract every claimed beta surface reads: the chrome inspector, the
//! headless CLI / inspector binary, the support-export wrapper, and the
//! reviewer docs all quote the same core fields, the same degradation
//! labels, and the same lane token.
//!
//! Truth is shaped here so UI and CLI/headless renderings cannot drift.
//! Surfaces never invent their own subsets, never re-derive labels from
//! raw env, and never embed raw command lines, raw environment values, or
//! unmanaged credentials.
//!
//! The cross-tool boundary schema lives at
//! [`/schemas/execution/env_inspect.schema.json`](../../../../schemas/execution/env_inspect.schema.json).
//! The reviewer-facing landing page is
//! [`/docs/runtime/m3/env_inspect_beta.md`](../../../../docs/runtime/m3/env_inspect_beta.md).

use serde::{Deserialize, Serialize};

use crate::execution_context::beta::{lane_for_context, ExecutionContextBetaLane};
use crate::execution_context::{
    ActivationStrategy, CacheDisposition, CapsuleDriftState, ConfidenceLevel, DegradedFieldReason,
    DegradedFieldRecord, EnvironmentCapsuleRef, ExecutionContext, ExecutionContextRequest,
    ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode, MixedVersionDriftState,
    MixedVersionReason, PrebuildInvalidationReason, PrebuildReuseState, ReachabilityState,
    ScopeClass, SurfaceClass, TargetClass, TargetConfidenceReason, ToolchainClass, TrustState,
    EXECUTION_CONTEXT_SCHEMA_VERSION,
};

/// Schema version stamped onto every env-inspect record this module emits.
pub const ENV_INSPECT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`EnvInspectSnapshot`] payloads.
pub const ENV_INSPECT_SNAPSHOT_RECORD_KIND: &str = "env_inspect_snapshot_record";

/// Stable record-kind tag for [`EnvInspectSupportExport`] payloads.
pub const ENV_INSPECT_SUPPORT_EXPORT_RECORD_KIND: &str = "env_inspect_support_export_record";

/// Closed vocabulary of inspect section bands. The render order is fixed so
/// every consumer surface (UI inspector, CLI/headless output, support
/// export plaintext) lists the same fields in the same order without
/// re-deriving the grouping locally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvInspectSection {
    /// Lane and boundary cue posture.
    Lane,
    /// Target identity (class, canonical id, working directory, reachability).
    Target,
    /// Toolchain identity (class, id, version, activation strategy).
    Toolchain,
    /// Environment capsule reference and drift state.
    EnvironmentCapsule,
    /// Trust state, identity mode, policy epoch.
    PolicyAndTrust,
    /// Workset scope class.
    Scope,
    /// Cache disposition.
    Cache,
    /// Prebuild reuse state and invalidation reason.
    Prebuild,
    /// Mixed-version posture between client and helper.
    MixedVersion,
    /// Target confidence level and reasons.
    TargetConfidence,
    /// Provenance record id, resolver version, recorded-at.
    Provenance,
}

impl EnvInspectSection {
    /// Canonical render order. Consumers MUST render sections in this order.
    pub const ORDER: [Self; 11] = [
        Self::Lane,
        Self::Target,
        Self::Toolchain,
        Self::EnvironmentCapsule,
        Self::PolicyAndTrust,
        Self::Scope,
        Self::Cache,
        Self::Prebuild,
        Self::MixedVersion,
        Self::TargetConfidence,
        Self::Provenance,
    ];

    /// Stable string token recorded on every core-field row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lane => "lane",
            Self::Target => "target",
            Self::Toolchain => "toolchain",
            Self::EnvironmentCapsule => "environment_capsule",
            Self::PolicyAndTrust => "policy_and_trust",
            Self::Scope => "scope",
            Self::Cache => "cache",
            Self::Prebuild => "prebuild",
            Self::MixedVersion => "mixed_version",
            Self::TargetConfidence => "target_confidence",
            Self::Provenance => "provenance",
        }
    }

    /// Reviewer-facing section label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Lane => "Lane",
            Self::Target => "Target",
            Self::Toolchain => "Toolchain",
            Self::EnvironmentCapsule => "Environment capsule",
            Self::PolicyAndTrust => "Policy & trust",
            Self::Scope => "Workset scope",
            Self::Cache => "Cache disposition",
            Self::Prebuild => "Prebuild",
            Self::MixedVersion => "Mixed-version posture",
            Self::TargetConfidence => "Target confidence",
            Self::Provenance => "Provenance",
        }
    }
}

/// Severity band for an inspect degradation label.
///
/// The bands let chrome render an honesty marker, a review-required warning,
/// or a blocking gate without re-deriving severity from
/// [`DegradedFieldReason`] locally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvInspectDegradationSeverity {
    /// The resolver settled the value but a non-blocking truth gap remains
    /// (fallback toolchain, low-confidence target, helper not negotiated).
    /// Chrome MUST surface the honesty marker. Dispatch is not blocked.
    Notice,
    /// Trust or policy gate is unresolved or narrowed. Dispatch requires
    /// re-authorisation review.
    Warning,
    /// The lane is fundamentally narrowed: the target is unreachable, the
    /// capsule is unresolved, or a workset member is missing. Dispatch MUST
    /// be blocked or re-authorised before launching.
    Blocking,
}

impl EnvInspectDegradationSeverity {
    /// Stable string token recorded on the label row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notice => "notice",
            Self::Warning => "warning",
            Self::Blocking => "blocking",
        }
    }

    /// True when the band requires a review before dispatch.
    pub const fn requires_review(self) -> bool {
        matches!(self, Self::Warning | Self::Blocking)
    }

    /// True when the band blocks dispatch.
    pub const fn blocks_dispatch(self) -> bool {
        matches!(self, Self::Blocking)
    }

    /// Map a degraded-field reason to its severity band.
    pub const fn for_reason(reason: DegradedFieldReason) -> Self {
        match reason {
            DegradedFieldReason::ToolchainFallback
            | DegradedFieldReason::ConfidenceLow
            | DegradedFieldReason::ProvenanceGap => Self::Notice,
            DegradedFieldReason::ActivatorBlockedByTrust
            | DegradedFieldReason::ActivatorBlockedByPolicy
            | DegradedFieldReason::PolicyEpochStale
            | DegradedFieldReason::TrustStateUnresolved
            | DegradedFieldReason::CapsuleDriftDetected
            | DegradedFieldReason::RemoteAgentScopeMismatch => Self::Warning,
            DegradedFieldReason::ActivatorUnsupportedOnTarget
            | DegradedFieldReason::CapsuleUnresolved
            | DegradedFieldReason::TargetUnreachable
            | DegradedFieldReason::WorksetMemberUnavailable => Self::Blocking,
        }
    }

    /// Reviewer-facing label for the band.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Notice => "Notice",
            Self::Warning => "Review required",
            Self::Blocking => "Blocked",
        }
    }
}

/// One typed degradation label rendered alongside the inspect snapshot.
///
/// The label quotes the structured reason, the severity band, the field path
/// it applies to, and the repair-hook reference when one is registered. Raw
/// env values, raw command lines, and raw secrets MUST NOT be embedded here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvInspectDegradationLabel {
    /// Dotted field path the label applies to.
    pub field_path: String,
    /// Structured reason from the canonical context.
    pub reason: DegradedFieldReason,
    /// Stable reason token.
    pub reason_token: String,
    /// Severity band derived from [`Self::reason`].
    pub severity: EnvInspectDegradationSeverity,
    /// Stable severity token.
    pub severity_token: String,
    /// Reviewer-facing one-line label suitable for chrome and CLI output.
    pub label: String,
    /// Repair-hook reference when the runtime registers one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repair_hook_ref: Option<String>,
}

impl EnvInspectDegradationLabel {
    /// Project a label from one degraded-field record on the canonical
    /// context.
    pub fn from_record(record: &DegradedFieldRecord) -> Self {
        let severity = EnvInspectDegradationSeverity::for_reason(record.reason);
        Self {
            field_path: record.field_path.clone(),
            reason: record.reason,
            reason_token: record.reason.as_str().to_owned(),
            severity,
            severity_token: severity.as_str().to_owned(),
            label: format!("{}: {}", severity.label(), reason_label(record.reason)),
            repair_hook_ref: record.repair_hook_ref.clone(),
        }
    }
}

/// One inspect core-field row. Both the UI inspector and the CLI/headless
/// output render the same row sequence, so reviewer / support evidence is
/// identical regardless of which surface produced it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvInspectCoreField {
    /// Section the row belongs to.
    pub section: EnvInspectSection,
    /// Stable section token.
    pub section_token: String,
    /// Dotted field path into the canonical context.
    pub field_path: String,
    /// Reviewer-facing label.
    pub label: String,
    /// Stable value token. `None` when the resolver did not record a value
    /// (the consumer renders an explicit "unset" marker instead of inventing
    /// one locally).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_token: Option<String>,
}

impl EnvInspectCoreField {
    fn new(
        section: EnvInspectSection,
        field_path: impl Into<String>,
        label: impl Into<String>,
        value_token: Option<String>,
    ) -> Self {
        Self {
            section,
            section_token: section.as_str().to_owned(),
            field_path: field_path.into(),
            label: label.into(),
            value_token,
        }
    }
}

/// Redaction class recorded on every env-inspect support export.
///
/// The vocabulary is intentionally closed and the only valid value bundles
/// the export-safety guarantee: no raw env values, no raw command lines, no
/// secrets, no unmanaged credentials.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvInspectRedactionClass {
    /// Only structured tokens, hashes, ids, and class labels are emitted.
    /// Reviewer / support flows MAY embed this payload verbatim.
    StructuredTokensOnly,
}

impl EnvInspectRedactionClass {
    /// Stable string token recorded on the export.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StructuredTokensOnly => "structured_tokens_only",
        }
    }
}

/// Canonical env-inspect snapshot.
///
/// One snapshot per resolved [`ExecutionContext`]. Surfaces consume the
/// record verbatim — they MUST NOT re-derive lane, target, toolchain, trust,
/// prebuild, mixed-version, or degradation truth from raw env / process
/// state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvInspectSnapshot {
    /// Stable record kind.
    pub record_kind: String,
    /// Env-inspect schema version.
    pub schema_version: u32,
    /// Execution-context schema version the snapshot was projected from.
    pub execution_context_schema_version: u32,
    /// Canonical execution-context id this snapshot was projected from.
    pub execution_context_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Surface that minted the canonical context.
    pub surface: SurfaceClass,
    /// Stable surface token.
    pub surface_token: String,
    /// Lane the canonical context resolved onto.
    pub lane: ExecutionContextBetaLane,
    /// Stable lane token.
    pub lane_token: String,
    /// Reviewer-facing lane label.
    pub lane_label: String,
    /// True when the local-vs-managed boundary cue MUST be rendered.
    pub boundary_cue_visible: bool,
    /// Resolved target class.
    pub target_class: TargetClass,
    /// Stable target-class token.
    pub target_class_token: String,
    /// Canonical target id.
    pub canonical_target_id: String,
    /// Resolved working directory, if recorded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    /// Target reachability state.
    pub reachability_state: ReachabilityState,
    /// Stable reachability token.
    pub reachability_state_token: String,
    /// Resolved toolchain class.
    pub toolchain_class: ToolchainClass,
    /// Stable toolchain-class token.
    pub toolchain_class_token: String,
    /// Toolchain id.
    pub toolchain_id: String,
    /// Resolved toolchain version.
    pub resolved_version: String,
    /// Activation strategy.
    pub activation_strategy: ActivationStrategy,
    /// Stable activation-strategy token.
    pub activation_strategy_token: String,
    /// True when the resolver settled on a degraded fallback toolchain.
    pub toolchain_degraded_fallback: bool,
    /// Environment capsule reference snapshot.
    pub environment_capsule_ref: EnvironmentCapsuleRef,
    /// Trust state.
    pub trust_state: TrustState,
    /// Stable trust-state token.
    pub trust_state_token: String,
    /// Identity mode.
    pub identity_mode: IdentityMode,
    /// Stable identity-mode token.
    pub identity_mode_token: String,
    /// Policy epoch in effect at resolve time.
    pub policy_epoch: u64,
    /// Workset scope class.
    pub workset_scope_class: ScopeClass,
    /// Stable workset-scope-class token.
    pub workset_scope_class_token: String,
    /// Cache disposition.
    pub cache_disposition: CacheDisposition,
    /// Stable cache-disposition token.
    pub cache_disposition_token: String,
    /// Prebuild reuse state.
    pub prebuild_reuse_state: PrebuildReuseState,
    /// Stable prebuild-reuse-state token.
    pub prebuild_reuse_state_token: String,
    /// Prebuild snapshot reference when one is in play.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prebuild_snapshot_ref: Option<String>,
    /// Prebuild compatibility fingerprint.
    pub prebuild_compatibility_fingerprint: String,
    /// Prebuild invalidation reason when [`Self::prebuild_reuse_state`] is
    /// rejected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prebuild_invalidation_reason: Option<PrebuildInvalidationReason>,
    /// Mixed-version posture.
    pub mixed_version_state: MixedVersionDriftState,
    /// Stable mixed-version-state token.
    pub mixed_version_state_token: String,
    /// Structured mixed-version reason.
    pub mixed_version_reason: MixedVersionReason,
    /// Stable mixed-version-reason token.
    pub mixed_version_reason_token: String,
    /// Client protocol family recorded by the resolver.
    pub client_protocol: String,
    /// Helper protocol family when a helper advertised one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helper_protocol: Option<String>,
    /// Target confidence level.
    pub target_confidence_level: ConfidenceLevel,
    /// Stable target-confidence-level token.
    pub target_confidence_level_token: String,
    /// Structured target-confidence reasons.
    pub target_confidence_reasons: Vec<TargetConfidenceReason>,
    /// Stable target-confidence-reason tokens.
    pub target_confidence_reason_tokens: Vec<String>,
    /// Provenance record id.
    pub provenance_record_ref: String,
    /// Resolver version recorded on the canonical context.
    pub resolver_version: String,
    /// Resolve-time timestamp recorded on the canonical context.
    pub recorded_at: String,
    /// Core inspect fields in canonical render order. UI and CLI/headless
    /// surfaces MUST iterate this list verbatim.
    pub core_fields: Vec<EnvInspectCoreField>,
    /// Typed degradation labels for every degraded-field record on the
    /// canonical context.
    pub degradation_labels: Vec<EnvInspectDegradationLabel>,
}

impl EnvInspectSnapshot {
    /// Project a canonical snapshot from a freshly resolved context.
    pub fn from_context(context: &ExecutionContext) -> Self {
        let lane = lane_for_context(context);
        let target = &context.target_identity;
        let toolchain = &context.toolchain_identity;
        let policy = &context.policy_and_trust;
        let prebuild = &context.prebuild_metadata;
        let mixed = &context.mixed_version_drift;
        let confidence = &context.target_confidence;

        let core_fields = build_core_fields(context, lane);
        let degradation_labels = context
            .degraded_fields
            .iter()
            .map(EnvInspectDegradationLabel::from_record)
            .collect();

        Self {
            record_kind: ENV_INSPECT_SNAPSHOT_RECORD_KIND.to_owned(),
            schema_version: ENV_INSPECT_SCHEMA_VERSION,
            execution_context_schema_version: EXECUTION_CONTEXT_SCHEMA_VERSION,
            execution_context_id: context.execution_context_id.clone(),
            workspace_id: context.invocation_subject.workspace_id.clone(),
            surface: context.invocation_subject.surface,
            surface_token: context.invocation_subject.surface.as_str().to_owned(),
            lane,
            lane_token: lane.as_str().to_owned(),
            lane_label: lane.label().to_owned(),
            boundary_cue_visible: context.boundary_cue_visible(),
            target_class: target.target_class,
            target_class_token: target.target_class.as_str().to_owned(),
            canonical_target_id: target.canonical_target_id.clone(),
            working_directory: target.working_directory.clone(),
            reachability_state: target.reachability_state,
            reachability_state_token: target.reachability_state.as_str().to_owned(),
            toolchain_class: toolchain.toolchain_class,
            toolchain_class_token: toolchain.toolchain_class.as_str().to_owned(),
            toolchain_id: toolchain.toolchain_id.clone(),
            resolved_version: toolchain.resolved_version.clone(),
            activation_strategy: toolchain.activation_strategy,
            activation_strategy_token: toolchain.activation_strategy.as_str().to_owned(),
            toolchain_degraded_fallback: toolchain.degraded_fallback_flag,
            environment_capsule_ref: context.environment_capsule_ref.clone(),
            trust_state: policy.trust_state,
            trust_state_token: trust_state_token(policy.trust_state).to_owned(),
            identity_mode: policy.identity_mode,
            identity_mode_token: policy.identity_mode.as_str().to_owned(),
            policy_epoch: policy.policy_epoch,
            workset_scope_class: context.workset_scope_class,
            workset_scope_class_token: context.workset_scope_class.as_str().to_owned(),
            cache_disposition: context.cache_disposition,
            cache_disposition_token: context.cache_disposition.as_str().to_owned(),
            prebuild_reuse_state: prebuild.reuse_state,
            prebuild_reuse_state_token: prebuild.reuse_state.as_str().to_owned(),
            prebuild_snapshot_ref: prebuild.snapshot_ref.clone(),
            prebuild_compatibility_fingerprint: prebuild.compatibility_fingerprint.clone(),
            prebuild_invalidation_reason: prebuild.invalidation_reason,
            mixed_version_state: mixed.state,
            mixed_version_state_token: mixed.state.as_str().to_owned(),
            mixed_version_reason: mixed.reason,
            mixed_version_reason_token: mixed.reason.as_str().to_owned(),
            client_protocol: mixed.client_protocol.clone(),
            helper_protocol: mixed.helper_protocol.clone(),
            target_confidence_level: confidence.level,
            target_confidence_level_token: confidence.level.as_str().to_owned(),
            target_confidence_reasons: confidence.reasons.clone(),
            target_confidence_reason_tokens: confidence
                .reasons
                .iter()
                .map(|reason| reason.as_str().to_owned())
                .collect(),
            provenance_record_ref: context.provenance.provenance_record_id.clone(),
            resolver_version: context.provenance.resolver_version.clone(),
            recorded_at: context.provenance.recorded_at.clone(),
            core_fields,
            degradation_labels,
        }
    }

    /// True when the snapshot carries at least one degradation label.
    pub fn has_degradation(&self) -> bool {
        !self.degradation_labels.is_empty()
    }

    /// True when any degradation label requires review before dispatch.
    pub fn requires_review_before_dispatch(&self) -> bool {
        self.degradation_labels
            .iter()
            .any(|label| label.severity.requires_review())
    }

    /// True when any degradation label blocks dispatch.
    pub fn blocks_dispatch(&self) -> bool {
        self.degradation_labels
            .iter()
            .any(|label| label.severity.blocks_dispatch())
    }

    /// Core fields belonging to one section, in canonical order.
    pub fn fields_for_section(
        &self,
        section: EnvInspectSection,
    ) -> impl Iterator<Item = &EnvInspectCoreField> {
        self.core_fields
            .iter()
            .filter(move |field| field.section == section)
    }

    /// Render the snapshot as a stable plaintext block suitable for the
    /// CLI / headless surface, the support-export wrapper, and copy-to-
    /// clipboard. The output is identical across UI inspector chrome and
    /// the headless binary: both consume [`Self::core_fields`] and
    /// [`Self::degradation_labels`] verbatim.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "env-inspect snapshot: {}\n",
            self.execution_context_id
        ));
        out.push_str(&format!(
            "  workspace: {}\n  surface: {}\n  lane: {} ({})\n  boundary_cue: {}\n",
            self.workspace_id,
            self.surface_token,
            self.lane_token,
            self.lane_label,
            self.boundary_cue_visible,
        ));
        for section in EnvInspectSection::ORDER {
            let mut section_rows: Vec<&EnvInspectCoreField> =
                self.fields_for_section(section).collect();
            if section_rows.is_empty() {
                continue;
            }
            section_rows.sort_by(|a, b| a.field_path.cmp(&b.field_path));
            out.push_str(&format!("  [{}] {}\n", section.as_str(), section.label()));
            for row in section_rows {
                let value = row.value_token.as_deref().unwrap_or("<unset>");
                out.push_str(&format!(
                    "    - {} ({}) = {}\n",
                    row.label, row.field_path, value
                ));
            }
        }
        if self.degradation_labels.is_empty() {
            out.push_str("  degradation: none\n");
        } else {
            out.push_str("  degradation:\n");
            for label in &self.degradation_labels {
                out.push_str(&format!(
                    "    - [{}] {} ({}): {}\n",
                    label.severity_token, label.reason_token, label.field_path, label.label,
                ));
            }
        }
        out
    }
}

/// Support-export packet bundling one or more env-inspect snapshots.
///
/// The packet stamps a redaction class so reviewer / support consumers can
/// embed the payload verbatim without re-validating that raw env, raw
/// command lines, or secrets were stripped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvInspectSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Env-inspect schema version.
    pub schema_version: u32,
    /// Execution-context schema version the snapshots were projected from.
    pub execution_context_schema_version: u32,
    /// Manifest id recorded by the support flow.
    pub manifest_id: String,
    /// Manifest timestamp.
    pub generated_at: String,
    /// Snapshots bundled in the export.
    pub snapshots: Vec<EnvInspectSnapshot>,
    /// Redaction class. Only `structured_tokens_only` is valid.
    pub redaction_class: EnvInspectRedactionClass,
    /// Stable redaction-class token.
    pub redaction_class_token: String,
    /// True when any bundled snapshot carries a degradation label.
    pub any_degradation: bool,
    /// True when any bundled snapshot requires review before dispatch.
    pub any_requires_review: bool,
    /// True when any bundled snapshot blocks dispatch outright.
    pub any_blocks_dispatch: bool,
}

impl EnvInspectSupportExport {
    /// Build a support-export packet for a snapshot collection.
    pub fn new(
        manifest_id: impl Into<String>,
        generated_at: impl Into<String>,
        snapshots: Vec<EnvInspectSnapshot>,
    ) -> Self {
        let any_degradation = snapshots.iter().any(EnvInspectSnapshot::has_degradation);
        let any_requires_review = snapshots
            .iter()
            .any(EnvInspectSnapshot::requires_review_before_dispatch);
        let any_blocks_dispatch = snapshots.iter().any(EnvInspectSnapshot::blocks_dispatch);
        Self {
            record_kind: ENV_INSPECT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: ENV_INSPECT_SCHEMA_VERSION,
            execution_context_schema_version: EXECUTION_CONTEXT_SCHEMA_VERSION,
            manifest_id: manifest_id.into(),
            generated_at: generated_at.into(),
            snapshots,
            redaction_class: EnvInspectRedactionClass::StructuredTokensOnly,
            redaction_class_token: EnvInspectRedactionClass::StructuredTokensOnly
                .as_str()
                .to_owned(),
            any_degradation,
            any_requires_review,
            any_blocks_dispatch,
        }
    }

    /// Project one snapshot for each provided context and bundle them.
    pub fn from_contexts<'a, I>(
        manifest_id: impl Into<String>,
        generated_at: impl Into<String>,
        contexts: I,
    ) -> Self
    where
        I: IntoIterator<Item = &'a ExecutionContext>,
    {
        let snapshots = contexts
            .into_iter()
            .map(EnvInspectSnapshot::from_context)
            .collect();
        Self::new(manifest_id, generated_at, snapshots)
    }
}

/// Identifier for a seeded env-inspect scenario the headless CLI / inspector
/// binary, the chrome panel projection, and the integration test all
/// replay verbatim.
///
/// Seeded scenarios MUST stay deterministic: reviewer / partner runs of the
/// headless inspector reproduce reviewer-fixture records byte-for-byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvInspectSeededScenario {
    /// Local-host terminal seed on the trusted lane.
    LocalTerminal,
    /// Remote-attach task seed with pending trust state (review required).
    RemoteAttachPendingTrust,
    /// Container task seed against a devcontainer target.
    ContainerDevcontainer,
    /// Managed-workspace task seed on the restricted trust lane.
    ManagedWorkspaceRestricted,
}

impl EnvInspectSeededScenario {
    /// Every seeded scenario in canonical order.
    pub const ALL: [Self; 4] = [
        Self::LocalTerminal,
        Self::RemoteAttachPendingTrust,
        Self::ContainerDevcontainer,
        Self::ManagedWorkspaceRestricted,
    ];

    /// Stable string token recorded in CLI output and reviewer fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalTerminal => "local_terminal",
            Self::RemoteAttachPendingTrust => "remote_attach_pending_trust",
            Self::ContainerDevcontainer => "container_devcontainer",
            Self::ManagedWorkspaceRestricted => "managed_workspace_restricted",
        }
    }
}

/// Build a fresh resolver pre-seeded with the env-inspect baseline config.
pub fn seeded_env_inspect_resolver() -> ExecutionContextResolver {
    ExecutionContextResolver::new(ExecutionContextResolverConfig {
        workspace_id: "ws-env-inspect-beta".to_owned(),
        profile_id: Some("prof.env-inspect-beta".to_owned()),
        identity_mode: IdentityMode::AccountFreeLocal,
        policy_epoch: 1,
        workspace_default_target_class: TargetClass::LocalHost,
        workspace_default_working_directory: Some("/workspace".to_owned()),
        workspace_default_scope_class: ScopeClass::CurrentRoot,
        local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
        environment_capsule_ref: EnvironmentCapsuleRef {
            capsule_id: "caps:env-inspect-beta:seed".to_owned(),
            capsule_hash: "sha256:env-inspect-seed".to_owned(),
            resolved_schema_version: "1".to_owned(),
            drift_state: CapsuleDriftState::InSync,
        },
        resolver_version: "env-inspect-beta-0".to_owned(),
    })
}

/// Build the seeded snapshot for one canonical scenario.
pub fn seeded_env_inspect_snapshot(scenario: EnvInspectSeededScenario) -> EnvInspectSnapshot {
    let mut resolver = seeded_env_inspect_resolver();
    let context = match scenario {
        EnvInspectSeededScenario::LocalTerminal => {
            resolver.resolve(ExecutionContextRequest::local_terminal_seed(
                "terminal.open",
                TrustState::Trusted,
                "mono:0",
            ))
        }
        EnvInspectSeededScenario::RemoteAttachPendingTrust => {
            resolver.resolve(ExecutionContextRequest::remote_attach_task_seed(
                "task.run.ssh_remote",
                TargetClass::SshRemote,
                TrustState::PendingEvaluation,
                "mono:0",
            ))
        }
        EnvInspectSeededScenario::ContainerDevcontainer => {
            resolver.resolve(ExecutionContextRequest::container_task_seed(
                "task.run.devcontainer",
                TargetClass::Devcontainer,
                TrustState::Trusted,
                "mono:0",
            ))
        }
        EnvInspectSeededScenario::ManagedWorkspaceRestricted => {
            resolver.resolve(ExecutionContextRequest::request_workspace_task_seed(
                "task.run.managed_workspace",
                TargetClass::ManagedWorkspace,
                TrustState::Restricted,
                "mono:0",
            ))
        }
    };
    EnvInspectSnapshot::from_context(&context)
}

/// Build seeded snapshots for every canonical scenario, bundled as a
/// support-export packet.
pub fn seeded_env_inspect_support_export(
    manifest_id: impl Into<String>,
    generated_at: impl Into<String>,
) -> EnvInspectSupportExport {
    let snapshots: Vec<EnvInspectSnapshot> = EnvInspectSeededScenario::ALL
        .into_iter()
        .map(seeded_env_inspect_snapshot)
        .collect();
    EnvInspectSupportExport::new(manifest_id, generated_at, snapshots)
}

fn build_core_fields(
    context: &ExecutionContext,
    lane: ExecutionContextBetaLane,
) -> Vec<EnvInspectCoreField> {
    let mut fields = Vec::new();
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Lane,
        "lane",
        "Lane",
        Some(lane.as_str().to_owned()),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Lane,
        "target_identity.local_vs_managed_boundary_visible",
        "Boundary cue visible",
        Some(context.boundary_cue_visible().to_string()),
    ));

    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Target,
        "target_identity.target_class",
        "Target class",
        Some(context.target_identity.target_class.as_str().to_owned()),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Target,
        "target_identity.canonical_target_id",
        "Canonical target id",
        Some(context.target_identity.canonical_target_id.clone()),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Target,
        "target_identity.working_directory",
        "Working directory",
        context.target_identity.working_directory.clone(),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Target,
        "target_identity.reachability_state",
        "Reachability",
        Some(
            context
                .target_identity
                .reachability_state
                .as_str()
                .to_owned(),
        ),
    ));

    let toolchain = &context.toolchain_identity;
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Toolchain,
        "toolchain_identity.toolchain_class",
        "Toolchain class",
        Some(toolchain.toolchain_class.as_str().to_owned()),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Toolchain,
        "toolchain_identity.toolchain_id",
        "Toolchain id",
        Some(toolchain.toolchain_id.clone()),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Toolchain,
        "toolchain_identity.resolved_version",
        "Resolved version",
        Some(toolchain.resolved_version.clone()),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Toolchain,
        "toolchain_identity.activation_strategy",
        "Activation strategy",
        Some(toolchain.activation_strategy.as_str().to_owned()),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Toolchain,
        "toolchain_identity.degraded_fallback_flag",
        "Degraded fallback",
        Some(toolchain.degraded_fallback_flag.to_string()),
    ));

    let capsule = &context.environment_capsule_ref;
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::EnvironmentCapsule,
        "environment_capsule_ref.capsule_id",
        "Capsule id",
        Some(capsule.capsule_id.clone()),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::EnvironmentCapsule,
        "environment_capsule_ref.capsule_hash",
        "Capsule hash",
        Some(capsule.capsule_hash.clone()),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::EnvironmentCapsule,
        "environment_capsule_ref.resolved_schema_version",
        "Capsule schema version",
        Some(capsule.resolved_schema_version.clone()),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::EnvironmentCapsule,
        "environment_capsule_ref.drift_state",
        "Capsule drift",
        Some(capsule.drift_state.as_str().to_owned()),
    ));

    let policy = &context.policy_and_trust;
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::PolicyAndTrust,
        "policy_and_trust.trust_state",
        "Trust state",
        Some(trust_state_token(policy.trust_state).to_owned()),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::PolicyAndTrust,
        "policy_and_trust.identity_mode",
        "Identity mode",
        Some(policy.identity_mode.as_str().to_owned()),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::PolicyAndTrust,
        "policy_and_trust.policy_epoch",
        "Policy epoch",
        Some(policy.policy_epoch.to_string()),
    ));

    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Scope,
        "workset_scope_class",
        "Workset scope",
        Some(context.workset_scope_class.as_str().to_owned()),
    ));

    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Cache,
        "cache_disposition",
        "Cache disposition",
        Some(context.cache_disposition.as_str().to_owned()),
    ));

    let prebuild = &context.prebuild_metadata;
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Prebuild,
        "prebuild_metadata.reuse_state",
        "Prebuild reuse",
        Some(prebuild.reuse_state.as_str().to_owned()),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Prebuild,
        "prebuild_metadata.compatibility_fingerprint",
        "Prebuild fingerprint",
        Some(prebuild.compatibility_fingerprint.clone()),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Prebuild,
        "prebuild_metadata.snapshot_ref",
        "Prebuild snapshot",
        prebuild.snapshot_ref.clone(),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Prebuild,
        "prebuild_metadata.invalidation_reason",
        "Prebuild invalidation",
        prebuild
            .invalidation_reason
            .map(|reason| reason.as_str().to_owned()),
    ));

    let mixed = &context.mixed_version_drift;
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::MixedVersion,
        "mixed_version_drift.state",
        "Mixed-version state",
        Some(mixed.state.as_str().to_owned()),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::MixedVersion,
        "mixed_version_drift.reason",
        "Mixed-version reason",
        Some(mixed.reason.as_str().to_owned()),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::MixedVersion,
        "mixed_version_drift.client_protocol",
        "Client protocol",
        Some(mixed.client_protocol.clone()),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::MixedVersion,
        "mixed_version_drift.helper_protocol",
        "Helper protocol",
        mixed.helper_protocol.clone(),
    ));

    let confidence = &context.target_confidence;
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::TargetConfidence,
        "target_confidence.level",
        "Confidence level",
        Some(confidence.level.as_str().to_owned()),
    ));
    let reasons_token = if confidence.reasons.is_empty() {
        "none".to_owned()
    } else {
        confidence
            .reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect::<Vec<_>>()
            .join("|")
    };
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::TargetConfidence,
        "target_confidence.reasons",
        "Confidence reasons",
        Some(reasons_token),
    ));

    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Provenance,
        "provenance.provenance_record_id",
        "Provenance id",
        Some(context.provenance.provenance_record_id.clone()),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Provenance,
        "provenance.resolver_version",
        "Resolver version",
        Some(context.provenance.resolver_version.clone()),
    ));
    fields.push(EnvInspectCoreField::new(
        EnvInspectSection::Provenance,
        "provenance.recorded_at",
        "Recorded at",
        Some(context.provenance.recorded_at.clone()),
    ));

    fields
}

const fn reason_label(reason: DegradedFieldReason) -> &'static str {
    match reason {
        DegradedFieldReason::ToolchainFallback => "Resolver fell back to a non-preferred toolchain",
        DegradedFieldReason::ActivatorBlockedByTrust => "Activator blocked by trust state",
        DegradedFieldReason::ActivatorBlockedByPolicy => "Activator blocked by workspace policy",
        DegradedFieldReason::ActivatorUnsupportedOnTarget => "Activator unsupported on target",
        DegradedFieldReason::CapsuleUnresolved => "Environment capsule unresolved",
        DegradedFieldReason::CapsuleDriftDetected => "Environment capsule drift detected",
        DegradedFieldReason::TargetUnreachable => "Target unreachable",
        DegradedFieldReason::PolicyEpochStale => "Policy epoch is stale",
        DegradedFieldReason::TrustStateUnresolved => "Trust state pending evaluation",
        DegradedFieldReason::WorksetMemberUnavailable => "Workset member unavailable",
        DegradedFieldReason::ProvenanceGap => "Provenance gap recorded",
        DegradedFieldReason::ConfidenceLow => "Resolver confidence is low",
        DegradedFieldReason::RemoteAgentScopeMismatch => "Remote-agent scope mismatch",
    }
}

const fn trust_state_token(state: TrustState) -> &'static str {
    match state {
        TrustState::Trusted => "trusted",
        TrustState::Restricted => "restricted",
        TrustState::PendingEvaluation => "pending_evaluation",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution_context::CapsuleDriftState;
    use crate::execution_context::{
        ExecutionContextRequest, ExecutionContextResolver, ExecutionContextResolverConfig,
    };

    fn baseline_config() -> ExecutionContextResolverConfig {
        ExecutionContextResolverConfig {
            workspace_id: "ws-env-inspect".to_owned(),
            profile_id: Some("prof.env-inspect".to_owned()),
            identity_mode: IdentityMode::AccountFreeLocal,
            policy_epoch: 1,
            workspace_default_target_class: TargetClass::LocalHost,
            workspace_default_working_directory: Some("/workspace".to_owned()),
            workspace_default_scope_class: ScopeClass::CurrentRoot,
            local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
            environment_capsule_ref: EnvironmentCapsuleRef {
                capsule_id: "caps:ws-env-inspect:seed".to_owned(),
                capsule_hash: "sha256:seed".to_owned(),
                resolved_schema_version: "1".to_owned(),
                drift_state: CapsuleDriftState::InSync,
            },
            resolver_version: "env-inspect-test-0".to_owned(),
        }
    }

    fn local_terminal_context() -> ExecutionContext {
        let mut resolver = ExecutionContextResolver::new(baseline_config());
        resolver.resolve(ExecutionContextRequest::local_terminal_seed(
            "terminal.open",
            TrustState::Trusted,
            "mono:0",
        ))
    }

    #[test]
    fn snapshot_carries_canonical_lane_and_target() {
        let context = local_terminal_context();
        let snapshot = EnvInspectSnapshot::from_context(&context);
        assert_eq!(snapshot.lane, ExecutionContextBetaLane::LocalHost);
        assert_eq!(snapshot.lane_token, "local_host");
        assert_eq!(snapshot.target_class, TargetClass::LocalHost);
        assert!(!snapshot.boundary_cue_visible);
        assert!(!snapshot.has_degradation());
        assert!(!snapshot.requires_review_before_dispatch());
    }

    #[test]
    fn snapshot_emits_every_section_in_canonical_order() {
        let context = local_terminal_context();
        let snapshot = EnvInspectSnapshot::from_context(&context);
        for section in EnvInspectSection::ORDER {
            assert!(
                snapshot.fields_for_section(section).next().is_some(),
                "section {} must have at least one core field",
                section.as_str()
            );
        }
        let core_token_set: Vec<&str> = snapshot
            .core_fields
            .iter()
            .map(|field| field.section_token.as_str())
            .collect();
        // Sections must appear in canonical order (no interleaving).
        let mut last_index = 0usize;
        for token in &core_token_set {
            let index = EnvInspectSection::ORDER
                .iter()
                .position(|section| section.as_str() == *token)
                .unwrap_or_else(|| panic!("unexpected section token: {token}"));
            assert!(
                index >= last_index,
                "section ordering violated: {token} appeared out of order"
            );
            last_index = index;
        }
    }

    #[test]
    fn pending_trust_lights_warning_degradation() {
        let mut resolver = ExecutionContextResolver::new(baseline_config());
        let context = resolver.resolve(ExecutionContextRequest::task_seed(
            "task.run",
            TrustState::PendingEvaluation,
            "mono:0",
        ));
        let snapshot = EnvInspectSnapshot::from_context(&context);
        assert!(snapshot.has_degradation());
        let trust_label = snapshot
            .degradation_labels
            .iter()
            .find(|label| label.reason == DegradedFieldReason::TrustStateUnresolved)
            .expect("trust degradation must be present");
        assert_eq!(trust_label.severity, EnvInspectDegradationSeverity::Warning);
        assert!(snapshot.requires_review_before_dispatch());
        assert!(!snapshot.blocks_dispatch());
    }

    #[test]
    fn capsule_drift_lights_warning_degradation() {
        let mut config = baseline_config();
        config.environment_capsule_ref.drift_state = CapsuleDriftState::StaleInputs;
        let mut resolver = ExecutionContextResolver::new(config);
        let context = resolver.resolve(ExecutionContextRequest::task_seed(
            "task.run",
            TrustState::Trusted,
            "mono:0",
        ));
        let snapshot = EnvInspectSnapshot::from_context(&context);
        assert!(snapshot.degradation_labels.iter().any(|label| label.reason
            == DegradedFieldReason::CapsuleDriftDetected
            && label.severity == EnvInspectDegradationSeverity::Warning));
    }

    #[test]
    fn render_plaintext_is_stable_across_calls() {
        let context = local_terminal_context();
        let snapshot = EnvInspectSnapshot::from_context(&context);
        let a = snapshot.render_plaintext();
        let b = snapshot.render_plaintext();
        assert_eq!(a, b);
        assert!(a.contains("env-inspect snapshot:"));
        assert!(a.contains("[target] Target"));
        assert!(a.contains("[toolchain] Toolchain"));
    }

    #[test]
    fn support_export_round_trips_through_serde() {
        let local = local_terminal_context();
        let mut resolver = ExecutionContextResolver::new(baseline_config());
        let pending = resolver.resolve(ExecutionContextRequest::remote_attach_task_seed(
            "task.run.ssh_remote",
            TargetClass::SshRemote,
            TrustState::PendingEvaluation,
            "mono:1",
        ));
        let export = EnvInspectSupportExport::from_contexts(
            "env-inspect:test:0",
            "2026-05-15T00:00:00Z",
            [&local, &pending],
        );
        let json = serde_json::to_string(&export).expect("serialize");
        let round: EnvInspectSupportExport = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round, export);
        assert_eq!(round.record_kind, ENV_INSPECT_SUPPORT_EXPORT_RECORD_KIND);
        assert_eq!(round.snapshots.len(), 2);
        assert!(round.any_degradation);
        assert!(round.any_requires_review);
        assert_eq!(
            round.redaction_class,
            EnvInspectRedactionClass::StructuredTokensOnly
        );
    }

    #[test]
    fn every_degraded_field_reason_maps_to_a_severity() {
        for reason in [
            DegradedFieldReason::ToolchainFallback,
            DegradedFieldReason::ActivatorBlockedByTrust,
            DegradedFieldReason::ActivatorBlockedByPolicy,
            DegradedFieldReason::ActivatorUnsupportedOnTarget,
            DegradedFieldReason::CapsuleUnresolved,
            DegradedFieldReason::CapsuleDriftDetected,
            DegradedFieldReason::TargetUnreachable,
            DegradedFieldReason::PolicyEpochStale,
            DegradedFieldReason::TrustStateUnresolved,
            DegradedFieldReason::WorksetMemberUnavailable,
            DegradedFieldReason::ProvenanceGap,
            DegradedFieldReason::ConfidenceLow,
            DegradedFieldReason::RemoteAgentScopeMismatch,
        ] {
            // Severity classification MUST be const-stable; assert each
            // reason has a non-empty token.
            let severity = EnvInspectDegradationSeverity::for_reason(reason);
            assert!(!severity.as_str().is_empty());
        }
    }
}
