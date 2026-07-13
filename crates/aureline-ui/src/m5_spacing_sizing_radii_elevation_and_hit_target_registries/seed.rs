//! Canonical seed builders for the M5 spacing / sizing / radii / border / elevation geometry and
//! hit-target registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and
//! the fixtures never drift. Every resolved example is built by calling the real resolvers so the packet
//! can only carry projections the resolvers actually produce. Clean geometry and hit-target entries are
//! built so the canonical spacing / sizing / radii / border / elevation primitives, the density-aware
//! application, the overlay / dialog elevation hierarchy, and the minimum hit-target rules for interactive
//! controls and resize handles are proven across the shell, list / table, editor, dialog, review, and
//! support surfaces without any local geometry fork, sub-minimum hit target, broken elevation, or
//! raw-value inlining.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_GEOMETRY_HIT_TARGET_REGISTRIES_PACKET_ID: &str =
    "m5-spacing-sizing-radii-elevation-and-hit-target-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn geometry(input: M5GeometryEntryResolutionInput) -> M5ResolvedGeometryEntry {
    resolve_geometry_entry(input).expect("seed geometry entry resolves")
}

fn hit_target(input: M5HitTargetEntryResolutionInput) -> M5ResolvedHitTargetEntry {
    resolve_hit_target_entry(input).expect("seed hit-target entry resolves")
}

