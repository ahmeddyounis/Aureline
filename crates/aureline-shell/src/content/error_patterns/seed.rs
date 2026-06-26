//! Canonical seed builders for the error/recovery copy catalog.
//!
//! These builders are the single producer of the checked-in support export and the
//! localized / offline-mirror fixtures. The headless emitter and the inline tests
//! both call them so the in-code catalog, the artifact, and the fixtures never
//! drift.

use std::collections::BTreeSet;

use super::*;

/// Stable catalog id for the canonical error/recovery copy catalog.
pub const ERROR_RECOVERY_COPY_CATALOG_ID: &str = "m5-error-recovery-copy-catalog:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

use CopyRole as Ro;
use CopyVariableRole as Vr;
use DegradedState as Ds;
use FailureDomain as Fd;
use RecoveryConsumerSurface as Su;
use RecoveryLinkKind as Lk;
use RecoverySeverity as Sv;

/// Every consumer surface; chips are valid everywhere unless a surface narrows.
fn all_surfaces() -> Vec<RecoveryConsumerSurface> {
    RecoveryConsumerSurface::ALL.to_vec()
}

#[allow(clippy::too_many_arguments)]
fn chip(
    chip_id: &str,
    state: DegradedState,
    machine_token: &str,
    canonical_label: &str,
    reserved_meaning: &str,
    severity: RecoverySeverity,
    self_heals: bool,
    offers_recovery: bool,
) -> ReasonChip {
    ReasonChip {
        chip_id: chip_id.to_owned(),
        state,
        machine_token: machine_token.to_owned(),
        canonical_label: canonical_label.to_owned(),
        reserved_meaning: reserved_meaning.to_owned(),
        severity,
        self_heals,
        offers_recovery,
        grounded: true,
        allowed_surfaces: all_surfaces(),
    }
}

fn var(name: &str, role: CopyVariableRole, truncatable: bool) -> CopyVariable {
    CopyVariable {
        name: name.to_owned(),
        role,
        locale_neutral_value: role.is_locale_neutral_value(),
        truncatable,
    }
}

fn line(
    role: CopyRole,
    reference_template: &str,
    chip_refs: &[&str],
    variables: Vec<CopyVariable>,
) -> CopyLine {
    CopyLine {
        role,
        reference_template: reference_template.to_owned(),
        chip_refs: chip_refs.iter().map(|s| (*s).to_owned()).collect(),
        variables,
    }
}

#[allow(clippy::too_many_arguments)]
fn link(link_id: &str, kind: RecoveryLinkKind, target_ref: &str, label: &str) -> RecoveryLink {
    RecoveryLink {
        link_id: link_id.to_owned(),
        kind,
        target_ref: target_ref.to_owned(),
        label: label.to_owned(),
        offline_available: true,
    }
}

fn next_action(
    action_id: &str,
    label: &str,
    variables: Vec<CopyVariable>,
    recovery_link: RecoveryLink,
) -> NextAction {
    NextAction {
        action_id: action_id.to_owned(),
        label: label.to_owned(),
        variables,
        recovery_link,
    }
}

/// Builds a recovery block, deriving `reason_chips` from the union of the three
/// copy lines' chip refs so the declared set can never drift from the templates.
fn block(
    block_id: &str,
    failure_domain: FailureDomain,
    severity: RecoverySeverity,
    what_failed: CopyLine,
    why_likely: CopyLine,
    what_still_works: CopyLine,
    next_action: NextAction,
    consumer_surfaces: &[RecoveryConsumerSurface],
) -> RecoveryBlock {
    let mut chips: BTreeSet<String> = BTreeSet::new();
    for refs in [
        &what_failed.chip_refs,
        &why_likely.chip_refs,
        &what_still_works.chip_refs,
    ] {
        for chip_id in refs {
            chips.insert(chip_id.clone());
        }
    }
    RecoveryBlock {
        block_id: block_id.to_owned(),
        failure_domain,
        severity,
        what_failed,
        why_likely,
        what_still_works,
        next_action,
        reason_chips: chips.into_iter().collect(),
        consumer_surfaces: consumer_surfaces.to_vec(),
    }
}

