//! Frozen M5 keyboard-mode / clipboard-route / drag-drop-verb / grouped-history
//! continuity matrix for every claimed M5 switching-wedge and power-user surface.
//!
//! Aureline's switching promise depends on keyboard-first, recoverable
//! interaction across every new M5 surface — editor, notebook, data/API,
//! preview, docs, review, runtime, and companion-adjacent panes — not just the
//! original editor core. This module freezes those expectations into one
//! machine-readable matrix so modal-mode state, leader-key sequences,
//! clipboard/register routes, drag/drop verbs, grouped undo/history classes,
//! reopen/recover paths, and orientation aids are explicit product contracts
//! instead of ad hoc per-surface behavior.
//!
//! * a [`KeyboardParityRow`] ties a durable [`KeyboardSurfaceSubject`] (keyed by
//!   a [`KeyboardSurfaceKind`], an origin class, and a non-display fingerprint)
//!   to the canonical interaction vocabulary it advertises: a [`ModeStripClass`],
//!   a [`SequenceGuideClass`], a [`ClipboardRouteClass`], a [`DragDropVerbClass`],
//!   an [`UndoClass`], a [`HistoryClass`], a [`ReopenRecoverClass`], and an
//!   [`OrientationAidClass`], plus a claimed and effective
//!   [`ContinuityParityGrade`];
//! * each row is **verification-bound, not asserted**: its [`AxisVerification`]
//!   names an [`AxisProofCurrency`] and, unless the proof is missing, a reopenable
//!   `proof_ref` keyed by a non-display fingerprint, so accessibility, help, and
//!   release review can reopen the same keyboard/clipboard/history evidence object
//!   that backs the parity claim;
//! * the row **auto-downgrades**: [`KeyboardParityRow::needs_downgrade`] is true
//!   whenever any axis is in an honestly-downgraded or denied state (an
//!   unsupported modal sequence, a rich-only clipboard route, a destructive
//!   drag/drop default, collapsed orientation aids), whenever a surface stops
//!   being keyboard-complete or its macro replay stops being explicit, or whenever
//!   its verification proof is stale, missing, review-pending, or imported proof
//!   standing in for a local claim. A downgraded row must carry an effective grade
//!   strictly below its claim, a recorded [`ParityDowngradeTrigger`], and a precise
//!   label — never a generic non-answer.
//!
//! [`KeyboardContinuityMatrixPacket::validate`] also refuses a packet that lets an
//! unsupported modal sequence read as silently approximated, lets a rich-only copy
//! become the only clipboard representation, lets a drag/drop verb default to a
//! destructive or ambiguous move, or flattens exact undo, grouped exact undo,
//! compensating action, and checkpoint restore into one vague history label.
//!
//! Raw clipboard bodies, raw key buffers, raw provider payloads, file contents,
//! private paths, and credentials never cross this boundary; the packet carries
//! only typed class tokens, booleans, opaque ids, fingerprint digests, and
//! redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/interaction/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.schema.json`](../../../../schemas/interaction/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.schema.json).
//! The contract doc is
//! [`docs/interaction/m5/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.md`](../../../../docs/interaction/m5/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.md).
//! The protected fixture directory is
//! [`fixtures/interaction/m5/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist/`](../../../../fixtures/interaction/m5/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`KeyboardContinuityMatrixPacket`].
pub const KEYBOARD_CONTINUITY_MATRIX_RECORD_KIND: &str =
    "freeze_m5_keyboard_mode_clipboard_drag_drop_grouped_history_matrix_packet";

/// Schema version for the keyboard-continuity matrix packet.
pub const KEYBOARD_CONTINUITY_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const KEYBOARD_CONTINUITY_MATRIX_SCHEMA_REF: &str =
    "schemas/interaction/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.schema.json";

/// Repo-relative path of the contract doc.
pub const KEYBOARD_CONTINUITY_MATRIX_DOC_REF: &str =
    "docs/interaction/m5/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.md";

/// Repo-relative path of the checked support-export artifact.
pub const KEYBOARD_CONTINUITY_MATRIX_ARTIFACT_REF: &str =
    "artifacts/interaction/m5/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const KEYBOARD_CONTINUITY_MATRIX_SUMMARY_REF: &str =
    "artifacts/interaction/m5/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.md";

/// Repo-relative path of the protected fixture directory.
pub const KEYBOARD_CONTINUITY_MATRIX_FIXTURE_DIR: &str =
    "fixtures/interaction/m5/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist";

/// Kind of claimed M5 surface a parity row covers. Each kind is a distinct claim
/// surface that must carry its own keyboard/clipboard/history behavior rather than
/// inheriting the editor core's parity by assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyboardSurfaceKind {
    /// The original editor / diff core surface.
    EditorCore,
    /// A notebook cell / notebook-output surface.
    NotebookSurface,
    /// A data-grid / API-result surface.
    DataApiSurface,
    /// A source-first preview / preview-runtime surface.
    PreviewSurface,
    /// A docs authoring / docs-browser surface.
    DocsSurface,
    /// A review / pull-request panel surface.
    ReviewSurface,
    /// An embedded runtime / terminal-adjacent surface.
    RuntimeSurface,
    /// A companion-adjacent / provider-linked surface.
    CompanionSurface,
}

impl KeyboardSurfaceKind {
    /// Every surface kind, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::EditorCore,
        Self::NotebookSurface,
        Self::DataApiSurface,
        Self::PreviewSurface,
        Self::DocsSurface,
        Self::ReviewSurface,
        Self::RuntimeSurface,
        Self::CompanionSurface,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorCore => "editor_core",
            Self::NotebookSurface => "notebook_surface",
            Self::DataApiSurface => "data_api_surface",
            Self::PreviewSurface => "preview_surface",
            Self::DocsSurface => "docs_surface",
            Self::ReviewSurface => "review_surface",
            Self::RuntimeSurface => "runtime_surface",
            Self::CompanionSurface => "companion_surface",
        }
    }
}

/// Origin of a claimed surface. A provider-linked or imported surface must never
/// read as a locally verified first-party surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceOriginClass {
    /// A first-party shell-owned surface verified locally.
    FirstPartySurface,
    /// An embedded runtime surface (browser-runtime / terminal host).
    EmbeddedRuntimeSurface,
    /// A provider-linked surface whose parity is provider-backed and read-only.
    ProviderLinkedSurface,
    /// An imported surface whose parity record is read-only.
    ImportedReadOnlySurface,
}

impl SurfaceOriginClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartySurface => "first_party_surface",
            Self::EmbeddedRuntimeSurface => "embedded_runtime_surface",
            Self::ProviderLinkedSurface => "provider_linked_surface",
            Self::ImportedReadOnlySurface => "imported_read_only_surface",
        }
    }

    /// Whether parity for this origin is provider-backed / imported rather than
    /// locally verified, so a current claim rests on imported proof.
    pub const fn is_provider_or_imported(self) -> bool {
        matches!(
            self,
            Self::ProviderLinkedSurface | Self::ImportedReadOnlySurface
        )
    }
}

/// Canonical mode-strip vocabulary: how a surface advertises its modal posture.
/// Mode changes are explicit; an unsupported modal mode downgrades honestly rather
/// than silently approximating an editor it cannot host.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModeStripClass {
    /// Full modal parity (Vim/Neovim/Emacs-class), mode visible in the strip.
    ModalParityComplete,
    /// Modal navigation only on a read-only surface, mode visible in the strip.
    ModalReadOnlyNavigation,
    /// No modal layer, but the surface is fully keyboard-driven.
    NonModalKeyboardComplete,
    /// A claimed modal mode is unsupported here and is downgraded honestly.
    ModeUnsupportedDowngraded,
}

