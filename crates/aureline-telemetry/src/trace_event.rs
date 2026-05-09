//! Normalised trace-event record types.
//!
//! The record structs in this module are intended to match the boundary schema
//! at `schemas/traces/trace_event.schema.json`. Emitters should treat the
//! vocabulary fields as stable, reviewable tokens and avoid inventing
//! per-surface synonyms.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Minimum build-identity record embedded in each trace-event record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildIdentityRecord {
    pub crate_name: String,
    pub crate_version: String,
    pub rustc_target_triple: String,
}

/// Reference to a protected benchmark corpus manifest revision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorpusManifestRefRecord {
    pub manifest_id: String,
    pub manifest_revision: u32,
}

/// Normalised trace-event record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceEventRecord {
    pub schema: Cow<'static, str>,
    pub schema_version: u32,
    pub record_kind: Cow<'static, str>,
    pub event_id: String,
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub span_kind: Cow<'static, str>,
    pub event_class: Cow<'static, str>,
    pub protected_journey: Cow<'static, str>,
    pub dispatch_layer: Cow<'static, str>,
    pub journey_segment_id: Cow<'static, str>,
    pub budget_ref: Cow<'static, str>,
    pub attempt_class: Cow<'static, str>,
    pub outcome_class: Cow<'static, str>,
    pub degraded_posture: Cow<'static, str>,
    pub fallback_posture: Cow<'static, str>,
    pub backend: Cow<'static, str>,
    pub host_os: Cow<'static, str>,
    pub build: BuildIdentityRecord,
    pub exact_build_identity_ref: Option<String>,
    pub hardware_definition_ref: Option<String>,
    pub environment_ref: Option<String>,
    pub fixture_ref: Option<String>,
    pub corpus_manifest: Option<CorpusManifestRefRecord>,
    pub sampling_profile: Cow<'static, str>,
    pub sampling_profile_ref: String,
    pub retention_class: Cow<'static, str>,
    pub export_posture: Cow<'static, str>,
    pub redaction_class: Cow<'static, str>,
    pub started_tick: u64,
    pub finished_tick: Option<u64>,
    pub duration_ticks: Option<u64>,
    pub linked_spike_trace_refs: Vec<String>,
    pub linked_journey_trace_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub requirement_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl TraceEventRecord {
    /// Trace-event schema token for this record family.
    pub const SCHEMA: &'static str = "aureline.trace_event.v1";

    /// Schema version for [`Self::SCHEMA`].
    pub const SCHEMA_VERSION: u32 = 1;

    /// Record-kind discriminator for this record family.
    pub const RECORD_KIND: &'static str = "trace_event_record";
}
