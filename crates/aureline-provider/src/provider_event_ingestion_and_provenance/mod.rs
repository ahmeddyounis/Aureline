//! Stable imported-provider event ingestion and provenance records.
//!
//! This module is the stable provider-event envelope used after account/grant
//! resolution, browser-handoff return, webhook ingress, mirror ingress, and
//! publish-later reconciliation have converged into one auditable record
//! family. Each imported event names the provider descriptor, acting authority,
//! canonical object identity, freshness, replay/dedupe keys, browser-handoff
//! origin when present, policy verdict, and resulting local object refs.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::account_scope::ActingIdentityClass;
use crate::browser_handoff::HandoffOriginClass;
use crate::registry::{FreshnessLabel, FreshnessTruth, ProviderFamily, ProviderObjectKind};

/// Schema version for imported-provider event provenance packets.
pub const PROVIDER_EVENT_INGESTION_PROVENANCE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref for imported-provider event provenance packets.
pub const PROVIDER_EVENT_INGESTION_PROVENANCE_SHARED_CONTRACT_REF: &str =
    "providers:provider_event_ingestion_and_provenance:v1";

/// Stable record kind for [`ProviderEventIngestionProvenancePacket`].
pub const PROVIDER_EVENT_INGESTION_PROVENANCE_PACKET_RECORD_KIND: &str =
    "provider_event_ingestion_provenance_packet";

/// Stable record kind for [`ImportedProviderEventEnvelope`].
pub const IMPORTED_PROVIDER_EVENT_ENVELOPE_RECORD_KIND: &str = "imported_provider_event_envelope";

/// Stable record kind for [`ProviderEventSurfaceProjection`].
pub const PROVIDER_EVENT_SURFACE_PROJECTION_RECORD_KIND: &str = "provider_event_surface_projection";

/// Stable record kind for [`ProviderEventSupportExportPacket`].
pub const PROVIDER_EVENT_SUPPORT_EXPORT_PACKET_RECORD_KIND: &str =
    "provider_event_support_export_packet";

/// Stable record kind for [`ProviderEventIngestionValidationReport`].
pub const PROVIDER_EVENT_INGESTION_VALIDATION_REPORT_RECORD_KIND: &str =
    "provider_event_ingestion_validation_report";

/// Stable record kind for [`ProviderEventIngestionValidationFinding`].
pub const PROVIDER_EVENT_INGESTION_VALIDATION_FINDING_RECORD_KIND: &str =
    "provider_event_ingestion_validation_finding";

/// Authority source used to evaluate one imported provider event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderEventAuthoritySourceClass {
    /// Event was evaluated against a connected human account.
    ConnectedAccount,
    /// Event was evaluated against an installation, app, or project grant.
    InstallationGrant,
    /// Event was evaluated against a delegated credential.
    DelegatedCredential,
    /// Event was evaluated against a policy-injected service identity.
    PolicyInjectedService,
}

impl ProviderEventAuthoritySourceClass {
    /// Returns the matching account-scope acting identity when one exists.
    pub const fn acting_identity(self) -> Option<ActingIdentityClass> {
        match self {
            Self::ConnectedAccount => Some(ActingIdentityClass::ConnectedAccount),
            Self::InstallationGrant => Some(ActingIdentityClass::InstallationGrant),
            Self::DelegatedCredential => Some(ActingIdentityClass::DelegatedCredential),
            Self::PolicyInjectedService => None,
        }
    }
}

/// Ingress class for an imported provider event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderEventIngressClass {
    /// Provider webhook delivery.
    Webhook,
    /// Browser-return callback from a typed handoff packet.
    BrowserReturnCallback,
    /// Polling or refresh import.
    PollingRefresh,
    /// Customer-operated mirror or self-hosted ingress.
    MirrorIngress,
    /// Provider import-session backfill.
    ProviderImportSession,
    /// Publish-later drain reconciliation.
    PublishLaterDrain,
}

/// Event class normalized before local state may change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportedProviderEventClass {
    /// Provider-owned object content changed.
    ObjectMutation,
    /// Provider-owned comment or note changed.
    CommentMutation,
    /// Hosted review state changed.
    ReviewMutation,
    /// Issue or work-item state changed.
    WorkItemMutation,
    /// CI/check state changed.
    CheckStateMutation,
    /// Provider scope or grant changed.
    ScopeMutation,
    /// Callback was denied before local mutation.
    CallbackDenied,
}

/// User-visible state label for imported provider events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportedEventSurfaceState {
    /// Event imported provider-owned state.
    Imported,
    /// Event is buffered pending sequence, backfill, or reconciliation.
    Buffered,
    /// Event was replayed or redelivered without duplicate mutation.
    Replayed,
    /// Event was denied and audit-only.
    Denied,
    /// Event was stale and could not update trusted local truth.
    Stale,
    /// Event must enter conflict review before local truth can change.
    ConflictReviewRequired,
}

impl ImportedEventSurfaceState {
    /// Returns true when the label means local provider-linked state can change.
    pub const fn may_import_state(self) -> bool {
        matches!(self, Self::Imported)
    }
}

/// Policy verdict attached before any imported event mutates state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderEventPolicyVerdictClass {
    /// Policy accepted the event without narrowing.
    Accepted,
    /// Policy accepted the event with narrower scope or state.
    AcceptedNarrowed,
    /// Policy denied the event.
    Denied,
    /// Policy admitted audit-only capture.
    AuditOnly,
    /// Policy requires conflict review before mutation.
    ConflictReviewRequired,
}

impl ProviderEventPolicyVerdictClass {
    /// Returns true when policy allows imported state to materialize.
    pub const fn admits_import(self) -> bool {
        matches!(self, Self::Accepted | Self::AcceptedNarrowed)
    }
}

