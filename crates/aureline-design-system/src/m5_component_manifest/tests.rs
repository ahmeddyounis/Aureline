//! Inline tests for the M5 design-system component-manifest package.

use std::collections::BTreeSet;

use super::*;

use crate::m5_foundation_package::seeded_m5_foundation_package;

fn canonical() -> M5ComponentManifestPackage {
    seeded_m5_component_manifest_package()
}

#[test]
fn canonical_package_validates() {
    let package = canonical();
    assert!(package.validate().is_empty(), "{:?}", package.validate());
    assert_eq!(
        package.record_kind,
        M5_COMPONENT_MANIFEST_PACKAGE_RECORD_KIND
    );
    assert_eq!(package.package_id, M5_COMPONENT_MANIFEST_PACKAGE_ID);
    assert_eq!(
        package.package_version,
        M5_COMPONENT_MANIFEST_PACKAGE_VERSION
    );
}

#[test]
fn package_publishes_one_manifest_per_component_kind() {
    let package = canonical();
    for kind in M5ComponentKind::ALL {
        let manifest = package
            .manifest(kind)
            .unwrap_or_else(|| panic!("missing {}", kind.as_str()));
        assert_eq!(manifest.component_kind, kind);
        assert_eq!(
            manifest.component_id,
            format!("design-system:component:{}", kind.as_str())
        );
        assert!(!manifest.anatomy.is_empty());
        assert!(!manifest.required_parts().is_empty());
    }
    assert_eq!(package.manifests.len(), M5ComponentKind::ALL.len());
}

#[test]
fn every_manifest_classifies_the_full_canonical_state_set() {
    let package = canonical();
    let canonical_states: BTreeSet<CanonicalStateClass> =
        CanonicalStateClass::required().iter().copied().collect();
    for manifest in &package.manifests {
        let mandatory: BTreeSet<CanonicalStateClass> =
            manifest.states.mandatory.iter().copied().collect();
        let optional: BTreeSet<CanonicalStateClass> =
            manifest.states.optional.iter().copied().collect();
        assert!(
            !manifest.states.mandatory.is_empty(),
            "{} has no mandatory states",
            manifest.component_id
        );
        assert!(
            mandatory.is_disjoint(&optional),
            "{} mandatory/optional overlap",
            manifest.component_id
        );
        let union: BTreeSet<CanonicalStateClass> = mandatory.union(&optional).copied().collect();
        assert_eq!(
            union, canonical_states,
            "{} does not classify the full canonical state set",
            manifest.component_id
        );
    }
}

#[test]
fn token_dependencies_resolve_to_published_foundation_entries() {
    // Each manifest renders from foundation token references that the foundation package actually
    // publishes, so the two lanes read from one shared source rather than feature-local wiring.
    let foundation = seeded_m5_foundation_package();
    let published: BTreeSet<String> = foundation
        .families
        .iter()
        .flat_map(|f| f.entries.iter().map(|e| e.entry_id.clone()))
        .collect();
    for manifest in &canonical().manifests {
        for token in &manifest.token_dependencies {
            assert!(
                published.contains(token),
                "{} depends on unpublished foundation token {token}",
                manifest.component_id
            );
        }
    }
}

#[test]
fn labels_and_commands_carry_governed_message_ids() {
    for manifest in &canonical().manifests {
        for label in &manifest.labels {
            assert!(label.message_id.starts_with(M5_COMPONENT_MESSAGE_ID_PREFIX));
        }
        for command in &manifest.commands {
            assert!(command
                .label_message_id
                .starts_with(M5_COMPONENT_MESSAGE_ID_PREFIX));
            assert!(!command.keys.trim().is_empty());
        }
        assert!(manifest
            .summary_message_id
            .starts_with(M5_COMPONENT_MESSAGE_ID_PREFIX));
    }
}

#[test]
fn export_import_round_trips_and_revalidates() {
    let package = canonical();
    let json = package.export_safe_json();
    let imported = M5ComponentManifestPackage::from_json(&json).expect("imports");
    assert_eq!(imported, package);
    assert!(imported.validate().is_empty());
}

#[test]
fn release_packet_projects_one_summary_per_manifest() {
    let package = canonical();
    let release = package.release_packet();
    assert_eq!(release.package_version, "1.0.0");
    assert_eq!(release.total_manifests, M5ComponentKind::ALL.len() as u32);
    assert_eq!(release.manifest_summaries.len(), M5ComponentKind::ALL.len());
    for (manifest, summary) in package.manifests.iter().zip(&release.manifest_summaries) {
        assert_eq!(summary.component_kind, manifest.component_kind);
        assert_eq!(summary.component_id, manifest.component_id);
        assert_eq!(summary.lifecycle_state, manifest.lifecycle.lifecycle_state);
        assert_eq!(
            summary.manifest_version,
            manifest.lifecycle.manifest_version
        );
        assert_eq!(summary.anatomy_part_count, manifest.anatomy.len() as u32);
        assert_eq!(
            summary.mandatory_state_count,
            manifest.states.mandatory.len() as u32
        );
        assert_eq!(summary.command_count, manifest.commands.len() as u32);
        assert_eq!(
            summary.keyboard_binding_count,
            manifest.keyboard.len() as u32
        );
        assert_eq!(
            summary.token_dependency_count,
            manifest.token_dependencies.len() as u32
        );
    }
}

