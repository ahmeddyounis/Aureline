//! Export-safe search packets, redaction defaults, replay-safety, and
//! literal-query consent gates for the M5 search and incident support lanes.
//!
//! Search and incident support flows routinely carry sensitive query text,
//! customer identifiers, hostnames, and policy terms. Where
//! [`crate::session_ledger`] owns the per-export [`SearchExportPacket`] and the
//! privacy vocabulary, and [`crate::saved_query_governance`] freezes the durable
//! saved-query artifacts, this module freezes the *export and replay* posture
//! into one delivery-grade packet so supportability never quietly broadens
//! retention of raw query text or result bodies:
//!
//! - [`SearchExportRow`] binds one canonical [`SearchExportPacket`] verbatim —
//!   its query-session ref, selected and included result refs, loaded/hidden
//!   counts, redaction mode, snapshot truth, and evidence refs — to the export
//!   class it ships under and the literal-query consent gate that governs it.
//! - [`SearchExportClass`] names the trust tier an export ships under
//!   (local replay, redacted support bundle, incident packet, managed
//!   analytics). Only a local-replay packet ever retains literal query text, and
//!   only under explicit [`ExportConsentClass::QueryTextElevated`] consent;
//!   everything that leaves the device defaults to hashes, scope summaries,
//!   result refs, omission counts, and reason summaries.
//! - [`ReplaySafetyDisclosure`] proves a packet preserves *intent and
//!   provenance* without claiming live current results: it is always a captured
//!   snapshot or a disclosed scope drift, never a live rerun, and a drifted scope
//!   requires a rerun before any current-truth claim.
//!
//! The [`SearchExportGovernancePacket`] proves replay/debug tooling, support
//! exports, incident packets, and managed analytics read the *same* export
//! packets and the *same* privacy rules across desktop, CLI/headless, support
//! export, and managed-analytics consumers ([`SearchExportConsumerClass`])
//! without one path silently widening what it retains. Raw query text stays
//! confined to local-only replay packets, and
//! [`SearchExportGovernancePacket::redact_for_export`] materializes the redacted
//! copy a support bundle or incident packet ships.
//!
//! [`SearchExportPacket`]: crate::session_ledger::SearchExportPacket
//! [`ExportConsentClass`]: crate::ranking_explainability::ExportConsentClass

use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::lexical::ScopeClass;
use crate::query_artifacts::{SearchResultSemantics, SearchScopeHonestyState};
use crate::query_session::{QueryTextMode, SearchQuerySession, SearchSurface};
use crate::ranking_explainability::ExportConsentClass;
use crate::session_ledger::{
    SavedQueryPrivacyClass, SearchExportDestination, SearchExportPacket, SearchExportSnapshotTruth,
    SearchPacketCountSummary, SearchPacketRedactionState, SAVED_QUERY_ALPHA_SCHEMA_VERSION,
};

/// Stable record-kind tag for [`SearchExportGovernancePacket`].
pub const SEARCH_EXPORT_GOVERNANCE_PACKET_RECORD_KIND: &str = "search_export_governance_packet";

/// Stable record-kind tag for [`SearchExportGovernanceSupportExport`].
pub const SEARCH_EXPORT_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "search_export_governance_support_export";

/// Integer schema version for the search-export governance packet.
pub const SEARCH_EXPORT_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable packet identifier reused by every consumer projection.
pub const SEARCH_EXPORT_GOVERNANCE_PACKET_ID: &str = "search.m5.search_export_governance.v1";

/// Repository-relative path of the boundary schema.
pub const SEARCH_EXPORT_GOVERNANCE_SCHEMA_REF: &str =
    "schemas/search/search-export-packet.schema.json";

/// Repository-relative path of the reviewer-facing contract document.
pub const SEARCH_EXPORT_GOVERNANCE_DOC_REF: &str = "docs/search/search-export-packet.md";

/// Repository-relative path of the checked review artifact.
pub const SEARCH_EXPORT_GOVERNANCE_ARTIFACT_REF: &str =
    "artifacts/search/m5/search-export-packet.md";

/// Repository-relative path of the protected fixture directory.
pub const SEARCH_EXPORT_GOVERNANCE_FIXTURE_DIR: &str = "fixtures/search/m5/support-export";

/// Fixed generation timestamp for the seeded corpus.
const SEEDED_GENERATED_AT: &str = "2026-06-17T00:00:00Z";

/// Export classes the packet realizes, in canonical order.
pub const ALL_EXPORT_CLASSES: [SearchExportClass; 4] = [
    SearchExportClass::LocalReplay,
    SearchExportClass::SupportBundle,
    SearchExportClass::IncidentPacket,
    SearchExportClass::ManagedAnalytics,
];

/// Packet redaction states the packet realizes, in canonical order.
pub const ALL_REDACTION_STATES: [SearchPacketRedactionState; 3] = [
    SearchPacketRedactionState::RawQueryLocalOnly,
    SearchPacketRedactionState::QueryHashOnly,
    SearchPacketRedactionState::QueryMaterialOmittedByPolicy,
];

/// Literal-query consent classes the packet realizes, in canonical order.
pub const ALL_CONSENT_CLASSES: [ExportConsentClass; 2] = [
    ExportConsentClass::MetadataOnly,
    ExportConsentClass::QueryTextElevated,
];

