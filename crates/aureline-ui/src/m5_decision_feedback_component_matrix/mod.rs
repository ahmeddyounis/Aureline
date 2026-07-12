//! Frozen M5 badge-chip-pill, popover, dialog-sheet, banner-inline-notice, toast, empty-state,
//! loading-state, and consequence-block component matrix.
//!
//! This module locks Aureline's most reused decision and feedback primitives into one export-safe
//! packet. Every claimed M5 surface that still ships its own badge / chip / pill, popover, dialog or
//! sheet, banner or inline notice, toast, empty state, loading state, or consequence block — across the
//! shell, entry, trust, review, repair, and notification surfaces — is named once here and constrained
//! by the same shared state taxonomy (info, success, warning, blocked, pending, degraded, acknowledged,
//! dismissed), the same badge plain-language rule, the same popover focus-return and secondary-only
//! rule, the same dialog rationale / scope / explicit-action truth, the same banner and inline-notice
//! scoping, the same toast durability rule, the same empty-state purpose / emptiness / next-action
//! rule, the same loading-state partial-data rule, and the same consequence-block blast-radius and
//! rollback truth regardless of the feature family that renders it.
//!
//! The matrix does not re-implement domain-specific advisory, trust, repair, review, or notification
//! routing — it is the shared reusable decision/feedback-honesty contract those flows consume. The
//! controlled vocabularies are frozen in one self-describing [`M5DecisionFeedbackVocabularySet`] rather
//! than minted per feature. The single controlled state vocabulary consumers bind to — info, success,
//! warning, blocked, pending, degraded, acknowledged, and dismissed — keeps badge and notice meaning
//! from depending on color alone, keeps popovers from carrying the only critical workflow instruction,
//! keeps high-risk dialogs from using generic Yes/No copy, keeps long-running or reviewable work from
//! being represented as toast-only truth, keeps useful panes from being blanked during loading, and
//! keeps full-screen spinners from replacing partial capability. Raw secret values and private
//! endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_decision_feedback_component_matrix,
    seeded_m5_decision_feedback_component_matrix_dialog_sheet_beta_narrowed,
    seeded_m5_decision_feedback_component_matrix_loading_state_preview_narrowed,
    M5_DECISION_FEEDBACK_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5DecisionFeedbackComponentMatrixPacket`].
pub const M5_DECISION_FEEDBACK_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_badge_chip_pill_popover_dialog_sheet_banner_inline_notice_toast_empty_state_loading_state_and_consequence_block_component_matrix";

/// Schema version for M5 decision-feedback component-matrix records.
pub const M5_DECISION_FEEDBACK_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined decision-feedback component-matrix schema.
pub const M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-decision-feedback-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_DECISION_FEEDBACK_COMPONENT_DOC_REF: &str =
    "docs/components/m5_decision_feedback_components_contract.md";

/// Repo-relative path of the badge-chip-pill canonical component schema.
pub const M5_BADGE_CHIP_PILL_SCHEMA_REF: &str = "schemas/ui/m5-badge-chip-pill.schema.json";

/// Repo-relative path of the popover canonical component schema.
pub const M5_POPOVER_SCHEMA_REF: &str = "schemas/ui/m5-popover.schema.json";

/// Repo-relative path of the dialog-sheet canonical component schema.
pub const M5_DIALOG_SHEET_SCHEMA_REF: &str = "schemas/ui/m5-dialog-sheet.schema.json";

/// Repo-relative path of the banner-inline-notice canonical component schema.
pub const M5_BANNER_INLINE_NOTICE_SCHEMA_REF: &str =
    "schemas/ui/m5-banner-inline-notice.schema.json";

/// Repo-relative path of the toast canonical component schema.
pub const M5_TOAST_SCHEMA_REF: &str = "schemas/ui/m5-toast.schema.json";

/// Repo-relative path of the empty-state canonical component schema.
pub const M5_EMPTY_STATE_SCHEMA_REF: &str = "schemas/ui/m5-empty-state.schema.json";

/// Repo-relative path of the loading-state canonical component schema.
pub const M5_LOADING_STATE_SCHEMA_REF: &str = "schemas/ui/m5-loading-state.schema.json";

/// Repo-relative path of the consequence-block canonical component schema.
pub const M5_CONSEQUENCE_BLOCK_SCHEMA_REF: &str = "schemas/ui/m5-consequence-block.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_DECISION_FEEDBACK_COMPONENT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-decision-feedback-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DECISION_FEEDBACK_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-decision-feedback-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_DECISION_FEEDBACK_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-decision-feedback-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_DECISION_FEEDBACK_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-decision-feedback-component-matrix.md";

/// One of the eight governed decision / feedback primitive families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionFeedbackFamily {
    /// A badge / chip / pill that expands into plain language instead of color-only shorthand.
    BadgeChipPill,
    /// A popover that stays a lightweight secondary control with safe focus return.
    Popover,
    /// A dialog / sheet that names rationale, scope, and explicit actions.
    DialogSheet,
    /// A banner / inline notice that stays scoped and actionable.
    BannerInlineNotice,
    /// A toast that acknowledges without becoming the only durable truth.
    Toast,
    /// An empty state that explains purpose, current emptiness, and next action.
    EmptyState,
    /// A loading state that preserves useful partial data instead of blanking.
    LoadingState,
    /// A consequence block that names blast radius and rollback / help posture.
    ConsequenceBlock,
}

impl M5DecisionFeedbackFamily {
    /// Every governed primitive family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::BadgeChipPill,
        Self::Popover,
        Self::DialogSheet,
        Self::BannerInlineNotice,
        Self::Toast,
        Self::EmptyState,
        Self::LoadingState,
        Self::ConsequenceBlock,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BadgeChipPill => "badge_chip_pill",
            Self::Popover => "popover",
            Self::DialogSheet => "dialog_sheet",
            Self::BannerInlineNotice => "banner_inline_notice",
            Self::Toast => "toast",
            Self::EmptyState => "empty_state",
            Self::LoadingState => "loading_state",
            Self::ConsequenceBlock => "consequence_block",
        }
    }

    /// The canonical per-component schema ref a downstream primitive points at instead of restating this
    /// primitive's state / rationale / scope / recovery truth by hand.
    pub const fn canonical_component_schema_ref(self) -> &'static str {
        match self {
            Self::BadgeChipPill => M5_BADGE_CHIP_PILL_SCHEMA_REF,
            Self::Popover => M5_POPOVER_SCHEMA_REF,
            Self::DialogSheet => M5_DIALOG_SHEET_SCHEMA_REF,
            Self::BannerInlineNotice => M5_BANNER_INLINE_NOTICE_SCHEMA_REF,
            Self::Toast => M5_TOAST_SCHEMA_REF,
            Self::EmptyState => M5_EMPTY_STATE_SCHEMA_REF,
            Self::LoadingState => M5_LOADING_STATE_SCHEMA_REF,
            Self::ConsequenceBlock => M5_CONSEQUENCE_BLOCK_SCHEMA_REF,
        }
    }

    /// `true` when this family must name a controlled badge expression.
    pub const fn declares_badge_expression(self) -> bool {
        matches!(self, Self::BadgeChipPill)
    }

    /// `true` when this family must name a controlled popover dismissal.
    pub const fn declares_popover_dismissal(self) -> bool {
        matches!(self, Self::Popover)
    }

    /// `true` when this family must name a controlled dialog action model.
    pub const fn declares_dialog_action_model(self) -> bool {
        matches!(self, Self::DialogSheet)
    }

    /// `true` when this family must name a controlled notice scope.
    pub const fn declares_notice_scope(self) -> bool {
        matches!(self, Self::BannerInlineNotice)
    }

    /// `true` when this family must name a controlled toast durability.
    pub const fn declares_toast_durability(self) -> bool {
        matches!(self, Self::Toast)
    }

    /// `true` when this family must name a controlled empty-state purpose.
    pub const fn declares_empty_state_purpose(self) -> bool {
        matches!(self, Self::EmptyState)
    }

    /// `true` when this family must name a controlled loading fidelity.
    pub const fn declares_loading_fidelity(self) -> bool {
        matches!(self, Self::LoadingState)
    }

    /// `true` when this family must name a controlled consequence disclosure.
    pub const fn declares_consequence_disclosure(self) -> bool {
        matches!(self, Self::ConsequenceBlock)
    }
}

/// The single controlled state vocabulary every shell, entry, trust, review, repair, or notification
/// consumer binds to. These are the exact acceptance-criteria tokens that keep `info`, `success`,
/// `warning`, `blocked`, `pending`, `degraded`, `acknowledged`, and `dismissed` meaning the same thing
/// everywhere these primitives ship. No feature family invents a parallel word for any of these states,
/// and none of them may be conveyed by color alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionFeedbackDisposition {
    /// Neutral / informational state.
    Info,
    /// A successful / completed state.
    Success,
    /// A warning that needs attention but does not block.
    Warning,
    /// A blocked state that stops the flow until resolved.
    Blocked,
    /// Work is pending / in progress.
    Pending,
    /// A required signal is unavailable, so the primitive is degraded (never hidden behind chrome).
    Degraded,
    /// The user has acknowledged the primitive.
    Acknowledged,
    /// The primitive has been dismissed.
    Dismissed,
}

impl M5DecisionFeedbackDisposition {
    /// Every state token, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Info,
        Self::Success,
        Self::Warning,
        Self::Blocked,
        Self::Pending,
        Self::Degraded,
        Self::Acknowledged,
        Self::Dismissed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Blocked => "blocked",
            Self::Pending => "pending",
            Self::Degraded => "degraded",
            Self::Acknowledged => "acknowledged",
            Self::Dismissed => "dismissed",
        }
    }

    /// Whether this state carries meaning that must never be conveyed by color alone and must never be
    /// hidden behind generic chrome (`warning`, `blocked`, `degraded`).
    pub const fn demands_plain_language_explanation(self) -> bool {
        matches!(self, Self::Warning | Self::Blocked | Self::Degraded)
    }
}

/// Controlled badge / chip / pill expression — how a badge conveys meaning beyond color, so a badge,
/// chip, or pill always expands into plain language and is never color-only shorthand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeExpression {
    /// A plain text label.
    TextLabel,
    /// An icon paired with a text label.
    IconWithText,
    /// A count paired with a label.
    CountWithLabel,
    /// A named status word.
    StatusWord,
    /// A removable chip with an explicit remove affordance.
    RemovableChip,
    /// Color-only meaning, which is disallowed.
    ColorOnlyDisallowed,
}

impl M5BadgeExpression {
    /// Every badge expression, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TextLabel,
        Self::IconWithText,
        Self::CountWithLabel,
        Self::StatusWord,
        Self::RemovableChip,
        Self::ColorOnlyDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TextLabel => "text_label",
            Self::IconWithText => "icon_with_text",
            Self::CountWithLabel => "count_with_label",
            Self::StatusWord => "status_word",
            Self::RemovableChip => "removable_chip",
            Self::ColorOnlyDisallowed => "color_only_disallowed",
        }
    }
}

/// Controlled popover dismissal / focus behavior — how a popover closes and returns focus, so a popover
/// stays a lightweight non-modal secondary control and never carries the only critical instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PopoverDismissal {
    /// Dismisses on an outside click.
    DismissOnOutsideClick,
    /// Dismisses on Escape.
    DismissOnEscape,
    /// Offers an explicit close button.
    ExplicitCloseButton,
    /// Returns focus to the trigger when closed.
    FocusReturnsToTrigger,
    /// Stays a non-modal secondary surface.
    NonModalSecondary,
    /// Carrying the only critical workflow instruction, which is disallowed.
    CarriesOnlyInstructionDisallowed,
}

impl M5PopoverDismissal {
    /// Every popover dismissal behavior, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DismissOnOutsideClick,
        Self::DismissOnEscape,
        Self::ExplicitCloseButton,
        Self::FocusReturnsToTrigger,
        Self::NonModalSecondary,
        Self::CarriesOnlyInstructionDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DismissOnOutsideClick => "dismiss_on_outside_click",
            Self::DismissOnEscape => "dismiss_on_escape",
            Self::ExplicitCloseButton => "explicit_close_button",
            Self::FocusReturnsToTrigger => "focus_returns_to_trigger",
            Self::NonModalSecondary => "non_modal_secondary",
            Self::CarriesOnlyInstructionDisallowed => "carries_only_instruction_disallowed",
        }
    }
}

/// Controlled dialog / sheet action model — how a dialog names its actions, rationale, and scope, so a
/// high-risk dialog never uses generic Yes/No copy and every dialog names rationale and scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DialogActionModel {
    /// Names specific verbs for each action.
    NamedSpecificActions,
    /// A primary action plus an explicit cancel.
    PrimaryAndCancel,
    /// A destructive confirm named after the destructive verb.
    DestructiveConfirmNamed,
    /// States rationale and scope before asking to confirm.
    RationaleAndScopeStated,
    /// Dismissible with a safe default.
    DismissibleSafe,
    /// Generic Yes/No copy, which is disallowed in high-risk dialogs.
    GenericYesNoDisallowed,
}

impl M5DialogActionModel {
    /// Every dialog action model, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NamedSpecificActions,
        Self::PrimaryAndCancel,
        Self::DestructiveConfirmNamed,
        Self::RationaleAndScopeStated,
        Self::DismissibleSafe,
        Self::GenericYesNoDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NamedSpecificActions => "named_specific_actions",
            Self::PrimaryAndCancel => "primary_and_cancel",
            Self::DestructiveConfirmNamed => "destructive_confirm_named",
            Self::RationaleAndScopeStated => "rationale_and_scope_stated",
            Self::DismissibleSafe => "dismissible_safe",
            Self::GenericYesNoDisallowed => "generic_yes_no_disallowed",
        }
    }
}

/// Controlled banner / inline-notice scope — how far a notice reaches and whether it is actionable, so a
/// banner or inline notice stays scoped and actionable and never relies on color alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NoticeScope {
    /// Scoped to a page.
    PageScoped,
    /// Scoped to a section.
    SectionScoped,
    /// Inline with a specific field.
    FieldInline,
    /// A global system-level notice.
    GlobalSystem,
    /// Actionable with a named next step.
    ActionableWithNextStep,
    /// Unscoped or color-only, which is disallowed.
    UnscopedColorOnlyDisallowed,
}

impl M5NoticeScope {
    /// Every notice scope, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PageScoped,
        Self::SectionScoped,
        Self::FieldInline,
        Self::GlobalSystem,
        Self::ActionableWithNextStep,
        Self::UnscopedColorOnlyDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PageScoped => "page_scoped",
            Self::SectionScoped => "section_scoped",
            Self::FieldInline => "field_inline",
            Self::GlobalSystem => "global_system",
            Self::ActionableWithNextStep => "actionable_with_next_step",
            Self::UnscopedColorOnlyDisallowed => "unscoped_color_only_disallowed",
        }
    }
}

/// Controlled toast durability — how a toast persists, so a toast acknowledges work without becoming the
/// only durable truth for long-running or reviewable work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ToastDurability {
    /// A transient acknowledgment.
    TransientAcknowledgment,
    /// Mirrored to the activity center for durable truth.
    MirroredToActivityCenter,
    /// Dismissible by the user.
    DismissibleByUser,
    /// Auto-dismisses on a timer.
    AutoDismissTimed,
    /// Its action is retained elsewhere (not toast-only).
    ActionRetainedElsewhere,
    /// Being the only durable truth, which is disallowed.
    ToastOnlyTruthDisallowed,
}

impl M5ToastDurability {
    /// Every toast durability, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TransientAcknowledgment,
        Self::MirroredToActivityCenter,
        Self::DismissibleByUser,
        Self::AutoDismissTimed,
        Self::ActionRetainedElsewhere,
        Self::ToastOnlyTruthDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TransientAcknowledgment => "transient_acknowledgment",
            Self::MirroredToActivityCenter => "mirrored_to_activity_center",
            Self::DismissibleByUser => "dismissible_by_user",
            Self::AutoDismissTimed => "auto_dismiss_timed",
            Self::ActionRetainedElsewhere => "action_retained_elsewhere",
            Self::ToastOnlyTruthDisallowed => "toast_only_truth_disallowed",
        }
    }
}

/// Controlled empty-state purpose — what an empty state explains, so it explains its purpose, why it is
/// currently empty, and the next action rather than showing a blank pane with no explanation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmptyStatePurpose {
    /// Explains the pane's purpose.
    ExplainsPurpose,
    /// Explains why the pane is currently empty.
    ExplainsCurrentEmptiness,
    /// Offers a next action.
    OffersNextAction,
    /// First-run guidance.
    FirstRunGuidance,
    /// A filtered no-results state.
    FilteredNoResults,
    /// A blank pane with no explanation, which is disallowed.
    BlankNoExplanationDisallowed,
}

impl M5EmptyStatePurpose {
    /// Every empty-state purpose, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExplainsPurpose,
        Self::ExplainsCurrentEmptiness,
        Self::OffersNextAction,
        Self::FirstRunGuidance,
        Self::FilteredNoResults,
        Self::BlankNoExplanationDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplainsPurpose => "explains_purpose",
            Self::ExplainsCurrentEmptiness => "explains_current_emptiness",
            Self::OffersNextAction => "offers_next_action",
            Self::FirstRunGuidance => "first_run_guidance",
            Self::FilteredNoResults => "filtered_no_results",
            Self::BlankNoExplanationDisallowed => "blank_no_explanation_disallowed",
        }
    }
}

/// Controlled loading fidelity — how a loading state represents in-progress work, so it preserves useful
/// partial data and never uses a full-screen spinner where partial capability exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LoadingFidelity {
    /// A skeleton that preserves the layout.
    SkeletonPreservesLayout,
    /// Partial data retained while the rest loads.
    PartialDataRetained,
    /// Scoped inline progress.
    InlineProgressScoped,
    /// A determinate progress indicator.
    DeterminateProgress,
    /// A scoped indeterminate spinner.
    IndeterminateSpinnerScoped,
    /// A full-screen spinner where partial capability exists, which is disallowed.
    FullScreenSpinnerDisallowed,
}

impl M5LoadingFidelity {
    /// Every loading fidelity, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SkeletonPreservesLayout,
        Self::PartialDataRetained,
        Self::InlineProgressScoped,
        Self::DeterminateProgress,
        Self::IndeterminateSpinnerScoped,
        Self::FullScreenSpinnerDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SkeletonPreservesLayout => "skeleton_preserves_layout",
            Self::PartialDataRetained => "partial_data_retained",
            Self::InlineProgressScoped => "inline_progress_scoped",
            Self::DeterminateProgress => "determinate_progress",
            Self::IndeterminateSpinnerScoped => "indeterminate_spinner_scoped",
            Self::FullScreenSpinnerDisallowed => "full_screen_spinner_disallowed",
        }
    }
}

/// Controlled consequence-block disclosure — what a consequence block names before a risky action, so it
/// names its blast radius and rollback / help posture and never reduces to generic Yes/No ambiguity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConsequenceDisclosure {
    /// Names the blast radius of the action.
    NamedBlastRadius,
    /// Rollback is available and stated.
    RollbackAvailable,
    /// Rollback is unavailable and that is stated.
    RollbackUnavailableStated,
    /// A help path is present.
    HelpPathPresent,
    /// Explicit named actions.
    ExplicitNamedActions,
    /// Generic Yes/No ambiguity, which is disallowed.
    GenericYesNoDisallowed,
}

impl M5ConsequenceDisclosure {
    /// Every consequence disclosure, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NamedBlastRadius,
        Self::RollbackAvailable,
        Self::RollbackUnavailableStated,
        Self::HelpPathPresent,
        Self::ExplicitNamedActions,
        Self::GenericYesNoDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NamedBlastRadius => "named_blast_radius",
            Self::RollbackAvailable => "rollback_available",
            Self::RollbackUnavailableStated => "rollback_unavailable_stated",
            Self::HelpPathPresent => "help_path_present",
            Self::ExplicitNamedActions => "explicit_named_actions",
            Self::GenericYesNoDisallowed => "generic_yes_no_disallowed",
        }
    }
}

/// Claimed M5 surface family that renders / consumes a decision-feedback primitive. No primitive may
/// invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionFeedbackSurfaceFamily {
    /// The shell surface.
    Shell,
    /// The start-center entry surface.
    Entry,
    /// The trust surface.
    Trust,
    /// The review surface.
    Review,
    /// The repair surface.
    Repair,
    /// The notification surface.
    Notification,
    /// The support export.
    SupportExport,
}

impl M5DecisionFeedbackSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Shell,
        Self::Entry,
        Self::Trust,
        Self::Review,
        Self::Repair,
        Self::Notification,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Entry => "entry",
            Self::Trust => "trust",
            Self::Review => "review",
            Self::Repair => "repair",
            Self::Notification => "notification",
            Self::SupportExport => "support_export",
        }
    }
}

/// Deployment line a primitive must survive with the same truth, so a primitive's state, rationale,
/// scope, or recovery truth never silently narrows or widens between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionFeedbackDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5DecisionFeedbackDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Subsystem that consumes a primitive's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionFeedbackConsumerSurface {
    /// The shell UI.
    ShellUi,
    /// The help UI.
    HelpUi,
    /// The support UI.
    SupportUi,
    /// The review UI.
    ReviewUi,
    /// The settings UI.
    SettingsUi,
    /// The updates UI.
    UpdatesUi,
    /// The CLI / export path.
    CliExport,
    /// The support export.
    SupportExport,
    /// The general product UI.
    ProductUi,
}

impl M5DecisionFeedbackConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ShellUi,
        Self::HelpUi,
        Self::SupportUi,
        Self::ReviewUi,
        Self::SettingsUi,
        Self::UpdatesUi,
        Self::CliExport,
        Self::SupportExport,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellUi => "shell_ui",
            Self::HelpUi => "help_ui",
            Self::SupportUi => "support_ui",
            Self::ReviewUi => "review_ui",
            Self::SettingsUi => "settings_ui",
            Self::UpdatesUi => "updates_ui",
            Self::CliExport => "cli_export",
            Self::SupportExport => "support_export",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every primitive must offer so no state, rationale, scope, or
/// recovery truth is hover-only, pointer-only, motion-only, or visually encoded alone. Records the
/// keyboard, screen-reader, high-zoom, reduced-motion, CLI/export, and support-packet requirements up
/// front.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionFeedbackAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Legible and usable with reduced motion.
    ReducedMotionSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never renderer-only.
    SupportPacketPresent,
}

impl M5DecisionFeedbackAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::ReducedMotionSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::ReducedMotionSafe => "reduced_motion_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason a decision-feedback primitive has degraded below its qualified state. Required on every row so
/// a stale, unresolved, or narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionFeedbackDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// The rationale source is unavailable.
    RationaleSourceUnavailable,
    /// The scope signal is unavailable.
    ScopeSignalUnavailable,
    /// The durability mirror (e.g. activity center) is unavailable.
    DurabilityMirrorUnavailable,
    /// The action / recovery route is unavailable.
    ActionRouteUnavailable,
    /// The state signal is unavailable.
    StateSignalUnavailable,
}

impl M5DecisionFeedbackDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::RationaleSourceUnavailable,
        Self::ScopeSignalUnavailable,
        Self::DurabilityMirrorUnavailable,
        Self::ActionRouteUnavailable,
        Self::StateSignalUnavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::RationaleSourceUnavailable => "rationale_source_unavailable",
            Self::ScopeSignalUnavailable => "scope_signal_unavailable",
            Self::DurabilityMirrorUnavailable => "durability_mirror_unavailable",
            Self::ActionRouteUnavailable => "action_route_unavailable",
            Self::StateSignalUnavailable => "state_signal_unavailable",
        }
    }
}

/// Mandatory label a claimed decision-feedback primitive must be able to show. The first three are hard
/// requirements on every primitive; the remaining three close the acceptance-criteria ambiguity about
/// rationale, scope, and recovery-path labeling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionFeedbackRequiredLabel {
    /// The primitive's stable identity.
    Identity,
    /// The primitive's current typed state.
    State,
    /// The non-visual keyboard route to the primitive.
    KeyboardRoute,
    /// The rationale for why this primitive is shown / what it means.
    Rationale,
    /// The scope / blast radius the primitive applies to.
    Scope,
    /// The recovery path (next action, rollback, dismissal, or help) behind the primitive.
    RecoveryPath,
}

impl M5DecisionFeedbackRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::Rationale,
        Self::Scope,
        Self::RecoveryPath,
    ];

    /// The three labels every claimed primitive must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::Rationale => "rationale",
            Self::Scope => "scope",
            Self::RecoveryPath => "recovery_path",
        }
    }
}

/// Qualification class for an M5 decision-feedback row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionFeedbackQualificationClass {
    /// Primitive qualifies for the Stable claim.
    Stable,
    /// Primitive is narrowed to Beta.
    Beta,
    /// Primitive is narrowed to Preview.
    Preview,
    /// Primitive is experimental and not claimed.
    Experimental,
    /// Primitive is unavailable on this build.
    Unavailable,
    /// Primitive is held pending upstream resolution.
    Held,
}

impl M5DecisionFeedbackQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the primitive may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a decision-feedback primitive below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionFeedbackDowngradeTrigger {
    /// Color alone was used to convey meaning.
    ColorAloneUsedForMeaning,
    /// A popover carried the only critical workflow instruction.
    PopoverCarriedOnlyCriticalInstruction,
    /// Generic Yes/No copy was used in a high-risk dialog.
    GenericYesNoUsedInHighRiskDialog,
    /// Long-running or reviewable work was shown as toast-only truth.
    DurableWorkShownAsToastOnly,
    /// A useful pane was blanked during loading.
    UsefulPaneBlankedDuringLoading,
    /// A full-screen spinner was used where partial capability exists.
    FullScreenSpinnerWhenPartialCapable,
    /// A primitive left its rationale unstated.
    RationaleUnstated,
    /// A primitive left its scope unstated.
    ScopeUnstated,
    /// A primitive left its recovery path unstated.
    RecoveryPathUnstated,
    /// A primitive drifted from the shared state taxonomy.
    StateTaxonomyDrifted,
    /// Generic chrome wording concealed primitive truth.
    GenericChromeWordingUsed,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5DecisionFeedbackDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ColorAloneUsedForMeaning,
        Self::PopoverCarriedOnlyCriticalInstruction,
        Self::GenericYesNoUsedInHighRiskDialog,
        Self::DurableWorkShownAsToastOnly,
        Self::UsefulPaneBlankedDuringLoading,
        Self::FullScreenSpinnerWhenPartialCapable,
        Self::RationaleUnstated,
        Self::ScopeUnstated,
        Self::RecoveryPathUnstated,
        Self::StateTaxonomyDrifted,
        Self::GenericChromeWordingUsed,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ColorAloneUsedForMeaning => "color_alone_used_for_meaning",
            Self::PopoverCarriedOnlyCriticalInstruction => {
                "popover_carried_only_critical_instruction"
            }
            Self::GenericYesNoUsedInHighRiskDialog => "generic_yes_no_used_in_high_risk_dialog",
            Self::DurableWorkShownAsToastOnly => "durable_work_shown_as_toast_only",
            Self::UsefulPaneBlankedDuringLoading => "useful_pane_blanked_during_loading",
            Self::FullScreenSpinnerWhenPartialCapable => "full_screen_spinner_when_partial_capable",
            Self::RationaleUnstated => "rationale_unstated",
            Self::ScopeUnstated => "scope_unstated",
            Self::RecoveryPathUnstated => "recovery_path_unstated",
            Self::StateTaxonomyDrifted => "state_taxonomy_drifted",
            Self::GenericChromeWordingUsed => "generic_chrome_wording_used",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed decision-feedback family bound to the surface-specific truth it
/// must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DecisionFeedbackComponentRow {
    /// Governed primitive family.
    pub component_family: M5DecisionFeedbackFamily,
    /// Qualification class earned by this primitive.
    pub qualification: M5DecisionFeedbackQualificationClass,
    /// Owner role accountable for keeping this primitive governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this primitive.
    pub surface_families: Vec<M5DecisionFeedbackSurfaceFamily>,
    /// Deployment lines this primitive keeps the same truth across.
    pub deployment_lines: Vec<M5DecisionFeedbackDeploymentLine>,
    /// Mandatory labels this primitive must be able to show (must include the three
    /// [`M5DecisionFeedbackRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5DecisionFeedbackRequiredLabel>,
    /// State dispositions this primitive can carry (the frozen AC vocabulary; required on every
    /// primitive).
    pub dispositions: Vec<M5DecisionFeedbackDisposition>,
    /// Badge expressions this primitive names (badge-chip-pill family only).
    pub badge_expressions: Vec<M5BadgeExpression>,
    /// Popover dismissals this primitive names (popover family only).
    pub popover_dismissals: Vec<M5PopoverDismissal>,
    /// Dialog action models this primitive names (dialog-sheet family only).
    pub dialog_action_models: Vec<M5DialogActionModel>,
    /// Notice scopes this primitive names (banner-inline-notice family only).
    pub notice_scopes: Vec<M5NoticeScope>,
    /// Toast durabilities this primitive names (toast family only).
    pub toast_durabilities: Vec<M5ToastDurability>,
    /// Empty-state purposes this primitive names (empty-state family only).
    pub empty_state_purposes: Vec<M5EmptyStatePurpose>,
    /// Loading fidelities this primitive names (loading-state family only).
    pub loading_fidelities: Vec<M5LoadingFidelity>,
    /// Consequence disclosures this primitive names (consequence-block family only).
    pub consequence_disclosures: Vec<M5ConsequenceDisclosure>,
    /// Degraded reasons this primitive can name (required on every primitive).
    pub degraded_reasons: Vec<M5DecisionFeedbackDegradedReason>,
    /// Non-visual accessibility routes this primitive offers.
    pub accessibility_routes: Vec<M5DecisionFeedbackAccessibilityRoute>,
    /// Subsystems that consume this primitive's projection.
    pub consumer_surfaces: Vec<M5DecisionFeedbackConsumerSurface>,
    /// Downgrade triggers that apply to this primitive.
    pub downgrade_triggers: Vec<M5DecisionFeedbackDowngradeTrigger>,
    /// Proof packet refs that keep this primitive current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this primitive (must include its own canonical component schema
    /// so downstream primitives have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this primitive never relies on color alone to convey meaning. MUST be `false`.
    pub relies_on_color_alone_for_meaning: bool,
    /// Hard invariant: this primitive never lets a popover carry the only critical workflow instruction.
    /// MUST be `false`.
    pub lets_popover_carry_only_critical_instruction: bool,
    /// Hard invariant: this primitive never uses generic Yes/No copy in a high-risk dialog. MUST be
    /// `false`.
    pub uses_generic_yes_no_in_high_risk_dialog: bool,
    /// Hard invariant: this primitive never represents long-running or reviewable work as toast-only
    /// truth. MUST be `false`.
    pub represents_durable_work_as_toast_only: bool,
    /// Hard invariant: this primitive never blanks a useful pane during loading. MUST be `false`.
    pub blanks_useful_pane_during_loading: bool,
    /// Hard invariant: this primitive never uses a full-screen spinner where partial capability exists.
    /// MUST be `false`.
    pub uses_full_screen_spinner_when_partial_capable: bool,
}

