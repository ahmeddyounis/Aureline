//! Two reusable M5 notebook components — the restart consequence card and the kernel recovery card
//! — so a user can tell, before they commit to restarting or reconnecting a kernel, what survives
//! and what must be recomputed, and how a kernel that failed, disconnected, or was intentionally
//! restarted can be recovered: the restart consequence card names a restart / interrupt / shutdown
//! action, what state it preserves (notebook source, prior outputs) and what it loses (live
//! variables, debugger frames, session), and whether a rerun is required to recompute the lost
//! state — and never implies that a rerun already ran; the kernel recovery card names where a
//! kernel's recovery stands (recoverable, reconnect available, restart required, no kernel
//! available, blocked, or recovered) and offers reconnect / restart-clean / choose-another-kernel /
//! open-inspect-only / export-evidence recovery — and never implies that code or cells were
//! silently executed during restore or repair.
//!
//! Aureline's frozen notebook-kernel-output component matrix
//! ([`crate::freeze_the_m5_notebook_document_header_kernel_state_strip_kernel_picker_row_kernel_origin_pill_output_trust_banner_output_provenance_chip_group_restart_consequence_card_and_kernel_recovery_card_component_matrix`])
//! names the restart consequence card and the kernel recovery card as two governed component
//! families and freezes their controlled vocabulary — the restart action classes (`restart_kernel`,
//! `restart_and_run_all`, `interrupt_kernel`, `shutdown_kernel`, `reconnect_kernel`,
//! `clear_outputs`) and restart consequence states (`state_preserved`, `state_lost`,
//! `variables_cleared`, `outputs_retained`, `outputs_cleared`, `no_consequence`) a restart card
//! binds; the kernel recovery action classes (`reconnect`, `restart_clean`, `choose_another_kernel`,
//! `reattach_session`, `start_local_fallback`, `wait_for_managed`) and kernel recovery states
//! (`recoverable`, `reconnect_available`, `restart_required`, `no_kernel_available`,
//! `recovery_blocked`, `recovered`) a recovery card binds; the one controlled disposition
//! vocabulary; the surface families; the deployment lines; the consumer surfaces; the accessibility
//! routes; the required labels; and the downgrade triggers. This module *implements* that contract
//! as two co-equal component vectors so a claimed M5 notebook, debug-bridge, review, support, or
//! companion-handoff surface can project a restart card and a recovery card that keep the same
//! truth.
//!
//! The module has two derived resolvers:
//!
//! 1. [`resolve_restart_consequence_card`] — takes a card's restart action class and consequence
//!    state and derives its restart impact class (state preserved, live state lost, variables
//!    cleared, outputs retained, outputs cleared, or no impact), its action scope (ends the session,
//!    keeps the session, or clears outputs only), whether state is preserved, whether a rerun is
//!    required, whether the debugger / session is affected, whether the card may claim state was
//!    preserved, and which notes it must carry — so a restart that loses live state never reads as
//!    one that preserved it, and the rerun a user must run to recompute lost state is always named
//!    before restart and never implied to have already happened.
//! 2. [`resolve_kernel_recovery_card`] — takes a card's kernel recovery action class and recovery
//!    state and derives its recovery posture (recoverable, reconnect offered, restart needed, no
//!    kernel, blocked, or recovered), its continuity class (continues the session, a clean new
//!    session, or awaits a managed kernel), whether the kernel is recovered, whether continuity is
//!    preserved, whether a rerun is required after recovery, and which notes it must carry — so a
//!    recovery that started a clean session never reads as recovered live state, and no recovery
//!    ever implies that code or cells were silently executed during restore or repair.
//!
//! A single controls packet — [`RestartConsequenceCardKernelRecoveryCardControlsPacket`] — binds one
//! vector of restart consequence cards and one vector of kernel recovery cards to the same restart,
//! consequence, recovery, continuity, deep-link, and non-visual accessibility vocabulary, so restart
//! consequence truth and kernel recovery truth stay distinct and explicit across notebook, debug,
//! review, headless / export, support, and companion-handoff consumers.
//!
//! The component family ([`M5NotebookKernelOutputComponentFamily`]), restart action class
//! ([`M5RestartActionClass`]), restart consequence state ([`M5RestartConsequenceState`]), kernel
//! recovery action class ([`M5KernelRecoveryActionClass`]), kernel recovery state
//! ([`M5KernelRecoveryState`]), disposition ([`M5NotebookKernelOutputDisposition`]), surface family
//! ([`M5NotebookKernelOutputSurfaceFamily`]), deployment line
//! ([`M5NotebookKernelOutputDeploymentLine`]), consumer surface
//! ([`M5NotebookKernelOutputConsumerSurface`]), accessibility route
//! ([`M5NotebookKernelOutputAccessibilityRoute`]), required label
//! ([`M5NotebookKernelOutputRequiredLabel`]), and downgrade trigger
//! ([`M5NotebookKernelOutputDowngradeTrigger`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the two components
//! themselves: the derived restart impact and recovery posture classes, the action scope and
//! continuity classes, the bounded restart-card and recovery-card actions, and the deep-link kinds.
//! No M5 notebook surface invents a second restart-consequence or kernel-recovery grammar.
//!
//! Raw notebook payloads, pasted paths, credentials, and private endpoints stay outside the export
//! boundary; every context line, deep-link reference, and component identity is carried only as an
//! opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_restart_consequence_card_kernel_recovery_card_controls,
    seeded_restart_consequence_card_kernel_recovery_card_controls_kernel_recovery_card_clean_session,
    seeded_restart_consequence_card_kernel_recovery_card_controls_restart_consequence_card_lost_state,
    RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_PACKET_ID,
};

// The restart action classes and consequence states, the kernel recovery action classes and states,
// the disposition vocabulary, and the surface / deployment / consumer / accessibility / label /
// downgrade vocabularies are frozen once, in the notebook-kernel-output component matrix. This lane
// reuses them verbatim so it never invents a parallel restart-consequence or kernel-recovery
// vocabulary.
pub use crate::freeze_the_m5_notebook_document_header_kernel_state_strip_kernel_picker_row_kernel_origin_pill_output_trust_banner_output_provenance_chip_group_restart_consequence_card_and_kernel_recovery_card_component_matrix::{
    M5KernelRecoveryActionClass, M5KernelRecoveryState, M5NotebookKernelOutputAccessibilityRoute,
    M5NotebookKernelOutputComponentFamily, M5NotebookKernelOutputConsumerSurface,
    M5NotebookKernelOutputDeploymentLine, M5NotebookKernelOutputDisposition,
    M5NotebookKernelOutputDowngradeTrigger, M5NotebookKernelOutputRequiredLabel,
    M5NotebookKernelOutputSurfaceFamily, M5RestartActionClass, M5RestartConsequenceState,
    M5_KERNEL_RECOVERY_CARD_SCHEMA_REF, M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_DOC_REF,
    M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF, M5_RESTART_CONSEQUENCE_CARD_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`RestartConsequenceCardKernelRecoveryCardControlsPacket`].
pub const RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_RECORD_KIND: &str =
    "implement_restart_consequence_cards_and_kernel_recovery_cards_with_preserved_state_lost_state_reconnect_restart_clean_choose_another_kernel_actions_and_no_hidden_rerun_truth_across_claimed_m5_notebook_restore_and_failure_flows";

/// Schema version for M5 restart-consequence-card / kernel-recovery-card control records.
pub const RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-restart-consequence-card-kernel-recovery-card-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_DOC_REF: &str =
    "docs/notebooks/m5_restart_consequence_card_kernel_recovery_card_controls.md";

/// Repo-relative path of the protected fixture directory.
pub const RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_FIXTURE_DIR: &str =
    "fixtures/ui/m5-restart-consequence-card-kernel-recovery-card-controls";

/// Repo-relative path of the checked support-export artifact.
pub const RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_ARTIFACT_REF: &str =
    "artifacts/release/m5-restart-consequence-card-kernel-recovery-card-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_CSV_REF: &str =
    "artifacts/release/m5-restart-consequence-card-kernel-recovery-card-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_REPORT_REF: &str =
    "artifacts/design/m5-restart-consequence-card-kernel-recovery-card.md";

// ---- shared deep-link vocabulary ----------------------------------------

