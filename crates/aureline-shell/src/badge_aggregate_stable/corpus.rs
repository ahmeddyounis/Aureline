//! Deterministic claimed-stable matrix for badge-aggregate records.
//!
//! Every record here is a genuine projection of the **live** attention stack.
//! The corpus reads
//! [`crate::notification_envelope_corpus::seeded_notification_envelope_corpus_packet`],
//! derives each durable object's routing facts (canonical event id, dedupe key,
//! reopen target, owning subsystem) from a real
//! [`crate::attention_router::NotificationRouteOutcome`], then reconciles the
//! whole-shell badge aggregate through the governed builder. So a badge-aggregate
//! record can never drift from what the router actually produces.
//!
//! Four postures pin the matrix:
//!
//! - `nominal` — a full set of active durable objects across the count classes,
//!   each appearing across desktop / companion / window copies that dedupe to one
//!   object; a user-muted backlog item proves a zero active badge with a tracked,
//!   lineage-explained held count. Qualifies **Stable**.
//! - `quiet_and_admin_suppression` — the same set under quiet hours and admin
//!   suppression, with an admin-suppressed advisory, a quiet-hours-muted run, and
//!   a companion per-class badge disablement, all carrying export-safe lineage
//!   that preserves the durable object and reopen target. Qualifies **Stable**.
//! - `companion_preview_surface` — the nominal set, but the companion badge
//!   surface marker is Preview, so the snapshot is narrowed below Stable by its
//!   lowest surface marker instead of inheriting an adjacent green row.
//! - `cross_client_inflation_drill` — an adversarial snapshot where the companion
//!   surface multiplies cross-client copies instead of deduping them; the lane
//!   detects the inflation and narrows the snapshot below Stable with a named
//!   reason.

use std::collections::BTreeSet;

use crate::notification_attention_stable::model::{
    AccessibilityDisclosure, AttentionRouteSurface, EntryRouteRecord, LayoutMode,
    LayoutModeDisclosure, LifecycleMarker, StableClaimClass,
};
use crate::notification_envelope_corpus::{
    seeded_notification_envelope_corpus_packet, BetaAttentionFamily,
    NotificationEnvelopeCorpusCase, NotificationEnvelopeCorpusPacket,
};
use crate::notifications::envelope::{ClientScope, DedupeKeyScheme, QuietHoursMode};

use super::model::{
    required_recovery_routes, AggregateCountClass, BadgeAggregateClaimCeiling, BadgeAggregateInput,
    BadgeAggregateRecord, BadgeAggregateUpstream, BadgeSuppressionReason, BadgeSurface,
    DurableItemDisposition, RawObjectAppearance, SuppressionLineageEntry, SuppressionScope,
    SurfaceClassCount, SurfaceProjectionInput,
};

/// Snapshot timestamp pinned for every record in the corpus.
pub const CORPUS_AS_OF: &str = "2026-05-25T12:00:00Z";

const DIAGNOSTICS_EXPORT_REF: &str = "aureline://diagnostics/badge-aggregate";
const SUPPORT_EXPORT_REF: &str = "aureline://support-export/badge-aggregate";
const EVIDENCE_ARTIFACT_REF: &str = "aureline://artifact/ux-m4-badge-aggregate";
const EVIDENCE_FIXTURE_REF: &str = "aureline://fixture/ux-m4-badge-aggregate";
const NARRATIVE_REF: &str = "aureline://doc/ux-m4-badge-aggregate";

/// One scenario in the claimed-stable badge-aggregate matrix.
#[derive(Debug, Clone)]
pub struct BadgeAggregateScenario {
    /// Stable scenario id (also the record id).
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: String,
    /// Posture token pinned for the scenario.
    pub expected_posture: String,
    /// Expected derived claim class.
    pub expected_claim_class: StableClaimClass,
    /// Expected stable-qualification verdict.
    pub expected_qualifies_stable: bool,
    /// Expected derived surface lifecycle marker (lowest badge surface).
    pub expected_surface_marker: LifecycleMarker,
    record: BadgeAggregateRecord,
}

impl BadgeAggregateScenario {
    /// Returns the governed record for this scenario.
    pub fn record(&self) -> BadgeAggregateRecord {
        self.record.clone()
    }
}