/// Trust tier an export packet ships under.
///
/// The class fixes the default redaction posture and whether literal query text
/// may be retained at all: only [`SearchExportClass::LocalReplay`] ever keeps the
/// literal, and then only under [`ExportConsentClass::QueryTextElevated`]
/// consent. Every class that leaves the device defaults to hashes, scope
/// summaries, result refs, omission counts, and reason summaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchExportClass {
    /// On-device replay/debug packet; may retain literal query text under consent.
    LocalReplay,
    /// Redacted support-bundle packet; hashes, refs, and counts only.
    SupportBundle,
    /// Incident/escalation packet joined to support; hashes, refs, and counts only.
    IncidentPacket,
    /// Managed analytics path; metadata only and never literal or hash material.
    ManagedAnalytics,
}

impl SearchExportClass {
    /// Every export class, in canonical order.
    pub const ALL: [Self; 4] = ALL_EXPORT_CLASSES;

    /// Stable snake-case token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalReplay => "local_replay",
            Self::SupportBundle => "support_bundle",
            Self::IncidentPacket => "incident_packet",
            Self::ManagedAnalytics => "managed_analytics",
        }
    }

    /// Review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalReplay => "Local replay",
            Self::SupportBundle => "Support bundle",
            Self::IncidentPacket => "Incident packet",
            Self::ManagedAnalytics => "Managed analytics",
        }
    }

    /// True when the packet leaves the device under this class.
    pub const fn leaves_device(self) -> bool {
        !matches!(self, Self::LocalReplay)
    }

    /// True when this class may retain literal query text (only under elevated
    /// consent). Literal text is never retained by a class that leaves the device.
    pub const fn permits_literal_query_text(self) -> bool {
        matches!(self, Self::LocalReplay)
    }

    /// True for the higher-trust export classes whose literal inclusion is gated
    /// behind explicit elevated consent before anything may leave the device.
    pub const fn is_higher_trust(self) -> bool {
        matches!(self, Self::SupportBundle | Self::IncidentPacket)
    }

    /// True when this class never carries even hashed query material.
    pub const fn forbids_all_query_material(self) -> bool {
        matches!(self, Self::ManagedAnalytics)
    }

    /// Destination class of the embedded [`SearchExportPacket`].
    pub const fn export_destination(self) -> SearchExportDestination {
        match self {
            Self::LocalReplay => SearchExportDestination::LocalReplay,
            Self::SupportBundle | Self::IncidentPacket | Self::ManagedAnalytics => {
                SearchExportDestination::SupportBundle
            }
        }
    }
}

/// Consumer that ingests the governed export packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchExportConsumerClass {
    /// Desktop search/support chrome and replay surfaces.
    DesktopShell,
    /// CLI/headless inspect and replay tooling.
    CliHeadless,
    /// Redacted support-bundle and incident export.
    SupportExport,
    /// Managed analytics ingestion path.
    ManagedAnalytics,
}

impl SearchExportConsumerClass {
    /// Every consumer, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::DesktopShell,
        Self::CliHeadless,
        Self::SupportExport,
        Self::ManagedAnalytics,
    ];

    /// Stable snake-case token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopShell => "desktop_shell",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::ManagedAnalytics => "managed_analytics",
        }
    }

    /// Review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DesktopShell => "Desktop shell",
            Self::CliHeadless => "CLI / headless",
            Self::SupportExport => "Support export",
            Self::ManagedAnalytics => "Managed analytics",
        }
    }
}

/// Replay-safety truth attached to one export packet.
///
/// A replay-safe packet preserves search *intent* and *provenance* without
/// claiming live current results: it is always a captured snapshot or a disclosed
/// scope drift, never a live rerun, and a drift requires a rerun before any
/// current-truth claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaySafetyDisclosure {
    /// Live-vs-captured semantics; never `current_live_results`.
    pub result_semantics: SearchResultSemantics,
    /// Captured-vs-current scope honesty state.
    pub scope_honesty_state: SearchScopeHonestyState,
    /// Snapshot truth carried by the embedded packet; never `live_rerun`.
    pub snapshot_truth: SearchExportSnapshotTruth,
    /// True when a live rerun is required before claiming current truth.
    pub rerun_required_for_current_truth: bool,
    /// Always `true`: the packet preserves intent and provenance for replay.
    pub preserves_intent_and_provenance: bool,
    /// Always `false`: a replay packet never claims live current results.
    pub claims_live_current_results: bool,
    /// User-visible replay-safety disclosure.
    pub disclosure: String,
}

impl ReplaySafetyDisclosure {
    /// True when the captured scope drifted since the packet was captured.
    pub fn scope_drifted(&self) -> bool {
        matches!(
            self.snapshot_truth,
            SearchExportSnapshotTruth::ScopeChangedSinceCapture
        ) || matches!(
            self.result_semantics,
            SearchResultSemantics::ScopeChangedSinceCapture
                | SearchResultSemantics::EmptyBecauseScopeChanged
        )
    }
}

/// One governed export row binding a canonical [`SearchExportPacket`] to the
/// export class it ships under, the literal-query consent gate, and its
/// replay-safety truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchExportRow {
    /// Stable row identity.
    pub row_id: String,
    /// Reviewable scenario summary.
    pub scenario: String,
    /// Trust tier this packet ships under.
    pub export_class: SearchExportClass,
    /// Literal-query consent posture applied to this export.
    pub literal_query_consent: ExportConsentClass,
    /// True when the export retains literal query text.
    pub literal_query_text_included: bool,
    /// Always `true`: literal inclusion is gated behind a higher-trust class.
    pub higher_trust_export_class_required_for_literal: bool,
    /// Canonical export packet, reused verbatim from the session ledger.
    pub export_packet: SearchExportPacket,
    /// Replay-safety truth for this packet.
    pub replay_safety: ReplaySafetyDisclosure,
    /// Reviewable summary of the governed row.
    pub summary: String,
}

