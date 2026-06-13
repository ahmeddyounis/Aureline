//! Canonical M5 provider event-ingestion packet, linked-object freshness
//! vocabulary, and redaction-safe support export.
//!
//! This module composes the lower-level imported-event provenance packet and
//! provider-event reconciliation page into one M5 contract that downstream
//! work-item, review, support/export, and docs/help surfaces can quote
//! directly. The packet keeps three truths aligned:
//!
//! - every external delivery still routes through typed envelopes, import
//!   sessions, replay ledgers, deny events, and reconciliation results;
//! - provider-linked objects reuse one stable freshness and partiality
//!   vocabulary across product and support/export surfaces; and
//! - raw provider payloads, callback URLs, and credential material remain
//!   outside the packet by default.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::provider_event_ingestion_and_provenance::{
    seeded_provider_event_ingestion_provenance_packet, ProviderEventAuthoritySourceClass,
    ProviderEventIngestionProvenancePacket,
};
use crate::reconciliation::{
    seeded_provider_event_reconciliation_page, CallbackDenyReasonClass, EventDispositionClass,
    ProviderEventReconciliationPage, ProviderEventSourceClass, ProviderEventTypeClass,
    TruthCompletenessClass,
};
use crate::registry::{FreshnessLabel, ProviderFamily, ProviderObjectKind, RedactionClass};

/// Schema version exported by canonical provider event-ingestion packets.
pub const PROVIDER_EVENT_INGESTION_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by canonical provider event-ingestion packets.
pub const PROVIDER_EVENT_INGESTION_SHARED_CONTRACT_REF: &str = "providers:event_ingestion:v1";

/// Stable record kind for [`ProviderEventIngestionPacket`].
pub const PROVIDER_EVENT_INGESTION_PACKET_RECORD_KIND: &str =
    "providers_event_ingestion_packet_record";

/// Stable record kind for [`ProviderLinkedObjectStateRow`].
pub const PROVIDER_LINKED_OBJECT_STATE_ROW_RECORD_KIND: &str =
    "provider_linked_object_state_row_record";

/// Stable record kind for [`ProviderEventIngestionConsumerProjectionRow`].
pub const PROVIDER_EVENT_INGESTION_CONSUMER_PROJECTION_RECORD_KIND: &str =
    "provider_event_ingestion_consumer_projection_record";

/// Stable record kind for [`ProviderEventIngestionSupportExport`].
pub const PROVIDER_EVENT_INGESTION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "providers_event_ingestion_support_export_record";

/// Stable record kind for [`ProviderEventIngestionValidationReport`].
pub const PROVIDER_EVENT_INGESTION_VALIDATION_REPORT_RECORD_KIND: &str =
    "providers_event_ingestion_validation_report";

/// Stable record kind for [`ProviderEventIngestionValidationFinding`].
pub const PROVIDER_EVENT_INGESTION_VALIDATION_FINDING_RECORD_KIND: &str =
    "providers_event_ingestion_validation_finding";

/// Repo-relative path of the boundary schema.
pub const PROVIDER_EVENT_INGESTION_SCHEMA_REF: &str =
    "schemas/providers/provider_event_ingestion.schema.json";

/// Repo-relative path of the contract doc.
pub const PROVIDER_EVENT_INGESTION_DOC_REF: &str = "docs/providers/m5/event_ingestion.md";

/// Repo-relative path of the checked fixture directory.
pub const PROVIDER_EVENT_INGESTION_FIXTURE_DIR: &str = "fixtures/providers/m5/event_ingestion";

/// Repo-relative path of the Markdown artifact summarizing the seeded packet.
pub const PROVIDER_EVENT_INGESTION_ARTIFACT_REF: &str = "artifacts/provider/m5/event_ingestion.md";

/// Repo-relative path of the checked support export artifact.
pub const PROVIDER_EVENT_INGESTION_SUPPORT_EXPORT_ARTIFACT_REF: &str =
    "artifacts/provider/m5/event_ingestion/support_export.json";

/// Stable, user-visible linked-object state vocabulary for external-event truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderLinkedObjectStateClass {
    /// Current provider-backed state is fresh for the declared scope.
    Fresh,
    /// Imported state is partial for the declared scope.
    Partial,
    /// Imported state is delayed and may be superseded by later provider truth.
    Delayed,
    /// Historical or missing state was backfilled explicitly.
    Backfilled,
    /// Imported or cached state is stale.
    Stale,
    /// State came through a mirror rather than the live provider path.
    MirrorDerived,
    /// Callback or webhook mutation was denied before local state changed.
    CallbackDenied,
}

impl ProviderLinkedObjectStateClass {
    /// Every stable linked-object state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Fresh,
        Self::Partial,
        Self::Delayed,
        Self::Backfilled,
        Self::Stale,
        Self::MirrorDerived,
        Self::CallbackDenied,
    ];

    /// Returns true when the state represents denied inbound authority.
    pub const fn is_denied(self) -> bool {
        matches!(self, Self::CallbackDenied)
    }
}

/// First consumer surface that projects the canonical linked-object state row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderEventIngestionConsumerSurface {
    /// Work-item detail headers and activity.
    WorkItemDetail,
    /// Review workspace or change-detail surface.
    ReviewWorkspace,
    /// CLI or headless inspection output.
    CliHeadless,
    /// Companion triage or follow surface.
    CompanionTriage,
    /// Support/export packet projection.
    SupportExport,
    /// Docs/help inspection of provider truth posture.
    DocsHelp,
    /// Audit or incident timeline surface.
    AuditTimeline,
}

