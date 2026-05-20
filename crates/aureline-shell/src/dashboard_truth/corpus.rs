//! Cross-surface dashboard & queue truth drill corpus.
//!
//! ## Why a corpus, not a single seeded view
//!
//! The model in [`crate::dashboard_truth::model`] proves the no-silent-green
//! invariant in isolation, but the beta-claim grade is about the four queue
//! surfaces and the service-health dashboard *agreeing* on freshness, order
//! reason, and hidden-scope state for the same object across desktop, CLI /
//! headless inspect, diagnostics, and support exports. This corpus mints one
//! [`DashboardTruthScenario`] per named drill and pins each rendered
//! [`DashboardTruthView`] bit-for-bit on disk under
//! `fixtures/ops/m3/dashboard_and_queue_truth/`, so a regression in the
//! downgrade rule, the order-reason vocabulary, the hidden-scope counters, or
//! the canonical-object routing fails the fixture-replay test instead of
//! shipping silently.
//!
//! The drills deliberately exercise:
//!
//! - every [`FreshnessClass`] (`fresh`, `cached`, `stale`, `partial`,
//!   `policy_blocked`, `unavailable`);
//! - every [`DowngradeReasonClass`];
//! - every [`OrderReasonClass`] and every [`NarrowingReasonClass`];
//! - every [`EvidenceKindClass`]; and
//! - all five [`DashboardSurfaceClass`] surfaces.

use super::model::{
    DashboardSurfaceClass, DashboardTruthView, DisplayedStateClass, EffectiveStateClass,
    EvidenceKindClass, FreshnessCardInput, FreshnessClass, HiddenScopeInput, NarrowingReasonClass,
    OrderReasonClass, QueueOrderInput, QueueRowInput,
};

/// Stable `as_of` instant the whole corpus is evaluated against. Pinned so the
/// on-disk fixtures stay deterministic.
pub const CORPUS_AS_OF: &str = "2026-05-20T12:00";

/// Stable view-id prefix shared by every scenario.
pub const CORPUS_VIEW_ID_PREFIX: &str = "dashboard_truth_view:m3.beta.corpus.";

/// One drill. Surfaces under review MUST reproduce the same view projection
/// bit-for-bit; the test in
/// `crates/aureline-shell/tests/dashboard_truth_fixtures.rs` pins each scenario
/// against the on-disk fixture under
/// `fixtures/ops/m3/dashboard_and_queue_truth/`.
#[derive(Clone)]
pub struct DashboardTruthScenario {
    /// Stable identifier, quoted in the matrix, the report, and the doc.
    pub scenario_id: &'static str,
    /// Stable human-readable label.
    pub scenario_label: &'static str,
    /// The surface the drill exercises.
    pub surface: DashboardSurfaceClass,
    /// One-sentence narrative the report and matrix quote.
    pub narrative: &'static str,
    /// On-disk fixture filename (relative to the corpus fixture dir).
    pub fixture_filename: &'static str,
    /// Expected `overall_effective_state` the surface MUST land on.
    pub expected_overall_effective_state: EffectiveStateClass,
    /// Expected `overall_freshness` the surface MUST land on.
    pub expected_overall_freshness: FreshnessClass,
    /// Expected `honesty_marker_present` value.
    pub expected_honesty_marker_present: bool,
    /// Expected count of cards withdrawn from a green claim.
    pub expected_green_downgrade_count: u32,
    /// Expected count of rows hidden by scope (0 for non-queue surfaces).
    pub expected_hidden_total: u32,
    view_id: String,
    cards: Vec<FreshnessCardInput>,
    queue: Option<QueueOrderInput>,
}

impl DashboardTruthScenario {
    /// Build the rendered view for this scenario. The corpus inputs are
    /// deterministic and validated, so a build failure is a bug.
    pub fn view(&self) -> DashboardTruthView {
        DashboardTruthView::build(
            self.view_id.clone(),
            self.surface,
            CORPUS_AS_OF,
            self.cards.clone(),
            self.queue.clone(),
        )
        .expect("dashboard-truth corpus scenario must build")
    }
}