/// Replay decision for one delivery key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderEventReplayDecisionClass {
    /// First delivery for the dedupe key.
    FirstDelivery,
    /// Duplicate delivery was deduped with no second mutation.
    DuplicateDedupeNoop,
    /// Replay only refreshed freshness metadata.
    ReplayFreshnessOnly,
    /// Delivery is buffered pending sequence or backfill.
    BufferedPendingSequence,
    /// Replay requires human review before state can change.
    ReplayRequiresReview,
}

impl ProviderEventReplayDecisionClass {
    /// Returns true when this replay decision is duplicate-safe.
    pub const fn is_duplicate_safe(self) -> bool {
        matches!(
            self,
            Self::DuplicateDedupeNoop | Self::ReplayFreshnessOnly | Self::ReplayRequiresReview
        )
    }
}

/// Local-provider overlap detected during event reconciliation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderEventOverlapClass {
    /// No local draft, queued publish, or open handoff overlaps the event.
    NoOverlap,
    /// Event overlaps an unreconciled local draft.
    LocalDraft,
    /// Event overlaps a queued publish-later item.
    PublishLaterQueue,
    /// Event overlaps an unreconciled browser-handoff session.
    BrowserHandoffSession,
    /// Event overlaps more than one local continuity lane.
    MultipleLocalContinuityLanes,
}

impl ProviderEventOverlapClass {
    /// Returns true when explicit conflict review is required.
    pub const fn requires_conflict_review(self) -> bool {
        !matches!(self, Self::NoOverlap)
    }
}

/// Resulting state for a local object after event ingestion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderEventResultingStateClass {
    /// Provider-owned state was imported into local projections.
    ImportedProviderState,
    /// Local draft remained authoritative pending review.
    LocalDraftPreserved,
    /// Publish-later item stayed queued pending review.
    PublishLaterPreserved,
    /// Provider commit was observed after reviewed mutation.
    ProviderCommittedObserved,
    /// Event buffered without local mutation.
    BufferedNoMutation,
    /// Event denied without local mutation.
    DeniedNoMutation,
    /// Event is stale and produced no trusted mutation.
    StaleNoMutation,
    /// Event opened or updated a conflict-review packet.
    ConflictReviewQueued,
}

impl ProviderEventResultingStateClass {
    /// Returns true when this outcome may mutate user-visible imported state.
    pub const fn mutates_user_visible_state(self) -> bool {
        matches!(
            self,
            Self::ImportedProviderState | Self::ProviderCommittedObserved
        )
    }
}

/// Canonical provider object identity attached to one imported event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalProviderObjectRef {
    /// Provider object kind.
    pub object_kind: ProviderObjectKind,
    /// Canonical provider-side object id.
    pub canonical_provider_object_id: String,
    /// Canonical local object row ref.
    pub canonical_local_object_ref: String,
    /// Provider target ref such as repository, project, board, or tenant.
    pub target_ref: String,
}

/// Dedupe and replay identity for one imported event.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ProviderEventDedupeEnvelope {
    /// External provider delivery id.
    pub external_delivery_id: String,
    /// Scoped provider object ref.
    pub scoped_object_ref: String,
    /// Provider host ref.
    pub provider_host_ref: String,
    /// Tenant, org, repository, board, or project scope ref.
    pub tenant_or_org_scope_ref: String,
    /// Dedupe key derived from delivery and object identity.
    pub dedupe_key: String,
    /// Replay key used by callback and mirror ledgers.
    pub replay_key: String,
    /// Replay decision for this delivery.
    pub replay_decision: ProviderEventReplayDecisionClass,
    /// Number of redeliveries beyond the first delivery.
    pub replay_count: u32,
}

impl ProviderEventDedupeEnvelope {
    /// Returns true when every identity component is present.
    pub fn is_complete(&self) -> bool {
        !self.external_delivery_id.trim().is_empty()
            && !self.scoped_object_ref.trim().is_empty()
            && !self.provider_host_ref.trim().is_empty()
            && !self.tenant_or_org_scope_ref.trim().is_empty()
            && !self.dedupe_key.trim().is_empty()
            && !self.replay_key.trim().is_empty()
    }
}

/// Browser-handoff origin attached to callback events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserHandoffOriginRef {
    /// Browser-handoff packet ref.
    pub handoff_packet_ref: String,
    /// Origin lane that minted the handoff.
    pub origin_class: HandoffOriginClass,
    /// Return anchor restored or reviewed after callback.
    pub return_anchor_ref: String,
    /// True when the browser-handoff session had not reconciled before the event.
    pub unreconciled_session: bool,
}

/// Policy verdict record for one imported event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventPolicyVerdict {
    /// Verdict class.
    pub verdict_class: ProviderEventPolicyVerdictClass,
    /// Policy epoch used to evaluate the event.
    pub policy_epoch_ref: String,
    /// Effective scope resolution ref.
    pub effective_scope_ref: String,
    /// Audit event refs emitted by policy evaluation.
    pub audit_event_refs: Vec<String>,
    /// Deny, narrow, or review reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Resulting local object ref touched by one imported event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventLocalObjectOutcome {
    /// Local object ref.
    pub local_object_ref: String,
    /// Resulting state after ingestion.
    pub resulting_state: ProviderEventResultingStateClass,
    /// Review or export-safe summary for this object.
    pub summary_label: String,
}

