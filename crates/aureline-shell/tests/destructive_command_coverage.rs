//! Registry coverage for destructive and external-effect command preview gates.

use aureline_commands::registry::seeded_registry;

#[test]
fn destructive_or_external_effect_commands_have_preview_metadata() {
    let registry = seeded_registry();
    let mut inspected_count = 0usize;
    let mut missing_metadata = Vec::new();
    let mut incomplete_gate_metadata = Vec::new();

    for entry in registry.entries() {
        let Some(effect_class) = entry.destructive_or_external_effect_class() else {
            continue;
        };
        inspected_count += 1;

        if !entry.has_preview_or_gate_metadata() {
            missing_metadata.push(format!("{} ({effect_class})", entry.command_id()));
            continue;
        }

        if let Some(metadata) = entry.preview_gate_metadata.as_ref() {
            if metadata.review_surface_refs.is_empty()
                || metadata.evidence_ref_class_required.is_empty()
                || metadata.apply_guard_ref.trim().is_empty()
                || metadata.revert_posture_class.trim().is_empty()
            {
                incomplete_gate_metadata.push(entry.command_id().clone());
            }
        }
    }

    assert!(
        inspected_count > 0,
        "coverage test must inspect at least one high-effect command"
    );
    assert!(
        missing_metadata.is_empty(),
        "high-effect commands without preview metadata: {}",
        missing_metadata.join(", ")
    );
    assert!(
        incomplete_gate_metadata.is_empty(),
        "commands with incomplete preview-gate metadata: {}",
        incomplete_gate_metadata.join(", ")
    );

    let save = registry
        .get("cmd:editor.save")
        .expect("save command remains in the canonical registry");
    assert!(
        save.preview_gate_metadata.is_some(),
        "save command must stay covered by the save-review gate metadata"
    );
}