#[allow(clippy::too_many_arguments)]
fn card(
    id: &str,
    title: &str,
    displayed: DisplayedStateClass,
    freshness: FreshnessClass,
    evidence_at: Option<&str>,
    kind: EvidenceKindClass,
    reference: &str,
    explanation: &str,
) -> FreshnessCardInput {
    FreshnessCardInput {
        card_id: id.to_owned(),
        title: title.to_owned(),
        displayed_state: displayed,
        freshness,
        last_successful_evidence_at: evidence_at.map(str::to_owned),
        evidence_kind: kind,
        evidence_ref: reference.to_owned(),
        state_explanation: explanation.to_owned(),
    }
}

fn row(
    row_id: &str,
    reason: OrderReasonClass,
    explanation: &str,
    open_details_ref: &str,
) -> QueueRowInput {
    QueueRowInput {
        row_id: row_id.to_owned(),
        order_reason: reason,
        order_explanation: explanation.to_owned(),
        open_details_ref: open_details_ref.to_owned(),
    }
}

fn hidden(
    reason: NarrowingReasonClass,
    count: u32,
    explanation: &str,
    reveal_ref: &str,
) -> HiddenScopeInput {
    HiddenScopeInput {
        narrowing_reason: reason,
        hidden_count: count,
        narrowing_explanation: explanation.to_owned(),
        reveal_ref: reveal_ref.to_owned(),
    }
}

fn view_id(suffix: &str) -> String {
    format!("{CORPUS_VIEW_ID_PREFIX}{suffix}")
}

// Timestamps relative to CORPUS_AS_OF (2026-05-20T12:00).
const FRESH: &str = "2026-05-20T11:58"; // 2 min  -> Fresh
const RECENT: &str = "2026-05-20T11:20"; // 40 min -> Recent
const HOURS_AGO: &str = "2026-05-20T06:00"; // 6 h  -> Stale
const DAY_AGO: &str = "2026-05-19T06:00"; // >24 h  -> VeryStale

/// The full ordered corpus.
pub fn dashboard_truth_corpus() -> Vec<DashboardTruthScenario> {
    vec![
        service_health_all_clear(),
        service_health_stale_green(),
        service_health_partial_offline(),
        review_inbox_ordered_narrowed(),
        incident_queue_severity_sla(),
        support_queue_policy_scoped(),
        admin_queue_offline_partial(),
    ]
}

fn service_health_all_clear() -> DashboardTruthScenario {
    DashboardTruthScenario {
        scenario_id: "service_health_all_clear",
        scenario_label: "Service health — all clear and current",
        surface: DashboardSurfaceClass::ServiceHealth,
        narrative: "Every service family probed within its review window with current evidence; \
                    the dashboard is allowed to render an all-clear headline with no honesty chip.",
        fixture_filename: "service_health_all_clear.json",
        expected_overall_effective_state: EffectiveStateClass::Clear,
        expected_overall_freshness: FreshnessClass::Fresh,
        expected_honesty_marker_present: false,
        expected_green_downgrade_count: 0,
        expected_hidden_total: 0,
        view_id: view_id("service_health_all_clear"),
        cards: vec![
            card(
                "card:language_services",
                "Language services",
                DisplayedStateClass::Clear,
                FreshnessClass::Fresh,
                Some(FRESH),
                EvidenceKindClass::ServiceHealthCard,
                "aureline://service_health_card/language_services",
                "Indexer and language servers responded within the review window.",
            ),
            card(
                "card:ai_assist",
                "AI assist",
                DisplayedStateClass::Clear,
                FreshnessClass::Fresh,
                Some(FRESH),
                EvidenceKindClass::ServiceHealthCard,
                "aureline://service_health_card/ai_assist",
                "Provider health probe returned current and within contract.",
            ),
            card(
                "card:sync",
                "Workspace sync",
                DisplayedStateClass::Clear,
                FreshnessClass::Fresh,
                Some(RECENT),
                EvidenceKindClass::ServiceHealthCard,
                "aureline://service_health_card/sync",
                "Last sync probe succeeded within the hour with no pending push backlog.",
            ),
        ],
        queue: None,
    }
}

