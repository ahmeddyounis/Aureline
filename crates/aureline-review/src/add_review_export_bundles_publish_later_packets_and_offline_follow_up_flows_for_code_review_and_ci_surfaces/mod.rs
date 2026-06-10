//! Review/export bundles, publish-later packets, and offline follow-up flows for code review and CI surfaces.
//!
//! This module implements the canonical M5 truth packet for the lane that lets a
//! code-review or CI surface gather its state into an export-safe bundle, queue a
//! publish for later, and queue follow-up actions while offline — without ever
//! losing the durable review anchor, hiding the bundle's provenance, hiding how
//! fresh the bundled truth is, hiding the export redaction class, or opening
//! hidden publish scope. A publish stays a read-only draft unless an attributable
//! publish is cited, a stale truth narrows the publish rather than shipping a
//! possibly-wrong state, and an offline follow-up is held for review rather than
//! auto-firing on reconnect. It binds four pillars into one export-safe record:
//!
//! - **Export bundles** — each [`ExportBundleRow`] names the source a bundle is
//!   gathered from (a review thread, a CI run, a pipeline, a mixed review + CI
//!   bundle, a generic bundle, or a provider-owned unknown) and discloses whether
//!   the bundle supports publish-later, offline replay, and a redacted export, so
//!   an export can never claim a capability the bundle does not have.
//! - **Bundle export rows** — each [`BundleExportRow`] carries its durable review
//!   anchor, the bundle it is gathered from, a redaction-aware subject label, an
//!   attention block, and an actor attribution with an audit row, so every export,
//!   publish, and follow-up is anchored, attributable, and honest about what it
//!   carries across the boundary.
//! - **Provenance and redaction** — each export carries a [`BundleProvenance`]
//!   (typed scope class, trust class, freshness class, source label, disclosure
//!   flag) and a [`BundleRedactionDisclosure`] (typed redaction class), so an
//!   export can never hide which source it bundles, how fresh that source is, or
//!   how redacted the exported bytes are.
//! - **Publish-later and offline follow-up** — each export carries a
//!   [`PublishDisposition`] (typed publish state, a read-only flag, and a publish
//!   ref when the publish commits externally) and a [`FollowUpAction`] (typed
//!   connectivity class, disposition class, and a replay-ready flag), so a publish
//!   is a read-only draft unless an attributable publish is cited, and an offline
//!   follow-up can never be pre-authorized to auto-fire — replay-ready is reserved
//!   for an online, reconnected surface.
//!
//! The packet references upstream handoff-continuity, merge-queue, pipeline, and
//! trust-class contracts by id rather than embedding their content. Raw export
//! bytes, raw bundle payloads, raw provider payloads, raw log bodies, raw artifact
//! bytes, raw absolute paths, raw author email addresses, credentials, and live
//! provider responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/review/add-review-export-bundles-publish-later-packets-and-offline-follow-up-flows-for-code-review-and-ci-surfaces.schema.json`](../../../../schemas/review/add-review-export-bundles-publish-later-packets-and-offline-follow-up-flows-for-code-review-and-ci-surfaces.schema.json).
//! The contract doc is
//! [`docs/review/m5/add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces.md`](../../../../docs/review/m5/add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces.md).
//! The protected fixture directory is
//! [`fixtures/review/m5/add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces/`](../../../../fixtures/review/m5/add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`ReviewExportBundlePacket`].
pub const REVIEW_EXPORT_BUNDLE_RECORD_KIND: &str =
    "add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces";

/// Schema version for review/export bundle records.
pub const REVIEW_EXPORT_BUNDLE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const REVIEW_EXPORT_BUNDLE_SCHEMA_REF: &str =
    "schemas/review/add-review-export-bundles-publish-later-packets-and-offline-follow-up-flows-for-code-review-and-ci-surfaces.schema.json";

/// Repo-relative path of the review/export bundle contract doc.
pub const REVIEW_EXPORT_BUNDLE_DOC_REF: &str =
    "docs/review/m5/add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces.md";

/// Repo-relative path of the browser/provider handoff continuity contract this lane builds on.
pub const REVIEW_EXPORT_BUNDLE_HANDOFF_CONTRACT_REF: &str =
    "schemas/review/ship-browser-provider-handoff-continuity-for-review-ci-logs-and-artifact-deep-links.schema.json";

/// Repo-relative path of the merge-queue / CI-status contract the freshness binds to.
pub const REVIEW_EXPORT_BUNDLE_MERGE_QUEUE_CONTRACT_REF: &str =
    "schemas/review/add-merge-queue-readiness-stale-base-invalidation-and-approval-recomputation-flows.schema.json";

