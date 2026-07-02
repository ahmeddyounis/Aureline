//! Canonical seed builders for the M5 discoverability-affordance-parity certification.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export, and CSV
//! artifacts plus the narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code parity proof, the artifacts, and the fixtures never drift. Every attribute each affordance row
//! certifies over — the canonical command binding, the driving surface's qualification, owner, required
//! labels, lifecycle label, preview class, feature families, and declared consumer surfaces, and the
//! applicable downgrade triggers — is pulled straight from the frozen discoverability matrix's seeded packet
//! for the surface family the affordance drives, so the certification cannot audit an affordance the matrix
//! does not anchor. Only the canonical record fields, reach modes, the four parity postures, and the scope
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

/// The parity posture seeded for one convenience affordance.
struct AffordanceSpec {
    /// Short conformance scope summary.
    scope_summary: &'static str,
    /// The canonical record fields this affordance reuses (defaults to all six).
    certified_record_fields: Vec<M5AffordanceRecordField>,
    /// The reach modes this affordance stays reachable in (defaults to all five).
    certified_reach_modes: Vec<M5AffordanceReachMode>,
    /// When set, the evaluated-surface set used instead of the driving surface's declared set (blocked
    /// fixtures use this to prove a partial certification blocks).
    evaluated_surfaces_override: Option<Vec<M5DiscoveryChannel>>,
    label_reuse: LabelReuseState,
    side_effect_truth: SideEffectTruthState,
    authority_reach: AuthorityReachState,
    origin_export: OriginExportState,
    headless_parity_preserved: bool,
    waiver: Option<AffordanceParityWaiver>,
    narrowing_reason: Option<&'static str>,
}

/// Short reviewer-facing label for a convenience affordance.
fn affordance_label(affordance: M5ConvenienceAffordance) -> &'static str {
    match affordance {
        M5ConvenienceAffordance::Button => "Primary / action button",
        M5ConvenienceAffordance::InlineAffordance => "Inline quick-action affordance",
        M5ConvenienceAffordance::Tooltip => "Tooltip / hovercard",
        M5ConvenienceAffordance::OnboardingTip => "Onboarding tip",
        M5ConvenienceAffordance::AiHint => "AI hint",
        M5ConvenienceAffordance::VoiceHint => "Voice hint",
        M5ConvenienceAffordance::CompanionHandoff => "Companion / browser handoff",
    }
}

/// Returns the frozen matrix surface row for a surface family.
fn matrix_surface_row(surface_family: M5CommandSurfaceFamily) -> M5DiscoverabilitySurfaceRow {
    seeded_m5_discoverability_matrix()
        .surface_rows
        .into_iter()
        .find(|row| row.surface_family == surface_family)
        .expect("frozen discoverability matrix declares every governed surface family")
}