fn service_health_stale_green() -> DashboardTruthScenario {
    DashboardTruthScenario {
        scenario_id: "service_health_stale_green",
        scenario_label: "Service health — stale green downgrade",
        surface: DashboardSurfaceClass::ServiceHealth,
        narrative: "A release-channel card whose probe expired and a docs card serving cached data \
                    cannot keep a green headline; both downgrade to unconfirmed and name why, while \
                    a current language card stays clear.",
        fixture_filename: "service_health_stale_green.json",
        expected_overall_effective_state: EffectiveStateClass::Unconfirmed,
        expected_overall_freshness: FreshnessClass::Stale,
        expected_honesty_marker_present: true,
        expected_green_downgrade_count: 2,
        expected_hidden_total: 0,
        view_id: view_id("service_health_stale_green"),
        cards: vec![
            card(
                "card:release_channel",
                "Release channel",
                DisplayedStateClass::Clear,
                FreshnessClass::Stale,
                Some(HOURS_AGO),
                EvidenceKindClass::ServiceHealthCard,
                "aureline://service_health_card/release_channel",
                "Last successful claim-manifest fetch is past its review window; held as stale.",
            ),
            card(
                "card:docs_knowledge",
                "Docs & knowledge",
                DisplayedStateClass::Clear,
                FreshnessClass::Cached,
                Some(RECENT),
                EvidenceKindClass::ServiceHealthCard,
                "aureline://service_health_card/docs_knowledge",
                "Serving the cached docs mirror; no fresh upstream probe in hand.",
            ),
            card(
                "card:language_services",
                "Language services",
                DisplayedStateClass::Clear,
                FreshnessClass::Fresh,
                Some(FRESH),
                EvidenceKindClass::ServiceHealthCard,
                "aureline://service_health_card/language_services",
                "Indexer and language servers responded within the review window.",
            ),
        ],
        queue: None,
    }
}

fn service_health_partial_offline() -> DashboardTruthScenario {
    DashboardTruthScenario {
        scenario_id: "service_health_partial_offline",
        scenario_label: "Service health — partial, offline, and policy-blocked",
        surface: DashboardSurfaceClass::ServiceHealth,
        narrative: "Sync reports a partial read, a hosted marketplace is offline (a green that \
                    downgrades to unconfirmed), and AI assist is policy-blocked; each names a \
                    distinct downgrade reason.",
        fixture_filename: "service_health_partial_offline.json",
        expected_overall_effective_state: EffectiveStateClass::Blocked,
        expected_overall_freshness: FreshnessClass::Unavailable,
        expected_honesty_marker_present: true,
        expected_green_downgrade_count: 1,
        expected_hidden_total: 0,
        view_id: view_id("service_health_partial_offline"),
        cards: vec![
            card(
                "card:sync",
                "Workspace sync",
                DisplayedStateClass::Attention,
                FreshnessClass::Partial,
                Some(RECENT),
                EvidenceKindClass::ServiceHealthCard,
                "aureline://service_health_card/sync",
                "Only part of the change set could be reconciled; remainder pending reconnect.",
            ),
            card(
                "card:marketplace",
                "Marketplace",
                DisplayedStateClass::Clear,
                FreshnessClass::Unavailable,
                Some(RECENT),
                EvidenceKindClass::ServiceHealthCard,
                "aureline://service_health_card/marketplace",
                "Marketplace fetch is unreachable; the row cannot confirm a healthy state.",
            ),
            card(
                "card:ai_assist",
                "AI assist",
                DisplayedStateClass::Blocked,
                FreshnessClass::PolicyBlocked,
                Some(RECENT),
                EvidenceKindClass::ServiceHealthCard,
                "aureline://service_health_card/ai_assist",
                "Workspace policy disables AI assist in this region; blocked until policy changes.",
            ),
        ],
        queue: None,
    }
}

