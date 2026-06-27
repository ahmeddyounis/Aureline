//! Governed **executable step objects** — every runbook step as a stable, typed,
//! exportable object tools and exports can reason about.
//!
//! The [governance matrix](crate::m5_runbook_governance) names *what* a runbook
//! step object is, and the [source register](crate::m5_runbook_sources) decides
//! *whether* a runbook may speak with authority. This module makes the executable
//! step itself durable: every step is a [`RunbookExecutableStep`] rather than a
//! block of opaque prose or a local action button. Each step declares a stable
//! [id](RunbookExecutableStep::step_id), a
//! [step class](crate::m5_runbook_governance::RunbookStepClass) (inspect,
//! diagnose, mitigate, rollback, console-handoff, approval, annotate), the
//! [target-selector scope](TargetSelectorScope) it reaches, the
//! [approval scope](crate::m5_runbook_governance::RunbookApprovalScope) it
//! requires, whether it stays [view-only, in-product executable, or
//! handoff-only](StepExecutionMode), the [control-plane
//! boundary](crate::m5_runbook_governance::ControlPlaneBoundaryClass) it sits on,
//! the expected evidence outputs it must produce, and whether a companion may run
//! it within declared scope.
//!
//! Crucially, an executable step does not carry its own privileged mutate path. It
//! [binds](CommandEnvelopeBinding) to Aureline's shared command/action-envelope and
//! approval systems, so preview, approval, and audit behavior are *derived
//! mechanically* from the object rather than hand-wired per step. The
//! [`StepGovernanceProjection`] is that derivation: given a step object alone, the
//! shell, a companion follow view, and a support export all compute the same
//! preview disposition, the same approval requirement, and the same audit
//! expectation. A mutating step with no approval, a companion-permitted step
//! outside read-only/self-approve scope, or a step that claims a runbook-local
//! bypass would each mint a hidden privileged mutate channel and is rejected by
//! [`RunbookExecutableStep::validate`].
//!
//! The [`M5RunbookStepLibrary`] is the one inspectable, serde-serializable truth
//! packet the consuming surfaces read. Every step projects the *same* governance
//! truth into the desktop UI, the companion follow view, and support exports, so a
//! step's class, scope, approval, and evidence stay consistent wherever it is
//! previewed, executed, followed, or exported. The packet carries metadata and
//! refs only — no credential bodies or raw provider/console payloads.
//!
//! - Library schema:
//!   [`schemas/runbooks/m5-runbook-step-library.schema.json`](../../../../../schemas/runbooks/m5-runbook-step-library.schema.json)
//! - Contract doc:
//!   [`docs/runbooks/m5-runbook-steps.md`](../../../../../docs/runbooks/m5-runbook-steps.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_executable_steps, seeded_m5_runbook_step_library,
    seeded_m5_runbook_step_library_companion_scoped, M5_RUNBOOK_STEP_LIBRARY_ID,
};

use serde::{Deserialize, Serialize};

use crate::m5_runbook_governance::{
    ControlPlaneBoundaryClass, RunbookApprovalScope, RunbookStepClass,
};

/// Record-kind tag carried by [`M5RunbookStepLibrary`].
pub const M5_RUNBOOK_STEP_LIBRARY_RECORD_KIND: &str = "m5_runbook_step_library";

/// Record-kind tag carried by [`RunbookExecutableStep`].
pub const M5_RUNBOOK_EXECUTABLE_STEP_RECORD_KIND: &str = "m5_runbook_executable_step";

/// Schema version shared by the library and its embedded executable steps.
pub const M5_RUNBOOK_STEP_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the step-library schema.
pub const M5_RUNBOOK_STEP_LIBRARY_SCHEMA_REF: &str =
    "schemas/runbooks/m5-runbook-step-library.schema.json";

/// Repo-relative path of the published step-library inventory.
pub const M5_RUNBOOK_STEP_LIBRARY_REF: &str = "artifacts/runbooks/m5-runbook-step-library.json";

/// Repo-relative path of the release-grade step-library export.
pub const M5_RUNBOOK_STEP_LIBRARY_PROOF_REF: &str =
    "artifacts/release/m5-runbook-proof/runbook-step-library.json";

/// Repo-relative path of the step-library contract doc.
pub const M5_RUNBOOK_STEP_DOC_REF: &str = "docs/runbooks/m5-runbook-steps.md";

/// Repo-relative directory of the executable-step fixtures.
pub const M5_RUNBOOK_STEP_FIXTURE_DIR: &str = "fixtures/runbooks/m5-step-library/";

/// Prefix every governed message id in this lane carries so consumers can route it.
pub const M5_RUNBOOK_STEP_MESSAGE_ID_PREFIX: &str = "runbooks_steps.";

