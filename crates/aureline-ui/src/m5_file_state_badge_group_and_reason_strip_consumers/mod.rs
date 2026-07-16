//! Shared file-state badge groups and reason strips that render the six B150 constrained-current-object
//! classes across the tab-chrome, breadcrumb-trail, status-bar, command-palette, editor-banner,
//! diff / review-header, write-review-sheet, AI / automation-path, and support / export consumers at **one
//! controlled state-class vocabulary and safe-action rule set**.
//!
//! This module is the B150 badge-group / reason-strip consumer lane over the six constrained-current-object
//! classes frozen in [`crate::m5_constrained_file_state_matrix`] and made machine-readable by the
//! constrained-state-descriptor implement lane
//! ([`crate::m5_constrained_state_descriptor_and_change_diff_registries`]). Where those lanes describe *what*
//! is constrained, this lane proves *how it is shown*: every surface that can surface a constrained object
//! frames it with the same file-state badge group and reason strip — a controlled state-class label
//! (`Read-only`, `Generated`, `Policy locked`, `Managed`, `Projection`, `Captured snapshot`), a plain-language
//! cause, the canonical source it relates back to, the write disposition that makes it mechanically distinct
//! from a directly-writable object, and the nearest safe next step — before a user tries to write to the wrong
//! thing.
//!
//! It binds each constrained-object profile to the concrete consumer surfaces that render it and proves — by
//! fixtures, not screenshots — that the same profile presents the same badge-role, state-class-label,
//! reason, canonical-source, write-disposition, and safe-next-step grammar wherever it appears, that a
//! multi-state object (`Generated` plus `Policy locked`, `Managed` plus `Captured snapshot`) keeps every
//! co-applicable state visible instead of letting one badge hide another, and that the write-capable
//! safe-next-step affordance only appears where the full badge group is rendered.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **One vocabulary / no drift.** For a given constrained-object profile every consumer surface — an editor
//!    banner, a diff / review header, a command-palette result, and a status-bar chip among them — must
//!    present identical [`FileStateBadgeGroupGrammar`]: the same badge-role word, the same state-class-label
//!    word, the same reason word, the same canonical-source word, the same write-disposition word, the same
//!    safe-next-step word, and the same co-applicable state labels. The badge-role word must be a token from
//!    the frozen [`M5ConstrainedFileStateRole`] vocabulary, so no surface rewrites `state_badge_classification`,
//!    `blocked_write_reason`, `canonical_source_relation`, or `exact_write_target` in its own words.
//! 2. **Constrained, never directly-writable-by-omission.** A badge group exposes inspect-state, copy-reason,
//!    and reveal-canonical-source actions, and a write-capable open-safe-next-step action *only* where the full
//!    badge group is rendered (the [`BadgeRenderPosture::FullBadgeGroup`] posture); a silent lossy direct write
//!    is disabled by construction (no direct-write action can even be represented). No binding may present a
//!    constrained object as directly writable or hide its recovery / regenerate path, let a generated, managed,
//!    projection, or archived object silently fall back to a lossy direct write, give an AI / automation /
//!    import / repair flow a hidden bypass around the constrained-state rules, leave the canonical source,
//!    exact write target, preserved-versus-lost sync, or recovery / regenerate path unstated, or let one state
//!    class hide another when both materially affect behavior.
//! 3. **Screen-reader and keyboard discoverable.** Every binding names the accessibility routes
//!    ([`M5ConstrainedFileStateAccessibilityRoute`]) through which the current state class, its reason, and its
//!    next safe action can be discovered without pointer-only chrome; keyboard focus and screen-reader
//!    announcement are mandatory.
//!
//! Narrowing is disclosed, never hidden: a compacted status chip, a gated command-palette availability, or an
//! exported, export-safe view carries an explicit [`BadgeGroupNarrowNote`] naming the reason, the preserved
//! grammar, and the next action, so a surface may narrow *which* actions remain without ever rewording the
//! underlying badge grammar or quietly implying the object is directly writable.
//!
//! The packet references upstream constrained-file-state contracts by id rather than embedding their content.
//! Raw secret values, credentials, and private endpoints stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/program/m5-file-state-badge-group-consumers.schema.json`](../../../../schemas/program/m5-file-state-badge-group-consumers.schema.json).
//! The contract doc is
//! [`docs/support/m5_file_state_badge_group_consumers.md`](../../../../docs/support/m5_file_state_badge_group_consumers.md).
//! The protected fixture directory is
//! [`fixtures/editor/m5-file-state-badge-group-consumers/`](../../../../fixtures/editor/m5-file-state-badge-group-consumers/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_file_state_badge_group_consumers,
    seeded_m5_file_state_badge_group_consumers_compact_status_narrowed,
    seeded_m5_file_state_badge_group_consumers_palette_gated_narrowed,
};