/// The kind of stable deep link a notebook component binds its next step against, so a restart
/// consequence card or kernel recovery card never routes through an ephemeral overlay — every next
/// step is a stable notebook / cell location, kernel-manager, docs, or support-bundle reference the
/// user can reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkKind {
    /// A stable notebook / cell location.
    NotebookLocation,
    /// A stable kernel-manager reference.
    KernelManager,
    /// A stable docs anchor.
    DocsAnchor,
    /// A stable support-bundle anchor.
    SupportBundle,
    /// No deep link is bound (the component names that it routes nowhere).
    NoDeepLink,
}

impl DeepLinkKind {
    /// Every deep-link kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NotebookLocation,
        Self::KernelManager,
        Self::DocsAnchor,
        Self::SupportBundle,
        Self::NoDeepLink,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookLocation => "notebook_location",
            Self::KernelManager => "kernel_manager",
            Self::DocsAnchor => "docs_anchor",
            Self::SupportBundle => "support_bundle",
            Self::NoDeepLink => "no_deep_link",
        }
    }

    /// True when this kind names a resolvable deep-link target.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoDeepLink)
    }
}

// ---- restart-consequence-card vocabulary --------------------------------

/// Derived restart impact class a restart consequence card may present.
///
/// This is the restart honesty axis: the class is derived from the frozen restart consequence
/// state, never asserted, so a restart that loses live state can never read as one that preserved
/// it and a user can always tell what survives (notebook source, prior outputs) and what is lost
/// (live variables, debugger frames, session) before they commit to a restart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartImpactClass {
    /// Kernel state is preserved across the action.
    StatePreservedImpact,
    /// Live kernel state is lost.
    LiveStateLostImpact,
    /// Variables are cleared.
    VariablesClearedImpact,
    /// Outputs are retained (the notebook document keeps them).
    OutputsRetainedImpact,
    /// Outputs are cleared.
    OutputsClearedImpact,
    /// The action has no material consequence.
    NoRestartImpact,
}

impl RestartImpactClass {
    /// Every restart impact class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StatePreservedImpact,
        Self::LiveStateLostImpact,
        Self::VariablesClearedImpact,
        Self::OutputsRetainedImpact,
        Self::OutputsClearedImpact,
        Self::NoRestartImpact,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StatePreservedImpact => "state_preserved_impact",
            Self::LiveStateLostImpact => "live_state_lost_impact",
            Self::VariablesClearedImpact => "variables_cleared_impact",
            Self::OutputsRetainedImpact => "outputs_retained_impact",
            Self::OutputsClearedImpact => "outputs_cleared_impact",
            Self::NoRestartImpact => "no_restart_impact",
        }
    }

    /// True when the impact preserves live kernel state.
    pub const fn preserves_state(self) -> bool {
        matches!(
            self,
            Self::StatePreservedImpact | Self::OutputsRetainedImpact | Self::NoRestartImpact
        )
    }

    /// True when the impact loses live kernel state (variables / session).
    pub const fn loses_live_state(self) -> bool {
        matches!(
            self,
            Self::LiveStateLostImpact | Self::VariablesClearedImpact
        )
    }

    /// True when the impact clears rendered outputs.
    pub const fn clears_outputs(self) -> bool {
        matches!(self, Self::OutputsClearedImpact)
    }
}

/// Derived restart action scope — what a restart action does to the running session, so a card
/// never leaves whether an action ends the session, keeps it, or only clears outputs implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartActionScope {
    /// The action ends the running session (restart / restart-and-run-all / shutdown).
    EndsSession,
    /// The action keeps the running session (interrupt / reconnect).
    KeepsSession,
    /// The action only clears outputs and does not touch the session.
    OutputsOnly,
}

impl RestartActionScope {
    /// Every restart action scope, in declaration order.
    pub const ALL: [Self; 3] = [Self::EndsSession, Self::KeepsSession, Self::OutputsOnly];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EndsSession => "ends_session",
            Self::KeepsSession => "keeps_session",
            Self::OutputsOnly => "outputs_only",
        }
    }

    /// True when the action ends the session (debugger frames / session are affected).
    pub const fn ends_session(self) -> bool {
        matches!(self, Self::EndsSession)
    }
}

/// One keyboard-complete default action a restart consequence card offers, so a card never hides its
/// review / confirm / cancel affordance behind a pointer-only gesture. `ReviewConsequences`,
/// `ConfirmRestart`, and `CancelRestart` are always offered so a user can always read what survives
/// and what is lost before they commit to a restart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartCardAction {
    /// Review the preserved and lost state before restarting (always available).
    ReviewConsequences,
    /// Confirm and commit to the restart (always available).
    ConfirmRestart,
    /// Cancel the restart (always available).
    CancelRestart,
    /// Export the restart-consequence evidence.
    ExportEvidence,
    /// Interrupt instead of restarting (keep the session).
    InterruptInstead,
    /// Open the stable notebook / kernel-manager / docs / support deep link.
    OpenDeepLink,
}

impl RestartCardAction {
    /// Every restart-card action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReviewConsequences,
        Self::ConfirmRestart,
        Self::CancelRestart,
        Self::ExportEvidence,
        Self::InterruptInstead,
        Self::OpenDeepLink,
    ];

    /// The default actions every keyboard-complete restart card must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::ReviewConsequences,
        Self::ConfirmRestart,
        Self::CancelRestart,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewConsequences => "review_consequences",
            Self::ConfirmRestart => "confirm_restart",
            Self::CancelRestart => "cancel_restart",
            Self::ExportEvidence => "export_evidence",
            Self::InterruptInstead => "interrupt_instead",
            Self::OpenDeepLink => "open_deep_link",
        }
    }
}

/// Disclosures a restart consequence card must carry, derived from the restart action class and
/// consequence state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RestartConsequenceCardDisclosure {
    /// The derived restart impact class this card may present.
    pub impact_class: RestartImpactClass,
    /// The derived action scope this card may present.
    pub action_scope: RestartActionScope,
    /// Whether live kernel state is preserved.
    pub preserves_state: bool,
    /// Whether live kernel state is lost.
    pub loses_live_state: bool,
    /// Whether a rerun is required to recompute lost state.
    pub requires_rerun: bool,
    /// Whether the debugger / session is affected (the action ends the session).
    pub affects_debugger_session: bool,
    /// Whether the card may claim state was preserved.
    pub may_claim_state_preserved: bool,
    /// Whether the card must carry an explicit lost-state note.
    pub needs_lost_state_note: bool,
    /// Whether the card must carry an explicit variables-cleared note.
    pub needs_variables_cleared_note: bool,
    /// Whether the card must carry an explicit outputs-cleared note.
    pub needs_outputs_cleared_note: bool,
    /// Whether the card must carry an explicit rerun-requirement note.
    pub needs_rerun_note: bool,
    /// Whether the card must carry an explicit debugger / session note.
    pub needs_debugger_session_note: bool,
}

/// Resolves the impact and scope truth a restart consequence card may present.
///
/// A `state_preserved` consequence preserves state, a `state_lost` consequence loses live state, a
/// `variables_cleared` consequence clears variables, an `outputs_retained` consequence keeps the
/// prior outputs, an `outputs_cleared` consequence clears outputs, and a `no_consequence` action has
/// no material impact — so a restart that loses live state never reads as one that preserved it. A
/// `restart_kernel`, `restart_and_run_all`, or `shutdown_kernel` action ends the session (debugger
/// frames and session are affected), an `interrupt_kernel` or `reconnect_kernel` action keeps the
/// session, and a `clear_outputs` action only clears outputs. A card may claim state was preserved
/// only when the impact preserves state; a rerun is required whenever live state is lost or outputs
/// are cleared, and the card must name that rerun before restart — it never implies the rerun
/// already ran.
pub fn resolve_restart_consequence_card(
    action: M5RestartActionClass,
    consequence: M5RestartConsequenceState,
) -> RestartConsequenceCardDisclosure {
    use M5RestartActionClass as Act;
    use M5RestartConsequenceState as Cons;
    use RestartActionScope as Scope;
    use RestartImpactClass as Impact;

    let impact_class = match consequence {
        Cons::StatePreserved => Impact::StatePreservedImpact,
        Cons::StateLost => Impact::LiveStateLostImpact,
        Cons::VariablesCleared => Impact::VariablesClearedImpact,
        Cons::OutputsRetained => Impact::OutputsRetainedImpact,
        Cons::OutputsCleared => Impact::OutputsClearedImpact,
        Cons::NoConsequence => Impact::NoRestartImpact,
    };

    let action_scope = match action {
        Act::RestartKernel | Act::RestartAndRunAll | Act::ShutdownKernel => Scope::EndsSession,
        Act::InterruptKernel | Act::ReconnectKernel => Scope::KeepsSession,
        Act::ClearOutputs => Scope::OutputsOnly,
    };

    let requires_rerun = impact_class.loses_live_state() || impact_class.clears_outputs();

    RestartConsequenceCardDisclosure {
        impact_class,
        action_scope,
        preserves_state: impact_class.preserves_state(),
        loses_live_state: impact_class.loses_live_state(),
        requires_rerun,
        affects_debugger_session: action_scope.ends_session(),
        may_claim_state_preserved: impact_class.preserves_state(),
        needs_lost_state_note: impact_class.loses_live_state(),
        needs_variables_cleared_note: matches!(impact_class, Impact::VariablesClearedImpact),
        needs_outputs_cleared_note: matches!(impact_class, Impact::OutputsClearedImpact),
        needs_rerun_note: requires_rerun,
        needs_debugger_session_note: action_scope.ends_session(),
    }
}