fn review_inbox_ordered_narrowed() -> DashboardTruthScenario {
    DashboardTruthScenario {
        scenario_id: "review_inbox_ordered_narrowed",
        scenario_label: "Review inbox — ordered and narrowed",
        surface: DashboardSurfaceClass::ReviewInbox,
        narrative: "A blocking review sorts to the top, a review assigned to you follows, and a \
                    review whose CI evidence expired downgrades from green; the inbox discloses \
                    reviews hidden by scope, assignee, and resolved filters.",
        fixture_filename: "review_inbox_ordered_narrowed.json",
        expected_overall_effective_state: EffectiveStateClass::Blocked,
        expected_overall_freshness: FreshnessClass::Stale,
        expected_honesty_marker_present: true,
        expected_green_downgrade_count: 1,
        expected_hidden_total: 21,
        view_id: view_id("review_inbox_ordered_narrowed"),
        cards: vec![
            card(
                "card:cr-blocking",
                "Conflicted review needs rebase",
                DisplayedStateClass::Blocked,
                FreshnessClass::Fresh,
                Some(FRESH),
                EvidenceKindClass::ChangeReview,
                "aureline://change_review/cr-4471",
                "Review has merge conflicts and a failing required check.",
            ),
            card(
                "card:cr-mine",
                "Review awaiting your sign-off",
                DisplayedStateClass::Attention,
                FreshnessClass::Fresh,
                Some(RECENT),
                EvidenceKindClass::ChangeReview,
                "aureline://change_review/cr-4490",
                "Assigned to you; checks green and awaiting your approval.",
            ),
            card(
                "card:cr-stale",
                "Review with expired check evidence",
                DisplayedStateClass::Clear,
                FreshnessClass::Stale,
                Some(HOURS_AGO),
                EvidenceKindClass::ChangeReview,
                "aureline://change_review/cr-4402",
                "Approved earlier, but its check evidence aged out of the review window.",
            ),
        ],
        queue: Some(QueueOrderInput {
            queue_id: "queue:review_inbox".to_owned(),
            rows: vec![
                row(
                    "card:cr-blocking",
                    OrderReasonClass::SeverityDescending,
                    "Sorted to the top because it is blocked and cannot merge.",
                    "aureline://change_review/cr-4471",
                ),
                row(
                    "card:cr-mine",
                    OrderReasonClass::AssignedToYou,
                    "Raised above default order because it is assigned to you.",
                    "aureline://change_review/cr-4490",
                ),
                row(
                    "card:cr-stale",
                    OrderReasonClass::OldestUnresolvedFirst,
                    "Oldest unresolved review still open in this scope.",
                    "aureline://change_review/cr-4402",
                ),
            ],
            hidden_scope: vec![
                hidden(
                    NarrowingReasonClass::ScopeFilter,
                    6,
                    "6 reviews are outside the active workspace scope.",
                    "aureline://change_review_query/all_scopes",
                ),
                hidden(
                    NarrowingReasonClass::AssigneeFilter,
                    3,
                    "3 reviews are hidden by the assigned-to-me filter.",
                    "aureline://change_review_query/all_assignees",
                ),
                hidden(
                    NarrowingReasonClass::ResolvedHidden,
                    12,
                    "12 resolved reviews are hidden by the open-only filter.",
                    "aureline://change_review_query/include_resolved",
                ),
            ],
        }),
    }
}