impl ModeStripClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModalParityComplete => "modal_parity_complete",
            Self::ModalReadOnlyNavigation => "modal_read_only_navigation",
            Self::NonModalKeyboardComplete => "non_modal_keyboard_complete",
            Self::ModeUnsupportedDowngraded => "mode_unsupported_downgraded",
        }
    }

    /// Whether the mode posture is honestly downgraded rather than supported.
    pub const fn is_downgraded(self) -> bool {
        matches!(self, Self::ModeUnsupportedDowngraded)
    }
}

/// Canonical sequence-guide vocabulary: how leader-key / multi-key sequences are
/// supported and surfaced. An unsupported sequence is downgraded, never silently
/// approximated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SequenceGuideClass {
    /// Full leader / multi-key sequences with an on-screen sequence guide.
    LeaderSequenceComplete,
    /// Prefix-key sequences with a discovery overlay.
    PrefixDiscoverable,
    /// Single-stroke commands only; no multi-key sequences claimed.
    SingleStrokeOnly,
    /// A claimed multi-key sequence is unsupported and is downgraded honestly.
    SequenceUnsupportedDowngraded,
}

impl SequenceGuideClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LeaderSequenceComplete => "leader_sequence_complete",
            Self::PrefixDiscoverable => "prefix_discoverable",
            Self::SingleStrokeOnly => "single_stroke_only",
            Self::SequenceUnsupportedDowngraded => "sequence_unsupported_downgraded",
        }
    }

    /// Whether the sequence support is honestly downgraded rather than supported.
    pub const fn is_downgraded(self) -> bool {
        matches!(self, Self::SequenceUnsupportedDowngraded)
    }
}

/// Canonical register / clipboard-route vocabulary: how copy/export preserves
/// representations. A rich-only route with no plain-text fallback is denied so
/// pretty rich text never becomes the only copy representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClipboardRouteClass {
    /// Copy/export preserves useful plain text as the default representation.
    PlainTextPreserved,
    /// Rich representation with a plain-text fallback always available.
    RichWithPlainFallback,
    /// Named-register routing (Vim-style registers) is supported.
    NamedRegisterRouted,
    /// Copy of sensitive content carries an explicit sensitive-copy warning.
    SensitiveCopyWarned,
    /// Rich-text-only with no plain-text fallback — a denied route.
    RichOnlyDenied,
}

impl ClipboardRouteClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlainTextPreserved => "plain_text_preserved",
            Self::RichWithPlainFallback => "rich_with_plain_fallback",
            Self::NamedRegisterRouted => "named_register_routed",
            Self::SensitiveCopyWarned => "sensitive_copy_warned",
            Self::RichOnlyDenied => "rich_only_denied",
        }
    }

    /// Whether this route denies a useful plain-text representation.
    pub const fn is_denied(self) -> bool {
        matches!(self, Self::RichOnlyDenied)
    }
}

/// Canonical drag/drop-verb vocabulary: which transfer verb a drop performs. The
/// verb is always explicit; a destructive or ambiguous default is denied so
/// drag/drop never hides verbs or scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DragDropVerbClass {
    /// Drop performs an explicit move and advertises it.
    MoveVerbExplicit,
    /// Drop performs an explicit copy and advertises it.
    CopyVerbExplicit,
    /// Drop performs an explicit link / reference and advertises it.
    LinkVerbExplicit,
    /// The verb is chosen explicitly (modifier or drop menu), never defaulted.
    VerbChoiceOnModifier,
    /// A destructive or ambiguous default verb — denied.
    DestructiveDefaultDenied,
}

impl DragDropVerbClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MoveVerbExplicit => "move_verb_explicit",
            Self::CopyVerbExplicit => "copy_verb_explicit",
            Self::LinkVerbExplicit => "link_verb_explicit",
            Self::VerbChoiceOnModifier => "verb_choice_on_modifier",
            Self::DestructiveDefaultDenied => "destructive_default_denied",
        }
    }

    /// Whether this verb class is a denied destructive / ambiguous default.
    pub const fn is_denied(self) -> bool {
        matches!(self, Self::DestructiveDefaultDenied)
    }
}

/// Canonical undo-class vocabulary. Exact undo, grouped exact undo, compensating
/// action, checkpoint restore, and honest no-undo stay distinct; they are never
/// flattened into one vague history label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UndoClass {
    /// An exact inverse of the prior action.
    ExactUndo,
    /// A named group of actions undone exactly as one unit.
    GroupedExactUndo,
    /// A compensating action that is not a literal inverse.
    CompensatingUndo,
    /// A checkpoint / snapshot restore rather than a step inverse.
    CheckpointRestore,
    /// The action is not undoable, stated honestly rather than flattened.
    NoUndoHonest,
}

impl UndoClass {
    /// Every undo class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ExactUndo,
        Self::GroupedExactUndo,
        Self::CompensatingUndo,
        Self::CheckpointRestore,
        Self::NoUndoHonest,
    ];

    /// The distinct undo classes that must never be flattened into one label.
    pub const DISTINCT_CORE: [Self; 3] = [
        Self::ExactUndo,
        Self::CompensatingUndo,
        Self::CheckpointRestore,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactUndo => "exact_undo",
            Self::GroupedExactUndo => "grouped_exact_undo",
            Self::CompensatingUndo => "compensating_undo",
            Self::CheckpointRestore => "checkpoint_restore",
            Self::NoUndoHonest => "no_undo_honest",
        }
    }
}

/// Canonical grouped-history vocabulary: how a surface lays out its history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryClass {
    /// A linear step history.
    LinearStepHistory,
    /// A grouped / named-entry step history.
    GroupedStepHistory,
    /// A branching / tree history.
    BranchingHistory,
    /// A checkpoint lineage chain.
    CheckpointLineage,
    /// History continuous across surfaces.
    CrossSurfaceContinuity,
}

impl HistoryClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LinearStepHistory => "linear_step_history",
            Self::GroupedStepHistory => "grouped_step_history",
            Self::BranchingHistory => "branching_history",
            Self::CheckpointLineage => "checkpoint_lineage",
            Self::CrossSurfaceContinuity => "cross_surface_continuity",
        }
    }
}

/// Canonical reopen/recover vocabulary. An unavailable reopen degrades honestly
/// rather than pretending an exact reopen happened.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenRecoverClass {
    /// Reopen restores the exact prior state.
    ExactReopen,
    /// Recovery restores an approximate state, with the approximation stated.
    RecoveredApproximate,
    /// Recovery restores from a checkpoint.
    CheckpointRecover,
    /// Reopen is unavailable here, stated honestly.
    ReopenUnavailableHonest,
}

impl ReopenRecoverClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactReopen => "exact_reopen",
            Self::RecoveredApproximate => "recovered_approximate",
            Self::CheckpointRecover => "checkpoint_recover",
            Self::ReopenUnavailableHonest => "reopen_unavailable_honest",
        }
    }
}

