//! Keyboard / screen-reader / high-zoom / high-contrast / CLI / export parity, and honest automatic claim
//! narrowing for the M5 shared terminal / debug view / control grant / presenter token / consent envelope /
//! retention review / session-restore-view objects.
//!
//! This module is the M05-1312 accessibility-and-auto-narrowing capstone over the frozen M5 shared terminal /
//! debug control-grant, presenter, consent, retention, and session-restore matrix
//! ([`crate::m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix`]).
//! Where the freeze matrix defines the reusable shared terminal / debug view, control grant, presenter token,
//! consent envelope, retention review, and session-restore-view objects, and the 1305-1311 implementation
//! lanes resolve their per-surface truth, this lane certifies — per object class — that control-authority,
//! grant, presenter-handoff, consent, retention, and restore claims stay **keyboard-complete,
//! assistive-tech-reachable, high-zoom / high-contrast-safe, CLI/export-safe, and self-narrowing** rather than
//! presenting an unresolved / presence-implied control authority, an unprovable or contended single active
//! driver, an unprovable / contested presenter handoff, an undisclosed / silently widening consent scope, a
//! stale / silently broadened retention state, or an unprovable replay-free restore as still a fully
//! explicitly-granted, single-driver controlled surface:
//!
//! - **Keyboard / screen-reader / high-zoom / high-contrast / CLI reach.** Every object exposes a
//!   keyboard-complete, screen-reader-reachable, high-zoom-legible, high-contrast-safe, and
//!   CLI/headless-reachable path into the same object identity, control-authority source, single active
//!   driver, presenter holder / handoff chain, join-time consent scope, recording / retention state, and
//!   restore transcript class the rich object shows — never a color-only control badge, a hover-only
//!   active-driver pill, or a pointer-only grant affordance that strands assistive-tech or headless-CLI users.
//!   Structure-heavy objects (the presenter token's presenter holder / handoff chain, the session-restore
//!   view's restore transcript / replay-free render summary set) additionally bind their structured layout to
//!   a flat list / textual path.
//! - **Export parity.** The support / CLI / release export reconstructs each object's meaning from typed
//!   tokens and opaque refs **without a raw payload**, preserving the same control-authority, active-driver,
//!   presenter-handoff, consent-scope, retention-state, and restore-replay-safety labels visible in-product so
//!   support, help, and release proof can reconstruct exactly what the user was shown without leaking a raw
//!   secret, command text, variable body, clipboard content, endpoint, or provider token.
//! - **Honest auto-narrowing.** When a shared terminal / debug view's control authority is unresolved or
//!   presence-implied, a control grant's single active driver is unprovable or contended, a presenter token's
//!   handoff is unprovable or contested, a consent envelope's join-time scope is undisclosed or would widen
//!   silently, a retention review's recording / retention state is stale or would broaden silently, or a
//!   session-restore view's replay-free restore safety is unprovable, the object's claim auto-narrows from
//!   `explicitly_granted_control_surface` / `view_first_observable_surface` to a control-authority-unverified /
//!   active-driver-unverified / presenter-handoff-unverified / consent-scope-unverified /
//!   retention-state-unverified / restore-replay-safety-unverified projection, discloses the narrowing with a
//!   precise trigger and binding dimension, and preserves the canonical object identity / last-known state. The
//!   underlying control, grant, presenter, consent, retention, and restore truth is never dropped opaquely. An
//!   object with every dimension intact must NOT carry a spurious narrowing, and an unresolved-authority /
//!   unprovable-driver / contested-handoff / undisclosed-consent / stale-retention / unprovable-restore state
//!   can never keep a fully explicitly-granted, single-driver controlled claim — presence never reads as
//!   control, no second active driver is shown on a sensitive surface, and no prior terminal / debug input is
//!   replayed on join or restore.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the shared terminal / debug view,
//!   collaboration join-review sheet, control-grant prompt, presenter-handoff sheet, paste / secret guard,
//!   collaboration retention sheet, session-restore view, support / export packet, and help / docs so product,
//!   help, and release publication stay aligned on downgrade behavior rather than drifting in copy — a
//!   controlled-looking object can never outrun the control authority, active driver, presenter handoff,
//!   consent scope, retention state, or restore replay safety it is being viewed away from.
//!
//! Each [`CollaborationControlAccessibilityRow`] keys on one
//! [`crate::m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix::M5CollaborationControlObject`] and reuses that frozen
//! object vocabulary plus the frozen [`M5CollaborationControlRequiredLabel`], [`M5CollaborationControlDowngradeTrigger`], and
//! shared [`M5CollaborationControlConsumerSurface`] consumer surfaces rather than minting parallel synonyms, so the
//! certified labels stay byte-identical to the matrix and the sibling collaboration-control packets.
//!
//! The packet is metadata-only: raw secrets, command text, variable bodies, clipboard contents, and endpoint refs
//! never cross this boundary; the packet carries only typed class tokens, opaque object refs, booleans, and
//! controlled labels so support, release, and diagnostics exports can reconstruct exactly what an accessible
//! fallback would have shown without leaking sensitive material or a raw payload.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen collaboration-control vocabulary — the capstone certifies the freeze matrix's objects, required
// labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix::{
    M5CollaborationControlConsumerSurface, M5CollaborationControlDowngradeTrigger,
    M5CollaborationControlObject, M5CollaborationControlRequiredLabel,
    M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_REF,
};

/// Schema version stamped on the M05-1312 collaboration-control accessibility parity packet.
pub const COLLABORATION_CONTROL_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`CollaborationControlAccessibilityPacket`].
pub const COLLABORATION_CONTROL_A11Y_RECORD_KIND: &str =
    "m5_collaboration_control_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`CollaborationControlAccessibilityRow`].
pub const COLLABORATION_CONTROL_A11Y_ROW_RECORD_KIND: &str =
    "m5_collaboration_control_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const COLLABORATION_CONTROL_A11Y_SCHEMA_REF: &str =
    "schemas/collaboration/m5-collaboration-control-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const COLLABORATION_CONTROL_A11Y_DOC_REF: &str =
    "docs/collaboration/m5_collaboration_control_accessibility_parity.md";

/// Repo-relative path of the frozen collaboration-control and engineering-lifecycle matrix this lane certifies.
pub const COLLABORATION_CONTROL_A11Y_MATRIX_REF: &str = M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const COLLABORATION_CONTROL_A11Y_FIXTURE_DIR: &str =
    "fixtures/collaboration/m5-collaboration-control-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const COLLABORATION_CONTROL_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-collaboration-control-accessibility-parity/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const COLLABORATION_CONTROL_A11Y_CSV_REF: &str =
    "artifacts/release/m5-collaboration-control-accessibility-parity/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const COLLABORATION_CONTROL_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-collaboration-control-accessibility-parity.md";

/// The reusable objects that render a dense, structured surface (the presenter token's presenter holder /
/// handoff chain, the session-restore view's restore transcript / replay-free render summary set) and
/// therefore MUST bind their structured layout to an equivalent flat list / textual path so the structure is
/// navigable non-visually.
const fn object_is_structure_heavy(object: M5CollaborationControlObject) -> bool {
    matches!(
        object,
        M5CollaborationControlObject::PresenterToken
            | M5CollaborationControlObject::SessionRestoreView
    )
}

/// The collaboration-control-truth dimension whose weakening an object primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn object_primary_dimension(
    object: M5CollaborationControlObject,
) -> M5CollaborationControlClaimDimension {
    match object {
        M5CollaborationControlObject::SharedTerminalDebugView => {
            M5CollaborationControlClaimDimension::ControlAuthorityClarity
        }
        M5CollaborationControlObject::ControlGrant => {
            M5CollaborationControlClaimDimension::ActiveDriverClarity
        }
        M5CollaborationControlObject::PresenterToken => {
            M5CollaborationControlClaimDimension::PresenterHandoffClarity
        }
        M5CollaborationControlObject::ConsentEnvelope => {
            M5CollaborationControlClaimDimension::ConsentScopeClarity
        }
        M5CollaborationControlObject::RetentionReview => {
            M5CollaborationControlClaimDimension::RetentionStateClarity
        }
        M5CollaborationControlObject::SessionRestoreView => {
            M5CollaborationControlClaimDimension::RestoreReplaySafetyClarity
        }
    }
}