fn incident_queue_severity_sla() -> DashboardTruthScenario {
    DashboardTruthScenario {
        scenario_id: "incident_queue_severity_sla",
        scenario_label: "Incident queue — severity and SLA ordering",
        surface: DashboardSurfaceClass::IncidentQueue,
        narrative: "A sev-1 incident leads on severity, an SLA-deadline incident follows, and a \
                    monitoring incident serving cached data downgrades from green; incidents below \
                    the severity filter are disclosed as hidden.",
        fixture_filename: "incident_queue_severity_sla.json",
        expected_overall_effective_state: EffectiveStateClass::Blocked,
        expected_overall_freshness: FreshnessClass::Cached,
        expected_honesty_marker_present: true,
        expected_green_downgrade_count: 1,
        expected_hidden_total: 5,
        view_id: view_id("incident_queue_severity_sla"),
        cards: vec![
            card(
                "card:inc-sev1",
                "Sev-1 sync outage",
                DisplayedStateClass::Blocked,
                FreshnessClass::Fresh,
                Some(FRESH),
                EvidenceKindClass::IncidentRecord,
                "aureline://incident_record/inc-2207",
                "Active sev-1 affecting workspace sync; responders engaged.",
            ),
            card(
                "card:inc-sla",
                "Sev-2 nearing SLA deadline",
                DisplayedStateClass::Attention,
                FreshnessClass::Fresh,
                Some(RECENT),
                EvidenceKindClass::IncidentRecord,
                "aureline://incident_record/inc-2210",
                "Response SLA deadline is approaching for this open incident.",
            ),
            card(
                "card:inc-monitoring",
                "Monitoring incident, cached status",
                DisplayedStateClass::Clear,
                FreshnessClass::Cached,
                Some(DAY_AGO),
                EvidenceKindClass::RunbookPacket,
                "aureline://runbook_packet/inc-2188",
                "Marked monitoring, but status is cached and its evidence aged out a day ago.",
            ),
        ],
        queue: Some(QueueOrderInput {
            queue_id: "queue:incident".to_owned(),
            rows: vec![
                row(
                    "card:inc-sev1",
                    OrderReasonClass::SeverityDescending,
                    "Highest severity active incident leads the queue.",
                    "aureline://incident_record/inc-2207",
                ),
                row(
                    "card:inc-sla",
                    OrderReasonClass::SlaDeadline,
                    "Raised because its response SLA deadline is soonest.",
                    "aureline://incident_record/inc-2210",
                ),
                row(
                    "card:inc-monitoring",
                    OrderReasonClass::RecentlyUpdated,
                    "Ordered by most recent update among monitoring incidents.",
                    "aureline://runbook_packet/inc-2188",
                ),
            ],
            hidden_scope: vec![hidden(
                NarrowingReasonClass::SeverityFilter,
                5,
                "5 incidents are below the active severity filter.",
                "aureline://incident_query/all_severities",
            )],
        }),
    }
}