/// Canonical orientation-aid vocabulary (mode strip, sequence guide, breadcrumb).
/// Orientation aids degrade honestly; a collapse to no aids is a downgrade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrientationAidClass {
    /// Full orientation aids present (mode strip, sequence guide, breadcrumb).
    FullOrientationAids,
    /// A reduced set of aids, honestly labeled.
    ReducedOrientationAidsHonest,
    /// Aids degraded for this surface, honestly disclosed.
    OrientationAidsDegradedHonest,
    /// Aids collapsed to nothing — a downgrade.
    OrientationAidsAbsentDowngraded,
}

impl OrientationAidClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullOrientationAids => "full_orientation_aids",
            Self::ReducedOrientationAidsHonest => "reduced_orientation_aids_honest",
            Self::OrientationAidsDegradedHonest => "orientation_aids_degraded_honest",
            Self::OrientationAidsAbsentDowngraded => "orientation_aids_absent_downgraded",
        }
    }

    /// Whether orientation aids collapsed rather than degrading honestly.
    pub const fn is_downgraded(self) -> bool {
        matches!(self, Self::OrientationAidsAbsentDowngraded)
    }
}

/// Continuity parity grade a surface row claims or effectively holds. Higher
/// [`Self::rank`] is a stronger claim, so a downgraded row must move strictly
/// lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityParityGrade {
    /// Release-bearing switching-wedge parity.
    SwitchingCertified,
    /// Full keyboard/clipboard/history parity, publicly claimed.
    ParityComplete,
    /// Partial parity (honest capability gaps stated, still keyboard-complete).
    ParityPartial,
    /// Parity not verified; held below a public claim.
    ParityUnverified,
    /// Parity does not apply on this row.
    NotApplicable,
}

impl ContinuityParityGrade {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SwitchingCertified => "switching_certified",
            Self::ParityComplete => "parity_complete",
            Self::ParityPartial => "parity_partial",
            Self::ParityUnverified => "parity_unverified",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this grade carries a public parity claim.
    pub const fn is_claimed(self) -> bool {
        matches!(
            self,
            Self::SwitchingCertified | Self::ParityComplete | Self::ParityPartial
        )
    }

    /// Ordinal rank; higher is a stronger claim, so a downgrade must move strictly
    /// lower.
    pub const fn rank(self) -> u8 {
        match self {
            Self::NotApplicable => 0,
            Self::ParityUnverified => 1,
            Self::ParityPartial => 2,
            Self::ParityComplete => 3,
            Self::SwitchingCertified => 4,
        }
    }
}

/// Currency of the proof backing a row's verification. Only a current, reopenable
/// proof backs a claim; a stale, missing, review-pending, or imported-on-local
/// proof auto-downgrades the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisProofCurrency {
    /// A fresh local proof verified inside its freshness window.
    VerifiedCurrent,
    /// A cached local proof still inside its freshness window.
    CachedWithinWindow,
    /// A current proof imported / provider-backed and read-only locally.
    ImportedCurrent,
    /// A proof that exists but has aged outside its freshness window.
    StaleExpired,
    /// A proof that still requires review and fails closed.
    RequiresReview,
    /// No proof object exists for this row.
    MissingProof,
}

impl AxisProofCurrency {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifiedCurrent => "verified_current",
            Self::CachedWithinWindow => "cached_within_window",
            Self::ImportedCurrent => "imported_current",
            Self::StaleExpired => "stale_expired",
            Self::RequiresReview => "requires_review",
            Self::MissingProof => "missing_proof",
        }
    }

    /// Whether this is a current, locally verified or cached proof.
    pub const fn is_current_local(self) -> bool {
        matches!(self, Self::VerifiedCurrent | Self::CachedWithinWindow)
    }

    /// Whether this is a current imported / provider-backed proof.
    pub const fn is_imported_current(self) -> bool {
        matches!(self, Self::ImportedCurrent)
    }

    /// Whether this currency carries no proof object.
    pub const fn is_absent(self) -> bool {
        matches!(self, Self::MissingProof)
    }
}

/// Reason a claimed row auto-downgraded below its claim. The chrome quotes the
/// trigger verbatim instead of a generic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParityDowngradeTrigger {
    /// The active modal mode could not be identified on this surface.
    ModeStateUnidentified,
    /// A claimed modal sequence is unsupported; downgraded rather than approximated.
    UnsupportedSequenceDowngraded,
    /// The clipboard route could not preserve a useful plain-text representation.
    ClipboardPlainTextUnavailable,
    /// The drag/drop verb would be destructive or ambiguous by default.
    DragDropVerbAmbiguous,
    /// The history class could not be identified for this surface.
    HistoryClassUnidentified,
    /// The reopen/recover path could not be verified.
    ReopenPathUnverified,
    /// Orientation aids collapsed rather than degrading honestly.
    OrientationAidsCollapsed,
    /// The verification proof aged outside its freshness window.
    StaleVerificationProof,
    /// Imported / provider proof stood in for a local-surface parity claim.
    ImportedProofOnLocalSurface,
    /// An upstream surface downgraded and dragged this row down with it.
    UpstreamSurfaceDowngraded,
}

impl ParityDowngradeTrigger {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModeStateUnidentified => "mode_state_unidentified",
            Self::UnsupportedSequenceDowngraded => "unsupported_sequence_downgraded",
            Self::ClipboardPlainTextUnavailable => "clipboard_plain_text_unavailable",
            Self::DragDropVerbAmbiguous => "drag_drop_verb_ambiguous",
            Self::HistoryClassUnidentified => "history_class_unidentified",
            Self::ReopenPathUnverified => "reopen_path_unverified",
            Self::OrientationAidsCollapsed => "orientation_aids_collapsed",
            Self::StaleVerificationProof => "stale_verification_proof",
            Self::ImportedProofOnLocalSurface => "imported_proof_on_local_surface",
            Self::UpstreamSurfaceDowngraded => "upstream_surface_downgraded",
        }
    }
}

/// Durable subject of a parity row, keyed by a surface kind, an origin class, and
/// a non-display fingerprint distinct from its id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardSurfaceSubject {
    /// Durable id of the surface.
    pub surface_id: String,
    /// Origin class of the surface.
    pub origin_class: SurfaceOriginClass,
    /// Non-display fingerprint token. Must differ from
    /// [`surface_id`](KeyboardSurfaceSubject::surface_id).
    pub surface_fingerprint_token: String,
}

impl KeyboardSurfaceSubject {
    /// Whether parity for this subject is provider-backed / imported.
    pub fn is_provider_or_imported(&self) -> bool {
        self.origin_class.is_provider_or_imported()
    }

    /// Whether the fingerprint is a real non-display basis distinct from the id.
    pub fn fingerprint_independent_of_id(&self) -> bool {
        let token = self.surface_fingerprint_token.trim();
        !token.is_empty() && token != self.surface_id.trim()
    }

    /// Whether the subject carries the durable identity a reopen needs.
    pub fn is_valid(&self) -> bool {
        !self.surface_id.trim().is_empty() && self.fingerprint_independent_of_id()
    }
}

/// A row's verification proof: the proof currency plus a reopenable evidence
/// object, so a parity grade is backed by an object a reviewer can reopen rather
/// than an asserted claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AxisVerification {
    /// Currency of the proof backing this row.
    pub proof_currency: AxisProofCurrency,
    /// Reopenable ref of the proof object. Present unless the proof is missing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_ref: Option<String>,
    /// Non-display fingerprint token of the proof object. Present iff `proof_ref`
    /// is present, and must differ from it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_fingerprint_token: Option<String>,
    /// Export-safe reviewable summary of the proof.
    pub summary: String,
}

