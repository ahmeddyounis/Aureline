//! Canonical seed builders for the M5 command-explainer certification.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export, and CSV
//! artifacts plus the narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code explainer proof, the artifacts, and the fixtures never drift. Every attribute each family row
//! certifies over — the canonical command binding, the surface's qualification, owner, required labels,
//! lifecycle label, feature families, why-unavailable reasons, and declared consumer surfaces, and the
//! applicable downgrade triggers — is pulled straight from the frozen discoverability matrix's seeded
//! packet, so the certification cannot audit a surface the matrix does not anchor. Only the leader-overlay
//! fields, blocker classes, remediation actions, reach modes, the four explanation postures, and the scope
//! summary are authored here.

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

/// The explanation posture seeded for one surface family.
struct SurfaceSpec {
    /// Short conformance scope summary.
    scope_summary: &'static str,
    /// The leader-overlay fields this row narrates (defaults to all six).
    certified_leader_overlay_fields: Vec<M5LeaderOverlayField>,
    /// The blocker classes this row can name (defaults to all seven).
    certified_blocker_classes: Vec<M5BlockerClass>,
    /// The remediation actions this row offers (defaults to all three).
    certified_remediation_actions: Vec<M5RemediationAction>,
    /// The reach modes this row stays reachable in (defaults to all five).
    certified_reach_modes: Vec<M5ExplanationReachMode>,
    /// When set, the evaluated-surface set used instead of the surface's declared set (blocked fixtures
    /// use this to prove a partial certification blocks).
    evaluated_surfaces_override: Option<Vec<M5DiscoveryChannel>>,
    leader_overlay: LeaderOverlayState,
    blocked_explainer: BlockedExplainerState,
    remediation_parity: RemediationParityState,
    explainer_export: ExplainerExportState,
    headless_parity_preserved: bool,
    waiver: Option<CommandExplainerWaiver>,
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

/// Builds one explainer row from a surface family and a posture. Every binding — the canonical command
/// binding, the surface's qualification, owner, required labels, lifecycle label, feature families,
/// why-unavailable reasons, and declared consumer surfaces, and the downgrade triggers — is pulled from the
/// frozen matrix row for the family.
fn row_from_family(family: M5CommandSurfaceFamily, spec: SurfaceSpec) -> CommandExplainerRow {
    let surface = matrix_surface_row(family);
    let required_consumer_surfaces = surface.consumer_surfaces.clone();
    let evaluated_consumer_surfaces = spec
        .evaluated_surfaces_override
        .unwrap_or_else(|| surface.consumer_surfaces.clone());
    let mut row = CommandExplainerRow {
        surface_family: family,
        surface_label: surface_label(family).to_owned(),
        qualification: surface.qualification,
        owner_role: surface.owner_role.clone(),
        scope_summary: spec.scope_summary.to_owned(),
        lifecycle_label: surface.canonical_command_binding.lifecycle_label,
        canonical_command_binding: surface.canonical_command_binding.clone(),
        required_labels: surface.required_labels.clone(),
        feature_families: surface.feature_families.clone(),
        covered_unavailable_reasons: surface.unavailable_reasons.clone(),
        certified_leader_overlay_fields: spec.certified_leader_overlay_fields,
        certified_blocker_classes: spec.certified_blocker_classes,
        certified_remediation_actions: spec.certified_remediation_actions,
        certified_reach_modes: spec.certified_reach_modes,
        required_consumer_surfaces,
        evaluated_consumer_surfaces,
        leader_overlay: spec.leader_overlay,
        blocked_explainer: spec.blocked_explainer,
        remediation_parity: spec.remediation_parity,
        explainer_export: spec.explainer_export,
        headless_parity_preserved: spec.headless_parity_preserved,
        applicable_downgrade_triggers: surface.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        derived_status: CommandExplainerStatus::Green,
        conformance_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.conformance_causes = row.recompute_causes();
    row
}

/// Builds the leader-overlay reduced-sequence waiver carried by the seed.
fn reduced_sequence_waiver() -> CommandExplainerWaiver {
    CommandExplainerWaiver {
        waiver_id: "waiver:command-explainer-reduced-sequence:0001".to_owned(),
        surface_family: M5CommandSurfaceFamily::LeaderSequenceHelp,
        reason:
            "On the space-constrained leader / sequence help overlay one deep prefix renders a disclosed, \
             waivered reduced form — the resulting command labels / ids are folded into an expandable hint \
             while the overlay still shows the typed prefix, the current mode, the available next keys, and \
             the timeout / cancel posture, and the command palette and keybinding UI keep the full \
             resulting-label detail — so the overlay is narrowed and disclosed rather than hiding \
             next-available actions behind hover. The exception retires when the overlay renders the full \
             resulting-label detail on every claimed prefix depth."
                .to_owned(),
        owner_role: "Shell/keyboard owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// A full-conformance posture: all four explanation dimensions hold, all six leader-overlay fields, all
/// seven blocker classes, all three remediation actions, and all five reach modes are certified, and
/// headless parity is preserved.
fn full(scope_summary: &'static str) -> SurfaceSpec {
    SurfaceSpec {
        scope_summary,
        certified_leader_overlay_fields: M5LeaderOverlayField::ALL.to_vec(),
        certified_blocker_classes: M5BlockerClass::ALL.to_vec(),
        certified_remediation_actions: M5RemediationAction::ALL.to_vec(),
        certified_reach_modes: M5ExplanationReachMode::ALL.to_vec(),
        evaluated_surfaces_override: None,
        leader_overlay: LeaderOverlayState::TypedPrefixNextKeysAndTimeoutNarrated,
        blocked_explainer: BlockedExplainerState::BlockerClassNextActionAndActionsCertified,
        remediation_parity: RemediationParityState::SharedReasonPacketAcrossAllSurfaces,
        explainer_export: ExplainerExportState::BlockerAndCommandIdReconstructable,
        headless_parity_preserved: true,
        waiver: None,
        narrowing_reason: None,
    }
}

/// Returns the seeded explanation posture for one surface family.
fn family_spec(family: M5CommandSurfaceFamily) -> SurfaceSpec {
    use M5CommandSurfaceFamily as F;
    match family {
        F::MenuItem => full(
            "Menu-bar item narrates any pending leader prefix, names the blocker class, next safe action, \
             and copy-id / open-help actions on a disabled item, reuses the shared reason packet, and \
             reconstructs its blocker and command id from durable evidence across every consumer surface \
             and reach mode",
        ),
        F::MenuGroup => full(
            "Menu group explains each member's blocked state with the shared reason packet, stays reachable \
             in every reach mode, and reconstructs the blocker and command id from durable evidence across \
             every consumer surface",
        ),
        F::KeybindingResolverLayer => full(
            "Keybinding resolver layer narrates the pending sequence and explains a reserved / blocked \
             binding with the shared reason packet and remediation actions across every consumer surface \
             and reach mode",
        ),
        F::ConflictReviewSheet => full(
            "Conflict review sheet explains a blocked / shadowed binding with the shared reason packet, \
             offers the remediation actions, and reconstructs the blocker and command id from durable \
             evidence across every consumer surface",
        ),
        F::DisabledCommandExplainer => full(
            "Disabled-command explainer names the blocker class, the next safe action, and the copy-id / \
             open-help actions from the shared reason packet, stays reachable in every reach mode, and \
             reconstructs the blocker and command id from durable evidence across every consumer surface",
        ),
        F::CommandDocumentationSurface => full(
            "Command-documentation surface explains a command's blocked / deprecated state with the shared \
             reason packet and remediation actions, and reconstructs the blocker and command id from \
             durable evidence across every consumer surface and reach mode",
        ),
        // Leader / sequence help overlay discloses a waivered reduced sequence overlay on a deep prefix
        // (yellow).
        F::LeaderSequenceHelp => SurfaceSpec {
            leader_overlay: LeaderOverlayState::DisclosedReducedSequenceOverlay,
            waiver: Some(reduced_sequence_waiver()),
            narrowing_reason: Some(
                "On the space-constrained leader / sequence help overlay one deep prefix renders a \
                 disclosed, waivered reduced form — the resulting command labels / ids are folded into an \
                 expandable hint while the typed prefix, current mode, available next keys, and timeout / \
                 cancel posture stay visible — so the overlay is narrowed and disclosed rather than hiding \
                 next-available actions behind hover.",
            ),
            ..full(
                "Leader / sequence help overlay narrates the typed prefix, mode, next keys, and timeout / \
                 cancel posture across every consumer surface, folding the resulting-label detail into an \
                 expandable hint on one deep prefix under a waivered exception",
            )
        },
        // Command / action bar discloses a reduced explainer detail on a constrained surface (yellow).
        F::CommandBar => SurfaceSpec {
            blocked_explainer: BlockedExplainerState::DisclosedReducedExplainerDetail,
            narrowing_reason: Some(
                "On the constrained command / action bar the disabled-command explainer takes a disclosed \
                 reduced detail — the next-safe-action guidance is folded into an expandable section while \
                 the blocker class and the copy-command-id / open-help actions stay visible — so the \
                 explainer is narrowed and disclosed rather than failing silently or showing only generic \
                 copy.",
            ),
            ..full(
                "Command / action bar explains a blocked command with the shared reason packet across every \
                 consumer surface, folding the next-safe-action guidance into an expandable section on the \
                 constrained bar",
            )
        },
        // Context menu discloses a short surface-local remediation note on a constrained surface (yellow).
        F::ContextMenu => SurfaceSpec {
            remediation_parity: RemediationParityState::DisclosedSurfaceLocalRemediationNote,
            narrowing_reason: Some(
                "On the space-constrained context menu one blocked action appends a disclosed short \
                 surface-local remediation note while still projecting the shared reason packet and \
                 remediation language — so the remediation is narrowed and disclosed rather than an \
                 invented surface-local error prose.",
            ),
            ..full(
                "Context menu explains each focused object's blocked action with the shared reason packet \
                 across every consumer surface, appending a disclosed short surface-local remediation note \
                 on one action",
            )
        },
        // Import-bridge row discloses a partial copy-safe export capture on a legacy export (yellow).
        F::ImportBridgeRow => SurfaceSpec {
            explainer_export: ExplainerExportState::DisclosedPartialCapture,
            narrowing_reason: Some(
                "On the legacy import export the copy-safe explainer export takes a disclosed partial \
                 capture — the export captures the blocker class and command id but not the full \
                 remediation-action list, while still disclosing the gap — so the copy-safe export parity \
                 is narrowed and disclosed rather than absent.",
            ),
            ..full(
                "Import-bridge row explains a rejected / unmapped binding with the shared reason packet \
                 across every consumer surface, capturing the blocker class and command id but not the full \
                 remediation-action list on one legacy export",
            )
        },
    }
}

/// Builds the explainer rows for the canonical seed, one per surface family.
fn seeded_rows() -> Vec<CommandExplainerRow> {
    M5CommandSurfaceFamily::ALL
        .iter()
        .map(|&family| row_from_family(family, family_spec(family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is resolved, used by the
/// blocked fixtures.
fn seeded_rows_with<F>(target: M5CommandSurfaceFamily, mutate: F) -> Vec<CommandExplainerRow>
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

fn packet_from_rows(rows: Vec<CommandExplainerRow>) -> CommandExplainerPacket {
    build_m5_command_explainers_packet(CommandExplainerInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_DISCOVERABILITY_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 command-explainer packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV artifacts. Six
/// families keep full leader-overlay, blocked-explainer, remediation-parity, and explainer-export truth
/// (green). The leader / sequence help overlay auto-narrows to yellow with a waivered reduced sequence
/// overlay, the command / action bar auto-narrows to yellow disclosing a reduced explainer detail, the
/// context menu auto-narrows to yellow disclosing a short surface-local remediation note, and the
/// import-bridge row auto-narrows to yellow disclosing a partial copy-safe export capture — and no row is
/// blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_command_explainers_packet() -> CommandExplainerPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the menu-bar item greys out a disabled command with no explanation / only generic
/// copy, proving that a silent failure blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_command_explainers_packet_menu_item_silent_failure_blocked(
) -> CommandExplainerPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::MenuItem, |spec| {
        spec.blocked_explainer = BlockedExplainerState::BlockedCommandFailsSilentlyOrGeneric;
        spec.narrowing_reason = Some(
            "The menu-bar item greyed out a disabled command with only generic \"unavailable\" copy, so a \
             reader could not see the blocker class, the next safe action, or how to copy the command id or \
             open help, and the item blocks before keeping an explanation claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the context menu invents surface-local error prose, proving that surface-local
/// prose blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_command_explainers_packet_context_menu_surface_local_prose_blocked(
) -> CommandExplainerPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::ContextMenu, |spec| {
        spec.remediation_parity = RemediationParityState::SurfaceLocalErrorProseInvented;
        spec.narrowing_reason = Some(
            "The context menu invented its own error and remediation prose that disagreed with the shared \
             reason packet the palette and keybinding UI project, so the exact blocker and remediation path \
             read differently depending on the reach, and the menu blocks before keeping an explanation \
             claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the leader / sequence help overlay's availability requires hidden knowledge,
/// proving that a hidden-knowledge sequence blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_command_explainers_packet_leader_hidden_knowledge_blocked(
) -> CommandExplainerPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::LeaderSequenceHelp, |spec| {
        spec.leader_overlay = LeaderOverlayState::SequenceAvailabilityRequiresHiddenKnowledge;
        spec.waiver = None;
        spec.narrowing_reason = Some(
            "The leader / sequence help overlay stopped narrating the typed prefix, next keys, and timeout \
             / cancel posture, so the sequence's next-available actions could only be discovered by already \
             knowing them, and the overlay blocks before keeping an explanation claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the import-bridge row's blocker reason / command id is absent from the durable
/// export, proving that an absent capture blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_command_explainers_packet_import_bridge_capture_absent_blocked(
) -> CommandExplainerPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::ImportBridgeRow, |spec| {
        spec.explainer_export = ExplainerExportState::BlockerReasonAbsentFromCapture;
        spec.narrowing_reason = Some(
            "The import-bridge row rendered its blocker reason only as a live badge that never reached the \
             durable, diffable explainer export, so a support bundle or migration packet could not \
             reconstruct the same blocker or remediation without a screenshot, and the row blocks before \
             keeping an explanation claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the disabled-command explainer loses the shared explanation in a headless / CLI
/// execution, proving that a headless parity loss blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_command_explainers_packet_explainer_headless_parity_lost_blocked(
) -> CommandExplainerPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::DisabledCommandExplainer, |spec| {
        spec.headless_parity_preserved = false;
        spec.narrowing_reason = Some(
            "In headless / CLI execution the disabled-command explainer reported a different blocker class \
             and remediation than the in-product surface, so the same command explained a different blocker \
             depending on how it ran, and the explainer blocks before keeping an explanation claim.",
        );
    });
    packet_from_rows(rows)
}
