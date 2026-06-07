//! Canonical corpus for declared shell slots, fallback ladders, and placeholders.

use super::model::{
    AdaptiveClass, DeclaredSlotRecord, DependencyLossClass, FallbackPlacement, PlaceholderClass,
    ResponsiveFallbackLadder, ShellSlotId, ShellZoningAuditRecord, ShellZoningPacket,
    SlotHydrationCase, StableSurfaceClaim, SurfaceKind, SHELL_ZONING_PACKET_RECORD_KIND,
    SHELL_ZONING_SCHEMA_VERSION,
};

const CONTRACT_REF: &str = "shell:stabilize_shell_zoning_and_responsive_fallback:v1";

/// Returns the canonical declared-slot registry.
pub fn canonical_slot_registry() -> Vec<DeclaredSlotRecord> {
    use FallbackPlacement::{Docked, Overflow, Placeholder, Sheet};
    use PlaceholderClass::{
        CapabilityLoss, MissingExtension, MissingProvider, MissingRemote, RequiredChrome,
    };
    use ShellSlotId::{
        ActivityRailPrimaryRoutes, BottomPanelToolPanels, LeftSidebarSectionSurface,
        MainWorkspaceReviewSurface, MainWorkspaceWorkingSet, RightInspectorContextualDetail,
        StatusBarExtensionScoped, StatusBarRecoveryPrimary, TitleContextBarIdentity,
        TransientOverlayCommandPalette, TransientOverlayDialogOrSheet,
    };
    use SurfaceKind::{ExtensionContributed, FirstParty, HostChrome, ProviderBacked};

    vec![
        slot(
            TitleContextBarIdentity,
            "Title/context identity",
            vec![HostChrome, FirstParty],
            vec![Docked, Overflow, Placeholder],
            "cmd:shell.title_context.close_identity_overflow",
            "cmd:shell.title_context.reopen_identity",
            RequiredChrome,
            true,
            "docs/ux/shell_zone_and_density_contract.md",
        ),
        slot(
            ActivityRailPrimaryRoutes,
            "Activity rail primary routes",
            vec![HostChrome, FirstParty, ExtensionContributed],
            vec![Docked, Overflow, Placeholder],
            "cmd:shell.rail.hide_route",
            "cmd:shell.rail.reopen_route",
            MissingExtension,
            true,
            "docs/ux/rail_sidebar_contract.md",
        ),
        slot(
            LeftSidebarSectionSurface,
            "Left sidebar section surface",
            vec![FirstParty, ProviderBacked, ExtensionContributed],
            vec![Docked, Sheet, Overflow, Placeholder],
            "cmd:shell.sidebar.close_surface",
            "cmd:shell.sidebar.reopen_surface",
            MissingExtension,
            true,
            "docs/ux/rail_sidebar_contract.md",
        ),
        slot(
            MainWorkspaceWorkingSet,
            "Main workspace working set",
            vec![FirstParty, ProviderBacked, ExtensionContributed],
            vec![Docked, Overflow, Placeholder],
            "cmd:shell.workspace.close_tab",
            "cmd:shell.workspace.reopen_last",
            MissingProvider,
            true,
            "docs/ux/tabs_editor_groups_contract.md",
        ),
        slot(
            MainWorkspaceReviewSurface,
            "Main workspace review surface",
            vec![FirstParty, ProviderBacked],
            vec![Docked, Sheet, Overflow, Placeholder],
            "cmd:shell.review.close_surface",
            "cmd:shell.review.reopen_surface",
            CapabilityLoss,
            true,
            "docs/ux/review_surface_contract.md",
        ),
        slot(
            RightInspectorContextualDetail,
            "Right inspector contextual detail",
            vec![FirstParty, ProviderBacked, ExtensionContributed],
            vec![Docked, Sheet, Overflow, Placeholder],
            "cmd:shell.inspector.close_detail",
            "cmd:shell.inspector.reopen_detail",
            MissingProvider,
            true,
            "docs/ux/shell_responsive_fallback_contract.md",
        ),
        slot(
            BottomPanelToolPanels,
            "Bottom panel tool panels",
            vec![FirstParty, ProviderBacked, ExtensionContributed],
            vec![Docked, Overflow, Sheet, Placeholder],
            "cmd:shell.bottom_panel.close_tab",
            "cmd:shell.bottom_panel.reopen_tab",
            MissingRemote,
            true,
            "docs/ux/output_log_viewer_contract.md",
        ),
        slot(
            StatusBarRecoveryPrimary,
            "Status bar recovery primary",
            vec![HostChrome, FirstParty],
            vec![Docked, Overflow, Placeholder],
            "cmd:shell.status.close_recovery_overflow",
            "cmd:shell.status.reopen_recovery",
            RequiredChrome,
            true,
            "docs/ux/status_bar_contract.md",
        ),
        slot(
            StatusBarExtensionScoped,
            "Status bar extension scoped item",
            vec![FirstParty, ProviderBacked, ExtensionContributed],
            vec![Docked, Overflow, Placeholder],
            "cmd:shell.status.close_scoped_item",
            "cmd:shell.status.reopen_scoped_item",
            MissingExtension,
            true,
            "docs/ux/status_bar_contract.md",
        ),
        slot(
            TransientOverlayCommandPalette,
            "Command palette overlay",
            vec![HostChrome, FirstParty],
            vec![Docked, Overflow, Placeholder],
            "cmd:shell.palette.dismiss",
            "cmd:shell.palette.open",
            RequiredChrome,
            true,
            "docs/ux/quick_open_contract.md",
        ),
        slot(
            TransientOverlayDialogOrSheet,
            "Dialog and sheet overlay",
            vec![HostChrome, FirstParty, ProviderBacked],
            vec![Docked, Sheet, Overflow, Placeholder],
            "cmd:shell.overlay.dismiss",
            "cmd:shell.overlay.reopen_last",
            CapabilityLoss,
            true,
            "docs/ux/dialog_sheet_contract.md",
        ),
    ]
}

