//! Shell consumer for the canonical reactive-truth cue layer.
//!
//! Each derived M5 surface the shell renders — the workspace tree, the
//! activity center, search results, graph neighborhoods, the AI context
//! panel, review and preview panes, the docs browser, and companion and
//! policy overlays — shows one **reactive-truth strip** built from the
//! canonical cue in
//! [`aureline_reactive_state::reactive_truth_surfaces`]. The strip names
//! where the surface's truth came from, how fresh and complete it is, what
//! invalidation changed it, and whether dangerous derived actions stay
//! live, all in the one shared grammar the engine emits.
//!
//! This module renders; it never decides. Every claim, gate, invalidation
//! reason, and resubscribe cue is read from the cue layer, so the shell can
//! never present a richer claim than the canonical engine permits, and the
//! UI strip, CLI/headless line, activity-center row, keyboard-help line,
//! and accessibility narration all carry the same tokens.

use std::fmt;

use aureline_reactive_state::{
    build_reactive_truth_cue, render_reactive_truth_cue, M5ReactiveInvalidationReason,
    M5ReactiveObservedState, M5ReactiveSurfaceClass, M5ReactiveTruthClaim, ReactiveTruthActionGate,
    ReactiveTruthCue, ReactiveTruthCueChannel, ReactiveTruthSurfacesError,
};

/// Presentation label rendered above the reactive-truth strips.
pub const REACTIVE_TRUTH_SURFACES_PRESENTATION_LABEL: &str = "Reactive-truth surfaces";

/// Presentation subtitle rendered above the reactive-truth strips.
pub const REACTIVE_TRUTH_SURFACES_PRESENTATION_SUBTITLE: &str =
    "For each derived surface: where its truth came from, how fresh it is, what changed it, and whether dangerous actions stay live.";

/// One rendered reactive-truth strip for a derived shell surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellReactiveTruthStrip {
    /// Reactive surface class.
    pub surface_class: M5ReactiveSurfaceClass,
    /// Canonically narrowed claim for the observed state.
    pub narrowed_claim: M5ReactiveTruthClaim,
    /// Where the surface's truth came from, in one phrase.
    pub source_summary: String,
    /// One-line truth headline.
    pub headline: String,
    /// Action gate governing dangerous derived actions.
    pub action_gate: ReactiveTruthActionGate,
    /// Whether a dangerous derived action stays live.
    pub dangerous_action_enabled: bool,
    /// Whether the surface must resubscribe before it can recover.
    pub resubscribe_required: bool,
    /// Dominant invalidation reason behind the narrowed claim.
    pub invalidation_reason: Option<M5ReactiveInvalidationReason>,
    /// Shell truth-strip line (UI).
    pub ui_strip_line: String,
    /// CLI / headless line.
    pub cli_line: String,
    /// Activity-center row.
    pub activity_center_line: String,
    /// Keyboard-help line.
    pub keyboard_help_line: String,
    /// Accessibility narration.
    pub narration: String,
}

impl ShellReactiveTruthStrip {
    fn from_cue(cue: &ReactiveTruthCue) -> Self {
        Self {
            surface_class: cue.surface_class,
            narrowed_claim: cue.narrowed_claim,
            source_summary: cue.source_summary.clone(),
            headline: cue.headline.clone(),
            action_gate: cue.action_gate,
            dangerous_action_enabled: cue.dangerous_action_enabled,
            resubscribe_required: cue.resubscribe_required,
            invalidation_reason: cue.invalidation_reason,
            ui_strip_line: render_reactive_truth_cue(cue, ReactiveTruthCueChannel::UiStrip),
            cli_line: render_reactive_truth_cue(cue, ReactiveTruthCueChannel::CliHeadless),
            activity_center_line: render_reactive_truth_cue(
                cue,
                ReactiveTruthCueChannel::ActivityCenter,
            ),
            keyboard_help_line: render_reactive_truth_cue(
                cue,
                ReactiveTruthCueChannel::KeyboardHelp,
            ),
            narration: render_reactive_truth_cue(cue, ReactiveTruthCueChannel::Accessibility),
        }
    }
}

