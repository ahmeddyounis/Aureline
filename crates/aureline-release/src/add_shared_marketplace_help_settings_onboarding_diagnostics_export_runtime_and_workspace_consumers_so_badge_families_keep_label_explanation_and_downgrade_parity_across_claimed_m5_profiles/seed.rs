//! Canonical seed builders for the M5 badge-family-consumer lane.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call them
//! so the in-code matrix, the artifact, the worked bindings, and the fixtures
//! never drift.

use super::*;

/// Stable packet id for the canonical badge-family-consumer packet.
pub const M5_BADGE_FAMILY_CONSUMER_PACKET_ID: &str = "m5-badge-family-consumer:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-08T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked binding case for one consumer/family adoption.
fn case(
    consumer: M5BadgeConsumer,
    badge_family: M5BadgeFamily,
    render_mode: M5BadgeRenderMode,
    downgrade_caveats: &[M5BadgeDowngradeTrigger],
    note: &str,
) -> M5BadgeConsumerBindingCase {
    M5BadgeConsumerBindingCase::resolved(M5BadgeConsumerBindingInput {
        consumer,
        badge_family,
        parity_facets: M5BadgeParityFacet::ALL.to_vec(),
        render_mode,
        downgrade_caveats: downgrade_caveats.to_vec(),
        note_repr: Some(note.to_owned()),
    })
}

/// Builds a family binding that points at its canonical family refs.
fn binding(
    badge_family: M5BadgeFamily,
    example_bindings: Vec<M5BadgeConsumerBindingCase>,
) -> M5BadgeFamilyBinding {
    M5BadgeFamilyBinding {
        badge_family,
        canonical_schema_ref: badge_family_canonical_schema_ref(badge_family).to_owned(),
        canonical_artifact_ref: badge_family_canonical_artifact_ref(badge_family).to_owned(),
        references_canonical_not_local_prose: true,
        example_bindings,
    }
}

/// A base row with the shared parity vocabulary filled in.
fn base_row(
    consumer: M5BadgeConsumer,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    family_bindings: Vec<M5BadgeFamilyBinding>,
) -> M5BadgeConsumerRow {
    M5BadgeConsumerRow {
        consumer,
        qualification: M5BadgeQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5BadgeSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DeploymentLine::ALL.to_vec(),
        anatomy_parts: M5BadgeConsumerAnatomyPart::ALL.to_vec(),
        parity_facets: M5BadgeParityFacet::ALL.to_vec(),
        render_modes: M5BadgeRenderMode::ALL.to_vec(),
        downgrade_caveats: M5BadgeDowngradeTrigger::ALL.to_vec(),
        parity_states: M5BadgeParityState::ALL.to_vec(),
        narrow_reasons: M5BadgeNarrowReason::ALL.to_vec(),
        narrow_next_actions: M5BadgeNarrowNextAction::ALL.to_vec(),
        export_fields: M5BadgeConsumerExportField::ALL.to_vec(),
        accessibility_routes: M5BadgeAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5BadgeConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5BadgeDowngradeTrigger::EvidenceFreshnessHidden,
            M5BadgeDowngradeTrigger::ExportLostBadgeMeaning,
            M5BadgeDowngradeTrigger::ProofStale,
        ],
        family_bindings,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_BADGE_FAMILY_CONSUMER_SCHEMA_REF,
            M5_BADGE_FAMILY_SCHEMA_REF,
        ]),
        collapses_axes_into_one_pill: false,
        implies_freshness_from_support_class: false,
        drops_badge_meaning_in_export: false,
        rewords_labels_per_surface: false,
    }
}

