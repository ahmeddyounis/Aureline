//! Deterministic builder for the M5 provenance-card register.
//!
//! [`build_m5_provenance_cards`] constructs the same register that the checked-in
//! JSON embeds, so the headless emitter can regenerate the artifact and a test can
//! prove the embedded JSON never drifts from the code.

use crate::freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix::{
    AttestationAvailability, EvidenceCompleteness, ExactBuildIdentity, M5ArtifactFamilyKind,
    MirrorFreshness, MirrorOfflineExpectation, RollbackRevocationPosture, SbomScope,
    SymbolSourceMapAvailability,
};
use crate::release_center_model::{BlastRadiusClass, RollbackOrRevocationKind, SignatureStateClass};
use crate::stable_claim_manifest::{FreshnessSlo, FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, StableClaimLevel,
};

use super::{
    canonical_badge_state, CardAction, CardGapReason, CardState, M5ProvenanceCardRegister,
    M5ProvenanceCardSummary, ProvenanceBadge, ProvenanceBadgeKind, ProvenanceBadgeState,
    ProvenanceCard, ProvenanceCardStopRule, ProvenanceSurfaceKind, SurfaceBinding,
    M5_PROVENANCE_CARDS_RECORD_KIND, M5_PROVENANCE_CARDS_SCHEMA_VERSION,
};

const AS_OF: &str = "2026-06-15";
const SLO_REGISTER_REF: &str = "release/freshness_slo_register";
const TARGET_MAX_AGE_DAYS: u32 = 90;
const WARN_WITHIN_DAYS: u32 = 14;