use crate::m5_constrained_file_state_matrix::{
    M5ConstrainedFileStateAccessibilityRoute, M5ConstrainedFileStateConsumerSurface,
    M5ConstrainedFileStateObject, M5ConstrainedFileStateRole,
    M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF, M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5FileStateBadgeGroupConsumersPacket`].
pub const M5_FILE_STATE_BADGE_GROUP_CONSUMERS_RECORD_KIND: &str =
    "m5_file_state_badge_group_and_reason_strip_consumer_registry";

/// Schema version for file-state badge-group consumer records.
pub const M5_FILE_STATE_BADGE_GROUP_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_FILE_STATE_BADGE_GROUP_CONSUMERS_PACKET_ID: &str =
    "m5-file-state-badge-group-consumers:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_FILE_STATE_BADGE_GROUP_CONSUMERS_SCHEMA_REF: &str =
    "schemas/program/m5-file-state-badge-group-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_FILE_STATE_BADGE_GROUP_CONSUMERS_DOC_REF: &str =
    "docs/support/m5_file_state_badge_group_consumers.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_FILE_STATE_BADGE_GROUP_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/support/m5-file-state-badge-group-consumers/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_FILE_STATE_BADGE_GROUP_CONSUMERS_CSV_REF: &str =
    "artifacts/support/m5-file-state-badge-group-consumers/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_FILE_STATE_BADGE_GROUP_CONSUMERS_REPORT_REF: &str =
    "artifacts/support/m5-file-state-badge-group-consumers/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_FILE_STATE_BADGE_GROUP_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/editor/m5-file-state-badge-group-consumers";

/// Proof-freshness SLO in hours for this lane.
pub const M5_FILE_STATE_BADGE_GROUP_CONSUMERS_PROOF_SLO_HOURS: u32 = 720;

/// Write-disposition sentinel words a constrained-object badge may never fall back to; a badge whose role must
/// be present before surfacing as a constrained object must always keep a real write-constrained disposition
/// rather than implying the object is directly writable, editable, or unconstrained.
const WRITE_DISPOSITION_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "directly_writable",
    "writable",
    "editable",
    "unconstrained",
];

/// Whether a consumer surface is an export / support path that must map an object class back to its
/// canonical contract by id.
pub const fn consumer_must_reference_canonical(
    consumer: M5ConstrainedFileStateConsumerSurface,
) -> bool {
    matches!(
        consumer,
        M5ConstrainedFileStateConsumerSurface::SupportExportPacket
    )
}

/// Whether `token` is a member of the frozen [`M5ConstrainedFileStateRole`] vocabulary.
///
/// This is the "one vocabulary" gate: a badge's role word must be a controlled role token rather than a
/// per-surface synonym.
pub fn is_known_constrained_file_state_role_token(token: &str) -> bool {
    constrained_file_state_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5ConstrainedFileStateRole`], if it is one.
pub fn constrained_file_state_role_from_token(token: &str) -> Option<M5ConstrainedFileStateRole> {
    M5ConstrainedFileStateRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// The render posture a file-state badge group takes on one binding.
///
/// The posture governs the discoverable action set and narrowing disclosure, never the badge grammar: a
/// narrowed posture still carries the same badge-role, state-class-label, reason, canonical-source,
/// write-disposition, and safe-next-step words, and discloses the narrowing through an explicit note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeRenderPosture {
    /// The full badge group and reason strip are rendered; the surface offers a validated
    /// open-safe-next-step (duplicate / detach / overlay / regenerate / request-approval) review action.
    FullBadgeGroup,
    /// A compact status chip (tab chrome, status bar, or breadcrumb) renders the badge and reason narrowed to
    /// a chip, disclosed through a note, with no write-capable safe-next-step action.
    CompactStatusChip,
    /// A command-palette result marks the write action's availability as gated, disclosed through a note, with
    /// no write-capable safe-next-step action.
    PaletteAvailabilityGated,
    /// An exported, export-safe-redacted badge-group view.
    ExportRedacted,
}

impl BadgeRenderPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullBadgeGroup,
        Self::CompactStatusChip,
        Self::PaletteAvailabilityGated,
        Self::ExportRedacted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullBadgeGroup => "full_badge_group",
            Self::CompactStatusChip => "compact_status_chip",
            Self::PaletteAvailabilityGated => "palette_availability_gated",
            Self::ExportRedacted => "export_redacted",
        }
    }

    /// Whether this posture narrows below the full badge-group disclosure.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullBadgeGroup)
    }
}

/// A discoverable action a file-state badge group may expose.
///
/// The set is deliberately closed and safe: there is no direct-write action variant, so a badge group can
/// never present a write-capable control that performs a silent lossy direct write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeGroupAction {
    /// Inspect the constrained object's state metadata.
    InspectState,
    /// Copy the plain-language reason strip.
    CopyReason,
    /// Reveal the canonical source or backing object the constrained object relates back to.
    RevealCanonicalSource,
    /// Open the safe next step (duplicate / detach / overlay / regenerate / request-approval) review — only
    /// where the full badge group is rendered.
    OpenSafeNextStep,
}

impl BadgeGroupAction {
    /// The safe base action set present on every badge-group binding.
    pub const SAFE_BASE: [Self; 3] = [
        Self::InspectState,
        Self::CopyReason,
        Self::RevealCanonicalSource,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectState => "inspect_state",
            Self::CopyReason => "copy_reason",
            Self::RevealCanonicalSource => "reveal_canonical_source",
            Self::OpenSafeNextStep => "open_safe_next_step",
        }
    }
}

/// Why a file-state badge group narrowed its action set below a full badge-group view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeGroupNarrowReason {
    /// The badge and reason were compacted to a status chip; only inspect / copy / reveal remain.
    CompactedToStatusChip,
    /// The command-palette availability for the write action is gated behind the safe-next-step review.
    PaletteAvailabilityGatedDisclosed,
    /// An exported view redacted its surrounding detail export-safe.
    ExportRedactionNarrowed,
}

impl BadgeGroupNarrowReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompactedToStatusChip => "compacted_to_status_chip",
            Self::PaletteAvailabilityGatedDisclosed => "palette_availability_gated_disclosed",
            Self::ExportRedactionNarrowed => "export_redaction_narrowed",
        }
    }
}

/// The next action a narrow note offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeGroupNarrowNextAction {
    /// Open the full badge group behind the compact status chip.
    OpenFullBadgeGroup,
    /// Open the command detail explaining the gated write availability.
    OpenCommandDetail,
    /// Open the full detail behind the redacted export.
    OpenFullDetail,
}

impl BadgeGroupNarrowNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenFullBadgeGroup => "open_full_badge_group",
            Self::OpenCommandDetail => "open_command_detail",
            Self::OpenFullDetail => "open_full_detail",
        }
    }
}

