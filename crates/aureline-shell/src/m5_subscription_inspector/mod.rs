//! Shell consumer for the cross-surface subscription contract.
//!
//! This module is the shell's end-to-end path onto the shared
//! subscription envelope. It drives the canonical contract in
//! [`aureline_reactive_state::subscriptions`]: it builds the bus from the
//! seeded bindings, publishes one representative frame per binding through
//! it, and renders the **subscription inspector** — the deterministic
//! rows the shell, CLI, and support review surfaces show when a user needs
//! to see, for each binding, **which authority published the current
//! view**, **which scope and epoch it belongs to**, and **what claim the
//! surface may present**.
//!
//! It does not cache its own reactive truth and it does not invent any
//! stale-state vocabulary: the inspector rows are a projection of one
//! shared [`aureline_reactive_state::PublishOutcome`], so every subscribed
//! surface — shell, search, graph, AI, review, support — sees the
//! identical stable subscription fields.

use std::fmt;

use aureline_reactive_state::{
    seeded_cross_surface_subscription_fixtures, seeded_cross_surface_subscription_packet,
    validate_cross_surface_subscription_packet, CrossSurfaceSubscriptionBus,
    CrossSurfaceSubscriptionValidationReport, PublishOutcome, SubscriptionError,
    SubscriptionInspectorReport,
};

/// Presentation label rendered for the subscription inspector.
pub const M5_SUBSCRIPTION_INSPECTOR_PRESENTATION_LABEL: &str = "M5 subscription inspector";

/// Presentation subtitle rendered for the subscription inspector.
pub const M5_SUBSCRIPTION_INSPECTOR_PRESENTATION_SUBTITLE: &str =
    "Show which authority published the current view of each binding, which scope and epoch it belongs to, and what claim the surface may present.";

/// Error returned when the inspector cannot project rows.
#[derive(Debug)]
pub enum M5SubscriptionInspectorError {
    /// The canonical contract failed validation.
    PacketValidation(CrossSurfaceSubscriptionValidationReport),
    /// A frame could not be published through the bus.
    Publish(SubscriptionError),
}

impl fmt::Display for M5SubscriptionInspectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PacketValidation(report) => {
                write!(f, "cross-surface subscription invalid: {report}")
            }
            Self::Publish(err) => write!(f, "subscription publish failed: {err}"),
        }
    }
}

impl std::error::Error for M5SubscriptionInspectorError {}

impl From<CrossSurfaceSubscriptionValidationReport> for M5SubscriptionInspectorError {
    fn from(report: CrossSurfaceSubscriptionValidationReport) -> Self {
        Self::PacketValidation(report)
    }
}

impl From<SubscriptionError> for M5SubscriptionInspectorError {
    fn from(err: SubscriptionError) -> Self {
        Self::Publish(err)
    }
}

/// Drives the seeded contract end-to-end: validates the bindings,
/// publishes one representative frame per binding through the shared bus,
/// and returns the inspector report naming which authority published each
/// current view and which scope and epoch it belongs to.
///
/// # Errors
///
/// Returns [`M5SubscriptionInspectorError`] when the contract fails
/// validation or a frame cannot be published.
pub fn build_subscription_inspector_report(
) -> Result<SubscriptionInspectorReport, M5SubscriptionInspectorError> {
    let packet = seeded_cross_surface_subscription_packet();
    validate_cross_surface_subscription_packet(&packet)?;
    let mut bus = CrossSurfaceSubscriptionBus::from_packet(&packet);
    for fixture in seeded_cross_surface_subscription_fixtures() {
        bus.publish(&fixture.binding_id, &fixture.frame)?;
    }
    Ok(bus.inspector_report())
}

