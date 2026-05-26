//! Fixture generator helper for the restricted-mode UX lineage replay
//! gate.
//!
//! Only runs when `RESTRICTED_MODE_UX_LINEAGE_GEN_FIXTURES=1` is set in
//! the environment. Emits the canonical fixture JSON files into
//! `fixtures/workspace/m4/restricted_mode_ux_lineage/` so the replay
//! gate has a deterministic, checked-in corpus.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    default_restricted_mode_ux_inspection_hooks, project_restricted_mode_ux_lineage_with_hooks,
    AccessibilityPostureClass, ClaimedStableTier, EscapePathClass, RestrictedAffordanceClass,
    RestrictedModeInspectionHook, RestrictedModeInspectionHookClass, RestrictedModePosture,
    RestrictedModeSupportExportInputs, RestrictedModeSupportExportPosture,
    RestrictedModeSurfaceKind, RestrictedModeSurfaceObservation, RestrictedModeUxInputs,
    RestrictedModeUxLineageRecord, RestrictionReasonClass, REQUIRED_ACCESSIBILITY_POSTURES,
};
use serde::Serialize;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m4/restricted_mode_ux_lineage")
}

fn all_accessibility_postures() -> Vec<AccessibilityPostureClass> {
    REQUIRED_ACCESSIBILITY_POSTURES.to_vec()
}

#[allow(clippy::too_many_arguments)]
fn make_surface(
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
    captured_at: &str,
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
        support_export: RestrictedModeSupportExportInputs::metadata_safe_baseline(
            RestrictedModeSupportExportPosture::MetadataSafeExport,
        ),
        captured_at: captured_at.to_owned(),
    }
}

fn restricted_corpus(captured_at: &str) -> Vec<RestrictedModeSurfaceObservation> {
    vec![
        make_surface(
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
            captured_at,
        ),
        make_surface(
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
            captured_at,
        ),
        make_surface(
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
            captured_at,
        ),
        make_surface(
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
            captured_at,
        ),
        make_surface(
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
            captured_at,
        ),
        make_surface(
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
            captured_at,
        ),
    ]
}

fn pending_corpus(captured_at: &str) -> Vec<RestrictedModeSurfaceObservation> {
    let mut surfaces = restricted_corpus(captured_at);
    for surface in &mut surfaces {
        if surface.restriction_reason == RestrictionReasonClass::WorkspaceRestricted {
            surface.restriction_reason = RestrictionReasonClass::WorkspacePendingEvaluation;
            // explanation ids carry the new reason class as the trailing
            // tag so the support export keeps the reason explicit.
            surface.explanation_id = surface
                .explanation_id
                .replace("workspace_restricted", "workspace_pending_evaluation");
        }
    }
    surfaces
}

fn trusted_corpus(captured_at: &str) -> Vec<RestrictedModeSurfaceObservation> {
    let mut surfaces = restricted_corpus(captured_at);
    for surface in &mut surfaces {
        surface.claimed_tier = ClaimedStableTier::StableFull;
    }
    surfaces
}

fn base_inputs(
    workspace_ref: &str,
    corpus_ref: &str,
    captured_at: &str,
    posture: RestrictedModePosture,
    surfaces: Vec<RestrictedModeSurfaceObservation>,
) -> RestrictedModeUxInputs {
    RestrictedModeUxInputs {
        workspace_ref: workspace_ref.to_owned(),
        producer_ref: "producer-aureline-fixtures-0001".to_owned(),
        corpus_ref: corpus_ref.to_owned(),
        captured_at: captured_at.to_owned(),
        posture,
        surfaces,
    }
}

#[derive(Debug, Serialize)]
struct FixtureEnvelope<'a> {
    posture_id: &'a str,
    inputs: &'a RestrictedModeUxInputs,
    inspection_hooks: &'a Vec<RestrictedModeInspectionHook>,
    expected: &'a RestrictedModeUxLineageRecord,
}

fn write_fixture(
    name: &str,
    posture_id: &str,
    inputs: RestrictedModeUxInputs,
    inspection_hooks: Vec<RestrictedModeInspectionHook>,
) {
    let record = project_restricted_mode_ux_lineage_with_hooks(
        posture_id,
        &inputs,
        inspection_hooks.clone(),
    );
    let envelope = FixtureEnvelope {
        posture_id,
        inputs: &inputs,
        inspection_hooks: &inspection_hooks,
        expected: &record,
    };
    let path = fixtures_dir().join(format!("{name}.json"));
    let json = serde_json::to_string_pretty(&envelope).expect("envelope serializes");
    std::fs::write(&path, json + "\n").expect("fixture write");
    eprintln!("wrote {}", path.display());
}

#[test]
fn generate_fixtures() {
    if std::env::var("RESTRICTED_MODE_UX_LINEAGE_GEN_FIXTURES")
        .ok()
        .as_deref()
        != Some("1")
    {
        return;
    }
    std::fs::create_dir_all(fixtures_dir()).expect("ensure fixture dir");

    // Restricted workspace: every restricted-mode UX surface ships a
    // read-only-tier claim with a disclosed escape path.
    let restricted_inputs = base_inputs(
        "workspace-rust-service-0001",
        "restricted-mode-ux-corpus-restricted-0001",
        "mono:1700000300",
        RestrictedModePosture::Restricted,
        restricted_corpus("mono:1700000300"),
    );
    write_fixture(
        "restricted_read_only_stable",
        "posture:restricted_read_only",
        restricted_inputs,
        default_restricted_mode_ux_inspection_hooks(),
    );

    // Pending workspace: every restricted-mode UX surface ships a
    // read-only-tier claim with a pending-evaluation reason.
    let pending_inputs = base_inputs(
        "workspace-rust-service-0001",
        "restricted-mode-ux-corpus-pending-0001",
        "mono:1700000310",
        RestrictedModePosture::PendingEvaluation,
        pending_corpus("mono:1700000310"),
    );
    write_fixture(
        "pending_read_only_stable",
        "posture:pending_read_only",
        pending_inputs,
        default_restricted_mode_ux_inspection_hooks(),
    );

    // Trusted workspace: every restricted-mode UX surface ships a
    // dormant `stable_full` claim. The restricted-mode chrome is
    // present but inactive.
    let trusted_inputs = base_inputs(
        "workspace-rust-service-0001",
        "restricted-mode-ux-corpus-trusted-0001",
        "mono:1700000320",
        RestrictedModePosture::Trusted,
        trusted_corpus("mono:1700000320"),
    );
    write_fixture(
        "trusted_stable_full",
        "posture:trusted_stable_full",
        trusted_inputs,
        default_restricted_mode_ux_inspection_hooks(),
    );

    // Narrowed: same restricted corpus but the review-escape-path hook
    // is unavailable.
    let narrowed_inputs = base_inputs(
        "workspace-rust-service-0001",
        "restricted-mode-ux-corpus-narrowed-0001",
        "mono:1700000330",
        RestrictedModePosture::Restricted,
        restricted_corpus("mono:1700000330"),
    );
    let mut narrowed_hooks = default_restricted_mode_ux_inspection_hooks();
    for hook in &mut narrowed_hooks {
        if hook.hook_class == RestrictedModeInspectionHookClass::ReviewEscapePath {
            hook.available = false;
            hook.disclosure = "Review-escape-path unavailable on this posture.".to_owned();
        }
    }
    write_fixture(
        "missing_review_escape_path_hook_narrowed",
        "posture:missing_review_escape_path_hook",
        narrowed_inputs,
        narrowed_hooks,
    );
}