/// Whether a binding preserves the full badge-group view or discloses a narrowed posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeGroupParityState {
    /// The badge grammar and full action set are preserved and shown.
    FacetsPreserved,
    /// The badge grammar is preserved and a narrowed action set is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl BadgeGroupParityState {
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
pub enum FileStateBadgeGroupConsumersDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Badge grammar drifted between surfaces for the same profile.
    BadgeGroupVocabularyDriftDetected,
    /// A badge dropped its write-constrained disposition and began to imply the object is directly writable.
    WriteDispositionDroppedForConstrainedObject,
    /// A surface presented a constrained object as directly writable or hid its recovery / regenerate path.
    PresentsConstrainedObjectAsDirectlyWritableOrHidesRecoveryPath,
    /// A surface let a generated / managed / projection / archived object silently fall back to a lossy direct
    /// write.
    LetsGeneratedManagedProjectionOrArchivedObjectsSilentlyFallBackToLossyDirectWrite,
    /// A surface gave an AI / automation / import / repair flow a hidden bypass around the constrained-state
    /// rules.
    GivesAiAutomationImportOrRepairFlowsAHiddenBypass,
    /// A surface left the canonical source, exact write target, preserved-versus-lost sync, or recovery /
    /// regenerate path unstated.
    LeavesCanonicalSourceExactWriteTargetSyncOrRecoveryPathUnstated,
    /// A surface let one state class hide another when both materially affect behavior.
    LetsOneStateClassHideAnotherWhenBothMateriallyAffectBehavior,
    /// An accessibility route for the state class, reason, or next safe action was dropped.
    AccessibilityRouteDropped,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream constrained-file-state contract narrowed.
    UpstreamConstrainedFileStateNarrowed,
}

impl FileStateBadgeGroupConsumersDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::BadgeGroupVocabularyDriftDetected,
        Self::WriteDispositionDroppedForConstrainedObject,
        Self::PresentsConstrainedObjectAsDirectlyWritableOrHidesRecoveryPath,
        Self::LetsGeneratedManagedProjectionOrArchivedObjectsSilentlyFallBackToLossyDirectWrite,
        Self::GivesAiAutomationImportOrRepairFlowsAHiddenBypass,
        Self::LeavesCanonicalSourceExactWriteTargetSyncOrRecoveryPathUnstated,
        Self::LetsOneStateClassHideAnotherWhenBothMateriallyAffectBehavior,
        Self::AccessibilityRouteDropped,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamConstrainedFileStateNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::BadgeGroupVocabularyDriftDetected => "badge_group_vocabulary_drift_detected",
            Self::WriteDispositionDroppedForConstrainedObject => {
                "write_disposition_dropped_for_constrained_object"
            }
            Self::PresentsConstrainedObjectAsDirectlyWritableOrHidesRecoveryPath => {
                "presents_constrained_object_as_directly_writable_or_hides_recovery_path"
            }
            Self::LetsGeneratedManagedProjectionOrArchivedObjectsSilentlyFallBackToLossyDirectWrite => {
                "lets_generated_managed_projection_or_archived_objects_silently_fall_back_to_lossy_direct_write"
            }
            Self::GivesAiAutomationImportOrRepairFlowsAHiddenBypass => {
                "gives_ai_automation_import_or_repair_flows_a_hidden_bypass"
            }
            Self::LeavesCanonicalSourceExactWriteTargetSyncOrRecoveryPathUnstated => {
                "leaves_canonical_source_exact_write_target_sync_or_recovery_path_unstated"
            }
            Self::LetsOneStateClassHideAnotherWhenBothMateriallyAffectBehavior => {
                "lets_one_state_class_hide_another_when_both_materially_affect_behavior"
            }
            Self::AccessibilityRouteDropped => "accessibility_route_dropped",
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamConstrainedFileStateNarrowed => "upstream_constrained_file_state_narrowed",
        }
    }
}

/// The controlled badge grammar a constrained-object profile presents.
///
/// These six words plus the co-applicable state labels must be identical across every consumer surface that
/// shows the same profile. The badge-role word must be a frozen role token; the rest are controlled words the
/// profile's badge carries. A surface may narrow which actions remain, but it may never reword any of these
/// values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileStateBadgeGroupGrammar {
    /// Badge-role word (must be a frozen [`M5ConstrainedFileStateRole`] token).
    pub badge_role_word: String,
    /// The controlled state-class label word (`read_only`, `generated`, `policy_locked`, `managed`,
    /// `projection`, or `captured_snapshot`).
    pub state_class_label_word: String,
    /// The plain-language reason / cause word the reason strip carries.
    pub reason_word: String,
    /// The canonical source or live target the constrained object relates back to.
    pub canonical_source_word: String,
    /// The write-disposition word that makes the object mechanically distinct from a directly-writable object.
    pub write_disposition_word: String,
    /// The nearest safe next-step word (duplicate / detach / overlay / regenerate / request-approval).
    pub safe_next_step_word: String,
    /// The controlled labels for any co-applicable state classes (empty for a single-state object); when an
    /// object is multi-state both facts stay visible here rather than one badge hiding another.
    pub co_applicable_state_labels: Vec<String>,
}

impl FileStateBadgeGroupGrammar {
    /// Whether every scalar grammar word is present.
    pub fn all_present(&self) -> bool {
        !self.badge_role_word.trim().is_empty()
            && !self.state_class_label_word.trim().is_empty()
            && !self.reason_word.trim().is_empty()
            && !self.canonical_source_word.trim().is_empty()
            && !self.write_disposition_word.trim().is_empty()
            && !self.safe_next_step_word.trim().is_empty()
    }

    /// Whether the badge-role word is a member of the frozen role vocabulary.
    pub fn badge_role_word_in_vocabulary(&self) -> bool {
        is_known_constrained_file_state_role_token(self.badge_role_word.trim())
    }

    /// Whether the profile honours the constrained-object rule: a badge whose role must be present before the
    /// object may be surfaced as a constrained object must pair it with a real write-constrained disposition
    /// word and never collapse to a directly-writable / writable / editable / unconstrained sentinel.
    pub fn write_disposition_satisfied(&self) -> bool {
        match constrained_file_state_role_from_token(self.badge_role_word.trim()) {
            Some(role) if role.must_be_present_before_surfacing_as_constrained_object() => {
                let disposition = self.write_disposition_word.trim().to_lowercase();
                !disposition.is_empty()
                    && !WRITE_DISPOSITION_ABSENT_SENTINELS.contains(&disposition.as_str())
            }
            _ => true,
        }
    }

    /// Whether every co-applicable state label is non-empty.
    pub fn co_applicable_labels_present(&self) -> bool {
        self.co_applicable_state_labels
            .iter()
            .all(|label| !label.trim().is_empty())
    }
}