impl SearchExportRow {
    /// True when this row carries literal query text confined to a local-only
    /// replay packet, or carries no literal query text at all.
    pub fn literal_query_text_is_local_only(&self) -> bool {
        self.export_packet.query_text.is_none()
            || (self.export_class == SearchExportClass::LocalReplay
                && !self.export_class.leaves_device())
    }
}

/// Consumer projection that reuses the governed export packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchExportConsumerProjection {
    /// Consumer that ingests the packet.
    pub consumer: SearchExportConsumerClass,
    /// Repository-relative pointer to the consumer.
    pub consumer_ref: String,
    /// Packet id the consumer ingests verbatim.
    pub ingested_packet_id: String,
    /// True when the consumer preserves the per-packet redaction mode.
    pub preserves_redaction_mode: bool,
    /// True when the consumer preserves loaded/hidden/omitted count disclosure.
    pub preserves_count_and_omission_disclosure: bool,
    /// True when the consumer preserves replay-safety (never claims live results).
    pub preserves_replay_safety: bool,
    /// True when the consumer reuses the same export packets, not raw UI state.
    pub reuses_same_export_packets: bool,
    /// True when literal query text is excluded from this projection.
    pub literal_query_text_excluded: bool,
    /// True when ambient credentials and authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Reviewable summary of how the consumer reuses the packets.
    pub summary: String,
}

/// One validation finding emitted by [`SearchExportGovernancePacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchExportGovernanceValidationFinding {
    /// Field or collection path that failed validation.
    pub path: String,
    /// Reviewable explanation of the failure.
    pub message: String,
}

/// Delivery-grade governance packet for export-safe search packets, redaction
/// defaults, replay safety, and literal-query consent gates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchExportGovernancePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet id reused by every consumer projection.
    pub packet_id: String,
    /// RFC 3339 UTC generation time.
    pub generated_at: String,
    /// Reviewer-facing contract document ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Checked review artifact ref.
    pub artifact_ref: String,
    /// Authoritative spec sections the packet answers to.
    pub source_spec_refs: Vec<String>,
    /// Existing lane schemas the packet composes.
    pub supporting_contract_refs: Vec<String>,
    /// Export classes the packet realizes.
    pub covered_export_classes: Vec<SearchExportClass>,
    /// Packet redaction states the packet realizes.
    pub covered_redaction_states: Vec<SearchPacketRedactionState>,
    /// Literal-query consent classes the packet realizes.
    pub covered_consent_classes: Vec<ExportConsentClass>,
    /// Governed export rows, one per export class.
    pub export_rows: Vec<SearchExportRow>,
    /// Consumer projections that reuse the governed packets.
    pub consumer_projections: Vec<SearchExportConsumerProjection>,
    /// Reviewable summary of the export and privacy posture.
    pub export_safe_summary: String,
}

impl SearchExportGovernancePacket {
    /// Returns the governed export row for one id, if present.
    pub fn export_row(&self, row_id: &str) -> Option<&SearchExportRow> {
        self.export_rows.iter().find(|row| row.row_id == row_id)
    }

    /// Export classes realized across the export rows.
    pub fn present_export_classes(&self) -> HashSet<SearchExportClass> {
        self.export_rows
            .iter()
            .map(|row| row.export_class)
            .collect()
    }

    /// Packet redaction states realized across the export rows.
    pub fn present_redaction_states(&self) -> HashSet<SearchPacketRedactionState> {
        self.export_rows
            .iter()
            .map(|row| row.export_packet.redaction_state)
            .collect()
    }

    /// Literal-query consent classes realized across the export rows.
    pub fn present_consent_classes(&self) -> HashSet<ExportConsentClass> {
        self.export_rows
            .iter()
            .map(|row| row.literal_query_consent)
            .collect()
    }

    /// True when every export row confines literal query text to a local-only
    /// replay packet; nothing that leaves the device carries the literal.
    pub fn literal_query_text_is_local_only(&self) -> bool {
        self.export_rows
            .iter()
            .all(SearchExportRow::literal_query_text_is_local_only)
    }

    /// True when the packet carries no literal query text at all (the posture of
    /// the redacted export copy a support bundle or incident packet ships).
    pub fn contains_no_literal_query_text(&self) -> bool {
        self.export_rows
            .iter()
            .all(|row| row.export_packet.query_text.is_none())
    }

    /// True when the packet is safe to project to support, incident, and managed
    /// consumers: it validates, confines literal text to local-only replay, and
    /// no consumer widens authority or carries literal text.
    pub fn is_export_safe(&self) -> bool {
        self.validate().is_empty()
            && self.literal_query_text_is_local_only()
            && self.consumer_projections.iter().all(|projection| {
                projection.literal_query_text_excluded && projection.ambient_authority_excluded
            })
    }

    /// Returns a redacted copy with all literal query text removed, as carried by
    /// a support bundle or incident packet. Hashes, scope metadata, result refs,
    /// counts, and replay-safety truth are preserved.
    pub fn redact_for_export(&self) -> Self {
        let mut redacted = self.clone();
        for row in &mut redacted.export_rows {
            if row.export_packet.query_text.is_some() {
                row.export_packet.query_text = None;
                if row.export_packet.query_text_mode == QueryTextMode::LocalText {
                    row.export_packet.query_text_mode = QueryTextMode::HashOnly;
                }
                if row.export_packet.redaction_state
                    == SearchPacketRedactionState::RawQueryLocalOnly
                {
                    row.export_packet.redaction_state = if row.export_packet.query_hash.is_some() {
                        SearchPacketRedactionState::QueryHashOnly
                    } else {
                        SearchPacketRedactionState::QueryMaterialOmittedByPolicy
                    };
                }
                row.literal_query_text_included = false;
                row.literal_query_consent = ExportConsentClass::MetadataOnly;
            }
        }
        redacted
    }

