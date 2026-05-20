//! Live shell channel context for attention routing.
//!
//! The notification [`super::super::notifications::router::NotificationRouter`]
//! is the dedupe + suppression core: it reasons about the envelope's own
//! `suppression_state`. But the *live* shell knows facts the envelope cannot
//! know at mint time — whether the target window is focused, whether a screen
//! reader is driving the session, whether a companion device is reachable, and
//! whether the user is presenting. [`ChannelContext`] carries those live facts
//! into the routing decision so one governed router can resolve a single
//! envelope onto in-app toasts, banners, status overflow, the activity center,
//! native OS notifications, and companion fanout consistently.
//!
//! The context only ever *narrows* delivery (a focused foreground window does
//! not need a redundant OS toast; an unreachable companion cannot receive a
//! push). It never widens authority and never upgrades a held, suppressed, or
//! deduped surface back to delivered — those guardrails live in
//! [`super::outcome`].

use serde::{Deserialize, Serialize};

use crate::notifications::envelope::QuietHoursMode;
use crate::notifications::quiet_hours::QuietHoursPosture;

/// Where the target's window sits relative to user attention right now.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActiveWindowState {
    /// The target workspace window is foreground and focused. In-app surfaces
    /// are seen immediately, so redundant OS / lock-screen fanout is dropped.
    ForegroundFocused,
    /// A workspace window is foreground but the target is not focused. In-app
    /// surfaces deliver; an OS notification is still useful.
    ForegroundUnfocused,
    /// No Aureline window is foreground. The OS notification carries the
    /// interruption; in-app surfaces remain durable truth for the return.
    BackgroundHidden,
    /// The device is locked or the user is away. Only the lock-screen summary
    /// is an external path, and only when privacy posture allows it.
    LockedOrAway,
}

impl ActiveWindowState {
    /// Stable token recorded in outcomes and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ForegroundFocused => "foreground_focused",
            Self::ForegroundUnfocused => "foreground_unfocused",
            Self::BackgroundHidden => "background_hidden",
            Self::LockedOrAway => "locked_or_away",
        }
    }
}

/// Whether a screen reader is currently driving the session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScreenReaderPosture {
    /// No assistive technology posture reported.
    Inactive,
    /// A screen reader is active. Routing must keep a durable, navigable
    /// surface in the resolved set and mark the announcement as required.
    Active,
}

impl ScreenReaderPosture {
    /// Stable token recorded in outcomes and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inactive => "inactive",
            Self::Active => "active",
        }
    }

    /// True when a screen reader is active.
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }
}

/// Whether a companion endpoint is reachable for summary fanout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionAvailability {
    /// No companion endpoint is paired. Companion fanout is not attempted.
    Unpaired,
    /// A companion endpoint is paired and reachable.
    PairedAvailable,
    /// A companion endpoint is paired but currently unreachable (offline,
    /// stale, or backgrounded). Fanout is not attempted but stays durable.
    PairedUnavailable,
    /// A managed policy forbids companion fanout for this endpoint.
    PolicyBlocked,
}

impl CompanionAvailability {
    /// Stable token recorded in outcomes and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unpaired => "unpaired",
            Self::PairedAvailable => "paired_available",
            Self::PairedUnavailable => "paired_unavailable",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// True when a companion push could actually be delivered.
    pub const fn can_deliver(self) -> bool {
        matches!(self, Self::PairedAvailable)
    }
}

/// Presentation / follow posture. Folds into the effective quiet posture so
/// audience-visible surfaces stay quiet while durable truth keeps flowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationFollowState {
    /// Not presenting and not following another presenter.
    None,
    /// The user is presenting or screen-sharing this session.
    Presenting,
    /// The user is following another presenter's session.
    FollowingPresenter,
}

impl PresentationFollowState {
    /// Stable token recorded in outcomes and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Presenting => "presenting",
            Self::FollowingPresenter => "following_presenter",
        }
    }

    /// Quiet-hours mode this presentation/follow posture contributes, if any.
    fn implied_mode(self) -> Option<QuietHoursMode> {
        match self {
            Self::None => None,
            Self::Presenting | Self::FollowingPresenter => Some(QuietHoursMode::ModePresentation),
        }
    }
}

