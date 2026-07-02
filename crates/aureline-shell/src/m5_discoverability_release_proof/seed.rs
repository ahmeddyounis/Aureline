//! Canonical seed builders for the M5 discoverability-release-proof certification.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export, and CSV
//! artifacts plus the narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code release-evidence proof, the artifacts, and the fixtures never drift. Every attribute each family
//! row certifies over — the canonical command binding, the surface's qualification, owner, required labels,
//! lifecycle label, preview class, feature families, and declared consumer surfaces, and the applicable
//! downgrade triggers — is pulled straight from the frozen discoverability matrix's seeded packet, so the
//! certification cannot audit a surface the matrix does not anchor. Only the proof dimensions, desktop
//! profiles, the four discoverability-truth postures, and the scope summary are authored here.

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

/// The discoverability-truth posture seeded for one surface family.
struct SurfaceSpec {
    /// Short conformance scope summary.
    scope_summary: &'static str,
    /// The discoverability proof dimensions this row certifies (defaults to all four).
    certified_proof_dimensions: Vec<M5DiscoverabilityProofDimension>,
    /// The desktop profiles this row certifies across (defaults to all six).
    certified_profiles: Vec<M5DesktopProfile>,
    /// When set, the evaluated-surface set used instead of the surface's declared set (blocked fixtures
    /// use this to prove a partial certification blocks).
    evaluated_surfaces_override: Option<Vec<M5DiscoveryChannel>>,
    menu_affordance_truth: MenuAffordanceTruthState,
    keybinding_resolver_truth: KeybindingResolverTruthState,
    leader_help_truth: LeaderHelpTruthState,
    command_documentation_truth: CommandDocumentationTruthState,
    headless_parity_preserved: bool,
    waiver: Option<ReleaseProofWaiver>,
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

/// Builds one release-proof row from a surface family and a posture. Every binding — the canonical command
/// binding, the surface's qualification, owner, required labels, lifecycle label, preview class, feature
/// families, and declared consumer surfaces, and the downgrade triggers — is pulled from the frozen matrix
/// row for the family.
fn row_from_family(family: M5CommandSurfaceFamily, spec: SurfaceSpec) -> ReleaseProofRow {
    let surface = matrix_surface_row(family);
    let required_consumer_surfaces = surface.consumer_surfaces.clone();
    let evaluated_consumer_surfaces = spec
        .evaluated_surfaces_override
        .unwrap_or_else(|| surface.consumer_surfaces.clone());
    let mut row = ReleaseProofRow {
        surface_family: family,
        surface_label: surface_label(family).to_owned(),
        qualification: surface.qualification,
        owner_role: surface.owner_role.clone(),
        scope_summary: spec.scope_summary.to_owned(),
        lifecycle_label: surface.canonical_command_binding.lifecycle_label,
        preview_class: surface.canonical_command_binding.preview_class,
        disabled_reason_mode: surface.canonical_command_binding.disabled_reason_mode,
        canonical_command_binding: surface.canonical_command_binding.clone(),
        required_labels: surface.required_labels.clone(),
        feature_families: surface.feature_families.clone(),
        certified_proof_dimensions: spec.certified_proof_dimensions,
        certified_profiles: spec.certified_profiles,
        required_consumer_surfaces,
        evaluated_consumer_surfaces,
        menu_affordance_truth: spec.menu_affordance_truth,
        keybinding_resolver_truth: spec.keybinding_resolver_truth,
        leader_help_truth: spec.leader_help_truth,
        command_documentation_truth: spec.command_documentation_truth,
        headless_parity_preserved: spec.headless_parity_preserved,
        applicable_downgrade_triggers: surface.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        derived_status: ReleaseProofStatus::Green,
        conformance_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.conformance_causes = row.recompute_causes();
    row
}

/// Builds the leader-help reduced-explainer-detail waiver carried by the seed.
fn reduced_explainer_detail_waiver() -> ReleaseProofWaiver {
    ReleaseProofWaiver {
        waiver_id: "waiver:release-proof-reduced-explainer:0001".to_owned(),
        surface_family: M5CommandSurfaceFamily::LeaderSequenceHelp,
        reason:
            "On the space-constrained compact-layout profile the leader / sequence help overlay renders a \
             disclosed, waivered reduced explainer — the next-safe-action detail folds into an expandable \
             note while the typed prefix, available next keys, resulting command id, and blocker class stay \
             present — so the blocked / in-progress intent stays explainable rather than silent. The \
             exception retires when the overlay renders the full explainer inline on every claimed profile."
                .to_owned(),
        owner_role: "Shell/command-discovery owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// A full-conformance posture: all four discoverability truth dimensions hold, all four proof dimensions and
/// all six desktop profiles are certified, and headless parity is preserved.
fn full(scope_summary: &'static str) -> SurfaceSpec {
    SurfaceSpec {
        scope_summary,
        certified_proof_dimensions: M5DiscoverabilityProofDimension::ALL.to_vec(),
        certified_profiles: M5DesktopProfile::ALL.to_vec(),
        evaluated_surfaces_override: None,
        menu_affordance_truth: MenuAffordanceTruthState::MenuAffordanceParityCertified,
        keybinding_resolver_truth: KeybindingResolverTruthState::ShortcutResolutionInspectable,
        leader_help_truth: LeaderHelpTruthState::LeaderAndBlockedExplainerCertified,
        command_documentation_truth: CommandDocumentationTruthState::CommandDocRecordCertified,
        headless_parity_preserved: true,
        waiver: None,
        narrowing_reason: None,
    }
}

/// Returns the seeded discoverability-truth posture for one surface family.
fn family_spec(family: M5CommandSurfaceFamily) -> SurfaceSpec {
    use M5CommandSurfaceFamily as F;
    match family {
        F::MenuItem => full(
            "Menu-bar item keeps a current menu-affordance, keybinding-resolver, leader-help, and \
             command-documentation proof across every consumer surface and every desktop profile",
        ),
        F::MenuGroup => full(
            "Menu group keeps the same canonical label, shortcut hint, and blocked-state reason for every \
             member across every reach, with a current proof across every desktop profile",
        ),
        F::ContextMenu => full(
            "Context menu projects the canonical label, shortcut hint, and blocked-state reason for every \
             focused-object action and keeps its resolver, explainer, and documentation proof current",
        ),
        F::ConflictReviewSheet => full(
            "Conflict review sheet keeps the winning-and-shadowed binding inspectable and its \
             menu-affordance, explainer, and documentation proof current across every desktop profile",
        ),
        F::ImportBridgeRow => full(
            "Import-bridge row keeps its translated-shortcut outcome inspectable and its menu-affordance, \
             explainer, and documentation proof current across every desktop profile",
        ),
        F::DisabledCommandExplainer => full(
            "Disabled-command explainer keeps its blocker class, next-safe-action, and command id current \
             and its menu-affordance, resolver, and documentation proof current across every desktop profile",
        ),
        // Command / action bar discloses a shortened affordance hint on a dense surface (yellow).
        F::CommandBar => SurfaceSpec {
            menu_affordance_truth: MenuAffordanceTruthState::DisclosedReducedAffordanceHint,
            narrowing_reason: Some(
                "On the dense command / action bar one affordance renders a disclosed shortened shortcut \
                 hint — the modifier chord folds into a compact glyph — while still projecting the \
                 canonical label and blocked-state reason, so the menu-affordance parity is narrowed and \
                 disclosed rather than inventing an alternate label.",
            ),
            ..full(
                "Command / action bar keeps its resolver, explainer, and documentation proof current, \
                 disclosing a shortened shortcut hint on one dense affordance while still projecting the \
                 canonical label and reason",
            )
        },
        // Keybinding resolver layer discloses a reduced inspector detail (yellow).
        F::KeybindingResolverLayer => SurfaceSpec {
            keybinding_resolver_truth: KeybindingResolverTruthState::DisclosedReducedResolverDetail,
            narrowing_reason: Some(
                "On the keybinding resolver layer the shadowed-candidate detail folds into an expandable \
                 inspector while the winning binding and its source layer stay named inline, so the \
                 shortcut resolution is narrowed and disclosed rather than hidden.",
            ),
            ..full(
                "Keybinding resolver layer keeps its menu-affordance, explainer, and documentation proof \
                 current, folding the shadowed-candidate detail into an expandable inspector while naming \
                 the winning binding and source inline",
            )
        },
        // Leader / sequence help overlay discloses a reduced explainer detail on a constrained profile
        // (yellow, waivered).
        F::LeaderSequenceHelp => SurfaceSpec {
            leader_help_truth: LeaderHelpTruthState::DisclosedReducedExplainerDetail,
            waiver: Some(reduced_explainer_detail_waiver()),
            narrowing_reason: Some(
                "On the space-constrained compact-layout profile the leader / sequence help overlay renders \
                 a disclosed, waivered reduced explainer — the next-safe-action detail folds into an \
                 expandable note while the typed prefix, available next keys, resulting command id, and \
                 blocker class stay present — so the blocked / in-progress intent stays explainable rather \
                 than silent.",
            ),
            ..full(
                "Leader / sequence help overlay keeps its menu-affordance, resolver, and documentation \
                 proof current, rendering a disclosed, waivered reduced explainer on the compact-layout \
                 profile while keeping the blocker class and command id present",
            )
        },
        // Command-documentation surface discloses a reduced doc detail on a legacy surface (yellow).
        F::CommandDocumentationSurface => SurfaceSpec {
            command_documentation_truth: CommandDocumentationTruthState::DisclosedReducedDocDetail,
            narrowing_reason: Some(
                "On one legacy documentation surface the canonical example set folds into a \"see full \
                 docs\" link while the command id, lifecycle state, aliases, and supported surfaces stay \
                 present, so the command-documentation truth is narrowed and disclosed rather than stale.",
            ),
            ..full(
                "Command-documentation surface keeps its menu-affordance, resolver, and explainer proof \
                 current, folding the canonical example set into a \"see full docs\" link on one legacy \
                 surface while keeping the command id and lifecycle state present",
            )
        },
    }
}

/// Builds the release-proof rows for the canonical seed, one per surface family.
fn seeded_rows() -> Vec<ReleaseProofRow> {
    M5CommandSurfaceFamily::ALL
        .iter()
        .map(|&family| row_from_family(family, family_spec(family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is resolved, used by the
/// blocked fixtures.
fn seeded_rows_with<F>(target: M5CommandSurfaceFamily, mutate: F) -> Vec<ReleaseProofRow>
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

fn packet_from_rows(rows: Vec<ReleaseProofRow>) -> ReleaseProofPacket {
    build_m5_release_proof_packet(ReleaseProofInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_DISCOVERABILITY_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 discoverability-release-proof packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV artifacts. Six
/// families keep full menu-affordance, keybinding-resolver, leader-help, and command-documentation truth
/// (green). The command / action bar auto-narrows to yellow disclosing a shortened affordance hint, the
/// keybinding resolver layer auto-narrows to yellow disclosing a reduced inspector detail, the leader /
/// sequence help overlay auto-narrows to yellow with a waivered reduced explainer detail, and the
/// command-documentation surface auto-narrows to yellow disclosing a reduced doc detail — and no row is
/// blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_discoverability_release_proof_packet() -> ReleaseProofPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the menu-bar item invents an alternate label or widens authority, proving that an
/// invented label blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_discoverability_release_proof_packet_menu_item_alternate_label_blocked(
) -> ReleaseProofPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::MenuItem, |spec| {
        spec.menu_affordance_truth = MenuAffordanceTruthState::AlternateLabelOrAuthorityInvented;
        spec.narrowing_reason = Some(
            "The menu-bar item renamed a stable command and dropped the approval-required badge its \
             canonical record carries, so the same action changed its label and appeared to widen its \
             authority depending on where it was reached, and the item blocks before keeping a \
             discoverability claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the keybinding resolver layer hides the winning or shadowed binding, proving that a
/// hidden binding blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_discoverability_release_proof_packet_resolver_binding_hidden_blocked(
) -> ReleaseProofPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::KeybindingResolverLayer, |spec| {
        spec.keybinding_resolver_truth =
            KeybindingResolverTruthState::WinningOrShadowedBindingHidden;
        spec.narrowing_reason = Some(
            "The keybinding resolver layer showed only the active shortcut and hid which binding it \
             shadowed and how an import translated it, so which shortcut wins required private knowledge, \
             and the resolver blocks before keeping a discoverability claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the disabled-command explainer lets blocked intent fail silently or go generic,
/// proving that silent blocked intent blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_discoverability_release_proof_packet_explainer_blocked_intent_silent_blocked(
) -> ReleaseProofPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::DisabledCommandExplainer, |spec| {
        spec.leader_help_truth = LeaderHelpTruthState::BlockedIntentSilentOrGeneric;
        spec.narrowing_reason = Some(
            "The disabled-command explainer greyed the command out with no reason and only a generic \
             \"unavailable\" tooltip, dropping the shared blocker class and next-safe-action, so a \
             keyboard-first user could not tell why the command was blocked or how to proceed, and the \
             explainer blocks before keeping a discoverability claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the command-documentation surface ships a stale or mismatched command record,
/// proving that a stale doc record blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_discoverability_release_proof_packet_doc_record_stale_blocked(
) -> ReleaseProofPacket {
    let rows = seeded_rows_with(
        M5CommandSurfaceFamily::CommandDocumentationSurface,
        |spec| {
            spec.command_documentation_truth =
                CommandDocumentationTruthState::DocRecordStaleOrMismatched;
            spec.narrowing_reason = Some(
                "The command-documentation surface still described a command as generally available after \
                 it was deprecated and listed a removed alias, so the documentation overclaimed a command \
                 the runtime no longer honored, and the surface blocks before keeping a discoverability \
                 claim.",
            );
        },
    );
    packet_from_rows(rows)
}

/// Builds a variant where the import-bridge row loses the shared discoverability proof in a headless / CLI
/// execution, proving that a headless parity loss blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_discoverability_release_proof_packet_import_bridge_headless_parity_lost_blocked(
) -> ReleaseProofPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::ImportBridgeRow, |spec| {
        spec.headless_parity_preserved = false;
        spec.narrowing_reason = Some(
            "In headless / CLI execution the import-bridge row reported a different translated-shortcut \
             outcome and command label than the in-product surface, so the same command explained a \
             different keybinding / documentation truth depending on how it ran, and the row blocks before \
             keeping a discoverability claim.",
        );
    });
    packet_from_rows(rows)
}