/// Stable imported-provider event envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedProviderEventEnvelope {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable imported-event id.
    pub event_id: String,
    /// Provider descriptor ref.
    pub provider_descriptor_ref: String,
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Authority source class used for evaluation.
    pub authority_source_class: ProviderEventAuthoritySourceClass,
    /// Account, grant, delegated credential, or service identity ref.
    pub authority_source_ref: String,
    /// Ingress class.
    pub ingress_class: ProviderEventIngressClass,
    /// Imported event class.
    pub event_class: ImportedProviderEventClass,
    /// Canonical provider object refs touched by this event.
    pub canonical_object_refs: Vec<CanonicalProviderObjectRef>,
    /// Freshness truth for the imported provider observation.
    pub freshness: FreshnessTruth,
    /// Replay and dedupe envelope.
    pub dedupe: ProviderEventDedupeEnvelope,
    /// Browser-handoff origin when the event came through a handoff callback.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_origin: Option<BrowserHandoffOriginRef>,
    /// Policy verdict applied before mutation.
    pub policy_verdict: ProviderEventPolicyVerdict,
    /// Local continuity lane overlap detected during ingestion.
    pub overlap_class: ProviderEventOverlapClass,
    /// Conflict review packet ref when conflict review is required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conflict_review_ref: Option<String>,
    /// Local object outcomes created or preserved by this event.
    pub resulting_local_objects: Vec<ProviderEventLocalObjectOutcome>,
    /// User-visible imported-event label.
    pub surface_state: ImportedEventSurfaceState,
    /// Redaction-safe support summary.
    pub support_summary: String,
    /// True when the envelope contains raw provider payload refs.
    pub raw_provider_payload_refs_present: bool,
    /// True when the envelope contains raw callback URL refs.
    pub raw_callback_url_refs_present: bool,
}

/// Surface projection consumed by work-item, review, activity, and support lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventSurfaceProjection {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable projection row id.
    pub row_id: String,
    /// Imported event ref.
    pub event_ref: String,
    /// Surface receiving the projection.
    pub surface: String,
    /// Required visible label on that surface.
    pub surface_state: ImportedEventSurfaceState,
    /// Canonical object ref shown by the surface.
    pub canonical_local_object_ref: String,
    /// Freshness label shown by the surface.
    pub freshness_class: FreshnessLabel,
    /// True when the row routes to conflict review.
    pub conflict_review_required: bool,
    /// Redaction-safe row summary.
    pub summary_label: String,
}

/// Redaction-safe support/export packet for imported provider events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventSupportExportPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support export id.
    pub support_export_id: String,
    /// Event summaries.
    pub event_summaries: Vec<ProviderEventSupportSummary>,
    /// True when raw provider payload export is allowed.
    pub raw_provider_payload_export_allowed: bool,
    /// True when raw callback URL export is allowed.
    pub raw_callback_url_export_allowed: bool,
    /// Reviewable support summary.
    pub summary_label: String,
}

/// Redaction-safe support summary for one imported provider event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventSupportSummary {
    /// Imported event id.
    pub event_id: String,
    /// Provider descriptor ref.
    pub provider_descriptor_ref: String,
    /// Authority source class.
    pub authority_source_class: ProviderEventAuthoritySourceClass,
    /// External delivery id.
    pub external_delivery_id: String,
    /// Dedupe key.
    pub dedupe_key: String,
    /// Surface state.
    pub surface_state: ImportedEventSurfaceState,
    /// Policy verdict.
    pub policy_verdict: ProviderEventPolicyVerdictClass,
    /// Resulting local object refs.
    pub resulting_local_object_refs: Vec<String>,
    /// Support summary.
    pub support_summary: String,
}

impl From<&ImportedProviderEventEnvelope> for ProviderEventSupportSummary {
    fn from(event: &ImportedProviderEventEnvelope) -> Self {
        Self {
            event_id: event.event_id.clone(),
            provider_descriptor_ref: event.provider_descriptor_ref.clone(),
            authority_source_class: event.authority_source_class,
            external_delivery_id: event.dedupe.external_delivery_id.clone(),
            dedupe_key: event.dedupe.dedupe_key.clone(),
            surface_state: event.surface_state,
            policy_verdict: event.policy_verdict.verdict_class,
            resulting_local_object_refs: event
                .resulting_local_objects
                .iter()
                .map(|outcome| outcome.local_object_ref.clone())
                .collect(),
            support_summary: event.support_summary.clone(),
        }
    }
}

/// Stable imported-provider event provenance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventIngestionProvenancePacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Fixture or export generation timestamp.
    pub generated_at: String,
    /// Imported provider event envelopes.
    pub event_envelopes: Vec<ImportedProviderEventEnvelope>,
    /// Surface projections that must preserve imported-event labels.
    pub surface_projections: Vec<ProviderEventSurfaceProjection>,
    /// Redaction-safe support/export packet.
    pub support_export: ProviderEventSupportExportPacket,
}

impl ProviderEventIngestionProvenancePacket {
    /// Validates this packet against stable imported-event invariants.
    pub fn validate(&self) -> ProviderEventIngestionValidationReport {
        let mut validator = Validator::new(self);
        validator.run();
        validator.finish()
    }

    /// Returns true when no raw provider payload or callback URL refs are exported.
    pub fn raw_escape_hatches_absent(&self) -> bool {
        !self.support_export.raw_provider_payload_export_allowed
            && !self.support_export.raw_callback_url_export_allowed
            && self.event_envelopes.iter().all(|event| {
                !event.raw_provider_payload_refs_present && !event.raw_callback_url_refs_present
            })
    }
}

/// Validation report emitted by imported-event provenance validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventIngestionValidationReport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Packet id.
    pub packet_id: String,
    /// Whether no error findings were emitted.
    pub passed: bool,
    /// Coverage observed during validation.
    pub coverage: ProviderEventIngestionCoverage,
    /// Findings emitted by validation.
    pub findings: Vec<ProviderEventIngestionValidationFinding>,
}

