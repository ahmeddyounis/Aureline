//! Browser / provider handoff continuity for review, CI, logs, and artifact deep links.
//!
//! This module implements the canonical M5 truth packet for the lane that keeps
//! a handoff from an in-product review, CI, log, or artifact surface out to a
//! browser tab or a provider web surface continuous and attributable: the jump
//! never loses its durable review anchor, never hides the target host / provider
//! identity, never hides how fresh the underlying truth is, never hides the
//! safe-preview trust class, and never opens write scope unless an attributable
//! handoff is cited. It binds four pillars into one export-safe record:
//!
//! - **Handoff targets** — each [`HandoffTargetRow`] names the surface a handoff
//!   leaves from (a review thread, a CI pipeline, a CI run, a log viewer, an
//!   artifact deep link, a generic target, or a provider-owned unknown) and
//!   discloses whether it supports anchored deep links, safe preview, and a
//!   provider handoff, so a handoff can never claim continuity the target does
//!   not have.
//! - **Handoff continuity rows** — each [`HandoffRow`] carries its durable review
//!   anchor, the target it leaves from, a redaction-aware subject label, an
//!   attention block, and an actor attribution with an audit row, so a handoff is
//!   always anchored, attributable, and honest about what it carries across the
//!   boundary.
//! - **Target identity** — each handoff carries a [`HandoffTargetIdentity`] with a
//!   typed destination class, a typed target trust class, a redaction-aware host
//!   label, a redaction-aware provider label, and a disclosure flag, so a handoff
//!   can never hide which destination, host, or provider it lands on.
//! - **Deep-link continuity, safe preview, and action** — each handoff carries a
//!   [`DeepLinkDisclosure`] (link exactness and truth freshness), a
//!   [`SafePreviewDisclosure`] (safe-preview trust class), and a [`HandoffAction`]
//!   (a typed action kind, a read-only flag, and a handoff ref when the action
//!   leaves the product), so a deep link is read-only navigation unless an
//!   attributable handoff is cited, and a stale truth, an unanchored link, or an
//!   unsafe preview narrows the action rather than jumping blind.
//!
//! The packet references upstream remote-preview, merge-queue, pipeline, and
//! trust-class contracts by id rather than embedding their content. Raw deep-link
//! URLs, raw host names, raw provider payloads, raw log bodies, raw artifact
//! bytes, raw absolute paths, raw author email addresses, credentials, and live
//! provider responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/review/ship-browser-provider-handoff-continuity-for-review-ci-logs-and-artifact-deep-links.schema.json`](../../../../schemas/review/ship-browser-provider-handoff-continuity-for-review-ci-logs-and-artifact-deep-links.schema.json).
//! The contract doc is
//! [`docs/review/m5/ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links.md`](../../../../docs/review/m5/ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links.md).
//! The protected fixture directory is
//! [`fixtures/review/m5/ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links/`](../../../../fixtures/review/m5/ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`HandoffContinuityPacket`].
pub const HANDOFF_CONTINUITY_RECORD_KIND: &str =
    "ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links";

/// Schema version for browser / provider handoff continuity records.
pub const HANDOFF_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const HANDOFF_CONTINUITY_SCHEMA_REF: &str =
    "schemas/review/ship-browser-provider-handoff-continuity-for-review-ci-logs-and-artifact-deep-links.schema.json";

/// Repo-relative path of the handoff continuity contract doc.
pub const HANDOFF_CONTINUITY_DOC_REF: &str =
    "docs/review/m5/ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links.md";

/// Repo-relative path of the remote-preview route contract this lane builds on.
pub const HANDOFF_CONTINUITY_REMOTE_PREVIEW_CONTRACT_REF: &str =
    "schemas/review/add-remote-preview-route-lifecycle-expiry-target-identity-and-preview-runtime-trust-disclosure.schema.json";

/// Repo-relative path of the merge-queue / CI-status contract the freshness binds to.
pub const HANDOFF_CONTINUITY_MERGE_QUEUE_CONTRACT_REF: &str =
    "schemas/review/add-merge-queue-readiness-stale-base-invalidation-and-approval-recomputation-flows.schema.json";

/// Repo-relative path of the pipeline run / log / artifact safe-preview contract this lane reuses.
pub const HANDOFF_CONTINUITY_PIPELINE_CONTRACT_REF: &str =
    "schemas/review/implement-normalized-pipeline-run-rows-log-viewers-artifact-browsers-and-safe-preview-trust-classes.schema.json";

/// Repo-relative path of the trust-class vocabulary this lane reuses.
pub const HANDOFF_CONTINUITY_TRUST_CLASS_CONTRACT_REF: &str =
    "schemas/security/trust_class.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const HANDOFF_CONTINUITY_FIXTURE_DIR: &str =
    "fixtures/review/m5/ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links";

