//! Typed local object models for code-host, issue, CI, and publish-later
//! continuity on provider-linked rows.
//!
//! The connected-provider registry alpha ([`crate::registry`]) named the
//! descriptors, surface claims, overlays, and run controls that surfaces may
//! claim. The publish-later queue alpha ([`crate::publish_later`]) named the
//! queue rows surfaces must consult before a deferred mutation drains. This
//! module owns the typed *local object models* that bind those two contracts
//! to one shared truth: every pull-request, branch, issue/work-item, check
//! run, pipeline run, log, artifact, or annotation Aureline references from
//! the workspace, runtime, review, or git lanes mints one
//! [`ProviderObjectRow`] that names its source, freshness, current publish
//! state, the user-facing mode (`local_draft`, `open_in_provider`,
//! `publish_later`, `inspect_only`, or `publish_now`), and the degraded
//! action the row still offers when the upstream provider is stale or
//! offline.
//!
//! [`ProviderObjectContinuityObservation`] records, per affected row, the
//! observation class (offline, stale-within-window, expired-beyond-window,
//! revoked/disconnected, disagrees-with-local, or never-observed), the
//! retained capability the local model preserves, the typed degraded action
//! the user can still take, and the rationale. This is the contract that
//! keeps offline or stale provider state from collapsing the whole workflow
//! into a generic error: the row continues to exist, names what is still
//! safe, and never silently widens mutation authority back to the upstream
//! provider.
//!
//! The cross-tool boundary lives at
//! [`/schemas/providers/provider_object.schema.json`](../../../../schemas/providers/provider_object.schema.json).
//! The reviewer-facing landing page lives at
//! [`/docs/runtime/m3/provider_object_model_alpha.md`](../../../../docs/runtime/m3/provider_object_model_alpha.md).

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::registry::{
    FreshnessLabel, FreshnessTruth, ProviderFamily, ProviderObjectKind, RedactionClass, TargetRef,
};

/// Alpha schema version exported with every provider-object record.
pub const PROVIDER_OBJECT_MODEL_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every provider-object record.
pub const PROVIDER_OBJECT_MODEL_ALPHA_SHARED_CONTRACT_REF: &str =
    "providers:provider_object_model_alpha:v1";

/// Stable record-kind tag for [`ProviderObjectModelAlphaPage`] payloads.
pub const PROVIDER_OBJECT_MODEL_ALPHA_PAGE_RECORD_KIND: &str =
    "provider_object_model_alpha_page_record";

/// Stable record-kind tag for [`ProviderObjectRow`] payloads.
pub const PROVIDER_OBJECT_MODEL_ALPHA_ROW_RECORD_KIND: &str =
    "provider_object_model_alpha_row_record";

/// Stable record-kind tag for [`ProviderObjectContinuityObservation`] payloads.
pub const PROVIDER_OBJECT_MODEL_ALPHA_CONTINUITY_OBSERVATION_RECORD_KIND: &str =
    "provider_object_model_alpha_continuity_observation_record";

/// Stable record-kind tag for [`ProviderObjectModelAlphaValidationReport`].
pub const PROVIDER_OBJECT_MODEL_ALPHA_VALIDATION_REPORT_RECORD_KIND: &str =
    "provider_object_model_alpha_validation_report";

/// Stable record-kind tag for [`ProviderObjectModelAlphaSupportExport`].
pub const PROVIDER_OBJECT_MODEL_ALPHA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "provider_object_model_alpha_support_export";

/// Source class observed for one provider-linked local object row.
///
/// `local_draft_only` and `offline_unverified_capture` are local-authority
/// sources: the row exists without observed live provider truth and must
/// continue to support local authoring while the upstream provider is
/// stale, offline, or revoked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectSourceClass {
    /// Live provider connection that observed the row.
    LiveProvider,
    /// Cached provider overlay that observed the row inside a freshness window.
    CachedProviderOverlay,
    /// Imported provider snapshot (inspect-only or detached from live).
    ImportedSnapshot,
    /// Mirrored or self-hosted provider route.
    MirroredOrSelfHosted,
    /// Local-only draft authored before any provider observation.
    LocalDraftOnly,
    /// Offline capture: row was minted offline and has never been verified
    /// against live provider truth.
    OfflineUnverifiedCapture,
}

impl ObjectSourceClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveProvider => "live_provider",
            Self::CachedProviderOverlay => "cached_provider_overlay",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::MirroredOrSelfHosted => "mirrored_or_self_hosted",
            Self::LocalDraftOnly => "local_draft_only",
            Self::OfflineUnverifiedCapture => "offline_unverified_capture",
        }
    }

    /// True when the source is rooted in local authority (no live provider
    /// truth was observed).
    pub const fn is_local_only(self) -> bool {
        matches!(self, Self::LocalDraftOnly | Self::OfflineUnverifiedCapture)
    }
}

/// Lifecycle posture observed for one provider-linked local object row.
///
/// `publish_state` is the *current state* the row sits in. It is distinct
/// from the user-facing [`ObjectModeClass`], which is the *intent* the user
/// chose when authoring the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectPublishStateClass {
    /// Local draft only. No provider mutation has been requested.
    LocalDraftOnly,
    /// User has requested publish-now and the action is pending review.
    PublishNowPendingReview,
    /// Publish-now drained and the provider observed the new state.
    PublishNowPublishedObserved,
    /// User selected open-in-provider and the typed handoff packet is pending.
    OpenInProviderPending,
    /// User selected publish-later and the row is queued for drain.
    PublishLaterQueued,
    /// Publish-later queue item drained and the provider observed the new state.
    PublishLaterDrained,
    /// Provider truth is authoritative and matches the local model.
    PublishedObservedAuthoritative,
    /// Row is inspect-only, projected from an imported snapshot.
    InspectOnlyImported,
    /// Row was minted offline and has not been verified against live provider truth.
    OfflineUnverified,
    /// Provider revoked the grant or disconnected; mutation authority is closed.
    RevokedAtProvider,
    /// Local model disagrees with the most recent provider observation;
    /// reviewer must resolve before mutation may proceed.
    DisagreesWithLocal,
}

impl ObjectPublishStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDraftOnly => "local_draft_only",
            Self::PublishNowPendingReview => "publish_now_pending_review",
            Self::PublishNowPublishedObserved => "publish_now_published_observed",
            Self::OpenInProviderPending => "open_in_provider_pending",
            Self::PublishLaterQueued => "publish_later_queued",
            Self::PublishLaterDrained => "publish_later_drained",
            Self::PublishedObservedAuthoritative => "published_observed_authoritative",
            Self::InspectOnlyImported => "inspect_only_imported",
            Self::OfflineUnverified => "offline_unverified",
            Self::RevokedAtProvider => "revoked_at_provider",
            Self::DisagreesWithLocal => "disagrees_with_local",
        }
    }

    /// True when the state admits no further upstream mutation without
    /// repair (offline, revoked, disagreement).
    pub const fn holds_mutation_closed(self) -> bool {
        matches!(
            self,
            Self::OfflineUnverified | Self::RevokedAtProvider | Self::DisagreesWithLocal
        )
    }
}