/// Contract refs consumed by the canonical packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventIngestionContractRefs {
    /// Canonical packet schema ref.
    pub provider_event_ingestion_schema_ref: String,
    /// Imported-provider event provenance schema ref.
    pub provenance_packet_schema_ref: String,
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
    /// Provider scope-review schema ref.
    pub scope_review_schema_ref: String,
}

impl ProviderEventIngestionContractRefs {
    fn all_refs(&self) -> [&str; 8] {
        [
            &self.provider_event_ingestion_schema_ref,
            &self.provenance_packet_schema_ref,
            &self.provider_event_envelope_schema_ref,
            &self.import_session_schema_ref,
            &self.replay_ledger_item_schema_ref,
            &self.reconciliation_result_schema_ref,
            &self.callback_deny_event_schema_ref,
            &self.scope_review_schema_ref,
        ]
    }
}

/// One canonical linked-object row for provider event-ingestion truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLinkedObjectStateRow {
    /// Stable record kind for this row.
    pub record_kind: String,
    /// Stable row id.
    pub row_id: String,
    /// Canonical local object ref the row updates or preserves.
    pub canonical_local_object_ref: String,
    /// Provider descriptor ref for the object.
    pub provider_descriptor_ref: String,
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Provider object kind.
    pub object_kind: ProviderObjectKind,
    /// Imported-provider provenance event that kept the user-facing label honest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance_event_ref: Option<String>,
    /// Provider-event reconciliation envelope that carried the external delivery.
    pub reconciliation_event_ref: String,
    /// Import session that materialized or attempted the latest provider truth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_session_ref: Option<String>,
    /// Replay/redelivery ledger item that decided the delivery.
    pub replay_ledger_item_ref: String,
    /// Draft or deferred-publish reconciliation result when local continuity overlapped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reconciliation_result_ref: Option<String>,
    /// Callback deny event when the inbound event was rejected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deny_event_ref: Option<String>,
    /// Authority source class visible to the user.
    pub authority_source_class: ProviderEventAuthoritySourceClass,
    /// External source class that produced the row.
    pub source_class: ProviderEventSourceClass,
    /// Event type that last changed or denied the row.
    pub event_type: ProviderEventTypeClass,
    /// Freshness label carried by the latest imported observation.
    pub freshness_class: FreshnessLabel,
    /// Completeness or replay truth class carried by the latest imported observation.
    pub truth_class: TruthCompletenessClass,
    /// Stable linked-object state projected across product and export surfaces.
    pub linked_state: ProviderLinkedObjectStateClass,
    /// Final disposition of the external delivery.
    pub final_disposition: EventDispositionClass,
    /// Redaction-safe row summary.
    pub summary_label: String,
}

/// Consumer projection for one linked-object state row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventIngestionConsumerProjectionRow {
    /// Stable record kind for this projection.
    pub record_kind: String,
    /// Stable projection row id.
    pub row_id: String,
    /// Consumer surface receiving the projection.
    pub surface: ProviderEventIngestionConsumerSurface,
    /// Back-reference to the canonical linked-object row.
    pub linked_object_state_row_ref: String,
    /// Canonical local object ref shown by the surface.
    pub canonical_local_object_ref: String,
    /// Stable linked-object state shown by the surface.
    pub linked_state: ProviderLinkedObjectStateClass,
    /// Redaction-safe projection summary.
    pub summary_label: String,
}

/// Redaction-safe support summary for one linked-object row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLinkedObjectSupportSummary {
    /// Linked-object row id.
    pub row_id: String,
    /// Canonical local object ref.
    pub canonical_local_object_ref: String,
    /// Stable linked-object state.
    pub linked_state: ProviderLinkedObjectStateClass,
    /// Source class that last updated the row.
    pub source_class: ProviderEventSourceClass,
    /// Event type that last updated the row.
    pub event_type: ProviderEventTypeClass,
    /// Truth class carried by the imported observation.
    pub truth_class: TruthCompletenessClass,
    /// Final disposition of the delivery.
    pub final_disposition: EventDispositionClass,
    /// Deny reason when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deny_reason: Option<CallbackDenyReasonClass>,
    /// Redaction-safe support summary.
    pub summary_label: String,
}

impl From<&ProviderLinkedObjectStateRow> for ProviderLinkedObjectSupportSummary {
    fn from(row: &ProviderLinkedObjectStateRow) -> Self {
        Self {
            row_id: row.row_id.clone(),
            canonical_local_object_ref: row.canonical_local_object_ref.clone(),
            linked_state: row.linked_state,
            source_class: row.source_class,
            event_type: row.event_type,
            truth_class: row.truth_class,
            final_disposition: row.final_disposition,
            deny_reason: None,
            summary_label: row.summary_label.clone(),
        }
    }
}

