//! Canonical seed builders for the M5 surface-qualification packet.
//!
//! These builders are the single producer of the checked-in qualification support export, the
//! published dashboard, the Markdown proof, and the stale / token-drift / missing-manifest /
//! waived drill fixtures. The headless emitter and the inline tests both call them so the in-code
//! packet, the artifacts, and the fixtures never drift. Every builder derives each surface's
//! verdict from the four checked-in lane packets, so the qualification is always generated from the
//! same contract Aureline ships: the canonical packet is all-qualified; the drills perturb one lane
//! input (re-evaluate evidence to a stale date, drift a token dependency, or drop a manifest) and
//! let the derivation recompute the status, gate, effective claim, and named gaps.

use super::*;

use crate::m5_component_manifest::seeded_m5_component_manifest_package;
use crate::m5_evidence_pack::{seeded_m5_evidence_pack, seeded_m5_evidence_pack_stale_narrowed};
use crate::m5_foundation_package::seeded_m5_foundation_package;
use crate::m5_reference_layout::seeded_m5_reference_layout_package;

/// Stable packet id for the canonical (all-qualified) qualification packet.
pub const M5_SURFACE_QUALIFICATION_PACKET_ID: &str = "m5-surface-qualification:stable:0001";

/// Evaluation / mint timestamp for the canonical packet — a date at which every component's
/// evidence is current.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

/// Evaluation / mint timestamp for the stale-narrowed drill — far enough past the staggered
/// capture dates that the older components' evidence falls outside its freshness window.
const STALE_EVALUATED_AT: &str = "2026-09-14T00:00:00Z";

/// The component family that the token-drift and missing-manifest drills perturb. It is bound by a
/// single surface, so the drill narrows / blocks exactly one surface.
const DRILL_COMPONENT: M5ComponentKind = M5ComponentKind::FormControl;

/// A foundation token id no foundation package publishes, used by the token-drift drill.
const UNRESOLVABLE_TOKEN: &str = "color.__unpublished__";

/// The claimed M5 workspace surfaces and the component families each renders. Together the bindings
/// cover every launch-critical component family, and at least two surfaces render only
/// freshly-captured families so they stay qualified when the older families go stale.
const SURFACE_BINDINGS: [(M5WorkspaceKind, &[M5ComponentKind]); 8] = [
    (
        M5WorkspaceKind::Notebook,
        &[
            M5ComponentKind::PlaceholderCard,
            M5ComponentKind::StateBlock,
            M5ComponentKind::ReviewSheet,
        ],
    ),
    (
        M5WorkspaceKind::DataGrid,
        &[
            M5ComponentKind::StateBlock,
            M5ComponentKind::DenseCollection,
            M5ComponentKind::FormControl,
        ],
    ),
    (
        M5WorkspaceKind::Profiler,
        &[
            M5ComponentKind::StateBlock,
            M5ComponentKind::DenseCollection,
            M5ComponentKind::JobRow,
        ],
    ),
    (
        M5WorkspaceKind::Pipeline,
        &[
            M5ComponentKind::JobRow,
            M5ComponentKind::StateBlock,
            M5ComponentKind::ReviewSheet,
        ],
    ),
    (
        M5WorkspaceKind::Docs,
        &[
            M5ComponentKind::PlaceholderCard,
            M5ComponentKind::StateBlock,
            M5ComponentKind::ReviewSheet,
        ],
    ),
    (
        M5WorkspaceKind::Preview,
        &[
            M5ComponentKind::BoundaryBar,
            M5ComponentKind::StateBlock,
            M5ComponentKind::JobRow,
        ],
    ),
    (
        M5WorkspaceKind::Incident,
        &[
            M5ComponentKind::JobRow,
            M5ComponentKind::ReviewSheet,
            M5ComponentKind::StateBlock,
        ],
    ),
    (
        M5WorkspaceKind::Companion,
        &[
            M5ComponentKind::BoundaryBar,
            M5ComponentKind::JobRow,
            M5ComponentKind::StateBlock,
        ],
    ),
];

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn surface_id(workspace: M5WorkspaceKind) -> String {
    format!("design-system-surface:{}", workspace.as_str())
}