/// Serializable projection of the live channel context recorded on every
/// route outcome so support can replay why a surface resolved the way it did.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelContextSnapshot {
    pub active_window_state: ActiveWindowState,
    pub screen_reader_posture: ScreenReaderPosture,
    pub companion_availability: CompanionAvailability,
    pub presentation_follow_state: PresentationFollowState,
    /// Effective quiet-hours modes after folding in presentation/follow state.
    /// Empty means no quiet mode is active.
    pub active_quiet_hours_modes: Vec<QuietHoursMode>,
}

/// The shell's live attention-routing context.
///
/// One [`ChannelContext`] lives on the shell's notification truth lane next to
/// the [`super::super::notifications::router::NotificationRouter`]. It is read
/// each time an envelope routes so the resolved surface set reflects what the
/// user can actually see and reach right now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelContext {
    active_window_state: ActiveWindowState,
    screen_reader_posture: ScreenReaderPosture,
    companion_availability: CompanionAvailability,
    presentation_follow_state: PresentationFollowState,
    quiet_hours: QuietHoursPosture,
}

impl Default for ChannelContext {
    fn default() -> Self {
        Self {
            active_window_state: ActiveWindowState::ForegroundFocused,
            screen_reader_posture: ScreenReaderPosture::Inactive,
            companion_availability: CompanionAvailability::Unpaired,
            presentation_follow_state: PresentationFollowState::None,
            quiet_hours: QuietHoursPosture::none(),
        }
    }
}

impl ChannelContext {
    /// Build a context. Quiet-hours posture defaults to none; use
    /// [`Self::with_quiet_hours`] to layer it in.
    pub fn new(
        active_window_state: ActiveWindowState,
        screen_reader_posture: ScreenReaderPosture,
        companion_availability: CompanionAvailability,
        presentation_follow_state: PresentationFollowState,
    ) -> Self {
        Self {
            active_window_state,
            screen_reader_posture,
            companion_availability,
            presentation_follow_state,
            quiet_hours: QuietHoursPosture::none(),
        }
    }

    /// Foreground, focused, no companion, no quiet mode — the common case.
    pub fn foreground_focused() -> Self {
        Self::default()
    }

    /// Layer a quiet-hours posture onto the context.
    pub fn with_quiet_hours(mut self, posture: QuietHoursPosture) -> Self {
        self.quiet_hours = posture;
        self
    }

    /// Set the companion availability.
    pub fn with_companion(mut self, availability: CompanionAvailability) -> Self {
        self.companion_availability = availability;
        self
    }

    /// Set the screen-reader posture.
    pub fn with_screen_reader(mut self, posture: ScreenReaderPosture) -> Self {
        self.screen_reader_posture = posture;
        self
    }

    /// Set the active window state.
    pub fn with_window_state(mut self, state: ActiveWindowState) -> Self {
        self.active_window_state = state;
        self
    }

    /// Set the presentation/follow posture.
    pub fn with_presentation(mut self, state: PresentationFollowState) -> Self {
        self.presentation_follow_state = state;
        self
    }

    pub fn active_window_state(&self) -> ActiveWindowState {
        self.active_window_state
    }

    pub fn screen_reader_posture(&self) -> ScreenReaderPosture {
        self.screen_reader_posture
    }

    pub fn companion_availability(&self) -> CompanionAvailability {
        self.companion_availability
    }

    pub fn presentation_follow_state(&self) -> PresentationFollowState {
        self.presentation_follow_state
    }

    /// The effective quiet-hours posture: the shell's quiet posture unioned
    /// with any mode implied by the presentation/follow state. This never
    /// removes a mode an upstream subsystem already set.
    pub fn effective_posture(&self) -> QuietHoursPosture {
        let mut modes: Vec<QuietHoursMode> = self.quiet_hours.active_modes_sorted();
        if let Some(mode) = self.presentation_follow_state.implied_mode() {
            modes.push(mode);
        }
        QuietHoursPosture::with_modes(modes)
    }

    /// Serializable projection recorded on the route outcome.
    pub fn snapshot(&self) -> ChannelContextSnapshot {
        ChannelContextSnapshot {
            active_window_state: self.active_window_state,
            screen_reader_posture: self.screen_reader_posture,
            companion_availability: self.companion_availability,
            presentation_follow_state: self.presentation_follow_state,
            active_quiet_hours_modes: self.effective_posture().active_modes_sorted(),
        }
    }
}
