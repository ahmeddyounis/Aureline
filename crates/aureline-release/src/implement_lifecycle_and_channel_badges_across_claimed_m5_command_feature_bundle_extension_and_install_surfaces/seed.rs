//! Canonical seed builders for the M5 lifecycle / channel badge primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical lifecycle / channel badge primitive packet.
pub const M5_MATURITY_BADGE_PRIMITIVE_PACKET_ID: &str =
    "m5-lifecycle-and-channel-badge-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-08T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked resolution case from a full lifecycle / channel state.
fn case(
    subject_label: &str,
    lifecycle: M5LifecycleBadgeValue,
    channel: M5ChannelBadgeValue,
    replacement_path_repr: Option<&str>,
    last_evaluated_repr: &str,
) -> M5LifecycleChannelResolutionCase {
    M5LifecycleChannelResolutionCase::resolved(M5LifecycleChannelBadgeInput {
        subject_label: subject_label.to_owned(),
        lifecycle,
        channel,
        replacement_path_repr: replacement_path_repr.map(str::to_owned),
        last_evaluated_repr: last_evaluated_repr.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full anatomy, lifecycle,
/// channel, effective-maturity, sunset-reason, next-action, explanation-field,
/// export-field, and accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5MaturityBadgeConsumerSurface,
    qualification: M5BadgeQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    example_resolutions: Vec<M5LifecycleChannelResolutionCase>,
) -> M5MaturityBadgeRow {
    M5MaturityBadgeRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5BadgeSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DeploymentLine::ALL.to_vec(),
        anatomy_parts: M5MaturityBadgeAnatomyPart::ALL.to_vec(),
        lifecycle_values: M5LifecycleBadgeValue::ALL.to_vec(),
        channel_values: M5ChannelBadgeValue::ALL.to_vec(),
        effective_maturity_postures: M5EffectiveMaturityPosture::ALL.to_vec(),
        sunset_reasons: M5LifecycleSunsetReason::ALL.to_vec(),
        next_actions: M5MaturityBadgeNextAction::ALL.to_vec(),
        explanation_fields: M5BadgeExplanationField::ALL.to_vec(),
        export_fields: M5MaturityBadgeExportField::ALL.to_vec(),
        accessibility_routes: M5BadgeAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5BadgeConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5BadgeDowngradeTrigger::LifecycleValueUnstated,
            M5BadgeDowngradeTrigger::ChannelValueUnstated,
            M5BadgeDowngradeTrigger::ExplanationDrawerMissing,
            M5BadgeDowngradeTrigger::AxisMergedIntoAnother,
            M5BadgeDowngradeTrigger::ExportLostBadgeMeaning,
            M5BadgeDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_MATURITY_BADGE_SCHEMA_REF,
            M5_MATURITY_BADGE_FAMILY_MATRIX_REF,
            M5_MATURITY_BADGE_LIFECYCLE_REF,
            M5_MATURITY_BADGE_CHANNEL_REF,
        ]),
        example_resolutions,
        collapses_lifecycle_and_channel_into_one_badge: false,
        implies_channel_from_lifecycle: false,
        drops_migration_path_on_deprecation: false,
        drops_badge_meaning_in_export: false,
    }
}