/// Repo-relative path of the pipeline run / log / artifact safe-preview contract this lane reuses.
pub const REVIEW_EXPORT_BUNDLE_PIPELINE_CONTRACT_REF: &str =
    "schemas/review/implement-normalized-pipeline-run-rows-log-viewers-artifact-browsers-and-safe-preview-trust-classes.schema.json";

/// Repo-relative path of the trust-class vocabulary this lane reuses.
pub const REVIEW_EXPORT_BUNDLE_TRUST_CLASS_CONTRACT_REF: &str =
    "schemas/security/trust_class.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const REVIEW_EXPORT_BUNDLE_FIXTURE_DIR: &str =
    "fixtures/review/m5/add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces";

/// Repo-relative path of the checked support-export artifact.
pub const REVIEW_EXPORT_BUNDLE_ARTIFACT_REF: &str =
    "artifacts/review/m5/add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const REVIEW_EXPORT_BUNDLE_SUMMARY_REF: &str =
    "artifacts/review/m5/add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces.md";

/// Source a bundle is gathered from.
///
/// `unknown_scope_provider_owned` must never be flattened into a known scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleScopeClass {
    /// A review thread or review workspace bundle.
    ReviewThreadBundle,
    /// A single CI run bundle.
    CiRunBundle,
    /// A CI pipeline bundle.
    PipelineBundle,
    /// A bundle that mixes review and CI sources.
    MixedReviewCiBundle,
    /// A generic bundle not specialised further.
    GenericBundle,
    /// Provider returned a scope the contract does not recognise yet.
    UnknownScopeProviderOwned,
}

impl BundleScopeClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewThreadBundle => "review_thread_bundle",
            Self::CiRunBundle => "ci_run_bundle",
            Self::PipelineBundle => "pipeline_bundle",
            Self::MixedReviewCiBundle => "mixed_review_ci_bundle",
            Self::GenericBundle => "generic_bundle",
            Self::UnknownScopeProviderOwned => "unknown_scope_provider_owned",
        }
    }

    /// Whether this scope class needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(self, Self::UnknownScopeProviderOwned)
    }
}

/// Trust class of a bundle's source identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleTrustClass {
    /// A first-party, trusted source.
    FirstPartyTrusted,
    /// A provider-verified source identity.
    ProviderVerified,
    /// A provider source whose identity is not verified.
    ProviderUnverified,
    /// An untrusted external source.
    UntrustedExternal,
    /// Provider returned a trust class the contract does not recognise yet.
    UnknownTrustProviderOwned,
}

impl BundleTrustClass {
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

/// Freshness of the truth a bundle carries relative to the current source of truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleFreshnessClass {
    /// The bundle carries the current source of truth.
    FreshCurrentTruth,
    /// The bundle carries a prior, possibly stale truth.
    StalePriorTruth,
    /// Provider returned a freshness the contract does not recognise yet.
    UnknownFreshnessProviderOwned,
}

impl BundleFreshnessClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshCurrentTruth => "fresh_current_truth",
            Self::StalePriorTruth => "stale_prior_truth",
            Self::UnknownFreshnessProviderOwned => "unknown_freshness_provider_owned",
        }
    }

    /// Whether the bundle carries the current source of truth.
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

/// Redaction class of a bundle's exported bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleRedactionClass {
    /// The export is fully redacted and safe to ship anywhere.
    FullyRedactedSafe,
    /// The export carries metadata only, no bodies.
    MetadataOnly,
    /// The export is partially redacted and needs a review before publish.
    PartialRedactionReviewRequired,
    /// The export is unredacted and blocked from leaving the boundary.
    UnredactedBlocked,
    /// Provider returned a redaction class the contract does not recognise yet.
    UnknownRedactionProviderOwned,
}

impl BundleRedactionClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyRedactedSafe => "fully_redacted_safe",
            Self::MetadataOnly => "metadata_only",
            Self::PartialRedactionReviewRequired => "partial_redaction_review_required",
            Self::UnredactedBlocked => "unredacted_blocked",
            Self::UnknownRedactionProviderOwned => "unknown_redaction_provider_owned",
        }
    }

    /// Whether the export is safe to leave the boundary.
    pub const fn is_export_safe(self) -> bool {
        matches!(self, Self::FullyRedactedSafe | Self::MetadataOnly)
    }

    /// Whether this redaction class needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(
            self,
            Self::PartialRedactionReviewRequired
                | Self::UnredactedBlocked
                | Self::UnknownRedactionProviderOwned
        )
    }
}

/// Publish state of a publish-later disposition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishStateClass {
    /// The bundle is held as a read-only draft; no publish scope is open.
    HeldDraft,
    /// The bundle is queued to publish later; commits externally and must be attributed.
    QueuedToPublish,
    /// The bundle is scheduled to publish at a later time; commits externally and must be attributed.
    ScheduledPublish,
    /// The bundle has been published; commits externally and must be attributed.
    Published,
    /// The publish is blocked; no publish scope is open.
    PublishBlocked,
    /// Provider returned a publish state the contract does not recognise yet.
    UnknownPublishProviderOwned,
}

