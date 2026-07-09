use super::*;

const PACKET_ID: &str = "package-management-component-accessibility:stable:0001";

fn trust_review() -> PackageComponentAccessibilityTrustReview {
    PackageComponentAccessibilityTrustReview {
        keyboard_reachable_on_every_claim: true,
        screen_reader_labeled_on_every_claim: true,
        cli_enum_exposed_on_every_claim: true,
        export_enum_exposed_on_every_claim: true,
        explanation_field_present_on_every_claim: true,
        no_component_pointer_only: true,
        no_component_export_opaque: true,
        desktop_never_stronger_than_cli: true,
        claim_narrows_when_package_truth_weakens: true,
        scope_and_side_effect_never_overstated_under_weakening: true,
        mirror_offline_continuity_kept_explicit: true,
        rollback_checkpoint_truth_kept_explicit_before_write: true,
    }
}

fn projection() -> PackageComponentAccessibilityProjection {
    PackageComponentAccessibilityProjection {
        exposes_keyboard_and_screen_reader_labels: true,
        exposes_cli_and_export_enums: true,
        exposes_explanation_fields: true,
        auto_narrows_on_partial_manifest_scope: true,
        auto_narrows_on_stale_registry_freshness: true,
        auto_narrows_on_unsatisfied_auth_state: true,
        auto_narrows_on_unavailable_lockfile_impact: true,
        auto_narrows_on_unavailable_rollback_checkpoint: true,
        desktop_cli_export_semantics_identical: true,
        narrowing_prevents_overstated_package_management_scope: true,
    }
}

fn proof_freshness() -> PackageComponentAccessibilityProofFreshness {
    PackageComponentAccessibilityProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<PackageComponentAccessibilityDowngradeTrigger> {
    vec![
        PackageComponentAccessibilityDowngradeTrigger::ProofStale,
        PackageComponentAccessibilityDowngradeTrigger::ManifestScopePartial,
        PackageComponentAccessibilityDowngradeTrigger::RegistryFreshnessStale,
        PackageComponentAccessibilityDowngradeTrigger::AuthStateUnsatisfied,
        PackageComponentAccessibilityDowngradeTrigger::LockfileImpactUnavailable,
        PackageComponentAccessibilityDowngradeTrigger::RollbackCheckpointUnavailable,
        PackageComponentAccessibilityDowngradeTrigger::ClaimOverstated,
    ]
}

fn rendering_surfaces() -> Vec<PackageComponentRenderingSurface> {
    PackageComponentRenderingSurface::ALL.to_vec()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_PACKAGE_COMPONENT_ACCESSIBILITY_SCHEMA_REF.to_owned(),
        M5_PACKAGE_COMPONENT_ACCESSIBILITY_DOC_REF.to_owned(),
        M5_PACKAGE_COMPONENT_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        M5_PACKAGE_COMPONENT_ACCESSIBILITY_CONSUMER_CONTRACT_REF.to_owned(),
        M5_PACKAGE_COMPONENT_ACCESSIBILITY_EXPLORER_ROW_CONTRACT_REF.to_owned(),
        M5_PACKAGE_COMPONENT_ACCESSIBILITY_MANIFEST_SCOPE_REGISTRY_CONTRACT_REF.to_owned(),
        M5_PACKAGE_COMPONENT_ACCESSIBILITY_INSTALL_REVIEW_LOCKFILE_CONTRACT_REF.to_owned(),
        M5_PACKAGE_COMPONENT_ACCESSIBILITY_SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_CONTRACT_REF
            .to_owned(),
    ]
}

fn row_refs(component: M5PackageComponent) -> Vec<String> {
    vec![
        M5_PACKAGE_COMPONENT_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        component_canonical_schema_ref(component).to_owned(),
    ]
}

fn human_component(component: M5PackageComponent) -> &'static str {
    match component {
        M5PackageComponent::PackageExplorerRow => "Package explorer row",
        M5PackageComponent::ManifestScopeSwitcher => "Manifest-scope switcher",
        M5PackageComponent::InstallReviewSheet => "Install-review sheet",
        M5PackageComponent::RegistryOrMirrorRow => "Registry or mirror row",
        M5PackageComponent::ScriptRiskNotice => "Script-risk notice",
        M5PackageComponent::LockfileImpactCard => "Lockfile-impact card",
        M5PackageComponent::GroupedUpdatePlanner => "Grouped-update planner",
        M5PackageComponent::RollbackCheckpointStrip => "Rollback/checkpoint strip",
    }
}