// -- Clean geometry entries ---------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn geometry_base(
    entry_id: &str,
    token_name: &str,
    geometry_role: M5GeometryRole,
    primitive_kind: M5GeometryPrimitiveKind,
    density_mode: M5GeometryDensityMode,
    elevation_tier: M5ElevationTier,
    surface_context: M5GeometrySurfaceContext,
) -> M5GeometryEntryResolutionInput {
    M5GeometryEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role: M5VisualSemanticRole::Neutral,
        geometry_role,
        primitive_kind,
        density_mode,
        elevation_tier,
        surface_context,
        density_aware: true,
        elevation_hierarchy_preserved: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn geo_spacing_clean() -> M5ResolvedGeometryEntry {
    geometry(geometry_base(
        "geometry:shell:spacing",
        "space.2",
        M5GeometryRole::SpacingStep,
        M5GeometryPrimitiveKind::Spacing,
        M5GeometryDensityMode::Standard,
        M5ElevationTier::Base,
        M5GeometrySurfaceContext::Shell,
    ))
}

fn geo_sizing_clean() -> M5ResolvedGeometryEntry {
    geometry(geometry_base(
        "geometry:list-table:sizing",
        "size.row",
        M5GeometryRole::SizingStep,
        M5GeometryPrimitiveKind::Sizing,
        M5GeometryDensityMode::Compact,
        M5ElevationTier::Base,
        M5GeometrySurfaceContext::ListTable,
    ))
}

fn geo_radius_clean() -> M5ResolvedGeometryEntry {
    geometry(geometry_base(
        "geometry:editor:radius",
        "radius.control",
        M5GeometryRole::RadiusStep,
        M5GeometryPrimitiveKind::Radius,
        M5GeometryDensityMode::Standard,
        M5ElevationTier::Base,
        M5GeometrySurfaceContext::Editor,
    ))
}

fn geo_border_clean() -> M5ResolvedGeometryEntry {
    geometry(geometry_base(
        "geometry:review:border",
        "border.hairline",
        M5GeometryRole::SizingStep,
        M5GeometryPrimitiveKind::Border,
        M5GeometryDensityMode::Comfortable,
        M5ElevationTier::Base,
        M5GeometrySurfaceContext::Review,
    ))
}

fn geo_elevation_clean() -> M5ResolvedGeometryEntry {
    geometry(geometry_base(
        "geometry:dialog:elevation",
        "elevation.dialog",
        M5GeometryRole::ElevationLevel,
        M5GeometryPrimitiveKind::Elevation,
        M5GeometryDensityMode::Standard,
        M5ElevationTier::Dialog,
        M5GeometrySurfaceContext::Dialog,
    ))
}

// -- Degraded geometry entries ------------------------------------------------------------------

/// Degraded geometry entry: the geometry role forks from the shared foundation.
fn geo_forked() -> M5ResolvedGeometryEntry {
    geometry(geometry_base(
        "geometry:shell:forked",
        "space.local",
        M5GeometryRole::LocalGeometryForkDisallowed,
        M5GeometryPrimitiveKind::Spacing,
        M5GeometryDensityMode::Standard,
        M5ElevationTier::Base,
        M5GeometrySurfaceContext::Shell,
    ))
}

/// Degraded geometry entry: the primitive is not density-aware.
fn geo_not_density_aware() -> M5ResolvedGeometryEntry {
    let mut input = geometry_base(
        "geometry:review:not-density-aware",
        "border.hairline",
        M5GeometryRole::SizingStep,
        M5GeometryPrimitiveKind::Border,
        M5GeometryDensityMode::Comfortable,
        M5ElevationTier::Base,
        M5GeometrySurfaceContext::Review,
    );
    input.density_aware = false;
    geometry(input)
}

/// Degraded geometry entry: an elevation primitive loses the overlay / dialog hierarchy.
fn geo_elevation_broken() -> M5ResolvedGeometryEntry {
    let mut input = geometry_base(
        "geometry:dialog:elevation-broken",
        "elevation.dialog",
        M5GeometryRole::ElevationLevel,
        M5GeometryPrimitiveKind::Elevation,
        M5GeometryDensityMode::Standard,
        M5ElevationTier::Dialog,
        M5GeometrySurfaceContext::Dialog,
    );
    input.elevation_hierarchy_preserved = false;
    geometry(input)
}

/// Degraded geometry entry: a raw geometry value is inlined instead of tracing to a canonical token.
fn geo_raw_inlined() -> M5ResolvedGeometryEntry {
    let mut input = geometry_base(
        "geometry:support:raw-inlined",
        "size.row",
        M5GeometryRole::SpacingStep,
        M5GeometryPrimitiveKind::Spacing,
        M5GeometryDensityMode::Standard,
        M5ElevationTier::Base,
        M5GeometrySurfaceContext::ListTable,
    );
    input.references_canonical_token = false;
    geometry(input)
}

/// Degraded geometry entry: the primitive kind is unstated.
fn geo_primitive_unstated() -> M5ResolvedGeometryEntry {
    geometry(geometry_base(
        "geometry:support:kind-unstated",
        "space.2",
        M5GeometryRole::SpacingStep,
        M5GeometryPrimitiveKind::KindUnknown,
        M5GeometryDensityMode::Standard,
        M5ElevationTier::Base,
        M5GeometrySurfaceContext::ListTable,
    ))
}

// -- Clean hit-target entries -------------------------------------------------------------------

fn hit_target_base(
    entry_id: &str,
    token_name: &str,
    hit_target_rule: M5HitTargetRule,
    control_kind: M5HitTargetControlKind,
    density_mode: M5GeometryDensityMode,
    surface_context: M5GeometrySurfaceContext,
) -> M5HitTargetEntryResolutionInput {
    M5HitTargetEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role: M5VisualSemanticRole::Interactive,
        hit_target_rule,
        control_kind,
        density_mode,
        surface_context,
        meets_supported_minimum: true,
        adequate_target_spacing: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn hit_button_shell_clean() -> M5ResolvedHitTargetEntry {
    hit_target(hit_target_base(
        "hit-target:shell:button",
        "target.comfortable",
        M5HitTargetRule::ComfortableMinimum,
        M5HitTargetControlKind::Button,
        M5GeometryDensityMode::Comfortable,
        M5GeometrySurfaceContext::Shell,
    ))
}

fn hit_row_compact_clean() -> M5ResolvedHitTargetEntry {
    hit_target(hit_target_base(
        "hit-target:list-table:row",
        "target.compact",
        M5HitTargetRule::CompactMinimum,
        M5HitTargetControlKind::MenuItem,
        M5GeometryDensityMode::Compact,
        M5GeometrySurfaceContext::ListTable,
    ))
}

fn hit_resize_editor_clean() -> M5ResolvedHitTargetEntry {
    hit_target(hit_target_base(
        "hit-target:editor:resize-handle",
        "target.pointer_coarse",
        M5HitTargetRule::PointerCoarseMinimum,
        M5HitTargetControlKind::ResizeHandle,
        M5GeometryDensityMode::Standard,
        M5GeometrySurfaceContext::Editor,
    ))
}

// -- Degraded hit-target entries ----------------------------------------------------------------

/// Degraded hit-target entry: a compact-density icon button shrinks below the supported minimum.
fn hit_shrinks_below_minimum() -> M5ResolvedHitTargetEntry {
    let mut input = hit_target_base(
        "hit-target:list-table:shrunk",
        "target.compact",
        M5HitTargetRule::CompactMinimum,
        M5HitTargetControlKind::IconButton,
        M5GeometryDensityMode::Compact,
        M5GeometrySurfaceContext::ListTable,
    );
    input.meets_supported_minimum = false;
    hit_target(input)
}

/// Degraded hit-target entry: the spacing between adjacent targets is inadequate.
fn hit_inadequate_spacing() -> M5ResolvedHitTargetEntry {
    let mut input = hit_target_base(
        "hit-target:editor:cramped",
        "target.spacing",
        M5HitTargetRule::SpacingBetweenTargets,
        M5HitTargetControlKind::Toggle,
        M5GeometryDensityMode::Standard,
        M5GeometrySurfaceContext::Editor,
    );
    input.adequate_target_spacing = false;
    hit_target(input)
}

/// Degraded hit-target entry: a raw geometry value is inlined instead of tracing to a canonical token.
fn hit_raw_inlined() -> M5ResolvedHitTargetEntry {
    let mut input = hit_target_base(
        "hit-target:support:raw-inlined",
        "target.comfortable",
        M5HitTargetRule::ComfortableMinimum,
        M5HitTargetControlKind::Button,
        M5GeometryDensityMode::Comfortable,
        M5GeometrySurfaceContext::Shell,
    );
    input.references_canonical_token = false;
    hit_target(input)
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5GeometryConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5VisualFoundationDowngradeTrigger>,
    geometry_entries: Vec<M5ResolvedGeometryEntry>,
    hit_target_entries: Vec<M5ResolvedHitTargetEntry>,
) -> M5GeometryHitTargetRegistriesRow {
    M5GeometryHitTargetRegistriesRow {
        consumer_surface,
        qualification: M5VisualFoundationQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5VisualFoundationDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5VisualFoundationRequiredLabel::Identity,
            M5VisualFoundationRequiredLabel::SemanticRole,
            M5VisualFoundationRequiredLabel::TokenReference,
            M5VisualFoundationRequiredLabel::DensityContext,
        ],
        accessibility_routes: M5VisualFoundationAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5GeometryAnatomyPart::ALL.to_vec(),
        export_fields: M5GeometryExportField::ALL.to_vec(),
        downgrade_triggers,
        geometry_entries,
        hit_target_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_GEOMETRY_HIT_TARGET_REGISTRIES_SCHEMA_REF,
            M5_TYPOGRAPHY_AND_GEOMETRY_SCHEMA_REF,
        ]),
        local_geometry_forked_from_foundation: false,
        hit_target_shrunk_below_minimum: false,
        elevation_hierarchy_broken: false,
        raw_geometry_value_inlined_instead_of_token: false,
    }
}

