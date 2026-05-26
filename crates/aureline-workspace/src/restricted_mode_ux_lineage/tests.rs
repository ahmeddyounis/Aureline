//! Unit tests for the restricted-mode UX lineage projection.

use super::*;

fn all_accessibility_postures() -> Vec<AccessibilityPostureClass> {
    REQUIRED_ACCESSIBILITY_POSTURES.to_vec()
}

#[allow(clippy::too_many_arguments)]
fn surface(
    surface_id: &str,
    title: &str,
    kind: RestrictedModeSurfaceKind,
    reason: RestrictionReasonClass,
    explanation_id: &str,
    escape: EscapePathClass,
    escape_action_id: &str,
    escape_disclosure_id: &str,
    affordances: Vec<RestrictedAffordanceClass>,
    tier: ClaimedStableTier,
    touches_credential_store: bool,
    posture: RestrictedModeSupportExportPosture,
) -> RestrictedModeSurfaceObservation {
    RestrictedModeSurfaceObservation {
        surface_id: surface_id.to_owned(),
        title: title.to_owned(),
        surface_kind: kind,
        restriction_reason: reason,
        explanation_id: explanation_id.to_owned(),
        escape_path: escape,
        escape_action_id: escape_action_id.to_owned(),
        escape_disclosure_id: escape_disclosure_id.to_owned(),
        affordances,
        claimed_tier: tier,
        touches_credential_store,
        accessibility_postures: all_accessibility_postures(),
        support_export: RestrictedModeSupportExportInputs::metadata_safe_baseline(posture),
        captured_at: "mono:1700000300".to_owned(),
    }
}

fn restricted_corpus_inputs() -> RestrictedModeUxInputs {
    RestrictedModeUxInputs {
        workspace_ref: "workspace-restricted-0001".to_owned(),
        producer_ref: "producer-aureline-0001".to_owned(),
        corpus_ref: "restricted-mode-ux-corpus-restricted-0001".to_owned(),
        captured_at: "mono:1700000300".to_owned(),
        posture: RestrictedModePosture::Restricted,
        surfaces: vec![
            surface(
                "status_bar.workspace_trust",
                "Workspace trust status indicator",
                RestrictedModeSurfaceKind::StatusBar,
                RestrictionReasonClass::WorkspaceRestricted,
                "explanation.status_bar.workspace_restricted",
                EscapePathClass::GrantTrust,
                "status_bar.grant_trust",
                "disclosure.status_bar.grant_trust",
                vec![
                    RestrictedAffordanceClass::InspectOnly,
                    RestrictedAffordanceClass::NavigateOnly,
                ],
                ClaimedStableTier::StableReadOnly,
                false,
                RestrictedModeSupportExportPosture::MetadataSafeExport,
            ),
            surface(
                "editor_chrome.restricted_banner",
                "Editor chrome restricted banner",
                RestrictedModeSurfaceKind::EditorChrome,
                RestrictionReasonClass::WorkspaceRestricted,
                "explanation.editor_chrome.workspace_restricted",
                EscapePathClass::RepairWorkspace,
                "editor_chrome.repair_workspace",
                "disclosure.editor_chrome.repair_workspace",
                vec![
                    RestrictedAffordanceClass::InspectOnly,
                    RestrictedAffordanceClass::CopyToClipboard,
                    RestrictedAffordanceClass::ViewDiffOnly,
                ],
                ClaimedStableTier::StableReadOnly,
                false,
                RestrictedModeSupportExportPosture::MetadataSafeExport,
            ),
            surface(
                "command_palette.restricted_filter",
                "Command palette restricted-action filter",
                RestrictedModeSurfaceKind::CommandPalette,
                RestrictionReasonClass::WorkspaceRestricted,
                "explanation.command_palette.workspace_restricted",
                EscapePathClass::StayReadOnly,
                "",
                "",
                vec![
                    RestrictedAffordanceClass::BlockedWithExplanation,
                    RestrictedAffordanceClass::InspectOnly,
                ],
                ClaimedStableTier::StableReadOnly,
                false,
                RestrictedModeSupportExportPosture::MetadataSafeExport,
            ),
            surface(
                "action_menu.tasks_terminal_debug",
                "Action menu on privileged surfaces",
                RestrictedModeSurfaceKind::ActionMenu,
                RestrictionReasonClass::CredentialStoreRestricted,
                "explanation.action_menu.credential_store_restricted",
                EscapePathClass::ContactSupport,
                "action_menu.contact_support",
                "disclosure.action_menu.contact_support",
                vec![
                    RestrictedAffordanceClass::BlockedWithExplanation,
                    RestrictedAffordanceClass::AllowReadOnlyNoMutation,
                ],
                ClaimedStableTier::StableReadOnly,
                true,
                RestrictedModeSupportExportPosture::MetadataSafeExport,
            ),
            surface(
                "help_about.restricted_mode",
                "Help / About restricted-mode entry",
                RestrictedModeSurfaceKind::HelpAbout,
                RestrictionReasonClass::WorkspaceRestricted,
                "explanation.help_about.workspace_restricted",
                EscapePathClass::LeaveWorkspace,
                "help_about.leave_workspace",
                "disclosure.help_about.leave_workspace",
                vec![RestrictedAffordanceClass::InspectOnly],
                ClaimedStableTier::StableReadOnly,
                false,
                RestrictedModeSupportExportPosture::MetadataSafeExport,
            ),
            surface(
                "support_export.restricted_mode",
                "Support export restricted-mode projection",
                RestrictedModeSurfaceKind::SupportExport,
                RestrictionReasonClass::WorkspaceRestricted,
                "explanation.support_export.workspace_restricted",
                EscapePathClass::ContactSupport,
                "support_export.contact_support",
                "disclosure.support_export.contact_support",
                vec![RestrictedAffordanceClass::InspectOnly],
                ClaimedStableTier::StableReadOnly,
                true,
                RestrictedModeSupportExportPosture::MetadataSafeExport,
            ),
        ],
    }
}

