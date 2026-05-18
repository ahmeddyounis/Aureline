//! Provider-event ingestion, replay ledgers, and publish-later reconciliation.
//!
//! This module owns the typed provider-event reconciliation records used by
//! callback ingress, webhook redelivery, polling refresh, provider imports, and
//! deferred publish drains. The model keeps inbound events attributable,
//! deduplicable by delivery identity plus scoped object reference, explicit
//! about partial or mirror-derived truth, and conservative about local drafts
//! whose provider target drifted before publish.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::registry::{
    FreshnessTruth, ProviderFamily, ProviderObjectKind, RedactionClass, TargetRef,
};

/// Schema version exported by provider-event reconciliation records.
pub const PROVIDER_EVENT_RECONCILIATION_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by provider-event reconciliation records.
pub const PROVIDER_EVENT_RECONCILIATION_SHARED_CONTRACT_REF: &str =
    "providers:provider_event_reconciliation:v1";

/// Stable record kind for [`ProviderEventReconciliationPage`].
pub const PROVIDER_EVENT_RECONCILIATION_PAGE_RECORD_KIND: &str =
    "provider_event_reconciliation_page_record";

/// Stable record kind for [`ProviderEventEnvelope`].
pub const PROVIDER_EVENT_ENVELOPE_RECORD_KIND: &str = "provider_event_envelope_record";

/// Stable record kind for [`ImportSession`].
pub const IMPORT_SESSION_RECORD_KIND: &str = "provider_import_session_record";

/// Stable record kind for [`ReplayLedgerItem`].
pub const REPLAY_LEDGER_ITEM_RECORD_KIND: &str = "provider_replay_ledger_item_record";

/// Stable record kind for [`ReconciliationResult`].
pub const RECONCILIATION_RESULT_RECORD_KIND: &str = "provider_reconciliation_result_record";

/// Stable record kind for [`ProviderCallbackDenyEvent`].
pub const PROVIDER_CALLBACK_DENY_EVENT_RECORD_KIND: &str = "provider_callback_deny_event_record";

/// Stable record kind for [`ProviderEventReconciliationValidationReport`].
pub const PROVIDER_EVENT_RECONCILIATION_VALIDATION_REPORT_RECORD_KIND: &str =
    "provider_event_reconciliation_validation_report";

/// Stable record kind for [`ProviderEventReconciliationSupportExport`].
pub const PROVIDER_EVENT_RECONCILIATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "provider_event_reconciliation_support_export";

/// Source class for one inbound provider delivery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderEventSourceClass {
    /// Provider webhook delivery.
    Webhook,
    /// Browser-return callback from a provider handoff.
    BrowserReturnCallback,
    /// Polling or refresh pass against the provider.
    PollingRefresh,
    /// Customer-operated or self-hosted mirror delivery.
    MirrorSync,
    /// Provider import session delivery.
    ImportSession,
    /// Deferred publish queue drain result.
    DeferredPublishQueue,
    /// Reviewed operator replay or backfill.
    OperatorReplay,
}

/// Provider event kind normalized before reconciliation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderEventTypeClass {
    /// Provider-side object was created.
    ObjectCreated,
    /// Provider-side object was updated.
    ObjectUpdated,
    /// Provider-side comment was created.
    CommentCreated,
    /// Provider-side status changed.
    StatusTransition,
    /// CI or check state changed.
    CheckStateChanged,
    /// Permission, grant, or account scope changed.
    PermissionChanged,
    /// Callback or webhook was denied before mutation.
    CallbackDenied,
    /// Missing page or backfill produced imported state.
    ImportPageBackfilled,
    /// Deferred publish drain produced a result.
    PublishDrainResult,
}

/// Proof class attached to a provider event before it can reconcile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceProofClass {
    /// Signature or equivalent provider proof was verified.
    VerifiedSignature,
    /// Provider contract supplies no signature and route proof was verified.
    UnsignedExpectedWithRouteProof,
    /// Browser-return packet proof was verified.
    BrowserReturnPacket,
    /// Mirror digest or customer-operated ingress proof was verified.
    MirrorSignedDigest,
    /// Policy denied the event before provider state changed locally.
    PolicyDeniedProof,
    /// Required proof was missing or invalid and the event was denied.
    ProofMissingDenied,
}

impl SourceProofClass {
    /// Returns true when the proof class denies mutation.
    pub const fn denies_mutation(self) -> bool {
        matches!(self, Self::PolicyDeniedProof | Self::ProofMissingDenied)
    }
}

/// Partial, delayed, backfilled, or mirror-derived truth class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TruthCompletenessClass {
    /// Declared scope was fully materialized.
    FullSnapshot,
    /// Declared scope was partially materialized within known bounds.
    BoundedPartialSnapshot,
    /// Scope was partially materialized without a complete bound.
    UnboundedPartialSnapshot,
    /// Delivery was delayed and may be superseded by later provider truth.
    DelayedDelivery,
    /// Historical state was backfilled.
    BackfilledSnapshot,
    /// State came from a mirror and is not live provider truth.
    MirrorDerivedSnapshot,
    /// No provider-derived state was imported.
    NoStateImported,
    /// Dry-run inspected the payload without writing provider-derived state.
    DryRunNoWrite,
}

impl TruthCompletenessClass {
    /// Returns true when the class must name omitted scope or fields.
    pub const fn requires_omissions(self) -> bool {
        matches!(
            self,
            Self::BoundedPartialSnapshot | Self::UnboundedPartialSnapshot
        )
    }

    /// Returns true when the class is not canonical live provider truth.
    pub const fn is_non_canonical(self) -> bool {
        !matches!(self, Self::FullSnapshot)
    }
}

/// Retryability posture for provider event and reconciliation records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetryabilityClass {
    /// No retry is required.
    NoRetryNeeded,
    /// Retry after refreshing the provider target.
    RetryableAfterRefresh,
    /// Retry after reauthentication.
    RetryableAfterReauth,
    /// Retry after provider scope repair.
    RetryableAfterRescope,
    /// Retry after missing pages or sequence gaps are backfilled.
    RetryableAfterBackfill,
    /// Policy denial is not retryable without policy change.
    NotRetryablePolicyDenied,
    /// Target mismatch is not retryable without retargeting.
    NotRetryableTargetMismatch,
}

/// Final disposition for provider event and replay processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventDispositionClass {
    /// Verified delivery applied exactly once.
    AppliedOnce,
    /// Duplicate delivery was suppressed.
    DedupedNoop,
    /// Redelivery refreshed freshness only.
    FreshnessRefreshedOnly,
    /// Delivery is held for sequence or backfill review.
    HeldPendingSequence,
    /// Partial import applied with explicit omissions.
    PartialImportApplied,
    /// Delivery was denied without user-visible state mutation.
    DeniedNoMutation,
    /// Delivery requires local draft or target reconciliation.
    ReconciliationRequired,
    /// Deferred publish drained only after reconciliation review.
    PublishDrainedAfterReview,
    /// Deferred publish stayed blocked because provider target drifted.
    PublishBlockedDrift,
    /// Dry-run completed without state mutation.
    DryRunOnly,
}

impl EventDispositionClass {
    /// Returns true when this disposition may change user-visible provider-linked state.
    pub const fn is_user_visible_mutation(self) -> bool {
        matches!(
            self,
            Self::AppliedOnce | Self::PartialImportApplied | Self::PublishDrainedAfterReview
        )
    }

    /// Returns true when this disposition is a duplicate or freshness-only outcome.
    pub const fn is_duplicate_safe(self) -> bool {
        matches!(self, Self::DedupedNoop | Self::FreshnessRefreshedOnly)
    }
}

/// Rate-limit posture captured on an import session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RateLimitPostureClass {
    /// No rate limit affected the session.
    NotRateLimited,
    /// Rate limit was respected with complete output.
    BoundedBackoff,
    /// Rate limit caused a partial session.
    PartialRateLimit,
    /// Rate limit state is unknown and must be reviewed.
    UnknownReviewRequired,
}

/// Drift class observed while reconciling a local draft or queued publish.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderDriftClass {
    /// No material provider drift was observed.
    NoMaterialDrift,
    /// Freshness floor is no longer satisfied.
    FreshnessFloorUnsatisfied,
    /// Provider content changed materially.
    TargetContentDrifted,
    /// Provider target identity changed.
    TargetIdentityChanged,
    /// Actor scope changed materially.
    ActorScopeChanged,
    /// Policy epoch changed materially.
    PolicyEpochChanged,
    /// Provider object was deleted or archived remotely.
    ProviderObjectDeleted,
    /// Drift is material but not classifiable yet.
    UnknownMaterialDrift,
}

impl ProviderDriftClass {
    /// Returns true when the drift must block silent provider mutation.
    pub const fn requires_review(self) -> bool {
        !matches!(self, Self::NoMaterialDrift)
    }
}

/// Next safe action after reconciliation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReconciliationNextActionClass {
    /// Provider mutation may proceed through the reviewed drainer.
    MutateProviderNow,
    /// Refresh the provider target and retry.
    RefreshThenRetry,
    /// Reauthenticate and retry.
    ReauthThenRetry,
    /// Repair scope and retry.
    RescopeThenRetry,
    /// Compare, rebase, or review the local draft against provider state.
    CompareRebaseReview,
    /// Cancel locally or export details.
    CancelOrExport,
    /// Open the object in the provider through typed handoff.
    OpenInProvider,
    /// Hold until backfill or sequencing completes.
    HoldForBackfill,
}

impl ReconciliationNextActionClass {
    /// Returns true when this action represents a manual compare/review gate.
    pub const fn is_manual_review_gate(self) -> bool {
        matches!(
            self,
            Self::CompareRebaseReview
                | Self::CancelOrExport
                | Self::OpenInProvider
                | Self::HoldForBackfill
        )
    }
}