/// Builds the canonical M5 provenance-card register in code.
pub fn build_m5_provenance_cards() -> M5ProvenanceCardRegister {
    let rows = vec![
        converged_card(ConvergedSpec {
            family_kind: M5ArtifactFamilyKind::NotebookPack,
            slug: "notebook-pack",
            title: "Notebook pack provenance card",
            summary: "Notebook packs and notebook-derived outputs, signed and attested.",
            claim_label: StableClaimLevel::Stable,
            owner: "notebook-release",
            signature_state: SignatureStateClass::Verified,
            attestation: AttestationAvailability::Attested,
            sbom_scope: SbomScope::FullGraph,
            symbols: SymbolSourceMapAvailability::Published,
            mirror: MirrorFreshness::Current,
            slo_state: FreshnessSloState::Current,
        }),
        converged_card(ConvergedSpec {
            family_kind: M5ArtifactFamilyKind::RequestDataAsset,
            slug: "request-data-asset",
            title: "Request/data asset provenance card",
            summary: "Saved requests, datasets, and fixtures, signed with retained symbols.",
            claim_label: StableClaimLevel::Stable,
            owner: "data-release",
            signature_state: SignatureStateClass::Verified,
            attestation: AttestationAvailability::Attested,
            sbom_scope: SbomScope::ComponentScoped,
            symbols: SymbolSourceMapAvailability::RetainedInternal,
            mirror: MirrorFreshness::Current,
            slo_state: FreshnessSloState::DueForRefresh,
        }),
        converged_card(ConvergedSpec {
            family_kind: M5ArtifactFamilyKind::ProfilerReplayArtifact,
            slug: "profiler-replay",
            title: "Profiler/replay artifact provenance card",
            summary: "Profiler traces and replay recordings, signed and symbolicated.",
            claim_label: StableClaimLevel::Stable,
            owner: "profiler-release",
            signature_state: SignatureStateClass::Verified,
            attestation: AttestationAvailability::Attested,
            sbom_scope: SbomScope::FullGraph,
            symbols: SymbolSourceMapAvailability::Published,
            mirror: MirrorFreshness::Current,
            slo_state: FreshnessSloState::Current,
        }),
        converged_card(ConvergedSpec {
            family_kind: M5ArtifactFamilyKind::FrameworkTemplatePack,
            slug: "framework-template-pack",
            title: "Framework/template pack provenance card",
            summary: "Framework and template packs, signed with an SBOM; no native symbols.",
            claim_label: StableClaimLevel::Stable,
            owner: "framework-release",
            signature_state: SignatureStateClass::Verified,
            attestation: AttestationAvailability::Attested,
            sbom_scope: SbomScope::FullGraph,
            symbols: SymbolSourceMapAvailability::NotApplicable,
            mirror: MirrorFreshness::Current,
            slo_state: FreshnessSloState::Current,
        }),
        converged_card(ConvergedSpec {
            family_kind: M5ArtifactFamilyKind::DocsPack,
            slug: "docs-pack",
            title: "Docs pack provenance card",
            summary: "User-facing and embedded documentation packs, signed and attested.",
            claim_label: StableClaimLevel::Lts,
            owner: "docs-release",
            signature_state: SignatureStateClass::Verified,
            attestation: AttestationAvailability::Attested,
            sbom_scope: SbomScope::NotApplicable,
            symbols: SymbolSourceMapAvailability::NotApplicable,
            mirror: MirrorFreshness::Current,
            slo_state: FreshnessSloState::Current,
        }),
        converged_card(ConvergedSpec {
            family_kind: M5ArtifactFamilyKind::ModelPack,
            slug: "model-pack",
            title: "Model pack provenance card",
            summary: "Local model bundles and metadata, signed with a full SBOM.",
            claim_label: StableClaimLevel::Stable,
            owner: "model-release",
            signature_state: SignatureStateClass::Verified,
            attestation: AttestationAvailability::Attested,
            sbom_scope: SbomScope::FullGraph,
            symbols: SymbolSourceMapAvailability::NotApplicable,
            mirror: MirrorFreshness::Current,
            slo_state: FreshnessSloState::DueForRefresh,
        }),
        converged_card(ConvergedSpec {
            family_kind: M5ArtifactFamilyKind::CompanionOffboardingPacket,
            slug: "companion-offboarding",
            title: "Companion/offboarding packet provenance card",
            summary: "Companion and offboarding packets, signed and offline-verifiable.",
            claim_label: StableClaimLevel::Stable,
            owner: "companion-release",
            signature_state: SignatureStateClass::Verified,
            attestation: AttestationAvailability::Attested,
            sbom_scope: SbomScope::ComponentScoped,
            symbols: SymbolSourceMapAvailability::NotApplicable,
            mirror: MirrorFreshness::NotApplicable,
            slo_state: FreshnessSloState::Current,
        }),
        managed_output_card(),
    ];

    let release_blocking_artifact_refs = rows
        .iter()
        .filter(|r| r.release_blocking)
        .map(|r| r.artifact_ref.clone())
        .collect();

    let mut register = M5ProvenanceCardRegister {
        schema_version: M5_PROVENANCE_CARDS_SCHEMA_VERSION,
        record_kind: M5_PROVENANCE_CARDS_RECORD_KIND.to_owned(),
        manifest_id: "m5-provenance-cards".to_owned(),
        status: "frozen".to_owned(),
        overview_page:
            "docs/m5/add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts.md"
                .to_owned(),
        as_of: AS_OF.to_owned(),
        claim_manifest_ref: "release/stable_claim_manifest".to_owned(),
        publication_matrix_ref:
            "release/freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix"
                .to_owned(),
        artifact_graph_ref:
            "release/implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs"
                .to_owned(),
        docs_help_about_truth_ref: "release/harden_docs_help_about_and_service_health_truth"
            .to_owned(),
        service_health_feed_ref: "service_health/feed".to_owned(),
        release_center_model_ref: "release/release_center_object_model".to_owned(),
        lifecycle_labels: StableClaimLevel::ALL.to_vec(),
        family_kinds: M5ArtifactFamilyKind::ALL.to_vec(),
        surface_kinds: ProvenanceSurfaceKind::ALL.to_vec(),
        badge_kinds: ProvenanceBadgeKind::ALL.to_vec(),
        badge_states: ProvenanceBadgeState::ALL.to_vec(),
        card_states: CardState::ALL.to_vec(),
        gap_reasons: CardGapReason::ALL.to_vec(),
        card_actions: CardAction::ALL.to_vec(),
        launch_cutline: launch_cutline(),
        release_blocking_artifact_refs,
        stop_rules: stop_rules(),
        rows,
        publication: PromotionDecisionRecord {
            promotion_gate: "m5_provenance_cards".to_owned(),
            decision: PromotionDecision::Proceed,
            blocking_rule_ids: Vec::new(),
            blocking_claim_ids: Vec::new(),
            rationale:
                "Every release-blocking family at or above the cutline converges one exact-build identity across About, Help, the release center, service health, and support/export, exposes honest copy-safe provenance badges, and verifies offline; the managed-output family already inherits a below-cutline claim, so it narrows without blocking the train."
                    .to_owned(),
        },
        summary: placeholder_summary(),
    };

    register.publication.decision = register.computed_publication_decision();
    register.publication.blocking_rule_ids = register.computed_blocking_rule_ids();
    register.publication.blocking_claim_ids = register.computed_blocking_entry_ids();
    register.summary = register.computed_summary();
    register
}

