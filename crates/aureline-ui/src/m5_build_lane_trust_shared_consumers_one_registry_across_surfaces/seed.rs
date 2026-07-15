//! Canonical seed for the build-lane-trust shared-consumer parity packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV,
//! Markdown summary, and narrowed fixtures. Every binding is derived from one per-profile
//! [`BuildLaneTrustStateFacetValues`] so the same build profile always carries the same grammar across
//! surfaces, and every narrowed representation derives its disclosure from
//! [`resolve_build_lane_trust_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

fn facets(
    build_lane_trust_role: &str,
    family: &str,
    registry_reference: &str,
    build_context: &str,
    surface_context: &str,
    replay_continuity: &str,
) -> BuildLaneTrustStateFacetValues {
    BuildLaneTrustStateFacetValues {
        build_lane_trust_role_word: build_lane_trust_role.to_owned(),
        family_word: family.to_owned(),
        registry_reference_word: registry_reference.to_owned(),
        build_context_word: build_context.to_owned(),
        surface_context_word: surface_context.to_owned(),
        replay_continuity_word: replay_continuity.to_owned(),
    }
}

fn preserved_note_for(reason: BuildLaneTrustNarrowReason) -> String {
    match reason {
        BuildLaneTrustNarrowReason::CompactionNarrowed => {
            "build-lane-trust-role, family, registry-reference, build-context, surface-context, and replay-continuity words preserved; only disclosure depth compacted"
        }
        BuildLaneTrustNarrowReason::RemoteProjectionNarrowed => {
            "all build-lane-trust grammar preserved; the family is projected from the remote source of truth"
        }
        BuildLaneTrustNarrowReason::ExportRedactionNarrowed => {
            "all build-lane-trust grammar preserved; only surrounding detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: BuildLaneTrustNarrowNextAction) -> String {
    match action {
        BuildLaneTrustNarrowNextAction::ExpandInDesktop => "Expand in the desktop surface",
        BuildLaneTrustNarrowNextAction::OpenRemoteSource => "Open the remote source",
        BuildLaneTrustNarrowNextAction::OpenFullDetail => "Open the full detail",
    }
    .to_owned()
}

fn binding_refs(family: M5BuildLaneFamily) -> Vec<String> {
    vec![
        M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_REF.to_owned(),
        family.canonical_domain_schema_ref().to_owned(),
    ]
}

fn make_binding(
    binding_id: &str,
    build_profile_id: &str,
    build_profile_label: &str,
    family: M5BuildLaneFamily,
    consumer: M5BuildLaneConsumerSurface,
    representation: BuildLaneTrustRepresentation,
    state_facets: BuildLaneTrustStateFacetValues,
) -> BuildLaneTrustConsumerBinding {
    let disclosure = resolve_build_lane_trust_render_disclosure(representation);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        BuildLaneTrustNarrowNote {
            reason,
            preserved_grammar_note: preserved_note_for(reason),
            next_action,
            next_action_label: next_action_label_for(next_action),
        }
    });
    let remote_source_note = if disclosure.needs_remote_source_note {
        "projected from the remote source of truth; the source stays remote".to_owned()
    } else {
        String::new()
    };
    let export_detail_note = if disclosure.needs_export_detail_note {
        "surrounding detail redacted export-safe in this packet; full detail available on request"
            .to_owned()
    } else {
        String::new()
    };

    BuildLaneTrustConsumerBinding {
        binding_id: binding_id.to_owned(),
        build_profile_id: build_profile_id.to_owned(),
        build_profile_label: build_profile_label.to_owned(),
        family,
        consumer,
        representation,
        state_facets,
        parity_state: disclosure.parity_state,
        narrow_note,
        remote_source_note,
        export_detail_note,
        pr_caches_publish_release_artifacts: false,
        treats_remote_cache_hits_as_reproducibility_proof: false,
        lets_docs_schema_sbom_or_symbol_sidecars_drift_from_binary_build_identity: false,
        overclaims_clean_room_parity_when_only_partial_artifact_classes_were_rebuilt: false,
        hides_non_hermetic_inputs_cache_poisoning_or_unreplayable_artifacts_behind_green_publication_rows:
            false,
        source_contract_refs: binding_refs(family),
    }
}

/// One consumer-surface adoption of a build profile, before any representation override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5BuildLaneConsumerSurface,
    representation: BuildLaneTrustRepresentation,
}

/// One build profile rendered across several consumer surfaces at one grammar.
struct ProfileSpec {
    profile_id: &'static str,
    profile_label: &'static str,
    family: M5BuildLaneFamily,
    facets: BuildLaneTrustStateFacetValues,
    bindings: Vec<BindingSpec>,
}