/// Redaction-safe support export projection for the canonical packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventIngestionSupportExport {
    /// Stable record kind for the support export.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Packet id that produced the export.
    pub packet_id: String,
    /// Linked-object row summaries.
    pub linked_object_summaries: Vec<ProviderLinkedObjectSupportSummary>,
    /// Guardrail: raw provider payload export is not allowed by default.
    pub raw_provider_payload_export_allowed: bool,
    /// Guardrail: raw callback URL export is not allowed by default.
    pub raw_callback_url_export_allowed: bool,
    /// Redaction posture for the support export.
    pub redaction_class: RedactionClass,
}

/// Canonical M5 provider event-ingestion packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventIngestionPacket {
    /// Stable record kind for the packet.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Packet generation time.
    pub generated_at: String,
    /// Source contract refs consumed by the packet.
    pub contract_refs: ProviderEventIngestionContractRefs,
    /// Imported-provider provenance packet used by user-visible state projections.
    pub provenance_packet: ProviderEventIngestionProvenancePacket,
    /// Provider-event reconciliation page used by replay, import, and drift review.
    pub reconciliation_page: ProviderEventReconciliationPage,
    /// Canonical linked-object state rows.
    pub linked_object_state_rows: Vec<ProviderLinkedObjectStateRow>,
    /// First-consumer surface projections.
    pub consumer_projections: Vec<ProviderEventIngestionConsumerProjectionRow>,
    /// Redaction-safe support export projection.
    pub support_export: ProviderEventIngestionSupportExport,
}

impl ProviderEventIngestionPacket {
    /// Validates the packet against stable M5 provider event-ingestion invariants.
    pub fn validate(&self) -> ProviderEventIngestionValidationReport {
        let mut validator = Validator::new(self);
        validator.run();
        validator.finish()
    }
}

/// Coverage observed while validating a canonical packet.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ProviderEventIngestionCoverage {
    /// Consumer surfaces covered by projections.
    pub consumer_surfaces: BTreeSet<ProviderEventIngestionConsumerSurface>,
    /// External source classes covered by linked-object rows.
    pub source_classes: BTreeSet<ProviderEventSourceClass>,
    /// Stable linked-object states covered by the packet.
    pub linked_states: BTreeSet<ProviderLinkedObjectStateClass>,
    /// Truth classes covered by linked-object rows.
    pub truth_classes: BTreeSet<TruthCompletenessClass>,
}

/// One validation finding for the canonical packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventIngestionValidationFinding {
    /// Stable record kind for the finding.
    pub record_kind: String,
    /// Stable check id.
    pub check_id: String,
    /// Redaction-safe message.
    pub message: String,
}

/// Validation report for the canonical packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEventIngestionValidationReport {
    /// Stable record kind for the validation report.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Packet id under validation.
    pub packet_id: String,
    /// Whether the packet passed validation.
    pub passed: bool,
    /// Coverage observed while validating the packet.
    pub coverage: ProviderEventIngestionCoverage,
    /// Validation findings.
    pub findings: Vec<ProviderEventIngestionValidationFinding>,
}

struct Validator<'a> {
    packet: &'a ProviderEventIngestionPacket,
    coverage: ProviderEventIngestionCoverage,
    findings: Vec<ProviderEventIngestionValidationFinding>,
    linked_object_row_ids: BTreeSet<&'a str>,
}

impl<'a> Validator<'a> {
    fn new(packet: &'a ProviderEventIngestionPacket) -> Self {
        Self {
            packet,
            coverage: ProviderEventIngestionCoverage::default(),
            findings: Vec::new(),
            linked_object_row_ids: BTreeSet::new(),
        }
    }

    fn run(&mut self) {
        self.validate_header();
        self.validate_nested_packets();
        self.validate_linked_object_rows();
        self.validate_consumer_projections();
        self.validate_support_export();
        self.validate_required_coverage();
    }

