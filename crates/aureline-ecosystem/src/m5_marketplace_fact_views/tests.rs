use super::*;

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::{
    current_m5_ecosystem_governance_matrix, ArtifactFamily,
};

fn packet() -> M5MarketplaceFactViews {
    current_m5_marketplace_fact_views().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_MARKETPLACE_FACT_VIEWS_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_MARKETPLACE_FACT_VIEWS_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_views() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_view_is_mutually_consistent() {
    let packet = packet();
    assert!(packet.all_views_consistent());
    for row in &packet.result_rows {
        assert_eq!(
            row.disclosure_level,
            row.computed_disclosure_level(),
            "row {} disclosure level diverges from the recomputed level",
            row.row_id
        );
        assert_eq!(
            row.disclosure_reasons,
            row.computed_disclosure_reasons(),
            "row {} disclosure reasons diverge from the recomputed set",
            row.row_id
        );
    }
}

#[test]
fn every_detail_grid_reproduces_its_row() {
    let packet = packet();
    assert!(!packet.detail_grids.is_empty());
    for grid in &packet.detail_grids {
        let row = packet
            .result_row(&grid.row_ref)
            .unwrap_or_else(|| panic!("grid {} references a missing row", grid.grid_id));
        assert_eq!(grid.fact_set(), row.fact_set(), "{}", grid.grid_id);
        assert_eq!(
            grid.disclosure_reasons, row.disclosure_reasons,
            "{}",
            grid.grid_id
        );
        // Reduced provenance must never collapse a backing ref.
        assert!(
            grid.has_required_refs(),
            "grid {} is missing a backing ref",
            grid.grid_id
        );
    }
}

#[test]
fn every_compare_entry_reproduces_its_row() {
    let packet = packet();
    assert!(!packet.compare_views.is_empty());
    for view in &packet.compare_views {
        assert!(view.compared_row_refs.len() >= 2, "{}", view.compare_id);
        assert!(view.entries_cover_compared_rows(), "{}", view.compare_id);
        for entry in &view.entries {
            let row = packet.result_row(&entry.row_ref).unwrap_or_else(|| {
                panic!(
                    "compare entry references a missing row in {}",
                    view.compare_id
                )
            });
            assert_eq!(entry.fact_set(), row.fact_set(), "{}", view.compare_id);
            assert_eq!(
                entry.disclosure_reasons, row.disclosure_reasons,
                "{}",
                view.compare_id
            );
        }
    }
}

#[test]
fn export_projection_reflects_rows() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.result_rows.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_views_consistent,
        packet.all_views_consistent()
    );
    assert_eq!(
        projection.widened_disclosure_count,
        packet.widened_disclosure_rows().count()
    );
    for export in &projection.rows {
        let row = packet
            .result_row(&export.row_id)
            .expect("export row resolves");
        assert_eq!(export.disclosure_level, row.disclosure_level.as_str());
        assert_eq!(export.widened_disclosure, row.requires_widened_disclosure());
    }
}

