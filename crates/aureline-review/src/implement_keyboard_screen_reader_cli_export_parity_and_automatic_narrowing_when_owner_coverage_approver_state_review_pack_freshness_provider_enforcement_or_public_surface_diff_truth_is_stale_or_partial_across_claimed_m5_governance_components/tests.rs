use super::*;

const PACKET_ID: &str = "protected-path-governance-accessibility:stable:0001";

fn trust_review() -> GovernanceComponentAccessibilityTrustReview {
    GovernanceComponentAccessibilityTrustReview {
        keyboard_reachable_on_every_claim: true,
        screen_reader_labeled_on_every_claim: true,
        cli_enum_exposed_on_every_claim: true,
        export_enum_exposed_on_every_claim: true,
        explanation_field_present_on_every_claim: true,
        no_component_pointer_only: true,
        no_component_export_opaque: true,
        desktop_never_stronger_than_cli: true,
        claim_narrows_when_governance_evidence_weakens: true,
        governed_authority_never_overstated_under_weakening: true,
        owner_approver_public_surface_semantics_kept_explicit: true,
        advisory_never_promoted_to_provider_authoritative: true,
    }
}

fn projection() -> GovernanceComponentAccessibilityProjection {
    GovernanceComponentAccessibilityProjection {
        exposes_keyboard_and_screen_reader_labels: true,
        exposes_cli_and_export_enums: true,
        exposes_explanation_fields: true,
        auto_narrows_on_stale_provider_enforcement: true,
        auto_narrows_on_partial_owner_coverage: true,
        auto_narrows_on_stale_approver_state: true,
        auto_narrows_on_stale_review_pack: true,
        auto_narrows_on_partial_public_surface_diff: true,
        desktop_cli_export_semantics_identical: true,
        narrowing_prevents_overstated_governed_authority: true,
    }
}

fn proof_freshness() -> GovernanceComponentAccessibilityProofFreshness {
    GovernanceComponentAccessibilityProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<GovernanceComponentAccessibilityDowngradeTrigger> {
    vec![
        GovernanceComponentAccessibilityDowngradeTrigger::ProofStale,
        GovernanceComponentAccessibilityDowngradeTrigger::ProviderEnforcementStaleOrPartial,
        GovernanceComponentAccessibilityDowngradeTrigger::OwnerCoveragePartial,
        GovernanceComponentAccessibilityDowngradeTrigger::ApproverStateStaleOrPartial,
        GovernanceComponentAccessibilityDowngradeTrigger::ReviewPackFreshnessStale,
        GovernanceComponentAccessibilityDowngradeTrigger::PublicSurfaceDiffTruthPartial,
        GovernanceComponentAccessibilityDowngradeTrigger::ClaimOverstated,
    ]
}

fn rendering_surfaces() -> Vec<GovernanceComponentRenderingSurface> {
    GovernanceComponentRenderingSurface::ALL.to_vec()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        GOVERNANCE_COMPONENT_ACCESSIBILITY_SCHEMA_REF.to_owned(),
        GOVERNANCE_COMPONENT_ACCESSIBILITY_DOC_REF.to_owned(),
        GOVERNANCE_COMPONENT_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        GOVERNANCE_COMPONENT_ACCESSIBILITY_CONSUMER_CONTRACT_REF.to_owned(),
        GOVERNANCE_COMPONENT_ACCESSIBILITY_PROTECTED_PATH_OWNERSHIP_CONTROLS_CONTRACT_REF
            .to_owned(),
        GOVERNANCE_COMPONENT_ACCESSIBILITY_APPROVER_REVIEW_PACK_CONTROLS_CONTRACT_REF.to_owned(),
        GOVERNANCE_COMPONENT_ACCESSIBILITY_PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_CONTRACT_REF
            .to_owned(),
        GOVERNANCE_COMPONENT_ACCESSIBILITY_DRI_REGISTRY_MERGE_READINESS_CONTROLS_CONTRACT_REF
            .to_owned(),
    ]
}

fn row_refs(component: M5GovernanceComponent) -> Vec<String> {
    vec![
        GOVERNANCE_COMPONENT_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        component_canonical_schema_ref(component).to_owned(),
    ]
}

