use super::*;

const PACKET_ID: &str = "generator-run:stable:0001";
const PACKET_LABEL: &str =
    "Framework Generators and Codemods with Preview, Diff, Rollback, and Execution-Context Reuse";

const SCAFFOLD_EXACT: &str = "generator-run-row:scaffold.resource.exact.reused:2026.06";
const CODEMOD_EXACT: &str = "generator-run-row:codemod.add_field.exact.fresh:2026.06";
const MIGRATION_HEURISTIC: &str = "generator-run-row:migration.heuristic.reuse_unavailable:2026.05";
const REFACTOR_ROLLBACK_UNAVAIL: &str =
    "generator-run-row:refactor.rollback_unavailable.blocked:2026.06";
const CONFIG_PREVIEW_UNAVAIL: &str = "generator-run-row:config.preview_unavailable.blocked:2026.04";
const BRIDGE_ROLLED_BACK: &str = "generator-run-row:codemod.bridge.rolled_back:2026.06";

fn proof_freshness() -> GeneratorRunProofFreshness {
    GeneratorRunProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> GeneratorRunPacket {
    canonical_generator_runs(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        "2026-06-08T00:00:00Z".to_owned(),
        proof_freshness(),
    )
}

fn row<'a>(packet: &'a GeneratorRunPacket, row_id: &str) -> &'a GeneratorRunRow {
    packet
        .rows
        .iter()
        .find(|row| row.row_id == row_id)
        .unwrap_or_else(|| panic!("missing row {row_id}"))
}

#[test]
fn generator_run_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_rows_cover_preview_spectrum() {
    let packet = packet();
    let preview: Vec<PreviewClass> = packet.rows.iter().map(|row| row.preview_class).collect();
    for required in [
        PreviewClass::PreviewAvailable,
        PreviewClass::PreviewPartial,
        PreviewClass::PreviewUnavailable,
    ] {
        assert!(
            preview.contains(&required),
            "missing preview {}",
            required.as_str()
        );
    }
}

#[test]
fn canonical_rows_cover_rollback_and_context_reuse_spectrum() {
    let packet = packet();
    let rollback: Vec<RollbackClass> = packet.rows.iter().map(|row| row.rollback_class).collect();
    assert!(rollback.contains(&RollbackClass::RollbackAvailable));
    assert!(rollback.contains(&RollbackClass::RollbackPartial));
    assert!(rollback.contains(&RollbackClass::RollbackUnavailable));
    assert!(rollback.contains(&RollbackClass::RolledBack));

    let reuse: Vec<ExecutionContextReuseClass> = packet
        .rows
        .iter()
        .map(|row| row.context_reuse_class)
        .collect();
    assert!(reuse.contains(&ExecutionContextReuseClass::ContextReused));
    assert!(reuse.contains(&ExecutionContextReuseClass::ContextFresh));
    assert!(reuse.contains(&ExecutionContextReuseClass::ContextReuseUnavailable));
    assert!(reuse.contains(&ExecutionContextReuseClass::ContextReuseUnknown));
}

#[test]
fn exact_scaffold_is_grounded_and_active() {
    let packet = packet();
    let scaffold = row(&packet, SCAFFOLD_EXACT);
    assert_eq!(scaffold.preview_class, PreviewClass::PreviewAvailable);
    assert_eq!(scaffold.diff_review_class, DiffReviewClass::DiffReviewed);
    assert_eq!(scaffold.rollback_class, RollbackClass::RollbackAvailable);
    assert!(!scaffold.rollback_handle_refs.is_empty());
    assert_eq!(
        scaffold.context_reuse_class,
        ExecutionContextReuseClass::ContextReused
    );
    assert!(scaffold.admitted_for_display);
    assert!(!scaffold.is_blocked());
}

#[test]
fn exact_codemod_runs_on_fresh_context_and_is_active() {
    let packet = packet();
    let codemod = row(&packet, CODEMOD_EXACT);
    assert_eq!(
        codemod.context_reuse_class,
        ExecutionContextReuseClass::ContextFresh
    );
    assert_eq!(codemod.rollback_class, RollbackClass::RollbackAvailable);
    assert!(codemod.admitted_for_display);
}

#[test]
fn heuristic_migration_discloses_banner_and_is_held() {
    let packet = packet();
    let migration = row(&packet, MIGRATION_HEURISTIC);
    assert!(migration.support_class.requires_disclosure());
    assert!(!migration.known_issue_refs.is_empty());
    assert!(migration.downgrade_banner_class.is_present());
    assert_eq!(
        migration.context_reuse_class,
        ExecutionContextReuseClass::ContextReuseUnavailable
    );
    assert!(migration
        .downgrade_triggers
        .contains(&GeneratorDowngradeTrigger::HeuristicMappingDisclosed));
    assert!(migration
        .downgrade_triggers
        .contains(&GeneratorDowngradeTrigger::ContextReuseUnavailable));
    assert!(!migration.admitted_for_display);
}

