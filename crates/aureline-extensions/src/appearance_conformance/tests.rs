//! Unit and fixture coverage for extension appearance conformance.

use super::{
    audit_appearance_conformance_rows, evaluate_appearance_conformance_row,
    project_appearance_conformance_support_export, project_appearance_conformance_support_row,
    seeded_appearance_conformance_inputs, seeded_appearance_conformance_packet,
    validate_appearance_conformance_packet, validate_appearance_conformance_support_export,
    AppearanceAxisClass, AppearanceConformanceDecisionClass, AppearanceConformanceDefectKind,
    AppearanceConformanceInput, AppearanceConformancePacket, AppearanceConformanceReasonClass,
    AppearanceConformanceSupportExport, AppearanceProofClass, AppearanceSupportClass,
    AppearanceSurfaceClass, EXTENSION_APPEARANCE_CONFORMANCE_SHARED_CONTRACT_REF,
};
use crate::webview_boundary::ExtensionInheritanceClass;

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/extensions/m3/appearance_inheritance"
);

fn load_json<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let path = format!("{}/{}", FIXTURE_DIR, filename);
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

fn axis_proof(
    input: &mut AppearanceConformanceInput,
    axis: AppearanceAxisClass,
    proof: AppearanceProofClass,
) {
    for probe in &mut input.probes {
        if probe.axis == axis {
            probe.proof_class = proof;
        }
    }
}

#[test]
fn seeded_packet_validates_with_required_coverage_and_decisions() {
    let packet = seeded_appearance_conformance_packet();
    validate_appearance_conformance_packet(&packet).expect("seeded packet must validate");
    assert_eq!(
        packet.shared_contract_ref,
        EXTENSION_APPEARANCE_CONFORMANCE_SHARED_CONTRACT_REF
    );
    assert_eq!(packet.summary.row_count, 4);
    assert_eq!(packet.summary.defect_count, 0);
    assert_eq!(packet.summary.conformant_row_count, 3);
    assert_eq!(packet.summary.needs_review_row_count, 1);
    assert_eq!(packet.summary.refused_row_count, 0);
    assert_eq!(packet.summary.fully_inherited_row_count, 1);
    assert_eq!(packet.summary.reduced_support_row_count, 2);
    assert_eq!(packet.summary.unsupported_row_count, 1);
    assert_eq!(packet.summary.overclaimed_row_count, 0);

    for axis in [
        AppearanceAxisClass::Theme,
        AppearanceAxisClass::Density,
        AppearanceAxisClass::FocusRing,
        AppearanceAxisClass::HighContrast,
        AppearanceAxisClass::ReducedMotion,
        AppearanceAxisClass::HostToken,
    ] {
        assert!(
            packet.summary.axes_present.contains(&axis),
            "seeded packet must cover axis {axis:?}"
        );
    }
    for surface in [
        AppearanceSurfaceClass::MarketplaceResultRow,
        AppearanceSurfaceClass::MarketplaceDetailPage,
        AppearanceSurfaceClass::InstallReview,
        AppearanceSurfaceClass::SideloadReview,
        AppearanceSurfaceClass::MirroredBundleReview,
        AppearanceSurfaceClass::PostInstallDiagnostics,
    ] {
        assert!(
            packet.summary.surfaces_present.contains(&surface),
            "seeded packet must cover surface {surface:?}"
        );
    }
}

#[test]
fn only_proven_inheritance_earns_a_full_badge() {
    let packet = seeded_appearance_conformance_packet();
    let full_row = packet
        .rows
        .iter()
        .find(|row| row.overall_support_class == AppearanceSupportClass::FullInheritance)
        .expect("seeded packet has a fully inherited row");
    assert_eq!(
        full_row.decision_class,
        AppearanceConformanceDecisionClass::Conformant
    );
    assert!(full_row
        .surface_caveats
        .iter()
        .all(|caveat| caveat.implies_full_inheritance));

    for row in &packet.rows {
        if row.overall_support_class != AppearanceSupportClass::FullInheritance {
            assert!(
                row.surface_caveats
                    .iter()
                    .all(|caveat| !caveat.implies_full_inheritance),
                "row {} must not imply full inheritance",
                row.row_id
            );
        }
    }
}