/// User-facing mode the local object row exposes. Modes are explicit and
/// separate; the acceptance contract requires `local_draft`,
/// `publish_later`, `open_in_provider`, and `inspect_only` to never collapse
/// into one another. `publish_now` is the in-product reviewed-authority mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectModeClass {
    /// Local-draft mode. The row never leaves the workspace without an
    /// explicit later choice.
    LocalDraftMode,
    /// Publish-now mode. The row mutates the provider through reviewed
    /// in-product authority bound to an approval ticket.
    PublishNowMode,
    /// Open-in-provider mode. The row mutates the provider through a typed
    /// browser handoff packet.
    OpenInProviderMode,
    /// Publish-later mode. The row enters the publish-later queue and only
    /// drains when prerequisites align.
    PublishLaterMode,
    /// Inspect-only mode. The row reads provider truth without mutation.
    InspectOnlyMode,
}

impl ObjectModeClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDraftMode => "local_draft_mode",
            Self::PublishNowMode => "publish_now_mode",
            Self::OpenInProviderMode => "open_in_provider_mode",
            Self::PublishLaterMode => "publish_later_mode",
            Self::InspectOnlyMode => "inspect_only_mode",
        }
    }
}

/// Typed degraded action a reviewer can take when provider state is stale,
/// offline, revoked, or disagrees with the local model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedActionClass {
    /// No degraded action is required.
    NoneRequired,
    /// Continue authoring locally; the row stays a local draft.
    ContinueLocalAuthoring,
    /// Queue the row into publish-later until prerequisites align.
    QueuePublishLater,
    /// Open the row through the typed browser-handoff packet.
    OpenInProviderBrowserHandoff,
    /// Export an evidence-safe packet for support or audit.
    ExportEvidencePacket,
    /// Hold the row until the freshness floor is repaired.
    HoldForFreshnessRepair,
    /// Hold the row until reauth completes.
    HoldForReauth,
    /// Hold the row until rescope completes.
    HoldForRescope,
}

impl DegradedActionClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneRequired => "none_required",
            Self::ContinueLocalAuthoring => "continue_local_authoring",
            Self::QueuePublishLater => "queue_publish_later",
            Self::OpenInProviderBrowserHandoff => "open_in_provider_browser_handoff",
            Self::ExportEvidencePacket => "export_evidence_packet",
            Self::HoldForFreshnessRepair => "hold_for_freshness_repair",
            Self::HoldForReauth => "hold_for_reauth",
            Self::HoldForRescope => "hold_for_rescope",
        }
    }
}

/// Continuity-observation class. One per provider-linked row whose upstream
/// truth is stale, offline, revoked, or disagrees with the local model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityObservationClass {
    /// Provider is unreachable (offline).
    ProviderOffline,
    /// Provider truth is stale but still inside a bounded review window.
    ProviderStaleWithinWindow,
    /// Provider truth is past the freshness window and requires re-observe.
    ProviderExpiredBeyondWindow,
    /// Provider revoked the grant or disconnected the row.
    ProviderRevokedOrDisconnected,
    /// Provider truth disagrees with the local model and requires review.
    ProviderDisagreesWithLocal,
    /// Provider has never been observed for this row.
    ProviderNeverObserved,
}

impl ContinuityObservationClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderOffline => "provider_offline",
            Self::ProviderStaleWithinWindow => "provider_stale_within_window",
            Self::ProviderExpiredBeyondWindow => "provider_expired_beyond_window",
            Self::ProviderRevokedOrDisconnected => "provider_revoked_or_disconnected",
            Self::ProviderDisagreesWithLocal => "provider_disagrees_with_local",
            Self::ProviderNeverObserved => "provider_never_observed",
        }
    }
}

/// Capability the local object row retains after a continuity observation.
/// `no_capability_retained` is admitted only when the workflow truly cannot
/// proceed (e.g. credential evicted with no local body); every other class
/// must keep local authoring or a typed degraded action alive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetainedCapabilityClass {
    /// Local authoring continues; the row stays editable.
    LocalDraftAuthoringRetained,
    /// Publish-later queuing remains available behind a typed dependency.
    PublishLaterQueuingRetained,
    /// Inspect-only continues against the imported snapshot.
    InspectOnlyRetained,
    /// A typed browser-handoff is offered as the only remaining capability.
    BrowserHandoffOffered,
    /// No capability could be retained.
    NoCapabilityRetained,
}

impl RetainedCapabilityClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDraftAuthoringRetained => "local_draft_authoring_retained",
            Self::PublishLaterQueuingRetained => "publish_later_queuing_retained",
            Self::InspectOnlyRetained => "inspect_only_retained",
            Self::BrowserHandoffOffered => "browser_handoff_offered",
            Self::NoCapabilityRetained => "no_capability_retained",
        }
    }

    /// True when this capability keeps local authoring or a typed degraded
    /// path alive (i.e. workflow does not collapse into a generic error).
    pub const fn keeps_workflow_alive(self) -> bool {
        !matches!(self, Self::NoCapabilityRetained)
    }
}

/// References to upstream schema and contract files consumed by this page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderObjectModelContractRefs {
    /// Existing connected-provider registry schema reference.
    pub connected_provider_registry_schema_ref: String,
    /// Existing publish-later queue alpha schema reference.
    pub publish_later_queue_alpha_schema_ref: String,
    /// Existing publish-later record schema reference.
    pub publish_later_record_schema_ref: String,
    /// Existing browser-handoff packet schema reference.
    pub browser_handoff_packet_schema_ref: String,
    /// Existing approval-ticket alpha schema reference.
    pub approval_ticket_alpha_schema_ref: String,
    /// Existing change-object alpha schema reference.
    pub change_object_alpha_schema_ref: String,
}

impl ProviderObjectModelContractRefs {
    fn all_refs(&self) -> [&str; 6] {
        [
            &self.connected_provider_registry_schema_ref,
            &self.publish_later_queue_alpha_schema_ref,
            &self.publish_later_record_schema_ref,
            &self.browser_handoff_packet_schema_ref,
            &self.approval_ticket_alpha_schema_ref,
            &self.change_object_alpha_schema_ref,
        ]
    }
}

/// Redaction-safe object source metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectSource {
    /// Source class for this row.
    pub source_class: ObjectSourceClass,
    /// Opaque canonical-host ref (e.g. `provider.host.code_host.primary`).
    pub canonical_host_ref: String,
    /// Opaque tenant, org, or project scope reference.
    pub tenant_or_org_scope_ref: String,
    /// Opaque environment ref when relevant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_ref: Option<String>,
    /// Opaque import-session ref for imported snapshots.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_session_ref: Option<String>,
}