fn human_component(component: M5GovernanceComponent) -> &'static str {
    match component {
        M5GovernanceComponent::ProtectedPathRow => "Protected-path row",
        M5GovernanceComponent::OwnershipCard => "Ownership card",
        M5GovernanceComponent::ApproverMatrix => "Approver matrix",
        M5GovernanceComponent::ReviewPackSummary => "Review-pack summary",
        M5GovernanceComponent::PublicSurfaceDiffCard => "Public-surface diff card",
        M5GovernanceComponent::MergeControlBanner => "Merge-control banner",
        M5GovernanceComponent::DriRegistryRow => "DRI-registry row",
        M5GovernanceComponent::MergeReadinessStrip => "Merge-readiness strip",
    }
}

fn claim_phrase(tier: GovernanceComponentClaimTier) -> &'static str {
    match tier {
        GovernanceComponentClaimTier::FullGovernedAuthority => {
            "provider-authoritative with full owner coverage, satisfied approvals, and a fresh review pack"
        }
        GovernanceComponentClaimTier::AdvisoryEnforcementOnly => {
            "advisory or local-estimate enforcement, never provider-authoritative"
        }
        GovernanceComponentClaimTier::OwnerBackupCoverageMissing => {
            "owner resolved but backup coverage is missing"
        }
        GovernanceComponentClaimTier::ApproverStateNarrowed => {
            "a required approval is waived or expired"
        }
        GovernanceComponentClaimTier::ReviewPackStaleDisclosed => {
            "the review pack is stale relative to the change"
        }
        GovernanceComponentClaimTier::PublicSurfaceEvidenceWithheld => {
            "public-surface diff and migration evidence are withheld pending generation"
        }
    }
}

fn condition_phrase(condition: GovernanceComponentClaimCondition) -> &'static str {
    match condition {
        GovernanceComponentClaimCondition::GovernanceTruthTrusted => {
            "provider enforcement, owner coverage, approver state, review pack, and public-surface diff are all trusted"
        }
        GovernanceComponentClaimCondition::ProviderEnforcementStaleOrPartial => {
            "provider enforcement is advisory, stale, or a local estimate"
        }
        GovernanceComponentClaimCondition::OwnerCoveragePartial => {
            "owner backup coverage is missing or unresolved"
        }
        GovernanceComponentClaimCondition::ApproverStateStaleOrPartial => {
            "a required approval is waived or has expired"
        }
        GovernanceComponentClaimCondition::ReviewPackFreshnessStale => {
            "the review pack is stale relative to the change it gates"
        }
        GovernanceComponentClaimCondition::PublicSurfaceDiffTruthPartial => {
            "the public-surface diff or migration evidence is partial or ungenerated"
        }
    }
}

fn next_action_label(action: GovernanceComponentClaimNextAction) -> String {
    match action {
        GovernanceComponentClaimNextAction::SeekProviderEnforcementClearance => {
            "Seek provider enforcement clearance before treating this guard as authoritative"
                .to_owned()
        }
        GovernanceComponentClaimNextAction::ResolveOwnerBackupCoverage => {
            "Resolve the missing owner backup coverage before relying on the guard".to_owned()
        }
        GovernanceComponentClaimNextAction::RefreshApproverState => {
            "Refresh the waived or expired approver state before merging".to_owned()
        }
        GovernanceComponentClaimNextAction::RefreshReviewPack => {
            "Refresh the review pack against the current change".to_owned()
        }
        GovernanceComponentClaimNextAction::GeneratePublicSurfaceDiff => {
            "Generate the machine public-surface diff and migration evidence before landing"
                .to_owned()
        }
        GovernanceComponentClaimNextAction::ContinueGovernedReview => {
            "Continue the governed review".to_owned()
        }
    }
}

