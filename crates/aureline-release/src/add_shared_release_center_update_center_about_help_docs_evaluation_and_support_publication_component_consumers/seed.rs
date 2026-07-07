//! Canonical seed builders for the M5 publication-component-consumer lane.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call them
//! so the in-code matrix, the artifact, the worked bindings, and the fixtures
//! never drift.

use super::*;

/// Stable packet id for the canonical publication-component-consumer packet.
pub const M5_PUBLICATION_COMPONENT_CONSUMER_PACKET_ID: &str =
    "m5-publication-component-consumer:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-06T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked binding case for one consumer/family adoption.
fn case(
    consumer: M5PublicationComponentConsumer,
    component_family: M5PublicationComponentFamily,
    client_scope_mode: M5ClientScopeMode,
    handoff_caveats: &[M5HandoffCaveat],
    note: &str,
) -> M5PublicationBindingCase {
    M5PublicationBindingCase::resolved(M5PublicationBindingInput {
        consumer,
        component_family,
        descriptor_families: M5PublicationDescriptor::ALL.to_vec(),
        client_scope_mode,
        handoff_caveats: handoff_caveats.to_vec(),
        note_repr: Some(note.to_owned()),
    })
}

/// Builds a component binding that points at its canonical family refs.
fn binding(
    component_family: M5PublicationComponentFamily,
    example_bindings: Vec<M5PublicationBindingCase>,
) -> M5PublicationComponentBinding {
    M5PublicationComponentBinding {
        component_family,
        canonical_schema_ref: component_family.canonical_schema_ref().to_owned(),
        canonical_artifact_ref: component_family.canonical_artifact_ref().to_owned(),
        references_canonical_not_local_prose: true,
        example_bindings,
    }
}

/// A base row with the shared parity vocabulary filled in.
fn base_row(
    consumer: M5PublicationComponentConsumer,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    component_bindings: Vec<M5PublicationComponentBinding>,
) -> M5PublicationConsumerRow {
    M5PublicationConsumerRow {
        consumer,
        qualification: M5ReleaseCenterQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5PublicationSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DeploymentLine::ALL.to_vec(),
        anatomy_parts: M5PublicationConsumerAnatomyPart::ALL.to_vec(),
        descriptor_families: M5PublicationDescriptor::ALL.to_vec(),
        client_scope_modes: M5ClientScopeMode::ALL.to_vec(),
        handoff_caveats: M5HandoffCaveat::ALL.to_vec(),
        descriptor_parity_states: M5DescriptorParityState::ALL.to_vec(),
        reduced_scope_reasons: M5ReducedScopeReason::ALL.to_vec(),
        scope_next_actions: M5ScopeNextAction::ALL.to_vec(),
        export_fields: M5PublicationConsumerExportField::ALL.to_vec(),
        accessibility_routes: M5ReleaseCenterAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ReleaseCenterConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5ReleaseCenterDowngradeTrigger::CandidateScopeUnstated,
            M5ReleaseCenterDowngradeTrigger::BlockerFreshnessHidden,
            M5ReleaseCenterDowngradeTrigger::ProofStale,
        ],
        component_bindings,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_PUBLICATION_CONSUMER_SCHEMA_REF,
            M5_PUBLICATION_CONSUMER_OBJECT_MODEL_REF,
        ]),
        rewords_descriptors_per_surface: false,
        invents_new_badge_vocabulary: false,
        drops_provenance_or_freshness_when_narrowed: false,
        hides_mirror_or_offline_handoff_caveat: false,
    }
}