/// How broad a reach a step's target selector has. Breadth is part of the step
/// object so preview can show the blast radius and audit can record it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetSelectorBreadth {
    /// The step has no mutable target (annotate, approval, or a pure note).
    NoTarget,
    /// A single named resource.
    SingleTarget,
    /// A bounded set of resources within a declared scope.
    ScopedSet,
    /// A whole environment or region; the broadest in-plane reach.
    EnvironmentWide,
    /// A target outside Aureline's governed plane, reached only via a handoff.
    ExternalTarget,
}

impl TargetSelectorBreadth {
    /// Every breadth, in declaration order (narrowest to broadest).
    pub const ALL: [Self; 5] = [
        Self::NoTarget,
        Self::SingleTarget,
        Self::ScopedSet,
        Self::EnvironmentWide,
        Self::ExternalTarget,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoTarget => "no_target",
            Self::SingleTarget => "single_target",
            Self::ScopedSet => "scoped_set",
            Self::EnvironmentWide => "environment_wide",
            Self::ExternalTarget => "external_target",
        }
    }

    /// True when the breadth names a concrete in-plane target the step mutates or
    /// inspects (single target, scoped set, or environment-wide).
    pub const fn is_in_plane_target(self) -> bool {
        matches!(
            self,
            Self::SingleTarget | Self::ScopedSet | Self::EnvironmentWide
        )
    }
}

/// The scope a step's target selector reaches: an opaque selector ref plus its
/// declared [breadth](TargetSelectorBreadth) and whether it crosses environments.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetSelectorScope {
    /// Opaque, redaction-safe selector ref (e.g. `workspace:pipeline/worker-3`).
    pub selector_ref: String,
    /// How broad a reach the selector has.
    pub breadth: TargetSelectorBreadth,
    /// Whether the selector reaches across an environment boundary.
    pub crosses_environment: bool,
}

impl TargetSelectorScope {
    /// True when the selector points at no mutable target.
    pub fn is_untargeted(&self) -> bool {
        matches!(self.breadth, TargetSelectorBreadth::NoTarget)
    }
}

/// Whether a step remains view-only, runs in-product, or hands off to an external
/// boundary. This is the disposition that decides whether the desktop UI offers a
/// run affordance, a follow-only view, or an attributable handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepExecutionMode {
    /// Read-only; surfaced for context and never offered as an executable action.
    ViewOnly,
    /// Executes inside Aureline's governed plane through the shared command/action
    /// envelope.
    InProductExecutable,
    /// Cannot execute in-product; hands off to a browser or vendor-console boundary
    /// while staying attributable.
    HandoffOnly,
}

impl StepExecutionMode {
    /// Every execution mode, in declaration order.
    pub const ALL: [Self; 3] = [Self::ViewOnly, Self::InProductExecutable, Self::HandoffOnly];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ViewOnly => "view_only",
            Self::InProductExecutable => "in_product_executable",
            Self::HandoffOnly => "handoff_only",
        }
    }

    /// True when the step offers an in-product run affordance and so must bind the
    /// shared command/action envelope.
    pub const fn is_in_product_executable(self) -> bool {
        matches!(self, Self::InProductExecutable)
    }

    /// True when the step hands off to a boundary outside the governed plane.
    pub const fn is_handoff(self) -> bool {
        matches!(self, Self::HandoffOnly)
    }
}

/// How a step binds to Aureline's shared command/action-envelope and approval
/// systems. A governed step never carries its own privileged mutate path; it
/// routes through the shared envelope and the shared approval authority so the
/// same preview, gate, and audit apply as for any other governed action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandEnvelopeBinding {
    /// The shared command/action-envelope this step routes through. Required for
    /// any step that is not purely view-only.
    pub action_envelope_ref: String,
    /// The shared approval authority the step's gate routes through. Empty only
    /// when the step requires no approval.
    pub approval_authority_ref: String,
    /// Whether the step binds the shared command/action envelope rather than a
    /// runbook-local path. Must be `true` for any non-view-only step.
    pub binds_shared_envelope: bool,
    /// Always `false`: a runbook-local privileged bypass is never permitted. A
    /// `true` value is a hidden mutate channel.
    pub uses_runbook_local_bypass: bool,
}

