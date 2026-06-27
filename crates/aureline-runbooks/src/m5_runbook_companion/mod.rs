//! Companion-scoped runbook surfaces — how a browser/mobile companion may
//! **follow, acknowledge, comment, request, or hand off** a governed runbook step
//! without ever becoming a hidden privileged mutate channel.
//!
//! The [step library](crate::m5_runbook_steps) freezes every executable step as a
//! typed object whose preview, approval, and audit behavior derive mechanically
//! from the object. This module narrows that same object model to the **companion
//! client scope**: given a governed [`RunbookExecutableStep`](crate::m5_runbook_steps::RunbookExecutableStep),
//! it derives exactly which companion-visible actions are available, which are
//! blocked, and what happens when the companion scope cannot safely execute a step.
//!
//! - A companion may always [`Follow`](CompanionActionClass::Follow),
//!   [`Acknowledge`](CompanionActionClass::Acknowledge), and
//!   [`Comment`](CompanionActionClass::Comment) within the step's declared scope —
//!   these are read-only/attributable and never mutate target state.
//! - When a step is an in-scope mutation the companion is permitted to run
//!   ([`companion_may_execute`](crate::m5_runbook_steps::RunbookExecutableStep::companion_may_execute)),
//!   the companion may [`ExecuteInScope`](CompanionActionClass::ExecuteInScope) and
//!   [`GrantScopedApproval`](CompanionActionClass::GrantScopedApproval) — and that
//!   approval **reuses the same shared approval-authority and action-envelope refs
//!   the desktop path uses**, so an approval taken from a companion creates the same
//!   durable audit/approval objects rather than a parallel companion-only record.
//! - When a step needs an approval the companion may not grant, or is a privileged
//!   mutation or an out-of-plane console/browser pivot, the companion's privileged
//!   mutate channel is [explicitly blocked](CompanionRunbookSurface::privileged_mutate_blocked_on_companion)
//!   and the step degrades to a clear [`HandoffToDesktop`](CompanionActionClass::HandoffToDesktop)
//!   path — never a silent failure and never a misleading claim of parity. The
//!   companion may still surface a [`RequestApproval`](CompanionActionClass::RequestApproval)
//!   (a request, not a grant) so a human on the desktop authority decides.
//!
//! The [`M5RunbookCompanionRegister`] is the one inspectable, serde-serializable
//! truth packet the consuming surfaces read. It embeds the same checked-in step
//! objects, derives one [`CompanionRunbookSurface`] per step, and exposes the same
//! truth identically across the companion app, the desktop incident workspace that
//! receives a handoff, and support exports — so a companion's authority over a
//! runbook step reads the same wherever it is rendered or exported. The packet
//! carries metadata and refs only: no credential bodies or raw provider/console
//! payloads.
//!
//! - Register schema:
//!   [`schemas/runbooks/m5-runbook-companion-register.schema.json`](../../../../../schemas/runbooks/m5-runbook-companion-register.schema.json)
//! - Contract doc:
//!   [`docs/runbooks/m5-runbook-companion.md`](../../../../../docs/runbooks/m5-runbook-companion.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_companion_surfaces, seeded_m5_runbook_companion_register,
    M5_RUNBOOK_COMPANION_REGISTER_ID,
};

use serde::{Deserialize, Serialize};

use crate::m5_runbook_governance::{
    ControlPlaneBoundaryClass, RunbookApprovalScope, RunbookStepClass,
};
use crate::m5_runbook_steps::{RunbookExecutableStep, StepExecutionMode};

/// Record-kind tag carried by [`M5RunbookCompanionRegister`].
pub const M5_RUNBOOK_COMPANION_REGISTER_RECORD_KIND: &str = "m5_runbook_companion_register";

/// Record-kind tag carried by [`CompanionRunbookSurface`].
pub const M5_RUNBOOK_COMPANION_SURFACE_RECORD_KIND: &str = "m5_runbook_companion_surface";

/// Schema version shared by the register and its embedded surfaces.
pub const M5_RUNBOOK_COMPANION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the companion-register schema.
pub const M5_RUNBOOK_COMPANION_REGISTER_SCHEMA_REF: &str =
    "schemas/runbooks/m5-runbook-companion-register.schema.json";

/// Repo-relative path of the published companion-register inventory.
pub const M5_RUNBOOK_COMPANION_REGISTER_REF: &str =
    "artifacts/runbooks/m5-runbook-companion-register.json";

/// Repo-relative path of the release-grade companion-register export.
pub const M5_RUNBOOK_COMPANION_REGISTER_PROOF_REF: &str =
    "artifacts/release/m5-runbook-proof/runbook-companion-register.json";

/// Repo-relative path of the companion-register contract doc.
pub const M5_RUNBOOK_COMPANION_DOC_REF: &str = "docs/runbooks/m5-runbook-companion.md";