/// Repo-relative path of the checked support-export artifact.
pub const HANDOFF_CONTINUITY_ARTIFACT_REF: &str =
    "artifacts/review/m5/ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const HANDOFF_CONTINUITY_SUMMARY_REF: &str =
    "artifacts/review/m5/ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links.md";

/// Surface a handoff leaves from.
///
/// `unknown_target_provider_owned` must never be flattened into a known target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffTargetClass {
    /// A review thread or review workspace surface.
    ReviewThread,
    /// A CI pipeline surface.
    CiPipeline,
    /// A single CI run surface.
    CiRun,
    /// A log viewer surface.
    LogViewer,
    /// An artifact deep-link surface.
    ArtifactDeepLink,
    /// A generic target not specialised further.
    GenericTarget,
    /// Provider returned a target the contract does not recognise yet.
    UnknownTargetProviderOwned,
}

impl HandoffTargetClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewThread => "review_thread",
            Self::CiPipeline => "ci_pipeline",
            Self::CiRun => "ci_run",
            Self::LogViewer => "log_viewer",
            Self::ArtifactDeepLink => "artifact_deep_link",
            Self::GenericTarget => "generic_target",
            Self::UnknownTargetProviderOwned => "unknown_target_provider_owned",
        }
    }

    /// Whether this target class needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(self, Self::UnknownTargetProviderOwned)
    }
}

/// Destination a handoff lands on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffDestinationClass {
    /// The handoff stays inside the product surface.
    InProductSurface,
    /// The handoff opens a browser tab.
    BrowserTab,
    /// The handoff opens a provider web surface.
    ProviderWebSurface,
    /// The handoff opens a native application.
    NativeApp,
    /// Provider returned a destination the contract does not recognise yet.
    UnknownDestinationProviderOwned,
}

impl HandoffDestinationClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InProductSurface => "in_product_surface",
            Self::BrowserTab => "browser_tab",
            Self::ProviderWebSurface => "provider_web_surface",
            Self::NativeApp => "native_app",
            Self::UnknownDestinationProviderOwned => "unknown_destination_provider_owned",
        }
    }

    /// Whether this destination leaves the product boundary.
    pub const fn leaves_product(self) -> bool {
        !matches!(self, Self::InProductSurface)
    }

    /// Whether this destination class needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(self, Self::UnknownDestinationProviderOwned)
    }
}

/// Trust class of a handoff target's host / provider identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetTrustClass {
    /// A first-party, trusted target.
    FirstPartyTrusted,
    /// A provider-verified target identity.
    ProviderVerified,
    /// A provider target whose identity is not verified.
    ProviderUnverified,
    /// An untrusted external target.
    UntrustedExternal,
    /// Provider returned a trust class the contract does not recognise yet.
    UnknownTrustProviderOwned,
}

impl TargetTrustClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyTrusted => "first_party_trusted",
            Self::ProviderVerified => "provider_verified",
            Self::ProviderUnverified => "provider_unverified",
            Self::UntrustedExternal => "untrusted_external",
            Self::UnknownTrustProviderOwned => "unknown_trust_provider_owned",
        }
    }

    /// Whether this trust class needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(
            self,
            Self::ProviderUnverified | Self::UntrustedExternal | Self::UnknownTrustProviderOwned
        )
    }
}

/// Exactness of the deep link that powers a handoff's continuity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkClass {
    /// The link resolves to a durable review anchor.
    AnchoredDeepLink,
    /// The link resolves to a stable, path-scoped location.
    PathScopedLink,
    /// The link resolves through a provider-owned opaque token.
    OpaqueTokenLink,
    /// The link carries no durable anchor and may not survive.
    UnanchoredLink,
    /// Provider returned a link the contract does not recognise yet.
    UnknownLinkProviderOwned,
}

impl DeepLinkClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AnchoredDeepLink => "anchored_deep_link",
            Self::PathScopedLink => "path_scoped_link",
            Self::OpaqueTokenLink => "opaque_token_link",
            Self::UnanchoredLink => "unanchored_link",
            Self::UnknownLinkProviderOwned => "unknown_link_provider_owned",
        }
    }

    /// Whether the link carries a durable, anchored continuity target.
    pub const fn is_anchored(self) -> bool {
        matches!(self, Self::AnchoredDeepLink | Self::PathScopedLink)
    }

    /// Whether this link class needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(self, Self::UnanchoredLink | Self::UnknownLinkProviderOwned)
    }
}

