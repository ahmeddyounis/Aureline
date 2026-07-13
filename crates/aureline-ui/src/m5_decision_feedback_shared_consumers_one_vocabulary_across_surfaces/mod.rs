//! Shared shell / help / entry / trust-repair / update-advisory / provider-account /
//! export-support consumers that keep the B135 decision and feedback primitives — badges /
//! chips / pills, popovers, dialogs / sheets, banners / inline notices, toasts, empty
//! states, loading states, and consequence blocks — at **one vocabulary** across every
//! claimed M5 surface.
//!
//! This module is the closing consumer-adoption lane for the eight reusable decision /
//! feedback primitives frozen in [`crate::m5_decision_feedback_component_matrix`] and
//! implemented by the badge / popover lane
//! ([`crate::m5_badge_chip_pill_and_popover_expansion_and_anchored_focus_return`]), the
//! dialog / consequence lane
//! ([`crate::m5_dialog_sheet_and_consequence_block_rationale_scope_and_rollback_continuity`]),
//! the banner / empty-state lane
//! ([`crate::m5_banner_inline_notice_and_empty_state_scoped_cause_and_next_action`]), and
//! the toast / loading-state lane
//! ([`crate::m5_toast_and_loading_state_acknowledgement_and_loading_fidelity`]).
//!
//! It binds each shared primitive to the concrete shell, help, support, review, settings,
//! updates, CLI/export, and support-export consumers that render it, and proves — by
//! fixtures, not screenshots — that the same primitive object presents the same
//! disposition, scope, severity, rationale, recovery-path, and durable-object vocabulary
//! wherever it appears.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **Reuse.** Each of the eight shared primitives must be adopted by at least two
//!    distinct consumers, so a primitive is proven to be shared product infrastructure
//!    rather than a one-surface feature-local fork.
//! 2. **One vocabulary / no drift.** For a given primitive object every consumer surface
//!    must present identical [`DecisionFeedbackStateFacetValues`] — the same disposition
//!    word, the same scope word, the same severity word, the same rationale word, the same
//!    recovery-path word, and the same durable-object word. The disposition word must be a
//!    token from the frozen [`M5DecisionFeedbackDisposition`] vocabulary, so no feature
//!    rewrites `info`, `success`, `warning`, `blocked`, `pending`, `degraded`,
//!    `acknowledged`, or `dismissed` in its own words. A surface may narrow *how much* it
//!    shows across desktop, compact, remote, and exported representations, but it may never
//!    reword the underlying vocabulary per surface.
//! 3. **Map back to one family.** Support and CLI/export consumers must point at the
//!    canonical per-primitive schema and the frozen matrix by id, so an exported packet can
//!    always map a shell / help / review / settings / updates decision-feedback surface back
//!    to one shared contract family.
//!
//! Narrowing is disclosed, never hidden: a compact, remote, or exported representation
//! carries an explicit [`DecisionFeedbackNarrowNote`] naming the reason, the preserved
//! vocabulary, and the next action, and an exported representation additionally names its
//! export-safe detail boundary rather than collapsing the object out of view.
//!
//! The packet references upstream primitive contracts by id rather than embedding their
//! content. Raw secret values, credentials, and private endpoints stay outside the support
//! boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-decision-feedback-shared-consumers.schema.json`](../../../../schemas/ui/m5-decision-feedback-shared-consumers.schema.json).
//! The contract doc is
//! [`docs/components/m5_decision_feedback_shared_consumers_one_vocabulary.md`](../../../../docs/components/m5_decision_feedback_shared_consumers_one_vocabulary.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-decision-feedback-shared-consumers/`](../../../../fixtures/ui/m5-decision-feedback-shared-consumers/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_decision_feedback_shared_consumers,
    seeded_m5_decision_feedback_shared_consumers_compact_remote_narrowed,
    seeded_m5_decision_feedback_shared_consumers_exported_redaction_narrowed,
};