#[test]
fn rollback_unavailable_run_is_blocked() {
    let packet = packet();
    let blocked = row(&packet, REFACTOR_ROLLBACK_UNAVAIL);
    assert!(blocked.rollback_class.is_unavailable());
    assert_eq!(
        blocked.downgrade_banner_class,
        GeneratorDowngradeBannerClass::RollbackUnavailableBanner
    );
    assert!(blocked.rollback_handle_refs.is_empty());
    assert!(blocked.is_blocked());
    assert!(!blocked.admitted_for_display);
}

#[test]
fn preview_unavailable_run_is_blocked() {
    let packet = packet();
    let blocked = row(&packet, CONFIG_PREVIEW_UNAVAIL);
    assert!(blocked.preview_class.is_unavailable());
    assert!(blocked.diff_review_class.is_unavailable());
    assert_eq!(
        blocked.downgrade_banner_class,
        GeneratorDowngradeBannerClass::PreviewUnavailableBanner
    );
    assert!(blocked.is_blocked());
    assert!(!blocked.admitted_for_display);
}

#[test]
fn bridged_codemod_discloses_known_issue_and_is_held() {
    let packet = packet();
    let bridge = row(&packet, BRIDGE_ROLLED_BACK);
    assert_eq!(bridge.support_class, GeneratorSupportClass::BridgeBehavior);
    assert_eq!(bridge.rollback_class, RollbackClass::RolledBack);
    assert!(!bridge.known_issue_refs.is_empty());
    assert!(bridge
        .downgrade_triggers
        .contains(&GeneratorDowngradeTrigger::BridgeBehaviorDisclosed));
    assert!(!bridge.admitted_for_display);
}

#[test]
fn rows_empty_fails_validation() {
    let mut packet = packet();
    packet.rows.clear();
    assert!(packet
        .validate()
        .contains(&GeneratorRunViolation::RowsEmpty));
}

#[test]
fn preview_unavailable_without_banner_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == CONFIG_PREVIEW_UNAVAIL)
        .unwrap()
        .downgrade_banner_class = GeneratorDowngradeBannerClass::FreshnessBanner;
    assert!(packet
        .validate()
        .contains(&GeneratorRunViolation::PreviewUnavailableBannerMissing));
}

#[test]
fn rollback_unavailable_without_banner_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == REFACTOR_ROLLBACK_UNAVAIL)
        .unwrap()
        .downgrade_banner_class = GeneratorDowngradeBannerClass::SupportClassBanner;
    assert!(packet
        .validate()
        .contains(&GeneratorRunViolation::RollbackUnavailableBannerMissing));
}

#[test]
fn captured_rollback_without_handle_refs_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == CODEMOD_EXACT)
        .unwrap()
        .rollback_handle_refs
        .clear();
    assert!(packet
        .validate()
        .contains(&GeneratorRunViolation::RollbackHandleRefsMissing));
}

#[test]
fn bridge_run_without_disclosure_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == BRIDGE_ROLLED_BACK)
        .unwrap()
        .known_issue_refs
        .clear();
    assert!(packet
        .validate()
        .contains(&GeneratorRunViolation::SupportClassUndisclosed));
}

#[test]
fn context_reuse_unavailable_without_banner_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == MIGRATION_HEURISTIC)
        .unwrap()
        .downgrade_banner_class = GeneratorDowngradeBannerClass::NoBanner;
    assert!(packet
        .validate()
        .contains(&GeneratorRunViolation::ContextReuseBannerMissing));
}

#[test]
fn blocked_run_admitted_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == REFACTOR_ROLLBACK_UNAVAIL)
        .unwrap()
        .admitted_for_display = true;
    assert!(packet
        .validate()
        .contains(&GeneratorRunViolation::BlockedDisplayAdmitted));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.rows[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&GeneratorRunViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.rows[0].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&GeneratorRunViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&GeneratorRunViolation::MissingSourceContracts));
}

