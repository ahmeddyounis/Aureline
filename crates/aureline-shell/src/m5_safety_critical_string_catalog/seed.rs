//! Canonical seed builders for the stable safety-critical string catalog.
//!
//! These builders are the single producer of the checked-in support export and the
//! localized / offline-mirror fixtures. The headless emitter and the inline tests
//! both call them so the in-code catalog, the artifact, and the fixtures never
//! drift.

use super::*;

/// Stable catalog id for the canonical safety-critical string catalog.
pub const SAFETY_CRITICAL_STRING_CATALOG_ID: &str = "m5-safety-critical-string-catalog:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

use AliasPosture as A;
use CatalogConsumerSurface as C;
use ControlledTermClass as K;
use MessageAudience as Au;
use MessageClass as Cl;
use MessageSeverity as Sv;
use MessageSurfaceFamily as Sf;
use TruncationStrategy as Tr;
use VariableRole as R;

fn surfaces(items: &[MessageSurfaceFamily]) -> Vec<MessageSurfaceFamily> {
    items.to_vec()
}

fn consumers(items: &[CatalogConsumerSurface]) -> Vec<CatalogConsumerSurface> {
    items.to_vec()
}

#[allow(clippy::too_many_arguments)]
fn term(
    term_id: &str,
    term_class: ControlledTermClass,
    machine_token: &str,
    canonical_label: &str,
    reserved_meaning: &str,
    alias_posture: AliasPosture,
    allowed_surfaces: &[MessageSurfaceFamily],
) -> ControlledTerm {
    ControlledTerm {
        term_id: term_id.to_owned(),
        term_class,
        machine_token: machine_token.to_owned(),
        canonical_label: canonical_label.to_owned(),
        reserved_meaning: reserved_meaning.to_owned(),
        alias_posture,
        never_softened: true,
        allowed_surfaces: surfaces(allowed_surfaces),
    }
}

fn var(name: &str, role: VariableRole, truncatable: bool) -> MessageVariable {
    MessageVariable {
        name: name.to_owned(),
        role,
        locale_neutral_value: role.is_locale_neutral_value(),
        truncatable,
        term_ref: None,
    }
}

fn trunc(strategy: TruncationStrategy, note: &str) -> TruncationGuidance {
    TruncationGuidance {
        strategy,
        controlled_terms_never_dropped: true,
        note: note.to_owned(),
    }
}

/// The four reserved variables every error/recovery block carries.
fn recovery_vars() -> Vec<MessageVariable> {
    vec![
        var("what_failed", R::ScopeLabel, false),
        var("likely_cause", R::ScopeLabel, true),
        var("what_still_works", R::ScopeLabel, false),
        var("next_safe_action", R::ScopeLabel, false),
    ]
}

#[allow(clippy::too_many_arguments)]
fn message(
    message_id: &str,
    message_class: MessageClass,
    audience: MessageAudience,
    severity: MessageSeverity,
    surface_family: MessageSurfaceFamily,
    glossary_term_refs: &[&str],
    variables: Vec<MessageVariable>,
    reference_template: &str,
    truncation: TruncationGuidance,
    consumer_surfaces: &[CatalogConsumerSurface],
    next_action_label_ref: Option<&str>,
) -> SafetyCriticalMessage {
    SafetyCriticalMessage {
        message_id: message_id.to_owned(),
        message_class,
        audience,
        severity,
        surface_family,
        glossary_term_refs: glossary_term_refs.iter().map(|s| (*s).to_owned()).collect(),
        variables,
        reference_template: reference_template.to_owned(),
        truncation,
        consumer_surfaces: consumers(consumer_surfaces),
        next_action_label_ref: next_action_label_ref.map(str::to_owned),
    }
}

