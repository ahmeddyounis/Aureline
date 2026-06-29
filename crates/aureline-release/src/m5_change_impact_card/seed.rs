//! Canonical seed builders for the change-impact card set.
//!
//! These builders are the single producer of the checked-in card-set packet, the published
//! inventory, the release-grade parity proof (and its Markdown report), the machine-readable per-card
//! CSV export, and the review / hold / speculative drill fixtures. The headless emitter and the inline
//! tests both call them so the in-code packet, the artifacts, and the fixtures never drift.
//!
//! The canonical packet forecasts a routine update: every dimension is clear to apply (no impact or
//! low-risk cache churn), so every consumer stands clear before restart. The drills perturb one
//! dimension and let the derivation recompute each consumer's readiness and gaps:
//!
//! - the **review** drill makes one dimension a *confirmed* migration, so the consumers that read it
//!   recommend review;
//! - the **hold** drill makes one dimension a *confirmed* destructive change, so those consumers hold
//!   for a pre-restart acknowledgement; and
//! - the **speculative** drill makes one dimension a destructive change forecast on *unknown* inputs,
//!   so it caps at review-recommended rather than becoming a hard failure — the lane's guardrail.

use super::*;

/// Stable packet id for the canonical change-impact card set.
pub const M5_CHANGE_IMPACT_CARD_SET_PACKET_ID: &str = "m5-change-impact-cards:stable:0001";

/// Evaluation / mint timestamp for the canonical packet.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

/// The dimension the review drill turns into a confirmed migration. Read by every consumer except the
/// admin console, so the drill narrows exactly those five.
const REVIEW_DRILL_DIMENSION: ImpactDimension = ImpactDimension::SchemaMigration;

/// The dimension the hold drill turns into a confirmed destructive change. Read by every consumer
/// except the migration assistant, so the drill holds exactly those five.
const HOLD_DRILL_DIMENSION: ImpactDimension = ImpactDimension::ExtensionCompatibility;

/// The dimension the speculative drill turns into a destructive forecast on unknown inputs. Read by
/// every consumer except the migration assistant, so the drill narrows (never blocks) those five.
const SPECULATIVE_DRILL_DIMENSION: ImpactDimension = ImpactDimension::BehaviorChange;

fn both_profiles() -> Vec<DeploymentProfile> {
    vec![DeploymentProfile::Managed, DeploymentProfile::SelfHosted]
}

/// Builds a clear-to-apply card for a dimension: no impact, confirmed, nothing to follow up.
fn clear_card(dimension: ImpactDimension) -> ChangeImpactCard {
    ChangeImpactCard::new(ChangeImpactCardInput {
        dimension,
        risk_class: RiskClass::NoImpact,
        confidence: ForecastConfidence::Confirmed,
        affected_artifact_classes: vec![dimension.primary_artifact_class()],
        affected_profiles: both_profiles(),
        from_version: None,
        to_version: None,
        follow_up: FollowUpTask::none(dimension),
        rollback_choice: RollbackChoice::NotApplicable,
        evidence_refs: vec![format!(
            "artifacts/release/m5-change-impact-proof/{}.evidence",
            dimension.as_str()
        )],
    })
}

/// The canonical, all-clear card set: a routine update with no significant impact.
fn canonical_cards() -> Vec<ChangeImpactCard> {
    vec![
        clear_card(ImpactDimension::WorkspaceMigration),
        clear_card(ImpactDimension::ProfileMigration),
        clear_card(ImpactDimension::SchemaMigration),
        // Cache migration: low-risk churn, recognized and auto-handled — distinct from a destructive
        // change. Still clear to apply.
        ChangeImpactCard::new(ChangeImpactCardInput {
            dimension: ImpactDimension::CacheMigration,
            risk_class: RiskClass::LowRiskCacheChurn,
            confidence: ForecastConfidence::Confirmed,
            affected_artifact_classes: vec![ArtifactClass::CoreRuntime],
            affected_profiles: both_profiles(),
            from_version: None,
            to_version: None,
            follow_up: FollowUpTask::new(
                ImpactDimension::CacheMigration,
                FollowUpTaskClass::CacheRebuild,
                TaskTiming::AfterRestart,
                TaskAutomation::Automatic,
                &["artifacts/release/m5-change-impact-proof/cache_rebuild.task"],
            ),
            rollback_choice: RollbackChoice::NotApplicable,
            evidence_refs: vec![
                "artifacts/release/m5-change-impact-proof/cache_migration.evidence".to_owned(),
            ],
        }),
        clear_card(ImpactDimension::ExtensionCompatibility),
        // Remote-helper skew: not applicable when no remote is configured — labeled honestly.
        ChangeImpactCard::new(ChangeImpactCardInput {
            dimension: ImpactDimension::RemoteHelperSkew,
            risk_class: RiskClass::NoImpact,
            confidence: ForecastConfidence::NotApplicable,
            affected_artifact_classes: vec![ArtifactClass::CoreRuntime],
            affected_profiles: vec![DeploymentProfile::Managed],
            from_version: None,
            to_version: None,
            follow_up: FollowUpTask::none(ImpactDimension::RemoteHelperSkew),
            rollback_choice: RollbackChoice::NotApplicable,
            evidence_refs: vec![
                "artifacts/release/m5-change-impact-proof/remote_helper_skew.evidence".to_owned(),
            ],
        }),
        clear_card(ImpactDimension::ToolchainFloor),
        clear_card(ImpactDimension::ToolchainCeiling),
        clear_card(ImpactDimension::CertifiedArchetype),
        clear_card(ImpactDimension::BehaviorChange),
    ]
}

