//! Canonical seed builders for the M5 host-rendered primitive layer.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked bindings, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical host-rendered-primitive packet.
pub const M5_HOST_RENDERED_PRIMITIVE_PACKET_ID: &str = "m5-host-rendered-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-30T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds one conformant worked binding for a family on a host surface. The consumer
/// wires exactly the family's fixed token slots, restyles only cosmetic aspects, and
/// overrides no contract part.
fn binding(
    family: M5HostRenderedPrimitiveFamily,
    consumer_id: &str,
    host_surface: M5PrimitiveHostSurface,
    render_mode: M5PrimitiveRenderMode,
    audited_wrapper_ref: Option<&str>,
) -> M5PrimitiveBindingCase {
    M5PrimitiveBindingCase::resolved(M5PrimitiveBindingInput {
        primitive_family: family,
        consumer_id: consumer_id.to_owned(),
        host_surface,
        render_mode,
        audited_wrapper_ref: audited_wrapper_ref.map(str::to_owned),
        wired_token_slots: family.fixed_token_slots(),
        restyled_aspects: vec![
            M5RestylableAspect::SpacingScale,
            M5RestylableAspect::AccentTint,
            M5RestylableAspect::IconSet,
        ],
        overridden_contract_parts: Vec::new(),
    })
}

/// The three worked bindings every primitive carries: one canonical render on the
/// desktop app, one canonical render on the companion surface, and one audited
/// wrapper on an extension host. The two canonical renders on distinct surfaces prove
/// token-wiring parity; the extension wrapper proves the audited-wrapper path.
fn standard_bindings(
    family: M5HostRenderedPrimitiveFamily,
    slug: &str,
) -> Vec<M5PrimitiveBindingCase> {
    vec![
        binding(
            family,
            &format!("consumer:desktop:{slug}"),
            M5PrimitiveHostSurface::DesktopApp,
            M5PrimitiveRenderMode::HostRenderedCanonical,
            None,
        ),
        binding(
            family,
            &format!("consumer:companion:{slug}"),
            M5PrimitiveHostSurface::CompanionSurface,
            M5PrimitiveRenderMode::HostRenderedCanonical,
            None,
        ),
        binding(
            family,
            &format!("consumer:extension:{slug}"),
            M5PrimitiveHostSurface::ExtensionHost,
            M5PrimitiveRenderMode::AuditedWrapper,
            Some(&format!("audit:host-wrapper:{slug}")),
        ),
    ]
}

fn naming(family: M5HostRenderedPrimitiveFamily) -> M5PrimitiveNamingParity {
    let token = family.as_str();
    M5PrimitiveNamingParity {
        demo_name: token.to_owned(),
        screenshot_name: token.to_owned(),
        support_export_name: token.to_owned(),
    }
}

/// A base row with the shared fields filled in and the full host-surface, render-mode,
/// contract-part, restylable-aspect, and accessibility parity every primitive carries.
/// The bound families and fixed token slots come from the family so the row can never
/// drift from the resolver's view of the family.
fn base_row(
    family: M5HostRenderedPrimitiveFamily,
    qualification: M5TrustQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_pills: Vec<M5SettingSourcePill>,
    provenance_badges: Vec<M5ProvenanceBadge>,
    example_bindings: Vec<M5PrimitiveBindingCase>,
) -> M5HostRenderedPrimitiveRow {
    M5HostRenderedPrimitiveRow {
        primitive_family: family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        bound_component_families: family.bound_component_families(),
        shell_zone_slot: family.canonical_zone(),
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes: M5WindowClass::ALL.to_vec(),
        host_surfaces: M5PrimitiveHostSurface::ALL.to_vec(),
        render_modes: vec![
            M5PrimitiveRenderMode::HostRenderedCanonical,
            M5PrimitiveRenderMode::AuditedWrapper,
        ],
        fixed_token_slots: family.fixed_token_slots(),
        fixed_contract_parts: M5PrimitiveContractPart::ALL.to_vec(),
        restylable_aspects: M5RestylableAspect::ALL.to_vec(),
        source_pills,
        provenance_badges,
        accessibility_routes: M5TrustAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ShellConsumerSurface::ShellFrame,
            M5ShellConsumerSurface::Layout,
            M5ShellConsumerSurface::DocsHelp,
            M5ShellConsumerSurface::ReleaseProof,
            M5ShellConsumerSurface::SupportExport,
            M5ShellConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5TrustComponentDowngradeTrigger::SourcePillMissing,
            M5TrustComponentDowngradeTrigger::ProvenanceBadgeMissing,
            M5TrustComponentDowngradeTrigger::AuditTruthLostOffPrimarySurface,
            M5TrustComponentDowngradeTrigger::ProofStale,
        ],
        naming_parity: naming(family),
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_HOST_RENDERED_SCHEMA_REF,
            M5_HOST_RENDERED_COMPONENT_MATRIX_REF,
        ]),
        example_bindings,
        allows_bespoke_local_variant: false,
        drops_fixed_token_wiring: false,
        restyles_fixed_contract_part: false,
        drops_export_or_audit_truth: false,
    }
}