fn badge_rows() -> Vec<M5MaturityBadgeRow> {
    use M5ChannelBadgeValue as Chan;
    use M5LifecycleBadgeValue as Life;

    vec![
    // 1. Command row — a stable command on the stable channel reads as a stable-maturity
    //    claim, while a deprecated command on the stable channel points to its native
    //    replacement command (the migration-path and channel-preservation proof).
    base_row(
        M5MaturityBadgeConsumerSurface::CommandRow,
        M5BadgeQualificationClass::Stable,
        "Command badge owner",
        "The command row renders the shared lifecycle and channel badges as two distinct cues so a stable command on the stable channel reads as a stable-maturity claim, while a deprecated command still points to its native replacement command with a migration note that preserves the channel it was running on and offers a follow-migration-path next action",
        "evidence:m5-lifecycle-badge-parity:001",
        vec![
            case(
                "aureline command: run workspace sync",
                Life::Stable,
                Chan::Stable,
                None,
                "2026-07-01T00:00:00Z",
            ),
            case(
                "aureline command: legacy export bundle",
                Life::Deprecated,
                Chan::Stable,
                Some("migration:command/export-center-v2"),
                "2026-05-14T00:00:00Z",
            ),
        ],
    ),

    // 2. Feature surface — a beta feature on the beta channel, and a stable feature that
    //    is merely running on the preview channel (the distinct-cues proof: Stable does
    //    not imply the channel, and Preview channel does not imply experimental).
    base_row(
        M5MaturityBadgeConsumerSurface::FeatureSurface,
        M5BadgeQualificationClass::Stable,
        "Feature badge owner",
        "The feature surface renders the shared badges so a beta feature on the beta channel reads as beta-maturity, while a stable feature merely running on the preview channel still reads as a stable-maturity claim — proving the lifecycle never implies the channel and a preview channel never implies an experimental lifecycle",
        "evidence:m5-lifecycle-badge-parity:002",
        vec![
            case(
                "aureline feature: inline diff review",
                Life::Beta,
                Chan::Beta,
                None,
                "2026-07-02T00:00:00Z",
            ),
            case(
                "aureline feature: graph runtime",
                Life::Stable,
                Chan::Preview,
                None,
                "2026-06-20T00:00:00Z",
            ),
        ],
    ),

    // 3. Workflow bundle — an LTS-surface bundle on the LTS channel, and a
    //    removal-scheduled bundle that points to its migration path before the removal
    //    date (the removal-scheduled proof).
    base_row(
        M5MaturityBadgeConsumerSurface::WorkflowBundle,
        M5BadgeQualificationClass::Stable,
        "Workflow bundle badge owner",
        "The workflow bundle launch card renders the shared badges so an LTS-surface bundle on the LTS channel reads as long-term-supported, while a bundle with a scheduled removal date points to its replacement bundle with a complete-migration-before-removal next action rather than becoming an inert warning",
        "evidence:m5-channel-badge-parity:001",
        vec![
            case(
                "aureline workflow bundle: certified release train",
                Life::LtsSurface,
                Chan::Lts,
                None,
                "2026-07-03T00:00:00Z",
            ),
            case(
                "aureline workflow bundle: legacy import pipeline",
                Life::RemovalScheduled,
                Chan::Stable,
                Some("migration:bundle/import-pipeline-v3"),
                "2026-02-11T00:00:00Z",
            ),
        ],
    ),

    // 4. Extension / install row — a labs extension on the nightly channel, and a
    //    preview extension on the preview channel, both reading pre-release maturity
    //    without borrowing the channel's meaning.
    base_row(
        M5MaturityBadgeConsumerSurface::ExtensionInstallRow,
        M5BadgeQualificationClass::Stable,
        "Extension install badge owner",
        "The extension / install row renders the shared badges so a labs extension on the nightly channel reads as experimental-maturity and a preview extension on the preview channel reads as preview-maturity — the same two-cue vocabulary an install reviewer reads elsewhere, with the lifecycle and channel stated separately",
        "evidence:m5-channel-badge-parity:002",
        vec![
            case(
                "aureline extension: experimental planner",
                Life::Labs,
                Chan::Nightly,
                None,
                "2026-04-01T00:00:00Z",
            ),
            case(
                "aureline extension: preview theme pack",
                Life::Preview,
                Chan::Preview,
                None,
                "2026-06-25T00:00:00Z",
            ),
        ],
    ),

    // 5. Release / install surface — a stable release on the stable channel, and an
    //    LTS-surface capability running on the nightly channel for validation (another
    //    distinct-cues proof).
    base_row(
        M5MaturityBadgeConsumerSurface::ReleaseInstallSurface,
        M5BadgeQualificationClass::Stable,
        "Release install badge owner",
        "The release / install surface renders the shared badges so a stable release on the stable channel reads as stable-maturity, while an LTS-surface capability being validated on the nightly channel still reads as long-term-supported — the channel a thing is running on never narrows or widens its lifecycle stage",
        "evidence:m5-lifecycle-badge-parity:003",
        vec![
            case(
                "aureline release: desktop app 5.2",
                Life::Stable,
                Chan::Stable,
                None,
                "2026-07-04T00:00:00Z",
            ),
            case(
                "aureline release: lts kernel validation",
                Life::LtsSurface,
                Chan::Nightly,
                None,
                "2026-06-30T00:00:00Z",
            ),
        ],
    ),

    // 6. Ecosystem lifecycle review — a deprecated capability being reviewed on the beta
    //    channel that still points to its replacement, and a beta capability that has
    //    been promoted to the stable channel (a pre-release lifecycle on the stable
    //    channel).
    base_row(
        M5MaturityBadgeConsumerSurface::EcosystemLifecycleReview,
        M5BadgeQualificationClass::Stable,
        "Ecosystem lifecycle badge owner",
        "The ecosystem lifecycle review lane renders the shared badges so a deprecated capability under review on the beta channel still points to its replacement with a preserved channel context, and a beta capability promoted to the stable channel still reads as beta-maturity — support for lifecycle and channel stay separate facts a reviewer reads together",
        "evidence:m5-channel-badge-parity:003",
        vec![
            case(
                "aureline ecosystem: legacy provider connector",
                Life::Deprecated,
                Chan::Beta,
                Some("migration:provider/native-connector"),
                "2026-03-18T00:00:00Z",
            ),
            case(
                "aureline ecosystem: staged planner beta",
                Life::Beta,
                Chan::Stable,
                None,
                "2026-06-28T00:00:00Z",
            ),
        ],
    ),
    ]
}