#[test]
fn unproven_claim_is_downgraded_to_needs_review_not_badged_full() {
    let packet = seeded_appearance_conformance_packet();
    let row = packet
        .rows
        .iter()
        .find(|row| row.decision_class == AppearanceConformanceDecisionClass::NeedsReview)
        .expect("seeded packet has a needs-review row");
    assert_eq!(
        row.reason_class,
        AppearanceConformanceReasonClass::NeedsVerificationBeforeBadge
    );
    assert!(row.axes.iter().any(|axis| axis.requires_verification));
    assert_ne!(
        row.overall_support_class,
        AppearanceSupportClass::FullInheritance
    );
}

#[test]
fn appearance_caveats_persist_after_install_on_every_row() {
    let packet = seeded_appearance_conformance_packet();
    for row in &packet.rows {
        let diagnostics = row
            .surface_caveats
            .iter()
            .find(|caveat| caveat.surface_class == AppearanceSurfaceClass::PostInstallDiagnostics)
            .expect("every row carries a post-install diagnostics caveat");
        assert!(diagnostics.persists_after_install);
        assert!(!diagnostics.caveat_line.trim().is_empty());
        assert!(diagnostics.host_labels_line.contains("trust="));
    }
}

#[test]
fn support_export_replays_visible_rows_without_private_material() {
    let packet = seeded_appearance_conformance_packet();
    let export = project_appearance_conformance_support_export(
        &packet,
        "extension-appearance-conformance:support-export:default",
        "2026-05-20T00:00:00Z",
    );
    validate_appearance_conformance_support_export(&packet, &export)
        .expect("support export must validate");
    assert!(export.raw_private_material_excluded);
    assert_eq!(export.support_rows.len(), packet.rows.len());
    for row in &packet.rows {
        let support = export
            .support_rows
            .iter()
            .find(|support| support.row_ref == row.row_id)
            .unwrap_or_else(|| panic!("missing support row for {}", row.row_id));
        assert_eq!(support.overall_support_class, row.overall_support_class);
        assert_eq!(support.decision_class, row.decision_class);
        assert_eq!(support.host_trust_label, row.host_trust_label);
        assert_eq!(support.axis_support_by_token.len(), row.axes.len());
    }
}

#[test]
fn checked_packet_fixture_matches_seeded_builder() {
    let from_file: AppearanceConformancePacket = load_json("conformance_packet.json");
    let from_code = seeded_appearance_conformance_packet();
    assert_eq!(from_file, from_code);
}

#[test]
fn checked_support_export_fixture_matches_seeded_builder() {
    let from_file: AppearanceConformanceSupportExport = load_json("support_export.json");
    let packet = seeded_appearance_conformance_packet();
    let from_code = project_appearance_conformance_support_export(
        &packet,
        "extension-appearance-conformance:support-export:default",
        "2026-05-20T00:00:00Z",
    );
    assert_eq!(from_file, from_code);
}

#[test]
fn input_fixtures_project_to_checked_packet() {
    let inputs: Vec<AppearanceConformanceInput> = load_json("inputs.json");
    let rows = inputs
        .into_iter()
        .map(evaluate_appearance_conformance_row)
        .collect();
    let from_inputs = AppearanceConformancePacket::from_rows(
        "extension-appearance-conformance:packet:beta:default",
        "2026-05-20T00:00:00Z",
        rows,
    );
    let checked: AppearanceConformancePacket = load_json("conformance_packet.json");
    assert_eq!(from_inputs, checked);
}