/// A rendered fallback modality for an collaboration-control object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlFallbackModality {
    /// A rich, structured (outbound action set / lifecycle history) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / label-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5CollaborationControlFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured surface
    /// (i.e. a keyboard / screen-reader / CLI path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Textual | Self::Cli)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::List => "list",
            Self::Textual => "textual",
            Self::Cli => "cli",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same object may
/// render at desktop-full capability or narrow to a companion, read-only browser, headless CLI, docs export,
/// or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlRenderingSurface {
    /// The full-capability desktop surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A docs / help export projection.
    DocsExport,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5CollaborationControlRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop full-capability baseline and
    /// therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::CliHeadless => "cli_headless",
            Self::DocsExport => "docs_export",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / high-zoom / high-contrast / CLI reach for an object's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationControlNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only / color-only surface that traps keyboard / assistive-tech / headless-CLI
    /// users (red).
    ViewOnlyTrap,
}

impl CollaborationControlNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / CLI users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndLabeled => "reachable_and_labeled",
            Self::DisclosedReducedButReachable => "disclosed_reduced_but_reachable",
            Self::ViewOnlyTrap => "view_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the object meaning without leaking a raw payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationControlExportSummaryState {
    /// The object meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl CollaborationControlExportSummaryState {
    /// Returns true when the export never falls back to leaking a raw payload.
    pub const fn never_requires_raw_payload(self) -> bool {
        !matches!(self, Self::RequiresRawPayload)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutRawPayload => "reconstructable_without_raw_payload",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::RequiresRawPayload => "requires_raw_payload",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationControlNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl CollaborationControlNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or actions.
    pub const fn never_drops_silently(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityPreserved => "parity_preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::SilentlyDropped => "silently_dropped",
        }
    }
}

/// The collaboration-control claim ceiling an object asserts: how strong an explicitly-granted,
/// single-driver controlled posture it lets a surface present. Auto-narrowing lowers this ceiling when a
/// control-authority / active-driver / presenter-handoff / consent-scope / retention-state /
/// restore-replay-safety dimension weakens so an unresolved / presence-implied control authority, an
/// unprovable or contended active driver, an unprovable / contested presenter handoff, an undisclosed /
/// silently widening consent scope, a stale / silently broadened retention state, or an unprovable
/// replay-free restore can never keep an old `ExplicitlyGrantedControlSurface` or
/// `ViewFirstObservableSurface` label — presence never masquerades as control from a narrowed object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlA11yClaim {
    /// Explicitly-granted control surface: a fully control-authority-bound, single-active-driver,
    /// consent-disclosed, retention-bound, replay-free object — the strongest claim, a collaboration-control
    /// surface Aureline can present as exactly explicitly-granted and single-driver controlled to observe,
    /// request control, export, or restore right now.
    ExplicitlyGrantedControlSurface,
    /// View-first observable surface: a self-sufficient, view-first read-only object (a session-restore view a
    /// user can observe) that is not itself a mutating, control-driving surface.
    ViewFirstObservableSurface,
    /// Control-authority-unverified projection: the shared terminal / debug view's control authority is
    /// unresolved or presence-implied; the object stays a control-authority-unverified projection with its
    /// last-known session identity and single-active-driver state preserved, never presence shown as control.
    ControlAuthorityUnverifiedProjection,
    /// Active-driver-unverified projection: the control grant's single active driver is unprovable or a second
    /// driver is contending; the object stays an active-driver-unverified projection that keeps the grant
    /// authority, scope, and expiry distinct, never allowing more than one active driver on the surface.
    ActiveDriverUnverifiedProjection,
    /// Presenter-handoff-unverified projection: the presenter handoff is unprovable or contested; the object
    /// stays a presenter-handoff-unverified projection that keeps the presenter holder, handoff chain, and
    /// moderation scope explicit, never letting moderation silently transfer shell / debug control.
    PresenterHandoffUnverifiedProjection,
    /// Consent-scope-unverified projection: the join-time consent scope is undisclosed or would widen silently;
    /// the object stays a consent-scope-unverified projection that names the recording / retention / guest /
    /// route consequences, never widening scope without a fresh visible consent event.
    ConsentScopeUnverifiedProjection,
    /// Retention-state-unverified projection: the recording / retention state is stale or would broaden
    /// silently; the object stays a retention-state-unverified projection that discloses the recording state,
    /// retention mode, and archive scope, never broadening retention silently.
    RetentionStateUnverifiedProjection,
    /// Restore-replay-safety-unverified projection: the replay-free restore safety is unprovable; the object
    /// stays a restore-replay-safety-unverified projection that reattaches read-only, keeps the restore
    /// transcript class and retention scope visible, and requires a fresh control grant, never replaying prior
    /// terminal / debug input on join or restore.
    RestoreReplaySafetyUnverifiedProjection,
}

impl M5CollaborationControlA11yClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 8] = [
        Self::ExplicitlyGrantedControlSurface,
        Self::ViewFirstObservableSurface,
        Self::ControlAuthorityUnverifiedProjection,
        Self::ActiveDriverUnverifiedProjection,
        Self::PresenterHandoffUnverifiedProjection,
        Self::ConsentScopeUnverifiedProjection,
        Self::RetentionStateUnverifiedProjection,
        Self::RestoreReplaySafetyUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::ExplicitlyGrantedControlSurface => 7,
            Self::ViewFirstObservableSurface => 6,
            Self::ControlAuthorityUnverifiedProjection => 5,
            Self::ActiveDriverUnverifiedProjection => 4,
            Self::PresenterHandoffUnverifiedProjection => 3,
            Self::ConsentScopeUnverifiedProjection => 2,
            Self::RetentionStateUnverifiedProjection => 1,
            Self::RestoreReplaySafetyUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully explicitly-granted, single-driver controlled surface.
    pub const fn asserts_trusted_surface(self) -> bool {
        matches!(self, Self::ExplicitlyGrantedControlSurface)
    }

    /// Returns true when this claim asserts a fully self-sufficient (granted or observable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::ExplicitlyGrantedControlSurface | Self::ViewFirstObservableSurface
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitlyGrantedControlSurface => "explicitly_granted_control_surface",
            Self::ViewFirstObservableSurface => "view_first_observable_surface",
            Self::ControlAuthorityUnverifiedProjection => "control_authority_unverified_projection",
            Self::ActiveDriverUnverifiedProjection => "active_driver_unverified_projection",
            Self::PresenterHandoffUnverifiedProjection => "presenter_handoff_unverified_projection",
            Self::ConsentScopeUnverifiedProjection => "consent_scope_unverified_projection",
            Self::RetentionStateUnverifiedProjection => "retention_state_unverified_projection",
            Self::RestoreReplaySafetyUnverifiedProjection => {
                "restore_replay_safety_unverified_projection"
            }
        }
    }
}

/// The control-authority / active-driver / presenter-handoff / consent-scope / retention-state /
/// restore-replay-safety dimension whose state governs how far an object may claim to be a fully
/// explicitly-granted, single-driver controlled surface. The dimensions map to the six frozen
/// collaboration-control objects so every object carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlClaimDimension {
    /// Control-authority clarity: does the shared terminal / debug view keep its control-authority source and
    /// view-first default explicit so presence, follow mode, or companion resume never acquires terminal /
    /// debug control without an explicit grant (shared-terminal-debug-view)?
    ControlAuthorityClarity,
    /// Active-driver clarity: does the control grant keep its single active driver, scope, and expiry explicit
    /// rather than letting a second driver contend for the same sensitive surface (control-grant)?
    ActiveDriverClarity,
    /// Presenter-handoff clarity: does the presenter token keep its holder, handoff chain, and moderation scope
    /// explicit rather than letting moderation silently transfer shell / debug control (presenter-token)?
    PresenterHandoffClarity,
    /// Consent-scope clarity: does the consent envelope keep its recording / retention / guest / route
    /// consequences explicit before join rather than widening scope silently (consent-envelope)?
    ConsentScopeClarity,
    /// Retention-state clarity: does the retention review keep its recording state, retention mode / duration,
    /// and replayable-archive scope explicit rather than starting or broadening retention silently
    /// (retention-review)?
    RetentionStateClarity,
    /// Restore-replay-safety clarity: does the session-restore view keep its replay-free reattachment, restore
    /// transcript class, and fresh-grant requirement explicit rather than replaying prior terminal / debug
    /// input on join or restore (session-restore-view)?
    RestoreReplaySafetyClarity,
}