/// A restart consequence card naming a restart / interrupt / shutdown action, what state it
/// preserves and loses, whether a rerun is required, its derived impact and scope, bounded review /
/// confirm / cancel actions, and a stable deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestartConsequenceCard {
    /// Frozen component this control implements; must be `restart_consequence_card`.
    pub component: M5NotebookKernelOutputComponentFamily,
    /// Stable card id.
    pub card_id: String,
    /// Human-readable card label; required and non-empty.
    pub card_label: String,
    /// Restart action class, reused from the frozen matrix.
    pub restart_action: M5RestartActionClass,
    /// Restart consequence state, reused from the frozen matrix.
    pub consequence_state: M5RestartConsequenceState,
    /// Derived restart impact class (must equal the resolved class).
    pub impact_class: RestartImpactClass,
    /// Derived action scope (must equal the resolved scope).
    pub action_scope: RestartActionScope,
    /// Whether the card claims live state was preserved. May be `true` only when the derived truth
    /// allows it.
    pub claims_state_preserved: bool,
    /// Whether the card claims a rerun is required (must equal the derived truth).
    pub requires_rerun: bool,
    /// Whether the card claims the debugger / session is affected (must equal the derived truth).
    pub affects_debugger_session: bool,
    /// Preserved-state label; always required so what survives (notebook source, prior outputs) is
    /// never left implicit.
    pub preserved_state_label: String,
    /// Lost-state note; required when live state is lost.
    pub lost_state_note: String,
    /// Variables-cleared note; required when variables are cleared.
    pub variables_cleared_note: String,
    /// Outputs-cleared note; required when outputs are cleared.
    pub outputs_cleared_note: String,
    /// Rerun-requirement note; required when a rerun is required. Names that a rerun is needed to
    /// recompute lost state — never that one already ran.
    pub rerun_requirement_note: String,
    /// Debugger / session note; required when the action ends the session.
    pub debugger_session_note: String,
    /// Restart action label; always required so the action is never hidden behind a hover-only
    /// affordance.
    pub restart_action_label: String,
    /// Consequence state label; always required so the preserved-versus-lost state stays explicit.
    pub consequence_state_label: String,
    /// Context note; always required so the card names what the restart truth means here.
    pub context_note: String,
    /// Kind of stable deep link this card binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include review / confirm / cancel).
    pub card_actions: Vec<RestartCardAction>,
    /// Dispositions this card binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5NotebookKernelOutputDisposition>,
    /// Downgrade triggers this card can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5NotebookKernelOutputDowngradeTrigger>,
    /// Mandatory labels this card can show (must include the mandatory labels).
    pub required_labels: Vec<M5NotebookKernelOutputRequiredLabel>,
    /// Claimed M5 surface families that render this card.
    pub surface_families: Vec<M5NotebookKernelOutputSurfaceFamily>,
    /// Deployment lines this card keeps the same truth across.
    pub deployment_lines: Vec<M5NotebookKernelOutputDeploymentLine>,
    /// Non-visual accessibility routes this card offers.
    pub accessibility_routes: Vec<M5NotebookKernelOutputAccessibilityRoute>,
    /// Notebook subsystems that consume this card's projection.
    pub consumer_surfaces: Vec<M5NotebookKernelOutputConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this card.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a restart / recovery never implies a rerun on restore or recovery. MUST be
    /// `false`.
    pub implies_rerun_on_restore_or_recovery: bool,
    /// Hard invariant: never presents lost state as preserved. MUST be `false`.
    pub presents_lost_state_as_preserved: bool,
    /// Hard invariant: never hides a consequence behind a hover-only affordance. MUST be `false`.
    pub hides_consequence_behind_hover_only: bool,
    /// Hard invariant: never collapses recovery into a generic notebook error. MUST be `false`.
    pub collapses_recovery_into_generic_error: bool,
}

impl RestartConsequenceCard {
    /// Impact / scope disclosures this card must carry, derived from the frozen states.
    pub fn restart_disclosure(&self) -> RestartConsequenceCardDisclosure {
        resolve_restart_consequence_card(self.restart_action, self.consequence_state)
    }

    /// Whether the card offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<RestartCardAction> = self.card_actions.iter().copied().collect();
        RestartCardAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the card declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5NotebookKernelOutputRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5NotebookKernelOutputRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the card offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.card_actions.contains(&RestartCardAction::OpenDeepLink)
    }
}

// ---- kernel-recovery-card vocabulary ------------------------------------

/// Derived kernel recovery posture a kernel recovery card may present.
///
/// This is the recovery honesty axis: the posture is derived from the frozen kernel recovery state,
/// never asserted, so a kernel that still needs a restart or has no kernel available can never read
/// as recovered and a user can always tell where recovery stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KernelRecoveryPosture {
    /// The kernel is recoverable.
    RecoverableNow,
    /// A reconnect is offered.
    ReconnectOffered,
    /// A restart is required.
    RestartNeeded,
    /// No kernel is available.
    NoKernelAvailable,
    /// Recovery is blocked.
    RecoveryBlocked,
    /// The kernel has recovered.
    RecoveredClean,
}

impl KernelRecoveryPosture {
    /// Every recovery posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RecoverableNow,
        Self::ReconnectOffered,
        Self::RestartNeeded,
        Self::NoKernelAvailable,
        Self::RecoveryBlocked,
        Self::RecoveredClean,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecoverableNow => "recoverable_now",
            Self::ReconnectOffered => "reconnect_offered",
            Self::RestartNeeded => "restart_needed",
            Self::NoKernelAvailable => "no_kernel_available",
            Self::RecoveryBlocked => "recovery_blocked",
            Self::RecoveredClean => "recovered_clean",
        }
    }

    /// True only when the kernel has recovered.
    pub const fn is_recovered(self) -> bool {
        matches!(self, Self::RecoveredClean)
    }

    /// True when a restart is required.
    pub const fn requires_restart(self) -> bool {
        matches!(self, Self::RestartNeeded)
    }

    /// True when no kernel is available.
    pub const fn has_no_kernel(self) -> bool {
        matches!(self, Self::NoKernelAvailable)
    }

    /// True when recovery is blocked.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::RecoveryBlocked)
    }
}

/// Derived recovery continuity class — whether a recovery keeps the running session, starts a clean
/// new session, or awaits a managed kernel, so a recovery that started a clean session never reads
/// as one that preserved continuity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryContinuityClass {
    /// The recovery continues the running session (reconnect / reattach).
    ContinuesSession,
    /// The recovery starts a clean new session (restart-clean / local fallback / another kernel).
    CleanSession,
    /// The recovery awaits a managed kernel.
    AwaitsManaged,
}

impl RecoveryContinuityClass {
    /// Every continuity class, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ContinuesSession,
        Self::CleanSession,
        Self::AwaitsManaged,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinuesSession => "continues_session",
            Self::CleanSession => "clean_session",
            Self::AwaitsManaged => "awaits_managed",
        }
    }

    /// True when the recovery preserves the running session's continuity.
    pub const fn preserves_continuity(self) -> bool {
        matches!(self, Self::ContinuesSession)
    }

    /// True when the recovery starts a clean new session (live state lost; a rerun is required).
    pub const fn is_clean_session(self) -> bool {
        matches!(self, Self::CleanSession)
    }
}

