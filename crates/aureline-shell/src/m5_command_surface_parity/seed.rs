//! Canonical seed builders for the M5 command-surface parity certification.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export, and CSV
//! artifacts plus the narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code certification proof, the artifacts, and the fixtures never drift. Every attribute each family
//! row certifies over — the canonical command binding, the surface's qualification, owner, required
//! labels, stale-target states, why-unavailable reasons, feature families, cross-modality parity
//! surfaces, and declared consumer surfaces, and the applicable downgrade triggers — is pulled straight
//! from the frozen discoverability matrix's seeded packet, so the certification cannot audit a surface
//! the matrix does not anchor, and the bindings are derived from the matrix rather than restated by
//! hand. Only the affordance open modes certified, the four parity postures, the per-family posture, and
//! the scope summary are authored here.

use super::*;
use crate::freeze_the_m5_menu_keybinding_resolver_and_command_documentation_matrix::{
    seeded_m5_discoverability_matrix, M5DiscoverabilitySurfaceRow,
    M5_DISCOVERABILITY_MATRIX_PACKET_ID,
};

/// Deterministic generated-at value carried by the seeded packet.
const GENERATED_AT: &str = "2026-06-30T00:00:00Z";

/// Frozen, representative exact-build identity ref used by the seed.
///
/// A live runtime stamps the exact build identity here; the seed uses a fixed value so the checked-in
/// fixtures stay reproducible.
pub const SEED_BUILD_IDENTITY_REF: &str =
    "build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2";

/// Frozen, representative release-channel class used by the seed.
pub const SEED_RELEASE_CHANNEL_CLASS: &str = "stable";

/// The parity posture seeded for one surface family.
struct SurfaceSpec {
    /// Short conformance scope summary.
    scope_summary: &'static str,
    /// The affordance open modes this row certifies fixtures for (defaults to all five).
    certified_open_modes: Vec<M5AffordanceOpenMode>,
    /// When set, the evaluated-surface set used instead of the surface's declared set (blocked fixtures
    /// use this to prove a partial certification blocks).
    evaluated_surfaces_override: Option<Vec<M5DiscoveryChannel>>,
    canonical_projection: CanonicalProjectionState,
    target_guard: TargetGuardState,
    route_parity: RouteParityState,
    support_export_parity: SupportExportParityState,
    headless_parity_preserved: bool,
    waiver: Option<CommandSurfaceParityWaiver>,
    narrowing_reason: Option<&'static str>,
}

/// Short reviewer-facing label for a surface family.
fn surface_label(family: M5CommandSurfaceFamily) -> &'static str {
    match family {
        M5CommandSurfaceFamily::MenuItem => "Menu-bar item",
        M5CommandSurfaceFamily::MenuGroup => "Menu group / submenu",
        M5CommandSurfaceFamily::ContextMenu => "Context menu",
        M5CommandSurfaceFamily::CommandBar => "Command / action bar",
        M5CommandSurfaceFamily::KeybindingResolverLayer => "Keybinding resolver layer",
        M5CommandSurfaceFamily::ConflictReviewSheet => "Conflict review sheet",
        M5CommandSurfaceFamily::ImportBridgeRow => "Import-bridge row",
        M5CommandSurfaceFamily::DisabledCommandExplainer => "Disabled-command explainer",
        M5CommandSurfaceFamily::LeaderSequenceHelp => "Leader / sequence help overlay",
        M5CommandSurfaceFamily::CommandDocumentationSurface => "Command-documentation surface",
    }
}

/// Returns the frozen matrix surface row for a family.
fn matrix_surface_row(surface_family: M5CommandSurfaceFamily) -> M5DiscoverabilitySurfaceRow {
    seeded_m5_discoverability_matrix()
        .surface_rows
        .into_iter()
        .find(|row| row.surface_family == surface_family)
        .expect("frozen discoverability matrix declares every governed surface family")
}

