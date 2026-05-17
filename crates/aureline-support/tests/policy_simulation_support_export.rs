//! Integration coverage for policy simulation and expiry support exports.

use aureline_support::policy_simulation::{
    seeded_policy_simulation_support_export, POLICY_SIMULATION_SUPPORT_EXPORT_RECORD_KIND,
};

#[test]
fn support_export_carries_action_time_policy_and_expiry_state() {
    let export = seeded_policy_simulation_support_export();
    assert_eq!(
        export.record_kind,
        POLICY_SIMULATION_SUPPORT_EXPORT_RECORD_KIND
    );
    assert!(export.preserves_historical_truth);
    assert!(export.raw_private_material_excluded);
    assert!(export.defect_kinds_present.is_empty());
    assert!(!export.action_time_policy_states.is_empty());
    assert!(export
        .page
        .summary
        .change_classes_present
        .contains(&"policy_bundle_change".to_owned()));
    assert!(export.page.summary.expiring_exception_count > 0);
    assert!(export.page.summary.drift_detected_count > 0);
}
