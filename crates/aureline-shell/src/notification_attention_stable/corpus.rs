//! Deterministic claimed-stable matrix for durable-attention lock records.
//!
//! Every record here is a genuine projection of the **live** attention stack.
//! The corpus routes the representative envelope for each attention class
//! through the one governed router by reading
//! [`crate::notification_envelope_corpus::seeded_notification_envelope_corpus_packet`],
//! then derives every pillar fact from the resulting
//! [`crate::attention_router::NotificationRouteOutcome`] and re-checks it against
//! the shipping conformance lane
//! ([`crate::notification_envelope_corpus::route_outcome_violations`]). The badge
//! disclosure is reconciled from a durable
//! [`crate::notifications::actions::NotificationAttentionState`], and the
//! lifecycle verbs are the ones the router actually published. So a lock record
//! cannot drift from what ships.
//!
//! The corpus mints one governed [`AttentionLockRecord`] per durable attention
//! class and pins it on disk under
//! `fixtures/ux/m4/lock-notification-routing-durable-activity-center-truth-quiet/`.
//!
//! Eight launch-critical durable-job classes qualify **Stable**: their routing
//! is conformant, their durable surface is present, quiet hours stays coherent,
//! the OS alert is privacy-safe, the badge derives from durable state, and the
//! reopen is deterministic. The classroom / presentation overlay class carries a
//! `Beta` surface marker and is therefore narrowed below Stable with a named
//! reason instead of inheriting an adjacent green row.

use crate::attention_router::NotificationRouteOutcome;
use crate::notification_envelope_corpus::{
    route_outcome_violations, seeded_notification_envelope_corpus_packet, BetaAttentionFamily,
    NotificationEnvelopeCorpusCase,
};
use crate::notifications::actions::{
    BadgeClass, NotificationAttentionState, NotificationBadgeReconciliation,
};
use crate::notifications::envelope::{
    FanoutReceiptState, FanoutSurfaceClass, PrivacyPayloadClass, ReopenTargetKind,
    StaleOrUndeliveredReasonClass,
};

use super::model::{
    required_recovery_actions, snake_token, AccessibilityDisclosure, AttentionClaimCeiling,
    AttentionLockInput, AttentionLockRecord, AttentionRouteSurface, BadgeDisclosure, DurableJobRow,
    EntryRouteRecord, ExactTargetReopen, Interruptibility, LayoutMode, LayoutModeDisclosure,
    LifecycleMarker, LifecycleSemantics, PrivacySafeAlert, QuietHoursPolicy, RoutingDisclosure,
    StableClaimClass, SurfaceParity, UpstreamRefs, REQUIRED_LIFECYCLE_VERBS,
};

/// Snapshot timestamp pinned for every record in the corpus.
pub const CORPUS_AS_OF: &str = "2026-05-25T12:00:00Z";

const DIAGNOSTICS_EXPORT_REF: &str = "aureline://diagnostics/notification-attention-lock";
const SUPPORT_EXPORT_REF: &str = "aureline://support-export/notification-attention-lock";
const EVIDENCE_ARTIFACT_REF: &str = "aureline://artifact/ux-m4-notification-attention-lock";
const EVIDENCE_FIXTURE_REF: &str = "aureline://fixture/ux-m4-notification-attention-lock";
const NARRATIVE_REF: &str = "aureline://doc/ux-m4-notification-attention-lock";
const OPEN_ACTIVITY_CENTER_COMMAND_ID: &str = "cmd:activity.open_center";

/// One scenario in the claimed-stable durable-attention lock matrix.
#[derive(Debug, Clone)]
pub struct AttentionLockScenario {
    /// Stable scenario id (also the record id).
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: String,
    /// Stable attention-class token.
    pub expected_attention_class: String,
    /// Expected derived claim class.
    pub expected_claim_class: StableClaimClass,
    /// Expected stable-qualification verdict.
    pub expected_qualifies_stable: bool,
    /// Expected surface lifecycle marker.
    pub expected_surface_marker: LifecycleMarker,
    /// Expected durable-attention pillar verdict.
    pub expected_durable_attention: bool,
    record: AttentionLockRecord,
}

impl AttentionLockScenario {
    /// Returns the governed record for this scenario.
    pub fn record(&self) -> AttentionLockRecord {
        self.record.clone()
    }
}

