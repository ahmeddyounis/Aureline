//! Inline tests for the M5 design-system foundation package.

use super::*;

fn canonical() -> M5FoundationPackage {
    seeded_m5_foundation_package()
}

#[test]
fn canonical_package_validates() {
    let package = canonical();
    assert!(package.validate().is_empty(), "{:?}", package.validate());
    assert_eq!(package.record_kind, M5_FOUNDATION_PACKAGE_RECORD_KIND);
    assert_eq!(package.package_id, M5_FOUNDATION_PACKAGE_ID);
    assert_eq!(package.package_version, M5_FOUNDATION_PACKAGE_VERSION);
}

#[test]
fn package_publishes_every_governed_family_kind() {
    let package = canonical();
    for kind in M5FoundationFamilyKind::ALL {
        let family = package
            .family(kind)
            .unwrap_or_else(|| panic!("missing {}", kind.as_str()));
        assert_eq!(family.family_kind, kind);
        assert!(family.family_version >= 1);
        assert!(!family.entries.is_empty());
    }
    assert_eq!(package.families.len(), M5FoundationFamilyKind::ALL.len());
}

#[test]
fn density_motion_contrast_state_rows_read_from_the_same_canonical_vocabulary() {
    let package = canonical();
    // The density / motion / contrast / state rows resolve the exact tokens aureline_ui and the
    // canonical state vocabulary publish, so they cannot drift by surface family.
    assert_eq!(
        package.density_tokens(),
        vec!["compact", "standard", "comfortable"]
    );
    assert_eq!(
        package.motion_postures(),
        vec![
            "motion_standard",
            "motion_reduced",
            "motion_low_motion",
            "motion_power_saver",
            "motion_critical_hot_path",
        ]
    );
    assert_eq!(
        package.contrast_tokens(),
        vec![
            "dark_reference",
            "light_parity",
            "high_contrast_dark",
            "high_contrast_light",
        ]
    );
    assert_eq!(
        package.high_contrast_tokens(),
        vec!["high_contrast_dark", "high_contrast_light"]
    );
    assert_eq!(
        package.state_tokens(),
        vec![
            "empty",
            "loading",
            "pending",
            "degraded",
            "blocked",
            "error",
            "completed"
        ]
    );
    assert_eq!(
        package.reduced_motion_entry().map(|e| e.entry_id.as_str()),
        Some("motion.reduced")
    );
    assert_eq!(
        package.power_saving_entry().map(|e| e.entry_id.as_str()),
        Some("motion.power_saver")
    );
}

#[test]
fn downgraded_entries_stay_inspectable() {
    let package = canonical();
    let downgraded = package.downgraded_entries();
    assert_eq!(downgraded.len(), 3, "{downgraded:?}");
    for (_, entry) in &downgraded {
        assert!(entry.is_downgraded());
        let downgrade = entry.downgrade.as_ref().expect("downgrade present");
        assert!(!downgrade.downgraded_to.trim().is_empty());
        assert!(downgrade
            .reason_message_id
            .starts_with(M5_FOUNDATION_MESSAGE_ID_PREFIX));
    }
    // The unsupported icon token is not dropped; it points at its fallback.
    let icon = package.family(M5FoundationFamilyKind::Icon).unwrap();
    let legacy = icon
        .entry("icon.legacy.spinner")
        .expect("legacy icon retained");
    assert_eq!(legacy.support_state, M5SupportState::Unsupported);
    assert_eq!(
        legacy.downgrade.as_ref().unwrap().downgraded_to,
        "icon.progress.spinner"
    );
}

#[test]
fn export_import_round_trips_and_revalidates() {
    let package = canonical();
    let json = package.export_safe_json();
    let imported = M5FoundationPackage::from_json(&json).expect("imports");
    assert_eq!(imported, package);
    assert!(imported.validate().is_empty());
}

#[test]
fn diff_names_added_removed_changed_and_downgraded_without_dropping_information() {
    let from = canonical();
    let to = seeded_m5_foundation_package_next();
    let diff = from.diff(&to);

    assert_eq!(diff.from_version, "1.0.0");
    assert_eq!(diff.to_version, "1.1.0");
    assert!(diff.retains_unsupported_and_downgraded);
    // Only the color family changed.
    assert_eq!(diff.family_diffs.len(), 1);
    let color = &diff.family_diffs[0];
    assert_eq!(color.family_kind, M5FoundationFamilyKind::Color);
    assert_eq!(color.from_version, Some(1));
    assert_eq!(color.to_version, Some(2));

    assert_eq!(color.added_entries, vec!["color.text.tertiary"]);

    // The removed entry is retained with its last support state, not dropped.
    assert_eq!(color.removed_entries.len(), 1);
    assert_eq!(color.removed_entries[0].entry_id, "color.state.success");
    assert_eq!(
        color.removed_entries[0].last_support_state,
        M5SupportState::Supported
    );

    // Value change + support downgrade, sorted by entry id.
    let changed_ids: Vec<&str> = color
        .changed_entries
        .iter()
        .map(|c| c.entry_id.as_str())
        .collect();
    assert_eq!(
        changed_ids,
        vec!["color.surface.raised", "color.text.muted"]
    );
    let raised = &color.changed_entries[0];
    assert!(raised.value_changed);
    let muted = &color.changed_entries[1];
    assert!(!muted.value_changed);
    assert_eq!(muted.support_from, M5SupportState::Deprecated);
    assert_eq!(muted.support_to, M5SupportState::Unsupported);

    assert_eq!(color.downgraded_entries, vec!["color.text.muted"]);

    assert_eq!(diff.added_entry_count, 1);
    assert_eq!(diff.removed_entry_count, 1);
    assert_eq!(diff.changed_entry_count, 2);
    assert_eq!(diff.downgraded_entry_count, 1);
}