/// Repo-relative directory of the per-surface companion fixtures.
pub const M5_RUNBOOK_COMPANION_FIXTURE_DIR: &str = "fixtures/runbooks/m5-companion-surfaces/";

/// Prefix every companion-lane message id carries so consumers can route it.
pub const M5_RUNBOOK_COMPANION_MESSAGE_ID_PREFIX: &str = "runbooks_companion.";

/// A companion-visible action over a governed runbook step. Naming every action
/// explicitly is what stops a companion from silently widening its authority: an
/// action a companion may not take is *named blocked*, never quietly available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionActionClass {
    /// Follow/track the step's execution. Read-only; never mutates target state.
    Follow,
    /// Acknowledge the step within scope. Attributable; never mutates target state.
    Acknowledge,
    /// Add a comment or annotation within scope. Never mutates target state.
    Comment,
    /// Execute the step in-product within the declared companion scope.
    ExecuteInScope,
    /// Grant a scoped self-approve approval from the companion, reusing the shared
    /// approval authority — the same durable object the desktop path creates.
    GrantScopedApproval,
    /// Surface a request for an approval the companion may not grant; the grant and
    /// execution route to the desktop/human authority.
    RequestApproval,
    /// Hand off to the desktop client because the companion scope cannot safely
    /// execute the step.
    HandoffToDesktop,
}

impl CompanionActionClass {
    /// Every action class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Follow,
        Self::Acknowledge,
        Self::Comment,
        Self::ExecuteInScope,
        Self::GrantScopedApproval,
        Self::RequestApproval,
        Self::HandoffToDesktop,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Follow => "follow",
            Self::Acknowledge => "acknowledge",
            Self::Comment => "comment",
            Self::ExecuteInScope => "execute_in_scope",
            Self::GrantScopedApproval => "grant_scoped_approval",
            Self::RequestApproval => "request_approval",
            Self::HandoffToDesktop => "handoff_to_desktop",
        }
    }

    /// True when the action mutates target state or grants a mutating approval — the
    /// class of action a companion may only take strictly within declared scope.
    pub const fn is_privileged_mutate(self) -> bool {
        matches!(self, Self::ExecuteInScope | Self::GrantScopedApproval)
    }
}

/// How a companion client's authority is narrowed for one runbook step. The
/// disposition is derived from the step object alone, so the companion app, the
/// desktop handoff target, and support exports all read the same narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionScopeDisposition {
    /// A read-only step: the companion may follow, acknowledge, and comment within
    /// scope. There is nothing to mutate.
    FollowInScope,
    /// An in-scope mutation the companion is permitted to run: the companion may
    /// execute and grant the scoped self-approve gate, reusing the shared approval
    /// authority and action envelope the desktop path uses.
    ActInScope,
    /// Out of companion scope: the step's approval or mutation cannot be granted or
    /// executed on the companion. The companion may follow/acknowledge/comment and
    /// surface a request, but the privileged mutate channel is blocked and the step
    /// degrades to an explicit desktop handoff.
    DesktopHandoffRequired,
}

impl CompanionScopeDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::FollowInScope,
        Self::ActInScope,
        Self::DesktopHandoffRequired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FollowInScope => "follow_in_scope",
            Self::ActInScope => "act_in_scope",
            Self::DesktopHandoffRequired => "desktop_handoff_required",
        }
    }

    /// True when the companion may execute the step within its declared scope.
    pub const fn allows_in_scope_action(self) -> bool {
        matches!(self, Self::ActInScope)
    }

    /// True when the step degrades to an explicit desktop handoff because the
    /// companion scope cannot safely execute it.
    pub const fn requires_desktop_handoff(self) -> bool {
        matches!(self, Self::DesktopHandoffRequired)
    }
}

/// A surface that renders the companion runbook register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionRunbookSurfaceKind {
    /// The browser/mobile companion app.
    CompanionApp,
    /// The desktop incident workspace that receives a handoff.
    DesktopHandoffTarget,
    /// Support exports / bundles.
    SupportExport,
}

impl CompanionRunbookSurfaceKind {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::CompanionApp,
        Self::DesktopHandoffTarget,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompanionApp => "companion_app",
            Self::DesktopHandoffTarget => "desktop_handoff_target",
            Self::SupportExport => "support_export",
        }
    }
}