fn launch_cutline() -> LaunchCutline {
    LaunchCutline {
        cutline_level: StableClaimLevel::Stable,
        above_cutline_levels: StableClaimLevel::ABOVE_CUTLINE.to_vec(),
        below_cutline_levels: StableClaimLevel::BELOW_CUTLINE.to_vec(),
        description:
            "A family converges at or above the cutline only when every user-visible surface — About, Help, release center, service health, support, and export — renders the same exact-build identity and provenance, when its signature/attestation/SBOM/symbol/mirror/rollback state holds, when its badges imply no stronger trust than the build actually carries, when the provenance verifies offline and under a mirror profile, when its proof packet is within SLO, and when it is owner-signed; otherwise it narrows below stable."
                .to_owned(),
    }
}

fn stop_rules() -> Vec<ProvenanceCardStopRule> {
    let rule = |id: &str,
                title: &str,
                trigger_reason: CardGapReason,
                default_action: CardAction,
                rationale: &str| ProvenanceCardStopRule {
        rule_id: id.to_owned(),
        title: title.to_owned(),
        trigger_reason,
        applies_to_labels: StableClaimLevel::ABOVE_CUTLINE.to_vec(),
        default_action,
        blocks_publication: true,
        rationale: rationale.to_owned(),
    };
    vec![
        rule(
            "stop-signature-missing",
            "Signature missing",
            CardGapReason::SignatureMissing,
            CardAction::ReSignArtifact,
            "A family without a verified release signature narrows; About/Help may not claim a signed build.",
        ),
        rule(
            "stop-signature-revoked",
            "Signature revoked",
            CardGapReason::SignatureRevoked,
            CardAction::ReSignArtifact,
            "A family whose release signature was revoked narrows below the cutline.",
        ),
        rule(
            "stop-attestation-missing",
            "Attestation missing",
            CardGapReason::AttestationMissing,
            CardAction::ReAttest,
            "A family without an available build attestation may not claim an attested build.",
        ),
        rule(
            "stop-sbom-incomplete",
            "SBOM incomplete",
            CardGapReason::SbomIncomplete,
            CardAction::RegenerateSbom,
            "A family with a partial or missing SBOM narrows; the SPDX/CycloneDX badge cannot claim availability.",
        ),
        rule(
            "stop-symbols-missing",
            "Symbols missing",
            CardGapReason::SymbolsMissing,
            CardAction::PublishSymbols,
            "A family whose symbols were stripped or are missing narrows; the symbol badge cannot claim availability.",
        ),
        rule(
            "stop-mirror-stale",
            "Mirror stale",
            CardGapReason::MirrorStale,
            CardAction::RefreshMirror,
            "A family whose mirror copy is stale or unpublished narrows; the mirror badge cannot claim a current copy.",
        ),
        rule(
            "stop-rollback-target-missing",
            "Rollback target missing",
            CardGapReason::RollbackTargetMissing,
            CardAction::RecordRollbackTarget,
            "A family without a recorded rollback target narrows below the cutline.",
        ),
        rule(
            "stop-exact-build-linkage-broken",
            "Exact-build linkage broken",
            CardGapReason::ExactBuildLinkageBroken,
            CardAction::RebuildExactBuild,
            "A family missing its one-build identity or provenance ref narrows; surfaces cannot converge.",
        ),
        rule(
            "stop-offline-unverifiable",
            "Offline-unverifiable provenance",
            CardGapReason::OfflineUnverifiable,
            CardAction::RestoreOfflineParity,
            "A family whose provenance needs live vendor connectivity narrows; provenance must survive offline and mirror profiles.",
        ),
        rule(
            "stop-surface-divergent",
            "Surface divergent",
            CardGapReason::SurfaceDivergent,
            CardAction::ReconcileSurfaces,
            "A family whose surfaces render different build identities narrows; About/Help/service-health/support/export must agree.",
        ),
        rule(
            "stop-surface-missing",
            "Surface missing",
            CardGapReason::SurfaceMissing,
            CardAction::RenderMissingSurface,
            "A family missing a required user-visible surface narrows; release-center-only truth is not allowed.",
        ),
        rule(
            "stop-evidence-incomplete",
            "Evidence incomplete",
            CardGapReason::EvidenceIncomplete,
            CardAction::RecaptureEvidence,
            "A family whose release evidence is incomplete narrows below the cutline.",
        ),
        rule(
            "stop-proof-packet-stale",
            "Proof packet stale",
            CardGapReason::ProofPacketStale,
            CardAction::RefreshProofPacket,
            "A proof packet outside its freshness SLO narrows the family.",
        ),
        rule(
            "stop-proof-packet-missing",
            "Proof packet missing",
            CardGapReason::ProofPacketMissing,
            CardAction::RefreshProofPacket,
            "A family without a captured proof packet narrows below the cutline.",
        ),
        rule(
            "stop-waiver-expired",
            "Waiver expired",
            CardGapReason::WaiverExpired,
            CardAction::RenewWaiver,
            "A family relying on an expired waiver narrows below the cutline.",
        ),
        rule(
            "stop-owner-signoff-missing",
            "Owner sign-off missing",
            CardGapReason::OwnerSignoffMissing,
            CardAction::RequestOwnerSignoff,
            "A family without owner sign-off cannot hold its claimed label.",
        ),
    ]
}