#[test]
fn every_package_kind_is_represented() {
    let packet = packet();
    let present: BTreeSet<ArtifactFamily> =
        packet.result_rows.iter().map(|r| r.package_kind).collect();
    for kind in ArtifactFamily::ALL {
        assert!(
            present.contains(&kind),
            "no result row exercises package kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn every_source_class_is_represented() {
    let packet = packet();
    let present: BTreeSet<SourceClass> =
        packet.result_rows.iter().map(|r| r.source_class).collect();
    for source in SourceClass::ALL {
        assert!(
            present.contains(&source),
            "no result row exercises source class {}",
            source.as_str()
        );
    }
}

#[test]
fn every_discovery_channel_is_represented() {
    let packet = packet();
    let present: BTreeSet<DiscoveryChannel> = packet
        .result_rows
        .iter()
        .map(|r| r.discovery_channel)
        .collect();
    for channel in DiscoveryChannel::ALL {
        assert!(
            present.contains(&channel),
            "no result row exercises discovery channel {}",
            channel.as_str()
        );
    }
}

#[test]
fn every_mirror_posture_is_represented() {
    let packet = packet();
    let present: BTreeSet<MirrorPosture> = packet
        .result_rows
        .iter()
        .map(|r| r.mirror_posture)
        .collect();
    for posture in MirrorPosture::ALL {
        assert!(
            present.contains(&posture),
            "no result row exercises mirror posture {}",
            posture.as_str()
        );
    }
}

#[test]
fn every_bridge_native_state_is_represented() {
    let packet = packet();
    let present: BTreeSet<BridgeNativeState> = packet
        .result_rows
        .iter()
        .map(|r| r.bridge_native_state)
        .collect();
    for state in BridgeNativeState::ALL {
        assert!(
            present.contains(&state),
            "no result row exercises bridge/native state {}",
            state.as_str()
        );
    }
}

#[test]
fn every_disclosure_level_is_represented() {
    let packet = packet();
    let present: BTreeSet<DisclosureLevel> = packet
        .result_rows
        .iter()
        .map(|r| r.disclosure_level)
        .collect();
    for level in DisclosureLevel::ALL {
        assert!(
            present.contains(&level),
            "no result row exercises disclosure level {}",
            level.as_str()
        );
    }
}

#[test]
fn every_disclosure_reason_is_represented() {
    let packet = packet();
    let present: BTreeSet<DisclosureReason> = packet
        .result_rows
        .iter()
        .flat_map(|r| r.disclosure_reasons.iter().copied())
        .collect();
    for reason in DisclosureReason::ALL {
        assert!(
            present.contains(&reason),
            "no result row exercises disclosure reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn reduced_provenance_widens_to_heightened() {
    let packet = packet();
    for row in &packet.result_rows {
        if row
            .disclosure_reasons
            .contains(&DisclosureReason::ReducedProvenance)
        {
            assert_eq!(
                row.disclosure_level,
                DisclosureLevel::Heightened,
                "reduced provenance must widen row {} to heightened disclosure",
                row.row_id
            );
        }
    }
}

#[test]
fn mirror_variant_keeps_full_disclosure() {
    // The lane guardrail: a mirror/private/manual listing must keep every field and
    // never present a narrower warning than its facts warrant.
    let packet = packet();
    for row in &packet.result_rows {
        if row.mirror_posture.is_mirrored_or_private() || row.mirror_posture.is_manual_import() {
            assert!(
                row.requires_widened_disclosure(),
                "row {} is mirrored/private/manual but carries standard disclosure",
                row.row_id
            );
            let grid = packet
                .detail_grids
                .iter()
                .find(|g| g.row_ref == row.row_id)
                .unwrap_or_else(|| {
                    panic!(
                        "mirror/private/manual row {} has no detail grid",
                        row.row_id
                    )
                });
            assert!(
                grid.has_required_refs(),
                "mirror/private/manual row {} drops a backing ref",
                row.row_id
            );
        }
    }
}

#[test]
fn information_architecture_holds_across_channels() {
    // Every channel must surface the same fact axes; prove every row in every
    // channel carries a recomputed disclosure level and a detail grid.
    let packet = packet();
    for channel in DiscoveryChannel::ALL {
        for row in packet.rows_in_channel(channel) {
            assert!(row.disclosure_consistent(), "{}", row.row_id);
            assert!(
                packet.detail_grids.iter().any(|g| g.row_ref == row.row_id),
                "row {} in channel {} has no detail grid",
                row.row_id,
                channel.as_str()
            );
        }
    }
}

#[test]
fn every_row_resolves_to_a_governance_family() {
    // Fact-views build on the governance matrix; every row's package kind must be a
    // claimed governance family and its ref must point at that family's row.
    let packet = packet();
    let governance = current_m5_ecosystem_governance_matrix().expect("governance matrix parses");
    for row in &packet.result_rows {
        let family = governance.family(row.package_kind).unwrap_or_else(|| {
            panic!(
                "package kind {} is not a governance family",
                row.package_kind.as_str()
            )
        });
        assert_eq!(
            row.governance_family_ref, family.family_id,
            "row {} does not bind to its governance family",
            row.row_id
        );
    }
}

#[test]
fn validate_flags_disclosure_level_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .result_rows
        .iter_mut()
        .find(|r| r.disclosure_level != DisclosureLevel::Heightened)
    {
        row.disclosure_level = DisclosureLevel::Heightened;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5MarketplaceFactViewsViolation::DisclosureLevelMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_disclosure_reasons_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet.result_rows.iter_mut().find(|r| {
        !r.disclosure_reasons
            .contains(&DisclosureReason::SupportNarrowed)
    }) {
        row.disclosure_reasons
            .push(DisclosureReason::SupportNarrowed);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5MarketplaceFactViewsViolation::DisclosureReasonsMismatch { .. }
                | M5MarketplaceFactViewsViolation::DisclosureLevelMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_grid_drift() {
    let mut packet = packet();
    if let Some(grid) = packet.detail_grids.first_mut() {
        grid.support_class = match grid.support_class {
            SupportClass::FullySupported => SupportClass::Unsupported,
            _ => SupportClass::FullySupported,
        };
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5MarketplaceFactViewsViolation::GridDriftsFromRow { .. })));
    }
}

#[test]
fn validate_flags_dangling_row_ref() {
    let mut packet = packet();
    if let Some(grid) = packet.detail_grids.first_mut() {
        grid.row_ref = "row:does_not_exist".to_owned();
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5MarketplaceFactViewsViolation::DanglingRowRef { .. })));
    }
}

#[test]
fn validate_flags_compare_view_too_small() {
    let mut packet = packet();
    if let Some(view) = packet.compare_views.first_mut() {
        view.compared_row_refs.truncate(1);
        view.entries.truncate(1);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5MarketplaceFactViewsViolation::CompareViewTooSmall { .. }
        )));
    }
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_result_rows = packet.summary.total_result_rows.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5MarketplaceFactViewsViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(SourceClass::FirstParty.as_str(), "first_party");
    assert_eq!(SourceClass::Unverified.as_str(), "unverified");
    assert_eq!(BridgeNativeState::BridgeBacked.as_str(), "bridge_backed");
    assert_eq!(
        BridgeNativeState::LocalModelHosted.as_str(),
        "local_model_hosted"
    );
    assert_eq!(
        MirrorPosture::EnterpriseMirrored.as_str(),
        "enterprise_mirrored"
    );
    assert_eq!(
        MirrorPosture::ManuallyImported.as_str(),
        "manually_imported"
    );
    assert_eq!(DiscoveryChannel::ManualImport.as_str(), "manual_import");
    assert_eq!(DisclosureLevel::Heightened.as_str(), "heightened");
    assert_eq!(
        DisclosureReason::ReducedProvenance.as_str(),
        "reduced_provenance"
    );
}

#[test]
fn disclosure_level_widens_monotonically() {
    assert!(DisclosureLevel::Standard.rank() < DisclosureLevel::Caution.rank());
    assert!(DisclosureLevel::Caution.rank() < DisclosureLevel::Heightened.rank());
    assert_eq!(
        DisclosureLevel::Standard.widen(DisclosureLevel::Heightened),
        DisclosureLevel::Heightened
    );
    assert_eq!(
        DisclosureLevel::Heightened.widen(DisclosureLevel::Caution),
        DisclosureLevel::Heightened
    );
}
