//! M5 clipboard-contract packet: plain-text-default copy, copy-with-context
//! variants, sensitive-copy labels, and relative-path / permalink / command-id /
//! diagnostic-detail / artifact-evidence copy parity across M5 artifact surfaces.
//!
//! Aureline's switching promise depends on keyboard-first, recoverable
//! interaction across every new M5 surface — editor, notebook, data/API,
//! preview, docs, review, runtime, and companion-adjacent panes. The frozen
//! keyboard-continuity matrix
//! [`crate::freeze_the_m5_keyboard_mode_modal_sequence_clipboard_route_drag_drop_verb_and_grouped_history_matrix`]
//! pins those surfaces to their canonical interaction vocabulary and requires
//! that *copy/export defaults preserve useful plain text and sensitive-copy
//! warnings*. This module discharges the clipboard half of that contract: it
//! takes the clipboard-route vocabulary and makes copy **safe and predictable**
//! on the new M5 artifact surfaces by binding each claimed copy flow to one
//! resolved copy variant, preserving the exact representation set instead of a
//! pretty rich blob, and labeling sensitive content before it reaches the
//! clipboard.
//!
//! * a [`ClipboardContractRecord`] binds a claimed M5 surface (keyed by a
//!   [`KeyboardSurfaceKind`] and a non-display [`KeyboardSurfaceSubject`]) to one
//!   copy flow: the [`CopyObjectRef`] it copies (a [`CopyObjectClass`] plus an
//!   opaque / relative object token), the ordered exact [`CopyRepresentation`]
//!   set it pushes to the clipboard, its [`CopySensitivityClass`], and the
//!   resolved [`CopyResolutionClass`];
//! * copy is **never a silent default push when it is non-trivial**: a record
//!   that bundles context beyond the bare object, that carries sensitive token /
//!   fingerprint / private-path / support-link material, that offers only a rich
//!   representation with no plain-text flavor, or whose copy proof is stale or
//!   missing fires one or more [`CopyContractTrigger`]s. Each trigger imposes a
//!   minimum-safety floor on the resolution, so a triggered record can never
//!   resolve to [`CopyResolutionClass::PlainTextDefaultCopy`]; it must expose an
//!   explicit copy-with-context variant, label the sensitive copy, relativize /
//!   redact the content, or be rejected;
//! * the representation set is **preserved, not collapsed**: every record keeps
//!   the ordered clipboard flavors it pushes (keyed by [`CopyFlavorClass`]) and
//!   always carries at least one plain-text flavor unless the copy is rejected,
//!   so help, migration, and support tooling can tell exactly which copy variant
//!   and route a surface exposes — and pretty rich text never becomes the only
//!   readable output.
//!
//! [`ClipboardContractPacket::validate`] refuses a packet that lets a non-trivial
//! copy push silently, that lowers a resolution below its required safety floor,
//! that drops the plain-text representation, that collapses the representation set
//! into an opaque rich blob, that silently pushes sensitive content, or that lets
//! a provider-linked surface read as a locally verified copy.
//!
//! Raw clipboard byte buffers, raw secret material, raw provider payloads, file
//! contents, and absolute private paths never cross this boundary; the packet
//! carries only typed class tokens, booleans, opaque / relative ids, fingerprint
//! digests, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/interaction/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive.schema.json`](../../../../schemas/interaction/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive.schema.json).
//! The contract doc is
//! [`docs/interaction/m5/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive.md`](../../../../docs/interaction/m5/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive.md).
//! The protected fixture directory is
//! [`fixtures/interaction/m5/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive/`](../../../../fixtures/interaction/m5/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Re-export the frozen taxonomy this consumer binds, so product, help, support,
// and migration surfaces can name those types through this module rather than
// reaching into the matrix module by hand.
pub use crate::freeze_the_m5_keyboard_mode_modal_sequence_clipboard_route_drag_drop_verb_and_grouped_history_matrix::{
    AxisProofCurrency, AxisVerification, KeyboardSurfaceKind, KeyboardSurfaceSubject,
    SurfaceOriginClass,
};

/// Stable record-kind tag carried by [`ClipboardContractPacket`].
pub const CLIPBOARD_CONTRACT_RECORD_KIND: &str =
    "m5_clipboard_contract_plain_text_default_copy_with_context_sensitive_label_packet";

/// Schema version for the clipboard-contract packet.
pub const CLIPBOARD_CONTRACT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const CLIPBOARD_CONTRACT_SCHEMA_REF: &str =
    "schemas/interaction/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive.schema.json";

/// Repo-relative path of the contract doc.
pub const CLIPBOARD_CONTRACT_DOC_REF: &str =
    "docs/interaction/m5/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive.md";

/// Repo-relative path of the checked support-export artifact.
pub const CLIPBOARD_CONTRACT_ARTIFACT_REF: &str =
    "artifacts/interaction/m5/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const CLIPBOARD_CONTRACT_SUMMARY_REF: &str =
    "artifacts/interaction/m5/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive.md";

/// Repo-relative path of the protected fixture directory.
pub const CLIPBOARD_CONTRACT_FIXTURE_DIR: &str =
    "fixtures/interaction/m5/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive";

/// Source contract ref of the frozen keyboard-continuity matrix this packet binds.
pub const KEYBOARD_CONTINUITY_MATRIX_DOC_REF: &str =
    "docs/interaction/m5/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.md";

/// Source contract ref of the clipboard / transfer history contract.
pub const CLIPBOARD_TRANSFER_CONTRACT_REF: &str = "docs/ux/clipboard_history_contract.md";

/// Class of object a copy flow puts on the clipboard. The class lets help,
/// migration, and support name the same copy targets the product exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CopyObjectClass {
    /// A fragment of source code / diff text.
    SourceCodeFragment,
    /// A workspace-relative path (never an absolute private path).
    RelativePath,
    /// A stable permalink to a line / range / artifact.
    Permalink,
    /// A command-graph command id token.
    CommandId,
    /// A diagnostic detail (message + location + code).
    DiagnosticDetail,
    /// A reopenable artifact / evidence ref.
    ArtifactEvidenceRef,
    /// A support / session link.
    SupportLink,
}

impl CopyObjectClass {
    /// Every object class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SourceCodeFragment,
        Self::RelativePath,
        Self::Permalink,
        Self::CommandId,
        Self::DiagnosticDetail,
        Self::ArtifactEvidenceRef,
        Self::SupportLink,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceCodeFragment => "source_code_fragment",
            Self::RelativePath => "relative_path",
            Self::Permalink => "permalink",
            Self::CommandId => "command_id",
            Self::DiagnosticDetail => "diagnostic_detail",
            Self::ArtifactEvidenceRef => "artifact_evidence_ref",
            Self::SupportLink => "support_link",
        }
    }
}

/// Sensitivity class of the content a copy flow would push. Any sensitive class
/// must be labeled or transformed before it reaches the clipboard; it is never a
/// silent default push.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CopySensitivityClass {
    /// No sensitive material — safe for a silent default push.
    NonSensitive,
    /// Access-token / credential-shaped material.
    AccessTokenMaterial,
    /// A fingerprint / digest that should not leak silently.
    FingerprintDigest,
    /// An absolute private path that must be relativized before copy.
    PrivateAbsolutePath,
    /// A support / session link that should be labeled before copy.
    SupportSessionLink,
}

impl CopySensitivityClass {
    /// Every sensitivity class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NonSensitive,
        Self::AccessTokenMaterial,
        Self::FingerprintDigest,
        Self::PrivateAbsolutePath,
        Self::SupportSessionLink,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NonSensitive => "non_sensitive",
            Self::AccessTokenMaterial => "access_token_material",
            Self::FingerprintDigest => "fingerprint_digest",
            Self::PrivateAbsolutePath => "private_absolute_path",
            Self::SupportSessionLink => "support_session_link",
        }
    }

    /// Whether this class carries sensitive material that must not be pushed
    /// silently.
    pub const fn is_sensitive(self) -> bool {
        !matches!(self, Self::NonSensitive)
    }

    /// Whether this class is a token / fingerprint digest requiring a label.
    pub const fn is_token_or_fingerprint(self) -> bool {
        matches!(self, Self::AccessTokenMaterial | Self::FingerprintDigest)
    }

    /// Whether this class is a support / session link requiring a label.
    pub const fn is_support_link(self) -> bool {
        matches!(self, Self::SupportSessionLink)
    }

    /// Whether this class is an absolute private path requiring relativization.
    pub const fn is_private_absolute_path(self) -> bool {
        matches!(self, Self::PrivateAbsolutePath)
    }
}

