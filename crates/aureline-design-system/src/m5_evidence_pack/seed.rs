//! Canonical seed builder for the M5 component-gallery evidence pack.
//!
//! This builder is the single producer of the checked-in evidence fixtures (the pack file and one
//! file per component) and the release-packet proof. The headless emitter and the inline tests both
//! call it, so the in-code evidence, the schema fixtures, and the proof never drift.
//!
//! Each component's scenes are rendered from the checked-in
//! [host primitive](crate::m5_host_primitive::seeded_m5_host_primitive_library): one scene per
//! controlled state, copying the primitive's rendered parts, non-color cues, and interactivity, so
//! the evidence is reproducible from the same contract Aureline ships rather than from a manual
//! capture. The owning identity (component id, owner role) is taken from the
//! [component manifest](crate::m5_component_manifest::seeded_m5_component_manifest_package), and each
//! scene captures every appearance variant — normal themes, both high-contrast variants, the
//! reduced-motion posture, and two zoom levels — with a deterministic baseline digest.

use super::*;

use crate::m5_component_manifest::seeded_m5_component_manifest_package;
use crate::m5_host_primitive::{
    seeded_m5_host_primitive_library, M5HostPrimitive, M5StateRenderPlan,
};

/// Stable id of the canonical evidence pack.
pub const M5_EVIDENCE_PACK_ID: &str = "design-system:evidence-pack:core";

/// Version of the canonical evidence pack.
pub const M5_EVIDENCE_PACK_VERSION: &str = "1.0.0";

/// The evaluation date the stale-narrowed seed re-evaluates against, far enough past the staggered
/// capture dates that the older components fall outside their freshness window.
pub const M5_EVIDENCE_PACK_STALE_EVALUATED_AT: &str = "2026-09-14T00:00:00Z";

/// Mint timestamp pinned by the seed builder.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

/// As-of date the canonical pack evaluates freshness against. All components are current at this
/// date.
const CANONICAL_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

/// Freshness window every component's evidence is held to, in days.
const FRESHNESS_WINDOW_DAYS: u32 = 90;

const PACK_OWNER_ROLE: &str = "Design system owner";

/// Capture dates staggered by component kind (five days apart), so a single evaluation date produces
/// a realistic spread of evidence ages — and a later evaluation date narrows the oldest components'
/// claims first. Indexed by [`M5ComponentKind::ALL`] order.
const CAPTURED_AT: [&str; 7] = [
    "2026-06-26T00:00:00Z",
    "2026-06-21T00:00:00Z",
    "2026-06-16T00:00:00Z",
    "2026-06-11T00:00:00Z",
    "2026-06-06T00:00:00Z",
    "2026-06-01T00:00:00Z",
    "2026-05-27T00:00:00Z",
];

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_EVIDENCE_PACK_SCHEMA_REF.to_owned(),
        M5_EVIDENCE_PACK_DOC_REF.to_owned(),
        M5_EVIDENCE_PACK_PROOF_REF.to_owned(),
    ]
}

fn scene_id(kind: M5ComponentKind, state: CanonicalStateClass) -> String {
    format!(
        "design-system:evidence:{}:{}",
        kind.as_str(),
        state.as_str()
    )
}

fn status_id(kind: M5ComponentKind, state: CanonicalStateClass) -> String {
    format!(
        "{}{}.{}.status",
        M5_EVIDENCE_MESSAGE_ID_PREFIX,
        kind.as_str(),
        state.as_str()
    )
}

fn summary_id(kind: M5ComponentKind) -> String {
    format!("{}{}.summary", M5_EVIDENCE_MESSAGE_ID_PREFIX, kind.as_str())
}

fn keyboard_ref(scene_id: &str) -> String {
    format!(
        "{}evidence-pack.json#keyboard/{scene_id}",
        M5_EVIDENCE_PACK_DIR
    )
}

fn assistive_technology_ref(scene_id: &str) -> String {
    format!("{}evidence-pack.json#at/{scene_id}", M5_EVIDENCE_PACK_DIR)
}

fn baseline_capture_ref(scene_id: &str, variant: M5EvidenceVariantKind) -> String {
    format!(
        "{}#baseline/{scene_id}/{}",
        M5_EVIDENCE_PACK_PROOF_REF,
        variant.as_str()
    )
}

fn diff_artifact_ref(scene_id: &str, variant: M5EvidenceVariantKind) -> String {
    format!(
        "{}#diff/{scene_id}/{}",
        M5_EVIDENCE_PACK_PROOF_REF,
        variant.as_str()
    )
}

/// Builds every captured appearance variant for one scene, computing each variant's deterministic
/// baseline digest from the scene's canonical descriptor.
fn build_variants(
    component_id: &str,
    scene_id: &str,
    digest_input: &SceneDigestInput<'_>,
    interactive: bool,
) -> Vec<M5AppearanceVariantEvidence> {
    M5EvidenceVariantKind::ALL
        .iter()
        .copied()
        .map(|kind| M5AppearanceVariantEvidence {
            variant_kind: kind,
            theme_class: kind.theme_class(),
            motion_posture: kind.motion_posture(),
            zoom_percent: kind.zoom_percent(),
            baseline_digest: variant_baseline_digest(component_id, digest_input, kind),
            baseline_capture_ref: baseline_capture_ref(scene_id, kind),
            diff_artifact_ref: diff_artifact_ref(scene_id, kind),
            non_color_meaning_present: true,
            // A focus indicator is captured wherever the scene can receive focus.
            focus_visible: interactive,
        })
        .collect()
}

