//! Canonical seed builders for the boundary-wording catalog.
//!
//! These builders are the single producer of the checked-in support export and the
//! localized / offline-mirror fixtures. The headless emitter and the inline tests both
//! call them so the in-code catalog, the artifact, and the fixtures never drift.
//!
//! Entries that share a [`BoundaryWordingEntry::concept_id`] are built from one
//! per-concept helper, so cross-surface copy parity holds by construction: the term,
//! support metadata, implication postures, local-capability posture, and disclosed
//! alternatives are identical across every surface that renders the concept, and only
//! the surface, claim act, and human prose differ.

use super::*;

/// Stable catalog id for the canonical boundary-wording catalog.
pub const BOUNDARY_WORDING_CATALOG_ID: &str = "m5-boundary-wording-catalog:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

// Compatibility/support metadata refs that anchor available alternatives. These name
// the support metadata a surface points at instead of prose-only marketing.
const REF_LOCAL_ONLY: &str = "support.metadata.local_only_path";
const REF_BYOK: &str = "support.metadata.byok_path";
const REF_SELF_HOSTED: &str = "support.metadata.self_hosted_path";
const REF_EXPORT: &str = "support.metadata.export_path";
const REF_ROLLBACK: &str = "support.metadata.rollback_path";

use ActualBoundaryPosture as Ap;
use AlternativePath as Alt;
use BoundaryClaimKind as Ck;
use BoundaryImplication as Im;
use BoundarySurface as Sf;
use BoundaryTerm as Bt;
use ImplicationPosture as Po;

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn imp(
    dimension: BoundaryImplication,
    posture: ImplicationPosture,
    disclosure: &str,
) -> ImplicationStatement {
    ImplicationStatement {
        dimension,
        posture,
        disclosure: disclosure.to_owned(),
    }
}

fn alt(
    path: AlternativePath,
    available: bool,
    disclosure: &str,
    reference_ref: Option<&str>,
) -> AlternativePathDisclosure {
    AlternativePathDisclosure {
        path,
        available,
        disclosure: disclosure.to_owned(),
        reference_ref: reference_ref.map(str::to_owned),
    }
}

#[allow(clippy::too_many_arguments)]
fn entry(
    entry_id: &str,
    concept_id: &str,
    term: BoundaryTerm,
    surface: BoundarySurface,
    canonical_text: &str,
    claim_kind: BoundaryClaimKind,
    support_metadata_ref: Option<&str>,
    actual_boundary_posture: ActualBoundaryPosture,
    introduces_managed_or_paid: bool,
    core_workflow_remains_local: bool,
    implications: Vec<ImplicationStatement>,
    alternative_paths: Vec<AlternativePathDisclosure>,
    source_ref: &str,
) -> BoundaryWordingEntry {
    BoundaryWordingEntry {
        entry_id: entry_id.to_owned(),
        concept_id: concept_id.to_owned(),
        term,
        surface,
        canonical_text: canonical_text.to_owned(),
        claim_kind,
        support_metadata_ref: support_metadata_ref.map(str::to_owned),
        actual_boundary_posture,
        introduces_managed_or_paid,
        core_workflow_remains_local,
        implications,
        alternative_paths,
        source_ref: source_ref.to_owned(),
    }
}

// --- concept.cloud_sync: a managed-optional capability on four surfaces -------------

const CLOUD_SYNC_CONCEPT: &str = "concept.cloud_sync";
const CLOUD_SYNC_SUPPORT: &str = "support.metadata.cloud_sync_compatibility";

fn cloud_sync_implications() -> Vec<ImplicationStatement> {
    vec![
        imp(
            Im::Identity,
            Po::Required,
            "Requires sign-in to the managed workspace to sync.",
        ),
        imp(
            Im::Network,
            Po::Required,
            "Requires network access to the managed sync service.",
        ),
        imp(
            Im::Data,
            Po::Optional,
            "You choose which workspace data syncs; nothing leaves the device without opt-in.",
        ),
        imp(
            Im::Export,
            Po::Retained,
            "A full local export of synced data remains available at any time.",
        ),
        imp(
            Im::Rollback,
            Po::Retained,
            "You can roll back to the local-only configuration and keep editing.",
        ),
    ]
}