/// The explicit note a narrowed posture shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeGroupNarrowNote {
    /// Why the posture narrowed.
    pub reason: BadgeGroupNarrowReason,
    /// Note naming the preserved grammar (never omitted).
    pub preserved_grammar_note: String,
    /// The next action offered.
    pub next_action: BadgeGroupNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadgeGroupRenderDisclosure {
    /// The parity state the posture requires.
    pub parity_state: BadgeGroupParityState,
    /// The narrow reason the posture requires, if any.
    pub narrow_reason: Option<BadgeGroupNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<BadgeGroupNarrowNextAction>,
    /// Whether the binding must carry an explicit narrow note.
    pub needs_narrow_note: bool,
    /// Whether the binding must carry an explicit palette-availability note.
    pub needs_palette_availability_note: bool,
    /// Whether the binding must carry an explicit export-safe-detail note.
    pub needs_export_detail_note: bool,
    /// Whether the binding offers a validated open-safe-next-step action.
    pub offers_safe_next_step: bool,
}

/// Resolves the render disclosures a consumer binding must carry from its posture.
///
/// The full-badge-group posture renders the full safe action set plus a validated open-safe-next-step action.
/// A compact status chip, a gated palette availability, and an exported view each narrow the action set and
/// disclose the narrowing through an explicit note — but all three keep every badge grammar word.
pub const fn resolve_badge_group_render_disclosure(
    posture: BadgeRenderPosture,
) -> BadgeGroupRenderDisclosure {
    match posture {
        BadgeRenderPosture::FullBadgeGroup => BadgeGroupRenderDisclosure {
            parity_state: BadgeGroupParityState::FacetsPreserved,
            narrow_reason: None,
            narrow_next_action: None,
            needs_narrow_note: false,
            needs_palette_availability_note: false,
            needs_export_detail_note: false,
            offers_safe_next_step: true,
        },
        BadgeRenderPosture::CompactStatusChip => BadgeGroupRenderDisclosure {
            parity_state: BadgeGroupParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(BadgeGroupNarrowReason::CompactedToStatusChip),
            narrow_next_action: Some(BadgeGroupNarrowNextAction::OpenFullBadgeGroup),
            needs_narrow_note: true,
            needs_palette_availability_note: false,
            needs_export_detail_note: false,
            offers_safe_next_step: false,
        },
        BadgeRenderPosture::PaletteAvailabilityGated => BadgeGroupRenderDisclosure {
            parity_state: BadgeGroupParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(BadgeGroupNarrowReason::PaletteAvailabilityGatedDisclosed),
            narrow_next_action: Some(BadgeGroupNarrowNextAction::OpenCommandDetail),
            needs_narrow_note: true,
            needs_palette_availability_note: true,
            needs_export_detail_note: false,
            offers_safe_next_step: false,
        },
        BadgeRenderPosture::ExportRedacted => BadgeGroupRenderDisclosure {
            parity_state: BadgeGroupParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(BadgeGroupNarrowReason::ExportRedactionNarrowed),
            narrow_next_action: Some(BadgeGroupNarrowNextAction::OpenFullDetail),
            needs_narrow_note: true,
            needs_palette_availability_note: false,
            needs_export_detail_note: true,
            offers_safe_next_step: false,
        },
    }
}