/// One companion-scoped surface for a governed runbook step: the actions a
/// companion may take, the actions explicitly blocked, the desktop refs reused when
/// a companion-allowed approval occurs, and the desktop handoff path when the
/// companion scope cannot safely execute.
///
/// Every field is derived from the source [`RunbookExecutableStep`], so the surface
/// can never grant a companion more authority than the governed step declares.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionRunbookSurface {
    /// Record kind; must equal [`M5_RUNBOOK_COMPANION_SURFACE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RUNBOOK_COMPANION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable step id this surface narrows.
    pub step_id: String,
    /// Reviewer-facing step label.
    pub step_label: String,
    /// Step-class token (shared governance taxonomy).
    pub step_class: RunbookStepClass,
    /// Approval-scope token (shared governance taxonomy).
    pub approval_scope: RunbookApprovalScope,
    /// Execution-mode token (shared step taxonomy).
    pub execution_mode: StepExecutionMode,
    /// Control-plane boundary token (shared governance taxonomy).
    pub control_plane_boundary: ControlPlaneBoundaryClass,
    /// How the companion's authority is narrowed for this step.
    pub scope_disposition: CompanionScopeDisposition,
    /// The companion-visible actions available on this step, in canonical order.
    pub available_actions: Vec<CompanionActionClass>,
    /// The companion-visible actions explicitly blocked on this step, in canonical
    /// order. A blocked privileged mutate is named, never silently dropped.
    pub blocked_actions: Vec<CompanionActionClass>,
    /// The shared approval-authority ref the companion reuses when it grants the
    /// scoped self-approve gate. `Some` only when the companion may grant in scope;
    /// it is byte-identical to the desktop step's approval authority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reused_approval_authority_ref: Option<String>,
    /// The shared command/action-envelope ref the companion reuses when it executes
    /// in scope. `Some` only when the companion may execute in scope; it is
    /// byte-identical to the desktop step's action envelope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reused_action_envelope_ref: Option<String>,
    /// The desktop approval authority a request routes to when the companion may not
    /// grant the approval itself. `Some` only when the step degrades to a handoff and
    /// the desktop path carries an approval authority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desktop_approval_authority_ref: Option<String>,
    /// Message id naming the clear desktop handoff path. `Some` only when the step
    /// degrades to a desktop handoff.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desktop_handoff_message_id: Option<String>,
    /// Whether the companion may execute this step within declared scope.
    pub companion_may_execute: bool,
    /// Whether the companion may grant the scoped self-approve gate within scope.
    pub companion_may_grant_approval: bool,
    /// Whether the companion may surface a request for this step (true unless the
    /// step declares a prohibited hidden-mutate scope).
    pub companion_may_request: bool,
    /// Whether the privileged mutate channel is blocked on the companion and the
    /// step degrades to a desktop handoff. The explicit marker the spec requires.
    pub privileged_mutate_blocked_on_companion: bool,
    /// Whether this surface would let a companion mutate beyond declared scope; must
    /// be false. The safety predicate the projection and validation both read.
    pub creates_hidden_mutate_channel: bool,
    /// Stable message id; prefixed [`M5_RUNBOOK_COMPANION_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl CompanionRunbookSurface {
    /// Derives the companion-scoped surface for one governed executable step. The
    /// derivation reads only the step object, so the companion surface can never
    /// widen the step's declared authority.
    pub fn derive(step: &RunbookExecutableStep) -> Self {
        let companion_may_request = step.companion_may_request();
        let requires_approval = step.requires_approval();
        // The companion may run an *in-product action* only when the step permits it
        // and it is actually an in-product action — a view-only step has nothing to
        // execute, so it follows in scope rather than acting.
        let companion_may_execute =
            step.companion_may_execute() && step.execution_mode.is_in_product_executable();

        // A view-only step is read-only context: follow, acknowledge, comment. Any
        // step the companion may execute is in-scope. Everything else — a human or
        // privileged approval the companion may not grant, or an out-of-plane pivot
        // — degrades to a desktop handoff.
        let scope_disposition = if matches!(step.execution_mode, StepExecutionMode::ViewOnly) {
            CompanionScopeDisposition::FollowInScope
        } else if companion_may_execute {
            CompanionScopeDisposition::ActInScope
        } else {
            CompanionScopeDisposition::DesktopHandoffRequired
        };

        let companion_may_grant_approval =
            scope_disposition.allows_in_scope_action() && requires_approval;

        // Follow / acknowledge / comment are always available within scope.
        let mut available = vec![
            CompanionActionClass::Follow,
            CompanionActionClass::Acknowledge,
            CompanionActionClass::Comment,
        ];
        let mut blocked = Vec::new();

        match scope_disposition {
            CompanionScopeDisposition::FollowInScope => {
                // Nothing to mutate; no privileged action is offered or blocked.
            }
            CompanionScopeDisposition::ActInScope => {
                available.push(CompanionActionClass::ExecuteInScope);
                if companion_may_grant_approval {
                    available.push(CompanionActionClass::GrantScopedApproval);
                }
            }
            CompanionScopeDisposition::DesktopHandoffRequired => {
                // The privileged mutate channels are named blocked, then the step
                // degrades to a desktop handoff. A request (not a grant) may still be
                // surfaced when the step is not a prohibited hidden-mutate path.
                blocked.push(CompanionActionClass::ExecuteInScope);
                if requires_approval {
                    blocked.push(CompanionActionClass::GrantScopedApproval);
                }
                if companion_may_request {
                    available.push(CompanionActionClass::RequestApproval);
                } else {
                    blocked.push(CompanionActionClass::RequestApproval);
                }
                available.push(CompanionActionClass::HandoffToDesktop);
            }
        }

        let reused_approval_authority_ref = if companion_may_grant_approval {
            Some(step.command_binding.approval_authority_ref.clone())
        } else {
            None
        };
        let reused_action_envelope_ref = if scope_disposition.allows_in_scope_action() {
            Some(step.command_binding.action_envelope_ref.clone())
        } else {
            None
        };
        let desktop_approval_authority_ref = if scope_disposition.requires_desktop_handoff()
            && requires_approval
            && !step
                .command_binding
                .approval_authority_ref
                .trim()
                .is_empty()
        {
            Some(step.command_binding.approval_authority_ref.clone())
        } else {
            None
        };
        let desktop_handoff_message_id = if scope_disposition.requires_desktop_handoff() {
            Some(format!(
                "{}handoff.{}",
                M5_RUNBOOK_COMPANION_MESSAGE_ID_PREFIX, step.step_id
            ))
        } else {
            None
        };

        let privileged_mutate_blocked_on_companion = scope_disposition.requires_desktop_handoff();

        // A companion surface mints a hidden mutate channel if the source step does,
        // or if it would offer a privileged action the step does not actually permit.
        let offers_unpermitted_execute =
            available.contains(&CompanionActionClass::ExecuteInScope) && !companion_may_execute;
        let offers_unpermitted_grant = available
            .contains(&CompanionActionClass::GrantScopedApproval)
            && !companion_may_grant_approval;
        let creates_hidden_mutate_channel = step.creates_hidden_mutate_channel()
            || offers_unpermitted_execute
            || offers_unpermitted_grant;

        Self {
            record_kind: M5_RUNBOOK_COMPANION_SURFACE_RECORD_KIND.to_owned(),
            schema_version: M5_RUNBOOK_COMPANION_SCHEMA_VERSION,
            step_id: step.step_id.clone(),
            step_label: step.step_label.clone(),
            step_class: step.step_class,
            approval_scope: step.approval_scope,
            execution_mode: step.execution_mode,
            control_plane_boundary: step.control_plane_boundary,
            scope_disposition,
            available_actions: available,
            blocked_actions: blocked,
            reused_approval_authority_ref,
            reused_action_envelope_ref,
            desktop_approval_authority_ref,
            desktop_handoff_message_id,
            companion_may_execute,
            companion_may_grant_approval,
            companion_may_request,
            privileged_mutate_blocked_on_companion,
            creates_hidden_mutate_channel,
            detail_message_id: format!(
                "{}surface.{}",
                M5_RUNBOOK_COMPANION_MESSAGE_ID_PREFIX, step.step_id
            ),
        }
    }

    /// True when this surface offers a given companion action.
    pub fn offers(&self, action: CompanionActionClass) -> bool {
        self.available_actions.contains(&action)
    }

    /// True when this surface explicitly blocks a given companion action.
    pub fn blocks(&self, action: CompanionActionClass) -> bool {
        self.blocked_actions.contains(&action)
    }

    /// Validates this companion surface's invariants.
    pub fn validate(&self) -> Vec<M5RunbookCompanionViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_RUNBOOK_COMPANION_SURFACE_RECORD_KIND
            || self.schema_version != M5_RUNBOOK_COMPANION_SCHEMA_VERSION
        {
            out.push(M5RunbookCompanionViolation::WrongSurfaceRecordKind);
        }
        if self.step_id.trim().is_empty() || self.step_label.trim().is_empty() {
            out.push(M5RunbookCompanionViolation::MissingIdentity);
        }
        if !self
            .detail_message_id
            .starts_with(M5_RUNBOOK_COMPANION_MESSAGE_ID_PREFIX)
            || self
                .desktop_handoff_message_id
                .as_deref()
                .is_some_and(|m| !m.starts_with(M5_RUNBOOK_COMPANION_MESSAGE_ID_PREFIX))
        {
            out.push(M5RunbookCompanionViolation::UnprefixedMessageId);
        }

        // Follow / acknowledge / comment are always available within scope.
        for required in [
            CompanionActionClass::Follow,
            CompanionActionClass::Acknowledge,
            CompanionActionClass::Comment,
        ] {
            if !self.offers(required) {
                out.push(M5RunbookCompanionViolation::FollowAcknowledgeNotInScope);
            }
        }

        // No action may be both available and blocked.
        if self
            .available_actions
            .iter()
            .any(|a| self.blocked_actions.contains(a))
        {
            out.push(M5RunbookCompanionViolation::ActionAvailableAndBlocked);
        }

        // A companion may only execute / grant when the step actually permits it.
        if self.offers(CompanionActionClass::ExecuteInScope) && !self.companion_may_execute {
            out.push(M5RunbookCompanionViolation::ExecuteOfferedOutsideScope);
        }
        if self.offers(CompanionActionClass::GrantScopedApproval)
            && !self.companion_may_grant_approval
        {
            out.push(M5RunbookCompanionViolation::ApprovalGrantedOutsideScope);
        }

        // A companion-granted approval must reuse the desktop refs; a non-granting
        // surface must not carry reuse refs (no parallel companion-only objects).
        if self.companion_may_grant_approval {
            if self
                .reused_approval_authority_ref
                .as_deref()
                .map(|r| r.trim().is_empty())
                .unwrap_or(true)
            {
                out.push(M5RunbookCompanionViolation::ApprovalReuseMissing);
            }
        } else if self.reused_approval_authority_ref.is_some() {
            out.push(M5RunbookCompanionViolation::SpuriousApprovalReuse);
        }
        // The action envelope is reused exactly when the companion acts in scope.
        if self.scope_disposition.allows_in_scope_action() {
            if self
                .reused_action_envelope_ref
                .as_deref()
                .map(|r| r.trim().is_empty())
                .unwrap_or(true)
            {
                out.push(M5RunbookCompanionViolation::ActionEnvelopeReuseMissing);
            }
        } else if self.reused_action_envelope_ref.is_some() {
            out.push(M5RunbookCompanionViolation::SpuriousActionEnvelopeReuse);
        }

        // A blocked privileged mutate must degrade to a clear desktop handoff, and a
        // handoff state must mark the privileged mutate blocked — the two imply each
        // other so a block can never be a silent failure.
        if self.scope_disposition.requires_desktop_handoff() {
            if !self.privileged_mutate_blocked_on_companion {
                out.push(M5RunbookCompanionViolation::BlockedActionNotMarked);
            }
            if self.desktop_handoff_message_id.is_none()
                || !self.offers(CompanionActionClass::HandoffToDesktop)
            {
                out.push(M5RunbookCompanionViolation::BlockedActionMissingHandoff);
            }
        } else {
            if self.privileged_mutate_blocked_on_companion {
                out.push(M5RunbookCompanionViolation::BlockedActionNotMarked);
            }
            if self.desktop_handoff_message_id.is_some()
                || self.offers(CompanionActionClass::HandoffToDesktop)
            {
                out.push(M5RunbookCompanionViolation::SpuriousHandoff);
            }
        }

        if self.creates_hidden_mutate_channel {
            out.push(M5RunbookCompanionViolation::HiddenMutateChannel);
        }

        out
    }
}