/// The claimed consumer rows. The update center, release center, team-lead review, and support export
/// read every dimension; the migration assistant reads only the migration dimensions; the admin
/// console reads the compatibility / skew / toolchain / archetype / behavior dimensions.
fn consumer_rows() -> Vec<ImpactConsumerRow> {
    vec![
        ImpactConsumerRow::new(ImpactConsumer::UpdateCenter, &ImpactDimension::ALL),
        ImpactConsumerRow::new(
            ImpactConsumer::MigrationAssistant,
            &[
                ImpactDimension::WorkspaceMigration,
                ImpactDimension::ProfileMigration,
                ImpactDimension::SchemaMigration,
                ImpactDimension::CacheMigration,
            ],
        ),
        ImpactConsumerRow::new(ImpactConsumer::ReleaseCenter, &ImpactDimension::ALL),
        ImpactConsumerRow::new(ImpactConsumer::TeamLeadReview, &ImpactDimension::ALL),
        ImpactConsumerRow::new(
            ImpactConsumer::AdminConsole,
            &[
                ImpactDimension::ExtensionCompatibility,
                ImpactDimension::RemoteHelperSkew,
                ImpactDimension::ToolchainFloor,
                ImpactDimension::ToolchainCeiling,
                ImpactDimension::CertifiedArchetype,
                ImpactDimension::BehaviorChange,
            ],
        ),
        ImpactConsumerRow::new(ImpactConsumer::SupportExport, &ImpactDimension::ALL),
    ]
}

/// Replaces the card for a dimension with a perturbed card.
fn with_card(
    mut cards: Vec<ChangeImpactCard>,
    dimension: ImpactDimension,
    replacement: ChangeImpactCard,
) -> Vec<ChangeImpactCard> {
    for card in &mut cards {
        if card.dimension == dimension {
            *card = replacement.clone();
        }
    }
    cards
}

/// The canonical forecast target.
fn canonical_target() -> ChangeImpactTarget {
    ChangeImpactTarget {
        channel: ChannelScope::Stable,
        profiles: both_profiles(),
        current_version: "1.8.0".to_owned(),
        target_version: "1.9.0".to_owned(),
        forecast_basis: ForecastBasis::ReleaseAndLocalScan,
    }
}