fn slot(
    slot_id: ShellSlotId,
    slot_label: &str,
    permitted_surface_kinds: Vec<SurfaceKind>,
    fallback_order: Vec<FallbackPlacement>,
    close_command_id: &str,
    reopen_command_id: &str,
    placeholder_class: PlaceholderClass,
    stable_required: bool,
    owner_ref: &str,
) -> DeclaredSlotRecord {
    DeclaredSlotRecord {
        slot_id,
        shell_zone: slot_id.zone(),
        slot_label: slot_label.to_string(),
        permitted_surface_kinds,
        fallback_order,
        close_command_id: close_command_id.to_string(),
        reopen_command_id: reopen_command_id.to_string(),
        placeholder_class,
        stable_required,
        owner_ref: owner_ref.to_string(),
        private_top_level_chrome_forbidden: true,
    }
}

/// Returns compact/standard/expanded fallback ladders for every declared slot.
pub fn responsive_fallback_ladders() -> Vec<ResponsiveFallbackLadder> {
    let mut ladders = Vec::new();
    for slot in canonical_slot_registry() {
        ladders.push(ladder(
            slot.slot_id,
            AdaptiveClass::ExpandedDesktop,
            560,
            vec![FallbackPlacement::Docked, FallbackPlacement::Placeholder],
        ));
        ladders.push(ladder(
            slot.slot_id,
            AdaptiveClass::StandardDesktop,
            480,
            match slot.slot_id {
                ShellSlotId::RightInspectorContextualDetail => vec![
                    FallbackPlacement::Sheet,
                    FallbackPlacement::Overflow,
                    FallbackPlacement::Placeholder,
                ],
                ShellSlotId::BottomPanelToolPanels => vec![
                    FallbackPlacement::Docked,
                    FallbackPlacement::Overflow,
                    FallbackPlacement::Placeholder,
                ],
                _ => vec![
                    FallbackPlacement::Docked,
                    FallbackPlacement::Overflow,
                    FallbackPlacement::Placeholder,
                ],
            },
        ));
        ladders.push(ladder(
            slot.slot_id,
            AdaptiveClass::CompactDesktop,
            420,
            match slot.slot_id {
                ShellSlotId::TitleContextBarIdentity
                | ShellSlotId::ActivityRailPrimaryRoutes
                | ShellSlotId::StatusBarRecoveryPrimary
                | ShellSlotId::TransientOverlayCommandPalette => vec![
                    FallbackPlacement::Docked,
                    FallbackPlacement::Overflow,
                    FallbackPlacement::Placeholder,
                ],
                ShellSlotId::MainWorkspaceWorkingSet => {
                    vec![FallbackPlacement::Docked, FallbackPlacement::Placeholder]
                }
                _ => vec![
                    FallbackPlacement::Sheet,
                    FallbackPlacement::Overflow,
                    FallbackPlacement::Placeholder,
                ],
            },
        ));
    }
    ladders
}

fn ladder(
    slot_id: ShellSlotId,
    adaptive_class: AdaptiveClass,
    minimum_editor_width_px: u32,
    placements_in_order: Vec<FallbackPlacement>,
) -> ResponsiveFallbackLadder {
    ResponsiveFallbackLadder {
        slot_id,
        adaptive_class,
        minimum_editor_width_px,
        placements_in_order,
        preserves_slot_identity: true,
        preserves_breadcrumb_truth: true,
        preserves_trust_truth: true,
        preserves_execution_target_truth: true,
        keyboard_reachable_recovery: true,
    }
}