impl PublishStateClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HeldDraft => "held_draft",
            Self::QueuedToPublish => "queued_to_publish",
            Self::ScheduledPublish => "scheduled_publish",
            Self::Published => "published",
            Self::PublishBlocked => "publish_blocked",
            Self::UnknownPublishProviderOwned => "unknown_publish_provider_owned",
        }
    }

    /// Whether this publish state commits externally and must cite a publish ref.
    pub const fn requires_publish_ref(self) -> bool {
        matches!(
            self,
            Self::QueuedToPublish | Self::ScheduledPublish | Self::Published
        )
    }

    /// Whether this publish state is read-only and opens no publish scope.
    pub const fn is_read_only(self) -> bool {
        !self.requires_publish_ref()
    }

    /// Whether this publish state needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(
            self,
            Self::PublishBlocked | Self::UnknownPublishProviderOwned
        )
    }
}

/// Connectivity state of an offline follow-up flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FollowUpConnectivityClass {
    /// The surface is online and reconnected.
    Online,
    /// The follow-up was queued while offline.
    OfflineQueued,
    /// The surface is reconnecting and not yet online.
    Reconnecting,
    /// Provider returned a connectivity the contract does not recognise yet.
    UnknownConnectivityProviderOwned,
}

impl FollowUpConnectivityClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Online => "online",
            Self::OfflineQueued => "offline_queued",
            Self::Reconnecting => "reconnecting",
            Self::UnknownConnectivityProviderOwned => "unknown_connectivity_provider_owned",
        }
    }

    /// Whether the surface is online and a replay may be authorized.
    pub const fn is_online(self) -> bool {
        matches!(self, Self::Online)
    }

    /// Whether this connectivity needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(
            self,
            Self::OfflineQueued | Self::Reconnecting | Self::UnknownConnectivityProviderOwned
        )
    }
}

/// Disposition of an offline follow-up flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FollowUpDispositionClass {
    /// No follow-up is pending for this export.
    NoPendingFollowUp,
    /// The follow-up will replay once the surface reconnects and is authorized.
    ReplayOnReconnect,
    /// The follow-up is held for an explicit review before it can run.
    HoldForReview,
    /// The follow-up was discarded.
    Discarded,
    /// The follow-up is blocked pending fresh truth.
    BlockedPendingTruth,
    /// Provider returned a disposition the contract does not recognise yet.
    UnknownDispositionProviderOwned,
}

impl FollowUpDispositionClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPendingFollowUp => "no_pending_follow_up",
            Self::ReplayOnReconnect => "replay_on_reconnect",
            Self::HoldForReview => "hold_for_review",
            Self::Discarded => "discarded",
            Self::BlockedPendingTruth => "blocked_pending_truth",
            Self::UnknownDispositionProviderOwned => "unknown_disposition_provider_owned",
        }
    }

    /// Whether this disposition needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(
            self,
            Self::HoldForReview
                | Self::Discarded
                | Self::BlockedPendingTruth
                | Self::UnknownDispositionProviderOwned
        )
    }
}

/// Why an export's publish or follow-up is blocked, if it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportBlockedClass {
    /// The export is admissible.
    NotBlocked,
    /// The underlying truth is stale and a review is required first.
    BlockedStaleTruthReviewRequired,
    /// The bundle source is untrusted and the publish is held.
    BlockedUntrustedBundle,
    /// Policy forbids the publish this export depends on.
    BlockedPolicyForbidsPublish,
    /// The export is unredacted and blocked from leaving the boundary.
    BlockedUnredactedExport,
    /// The surface is offline and no replay authority is present.
    BlockedOfflineNoReplayAuthority,
    /// Provider returned a block reason the contract does not recognise yet.
    BlockedUnknownReasonProviderOwned,
}