fn support_queue_policy_scoped() -> DashboardTruthScenario {
    DashboardTruthScenario {
        scenario_id: "support_queue_policy_scoped",
        scenario_label: "Support queue — policy-scoped and blocked",
        surface: DashboardSurfaceClass::SupportQueue,
        narrative: "A blocking dependency leads, an SLA case follows, and a current case stays \
                    clear; cases hidden by policy scope are disclosed as unknown rather than \
                    silently dropped.",
        fixture_filename: "support_queue_policy_scoped.json",
        expected_overall_effective_state: EffectiveStateClass::Blocked,
        expected_overall_freshness: FreshnessClass::PolicyBlocked,
        expected_honesty_marker_present: true,
        expected_green_downgrade_count: 0,
        expected_hidden_total: 9,
        view_id: view_id("support_queue_policy_scoped"),
        cards: vec![
            card(
                "card:sc-blocked",
                "Export blocked by policy",
                DisplayedStateClass::Blocked,
                FreshnessClass::PolicyBlocked,
                Some(RECENT),
                EvidenceKindClass::SupportCase,
                "aureline://support_case/sc-8801",
                "Support export is blocked pending a policy decision; cannot proceed.",
            ),
            card(
                "card:sc-sla",
                "Case nearing response SLA",
                DisplayedStateClass::Attention,
                FreshnessClass::Fresh,
                Some(RECENT),
                EvidenceKindClass::SupportCase,
                "aureline://support_case/sc-8820",
                "Open support case approaching its first-response SLA.",
            ),
            card(
                "card:sc-current",
                "Case awaiting customer reply",
                DisplayedStateClass::Clear,
                FreshnessClass::Fresh,
                Some(FRESH),
                EvidenceKindClass::SupportCase,
                "aureline://support_case/sc-8834",
                "Up to date; waiting on the customer with current evidence.",
            ),
        ],
        queue: Some(QueueOrderInput {
            queue_id: "queue:support".to_owned(),
            rows: vec![
                row(
                    "card:sc-blocked",
                    OrderReasonClass::BlockingDependency,
                    "Leads the queue because it blocks dependent export work.",
                    "aureline://support_case/sc-8801",
                ),
                row(
                    "card:sc-sla",
                    OrderReasonClass::SlaDeadline,
                    "Raised because its first-response SLA is soonest.",
                    "aureline://support_case/sc-8820",
                ),
                row(
                    "card:sc-current",
                    OrderReasonClass::DefaultRecency,
                    "Default recency order; no stronger reason applied.",
                    "aureline://support_case/sc-8834",
                ),
            ],
            hidden_scope: vec![hidden(
                NarrowingReasonClass::PolicyScope,
                9,
                "9 cases exist outside your policy scope and cannot be listed here.",
                "aureline://support_case_query/policy_scope_request",
            )],
        }),
    }
}