fn primitive_rows() -> Vec<M5HostRenderedPrimitiveRow> {
    use M5HostRenderedPrimitiveFamily as F;

    vec![
        // 1. Settings row — wires the source pill; lives in the main workspace.
        base_row(
            F::SettingsRow,
            M5TrustQualificationClass::Stable,
            "Settings component owner",
            "The settings-row primitive host-renders effective-versus-configured truth with the source pill, lock state, and severity colour pinned; desktop and companion render it canonically and an extension host renders it through an audited wrapper",
            "host-rendered:m5-settings-row:001",
            M5SettingSourcePill::ALL.to_vec(),
            Vec::new(),
            standard_bindings(F::SettingsRow, "settings-row"),
        ),
        // 2. Capability sheet — no badge/pill; lives in the transient overlay.
        base_row(
            F::CapabilitySheet,
            M5TrustQualificationClass::Stable,
            "Capability sheet owner",
            "The capability-sheet primitive host-renders consequence-grouped requests with severity, disclosure, and state labels pinned; desktop and companion render it canonically and an extension host renders it through an audited wrapper",
            "host-rendered:m5-capability-sheet:001",
            Vec::new(),
            Vec::new(),
            standard_bindings(F::CapabilitySheet, "capability-sheet"),
        ),
        // 3. Event / history row — wires the provenance badge; lives in the bottom
        //    panel.
        base_row(
            F::EventHistoryRow,
            M5TrustQualificationClass::Stable,
            "Chronology row owner",
            "The event/history-row primitive host-renders a stable verb, provenance badge, and reopenable detail with severity pinned; desktop and companion render it canonically and an extension host renders it through an audited wrapper",
            "host-rendered:m5-evidence-row:001",
            Vec::new(),
            M5ProvenanceBadge::ALL.to_vec(),
            standard_bindings(F::EventHistoryRow, "event-history-row"),
        ),
        // 4. Timeline group — binds both the timeline group and the narrative summary
        //    card; wires the provenance badge; lives in the bottom panel.
        base_row(
            F::TimelineGroup,
            M5TrustQualificationClass::Stable,
            "Chronology group owner",
            "The timeline-group primitive host-renders phase-grouped timeline groups and the narrative summary card with the provenance badge and severity pinned; desktop and companion render it canonically and an extension host renders it through an audited wrapper",
            "host-rendered:m5-chronology-group:001",
            Vec::new(),
            M5ProvenanceBadge::ALL.to_vec(),
            standard_bindings(F::TimelineGroup, "timeline-group"),
        ),
        // 5. Chronology export preview — wires the provenance badge; lives in the
        //    bottom panel.
        base_row(
            F::ChronologyExportPreview,
            M5TrustQualificationClass::Stable,
            "Chronology export owner",
            "The chronology-export-preview primitive host-renders the selected range, included fields, redaction class, and format with the provenance badge and severity pinned; desktop and companion render it canonically and an extension host renders it through an audited wrapper",
            "host-rendered:m5-chronology-export:001",
            Vec::new(),
            M5ProvenanceBadge::ALL.to_vec(),
            standard_bindings(F::ChronologyExportPreview, "chronology-export-preview"),
        ),
    ]
}

