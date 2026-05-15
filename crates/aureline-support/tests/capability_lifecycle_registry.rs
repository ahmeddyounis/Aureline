//! Fixture coverage for the capability lifecycle registry consumer.

use aureline_support::capabilities::{
    current_capability_lifecycle_registry, DenialReason, LifecycleState, MarkerKind,
    CURRENT_CAPABILITY_LIFECYCLE_REGISTRY_PATH,
};

#[test]
fn current_registry_parses_with_schema_vocabulary() {
    let registry = current_capability_lifecycle_registry().expect("registry parses");

    assert_eq!(registry.schema_version(), 1);
    assert_eq!(
        registry.registry_id(),
        "external_alpha_capability_lifecycle_registry"
    );
    assert_eq!(
        registry.lifecycle_vocabulary(),
        &[
            LifecycleState::Labs,
            LifecycleState::Preview,
            LifecycleState::Beta,
            LifecycleState::Stable,
            LifecycleState::Deprecated,
            LifecycleState::DisabledByPolicy,
            LifecycleState::Retired,
        ]
    );
    assert_eq!(
        registry
            .schema_projection()
            .get("Preview")
            .map(String::as_str),
        Some("preview")
    );
    assert!(
        !registry.surface_rows().is_empty(),
        "{CURRENT_CAPABILITY_LIFECYCLE_REGISTRY_PATH} should contain rows"
    );
}

#[test]
fn dependency_markers_normalize_registry_aliases_to_schema_kinds() {
    let registry = current_capability_lifecycle_registry().expect("registry parses");
    let marker = registry
        .marker_by_id("dependency_marker:alpha.helper_backed_service_preview")
        .expect("hosted marker exists");

    assert_eq!(marker.marker_kind(), MarkerKind::ProviderLinkedDependency);
    assert_eq!(
        marker.marker_kind().as_schema_token(),
        "provider_linked_dependency"
    );
}

#[test]
fn stable_claim_on_preview_lifecycle_fails_validation() {
    let registry = current_capability_lifecycle_registry().expect("registry parses");
    let row_refs = vec!["capability_lifecycle:alpha.ai.routing_cost".to_string()];

    let validation = registry.validate_claim(&row_refs, LifecycleState::Stable);

    assert!(!validation.is_valid());
    let failure = validation.failures().first().expect("failure is present");
    assert_eq!(
        failure.denial_reason(),
        DenialReason::ClaimEffectiveStateBelowDeclared
    );
    assert_eq!(
        failure.effective_lifecycle_state(),
        Some(LifecycleState::Preview)
    );
}
