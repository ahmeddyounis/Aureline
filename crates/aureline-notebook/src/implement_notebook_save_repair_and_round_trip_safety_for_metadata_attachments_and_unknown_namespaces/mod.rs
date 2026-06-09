//! Notebook save, repair, and round-trip safety for metadata, attachments, and
//! unknown namespaces.
//!
//! This module materializes the typed records that keep notebook save, repair,
//! and round-trip flows honest about what is preserved, what is lost, and why.
//! It builds on the canonical document model in
//! [`crate::materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability`]
//! and adds save-operation, repair-action, and round-trip assertion records so
//! that metadata, attachments, and unknown namespaces never disappear silently.
//!
//! The module exposes:
//!
//! - the [`NotebookSaveOperation`] record that describes a save (full, auto,
//!   checkpoint, or export), the preservation posture for metadata, attachments,
//!   and unknown namespaces, and whether the operation is round-trip safe;
//! - the [`NotebookRepairAction`] record that describes a repair applied to a
//!   damaged or invalid notebook, the kind of repair, and its consequence
//!   (lossless, lossy with explicit note, or lossy with silent fallback);
//! - the [`NotebookRoundTripAssertion`] record that asserts whether a specific
//!   property survives an open/edit/save cycle, with an explicit result class
//!   and loss summary when preservation is partial or blocked;
//! - the [`NotebookSaveRepairRoundTripPacket`] checked-in artifact that
//!   downstream docs, help, support, and CI surfaces ingest instead of cloning
//!   status text.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw attachment bytes, and
//! raw URLs MUST NOT appear on any record carried here. Only opaque handles
//! and closed-vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every record carried by this module. Bumped only
/// on breaking payload changes; additive-optional fields do not bump this
/// value.
pub const NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`NotebookSaveOperation`] payloads.
pub const NOTEBOOK_SAVE_OPERATION_RECORD_KIND: &str = "notebook_save_operation";

/// Stable record-kind tag for serialized [`NotebookRepairAction`] payloads.
pub const NOTEBOOK_REPAIR_ACTION_RECORD_KIND: &str = "notebook_repair_action";

/// Stable record-kind tag for serialized [`NotebookRoundTripAssertion`] payloads.
pub const NOTEBOOK_ROUND_TRIP_ASSERTION_RECORD_KIND: &str = "notebook_round_trip_assertion";

/// Stable record-kind tag for the checked-in [`NotebookSaveRepairRoundTripPacket`].
pub const NOTEBOOK_SAVE_REPAIR_ROUND_TRIP_PACKET_RECORD_KIND: &str =
    "notebook_save_repair_round_trip_packet";

/// Repo-relative path to the checked-in save-repair-round-trip packet JSON.
pub const NOTEBOOK_SAVE_REPAIR_ROUND_TRIP_PACKET_PATH: &str =
    "artifacts/notebook/m5/implement_notebook_save_repair_and_round_trip_safety_for_metadata_attachments_and_unknown_namespaces.json";

/// Embedded checked-in save-repair-round-trip packet JSON.
pub const NOTEBOOK_SAVE_REPAIR_ROUND_TRIP_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/implement_notebook_save_repair_and_round_trip_safety_for_metadata_attachments_and_unknown_namespaces.json"
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
    /// Save-kind class. Distinguishes full explicit saves, auto-saves,
    /// checkpoint saves, and derived-format exports so the row never implies
    /// a non-canonical export is a safe round-trip save.
    NotebookSaveKindClass {
        FullSave => "full_save",
        AutoSave => "auto_save",
        CheckpointSave => "checkpoint_save",
        ExportDerivedFormat => "export_derived_format",
    }
);

closed_vocab!(
    /// Metadata-preservation class. Names whether notebook metadata (kernelspec,
    /// language_info, aureline namespaces, and vendor namespaces) was preserved,
    /// partially lost with an explicit note, explicitly dropped with a note, or
    /// blocked by a documented format boundary.
    NotebookMetadataPreservationClass {
        Preserved => "preserved",
        PartialLossExplicit => "partial_loss_explicit",
        ExplicitLossWithNote => "explicit_loss_with_note",
        BlockedByFormatBoundary => "blocked_by_format_boundary",
    }
);

closed_vocab!(
    /// Attachment-preservation class. Names whether cell attachments survived
    /// the save, were externalized with an explicit note, dropped with a note,
    /// or blocked by a format boundary.
    NotebookAttachmentPreservationClass {
        Preserved => "preserved",
        ExternalizedWithNote => "externalized_with_note",
        DroppedWithNote => "dropped_with_note",
        BlockedByFormatBoundary => "blocked_by_format_boundary",
    }
);

