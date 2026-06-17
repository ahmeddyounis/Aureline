//! Protected fixture checks for the native-desktop qualification family.
//!
//! The integration test replays every JSON fixture under
//! `fixtures/platform/m5-native-desktop-qualification/` through the Rust types
//! and asserts the contract invariants. The report, support-export, and
//! claim-packet fixtures are asserted bit-for-bit equal to the records minted by
//! `seeded_native_desktop_qualification`, and the markdown artifacts under
//! `artifacts/platform/m5-native-desktop-qualification/` and
//! `artifacts/shiproom/m5-native-desktop-claim-packet/` are asserted bit-for-bit
//! equal to the rendering, so the headless inspector remains the only
//! mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::m5_native_desktop_qualification::{
    seeded_native_desktop_qualification, seeded_qualification_claim_packet,
    seeded_qualification_support_export, validate_qualification_report, ClaimState,
    NativeDesktopClaimPacket, NativeDesktopQualificationReport,
    NativeDesktopQualificationSupportExport, QualificationDimension, QualificationStatus,
    QUALIFICATION_REPORT_RECORD_KIND, QUALIFICATION_SHARED_CONTRACT_REF,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/platform/m5-native-desktop-qualification")
}

fn load_json<T: serde::de::DeserializeOwned>(file: &str) -> T {
    let path = fixtures_root().join(file);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn fixture_report_is_bit_for_bit_equal_to_seed() {
    let on_disk: NativeDesktopQualificationReport = load_json("report.json");
    let seeded = seeded_native_desktop_qualification();
    assert_eq!(
        on_disk, seeded,
        "fixture report diverged from seeded report"
    );
    assert_eq!(seeded.record_kind, QUALIFICATION_REPORT_RECORD_KIND);
    assert_eq!(
        seeded.shared_contract_ref,
        QUALIFICATION_SHARED_CONTRACT_REF
    );
}

#[test]
fn fixture_report_passes_validation() {
    let report: NativeDesktopQualificationReport = load_json("report.json");
    validate_qualification_report(&report).expect("fixture report must validate");
    assert!(report.report_clean);
}

#[test]
fn fixture_report_qualifies_every_dimension() {
    let report: NativeDesktopQualificationReport = load_json("report.json");
    assert!(report.every_dimension_qualified());
    for dimension in QualificationDimension::required_dimensions() {
        let any_qualified = report.profiles.iter().any(|profile| {
            profile.bindings.iter().any(|binding| {
                binding.dimension == dimension && binding.status == QualificationStatus::Qualified
            })
        });
        assert!(
            any_qualified,
            "no qualified profile for required dimension {}",
            dimension.as_str()
        );
    }
    for profile in &report.profiles {
        assert_eq!(
            profile.bindings.len(),
            QualificationDimension::required_dimensions().len(),
            "{} must bind every dimension",
            profile.descriptor.profile_id
        );
    }
}

#[test]
fn fixture_claim_state_is_never_greener_than_proof() {
    let report: NativeDesktopQualificationReport = load_json("report.json");
    for profile in &report.profiles {
        let qualified = profile
            .bindings
            .iter()
            .filter(|binding| binding.status == QualificationStatus::Qualified)
            .count();
        // A published claim requires every dimension qualified-fresh.
        if profile.claim_state == ClaimState::Published {
            assert_eq!(
                qualified,
                profile.bindings.len(),
                "{} is published but not fully qualified",
                profile.descriptor.profile_id
            );
        }
        if qualified == 0 {
            assert_eq!(
                profile.claim_state,
                ClaimState::Withheld,
                "{} qualifies nothing but is not withheld",
                profile.descriptor.profile_id
            );
        }
    }
}

#[test]
fn fixture_support_export_quotes_report_and_case_ids() {
    let report: NativeDesktopQualificationReport = load_json("report.json");
    let export: NativeDesktopQualificationSupportExport = load_json("support_export.json");
    let expected = NativeDesktopQualificationSupportExport::from_report(
        export.support_export_id.clone(),
        report.clone(),
    );
    assert_eq!(export, expected);
    assert_eq!(export, seeded_qualification_support_export());
    assert!(export.case_ids.contains(&report.report_id));
    for profile in &report.profiles {
        assert!(
            export.case_ids.contains(&profile.descriptor.profile_id),
            "support export must quote profile id {}",
            profile.descriptor.profile_id
        );
        assert!(
            export
                .case_ids
                .contains(&profile.descriptor.descriptor_revision_ref),
            "support export must quote descriptor revision {}",
            profile.descriptor.descriptor_revision_ref
        );
    }
}

#[test]
fn fixture_claim_packet_matches_seed_and_partitions_profiles() {
    let report: NativeDesktopQualificationReport = load_json("report.json");
    let packet: NativeDesktopClaimPacket = load_json("claim_packet.json");
    let expected =
        NativeDesktopClaimPacket::from_report(packet.claim_packet_id.clone(), report.clone());
    assert_eq!(packet, expected);
    assert_eq!(packet, seeded_qualification_claim_packet());
    // Every profile lands in exactly one partition.
    let total = packet.publishable_profiles.len()
        + packet.narrowed_profiles.len()
        + packet.withheld_profiles.len();
    assert_eq!(total, report.profiles.len());
}

#[test]
fn published_qualification_md_matches_seeded_rendering() {
    let report = seeded_native_desktop_qualification();
    let rendered = report.render_markdown();
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../artifacts/platform/m5-native-desktop-qualification/m5_native_desktop_qualification.md",
    );
    let on_disk = std::fs::read_to_string(&path)
        .expect("published m5_native_desktop_qualification.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published m5_native_desktop_qualification.md diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop_qualification -- report-md`",
    );
}

#[test]
fn published_claim_packet_md_matches_seeded_rendering() {
    let packet = seeded_qualification_claim_packet();
    let rendered = packet.render_markdown();
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../artifacts/shiproom/m5-native-desktop-claim-packet/m5_native_desktop_claim_packet.md",
    );
    let on_disk = std::fs::read_to_string(&path)
        .expect("published m5_native_desktop_claim_packet.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published m5_native_desktop_claim_packet.md diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop_qualification -- claim-packet-md`",
    );
}

#[test]
fn published_doc_links_every_dimension_drill_and_artifact() {
    let doc_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/m5/native-desktop-qualification.md");
    let body = std::fs::read_to_string(&doc_path)
        .expect("published native-desktop-qualification doc must exist");
    for dimension in QualificationDimension::required_dimensions() {
        assert!(
            body.contains(dimension.as_str()),
            "doc must quote required dimension {}",
            dimension.as_str()
        );
        assert!(
            body.contains(dimension.required_drill().as_str()),
            "doc must quote required drill {}",
            dimension.required_drill().as_str()
        );
    }
    assert!(body.contains(
        "artifacts/platform/m5-native-desktop-qualification/m5_native_desktop_qualification.md"
    ));
    assert!(body.contains("fixtures/platform/m5-native-desktop-qualification/report.json"));
    assert!(body.contains("schemas/platform/m5-native-desktop-qualification.schema.json"));
    assert!(body.contains("tools/ci/m5/native_desktop_qualification_check.py"));
}

#[test]
fn fixture_compact_lines_match_seed() {
    let compact_path = fixtures_root().join("compact.txt");
    let on_disk = std::fs::read_to_string(&compact_path).expect("compact fixture must exist");
    let seeded = seeded_native_desktop_qualification();
    let mut rendered = seeded.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop_qualification -- compact`",
    );
}