/// One keyboard-complete default action a kernel recovery card offers, so a card never hides its
/// reconnect / restart-clean / choose-another-kernel affordance behind a pointer-only gesture.
/// `Reconnect`, `RestartClean`, and `ChooseAnotherKernel` are always offered so recovery is always
/// keyboard-reachable, with open-inspect-only and export-evidence available as applicable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryCardAction {
    /// Reconnect to the kernel (always available).
    Reconnect,
    /// Restart clean (always available).
    RestartClean,
    /// Choose another kernel (always available).
    ChooseAnotherKernel,
    /// Open an inspect-only view of the failed kernel / session.
    OpenInspectOnly,
    /// Export the recovery evidence.
    ExportEvidence,
    /// Open the stable notebook / kernel-manager / docs / support deep link.
    OpenDeepLink,
}

impl RecoveryCardAction {
    /// Every recovery-card action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Reconnect,
        Self::RestartClean,
        Self::ChooseAnotherKernel,
        Self::OpenInspectOnly,
        Self::ExportEvidence,
        Self::OpenDeepLink,
    ];

    /// The default actions every keyboard-complete recovery card must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::Reconnect,
        Self::RestartClean,
        Self::ChooseAnotherKernel,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reconnect => "reconnect",
            Self::RestartClean => "restart_clean",
            Self::ChooseAnotherKernel => "choose_another_kernel",
            Self::OpenInspectOnly => "open_inspect_only",
            Self::ExportEvidence => "export_evidence",
            Self::OpenDeepLink => "open_deep_link",
        }
    }
}

/// Disclosures a kernel recovery card must carry, derived from the recovery action class and
/// recovery state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KernelRecoveryCardDisclosure {
    /// The derived recovery posture this card may present.
    pub posture_class: KernelRecoveryPosture,
    /// The derived continuity class this card may present.
    pub continuity_class: RecoveryContinuityClass,
    /// Whether the kernel has recovered.
    pub is_recovered: bool,
    /// Whether the card may claim the kernel has recovered.
    pub may_claim_recovered: bool,
    /// Whether the recovery preserves the running session's continuity.
    pub preserves_continuity: bool,
    /// Whether a rerun is required after this recovery (a clean session was started).
    pub requires_rerun_after_recovery: bool,
    /// Whether a restart is required.
    pub requires_restart: bool,
    /// Whether the card must carry an explicit clean-session note.
    pub needs_clean_session_note: bool,
    /// Whether the card must carry an explicit no-kernel note.
    pub needs_no_kernel_note: bool,
    /// Whether the card must carry an explicit blocked note.
    pub needs_blocked_note: bool,
    /// Whether the card must carry an explicit restart-required note.
    pub needs_restart_note: bool,
    /// Whether the card must carry an explicit awaits-managed note.
    pub needs_await_note: bool,
}

/// Resolves the posture and continuity truth a kernel recovery card may present.
///
/// A `recoverable` state is recoverable, a `reconnect_available` state offers a reconnect, a
/// `restart_required` state needs a restart, a `no_kernel_available` state has no kernel, a
/// `recovery_blocked` state is blocked, and a `recovered` state has recovered — so a kernel that
/// still needs a restart never reads as recovered. A `reconnect` or `reattach_session` recovery
/// continues the session, a `restart_clean`, `start_local_fallback`, or `choose_another_kernel`
/// recovery starts a clean new session, and a `wait_for_managed` recovery awaits a managed kernel. A
/// card may claim the kernel recovered only when its state is `recovered`; a clean session loses
/// live state and requires a rerun to recompute it — the card names that rerun and never implies one
/// already ran.
pub fn resolve_kernel_recovery_card(
    action: M5KernelRecoveryActionClass,
    state: M5KernelRecoveryState,
) -> KernelRecoveryCardDisclosure {
    use KernelRecoveryPosture as Posture;
    use M5KernelRecoveryActionClass as Act;
    use M5KernelRecoveryState as State;
    use RecoveryContinuityClass as Continuity;

    let posture_class = match state {
        State::Recoverable => Posture::RecoverableNow,
        State::ReconnectAvailable => Posture::ReconnectOffered,
        State::RestartRequired => Posture::RestartNeeded,
        State::NoKernelAvailable => Posture::NoKernelAvailable,
        State::RecoveryBlocked => Posture::RecoveryBlocked,
        State::Recovered => Posture::RecoveredClean,
    };

    let continuity_class = match action {
        Act::Reconnect | Act::ReattachSession => Continuity::ContinuesSession,
        Act::RestartClean | Act::StartLocalFallback | Act::ChooseAnotherKernel => {
            Continuity::CleanSession
        }
        Act::WaitForManaged => Continuity::AwaitsManaged,
    };

    KernelRecoveryCardDisclosure {
        posture_class,
        continuity_class,
        is_recovered: posture_class.is_recovered(),
        may_claim_recovered: posture_class.is_recovered(),
        preserves_continuity: continuity_class.preserves_continuity(),
        requires_rerun_after_recovery: continuity_class.is_clean_session(),
        requires_restart: posture_class.requires_restart(),
        needs_clean_session_note: continuity_class.is_clean_session(),
        needs_no_kernel_note: posture_class.has_no_kernel(),
        needs_blocked_note: posture_class.is_blocked(),
        needs_restart_note: posture_class.requires_restart(),
        needs_await_note: matches!(continuity_class, Continuity::AwaitsManaged),
    }
}

/// A kernel recovery card naming where a kernel's recovery stands, its continuity class, whether a
/// rerun is required after recovery, its derived posture, bounded reconnect / restart-clean /
/// choose-another-kernel actions, a no-hidden-rerun note, and a stable deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelRecoveryCard {
    /// Frozen component this control implements; must be `kernel_recovery_card`.
    pub component: M5NotebookKernelOutputComponentFamily,
    /// Stable card id.
    pub card_id: String,
    /// Human-readable card label; required and non-empty.
    pub card_label: String,
    /// Kernel recovery action class, reused from the frozen matrix.
    pub recovery_action: M5KernelRecoveryActionClass,
    /// Kernel recovery state, reused from the frozen matrix.
    pub recovery_state: M5KernelRecoveryState,
    /// Derived recovery posture (must equal the resolved posture).
    pub posture_class: KernelRecoveryPosture,
    /// Derived continuity class (must equal the resolved class).
    pub continuity_class: RecoveryContinuityClass,
    /// Whether the card claims the kernel has recovered. May be `true` only when the derived truth
    /// allows it.
    pub claims_recovered: bool,
    /// Whether the card claims the recovery preserves continuity (must equal the derived truth).
    pub claims_continuity_preserved: bool,
    /// Whether the card claims a rerun is required after recovery (must equal the derived truth).
    pub requires_rerun_after_recovery: bool,
    /// No-hidden-rerun note; always required so a recovery never implies that code or cells were
    /// silently executed during restore or repair.
    pub no_rerun_note: String,
    /// Clean-session note; required when the recovery starts a clean new session.
    pub clean_session_note: String,
    /// No-kernel note; required when no kernel is available.
    pub no_kernel_note: String,
    /// Blocked note; required when recovery is blocked.
    pub blocked_note: String,
    /// Restart-required note; required when a restart is required.
    pub restart_note: String,
    /// Awaits-managed note; required when the recovery awaits a managed kernel.
    pub await_note: String,
    /// Recovery action label; always required so the recovery action is never hidden behind a
    /// hover-only affordance.
    pub recovery_action_label: String,
    /// Recovery state label; always required so where recovery stands stays explicit.
    pub recovery_state_label: String,
    /// Context note; always required so the card names what the recovery truth means here.
    pub context_note: String,
    /// Kind of stable deep link this card binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include reconnect / restart-clean /
    /// choose-another-kernel).
    pub card_actions: Vec<RecoveryCardAction>,
    /// Dispositions this card binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5NotebookKernelOutputDisposition>,
    /// Downgrade triggers this card can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5NotebookKernelOutputDowngradeTrigger>,
    /// Mandatory labels this card can show (must include the mandatory labels).
    pub required_labels: Vec<M5NotebookKernelOutputRequiredLabel>,
    /// Claimed M5 surface families that render this card.
    pub surface_families: Vec<M5NotebookKernelOutputSurfaceFamily>,
    /// Deployment lines this card keeps the same truth across.
    pub deployment_lines: Vec<M5NotebookKernelOutputDeploymentLine>,
    /// Non-visual accessibility routes this card offers.
    pub accessibility_routes: Vec<M5NotebookKernelOutputAccessibilityRoute>,
    /// Notebook subsystems that consume this card's projection.
    pub consumer_surfaces: Vec<M5NotebookKernelOutputConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this card.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a restart / recovery never implies a rerun on restore or recovery. MUST be
    /// `false`.
    pub implies_rerun_on_restore_or_recovery: bool,
    /// Hard invariant: never presents lost state as preserved. MUST be `false`.
    pub presents_lost_state_as_preserved: bool,
    /// Hard invariant: never hides a consequence behind a hover-only affordance. MUST be `false`.
    pub hides_consequence_behind_hover_only: bool,
    /// Hard invariant: never collapses recovery into a generic notebook error. MUST be `false`.
    pub collapses_recovery_into_generic_error: bool,
}