impl AxisVerification {
    /// Whether the proof object is reopenable: a present ref carries a distinct
    /// non-display fingerprint and a non-empty summary.
    pub fn proof_reopenable(&self) -> bool {
        match (&self.proof_ref, &self.proof_fingerprint_token) {
            (Some(reference), Some(fingerprint)) => {
                let reference = reference.trim();
                let fingerprint = fingerprint.trim();
                !reference.is_empty() && !fingerprint.is_empty() && fingerprint != reference
            }
            _ => false,
        }
    }

    /// Whether this verification is well-formed: a missing proof carries no ref,
    /// any other currency carries a reopenable proof, and the summary is present.
    pub fn is_well_formed(&self) -> bool {
        if self.summary.trim().is_empty() {
            return false;
        }
        if self.proof_currency.is_absent() {
            self.proof_ref.is_none() && self.proof_fingerprint_token.is_none()
        } else {
            self.proof_reopenable()
        }
    }

    /// Whether this verification backs a current claim for the given origin
    /// posture. A local surface needs locally verified or cached proof; a
    /// provider/imported surface needs current imported proof. Either way the
    /// proof must be reopenable.
    pub fn backs_claim(&self, provider_or_imported: bool) -> bool {
        if !self.proof_reopenable() {
            return false;
        }
        if provider_or_imported {
            self.proof_currency.is_imported_current()
        } else {
            self.proof_currency.is_current_local()
        }
    }
}

/// One claimed M5 surface row in the keyboard-continuity matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardParityRow {
    /// Stable row id.
    pub row_id: String,
    /// Kind of claimed M5 surface.
    pub surface_kind: KeyboardSurfaceKind,
    /// Durable subject the row covers.
    pub subject: KeyboardSurfaceSubject,
    /// Human-readable row label.
    pub label_summary: String,
    /// Whether the surface remains keyboard-complete (no mouse-only fallback).
    pub keyboard_complete: bool,
    /// Whether mode changes and macro replay are explicit on this surface.
    pub macro_replay_explicit: bool,
    /// Canonical mode-strip class.
    pub mode_strip: ModeStripClass,
    /// Canonical sequence-guide class.
    pub sequence_guide: SequenceGuideClass,
    /// Canonical clipboard-route class.
    pub clipboard_route: ClipboardRouteClass,
    /// Canonical drag/drop-verb class.
    pub drag_drop_verb: DragDropVerbClass,
    /// Canonical undo class.
    pub undo_class: UndoClass,
    /// Canonical grouped-history class.
    pub history_class: HistoryClass,
    /// Canonical reopen/recover class.
    pub reopen_recover: ReopenRecoverClass,
    /// Canonical orientation-aid class.
    pub orientation_aid: OrientationAidClass,
    /// Reopenable verification proof backing the parity claim.
    pub verification: AxisVerification,
    /// Headline parity grade publicly claimed for this row.
    pub claimed_grade: ContinuityParityGrade,
    /// Effective grade after auto-downgrading; equals the claim when every axis is
    /// supported and the proof is current, and ranks strictly below it otherwise.
    pub effective_grade: ContinuityParityGrade,
    /// Trigger that fired the downgrade, required when the row is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<ParityDowngradeTrigger>,
    /// Precise downgraded label, required when the row is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgraded_label: Option<String>,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl KeyboardParityRow {
    /// Whether parity for this row is provider-backed / imported.
    pub fn provider_or_imported(&self) -> bool {
        self.subject.is_provider_or_imported()
    }

    /// Whether the row carries a public parity claim.
    pub fn is_claimed(&self) -> bool {
        self.claimed_grade.is_claimed()
    }

    /// Whether any interaction axis is in an honestly-downgraded or denied state.
    pub fn any_axis_downgraded(&self) -> bool {
        self.mode_strip.is_downgraded()
            || self.sequence_guide.is_downgraded()
            || self.clipboard_route.is_denied()
            || self.drag_drop_verb.is_denied()
            || self.orientation_aid.is_downgraded()
    }

    /// Whether the verification proof backs a current claim for this row's origin
    /// posture.
    pub fn verification_current(&self) -> bool {
        self.verification.backs_claim(self.provider_or_imported())
    }

    /// Whether the row must downgrade below its claim because an axis is denied or
    /// downgraded, the surface stopped being keyboard-complete or macro-explicit,
    /// or the verification proof is not current.
    pub fn needs_downgrade(&self) -> bool {
        self.any_axis_downgraded()
            || !self.verification_current()
            || !self.keyboard_complete
            || !self.macro_replay_explicit
    }

    /// Whether the effective grade ranks strictly below the claim.
    pub fn properly_downgraded(&self) -> bool {
        self.effective_grade.rank() < self.claimed_grade.rank()
    }

    /// Whether the effective grade and downgrade evidence are consistent.
    ///
    /// When the row does not need downgrade the effective grade equals the claim;
    /// otherwise it must rank strictly below the claim and carry both a recorded
    /// trigger and a precise downgraded label.
    pub fn downgrade_consistent(&self) -> bool {
        if self.needs_downgrade() {
            self.properly_downgraded()
                && self.downgrade_trigger.is_some()
                && self
                    .downgraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label))
        } else {
            self.effective_grade == self.claimed_grade
        }
    }

    /// Whether the imported posture is consistent: a provider/imported surface
    /// never reads as a locally verified surface, and a local surface never leans
    /// on imported proof.
    pub fn imported_posture_consistent(&self) -> bool {
        if self.provider_or_imported() {
            !self.verification.proof_currency.is_current_local()
        } else {
            !self.verification.proof_currency.is_imported_current()
        }
    }

    /// Whether every field required to record this row is present and its
    /// invariants hold.
    pub fn is_complete(&self) -> bool {
        !self.row_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && self.subject.is_valid()
            && self.verification.is_well_formed()
            && self.downgrade_consistent()
            && self.imported_posture_consistent()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.source_contract_refs.is_empty()
            && self
                .source_contract_refs
                .iter()
                .all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardContinuityGuardrails {
    /// Unsupported modal sequences are never silently approximated.
    pub unsupported_sequences_never_silently_approximated: bool,
    /// Copy/export always preserves a useful plain-text representation.
    pub plain_text_copy_always_available: bool,
    /// Drag/drop never defaults to a destructive or ambiguous verb.
    pub drag_drop_never_destructive_by_default: bool,
    /// Exact, grouped exact, compensating, and checkpoint undo stay distinct.
    pub undo_classes_never_flattened: bool,
    /// Orientation aids degrade honestly rather than collapsing silently.
    pub orientation_aids_degrade_honestly: bool,
    /// Any claimed surface lacking identified behavior or current proof
    /// auto-downgrades below its claim.
    pub rows_auto_downgrade_without_identified_behavior: bool,
}

impl KeyboardContinuityGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.unsupported_sequences_never_silently_approximated
            && self.plain_text_copy_always_available
            && self.drag_drop_never_destructive_by_default
            && self.undo_classes_never_flattened
            && self.orientation_aids_degrade_honestly
            && self.rows_auto_downgrade_without_identified_behavior
    }
}