/// Clipboard representation flavor pushed by a copy flow. A plain-text flavor is
/// always human-readable; a rich flavor is not counted as the guaranteed
/// plain-text fallback, so a copy can never be rich-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CopyFlavorClass {
    /// A `text/plain` UTF-8 representation.
    PlainTextUtf8,
    /// A `text/markdown` rich representation.
    MarkdownRich,
    /// A `text/html` rich representation.
    HtmlRich,
    /// A workspace-relative path as plain text.
    RelativePathText,
    /// A permalink URL as plain text.
    PermalinkUrl,
    /// A command-id token as plain text.
    CommandIdToken,
    /// A diagnostic detail as plain text.
    DiagnosticDetailText,
}

impl CopyFlavorClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlainTextUtf8 => "plain_text_utf8",
            Self::MarkdownRich => "markdown_rich",
            Self::HtmlRich => "html_rich",
            Self::RelativePathText => "relative_path_text",
            Self::PermalinkUrl => "permalink_url",
            Self::CommandIdToken => "command_id_token",
            Self::DiagnosticDetailText => "diagnostic_detail_text",
        }
    }

    /// Whether this flavor is a readable plain-text representation that satisfies
    /// the plain-text-default guarantee. A `markdown` / `html` rich flavor does
    /// not count.
    pub const fn is_plain_text(self) -> bool {
        matches!(
            self,
            Self::PlainTextUtf8
                | Self::RelativePathText
                | Self::PermalinkUrl
                | Self::CommandIdToken
                | Self::DiagnosticDetailText
        )
    }
}

/// Resolved copy variant a surface exposes for one copy flow. This is the
/// canonical copy-variant vocabulary help, migration, and support name.
///
/// Only [`Self::PlainTextDefaultCopy`] pushes the plain-text representation
/// silently; every other resolution exposes an explicit copy-with-context
/// variant, labels the sensitive copy, relativizes / redacts the content, or
/// rejects a rich-only / unsafe copy. The [`Self::safety_rank`] orders the
/// resolutions so a triggered record can be held at or above a required floor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CopyResolutionClass {
    /// Plain text is the default representation, pushed without extra labeling.
    PlainTextDefaultCopy,
    /// An explicit copy-with-context variant (e.g. permalink with repo + ref,
    /// diagnostic with detail, code with `file:line`) is exposed alongside plain
    /// text.
    CopyWithContextVariant,
    /// Sensitive content is visibly labeled / previewed before reaching the
    /// clipboard.
    SensitiveLabeledBeforeCopy,
    /// A private absolute path is relativized or a secret is redacted before copy.
    RelativizedOrRedactedCopy,
    /// A rich-only or otherwise unsafe copy is rejected.
    RejectedRichOnlyOrUnsafe,
}

impl CopyResolutionClass {
    /// Every resolution class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PlainTextDefaultCopy,
        Self::CopyWithContextVariant,
        Self::SensitiveLabeledBeforeCopy,
        Self::RelativizedOrRedactedCopy,
        Self::RejectedRichOnlyOrUnsafe,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlainTextDefaultCopy => "plain_text_default_copy",
            Self::CopyWithContextVariant => "copy_with_context_variant",
            Self::SensitiveLabeledBeforeCopy => "sensitive_labeled_before_copy",
            Self::RelativizedOrRedactedCopy => "relativized_or_redacted_copy",
            Self::RejectedRichOnlyOrUnsafe => "rejected_rich_only_or_unsafe",
        }
    }

    /// Monotonic safety rank; higher is a stronger / more restrictive resolution,
    /// so a triggered record must hold a resolution whose rank meets its floor.
    pub const fn safety_rank(self) -> u8 {
        match self {
            Self::PlainTextDefaultCopy => 0,
            Self::CopyWithContextVariant => 1,
            Self::SensitiveLabeledBeforeCopy => 2,
            Self::RelativizedOrRedactedCopy => 3,
            Self::RejectedRichOnlyOrUnsafe => 4,
        }
    }

    /// Whether this resolution pushes plain text silently as the default.
    pub const fn is_silent_default(self) -> bool {
        matches!(self, Self::PlainTextDefaultCopy)
    }

    /// Whether this resolution must cite a `context_label`.
    pub const fn requires_context_label(self) -> bool {
        matches!(self, Self::CopyWithContextVariant)
    }

    /// Whether this resolution must cite a `sensitive_label`.
    pub const fn requires_sensitive_label(self) -> bool {
        matches!(self, Self::SensitiveLabeledBeforeCopy)
    }

    /// Whether this resolution must cite a `transform_note`.
    pub const fn requires_transform_note(self) -> bool {
        matches!(self, Self::RelativizedOrRedactedCopy)
    }

    /// Whether this resolution must cite a `rejection_reason_label`.
    pub const fn requires_rejection_reason(self) -> bool {
        matches!(self, Self::RejectedRichOnlyOrUnsafe)
    }
}

/// Why a record's copy was held off the silent default lane. Each trigger imposes
/// a minimum-safety floor; the chrome quotes the trigger verbatim instead of a
/// generic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CopyContractTrigger {
    /// The copy bundles context beyond the bare object.
    ContextBeyondBareObject,
    /// The content carries sensitive token / fingerprint material.
    SensitiveTokenOrFingerprint,
    /// The content carries a support / session link.
    SupportLinkPresent,
    /// The content carries an absolute private path.
    PrivateAbsolutePath,
    /// The copy offers only a rich representation with no plain-text flavor.
    RichOnlyNoPlainText,
    /// The copy-contract proof backing this record is stale or missing.
    StaleOrMissingCopyProof,
}

impl CopyContractTrigger {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContextBeyondBareObject => "context_beyond_bare_object",
            Self::SensitiveTokenOrFingerprint => "sensitive_token_or_fingerprint",
            Self::SupportLinkPresent => "support_link_present",
            Self::PrivateAbsolutePath => "private_absolute_path",
            Self::RichOnlyNoPlainText => "rich_only_no_plain_text",
            Self::StaleOrMissingCopyProof => "stale_or_missing_copy_proof",
        }
    }

    /// Minimum resolution safety rank this trigger imposes.
    ///
    /// A bundled-context copy or stale proof requires at least an explicit
    /// copy-with-context variant; a sensitive token / fingerprint or a support
    /// link requires a sensitive-copy label; an absolute private path requires
    /// relativization / redaction; a rich-only copy with no plain-text flavor is
    /// rejected outright.
    pub const fn minimum_resolution_rank(self) -> u8 {
        match self {
            Self::ContextBeyondBareObject | Self::StaleOrMissingCopyProof => {
                CopyResolutionClass::CopyWithContextVariant.safety_rank()
            }
            Self::SensitiveTokenOrFingerprint | Self::SupportLinkPresent => {
                CopyResolutionClass::SensitiveLabeledBeforeCopy.safety_rank()
            }
            Self::PrivateAbsolutePath => {
                CopyResolutionClass::RelativizedOrRedactedCopy.safety_rank()
            }
            Self::RichOnlyNoPlainText => {
                CopyResolutionClass::RejectedRichOnlyOrUnsafe.safety_rank()
            }
        }
    }
}

/// One clipboard representation flavor pushed by a copy flow. The flavor keeps the
/// representation explicit so the copy is never reduced to an opaque rich blob.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyRepresentation {
    /// Stable opaque representation id, unique within a record.
    pub representation_id: String,
    /// Clipboard flavor of this representation.
    pub flavor_class: CopyFlavorClass,
    /// Reviewable label safe for support export.
    pub display_label: String,
}

impl CopyRepresentation {
    /// Whether this representation is a readable plain-text flavor.
    pub fn is_plain_text(&self) -> bool {
        self.flavor_class.is_plain_text()
    }

    /// Whether every required field is present.
    pub fn is_well_formed(&self) -> bool {
        !self.representation_id.trim().is_empty() && !self.display_label.trim().is_empty()
    }
}

/// The object a copy flow puts on the clipboard. Preserved so a copy can be named
/// and reconstructed rather than collapsed into opaque text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyObjectRef {
    /// Class of the copied object.
    pub object_class: CopyObjectClass,
    /// Opaque / workspace-relative object token. Never an absolute private path.
    pub object_token: String,
    /// Reviewable label.
    pub display_label: String,
}