fn terms() -> Vec<ControlledTerm> {
    // Surface groups shared by term class; recovery action blocks render verb
    // labels, never state terms, so no state term is allowed there.
    let trust_policy = &[
        Sf::TrustPrompt,
        Sf::DegradedStateBanner,
        Sf::ProjectDoctorFinding,
        Sf::AiReviewFlow,
        Sf::ExecutionContextSheet,
        Sf::SupportExportHeading,
        Sf::RuntimeStatus,
    ];
    let freshness = &[
        Sf::DegradedStateBanner,
        Sf::ProjectDoctorFinding,
        Sf::AiReviewFlow,
        Sf::ExecutionContextSheet,
        Sf::SupportExportHeading,
        Sf::RuntimeStatus,
    ];
    let compatibility = &[
        Sf::ProjectDoctorFinding,
        Sf::ExecutionContextSheet,
        Sf::SupportExportHeading,
        Sf::RuntimeStatus,
    ];
    let lifecycle = &[
        Sf::AiReviewFlow,
        Sf::ExecutionContextSheet,
        Sf::SupportExportHeading,
        Sf::RuntimeStatus,
    ];
    let client_scope = &[
        Sf::DegradedStateBanner,
        Sf::ExecutionContextSheet,
        Sf::SupportExportHeading,
        Sf::RuntimeStatus,
    ];

    vec![
        term(
            "term.unverified_source",
            K::Trust,
            "unverified_source",
            "Unverified source",
            "The source has not had its authority verified; trust must be established before it acts.",
            A::NoAlias,
            trust_policy,
        ),
        term(
            "term.official_source",
            K::Trust,
            "official_source",
            "Official source",
            "A first-party, verifiable source that may act without a per-use trust prompt.",
            A::NoAlias,
            trust_policy,
        ),
        term(
            "term.trust_required",
            K::Policy,
            "trust_required",
            "Trust required",
            "The action cannot proceed until the operator grants trust to the source or target.",
            A::NoAlias,
            trust_policy,
        ),
        term(
            "term.policy_blocked",
            K::Policy,
            "policy_blocked",
            "Policy blocked",
            "The action is blocked by an active policy and cannot be run as requested.",
            A::NoAlias,
            trust_policy,
        ),
        term(
            "term.restricted_scope",
            K::Policy,
            "restricted",
            "Restricted",
            "The action is permitted only within a narrower, disclosed scope.",
            A::ControlledAlias,
            trust_policy,
        ),
        term(
            "term.requires_review",
            K::Policy,
            "requires_review",
            "Requires review",
            "The action must be explicitly reviewed and confirmed before it runs.",
            A::NoAlias,
            trust_policy,
        ),
        term(
            "term.incompatible_target",
            K::Compatibility,
            "incompatible",
            "Incompatible",
            "The claim is incompatible with the active target and cannot be applied safely.",
            A::NoAlias,
            compatibility,
        ),
        term(
            "term.minor_skew",
            K::Compatibility,
            "minor_skew_compatible",
            "Minor version skew",
            "The claim is compatible within an accepted minor-skew window, not exact.",
            A::NoAlias,
            compatibility,
        ),
        term(
            "term.proven_current",
            K::Freshness,
            "proven_current",
            "Current",
            "The data is proven current for its declared scope and freshness basis.",
            A::NoAlias,
            freshness,
        ),
        term(
            "term.cached",
            K::Freshness,
            "cached",
            "Cached",
            "The data is shown from a cache with a disclosed cache posture, not proven current.",
            A::NoAlias,
            freshness,
        ),
        term(
            "term.stale",
            K::Freshness,
            "stale",
            "Stale",
            "Prior data shown after its freshness floor or causal continuity was lost.",
            A::NoAlias,
            freshness,
        ),
        term(
            "term.warming",
            K::Freshness,
            "warming",
            "Warming",
            "The data is warming and not yet complete for the declared scope.",
            A::NoAlias,
            freshness,
        ),
        term(
            "term.local_only",
            K::ClientScope,
            "local_only",
            "Local only",
            "A local-only posture with no managed recall, sync, or hosted evidence.",
            A::NoAlias,
            client_scope,
        ),
        term(
            "term.browser_companion",
            K::ClientScope,
            "browser_companion",
            "Browser companion",
            "A browser companion surface that does not imply full desktop parity.",
            A::NoAlias,
            client_scope,
        ),
        term(
            "term.preview",
            K::Lifecycle,
            "preview",
            "Preview",
            "A preview capability that is not yet broadly claimed as stable.",
            A::NoAlias,
            lifecycle,
        ),
        term(
            "term.beta",
            K::Lifecycle,
            "beta",
            "Beta",
            "A beta capability under active hardening, narrower than stable.",
            A::NoAlias,
            lifecycle,
        ),
        term(
            "term.disabled_by_policy",
            K::Lifecycle,
            "disabled_by_policy",
            "Disabled by policy",
            "The capability is disabled by an active policy on this deployment.",
            A::NoAlias,
            lifecycle,
        ),
    ]
}