/// Builds one accessibility row, deriving the claim, narrowing, notes, and labels from the
/// component and condition so the fixture stays self-consistent.
fn row(
    row_id: &str,
    component: M5GovernanceComponent,
    condition: GovernanceComponentClaimCondition,
) -> GovernanceComponentAccessibilityRow {
    let resolution = resolve_governance_component_claim_narrowing(condition);
    let effective_claim = resolution.permitted_ceiling;

    let narrowing = if resolution.requires_narrowing {
        Some(GovernanceComponentClaimNarrowing {
            trigger: resolution
                .expected_trigger
                .expect("weakening condition has a trigger"),
            narrowed_to: resolution.permitted_ceiling,
            preserved_truth_note: format!(
                "{} stays keyboard-reachable, screen-reader labelled, and export-legible; only the governed-authority claim is narrowed",
                human_component(component)
            ),
            next_action: resolution.expected_next_action,
            next_action_label: next_action_label(resolution.expected_next_action),
        })
    } else {
        None
    };

    let governed_semantics_note = if resolution.needs_governed_semantics_note {
        format!(
            "The {} keeps its explicit owner, approver, and public-surface semantics here; it never drops to a vague `governed` label",
            human_component(component).to_lowercase()
        )
    } else {
        String::new()
    };
    let enforcement_authority_note = if resolution.needs_enforcement_authority_note {
        "Enforcement is advisory or a local estimate here; it is never presented as provider-authoritative".to_owned()
    } else {
        String::new()
    };
    let backup_coverage_note = if resolution.needs_backup_coverage_note {
        "Owner backup coverage is missing; this guarded path is not fully covered and the gap is never hidden".to_owned()
    } else {
        String::new()
    };
    let approver_state_note = if resolution.needs_approver_state_note {
        "A required approval is waived or expired; the narrowed approver state is never hidden behind a clean pass".to_owned()
    } else {
        String::new()
    };
    let public_surface_evidence_note = if resolution.needs_public_surface_evidence_note {
        "The machine public-surface diff and migration evidence are missing; the change never reads clean without them".to_owned()
    } else {
        String::new()
    };

    GovernanceComponentAccessibilityRow {
        row_id: row_id.to_owned(),
        component,
        condition,
        effective_claim,
        keyboard_label: format!(
            "{}: focusable, Enter opens, Space toggles detail",
            human_component(component)
        ),
        screen_reader_label: format!(
            "{}, {}",
            human_component(component),
            claim_phrase(effective_claim)
        ),
        cli_enum_token: format!("{}:{}", component.as_str(), effective_claim.as_str()),
        export_enum_token: effective_claim.as_str().to_owned(),
        explanation_field: format!(
            "{} — {}",
            claim_phrase(effective_claim),
            condition_phrase(condition)
        ),
        rendering_surfaces: rendering_surfaces(),
        narrowing,
        governed_semantics_note,
        enforcement_authority_note,
        backup_coverage_note,
        approver_state_note,
        public_surface_evidence_note,
        is_pointer_only: false,
        is_export_opaque: false,
        desktop_stronger_than_cli: false,
        source_contract_refs: row_refs(component),
    }
}

/// The canonical row set: all eight components, covering all six conditions and all six
/// claim tiers.
fn accessibility_rows() -> Vec<GovernanceComponentAccessibilityRow> {
    vec![
        row(
            "row:protected-path-trusted",
            M5GovernanceComponent::ProtectedPathRow,
            GovernanceComponentClaimCondition::GovernanceTruthTrusted,
        ),
        row(
            "row:ownership-card-owner-coverage",
            M5GovernanceComponent::OwnershipCard,
            GovernanceComponentClaimCondition::OwnerCoveragePartial,
        ),
        row(
            "row:approver-matrix-approver-state",
            M5GovernanceComponent::ApproverMatrix,
            GovernanceComponentClaimCondition::ApproverStateStaleOrPartial,
        ),
        row(
            "row:review-pack-summary-stale",
            M5GovernanceComponent::ReviewPackSummary,
            GovernanceComponentClaimCondition::ReviewPackFreshnessStale,
        ),
        row(
            "row:public-surface-diff-partial",
            M5GovernanceComponent::PublicSurfaceDiffCard,
            GovernanceComponentClaimCondition::PublicSurfaceDiffTruthPartial,
        ),
        row(
            "row:merge-control-banner-enforcement",
            M5GovernanceComponent::MergeControlBanner,
            GovernanceComponentClaimCondition::ProviderEnforcementStaleOrPartial,
        ),
        row(
            "row:dri-registry-trusted",
            M5GovernanceComponent::DriRegistryRow,
            GovernanceComponentClaimCondition::GovernanceTruthTrusted,
        ),
        row(
            "row:merge-readiness-strip-enforcement",
            M5GovernanceComponent::MergeReadinessStrip,
            GovernanceComponentClaimCondition::ProviderEnforcementStaleOrPartial,
        ),
    ]
}