fn cloud_sync_alternatives() -> Vec<AlternativePathDisclosure> {
    vec![
        alt(
            Alt::LocalOnly,
            true,
            "Keep working fully local without sync.",
            Some(REF_LOCAL_ONLY),
        ),
        alt(
            Alt::Byok,
            true,
            "Use your own storage credentials instead of managed storage.",
            Some(REF_BYOK),
        ),
        alt(
            Alt::SelfHosted,
            true,
            "Point sync at a self-hosted endpoint you operate.",
            Some(REF_SELF_HOSTED),
        ),
        alt(
            Alt::Export,
            true,
            "Export everything locally before or after enabling sync.",
            Some(REF_EXPORT),
        ),
        alt(
            Alt::Rollback,
            true,
            "Disable sync and revert to the local-only configuration.",
            Some(REF_ROLLBACK),
        ),
    ]
}

fn cloud_sync_entry(
    entry_id: &str,
    surface: BoundarySurface,
    canonical_text: &str,
    claim_kind: BoundaryClaimKind,
    source_ref: &str,
) -> BoundaryWordingEntry {
    entry(
        entry_id,
        CLOUD_SYNC_CONCEPT,
        Bt::Managed,
        surface,
        canonical_text,
        claim_kind,
        Some(CLOUD_SYNC_SUPPORT),
        Ap::ManagedOptional,
        true,
        true,
        cloud_sync_implications(),
        cloud_sync_alternatives(),
        source_ref,
    )
}

// --- concept.premium_models: a commercial-paid capability on three surfaces ----------

const PREMIUM_MODELS_CONCEPT: &str = "concept.premium_models";
const PREMIUM_MODELS_SUPPORT: &str = "support.metadata.premium_models_compatibility";

fn premium_models_implications() -> Vec<ImplicationStatement> {
    vec![
        imp(
            Im::Identity,
            Po::Required,
            "Requires a paid account to call the premium hosted models.",
        ),
        imp(
            Im::Network,
            Po::Required,
            "Requires network access to the hosted model provider.",
        ),
        imp(
            Im::Data,
            Po::Optional,
            "Only prompts you send to a premium model leave the device; local models stay local.",
        ),
        imp(
            Im::Export,
            Po::Retained,
            "Conversations and outputs remain exportable locally.",
        ),
        imp(
            Im::Rollback,
            Po::Retained,
            "You can switch back to local or BYOK models without losing work.",
        ),
    ]
}

fn premium_models_alternatives() -> Vec<AlternativePathDisclosure> {
    vec![
        alt(
            Alt::LocalOnly,
            true,
            "Run local models with no account.",
            Some(REF_LOCAL_ONLY),
        ),
        alt(
            Alt::Byok,
            true,
            "Bring your own provider key instead of a premium plan.",
            Some(REF_BYOK),
        ),
        alt(
            Alt::SelfHosted,
            true,
            "Point at a self-hosted inference endpoint.",
            Some(REF_SELF_HOSTED),
        ),
        alt(
            Alt::Export,
            true,
            "Export conversations and outputs locally.",
            Some(REF_EXPORT),
        ),
        alt(
            Alt::Rollback,
            true,
            "Revert to local or BYOK models at any time.",
            Some(REF_ROLLBACK),
        ),
    ]
}

fn premium_models_entry(
    entry_id: &str,
    surface: BoundarySurface,
    canonical_text: &str,
    claim_kind: BoundaryClaimKind,
    source_ref: &str,
) -> BoundaryWordingEntry {
    entry(
        entry_id,
        PREMIUM_MODELS_CONCEPT,
        Bt::Premium,
        surface,
        canonical_text,
        claim_kind,
        Some(PREMIUM_MODELS_SUPPORT),
        Ap::CommercialPaid,
        true,
        true,
        premium_models_implications(),
        premium_models_alternatives(),
        source_ref,
    )
}

// --- concept.local_indexing: a local-only capability on two surfaces -----------------

const LOCAL_INDEXING_CONCEPT: &str = "concept.local_indexing";

