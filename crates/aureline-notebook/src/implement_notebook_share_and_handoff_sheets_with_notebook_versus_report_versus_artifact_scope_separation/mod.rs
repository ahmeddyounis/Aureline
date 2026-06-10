//! Notebook share and handoff sheets with notebook-versus-report-versus-artifact
//! scope separation.
//!
//! This module materializes the typed records that keep notebook sharing and
//! handoff honest about what scope is being transferred — live notebook,
//! captured report, or derived artifact — and what redactions or downgrades
//! apply before the transfer. The records and closed vocabularies here mirror
//! the boundary schema at
//! `/schemas/notebook/implement_notebook_share_and_handoff_sheets_with_notebook_versus_report_versus_artifact_scope_separation.schema.json`
//! and reuse the session-role and admission vocabulary already frozen in
//! `/schemas/collab/session_role_admission_and_retention_qualification.schema.json`.
//!
//! The module exposes:
//!
//! - the [`NotebookShareSheet`] record that carries a share action’s scope,
//!   posture, redaction explanation, and recipient set so the sharing surface
//!   never silently broadens or narrows what is being shared;
//! - the [`NotebookHandoffSheet`] record that carries a handoff action’s
//!   scope, posture, sender, recipient, and state so the handoff surface always
//!   knows whether the transfer is pending, accepted, declined, expired, or
//!   revoked;
//! - the [`NotebookScopeClass`] closed vocabulary that names the three
//!   separable scopes — notebook, report, artifact — so consumers never
//!   conflate live runtime state with captured output or derived exports;
//! - the [`NotebookSharePostureClass`] closed vocabulary that names the
//!   redaction and downgrade posture of a share so the UI shows
//!   redaction-before-share labels instead of optimistic placeholder language;
//! - the [`NotebookHandoffPostureClass`] closed vocabulary that names the
//!   lifecycle state of a handoff so participants know whether the transfer is
//!   still actionable;
//! - the [`NotebookShareAndHandoffPacket`] checked-in artifact that downstream
//!   docs, help, support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT
//! appear on any record carried here. Only opaque handles and closed-
//! vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every share/handoff record carried by this
/// module. Bumped only on breaking payload changes; additive-optional fields
/// do not bump this value.
pub const NOTEBOOK_SHARE_HANDOFF_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`NotebookShareSheet`] payloads.
pub const NOTEBOOK_SHARE_SHEET_RECORD_KIND: &str = "notebook_share_sheet";

/// Stable record-kind tag for serialized [`NotebookHandoffSheet`] payloads.
pub const NOTEBOOK_HANDOFF_SHEET_RECORD_KIND: &str = "notebook_handoff_sheet";

/// Stable record-kind tag for the checked-in [`NotebookShareAndHandoffPacket`].
pub const NOTEBOOK_SHARE_HANDOFF_PACKET_RECORD_KIND: &str = "notebook_share_and_handoff_packet";

/// Repo-relative path to the checked-in share/handoff packet JSON.
pub const NOTEBOOK_SHARE_HANDOFF_PACKET_PATH: &str =
    "artifacts/notebook/m5/implement_notebook_share_and_handoff_sheets_with_notebook_versus_report_versus_artifact_scope_separation.json";

/// Embedded checked-in share/handoff packet JSON.
pub const NOTEBOOK_SHARE_HANDOFF_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/implement_notebook_share_and_handoff_sheets_with_notebook_versus_report_versus_artifact_scope_separation.json"
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
    /// Scope class. Names the separable scope of a share or handoff so
    /// consumers never conflate live runtime state with captured output or
    /// derived exports.
    NotebookScopeClass {
        Notebook => "notebook",
        Report => "report",
        Artifact => "artifact",
    }
);

closed_vocab!(
    /// Share-posture class. Names the redaction and downgrade posture of a
    /// share so the UI shows redaction-before-share labels instead of
    /// optimistic placeholder language.
    NotebookSharePostureClass {
        RedactedBeforeShare => "redacted_before_share",
        FullDocument => "full_document",
        ExportOnly => "export_only",
        DegradedScope => "degraded_scope",
    }
);

closed_vocab!(
    /// Handoff-posture class. Names the lifecycle state of a handoff so
    /// participants know whether the transfer is still actionable.
    NotebookHandoffPostureClass {
        Pending => "pending",
        Accepted => "accepted",
        Declined => "declined",
        Expired => "expired",
        Revoked => "revoked",
    }
);

/// Generic finding shape used by every share/handoff validator. Mirrors the
/// finding shapes other Aureline crates expose so a single review/audit/support
/// pipeline can consume them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShareHandoffFinding {
    /// Stable check id (e.g. `notebook_share_sheet.scope_required`).
    pub check_id: String,
    /// Subject row id (record id, sheet id, document id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl ShareHandoffFinding {
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

/// Typed validation finding for a [`NotebookShareSheet`].
pub type NotebookShareSheetFinding = ShareHandoffFinding;