/// Builds one parity row from a convenience affordance and a posture. Every binding — the canonical command
/// binding, the driving surface's qualification, owner, required labels, lifecycle label, preview class,
/// feature families, and declared consumer surfaces, and the downgrade triggers — is pulled from the frozen
/// matrix row for the surface family the affordance drives.
fn row_from_affordance(
    affordance: M5ConvenienceAffordance,
    spec: AffordanceSpec,
) -> AffordanceParityRow {
    let driving_surface_family = affordance.driving_surface_family();
    let surface = matrix_surface_row(driving_surface_family);
    let required_consumer_surfaces = surface.consumer_surfaces.clone();
    let evaluated_consumer_surfaces = spec
        .evaluated_surfaces_override
        .unwrap_or_else(|| surface.consumer_surfaces.clone());
    let mut row = AffordanceParityRow {
        affordance,
        affordance_label: affordance_label(affordance).to_owned(),
        driving_surface_family,
        qualification: surface.qualification,
        owner_role: surface.owner_role.clone(),
        scope_summary: spec.scope_summary.to_owned(),
        lifecycle_label: surface.canonical_command_binding.lifecycle_label,
        preview_class: surface.canonical_command_binding.preview_class,
        disabled_reason_mode: surface.canonical_command_binding.disabled_reason_mode,
        canonical_command_binding: surface.canonical_command_binding.clone(),
        required_labels: surface.required_labels.clone(),
        feature_families: surface.feature_families.clone(),
        certified_record_fields: spec.certified_record_fields,
        certified_reach_modes: spec.certified_reach_modes,
        required_consumer_surfaces,
        evaluated_consumer_surfaces,
        label_reuse: spec.label_reuse,
        side_effect_truth: spec.side_effect_truth,
        authority_reach: spec.authority_reach,
        origin_export: spec.origin_export,
        headless_parity_preserved: spec.headless_parity_preserved,
        applicable_downgrade_triggers: surface.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        derived_status: AffordanceParityStatus::Green,
        conformance_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.conformance_causes = row.recompute_causes();
    row
}

/// Builds the authority-reach reduced-hover-fallback waiver carried by the seed.
fn reduced_hover_fallback_waiver() -> AffordanceParityWaiver {
    AffordanceParityWaiver {
        waiver_id: "waiver:affordance-parity-reduced-hover:0001".to_owned(),
        affordance: M5ConvenienceAffordance::CompanionHandoff,
        reason:
            "On a touch / narrow companion surface the desktop hover affordance falls back to a disclosed, \
             waivered reduced form — the hovercard detail collapses into a tap-to-open sheet while the \
             companion keeps a keyboard-focus and context-action equivalent, and the companion hint still \
             names the same canonical command id and stays within the desktop command's authority — so the \
             reach is narrowed and disclosed rather than hover-only. The exception retires when the \
             companion renders the full hovercard equivalent on every touch surface."
                .to_owned(),
        owner_role: "Shell/companion owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// A full-conformance posture: all four parity dimensions hold, all six canonical record fields and all five
/// reach modes are certified, and headless parity is preserved.
fn full(scope_summary: &'static str) -> AffordanceSpec {
    AffordanceSpec {
        scope_summary,
        certified_record_fields: M5AffordanceRecordField::ALL.to_vec(),
        certified_reach_modes: M5AffordanceReachMode::ALL.to_vec(),
        evaluated_surfaces_override: None,
        label_reuse: LabelReuseState::CanonicalLabelAliasAndLifecycleReused,
        side_effect_truth: SideEffectTruthState::SideEffectAndPreviewTruthPreserved,
        authority_reach: AuthorityReachState::FocusEquivalentAndBoundedAuthority,
        origin_export: OriginExportState::OriginCommandIdentityReconstructable,
        headless_parity_preserved: true,
        waiver: None,
        narrowing_reason: None,
    }
}

/// Returns the seeded parity posture for one convenience affordance.
fn affordance_spec(affordance: M5ConvenienceAffordance) -> AffordanceSpec {
    use M5ConvenienceAffordance as A;
    match affordance {
        A::Button => full(
            "Primary / action button reuses the canonical label, alias, shortcut hint, and lifecycle badge, \
             preserves the side-effect and preview / approval requirement, keeps a keyboard-focus and \
             context-action equivalent within the canonical authority, and reconstructs its originating \
             command id from durable evidence across every consumer surface and reach mode",
        ),
        A::OnboardingTip => full(
            "Onboarding tip references the canonical command id, label, alias, and lifecycle badge rather \
             than teaching a convenience name, keeps the side-effect and preview truth, stays reachable in \
             every reach mode, and reconstructs its originating command id from durable evidence",
        ),
        A::AiHint => full(
            "AI hint string quotes the canonical command id, label, alias, shortcut hint, and lifecycle \
             badge, preserves the side-effect class and preview / approval requirement, cannot imply a \
             stronger action than the canonical authority, and reconstructs its originating command id from \
             durable evidence across every consumer surface",
        ),
        // Inline affordance discloses a shortened label on a constrained card (yellow).
        A::InlineAffordance => AffordanceSpec {
            label_reuse: LabelReuseState::DisclosedShortenedAffordanceLabel,
            narrowing_reason: Some(
                "On the space-constrained inline quick-action card the label renders a disclosed shortened \
                 form while the card still links the canonical command id, alias set, shortcut hint, and \
                 lifecycle badge — so the label is narrowed and disclosed rather than an invented \
                 convenience-specific label.",
            ),
            ..full(
                "Inline quick-action affordance projects the canonical record across every consumer surface, \
                 rendering a disclosed shortened label on the constrained card while keeping the canonical \
                 id, alias, shortcut hint, and lifecycle badge linked",
            )
        },
        // Tooltip discloses a summarized side-effect note on a constrained hovercard (yellow).
        A::Tooltip => AffordanceSpec {
            side_effect_truth: SideEffectTruthState::DisclosedSummarizedSideEffectNote,
            narrowing_reason: Some(
                "On the constrained tooltip / hovercard the full side-effect prose is folded into a \
                 disclosed summary while the preview / approval requirement and side-effect class stay \
                 visible — so the side-effect truth is narrowed and disclosed rather than softened into a \
                 one-tap convenience.",
            ),
            ..full(
                "Tooltip / hovercard reuses the canonical record across every consumer surface, folding the \
                 full side-effect prose into a disclosed summary while the preview / approval requirement \
                 stays visible on the constrained hovercard",
            )
        },
        // Voice hint discloses a partial copy-safe export capture on a legacy export (yellow).
        A::VoiceHint => AffordanceSpec {
            origin_export: OriginExportState::DisclosedPartialCapture,
            narrowing_reason: Some(
                "On the legacy voice-transcript export the copy-safe origin export takes a disclosed partial \
                 capture — the export captures the affordance and command id but not the full canonical \
                 record, while still disclosing the gap — so the origin-export parity is narrowed and \
                 disclosed rather than absent.",
            ),
            ..full(
                "Voice hint string reuses the canonical record across every consumer surface, capturing the \
                 affordance and command id but not the full canonical record on one legacy voice-transcript \
                 export",
            )
        },
        // Companion handoff discloses a waivered reduced hover fallback on touch surfaces (yellow).
        A::CompanionHandoff => AffordanceSpec {
            authority_reach: AuthorityReachState::DisclosedReducedHoverFallback,
            waiver: Some(reduced_hover_fallback_waiver()),
            narrowing_reason: Some(
                "On a touch / narrow companion surface the desktop hover affordance falls back to a \
                 disclosed, waivered reduced form — the hovercard detail collapses into a tap-to-open sheet \
                 while the companion keeps a keyboard-focus and context-action equivalent and still names \
                 the same canonical command id within the desktop command's authority — so the reach is \
                 narrowed and disclosed rather than hover-only.",
            ),
            ..full(
                "Companion / browser handoff reuses the canonical record across every consumer surface, \
                 falling back to a disclosed, waivered reduced hover form on touch surfaces while keeping a \
                 keyboard-focus and context-action equivalent and staying within the desktop command's \
                 authority",
            )
        },
    }
}

/// Builds the parity rows for the canonical seed, one per convenience affordance.
fn seeded_rows() -> Vec<AffordanceParityRow> {
    M5ConvenienceAffordance::ALL
        .iter()
        .map(|&affordance| row_from_affordance(affordance, affordance_spec(affordance)))
        .collect()
}

/// Builds a variant where one affordance's spec is mutated after the canonical spec is resolved, used by the
/// blocked fixtures.
fn seeded_rows_with<F>(target: M5ConvenienceAffordance, mutate: F) -> Vec<AffordanceParityRow>
where
    F: Fn(&mut AffordanceSpec),
{
    M5ConvenienceAffordance::ALL
        .iter()
        .map(|&affordance| {
            let mut spec = affordance_spec(affordance);
            if affordance == target {
                mutate(&mut spec);
            }
            row_from_affordance(affordance, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<AffordanceParityRow>) -> AffordanceParityPacket {
    build_m5_affordance_parity_packet(AffordanceParityInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_DISCOVERABILITY_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 discoverability-affordance-parity packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV artifacts.
/// Three affordances keep full label-reuse, side-effect-truth, authority-reach, and origin-export truth
/// (green). The inline affordance auto-narrows to yellow disclosing a shortened label, the tooltip
/// auto-narrows to yellow disclosing a summarized side-effect note, the voice hint auto-narrows to yellow
/// disclosing a partial copy-safe export capture, and the companion handoff auto-narrows to yellow with a
/// waivered reduced hover fallback — and no row is blocked, so the packet is clean and every row is
/// publishable.
pub fn seeded_m5_discoverability_affordance_parity_packet() -> AffordanceParityPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the primary / action button invents a private label or lifecycle language for a
/// stable command, proving that an invented label blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_discoverability_affordance_parity_packet_button_private_label_blocked(
) -> AffordanceParityPacket {
    let rows = seeded_rows_with(M5ConvenienceAffordance::Button, |spec| {
        spec.label_reuse = LabelReuseState::PrivateLabelOrLifecycleInvented;
        spec.narrowing_reason = Some(
            "The primary / action button invented a private label and dropped the lifecycle badge for a \
             stable command, so the same action read under a different name depending on whether it was \
             reached from the button or the canonical command record, and the button blocks before keeping \
             a parity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the tooltip weakens the side-effect or preview / approval truth, proving that a
/// weakened side-effect story blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_discoverability_affordance_parity_packet_tooltip_side_effect_weakened_blocked(
) -> AffordanceParityPacket {
    let rows = seeded_rows_with(M5ConvenienceAffordance::Tooltip, |spec| {
        spec.side_effect_truth = SideEffectTruthState::SideEffectOrPreviewTruthWeakened;
        spec.narrowing_reason = Some(
            "The tooltip dropped the preview / approval requirement the canonical command record pins, so a \
             preview-gated action read as a one-tap convenience in the hovercard, and the tooltip blocks \
             before keeping a parity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the companion handoff renders hover-only or overreaches the canonical authority,
/// proving that a hover-only / authority overreach blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_discoverability_affordance_parity_packet_companion_authority_overreach_blocked(
) -> AffordanceParityPacket {
    let rows = seeded_rows_with(M5ConvenienceAffordance::CompanionHandoff, |spec| {
        spec.authority_reach = AuthorityReachState::HoverOnlyOrAuthorityOverreach;
        spec.waiver = None;
        spec.narrowing_reason = Some(
            "The companion / browser handoff implied a stronger apply-without-review action than the desktop \
             command record allows and left the reach hover-only with no keyboard-focus or context-action \
             equivalent, so the companion widened authority beyond the canonical command, and the handoff \
             blocks before keeping a parity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the voice hint's originating command id is absent from the durable export,
/// proving that an absent origin capture blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_discoverability_affordance_parity_packet_voice_hint_origin_absent_blocked(
) -> AffordanceParityPacket {
    let rows = seeded_rows_with(M5ConvenienceAffordance::VoiceHint, |spec| {
        spec.origin_export = OriginExportState::OriginatingCommandAbsentFromCapture;
        spec.narrowing_reason = Some(
            "The voice hint rendered its originating command only as a spoken confirmation that never \
             reached the durable, diffable export, so a support bundle or migration packet could not \
             reconstruct which command the voice affordance triggered without a screenshot, and the hint \
             blocks before keeping a parity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the AI hint loses the shared command record in a headless / CLI execution, proving
/// that a headless parity loss blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_discoverability_affordance_parity_packet_ai_hint_headless_parity_lost_blocked(
) -> AffordanceParityPacket {
    let rows = seeded_rows_with(M5ConvenienceAffordance::AiHint, |spec| {
        spec.headless_parity_preserved = false;
        spec.narrowing_reason = Some(
            "In headless / CLI execution the AI hint projected a different label and side-effect class than \
             the in-product affordance, so the same action projected a different command record depending on \
             how it ran, and the hint blocks before keeping a parity claim.",
        );
    });
    packet_from_rows(rows)
}
