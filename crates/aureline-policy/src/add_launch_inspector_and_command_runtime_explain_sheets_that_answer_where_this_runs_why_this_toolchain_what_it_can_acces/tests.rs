use super::*;

fn packet() -> M5LaunchInspectorPacket {
    frozen_stable_m5_launch_inspector_packet()
}

#[test]
fn frozen_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn frozen_packet_names_every_route() {
    let present: std::collections::BTreeSet<_> =
        packet().sheets.iter().map(|sheet| sheet.route).collect();
    for route in M5LaunchRoute::ALL {
        assert!(
            present.contains(&route),
            "packet missing explain sheet for route {}",
            route.as_str()
        );
    }
}

#[test]
fn every_sheet_answers_all_four_questions() {
    for sheet in packet().sheets {
        assert!(!sheet.where_it_runs.isolation_label.trim().is_empty());
        assert!(!sheet.why_this_toolchain.toolchain_label.trim().is_empty());
        assert!(!sheet
            .what_it_can_access
            .granted_capability_classes
            .is_empty());
        assert!(!sheet.who_approved_it.decision_chain.is_empty());
    }
}

#[test]
fn missing_route_fails_validation() {
    let mut packet = packet();
    packet
        .sheets
        .retain(|sheet| sheet.route != M5LaunchRoute::Companion);
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::RequiredRouteMissing));
}

#[test]
fn empty_capabilities_fail() {
    let mut packet = packet();
    packet.sheets[0]
        .what_it_can_access
        .granted_capability_classes
        .clear();
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::WhatAnswerIncomplete));
}

#[test]
fn missing_allowed_scope_fails() {
    let mut packet = packet();
    packet.sheets[0]
        .what_it_can_access
        .allowed_scope_labels
        .clear();
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::WhatAnswerIncomplete));
}

#[test]
fn empty_isolation_label_fails() {
    let mut packet = packet();
    packet.sheets[0].where_it_runs.isolation_label.clear();
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::WhereAnswerIncomplete));
}

#[test]
fn empty_toolchain_label_fails() {
    let mut packet = packet();
    packet.sheets[0].why_this_toolchain.toolchain_label.clear();
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::WhyAnswerIncomplete));
}

#[test]
fn empty_decision_chain_fails() {
    let mut packet = packet();
    packet.sheets[0].who_approved_it.decision_chain.clear();
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::WhoAnswerIncomplete));
}

#[test]
fn capability_widening_beyond_matrix_fails() {
    let mut packet = packet();
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|sheet| sheet.surface() == M5ExecutingSurface::NotebookKernel)
        .expect("notebook sheet exists");
    // The notebook matrix row never grants remote mutation.
    sheet
        .what_it_can_access
        .granted_capability_classes
        .push(M5CapabilityClass::RemoteMutation);
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::CapabilityWidensBeyondMatrix));
}

#[test]
fn sandbox_profile_widening_fails() {
    let mut packet = packet();
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|sheet| sheet.route == M5LaunchRoute::Remote)
        .expect("remote sheet exists");
    sheet.where_it_runs.sandbox_profile = M5SandboxProfile::InProcessTrustedLocal;
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::SandboxProfileWidens));
}

#[test]
fn inert_fail_closed_profile_is_allowed() {
    let mut packet = packet();
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|sheet| sheet.route == M5LaunchRoute::Remote)
        .expect("remote sheet exists");
    sheet.where_it_runs.sandbox_profile = M5SandboxProfile::InertNoExecution;
    assert!(!packet
        .validate()
        .contains(&M5LaunchInspectorViolation::SandboxProfileWidens));
}

#[test]
fn helper_route_self_issuing_authority_fails() {
    let mut packet = packet();
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|sheet| sheet.route == M5LaunchRoute::Ai)
        .expect("ai sheet exists");
    sheet.who_approved_it.self_issued_by_executor = true;
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::SelfIssuedAuthorityForbidden));
}

#[test]
fn elevated_capability_without_ticket_fails() {
    let mut packet = packet();
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|sheet| {
            sheet
                .what_it_can_access
                .granted_capability_classes
                .iter()
                .any(|cap| cap.is_elevated())
        })
        .expect("an elevated sheet exists");
    sheet.who_approved_it.approval_ticket_ref.clear();
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::ElevatedCapabilityWithoutTicket));
}

#[test]
fn secret_scope_without_projection_fails() {
    let mut packet = packet();
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|sheet| sheet.route == M5LaunchRoute::Recipe)
        .expect("recipe sheet exists");
    // Recipe grants no secret-projecting capability, so a secret scope is inconsistent.
    sheet.what_it_can_access.secret_scope = M5SecretScope::HandleOnlyDelegated;
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::SecretScopeInconsistent));
}