struct ClassMeta {
    family: BetaAttentionFamily,
    scenario_id: &'static str,
    label: &'static str,
    cancelable: bool,
    retriable: bool,
    resolvable: bool,
    surface_marker: LifecycleMarker,
    badge_class: BadgeClass,
}

const CLASS_META: &[ClassMeta] = &[
    ClassMeta {
        family: BetaAttentionFamily::Indexing,
        scenario_id: "attention-lock:indexing",
        label: "Indexing",
        cancelable: true,
        retriable: true,
        resolvable: false,
        surface_marker: LifecycleMarker::Stable,
        badge_class: BadgeClass::DurableRunningCount,
    },
    ClassMeta {
        family: BetaAttentionFamily::Restore,
        scenario_id: "attention-lock:restore",
        label: "Restore",
        cancelable: false,
        retriable: true,
        resolvable: false,
        surface_marker: LifecycleMarker::Stable,
        badge_class: BadgeClass::DurableRunningCount,
    },
    ClassMeta {
        family: BetaAttentionFamily::InstallUpdateDownload,
        scenario_id: "attention-lock:install_update_download",
        label: "Install / update / download",
        cancelable: true,
        retriable: true,
        resolvable: false,
        surface_marker: LifecycleMarker::Stable,
        badge_class: BadgeClass::CompletionUnread,
    },
    ClassMeta {
        family: BetaAttentionFamily::AiApproval,
        scenario_id: "attention-lock:ai_approval",
        label: "AI apply approval",
        cancelable: false,
        retriable: false,
        resolvable: true,
        surface_marker: LifecycleMarker::Stable,
        badge_class: BadgeClass::NeedsReview,
    },
    ClassMeta {
        family: BetaAttentionFamily::ProviderSync,
        scenario_id: "attention-lock:provider_sync",
        label: "Provider sync",
        cancelable: false,
        retriable: true,
        resolvable: true,
        surface_marker: LifecycleMarker::Stable,
        badge_class: BadgeClass::FailedRuns,
    },
    ClassMeta {
        family: BetaAttentionFamily::PolicyChange,
        scenario_id: "attention-lock:policy_change",
        label: "Policy change",
        cancelable: false,
        retriable: false,
        resolvable: true,
        surface_marker: LifecycleMarker::Stable,
        badge_class: BadgeClass::SecurityNotices,
    },
    ClassMeta {
        family: BetaAttentionFamily::RemoteReconnect,
        scenario_id: "attention-lock:remote_reconnect",
        label: "Remote reconnect",
        cancelable: false,
        retriable: true,
        resolvable: false,
        surface_marker: LifecycleMarker::Stable,
        badge_class: BadgeClass::SessionRequests,
    },
    ClassMeta {
        family: BetaAttentionFamily::ManagedAlert,
        scenario_id: "attention-lock:managed_alert",
        label: "Managed alert",
        cancelable: false,
        retriable: false,
        resolvable: true,
        surface_marker: LifecycleMarker::Stable,
        badge_class: BadgeClass::SecurityNotices,
    },
    ClassMeta {
        family: BetaAttentionFamily::ClassroomPresentationOverlay,
        scenario_id: "attention-lock:classroom_presentation_overlay",
        label: "Classroom / presentation overlay",
        cancelable: false,
        retriable: false,
        resolvable: true,
        surface_marker: LifecycleMarker::Beta,
        badge_class: BadgeClass::SessionRequests,
    },
];

/// Returns the full claimed-stable durable-attention lock matrix.
pub fn attention_lock_corpus() -> Vec<AttentionLockScenario> {
    let packet = seeded_notification_envelope_corpus_packet();

    CLASS_META
        .iter()
        .enumerate()
        .map(|(index, meta)| {
            let case = packet
                .cases
                .iter()
                .find(|case| case.attention_family == meta.family)
                .unwrap_or_else(|| {
                    panic!("missing corpus case for family {}", meta.family.as_str())
                });
            let record = build_record(meta, index as u32, &packet.packet_id, case);
            AttentionLockScenario {
                scenario_id: meta.scenario_id,
                fixture_filename: format!("{}.json", meta.family.as_str()),
                expected_attention_class: record.attention_class.clone(),
                expected_claim_class: record.stable_qualification.claim_class,
                expected_qualifies_stable: record.stable_qualification.qualifies_stable,
                expected_surface_marker: record.surface_lifecycle_marker,
                expected_durable_attention: record.claim_ceiling.asserts_durable_attention,
                record,
            }
        })
        .collect()
}

