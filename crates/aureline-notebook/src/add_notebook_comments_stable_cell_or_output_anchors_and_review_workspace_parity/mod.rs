//! Notebook comments, stable cell or output anchors, and review-workspace parity.
//!
//! This module materializes the typed records that keep notebook review,
//! collaboration, and experiment lineage honest about cell-aware comments,
//! stable anchors, runtime-boundary truth, and review-workspace parity. The
//! records and closed vocabularies here mirror the boundary schema at
//! `/schemas/notebook/add_notebook_comments_stable_cell_or_output_anchors_and_review_workspace_parity.schema.json`
//! and reuse the cell-id stability vocabulary already frozen in
//! `/schemas/notebook/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability.schema.json`.
//!
//! The module exposes:
//!
//! - the [`NotebookComment`] record that carries a comment bound to a stable
//!   cell or output anchor, with status, thread state, and redaction posture
//!   so comments never drift silently when cells move or outputs are rerun;
//! - the [`NotebookAnchor`] record that carries a stable anchor to a cell or
//!   an output within a cell, with an [`NotebookAnchorKind`] discriminator so
//!   consumers know whether the anchor refers to source or captured runtime
//!   state;
//! - the [`NotebookReviewWorkspaceParity`] record that surfaces the parity
//!   between a notebook document and its review-workspace projection, with
//!   explicit downgrade reasons when stable ids, runtime bounds, or trust
//!   classes prevent full cell-aware review;
//! - the [`NotebookCommentAnchorPacket`] checked-in artifact that downstream
//!   docs, help, support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT
//! appear on any record carried here. Only opaque handles and closed-
//! vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every comment/anchor/parity record carried by
/// this module. Bumped only on breaking payload changes; additive-optional
/// fields do not bump this value.
pub const NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`NotebookComment`] payloads.
pub const NOTEBOOK_COMMENT_RECORD_KIND: &str = "notebook_comment";

/// Stable record-kind tag for serialized [`NotebookAnchor`] payloads.
pub const NOTEBOOK_ANCHOR_RECORD_KIND: &str = "notebook_anchor";

/// Stable record-kind tag for serialized [`NotebookReviewWorkspaceParity`] payloads.
pub const NOTEBOOK_REVIEW_WORKSPACE_PARITY_RECORD_KIND: &str = "notebook_review_workspace_parity";

/// Stable record-kind tag for the checked-in [`NotebookCommentAnchorPacket`].
pub const NOTEBOOK_COMMENT_ANCHOR_PACKET_RECORD_KIND: &str = "notebook_comment_anchor_packet";

/// Repo-relative path to the checked-in comment/anchor/parity packet JSON.
pub const NOTEBOOK_COMMENT_ANCHOR_PACKET_PATH: &str =
    "artifacts/notebook/m5/add_notebook_comments_stable_cell_or_output_anchors_and_review_workspace_parity.json";

/// Embedded checked-in comment/anchor/parity packet JSON.
pub const NOTEBOOK_COMMENT_ANCHOR_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/add_notebook_comments_stable_cell_or_output_anchors_and_review_workspace_parity.json"
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
    /// Comment-target class. Names whether the comment is anchored to a cell,
    /// an output within a cell, or notebook-level metadata.
    NotebookCommentTargetClass {
        Cell => "cell",
        Output => "output",
        NotebookMetadata => "notebook_metadata",
    }
);

closed_vocab!(
    /// Comment-status class. Names the lifecycle state of a comment so the
    /// review surface never presents a resolved or redacted comment as active.
    NotebookCommentStatusClass {
        Active => "active",
        Resolved => "resolved",
        Outdated => "outdated",
        Redacted => "redacted",
    }
);

closed_vocab!(
    /// Comment-thread-state class. Names whether the comment is a single
    /// standalone note, part of an open thread, resolved, or stale because the
    /// anchor has drifted.
    NotebookCommentThreadState {
        Single => "single",
        Open => "open",
        ThreadResolved => "thread_resolved",
        Stale => "stale",
    }
);

closed_vocab!(
    /// Anchor-kind class. Names whether the anchor points to a cell or to an
    /// output within a cell so consumers know which runtime boundary applies.
    NotebookAnchorKind {
        Cell => "cell",
        Output => "output",
    }
);

closed_vocab!(
    /// Review-workspace-parity class. Names the level of parity between the
    /// notebook document and its review-workspace projection.
    NotebookReviewWorkspaceParityClass {
        Full => "full",
        PartialCellAware => "partial_cell_aware",
        RawFallback => "raw_fallback",
        Degraded => "degraded",
    }
);