/// Renders the subscription inspector as deterministic plaintext for CLI,
/// support review, and docs consumers.
///
/// # Errors
///
/// Returns [`M5SubscriptionInspectorError`] when the contract fails
/// validation or a frame cannot be published.
pub fn render_subscription_inspector_plaintext() -> Result<String, M5SubscriptionInspectorError> {
    let report = build_subscription_inspector_report()?;
    let mut lines = vec![
        M5_SUBSCRIPTION_INSPECTOR_PRESENTATION_LABEL.to_string(),
        "binding | authority | scope | epoch.delta | claim | subscribers".to_string(),
    ];
    for row in &report.rows {
        let s = &row.subscription;
        lines.push(format!(
            "{} | {} | {}:{} | {}.{} | {} | {}",
            s.binding_id,
            s.authority_class.as_str(),
            s.scope_class.as_str(),
            s.scope_id,
            s.snapshot_epoch,
            s.delta_seq,
            s.truth_claim.as_str(),
            join_surfaces(&row.consumer_surfaces),
        ));
    }
    lines.push(String::new());
    Ok(lines.join("\n"))
}

/// Publishes the all-six cross-surface binding's first seeded frame and
/// returns the per-surface fan-out, so a reviewer can confirm every
/// subscriber observed the identical shared envelope.
///
/// # Errors
///
/// Returns [`M5SubscriptionInspectorError`] when the contract fails
/// validation or the frame cannot be published.
pub fn cross_surface_fan_out_demo() -> Result<PublishOutcome, M5SubscriptionInspectorError> {
    let packet = seeded_cross_surface_subscription_packet();
    validate_cross_surface_subscription_packet(&packet)?;
    let fixtures = seeded_cross_surface_subscription_fixtures();
    let frame = &fixtures
        .iter()
        .find(|f| f.binding_id == "binding:workspace_tree")
        .expect("seeded contract carries the all-six workspace-tree binding")
        .frame;
    let mut bus = CrossSurfaceSubscriptionBus::from_packet(&packet);
    Ok(bus.publish("binding:workspace_tree", frame)?)
}

fn join_surfaces(surfaces: &[aureline_reactive_state::ConsumerSurface]) -> String {
    surfaces
        .iter()
        .map(|surface| surface.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_reactive_state::ConsumerView;

    #[test]
    fn inspector_names_authority_scope_and_epoch_for_every_binding() {
        let report = build_subscription_inspector_report().expect("report builds");
        // One row per seeded binding.
        assert_eq!(report.rows.len(), 8);
        // Rows name the publishing authority and a concrete scope.
        for row in &report.rows {
            assert!(!row.subscription.scope_id.is_empty());
            assert!(!row.consumer_surfaces.is_empty());
        }
        // The review overlay row names the provider authority as unavailable.
        let review = report
            .rows
            .iter()
            .find(|r| r.subscription.binding_id == "binding:review_overlay")
            .expect("review overlay row present");
        assert_eq!(
            review.subscription.authority_class.as_str(),
            "provider_overlay"
        );
        assert_eq!(
            review.subscription.truth_claim.as_str(),
            "provider_unavailable"
        );
    }

    #[test]
    fn plaintext_is_deterministic_and_names_authorities() {
        let first = render_subscription_inspector_plaintext().expect("plaintext");
        let second = render_subscription_inspector_plaintext().expect("plaintext");
        assert_eq!(first, second);
        assert!(first.contains("M5 subscription inspector"));
        assert!(first.contains("workspace_vfs"));
        assert!(first.contains("provider_overlay"));
        assert!(first.contains("binding:workspace_tree"));
    }

    #[test]
    fn fan_out_demo_shares_one_envelope_across_all_six_surfaces() {
        let outcome = cross_surface_fan_out_demo().expect("fan-out");
        assert_eq!(outcome.views.len(), 6);
        // Every subscriber observed the identical stable subscription fields.
        for ConsumerView { subscription, .. } in &outcome.views {
            assert_eq!(subscription, &outcome.stable);
        }
        // The shared frame is the canonical subscription envelope.
        assert!(outcome
            .envelope_json
            .contains("\"subscription_schema_version\""));
    }
}