/// One typed local object model for a provider-linked row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderObjectRow {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque object row id.
    pub object_row_id: String,
    /// Reviewable row label safe for support export.
    pub display_label: String,
    /// Connected-provider descriptor this row reads from.
    pub provider_descriptor_ref: String,
    /// Provider family for the row.
    pub provider_family: ProviderFamily,
    /// Provider-side object kind named by this row.
    pub object_kind: ProviderObjectKind,
    /// Target ref this row binds to.
    pub target_ref: TargetRef,
    /// Source metadata for this row.
    pub source: ObjectSource,
    /// Freshness truth for the local object model.
    pub freshness: FreshnessTruth,
    /// Current publish state.
    pub publish_state: ObjectPublishStateClass,
    /// User-facing mode.
    pub mode: ObjectModeClass,
    /// Typed degraded action the row still offers.
    pub degraded_action: DegradedActionClass,
    /// Optional opaque local-draft ref for local-draft and publish-later rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_draft_ref: Option<String>,
    /// Optional opaque publish-later queue item ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_later_queue_item_ref: Option<String>,
    /// Optional opaque approval-ticket ref required for publish-now rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ticket_ref: Option<String>,
    /// Optional opaque browser-handoff packet ref required for
    /// open-in-provider rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Optional opaque imported-snapshot ref for inspect-only rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_snapshot_ref: Option<String>,
    /// Optional opaque parent row ref (e.g. a check run that belongs to a
    /// pipeline run, an annotation that belongs to a log).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_object_row_ref: Option<String>,
    /// Audit event refs minted or expected by this row.
    #[serde(default)]
    pub audit_event_refs: Vec<String>,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
    /// Guardrail: raw provider-payload refs are not present on the row.
    pub raw_payload_refs_present: bool,
    /// Guardrail: local editing is preserved through this row.
    pub local_editing_preserved: bool,
    /// Export-safe summary of the row.
    pub support_export_summary: String,
    /// Timestamp at which the local model was last observed.
    pub observed_at: String,
}

/// One continuity observation on a provider-linked local object row.
///
/// Continuity observations exist precisely so offline or stale provider
/// state does not collapse the workflow into a generic error. Every
/// observation names the typed observation class, the retained capability,
/// the typed degraded action the user can still take, and the rationale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderObjectContinuityObservation {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque observation id.
    pub observation_id: String,
    /// Object row id the observation binds to.
    pub object_row_ref: String,
    /// Typed observation class.
    pub observation_class: ContinuityObservationClass,
    /// Capability the local object row retains under this observation.
    pub retained_capability_class: RetainedCapabilityClass,
    /// Typed degraded action the row still offers.
    pub degraded_action: DegradedActionClass,
    /// Export-safe rationale for the observation.
    pub rationale_summary: String,
    /// Timestamp at which the observation was made.
    pub observed_at: String,
    /// Guardrail: observation did not silently widen mutation authority.
    pub silent_mutation_authority_widened: bool,
    /// Guardrail: local editing remains preserved when the row admits it.
    pub local_editing_preserved: bool,
}

/// One alpha page: rows plus continuity observations under one packet id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderObjectModelAlphaPage {
    /// Optional fixture metadata for validation lanes.
    #[serde(default, rename = "__fixture__", skip_serializing_if = "Option::is_none")]
    pub fixture_metadata: Option<ProviderObjectFixtureMetadata>,
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for the page.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque page id.
    pub page_id: String,
    /// Upstream contracts this page consumes by reference.
    pub contract_refs: ProviderObjectModelContractRefs,
    /// Local object rows in this page.
    pub rows: Vec<ProviderObjectRow>,
    /// Continuity observations attached to rows in this page.
    #[serde(default)]
    pub continuity_observations: Vec<ProviderObjectContinuityObservation>,
    /// Export-safe page summary.
    pub support_export_summary: String,
}

/// Fixture metadata used by protected cases.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderObjectFixtureMetadata {
    /// Short fixture name.
    pub name: String,
    /// Reviewer-safe scenario summary.
    pub scenario: String,
}

impl ProviderObjectModelAlphaPage {
    /// Validate the page against alpha invariants.
    pub fn validate(&self) -> ProviderObjectModelAlphaValidationReport {
        let mut validator = Validator::new(self);
        validator.run();
        validator.finish()
    }

    /// Build a redaction-safe support export projection.
    pub fn support_export_projection(&self) -> ProviderObjectModelAlphaSupportExport {
        let row_summaries = self
            .rows
            .iter()
            .map(|row| ProviderObjectRowSummary {
                object_row_id: row.object_row_id.clone(),
                display_label: row.display_label.clone(),
                provider_descriptor_ref: row.provider_descriptor_ref.clone(),
                provider_family: row.provider_family,
                object_kind: row.object_kind,
                target_ref: row.target_ref.clone(),
                source_class: row.source.source_class,
                freshness_class: row.freshness.freshness_class,
                publish_state: row.publish_state,
                mode: row.mode,
                degraded_action: row.degraded_action,
                support_export_summary: row.support_export_summary.clone(),
            })
            .collect();
        let continuity_summaries = self
            .continuity_observations
            .iter()
            .map(|observation| ProviderObjectContinuityObservationSummary {
                observation_id: observation.observation_id.clone(),
                object_row_ref: observation.object_row_ref.clone(),
                observation_class: observation.observation_class,
                retained_capability_class: observation.retained_capability_class,
                degraded_action: observation.degraded_action,
                rationale_summary: observation.rationale_summary.clone(),
            })
            .collect();
        ProviderObjectModelAlphaSupportExport {
            record_kind: PROVIDER_OBJECT_MODEL_ALPHA_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: PROVIDER_OBJECT_MODEL_ALPHA_SCHEMA_VERSION,
            page_id: self.page_id.clone(),
            row_summaries,
            continuity_summaries,
            redaction_class: RedactionClass::MetadataSafeDefault,
        }
    }
}

/// Validation report emitted by the alpha validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderObjectModelAlphaValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version validated.
    pub schema_version: u32,
    /// Page id under validation.
    pub page_id: String,
    /// Whether no error-severity checks failed.
    pub passed: bool,
    /// Coverage observed while validating the page.
    pub coverage: ProviderObjectModelAlphaCoverage,
    /// Findings emitted by failed checks.
    pub findings: Vec<ProviderObjectModelAlphaFinding>,
}

/// Coverage observed during alpha validation.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ProviderObjectModelAlphaCoverage {
    /// Provider families covered by rows.
    pub provider_families: BTreeSet<ProviderFamily>,
    /// Object kinds covered by rows.
    pub object_kinds: BTreeSet<ProviderObjectKind>,
    /// User-facing modes covered by rows.
    pub modes: BTreeSet<ObjectModeClass>,
    /// Source classes covered by rows.
    pub source_classes: BTreeSet<ObjectSourceClass>,
    /// Publish states covered by rows.
    pub publish_states: BTreeSet<ObjectPublishStateClass>,
    /// Continuity observation classes seen in the page.
    pub continuity_observation_classes: BTreeSet<ContinuityObservationClass>,
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderObjectModelAlphaFinding {
    /// Severity.
    pub severity: ProviderObjectModelAlphaFindingSeverity,
    /// Stable check id.
    pub check_id: String,
    /// Redaction-safe message.
    pub message: String,
}

/// Validation finding severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderObjectModelAlphaFindingSeverity {
    /// Error that blocks the page.
    Error,
    /// Warning that keeps the page reviewable but visibly degraded.
    Warning,
}

/// Redaction-safe support export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderObjectModelAlphaSupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Page id.
    pub page_id: String,
    /// Row summaries safe for support bundles.
    pub row_summaries: Vec<ProviderObjectRowSummary>,
    /// Continuity-observation summaries safe for support bundles.
    pub continuity_summaries: Vec<ProviderObjectContinuityObservationSummary>,
    /// Redaction posture for the projection.
    pub redaction_class: RedactionClass,
}