closed_vocab!(
    /// Review-workspace-downgrade-reason class. Names why review-workspace
    /// parity is less than full so the UI can show truthful degraded-state
    /// labels instead of optimistic placeholder language.
    NotebookReviewWorkspaceDowngradeReason {
        MissingStableIds => "missing_stable_ids",
        RuntimeBound => "runtime_bound",
        OutputUntrusted => "output_untrusted",
        Redacted => "redacted",
        KernelUnavailable => "kernel_unavailable",
    }
);

/// Generic finding shape used by every comment/anchor/parity validator.
/// Mirrors the finding shapes other Aureline crates expose so a single
/// review/audit/support pipeline can consume them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommentAnchorFinding {
    /// Stable check id (e.g. `notebook_comment.anchor_ref_required`).
    pub check_id: String,
    /// Subject row id (record id, comment id, anchor id, parity id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl CommentAnchorFinding {
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

/// Typed validation finding for a [`NotebookComment`].
pub type NotebookCommentFinding = CommentAnchorFinding;

/// Typed validation finding for a [`NotebookAnchor`].
pub type NotebookAnchorFinding = CommentAnchorFinding;

/// Typed validation finding for a [`NotebookReviewWorkspaceParity`].
pub type NotebookReviewWorkspaceParityFinding = CommentAnchorFinding;

/// Typed validation finding for a [`NotebookCommentAnchorPacket`].
pub type NotebookCommentAnchorPacketFinding = CommentAnchorFinding;

