//! Cross-surface M5 reactive-state explainer.
//!
//! This module projects the canonical M5 reactive-governance matrix into
//! the deterministic explainability rows the shell, CLI, and support
//! review surfaces render when a user needs to understand, for each
//! reactive surface, **where its truth came from**, **how fresh it is**,
//! **whether scope is partial**, and **what invalidation can change it**.
//!
//! It does not reinvent any stale-state vocabulary: every label is read
//! from the matrix in
//! [`aureline_reactive_state::m5_reactive_governance`], and every claim
//! downgrade is computed by the canonical narrowing engine so the shell
//! cannot present a richer claim than the matrix permits.

use std::fmt;

use aureline_reactive_state::{
    narrow_m5_reactive_truth_claim, seeded_m5_reactive_governance_packet,
    validate_m5_reactive_governance_packet, M5ReactiveAuthorityClass, M5ReactiveBackpressureMode,
    M5ReactiveCompleteness, M5ReactiveDerivationClass, M5ReactiveFreshness,
    M5ReactiveGovernancePacket, M5ReactiveGovernanceValidationReport, M5ReactiveInvalidationReason,
    M5ReactiveObservedState, M5ReactiveScopeClass, M5ReactiveSurfaceClass, M5ReactiveSurfaceRow,
    M5ReactiveTruthClaim, M5ReactiveViewClass,
};

/// Presentation label rendered for the reactive-state explainer.
pub const M5_REACTIVE_STATE_EXPLAINER_PRESENTATION_LABEL: &str = "M5 reactive-state explainer";

/// Presentation subtitle rendered for the reactive-state explainer.
pub const M5_REACTIVE_STATE_EXPLAINER_PRESENTATION_SUBTITLE: &str =
    "Explain where each surface's truth came from, how fresh it is, whether scope is partial, and what invalidation can change it.";

/// One row rendered in the shell reactive-state explainer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ReactiveStateExplainerRow {
    /// Reactive surface class.
    pub surface_class: M5ReactiveSurfaceClass,
    /// Authority that owns the canonical truth.
    pub authority_class: M5ReactiveAuthorityClass,
    /// Whether the surface is authoritative or derived.
    pub derivation_class: M5ReactiveDerivationClass,
    /// Subscription scope class.
    pub scope_class: M5ReactiveScopeClass,
    /// Materialized-view class.
    pub view_class: M5ReactiveViewClass,
    /// Canonical query family.
    pub query_family: String,
    /// Strongest claim the surface presents when healthy.
    pub healthy_claim: M5ReactiveTruthClaim,
    /// Degraded freshness states the surface can present.
    pub supported_freshness: Vec<M5ReactiveFreshness>,
    /// Degraded completeness states the surface can present.
    pub supported_completeness: Vec<M5ReactiveCompleteness>,
    /// Non-realtime backpressure modes the surface can experience.
    pub supported_backpressure: Vec<M5ReactiveBackpressureMode>,
    /// Invalidation reasons the surface honors.
    pub honored_invalidation_reasons: Vec<M5ReactiveInvalidationReason>,
    /// Reviewer note.
    pub notes: String,
}

impl M5ReactiveStateExplainerRow {
    fn from_row(row: &M5ReactiveSurfaceRow) -> Self {
        Self {
            surface_class: row.surface_class,
            authority_class: row.authority_class,
            derivation_class: row.derivation_class,
            scope_class: row.scope_class,
            view_class: row.view_class,
            query_family: row.query_family.clone(),
            healthy_claim: row.healthy_claim,
            supported_freshness: row.supported_freshness.clone(),
            supported_completeness: row.supported_completeness.clone(),
            supported_backpressure: row.supported_backpressure.clone(),
            honored_invalidation_reasons: row.honored_invalidation_reasons.clone(),
            notes: row.notes.clone(),
        }
    }
}

/// Error returned when the explainer cannot project rows.
#[derive(Debug)]
pub enum M5ReactiveStateExplainerError {
    /// The canonical matrix failed validation.
    PacketValidation(M5ReactiveGovernanceValidationReport),
    /// A requested surface is missing from the matrix.
    UnknownSurface(M5ReactiveSurfaceClass),
}

impl fmt::Display for M5ReactiveStateExplainerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PacketValidation(report) => {
                write!(f, "m5 reactive governance invalid: {report}")
            }
            Self::UnknownSurface(surface) => {
                write!(f, "unknown reactive surface: {}", surface.as_str())
            }
        }
    }
}

impl std::error::Error for M5ReactiveStateExplainerError {}

impl From<M5ReactiveGovernanceValidationReport> for M5ReactiveStateExplainerError {
    fn from(report: M5ReactiveGovernanceValidationReport) -> Self {
        Self::PacketValidation(report)
    }
}

/// The narrowed claim a surface may present for an observed state,
/// paired with the surface's declared healthy claim for contrast.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplainedClaim {
    /// Surface under explanation.
    pub surface_class: M5ReactiveSurfaceClass,
    /// Claim the surface presents when healthy.
    pub healthy_claim: M5ReactiveTruthClaim,
    /// Claim the surface must narrow to for the observed state.
    pub narrowed_claim: M5ReactiveTruthClaim,
    /// Whether the observed state forced a downgrade from the healthy claim.
    pub narrowed: bool,
}