/// Consumer projection block: the surfaces that read this matrix without cloning
/// switching-wedge language by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardContinuityConsumerProjection {
    /// Product surfaces ingest this matrix.
    pub product_ingests_matrix: bool,
    /// Help / migration guidance ingests the same matrix.
    pub help_migration_ingests_matrix: bool,
    /// Accessibility surfaces ingest the same matrix.
    pub accessibility_ingests_matrix: bool,
    /// Diagnostics surfaces ingest the same matrix.
    pub diagnostics_ingests_matrix: bool,
    /// Release-control surfaces ingest the same matrix.
    pub release_control_ingests_matrix: bool,
    /// Downgraded rows are visibly labeled below their claim in every surface.
    pub downgraded_rows_labeled_below_claim: bool,
}

impl KeyboardContinuityConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_matrix
            && self.help_migration_ingests_matrix
            && self.accessibility_ingests_matrix
            && self.diagnostics_ingests_matrix
            && self.release_control_ingests_matrix
            && self.downgraded_rows_labeled_below_claim
    }
}

/// Verification freshness block for the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardContinuityFreshness {
    /// Verification-freshness SLO in hours.
    pub verification_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last verification refresh.
    pub last_verification_refresh: String,
    /// True when stale verification automatically downgrades claimed rows.
    pub auto_downgrade_on_stale: bool,
}

impl KeyboardContinuityFreshness {
    /// Whether the freshness block is well-formed.
    pub fn is_valid(&self) -> bool {
        self.verification_freshness_slo_hours > 0
            && !self.last_verification_refresh.trim().is_empty()
    }
}