/// Freshness of the truth a handoff carries relative to the current source of truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffFreshnessClass {
    /// The handoff carries the current source of truth.
    FreshCurrentTruth,
    /// The handoff carries a prior, possibly stale truth.
    StalePriorTruth,
    /// Provider returned a freshness the contract does not recognise yet.
    UnknownFreshnessProviderOwned,
}

impl HandoffFreshnessClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshCurrentTruth => "fresh_current_truth",
            Self::StalePriorTruth => "stale_prior_truth",
            Self::UnknownFreshnessProviderOwned => "unknown_freshness_provider_owned",
        }
    }

    /// Whether the handoff carries the current source of truth.
    pub const fn is_fresh(self) -> bool {
        matches!(self, Self::FreshCurrentTruth)
    }

    /// Whether this freshness needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(
            self,
            Self::StalePriorTruth | Self::UnknownFreshnessProviderOwned
        )
    }
}

/// Safe-preview trust class of the handoff target content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafePreviewClass {
    /// The preview renders in a sandboxed, isolated context.
    SafePreviewSandboxed,
    /// The preview is read-only with no active content.
    SafePreviewReadOnly,
    /// The preview is unsafe and is blocked from rendering.
    UnsafePreviewBlocked,
    /// The target does not support a safe preview.
    PreviewUnsupported,
    /// Provider returned a preview class the contract does not recognise yet.
    UnknownPreviewProviderOwned,
}

impl SafePreviewClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafePreviewSandboxed => "safe_preview_sandboxed",
            Self::SafePreviewReadOnly => "safe_preview_read_only",
            Self::UnsafePreviewBlocked => "unsafe_preview_blocked",
            Self::PreviewUnsupported => "preview_unsupported",
            Self::UnknownPreviewProviderOwned => "unknown_preview_provider_owned",
        }
    }

    /// Whether the preview is safe to render in-product.
    pub const fn is_safe(self) -> bool {
        matches!(self, Self::SafePreviewSandboxed | Self::SafePreviewReadOnly)
    }

    /// Whether this preview class needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(
            self,
            Self::UnsafePreviewBlocked
                | Self::PreviewUnsupported
                | Self::UnknownPreviewProviderOwned
        )
    }
}

/// Typed handoff action kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffActionKind {
    /// Open the target inside the product; read-only navigation.
    OpenInProduct,
    /// Copy the deep link to the clipboard; read-only navigation.
    CopyDeepLink,
    /// Reveal the target in a local product surface; read-only navigation.
    RevealTargetLocal,
    /// Hand off to the browser to open the target; leaves the product and must be attributed.
    OpenInBrowserHandoff,
    /// Hand off to the provider web surface; leaves the product and must be attributed.
    OpenInProviderHandoff,
    /// No durable continuity is available, so no handoff action is offered.
    UnsupportedNoContinuity,
}

impl HandoffActionKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenInProduct => "open_in_product",
            Self::CopyDeepLink => "copy_deep_link",
            Self::RevealTargetLocal => "reveal_target_local",
            Self::OpenInBrowserHandoff => "open_in_browser_handoff",
            Self::OpenInProviderHandoff => "open_in_provider_handoff",
            Self::UnsupportedNoContinuity => "unsupported_no_continuity",
        }
    }

    /// Whether this action is read-only navigation that mutates no state.
    pub const fn is_read_only(self) -> bool {
        !matches!(
            self,
            Self::OpenInBrowserHandoff | Self::OpenInProviderHandoff
        )
    }

    /// Whether this action must cite an attributable handoff packet ref.
    pub const fn requires_handoff_ref(self) -> bool {
        matches!(
            self,
            Self::OpenInBrowserHandoff | Self::OpenInProviderHandoff
        )
    }

    /// Whether this action is the unsupported (no-continuity) action.
    pub const fn is_unsupported(self) -> bool {
        matches!(self, Self::UnsupportedNoContinuity)
    }
}

/// Why a handoff action is blocked, if it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffBlockedClass {
    /// The action is admissible.
    NotBlocked,
    /// No durable anchor is available to resolve continuity.
    BlockedNoDurableAnchor,
    /// The underlying truth is stale and a review is required first.
    BlockedStaleTruthReviewRequired,
    /// The target is untrusted and the handoff is held.
    BlockedUntrustedTarget,
    /// Policy forbids the handoff this action depends on.
    BlockedPolicyForbidsHandoff,
    /// The preview is unsafe and the handoff is held.
    BlockedUnsafePreview,
    /// The surface is offline or disconnected.
    BlockedOfflineOrDisconnected,
    /// Provider returned a block reason the contract does not recognise yet.
    BlockedUnknownReasonProviderOwned,
}