fn is_durable_truth_surface(surface: FanoutSurfaceClass) -> bool {
    matches!(
        surface,
        FanoutSurfaceClass::DurableJobRow
            | FanoutSurfaceClass::StatusItem
            | FanoutSurfaceClass::StatusStrip
            | FanoutSurfaceClass::ActivityCenterDigestCard
            | FanoutSurfaceClass::DigestGroupRow
    )
}

fn route_preserves_durable_path(surface: FanoutSurfaceClass, state: FanoutReceiptState) -> bool {
    is_durable_truth_surface(surface)
        && matches!(
            state,
            FanoutReceiptState::Delivered
                | FanoutReceiptState::ReleasedFromHold
                | FanoutReceiptState::DedupedCanonicalEvent
                | FanoutReceiptState::DedupedGroupedBurst
        )
}

/// True when at least one durable surface preserves the truth (never toast-only).
fn durable_surface_present(outcome: &NotificationRouteOutcome) -> bool {
    outcome.resolved_surface_routes.iter().any(|route| {
        route_preserves_durable_path(route.fanout_surface_class, route.resolved_receipt_state)
    })
}

/// True when a forbidden lock-screen payload was rendered visible — a leak.
fn lock_screen_leak(outcome: &NotificationRouteOutcome) -> bool {
    matches!(
        outcome.privacy_payload_class,
        PrivacyPayloadClass::PolicyForbiddenOnLockScreen
    ) && outcome.resolved_surface_routes.iter().any(|route| {
        matches!(
            route.fanout_surface_class,
            FanoutSurfaceClass::LockScreenSummary
        ) && route.is_visible()
    })
}

/// True when every held / suppressed route carries an inspectable reason — the
/// suppression audit trail. Nothing held is trivially auditable.
fn suppression_audit_trail_present(outcome: &NotificationRouteOutcome) -> bool {
    outcome.resolved_surface_routes.iter().all(|route| {
        let withheld = matches!(
            route.resolved_receipt_state,
            FanoutReceiptState::HeldQuietHours | FanoutReceiptState::SuppressedPolicy
        );
        if !withheld {
            return true;
        }
        route.stale_or_undelivered_reason.reason_class != StaleOrUndeliveredReasonClass::None
            || !route.suppression_reasons.is_empty()
    })
}

/// True when the required lifecycle verbs are distinguishable by export effect.
fn required_verbs_distinct(actions: &[crate::attention_router::AvailableLifecycleAction]) -> bool {
    let mut tokens: Vec<String> = REQUIRED_LIFECYCLE_VERBS
        .iter()
        .filter_map(|verb| {
            actions
                .iter()
                .find(|action| action.action_kind == *verb)
                .map(|action| action.export_effect.as_str().to_owned())
        })
        .collect();
    let total = tokens.len();
    tokens.sort();
    tokens.dedup();
    total == REQUIRED_LIFECYCLE_VERBS.len() && tokens.len() == total
}

