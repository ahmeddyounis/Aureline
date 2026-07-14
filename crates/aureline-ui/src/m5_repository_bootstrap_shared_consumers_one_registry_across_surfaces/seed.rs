//! Canonical seed for the repository-bootstrap shared-consumer parity packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV,
//! Markdown summary, and narrowed fixtures. Every binding is derived from one per-profile
//! [`RepositoryBootstrapStateFacetValues`] so the same acquisition profile always carries the same grammar
//! across surfaces, and every narrowed representation derives its disclosure from
//! [`resolve_repository_bootstrap_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

fn facets(
    repository_bootstrap_role: &str,
    family: &str,
    registry_reference: &str,
    entry_context: &str,
    surface_context: &str,
    trust_stage_continuity: &str,
) -> RepositoryBootstrapStateFacetValues {
    RepositoryBootstrapStateFacetValues {
        repository_bootstrap_role_word: repository_bootstrap_role.to_owned(),
        family_word: family.to_owned(),
        registry_reference_word: registry_reference.to_owned(),
        entry_context_word: entry_context.to_owned(),
        surface_context_word: surface_context.to_owned(),
        trust_stage_continuity_word: trust_stage_continuity.to_owned(),
    }
}

fn preserved_note_for(reason: RepositoryBootstrapNarrowReason) -> String {
    match reason {
        RepositoryBootstrapNarrowReason::CompactionNarrowed => {
            "repository-bootstrap-role, family, registry-reference, entry-context, surface-context, and trust-stage-continuity words preserved; only disclosure depth compacted"
        }
        RepositoryBootstrapNarrowReason::RemoteProjectionNarrowed => {
            "all repository-bootstrap grammar preserved; the family is projected from the remote source of truth"
        }
        RepositoryBootstrapNarrowReason::ExportRedactionNarrowed => {
            "all repository-bootstrap grammar preserved; only surrounding detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: RepositoryBootstrapNarrowNextAction) -> String {
    match action {
        RepositoryBootstrapNarrowNextAction::ExpandInDesktop => "Expand in the desktop surface",
        RepositoryBootstrapNarrowNextAction::OpenRemoteSource => "Open the remote source",
        RepositoryBootstrapNarrowNextAction::OpenFullDetail => "Open the full detail",
    }
    .to_owned()
}

fn binding_refs(family: M5RepositoryBootstrapFamily) -> Vec<String> {
    vec![
        M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF.to_owned(),
        family.canonical_domain_schema_ref().to_owned(),
    ]
}

fn make_binding(
    binding_id: &str,
    bootstrap_profile_id: &str,
    bootstrap_profile_label: &str,
    family: M5RepositoryBootstrapFamily,
    consumer: M5RepositoryBootstrapConsumerSurface,
    representation: RepositoryBootstrapRepresentation,
    state_facets: RepositoryBootstrapStateFacetValues,
) -> RepositoryBootstrapConsumerBinding {
    let disclosure = resolve_repository_bootstrap_render_disclosure(representation);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        RepositoryBootstrapNarrowNote {
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

    RepositoryBootstrapConsumerBinding {
        binding_id: binding_id.to_owned(),
        bootstrap_profile_id: bootstrap_profile_id.to_owned(),
        bootstrap_profile_label: bootstrap_profile_label.to_owned(),
        family,
        consumer,
        representation,
        state_facets,
        parity_state: disclosure.parity_state,
        narrow_note,
        remote_source_note,
        export_detail_note,
        rewrites_clone_into_open_when_local_checkout_already_exists: false,
        runs_repo_owned_actions_implicitly_during_acquisition: false,
        loses_signer_or_mirror_provenance_across_offline_or_mirrored_fetches: false,
        strands_partial_acquisition_without_resume_discard_or_readonly_choices: false,
        hides_bootstrap_credential_posture_behind_generic_connected_state_copy: false,
        source_contract_refs: binding_refs(family),
    }
}

/// One consumer-surface adoption of an acquisition profile, before any representation override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5RepositoryBootstrapConsumerSurface,
    representation: RepositoryBootstrapRepresentation,
}

/// One acquisition profile rendered across several consumer surfaces at one grammar.
struct ProfileSpec {
    profile_id: &'static str,
    profile_label: &'static str,
    family: M5RepositoryBootstrapFamily,
    facets: RepositoryBootstrapStateFacetValues,
    bindings: Vec<BindingSpec>,
}

fn spec(
    profile_id: &'static str,
    profile_label: &'static str,
    family: M5RepositoryBootstrapFamily,
    facets: RepositoryBootstrapStateFacetValues,
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
    consumer: M5RepositoryBootstrapConsumerSurface,
    representation: RepositoryBootstrapRepresentation,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        representation,
    }
}

/// The five acquisition profiles — one per B142 repository-bootstrap family — and the surfaces that adopt
/// each, drawn from the acquisition-engine, shell, workspace, git-service, trust-service, diagnostics,
/// docs / help, CLI / export, and support-export consumers.
fn profile_specs() -> Vec<ProfileSpec> {
    use M5RepositoryBootstrapConsumerSurface::*;
    use M5RepositoryBootstrapFamily::*;
    use RepositoryBootstrapRepresentation::*;

    let source_locator_registry = "source_locator_registry";
    let checkout_plan_registry = "checkout_plan_registry";
    let bootstrap_evidence_registry = "bootstrap_evidence_registry";
    let staged_continuity = "trust_staged_and_provenance_disclosed_before_bootstrap";
    let acquisition_scoped_descriptor = "acquisition_scoped_descriptor";

    vec![
        spec(
            "open-local/existing-checkout-not-recloned",
            "Open local (existing checkout detected, never recloned)",
            OpenLocal,
            facets(
                "source_locator",
                "open_local",
                source_locator_registry,
                "returning_workspace",
                "start_center_and_shell",
                acquisition_scoped_descriptor,
            ),
            vec![
                bs("rbsc-open-local-acquisition", AcquisitionEngine, DesktopFull),
                bs("rbsc-open-local-shell", ShellUi, DesktopFull),
                bs("rbsc-open-local-cli", CliExport, ExportedRedacted),
            ],
        ),
        spec(
            "clone-remote/credential-posture-before-network",
            "Clone remote (checkout plan and credential posture shown before the fetch)",
            CloneRemote,
            facets(
                "credential_posture",
                "clone_remote",
                checkout_plan_registry,
                "first_run",
                "os_open_and_workspace",
                staged_continuity,
            ),
            vec![
                bs("rbsc-clone-remote-git", GitService, DesktopFull),
                bs("rbsc-clone-remote-shell", ShellUi, DesktopFull),
                bs("rbsc-clone-remote-support", SupportExport, ExportedRedacted),
            ],
        ),
        spec(
            "open-archive/digest-verified-before-extraction",
            "Open archive (digest and extraction evidence verified before disk mutation)",
            OpenArchive,
            facets(
                "evidence_packet",
                "open_archive",
                source_locator_registry,
                "offline_or_air_gapped",
                "deep_link_and_import",
                staged_continuity,
            ),
            vec![
                bs("rbsc-open-archive-diagnostics", Diagnostics, DesktopFull),
                bs("rbsc-open-archive-acquisition", AcquisitionEngine, DesktopFull),
                bs("rbsc-open-archive-workspace", WorkspaceService, RemoteProjected),
            ],
        ),
        spec(
            "import-bundle/signer-provenance-staged-trust",
            "Import bundle (signer / mirror provenance preserved, trust staged before repo actions)",
            ImportBundle,
            facets(
                "staged_trust",
                "import_bundle",
                bootstrap_evidence_registry,
                "mirrored_registry",
                "cli_and_headless",
                staged_continuity,
            ),
            vec![
                bs("rbsc-import-bundle-trust", TrustService, DesktopFull),
                bs("rbsc-import-bundle-diagnostics", Diagnostics, DesktopFull),
                bs("rbsc-import-bundle-docs", DocsHelp, RemoteProjected),
            ],
        ),
        spec(
            "resume-snapshot/post-open-queue-never-auto-runs",
            "Resume snapshot (partial acquisition resumable / discardable, post-open queue never auto-runs)",
            ResumeSnapshot,
            facets(
                "post_open_queue",
                "resume_snapshot",
                bootstrap_evidence_registry,
                "resumed_after_interrupt",
                "docs_and_support",
                staged_continuity,
            ),
            vec![
                bs("rbsc-resume-snapshot-docs", DocsHelp, DesktopFull),
                bs("rbsc-resume-snapshot-workspace", WorkspaceService, CompactNarrowed),
                bs("rbsc-resume-snapshot-support", SupportExport, ExportedRedacted),
            ],
        ),
    ]
}

/// Builds all consumer bindings, applying `rep` to override a binding's representation.
fn build_bindings<F>(rep: F) -> Vec<RepositoryBootstrapConsumerBinding>
where
    F: Fn(&str, RepositoryBootstrapRepresentation) -> RepositoryBootstrapRepresentation,
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

fn trust_review() -> RepositoryBootstrapSharedConsumersTrustReview {
    RepositoryBootstrapSharedConsumersTrustReview {
        family_reuse_proven_by_fixtures: true,
        same_profile_same_repository_bootstrap_across_surfaces: true,
        repository_bootstrap_role_words_stay_in_frozen_vocabulary: true,
        trust_roles_never_run_repo_actions_or_lose_provenance: true,
        acquisition_never_rewrites_clone_into_open_over_existing_checkout: true,
        acquisition_never_runs_repo_owned_actions_implicitly: true,
        acquisition_never_loses_signer_or_mirror_provenance: true,
        partial_acquisition_never_stranded_without_resume_discard_or_readonly: true,
        bootstrap_credential_posture_never_hidden_behind_generic_copy: true,
        narrowing_disclosed_across_representations: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> RepositoryBootstrapSharedConsumersProjection {
    RepositoryBootstrapSharedConsumersProjection {
        acquisition_engine_consumes_shared_repository_bootstrap: true,
        shell_ui_consumes_shared_repository_bootstrap: true,
        workspace_service_consumes_shared_repository_bootstrap: true,
        git_service_consumes_shared_repository_bootstrap: true,
        trust_service_consumes_shared_repository_bootstrap: true,
        diagnostics_consumes_shared_repository_bootstrap: true,
        docs_help_consumes_shared_repository_bootstrap: true,
        cli_export_consumes_shared_repository_bootstrap: true,
        support_export_consumes_shared_repository_bootstrap: true,
        every_family_adopted_by_two_or_more_consumers: true,
        repository_bootstrap_identical_for_same_profile: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_repository_bootstrap_family: true,
    }
}

fn proof_freshness() -> RepositoryBootstrapSharedConsumersProofFreshness {
    RepositoryBootstrapSharedConsumersProofFreshness {
        proof_freshness_slo_hours: M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_SCHEMA_REF.to_owned(),
        M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_DOC_REF.to_owned(),
        M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF.to_owned(),
        M5_REPOSITORY_BOOTSTRAP_MATRIX_DOC_REF.to_owned(),
    ];
    // The five families map to three canonical domain schemas; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for family in M5RepositoryBootstrapFamily::ALL {
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
    consumer_bindings: Vec<RepositoryBootstrapConsumerBinding>,
) -> M5RepositoryBootstrapSharedConsumersPacket {
    M5RepositoryBootstrapSharedConsumersPacket::new(
        M5RepositoryBootstrapSharedConsumersPacketInput {
            packet_id: packet_id.to_owned(),
            surface_label: surface_label.to_owned(),
            consumer_bindings,
            downgrade_triggers: RepositoryBootstrapSharedConsumersDowngradeTrigger::ALL.to_vec(),
            consumer_surfaces: M5RepositoryBootstrapConsumerSurface::ALL.to_vec(),
            trust_review: trust_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// The canonical, checked-in repository-bootstrap shared-consumer parity packet.
pub fn seeded_m5_repository_bootstrap_shared_consumers(
) -> M5RepositoryBootstrapSharedConsumersPacket {
    packet_from_bindings(
        M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_PACKET_ID,
        "M5 repository-bootstrap shared consumers (one registry across surfaces)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same profiles with two more desktop surfaces narrowed to compact and remote
/// representations, proving grammar survives compact and remote forms.
pub fn seeded_m5_repository_bootstrap_shared_consumers_compact_remote_narrowed(
) -> M5RepositoryBootstrapSharedConsumersPacket {
    packet_from_bindings(
        "m5-repository-bootstrap-shared-consumers:compact-remote:0001",
        "M5 repository-bootstrap shared consumers (compact / remote narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "rbsc-open-local-shell" => RepositoryBootstrapRepresentation::CompactNarrowed,
            "rbsc-clone-remote-git" => RepositoryBootstrapRepresentation::RemoteProjected,
            _ => default,
        }),
    )
}

/// Fixture: the same profiles with two more surfaces narrowed to exported, export-safe
/// representations, proving grammar survives into exported forms.
pub fn seeded_m5_repository_bootstrap_shared_consumers_exported_redaction_narrowed(
) -> M5RepositoryBootstrapSharedConsumersPacket {
    packet_from_bindings(
        "m5-repository-bootstrap-shared-consumers:exported-redaction:0001",
        "M5 repository-bootstrap shared consumers (exported redaction narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "rbsc-open-archive-diagnostics" => RepositoryBootstrapRepresentation::ExportedRedacted,
            "rbsc-import-bundle-trust" => RepositoryBootstrapRepresentation::ExportedRedacted,
            _ => default,
        }),
    )
}