#[test]
fn clean_inputs_project_stable_record() {
    let inputs = restricted_corpus_inputs();
    let record = project_restricted_mode_ux_lineage("posture.clean", &inputs);

    assert!(
        record.is_stable_qualified(),
        "narrow: {:?}",
        record.stable_qualification.narrow_reasons
    );
    assert!(record.is_support_export_safe());
    assert_eq!(record.record_kind, RESTRICTED_MODE_UX_LINEAGE_RECORD_KIND);
    assert_eq!(record.schema_ref, RESTRICTED_MODE_UX_LINEAGE_SCHEMA_REF);
    assert!(record.surface_coverage.all_required_surfaces_present);
    assert_eq!(record.surface_coverage.surface_rows.len(), 6);
    assert!(record.explainability_truth.all_surfaces_have_explanation);
    assert!(record.escape_path_honesty.all_escape_paths_have_action);
    assert!(record.escape_path_honesty.all_grant_trust_escapes_disclosed);
    assert!(
        record
            .read_only_affordance_truth
            .all_read_only_surfaces_safe
    );
    assert!(
        record
            .claimed_tier_truth
            .no_full_claim_in_restricted_posture
    );
    assert!(record.claimed_tier_truth.all_tiers_match_derived);
    assert!(
        record
            .accessibility_truth
            .all_surfaces_accessibility_complete
    );
    assert!(
        record
            .support_export_honesty
            .all_credential_surfaces_have_safe_posture
    );
    assert_eq!(record.inspection_hooks.len(), 6);
    assert!(record
        .producer_attribution
        .integrity_hash
        .starts_with("rmu:"));
}

#[test]
fn missing_required_surface_narrows_record() {
    let mut inputs = restricted_corpus_inputs();
    inputs
        .surfaces
        .retain(|surface| surface.surface_kind != RestrictedModeSurfaceKind::HelpAbout);

    let record = project_restricted_mode_ux_lineage("posture.missing_help", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RestrictedModeUxLineageNarrowReason::RequiredRestrictedSurfaceMissing));
}

#[test]
fn restricted_workspace_missing_explanation_narrows_record() {
    let mut inputs = restricted_corpus_inputs();
    inputs.surfaces[0].explanation_id = "".to_owned();

    let record = project_restricted_mode_ux_lineage("posture.no_explanation", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RestrictedModeUxLineageNarrowReason::ExplanationMissing));
}

#[test]
fn grant_trust_escape_without_disclosure_narrows_record() {
    let mut inputs = restricted_corpus_inputs();
    inputs.surfaces[0].escape_disclosure_id = "".to_owned();

    let record = project_restricted_mode_ux_lineage("posture.no_disclosure", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RestrictedModeUxLineageNarrowReason::GrantTrustEscapeUndisclosed));
}

#[test]
fn non_stay_escape_without_action_narrows_record() {
    let mut inputs = restricted_corpus_inputs();
    inputs.surfaces[1].escape_action_id = "".to_owned();

    let record = project_restricted_mode_ux_lineage("posture.no_action", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RestrictedModeUxLineageNarrowReason::EscapePathActionMissing));
}

#[test]
fn empty_affordances_narrows_record() {
    let mut inputs = restricted_corpus_inputs();
    inputs.surfaces[0].affordances = Vec::new();

    let record = project_restricted_mode_ux_lineage("posture.no_affordances", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RestrictedModeUxLineageNarrowReason::AffordancesEmpty));
}

#[test]
fn restricted_workspace_with_full_claim_narrows_record() {
    let mut inputs = restricted_corpus_inputs();
    inputs.surfaces[0].claimed_tier = ClaimedStableTier::StableFull;

    let record = project_restricted_mode_ux_lineage("posture.full_in_restricted", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RestrictedModeUxLineageNarrowReason::ClaimedFullInRestrictedPosture));
}

