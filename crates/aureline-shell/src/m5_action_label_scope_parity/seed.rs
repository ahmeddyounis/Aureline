//! Canonical seed builders for the action-label and count/scope-language parity
//! catalog.
//!
//! These builders are the single producer of the checked-in support export and the
//! localized / offline-mirror fixtures. The headless emitter and the inline tests
//! both call them so the in-code catalog, the artifact, and the fixtures never
//! drift.

use super::*;

/// Stable catalog id for the canonical action-label/scope catalog.
pub const ACTION_LABEL_SCOPE_CATALOG_ID: &str = "m5-action-label-scope-catalog:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

use ActionSurface as Sf;
use ConsumerSurface as C;
use CountStatus as Cs;
use MutationClass as Mu;
use ReversibilityClass as Rv;
use ReviewState as Rs;
use ScopeClass as Sc;

fn scope_def(class: ScopeClass, phrase: &str) -> ScopeDefinition {
    ScopeDefinition {
        scope_id: class.as_str().to_owned(),
        scope_class: class,
        canonical_phrase: phrase.to_owned(),
        actionable: class.actionable(),
        is_exclusion: class.is_exclusion(),
        requires_count: class.requires_count(),
    }
}

fn verb(
    verb_id: &str,
    canonical_label: &str,
    reversibility: ReversibilityClass,
    default_mutation_class: MutationClass,
) -> ActionVerb {
    ActionVerb {
        verb_id: verb_id.to_owned(),
        canonical_label: canonical_label.to_owned(),
        reversibility,
        default_mutation_class,
    }
}