fn claim_phrase(tier: PackageComponentClaimTier) -> &'static str {
    match tier {
        PackageComponentClaimTier::FullReviewableManagement => {
            "full first-class integrated management with safe reviewable mutation"
        }
        PackageComponentClaimTier::ManifestRangeScoped => {
            "scoped to the manifest range only; the exact scope is not pinned"
        }
        PackageComponentClaimTier::MirrorOrOfflineSourced => {
            "mirror- or offline-sourced rather than upstream-fresh"
        }
        PackageComponentClaimTier::AuthRequiredReadOnly => {
            "read-only; registry authentication is required and not satisfied"
        }
        PackageComponentClaimTier::LockfileImpactUnknown => {
            "with lockfile churn unquantified; no safe mutation is claimed"
        }
        PackageComponentClaimTier::RollbackUnavailableManualRecovery => {
            "with no durable checkpoint; only manual recovery is available"
        }
    }
}

fn condition_phrase(condition: PackageComponentClaimCondition) -> &'static str {
    match condition {
        PackageComponentClaimCondition::PackageTruthTrusted => {
            "manifest scope, registry freshness, auth state, lockfile impact, and rollback are all trusted"
        }
        PackageComponentClaimCondition::ManifestScopePartial => {
            "the target manifest scope is only partially known"
        }
        PackageComponentClaimCondition::RegistryFreshnessStale => {
            "registry freshness is stale and the answer is mirror- or offline-sourced"
        }
        PackageComponentClaimCondition::AuthStateUnsatisfied => {
            "registry authentication is required and not satisfied"
        }
        PackageComponentClaimCondition::LockfileImpactUnavailable => {
            "lockfile-impact computation is unavailable or stale"
        }
        PackageComponentClaimCondition::RollbackCheckpointUnavailable => {
            "the rollback / checkpoint truth is unavailable or policy-blocked"
        }
    }
}

fn next_action_label(action: PackageComponentClaimNextAction) -> String {
    match action {
        PackageComponentClaimNextAction::ContinueReviewedManagement => {
            "Continue the reviewed package-management flow".to_owned()
        }
        PackageComponentClaimNextAction::ConfirmManifestScope => {
            "Confirm the exact target manifest scope before mutating".to_owned()
        }
        PackageComponentClaimNextAction::ReviewMirrorOrOfflineContinuity => {
            "Review the mirror / offline continuity before relying on the answer".to_owned()
        }
        PackageComponentClaimNextAction::AuthenticateToRegistry => {
            "Authenticate to the registry before completing the operation".to_owned()
        }
        PackageComponentClaimNextAction::RecomputeLockfileImpact => {
            "Recompute the lockfile impact before claiming a safe mutation".to_owned()
        }
        PackageComponentClaimNextAction::EstablishRollbackCheckpoint => {
            "Establish a durable rollback checkpoint before any write".to_owned()
        }
    }
}

/// Builds one accessibility row, deriving the claim, narrowing, notes, and labels
/// from the component and condition so the fixture stays self-consistent.
fn row(
    row_id: &str,
    component: M5PackageComponent,
    condition: PackageComponentClaimCondition,
) -> PackageComponentAccessibilityRow {
    let resolution = resolve_package_component_claim_narrowing(condition);
    let effective_claim = resolution.permitted_ceiling;

    let narrowing = if resolution.requires_narrowing {
        Some(PackageComponentClaimNarrowing {
            trigger: resolution
                .expected_trigger
                .expect("weakening condition has a trigger"),
            narrowed_to: resolution.permitted_ceiling,
            preserved_truth_note: format!(
                "{} stays keyboard-reachable, screen-reader labelled, and export-legible; only the reviewable-management claim is narrowed",
                human_component(component)
            ),
            next_action: resolution.expected_next_action,
            next_action_label: next_action_label(resolution.expected_next_action),
        })
    } else {
        None
    };

    let scope_disclosure_note = if resolution.needs_scope_disclosure_note {
        format!(
            "The target manifest scope and any script/native-build side-effect class for the {} stay explicit here",
            human_component(component).to_lowercase()
        )
    } else {
        String::new()
    };
    let continuity_note = if resolution.needs_continuity_note {
        "This answer is mirror- or offline-sourced; the mirror/offline continuity stays labelled and is never presented as upstream-fresh"
            .to_owned()
    } else {
        String::new()
    };
    let auth_note = if resolution.needs_auth_note {
        "Registry authentication is required and not satisfied; the operation stays read-only until sign-in"
            .to_owned()
    } else {
        String::new()
    };
    let rollback_note = if resolution.needs_rollback_note {
        "No durable rollback checkpoint is available; the recovery posture stays explicit before any write"
            .to_owned()
    } else {
        String::new()
    };

    PackageComponentAccessibilityRow {
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
        scope_disclosure_note,
        continuity_note,
        auth_note,
        rollback_note,
        is_pointer_only: false,
        is_export_opaque: false,
        desktop_stronger_than_cli: false,
        source_contract_refs: row_refs(component),
    }
}