impl HandoffBlockedClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotBlocked => "not_blocked",
            Self::BlockedNoDurableAnchor => "blocked_no_durable_anchor",
            Self::BlockedStaleTruthReviewRequired => "blocked_stale_truth_review_required",
            Self::BlockedUntrustedTarget => "blocked_untrusted_target",
            Self::BlockedPolicyForbidsHandoff => "blocked_policy_forbids_handoff",
            Self::BlockedUnsafePreview => "blocked_unsafe_preview",
            Self::BlockedOfflineOrDisconnected => "blocked_offline_or_disconnected",
            Self::BlockedUnknownReasonProviderOwned => "blocked_unknown_reason_provider_owned",
        }
    }

    /// Whether the action is blocked.
    pub const fn is_blocked(self) -> bool {
        !matches!(self, Self::NotBlocked)
    }

    /// Whether this block class needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        self.is_blocked()
    }
}

/// Downgrade trigger that can narrow this lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffContinuityDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// A handoff was surfaced without attribution.
    HandoffAttributionMissing,
    /// A handoff carried a stale underlying truth.
    TruthStale,
    /// A deep link was unanchored and may not survive.
    DeepLinkUnanchored,
    /// A target host / provider identity was not disclosed.
    TargetIdentityUndisclosed,
    /// A target trust class was unknown or unverified.
    TargetTrustUnknown,
    /// A safe-preview was unsupported or unsafe.
    SafePreviewUnsupported,
    /// Workspace trust narrowed.
    TrustNarrowing,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl HandoffContinuityDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::HandoffAttributionMissing,
        Self::TruthStale,
        Self::DeepLinkUnanchored,
        Self::TargetIdentityUndisclosed,
        Self::TargetTrustUnknown,
        Self::SafePreviewUnsupported,
        Self::TrustNarrowing,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::HandoffAttributionMissing => "handoff_attribution_missing",
            Self::TruthStale => "truth_stale",
            Self::DeepLinkUnanchored => "deep_link_unanchored",
            Self::TargetIdentityUndisclosed => "target_identity_undisclosed",
            Self::TargetTrustUnknown => "target_trust_unknown",
            Self::SafePreviewUnsupported => "safe_preview_unsupported",
            Self::TrustNarrowing => "trust_narrowing",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project this lane's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffContinuityConsumerSurface {
    /// Review workspace header.
    ReviewWorkspaceHeader,
    /// Merge-queue panel.
    MergeQueuePanel,
    /// Pipeline run viewer.
    PipelineRunViewer,
    /// Log viewer.
    LogViewer,
    /// Artifact browser.
    ArtifactBrowser,
    /// Handoff action affordance.
    HandoffAction,
    /// Command palette.
    CommandPalette,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
}

impl HandoffContinuityConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ReviewWorkspaceHeader,
        Self::MergeQueuePanel,
        Self::PipelineRunViewer,
        Self::LogViewer,
        Self::ArtifactBrowser,
        Self::HandoffAction,
        Self::CommandPalette,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewWorkspaceHeader => "review_workspace_header",
            Self::MergeQueuePanel => "merge_queue_panel",
            Self::PipelineRunViewer => "pipeline_run_viewer",
            Self::LogViewer => "log_viewer",
            Self::ArtifactBrowser => "artifact_browser",
            Self::HandoffAction => "handoff_action",
            Self::CommandPalette => "command_palette",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
        }
    }
}

/// Target host / provider identity a handoff lands on.
///
/// The identity must be disclosed: a non-empty host label, a non-empty provider
/// label, and `identity_disclosed` set true, so a handoff can never hide which
/// destination, host, or provider it lands on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffTargetIdentity {
    /// Typed destination class.
    pub destination_class: HandoffDestinationClass,
    /// Typed target trust class.
    pub trust_class: TargetTrustClass,
    /// Redaction-aware host label (no raw host name).
    pub host_label: String,
    /// Redaction-aware provider label (no raw provider payload).
    pub provider_label: String,
    /// Whether the target identity is disclosed; required true.
    pub identity_disclosed: bool,
}

impl HandoffTargetIdentity {
    /// Whether the target identity is fully disclosed.
    pub fn is_disclosed(&self) -> bool {
        self.identity_disclosed
            && !self.host_label.trim().is_empty()
            && !self.provider_label.trim().is_empty()
    }

    /// Whether this identity needs at least one explicit attention reason.
    pub const fn requires_attention_reason(&self) -> bool {
        self.destination_class.requires_attention_reason()
            || self.trust_class.requires_attention_reason()
    }
}

/// Deep-link continuity disclosure for a handoff.
///
/// The disclosure must carry a non-empty link label and `link_disclosed` set
/// true, so the exactness and truth-freshness of a handoff's deep link is never
/// hidden.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeepLinkDisclosure {
    /// Typed link exactness.
    pub link_class: DeepLinkClass,
    /// Typed truth freshness.
    pub freshness_class: HandoffFreshnessClass,
    /// Whether the deep link is disclosed; required true.
    pub link_disclosed: bool,
    /// Redaction-aware link label (no raw deep-link URL).
    pub link_label: String,
}

