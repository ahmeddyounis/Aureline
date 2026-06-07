//! Canonical chronology grammar and export-safe history rows.
//!
//! This crate owns the stable object model used when activity-center rows,
//! provider events, AI run history, policy notices, recovery timelines,
//! companion delivery, and support exports need to describe the same event
//! without surface-local wording drift.

#![doc(html_root_url = "https://docs.rs/aureline-chronology/0.0.0")]

pub mod stabilize_chronology_grammar_and_history_row_truth;

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