#[test]
fn off_device_unverified_target_fails() {
    let mut packet = packet();
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|sheet| sheet.where_it_runs.off_device)
        .expect("an off-device sheet exists");
    sheet.where_it_runs.target_identity_verified = false;
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::OffDeviceTargetUnverified));
}

#[test]
fn complete_status_with_degradation_fails() {
    let mut packet = packet();
    packet.sheets[0].degradation = Some(M5ExplainDegradation {
        reason: M5ExplainDegradationReason::StaleProofPacket,
        narrowed_to: M5DegradedFallback::NarrowToReadOnly,
        explanation: "stale".to_owned(),
    });
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::StatusInconsistent));
}

#[test]
fn degraded_status_without_reason_fails() {
    let mut packet = packet();
    packet.sheets[0].status = M5ExplainStatus::PartialDegraded;
    packet.sheets[0].degradation = None;
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::DegradedReasonMissing));
}

#[test]
fn degraded_status_without_explanation_fails() {
    let mut packet = packet();
    packet.sheets[0].status = M5ExplainStatus::PartialDegraded;
    packet.sheets[0].degradation = Some(M5ExplainDegradation {
        reason: M5ExplainDegradationReason::StaleProofPacket,
        narrowed_to: M5DegradedFallback::NarrowToReadOnly,
        explanation: "   ".to_owned(),
    });
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::DegradedExplanationMissing));
}

#[test]
fn unsupported_status_without_behavior_fails() {
    let mut packet = packet();
    packet.sheets[0].status = M5ExplainStatus::UnsupportedOnPlatform;
    packet.sheets[0].degradation = Some(M5ExplainDegradation {
        reason: M5ExplainDegradationReason::UnsupportedProfileOnPlatform,
        narrowed_to: M5DegradedFallback::FailClosedBlock,
        explanation: "unsupported on this platform".to_owned(),
    });
    packet.sheets[0].unsupported_profile_behavior = None;
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::UnsupportedBehaviorMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_ambient_machine_privilege = false;
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.all_routes_project_same_facts = false;
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5LaunchInspectorViolation::ProofFreshnessIncomplete));
}

#[test]
fn helper_routes_never_self_issue() {
    for sheet in packet().sheets {
        if sheet.route.is_helper_route() {
            assert!(
                !sheet.who_approved_it.self_issued_by_executor,
                "helper route {} must not self-issue authority",
                sheet.route.as_str()
            );
        }
    }
}

#[test]
fn granted_capabilities_never_widen_the_matrix() {
    let matrix = frozen_stable_m5_runtime_authority_matrix_packet();
    for sheet in packet().sheets {
        let row = matrix
            .surface_rows
            .iter()
            .find(|row| row.surface == sheet.surface())
            .expect("matrix has a row for every resolved surface");
        for cap in &sheet.what_it_can_access.granted_capability_classes {
            assert!(
                row.allowed_capability_classes.contains(cap),
                "sheet for route {} granted {} outside the matrix row",
                sheet.route.as_str(),
                cap.as_str()
            );
        }
    }
}

#[test]
fn every_sheet_references_a_covered_envelope_surface() {
    let envelope_surfaces: std::collections::BTreeSet<_> =
        frozen_stable_m5_capability_envelope_packet()
            .envelopes
            .iter()
            .map(|envelope| envelope.surface)
            .collect();
    for sheet in packet().sheets {
        assert!(
            envelope_surfaces.contains(&sheet.surface()),
            "sheet for route {} resolves to a surface with no capability envelope",
            sheet.route.as_str()
        );
    }
}

#[test]
fn markdown_summary_lists_every_route() {
    let summary = packet().render_markdown_summary();
    for route in M5LaunchRoute::ALL {
        assert!(
            summary.contains(route.as_str()),
            "summary missing route {}",
            route.as_str()
        );
    }
}

#[test]
fn checked_support_export_matches_frozen_packet() {
    let checked = current_stable_m5_launch_inspector_export()
        .expect("checked M5 launch-inspector export validates");
    assert_eq!(checked.packet_id, M5_LAUNCH_INSPECTOR_PACKET_ID);
    assert_eq!(
        checked,
        frozen_stable_m5_launch_inspector_packet(),
        "checked-in support export drifted from the frozen in-code packet; regenerate with the launch-inspector dumper"
    );
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/add-launch-inspector-and-command-runtime-explain-sheets-that-answer-where-this-runs-why-this-toolchain-what-it-can-acces/ai_tool_ticket_expired_partial.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/add-launch-inspector-and-command-runtime-explain-sheets-that-answer-where-this-runs-why-this-toolchain-what-it-can-acces/remote_profile_unsupported_headless.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/add-launch-inspector-and-command-runtime-explain-sheets-that-answer-where-this-runs-why-this-toolchain-what-it-can-acces/companion_stale_proof_partial.json"
        )),
    ] {
        let packet: M5LaunchInspectorPacket =
            serde_json::from_str(raw).expect("fixture parses as launch-inspector packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
