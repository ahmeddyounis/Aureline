use std::path::{Path, PathBuf};

use aureline_policy::{
    audit_runtime_authority_issuer_page, seeded_runtime_authority_issuer_page,
    validate_runtime_authority_issuer_page, AuthoritySourceClass, AuthorityTicketClass,
    IssuerBoundaryDecisionClass, IssuerBoundaryRejectionReason, RequestingSurfaceClass,
    RuntimeAuthorityIssuerDefectKind, RuntimeAuthorityIssuerPage, RuntimeAuthorityLineagePacket,
    RuntimeAuthorityLineageRow,
};

#[test]
fn seeded_page_validates_with_zero_defects() {
    let page = seeded_runtime_authority_issuer_page();
    validate_runtime_authority_issuer_page(&page)
        .expect("seeded runtime-authority-issuer page validates");
    assert!(page.defects.is_empty(), "defects: {:?}", page.defects);
}

#[test]
fn seeded_page_covers_every_requesting_surface_class() {
    let page = seeded_runtime_authority_issuer_page();
    for surface_class in RequestingSurfaceClass::ALL {
        assert!(
            page.summary
                .requesting_surface_classes_present
                .iter()
                .any(|token| token == surface_class.as_str()),
            "requesting surface {} is missing",
            surface_class.as_str()
        );
    }
}

#[test]
fn seeded_page_covers_required_rejection_reasons_and_decision_classes() {
    let page = seeded_runtime_authority_issuer_page();
    for reason in [
        IssuerBoundaryRejectionReason::SelfAuthorizationByNonIssuer,
        IssuerBoundaryRejectionReason::AmbientPrivilegeInferred,
        IssuerBoundaryRejectionReason::RememberedDecisionTooBroad,
        IssuerBoundaryRejectionReason::RememberedDecisionTargetDrift,
        IssuerBoundaryRejectionReason::AuthoritySourceMismatch,
        IssuerBoundaryRejectionReason::AuthoritySourceUnreachableTarget,
    ] {
        assert!(
            page.summary
                .rejection_reasons_present
                .iter()
                .any(|token| token == reason.as_str()),
            "rejection reason {} missing",
            reason.as_str()
        );
    }
    for decision_class in [
        IssuerBoundaryDecisionClass::Granted,
        IssuerBoundaryDecisionClass::RememberedDecisionNarrowed,
        IssuerBoundaryDecisionClass::Refused,
    ] {
        assert!(
            page.summary.decisions_by_class.contains_key(decision_class.as_str()),
            "decision class {} missing",
            decision_class.as_str()
        );
    }
}

#[test]
fn lineage_packet_redacts_raw_credentials_and_distinguishes_provider_versus_local() {
    let page = seeded_runtime_authority_issuer_page();
    let packet = RuntimeAuthorityLineagePacket::from_page(
        "runtime-authority-lineage-packet:m3-release",
        "Runtime authority lineage packet (M3 release evidence)",
        "2026-05-18T10:30:00Z",
        &page,
    );
    assert!(packet.raw_credentials_excluded);
    assert!(packet.provider_versus_local_distinguished);
    assert_eq!(packet.lineage_rows.len(), page.decisions.len());
    let has_provider_admit = packet.lineage_rows.iter().any(|row| {
        row.decision_class_token == IssuerBoundaryDecisionClass::Granted.as_str()
            && row.requested_ticket_class_token == "external_provider_mutation"
    });
    assert!(has_provider_admit, "expected provider granted row");
    let has_remembered_narrowed = packet.lineage_rows.iter().any(|row| {
        row.decision_class_token
            == IssuerBoundaryDecisionClass::RememberedDecisionNarrowed.as_str()
    });
    assert!(has_remembered_narrowed, "expected remembered narrowed row");
}

#[test]
fn extension_self_authorization_is_refused_with_structured_reason() {
    let page = seeded_runtime_authority_issuer_page();
    let extension_decision = page
        .decisions
        .iter()
        .find(|decision| decision.request_id == "request:extension:self-authorize:0003")
        .expect("extension self-authorize decision");
    assert_eq!(
        extension_decision.decision_class,
        IssuerBoundaryDecisionClass::Refused
    );
    assert!(extension_decision
        .rejection_reasons
        .contains(&IssuerBoundaryRejectionReason::SelfAuthorizationByNonIssuer));
    assert!(extension_decision.local_editing_preserved);
    assert!(extension_decision.reprompt_required);
}

#[test]
fn local_only_authority_cannot_reach_provider_targets() {
    let page = seeded_runtime_authority_issuer_page();
    let local_only_decision = page
        .decisions
        .iter()
        .find(|decision| decision.request_id == "request:remote-helper:local-only-provider:0006")
        .expect("local-only remote-helper decision");
    assert_eq!(
        local_only_decision.decision_class,
        IssuerBoundaryDecisionClass::Refused
    );
    assert!(local_only_decision
        .rejection_reasons
        .contains(&IssuerBoundaryRejectionReason::AuthoritySourceUnreachableTarget));
    assert_eq!(
        local_only_decision.authority_source_class,
        AuthoritySourceClass::LocalOnlyAuthority
    );
}