fn packet_with(
    rows: Vec<GovernanceComponentAccessibilityRow>,
) -> GovernanceComponentAccessibilityPacket {
    GovernanceComponentAccessibilityPacket::new(GovernanceComponentAccessibilityPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Protected-path governance accessibility, headless, and export parity"
            .to_owned(),
        accessibility_rows: rows,
        downgrade_triggers: downgrade_triggers(),
        rendering_surfaces: rendering_surfaces(),
        trust_review: trust_review(),
        projection: projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

fn packet() -> GovernanceComponentAccessibilityPacket {
    packet_with(accessibility_rows())
}

#[test]
fn accessibility_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_canonical_row_is_honest() {
    for row in accessibility_rows() {
        assert!(row.claim_is_honest(), "row not honest: {}", row.row_id);
    }
}

#[test]
fn claim_narrowing_maps_condition_to_ceiling() {
    let trusted = resolve_governance_component_claim_narrowing(
        GovernanceComponentClaimCondition::GovernanceTruthTrusted,
    );
    assert_eq!(
        trusted.permitted_ceiling,
        GovernanceComponentClaimTier::FullGovernedAuthority
    );
    assert!(!trusted.requires_narrowing);
    assert!(trusted.expected_trigger.is_none());
    assert!(!trusted.needs_governed_semantics_note);
    assert!(!trusted.needs_enforcement_authority_note);
    assert!(!trusted.needs_backup_coverage_note);
    assert!(!trusted.needs_approver_state_note);
    assert!(!trusted.needs_public_surface_evidence_note);

    let enforcement = resolve_governance_component_claim_narrowing(
        GovernanceComponentClaimCondition::ProviderEnforcementStaleOrPartial,
    );
    assert_eq!(
        enforcement.permitted_ceiling,
        GovernanceComponentClaimTier::AdvisoryEnforcementOnly
    );
    assert!(enforcement.requires_narrowing);
    assert!(enforcement.needs_governed_semantics_note);
    assert!(enforcement.needs_enforcement_authority_note);
    assert!(!enforcement.needs_backup_coverage_note);

    let coverage = resolve_governance_component_claim_narrowing(
        GovernanceComponentClaimCondition::OwnerCoveragePartial,
    );
    assert_eq!(
        coverage.permitted_ceiling,
        GovernanceComponentClaimTier::OwnerBackupCoverageMissing
    );
    assert!(coverage.needs_backup_coverage_note);
    assert!(!coverage.needs_enforcement_authority_note);

    let approver = resolve_governance_component_claim_narrowing(
        GovernanceComponentClaimCondition::ApproverStateStaleOrPartial,
    );
    assert_eq!(
        approver.permitted_ceiling,
        GovernanceComponentClaimTier::ApproverStateNarrowed
    );
    assert_eq!(
        approver.expected_trigger,
        Some(GovernanceComponentAccessibilityDowngradeTrigger::ApproverStateStaleOrPartial)
    );
    assert!(approver.needs_approver_state_note);

    let review_pack = resolve_governance_component_claim_narrowing(
        GovernanceComponentClaimCondition::ReviewPackFreshnessStale,
    );
    assert_eq!(
        review_pack.permitted_ceiling,
        GovernanceComponentClaimTier::ReviewPackStaleDisclosed
    );
    assert!(review_pack.needs_governed_semantics_note);
    assert!(!review_pack.needs_backup_coverage_note);
    assert!(!review_pack.needs_public_surface_evidence_note);

    let public_surface = resolve_governance_component_claim_narrowing(
        GovernanceComponentClaimCondition::PublicSurfaceDiffTruthPartial,
    );
    assert_eq!(
        public_surface.permitted_ceiling,
        GovernanceComponentClaimTier::PublicSurfaceEvidenceWithheld
    );
    assert!(public_surface.needs_public_surface_evidence_note);
    assert!(!public_surface.needs_approver_state_note);
}

// --- AC2: narrowing prevents overstated governed authority --------------------

#[test]
fn full_authority_claim_never_survives_a_weakening_condition() {
    // A component that keeps asserting full governed authority while owner coverage is
    // partial overstates its truth and must be caught.
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == GovernanceComponentClaimCondition::OwnerCoveragePartial)
        .expect("owner-coverage row present");
    packet.accessibility_rows[index].effective_claim =
        GovernanceComponentClaimTier::FullGovernedAuthority;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::ClaimCeilingExceeded));
}