impl DeepLinkDisclosure {
    /// Whether the deep link is fully disclosed.
    pub fn is_disclosed(&self) -> bool {
        self.link_disclosed && !self.link_label.trim().is_empty()
    }

    /// Whether this disclosure needs at least one explicit attention reason.
    pub const fn requires_attention_reason(&self) -> bool {
        self.link_class.requires_attention_reason()
            || self.freshness_class.requires_attention_reason()
    }
}

/// Safe-preview disclosure for a handoff target.
///
/// The disclosure must carry a non-empty preview label and `preview_disclosed`
/// set true, so the safe-preview trust class of a handoff target is never hidden.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafePreviewDisclosure {
    /// Typed safe-preview class.
    pub preview_class: SafePreviewClass,
    /// Whether the safe-preview class is disclosed; required true.
    pub preview_disclosed: bool,
    /// Redaction-aware preview label.
    pub preview_label: String,
}

impl SafePreviewDisclosure {
    /// Whether the safe-preview class is fully disclosed.
    pub fn is_disclosed(&self) -> bool {
        self.preview_disclosed && !self.preview_label.trim().is_empty()
    }

    /// Whether this disclosure needs at least one explicit attention reason.
    pub const fn requires_attention_reason(&self) -> bool {
        self.preview_class.requires_attention_reason()
    }
}

/// Handoff action bound to a continuity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffAction {
    /// Typed action kind.
    pub action_kind: HandoffActionKind,
    /// Whether the action is disclosed; required true.
    pub action_disclosed: bool,
    /// Whether the action is read-only navigation; must match the action kind.
    pub read_only: bool,
    /// Human-readable, redaction-aware action label.
    pub action_label: String,
    /// Handoff packet ref; required when the action kind leaves the product.
    pub handoff_ref: Option<String>,
}

impl HandoffAction {
    /// Whether the action is fully disclosed.
    pub fn is_disclosed(&self) -> bool {
        self.action_disclosed && !self.action_label.trim().is_empty()
    }

    /// Whether the read-only flag matches the action kind.
    pub fn read_only_flag_consistent(&self) -> bool {
        self.read_only == self.action_kind.is_read_only()
    }
}

/// One handoff target row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffTargetRow {
    /// Stable target id handoffs reference.
    pub target_id: String,
    /// Typed target class.
    pub target_class: HandoffTargetClass,
    /// Human-readable target label.
    pub target_label: String,
    /// Whether the target supports anchored deep links.
    pub supports_deep_link: bool,
    /// Whether the target supports a safe preview.
    pub supports_safe_preview: bool,
    /// Whether the target supports a provider handoff.
    pub supports_provider_handoff: bool,
    /// Human-readable coverage label.
    pub coverage_label: String,
    /// Human-readable disclosure label.
    pub disclosure_label: String,
}