/// Which surfaces expose the companion register. Every flag must hold so a
/// companion's authority over a step reads identically wherever it is rendered.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionRunbookSurfaceExposure {
    /// The companion app exposes the register.
    pub companion_app_exposes_surfaces: bool,
    /// The desktop handoff target exposes the register.
    pub desktop_handoff_target_exposes_surfaces: bool,
    /// Support exports expose the register.
    pub support_export_exposes_surfaces: bool,
}

impl CompanionRunbookSurfaceExposure {
    /// The canonical exposure: every surface renders the register.
    pub const fn all_surfaces() -> Self {
        Self {
            companion_app_exposes_surfaces: true,
            desktop_handoff_target_exposes_surfaces: true,
            support_export_exposes_surfaces: true,
        }
    }

    /// True when every surface exposes the register.
    pub const fn all_expose(&self) -> bool {
        self.companion_app_exposes_surfaces
            && self.desktop_handoff_target_exposes_surfaces
            && self.support_export_exposes_surfaces
    }
}

/// Self-describing controlled-vocabulary set so the packet resolves every token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionRunbookVocabulary {
    /// Companion-action tokens.
    pub action_classes: Vec<String>,
    /// Scope-disposition tokens.
    pub scope_dispositions: Vec<String>,
    /// Step-class tokens.
    pub step_classes: Vec<String>,
    /// Approval-scope tokens.
    pub approval_scopes: Vec<String>,
    /// Execution-mode tokens.
    pub execution_modes: Vec<String>,
    /// Control-plane boundary tokens.
    pub control_plane_boundaries: Vec<String>,
    /// Surface tokens.
    pub surfaces: Vec<String>,
}