fn consumer_rows() -> Vec<M5PublicationConsumerRow> {
    use M5ClientScopeMode as Scope;
    use M5HandoffCaveat as Caveat;
    use M5PublicationComponentConsumer as Consumer;
    use M5PublicationComponentFamily as Family;

    let mut rows = Vec::new();

    // 1. Release center — the authoritative full-scope rendering of the
    //    release-candidate card and the version-bump / publish-target review.
    rows.push(base_row(
        Consumer::ReleaseCenter,
        "Release-center surface owner",
        "The release center adopts the release-candidate card and version-bump / publish-target primitives at full client scope, pointing at their canonical schemas so provenance, freshness, qualification, and client-scope descriptors stay identical to what the update center, docs, evaluation packet, and support export read",
        "evidence:m5-publication-consumer-release-center:001",
        vec![
            binding(
                Family::ReleaseCandidateCard,
                vec![case(
                    Consumer::ReleaseCenter,
                    Family::ReleaseCandidateCard,
                    Scope::FullClientScope,
                    &[],
                    "release-center candidate card at full scope",
                )],
            ),
            binding(
                Family::VersionBumpPublishTarget,
                vec![case(
                    Consumer::ReleaseCenter,
                    Family::VersionBumpPublishTarget,
                    Scope::FullClientScope,
                    &[],
                    "release-center publish-target review at full scope",
                )],
            ),
        ],
    ));

    // 2. Update center — the release-candidate card under a narrowed client scope,
    //    and the promotion / rollback history at full scope.
    rows.push(base_row(
        Consumer::UpdateCenter,
        "Update-center surface owner",
        "The update center adopts the release-candidate card under a narrowed client scope (a client-limited view) and the promotion / rollback history at full scope, disclosing the narrowing with a self-contained banner while keeping the same descriptor vocabulary the release center uses",
        "evidence:m5-publication-consumer-update-center:001",
        vec![
            binding(
                Family::ReleaseCandidateCard,
                vec![case(
                    Consumer::UpdateCenter,
                    Family::ReleaseCandidateCard,
                    Scope::NarrowedClientScope,
                    &[Caveat::CompanionDeepLink],
                    "update-center candidate card narrowed to the installed client",
                )],
            ),
            binding(
                Family::PromotionRollbackHistory,
                vec![case(
                    Consumer::UpdateCenter,
                    Family::PromotionRollbackHistory,
                    Scope::FullClientScope,
                    &[],
                    "update-center rollback history at full scope",
                )],
            ),
        ],
    ));

    // 3. About / help — the provenance bundle under a browser / companion handoff,
    //    and the release-candidate card at full scope.
    rows.push(base_row(
        Consumer::AboutHelp,
        "About / help surface owner",
        "The About/help surface adopts the artifact provenance bundle under a browser / companion handoff and the release-candidate card at full scope, referencing the canonical component schemas so its prose can never drift from the product truth and preserving the browser handoff caveat",
        "evidence:m5-publication-consumer-about-help:001",
        vec![
            binding(
                Family::ArtifactProvenanceBundle,
                vec![case(
                    Consumer::AboutHelp,
                    Family::ArtifactProvenanceBundle,
                    Scope::BrowserCompanionHandoff,
                    &[Caveat::BrowserReadOnly, Caveat::CompanionDeepLink],
                    "about/help provenance bundle rendered via browser handoff",
                )],
            ),
            binding(
                Family::ReleaseCandidateCard,
                vec![case(
                    Consumer::AboutHelp,
                    Family::ReleaseCandidateCard,
                    Scope::FullClientScope,
                    &[],
                    "about/help candidate summary at full scope",
                )],
            ),
        ],
    ));

    // 4. Docs portal — the promotion / rollback history from a mirror / offline
    //    snapshot, and the provenance bundle at full scope.
    rows.push(base_row(
        Consumer::DocsPortal,
        "Docs-portal surface owner",
        "The docs portal adopts the promotion / rollback history from a mirror / offline snapshot and the provenance bundle at full scope, referencing the canonical component schemas and preserving the mirror-replication-lag and offline-snapshot caveats when the history renders outside the release center",
        "evidence:m5-publication-consumer-docs:001",
        vec![
            binding(
                Family::PromotionRollbackHistory,
                vec![case(
                    Consumer::DocsPortal,
                    Family::PromotionRollbackHistory,
                    Scope::MirrorOfflineScope,
                    &[Caveat::MirrorReplicationLag, Caveat::OfflineSnapshot],
                    "docs promotion history from an offline mirror snapshot",
                )],
            ),
            binding(
                Family::ArtifactProvenanceBundle,
                vec![case(
                    Consumer::DocsPortal,
                    Family::ArtifactProvenanceBundle,
                    Scope::FullClientScope,
                    &[],
                    "docs provenance reference at full scope",
                )],
            ),
        ],
    ));

    // 5. Enterprise evaluation — the provenance bundle and the version-bump /
    //    publish-target review, both at full scope.
    rows.push(base_row(
        Consumer::EnterpriseEvaluation,
        "Enterprise-evaluation packet owner",
        "The enterprise-evaluation packet adopts the artifact provenance bundle and the version-bump / publish-target review at full scope, reading the same provenance, freshness, qualification, and client-scope descriptors so an evaluation reviewer sees identical facts to the product UI",
        "evidence:m5-publication-consumer-evaluation:001",
        vec![
            binding(
                Family::ArtifactProvenanceBundle,
                vec![case(
                    Consumer::EnterpriseEvaluation,
                    Family::ArtifactProvenanceBundle,
                    Scope::FullClientScope,
                    &[],
                    "evaluation provenance bundle at full scope",
                )],
            ),
            binding(
                Family::VersionBumpPublishTarget,
                vec![case(
                    Consumer::EnterpriseEvaluation,
                    Family::VersionBumpPublishTarget,
                    Scope::FullClientScope,
                    &[],
                    "evaluation publish-target review at full scope",
                )],
            ),
        ],
    ));

    // 6. Support export — the version-bump / publish-target review and the
    //    promotion / rollback history, both at full scope.
    rows.push(base_row(
        Consumer::SupportExport,
        "Support-export owner",
        "The support export adopts the version-bump / publish-target review and the promotion / rollback history at full scope, reconstructing consumer parity from the shared model so a support reviewer reads the same descriptor vocabulary that every product surface shows",
        "evidence:m5-publication-consumer-support:001",
        vec![
            binding(
                Family::VersionBumpPublishTarget,
                vec![case(
                    Consumer::SupportExport,
                    Family::VersionBumpPublishTarget,
                    Scope::FullClientScope,
                    &[],
                    "support publish-target review at full scope",
                )],
            ),
            binding(
                Family::PromotionRollbackHistory,
                vec![case(
                    Consumer::SupportExport,
                    Family::PromotionRollbackHistory,
                    Scope::FullClientScope,
                    &[],
                    "support rollback history at full scope",
                )],
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5PublicationConsumerGovernanceReview {
    M5PublicationConsumerGovernanceReview {
        consumers_adopt_shared_primitives: true,
        consumers_reference_canonical_schema: true,
        descriptor_vocabulary_shared_not_reworded: true,
        no_consumer_invents_new_badge: true,
        descriptors_explicit_on_every_surface: true,
        mirror_offline_and_handoff_caveats_preserved: true,
        narrowed_rendering_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_next_action: true,
        support_export_reconstructs_consumer_parity: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5PublicationConsumerProjection {
    M5PublicationConsumerProjection {
        all_consumers_adopt_shared_components: true,
        provenance_reads_single_source: true,
        freshness_reads_single_source: true,
        qualification_reads_single_source: true,
        client_scope_reads_single_source: true,
    }
}

fn proof_freshness() -> M5PublicationConsumerProofFreshness {
    M5PublicationConsumerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5PublicationConsumerReleasePosture {
    M5PublicationConsumerReleasePosture {
        release_packet_ref: M5_PUBLICATION_CONSUMER_ARTIFACT_REF.to_owned(),
        consumer_audit_ref: M5_PUBLICATION_CONSUMER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_PUBLICATION_CONSUMER_SCHEMA_REF,
        M5_PUBLICATION_CONSUMER_DOC_REF,
        M5_PUBLICATION_CONSUMER_COMPONENT_MATRIX_REF,
        M5_PUBLICATION_CONSUMER_OBJECT_MODEL_REF,
        M5PublicationComponentFamily::ReleaseCandidateCard.canonical_schema_ref(),
        M5PublicationComponentFamily::VersionBumpPublishTarget.canonical_schema_ref(),
        M5PublicationComponentFamily::ArtifactProvenanceBundle.canonical_schema_ref(),
        M5PublicationComponentFamily::PromotionRollbackHistory.canonical_schema_ref(),
    ])
}

/// Builds the canonical M5 publication-component-consumer packet.
pub fn seeded_m5_publication_component_consumer_packet() -> M5PublicationComponentConsumerPacket {
    M5PublicationComponentConsumerPacket::new(M5PublicationComponentConsumerPacketInput {
        packet_id: M5_PUBLICATION_COMPONENT_CONSUMER_PACKET_ID.to_owned(),
        matrix_label:
            "M5 publication-component consumers: release center, update center, About/help, docs, enterprise evaluation, and support keep provenance, freshness, qualification, and client-scope parity"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5PublicationConsumerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the About/help surface is held at Beta because a slice of
/// About/help renderings do not yet expose the reduced-scope banner on every
/// browser-handoff path; every consumer stays visible.
pub fn seeded_m5_publication_component_consumer_about_help_handoff_narrowed(
) -> M5PublicationComponentConsumerPacket {
    let mut packet = seeded_m5_publication_component_consumer_packet();
    packet.packet_id = "m5-publication-component-consumer:about-help-beta:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5PublicationComponentConsumer::AboutHelp)
        .expect("about/help row present");
    row.qualification = M5ReleaseCenterQualificationClass::Beta;
    packet
}

/// Narrowed variant: the docs portal is narrowed to Preview pending mirror/offline
/// caveat-parity proof across every snapshot path; every consumer stays visible.
pub fn seeded_m5_publication_component_consumer_docs_mirror_offline_narrowed(
) -> M5PublicationComponentConsumerPacket {
    let mut packet = seeded_m5_publication_component_consumer_packet();
    packet.packet_id = "m5-publication-component-consumer:docs-preview:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5PublicationComponentConsumer::DocsPortal)
        .expect("docs-portal row present");
    row.qualification = M5ReleaseCenterQualificationClass::Preview;
    packet
}