fn chips() -> Vec<ReasonChip> {
    vec![
        chip(
            "chip.restricted",
            Ds::Restricted,
            "restricted",
            "Restricted",
            "The capability is permitted only within a narrower, disclosed scope until the restriction is lifted.",
            Sv::Caution,
            false,
            true,
        ),
        chip(
            "chip.partial_index",
            Ds::PartialIndex,
            "partial_index",
            "Partial index",
            "The index is still building, so results cover only the part indexed so far.",
            Sv::Caution,
            true,
            true,
        ),
        chip(
            "chip.remote_host",
            Ds::RemoteHost,
            "remote_host",
            "Remote host",
            "The work depends on a remote host whose reachability is not guaranteed.",
            Sv::Notice,
            false,
            false,
        ),
        chip(
            "chip.policy_blocked",
            Ds::PolicyBlocked,
            "policy_blocked",
            "Policy blocked",
            "An active policy blocks the action; it cannot run as requested on this deployment.",
            Sv::Blocking,
            false,
            true,
        ),
        chip(
            "chip.cached",
            Ds::Cached,
            "cached",
            "Cached",
            "Data is shown from a local cache with a disclosed cache posture, not proven current.",
            Sv::Notice,
            false,
            false,
        ),
        chip(
            "chip.stale",
            Ds::Stale,
            "stale",
            "Stale",
            "Prior data is shown after its freshness floor was passed; it may no longer be current.",
            Sv::Warning,
            false,
            true,
        ),
        chip(
            "chip.reconnecting",
            Ds::Reconnecting,
            "reconnecting",
            "Reconnecting",
            "A connection is being re-established; the state clears on its own when the link returns.",
            Sv::Notice,
            true,
            false,
        ),
        chip(
            "chip.rollback_available",
            Ds::RollbackAvailable,
            "rollback_available",
            "Rollback available",
            "A prior, known-good state is retained and can be restored.",
            Sv::Notice,
            false,
            true,
        ),
    ]
}