/// Returns representative stable surface claims admitted by the registry.
pub fn stable_surface_claims() -> Vec<StableSurfaceClaim> {
    use ShellSlotId::{
        ActivityRailPrimaryRoutes, BottomPanelToolPanels, LeftSidebarSectionSurface,
        MainWorkspaceReviewSurface, MainWorkspaceWorkingSet, RightInspectorContextualDetail,
        StatusBarExtensionScoped, StatusBarRecoveryPrimary, TitleContextBarIdentity,
        TransientOverlayCommandPalette, TransientOverlayDialogOrSheet,
    };
    use SurfaceKind::{ExtensionContributed, FirstParty, HostChrome, ProviderBacked};

    vec![
        claim(
            "surface:title-context-trust-target",
            "aureline-shell",
            TitleContextBarIdentity,
            HostChrome,
            "preserves workspace, trust, remote target, profile, and route identity",
        ),
        claim(
            "surface:activity-rail-primary",
            "aureline-shell",
            ActivityRailPrimaryRoutes,
            HostChrome,
            "owns durable mode switching without duplicate sidebars",
        ),
        claim(
            "surface:explorer-tree",
            "aureline-shell",
            LeftSidebarSectionSurface,
            FirstParty,
            "structural navigation uses the declared left sidebar section slot",
        ),
        claim(
            "surface:editor-group",
            "aureline-editor",
            MainWorkspaceWorkingSet,
            FirstParty,
            "primary working set preserves tab and breadcrumb truth",
        ),
        claim(
            "surface:review-queue",
            "aureline-review",
            MainWorkspaceReviewSurface,
            FirstParty,
            "review work stays in main workspace or declared sheet fallback",
        ),
        claim(
            "surface:debug-inspector",
            "aureline-runtime",
            RightInspectorContextualDetail,
            ProviderBacked,
            "debug variables and watch detail move to sheet before shrinking editors",
        ),
        claim(
            "surface:integrated-terminal",
            "aureline-terminal",
            BottomPanelToolPanels,
            ProviderBacked,
            "terminal boundary and transcript truth remain in bottom-panel slot",
        ),
        claim(
            "surface:status-recovery",
            "aureline-shell",
            StatusBarRecoveryPrimary,
            HostChrome,
            "recovery and degraded-state truth stays persistent",
        ),
        claim(
            "surface:extension-watch-status",
            "aureline-extensions",
            StatusBarExtensionScoped,
            ExtensionContributed,
            "scoped extension status occupies the declared status slot",
        ),
        claim(
            "surface:command-palette",
            "aureline-commands",
            TransientOverlayCommandPalette,
            FirstParty,
            "keyboard command entry remains reachable across fallback",
        ),
        claim(
            "surface:settings-review-sheet",
            "aureline-settings",
            TransientOverlayDialogOrSheet,
            FirstParty,
            "review sheets attach to the overlay slot instead of private chrome",
        ),
    ]
}

fn claim(
    surface_id: &str,
    owned_by: &str,
    declared_slot_id: ShellSlotId,
    surface_kind: SurfaceKind,
    occupancy_reason: &str,
) -> StableSurfaceClaim {
    StableSurfaceClaim {
        surface_id: surface_id.to_string(),
        owned_by: owned_by.to_string(),
        declared_slot_id,
        surface_kind,
        stable_claim: true,
        has_private_top_level_chrome: false,
        creates_duplicate_sidebar: false,
        creates_floating_global_button: false,
        occupancy_reason: occupancy_reason.to_string(),
    }
}

/// Returns placeholder-hydration fixtures for restore and contribution loss.
pub fn placeholder_hydration_cases() -> Vec<SlotHydrationCase> {
    use DependencyLossClass::{
        DisplayTopologyChanged, ExtensionRemoved, ProviderCapabilityLost, RemoteUnavailable,
        WarmRestorePendingRebind,
    };
    use PlaceholderClass::{
        CapabilityLoss, MissingExtension, MissingProvider, MissingRemote, TopologyDrift,
    };
    use ShellSlotId::{
        BottomPanelToolPanels, LeftSidebarSectionSurface, MainWorkspaceWorkingSet,
        RightInspectorContextualDetail, StatusBarExtensionScoped,
    };

    vec![
        hydration(
            "hydration:warm-restore-editor-provider",
            "surface:editor-group",
            MainWorkspaceWorkingSet,
            WarmRestorePendingRebind,
            MissingProvider,
        ),
        hydration(
            "hydration:extension-sidebar-removed",
            "surface:extension-dependency-tree",
            LeftSidebarSectionSurface,
            ExtensionRemoved,
            MissingExtension,
        ),
        hydration(
            "hydration:remote-terminal-unavailable",
            "surface:integrated-terminal",
            BottomPanelToolPanels,
            RemoteUnavailable,
            MissingRemote,
        ),
        hydration(
            "hydration:debug-capability-lost",
            "surface:debug-inspector",
            RightInspectorContextualDetail,
            ProviderCapabilityLost,
            CapabilityLoss,
        ),
        hydration(
            "hydration:mixed-dpi-status-extension",
            "surface:extension-watch-status",
            StatusBarExtensionScoped,
            DisplayTopologyChanged,
            TopologyDrift,
        ),
    ]
}

