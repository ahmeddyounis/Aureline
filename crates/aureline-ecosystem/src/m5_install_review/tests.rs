use super::*;

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::current_m5_ecosystem_governance_matrix;

fn packet() -> M5InstallReview {
    current_m5_install_review().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_INSTALL_REVIEW_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_INSTALL_REVIEW_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_sheets() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_sheet_gate_is_consistent() {
    let packet = packet();
    assert!(packet.all_gates_consistent());
    for sheet in &packet.review_sheets {
        assert_eq!(
            sheet.compatibility_floor_change,
            sheet.computed_compatibility_floor_change(),
            "sheet {} compatibility floor diverges from the recomputed value",
            sheet.sheet_id
        );
        assert_eq!(
            sheet.review_triggers,
            sheet.computed_review_triggers(),
            "sheet {} review triggers diverge from the recomputed set",
            sheet.sheet_id
        );
        assert_eq!(
            sheet.commit_disposition,
            sheet.computed_commit_disposition(),
            "sheet {} commit disposition diverges from the recomputed gate",
            sheet.sheet_id
        );
    }
}

#[test]
fn guardrail_blocks_one_click_on_widening_publisher_or_runtime_change() {
    // The lane guardrail: widened permissions, a changed publisher, or a changed
    // runtime origin must never be committed with one click.
    let packet = packet();
    for sheet in &packet.review_sheets {
        if sheet.widens_permissions()
            || sheet.is_publisher_discontinuity()
            || sheet.runtime_origin_changed()
        {
            assert!(
                !sheet.allows_one_click(),
                "sheet {} offers a one-click commit despite a guardrail trigger",
                sheet.sheet_id
            );
        }
    }
}

#[test]
fn blocked_commit_never_enabled() {
    let packet = packet();
    for sheet in &packet.review_sheets {
        if sheet.commit_disposition == CommitDisposition::Blocked {
            if let Some(commit) = sheet.commit_action() {
                assert!(
                    !commit.enabled,
                    "sheet {} blocks the commit but enables the commit action",
                    sheet.sheet_id
                );
            }
        }
    }
}

#[test]
fn every_sheet_offers_commit_cancel_and_scoped_actions() {
    let packet = packet();
    assert!(!packet.review_sheets.is_empty());
    for sheet in &packet.review_sheets {
        assert!(
            sheet.offers_action(ReviewActionKind::Commit),
            "{}",
            sheet.sheet_id
        );
        assert!(
            sheet.offers_action(ReviewActionKind::Cancel),
            "{}",
            sheet.sheet_id
        );
        for action in &sheet.actions {
            assert_eq!(
                action.scope,
                sheet.scope,
                "sheet {} action {} escapes the sheet scope",
                sheet.sheet_id,
                action.action_kind.as_str()
            );
        }
    }
}

#[test]
fn state_changing_sheets_carry_a_rollback_route() {
    let packet = packet();
    for sheet in &packet.review_sheets {
        if sheet.rollback.rollback_posture != RollbackPosture::NotApplicable {
            assert!(
                !sheet.rollback.checkpoint_handle_ref.trim().is_empty(),
                "sheet {} has no rollback checkpoint handle",
                sheet.sheet_id
            );
            assert!(
                !sheet.rollback.fallback_package_ref.trim().is_empty(),
                "sheet {} has no current-package fallback route",
                sheet.sheet_id
            );
        }
    }
}

#[test]
fn every_package_kind_is_represented() {
    let packet = packet();
    let present: BTreeSet<ArtifactFamily> = packet
        .review_sheets
        .iter()
        .map(|s| s.package_kind)
        .collect();
    for kind in [
        ArtifactFamily::FirstPartyFrameworkPack,
        ArtifactFamily::DocsPack,
        ArtifactFamily::LocalModelPack,
        ArtifactFamily::SignedRecipePack,
        ArtifactFamily::TemplateArtifact,
        ArtifactFamily::BridgeBackedPackage,
    ] {
        assert!(
            present.contains(&kind),
            "no review sheet exercises package kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn every_scope_is_represented() {
    let packet = packet();
    let present: BTreeSet<InstallScope> = packet.review_sheets.iter().map(|s| s.scope).collect();
    for scope in InstallScope::ALL {
        assert!(
            present.contains(&scope),
            "no review sheet exercises scope {}",
            scope.as_str()
        );
    }
}

#[test]
fn every_commit_disposition_is_represented() {
    let packet = packet();
    let present: BTreeSet<CommitDisposition> = packet
        .review_sheets
        .iter()
        .map(|s| s.commit_disposition)
        .collect();
    for disposition in CommitDisposition::ALL {
        assert!(
            present.contains(&disposition),
            "no review sheet exercises commit disposition {}",
            disposition.as_str()
        );
    }
}

#[test]
fn every_review_trigger_is_represented() {
    let packet = packet();
    let present: BTreeSet<ReviewTrigger> = packet
        .review_sheets
        .iter()
        .flat_map(|s| s.review_triggers.iter().copied())
        .collect();
    for trigger in ReviewTrigger::ALL {
        assert!(
            present.contains(&trigger),
            "no review sheet exercises review trigger {}",
            trigger.as_str()
        );
    }
}

#[test]
fn transitive_widening_is_exercised() {
    let packet = packet();
    assert!(
        packet
            .review_sheets
            .iter()
            .any(|s| s.widens_transitive_permissions()),
        "no review sheet exercises transitive capability widening"
    );
}

#[test]
fn fresh_install_has_no_current_revision() {
    let packet = packet();
    for sheet in &packet.review_sheets {
        if sheet.change_kind == ReviewChangeKind::Install {
            assert!(
                sheet.current.is_none(),
                "fresh install sheet {} carries a current revision",
                sheet.sheet_id
            );
            assert_eq!(
                sheet.compatibility_floor_change,
                CompatibilityFloorChange::Initial,
                "{}",
                sheet.sheet_id
            );
        } else {
            assert!(
                sheet.current.is_some(),
                "comparison sheet {} is missing a current revision",
                sheet.sheet_id
            );
        }
    }
}

#[test]
fn every_sheet_resolves_to_a_governance_family() {
    let packet = packet();
    let governance = current_m5_ecosystem_governance_matrix().expect("governance matrix parses");
    for sheet in &packet.review_sheets {
        let family = governance.family(sheet.package_kind).unwrap_or_else(|| {
            panic!(
                "package kind {} is not a governance family",
                sheet.package_kind.as_str()
            )
        });
        assert_eq!(
            sheet.governance_family_ref, family.family_id,
            "sheet {} does not bind to its governance family",
            sheet.sheet_id
        );
    }
}

#[test]
fn export_projection_reflects_sheets() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.review_sheets.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_gates_consistent,
        packet.all_gates_consistent()
    );
    for export in &projection.rows {
        let sheet = packet
            .review_sheet(&export.sheet_id)
            .expect("export row resolves");
        assert_eq!(export.commit_disposition, sheet.commit_disposition.as_str());
        assert_eq!(export.one_click_allowed, sheet.allows_one_click());
        assert_eq!(export.widens_permissions, sheet.widens_permissions());
    }
}

#[test]
fn validate_flags_commit_disposition_mismatch() {
    let mut packet = packet();
    if let Some(sheet) = packet
        .review_sheets
        .iter_mut()
        .find(|s| s.commit_disposition != CommitDisposition::Blocked)
    {
        sheet.commit_disposition = CommitDisposition::Blocked;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5InstallReviewViolation::CommitDispositionMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_review_triggers_mismatch() {
    let mut packet = packet();
    if let Some(sheet) = packet
        .review_sheets
        .iter_mut()
        .find(|s| !s.review_triggers.contains(&ReviewTrigger::OpenWorkImpacted))
    {
        sheet.review_triggers.push(ReviewTrigger::OpenWorkImpacted);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5InstallReviewViolation::ReviewTriggersMismatch { .. }
                | M5InstallReviewViolation::CommitDispositionMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_action_scope_mismatch() {
    let mut packet = packet();
    if let Some(sheet) = packet
        .review_sheets
        .iter_mut()
        .find(|s| !s.actions.is_empty())
    {
        let other = match sheet.scope {
            InstallScope::Workspace => InstallScope::Global,
            _ => InstallScope::Workspace,
        };
        sheet.actions[0].scope = other;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5InstallReviewViolation::ActionScopeMismatch { .. })));
    }
}

#[test]
fn validate_flags_publisher_transfer_inconsistency() {
    let mut packet = packet();
    if let Some(sheet) = packet.review_sheets.iter_mut().find(|s| {
        s.current.is_some() && s.publisher_transfer == PublisherTransferState::SamePublisher
    }) {
        // Same-publisher labels with a divergent publisher ref must be caught.
        sheet.proposed.publisher_ref = "publisher:unrelated".to_owned();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5InstallReviewViolation::PublisherTransferInconsistent { .. }
        )));
    }
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_sheets = packet.summary.total_sheets.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5InstallReviewViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(ReviewChangeKind::Update.as_str(), "update");
    assert_eq!(InstallScope::Global.as_str(), "global");
    assert_eq!(HostClass::ManagedWorkspace.as_str(), "managed_workspace");
    assert_eq!(CapabilityOrigin::Transitive.as_str(), "transitive");
    assert_eq!(CapabilityChange::Widened.as_str(), "widened");
    assert_eq!(
        PublisherTransferState::TransferredUnverified.as_str(),
        "transferred_unverified"
    );
    assert_eq!(SigningRootContinuity::RootChanged.as_str(), "root_changed");
    assert_eq!(NamespaceState::Orphaned.as_str(), "orphaned");
    assert_eq!(
        RestartImpact::ReattachRequired.as_str(),
        "reattach_required"
    );
    assert_eq!(
        CompatibilityFloorChange::RegressedToUnsupported.as_str(),
        "regressed_to_unsupported"
    );
    assert_eq!(
        ReviewTrigger::PermissionsWidened.as_str(),
        "permissions_widened"
    );
    assert_eq!(
        CommitDisposition::UnifiedReviewRequired.as_str(),
        "unified_review_required"
    );
}

#[test]
fn disposition_widens_monotonically() {
    assert!(
        CommitDisposition::OneClickAllowed.rank() < CommitDisposition::UnifiedReviewRequired.rank()
    );
    assert!(CommitDisposition::UnifiedReviewRequired.rank() < CommitDisposition::Blocked.rank());
    assert_eq!(
        CommitDisposition::OneClickAllowed.widen(CommitDisposition::Blocked),
        CommitDisposition::Blocked
    );
    assert_eq!(
        CommitDisposition::Blocked.widen(CommitDisposition::UnifiedReviewRequired),
        CommitDisposition::Blocked
    );
}