/// Assembles a packet from the given cards.
fn assemble_packet(
    packet_id: &str,
    report_label: &str,
    cards: Vec<ChangeImpactCard>,
) -> ChangeImpactCardSet {
    ChangeImpactCardSet::new(ChangeImpactCardSetInput {
        packet_id: packet_id.to_owned(),
        report_label: report_label.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        target: canonical_target(),
        cards,
        consumers: consumer_rows(),
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// The canonical, all-clear change-impact card set: a routine update with no significant impact, so
/// every consumer is clear to apply before restart.
pub fn seeded_m5_change_impact_card_set() -> ChangeImpactCardSet {
    assemble_packet(
        M5_CHANGE_IMPACT_CARD_SET_PACKET_ID,
        "Aureline M5 change-impact cards",
        canonical_cards(),
    )
}

/// Drill: one dimension is a confirmed migration, so the consumers that read it recommend review
/// while the rest stay clear and no pre-restart acknowledgement is required.
pub fn seeded_m5_change_impact_card_set_review() -> ChangeImpactCardSet {
    let card = ChangeImpactCard::new(ChangeImpactCardInput {
        dimension: REVIEW_DRILL_DIMENSION,
        risk_class: RiskClass::MigrationRequired,
        confidence: ForecastConfidence::Confirmed,
        affected_artifact_classes: vec![
            ArtifactClass::SchemaContracts,
            ArtifactClass::WorkspaceState,
        ],
        affected_profiles: both_profiles(),
        from_version: Some("12".to_owned()),
        to_version: Some("13".to_owned()),
        follow_up: FollowUpTask::new(
            REVIEW_DRILL_DIMENSION,
            FollowUpTaskClass::MigrationScanRequired,
            TaskTiming::BeforeRestart,
            TaskAutomation::AssistantAvailable,
            &["artifacts/release/m5-change-impact-proof/schema_migration.task"],
        ),
        rollback_choice: RollbackChoice::RollbackSupported,
        evidence_refs: vec![
            "artifacts/release/m5-change-impact-proof/schema_migration.evidence".to_owned(),
        ],
    });
    assemble_packet(
        "m5-change-impact-cards:drill-review:0001",
        "Aureline M5 change-impact cards — review drill",
        with_card(canonical_cards(), REVIEW_DRILL_DIMENSION, card),
    )
}

/// Drill: one dimension is a confirmed destructive change, so the consumers that read it hold for a
/// pre-restart acknowledgement while the rest stay clear.
pub fn seeded_m5_change_impact_card_set_hold() -> ChangeImpactCardSet {
    let card = ChangeImpactCard::new(ChangeImpactCardInput {
        dimension: HOLD_DRILL_DIMENSION,
        risk_class: RiskClass::DestructiveChange,
        confidence: ForecastConfidence::Confirmed,
        affected_artifact_classes: vec![ArtifactClass::ExtensionPacks],
        affected_profiles: both_profiles(),
        from_version: Some("3.2.1".to_owned()),
        to_version: Some("3.4.0".to_owned()),
        follow_up: FollowUpTask::new(
            HOLD_DRILL_DIMENSION,
            FollowUpTaskClass::ExtensionRangeUpdate,
            TaskTiming::BeforeRestart,
            TaskAutomation::ManualStepsRequired,
            &["artifacts/release/m5-change-impact-proof/extension_compatibility.task"],
        ),
        rollback_choice: RollbackChoice::PinCurrentVersion,
        evidence_refs: vec![
            "artifacts/release/m5-change-impact-proof/extension_compatibility.evidence".to_owned(),
        ],
    });
    assemble_packet(
        "m5-change-impact-cards:drill-hold:0001",
        "Aureline M5 change-impact cards — hold drill",
        with_card(canonical_cards(), HOLD_DRILL_DIMENSION, card),
    )
}

/// Drill: one dimension is a destructive change forecast on *unknown* inputs. The guardrail caps it at
/// review-recommended rather than a hard failure, so the consumers that read it recommend review and
/// no pre-restart acknowledgement is forced.
pub fn seeded_m5_change_impact_card_set_speculative() -> ChangeImpactCardSet {
    let card = ChangeImpactCard::new(ChangeImpactCardInput {
        dimension: SPECULATIVE_DRILL_DIMENSION,
        risk_class: RiskClass::DestructiveChange,
        confidence: ForecastConfidence::Unknown,
        affected_artifact_classes: vec![ArtifactClass::CoreRuntime],
        affected_profiles: both_profiles(),
        from_version: None,
        to_version: None,
        follow_up: FollowUpTask::new(
            SPECULATIVE_DRILL_DIMENSION,
            FollowUpTaskClass::ReviewBeforeRestart,
            TaskTiming::ManualReviewOnly,
            TaskAutomation::ManualStepsRequired,
            &["artifacts/release/m5-change-impact-proof/behavior_change.task"],
        ),
        rollback_choice: RollbackChoice::SideBySideFallback,
        evidence_refs: vec![
            "artifacts/release/m5-change-impact-proof/behavior_change.evidence".to_owned(),
        ],
    });
    assemble_packet(
        "m5-change-impact-cards:drill-speculative:0001",
        "Aureline M5 change-impact cards — speculative-input drill",
        with_card(canonical_cards(), SPECULATIVE_DRILL_DIMENSION, card),
    )
}
