//! Fixture-driven coverage for the stable Spring Boot Expert workflow
//! pack truth packet covering create, open, run, test, debug, rename,
//! and review loops plus the Spring Boot target discovery surface
//! (`./gradlew bootRun`, `./gradlew bootBuildImage`,
//! `./gradlew bootJar`, `./gradlew bootWar`, `mvn spring-boot:run`,
//! `mvn spring-boot:build-image`, `mvn spring-boot:repackage`, custom
//! Gradle/Maven targets, JUnit 5 / JUnit 4 / Spock / Kotest test
//! tasks, and JDWP / JPDA debug targets), the Spring Boot
//! config/property awareness surface (`application.properties`,
//! `application.yml`, profile-specific config,
//! `@ConfigurationProperties`, `@Value`, `@ConditionalOnProperty`,
//! `@Profile`, `spring.config.import`, `spring.profiles.active`, and
//! auto-configuration imports resolution), and the Spring Boot
//! review-safe refactor surface (DI/IoC-aware rename, `@Component`/
//! `@Service`/`@Repository`/`@Controller`/`@RestController`/
//! `@Configuration` bean lifecycle safety, `@Autowired`/
//! constructor-injection/`@Qualifier` resolution, AOP advice targets,
//! and bean rename across `@Bean(name)` references) with known limits,
//! downgrade automation, and evidence binding.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_language::{
    current_stable_spring_boot_expert_workflow_pack_truth_packet,
    SpringBootExpertWorkflowLoopClass, SpringBootExpertWorkflowPackClass,
    SpringBootExpertWorkflowPackConsumerSurface,
    SpringBootExpertWorkflowPackDowngradeAutomationClass,
    SpringBootExpertWorkflowPackEvidenceClass, SpringBootExpertWorkflowPackFindingKind,
    SpringBootExpertWorkflowPackKnownLimitClass, SpringBootExpertWorkflowPackPromotionState,
    SpringBootExpertWorkflowPackRowClass, SpringBootExpertWorkflowPackSupportClass,
    SpringBootExpertWorkflowPackTruthPacket, SpringBootExpertWorkflowPackTruthPacketInput,
    SPRING_BOOT_EXPERT_WORKFLOW_PACK_TRUTH_ARTIFACT_DOC_REF,
    SPRING_BOOT_EXPERT_WORKFLOW_PACK_TRUTH_DOC_REF,
    SPRING_BOOT_EXPERT_WORKFLOW_PACK_TRUTH_FIXTURE_DIR,
    SPRING_BOOT_EXPERT_WORKFLOW_PACK_TRUTH_PACKET_ARTIFACT_REF,
    SPRING_BOOT_EXPERT_WORKFLOW_PACK_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SpringBootExpertWorkflowPackFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: SpringBootExpertWorkflowPackTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    pack_tokens: Vec<String>,
    row_class_tokens: Vec<String>,
    support_class_tokens: Vec<String>,
    workflow_loop_tokens: Vec<String>,
    known_limit_tokens: Vec<String>,
    downgrade_automation_tokens: Vec<String>,
    evidence_class_tokens: Vec<String>,
    support_export_safe: bool,
    #[serde(default)]
    expected_finding_kinds: Vec<String>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root canonicalizes")
}

fn assert_exists(rel: &str) {
    let path = repo_root().join(rel);
    assert!(
        path.exists(),
        "expected path to exist on disk: {} ({})",
        rel,
        path.display()
    );
}