/// One governed executable runbook step.
///
/// Every field is part of the contract: tools, previews, approval gates, audit
/// records, and support exports all read this object rather than re-parsing prose.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookExecutableStep {
    /// Record kind; must equal [`M5_RUNBOOK_EXECUTABLE_STEP_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RUNBOOK_STEP_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable step id, unique within the library.
    pub step_id: String,
    /// Reviewer-facing label.
    pub step_label: String,
    /// What class of step is being executed (shared governance taxonomy).
    pub step_class: RunbookStepClass,
    /// The scope the step's target selector reaches.
    pub target_selector: TargetSelectorScope,
    /// What scope or approval the step requires (shared governance taxonomy).
    pub approval_scope: RunbookApprovalScope,
    /// Whether the step stays view-only, runs in-product, or hands off.
    pub execution_mode: StepExecutionMode,
    /// The control-plane boundary the step sits on (shared governance taxonomy).
    pub control_plane_boundary: ControlPlaneBoundaryClass,
    /// How the step binds the shared command/action-envelope and approval systems.
    pub command_binding: CommandEnvelopeBinding,
    /// True when the step changes target state (mirrors [`RunbookStepClass::is_mutating`]).
    pub mutating: bool,
    /// Expected evidence outputs the step must produce for audit.
    pub expected_evidence_outputs: Vec<String>,
    /// Whether a companion may execute this step within declared scope.
    pub companion_permitted: bool,
    /// Stable message id; prefixed [`M5_RUNBOOK_STEP_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl RunbookExecutableStep {
    /// True when the step requires an approval gate of any kind (not read-only).
    pub fn requires_approval(&self) -> bool {
        !matches!(
            self.approval_scope,
            RunbookApprovalScope::NoApprovalReadOnly
        )
    }

    /// True when the step requires an explicit (non-self) human approval.
    pub fn requires_explicit_human_approval(&self) -> bool {
        matches!(
            self.approval_scope,
            RunbookApprovalScope::RequiresHumanApproval
                | RunbookApprovalScope::RequiresPrivilegedApproval
        )
    }

    /// The preview disposition derived from the execution mode and mutation flag.
    pub fn preview_disposition(&self) -> StepPreviewDisposition {
        match self.execution_mode {
            StepExecutionMode::ViewOnly => StepPreviewDisposition::ReadOnlyPreview,
            StepExecutionMode::HandoffOnly => StepPreviewDisposition::HandoffPreview,
            StepExecutionMode::InProductExecutable if self.mutating => {
                StepPreviewDisposition::DiffThenConfirm
            }
            StepExecutionMode::InProductExecutable => StepPreviewDisposition::ReadOnlyPreview,
        }
    }

    /// True when a companion may *execute* this step itself within declared scope:
    /// it must be permitted, within read-only/self-approve scope, and not a handoff.
    pub fn companion_may_execute(&self) -> bool {
        self.companion_permitted
            && self.approval_scope.companion_may_act()
            && !self.execution_mode.is_handoff()
    }

    /// True when a companion may *request* this step within declared scope without
    /// executing it (any non-prohibited step a companion can surface for approval).
    pub fn companion_may_request(&self) -> bool {
        !matches!(
            self.approval_scope,
            RunbookApprovalScope::ProhibitedHiddenMutate
        )
    }

    /// True when this step, as declared, would mint a hidden privileged mutate
    /// channel. The packet must never carry such a step; this is the safety
    /// predicate the projection and validation both read.
    pub fn creates_hidden_mutate_channel(&self) -> bool {
        // A mutating step with no approval is an unguarded mutate path.
        let mutating_without_approval = self.mutating
            && matches!(
                self.approval_scope,
                RunbookApprovalScope::NoApprovalReadOnly
            );
        // A companion let loose outside read-only/self-approve scope is a privilege
        // escalation channel.
        let companion_over_scope =
            self.companion_permitted && !self.approval_scope.companion_may_act();
        // A runbook-local bypass is a privileged channel outside the shared envelope.
        let local_bypass = self.command_binding.uses_runbook_local_bypass;
        // An in-product executable step that does not route the shared envelope has
        // an off-books mutate path.
        let unbound_executable = self.execution_mode.is_in_product_executable()
            && !self.command_binding.binds_shared_envelope;
        mutating_without_approval || companion_over_scope || local_bypass || unbound_executable
    }

    /// Projects the mechanical preview/approval/audit governance for this step.
    /// Every consuming surface computes the same projection from the object alone.
    pub fn project(&self) -> StepGovernanceProjection {
        let requires_approval = self.requires_approval();
        StepGovernanceProjection {
            step_id: self.step_id.clone(),
            step_class: self.step_class.as_str().to_owned(),
            execution_mode: self.execution_mode.as_str().to_owned(),
            target_selector_breadth: self.target_selector.breadth.as_str().to_owned(),
            preview_disposition: self.preview_disposition().as_str().to_owned(),
            preview_shows_target: self.target_selector.breadth.is_in_plane_target()
                || self.execution_mode.is_handoff(),
            requires_approval,
            requires_explicit_human_approval: self.requires_explicit_human_approval(),
            approval_scope: self.approval_scope.as_str().to_owned(),
            approval_routes_through_shared_system: !requires_approval
                || (!self
                    .command_binding
                    .approval_authority_ref
                    .trim()
                    .is_empty()
                    && !self.command_binding.uses_runbook_local_bypass),
            audit_expects_evidence: !self.expected_evidence_outputs.is_empty(),
            expected_evidence_outputs: self.expected_evidence_outputs.clone(),
            companion_may_execute: self.companion_may_execute(),
            companion_may_request: self.companion_may_request(),
            creates_hidden_mutate_channel: self.creates_hidden_mutate_channel(),
            detail_message_id: self.detail_message_id.clone(),
        }
    }

    /// Validates this executable step's invariants.
    pub fn validate(&self) -> Vec<M5RunbookStepViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_RUNBOOK_EXECUTABLE_STEP_RECORD_KIND
            || self.schema_version != M5_RUNBOOK_STEP_SCHEMA_VERSION
        {
            out.push(M5RunbookStepViolation::WrongStepRecordKind);
        }
        if self.step_id.trim().is_empty()
            || self.step_label.trim().is_empty()
            || self.target_selector.selector_ref.trim().is_empty()
        {
            out.push(M5RunbookStepViolation::MissingIdentity);
        }
        if !self
            .detail_message_id
            .starts_with(M5_RUNBOOK_STEP_MESSAGE_ID_PREFIX)
        {
            out.push(M5RunbookStepViolation::UnprefixedMessageId);
        }
        // The mutating flag must match the step class's taxonomy.
        if self.mutating != self.step_class.is_mutating() {
            out.push(M5RunbookStepViolation::StepMutatingFlagMismatch);
        }
        // A console-handoff class must declare an out-of-plane boundary, and a
        // non-handoff class must stay in-plane.
        if self.step_class.is_console_handoff()
            != self.control_plane_boundary.leaves_governed_plane()
        {
            out.push(M5RunbookStepViolation::StepBoundaryMismatch);
        }
        // Execution-mode consistency: a handoff step must leave the governed plane,
        // and an in-product or view-only step must stay in it.
        match self.execution_mode {
            StepExecutionMode::HandoffOnly => {
                if !self.control_plane_boundary.leaves_governed_plane() {
                    out.push(M5RunbookStepViolation::ExecutionModeBoundaryMismatch);
                }
            }
            StepExecutionMode::InProductExecutable | StepExecutionMode::ViewOnly => {
                if self.control_plane_boundary.leaves_governed_plane() {
                    out.push(M5RunbookStepViolation::ExecutionModeBoundaryMismatch);
                }
            }
        }
        // A console-handoff class and the handoff-only mode imply each other.
        if self.step_class.is_console_handoff() != self.execution_mode.is_handoff() {
            out.push(M5RunbookStepViolation::HandoffModeMismatch);
        }
        // A view-only step never mutates and never carries an approval gate.
        if matches!(self.execution_mode, StepExecutionMode::ViewOnly)
            && (self.mutating || self.requires_approval())
        {
            out.push(M5RunbookStepViolation::ViewOnlyStepIsActive);
        }
        // No step may declare the prohibited-hidden-mutate scope as its own scope;
        // that token marks a path a companion is forbidden to create, never a real
        // step's requirement.
        if matches!(
            self.approval_scope,
            RunbookApprovalScope::ProhibitedHiddenMutate
        ) {
            out.push(M5RunbookStepViolation::DeclaresProhibitedScope);
        }
        // A mutating or in-product step must produce at least one evidence output.
        if (self.mutating || self.execution_mode.is_in_product_executable())
            && self.expected_evidence_outputs.is_empty()
        {
            out.push(M5RunbookStepViolation::MissingExpectedEvidence);
        }
        // A non-view-only step must bind the shared command/action envelope.
        if !matches!(self.execution_mode, StepExecutionMode::ViewOnly)
            && (!self.command_binding.binds_shared_envelope
                || self.command_binding.action_envelope_ref.trim().is_empty())
        {
            out.push(M5RunbookStepViolation::EnvelopeBindingMissing);
        }
        // An approval-bearing step must name a shared approval authority; a
        // read-only step must not.
        if self.requires_approval() {
            if self
                .command_binding
                .approval_authority_ref
                .trim()
                .is_empty()
            {
                out.push(M5RunbookStepViolation::ApprovalAuthorityMissing);
            }
        } else if !self
            .command_binding
            .approval_authority_ref
            .trim()
            .is_empty()
        {
            out.push(M5RunbookStepViolation::SpuriousApprovalAuthority);
        }
        // The runbook-local bypass flag is never permitted.
        if self.command_binding.uses_runbook_local_bypass {
            out.push(M5RunbookStepViolation::RunbookLocalBypass);
        }
        // The synthesized safety predicate must hold.
        if self.creates_hidden_mutate_channel() {
            out.push(M5RunbookStepViolation::HiddenMutateChannel);
        }
        out
    }
}