fn messages() -> Vec<SafetyCriticalMessage> {
    vec![
        message(
            "msg.trust.unverified_source_prompt",
            Cl::SafetyCriticalString,
            Au::EndUser,
            Sv::Warning,
            Sf::TrustPrompt,
            &["term.unverified_source", "term.trust_required"],
            vec![var("source_name", R::EntityName, true)],
            "{term:term.unverified_source}: \u{201c}{var:source_name}\u{201d} has not been verified. {term:term.trust_required} before this runs.",
            trunc(
                Tr::PriorityDropTrailingClause,
                "Keep the state term and the trust requirement; the source name may elide.",
            ),
            &[C::ProductUi, C::SupportExport, C::HelpAbout],
            Some("msg.trust.grant_trust_action"),
        ),
        message(
            "msg.trust.grant_trust_action",
            Cl::ActionLabel,
            Au::EndUser,
            Sv::Notice,
            Sf::TrustPrompt,
            &[],
            vec![var("source_name", R::EntityName, true)],
            "Grant trust to {var:source_name}",
            trunc(Tr::TruncateVariableTail, "The verb stays; the source name may elide."),
            &[C::ProductUi, C::CliHelp],
            None,
        ),
        message(
            "msg.policy.action_blocked_banner",
            Cl::ErrorRecoveryBlock,
            Au::EndUser,
            Sv::Blocking,
            Sf::DegradedStateBanner,
            &["term.policy_blocked"],
            recovery_vars(),
            "{term:term.policy_blocked}: {var:what_failed}. Likely cause: {var:likely_cause}. Still available: {var:what_still_works}. Next: {var:next_safe_action}.",
            trunc(
                Tr::PriorityDropTrailingClause,
                "Keep the blocked state, what still works, and the next action; the cause may elide.",
            ),
            &[C::ProductUi, C::SupportExport, C::Docs],
            Some("msg.recovery.request_access_action"),
        ),
        message(
            "msg.policy.restricted_scope_sheet",
            Cl::SafetyCriticalString,
            Au::Operator,
            Sv::Caution,
            Sf::ExecutionContextSheet,
            &["term.restricted_scope", "term.requires_review"],
            vec![var("scope_name", R::ScopeLabel, true)],
            "{term:term.restricted_scope} to {var:scope_name}; {term:term.requires_review} to widen.",
            trunc(Tr::PriorityDropTrailingClause, "Keep both state terms; the scope name may elide."),
            &[C::ProductUi, C::SupportExport, C::HelpAbout],
            None,
        ),
        message(
            "msg.runtime.degraded_local_only_banner",
            Cl::SafetyCriticalString,
            Au::EndUser,
            Sv::Warning,
            Sf::DegradedStateBanner,
            &["term.local_only", "term.stale", "term.warming"],
            vec![var("feature_name", R::EntityName, true)],
            "{term:term.local_only}. {var:feature_name} shows {term:term.stale} or {term:term.warming} data until reconnected.",
            trunc(
                Tr::PriorityDropTrailingClause,
                "Keep the local-only and freshness terms; the feature name may elide.",
            ),
            &[C::ProductUi, C::SupportExport, C::ScreenReader],
            Some("msg.recovery.reconnect_action"),
        ),
        message(
            "msg.doctor.stale_index_finding",
            Cl::ErrorRecoveryBlock,
            Au::Developer,
            Sv::Caution,
            Sf::ProjectDoctorFinding,
            &["term.stale"],
            recovery_vars(),
            "{term:term.stale} index: {var:what_failed}. Likely cause: {var:likely_cause}. Still available: {var:what_still_works}. Next: {var:next_safe_action}.",
            trunc(
                Tr::PriorityDropTrailingClause,
                "Keep the stale state, what still works, and the next action; the cause may elide.",
            ),
            &[C::ProductUi, C::SupportExport, C::Docs],
            Some("msg.recovery.rebuild_index_action"),
        ),
        message(
            "msg.doctor.incompatible_target_finding",
            Cl::ErrorRecoveryBlock,
            Au::Developer,
            Sv::Critical,
            Sf::ProjectDoctorFinding,
            &["term.incompatible_target", "term.minor_skew"],
            recovery_vars(),
            "{term:term.incompatible_target} (not {term:term.minor_skew}): {var:what_failed}. Likely cause: {var:likely_cause}. Still available: {var:what_still_works}. Next: {var:next_safe_action}.",
            trunc(
                Tr::PriorityDropTrailingClause,
                "Keep the compatibility terms, what still works, and the next action; the cause may elide.",
            ),
            &[C::ProductUi, C::SupportExport, C::Docs],
            None,
        ),
        message(
            "msg.ai.evidence_basis_line",
            Cl::AiCopyLine,
            Au::EndUser,
            Sv::Notice,
            Sf::AiReviewFlow,
            &["term.cached", "term.proven_current"],
            vec![var("source_count", R::Count, false)],
            "Based on {var:source_count} sources; freshness is {term:term.cached}, not {term:term.proven_current}.",
            trunc(Tr::PriorityDropTrailingClause, "Keep the freshness disclosure; the count stays exact."),
            &[C::ProductUi, C::AiSurface, C::SupportExport],
            None,
        ),
        message(
            "msg.ai.autonomy_disclosure_line",
            Cl::AiCopyLine,
            Au::EndUser,
            Sv::Notice,
            Sf::AiReviewFlow,
            &["term.requires_review"],
            vec![var("step_count", R::Count, false)],
            "Proposed {var:step_count} steps; each {term:term.requires_review} before it runs.",
            trunc(Tr::PriorityDropTrailingClause, "Keep the review requirement; the count stays exact."),
            &[C::ProductUi, C::AiSurface, C::SupportExport],
            None,
        ),
        message(
            "msg.exec.trust_required_sheet_heading",
            Cl::SafetyCriticalString,
            Au::Operator,
            Sv::Caution,
            Sf::ExecutionContextSheet,
            &["term.trust_required", "term.official_source"],
            vec![var("target_name", R::EntityName, true)],
            "{term:term.trust_required} for {var:target_name}; only an {term:term.official_source} runs unprompted.",
            trunc(Tr::PriorityDropTrailingClause, "Keep both trust terms; the target name may elide."),
            &[C::ProductUi, C::SupportExport, C::HelpAbout],
            None,
        ),
        message(
            "msg.count.visible_scope_phrase",
            Cl::CountScopePhrase,
            Au::EndUser,
            Sv::Info,
            Sf::RuntimeStatus,
            &["term.cached"],
            vec![
                var("visible_count", R::Count, false),
                var("total_count", R::Count, false),
                var("omitted_reason", R::ScopeLabel, true),
            ],
            "Showing {var:visible_count} of {var:total_count}; {var:omitted_reason}. Count is {term:term.cached}.",
            trunc(
                Tr::PriorityDropTrailingClause,
                "Keep both counts and the freshness term; the omission reason may elide.",
            ),
            &[C::ProductUi, C::CliHelp, C::SupportExport],
            None,
        ),
        message(
            "msg.count.search_stale_phrase",
            Cl::CountScopePhrase,
            Au::EndUser,
            Sv::Info,
            Sf::RuntimeStatus,
            &["term.stale"],
            vec![var("match_count", R::Count, false)],
            "{var:match_count} matches ({term:term.stale}).",
            trunc(Tr::NeverTruncate, "Both the count and the freshness term are kept."),
            &[C::ProductUi, C::CliHelp, C::SupportExport],
            None,
        ),
        message(
            "msg.support.trust_state_heading",
            Cl::SafetyCriticalString,
            Au::Support,
            Sv::Info,
            Sf::SupportExportHeading,
            &["term.trust_required", "term.policy_blocked", "term.unverified_source"],
            vec![var("subject_name", R::EntityName, true)],
            "Trust state for {var:subject_name}: {term:term.trust_required}, {term:term.policy_blocked}, {term:term.unverified_source}.",
            trunc(Tr::TruncateVariableTail, "All state terms are kept; the subject name may elide."),
            &[C::SupportExport, C::Docs],
            None,
        ),
        message(
            "msg.support.freshness_state_heading",
            Cl::SafetyCriticalString,
            Au::Support,
            Sv::Info,
            Sf::SupportExportHeading,
            &["term.proven_current", "term.cached", "term.stale"],
            vec![var("subject_name", R::EntityName, true)],
            "Freshness state for {var:subject_name}: {term:term.proven_current}, {term:term.cached}, {term:term.stale}.",
            trunc(Tr::TruncateVariableTail, "All freshness terms are kept; the subject name may elide."),
            &[C::SupportExport, C::Docs],
            None,
        ),
        message(
            "msg.recovery.reconnect_action",
            Cl::ActionLabel,
            Au::EndUser,
            Sv::Notice,
            Sf::RecoveryActionBlock,
            &[],
            vec![var("target_name", R::EntityName, true)],
            "Reconnect to {var:target_name}",
            trunc(Tr::TruncateVariableTail, "The verb stays; the target name may elide."),
            &[C::ProductUi, C::CliHelp],
            None,
        ),
        message(
            "msg.recovery.request_access_action",
            Cl::ActionLabel,
            Au::EndUser,
            Sv::Notice,
            Sf::RecoveryActionBlock,
            &[],
            vec![var("action_name", R::EntityName, true)],
            "Request access for {var:action_name}",
            trunc(Tr::TruncateVariableTail, "The verb stays; the action name may elide."),
            &[C::ProductUi, C::CliHelp],
            None,
        ),
        message(
            "msg.recovery.rebuild_index_action",
            Cl::ActionLabel,
            Au::Developer,
            Sv::Notice,
            Sf::RecoveryActionBlock,
            &[],
            vec![],
            "Rebuild index",
            trunc(Tr::NeverTruncate, "A fixed verb phrase; nothing is dropped."),
            &[C::ProductUi, C::CliHelp],
            None,
        ),
        message(
            "msg.runtime.disabled_by_policy_status",
            Cl::SafetyCriticalString,
            Au::Operator,
            Sv::Caution,
            Sf::RuntimeStatus,
            &["term.disabled_by_policy", "term.policy_blocked"],
            vec![var("capability_name", R::EntityName, true)],
            "{var:capability_name} is {term:term.disabled_by_policy} ({term:term.policy_blocked}).",
            trunc(Tr::TruncateVariableTail, "Both state terms are kept; the capability name may elide."),
            &[C::ProductUi, C::SupportExport, C::HelpAbout],
            None,
        ),
        message(
            "msg.runtime.lifecycle_status",
            Cl::SafetyCriticalString,
            Au::Operator,
            Sv::Notice,
            Sf::RuntimeStatus,
            &["term.preview", "term.beta"],
            vec![var("capability_name", R::EntityName, true)],
            "{var:capability_name} lifecycle: {term:term.preview} or {term:term.beta}.",
            trunc(Tr::TruncateVariableTail, "Both lifecycle terms are kept; the capability name may elide."),
            &[C::ProductUi, C::SupportExport, C::HelpAbout],
            None,
        ),
        message(
            "msg.exec.client_scope_sheet",
            Cl::SafetyCriticalString,
            Au::EndUser,
            Sv::Info,
            Sf::ExecutionContextSheet,
            &["term.local_only", "term.browser_companion"],
            vec![var("surface_name", R::EntityName, true)],
            "{var:surface_name} scope: {term:term.local_only} or {term:term.browser_companion}.",
            trunc(Tr::TruncateVariableTail, "Both scope terms are kept; the surface name may elide."),
            &[C::ProductUi, C::SupportExport, C::HelpAbout],
            None,
        ),
        message(
            "msg.a11y.degraded_announcement",
            Cl::SafetyCriticalString,
            Au::ScreenReader,
            Sv::Warning,
            Sf::DegradedStateBanner,
            &["term.local_only", "term.stale"],
            vec![var("feature_name", R::EntityName, true)],
            "Now {term:term.local_only}. {var:feature_name} data is {term:term.stale}.",
            trunc(Tr::NeverTruncate, "Narrated state terms are always announced in full."),
            &[C::ProductUi, C::ScreenReader, C::SupportExport],
            None,
        ),
    ]
}