    /// Builds a redacted support export that wraps the redacted packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> SearchExportGovernanceSupportExport {
        let redacted_packet = self.redact_for_export();
        SearchExportGovernanceSupportExport {
            record_kind: SEARCH_EXPORT_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SEARCH_EXPORT_GOVERNANCE_SCHEMA_VERSION,
            export_id: export_id.into(),
            packet_id_ref: redacted_packet.packet_id.clone(),
            exported_at: exported_at.into(),
            literal_query_text_excluded: true,
            ambient_authority_excluded: true,
            redacted_packet,
        }
    }

    /// Validates the packet against the search-export governance guardrails.
    ///
    /// An empty result means every export packet is export-safe, literal query
    /// text stays confined to consented local-only replay packets, every packet
    /// is replay-safe, count and omission disclosure is preserved, and every
    /// consumer reads the same packets under the same privacy rules.
    pub fn validate(&self) -> Vec<SearchExportGovernanceValidationFinding> {
        let mut findings = Vec::new();

        if self.record_kind != SEARCH_EXPORT_GOVERNANCE_PACKET_RECORD_KIND {
            push(&mut findings, "record_kind", "unexpected record_kind");
        }
        if self.schema_version != SEARCH_EXPORT_GOVERNANCE_SCHEMA_VERSION {
            push(&mut findings, "schema_version", "unexpected schema_version");
        }
        if self.packet_id != SEARCH_EXPORT_GOVERNANCE_PACKET_ID {
            push(&mut findings, "packet_id", "unexpected packet_id");
        }
        if self.generated_at.trim().is_empty() {
            push(&mut findings, "generated_at", "generated_at is required");
        }
        if self.doc_ref != SEARCH_EXPORT_GOVERNANCE_DOC_REF {
            push(&mut findings, "doc_ref", "unexpected doc_ref");
        }
        if self.schema_ref != SEARCH_EXPORT_GOVERNANCE_SCHEMA_REF {
            push(&mut findings, "schema_ref", "unexpected schema_ref");
        }
        if self.artifact_ref != SEARCH_EXPORT_GOVERNANCE_ARTIFACT_REF {
            push(&mut findings, "artifact_ref", "unexpected artifact_ref");
        }
        if self.source_spec_refs.is_empty() {
            push(
                &mut findings,
                "source_spec_refs",
                "source_spec_refs is required",
            );
        }
        if self.supporting_contract_refs.is_empty() {
            push(
                &mut findings,
                "supporting_contract_refs",
                "supporting_contract_refs is required",
            );
        }

        self.validate_coverage(&mut findings);
        self.validate_export_rows(&mut findings);
        self.validate_consumers(&mut findings);

        findings
    }

    fn validate_coverage(&self, findings: &mut Vec<SearchExportGovernanceValidationFinding>) {
        if self.covered_export_classes != ALL_EXPORT_CLASSES.to_vec() {
            push(
                findings,
                "covered_export_classes",
                "covered_export_classes must list every export class in canonical order",
            );
        }
        if self.covered_redaction_states != ALL_REDACTION_STATES.to_vec() {
            push(
                findings,
                "covered_redaction_states",
                "covered_redaction_states must list every redaction state in canonical order",
            );
        }
        if self.covered_consent_classes != ALL_CONSENT_CLASSES.to_vec() {
            push(
                findings,
                "covered_consent_classes",
                "covered_consent_classes must list every consent class in canonical order",
            );
        }

        let present = self.present_export_classes();
        for required in ALL_EXPORT_CLASSES {
            if !present.contains(&required) {
                push(
                    findings,
                    "export_rows",
                    &format!("no row realizes export class {}", required.as_str()),
                );
            }
        }
    }

    fn validate_export_rows(&self, findings: &mut Vec<SearchExportGovernanceValidationFinding>) {
        if self.export_rows.is_empty() {
            push(
                findings,
                "export_rows",
                "at least one governed export row is required",
            );
        }
        for row in &self.export_rows {
            let base = format!("export_rows.{}", row.row_id);

            // The embedded export packet must itself be export-safe: non-local
            // destinations carry no raw text, policy-withheld carries no hash, and
            // partial/omitted packets preserve their disclosure flags.
            for finding in row.export_packet.validate_export_safe() {
                push(findings, &format!("{base}.export_packet"), &finding.summary);
            }
            if row.export_packet.destination != row.export_class.export_destination() {
                push(
                    findings,
                    &format!("{base}.export_packet"),
                    "embedded packet destination must match the export class",
                );
            }

            // Literal-query consent gate.
            if !row.higher_trust_export_class_required_for_literal {
                push(
                    findings,
                    &format!("{base}.higher_trust_export_class_required_for_literal"),
                    "literal inclusion must always require a higher-trust export class",
                );
            }
            let packet_has_literal = row.export_packet.query_text.is_some();
            if row.literal_query_text_included != packet_has_literal {
                push(
                    findings,
                    &format!("{base}.literal_query_text_included"),
                    "literal_query_text_included must match the embedded packet's query text",
                );
            }
            if row.literal_query_text_included {
                if row.literal_query_consent != ExportConsentClass::QueryTextElevated {
                    push(
                        findings,
                        &format!("{base}.literal_query_consent"),
                        "literal query text requires explicit elevated consent",
                    );
                }
                if !row.export_class.permits_literal_query_text() {
                    push(
                        findings,
                        &format!("{base}.export_class"),
                        "this export class must not retain literal query text",
                    );
                }
            }
            // Nothing that leaves the device may carry literal query text.
            if row.export_class.leaves_device() && packet_has_literal {
                push(
                    findings,
                    &format!("{base}.export_packet"),
                    "literal query text must never leave the device",
                );
            }
            // Managed analytics never carries any query material.
            if row.export_class.forbids_all_query_material()
                && (packet_has_literal || row.export_packet.query_hash.is_some())
            {
                push(
                    findings,
                    &format!("{base}.export_packet"),
                    "managed analytics exports must not carry literal or hash query material",
                );
            }
            if !row.literal_query_text_is_local_only() {
                push(
                    findings,
                    &format!("{base}.export_packet"),
                    "literal query text must stay confined to a local-only replay packet",
                );
            }

            // Replay safety: preserve intent and provenance, never claim live.
            let replay = &row.replay_safety;
            if replay.claims_live_current_results {
                push(
                    findings,
                    &format!("{base}.replay_safety"),
                    "a replay packet must never claim live current results",
                );
            }
            if !replay.preserves_intent_and_provenance {
                push(
                    findings,
                    &format!("{base}.replay_safety"),
                    "a replay packet must preserve intent and provenance",
                );
            }
            if matches!(
                replay.result_semantics,
                SearchResultSemantics::CurrentLiveResults
            ) {
                push(
                    findings,
                    &format!("{base}.replay_safety"),
                    "replay semantics must not claim current live results",
                );
            }
            if matches!(replay.snapshot_truth, SearchExportSnapshotTruth::LiveRerun) {
                push(
                    findings,
                    &format!("{base}.replay_safety"),
                    "a replay packet must be a captured snapshot, never a live rerun",
                );
            }
            if replay.snapshot_truth != row.export_packet.snapshot_truth {
                push(
                    findings,
                    &format!("{base}.replay_safety"),
                    "replay snapshot truth must match the embedded packet",
                );
            }
            if replay.scope_drifted() && !replay.rerun_required_for_current_truth {
                push(
                    findings,
                    &format!("{base}.replay_safety"),
                    "a drifted scope must require a rerun before claiming current truth",
                );
            }

            // Count and omission disclosure must survive into the packet.
            let counts = &row.export_packet.count_summary;
            if (counts.omitted_result_count > 0 || counts.count_is_partial)
                && row.export_packet.omitted_or_truncated_flags.is_empty()
            {
                push(
                    findings,
                    &format!("{base}.export_packet"),
                    "partial or omitted packets must preserve omitted/truncated flags",
                );
            }
            if row.export_packet.evidence_refs.is_empty() {
                push(
                    findings,
                    &format!("{base}.export_packet"),
                    "export packets must carry evidence refs for replay and audit",
                );
            }
            if row.export_packet.query_session_id_ref.trim().is_empty() {
                push(
                    findings,
                    &format!("{base}.export_packet"),
                    "export packets must preserve the query-session ref",
                );
            }
        }
    }

    fn validate_consumers(&self, findings: &mut Vec<SearchExportGovernanceValidationFinding>) {
        for required in SearchExportConsumerClass::ALL {
            if !self
                .consumer_projections
                .iter()
                .any(|projection| projection.consumer == required)
            {
                push(
                    findings,
                    "consumer_projections",
                    &format!("missing consumer {}", required.as_str()),
                );
            }
        }
        for projection in &self.consumer_projections {
            let base = format!("consumer_projections.{}", projection.consumer.as_str());
            if projection.ingested_packet_id != self.packet_id {
                push(
                    findings,
                    &base,
                    "consumer must ingest the packet id verbatim",
                );
            }
            if !projection.preserves_redaction_mode {
                push(findings, &base, "consumer must preserve the redaction mode");
            }
            if !projection.preserves_count_and_omission_disclosure {
                push(
                    findings,
                    &base,
                    "consumer must preserve count and omission disclosure",
                );
            }
            if !projection.preserves_replay_safety {
                push(findings, &base, "consumer must preserve replay safety");
            }
            if !projection.reuses_same_export_packets {
                push(
                    findings,
                    &base,
                    "consumer must reuse the same export packets, not raw UI state",
                );
            }
            if !projection.literal_query_text_excluded {
                push(findings, &base, "consumer must exclude literal query text");
            }
            if !projection.ambient_authority_excluded {
                push(findings, &base, "consumer must exclude ambient authority");
            }
        }
    }
}