impl CopyObjectRef {
    /// Whether the object carries the identity a copy needs.
    pub fn is_valid(&self) -> bool {
        !self.object_token.trim().is_empty() && !self.display_label.trim().is_empty()
    }
}

/// Constructor input for [`ClipboardContractRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardContractRecordInput {
    /// Stable record id.
    pub record_id: String,
    /// Kind of claimed M5 surface.
    pub surface_kind: KeyboardSurfaceKind,
    /// Durable subject the record covers.
    pub subject: KeyboardSurfaceSubject,
    /// Reviewable record label.
    pub label_summary: String,
    /// The copied object.
    pub object: CopyObjectRef,
    /// Ordered, exact clipboard representation set.
    pub representations: Vec<CopyRepresentation>,
    /// Reviewable representation summary safe for export.
    pub representations_summary: String,
    /// Whether the copy bundles context beyond the bare object.
    pub context_bundled: bool,
    /// Sensitivity of the copied content.
    pub sensitivity: CopySensitivityClass,
    /// Reopenable verification proof backing the resolution.
    pub verification: AxisVerification,
    /// The resolved copy variant.
    pub resolution: CopyResolutionClass,
    /// Triggers recorded as firing for this record.
    pub fired_triggers: Vec<CopyContractTrigger>,
    /// Required when `resolution` is `copy_with_context_variant`.
    pub context_label: Option<String>,
    /// Required when `resolution` is `sensitive_labeled_before_copy`.
    pub sensitive_label: Option<String>,
    /// Required when `resolution` is `relativized_or_redacted_copy`.
    pub transform_note: Option<String>,
    /// Required when `resolution` is `rejected_rich_only_or_unsafe`.
    pub rejection_reason_label: Option<String>,
    /// Evidence packet refs backing this record.
    pub evidence_refs: Vec<String>,
    /// Mint timestamp.
    pub minted_at: String,
}

/// One clipboard-contract record binding a claimed M5 surface to one resolved
/// copy variant with an exact representation set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardContractRecord {
    /// Stable record id.
    pub record_id: String,
    /// Kind of claimed M5 surface.
    pub surface_kind: KeyboardSurfaceKind,
    /// Durable subject the record covers.
    pub subject: KeyboardSurfaceSubject,
    /// Reviewable record label.
    pub label_summary: String,
    /// The copied object.
    pub object: CopyObjectRef,
    /// Ordered, exact clipboard representation set.
    pub representations: Vec<CopyRepresentation>,
    /// Reviewable representation summary safe for export.
    pub representations_summary: String,
    /// Whether the copy bundles context beyond the bare object.
    pub context_bundled: bool,
    /// Sensitivity of the copied content.
    pub sensitivity: CopySensitivityClass,
    /// Reopenable verification proof backing the resolution.
    pub verification: AxisVerification,
    /// The resolved copy variant.
    pub resolution: CopyResolutionClass,
    /// Triggers recorded as firing for this record. Must equal the computed set.
    pub fired_triggers: Vec<CopyContractTrigger>,
    /// Required when `resolution` is `copy_with_context_variant`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_label: Option<String>,
    /// Required when `resolution` is `sensitive_labeled_before_copy`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensitive_label: Option<String>,
    /// Required when `resolution` is `relativized_or_redacted_copy`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform_note: Option<String>,
    /// Required when `resolution` is `rejected_rich_only_or_unsafe`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rejection_reason_label: Option<String>,
    /// Guardrail: record does not carry raw clipboard byte buffers.
    pub raw_clipboard_bytes_present: bool,
    /// Guardrail: record does not carry raw secret material.
    pub raw_secret_material_present: bool,
    /// Guardrail: record does not carry an absolute private path.
    pub absolute_private_path_present: bool,
    /// Guardrail: the representation set was not collapsed into an opaque rich blob.
    pub representations_collapsed_to_rich_blob: bool,
    /// Guardrail: the record did not silently push sensitive content.
    pub silent_sensitive_push_taken: bool,
    /// Evidence packet refs backing this record.
    pub evidence_refs: Vec<String>,
    /// Mint timestamp.
    pub minted_at: String,
}

impl ClipboardContractRecord {
    /// Builds a record from its input, defaulting the redaction guardrail flags to
    /// their safe values.
    pub fn new(input: ClipboardContractRecordInput) -> Self {
        Self {
            record_id: input.record_id,
            surface_kind: input.surface_kind,
            subject: input.subject,
            label_summary: input.label_summary,
            object: input.object,
            representations: input.representations,
            representations_summary: input.representations_summary,
            context_bundled: input.context_bundled,
            sensitivity: input.sensitivity,
            verification: input.verification,
            resolution: input.resolution,
            fired_triggers: input.fired_triggers,
            context_label: input.context_label,
            sensitive_label: input.sensitive_label,
            transform_note: input.transform_note,
            rejection_reason_label: input.rejection_reason_label,
            raw_clipboard_bytes_present: false,
            raw_secret_material_present: false,
            absolute_private_path_present: false,
            representations_collapsed_to_rich_blob: false,
            silent_sensitive_push_taken: false,
            evidence_refs: input.evidence_refs,
            minted_at: input.minted_at,
        }
    }

    /// Whether copy for this record is provider-backed / imported.
    pub fn provider_or_imported(&self) -> bool {
        self.subject.origin_class.is_provider_or_imported()
    }

    /// Whether the verification proof backs a current copy claim for this record's
    /// origin posture.
    pub fn copy_proof_current(&self) -> bool {
        self.verification.backs_claim(self.provider_or_imported())
    }

    /// Whether at least one plain-text representation is present.
    pub fn plain_text_preserved(&self) -> bool {
        self.representations
            .iter()
            .any(CopyRepresentation::is_plain_text)
    }

    /// The set of triggers that actually fire for this record, computed from its
    /// context, sensitivity, representations, and proof.
    pub fn computed_triggers(&self) -> BTreeSet<CopyContractTrigger> {
        let mut triggers = BTreeSet::new();
        if self.context_bundled {
            triggers.insert(CopyContractTrigger::ContextBeyondBareObject);
        }
        if self.sensitivity.is_token_or_fingerprint() {
            triggers.insert(CopyContractTrigger::SensitiveTokenOrFingerprint);
        }
        if self.sensitivity.is_support_link() {
            triggers.insert(CopyContractTrigger::SupportLinkPresent);
        }
        if self.sensitivity.is_private_absolute_path() {
            triggers.insert(CopyContractTrigger::PrivateAbsolutePath);
        }
        if !self.plain_text_preserved() {
            triggers.insert(CopyContractTrigger::RichOnlyNoPlainText);
        }
        if !self.copy_proof_current() {
            triggers.insert(CopyContractTrigger::StaleOrMissingCopyProof);
        }
        triggers
    }

    /// The recorded triggers as a set.
    pub fn recorded_triggers(&self) -> BTreeSet<CopyContractTrigger> {
        self.fired_triggers.iter().copied().collect()
    }

    /// The minimum resolution safety rank this record must meet, given its
    /// triggers.
    pub fn required_floor_rank(&self) -> u8 {
        self.computed_triggers()
            .iter()
            .map(|trigger| trigger.minimum_resolution_rank())
            .max()
            .unwrap_or(0)
    }

    /// Whether the copy must be held off the silent default lane.
    pub fn must_not_push_silently(&self) -> bool {
        self.required_floor_rank() > 0
    }

    /// Whether the recorded resolution meets the required safety floor.
    pub fn resolution_meets_floor(&self) -> bool {
        self.resolution.safety_rank() >= self.required_floor_rank()
    }

    /// Whether the recorded resolution silently pushes a copy that must not.
    pub fn silently_pushes_unsafe(&self) -> bool {
        self.resolution.is_silent_default() && self.must_not_push_silently()
    }

    /// Whether the recorded trigger set matches the computed set.
    pub fn triggers_consistent(&self) -> bool {
        self.recorded_triggers() == self.computed_triggers()
    }