fn admin_queue_offline_partial() -> DashboardTruthScenario {
    DashboardTruthScenario {
        scenario_id: "admin_queue_offline_partial",
        scenario_label: "Admin queue — offline partial list",
        surface: DashboardSurfaceClass::AdminQueue,
        narrative: "An audit follow-up loaded only partially offline (a green that downgrades to \
                    unconfirmed), a pinned policy item serves cached data, and the queue discloses \
                    both an incomplete offline list and archived items as hidden.",
        fixture_filename: "admin_queue_offline_partial.json",
        expected_overall_effective_state: EffectiveStateClass::Unconfirmed,
        expected_overall_freshness: FreshnessClass::Partial,
        expected_honesty_marker_present: true,
        expected_green_downgrade_count: 1,
        expected_hidden_total: 27,
        view_id: view_id("admin_queue_offline_partial"),
        cards: vec![
            card(
                "card:adm-audit",
                "Audit follow-up (partial offline load)",
                DisplayedStateClass::Clear,
                FreshnessClass::Partial,
                Some(RECENT),
                EvidenceKindClass::AuditEntry,
                "aureline://audit_entry/ae-5512",
                "Audit list loaded only partially while offline; cannot confirm it is clear.",
            ),
            card(
                "card:adm-policy",
                "Pinned policy decision",
                DisplayedStateClass::Attention,
                FreshnessClass::Cached,
                Some(RECENT),
                EvidenceKindClass::PolicyDecision,
                "aureline://policy_decision/pd-330",
                "Pinned for review; serving the last cached policy decision.",
            ),
        ],
        queue: Some(QueueOrderInput {
            queue_id: "queue:admin".to_owned(),
            rows: vec![
                row(
                    "card:adm-policy",
                    OrderReasonClass::ManualPin,
                    "Pinned to the top of the admin queue.",
                    "aureline://policy_decision/pd-330",
                ),
                row(
                    "card:adm-audit",
                    OrderReasonClass::RecentlyUpdated,
                    "Ordered by most recent update below the pin.",
                    "aureline://audit_entry/ae-5512",
                ),
            ],
            hidden_scope: vec![
                hidden(
                    NarrowingReasonClass::OfflinePartialList,
                    7,
                    "7 entries could not be loaded while offline; the remainder is unknown.",
                    "aureline://admin_audit_query/retry_full_load",
                ),
                hidden(
                    NarrowingReasonClass::ArchivedHidden,
                    20,
                    "20 archived items are hidden by the active-only filter.",
                    "aureline://admin_audit_query/include_archived",
                ),
            ],
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_scenario_builds_and_matches_expected_rollups() {
        for scenario in dashboard_truth_corpus() {
            let view = scenario.view();
            assert_eq!(
                view.overall_effective_state, scenario.expected_overall_effective_state,
                "{} overall_effective_state",
                scenario.scenario_id,
            );
            assert_eq!(
                view.overall_freshness, scenario.expected_overall_freshness,
                "{} overall_freshness",
                scenario.scenario_id,
            );
            assert_eq!(
                view.honesty_marker_present, scenario.expected_honesty_marker_present,
                "{} honesty_marker_present",
                scenario.scenario_id,
            );
            assert_eq!(
                view.summary.green_downgrade_count, scenario.expected_green_downgrade_count,
                "{} green_downgrade_count",
                scenario.scenario_id,
            );
            let hidden = view
                .queue_order
                .as_ref()
                .map(|q| q.hidden_total)
                .unwrap_or(0);
            assert_eq!(
                hidden, scenario.expected_hidden_total,
                "{} hidden_total",
                scenario.scenario_id,
            );
            assert_eq!(
                scenario.surface.is_queue(),
                view.queue_order.is_some(),
                "{} queue presence must match surface",
                scenario.scenario_id,
            );
        }
    }

    #[test]
    fn scenario_ids_and_fixture_names_are_unique() {
        let corpus = dashboard_truth_corpus();
        let mut ids: Vec<&str> = corpus.iter().map(|s| s.scenario_id).collect();
        ids.sort_unstable();
        let unique = ids.len();
        ids.dedup();
        assert_eq!(unique, ids.len(), "scenario ids must be unique");

        let mut files: Vec<&str> = corpus.iter().map(|s| s.fixture_filename).collect();
        files.sort_unstable();
        let unique_files = files.len();
        files.dedup();
        assert_eq!(
            unique_files,
            files.len(),
            "fixture filenames must be unique"
        );
    }

    #[test]
    fn corpus_exercises_every_freshness_class() {
        use std::collections::BTreeSet;
        let mut seen = BTreeSet::new();
        for scenario in dashboard_truth_corpus() {
            for c in &scenario.view().cards {
                seen.insert(c.freshness_token.clone());
            }
        }
        for token in [
            "fresh",
            "cached",
            "stale",
            "partial",
            "policy_blocked",
            "unavailable",
        ] {
            assert!(
                seen.contains(token),
                "freshness class {token} not exercised"
            );
        }
    }

    #[test]
    fn corpus_exercises_every_order_and_narrowing_reason() {
        use std::collections::BTreeSet;
        let mut orders = BTreeSet::new();
        let mut narrowings = BTreeSet::new();
        for scenario in dashboard_truth_corpus() {
            if let Some(q) = scenario.view().queue_order {
                for r in &q.rows {
                    orders.insert(r.order_reason_token.clone());
                }
                for h in &q.hidden_scope {
                    narrowings.insert(h.narrowing_reason_token.clone());
                }
            }
        }
        for token in [
            "severity_descending",
            "sla_deadline",
            "oldest_unresolved_first",
            "recently_updated",
            "assigned_to_you",
            "blocking_dependency",
            "manual_pin",
            "default_recency",
        ] {
            assert!(orders.contains(token), "order reason {token} not exercised");
        }
        for token in [
            "scope_filter",
            "policy_scope",
            "assignee_filter",
            "resolved_hidden",
            "archived_hidden",
            "severity_filter",
            "offline_partial_list",
        ] {
            assert!(
                narrowings.contains(token),
                "narrowing reason {token} not exercised",
            );
        }
    }
}