fn consumer_rows() -> Vec<M5BadgeConsumerRow> {
    use M5BadgeConsumer as Consumer;
    use M5BadgeDowngradeTrigger as Trigger;
    use M5BadgeFamily as Family;
    use M5BadgeRenderMode as Mode;

    let mut rows = Vec::with_capacity(M5BadgeConsumer::ALL.len());

    // 1. Marketplace / install — support-class and lifecycle badges at full claim
    //    scope, pointing at their canonical schemas so the label, explanation, and
    //    downgrade reason match every other surface.
    rows.push(base_row(
        Consumer::Marketplace,
        "Marketplace surface owner",
        "The marketplace/install surface adopts the support-class and lifecycle badges at full claim scope, pointing at their canonical badge schemas so the label, explanation drawer, and downgrade reason stay identical to what Help/About, settings, onboarding, diagnostics, the support export, runtime cards, and the workspace surface show",
        "evidence:m5-badge-family-consumer-marketplace:001",
        vec![
            binding(
                Family::SupportClass,
                vec![case(
                    Consumer::Marketplace,
                    Family::SupportClass,
                    Mode::FullClaimScope,
                    &[],
                    "marketplace support-class badge at full claim scope",
                )],
            ),
            binding(
                Family::Lifecycle,
                vec![case(
                    Consumer::Marketplace,
                    Family::Lifecycle,
                    Mode::FullClaimScope,
                    &[],
                    "marketplace lifecycle badge at full claim scope",
                )],
            ),
        ],
    ));

    // 2. Help / About — evidence-freshness and support-class badges at full claim
    //    scope, referencing the canonical schemas so its prose can never drift.
    rows.push(base_row(
        Consumer::HelpAbout,
        "Help / About surface owner",
        "The Help/About surface adopts the evidence-freshness and support-class badges at full claim scope, referencing the canonical badge schemas so its prose can never drift from the live badge label, explanation, and downgrade reason",
        "evidence:m5-badge-family-consumer-help-about:001",
        vec![
            binding(
                Family::EvidenceFreshness,
                vec![case(
                    Consumer::HelpAbout,
                    Family::EvidenceFreshness,
                    Mode::FullClaimScope,
                    &[],
                    "help/about evidence-freshness badge at full claim scope",
                )],
            ),
            binding(
                Family::SupportClass,
                vec![case(
                    Consumer::HelpAbout,
                    Family::SupportClass,
                    Mode::FullClaimScope,
                    &[],
                    "help/about support-class badge at full claim scope",
                )],
            ),
        ],
    ));

    // 3. Settings / policy — deployment-scope and channel badges at full claim
    //    scope in the policy explainers.
    rows.push(base_row(
        Consumer::Settings,
        "Settings / policy surface owner",
        "The settings/policy-explainer surface adopts the deployment-scope and channel badges at full claim scope, reading the same label, explanation, and downgrade reason so a policy explainer never re-words a badge axis locally",
        "evidence:m5-badge-family-consumer-settings:001",
        vec![
            binding(
                Family::DeploymentScope,
                vec![case(
                    Consumer::Settings,
                    Family::DeploymentScope,
                    Mode::FullClaimScope,
                    &[],
                    "settings deployment-scope badge at full claim scope",
                )],
            ),
            binding(
                Family::Channel,
                vec![case(
                    Consumer::Settings,
                    Family::Channel,
                    Mode::FullClaimScope,
                    &[],
                    "settings channel badge at full claim scope",
                )],
            ),
        ],
    ));

    // 4. Onboarding / start-center — lifecycle at full scope and evidence-freshness
    //    auto-narrowed because a badge's evidence went stale.
    rows.push(base_row(
        Consumer::Onboarding,
        "Onboarding / start-center surface owner",
        "The onboarding/start-center surface adopts the lifecycle badge at full claim scope and the evidence-freshness badge auto-narrowed when its evidence goes stale, disclosing the narrowing with a self-contained banner while keeping the same parity vocabulary every surface uses",
        "evidence:m5-badge-family-consumer-onboarding:001",
        vec![
            binding(
                Family::Lifecycle,
                vec![case(
                    Consumer::Onboarding,
                    Family::Lifecycle,
                    Mode::FullClaimScope,
                    &[],
                    "onboarding lifecycle badge at full claim scope",
                )],
            ),
            binding(
                Family::EvidenceFreshness,
                vec![case(
                    Consumer::Onboarding,
                    Family::EvidenceFreshness,
                    Mode::FreshnessNarrowed,
                    &[Trigger::EvidenceFreshnessHidden, Trigger::ProofStale],
                    "onboarding evidence-freshness badge auto-narrowed on stale proof",
                )],
            ),
        ],
    ));

    // 5. Diagnostics — evidence-freshness auto-narrowed on stale proof and
    //    compatibility-state auto-narrowed because its scope reduced.
    rows.push(base_row(
        Consumer::Diagnostics,
        "Diagnostics surface owner",
        "The diagnostics surface adopts the evidence-freshness badge auto-narrowed on stale proof and the compatibility-state badge auto-narrowed when its scope reduces, preserving the label, explanation, and downgrade reason and naming the exact downgrade reason on a self-contained banner",
        "evidence:m5-badge-family-consumer-diagnostics:001",
        vec![
            binding(
                Family::EvidenceFreshness,
                vec![case(
                    Consumer::Diagnostics,
                    Family::EvidenceFreshness,
                    Mode::FreshnessNarrowed,
                    &[Trigger::EvidenceFreshnessHidden],
                    "diagnostics evidence-freshness badge auto-narrowed on stale proof",
                )],
            ),
            binding(
                Family::CompatibilityState,
                vec![case(
                    Consumer::Diagnostics,
                    Family::CompatibilityState,
                    Mode::ScopeNarrowed,
                    &[Trigger::CompatibilityStateUnstated],
                    "diagnostics compatibility-state badge auto-narrowed on reduced scope",
                )],
            ),
        ],
    ));

    // 6. Support export — support-class and compatibility-state badges rendered from
    //    an export snapshot, referencing the canonical schemas so the export never
    //    loses the badge meaning.
    rows.push(base_row(
        Consumer::SupportExport,
        "Support-export owner",
        "The support export adopts the support-class and compatibility-state badges from an export snapshot, reconstructing consumer parity from the shared model so a support reviewer reads the same label, explanation, and downgrade reason every product surface shows",
        "evidence:m5-badge-family-consumer-support-export:001",
        vec![
            binding(
                Family::SupportClass,
                vec![case(
                    Consumer::SupportExport,
                    Family::SupportClass,
                    Mode::ExportProjection,
                    &[Trigger::ExportLostBadgeMeaning],
                    "support-export support-class badge from an export snapshot",
                )],
            ),
            binding(
                Family::CompatibilityState,
                vec![case(
                    Consumer::SupportExport,
                    Family::CompatibilityState,
                    Mode::ExportProjection,
                    &[Trigger::ExportLostBadgeMeaning],
                    "support-export compatibility-state badge from an export snapshot",
                )],
            ),
        ],
    ));

    // 7. Runtime / deployment cards — deployment-scope auto-narrowed on reduced
    //    scope and channel at full claim scope.
    rows.push(base_row(
        Consumer::Runtime,
        "Runtime / deployment surface owner",
        "The runtime/deployment-card surface adopts the deployment-scope badge auto-narrowed when its scope reduces and the channel badge at full claim scope, keeping the same parity vocabulary and disclosing the narrowing with a self-contained banner",
        "evidence:m5-badge-family-consumer-runtime:001",
        vec![
            binding(
                Family::DeploymentScope,
                vec![case(
                    Consumer::Runtime,
                    Family::DeploymentScope,
                    Mode::ScopeNarrowed,
                    &[Trigger::DeploymentScopeUnstated],
                    "runtime deployment-scope badge auto-narrowed on reduced scope",
                )],
            ),
            binding(
                Family::Channel,
                vec![case(
                    Consumer::Runtime,
                    Family::Channel,
                    Mode::FullClaimScope,
                    &[],
                    "runtime channel badge at full claim scope",
                )],
            ),
        ],
    ));

    // 8. Workspace / archetype qualification — compatibility-state and
    //    deployment-scope badges at full claim scope.
    rows.push(base_row(
        Consumer::Workspace,
        "Workspace / archetype surface owner",
        "The workspace/archetype-qualification surface adopts the compatibility-state and deployment-scope badges at full claim scope, reading the same label, explanation, and downgrade reason so an archetype qualification never reinterprets a badge axis locally",
        "evidence:m5-badge-family-consumer-workspace:001",
        vec![
            binding(
                Family::CompatibilityState,
                vec![case(
                    Consumer::Workspace,
                    Family::CompatibilityState,
                    Mode::FullClaimScope,
                    &[],
                    "workspace compatibility-state badge at full claim scope",
                )],
            ),
            binding(
                Family::DeploymentScope,
                vec![case(
                    Consumer::Workspace,
                    Family::DeploymentScope,
                    Mode::FullClaimScope,
                    &[],
                    "workspace deployment-scope badge at full claim scope",
                )],
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5BadgeConsumerGovernanceReview {
    M5BadgeConsumerGovernanceReview {
        consumers_adopt_shared_badge_families: true,
        consumers_reference_canonical_badge_schema: true,
        label_vocabulary_shared_not_reworded: true,
        no_consumer_collapses_axes_into_one_pill: true,
        explanation_and_downgrade_explicit_on_every_surface: true,
        freshness_never_implied_from_support_class: true,
        narrowed_badge_always_shows_self_contained_banner: true,
        banner_names_exact_downgrade_reason_and_next_action: true,
        support_export_preserves_label_explanation_downgrade: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5BadgeConsumerProjection {
    M5BadgeConsumerProjection {
        all_consumers_adopt_shared_badge_families: true,
        label_reads_single_source: true,
        explanation_reads_single_source: true,
        downgrade_reason_reads_single_source: true,
        filter_key_reads_single_source: true,
    }
}

fn proof_freshness() -> M5BadgeConsumerProofFreshness {
    M5BadgeConsumerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5BadgeConsumerReleasePosture {
    M5BadgeConsumerReleasePosture {
        release_packet_ref: M5_BADGE_FAMILY_CONSUMER_ARTIFACT_REF.to_owned(),
        consumer_audit_ref: M5_BADGE_FAMILY_CONSUMER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_BADGE_FAMILY_CONSUMER_SCHEMA_REF,
        M5_BADGE_FAMILY_CONSUMER_DOC_REF,
        M5_BADGE_FAMILY_SCHEMA_REF,
        M5_BADGE_FAMILY_DOC_REF,
        badge_family_canonical_schema_ref(M5BadgeFamily::SupportClass),
        badge_family_canonical_schema_ref(M5BadgeFamily::Lifecycle),
        badge_family_canonical_schema_ref(M5BadgeFamily::DeploymentScope),
        badge_family_canonical_schema_ref(M5BadgeFamily::CompatibilityState),
    ])
}

/// Builds the canonical M5 badge-family-consumer packet.
pub fn seeded_m5_badge_family_consumer_packet() -> M5BadgeFamilyConsumerPacket {
    M5BadgeFamilyConsumerPacket::new(M5BadgeFamilyConsumerPacketInput {
        packet_id: M5_BADGE_FAMILY_CONSUMER_PACKET_ID.to_owned(),
        matrix_label:
            "M5 badge-family consumers: marketplace, Help/About, settings, onboarding, diagnostics, support export, runtime, and workspace keep label, explanation, and downgrade parity"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5BadgeConsumerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the diagnostics surface is held at Beta because a slice of
/// diagnostics renderings do not yet expose the narrow banner on every
/// freshness-narrowed path; every consumer stays visible.
pub fn seeded_m5_badge_family_consumer_diagnostics_freshness_beta_narrowed(
) -> M5BadgeFamilyConsumerPacket {
    let mut packet = seeded_m5_badge_family_consumer_packet();
    packet.packet_id = "m5-badge-family-consumer:diagnostics-beta:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5BadgeConsumer::Diagnostics)
        .expect("diagnostics row present");
    row.qualification = M5BadgeQualificationClass::Beta;
    packet
}

/// Narrowed variant: the support export is narrowed to Preview pending
/// export-snapshot parity proof across every export path; every consumer stays
/// visible.
pub fn seeded_m5_badge_family_consumer_support_export_scope_preview_narrowed(
) -> M5BadgeFamilyConsumerPacket {
    let mut packet = seeded_m5_badge_family_consumer_packet();
    packet.packet_id = "m5-badge-family-consumer:support-export-preview:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5BadgeConsumer::SupportExport)
        .expect("support-export row present");
    row.qualification = M5BadgeQualificationClass::Preview;
    packet
}