/// Reason a callback, webhook, or browser return was denied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallbackDenyReasonClass {
    /// Origin proof was missing or invalid.
    OriginProofInvalid,
    /// Provider host did not match the configured host.
    HostMismatch,
    /// Route class was not allowed for this callback.
    RouteClassDenied,
    /// Policy bundle denied the callback.
    PolicyDenied,
    /// Tenant, org, or project target did not match.
    TenantOrTargetMismatch,
    /// Actor or install ref no longer had scope.
    ActorScopeRevoked,
    /// Freshness floor prevented callback mutation.
    FreshnessFloorDenied,
}

/// Route class observed at callback ingress.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallbackRouteClass {
    /// Browser-return route.
    BrowserReturn,
    /// Provider webhook route.
    ProviderWebhook,
    /// Machine-to-machine callback route.
    MachineToMachineCallback,
    /// Customer-operated mirror ingress route.
    CustomerMirrorIngress,
    /// Route was denied before mutation.
    DeniedRoute,
}

/// Stable delivery identity used for idempotency.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ProviderDeliveryIdentity {
    /// Provider delivery id or opaque delivery ref.
    pub external_delivery_id: String,
    /// Scoped provider object ref the delivery targets.
    pub scoped_object_ref: String,
    /// Provider host ref used to avoid cross-host collisions.
    pub provider_host_ref: String,
    /// Tenant, org, project, or repository scope ref.
    pub tenant_or_org_scope_ref: String,
}

impl ProviderDeliveryIdentity {
    /// Returns true when all identity components are present.
    pub fn is_complete(&self) -> bool {
        !self.external_delivery_id.trim().is_empty()
            && !self.scoped_object_ref.trim().is_empty()
            && !self.provider_host_ref.trim().is_empty()
            && !self.tenant_or_org_scope_ref.trim().is_empty()
    }
}

/// Provider object reference touched by one event or import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderScopedObjectRef {
    /// Opaque local object ref.
    pub object_ref: String,
    /// Provider-side object kind.
    pub object_kind: ProviderObjectKind,
    /// Opaque provider-side remote ref.
    pub provider_remote_ref: String,
    /// Optional local surrogate ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_surrogate_ref: Option<String>,
    /// Target identity bound to this object ref.
    pub target_ref: TargetRef,
    /// Truth class preserved for this object ref.
    pub truth_class: TruthCompletenessClass,
}

/// Omitted provider scope, page, object, or field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderOmission {
    /// Opaque omission id.
    pub omission_id: String,
    /// Class of omitted material.
    pub omission_class: String,
    /// Redaction-safe scope ref for the omission.
    pub scope_ref: String,
    /// Reviewable explanation for why the material was omitted.
    pub reason_summary: String,
}

/// Schema and document references consumed by the reconciliation page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventReconciliationContractRefs {
    /// Provider-event envelope schema ref.
    pub provider_event_envelope_schema_ref: String,
    /// Import-session schema ref.
    pub import_session_schema_ref: String,
    /// Replay-ledger schema ref.
    pub replay_ledger_item_schema_ref: String,
    /// Reconciliation-result schema ref.
    pub reconciliation_result_schema_ref: String,
    /// Callback-deny-event schema ref.
    pub callback_deny_event_schema_ref: String,
    /// Deferred-publish queue schema ref.
    pub deferred_publish_queue_schema_ref: String,
    /// Provider-object schema ref.
    pub provider_object_schema_ref: String,
}

impl ProviderEventReconciliationContractRefs {
    fn all_refs(&self) -> [&str; 7] {
        [
            &self.provider_event_envelope_schema_ref,
            &self.import_session_schema_ref,
            &self.replay_ledger_item_schema_ref,
            &self.reconciliation_result_schema_ref,
            &self.callback_deny_event_schema_ref,
            &self.deferred_publish_queue_schema_ref,
            &self.provider_object_schema_ref,
        ]
    }
}

/// Normalized envelope for one inbound provider delivery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventEnvelope {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable event id.
    pub event_id: String,
    /// Provider descriptor ref.
    pub provider_descriptor_ref: String,
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Inbound source class.
    pub source_class: ProviderEventSourceClass,
    /// Normalized event type.
    pub event_type: ProviderEventTypeClass,
    /// Delivery identity used for idempotency.
    pub delivery_identity: ProviderDeliveryIdentity,
    /// Source proof used before mutation.
    pub source_proof: SourceProofClass,
    /// Provider object refs touched by this delivery.
    pub object_refs: Vec<ProviderScopedObjectRef>,
    /// Provider event time.
    pub event_time: String,
    /// Local ingest time.
    pub ingested_at: String,
    /// Import session that materialized state from this delivery.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_session_ref: Option<String>,
    /// Replay ledger item deciding this delivery.
    pub replay_ledger_item_ref: String,
    /// Freshness truth preserved from provider observation.
    pub freshness: FreshnessTruth,
    /// Completeness class preserved for the imported or observed truth.
    pub truth_class: TruthCompletenessClass,
    /// Explicit omissions for partial, delayed, or denied truth.
    #[serde(default)]
    pub omissions: Vec<ProviderOmission>,
    /// Retryability posture.
    pub retryability: RetryabilityClass,
    /// Final disposition of the event.
    pub final_disposition: EventDispositionClass,
    /// Callback deny event ref when the event was denied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deny_event_ref: Option<String>,
    /// Policy epoch used during event validation.
    pub policy_epoch_ref: String,
    /// Redaction-safe support summary.
    pub support_export_summary: String,
    /// Guardrail: raw provider payload refs are absent from this envelope.
    pub raw_payload_refs_present: bool,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
}

/// Import session that materializes or attempts provider state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportSession {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable import-session id.
    pub import_session_id: String,
    /// Provider descriptor ref.
    pub provider_descriptor_ref: String,
    /// Event refs that caused or bounded this session.
    pub source_event_refs: Vec<String>,
    /// Scope ref imported by this session.
    pub object_scope_ref: String,
    /// Snapshot time for imported provider truth.
    pub snapshot_time: String,
    /// Freshness truth attached to imported objects.
    pub freshness: FreshnessTruth,
    /// Full, partial, delayed, backfilled, mirror, denied, or dry-run class.
    pub truth_class: TruthCompletenessClass,
    /// Explicit omitted scope or fields.
    #[serde(default)]
    pub omissions: Vec<ProviderOmission>,
    /// Rate-limit posture for the session.
    pub rate_limit_posture: RateLimitPostureClass,
    /// Imported object refs produced by this session.
    #[serde(default)]
    pub imported_object_refs: Vec<String>,
    /// Retryability posture.
    pub retryability: RetryabilityClass,
    /// Final disposition for the session.
    pub final_disposition: EventDispositionClass,
    /// Redaction-safe support summary.
    pub support_export_summary: String,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
}

/// Replay or redelivery ledger entry for one delivery identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayLedgerItem {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable replay ledger item id.
    pub replay_ledger_item_id: String,
    /// Delivery identity covered by this ledger item.
    pub delivery_identity: ProviderDeliveryIdentity,
    /// First time this delivery identity was seen.
    pub first_seen_at: String,
    /// Most recent time this delivery identity was seen.
    pub last_seen_at: String,
    /// Replay or redelivery dedupe window.
    pub dedupe_window: String,
    /// Count of additional deliveries beyond the first.
    pub replay_count: u32,
    /// Final replay disposition.
    pub final_disposition: EventDispositionClass,
    /// Retryability posture.
    pub retryability: RetryabilityClass,
    /// Event refs covered by this item.
    pub event_refs: Vec<String>,
    /// Import session ref associated with this replay item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_session_ref: Option<String>,
    /// Audit refs emitted by replay or denial handling.
    #[serde(default)]
    pub audit_event_refs: Vec<String>,
    /// Redaction-safe support summary.
    pub support_export_summary: String,
    /// Guardrail: raw provider payload refs are absent from this item.
    pub raw_payload_refs_present: bool,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
}

/// Subject reconciled before a queued or local draft provider mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconciliationSubject {
    /// Local draft ref.
    pub local_draft_ref: String,
    /// Deferred publish queue item ref.
    pub deferred_publish_queue_item_ref: String,
    /// Target identity bound to the draft.
    pub target_ref: TargetRef,
    /// Expected delivery identity or provider object identity.
    pub expected_identity: ProviderDeliveryIdentity,
}

/// Reconciliation result for a local draft or deferred publish item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconciliationResult {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable reconciliation result id.
    pub reconciliation_result_id: String,
    /// Subject reconciled before provider mutation.
    pub subject: ReconciliationSubject,
    /// Import session containing the queued-time provider snapshot.
    pub baseline_import_session_ref: String,
    /// Import session containing the latest provider snapshot.
    pub latest_import_session_ref: String,
    /// Latest provider snapshot ref.
    pub latest_provider_snapshot_ref: String,
    /// Matched object refs in the latest provider snapshot.
    pub matched_object_refs: Vec<String>,
    /// Created object count.
    pub created_count: u32,
    /// Updated object count.
    pub updated_count: u32,
    /// Skipped object count.
    pub skipped_count: u32,
    /// Conflict refs produced by reconciliation.
    #[serde(default)]
    pub conflict_refs: Vec<String>,
    /// Provider drift class observed before mutation.
    pub drift_class: ProviderDriftClass,
    /// Retryability posture.
    pub retryability: RetryabilityClass,
    /// Final reconciliation disposition.
    pub final_disposition: EventDispositionClass,
    /// Next safe action for the reviewer or drainer.
    pub next_action: ReconciliationNextActionClass,
    /// True only when provider mutation may proceed after revalidation.
    pub safe_to_mutate_provider: bool,
    /// Policy epoch used for reconciliation.
    pub policy_epoch_ref: String,
    /// Auth scope used for reconciliation.
    pub auth_scope_ref: String,
    /// Redaction-safe support summary.
    pub support_export_summary: String,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
}