/// Constructor input for [`KeyboardContinuityMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardContinuityMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub label: String,
    /// Per-surface parity rows.
    pub rows: Vec<KeyboardParityRow>,
    /// Guardrail invariants block.
    pub guardrails: KeyboardContinuityGuardrails,
    /// Consumer projection block.
    pub consumer_projection: KeyboardContinuityConsumerProjection,
    /// Verification freshness block.
    pub verification_freshness: KeyboardContinuityFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe keyboard-continuity matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardContinuityMatrixPacket {
    /// Record kind; must equal [`KEYBOARD_CONTINUITY_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`KEYBOARD_CONTINUITY_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub label: String,
    /// Per-surface parity rows.
    pub rows: Vec<KeyboardParityRow>,
    /// Guardrail invariants block.
    pub guardrails: KeyboardContinuityGuardrails,
    /// Consumer projection block.
    pub consumer_projection: KeyboardContinuityConsumerProjection,
    /// Verification freshness block.
    pub verification_freshness: KeyboardContinuityFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl KeyboardContinuityMatrixPacket {
    /// Builds a keyboard-continuity matrix packet.
    pub fn new(input: KeyboardContinuityMatrixPacketInput) -> Self {
        Self {
            record_kind: KEYBOARD_CONTINUITY_MATRIX_RECORD_KIND.to_owned(),
            schema_version: KEYBOARD_CONTINUITY_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            rows: input.rows,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            verification_freshness: input.verification_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surface kinds represented by some row in this packet.
    pub fn represented_surface_kinds(&self) -> BTreeSet<KeyboardSurfaceKind> {
        self.rows.iter().map(|row| row.surface_kind).collect()
    }

    /// Undo classes represented across rows.
    pub fn represented_undo_classes(&self) -> BTreeSet<UndoClass> {
        self.rows.iter().map(|row| row.undo_class).collect()
    }

    /// Proof currencies represented across rows.
    pub fn represented_currencies(&self) -> BTreeSet<AxisProofCurrency> {
        self.rows
            .iter()
            .map(|row| row.verification.proof_currency)
            .collect()
    }

    /// Count of rows that auto-downgraded below their claim.
    pub fn downgraded_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.needs_downgrade()).count()
    }

    /// Count of rows holding a public parity claim.
    pub fn claimed_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.is_claimed()).count()
    }

    /// Count of provider-linked / imported rows.
    pub fn provider_or_imported_row_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.provider_or_imported())
            .count()
    }

    /// Resolves a row by its id.
    pub fn row(&self, row_id: &str) -> Option<&KeyboardParityRow> {
        self.rows.iter().find(|row| row.row_id == row_id)
    }

    /// Validates the keyboard-continuity matrix invariants.
    pub fn validate(&self) -> Vec<KeyboardContinuityViolation> {
        let mut violations = Vec::new();

        if self.record_kind != KEYBOARD_CONTINUITY_MATRIX_RECORD_KIND {
            violations.push(KeyboardContinuityViolation::WrongRecordKind);
        }
        if self.schema_version != KEYBOARD_CONTINUITY_MATRIX_SCHEMA_VERSION {
            violations.push(KeyboardContinuityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(KeyboardContinuityViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(KeyboardContinuityViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(KeyboardContinuityViolation::ConsumerProjectionIncomplete);
        }
        if !self.verification_freshness.is_valid() {
            violations.push(KeyboardContinuityViolation::VerificationFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("keyboard continuity matrix packet serializes"),
        ) {
            violations.push(KeyboardContinuityViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("keyboard continuity matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, accessibility, or release
    /// handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Keyboard-Mode / Clipboard-Route / Drag-Drop-Verb / Grouped-History Continuity Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!(
            "- Rows: {} ({} claimed, {} provider/imported, {} downgraded)\n",
            self.rows.len(),
            self.claimed_row_count(),
            self.provider_or_imported_row_count(),
            self.downgraded_row_count()
        ));
        out.push_str(&format!(
            "- Surface kinds: {} / {}\n",
            self.represented_surface_kinds().len(),
            KeyboardSurfaceKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Undo classes: {} / {}\n",
            self.represented_undo_classes().len(),
            UndoClass::ALL.len()
        ));
        out.push_str(&format!(
            "- Verification freshness SLO: {} hours (last refresh: {})\n",
            self.verification_freshness.verification_freshness_slo_hours,
            self.verification_freshness.last_verification_refresh
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}): claim `{}` -> effective `{}`\n",
                row.row_id,
                row.surface_kind.as_str(),
                row.claimed_grade.as_str(),
                row.effective_grade.as_str()
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!(
                "  - subject `{}` ({}), keyboard_complete={}, macro_replay_explicit={}\n",
                row.subject.surface_id,
                row.subject.origin_class.as_str(),
                row.keyboard_complete,
                row.macro_replay_explicit
            ));
            out.push_str(&format!(
                "  - mode_strip = `{}`, sequence_guide = `{}`\n",
                row.mode_strip.as_str(),
                row.sequence_guide.as_str()
            ));
            out.push_str(&format!(
                "  - clipboard_route = `{}`, drag_drop_verb = `{}`\n",
                row.clipboard_route.as_str(),
                row.drag_drop_verb.as_str()
            ));
            out.push_str(&format!(
                "  - undo_class = `{}`, history_class = `{}`\n",
                row.undo_class.as_str(),
                row.history_class.as_str()
            ));
            out.push_str(&format!(
                "  - reopen_recover = `{}`, orientation_aid = `{}`\n",
                row.reopen_recover.as_str(),
                row.orientation_aid.as_str()
            ));
            out.push_str(&format!(
                "  - verification = `{}`\n",
                row.verification.proof_currency.as_str()
            ));
            if let Some(label) = &row.downgraded_label {
                out.push_str(&format!("  - Downgraded: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in packet export.
#[derive(Debug)]
pub enum KeyboardContinuityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<KeyboardContinuityViolation>),
}

impl fmt::Display for KeyboardContinuityArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "keyboard continuity matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "keyboard continuity matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for KeyboardContinuityArtifactError {}

/// Validation failures emitted by [`KeyboardContinuityMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyboardContinuityViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required claimed surface kind is represented by no row.
    RequiredSurfaceKindMissing,
    /// The distinct undo classes (exact / compensating / checkpoint) are not all
    /// represented, so the matrix cannot prove they stay unflattened.
    UndoClassCoverageMissing,
    /// No row demonstrates honest auto-downgrade on unidentified behavior.
    DowngradedRowCaseMissing,
    /// No fully keyboard-complete, current row anchors a clean claim.
    KeyboardCompleteCaseMissing,
    /// No provider-linked / imported row is present.
    ProviderOrImportedCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A claimed row was not downgraded below its claim despite unidentified
    /// behavior or uncurrent proof.
    RowNotDowngradedOnUnidentifiedBehavior,
    /// A downgraded row lacks a precise downgraded label or trigger.
    DowngradedRowMissingLabelOrTrigger,
    /// A row's subject fingerprint stands in for its bare id.
    FingerprintSubstitutesIdentity,
    /// A claimed surface is not keyboard-complete.
    SurfaceNotKeyboardComplete,
    /// A surface's mode changes / macro replay are not explicit.
    MacroReplayNotExplicit,
    /// A clipboard route dropped the plain-text representation without downgrading.
    ClipboardPlainTextLost,
    /// A drag/drop verb defaulted to a destructive / ambiguous action without
    /// downgrading.
    DragDropDestructiveDefault,
    /// Orientation aids collapsed without an honest downgrade.
    OrientationAidsCollapsedSilently,
    /// A provider/imported row reads as a locally verified surface.
    ImportedReadsAsLocal,
    /// A row's verification proof is not reopenable.
    VerificationProofNotReopenable,
    /// A row lacks evidence refs.
    RowEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Verification freshness block is incomplete.
    VerificationFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl KeyboardContinuityViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceKindMissing => "required_surface_kind_missing",
            Self::UndoClassCoverageMissing => "undo_class_coverage_missing",
            Self::DowngradedRowCaseMissing => "downgraded_row_case_missing",
            Self::KeyboardCompleteCaseMissing => "keyboard_complete_case_missing",
            Self::ProviderOrImportedCaseMissing => "provider_or_imported_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::RowNotDowngradedOnUnidentifiedBehavior => {
                "row_not_downgraded_on_unidentified_behavior"
            }
            Self::DowngradedRowMissingLabelOrTrigger => "downgraded_row_missing_label_or_trigger",
            Self::FingerprintSubstitutesIdentity => "fingerprint_substitutes_identity",
            Self::SurfaceNotKeyboardComplete => "surface_not_keyboard_complete",
            Self::MacroReplayNotExplicit => "macro_replay_not_explicit",
            Self::ClipboardPlainTextLost => "clipboard_plain_text_lost",
            Self::DragDropDestructiveDefault => "drag_drop_destructive_default",
            Self::OrientationAidsCollapsedSilently => "orientation_aids_collapsed_silently",
            Self::ImportedReadsAsLocal => "imported_reads_as_local",
            Self::VerificationProofNotReopenable => "verification_proof_not_reopenable",
            Self::RowEvidenceMissing => "row_evidence_missing",
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
pub fn current_keyboard_continuity_matrix_export(
) -> Result<KeyboardContinuityMatrixPacket, KeyboardContinuityArtifactError> {
    let packet: KeyboardContinuityMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/interaction/m5/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist/support_export.json"
    )))
    .map_err(KeyboardContinuityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(KeyboardContinuityArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &KeyboardContinuityMatrixPacket,
    violations: &mut Vec<KeyboardContinuityViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        KEYBOARD_CONTINUITY_MATRIX_SCHEMA_REF,
        KEYBOARD_CONTINUITY_MATRIX_DOC_REF,
        KEYBOARD_CONTINUITY_MATRIX_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(KeyboardContinuityViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &KeyboardContinuityMatrixPacket,
    violations: &mut Vec<KeyboardContinuityViolation>,
) {
    let surface_kinds = packet.represented_surface_kinds();
    for required in KeyboardSurfaceKind::ALL {
        if !surface_kinds.contains(&required) {
            violations.push(KeyboardContinuityViolation::RequiredSurfaceKindMissing);
            break;
        }
    }

    let undo_classes = packet.represented_undo_classes();
    for required in UndoClass::DISTINCT_CORE {
        if !undo_classes.contains(&required) {
            violations.push(KeyboardContinuityViolation::UndoClassCoverageMissing);
            break;
        }
    }

    if !packet
        .rows
        .iter()
        .any(|row| row.needs_downgrade() && row.downgrade_consistent())
    {
        violations.push(KeyboardContinuityViolation::DowngradedRowCaseMissing);
    }

    if !packet.rows.iter().any(|row| {
        !row.needs_downgrade()
            && row.is_claimed()
            && row.keyboard_complete
            && row.verification_current()
    }) {
        violations.push(KeyboardContinuityViolation::KeyboardCompleteCaseMissing);
    }

    if packet.provider_or_imported_row_count() == 0 {
        violations.push(KeyboardContinuityViolation::ProviderOrImportedCaseMissing);
    }
}

fn validate_rows(
    packet: &KeyboardContinuityMatrixPacket,
    violations: &mut Vec<KeyboardContinuityViolation>,
) {
    for row in &packet.rows {
        if !row.is_complete() {
            violations.push(KeyboardContinuityViolation::RowIncomplete);
        }
        if row.needs_downgrade() && !row.properly_downgraded() {
            violations.push(KeyboardContinuityViolation::RowNotDowngradedOnUnidentifiedBehavior);
        }
        if row.needs_downgrade()
            && (row.downgrade_trigger.is_none()
                || !row
                    .downgraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations.push(KeyboardContinuityViolation::DowngradedRowMissingLabelOrTrigger);
        }
        if !row.subject.fingerprint_independent_of_id() {
            violations.push(KeyboardContinuityViolation::FingerprintSubstitutesIdentity);
        }
        if !row.keyboard_complete {
            violations.push(KeyboardContinuityViolation::SurfaceNotKeyboardComplete);
        }
        if !row.macro_replay_explicit {
            violations.push(KeyboardContinuityViolation::MacroReplayNotExplicit);
        }
        if row.clipboard_route.is_denied() && !row.properly_downgraded() {
            violations.push(KeyboardContinuityViolation::ClipboardPlainTextLost);
        }
        if row.drag_drop_verb.is_denied() && !row.properly_downgraded() {
            violations.push(KeyboardContinuityViolation::DragDropDestructiveDefault);
        }
        if row.orientation_aid.is_downgraded() && !row.properly_downgraded() {
            violations.push(KeyboardContinuityViolation::OrientationAidsCollapsedSilently);
        }
        if !row.imported_posture_consistent() {
            violations.push(KeyboardContinuityViolation::ImportedReadsAsLocal);
        }
        if !row.verification.is_well_formed() {
            violations.push(KeyboardContinuityViolation::VerificationProofNotReopenable);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(KeyboardContinuityViolation::RowEvidenceMissing);
        }
    }
}

/// Whether a downgraded label is a generic non-answer rather than a precise label.
///
/// A generic provider error must never stand in for a precise downgrade truth.
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
            | "downgraded"
            | "unverified"
    )
}

/// Stable packet id minted by [`seeded_keyboard_continuity_matrix_packet`].
pub const SEED_KEYBOARD_CONTINUITY_PACKET_ID: &str = "m5-keyboard-continuity-matrix:stable:0001";

/// Mint timestamp used by [`seeded_keyboard_continuity_matrix_packet`].
pub const SEED_KEYBOARD_CONTINUITY_MINTED_AT: &str = "2026-06-14T00:00:00Z";

/// Builds the canonical, validating keyboard-continuity matrix packet that the
/// checked-in support export, the Markdown summary, and the conformance tests all
/// share, so the in-crate builder stays byte-aligned with the artifact.
///
/// The seed covers every claimed M5 surface kind, exercises the distinct undo
/// classes the matrix must keep unflattened, anchors clean switching-certified
/// rows, holds one provider-linked row that never reads as a local rerun, and
/// includes one row that auto-downgrades honestly because a claimed modal
/// sequence is unsupported.
pub fn seeded_keyboard_continuity_matrix_packet() -> KeyboardContinuityMatrixPacket {
    KeyboardContinuityMatrixPacket::new(KeyboardContinuityMatrixPacketInput {
        packet_id: SEED_KEYBOARD_CONTINUITY_PACKET_ID.to_owned(),
        label: "M5 Keyboard-Mode / Clipboard / Drag-Drop / Grouped-History Continuity Matrix"
            .to_owned(),
        rows: seeded_rows(),
        guardrails: KeyboardContinuityGuardrails {
            unsupported_sequences_never_silently_approximated: true,
            plain_text_copy_always_available: true,
            drag_drop_never_destructive_by_default: true,
            undo_classes_never_flattened: true,
            orientation_aids_degrade_honestly: true,
            rows_auto_downgrade_without_identified_behavior: true,
        },
        consumer_projection: KeyboardContinuityConsumerProjection {
            product_ingests_matrix: true,
            help_migration_ingests_matrix: true,
            accessibility_ingests_matrix: true,
            diagnostics_ingests_matrix: true,
            release_control_ingests_matrix: true,
            downgraded_rows_labeled_below_claim: true,
        },
        verification_freshness: KeyboardContinuityFreshness {
            verification_freshness_slo_hours: 168,
            last_verification_refresh: SEED_KEYBOARD_CONTINUITY_MINTED_AT.to_owned(),
            auto_downgrade_on_stale: true,
        },
        source_contract_refs: vec![
            KEYBOARD_CONTINUITY_MATRIX_SCHEMA_REF.to_owned(),
            KEYBOARD_CONTINUITY_MATRIX_DOC_REF.to_owned(),
            KEYBOARD_CONTINUITY_MATRIX_ARTIFACT_REF.to_owned(),
            "shell:interaction_transfer_beta:v1".to_owned(),
            "shell:interaction_integrity_beta:v1".to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_KEYBOARD_CONTINUITY_MINTED_AT.to_owned(),
    })
}

fn seeded_rows() -> Vec<KeyboardParityRow> {
    vec![
        seed_row(SeedRow {
            row_id: "kbd-cont:editor-core:0001",
            surface_kind: KeyboardSurfaceKind::EditorCore,
            origin_class: SurfaceOriginClass::FirstPartySurface,
            label: "Editor / diff core with full modal parity, leader sequences, and cross-surface history",
            mode_strip: ModeStripClass::ModalParityComplete,
            sequence_guide: SequenceGuideClass::LeaderSequenceComplete,
            clipboard_route: ClipboardRouteClass::PlainTextPreserved,
            drag_drop_verb: DragDropVerbClass::MoveVerbExplicit,
            undo_class: UndoClass::ExactUndo,
            history_class: HistoryClass::CrossSurfaceContinuity,
            reopen_recover: ReopenRecoverClass::ExactReopen,
            orientation_aid: OrientationAidClass::FullOrientationAids,
            currency: AxisProofCurrency::VerifiedCurrent,
            claimed: ContinuityParityGrade::SwitchingCertified,
        }),
        seed_row(SeedRow {
            row_id: "kbd-cont:notebook:0001",
            surface_kind: KeyboardSurfaceKind::NotebookSurface,
            origin_class: SurfaceOriginClass::FirstPartySurface,
            label: "Notebook cells with modal parity, named-register routing, and grouped exact undo",
            mode_strip: ModeStripClass::ModalParityComplete,
            sequence_guide: SequenceGuideClass::PrefixDiscoverable,
            clipboard_route: ClipboardRouteClass::NamedRegisterRouted,
            drag_drop_verb: DragDropVerbClass::CopyVerbExplicit,
            undo_class: UndoClass::GroupedExactUndo,
            history_class: HistoryClass::GroupedStepHistory,
            reopen_recover: ReopenRecoverClass::ExactReopen,
            orientation_aid: OrientationAidClass::FullOrientationAids,
            currency: AxisProofCurrency::VerifiedCurrent,
            claimed: ContinuityParityGrade::ParityComplete,
        }),
        seed_row(SeedRow {
            row_id: "kbd-cont:data-api:0001",
            surface_kind: KeyboardSurfaceKind::DataApiSurface,
            origin_class: SurfaceOriginClass::FirstPartySurface,
            label: "Data-grid / API result with keyboard-complete navigation, sensitive-copy warning, and compensating undo",
            mode_strip: ModeStripClass::NonModalKeyboardComplete,
            sequence_guide: SequenceGuideClass::SingleStrokeOnly,
            clipboard_route: ClipboardRouteClass::SensitiveCopyWarned,
            drag_drop_verb: DragDropVerbClass::VerbChoiceOnModifier,
            undo_class: UndoClass::CompensatingUndo,
            history_class: HistoryClass::BranchingHistory,
            reopen_recover: ReopenRecoverClass::RecoveredApproximate,
            orientation_aid: OrientationAidClass::ReducedOrientationAidsHonest,
            currency: AxisProofCurrency::CachedWithinWindow,
            claimed: ContinuityParityGrade::ParityComplete,
        }),
        seed_row(SeedRow {
            row_id: "kbd-cont:preview:0001",
            surface_kind: KeyboardSurfaceKind::PreviewSurface,
            origin_class: SurfaceOriginClass::FirstPartySurface,
            label: "Source-first preview with read-only modal navigation, plain-text copy, and checkpoint restore",
            mode_strip: ModeStripClass::ModalReadOnlyNavigation,
            sequence_guide: SequenceGuideClass::PrefixDiscoverable,
            clipboard_route: ClipboardRouteClass::PlainTextPreserved,
            drag_drop_verb: DragDropVerbClass::LinkVerbExplicit,
            undo_class: UndoClass::CheckpointRestore,
            history_class: HistoryClass::CheckpointLineage,
            reopen_recover: ReopenRecoverClass::CheckpointRecover,
            orientation_aid: OrientationAidClass::FullOrientationAids,
            currency: AxisProofCurrency::VerifiedCurrent,
            claimed: ContinuityParityGrade::ParityComplete,
        }),
        seed_row(SeedRow {
            row_id: "kbd-cont:docs:0001",
            surface_kind: KeyboardSurfaceKind::DocsSurface,
            origin_class: SurfaceOriginClass::FirstPartySurface,
            label: "Docs authoring with modal parity, leader sequences, and rich-with-plain-fallback copy",
            mode_strip: ModeStripClass::ModalParityComplete,
            sequence_guide: SequenceGuideClass::LeaderSequenceComplete,
            clipboard_route: ClipboardRouteClass::RichWithPlainFallback,
            drag_drop_verb: DragDropVerbClass::CopyVerbExplicit,
            undo_class: UndoClass::ExactUndo,
            history_class: HistoryClass::LinearStepHistory,
            reopen_recover: ReopenRecoverClass::ExactReopen,
            orientation_aid: OrientationAidClass::FullOrientationAids,
            currency: AxisProofCurrency::VerifiedCurrent,
            claimed: ContinuityParityGrade::ParityComplete,
        }),
        seed_row(SeedRow {
            row_id: "kbd-cont:review:0001",
            surface_kind: KeyboardSurfaceKind::ReviewSurface,
            origin_class: SurfaceOriginClass::FirstPartySurface,
            label: "Review / pull-request panel with keyboard-complete navigation, explicit drop verbs, and cross-surface history",
            mode_strip: ModeStripClass::NonModalKeyboardComplete,
            sequence_guide: SequenceGuideClass::PrefixDiscoverable,
            clipboard_route: ClipboardRouteClass::PlainTextPreserved,
            drag_drop_verb: DragDropVerbClass::VerbChoiceOnModifier,
            undo_class: UndoClass::GroupedExactUndo,
            history_class: HistoryClass::CrossSurfaceContinuity,
            reopen_recover: ReopenRecoverClass::ExactReopen,
            orientation_aid: OrientationAidClass::FullOrientationAids,
            currency: AxisProofCurrency::VerifiedCurrent,
            claimed: ContinuityParityGrade::SwitchingCertified,
        }),
        seed_row(SeedRow {
            row_id: "kbd-cont:runtime:0001",
            surface_kind: KeyboardSurfaceKind::RuntimeSurface,
            origin_class: SurfaceOriginClass::EmbeddedRuntimeSurface,
            label: "Embedded runtime / terminal-adjacent surface that honestly states no exact undo and an unavailable reopen",
            mode_strip: ModeStripClass::NonModalKeyboardComplete,
            sequence_guide: SequenceGuideClass::SingleStrokeOnly,
            clipboard_route: ClipboardRouteClass::SensitiveCopyWarned,
            drag_drop_verb: DragDropVerbClass::CopyVerbExplicit,
            undo_class: UndoClass::NoUndoHonest,
            history_class: HistoryClass::LinearStepHistory,
            reopen_recover: ReopenRecoverClass::ReopenUnavailableHonest,
            orientation_aid: OrientationAidClass::OrientationAidsDegradedHonest,
            currency: AxisProofCurrency::VerifiedCurrent,
            claimed: ContinuityParityGrade::ParityPartial,
        }),
        seed_row(SeedRow {
            row_id: "kbd-cont:companion:0001",
            surface_kind: KeyboardSurfaceKind::CompanionSurface,
            origin_class: SurfaceOriginClass::ProviderLinkedSurface,
            label: "Provider-linked companion surface whose parity is provider-backed and never reads as a local rerun",
            mode_strip: ModeStripClass::ModalReadOnlyNavigation,
            sequence_guide: SequenceGuideClass::PrefixDiscoverable,
            clipboard_route: ClipboardRouteClass::RichWithPlainFallback,
            drag_drop_verb: DragDropVerbClass::LinkVerbExplicit,
            undo_class: UndoClass::CompensatingUndo,
            history_class: HistoryClass::GroupedStepHistory,
            reopen_recover: ReopenRecoverClass::RecoveredApproximate,
            orientation_aid: OrientationAidClass::ReducedOrientationAidsHonest,
            currency: AxisProofCurrency::ImportedCurrent,
            claimed: ContinuityParityGrade::ParityPartial,
        }),
        downgraded_sequence_row(),
    ]
}

/// Inline constructor input for one seeded parity row.
struct SeedRow {
    row_id: &'static str,
    surface_kind: KeyboardSurfaceKind,
    origin_class: SurfaceOriginClass,
    label: &'static str,
    mode_strip: ModeStripClass,
    sequence_guide: SequenceGuideClass,
    clipboard_route: ClipboardRouteClass,
    drag_drop_verb: DragDropVerbClass,
    undo_class: UndoClass,
    history_class: HistoryClass,
    reopen_recover: ReopenRecoverClass,
    orientation_aid: OrientationAidClass,
    currency: AxisProofCurrency,
    claimed: ContinuityParityGrade,
}

fn seed_row(seed: SeedRow) -> KeyboardParityRow {
    let (proof_ref, proof_fingerprint_token) = if seed.currency.is_absent() {
        (None, None)
    } else {
        (
            Some(format!("evidence:{}", seed.row_id)),
            Some(format!("fp:proof:{}", seed.row_id)),
        )
    };
    KeyboardParityRow {
        row_id: seed.row_id.to_owned(),
        surface_kind: seed.surface_kind,
        subject: KeyboardSurfaceSubject {
            surface_id: format!("surface:{}", seed.row_id),
            origin_class: seed.origin_class,
            surface_fingerprint_token: format!("fp:surface:{}", seed.row_id),
        },
        label_summary: seed.label.to_owned(),
        keyboard_complete: true,
        macro_replay_explicit: true,
        mode_strip: seed.mode_strip,
        sequence_guide: seed.sequence_guide,
        clipboard_route: seed.clipboard_route,
        drag_drop_verb: seed.drag_drop_verb,
        undo_class: seed.undo_class,
        history_class: seed.history_class,
        reopen_recover: seed.reopen_recover,
        orientation_aid: seed.orientation_aid,
        verification: AxisVerification {
            proof_currency: seed.currency,
            proof_ref,
            proof_fingerprint_token,
            summary: format!(
                "{} parity verified with {} proof",
                seed.surface_kind.as_str(),
                seed.currency.as_str()
            ),
        },
        claimed_grade: seed.claimed,
        effective_grade: seed.claimed,
        downgrade_trigger: None,
        downgraded_label: None,
        evidence_refs: vec![format!("evidence:row:{}", seed.row_id)],
        source_contract_refs: vec![KEYBOARD_CONTINUITY_MATRIX_DOC_REF.to_owned()],
    }
}

/// A row that auto-downgrades because a claimed modal sequence is unsupported on
/// this surface — the matrix downgrades honestly rather than silently
/// approximating the sequence.
fn downgraded_sequence_row() -> KeyboardParityRow {
    let mut row = seed_row(SeedRow {
        row_id: "kbd-cont:data-api:unsupported-sequence:0001",
        surface_kind: KeyboardSurfaceKind::DataApiSurface,
        origin_class: SurfaceOriginClass::FirstPartySurface,
        label:
            "Data-grid row that claimed leader-key parity but cannot host the multi-key sequence",
        mode_strip: ModeStripClass::NonModalKeyboardComplete,
        sequence_guide: SequenceGuideClass::SequenceUnsupportedDowngraded,
        clipboard_route: ClipboardRouteClass::PlainTextPreserved,
        drag_drop_verb: DragDropVerbClass::MoveVerbExplicit,
        undo_class: UndoClass::ExactUndo,
        history_class: HistoryClass::LinearStepHistory,
        reopen_recover: ReopenRecoverClass::ExactReopen,
        orientation_aid: OrientationAidClass::FullOrientationAids,
        currency: AxisProofCurrency::VerifiedCurrent,
        claimed: ContinuityParityGrade::ParityComplete,
    });
    row.effective_grade = ContinuityParityGrade::ParityUnverified;
    row.downgrade_trigger = Some(ParityDowngradeTrigger::UnsupportedSequenceDowngraded);
    row.downgraded_label = Some(
        "Claimed leader-key sequence is unsupported on this surface; held parity-unverified rather than silently approximating the sequence"
            .to_owned(),
    );
    row
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