impl M5CollaborationControlClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ControlAuthorityClarity,
        Self::ActiveDriverClarity,
        Self::PresenterHandoffClarity,
        Self::ConsentScopeClarity,
        Self::RetentionStateClarity,
        Self::RestoreReplaySafetyClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlAuthorityClarity => "control_authority_clarity",
            Self::ActiveDriverClarity => "active_driver_clarity",
            Self::PresenterHandoffClarity => "presenter_handoff_clarity",
            Self::ConsentScopeClarity => "consent_scope_clarity",
            Self::RetentionStateClarity => "retention_state_clarity",
            Self::RestoreReplaySafetyClarity => "restore_replay_safety_clarity",
        }
    }
}

/// The observed condition of one collaboration-control-truth dimension. Anything weaker than [`Self::FullyQualified`]
/// imposes a narrowing ceiling on the object's claim. The unresolved / unprovable / contested / undisclosed /
/// stale states the lane must auto-narrow on — an unresolved / presence-implied control authority, an
/// unprovable or contended active driver, an unprovable / contested presenter handoff, an undisclosed /
/// silently widening consent scope, a stale / silently broadened retention state, and an unprovable
/// replay-free restore — are the states that [`Self::cannot_be_shown_trusted`] flags: each is a genuine truth
/// degradation that can never be shown as a fully explicitly-granted, single-driver controlled surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlConditionState {
    /// Fully control-authority-bound, single-active-driver, presenter-provable, consent-disclosed,
    /// retention-bound, replay-free — imposes no ceiling.
    FullyQualified,
    /// The control authority is unresolved or would be implied by presence — claim drops to a
    /// control-authority-unverified projection.
    ControlAuthorityUnresolvedOrPresenceImplied,
    /// The single active driver is unprovable or a second driver is contending — claim drops to an
    /// active-driver-unverified projection.
    ActiveDriverUnprovableOrMultiDriver,
    /// The presenter handoff is unprovable or contested — claim drops to a presenter-handoff-unverified
    /// projection.
    PresenterHandoffUnprovableOrContested,
    /// The join-time consent scope is undisclosed or would widen silently — claim drops to a
    /// consent-scope-unverified projection.
    ConsentScopeUndisclosedOrWidened,
    /// The recording / retention state is stale or would broaden silently — claim drops to a
    /// retention-state-unverified projection.
    RetentionStateStaleOrBroadenedSilently,
    /// The replay-free restore safety is unprovable — claim drops to a restore-replay-safety-unverified
    /// projection.
    RestoreReplaySafetyUnprovable,
}

impl M5CollaborationControlConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::FullyQualified,
        Self::ControlAuthorityUnresolvedOrPresenceImplied,
        Self::ActiveDriverUnprovableOrMultiDriver,
        Self::PresenterHandoffUnprovableOrContested,
        Self::ConsentScopeUndisclosedOrWidened,
        Self::RetentionStateStaleOrBroadenedSilently,
        Self::RestoreReplaySafetyUnprovable,
    ];

    /// Returns true when the dimension is weaker than fully qualified and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FullyQualified)
    }

    /// Returns true when the condition reflects a weakened state that cannot be shown as a fully
    /// explicitly-granted, single-driver controlled surface and must never be shown as such. Every weak collaboration-control
    /// condition is a genuine truth degradation, so all six flag here.
    pub const fn cannot_be_shown_trusted(self) -> bool {
        matches!(
            self,
            Self::ControlAuthorityUnresolvedOrPresenceImplied
                | Self::ActiveDriverUnprovableOrMultiDriver
                | Self::PresenterHandoffUnprovableOrContested
                | Self::ConsentScopeUndisclosedOrWidened
                | Self::RetentionStateStaleOrBroadenedSilently
                | Self::RestoreReplaySafetyUnprovable
        )
    }

    /// The strongest claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5CollaborationControlA11yClaim {
        match self {
            Self::FullyQualified => {
                M5CollaborationControlA11yClaim::ExplicitlyGrantedControlSurface
            }
            Self::ControlAuthorityUnresolvedOrPresenceImplied => {
                M5CollaborationControlA11yClaim::ControlAuthorityUnverifiedProjection
            }
            Self::ActiveDriverUnprovableOrMultiDriver => {
                M5CollaborationControlA11yClaim::ActiveDriverUnverifiedProjection
            }
            Self::PresenterHandoffUnprovableOrContested => {
                M5CollaborationControlA11yClaim::PresenterHandoffUnverifiedProjection
            }
            Self::ConsentScopeUndisclosedOrWidened => {
                M5CollaborationControlA11yClaim::ConsentScopeUnverifiedProjection
            }
            Self::RetentionStateStaleOrBroadenedSilently => {
                M5CollaborationControlA11yClaim::RetentionStateUnverifiedProjection
            }
            Self::RestoreReplaySafetyUnprovable => {
                M5CollaborationControlA11yClaim::RestoreReplaySafetyUnverifiedProjection
            }
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each state
    /// maps to the on-topic frozen trigger the freeze matrix already governs, so the certified reason stays
    /// byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5CollaborationControlDowngradeTrigger {
        match self {
            // The fully-qualified baseline never narrows; kept for exhaustiveness.
            Self::FullyQualified => {
                M5CollaborationControlDowngradeTrigger::CollaborationControlMatrixStale
            }
            Self::ControlAuthorityUnresolvedOrPresenceImplied => {
                M5CollaborationControlDowngradeTrigger::ControlAuthorityUnstated
            }
            Self::ActiveDriverUnprovableOrMultiDriver => {
                M5CollaborationControlDowngradeTrigger::ActiveDriverUnstated
            }
            Self::PresenterHandoffUnprovableOrContested => {
                M5CollaborationControlDowngradeTrigger::ViewFirstDefaultUnstated
            }
            Self::ConsentScopeUndisclosedOrWidened => {
                M5CollaborationControlDowngradeTrigger::ConsentScopeUnstated
            }
            Self::RetentionStateStaleOrBroadenedSilently => {
                M5CollaborationControlDowngradeTrigger::RetentionStateUnstated
            }
            Self::RestoreReplaySafetyUnprovable => {
                M5CollaborationControlDowngradeTrigger::RestoreReplaySafetyUnstated
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyQualified => "fully_qualified",
            Self::ControlAuthorityUnresolvedOrPresenceImplied => {
                "control_authority_unresolved_or_presence_implied"
            }
            Self::ActiveDriverUnprovableOrMultiDriver => "active_driver_unprovable_or_multi_driver",
            Self::PresenterHandoffUnprovableOrContested => {
                "presenter_handoff_unprovable_or_contested"
            }
            Self::ConsentScopeUndisclosedOrWidened => "consent_scope_undisclosed_or_widened",
            Self::RetentionStateStaleOrBroadenedSilently => {
                "retention_state_stale_or_broadened_silently"
            }
            Self::RestoreReplaySafetyUnprovable => "restore_replay_safety_unprovable",
        }
    }
}

/// One collaboration-control-truth dimension's observed condition on an object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationControlClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5CollaborationControlClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5CollaborationControlConditionState,
}

/// An honest claim auto-narrow block. When a collaboration-control-truth dimension weakens, the object's
/// claim lowers to the permitted ceiling, names the binding dimension and frozen trigger, and preserves the
/// canonical object identity / last-known state rather than silently dropping it — the underlying control,
/// grant, presenter, consent, and retention truth is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationControlClaimAutoNarrow {
    /// The claim the object is narrowed to.
    pub narrowed_to: M5CollaborationControlA11yClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling constraint).
    pub binding_dimension: M5CollaborationControlClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5CollaborationControlDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical object identity and last-known state are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying control / grant / presenter / consent / retention truth is preserved (never dropped)
    /// across the narrowing; must hold so control-authority-unverified, active-driver-unverified,
    /// consent-scope-unverified, and restore-replay-safety-unverified states never fail opaquely, and no local
    /// draft or evidence is lost.
    pub preserves_truth_continuity: bool,
}