/// Auditable callback or webhook deny event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderCallbackDenyEvent {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable deny event id.
    pub deny_event_id: String,
    /// Denied provider event ref.
    pub denied_event_ref: String,
    /// Denied delivery identity.
    pub delivery_identity: ProviderDeliveryIdentity,
    /// Deny reason.
    pub reason: CallbackDenyReasonClass,
    /// Policy source or proof source ref.
    pub policy_source_ref: String,
    /// Callback route class.
    pub route_class: CallbackRouteClass,
    /// Actor, install, or service identity ref involved in the deny.
    pub actor_or_install_ref: String,
    /// Remediation hint safe for support export.
    pub remediation_hint: String,
    /// Time the event was denied.
    pub occurred_at: String,
    /// Guardrail: deny did not mutate user-visible provider state.
    pub no_user_visible_mutation: bool,
    /// Audit refs emitted for the deny.
    pub audit_event_refs: Vec<String>,
    /// Redaction-safe support summary.
    pub support_export_summary: String,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
}

/// Fixture metadata used by protected provider-event cases.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventReconciliationFixtureMetadata {
    /// Short fixture name.
    pub name: String,
    /// Reviewer-safe scenario summary.
    pub scenario: String,
}

/// Provider-event reconciliation page containing envelopes, ledgers, imports, denies, and draft reconciliation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventReconciliationPage {
    /// Optional fixture metadata.
    #[serde(
        default,
        rename = "__fixture__",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture_metadata: Option<ProviderEventReconciliationFixtureMetadata>,
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this page.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Contract refs this page consumes.
    pub contract_refs: ProviderEventReconciliationContractRefs,
    /// Provider event envelopes.
    pub event_envelopes: Vec<ProviderEventEnvelope>,
    /// Import sessions.
    pub import_sessions: Vec<ImportSession>,
    /// Replay and redelivery ledger items.
    pub replay_ledger_items: Vec<ReplayLedgerItem>,
    /// Draft and publish-later reconciliation results.
    pub reconciliation_results: Vec<ReconciliationResult>,
    /// Callback deny audit events.
    pub callback_deny_events: Vec<ProviderCallbackDenyEvent>,
    /// Redaction-safe page summary.
    pub support_export_summary: String,
}

impl ProviderEventReconciliationPage {
    /// Validates the page against provider-event reconciliation invariants.
    pub fn validate(&self) -> ProviderEventReconciliationValidationReport {
        let mut validator = Validator::new(self);
        validator.run();
        validator.finish()
    }

    /// Builds a redaction-safe support export projection.
    pub fn support_export_projection(&self) -> ProviderEventReconciliationSupportExport {
        ProviderEventReconciliationSupportExport {
            record_kind: PROVIDER_EVENT_RECONCILIATION_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: PROVIDER_EVENT_RECONCILIATION_SCHEMA_VERSION,
            page_id: self.page_id.clone(),
            event_summaries: self
                .event_envelopes
                .iter()
                .map(ProviderEventEnvelopeSummary::from)
                .collect(),
            import_session_summaries: self
                .import_sessions
                .iter()
                .map(ProviderImportSessionSummary::from)
                .collect(),
            replay_ledger_summaries: self
                .replay_ledger_items
                .iter()
                .map(ReplayLedgerItemSummary::from)
                .collect(),
            reconciliation_summaries: self
                .reconciliation_results
                .iter()
                .map(ReconciliationResultSummary::from)
                .collect(),
            deny_event_summaries: self
                .callback_deny_events
                .iter()
                .map(ProviderCallbackDenyEventSummary::from)
                .collect(),
            redaction_class: RedactionClass::MetadataSafeDefault,
        }
    }
}

/// Validation report emitted by provider-event reconciliation validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventReconciliationValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version validated.
    pub schema_version: u32,
    /// Page id under validation.
    pub page_id: String,
    /// Whether no error-severity checks failed.
    pub passed: bool,
    /// Coverage observed during validation.
    pub coverage: ProviderEventReconciliationCoverage,
    /// Findings emitted by failed checks.
    pub findings: Vec<ProviderEventReconciliationFinding>,
}

/// Coverage observed during provider-event reconciliation validation.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ProviderEventReconciliationCoverage {
    /// Source classes covered by envelopes.
    pub source_classes: BTreeSet<ProviderEventSourceClass>,
    /// Event types covered by envelopes.
    pub event_types: BTreeSet<ProviderEventTypeClass>,
    /// Truth classes covered by events or imports.
    pub truth_classes: BTreeSet<TruthCompletenessClass>,
    /// Final dispositions covered by events, ledgers, imports, or reconciliations.
    pub dispositions: BTreeSet<EventDispositionClass>,
    /// Drift classes covered by reconciliation results.
    pub drift_classes: BTreeSet<ProviderDriftClass>,
    /// Callback deny reasons covered by audit events.
    pub deny_reasons: BTreeSet<CallbackDenyReasonClass>,
}

/// One provider-event reconciliation validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventReconciliationFinding {
    /// Severity.
    pub severity: ProviderEventReconciliationFindingSeverity,
    /// Stable check id.
    pub check_id: String,
    /// Redaction-safe finding message.
    pub message: String,
}

/// Provider-event reconciliation validation finding severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderEventReconciliationFindingSeverity {
    /// Error that blocks validation.
    Error,
    /// Warning that keeps output reviewable but degraded.
    Warning,
}

/// Redaction-safe support export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventReconciliationSupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version exported.
    pub schema_version: u32,
    /// Page id.
    pub page_id: String,
    /// Event summaries.
    pub event_summaries: Vec<ProviderEventEnvelopeSummary>,
    /// Import session summaries.
    pub import_session_summaries: Vec<ProviderImportSessionSummary>,
    /// Replay ledger summaries.
    pub replay_ledger_summaries: Vec<ReplayLedgerItemSummary>,
    /// Reconciliation result summaries.
    pub reconciliation_summaries: Vec<ReconciliationResultSummary>,
    /// Callback deny summaries.
    pub deny_event_summaries: Vec<ProviderCallbackDenyEventSummary>,
    /// Redaction posture for the support export.
    pub redaction_class: RedactionClass,
}

/// Redaction-safe event summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventEnvelopeSummary {
    /// Event id.
    pub event_id: String,
    /// External delivery id.
    pub external_delivery_id: String,
    /// Scoped object ref.
    pub scoped_object_ref: String,
    /// Source class.
    pub source_class: ProviderEventSourceClass,
    /// Event type.
    pub event_type: ProviderEventTypeClass,
    /// Truth class.
    pub truth_class: TruthCompletenessClass,
    /// Final disposition.
    pub final_disposition: EventDispositionClass,
    /// Retryability posture.
    pub retryability: RetryabilityClass,
    /// Support summary.
    pub support_export_summary: String,
}

impl From<&ProviderEventEnvelope> for ProviderEventEnvelopeSummary {
    fn from(envelope: &ProviderEventEnvelope) -> Self {
        Self {
            event_id: envelope.event_id.clone(),
            external_delivery_id: envelope.delivery_identity.external_delivery_id.clone(),
            scoped_object_ref: envelope.delivery_identity.scoped_object_ref.clone(),
            source_class: envelope.source_class,
            event_type: envelope.event_type,
            truth_class: envelope.truth_class,
            final_disposition: envelope.final_disposition,
            retryability: envelope.retryability,
            support_export_summary: envelope.support_export_summary.clone(),
        }
    }
}

/// Redaction-safe import-session summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderImportSessionSummary {
    /// Import session id.
    pub import_session_id: String,
    /// Object scope ref.
    pub object_scope_ref: String,
    /// Truth class.
    pub truth_class: TruthCompletenessClass,
    /// Rate-limit posture.
    pub rate_limit_posture: RateLimitPostureClass,
    /// Final disposition.
    pub final_disposition: EventDispositionClass,
    /// Support summary.
    pub support_export_summary: String,
}

impl From<&ImportSession> for ProviderImportSessionSummary {
    fn from(session: &ImportSession) -> Self {
        Self {
            import_session_id: session.import_session_id.clone(),
            object_scope_ref: session.object_scope_ref.clone(),
            truth_class: session.truth_class,
            rate_limit_posture: session.rate_limit_posture,
            final_disposition: session.final_disposition,
            support_export_summary: session.support_export_summary.clone(),
        }
    }
}

/// Redaction-safe replay-ledger summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayLedgerItemSummary {
    /// Replay ledger item id.
    pub replay_ledger_item_id: String,
    /// External delivery id.
    pub external_delivery_id: String,
    /// Scoped object ref.
    pub scoped_object_ref: String,
    /// Replay count.
    pub replay_count: u32,
    /// Final disposition.
    pub final_disposition: EventDispositionClass,
    /// Support summary.
    pub support_export_summary: String,
}

impl From<&ReplayLedgerItem> for ReplayLedgerItemSummary {
    fn from(item: &ReplayLedgerItem) -> Self {
        Self {
            replay_ledger_item_id: item.replay_ledger_item_id.clone(),
            external_delivery_id: item.delivery_identity.external_delivery_id.clone(),
            scoped_object_ref: item.delivery_identity.scoped_object_ref.clone(),
            replay_count: item.replay_count,
            final_disposition: item.final_disposition,
            support_export_summary: item.support_export_summary.clone(),
        }
    }
}

/// Redaction-safe reconciliation-result summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconciliationResultSummary {
    /// Reconciliation result id.
    pub reconciliation_result_id: String,
    /// Deferred publish queue item ref.
    pub deferred_publish_queue_item_ref: String,
    /// Drift class.
    pub drift_class: ProviderDriftClass,
    /// Final disposition.
    pub final_disposition: EventDispositionClass,
    /// Next safe action.
    pub next_action: ReconciliationNextActionClass,
    /// Whether provider mutation may proceed.
    pub safe_to_mutate_provider: bool,
    /// Support summary.
    pub support_export_summary: String,
}

