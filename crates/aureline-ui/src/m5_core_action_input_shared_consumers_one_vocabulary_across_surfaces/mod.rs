//! Shared settings / request / package-install / provider-account / template-starter /
//! admin-policy / repair / entry consumers that keep the B134 core action and input
//! controls — buttons, icon buttons, split buttons, text fields, search fields,
//! comboboxes, checkbox/radio/switch toggle controls, and segmented controls — at **one
//! vocabulary** across every claimed M5 surface.
//!
//! This module is the closing consumer-adoption lane for the eight reusable atomic
//! controls frozen in [`crate::m5_core_action_input_component_matrix`] and implemented by
//! the button / icon-button lane
//! ([`crate::m5_button_and_icon_button_state_and_command_attribution`]), the split-button
//! / segmented-control lane
//! ([`crate::m5_split_button_and_segmented_control_safe_default_and_selected_mode`]), the
//! text-field / search-field lane
//! ([`crate::m5_text_field_and_search_field_labels_validation_and_privacy`]), and the
//! combobox / toggle-control lane
//! ([`crate::m5_combobox_and_checkbox_radio_switch_value_source_and_toggle_semantics`]).
//!
//! It binds each shared control to the concrete forms, settings, search, entry, review,
//! repair, CLI/export, support, and product consumers that render it, and proves — by
//! fixtures, not screenshots — that the same control object presents the same state,
//! command binding, value source, validation, and lock/policy vocabulary wherever it
//! appears.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **Reuse.** Each of the eight shared controls must be adopted by at least two
//!    distinct consumers, so a control is proven to be shared product infrastructure
//!    rather than a one-surface feature-local fork.
//! 2. **One vocabulary / no drift.** For a given control object every consumer surface
//!    must present identical [`CoreControlStateFacetValues`] — the same state word, the
//!    same command-binding word, the same value-source word, the same validation word,
//!    and the same lock/policy word. The state word must be a token from the frozen
//!    [`M5CoreControlDisposition`] vocabulary, so no feature rewrites `default`,
//!    `loading`, `locked`, `read_only`, or `degraded` in its own words. A surface may
//!    narrow *how much* it shows across desktop, compact, remote, and exported
//!    representations, but it may never reword the underlying vocabulary per surface.
//! 3. **Map back to one family.** Support and CLI/export consumers must point at the
//!    canonical per-control schema and the frozen matrix by id, so an exported packet can
//!    always map a settings / request / provider / admin / repair / entry control back to
//!    one shared contract family.
//!
//! Narrowing is disclosed, never hidden: a compact, remote, or exported representation
//! carries an explicit [`CoreControlNarrowNote`] naming the reason, the preserved
//! vocabulary, and the next action, and an exported representation additionally names its
//! export-safe detail boundary rather than collapsing the object out of view.
//!
//! The packet references upstream control contracts by id rather than embedding their
//! content. Raw secret values, credentials, and private endpoints stay outside the
//! support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-core-action-input-shared-consumers.schema.json`](../../../../schemas/ui/m5-core-action-input-shared-consumers.schema.json).
//! The contract doc is
//! [`docs/components/m5_core_action_input_shared_consumers_one_vocabulary.md`](../../../../docs/components/m5_core_action_input_shared_consumers_one_vocabulary.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-core-action-input-shared-consumers/`](../../../../fixtures/ui/m5-core-action-input-shared-consumers/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_core_action_input_shared_consumers,
    seeded_m5_core_action_input_shared_consumers_compact_remote_narrowed,
    seeded_m5_core_action_input_shared_consumers_exported_redaction_narrowed,
};