fn shared_reuse_term_ids() -> Vec<String> {
    [
        "term.trust_required",
        "term.policy_blocked",
        "term.stale",
        "term.cached",
    ]
    .iter()
    .map(|s| (*s).to_owned())
    .collect()
}

fn trust_review() -> CatalogTrustReview {
    CatalogTrustReview {
        messages_have_stable_locale_neutral_ids: true,
        controlled_terms_resolved_not_inlined: true,
        audience_and_severity_metadata_present: true,
        machine_tokens_stay_locale_neutral: true,
        human_prose_localizes_around_tokens: true,
        error_copy_explains_failure_remaining_and_next_action: true,
        ai_copy_never_overstates_confidence_or_autonomy: true,
        counts_disclose_freshness_and_scope: true,
        one_catalog_not_parallel_string_islands: true,
        truncation_never_drops_controlled_terms: true,
    }
}

fn consumer_projection() -> CatalogConsumerProjection {
    CatalogConsumerProjection {
        product_ui_resolves_through_catalog: true,
        cli_help_shows_controlled_terms: true,
        docs_render_controlled_terms: true,
        support_export_uses_catalog_headings: true,
        screen_reader_reuses_message_identities: true,
        ai_surfaces_honor_copy_guardrails: true,
        onboarding_uses_controlled_terms: true,
        help_about_shows_state_terms: true,
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
        release_packet_ref: "evidence:safety-critical-string-catalog-release-packet:m5".to_owned(),
        mirror_offline_packet_ref:
            "evidence:safety-critical-string-catalog-mirror-offline-packet:m5".to_owned(),
        support_export_parity_required: true,
        mirror_offline_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    [
        SAFETY_CRITICAL_STRING_CATALOG_SCHEMA_REF,
        SAFETY_CRITICAL_STRING_CATALOG_DOC_REF,
        CATALOG_UI_COPY_CONTRACT_REF,
        CATALOG_NAMING_LABEL_CONTRACT_REF,
        CATALOG_COUNT_SCOPE_GRAMMAR_REF,
        CATALOG_CONTENT_OPS_CONTRACT_REF,
        CATALOG_AI_COPY_GUARDRAILS_CONTRACT_REF,
        CATALOG_CONTROLLED_GLOSSARY_REF,
        CATALOG_PRODUCT_TRUTH_VOCABULARY_REF,
        CATALOG_WORDING_MATRIX_SCHEMA_REF,
        CATALOG_WORDING_MATRIX_DOC_REF,
    ]
    .iter()
    .map(|s| (*s).to_owned())
    .collect()
}

fn base_input() -> SafetyCriticalStringCatalogInput {
    SafetyCriticalStringCatalogInput {
        catalog_id: SAFETY_CRITICAL_STRING_CATALOG_ID.to_owned(),
        catalog_label: "Stable Safety-Critical String Catalog and Controlled Terms".to_owned(),
        reference_locale: "en".to_owned(),
        terms: terms(),
        messages: messages(),
        shared_reuse_term_ids: shared_reuse_term_ids(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Builds the canonical stable safety-critical string catalog.
///
/// This is the single producer of the checked-in support export.
pub fn seeded_safety_critical_string_catalog() -> SafetyCriticalStringCatalog {
    SafetyCriticalStringCatalog::new(base_input())
}

/// Builds a localized overlay of the canonical catalog.
///
/// Only the human prose changes: every reference template is pseudo-localized while
/// each message id, term id, machine token, variable name, and `{term:...}` /
/// `{var:...}` placeholder stays byte-for-byte identical. This proves machine-facing
/// identity stays locale-neutral while prose localizes safely around it.
pub fn seeded_safety_critical_string_catalog_localized() -> SafetyCriticalStringCatalog {
    let mut input = base_input();
    input.catalog_id = "m5-safety-critical-string-catalog:localized:0001".to_owned();
    input.catalog_label =
        "Stable Safety-Critical String Catalog and Controlled Terms (localized overlay)".to_owned();
    input.reference_locale = "qps-ploc".to_owned();
    for message in &mut input.messages {
        message.reference_template = pseudo_localize_template(&message.reference_template);
    }
    SafetyCriticalStringCatalog::new(input)
}

/// Builds an offline-mirror variant of the canonical catalog.
///
/// The catalog identity, terms, and messages are unchanged; only the catalog id and
/// the mirror/offline release refs differ. This proves the catalog survives an
/// offline mirror without forking the meaning of any state.
pub fn seeded_safety_critical_string_catalog_offline_mirror() -> SafetyCriticalStringCatalog {
    let mut input = base_input();
    input.catalog_id = "m5-safety-critical-string-catalog:offline-mirror:0001".to_owned();
    input.catalog_label =
        "Stable Safety-Critical String Catalog and Controlled Terms (offline mirror)".to_owned();
    input.release_posture.release_packet_ref =
        "evidence:safety-critical-string-catalog-release-packet:m5:mirror".to_owned();
    SafetyCriticalStringCatalog::new(input)
}