struct ConvergedSpec {
    family_kind: M5ArtifactFamilyKind,
    slug: &'static str,
    title: &'static str,
    summary: &'static str,
    claim_label: StableClaimLevel,
    owner: &'static str,
    signature_state: SignatureStateClass,
    attestation: AttestationAvailability,
    sbom_scope: SbomScope,
    symbols: SymbolSourceMapAvailability,
    mirror: MirrorFreshness,
    slo_state: FreshnessSloState,
}

fn exact_build(
    slug: &str,
    signature_state: SignatureStateClass,
    attestation: AttestationAvailability,
    sbom_scope: SbomScope,
    symbols: SymbolSourceMapAvailability,
    mirror: MirrorFreshness,
    evidence: EvidenceCompleteness,
) -> ExactBuildIdentity {
    ExactBuildIdentity {
        build_identity_ref: format!("exact_build/m5-{slug}"),
        provenance_ref: format!("provenance/m5-{slug}"),
        signature_state,
        attestation_availability: attestation,
        sbom_scope,
        symbol_availability: symbols,
        mirror_freshness: mirror,
        rollback_target_ref: format!("rollback/m5-{slug}/last-known-good"),
        evidence_completeness: evidence,
    }
}

fn rollback_posture(slug: &str) -> RollbackRevocationPosture {
    RollbackRevocationPosture {
        kind: RollbackOrRevocationKind::Rollback,
        blast_radius: BlastRadiusClass::ArtifactFamilyScoped,
        revocable: true,
        posture_ref: format!("rollback_posture/m5-{slug}"),
        summary: "Last-known-good rollback and revocation are recorded for the family.".to_owned(),
    }
}