impl CompanionRunbookVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            action_classes: CompanionActionClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            scope_dispositions: CompanionScopeDisposition::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
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
            control_plane_boundaries: ControlPlaneBoundaryClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            surfaces: CompanionRunbookSurfaceKind::ALL
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

/// Conformance review for the companion register. Every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionRunbookConformance {
    /// Every companion surface offers follow/acknowledge/comment within scope.
    pub follow_acknowledge_available_within_scope: bool,
    /// No companion surface lets a companion mutate beyond its declared scope.
    pub companion_never_mutates_beyond_scope: bool,
    /// Every companion-granted approval reuses the same desktop approval/audit refs.
    pub companion_approval_reuses_desktop_objects: bool,
    /// Every blocked privileged action degrades to a clear desktop handoff state.
    pub blocked_actions_degrade_to_desktop_handoff: bool,
    /// No companion surface mints a hidden privileged mutate channel.
    pub no_companion_surface_mints_hidden_mutate_channel: bool,
    /// Every embedded step is companion-scoped from the same checked-in step object.
    pub surfaces_derived_from_checked_in_steps: bool,
    /// The export carries no raw boundary material.
    pub export_carries_no_raw_boundary_material: bool,
}

impl CompanionRunbookConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.follow_acknowledge_available_within_scope
            && self.companion_never_mutates_beyond_scope
            && self.companion_approval_reuses_desktop_objects
            && self.blocked_actions_degrade_to_desktop_handoff
            && self.no_companion_surface_mints_hidden_mutate_channel
            && self.surfaces_derived_from_checked_in_steps
            && self.export_carries_no_raw_boundary_material
    }
}