impl ExportBlockedClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotBlocked => "not_blocked",
            Self::BlockedStaleTruthReviewRequired => "blocked_stale_truth_review_required",
            Self::BlockedUntrustedBundle => "blocked_untrusted_bundle",
            Self::BlockedPolicyForbidsPublish => "blocked_policy_forbids_publish",
            Self::BlockedUnredactedExport => "blocked_unredacted_export",
            Self::BlockedOfflineNoReplayAuthority => "blocked_offline_no_replay_authority",
            Self::BlockedUnknownReasonProviderOwned => "blocked_unknown_reason_provider_owned",
        }
    }

    /// Whether the export is blocked.
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
pub enum ReviewExportBundleDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// A publish was surfaced without attribution.
    PublishAttributionMissing,
    /// A bundle carried a stale underlying truth.
    TruthStale,
    /// A bundle export redaction was unverified or unsafe.
    BundleRedactionUnverified,
    /// A bundle source trust class was unknown or unverified.
    BundleTrustUnknown,
    /// An offline follow-up was authorized to replay without reconnect authority.
    OfflineReplayUnauthorized,
    /// A follow-up was surfaced without attribution.
    FollowUpUnattributed,
    /// Workspace trust narrowed.
    TrustNarrowing,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl ReviewExportBundleDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::PublishAttributionMissing,
        Self::TruthStale,
        Self::BundleRedactionUnverified,
        Self::BundleTrustUnknown,
        Self::OfflineReplayUnauthorized,
        Self::FollowUpUnattributed,
        Self::TrustNarrowing,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::PublishAttributionMissing => "publish_attribution_missing",
            Self::TruthStale => "truth_stale",
            Self::BundleRedactionUnverified => "bundle_redaction_unverified",
            Self::BundleTrustUnknown => "bundle_trust_unknown",
            Self::OfflineReplayUnauthorized => "offline_replay_unauthorized",
            Self::FollowUpUnattributed => "follow_up_unattributed",
            Self::TrustNarrowing => "trust_narrowing",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project this lane's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewExportBundleConsumerSurface {
    /// Review workspace header.
    ReviewWorkspaceHeader,
    /// Merge-queue panel.
    MergeQueuePanel,
    /// Pipeline run viewer.
    PipelineRunViewer,
    /// Export bundle panel.
    ExportBundlePanel,
    /// Publish-later queue.
    PublishLaterQueue,
    /// Offline follow-up tray.
    OfflineFollowUpTray,
    /// Command palette.
    CommandPalette,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
}

impl ReviewExportBundleConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ReviewWorkspaceHeader,
        Self::MergeQueuePanel,
        Self::PipelineRunViewer,
        Self::ExportBundlePanel,
        Self::PublishLaterQueue,
        Self::OfflineFollowUpTray,
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
            Self::ExportBundlePanel => "export_bundle_panel",
            Self::PublishLaterQueue => "publish_later_queue",
            Self::OfflineFollowUpTray => "offline_follow_up_tray",
            Self::CommandPalette => "command_palette",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
        }
    }
}

/// Source provenance a bundle export carries.
///
/// The provenance must be disclosed: a non-empty source label and
/// `identity_disclosed` set true, so an export can never hide which source it
/// bundles, how fresh that source is, or who owns it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleProvenance {
    /// Typed bundle scope class.
    pub scope_class: BundleScopeClass,
    /// Typed source trust class.
    pub trust_class: BundleTrustClass,
    /// Typed truth freshness.
    pub freshness_class: BundleFreshnessClass,
    /// Redaction-aware source label (no raw bundle payload).
    pub source_label: String,
    /// Whether the source provenance is disclosed; required true.
    pub identity_disclosed: bool,
}

impl BundleProvenance {
    /// Whether the source provenance is fully disclosed.
    pub fn is_disclosed(&self) -> bool {
        self.identity_disclosed && !self.source_label.trim().is_empty()
    }

    /// Whether this provenance needs at least one explicit attention reason.
    pub const fn requires_attention_reason(&self) -> bool {
        self.scope_class.requires_attention_reason()
            || self.trust_class.requires_attention_reason()
            || self.freshness_class.requires_attention_reason()
    }
}

/// Redaction disclosure for a bundle export.
///
/// The disclosure must carry a non-empty redaction label and `redaction_disclosed`
/// set true, so the redaction class of an export's bytes is never hidden.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleRedactionDisclosure {
    /// Typed redaction class.
    pub redaction_class: BundleRedactionClass,
    /// Whether the redaction class is disclosed; required true.
    pub redaction_disclosed: bool,
    /// Redaction-aware redaction label.
    pub redaction_label: String,
}

impl BundleRedactionDisclosure {
    /// Whether the redaction class is fully disclosed.
    pub fn is_disclosed(&self) -> bool {
        self.redaction_disclosed && !self.redaction_label.trim().is_empty()
    }

    /// Whether this disclosure needs at least one explicit attention reason.
    pub const fn requires_attention_reason(&self) -> bool {
        self.redaction_class.requires_attention_reason()
    }
}

/// Publish-later disposition bound to a bundle export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishDisposition {
    /// Typed publish state.
    pub publish_state: PublishStateClass,
    /// Whether the publish disposition is disclosed; required true.
    pub publish_disclosed: bool,
    /// Whether the publish is read-only; must match the publish state.
    pub read_only: bool,
    /// Human-readable, redaction-aware publish label.
    pub publish_label: String,
    /// Publish packet ref; required when the publish state commits externally.
    pub publish_ref: Option<String>,
}