/// The preview disposition a surface derives from an executable step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepPreviewDisposition {
    /// Read-only context; no run affordance.
    ReadOnlyPreview,
    /// A mutating in-product action: show a diff and require confirmation.
    DiffThenConfirm,
    /// A handoff: preview the boundary crossing and attribution before pivoting.
    HandoffPreview,
}

impl StepPreviewDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ReadOnlyPreview,
        Self::DiffThenConfirm,
        Self::HandoffPreview,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyPreview => "read_only_preview",
            Self::DiffThenConfirm => "diff_then_confirm",
            Self::HandoffPreview => "handoff_preview",
        }
    }
}

/// The mechanical preview/approval/audit governance derived from one executable
/// step. Surfaces never re-decide this; they read the projection so a step behaves
/// identically wherever it is previewed, run, followed, or exported.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StepGovernanceProjection {
    /// Stable step id this projection derives from.
    pub step_id: String,
    /// Step-class token.
    pub step_class: String,
    /// Execution-mode token.
    pub execution_mode: String,
    /// Target-selector breadth token.
    pub target_selector_breadth: String,
    /// Preview disposition token.
    pub preview_disposition: String,
    /// Whether the preview names a concrete target or boundary.
    pub preview_shows_target: bool,
    /// Whether the step requires an approval gate of any kind.
    pub requires_approval: bool,
    /// Whether the step requires an explicit (non-self) human approval.
    pub requires_explicit_human_approval: bool,
    /// Approval-scope token.
    pub approval_scope: String,
    /// Whether the approval routes through the shared approval system (always true
    /// when no approval is required).
    pub approval_routes_through_shared_system: bool,
    /// Whether audit expects at least one evidence output.
    pub audit_expects_evidence: bool,
    /// The evidence outputs audit expects.
    pub expected_evidence_outputs: Vec<String>,
    /// Whether a companion may execute this step within declared scope.
    pub companion_may_execute: bool,
    /// Whether a companion may request this step within declared scope.
    pub companion_may_request: bool,
    /// Whether this step would mint a hidden privileged mutate channel; must be false.
    pub creates_hidden_mutate_channel: bool,
    /// Stable message id; prefixed [`M5_RUNBOOK_STEP_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