fn load_fixture(file_name: &str) -> SpringBootExpertWorkflowPackFixture {
    let path = repo_root()
        .join(SPRING_BOOT_EXPERT_WORKFLOW_PACK_TRUTH_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_token_set_matches(observed: &[&str], expected: &[String], label: &str) {
    let observed: BTreeSet<&str> = observed.iter().copied().collect();
    let expected: BTreeSet<&str> = expected.iter().map(String::as_str).collect();
    assert_eq!(
        observed, expected,
        "{label} token set drift: observed={observed:?}, expected={expected:?}"
    );
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "spring_boot_expert_workflow_pack_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = SpringBootExpertWorkflowPackTruthPacket::materialize(fixture.input.clone());
    assert_eq!(
        packet.promotion_state.as_str(),
        expect.promotion_state,
        "fixture {} expected promotion {}, got {:?}",
        fixture.case_name,
        expect.promotion_state,
        packet.promotion_state
    );
    assert_eq!(
        packet.rows.len(),
        expect.row_count,
        "fixture {} row count drift",
        fixture.case_name
    );
    assert_eq!(
        packet.validation_findings.len(),
        expect.validation_finding_count,
        "fixture {} finding count drift; got {:?}",
        fixture.case_name,
        packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    assert_token_set_matches(&packet.pack_tokens(), &expect.pack_tokens, "pack");
    assert_token_set_matches(
        &packet.row_class_tokens(),
        &expect.row_class_tokens,
        "row_class",
    );
    assert_token_set_matches(
        &packet.support_class_tokens(),
        &expect.support_class_tokens,
        "support_class",
    );
    assert_token_set_matches(
        &packet.workflow_loop_tokens(),
        &expect.workflow_loop_tokens,
        "workflow_loop",
    );
    assert_token_set_matches(
        &packet.known_limit_tokens(),
        &expect.known_limit_tokens,
        "known_limit",
    );
    assert_token_set_matches(
        &packet.downgrade_automation_tokens(),
        &expect.downgrade_automation_tokens,
        "downgrade_automation",
    );
    assert_token_set_matches(
        &packet.evidence_class_tokens(),
        &expect.evidence_class_tokens,
        "evidence_class",
    );

    let export = packet.support_export(
        format!("support-export:{}", fixture.case_name),
        "2026-05-26T12:00:10Z",
    );
    assert_eq!(
        export.is_export_safe(),
        expect.support_export_safe,
        "fixture {} support-export safety drift",
        fixture.case_name
    );

    if !expect.expected_finding_kinds.is_empty() {
        let observed: BTreeSet<&str> = packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect();
        for kind in &expect.expected_finding_kinds {
            assert!(
                observed.contains(kind.as_str()),
                "fixture {} expected finding kind {kind}; observed {:?}",
                fixture.case_name,
                observed
            );
        }
    }
}

#[test]
fn schema_doc_fixture_and_artifact_exist_on_disk() {
    assert_exists(SPRING_BOOT_EXPERT_WORKFLOW_PACK_TRUTH_SCHEMA_REF);
    assert_exists(SPRING_BOOT_EXPERT_WORKFLOW_PACK_TRUTH_DOC_REF);
    assert_exists(SPRING_BOOT_EXPERT_WORKFLOW_PACK_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(SPRING_BOOT_EXPERT_WORKFLOW_PACK_TRUTH_FIXTURE_DIR);
    assert_exists(SPRING_BOOT_EXPERT_WORKFLOW_PACK_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn expert_grade_with_unbound_evidence_blocks_stable() {
    assert_fixture_matches("expert_grade_with_unbound_evidence_blocks_stable.json");
}

#[test]
fn missing_workflow_loop_for_expert_grade_blocks_stable() {
    assert_fixture_matches("missing_workflow_loop_for_expert_grade_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_workflow_loop_vocabulary_blocks_stable() {
    assert_fixture_matches("projection_collapses_workflow_loop_vocabulary_blocks_stable.json");
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_pack() {
    let packet = current_stable_spring_boot_expert_workflow_pack_truth_packet()
        .expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        SpringBootExpertWorkflowPackPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in SpringBootExpertWorkflowPackClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.pack_class == required),
            "stable packet must include row for workflow pack {}",
            required.as_str()
        );
    }
    for surface in SpringBootExpertWorkflowPackConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_migration_target_discovery_config_and_review_safe_refactor() {
    let packet = current_stable_spring_boot_expert_workflow_pack_truth_packet()
        .expect("checked-in packet validates");
    assert!(
        packet.rows.iter().any(|row| row.row_class
            == SpringBootExpertWorkflowPackRowClass::FrameworkMigrationRow),
        "stable packet must include a framework_migration_row binding the Spring Boot 2.x → 3.x migration"
    );
    assert!(
        packet.rows.iter().any(|row| row.row_class
            == SpringBootExpertWorkflowPackRowClass::ArchetypeRepoRow),
        "stable packet must include an archetype_repo_row"
    );
    assert!(
        packet.rows.iter().any(|row| row.row_class
            == SpringBootExpertWorkflowPackRowClass::TargetDiscoveryRow),
        "stable packet must include a target_discovery_row"
    );
    assert!(
        packet.rows.iter().any(|row| row.row_class
            == SpringBootExpertWorkflowPackRowClass::ConfigPropertyAwarenessRow),
        "stable packet must include a config_property_awareness_row"
    );
    assert!(
        packet.rows.iter().any(|row| row.row_class
            == SpringBootExpertWorkflowPackRowClass::ReviewSafeRefactorRow),
        "stable packet must include a review_safe_refactor_row"
    );
}

#[test]
fn closed_spring_boot_expert_workflow_pack_tokens_are_pinned() {
    assert_eq!(
        SpringBootExpertWorkflowPackClass::SpringBootExpertWorkflowPack.as_str(),
        "spring_boot_expert_workflow_pack"
    );
    assert_eq!(
        SpringBootExpertWorkflowPackRowClass::TargetDiscoveryRow.as_str(),
        "target_discovery_row"
    );
    assert_eq!(
        SpringBootExpertWorkflowPackRowClass::ConfigPropertyAwarenessRow.as_str(),
        "config_property_awareness_row"
    );
    assert_eq!(
        SpringBootExpertWorkflowPackRowClass::ReviewSafeRefactorRow.as_str(),
        "review_safe_refactor_row"
    );
    assert_eq!(
        SpringBootExpertWorkflowPackRowClass::DowngradeAutomation.as_str(),
        "downgrade_automation"
    );
    assert_eq!(
        SpringBootExpertWorkflowPackSupportClass::SupportUnbound.as_str(),
        "support_unbound"
    );
    assert_eq!(SpringBootExpertWorkflowLoopClass::Review.as_str(), "review");
    assert_eq!(
        SpringBootExpertWorkflowPackEvidenceClass::TargetDiscoveryEvidence.as_str(),
        "target_discovery_evidence"
    );
    assert_eq!(
        SpringBootExpertWorkflowPackEvidenceClass::ConfigPropertyAwarenessEvidence.as_str(),
        "config_property_awareness_evidence"
    );
    assert_eq!(
        SpringBootExpertWorkflowPackEvidenceClass::ReviewSafeRefactorEvidence.as_str(),
        "review_safe_refactor_evidence"
    );
    assert_eq!(
        SpringBootExpertWorkflowPackEvidenceClass::EvidenceUnbound.as_str(),
        "evidence_unbound"
    );
    assert_eq!(
        SpringBootExpertWorkflowPackKnownLimitClass::TargetDiscoverySubsetOnly.as_str(),
        "target_discovery_subset_only"
    );
    assert_eq!(
        SpringBootExpertWorkflowPackKnownLimitClass::ConfigPropertySubsetOnly.as_str(),
        "config_property_subset_only"
    );
    assert_eq!(
        SpringBootExpertWorkflowPackKnownLimitClass::ReviewSafeRefactorSubsetOnly.as_str(),
        "review_safe_refactor_subset_only"
    );
    assert_eq!(
        SpringBootExpertWorkflowPackKnownLimitClass::LimitUnbound.as_str(),
        "limit_unbound"
    );
    assert_eq!(
        SpringBootExpertWorkflowPackDowngradeAutomationClass::AutoNarrowOnUnprovenTargetDiscovery
            .as_str(),
        "auto_narrow_on_unproven_target_discovery"
    );
    assert_eq!(
        SpringBootExpertWorkflowPackDowngradeAutomationClass::AutoNarrowOnUnprovenConfigPropertyAwareness
            .as_str(),
        "auto_narrow_on_unproven_config_property_awareness"
    );
    assert_eq!(
        SpringBootExpertWorkflowPackDowngradeAutomationClass::AutoNarrowOnUnprovenReviewSafeRefactor
            .as_str(),
        "auto_narrow_on_unproven_review_safe_refactor"
    );
    assert_eq!(
        SpringBootExpertWorkflowPackDowngradeAutomationClass::AutomationUnbound.as_str(),
        "automation_unbound"
    );
    assert_eq!(
        SpringBootExpertWorkflowPackConsumerSurface::ConformanceDashboard.as_str(),
        "conformance_dashboard"
    );
    assert_eq!(
        SpringBootExpertWorkflowPackFindingKind::WorkflowLoopVocabularyCollapsed.as_str(),
        "workflow_loop_vocabulary_collapsed"
    );
}