/// Constructor input for [`M5RunbookCompanionRegister::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RunbookCompanionRegisterInput {
    /// Stable register id.
    pub register_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the register was computed as-of.
    pub evaluated_at: String,
    /// The governed executable steps the companion surfaces narrow.
    pub steps: Vec<RunbookExecutableStep>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 runbook companion register: the inventory of governed executable
/// steps, narrowed to the companion client scope, and the per-step companion surface
/// every consuming surface reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RunbookCompanionRegister {
    /// Record kind; must equal [`M5_RUNBOOK_COMPANION_REGISTER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RUNBOOK_COMPANION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable register id.
    pub register_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the register was computed as-of.
    pub evaluated_at: String,
    /// The governed executable steps the surfaces narrow.
    pub steps: Vec<RunbookExecutableStep>,
    /// One companion surface per step, in step order.
    pub surfaces: Vec<CompanionRunbookSurface>,
    /// Which surfaces expose the register.
    pub surface_exposure: CompanionRunbookSurfaceExposure,
    /// Controlled-vocabulary set.
    pub vocabulary: CompanionRunbookVocabulary,
    /// Conformance review block.
    pub conformance: CompanionRunbookConformance,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RunbookCompanionRegister {
    /// Builds a register from seed input, deriving each step's companion surface and
    /// the conformance review from the step objects.
    pub fn new(input: M5RunbookCompanionRegisterInput) -> Self {
        let surfaces: Vec<CompanionRunbookSurface> = input
            .steps
            .iter()
            .map(CompanionRunbookSurface::derive)
            .collect();
        let conformance = derive_conformance(&input.steps, &surfaces);
        Self {
            record_kind: M5_RUNBOOK_COMPANION_REGISTER_RECORD_KIND.to_owned(),
            schema_version: M5_RUNBOOK_COMPANION_SCHEMA_VERSION,
            register_id: input.register_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            steps: input.steps,
            surfaces,
            surface_exposure: CompanionRunbookSurfaceExposure::all_surfaces(),
            vocabulary: CompanionRunbookVocabulary::canonical(),
            conformance,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Finds a companion surface by step id.
    pub fn surface(&self, step_id: &str) -> Option<&CompanionRunbookSurface> {
        self.surfaces.iter().find(|s| s.step_id == step_id)
    }

    /// The surfaces a given consuming surface renders. Every surface reads the same
    /// truth; this is the method that proves cross-surface consistency.
    pub fn surfaces_for(
        &self,
        _surface: CompanionRunbookSurfaceKind,
    ) -> Vec<CompanionRunbookSurface> {
        self.steps
            .iter()
            .map(CompanionRunbookSurface::derive)
            .collect()
    }

    /// Validates the register's invariants.
    pub fn validate(&self) -> Vec<M5RunbookCompanionViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_RUNBOOK_COMPANION_REGISTER_RECORD_KIND {
            out.push(M5RunbookCompanionViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RUNBOOK_COMPANION_SCHEMA_VERSION {
            out.push(M5RunbookCompanionViolation::WrongSchemaVersion);
        }
        if self.register_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5RunbookCompanionViolation::MissingIdentity);
        }
        if self.steps.is_empty() {
            out.push(M5RunbookCompanionViolation::RegisterHasNoSteps);
        }

        // Unique step ids, and every embedded step passes step-library validation.
        let mut seen = std::collections::BTreeSet::new();
        for step in &self.steps {
            if !seen.insert(step.step_id.as_str()) {
                out.push(M5RunbookCompanionViolation::DuplicateStepId);
            }
            if !step.validate().is_empty() {
                out.push(M5RunbookCompanionViolation::EmbeddedStepInvalid);
            }
        }

        for surface in &self.surfaces {
            out.extend(surface.validate());
        }

        // The surfaces must recompute exactly from the steps.
        let expected: Vec<CompanionRunbookSurface> = self
            .steps
            .iter()
            .map(CompanionRunbookSurface::derive)
            .collect();
        if expected != self.surfaces {
            out.push(M5RunbookCompanionViolation::SurfaceDrift);
        }

        if !self.surface_exposure.all_expose() {
            out.push(M5RunbookCompanionViolation::SurfaceExposureIncomplete);
        }
        if !self.vocabulary.matches_canonical() {
            out.push(M5RunbookCompanionViolation::VocabularyMismatch);
        }
        if self.conformance != derive_conformance(&self.steps, &self.surfaces)
            || !self.conformance.all_hold()
        {
            out.push(M5RunbookCompanionViolation::ConformanceReviewFailed);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 runbook companion register serializes"),
        ) {
            out.push(M5RunbookCompanionViolation::RawBoundaryMaterialInExport);
        }

        out
    }

    /// Deterministic export-safe JSON for the register.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 runbook companion register serializes")
    }

    /// Deterministic Markdown proof for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Runbook Companion-Scoped Surface Register\n\n");
        out.push_str(&format!("- Register: `{}`\n", self.register_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!("- Steps: {}\n", self.steps.len()));
        let act = self
            .surfaces
            .iter()
            .filter(|s| s.scope_disposition.allows_in_scope_action())
            .count();
        let handoff = self
            .surfaces
            .iter()
            .filter(|s| s.privileged_mutate_blocked_on_companion)
            .count();
        let follow = self
            .surfaces
            .iter()
            .filter(|s| {
                matches!(
                    s.scope_disposition,
                    CompanionScopeDisposition::FollowInScope
                )
            })
            .count();
        out.push_str(&format!(
            "- Follow-in-scope: {follow} · Act-in-scope: {act} · Desktop-handoff-required: {handoff}\n"
        ));
        out.push_str("- Exposed on: companion app, desktop handoff target, support exports\n");

        out.push_str("\n## Companion-scoped step surfaces\n\n");
        out.push_str(
            "| Step | Class | Scope disposition | Available | Blocked | Reuses desktop approval | Desktop handoff |\n",
        );
        out.push_str(
            "|------|-------|-------------------|-----------|---------|-------------------------|-----------------|\n",
        );
        for s in &self.surfaces {
            let available: Vec<&str> = s.available_actions.iter().map(|a| a.as_str()).collect();
            let blocked: Vec<&str> = s.blocked_actions.iter().map(|a| a.as_str()).collect();
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | {} | {} | {} | {} |\n",
                s.step_id,
                s.step_class.as_str(),
                s.scope_disposition.as_str(),
                if available.is_empty() {
                    "—".to_owned()
                } else {
                    available.join(", ")
                },
                if blocked.is_empty() {
                    "—".to_owned()
                } else {
                    blocked.join(", ")
                },
                if s.companion_may_grant_approval {
                    "yes"
                } else {
                    "no"
                },
                if s.privileged_mutate_blocked_on_companion {
                    "required"
                } else {
                    "—"
                },
            ));
        }
        out
    }
}