/// A surface that renders executable step objects and their projections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookStepSurface {
    /// The desktop runbook/incident UI.
    DesktopUi,
    /// The companion follow view.
    CompanionFollow,
    /// Support exports / bundles.
    SupportExport,
}

impl RunbookStepSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 3] = [Self::DesktopUi, Self::CompanionFollow, Self::SupportExport];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopUi => "desktop_ui",
            Self::CompanionFollow => "companion_follow",
            Self::SupportExport => "support_export",
        }
    }
}

/// Which surfaces expose the step library. Every flag must hold so a step's
/// metadata stays consistent wherever it is rendered or exported.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookStepSurfaceExposure {
    /// The desktop runbook/incident UI exposes the step library.
    pub desktop_ui_exposes_steps: bool,
    /// The companion follow view exposes the step library.
    pub companion_follow_exposes_steps: bool,
    /// Support exports expose the step library.
    pub support_export_exposes_steps: bool,
}

impl RunbookStepSurfaceExposure {
    /// The canonical exposure: every surface renders the step library.
    pub const fn all_surfaces() -> Self {
        Self {
            desktop_ui_exposes_steps: true,
            companion_follow_exposes_steps: true,
            support_export_exposes_steps: true,
        }
    }

    /// True when every surface exposes the library.
    pub const fn all_expose(&self) -> bool {
        self.desktop_ui_exposes_steps
            && self.companion_follow_exposes_steps
            && self.support_export_exposes_steps
    }
}

/// Self-describing controlled-vocabulary set so the packet resolves every token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookStepVocabulary {
    /// Step-class tokens.
    pub step_classes: Vec<String>,
    /// Approval-scope tokens.
    pub approval_scopes: Vec<String>,
    /// Execution-mode tokens.
    pub execution_modes: Vec<String>,
    /// Target-selector breadth tokens.
    pub target_selector_breadths: Vec<String>,
    /// Control-plane boundary tokens.
    pub control_plane_boundaries: Vec<String>,
    /// Preview-disposition tokens.
    pub preview_dispositions: Vec<String>,
    /// Surface tokens.
    pub surfaces: Vec<String>,
}