    /// Whether the resolution carries exactly the detail field it requires.
    pub fn resolution_detail_consistent(&self) -> bool {
        let present = |opt: &Option<String>| {
            opt.as_deref()
                .is_some_and(|value| !value.trim().is_empty() && !label_is_generic(value))
        };
        let context_ok = if self.resolution.requires_context_label() {
            present(&self.context_label)
        } else {
            self.context_label.is_none()
        };
        let sensitive_ok = if self.resolution.requires_sensitive_label() {
            present(&self.sensitive_label)
        } else {
            self.sensitive_label.is_none()
        };
        let transform_ok = if self.resolution.requires_transform_note() {
            present(&self.transform_note)
        } else {
            self.transform_note.is_none()
        };
        let reject_ok = if self.resolution.requires_rejection_reason() {
            present(&self.rejection_reason_label)
        } else {
            self.rejection_reason_label.is_none()
        };
        context_ok && sensitive_ok && transform_ok && reject_ok
    }

    /// Whether the imported posture is consistent: a provider/imported surface
    /// never reads as a locally verified copy, and a local surface never leans on
    /// imported proof.
    pub fn imported_posture_consistent(&self) -> bool {
        if self.provider_or_imported() {
            !self.verification.proof_currency.is_current_local()
        } else {
            !self.verification.proof_currency.is_imported_current()
        }
    }

    /// Whether the representation set is exact and not collapsed into a rich blob.
    pub fn representations_exact(&self) -> bool {
        !self.representations_collapsed_to_rich_blob
            && !self.representations.is_empty()
            && self
                .representations
                .iter()
                .all(CopyRepresentation::is_well_formed)
    }

    /// Whether plain text is preserved unless the copy is rejected.
    pub fn plain_text_guarantee_holds(&self) -> bool {
        self.plain_text_preserved()
            || self.resolution == CopyResolutionClass::RejectedRichOnlyOrUnsafe
    }

    /// Whether no raw boundary material is flagged present.
    pub fn no_raw_boundary_material(&self) -> bool {
        !self.raw_clipboard_bytes_present
            && !self.raw_secret_material_present
            && !self.absolute_private_path_present
            && !self.silent_sensitive_push_taken
    }

    /// Whether every field required to record this record is present and its
    /// invariants hold.
    pub fn is_complete(&self) -> bool {
        !self.record_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.representations_summary.trim().is_empty()
            && !self.minted_at.trim().is_empty()
            && self.subject.is_valid()
            && self.object.is_valid()
            && self.representations_exact()
            && self.plain_text_guarantee_holds()
            && self.verification.is_well_formed()
            && self.triggers_consistent()
            && !self.silently_pushes_unsafe()
            && self.resolution_meets_floor()
            && self.resolution_detail_consistent()
            && self.imported_posture_consistent()
            && self.no_raw_boundary_material()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardContractGuardrails {
    /// Every non-rejected copy preserves a useful plain-text representation.
    pub plain_text_preserved_by_default: bool,
    /// Pretty rich text never becomes the only readable copy representation.
    pub rich_text_never_the_only_representation: bool,
    /// Sensitive copy is visibly labeled / previewed, never silently pushed.
    pub sensitive_copy_labeled_not_silent: bool,
    /// The copied object and its representation set are preserved on every record.
    pub object_and_representations_preserved: bool,
    /// Provider-linked copies never read as a locally verified copy.
    pub provider_copies_never_read_as_local: bool,
    /// No new general macro language or editor core is introduced here.
    pub no_new_macro_language_introduced: bool,
}

impl ClipboardContractGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.plain_text_preserved_by_default
            && self.rich_text_never_the_only_representation
            && self.sensitive_copy_labeled_not_silent
            && self.object_and_representations_preserved
            && self.provider_copies_never_read_as_local
            && self.no_new_macro_language_introduced
    }
}

/// Consumer projection block: the surfaces that read this packet without cloning
/// copy-variant language by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardContractConsumerProjection {
    /// Product surfaces ingest this packet.
    pub product_ingests_packet: bool,
    /// Help / migration guidance ingests the same packet.
    pub help_migration_ingests_packet: bool,
    /// Support / export tooling ingests the same packet.
    pub support_export_ingests_packet: bool,
    /// Release-control surfaces ingest the same packet.
    pub release_control_ingests_packet: bool,
    /// Help / migration / support can name the same copy variants and route
    /// classes the product exposes from this packet.
    pub copy_variants_and_routes_nameable: bool,
}

impl ClipboardContractConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_packet
            && self.help_migration_ingests_packet
            && self.support_export_ingests_packet
            && self.release_control_ingests_packet
            && self.copy_variants_and_routes_nameable
    }
}

/// Verification freshness block for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardContractFreshness {
    /// Verification-freshness SLO in hours.
    pub verification_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last verification refresh.
    pub last_verification_refresh: String,
    /// True when stale verification automatically forces records off silent copy.
    pub auto_label_on_stale: bool,
}

impl ClipboardContractFreshness {
    /// Whether the freshness block is well-formed.
    pub fn is_valid(&self) -> bool {
        self.verification_freshness_slo_hours > 0
            && !self.last_verification_refresh.trim().is_empty()
    }
}