/// Typed validation finding for a [`NotebookHandoffSheet`].
pub type NotebookHandoffSheetFinding = ShareHandoffFinding;

/// Typed validation finding for a [`NotebookShareAndHandoffPacket`].
pub type NotebookShareAndHandoffPacketFinding = ShareHandoffFinding;

/// Notebook share-sheet record. Carries a share action’s scope, posture,
/// redaction explanation, and recipient set so the sharing surface never
/// silently broadens or narrows what is being shared.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookShareSheet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_share_handoff_schema_version: u32,
    /// Stable opaque share-sheet id.
    pub share_sheet_id: String,
    /// Opaque ref to the notebook document being shared.
    pub document_id_ref: String,
    /// Opaque ref to the actor initiating the share.
    pub sharer_actor_ref: String,
    /// Opaque refs to the recipients of the share.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipient_refs: Vec<String>,
    /// Scope class — notebook, report, or artifact.
    pub scope_class: NotebookScopeClass,
    /// Share-posture class.
    pub share_posture: NotebookSharePostureClass,
    /// Export-safe explanation when share_posture is
    /// [`NotebookSharePostureClass::RedactedBeforeShare`] or
    /// [`NotebookSharePostureClass::DegradedScope`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redaction_explanation: Option<String>,
    /// Opaque refs to the cells included in the share, when scope is partial.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cell_scope_refs: Vec<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookShareSheet {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookShareSheetFinding> {
        let mut findings = Vec::new();
        let subject = self.share_sheet_id.as_str();

        if self.record_kind != NOTEBOOK_SHARE_SHEET_RECORD_KIND {
            findings.push(NotebookShareSheetFinding::new(
                "notebook_share_sheet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_SHARE_SHEET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_share_handoff_schema_version != NOTEBOOK_SHARE_HANDOFF_SCHEMA_VERSION {
            findings.push(NotebookShareSheetFinding::new(
                "notebook_share_sheet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_SHARE_HANDOFF_SCHEMA_VERSION}, found {}",
                    self.notebook_share_handoff_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookShareSheetFinding::new(
                "notebook_share_sheet.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }
        if self.sharer_actor_ref.trim().is_empty() {
            findings.push(NotebookShareSheetFinding::new(
                "notebook_share_sheet.sharer_actor_ref_required",
                subject,
                "sharer_actor_ref must be non-empty",
            ));
        }
        if self.recipient_refs.is_empty() {
            findings.push(NotebookShareSheetFinding::new(
                "notebook_share_sheet.recipient_refs_required",
                subject,
                "recipient_refs must not be empty",
            ));
        }
        for (i, r) in self.recipient_refs.iter().enumerate() {
            if r.trim().is_empty() {
                findings.push(NotebookShareSheetFinding::new(
                    format!("notebook_share_sheet.recipient_refs[{i}]_non_empty"),
                    subject,
                    format!("recipient_refs[{i}] must be non-empty"),
                ));
            }
        }

        if (self.share_posture == NotebookSharePostureClass::RedactedBeforeShare
            || self.share_posture == NotebookSharePostureClass::DegradedScope)
            && self.redaction_explanation.is_none()
        {
            findings.push(NotebookShareSheetFinding::new(
                "notebook_share_sheet.redaction_explanation_required",
                subject,
                "redaction_explanation must be Some when share_posture is redacted_before_share or degraded_scope",
            ));
        }

        if self.share_posture == NotebookSharePostureClass::ExportOnly
            && self.scope_class != NotebookScopeClass::Artifact
            && self.scope_class != NotebookScopeClass::Report
        {
            findings.push(NotebookShareSheetFinding::new(
                "notebook_share_sheet.export_only_scope_invariant",
                subject,
                "share_posture export_only requires scope_class to be report or artifact",
            ));
        }

        if self.summary.trim().is_empty() {
            findings.push(NotebookShareSheetFinding::new(
                "notebook_share_sheet.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Notebook handoff-sheet record. Carries a handoff action’s scope, posture,
/// sender, recipient, and state so the handoff surface always knows whether the
/// transfer is pending, accepted, declined, expired, or revoked.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookHandoffSheet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_share_handoff_schema_version: u32,
    /// Stable opaque handoff-sheet id.
    pub handoff_sheet_id: String,
    /// Opaque ref to the notebook document being handed off.
    pub document_id_ref: String,
    /// Opaque ref to the actor sending the handoff.
    pub sender_actor_ref: String,
    /// Opaque ref to the actor receiving the handoff.
    pub recipient_actor_ref: String,
    /// Scope class — notebook, report, or artifact.
    pub scope_class: NotebookScopeClass,
    /// Handoff-posture class.
    pub handoff_posture: NotebookHandoffPostureClass,
    /// Export-safe explanation when handoff_posture is
    /// [`NotebookHandoffPostureClass::Declined`] or
    /// [`NotebookHandoffPostureClass::Revoked`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handoff_explanation: Option<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookHandoffSheet {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookHandoffSheetFinding> {
        let mut findings = Vec::new();
        let subject = self.handoff_sheet_id.as_str();

        if self.record_kind != NOTEBOOK_HANDOFF_SHEET_RECORD_KIND {
            findings.push(NotebookHandoffSheetFinding::new(
                "notebook_handoff_sheet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_HANDOFF_SHEET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_share_handoff_schema_version != NOTEBOOK_SHARE_HANDOFF_SCHEMA_VERSION {
            findings.push(NotebookHandoffSheetFinding::new(
                "notebook_handoff_sheet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_SHARE_HANDOFF_SCHEMA_VERSION}, found {}",
                    self.notebook_share_handoff_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookHandoffSheetFinding::new(
                "notebook_handoff_sheet.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }
        if self.sender_actor_ref.trim().is_empty() {
            findings.push(NotebookHandoffSheetFinding::new(
                "notebook_handoff_sheet.sender_actor_ref_required",
                subject,
                "sender_actor_ref must be non-empty",
            ));
        }
        if self.recipient_actor_ref.trim().is_empty() {
            findings.push(NotebookHandoffSheetFinding::new(
                "notebook_handoff_sheet.recipient_actor_ref_required",
                subject,
                "recipient_actor_ref must be non-empty",
            ));
        }

        if (self.handoff_posture == NotebookHandoffPostureClass::Declined
            || self.handoff_posture == NotebookHandoffPostureClass::Revoked)
            && self.handoff_explanation.is_none()
        {
            findings.push(NotebookHandoffSheetFinding::new(
                "notebook_handoff_sheet.handoff_explanation_required",
                subject,
                "handoff_explanation must be Some when handoff_posture is declined or revoked",
            ));
        }

        if self.summary.trim().is_empty() {
            findings.push(NotebookHandoffSheetFinding::new(
                "notebook_handoff_sheet.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Checked-in share and handoff packet that downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookShareAndHandoffPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: scope classes.
    pub scope_classes: Vec<NotebookScopeClass>,
    /// Closed vocabulary: share posture classes.
    pub share_posture_classes: Vec<NotebookSharePostureClass>,
    /// Closed vocabulary: handoff posture classes.
    pub handoff_posture_classes: Vec<NotebookHandoffPostureClass>,
    /// Worked example share sheets.
    pub example_share_sheets: Vec<NotebookShareSheet>,
    /// Worked example handoff sheets.
    pub example_handoff_sheets: Vec<NotebookHandoffSheet>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookShareAndHandoffPacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<NotebookShareAndHandoffPacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_SHARE_HANDOFF_SCHEMA_VERSION {
            findings.push(NotebookShareAndHandoffPacketFinding::new(
                "notebook_share_and_handoff_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_SHARE_HANDOFF_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != NOTEBOOK_SHARE_HANDOFF_PACKET_RECORD_KIND {
            findings.push(NotebookShareAndHandoffPacketFinding::new(
                "notebook_share_and_handoff_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_SHARE_HANDOFF_PACKET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }

        if self.scope_classes.len() != NotebookScopeClass::ALL.len() {
            findings.push(NotebookShareAndHandoffPacketFinding::new(
                "notebook_share_and_handoff_packet.scope_classes_coverage",
                subject,
                "scope_classes must list every variant",
            ));
        }
        if self.share_posture_classes.len() != NotebookSharePostureClass::ALL.len() {
            findings.push(NotebookShareAndHandoffPacketFinding::new(
                "notebook_share_and_handoff_packet.share_posture_classes_coverage",
                subject,
                "share_posture_classes must list every variant",
            ));
        }
        if self.handoff_posture_classes.len() != NotebookHandoffPostureClass::ALL.len() {
            findings.push(NotebookShareAndHandoffPacketFinding::new(
                "notebook_share_and_handoff_packet.handoff_posture_classes_coverage",
                subject,
                "handoff_posture_classes must list every variant",
            ));
        }

        for sheet in &self.example_share_sheets {
            findings.extend(sheet.validate().into_iter().map(|f| {
                NotebookShareAndHandoffPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for sheet in &self.example_handoff_sheets {
            findings.extend(sheet.validate().into_iter().map(|f| {
                NotebookShareAndHandoffPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        findings
    }
}

/// Parses the checked-in share and handoff packet JSON.
pub fn current_notebook_share_and_handoff_packet(
) -> Result<NotebookShareAndHandoffPacket, serde_json::Error> {
    serde_json::from_str(NOTEBOOK_SHARE_HANDOFF_PACKET_JSON)
}

impl NotebookScopeClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 3] = [Self::Notebook, Self::Report, Self::Artifact];
}

impl NotebookSharePostureClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RedactedBeforeShare,
        Self::FullDocument,
        Self::ExportOnly,
        Self::DegradedScope,
    ];
}

impl NotebookHandoffPostureClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Pending,
        Self::Accepted,
        Self::Declined,
        Self::Expired,
        Self::Revoked,
    ];
}

#[cfg(test)]
mod tests;