fn local_indexing_implications() -> Vec<ImplicationStatement> {
    vec![
        imp(
            Im::Identity,
            Po::NotRequired,
            "No account or sign-in is required.",
        ),
        imp(
            Im::Network,
            Po::NotRequired,
            "Indexing runs fully offline; no network is used.",
        ),
        imp(
            Im::Data,
            Po::LocalOnly,
            "The index is built and stored entirely on this device.",
        ),
        imp(
            Im::Export,
            Po::Retained,
            "The index and its sources remain exportable locally.",
        ),
        imp(
            Im::Rollback,
            Po::Retained,
            "You can clear or rebuild the local index at any time.",
        ),
    ]
}

fn local_indexing_alternatives() -> Vec<AlternativePathDisclosure> {
    vec![
        alt(
            Alt::LocalOnly,
            true,
            "This is the local-only path; nothing else is required.",
            Some(REF_LOCAL_ONLY),
        ),
        alt(
            Alt::Export,
            true,
            "Export the index and its sources locally.",
            Some(REF_EXPORT),
        ),
        alt(
            Alt::Rollback,
            true,
            "Clear or rebuild the local index at any time.",
            Some(REF_ROLLBACK),
        ),
    ]
}

fn local_indexing_entry(
    entry_id: &str,
    surface: BoundarySurface,
    canonical_text: &str,
    source_ref: &str,
) -> BoundaryWordingEntry {
    entry(
        entry_id,
        LOCAL_INDEXING_CONCEPT,
        Bt::LocalOnly,
        surface,
        canonical_text,
        Ck::StatesBoundary,
        None,
        Ap::LocalIndependent,
        false,
        true,
        local_indexing_implications(),
        local_indexing_alternatives(),
        source_ref,
    )
}