#[test]
fn validator_flags_lineage_drift_when_decision_widens_remembered_rule() {
    let mut page = seeded_runtime_authority_issuer_page();
    let decision = page
        .decisions
        .iter_mut()
        .find(|decision| {
            decision.request_id == "request:recipe:broaden-remembered:0005"
        })
        .expect("recipe broaden decision");
    decision.decision_class = IssuerBoundaryDecisionClass::RememberedDecisionNarrowed;
    decision.decision_class_token =
        IssuerBoundaryDecisionClass::RememberedDecisionNarrowed.as_str().to_owned();
    decision.rejection_reasons.clear();
    decision.rejection_reason_tokens.clear();
    decision.renewed_from_rule_id = Some("remembered-rule:local-format:0001".to_owned());
    decision.minted_authority_ticket_ref =
        Some("authority-ticket:local:invalid-broadening:0099".to_owned());
    let defects = audit_runtime_authority_issuer_page(
        &page.issuers,
        &page.requesting_surfaces,
        &page.remembered_rules,
        &page.requests,
        &page.decisions,
    );
    assert!(defects.iter().any(|defect| defect.defect_kind
        == RuntimeAuthorityIssuerDefectKind::DecisionAdmittedOnSourceMismatch));
}

fn issuer_fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/security/m3/runtime_authority_issuer")
}

fn lineage_fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/security/m3/privileged_action_lineage")
}

fn load_page(dir: &Path, file_name: &str) -> RuntimeAuthorityIssuerPage {
    let path = dir.join(file_name);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("fixture {file_name} must parse as a page: {err}"))
}

fn load_lineage_packet(dir: &Path, file_name: &str) -> RuntimeAuthorityLineagePacket {
    let path = dir.join(file_name);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("fixture {file_name} must parse as a lineage packet: {err}"))
}

fn load_lineage_row(dir: &Path, file_name: &str) -> RuntimeAuthorityLineageRow {
    let path = dir.join(file_name);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("fixture {file_name} must parse as a lineage row: {err}"))
}

#[test]
fn fixture_page_matches_seeded_projection() {
    let fixture = load_page(&issuer_fixture_dir(), "page.json");
    let live = seeded_runtime_authority_issuer_page();
    assert_eq!(fixture, live, "checked-in page.json must match the in-crate seed");
}

#[test]
fn fixture_page_validates_with_zero_defects() {
    let page = load_page(&issuer_fixture_dir(), "page.json");
    validate_runtime_authority_issuer_page(&page)
        .expect("fixture page must validate");
    assert!(page.defects.is_empty(), "defects: {:?}", page.defects);
}

#[test]
fn fixture_page_covers_every_high_risk_ticket_class() {
    let page = load_page(&issuer_fixture_dir(), "page.json");
    let observed: std::collections::BTreeSet<&str> = page
        .requests
        .iter()
        .map(|request| request.requested_ticket_class_token.as_str())
        .collect();
    for class in AuthorityTicketClass::ALL {
        assert!(
            observed.contains(class.as_str()),
            "ticket class {} must be present on the conformance page",
            class.as_str()
        );
    }
}

#[test]
fn fixture_lineage_packet_matches_projection() {
    let page = load_page(&issuer_fixture_dir(), "page.json");
    let expected = RuntimeAuthorityLineagePacket::from_page(
        "runtime-authority-lineage-packet:m3:fixture-001",
        "Runtime authority lineage packet (M3 conformance fixture)",
        "2026-05-18T10:30:00Z",
        &page,
    );
    let fixture = load_lineage_packet(&issuer_fixture_dir(), "lineage_packet.json");
    assert_eq!(
        fixture, expected,
        "checked-in lineage_packet.json must match the projection of page.json"
    );
    assert!(fixture.raw_credentials_excluded);
    assert!(fixture.provider_versus_local_distinguished);
}

#[test]
fn fixture_lineage_packet_is_mirrored_under_privileged_action_lineage() {
    let primary = load_lineage_packet(&issuer_fixture_dir(), "lineage_packet.json");
    let mirror = load_lineage_packet(&lineage_fixture_dir(), "lineage_packet.json");
    assert_eq!(primary, mirror, "lineage packet mirrors must stay bit-for-bit identical");
}

