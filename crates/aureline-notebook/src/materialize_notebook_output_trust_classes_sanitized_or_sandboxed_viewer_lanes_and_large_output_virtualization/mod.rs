//! Notebook output trust classes, sanitized or sandboxed viewer lanes, and
//! large-output virtualization.
//!
//! This module materializes the viewer-lane and virtualization layer that sits
//! between the output trust record and the rendered notebook surface. It
//! produces [`NotebookOutputViewerLane`] records and
//! [`LargeOutputVirtualizationRecord`] records so the chrome never silently
//! escalates trust, never freezes on heavy outputs, and always shows the user
//! why an output is rendered inline, virtualized, opened in detail, or blocked.
//!
//! The module exposes:
//!
//! - the [`NotebookOutputViewerLane`] record that binds an output block to its
//!   viewer lane (`inline`, `virtualized`, `open_detail`, `blocked_active_content`)
//!   based on its trust class, size bucket, and virtualization state;
//! - the [`LargeOutputVirtualizationRecord`] record that carries size estimates,
//!   virtualization state, truncation notes, and available actions for outputs
//!   that exceed inline-rendering budgets;
//! - the [`NotebookOutputViewerPacket`] checked-in artifact that downstream docs,
//!   help, support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT
//! appear on any record carried here. Only opaque handles and closed-
//! vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every record carried by this module. Bumped only
/// on breaking payload changes; additive-optional fields do not bump this
/// value.
pub const NOTEBOOK_OUTPUT_VIEWER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`NotebookOutputViewerLane`] payloads.
pub const NOTEBOOK_OUTPUT_VIEWER_LANE_RECORD_KIND: &str = "notebook_output_viewer_lane";

/// Stable record-kind tag for serialized [`LargeOutputVirtualizationRecord`]
/// payloads.
pub const LARGE_OUTPUT_VIRTUALIZATION_RECORD_KIND: &str = "notebook_large_output_virtualization";

/// Stable record-kind tag for the checked-in [`NotebookOutputViewerPacket`].
pub const NOTEBOOK_OUTPUT_VIEWER_PACKET_RECORD_KIND: &str = "notebook_output_viewer_packet";

/// Repo-relative path to the checked-in output-viewer packet JSON.
pub const NOTEBOOK_OUTPUT_VIEWER_PACKET_PATH: &str =
    "artifacts/notebook/m5/materialize_notebook_output_trust_classes_sanitized_or_sandboxed_viewer_lanes_and_large_output_virtualization.json";

/// Embedded checked-in output-viewer packet JSON.
pub const NOTEBOOK_OUTPUT_VIEWER_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/materialize_notebook_output_trust_classes_sanitized_or_sandboxed_viewer_lanes_and_large_output_virtualization.json"
));

macro_rules! closed_vocab {
    (
        $(#[$type_doc:meta])*
        $name:ident {
            $(
                $(#[$variant_doc:meta])*
                $variant:ident => $token:literal
            ),+ $(,)?
        }
    ) => {
        $(#[$type_doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum $name {
            $(
                $(#[$variant_doc])*
                #[serde(rename = $token)]
                $variant
            ),+
        }

        impl $name {
            /// Stable closed-vocabulary token recorded in records, schemas,
            /// fixtures, and exports.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $token),+
                }
            }
        }
    };
}

closed_vocab!(
    /// Where an output is rendered relative to the cell that produced it.
    /// `inline` keeps the output in the cell flow; `virtualized` adds
    /// windowing or lazy materialization; `open_detail` moves the output
    /// into a dedicated detail pane; `blocked_active_content` shows a
    /// safe placeholder instead of the active payload.
    OutputViewerLaneClass {
        Inline => "inline",
        Virtualized => "virtualized",
        OpenDetail => "open_detail",
        BlockedActiveContent => "blocked_active_content",
    }
);

impl OutputViewerLaneClass {
    /// True for lanes that require a compatible viewer to be available.
    pub const fn requires_compatible_viewer(self) -> bool {
        matches!(self, Self::Inline | Self::Virtualized | Self::OpenDetail)
    }

    /// True for lanes that present a placeholder rather than rendered content.
    pub const fn is_placeholder(self) -> bool {
        matches!(self, Self::BlockedActiveContent)
    }
}

closed_vocab!(
    /// Size bucket that drives virtualization decisions. The thresholds are
    /// chrome policy; the bucket is the contract surfaced to the user.
    OutputSizeBucket {
        Small => "small",
        Medium => "medium",
        Large => "large",
        VeryLarge => "very_large",
    }
);

impl OutputSizeBucket {
    /// True for buckets that admit inline rendering without virtualization.
    pub const fn admits_inline(self) -> bool {
        matches!(self, Self::Small | Self::Medium)
    }

    /// True for buckets that should trigger virtualization or detail-view
    /// rendering by default.
    pub const fn requires_virtualization(self) -> bool {
        matches!(self, Self::Large | Self::VeryLarge)
    }
}

closed_vocab!(
    /// Virtualization state of an output. Distinguishes outputs that need no
    /// special handling, outputs that are actively windowed, outputs that
    /// were truncated with an expand action, and outputs that are deferred
    /// until the user explicitly requests them.
    OutputVirtualizationStateClass {
        NotNeeded => "not_needed",
        Virtualized => "virtualized",
        Truncated => "truncated",
        LazyPending => "lazy_pending",
    }
);

impl OutputVirtualizationStateClass {
    /// True for states that imply the output is not fully materialized.
    pub const fn is_partial(self) -> bool {
        matches!(self, Self::Virtualized | Self::Truncated | Self::LazyPending)
    }
}

/// Typed validation finding for a [`NotebookOutputViewerLane`] or
/// [`LargeOutputVirtualizationRecord`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputViewerFinding {
    /// Stable check identifier.
    pub check_id: String,
    /// Opaque subject reference (usually the record id).
    pub subject_ref: String,
    /// Human-readable message.
    pub message: String,
}