impl KernelRecoveryCard {
    /// Posture / continuity disclosures this card must carry, derived from the frozen states.
    pub fn recovery_disclosure(&self) -> KernelRecoveryCardDisclosure {
        resolve_kernel_recovery_card(self.recovery_action, self.recovery_state)
    }

    /// Whether the card offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<RecoveryCardAction> = self.card_actions.iter().copied().collect();
        RecoveryCardAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the card declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5NotebookKernelOutputRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5NotebookKernelOutputRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the card offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.card_actions
            .contains(&RecoveryCardAction::OpenDeepLink)
    }
}

// ---- review blocks ------------------------------------------------------

/// First-glance restart-consequence / kernel-recovery review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestartRecoveryReview {
    /// The restart card names what state survives.
    pub restart_card_shows_preserved_state: bool,
    /// The restart card names what live state is lost.
    pub restart_card_shows_lost_state: bool,
    /// The restart card names its debugger / session impact.
    pub restart_card_shows_debugger_session_impact: bool,
    /// The restart card names its rerun requirement before restart.
    pub restart_card_shows_rerun_requirement: bool,
    /// The recovery card offers reconnect / restart-clean / choose-another-kernel.
    pub recovery_card_offers_reconnect_restart_choose: bool,
    /// The recovery card names where recovery stands.
    pub recovery_card_shows_recovery_state: bool,
    /// The recovery card never implies a rerun.
    pub recovery_card_never_implies_rerun: bool,
    /// Recovery never overclaims that the kernel recovered.
    pub recovery_never_overclaims_recovered: bool,
    /// Impact and posture are derived from state, never asserted.
    pub impact_and_posture_derived_never_asserted: bool,
    /// Lost state is never presented as preserved.
    pub lost_state_never_presented_as_preserved: bool,
    /// A consequence is never hidden behind a hover-only affordance.
    pub consequence_never_hover_only: bool,
    /// A rerun requirement is named before restart, never after the fact.
    pub rerun_requirement_named_before_restart: bool,
    /// Failure degrades to an attributable recovery state, never a generic notebook error.
    pub recovery_degrades_to_attributable_state_not_generic_error: bool,
    /// Kernel origin is never collapsed into one unlabeled badge.
    pub kernel_origin_never_collapsed_into_one_badge: bool,
    /// Every next step names one stable notebook / kernel-manager / docs / support deep link.
    pub every_next_step_names_stable_deep_link: bool,
    /// Cards stay consistent across notebook, debug, review, support, and companion surfaces.
    pub cards_consistent_across_surfaces: bool,
    /// No component widens export scope or exposes raw payloads by default.
    pub no_component_widens_export_scope_or_exposes_raw_by_default: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl RestartRecoveryReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.restart_card_shows_preserved_state
            && self.restart_card_shows_lost_state
            && self.restart_card_shows_debugger_session_impact
            && self.restart_card_shows_rerun_requirement
            && self.recovery_card_offers_reconnect_restart_choose
            && self.recovery_card_shows_recovery_state
            && self.recovery_card_never_implies_rerun
            && self.recovery_never_overclaims_recovered
            && self.impact_and_posture_derived_never_asserted
            && self.lost_state_never_presented_as_preserved
            && self.consequence_never_hover_only
            && self.rerun_requirement_named_before_restart
            && self.recovery_degrades_to_attributable_state_not_generic_error
            && self.kernel_origin_never_collapsed_into_one_badge
            && self.every_next_step_names_stable_deep_link
            && self.cards_consistent_across_surfaces
            && self.no_component_widens_export_scope_or_exposes_raw_by_default
            && self.components_stable_across_deployment_lines
            && self.no_surface_invents_alternate_state_label
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestartRecoveryConsumerProjection {
    /// The notebook tab reads a single canonical source.
    pub notebook_tab_reads_single_source: bool,
    /// The debug bridge shows restart consequences.
    pub debug_bridge_shows_restart_consequences: bool,
    /// The support packet shows the recovery state.
    pub support_packet_shows_recovery_state: bool,
    /// The companion handoff shows a recovery summary.
    pub companion_handoff_shows_recovery_summary: bool,
    /// The CLI export preserves the no-hidden-rerun truth.
    pub cli_export_preserves_no_rerun_truth: bool,
    /// Help / docs shows component truth.
    pub help_docs_shows_component_truth: bool,
}