fn blocks() -> Vec<RecoveryBlock> {
    vec![
        block(
            "recovery.network.remote_host_unreachable",
            Fd::Network,
            Sv::Warning,
            line(
                Ro::WhatFailed,
                "The {chip:chip.remote_host} connection to {var:host_name} dropped.",
                &["chip.remote_host"],
                vec![var("host_name", Vr::EntityName, true)],
            ),
            line(
                Ro::WhyLikely,
                "The network path to {var:host_name} is likely unavailable right now.",
                &[],
                vec![var("host_name", Vr::EntityName, true)],
            ),
            line(
                Ro::WhatStillWorks,
                "Local edits, search, and history stay available; data shows as {chip:chip.stale} while {chip:chip.reconnecting}.",
                &["chip.stale", "chip.reconnecting"],
                vec![],
            ),
            next_action(
                "action.reconnect_remote_host",
                "Reconnect to {var:host_name}",
                vec![var("host_name", Vr::EntityName, true)],
                link(
                    "link.flow.reconnect_host",
                    Lk::ReconnectFlow,
                    "flow.network.reconnect",
                    "Open reconnect status",
                ),
            ),
            &[Su::DynamicBanner, Su::InlineBlocker, Su::SupportExport, Su::ScreenReader],
        ),
        block(
            "recovery.runtime.policy_blocked_action",
            Fd::Runtime,
            Sv::Blocking,
            line(
                Ro::WhatFailed,
                "{chip:chip.policy_blocked}: {var:action_name} cannot run on this deployment.",
                &["chip.policy_blocked"],
                vec![var("action_name", Vr::EntityName, true)],
            ),
            line(
                Ro::WhyLikely,
                "An administrator policy named {var:policy_name} disallows this action.",
                &[],
                vec![var("policy_name", Vr::EntityName, true)],
            ),
            line(
                Ro::WhatStillWorks,
                "Read-only work continues; allowed actions still run and recent results stay {chip:chip.cached}.",
                &["chip.cached"],
                vec![],
            ),
            next_action(
                "action.request_policy_access",
                "Request access for {var:action_name}",
                vec![var("action_name", Vr::EntityName, true)],
                link(
                    "link.pane.policy_access",
                    Lk::SettingsPane,
                    "pane.policy.access_request",
                    "Open policy settings",
                ),
            ),
            &[Su::InlineBlocker, Su::ProjectDoctor, Su::CliHelpSummary, Su::SupportExport],
        ),
        block(
            "recovery.repair.partial_index",
            Fd::Repair,
            Sv::Caution,
            line(
                Ro::WhatFailed,
                "Search is running on a {chip:chip.partial_index}; some results are missing.",
                &["chip.partial_index"],
                vec![],
            ),
            line(
                Ro::WhyLikely,
                "Indexing of {var:scope_name} has not finished after the last change.",
                &[],
                vec![var("scope_name", Vr::ScopeLabel, true)],
            ),
            line(
                Ro::WhatStillWorks,
                "Indexed files search normally; results so far are correct and shown as {chip:chip.cached}.",
                &["chip.cached"],
                vec![],
            ),
            next_action(
                "action.rebuild_project_index",
                "Rebuild the project index",
                vec![],
                link(
                    "link.flow.rebuild_index",
                    Lk::RepairFlow,
                    "flow.repair.rebuild_index",
                    "Open repair",
                ),
            ),
            &[Su::ProjectDoctor, Su::CliHelpSummary, Su::SupportExport, Su::ScreenshotCaption],
        ),
        block(
            "recovery.install.partial_install_rollback",
            Fd::Install,
            Sv::Critical,
            line(
                Ro::WhatFailed,
                "Installing {var:package_name} version {var:version_code} failed partway.",
                &[],
                vec![
                    var("package_name", Vr::EntityName, true),
                    var("version_code", Vr::Code, false),
                ],
            ),
            line(
                Ro::WhyLikely,
                "The install step likely could not write to {var:install_path}.",
                &[],
                vec![var("install_path", Vr::Location, true)],
            ),
            line(
                Ro::WhatStillWorks,
                "The previous version keeps running; a {chip:chip.rollback_available} restores it cleanly.",
                &["chip.rollback_available"],
                vec![],
            ),
            next_action(
                "action.roll_back_install",
                "Roll back the install",
                vec![],
                link(
                    "link.flow.install_rollback",
                    Lk::RollbackFlow,
                    "flow.install.rollback",
                    "Open rollback",
                ),
            ),
            &[Su::DynamicBanner, Su::SupportExport, Su::ScreenshotCaption],
        ),
        block(
            "recovery.review.restricted_scope",
            Fd::Review,
            Sv::Caution,
            line(
                Ro::WhatFailed,
                "Applying the change is {chip:chip.restricted} to {var:scope_name}.",
                &["chip.restricted"],
                vec![var("scope_name", Vr::ScopeLabel, true)],
            ),
            line(
                Ro::WhyLikely,
                "The change touches paths outside the reviewed scope {var:scope_name}.",
                &[],
                vec![var("scope_name", Vr::ScopeLabel, true)],
            ),
            line(
                Ro::WhatStillWorks,
                "In-scope edits apply now; out-of-scope changes stay {chip:chip.cached} for a later review.",
                &["chip.cached"],
                vec![],
            ),
            next_action(
                "action.request_scope_review",
                "Request review to widen scope",
                vec![],
                link(
                    "link.help.review_scope",
                    Lk::HelpTopic,
                    "help.review.widen_scope",
                    "Open review help",
                ),
            ),
            &[Su::InlineBlocker, Su::ProjectDoctor, Su::SupportExport],
        ),
        block(
            "recovery.docs_help.stale_offline",
            Fd::DocsHelp,
            Sv::Notice,
            line(
                Ro::WhatFailed,
                "Help content is {chip:chip.stale} while offline.",
                &["chip.stale"],
                vec![],
            ),
            line(
                Ro::WhyLikely,
                "The docs pack has not refreshed since {var:since_time} without a connection.",
                &[],
                vec![var("since_time", Vr::Duration, false)],
            ),
            line(
                Ro::WhatStillWorks,
                "Bundled help opens normally and shows the last {chip:chip.cached} copy.",
                &["chip.cached"],
                vec![],
            ),
            next_action(
                "action.refresh_docs_pack",
                "Refresh the docs pack",
                vec![],
                link(
                    "link.docs.offline_mode",
                    Lk::DocsTopic,
                    "docs.help.offline_mode",
                    "Open offline docs",
                ),
            ),
            &[Su::DynamicBanner, Su::CliHelpSummary, Su::SupportExport, Su::ScreenReader],
        ),
    ]
}

fn shared_reuse_chip_ids() -> Vec<String> {
    ["chip.policy_blocked", "chip.stale", "chip.cached"]
        .iter()
        .map(|s| (*s).to_owned())
        .collect()
}

fn trust_review() -> RecoveryTrustReview {
    RecoveryTrustReview {
        blocks_explain_failure_cause_remaining_and_next_action: true,
        recovery_messaging_states_what_still_works_and_how_to_proceed: true,
        degraded_state_chips_reused_not_reinvented_per_surface: true,
        chips_use_grounded_cause_language_not_euphemism: true,
        next_action_labels_are_verb_first_with_recovery_link: true,
        machine_tokens_and_ids_stay_locale_neutral: true,
        human_prose_localizes_around_tokens: true,
        support_export_reconstructs_in_product_explanation: true,
        one_catalog_not_parallel_recovery_islands: true,
        recovery_links_resolve_offline: true,
    }
}