/// Redacted support export that wraps the redacted governance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchExportGovernanceSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Product packet id preserved by the export.
    pub packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when literal query text is excluded.
    pub literal_query_text_excluded: bool,
    /// True when ambient credentials and authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Redacted packet preserved by the export.
    pub redacted_packet: SearchExportGovernancePacket,
}

impl SearchExportGovernanceSupportExport {
    /// True when the export preserves the packet safely with no literal query text.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == SEARCH_EXPORT_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == SEARCH_EXPORT_GOVERNANCE_SCHEMA_VERSION
            && self.packet_id_ref == self.redacted_packet.packet_id
            && self.literal_query_text_excluded
            && self.ambient_authority_excluded
            && self.redacted_packet.validate().is_empty()
            && self.redacted_packet.contains_no_literal_query_text()
    }
}

/// Errors returned when reading the checked-in governance packet.
#[derive(Debug)]
pub enum SearchExportGovernanceArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<SearchExportGovernanceValidationFinding>),
}

impl fmt::Display for SearchExportGovernanceArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(
                    formatter,
                    "search-export governance packet parse failed: {error}"
                )
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.path.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "search-export governance packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for SearchExportGovernanceArtifactError {}

/// Returns the checked-in canonical governance packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_search_export_governance_packet(
) -> Result<SearchExportGovernancePacket, SearchExportGovernanceArtifactError> {
    let packet: SearchExportGovernancePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/search/m5/support-export/packet.json"
    )))
    .map_err(SearchExportGovernanceArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(SearchExportGovernanceArtifactError::Validation(findings))
    }
}

