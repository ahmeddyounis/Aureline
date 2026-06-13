//! Canonical chronology grammar and export-safe history rows.
//!
//! This crate owns the stable object model used when activity-center rows,
//! provider events, AI run history, policy notices, recovery timelines,
//! companion delivery, and support exports need to describe the same event
//! without surface-local wording drift.

#![doc(html_root_url = "https://docs.rs/aureline-chronology/0.0.0")]

pub mod m5_evidence_chronology_lineage;
pub mod stabilize_chronology_grammar_and_history_row_truth;

pub use m5_evidence_chronology_lineage::{
    seeded_m5_evidence_chronology_packet, ActorLineage, ActorLineageRole, ActorLineageStep,
    AdminEvidenceRow, EvidenceResidencyClass, LineageStepTime, M5EvidenceChronologyPacket,
    M5EvidenceChronologyRow, M5EvidenceChronologyViolation, M5EvidenceWorkflowClass,
    ProductEvidenceRow, SupportExportEvidenceRow, M5_EVIDENCE_CHRONOLOGY_ARTIFACT_REF,
    M5_EVIDENCE_CHRONOLOGY_DOC_REF, M5_EVIDENCE_CHRONOLOGY_FIXTURE_DIR,
    M5_EVIDENCE_CHRONOLOGY_PACKET_RECORD_KIND, M5_EVIDENCE_CHRONOLOGY_ROW_RECORD_KIND,
    M5_EVIDENCE_CHRONOLOGY_SCHEMA_REF, M5_EVIDENCE_CHRONOLOGY_SCHEMA_VERSION,
    M5_EVIDENCE_CHRONOLOGY_SHARED_CONTRACT_REF,
};

pub use stabilize_chronology_grammar_and_history_row_truth::{
    seeded_accessibility_fixture, seeded_chronology_export_packet, seeded_chronology_packet,
    validate_accessibility_fixture, validate_chronology_export_packet, validate_chronology_packet,
    AccessibilityChronologyFixture, AccessibilityHistoryRowProjection, ActionVerb, ActorKind,
    ChronologyExportPacket, ChronologyExportRow, ChronologyFreshnessClass, ChronologyHistoryPacket,
    ChronologyHistoryRow, ChronologyImportedClass, ChronologyObjectKind, ChronologyRowAuditReport,
    ChronologyRowFinding, ChronologySourceClass, ChronologySurfaceClass, FollowUpState,
    FollowUpTransition, FollowUpTransitionKind, HistoryOutcomeClass, LocalAuthorityEffectClass,
    ProvenanceBadge, RelativeAgeHint, ReopenTarget, TimePosture,
    ACCESSIBILITY_CHRONOLOGY_FIXTURE_RECORD_KIND, CHRONOLOGY_EXPORT_PACKET_RECORD_KIND,
    CHRONOLOGY_EXPORT_ROW_RECORD_KIND, CHRONOLOGY_HISTORY_PACKET_RECORD_KIND,
    CHRONOLOGY_HISTORY_ROW_RECORD_KIND, CHRONOLOGY_HISTORY_SCHEMA_REF,
    CHRONOLOGY_HISTORY_SCHEMA_VERSION,
};