impl RestartRecoveryConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.notebook_tab_reads_single_source
            && self.debug_bridge_shows_restart_consequences
            && self.support_packet_shows_recovery_state
            && self.companion_handoff_shows_recovery_summary
            && self.cli_export_preserves_no_rerun_truth
            && self.help_docs_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestartRecoveryProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`RestartConsequenceCardKernelRecoveryCardControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestartConsequenceCardKernelRecoveryCardControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Restart consequence cards.
    pub restart_consequence_cards: Vec<RestartConsequenceCard>,
    /// Kernel recovery cards.
    pub kernel_recovery_cards: Vec<KernelRecoveryCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5NotebookKernelOutputDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5NotebookKernelOutputConsumerSurface>,
    /// Restart / recovery review block.
    pub restart_recovery_review: RestartRecoveryReview,
    /// Consumer projection block.
    pub consumer_projection: RestartRecoveryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RestartRecoveryProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe restart-consequence-card / kernel-recovery-card controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestartConsequenceCardKernelRecoveryCardControlsPacket {
    /// Record kind; must equal
    /// [`RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Restart consequence cards.
    pub restart_consequence_cards: Vec<RestartConsequenceCard>,
    /// Kernel recovery cards.
    pub kernel_recovery_cards: Vec<KernelRecoveryCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5NotebookKernelOutputDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5NotebookKernelOutputConsumerSurface>,
    /// Restart / recovery review block.
    pub restart_recovery_review: RestartRecoveryReview,
    /// Consumer projection block.
    pub consumer_projection: RestartRecoveryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RestartRecoveryProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl RestartConsequenceCardKernelRecoveryCardControlsPacket {
    /// Builds a restart-consequence-card / kernel-recovery-card controls packet from stable-lane
    /// input.
    pub fn new(input: RestartConsequenceCardKernelRecoveryCardControlsPacketInput) -> Self {
        Self {
            record_kind: RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_RECORD_KIND.to_owned(),
            schema_version: RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            restart_consequence_cards: input.restart_consequence_cards,
            kernel_recovery_cards: input.kernel_recovery_cards,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            restart_recovery_review: input.restart_recovery_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the restart-consequence-card / kernel-recovery-card control invariants.
    pub fn validate(&self) -> Vec<RestartConsequenceCardKernelRecoveryCardViolation> {
        let mut violations = Vec::new();

        if self.record_kind != RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_RECORD_KIND {
            violations.push(RestartConsequenceCardKernelRecoveryCardViolation::WrongRecordKind);
        }
        if self.schema_version != RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_SCHEMA_VERSION {
            violations.push(RestartConsequenceCardKernelRecoveryCardViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(RestartConsequenceCardKernelRecoveryCardViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations
                .push(RestartConsequenceCardKernelRecoveryCardViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations
                .push(RestartConsequenceCardKernelRecoveryCardViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_restart_cards(self, &mut violations);
        validate_recovery_cards(self, &mut violations);

        if !self.restart_recovery_review.all_hold() {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::RestartRecoveryReviewIncomplete,
            );
        }
        if !self.consumer_projection.all_hold() {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::ConsumerProjectionIncomplete,
            );
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations
                .push(RestartConsequenceCardKernelRecoveryCardViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("restart consequence card kernel recovery card packet serializes"),
        ) {
            violations.push(RestartConsequenceCardKernelRecoveryCardViolation::RawMaterialInExport);
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
            .expect("restart consequence card kernel recovery card packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str("component,id,action,state,derived,preserved_or_recovered,deep_link_kind\n");
        for card in &self.restart_consequence_cards {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "restart_consequence_card",
                csv_field(&card.card_id),
                card.restart_action.as_str(),
                card.consequence_state.as_str(),
                card.restart_disclosure().impact_class.as_str(),
                card.restart_disclosure().may_claim_state_preserved,
                card.deep_link_kind.as_str(),
            ));
        }
        for card in &self.kernel_recovery_cards {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "kernel_recovery_card",
                csv_field(&card.card_id),
                card.recovery_action.as_str(),
                card.recovery_state.as_str(),
                card.recovery_disclosure().posture_class.as_str(),
                card.recovery_disclosure().may_claim_recovered,
                card.deep_link_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let loses_state = self
            .restart_consequence_cards
            .iter()
            .filter(|card| card.restart_disclosure().loses_live_state)
            .count();
        let not_recovered = self
            .kernel_recovery_cards
            .iter()
            .filter(|card| !card.recovery_disclosure().is_recovered)
            .count();

        let mut out = String::new();
        out.push_str("# Restart consequence cards and kernel recovery cards\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Restart consequence cards: {} ({} lose live state)\n",
            self.restart_consequence_cards.len(),
            loses_state
        ));
        out.push_str(&format!(
            "- Kernel recovery cards: {} ({} not recovered)\n",
            self.kernel_recovery_cards.len(),
            not_recovered
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Restart consequence cards\n\n");
        for card in &self.restart_consequence_cards {
            out.push_str(&format!(
                "- **{}** — action `{}`, consequence `{}` → `{}`, scope `{}`, deep link `{}`\n",
                card.card_label,
                card.restart_action.as_str(),
                card.consequence_state.as_str(),
                card.restart_disclosure().impact_class.as_str(),
                card.restart_disclosure().action_scope.as_str(),
                card.deep_link_kind.as_str(),
            ));
        }

        out.push_str("\n## Kernel recovery cards\n\n");
        for card in &self.kernel_recovery_cards {
            out.push_str(&format!(
                "- **{}** — action `{}`, state `{}` → `{}` / `{}`, deep link `{}`\n",
                card.card_label,
                card.recovery_action.as_str(),
                card.recovery_state.as_str(),
                card.recovery_disclosure().posture_class.as_str(),
                card.recovery_disclosure().continuity_class.as_str(),
                card.deep_link_kind.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in restart-consequence-card / kernel-recovery-card
/// export.
#[derive(Debug)]
pub enum RestartConsequenceCardKernelRecoveryCardArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RestartConsequenceCardKernelRecoveryCardViolation>),
}

impl fmt::Display for RestartConsequenceCardKernelRecoveryCardArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "restart consequence card kernel recovery card export parse failed: {error}"
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
                    "restart consequence card kernel recovery card export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for RestartConsequenceCardKernelRecoveryCardArtifactError {}

/// Validation failures emitted by
/// [`RestartConsequenceCardKernelRecoveryCardControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RestartConsequenceCardKernelRecoveryCardViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No restart consequence cards are present.
    RestartCardsMissing,
    /// A restart consequence card is incomplete.
    RestartCardIncomplete,
    /// A restart consequence card carries the wrong frozen component class.
    RestartCardWrongComponentClass,
    /// A restart card misrepresents its derived impact or scope.
    ImpactMisrepresented,
    /// A restart card claims state was preserved when the impact does not allow it.
    StatePreservationOverclaimed,
    /// A restart card does not name its preserved state.
    PreservedStateLabelMissing,
    /// A lost-state restart card does not name its lost state.
    LostStateNoteMissing,
    /// A variables-cleared restart card does not name its cleared variables.
    VariablesClearedNoteMissing,
    /// An outputs-cleared restart card does not name its cleared outputs.
    OutputsClearedNoteMissing,
    /// A restart card that requires a rerun does not name its rerun requirement.
    RerunNoteMissing,
    /// A session-ending restart card does not name its debugger / session impact.
    DebuggerSessionNoteMissing,
    /// A restart card does not name its restart action.
    RestartActionLabelMissing,
    /// A restart card does not name its consequence state.
    ConsequenceStateLabelMissing,
    /// A restart card omits a mandatory review / confirm / cancel action.
    RestartCardActionsIncomplete,
    /// The restart cards do not cover every restart action class.
    RestartActionClassCoverageMissing,
    /// The restart cards do not cover every restart consequence state.
    RestartConsequenceStateCoverageMissing,
    /// The restart cards do not cover every derived restart impact class.
    RestartImpactClassCoverageMissing,
    /// The restart cards do not cover every derived restart action scope.
    RestartActionScopeCoverageMissing,
    /// No kernel recovery cards are present.
    RecoveryCardsMissing,
    /// A kernel recovery card is incomplete.
    RecoveryCardIncomplete,
    /// A kernel recovery card carries the wrong frozen component class.
    RecoveryCardWrongComponentClass,
    /// A recovery card misrepresents its derived posture or continuity.
    RecoveryMisrepresented,
    /// A recovery card claims the kernel recovered when the state does not allow it.
    RecoveryOverclaimed,
    /// A recovery card does not carry its no-hidden-rerun note.
    NoRerunNoteMissing,
    /// A clean-session recovery card does not name its clean session.
    CleanSessionNoteMissing,
    /// A no-kernel recovery card does not name its missing kernel.
    NoKernelNoteMissing,
    /// A blocked recovery card does not name its blocked recovery.
    BlockedNoteMissing,
    /// A restart-required recovery card does not name its required restart.
    RestartNoteMissing,
    /// An awaits-managed recovery card does not name its awaited kernel.
    AwaitNoteMissing,
    /// A recovery card does not name its recovery action.
    RecoveryActionLabelMissing,
    /// A recovery card does not name its recovery state.
    RecoveryStateLabelMissing,
    /// A recovery card omits a mandatory reconnect / restart-clean / choose-another-kernel action.
    RecoveryCardActionsIncomplete,
    /// The recovery cards do not cover every kernel recovery action class.
    RecoveryActionClassCoverageMissing,
    /// The recovery cards do not cover every kernel recovery state.
    RecoveryStateCoverageMissing,
    /// The recovery cards do not cover every derived recovery posture.
    RecoveryPostureCoverageMissing,
    /// The recovery cards do not cover every derived recovery continuity class.
    RecoveryContinuityCoverageMissing,
    /// A component does not name its context.
    ContextNoteMissing,
    /// A component offers a deep-link action but its deep link does not resolve exactly.
    DeepLinkUnresolved,
    /// A component names a deep-link kind but not its stable reference.
    DeepLinkRefMissing,
    /// A component does not bind any disposition.
    DispositionsMissing,
    /// A component does not declare its downgrade triggers.
    DowngradeTriggersMissing,
    /// A component does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A component does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A component implies a rerun on restore or recovery.
    RerunImpliedOnRestoreOrRecovery,
    /// A component presents lost state as preserved.
    LostStateShownAsPreserved,
    /// A component hides a consequence behind a hover-only affordance.
    ConsequenceHoverOnly,
    /// A component collapses recovery into a generic notebook error.
    RecoveryCollapsedIntoGenericError,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Restart / recovery review does not satisfy required invariants.
    RestartRecoveryReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl RestartConsequenceCardKernelRecoveryCardViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RestartCardsMissing => "restart_cards_missing",
            Self::RestartCardIncomplete => "restart_card_incomplete",
            Self::RestartCardWrongComponentClass => "restart_card_wrong_component_class",
            Self::ImpactMisrepresented => "impact_misrepresented",
            Self::StatePreservationOverclaimed => "state_preservation_overclaimed",
            Self::PreservedStateLabelMissing => "preserved_state_label_missing",
            Self::LostStateNoteMissing => "lost_state_note_missing",
            Self::VariablesClearedNoteMissing => "variables_cleared_note_missing",
            Self::OutputsClearedNoteMissing => "outputs_cleared_note_missing",
            Self::RerunNoteMissing => "rerun_note_missing",
            Self::DebuggerSessionNoteMissing => "debugger_session_note_missing",
            Self::RestartActionLabelMissing => "restart_action_label_missing",
            Self::ConsequenceStateLabelMissing => "consequence_state_label_missing",
            Self::RestartCardActionsIncomplete => "restart_card_actions_incomplete",
            Self::RestartActionClassCoverageMissing => "restart_action_class_coverage_missing",
            Self::RestartConsequenceStateCoverageMissing => {
                "restart_consequence_state_coverage_missing"
            }
            Self::RestartImpactClassCoverageMissing => "restart_impact_class_coverage_missing",
            Self::RestartActionScopeCoverageMissing => "restart_action_scope_coverage_missing",
            Self::RecoveryCardsMissing => "recovery_cards_missing",
            Self::RecoveryCardIncomplete => "recovery_card_incomplete",
            Self::RecoveryCardWrongComponentClass => "recovery_card_wrong_component_class",
            Self::RecoveryMisrepresented => "recovery_misrepresented",
            Self::RecoveryOverclaimed => "recovery_overclaimed",
            Self::NoRerunNoteMissing => "no_rerun_note_missing",
            Self::CleanSessionNoteMissing => "clean_session_note_missing",
            Self::NoKernelNoteMissing => "no_kernel_note_missing",
            Self::BlockedNoteMissing => "blocked_note_missing",
            Self::RestartNoteMissing => "restart_note_missing",
            Self::AwaitNoteMissing => "await_note_missing",
            Self::RecoveryActionLabelMissing => "recovery_action_label_missing",
            Self::RecoveryStateLabelMissing => "recovery_state_label_missing",
            Self::RecoveryCardActionsIncomplete => "recovery_card_actions_incomplete",
            Self::RecoveryActionClassCoverageMissing => "recovery_action_class_coverage_missing",
            Self::RecoveryStateCoverageMissing => "recovery_state_coverage_missing",
            Self::RecoveryPostureCoverageMissing => "recovery_posture_coverage_missing",
            Self::RecoveryContinuityCoverageMissing => "recovery_continuity_coverage_missing",
            Self::ContextNoteMissing => "context_note_missing",
            Self::DeepLinkUnresolved => "deep_link_unresolved",
            Self::DeepLinkRefMissing => "deep_link_ref_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::RerunImpliedOnRestoreOrRecovery => "rerun_implied_on_restore_or_recovery",
            Self::LostStateShownAsPreserved => "lost_state_shown_as_preserved",
            Self::ConsequenceHoverOnly => "consequence_hover_only",
            Self::RecoveryCollapsedIntoGenericError => "recovery_collapsed_into_generic_error",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::RestartRecoveryReviewIncomplete => "restart_recovery_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable restart-consequence-card / kernel-recovery-card export.
pub fn current_restart_consequence_card_kernel_recovery_card_export() -> Result<
    RestartConsequenceCardKernelRecoveryCardControlsPacket,
    RestartConsequenceCardKernelRecoveryCardArtifactError,
> {
    let packet: RestartConsequenceCardKernelRecoveryCardControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-restart-consequence-card-kernel-recovery-card-proof/support_export.json"
        )))
        .map_err(RestartConsequenceCardKernelRecoveryCardArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RestartConsequenceCardKernelRecoveryCardArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &RestartConsequenceCardKernelRecoveryCardControlsPacket,
    violations: &mut Vec<RestartConsequenceCardKernelRecoveryCardViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_SCHEMA_REF,
        RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_DOC_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_DOC_REF,
        M5_RESTART_CONSEQUENCE_CARD_SCHEMA_REF,
        M5_KERNEL_RECOVERY_CARD_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations
                .push(RestartConsequenceCardKernelRecoveryCardViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_restart_cards(
    packet: &RestartConsequenceCardKernelRecoveryCardControlsPacket,
    violations: &mut Vec<RestartConsequenceCardKernelRecoveryCardViolation>,
) {
    if packet.restart_consequence_cards.is_empty() {
        violations.push(RestartConsequenceCardKernelRecoveryCardViolation::RestartCardsMissing);
        return;
    }

    let mut impact_classes: BTreeSet<RestartImpactClass> = BTreeSet::new();
    let mut action_scopes: BTreeSet<RestartActionScope> = BTreeSet::new();
    let mut actions: BTreeSet<M5RestartActionClass> = BTreeSet::new();
    let mut consequences: BTreeSet<M5RestartConsequenceState> = BTreeSet::new();

    for card in &packet.restart_consequence_cards {
        let disclosure = card.restart_disclosure();
        impact_classes.insert(disclosure.impact_class);
        action_scopes.insert(disclosure.action_scope);
        actions.insert(card.restart_action);
        consequences.insert(card.consequence_state);

        if card.card_id.trim().is_empty()
            || card.card_label.trim().is_empty()
            || card.fields_shown.is_empty()
            || card.surface_families.is_empty()
            || card.deployment_lines.is_empty()
            || card.consumer_surfaces.is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations
                .push(RestartConsequenceCardKernelRecoveryCardViolation::RestartCardIncomplete);
        }
        if card.component != M5NotebookKernelOutputComponentFamily::RestartConsequenceCard {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::RestartCardWrongComponentClass,
            );
        }
        if card.impact_class != disclosure.impact_class
            || card.action_scope != disclosure.action_scope
            || card.requires_rerun != disclosure.requires_rerun
            || card.affects_debugger_session != disclosure.affects_debugger_session
        {
            violations
                .push(RestartConsequenceCardKernelRecoveryCardViolation::ImpactMisrepresented);
        }
        if card.claims_state_preserved && !disclosure.may_claim_state_preserved {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::StatePreservationOverclaimed,
            );
        }
        if card.preserved_state_label.trim().is_empty() {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::PreservedStateLabelMissing,
            );
        }
        if disclosure.needs_lost_state_note && card.lost_state_note.trim().is_empty() {
            violations
                .push(RestartConsequenceCardKernelRecoveryCardViolation::LostStateNoteMissing);
        }
        if disclosure.needs_variables_cleared_note && card.variables_cleared_note.trim().is_empty()
        {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::VariablesClearedNoteMissing,
            );
        }
        if disclosure.needs_outputs_cleared_note && card.outputs_cleared_note.trim().is_empty() {
            violations
                .push(RestartConsequenceCardKernelRecoveryCardViolation::OutputsClearedNoteMissing);
        }
        if disclosure.needs_rerun_note && card.rerun_requirement_note.trim().is_empty() {
            violations.push(RestartConsequenceCardKernelRecoveryCardViolation::RerunNoteMissing);
        }
        if disclosure.needs_debugger_session_note && card.debugger_session_note.trim().is_empty() {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::DebuggerSessionNoteMissing,
            );
        }
        if card.restart_action_label.trim().is_empty() {
            violations
                .push(RestartConsequenceCardKernelRecoveryCardViolation::RestartActionLabelMissing);
        }
        if card.consequence_state_label.trim().is_empty() {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::ConsequenceStateLabelMissing,
            );
        }
        if !card.declares_mandatory_actions() {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::RestartCardActionsIncomplete,
            );
        }
        validate_deep_link(
            card.offers_deep_link_action(),
            card.deep_link_kind,
            &card.deep_link_ref,
            &card.context_note,
            violations,
        );
        validate_common_control(
            &card.dispositions,
            &card.downgrade_triggers,
            card.declares_mandatory_labels(),
            &card.accessibility_routes,
            ControlInvariants {
                implies_rerun_on_restore_or_recovery: card.implies_rerun_on_restore_or_recovery,
                presents_lost_state_as_preserved: card.presents_lost_state_as_preserved,
                hides_consequence_behind_hover_only: card.hides_consequence_behind_hover_only,
                collapses_recovery_into_generic_error: card.collapses_recovery_into_generic_error,
            },
            violations,
        );
    }

    for required in RestartImpactClass::ALL {
        if !impact_classes.contains(&required) {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::RestartImpactClassCoverageMissing,
            );
            break;
        }
    }
    for required in RestartActionScope::ALL {
        if !action_scopes.contains(&required) {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::RestartActionScopeCoverageMissing,
            );
            break;
        }
    }
    for required in M5RestartActionClass::ALL {
        if !actions.contains(&required) {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::RestartActionClassCoverageMissing,
            );
            break;
        }
    }
    for required in M5RestartConsequenceState::ALL {
        if !consequences.contains(&required) {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::RestartConsequenceStateCoverageMissing,
            );
            break;
        }
    }
}