/// Returns the canonical seeded search-export governance packet.
pub fn seeded_search_export_governance_packet() -> SearchExportGovernancePacket {
    let export_rows = vec![
        build_export_row(ExportRowSeed {
            row_id: "export:local-replay",
            scenario: "Local replay/debug capture of a quick-open pass.",
            export_class: SearchExportClass::LocalReplay,
            consent: ExportConsentClass::QueryTextElevated,
            surface: SearchSurface::QuickOpen,
            query_text: "retry budget",
            scope_class: ScopeClass::CurrentRepo,
            scope_label: "Current repo",
            readiness_state: "hot_set_ready",
            drifted: false,
            visible_rows: 5,
            included: &["workspace:file:src/retry.rs", "workspace:file:src/budget.rs"],
            source_labels: &["lexical_filename", "lexical_path"],
            partial_causes: &["indexing_in_progress"],
            hidden_by_scope: 0,
            hidden_by_policy: 0,
            hidden_by_remote_cache: 0,
            summary: "A local replay packet keeps its literal query text on device under explicit elevated consent and preserves the captured query session, selected results, and counts for debugging without claiming live results.",
        }),
        build_export_row(ExportRowSeed {
            row_id: "export:support-bundle",
            scenario: "Default redacted support bundle for a file-search pass.",
            export_class: SearchExportClass::SupportBundle,
            consent: ExportConsentClass::MetadataOnly,
            surface: SearchSurface::FileSearch,
            query_text: "kind:file flaky",
            scope_class: ScopeClass::FullWorkspace,
            scope_label: "Full workspace",
            readiness_state: "warming",
            drifted: false,
            visible_rows: 12,
            included: &["workspace:file:tests/flaky.rs"],
            source_labels: &["lexical_path"],
            partial_causes: &["indexing_in_progress"],
            hidden_by_scope: 3,
            hidden_by_policy: 0,
            hidden_by_remote_cache: 0,
            summary: "A support bundle explains what ran, what was selected, and what was omitted with a query hash, scope summary, result refs, and omission counts — never the literal query text.",
        }),
        build_export_row(ExportRowSeed {
            row_id: "export:incident-packet",
            scenario: "Incident packet for a drifted symbol-search escalation.",
            export_class: SearchExportClass::IncidentPacket,
            consent: ExportConsentClass::MetadataOnly,
            surface: SearchSurface::SymbolSearch,
            query_text: "SearchPlanner",
            scope_class: ScopeClass::SelectedWorkset,
            scope_label: "Triage workset",
            readiness_state: "partial",
            drifted: true,
            visible_rows: 8,
            included: &["workspace:symbol:SearchPlannerAlpha"],
            source_labels: &["graph_symbol"],
            partial_causes: &["graph_warming", "scope_changed_since_capture"],
            hidden_by_scope: 1,
            hidden_by_policy: 1,
            hidden_by_remote_cache: 0,
            summary: "An incident packet preserves the search intent and provenance, discloses that its captured scope drifted and must rerun, and keeps query material redacted to hashes and refs.",
        }),
        build_export_row(ExportRowSeed {
            row_id: "export:managed-analytics",
            scenario: "Managed analytics ingestion of a docs-search pass.",
            export_class: SearchExportClass::ManagedAnalytics,
            consent: ExportConsentClass::MetadataOnly,
            surface: SearchSurface::DocsSearch,
            query_text: "auth policy",
            scope_class: ScopeClass::PolicyLimitedView,
            scope_label: "Policy-limited view",
            readiness_state: "ready",
            drifted: false,
            visible_rows: 4,
            included: &["docs:anchor:auth-policy-overview"],
            source_labels: &["docs_linked"],
            partial_causes: &["policy_hidden_candidates"],
            hidden_by_scope: 0,
            hidden_by_policy: 2,
            hidden_by_remote_cache: 0,
            summary: "A managed analytics export carries neither literal nor hash query material — only scope summaries, result refs, omission counts, and reason summaries — so the same privacy rules hold on the analytics path.",
        }),
    ];

    let consumer_projections = seeded_consumer_projections();

    SearchExportGovernancePacket {
        record_kind: SEARCH_EXPORT_GOVERNANCE_PACKET_RECORD_KIND.to_owned(),
        schema_version: SEARCH_EXPORT_GOVERNANCE_SCHEMA_VERSION,
        packet_id: SEARCH_EXPORT_GOVERNANCE_PACKET_ID.to_owned(),
        generated_at: SEEDED_GENERATED_AT.to_owned(),
        doc_ref: SEARCH_EXPORT_GOVERNANCE_DOC_REF.to_owned(),
        schema_ref: SEARCH_EXPORT_GOVERNANCE_SCHEMA_REF.to_owned(),
        artifact_ref: SEARCH_EXPORT_GOVERNANCE_ARTIFACT_REF.to_owned(),
        source_spec_refs: vec![
            ".t2/docs/Aureline_Milestones_Document.md".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md".to_owned(),
            ".t2/docs/Aureline_UI_UX_Spec_Document.md".to_owned(),
            ".t2/docs/Aureline_PRD.md".to_owned(),
        ],
        supporting_contract_refs: vec![
            "schemas/search/search_export_snapshot.schema.json".to_owned(),
            "schemas/search/query_session.schema.json".to_owned(),
            "schemas/search/saved-query-governance.schema.json".to_owned(),
            "schemas/search/ranking-explainability.schema.json".to_owned(),
            "schemas/search/search_result_truth_packet.schema.json".to_owned(),
        ],
        covered_export_classes: ALL_EXPORT_CLASSES.to_vec(),
        covered_redaction_states: ALL_REDACTION_STATES.to_vec(),
        covered_consent_classes: ALL_CONSENT_CLASSES.to_vec(),
        export_rows,
        consumer_projections,
        export_safe_summary:
            "Support exports and incident packets explain what search ran, what was selected, and what was omitted using query hashes, scope summaries, result refs, omission counts, and reason summaries. Literal query text stays confined to consented local-only replay packets, every packet is a captured snapshot that never claims live results, and desktop, CLI/headless, support export, and managed analytics read the same packets under the same privacy rules."
                .to_owned(),
    }
}

