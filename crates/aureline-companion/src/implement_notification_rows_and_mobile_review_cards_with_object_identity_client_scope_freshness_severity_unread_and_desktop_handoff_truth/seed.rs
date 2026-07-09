//! Canonical seed builders for the notification-row / mobile-review-card controls.
//!
//! These builders are the single producer of the checked-in support export and the
//! scenario fixtures. The headless emitter and the inline tests both call them so the
//! in-code controls, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical notification-row / mobile-review-card packet.
pub const NOTIFICATION_ROW_MOBILE_REVIEW_CARD_PACKET_ID: &str =
    "m5-notification-row-mobile-review-card-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn row_source_refs() -> Vec<String> {
    strings(&[
        M5_COMPANION_NOTIFICATION_ROW_SCHEMA_REF,
        M5_COMPANION_COMPONENT_FOUNDATION_TRIAGE_REF,
    ])
}

fn card_source_refs() -> Vec<String> {
    strings(&[
        M5_MOBILE_REVIEW_CARD_SCHEMA_REF,
        M5_COMPANION_COMPONENT_FOUNDATION_TRIAGE_REF,
    ])
}

/// Builds a notification row, deriving the delivery class, the live claim, and the
/// required notes from the honest inputs so the seed is always self-consistent with
/// the resolver.
#[allow(clippy::too_many_arguments)]
fn notification_row(
    row_id: &str,
    event_label: &str,
    object_kind: M5CompanionObjectKind,
    object_label: &str,
    object_landing_ref: &str,
    client_scope: M5CompanionClientScope,
    scope_label: &str,
    severity: M5CompanionSeverity,
    notification_category: M5CompanionNotificationCategory,
    is_unread: bool,
    freshness: M5CompanionFreshness,
    handoff_target: M5CompanionHandoffTarget,
    handoff_label: &str,
    triage_verbs: Vec<NotificationTriageVerb>,
) -> NotificationRow {
    let disclosure = resolve_notification_delivery(freshness);
    NotificationRow {
        component: M5CompanionComponentFamily::NotificationRow,
        row_id: row_id.to_owned(),
        event_label: event_label.to_owned(),
        object_kind,
        object_label: object_label.to_owned(),
        object_landing_ref: object_landing_ref.to_owned(),
        client_scope,
        scope_label: scope_label.to_owned(),
        severity,
        severity_label: format!("Severity {}", severity.as_str()),
        notification_category,
        is_unread,
        freshness,
        delivery_class: disclosure.delivery_class,
        claims_live: disclosure.is_live,
        cached_note: if disclosure.needs_cached_note {
            "Showing a last-known cached value, not a live update".to_owned()
        } else {
            String::new()
        },
        stale_note: if disclosure.needs_stale_note {
            format!(
                "Freshness {}: shown stale, not live — refresh before acting",
                freshness.as_str()
            )
        } else {
            String::new()
        },
        unknown_note: if disclosure.needs_unknown_note {
            "Freshness could not be determined; not shown as live".to_owned()
        } else {
            String::new()
        },
        scope_and_freshness_note: format!(
            "Scoped to {}; freshness {}",
            client_scope.as_str(),
            freshness.as_str()
        ),
        handoff_target,
        handoff_label: handoff_label.to_owned(),
        triage_verbs,
        degraded_reasons: M5CompanionDegradedReason::ALL.to_vec(),
        required_labels: M5CompanionRequiredLabel::ALL.to_vec(),
        surface_families: M5CompanionSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5CompanionDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5CompanionAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5CompanionConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "event_label",
            "object_label",
            "client_scope",
            "severity",
            "notification_category",
            "is_unread",
            "freshness",
            "handoff_target",
        ]),
        source_contract_refs: row_source_refs(),
        masks_scope_or_freshness: false,
        hides_capability_boundary: false,
        invents_alternate_state_label: false,
        implies_desktop_action_is_companion_safe: false,
        routes_to_generic_activity_page: false,
    }
}