fn mirror_offline(
    slug: &str,
    offline_verifiable: bool,
    mirror_publish_expected: bool,
) -> MirrorOfflineExpectation {
    MirrorOfflineExpectation {
        mirror_publish_expected,
        offline_verifiable,
        parity_ref: format!("mirror_offline/m5-{slug}"),
        summary: if offline_verifiable {
            "Provenance verifies offline and under a mirror profile without contacting the origin."
                .to_owned()
        } else {
            "Provenance currently needs live vendor connectivity to verify.".to_owned()
        },
    }
}

fn badge_kind_label(kind: ProvenanceBadgeKind) -> &'static str {
    match kind {
        ProvenanceBadgeKind::BuildIdentity => "Exact-build identity",
        ProvenanceBadgeKind::Signature => "Signature",
        ProvenanceBadgeKind::Attestation => "Attestation",
        ProvenanceBadgeKind::SpdxSbom => "SPDX SBOM",
        ProvenanceBadgeKind::CycloneDxExport => "CycloneDX export",
        ProvenanceBadgeKind::Symbols => "Symbols",
        ProvenanceBadgeKind::Mirror => "Mirror",
        ProvenanceBadgeKind::Rollback => "Rollback",
    }
}

fn badge_state_label(state: ProvenanceBadgeState) -> &'static str {
    match state {
        ProvenanceBadgeState::Verified => "verified",
        ProvenanceBadgeState::Available => "available",
        ProvenanceBadgeState::Official => "official",
        ProvenanceBadgeState::Mirrored => "mirrored",
        ProvenanceBadgeState::Partial => "partial",
        ProvenanceBadgeState::Pending => "pending",
        ProvenanceBadgeState::Revoked => "revoked",
        ProvenanceBadgeState::NotProvided => "not provided",
    }
}

/// The canonical, honest copy-safe badge set for an exact-build identity.
fn canonical_badges(
    exact_build: &ExactBuildIdentity,
    rollback: &RollbackRevocationPosture,
) -> Vec<ProvenanceBadge> {
    ProvenanceBadgeKind::ALL
        .iter()
        .map(|&kind| {
            let state = canonical_badge_state(kind, exact_build, rollback);
            ProvenanceBadge {
                kind,
                state,
                machine_token: format!("{}:{}", kind.as_str(), state.as_str()),
                label: format!("{}: {}", badge_kind_label(kind), badge_state_label(state)),
                copyable: true,
            }
        })
        .collect()
}

fn surface_binding(
    surface: ProvenanceSurfaceKind,
    slug: &str,
    build_identity_ref: String,
    provenance_ref: String,
    offline_available: bool,
) -> SurfaceBinding {
    SurfaceBinding {
        surface,
        surface_ref: format!("surface/{}/m5-{slug}", surface.as_str()),
        build_identity_ref,
        provenance_ref,
        renders_badges: true,
        copyable: true,
        offline_available,
        summary: format!(
            "The {} surface renders the family's exact-build identity and copy-safe provenance badges.",
            surface.as_str()
        ),
    }
}

/// Converged surface bindings: every surface renders the same identity offline.
fn converged_surfaces(slug: &str, exact_build: &ExactBuildIdentity) -> Vec<SurfaceBinding> {
    ProvenanceSurfaceKind::ALL
        .iter()
        .map(|&surface| {
            surface_binding(
                surface,
                slug,
                exact_build.build_identity_ref.clone(),
                exact_build.provenance_ref.clone(),
                true,
            )
        })
        .collect()
}