#[test]
fn validator_refuses_overclaimed_inheritance() {
    let mut input = seeded_appearance_conformance_inputs()
        .into_iter()
        .next()
        .expect("seeded inputs are non-empty");
    axis_proof(
        &mut input,
        AppearanceAxisClass::Density,
        AppearanceProofClass::ProvenUnsupported,
    );
    let row = evaluate_appearance_conformance_row(input);
    assert_eq!(
        row.decision_class,
        AppearanceConformanceDecisionClass::Refused
    );
    assert_eq!(
        row.reason_class,
        AppearanceConformanceReasonClass::OverclaimedInheritanceRefused
    );
    assert!(row.row_defect_kind_tokens.contains(
        &AppearanceConformanceDefectKind::OverclaimedInheritance
            .as_str()
            .to_owned()
    ));
}

#[test]
fn validator_flags_undisclosed_axis() {
    let mut input = seeded_appearance_conformance_inputs()
        .into_iter()
        .next()
        .expect("seeded inputs are non-empty");
    for axis in &mut input.declaration.declared_axes {
        if axis.axis == AppearanceAxisClass::HostToken {
            axis.declared_class = ExtensionInheritanceClass::NotDisclosed;
        }
    }
    let row = evaluate_appearance_conformance_row(input);
    assert_eq!(
        row.decision_class,
        AppearanceConformanceDecisionClass::NeedsReview
    );
    assert_eq!(
        row.reason_class,
        AppearanceConformanceReasonClass::DisclosureIncomplete
    );
    assert!(row.row_defect_kind_tokens.contains(
        &AppearanceConformanceDefectKind::AxisDisclosureMissing
            .as_str()
            .to_owned()
    ));
}

#[test]
fn validator_refuses_when_host_stable_labels_are_hidden() {
    let mut input = seeded_appearance_conformance_inputs()
        .into_iter()
        .next()
        .expect("seeded inputs are non-empty");
    input.host_rendered_trust_and_severity = false;
    let row = evaluate_appearance_conformance_row(input);
    assert_eq!(
        row.decision_class,
        AppearanceConformanceDecisionClass::Refused
    );
    assert_eq!(
        row.reason_class,
        AppearanceConformanceReasonClass::HostStableLabelHiddenRefused
    );
    assert!(row.row_defect_kind_tokens.contains(
        &AppearanceConformanceDefectKind::HostStableLabelHidden
            .as_str()
            .to_owned()
    ));
}

#[test]
fn validator_flags_support_export_parity_drift() {
    let packet = seeded_appearance_conformance_packet();
    let mut support_rows = packet
        .rows
        .iter()
        .map(project_appearance_conformance_support_row)
        .collect::<Vec<_>>();
    support_rows[0].publisher_label = "Different publisher".to_owned();
    let defects = audit_appearance_conformance_rows(&packet.rows, &support_rows);
    assert!(defects
        .iter()
        .any(|defect| defect.defect_kind
            == AppearanceConformanceDefectKind::SupportExportParityDrift));
}

#[test]
fn validator_flags_unsupported_state_on_fully_inherited_axis() {
    let mut input = seeded_appearance_conformance_inputs()
        .into_iter()
        .next()
        .expect("seeded inputs are non-empty");
    input
        .declaration
        .known_unsupported_states
        .push(super::AppearanceUnsupportedState {
            axis: AppearanceAxisClass::Theme,
            state_label: "high_contrast_theme".to_owned(),
            summary: "Theme is actually fully inherited; this state is stale.".to_owned(),
        });
    let row = evaluate_appearance_conformance_row(input);
    assert!(row.row_defect_kind_tokens.contains(
        &AppearanceConformanceDefectKind::UnsupportedStateInconsistent
            .as_str()
            .to_owned()
    ));
}

#[test]
fn support_export_validation_blocks_raw_private_material() {
    let packet = seeded_appearance_conformance_packet();
    let mut export = project_appearance_conformance_support_export(
        &packet,
        "extension-appearance-conformance:support-export:default",
        "2026-05-20T00:00:00Z",
    );
    export.raw_private_material_excluded = false;
    let defects = validate_appearance_conformance_support_export(&packet, &export)
        .expect_err("raw private material export must fail");
    assert!(defects
        .iter()
        .any(|defect| defect.defect_kind
            == AppearanceConformanceDefectKind::RawPrivateMaterialExported));
}