/// Builds a mobile review card, deriving the capability class, the companion-sufficient
/// claim, and the required notes from the honest inputs so the seed is always
/// self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn review_card(
    card_id: &str,
    review_label: &str,
    object_kind: M5CompanionObjectKind,
    object_label: &str,
    object_landing_ref: &str,
    client_scope: M5CompanionClientScope,
    scope_label: &str,
    review_kind: M5CompanionReviewKind,
    freshness: M5CompanionFreshness,
    is_unread: bool,
    disposition: M5CompanionComponentDisposition,
    handoff_target: M5CompanionHandoffTarget,
    handoff_label: &str,
    review_verbs: Vec<MobileReviewVerb>,
) -> MobileReviewCard {
    let disclosure = resolve_review_capability(disposition);
    MobileReviewCard {
        component: M5CompanionComponentFamily::MobileReviewCard,
        card_id: card_id.to_owned(),
        review_label: review_label.to_owned(),
        object_kind,
        object_label: object_label.to_owned(),
        object_landing_ref: object_landing_ref.to_owned(),
        client_scope,
        scope_label: scope_label.to_owned(),
        review_kind,
        review_kind_label: format!("Review kind {}", review_kind.as_str()),
        freshness,
        is_unread,
        disposition,
        capability_class: disclosure.capability_class,
        claims_companion_sufficient: disclosure.companion_execution_sufficient,
        capability_note: format!(
            "Companion capability {} (companion execution sufficient: {})",
            disclosure.capability_class.as_str(),
            disclosure.companion_execution_sufficient
        ),
        desktop_required_note: if disclosure.needs_desktop_required_note {
            "This action must complete on desktop; the companion cannot finish it".to_owned()
        } else {
            String::new()
        },
        policy_blocked_note: if disclosure.needs_policy_blocked_note {
            "This action is blocked by policy on the companion".to_owned()
        } else {
            String::new()
        },
        scope_and_freshness_note: format!(
            "Scoped to {}; freshness {}",
            client_scope.as_str(),
            freshness.as_str()
        ),
        handoff_target,
        handoff_label: handoff_label.to_owned(),
        review_verbs,
        degraded_reasons: M5CompanionDegradedReason::ALL.to_vec(),
        required_labels: M5CompanionRequiredLabel::ALL.to_vec(),
        surface_families: M5CompanionSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5CompanionDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5CompanionAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5CompanionConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "review_label",
            "object_label",
            "client_scope",
            "review_kind",
            "disposition",
            "is_unread",
            "freshness",
            "handoff_target",
        ]),
        source_contract_refs: card_source_refs(),
        masks_scope_or_freshness: false,
        hides_capability_boundary: false,
        invents_alternate_state_label: false,
        implies_desktop_action_is_companion_safe: false,
        routes_to_generic_activity_page: false,
    }
}

