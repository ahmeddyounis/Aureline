//! Canonical seed builders for the M5 discoverability-access-parity certification.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export, and CSV
//! artifacts plus the narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code parity proof, the artifacts, and the fixtures never drift. Every attribute each family row
//! certifies over — the canonical command binding, the surface's qualification, owner, required labels,
//! lifecycle label, preview class, feature families, and declared consumer surfaces, and the applicable
//! downgrade triggers — is pulled straight from the frozen discoverability matrix's seeded packet, so the
//! certification cannot audit a surface the matrix does not anchor. Only the non-pointer reach channels,
//! accessibility-incident fields, desktop access profiles, the four accessibility / export postures, and
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

/// The accessibility / export posture seeded for one surface family.
struct SurfaceSpec {
    /// Short conformance scope summary.
    scope_summary: &'static str,
    /// The non-pointer reach channels this row certifies (defaults to all five).
    certified_reach_channels: Vec<M5NonPointerReachChannel>,
    /// The accessibility-incident fields this row captures (defaults to all five).
    certified_incident_fields: Vec<M5AccessibilityIncidentField>,
    /// The desktop access profiles this row stays stable in (defaults to all four).
    certified_access_profiles: Vec<M5DesktopAccessProfile>,
    /// When set, the evaluated-surface set used instead of the surface's declared set (blocked fixtures
    /// use this to prove a partial certification blocks).
    evaluated_surfaces_override: Option<Vec<M5DiscoveryChannel>>,
    non_pointer_reach: NonPointerReachState,
    support_export_evidence: SupportExportEvidenceState,
    profile_stability: ProfileStabilityState,
    release_evidence: ReleaseEvidenceFreshnessState,
    headless_parity_preserved: bool,
    waiver: Option<AccessParityWaiver>,
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

/// Builds one access-parity row from a surface family and a posture. Every binding — the canonical command
/// binding, the surface's qualification, owner, required labels, lifecycle label, preview class, feature
/// families, and declared consumer surfaces, and the downgrade triggers — is pulled from the frozen matrix
/// row for the family.
fn row_from_family(family: M5CommandSurfaceFamily, spec: SurfaceSpec) -> AccessParityRow {
    let surface = matrix_surface_row(family);
    let required_consumer_surfaces = surface.consumer_surfaces.clone();
    let evaluated_consumer_surfaces = spec
        .evaluated_surfaces_override
        .unwrap_or_else(|| surface.consumer_surfaces.clone());
    let mut row = AccessParityRow {
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
        certified_reach_channels: spec.certified_reach_channels,
        certified_incident_fields: spec.certified_incident_fields,
        certified_access_profiles: spec.certified_access_profiles,
        required_consumer_surfaces,
        evaluated_consumer_surfaces,
        non_pointer_reach: spec.non_pointer_reach,
        support_export_evidence: spec.support_export_evidence,
        profile_stability: spec.profile_stability,
        release_evidence: spec.release_evidence,
        headless_parity_preserved: spec.headless_parity_preserved,
        applicable_downgrade_triggers: surface.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        derived_status: AccessParityStatus::Green,
        conformance_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.conformance_causes = row.recompute_causes();
    row
}

/// Builds the non-pointer-reach reduced-touch-fallback waiver carried by the seed.
fn reduced_touch_fallback_waiver() -> AccessParityWaiver {
    AccessParityWaiver {
        waiver_id: "waiver:access-parity-reduced-touch:0001".to_owned(),
        surface_family: M5CommandSurfaceFamily::CommandBar,
        reason:
            "On a constrained touch surface the command / action bar's hover affordance falls back to a \
             disclosed, waivered reduced form — the hover detail collapses into a tap-to-open sheet while \
             the keyboard path and screen-reader narration stay present and the focus still returns \
             predictably — so the reach is narrowed and disclosed rather than hover-only. The exception \
             retires when the bar renders the full touch-equivalent affordance on every claimed touch \
             surface."
                .to_owned(),
        owner_role: "Shell/accessibility owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// A full-conformance posture: all four accessibility / export dimensions hold, all five non-pointer reach
/// channels, all five accessibility-incident fields, and all four desktop access profiles are certified, and
/// headless parity is preserved.
fn full(scope_summary: &'static str) -> SurfaceSpec {
    SurfaceSpec {
        scope_summary,
        certified_reach_channels: M5NonPointerReachChannel::ALL.to_vec(),
        certified_incident_fields: M5AccessibilityIncidentField::ALL.to_vec(),
        certified_access_profiles: M5DesktopAccessProfile::ALL.to_vec(),
        evaluated_surfaces_override: None,
        non_pointer_reach: NonPointerReachState::KeyboardScreenReaderAndTouchParityCertified,
        support_export_evidence:
            SupportExportEvidenceState::StructuredIncidentEvidenceReconstructable,
        profile_stability: ProfileStabilityState::ReachableAndStableAcrossAllProfiles,
        release_evidence: ReleaseEvidenceFreshnessState::ParityChecksGateReleaseEvidence,
        headless_parity_preserved: true,
        waiver: None,
        narrowing_reason: None,
    }
}

/// Returns the seeded accessibility / export posture for one surface family.
fn family_spec(family: M5CommandSurfaceFamily) -> SurfaceSpec {
    use M5CommandSurfaceFamily as F;
    match family {
        F::MenuItem => full(
            "Menu-bar item is fully keyboard- and screen-reader-addressable with focus return and a touch \
             / context-action equivalent, reconstructs its command and blocked-state evidence from the \
             structured support-export, stays stable across every desktop profile, and gates its \
             accessibility claim on fresh release evidence across every consumer surface",
        ),
        F::MenuGroup => full(
            "Menu group keeps every member keyboard- and screen-reader-addressable with focus return, \
             reconstructs the incident evidence from the structured export, and stays reachable and stable \
             across every desktop profile and consumer surface",
        ),
        F::ContextMenu => full(
            "Context menu offers a keyboard and touch / context-action equivalent for every focused \
             object's action, narrates through the screen reader, reconstructs the incident evidence, and \
             stays stable across every desktop profile",
        ),
        F::KeybindingResolverLayer => full(
            "Keybinding resolver layer is keyboard- and screen-reader-addressable with focus return, \
             reconstructs the winning-binding and reserved-state evidence from the structured export, and \
             stays reachable and stable across every desktop profile and consumer surface",
        ),
        F::ConflictReviewSheet => full(
            "Conflict review sheet is keyboard- and screen-reader-addressable, reconstructs the conflict \
             evidence from the structured export, and stays reachable and stable across every desktop \
             profile and consumer surface",
        ),
        F::DisabledCommandExplainer => full(
            "Disabled-command explainer is keyboard- and screen-reader-addressable with focus return and a \
             touch / context-action equivalent, reconstructs the blocker evidence from the structured \
             export, and stays stable across every desktop profile and consumer surface",
        ),
        // Command / action bar discloses a waivered reduced touch fallback on a constrained surface
        // (yellow).
        F::CommandBar => SurfaceSpec {
            non_pointer_reach: NonPointerReachState::DisclosedReducedTouchFallback,
            waiver: Some(reduced_touch_fallback_waiver()),
            narrowing_reason: Some(
                "On a constrained touch surface the command / action bar's hover affordance falls back to \
                 a disclosed, waivered reduced form — the hover detail collapses into a tap-to-open sheet \
                 while the keyboard path and screen-reader narration stay present and the focus returns \
                 predictably — so the reach is narrowed and disclosed rather than hover-only.",
            ),
            ..full(
                "Command / action bar reconstructs its incident evidence and stays stable across every \
                 desktop profile, falling back to a disclosed, waivered reduced touch form on a \
                 constrained touch surface while keeping the keyboard path and screen-reader narration",
            )
        },
        // Import-bridge row discloses a partial support-export capture on a legacy export (yellow).
        F::ImportBridgeRow => SurfaceSpec {
            support_export_evidence: SupportExportEvidenceState::DisclosedPartialCapture,
            narrowing_reason: Some(
                "On the legacy import export the copy-safe support-export takes a disclosed partial \
                 capture — the export captures the command id and blocker reason but not the full \
                 incident-field set, while still disclosing the gap — so the support-export evidence is \
                 narrowed and disclosed rather than absent.",
            ),
            ..full(
                "Import-bridge row stays keyboard- and screen-reader-addressable and stable across every \
                 desktop profile, capturing the command id and blocker reason but not the full \
                 incident-field set on one legacy export",
            )
        },
        // Leader / sequence help overlay discloses a reduced profile coverage on a constrained profile
        // (yellow).
        F::LeaderSequenceHelp => SurfaceSpec {
            profile_stability: ProfileStabilityState::DisclosedReducedProfileCoverage,
            narrowing_reason: Some(
                "On the space-constrained compact-layout profile the leader / sequence help overlay \
                 renders a disclosed reduced form — the resulting-label detail folds into an expandable \
                 hint while the overlay stays reachable and stable and keeps its keyboard path and \
                 screen-reader narration — so the profile coverage is narrowed and disclosed rather than \
                 unreachable.",
            ),
            ..full(
                "Leader / sequence help overlay stays keyboard- and screen-reader-addressable and \
                 reconstructs its incident evidence, rendering a disclosed reduced form on the \
                 compact-layout profile while staying reachable and stable",
            )
        },
        // Command-documentation surface discloses a partial release-evidence refresh (yellow).
        F::CommandDocumentationSurface => SurfaceSpec {
            release_evidence: ReleaseEvidenceFreshnessState::DisclosedPartialEvidenceRefresh,
            narrowing_reason: Some(
                "On one legacy release-evidence surface the command-documentation parity check refreshes \
                 on a disclosed delayed cadence while still gating the claim — a stale help anchor or \
                 missing narration still narrows the claim on the next refresh — so the release-evidence \
                 freshness is narrowed and disclosed rather than stale.",
            ),
            ..full(
                "Command-documentation surface stays keyboard- and screen-reader-addressable and stable \
                 across every desktop profile, refreshing its release-evidence parity check on a disclosed \
                 delayed cadence on one legacy surface while still gating the claim",
            )
        },
    }
}

/// Builds the access-parity rows for the canonical seed, one per surface family.
fn seeded_rows() -> Vec<AccessParityRow> {
    M5CommandSurfaceFamily::ALL
        .iter()
        .map(|&family| row_from_family(family, family_spec(family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is resolved, used by the
/// blocked fixtures.
fn seeded_rows_with<F>(target: M5CommandSurfaceFamily, mutate: F) -> Vec<AccessParityRow>
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

fn packet_from_rows(rows: Vec<AccessParityRow>) -> AccessParityPacket {
    build_m5_access_parity_packet(AccessParityInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_DISCOVERABILITY_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 discoverability-access-parity packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV artifacts. Six
/// families keep full non-pointer-reach, support-export-evidence, profile-stability, and release-evidence
/// truth (green). The command / action bar auto-narrows to yellow with a waivered reduced touch fallback,
/// the import-bridge row auto-narrows to yellow disclosing a partial support-export capture, the leader /
/// sequence help overlay auto-narrows to yellow disclosing a reduced profile coverage, and the
/// command-documentation surface auto-narrows to yellow disclosing a partial release-evidence refresh — and
/// no row is blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_discoverability_access_parity_packet() -> AccessParityPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the menu-bar item renders hover-only or drops its screen-reader narration, proving
/// that a hover-only surface blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_discoverability_access_parity_packet_menu_item_hover_only_blocked(
) -> AccessParityPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::MenuItem, |spec| {
        spec.non_pointer_reach = NonPointerReachState::HoverOnlyOrNarrationMissing;
        spec.narrowing_reason = Some(
            "The menu-bar item exposed its overflow actions only on pointer hover with no keyboard or \
             touch / context-action equivalent and dropped the screen-reader narration, so the same \
             command could not be addressed without a pointer, and the item blocks before keeping an \
             accessibility claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the context menu's blocked-state evidence is absent from the durable export,
/// proving that an absent capture blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_discoverability_access_parity_packet_context_menu_evidence_absent_blocked(
) -> AccessParityPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::ContextMenu, |spec| {
        spec.support_export_evidence = SupportExportEvidenceState::BlockedStateAbsentFromCapture;
        spec.narrowing_reason = Some(
            "The context menu rendered its blocked-state reason only as a live tooltip that never reached \
             the durable, diffable support-export, so a support reviewer could not reconstruct the command \
             id, source layer, blocker reason, lifecycle state, or help anchor without a screenshot, and \
             the menu blocks before keeping an accessibility claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the keybinding resolver layer becomes unreachable or unstable on a claimed desktop
/// profile, proving that an unstable profile blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_discoverability_access_parity_packet_resolver_profile_unstable_blocked(
) -> AccessParityPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::KeybindingResolverLayer, |spec| {
        spec.profile_stability = ProfileStabilityState::SurfaceUnreachableOrUnstableInProfile;
        spec.narrowing_reason = Some(
            "On the high-zoom profile the keybinding resolver layer's inspector clipped its winning-binding \
             detail off-screen with no scroll path, so the surface became unreachable on a claimed desktop \
             profile, and the resolver blocks before keeping an accessibility claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the command-documentation surface ships a stale help anchor without narrowing the
/// claim, proving that a stale-anchor regression blocks a stable claim (red) rather than staying green.
pub fn seeded_m5_discoverability_access_parity_packet_doc_stale_anchor_blocked(
) -> AccessParityPacket {
    let rows = seeded_rows_with(
        M5CommandSurfaceFamily::CommandDocumentationSurface,
        |spec| {
            spec.release_evidence = ReleaseEvidenceFreshnessState::StaleAnchorOrRegressionUnblocked;
            spec.narrowing_reason = Some(
            "The command-documentation surface shipped a help anchor that pointed at a removed section and \
             a narration string that no longer matched the command, and the release-evidence parity check \
             did not narrow the claim, so the release evidence overclaimed accessibility the surface no \
             longer provided, and the surface blocks before keeping an accessibility claim.",
        );
        },
    );
    packet_from_rows(rows)
}

/// Builds a variant where the disabled-command explainer loses the shared accessibility / export parity in a
/// headless / CLI execution, proving that a headless parity loss blocks a stable claim (red) rather than
/// staying green.
pub fn seeded_m5_discoverability_access_parity_packet_explainer_headless_parity_lost_blocked(
) -> AccessParityPacket {
    let rows = seeded_rows_with(M5CommandSurfaceFamily::DisabledCommandExplainer, |spec| {
        spec.headless_parity_preserved = false;
        spec.narrowing_reason = Some(
            "In headless / CLI execution the disabled-command explainer exported a different blocker reason \
             and help anchor than the in-product surface, so the same command reconstructed a different \
             incident depending on how it ran, and the explainer blocks before keeping an accessibility \
             claim.",
        );
    });
    packet_from_rows(rows)
}
