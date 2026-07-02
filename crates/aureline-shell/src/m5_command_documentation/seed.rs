//! Canonical seed builders for the M5 command-documentation certification.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export, and CSV
//! artifacts plus the narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code documentation proof, the artifacts, and the fixtures never drift. Every attribute each family
//! row certifies over — the canonical command binding, the surface's qualification, owner, required labels,
//! lifecycle label, feature families, and declared consumer surfaces, and the applicable downgrade
//! triggers — is pulled straight from the frozen discoverability matrix's seeded packet, so the
//! certification cannot audit a surface the matrix does not anchor. Only the documentation-record fields,
//! parity cards, derivation anchors, the four documentation postures, and the scope summary are authored
//! here.

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

/// The documentation posture seeded for one surface family.
struct SurfaceSpec {
    /// Short conformance scope summary.
    scope_summary: &'static str,
    /// The documentation-record fields this row publishes (defaults to all eight).
    certified_doc_fields: Vec<M5CommandDocField>,
    /// The parity cards this row renders (defaults to all seven).
    certified_parity_cards: Vec<M5CommandParityCard>,
    /// The derivation anchors this row derives (defaults to all three).
    certified_derivation_anchors: Vec<M5DocDerivationAnchor>,
    /// When set, the evaluated-surface set used instead of the surface's declared set (blocked fixtures
    /// use this to prove a partial certification blocks).
    evaluated_surfaces_override: Option<Vec<M5DiscoveryChannel>>,
    documentation_record: DocumentationRecordState,
    cross_surface_naming: CrossSurfaceNamingState,
    example_freshness: ExampleFreshnessState,
    doc_export_parity: DocExportParityState,
    headless_parity_preserved: bool,
    waiver: Option<CommandDocWaiver>,
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

/// Builds one documentation row from a surface family and a posture. Every binding — the canonical command
/// binding, the surface's qualification, owner, required labels, lifecycle label, feature families, and
/// declared consumer surfaces, and the downgrade triggers — is pulled from the frozen matrix row for the
/// family.
fn row_from_family(family: M5CommandSurfaceFamily, spec: SurfaceSpec) -> CommandDocRow {
    let surface = matrix_surface_row(family);
    let required_consumer_surfaces = surface.consumer_surfaces.clone();
    let evaluated_consumer_surfaces = spec
        .evaluated_surfaces_override
        .unwrap_or_else(|| surface.consumer_surfaces.clone());
    let mut row = CommandDocRow {
        surface_family: family,
        surface_label: surface_label(family).to_owned(),
        qualification: surface.qualification,
        owner_role: surface.owner_role.clone(),
        scope_summary: spec.scope_summary.to_owned(),
        lifecycle_label: surface.canonical_command_binding.lifecycle_label,
        canonical_command_binding: surface.canonical_command_binding.clone(),
        required_labels: surface.required_labels.clone(),
        feature_families: surface.feature_families.clone(),
        certified_doc_fields: spec.certified_doc_fields,
        certified_parity_cards: spec.certified_parity_cards,
        certified_derivation_anchors: spec.certified_derivation_anchors,
        required_consumer_surfaces,
        evaluated_consumer_surfaces,
        documentation_record: spec.documentation_record,
        cross_surface_naming: spec.cross_surface_naming,
        example_freshness: spec.example_freshness,
        doc_export_parity: spec.doc_export_parity,
        headless_parity_preserved: spec.headless_parity_preserved,
        applicable_downgrade_triggers: surface.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        derived_status: CommandDocStatus::Green,
        conformance_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.conformance_causes = row.recompute_causes();
    row
}

/// Builds the cross-surface paraphrase waiver carried by the seed.
fn surface_paraphrase_waiver() -> CommandDocWaiver {
    CommandDocWaiver {
        waiver_id: "waiver:command-doc-surface-paraphrase:0001".to_owned(),
        surface_family: M5CommandSurfaceFamily::ContextMenu,
        reason:
            "On the space-constrained context menu one command renders a disclosed, waivered short \
             paraphrase of its canonical primary label — the compact affordance shortens the label while \
             the surface still points at the canonical command id, the same lifecycle / deprecation truth, \
             and the same replacement guidance, and the command-documentation surface and help pages keep \
             the full canonical label — so the naming is narrowed and disclosed rather than an invented \
             alternate label. The exception retires when the context menu renders the full canonical label \
             on every claimed family."
                .to_owned(),
        owner_role: "Shell/command-docs owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// A full-conformance posture: all four documentation dimensions hold, all eight documentation-record
/// fields, all seven parity cards, and all three derivation anchors are certified, and headless parity is
/// preserved.
fn full(scope_summary: &'static str) -> SurfaceSpec {
    SurfaceSpec {
        scope_summary,
        certified_doc_fields: M5CommandDocField::ALL.to_vec(),
        certified_parity_cards: M5CommandParityCard::ALL.to_vec(),
        certified_derivation_anchors: M5DocDerivationAnchor::ALL.to_vec(),
        evaluated_surfaces_override: None,
        documentation_record: DocumentationRecordState::CommandRecordExamplesAndLifecycleCertified,
        cross_surface_naming: CrossSurfaceNamingState::CanonicalNamingAndReplacementStable,
        example_freshness: ExampleFreshnessState::CanonicalExamplesFreshAndNotAliasOnly,
        doc_export_parity: DocExportParityState::CommandIdAndReplacementReconstructable,
        headless_parity_preserved: true,
        waiver: None,
        narrowing_reason: None,
    }
}

/// Returns the seeded documentation posture for one surface family.
fn family_spec(family: M5CommandSurfaceFamily) -> SurfaceSpec {
    use M5CommandSurfaceFamily as F;
    match family {
        F::MenuItem => full(
            "Menu-bar item documents the canonical command id, primary label, aliases, lifecycle / \
             deprecation state, supported surfaces, invocation-schema summary, side-effect / risk class, \
             and result / rollback semantics with fresh canonical examples and copy-safe export across \
             every consumer surface",
        ),
        F::MenuGroup => full(
            "Menu group documents each member's canonical command record with fresh examples, renders the \
             same parity cards, and reconstructs the command id and replacement guidance from durable \
             evidence across every consumer surface",
        ),
        F::KeybindingResolverLayer => full(
            "Keybinding resolver layer documents each command's canonical record and shortcut notation \
             derived from the shared record with fresh examples and copy-safe export across every consumer \
             surface",
        ),
        F::ConflictReviewSheet => full(
            "Conflict review sheet documents each conflicting command's canonical record, replacement \
             guidance, and fresh examples, and reconstructs the command id from durable evidence across \
             every consumer surface",
        ),
        F::DisabledCommandExplainer => full(
            "Disabled-command explainer documents the canonical command record, lifecycle / deprecation \
             truth, and replacement guidance even when the command is unavailable, with fresh examples and \
             copy-safe export across every consumer surface",
        ),
        F::LeaderSequenceHelp => full(
            "Leader / sequence help overlay documents each command's canonical record and shortcut \
             notation derived from the shared record with fresh examples and copy-safe export across every \
             consumer surface",
        ),
        // Command / action bar discloses a reduced documentation detail on a constrained surface (yellow).
        F::CommandBar => SurfaceSpec {
            documentation_record: DocumentationRecordState::DisclosedReducedDocDetail,
            narrowing_reason: Some(
                "On the constrained command / action bar the documentation record takes a disclosed \
                 reduced detail — the invocation-schema summary and side-effect / risk detail are folded \
                 into an expandable section while the command id, primary label, aliases, and lifecycle / \
                 deprecation truth stay visible — so the record is narrowed and disclosed rather than \
                 missing or mismatched.",
            ),
            ..full(
                "Command / action bar documents each command's canonical record across every consumer \
                 surface, folding the invocation-schema summary and side-effect detail into an expandable \
                 section on the constrained bar",
            )
        },
        // Context menu discloses a waivered surface paraphrase of the canonical label (yellow).
        F::ContextMenu => SurfaceSpec {
            cross_surface_naming: CrossSurfaceNamingState::DisclosedSurfaceParaphrase,
            waiver: Some(surface_paraphrase_waiver()),
            narrowing_reason: Some(
                "On the space-constrained context menu one command renders a disclosed, waivered short \
                 paraphrase of its canonical primary label while still pointing at the canonical command \
                 id, the same lifecycle / deprecation truth, and the same replacement guidance — so the \
                 naming is narrowed and disclosed rather than an invented alternate label.",
            ),
            ..full(
                "Context menu documents each focused object's canonical command record and replacement \
                 guidance across every consumer surface, rendering a disclosed short paraphrase of one \
                 label under a waivered exception",
            )
        },
        // Import-bridge row discloses a partial example refresh while migration examples catch up (yellow).
        F::ImportBridgeRow => SurfaceSpec {
            example_freshness: ExampleFreshnessState::DisclosedPartialExampleRefresh,
            narrowing_reason: Some(
                "One imported-binding example slice takes a disclosed partial refresh — the stale \
                 migration example is flagged and scheduled for refresh rather than presented as current — \
                 so the example freshness is narrowed and disclosed rather than shipping a stale or \
                 alias-only example unnoticed.",
            ),
            ..full(
                "Import-bridge row documents each translated command's canonical record and replacement \
                 guidance across every consumer surface, disclosing one stale migration-example slice \
                 while the refresh completes",
            )
        },
        // Command-documentation surface discloses a partial copy-safe export capture on a legacy export
        // (yellow).
        F::CommandDocumentationSurface => SurfaceSpec {
            doc_export_parity: DocExportParityState::DisclosedPartialCapture,
            narrowing_reason: Some(
                "On the legacy documentation export the copy-safe export surface takes a disclosed partial \
                 capture — the export captures the command id and replacement guidance but not the full \
                 alias list, while still disclosing the gap — so the copy-safe export parity is narrowed \
                 and disclosed rather than absent.",
            ),
            ..full(
                "Command-documentation surface documents the canonical command record without inventing a \
                 second naming system across every consumer surface, capturing everything but the full \
                 alias list on one legacy export",
            )
        },
    }
}

/// Builds the documentation rows for the canonical seed, one per surface family.
fn seeded_rows() -> Vec<CommandDocRow> {
    M5CommandSurfaceFamily::ALL
        .iter()
        .map(|&family| row_from_family(family, family_spec(family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is resolved, used by the
/// blocked fixtures.
fn seeded_rows_with<F>(target: M5CommandSurfaceFamily, mutate: F) -> Vec<CommandDocRow>
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

fn packet_from_rows(rows: Vec<CommandDocRow>) -> CommandDocPacket {
    build_m5_command_documentation_packet(CommandDocInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_DISCOVERABILITY_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 command-documentation packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV artifacts. Six
/// families keep full documentation-record, cross-surface-naming, example-freshness, and doc-export truth
/// (green). The command / action bar auto-narrows to yellow disclosing a reduced documentation detail on a
/// constrained surface, the context menu auto-narrows to yellow with a waivered surface paraphrase, the
/// import-bridge row auto-narrows to yellow disclosing a partial example refresh, and the
/// command-documentation surface auto-narrows to yellow disclosing a partial copy-safe export capture — and
/// no row is blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_command_documentation_packet() -> CommandDocPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the menu-bar item ships a documentation record that disagrees with the shipped
/// command record, proving that a mismatched record blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_command_documentation_packet_menu_item_record_mismatch_blocked() -> CommandDocPacket
{
    let rows = seeded_rows_with(M5CommandSurfaceFamily::MenuItem, |spec| {
        spec.documentation_record = DocumentationRecordState::DocRecordMissingOrMismatched;
        spec.narrowing_reason = Some(
            "The menu-bar item documented a lifecycle state and side-effect class that disagreed with the \
             shipped command record, so a reader could not trust the documented command truth, and the \
             item blocks before keeping a documentation claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the context menu invents an alternate label / drifts on replacement guidance,
/// proving that a naming drift blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_command_documentation_packet_context_menu_naming_drift_blocked() -> CommandDocPacket
{
    let rows = seeded_rows_with(M5CommandSurfaceFamily::ContextMenu, |spec| {
        spec.cross_surface_naming = CrossSurfaceNamingState::NamingOrReplacementDrifted;
        spec.waiver = None;
        spec.narrowing_reason = Some(
            "The context menu invented an alternate label for the command and pointed at a different \
             replacement command id than the help page, so the same command read with different names and \
             replacement guidance depending on the reach, and the menu blocks before keeping a \
             documentation claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the command-documentation surface ships a stale / alias-only example, proving
/// that a stale example blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_command_documentation_packet_documentation_surface_stale_example_blocked(
) -> CommandDocPacket {
    let rows = seeded_rows_with(
        M5CommandSurfaceFamily::CommandDocumentationSurface,
        |spec| {
            spec.example_freshness = ExampleFreshnessState::StaleOrAliasOnlyExampleShipped;
            spec.narrowing_reason = Some(
            "The command-documentation surface shipped a canonical example that quoted only a deprecated \
             alias instead of the canonical command id, so a reader could copy an alias-only invocation, \
             and the surface blocks before keeping a documentation claim.",
        );
        },
    );
    packet_from_rows(rows)
}

/// Builds a variant where the import-bridge row's command id / replacement guidance is absent from the
/// durable export, proving that an absent capture blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_command_documentation_packet_import_bridge_capture_absent_blocked(
) -> CommandDocPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::ImportBridgeRow, |spec| {
        spec.doc_export_parity = DocExportParityState::DocTruthAbsentFromCapture;
        spec.narrowing_reason = Some(
            "The import-bridge row rendered its replacement guidance only as a live badge that never \
             reached the durable, diffable documentation export, so a support bundle or migration packet \
             could not reconstruct the command id or replacement guidance without a screenshot, and the \
             row blocks before keeping a documentation claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the disabled-command explainer loses the shared documentation in a headless / CLI
/// execution, proving that a headless parity loss blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_command_documentation_packet_explainer_headless_parity_lost_blocked(
) -> CommandDocPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::DisabledCommandExplainer, |spec| {
        spec.headless_parity_preserved = false;
        spec.narrowing_reason = Some(
            "In headless / CLI execution the disabled-command explainer documented a different lifecycle \
             state and replacement guidance than the in-product surface, so the same command documented a \
             different record depending on how it ran, and the explainer blocks before keeping a \
             documentation claim.",
        );
    });
    packet_from_rows(rows)
}