#[test]
fn claim_ceiling_exceeded_on_public_surface_partial_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| {
            r.condition == GovernanceComponentClaimCondition::PublicSurfaceDiffTruthPartial
        })
        .expect("public-surface row present");
    // Claim owner-backup-coverage-missing (rank 4) above the public-surface ceiling (rank 1).
    packet.accessibility_rows[index].effective_claim =
        GovernanceComponentClaimTier::OwnerBackupCoverageMissing;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::ClaimCeilingExceeded));
}

#[test]
fn weakening_condition_without_narrowing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    packet.accessibility_rows[index].narrowing = None;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::ClaimNarrowingMissing));
}

#[test]
fn baseline_condition_with_narrowing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == GovernanceComponentClaimCondition::GovernanceTruthTrusted)
        .expect("trusted row present");
    packet.accessibility_rows[index].narrowing = Some(GovernanceComponentClaimNarrowing {
        trigger: GovernanceComponentAccessibilityDowngradeTrigger::OwnerCoveragePartial,
        narrowed_to: GovernanceComponentClaimTier::FullGovernedAuthority,
        preserved_truth_note: "note".to_owned(),
        next_action: GovernanceComponentClaimNextAction::ContinueGovernedReview,
        next_action_label: "Continue".to_owned(),
    });
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::ClaimNarrowingUnexpected));
}

#[test]
fn narrowed_to_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.narrowed_to = GovernanceComponentClaimTier::PublicSurfaceEvidenceWithheld;
    }
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::NarrowedToMismatch));
}

#[test]
fn narrow_trigger_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == GovernanceComponentClaimCondition::OwnerCoveragePartial)
        .expect("owner-coverage row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.trigger =
            GovernanceComponentAccessibilityDowngradeTrigger::ProviderEnforcementStaleOrPartial;
    }
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::NarrowTriggerMismatch));
}

#[test]
fn narrow_next_action_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| {
            r.condition == GovernanceComponentClaimCondition::ProviderEnforcementStaleOrPartial
        })
        .expect("enforcement row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.next_action = GovernanceComponentClaimNextAction::ContinueGovernedReview;
    }
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::NarrowNextActionMismatch));
}

#[test]
fn narrow_missing_preserved_truth_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.preserved_truth_note = String::new();
    }
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::NarrowPreservedTruthMissing));
}

#[test]
fn narrow_missing_next_action_label_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.next_action_label = String::new();
    }
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::NarrowNextActionMissing));
}

// --- AC1: parity across keyboard / screen-reader / CLI / export ---------------

#[test]
fn keyboard_label_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].keyboard_label = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::KeyboardLabelMissing));
}

#[test]
fn screen_reader_label_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].screen_reader_label = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::ScreenReaderLabelMissing));
}

#[test]
fn cli_enum_token_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].cli_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::CliEnumTokenMissing));
}

#[test]
fn export_enum_token_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].export_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::ExportEnumTokenMissing));
}

#[test]
fn explanation_field_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].explanation_field = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::ExplanationFieldMissing));
}

#[test]
fn rendering_surface_coverage_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].rendering_surfaces =
        vec![GovernanceComponentRenderingSurface::DesktopFull];
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::RenderingSurfaceCoverageMissing));
}

#[test]
fn pointer_only_component_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].is_pointer_only = true;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::PointerOnlyComponent));
}

#[test]
fn export_opaque_component_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].is_export_opaque = true;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::ExportOpaqueComponent));
}

#[test]
fn desktop_stronger_than_cli_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].desktop_stronger_than_cli = true;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::DesktopStrongerThanCli));
}

#[test]
fn governed_semantics_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    packet.accessibility_rows[index].governed_semantics_note = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::GovernedSemanticsNoteMissing));
}