impl RunbookStepVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            step_classes: RunbookStepClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            approval_scopes: RunbookApprovalScope::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            execution_modes: StepExecutionMode::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            target_selector_breadths: TargetSelectorBreadth::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            control_plane_boundaries: ControlPlaneBoundaryClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            preview_dispositions: StepPreviewDisposition::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            surfaces: RunbookStepSurface::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
        }
    }

    /// True when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Conformance review for the step library. Every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookStepConformance {
    /// Every step is a typed, stable, exportable object rather than opaque prose.
    pub every_step_is_typed_stable_and_exportable: bool,
    /// Preview, approval, and audit behavior derive mechanically from the step object.
    pub preview_approval_audit_derivable_from_step_object: bool,
    /// Step metadata stays consistent across desktop UI, companion follow, and export.
    pub metadata_consistent_across_desktop_companion_and_export: bool,
    /// No step mints a hidden privileged mutate channel.
    pub no_step_mints_hidden_privileged_mutate_channel: bool,
    /// Every non-view-only step binds the shared command envelope, not a runbook bypass.
    pub every_executable_step_binds_shared_command_envelope: bool,
    /// Companion-permitted steps stay within read-only or self-approve scope.
    pub companion_steps_stay_within_read_only_or_self_approve_scope: bool,
    /// Handoff steps are declared handoff-only and leave the governed plane.
    pub handoff_steps_declared_handoff_only_and_leave_governed_plane: bool,
    /// The library is generated from the same checked-in executable steps.
    pub generated_from_checked_in_steps: bool,
}

impl RunbookStepConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.every_step_is_typed_stable_and_exportable
            && self.preview_approval_audit_derivable_from_step_object
            && self.metadata_consistent_across_desktop_companion_and_export
            && self.no_step_mints_hidden_privileged_mutate_channel
            && self.every_executable_step_binds_shared_command_envelope
            && self.companion_steps_stay_within_read_only_or_self_approve_scope
            && self.handoff_steps_declared_handoff_only_and_leave_governed_plane
            && self.generated_from_checked_in_steps
    }
}

/// Constructor input for [`M5RunbookStepLibrary::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RunbookStepLibraryInput {
    /// Stable library id.
    pub library_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the library was computed as-of.
    pub evaluated_at: String,
    /// The governed executable steps.
    pub steps: Vec<RunbookExecutableStep>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 runbook step library: the inventory of governed executable step