/// The canonical row set: all eight components, covering all six conditions and all
/// six claim tiers.
fn accessibility_rows() -> Vec<PackageComponentAccessibilityRow> {
    vec![
        row(
            "row:package-explorer-trusted",
            M5PackageComponent::PackageExplorerRow,
            PackageComponentClaimCondition::PackageTruthTrusted,
        ),
        row(
            "row:manifest-scope-switcher-scope-partial",
            M5PackageComponent::ManifestScopeSwitcher,
            PackageComponentClaimCondition::ManifestScopePartial,
        ),
        row(
            "row:install-review-lockfile-unavailable",
            M5PackageComponent::InstallReviewSheet,
            PackageComponentClaimCondition::LockfileImpactUnavailable,
        ),
        row(
            "row:registry-or-mirror-freshness-stale",
            M5PackageComponent::RegistryOrMirrorRow,
            PackageComponentClaimCondition::RegistryFreshnessStale,
        ),
        row(
            "row:script-risk-notice-trusted",
            M5PackageComponent::ScriptRiskNotice,
            PackageComponentClaimCondition::PackageTruthTrusted,
        ),
        row(
            "row:lockfile-impact-card-rollback-unavailable",
            M5PackageComponent::LockfileImpactCard,
            PackageComponentClaimCondition::RollbackCheckpointUnavailable,
        ),
        row(
            "row:grouped-update-planner-auth-unsatisfied",
            M5PackageComponent::GroupedUpdatePlanner,
            PackageComponentClaimCondition::AuthStateUnsatisfied,
        ),
        row(
            "row:rollback-checkpoint-strip-rollback-unavailable",
            M5PackageComponent::RollbackCheckpointStrip,
            PackageComponentClaimCondition::RollbackCheckpointUnavailable,
        ),
    ]
}