/// Derives the conformance review from the step objects and surfaces so the stored
/// block reflects the actual register rather than an assertion.
fn derive_conformance(
    steps: &[RunbookExecutableStep],
    surfaces: &[CompanionRunbookSurface],
) -> CompanionRunbookConformance {
    let follow_ack = !surfaces.is_empty()
        && surfaces.iter().all(|s| {
            s.offers(CompanionActionClass::Follow)
                && s.offers(CompanionActionClass::Acknowledge)
                && s.offers(CompanionActionClass::Comment)
        });

    // A companion may only ever execute / grant when the underlying step permits it.
    let never_over_scope = surfaces.iter().all(|s| {
        (!s.offers(CompanionActionClass::ExecuteInScope) || s.companion_may_execute)
            && (!s.offers(CompanionActionClass::GrantScopedApproval)
                || s.companion_may_grant_approval)
    });

    // Every companion-granted approval reuses the byte-identical desktop refs.
    let approval_reuses_desktop = steps.iter().zip(surfaces.iter()).all(|(step, surface)| {
        if surface.companion_may_grant_approval {
            surface.reused_approval_authority_ref.as_deref()
                == Some(step.command_binding.approval_authority_ref.as_str())
                && surface.reused_action_envelope_ref.as_deref()
                    == Some(step.command_binding.action_envelope_ref.as_str())
        } else {
            surface.reused_approval_authority_ref.is_none()
        }
    });

    // Every blocked privileged action degrades to a clear desktop handoff state.
    let blocked_degrade = surfaces.iter().all(|s| {
        !s.privileged_mutate_blocked_on_companion
            || (s.desktop_handoff_message_id.is_some()
                && s.offers(CompanionActionClass::HandoffToDesktop)
                && s.scope_disposition.requires_desktop_handoff())
    });

    let no_hidden = surfaces.iter().all(|s| !s.creates_hidden_mutate_channel);

    // The surfaces recompute from the same checked-in steps.
    let derived: Vec<CompanionRunbookSurface> =
        steps.iter().map(CompanionRunbookSurface::derive).collect();
    let from_steps =
        !steps.is_empty() && derived == surfaces && steps.iter().all(|s| s.validate().is_empty());

    let export_clean = surfaces.iter().all(|s| !s.creates_hidden_mutate_channel);

    CompanionRunbookConformance {
        follow_acknowledge_available_within_scope: follow_ack,
        companion_never_mutates_beyond_scope: never_over_scope,
        companion_approval_reuses_desktop_objects: approval_reuses_desktop,
        blocked_actions_degrade_to_desktop_handoff: blocked_degrade,
        no_companion_surface_mints_hidden_mutate_channel: no_hidden,
        surfaces_derived_from_checked_in_steps: from_steps,
        export_carries_no_raw_boundary_material: export_clean,
    }
}

