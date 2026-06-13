use super::*;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_records_policy_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn seeded_packet_covers_every_family() {
    let packet = seeded_m5_records_policy_packet();
    for family in GovernedArtifactFamily::ALL {
        assert!(
            packet.rows.iter().any(|row| row.artifact_family == family),
            "missing family: {family:?}"
        );
    }
}

#[test]
fn active_and_indeterminate_holds_block_delete() {
    let packet = seeded_m5_records_policy_packet();
    for row in &packet.rows {
        if row.legal_hold_notice.status_requires_block() {
            assert!(row.legal_hold_notice.blocks_destructive_action, "{row:?}");
            assert_eq!(
                row.pre_delete_truth.projected_outcome,
                RecordOperationOutcome::BlockedByHold,
                "{row:?}"
            );
        }
    }
}

#[test]
fn local_only_families_never_claim_managed_control() {
    let packet = seeded_m5_records_policy_packet();
    for row in &packet.rows {
        if matches!(row.authority_boundary, AuthorityBoundaryClass::LocalOnly) {
            assert!(!row.claims_managed_hold, "{row:?}");
            assert!(!row.claims_managed_export, "{row:?}");
            assert!(!row.claims_managed_delete, "{row:?}");
        }
    }
}

#[test]
fn outside_scope_flag_matches_outcome() {
    let packet = seeded_m5_records_policy_packet();
    for row in &packet.rows {
        for truth in [&row.pre_delete_truth, &row.pre_export_truth] {
            let outcome_is_outside =
                truth.projected_outcome == RecordOperationOutcome::OutsidePlatformScope;
            assert_eq!(
                truth.outside_platform_scope, outcome_is_outside,
                "{truth:?}"
            );
        }
    }
}

#[test]
fn projections_cover_every_row() {
    let packet = seeded_m5_records_policy_packet();
    assert_eq!(packet.product_projection().len(), packet.rows.len());
    assert_eq!(packet.cli_headless_projection().len(), packet.rows.len());
    assert_eq!(packet.support_export_projection().len(), packet.rows.len());
}

#[test]
fn projections_share_one_vocabulary() {
    let packet = seeded_m5_records_policy_packet();
    let product = packet.product_projection();
    let cli = packet.cli_headless_projection();
    let support = packet.support_export_projection();
    for ((p, c), s) in product.iter().zip(cli.iter()).zip(support.iter()) {
        assert_eq!(p.pre_delete_outcome, c.pre_delete_outcome);
        assert_eq!(p.pre_delete_outcome, s.pre_delete_outcome);
        assert_eq!(p.pre_export_outcome, c.pre_export_outcome);
        assert_eq!(p.pre_export_outcome, s.pre_export_outcome);
        assert_eq!(p.hold_status, c.hold_status);
        assert_eq!(p.hold_status, s.hold_status);
    }
}

#[test]
fn local_only_managed_hold_claim_is_rejected() {
    let mut packet = seeded_m5_records_policy_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| matches!(row.authority_boundary, AuthorityBoundaryClass::LocalOnly))
        .expect("a local-only row exists");
    row.claims_managed_hold = true;

    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5RecordsPolicyViolation::LocalOnlyClaimsManagedHold { .. }
    )));
}

#[test]
fn missing_hold_fail_closed_is_rejected() {
    let mut packet = seeded_m5_records_policy_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.legal_hold_notice.hold_status == HoldStatus::Active)
        .expect("an active-hold row exists");
    row.pre_delete_truth.projected_outcome = RecordOperationOutcome::Completed;

    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5RecordsPolicyViolation::HoldNotFailClosed { .. }
    )));
}

#[test]
fn checked_in_canonical_fixture_matches_seeded_packet() {
    let fixture =
        repo_root().join("fixtures/governance/m5_records_policy_sim/canonical_packet.yaml");
    let raw = std::fs::read_to_string(&fixture).expect("canonical fixture is readable");
    let parsed: M5RecordsPolicyPacket =
        serde_yaml::from_str(&raw).expect("canonical fixture parses");

    assert!(
        parsed.validate().is_empty(),
        "canonical fixture must validate cleanly: {:?}",
        parsed.validate()
    );
    assert_eq!(
        parsed,
        seeded_m5_records_policy_packet(),
        "canonical fixture drifted from the seeded packet; regenerate it"
    );
}