impl OutputViewerFinding {
    /// Constructs a finding.
    pub fn new(check_id: impl Into<String>, subject_ref: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            check_id: check_id.into(),
            subject_ref: subject_ref.into(),
            message: message.into(),
        }
    }
}

/// Canonical notebook output viewer lane record.
///
/// Maps an output block to the lane where it should be rendered, the size
/// bucket that drove the decision, and the virtualization state that governs
/// how much of the output is currently materialized.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookOutputViewerLane {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_output_viewer_schema_version: u32,
    /// Stable opaque lane record id.
    pub lane_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Opaque cell id this output is attributed to.
    pub cell_id_ref: String,
    /// Opaque output-block id.
    pub output_block_ref: String,
    /// Output trust class projected from the runtime-trust model.
    pub trust_class: crate::OutputTrustClass,
    /// Viewer lane assigned to this output.
    pub viewer_lane_class: OutputViewerLaneClass,
    /// Size bucket for the output.
    pub size_bucket: OutputSizeBucket,
    /// Virtualization state class.
    pub virtualization_state_class: OutputVirtualizationStateClass,
    /// Whether a compatible viewer is currently available.
    pub compatible_viewer_available: bool,
    /// Whether a raw fallback viewer is available.
    pub raw_fallback_available: bool,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookOutputViewerLane {
    /// Returns typed validation findings; an empty vector means the record
    /// is internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<OutputViewerFinding> {
        let mut findings = Vec::new();
        let subject = self.lane_id.as_str();

        if self.record_kind != NOTEBOOK_OUTPUT_VIEWER_LANE_RECORD_KIND {
            findings.push(OutputViewerFinding::new(
                "output_viewer_lane.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_OUTPUT_VIEWER_LANE_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_output_viewer_schema_version != NOTEBOOK_OUTPUT_VIEWER_SCHEMA_VERSION {
            findings.push(OutputViewerFinding::new(
                "output_viewer_lane.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_OUTPUT_VIEWER_SCHEMA_VERSION}, found {}",
                    self.notebook_output_viewer_schema_version
                ),
            ));
        }

        // Trust-class / lane consistency.
        match self.trust_class {
            crate::OutputTrustClass::TrustedActive => {
                if self.viewer_lane_class == OutputViewerLaneClass::BlockedActiveContent
                    && self.compatible_viewer_available
                {
                    findings.push(OutputViewerFinding::new(
                        "output_viewer_lane.trusted_active_blocked_despite_viewer",
                        subject,
                        "trusted_active output with a compatible viewer should not be blocked",
                    ));
                }
            }
            crate::OutputTrustClass::Sanitized | crate::OutputTrustClass::Sandboxed => {
                if self.viewer_lane_class == OutputViewerLaneClass::BlockedActiveContent {
                    findings.push(OutputViewerFinding::new(
                        "output_viewer_lane.sanitized_sandboxed_not_blocked",
                        subject,
                        "sanitized/sandboxed outputs render in constrained viewers, not blocked",
                    ));
                }
            }
            crate::OutputTrustClass::Stale => {
                // Stale outputs may appear in any lane as a fallback;
                // no hard rule beyond the compatible-viewer check below.
            }
        }

        // Lane / size-bucket consistency.
        if self.viewer_lane_class == OutputViewerLaneClass::Virtualized
            && self.size_bucket.admits_inline()
        {
            findings.push(OutputViewerFinding::new(
                "output_viewer_lane.virtualized_but_small",
                subject,
                "virtualized lane should not be assigned to small/medium outputs",
            ));
        }

        if self.viewer_lane_class == OutputViewerLaneClass::Inline
            && self.size_bucket.requires_virtualization()
        {
            findings.push(OutputViewerFinding::new(
                "output_viewer_lane.inline_but_large",
                subject,
                "inline lane should not be assigned to large/very_large outputs",
            ));
        }

        // Virtualization-state / size-bucket consistency.
        if self.virtualization_state_class == OutputVirtualizationStateClass::NotNeeded
            && self.size_bucket.requires_virtualization()
        {
            findings.push(OutputViewerFinding::new(
                "output_viewer_lane.not_needed_but_large",
                subject,
                "not_needed virtualization state is inconsistent with large/very_large size",
            ));
        }

        if self.virtualization_state_class != OutputVirtualizationStateClass::NotNeeded
            && self.size_bucket == OutputSizeBucket::Small
        {
            findings.push(OutputViewerFinding::new(
                "output_viewer_lane.virtualized_but_small_bucket",
                subject,
                "virtualization states are inconsistent with small size bucket",
            ));
        }

        // Compatible-viewer requirement for non-blocked lanes.
        if self.viewer_lane_class.requires_compatible_viewer() && !self.compatible_viewer_available
        {
            findings.push(OutputViewerFinding::new(
                "output_viewer_lane.viewer_required_but_missing",
                subject,
                "inline, virtualized, and open_detail lanes require a compatible viewer",
            ));
        }

        findings
    }
}