/// Error returned when a strip cannot be rendered.
#[derive(Debug)]
pub struct ShellReactiveTruthError(ReactiveTruthSurfacesError);

impl fmt::Display for ShellReactiveTruthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "reactive-truth strip unavailable: {}", self.0)
    }
}

impl std::error::Error for ShellReactiveTruthError {}

impl From<ReactiveTruthSurfacesError> for ShellReactiveTruthError {
    fn from(err: ReactiveTruthSurfacesError) -> Self {
        Self(err)
    }
}

/// Every derived surface the shell renders a reactive-truth strip for.
const SHELL_SURFACES: [M5ReactiveSurfaceClass; 13] = [
    M5ReactiveSurfaceClass::ShellWorkspaceTree,
    M5ReactiveSurfaceClass::ShellActivityCenter,
    M5ReactiveSurfaceClass::EditorBufferOutline,
    M5ReactiveSurfaceClass::SearchResults,
    M5ReactiveSurfaceClass::GraphNeighborhood,
    M5ReactiveSurfaceClass::DocsBrowser,
    M5ReactiveSurfaceClass::AiContextPanel,
    M5ReactiveSurfaceClass::ReviewWorkspace,
    M5ReactiveSurfaceClass::PreviewOutput,
    M5ReactiveSurfaceClass::CompanionPanel,
    M5ReactiveSurfaceClass::PolicyTrustBanner,
    M5ReactiveSurfaceClass::HeadlessWorkspaceMirror,
    M5ReactiveSurfaceClass::SupportExportView,
];

/// Renders the reactive-truth strip for a surface and an observed state.
///
/// # Errors
///
/// Returns [`ShellReactiveTruthError`] when the canonical matrix fails
/// validation or the surface is unknown.
pub fn explain_surface_truth(
    surface_class: M5ReactiveSurfaceClass,
    observed: &M5ReactiveObservedState,
) -> Result<ShellReactiveTruthStrip, ShellReactiveTruthError> {
    let cue = build_reactive_truth_cue(surface_class, *observed)?;
    Ok(ShellReactiveTruthStrip::from_cue(&cue))
}

/// Whether a dangerous (mutating) derived action stays live for a surface
/// and observed state. Returns `false` for any degraded state that cannot
/// prove a consistent snapshot.
///
/// # Errors
///
/// Returns [`ShellReactiveTruthError`] when the surface is unknown.
pub fn dangerous_derived_action_enabled(
    surface_class: M5ReactiveSurfaceClass,
    observed: &M5ReactiveObservedState,
) -> Result<bool, ShellReactiveTruthError> {
    let cue = build_reactive_truth_cue(surface_class, *observed)?;
    Ok(cue.dangerous_action_enabled)
}

/// Builds the healthy-baseline reactive-truth strips for every derived
/// shell surface.
///
/// # Errors
///
/// Returns [`ShellReactiveTruthError`] when the canonical matrix fails
/// validation.
pub fn build_reactive_truth_strips() -> Result<Vec<ShellReactiveTruthStrip>, ShellReactiveTruthError>
{
    let healthy = M5ReactiveObservedState::healthy();
    let mut strips = Vec::with_capacity(SHELL_SURFACES.len());
    for surface in SHELL_SURFACES {
        strips.push(explain_surface_truth(surface, &healthy)?);
    }
    strips.sort_by(|a, b| a.surface_class.as_str().cmp(b.surface_class.as_str()));
    Ok(strips)
}