impl CollaborationControlClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and control / grant /
    /// consent / retention truth and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_truth_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for an object's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a raw payload is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationControlCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl CollaborationControlCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at least one
    /// export field is named, and a raw-payload-only export is prohibited.
    pub fn is_complete(&self) -> bool {
        self.raw_payload_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationControlRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5CollaborationControlRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: CollaborationControlNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for an collaboration-control accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationControlAccessibilityStatus {
    /// Full keyboard / screen-reader / high-zoom / high-contrast / CLI / export parity with no narrowing
    /// (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims trusted, or drops state silently (red).
    Stranded,
}

impl CollaborationControlAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one collaboration-control object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationControlAccessibilityRow {
    /// Record kind; must equal [`COLLABORATION_CONTROL_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`COLLABORATION_CONTROL_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen object this row certifies.
    pub object: M5CollaborationControlObject,
    /// Ref to the frozen per-object domain schema this row certifies.
    pub source_object_schema_ref: String,
    /// Opaque ref to the object this row represents; stays visible on every surface, so this is never empty.
    pub object_context_ref: String,
    /// Rendered modalities offered; a structure-heavy object must also offer a non-visual (list / textual /
    /// CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5CollaborationControlFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical object identity, control-authority source,
    /// single active driver, presenter holder / handoff chain, join-time consent scope, retention state, and
    /// restore transcript class as the rich object; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: CollaborationControlNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: CollaborationControlNonVisualReachState,
    /// High-zoom (reflow / magnification) legibility of the non-visual path.
    pub high_zoom_reach: CollaborationControlNonVisualReachState,
    /// High-contrast / forced-colors behavior of the non-visual path.
    pub high_contrast_reach: CollaborationControlNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: CollaborationControlNonVisualReachState,
    /// Whether the export-safe summary preserves object meaning.
    pub export_summary: CollaborationControlExportSummaryState,
    /// Ref to the export-safe summary object for this object.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: CollaborationControlCopyExportParity,
    /// The full claim this object asserts when every dimension is intact.
    pub full_ready_claim: M5CollaborationControlA11yClaim,
    /// The observed condition of each modeled collaboration-control-truth dimension.
    #[serde(default)]
    pub claim_conditions: Vec<CollaborationControlClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the object's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<CollaborationControlClaimAutoNarrow>,
    /// Whether the underlying control / grant / presenter / consent / retention truth is preserved on this object
    /// regardless of narrowing; must hold so every unverified projection never fails opaquely.
    pub truth_preserved: bool,
    /// Rendering surfaces this object is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5CollaborationControlRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<CollaborationControlRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5CollaborationControlRequiredLabel>,
    /// Semantic consumer surfaces this object is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5CollaborationControlConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl CollaborationControlAccessibilityRow {
    /// Returns true when this object renders a dense, structured surface and must bind to a flat non-visual
    /// path.
    pub const fn is_structure_heavy(&self) -> bool {
        object_is_structure_heavy(self.object)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `FullyQualified` when the row does not model that
    /// dimension.
    pub fn condition_for(
        &self,
        dimension: M5CollaborationControlClaimDimension,
    ) -> M5CollaborationControlConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5CollaborationControlConditionState::FullyQualified)
    }

    /// Whether any modeled dimension is weaker than fully qualified.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling, capped at the
    /// object's full claim.
    pub fn permitted_claim(&self) -> M5CollaborationControlA11yClaim {
        let mut permitted = self.full_ready_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension narrows below
    /// the object's full claim.
    pub fn binding_condition(&self) -> Option<&CollaborationControlClaimConditionEntry> {
        let mut binding: Option<(&CollaborationControlClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_ready_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition, rank)),
            }
        }
        binding.map(|(condition, _)| condition)
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any.
    pub fn binding_dimension(&self) -> Option<M5CollaborationControlClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The claim this object effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5CollaborationControlA11yClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_ready_claim,
        }
    }

    /// AC / auto-narrowing honesty: an unresolved control authority, a contested active driver, an
    /// undisclosed consent scope, or a stale retention state can no longer keep an old `ExplicitlyGrantedControlSurface` /
    /// `ViewFirstObservableSurface` label. The effective claim never exceeds the permitted ceiling; when a
    /// dimension narrows below the full claim, an honest narrow block is present, narrows to exactly the
    /// permitted ceiling, binds to the ceiling-imposing dimension with its frozen trigger, and preserves
    /// canonical identity and truth. When nothing narrows, no spurious narrow block is present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_condition()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding.dimension
                    && narrow.trigger == binding.state.default_trigger()
                    && binding.state.is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / controlled honesty: an unresolved-authority / unprovable-driver / contested-handoff /
    /// undisclosed-consent / stale-retention / unprovable-restore state never keeps a granted claim — presence
    /// never masquerades as control from a narrowed object. When such a state is modeled, the effective claim
    /// must not assert `ExplicitlyGrantedControlSurface`.
    pub fn trusted_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_shown_trusted());
        !(has_unprovable_state && self.effective_claim().asserts_trusted_surface())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical truth — no
    /// keyboard / screen-reader / high-zoom / high-contrast / CLI trap, a structure-heavy object offers a
    /// non-visual fallback, and the export reconstructs meaning without a raw payload.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.object_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.high_zoom_reach.never_traps()
            && self.high_contrast_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_structure_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the object meaning without leaking a raw payload.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_payload()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: every unverified projection preserves the underlying control / grant / consent /
    /// retention truth. The row must assert `truth_preserved`, and any narrow block must preserve truth
    /// continuity too.
    pub fn preserves_truth_continuity(&self) -> bool {
        self.truth_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_truth_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the object carries an honest claim
    /// narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.high_zoom_reach.is_disclosed_reduction()
            || self.high_contrast_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced interactivity
    /// and keeps its labels, so product / help / release publication stay aligned on the same narrowed
    /// state.
    pub fn narrowing_disclosed(&self) -> bool {
        // Every declared narrowed rendering surface has a disclosure entry.
        for surface in &self.rendering_surfaces {
            if surface.is_narrowed()
                && !self
                    .narrowing_disclosures
                    .iter()
                    .any(|d| d.rendering_surface == *surface)
            {
                return false;
            }
        }
        // Every disclosure never silently drops and preserves labels on a narrowed surface.
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Whether the row models its object's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = object_primary_dimension(self.object);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5CollaborationControlRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> CollaborationControlAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.trusted_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_truth_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return CollaborationControlAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            CollaborationControlAccessibilityStatus::NarrowedDisclosed
        } else {
            CollaborationControlAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == COLLABORATION_CONTROL_A11Y_ROW_RECORD_KIND
            && self.schema_version == COLLABORATION_CONTROL_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_object_schema_ref.trim().is_empty()
            && !self.object_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "object={object} keyboard={keyboard} screen_reader={screen_reader} \
high_zoom={high_zoom} high_contrast={high_contrast} cli={cli} export={export} \
full_claim={full} effective_claim={effective} status={status}",
            object = self.object.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            high_zoom = self.high_zoom_reach.as_str(),
            high_contrast = self.high_contrast_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_ready_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1312 collaboration-control accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationControlAccessibilitySummary {
    pub row_count: usize,
    pub object_count: usize,
    pub structure_heavy_object_count: usize,
    pub all_structure_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_trusted_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_truth_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`CollaborationControlAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollaborationControlAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<CollaborationControlAccessibilityRow>,
}

/// Checked-in M05-1312 collaboration-control accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationControlAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<CollaborationControlAccessibilityRow>,
    pub summary: CollaborationControlAccessibilitySummary,
}

impl CollaborationControlAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: CollaborationControlAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: COLLABORATION_CONTROL_A11Y_SCHEMA_VERSION,
            record_kind: COLLABORATION_CONTROL_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: CollaborationControlAccessibilitySummary {
                row_count: 0,
                object_count: 0,
                structure_heavy_object_count: 0,
                all_structure_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_trusted_honesty_holds: false,
                all_export_summaries_preserve_meaning: false,
                all_truth_preserved: false,
                all_narrowing_disclosed: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                rendering_surface_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Objects represented by some row in this packet.
    pub fn represented_objects(&self) -> BTreeSet<M5CollaborationControlObject> {
        self.rows.iter().map(|r| r.object).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5CollaborationControlClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5CollaborationControlConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5CollaborationControlA11yClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5CollaborationControlConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> CollaborationControlAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5CollaborationControlConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let structure_heavy: Vec<&CollaborationControlAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_structure_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                CollaborationControlAccessibilityStatus::Parity => green += 1,
                CollaborationControlAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                CollaborationControlAccessibilityStatus::Stranded => red += 1,
            }
        }

        CollaborationControlAccessibilitySummary {
            row_count: self.rows.len(),
            object_count: self.represented_objects().len(),
            structure_heavy_object_count: structure_heavy.len(),
            all_structure_heavy_have_non_visual_fallback: structure_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(CollaborationControlAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(CollaborationControlAccessibilityRow::claim_is_honest),
            all_trusted_honesty_holds: self
                .rows
                .iter()
                .all(CollaborationControlAccessibilityRow::trusted_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(CollaborationControlAccessibilityRow::export_preserves_meaning),
            all_truth_preserved: self
                .rows
                .iter()
                .all(CollaborationControlAccessibilityRow::preserves_truth_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(CollaborationControlAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<CollaborationControlAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != COLLABORATION_CONTROL_A11Y_SCHEMA_VERSION {
            violations.push(CollaborationControlAccessibilityViolation::SchemaVersion {
                expected: COLLABORATION_CONTROL_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != COLLABORATION_CONTROL_A11Y_RECORD_KIND {
            violations.push(CollaborationControlAccessibilityViolation::RecordKind {
                expected: COLLABORATION_CONTROL_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(CollaborationControlAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_objects = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(CollaborationControlAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_objects.insert(row.object);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_shown_trusted())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(CollaborationControlAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its object's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    CollaborationControlAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: object_primary_dimension(row.object),
                    },
                );
            }

            // Each row must preserve every mandatory object label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    CollaborationControlAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A structure-heavy object must render a structured projection *and* a non-visual path.
            if row.is_structure_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5CollaborationControlFallbackModality::Structured)
            {
                violations.push(
                    CollaborationControlAccessibilityViolation::StructureHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: claim never over-asserts a trusted / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(
                    CollaborationControlAccessibilityViolation::ClaimOverAsserted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / controlled honesty: an unresolved-authority / contested-driver / undisclosed-consent /
            // stale-retention state never keeps a granted claim.
            if !row.trusted_honesty_holds() {
                violations.push(
                    CollaborationControlAccessibilityViolation::WeakStateShownAsTrusted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    CollaborationControlAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(
                    CollaborationControlAccessibilityViolation::ExportRequiresRawPayload {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: weakened states preserve control / grant / presenter / consent / retention truth.
            if !row.preserves_truth_continuity() {
                violations.push(CollaborationControlAccessibilityViolation::TruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    CollaborationControlAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    CollaborationControlAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == CollaborationControlAccessibilityStatus::Stranded {
                violations.push(CollaborationControlAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen object is certified at least once.
        for object in M5CollaborationControlObject::ALL {
            if !seen_objects.contains(&object) {
                violations.push(
                    CollaborationControlAccessibilityViolation::MissingObjectCoverage { object },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5CollaborationControlClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    CollaborationControlAccessibilityViolation::MissingDimensionCoverage {
                        dimension,
                    },
                );
            }
        }

        // Coverage: every condition state (the fully-qualified baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5CollaborationControlConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    CollaborationControlAccessibilityViolation::MissingConditionStateCoverage {
                        state,
                    },
                );
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing spectrum
        // (granted → … → restore-replay-safety-unverified) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5CollaborationControlA11yClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    CollaborationControlAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Controlled honesty must be proven with at least one unresolved-authority / contested-driver /
        // undisclosed-consent / stale-retention row in the packet, so the "cannot-prove never
        // shown as controlled" guarantee is exercised end-to-end.
        if !has_unprovable_row {
            violations.push(CollaborationControlAccessibilityViolation::TrustedHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the shared terminal / debug view, join-review
        // sheet, control-grant prompt, presenter-handoff sheet, paste / secret guard, retention sheet,
        // session-restore view, and support / export packet — so every consumer surface is exercised at least once.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5CollaborationControlConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    CollaborationControlAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(CollaborationControlAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("collaboration-control accessibility parity packet serializes"),
        ) {
            violations.push(CollaborationControlAccessibilityViolation::RawObjectMaterialInExport);
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
            .expect("collaboration-control accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,object,keyboard_reach,screen_reader_reach,high_zoom_reach,high_contrast_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{object},{keyboard},{screen_reader},{high_zoom},{high_contrast},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                object = row.object.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                high_zoom = row.high_zoom_reach.as_str(),
                high_contrast = row.high_contrast_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_ready_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, help, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Collaboration-Control Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Objects: {} certified across {} / {} frozen objects\n",
            self.summary.object_count,
            self.represented_objects().len(),
            M5CollaborationControlObject::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.object.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_ready_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in collaboration-control accessibility parity export.
pub fn current_m5_collaboration_control_accessibility_parity_export(
) -> Result<CollaborationControlAccessibilityPacket, CollaborationControlAccessibilityArtifactError>
{
    let packet: CollaborationControlAccessibilityPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-collaboration-control-accessibility-parity/support_export.json"
    )))
        .map_err(CollaborationControlAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CollaborationControlAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in collaboration-control accessibility parity export.
#[derive(Debug)]
pub enum CollaborationControlAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CollaborationControlAccessibilityViolation>),
}

impl fmt::Display for CollaborationControlAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "collaboration-control accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "collaboration-control accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for CollaborationControlAccessibilityArtifactError {}

/// Validation failure for M05-1312 collaboration-control accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollaborationControlAccessibilityViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    MissingPrimaryDimension {
        id: String,
        dimension: M5CollaborationControlClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    StructureHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    WeakStateShownAsTrusted {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresRawPayload {
        id: String,
    },
    TruthDropped {
        id: String,
    },
    NarrowingDropsContextSilently {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    StrandedRow {
        id: String,
    },
    MissingObjectCoverage {
        object: M5CollaborationControlObject,
    },
    MissingDimensionCoverage {
        dimension: M5CollaborationControlClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5CollaborationControlConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5CollaborationControlA11yClaim,
    },
    TrustedHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5CollaborationControlConsumerSurface,
    },
    SummaryMismatch,
    RawObjectMaterialInExport,
}

impl CollaborationControlAccessibilityViolation {
    /// Stable token for CLI / support handoff.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaVersion { .. } => "schema_version",
            Self::RecordKind { .. } => "record_kind",
            Self::MissingIdentity => "missing_identity",
            Self::DuplicateId { .. } => "duplicate_id",
            Self::IncompleteRow { .. } => "incomplete_row",
            Self::MissingPrimaryDimension { .. } => "missing_primary_dimension",
            Self::MissingMandatoryLabel { .. } => "missing_mandatory_label",
            Self::StructureHeavyMissingStructured { .. } => "structure_heavy_missing_structured",
            Self::ClaimOverAsserted { .. } => "claim_over_asserted",
            Self::WeakStateShownAsTrusted { .. } => "weak_state_shown_as_trusted",
            Self::AssistiveTechStranded { .. } => "assistive_tech_stranded",
            Self::ExportRequiresRawPayload { .. } => "export_requires_raw_payload",
            Self::TruthDropped { .. } => "truth_dropped",
            Self::NarrowingDropsContextSilently { .. } => "narrowing_drops_context_silently",
            Self::MissingConsumerParity { .. } => "missing_consumer_parity",
            Self::StrandedRow { .. } => "stranded_row",
            Self::MissingObjectCoverage { .. } => "missing_object_coverage",
            Self::MissingDimensionCoverage { .. } => "missing_dimension_coverage",
            Self::MissingConditionStateCoverage { .. } => "missing_condition_state_coverage",
            Self::MissingClaimTierCoverage { .. } => "missing_claim_tier_coverage",
            Self::TrustedHonestyUnproven => "trusted_honesty_unproven",
            Self::MissingConsumerSurfaceCoverage { .. } => "missing_consumer_surface_coverage",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawObjectMaterialInExport => "raw_object_material_in_export",
        }
    }
}

impl fmt::Display for CollaborationControlAccessibilityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete accessibility row: {id}"),
            Self::MissingPrimaryDimension { id, dimension } => {
                write!(
                    f,
                    "row {id} does not model its object's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory object label")
            }
            Self::StructureHeavyMissingStructured { id } => {
                write!(
                    f,
                    "structure-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts a trusted / reviewable surface for a weakened one, or narrows spuriously"
                )
            }
            Self::WeakStateShownAsTrusted { id } => {
                write!(
                    f,
                    "row {id} shows an unresolved-authority / contested-driver / undisclosed-consent / stale-retention state as an explicitly-granted controlled surface"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / high-zoom / high-contrast / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresRawPayload { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without leaking a raw payload"
                )
            }
            Self::TruthDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve control / grant / consent / retention truth across narrowing"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing it"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingObjectCoverage { object } => {
                write!(f, "object {object:?} is not certified in the packet")
            }
            Self::MissingDimensionCoverage { dimension } => {
                write!(
                    f,
                    "claim dimension {} is not exercised in the packet",
                    dimension.as_str()
                )
            }
            Self::MissingConditionStateCoverage { state } => {
                write!(
                    f,
                    "condition state {} is not exercised in the packet",
                    state.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::TrustedHonestyUnproven => {
                write!(
                    f,
                    "no unresolved-authority / contested-driver / undisclosed-consent / stale-retention row is present to prove the controlled-honesty guarantee"
                )
            }
            Self::MissingConsumerSurfaceCoverage { surface } => {
                write!(
                    f,
                    "consumer surface {} does not ingest any row in the packet",
                    surface.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawObjectMaterialInExport => {
                write!(f, "export contains raw object material")
            }
        }
    }
}

impl Error for CollaborationControlAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "blocked"
            | "unresolved"
            | "partial"
            | "stale"
            | "incomplete"
            | "not comparable"
            | "restricted"
            | "collapsed"
            | "ellipsis"
            | "mixed"
            | "expired"
            | "inferred"
            | "unverified"
            | "trusted"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            // Bare "secret" / "api_key" are intentionally NOT flagged: the governed collaboration-control
            // vocabulary legitimately contains tokens such as `paste_secret_guard`. Real raw material is
            // caught by the credential-shaped patterns below (matching the sibling collaboration-control
            // lanes).
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

/// The canonical packet id for the checked-in stable export.
pub const COLLABORATION_CONTROL_A11Y_PACKET_ID: &str =
    "m5-collaboration-control-accessibility-parity:stable:0001";

/// Builds the canonical, checked-in collaboration-control accessibility parity packet. This is the one source of
/// truth shared by the tests and the on-disk support export so both stay byte-aligned.
pub fn seeded_m5_collaboration_control_accessibility_parity_packet(
) -> CollaborationControlAccessibilityPacket {
    CollaborationControlAccessibilityPacket::new(CollaborationControlAccessibilityPacketInput {
        packet_id: COLLABORATION_CONTROL_A11Y_PACKET_ID.to_owned(),
        as_of: "2026-07-16T00:00:00Z".to_owned(),
        matrix_ref: COLLABORATION_CONTROL_A11Y_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!(
        "evidence:collaboration-control-accessibility-parity:{id}"
    )]
}

fn all_required_labels() -> Vec<M5CollaborationControlRequiredLabel> {
    M5CollaborationControlRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> CollaborationControlCopyExportParity {
    CollaborationControlCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5CollaborationControlClaimDimension,
    state: M5CollaborationControlConditionState,
) -> CollaborationControlClaimConditionEntry {
    CollaborationControlClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — the support / export packet and the
/// shared terminal / debug view — so the narrowed state always reaches headless field triage.
fn base_consumers(
    extra: &[M5CollaborationControlConsumerSurface],
) -> Vec<M5CollaborationControlConsumerSurface> {
    let mut out = vec![
        M5CollaborationControlConsumerSurface::SupportExportPacket,
        M5CollaborationControlConsumerSurface::SharedTerminalDebugView,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps full label
/// and summary parity on the narrower surfaces; a narrowed row discloses the reduced interactions it drops
/// there.
fn surface_disclosures(
    labels: &[&str],
    state: CollaborationControlNarrowingDisclosureState,
) -> Vec<CollaborationControlRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        CollaborationControlRenderingNarrowingDisclosure {
            rendering_surface: M5CollaborationControlRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        CollaborationControlRenderingNarrowingDisclosure {
            rendering_surface: M5CollaborationControlRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_control_affordance".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<CollaborationControlRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        CollaborationControlNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced interactions while
/// preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<CollaborationControlRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        CollaborationControlNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5CollaborationControlRenderingSurface> {
    vec![
        M5CollaborationControlRenderingSurface::DesktopFull,
        M5CollaborationControlRenderingSurface::CliHeadless,
        M5CollaborationControlRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5CollaborationControlFallbackModality> {
    vec![
        M5CollaborationControlFallbackModality::List,
        M5CollaborationControlFallbackModality::Textual,
        M5CollaborationControlFallbackModality::Cli,
    ]
}

fn structured_modalities() -> Vec<M5CollaborationControlFallbackModality> {
    vec![
        M5CollaborationControlFallbackModality::Structured,
        M5CollaborationControlFallbackModality::List,
        M5CollaborationControlFallbackModality::Textual,
        M5CollaborationControlFallbackModality::Cli,
    ]
}

const REACHABLE: CollaborationControlNonVisualReachState =
    CollaborationControlNonVisualReachState::ReachableAndLabeled;
const REDUCED: CollaborationControlNonVisualReachState =
    CollaborationControlNonVisualReachState::DisclosedReducedButReachable;

fn seeded_rows() -> Vec<CollaborationControlAccessibilityRow> {
    vec![
        // Shared terminal / debug view (control-authority bound) — the shared terminal or debugger stream
        // begins view-first, names its single active driver, and shows the provenance of every input, so it is
        // a fully explicitly-granted, single-driver controlled surface reachable on every surface with no
        // narrowing (green). Keyboard-only and screen-reader users can observe, request control, and export it
        // without losing the control-authority source or single-active-driver truth, and presence never reads
        // as control.
        CollaborationControlAccessibilityRow {
            record_kind: COLLABORATION_CONTROL_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: COLLABORATION_CONTROL_A11Y_SCHEMA_VERSION,
            row_id: "a11y:shared-terminal-debug-view-control-authority-bound".to_owned(),
            object: M5CollaborationControlObject::SharedTerminalDebugView,
            source_object_schema_ref: M5CollaborationControlObject::SharedTerminalDebugView
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "collab:shared-terminal-debug-view:0001".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: CollaborationControlExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:shared-terminal-debug-view-control-authority-bound:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "single_active_driver",
                "control_authority_source",
                "input_provenance",
            ]),
            full_ready_claim: M5CollaborationControlA11yClaim::ExplicitlyGrantedControlSurface,
            claim_conditions: vec![condition(
                M5CollaborationControlClaimDimension::ControlAuthorityClarity,
                M5CollaborationControlConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "object_identity",
                "single_active_driver",
                "control_authority_source",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CollaborationControlConsumerSurface::ControlGrantPrompt,
                M5CollaborationControlConsumerSurface::PasteSecretGuard,
            ]),
            source_refs: vec![
                "TAD v1.25 §19.4.1 — shared terminal / debugger control & view-first default".to_owned(),
                COLLABORATION_CONTROL_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-18T00:00:00Z".to_owned(),
            evidence_refs: ev("shared-terminal-debug-view-control-authority-bound"),
        },
        // Session-restore view (replay-free bound) — structure-heavy (restore transcript class / replay-free
        // render summary / retention scope / reopened-target set); it reattaches read-only with no prior input
        // replayed, so it is a self-sufficient, view-first observable surface a user can inspect, with full
        // parity on every surface (green). Its structured transcript-summary set binds to a flat list / textual
        // path.
        CollaborationControlAccessibilityRow {
            record_kind: COLLABORATION_CONTROL_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: COLLABORATION_CONTROL_A11Y_SCHEMA_VERSION,
            row_id: "a11y:session-restore-view-replay-free-bound".to_owned(),
            object: M5CollaborationControlObject::SessionRestoreView,
            source_object_schema_ref: M5CollaborationControlObject::SessionRestoreView
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "collab:session-restore-view:0002".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: CollaborationControlExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:session-restore-view-replay-free-bound:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "restore_transcript_class",
                "replay_free_render_summary",
                "retention_scope",
            ]),
            full_ready_claim: M5CollaborationControlA11yClaim::ViewFirstObservableSurface,
            claim_conditions: vec![condition(
                M5CollaborationControlClaimDimension::RestoreReplaySafetyClarity,
                M5CollaborationControlConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "object_identity",
                "restore_transcript_class",
                "replay_free_render_summary",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CollaborationControlConsumerSurface::SessionRestoreView,
                M5CollaborationControlConsumerSurface::HelpDocs,
            ]),
            source_refs: vec![
                "TDD v3.6 — session restore / no-rerun behavior".to_owned(),
                COLLABORATION_CONTROL_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-18T00:00:00Z".to_owned(),
            evidence_refs: ev("session-restore-view-replay-free-bound"),
        },
        // Shared terminal / debug view (control authority unresolved / presence-implied) — the control
        // authority is unresolved or would be implied by presence, so it auto-narrows to a
        // control-authority-unverified projection that keeps the last-known session identity, single-active-driver
        // state, and view-first default explicit without letting presence, follow mode, or companion resume
        // acquire terminal / debug control (yellow). Its screen-reader traversal discloses a reduced linear walk.
        CollaborationControlAccessibilityRow {
            record_kind: COLLABORATION_CONTROL_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: COLLABORATION_CONTROL_A11Y_SCHEMA_VERSION,
            row_id: "a11y:shared-terminal-debug-view-control-authority-unresolved".to_owned(),
            object: M5CollaborationControlObject::SharedTerminalDebugView,
            source_object_schema_ref: M5CollaborationControlObject::SharedTerminalDebugView
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "collab:shared-terminal-debug-view:0003".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REDUCED,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: CollaborationControlExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:shared-terminal-debug-view-control-authority-unresolved:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "last_known_session_identity",
                "unresolved_control_authority_reason",
                "view_first_default_state",
            ]),
            full_ready_claim: M5CollaborationControlA11yClaim::ExplicitlyGrantedControlSurface,
            claim_conditions: vec![condition(
                M5CollaborationControlClaimDimension::ControlAuthorityClarity,
                M5CollaborationControlConditionState::ControlAuthorityUnresolvedOrPresenceImplied,
            )],
            claim_narrow: Some(CollaborationControlClaimAutoNarrow {
                narrowed_to: M5CollaborationControlA11yClaim::ControlAuthorityUnverifiedProjection,
                binding_dimension: M5CollaborationControlClaimDimension::ControlAuthorityClarity,
                trigger: M5CollaborationControlDowngradeTrigger::ControlAuthorityUnstated,
                narrowed_label:
                    "This shared terminal / debug view's control authority is unresolved or would be implied by presence — shown as a control-authority-unverified projection that keeps the session identity, last-known single-active-driver state, and view-first default explicit, never letting presence, follow mode, browser handoff, or companion resume acquire terminal / debug control without an explicit grant"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "last_known_session_identity",
                "unresolved_control_authority_reason",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CollaborationControlConsumerSurface::ControlGrantPrompt,
                M5CollaborationControlConsumerSurface::PresenterHandoffSheet,
            ]),
            source_refs: vec![
                "TAD v1.25 §19.4 — collaboration presence never implies control".to_owned(),
                COLLABORATION_CONTROL_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-18T00:00:00Z".to_owned(),
            evidence_refs: ev("shared-terminal-debug-view-control-authority-unresolved"),
        },
        // Control grant (active driver unprovable / multi-driver) — the single active driver is unprovable or a
        // second driver is contending, so it auto-narrows to an active-driver-unverified projection that keeps
        // the grant authority, scope, expiry, and revoke / reclaim path explicit, never allowing more than one
        // active driver on the sensitive surface (yellow).
        CollaborationControlAccessibilityRow {
            record_kind: COLLABORATION_CONTROL_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: COLLABORATION_CONTROL_A11Y_SCHEMA_VERSION,
            row_id: "a11y:control-grant-active-driver-unprovable".to_owned(),
            object: M5CollaborationControlObject::ControlGrant,
            source_object_schema_ref: M5CollaborationControlObject::ControlGrant
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "collab:control-grant:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REDUCED,
            cli_reach: REACHABLE,
            export_summary: CollaborationControlExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:control-grant-active-driver-unprovable:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "control_authority_source",
                "single_active_driver",
                "grant_scope_and_expiry",
            ]),
            full_ready_claim: M5CollaborationControlA11yClaim::ExplicitlyGrantedControlSurface,
            claim_conditions: vec![condition(
                M5CollaborationControlClaimDimension::ActiveDriverClarity,
                M5CollaborationControlConditionState::ActiveDriverUnprovableOrMultiDriver,
            )],
            claim_narrow: Some(CollaborationControlClaimAutoNarrow {
                narrowed_to: M5CollaborationControlA11yClaim::ActiveDriverUnverifiedProjection,
                binding_dimension: M5CollaborationControlClaimDimension::ActiveDriverClarity,
                trigger: M5CollaborationControlDowngradeTrigger::ActiveDriverUnstated,
                narrowed_label:
                    "This control grant's single active driver is unprovable or a second driver is contending — shown as an active-driver-unverified projection that keeps the grant authority, granted scope, time-box, expiry, and revoke / reclaim path explicit, never allowing more than one active driver to hold mutating control of the same sensitive surface"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "control_authority_source",
                "single_active_driver",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CollaborationControlConsumerSurface::ControlGrantPrompt,
                M5CollaborationControlConsumerSurface::PasteSecretGuard,
            ]),
            source_refs: vec![
                "TAD v1.25 §19.4.1 — explicit, time-boxed, single-driver control grants".to_owned(),
                COLLABORATION_CONTROL_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-18T00:00:00Z".to_owned(),
            evidence_refs: ev("control-grant-active-driver-unprovable"),
        },
        // Presenter token (handoff unprovable / contested) — structure-heavy (presenter holder / handoff chain /
        // moderation scope); the presenter handoff is unprovable or contested, so it auto-narrows to a
        // presenter-handoff-unverified projection that keeps the presenter holder, handoff chain, and moderation
        // scope explicit, never letting moderation silently transfer shell / debug control (yellow). Its dense
        // reflow narrows the high-zoom legibility to a disclosed reduction.
        CollaborationControlAccessibilityRow {
            record_kind: COLLABORATION_CONTROL_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: COLLABORATION_CONTROL_A11Y_SCHEMA_VERSION,
            row_id: "a11y:presenter-token-handoff-contested".to_owned(),
            object: M5CollaborationControlObject::PresenterToken,
            source_object_schema_ref: M5CollaborationControlObject::PresenterToken
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "collab:presenter-token:0005".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REDUCED,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: CollaborationControlExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:presenter-token-handoff-contested:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "presenter_holder_and_handoff_chain",
                "moderation_scope",
                "contested_handoff_reason",
            ]),
            full_ready_claim: M5CollaborationControlA11yClaim::ExplicitlyGrantedControlSurface,
            claim_conditions: vec![condition(
                M5CollaborationControlClaimDimension::PresenterHandoffClarity,
                M5CollaborationControlConditionState::PresenterHandoffUnprovableOrContested,
            )],
            claim_narrow: Some(CollaborationControlClaimAutoNarrow {
                narrowed_to: M5CollaborationControlA11yClaim::PresenterHandoffUnverifiedProjection,
                binding_dimension: M5CollaborationControlClaimDimension::PresenterHandoffClarity,
                trigger: M5CollaborationControlDowngradeTrigger::ViewFirstDefaultUnstated,
                narrowed_label:
                    "This presenter token's handoff is unprovable or contested — shown as a presenter-handoff-unverified projection that keeps the presenter / moderator holder, handoff chain, and moderation scope explicit, never letting moderation silently transfer shell / debug control or two presenters drive one sensitive surface"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "presenter_holder_and_handoff_chain",
                "moderation_scope",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CollaborationControlConsumerSurface::PresenterHandoffSheet,
                M5CollaborationControlConsumerSurface::SessionRestoreView,
            ]),
            source_refs: vec![
                "TAD v1.25 §19.4.1 — presenter-handoff architecture & moderation".to_owned(),
                COLLABORATION_CONTROL_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-18T00:00:00Z".to_owned(),
            evidence_refs: ev("presenter-token-handoff-contested"),
        },
        // Consent envelope (join-time scope undisclosed / would widen silently) — the join-time consent scope is
        // undisclosed or would widen silently, so it auto-narrows to a consent-scope-unverified projection that
        // keeps the recording, retention, guest-scope, and route-visibility consequences explicit before join,
        // never widening scope without a fresh visible consent event (yellow).
        CollaborationControlAccessibilityRow {
            record_kind: COLLABORATION_CONTROL_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: COLLABORATION_CONTROL_A11Y_SCHEMA_VERSION,
            row_id: "a11y:consent-envelope-scope-undisclosed".to_owned(),
            object: M5CollaborationControlObject::ConsentEnvelope,
            source_object_schema_ref: M5CollaborationControlObject::ConsentEnvelope
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "collab:consent-envelope:0006".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REDUCED,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: CollaborationControlExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:consent-envelope-scope-undisclosed:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "join_time_consent_scope",
                "recording_retention_guest_route_consequences",
                "undisclosed_scope_reason",
            ]),
            full_ready_claim: M5CollaborationControlA11yClaim::ExplicitlyGrantedControlSurface,
            claim_conditions: vec![condition(
                M5CollaborationControlClaimDimension::ConsentScopeClarity,
                M5CollaborationControlConditionState::ConsentScopeUndisclosedOrWidened,
            )],
            claim_narrow: Some(CollaborationControlClaimAutoNarrow {
                narrowed_to: M5CollaborationControlA11yClaim::ConsentScopeUnverifiedProjection,
                binding_dimension: M5CollaborationControlClaimDimension::ConsentScopeClarity,
                trigger: M5CollaborationControlDowngradeTrigger::ConsentScopeUnstated,
                narrowed_label:
                    "This consent envelope's join-time scope is undisclosed or would widen silently — shown as a consent-scope-unverified projection that keeps the recording, retention, guest-scope, and route-visibility consequences explicit before a participant joins, never widening recording, retention, guest scope, or route visibility without a fresh visible consent event"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "join_time_consent_scope",
                "recording_retention_guest_route_consequences",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CollaborationControlConsumerSurface::CollaborationJoinReviewSheet,
                M5CollaborationControlConsumerSurface::PresenterHandoffSheet,
            ]),
            source_refs: vec![
                "TAD v1.25 §19.4.2 — collaboration consent / recording / retention envelope".to_owned(),
                COLLABORATION_CONTROL_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-18T00:00:00Z".to_owned(),
            evidence_refs: ev("consent-envelope-scope-undisclosed"),
        },
        // Retention review (recording / retention state stale / would broaden silently) — the recording /
        // retention state is stale or would broaden silently, so it auto-narrows to a retention-state-unverified
        // projection that keeps the recording state, retention mode and duration, and replayable-archive scope
        // explicit, never starting recording or broadening retention silently (yellow).
        CollaborationControlAccessibilityRow {
            record_kind: COLLABORATION_CONTROL_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: COLLABORATION_CONTROL_A11Y_SCHEMA_VERSION,
            row_id: "a11y:retention-review-retention-stale".to_owned(),
            object: M5CollaborationControlObject::RetentionReview,
            source_object_schema_ref: M5CollaborationControlObject::RetentionReview
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "collab:retention-review:0007".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REDUCED,
            export_summary: CollaborationControlExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:retention-review-retention-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "recording_state_and_retention_mode",
                "replayable_archive_scope",
                "stale_retention_reason",
            ]),
            full_ready_claim: M5CollaborationControlA11yClaim::ExplicitlyGrantedControlSurface,
            claim_conditions: vec![condition(
                M5CollaborationControlClaimDimension::RetentionStateClarity,
                M5CollaborationControlConditionState::RetentionStateStaleOrBroadenedSilently,
            )],
            claim_narrow: Some(CollaborationControlClaimAutoNarrow {
                narrowed_to: M5CollaborationControlA11yClaim::RetentionStateUnverifiedProjection,
                binding_dimension: M5CollaborationControlClaimDimension::RetentionStateClarity,
                trigger: M5CollaborationControlDowngradeTrigger::RetentionStateUnstated,
                narrowed_label:
                    "This retention review's recording / retention state is stale or would broaden silently — shown as a retention-state-unverified projection that keeps the recording state, retention mode and duration, and replayable-archive scope explicit, never starting recording, transcript retention, or replayable archives or broadening retention silently"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "recording_state_and_retention_mode",
                "replayable_archive_scope",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CollaborationControlConsumerSurface::CollaborationRetentionSheet,
                M5CollaborationControlConsumerSurface::HelpDocs,
            ]),
            source_refs: vec![
                "TAD v1.25 §19.4.2 — recording / retention / evidence architecture".to_owned(),
                COLLABORATION_CONTROL_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-18T00:00:00Z".to_owned(),
            evidence_refs: ev("retention-review-retention-stale"),
        },
        // Session-restore view (replay-free restore safety unprovable) — structure-heavy (restore transcript
        // class / replay-free render summary / retention scope / reopened-target set); the replay-free restore
        // safety is unprovable, so it auto-narrows to a restore-replay-safety-unverified projection that
        // reattaches read-only, keeps the restore transcript class and retention scope explicit, and requires a
        // fresh control grant before write control resumes, never replaying prior terminal / debug input on join
        // or restore (yellow).
        CollaborationControlAccessibilityRow {
            record_kind: COLLABORATION_CONTROL_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: COLLABORATION_CONTROL_A11Y_SCHEMA_VERSION,
            row_id: "a11y:session-restore-view-replay-safety-unprovable".to_owned(),
            object: M5CollaborationControlObject::SessionRestoreView,
            source_object_schema_ref: M5CollaborationControlObject::SessionRestoreView
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "collab:session-restore-view:0008".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: CollaborationControlExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:session-restore-view-replay-safety-unprovable:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "restore_transcript_class",
                "replay_free_render_summary",
                "reopened_fresh_grant_requirement",
            ]),
            full_ready_claim: M5CollaborationControlA11yClaim::ExplicitlyGrantedControlSurface,
            claim_conditions: vec![condition(
                M5CollaborationControlClaimDimension::RestoreReplaySafetyClarity,
                M5CollaborationControlConditionState::RestoreReplaySafetyUnprovable,
            )],
            claim_narrow: Some(CollaborationControlClaimAutoNarrow {
                narrowed_to: M5CollaborationControlA11yClaim::RestoreReplaySafetyUnverifiedProjection,
                binding_dimension: M5CollaborationControlClaimDimension::RestoreReplaySafetyClarity,
                trigger: M5CollaborationControlDowngradeTrigger::RestoreReplaySafetyUnstated,
                narrowed_label:
                    "This session-restore view's replay-free restore safety is unprovable — shown as a restore-replay-safety-unverified projection that reattaches read-only, keeps the restore transcript class, replay-free render summary, and retention scope explicit, and requires a fresh control grant before write control resumes, never replaying prior terminal / debug input, signals, or breakpoint edits on join or restore"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "restore_transcript_class",
                "replay_free_render_summary",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CollaborationControlConsumerSurface::SessionRestoreView,
                M5CollaborationControlConsumerSurface::ControlGrantPrompt,
            ]),
            source_refs: vec![
                "TDD v3.6 — restore / no-rerun & fresh-grant re-request".to_owned(),
                COLLABORATION_CONTROL_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-18T00:00:00Z".to_owned(),
            evidence_refs: ev("session-restore-view-replay-safety-unprovable"),
        },
    ]
}