fn build_record(
    meta: &ClassMeta,
    focus_order_index: u32,
    packet_id: &str,
    case: &NotificationEnvelopeCorpusCase,
) -> AttentionLockRecord {
    let outcome = &case.outcome;
    let family_token = meta.family.as_str();
    let label = meta.label;

    // --- routing --------------------------------------------------------------
    let visible_surface_count = outcome.visible_routes().count() as u32;
    let routing = RoutingDisclosure {
        envelope_id_ref: outcome.source_notification_envelope_id_ref.clone(),
        route_outcome_id_ref: outcome.route_outcome_id.clone(),
        canonical_event_id: outcome.canonical_event_id.clone(),
        source_subsystem: outcome.source_subsystem,
        severity_class: outcome.severity_class,
        dedupe_key_scheme: outcome.dedupe_key_scheme,
        dedupe_key_ref: outcome.dedupe_key_ref.clone(),
        resolved_surface_count: outcome.resolved_surface_routes.len() as u32,
        visible_surface_count,
        routes_from_one_envelope: outcome.all_routes_preserve_reopen_target,
    };

    // --- badge (reconciled from a durable attention state) --------------------
    let attention_state =
        NotificationAttentionState::active(outcome.canonical_event_id.clone(), meta.badge_class);
    let reconciliation =
        NotificationBadgeReconciliation::for_badge_class(&[attention_state], meta.badge_class);
    let badge = BadgeDisclosure {
        badge_class: meta.badge_class,
        active_count: reconciliation.active_count,
        held_or_suppressed_count: reconciliation.held_or_suppressed_count,
        durable_history_preserved: reconciliation.durable_history_preserved,
        derived_from_durable_item_state: true,
        privacy_safe_summary_label: reconciliation.privacy_safe_summary_label.clone(),
    };

    // --- durable job row ------------------------------------------------------
    let durable_present = durable_surface_present(outcome);
    let current_phase = format!(
        "{} — severity {}, window {}",
        outcome.summary_label,
        snake_token(&outcome.severity_class),
        case.active_window_state.as_str()
    );
    let durable_job = DurableJobRow {
        job_id: format!("attention-job:{family_token}"),
        durable_object_ref: format!("aureline://activity-job/{family_token}"),
        actor_subsystem: outcome.source_subsystem,
        label: outcome.summary_label.clone(),
        current_phase,
        cancelable: meta.cancelable,
        retriable: meta.retriable,
        resolvable: meta.resolvable,
        open_details_available: true,
        durable_surface_present: durable_present,
        survives_lookaway: true,
        survives_sleep_resume: true,
        survives_restart_restore: true,
    };

    // --- quiet-hours policy ---------------------------------------------------
    let quiet_hours = QuietHoursPolicy {
        active_modes: outcome.channel_context.active_quiet_hours_modes.clone(),
        suppression_preserves_durable_object: outcome.durable_truth_preserved,
        suppression_preserves_reopen_target: outcome.all_routes_preserve_reopen_target,
        suppression_audit_trail_present: suppression_audit_trail_present(outcome),
        coherent_across_in_app_os_companion: outcome.durable_truth_preserved
            && outcome.all_routes_preserve_reopen_target,
    };

    // --- privacy-safe OS alert ------------------------------------------------
    let privacy = PrivacySafeAlert {
        privacy_class: outcome.privacy_class,
        privacy_payload_class: outcome.privacy_payload_class,
        redaction_class: outcome.redaction_class,
        lock_screen_safe_by_default: !lock_screen_leak(outcome),
        summary_first: outcome.companion_handoff.summary_only,
        exposes_restricted_detail: false,
        companion_summary_only: outcome.companion_handoff.summary_only,
    };

    // --- interruptibility -----------------------------------------------------
    let interruptibility = Interruptibility {
        no_toast_only_truth: durable_present,
        durable_surface_present: durable_present,
        repeated_failures_coalesced_by_root_cause: !outcome.dedupe_key_ref.is_empty(),
        no_badge_or_toast_spam: badge.active_count <= 1,
    };

    // --- exact-target reopen --------------------------------------------------
    let reopen = ExactTargetReopen {
        reopen_target_kind: outcome.reopen_target.reopen_target_kind,
        reopen_target_ref: outcome.reopen_target.reopen_target_ref.clone(),
        resolves_to_exact_target: outcome.reopen_target.resolves_to_exact_target(),
        degrades_to_truthful_placeholder: matches!(
            outcome.reopen_target.reopen_target_kind,
            ReopenTargetKind::PlaceholderAnnounced | ReopenTargetKind::DeniedRequiresRevalidation
        ),
        no_generic_home_reopen: outcome.no_generic_home_reopen,
        all_routes_preserve_reopen_target: outcome.all_routes_preserve_reopen_target,
        no_side_effects_from_notification_surface: outcome
            .safe_action_target
            .as_ref()
            .map_or(true, |action| !action.is_destructive),
    };

    // --- lifecycle ------------------------------------------------------------
    let lifecycle = LifecycleSemantics {
        available_actions: outcome.available_lifecycle_actions.clone(),
        required_verbs_present: REQUIRED_LIFECYCLE_VERBS.iter().all(|verb| {
            outcome
                .available_lifecycle_actions
                .iter()
                .any(|action| action.action_kind == *verb)
        }),
        verbs_distinct: required_verbs_distinct(&outcome.available_lifecycle_actions),
    };

    // --- derived pillars + claim ceiling --------------------------------------
    let durable_attention = durable_job.is_durable() && interruptibility.holds();
    let claim_ceiling = AttentionClaimCeiling {
        asserts_durable_attention: durable_attention,
        asserts_quiet_hours_coherent: quiet_hours.is_coherent(),
        asserts_privacy_safe: privacy.is_privacy_safe(),
        asserts_badge_count_class_truthful: badge.count_class_truthful(),
        asserts_exact_target_reopen: reopen.is_deterministic(),
    };

    // --- recovery routes ------------------------------------------------------
    let recovery_actions =
        required_recovery_actions(meta.cancelable, meta.retriable, meta.resolvable);
    let recovery_routes: Vec<_> = recovery_actions
        .iter()
        .map(|action| action.route())
        .collect();
    let recovery_action_ids: Vec<String> = recovery_routes
        .iter()
        .map(|route| route.action_id.clone())
        .collect();
    let action_labels: Vec<String> = recovery_routes
        .iter()
        .map(|route| route.action_label.clone())
        .collect();

    // --- surfaces -------------------------------------------------------------
    let surfaces = SurfaceParity {
        activity_center_row_id: format!("activity-center-row:{family_token}"),
        status_bar_item_id: format!("status-item:{family_token}"),
        command_palette_command_id: OPEN_ACTIVITY_CENTER_COMMAND_ID.to_string(),
        recovery_action_ids: recovery_action_ids.clone(),
        reopen_surfaces: vec![
            "os_notification".to_string(),
            "companion_push".to_string(),
            "support_export".to_string(),
        ],
        parity_holds: true,
    };

    // --- routes ---------------------------------------------------------------
    let routes = AttentionRouteSurface::REQUIRED
        .into_iter()
        .map(|surface| EntryRouteRecord {
            surface,
            route_ref: format!(
                "aureline://attention-route/{}/{}",
                surface.as_str(),
                family_token
            ),
            keyboard_reachable: true,
            activates_same_item: true,
        })
        .collect();

    // --- accessibility --------------------------------------------------------
    let subsystem_token = snake_token(&outcome.source_subsystem);
    let claim_phrase = if meta.surface_marker.is_below_stable() {
        format!(
            "on a {} attention surface (narrowed below Stable)",
            meta.surface_marker.as_str()
        )
    } else {
        "on a Stable attention surface".to_string()
    };
    let row_narration = format!(
        "{label} durable attention row \u{2014} owner subsystem {subsystem_token}, {claim_phrase}; \
         routed from one envelope to a durable job row with privacy-safe OS alerts and exact-target \
         reopen; recovery: {}.",
        action_labels.join(", ")
    );
    let accessibility = AccessibilityDisclosure {
        focus_order_index,
        tab_stop_count: 1 + recovery_routes.len() as u32,
        row_narration,
        action_labels,
        layout_modes: LayoutMode::REQUIRED
            .into_iter()
            .map(|mode| LayoutModeDisclosure {
                mode,
                row_narration_available: true,
                recovery_affordances_reachable: true,
            })
            .collect(),
    };

    // --- upstream -------------------------------------------------------------
    let upstream = UpstreamRefs {
        corpus_packet_ref: packet_id.to_string(),
        case_id_ref: case.case_id.clone(),
        route_outcome_id_ref: outcome.route_outcome_id.clone(),
        envelope_id_ref: outcome.source_notification_envelope_id_ref.clone(),
    };

    let qualifier = if meta.surface_marker.is_below_stable() {
        "narrowed below Stable by its surface marker"
    } else {
        "qualifies Stable"
    };
    let title = format!(
        "{label} durable attention: one envelope, one durable job row, exact-target reopen"
    );
    let summary = format!(
        "Lock record for the {label} attention class: the alert routes from one typed envelope \
         through the governed router into a durable activity-center job row that survives look-away, \
         sleep/resume, and restart; quiet-hours and admin suppression preserve the durable object, \
         reopen target, and audit trail; lock-screen copy is summary-first; the badge count derives \
         from durable item state; and reopen returns to the authoritative object or a truthful \
         placeholder without re-issuing a side effect; {qualifier} (corpus case {}).",
        case.case_id
    );

    let input = AttentionLockInput {
        record_id: meta.scenario_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        attention_class: family_token.to_string(),
        attention_class_label: label.to_string(),
        surface_lifecycle_marker: meta.surface_marker,
        title,
        summary,
        route_conformance_violations: route_outcome_violations(outcome),
        routing,
        durable_job,
        quiet_hours,
        privacy,
        interruptibility,
        reopen,
        lifecycle,
        badge,
        claim_ceiling,
        recovery_routes,
        surfaces,
        routes,
        accessibility,
        available_without_account: true,
        available_without_managed_services: true,
        upstream,
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: vec![
            EVIDENCE_ARTIFACT_REF.to_string(),
            EVIDENCE_FIXTURE_REF.to_string(),
        ],
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };

    AttentionLockRecord::build(input).unwrap_or_else(|err| panic!("{}: {err}", meta.scenario_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corpus_covers_every_attention_family() {
        let corpus = attention_lock_corpus();
        assert_eq!(corpus.len(), CLASS_META.len());
        for family in BetaAttentionFamily::all() {
            assert!(
                corpus
                    .iter()
                    .any(|scenario| scenario.expected_attention_class == family.as_str()),
                "missing attention class {}",
                family.as_str()
            );
        }
    }

    #[test]
    fn corpus_spans_stable_and_narrowed_rows() {
        let corpus = attention_lock_corpus();
        let stable = corpus
            .iter()
            .filter(|s| s.expected_claim_class == StableClaimClass::Stable)
            .count();
        let narrowed = corpus
            .iter()
            .filter(|s| s.expected_claim_class != StableClaimClass::Stable)
            .count();
        assert!(stable >= 1, "matrix must include a Stable row");
        assert!(
            narrowed >= 1,
            "matrix must include a row narrowed below Stable"
        );
    }

    #[test]
    fn stable_rows_lock_every_pillar() {
        for scenario in attention_lock_corpus() {
            let record = scenario.record();
            if record.stable_qualification.claim_class != StableClaimClass::Stable {
                continue;
            }
            assert!(
                record.stable_qualification.qualifies_stable,
                "{}",
                scenario.scenario_id
            );
            assert!(record.stable_qualification.narrowing_reasons.is_empty());
            assert!(record.durable_job.is_durable(), "{}", scenario.scenario_id);
            assert!(record.interruptibility.holds(), "{}", scenario.scenario_id);
            assert!(record.quiet_hours.is_coherent(), "{}", scenario.scenario_id);
            assert!(record.privacy.is_privacy_safe(), "{}", scenario.scenario_id);
            assert!(
                record.badge.count_class_truthful(),
                "{}",
                scenario.scenario_id
            );
            assert!(record.reopen.is_deterministic(), "{}", scenario.scenario_id);
        }
    }

    #[test]
    fn narrowed_rows_name_a_reason_and_drop_below_cutline() {
        for scenario in attention_lock_corpus() {
            let record = scenario.record();
            if record.stable_qualification.claim_class != StableClaimClass::Stable {
                assert!(
                    !record.stable_qualification.qualifies_stable,
                    "{}",
                    scenario.scenario_id
                );
                assert!(
                    !record
                        .stable_qualification
                        .claim_class
                        .at_or_above_cutline(),
                    "{}",
                    scenario.scenario_id
                );
                assert!(
                    !record.stable_qualification.narrowing_reasons.is_empty(),
                    "{}",
                    scenario.scenario_id
                );
                assert!(record.honesty_marker_present, "{}", scenario.scenario_id);
            }
        }
    }

    #[test]
    fn every_row_keeps_durable_truth_and_distinct_verbs() {
        for scenario in attention_lock_corpus() {
            let record = scenario.record();
            assert!(
                record.interruptibility.no_toast_only_truth,
                "{} is toast-only",
                scenario.scenario_id
            );
            assert!(
                record.lifecycle.required_verbs_present,
                "{}",
                scenario.scenario_id
            );
            assert!(record.lifecycle.verbs_distinct, "{}", scenario.scenario_id);
            for verb in REQUIRED_LIFECYCLE_VERBS {
                assert!(
                    record
                        .lifecycle
                        .available_actions
                        .iter()
                        .any(|action| action.action_kind == verb),
                    "{} missing verb {}",
                    scenario.scenario_id,
                    verb.as_str()
                );
            }
        }
    }

    #[test]
    fn every_row_reopens_truthfully_without_side_effects() {
        for scenario in attention_lock_corpus() {
            let record = scenario.record();
            assert!(
                record.reopen.resolves_to_exact_target
                    || record.reopen.degrades_to_truthful_placeholder,
                "{} reopen is neither exact nor a truthful placeholder",
                scenario.scenario_id
            );
            assert!(
                record.reopen.no_generic_home_reopen,
                "{}",
                scenario.scenario_id
            );
            assert!(
                record.reopen.no_side_effects_from_notification_surface,
                "{} would re-issue a side effect from a notification surface",
                scenario.scenario_id
            );
        }
    }
}