fn entries() -> Vec<BoundaryWordingEntry> {
    vec![
        // concept.cloud_sync across settings, onboarding, account/upgrade, help/About.
        cloud_sync_entry(
            "entry.cloud_sync.settings",
            Sf::Settings,
            "Managed cloud sync keeps the workspaces you choose in sync across devices.",
            Ck::StatesBoundary,
            "glossary.term.managed_cloud_sync",
        ),
        cloud_sync_entry(
            "entry.cloud_sync.onboarding",
            Sf::Onboarding,
            "Turn on managed cloud sync now, or stay fully local and decide later.",
            Ck::StatesBoundary,
            "glossary.term.managed_cloud_sync",
        ),
        cloud_sync_entry(
            "entry.cloud_sync.account_upgrade",
            Sf::AccountUpgradePrompt,
            "Add managed cloud sync to this account, or keep your local-only setup.",
            Ck::WidensBoundary,
            "glossary.term.managed_cloud_sync",
        ),
        cloud_sync_entry(
            "entry.cloud_sync.help_about",
            Sf::HelpAbout,
            "Managed cloud sync is optional; local-only editing is always available.",
            Ck::StatesBoundary,
            "glossary.term.managed_cloud_sync",
        ),
        // concept.premium_models across marketplace, account/upgrade, help/About.
        premium_models_entry(
            "entry.premium_models.marketplace",
            Sf::Marketplace,
            "Premium hosted models are a paid add-on; local and BYOK models stay free.",
            Ck::StatesBoundary,
            "glossary.term.premium_models",
        ),
        premium_models_entry(
            "entry.premium_models.account_upgrade",
            Sf::AccountUpgradePrompt,
            "Upgrade for premium hosted models, or keep using local and BYOK models.",
            Ck::WidensBoundary,
            "glossary.term.premium_models",
        ),
        premium_models_entry(
            "entry.premium_models.help_about",
            Sf::HelpAbout,
            "Premium models require a paid plan; the editor works fully with local models.",
            Ck::StatesBoundary,
            "glossary.term.premium_models",
        ),
        // concept.local_indexing across settings and help/About.
        local_indexing_entry(
            "entry.local_indexing.settings",
            Sf::Settings,
            "Local-only indexing builds your code index on this device with no account.",
            "glossary.term.local_indexing",
        ),
        local_indexing_entry(
            "entry.local_indexing.help_about",
            Sf::HelpAbout,
            "Indexing is local only: nothing is uploaded and no sign-in is required.",
            "glossary.term.local_indexing",
        ),
        // concept.self_hosted_runner — a self-hostable capability widened on settings.
        entry(
            "entry.self_hosted_runner.settings",
            "concept.self_hosted_runner",
            Bt::SelfHosted,
            Sf::Settings,
            "Self-hosted runners let you run builds on infrastructure you operate.",
            Ck::WidensBoundary,
            Some("support.metadata.self_hosted_runner_compatibility"),
            Ap::SelfHostable,
            false,
            true,
            vec![
                imp(Im::Identity, Po::Optional, "Sign-in is optional; the runner authenticates to your own infrastructure."),
                imp(Im::Network, Po::Required, "Requires network access to the self-hosted runner you operate."),
                imp(Im::Data, Po::Optional, "Build data stays on your infrastructure; you control what is shared."),
                imp(Im::Export, Po::Retained, "Build logs and artifacts remain exportable locally."),
                imp(Im::Rollback, Po::Retained, "You can revert to the local build runner at any time."),
            ],
            vec![
                alt(Alt::SelfHosted, true, "This is the self-hosted path; you operate the runner.", Some(REF_SELF_HOSTED)),
                alt(Alt::Export, true, "Export build logs and artifacts locally.", Some(REF_EXPORT)),
                alt(Alt::Rollback, true, "Revert to the local build runner.", Some(REF_ROLLBACK)),
            ],
            "glossary.term.self_hosted_runner",
        ),
        // concept.byok_provider — a bring-your-own-key capability on settings.
        entry(
            "entry.byok_provider.settings",
            "concept.byok_provider",
            Bt::Byok,
            Sf::Settings,
            "Bring your own provider key to use your own model account directly.",
            Ck::StatesBoundary,
            None,
            Ap::Byok,
            false,
            true,
            vec![
                imp(Im::Identity, Po::NotRequired, "No Aureline account is required; you supply the provider key."),
                imp(Im::Network, Po::Required, "Requires network access to the provider you bring a key for."),
                imp(Im::Data, Po::Optional, "Only prompts you send to the provider leave the device."),
                imp(Im::Export, Po::Retained, "Conversations remain exportable locally."),
                imp(Im::Rollback, Po::Retained, "You can remove the key and fall back to local models."),
            ],
            vec![
                alt(Alt::Byok, true, "This is the BYOK path; you bring your own key.", Some(REF_BYOK)),
                alt(Alt::Export, true, "Export conversations locally.", Some(REF_EXPORT)),
                alt(Alt::Rollback, true, "Remove the key and use local models.", Some(REF_ROLLBACK)),
            ],
            "glossary.term.byok_provider",
        ),
        // concept.hosted_build_farm — a managed-required capability widened on marketplace.
        entry(
            "entry.hosted_build_farm.marketplace",
            "concept.hosted_build_farm",
            Bt::Hosted,
            Sf::Marketplace,
            "The hosted build farm runs heavy builds on managed infrastructure.",
            Ck::WidensBoundary,
            Some("support.metadata.hosted_build_farm_compatibility"),
            Ap::ManagedRequired,
            true,
            true,
            vec![
                imp(Im::Identity, Po::Required, "Requires a signed-in account to dispatch hosted builds."),
                imp(Im::Network, Po::Required, "Requires network access to the hosted build farm."),
                imp(Im::Data, Po::Optional, "Only the build inputs you dispatch leave the device."),
                imp(Im::Export, Po::Retained, "Build outputs remain exportable locally."),
                imp(Im::Rollback, Po::Retained, "Local and self-hosted builds remain available as a fallback."),
            ],
            vec![
                alt(Alt::LocalOnly, true, "Run the build locally with no managed dependency.", Some(REF_LOCAL_ONLY)),
                alt(Alt::SelfHosted, true, "Dispatch to a self-hosted runner instead.", Some(REF_SELF_HOSTED)),
                alt(Alt::Export, true, "Export build outputs locally.", Some(REF_EXPORT)),
                alt(Alt::Rollback, true, "Fall back to the local build runner.", Some(REF_ROLLBACK)),
            ],
            "glossary.term.hosted_build_farm",
        ),
        // concept.managed_policy_pack — a managed-required capability narrowed on release notes.
        entry(
            "entry.managed_policy_pack.release_notes",
            "concept.managed_policy_pack",
            Bt::Managed,
            Sf::ReleaseNotes,
            "Org policy packs now require a managed workspace; self-hosted policy packs stay supported.",
            Ck::NarrowsBoundary,
            Some("support.metadata.managed_policy_pack_compatibility"),
            Ap::ManagedRequired,
            true,
            true,
            vec![
                imp(Im::Identity, Po::Required, "Org policy packs require a managed workspace identity."),
                imp(Im::Network, Po::Required, "Requires network access to the managed policy service."),
                imp(Im::Data, Po::Optional, "Only policy decisions, not source, are evaluated remotely."),
                imp(Im::Export, Po::Retained, "Policy packs and decisions remain exportable locally."),
                imp(Im::Rollback, Po::Retained, "You can revert to a self-hosted or local policy pack."),
            ],
            vec![
                alt(Alt::SelfHosted, true, "Run the policy pack on a self-hosted workspace.", Some(REF_SELF_HOSTED)),
                alt(Alt::Export, true, "Export policy packs and decisions locally.", Some(REF_EXPORT)),
                alt(Alt::Rollback, true, "Revert to a self-hosted or local policy pack.", Some(REF_ROLLBACK)),
                alt(Alt::LocalOnly, false, "A fully local org policy pack is not offered for this capability.", None),
            ],
            "glossary.term.managed_policy_pack",
        ),
        // concept.trial_window — a commercial-paid capability stated on account/upgrade.
        entry(
            "entry.trial_window.account_upgrade",
            "concept.trial_window",
            Bt::Trial,
            Sf::AccountUpgradePrompt,
            "Your premium trial is time-limited; local and BYOK paths continue after it ends.",
            Ck::StatesBoundary,
            None,
            Ap::CommercialPaid,
            true,
            true,
            vec![
                imp(Im::Identity, Po::Required, "The trial is tied to a signed-in account."),
                imp(Im::Network, Po::Required, "Trial features use hosted services over the network."),
                imp(Im::Data, Po::Optional, "Only data you send to trial features leaves the device."),
                imp(Im::Export, Po::Retained, "Everything created during the trial remains exportable locally."),
                imp(Im::Rollback, Po::Retained, "When the trial ends, you keep working on local and BYOK paths."),
            ],
            vec![
                alt(Alt::LocalOnly, true, "Keep working fully local after the trial.", Some(REF_LOCAL_ONLY)),
                alt(Alt::Byok, true, "Use your own provider key after the trial.", Some(REF_BYOK)),
                alt(Alt::SelfHosted, true, "Move to a self-hosted setup after the trial.", Some(REF_SELF_HOSTED)),
                alt(Alt::Export, true, "Export everything created during the trial.", Some(REF_EXPORT)),
                alt(Alt::Rollback, true, "Revert to your pre-trial configuration.", Some(REF_ROLLBACK)),
            ],
            "glossary.term.trial_window",
        ),
    ]
}