use crate::m5_core_action_input_component_matrix::{
    M5CoreControlConsumerSurface, M5CoreControlDisposition, M5CoreControlFamily,
    M5_CORE_CONTROL_COMPONENT_DOC_REF, M5_CORE_CONTROL_COMPONENT_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5CoreControlSharedConsumersPacket`].
pub const M5_CORE_CONTROL_SHARED_CONSUMERS_RECORD_KIND: &str =
    "m5_core_action_input_shared_consumer_vocabulary_parity";

/// Schema version for core-control shared-consumer parity records.
pub const M5_CORE_CONTROL_SHARED_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_CORE_CONTROL_SHARED_CONSUMERS_PACKET_ID: &str =
    "m5-core-action-input-shared-consumers:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_CORE_CONTROL_SHARED_CONSUMERS_SCHEMA_REF: &str =
    "schemas/ui/m5-core-action-input-shared-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_CORE_CONTROL_SHARED_CONSUMERS_DOC_REF: &str =
    "docs/components/m5_core_action_input_shared_consumers_one_vocabulary.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CORE_CONTROL_SHARED_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/release/m5-core-action-input-shared-consumers-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_CORE_CONTROL_SHARED_CONSUMERS_CSV_REF: &str =
    "artifacts/release/m5-core-action-input-shared-consumers-proof/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_CORE_CONTROL_SHARED_CONSUMERS_REPORT_REF: &str =
    "artifacts/release/m5-core-action-input-shared-consumers-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_CORE_CONTROL_SHARED_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-core-action-input-shared-consumers";

/// Proof-freshness SLO in hours for this lane.
pub const M5_CORE_CONTROL_SHARED_CONSUMERS_PROOF_SLO_HOURS: u32 = 720;

/// Whether a consumer surface is an export / support path that must map a control back to
/// its canonical contract family by id.
pub const fn consumer_must_reference_canonical(consumer: M5CoreControlConsumerSurface) -> bool {
    matches!(
        consumer,
        M5CoreControlConsumerSurface::SupportExport | M5CoreControlConsumerSurface::CliExport
    )
}

/// Whether `token` is a member of the frozen [`M5CoreControlDisposition`] vocabulary.
///
/// This is the "one vocabulary" gate: a control object's state word must be a controlled
/// disposition token rather than a per-surface synonym.
pub fn is_known_disposition_token(token: &str) -> bool {
    M5CoreControlDisposition::ALL
        .iter()
        .any(|disposition| disposition.as_str() == token)
}

/// How much of a shared control a consumer renders for one representation.
///
/// Narrowing changes how much is shown, never the underlying vocabulary: a narrowed
/// representation still carries the same state, command-binding, value-source, validation,
/// and lock/policy words, and discloses the narrowing through an explicit note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoreControlRepresentation {
    /// The full desktop representation; nothing is narrowed.
    DesktopFull,
    /// A compact representation that narrows disclosure depth.
    CompactNarrowed,
    /// A remote-projected representation backed by a remote source.
    RemoteProjected,
    /// An exported, export-safe-redacted representation.
    ExportedRedacted,
}

impl CoreControlRepresentation {
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
pub enum CoreControlParityFacet {
    /// The control state / disposition word (a frozen disposition token).
    StateWord,
    /// The command / action id the control binds back to.
    CommandBindingWord,
    /// The source-of-value word.
    ValueSourceWord,
    /// The validation / constraint word.
    ValidationWord,
    /// The lock / policy word.
    LockPolicyWord,
}

impl CoreControlParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::StateWord,
        Self::CommandBindingWord,
        Self::ValueSourceWord,
        Self::ValidationWord,
        Self::LockPolicyWord,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateWord => "state_word",
            Self::CommandBindingWord => "command_binding_word",
            Self::ValueSourceWord => "value_source_word",
            Self::ValidationWord => "validation_word",
            Self::LockPolicyWord => "lock_policy_word",
        }
    }
}

/// Why a surface narrowed its rendering of a shared control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoreControlNarrowReason {
    /// A compact representation narrowed disclosure depth.
    CompactionNarrowed,
    /// A remote-projected representation narrowed to remote-backed truth.
    RemoteProjectionNarrowed,
    /// An exported representation narrowed to export-safe-redacted truth.
    ExportRedactionNarrowed,
}

impl CoreControlNarrowReason {
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
pub enum CoreControlNarrowNextAction {
    /// Expand the control in the full desktop representation.
    ExpandInDesktop,
    /// Open the remote source backing the projection.
    OpenRemoteSource,
    /// Open the full detail behind the redacted export.
    OpenFullDetail,
}

impl CoreControlNarrowNextAction {
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
pub enum CoreControlParityState {
    /// All vocabulary is preserved and shown in full.
    FacetsPreserved,
    /// All vocabulary is preserved and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl CoreControlParityState {
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
pub enum CoreControlSharedConsumersDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Control vocabulary drifted between surfaces for the same object.
    VocabularyDriftDetected,
    /// Placeholder text was used as the only label.
    PlaceholderUsedAsLabel,
    /// A loading control relabeled the action or lost attribution.
    LoadingRelabeledOrResized,
    /// An icon-only destructive action was left unlabeled.
    IconOnlyDestructiveUnlabeled,
    /// A switch was blurred with a deferred checkbox.
    SwitchAndDeferredCheckboxBlurred,
    /// A split button defaulted to a riskier alternate.
    SplitDefaultedToRiskierAlternate,
    /// Locked or degraded semantics were hidden behind generic disabled chrome.
    LockedOrDegradedHiddenBehindDisabled,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalReferenceMissing,
    /// An upstream shared control narrowed.
    UpstreamControlNarrowed,
}

impl CoreControlSharedConsumersDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::VocabularyDriftDetected,
        Self::PlaceholderUsedAsLabel,
        Self::LoadingRelabeledOrResized,
        Self::IconOnlyDestructiveUnlabeled,
        Self::SwitchAndDeferredCheckboxBlurred,
        Self::SplitDefaultedToRiskierAlternate,
        Self::LockedOrDegradedHiddenBehindDisabled,
        Self::CanonicalReferenceMissing,
        Self::UpstreamControlNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::VocabularyDriftDetected => "vocabulary_drift_detected",
            Self::PlaceholderUsedAsLabel => "placeholder_used_as_label",
            Self::LoadingRelabeledOrResized => "loading_relabeled_or_resized",
            Self::IconOnlyDestructiveUnlabeled => "icon_only_destructive_unlabeled",
            Self::SwitchAndDeferredCheckboxBlurred => "switch_and_deferred_checkbox_blurred",
            Self::SplitDefaultedToRiskierAlternate => "split_defaulted_to_riskier_alternate",
            Self::LockedOrDegradedHiddenBehindDisabled => {
                "locked_or_degraded_hidden_behind_disabled"
            }
            Self::CanonicalReferenceMissing => "canonical_reference_missing",
            Self::UpstreamControlNarrowed => "upstream_control_narrowed",
        }
    }
}

/// The controlled vocabulary a control object presents.
///
/// These five words must be identical across every consumer surface that shows the same
/// control object. The state word must be a frozen disposition token; the rest are
/// controlled words the object's family carries. A surface may narrow how much it renders,
/// but it may never reword any of these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreControlStateFacetValues {
    /// Control state / disposition word (must be a frozen disposition token).
    pub state_word: String,
    /// Command / action id the control binds back to.
    pub command_binding_word: String,
    /// Source-of-value word.
    pub value_source_word: String,
    /// Validation / constraint word.
    pub validation_word: String,
    /// Lock / policy word.
    pub lock_policy_word: String,
}

impl CoreControlStateFacetValues {
    /// Whether every vocabulary word is present.
    pub fn all_present(&self) -> bool {
        !self.state_word.trim().is_empty()
            && !self.command_binding_word.trim().is_empty()
            && !self.value_source_word.trim().is_empty()
            && !self.validation_word.trim().is_empty()
            && !self.lock_policy_word.trim().is_empty()
    }

    /// Whether the state word is a member of the frozen disposition vocabulary.
    pub fn state_word_in_vocabulary(&self) -> bool {
        is_known_disposition_token(self.state_word.trim())
    }
}

/// The explicit note a narrowed representation shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreControlNarrowNote {
    /// Why the representation narrowed.
    pub reason: CoreControlNarrowReason,
    /// Note naming the preserved vocabulary (never omitted).
    pub preserved_vocabulary_note: String,
    /// The next action offered.
    pub next_action: CoreControlNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoreControlRenderDisclosure {
    /// The parity state the representation requires.
    pub parity_state: CoreControlParityState,
    /// The narrow reason the representation requires, if any.
    pub narrow_reason: Option<CoreControlNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<CoreControlNarrowNextAction>,
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
pub const fn resolve_core_control_render_disclosure(
    representation: CoreControlRepresentation,
) -> CoreControlRenderDisclosure {
    match representation {
        CoreControlRepresentation::DesktopFull => CoreControlRenderDisclosure {
            parity_state: CoreControlParityState::FacetsPreserved,
            narrow_reason: None,
            narrow_next_action: None,
            needs_narrow_note: false,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        CoreControlRepresentation::CompactNarrowed => CoreControlRenderDisclosure {
            parity_state: CoreControlParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(CoreControlNarrowReason::CompactionNarrowed),
            narrow_next_action: Some(CoreControlNarrowNextAction::ExpandInDesktop),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        CoreControlRepresentation::RemoteProjected => CoreControlRenderDisclosure {
            parity_state: CoreControlParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(CoreControlNarrowReason::RemoteProjectionNarrowed),
            narrow_next_action: Some(CoreControlNarrowNextAction::OpenRemoteSource),
            needs_narrow_note: true,
            needs_remote_source_note: true,
            needs_export_detail_note: false,
        },
        CoreControlRepresentation::ExportedRedacted => CoreControlRenderDisclosure {
            parity_state: CoreControlParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(CoreControlNarrowReason::ExportRedactionNarrowed),
            narrow_next_action: Some(CoreControlNarrowNextAction::OpenFullDetail),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: true,
        },
    }
}

/// One consumer binding: a shared control rendered on one consumer surface in one
/// representation for one control object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreControlConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable control-object id (shared across surfaces that show the same object).
    pub control_object_id: String,
    /// Human-readable control-object identity.
    pub control_object_label: String,
    /// Which shared control this binding renders.
    pub component: M5CoreControlFamily,
    /// Which consumer surface renders it.
    pub consumer: M5CoreControlConsumerSurface,
    /// Which representation this surface renders.
    pub representation: CoreControlRepresentation,
    /// The controlled vocabulary presented (identical across surfaces for one object).
    pub state_facets: CoreControlStateFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub parity_state: CoreControlParityState,
    /// The explicit narrow note; required and complete when the binding narrows.
    pub narrow_note: Option<CoreControlNarrowNote>,
    /// Remote-source note; required and non-empty when the disclosure demands it.
    pub remote_source_note: String,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Guardrail: this surface lets placeholder text replace the label. MUST be `false`.
    pub lets_placeholder_text_replace_the_label: bool,
    /// Guardrail: this surface lets a loading control relabel the action or lose
    /// attribution. MUST be `false`.
    pub lets_loading_relabel_the_action_or_lose_attribution: bool,
    /// Guardrail: this surface leaves an icon-only destructive action unlabeled. MUST be
    /// `false`.
    pub leaves_icon_only_destructive_action_unlabeled: bool,
    /// Guardrail: this surface blurs a switch with a deferred checkbox. MUST be `false`.
    pub blurs_switch_with_deferred_checkbox: bool,
    /// Guardrail: this surface lets a split button default to a riskier alternate. MUST be
    /// `false`.
    pub lets_split_button_default_to_riskier_alternate: bool,
    /// Guardrail: this surface hides locked or degraded semantics behind generic disabled
    /// chrome. MUST be `false`.
    pub hides_locked_or_degraded_semantics_behind_generic_disabled: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl CoreControlConsumerBinding {
    /// Disclosures this binding must carry, derived from its representation.
    pub const fn disclosure(&self) -> CoreControlRenderDisclosure {
        resolve_core_control_render_disclosure(self.representation)
    }

    /// Whether this binding renders below full parity.
    pub const fn is_narrowed(&self) -> bool {
        self.representation.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.lets_placeholder_text_replace_the_label
            && !self.lets_loading_relabel_the_action_or_lose_attribution
            && !self.leaves_icon_only_destructive_action_unlabeled
            && !self.blurs_switch_with_deferred_checkbox
            && !self.lets_split_button_default_to_riskier_alternate
            && !self.hides_locked_or_degraded_semantics_behind_generic_disabled
    }

    /// Whether this binding points at the canonical control schema and the matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let component_ref = self.component.canonical_component_schema_ref();
        self.source_contract_refs
            .iter()
            .any(|reference| reference == component_ref)
            && self
                .source_contract_refs
                .iter()
                .any(|reference| reference == M5_CORE_CONTROL_COMPONENT_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreControlSharedConsumersTrustReview {
    /// Control reuse is proven by fixtures rather than inferred from screenshots.
    pub control_reuse_proven_by_fixtures: bool,
    /// The same control object presents the same vocabulary across surfaces.
    pub same_object_same_vocabulary_across_surfaces: bool,
    /// Every state word is a frozen disposition token.
    pub state_words_stay_in_frozen_vocabulary: bool,
    /// Placeholder text never replaces a permanent label.
    pub placeholder_never_replaces_label: bool,
    /// Loading never relabels the action or loses attribution.
    pub loading_never_relabels_or_loses_attribution: bool,
    /// Icon-only destructive actions are never left unlabeled.
    pub icon_destructive_never_unlabeled: bool,
    /// Switches are never blurred with deferred checkboxes.
    pub switch_never_blurred_with_deferred_checkbox: bool,
    /// Split buttons never default to a riskier alternate.
    pub split_never_defaults_to_riskier_alternate: bool,
    /// Locked and degraded semantics stay distinct from generic disabled chrome.
    pub locked_and_degraded_stay_distinct_from_disabled: bool,
    /// Narrowing is disclosed across desktop, compact, remote, and exported forms.
    pub narrowing_disclosed_across_representations: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl CoreControlSharedConsumersTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.control_reuse_proven_by_fixtures
            && self.same_object_same_vocabulary_across_surfaces
            && self.state_words_stay_in_frozen_vocabulary
            && self.placeholder_never_replaces_label
            && self.loading_never_relabels_or_loses_attribution
            && self.icon_destructive_never_unlabeled
            && self.switch_never_blurred_with_deferred_checkbox
            && self.split_never_defaults_to_riskier_alternate
            && self.locked_and_degraded_stay_distinct_from_disabled
            && self.narrowing_disclosed_across_representations
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreControlSharedConsumersProjection {
    /// The forms UI reuses the shared controls.
    pub forms_ui_reuses_shared_controls: bool,
    /// The settings UI reuses the shared controls.
    pub settings_ui_reuses_shared_controls: bool,
    /// The search UI reuses the shared controls.
    pub search_ui_reuses_shared_controls: bool,
    /// The start-center entry UI reuses the shared controls.
    pub entry_ui_reuses_shared_controls: bool,
    /// The review UI reuses the shared controls.
    pub review_ui_reuses_shared_controls: bool,
    /// The repair UI reuses the shared controls.
    pub repair_ui_reuses_shared_controls: bool,
    /// The support / export path reuses the shared controls.
    pub support_export_reuses_shared_controls: bool,
    /// Every control is adopted by two or more consumers.
    pub every_control_adopted_by_two_or_more_consumers: bool,
    /// Vocabulary is identical for the same control object.
    pub vocabulary_identical_for_same_object: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps a control back to one shared contract family.
    pub export_maps_back_to_one_contract_family: bool,
}

impl CoreControlSharedConsumersProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.forms_ui_reuses_shared_controls
            && self.settings_ui_reuses_shared_controls
            && self.search_ui_reuses_shared_controls
            && self.entry_ui_reuses_shared_controls
            && self.review_ui_reuses_shared_controls
            && self.repair_ui_reuses_shared_controls
            && self.support_export_reuses_shared_controls
            && self.every_control_adopted_by_two_or_more_consumers
            && self.vocabulary_identical_for_same_object
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_contract_family
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreControlSharedConsumersProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5CoreControlSharedConsumersPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CoreControlSharedConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<CoreControlConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<CoreControlSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5CoreControlConsumerSurface>,
    /// Trust review block.
    pub trust_review: CoreControlSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: CoreControlSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: CoreControlSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe core-control shared-consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoreControlSharedConsumersPacket {
    /// Record kind; must equal [`M5_CORE_CONTROL_SHARED_CONSUMERS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CORE_CONTROL_SHARED_CONSUMERS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<CoreControlConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<CoreControlSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5CoreControlConsumerSurface>,
    /// Trust review block.
    pub trust_review: CoreControlSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: CoreControlSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: CoreControlSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CoreControlSharedConsumersPacket {
    /// Builds a core-control shared-consumer packet from stable-lane input.
    pub fn new(input: M5CoreControlSharedConsumersPacketInput) -> Self {
        Self {
            record_kind: M5_CORE_CONTROL_SHARED_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: M5_CORE_CONTROL_SHARED_CONSUMERS_SCHEMA_VERSION,
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

    /// Validates the core-control shared-consumer parity invariants.
    pub fn validate(&self) -> Vec<M5CoreControlSharedConsumersViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_CORE_CONTROL_SHARED_CONSUMERS_RECORD_KIND {
            violations.push(M5CoreControlSharedConsumersViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CORE_CONTROL_SHARED_CONSUMERS_SCHEMA_VERSION {
            violations.push(M5CoreControlSharedConsumersViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CoreControlSharedConsumersViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5CoreControlSharedConsumersViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5CoreControlSharedConsumersViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5CoreControlSharedConsumersViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(M5CoreControlSharedConsumersViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5CoreControlSharedConsumersViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("core-control shared-consumer packet serializes"),
        ) {
            violations.push(M5CoreControlSharedConsumersViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("core-control shared-consumer packet serializes")
    }

    /// Deterministic matrix CSV, one row per consumer binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from("component,consumer,representation,state_word,parity_state\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                binding.component.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.state_word,
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
            "# Shared Core Action / Input Control Consumers: One Vocabulary Across Surfaces\n\n",
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
                "- **{}** [`{}`]: control `{}` on `{}`, representation `{}`, state `{}`\n",
                binding.control_object_label,
                binding.binding_id,
                binding.component.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.state_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in core-control shared-consumer export.
#[derive(Debug)]
pub enum M5CoreControlSharedConsumersArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CoreControlSharedConsumersViolation>),
}

impl fmt::Display for M5CoreControlSharedConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "core-control shared-consumer export parse failed: {error}"
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
                    "core-control shared-consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CoreControlSharedConsumersArtifactError {}

/// Validation failures emitted by [`M5CoreControlSharedConsumersPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CoreControlSharedConsumersViolation {
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
    /// A binding's state word is not a frozen disposition token.
    StateWordOutsideVocabulary,
    /// A binding's parity state does not match its representation.
    ParityStateMismatch,
    /// Two surfaces show the same control object with different vocabulary.
    VocabularyDriftAcrossSurfaces,
    /// A shared control is not adopted by at least two distinct consumers.
    ControlReuseUnproven,
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
    /// A binding lets placeholder text replace the label.
    PlaceholderReplacesLabel,
    /// A binding lets a loading control relabel the action or lose attribution.
    LoadingRelabelsOrLosesAttribution,
    /// A binding leaves an icon-only destructive action unlabeled.
    IconDestructiveUnlabeled,
    /// A binding blurs a switch with a deferred checkbox.
    SwitchBlurredWithDeferredCheckbox,
    /// A binding lets a split button default to a riskier alternate.
    SplitDefaultsToRiskierAlternate,
    /// A binding hides locked or degraded semantics behind generic disabled chrome.
    LockedOrDegradedHiddenBehindDisabled,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared control appears among the bindings.
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

impl M5CoreControlSharedConsumersViolation {
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
            Self::StateWordOutsideVocabulary => "state_word_outside_vocabulary",
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::VocabularyDriftAcrossSurfaces => "vocabulary_drift_across_surfaces",
            Self::ControlReuseUnproven => "control_reuse_unproven",
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
            Self::PlaceholderReplacesLabel => "placeholder_replaces_label",
            Self::LoadingRelabelsOrLosesAttribution => "loading_relabels_or_loses_attribution",
            Self::IconDestructiveUnlabeled => "icon_destructive_unlabeled",
            Self::SwitchBlurredWithDeferredCheckbox => "switch_blurred_with_deferred_checkbox",
            Self::SplitDefaultsToRiskierAlternate => "split_defaults_to_riskier_alternate",
            Self::LockedOrDegradedHiddenBehindDisabled => {
                "locked_or_degraded_hidden_behind_disabled"
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

/// Reads and validates the checked-in stable core-control shared-consumer export.
pub fn current_stable_m5_core_control_shared_consumers_export(
) -> Result<M5CoreControlSharedConsumersPacket, M5CoreControlSharedConsumersArtifactError> {
    let packet: M5CoreControlSharedConsumersPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-core-action-input-shared-consumers-proof/support_export.json"
    )))
    .map_err(M5CoreControlSharedConsumersArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CoreControlSharedConsumersArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5CoreControlSharedConsumersPacket,
    violations: &mut Vec<M5CoreControlSharedConsumersViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_CORE_CONTROL_SHARED_CONSUMERS_SCHEMA_REF,
        M5_CORE_CONTROL_SHARED_CONSUMERS_DOC_REF,
        M5_CORE_CONTROL_COMPONENT_SCHEMA_REF,
        M5_CORE_CONTROL_COMPONENT_DOC_REF,
    ];
    for family in M5CoreControlFamily::ALL {
        required.push(family.canonical_component_schema_ref());
    }
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5CoreControlSharedConsumersViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5CoreControlSharedConsumersPacket,
    violations: &mut Vec<M5CoreControlSharedConsumersViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(M5CoreControlSharedConsumersViolation::ConsumerBindingsMissing);
        return;
    }

    // One vocabulary: the facet values must be identical for every binding that renders
    // the same control object.
    let mut object_facets: BTreeMap<&str, &CoreControlStateFacetValues> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each control must be adopted by at least two distinct consumers.
    let mut component_consumers: BTreeMap<
        M5CoreControlFamily,
        BTreeSet<M5CoreControlConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5CoreControlConsumerSurface> = BTreeSet::new();
    let mut seen_components: BTreeSet<M5CoreControlFamily> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.control_object_id.trim().is_empty()
            || binding.control_object_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5CoreControlSharedConsumersViolation::BindingIncomplete);
        }
        if !binding.state_facets.all_present() {
            violations.push(M5CoreControlSharedConsumersViolation::VocabularyFacetIncomplete);
        }
        if !binding.state_facets.state_word_in_vocabulary() {
            violations.push(M5CoreControlSharedConsumersViolation::StateWordOutsideVocabulary);
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5CoreControlSharedConsumersViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations.push(M5CoreControlSharedConsumersViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations
                            .push(M5CoreControlSharedConsumersViolation::NarrowReasonMismatch);
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations
                            .push(M5CoreControlSharedConsumersViolation::NarrowNextActionMismatch);
                    }
                    if note.preserved_vocabulary_note.trim().is_empty() {
                        violations.push(
                            M5CoreControlSharedConsumersViolation::NarrowNotePreservedVocabularyMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5CoreControlSharedConsumersViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5CoreControlSharedConsumersViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_remote_source_note && binding.remote_source_note.trim().is_empty() {
            violations.push(M5CoreControlSharedConsumersViolation::RemoteSourceNoteMissing);
        }
        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations.push(M5CoreControlSharedConsumersViolation::ExportDetailNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.lets_placeholder_text_replace_the_label {
            violations.push(M5CoreControlSharedConsumersViolation::PlaceholderReplacesLabel);
        }
        if binding.lets_loading_relabel_the_action_or_lose_attribution {
            violations
                .push(M5CoreControlSharedConsumersViolation::LoadingRelabelsOrLosesAttribution);
        }
        if binding.leaves_icon_only_destructive_action_unlabeled {
            violations.push(M5CoreControlSharedConsumersViolation::IconDestructiveUnlabeled);
        }
        if binding.blurs_switch_with_deferred_checkbox {
            violations
                .push(M5CoreControlSharedConsumersViolation::SwitchBlurredWithDeferredCheckbox);
        }
        if binding.lets_split_button_default_to_riskier_alternate {
            violations.push(M5CoreControlSharedConsumersViolation::SplitDefaultsToRiskierAlternate);
        }
        if binding.hides_locked_or_degraded_semantics_behind_generic_disabled {
            violations
                .push(M5CoreControlSharedConsumersViolation::LockedOrDegradedHiddenBehindDisabled);
        }

        // Support / export consumers must map a control back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations.push(M5CoreControlSharedConsumersViolation::SupportExportReferenceMissing);
        }

        // Vocabulary-drift accumulation.
        match object_facets.get(binding.control_object_id.as_str()) {
            None => {
                object_facets.insert(binding.control_object_id.as_str(), &binding.state_facets);
            }
            Some(existing) => {
                if **existing != binding.state_facets && !drift_reported {
                    violations
                        .push(M5CoreControlSharedConsumersViolation::VocabularyDriftAcrossSurfaces);
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

    // Coverage: every consumer surface and every control must appear.
    for consumer in M5CoreControlConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5CoreControlSharedConsumersViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for component in M5CoreControlFamily::ALL {
        if !seen_components.contains(&component) {
            violations.push(M5CoreControlSharedConsumersViolation::ComponentCoverageMissing);
            break;
        }
    }

    // Reuse: every present control must be adopted by two or more distinct consumers.
    for consumers in component_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5CoreControlSharedConsumersViolation::ControlReuseUnproven);
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