fn workspace_display_name(workspace: M5WorkspaceKind) -> &'static str {
    match workspace {
        M5WorkspaceKind::Notebook => "Notebook",
        M5WorkspaceKind::DataGrid => "Data grid",
        M5WorkspaceKind::Profiler => "Profiler",
        M5WorkspaceKind::Pipeline => "Pipeline",
        M5WorkspaceKind::Docs => "Docs",
        M5WorkspaceKind::Preview => "Preview",
        M5WorkspaceKind::Incident => "Incident",
        M5WorkspaceKind::Companion => "Companion",
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SURFACE_QUALIFICATION_SCHEMA_REF,
        M5_SURFACE_QUALIFICATION_DASHBOARD_SCHEMA_REF,
        M5_FOUNDATION_PACKAGE_SCHEMA_REF,
        M5_COMPONENT_MANIFEST_SCHEMA_REF,
        M5_REFERENCE_LAYOUT_SCHEMA_REF,
        M5_EVIDENCE_PACK_SCHEMA_REF,
        M5_SURFACE_QUALIFICATION_DOC_REF,
        M5_SURFACE_QUALIFICATION_PROOF_REF,
    ])
}

fn conformance_review() -> M5QualificationConformanceReview {
    M5QualificationConformanceReview {
        every_surface_binds_all_four_lanes: true,
        every_surface_names_bound_component_families: true,
        token_state_conformance_computed_from_foundation: true,
        missing_contract_blocks_stable_promotion: true,
        stale_or_failing_conformance_auto_narrows_before_stable: true,
        waivers_disclosed_with_scope_owner_and_expiry: true,
        exact_gaps_named: true,
        dashboard_traffic_light_matches_rows: true,
        generated_from_checked_in_lane_contracts: true,
        support_export_carries_no_raw_boundary_material: true,
    }
}

fn consumer_projection() -> M5QualificationConsumerProjection {
    M5QualificationConsumerProjection {
        help_about_surfaces_qualification: true,
        release_center_gates_on_qualification: true,
        shiproom_watches_qualification_dashboard: true,
        support_export_ships_qualification: true,
        stable_claim_matrix_reads_effective_claim: true,
    }
}

fn lane_sources(inputs: &M5QualificationLaneInputs) -> Vec<M5QualificationLaneSource> {
    let source = |lane: M5QualificationLane, source_id: &str, source_version: &str| {
        M5QualificationLaneSource {
            lane,
            source_ref: lane.lane_proof_ref().to_owned(),
            schema_ref: lane.lane_schema_ref().to_owned(),
            source_id: source_id.to_owned(),
            source_version: source_version.to_owned(),
        }
    };
    vec![
        source(
            M5QualificationLane::Foundation,
            &inputs.foundation.package_id,
            &inputs.foundation.package_version,
        ),
        source(
            M5QualificationLane::ComponentContract,
            &inputs.manifests.package_id,
            &inputs.manifests.package_version,
        ),
        source(
            M5QualificationLane::ReferenceLayout,
            &inputs.layouts.package_id,
            &inputs.layouts.package_version,
        ),
        source(
            M5QualificationLane::Evidence,
            &inputs.evidence.pack_id,
            &inputs.evidence.pack_version,
        ),
    ]
}

/// Builds one claimed-surface qualification row and reconciles its derived fields against the four
/// lane packets.
fn build_surface(
    workspace: M5WorkspaceKind,
    kinds: &[M5ComponentKind],
    inputs: &M5QualificationLaneInputs,
) -> M5SurfaceQualification {
    let surface_id = surface_id(workspace);
    let mut surface = M5SurfaceQualification {
        surface_id: surface_id.clone(),
        workspace_kind: workspace,
        surface_label: format!("{} workspace surface", workspace_display_name(workspace)),
        owner_role: "Workspace surface owner".to_owned(),
        bound_component_kinds: kinds.to_vec(),
        claimed_class: M5DesignSystemClaimClass::Stable,
        effective_class: M5DesignSystemClaimClass::Stable,
        status: M5QualificationStatus::Qualified,
        signal: M5QualificationSignal::Green,
        gate_decision: M5QualificationGate::CertifiedPromote,
        lane_bindings: Vec::new(),
        waivers: Vec::new(),
        gaps: Vec::new(),
        consumer_surfaces: M5QualificationConsumer::ALL.to_vec(),
        status_message_id: format!(
            "{}{}.status",
            M5_QUALIFICATION_MESSAGE_ID_PREFIX, surface_id
        ),
        gate_message_id: format!("{}{}.gate", M5_QUALIFICATION_MESSAGE_ID_PREFIX, surface_id),
    };
    surface.recompute(inputs);
    surface
}