fn spec(
    profile_id: &'static str,
    profile_label: &'static str,
    family: M5BuildLaneFamily,
    facets: BuildLaneTrustStateFacetValues,
    bindings: Vec<BindingSpec>,
) -> ProfileSpec {
    ProfileSpec {
        profile_id,
        profile_label,
        family,
        facets,
        bindings,
    }
}

fn bs(
    binding_id: &'static str,
    consumer: M5BuildLaneConsumerSurface,
    representation: BuildLaneTrustRepresentation,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        representation,
    }
}

/// The four build profiles — one per B144 build-lane-trust family — and the surfaces that adopt each, drawn
/// from the build-farm, cache-service, release-center, shiproom, provenance-service, diagnostics, docs / help,
/// CLI / export, and support-export consumers that back the About / provenance, Help, service-health,
/// release-center, and support-export surfaces.
fn profile_specs() -> Vec<ProfileSpec> {
    use BuildLaneTrustRepresentation::*;
    use M5BuildLaneConsumerSurface::*;
    use M5BuildLaneFamily::*;

    let build_lane_descriptor_registry = "build_lane_descriptor_registry";
    let reproducibility_proof_registry = "reproducibility_proof_registry";
    let replay_continuity = "replay_proven_and_build_identity_converged_before_promotion";
    let build_lane_scoped_descriptor = "build_lane_scoped_descriptor";

    vec![
        spec(
            "contributor-pr/reads-caches-never-publishes",
            "Contributor / PR lane (reads shared caches, never publishes release artifacts)",
            ContributorPr,
            facets(
                "cache_posture",
                "contributor_pr",
                build_lane_descriptor_registry,
                "continuous_integration",
                "build_farm_and_cache_service",
                replay_continuity,
            ),
            vec![
                bs("bltsc-contributor-pr-build-farm", BuildFarm, DesktopFull),
                bs("bltsc-contributor-pr-cache", CacheService, DesktopFull),
                bs("bltsc-contributor-pr-cli", CliExport, ExportedRedacted),
            ],
        ),
        spec(
            "protected-merge/controlled-credentials-verified-caches",
            "Protected-merge lane (controlled credentials and verified caches only)",
            ProtectedMerge,
            facets(
                "publication_authority",
                "protected_merge",
                build_lane_descriptor_registry,
                "protected_release_channel",
                "release_center_and_shiproom",
                replay_continuity,
            ),
            vec![
                bs(
                    "bltsc-protected-merge-release-center",
                    ReleaseCenter,
                    DesktopFull,
                ),
                bs("bltsc-protected-merge-shiproom", Shiproom, DesktopFull),
                bs(
                    "bltsc-protected-merge-diagnostics",
                    Diagnostics,
                    RemoteProjected,
                ),
            ],
        ),
        spec(
            "release/verified-inputs-one-exact-build-identity",
            "Release lane (verified or re-materialized inputs converging on one exact build identity)",
            Release,
            facets(
                "reproducibility_proof",
                "release",
                reproducibility_proof_registry,
                "offline_or_air_gapped_mirror",
                "provenance_and_diagnostics",
                replay_continuity,
            ),
            vec![
                bs("bltsc-release-provenance", ProvenanceService, DesktopFull),
                bs("bltsc-release-diagnostics", Diagnostics, DesktopFull),
                bs("bltsc-release-support", SupportExport, ExportedRedacted),
            ],
        ),
        spec(
            "emergency-hotfix/expedited-verified-one-build-identity",
            "Emergency-hotfix lane (expedited yet verified inputs, one exact build identity for support)",
            EmergencyHotfix,
            facets(
                "support_identity",
                "emergency_hotfix",
                reproducibility_proof_registry,
                "emergency_hotfix_channel",
                "docs_help_and_support",
                build_lane_scoped_descriptor,
            ),
            vec![
                bs("bltsc-emergency-hotfix-docs", DocsHelp, DesktopFull),
                bs(
                    "bltsc-emergency-hotfix-release-center",
                    ReleaseCenter,
                    CompactNarrowed,
                ),
                bs(
                    "bltsc-emergency-hotfix-support",
                    SupportExport,
                    ExportedRedacted,
                ),
            ],
        ),
    ]
}

/// Builds all consumer bindings, applying `rep` to override a binding's representation.
fn build_bindings<F>(rep: F) -> Vec<BuildLaneTrustConsumerBinding>
where
    F: Fn(&str, BuildLaneTrustRepresentation) -> BuildLaneTrustRepresentation,
{
    let mut bindings = Vec::new();
    for profile in profile_specs() {
        for spec in &profile.bindings {
            let representation = rep(spec.binding_id, spec.representation);
            bindings.push(make_binding(
                spec.binding_id,
                profile.profile_id,
                profile.profile_label,
                profile.family,
                spec.consumer,
                representation,
                profile.facets.clone(),
            ));
        }
    }
    bindings
}