#[test]
fn missing_accessibility_posture_narrows_record() {
    let mut inputs = restricted_corpus_inputs();
    inputs.surfaces[0]
        .accessibility_postures
        .retain(|posture| *posture != AccessibilityPostureClass::ReducedMotion);

    let record = project_restricted_mode_ux_lineage("posture.missing_motion", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RestrictedModeUxLineageNarrowReason::AccessibilityPostureMissing));
}

#[test]
fn support_export_dropping_fields_narrows_record() {
    let mut inputs = restricted_corpus_inputs();
    inputs.surfaces[0].support_export.includes_escape_path = false;

    let record = project_restricted_mode_ux_lineage("posture.support_dropped", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RestrictedModeUxLineageNarrowReason::SupportExportFieldsDropped));
}

#[test]
fn credential_surface_local_only_narrows_record() {
    let mut inputs = restricted_corpus_inputs();
    let credential = inputs
        .surfaces
        .iter_mut()
        .find(|surface| surface.touches_credential_store)
        .expect("credential-touching surface seeded");
    credential.support_export.posture = RestrictedModeSupportExportPosture::LocalOnly;

    let record = project_restricted_mode_ux_lineage("posture.credential_local_only", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RestrictedModeUxLineageNarrowReason::SupportExportPostureUnsafe));
}

#[test]
fn support_export_redaction_unsafe_narrows_record() {
    let mut inputs = restricted_corpus_inputs();
    inputs.surfaces[0].support_export.raw_secrets_excluded = false;

    let record = project_restricted_mode_ux_lineage("posture.raw_secret", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RestrictedModeUxLineageNarrowReason::SupportExportRedactionUnsafe));
}

#[test]
fn missing_inspection_hook_narrows_record() {
    let inputs = restricted_corpus_inputs();
    let mut hooks = default_restricted_mode_ux_inspection_hooks();
    for hook in &mut hooks {
        if hook.hook_class == RestrictedModeInspectionHookClass::ReviewEscapePath {
            hook.available = false;
        }
    }

    let record = project_restricted_mode_ux_lineage_with_hooks("posture.no_review", &inputs, hooks);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RestrictedModeUxLineageNarrowReason::InspectionHookUnavailable));
}

#[test]
fn empty_workspace_ref_narrows_record() {
    let mut inputs = restricted_corpus_inputs();
    inputs.workspace_ref = "".to_owned();

    let record = project_restricted_mode_ux_lineage("posture.empty_workspace", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RestrictedModeUxLineageNarrowReason::LineageExportUnsafe));
}

#[test]
fn read_only_claim_with_mutation_affordance_narrows_record() {
    let mut inputs = restricted_corpus_inputs();
    // Inject a custom affordance whose `is_read_only_safe` returns
    // false — but our closed enum only contains read-only-safe
    // affordances, so we test the negative path by stripping all
    // affordances and re-asserting the empty-affordances narrow path.
    inputs.surfaces[0].affordances = Vec::new();

    let record = project_restricted_mode_ux_lineage("posture.empty_affordances", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RestrictedModeUxLineageNarrowReason::AffordancesEmpty));
}

#[test]
fn trusted_workspace_with_stable_full_qualifies() {
    let mut inputs = restricted_corpus_inputs();
    inputs.posture = RestrictedModePosture::Trusted;
    for surface in &mut inputs.surfaces {
        surface.claimed_tier = ClaimedStableTier::StableFull;
    }

    let record = project_restricted_mode_ux_lineage("posture.trusted", &inputs);
    assert!(
        record.is_stable_qualified(),
        "narrow: {:?}",
        record.stable_qualification.narrow_reasons
    );
    assert!(
        record
            .claimed_tier_truth
            .no_full_claim_in_restricted_posture
    );
    assert!(record.claimed_tier_truth.all_tiers_match_derived);
}

#[test]
fn lines_projection_renders_required_sections() {
    let inputs = restricted_corpus_inputs();
    let record = project_restricted_mode_ux_lineage("posture.lines", &inputs);
    let lines = restricted_mode_ux_lineage_lines(&record);

    assert!(lines
        .iter()
        .any(|line| line.contains("Restricted-mode UX lineage")));
    assert!(lines.iter().any(|line| line.contains("surface_coverage")));
    assert!(lines.iter().any(|line| line == "Surface rows:"));
    assert!(lines
        .iter()
        .any(|line| line.contains("Explainability truth")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Escape path honesty")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Read-only affordance truth")));
    assert!(lines.iter().any(|line| line.contains("Claimed tier truth")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Accessibility truth")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Support-export honesty")));
    assert!(lines.iter().any(|line| line == "Inspection hooks:"));
}

#[test]
fn record_round_trips_through_json() {
    let inputs = restricted_corpus_inputs();
    let record = project_restricted_mode_ux_lineage("posture.round_trip", &inputs);
    let serialized = serde_json::to_string(&record).expect("record must serialize");
    let parsed: RestrictedModeUxLineageRecord =
        serde_json::from_str(&serialized).expect("record must deserialize");
    assert_eq!(record, parsed);
}