/// Constructor input for [`ClipboardContractPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardContractPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Per-surface clipboard-contract records.
    pub records: Vec<ClipboardContractRecord>,
    /// Guardrail invariants block.
    pub guardrails: ClipboardContractGuardrails,
    /// Consumer projection block.
    pub consumer_projection: ClipboardContractConsumerProjection,
    /// Verification freshness block.
    pub verification_freshness: ClipboardContractFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe clipboard-contract packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardContractPacket {
    /// Record kind; must equal [`CLIPBOARD_CONTRACT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`CLIPBOARD_CONTRACT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Per-surface clipboard-contract records.
    pub records: Vec<ClipboardContractRecord>,
    /// Guardrail invariants block.
    pub guardrails: ClipboardContractGuardrails,
    /// Consumer projection block.
    pub consumer_projection: ClipboardContractConsumerProjection,
    /// Verification freshness block.
    pub verification_freshness: ClipboardContractFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ClipboardContractPacket {
    /// Builds a clipboard-contract packet.
    pub fn new(input: ClipboardContractPacketInput) -> Self {
        Self {
            record_kind: CLIPBOARD_CONTRACT_RECORD_KIND.to_owned(),
            schema_version: CLIPBOARD_CONTRACT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            records: input.records,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            verification_freshness: input.verification_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surface kinds represented by some record in this packet.
    pub fn represented_surface_kinds(&self) -> BTreeSet<KeyboardSurfaceKind> {
        self.records
            .iter()
            .map(|record| record.surface_kind)
            .collect()
    }

    /// Object classes represented across records.
    pub fn represented_object_classes(&self) -> BTreeSet<CopyObjectClass> {
        self.records
            .iter()
            .map(|record| record.object.object_class)
            .collect()
    }

    /// Resolution classes represented across records.
    pub fn represented_resolutions(&self) -> BTreeSet<CopyResolutionClass> {
        self.records
            .iter()
            .map(|record| record.resolution)
            .collect()
    }

    /// Count of records held off the silent default lane.
    pub fn forced_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.must_not_push_silently())
            .count()
    }

    /// Count of records resolved to a silent plain-text default copy.
    pub fn silent_default_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.resolution.is_silent_default())
            .count()
    }

    /// Count of records resolved to a sensitive-labeled copy.
    pub fn sensitive_labeled_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.resolution == CopyResolutionClass::SensitiveLabeledBeforeCopy)
            .count()
    }

    /// Count of provider-linked / imported records.
    pub fn provider_or_imported_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.provider_or_imported())
            .count()
    }

    /// Resolves a record by its id.
    pub fn record(&self, record_id: &str) -> Option<&ClipboardContractRecord> {
        self.records
            .iter()
            .find(|record| record.record_id == record_id)
    }

    /// Validates the clipboard-contract invariants.
    pub fn validate(&self) -> Vec<ClipboardContractViolation> {
        let mut violations = Vec::new();

        if self.record_kind != CLIPBOARD_CONTRACT_RECORD_KIND {
            violations.push(ClipboardContractViolation::WrongRecordKind);
        }
        if self.schema_version != CLIPBOARD_CONTRACT_SCHEMA_VERSION {
            violations.push(ClipboardContractViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ClipboardContractViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_records(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(ClipboardContractViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(ClipboardContractViolation::ConsumerProjectionIncomplete);
        }
        if !self.verification_freshness.is_valid() {
            violations.push(ClipboardContractViolation::VerificationFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("clipboard contract packet serializes"),
        ) {
            violations.push(ClipboardContractViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("clipboard contract packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, help, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Clipboard Contracts: Plain-Text-Default Copy, Copy-With-Context Variants, and Sensitive-Copy Labels\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!(
            "- Records: {} ({} silent default, {} forced off silent, {} sensitive-labeled, {} provider/imported)\n",
            self.records.len(),
            self.silent_default_record_count(),
            self.forced_record_count(),
            self.sensitive_labeled_record_count(),
            self.provider_or_imported_record_count()
        ));
        out.push_str(&format!(
            "- Surface kinds: {} / {}\n",
            self.represented_surface_kinds().len(),
            KeyboardSurfaceKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Object classes: {} / {}\n",
            self.represented_object_classes().len(),
            CopyObjectClass::ALL.len()
        ));
        out.push_str(&format!(
            "- Resolution classes: {} / {}\n",
            self.represented_resolutions().len(),
            CopyResolutionClass::ALL.len()
        ));
        out.push_str(&format!(
            "- Verification freshness SLO: {} hours (last refresh: {})\n",
            self.verification_freshness.verification_freshness_slo_hours,
            self.verification_freshness.last_verification_refresh
        ));
        out.push_str("\n## Records\n\n");
        for record in &self.records {
            out.push_str(&format!(
                "- **{}** ({}): resolution `{}`\n",
                record.record_id,
                record.surface_kind.as_str(),
                record.resolution.as_str()
            ));
            out.push_str(&format!("  - {}\n", record.label_summary));
            out.push_str(&format!(
                "  - object `{}` ({}), sensitivity `{}`\n",
                record.object.object_token,
                record.object.object_class.as_str(),
                record.sensitivity.as_str()
            ));
            let flavors = record
                .representations
                .iter()
                .map(|representation| representation.flavor_class.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!(
                "  - representations: [{}] (plain-text preserved={})\n",
                flavors,
                record.plain_text_preserved()
            ));
            let triggers = record
                .fired_triggers
                .iter()
                .map(|trigger| trigger.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!(
                "  - triggers: [{}]\n",
                if triggers.is_empty() {
                    "none"
                } else {
                    &triggers
                }
            ));
            if let Some(label) = &record.context_label {
                out.push_str(&format!("  - Copy-with-context: {label}\n"));
            }
            if let Some(label) = &record.sensitive_label {
                out.push_str(&format!("  - Sensitive-copy label: {label}\n"));
            }
            if let Some(note) = &record.transform_note {
                out.push_str(&format!("  - Relativized/redacted: {note}\n"));
            }
            if let Some(label) = &record.rejection_reason_label {
                out.push_str(&format!("  - Rejected: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in packet export.
#[derive(Debug)]
pub enum ClipboardContractArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ClipboardContractViolation>),
}

impl fmt::Display for ClipboardContractArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "clipboard contract export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "clipboard contract export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ClipboardContractArtifactError {}

/// Validation failures emitted by [`ClipboardContractPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClipboardContractViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required claimed surface kind is represented by no record.
    RequiredSurfaceKindMissing,
    /// A required copy object class is represented by no record.
    RequiredObjectClassMissing,
    /// The required copy-resolution classes are not all represented.
    ResolutionCoverageMissing,
    /// No record demonstrates a copy held off the silent default lane.
    ForcedRecordCaseMissing,
    /// No clean silent plain-text-default baseline record is present.
    SilentDefaultBaselineMissing,
    /// No sensitive-labeled copy record is present.
    SensitiveLabeledCaseMissing,
    /// No provider-linked / imported record is present.
    ProviderOrImportedCaseMissing,
    /// A record is incomplete.
    RecordIncomplete,
    /// A non-trivial copy was allowed to push silently on the default lane.
    SilentPushOfUnsafeCopy,
    /// A record's resolution ranks below its required safety floor.
    ResolutionBelowRequiredFloor,
    /// A record's recorded triggers do not match the computed set.
    TriggerSetInconsistent,
    /// A record's resolution detail field is missing, generic, or unexpected.
    ResolutionDetailInconsistent,
    /// A non-rejected record dropped its plain-text representation.
    PlainTextRepresentationMissing,
    /// A record's representation set was collapsed into an opaque rich blob.
    RepresentationsCollapsedToRichBlob,
    /// A record dropped its copied object.
    CopyObjectMissing,
    /// A provider/imported record reads as a locally verified copy.
    ImportedReadsAsLocal,
    /// A record's verification proof is not reopenable.
    VerificationProofNotReopenable,
    /// A record lacks evidence refs.
    RecordEvidenceMissing,
    /// A record's subject fingerprint stands in for its bare id.
    FingerprintSubstitutesIdentity,
    /// A record flags raw boundary material present.
    RawBoundaryMaterialPresent,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Verification freshness block is incomplete.
    VerificationFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ClipboardContractViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceKindMissing => "required_surface_kind_missing",
            Self::RequiredObjectClassMissing => "required_object_class_missing",
            Self::ResolutionCoverageMissing => "resolution_coverage_missing",
            Self::ForcedRecordCaseMissing => "forced_record_case_missing",
            Self::SilentDefaultBaselineMissing => "silent_default_baseline_missing",
            Self::SensitiveLabeledCaseMissing => "sensitive_labeled_case_missing",
            Self::ProviderOrImportedCaseMissing => "provider_or_imported_case_missing",
            Self::RecordIncomplete => "record_incomplete",
            Self::SilentPushOfUnsafeCopy => "silent_push_of_unsafe_copy",
            Self::ResolutionBelowRequiredFloor => "resolution_below_required_floor",
            Self::TriggerSetInconsistent => "trigger_set_inconsistent",
            Self::ResolutionDetailInconsistent => "resolution_detail_inconsistent",
            Self::PlainTextRepresentationMissing => "plain_text_representation_missing",
            Self::RepresentationsCollapsedToRichBlob => "representations_collapsed_to_rich_blob",
            Self::CopyObjectMissing => "copy_object_missing",
            Self::ImportedReadsAsLocal => "imported_reads_as_local",
            Self::VerificationProofNotReopenable => "verification_proof_not_reopenable",
            Self::RecordEvidenceMissing => "record_evidence_missing",
            Self::FingerprintSubstitutesIdentity => "fingerprint_substitutes_identity",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::VerificationFreshnessIncomplete => "verification_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable packet export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_clipboard_contract_export(
) -> Result<ClipboardContractPacket, ClipboardContractArtifactError> {
    let packet: ClipboardContractPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/interaction/m5/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive/support_export.json"
    )))
    .map_err(ClipboardContractArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ClipboardContractArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &ClipboardContractPacket,
    violations: &mut Vec<ClipboardContractViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        CLIPBOARD_CONTRACT_SCHEMA_REF,
        CLIPBOARD_CONTRACT_DOC_REF,
        CLIPBOARD_CONTRACT_ARTIFACT_REF,
        KEYBOARD_CONTINUITY_MATRIX_DOC_REF,
        CLIPBOARD_TRANSFER_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ClipboardContractViolation::MissingSourceContracts);
            break;
        }
    }
}

/// Surface kinds that must appear so the packet proves clipboard-contract parity
/// across the new M5 artifact surfaces, plus the editor-core baseline.
const REQUIRED_SURFACE_KINDS: [KeyboardSurfaceKind; 6] = [
    KeyboardSurfaceKind::EditorCore,
    KeyboardSurfaceKind::NotebookSurface,
    KeyboardSurfaceKind::DataApiSurface,
    KeyboardSurfaceKind::PreviewSurface,
    KeyboardSurfaceKind::DocsSurface,
    KeyboardSurfaceKind::ReviewSurface,
];

/// Object classes whose copy parity this packet must demonstrate.
const REQUIRED_OBJECT_CLASSES: [CopyObjectClass; 5] = [
    CopyObjectClass::RelativePath,
    CopyObjectClass::Permalink,
    CopyObjectClass::CommandId,
    CopyObjectClass::DiagnosticDetail,
    CopyObjectClass::ArtifactEvidenceRef,
];

fn validate_coverage(
    packet: &ClipboardContractPacket,
    violations: &mut Vec<ClipboardContractViolation>,
) {
    let surface_kinds = packet.represented_surface_kinds();
    for required in REQUIRED_SURFACE_KINDS {
        if !surface_kinds.contains(&required) {
            violations.push(ClipboardContractViolation::RequiredSurfaceKindMissing);
            break;
        }
    }

    let object_classes = packet.represented_object_classes();
    for required in REQUIRED_OBJECT_CLASSES {
        if !object_classes.contains(&required) {
            violations.push(ClipboardContractViolation::RequiredObjectClassMissing);
            break;
        }
    }

    let resolutions = packet.represented_resolutions();
    for required in CopyResolutionClass::ALL {
        if !resolutions.contains(&required) {
            violations.push(ClipboardContractViolation::ResolutionCoverageMissing);
            break;
        }
    }

    if !packet
        .records
        .iter()
        .any(|record| record.must_not_push_silently() && record.is_complete())
    {
        violations.push(ClipboardContractViolation::ForcedRecordCaseMissing);
    }

    if !packet.records.iter().any(|record| {
        record.resolution.is_silent_default()
            && !record.must_not_push_silently()
            && record.is_complete()
    }) {
        violations.push(ClipboardContractViolation::SilentDefaultBaselineMissing);
    }

    if !packet.records.iter().any(|record| {
        record.resolution == CopyResolutionClass::SensitiveLabeledBeforeCopy
            && record.sensitivity.is_sensitive()
            && record.is_complete()
    }) {
        violations.push(ClipboardContractViolation::SensitiveLabeledCaseMissing);
    }

    if packet.provider_or_imported_record_count() == 0 {
        violations.push(ClipboardContractViolation::ProviderOrImportedCaseMissing);
    }
}

fn validate_records(
    packet: &ClipboardContractPacket,
    violations: &mut Vec<ClipboardContractViolation>,
) {
    for record in &packet.records {
        if !record.is_complete() {
            violations.push(ClipboardContractViolation::RecordIncomplete);
        }
        if record.silently_pushes_unsafe() {
            violations.push(ClipboardContractViolation::SilentPushOfUnsafeCopy);
        }
        if !record.resolution_meets_floor() {
            violations.push(ClipboardContractViolation::ResolutionBelowRequiredFloor);
        }
        if !record.triggers_consistent() {
            violations.push(ClipboardContractViolation::TriggerSetInconsistent);
        }
        if !record.resolution_detail_consistent() {
            violations.push(ClipboardContractViolation::ResolutionDetailInconsistent);
        }
        if !record.plain_text_guarantee_holds() {
            violations.push(ClipboardContractViolation::PlainTextRepresentationMissing);
        }
        if record.representations_collapsed_to_rich_blob || !record.representations_exact() {
            violations.push(ClipboardContractViolation::RepresentationsCollapsedToRichBlob);
        }
        if !record.object.is_valid() {
            violations.push(ClipboardContractViolation::CopyObjectMissing);
        }
        if !record.imported_posture_consistent() {
            violations.push(ClipboardContractViolation::ImportedReadsAsLocal);
        }
        if !record.verification.is_well_formed() {
            violations.push(ClipboardContractViolation::VerificationProofNotReopenable);
        }
        if record.evidence_refs.is_empty()
            || record.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(ClipboardContractViolation::RecordEvidenceMissing);
        }
        if !record.subject.fingerprint_independent_of_id() {
            violations.push(ClipboardContractViolation::FingerprintSubstitutesIdentity);
        }
        if !record.no_raw_boundary_material() {
            violations.push(ClipboardContractViolation::RawBoundaryMaterialPresent);
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "provider error"
            | "request failed"
            | "failed"
            | "sensitive"
            | "redacted"
            | "rejected"
            | "unverified"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key") || lower.contains("password") || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Stable packet id minted by [`seeded_clipboard_contract_packet`].
pub const SEED_CLIPBOARD_CONTRACT_PACKET_ID: &str = "m5-clipboard-contract:stable:0001";

/// Mint timestamp used by [`seeded_clipboard_contract_packet`].
pub const SEED_CLIPBOARD_CONTRACT_MINTED_AT: &str = "2026-06-14T00:00:00Z";

/// Builds the canonical, validating clipboard-contract packet that the checked-in
/// support export, the Markdown summary, and the conformance tests all share, so
/// the in-crate builder stays byte-aligned with the artifact.
///
/// The seed anchors clean plain-text-default copy baselines, then exercises each
/// non-default resolution on a distinct M5 surface: a notebook diagnostic copied
/// with context, a data/API permalink copied with context, a review evidence ref
/// offered rich-with-plain-fallback as an explicit context variant, a runtime
/// support link labeled before copy, a data/API evidence ref carrying a
/// fingerprint labeled before copy, an editor private path relativized before
/// copy, a docs rich-only copy rejected, and a provider-linked companion permalink
/// copied with context whose imported proof never reads as a local copy.
pub fn seeded_clipboard_contract_packet() -> ClipboardContractPacket {
    ClipboardContractPacket::new(ClipboardContractPacketInput {
        packet_id: SEED_CLIPBOARD_CONTRACT_PACKET_ID.to_owned(),
        label: "M5 Clipboard Contracts: Plain-Text-Default Copy, Copy-With-Context, and Sensitive-Copy Labels"
            .to_owned(),
        records: seeded_records(),
        guardrails: ClipboardContractGuardrails {
            plain_text_preserved_by_default: true,
            rich_text_never_the_only_representation: true,
            sensitive_copy_labeled_not_silent: true,
            object_and_representations_preserved: true,
            provider_copies_never_read_as_local: true,
            no_new_macro_language_introduced: true,
        },
        consumer_projection: ClipboardContractConsumerProjection {
            product_ingests_packet: true,
            help_migration_ingests_packet: true,
            support_export_ingests_packet: true,
            release_control_ingests_packet: true,
            copy_variants_and_routes_nameable: true,
        },
        verification_freshness: ClipboardContractFreshness {
            verification_freshness_slo_hours: 168,
            last_verification_refresh: SEED_CLIPBOARD_CONTRACT_MINTED_AT.to_owned(),
            auto_label_on_stale: true,
        },
        source_contract_refs: vec![
            CLIPBOARD_CONTRACT_SCHEMA_REF.to_owned(),
            CLIPBOARD_CONTRACT_DOC_REF.to_owned(),
            CLIPBOARD_CONTRACT_ARTIFACT_REF.to_owned(),
            KEYBOARD_CONTINUITY_MATRIX_DOC_REF.to_owned(),
            CLIPBOARD_TRANSFER_CONTRACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_CLIPBOARD_CONTRACT_MINTED_AT.to_owned(),
    })
}

fn seeded_records() -> Vec<ClipboardContractRecord> {
    vec![
        editor_core_plain_text_record(),
        preview_command_id_record(),
        docs_relative_path_record(),
        notebook_diagnostic_context_record(),
        data_api_permalink_context_record(),
        review_evidence_context_record(),
        runtime_support_link_record(),
        data_api_fingerprint_record(),
        editor_private_path_record(),
        docs_rich_only_reject_record(),
        companion_provider_record(),
    ]
}

/// Builds a verification proof keyed by a non-display fingerprint distinct from
/// the record id.
fn proof_for(record_id: &str, currency: AxisProofCurrency, summary: &str) -> AxisVerification {
    let (proof_ref, proof_fingerprint_token) = if currency.is_absent() {
        (None, None)
    } else {
        (
            Some(format!("evidence:{record_id}")),
            Some(format!("fp:proof:{record_id}")),
        )
    };
    AxisVerification {
        proof_currency: currency,
        proof_ref,
        proof_fingerprint_token,
        summary: summary.to_owned(),
    }
}

/// Builds a subject whose fingerprint is independent of its surface id.
fn subject_for(record_id: &str, origin_class: SurfaceOriginClass) -> KeyboardSurfaceSubject {
    KeyboardSurfaceSubject {
        surface_id: format!("surface:{record_id}"),
        origin_class,
        surface_fingerprint_token: format!("fp:surface:{record_id}"),
    }
}

fn representation(
    representation_id: &str,
    flavor_class: CopyFlavorClass,
    display_label: &str,
) -> CopyRepresentation {
    CopyRepresentation {
        representation_id: representation_id.to_owned(),
        flavor_class,
        display_label: display_label.to_owned(),
    }
}

fn object(object_class: CopyObjectClass, object_token: &str, display_label: &str) -> CopyObjectRef {
    CopyObjectRef {
        object_class,
        object_token: object_token.to_owned(),
        display_label: display_label.to_owned(),
    }
}

fn editor_core_plain_text_record() -> ClipboardContractRecord {
    let record_id = "clipboard:editor-core:0001";
    ClipboardContractRecord::new(ClipboardContractRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::EditorCore,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary: "Editor-core source selection copied as plain text by default".to_owned(),
        object: object(
            CopyObjectClass::SourceCodeFragment,
            "fragment:src/lib.rs#L10-L18",
            "Source selection in src/lib.rs",
        ),
        representations: vec![representation(
            "rep-1",
            CopyFlavorClass::PlainTextUtf8,
            "Plain UTF-8 source text",
        )],
        representations_summary: "Plain-text source representation".to_owned(),
        context_bundled: false,
        sensitivity: CopySensitivityClass::NonSensitive,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Editor-core plain-text copy verified to preserve readable text by default",
        ),
        resolution: CopyResolutionClass::PlainTextDefaultCopy,
        fired_triggers: vec![],
        context_label: None,
        sensitive_label: None,
        transform_note: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_CLIPBOARD_CONTRACT_MINTED_AT.to_owned(),
    })
}

fn preview_command_id_record() -> ClipboardContractRecord {
    let record_id = "clipboard:preview:0001";
    ClipboardContractRecord::new(ClipboardContractRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::PreviewSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary: "Preview command id copied as a plain-text token by default".to_owned(),
        object: object(
            CopyObjectClass::CommandId,
            "command:preview.runtime.reload",
            "Command id preview.runtime.reload",
        ),
        representations: vec![representation(
            "rep-1",
            CopyFlavorClass::CommandIdToken,
            "Command-id token text",
        )],
        representations_summary: "Plain-text command-id token".to_owned(),
        context_bundled: false,
        sensitivity: CopySensitivityClass::NonSensitive,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Preview command-id copy verified to push a readable token by default",
        ),
        resolution: CopyResolutionClass::PlainTextDefaultCopy,
        fired_triggers: vec![],
        context_label: None,
        sensitive_label: None,
        transform_note: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_CLIPBOARD_CONTRACT_MINTED_AT.to_owned(),
    })
}

fn docs_relative_path_record() -> ClipboardContractRecord {
    let record_id = "clipboard:docs:0001";
    ClipboardContractRecord::new(ClipboardContractRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::DocsSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary: "Docs relative path copied as plain text by default".to_owned(),
        object: object(
            CopyObjectClass::RelativePath,
            "docs/interaction/m5/overview.md",
            "Workspace-relative docs path",
        ),
        representations: vec![representation(
            "rep-1",
            CopyFlavorClass::RelativePathText,
            "Workspace-relative path text",
        )],
        representations_summary: "Plain-text relative path".to_owned(),
        context_bundled: false,
        sensitivity: CopySensitivityClass::NonSensitive,
        verification: proof_for(
            record_id,
            AxisProofCurrency::CachedWithinWindow,
            "Docs relative-path copy verified to push a workspace-relative path",
        ),
        resolution: CopyResolutionClass::PlainTextDefaultCopy,
        fired_triggers: vec![],
        context_label: None,
        sensitive_label: None,
        transform_note: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_CLIPBOARD_CONTRACT_MINTED_AT.to_owned(),
    })
}