impl From<&ReconciliationResult> for ReconciliationResultSummary {
    fn from(result: &ReconciliationResult) -> Self {
        Self {
            reconciliation_result_id: result.reconciliation_result_id.clone(),
            deferred_publish_queue_item_ref: result.subject.deferred_publish_queue_item_ref.clone(),
            drift_class: result.drift_class,
            final_disposition: result.final_disposition,
            next_action: result.next_action,
            safe_to_mutate_provider: result.safe_to_mutate_provider,
            support_export_summary: result.support_export_summary.clone(),
        }
    }
}

/// Redaction-safe callback-deny summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderCallbackDenyEventSummary {
    /// Deny event id.
    pub deny_event_id: String,
    /// Denied event ref.
    pub denied_event_ref: String,
    /// Deny reason.
    pub reason: CallbackDenyReasonClass,
    /// Route class.
    pub route_class: CallbackRouteClass,
    /// Support summary.
    pub support_export_summary: String,
}

impl From<&ProviderCallbackDenyEvent> for ProviderCallbackDenyEventSummary {
    fn from(event: &ProviderCallbackDenyEvent) -> Self {
        Self {
            deny_event_id: event.deny_event_id.clone(),
            denied_event_ref: event.denied_event_ref.clone(),
            reason: event.reason,
            route_class: event.route_class,
            support_export_summary: event.support_export_summary.clone(),
        }
    }
}

struct Validator<'a> {
    page: &'a ProviderEventReconciliationPage,
    event_ids: BTreeSet<&'a str>,
    import_session_ids: BTreeSet<&'a str>,
    replay_item_ids: BTreeSet<&'a str>,
    reconciliation_ids: BTreeSet<&'a str>,
    deny_event_ids: BTreeSet<&'a str>,
    delivery_groups: BTreeMap<&'a ProviderDeliveryIdentity, Vec<&'a ProviderEventEnvelope>>,
    coverage: ProviderEventReconciliationCoverage,
    findings: Vec<ProviderEventReconciliationFinding>,
}

impl<'a> Validator<'a> {
    fn new(page: &'a ProviderEventReconciliationPage) -> Self {
        Self {
            page,
            event_ids: BTreeSet::new(),
            import_session_ids: BTreeSet::new(),
            replay_item_ids: BTreeSet::new(),
            reconciliation_ids: BTreeSet::new(),
            deny_event_ids: BTreeSet::new(),
            delivery_groups: BTreeMap::new(),
            coverage: ProviderEventReconciliationCoverage::default(),
            findings: Vec::new(),
        }
    }

    fn run(&mut self) {
        self.validate_page_header();
        self.collect_ids();
        self.validate_event_envelopes();
        self.validate_import_sessions();
        self.validate_replay_ledger_items();
        self.validate_reconciliation_results();
        self.validate_callback_deny_events();
        self.validate_delivery_idempotency();
        self.validate_required_coverage();
    }