fn validate_recovery_cards(
    packet: &RestartConsequenceCardKernelRecoveryCardControlsPacket,
    violations: &mut Vec<RestartConsequenceCardKernelRecoveryCardViolation>,
) {
    if packet.kernel_recovery_cards.is_empty() {
        violations.push(RestartConsequenceCardKernelRecoveryCardViolation::RecoveryCardsMissing);
        return;
    }

    let mut postures: BTreeSet<KernelRecoveryPosture> = BTreeSet::new();
    let mut continuities: BTreeSet<RecoveryContinuityClass> = BTreeSet::new();
    let mut actions: BTreeSet<M5KernelRecoveryActionClass> = BTreeSet::new();
    let mut states: BTreeSet<M5KernelRecoveryState> = BTreeSet::new();

    for card in &packet.kernel_recovery_cards {
        let disclosure = card.recovery_disclosure();
        postures.insert(disclosure.posture_class);
        continuities.insert(disclosure.continuity_class);
        actions.insert(card.recovery_action);
        states.insert(card.recovery_state);

        if card.card_id.trim().is_empty()
            || card.card_label.trim().is_empty()
            || card.fields_shown.is_empty()
            || card.surface_families.is_empty()
            || card.deployment_lines.is_empty()
            || card.consumer_surfaces.is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations
                .push(RestartConsequenceCardKernelRecoveryCardViolation::RecoveryCardIncomplete);
        }
        if card.component != M5NotebookKernelOutputComponentFamily::KernelRecoveryCard {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::RecoveryCardWrongComponentClass,
            );
        }
        if card.posture_class != disclosure.posture_class
            || card.continuity_class != disclosure.continuity_class
            || card.claims_continuity_preserved != disclosure.preserves_continuity
            || card.requires_rerun_after_recovery != disclosure.requires_rerun_after_recovery
        {
            violations
                .push(RestartConsequenceCardKernelRecoveryCardViolation::RecoveryMisrepresented);
        }
        if card.claims_recovered && !disclosure.may_claim_recovered {
            violations.push(RestartConsequenceCardKernelRecoveryCardViolation::RecoveryOverclaimed);
        }
        if card.no_rerun_note.trim().is_empty() {
            violations.push(RestartConsequenceCardKernelRecoveryCardViolation::NoRerunNoteMissing);
        }
        if disclosure.needs_clean_session_note && card.clean_session_note.trim().is_empty() {
            violations
                .push(RestartConsequenceCardKernelRecoveryCardViolation::CleanSessionNoteMissing);
        }
        if disclosure.needs_no_kernel_note && card.no_kernel_note.trim().is_empty() {
            violations.push(RestartConsequenceCardKernelRecoveryCardViolation::NoKernelNoteMissing);
        }
        if disclosure.needs_blocked_note && card.blocked_note.trim().is_empty() {
            violations.push(RestartConsequenceCardKernelRecoveryCardViolation::BlockedNoteMissing);
        }
        if disclosure.needs_restart_note && card.restart_note.trim().is_empty() {
            violations.push(RestartConsequenceCardKernelRecoveryCardViolation::RestartNoteMissing);
        }
        if disclosure.needs_await_note && card.await_note.trim().is_empty() {
            violations.push(RestartConsequenceCardKernelRecoveryCardViolation::AwaitNoteMissing);
        }
        if card.recovery_action_label.trim().is_empty() {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::RecoveryActionLabelMissing,
            );
        }
        if card.recovery_state_label.trim().is_empty() {
            violations
                .push(RestartConsequenceCardKernelRecoveryCardViolation::RecoveryStateLabelMissing);
        }
        if !card.declares_mandatory_actions() {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::RecoveryCardActionsIncomplete,
            );
        }
        validate_deep_link(
            card.offers_deep_link_action(),
            card.deep_link_kind,
            &card.deep_link_ref,
            &card.context_note,
            violations,
        );
        validate_common_control(
            &card.dispositions,
            &card.downgrade_triggers,
            card.declares_mandatory_labels(),
            &card.accessibility_routes,
            ControlInvariants {
                implies_rerun_on_restore_or_recovery: card.implies_rerun_on_restore_or_recovery,
                presents_lost_state_as_preserved: card.presents_lost_state_as_preserved,
                hides_consequence_behind_hover_only: card.hides_consequence_behind_hover_only,
                collapses_recovery_into_generic_error: card.collapses_recovery_into_generic_error,
            },
            violations,
        );
    }

    for required in KernelRecoveryPosture::ALL {
        if !postures.contains(&required) {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::RecoveryPostureCoverageMissing,
            );
            break;
        }
    }
    for required in RecoveryContinuityClass::ALL {
        if !continuities.contains(&required) {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::RecoveryContinuityCoverageMissing,
            );
            break;
        }
    }
    for required in M5KernelRecoveryActionClass::ALL {
        if !actions.contains(&required) {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::RecoveryActionClassCoverageMissing,
            );
            break;
        }
    }
    for required in M5KernelRecoveryState::ALL {
        if !states.contains(&required) {
            violations.push(
                RestartConsequenceCardKernelRecoveryCardViolation::RecoveryStateCoverageMissing,
            );
            break;
        }
    }
}