/// Coverage observed by imported-event provenance validation.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ProviderEventIngestionCoverage {
    /// Authority source classes covered by events.
    pub authority_sources: BTreeSet<ProviderEventAuthoritySourceClass>,
    /// Ingress classes covered by events.
    pub ingress_classes: BTreeSet<ProviderEventIngressClass>,
    /// Surface states covered by events and projections.
    pub surface_states: BTreeSet<ImportedEventSurfaceState>,
    /// Policy verdicts covered by events.
    pub policy_verdicts: BTreeSet<ProviderEventPolicyVerdictClass>,
    /// Overlap classes covered by events.
    pub overlap_classes: BTreeSet<ProviderEventOverlapClass>,
}

/// One imported-event provenance validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventIngestionValidationFinding {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable check id.
    pub check_id: String,
    /// Redaction-safe message.
    pub message: String,
}

struct Validator<'a> {
    packet: &'a ProviderEventIngestionProvenancePacket,
    event_ids: BTreeSet<&'a str>,
    delivery_groups: BTreeMap<&'a str, Vec<&'a ImportedProviderEventEnvelope>>,
    coverage: ProviderEventIngestionCoverage,
    findings: Vec<ProviderEventIngestionValidationFinding>,
}

impl<'a> Validator<'a> {
    fn new(packet: &'a ProviderEventIngestionProvenancePacket) -> Self {
        Self {
            packet,
            event_ids: BTreeSet::new(),
            delivery_groups: BTreeMap::new(),
            coverage: ProviderEventIngestionCoverage::default(),
            findings: Vec::new(),
        }
    }

    fn run(&mut self) {
        self.validate_header();
        self.collect_events();
        self.validate_events();
        self.validate_surface_projections();
        self.validate_support_export();
        self.validate_delivery_idempotency();
        self.validate_required_coverage();
    }