fn packet_with(rows: Vec<PackageComponentAccessibilityRow>) -> PackageComponentAccessibilityPacket {
    PackageComponentAccessibilityPacket::new(PackageComponentAccessibilityPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Package-management component accessibility, headless, and export parity"
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

fn packet() -> PackageComponentAccessibilityPacket {
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
    let trusted = resolve_package_component_claim_narrowing(
        PackageComponentClaimCondition::PackageTruthTrusted,
    );
    assert_eq!(
        trusted.permitted_ceiling,
        PackageComponentClaimTier::FullReviewableManagement
    );
    assert!(!trusted.requires_narrowing);
    assert!(trusted.expected_trigger.is_none());
    assert!(!trusted.needs_scope_disclosure_note);
    assert!(!trusted.needs_continuity_note);
    assert!(!trusted.needs_auth_note);
    assert!(!trusted.needs_rollback_note);

    let manifest = resolve_package_component_claim_narrowing(
        PackageComponentClaimCondition::ManifestScopePartial,
    );
    assert_eq!(
        manifest.permitted_ceiling,
        PackageComponentClaimTier::ManifestRangeScoped
    );
    assert!(manifest.requires_narrowing);
    assert!(manifest.needs_scope_disclosure_note);
    assert!(!manifest.needs_continuity_note);
    assert!(!manifest.needs_auth_note);
    assert!(!manifest.needs_rollback_note);

    let registry = resolve_package_component_claim_narrowing(
        PackageComponentClaimCondition::RegistryFreshnessStale,
    );
    assert_eq!(
        registry.permitted_ceiling,
        PackageComponentClaimTier::MirrorOrOfflineSourced
    );
    assert_eq!(
        registry.expected_trigger,
        Some(PackageComponentAccessibilityDowngradeTrigger::RegistryFreshnessStale)
    );
    assert!(registry.needs_continuity_note);
    assert!(registry.needs_scope_disclosure_note);
    assert!(!registry.needs_auth_note);

    let auth = resolve_package_component_claim_narrowing(
        PackageComponentClaimCondition::AuthStateUnsatisfied,
    );
    assert_eq!(
        auth.permitted_ceiling,
        PackageComponentClaimTier::AuthRequiredReadOnly
    );
    assert!(auth.needs_auth_note);
    assert!(auth.needs_scope_disclosure_note);
    assert!(!auth.needs_continuity_note);
    assert!(!auth.needs_rollback_note);

    let lockfile = resolve_package_component_claim_narrowing(
        PackageComponentClaimCondition::LockfileImpactUnavailable,
    );
    assert_eq!(
        lockfile.permitted_ceiling,
        PackageComponentClaimTier::LockfileImpactUnknown
    );
    assert!(lockfile.needs_scope_disclosure_note);
    assert!(!lockfile.needs_rollback_note);

    let rollback = resolve_package_component_claim_narrowing(
        PackageComponentClaimCondition::RollbackCheckpointUnavailable,
    );
    assert_eq!(
        rollback.permitted_ceiling,
        PackageComponentClaimTier::RollbackUnavailableManualRecovery
    );
    assert!(rollback.needs_rollback_note);
    assert!(rollback.needs_scope_disclosure_note);
    assert!(!rollback.needs_auth_note);
}

// --- AC2: narrowing prevents overstated package-management scope ---------------

#[test]
fn full_management_claim_never_survives_a_weakening_condition() {
    // A component that keeps asserting full reviewable management while the manifest
    // scope is only partially known overstates its truth and must be caught.
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == PackageComponentClaimCondition::ManifestScopePartial)
        .expect("manifest-scope-partial row present");
    packet.accessibility_rows[index].effective_claim =
        PackageComponentClaimTier::FullReviewableManagement;
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::ClaimCeilingExceeded));
}

#[test]
fn claim_ceiling_exceeded_on_registry_stale_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == PackageComponentClaimCondition::RegistryFreshnessStale)
        .expect("registry-stale row present");
    // Claim manifest-range-scoped (rank 5) above the mirror/offline ceiling (rank 4).
    packet.accessibility_rows[index].effective_claim =
        PackageComponentClaimTier::ManifestRangeScoped;
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::ClaimCeilingExceeded));
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
        .contains(&PackageComponentAccessibilityViolation::ClaimNarrowingMissing));
}

#[test]
fn baseline_condition_with_narrowing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == PackageComponentClaimCondition::PackageTruthTrusted)
        .expect("trusted row present");
    packet.accessibility_rows[index].narrowing = Some(PackageComponentClaimNarrowing {
        trigger: PackageComponentAccessibilityDowngradeTrigger::ManifestScopePartial,
        narrowed_to: PackageComponentClaimTier::FullReviewableManagement,
        preserved_truth_note: "note".to_owned(),
        next_action: PackageComponentClaimNextAction::ContinueReviewedManagement,
        next_action_label: "Continue".to_owned(),
    });
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::ClaimNarrowingUnexpected));
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
        narrowing.narrowed_to = PackageComponentClaimTier::RollbackUnavailableManualRecovery;
    }
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::NarrowedToMismatch));
}

#[test]
fn narrow_trigger_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == PackageComponentClaimCondition::ManifestScopePartial)
        .expect("manifest-scope-partial row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.trigger = PackageComponentAccessibilityDowngradeTrigger::RegistryFreshnessStale;
    }
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::NarrowTriggerMismatch));
}

#[test]
fn narrow_next_action_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == PackageComponentClaimCondition::AuthStateUnsatisfied)
        .expect("auth row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.next_action = PackageComponentClaimNextAction::ContinueReviewedManagement;
    }
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::NarrowNextActionMismatch));
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
        .contains(&PackageComponentAccessibilityViolation::NarrowPreservedTruthMissing));
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
        .contains(&PackageComponentAccessibilityViolation::NarrowNextActionMissing));
}