/// Returns the seeded redacted export copy of the canonical packet.
pub fn seeded_redacted_search_export_packet() -> SearchExportGovernancePacket {
    seeded_search_export_governance_packet().redact_for_export()
}

/// Seed for one governed export row.
struct ExportRowSeed {
    row_id: &'static str,
    scenario: &'static str,
    export_class: SearchExportClass,
    consent: ExportConsentClass,
    surface: SearchSurface,
    query_text: &'static str,
    scope_class: ScopeClass,
    scope_label: &'static str,
    readiness_state: &'static str,
    drifted: bool,
    visible_rows: u64,
    included: &'static [&'static str],
    source_labels: &'static [&'static str],
    partial_causes: &'static [&'static str],
    hidden_by_scope: u64,
    hidden_by_policy: u64,
    hidden_by_remote_cache: u64,
    summary: &'static str,
}

fn build_export_row(seed: ExportRowSeed) -> SearchExportRow {
    let session = SearchQuerySession::for_local_text(
        format!("search:session:{}", seed.row_id),
        seed.surface,
        seed.query_text,
        seed.scope_class,
        seed.scope_label,
        "search-planner-alpha",
        seed.readiness_state,
        SEEDED_GENERATED_AT,
    );

    // Per-class redaction posture. Only a local-replay class under elevated
    // consent retains the literal; everything that leaves the device is hashed,
    // and managed analytics drops even the hash.
    let keeps_literal = seed.export_class.permits_literal_query_text()
        && seed.consent == ExportConsentClass::QueryTextElevated;
    let (privacy_class, redaction_state, query_text, query_hash, query_text_mode) = if keeps_literal
    {
        (
            SavedQueryPrivacyClass::LocalOnlyPrivate,
            SearchPacketRedactionState::RawQueryLocalOnly,
            session.query_text.clone(),
            session.query_hash.clone(),
            QueryTextMode::LocalText,
        )
    } else if seed.export_class.forbids_all_query_material() {
        (
            SavedQueryPrivacyClass::PolicyWithheld,
            SearchPacketRedactionState::QueryMaterialOmittedByPolicy,
            None,
            None,
            QueryTextMode::OmittedByPolicy,
        )
    } else {
        (
            SavedQueryPrivacyClass::SupportExportRedacted,
            SearchPacketRedactionState::QueryHashOnly,
            None,
            session.query_hash.clone(),
            QueryTextMode::HashOnly,
        )
    };

    let included_result_refs: Vec<String> =
        seed.included.iter().map(|id| (*id).to_owned()).collect();
    let result_source_labels: Vec<String> =
        seed.source_labels.iter().map(|s| (*s).to_owned()).collect();
    let partial_truth_causes: Vec<String> = seed
        .partial_causes
        .iter()
        .map(|s| (*s).to_owned())
        .collect();

    let included_rows = included_result_refs.len() as u64;
    let omitted_result_count = seed.visible_rows.saturating_sub(included_rows);
    let count_is_partial = seed.hidden_by_scope > 0
        || seed.hidden_by_policy > 0
        || seed.hidden_by_remote_cache > 0
        || seed.drifted
        || !partial_truth_causes.is_empty();
    let count_summary = SearchPacketCountSummary {
        visible_rows: seed.visible_rows,
        selected_rows: included_rows,
        included_rows,
        omitted_result_count,
        hidden_by_current_scope_rows: seed.hidden_by_scope,
        hidden_by_policy_rows: seed.hidden_by_policy,
        hidden_by_remote_cache_rows: seed.hidden_by_remote_cache,
        count_is_partial,
    };
    let omitted_or_truncated_flags = omission_flags(&count_summary);

    let snapshot_truth = if seed.drifted {
        SearchExportSnapshotTruth::ScopeChangedSinceCapture
    } else {
        SearchExportSnapshotTruth::CapturedSnapshot
    };

    let result_set_id = format!("search:result_set:{}", seed.row_id);
    let planner_pass_id = format!("search:planner_pass:{}", seed.row_id);
    let evidence_refs = vec![
        format!("query_session:{}", session.query_session_id),
        format!("result_set:{result_set_id}"),
        format!("planner_pass:{planner_pass_id}"),
    ];

    let export_packet = SearchExportPacket {
        record_kind: SearchExportPacket::RECORD_KIND.to_owned(),
        schema_version: SAVED_QUERY_ALPHA_SCHEMA_VERSION,
        packet_id: format!("search:export_packet:{}", seed.row_id),
        destination: seed.export_class.export_destination(),
        query_session_id_ref: session.query_session_id.clone(),
        result_set_id_ref: result_set_id,
        planner_pass_id_ref: planner_pass_id,
        surface: session.surface,
        scope_class: session.scope_class,
        scope_label: session.scope_label.clone(),
        stable_scope_id: session.stable_scope_id.clone(),
        readiness_state: session.readiness_state.clone(),
        index_epoch: None,
        graph_epoch: None,
        privacy_class,
        redaction_state,
        snapshot_truth,
        query_text_mode,
        query_text,
        query_hash,
        selected_result_refs: included_result_refs.clone(),
        included_result_refs,
        result_source_labels,
        partial_truth_causes,
        count_summary,
        omitted_or_truncated_flags,
        evidence_refs,
        exported_at: SEEDED_GENERATED_AT.to_owned(),
    };

    let literal_query_text_included = export_packet.query_text.is_some();

    let replay_safety = if seed.drifted {
        ReplaySafetyDisclosure {
            result_semantics: SearchResultSemantics::ScopeChangedSinceCapture,
            scope_honesty_state: SearchScopeHonestyState::CurrentScopeChangedRebindRequired,
            snapshot_truth,
            rerun_required_for_current_truth: true,
            preserves_intent_and_provenance: true,
            claims_live_current_results: false,
            disclosure:
                "The captured scope changed since this packet was captured; reopening rebinds to the current scope and reruns before claiming truth."
                    .to_owned(),
        }
    } else {
        ReplaySafetyDisclosure {
            result_semantics: SearchResultSemantics::LiveRerunRequired,
            scope_honesty_state: SearchScopeHonestyState::CapturedScopeStillCurrent,
            snapshot_truth,
            rerun_required_for_current_truth: true,
            preserves_intent_and_provenance: true,
            claims_live_current_results: false,
            disclosure:
                "This packet is a captured snapshot of a prior search pass; it preserves intent and provenance and reruns before claiming current results."
                    .to_owned(),
        }
    };

    SearchExportRow {
        row_id: seed.row_id.to_owned(),
        scenario: seed.scenario.to_owned(),
        export_class: seed.export_class,
        literal_query_consent: seed.consent,
        literal_query_text_included,
        higher_trust_export_class_required_for_literal: true,
        export_packet,
        replay_safety,
        summary: seed.summary.to_owned(),
    }
}