/// One consumer binding: a constrained-object class rendered on one consumer surface in one posture for one
/// constrained-object profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileStateBadgeGroupConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable constrained-object-profile id (shared across surfaces that show the same profile).
    pub object_profile_id: String,
    /// Human-readable constrained-object-profile identity.
    pub object_profile_label: String,
    /// Which constrained-object class this binding renders as its primary state.
    pub object_class: M5ConstrainedFileStateObject,
    /// Any co-applicable state classes that also apply (empty for a single-state object); when present, both
    /// facts must stay visible.
    pub co_applicable_states: Vec<M5ConstrainedFileStateObject>,
    /// Which consumer surface renders it.
    pub consumer: M5ConstrainedFileStateConsumerSurface,
    /// Which render posture this surface takes.
    pub posture: BadgeRenderPosture,
    /// The controlled badge grammar presented (identical across surfaces for one profile).
    pub badge_grammar: FileStateBadgeGroupGrammar,
    /// Whether grammar is preserved in full or a narrowing is disclosed.
    pub parity_state: BadgeGroupParityState,
    /// The discoverable action set allowed on this badge-group view.
    pub allowed_actions: Vec<BadgeGroupAction>,
    /// The accessibility routes through which the state class, reason, and next safe action can be discovered
    /// without pointer-only chrome.
    pub accessibility_routes: Vec<M5ConstrainedFileStateAccessibilityRoute>,
    /// The explicit narrow note; required and complete when the posture narrows.
    pub narrow_note: Option<BadgeGroupNarrowNote>,
    /// Palette-availability note; required and non-empty when the disclosure demands it.
    pub palette_availability_note: String,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Guardrail: this surface presents a constrained object as directly writable or hides its recovery /
    /// regenerate path. MUST be `false`.
    pub presents_constrained_object_as_directly_writable_or_hides_recovery_path: bool,
    /// Guardrail: this surface lets a generated / managed / projection / archived object silently fall back to
    /// a lossy direct write. MUST be `false`.
    pub lets_generated_managed_projection_or_archived_objects_silently_fall_back_to_lossy_direct_write:
        bool,
    /// Guardrail: this surface gives an AI / automation / import / repair flow a hidden bypass around the
    /// constrained-state rules. MUST be `false`.
    pub gives_ai_automation_import_or_repair_flows_a_hidden_bypass: bool,
    /// Guardrail: this surface leaves the canonical source, exact write target, preserved-versus-lost sync, or
    /// recovery / regenerate path unstated. MUST be `false`.
    pub leaves_canonical_source_exact_write_target_sync_or_recovery_path_unstated: bool,
    /// Guardrail: this surface lets one state class hide another when both materially affect behavior. MUST be
    /// `false`.
    pub lets_one_state_class_hide_another_when_both_materially_affect_behavior: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl FileStateBadgeGroupConsumerBinding {
    /// Disclosures this binding must carry, derived from its posture.
    pub const fn disclosure(&self) -> BadgeGroupRenderDisclosure {
        resolve_badge_group_render_disclosure(self.posture)
    }

    /// Whether this binding renders below the full badge-group view.
    pub const fn is_narrowed(&self) -> bool {
        self.posture.is_narrowed()
    }

    /// Whether this binding renders a multi-state (more than one co-applicable constraint) object.
    pub fn is_multi_state(&self) -> bool {
        !self.co_applicable_states.is_empty()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.presents_constrained_object_as_directly_writable_or_hides_recovery_path
            && !self
                .lets_generated_managed_projection_or_archived_objects_silently_fall_back_to_lossy_direct_write
            && !self.gives_ai_automation_import_or_repair_flows_a_hidden_bypass
            && !self.leaves_canonical_source_exact_write_target_sync_or_recovery_path_unstated
            && !self.lets_one_state_class_hide_another_when_both_materially_affect_behavior
    }

    /// Whether the safe base action set is present.
    pub fn has_safe_base_actions(&self) -> bool {
        BadgeGroupAction::SAFE_BASE
            .iter()
            .all(|action| self.allowed_actions.contains(action))
    }

    /// Whether the open-safe-next-step action is present exactly when the posture offers it.
    pub fn safe_next_step_action_matches_posture(&self) -> bool {
        let offered = self.disclosure().offers_safe_next_step;
        let present = self
            .allowed_actions
            .contains(&BadgeGroupAction::OpenSafeNextStep);
        offered == present
    }

    /// Whether the multi-state facets stay consistent: the binding's co-applicable state classes, the grammar
    /// labels, and the requirement that every co-state is distinct from the primary object class all hold, so
    /// no co-applicable state is hidden.
    pub fn multi_state_facets_consistent(&self) -> bool {
        if self.co_applicable_states.len() != self.badge_grammar.co_applicable_state_labels.len() {
            return false;
        }
        if !self.badge_grammar.co_applicable_labels_present() {
            return false;
        }
        let mut seen: BTreeSet<M5ConstrainedFileStateObject> = BTreeSet::new();
        seen.insert(self.object_class);
        for state in &self.co_applicable_states {
            if !seen.insert(*state) {
                return false;
            }
        }
        true
    }

    /// Whether keyboard focus and screen-reader announcement are both discoverable.
    pub fn accessibility_state_discoverable(&self) -> bool {
        self.accessibility_routes
            .contains(&M5ConstrainedFileStateAccessibilityRoute::KeyboardFocusable)
            && self
                .accessibility_routes
                .contains(&M5ConstrainedFileStateAccessibilityRoute::ScreenReaderAnnounced)
    }

    /// Whether this binding points at the canonical per-domain schema and the matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let domain_ref = self.object_class.canonical_domain_schema_ref();
        self.source_contract_refs
            .iter()
            .any(|reference| reference == domain_ref)
            && self
                .source_contract_refs
                .iter()
                .any(|reference| reference == M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileStateBadgeGroupConsumersTrustReview {
    /// Object-class reuse is proven by fixtures rather than inferred from screenshots.
    pub object_class_reuse_proven_by_fixtures: bool,
    /// The same profile presents the same badge grammar across surfaces.
    pub same_profile_same_badge_across_surfaces: bool,
    /// Every badge-role word is a frozen role token.
    pub badge_role_words_stay_in_frozen_vocabulary: bool,
    /// A badge's write disposition never masquerades as a directly-writable object.
    pub write_disposition_never_masquerades_as_directly_writable: bool,
    /// A constrained object is never presented as directly writable and its recovery path is never hidden.
    pub constrained_object_never_directly_writable_recovery_never_hidden: bool,
    /// A generated / managed / projection / archived object never silently falls back to a lossy direct write.
    pub no_silent_lossy_direct_write_fallback: bool,
    /// AI / automation / import / repair flows never get a hidden bypass around the constrained-state rules.
    pub no_hidden_bypass_for_ai_automation_import_repair: bool,
    /// Canonical source, exact write target, preserved-versus-lost sync, and recovery / regenerate paths stay
    /// explicit.
    pub canonical_source_write_target_sync_recovery_always_stated: bool,
    /// A multi-state object always keeps every co-applicable state visible.
    pub multi_state_objects_keep_every_state_visible: bool,
    /// Accessibility routes for the state class, reason, and next safe action are present.
    pub accessibility_routes_present_for_state_reason_and_next_step: bool,
    /// Narrowing is disclosed across full, compact-chip, palette-gated, and exported postures.
    pub narrowing_disclosed_across_postures: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the object class.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl FileStateBadgeGroupConsumersTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.object_class_reuse_proven_by_fixtures
            && self.same_profile_same_badge_across_surfaces
            && self.badge_role_words_stay_in_frozen_vocabulary
            && self.write_disposition_never_masquerades_as_directly_writable
            && self.constrained_object_never_directly_writable_recovery_never_hidden
            && self.no_silent_lossy_direct_write_fallback
            && self.no_hidden_bypass_for_ai_automation_import_repair
            && self.canonical_source_write_target_sync_recovery_always_stated
            && self.multi_state_objects_keep_every_state_visible
            && self.accessibility_routes_present_for_state_reason_and_next_step
            && self.narrowing_disclosed_across_postures
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileStateBadgeGroupConsumersProjection {
    /// The tab-chrome surface consumes the shared badge group.
    pub tab_chrome_consumes_badge_group: bool,
    /// The breadcrumb-trail surface consumes the shared badge group.
    pub breadcrumb_trail_consumes_badge_group: bool,
    /// The status-bar surface consumes the shared badge group.
    pub status_bar_consumes_badge_group: bool,
    /// The command-palette surface consumes the shared badge group.
    pub command_palette_consumes_badge_group: bool,
    /// The editor-banner surface consumes the shared badge group.
    pub editor_banner_consumes_badge_group: bool,
    /// The diff / review-header surface consumes the shared badge group.
    pub diff_review_header_consumes_badge_group: bool,
    /// The write-review-sheet surface consumes the shared badge group.
    pub write_review_sheet_consumes_badge_group: bool,
    /// The AI / automation-path surface consumes the shared badge group.
    pub ai_automation_path_consumes_badge_group: bool,
    /// The support / export-packet surface consumes the shared badge group.
    pub support_export_packet_consumes_badge_group: bool,
    /// Every object class is adopted by two or more consumers.
    pub every_object_class_adopted_by_two_or_more_consumers: bool,
    /// Badge grammar is identical for the same profile.
    pub badge_grammar_identical_for_same_profile: bool,
    /// Multi-state objects keep both facts visible.
    pub multi_state_objects_keep_both_facts_visible: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps an object class back to one constrained-file-state object class.
    pub export_maps_back_to_one_constrained_file_state_object: bool,
}

impl FileStateBadgeGroupConsumersProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.tab_chrome_consumes_badge_group
            && self.breadcrumb_trail_consumes_badge_group
            && self.status_bar_consumes_badge_group
            && self.command_palette_consumes_badge_group
            && self.editor_banner_consumes_badge_group
            && self.diff_review_header_consumes_badge_group
            && self.write_review_sheet_consumes_badge_group
            && self.ai_automation_path_consumes_badge_group
            && self.support_export_packet_consumes_badge_group
            && self.every_object_class_adopted_by_two_or_more_consumers
            && self.badge_grammar_identical_for_same_profile
            && self.multi_state_objects_keep_both_facts_visible
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_constrained_file_state_object
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileStateBadgeGroupConsumersProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5FileStateBadgeGroupConsumersPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5FileStateBadgeGroupConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<FileStateBadgeGroupConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<FileStateBadgeGroupConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5ConstrainedFileStateConsumerSurface>,
    /// Trust review block.
    pub trust_review: FileStateBadgeGroupConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: FileStateBadgeGroupConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: FileStateBadgeGroupConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe file-state badge-group consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FileStateBadgeGroupConsumersPacket {
    /// Record kind; must equal [`M5_FILE_STATE_BADGE_GROUP_CONSUMERS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_FILE_STATE_BADGE_GROUP_CONSUMERS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<FileStateBadgeGroupConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<FileStateBadgeGroupConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5ConstrainedFileStateConsumerSurface>,
    /// Trust review block.
    pub trust_review: FileStateBadgeGroupConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: FileStateBadgeGroupConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: FileStateBadgeGroupConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5FileStateBadgeGroupConsumersPacket {
    /// Builds a file-state badge-group consumer packet from stable-lane input.
    pub fn new(input: M5FileStateBadgeGroupConsumersPacketInput) -> Self {
        Self {
            record_kind: M5_FILE_STATE_BADGE_GROUP_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: M5_FILE_STATE_BADGE_GROUP_CONSUMERS_SCHEMA_VERSION,
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

    /// Validates the file-state badge-group consumer invariants.
    pub fn validate(&self) -> Vec<M5FileStateBadgeGroupConsumersViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_FILE_STATE_BADGE_GROUP_CONSUMERS_RECORD_KIND {
            violations.push(M5FileStateBadgeGroupConsumersViolation::WrongRecordKind);
        }
        if self.schema_version != M5_FILE_STATE_BADGE_GROUP_CONSUMERS_SCHEMA_VERSION {
            violations.push(M5FileStateBadgeGroupConsumersViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5FileStateBadgeGroupConsumersViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5FileStateBadgeGroupConsumersViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5FileStateBadgeGroupConsumersViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5FileStateBadgeGroupConsumersViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(M5FileStateBadgeGroupConsumersViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5FileStateBadgeGroupConsumersViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("file-state badge-group consumer packet serializes"),
        ) {
            violations.push(M5FileStateBadgeGroupConsumersViolation::RawBoundaryMaterialInExport);
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
            .expect("file-state badge-group consumer packet serializes")
    }

    /// Deterministic matrix CSV, one row per consumer binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "object_class,co_applicable_states,consumer,posture,badge_role_word,parity_state\n",
        );
        for binding in &self.consumer_bindings {
            let co_states = binding
                .co_applicable_states
                .iter()
                .map(|state| state.as_str())
                .collect::<Vec<_>>()
                .join("|");
            out.push_str(&format!(
                "{},{},{},{},{},{}\n",
                binding.object_class.as_str(),
                co_states,
                binding.consumer.as_str(),
                binding.posture.as_str(),
                binding.badge_grammar.badge_role_word,
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
        let multi_state = self
            .consumer_bindings
            .iter()
            .filter(|binding| binding.is_multi_state())
            .count();

        let mut out = String::new();
        out.push_str(
            "# File-State Badge Groups & Reason Strips: One Vocabulary Across Surfaces\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Consumer bindings: {} ({} narrowed, {} multi-state)\n",
            self.consumer_bindings.len(),
            narrowed,
            multi_state
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Consumer bindings\n\n");
        for binding in &self.consumer_bindings {
            let co_states = if binding.co_applicable_states.is_empty() {
                String::new()
            } else {
                format!(
                    " (+ {})",
                    binding
                        .co_applicable_states
                        .iter()
                        .map(|state| state.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            out.push_str(&format!(
                "- **{}** [`{}`]: object `{}`{} on `{}`, posture `{}`, role `{}`\n",
                binding.object_profile_label,
                binding.binding_id,
                binding.object_class.as_str(),
                co_states,
                binding.consumer.as_str(),
                binding.posture.as_str(),
                binding.badge_grammar.badge_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in file-state badge-group consumer export.
#[derive(Debug)]
pub enum M5FileStateBadgeGroupConsumersArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5FileStateBadgeGroupConsumersViolation>),
}

impl fmt::Display for M5FileStateBadgeGroupConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "file-state badge-group consumer export parse failed: {error}"
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
                    "file-state badge-group consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5FileStateBadgeGroupConsumersArtifactError {}

/// Validation failures emitted by [`M5FileStateBadgeGroupConsumersPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5FileStateBadgeGroupConsumersViolation {
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
    /// A binding's badge grammar values are incomplete.
    GrammarFacetIncomplete,
    /// A binding's badge-role word is not a frozen role token.
    BadgeRoleWordOutsideVocabulary,
    /// A binding's gate-role dropped its write-constrained disposition.
    WriteDispositionMissingForGateRole,
    /// A binding's parity state does not match its posture.
    ParityStateMismatch,
    /// Two surfaces show the same profile with different badge grammar.
    BadgeGroupVocabularyDriftAcrossSurfaces,
    /// A shared object class is not adopted by at least two distinct consumers.
    ObjectClassReuseUnproven,
    /// A support / export binding does not point at the canonical contracts.
    SupportExportReferenceMissing,
    /// A narrowed binding is missing its explicit narrow note.
    NarrowNoteMissing,
    /// A narrow note's reason does not match the required narrow reason.
    NarrowReasonMismatch,
    /// A narrow note's next action does not match the required next action.
    NarrowNextActionMismatch,
    /// A narrow note is missing its preserved-grammar note.
    NarrowNotePreservedGrammarMissing,
    /// A narrow note is missing its next-action copy.
    NarrowNextActionLabelMissing,
    /// A full-badge-group binding carries a narrow note it must not.
    UnexpectedNarrowNote,
    /// A binding that needs an explicit palette-availability note is missing it.
    PaletteAvailabilityNoteMissing,
    /// A binding that needs an explicit export-detail note is missing it.
    ExportDetailNoteMissing,
    /// A binding is missing the safe base action set.
    SafeBaseActionsMissing,
    /// A binding's open-safe-next-step action does not match its posture.
    SafeNextStepActionPostureMismatch,
    /// A multi-state binding hides a co-applicable state facet.
    MultiStateFacetHidden,
    /// A binding cannot discover its state via keyboard focus and screen-reader announcement.
    AccessibilityStateUndiscoverable,
    /// A binding presents a constrained object as directly writable or hides its recovery / regenerate path.
    PresentsConstrainedObjectAsDirectlyWritableOrHidesRecoveryPath,
    /// A binding lets a generated / managed / projection / archived object silently fall back to a lossy direct
    /// write.
    LetsGeneratedManagedProjectionOrArchivedObjectsSilentlyFallBackToLossyDirectWrite,
    /// A binding gives an AI / automation / import / repair flow a hidden bypass.
    GivesAiAutomationImportOrRepairFlowsAHiddenBypass,
    /// A binding leaves the canonical source, exact write target, sync, or recovery path unstated.
    LeavesCanonicalSourceExactWriteTargetSyncOrRecoveryPathUnstated,
    /// A binding lets one state class hide another when both materially affect behavior.
    LetsOneStateClassHideAnotherWhenBothMateriallyAffectBehavior,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared object class appears among the bindings.
    ObjectClassCoverageMissing,
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

impl M5FileStateBadgeGroupConsumersViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ConsumerBindingsMissing => "consumer_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::GrammarFacetIncomplete => "grammar_facet_incomplete",
            Self::BadgeRoleWordOutsideVocabulary => "badge_role_word_outside_vocabulary",
            Self::WriteDispositionMissingForGateRole => "write_disposition_missing_for_gate_role",
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::BadgeGroupVocabularyDriftAcrossSurfaces => {
                "badge_group_vocabulary_drift_across_surfaces"
            }
            Self::ObjectClassReuseUnproven => "object_class_reuse_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::NarrowNoteMissing => "narrow_note_missing",
            Self::NarrowReasonMismatch => "narrow_reason_mismatch",
            Self::NarrowNextActionMismatch => "narrow_next_action_mismatch",
            Self::NarrowNotePreservedGrammarMissing => "narrow_note_preserved_grammar_missing",
            Self::NarrowNextActionLabelMissing => "narrow_next_action_label_missing",
            Self::UnexpectedNarrowNote => "unexpected_narrow_note",
            Self::PaletteAvailabilityNoteMissing => "palette_availability_note_missing",
            Self::ExportDetailNoteMissing => "export_detail_note_missing",
            Self::SafeBaseActionsMissing => "safe_base_actions_missing",
            Self::SafeNextStepActionPostureMismatch => "safe_next_step_action_posture_mismatch",
            Self::MultiStateFacetHidden => "multi_state_facet_hidden",
            Self::AccessibilityStateUndiscoverable => "accessibility_state_undiscoverable",
            Self::PresentsConstrainedObjectAsDirectlyWritableOrHidesRecoveryPath => {
                "presents_constrained_object_as_directly_writable_or_hides_recovery_path"
            }
            Self::LetsGeneratedManagedProjectionOrArchivedObjectsSilentlyFallBackToLossyDirectWrite => {
                "lets_generated_managed_projection_or_archived_objects_silently_fall_back_to_lossy_direct_write"
            }
            Self::GivesAiAutomationImportOrRepairFlowsAHiddenBypass => {
                "gives_ai_automation_import_or_repair_flows_a_hidden_bypass"
            }
            Self::LeavesCanonicalSourceExactWriteTargetSyncOrRecoveryPathUnstated => {
                "leaves_canonical_source_exact_write_target_sync_or_recovery_path_unstated"
            }
            Self::LetsOneStateClassHideAnotherWhenBothMateriallyAffectBehavior => {
                "lets_one_state_class_hide_another_when_both_materially_affect_behavior"
            }
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::ObjectClassCoverageMissing => "object_class_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable file-state badge-group consumer export.
pub fn current_stable_m5_file_state_badge_group_consumers_export(
) -> Result<M5FileStateBadgeGroupConsumersPacket, M5FileStateBadgeGroupConsumersArtifactError> {
    let packet: M5FileStateBadgeGroupConsumersPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/support/m5-file-state-badge-group-consumers/support_export.json"
    )))
    .map_err(M5FileStateBadgeGroupConsumersArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5FileStateBadgeGroupConsumersArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5FileStateBadgeGroupConsumersPacket,
    violations: &mut Vec<M5FileStateBadgeGroupConsumersViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_FILE_STATE_BADGE_GROUP_CONSUMERS_SCHEMA_REF,
        M5_FILE_STATE_BADGE_GROUP_CONSUMERS_DOC_REF,
        M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
        M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF,
    ];
    // The six object classes map to three canonical domain schemas; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object_class in M5ConstrainedFileStateObject::ALL {
        domains.insert(object_class.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5FileStateBadgeGroupConsumersViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5FileStateBadgeGroupConsumersPacket,
    violations: &mut Vec<M5FileStateBadgeGroupConsumersViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(M5FileStateBadgeGroupConsumersViolation::ConsumerBindingsMissing);
        return;
    }

    // One vocabulary: the badge grammar must be identical for every binding that renders the same
    // constrained-object profile.
    let mut profile_grammar: BTreeMap<&str, &FileStateBadgeGroupGrammar> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each object class must be adopted by at least two distinct consumers.
    let mut object_consumers: BTreeMap<
        M5ConstrainedFileStateObject,
        BTreeSet<M5ConstrainedFileStateConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5ConstrainedFileStateConsumerSurface> = BTreeSet::new();
    let mut seen_objects: BTreeSet<M5ConstrainedFileStateObject> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.object_profile_id.trim().is_empty()
            || binding.object_profile_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5FileStateBadgeGroupConsumersViolation::BindingIncomplete);
        }
        if !binding.badge_grammar.all_present() {
            violations.push(M5FileStateBadgeGroupConsumersViolation::GrammarFacetIncomplete);
        }
        if !binding.badge_grammar.badge_role_word_in_vocabulary() {
            violations
                .push(M5FileStateBadgeGroupConsumersViolation::BadgeRoleWordOutsideVocabulary);
        }
        if !binding.badge_grammar.write_disposition_satisfied() {
            violations
                .push(M5FileStateBadgeGroupConsumersViolation::WriteDispositionMissingForGateRole);
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5FileStateBadgeGroupConsumersViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations.push(M5FileStateBadgeGroupConsumersViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations
                            .push(M5FileStateBadgeGroupConsumersViolation::NarrowReasonMismatch);
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations.push(
                            M5FileStateBadgeGroupConsumersViolation::NarrowNextActionMismatch,
                        );
                    }
                    if note.preserved_grammar_note.trim().is_empty() {
                        violations.push(
                            M5FileStateBadgeGroupConsumersViolation::NarrowNotePreservedGrammarMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5FileStateBadgeGroupConsumersViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5FileStateBadgeGroupConsumersViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_palette_availability_note
            && binding.palette_availability_note.trim().is_empty()
        {
            violations
                .push(M5FileStateBadgeGroupConsumersViolation::PaletteAvailabilityNoteMissing);
        }
        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations.push(M5FileStateBadgeGroupConsumersViolation::ExportDetailNoteMissing);
        }

        // Action rules.
        if !binding.has_safe_base_actions() {
            violations.push(M5FileStateBadgeGroupConsumersViolation::SafeBaseActionsMissing);
        }
        if !binding.safe_next_step_action_matches_posture() {
            violations
                .push(M5FileStateBadgeGroupConsumersViolation::SafeNextStepActionPostureMismatch);
        }

        // Multi-state facets.
        if !binding.multi_state_facets_consistent() {
            violations.push(M5FileStateBadgeGroupConsumersViolation::MultiStateFacetHidden);
        }

        // Accessibility discovery.
        if !binding.accessibility_state_discoverable() {
            violations
                .push(M5FileStateBadgeGroupConsumersViolation::AccessibilityStateUndiscoverable);
        }

        // Guardrail row-invariants (each must be false).
        if binding.presents_constrained_object_as_directly_writable_or_hides_recovery_path {
            violations.push(
                M5FileStateBadgeGroupConsumersViolation::PresentsConstrainedObjectAsDirectlyWritableOrHidesRecoveryPath,
            );
        }
        if binding
            .lets_generated_managed_projection_or_archived_objects_silently_fall_back_to_lossy_direct_write
        {
            violations.push(
                M5FileStateBadgeGroupConsumersViolation::LetsGeneratedManagedProjectionOrArchivedObjectsSilentlyFallBackToLossyDirectWrite,
            );
        }
        if binding.gives_ai_automation_import_or_repair_flows_a_hidden_bypass {
            violations.push(
                M5FileStateBadgeGroupConsumersViolation::GivesAiAutomationImportOrRepairFlowsAHiddenBypass,
            );
        }
        if binding.leaves_canonical_source_exact_write_target_sync_or_recovery_path_unstated {
            violations.push(
                M5FileStateBadgeGroupConsumersViolation::LeavesCanonicalSourceExactWriteTargetSyncOrRecoveryPathUnstated,
            );
        }
        if binding.lets_one_state_class_hide_another_when_both_materially_affect_behavior {
            violations.push(
                M5FileStateBadgeGroupConsumersViolation::LetsOneStateClassHideAnotherWhenBothMateriallyAffectBehavior,
            );
        }

        // Support / export consumers must map an object class back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations.push(M5FileStateBadgeGroupConsumersViolation::SupportExportReferenceMissing);
        }

        // Grammar-drift accumulation.
        match profile_grammar.get(binding.object_profile_id.as_str()) {
            None => {
                profile_grammar.insert(binding.object_profile_id.as_str(), &binding.badge_grammar);
            }
            Some(existing) => {
                if **existing != binding.badge_grammar && !drift_reported {
                    violations.push(
                        M5FileStateBadgeGroupConsumersViolation::BadgeGroupVocabularyDriftAcrossSurfaces,
                    );
                    drift_reported = true;
                }
            }
        }

        object_consumers
            .entry(binding.object_class)
            .or_default()
            .insert(binding.consumer);
        seen_consumers.insert(binding.consumer);
        seen_objects.insert(binding.object_class);
    }

    // Coverage: every consumer surface and every object class must appear.
    for consumer in M5ConstrainedFileStateConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5FileStateBadgeGroupConsumersViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for object_class in M5ConstrainedFileStateObject::ALL {
        if !seen_objects.contains(&object_class) {
            violations.push(M5FileStateBadgeGroupConsumersViolation::ObjectClassCoverageMissing);
            break;
        }
    }

    // Reuse: every present object class must be adopted by two or more distinct consumers.
    for consumers in object_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5FileStateBadgeGroupConsumersViolation::ObjectClassReuseUnproven);
            break;
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