fn object(object_id: &str, singular: &str, plural: &str) -> ActionObject {
    ActionObject {
        object_id: object_id.to_owned(),
        singular_label: singular.to_owned(),
        plural_label: plural.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn label(
    label_id: &str,
    verb_ref: &str,
    object_ref: &str,
    scope_ref: ScopeClass,
    mutation_class: MutationClass,
    review_state: ReviewState,
    surface: ActionSurface,
    count_var: Option<&str>,
    reference_label: &str,
    scope_unambiguous_in_sheet: bool,
    discloses_side_effect: bool,
    screen_reader_label: &str,
    consumer_surfaces: &[ConsumerSurface],
) -> ActionLabel {
    ActionLabel {
        label_id: label_id.to_owned(),
        verb_ref: verb_ref.to_owned(),
        object_ref: object_ref.to_owned(),
        scope_ref: scope_ref.as_str().to_owned(),
        mutation_class,
        review_state,
        surface,
        count_var: count_var.map(str::to_owned),
        reference_label: reference_label.to_owned(),
        scope_unambiguous_in_sheet,
        discloses_side_effect,
        screen_reader_label: screen_reader_label.to_owned(),
        consumer_surfaces: consumer_surfaces.to_vec(),
    }
}

#[allow(clippy::too_many_arguments)]
fn disclosure(
    disclosure_id: &str,
    surface: ActionSurface,
    object_ref: &str,
    verb_ref: Option<&str>,
    primary_scope: ScopeClass,
    disclosed_scopes: &[ScopeClass],
    count_status: CountStatus,
    count_vars: &[&str],
    reference_phrase: &str,
    consumer_surfaces: &[ConsumerSurface],
) -> ScopeDisclosure {
    ScopeDisclosure {
        disclosure_id: disclosure_id.to_owned(),
        surface,
        object_ref: object_ref.to_owned(),
        verb_ref: verb_ref.map(str::to_owned),
        primary_scope_ref: primary_scope.as_str().to_owned(),
        disclosed_scope_refs: disclosed_scopes
            .iter()
            .map(|s| s.as_str().to_owned())
            .collect(),
        count_status,
        count_vars: count_vars.iter().map(|s| (*s).to_owned()).collect(),
        reference_phrase: reference_phrase.to_owned(),
        consumer_surfaces: consumer_surfaces.to_vec(),
    }
}

fn scopes() -> Vec<ScopeDefinition> {
    vec![
        scope_def(Sc::Selected, "selected"),
        scope_def(Sc::Visible, "visible"),
        scope_def(Sc::Loaded, "loaded"),
        scope_def(Sc::AllMatching, "all matching"),
        scope_def(Sc::HiddenByPolicy, "hidden by policy"),
        scope_def(Sc::OutsideCurrentWorkset, "outside current workset"),
        scope_def(Sc::SingleObject, "this item"),
    ]
}

fn verbs() -> Vec<ActionVerb> {
    vec![
        verb("approve", "Approve", Rv::UndoableWindow, Mu::Approval),
        verb("rerun", "Rerun", Rv::Reversible, Mu::BatchMutation),
        verb("apply", "Apply", Rv::UndoableWindow, Mu::BatchMutation),
        verb("delete", "Delete", Rv::Irreversible, Mu::Destructive),
        verb("export", "Export", Rv::Reversible, Mu::Export),
        verb("install", "Install", Rv::UndoableWindow, Mu::Install),
        verb("publish", "Publish", Rv::Irreversible, Mu::Publish),
    ]
}

fn objects() -> Vec<ActionObject> {
    vec![
        object("change", "change", "changes"),
        object("task", "task", "tasks"),
        object("file", "file", "files"),
        object("result", "result", "results"),
        object("extension", "extension", "extensions"),
        object("document", "document", "documents"),
        object("fix", "fix", "fixes"),
        object("finding", "finding", "findings"),
    ]
}

fn labels() -> Vec<ActionLabel> {
    let full = &[
        C::ProductUi,
        C::CliHelp,
        C::Docs,
        C::SupportExport,
        C::ScreenReader,
    ];
    let dialog = &[C::ProductUi, C::Docs, C::SupportExport, C::ScreenReader];

    vec![
        label(
            "action.review.approve_selected_changes",
            "approve",
            "change",
            Sc::Selected,
            Mu::Approval,
            Rs::Reviewed,
            Sf::ReviewSheet,
            Some("count"),
            "{verb} {count:count} {scope:selected} {object_many}",
            false,
            true,
            "{verb} {count:count} {scope:selected} {object_many}",
            full,
        ),
        label(
            "action.batch.approve_all_matching_changes",
            "approve",
            "change",
            Sc::AllMatching,
            Mu::BatchMutation,
            Rs::UnreviewedBatch,
            Sf::BatchActionBar,
            None,
            "{verb} {scope:all_matching} {object_many}",
            false,
            true,
            "{verb} {scope:all_matching} {object_many}",
            &[
                C::ProductUi,
                C::CliHelp,
                C::Docs,
                C::SupportExport,
                C::ScreenReader,
                C::ActivityFeed,
            ],
        ),
        label(
            "action.batch.rerun_visible_tasks",
            "rerun",
            "task",
            Sc::Visible,
            Mu::BatchMutation,
            Rs::ReviewRequired,
            Sf::BatchActionBar,
            Some("count"),
            "{verb} {count:count} {scope:visible} {object_many}",
            false,
            true,
            "{verb} {count:count} {scope:visible} {object_many}",
            full,
        ),
        label(
            "action.batch.delete_selected_files",
            "delete",
            "file",
            Sc::Selected,
            Mu::Destructive,
            Rs::ReviewRequired,
            Sf::ConfirmationDialog,
            Some("count"),
            "{verb} {count:count} {scope:selected} {object_many}",
            false,
            true,
            "{verb} {count:count} {scope:selected} {object_many}",
            dialog,
        ),
        label(
            "action.export.export_loaded_results",
            "export",
            "result",
            Sc::Loaded,
            Mu::Export,
            Rs::NoReviewNeeded,
            Sf::ExportReportHeading,
            Some("count"),
            "{verb} {count:count} {scope:loaded} {object_many}",
            false,
            true,
            "{verb} {count:count} {scope:loaded} {object_many}",
            full,
        ),
        label(
            "action.install.install_extension",
            "install",
            "extension",
            Sc::SingleObject,
            Mu::Install,
            Rs::ReviewRequired,
            Sf::ConfirmationDialog,
            None,
            "{verb} {object_one}",
            false,
            true,
            "{verb} {object_one}",
            dialog,
        ),
        label(
            "action.publish.publish_selected_documents",
            "publish",
            "document",
            Sc::Selected,
            Mu::Publish,
            Rs::ReviewRequired,
            Sf::ConfirmationDialog,
            Some("count"),
            "{verb} {count:count} {scope:selected} {object_many}",
            false,
            true,
            "{verb} {count:count} {scope:selected} {object_many}",
            dialog,
        ),
        label(
            "action.review.approve_changes_in_sheet",
            "approve",
            "change",
            Sc::Selected,
            Mu::Approval,
            Rs::Reviewed,
            Sf::ReviewSheet,
            Some("count"),
            // The review sheet already lists the selected changes, so the visible
            // button may omit the scope word; the narrated label still names it.
            "{verb} {count:count} {object_many}",
            true,
            true,
            "{verb} {count:count} {scope:selected} {object_many}",
            dialog,
        ),
        label(
            "action.batch.apply_all_matching_fixes",
            "apply",
            "fix",
            Sc::AllMatching,
            Mu::BatchMutation,
            Rs::PartiallyReviewed,
            Sf::BatchActionBar,
            None,
            "{verb} {scope:all_matching} {object_many}",
            false,
            true,
            "{verb} {scope:all_matching} {object_many}",
            full,
        ),
        label(
            "action.cli.export_all_matching_results",
            "export",
            "result",
            Sc::AllMatching,
            Mu::Export,
            Rs::NoReviewNeeded,
            Sf::CliHelpSummary,
            None,
            "{verb} {scope:all_matching} {object_many}",
            false,
            false,
            "{verb} {scope:all_matching} {object_many}",
            full,
        ),
    ]
}

fn disclosures() -> Vec<ScopeDisclosure> {
    vec![
        disclosure(
            "disclosure.batch_bar.selected_with_policy_excluded",
            Sf::BatchActionBar,
            "change",
            None,
            Sc::Selected,
            &[Sc::HiddenByPolicy, Sc::OutsideCurrentWorkset],
            Cs::Exact,
            &["acted_count", "hidden_count", "outside_count"],
            "{count:acted_count} {scope:selected} {object_many} ({count_status}); {count:hidden_count} {scope:hidden_by_policy}, {count:outside_count} {scope:outside_current_workset} not included.",
            &[C::ProductUi, C::Docs, C::SupportExport, C::ScreenReader],
        ),
        disclosure(
            "disclosure.activity_row.reran_loaded_tasks",
            Sf::ToastActivityRow,
            "task",
            Some("rerun"),
            Sc::Loaded,
            &[],
            Cs::Exact,
            &["acted_count"],
            "{verb}: {count:acted_count} {scope:loaded} {object_many} ({count_status}).",
            &[C::ProductUi, C::ActivityFeed, C::SupportExport, C::ScreenReader],
        ),
        disclosure(
            "disclosure.export_heading.all_matching_with_status",
            Sf::ExportReportHeading,
            "finding",
            None,
            Sc::AllMatching,
            &[Sc::HiddenByPolicy],
            Cs::Approximate,
            &["total_count", "hidden_count"],
            "{count:total_count} {scope:all_matching} {object_many} ({count_status}); {count:hidden_count} {scope:hidden_by_policy} withheld.",
            &[C::ProductUi, C::Docs, C::SupportExport],
        ),
        disclosure(
            "disclosure.cli.loaded_vs_all_matching_results",
            Sf::CliHelpSummary,
            "result",
            None,
            Sc::Loaded,
            &[Sc::AllMatching],
            Cs::Partial,
            &["loaded_count", "matching_count"],
            "{count:loaded_count} {scope:loaded} of {count:matching_count} {scope:all_matching} {object_many} ({count_status}).",
            &[C::CliHelp, C::Docs, C::SupportExport],
        ),
        disclosure(
            "disclosure.review_sheet.selected_outside_workset",
            Sf::ReviewSheet,
            "change",
            None,
            Sc::Selected,
            &[Sc::OutsideCurrentWorkset],
            Cs::Exact,
            &["acted_count", "outside_count"],
            "{count:acted_count} {scope:selected} {object_many} ({count_status}); {count:outside_count} {scope:outside_current_workset} not included.",
            &[C::ProductUi, C::Docs, C::SupportExport, C::ScreenReader],
        ),
    ]
}

fn banned_ambiguous_tokens() -> Vec<String> {
    ActionLabelScopeCatalog::required_banned_tokens()
        .iter()
        .map(|s| (*s).to_owned())
        .collect()
}

fn shared_scope_phrase_ids() -> Vec<String> {
    [
        ScopeClass::Selected,
        ScopeClass::AllMatching,
        ScopeClass::Loaded,
    ]
    .iter()
    .map(|s| s.as_str().to_owned())
    .collect()
}

fn parity_review() -> ParityReview {
    ParityReview {
        labels_are_verb_first: true,
        no_ambiguous_primary_labels: true,
        scope_declared_or_unambiguous_in_sheet: true,
        batch_actions_declare_count: true,
        review_state_declared_on_approval_and_batch: true,
        object_class_narrowed: true,
        one_controlled_scope_phrase_set: true,
        side_effects_disclosed: true,
        screen_reader_labels_complete: true,
        docs_and_export_reuse_runtime_labels: true,
    }
}

fn consumer_projection() -> ConsumerProjection {
    ConsumerProjection {
        product_ui_resolves_through_catalog: true,
        cli_help_uses_action_labels: true,
        docs_render_action_labels: true,
        support_export_uses_action_labels: true,
        screen_reader_reuses_labels: true,
        activity_feed_reuses_labels: true,
    }
}

fn proof_freshness() -> CatalogProofFreshness {
    CatalogProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> CatalogReleasePosture {
    CatalogReleasePosture {
        release_packet_ref: "evidence:action-label-scope-catalog-release-packet:m5".to_owned(),
        mirror_offline_packet_ref: "evidence:action-label-scope-catalog-mirror-offline-packet:m5"
            .to_owned(),
        support_export_parity_required: true,
        mirror_offline_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    [
        ACTION_LABEL_SCOPE_CATALOG_SCHEMA_REF,
        ACTION_LABEL_SCOPE_CATALOG_DOC_REF,
        CATALOG_UI_COPY_CONTRACT_REF,
        CATALOG_NAMING_LABEL_CONTRACT_REF,
        CATALOG_COUNT_SCOPE_GRAMMAR_REF,
        CATALOG_COUNT_SCOPE_TERM_SET_REF,
        CATALOG_SAFETY_CRITICAL_SCHEMA_REF,
        CATALOG_SAFETY_CRITICAL_DOC_REF,
    ]
    .iter()
    .map(|s| (*s).to_owned())
    .collect()
}

fn base_input() -> ActionLabelScopeCatalogInput {
    ActionLabelScopeCatalogInput {
        catalog_id: ACTION_LABEL_SCOPE_CATALOG_ID.to_owned(),
        catalog_label: "Stable Action-Label and Count/Scope-Language Parity Catalog".to_owned(),
        reference_locale: "en".to_owned(),
        banned_ambiguous_tokens: banned_ambiguous_tokens(),
        scopes: scopes(),
        verbs: verbs(),
        objects: objects(),
        labels: labels(),
        disclosures: disclosures(),
        shared_scope_phrase_ids: shared_scope_phrase_ids(),
        parity_review: parity_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Builds the canonical stable action-label/scope catalog.
///
/// This is the single producer of the checked-in support export.
pub fn seeded_action_label_scope_catalog() -> ActionLabelScopeCatalog {
    ActionLabelScopeCatalog::new(base_input())
}

/// Builds a localized overlay of the canonical catalog.
///
/// Only the human prose changes: every verb label, scope phrase, object noun, and
/// reference template is pseudo-localized while each id, ref, count-variable name,
/// and `{...}` placeholder stays byte-for-byte identical. This proves machine-facing
/// identity stays locale-neutral while prose localizes safely around it.
pub fn seeded_action_label_scope_catalog_localized() -> ActionLabelScopeCatalog {
    let mut input = base_input();
    input.catalog_id = "m5-action-label-scope-catalog:localized:0001".to_owned();
    input.catalog_label =
        "Stable Action-Label and Count/Scope-Language Parity Catalog (localized overlay)"
            .to_owned();
    input.reference_locale = "qps-ploc".to_owned();
    for verb in &mut input.verbs {
        verb.canonical_label = pseudo_localize_phrase(&verb.canonical_label);
    }
    for scope in &mut input.scopes {
        scope.canonical_phrase = pseudo_localize_phrase(&scope.canonical_phrase);
    }
    for object in &mut input.objects {
        object.singular_label = pseudo_localize_phrase(&object.singular_label);
        object.plural_label = pseudo_localize_phrase(&object.plural_label);
    }
    for label in &mut input.labels {
        label.reference_label = pseudo_localize_template(&label.reference_label);
        label.screen_reader_label = pseudo_localize_template(&label.screen_reader_label);
    }
    for disclosure in &mut input.disclosures {
        disclosure.reference_phrase = pseudo_localize_template(&disclosure.reference_phrase);
    }
    ActionLabelScopeCatalog::new(input)
}

/// Builds an offline-mirror variant of the canonical catalog.
///
/// The catalog identity, scopes, verbs, objects, labels, and disclosures are
/// unchanged; only the catalog id and the mirror/offline release refs differ. This
/// proves the catalog survives an offline mirror without forking any wording.
pub fn seeded_action_label_scope_catalog_offline_mirror() -> ActionLabelScopeCatalog {
    let mut input = base_input();
    input.catalog_id = "m5-action-label-scope-catalog:offline-mirror:0001".to_owned();
    input.catalog_label =
        "Stable Action-Label and Count/Scope-Language Parity Catalog (offline mirror)".to_owned();
    input.release_posture.release_packet_ref =
        "evidence:action-label-scope-catalog-release-packet:m5:mirror".to_owned();
    ActionLabelScopeCatalog::new(input)
}