fn omission_flags(count_summary: &SearchPacketCountSummary) -> Vec<String> {
    let mut flags = Vec::new();
    if count_summary.omitted_result_count > 0 {
        flags.push("omitted_unselected_results".to_owned());
    }
    if count_summary.hidden_by_current_scope_rows > 0 {
        flags.push("hidden_by_current_scope".to_owned());
    }
    if count_summary.hidden_by_policy_rows > 0 {
        flags.push("hidden_by_policy".to_owned());
    }
    if count_summary.hidden_by_remote_cache_rows > 0 {
        flags.push("hidden_by_remote_cache".to_owned());
    }
    if count_summary.count_is_partial {
        flags.push("partial_counts".to_owned());
    }
    flags
}

fn seeded_consumer_projections() -> Vec<SearchExportConsumerProjection> {
    let make = |consumer: SearchExportConsumerClass, consumer_ref: &str, summary: &str| {
        SearchExportConsumerProjection {
            consumer,
            consumer_ref: consumer_ref.to_owned(),
            ingested_packet_id: SEARCH_EXPORT_GOVERNANCE_PACKET_ID.to_owned(),
            preserves_redaction_mode: true,
            preserves_count_and_omission_disclosure: true,
            preserves_replay_safety: true,
            reuses_same_export_packets: true,
            literal_query_text_excluded: true,
            ambient_authority_excluded: true,
            summary: summary.to_owned(),
        }
    };

    vec![
        make(
            SearchExportConsumerClass::DesktopShell,
            "crates/aureline-shell/src/search_export_governance/mod.rs",
            "The desktop search/support chrome replays the export packets, renders the redaction mode and omission counts, and never reads raw UI state or screenshots.",
        ),
        make(
            SearchExportConsumerClass::CliHeadless,
            "docs/search/search-export-packet.md",
            "CLI/headless inspect reads the same serialized export packets to replay what ran, what was selected, and what was omitted under the same privacy rules.",
        ),
        make(
            SearchExportConsumerClass::SupportExport,
            "artifacts/search/m5/search-export-packet.md",
            "Support and incident export wraps the redacted packet so a bundle inspects the export packets with no literal query material.",
        ),
        make(
            SearchExportConsumerClass::ManagedAnalytics,
            "docs/search/search-export-packet.md",
            "The managed analytics path ingests the metadata-only packets and never widens retention to literal or hashed query material.",
        ),
    ]
}

fn push(findings: &mut Vec<SearchExportGovernanceValidationFinding>, path: &str, message: &str) {
    findings.push(SearchExportGovernanceValidationFinding {
        path: path.to_owned(),
        message: message.to_owned(),
    });
}

#[cfg(test)]
mod tests;