impl PublishDisposition {
    /// Whether the publish disposition is fully disclosed.
    pub fn is_disclosed(&self) -> bool {
        self.publish_disclosed && !self.publish_label.trim().is_empty()
    }

    /// Whether the read-only flag matches the publish state.
    pub fn read_only_flag_consistent(&self) -> bool {
        self.read_only == self.publish_state.is_read_only()
    }
}

/// Offline follow-up action bound to a bundle export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowUpAction {
    /// Typed connectivity class.
    pub connectivity_class: FollowUpConnectivityClass,
    /// Typed follow-up disposition.
    pub disposition_class: FollowUpDispositionClass,
    /// Whether the follow-up action is disclosed; required true.
    pub action_disclosed: bool,
    /// Whether the follow-up is ready to replay; reserved for an online surface.
    pub replay_ready: bool,
    /// Human-readable, redaction-aware action label.
    pub action_label: String,
}

impl FollowUpAction {
    /// Whether the follow-up action is fully disclosed.
    pub fn is_disclosed(&self) -> bool {
        self.action_disclosed && !self.action_label.trim().is_empty()
    }

    /// Whether the replay-ready flag is consistent with connectivity.
    ///
    /// A replay can only be ready on an online, reconnected surface; an offline,
    /// reconnecting, or unknown surface can never be pre-authorized to replay.
    pub fn replay_ready_consistent(&self) -> bool {
        !self.replay_ready || self.connectivity_class.is_online()
    }
}

/// One export bundle registry row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportBundleRow {
    /// Stable bundle id exports reference.
    pub bundle_id: String,
    /// Typed bundle scope class.
    pub scope_class: BundleScopeClass,
    /// Human-readable bundle label.
    pub bundle_label: String,
    /// Whether the bundle supports a publish-later flow.
    pub supports_publish_later: bool,
    /// Whether the bundle supports an offline replay flow.
    pub supports_offline_replay: bool,
    /// Whether the bundle supports a redacted export.
    pub supports_redacted_export: bool,
    /// Human-readable coverage label.
    pub coverage_label: String,
    /// Human-readable disclosure label.
    pub disclosure_label: String,
}