closed_vocab!(
    /// Unknown-namespace preservation class. Names whether unknown metadata
    /// namespaces survived the save, were filtered with an explicit note,
    /// dropped with a note, or blocked by a format boundary.
    NotebookUnknownNamespacePreservationClass {
        Preserved => "preserved",
        FilteredWithNote => "filtered_with_note",
        DroppedWithNote => "dropped_with_note",
        BlockedByFormatBoundary => "blocked_by_format_boundary",
    }
);

closed_vocab!(
    /// Repair-kind class. Names the specific repair applied to a damaged or
    /// invalid notebook so the user and audit pipeline know exactly what was
    /// changed and why.
    NotebookRepairKindClass {
        MintedMissingCellId => "minted_missing_cell_id",
        RestoredAttachmentReference => "restored_attachment_reference",
        ReconstructedMetadataNamespace => "reconstructed_metadata_namespace",
        RemovedCorruptCell => "removed_corrupt_cell",
        NormalizedCellOrder => "normalized_cell_order",
        RebuiltCellOrderDigest => "rebuilt_cell_order_digest",
        PreservedRawJsonFallback => "preserved_raw_json_fallback",
    }
);

closed_vocab!(
    /// Repair-consequence class. Names whether the repair was lossless, lossy
    /// with an explicit note, or lossy with a silent fallback. Silent fallback
    /// is non-conforming and must surface as a finding.
    NotebookRepairConsequenceClass {
        Lossless => "lossless",
        LossyWithExplicitNote => "lossy_with_explicit_note",
        LossyWithSilentFallback => "lossy_with_silent_fallback",
    }
);

closed_vocab!(
    /// Round-trip assertion kind. Names the specific property being asserted
    /// across an open/edit/save cycle.
    NotebookRoundTripAssertionKindClass {
        MetadataSurvives => "metadata_survives",
        AttachmentSurvives => "attachment_survives",
        UnknownNamespaceSurvives => "unknown_namespace_survives",
        CellOrderSurvives => "cell_order_survives",
        CellIdSurvives => "cell_id_survives",
        SourceSurvives => "source_survives",
        OutputSurvives => "output_survives",
    }
);

closed_vocab!(
    /// Round-trip result class. Names whether the assertion passed, failed,
    /// produced a partial result, or was blocked by a documented format boundary.
    NotebookRoundTripResultClass {
        Pass => "pass",
        Fail => "fail",
        Partial => "partial",
        BlockedByFormatBoundary => "blocked_by_format_boundary",
    }
);

/// Generic finding shape used by every record validator in this module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookSaveRepairFinding {
    /// Stable check id (e.g. `notebook_save_operation.round_trip_safe_required`).
    pub check_id: String,
    /// Subject row id (record id, operation id, assertion id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl NotebookSaveRepairFinding {
    fn new(
        check_id: impl Into<String>,
        subject_ref: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            subject_ref: subject_ref.into(),
            message: message.into(),
        }
    }
}

/// Typed validation finding for a [`NotebookSaveOperation`].
pub type NotebookSaveOperationFinding = NotebookSaveRepairFinding;

/// Typed validation finding for a [`NotebookRepairAction`].
pub type NotebookRepairActionFinding = NotebookSaveRepairFinding;

/// Typed validation finding for a [`NotebookRoundTripAssertion`].
pub type NotebookRoundTripAssertionFinding = NotebookSaveRepairFinding;

/// Typed validation finding for a [`NotebookSaveRepairRoundTripPacket`].
pub type NotebookSaveRepairRoundTripPacketFinding = NotebookSaveRepairFinding;