/// One handoff continuity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffRow {
    /// Stable handoff id.
    pub handoff_id: String,
    /// Durable review anchor id bound to this handoff.
    pub durable_anchor_id: String,
    /// Target id this handoff leaves from.
    pub target_id: String,
    /// Human-readable subject label (what is being handed off).
    pub subject_label: String,
    /// Target host / provider identity this handoff lands on.
    pub target_identity: HandoffTargetIdentity,
    /// Deep-link continuity disclosure.
    pub deep_link: DeepLinkDisclosure,
    /// Safe-preview disclosure for the target.
    pub safe_preview: SafePreviewDisclosure,
    /// Handoff action bound to the continuity row.
    pub handoff_action: HandoffAction,
    /// Why the handoff action is blocked, if it is.
    pub blocked_class: HandoffBlockedClass,
    /// Human-readable actor attribution (under whose authority the handoff fired).
    pub actor_attribution_label: String,
    /// Opaque ref to the audit row that lands when the handoff action fires.
    pub audit_row_ref: String,
    /// Attention reasons; required and non-empty when the handoff needs attention.
    pub attention_reasons: Vec<String>,
    /// Human-readable review summary.
    pub review_summary: String,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl HandoffRow {
    /// Whether this handoff needs at least one explicit attention reason.
    pub const fn requires_attention_reason(&self) -> bool {
        self.target_identity.requires_attention_reason()
            || self.deep_link.requires_attention_reason()
            || self.safe_preview.requires_attention_reason()
            || self.blocked_class.requires_attention_reason()
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffContinuityTrustReview {
    /// The target host / provider identity is disclosed, never hidden.
    pub target_identity_disclosed: bool,
    /// The target trust class is explicit, never assumed.
    pub target_trust_explicit: bool,
    /// The deep link is disclosed, never hidden.
    pub deep_link_disclosed: bool,
    /// The truth freshness is disclosed.
    pub truth_freshness_disclosed: bool,
    /// The safe-preview class is disclosed, never hidden.
    pub safe_preview_disclosed: bool,
    /// The handoff action is disclosed, never hidden.
    pub handoff_action_disclosed: bool,
    /// A handoff action is read-only unless an attributable handoff is cited.
    pub handoff_read_only_unless_attributed: bool,
    /// Every handoff is bound to a durable review anchor.
    pub every_handoff_anchored: bool,
    /// Every handoff action is attributable to an actor.
    pub every_action_attributable: bool,
    /// No handoff action creates hidden write scope.
    pub no_hidden_write_scope: bool,
    /// A stale truth narrows the handoff action rather than jumping blind.
    pub stale_truth_narrows_action: bool,
    /// Downgrade narrows the claim rather than hiding the lane.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl HandoffContinuityTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.target_identity_disclosed
            && self.target_trust_explicit
            && self.deep_link_disclosed
            && self.truth_freshness_disclosed
            && self.safe_preview_disclosed
            && self.handoff_action_disclosed
            && self.handoff_read_only_unless_attributed
            && self.every_handoff_anchored
            && self.every_action_attributable
            && self.no_hidden_write_scope
            && self.stale_truth_narrows_action
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffContinuityConsumerProjection {
    /// Review workspace header shows the durable anchor.
    pub review_workspace_header_shows_anchor: bool,
    /// Merge-queue panel shows the truth freshness.
    pub merge_queue_panel_shows_freshness: bool,
    /// Pipeline run viewer shows the target identity.
    pub pipeline_run_viewer_shows_target_identity: bool,
    /// Log viewer shows the safe-preview class.
    pub log_viewer_shows_safe_preview: bool,
    /// Artifact browser shows the deep-link class.
    pub artifact_browser_shows_deep_link: bool,
    /// Handoff action shows the target trust class.
    pub handoff_action_shows_trust: bool,
    /// Command palette shows the handoff state.
    pub command_palette_shows_handoff_state: bool,
    /// CLI / headless shows qualification truth.
    pub cli_headless_shows_truth: bool,
    /// Support export shows qualification truth.
    pub support_export_shows_truth: bool,
    /// Diagnostics shows qualification truth.
    pub diagnostics_shows_truth: bool,
    /// Review / CI / preview lanes are labeled when not covered by this packet.
    pub label_for_unqualified: bool,
}

impl HandoffContinuityConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.review_workspace_header_shows_anchor
            && self.merge_queue_panel_shows_freshness
            && self.pipeline_run_viewer_shows_target_identity
            && self.log_viewer_shows_safe_preview
            && self.artifact_browser_shows_deep_link
            && self.handoff_action_shows_trust
            && self.command_palette_shows_handoff_state
            && self.cli_headless_shows_truth
            && self.support_export_shows_truth
            && self.diagnostics_shows_truth
            && self.label_for_unqualified
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffContinuityProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`HandoffContinuityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandoffContinuityPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Handoff target rows.
    pub target_rows: Vec<HandoffTargetRow>,
    /// Handoff continuity rows.
    pub handoff_rows: Vec<HandoffRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<HandoffContinuityDowngradeTrigger>,
    /// Consumer surfaces that must project this lane's truth.
    pub consumer_surfaces: Vec<HandoffContinuityConsumerSurface>,
    /// Trust review block.
    pub trust_review: HandoffContinuityTrustReview,
    /// Consumer projection block.
    pub consumer_projection: HandoffContinuityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: HandoffContinuityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe browser / provider handoff continuity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffContinuityPacket {
    /// Record kind; must equal [`HANDOFF_CONTINUITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`HANDOFF_CONTINUITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Handoff target rows.
    pub target_rows: Vec<HandoffTargetRow>,
    /// Handoff continuity rows.
    pub handoff_rows: Vec<HandoffRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<HandoffContinuityDowngradeTrigger>,
    /// Consumer surfaces that must project this lane's truth.
    pub consumer_surfaces: Vec<HandoffContinuityConsumerSurface>,
    /// Trust review block.
    pub trust_review: HandoffContinuityTrustReview,
    /// Consumer projection block.
    pub consumer_projection: HandoffContinuityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: HandoffContinuityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl HandoffContinuityPacket {
    /// Builds a handoff continuity packet from stable-lane input.
    pub fn new(input: HandoffContinuityPacketInput) -> Self {
        Self {
            record_kind: HANDOFF_CONTINUITY_RECORD_KIND.to_owned(),
            schema_version: HANDOFF_CONTINUITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            target_rows: input.target_rows,
            handoff_rows: input.handoff_rows,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the handoff continuity review invariants.
    pub fn validate(&self) -> Vec<HandoffContinuityViolation> {
        let mut violations = Vec::new();

        if self.record_kind != HANDOFF_CONTINUITY_RECORD_KIND {
            violations.push(HandoffContinuityViolation::WrongRecordKind);
        }
        if self.schema_version != HANDOFF_CONTINUITY_SCHEMA_VERSION {
            violations.push(HandoffContinuityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(HandoffContinuityViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(HandoffContinuityViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(HandoffContinuityViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_target_rows(self, &mut violations);
        validate_handoff_rows(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(HandoffContinuityViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(HandoffContinuityViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(HandoffContinuityViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("handoff continuity packet serializes"),
        ) {
            violations.push(HandoffContinuityViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("handoff continuity packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let leaving_product = self
            .handoff_rows
            .iter()
            .filter(|row| row.target_identity.destination_class.leaves_product())
            .count();
        let blocked_actions = self
            .handoff_rows
            .iter()
            .filter(|row| row.blocked_class.is_blocked())
            .count();
        let stale_truths = self
            .handoff_rows
            .iter()
            .filter(|row| !row.deep_link.freshness_class.is_fresh())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Browser/Provider Handoff Continuity for Review, CI, Logs, and Artifact Deep Links\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!("- Handoff targets: {}\n", self.target_rows.len()));
        out.push_str(&format!(
            "- Handoffs: {} ({} leaving product, {} blocked actions, {} stale truths)\n",
            self.handoff_rows.len(),
            leaving_product,
            blocked_actions,
            stale_truths
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Handoff targets\n\n");
        for row in &self.target_rows {
            out.push_str(&format!(
                "- **{}** (`{}`): deep-link {}, safe-preview {}, provider-handoff {}\n",
                row.target_label,
                row.target_class.as_str(),
                row.supports_deep_link,
                row.supports_safe_preview,
                row.supports_provider_handoff
            ));
        }

        out.push_str("\n## Handoffs\n\n");
        for row in &self.handoff_rows {
            out.push_str(&format!(
                "- **{}** on target `{}` → anchor `{}`: destination `{}`, trust `{}`, link `{}`/`{}`, preview `{}`, action `{}`, blocked `{}`, authority `{}`\n",
                row.subject_label,
                row.target_id,
                row.durable_anchor_id,
                row.target_identity.destination_class.as_str(),
                row.target_identity.trust_class.as_str(),
                row.deep_link.link_class.as_str(),
                row.deep_link.freshness_class.as_str(),
                row.safe_preview.preview_class.as_str(),
                row.handoff_action.action_kind.as_str(),
                row.blocked_class.as_str(),
                row.actor_attribution_label
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in handoff continuity export.
#[derive(Debug)]
pub enum HandoffContinuityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<HandoffContinuityViolation>),
}

impl fmt::Display for HandoffContinuityArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "handoff continuity export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "handoff continuity export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for HandoffContinuityArtifactError {}

/// Validation failures emitted by [`HandoffContinuityPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HandoffContinuityViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No handoff target rows are present.
    TargetRowsMissing,
    /// A handoff target row is incomplete.
    TargetRowIncomplete,
    /// No handoff rows are present.
    HandoffRowsMissing,
    /// A handoff row is incomplete.
    HandoffRowIncomplete,
    /// A handoff references a target id with no target row.
    OrphanTargetReference,
    /// A handoff's target host / provider identity is undisclosed.
    TargetIdentityUndisclosed,
    /// A handoff's deep link is undisclosed.
    DeepLinkUndisclosed,
    /// A handoff's safe-preview class is undisclosed.
    SafePreviewUndisclosed,
    /// A handoff's action is undisclosed.
    HandoffActionUndisclosed,
    /// A handoff action's read-only flag does not match its kind.
    HandoffActionReadOnlyMismatch,
    /// A handoff action that leaves the product is missing its handoff ref.
    HandoffRefMissing,
    /// An unsupported handoff action is not blocked for it.
    UnsupportedHandoffNotBlocked,
    /// A handoff is missing its actor attribution or audit row.
    AttributionMissing,
    /// A handoff needing attention is missing its attention reasons.
    AttentionReasonMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
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

impl HandoffContinuityViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::TargetRowsMissing => "target_rows_missing",
            Self::TargetRowIncomplete => "target_row_incomplete",
            Self::HandoffRowsMissing => "handoff_rows_missing",
            Self::HandoffRowIncomplete => "handoff_row_incomplete",
            Self::OrphanTargetReference => "orphan_target_reference",
            Self::TargetIdentityUndisclosed => "target_identity_undisclosed",
            Self::DeepLinkUndisclosed => "deep_link_undisclosed",
            Self::SafePreviewUndisclosed => "safe_preview_undisclosed",
            Self::HandoffActionUndisclosed => "handoff_action_undisclosed",
            Self::HandoffActionReadOnlyMismatch => "handoff_action_read_only_mismatch",
            Self::HandoffRefMissing => "handoff_ref_missing",
            Self::UnsupportedHandoffNotBlocked => "unsupported_handoff_not_blocked",
            Self::AttributionMissing => "attribution_missing",
            Self::AttentionReasonMissing => "attention_reason_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable handoff continuity export.
pub fn current_handoff_continuity_export(
) -> Result<HandoffContinuityPacket, HandoffContinuityArtifactError> {
    let packet: HandoffContinuityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links/support_export.json"
    )))
    .map_err(HandoffContinuityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(HandoffContinuityArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &HandoffContinuityPacket,
    violations: &mut Vec<HandoffContinuityViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        HANDOFF_CONTINUITY_SCHEMA_REF,
        HANDOFF_CONTINUITY_DOC_REF,
        HANDOFF_CONTINUITY_REMOTE_PREVIEW_CONTRACT_REF,
        HANDOFF_CONTINUITY_MERGE_QUEUE_CONTRACT_REF,
        HANDOFF_CONTINUITY_PIPELINE_CONTRACT_REF,
        HANDOFF_CONTINUITY_TRUST_CLASS_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(HandoffContinuityViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_target_rows(
    packet: &HandoffContinuityPacket,
    violations: &mut Vec<HandoffContinuityViolation>,
) {
    if packet.target_rows.is_empty() {
        violations.push(HandoffContinuityViolation::TargetRowsMissing);
        return;
    }

    for row in &packet.target_rows {
        if row.target_id.trim().is_empty()
            || row.target_label.trim().is_empty()
            || row.coverage_label.trim().is_empty()
            || row.disclosure_label.trim().is_empty()
        {
            violations.push(HandoffContinuityViolation::TargetRowIncomplete);
        }
    }
}

fn validate_handoff_rows(
    packet: &HandoffContinuityPacket,
    violations: &mut Vec<HandoffContinuityViolation>,
) {
    if packet.handoff_rows.is_empty() {
        violations.push(HandoffContinuityViolation::HandoffRowsMissing);
        return;
    }

    let target_ids: BTreeSet<&str> = packet
        .target_rows
        .iter()
        .map(|row| row.target_id.as_str())
        .collect();

    for row in &packet.handoff_rows {
        if row.handoff_id.trim().is_empty()
            || row.durable_anchor_id.trim().is_empty()
            || row.subject_label.trim().is_empty()
            || row.review_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(HandoffContinuityViolation::HandoffRowIncomplete);
        }
        if !row.target_id.trim().is_empty() && !target_ids.contains(row.target_id.as_str()) {
            violations.push(HandoffContinuityViolation::OrphanTargetReference);
        }
        if !row.target_identity.is_disclosed() {
            violations.push(HandoffContinuityViolation::TargetIdentityUndisclosed);
        }
        if !row.deep_link.is_disclosed() {
            violations.push(HandoffContinuityViolation::DeepLinkUndisclosed);
        }
        if !row.safe_preview.is_disclosed() {
            violations.push(HandoffContinuityViolation::SafePreviewUndisclosed);
        }
        validate_handoff_action(row, violations);
        if row.actor_attribution_label.trim().is_empty() || row.audit_row_ref.trim().is_empty() {
            violations.push(HandoffContinuityViolation::AttributionMissing);
        }
        if row.requires_attention_reason() && row.attention_reasons.is_empty() {
            violations.push(HandoffContinuityViolation::AttentionReasonMissing);
        }
    }
}

fn validate_handoff_action(row: &HandoffRow, violations: &mut Vec<HandoffContinuityViolation>) {
    let action = &row.handoff_action;
    if !action.is_disclosed() {
        violations.push(HandoffContinuityViolation::HandoffActionUndisclosed);
    }
    if !action.read_only_flag_consistent() {
        violations.push(HandoffContinuityViolation::HandoffActionReadOnlyMismatch);
    }
    if action.action_kind.requires_handoff_ref()
        && !action
            .handoff_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty())
    {
        violations.push(HandoffContinuityViolation::HandoffRefMissing);
    }
    if action.action_kind.is_unsupported() && !row.blocked_class.is_blocked() {
        violations.push(HandoffContinuityViolation::UnsupportedHandoffNotBlocked);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret ")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