/// Canonical large-output virtualization record.
///
/// Carries the size estimates, virtualization state, and available actions
/// for an output that exceeds inline-rendering budgets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LargeOutputVirtualizationRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_output_viewer_schema_version: u32,
    /// Stable opaque virtualization record id.
    pub virtualization_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Opaque cell id this output is attributed to.
    pub cell_id_ref: String,
    /// Opaque output-block id.
    pub output_block_ref: String,
    /// Size bucket for the output.
    pub size_bucket: OutputSizeBucket,
    /// Estimated byte size of the raw output payload; zero when unknown.
    pub byte_size_estimate: u64,
    /// Estimated row count for tabular outputs; zero when not applicable.
    pub row_count_estimate: u64,
    /// Current virtualization state.
    pub virtualization_state_class: OutputVirtualizationStateClass,
    /// Truncation note visible to the user. Required when state is
    /// [`OutputVirtualizationStateClass::Truncated`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub truncation_note: Option<String>,
    /// Whether the user can expand the full output.
    pub expand_action_available: bool,
    /// Whether the user can export the full output.
    pub export_action_available: bool,
    /// Export-safe summary line.
    pub summary: String,
}

impl LargeOutputVirtualizationRecord {
    /// Returns typed validation findings; an empty vector means the record
    /// is internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<OutputViewerFinding> {
        let mut findings = Vec::new();
        let subject = self.virtualization_id.as_str();