/// Renders the healthy-baseline strips as deterministic plaintext for CLI,
/// support review, and docs consumers.
///
/// # Errors
///
/// Returns [`ShellReactiveTruthError`] when the canonical matrix fails
/// validation.
pub fn render_reactive_truth_surfaces_plaintext() -> Result<String, ShellReactiveTruthError> {
    let strips = build_reactive_truth_strips()?;
    let mut lines = vec![
        REACTIVE_TRUTH_SURFACES_PRESENTATION_LABEL.to_string(),
        "surface | claim | gate | dangerous_action | resubscribe".to_string(),
    ];
    for strip in strips {
        lines.push(format!(
            "{} | {} | {} | {} | {}",
            strip.surface_class.as_str(),
            strip.narrowed_claim.as_str(),
            strip.action_gate.as_str(),
            strip.dangerous_action_enabled,
            strip.resubscribe_required,
        ));
    }
    lines.push(String::new());
    Ok(lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    use aureline_reactive_state::{
        M5ReactiveBackpressureMode, M5ReactiveCompleteness, M5ReactiveFreshness,
        M5ReactiveTerminalReason,
    };

    fn stale() -> M5ReactiveObservedState {
        M5ReactiveObservedState {
            freshness: M5ReactiveFreshness::Stale,
            completeness: M5ReactiveCompleteness::Full,
            backpressure_mode: M5ReactiveBackpressureMode::Realtime,
            terminal_reason: None,
            policy_limited: false,
        }
    }

    #[test]
    fn strips_build_for_every_surface() {
        let strips = build_reactive_truth_strips().expect("strips build");
        assert_eq!(strips.len(), 13);
        for strip in &strips {
            // The healthy ceiling is a consistent snapshot with live actions.
            assert_eq!(
                strip.narrowed_claim,
                M5ReactiveTruthClaim::ConsistentSnapshot
            );
            assert_eq!(strip.action_gate, ReactiveTruthActionGate::Enabled);
            assert!(strip.dangerous_action_enabled);
            assert!(!strip.resubscribe_required);
        }
    }

    #[test]
    fn stale_search_blocks_dangerous_actions_and_names_the_cause() {
        let strip =
            explain_surface_truth(M5ReactiveSurfaceClass::SearchResults, &stale()).expect("strip");
        assert_eq!(strip.narrowed_claim, M5ReactiveTruthClaim::StaleSnapshot);
        assert_eq!(strip.action_gate, ReactiveTruthActionGate::Blocked);
        assert!(!strip.dangerous_action_enabled);
        assert_eq!(
            strip.invalidation_reason,
            Some(M5ReactiveInvalidationReason::UpstreamInputStale)
        );
        // Every channel carries the same claim + gate tokens.
        for line in [
            &strip.ui_strip_line,
            &strip.cli_line,
            &strip.activity_center_line,
            &strip.keyboard_help_line,
        ] {
            assert!(line.contains("stale_snapshot"));
            assert!(line.contains("blocked"));
        }
        assert!(strip.narration.contains("upstream_input_stale"));
    }

    #[test]
    fn unavailable_provider_requires_resubscribe() {
        let observed = M5ReactiveObservedState {
            freshness: M5ReactiveFreshness::Authoritative,
            completeness: M5ReactiveCompleteness::Unavailable,
            backpressure_mode: M5ReactiveBackpressureMode::Realtime,
            terminal_reason: Some(M5ReactiveTerminalReason::Unavailable),
            policy_limited: false,
        };
        let enabled =
            dangerous_derived_action_enabled(M5ReactiveSurfaceClass::CompanionPanel, &observed)
                .expect("gate");
        assert!(!enabled);
        let strip = explain_surface_truth(M5ReactiveSurfaceClass::CompanionPanel, &observed)
            .expect("strip");
        assert!(strip.resubscribe_required);
        assert_eq!(
            strip.narrowed_claim,
            M5ReactiveTruthClaim::ProviderUnavailable
        );
    }

    #[test]
    fn plaintext_is_deterministic() {
        let first = render_reactive_truth_surfaces_plaintext().expect("plaintext");
        let second = render_reactive_truth_surfaces_plaintext().expect("plaintext");
        assert_eq!(first, second);
        assert!(first.contains("Reactive-truth surfaces"));
        assert!(first.contains("search_results"));
        assert!(first.contains("consistent_snapshot"));
    }
}