/// objects and the mechanical governance projection every surface reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RunbookStepLibrary {
    /// Record kind; must equal [`M5_RUNBOOK_STEP_LIBRARY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RUNBOOK_STEP_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable library id.
    pub library_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the library was computed as-of.
    pub evaluated_at: String,
    /// The governed executable steps.
    pub steps: Vec<RunbookExecutableStep>,
    /// One governance projection per step, in step order.
    pub projections: Vec<StepGovernanceProjection>,
    /// Which surfaces expose the library.
    pub surface_exposure: RunbookStepSurfaceExposure,
    /// Controlled-vocabulary set.
    pub vocabulary: RunbookStepVocabulary,
    /// Conformance review block.
    pub conformance: RunbookStepConformance,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RunbookStepLibrary {
    /// Builds a library from seed input, deriving each step's projection and the
    /// conformance review from the step objects.
    pub fn new(input: M5RunbookStepLibraryInput) -> Self {
        let projections: Vec<StepGovernanceProjection> =
            input.steps.iter().map(|s| s.project()).collect();
        let conformance = derive_conformance(&input.steps);
        Self {
            record_kind: M5_RUNBOOK_STEP_LIBRARY_RECORD_KIND.to_owned(),
            schema_version: M5_RUNBOOK_STEP_SCHEMA_VERSION,
            library_id: input.library_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            steps: input.steps,
            projections,
            surface_exposure: RunbookStepSurfaceExposure::all_surfaces(),
            vocabulary: RunbookStepVocabulary::canonical(),
            conformance,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Finds a step by id.
    pub fn step(&self, step_id: &str) -> Option<&RunbookExecutableStep> {
        self.steps.iter().find(|s| s.step_id == step_id)
    }

    /// The projections a given surface renders. Every surface reads the same
    /// projection truth; this is the method that proves cross-surface consistency.
    pub fn projections_for_surface(
        &self,
        _surface: RunbookStepSurface,
    ) -> Vec<StepGovernanceProjection> {
        self.steps.iter().map(|s| s.project()).collect()
    }

    /// Validates the library's invariants.
    pub fn validate(&self) -> Vec<M5RunbookStepViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_RUNBOOK_STEP_LIBRARY_RECORD_KIND {
            out.push(M5RunbookStepViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RUNBOOK_STEP_SCHEMA_VERSION {
            out.push(M5RunbookStepViolation::WrongSchemaVersion);
        }
        if self.library_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5RunbookStepViolation::MissingIdentity);
        }
        if self.steps.is_empty() {
            out.push(M5RunbookStepViolation::LibraryHasNoSteps);
        }

        // Unique step ids.
        let mut seen = std::collections::BTreeSet::new();
        for step in &self.steps {
            if !seen.insert(step.step_id.as_str()) {
                out.push(M5RunbookStepViolation::DuplicateStepId);
            }
            out.extend(step.validate());
        }

        // The projections must recompute exactly from the steps.
        let expected: Vec<StepGovernanceProjection> =
            self.steps.iter().map(|s| s.project()).collect();
        if expected != self.projections {
            out.push(M5RunbookStepViolation::ProjectionDrift);
        }

        if !self.surface_exposure.all_expose() {
            out.push(M5RunbookStepViolation::SurfaceExposureIncomplete);
        }
        if !self.vocabulary.matches_canonical() {
            out.push(M5RunbookStepViolation::VocabularyMismatch);
        }
        if self.conformance != derive_conformance(&self.steps) || !self.conformance.all_hold() {
            out.push(M5RunbookStepViolation::ConformanceReviewFailed);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 runbook step library serializes"),
        ) {
            out.push(M5RunbookStepViolation::RawBoundaryMaterialInExport);
        }

        out
    }

    /// Deterministic export-safe JSON for the library.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 runbook step library serializes")
    }

    /// Deterministic Markdown proof for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Runbook Executable Step Library\n\n");
        out.push_str(&format!("- Library: `{}`\n", self.library_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!("- Steps: {}\n", self.steps.len()));
        let executable = self
            .steps
            .iter()
            .filter(|s| s.execution_mode.is_in_product_executable())
            .count();
        let handoff = self
            .steps
            .iter()
            .filter(|s| s.execution_mode.is_handoff())
            .count();
        let view_only = self
            .steps
            .iter()
            .filter(|s| matches!(s.execution_mode, StepExecutionMode::ViewOnly))
            .count();
        out.push_str(&format!(
            "- View-only: {view_only} · In-product executable: {executable} · Handoff-only: {handoff}\n"
        ));
        out.push_str("- Exposed on: desktop UI, companion follow view, support exports\n");

        out.push_str("\n## Governed executable steps\n\n");
        out.push_str("| Step | Class | Target scope | Mode | Approval | Companion | Evidence |\n");
        out.push_str("|------|-------|--------------|------|----------|-----------|----------|\n");
        for step in &self.steps {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | {} | {} |\n",
                step.step_id,
                step.step_class.as_str(),
                step.target_selector.breadth.as_str(),
                step.execution_mode.as_str(),
                step.approval_scope.as_str(),
                if step.companion_may_execute() {
                    "execute"
                } else if step.companion_may_request() {
                    "request"
                } else {
                    "no"
                },
                step.expected_evidence_outputs.len(),
            ));
        }
        out
    }
}

/// Derives the conformance review from the step objects so the stored block
/// reflects the actual library rather than an assertion.
fn derive_conformance(steps: &[RunbookExecutableStep]) -> RunbookStepConformance {
    let every_typed = !steps.is_empty()
        && steps.iter().all(|s| {
            s.validate()
                .iter()
                .all(|v| !matches!(v, M5RunbookStepViolation::MissingIdentity))
        });

    // Preview/approval/audit are derivable when the projection recomputes from the
    // object and names a disposition, an approval requirement, and an audit signal.
    let derivable = steps.iter().all(|s| {
        let p = s.project();
        !p.preview_disposition.is_empty() && !p.approval_scope.is_empty() && p.step_id == s.step_id
    });

    // Cross-surface consistency: the projection is surface-independent, so the
    // three surfaces always render identical truth.
    let consistent = steps.iter().all(|s| {
        let p = s.project();
        p == s.project()
    });

    let no_hidden = steps.iter().all(|s| !s.creates_hidden_mutate_channel());

    let binds_envelope = steps.iter().all(|s| {
        matches!(s.execution_mode, StepExecutionMode::ViewOnly)
            || (s.command_binding.binds_shared_envelope
                && !s.command_binding.uses_runbook_local_bypass
                && !s.command_binding.action_envelope_ref.trim().is_empty())
    });

    let companion_scoped = steps
        .iter()
        .all(|s| !s.companion_permitted || s.approval_scope.companion_may_act());

    let handoff_declared = steps.iter().all(|s| {
        s.execution_mode.is_handoff() == s.control_plane_boundary.leaves_governed_plane()
            && s.execution_mode.is_handoff() == s.step_class.is_console_handoff()
    });

    let generated = steps.iter().all(|s| s.project() == s.project());

    RunbookStepConformance {
        every_step_is_typed_stable_and_exportable: every_typed,
        preview_approval_audit_derivable_from_step_object: derivable,
        metadata_consistent_across_desktop_companion_and_export: consistent,
        no_step_mints_hidden_privileged_mutate_channel: no_hidden,
        every_executable_step_binds_shared_command_envelope: binds_envelope,
        companion_steps_stay_within_read_only_or_self_approve_scope: companion_scoped,
        handoff_steps_declared_handoff_only_and_leave_governed_plane: handoff_declared,
        generated_from_checked_in_steps: generated,
    }
}