fn governance_review() -> M5HostRenderedGovernanceReview {
    M5HostRenderedGovernanceReview {
        every_family_binds_one_canonical_primitive: true,
        consumers_render_through_canonical_or_wrapper: true,
        shared_token_state_wiring_pinned: true,
        contract_parts_fixed_only_cosmetics_restylable: true,
        badges_pills_and_severity_wired_through_host: true,
        meaning_stable_across_host_surfaces: true,
        demos_screenshots_and_exports_share_names: true,
        no_consumer_invents_second_row_grammar: true,
        every_primitive_bound_to_shell_zone: true,
        later_lanes_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5HostRenderedConsumerProjection {
    M5HostRenderedConsumerProjection {
        desktop_consumers_render_canonical: true,
        companion_consumers_render_canonical: true,
        extension_consumers_render_canonical_or_wrapper: true,
        token_state_wiring_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5HostRenderedProofFreshness {
    M5HostRenderedProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5HostRenderedReleasePosture {
    M5HostRenderedReleasePosture {
        release_packet_ref: M5_HOST_RENDERED_ARTIFACT_REF.to_owned(),
        host_rendered_audit_ref: M5_HOST_RENDERED_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_HOST_RENDERED_SCHEMA_REF,
        M5_HOST_RENDERED_DOC_REF,
        M5_HOST_RENDERED_SHELL_ZONE_REF,
        M5_HOST_RENDERED_COMPONENT_MATRIX_REF,
        M5_HOST_RENDERED_SETTINGS_ROW_REF,
        M5_HOST_RENDERED_CAPABILITY_SHEET_REF,
        M5_HOST_RENDERED_EVIDENCE_ROW_REF,
        M5_HOST_RENDERED_CHRONOLOGY_PREVIEW_REF,
    ])
}

/// Builds the canonical M5 host-rendered-primitive packet.
pub fn seeded_m5_host_rendered_primitive_packet() -> M5HostRenderedPrimitivePacket {
    M5HostRenderedPrimitivePacket::new(M5HostRenderedPrimitivePacketInput {
        packet_id: M5_HOST_RENDERED_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 host-rendered trust-component primitives: canonical settings-row, capability-sheet, event/history-row, timeline-group, and chronology-export-preview binding with shared token / state wiring"
                .to_owned(),
        primitive_rows: primitive_rows(),
        vocabulary_set: M5HostRenderedVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the capability-sheet primitive is held at Beta because a slice of
/// its extension-host wrapper does not yet render on every profile; every primitive
/// stays visible.
pub fn seeded_m5_host_rendered_primitive_capability_sheet_beta_narrowed(
) -> M5HostRenderedPrimitivePacket {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    packet.packet_id = "m5-host-rendered-primitive:capability-sheet-beta:0001".to_owned();
    let row = packet
        .primitive_rows
        .iter_mut()
        .find(|row| row.primitive_family == M5HostRenderedPrimitiveFamily::CapabilitySheet)
        .expect("capability-sheet row present");
    row.qualification = M5TrustQualificationClass::Beta;
    packet
}

/// Narrowed variant: the chronology-export-preview primitive is narrowed to Preview
/// pending token-wiring parity across every export format surface; every primitive
/// stays visible.
pub fn seeded_m5_host_rendered_primitive_chronology_export_preview_narrowed(
) -> M5HostRenderedPrimitivePacket {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    packet.packet_id = "m5-host-rendered-primitive:chronology-export-preview:0001".to_owned();
    let row = packet
        .primitive_rows
        .iter_mut()
        .find(|row| row.primitive_family == M5HostRenderedPrimitiveFamily::ChronologyExportPreview)
        .expect("chronology-export-preview row present");
    row.qualification = M5TrustQualificationClass::Preview;
    packet
}