fn registry_rows() -> Vec<M5GeometryHitTargetRegistriesRow> {
    use M5VisualFoundationConsumerSurface as C;
    use M5VisualFoundationDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell spaces chrome on the canonical spacing step and sizes buttons to the comfortable minimum target; a spacing step that forks the shared foundation degrades honestly instead of reading as a clean pass",
            "evidence:m5-geometry-hit-target-shell-ui:001",
            vec![
                D::LocalGeometryForkedFromFoundation,
                D::SemanticRoleUnstated,
                D::ProofStale,
            ],
            vec![geo_spacing_clean(), geo_forked()],
            vec![hit_button_shell_clean()],
        ),
        base_row(
            C::DataUi,
            "List / table surface owner",
            "The dense list / table sizes rows on the canonical sizing step and keeps compact-density menu / row targets at the compact minimum; an icon button that shrinks below the supported minimum under compact density degrades honestly",
            "evidence:m5-geometry-hit-target-list-table-ui:001",
            vec![
                D::HitTargetShrunkBelowMinimum,
                D::LocalGeometryForkedFromFoundation,
                D::ProofStale,
            ],
            vec![geo_sizing_clean()],
            vec![hit_row_compact_clean(), hit_shrinks_below_minimum()],
        ),
        base_row(
            C::EditorUi,
            "Editor surface owner",
            "The editor rounds controls on the canonical radius step and keeps resize handles at the coarse-pointer minimum; a toggle with inadequate spacing between adjacent targets degrades honestly",
            "evidence:m5-geometry-hit-target-editor-ui:001",
            vec![
                D::HitTargetShrunkBelowMinimum,
                D::LocalGeometryForkedFromFoundation,
                D::ProofStale,
            ],
            vec![geo_radius_clean()],
            vec![hit_resize_editor_clean(), hit_inadequate_spacing()],
        ),
        base_row(
            C::SettingsUi,
            "Dialog / overlay surface owner",
            "The dialog host elevates modals on the canonical elevation level so overlays and dialogs stay above base content; an elevation entry that loses the intended hierarchy degrades honestly",
            "evidence:m5-geometry-hit-target-dialog-ui:001",
            vec![
                D::LocalGeometryForkedFromFoundation,
                D::SemanticRoleUnstated,
                D::ProofStale,
            ],
            vec![geo_elevation_clean(), geo_elevation_broken()],
            vec![],
        ),
        base_row(
            C::ReviewUi,
            "Review surface owner",
            "The review surface draws borders on the canonical hairline step and stays density-aware across compact / standard / comfortable modes; a border that applies one geometry regardless of density degrades honestly",
            "evidence:m5-geometry-hit-target-review-ui:001",
            vec![
                D::LocalGeometryForkedFromFoundation,
                D::SemanticRoleUnstated,
                D::ProofStale,
            ],
            vec![geo_border_clean(), geo_not_density_aware()],
            vec![],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved geometry and hit-target truth, so a raw-value regression, an unstated primitive kind, and a raw-layout hit target are visible in evidence rather than hidden behind rendering",
            "evidence:m5-geometry-hit-target-support-export:001",
            vec![
                D::TokenReferenceUnstated,
                D::SemanticRoleUnstated,
                D::ProofStale,
            ],
            vec![geo_raw_inlined(), geo_primitive_unstated()],
            vec![hit_raw_inlined()],
        ),
    ]
}

