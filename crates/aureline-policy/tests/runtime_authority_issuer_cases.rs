use aureline_policy::{
    audit_runtime_authority_issuer_page, seeded_runtime_authority_issuer_page,
    validate_runtime_authority_issuer_page, AuthoritySourceClass, IssuerBoundaryDecisionClass,
    IssuerBoundaryRejectionReason, RequestingSurfaceClass, RuntimeAuthorityIssuerDefectKind,
    RuntimeAuthorityLineagePacket,
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
