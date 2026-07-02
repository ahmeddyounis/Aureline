//! Canonical seed builders for the M5 keybinding resolver inspection certification.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export, and CSV
//! artifacts plus the narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code inspection proof, the artifacts, and the fixtures never drift. Every attribute each family row
//! certifies over — the canonical command binding, the surface's qualification, owner, required labels,
//! shortcut-source classes, conflict reasons, import-translation states, stale-target states,
//! why-unavailable reasons, feature families, and declared consumer surfaces, and the applicable downgrade
//! triggers — is pulled straight from the frozen discoverability matrix's seeded packet, so the
//! certification cannot audit a surface the matrix does not anchor, and the winner/shadowed resolution is
//! derived from the matrix's shortcut-source classes rather than restated by hand. Only the inspector
//! fields, controlled bridge outcomes, migration actions, the four inspection postures, and the scope
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

/// The inspection posture seeded for one surface family.
struct SurfaceSpec {
    /// Short conformance scope summary.
    scope_summary: &'static str,
    /// The inspector fields this row reveals (defaults to all seven).
    certified_inspector_fields: Vec<M5InspectorField>,
    /// The controlled bridge-outcome states this row renders (defaults to all six).
    certified_bridge_outcomes: Vec<M5BridgeOutcomeState>,
    /// The migration actions this row offers (defaults to all three).
    certified_migration_actions: Vec<M5MigrationAction>,
    /// When set, the evaluated-surface set used instead of the surface's declared set (blocked fixtures
    /// use this to prove a partial certification blocks).
    evaluated_surfaces_override: Option<Vec<M5DiscoveryChannel>>,
    resolver_inspection: ResolverInspectionState,
    bridge_outcome: BridgeOutcomeState,
    leader_sequence_inspection: LeaderSequenceInspectionState,
    resolver_export: ResolverExportState,
    headless_parity_preserved: bool,
    waiver: Option<ResolverInspectorWaiver>,
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

/// Builds one inspection row from a surface family and a posture. Every binding — the canonical command
/// binding, the surface's qualification, owner, required labels, shortcut-source classes, conflict
/// reasons, import-translation states, stale-target states, why-unavailable reasons, feature families,
/// declared consumer surfaces, and downgrade triggers — is pulled from the frozen matrix row for the
/// family, and the winner/shadowed resolution is derived from the matrix's shortcut-source classes.
fn row_from_family(family: M5CommandSurfaceFamily, spec: SurfaceSpec) -> ResolverInspectorRow {
    let surface = matrix_surface_row(family);
    let required_consumer_surfaces = surface.consumer_surfaces.clone();
    let evaluated_consumer_surfaces = spec
        .evaluated_surfaces_override
        .unwrap_or_else(|| surface.consumer_surfaces.clone());
    let mut row = ResolverInspectorRow {
        surface_family: family,
        surface_label: surface_label(family).to_owned(),
        qualification: surface.qualification,
        owner_role: surface.owner_role.clone(),
        scope_summary: spec.scope_summary.to_owned(),
        canonical_command_binding: surface.canonical_command_binding.clone(),
        required_labels: surface.required_labels.clone(),
        shortcut_source_classes: surface.shortcut_source_classes.clone(),
        // Recomputed by the builder; the seed value is the derived resolution.
        winning_source_class: None,
        shadowed_source_classes: Vec::new(),
        conflict_reasons: surface.conflict_reasons.clone(),
        import_translation_states: surface.import_translation_states.clone(),
        stale_target_states: surface.stale_target_states.clone(),
        unavailable_reasons: surface.unavailable_reasons.clone(),
        feature_families: surface.feature_families.clone(),
        certified_inspector_fields: spec.certified_inspector_fields,
        certified_bridge_outcomes: spec.certified_bridge_outcomes,
        certified_migration_actions: spec.certified_migration_actions,
        required_consumer_surfaces,
        evaluated_consumer_surfaces,
        resolver_inspection: spec.resolver_inspection,
        bridge_outcome: spec.bridge_outcome,
        leader_sequence_inspection: spec.leader_sequence_inspection,
        resolver_export: spec.resolver_export,
        headless_parity_preserved: spec.headless_parity_preserved,
        applicable_downgrade_triggers: surface.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        derived_status: ResolverInspectorStatus::Green,
        conformance_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.winning_source_class = row.recompute_winner();
    row.shadowed_source_classes = row.recompute_shadowed();
    row.derived_status = row.recompute_status();
    row.conformance_causes = row.recompute_causes();
    row
}

/// Builds the leader-sequence reduced-hint waiver carried by the seed.
fn leader_reduced_hint_waiver() -> ResolverInspectorWaiver {
    ResolverInspectorWaiver {
        waiver_id: "waiver:leader-sequence-reduced-hint:0001".to_owned(),
        surface_family: M5CommandSurfaceFamily::LeaderSequenceHelp,
        reason:
            "The armed leader / multi-key sequence overlay renders a reduced next-key hint under a \
             disclosed, waivered exception — a half-typed sequence continuation folds its next-key list \
             into a compact hint while the precedence model, the timeout / cancel hints, and the \
             accessibility narration stay available and every resolved sequence still names its winning \
             source and fallback command path — so the sequence availability is narrowed and disclosed \
             rather than requiring hidden knowledge. The exception retires when the overlay renders the \
             full next-key list on every claimed family."
                .to_owned(),
        owner_role: "Shell/keybinding owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// A full-conformance posture: all four inspection dimensions hold, all seven inspector fields, all six
/// controlled bridge outcomes, and all three migration actions are certified, and headless parity is
/// preserved.
fn full(scope_summary: &'static str) -> SurfaceSpec {
    SurfaceSpec {
        scope_summary,
        certified_inspector_fields: M5InspectorField::ALL.to_vec(),
        certified_bridge_outcomes: M5BridgeOutcomeState::ALL.to_vec(),
        certified_migration_actions: M5MigrationAction::ALL.to_vec(),
        evaluated_surfaces_override: None,
        resolver_inspection: ResolverInspectionState::WinnerShadowedSourceAndFallbackCertified,
        bridge_outcome: BridgeOutcomeState::ControlledStatesAndMigrationActionsCertified,
        leader_sequence_inspection:
            LeaderSequenceInspectionState::PrecedenceTimeoutCancelNarrationCertified,
        resolver_export: ResolverExportState::CommandIdAndWinningSourceReconstructable,
        headless_parity_preserved: true,
        waiver: None,
        narrowing_reason: None,
    }
}

/// Returns the seeded inspection posture for one surface family.
fn family_spec(family: M5CommandSurfaceFamily) -> SurfaceSpec {
    use M5CommandSurfaceFamily as F;
    match family {
        F::MenuItem => full(
            "Menu-bar item names the winning source layer and shadowed losers for its command's chord, \
             renders controlled bridge outcomes and migration actions for imported bindings, and \
             reconstructs the command id and winning source from durable evidence across every consumer \
             surface",
        ),
        F::MenuGroup => full(
            "Menu group names each member's winning source layer and fallback path, renders controlled \
             bridge outcomes for imported members, and reconstructs the resolution from durable evidence \
             across every consumer surface",
        ),
        F::ContextMenu => full(
            "Context menu names the winning source layer and shadowed losers for the focused object's \
             actions, renders controlled bridge outcomes and migration actions, and reconstructs the \
             resolution from durable evidence across every consumer surface",
        ),
        F::CommandBar => full(
            "Command / action bar names the winning source layer and shadowed losers for the active \
             surface's commands, renders controlled bridge outcomes, and reconstructs the resolution from \
             durable evidence across every consumer surface",
        ),
        F::ConflictReviewSheet => full(
            "Conflict review sheet names each conflict's controlled reason, its winning source layer, and \
             every shadowed loser, renders controlled bridge outcomes with migration actions, and \
             reconstructs the resolution from durable evidence across every consumer surface",
        ),
        F::DisabledCommandExplainer => full(
            "Disabled-command explainer names the winning source layer and fallback command path even \
             when the command is unavailable, renders controlled bridge outcomes, and reconstructs the \
             command id and winning source from durable evidence across every consumer surface",
        ),
        // Keybinding resolver layer discloses a reduced inspector detail on a constrained surface
        // (yellow).
        F::KeybindingResolverLayer => SurfaceSpec {
            resolver_inspection: ResolverInspectionState::DisclosedReducedInspectorDetail,
            narrowing_reason: Some(
                "On a constrained resolver surface the inspector takes a disclosed reduced inspector \
                 detail — the full losing-candidate list is folded into an expandable \"N shadowed\" \
                 summary while the winning source layer, the fallback command path, the scope, the \
                 current mode, and the reserved/unavailable state stay visible — so the shadowed truth is \
                 narrowed and disclosed rather than hidden.",
            ),
            ..full(
                "Keybinding resolver layer names the winning source layer and fallback path across every \
                 consumer surface, folding the full losing-candidate list into an expandable summary on \
                 the constrained surface",
            )
        },
        // Import-bridge row discloses a partial bridge coverage while manual review completes (yellow).
        F::ImportBridgeRow => SurfaceSpec {
            bridge_outcome: BridgeOutcomeState::DisclosedPartialBridgeCoverage,
            narrowing_reason: Some(
                "One slice of imported bindings takes a disclosed partial bridge coverage — it is \
                 reported with a controlled `partial` / `shimmed` state and an open-docs / manual-fix \
                 action while manual review completes — so the import outcome is narrowed and disclosed \
                 with the controlled bridge-outcome vocabulary rather than generic imported wording.",
            ),
            ..full(
                "Import-bridge row renders controlled bridge outcomes and migration actions across every \
                 consumer surface, disclosing a partial coverage slice while manual review completes",
            )
        },
        // Leader / sequence help overlay carries a disclosed, waivered reduced sequence hint (yellow).
        F::LeaderSequenceHelp => SurfaceSpec {
            leader_sequence_inspection: LeaderSequenceInspectionState::DisclosedReducedSequenceHint,
            waiver: Some(leader_reduced_hint_waiver()),
            narrowing_reason: Some(
                "The armed leader / multi-key sequence overlay renders a disclosed, waivered reduced \
                 sequence hint — a half-typed continuation folds its next-key list into a compact hint \
                 while the precedence model, the timeout / cancel hints, and the narration stay available \
                 and every resolved sequence still names its winning source and fallback — so the \
                 sequence availability is narrowed and disclosed rather than requiring hidden knowledge.",
            ),
            ..full(
                "Leader / sequence help overlay keeps the same precedence / timeout / narration model as \
                 ordinary bindings across every consumer surface, rendering a reduced next-key hint under \
                 a disclosed architectural exception",
            )
        },
        // Command-documentation surface discloses a partial resolver/export capture on a legacy export
        // (yellow).
        F::CommandDocumentationSurface => SurfaceSpec {
            resolver_export: ResolverExportState::DisclosedPartialCapture,
            narrowing_reason: Some(
                "On the legacy documentation export the resolver/export surface takes a disclosed partial \
                 capture — the export captures the command id and the winning source but not the full \
                 shadowed list, while still disclosing the gap — so the resolver/export parity is \
                 narrowed and disclosed rather than absent.",
            ),
            ..full(
                "Command-documentation surface renders the winning source layer and fallback path without \
                 inventing a second naming system across every consumer surface, capturing everything but \
                 the full shadowed list on one legacy export",
            )
        },
    }
}

/// Builds the inspection rows for the canonical seed, one per surface family.
fn seeded_rows() -> Vec<ResolverInspectorRow> {
    M5CommandSurfaceFamily::ALL
        .iter()
        .map(|&family| row_from_family(family, family_spec(family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is resolved, used by the
/// blocked fixtures.
fn seeded_rows_with<F>(target: M5CommandSurfaceFamily, mutate: F) -> Vec<ResolverInspectorRow>
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

fn packet_from_rows(rows: Vec<ResolverInspectorRow>) -> ResolverInspectorPacket {
    build_m5_keybinding_resolver_inspectors_packet(ResolverInspectorInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_DISCOVERABILITY_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 keybinding resolver inspection packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV artifacts.
/// Six families keep full resolver-inspection, bridge-outcome, leader-sequence, and resolver-export truth
/// (green). The keybinding resolver layer auto-narrows to yellow disclosing a reduced inspector detail on
/// a constrained surface, the import-bridge row auto-narrows to yellow disclosing a partial bridge
/// coverage, the leader / sequence help overlay auto-narrows to yellow with a waivered reduced sequence
/// hint, and the command-documentation surface auto-narrows to yellow disclosing a partial resolver/export
/// capture — and no row is blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_keybinding_resolver_inspectors_packet() -> ResolverInspectorPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the keybinding resolver layer hides its winning or shadowed binding, proving
/// that a broken resolver inspection blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_keybinding_resolver_inspectors_packet_resolver_layer_hidden_binding_blocked(
) -> ResolverInspectorPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::KeybindingResolverLayer, |spec| {
        spec.resolver_inspection = ResolverInspectionState::WinningOrShadowedBindingHidden;
        spec.narrowing_reason = Some(
            "The keybinding resolver layer showed only the active binding and hid both the winning \
             source layer and the shadowed losers, so a user could not see which binding won, why it \
             won, or what lost without hidden resolver knowledge, and the layer blocks before keeping a \
             resolver-inspection claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the import-bridge row falls back to generic imported wording, proving that a
/// broken bridge outcome blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_keybinding_resolver_inspectors_packet_import_bridge_generic_wording_blocked(
) -> ResolverInspectorPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::ImportBridgeRow, |spec| {
        spec.bridge_outcome = BridgeOutcomeState::GenericImportedWordingUsed;
        spec.narrowing_reason = Some(
            "The import-bridge row labelled a translated binding with generic \"imported\" wording \
             instead of one of the controlled bridge-outcome states, so a user could not tell whether \
             the shortcut mapped exactly, was shimmed, or was unsupported, and the row blocks before \
             keeping a resolver-inspection claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the leader / sequence help overlay requires hidden knowledge to explain a
/// sequence's availability, proving that a broken leader-sequence inspection blocks a stable claim (red)
/// rather than staying green.
pub fn seeded_m5_keybinding_resolver_inspectors_packet_leader_hidden_knowledge_blocked(
) -> ResolverInspectorPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::LeaderSequenceHelp, |spec| {
        spec.leader_sequence_inspection =
            LeaderSequenceInspectionState::SequenceAvailabilityRequiresHiddenKnowledge;
        spec.waiver = None;
        spec.narrowing_reason = Some(
            "The leader / sequence help overlay showed no precedence, timeout, or narration hint for an \
             armed sequence, so a user could only tell why the sequence was or was not available with \
             hidden resolver knowledge, and the overlay blocks before keeping a resolver-inspection \
             claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the command-documentation surface's winning source is absent from the durable
/// export, proving that a broken resolver export blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_keybinding_resolver_inspectors_packet_documentation_export_absent_blocked(
) -> ResolverInspectorPacket {
    let rows = seeded_rows_with(
        M5CommandSurfaceFamily::CommandDocumentationSurface,
        |spec| {
            spec.resolver_export = ResolverExportState::WinningSourceAbsentFromCapture;
            spec.narrowing_reason = Some(
                "The command-documentation surface rendered the winning source only as a live badge that \
                 never reached the durable resolver/export packet, so a support bundle or migration \
                 packet could not reconstruct which binding won or its command id without a screenshot, \
                 and the surface blocks before keeping a resolver-inspection claim.",
            );
        },
    );
    packet_from_rows(rows)
}

/// Builds a variant where the conflict review sheet loses the shared resolution in a headless / CLI
/// execution, proving that a headless parity loss blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_keybinding_resolver_inspectors_packet_conflict_sheet_headless_parity_lost_blocked(
) -> ResolverInspectorPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::ConflictReviewSheet, |spec| {
        spec.headless_parity_preserved = false;
        spec.narrowing_reason = Some(
            "In headless / CLI execution the conflict review sheet resolved a chord to a different \
             winner than the in-product resolver, so the same conflict reported a different winning \
             source depending on how it ran, and the sheet blocks before keeping a resolver-inspection \
             claim.",
        );
    });
    packet_from_rows(rows)
}