/// Notebook comment record. Carries a comment bound to a stable cell or output
/// anchor, with status, thread state, and redaction posture so comments never
/// drift silently when cells move or outputs are rerun.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookComment {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_comment_anchor_schema_version: u32,
    /// Stable opaque comment id.
    pub comment_id: String,
    /// Opaque ref to the notebook document that owns this comment.
    pub document_id_ref: String,
    /// Opaque ref to the [`NotebookAnchor`] this comment is bound to.
    pub anchor_ref: String,
    /// Comment-target class.
    pub comment_target_class: NotebookCommentTargetClass,
    /// Comment-status class.
    pub comment_status: NotebookCommentStatusClass,
    /// Comment-thread-state class.
    pub thread_state: NotebookCommentThreadState,
    /// Opaque ref to the author actor.
    pub author_ref: String,
    /// Opaque refs to reply comments bound to this comment.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reply_refs: Vec<String>,
    /// Opaque ref to the redaction profile applied when sharing this comment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redaction_profile_ref: Option<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookComment {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookCommentFinding> {
        let mut findings = Vec::new();
        let subject = self.comment_id.as_str();

        if self.record_kind != NOTEBOOK_COMMENT_RECORD_KIND {
            findings.push(NotebookCommentFinding::new(
                "notebook_comment.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_COMMENT_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_comment_anchor_schema_version != NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION {
            findings.push(NotebookCommentFinding::new(
                "notebook_comment.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION}, found {}",
                    self.notebook_comment_anchor_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookCommentFinding::new(
                "notebook_comment.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }
        if self.anchor_ref.trim().is_empty() {
            findings.push(NotebookCommentFinding::new(
                "notebook_comment.anchor_ref_required",
                subject,
                "anchor_ref must be non-empty",
            ));
        }
        if self.author_ref.trim().is_empty() {
            findings.push(NotebookCommentFinding::new(
                "notebook_comment.author_ref_required",
                subject,
                "author_ref must be non-empty",
            ));
        }
        if self.summary.trim().is_empty() {
            findings.push(NotebookCommentFinding::new(
                "notebook_comment.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Stable notebook anchor record. Carries a durable anchor to a cell or to an
/// output within a cell, with an [`NotebookAnchorKind`] discriminator so
/// consumers know whether the anchor refers to source or captured runtime state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookAnchor {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_comment_anchor_schema_version: u32,
    /// Stable opaque anchor id.
    pub anchor_id: String,
    /// Opaque ref to the notebook document that owns this anchor.
    pub document_id_ref: String,
    /// Anchor-kind class.
    pub anchor_kind: NotebookAnchorKind,
    /// Opaque ref to the stable cell id this anchor targets.
    pub cell_id_ref: String,
    /// Opaque ref to the output handle, when `anchor_kind` is [`Output`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_handle_ref: Option<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookAnchor {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookAnchorFinding> {
        let mut findings = Vec::new();
        let subject = self.anchor_id.as_str();

        if self.record_kind != NOTEBOOK_ANCHOR_RECORD_KIND {
            findings.push(NotebookAnchorFinding::new(
                "notebook_anchor.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_ANCHOR_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_comment_anchor_schema_version != NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION {
            findings.push(NotebookAnchorFinding::new(
                "notebook_anchor.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION}, found {}",
                    self.notebook_comment_anchor_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookAnchorFinding::new(
                "notebook_anchor.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }
        if self.cell_id_ref.trim().is_empty() {
            findings.push(NotebookAnchorFinding::new(
                "notebook_anchor.cell_id_ref_required",
                subject,
                "cell_id_ref must be non-empty",
            ));
        }

        if self.anchor_kind == NotebookAnchorKind::Output && self.output_handle_ref.is_none() {
            findings.push(NotebookAnchorFinding::new(
                "notebook_anchor.output_handle_ref_required_for_output",
                subject,
                "output_handle_ref must be Some when anchor_kind is output",
            ));
        }

        if self.summary.trim().is_empty() {
            findings.push(NotebookAnchorFinding::new(
                "notebook_anchor.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Review-workspace parity record. Surfaces the parity between a notebook
/// document and its review-workspace projection, with explicit downgrade
/// reasons when stable ids, runtime bounds, or trust classes prevent full
/// cell-aware review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookReviewWorkspaceParity {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_comment_anchor_schema_version: u32,
    /// Stable opaque parity id.
    pub parity_id: String,
    /// Opaque ref to the notebook document.
    pub document_id_ref: String,
    /// Review-workspace-parity class.
    pub parity_class: NotebookReviewWorkspaceParityClass,
    /// Downgrade reasons when parity is not full.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub downgrade_reasons: Vec<NotebookReviewWorkspaceDowngradeReason>,
    /// Opaque refs to [`NotebookAnchor`] records known in the review workspace.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub anchor_refs: Vec<String>,
    /// Opaque refs to [`NotebookComment`] records known in the review workspace.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment_refs: Vec<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookReviewWorkspaceParity {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookReviewWorkspaceParityFinding> {
        let mut findings = Vec::new();
        let subject = self.parity_id.as_str();

        if self.record_kind != NOTEBOOK_REVIEW_WORKSPACE_PARITY_RECORD_KIND {
            findings.push(NotebookReviewWorkspaceParityFinding::new(
                "notebook_review_workspace_parity.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_REVIEW_WORKSPACE_PARITY_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_comment_anchor_schema_version != NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION {
            findings.push(NotebookReviewWorkspaceParityFinding::new(
                "notebook_review_workspace_parity.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION}, found {}",
                    self.notebook_comment_anchor_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookReviewWorkspaceParityFinding::new(
                "notebook_review_workspace_parity.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }

        if self.parity_class != NotebookReviewWorkspaceParityClass::Full
            && self.downgrade_reasons.is_empty()
        {
            findings.push(NotebookReviewWorkspaceParityFinding::new(
                "notebook_review_workspace_parity.downgrade_reasons_required_when_not_full",
                subject,
                "downgrade_reasons must not be empty when parity_class is not full",
            ));
        }

        if self.parity_class == NotebookReviewWorkspaceParityClass::Full
            && !self.downgrade_reasons.is_empty()
        {
            findings.push(NotebookReviewWorkspaceParityFinding::new(
                "notebook_review_workspace_parity.downgrade_reasons_must_be_empty_when_full",
                subject,
                "downgrade_reasons must be empty when parity_class is full",
            ));
        }

        if self.summary.trim().is_empty() {
            findings.push(NotebookReviewWorkspaceParityFinding::new(
                "notebook_review_workspace_parity.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Checked-in comment/anchor/parity packet that downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookCommentAnchorPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: comment target classes.
    pub comment_target_classes: Vec<NotebookCommentTargetClass>,
    /// Closed vocabulary: comment status classes.
    pub comment_status_classes: Vec<NotebookCommentStatusClass>,
    /// Closed vocabulary: comment thread states.
    pub comment_thread_states: Vec<NotebookCommentThreadState>,
    /// Closed vocabulary: anchor kinds.
    pub anchor_kinds: Vec<NotebookAnchorKind>,
    /// Closed vocabulary: review workspace parity classes.
    pub review_workspace_parity_classes: Vec<NotebookReviewWorkspaceParityClass>,
    /// Closed vocabulary: review workspace downgrade reasons.
    pub review_workspace_downgrade_reasons: Vec<NotebookReviewWorkspaceDowngradeReason>,
    /// Worked example comments.
    pub example_comments: Vec<NotebookComment>,
    /// Worked example anchors.
    pub example_anchors: Vec<NotebookAnchor>,
    /// Worked example review-workspace parity records.
    pub example_review_workspace_parities: Vec<NotebookReviewWorkspaceParity>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookCommentAnchorPacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<NotebookCommentAnchorPacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION {
            findings.push(NotebookCommentAnchorPacketFinding::new(
                "notebook_comment_anchor_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != NOTEBOOK_COMMENT_ANCHOR_PACKET_RECORD_KIND {
            findings.push(NotebookCommentAnchorPacketFinding::new(
                "notebook_comment_anchor_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_COMMENT_ANCHOR_PACKET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }

        if self.comment_target_classes.len() != NotebookCommentTargetClass::ALL.len() {
            findings.push(NotebookCommentAnchorPacketFinding::new(
                "notebook_comment_anchor_packet.comment_target_classes_coverage",
                subject,
                "comment_target_classes must list every variant",
            ));
        }
        if self.comment_status_classes.len() != NotebookCommentStatusClass::ALL.len() {
            findings.push(NotebookCommentAnchorPacketFinding::new(
                "notebook_comment_anchor_packet.comment_status_classes_coverage",
                subject,
                "comment_status_classes must list every variant",
            ));
        }
        if self.comment_thread_states.len() != NotebookCommentThreadState::ALL.len() {
            findings.push(NotebookCommentAnchorPacketFinding::new(
                "notebook_comment_anchor_packet.comment_thread_states_coverage",
                subject,
                "comment_thread_states must list every variant",
            ));
        }
        if self.anchor_kinds.len() != NotebookAnchorKind::ALL.len() {
            findings.push(NotebookCommentAnchorPacketFinding::new(
                "notebook_comment_anchor_packet.anchor_kinds_coverage",
                subject,
                "anchor_kinds must list every variant",
            ));
        }
        if self.review_workspace_parity_classes.len()
            != NotebookReviewWorkspaceParityClass::ALL.len()
        {
            findings.push(NotebookCommentAnchorPacketFinding::new(
                "notebook_comment_anchor_packet.review_workspace_parity_classes_coverage",
                subject,
                "review_workspace_parity_classes must list every variant",
            ));
        }
        if self.review_workspace_downgrade_reasons.len()
            != NotebookReviewWorkspaceDowngradeReason::ALL.len()
        {
            findings.push(NotebookCommentAnchorPacketFinding::new(
                "notebook_comment_anchor_packet.review_workspace_downgrade_reasons_coverage",
                subject,
                "review_workspace_downgrade_reasons must list every variant",
            ));
        }

        for comment in &self.example_comments {
            findings.extend(comment.validate().into_iter().map(|f| {
                NotebookCommentAnchorPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for anchor in &self.example_anchors {
            findings.extend(anchor.validate().into_iter().map(|f| {
                NotebookCommentAnchorPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for parity in &self.example_review_workspace_parities {
            findings.extend(parity.validate().into_iter().map(|f| {
                NotebookCommentAnchorPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        findings
    }
}

/// Parses the checked-in comment/anchor/parity packet JSON.
pub fn current_notebook_comment_anchor_packet(
) -> Result<NotebookCommentAnchorPacket, serde_json::Error> {
    serde_json::from_str(NOTEBOOK_COMMENT_ANCHOR_PACKET_JSON)
}

impl NotebookCommentTargetClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 3] = [Self::Cell, Self::Output, Self::NotebookMetadata];
}

impl NotebookCommentStatusClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [Self::Active, Self::Resolved, Self::Outdated, Self::Redacted];
}

impl NotebookCommentThreadState {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [Self::Single, Self::Open, Self::ThreadResolved, Self::Stale];
}

impl NotebookAnchorKind {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 2] = [Self::Cell, Self::Output];
}

impl NotebookReviewWorkspaceParityClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Full,
        Self::PartialCellAware,
        Self::RawFallback,
        Self::Degraded,
    ];
}

impl NotebookReviewWorkspaceDowngradeReason {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::MissingStableIds,
        Self::RuntimeBound,
        Self::OutputUntrusted,
        Self::Redacted,
        Self::KernelUnavailable,
    ];
}

#[cfg(test)]
mod tests;