fn trust_review() -> BuildLaneTrustSharedConsumersTrustReview {
    BuildLaneTrustSharedConsumersTrustReview {
        family_reuse_proven_by_fixtures: true,
        same_profile_same_build_lane_trust_across_surfaces: true,
        build_lane_trust_role_words_stay_in_frozen_vocabulary: true,
        trust_roles_never_publish_untrusted_or_treat_cache_as_proof: true,
        pr_cache_never_publishes_release_artifacts: true,
        remote_cache_hit_never_treated_as_reproducibility_proof: true,
        sidecars_never_drift_from_binary_build_identity: true,
        clean_room_parity_never_overclaimed_on_partial_rebuild: true,
        non_hermetic_inputs_cache_poisoning_or_unreplayable_artifacts_never_hidden: true,
        narrowing_disclosed_across_representations: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> BuildLaneTrustSharedConsumersProjection {
    BuildLaneTrustSharedConsumersProjection {
        build_farm_consumes_shared_build_lane_trust: true,
        cache_service_consumes_shared_build_lane_trust: true,
        release_center_consumes_shared_build_lane_trust: true,
        shiproom_consumes_shared_build_lane_trust: true,
        provenance_service_consumes_shared_build_lane_trust: true,
        diagnostics_consumes_shared_build_lane_trust: true,
        docs_help_consumes_shared_build_lane_trust: true,
        cli_export_consumes_shared_build_lane_trust: true,
        support_export_consumes_shared_build_lane_trust: true,
        every_family_adopted_by_two_or_more_consumers: true,
        build_lane_trust_identical_for_same_profile: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_build_lane_family: true,
    }
}

fn proof_freshness() -> BuildLaneTrustSharedConsumersProofFreshness {
    BuildLaneTrustSharedConsumersProofFreshness {
        proof_freshness_slo_hours: M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_SCHEMA_REF.to_owned(),
        M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_DOC_REF.to_owned(),
        M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_REF.to_owned(),
        M5_BUILD_LANE_TRUST_MATRIX_DOC_REF.to_owned(),
    ];
    // The four families map to two canonical domain schemas; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for family in M5BuildLaneFamily::ALL {
        domains.insert(family.canonical_domain_schema_ref());
    }
    for domain in domains {
        refs.push(domain.to_owned());
    }
    refs
}

fn packet_from_bindings(
    packet_id: &str,
    surface_label: &str,
    consumer_bindings: Vec<BuildLaneTrustConsumerBinding>,
) -> M5BuildLaneTrustSharedConsumersPacket {
    M5BuildLaneTrustSharedConsumersPacket::new(M5BuildLaneTrustSharedConsumersPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        consumer_bindings,
        downgrade_triggers: BuildLaneTrustSharedConsumersDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5BuildLaneConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in build-lane-trust shared-consumer parity packet.
pub fn seeded_m5_build_lane_trust_shared_consumers() -> M5BuildLaneTrustSharedConsumersPacket {
    packet_from_bindings(
        M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_PACKET_ID,
        "M5 build-lane-trust shared consumers (one registry across surfaces)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same profiles with two more desktop surfaces narrowed to compact and remote
/// representations, proving grammar survives compact and remote forms.
pub fn seeded_m5_build_lane_trust_shared_consumers_compact_remote_narrowed(
) -> M5BuildLaneTrustSharedConsumersPacket {
    packet_from_bindings(
        "m5-build-lane-trust-shared-consumers:compact-remote:0001",
        "M5 build-lane-trust shared consumers (compact / remote narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "bltsc-contributor-pr-cache" => BuildLaneTrustRepresentation::CompactNarrowed,
            "bltsc-protected-merge-release-center" => BuildLaneTrustRepresentation::RemoteProjected,
            _ => default,
        }),
    )
}

/// Fixture: the same profiles with two more surfaces narrowed to exported, export-safe
/// representations, proving grammar survives into exported forms.
pub fn seeded_m5_build_lane_trust_shared_consumers_exported_redaction_narrowed(
) -> M5BuildLaneTrustSharedConsumersPacket {
    packet_from_bindings(
        "m5-build-lane-trust-shared-consumers:exported-redaction:0001",
        "M5 build-lane-trust shared consumers (exported redaction narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "bltsc-release-diagnostics" => BuildLaneTrustRepresentation::ExportedRedacted,
            "bltsc-protected-merge-shiproom" => BuildLaneTrustRepresentation::ExportedRedacted,
            _ => default,
        }),
    )
}