/// Redaction-safe summary of one local object row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderObjectRowSummary {
    /// Object row id.
    pub object_row_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Provider descriptor ref.
    pub provider_descriptor_ref: String,
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Object kind.
    pub object_kind: ProviderObjectKind,
    /// Target ref.
    pub target_ref: TargetRef,
    /// Source class.
    pub source_class: ObjectSourceClass,
    /// Freshness class.
    pub freshness_class: FreshnessLabel,
    /// Publish state.
    pub publish_state: ObjectPublishStateClass,
    /// Mode.
    pub mode: ObjectModeClass,
    /// Degraded action.
    pub degraded_action: DegradedActionClass,
    /// Export-safe summary.
    pub support_export_summary: String,
}

/// Redaction-safe summary of one continuity observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderObjectContinuityObservationSummary {
    /// Observation id.
    pub observation_id: String,
    /// Bound row ref.
    pub object_row_ref: String,
    /// Observation class.
    pub observation_class: ContinuityObservationClass,
    /// Retained capability class.
    pub retained_capability_class: RetainedCapabilityClass,
    /// Degraded action.
    pub degraded_action: DegradedActionClass,
    /// Rationale summary.
    pub rationale_summary: String,
}

struct Validator<'a> {
    page: &'a ProviderObjectModelAlphaPage,
    row_ids: BTreeSet<&'a str>,
    observation_ids: BTreeSet<&'a str>,
    coverage: ProviderObjectModelAlphaCoverage,
    findings: Vec<ProviderObjectModelAlphaFinding>,
}

impl<'a> Validator<'a> {
    fn new(page: &'a ProviderObjectModelAlphaPage) -> Self {
        Self {
            page,
            row_ids: BTreeSet::new(),
            observation_ids: BTreeSet::new(),
            coverage: ProviderObjectModelAlphaCoverage::default(),
            findings: Vec::new(),
        }
    }

    fn run(&mut self) {
        self.validate_page_header();
        self.validate_rows();
        self.validate_continuity_observations();
        self.validate_required_coverage();
    }