/// Validation failures for the companion-register lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RunbookCompanionViolation {
    /// The register record kind is wrong.
    WrongRecordKind,
    /// The register schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// The register declares no steps.
    RegisterHasNoSteps,
    /// Two steps share a step id.
    DuplicateStepId,
    /// An embedded step record carries the wrong record kind or schema version.
    WrongSurfaceRecordKind,
    /// An embedded executable step failed step-library validation.
    EmbeddedStepInvalid,
    /// A message id is missing the lane prefix.
    UnprefixedMessageId,
    /// A surface does not offer follow/acknowledge/comment within scope.
    FollowAcknowledgeNotInScope,
    /// A surface lists the same action as both available and blocked.
    ActionAvailableAndBlocked,
    /// A surface offers execute-in-scope the underlying step does not permit.
    ExecuteOfferedOutsideScope,
    /// A surface offers an approval grant the underlying step does not permit.
    ApprovalGrantedOutsideScope,
    /// A companion-granting surface does not reuse the shared approval authority.
    ApprovalReuseMissing,
    /// A non-granting surface carries an approval-authority reuse it does not need.
    SpuriousApprovalReuse,
    /// An in-scope-acting surface does not reuse the shared command/action envelope.
    ActionEnvelopeReuseMissing,
    /// A non-acting surface carries an action-envelope reuse it does not need.
    SpuriousActionEnvelopeReuse,
    /// A blocked privileged action is not marked blocked on the companion.
    BlockedActionNotMarked,
    /// A blocked privileged action does not degrade to a clear desktop handoff.
    BlockedActionMissingHandoff,
    /// An in-scope surface carries a desktop handoff it does not need.
    SpuriousHandoff,
    /// A surface would mint a hidden privileged mutate channel.
    HiddenMutateChannel,
    /// The stored surfaces drifted from a fresh recompute.
    SurfaceDrift,
    /// A surface does not expose the register.
    SurfaceExposureIncomplete,
    /// The controlled-vocabulary set does not match the canonical tokens.
    VocabularyMismatch,
    /// A conformance-review flag does not hold or drifted.
    ConformanceReviewFailed,
    /// The export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5RunbookCompanionViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::RegisterHasNoSteps => "register_has_no_steps",
            Self::DuplicateStepId => "duplicate_step_id",
            Self::WrongSurfaceRecordKind => "wrong_surface_record_kind",
            Self::EmbeddedStepInvalid => "embedded_step_invalid",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::FollowAcknowledgeNotInScope => "follow_acknowledge_not_in_scope",
            Self::ActionAvailableAndBlocked => "action_available_and_blocked",
            Self::ExecuteOfferedOutsideScope => "execute_offered_outside_scope",
            Self::ApprovalGrantedOutsideScope => "approval_granted_outside_scope",
            Self::ApprovalReuseMissing => "approval_reuse_missing",
            Self::SpuriousApprovalReuse => "spurious_approval_reuse",
            Self::ActionEnvelopeReuseMissing => "action_envelope_reuse_missing",
            Self::SpuriousActionEnvelopeReuse => "spurious_action_envelope_reuse",
            Self::BlockedActionNotMarked => "blocked_action_not_marked",
            Self::BlockedActionMissingHandoff => "blocked_action_missing_handoff",
            Self::SpuriousHandoff => "spurious_handoff",
            Self::HiddenMutateChannel => "hidden_mutate_channel",
            Self::SurfaceDrift => "surface_drift",
            Self::SurfaceExposureIncomplete => "surface_exposure_incomplete",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Keys whose presence would mean an export leaked boundary material. Mirrors the
/// redaction posture of the source, step, and handoff lanes.
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