fn shared_concept_ids() -> Vec<String> {
    strings(&[CLOUD_SYNC_CONCEPT, PREMIUM_MODELS_CONCEPT])
}

fn trust_review() -> BoundaryTrustReview {
    BoundaryTrustReview {
        one_controlled_vocabulary_across_surfaces: true,
        wording_never_overstates_actual_boundary: true,
        narrowing_or_widening_references_support_metadata: true,
        identity_network_data_export_rollback_explained: true,
        upgrade_prompts_disclose_local_byok_self_hosted_alternatives: true,
        never_pressures_away_from_local_or_open_path: true,
        never_implies_vendor_dependence_when_core_local: true,
        managed_or_paid_introductions_keep_export_and_rollback: true,
        copy_parity_lint_blocks_cross_surface_drift: true,
        review_can_fail_on_parity_or_honesty_drift_without_code_change: true,
        machine_anchored_to_compatibility_and_support_metadata: true,
        one_catalog_not_parallel_boundary_prose_islands: true,
    }
}

fn parity_projection() -> BoundaryParityProjection {
    BoundaryParityProjection {
        settings_resolves_through_catalog: true,
        onboarding_resolves_through_catalog: true,
        marketplace_resolves_through_catalog: true,
        help_about_resolves_through_catalog: true,
        release_notes_resolve_through_catalog: true,
        account_upgrade_prompt_resolves_through_catalog: true,
    }
}