fn notebook_diagnostic_context_record() -> ClipboardContractRecord {
    let record_id = "clipboard:notebook:0001";
    ClipboardContractRecord::new(ClipboardContractRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::NotebookSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Notebook diagnostic copied with detail and location as an explicit context variant"
                .to_owned(),
        object: object(
            CopyObjectClass::DiagnosticDetail,
            "diagnostic:notebook/cell-3#E0277",
            "Diagnostic E0277 in notebook cell 3",
        ),
        representations: vec![
            representation(
                "rep-1",
                CopyFlavorClass::PlainTextUtf8,
                "Plain diagnostic message",
            ),
            representation(
                "rep-2",
                CopyFlavorClass::DiagnosticDetailText,
                "Diagnostic detail with code and location",
            ),
        ],
        representations_summary: "Plain message plus diagnostic detail with location".to_owned(),
        context_bundled: true,
        sensitivity: CopySensitivityClass::NonSensitive,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Notebook diagnostic copy verified to expose an explicit copy-with-context variant",
        ),
        resolution: CopyResolutionClass::CopyWithContextVariant,
        fired_triggers: vec![CopyContractTrigger::ContextBeyondBareObject],
        context_label: Some(
            "Copy-with-context adds the diagnostic code, message, and cell location".to_owned(),
        ),
        sensitive_label: None,
        transform_note: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_CLIPBOARD_CONTRACT_MINTED_AT.to_owned(),
    })
}