#[test]
fn identical_packages_diff_empty() {
    let package = canonical();
    let diff = package.diff(&canonical());
    assert!(diff.is_empty());
    assert_eq!(diff.added_entry_count, 0);
    assert_eq!(diff.removed_entry_count, 0);
}

#[test]
fn release_packet_preserves_downgraded_entries() {
    let package = canonical();
    let release = package.release_packet();
    assert_eq!(release.package_version, "1.0.0");
    assert_eq!(release.total_entries, 36);
    assert_eq!(release.total_downgraded, 3);
    assert_eq!(release.total_supported, 33);
    assert_eq!(release.family_summaries.len(), 8);
    assert_eq!(release.downgraded_entries.len(), 3);
    let ids: Vec<&str> = release
        .downgraded_entries
        .iter()
        .map(|d| d.entry_id.as_str())
        .collect();
    assert_eq!(
        ids,
        vec![
            "color.text.muted",
            "icon.legacy.spinner",
            "space.legacy.tight"
        ]
    );
}

#[test]
fn validation_rejects_supported_entry_with_downgrade() {
    let mut package = canonical();
    let color = package
        .families
        .iter_mut()
        .find(|f| f.family_kind == M5FoundationFamilyKind::Color)
        .unwrap();
    color.entries[0].downgrade = Some(M5EntryDowngrade {
        downgraded_to: "color.text.primary".to_owned(),
        reason_message_id: format!("{}bogus.downgrade", M5_FOUNDATION_MESSAGE_ID_PREFIX),
        since_package_version: "1.0.0".to_owned(),
    });
    assert!(package
        .validate()
        .contains(&M5FoundationPackageViolation::DowngradeInconsistent));
}

#[test]
fn validation_rejects_downgraded_entry_without_downgrade() {
    let mut package = canonical();
    let color = package
        .families
        .iter_mut()
        .find(|f| f.family_kind == M5FoundationFamilyKind::Color)
        .unwrap();
    let muted = color
        .entries
        .iter_mut()
        .find(|e| e.entry_id == "color.text.muted")
        .unwrap();
    muted.downgrade = None;
    assert!(package
        .validate()
        .contains(&M5FoundationPackageViolation::DowngradeInconsistent));
}

#[test]
fn validation_rejects_density_row_drift() {
    let mut package = canonical();
    let density = package
        .families
        .iter_mut()
        .find(|f| f.family_kind == M5FoundationFamilyKind::Density)
        .unwrap();
    density.entries[0].value_token = "ultra_compact".to_owned();
    assert!(package
        .validate()
        .contains(&M5FoundationPackageViolation::DensityRowsIncomplete));
}

#[test]
fn validation_rejects_bad_package_version() {
    let mut package = canonical();
    package.package_version = "1.0".to_owned();
    assert!(package
        .validate()
        .contains(&M5FoundationPackageViolation::BadPackageVersion));
}

#[test]
fn validation_rejects_duplicate_family_kind() {
    let mut package = canonical();
    let extra = package.families[0].clone();
    package.families.push(extra);
    let violations = package.validate();
    assert!(violations.contains(&M5FoundationPackageViolation::DuplicateFamilyKind));
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
    let from_disk = current_stable_m5_foundation_package().expect("checked package validates");
    assert_eq!(
        from_disk,
        canonical(),
        "checked foundation package drifted from the seed builder"
    );
}

#[test]
fn checked_next_fixture_matches_seed() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-foundation-package/foundation-package-next.json"
    ));
    let from_disk = M5FoundationPackage::from_json(raw).expect("next fixture parses");
    assert!(
        from_disk.validate().is_empty(),
        "{:?}",
        from_disk.validate()
    );
    assert_eq!(from_disk, seeded_m5_foundation_package_next());
}

#[test]
fn checked_diff_fixture_matches_computed_diff() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-foundation-package/foundation-package-diff.json"
    ));
    let from_disk: M5FoundationPackageDiff =
        serde_json::from_str(raw).expect("diff fixture parses");
    let computed = seeded_m5_foundation_package().diff(&seeded_m5_foundation_package_next());
    assert_eq!(
        from_disk, computed,
        "checked diff drifted from the computed diff"
    );
}

#[test]
fn checked_release_packet_matches_computed() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-design-system-proof/foundation-package-release.json"
    ));
    let from_disk: M5FoundationPackageReleasePacket =
        serde_json::from_str(raw).expect("release packet parses");
    assert_eq!(
        from_disk,
        seeded_m5_foundation_package().release_packet(),
        "checked release packet drifted from the computed release packet"
    );
}