/// Canonical notebook save-operation record. Describes a save, the preservation
/// posture for metadata, attachments, and unknown namespaces, and whether the
/// operation is round-trip safe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookSaveOperation {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_save_repair_schema_version: u32,
    /// Stable opaque save-operation id.
    pub save_operation_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Save-kind class.
    pub save_kind_class: NotebookSaveKindClass,
    /// Metadata-preservation class.
    pub metadata_preservation_class: NotebookMetadataPreservationClass,
    /// Attachment-preservation class.
    pub attachment_preservation_class: NotebookAttachmentPreservationClass,
    /// Unknown-namespace preservation class.
    pub unknown_namespace_preservation_class: NotebookUnknownNamespacePreservationClass,
    /// Whether the save is believed to be round-trip safe.
    pub round_trip_safe: bool,
    /// Opaque refs to repair actions taken before or during this save.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repair_action_refs: Vec<String>,
    /// Loss summary when preservation is partial, explicit loss, or blocked;
    /// MUST be `None` when all preservation classes are `Preserved`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loss_summary: Option<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookSaveOperation {
    /// Returns typed truth-rule findings; an empty vector means the operation is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookSaveOperationFinding> {
        let mut findings = Vec::new();
        let subject = self.save_operation_id.as_str();

        if self.record_kind != NOTEBOOK_SAVE_OPERATION_RECORD_KIND {
            findings.push(NotebookSaveOperationFinding::new(
                "notebook_save_operation.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_SAVE_OPERATION_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_save_repair_schema_version != NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION {
            findings.push(NotebookSaveOperationFinding::new(
                "notebook_save_operation.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION}, found {}",
                    self.notebook_save_repair_schema_version
                ),
            ));
        }

        if matches!(self.save_kind_class, NotebookSaveKindClass::ExportDerivedFormat)
            && self.round_trip_safe
        {
            findings.push(NotebookSaveOperationFinding::new(
                "notebook_save_operation.export_not_round_trip_safe",
                subject,
                "export_derived_format saves must not claim round_trip_safe=true",
            ));
        }

        let any_loss = matches!(
            self.metadata_preservation_class,
            NotebookMetadataPreservationClass::PartialLossExplicit
                | NotebookMetadataPreservationClass::ExplicitLossWithNote
                | NotebookMetadataPreservationClass::BlockedByFormatBoundary
        ) || matches!(
            self.attachment_preservation_class,
            NotebookAttachmentPreservationClass::ExternalizedWithNote
                | NotebookAttachmentPreservationClass::DroppedWithNote
                | NotebookAttachmentPreservationClass::BlockedByFormatBoundary
        ) || matches!(
            self.unknown_namespace_preservation_class,
            NotebookUnknownNamespacePreservationClass::FilteredWithNote
                | NotebookUnknownNamespacePreservationClass::DroppedWithNote
                | NotebookUnknownNamespacePreservationClass::BlockedByFormatBoundary
        );

        if any_loss && self.loss_summary.is_none() {
            findings.push(NotebookSaveOperationFinding::new(
                "notebook_save_operation.loss_summary_required",
                subject,
                "any non-preserved preservation class requires a loss_summary",
            ));
        }

        if !any_loss && self.loss_summary.is_some() {
            findings.push(NotebookSaveOperationFinding::new(
                "notebook_save_operation.loss_summary_not_allowed",
                subject,
                "all preservation classes are preserved; loss_summary must be None",
            ));
        }

        findings
    }
}

/// Canonical notebook repair-action record. Describes a repair applied to a
/// damaged or invalid notebook, the kind of repair, and its consequence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookRepairAction {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_save_repair_schema_version: u32,
    /// Stable opaque repair-action id.
    pub repair_action_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Repair-kind class.
    pub repair_kind_class: NotebookRepairKindClass,
    /// Repair-consequence class.
    pub consequence_class: NotebookRepairConsequenceClass,
    /// Whether the repair was actually applied.
    pub applied: bool,
    /// Opaque ref to the save operation that incorporated this repair, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incorporated_into_save_operation_ref: Option<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookRepairAction {
    /// Returns typed truth-rule findings; an empty vector means the action is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookRepairActionFinding> {
        let mut findings = Vec::new();
        let subject = self.repair_action_id.as_str();

        if self.record_kind != NOTEBOOK_REPAIR_ACTION_RECORD_KIND {
            findings.push(NotebookRepairActionFinding::new(
                "notebook_repair_action.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_REPAIR_ACTION_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_save_repair_schema_version != NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION {
            findings.push(NotebookRepairActionFinding::new(
                "notebook_repair_action.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION}, found {}",
                    self.notebook_save_repair_schema_version
                ),
            ));
        }

        if matches!(
            self.consequence_class,
            NotebookRepairConsequenceClass::LossyWithSilentFallback
        ) {
            findings.push(NotebookRepairActionFinding::new(
                "notebook_repair_action.silent_fallback_non_conforming",
                subject,
                "lossy_with_silent_fallback is non-conforming; repairs must be explicit",
            ));
        }

        if !self.applied
            && matches!(
                self.consequence_class,
                NotebookRepairConsequenceClass::Lossless
            )
        {
            findings.push(NotebookRepairActionFinding::new(
                "notebook_repair_action.lossless_not_applied",
                subject,
                "lossless repairs that were not applied are inconsistent",
            ));
        }

        findings
    }
}