/// Builds one certification row from a surface family and a parity posture. Every binding — the
/// canonical command binding, the surface's qualification, owner, required labels, stale-target states,
/// why-unavailable reasons, feature families, cross-modality parity surfaces, declared consumer
/// surfaces, and downgrade triggers — is pulled from the frozen matrix row for the family.
fn row_from_family(family: M5CommandSurfaceFamily, spec: SurfaceSpec) -> CommandSurfaceParityRow {
    let surface = matrix_surface_row(family);
    let required_consumer_surfaces = surface.consumer_surfaces.clone();
    let evaluated_consumer_surfaces = spec
        .evaluated_surfaces_override
        .unwrap_or_else(|| surface.consumer_surfaces.clone());
    let mut row = CommandSurfaceParityRow {
        surface_family: family,
        surface_label: surface_label(family).to_owned(),
        qualification: surface.qualification,
        owner_role: surface.owner_role.clone(),
        scope_summary: spec.scope_summary.to_owned(),
        canonical_command_binding: surface.canonical_command_binding.clone(),
        required_labels: surface.required_labels.clone(),
        stale_target_states: surface.stale_target_states.clone(),
        unavailable_reasons: surface.unavailable_reasons.clone(),
        feature_families: surface.feature_families.clone(),
        required_parity_surfaces: surface.parity_surfaces.clone(),
        certified_parity_surfaces: surface.parity_surfaces.clone(),
        certified_open_modes: spec.certified_open_modes,
        required_consumer_surfaces,
        evaluated_consumer_surfaces,
        canonical_projection: spec.canonical_projection,
        target_guard: spec.target_guard,
        route_parity: spec.route_parity,
        support_export_parity: spec.support_export_parity,
        headless_parity_preserved: spec.headless_parity_preserved,
        applicable_downgrade_triggers: surface.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: CommandSurfaceParityStatus::Green,
        conformance_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.conformance_causes = row.recompute_causes();
    row
}

/// Builds the leader-sequence-help architectural-route-exception waiver carried by the seed.
fn leader_route_exception_waiver() -> CommandSurfaceParityWaiver {
    CommandSurfaceParityWaiver {
        waiver_id: "waiver:leader-sequence-route-exception:0001".to_owned(),
        surface_family: M5CommandSurfaceFamily::LeaderSequenceHelp,
        reason:
            "The in-progress leader / multi-key sequence continuation is contextual-only under a \
             disclosed, waivered architectural exception — a half-typed sequence prefix only makes \
             sense while the leader is armed, so it is not surfaced as a standalone palette row, but \
             every resolved sequence still resolves to a canonical command with a palette, help, and \
             keyboard route and its behaviour is documented in the command-documentation surface — so \
             the route parity is narrowed and disclosed rather than a hidden-only route. The exception \
             retires when the palette adopts sequence-prefix rows."
                .to_owned(),
        owner_role: "Shell/keybinding owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// A full-conformance posture: all four parity dimensions hold, all five affordance open modes are
/// certified, and headless parity is preserved.
fn full(scope_summary: &'static str) -> SurfaceSpec {
    SurfaceSpec {
        scope_summary,
        certified_open_modes: M5AffordanceOpenMode::ALL.to_vec(),
        evaluated_surfaces_override: None,
        canonical_projection: CanonicalProjectionState::CanonicalLabelShortcutReasonCertified,
        target_guard: TargetGuardState::StaleTargetAndDestructiveGroupingCertified,
        route_parity: RouteParityState::EveryActionHasPaletteHelpKeyboardRoute,
        support_export_parity: SupportExportParityState::CommandIdLabelReasonReconstructable,
        headless_parity_preserved: true,
        waiver: None,
        narrowing_reason: None,
    }
}

/// Returns the seeded parity posture for one surface family.
fn family_spec(family: M5CommandSurfaceFamily) -> SurfaceSpec {
    use M5CommandSurfaceFamily as F;
    match family {
        F::MenuItem => full(
            "Menu-bar item projects the canonical label, resolved shortcut hint, and typed blocked-state \
             reason, invalidates removed or context-lost targets, keeps every action on a \
             palette/help/keyboard route, and reconstructs its command id, label, and reason from \
             durable evidence across every modality and open mode",
        ),
        F::MenuGroup => full(
            "Menu group keeps each member's canonical label and blocked-state reason, groups destructive \
             members clearly, and keeps every member on a palette/help/keyboard route across every \
             modality and open mode",
        ),
        F::KeybindingResolverLayer => full(
            "Keybinding resolver layer names the winning source layer and the resolved command id with \
             the same label and lifecycle truth the palette shows, and reconstructs its resolution from \
             durable evidence across every modality and open mode",
        ),
        F::ConflictReviewSheet => full(
            "Conflict review sheet names each conflict's controlled reason, winner, and losers with the \
             same canonical labels the palette shows, and reconstructs them from durable evidence across \
             every modality and open mode",
        ),
        F::DisabledCommandExplainer => full(
            "Disabled-command explainer names one controlled blocked-state reason, keeps the command id \
             and lifecycle truth visible, and reconstructs the command id, label, and reason from \
             durable evidence across every modality and open mode",
        ),
        F::CommandDocumentationSurface => full(
            "Command-documentation surface renders the canonical descriptor's label, shortcut source \
             layer, and lifecycle/deprecation truth without inventing a second naming system, and \
             reconstructs them from durable evidence across every modality and open mode",
        ),
        // Command bar discloses a reduced shortcut hint on a compact layout (yellow).
        F::CommandBar => SurfaceSpec {
            canonical_projection: CanonicalProjectionState::DisclosedReducedShortcutHint,
            narrowing_reason: Some(
                "On a compact command-bar layout the surface takes a disclosed reduced shortcut hint — \
                 the resolved source-layer chip is folded into a tooltip while the chord, the canonical \
                 label, and the typed blocked-state reason stay visible on every action — so the \
                 shortcut truth is narrowed and disclosed rather than hidden or relabelled.",
            ),
            ..full(
                "Command / action bar keeps its canonical labels, stale-target guard, route parity, and \
                 support/export parity across every modality and open mode, folding the source-layer \
                 chip into a tooltip on the compact layout",
            )
        },
        // Context menu discloses a deferred stale-target revalidation on a background-refresh surface
        // (yellow).
        F::ContextMenu => SurfaceSpec {
            target_guard: TargetGuardState::DisclosedDeferredTargetRevalidation,
            narrowing_reason: Some(
                "On a background-refresh surface the context menu takes a disclosed deferred \
                 stale-target revalidation — an item whose target may have moved is marked provisional \
                 and revalidated on next open while destructive items stay clearly grouped and every \
                 label and reason stays canonical — so the guard is narrowed and disclosed rather than \
                 silently misfiring.",
            ),
            ..full(
                "Context menu keeps its canonical labels, destructive grouping, route parity, and \
                 support/export parity across every modality and open mode, deferring stale-target \
                 revalidation to next open on a background-refresh surface",
            )
        },
        // Import-bridge row discloses a partial support/export capture on a legacy export (yellow).
        F::ImportBridgeRow => SurfaceSpec {
            support_export_parity: SupportExportParityState::DisclosedPartialCapture,
            narrowing_reason: Some(
                "On the legacy import diagnostics export the import-bridge row takes a disclosed partial \
                 capture — the export captures the command id, the translated native binding, and the \
                 blocked-state reason but not the resolved shortcut hint, while still disclosing the gap \
                 — so the support/export parity is narrowed and disclosed rather than absent.",
            ),
            ..full(
                "Import-bridge row keeps its canonical labels, stale-target guard, and route parity \
                 across every modality and open mode, capturing everything but the resolved shortcut hint \
                 on one legacy diagnostics export",
            )
        },
        // Leader / sequence help overlay carries a disclosed, waivered architectural route exception
        // (yellow).
        F::LeaderSequenceHelp => SurfaceSpec {
            route_parity: RouteParityState::DisclosedArchitecturalRouteException,
            waiver: Some(leader_route_exception_waiver()),
            narrowing_reason: Some(
                "The in-progress leader / multi-key sequence continuation is contextual-only under a \
                 disclosed, waivered architectural exception — a half-typed sequence prefix only makes \
                 sense while the leader is armed — but every resolved sequence still resolves to a \
                 canonical command with a palette, help, and keyboard route and its behaviour is \
                 documented, so the route parity is narrowed and disclosed rather than a hidden-only \
                 route.",
            ),
            ..full(
                "Leader / sequence help overlay keeps its canonical labels, stale-target guard, and \
                 support/export parity across every modality and open mode, surfacing the armed-sequence \
                 continuation contextual-only under a disclosed architectural exception",
            )
        },
    }
}

/// Builds the certification rows for the canonical seed, one per surface family.
fn seeded_rows() -> Vec<CommandSurfaceParityRow> {
    M5CommandSurfaceFamily::ALL
        .iter()
        .map(|&family| row_from_family(family, family_spec(family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is resolved, used by the
/// blocked fixtures.
fn seeded_rows_with<F>(target: M5CommandSurfaceFamily, mutate: F) -> Vec<CommandSurfaceParityRow>
where
    F: Fn(&mut SurfaceSpec),
{
    M5CommandSurfaceFamily::ALL
        .iter()
        .map(|&family| {
            let mut spec = family_spec(family);
            if family == target {
                mutate(&mut spec);
            }
            row_from_family(family, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<CommandSurfaceParityRow>) -> CommandSurfaceParityPacket {
    build_m5_command_surface_parity_packet(CommandSurfaceParityInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_DISCOVERABILITY_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 command-surface parity packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV artifacts.
/// Six families keep full canonical-projection, target-guard, route-parity, and support-export-parity
/// truth (green). The command bar auto-narrows to yellow disclosing a reduced shortcut hint on a compact
/// layout, the context menu auto-narrows to yellow disclosing a deferred stale-target revalidation, the
/// import-bridge row auto-narrows to yellow disclosing a partial support/export capture, and the leader /
/// sequence help overlay auto-narrows to yellow with a waivered architectural route exception — and no
/// row is blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_command_surface_parity_packet() -> CommandSurfaceParityPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the menu-bar item invents an alternate label or hides its blocked-state
/// reason, proving that a broken canonical projection blocks a stable claim (red) rather than staying
/// green.
pub fn seeded_m5_command_surface_parity_packet_menu_item_alternate_label_blocked(
) -> CommandSurfaceParityPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::MenuItem, |spec| {
        spec.canonical_projection = CanonicalProjectionState::AlternateLabelOrReasonInvented;
        spec.narrowing_reason = Some(
            "The menu-bar item relabelled a stable command with a menu-local phrase and greyed it out \
             with no typed reason, so the same action read one way in the menu and another in the \
             palette, help, and keyboard, and the item blocks before keeping a discoverability claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the context menu leaves a stale target un-invalidated or a destructive item
/// un-separated, proving that a broken target guard blocks a stable claim (red) rather than staying
/// green.
pub fn seeded_m5_command_surface_parity_packet_context_menu_stale_target_blocked(
) -> CommandSurfaceParityPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::ContextMenu, |spec| {
        spec.target_guard = TargetGuardState::StaleTargetNotInvalidatedOrDestructiveUnseparated;
        spec.narrowing_reason = Some(
            "After the focused object was deleted the context menu kept firing its actions against the \
             removed target instead of invalidating them, and mixed a destructive delete into the \
             routine edit group, so the menu was untruthful under a changing target, and it blocks \
             before keeping a discoverability claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the command bar hides a claimed action behind a contextual-only route,
/// proving that a broken route parity blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_command_surface_parity_packet_command_bar_contextual_only_blocked(
) -> CommandSurfaceParityPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::CommandBar, |spec| {
        spec.canonical_projection = CanonicalProjectionState::CanonicalLabelShortcutReasonCertified;
        spec.route_parity = RouteParityState::ContextualOnlyActionWithoutRoute;
        spec.narrowing_reason = Some(
            "The command bar exposed an apply-to-selection action that existed only on the bar with no \
             matching palette, help, or keyboard route and no disclosed architectural exception, so the \
             action could not be discovered or invoked from any other surface, and the bar blocks before \
             keeping a discoverability claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the command-documentation surface's blocked-state reason is absent from
/// durable evidence, proving that a broken support/export parity blocks a stable claim (red) rather than
/// staying green.
pub fn seeded_m5_command_surface_parity_packet_documentation_capture_absent_blocked(
) -> CommandSurfaceParityPacket {
    let rows = seeded_rows_with(
        M5CommandSurfaceFamily::CommandDocumentationSurface,
        |spec| {
            spec.support_export_parity = SupportExportParityState::BlockedReasonAbsentFromCapture;
            spec.narrowing_reason = Some(
            "The command-documentation surface rendered the disabled command's reason only as a live \
             tooltip that never reached the support export, so a support reviewer could not reconstruct \
             the command id or why it was blocked without a screenshot, and the surface blocks before \
             keeping a discoverability claim.",
        );
        },
    );
    packet_from_rows(rows)
}

/// Builds a variant where the disabled-command explainer loses the shared command semantics in a
/// headless / CLI execution, proving that a headless parity loss blocks a stable claim (red) rather than
/// staying green.
pub fn seeded_m5_command_surface_parity_packet_disabled_explainer_headless_parity_lost_blocked(
) -> CommandSurfaceParityPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::DisabledCommandExplainer, |spec| {
        spec.headless_parity_preserved = false;
        spec.narrowing_reason = Some(
            "In headless / CLI execution the disabled-command explainer emitted a private reason phrase \
             that diverged from the controlled blocked-state reason shown in-product, so the same \
             command explained its disabled state with a different language depending on how it ran, and \
             the explainer blocks before keeping a discoverability claim.",
        );
    });
    packet_from_rows(rows)
}