use crate::m5_decision_feedback_component_matrix::{
    M5DecisionFeedbackConsumerSurface, M5DecisionFeedbackDisposition, M5DecisionFeedbackFamily,
    M5_DECISION_FEEDBACK_COMPONENT_DOC_REF, M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5DecisionFeedbackSharedConsumersPacket`].
pub const M5_DECISION_FEEDBACK_SHARED_CONSUMERS_RECORD_KIND: &str =
    "m5_decision_feedback_shared_consumer_vocabulary_parity";

/// Schema version for decision-feedback shared-consumer parity records.
pub const M5_DECISION_FEEDBACK_SHARED_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_DECISION_FEEDBACK_SHARED_CONSUMERS_PACKET_ID: &str =
    "m5-decision-feedback-shared-consumers:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_DECISION_FEEDBACK_SHARED_CONSUMERS_SCHEMA_REF: &str =
    "schemas/ui/m5-decision-feedback-shared-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_DECISION_FEEDBACK_SHARED_CONSUMERS_DOC_REF: &str =
    "docs/components/m5_decision_feedback_shared_consumers_one_vocabulary.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DECISION_FEEDBACK_SHARED_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/release/m5-decision-feedback-shared-consumers-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_DECISION_FEEDBACK_SHARED_CONSUMERS_CSV_REF: &str =
    "artifacts/release/m5-decision-feedback-shared-consumers-proof/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_DECISION_FEEDBACK_SHARED_CONSUMERS_REPORT_REF: &str =
    "artifacts/release/m5-decision-feedback-shared-consumers-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_DECISION_FEEDBACK_SHARED_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-decision-feedback-shared-consumers";

/// Proof-freshness SLO in hours for this lane.
pub const M5_DECISION_FEEDBACK_SHARED_CONSUMERS_PROOF_SLO_HOURS: u32 = 720;

/// Whether a consumer surface is an export / support path that must map a primitive back to
/// its canonical contract family by id.
pub const fn consumer_must_reference_canonical(
    consumer: M5DecisionFeedbackConsumerSurface,
) -> bool {
    matches!(
        consumer,
        M5DecisionFeedbackConsumerSurface::SupportExport
            | M5DecisionFeedbackConsumerSurface::CliExport
    )
}

/// Whether `token` is a member of the frozen [`M5DecisionFeedbackDisposition`] vocabulary.
///
/// This is the "one vocabulary" gate: a primitive object's disposition word must be a
/// controlled disposition token rather than a per-surface synonym.
pub fn is_known_disposition_token(token: &str) -> bool {
    M5DecisionFeedbackDisposition::ALL
        .iter()
        .any(|disposition| disposition.as_str() == token)
}

/// How much of a shared primitive a consumer renders for one representation.
///
/// Narrowing changes how much is shown, never the underlying vocabulary: a narrowed
/// representation still carries the same disposition, scope, severity, rationale,
/// recovery-path, and durable-object words, and discloses the narrowing through an explicit
/// note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionFeedbackRepresentation {
    /// The full desktop representation; nothing is narrowed.
    DesktopFull,
    /// A compact representation that narrows disclosure depth.
    CompactNarrowed,
    /// A remote-projected representation backed by a remote source.
    RemoteProjected,
    /// An exported, export-safe-redacted representation.
    ExportedRedacted,
}

impl DecisionFeedbackRepresentation {
    /// Every representation, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DesktopFull,
        Self::CompactNarrowed,
        Self::RemoteProjected,
        Self::ExportedRedacted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompactNarrowed => "compact_narrowed",
            Self::RemoteProjected => "remote_projected",
            Self::ExportedRedacted => "exported_redacted",
        }
    }

    /// Whether this representation narrows below full desktop disclosure.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }
}

/// A vocabulary axis whose word must stay identical across surfaces for one object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionFeedbackParityFacet {
    /// The primitive disposition / state word (a frozen disposition token).
    DispositionWord,
    /// The scope word naming what the primitive is about.
    ScopeWord,
    /// The severity word.
    SeverityWord,
    /// The rationale word explaining why the primitive is shown.
    RationaleWord,
    /// The recovery-path word naming the next action.
    RecoveryPathWord,
    /// The durable-object word linking to the durable record behind the primitive.
    DurableObjectWord,
}

impl DecisionFeedbackParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DispositionWord,
        Self::ScopeWord,
        Self::SeverityWord,
        Self::RationaleWord,
        Self::RecoveryPathWord,
        Self::DurableObjectWord,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DispositionWord => "disposition_word",
            Self::ScopeWord => "scope_word",
            Self::SeverityWord => "severity_word",
            Self::RationaleWord => "rationale_word",
            Self::RecoveryPathWord => "recovery_path_word",
            Self::DurableObjectWord => "durable_object_word",
        }
    }
}

/// Why a surface narrowed its rendering of a shared primitive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionFeedbackNarrowReason {
    /// A compact representation narrowed disclosure depth.
    CompactionNarrowed,
    /// A remote-projected representation narrowed to remote-backed truth.
    RemoteProjectionNarrowed,
    /// An exported representation narrowed to export-safe-redacted truth.
    ExportRedactionNarrowed,
}

impl DecisionFeedbackNarrowReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompactionNarrowed => "compaction_narrowed",
            Self::RemoteProjectionNarrowed => "remote_projection_narrowed",
            Self::ExportRedactionNarrowed => "export_redaction_narrowed",
        }
    }
}

/// The next action a narrow note offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionFeedbackNarrowNextAction {
    /// Expand the primitive in the full desktop representation.
    ExpandInDesktop,
    /// Open the remote source backing the projection.
    OpenRemoteSource,
    /// Open the full detail behind the redacted export.
    OpenFullDetail,
}

impl DecisionFeedbackNarrowNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandInDesktop => "expand_in_desktop",
            Self::OpenRemoteSource => "open_remote_source",
            Self::OpenFullDetail => "open_full_detail",
        }
    }
}

/// Whether a binding preserves full parity or discloses a narrowed representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionFeedbackParityState {
    /// All vocabulary is preserved and shown in full.
    FacetsPreserved,
    /// All vocabulary is preserved and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl DecisionFeedbackParityState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetsPreserved => "facets_preserved",
            Self::FacetsDisclosedNarrowed => "facets_disclosed_narrowed",
        }
    }
}

/// Downgrade trigger that can narrow this consumer lane below its claimed parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionFeedbackSharedConsumersDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Primitive vocabulary drifted between surfaces for the same object.
    VocabularyDriftDetected,
    /// A primitive relied on color alone to carry meaning.
    ColorAloneUsedForMeaning,
    /// A popover carried the only critical workflow instruction.
    PopoverCarriedOnlyCriticalInstruction,
    /// A high-risk dialog used generic Yes/No confirmation copy.
    GenericYesNoConfirmationCopyUsed,
    /// Long-running or reviewable work was represented as toast-only truth.
    DurableWorkShownAsToastOnly,
    /// A useful pane was blanked during loading.
    UsefulPaneBlankedDuringLoading,
    /// A full-screen spinner was used where partial capability existed.
    FullScreenSpinnerWhenPartialCapable,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalReferenceMissing,
    /// An upstream shared primitive narrowed.
    UpstreamPrimitiveNarrowed,
}

impl DecisionFeedbackSharedConsumersDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::VocabularyDriftDetected,
        Self::ColorAloneUsedForMeaning,
        Self::PopoverCarriedOnlyCriticalInstruction,
        Self::GenericYesNoConfirmationCopyUsed,
        Self::DurableWorkShownAsToastOnly,
        Self::UsefulPaneBlankedDuringLoading,
        Self::FullScreenSpinnerWhenPartialCapable,
        Self::CanonicalReferenceMissing,
        Self::UpstreamPrimitiveNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::VocabularyDriftDetected => "vocabulary_drift_detected",
            Self::ColorAloneUsedForMeaning => "color_alone_used_for_meaning",
            Self::PopoverCarriedOnlyCriticalInstruction => {
                "popover_carried_only_critical_instruction"
            }
            Self::GenericYesNoConfirmationCopyUsed => "generic_yes_no_confirmation_copy_used",
            Self::DurableWorkShownAsToastOnly => "durable_work_shown_as_toast_only",
            Self::UsefulPaneBlankedDuringLoading => "useful_pane_blanked_during_loading",
            Self::FullScreenSpinnerWhenPartialCapable => "full_screen_spinner_when_partial_capable",
            Self::CanonicalReferenceMissing => "canonical_reference_missing",
            Self::UpstreamPrimitiveNarrowed => "upstream_primitive_narrowed",
        }
    }
}

/// The controlled vocabulary a primitive object presents.
///
/// These six words must be identical across every consumer surface that shows the same
/// primitive object. The disposition word must be a frozen disposition token; the rest are
/// controlled words the object's family carries. A surface may narrow how much it renders,
/// but it may never reword any of these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionFeedbackStateFacetValues {
    /// Primitive disposition / state word (must be a frozen disposition token).
    pub disposition_word: String,
    /// Scope word naming what the primitive is about.
    pub scope_word: String,
    /// Severity word.
    pub severity_word: String,
    /// Rationale word explaining why the primitive is shown.
    pub rationale_word: String,
    /// Recovery-path word naming the next action.
    pub recovery_path_word: String,
    /// Durable-object word linking to the durable record behind the primitive.
    pub durable_object_word: String,
}

impl DecisionFeedbackStateFacetValues {
    /// Whether every vocabulary word is present.
    pub fn all_present(&self) -> bool {
        !self.disposition_word.trim().is_empty()
            && !self.scope_word.trim().is_empty()
            && !self.severity_word.trim().is_empty()
            && !self.rationale_word.trim().is_empty()
            && !self.recovery_path_word.trim().is_empty()
            && !self.durable_object_word.trim().is_empty()
    }

    /// Whether the disposition word is a member of the frozen disposition vocabulary.
    pub fn disposition_word_in_vocabulary(&self) -> bool {
        is_known_disposition_token(self.disposition_word.trim())
    }
}

/// The explicit note a narrowed representation shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionFeedbackNarrowNote {
    /// Why the representation narrowed.
    pub reason: DecisionFeedbackNarrowReason,
    /// Note naming the preserved vocabulary (never omitted).
    pub preserved_vocabulary_note: String,
    /// The next action offered.
    pub next_action: DecisionFeedbackNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecisionFeedbackRenderDisclosure {
    /// The parity state the representation requires.
    pub parity_state: DecisionFeedbackParityState,
    /// The narrow reason the representation requires, if any.
    pub narrow_reason: Option<DecisionFeedbackNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<DecisionFeedbackNarrowNextAction>,
    /// Whether the binding must carry an explicit narrow note.
    pub needs_narrow_note: bool,
    /// Whether the binding must carry an explicit remote-source note.
    pub needs_remote_source_note: bool,
    /// Whether the binding must carry an explicit export-safe-detail note.
    pub needs_export_detail_note: bool,
}

/// Resolves the render disclosures a consumer binding must carry from its representation.
///
/// The full desktop representation renders at full parity. A compact representation
/// narrows disclosure depth, a remote-projected representation names its remote source,
/// and an exported representation names its export-safe-detail boundary — but all three
/// keep every vocabulary word and disclose the narrowing through an explicit note.
pub const fn resolve_decision_feedback_render_disclosure(
    representation: DecisionFeedbackRepresentation,
) -> DecisionFeedbackRenderDisclosure {
    match representation {
        DecisionFeedbackRepresentation::DesktopFull => DecisionFeedbackRenderDisclosure {
            parity_state: DecisionFeedbackParityState::FacetsPreserved,
            narrow_reason: None,
            narrow_next_action: None,
            needs_narrow_note: false,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        DecisionFeedbackRepresentation::CompactNarrowed => DecisionFeedbackRenderDisclosure {
            parity_state: DecisionFeedbackParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(DecisionFeedbackNarrowReason::CompactionNarrowed),
            narrow_next_action: Some(DecisionFeedbackNarrowNextAction::ExpandInDesktop),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        DecisionFeedbackRepresentation::RemoteProjected => DecisionFeedbackRenderDisclosure {
            parity_state: DecisionFeedbackParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(DecisionFeedbackNarrowReason::RemoteProjectionNarrowed),
            narrow_next_action: Some(DecisionFeedbackNarrowNextAction::OpenRemoteSource),
            needs_narrow_note: true,
            needs_remote_source_note: true,
            needs_export_detail_note: false,
        },
        DecisionFeedbackRepresentation::ExportedRedacted => DecisionFeedbackRenderDisclosure {
            parity_state: DecisionFeedbackParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(DecisionFeedbackNarrowReason::ExportRedactionNarrowed),
            narrow_next_action: Some(DecisionFeedbackNarrowNextAction::OpenFullDetail),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: true,
        },
    }
}

/// One consumer binding: a shared primitive rendered on one consumer surface in one
/// representation for one primitive object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionFeedbackConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable primitive-object id (shared across surfaces that show the same object).
    pub primitive_object_id: String,
    /// Human-readable primitive-object identity.
    pub primitive_object_label: String,
    /// Which shared primitive this binding renders.
    pub component: M5DecisionFeedbackFamily,
    /// Which consumer surface renders it.
    pub consumer: M5DecisionFeedbackConsumerSurface,
    /// Which representation this surface renders.
    pub representation: DecisionFeedbackRepresentation,
    /// The controlled vocabulary presented (identical across surfaces for one object).
    pub state_facets: DecisionFeedbackStateFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub parity_state: DecisionFeedbackParityState,
    /// The explicit narrow note; required and complete when the binding narrows.
    pub narrow_note: Option<DecisionFeedbackNarrowNote>,
    /// Remote-source note; required and non-empty when the disclosure demands it.
    pub remote_source_note: String,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Guardrail: this surface relies on color alone to carry meaning. MUST be `false`.
    pub relies_on_color_alone_for_meaning: bool,
    /// Guardrail: this surface lets a popover carry the only critical instruction. MUST be
    /// `false`.
    pub lets_a_popover_carry_the_only_critical_instruction: bool,
    /// Guardrail: this surface uses generic Yes/No confirmation copy in a high-risk dialog.
    /// MUST be `false`.
    pub uses_generic_yes_no_confirmation_copy: bool,
    /// Guardrail: this surface represents durable work as toast-only truth. MUST be `false`.
    pub represents_durable_work_as_toast_only_truth: bool,
    /// Guardrail: this surface blanks a useful pane during loading. MUST be `false`.
    pub blanks_a_useful_pane_during_loading: bool,
    /// Guardrail: this surface uses a full-screen spinner where partial capability exists.
    /// MUST be `false`.
    pub uses_a_full_screen_spinner_where_partial_capable: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl DecisionFeedbackConsumerBinding {
    /// Disclosures this binding must carry, derived from its representation.
    pub const fn disclosure(&self) -> DecisionFeedbackRenderDisclosure {
        resolve_decision_feedback_render_disclosure(self.representation)
    }

    /// Whether this binding renders below full parity.
    pub const fn is_narrowed(&self) -> bool {
        self.representation.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.relies_on_color_alone_for_meaning
            && !self.lets_a_popover_carry_the_only_critical_instruction
            && !self.uses_generic_yes_no_confirmation_copy
            && !self.represents_durable_work_as_toast_only_truth
            && !self.blanks_a_useful_pane_during_loading
            && !self.uses_a_full_screen_spinner_where_partial_capable
    }

    /// Whether this binding points at the canonical primitive schema and the matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let component_ref = self.component.canonical_component_schema_ref();
        self.source_contract_refs
            .iter()
            .any(|reference| reference == component_ref)
            && self
                .source_contract_refs
                .iter()
                .any(|reference| reference == M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionFeedbackSharedConsumersTrustReview {
    /// Primitive reuse is proven by fixtures rather than inferred from screenshots.
    pub primitive_reuse_proven_by_fixtures: bool,
    /// The same primitive object presents the same vocabulary across surfaces.
    pub same_object_same_vocabulary_across_surfaces: bool,
    /// Every disposition word is a frozen disposition token.
    pub disposition_words_stay_in_frozen_vocabulary: bool,
    /// Meaning never relies on color alone.
    pub meaning_never_relies_on_color_alone: bool,
    /// A popover never carries the only critical instruction.
    pub popover_never_carries_only_critical_instruction: bool,
    /// High-risk dialogs never use generic Yes/No copy.
    pub dialogs_never_use_generic_yes_no_copy: bool,
    /// A toast never becomes the only durable truth.
    pub toast_never_becomes_only_durable_truth: bool,
    /// Loading never blanks a useful pane.
    pub loading_never_blanks_useful_pane: bool,
    /// Loading never uses a full-screen spinner when partial capability exists.
    pub loading_never_uses_full_screen_spinner_when_partial_capable: bool,
    /// Narrowing is disclosed across desktop, compact, remote, and exported forms.
    pub narrowing_disclosed_across_representations: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the primitive.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl DecisionFeedbackSharedConsumersTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.primitive_reuse_proven_by_fixtures
            && self.same_object_same_vocabulary_across_surfaces
            && self.disposition_words_stay_in_frozen_vocabulary
            && self.meaning_never_relies_on_color_alone
            && self.popover_never_carries_only_critical_instruction
            && self.dialogs_never_use_generic_yes_no_copy
            && self.toast_never_becomes_only_durable_truth
            && self.loading_never_blanks_useful_pane
            && self.loading_never_uses_full_screen_spinner_when_partial_capable
            && self.narrowing_disclosed_across_representations
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionFeedbackSharedConsumersProjection {
    /// The shell UI reuses the shared primitives.
    pub shell_ui_reuses_shared_primitives: bool,
    /// The help UI reuses the shared primitives.
    pub help_ui_reuses_shared_primitives: bool,
    /// The support UI reuses the shared primitives.
    pub support_ui_reuses_shared_primitives: bool,
    /// The review UI reuses the shared primitives.
    pub review_ui_reuses_shared_primitives: bool,
    /// The settings UI reuses the shared primitives.
    pub settings_ui_reuses_shared_primitives: bool,
    /// The updates UI reuses the shared primitives.
    pub updates_ui_reuses_shared_primitives: bool,
    /// The support / export path reuses the shared primitives.
    pub support_export_reuses_shared_primitives: bool,
    /// Every primitive is adopted by two or more consumers.
    pub every_primitive_adopted_by_two_or_more_consumers: bool,
    /// Vocabulary is identical for the same primitive object.
    pub vocabulary_identical_for_same_object: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps a primitive back to one shared contract family.
    pub export_maps_back_to_one_contract_family: bool,
}

impl DecisionFeedbackSharedConsumersProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.shell_ui_reuses_shared_primitives
            && self.help_ui_reuses_shared_primitives
            && self.support_ui_reuses_shared_primitives
            && self.review_ui_reuses_shared_primitives
            && self.settings_ui_reuses_shared_primitives
            && self.updates_ui_reuses_shared_primitives
            && self.support_export_reuses_shared_primitives
            && self.every_primitive_adopted_by_two_or_more_consumers
            && self.vocabulary_identical_for_same_object
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_contract_family
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionFeedbackSharedConsumersProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5DecisionFeedbackSharedConsumersPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DecisionFeedbackSharedConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<DecisionFeedbackConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<DecisionFeedbackSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5DecisionFeedbackConsumerSurface>,
    /// Trust review block.
    pub trust_review: DecisionFeedbackSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: DecisionFeedbackSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: DecisionFeedbackSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe decision-feedback shared-consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DecisionFeedbackSharedConsumersPacket {
    /// Record kind; must equal [`M5_DECISION_FEEDBACK_SHARED_CONSUMERS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DECISION_FEEDBACK_SHARED_CONSUMERS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<DecisionFeedbackConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<DecisionFeedbackSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5DecisionFeedbackConsumerSurface>,
    /// Trust review block.
    pub trust_review: DecisionFeedbackSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: DecisionFeedbackSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: DecisionFeedbackSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DecisionFeedbackSharedConsumersPacket {
    /// Builds a decision-feedback shared-consumer packet from stable-lane input.
    pub fn new(input: M5DecisionFeedbackSharedConsumersPacketInput) -> Self {
        Self {
            record_kind: M5_DECISION_FEEDBACK_SHARED_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: M5_DECISION_FEEDBACK_SHARED_CONSUMERS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            consumer_bindings: input.consumer_bindings,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the decision-feedback shared-consumer parity invariants.
    pub fn validate(&self) -> Vec<M5DecisionFeedbackSharedConsumersViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DECISION_FEEDBACK_SHARED_CONSUMERS_RECORD_KIND {
            violations.push(M5DecisionFeedbackSharedConsumersViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DECISION_FEEDBACK_SHARED_CONSUMERS_SCHEMA_VERSION {
            violations.push(M5DecisionFeedbackSharedConsumersViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DecisionFeedbackSharedConsumersViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5DecisionFeedbackSharedConsumersViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5DecisionFeedbackSharedConsumersViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5DecisionFeedbackSharedConsumersViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(M5DecisionFeedbackSharedConsumersViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5DecisionFeedbackSharedConsumersViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("decision-feedback shared-consumer packet serializes"),
        ) {
            violations
                .push(M5DecisionFeedbackSharedConsumersViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("decision-feedback shared-consumer packet serializes")
    }

    /// Deterministic matrix CSV, one row per consumer binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out =
            String::from("component,consumer,representation,disposition_word,parity_state\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                binding.component.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.disposition_word,
                binding.parity_state.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .consumer_bindings
            .iter()
            .filter(|binding| binding.is_narrowed())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Shared Decision / Feedback Primitive Consumers: One Vocabulary Across Surfaces\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Consumer bindings: {} ({} narrowed)\n",
            self.consumer_bindings.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Consumer bindings\n\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "- **{}** [`{}`]: primitive `{}` on `{}`, representation `{}`, disposition `{}`\n",
                binding.primitive_object_label,
                binding.binding_id,
                binding.component.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.disposition_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in decision-feedback shared-consumer export.
#[derive(Debug)]
pub enum M5DecisionFeedbackSharedConsumersArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DecisionFeedbackSharedConsumersViolation>),
}

impl fmt::Display for M5DecisionFeedbackSharedConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "decision-feedback shared-consumer export parse failed: {error}"
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
                    "decision-feedback shared-consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DecisionFeedbackSharedConsumersArtifactError {}

/// Validation failures emitted by [`M5DecisionFeedbackSharedConsumersPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DecisionFeedbackSharedConsumersViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No consumer bindings are present.
    ConsumerBindingsMissing,
    /// A consumer binding is incomplete.
    BindingIncomplete,
    /// A binding's vocabulary values are incomplete.
    VocabularyFacetIncomplete,
    /// A binding's disposition word is not a frozen disposition token.
    DispositionWordOutsideVocabulary,
    /// A binding's parity state does not match its representation.
    ParityStateMismatch,
    /// Two surfaces show the same primitive object with different vocabulary.
    VocabularyDriftAcrossSurfaces,
    /// A shared primitive is not adopted by at least two distinct consumers.
    PrimitiveReuseUnproven,
    /// A support / export binding does not point at the canonical contracts.
    SupportExportReferenceMissing,
    /// A narrowed binding is missing its explicit narrow note.
    NarrowNoteMissing,
    /// A narrow note's reason does not match the required narrow reason.
    NarrowReasonMismatch,
    /// A narrow note's next action does not match the required next action.
    NarrowNextActionMismatch,
    /// A narrow note is missing its preserved-vocabulary note.
    NarrowNotePreservedVocabularyMissing,
    /// A narrow note is missing its next-action copy.
    NarrowNextActionLabelMissing,
    /// A full-desktop binding carries a narrow note it must not.
    UnexpectedNarrowNote,
    /// A binding that needs an explicit remote-source note is missing it.
    RemoteSourceNoteMissing,
    /// A binding that needs an explicit export-detail note is missing it.
    ExportDetailNoteMissing,
    /// A binding relies on color alone for meaning.
    ColorAloneForMeaning,
    /// A binding lets a popover carry the only critical instruction.
    PopoverCarriesOnlyCriticalInstruction,
    /// A binding uses generic Yes/No confirmation copy in a high-risk dialog.
    GenericYesNoConfirmationCopy,
    /// A binding represents durable work as toast-only truth.
    DurableWorkAsToastOnlyTruth,
    /// A binding blanks a useful pane during loading.
    UsefulPaneBlankedDuringLoading,
    /// A binding uses a full-screen spinner where partial capability exists.
    FullScreenSpinnerWherePartialCapable,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared primitive appears among the bindings.
    ComponentCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5DecisionFeedbackSharedConsumersViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ConsumerBindingsMissing => "consumer_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::VocabularyFacetIncomplete => "vocabulary_facet_incomplete",
            Self::DispositionWordOutsideVocabulary => "disposition_word_outside_vocabulary",
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::VocabularyDriftAcrossSurfaces => "vocabulary_drift_across_surfaces",
            Self::PrimitiveReuseUnproven => "primitive_reuse_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::NarrowNoteMissing => "narrow_note_missing",
            Self::NarrowReasonMismatch => "narrow_reason_mismatch",
            Self::NarrowNextActionMismatch => "narrow_next_action_mismatch",
            Self::NarrowNotePreservedVocabularyMissing => {
                "narrow_note_preserved_vocabulary_missing"
            }
            Self::NarrowNextActionLabelMissing => "narrow_next_action_label_missing",
            Self::UnexpectedNarrowNote => "unexpected_narrow_note",
            Self::RemoteSourceNoteMissing => "remote_source_note_missing",
            Self::ExportDetailNoteMissing => "export_detail_note_missing",
            Self::ColorAloneForMeaning => "color_alone_for_meaning",
            Self::PopoverCarriesOnlyCriticalInstruction => {
                "popover_carries_only_critical_instruction"
            }
            Self::GenericYesNoConfirmationCopy => "generic_yes_no_confirmation_copy",
            Self::DurableWorkAsToastOnlyTruth => "durable_work_as_toast_only_truth",
            Self::UsefulPaneBlankedDuringLoading => "useful_pane_blanked_during_loading",
            Self::FullScreenSpinnerWherePartialCapable => {
                "full_screen_spinner_where_partial_capable"
            }
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::ComponentCoverageMissing => "component_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable decision-feedback shared-consumer export.
pub fn current_stable_m5_decision_feedback_shared_consumers_export(
) -> Result<M5DecisionFeedbackSharedConsumersPacket, M5DecisionFeedbackSharedConsumersArtifactError>
{
    let packet: M5DecisionFeedbackSharedConsumersPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-decision-feedback-shared-consumers-proof/support_export.json"
        )
    ))
    .map_err(M5DecisionFeedbackSharedConsumersArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DecisionFeedbackSharedConsumersArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5DecisionFeedbackSharedConsumersPacket,
    violations: &mut Vec<M5DecisionFeedbackSharedConsumersViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_DECISION_FEEDBACK_SHARED_CONSUMERS_SCHEMA_REF,
        M5_DECISION_FEEDBACK_SHARED_CONSUMERS_DOC_REF,
        M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF,
        M5_DECISION_FEEDBACK_COMPONENT_DOC_REF,
    ];
    for family in M5DecisionFeedbackFamily::ALL {
        required.push(family.canonical_component_schema_ref());
    }
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5DecisionFeedbackSharedConsumersViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5DecisionFeedbackSharedConsumersPacket,
    violations: &mut Vec<M5DecisionFeedbackSharedConsumersViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(M5DecisionFeedbackSharedConsumersViolation::ConsumerBindingsMissing);
        return;
    }

    // One vocabulary: the facet values must be identical for every binding that renders
    // the same primitive object.
    let mut object_facets: BTreeMap<&str, &DecisionFeedbackStateFacetValues> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each primitive must be adopted by at least two distinct consumers.
    let mut component_consumers: BTreeMap<
        M5DecisionFeedbackFamily,
        BTreeSet<M5DecisionFeedbackConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5DecisionFeedbackConsumerSurface> = BTreeSet::new();
    let mut seen_components: BTreeSet<M5DecisionFeedbackFamily> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.primitive_object_id.trim().is_empty()
            || binding.primitive_object_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5DecisionFeedbackSharedConsumersViolation::BindingIncomplete);
        }
        if !binding.state_facets.all_present() {
            violations.push(M5DecisionFeedbackSharedConsumersViolation::VocabularyFacetIncomplete);
        }
        if !binding.state_facets.disposition_word_in_vocabulary() {
            violations
                .push(M5DecisionFeedbackSharedConsumersViolation::DispositionWordOutsideVocabulary);
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5DecisionFeedbackSharedConsumersViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations.push(M5DecisionFeedbackSharedConsumersViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations
                            .push(M5DecisionFeedbackSharedConsumersViolation::NarrowReasonMismatch);
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations.push(
                            M5DecisionFeedbackSharedConsumersViolation::NarrowNextActionMismatch,
                        );
                    }
                    if note.preserved_vocabulary_note.trim().is_empty() {
                        violations.push(
                            M5DecisionFeedbackSharedConsumersViolation::NarrowNotePreservedVocabularyMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5DecisionFeedbackSharedConsumersViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5DecisionFeedbackSharedConsumersViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_remote_source_note && binding.remote_source_note.trim().is_empty() {
            violations.push(M5DecisionFeedbackSharedConsumersViolation::RemoteSourceNoteMissing);
        }
        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations.push(M5DecisionFeedbackSharedConsumersViolation::ExportDetailNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.relies_on_color_alone_for_meaning {
            violations.push(M5DecisionFeedbackSharedConsumersViolation::ColorAloneForMeaning);
        }
        if binding.lets_a_popover_carry_the_only_critical_instruction {
            violations.push(
                M5DecisionFeedbackSharedConsumersViolation::PopoverCarriesOnlyCriticalInstruction,
            );
        }
        if binding.uses_generic_yes_no_confirmation_copy {
            violations
                .push(M5DecisionFeedbackSharedConsumersViolation::GenericYesNoConfirmationCopy);
        }
        if binding.represents_durable_work_as_toast_only_truth {
            violations
                .push(M5DecisionFeedbackSharedConsumersViolation::DurableWorkAsToastOnlyTruth);
        }
        if binding.blanks_a_useful_pane_during_loading {
            violations
                .push(M5DecisionFeedbackSharedConsumersViolation::UsefulPaneBlankedDuringLoading);
        }
        if binding.uses_a_full_screen_spinner_where_partial_capable {
            violations.push(
                M5DecisionFeedbackSharedConsumersViolation::FullScreenSpinnerWherePartialCapable,
            );
        }

        // Support / export consumers must map a primitive back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations
                .push(M5DecisionFeedbackSharedConsumersViolation::SupportExportReferenceMissing);
        }

        // Vocabulary-drift accumulation.
        match object_facets.get(binding.primitive_object_id.as_str()) {
            None => {
                object_facets.insert(binding.primitive_object_id.as_str(), &binding.state_facets);
            }
            Some(existing) => {
                if **existing != binding.state_facets && !drift_reported {
                    violations.push(
                        M5DecisionFeedbackSharedConsumersViolation::VocabularyDriftAcrossSurfaces,
                    );
                    drift_reported = true;
                }
            }
        }

        component_consumers
            .entry(binding.component)
            .or_default()
            .insert(binding.consumer);
        seen_consumers.insert(binding.consumer);
        seen_components.insert(binding.component);
    }

    // Coverage: every consumer surface and every primitive must appear.
    for consumer in M5DecisionFeedbackConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5DecisionFeedbackSharedConsumersViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for component in M5DecisionFeedbackFamily::ALL {
        if !seen_components.contains(&component) {
            violations.push(M5DecisionFeedbackSharedConsumersViolation::ComponentCoverageMissing);
            break;
        }
    }

    // Reuse: every present primitive must be adopted by two or more distinct consumers.
    for consumers in component_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5DecisionFeedbackSharedConsumersViolation::PrimitiveReuseUnproven);
            break;
        }
    }
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
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