fn data_api_permalink_context_record() -> ClipboardContractRecord {
    let record_id = "clipboard:data-api:0001";
    ClipboardContractRecord::new(ClipboardContractRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::DataApiSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary: "Data/API permalink copied with repo and ref context as an explicit variant"
            .to_owned(),
        object: object(
            CopyObjectClass::Permalink,
            "permalink:data/api/users.json#L42@rev-abc",
            "Permalink to a data/API row",
        ),
        representations: vec![
            representation(
                "rep-1",
                CopyFlavorClass::PlainTextUtf8,
                "Plain relative location text",
            ),
            representation(
                "rep-2",
                CopyFlavorClass::PermalinkUrl,
                "Permalink URL with repo and ref",
            ),
        ],
        representations_summary: "Plain relative location plus a permalink URL".to_owned(),
        context_bundled: true,
        sensitivity: CopySensitivityClass::NonSensitive,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Data/API permalink copy verified to expose an explicit copy-with-context variant",
        ),
        resolution: CopyResolutionClass::CopyWithContextVariant,
        fired_triggers: vec![CopyContractTrigger::ContextBeyondBareObject],
        context_label: Some(
            "Copy-with-context binds the row to its repo, ref, and relative path".to_owned(),
        ),
        sensitive_label: None,
        transform_note: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_CLIPBOARD_CONTRACT_MINTED_AT.to_owned(),
    })
}

fn review_evidence_context_record() -> ClipboardContractRecord {
    let record_id = "clipboard:review:0001";
    ClipboardContractRecord::new(ClipboardContractRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::ReviewSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Review evidence ref copied rich-with-plain-fallback as an explicit context variant"
                .to_owned(),
        object: object(
            CopyObjectClass::ArtifactEvidenceRef,
            "evidence:review/run-9921",
            "Reopenable review evidence ref",
        ),
        representations: vec![
            representation(
                "rep-1",
                CopyFlavorClass::PlainTextUtf8,
                "Plain evidence summary text",
            ),
            representation(
                "rep-2",
                CopyFlavorClass::MarkdownRich,
                "Markdown evidence summary",
            ),
        ],
        representations_summary: "Plain-text fallback plus a markdown rich representation"
            .to_owned(),
        context_bundled: true,
        sensitivity: CopySensitivityClass::NonSensitive,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Review evidence copy verified to keep a plain-text fallback behind the rich variant",
        ),
        resolution: CopyResolutionClass::CopyWithContextVariant,
        fired_triggers: vec![CopyContractTrigger::ContextBeyondBareObject],
        context_label: Some(
            "Copy-with-context offers a markdown summary with a plain-text fallback".to_owned(),
        ),
        sensitive_label: None,
        transform_note: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_CLIPBOARD_CONTRACT_MINTED_AT.to_owned(),
    })
}

fn runtime_support_link_record() -> ClipboardContractRecord {
    let record_id = "clipboard:runtime:0001";
    ClipboardContractRecord::new(ClipboardContractRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::RuntimeSurface,
        subject: subject_for(record_id, SurfaceOriginClass::EmbeddedRuntimeSurface),
        label_summary: "Runtime support link labeled before it reaches the clipboard".to_owned(),
        object: object(
            CopyObjectClass::SupportLink,
            "support-link:session/opaque-handle",
            "Support session link",
        ),
        representations: vec![representation(
            "rep-1",
            CopyFlavorClass::PermalinkUrl,
            "Support session link text",
        )],
        representations_summary: "Plain-text support link with a sensitive-copy label".to_owned(),
        context_bundled: false,
        sensitivity: CopySensitivityClass::SupportSessionLink,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Runtime support-link copy verified to label the link before copying",
        ),
        resolution: CopyResolutionClass::SensitiveLabeledBeforeCopy,
        fired_triggers: vec![CopyContractTrigger::SupportLinkPresent],
        context_label: None,
        sensitive_label: Some(
            "This support link identifies your session; review before sharing it".to_owned(),
        ),
        transform_note: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_CLIPBOARD_CONTRACT_MINTED_AT.to_owned(),
    })
}

