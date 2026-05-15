//! Badge projection for capability lifecycle promotion gates.
//!
//! Shell surfaces use this projection to render the effective lifecycle state
//! and dependency marker count from the shared registry. The badge never
//! promotes a surface from its effective state; it only exposes whether a
//! stable-facing claim is currently blocked.

use aureline_support::capabilities::{
    CapabilityLifecycleRegistry, CapabilityLifecycleSurfaceRow, DependencyMarker, LifecycleState,
    MarkerKind,
};

/// Promotion-gate class rendered by shell lifecycle badges.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityPromotionGateClass {
    /// Effective lifecycle state satisfies a stable claim.
    StableReady,
    /// Effective lifecycle state is below stable.
    LifecycleNarrowed,
    /// Effective lifecycle state is disabled by policy.
    PolicyDisabled,
    /// Effective lifecycle state is retired.
    Retired,
}

/// Shell badge projection for one capability lifecycle row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityPromotionGateBadge {
    row_id: String,
    surface_ref: String,
    effective_lifecycle_state: LifecycleState,
    gate_class: CapabilityPromotionGateClass,
    marker_count: usize,
    marker_kinds: Vec<MarkerKind>,
    label: String,
}

impl CapabilityPromotionGateBadge {
    /// Returns the lifecycle row id backing this badge.
    pub fn row_id(&self) -> &str {
        &self.row_id
    }

    /// Returns the surface ref backing this badge.
    pub fn surface_ref(&self) -> &str {
        &self.surface_ref
    }

    /// Returns the effective lifecycle state rendered by the badge.
    pub const fn effective_lifecycle_state(&self) -> LifecycleState {
        self.effective_lifecycle_state
    }

    /// Returns the promotion-gate class.
    pub const fn gate_class(&self) -> CapabilityPromotionGateClass {
        self.gate_class
    }

    /// Returns whether this badge blocks a stable promotion claim.
    pub const fn blocks_stable_promotion(&self) -> bool {
        !matches!(self.gate_class, CapabilityPromotionGateClass::StableReady)
    }

    /// Returns the number of dependency markers attached to the row.
    pub const fn marker_count(&self) -> usize {
        self.marker_count
    }

    /// Returns marker kinds attached to the row.
    pub fn marker_kinds(&self) -> &[MarkerKind] {
        &self.marker_kinds
    }

    /// Returns the compact badge label.
    pub fn label(&self) -> &str {
        &self.label
    }
}

/// Builds lifecycle promotion-gate badges for every row in the registry.
pub fn promotion_gate_badges(
    registry: &CapabilityLifecycleRegistry,
) -> Vec<CapabilityPromotionGateBadge> {
    registry
        .surface_rows()
        .iter()
        .map(|row| promotion_gate_badge_for_row(row, &registry.markers_for_row(row)))
        .collect()
}

/// Builds a lifecycle promotion-gate badge for one row.
pub fn promotion_gate_badge_for_row(
    row: &CapabilityLifecycleSurfaceRow,
    markers: &[&DependencyMarker],
) -> CapabilityPromotionGateBadge {
    let effective_lifecycle_state = row.effective_lifecycle_state();
    let gate_class = match effective_lifecycle_state {
        LifecycleState::Stable | LifecycleState::LtsFacing => {
            CapabilityPromotionGateClass::StableReady
        }
        LifecycleState::DisabledByPolicy => CapabilityPromotionGateClass::PolicyDisabled,
        LifecycleState::Retired => CapabilityPromotionGateClass::Retired,
        LifecycleState::Labs
        | LifecycleState::Preview
        | LifecycleState::Beta
        | LifecycleState::Deprecated => CapabilityPromotionGateClass::LifecycleNarrowed,
    };
    let marker_kinds = markers.iter().map(|marker| marker.marker_kind()).collect();
    let label = match gate_class {
        CapabilityPromotionGateClass::StableReady => "stable",
        CapabilityPromotionGateClass::LifecycleNarrowed => {
            effective_lifecycle_state.as_schema_token()
        }
        CapabilityPromotionGateClass::PolicyDisabled => "disabled_by_policy",
        CapabilityPromotionGateClass::Retired => "retired",
    }
    .to_string();

    CapabilityPromotionGateBadge {
        row_id: row.row_id().to_string(),
        surface_ref: row.surface_ref().to_string(),
        effective_lifecycle_state,
        gate_class,
        marker_count: markers.len(),
        marker_kinds,
        label,
    }
}

#[cfg(test)]
mod tests {
    use aureline_support::capabilities::current_capability_lifecycle_registry;

    use super::*;

    #[test]
    fn preview_rows_block_stable_promotion_badges() {
        let registry = current_capability_lifecycle_registry().expect("registry parses");
        let row = registry
            .row_by_id("capability_lifecycle:alpha.ai.routing_cost")
            .expect("fixture row exists");
        let badge = promotion_gate_badge_for_row(row, &registry.markers_for_row(row));

        assert_eq!(badge.effective_lifecycle_state(), LifecycleState::Preview);
        assert_eq!(
            badge.gate_class(),
            CapabilityPromotionGateClass::LifecycleNarrowed
        );
        assert!(badge.blocks_stable_promotion());
        assert_eq!(badge.marker_count(), 1);
    }

    #[test]
    fn policy_disabled_rows_render_policy_gate() {
        let registry = current_capability_lifecycle_registry().expect("registry parses");
        let row = registry
            .row_by_id("capability_lifecycle:alpha.managed_cloud_disabled")
            .expect("fixture row exists");
        let badge = promotion_gate_badge_for_row(row, &registry.markers_for_row(row));

        assert_eq!(
            badge.gate_class(),
            CapabilityPromotionGateClass::PolicyDisabled
        );
        assert!(badge.blocks_stable_promotion());
    }
}