fn notification_rows() -> Vec<NotificationRow> {
    use M5CompanionClientScope as Scope;
    use M5CompanionFreshness as Fresh;
    use M5CompanionHandoffTarget as Handoff;
    use M5CompanionNotificationCategory as Category;
    use M5CompanionObjectKind as Object;
    use M5CompanionSeverity as Severity;
    use NotificationTriageVerb as Verb;

    vec![
        // 1. Critical build failure, live: opens the exact CI run, hands off to the pipeline.
        notification_row(
            "notif-build-critical",
            "Release build failed",
            Object::CiRun,
            "CI run #4821",
            "ci_run:pipeline-4821",
            Scope::RepoScoped,
            "Repository aureline/aureline",
            Severity::Critical,
            Category::Build,
            true,
            Fresh::Live,
            Handoff::CiPipelineRun,
            "Open CI run #4821 on desktop",
            vec![
                Verb::Open,
                Verb::Acknowledge,
                Verb::MarkRead,
                Verb::HandoffToDesktop,
            ],
        ),
        // 2. High-severity incident, stale: opens the exact incident, hands off to workspace.
        notification_row(
            "notif-incident-high",
            "Incident opened",
            Object::IncidentRecord,
            "Incident INC-204",
            "incident_record:inc-204",
            Scope::WorkspaceScoped,
            "Workspace platform",
            Severity::High,
            Category::Incident,
            true,
            Fresh::Stale,
            Handoff::IncidentWorkspace,
            "Open incident INC-204 workspace on desktop",
            vec![Verb::Open, Verb::Acknowledge, Verb::HandoffToDesktop],
        ),
        // 3. Moderate review request, cached: opens the exact change set, hands off to review.
        notification_row(
            "notif-review-moderate",
            "Review requested",
            Object::ReviewItem,
            "Change set CS-88",
            "review_item:cs-88",
            Scope::RepoScoped,
            "Repository aureline/aureline",
            Severity::Moderate,
            Category::Review,
            false,
            Fresh::Cached,
            Handoff::ReviewPanel,
            "Open change set CS-88 review panel on desktop",
            vec![Verb::Open, Verb::MarkRead, Verb::HandoffToDesktop],
        ),
        // 4. Low-severity agent run, offline-held (stale): opens the exact session.
        notification_row(
            "notif-agent-low",
            "Agent run finished",
            Object::FollowedSession,
            "Session sess-51",
            "followed_session:sess-51",
            Scope::DeviceScoped,
            "This device",
            Severity::Low,
            Category::Agent,
            true,
            Fresh::OfflineHeld,
            Handoff::AgentSession,
            "Open agent session sess-51 on desktop",
            vec![Verb::Open, Verb::Mute, Verb::HandoffToDesktop],
        ),
        // 5. Informational sync event, unknown freshness: no desktop handoff, so no
        //    handoff verb is offered — the row never invents a target it cannot resolve.
        notification_row(
            "notif-sync-info",
            "Sync completed",
            Object::NotificationEvent,
            "Sync event evt-12",
            "notification_event:evt-12",
            Scope::OrgScoped,
            "Organization aureline",
            Severity::Informational,
            Category::Sync,
            false,
            Fresh::UnknownFreshness,
            Handoff::NoHandoff,
            "No desktop handoff for this sync event",
            vec![Verb::Open, Verb::MarkRead, Verb::Snooze],
        ),
        // 6. Unspecified mention, expired snapshot (stale): opens the exact file location.
        notification_row(
            "notif-mention-unspec",
            "You were mentioned",
            Object::NotificationEvent,
            "Mention msg-9",
            "notification_event:msg-9",
            Scope::AccountGlobal,
            "Account-wide",
            Severity::Unspecified,
            Category::Mention,
            false,
            Fresh::ExpiredSnapshot,
            Handoff::FileLocation,
            "Open the referenced file location on desktop",
            vec![Verb::Open, Verb::MarkRead, Verb::HandoffToDesktop],
        ),
    ]
}