/// Canonical notebook round-trip assertion record. Asserts whether a specific
/// property survives an open/edit/save cycle, with an explicit result class
/// and loss summary when preservation is partial or blocked.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookRoundTripAssertion {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_save_repair_schema_version: u32,
    /// Stable opaque assertion id.
    pub assertion_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Round-trip assertion kind.
    pub assertion_kind_class: NotebookRoundTripAssertionKindClass,
    /// Result class.
    pub result_class: NotebookRoundTripResultClass,
    /// Loss summary when result is `partial`, `fail`, or `blocked_by_format_boundary`;
    /// MUST be `None` when result is `pass`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loss_summary: Option<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookRoundTripAssertion {
    /// Returns typed truth-rule findings; an empty vector means the assertion is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookRoundTripAssertionFinding> {
        let mut findings = Vec::new();
        let subject = self.assertion_id.as_str();

        if self.record_kind != NOTEBOOK_ROUND_TRIP_ASSERTION_RECORD_KIND {
            findings.push(NotebookRoundTripAssertionFinding::new(
                "notebook_round_trip_assertion.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_ROUND_TRIP_ASSERTION_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_save_repair_schema_version != NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION {
            findings.push(NotebookRoundTripAssertionFinding::new(
                "notebook_round_trip_assertion.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION}, found {}",
                    self.notebook_save_repair_schema_version
                ),
            ));
        }

        let needs_loss_summary = !matches!(self.result_class, NotebookRoundTripResultClass::Pass);
        if needs_loss_summary && self.loss_summary.is_none() {
            findings.push(NotebookRoundTripAssertionFinding::new(
                "notebook_round_trip_assertion.loss_summary_required",
                subject,
                "non-pass results require a loss_summary",
            ));
        }
        if !needs_loss_summary && self.loss_summary.is_some() {
            findings.push(NotebookRoundTripAssertionFinding::new(
                "notebook_round_trip_assertion.loss_summary_not_allowed",
                subject,
                "pass results must not carry a loss_summary",
            ));
        }

        findings
    }
}