    fn finish(self) -> ProviderEventIngestionValidationReport {
        ProviderEventIngestionValidationReport {
            record_kind: PROVIDER_EVENT_INGESTION_VALIDATION_REPORT_RECORD_KIND.to_string(),
            schema_version: PROVIDER_EVENT_INGESTION_PROVENANCE_SCHEMA_VERSION,
            packet_id: self.packet.packet_id.clone(),
            passed: self.findings.is_empty(),
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn validate_header(&mut self) {
        self.expect(
            self.packet.record_kind == PROVIDER_EVENT_INGESTION_PROVENANCE_PACKET_RECORD_KIND,
            "provider_event_ingestion.packet_record_kind",
            "packet record_kind must match the stable discriminator",
        );
        self.expect(
            self.packet.schema_version == PROVIDER_EVENT_INGESTION_PROVENANCE_SCHEMA_VERSION,
            "provider_event_ingestion.packet_schema_version",
            "packet schema_version must match the crate constant",
        );
        self.expect(
            self.packet.shared_contract_ref
                == PROVIDER_EVENT_INGESTION_PROVENANCE_SHARED_CONTRACT_REF,
            "provider_event_ingestion.packet_shared_contract_ref",
            "packet shared_contract_ref must match the stable contract ref",
        );
        self.expect_nonempty(
            &self.packet.packet_id,
            "provider_event_ingestion.packet_id_missing",
        );
        self.expect_nonempty(
            &self.packet.generated_at,
            "provider_event_ingestion.generated_at_missing",
        );
        self.expect(
            !self.packet.event_envelopes.is_empty(),
            "provider_event_ingestion.events_missing",
            "packet must include imported provider event envelopes",
        );
    }

    fn collect_events(&mut self) {
        for event in &self.packet.event_envelopes {
            let inserted = self.event_ids.insert(event.event_id.as_str());
            self.expect(
                inserted,
                "provider_event_ingestion.event_id_duplicate",
                "event ids must be unique",
            );
            self.delivery_groups
                .entry(event.dedupe.dedupe_key.as_str())
                .or_default()
                .push(event);
        }
    }

    fn validate_events(&mut self) {
        for event in &self.packet.event_envelopes {
            self.coverage
                .authority_sources
                .insert(event.authority_source_class);
            self.coverage.ingress_classes.insert(event.ingress_class);
            self.coverage.surface_states.insert(event.surface_state);
            self.coverage
                .policy_verdicts
                .insert(event.policy_verdict.verdict_class);
            self.coverage.overlap_classes.insert(event.overlap_class);

            self.validate_record(
                &event.record_kind,
                event.schema_version,
                &event.shared_contract_ref,
                IMPORTED_PROVIDER_EVENT_ENVELOPE_RECORD_KIND,
                "provider_event_ingestion.event",
            );
            self.expect_nonempty(&event.event_id, "provider_event_ingestion.event_id_missing");
            self.expect_nonempty(
                &event.provider_descriptor_ref,
                "provider_event_ingestion.provider_descriptor_missing",
            );
            self.expect_nonempty(
                &event.authority_source_ref,
                "provider_event_ingestion.authority_source_ref_missing",
            );
            self.expect(
                !event.canonical_object_refs.is_empty(),
                "provider_event_ingestion.canonical_object_refs_missing",
                "event must name canonical provider and local object identity",
            );
            self.expect(
                event.dedupe.is_complete(),
                "provider_event_ingestion.dedupe_incomplete",
                "event must include delivery id, scoped object, provider host, tenant/org scope, dedupe key, and replay key",
            );
            self.expect_nonempty(
                &event.policy_verdict.policy_epoch_ref,
                "provider_event_ingestion.policy_epoch_missing",
            );
            self.expect_nonempty(
                &event.policy_verdict.effective_scope_ref,
                "provider_event_ingestion.effective_scope_missing",
            );
            self.expect(
                !event.resulting_local_objects.is_empty(),
                "provider_event_ingestion.local_outcomes_missing",
                "event must name resulting local object refs",
            );
            self.expect_nonempty(
                &event.support_summary,
                "provider_event_ingestion.support_summary_missing",
            );
            self.expect(
                !event.raw_provider_payload_refs_present && !event.raw_callback_url_refs_present,
                "provider_event_ingestion.raw_refs_present",
                "imported-event envelopes must not carry raw provider payloads or callback URLs",
            );

            if event.ingress_class == ProviderEventIngressClass::BrowserReturnCallback {
                self.expect(
                    event.browser_handoff_origin.is_some(),
                    "provider_event_ingestion.browser_origin_missing",
                    "browser-return callbacks must cite a typed browser-handoff origin",
                );
            }

            if event.policy_verdict.verdict_class == ProviderEventPolicyVerdictClass::Denied {
                self.expect(
                    event.surface_state == ImportedEventSurfaceState::Denied,
                    "provider_event_ingestion.denied_label_wrong",
                    "denied events must use the Denied imported-event label",
                );
                self.expect(
                    !event.policy_verdict.audit_event_refs.is_empty(),
                    "provider_event_ingestion.denied_audit_missing",
                    "denied events must emit audit refs",
                );
            }

            if event.overlap_class.requires_conflict_review() {
                self.expect(
                    event.surface_state == ImportedEventSurfaceState::ConflictReviewRequired,
                    "provider_event_ingestion.overlap_label_wrong",
                    "events overlapping local drafts, publish-later items, or unreconciled handoffs must require conflict review",
                );
                self.expect(
                    event
                        .conflict_review_ref
                        .as_ref()
                        .is_some_and(|value| !value.trim().is_empty()),
                    "provider_event_ingestion.conflict_ref_missing",
                    "conflict-review-required events must cite a conflict review ref",
                );
            }

            if event.freshness.freshness_class != FreshnessLabel::Fresh {
                self.expect(
                    matches!(
                        event.surface_state,
                        ImportedEventSurfaceState::Stale
                            | ImportedEventSurfaceState::ConflictReviewRequired
                            | ImportedEventSurfaceState::Buffered
                    ),
                    "provider_event_ingestion.stale_label_wrong",
                    "stale imported events must remain labeled Stale, Buffered, or Conflict review required",
                );
            }

            if event.surface_state.may_import_state() {
                self.expect(
                    event.policy_verdict.verdict_class.admits_import(),
                    "provider_event_ingestion.import_without_policy_accept",
                    "Imported events must have an accepting policy verdict before local state changes",
                );
            }
        }
    }

    fn validate_surface_projections(&mut self) {
        for row in &self.packet.surface_projections {
            self.coverage.surface_states.insert(row.surface_state);
            self.validate_record(
                &row.record_kind,
                row.schema_version,
                &row.shared_contract_ref,
                PROVIDER_EVENT_SURFACE_PROJECTION_RECORD_KIND,
                "provider_event_ingestion.surface_projection",
            );
            self.expect_nonempty(
                &row.row_id,
                "provider_event_ingestion.surface_row_id_missing",
            );
            self.expect(
                self.event_ids.contains(row.event_ref.as_str()),
                "provider_event_ingestion.surface_event_ref_unknown",
                "surface projections must cite a known imported event",
            );
            self.expect_nonempty(
                &row.canonical_local_object_ref,
                "provider_event_ingestion.surface_local_ref_missing",
            );
            if row.conflict_review_required {
                self.expect(
                    row.surface_state == ImportedEventSurfaceState::ConflictReviewRequired,
                    "provider_event_ingestion.surface_conflict_label_wrong",
                    "conflict-review surface rows must use the conflict review label",
                );
            }
        }
    }

    fn validate_support_export(&mut self) {
        let export = &self.packet.support_export;
        self.validate_record(
            &export.record_kind,
            export.schema_version,
            &export.shared_contract_ref,
            PROVIDER_EVENT_SUPPORT_EXPORT_PACKET_RECORD_KIND,
            "provider_event_ingestion.support_export",
        );
        self.expect_nonempty(
            &export.support_export_id,
            "provider_event_ingestion.support_export_id_missing",
        );
        self.expect(
            !export.raw_provider_payload_export_allowed && !export.raw_callback_url_export_allowed,
            "provider_event_ingestion.support_raw_export_allowed",
            "support exports must not allow raw provider payload or callback URL export",
        );
        self.expect(
            export.event_summaries.len() == self.packet.event_envelopes.len(),
            "provider_event_ingestion.support_summary_count_mismatch",
            "support exports must summarize every imported event",
        );
    }

    fn validate_delivery_idempotency(&mut self) {
        let groups = self
            .delivery_groups
            .iter()
            .map(|(key, events)| (*key, events.clone()))
            .collect::<Vec<_>>();
        for (_key, events) in groups {
            let mutating_count = events
                .iter()
                .flat_map(|event| event.resulting_local_objects.iter())
                .filter(|outcome| outcome.resulting_state.mutates_user_visible_state())
                .count();
            self.expect(
                mutating_count <= 1,
                "provider_event_ingestion.duplicate_mutated_twice",
                "one dedupe key must not mutate user-visible provider-linked state more than once",
            );
            if events.len() > 1 {
                self.expect(
                    events.iter().any(|event| {
                        event.dedupe.replay_count > 0
                            && event.dedupe.replay_decision.is_duplicate_safe()
                            && event.surface_state == ImportedEventSurfaceState::Replayed
                    }),
                    "provider_event_ingestion.duplicate_replay_missing",
                    "duplicate deliveries must include a replayed dedupe/noop event",
                );
            }
        }
    }

    fn validate_required_coverage(&mut self) {
        for state in [
            ImportedEventSurfaceState::Imported,
            ImportedEventSurfaceState::Buffered,
            ImportedEventSurfaceState::Replayed,
            ImportedEventSurfaceState::Denied,
            ImportedEventSurfaceState::Stale,
            ImportedEventSurfaceState::ConflictReviewRequired,
        ] {
            self.expect(
                self.coverage.surface_states.contains(&state),
                "provider_event_ingestion.coverage_surface_state_missing",
                "coverage must include Imported, Buffered, Replayed, Denied, Stale, and Conflict review required labels",
            );
        }
        for authority in [
            ProviderEventAuthoritySourceClass::ConnectedAccount,
            ProviderEventAuthoritySourceClass::InstallationGrant,
        ] {
            self.expect(
                self.coverage.authority_sources.contains(&authority),
                "provider_event_ingestion.coverage_authority_missing",
                "coverage must include connected-account and installation-grant authority sources",
            );
        }
        for ingress in [
            ProviderEventIngressClass::Webhook,
            ProviderEventIngressClass::BrowserReturnCallback,
            ProviderEventIngressClass::MirrorIngress,
            ProviderEventIngressClass::PublishLaterDrain,
        ] {
            self.expect(
                self.coverage.ingress_classes.contains(&ingress),
                "provider_event_ingestion.coverage_ingress_missing",
                "coverage must include webhook, browser callback, mirror ingress, and publish-later drain paths",
            );
        }
        self.expect(
            self.coverage
                .overlap_classes
                .iter()
                .any(|overlap| overlap.requires_conflict_review()),
            "provider_event_ingestion.coverage_overlap_missing",
            "coverage must include a local draft, publish-later, or browser-handoff overlap",
        );
    }

    fn validate_record(
        &mut self,
        record_kind: &str,
        schema_version: u32,
        shared_contract_ref: &str,
        expected_record_kind: &str,
        prefix: &str,
    ) {
        self.expect(
            record_kind == expected_record_kind,
            &format!("{prefix}.record_kind"),
            "record_kind must match the expected discriminator",
        );
        self.expect(
            schema_version == PROVIDER_EVENT_INGESTION_PROVENANCE_SCHEMA_VERSION,
            &format!("{prefix}.schema_version"),
            "schema_version must match the crate constant",
        );
        self.expect(
            shared_contract_ref == PROVIDER_EVENT_INGESTION_PROVENANCE_SHARED_CONTRACT_REF,
            &format!("{prefix}.shared_contract_ref"),
            "shared_contract_ref must match the stable contract ref",
        );
    }

    fn expect_nonempty(&mut self, value: &str, check_id: &str) {
        self.expect(
            !value.trim().is_empty(),
            check_id,
            "required string value must be non-empty",
        );
    }

    fn expect(&mut self, condition: bool, check_id: &str, message: &str) {
        if !condition {
            self.findings.push(ProviderEventIngestionValidationFinding {
                record_kind: PROVIDER_EVENT_INGESTION_VALIDATION_FINDING_RECORD_KIND.to_string(),
                check_id: check_id.to_string(),
                message: message.to_string(),
            });
        }
    }
}

/// Validates an imported-provider event provenance packet.
pub fn validate_provider_event_ingestion_provenance_packet(
    packet: &ProviderEventIngestionProvenancePacket,
) -> ProviderEventIngestionValidationReport {
    packet.validate()
}

/// Builds a seeded imported-provider event provenance packet.
pub fn seeded_provider_event_ingestion_provenance_packet() -> ProviderEventIngestionProvenancePacket
{
    let events = vec![
        imported_event(
            "provider_event_ingestion.event.pr_comment.imported",
            ProviderEventAuthoritySourceClass::ConnectedAccount,
            "account_scope.connected_account.primary",
            ProviderEventIngressClass::Webhook,
            ImportedProviderEventClass::CommentMutation,
            ProviderFamily::CodeHost,
            ProviderObjectKind::PullRequest,
            "provider.delivery.pr_comment.1001",
            "provider.object.pr.42.comment.7",
            "provider.dedupe.pr_comment.1001",
            ProviderEventReplayDecisionClass::FirstDelivery,
            0,
            FreshnessLabel::Fresh,
            ProviderEventPolicyVerdictClass::Accepted,
            ProviderEventOverlapClass::NoOverlap,
            None,
            None,
            ImportedEventSurfaceState::Imported,
            ProviderEventResultingStateClass::ImportedProviderState,
            "Imported verified provider comment into the review lane.",
        ),
        imported_event(
            "provider_event_ingestion.event.pr_comment.replayed",
            ProviderEventAuthoritySourceClass::ConnectedAccount,
            "account_scope.connected_account.primary",
            ProviderEventIngressClass::Webhook,
            ImportedProviderEventClass::CommentMutation,
            ProviderFamily::CodeHost,
            ProviderObjectKind::PullRequest,
            "provider.delivery.pr_comment.1001",
            "provider.object.pr.42.comment.7",
            "provider.dedupe.pr_comment.1001",
            ProviderEventReplayDecisionClass::DuplicateDedupeNoop,
            1,
            FreshnessLabel::Fresh,
            ProviderEventPolicyVerdictClass::Accepted,
            ProviderEventOverlapClass::NoOverlap,
            None,
            None,
            ImportedEventSurfaceState::Replayed,
            ProviderEventResultingStateClass::BufferedNoMutation,
            "Duplicate provider comment delivery was replayed as a dedupe noop.",
        ),
        imported_event(
            "provider_event_ingestion.event.issue.buffered",
            ProviderEventAuthoritySourceClass::InstallationGrant,
            "account_scope.installation_grant.issue_tracker",
            ProviderEventIngressClass::ProviderImportSession,
            ImportedProviderEventClass::WorkItemMutation,
            ProviderFamily::IssueTracker,
            ProviderObjectKind::IssueOrWorkItem,
            "provider.delivery.issue.2001",
            "provider.object.issue.84",
            "provider.dedupe.issue.2001",
            ProviderEventReplayDecisionClass::BufferedPendingSequence,
            0,
            FreshnessLabel::Fresh,
            ProviderEventPolicyVerdictClass::AcceptedNarrowed,
            ProviderEventOverlapClass::NoOverlap,
            None,
            None,
            ImportedEventSurfaceState::Buffered,
            ProviderEventResultingStateClass::BufferedNoMutation,
            "Issue import was buffered until a missing page backfills.",
        ),
        imported_event(
            "provider_event_ingestion.event.callback.denied",
            ProviderEventAuthoritySourceClass::InstallationGrant,
            "account_scope.installation_grant.code_host",
            ProviderEventIngressClass::BrowserReturnCallback,
            ImportedProviderEventClass::CallbackDenied,
            ProviderFamily::CodeHost,
            ProviderObjectKind::PullRequest,
            "provider.delivery.callback.3001",
            "provider.object.pr.42",
            "provider.dedupe.callback.3001",
            ProviderEventReplayDecisionClass::FirstDelivery,
            0,
            FreshnessLabel::Fresh,
            ProviderEventPolicyVerdictClass::Denied,
            ProviderEventOverlapClass::NoOverlap,
            Some(browser_handoff_origin(false)),
            None,
            ImportedEventSurfaceState::Denied,
            ProviderEventResultingStateClass::DeniedNoMutation,
            "Browser callback was denied after host proof failed.",
        ),
        imported_event(
            "provider_event_ingestion.event.check.stale",
            ProviderEventAuthoritySourceClass::InstallationGrant,
            "account_scope.installation_grant.ci",
            ProviderEventIngressClass::MirrorIngress,
            ImportedProviderEventClass::CheckStateMutation,
            ProviderFamily::CiChecks,
            ProviderObjectKind::CheckRun,
            "provider.delivery.check.4001",
            "provider.object.check.510",
            "provider.dedupe.check.4001",
            ProviderEventReplayDecisionClass::ReplayFreshnessOnly,
            1,
            FreshnessLabel::ExpiredBeyondWindow,
            ProviderEventPolicyVerdictClass::AuditOnly,
            ProviderEventOverlapClass::NoOverlap,
            None,
            None,
            ImportedEventSurfaceState::Stale,
            ProviderEventResultingStateClass::StaleNoMutation,
            "Mirror-derived check state was stale and stayed audit-only.",
        ),
        imported_event(
            "provider_event_ingestion.event.publish_later.conflict",
            ProviderEventAuthoritySourceClass::ConnectedAccount,
            "account_scope.connected_account.issue_tracker",
            ProviderEventIngressClass::PublishLaterDrain,
            ImportedProviderEventClass::WorkItemMutation,
            ProviderFamily::IssueTracker,
            ProviderObjectKind::IssueOrWorkItem,
            "provider.delivery.issue.5001",
            "provider.object.issue.84",
            "provider.dedupe.issue.5001",
            ProviderEventReplayDecisionClass::ReplayRequiresReview,
            0,
            FreshnessLabel::StaleWithinWindow,
            ProviderEventPolicyVerdictClass::ConflictReviewRequired,
            ProviderEventOverlapClass::PublishLaterQueue,
            None,
            Some("provider_conflict_review.issue.84.remote_status".to_string()),
            ImportedEventSurfaceState::ConflictReviewRequired,
            ProviderEventResultingStateClass::ConflictReviewQueued,
            "Publish-later drain overlapped a local draft and forced conflict review.",
        ),
        imported_event(
            "provider_event_ingestion.event.handoff.conflict",
            ProviderEventAuthoritySourceClass::ConnectedAccount,
            "account_scope.connected_account.code_host",
            ProviderEventIngressClass::BrowserReturnCallback,
            ImportedProviderEventClass::ReviewMutation,
            ProviderFamily::CodeHost,
            ProviderObjectKind::PullRequest,
            "provider.delivery.review.6001",
            "provider.object.pr.42.review",
            "provider.dedupe.review.6001",
            ProviderEventReplayDecisionClass::ReplayRequiresReview,
            0,
            FreshnessLabel::Fresh,
            ProviderEventPolicyVerdictClass::ConflictReviewRequired,
            ProviderEventOverlapClass::BrowserHandoffSession,
            Some(browser_handoff_origin(true)),
            Some("provider_conflict_review.pr.42.browser_return".to_string()),
            ImportedEventSurfaceState::ConflictReviewRequired,
            ProviderEventResultingStateClass::LocalDraftPreserved,
            "Browser-return review update overlapped an unreconciled handoff session.",
        ),
    ];

    let surface_projections = events
        .iter()
        .enumerate()
        .flat_map(|(index, event)| {
            ["work_item", "review", "activity_center", "support_export"]
                .into_iter()
                .map(move |surface| surface_projection(index, surface, event))
        })
        .collect::<Vec<_>>();
    let support_export = ProviderEventSupportExportPacket {
        record_kind: PROVIDER_EVENT_SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
        schema_version: PROVIDER_EVENT_INGESTION_PROVENANCE_SCHEMA_VERSION,
        shared_contract_ref: PROVIDER_EVENT_INGESTION_PROVENANCE_SHARED_CONTRACT_REF.to_string(),
        support_export_id: "provider_event_ingestion.support_export.seed".to_string(),
        event_summaries: events.iter().map(ProviderEventSupportSummary::from).collect(),
        raw_provider_payload_export_allowed: false,
        raw_callback_url_export_allowed: false,
        summary_label: "Support export names imported-event source, replay, policy, and resulting local refs without raw payloads.".to_string(),
    };

    ProviderEventIngestionProvenancePacket {
        record_kind: PROVIDER_EVENT_INGESTION_PROVENANCE_PACKET_RECORD_KIND.to_string(),
        schema_version: PROVIDER_EVENT_INGESTION_PROVENANCE_SCHEMA_VERSION,
        shared_contract_ref: PROVIDER_EVENT_INGESTION_PROVENANCE_SHARED_CONTRACT_REF.to_string(),
        packet_id: "provider_event_ingestion.packet.seed".to_string(),
        generated_at: "2026-05-21T12:00:00Z".to_string(),
        event_envelopes: events,
        surface_projections,
        support_export,
    }
}

fn imported_event(
    event_id: &str,
    authority_source_class: ProviderEventAuthoritySourceClass,
    authority_source_ref: &str,
    ingress_class: ProviderEventIngressClass,
    event_class: ImportedProviderEventClass,
    provider_family: ProviderFamily,
    object_kind: ProviderObjectKind,
    external_delivery_id: &str,
    scoped_object_ref: &str,
    dedupe_key: &str,
    replay_decision: ProviderEventReplayDecisionClass,
    replay_count: u32,
    freshness_class: FreshnessLabel,
    verdict_class: ProviderEventPolicyVerdictClass,
    overlap_class: ProviderEventOverlapClass,
    browser_handoff_origin: Option<BrowserHandoffOriginRef>,
    conflict_review_ref: Option<String>,
    surface_state: ImportedEventSurfaceState,
    resulting_state: ProviderEventResultingStateClass,
    support_summary: &str,
) -> ImportedProviderEventEnvelope {
    ImportedProviderEventEnvelope {
        record_kind: IMPORTED_PROVIDER_EVENT_ENVELOPE_RECORD_KIND.to_string(),
        schema_version: PROVIDER_EVENT_INGESTION_PROVENANCE_SCHEMA_VERSION,
        shared_contract_ref: PROVIDER_EVENT_INGESTION_PROVENANCE_SHARED_CONTRACT_REF.to_string(),
        event_id: event_id.to_string(),
        provider_descriptor_ref: format!("provider.descriptor.{:?}", provider_family)
            .to_lowercase(),
        provider_family,
        authority_source_class,
        authority_source_ref: authority_source_ref.to_string(),
        ingress_class,
        event_class,
        canonical_object_refs: vec![CanonicalProviderObjectRef {
            object_kind,
            canonical_provider_object_id: scoped_object_ref.to_string(),
            canonical_local_object_ref: format!("local.object.{scoped_object_ref}"),
            target_ref: "provider.target.repo_or_project.primary".to_string(),
        }],
        freshness: FreshnessTruth {
            freshness_class,
            observed_at: Some("2026-05-21T12:00:00Z".to_string()),
            freshness_floor_ref: "provider.freshness.floor.stable".to_string(),
            stale_after: Some("PT30M".to_string()),
            degraded_reason: (freshness_class != FreshnessLabel::Fresh)
                .then(|| "Provider event did not satisfy the current freshness floor.".to_string()),
            import_session_ref: Some(format!("provider.import_session.{dedupe_key}")),
        },
        dedupe: ProviderEventDedupeEnvelope {
            external_delivery_id: external_delivery_id.to_string(),
            scoped_object_ref: scoped_object_ref.to_string(),
            provider_host_ref: "provider.host.primary".to_string(),
            tenant_or_org_scope_ref: "provider.scope.tenant.primary".to_string(),
            dedupe_key: dedupe_key.to_string(),
            replay_key: format!("provider.replay.{dedupe_key}"),
            replay_decision,
            replay_count,
        },
        browser_handoff_origin,
        policy_verdict: ProviderEventPolicyVerdict {
            verdict_class,
            policy_epoch_ref: "policy.provider_events.epoch.7".to_string(),
            effective_scope_ref: "account_scope.effective_scope.provider_events".to_string(),
            audit_event_refs: vec![format!("audit.{event_id}")],
            reason: matches!(
                verdict_class,
                ProviderEventPolicyVerdictClass::Denied
                    | ProviderEventPolicyVerdictClass::AcceptedNarrowed
                    | ProviderEventPolicyVerdictClass::ConflictReviewRequired
                    | ProviderEventPolicyVerdictClass::AuditOnly
            )
            .then(|| support_summary.to_string()),
        },
        overlap_class,
        conflict_review_ref,
        resulting_local_objects: vec![ProviderEventLocalObjectOutcome {
            local_object_ref: format!("local.object.{scoped_object_ref}"),
            resulting_state,
            summary_label: support_summary.to_string(),
        }],
        surface_state,
        support_summary: support_summary.to_string(),
        raw_provider_payload_refs_present: false,
        raw_callback_url_refs_present: false,
    }
}

fn browser_handoff_origin(unreconciled_session: bool) -> BrowserHandoffOriginRef {
    BrowserHandoffOriginRef {
        handoff_packet_ref: "provider_browser_handoff.packet.pr.42".to_string(),
        origin_class: HandoffOriginClass::WorkspaceReviewLane,
        return_anchor_ref: "review.anchor.pr.42".to_string(),
        unreconciled_session,
    }
}

fn surface_projection(
    index: usize,
    surface: &str,
    event: &ImportedProviderEventEnvelope,
) -> ProviderEventSurfaceProjection {
    ProviderEventSurfaceProjection {
        record_kind: PROVIDER_EVENT_SURFACE_PROJECTION_RECORD_KIND.to_string(),
        schema_version: PROVIDER_EVENT_INGESTION_PROVENANCE_SCHEMA_VERSION,
        shared_contract_ref: PROVIDER_EVENT_INGESTION_PROVENANCE_SHARED_CONTRACT_REF.to_string(),
        row_id: format!("provider_event_ingestion.surface.{surface}.{index}"),
        event_ref: event.event_id.clone(),
        surface: surface.to_string(),
        surface_state: event.surface_state,
        canonical_local_object_ref: event
            .resulting_local_objects
            .first()
            .map(|outcome| outcome.local_object_ref.clone())
            .unwrap_or_default(),
        freshness_class: event.freshness.freshness_class,
        conflict_review_required: event.surface_state
            == ImportedEventSurfaceState::ConflictReviewRequired,
        summary_label: event.support_summary.clone(),
    }
}