#[test]
fn enforcement_authority_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| {
            r.condition == GovernanceComponentClaimCondition::ProviderEnforcementStaleOrPartial
        })
        .expect("enforcement row present");
    packet.accessibility_rows[index].enforcement_authority_note = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::EnforcementAuthorityNoteMissing));
}

#[test]
fn backup_coverage_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == GovernanceComponentClaimCondition::OwnerCoveragePartial)
        .expect("owner-coverage row present");
    packet.accessibility_rows[index].backup_coverage_note = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::BackupCoverageNoteMissing));
}

#[test]
fn approver_state_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == GovernanceComponentClaimCondition::ApproverStateStaleOrPartial)
        .expect("approver row present");
    packet.accessibility_rows[index].approver_state_note = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::ApproverStateNoteMissing));
}

#[test]
fn public_surface_evidence_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| {
            r.condition == GovernanceComponentClaimCondition::PublicSurfaceDiffTruthPartial
        })
        .expect("public-surface row present");
    packet.accessibility_rows[index].public_surface_evidence_note = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::PublicSurfaceEvidenceNoteMissing));
}

#[test]
fn canonical_contract_reference_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].source_contract_refs =
        vec![GOVERNANCE_COMPONENT_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::CanonicalContractReferenceMissing));
}

// --- Coverage -----------------------------------------------------------------

#[test]
fn missing_component_coverage_fails() {
    let mut rows = accessibility_rows();
    rows.retain(|r| r.component != M5GovernanceComponent::MergeReadinessStrip);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::ComponentCoverageMissing));
}

#[test]
fn missing_condition_coverage_fails() {
    let mut rows = accessibility_rows();
    // Drop the only approver-state row.
    rows.retain(|r| r.condition != GovernanceComponentClaimCondition::ApproverStateStaleOrPartial);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::ConditionCoverageMissing));
}

#[test]
fn missing_claim_tier_coverage_fails() {
    let mut rows = accessibility_rows();
    // Drop the only review-pack-stale row; that tier is then unreachable.
    rows.retain(|r| r.effective_claim != GovernanceComponentClaimTier::ReviewPackStaleDisclosed);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::ClaimTierCoverageMissing));
}

// --- Structural ---------------------------------------------------------------

#[test]
fn row_incomplete_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].row_id = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::RowIncomplete));
}

#[test]
fn missing_rows_fails() {
    let mut packet = packet();
    packet.accessibility_rows.clear();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::AccessibilityRowsMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .advisory_never_promoted_to_provider_authoritative = false;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::TrustReviewIncomplete));
}