fn governance_review() -> M5GeometryHitTargetGovernanceReview {
    M5GeometryHitTargetGovernanceReview {
        one_canonical_geometry_across_surfaces: true,
        spacing_sizing_radii_border_elevation_primitives_shared: true,
        density_aware_application_holds: true,
        compact_density_preserves_hit_target_minima: true,
        overlays_and_dialogs_preserve_elevation_hierarchy: true,
        resize_handles_meet_minimum_targets: true,
        geometry_drift_caught_before_release: true,
        raw_geometry_value_drift_caught_before_release: true,
        first_consumers_use_canonical_geometry: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5GeometryHitTargetConsumerProjection {
    M5GeometryHitTargetConsumerProjection {
        shell_and_list_table_consume_shared_geometry: true,
        editor_consumes_shared_geometry: true,
        dialog_consumes_elevation_hierarchy: true,
        review_consumes_shared_geometry: true,
        geometry_meaning_traces_to_single_domain_contract: true,
        support_export_reads_single_geometry_source: true,
    }
}

fn proof_freshness() -> M5GeometryHitTargetProofFreshness {
    M5GeometryHitTargetProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5GeometryHitTargetReleasePosture {
    M5GeometryHitTargetReleasePosture {
        proof_packet_ref: M5_GEOMETRY_HIT_TARGET_REGISTRIES_ARTIFACT_REF.to_owned(),
        foundation_audit_ref: M5_GEOMETRY_HIT_TARGET_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_GEOMETRY_HIT_TARGET_REGISTRIES_SCHEMA_REF,
        M5_GEOMETRY_HIT_TARGET_REGISTRIES_DOC_REF,
        M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF,
        M5_VISUAL_FOUNDATION_MATRIX_DOC_REF,
        M5_TYPOGRAPHY_AND_GEOMETRY_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 geometry and hit-target registries packet.
pub fn seeded_m5_geometry_hit_target_registries() -> M5GeometryHitTargetRegistriesPacket {
    M5GeometryHitTargetRegistriesPacket::new(M5GeometryHitTargetRegistriesPacketInput {
        packet_id: M5_GEOMETRY_HIT_TARGET_REGISTRIES_PACKET_ID.to_owned(),
        registries_label:
            "M5 spacing / sizing / radii / border / elevation geometry and minimum hit-target registries with canonical density-aware primitives, an overlay / dialog elevation hierarchy, and minimum-target rules for interactive controls and resize handles across the shell, list / table, editor, dialog, review, and support surfaces"
                .to_owned(),
        registry_rows: registry_rows(),
        vocabulary_set: M5GeometryHitTargetVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the shell-UI row is held at Beta pending spacing-step proof on every deployment line;
/// every row stays visible and every example stays honest.
pub fn seeded_m5_geometry_hit_target_registries_shell_ui_beta_narrowed(
) -> M5GeometryHitTargetRegistriesPacket {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    packet.packet_id =
        "m5-spacing-sizing-radii-elevation-and-hit-target-registries:shell-ui-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5VisualFoundationConsumerSurface::ShellUi)
        .expect("shell-ui row present");
    row.qualification = M5VisualFoundationQualificationClass::Beta;
    packet
}

/// Narrowed variant: the data-UI (list / table) row is narrowed to Preview pending compact-minimum parity
/// on every dense table; every row stays visible and every example stays honest.
pub fn seeded_m5_geometry_hit_target_registries_data_ui_preview_narrowed(
) -> M5GeometryHitTargetRegistriesPacket {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    packet.packet_id =
        "m5-spacing-sizing-radii-elevation-and-hit-target-registries:data-ui-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5VisualFoundationConsumerSurface::DataUi)
        .expect("data-ui row present");
    row.qualification = M5VisualFoundationQualificationClass::Preview;
    packet
}