    fn finish(self) -> ProviderObjectModelAlphaValidationReport {
        let passed = self
            .findings
            .iter()
            .all(|finding| finding.severity != ProviderObjectModelAlphaFindingSeverity::Error);
        ProviderObjectModelAlphaValidationReport {
            record_kind: PROVIDER_OBJECT_MODEL_ALPHA_VALIDATION_REPORT_RECORD_KIND.to_string(),
            schema_version: PROVIDER_OBJECT_MODEL_ALPHA_SCHEMA_VERSION,
            page_id: self.page.page_id.clone(),
            passed,
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn validate_page_header(&mut self) {
        let page = self.page;
        self.expect(
            page.record_kind == PROVIDER_OBJECT_MODEL_ALPHA_PAGE_RECORD_KIND,
            "provider_object_alpha.page_record_kind",
            "page.record_kind must be provider_object_model_alpha_page_record",
        );
        self.expect(
            page.schema_version == PROVIDER_OBJECT_MODEL_ALPHA_SCHEMA_VERSION,
            "provider_object_alpha.page_schema_version",
            "page.schema_version must match the crate constant",
        );
        self.expect(
            page.shared_contract_ref == PROVIDER_OBJECT_MODEL_ALPHA_SHARED_CONTRACT_REF,
            "provider_object_alpha.page_shared_contract_ref",
            "page.shared_contract_ref must match the shared contract id",
        );
        self.expect(
            !page.page_id.trim().is_empty(),
            "provider_object_alpha.page_id_missing",
            "page.page_id must be non-empty",
        );
        self.expect(
            !page.support_export_summary.trim().is_empty(),
            "provider_object_alpha.page_support_summary_missing",
            "page.support_export_summary must be non-empty",
        );
        for contract_ref in page.contract_refs.all_refs() {
            self.expect(
                !contract_ref.trim().is_empty(),
                "provider_object_alpha.contract_ref_missing",
                "every consumed upstream contract ref must be non-empty",
            );
        }
        self.expect(
            !page.rows.is_empty(),
            "provider_object_alpha.rows_missing",
            "page must contain at least one provider-object row",
        );
    }

    fn validate_rows(&mut self) {
        for row in &self.page.rows {
            self.expect(
                row.record_kind == PROVIDER_OBJECT_MODEL_ALPHA_ROW_RECORD_KIND,
                "provider_object_alpha.row_record_kind",
                "row.record_kind is wrong",
            );
            self.expect(
                row.schema_version == PROVIDER_OBJECT_MODEL_ALPHA_SCHEMA_VERSION,
                "provider_object_alpha.row_schema_version",
                "row.schema_version is wrong",
            );
            self.expect(
                row.shared_contract_ref == PROVIDER_OBJECT_MODEL_ALPHA_SHARED_CONTRACT_REF,
                "provider_object_alpha.row_shared_contract_ref",
                "row.shared_contract_ref must match the shared contract id",
            );
            let id_is_unique = self.row_ids.insert(&row.object_row_id);
            self.expect(
                id_is_unique,
                "provider_object_alpha.row_duplicate",
                "object_row_id values must be unique within a page",
            );
            self.expect(
                !row.display_label.trim().is_empty(),
                "provider_object_alpha.row_display_label_missing",
                "row.display_label must be non-empty",
            );
            self.expect(
                !row.provider_descriptor_ref.trim().is_empty(),
                "provider_object_alpha.row_descriptor_ref_missing",
                "row.provider_descriptor_ref must be non-empty",
            );
            self.expect(
                !row.target_ref.target_ref_class.trim().is_empty()
                    && !row.target_ref.target_ref.trim().is_empty()
                    && !row.target_ref.target_label.trim().is_empty(),
                "provider_object_alpha.row_target_ref_invalid",
                "row.target_ref must carry class, id, and label",
            );
            self.expect(
                !row.source.canonical_host_ref.trim().is_empty()
                    && !row.source.tenant_or_org_scope_ref.trim().is_empty(),
                "provider_object_alpha.row_source_missing",
                "row.source must cite canonical host and tenant/org scope refs",
            );
            self.expect(
                !row.support_export_summary.trim().is_empty(),
                "provider_object_alpha.row_support_summary_missing",
                "row.support_export_summary must be non-empty",
            );
            self.expect(
                !row.raw_payload_refs_present,
                "provider_object_alpha.row_raw_payload_present",
                "row.raw_payload_refs_present must be false",
            );
            self.expect(
                row.local_editing_preserved,
                "provider_object_alpha.row_local_editing_not_preserved",
                "row.local_editing_preserved must be true; local authoring cannot silently collapse",
            );

            self.validate_freshness(&row.freshness, &row.object_row_id);
            self.validate_family_kind(row);
            self.validate_mode_state_refs(row);
            self.validate_source_state_coherence(row);
            self.validate_degraded_action(row);

            self.coverage.provider_families.insert(row.provider_family);
            self.coverage.object_kinds.insert(row.object_kind);
            self.coverage.modes.insert(row.mode);
            self.coverage.source_classes.insert(row.source.source_class);
            self.coverage.publish_states.insert(row.publish_state);
        }
    }

    fn validate_family_kind(&mut self, row: &ProviderObjectRow) {
        let ok = match row.provider_family {
            ProviderFamily::CodeHost => matches!(
                row.object_kind,
                ProviderObjectKind::PullRequest | ProviderObjectKind::Branch
            ),
            ProviderFamily::IssueTracker => {
                matches!(row.object_kind, ProviderObjectKind::IssueOrWorkItem)
            }
            ProviderFamily::CiChecks => matches!(
                row.object_kind,
                ProviderObjectKind::CheckRun
                    | ProviderObjectKind::PipelineRun
                    | ProviderObjectKind::PipelineLog
                    | ProviderObjectKind::PipelineArtifact
                    | ProviderObjectKind::PipelineAnnotation
            ),
        };
        self.expect(
            ok,
            "provider_object_alpha.row_family_kind_mismatch",
            "row.object_kind is not admissible for row.provider_family",
        );
    }

    fn validate_mode_state_refs(&mut self, row: &ProviderObjectRow) {
        let publish_state_ok = match row.mode {
            ObjectModeClass::LocalDraftMode => matches!(
                row.publish_state,
                ObjectPublishStateClass::LocalDraftOnly | ObjectPublishStateClass::OfflineUnverified
            ),
            ObjectModeClass::PublishNowMode => matches!(
                row.publish_state,
                ObjectPublishStateClass::PublishNowPendingReview
                    | ObjectPublishStateClass::PublishNowPublishedObserved
                    | ObjectPublishStateClass::PublishedObservedAuthoritative
                    | ObjectPublishStateClass::DisagreesWithLocal
            ),
            ObjectModeClass::OpenInProviderMode => matches!(
                row.publish_state,
                ObjectPublishStateClass::OpenInProviderPending
                    | ObjectPublishStateClass::PublishedObservedAuthoritative
            ),
            ObjectModeClass::PublishLaterMode => matches!(
                row.publish_state,
                ObjectPublishStateClass::PublishLaterQueued
                    | ObjectPublishStateClass::PublishLaterDrained
                    | ObjectPublishStateClass::PublishedObservedAuthoritative
            ),
            ObjectModeClass::InspectOnlyMode => matches!(
                row.publish_state,
                ObjectPublishStateClass::InspectOnlyImported
                    | ObjectPublishStateClass::OfflineUnverified
                    | ObjectPublishStateClass::RevokedAtProvider
                    | ObjectPublishStateClass::DisagreesWithLocal
                    | ObjectPublishStateClass::PublishedObservedAuthoritative
            ),
        };
        self.expect(
            publish_state_ok,
            "provider_object_alpha.row_mode_state_incompatible",
            "row.publish_state is not admissible for row.mode",
        );

        let non_empty = |opt: &Option<String>| opt.as_deref().is_some_and(|v| !v.trim().is_empty());

        match row.mode {
            ObjectModeClass::LocalDraftMode => self.expect(
                non_empty(&row.local_draft_ref),
                "provider_object_alpha.row_local_draft_ref_missing",
                "local_draft_mode rows must cite a local_draft_ref",
            ),
            ObjectModeClass::PublishNowMode => {
                if !matches!(
                    row.publish_state,
                    ObjectPublishStateClass::PublishedObservedAuthoritative
                ) {
                    self.expect(
                        non_empty(&row.approval_ticket_ref),
                        "provider_object_alpha.row_approval_ticket_ref_missing",
                        "publish_now_mode rows must cite an approval_ticket_ref before publish",
                    );
                }
            }
            ObjectModeClass::OpenInProviderMode => self.expect(
                non_empty(&row.browser_handoff_packet_ref),
                "provider_object_alpha.row_browser_handoff_ref_missing",
                "open_in_provider_mode rows must cite a browser_handoff_packet_ref",
            ),
            ObjectModeClass::PublishLaterMode => self.expect(
                non_empty(&row.publish_later_queue_item_ref),
                "provider_object_alpha.row_publish_later_ref_missing",
                "publish_later_mode rows must cite a publish_later_queue_item_ref",
            ),
            ObjectModeClass::InspectOnlyMode => {
                let snapshot_or_local =
                    non_empty(&row.imported_snapshot_ref) || non_empty(&row.local_draft_ref);
                self.expect(
                    snapshot_or_local,
                    "provider_object_alpha.row_inspect_ref_missing",
                    "inspect_only_mode rows must cite an imported_snapshot_ref or local_draft_ref",
                );
            }
        }
    }

    fn validate_source_state_coherence(&mut self, row: &ProviderObjectRow) {
        match row.source.source_class {
            ObjectSourceClass::LocalDraftOnly => {
                self.expect(
                    row.mode == ObjectModeClass::LocalDraftMode,
                    "provider_object_alpha.row_local_only_mode",
                    "local_draft_only source rows must be in local_draft_mode",
                );
                self.expect(
                    matches!(
                        row.publish_state,
                        ObjectPublishStateClass::LocalDraftOnly
                            | ObjectPublishStateClass::OfflineUnverified
                    ),
                    "provider_object_alpha.row_local_only_state",
                    "local_draft_only source rows must hold a local-only publish state",
                );
            }
            ObjectSourceClass::OfflineUnverifiedCapture => {
                self.expect(
                    matches!(
                        row.publish_state,
                        ObjectPublishStateClass::OfflineUnverified
                            | ObjectPublishStateClass::LocalDraftOnly
                    ),
                    "provider_object_alpha.row_offline_state",
                    "offline_unverified_capture rows must hold offline_unverified or local_draft_only publish state",
                );
                self.expect(
                    !matches!(row.freshness.freshness_class, FreshnessLabel::Fresh),
                    "provider_object_alpha.row_offline_freshness",
                    "offline_unverified_capture rows cannot claim fresh freshness",
                );
            }
            ObjectSourceClass::LiveProvider
            | ObjectSourceClass::CachedProviderOverlay
            | ObjectSourceClass::MirroredOrSelfHosted => {
                if matches!(
                    row.freshness.freshness_class,
                    FreshnessLabel::RevokedOrDisconnected
                ) {
                    self.expect(
                        matches!(
                            row.publish_state,
                            ObjectPublishStateClass::RevokedAtProvider
                                | ObjectPublishStateClass::DisagreesWithLocal
                                | ObjectPublishStateClass::OfflineUnverified
                        ),
                        "provider_object_alpha.row_revoked_state_missing",
                        "rows with revoked_or_disconnected freshness must hold a degraded publish state",
                    );
                }
            }
            ObjectSourceClass::ImportedSnapshot => {
                self.expect(
                    row.source.import_session_ref.is_some(),
                    "provider_object_alpha.row_import_session_missing",
                    "imported_snapshot rows must cite an import_session_ref",
                );
            }
        }
    }

    fn validate_degraded_action(&mut self, row: &ProviderObjectRow) {
        if row.publish_state.holds_mutation_closed() {
            self.expect(
                row.degraded_action != DegradedActionClass::NoneRequired,
                "provider_object_alpha.row_degraded_action_required",
                "rows whose publish state holds mutation closed must name a degraded_action",
            );
        }
        if matches!(
            row.freshness.freshness_class,
            FreshnessLabel::StaleWithinWindow
                | FreshnessLabel::ExpiredBeyondWindow
                | FreshnessLabel::NeverObserved
                | FreshnessLabel::RevokedOrDisconnected
        ) {
            self.expect(
                row.degraded_action != DegradedActionClass::NoneRequired,
                "provider_object_alpha.row_degraded_action_missing_for_freshness",
                "rows with degraded freshness must name a degraded_action",
            );
        }
    }

    fn validate_continuity_observations(&mut self) {
        for observation in &self.page.continuity_observations {
            self.expect(
                observation.record_kind
                    == PROVIDER_OBJECT_MODEL_ALPHA_CONTINUITY_OBSERVATION_RECORD_KIND,
                "provider_object_alpha.observation_record_kind",
                "continuity_observation.record_kind is wrong",
            );
            self.expect(
                observation.schema_version == PROVIDER_OBJECT_MODEL_ALPHA_SCHEMA_VERSION,
                "provider_object_alpha.observation_schema_version",
                "continuity_observation.schema_version is wrong",
            );
            self.expect(
                observation.shared_contract_ref == PROVIDER_OBJECT_MODEL_ALPHA_SHARED_CONTRACT_REF,
                "provider_object_alpha.observation_shared_contract_ref",
                "continuity_observation.shared_contract_ref must match the shared contract id",
            );
            let id_is_unique = self.observation_ids.insert(&observation.observation_id);
            self.expect(
                id_is_unique,
                "provider_object_alpha.observation_duplicate",
                "observation_id values must be unique within a page",
            );
            self.expect(
                self.row_ids.contains(observation.object_row_ref.as_str()),
                "provider_object_alpha.observation_row_ref_unknown",
                "continuity_observation.object_row_ref must reference a row in the page",
            );
            self.expect(
                !observation.rationale_summary.trim().is_empty(),
                "provider_object_alpha.observation_rationale_missing",
                "continuity_observation.rationale_summary must be non-empty",
            );
            self.expect(
                !observation.silent_mutation_authority_widened,
                "provider_object_alpha.observation_silent_widen",
                "continuity_observation.silent_mutation_authority_widened must be false",
            );
            if observation.retained_capability_class
                == RetainedCapabilityClass::NoCapabilityRetained
            {
                self.expect(
                    !observation.local_editing_preserved,
                    "provider_object_alpha.observation_no_capability_with_local_editing",
                    "no_capability_retained cannot coexist with local_editing_preserved=true",
                );
                self.expect(
                    observation.degraded_action != DegradedActionClass::ContinueLocalAuthoring,
                    "provider_object_alpha.observation_no_capability_continue",
                    "no_capability_retained cannot pair with continue_local_authoring",
                );
            }
            self.coverage
                .continuity_observation_classes
                .insert(observation.observation_class);
        }
    }

    fn validate_required_coverage(&mut self) {
        for family in [
            ProviderFamily::CodeHost,
            ProviderFamily::IssueTracker,
            ProviderFamily::CiChecks,
        ] {
            self.expect(
                self.coverage.provider_families.contains(&family),
                "provider_object_alpha.coverage_provider_family_missing",
                "page must cover code-host, issue, and CI provider families",
            );
        }
        for mode in [
            ObjectModeClass::LocalDraftMode,
            ObjectModeClass::PublishLaterMode,
            ObjectModeClass::OpenInProviderMode,
            ObjectModeClass::InspectOnlyMode,
        ] {
            self.expect(
                self.coverage.modes.contains(&mode),
                "provider_object_alpha.coverage_mode_missing",
                "page must cover the local_draft, publish_later, open_in_provider, and inspect_only modes",
            );
        }
    }

    fn validate_freshness(&mut self, freshness: &FreshnessTruth, owner: &str) {
        self.expect(
            !freshness.freshness_floor_ref.trim().is_empty(),
            "provider_object_alpha.row_freshness_floor_missing",
            &format!("row {owner} freshness must cite a freshness_floor_ref"),
        );
        if matches!(
            freshness.freshness_class,
            FreshnessLabel::StaleWithinWindow
                | FreshnessLabel::ExpiredBeyondWindow
                | FreshnessLabel::NeverObserved
                | FreshnessLabel::RevokedOrDisconnected
        ) {
            self.expect(
                freshness
                    .degraded_reason
                    .as_deref()
                    .is_some_and(|reason| !reason.trim().is_empty()),
                "provider_object_alpha.row_freshness_degraded_reason_missing",
                &format!("row {owner} degraded freshness must name a reason"),
            );
        }
    }

    fn expect(&mut self, passed: bool, check_id: &str, message: &str) {
        if !passed {
            self.findings.push(ProviderObjectModelAlphaFinding {
                severity: ProviderObjectModelAlphaFindingSeverity::Error,
                check_id: check_id.to_string(),
                message: message.to_string(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::{FreshnessLabel, FreshnessTruth, ProviderFamily, ProviderObjectKind};

    fn freshness_fresh() -> FreshnessTruth {
        FreshnessTruth {
            freshness_class: FreshnessLabel::Fresh,
            observed_at: Some("2026-05-13T18:00:00Z".to_string()),
            freshness_floor_ref: "freshness.provider.host.fresh".to_string(),
            stale_after: Some("PT10M".to_string()),
            degraded_reason: None,
            import_session_ref: None,
        }
    }

    fn freshness_stale() -> FreshnessTruth {
        FreshnessTruth {
            freshness_class: FreshnessLabel::StaleWithinWindow,
            observed_at: Some("2026-05-13T17:30:00Z".to_string()),
            freshness_floor_ref: "freshness.provider.host.stale".to_string(),
            stale_after: Some("PT45M".to_string()),
            degraded_reason: Some("Issue import is older than the floor.".to_string()),
            import_session_ref: Some("import.session.stale".to_string()),
        }
    }

    fn pr_row(id: &str) -> ProviderObjectRow {
        ProviderObjectRow {
            record_kind: PROVIDER_OBJECT_MODEL_ALPHA_ROW_RECORD_KIND.to_string(),
            schema_version: PROVIDER_OBJECT_MODEL_ALPHA_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_OBJECT_MODEL_ALPHA_SHARED_CONTRACT_REF.to_string(),
            object_row_id: id.to_string(),
            display_label: "Pull request #4012".to_string(),
            provider_descriptor_ref: "provider_descriptor.code_host.primary".to_string(),
            provider_family: ProviderFamily::CodeHost,
            object_kind: ProviderObjectKind::PullRequest,
            target_ref: TargetRef {
                target_ref_class: "code_host.pull_request".to_string(),
                target_ref: "target.code_host.pr.4012".to_string(),
                target_label: "PR #4012".to_string(),
                route_origin: None,
            },
            source: ObjectSource {
                source_class: ObjectSourceClass::LiveProvider,
                canonical_host_ref: "provider.host.code_host.primary".to_string(),
                tenant_or_org_scope_ref: "provider.tenant.aureline".to_string(),
                environment_ref: None,
                import_session_ref: None,
            },
            freshness: freshness_fresh(),
            publish_state: ObjectPublishStateClass::PublishNowPendingReview,
            mode: ObjectModeClass::PublishNowMode,
            degraded_action: DegradedActionClass::NoneRequired,
            local_draft_ref: None,
            publish_later_queue_item_ref: None,
            approval_ticket_ref: Some("approval_ticket.pr.4012".to_string()),
            browser_handoff_packet_ref: None,
            imported_snapshot_ref: None,
            parent_object_row_ref: None,
            audit_event_refs: vec!["audit.event.row.pr.4012".to_string()],
            redaction_class: RedactionClass::MetadataSafeDefault,
            raw_payload_refs_present: false,
            local_editing_preserved: true,
            support_export_summary: "PR row is fresh and pending publish review.".to_string(),
            observed_at: "2026-05-13T18:00:00Z".to_string(),
        }
    }

    fn issue_row_publish_later(id: &str) -> ProviderObjectRow {
        ProviderObjectRow {
            record_kind: PROVIDER_OBJECT_MODEL_ALPHA_ROW_RECORD_KIND.to_string(),
            schema_version: PROVIDER_OBJECT_MODEL_ALPHA_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_OBJECT_MODEL_ALPHA_SHARED_CONTRACT_REF.to_string(),
            object_row_id: id.to_string(),
            display_label: "Issue AUR-104".to_string(),
            provider_descriptor_ref: "provider_descriptor.issue.primary".to_string(),
            provider_family: ProviderFamily::IssueTracker,
            object_kind: ProviderObjectKind::IssueOrWorkItem,
            target_ref: TargetRef {
                target_ref_class: "issue.work_item".to_string(),
                target_ref: "target.issue.aur.104".to_string(),
                target_label: "AUR-104".to_string(),
                route_origin: None,
            },
            source: ObjectSource {
                source_class: ObjectSourceClass::CachedProviderOverlay,
                canonical_host_ref: "provider.host.issue.primary".to_string(),
                tenant_or_org_scope_ref: "provider.tenant.aureline".to_string(),
                environment_ref: None,
                import_session_ref: None,
            },
            freshness: freshness_stale(),
            publish_state: ObjectPublishStateClass::PublishLaterQueued,
            mode: ObjectModeClass::PublishLaterMode,
            degraded_action: DegradedActionClass::HoldForFreshnessRepair,
            local_draft_ref: Some("local_draft.issue.aur.104".to_string()),
            publish_later_queue_item_ref: Some("queue.issue.aur.104.publish".to_string()),
            approval_ticket_ref: None,
            browser_handoff_packet_ref: None,
            imported_snapshot_ref: None,
            parent_object_row_ref: None,
            audit_event_refs: vec!["audit.event.row.issue.aur.104".to_string()],
            redaction_class: RedactionClass::MetadataSafeDefault,
            raw_payload_refs_present: false,
            local_editing_preserved: true,
            support_export_summary:
                "Issue queued for publish-later until upstream import refreshes.".to_string(),
            observed_at: "2026-05-13T17:32:00Z".to_string(),
        }
    }

    fn check_row_inspect(id: &str) -> ProviderObjectRow {
        ProviderObjectRow {
            record_kind: PROVIDER_OBJECT_MODEL_ALPHA_ROW_RECORD_KIND.to_string(),
            schema_version: PROVIDER_OBJECT_MODEL_ALPHA_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_OBJECT_MODEL_ALPHA_SHARED_CONTRACT_REF.to_string(),
            object_row_id: id.to_string(),
            display_label: "Check run smoke-build".to_string(),
            provider_descriptor_ref: "provider_descriptor.ci.primary".to_string(),
            provider_family: ProviderFamily::CiChecks,
            object_kind: ProviderObjectKind::CheckRun,
            target_ref: TargetRef {
                target_ref_class: "ci.check_run".to_string(),
                target_ref: "target.ci.check.smoke".to_string(),
                target_label: "smoke-build".to_string(),
                route_origin: None,
            },
            source: ObjectSource {
                source_class: ObjectSourceClass::ImportedSnapshot,
                canonical_host_ref: "provider.host.ci.primary".to_string(),
                tenant_or_org_scope_ref: "provider.tenant.aureline".to_string(),
                environment_ref: None,
                import_session_ref: Some("import.session.ci.smoke".to_string()),
            },
            freshness: freshness_stale(),
            publish_state: ObjectPublishStateClass::InspectOnlyImported,
            mode: ObjectModeClass::InspectOnlyMode,
            degraded_action: DegradedActionClass::ExportEvidencePacket,
            local_draft_ref: None,
            publish_later_queue_item_ref: None,
            approval_ticket_ref: None,
            browser_handoff_packet_ref: None,
            imported_snapshot_ref: Some("import.snapshot.ci.smoke".to_string()),
            parent_object_row_ref: None,
            audit_event_refs: vec!["audit.event.row.ci.smoke".to_string()],
            redaction_class: RedactionClass::MetadataSafeDefault,
            raw_payload_refs_present: false,
            local_editing_preserved: true,
            support_export_summary: "Check run rendered from imported snapshot.".to_string(),
            observed_at: "2026-05-13T17:00:00Z".to_string(),
        }
    }

    fn open_in_provider_row(id: &str) -> ProviderObjectRow {
        ProviderObjectRow {
            record_kind: PROVIDER_OBJECT_MODEL_ALPHA_ROW_RECORD_KIND.to_string(),
            schema_version: PROVIDER_OBJECT_MODEL_ALPHA_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_OBJECT_MODEL_ALPHA_SHARED_CONTRACT_REF.to_string(),
            object_row_id: id.to_string(),
            display_label: "PR #4013 (browser handoff)".to_string(),
            provider_descriptor_ref: "provider_descriptor.code_host.primary".to_string(),
            provider_family: ProviderFamily::CodeHost,
            object_kind: ProviderObjectKind::PullRequest,
            target_ref: TargetRef {
                target_ref_class: "code_host.pull_request".to_string(),
                target_ref: "target.code_host.pr.4013".to_string(),
                target_label: "PR #4013".to_string(),
                route_origin: None,
            },
            source: ObjectSource {
                source_class: ObjectSourceClass::LiveProvider,
                canonical_host_ref: "provider.host.code_host.primary".to_string(),
                tenant_or_org_scope_ref: "provider.tenant.aureline".to_string(),
                environment_ref: None,
                import_session_ref: None,
            },
            freshness: freshness_fresh(),
            publish_state: ObjectPublishStateClass::OpenInProviderPending,
            mode: ObjectModeClass::OpenInProviderMode,
            degraded_action: DegradedActionClass::NoneRequired,
            local_draft_ref: None,
            publish_later_queue_item_ref: None,
            approval_ticket_ref: None,
            browser_handoff_packet_ref: Some("handoff.packet.pr.4013".to_string()),
            imported_snapshot_ref: None,
            parent_object_row_ref: None,
            audit_event_refs: vec!["audit.event.row.pr.4013".to_string()],
            redaction_class: RedactionClass::MetadataSafeDefault,
            raw_payload_refs_present: false,
            local_editing_preserved: true,
            support_export_summary: "PR is pending typed browser handoff.".to_string(),
            observed_at: "2026-05-13T18:00:00Z".to_string(),
        }
    }

    fn local_draft_row(id: &str) -> ProviderObjectRow {
        ProviderObjectRow {
            record_kind: PROVIDER_OBJECT_MODEL_ALPHA_ROW_RECORD_KIND.to_string(),
            schema_version: PROVIDER_OBJECT_MODEL_ALPHA_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_OBJECT_MODEL_ALPHA_SHARED_CONTRACT_REF.to_string(),
            object_row_id: id.to_string(),
            display_label: "PR #4014 (local draft)".to_string(),
            provider_descriptor_ref: "provider_descriptor.code_host.primary".to_string(),
            provider_family: ProviderFamily::CodeHost,
            object_kind: ProviderObjectKind::PullRequest,
            target_ref: TargetRef {
                target_ref_class: "code_host.pull_request".to_string(),
                target_ref: "target.code_host.pr.4014.draft".to_string(),
                target_label: "PR #4014 (draft)".to_string(),
                route_origin: None,
            },
            source: ObjectSource {
                source_class: ObjectSourceClass::LocalDraftOnly,
                canonical_host_ref: "provider.host.code_host.primary".to_string(),
                tenant_or_org_scope_ref: "provider.tenant.aureline".to_string(),
                environment_ref: None,
                import_session_ref: None,
            },
            freshness: FreshnessTruth {
                freshness_class: FreshnessLabel::NeverObserved,
                observed_at: None,
                freshness_floor_ref: "freshness.provider.host.never".to_string(),
                stale_after: None,
                degraded_reason: Some(
                    "Draft has never been observed by the upstream provider.".to_string(),
                ),
                import_session_ref: None,
            },
            publish_state: ObjectPublishStateClass::LocalDraftOnly,
            mode: ObjectModeClass::LocalDraftMode,
            degraded_action: DegradedActionClass::ContinueLocalAuthoring,
            local_draft_ref: Some("local_draft.pr.4014".to_string()),
            publish_later_queue_item_ref: None,
            approval_ticket_ref: None,
            browser_handoff_packet_ref: None,
            imported_snapshot_ref: None,
            parent_object_row_ref: None,
            audit_event_refs: vec!["audit.event.row.pr.4014.draft".to_string()],
            redaction_class: RedactionClass::MetadataSafeDefault,
            raw_payload_refs_present: false,
            local_editing_preserved: true,
            support_export_summary: "Local-only PR draft retained.".to_string(),
            observed_at: "2026-05-13T18:00:00Z".to_string(),
        }
    }

    fn baseline_page() -> ProviderObjectModelAlphaPage {
        ProviderObjectModelAlphaPage {
            fixture_metadata: None,
            record_kind: PROVIDER_OBJECT_MODEL_ALPHA_PAGE_RECORD_KIND.to_string(),
            schema_version: PROVIDER_OBJECT_MODEL_ALPHA_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_OBJECT_MODEL_ALPHA_SHARED_CONTRACT_REF.to_string(),
            page_id: "provider_object_alpha.page.unit_test".to_string(),
            contract_refs: ProviderObjectModelContractRefs {
                connected_provider_registry_schema_ref:
                    "schemas/providers/connected_provider_registry.schema.json".to_string(),
                publish_later_queue_alpha_schema_ref:
                    "schemas/providers/publish_later_queue_alpha.schema.json".to_string(),
                publish_later_record_schema_ref:
                    "schemas/providers/publish_later_record.schema.json".to_string(),
                browser_handoff_packet_schema_ref:
                    "schemas/providers/browser_handoff_packet.schema.json".to_string(),
                approval_ticket_alpha_schema_ref:
                    "schemas/security/approval_ticket_alpha.schema.json".to_string(),
                change_object_alpha_schema_ref:
                    "schemas/git/change_object_alpha.schema.json".to_string(),
            },
            rows: vec![
                pr_row("row.pr.4012"),
                issue_row_publish_later("row.issue.aur.104"),
                check_row_inspect("row.ci.smoke"),
                open_in_provider_row("row.pr.4013"),
                local_draft_row("row.pr.4014"),
            ],
            continuity_observations: vec![ProviderObjectContinuityObservation {
                record_kind: PROVIDER_OBJECT_MODEL_ALPHA_CONTINUITY_OBSERVATION_RECORD_KIND
                    .to_string(),
                schema_version: PROVIDER_OBJECT_MODEL_ALPHA_SCHEMA_VERSION,
                shared_contract_ref: PROVIDER_OBJECT_MODEL_ALPHA_SHARED_CONTRACT_REF.to_string(),
                observation_id: "observation.issue.aur.104.stale".to_string(),
                object_row_ref: "row.issue.aur.104".to_string(),
                observation_class: ContinuityObservationClass::ProviderStaleWithinWindow,
                retained_capability_class: RetainedCapabilityClass::PublishLaterQueuingRetained,
                degraded_action: DegradedActionClass::HoldForFreshnessRepair,
                rationale_summary:
                    "Issue tracker import older than freshness floor; queue retained for drain.".to_string(),
                observed_at: "2026-05-13T17:32:00Z".to_string(),
                silent_mutation_authority_widened: false,
                local_editing_preserved: true,
            }],
            support_export_summary:
                "Provider-object alpha page for unit-test coverage.".to_string(),
        }
    }

    #[test]
    fn baseline_page_validates() {
        let page = baseline_page();
        let report = page.validate();
        assert!(report.passed, "baseline must pass: {:#?}", report.findings);
        assert!(report
            .coverage
            .modes
            .contains(&ObjectModeClass::LocalDraftMode));
        assert!(report
            .coverage
            .modes
            .contains(&ObjectModeClass::PublishLaterMode));
        assert!(report
            .coverage
            .modes
            .contains(&ObjectModeClass::OpenInProviderMode));
        assert!(report
            .coverage
            .modes
            .contains(&ObjectModeClass::InspectOnlyMode));
    }

    #[test]
    fn missing_approval_ticket_for_publish_now_fails() {
        let mut page = baseline_page();
        page.rows[0].approval_ticket_ref = None;
        let report = page.validate();
        assert!(!report.passed);
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.check_id == "provider_object_alpha.row_approval_ticket_ref_missing"));
    }

    #[test]
    fn local_draft_only_source_must_be_local_draft_mode() {
        let mut page = baseline_page();
        page.rows[4].mode = ObjectModeClass::PublishNowMode;
        page.rows[4].publish_state = ObjectPublishStateClass::PublishNowPendingReview;
        page.rows[4].approval_ticket_ref = Some("approval.draft.4014".to_string());
        let report = page.validate();
        assert!(!report.passed);
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.check_id == "provider_object_alpha.row_local_only_mode"));
    }

    #[test]
    fn raw_payload_flag_must_be_false() {
        let mut page = baseline_page();
        page.rows[1].raw_payload_refs_present = true;
        let report = page.validate();
        assert!(!report.passed);
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.check_id == "provider_object_alpha.row_raw_payload_present"));
    }

    #[test]
    fn local_editing_must_be_preserved() {
        let mut page = baseline_page();
        page.rows[3].local_editing_preserved = false;
        let report = page.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| {
            finding.check_id == "provider_object_alpha.row_local_editing_not_preserved"
        }));
    }

    #[test]
    fn continuity_observation_unknown_row_ref_fails() {
        let mut page = baseline_page();
        page.continuity_observations[0].object_row_ref = "row.unknown".to_string();
        let report = page.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| {
            finding.check_id == "provider_object_alpha.observation_row_ref_unknown"
        }));
    }

    #[test]
    fn no_capability_retained_cannot_continue_local_authoring() {
        let mut page = baseline_page();
        let observation = &mut page.continuity_observations[0];
        observation.retained_capability_class = RetainedCapabilityClass::NoCapabilityRetained;
        observation.degraded_action = DegradedActionClass::ContinueLocalAuthoring;
        observation.local_editing_preserved = false;
        let report = page.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| {
            finding.check_id == "provider_object_alpha.observation_no_capability_continue"
        }));
    }

    #[test]
    fn support_export_excludes_raw_payload_fields() {
        let page = baseline_page();
        let projection = page.support_export_projection();
        let json = serde_json::to_string(&projection).expect("projection serializes");
        assert!(!json.contains("raw_payload"));
        assert!(!json.contains("raw_token"));
        assert!(!json.contains("approval_ticket_ref"));
        assert!(!json.contains("browser_handoff_packet_ref"));
        assert!(!json.contains("imported_snapshot_ref"));
    }
}