fn data_api_fingerprint_record() -> ClipboardContractRecord {
    let record_id = "clipboard:data-api:fingerprint:0001";
    ClipboardContractRecord::new(ClipboardContractRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::DataApiSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary: "Data/API evidence ref embedding a fingerprint labeled before copy"
            .to_owned(),
        object: object(
            CopyObjectClass::ArtifactEvidenceRef,
            "evidence:data-api/run-7741",
            "Evidence ref carrying a response fingerprint",
        ),
        representations: vec![representation(
            "rep-1",
            CopyFlavorClass::PlainTextUtf8,
            "Plain evidence summary with fingerprint",
        )],
        representations_summary: "Plain-text evidence summary with a sensitive-copy label"
            .to_owned(),
        context_bundled: false,
        sensitivity: CopySensitivityClass::FingerprintDigest,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Data/API fingerprint copy verified to label the digest before copying",
        ),
        resolution: CopyResolutionClass::SensitiveLabeledBeforeCopy,
        fired_triggers: vec![CopyContractTrigger::SensitiveTokenOrFingerprint],
        context_label: None,
        sensitive_label: Some(
            "This evidence ref embeds a response fingerprint; review before sharing".to_owned(),
        ),
        transform_note: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_CLIPBOARD_CONTRACT_MINTED_AT.to_owned(),
    })
}

fn editor_private_path_record() -> ClipboardContractRecord {
    let record_id = "clipboard:editor-core:private-path:0001";
    ClipboardContractRecord::new(ClipboardContractRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::EditorCore,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Editor private absolute path relativized before it reaches the clipboard".to_owned(),
        object: object(
            CopyObjectClass::RelativePath,
            "src/secret_module/handler.rs",
            "Relativized path for a private absolute path",
        ),
        representations: vec![representation(
            "rep-1",
            CopyFlavorClass::RelativePathText,
            "Workspace-relative path text",
        )],
        representations_summary: "Plain-text workspace-relative path after relativization"
            .to_owned(),
        context_bundled: false,
        sensitivity: CopySensitivityClass::PrivateAbsolutePath,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Editor private-path copy verified to relativize the absolute path before copying",
        ),
        resolution: CopyResolutionClass::RelativizedOrRedactedCopy,
        fired_triggers: vec![CopyContractTrigger::PrivateAbsolutePath],
        context_label: None,
        sensitive_label: None,
        transform_note: Some(
            "The absolute home-directory path is relativized to a workspace-relative path before copy"
                .to_owned(),
        ),
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_CLIPBOARD_CONTRACT_MINTED_AT.to_owned(),
    })
}

fn docs_rich_only_reject_record() -> ClipboardContractRecord {
    let record_id = "clipboard:docs:rich-only:0001";
    ClipboardContractRecord::new(ClipboardContractRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::DocsSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary: "Docs rich-only copy with no plain-text flavor is rejected".to_owned(),
        object: object(
            CopyObjectClass::SourceCodeFragment,
            "fragment:docs/example.html#L1-L4",
            "Rendered docs fragment",
        ),
        representations: vec![representation(
            "rep-1",
            CopyFlavorClass::HtmlRich,
            "HTML-only rendered fragment",
        )],
        representations_summary: "Rich HTML representation with no plain-text fallback".to_owned(),
        context_bundled: false,
        sensitivity: CopySensitivityClass::NonSensitive,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Docs rich-only copy verified to reject because no plain-text flavor is offered",
        ),
        resolution: CopyResolutionClass::RejectedRichOnlyOrUnsafe,
        fired_triggers: vec![CopyContractTrigger::RichOnlyNoPlainText],
        context_label: None,
        sensitive_label: None,
        transform_note: None,
        rejection_reason_label: Some(
            "A rich-only HTML copy with no readable plain-text fallback is rejected".to_owned(),
        ),
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_CLIPBOARD_CONTRACT_MINTED_AT.to_owned(),
    })
}

fn companion_provider_record() -> ClipboardContractRecord {
    let record_id = "clipboard:companion:0001";
    ClipboardContractRecord::new(ClipboardContractRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::CompanionSurface,
        subject: subject_for(record_id, SurfaceOriginClass::ProviderLinkedSurface),
        label_summary:
            "Provider-linked companion permalink copied with context; imported proof never reads as local"
                .to_owned(),
        object: object(
            CopyObjectClass::Permalink,
            "permalink:companion/thread-204#message-7",
            "Permalink to a provider-backed companion message",
        ),
        representations: vec![
            representation(
                "rep-1",
                CopyFlavorClass::PlainTextUtf8,
                "Plain provider reference text",
            ),
            representation(
                "rep-2",
                CopyFlavorClass::PermalinkUrl,
                "Provider permalink URL",
            ),
        ],
        representations_summary: "Plain reference text plus a provider permalink URL".to_owned(),
        context_bundled: true,
        sensitivity: CopySensitivityClass::NonSensitive,
        verification: proof_for(
            record_id,
            AxisProofCurrency::ImportedCurrent,
            "Provider-backed companion copy verified with imported proof, never a local copy",
        ),
        resolution: CopyResolutionClass::CopyWithContextVariant,
        fired_triggers: vec![CopyContractTrigger::ContextBeyondBareObject],
        context_label: Some(
            "Copy-with-context binds the message to its provider thread and permalink".to_owned(),
        ),
        sensitive_label: None,
        transform_note: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_CLIPBOARD_CONTRACT_MINTED_AT.to_owned(),
    })
}

/// Packet id minted by [`fixture_clipboard_contract_packet`].
pub const FIXTURE_CLIPBOARD_CONTRACT_PACKET_ID: &str =
    "m5-clipboard-contract:fixture:stale-proof-forces-label:0001";

/// Builds the protected fixture variant: it keeps the full seeded record set —
/// including the clean plain-text-default baselines — and adds one drill record
/// for a docs relative-path copy that would otherwise push plain text silently
/// but is forced off the silent default lane because its copy proof aged outside
/// the freshness window.
///
/// The fixture is a *valid* packet: the drill record correctly records the
/// [`CopyContractTrigger::StaleOrMissingCopyProof`] trigger and resolves to
/// [`CopyResolutionClass::CopyWithContextVariant`] with a precise context label,
/// so it validates while demonstrating that stale evidence — not just context or
/// sensitivity — forces a copy off the silent default lane.
pub fn fixture_clipboard_contract_packet() -> ClipboardContractPacket {
    let mut packet = seeded_clipboard_contract_packet();
    packet.packet_id = FIXTURE_CLIPBOARD_CONTRACT_PACKET_ID.to_owned();
    packet.label =
        "M5 Clipboard Contracts fixture: stale copy proof forces a silent default copy into an explicit variant"
            .to_owned();
    packet.records.push(stale_proof_drill_record());
    packet
}

/// A docs relative-path copy that would push plain text silently, but whose copy
/// proof has aged outside its freshness window, so it is forced into an explicit
/// copy-with-context variant.
fn stale_proof_drill_record() -> ClipboardContractRecord {
    let record_id = "clipboard:docs:stale-proof:0001";
    ClipboardContractRecord::new(ClipboardContractRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::DocsSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Docs relative-path copy whose stale copy proof forces it into an explicit variant"
                .to_owned(),
        object: object(
            CopyObjectClass::RelativePath,
            "docs/interaction/m5/stale.md",
            "Workspace-relative docs path with stale proof",
        ),
        representations: vec![representation(
            "rep-1",
            CopyFlavorClass::RelativePathText,
            "Workspace-relative path text",
        )],
        representations_summary: "Plain-text relative path forced into an explicit variant"
            .to_owned(),
        context_bundled: false,
        sensitivity: CopySensitivityClass::NonSensitive,
        verification: proof_for(
            record_id,
            AxisProofCurrency::StaleExpired,
            "Docs relative-path copy proof aged outside its freshness window",
        ),
        resolution: CopyResolutionClass::CopyWithContextVariant,
        fired_triggers: vec![CopyContractTrigger::StaleOrMissingCopyProof],
        context_label: Some(
            "Copy proof aged outside its freshness window; the copy is offered as an explicit, re-verified variant"
                .to_owned(),
        ),
        sensitive_label: None,
        transform_note: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_CLIPBOARD_CONTRACT_MINTED_AT.to_owned(),
    })
}