    fn finish(self) -> ProviderEventReconciliationValidationReport {
        let passed = self
            .findings
            .iter()
            .all(|finding| finding.severity != ProviderEventReconciliationFindingSeverity::Error);
        ProviderEventReconciliationValidationReport {
            record_kind: PROVIDER_EVENT_RECONCILIATION_VALIDATION_REPORT_RECORD_KIND.to_string(),
            schema_version: PROVIDER_EVENT_RECONCILIATION_SCHEMA_VERSION,
            page_id: self.page.page_id.clone(),
            passed,
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn validate_page_header(&mut self) {
        self.expect(
            self.page.record_kind == PROVIDER_EVENT_RECONCILIATION_PAGE_RECORD_KIND,
            "provider_event_reconciliation.page_record_kind",
            "page.record_kind must match the provider-event reconciliation page record kind",
        );
        self.expect(
            self.page.schema_version == PROVIDER_EVENT_RECONCILIATION_SCHEMA_VERSION,
            "provider_event_reconciliation.page_schema_version",
            "page.schema_version must match the crate constant",
        );
        self.expect(
            self.page.shared_contract_ref == PROVIDER_EVENT_RECONCILIATION_SHARED_CONTRACT_REF,
            "provider_event_reconciliation.page_shared_contract_ref",
            "page.shared_contract_ref must match the shared contract id",
        );
        self.expect(
            !self.page.page_id.trim().is_empty(),
            "provider_event_reconciliation.page_id_missing",
            "page.page_id must be non-empty",
        );
        self.expect(
            !self.page.support_export_summary.trim().is_empty(),
            "provider_event_reconciliation.page_summary_missing",
            "page.support_export_summary must be non-empty",
        );
        for contract_ref in self.page.contract_refs.all_refs() {
            self.expect(
                !contract_ref.trim().is_empty(),
                "provider_event_reconciliation.contract_ref_missing",
                "every consumed contract ref must be non-empty",
            );
        }
        self.expect(
            !self.page.event_envelopes.is_empty(),
            "provider_event_reconciliation.events_missing",
            "page must contain at least one provider event envelope",
        );
        self.expect(
            !self.page.replay_ledger_items.is_empty(),
            "provider_event_reconciliation.ledger_missing",
            "page must contain at least one replay ledger item",
        );
    }

    fn collect_ids(&mut self) {
        for event in &self.page.event_envelopes {
            let inserted = self.event_ids.insert(event.event_id.as_str());
            self.expect(
                inserted,
                "provider_event_reconciliation.event_id_duplicate",
                "event.event_id must be unique",
            );
            self.delivery_groups
                .entry(&event.delivery_identity)
                .or_default()
                .push(event);
        }
        for session in &self.page.import_sessions {
            let inserted = self
                .import_session_ids
                .insert(session.import_session_id.as_str());
            self.expect(
                inserted,
                "provider_event_reconciliation.import_session_id_duplicate",
                "import_session.import_session_id must be unique",
            );
        }
        for item in &self.page.replay_ledger_items {
            let inserted = self
                .replay_item_ids
                .insert(item.replay_ledger_item_id.as_str());
            self.expect(
                inserted,
                "provider_event_reconciliation.ledger_id_duplicate",
                "replay_ledger_item.replay_ledger_item_id must be unique",
            );
        }
        for result in &self.page.reconciliation_results {
            let inserted = self
                .reconciliation_ids
                .insert(result.reconciliation_result_id.as_str());
            self.expect(
                inserted,
                "provider_event_reconciliation.reconciliation_id_duplicate",
                "reconciliation_result.reconciliation_result_id must be unique",
            );
        }
        for event in &self.page.callback_deny_events {
            let inserted = self.deny_event_ids.insert(event.deny_event_id.as_str());
            self.expect(
                inserted,
                "provider_event_reconciliation.deny_event_id_duplicate",
                "callback_deny_event.deny_event_id must be unique",
            );
        }
    }

    fn validate_event_envelopes(&mut self) {
        for event in &self.page.event_envelopes {
            self.coverage.source_classes.insert(event.source_class);
            self.coverage.event_types.insert(event.event_type);
            self.coverage.truth_classes.insert(event.truth_class);
            self.coverage.dispositions.insert(event.final_disposition);
            self.expect_record(
                event.record_kind.as_str(),
                event.schema_version,
                event.shared_contract_ref.as_str(),
                PROVIDER_EVENT_ENVELOPE_RECORD_KIND,
                "provider_event_reconciliation.event",
            );
            self.expect(
                event.delivery_identity.is_complete(),
                "provider_event_reconciliation.event_delivery_identity_incomplete",
                "event delivery identity must include delivery id, scoped object, provider host, and tenant/org scope",
            );
            self.expect(
                !event.object_refs.is_empty(),
                "provider_event_reconciliation.event_object_refs_missing",
                "event must name at least one scoped object ref",
            );
            self.expect(
                !event.replay_ledger_item_ref.trim().is_empty()
                    && self
                        .replay_item_ids
                        .contains(event.replay_ledger_item_ref.as_str()),
                "provider_event_reconciliation.event_ledger_ref_missing",
                "event.replay_ledger_item_ref must reference a replay ledger item",
            );
            if let Some(import_session_ref) = &event.import_session_ref {
                self.expect(
                    self.import_session_ids
                        .contains(import_session_ref.as_str()),
                    "provider_event_reconciliation.event_import_session_ref_unknown",
                    "event.import_session_ref must reference an import session",
                );
            }
            self.expect(
                !event.support_export_summary.trim().is_empty(),
                "provider_event_reconciliation.event_summary_missing",
                "event.support_export_summary must be non-empty",
            );
            self.expect(
                !event.raw_payload_refs_present,
                "provider_event_reconciliation.event_raw_payload_ref_present",
                "event envelope must not carry raw provider payload refs",
            );
            if event.truth_class.requires_omissions() {
                self.expect(
                    !event.omissions.is_empty(),
                    "provider_event_reconciliation.event_partial_omissions_missing",
                    "partial event truth must name explicit omissions",
                );
            }
            if event.source_proof.denies_mutation() {
                self.expect(
                    event.final_disposition == EventDispositionClass::DeniedNoMutation,
                    "provider_event_reconciliation.event_denied_disposition_wrong",
                    "denied source proof must end with denied_no_mutation disposition",
                );
            }
            if event.final_disposition == EventDispositionClass::DeniedNoMutation {
                self.expect(
                    event
                        .deny_event_ref
                        .as_ref()
                        .is_some_and(|deny_ref| self.deny_event_ids.contains(deny_ref.as_str())),
                    "provider_event_reconciliation.event_deny_ref_missing",
                    "denied event must reference an auditable callback deny event",
                );
                self.expect(
                    !event.final_disposition.is_user_visible_mutation(),
                    "provider_event_reconciliation.denied_event_mutated",
                    "denied event must not mutate user-visible provider state",
                );
            }
            for object_ref in &event.object_refs {
                self.expect(
                    !object_ref.object_ref.trim().is_empty()
                        && !object_ref.provider_remote_ref.trim().is_empty(),
                    "provider_event_reconciliation.object_ref_incomplete",
                    "event object refs must include local and provider refs",
                );
            }
        }
    }

    fn validate_import_sessions(&mut self) {
        for session in &self.page.import_sessions {
            self.coverage.truth_classes.insert(session.truth_class);
            self.coverage.dispositions.insert(session.final_disposition);
            self.expect_record(
                session.record_kind.as_str(),
                session.schema_version,
                session.shared_contract_ref.as_str(),
                IMPORT_SESSION_RECORD_KIND,
                "provider_event_reconciliation.import_session",
            );
            self.expect(
                !session.source_event_refs.is_empty(),
                "provider_event_reconciliation.import_source_events_missing",
                "import session must cite source event refs",
            );
            for event_ref in &session.source_event_refs {
                self.expect(
                    self.event_ids.contains(event_ref.as_str()),
                    "provider_event_reconciliation.import_source_event_unknown",
                    "import session source_event_refs must reference provider events",
                );
            }
            self.expect(
                !session.object_scope_ref.trim().is_empty(),
                "provider_event_reconciliation.import_scope_missing",
                "import session must name object scope",
            );
            if session.truth_class.requires_omissions() {
                self.expect(
                    !session.omissions.is_empty(),
                    "provider_event_reconciliation.import_partial_omissions_missing",
                    "partial import session must name explicit omissions",
                );
            }
            self.expect(
                !(session.truth_class == TruthCompletenessClass::FullSnapshot
                    && session.final_disposition == EventDispositionClass::PartialImportApplied),
                "provider_event_reconciliation.import_full_marked_partial",
                "full import sessions cannot use partial-import disposition",
            );
            self.expect(
                !session.support_export_summary.trim().is_empty(),
                "provider_event_reconciliation.import_summary_missing",
                "import session support summary must be non-empty",
            );
        }
    }

    fn validate_replay_ledger_items(&mut self) {
        for item in &self.page.replay_ledger_items {
            self.coverage.dispositions.insert(item.final_disposition);
            self.expect_record(
                item.record_kind.as_str(),
                item.schema_version,
                item.shared_contract_ref.as_str(),
                REPLAY_LEDGER_ITEM_RECORD_KIND,
                "provider_event_reconciliation.replay_ledger",
            );
            self.expect(
                item.delivery_identity.is_complete(),
                "provider_event_reconciliation.ledger_delivery_identity_incomplete",
                "replay ledger delivery identity must be complete",
            );
            self.expect(
                !item.event_refs.is_empty(),
                "provider_event_reconciliation.ledger_event_refs_missing",
                "replay ledger item must cite event refs",
            );
            for event_ref in &item.event_refs {
                self.expect(
                    self.event_ids.contains(event_ref.as_str()),
                    "provider_event_reconciliation.ledger_event_ref_unknown",
                    "replay ledger item event_refs must reference provider events",
                );
            }
            if item.final_disposition.is_duplicate_safe() {
                self.expect(
                    item.replay_count > 0,
                    "provider_event_reconciliation.ledger_duplicate_count_missing",
                    "dedupe or freshness-only replay items must increment replay_count",
                );
            }
            if item.final_disposition == EventDispositionClass::DeniedNoMutation {
                self.expect(
                    !item.audit_event_refs.is_empty(),
                    "provider_event_reconciliation.ledger_denial_audit_missing",
                    "denied replay ledger items must cite audit refs",
                );
            }
            self.expect(
                !item.raw_payload_refs_present,
                "provider_event_reconciliation.ledger_raw_payload_ref_present",
                "replay ledger item must not carry raw provider payload refs",
            );
            self.expect(
                !item.support_export_summary.trim().is_empty(),
                "provider_event_reconciliation.ledger_summary_missing",
                "replay ledger support summary must be non-empty",
            );
        }
    }

    fn validate_reconciliation_results(&mut self) {
        for result in &self.page.reconciliation_results {
            self.coverage.drift_classes.insert(result.drift_class);
            self.coverage.dispositions.insert(result.final_disposition);
            self.expect_record(
                result.record_kind.as_str(),
                result.schema_version,
                result.shared_contract_ref.as_str(),
                RECONCILIATION_RESULT_RECORD_KIND,
                "provider_event_reconciliation.reconciliation",
            );
            self.expect(
                !result.subject.local_draft_ref.trim().is_empty()
                    && !result
                        .subject
                        .deferred_publish_queue_item_ref
                        .trim()
                        .is_empty(),
                "provider_event_reconciliation.reconciliation_subject_missing",
                "reconciliation subject must name local draft and deferred publish queue refs",
            );
            self.expect(
                self.import_session_ids
                    .contains(result.baseline_import_session_ref.as_str())
                    && self
                        .import_session_ids
                        .contains(result.latest_import_session_ref.as_str()),
                "provider_event_reconciliation.reconciliation_import_refs_unknown",
                "reconciliation must reference known baseline and latest import sessions",
            );
            if result.drift_class.requires_review() {
                self.expect(
                    !result.safe_to_mutate_provider,
                    "provider_event_reconciliation.reconciliation_drift_safe_to_mutate",
                    "material provider drift must block silent provider mutation",
                );
                self.expect(
                    result.next_action.is_manual_review_gate()
                        || matches!(
                            result.next_action,
                            ReconciliationNextActionClass::RefreshThenRetry
                                | ReconciliationNextActionClass::ReauthThenRetry
                                | ReconciliationNextActionClass::RescopeThenRetry
                        ),
                    "provider_event_reconciliation.reconciliation_drift_next_action",
                    "material provider drift must force compare, rebase, review, repair, or export",
                );
                self.expect(
                    matches!(
                        result.final_disposition,
                        EventDispositionClass::PublishBlockedDrift
                            | EventDispositionClass::ReconciliationRequired
                    ),
                    "provider_event_reconciliation.reconciliation_drift_disposition",
                    "material provider drift must not claim committed provider mutation",
                );
            } else {
                self.expect(
                    result.safe_to_mutate_provider
                        && result.next_action == ReconciliationNextActionClass::MutateProviderNow,
                    "provider_event_reconciliation.reconciliation_clean_not_mutable",
                    "no-drift reconciliation must be ready for reviewed provider mutation",
                );
            }
            self.expect(
                !result.support_export_summary.trim().is_empty(),
                "provider_event_reconciliation.reconciliation_summary_missing",
                "reconciliation support summary must be non-empty",
            );
        }
    }

    fn validate_callback_deny_events(&mut self) {
        for event in &self.page.callback_deny_events {
            self.coverage.deny_reasons.insert(event.reason);
            self.expect_record(
                event.record_kind.as_str(),
                event.schema_version,
                event.shared_contract_ref.as_str(),
                PROVIDER_CALLBACK_DENY_EVENT_RECORD_KIND,
                "provider_event_reconciliation.callback_deny",
            );
            self.expect(
                self.event_ids.contains(event.denied_event_ref.as_str()),
                "provider_event_reconciliation.callback_deny_event_ref_unknown",
                "callback deny event must reference a known provider event",
            );
            self.expect(
                event.delivery_identity.is_complete(),
                "provider_event_reconciliation.callback_deny_identity_incomplete",
                "callback deny event delivery identity must be complete",
            );
            self.expect(
                event.no_user_visible_mutation,
                "provider_event_reconciliation.callback_deny_mutated",
                "callback deny event must not mutate user-visible provider state",
            );
            self.expect(
                !event.audit_event_refs.is_empty(),
                "provider_event_reconciliation.callback_deny_audit_missing",
                "callback deny event must cite audit refs",
            );
            self.expect(
                !event.support_export_summary.trim().is_empty()
                    && !event.remediation_hint.trim().is_empty(),
                "provider_event_reconciliation.callback_deny_summary_missing",
                "callback deny event must include support summary and remediation hint",
            );
        }
    }

    fn validate_delivery_idempotency(&mut self) {
        let groups: Vec<(ProviderDeliveryIdentity, Vec<ProviderEventEnvelope>)> = self
            .delivery_groups
            .iter()
            .map(|(identity, events)| {
                (
                    (*identity).clone(),
                    events.iter().map(|event| (*event).clone()).collect(),
                )
            })
            .collect();
        for (identity, events) in groups {
            let mutating_count = events
                .iter()
                .filter(|event| event.final_disposition.is_user_visible_mutation())
                .count();
            self.expect(
                mutating_count <= 1,
                "provider_event_reconciliation.delivery_duplicate_mutation",
                "duplicate provider delivery identity must not mutate user-visible state more than once",
            );
            if events.len() > 1 {
                self.expect(
                    events
                        .iter()
                        .any(|event| event.final_disposition.is_duplicate_safe()),
                    "provider_event_reconciliation.delivery_duplicate_no_dedupe",
                    "duplicate delivery identity must include a deduped or freshness-only event",
                );
                self.expect(
                    self.page.replay_ledger_items.iter().any(|item| {
                        item.delivery_identity == identity
                            && item.replay_count > 0
                            && item.final_disposition.is_duplicate_safe()
                    }),
                    "provider_event_reconciliation.delivery_duplicate_ledger_missing",
                    "duplicate delivery identity must have a replay ledger item with replay_count",
                );
            }
        }
    }

    fn validate_required_coverage(&mut self) {
        for source_class in [
            ProviderEventSourceClass::Webhook,
            ProviderEventSourceClass::BrowserReturnCallback,
            ProviderEventSourceClass::MirrorSync,
            ProviderEventSourceClass::DeferredPublishQueue,
        ] {
            self.expect(
                self.coverage.source_classes.contains(&source_class),
                "provider_event_reconciliation.coverage_source_class_missing",
                "coverage must include webhook, browser-return, mirror, and deferred-publish sources",
            );
        }
        for disposition in [
            EventDispositionClass::AppliedOnce,
            EventDispositionClass::DedupedNoop,
            EventDispositionClass::DeniedNoMutation,
            EventDispositionClass::PublishBlockedDrift,
            EventDispositionClass::PublishDrainedAfterReview,
        ] {
            self.expect(
                self.coverage.dispositions.contains(&disposition),
                "provider_event_reconciliation.coverage_disposition_missing",
                "coverage must include applied, deduped, denied, blocked-drift, and drained-after-review dispositions",
            );
        }
        for truth_class in [
            TruthCompletenessClass::FullSnapshot,
            TruthCompletenessClass::BoundedPartialSnapshot,
            TruthCompletenessClass::MirrorDerivedSnapshot,
        ] {
            self.expect(
                self.coverage.truth_classes.contains(&truth_class),
                "provider_event_reconciliation.coverage_truth_class_missing",
                "coverage must include full, bounded-partial, and mirror-derived truth classes",
            );
        }
        self.expect(
            self.coverage
                .drift_classes
                .contains(&ProviderDriftClass::TargetContentDrifted),
            "provider_event_reconciliation.coverage_drift_missing",
            "coverage must include material provider drift",
        );
        self.expect(
            !self.coverage.deny_reasons.is_empty(),
            "provider_event_reconciliation.coverage_deny_reason_missing",
            "coverage must include at least one callback deny reason",
        );
    }

    fn expect_record(
        &mut self,
        record_kind: &str,
        schema_version: u32,
        shared_contract_ref: &str,
        expected_record_kind: &str,
        check_prefix: &str,
    ) {
        self.expect(
            record_kind == expected_record_kind,
            &format!("{check_prefix}.record_kind"),
            "record_kind must match the expected discriminator",
        );
        self.expect(
            schema_version == PROVIDER_EVENT_RECONCILIATION_SCHEMA_VERSION,
            &format!("{check_prefix}.schema_version"),
            "schema_version must match the crate constant",
        );
        self.expect(
            shared_contract_ref == PROVIDER_EVENT_RECONCILIATION_SHARED_CONTRACT_REF,
            &format!("{check_prefix}.shared_contract_ref"),
            "shared_contract_ref must match the shared contract id",
        );
    }

    fn expect(&mut self, condition: bool, check_id: &str, message: &str) {
        if !condition {
            self.findings.push(ProviderEventReconciliationFinding {
                severity: ProviderEventReconciliationFindingSeverity::Error,
                check_id: check_id.to_string(),
                message: message.to_string(),
            });
        }
    }
}

/// Builds a seeded provider-event reconciliation page for fixtures and CLIs.
pub fn seeded_provider_event_reconciliation_page() -> ProviderEventReconciliationPage {
    let identity_pr =
        delivery_identity("provider.delivery.1001", "provider.object.pr.42.comment.7");
    let identity_issue = delivery_identity("provider.delivery.2001", "provider.object.issue.84");
    let identity_mirror = delivery_identity("provider.delivery.3001", "provider.object.check.510");
    let identity_deny = delivery_identity("provider.delivery.4001", "provider.object.pr.42");
    let identity_publish = delivery_identity("provider.delivery.5001", "provider.object.issue.84");

    ProviderEventReconciliationPage {
        fixture_metadata: Some(ProviderEventReconciliationFixtureMetadata {
            name: "provider_event_reconciliation".to_string(),
            scenario: "Provider event envelopes, replay ledgers, import sessions, callback denies, and publish-later reconciliation preserve idempotency, partial truth, and drift review.".to_string(),
        }),
        record_kind: PROVIDER_EVENT_RECONCILIATION_PAGE_RECORD_KIND.to_string(),
        schema_version: PROVIDER_EVENT_RECONCILIATION_SCHEMA_VERSION,
        shared_contract_ref: PROVIDER_EVENT_RECONCILIATION_SHARED_CONTRACT_REF.to_string(),
        page_id: "providers.event_reconciliation.page".to_string(),
        contract_refs: ProviderEventReconciliationContractRefs {
            provider_event_envelope_schema_ref:
                "schemas/providers/provider_event_envelope.schema.json".to_string(),
            import_session_schema_ref: "schemas/providers/import_session.schema.json".to_string(),
            replay_ledger_item_schema_ref:
                "schemas/providers/replay_ledger_item.schema.json".to_string(),
            reconciliation_result_schema_ref:
                "schemas/providers/reconciliation_result.schema.json".to_string(),
            callback_deny_event_schema_ref:
                "schemas/providers/provider_callback_deny_event.schema.json".to_string(),
            deferred_publish_queue_schema_ref:
                "schemas/providers/deferred_publish_queue_item.schema.json".to_string(),
            provider_object_schema_ref: "schemas/providers/provider_object.schema.json".to_string(),
        },
        event_envelopes: vec![
            event_envelope(
                "provider_event.event.pr_comment.applied",
                ProviderEventSourceClass::Webhook,
                ProviderEventTypeClass::CommentCreated,
                identity_pr.clone(),
                SourceProofClass::VerifiedSignature,
                TruthCompletenessClass::FullSnapshot,
                EventDispositionClass::AppliedOnce,
                RetryabilityClass::NoRetryNeeded,
                Some("provider_import.session.pr_comment.full"),
                "provider_replay.ledger.pr_comment",
                None,
                vec![],
                "Verified review-comment delivery applied once.",
            ),
            event_envelope(
                "provider_event.event.pr_comment.duplicate",
                ProviderEventSourceClass::Webhook,
                ProviderEventTypeClass::CommentCreated,
                identity_pr.clone(),
                SourceProofClass::VerifiedSignature,
                TruthCompletenessClass::FullSnapshot,
                EventDispositionClass::DedupedNoop,
                RetryabilityClass::NoRetryNeeded,
                Some("provider_import.session.pr_comment.full"),
                "provider_replay.ledger.pr_comment_redelivery",
                None,
                vec![],
                "Duplicate review-comment delivery refreshed freshness only.",
            ),
            event_envelope(
                "provider_event.event.issue.partial",
                ProviderEventSourceClass::BrowserReturnCallback,
                ProviderEventTypeClass::StatusTransition,
                identity_issue.clone(),
                SourceProofClass::BrowserReturnPacket,
                TruthCompletenessClass::BoundedPartialSnapshot,
                EventDispositionClass::PartialImportApplied,
                RetryabilityClass::RetryableAfterBackfill,
                Some("provider_import.session.issue.partial"),
                "provider_replay.ledger.issue.partial",
                None,
                vec![omission(
                    "provider_omission.issue.comments",
                    "comments_page",
                    "provider.scope.issue.84.comments.page.2",
                    "Second comments page was rate limited and remains omitted.",
                )],
                "Issue transition imported with an explicit bounded omission.",
            ),
            event_envelope(
                "provider_event.event.check.mirror",
                ProviderEventSourceClass::MirrorSync,
                ProviderEventTypeClass::CheckStateChanged,
                identity_mirror.clone(),
                SourceProofClass::MirrorSignedDigest,
                TruthCompletenessClass::MirrorDerivedSnapshot,
                EventDispositionClass::AppliedOnce,
                RetryabilityClass::RetryableAfterRefresh,
                Some("provider_import.session.check.mirror"),
                "provider_replay.ledger.check.mirror",
                None,
                vec![],
                "Mirror-derived check state stayed labeled as mirrored truth.",
            ),
            event_envelope(
                "provider_event.event.callback.denied",
                ProviderEventSourceClass::BrowserReturnCallback,
                ProviderEventTypeClass::CallbackDenied,
                identity_deny.clone(),
                SourceProofClass::ProofMissingDenied,
                TruthCompletenessClass::NoStateImported,
                EventDispositionClass::DeniedNoMutation,
                RetryabilityClass::NotRetryableTargetMismatch,
                None,
                "provider_replay.ledger.callback.denied",
                Some("provider_callback_deny.event.host_mismatch"),
                vec![omission(
                    "provider_omission.callback.host",
                    "host_scope",
                    "provider.scope.host.expected",
                    "Callback host did not match the configured provider host.",
                )],
                "Callback was denied and exported without mutating provider state.",
            ),
            event_envelope(
                "provider_event.event.publish_later.blocked",
                ProviderEventSourceClass::DeferredPublishQueue,
                ProviderEventTypeClass::PublishDrainResult,
                identity_publish.clone(),
                SourceProofClass::VerifiedSignature,
                TruthCompletenessClass::FullSnapshot,
                EventDispositionClass::PublishBlockedDrift,
                RetryabilityClass::RetryableAfterRefresh,
                Some("provider_import.session.issue.latest"),
                "provider_replay.ledger.publish.blocked",
                None,
                vec![],
                "Queued publish found provider drift and forced compare review.",
            ),
            event_envelope(
                "provider_event.event.publish_later.drained",
                ProviderEventSourceClass::DeferredPublishQueue,
                ProviderEventTypeClass::PublishDrainResult,
                delivery_identity("provider.delivery.5002", "provider.object.issue.85"),
                SourceProofClass::VerifiedSignature,
                TruthCompletenessClass::FullSnapshot,
                EventDispositionClass::PublishDrainedAfterReview,
                RetryabilityClass::NoRetryNeeded,
                Some("provider_import.session.issue.clean_latest"),
                "provider_replay.ledger.publish.drained",
                None,
                vec![],
                "Queued publish drained after clean reconciliation.",
            ),
        ],
        import_sessions: vec![
            import_session(
                "provider_import.session.pr_comment.full",
                vec![
                    "provider_event.event.pr_comment.applied",
                    "provider_event.event.pr_comment.duplicate",
                ],
                "provider.scope.pr.42.comments",
                TruthCompletenessClass::FullSnapshot,
                RateLimitPostureClass::NotRateLimited,
                EventDispositionClass::AppliedOnce,
                vec![],
                vec!["provider.object.pr.42.comment.7"],
                "Full comment snapshot materialized for the review object.",
            ),
            import_session(
                "provider_import.session.issue.partial",
                vec!["provider_event.event.issue.partial"],
                "provider.scope.issue.84",
                TruthCompletenessClass::BoundedPartialSnapshot,
                RateLimitPostureClass::PartialRateLimit,
                EventDispositionClass::PartialImportApplied,
                vec![omission(
                    "provider_omission.issue.comments",
                    "comments_page",
                    "provider.scope.issue.84.comments.page.2",
                    "Second comments page was rate limited and remains omitted.",
                )],
                vec!["provider.object.issue.84"],
                "Issue import disclosed bounded partial truth and omissions.",
            ),
            import_session(
                "provider_import.session.check.mirror",
                vec!["provider_event.event.check.mirror"],
                "provider.scope.check.510",
                TruthCompletenessClass::MirrorDerivedSnapshot,
                RateLimitPostureClass::BoundedBackoff,
                EventDispositionClass::AppliedOnce,
                vec![],
                vec!["provider.object.check.510"],
                "Check state arrived through a customer mirror and stayed non-canonical.",
            ),
            import_session(
                "provider_import.session.issue.baseline",
                vec!["provider_event.event.issue.partial"],
                "provider.scope.issue.84",
                TruthCompletenessClass::FullSnapshot,
                RateLimitPostureClass::NotRateLimited,
                EventDispositionClass::AppliedOnce,
                vec![],
                vec!["provider.object.issue.84"],
                "Queued-time issue snapshot used as reconciliation baseline.",
            ),
            import_session(
                "provider_import.session.issue.latest",
                vec!["provider_event.event.publish_later.blocked"],
                "provider.scope.issue.84",
                TruthCompletenessClass::FullSnapshot,
                RateLimitPostureClass::NotRateLimited,
                EventDispositionClass::ReconciliationRequired,
                vec![],
                vec!["provider.object.issue.84"],
                "Latest provider snapshot showed material drift.",
            ),
            import_session(
                "provider_import.session.issue.clean_baseline",
                vec!["provider_event.event.publish_later.drained"],
                "provider.scope.issue.85",
                TruthCompletenessClass::FullSnapshot,
                RateLimitPostureClass::NotRateLimited,
                EventDispositionClass::AppliedOnce,
                vec![],
                vec!["provider.object.issue.85"],
                "Queued-time clean issue snapshot.",
            ),
            import_session(
                "provider_import.session.issue.clean_latest",
                vec!["provider_event.event.publish_later.drained"],
                "provider.scope.issue.85",
                TruthCompletenessClass::FullSnapshot,
                RateLimitPostureClass::NotRateLimited,
                EventDispositionClass::PublishDrainedAfterReview,
                vec![],
                vec!["provider.object.issue.85"],
                "Latest clean issue snapshot admitted provider mutation.",
            ),
        ],
        replay_ledger_items: vec![
            replay_ledger_item(
                "provider_replay.ledger.pr_comment",
                identity_pr.clone(),
                0,
                EventDispositionClass::AppliedOnce,
                RetryabilityClass::NoRetryNeeded,
                vec!["provider_event.event.pr_comment.applied"],
                Some("provider_import.session.pr_comment.full"),
                vec!["audit.provider_event.pr_comment.applied"],
                "First review-comment delivery applied once.",
            ),
            replay_ledger_item(
                "provider_replay.ledger.pr_comment_redelivery",
                identity_pr,
                1,
                EventDispositionClass::DedupedNoop,
                RetryabilityClass::NoRetryNeeded,
                vec!["provider_event.event.pr_comment.duplicate"],
                Some("provider_import.session.pr_comment.full"),
                vec!["audit.provider_event.pr_comment.deduped"],
                "Redelivery was deduped without creating a duplicate comment.",
            ),
            replay_ledger_item(
                "provider_replay.ledger.issue.partial",
                identity_issue,
                0,
                EventDispositionClass::PartialImportApplied,
                RetryabilityClass::RetryableAfterBackfill,
                vec!["provider_event.event.issue.partial"],
                Some("provider_import.session.issue.partial"),
                vec!["audit.provider_event.issue.partial"],
                "Partial import is retryable after missing page backfill.",
            ),
            replay_ledger_item(
                "provider_replay.ledger.check.mirror",
                identity_mirror,
                0,
                EventDispositionClass::AppliedOnce,
                RetryabilityClass::RetryableAfterRefresh,
                vec!["provider_event.event.check.mirror"],
                Some("provider_import.session.check.mirror"),
                vec!["audit.provider_event.check.mirror"],
                "Mirror delivery refreshed imported check state.",
            ),
            replay_ledger_item(
                "provider_replay.ledger.callback.denied",
                identity_deny,
                0,
                EventDispositionClass::DeniedNoMutation,
                RetryabilityClass::NotRetryableTargetMismatch,
                vec!["provider_event.event.callback.denied"],
                None,
                vec!["audit.provider_event.callback.denied"],
                "Callback deny stayed audit-only with no local provider mutation.",
            ),
            replay_ledger_item(
                "provider_replay.ledger.publish.blocked",
                identity_publish.clone(),
                0,
                EventDispositionClass::PublishBlockedDrift,
                RetryabilityClass::RetryableAfterRefresh,
                vec!["provider_event.event.publish_later.blocked"],
                Some("provider_import.session.issue.latest"),
                vec!["audit.provider_event.publish.blocked"],
                "Publish-later drain held after target drift.",
            ),
            replay_ledger_item(
                "provider_replay.ledger.publish.drained",
                delivery_identity("provider.delivery.5002", "provider.object.issue.85"),
                0,
                EventDispositionClass::PublishDrainedAfterReview,
                RetryabilityClass::NoRetryNeeded,
                vec!["provider_event.event.publish_later.drained"],
                Some("provider_import.session.issue.clean_latest"),
                vec!["audit.provider_event.publish.drained"],
                "Publish-later drain committed after clean revalidation.",
            ),
        ],
        reconciliation_results: vec![
            reconciliation_result(
                "provider_reconcile.result.issue.84.drift",
                "local_draft.issue.84.transition",
                "deferred_publish.queue.issue.84.transition",
                identity_publish,
                "provider_import.session.issue.baseline",
                "provider_import.session.issue.latest",
                ProviderDriftClass::TargetContentDrifted,
                EventDispositionClass::PublishBlockedDrift,
                ReconciliationNextActionClass::CompareRebaseReview,
                false,
                vec!["provider.conflict.issue.84.status"],
                "Issue changed remotely before queued transition drained; compare review is required.",
            ),
            reconciliation_result(
                "provider_reconcile.result.issue.85.clean",
                "local_draft.issue.85.comment",
                "deferred_publish.queue.issue.85.comment",
                delivery_identity("provider.delivery.5002", "provider.object.issue.85"),
                "provider_import.session.issue.clean_baseline",
                "provider_import.session.issue.clean_latest",
                ProviderDriftClass::NoMaterialDrift,
                EventDispositionClass::PublishDrainedAfterReview,
                ReconciliationNextActionClass::MutateProviderNow,
                true,
                vec![],
                "Queued comment matched the latest provider snapshot and drained through reviewed authority.",
            ),
        ],
        callback_deny_events: vec![ProviderCallbackDenyEvent {
            record_kind: PROVIDER_CALLBACK_DENY_EVENT_RECORD_KIND.to_string(),
            schema_version: PROVIDER_EVENT_RECONCILIATION_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_EVENT_RECONCILIATION_SHARED_CONTRACT_REF.to_string(),
            deny_event_id: "provider_callback_deny.event.host_mismatch".to_string(),
            denied_event_ref: "provider_event.event.callback.denied".to_string(),
            delivery_identity: delivery_identity("provider.delivery.4001", "provider.object.pr.42"),
            reason: CallbackDenyReasonClass::HostMismatch,
            policy_source_ref: "policy.provider_callback.host_binding".to_string(),
            route_class: CallbackRouteClass::DeniedRoute,
            actor_or_install_ref: "provider.actor.installation.primary".to_string(),
            remediation_hint: "Reconnect the provider account or open the object in the provider after host repair.".to_string(),
            occurred_at: "2026-05-18T09:11:00Z".to_string(),
            no_user_visible_mutation: true,
            audit_event_refs: vec!["audit.provider_callback.host_mismatch".to_string()],
            support_export_summary: "Host mismatch callback deny is exportable and did not mutate provider state.".to_string(),
            redaction_class: RedactionClass::MetadataSafeDefault,
        }],
        support_export_summary: "Provider event reconciliation page covers duplicate delivery, partial imports, mirror truth, denied callbacks, and publish-later drift review.".to_string(),
    }
}

fn event_envelope(
    event_id: &str,
    source_class: ProviderEventSourceClass,
    event_type: ProviderEventTypeClass,
    delivery_identity: ProviderDeliveryIdentity,
    source_proof: SourceProofClass,
    truth_class: TruthCompletenessClass,
    final_disposition: EventDispositionClass,
    retryability: RetryabilityClass,
    import_session_ref: Option<&str>,
    replay_ledger_item_ref: &str,
    deny_event_ref: Option<&str>,
    omissions: Vec<ProviderOmission>,
    support_export_summary: &str,
) -> ProviderEventEnvelope {
    ProviderEventEnvelope {
        record_kind: PROVIDER_EVENT_ENVELOPE_RECORD_KIND.to_string(),
        schema_version: PROVIDER_EVENT_RECONCILIATION_SCHEMA_VERSION,
        shared_contract_ref: PROVIDER_EVENT_RECONCILIATION_SHARED_CONTRACT_REF.to_string(),
        event_id: event_id.to_string(),
        provider_descriptor_ref: "provider.descriptor.code_host.primary".to_string(),
        provider_family: ProviderFamily::CodeHost,
        source_class,
        event_type,
        delivery_identity: delivery_identity.clone(),
        source_proof,
        object_refs: vec![ProviderScopedObjectRef {
            object_ref: delivery_identity.scoped_object_ref.clone(),
            object_kind: ProviderObjectKind::IssueOrWorkItem,
            provider_remote_ref: delivery_identity.scoped_object_ref.clone(),
            local_surrogate_ref: Some(format!(
                "local.surrogate.{}",
                delivery_identity.scoped_object_ref
            )),
            target_ref: target_ref(delivery_identity.scoped_object_ref.as_str()),
            truth_class,
        }],
        event_time: "2026-05-18T09:10:00Z".to_string(),
        ingested_at: "2026-05-18T09:10:02Z".to_string(),
        import_session_ref: import_session_ref.map(ToOwned::to_owned),
        replay_ledger_item_ref: replay_ledger_item_ref.to_string(),
        freshness: freshness(truth_class),
        truth_class,
        omissions,
        retryability,
        final_disposition,
        deny_event_ref: deny_event_ref.map(ToOwned::to_owned),
        policy_epoch_ref: "policy.epoch.provider.2026-05-18".to_string(),
        support_export_summary: support_export_summary.to_string(),
        raw_payload_refs_present: false,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn import_session(
    import_session_id: &str,
    source_event_refs: Vec<&str>,
    object_scope_ref: &str,
    truth_class: TruthCompletenessClass,
    rate_limit_posture: RateLimitPostureClass,
    final_disposition: EventDispositionClass,
    omissions: Vec<ProviderOmission>,
    imported_object_refs: Vec<&str>,
    support_export_summary: &str,
) -> ImportSession {
    ImportSession {
        record_kind: IMPORT_SESSION_RECORD_KIND.to_string(),
        schema_version: PROVIDER_EVENT_RECONCILIATION_SCHEMA_VERSION,
        shared_contract_ref: PROVIDER_EVENT_RECONCILIATION_SHARED_CONTRACT_REF.to_string(),
        import_session_id: import_session_id.to_string(),
        provider_descriptor_ref: "provider.descriptor.code_host.primary".to_string(),
        source_event_refs: source_event_refs
            .into_iter()
            .map(ToOwned::to_owned)
            .collect(),
        object_scope_ref: object_scope_ref.to_string(),
        snapshot_time: "2026-05-18T09:10:03Z".to_string(),
        freshness: freshness(truth_class),
        truth_class,
        omissions,
        rate_limit_posture,
        imported_object_refs: imported_object_refs
            .into_iter()
            .map(ToOwned::to_owned)
            .collect(),
        retryability: RetryabilityClass::NoRetryNeeded,
        final_disposition,
        support_export_summary: support_export_summary.to_string(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn replay_ledger_item(
    replay_ledger_item_id: &str,
    delivery_identity: ProviderDeliveryIdentity,
    replay_count: u32,
    final_disposition: EventDispositionClass,
    retryability: RetryabilityClass,
    event_refs: Vec<&str>,
    import_session_ref: Option<&str>,
    audit_event_refs: Vec<&str>,
    support_export_summary: &str,
) -> ReplayLedgerItem {
    ReplayLedgerItem {
        record_kind: REPLAY_LEDGER_ITEM_RECORD_KIND.to_string(),
        schema_version: PROVIDER_EVENT_RECONCILIATION_SCHEMA_VERSION,
        shared_contract_ref: PROVIDER_EVENT_RECONCILIATION_SHARED_CONTRACT_REF.to_string(),
        replay_ledger_item_id: replay_ledger_item_id.to_string(),
        delivery_identity,
        first_seen_at: "2026-05-18T09:10:02Z".to_string(),
        last_seen_at: "2026-05-18T09:10:12Z".to_string(),
        dedupe_window: "PT24H".to_string(),
        replay_count,
        final_disposition,
        retryability,
        event_refs: event_refs.into_iter().map(ToOwned::to_owned).collect(),
        import_session_ref: import_session_ref.map(ToOwned::to_owned),
        audit_event_refs: audit_event_refs
            .into_iter()
            .map(ToOwned::to_owned)
            .collect(),
        support_export_summary: support_export_summary.to_string(),
        raw_payload_refs_present: false,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn reconciliation_result(
    reconciliation_result_id: &str,
    local_draft_ref: &str,
    deferred_publish_queue_item_ref: &str,
    expected_identity: ProviderDeliveryIdentity,
    baseline_import_session_ref: &str,
    latest_import_session_ref: &str,
    drift_class: ProviderDriftClass,
    final_disposition: EventDispositionClass,
    next_action: ReconciliationNextActionClass,
    safe_to_mutate_provider: bool,
    conflict_refs: Vec<&str>,
    support_export_summary: &str,
) -> ReconciliationResult {
    ReconciliationResult {
        record_kind: RECONCILIATION_RESULT_RECORD_KIND.to_string(),
        schema_version: PROVIDER_EVENT_RECONCILIATION_SCHEMA_VERSION,
        shared_contract_ref: PROVIDER_EVENT_RECONCILIATION_SHARED_CONTRACT_REF.to_string(),
        reconciliation_result_id: reconciliation_result_id.to_string(),
        subject: ReconciliationSubject {
            local_draft_ref: local_draft_ref.to_string(),
            deferred_publish_queue_item_ref: deferred_publish_queue_item_ref.to_string(),
            target_ref: target_ref(expected_identity.scoped_object_ref.as_str()),
            expected_identity,
        },
        baseline_import_session_ref: baseline_import_session_ref.to_string(),
        latest_import_session_ref: latest_import_session_ref.to_string(),
        latest_provider_snapshot_ref: format!("snapshot.{latest_import_session_ref}"),
        matched_object_refs: vec!["provider.object.issue.matched".to_string()],
        created_count: if safe_to_mutate_provider { 1 } else { 0 },
        updated_count: 0,
        skipped_count: if safe_to_mutate_provider { 0 } else { 1 },
        conflict_refs: conflict_refs.into_iter().map(ToOwned::to_owned).collect(),
        drift_class,
        retryability: if safe_to_mutate_provider {
            RetryabilityClass::NoRetryNeeded
        } else {
            RetryabilityClass::RetryableAfterRefresh
        },
        final_disposition,
        next_action,
        safe_to_mutate_provider,
        policy_epoch_ref: "policy.epoch.provider.2026-05-18".to_string(),
        auth_scope_ref: "provider.scope.write.issue".to_string(),
        support_export_summary: support_export_summary.to_string(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn delivery_identity(
    external_delivery_id: &str,
    scoped_object_ref: &str,
) -> ProviderDeliveryIdentity {
    ProviderDeliveryIdentity {
        external_delivery_id: external_delivery_id.to_string(),
        scoped_object_ref: scoped_object_ref.to_string(),
        provider_host_ref: "provider.host.code.example".to_string(),
        tenant_or_org_scope_ref: "provider.tenant.example-org".to_string(),
    }
}

fn target_ref(target_ref_id: &str) -> TargetRef {
    TargetRef {
        target_ref_class: "provider_object".to_string(),
        target_ref: target_ref_id.to_string(),
        target_label: target_ref_id.to_string(),
        route_origin: None,
    }
}

fn omission(
    omission_id: &str,
    omission_class: &str,
    scope_ref: &str,
    reason_summary: &str,
) -> ProviderOmission {
    ProviderOmission {
        omission_id: omission_id.to_string(),
        omission_class: omission_class.to_string(),
        scope_ref: scope_ref.to_string(),
        reason_summary: reason_summary.to_string(),
    }
}

fn freshness(truth_class: TruthCompletenessClass) -> FreshnessTruth {
    let (freshness_class, degraded_reason) = if truth_class.is_non_canonical() {
        (
            crate::registry::FreshnessLabel::StaleWithinWindow,
            Some(format!(
                "Provider truth is {truth_class:?} and must stay labeled."
            )),
        )
    } else {
        (crate::registry::FreshnessLabel::Fresh, None)
    };
    FreshnessTruth {
        freshness_class,
        observed_at: Some("2026-05-18T09:10:03Z".to_string()),
        freshness_floor_ref: "freshness.provider.default".to_string(),
        stale_after: Some("PT30M".to_string()),
        degraded_reason,
        import_session_ref: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates() {
        let page = seeded_provider_event_reconciliation_page();
        let report = page.validate();
        assert!(report.passed, "seeded page failed: {:#?}", report.findings);
    }

    #[test]
    fn duplicate_delivery_cannot_mutate_twice() {
        let mut page = seeded_provider_event_reconciliation_page();
        let duplicate = page
            .event_envelopes
            .iter_mut()
            .find(|event| event.event_id == "provider_event.event.pr_comment.duplicate")
            .expect("duplicate event present");
        duplicate.final_disposition = EventDispositionClass::AppliedOnce;

        let report = page.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| {
            finding.check_id == "provider_event_reconciliation.delivery_duplicate_mutation"
        }));
    }

    #[test]
    fn material_drift_blocks_provider_mutation() {
        let mut page = seeded_provider_event_reconciliation_page();
        let drifted = page
            .reconciliation_results
            .iter_mut()
            .find(|result| result.drift_class == ProviderDriftClass::TargetContentDrifted)
            .expect("drifted result present");
        drifted.safe_to_mutate_provider = true;

        let report = page.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| {
            finding.check_id == "provider_event_reconciliation.reconciliation_drift_safe_to_mutate"
        }));
    }
}