        if self.record_kind != LARGE_OUTPUT_VIRTUALIZATION_RECORD_KIND {
            findings.push(OutputViewerFinding::new(
                "large_output_virtualization.record_kind",
                subject,
                format!(
                    "record_kind must be '{LARGE_OUTPUT_VIRTUALIZATION_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_output_viewer_schema_version != NOTEBOOK_OUTPUT_VIEWER_SCHEMA_VERSION {
            findings.push(OutputViewerFinding::new(
                "large_output_virtualization.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_OUTPUT_VIEWER_SCHEMA_VERSION}, found {}",
                    self.notebook_output_viewer_schema_version
                ),
            ));
        }

        if self.byte_size_estimate == 0 && self.row_count_estimate == 0 {
            findings.push(OutputViewerFinding::new(
                "large_output_virtualization.no_size_estimate",
                subject,
                "at least one of byte_size_estimate or row_count_estimate must be provided",
            ));
        }

        if self.virtualization_state_class == OutputVirtualizationStateClass::Truncated
            && self.truncation_note.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true)
        {
            findings.push(OutputViewerFinding::new(
                "large_output_virtualization.truncated_note_required",
                subject,
                "truncated state requires a non-empty truncation_note",
            ));
        }

        if self.size_bucket.admits_inline()
            && self.virtualization_state_class != OutputVirtualizationStateClass::NotNeeded
        {
            findings.push(OutputViewerFinding::new(
                "large_output_virtualization.small_but_virtualized",
                subject,
                "small/medium outputs should use not_needed virtualization state",
            ));
        }

        if self.size_bucket.requires_virtualization()
            && self.virtualization_state_class == OutputVirtualizationStateClass::NotNeeded
        {
            findings.push(OutputViewerFinding::new(
                "large_output_virtualization.large_but_not_virtualized",
                subject,
                "large/very_large outputs must use a virtualization state",
            ));
        }

        findings
    }
}

/// Checked-in notebook output viewer packet that downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookOutputViewerPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: output viewer lane classes.
    pub output_viewer_lane_classes: Vec<OutputViewerLaneClass>,
    /// Closed vocabulary: output size buckets.
    pub output_size_buckets: Vec<OutputSizeBucket>,
    /// Closed vocabulary: output virtualization state classes.
    pub output_virtualization_state_classes: Vec<OutputVirtualizationStateClass>,
    /// Worked example viewer lane records.
    pub example_viewer_lanes: Vec<NotebookOutputViewerLane>,
    /// Worked example large-output virtualization records.
    pub example_large_output_virtualizations: Vec<LargeOutputVirtualizationRecord>,
    /// Export-safe summary line.
    pub summary: String,
}

/// Typed validation finding for the checked-in packet.
pub type NotebookOutputViewerPacketFinding = OutputViewerFinding;

impl NotebookOutputViewerPacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<NotebookOutputViewerPacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_OUTPUT_VIEWER_SCHEMA_VERSION {
            findings.push(NotebookOutputViewerPacketFinding::new(
                "output_viewer_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_OUTPUT_VIEWER_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != NOTEBOOK_OUTPUT_VIEWER_PACKET_RECORD_KIND {
            findings.push(NotebookOutputViewerPacketFinding::new(
                "output_viewer_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_OUTPUT_VIEWER_PACKET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }

        if self.output_viewer_lane_classes.len() != OutputViewerLaneClass::ALL.len() {
            findings.push(NotebookOutputViewerPacketFinding::new(
                "output_viewer_packet.lane_classes_coverage",
                subject,
                "output_viewer_lane_classes must list every variant",
            ));
        }
        if self.output_size_buckets.len() != OutputSizeBucket::ALL.len() {
            findings.push(NotebookOutputViewerPacketFinding::new(
                "output_viewer_packet.size_buckets_coverage",
                subject,
                "output_size_buckets must list every variant",
            ));
        }
        if self.output_virtualization_state_classes.len() != OutputVirtualizationStateClass::ALL.len()
        {
            findings.push(NotebookOutputViewerPacketFinding::new(
                "output_viewer_packet.virtualization_state_classes_coverage",
                subject,
                "output_virtualization_state_classes must list every variant",
            ));
        }

        for lane in &self.example_viewer_lanes {
            findings.extend(
                lane.validate()
                    .into_iter()
                    .map(|f| NotebookOutputViewerPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)),
            );
        }
        for virt in &self.example_large_output_virtualizations {
            findings.extend(
                virt.validate()
                    .into_iter()
                    .map(|f| NotebookOutputViewerPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)),
            );
        }

        findings
    }
}

impl OutputViewerLaneClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [Self::Inline, Self::Virtualized, Self::OpenDetail, Self::BlockedActiveContent];
}

impl OutputSizeBucket {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [Self::Small, Self::Medium, Self::Large, Self::VeryLarge];
}

impl OutputVirtualizationStateClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [Self::NotNeeded, Self::Virtualized, Self::Truncated, Self::LazyPending];
}

/// Parses the checked-in output-viewer packet JSON.
pub fn current_notebook_output_viewer_packet() -> Result<NotebookOutputViewerPacket, serde_json::Error> {
    serde_json::from_str(NOTEBOOK_OUTPUT_VIEWER_PACKET_JSON)
}

#[cfg(test)]
mod tests;