/// Spec for one durable object in a scenario: which corpus case supplies its
/// routing facts, its canonical-object suffix, its count class and disposition,
/// and the client scopes it was reported from (its cross-client / cross-window
/// copies).
struct ObjectSpec {
    family: BetaAttentionFamily,
    suffix: &'static str,
    count_class: AggregateCountClass,
    disposition: DurableItemDisposition,
    scopes: &'static [ClientScope],
}

/// Spec for one suppression-lineage entry.
struct LineageSpec {
    reason: BadgeSuppressionReason,
    scope: SuppressionScope,
    object_suffix: Option<&'static str>,
    count_class: Option<AggregateCountClass>,
    surface: Option<BadgeSurface>,
    affected_surfaces: &'static [BadgeSurface],
    summary: &'static str,
}

/// Spec for one whole-shell snapshot.
struct ScenarioSpec {
    scenario_id: &'static str,
    posture_id: &'static str,
    posture_label: &'static str,
    title: &'static str,
    summary: &'static str,
    objects: &'static [ObjectSpec],
    lineage: &'static [LineageSpec],
    quiet_modes: &'static [QuietHoursMode],
    /// Companion badge surface marker (every other surface is Stable).
    companion_marker: LifecycleMarker,
    /// Count classes whose badge is disabled on the companion surface.
    companion_disabled: &'static [AggregateCountClass],
    /// Companion count overrides (used by the inflation drill).
    companion_overrides: &'static [(AggregateCountClass, u32)],
    claim_ceiling: BadgeAggregateClaimCeiling,
}

fn object_ref(suffix: &str) -> String {
    format!("aureline://activity-object/{suffix}")
}

fn case_for<'a>(
    packet: &'a NotificationEnvelopeCorpusPacket,
    family: BetaAttentionFamily,
) -> &'a NotificationEnvelopeCorpusCase {
    packet
        .cases
        .iter()
        .find(|case| case.attention_family == family)
        .unwrap_or_else(|| panic!("missing corpus case for family {}", family.as_str()))
}

/// Builds the raw appearances for a scenario from the live packet.
fn raw_appearances(
    packet: &NotificationEnvelopeCorpusPacket,
    specs: &[ObjectSpec],
) -> Vec<RawObjectAppearance> {
    let mut appearances = Vec::new();
    for spec in specs {
        let case = case_for(packet, spec.family);
        let outcome = &case.outcome;
        let canonical_event_id = format!("{}::{}", outcome.canonical_event_id, spec.suffix);
        let durable_object_preserved = true;
        for scope in spec.scopes {
            appearances.push(RawObjectAppearance {
                object_ref: object_ref(spec.suffix),
                client_scope: *scope,
                count_class: spec.count_class,
                disposition: spec.disposition,
                label: format!("{} ({})", outcome.summary_label, spec.suffix),
                owner_subsystem: outcome.source_subsystem,
                durable_object_preserved,
                reopen_target_ref: outcome.reopen_target.reopen_target_ref.clone(),
                canonical_event_id: canonical_event_id.clone(),
                dedupe_key_scheme: DedupeKeyScheme::CrossClientCanonicalEventId,
                dedupe_key_ref: canonical_event_id.clone(),
                route_outcome_id_ref: outcome.route_outcome_id.clone(),
                envelope_id_ref: outcome.source_notification_envelope_id_ref.clone(),
            });
        }
    }
    appearances
}

/// Active count per class from the object specs.
fn active_counts(specs: &[ObjectSpec]) -> Vec<(AggregateCountClass, u32)> {
    let present: BTreeSet<AggregateCountClass> = specs.iter().map(|s| s.count_class).collect();
    AggregateCountClass::ALL
        .into_iter()
        .filter(|class| present.contains(class))
        .map(|class| {
            let count = specs
                .iter()
                .filter(|s| {
                    s.count_class == class && s.disposition == DurableItemDisposition::Active
                })
                .count() as u32;
            (class, count)
        })
        .collect()
}

fn surface_counts(
    active: &[(AggregateCountClass, u32)],
    disabled: &[AggregateCountClass],
    overrides: &[(AggregateCountClass, u32)],
) -> Vec<SurfaceClassCount> {
    active
        .iter()
        .map(|(class, active_count)| {
            let reported_count = if disabled.contains(class) {
                0
            } else if let Some((_, override_count)) = overrides.iter().find(|(c, _)| c == class) {
                *override_count
            } else {
                *active_count
            };
            SurfaceClassCount {
                count_class: *class,
                reported_count,
            }
        })
        .collect()
}