/// Validates the context and stable deep-link truth shared by both component vectors.
///
/// A component that offers a deep-link action must name a resolvable deep-link kind, a component
/// that names a resolvable kind must carry its stable reference, and every component must name its
/// context — so a next step is never an ephemeral overlay or hidden route.
fn validate_deep_link(
    offers_deep_link_action: bool,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    context_note: &str,
    violations: &mut Vec<RestartConsequenceCardKernelRecoveryCardViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(RestartConsequenceCardKernelRecoveryCardViolation::ContextNoteMissing);
    }
    if offers_deep_link_action && !deep_link_kind.is_resolvable() {
        violations.push(RestartConsequenceCardKernelRecoveryCardViolation::DeepLinkUnresolved);
    }
    if deep_link_kind.is_resolvable() && deep_link_ref.trim().is_empty() {
        violations.push(RestartConsequenceCardKernelRecoveryCardViolation::DeepLinkRefMissing);
    }
}

/// The four hard-invariant bools every component must keep `false`.
struct ControlInvariants {
    implies_rerun_on_restore_or_recovery: bool,
    presents_lost_state_as_preserved: bool,
    hides_consequence_behind_hover_only: bool,
    collapses_recovery_into_generic_error: bool,
}

/// Validates the axes shared by both component vectors.
fn validate_common_control(
    dispositions: &[M5NotebookKernelOutputDisposition],
    downgrade_triggers: &[M5NotebookKernelOutputDowngradeTrigger],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5NotebookKernelOutputAccessibilityRoute],
    invariants: ControlInvariants,
    violations: &mut Vec<RestartConsequenceCardKernelRecoveryCardViolation>,
) {
    if dispositions.is_empty() {
        violations.push(RestartConsequenceCardKernelRecoveryCardViolation::DispositionsMissing);
    }
    if downgrade_triggers.is_empty() {
        violations
            .push(RestartConsequenceCardKernelRecoveryCardViolation::DowngradeTriggersMissing);
    }
    if !declares_mandatory_labels {
        violations
            .push(RestartConsequenceCardKernelRecoveryCardViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes
            .contains(&M5NotebookKernelOutputAccessibilityRoute::KeyboardFocusable)
    {
        violations
            .push(RestartConsequenceCardKernelRecoveryCardViolation::AccessibilityRouteMissing);
    }
    if invariants.implies_rerun_on_restore_or_recovery {
        violations.push(
            RestartConsequenceCardKernelRecoveryCardViolation::RerunImpliedOnRestoreOrRecovery,
        );
    }
    if invariants.presents_lost_state_as_preserved {
        violations
            .push(RestartConsequenceCardKernelRecoveryCardViolation::LostStateShownAsPreserved);
    }
    if invariants.hides_consequence_behind_hover_only {
        violations.push(RestartConsequenceCardKernelRecoveryCardViolation::ConsequenceHoverOnly);
    }
    if invariants.collapses_recovery_into_generic_error {
        violations.push(
            RestartConsequenceCardKernelRecoveryCardViolation::RecoveryCollapsedIntoGenericError,
        );
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