fn build_surfaces(inputs: &M5QualificationLaneInputs) -> Vec<M5SurfaceQualification> {
    SURFACE_BINDINGS
        .iter()
        .map(|(workspace, kinds)| build_surface(*workspace, kinds, inputs))
        .collect()
}

fn aggregate_release_gate(surfaces: &[M5SurfaceQualification]) -> M5QualificationReleaseGate {
    let collect = |predicate: &dyn Fn(&M5SurfaceQualification) -> bool| -> Vec<String> {
        let mut ids: Vec<String> = surfaces
            .iter()
            .filter(|s| predicate(s))
            .map(|s| s.surface_id.clone())
            .collect();
        ids.sort();
        ids
    };
    let blocked = collect(&|s| s.is_blocked());
    M5QualificationReleaseGate {
        blocks_stable_promotion: !blocked.is_empty(),
        blocked_surface_ids: blocked,
        auto_narrowed_surface_ids: collect(&|s| s.is_auto_narrowed()),
        qualified_surface_ids: collect(&|s| s.is_qualified()),
        waived_surface_ids: collect(&|s| !s.waivers.is_empty()),
        gate_message_id: format!("{}release_gate", M5_QUALIFICATION_MESSAGE_ID_PREFIX),
    }
}

fn build_packet(
    packet_id: &str,
    evaluated_at: &str,
    inputs: &M5QualificationLaneInputs,
    surfaces: Vec<M5SurfaceQualification>,
) -> M5SurfaceQualificationPacket {
    let release_gate = aggregate_release_gate(&surfaces);
    M5SurfaceQualificationPacket::new(M5SurfaceQualificationPacketInput {
        packet_id: packet_id.to_owned(),
        report_label: "M5 Surface-Qualification Packet".to_owned(),
        evaluated_at: evaluated_at.to_owned(),
        lane_sources: lane_sources(inputs),
        surfaces,
        vocabulary_set: M5QualificationVocabularySet::canonical(),
        conformance_review: conformance_review(),
        consumer_projection: consumer_projection(),
        release_gate,
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: evaluated_at.to_owned(),
    })
}

/// Builds the canonical all-qualified surface-qualification packet.
///
/// This is the single producer of the checked-in support export and dashboard: every claimed
/// surface binds current foundation tokens, published component contracts, a published reference
/// layout, and current evidence proof, so the release gate certifies every surface for Stable
/// promotion.
pub fn seeded_m5_surface_qualification_packet() -> M5SurfaceQualificationPacket {
    let foundation = seeded_m5_foundation_package();
    let manifests = seeded_m5_component_manifest_package();
    let layouts = seeded_m5_reference_layout_package();
    let evidence = seeded_m5_evidence_pack();
    let inputs = M5QualificationLaneInputs {
        foundation: &foundation,
        manifests: &manifests,
        layouts: &layouts,
        evidence: &evidence,
    };
    let surfaces = build_surfaces(&inputs);
    build_packet(
        M5_SURFACE_QUALIFICATION_PACKET_ID,
        SEED_EVALUATED_AT,
        &inputs,
        surfaces,
    )
}

/// Qualification packet evaluated at a later release date, so the surfaces that render the
/// older, freshly-staler component families auto-narrow below Stable while surfaces that render
/// only freshly-captured families stay qualified. Stale proof narrows but never blocks.
pub fn seeded_m5_surface_qualification_packet_stale_narrowed() -> M5SurfaceQualificationPacket {
    let foundation = seeded_m5_foundation_package();
    let manifests = seeded_m5_component_manifest_package();
    let layouts = seeded_m5_reference_layout_package();
    let evidence = seeded_m5_evidence_pack_stale_narrowed();
    let inputs = M5QualificationLaneInputs {
        foundation: &foundation,
        manifests: &manifests,
        layouts: &layouts,
        evidence: &evidence,
    };
    let surfaces = build_surfaces(&inputs);
    build_packet(
        "m5-surface-qualification:drill:stale-narrowed",
        STALE_EVALUATED_AT,
        &inputs,
        surfaces,
    )
}