fn build_scenario(
    packet: &NotificationEnvelopeCorpusPacket,
    spec: &ScenarioSpec,
    focus_order_index: u32,
) -> BadgeAggregateScenario {
    let raw = raw_appearances(packet, spec.objects);
    let active = active_counts(spec.objects);

    // Surfaces: activity center is authoritative; the standard desktop surfaces
    // echo the active counts; the companion surface carries the scenario's
    // marker, disablements, and any inflation override.
    let stable_surface = |surface: BadgeSurface| SurfaceProjectionInput {
        surface,
        surface_marker: LifecycleMarker::Stable,
        class_counts: surface_counts(&active, &[], &[]),
        disabled_classes: Vec::new(),
    };
    let surface_projections = vec![
        stable_surface(BadgeSurface::ActivityCenter),
        stable_surface(BadgeSurface::DockTaskbar),
        stable_surface(BadgeSurface::TitleBar),
        stable_surface(BadgeSurface::InShell),
        SurfaceProjectionInput {
            surface: BadgeSurface::Companion,
            surface_marker: spec.companion_marker,
            class_counts: surface_counts(
                &active,
                spec.companion_disabled,
                spec.companion_overrides,
            ),
            disabled_classes: spec.companion_disabled.to_vec(),
        },
    ];

    let suppression_lineage: Vec<SuppressionLineageEntry> = spec
        .lineage
        .iter()
        .map(|entry| SuppressionLineageEntry {
            reason: entry.reason,
            scope: entry.scope,
            object_ref: entry.object_suffix.map(object_ref),
            count_class: entry.count_class,
            surface: entry.surface,
            affected_surfaces: entry.affected_surfaces.to_vec(),
            durable_object_preserved: true,
            reopen_target_preserved: true,
            export_safe_summary: entry.summary.to_string(),
        })
        .collect();

    // Upstream traceability: the cases and outcomes the appearances projected
    // from.
    let mut case_refs = BTreeSet::new();
    let mut outcome_refs = BTreeSet::new();
    for object in spec.objects {
        let case = case_for(packet, object.family);
        case_refs.insert(case.case_id.clone());
        outcome_refs.insert(case.outcome.route_outcome_id.clone());
    }

    let recovery_routes = required_recovery_routes();
    let action_labels: Vec<String> = recovery_routes
        .iter()
        .map(|route| route.action_label.clone())
        .collect();
    let routes = AttentionRouteSurface::REQUIRED
        .into_iter()
        .map(|surface| EntryRouteRecord {
            surface,
            route_ref: format!(
                "aureline://badge-aggregate-route/{}/{}",
                surface.as_str(),
                spec.posture_id
            ),
            keyboard_reachable: true,
            activates_same_item: true,
        })
        .collect();

    let row_narration = format!(
        "Badge aggregate ({posture}) \u{2014} every count typed by class and derived from one \
         durable object set; cross-client copies dedupe to one object; suppressed and disabled \
         badges carry export-safe lineage; recovery: {actions}.",
        posture = spec.posture_label,
        actions = action_labels.join(", ")
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

    let input = BadgeAggregateInput {
        record_id: spec.scenario_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: spec.posture_id.to_string(),
        posture_label: spec.posture_label.to_string(),
        title: spec.title.to_string(),
        summary: spec.summary.to_string(),
        summary_id: format!("badge-aggregate-summary:{}", spec.posture_id),
        raw_appearances: raw,
        surface_projections,
        suppression_lineage,
        active_quiet_hours_modes: spec.quiet_modes.to_vec(),
        claim_ceiling: spec.claim_ceiling,
        recovery_routes,
        routes,
        accessibility,
        available_without_account: true,
        available_without_managed_services: true,
        upstream: BadgeAggregateUpstream {
            corpus_packet_ref: packet.packet_id.clone(),
            contributing_case_refs: case_refs.into_iter().collect(),
            contributing_route_outcome_refs: outcome_refs.into_iter().collect(),
        },
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: vec![
            EVIDENCE_ARTIFACT_REF.to_string(),
            EVIDENCE_FIXTURE_REF.to_string(),
        ],
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };

    let record = BadgeAggregateRecord::build(input)
        .unwrap_or_else(|err| panic!("{}: {err}", spec.scenario_id));

    BadgeAggregateScenario {
        scenario_id: spec.scenario_id,
        fixture_filename: format!("{}.json", spec.posture_id),
        expected_posture: record.posture_id.clone(),
        expected_claim_class: record.stable_qualification.claim_class,
        expected_qualifies_stable: record.stable_qualification.qualifies_stable,
        expected_surface_marker: record.surface_lifecycle_marker,
        record,
    }
}

// ---------------------------------------------------------------------------
// Object sets
// ---------------------------------------------------------------------------

const DESKTOP: ClientScope = ClientScope::DesktopProduct;
const COMPANION: ClientScope = ClientScope::CompanionSurface;
const REMOTE: ClientScope = ClientScope::RemoteAgent;
const MANAGED: ClientScope = ClientScope::ManagedAdminSurface;

/// The nominal active set: one object per count class, several appearing across
/// desktop / companion / window copies, plus a user-muted backlog item.
const NOMINAL_OBJECTS: &[ObjectSpec] = &[
    ObjectSpec {
        family: BetaAttentionFamily::AiApproval,
        suffix: "ai-approval",
        count_class: AggregateCountClass::PendingReviewApproval,
        disposition: DurableItemDisposition::Active,
        scopes: &[DESKTOP, COMPANION],
    },
    ObjectSpec {
        family: BetaAttentionFamily::ProviderSync,
        suffix: "provider-auth",
        count_class: AggregateCountClass::ProviderAuthAttention,
        disposition: DurableItemDisposition::Active,
        scopes: &[DESKTOP, COMPANION],
    },
    ObjectSpec {
        family: BetaAttentionFamily::ProviderSync,
        suffix: "provider-failed-run",
        count_class: AggregateCountClass::FailedRuns,
        disposition: DurableItemDisposition::Active,
        scopes: &[DESKTOP],
    },
    ObjectSpec {
        family: BetaAttentionFamily::InstallUpdateDownload,
        suffix: "install-publish",
        count_class: AggregateCountClass::QueuedPublishLater,
        disposition: DurableItemDisposition::Active,
        // Two desktop windows reported the same object — a cross-window copy.
        scopes: &[DESKTOP, DESKTOP],
    },
    ObjectSpec {
        family: BetaAttentionFamily::PolicyChange,
        suffix: "policy-advisory",
        count_class: AggregateCountClass::ManagedAdvisories,
        disposition: DurableItemDisposition::Active,
        scopes: &[DESKTOP, MANAGED],
    },
    ObjectSpec {
        family: BetaAttentionFamily::Indexing,
        suffix: "indexing-run",
        count_class: AggregateCountClass::DurableRunning,
        disposition: DurableItemDisposition::Active,
        scopes: &[DESKTOP],
    },
    ObjectSpec {
        family: BetaAttentionFamily::RemoteReconnect,
        suffix: "remote-session",
        count_class: AggregateCountClass::SessionRequests,
        disposition: DurableItemDisposition::Active,
        scopes: &[DESKTOP, REMOTE],
    },
    ObjectSpec {
        family: BetaAttentionFamily::ManagedAlert,
        suffix: "muted-backlog",
        count_class: AggregateCountClass::MutedInformationalBacklog,
        disposition: DurableItemDisposition::HeldOrSuppressed,
        scopes: &[DESKTOP],
    },
];

const NOMINAL_LINEAGE: &[LineageSpec] = &[LineageSpec {
    reason: BadgeSuppressionReason::UserMuted,
    scope: SuppressionScope::Object,
    object_suffix: Some("muted-backlog"),
    count_class: Some(AggregateCountClass::MutedInformationalBacklog),
    surface: None,
    affected_surfaces: &[
        BadgeSurface::DockTaskbar,
        BadgeSurface::TitleBar,
        BadgeSurface::InShell,
        BadgeSurface::Companion,
    ],
    summary: "User muted this informational backlog item; it stays in durable history and reopens \
              its source, but produces no active badge.",
}];

/// The suppression set: active objects plus an admin-suppressed advisory, a
/// quiet-hours-muted run, and an active informational item disabled on companion.
const SUPPRESSION_OBJECTS: &[ObjectSpec] = &[
    ObjectSpec {
        family: BetaAttentionFamily::AiApproval,
        suffix: "ai-approval",
        count_class: AggregateCountClass::PendingReviewApproval,
        disposition: DurableItemDisposition::Active,
        scopes: &[DESKTOP, COMPANION],
    },
    ObjectSpec {
        family: BetaAttentionFamily::ProviderSync,
        suffix: "provider-auth",
        count_class: AggregateCountClass::ProviderAuthAttention,
        disposition: DurableItemDisposition::Active,
        scopes: &[DESKTOP, COMPANION],
    },
    ObjectSpec {
        family: BetaAttentionFamily::ProviderSync,
        suffix: "provider-failed-run",
        count_class: AggregateCountClass::FailedRuns,
        disposition: DurableItemDisposition::Active,
        scopes: &[DESKTOP],
    },
    ObjectSpec {
        family: BetaAttentionFamily::InstallUpdateDownload,
        suffix: "install-publish",
        count_class: AggregateCountClass::QueuedPublishLater,
        disposition: DurableItemDisposition::Active,
        scopes: &[DESKTOP],
    },
    ObjectSpec {
        family: BetaAttentionFamily::PolicyChange,
        suffix: "policy-advisory",
        count_class: AggregateCountClass::ManagedAdvisories,
        disposition: DurableItemDisposition::Active,
        scopes: &[DESKTOP],
    },
    ObjectSpec {
        family: BetaAttentionFamily::ManagedAlert,
        suffix: "managed-suppressed",
        count_class: AggregateCountClass::ManagedAdvisories,
        disposition: DurableItemDisposition::HeldOrSuppressed,
        scopes: &[DESKTOP, MANAGED],
    },
    ObjectSpec {
        family: BetaAttentionFamily::Restore,
        suffix: "restore-muted",
        count_class: AggregateCountClass::DurableRunning,
        disposition: DurableItemDisposition::HeldOrSuppressed,
        scopes: &[DESKTOP],
    },
    ObjectSpec {
        family: BetaAttentionFamily::ManagedAlert,
        suffix: "info-backlog",
        count_class: AggregateCountClass::MutedInformationalBacklog,
        disposition: DurableItemDisposition::Active,
        scopes: &[DESKTOP],
    },
];

const SUPPRESSION_LINEAGE: &[LineageSpec] = &[
    LineageSpec {
        reason: BadgeSuppressionReason::AdminPolicySuppression,
        scope: SuppressionScope::Object,
        object_suffix: Some("managed-suppressed"),
        count_class: Some(AggregateCountClass::ManagedAdvisories),
        surface: None,
        affected_surfaces: &[
            BadgeSurface::DockTaskbar,
            BadgeSurface::TitleBar,
            BadgeSurface::InShell,
            BadgeSurface::Companion,
        ],
        summary:
            "Administrator policy suppressed this advisory's alerts; the durable advisory and \
                  its reopen target stay reachable in the activity center.",
    },
    LineageSpec {
        reason: BadgeSuppressionReason::QuietHoursMuting,
        scope: SuppressionScope::Object,
        object_suffix: Some("restore-muted"),
        count_class: Some(AggregateCountClass::DurableRunning),
        surface: None,
        affected_surfaces: &[
            BadgeSurface::DockTaskbar,
            BadgeSurface::TitleBar,
            BadgeSurface::InShell,
            BadgeSurface::Companion,
        ],
        summary: "Quiet hours muted this running job's alerts; the durable job row and its reopen \
                  target stay reachable, so the zero active running badge means none are active.",
    },
    LineageSpec {
        reason: BadgeSuppressionReason::PerClassBadgeDisabled,
        scope: SuppressionScope::SurfaceClass,
        object_suffix: None,
        count_class: Some(AggregateCountClass::MutedInformationalBacklog),
        surface: Some(BadgeSurface::Companion),
        affected_surfaces: &[BadgeSurface::Companion],
        summary: "Informational backlog badges are disabled on the companion surface by setting; \
                  the items stay badged in-shell and reachable from the activity center.",
    },
];

const SUPPRESSION_QUIET_MODES: &[QuietHoursMode] = &[
    QuietHoursMode::ModeQuietHoursUser,
    QuietHoursMode::ModeAdminSuppression,
];

const COMPANION_DISABLED_MUTED: &[AggregateCountClass] =
    &[AggregateCountClass::MutedInformationalBacklog];

const STABLE_CEILING: BadgeAggregateClaimCeiling = BadgeAggregateClaimCeiling {
    asserts_one_durable_set: true,
    asserts_cross_client_dedupe: true,
    asserts_suppression_lineage_export_safe: true,
    asserts_zero_means_no_durable_items: true,
    asserts_summary_persistent_inspectable: true,
};

/// The inflation drill's claim ceiling: it cannot assert one durable set,
/// because the companion surface multiplies cross-client copies.
const DRILL_CEILING: BadgeAggregateClaimCeiling = BadgeAggregateClaimCeiling {
    asserts_one_durable_set: false,
    asserts_cross_client_dedupe: true,
    asserts_suppression_lineage_export_safe: true,
    asserts_zero_means_no_durable_items: true,
    asserts_summary_persistent_inspectable: true,
};

/// The companion surface in the drill double-counts the deduped cross-client
/// copies for two classes, inflating them above the authoritative active count.
const DRILL_COMPANION_OVERRIDES: &[(AggregateCountClass, u32)] = &[
    (AggregateCountClass::PendingReviewApproval, 2),
    (AggregateCountClass::ProviderAuthAttention, 2),
];

fn scenario_specs() -> Vec<ScenarioSpec> {
    vec![
        ScenarioSpec {
            scenario_id: "badge-aggregate:nominal",
            posture_id: "nominal",
            posture_label: "Nominal",
            title: "Badge aggregate: typed count classes from one durable object set",
            summary: "Whole-shell badge snapshot: every count is typed by class and derived from \
                      the same durable object set the activity center reads; cross-client and \
                      cross-window copies dedupe to one object so no badge multiplies; a \
                      user-muted backlog item shows a zero active badge with its held count and \
                      lineage; the dock/taskbar, title-bar, in-shell, and companion projections \
                      agree with the activity center; qualifies Stable.",
            objects: NOMINAL_OBJECTS,
            lineage: NOMINAL_LINEAGE,
            quiet_modes: &[],
            companion_marker: LifecycleMarker::Stable,
            companion_disabled: &[],
            companion_overrides: &[],
            claim_ceiling: STABLE_CEILING,
        },
        ScenarioSpec {
            scenario_id: "badge-aggregate:quiet_and_admin_suppression",
            posture_id: "quiet_and_admin_suppression",
            posture_label: "Quiet hours + admin suppression",
            title: "Badge aggregate: suppression and disablement with export-safe lineage",
            summary: "Whole-shell badge snapshot under quiet hours and admin suppression: an \
                      admin-suppressed advisory and a quiet-hours-muted run move out of the active \
                      badge into a tracked held count, and informational backlog badges are \
                      disabled on the companion surface — every difference carries export-safe \
                      lineage that preserves the durable object and reopen target, so support can \
                      explain each missing alert; qualifies Stable.",
            objects: SUPPRESSION_OBJECTS,
            lineage: SUPPRESSION_LINEAGE,
            quiet_modes: SUPPRESSION_QUIET_MODES,
            companion_marker: LifecycleMarker::Stable,
            companion_disabled: COMPANION_DISABLED_MUTED,
            companion_overrides: &[],
            claim_ceiling: STABLE_CEILING,
        },
        ScenarioSpec {
            scenario_id: "badge-aggregate:companion_preview_surface",
            posture_id: "companion_preview_surface",
            posture_label: "Companion badge surface in preview",
            title: "Badge aggregate: narrowed below Stable by a preview companion surface",
            summary: "The nominal durable object set, but the companion badge surface marker is \
                      Preview; the snapshot keeps every pillar yet is narrowed below Stable by its \
                      lowest badge surface marker instead of inheriting an adjacent green row.",
            objects: NOMINAL_OBJECTS,
            lineage: NOMINAL_LINEAGE,
            quiet_modes: &[],
            companion_marker: LifecycleMarker::Preview,
            companion_disabled: &[],
            companion_overrides: &[],
            claim_ceiling: STABLE_CEILING,
        },
        ScenarioSpec {
            scenario_id: "badge-aggregate:cross_client_inflation_drill",
            posture_id: "cross_client_inflation_drill",
            posture_label: "Cross-client inflation drill",
            title: "Badge aggregate: inflation drill narrowed below Stable",
            summary: "Adversarial snapshot where the companion surface multiplies cross-client \
                      copies of the review and provider-auth objects instead of deduping them; the \
                      lane detects the inflation against the authoritative durable set and narrows \
                      the snapshot below Stable with a named reason rather than publishing an \
                      inflated badge.",
            objects: NOMINAL_OBJECTS,
            lineage: NOMINAL_LINEAGE,
            quiet_modes: &[],
            companion_marker: LifecycleMarker::Stable,
            companion_disabled: &[],
            companion_overrides: DRILL_COMPANION_OVERRIDES,
            claim_ceiling: DRILL_CEILING,
        },
    ]
}

/// Returns the full claimed-stable badge-aggregate matrix.
pub fn badge_aggregate_corpus() -> Vec<BadgeAggregateScenario> {
    let packet = seeded_notification_envelope_corpus_packet();
    scenario_specs()
        .iter()
        .enumerate()
        .map(|(index, spec)| build_scenario(&packet, spec, index as u32))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corpus_spans_stable_and_narrowed_rows() {
        let corpus = badge_aggregate_corpus();
        assert_eq!(corpus.len(), 4);
        let stable = corpus
            .iter()
            .filter(|s| s.expected_claim_class == StableClaimClass::Stable)
            .count();
        let narrowed = corpus.len() - stable;
        assert!(stable >= 2, "matrix must include Stable rows");
        assert!(narrowed >= 2, "matrix must include narrowed rows");
    }

    #[test]
    fn every_required_count_class_is_exercised() {
        let corpus = badge_aggregate_corpus();
        let mut seen = BTreeSet::new();
        for scenario in &corpus {
            for class in &scenario.record().class_aggregates {
                seen.insert(class.count_class);
            }
        }
        for required in AggregateCountClass::REQUIRED {
            assert!(
                seen.contains(&required),
                "missing class {}",
                required.as_str()
            );
        }
    }

    #[test]
    fn stable_rows_lock_every_pillar() {
        for scenario in badge_aggregate_corpus() {
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
            assert!(
                record.pillars.one_durable_set_holds,
                "{}",
                scenario.scenario_id
            );
            assert!(
                record.pillars.cross_client_dedupe_holds,
                "{}",
                scenario.scenario_id
            );
            assert!(
                record.pillars.suppression_lineage_export_safe,
                "{}",
                scenario.scenario_id
            );
            assert!(
                record.pillars.zero_means_no_durable_items,
                "{}",
                scenario.scenario_id
            );
            assert!(
                record.pillars.summary_persistent_inspectable,
                "{}",
                scenario.scenario_id
            );
        }
    }

    #[test]
    fn cross_client_dedupe_collapses_copies() {
        let nominal = badge_aggregate_corpus()
            .into_iter()
            .find(|s| s.scenario_id == "badge-aggregate:nominal")
            .expect("nominal scenario")
            .record();
        // More raw appearances than deduped objects: copies collapsed.
        assert!(
            nominal.cross_client_dedupe.cross_client_collapsed > 0,
            "expected cross-client / cross-window copies to collapse"
        );
        assert_eq!(
            nominal.cross_client_dedupe.deduped_object_count,
            nominal.deduped_objects.len() as u32
        );
        // No surface inflates a class beyond the authoritative active count.
        for projection in &nominal.surface_projections {
            assert!(
                !projection.inflates_any_class,
                "{} inflated",
                projection.surface.as_str()
            );
        }
    }

    #[test]
    fn zero_active_means_no_active_durable_items() {
        let record = badge_aggregate_corpus()
            .into_iter()
            .find(|s| s.scenario_id == "badge-aggregate:nominal")
            .expect("nominal scenario")
            .record();
        let muted = record
            .class_aggregates
            .iter()
            .find(|c| c.count_class == AggregateCountClass::MutedInformationalBacklog)
            .expect("muted backlog class present");
        assert_eq!(
            muted.active_count, 0,
            "muted backlog must not be in the active badge"
        );
        assert!(
            muted.held_or_suppressed_count >= 1,
            "held count must track the muted item"
        );
    }

    #[test]
    fn inflation_drill_narrows_below_stable() {
        let drill = badge_aggregate_corpus()
            .into_iter()
            .find(|s| s.scenario_id == "badge-aggregate:cross_client_inflation_drill")
            .expect("drill scenario")
            .record();
        assert!(!drill.stable_qualification.qualifies_stable);
        assert!(!drill.pillars.one_durable_set_holds);
        assert!(drill
            .stable_qualification
            .narrowing_reasons
            .contains(&super::super::model::BadgeAggregateNarrowingReason::OneDurableSetNotProven));
    }
}