#[test]
fn validation_rejects_bad_package_version() {
    let mut package = canonical();
    package.package_version = "1.0".to_owned();
    assert!(package
        .validate()
        .contains(&M5ComponentManifestViolation::BadPackageVersion));
}

#[test]
fn validation_rejects_duplicate_component_kind() {
    let mut package = canonical();
    let extra = package.manifests[0].clone();
    package.manifests.push(extra);
    let violations = package.validate();
    assert!(violations.contains(&M5ComponentManifestViolation::DuplicateComponentKind));
}

#[test]
fn validation_rejects_missing_component_kind() {
    let mut package = canonical();
    package.manifests.pop();
    assert!(package
        .validate()
        .contains(&M5ComponentManifestViolation::RequiredComponentKindMissing));
}

#[test]
fn validation_rejects_state_set_that_does_not_cover_canonical() {
    let mut package = canonical();
    // Drop a state from both mandatory and optional so the union no longer equals the canonical
    // set.
    let manifest = &mut package.manifests[0];
    manifest
        .states
        .mandatory
        .retain(|s| *s != CanonicalStateClass::Empty);
    manifest
        .states
        .optional
        .retain(|s| *s != CanonicalStateClass::Empty);
    assert!(package
        .validate()
        .contains(&M5ComponentManifestViolation::StatesIncomplete));
}

#[test]
fn validation_rejects_overlapping_mandatory_and_optional_states() {
    let mut package = canonical();
    let manifest = &mut package.manifests[0];
    let dup = manifest.states.mandatory[0];
    manifest.states.optional.push(dup);
    assert!(package
        .validate()
        .contains(&M5ComponentManifestViolation::StatesIncomplete));
}

#[test]
fn validation_rejects_anatomy_without_a_required_part() {
    let mut package = canonical();
    for part in &mut package.manifests[0].anatomy {
        part.required = false;
    }
    assert!(package
        .validate()
        .contains(&M5ComponentManifestViolation::AnatomyIncomplete));
}

#[test]
fn validation_rejects_lifecycle_without_version() {
    let mut package = canonical();
    package.manifests[0].lifecycle.manifest_version = 0;
    assert!(package
        .validate()
        .contains(&M5ComponentManifestViolation::LifecycleIncomplete));
}

#[test]
fn validation_rejects_empty_keyboard_model() {
    let mut package = canonical();
    package.manifests[0].keyboard.clear();
    assert!(package
        .validate()
        .contains(&M5ComponentManifestViolation::KeyboardIncomplete));
}

#[test]
fn validation_rejects_accessibility_without_notes() {
    let mut package = canonical();
    package.manifests[0].accessibility.notes.clear();
    assert!(package
        .validate()
        .contains(&M5ComponentManifestViolation::AccessibilityIncomplete));
}

#[test]
fn validation_rejects_missing_source_contracts() {
    let mut package = canonical();
    package.source_contract_refs = vec!["schemas/design-system/wrong.schema.json".to_owned()];
    assert!(package
        .validate()
        .contains(&M5ComponentManifestViolation::MissingSourceContracts));
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = canonical().export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("authorization"));
    assert!(!json.contains("bearer "));
}

#[test]
fn checked_package_fixture_matches_seed_and_validates() {
    let from_disk =
        current_stable_m5_component_manifest_package().expect("checked package validates");
    assert_eq!(
        from_disk,
        canonical(),
        "checked component-manifest package drifted from the seed builder"
    );
}

#[test]
fn checked_per_manifest_fixtures_match_seed() {
    // Each per-manifest fixture is a single manifest extracted from the package, so the gallery has
    // a stable, cite-able file per component family.
    let package = canonical();
    let fixtures: &[(M5ComponentKind, &str)] = &[
        (
            M5ComponentKind::PlaceholderCard,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/component-manifest-placeholder_card.json"
            )),
        ),
        (
            M5ComponentKind::StateBlock,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/component-manifest-state_block.json"
            )),
        ),
        (
            M5ComponentKind::ReviewSheet,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/component-manifest-review_sheet.json"
            )),
        ),
        (
            M5ComponentKind::JobRow,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/component-manifest-job_row.json"
            )),
        ),
        (
            M5ComponentKind::BoundaryBar,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/component-manifest-boundary_bar.json"
            )),
        ),
        (
            M5ComponentKind::FormControl,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/component-manifest-form_control.json"
            )),
        ),
        (
            M5ComponentKind::DenseCollection,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/component-manifest-dense_collection.json"
            )),
        ),
    ];
    for (kind, raw) in fixtures {
        let from_disk: M5ComponentManifest =
            serde_json::from_str(raw).expect("per-manifest fixture parses");
        let seeded = package.manifest(*kind).expect("manifest present");
        assert_eq!(
            &from_disk,
            seeded,
            "checked manifest fixture for {} drifted from the seed",
            kind.as_str()
        );
    }
}

#[test]
fn checked_release_packet_matches_computed() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-design-system-proof/component-manifest-release.json"
    ));
    let from_disk: M5ComponentManifestReleasePacket =
        serde_json::from_str(raw).expect("release packet parses");
    assert_eq!(
        from_disk,
        canonical().release_packet(),
        "checked release packet drifted from the computed release packet"
    );
}