#[test]
fn review_incomplete_fails() {
    let mut packet = packet();
    packet.review.rollback_unavailable_blocks_confident_apply = false;
    assert!(packet
        .validate()
        .contains(&GeneratorRunViolation::ReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.blocked_runs_labeled_not_hidden = false;
    assert!(packet
        .validate()
        .contains(&GeneratorRunViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&GeneratorRunViolation::ProofFreshnessIncomplete));
}

#[test]
fn unavailable_preview_blocks_a_run() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[GeneratorRunRowObservation {
        row_id: CODEMOD_EXACT.to_owned(),
        preview_available: false,
        diff_available: true,
        rollback_available: true,
        context_reused: true,
        run_fresh: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let codemod = row(&packet, CODEMOD_EXACT);
    assert_eq!(codemod.preview_class, PreviewClass::PreviewUnavailable);
    assert_eq!(codemod.diff_review_class, DiffReviewClass::DiffUnavailable);
    assert_eq!(
        codemod.downgrade_banner_class,
        GeneratorDowngradeBannerClass::PreviewUnavailableBanner
    );
    assert!(!codemod.admitted_for_display);
    assert!(codemod
        .downgrade_triggers
        .contains(&GeneratorDowngradeTrigger::PreviewUnavailable));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn unavailable_rollback_blocks_a_run() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[GeneratorRunRowObservation {
        row_id: SCAFFOLD_EXACT.to_owned(),
        preview_available: true,
        diff_available: true,
        rollback_available: false,
        context_reused: true,
        run_fresh: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let scaffold = row(&packet, SCAFFOLD_EXACT);
    assert_eq!(scaffold.rollback_class, RollbackClass::RollbackUnavailable);
    assert_eq!(
        scaffold.downgrade_banner_class,
        GeneratorDowngradeBannerClass::RollbackUnavailableBanner
    );
    assert!(!scaffold.admitted_for_display);
    assert!(scaffold
        .downgrade_triggers
        .contains(&GeneratorDowngradeTrigger::RollbackUnavailable));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn unavailable_diff_blocks_a_run() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[GeneratorRunRowObservation {
        row_id: CODEMOD_EXACT.to_owned(),
        preview_available: true,
        diff_available: false,
        rollback_available: true,
        context_reused: false,
        run_fresh: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let codemod = row(&packet, CODEMOD_EXACT);
    assert_eq!(codemod.diff_review_class, DiffReviewClass::DiffUnavailable);
    assert_eq!(
        codemod.downgrade_banner_class,
        GeneratorDowngradeBannerClass::DiffUnavailableBanner
    );
    assert!(!codemod.admitted_for_display);
    assert!(codemod
        .downgrade_triggers
        .contains(&GeneratorDowngradeTrigger::DiffUnavailable));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn failed_context_reuse_is_labeled_not_withdrawn() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[GeneratorRunRowObservation {
        row_id: SCAFFOLD_EXACT.to_owned(),
        preview_available: true,
        diff_available: true,
        rollback_available: true,
        context_reused: false,
        run_fresh: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let scaffold = row(&packet, SCAFFOLD_EXACT);
    assert_eq!(
        scaffold.context_reuse_class,
        ExecutionContextReuseClass::ContextReuseUnavailable
    );
    assert_eq!(
        scaffold.downgrade_banner_class,
        GeneratorDowngradeBannerClass::ContextReuseBanner
    );
    assert!(scaffold
        .downgrade_triggers
        .contains(&GeneratorDowngradeTrigger::ContextReuseUnavailable));
    // Falling back to a fresh context is honest, not a block: the run stays offered.
    assert!(scaffold.admitted_for_display);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn stale_run_record_raises_banner_and_withdraws_display() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[GeneratorRunRowObservation {
        row_id: CODEMOD_EXACT.to_owned(),
        preview_available: true,
        diff_available: true,
        rollback_available: true,
        context_reused: true,
        run_fresh: false,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let codemod = row(&packet, CODEMOD_EXACT);
    assert_eq!(codemod.freshness_class, GeneratorFreshnessClass::Stale);
    assert!(codemod.downgrade_banner_class.is_present());
    assert!(!codemod.admitted_for_display);
    assert!(codemod
        .downgrade_triggers
        .contains(&GeneratorDowngradeTrigger::RunRecordStale));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn stale_proof_withholds_display() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[GeneratorRunRowObservation {
        row_id: SCAFFOLD_EXACT.to_owned(),
        preview_available: true,
        diff_available: true,
        rollback_available: true,
        context_reused: true,
        run_fresh: true,
        proof_fresh: false,
        upstream_narrowed: false,
    }]);
    let scaffold = row(&packet, SCAFFOLD_EXACT);
    assert!(!scaffold.admitted_for_display);
    assert!(scaffold
        .downgrade_triggers
        .contains(&GeneratorDowngradeTrigger::ProofStale));
}

#[test]
fn markdown_summary_lists_every_generator() {
    let summary = packet().render_markdown_summary();
    for row in &packet().rows {
        assert!(
            summary.contains(&row.generator_label),
            "summary missing generator {}",
            row.generator_label
        );
    }
    assert!(summary.contains("preview_unavailable_banner"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_generator_run_export().expect("checked generator-run export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked = current_generator_run_export().expect("checked generator-run export validates");
    assert_eq!(checked, packet());
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse/rollback_unavailable_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse/context_reuse_unavailable_labeled.json"
        )),
    ] {
        let packet: GeneratorRunPacket =
            serde_json::from_str(raw).expect("fixture parses as generator-run packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