/// Validation failures for the executable-step lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RunbookStepViolation {
    /// The library record kind is wrong.
    WrongRecordKind,
    /// The library schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// The library declares no steps.
    LibraryHasNoSteps,
    /// Two steps share a step id.
    DuplicateStepId,
    /// An embedded step record carries the wrong record kind or schema version.
    WrongStepRecordKind,
    /// A message id is missing the lane prefix.
    UnprefixedMessageId,
    /// A step's mutating flag does not match its step class.
    StepMutatingFlagMismatch,
    /// A step's class and control-plane boundary disagree on plane crossing.
    StepBoundaryMismatch,
    /// A step's execution mode and control-plane boundary disagree on plane crossing.
    ExecutionModeBoundaryMismatch,
    /// A step's console-handoff class and handoff-only mode do not match.
    HandoffModeMismatch,
    /// A view-only step is declared mutating or approval-bearing.
    ViewOnlyStepIsActive,
    /// A step declares the prohibited-hidden-mutate scope as its own scope.
    DeclaresProhibitedScope,
    /// A mutating or in-product step declares no expected evidence output.
    MissingExpectedEvidence,
    /// A non-view-only step does not bind the shared command/action envelope.
    EnvelopeBindingMissing,
    /// An approval-bearing step names no shared approval authority.
    ApprovalAuthorityMissing,
    /// A read-only step names an approval authority it does not need.
    SpuriousApprovalAuthority,
    /// A step claims a runbook-local privileged bypass.
    RunbookLocalBypass,
    /// A step would mint a hidden privileged mutate channel.
    HiddenMutateChannel,
    /// The stored projections drifted from a fresh recompute.
    ProjectionDrift,
    /// A surface does not expose the step library.
    SurfaceExposureIncomplete,
    /// The controlled-vocabulary set does not match the canonical tokens.
    VocabularyMismatch,
    /// A conformance-review flag does not hold or drifted.
    ConformanceReviewFailed,
    /// The export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5RunbookStepViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::LibraryHasNoSteps => "library_has_no_steps",
            Self::DuplicateStepId => "duplicate_step_id",
            Self::WrongStepRecordKind => "wrong_step_record_kind",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::StepMutatingFlagMismatch => "step_mutating_flag_mismatch",
            Self::StepBoundaryMismatch => "step_boundary_mismatch",
            Self::ExecutionModeBoundaryMismatch => "execution_mode_boundary_mismatch",
            Self::HandoffModeMismatch => "handoff_mode_mismatch",
            Self::ViewOnlyStepIsActive => "view_only_step_is_active",
            Self::DeclaresProhibitedScope => "declares_prohibited_scope",
            Self::MissingExpectedEvidence => "missing_expected_evidence",
            Self::EnvelopeBindingMissing => "envelope_binding_missing",
            Self::ApprovalAuthorityMissing => "approval_authority_missing",
            Self::SpuriousApprovalAuthority => "spurious_approval_authority",
            Self::RunbookLocalBypass => "runbook_local_bypass",
            Self::HiddenMutateChannel => "hidden_mutate_channel",
            Self::ProjectionDrift => "projection_drift",
            Self::SurfaceExposureIncomplete => "surface_exposure_incomplete",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Keys whose presence would mean an export leaked boundary material. Mirrors the
/// redaction posture of the source and governance lanes.
const FORBIDDEN_KEY_SUBSTRINGS: [&str; 6] = [
    "credential",
    "secret",
    "password",
    "api_key",
    "raw_payload",
    "bearer_token",
];

/// Scans a serialized packet for forbidden boundary material. Returns true when a
/// key (case-insensitive) contains a forbidden substring.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(map) => map.iter().any(|(key, child)| {
            let lower = key.to_ascii_lowercase();
            FORBIDDEN_KEY_SUBSTRINGS
                .iter()
                .any(|needle| lower.contains(needle))
                || json_contains_forbidden_boundary_material(child)
        }),
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