impl M5DecisionFeedbackComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5DecisionFeedbackRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5DecisionFeedbackRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.relies_on_color_alone_for_meaning
            && !self.lets_popover_carry_only_critical_instruction
            && !self.uses_generic_yes_no_in_high_risk_dialog
            && !self.represents_durable_work_as_toast_only
            && !self.blanks_useful_pane_during_loading
            && !self.uses_full_screen_spinner_when_partial_capable
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DecisionFeedbackVocabularySet {
    /// Primitive-family tokens.
    pub component_families: Vec<String>,
    /// State disposition tokens.
    pub dispositions: Vec<String>,
    /// Badge-expression tokens.
    pub badge_expressions: Vec<String>,
    /// Popover-dismissal tokens.
    pub popover_dismissals: Vec<String>,
    /// Dialog-action-model tokens.
    pub dialog_action_models: Vec<String>,
    /// Notice-scope tokens.
    pub notice_scopes: Vec<String>,
    /// Toast-durability tokens.
    pub toast_durabilities: Vec<String>,
    /// Empty-state-purpose tokens.
    pub empty_state_purposes: Vec<String>,
    /// Loading-fidelity tokens.
    pub loading_fidelities: Vec<String>,
    /// Consequence-disclosure tokens.
    pub consequence_disclosures: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded-reason tokens.
    pub degraded_reasons: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
    /// Downgrade-trigger tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5DecisionFeedbackVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5DecisionFeedbackFamily::ALL, |v| v.as_str()),
            dispositions: tokens(&M5DecisionFeedbackDisposition::ALL, |v| v.as_str()),
            badge_expressions: tokens(&M5BadgeExpression::ALL, |v| v.as_str()),
            popover_dismissals: tokens(&M5PopoverDismissal::ALL, |v| v.as_str()),
            dialog_action_models: tokens(&M5DialogActionModel::ALL, |v| v.as_str()),
            notice_scopes: tokens(&M5NoticeScope::ALL, |v| v.as_str()),
            toast_durabilities: tokens(&M5ToastDurability::ALL, |v| v.as_str()),
            empty_state_purposes: tokens(&M5EmptyStatePurpose::ALL, |v| v.as_str()),
            loading_fidelities: tokens(&M5LoadingFidelity::ALL, |v| v.as_str()),
            consequence_disclosures: tokens(&M5ConsequenceDisclosure::ALL, |v| v.as_str()),
            surface_families: tokens(&M5DecisionFeedbackSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5DecisionFeedbackDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5DecisionFeedbackConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5DecisionFeedbackAccessibilityRoute::ALL, |v| {
                v.as_str()
            }),
            degraded_reasons: tokens(&M5DecisionFeedbackDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5DecisionFeedbackRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5DecisionFeedbackDowngradeTrigger::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DecisionFeedbackGovernanceReview {
    /// Badge / chip / pill meaning is never conveyed by color alone.
    pub badge_meaning_never_color_alone: bool,
    /// Popovers never carry the only critical workflow instruction.
    pub popover_never_carries_only_critical_instruction: bool,
    /// Dialogs / sheets name rationale, scope, and explicit actions.
    pub dialog_names_rationale_scope_and_explicit_actions: bool,
    /// Banners and inline notices stay scoped and actionable.
    pub banner_and_inline_notice_stay_scoped_and_actionable: bool,
    /// Toasts never become the only durable truth.
    pub toast_never_the_only_durable_truth: bool,
    /// Empty states explain purpose, emptiness, and next action.
    pub empty_state_explains_purpose_emptiness_and_next_action: bool,
    /// Loading states preserve useful partial data.
    pub loading_state_preserves_useful_partial_data: bool,
    /// Consequence blocks name blast radius and rollback / help posture.
    pub consequence_block_names_blast_radius_and_rollback_posture: bool,
    /// The state taxonomy means the same thing everywhere.
    pub state_taxonomy_means_the_same_everywhere: bool,
    /// No generic Yes/No copy in high-risk confirmation.
    pub no_generic_yes_no_in_high_risk_confirmation: bool,
    /// No full-screen spinner where partial capability exists.
    pub no_full_screen_spinner_where_partial_capable: bool,
    /// Blocked and degraded semantics are never hidden behind generic chrome.
    pub blocked_and_degraded_never_hidden_behind_generic_chrome: bool,
    /// Every primitive binds back to one rationale or recovery path.
    pub every_primitive_binds_to_one_rationale_or_recovery_path: bool,
    /// Every primitive keeps the same truth across every deployment line.
    pub every_primitive_declares_deployment_lines: bool,
    /// Every primitive declares a non-visual accessibility route.
    pub every_primitive_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel feedback vocabulary.
    pub later_rows_cannot_invent_parallel_feedback_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DecisionFeedbackConsumerProjection {
    /// Shell and notification consume the shared feedback vocabulary.
    pub shell_and_notification_consume_shared_feedback_vocabulary: bool,
    /// Entry and trust consume the shared decision vocabulary.
    pub entry_and_trust_consume_shared_decision_vocabulary: bool,
    /// Review consumes the shared decision and feedback vocabulary.
    pub review_consumes_shared_decision_and_feedback_vocabulary: bool,
    /// Repair consumes the shared consequence vocabulary.
    pub repair_consumes_shared_consequence_vocabulary: bool,
    /// Help and updates consume the shared state vocabulary.
    pub help_and_updates_consume_shared_state_vocabulary: bool,
    /// Support / export reads a single canonical feedback source.
    pub support_export_reads_single_feedback_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DecisionFeedbackProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the decision-feedback lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DecisionFeedbackReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting decision-feedback audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every primitive.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every primitive.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DecisionFeedbackComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DecisionFeedbackComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Primitive rows.
    pub component_rows: Vec<M5DecisionFeedbackComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DecisionFeedbackVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DecisionFeedbackGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DecisionFeedbackConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DecisionFeedbackProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DecisionFeedbackReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 decision-feedback component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DecisionFeedbackComponentMatrixPacket {
    /// Record kind; must equal [`M5_DECISION_FEEDBACK_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DECISION_FEEDBACK_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Primitive rows.
    pub component_rows: Vec<M5DecisionFeedbackComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DecisionFeedbackVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DecisionFeedbackGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DecisionFeedbackConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DecisionFeedbackProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DecisionFeedbackReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DecisionFeedbackComponentMatrixPacket {
    /// Builds an M5 decision-feedback component matrix packet from stable-lane input.
    pub fn new(input: M5DecisionFeedbackComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_DECISION_FEEDBACK_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_DECISION_FEEDBACK_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 decision-feedback component matrix invariants.
    pub fn validate(&self) -> Vec<M5DecisionFeedbackComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DECISION_FEEDBACK_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DECISION_FEEDBACK_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 decision-feedback component matrix serializes"),
        ) {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::RawMaterialInExport);
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
            .expect("m5 decision-feedback component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed primitive.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,canonical_schema,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.component_family.canonical_component_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Badge-Chip-Pill, Popover, Dialog-Sheet, Banner-Inline-Notice, Toast, Empty-State, Loading-State, and Consequence-Block Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Primitive families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Feedback states: {}\n",
            self.vocabulary_set.dispositions.join(", ")
        ));
        out.push_str(&format!(
            "- Badge expressions: {}\n",
            self.vocabulary_set.badge_expressions.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Primitive families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.component_family.canonical_component_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 decision-feedback matrix export.
#[derive(Debug)]
pub enum M5DecisionFeedbackComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DecisionFeedbackComponentMatrixViolation>),
}

impl fmt::Display for M5DecisionFeedbackComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 decision-feedback component matrix export parse failed: {error}"
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
                    "m5 decision-feedback component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DecisionFeedbackComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5DecisionFeedbackComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DecisionFeedbackComponentMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed primitive family is missing from the matrix.
    RequiredComponentMissing,
    /// A primitive row is incomplete.
    ComponentRowIncomplete,
    /// A primitive row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A primitive row does not point at its own canonical component schema.
    ComponentSchemaRefMissing,
    /// A primitive declares no state dispositions.
    DispositionMissing,
    /// The badge-chip-pill primitive declares no badge expressions.
    BadgeExpressionMissing,
    /// The popover primitive declares no popover dismissals.
    PopoverDismissalMissing,
    /// The dialog-sheet primitive declares no dialog action models.
    DialogActionModelMissing,
    /// The banner-inline-notice primitive declares no notice scopes.
    NoticeScopeMissing,
    /// The toast primitive declares no toast durabilities.
    ToastDurabilityMissing,
    /// The empty-state primitive declares no empty-state purposes.
    EmptyStatePurposeMissing,
    /// The loading-state primitive declares no loading fidelities.
    LoadingFidelityMissing,
    /// The consequence-block primitive declares no consequence disclosures.
    ConsequenceDisclosureMissing,
    /// A primitive declares no degraded reasons.
    DegradedReasonMissing,
    /// A primitive declares no surface families.
    SurfaceFamilyMissing,
    /// A primitive declares no deployment lines.
    DeploymentLineMissing,
    /// A primitive declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A primitive declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A primitive declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A primitive claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A primitive violates a hard invariant (color-alone meaning, popover carrying the only critical
    /// instruction, generic Yes/No in a high-risk dialog, durable work shown as toast-only, a useful
    /// pane blanked during loading, or a full-screen spinner where partial capability exists).
    ComponentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5DecisionFeedbackComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::DispositionMissing => "disposition_missing",
            Self::BadgeExpressionMissing => "badge_expression_missing",
            Self::PopoverDismissalMissing => "popover_dismissal_missing",
            Self::DialogActionModelMissing => "dialog_action_model_missing",
            Self::NoticeScopeMissing => "notice_scope_missing",
            Self::ToastDurabilityMissing => "toast_durability_missing",
            Self::EmptyStatePurposeMissing => "empty_state_purpose_missing",
            Self::LoadingFidelityMissing => "loading_fidelity_missing",
            Self::ConsequenceDisclosureMissing => "consequence_disclosure_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 decision-feedback matrix export.
pub fn current_stable_m5_decision_feedback_component_matrix_export(
) -> Result<M5DecisionFeedbackComponentMatrixPacket, M5DecisionFeedbackComponentMatrixArtifactError>
{
    let packet: M5DecisionFeedbackComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-decision-feedback-proof/support_export.json"
        )))
        .map_err(M5DecisionFeedbackComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DecisionFeedbackComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5DecisionFeedbackComponentMatrixPacket,
    violations: &mut Vec<M5DecisionFeedbackComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF,
        M5_DECISION_FEEDBACK_COMPONENT_DOC_REF,
        M5_BADGE_CHIP_PILL_SCHEMA_REF,
        M5_POPOVER_SCHEMA_REF,
        M5_DIALOG_SHEET_SCHEMA_REF,
        M5_BANNER_INLINE_NOTICE_SCHEMA_REF,
        M5_TOAST_SCHEMA_REF,
        M5_EMPTY_STATE_SCHEMA_REF,
        M5_LOADING_STATE_SCHEMA_REF,
        M5_CONSEQUENCE_BLOCK_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5DecisionFeedbackComponentMatrixPacket,
    violations: &mut Vec<M5DecisionFeedbackComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5DecisionFeedbackComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5DecisionFeedbackComponentMatrixPacket,
    violations: &mut Vec<M5DecisionFeedbackComponentMatrixViolation>,
) {
    let present: BTreeSet<M5DecisionFeedbackFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5DecisionFeedbackFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_component_schema_ref())
        {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::ComponentSchemaRefMissing);
        }
        if row.dispositions.is_empty() {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::DispositionMissing);
        }
        if family.declares_badge_expression() && row.badge_expressions.is_empty() {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::BadgeExpressionMissing);
        }
        if family.declares_popover_dismissal() && row.popover_dismissals.is_empty() {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::PopoverDismissalMissing);
        }
        if family.declares_dialog_action_model() && row.dialog_action_models.is_empty() {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::DialogActionModelMissing);
        }
        if family.declares_notice_scope() && row.notice_scopes.is_empty() {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::NoticeScopeMissing);
        }
        if family.declares_toast_durability() && row.toast_durabilities.is_empty() {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::ToastDurabilityMissing);
        }
        if family.declares_empty_state_purpose() && row.empty_state_purposes.is_empty() {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::EmptyStatePurposeMissing);
        }
        if family.declares_loading_fidelity() && row.loading_fidelities.is_empty() {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::LoadingFidelityMissing);
        }
        if family.declares_consequence_disclosure() && row.consequence_disclosures.is_empty() {
            violations
                .push(M5DecisionFeedbackComponentMatrixViolation::ConsequenceDisclosureMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations
                .push(M5DecisionFeedbackComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5DecisionFeedbackComponentMatrixPacket,
    violations: &mut Vec<M5DecisionFeedbackComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.badge_meaning_never_color_alone,
        review.popover_never_carries_only_critical_instruction,
        review.dialog_names_rationale_scope_and_explicit_actions,
        review.banner_and_inline_notice_stay_scoped_and_actionable,
        review.toast_never_the_only_durable_truth,
        review.empty_state_explains_purpose_emptiness_and_next_action,
        review.loading_state_preserves_useful_partial_data,
        review.consequence_block_names_blast_radius_and_rollback_posture,
        review.state_taxonomy_means_the_same_everywhere,
        review.no_generic_yes_no_in_high_risk_confirmation,
        review.no_full_screen_spinner_where_partial_capable,
        review.blocked_and_degraded_never_hidden_behind_generic_chrome,
        review.every_primitive_binds_to_one_rationale_or_recovery_path,
        review.every_primitive_declares_deployment_lines,
        review.every_primitive_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_feedback_vocabulary,
    ] {
        if !ok {
            violations.push(M5DecisionFeedbackComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DecisionFeedbackComponentMatrixPacket,
    violations: &mut Vec<M5DecisionFeedbackComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_and_notification_consume_shared_feedback_vocabulary,
        projection.entry_and_trust_consume_shared_decision_vocabulary,
        projection.review_consumes_shared_decision_and_feedback_vocabulary,
        projection.repair_consumes_shared_consequence_vocabulary,
        projection.help_and_updates_consume_shared_state_vocabulary,
        projection.support_export_reads_single_feedback_source,
    ] {
        if !ok {
            violations
                .push(M5DecisionFeedbackComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DecisionFeedbackComponentMatrixPacket,
    violations: &mut Vec<M5DecisionFeedbackComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DecisionFeedbackComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DecisionFeedbackComponentMatrixPacket,
    violations: &mut Vec<M5DecisionFeedbackComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DecisionFeedbackComponentMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled
/// vocabulary deliberately uses feedback / decision words; what is rejected is a raw secret *value*
/// shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