fn review_cards() -> Vec<MobileReviewCard> {
    use M5CompanionClientScope as Scope;
    use M5CompanionComponentDisposition as Disp;
    use M5CompanionFreshness as Fresh;
    use M5CompanionHandoffTarget as Handoff;
    use M5CompanionObjectKind as Object;
    use M5CompanionReviewKind as Kind;
    use MobileReviewVerb as Verb;

    vec![
        // 1. Agent change, comment-capable: companion can comment inline.
        review_card(
            "review-agent-change",
            "Agent change ready",
            Object::ReviewItem,
            "Change CS-90",
            "review_item:cs-90",
            Scope::RepoScoped,
            "Repository aureline/aureline",
            Kind::AgentChange,
            Fresh::Live,
            true,
            Disp::CommentCapable,
            Handoff::ReviewPanel,
            "Open change CS-90 review panel on desktop",
            vec![Verb::Open, Verb::Comment, Verb::HandoffToDesktop],
        ),
        // 2. Diff review, review-only: companion can view, not act.
        review_card(
            "review-diff",
            "Diff to review",
            Object::ReviewItem,
            "Diff D-14",
            "review_item:diff-14",
            Scope::RepoScoped,
            "Repository aureline/aureline",
            Kind::DiffReview,
            Fresh::Cached,
            false,
            Disp::ReviewOnly,
            Handoff::ReviewPanel,
            "Open diff D-14 review panel on desktop",
            vec![Verb::Open, Verb::HandoffToDesktop],
        ),
        // 3. Comment thread, comment-capable: companion can post a bounded comment.
        review_card(
            "review-comment-thread",
            "Comment thread awaiting response",
            Object::ReviewItem,
            "Thread T-3",
            "review_item:thread-3",
            Scope::WorkspaceScoped,
            "Workspace platform",
            Kind::CommentThread,
            Fresh::Live,
            true,
            Disp::CommentCapable,
            Handoff::ReviewPanel,
            "Open thread T-3 review panel on desktop",
            vec![Verb::Open, Verb::Comment, Verb::HandoffToDesktop],
        ),
        // 4. Approval request, desktop-required: approving must complete on desktop, so
        //    no inline approve verb is offered and the desktop-required note is set.
        review_card(
            "review-approval",
            "Approval requested",
            Object::ReviewItem,
            "PR-77",
            "review_item:pr-77",
            Scope::RepoScoped,
            "Repository aureline/aureline",
            Kind::ApprovalRequest,
            Fresh::Live,
            true,
            Disp::DesktopRequired,
            Handoff::ReviewPanel,
            "Open PR-77 review panel on desktop to approve",
            vec![Verb::Open, Verb::HandoffToDesktop],
        ),
        // 5. Policy gate, policy-blocked: the action is blocked on the companion.
        review_card(
            "review-policy-gate",
            "Policy gate awaiting acknowledgement",
            Object::ReviewItem,
            "Gate G-2",
            "review_item:gate-2",
            Scope::OrgScoped,
            "Organization aureline",
            Kind::PolicyGate,
            Fresh::Stale,
            false,
            Disp::PolicyBlocked,
            Handoff::ReviewPanel,
            "Open policy gate G-2 review panel on desktop",
            vec![Verb::Open, Verb::HandoffToDesktop],
        ),
        // 6. Merge readiness, review-only: companion can view the summary.
        review_card(
            "review-merge-readiness",
            "Merge readiness summary",
            Object::ReviewItem,
            "PR-80",
            "review_item:pr-80",
            Scope::RepoScoped,
            "Repository aureline/aureline",
            Kind::MergeReadiness,
            Fresh::Cached,
            false,
            Disp::ReviewOnly,
            Handoff::ReviewPanel,
            "Open PR-80 review panel on desktop",
            vec![Verb::Open, Verb::HandoffToDesktop],
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5CompanionDowngradeTrigger> {
    vec![
        M5CompanionDowngradeTrigger::ObjectIdentityUnstated,
        M5CompanionDowngradeTrigger::ClientScopeUnstated,
        M5CompanionDowngradeTrigger::FreshnessHidden,
        M5CompanionDowngradeTrigger::CapabilityBoundaryUnstated,
        M5CompanionDowngradeTrigger::SeverityUnstated,
        M5CompanionDowngradeTrigger::HandoffTargetUnresolved,
        M5CompanionDowngradeTrigger::DispositionUnstated,
        M5CompanionDowngradeTrigger::AlternateStateLabelInvented,
        M5CompanionDowngradeTrigger::StaleShownAsLive,
        M5CompanionDowngradeTrigger::DesktopRequiredActionOfferedInline,
        M5CompanionDowngradeTrigger::ProofStale,
    ]
}

fn glance_review() -> NotificationRowMobileReviewCardGlanceReview {
    NotificationRowMobileReviewCardGlanceReview {
        notification_row_shows_object_and_severity: true,
        notification_row_shows_scope_and_freshness: true,
        notification_row_shows_unread_state: true,
        review_card_shows_capability_boundary: true,
        review_card_states_companion_sufficiency: true,
        review_card_shows_review_kind: true,
        object_identity_always_explicit: true,
        client_scope_always_explicit: true,
        freshness_always_explicit: true,
        delivery_and_capability_derived_never_asserted: true,
        stale_never_shown_as_live: true,
        every_verb_traces_to_one_object: true,
        every_handoff_names_exact_target: true,
        no_surface_invents_alternate_state_label: true,
        controls_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> NotificationRowMobileReviewCardConsumerProjection {
    NotificationRowMobileReviewCardConsumerProjection {
        notification_triage_ui_reads_single_source: true,
        review_queue_ui_reads_single_source: true,
        first_glance_names_object_scope_and_freshness: true,
        capability_boundary_visible_before_tap: true,
        support_export_shows_control_truth: true,
        help_about_shows_control_truth: true,
    }
}

fn proof_freshness() -> NotificationRowMobileReviewCardProofFreshness {
    NotificationRowMobileReviewCardProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        NOTIFICATION_ROW_MOBILE_REVIEW_CARD_SCHEMA_REF,
        NOTIFICATION_ROW_MOBILE_REVIEW_CARD_DOC_REF,
        M5_COMPANION_COMPONENT_SCHEMA_REF,
        M5_COMPANION_COMPONENT_DOC_REF,
        M5_COMPANION_NOTIFICATION_ROW_SCHEMA_REF,
        M5_MOBILE_REVIEW_CARD_SCHEMA_REF,
    ])
}

/// Builds the canonical notification-row / mobile-review-card controls packet.
pub fn seeded_notification_row_mobile_review_card_controls(
) -> NotificationRowMobileReviewCardControlsPacket {
    NotificationRowMobileReviewCardControlsPacket::new(
        NotificationRowMobileReviewCardControlsPacketInput {
            packet_id: NOTIFICATION_ROW_MOBILE_REVIEW_CARD_PACKET_ID.to_owned(),
            surface_label:
                "M5 notification rows and mobile review cards: event/object identity, repo/workspace client scope, freshness, severity/importance, unread state, keyboard-complete quick triage verbs, companion-versus-desktop capability boundary, and an exact desktop-handoff target"
                    .to_owned(),
            notification_rows: notification_rows(),
            review_cards: review_cards(),
            downgrade_triggers: downgrade_triggers(),
            consumer_surfaces: M5CompanionConsumerSurface::ALL.to_vec(),
            glance_review: glance_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Scenario fixture: spotlights a stale notification row that must never read as live.
/// Every delivery class and severity stays covered so the fixture validates on its own.
pub fn seeded_notification_row_mobile_review_card_controls_notification_row_stale(
) -> NotificationRowMobileReviewCardControlsPacket {
    let mut packet = seeded_notification_row_mobile_review_card_controls();
    packet.packet_id =
        "m5-notification-row-mobile-review-card-controls:fixture:notification-row-stale".to_owned();
    packet.surface_label =
        "M5 notification rows: a stale notification never reads as live".to_owned();
    packet
}

/// Scenario fixture: spotlights a desktop-required review card that must never read as
/// companion-completable. Every capability class and review kind stays covered so the
/// fixture validates on its own.
pub fn seeded_notification_row_mobile_review_card_controls_mobile_review_card_desktop_required(
) -> NotificationRowMobileReviewCardControlsPacket {
    let mut packet = seeded_notification_row_mobile_review_card_controls();
    packet.packet_id =
        "m5-notification-row-mobile-review-card-controls:fixture:mobile-review-card-desktop-required"
            .to_owned();
    packet.surface_label =
        "M5 mobile review cards: a desktop-required review never reads as companion-safe"
            .to_owned();
    packet
}