fn proof_packet(slug: &str, slo_state: FreshnessSloState) -> ProofPacket {
    let captured_at = if slo_state == FreshnessSloState::Missing {
        None
    } else {
        Some(AS_OF.to_owned())
    };
    let evidence_refs = if slo_state == FreshnessSloState::Missing {
        Vec::new()
    } else {
        vec![format!("evidence/proof/m5-{slug}")]
    };
    ProofPacket {
        packet_id: format!("packet-m5-{slug}"),
        packet_ref: format!("proof/m5-{slug}"),
        proof_index_ref: format!("proof_index/m5-{slug}"),
        captured_at,
        freshness_slo: FreshnessSlo {
            target_max_age_days: TARGET_MAX_AGE_DAYS,
            warn_within_days: WARN_WITHIN_DAYS,
            slo_register_ref: SLO_REGISTER_REF.to_owned(),
        },
        slo_state,
        evidence_refs,
    }
}

fn signed(owner: &str) -> OwnerSignoff {
    OwnerSignoff {
        owner_ref: owner.to_owned(),
        signed_off: true,
        signed_at: Some(AS_OF.to_owned()),
    }
}

fn destinations() -> Vec<String> {
    vec![
        "about".to_owned(),
        "help".to_owned(),
        "release_center".to_owned(),
        "service_health".to_owned(),
        "support_export".to_owned(),
    ]
}

fn converged_card(spec: ConvergedSpec) -> ProvenanceCard {
    let slug = spec.slug;
    let exact_build = exact_build(
        slug,
        spec.signature_state,
        spec.attestation,
        spec.sbom_scope,
        spec.symbols,
        spec.mirror,
        EvidenceCompleteness::Complete,
    );
    let rollback = rollback_posture(slug);
    let badges = canonical_badges(&exact_build, &rollback);
    let surface_bindings = converged_surfaces(slug, &exact_build);

    ProvenanceCard {
        entry_id: format!("card-{slug}"),
        title: spec.title.to_owned(),
        family_kind: spec.family_kind,
        artifact_ref: format!("artifact/m5/{slug}"),
        artifact_summary: spec.summary.to_owned(),
        release_blocking: true,
        claim_ref: format!("claim/m5-{slug}"),
        claim_label: spec.claim_label,
        card_state: CardState::Converged,
        exact_build,
        rollback_revocation: rollback,
        mirror_offline: mirror_offline(slug, true, spec.mirror != MirrorFreshness::NotApplicable),
        badges,
        surface_bindings,
        proof_packet: proof_packet(slug, spec.slo_state),
        waiver: None,
        owner_signoff: signed(spec.owner),
        active_gap_reasons: Vec::new(),
        published_label: spec.claim_label,
        publication_destinations: destinations(),
        rationale:
            "Every user-visible surface renders the same exact-build identity and provenance, every copy-safe badge is honest, the provenance verifies offline, the proof packet is within SLO, and the owner signed; the family converges its claimed label."
                .to_owned(),
    }
}