/// Builds reactive-state explainer rows from the canonical matrix.
///
/// # Errors
///
/// Returns [`M5ReactiveStateExplainerError`] when the matrix fails
/// validation.
pub fn build_m5_reactive_state_explainer_rows(
) -> Result<Vec<M5ReactiveStateExplainerRow>, M5ReactiveStateExplainerError> {
    let packet = seeded_m5_reactive_governance_packet();
    validate_m5_reactive_governance_packet(&packet)?;
    Ok(rows_from_packet(&packet))
}

/// Explains the claim a surface may present for an observed subscription
/// state, narrowing through the canonical engine.
///
/// # Errors
///
/// Returns [`M5ReactiveStateExplainerError`] when the matrix fails
/// validation or the surface is unknown.
pub fn explain_surface_claim(
    surface_class: M5ReactiveSurfaceClass,
    observed: &M5ReactiveObservedState,
) -> Result<ExplainedClaim, M5ReactiveStateExplainerError> {
    let packet = seeded_m5_reactive_governance_packet();
    validate_m5_reactive_governance_packet(&packet)?;
    let row = packet
        .surfaces
        .iter()
        .find(|row| row.surface_class == surface_class)
        .ok_or(M5ReactiveStateExplainerError::UnknownSurface(surface_class))?;
    let narrowed = narrow_m5_reactive_truth_claim(row.derivation_class, observed);
    Ok(ExplainedClaim {
        surface_class,
        healthy_claim: row.healthy_claim,
        narrowed_claim: narrowed.claim,
        narrowed: narrowed.claim != row.healthy_claim,
    })
}

/// Renders the explainer projection as deterministic plaintext for CLI,
/// support review, and docs consumers.
///
/// # Errors
///
/// Returns [`M5ReactiveStateExplainerError`] when the matrix fails
/// validation.
pub fn render_m5_reactive_state_explainer_plaintext(
) -> Result<String, M5ReactiveStateExplainerError> {
    let rows = build_m5_reactive_state_explainer_rows()?;
    let mut lines = vec![
        M5_REACTIVE_STATE_EXPLAINER_PRESENTATION_LABEL.to_string(),
        "surface | authority | view_class | scope | healthy_claim | degraded_freshness | invalidation".to_string(),
    ];
    for row in rows {
        lines.push(format!(
            "{} | {} | {} | {} | {} | {} | {}",
            row.surface_class.as_str(),
            row.authority_class.as_str(),
            row.view_class.as_str(),
            row.scope_class.as_str(),
            row.healthy_claim.as_str(),
            join_freshness(&row.supported_freshness),
            join_invalidation(&row.honored_invalidation_reasons),
        ));
    }
    lines.push(String::new());
    Ok(lines.join("\n"))
}

fn rows_from_packet(packet: &M5ReactiveGovernancePacket) -> Vec<M5ReactiveStateExplainerRow> {
    let mut rows: Vec<_> = packet
        .surfaces
        .iter()
        .map(M5ReactiveStateExplainerRow::from_row)
        .collect();
    rows.sort_by(|a, b| a.surface_class.as_str().cmp(b.surface_class.as_str()));
    rows
}

fn join_freshness(items: &[M5ReactiveFreshness]) -> String {
    items
        .iter()
        .map(|item| item.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

fn join_invalidation(items: &[M5ReactiveInvalidationReason]) -> String {
    items
        .iter()
        .map(|item| item.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows_build_from_canonical_matrix() {
        let rows = build_m5_reactive_state_explainer_rows().expect("rows build");
        assert_eq!(rows.len(), 13);
        // Every reactive surface is a derived projection with a non-exact claim.
        for row in &rows {
            assert_eq!(row.derivation_class, M5ReactiveDerivationClass::Derived);
            assert_ne!(row.healthy_claim, M5ReactiveTruthClaim::ExactCurrentTruth);
        }
    }

    #[test]
    fn explainer_narrows_a_stale_search_surface() {
        let observed = M5ReactiveObservedState {
            freshness: M5ReactiveFreshness::Stale,
            completeness: M5ReactiveCompleteness::Full,
            backpressure_mode: M5ReactiveBackpressureMode::Realtime,
            terminal_reason: None,
            policy_limited: false,
        };
        let explained =
            explain_surface_claim(M5ReactiveSurfaceClass::SearchResults, &observed).expect("ok");
        assert!(explained.narrowed);
        assert_eq!(
            explained.narrowed_claim,
            M5ReactiveTruthClaim::StaleSnapshot
        );
        assert_eq!(
            explained.healthy_claim,
            M5ReactiveTruthClaim::ConsistentSnapshot
        );
    }

    #[test]
    fn plaintext_is_deterministic_and_names_authority_and_invalidation() {
        let first = render_m5_reactive_state_explainer_plaintext().expect("plaintext");
        let second = render_m5_reactive_state_explainer_plaintext().expect("plaintext");
        assert_eq!(first, second);
        assert!(first.contains("M5 reactive-state explainer"));
        assert!(first.contains("workspace_vfs"));
        assert!(first.contains("provider_overlay"));
        assert!(first.contains("authority_epoch_rolled"));
    }
}
