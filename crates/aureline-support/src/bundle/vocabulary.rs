//! Frozen string tokens shared by the support-bundle manifest, the shell
//! preview copy, and the support/export documentation.
//!
//! Every enum here mirrors a closed enum in
//! `/schemas/support/support_bundle_manifest.schema.json` or
//! `/schemas/support/support_bundle_preview_item.schema.json`. The `as_str`
//! accessors are the one place the in-shell projection and the export
//! manifest resolve to the same canonical token; see
//! `/docs/support/support_export_vocabulary_seed.md` for the reviewer-
//! facing word list.

use serde::{Deserialize, Serialize};

/// Shared diagnostic risk class for every preview row.
///
/// Mirrors `diagnostic_data_class` in the boundary schemas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticDataClass {
    /// Build ids, version, policy fingerprints, summary counters.
    MetadataOnly,
    /// Toolchain versions, target classes, route summaries.
    EnvironmentAdjacent,
    /// Filenames, stack traces, snippets, command-argument summaries.
    CodeAdjacent,
    /// Secret-bearing material, raw dumps, full transcripts.
    HighRisk,
}

impl DiagnosticDataClass {
    /// Stable string token used in manifests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata_only",
            Self::EnvironmentAdjacent => "environment_adjacent",
            Self::CodeAdjacent => "code_adjacent",
            Self::HighRisk => "high_risk",
        }
    }
}

/// Required high-risk subtype whenever [`DiagnosticDataClass::HighRisk`] is
/// declared. Mirrors `high_risk_content_class` in the boundary schemas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HighRiskContentClass {
    NotApplicable,
    SecretBearing,
    RawDumpOrMemory,
    FullShellHistory,
    RawTraceOrTranscript,
    PolicyProhibitedUnknown,
}

impl HighRiskContentClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::SecretBearing => "secret_bearing",
            Self::RawDumpOrMemory => "raw_dump_or_memory",
            Self::FullShellHistory => "full_shell_history",
            Self::RawTraceOrTranscript => "raw_trace_or_transcript",
            Self::PolicyProhibitedUnknown => "policy_prohibited_unknown",
        }
    }
}

/// Visible state shown to the reviewer before export. Mirrors
/// `redaction_state`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionState {
    NotRequiredMetadata,
    RedactedSummary,
    SanitizedSnapshot,
    RetainedLocalOnly,
    OmittedPendingOptIn,
    Prohibited,
    PolicyLocked,
}

impl RedactionState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequiredMetadata => "not_required_metadata",
            Self::RedactedSummary => "redacted_summary",
            Self::SanitizedSnapshot => "sanitized_snapshot",
            Self::RetainedLocalOnly => "retained_local_only",
            Self::OmittedPendingOptIn => "omitted_pending_opt_in",
            Self::Prohibited => "prohibited",
            Self::PolicyLocked => "policy_locked",
        }
    }

    /// Reviewer-facing label shown next to the redaction chip.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotRequiredMetadata => "Metadata only — no redaction needed",
            Self::RedactedSummary => "Redacted summary",
            Self::SanitizedSnapshot => "Sanitized snapshot",
            Self::RetainedLocalOnly => "Retained local only — not exported",
            Self::OmittedPendingOptIn => "Omitted, awaiting opt-in",
            Self::Prohibited => "Prohibited — never exported",
            Self::PolicyLocked => "Policy locked",
        }
    }
}

/// Mirrors `actionability_impact_class`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionabilityImpactClass {
    None,
    Low,
    Medium,
    High,
    BlocksFirstActionableDiagnosis,
}

impl ActionabilityImpactClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::BlocksFirstActionableDiagnosis => "blocks_first_actionable_diagnosis",
        }
    }
}

/// Mirrors `policy_note_severity`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyNoteSeverity {
    Info,
    Warning,
    Blocking,
}

impl PolicyNoteSeverity {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Blocking => "blocking",
        }
    }
}

/// Mirrors `review_decision_class`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDecisionClass {
    IncludedDefault,
    IncludedAfterOptIn,
    OmittedUserDeselected,
    OmittedPolicyLocked,
    OmittedProhibited,
    StrongerRedactionApplied,
    RetainedLocalOnly,
}