fn hydration(
    scenario_id: &str,
    surface_id: &str,
    slot_id: ShellSlotId,
    dependency_loss: DependencyLossClass,
    placeholder_class: PlaceholderClass,
) -> SlotHydrationCase {
    SlotHydrationCase {
        scenario_id: scenario_id.to_string(),
        surface_id: surface_id.to_string(),
        slot_id,
        shell_zone: slot_id.zone(),
        dependency_loss,
        placeholder_class,
        preserves_breadcrumb_truth: true,
        preserves_trust_truth: true,
        preserves_target_truth: true,
        preserves_reopen_path: true,
        adjacent_layout_collapsed: false,
        topology_identity_preserved: true,
    }
}

/// Builds the canonical packet and computes its conformance audit.
pub fn canonical_shell_zoning_packet() -> ShellZoningPacket {
    let declared_slots = canonical_slot_registry();
    let responsive_ladders = responsive_fallback_ladders();
    let stable_surface_claims = stable_surface_claims();
    let placeholder_hydration_cases = placeholder_hydration_cases();
    let audit = audit(
        &declared_slots,
        &responsive_ladders,
        &stable_surface_claims,
        &placeholder_hydration_cases,
    );

    ShellZoningPacket {
        record_kind: SHELL_ZONING_PACKET_RECORD_KIND.to_string(),
        shell_zoning_schema_version: SHELL_ZONING_SCHEMA_VERSION,
        contract_ref: CONTRACT_REF.to_string(),
        declared_slots,
        responsive_ladders,
        stable_surface_claims,
        placeholder_hydration_cases,
        audit,
    }
}

fn audit(
    declared_slots: &[DeclaredSlotRecord],
    responsive_ladders: &[ResponsiveFallbackLadder],
    stable_surface_claims: &[StableSurfaceClaim],
    placeholder_hydration_cases: &[SlotHydrationCase],
) -> ShellZoningAuditRecord {
    ShellZoningAuditRecord {
        every_stable_surface_declared: stable_surface_claims
            .iter()
            .all(|claim| claim.is_admitted_by(declared_slots)),
        every_slot_valid: declared_slots.iter().all(DeclaredSlotRecord::validates),
        no_private_top_level_chrome: stable_surface_claims
            .iter()
            .all(|claim| !claim.has_private_top_level_chrome),
        no_duplicate_sidebars: stable_surface_claims
            .iter()
            .all(|claim| !claim.creates_duplicate_sidebar),
        no_floating_global_buttons: stable_surface_claims
            .iter()
            .all(|claim| !claim.creates_floating_global_button),
        responsive_ladders_preserve_truth: responsive_ladders
            .iter()
            .all(ResponsiveFallbackLadder::protects_truth_cues),
        placeholders_preserve_declared_slot: placeholder_hydration_cases
            .iter()
            .all(SlotHydrationCase::preserves_layout_truth),
    }
}

/// Returns compact support-export lines for diagnostics and Help/About.
pub fn support_export_lines() -> Vec<String> {
    let packet = canonical_shell_zoning_packet();
    let mut lines = vec![format!(
        "{} schema={} audit_pass={}",
        packet.record_kind,
        packet.shell_zoning_schema_version,
        packet.audit.passes()
    )];

    for claim in &packet.stable_surface_claims {
        lines.push(format!(
            "surface={} owner={} slot={} zone={} reason={}",
            claim.surface_id,
            claim.owned_by,
            claim.declared_slot_id.as_str(),
            claim.declared_slot_id.zone().as_str(),
            claim.occupancy_reason
        ));
    }

    for case in &packet.placeholder_hydration_cases {
        lines.push(format!(
            "placeholder_case={} surface={} slot={} zone={} loss={} placeholder={} preserves_truth={}",
            case.scenario_id,
            case.surface_id,
            case.slot_id.as_str(),
            case.shell_zone.as_str(),
            case.dependency_loss.as_str(),
            case.placeholder_class.as_str(),
            case.preserves_layout_truth()
        ));
    }

    lines
}