fn governance_review() -> M5MaturityBadgeGovernanceReview {
    M5MaturityBadgeGovernanceReview {
        lifecycle_and_channel_shown_as_distinct_cues: true,
        neither_badge_collapsed_into_the_other: true,
        lifecycle_never_implies_channel: true,
        channel_never_implies_lifecycle: true,
        deprecated_or_removal_auto_points_to_migration_path: true,
        migration_note_preserves_channel_context: true,
        every_badge_opens_explanation_drawer: true,
        every_badge_is_separately_filterable: true,
        exported_evidence_keeps_badge_meaning: true,
        no_surface_invents_second_badge_grammar: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5MaturityBadgeConsumerProjection {
    M5MaturityBadgeConsumerProjection {
        command_feature_bundle_surfaces_consume_shared_badges: true,
        extension_install_release_surfaces_consume_shared_badges: true,
        lifecycle_filter_reads_single_source: true,
        channel_filter_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5MaturityBadgeProofFreshness {
    M5MaturityBadgeProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5MaturityBadgeReleasePosture {
    M5MaturityBadgeReleasePosture {
        release_packet_ref: M5_MATURITY_BADGE_ARTIFACT_REF.to_owned(),
        badge_audit_ref: M5_MATURITY_BADGE_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_MATURITY_BADGE_SCHEMA_REF,
        M5_MATURITY_BADGE_DOC_REF,
        M5_MATURITY_BADGE_FAMILY_MATRIX_REF,
        M5_MATURITY_BADGE_LIFECYCLE_REF,
        M5_MATURITY_BADGE_CHANNEL_REF,
    ])
}

/// Builds the canonical M5 lifecycle / channel badge primitive packet.
pub fn seeded_m5_maturity_badge_primitive_packet() -> M5MaturityBadgePrimitivePacket {
    M5MaturityBadgePrimitivePacket::new(M5MaturityBadgePrimitivePacketInput {
        packet_id: M5_MATURITY_BADGE_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 lifecycle and channel badge primitive: labs/preview/beta/stable/lts-surface/deprecated/removal-scheduled lifecycle and nightly/preview/beta/stable/lts channel as two distinct, composable cues"
                .to_owned(),
        badge_rows: badge_rows(),
        vocabulary_set: M5MaturityBadgeVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the extension / install row is held at Beta because a slice of
/// extension badges do not yet render the channel explanation drawer on every profile;
/// every badge consumer stays visible.
pub fn seeded_m5_maturity_badge_primitive_extension_install_row_beta_narrowed(
) -> M5MaturityBadgePrimitivePacket {
    let mut packet = seeded_m5_maturity_badge_primitive_packet();
    packet.packet_id = "m5-lifecycle-and-channel-badge-primitive:extension-beta:0001".to_owned();
    let row = packet
        .badge_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5MaturityBadgeConsumerSurface::ExtensionInstallRow)
        .expect("extension install row present");
    row.qualification = M5BadgeQualificationClass::Beta;
    packet
}

/// Narrowed variant: the ecosystem lifecycle review lane is narrowed to Preview pending
/// migration-path parity proof across every export path; every badge consumer stays
/// visible.
pub fn seeded_m5_maturity_badge_primitive_ecosystem_review_preview_narrowed(
) -> M5MaturityBadgePrimitivePacket {
    let mut packet = seeded_m5_maturity_badge_primitive_packet();
    packet.packet_id = "m5-lifecycle-and-channel-badge-primitive:ecosystem-preview:0001".to_owned();
    let row = packet
        .badge_rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5MaturityBadgeConsumerSurface::EcosystemLifecycleReview
        })
        .expect("ecosystem lifecycle review row present");
    row.qualification = M5BadgeQualificationClass::Preview;
    packet
}