/// The managed-output card narrows: its release signature is pending, its
/// attestation is pending, its SBOM is partial, its symbols were stripped, its
/// mirror is stale, its evidence is incomplete, its provenance is not
/// offline-verifiable, and its export surface renders a different build identity.
/// Every badge stays honest (no overclaim), so the card narrows while still
/// exposing copy-safe truth.
fn managed_output_card() -> ProvenanceCard {
    let slug = "managed-output";
    let exact_build = exact_build(
        slug,
        SignatureStateClass::PendingReleaseSignature,
        AttestationAvailability::PendingAttestation,
        SbomScope::Partial,
        SymbolSourceMapAvailability::Stripped,
        MirrorFreshness::Stale,
        EvidenceCompleteness::Partial,
    );
    let rollback = rollback_posture(slug);
    let badges = canonical_badges(&exact_build, &rollback);

    // Five surfaces converge on the canonical identity; the export surface renders
    // a different build identity, so the surfaces diverge.
    let mut surface_bindings: Vec<SurfaceBinding> = ProvenanceSurfaceKind::ALL
        .iter()
        .filter(|&&surface| surface != ProvenanceSurfaceKind::Export)
        .map(|&surface| {
            surface_binding(
                surface,
                slug,
                exact_build.build_identity_ref.clone(),
                exact_build.provenance_ref.clone(),
                false,
            )
        })
        .collect();
    surface_bindings.push(SurfaceBinding {
        surface: ProvenanceSurfaceKind::Export,
        surface_ref: format!("surface/export/m5-{slug}"),
        build_identity_ref: format!("exact_build/m5-{slug}/stale-pointer"),
        provenance_ref: format!("provenance/m5-{slug}"),
        renders_badges: true,
        copyable: true,
        offline_available: false,
        summary: "The export surface renders a stale build identity that diverges from the card."
            .to_owned(),
    });

    ProvenanceCard {
        entry_id: format!("card-{slug}"),
        title: "Managed output provenance card".to_owned(),
        family_kind: M5ArtifactFamilyKind::ManagedOutput,
        artifact_ref: format!("artifact/m5/{slug}"),
        artifact_summary: "Managed outputs from managed/tenant-scoped lanes, pending re-sign."
            .to_owned(),
        release_blocking: true,
        claim_ref: format!("claim/m5-{slug}"),
        claim_label: StableClaimLevel::Beta,
        card_state: CardState::ProvenanceThin,
        exact_build,
        rollback_revocation: rollback,
        mirror_offline: mirror_offline(slug, false, true),
        badges,
        surface_bindings,
        proof_packet: proof_packet(slug, FreshnessSloState::Breached),
        waiver: None,
        owner_signoff: signed("managed-release"),
        active_gap_reasons: vec![
            CardGapReason::SignatureMissing,
            CardGapReason::AttestationMissing,
            CardGapReason::SbomIncomplete,
            CardGapReason::SymbolsMissing,
            CardGapReason::MirrorStale,
            CardGapReason::EvidenceIncomplete,
            CardGapReason::OfflineUnverifiable,
            CardGapReason::SurfaceDivergent,
            CardGapReason::ProofPacketStale,
        ],
        published_label: StableClaimLevel::Preview,
        publication_destinations: destinations(),
        rationale:
            "The managed-output family is pending its release signature and attestation, ships a partial SBOM with stripped symbols and a stale mirror, has incomplete evidence, cannot verify its provenance offline, and renders a divergent build identity on its export surface; its badges stay honest while it inherits its below-cutline beta claim and narrows to preview, naming every gap."
                .to_owned(),
    }
}

fn placeholder_summary() -> M5ProvenanceCardSummary {
    M5ProvenanceCardSummary {
        total_entries: 0,
        entries_converged: 0,
        entries_narrowed: 0,
        entries_on_active_waiver: 0,
        release_blocking_total: 0,
        release_blocking_converged: 0,
        release_blocking_narrowed: 0,
        notebook_pack_entries: 0,
        request_data_asset_entries: 0,
        profiler_replay_entries: 0,
        framework_template_entries: 0,
        docs_pack_entries: 0,
        model_pack_entries: 0,
        companion_offboarding_entries: 0,
        managed_output_entries: 0,
        signatures_verified: 0,
        attestations_available: 0,
        spdx_sbom_in_scope: 0,
        cyclonedx_exportable: 0,
        symbols_available: 0,
        mirror_current: 0,
        rollback_targets_recorded: 0,
        offline_verifiable: 0,
        surfaces_converged: 0,
        packets_current: 0,
        packets_due_for_refresh: 0,
        packets_breached: 0,
        packets_missing: 0,
        total_surface_bindings: 0,
        total_badges: 0,
        total_active_gap_reasons: 0,
        rules_firing: 0,
    }
}