#[test]
fn per_row_lineage_fixtures_match_packet_rows() {
    let packet = load_lineage_packet(&lineage_fixture_dir(), "lineage_packet.json");
    let rows: std::collections::BTreeMap<String, &RuntimeAuthorityLineageRow> = packet
        .lineage_rows
        .iter()
        .map(|row| (row.decision_id.clone(), row))
        .collect();
    for (file_name, decision_id) in [
        (
            "lineage_row_external_provider_mutation_granted.json",
            "decision:ai-tool:provider-publish:granted:0001",
        ),
        (
            "lineage_row_local_mutation_remembered_narrowed.json",
            "decision:cli-script:remembered-format:renewed:0002",
        ),
        (
            "lineage_row_credential_projection_self_authorization_refused.json",
            "decision:extension:self-authorize:refused:0003",
        ),
        (
            "lineage_row_browser_companion_ambient_privilege_refused.json",
            "decision:browser-companion:ambient:refused:0004",
        ),
        (
            "lineage_row_recipe_remembered_decision_drift_refused.json",
            "decision:recipe:broaden-remembered:refused:0005",
        ),
        (
            "lineage_row_remote_helper_local_only_source_refused.json",
            "decision:remote-helper:local-only-provider:refused:0006",
        ),
        (
            "lineage_row_admin_root_authority_change_granted.json",
            "decision:admin-console:root-rotation:granted:0007",
        ),
        (
            "lineage_row_privileged_debug_attach_granted.json",
            "decision:local-admin:debug-attach:granted:0008",
        ),
        (
            "lineage_row_credential_projection_granted.json",
            "decision:admin-console:credential-projection:granted:0009",
        ),
    ] {
        let row = load_lineage_row(&lineage_fixture_dir(), file_name);
        let expected = rows
            .get(decision_id)
            .unwrap_or_else(|| panic!("packet must contain {decision_id}"));
        assert_eq!(&row, *expected, "per-row fixture {file_name} drifted from packet");
    }
}

#[test]
fn drill_fixtures_surface_expected_defect_kinds() {
    let dir = issuer_fixture_dir();
    let cases: [(&str, &[RuntimeAuthorityIssuerDefectKind]); 12] = [
        (
            "drill_admitted_self_authorization.json",
            &[
                RuntimeAuthorityIssuerDefectKind::AdmittedSelfAuthorization,
                RuntimeAuthorityIssuerDefectKind::DecisionAdmittedWithoutChain,
            ],
        ),
        (
            "drill_admitted_ambient_privilege.json",
            &[RuntimeAuthorityIssuerDefectKind::AdmittedAmbientPrivilege],
        ),
        (
            "drill_admitted_broadened_remembered_rule.json",
            &[RuntimeAuthorityIssuerDefectKind::DecisionAdmittedOnSourceMismatch],
        ),
        (
            "drill_remembered_rule_broadened_scope.json",
            &[RuntimeAuthorityIssuerDefectKind::RememberedRuleNotNarrow],
        ),
        (
            "drill_remembered_rule_lifetime_exceeds_budget.json",
            &[RuntimeAuthorityIssuerDefectKind::RememberedRuleLifetimeExceedsBudget],
        ),
        (
            "drill_remembered_rule_forbidden_class.json",
            &[RuntimeAuthorityIssuerDefectKind::RememberedRuleForbiddenClass],
        ),
        (
            "drill_shell_root_authority_overreach.json",
            &[
                RuntimeAuthorityIssuerDefectKind::UnauthorizedRootAuthorityClaim,
                RuntimeAuthorityIssuerDefectKind::IssuerOverreach,
            ],
        ),
        (
            "drill_refused_without_reason.json",
            &[RuntimeAuthorityIssuerDefectKind::RefusedDecisionMissingReason],
        ),
        (
            "drill_local_only_admitted_to_provider.json",
            &[RuntimeAuthorityIssuerDefectKind::DecisionAdmittedOnSourceMismatch],
        ),
        (
            "drill_admitted_beyond_rule_expiry.json",
            &[RuntimeAuthorityIssuerDefectKind::DecisionAdmittedBeyondRuleExpiry],
        ),
        (
            "drill_admitted_without_root_proof.json",
            &[RuntimeAuthorityIssuerDefectKind::DecisionAdmittedWithoutChain],
        ),
        (
            "drill_refused_without_recovery_guidance.json",
            &[RuntimeAuthorityIssuerDefectKind::DecisionDroppedRecoveryGuidance],
        ),
    ];
    for (file_name, required) in cases {
        let page = load_page(&dir, file_name);
        let defects = audit_runtime_authority_issuer_page(
            &page.issuers,
            &page.requesting_surfaces,
            &page.remembered_rules,
            &page.requests,
            &page.decisions,
        );
        let observed: std::collections::BTreeSet<RuntimeAuthorityIssuerDefectKind> =
            defects.iter().map(|defect| defect.defect_kind).collect();
        for kind in required {
            assert!(
                observed.contains(kind),
                "drill {file_name} must surface {:?}, saw {:?}",
                kind,
                observed
            );
        }
        // The checked-in defects array must equal the validator recompute.
        let checked_in: std::collections::BTreeSet<RuntimeAuthorityIssuerDefectKind> =
            page.defects.iter().map(|defect| defect.defect_kind).collect();
        assert_eq!(
            checked_in, observed,
            "drill {file_name}: checked-in defects must match the validator recompute"
        );
    }
}