/// Builds one gallery scene from a host primitive's render plan for a single controlled state.
fn build_scene(
    kind: M5ComponentKind,
    component_id: &str,
    display_name: &str,
    plan: &M5StateRenderPlan,
) -> M5GalleryScene {
    let id = scene_id(kind, plan.state);
    let status = status_id(kind, plan.state);
    let rendered_parts = plan.rendered_parts.clone();
    let non_color_cues = plan.non_color_cues.clone();
    let interactive = plan.interactive;

    let digest_input = SceneDigestInput {
        state: plan.state,
        status_message_id: &status,
        rendered_parts: &rendered_parts,
        non_color_cues: &non_color_cues,
        interactive,
    };
    let variants = build_variants(component_id, &id, &digest_input, interactive);

    M5GalleryScene {
        scene_id: id.clone(),
        state: plan.state,
        mandatory: plan.mandatory,
        display_name: format!("{display_name} — {} state", plan.state.as_str()),
        rendered_parts,
        non_color_cues,
        status_message_id: status,
        interactive,
        keyboard_journey_ref: keyboard_ref(&id),
        assistive_technology_ref: assistive_technology_ref(&id),
        variants,
    }
}

/// Builds one component's evidence from its host primitive and owner role.
fn build_component(
    index: usize,
    primitive: &M5HostPrimitive,
    owner_role: &str,
) -> M5ComponentEvidence {
    let kind = primitive.component_kind;
    let component_id = primitive.component_id.clone();
    let display_name = primitive.display_name.clone();
    let scenes: Vec<M5GalleryScene> = primitive
        .state_render_plans
        .iter()
        .map(|plan| build_scene(kind, &component_id, &display_name, plan))
        .collect();

    let captured_at = CAPTURED_AT[index].to_owned();
    let freshness = evidence_freshness(&captured_at, CANONICAL_EVALUATED_AT, FRESHNESS_WINDOW_DAYS);

    let mut component = M5ComponentEvidence {
        component_kind: kind,
        component_id,
        primitive_id: primitive.primitive_id.clone(),
        owner_role: owner_role.to_owned(),
        display_name,
        captured_at,
        evaluated_at: CANONICAL_EVALUATED_AT.to_owned(),
        freshness_window_days: FRESHNESS_WINDOW_DAYS,
        freshness,
        // Filled in below from the freshness and computed coverage.
        claim_gate: M5EvidenceClaimGate::Certified,
        scenes,
        summary_message_id: summary_id(kind),
    };
    component.claim_gate = derive_claim_gate(component.freshness, component.coverage_complete());
    component
}

/// Builds the canonical M5 component-gallery evidence pack (version 1.0.0).
///
/// Renders one scene per controlled state for each launch-critical family from the checked-in host
/// primitive library, captures every appearance variant (normal themes, both high-contrast variants,
/// reduced motion, and two zoom levels) with a deterministic baseline digest, and attaches each
/// component's owning identity and computed freshness. Every component is current at the canonical
/// evaluation date, so every claim gate certifies.
pub fn seeded_m5_evidence_pack() -> M5EvidencePack {
    let library = seeded_m5_host_primitive_library();
    let manifests = seeded_m5_component_manifest_package();

    let components: Vec<M5ComponentEvidence> = M5ComponentKind::ALL
        .iter()
        .enumerate()
        .map(|(index, kind)| {
            let primitive = library
                .primitive(*kind)
                .expect("host primitive library publishes one primitive per kind");
            let manifest = manifests
                .manifest(*kind)
                .expect("component-manifest package publishes one manifest per kind");
            build_component(index, primitive, &manifest.lifecycle.owner_role)
        })
        .collect();

    M5EvidencePack {
        record_kind: M5_EVIDENCE_PACK_RECORD_KIND.to_owned(),
        schema_version: M5_EVIDENCE_PACK_SCHEMA_VERSION,
        pack_id: M5_EVIDENCE_PACK_ID.to_owned(),
        pack_version: M5_EVIDENCE_PACK_VERSION.to_owned(),
        owner_role: PACK_OWNER_ROLE.to_owned(),
        source_primitive_library_ref: crate::m5_host_primitive::M5_HOST_PRIMITIVE_LIBRARY_ID
            .to_owned(),
        source_manifest_package_ref: manifests.package_id.clone(),
        source_foundation_package_ref: crate::m5_foundation_package::M5_FOUNDATION_PACKAGE_ID
            .to_owned(),
        components,
        proof_lane_ref: M5_EVIDENCE_PACK_PROOF_REF.to_owned(),
        release_packet_ref: M5_EVIDENCE_PACK_RELEASE_PACKET_REF.to_owned(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        summary_message_id: format!(
            "{}{}.summary",
            M5_EVIDENCE_MESSAGE_ID_PREFIX, M5_EVIDENCE_PACK_ID
        ),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Builds the canonical pack re-evaluated at a later date, so the older components' evidence is stale
/// and their shell-quality claims auto-narrow while the freshly-captured components stay certified.
/// Exposed so consumers and tests can exercise the narrowing path from a checked-in seed.
pub fn seeded_m5_evidence_pack_stale_narrowed() -> M5EvidencePack {
    seeded_m5_evidence_pack().reevaluate(M5_EVIDENCE_PACK_STALE_EVALUATED_AT)
}