fn proof_freshness() -> BoundaryProofFreshness {
    BoundaryProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> BoundaryReleasePosture {
    BoundaryReleasePosture {
        release_packet_ref: "evidence:boundary-wording-catalog-release-packet:m5".to_owned(),
        mirror_offline_packet_ref: "evidence:boundary-wording-catalog-mirror-offline-packet:m5"
            .to_owned(),
        support_export_parity_required: true,
        mirror_offline_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        BOUNDARY_WORDING_CATALOG_SCHEMA_REF,
        BOUNDARY_WORDING_CATALOG_DOC_REF,
        CONTENT_WORDING_MATRIX_DOC_REF,
        UI_COPY_CONTRACT_REF,
        NAMING_LABEL_CONTRACT_REF,
        CONTROLLED_GLOSSARY_REF,
        DEPLOYMENT_PROFILES_REF,
        PRODUCT_TRUTH_VOCABULARY_REF,
    ])
}

fn base_input() -> BoundaryWordingCatalogInput {
    BoundaryWordingCatalogInput {
        catalog_id: BOUNDARY_WORDING_CATALOG_ID.to_owned(),
        catalog_label: "Hosted/Local/Self-hosted/Commercial Boundary Wording Across M5 Surfaces"
            .to_owned(),
        reference_locale: "en".to_owned(),
        entries: entries(),
        shared_concept_ids: shared_concept_ids(),
        trust_review: trust_review(),
        parity_projection: parity_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Builds the canonical boundary-wording catalog.
///
/// This is the single producer of the checked-in support export.
pub fn seeded_boundary_wording_catalog() -> BoundaryWordingCatalog {
    BoundaryWordingCatalog::new(base_input())
}

/// Builds a localized overlay of the canonical catalog.
///
/// Only the human prose changes: entry canonical text, implication disclosures, and
/// alternative disclosures are pseudo-localized, while every entry id, concept id,
/// term, surface, posture, support ref, alternative ref, and source ref stays
/// byte-for-byte identical. A localized overlay can never fork a concept id or a
/// support ref into a different boundary claim.
pub fn seeded_boundary_wording_catalog_localized() -> BoundaryWordingCatalog {
    let mut input = base_input();
    input.catalog_id = "m5-boundary-wording-catalog:localized:0001".to_owned();
    input.catalog_label = format!("{} (localized overlay)", input.catalog_label);
    input.reference_locale = "qps-ploc".to_owned();
    for entry in &mut input.entries {
        entry.canonical_text = pseudo_localize_prose(&entry.canonical_text);
        for statement in &mut entry.implications {
            statement.disclosure = pseudo_localize_prose(&statement.disclosure);
        }
        for disclosure in &mut entry.alternative_paths {
            disclosure.disclosure = pseudo_localize_prose(&disclosure.disclosure);
        }
    }
    BoundaryWordingCatalog::new(input)
}

/// Builds an offline-mirror variant of the canonical catalog.
///
/// The catalog identity and entries are unchanged; only the catalog id and the
/// mirror/offline release ref differ. This proves the catalog survives an offline
/// mirror without forking the meaning of any boundary claim.
pub fn seeded_boundary_wording_catalog_offline_mirror() -> BoundaryWordingCatalog {
    let mut input = base_input();
    input.catalog_id = "m5-boundary-wording-catalog:offline-mirror:0001".to_owned();
    input.catalog_label = format!("{} (offline mirror)", input.catalog_label);
    input.release_posture.release_packet_ref =
        "evidence:boundary-wording-catalog-release-packet:m5:mirror".to_owned();
    BoundaryWordingCatalog::new(input)
}