/// Returns the canonical manifest package with one bound family's token dependencies drifted to
/// reference a foundation token the package does not publish.
fn manifests_with_token_drift() -> M5ComponentManifestPackage {
    let mut manifests = seeded_m5_component_manifest_package();
    if let Some(manifest) = manifests
        .manifests
        .iter_mut()
        .find(|m| m.component_kind == DRILL_COMPONENT)
    {
        manifest
            .token_dependencies
            .push(UNRESOLVABLE_TOKEN.to_owned());
    }
    manifests
}

/// Returns the canonical manifest package with one bound family's manifest dropped.
fn manifests_without_drill_component() -> M5ComponentManifestPackage {
    let mut manifests = seeded_m5_component_manifest_package();
    manifests
        .manifests
        .retain(|m| m.component_kind != DRILL_COMPONENT);
    manifests
}

/// Qualification packet where one surface's bound component contract names a foundation token the
/// package does not publish, so that surface's token/state conformance fails and it auto-narrows to
/// Beta before Stable promotion. Failing conformance narrows but never blocks.
pub fn seeded_m5_surface_qualification_packet_token_drift_narrowed() -> M5SurfaceQualificationPacket
{
    let foundation = seeded_m5_foundation_package();
    let manifests = manifests_with_token_drift();
    let layouts = seeded_m5_reference_layout_package();
    let evidence = seeded_m5_evidence_pack();
    let inputs = M5QualificationLaneInputs {
        foundation: &foundation,
        manifests: &manifests,
        layouts: &layouts,
        evidence: &evidence,
    };
    let surfaces = build_surfaces(&inputs);
    build_packet(
        "m5-surface-qualification:drill:token-drift-narrowed",
        SEED_EVALUATED_AT,
        &inputs,
        surfaces,
    )
}

/// Qualification packet where one bound component family has no published manifest, so the surface
/// that renders it is disqualified and blocked from Stable promotion — and named, not hidden.
pub fn seeded_m5_surface_qualification_packet_missing_manifest_blocked(
) -> M5SurfaceQualificationPacket {
    let foundation = seeded_m5_foundation_package();
    let manifests = manifests_without_drill_component();
    let layouts = seeded_m5_reference_layout_package();
    let evidence = seeded_m5_evidence_pack();
    let inputs = M5QualificationLaneInputs {
        foundation: &foundation,
        manifests: &manifests,
        layouts: &layouts,
        evidence: &evidence,
    };
    let surfaces = build_surfaces(&inputs);
    build_packet(
        "m5-surface-qualification:drill:missing-manifest-blocked",
        SEED_EVALUATED_AT,
        &inputs,
        surfaces,
    )
}

/// Qualification packet where a surface's missing-manifest gap is accepted under an active,
/// disclosed waiver, so the surface ships auto-narrowed to its waived claim while its true status
/// stays disqualified (red) and the gap is named as waived.
pub fn seeded_m5_surface_qualification_packet_waived_narrowed() -> M5SurfaceQualificationPacket {
    let foundation = seeded_m5_foundation_package();
    let manifests = manifests_without_drill_component();
    let layouts = seeded_m5_reference_layout_package();
    let evidence = seeded_m5_evidence_pack();
    let inputs = M5QualificationLaneInputs {
        foundation: &foundation,
        manifests: &manifests,
        layouts: &layouts,
        evidence: &evidence,
    };
    let mut surfaces = build_surfaces(&inputs);
    let data_grid = surfaces
        .iter_mut()
        .find(|s| s.workspace_kind == M5WorkspaceKind::DataGrid)
        .expect("data-grid surface present");
    data_grid.waivers.push(M5QualificationWaiver {
        waiver_id: "waiver:data-grid-form-control".to_owned(),
        gap_kind: M5QualificationGapKind::ComponentManifestMissing,
        subject: DRILL_COMPONENT.as_str().to_owned(),
        reason_message_id: format!(
            "{}{}.waiver.form_control",
            M5_QUALIFICATION_MESSAGE_ID_PREFIX, data_grid.surface_id
        ),
        owner_role: "Workspace surface owner".to_owned(),
        expires_at: "2026-09-26T00:00:00Z".to_owned(),
        narrowed_to: M5DesignSystemClaimClass::Preview,
    });
    data_grid.recompute(&inputs);
    build_packet(
        "m5-surface-qualification:drill:waived-narrowed",
        SEED_EVALUATED_AT,
        &inputs,
        surfaces,
    )
}