/// One bundle export row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleExportRow {
    /// Stable export id.
    pub export_id: String,
    /// Durable review anchor id bound to this export.
    pub durable_anchor_id: String,
    /// Bundle id this export is gathered from.
    pub bundle_id: String,
    /// Human-readable subject label (what is being exported).
    pub subject_label: String,
    /// Source provenance this export carries.
    pub provenance: BundleProvenance,
    /// Redaction disclosure for the export.
    pub redaction: BundleRedactionDisclosure,
    /// Publish-later disposition bound to the export.
    pub publish_disposition: PublishDisposition,
    /// Offline follow-up action bound to the export.
    pub follow_up_action: FollowUpAction,
    /// Why the publish or follow-up is blocked, if it is.
    pub blocked_class: ExportBlockedClass,
    /// Human-readable actor attribution (under whose authority the export fired).
    pub actor_attribution_label: String,
    /// Opaque ref to the audit row that lands when the export action fires.
    pub audit_row_ref: String,
    /// Attention reasons; required and non-empty when the export needs attention.
    pub attention_reasons: Vec<String>,
    /// Human-readable review summary.
    pub review_summary: String,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl BundleExportRow {
    /// Whether this export needs at least one explicit attention reason.
    pub const fn requires_attention_reason(&self) -> bool {
        self.provenance.requires_attention_reason()
            || self.redaction.requires_attention_reason()
            || self
                .publish_disposition
                .publish_state
                .requires_attention_reason()
            || self
                .follow_up_action
                .connectivity_class
                .requires_attention_reason()
            || self
                .follow_up_action
                .disposition_class
                .requires_attention_reason()
            || self.blocked_class.requires_attention_reason()
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewExportBundleTrustReview {
    /// The bundle source provenance is disclosed, never hidden.
    pub bundle_provenance_disclosed: bool,
    /// The bundle source trust class is explicit, never assumed.
    pub bundle_trust_explicit: bool,
    /// The bundle export redaction class is disclosed, never hidden.
    pub bundle_redaction_disclosed: bool,
    /// The truth freshness is disclosed.
    pub truth_freshness_disclosed: bool,
    /// The publish disposition is disclosed, never hidden.
    pub publish_disposition_disclosed: bool,
    /// The offline follow-up action is disclosed, never hidden.
    pub follow_up_disclosed: bool,
    /// A publish is a read-only draft unless an attributable publish is cited.
    pub publish_read_only_unless_attributed: bool,
    /// Every export is bound to a durable review anchor.
    pub every_export_anchored: bool,
    /// Every export action is attributable to an actor.
    pub every_action_attributable: bool,
    /// No publish or follow-up creates hidden publish scope.
    pub no_hidden_publish_scope: bool,
    /// A stale truth narrows the publish rather than shipping a possibly-wrong state.
    pub stale_truth_narrows_publish: bool,
    /// An offline replay waits for reconnect authority rather than auto-firing.
    pub offline_replay_requires_authority: bool,
    /// Downgrade narrows the claim rather than hiding the lane.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl ReviewExportBundleTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.bundle_provenance_disclosed
            && self.bundle_trust_explicit
            && self.bundle_redaction_disclosed
            && self.truth_freshness_disclosed
            && self.publish_disposition_disclosed
            && self.follow_up_disclosed
            && self.publish_read_only_unless_attributed
            && self.every_export_anchored
            && self.every_action_attributable
            && self.no_hidden_publish_scope
            && self.stale_truth_narrows_publish
            && self.offline_replay_requires_authority
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewExportBundleConsumerProjection {
    /// Review workspace header shows the durable anchor.
    pub review_workspace_header_shows_anchor: bool,
    /// Merge-queue panel shows the truth freshness.
    pub merge_queue_panel_shows_freshness: bool,
    /// Pipeline run viewer shows the bundle provenance.
    pub pipeline_run_viewer_shows_provenance: bool,
    /// Export bundle panel shows the redaction class.
    pub export_bundle_panel_shows_redaction: bool,
    /// Publish-later queue shows the publish disposition.
    pub publish_later_queue_shows_disposition: bool,
    /// Offline follow-up tray shows the connectivity class.
    pub offline_follow_up_tray_shows_connectivity: bool,
    /// Command palette shows the bundle state.
    pub command_palette_shows_bundle_state: bool,
    /// CLI / headless shows qualification truth.
    pub cli_headless_shows_truth: bool,
    /// Support export shows qualification truth.
    pub support_export_shows_truth: bool,
    /// Diagnostics shows qualification truth.
    pub diagnostics_shows_truth: bool,
    /// Review / CI / export lanes are labeled when not covered by this packet.
    pub label_for_unqualified: bool,
}

impl ReviewExportBundleConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.review_workspace_header_shows_anchor
            && self.merge_queue_panel_shows_freshness
            && self.pipeline_run_viewer_shows_provenance
            && self.export_bundle_panel_shows_redaction
            && self.publish_later_queue_shows_disposition
            && self.offline_follow_up_tray_shows_connectivity
            && self.command_palette_shows_bundle_state
            && self.cli_headless_shows_truth
            && self.support_export_shows_truth
            && self.diagnostics_shows_truth
            && self.label_for_unqualified
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewExportBundleProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`ReviewExportBundlePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewExportBundlePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Export bundle registry rows.
    pub bundle_rows: Vec<ExportBundleRow>,
    /// Bundle export rows.
    pub export_rows: Vec<BundleExportRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ReviewExportBundleDowngradeTrigger>,
    /// Consumer surfaces that must project this lane's truth.
    pub consumer_surfaces: Vec<ReviewExportBundleConsumerSurface>,
    /// Trust review block.
    pub trust_review: ReviewExportBundleTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ReviewExportBundleConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ReviewExportBundleProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe review/export bundle packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewExportBundlePacket {
    /// Record kind; must equal [`REVIEW_EXPORT_BUNDLE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`REVIEW_EXPORT_BUNDLE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Export bundle registry rows.
    pub bundle_rows: Vec<ExportBundleRow>,
    /// Bundle export rows.
    pub export_rows: Vec<BundleExportRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ReviewExportBundleDowngradeTrigger>,
    /// Consumer surfaces that must project this lane's truth.
    pub consumer_surfaces: Vec<ReviewExportBundleConsumerSurface>,
    /// Trust review block.
    pub trust_review: ReviewExportBundleTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ReviewExportBundleConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ReviewExportBundleProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ReviewExportBundlePacket {
    /// Builds a review/export bundle packet from stable-lane input.
    pub fn new(input: ReviewExportBundlePacketInput) -> Self {
        Self {
            record_kind: REVIEW_EXPORT_BUNDLE_RECORD_KIND.to_owned(),
            schema_version: REVIEW_EXPORT_BUNDLE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            bundle_rows: input.bundle_rows,
            export_rows: input.export_rows,
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

    /// Validates the review/export bundle review invariants.
    pub fn validate(&self) -> Vec<ReviewExportBundleViolation> {
        let mut violations = Vec::new();

        if self.record_kind != REVIEW_EXPORT_BUNDLE_RECORD_KIND {
            violations.push(ReviewExportBundleViolation::WrongRecordKind);
        }
        if self.schema_version != REVIEW_EXPORT_BUNDLE_SCHEMA_VERSION {
            violations.push(ReviewExportBundleViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ReviewExportBundleViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(ReviewExportBundleViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(ReviewExportBundleViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bundle_rows(self, &mut violations);
        validate_export_rows(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(ReviewExportBundleViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(ReviewExportBundleViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(ReviewExportBundleViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("review export bundle packet serializes"),
        ) {
            violations.push(ReviewExportBundleViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("review export bundle packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let publishing = self
            .export_rows
            .iter()
            .filter(|row| row.publish_disposition.publish_state.requires_publish_ref())
            .count();
        let blocked_exports = self
            .export_rows
            .iter()
            .filter(|row| row.blocked_class.is_blocked())
            .count();
        let stale_truths = self
            .export_rows
            .iter()
            .filter(|row| !row.provenance.freshness_class.is_fresh())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Review/Export Bundles, Publish-Later Packets, and Offline Follow-Up Flows\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!("- Export bundles: {}\n", self.bundle_rows.len()));
        out.push_str(&format!(
            "- Exports: {} ({} publishing, {} blocked, {} stale truths)\n",
            self.export_rows.len(),
            publishing,
            blocked_exports,
            stale_truths
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Export bundles\n\n");
        for row in &self.bundle_rows {
            out.push_str(&format!(
                "- **{}** (`{}`): publish-later {}, offline-replay {}, redacted-export {}\n",
                row.bundle_label,
                row.scope_class.as_str(),
                row.supports_publish_later,
                row.supports_offline_replay,
                row.supports_redacted_export
            ));
        }

        out.push_str("\n## Exports\n\n");
        for row in &self.export_rows {
            out.push_str(&format!(
                "- **{}** on bundle `{}` → anchor `{}`: scope `{}`, trust `{}`, freshness `{}`, redaction `{}`, publish `{}`, connectivity `{}`/`{}`, blocked `{}`, authority `{}`\n",
                row.subject_label,
                row.bundle_id,
                row.durable_anchor_id,
                row.provenance.scope_class.as_str(),
                row.provenance.trust_class.as_str(),
                row.provenance.freshness_class.as_str(),
                row.redaction.redaction_class.as_str(),
                row.publish_disposition.publish_state.as_str(),
                row.follow_up_action.connectivity_class.as_str(),
                row.follow_up_action.disposition_class.as_str(),
                row.blocked_class.as_str(),
                row.actor_attribution_label
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in review/export bundle export.
#[derive(Debug)]
pub enum ReviewExportBundleArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ReviewExportBundleViolation>),
}

impl fmt::Display for ReviewExportBundleArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "review export bundle export parse failed: {error}"
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
                    "review export bundle export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ReviewExportBundleArtifactError {}

/// Validation failures emitted by [`ReviewExportBundlePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReviewExportBundleViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No export bundle rows are present.
    BundleRowsMissing,
    /// An export bundle row is incomplete.
    BundleRowIncomplete,
    /// No bundle export rows are present.
    ExportRowsMissing,
    /// A bundle export row is incomplete.
    ExportRowIncomplete,
    /// An export references a bundle id with no bundle row.
    OrphanBundleReference,
    /// An export's source provenance is undisclosed.
    BundleProvenanceUndisclosed,
    /// An export's redaction class is undisclosed.
    BundleRedactionUndisclosed,
    /// An export's publish disposition is undisclosed.
    PublishDispositionUndisclosed,
    /// A publish disposition's read-only flag does not match its state.
    PublishReadOnlyMismatch,
    /// A publish that commits externally is missing its publish ref.
    PublishRefMissing,
    /// A blocked publish state is not marked blocked.
    PublishBlockedNotMarked,
    /// An export's follow-up action is undisclosed.
    FollowUpUndisclosed,
    /// An offline follow-up is marked replay-ready without reconnect authority.
    OfflineReplayWithoutAuthority,
    /// An export is missing its actor attribution or audit row.
    AttributionMissing,
    /// An export needing attention is missing its attention reasons.
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

impl ReviewExportBundleViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::BundleRowsMissing => "bundle_rows_missing",
            Self::BundleRowIncomplete => "bundle_row_incomplete",
            Self::ExportRowsMissing => "export_rows_missing",
            Self::ExportRowIncomplete => "export_row_incomplete",
            Self::OrphanBundleReference => "orphan_bundle_reference",
            Self::BundleProvenanceUndisclosed => "bundle_provenance_undisclosed",
            Self::BundleRedactionUndisclosed => "bundle_redaction_undisclosed",
            Self::PublishDispositionUndisclosed => "publish_disposition_undisclosed",
            Self::PublishReadOnlyMismatch => "publish_read_only_mismatch",
            Self::PublishRefMissing => "publish_ref_missing",
            Self::PublishBlockedNotMarked => "publish_blocked_not_marked",
            Self::FollowUpUndisclosed => "follow_up_undisclosed",
            Self::OfflineReplayWithoutAuthority => "offline_replay_without_authority",
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

/// Reads and validates the checked-in stable review/export bundle export.
pub fn current_review_export_bundle_export(
) -> Result<ReviewExportBundlePacket, ReviewExportBundleArtifactError> {
    let packet: ReviewExportBundlePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces/support_export.json"
    )))
    .map_err(ReviewExportBundleArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ReviewExportBundleArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &ReviewExportBundlePacket,
    violations: &mut Vec<ReviewExportBundleViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        REVIEW_EXPORT_BUNDLE_SCHEMA_REF,
        REVIEW_EXPORT_BUNDLE_DOC_REF,
        REVIEW_EXPORT_BUNDLE_HANDOFF_CONTRACT_REF,
        REVIEW_EXPORT_BUNDLE_MERGE_QUEUE_CONTRACT_REF,
        REVIEW_EXPORT_BUNDLE_PIPELINE_CONTRACT_REF,
        REVIEW_EXPORT_BUNDLE_TRUST_CLASS_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ReviewExportBundleViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bundle_rows(
    packet: &ReviewExportBundlePacket,
    violations: &mut Vec<ReviewExportBundleViolation>,
) {
    if packet.bundle_rows.is_empty() {
        violations.push(ReviewExportBundleViolation::BundleRowsMissing);
        return;
    }

    for row in &packet.bundle_rows {
        if row.bundle_id.trim().is_empty()
            || row.bundle_label.trim().is_empty()
            || row.coverage_label.trim().is_empty()
            || row.disclosure_label.trim().is_empty()
        {
            violations.push(ReviewExportBundleViolation::BundleRowIncomplete);
        }
    }
}

fn validate_export_rows(
    packet: &ReviewExportBundlePacket,
    violations: &mut Vec<ReviewExportBundleViolation>,
) {
    if packet.export_rows.is_empty() {
        violations.push(ReviewExportBundleViolation::ExportRowsMissing);
        return;
    }

    let bundle_ids: BTreeSet<&str> = packet
        .bundle_rows
        .iter()
        .map(|row| row.bundle_id.as_str())
        .collect();

    for row in &packet.export_rows {
        if row.export_id.trim().is_empty()
            || row.durable_anchor_id.trim().is_empty()
            || row.subject_label.trim().is_empty()
            || row.review_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(ReviewExportBundleViolation::ExportRowIncomplete);
        }
        if !row.bundle_id.trim().is_empty() && !bundle_ids.contains(row.bundle_id.as_str()) {
            violations.push(ReviewExportBundleViolation::OrphanBundleReference);
        }
        if !row.provenance.is_disclosed() {
            violations.push(ReviewExportBundleViolation::BundleProvenanceUndisclosed);
        }
        if !row.redaction.is_disclosed() {
            violations.push(ReviewExportBundleViolation::BundleRedactionUndisclosed);
        }
        validate_publish_disposition(row, violations);
        validate_follow_up_action(row, violations);
        if row.actor_attribution_label.trim().is_empty() || row.audit_row_ref.trim().is_empty() {
            violations.push(ReviewExportBundleViolation::AttributionMissing);
        }
        if row.requires_attention_reason() && row.attention_reasons.is_empty() {
            violations.push(ReviewExportBundleViolation::AttentionReasonMissing);
        }
    }
}

fn validate_publish_disposition(
    row: &BundleExportRow,
    violations: &mut Vec<ReviewExportBundleViolation>,
) {
    let publish = &row.publish_disposition;
    if !publish.is_disclosed() {
        violations.push(ReviewExportBundleViolation::PublishDispositionUndisclosed);
    }
    if !publish.read_only_flag_consistent() {
        violations.push(ReviewExportBundleViolation::PublishReadOnlyMismatch);
    }
    if publish.publish_state.requires_publish_ref()
        && !publish
            .publish_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty())
    {
        violations.push(ReviewExportBundleViolation::PublishRefMissing);
    }
    if publish.publish_state == PublishStateClass::PublishBlocked && !row.blocked_class.is_blocked()
    {
        violations.push(ReviewExportBundleViolation::PublishBlockedNotMarked);
    }
}

fn validate_follow_up_action(
    row: &BundleExportRow,
    violations: &mut Vec<ReviewExportBundleViolation>,
) {
    let follow_up = &row.follow_up_action;
    if !follow_up.is_disclosed() {
        violations.push(ReviewExportBundleViolation::FollowUpUndisclosed);
    }
    if !follow_up.replay_ready_consistent() {
        violations.push(ReviewExportBundleViolation::OfflineReplayWithoutAuthority);
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