fn consumer_projection() -> RecoveryConsumerProjection {
    RecoveryConsumerProjection {
        dynamic_banners_resolve_through_catalog: true,
        inline_blockers_resolve_through_catalog: true,
        project_doctor_reuses_block_identities: true,
        cli_help_summaries_show_same_copy: true,
        support_export_uses_catalog_blocks: true,
        screenshot_captions_reuse_block_copy: true,
        screen_reader_reuses_block_identities: true,
    }
}

fn proof_freshness() -> RecoveryProofFreshness {
    RecoveryProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> RecoveryReleasePosture {
    RecoveryReleasePosture {
        release_packet_ref: "evidence:error-recovery-copy-catalog-release-packet:m5".to_owned(),
        mirror_offline_packet_ref: "evidence:error-recovery-copy-catalog-mirror-offline-packet:m5"
            .to_owned(),
        support_export_parity_required: true,
        mirror_offline_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    [
        ERROR_RECOVERY_COPY_CATALOG_SCHEMA_REF,
        ERROR_RECOVERY_COPY_CATALOG_DOC_REF,
        RECOVERY_UI_COPY_CONTRACT_REF,
        RECOVERY_NAMING_LABEL_CONTRACT_REF,
        RECOVERY_CONTROLLED_GLOSSARY_REF,
        RECOVERY_SAFETY_CRITICAL_SCHEMA_REF,
        RECOVERY_SAFETY_CRITICAL_DOC_REF,
    ]
    .iter()
    .map(|s| (*s).to_owned())
    .collect()
}

fn base_input() -> ErrorRecoveryCopyCatalogInput {
    ErrorRecoveryCopyCatalogInput {
        catalog_id: ERROR_RECOVERY_COPY_CATALOG_ID.to_owned(),
        catalog_label: "Error/Recovery Copy Objects and Degraded-State Reason Chips".to_owned(),
        reference_locale: "en".to_owned(),
        chips: chips(),
        blocks: blocks(),
        shared_reuse_chip_ids: shared_reuse_chip_ids(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Builds the canonical error/recovery copy catalog.
///
/// This is the single producer of the checked-in support export.
pub fn seeded_error_recovery_copy_catalog() -> ErrorRecoveryCopyCatalog {
    ErrorRecoveryCopyCatalog::new(base_input())
}

/// Builds a localized overlay of the canonical catalog.
///
/// Only the human prose of the three copy lines and the recovery-link labels
/// changes: every copy-line template is pseudo-localized while each block id, chip
/// id, machine token, variable name, and `{chip:...}` / `{var:...}` placeholder
/// stays byte-for-byte identical. The verb-first next-action label and its verb
/// stay fixed — they resolve from the controlled verb register, not free prose — so
/// the localized overlay still proves a verb-first action with a recovery link.
pub fn seeded_error_recovery_copy_catalog_localized() -> ErrorRecoveryCopyCatalog {
    let mut input = base_input();
    input.catalog_id = "m5-error-recovery-copy-catalog:localized:0001".to_owned();
    input.catalog_label =
        "Error/Recovery Copy Objects and Degraded-State Reason Chips (localized overlay)"
            .to_owned();
    input.reference_locale = "qps-ploc".to_owned();
    for block in &mut input.blocks {
        block.what_failed.reference_template =
            pseudo_localize_template(&block.what_failed.reference_template);
        block.why_likely.reference_template =
            pseudo_localize_template(&block.why_likely.reference_template);
        block.what_still_works.reference_template =
            pseudo_localize_template(&block.what_still_works.reference_template);
        block.next_action.recovery_link.label =
            pseudo_localize_template(&block.next_action.recovery_link.label);
    }
    ErrorRecoveryCopyCatalog::new(input)
}

/// Builds an offline-mirror variant of the canonical catalog.
///
/// The catalog identity, chips, and blocks are unchanged; only the catalog id and
/// the mirror/offline release refs differ. This proves the catalog survives an
/// offline mirror without forking the meaning of any failure or degraded state.
pub fn seeded_error_recovery_copy_catalog_offline_mirror() -> ErrorRecoveryCopyCatalog {
    let mut input = base_input();
    input.catalog_id = "m5-error-recovery-copy-catalog:offline-mirror:0001".to_owned();
    input.catalog_label =
        "Error/Recovery Copy Objects and Degraded-State Reason Chips (offline mirror)".to_owned();
    input.release_posture.release_packet_ref =
        "evidence:error-recovery-copy-catalog-release-packet:m5:mirror".to_owned();
    ErrorRecoveryCopyCatalog::new(input)
}