/// Checked-in save-repair-round-trip packet that downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookSaveRepairRoundTripPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: save-kind classes.
    pub save_kind_classes: Vec<NotebookSaveKindClass>,
    /// Closed vocabulary: metadata-preservation classes.
    pub metadata_preservation_classes: Vec<NotebookMetadataPreservationClass>,
    /// Closed vocabulary: attachment-preservation classes.
    pub attachment_preservation_classes: Vec<NotebookAttachmentPreservationClass>,
    /// Closed vocabulary: unknown-namespace preservation classes.
    pub unknown_namespace_preservation_classes: Vec<NotebookUnknownNamespacePreservationClass>,
    /// Closed vocabulary: repair-kind classes.
    pub repair_kind_classes: Vec<NotebookRepairKindClass>,
    /// Closed vocabulary: repair-consequence classes.
    pub repair_consequence_classes: Vec<NotebookRepairConsequenceClass>,
    /// Closed vocabulary: round-trip assertion-kind classes.
    pub round_trip_assertion_kind_classes: Vec<NotebookRoundTripAssertionKindClass>,
    /// Closed vocabulary: round-trip result classes.
    pub round_trip_result_classes: Vec<NotebookRoundTripResultClass>,
    /// Worked example save operations.
    pub example_save_operations: Vec<NotebookSaveOperation>,
    /// Worked example repair actions.
    pub example_repair_actions: Vec<NotebookRepairAction>,
    /// Worked example round-trip assertions.
    pub example_round_trip_assertions: Vec<NotebookRoundTripAssertion>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookSaveRepairRoundTripPacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<NotebookSaveRepairRoundTripPacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION {
            findings.push(NotebookSaveRepairRoundTripPacketFinding::new(
                "notebook_save_repair_round_trip_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != NOTEBOOK_SAVE_REPAIR_ROUND_TRIP_PACKET_RECORD_KIND {
            findings.push(NotebookSaveRepairRoundTripPacketFinding::new(
                "notebook_save_repair_round_trip_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_SAVE_REPAIR_ROUND_TRIP_PACKET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }

        if self.save_kind_classes.len() != NotebookSaveKindClass::ALL.len() {
            findings.push(NotebookSaveRepairRoundTripPacketFinding::new(
                "notebook_save_repair_round_trip_packet.save_kind_classes_coverage",
                subject,
                "save_kind_classes must list every variant",
            ));
        }
        if self.metadata_preservation_classes.len() != NotebookMetadataPreservationClass::ALL.len() {
            findings.push(NotebookSaveRepairRoundTripPacketFinding::new(
                "notebook_save_repair_round_trip_packet.metadata_preservation_classes_coverage",
                subject,
                "metadata_preservation_classes must list every variant",
            ));
        }
        if self.attachment_preservation_classes.len() != NotebookAttachmentPreservationClass::ALL.len() {
            findings.push(NotebookSaveRepairRoundTripPacketFinding::new(
                "notebook_save_repair_round_trip_packet.attachment_preservation_classes_coverage",
                subject,
                "attachment_preservation_classes must list every variant",
            ));
        }
        if self.unknown_namespace_preservation_classes.len() != NotebookUnknownNamespacePreservationClass::ALL.len() {
            findings.push(NotebookSaveRepairRoundTripPacketFinding::new(
                "notebook_save_repair_round_trip_packet.unknown_namespace_preservation_classes_coverage",
                subject,
                "unknown_namespace_preservation_classes must list every variant",
            ));
        }
        if self.repair_kind_classes.len() != NotebookRepairKindClass::ALL.len() {
            findings.push(NotebookSaveRepairRoundTripPacketFinding::new(
                "notebook_save_repair_round_trip_packet.repair_kind_classes_coverage",
                subject,
                "repair_kind_classes must list every variant",
            ));
        }
        if self.repair_consequence_classes.len() != NotebookRepairConsequenceClass::ALL.len() {
            findings.push(NotebookSaveRepairRoundTripPacketFinding::new(
                "notebook_save_repair_round_trip_packet.repair_consequence_classes_coverage",
                subject,
                "repair_consequence_classes must list every variant",
            ));
        }
        if self.round_trip_assertion_kind_classes.len() != NotebookRoundTripAssertionKindClass::ALL.len() {
            findings.push(NotebookSaveRepairRoundTripPacketFinding::new(
                "notebook_save_repair_round_trip_packet.round_trip_assertion_kind_classes_coverage",
                subject,
                "round_trip_assertion_kind_classes must list every variant",
            ));
        }
        if self.round_trip_result_classes.len() != NotebookRoundTripResultClass::ALL.len() {
            findings.push(NotebookSaveRepairRoundTripPacketFinding::new(
                "notebook_save_repair_round_trip_packet.round_trip_result_classes_coverage",
                subject,
                "round_trip_result_classes must list every variant",
            ));
        }

        for op in &self.example_save_operations {
            findings.extend(op.validate().into_iter().map(|f| {
                NotebookSaveRepairRoundTripPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for action in &self.example_repair_actions {
            findings.extend(action.validate().into_iter().map(|f| {
                NotebookSaveRepairRoundTripPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for assertion in &self.example_round_trip_assertions {
            findings.extend(assertion.validate().into_iter().map(|f| {
                NotebookSaveRepairRoundTripPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        findings
    }
}

impl NotebookSaveKindClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [Self::FullSave, Self::AutoSave, Self::CheckpointSave, Self::ExportDerivedFormat];
}

impl NotebookMetadataPreservationClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Preserved,
        Self::PartialLossExplicit,
        Self::ExplicitLossWithNote,
        Self::BlockedByFormatBoundary,
    ];
}

impl NotebookAttachmentPreservationClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Preserved,
        Self::ExternalizedWithNote,
        Self::DroppedWithNote,
        Self::BlockedByFormatBoundary,
    ];
}

impl NotebookUnknownNamespacePreservationClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Preserved,
        Self::FilteredWithNote,
        Self::DroppedWithNote,
        Self::BlockedByFormatBoundary,
    ];
}

impl NotebookRepairKindClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::MintedMissingCellId,
        Self::RestoredAttachmentReference,
        Self::ReconstructedMetadataNamespace,
        Self::RemovedCorruptCell,
        Self::NormalizedCellOrder,
        Self::RebuiltCellOrderDigest,
        Self::PreservedRawJsonFallback,
    ];
}

impl NotebookRepairConsequenceClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 3] = [Self::Lossless, Self::LossyWithExplicitNote, Self::LossyWithSilentFallback];
}

impl NotebookRoundTripAssertionKindClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::MetadataSurvives,
        Self::AttachmentSurvives,
        Self::UnknownNamespaceSurvives,
        Self::CellOrderSurvives,
        Self::CellIdSurvives,
        Self::SourceSurvives,
        Self::OutputSurvives,
    ];
}

impl NotebookRoundTripResultClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [Self::Pass, Self::Fail, Self::Partial, Self::BlockedByFormatBoundary];
}

/// Parses the checked-in save-repair-round-trip packet JSON.
pub fn current_notebook_save_repair_round_trip_packet(
) -> Result<NotebookSaveRepairRoundTripPacket, serde_json::Error> {
    serde_json::from_str(NOTEBOOK_SAVE_REPAIR_ROUND_TRIP_PACKET_JSON)
}

#[cfg(test)]
mod tests;