impl ReviewDecisionClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncludedDefault => "included_default",
            Self::IncludedAfterOptIn => "included_after_opt_in",
            Self::OmittedUserDeselected => "omitted_user_deselected",
            Self::OmittedPolicyLocked => "omitted_policy_locked",
            Self::OmittedProhibited => "omitted_prohibited",
            Self::StrongerRedactionApplied => "stronger_redaction_applied",
            Self::RetainedLocalOnly => "retained_local_only",
        }
    }
}

/// Mirrors `review_decision.decided_by_class`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDecidedByClass {
    DefaultRule,
    UserChoice,
    AdminPolicy,
    ProhibitedClassRule,
}

impl ReviewDecidedByClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefaultRule => "default_rule",
            Self::UserChoice => "user_choice",
            Self::AdminPolicy => "admin_policy",
            Self::ProhibitedClassRule => "prohibited_class_rule",
        }
    }
}

/// Mirrors `excluded_reason_class`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExcludedReasonClass {
    NotRequested,
    UserDeselected,
    PolicyDenied,
    ProhibitedSecretOrToken,
    ProhibitedFullShellHistory,
    RetainedLocalOnlyPendingReview,
    AwaitingExplicitOptIn,
    SourceUnavailableOrExpired,
    NotCollectedOnThisPlatform,
}

impl ExcludedReasonClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequested => "not_requested",
            Self::UserDeselected => "user_deselected",
            Self::PolicyDenied => "policy_denied",
            Self::ProhibitedSecretOrToken => "prohibited_secret_or_token",
            Self::ProhibitedFullShellHistory => "prohibited_full_shell_history",
            Self::RetainedLocalOnlyPendingReview => "retained_local_only_pending_review",
            Self::AwaitingExplicitOptIn => "awaiting_explicit_opt_in",
            Self::SourceUnavailableOrExpired => "source_unavailable_or_expired",
            Self::NotCollectedOnThisPlatform => "not_collected_on_this_platform",
        }
    }
}

/// Mirrors `secret_scan_summary.scan_state`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretScanState {
    NotApplicableMetadataOnly,
    PassedNoMarkers,
    PassedWithRedactionMarkers,
    BlockedByPolicy,
    ManualReviewRequired,
}

impl SecretScanState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicableMetadataOnly => "not_applicable_metadata_only",
            Self::PassedNoMarkers => "passed_no_markers",
            Self::PassedWithRedactionMarkers => "passed_with_redaction_markers",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::ManualReviewRequired => "manual_review_required",
        }
    }
}

/// Mirrors `actor_class`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorClass {
    UserInitiated,
    AdminInitiated,
    HeadlessCli,
    SupportCenterPreview,
}

impl ActorClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserInitiated => "user_initiated",
            Self::AdminInitiated => "admin_initiated",
            Self::HeadlessCli => "headless_cli",
            Self::SupportCenterPreview => "support_center_preview",
        }
    }
}

/// Mirrors `release_channel_class`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseChannelClass {
    Stable,
    Preview,
    Beta,
    Lts,
    PortableStable,
    PortablePreview,
    DevLocal,
}

impl ReleaseChannelClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Lts => "lts",
            Self::PortableStable => "portable_stable",
            Self::PortablePreview => "portable_preview",
            Self::DevLocal => "dev_local",
        }
    }

    /// Map a build-info channel token onto a stable enum value. Tokens
    /// outside the manifest schema's vocabulary settle on
    /// [`ReleaseChannelClass::DevLocal`] so the manifest never silently
    /// labels an unknown channel as `Stable`.
    pub fn from_build_token(token: &str) -> Self {
        match token {
            "stable" => Self::Stable,
            "preview" => Self::Preview,
            "beta" => Self::Beta,
            "lts" => Self::Lts,
            "portable_stable" => Self::PortableStable,
            "portable_preview" => Self::PortablePreview,
            "dev_local" => Self::DevLocal,
            _ => Self::DevLocal,
        }
    }
}

/// Mirrors `policy_context.trust_state`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustState {
    Untrusted,
    Restricted,
    Trusted,
    ManagedAdmin,
}

impl TrustState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Untrusted => "untrusted",
            Self::Restricted => "restricted",
            Self::Trusted => "trusted",
            Self::ManagedAdmin => "managed_admin",
        }
    }
}