    fn finish(self) -> ProviderEventIngestionValidationReport {
        ProviderEventIngestionValidationReport {
            record_kind: PROVIDER_EVENT_INGESTION_VALIDATION_REPORT_RECORD_KIND.to_string(),
            schema_version: PROVIDER_EVENT_INGESTION_SCHEMA_VERSION,
            packet_id: self.packet.packet_id.clone(),
            passed: self.findings.is_empty(),
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn validate_header(&mut self) {
        self.expect(
            self.packet.record_kind == PROVIDER_EVENT_INGESTION_PACKET_RECORD_KIND,
            "provider_event_ingestion.packet_record_kind",
            "packet record_kind must match the canonical discriminator",
        );
        self.expect(
            self.packet.schema_version == PROVIDER_EVENT_INGESTION_SCHEMA_VERSION,
            "provider_event_ingestion.packet_schema_version",
            "packet schema_version must match the crate constant",
        );
        self.expect(
            self.packet.shared_contract_ref == PROVIDER_EVENT_INGESTION_SHARED_CONTRACT_REF,
            "provider_event_ingestion.packet_shared_contract_ref",
            "packet shared_contract_ref must match the canonical contract ref",
        );
        self.expect_nonempty(
            &self.packet.packet_id,
            "provider_event_ingestion.packet_id_missing",
        );
        self.expect_nonempty(
            &self.packet.packet_label,
            "provider_event_ingestion.packet_label_missing",
        );
        self.expect_nonempty(
            &self.packet.generated_at,
            "provider_event_ingestion.generated_at_missing",
        );
        self.expect(
            !self.packet.linked_object_state_rows.is_empty(),
            "provider_event_ingestion.linked_rows_missing",
            "packet must include canonical linked-object state rows",
        );
        for contract_ref in self.packet.contract_refs.all_refs() {
            self.expect(
                !contract_ref.trim().is_empty(),
                "provider_event_ingestion.contract_ref_missing",
                "every contract ref must be non-empty",
            );
        }
    }

    fn validate_nested_packets(&mut self) {
        let provenance_report = self.packet.provenance_packet.validate();
        self.expect(
            provenance_report.passed,
            "provider_event_ingestion.provenance_invalid",
            "nested provenance packet must validate",
        );

        let reconciliation_report = self.packet.reconciliation_page.validate();
        self.expect(
            reconciliation_report.passed,
            "provider_event_ingestion.reconciliation_invalid",
            "nested reconciliation page must validate",
        );
    }

    fn validate_linked_object_rows(&mut self) {
        let provenance_event_ids = self
            .packet
            .provenance_packet
            .event_envelopes
            .iter()
            .map(|event| event.event_id.as_str())
            .collect::<BTreeSet<_>>();
        let reconciliation_events = self
            .packet
            .reconciliation_page
            .event_envelopes
            .iter()
            .map(|event| (event.event_id.as_str(), event))
            .collect::<BTreeMap<_, _>>();
        let import_sessions = self
            .packet
            .reconciliation_page
            .import_sessions
            .iter()
            .map(|session| (session.import_session_id.as_str(), session))
            .collect::<BTreeMap<_, _>>();
        let replay_ledger_items = self
            .packet
            .reconciliation_page
            .replay_ledger_items
            .iter()
            .map(|item| (item.replay_ledger_item_id.as_str(), item))
            .collect::<BTreeMap<_, _>>();
        let reconciliation_results = self
            .packet
            .reconciliation_page
            .reconciliation_results
            .iter()
            .map(|result| (result.reconciliation_result_id.as_str(), result))
            .collect::<BTreeMap<_, _>>();
        let deny_events = self
            .packet
            .reconciliation_page
            .callback_deny_events
            .iter()
            .map(|event| (event.deny_event_id.as_str(), event))
            .collect::<BTreeMap<_, _>>();

        for row in &self.packet.linked_object_state_rows {
            let inserted = self.linked_object_row_ids.insert(row.row_id.as_str());
            self.expect(
                inserted,
                "provider_event_ingestion.linked_row_id_duplicate",
                "linked-object row ids must be unique",
            );
            self.coverage.source_classes.insert(row.source_class);
            self.coverage.linked_states.insert(row.linked_state);
            self.coverage.truth_classes.insert(row.truth_class);

            self.expect(
                row.record_kind == PROVIDER_LINKED_OBJECT_STATE_ROW_RECORD_KIND,
                "provider_event_ingestion.linked_row_record_kind",
                "linked-object row record_kind must match the canonical discriminator",
            );
            self.expect_nonempty(
                &row.canonical_local_object_ref,
                "provider_event_ingestion.linked_row_local_ref_missing",
            );
            self.expect_nonempty(
                &row.provider_descriptor_ref,
                "provider_event_ingestion.linked_row_provider_ref_missing",
            );
            if let Some(provenance_event_ref) = &row.provenance_event_ref {
                self.expect(
                    provenance_event_ids.contains(provenance_event_ref.as_str()),
                    "provider_event_ingestion.linked_row_provenance_ref_unknown",
                    "linked-object rows must reference known provenance events when present",
                );
            }
            let reconciliation_event = reconciliation_events
                .get(row.reconciliation_event_ref.as_str())
                .copied();
            self.expect(
                reconciliation_event.is_some(),
                "provider_event_ingestion.linked_row_reconciliation_event_unknown",
                "linked-object rows must reference known reconciliation events",
            );
            if let Some(import_session_ref) = &row.import_session_ref {
                self.expect(
                    import_sessions.contains_key(import_session_ref.as_str()),
                    "provider_event_ingestion.linked_row_import_session_unknown",
                    "linked-object rows must reference known import sessions",
                );
            }
            self.expect(
                replay_ledger_items.contains_key(row.replay_ledger_item_ref.as_str()),
                "provider_event_ingestion.linked_row_replay_ledger_unknown",
                "linked-object rows must reference known replay-ledger items",
            );
            if let Some(reconciliation_result_ref) = &row.reconciliation_result_ref {
                self.expect(
                    reconciliation_results.contains_key(reconciliation_result_ref.as_str()),
                    "provider_event_ingestion.linked_row_reconcile_result_unknown",
                    "linked-object rows must reference known draft reconciliation results",
                );
            }
            if let Some(deny_event_ref) = &row.deny_event_ref {
                self.expect(
                    deny_events.contains_key(deny_event_ref.as_str()),
                    "provider_event_ingestion.linked_row_deny_event_unknown",
                    "linked-object rows must reference known callback deny events",
                );
            }
            self.expect_nonempty(
                &row.summary_label,
                "provider_event_ingestion.linked_row_summary_missing",
            );

            match row.linked_state {
                ProviderLinkedObjectStateClass::Fresh => {
                    self.expect(
                        row.freshness_class == FreshnessLabel::Fresh
                            && row.truth_class == TruthCompletenessClass::FullSnapshot,
                        "provider_event_ingestion.linked_row_fresh_incoherent",
                        "fresh linked-object rows must carry fresh full-snapshot truth",
                    );
                }
                ProviderLinkedObjectStateClass::Partial => {
                    self.expect(
                        matches!(
                            row.truth_class,
                            TruthCompletenessClass::BoundedPartialSnapshot
                                | TruthCompletenessClass::UnboundedPartialSnapshot
                        ),
                        "provider_event_ingestion.linked_row_partial_incoherent",
                        "partial linked-object rows must carry a partial snapshot truth class",
                    );
                }
                ProviderLinkedObjectStateClass::Delayed => {
                    self.expect(
                        row.truth_class == TruthCompletenessClass::DelayedDelivery,
                        "provider_event_ingestion.linked_row_delayed_incoherent",
                        "delayed linked-object rows must carry delayed delivery truth",
                    );
                }
                ProviderLinkedObjectStateClass::Backfilled => {
                    self.expect(
                        row.truth_class == TruthCompletenessClass::BackfilledSnapshot,
                        "provider_event_ingestion.linked_row_backfilled_incoherent",
                        "backfilled linked-object rows must carry backfilled snapshot truth",
                    );
                }
                ProviderLinkedObjectStateClass::Stale => {
                    self.expect(
                        row.freshness_class != FreshnessLabel::Fresh
                            || row.truth_class.is_non_canonical(),
                        "provider_event_ingestion.linked_row_stale_incoherent",
                        "stale linked-object rows must carry stale freshness or non-canonical truth",
                    );
                }
                ProviderLinkedObjectStateClass::MirrorDerived => {
                    self.expect(
                        row.truth_class == TruthCompletenessClass::MirrorDerivedSnapshot,
                        "provider_event_ingestion.linked_row_mirror_incoherent",
                        "mirror-derived linked-object rows must carry mirror-derived truth",
                    );
                }
                ProviderLinkedObjectStateClass::CallbackDenied => {
                    self.expect(
                        row.final_disposition == EventDispositionClass::DeniedNoMutation
                            && row.deny_event_ref.is_some(),
                        "provider_event_ingestion.linked_row_denied_incoherent",
                        "callback-denied rows must cite a deny event and denied disposition",
                    );
                }
            }

            if let Some(event) = reconciliation_event {
                self.expect(
                    row.source_class == event.source_class
                        && row.event_type == event.event_type
                        && row.final_disposition == event.final_disposition,
                    "provider_event_ingestion.linked_row_event_mismatch",
                    "linked-object row source, event type, and disposition must match the referenced reconciliation event",
                );
            }
        }
    }

    fn validate_consumer_projections(&mut self) {
        for row in &self.packet.consumer_projections {
            self.coverage.consumer_surfaces.insert(row.surface);
            self.expect(
                row.record_kind == PROVIDER_EVENT_INGESTION_CONSUMER_PROJECTION_RECORD_KIND,
                "provider_event_ingestion.consumer_projection_record_kind",
                "consumer projection record_kind must match the canonical discriminator",
            );
            self.expect_nonempty(
                &row.row_id,
                "provider_event_ingestion.consumer_projection_row_id_missing",
            );
            self.expect(
                self.linked_object_row_ids
                    .contains(row.linked_object_state_row_ref.as_str()),
                "provider_event_ingestion.consumer_projection_linked_row_unknown",
                "consumer projections must reference known linked-object rows",
            );
            self.expect_nonempty(
                &row.canonical_local_object_ref,
                "provider_event_ingestion.consumer_projection_local_ref_missing",
            );
            self.expect_nonempty(
                &row.summary_label,
                "provider_event_ingestion.consumer_projection_summary_missing",
            );
        }
    }

    fn validate_support_export(&mut self) {
        let export = &self.packet.support_export;
        self.expect(
            export.record_kind == PROVIDER_EVENT_INGESTION_SUPPORT_EXPORT_RECORD_KIND,
            "provider_event_ingestion.support_export_record_kind",
            "support export record_kind must match the canonical discriminator",
        );
        self.expect(
            export.schema_version == PROVIDER_EVENT_INGESTION_SCHEMA_VERSION,
            "provider_event_ingestion.support_export_schema_version",
            "support export schema_version must match the crate constant",
        );
        self.expect(
            export.packet_id == self.packet.packet_id,
            "provider_event_ingestion.support_export_packet_id_mismatch",
            "support export packet_id must match the canonical packet id",
        );
        self.expect(
            export.linked_object_summaries.len() == self.packet.linked_object_state_rows.len(),
            "provider_event_ingestion.support_export_summary_count_mismatch",
            "support export must summarize every linked-object state row",
        );
        self.expect(
            !export.raw_provider_payload_export_allowed && !export.raw_callback_url_export_allowed,
            "provider_event_ingestion.support_export_raw_refs_allowed",
            "support export must not allow raw provider payload or callback URL export",
        );
    }

    fn validate_required_coverage(&mut self) {
        for state in ProviderLinkedObjectStateClass::ALL {
            self.expect(
                self.coverage.linked_states.contains(&state),
                "provider_event_ingestion.coverage_state_missing",
                "coverage must include every stable linked-object state",
            );
        }

        for source_class in [
            ProviderEventSourceClass::Webhook,
            ProviderEventSourceClass::BrowserReturnCallback,
            ProviderEventSourceClass::PollingRefresh,
            ProviderEventSourceClass::MirrorSync,
            ProviderEventSourceClass::ImportSession,
            ProviderEventSourceClass::DeferredPublishQueue,
        ] {
            self.expect(
                self.coverage.source_classes.contains(&source_class),
                "provider_event_ingestion.coverage_source_missing",
                "coverage must include webhook, browser callback, polling, mirror, import-session, and deferred-publish sources",
            );
        }

        for surface in [
            ProviderEventIngestionConsumerSurface::WorkItemDetail,
            ProviderEventIngestionConsumerSurface::ReviewWorkspace,
            ProviderEventIngestionConsumerSurface::SupportExport,
            ProviderEventIngestionConsumerSurface::DocsHelp,
            ProviderEventIngestionConsumerSurface::AuditTimeline,
        ] {
            self.expect(
                self.coverage.consumer_surfaces.contains(&surface),
                "provider_event_ingestion.coverage_surface_missing",
                "coverage must include work-item, review, support, docs/help, and audit consumers",
            );
        }
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

/// Validates a canonical M5 provider event-ingestion packet.
pub fn validate_provider_event_ingestion_packet(
    packet: &ProviderEventIngestionPacket,
) -> ProviderEventIngestionValidationReport {
    packet.validate()
}

/// Builds the canonical M5 provider event-ingestion packet.
pub fn seeded_provider_event_ingestion_packet() -> ProviderEventIngestionPacket {
    let provenance_packet = seeded_provider_event_ingestion_provenance_packet();
    let reconciliation_page = seeded_provider_event_reconciliation_page();
    let linked_object_state_rows = seeded_linked_object_state_rows();
    let consumer_projections = seeded_consumer_projections(&linked_object_state_rows);
    let support_export = build_support_export(&linked_object_state_rows, &reconciliation_page);

    ProviderEventIngestionPacket {
        record_kind: PROVIDER_EVENT_INGESTION_PACKET_RECORD_KIND.to_string(),
        schema_version: PROVIDER_EVENT_INGESTION_SCHEMA_VERSION,
        shared_contract_ref: PROVIDER_EVENT_INGESTION_SHARED_CONTRACT_REF.to_string(),
        packet_id: "provider-event-ingestion:m5:0001".to_string(),
        packet_label: "M5 provider event ingestion and linked-object truth".to_string(),
        generated_at: "2026-06-12T23:55:00Z".to_string(),
        contract_refs: ProviderEventIngestionContractRefs {
            provider_event_ingestion_schema_ref: PROVIDER_EVENT_INGESTION_SCHEMA_REF.to_string(),
            provenance_packet_schema_ref:
                "schemas/providers/provider_event_ingestion_and_provenance.schema.json".to_string(),
            provider_event_envelope_schema_ref:
                "schemas/providers/provider_event_envelope.schema.json".to_string(),
            import_session_schema_ref: "schemas/providers/import_session.schema.json".to_string(),
            replay_ledger_item_schema_ref: "schemas/providers/replay_ledger_item.schema.json"
                .to_string(),
            reconciliation_result_schema_ref: "schemas/providers/reconciliation_result.schema.json"
                .to_string(),
            callback_deny_event_schema_ref:
                "schemas/providers/provider_callback_deny_event.schema.json".to_string(),
            scope_review_schema_ref: "schemas/providers/provider_scope_review.schema.json"
                .to_string(),
        },
        provenance_packet,
        reconciliation_page,
        linked_object_state_rows,
        consumer_projections,
        support_export,
    }
}

fn build_support_export(
    linked_object_state_rows: &[ProviderLinkedObjectStateRow],
    reconciliation_page: &ProviderEventReconciliationPage,
) -> ProviderEventIngestionSupportExport {
    let deny_reasons = reconciliation_page
        .callback_deny_events
        .iter()
        .map(|event| (event.deny_event_id.as_str(), event.reason))
        .collect::<BTreeMap<_, _>>();
    let linked_object_summaries = linked_object_state_rows
        .iter()
        .map(|row| {
            let mut summary = ProviderLinkedObjectSupportSummary::from(row);
            summary.deny_reason = row
                .deny_event_ref
                .as_deref()
                .and_then(|deny_event_ref| deny_reasons.get(deny_event_ref).copied());
            summary
        })
        .collect();

    ProviderEventIngestionSupportExport {
        record_kind: PROVIDER_EVENT_INGESTION_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: PROVIDER_EVENT_INGESTION_SCHEMA_VERSION,
        packet_id: "provider-event-ingestion:m5:0001".to_string(),
        linked_object_summaries,
        raw_provider_payload_export_allowed: false,
        raw_callback_url_export_allowed: false,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn seeded_linked_object_state_rows() -> Vec<ProviderLinkedObjectStateRow> {
    vec![
        linked_object_state_row(
            "linked-object:pr-comment:fresh",
            "provider.object.pr.42.comment.7",
            Some("provider_event_ingestion.event.pr_comment.imported"),
            "provider_event.event.pr_comment.applied",
            Some("provider_import.session.pr_comment.full"),
            "provider_replay.ledger.pr_comment",
            None,
            None,
            ProviderEventAuthoritySourceClass::ConnectedAccount,
            ProviderEventSourceClass::Webhook,
            ProviderEventTypeClass::CommentCreated,
            FreshnessLabel::Fresh,
            TruthCompletenessClass::FullSnapshot,
            ProviderLinkedObjectStateClass::Fresh,
            EventDispositionClass::AppliedOnce,
            "Review comment is fresh and provider-committed for the declared scope.",
        ),
        linked_object_state_row(
            "linked-object:issue-84:partial",
            "provider.object.issue.84",
            Some("provider_event_ingestion.event.issue.buffered"),
            "provider_event.event.issue.partial",
            Some("provider_import.session.issue.partial"),
            "provider_replay.ledger.issue.partial",
            None,
            None,
            ProviderEventAuthoritySourceClass::InstallationGrant,
            ProviderEventSourceClass::BrowserReturnCallback,
            ProviderEventTypeClass::StatusTransition,
            FreshnessLabel::Fresh,
            TruthCompletenessClass::BoundedPartialSnapshot,
            ProviderLinkedObjectStateClass::Partial,
            EventDispositionClass::PartialImportApplied,
            "Issue detail shows explicit partial truth because a comments page remains omitted.",
        ),
        linked_object_state_row(
            "linked-object:issue-87:delayed",
            "provider.object.issue.87",
            Some("provider_event_ingestion.event.issue.polling"),
            "provider_event.event.issue.delayed",
            Some("provider_import.session.issue.delayed"),
            "provider_replay.ledger.issue.delayed",
            None,
            None,
            ProviderEventAuthoritySourceClass::ConnectedAccount,
            ProviderEventSourceClass::PollingRefresh,
            ProviderEventTypeClass::StatusTransition,
            FreshnessLabel::StaleWithinWindow,
            TruthCompletenessClass::DelayedDelivery,
            ProviderLinkedObjectStateClass::Delayed,
            EventDispositionClass::FreshnessRefreshedOnly,
            "Polling refresh arrived after the freshness floor and remains labeled delayed.",
        ),
        linked_object_state_row(
            "linked-object:issue-86:backfilled",
            "provider.object.issue.86",
            None,
            "provider_event.event.issue.backfilled",
            Some("provider_import.session.issue.backfilled"),
            "provider_replay.ledger.issue.backfilled",
            None,
            None,
            ProviderEventAuthoritySourceClass::InstallationGrant,
            ProviderEventSourceClass::ImportSession,
            ProviderEventTypeClass::ImportPageBackfilled,
            FreshnessLabel::StaleWithinWindow,
            TruthCompletenessClass::BackfilledSnapshot,
            ProviderLinkedObjectStateClass::Backfilled,
            EventDispositionClass::AppliedOnce,
            "Historical issue state was backfilled explicitly and remains labeled as such.",
        ),
        linked_object_state_row(
            "linked-object:check-510:mirror",
            "provider.object.check.510",
            Some("provider_event_ingestion.event.check.stale"),
            "provider_event.event.check.mirror",
            Some("provider_import.session.check.mirror"),
            "provider_replay.ledger.check.mirror",
            None,
            None,
            ProviderEventAuthoritySourceClass::InstallationGrant,
            ProviderEventSourceClass::MirrorSync,
            ProviderEventTypeClass::CheckStateChanged,
            FreshnessLabel::ExpiredBeyondWindow,
            TruthCompletenessClass::MirrorDerivedSnapshot,
            ProviderLinkedObjectStateClass::MirrorDerived,
            EventDispositionClass::AppliedOnce,
            "Check state came from a mirror and never upgrades itself to live-provider truth.",
        ),
        linked_object_state_row(
            "linked-object:issue-84:stale",
            "provider.object.issue.84",
            Some("provider_event_ingestion.event.publish_later.conflict"),
            "provider_event.event.publish_later.blocked",
            Some("provider_import.session.issue.latest"),
            "provider_replay.ledger.publish.blocked",
            Some("provider_reconcile.result.issue.84.drift"),
            None,
            ProviderEventAuthoritySourceClass::ConnectedAccount,
            ProviderEventSourceClass::DeferredPublishQueue,
            ProviderEventTypeClass::PublishDrainResult,
            FreshnessLabel::StaleWithinWindow,
            TruthCompletenessClass::FullSnapshot,
            ProviderLinkedObjectStateClass::Stale,
            EventDispositionClass::PublishBlockedDrift,
            "Queued publish is stale against the latest provider snapshot and requires compare review.",
        ),
        linked_object_state_row(
            "linked-object:pr-42:callback-denied",
            "provider.object.pr.42",
            Some("provider_event_ingestion.event.callback.denied"),
            "provider_event.event.callback.denied",
            None,
            "provider_replay.ledger.callback.denied",
            None,
            Some("provider_callback_deny.event.host_mismatch"),
            ProviderEventAuthoritySourceClass::InstallationGrant,
            ProviderEventSourceClass::BrowserReturnCallback,
            ProviderEventTypeClass::CallbackDenied,
            FreshnessLabel::Fresh,
            TruthCompletenessClass::NoStateImported,
            ProviderLinkedObjectStateClass::CallbackDenied,
            EventDispositionClass::DeniedNoMutation,
            "Callback deny stayed audit-only because the provider host proof failed.",
        ),
    ]
}

fn linked_object_state_row(
    row_id: &str,
    canonical_local_object_ref: &str,
    provenance_event_ref: Option<&str>,
    reconciliation_event_ref: &str,
    import_session_ref: Option<&str>,
    replay_ledger_item_ref: &str,
    reconciliation_result_ref: Option<&str>,
    deny_event_ref: Option<&str>,
    authority_source_class: ProviderEventAuthoritySourceClass,
    source_class: ProviderEventSourceClass,
    event_type: ProviderEventTypeClass,
    freshness_class: FreshnessLabel,
    truth_class: TruthCompletenessClass,
    linked_state: ProviderLinkedObjectStateClass,
    final_disposition: EventDispositionClass,
    summary_label: &str,
) -> ProviderLinkedObjectStateRow {
    ProviderLinkedObjectStateRow {
        record_kind: PROVIDER_LINKED_OBJECT_STATE_ROW_RECORD_KIND.to_string(),
        row_id: row_id.to_string(),
        canonical_local_object_ref: canonical_local_object_ref.to_string(),
        provider_descriptor_ref: "provider.descriptor.code_host.primary".to_string(),
        provider_family: ProviderFamily::CodeHost,
        object_kind: ProviderObjectKind::IssueOrWorkItem,
        provenance_event_ref: provenance_event_ref.map(ToOwned::to_owned),
        reconciliation_event_ref: reconciliation_event_ref.to_string(),
        import_session_ref: import_session_ref.map(ToOwned::to_owned),
        replay_ledger_item_ref: replay_ledger_item_ref.to_string(),
        reconciliation_result_ref: reconciliation_result_ref.map(ToOwned::to_owned),
        deny_event_ref: deny_event_ref.map(ToOwned::to_owned),
        authority_source_class,
        source_class,
        event_type,
        freshness_class,
        truth_class,
        linked_state,
        final_disposition,
        summary_label: summary_label.to_string(),
    }
}

fn seeded_consumer_projections(
    linked_object_state_rows: &[ProviderLinkedObjectStateRow],
) -> Vec<ProviderEventIngestionConsumerProjectionRow> {
    linked_object_state_rows
        .iter()
        .flat_map(|row| {
            consumer_surfaces_for_state(row.linked_state)
                .into_iter()
                .map(move |surface| ProviderEventIngestionConsumerProjectionRow {
                    record_kind: PROVIDER_EVENT_INGESTION_CONSUMER_PROJECTION_RECORD_KIND
                        .to_string(),
                    row_id: format!("projection:{}:{}", surface_token(surface), row.row_id),
                    surface,
                    linked_object_state_row_ref: row.row_id.clone(),
                    canonical_local_object_ref: row.canonical_local_object_ref.clone(),
                    linked_state: row.linked_state,
                    summary_label: format!(
                        "{} projects {} as {}",
                        surface_token(surface),
                        row.canonical_local_object_ref,
                        row.summary_label
                    ),
                })
        })
        .collect()
}

fn consumer_surfaces_for_state(
    state: ProviderLinkedObjectStateClass,
) -> Vec<ProviderEventIngestionConsumerSurface> {
    match state {
        ProviderLinkedObjectStateClass::Fresh => vec![
            ProviderEventIngestionConsumerSurface::WorkItemDetail,
            ProviderEventIngestionConsumerSurface::ReviewWorkspace,
            ProviderEventIngestionConsumerSurface::SupportExport,
            ProviderEventIngestionConsumerSurface::DocsHelp,
        ],
        ProviderLinkedObjectStateClass::Partial
        | ProviderLinkedObjectStateClass::Delayed
        | ProviderLinkedObjectStateClass::Backfilled
        | ProviderLinkedObjectStateClass::Stale
        | ProviderLinkedObjectStateClass::MirrorDerived => vec![
            ProviderEventIngestionConsumerSurface::WorkItemDetail,
            ProviderEventIngestionConsumerSurface::ReviewWorkspace,
            ProviderEventIngestionConsumerSurface::CliHeadless,
            ProviderEventIngestionConsumerSurface::CompanionTriage,
            ProviderEventIngestionConsumerSurface::SupportExport,
            ProviderEventIngestionConsumerSurface::DocsHelp,
            ProviderEventIngestionConsumerSurface::AuditTimeline,
        ],
        ProviderLinkedObjectStateClass::CallbackDenied => vec![
            ProviderEventIngestionConsumerSurface::WorkItemDetail,
            ProviderEventIngestionConsumerSurface::SupportExport,
            ProviderEventIngestionConsumerSurface::DocsHelp,
            ProviderEventIngestionConsumerSurface::AuditTimeline,
        ],
    }
}

fn surface_token(surface: ProviderEventIngestionConsumerSurface) -> &'static str {
    match surface {
        ProviderEventIngestionConsumerSurface::WorkItemDetail => "work_item_detail",
        ProviderEventIngestionConsumerSurface::ReviewWorkspace => "review_workspace",
        ProviderEventIngestionConsumerSurface::CliHeadless => "cli_headless",
        ProviderEventIngestionConsumerSurface::CompanionTriage => "companion_triage",
        ProviderEventIngestionConsumerSurface::SupportExport => "support_export",
        ProviderEventIngestionConsumerSurface::DocsHelp => "docs_help",
        ProviderEventIngestionConsumerSurface::AuditTimeline => "audit_timeline",
    }
}