// --- AC1: parity across keyboard / screen-reader / CLI / export ---------------

#[test]
fn keyboard_label_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].keyboard_label = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::KeyboardLabelMissing));
}

#[test]
fn screen_reader_label_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].screen_reader_label = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::ScreenReaderLabelMissing));
}

#[test]
fn cli_enum_token_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].cli_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::CliEnumTokenMissing));
}

#[test]
fn export_enum_token_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].export_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::ExportEnumTokenMissing));
}

#[test]
fn explanation_field_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].explanation_field = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::ExplanationFieldMissing));
}

#[test]
fn rendering_surface_coverage_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].rendering_surfaces =
        vec![PackageComponentRenderingSurface::DesktopFull];
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::RenderingSurfaceCoverageMissing));
}

#[test]
fn pointer_only_component_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].is_pointer_only = true;
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::PointerOnlyComponent));
}

#[test]
fn export_opaque_component_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].is_export_opaque = true;
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::ExportOpaqueComponent));
}

#[test]
fn desktop_stronger_than_cli_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].desktop_stronger_than_cli = true;
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::DesktopStrongerThanCli));
}

#[test]
fn scope_disclosure_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    packet.accessibility_rows[index].scope_disclosure_note = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::ScopeDisclosureNoteMissing));
}

#[test]
fn continuity_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == PackageComponentClaimCondition::RegistryFreshnessStale)
        .expect("registry-stale row present");
    packet.accessibility_rows[index].continuity_note = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::ContinuityNoteMissing));
}

#[test]
fn auth_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == PackageComponentClaimCondition::AuthStateUnsatisfied)
        .expect("auth row present");
    packet.accessibility_rows[index].auth_note = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::AuthNoteMissing));
}

#[test]
fn rollback_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == PackageComponentClaimCondition::RollbackCheckpointUnavailable)
        .expect("rollback row present");
    packet.accessibility_rows[index].rollback_note = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::RollbackNoteMissing));
}

#[test]
fn canonical_contract_reference_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].source_contract_refs =
        vec![M5_PACKAGE_COMPONENT_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::CanonicalContractReferenceMissing));
}

// --- Coverage -----------------------------------------------------------------

#[test]
fn missing_component_coverage_fails() {
    let mut rows = accessibility_rows();
    // Drop the rollback-checkpoint-strip; the rollback condition and tier stay covered
    // by the lockfile-impact-card row, isolating the component-coverage failure.
    rows.retain(|r| r.component != M5PackageComponent::RollbackCheckpointStrip);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::ComponentCoverageMissing));
}

#[test]
fn missing_condition_coverage_fails() {
    let mut rows = accessibility_rows();
    // Drop the only manifest-scope-partial row.
    rows.retain(|r| r.condition != PackageComponentClaimCondition::ManifestScopePartial);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::ConditionCoverageMissing));
}

#[test]
fn missing_claim_tier_coverage_fails() {
    let mut rows = accessibility_rows();
    // Drop the only manifest-range-scoped row; that tier is then unreachable.
    rows.retain(|r| r.effective_claim != PackageComponentClaimTier::ManifestRangeScoped);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::ClaimTierCoverageMissing));
}

// --- Structural ---------------------------------------------------------------

#[test]
fn row_incomplete_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].row_id = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::RowIncomplete));
}

#[test]
fn missing_rows_fails() {
    let mut packet = packet();
    packet.accessibility_rows.clear();
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::AccessibilityRowsMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .rollback_checkpoint_truth_kept_explicit_before_write = false;
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::TrustReviewIncomplete));
}