#[test]
fn projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .projection
        .narrowing_prevents_overstated_governed_authority = false;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::ProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentAccessibilityViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Accessibility rows"));
    assert!(summary.contains("protected_path_row"));
    assert!(summary.contains("merge_readiness_strip"));
    assert!(summary.contains("advisory_enforcement_only"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_governance_component_accessibility_export()
        .expect("checked protected-path governance accessibility export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-protected-path-governance-component-accessibility-parity/enforcement_and_coverage_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-protected-path-governance-component-accessibility-parity/review_pack_and_public_surface_narrowed.json"
        )),
    ] {
        let packet: GovernanceComponentAccessibilityPacket = serde_json::from_str(raw)
            .expect("fixture parses as protected-path governance accessibility packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

// --- Fixture builders ---------------------------------------------------------

/// The canonical rows plus extra scenario rows that demonstrate a normally-trusted
/// component auto-narrowing under stale provider enforcement and partial owner coverage.
fn fixture_enforcement_and_coverage_narrowed() -> GovernanceComponentAccessibilityPacket {
    let mut rows = accessibility_rows();
    rows.push(row(
        "row:protected-path-enforcement-narrowed",
        M5GovernanceComponent::ProtectedPathRow,
        GovernanceComponentClaimCondition::ProviderEnforcementStaleOrPartial,
    ));
    rows.push(row(
        "row:dri-registry-owner-coverage-narrowed",
        M5GovernanceComponent::DriRegistryRow,
        GovernanceComponentClaimCondition::OwnerCoveragePartial,
    ));
    GovernanceComponentAccessibilityPacket::new(GovernanceComponentAccessibilityPacketInput {
        packet_id:
            "protected-path-governance-accessibility:fixture:enforcement-and-coverage-narrowed"
                .to_owned(),
        surface_label:
            "Protected-path governance accessibility: provider enforcement stale and owner coverage partial, claim auto-narrowed"
                .to_owned(),
        accessibility_rows: rows,
        downgrade_triggers: vec![
            GovernanceComponentAccessibilityDowngradeTrigger::ProviderEnforcementStaleOrPartial,
            GovernanceComponentAccessibilityDowngradeTrigger::OwnerCoveragePartial,
            GovernanceComponentAccessibilityDowngradeTrigger::ClaimOverstated,
        ],
        rendering_surfaces: rendering_surfaces(),
        trust_review: trust_review(),
        projection: projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

/// The canonical rows plus extra scenario rows for a review-pack summary going stale and a
/// merge-control banner losing public-surface diff truth.
fn fixture_review_pack_and_public_surface_narrowed() -> GovernanceComponentAccessibilityPacket {
    let mut rows = accessibility_rows();
    rows.push(row(
        "row:merge-readiness-review-pack-narrowed",
        M5GovernanceComponent::MergeReadinessStrip,
        GovernanceComponentClaimCondition::ReviewPackFreshnessStale,
    ));
    rows.push(row(
        "row:merge-control-public-surface-narrowed",
        M5GovernanceComponent::MergeControlBanner,
        GovernanceComponentClaimCondition::PublicSurfaceDiffTruthPartial,
    ));
    GovernanceComponentAccessibilityPacket::new(GovernanceComponentAccessibilityPacketInput {
        packet_id:
            "protected-path-governance-accessibility:fixture:review-pack-and-public-surface-narrowed"
                .to_owned(),
        surface_label:
            "Protected-path governance accessibility: review pack stale and public-surface diff partial"
                .to_owned(),
        accessibility_rows: rows,
        downgrade_triggers: vec![
            GovernanceComponentAccessibilityDowngradeTrigger::ReviewPackFreshnessStale,
            GovernanceComponentAccessibilityDowngradeTrigger::PublicSurfaceDiffTruthPartial,
            GovernanceComponentAccessibilityDowngradeTrigger::ClaimOverstated,
        ],
        rendering_surfaces: rendering_surfaces(),
        trust_review: trust_review(),
        projection: projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

/// Regenerates the checked-in support export, summary, and fixtures.
///
/// Gated behind `GEN_GOVERNANCE_COMPONENT_A11Y_ARTIFACTS` so it never writes during a
/// normal test run. Run with the env var set to refresh the artifacts after a contract
/// change, then review the diff.
#[test]
fn regenerate_governance_component_accessibility_artifacts() {
    if std::env::var("GEN_GOVERNANCE_COMPONENT_A11Y_ARTIFACTS").is_err() {
        return;
    }

    let manifest = env!("CARGO_MANIFEST_DIR");
    let root = format!("{manifest}/../..");

    let canonical = packet();
    assert!(
        canonical.validate().is_empty(),
        "{:?}",
        canonical.validate()
    );

    let artifact_dir =
        format!("{root}/artifacts/release/m5-protected-path-governance-accessibility-proof");
    std::fs::create_dir_all(&artifact_dir).expect("create artifact dir");
    std::fs::write(
        format!("{artifact_dir}/support_export.json"),
        format!("{}\n", canonical.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        format!("{artifact_dir}/summary.md"),
        canonical.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir =
        format!("{root}/fixtures/ui/m5-protected-path-governance-component-accessibility-parity");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    for (name, fixture) in [
        (
            "enforcement_and_coverage_narrowed.json",
            fixture_enforcement_and_coverage_narrowed(),
        ),
        (
            "review_pack_and_public_surface_narrowed.json",
            fixture_review_pack_and_public_surface_narrowed(),
        ),
    ] {
        assert!(
            fixture.validate().is_empty(),
            "{name}: {:?}",
            fixture.validate()
        );
        std::fs::write(
            format!("{fixture_dir}/{name}"),
            format!("{}\n", fixture.export_safe_json()),
        )
        .expect("write fixture");
    }
}