#[test]
fn projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .projection
        .narrowing_prevents_overstated_package_management_scope = false;
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::ProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&PackageComponentAccessibilityViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Accessibility rows"));
    assert!(summary.contains("package_explorer_row"));
    assert!(summary.contains("rollback_checkpoint_strip"));
    assert!(summary.contains("mirror_or_offline_sourced"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_package_component_accessibility_export()
        .expect("checked package-management component accessibility export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-package-management-component-accessibility-parity/manifest_scope_and_registry_freshness_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-package-management-component-accessibility-parity/auth_state_and_rollback_checkpoint_narrowed.json"
        )),
    ] {
        let packet: PackageComponentAccessibilityPacket = serde_json::from_str(raw)
            .expect("fixture parses as package-management component accessibility packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

// --- Fixture builders ---------------------------------------------------------

/// The canonical rows plus extra scenario rows that demonstrate normally-trusted
/// components auto-narrowing under a partially-known manifest scope and stale registry
/// freshness. The base rows keep full component / condition / tier coverage; the extra
/// rows show the narrowing.
fn fixture_manifest_scope_and_registry_freshness_narrowed() -> PackageComponentAccessibilityPacket {
    let mut rows = accessibility_rows();
    rows.push(row(
        "row:package-explorer-scope-narrowed",
        M5PackageComponent::PackageExplorerRow,
        PackageComponentClaimCondition::ManifestScopePartial,
    ));
    rows.push(row(
        "row:script-risk-notice-registry-narrowed",
        M5PackageComponent::ScriptRiskNotice,
        PackageComponentClaimCondition::RegistryFreshnessStale,
    ));
    PackageComponentAccessibilityPacket::new(PackageComponentAccessibilityPacketInput {
        packet_id: "package-management-component-accessibility:fixture:manifest-scope-and-registry-freshness-narrowed"
            .to_owned(),
        surface_label:
            "Package-management component accessibility: manifest scope partial and registry freshness stale, claim auto-narrowed"
                .to_owned(),
        accessibility_rows: rows,
        downgrade_triggers: vec![
            PackageComponentAccessibilityDowngradeTrigger::ManifestScopePartial,
            PackageComponentAccessibilityDowngradeTrigger::RegistryFreshnessStale,
            PackageComponentAccessibilityDowngradeTrigger::ClaimOverstated,
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

/// The canonical rows plus extra scenario rows for a package-explorer row losing auth
/// state and a manifest-scope switcher losing its rollback / checkpoint truth.
fn fixture_auth_state_and_rollback_checkpoint_narrowed() -> PackageComponentAccessibilityPacket {
    let mut rows = accessibility_rows();
    rows.push(row(
        "row:package-explorer-auth-narrowed",
        M5PackageComponent::PackageExplorerRow,
        PackageComponentClaimCondition::AuthStateUnsatisfied,
    ));
    rows.push(row(
        "row:manifest-scope-switcher-rollback-narrowed",
        M5PackageComponent::ManifestScopeSwitcher,
        PackageComponentClaimCondition::RollbackCheckpointUnavailable,
    ));
    PackageComponentAccessibilityPacket::new(PackageComponentAccessibilityPacketInput {
        packet_id:
            "package-management-component-accessibility:fixture:auth-state-and-rollback-checkpoint-narrowed"
                .to_owned(),
        surface_label:
            "Package-management component accessibility: auth state unsatisfied and rollback checkpoint unavailable"
                .to_owned(),
        accessibility_rows: rows,
        downgrade_triggers: vec![
            PackageComponentAccessibilityDowngradeTrigger::AuthStateUnsatisfied,
            PackageComponentAccessibilityDowngradeTrigger::RollbackCheckpointUnavailable,
            PackageComponentAccessibilityDowngradeTrigger::ClaimOverstated,
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
/// Gated behind `GEN_PACKAGE_COMPONENT_ACCESSIBILITY_ARTIFACTS` so it never writes
/// during a normal test run. Run with the env var set to refresh the artifacts after a
/// contract change, then review the diff.
#[test]
fn regenerate_package_component_accessibility_artifacts() {
    if std::env::var("GEN_PACKAGE_COMPONENT_ACCESSIBILITY_ARTIFACTS").is_err() {
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
        format!("{root}/artifacts/release/m5-package-management-accessibility-proof");
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
        format!("{root}/fixtures/ui/m5-package-management-component-accessibility-parity");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    for (name, fixture) in [
        (
            "manifest_scope_and_registry_freshness_narrowed.json",
            fixture_manifest_scope_and_registry_freshness_narrowed(),
        ),
        (
            "auth_state_and_rollback_checkpoint_narrowed.json",
            fixture_auth_state_and_rollback_checkpoint_narrowed(),
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
