//! Implemented M5 post-launch orr-history-event and follow-up-closure registries.
//!
//! The frozen [supported-line-transparency matrix][matrix] names Aureline's governed post-launch external-proof
//! objects and locks their controlled vocabulary. This module makes the supported-line ORR-history archive operable
//! so later promotion, support, and postmortem work never depends on shiproom folklore: it preserves supported-line
//! launch and servicing memory rather than leaving it in oral history and archived meeting packets, and turns the
//! *ORR-history-event* grammar (one typed archive entry per recorded operational-readiness decision on an active
//! supported line — an archived ORR packet, a freeze exception, a rehearsal outcome, a cohort transition, a go/no-go
//! decision, and a post-review action-item closure — each bound to one supported-line identity with its decision
//! dates, cohort transitions, freeze exceptions, and follow-up closure state, tracked against exact build /
//! release-line identity, with public-safe cohort-transition and go/no-go decision history separated from
//! internal-only freeze / rehearsal / action-item minutiae) and the *follow-up-closure* grammar (the closure-drift
//! scope a line's follow-up state sits in versus its archived ORR history — an unclosed action item, stale rehearsal
//! evidence, or a line history that can no longer be reconstructed from the archive) into registry resolvers that
//! produce export-safe, honest projections. Every active stable or LTS-candidate line then resolves to one typed
//! ORR-history event — the decision it records, its go/no-go outcome, the cohort and freeze context, the linked
//! supported-line-matrix / active-claim / correction-train / line-history refs, and its recorded decision history,
//! all preserved before a go/no-go or cohort claim widens so a line never keeps a claim ahead of recorded ORR history
//! — and to one follow-up-closure event — the resolved line identity, the affected history-entry reference, the
//! archived-versus-active-line reference, the closure-scope state, and the active closure reason — that the release /
//! help, docs, support, and governance surfaces can inspect without shiproom notes, so unclosed follow-up work and
//! stale rehearsal evidence stay visible on the active line, a line that can no longer be reconstructed from ORR
//! history surfaces automatically, and an archive that cannot bind an entry to its recorded decision degrades
//! honestly instead of leaving a stale go/no-go claim to read as still green.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Archive ORR packets, cohort changes, freeze exceptions, rehearsal outcomes, go/no-go decisions, and
//!   post-review action-item closure against each supported-line identity.**
//!   [`resolve_orr_history_event_entry`] refuses to read as a clean, registry-bound ORR-history event
//!   unless it names a canonical registry token, a classified [event class][M5OrrHistoryEventKind], a
//!   transparency role, covers every [resolution form][M5SupportedLineOrrHistoryEventResolutionForm] (the
//!   canonical object, the accessible summary, and the audit record), publishes every archive field (recorded
//!   decision rows, linked supported-line-matrix / active-claim / correction-train / line-history refs, decision
//!   outcome, rollback / rehearsal target, and owning roster), preserves its recorded decision history before a claim
//!   widens, and keeps any event class bound to its recorded decision; otherwise it degrades.
//! * **Link archive entries to supported-line matrices, active claims, correction-train packets, and line-history
//!   snapshots so widening and maintenance decisions remain reconstructable.**
//!   [`line_preserves_rollback_and_diagnostics_before_widening`] rejects an
//!   ORR-history event whose recorded decision history is not preserved before widening (a line resuming a wider
//!   go/no-go or cohort claim on stale rehearsal evidence) so it degrades to
//!   [`M5OrrHistoryEventEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof`],
//!   and a public-facing event class whose published claim outruns recorded history degrades the same way — the
//!   structured blocker reason a widen-on-stale-history attempt must surface. Public-safe and internal-only
//!   variants share one canonical record identity so they never diverge on core supported-line facts.
//! * **Provide export-safe history views for support, partner, procurement, and governance consumers that need
//!   durable change context without privileged internal minutiae.**
//!   [`resolve_follow_up_closure_entry`] names a classified [closure scope][M5FollowUpClosureScope]
//!   (unclosed-action-item, stale-rehearsal-evidence, or unreconstructable-line-history), requires the full
//!   line-identity / affected-history-entry / archived-versus-active-line / closure-scope /
//!   active-reason closure object, covers every resolution form, and degrades to
//!   [`M5FollowUpClosureEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence`]
//!   when the closure event would keep a claim ahead of recorded ORR history, hide the closure, or let an unclosed
//!   follow-up masquerade as closed, so a follow-up-closure event can never read as trustworthy when it has quietly
//!   dropped the reason a line's follow-up state changed.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5SupportedLineTransparencyRole`] role
//! vocabulary and the [`M5SupportedLineTransparencyConsumerSurface`] consumer-surface taxonomy — so the shiproom,
//! release-center, executive-steering, program-governance, diagnostics, docs, CLI, support, and governance
//! surfaces can never fork their own line meaning. Raw secret values and private endpoints stay outside the
//! export boundary.
//!
//! [matrix]: crate::m5_supported_line_transparency_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_supported_line_orr_history_and_follow_up_closure_registries,
    seeded_m5_supported_line_orr_history_and_follow_up_closure_registries_follow_up_closure_preview_narrowed,
    seeded_m5_supported_line_orr_history_and_follow_up_closure_registries_orr_history_event_beta_narrowed,
    M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_supported_line_transparency_matrix::{
    M5SupportedLineTransparencyAccessibilityRoute, M5SupportedLineTransparencyConsumerSurface,
    M5SupportedLineTransparencyDowngradeTrigger, M5SupportedLineTransparencyObject,
    M5SupportedLineTransparencyQualificationClass, M5SupportedLineTransparencyRequiredLabel,
    M5SupportedLineTransparencyRole, M5SupportedLineTransparencyWideningStage,
    M5_SUPPORTED_LINE_ORR_HISTORY_DOMAIN_SCHEMA_REF, M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_DOC_REF,
    M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket`].
pub const M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_supported_line_orr_history_and_follow_up_closure_packet_registries";

/// Schema version for M5 line-orr_history_event / line-downgrade-packet registry records.
pub const M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_SCHEMA_REF: &str =
    "schemas/program/m5-supported-line-orr-history-and-follow-up-closure-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_DOC_REF: &str =
    "docs/release/m5_supported_line_orr_history_and_follow_up_closure_registries.md";

/// Repo-relative path of the canonical follow-up-closure domain schema minted by this lane (the
/// machine-readable closure-drift event emitted when a line carries an unclosed action item, has stale
/// rehearsal evidence, or can no longer be reconstructed from its archived ORR history).
pub const M5_FOLLOW_UP_CLOSURE_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-follow-up-closure.schema.json";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-supported-line-orr-history-and-follow-up-closure-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-supported-line-orr-history-and-follow-up-closure-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-supported-line-orr-history-and-follow-up-closure-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/release/m5-supported-line-orr-history-and-follow-up-closure-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// line invents a parallel surface set.
pub type M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesConsumerSurface =
    M5SupportedLineTransparencyConsumerSurface;

/// One of the three resolution forms every line-orr_history_event or line-downgrade-packet entry must hold across so
/// its truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or
/// written to the audit / support record. Minted by this lane because the frozen matrix names the
/// line-orr_history_event and line-downgrade *domains* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportedLineOrrHistoryEventResolutionForm {
    /// The canonical resolved line-orr_history_event / line-downgrade-packet object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved line discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved line inspectable off-renderer.
    AuditRecord,
}

impl M5SupportedLineOrrHistoryEventResolutionForm {
    /// Every resolution form, in declaration order. A clean entry must cover all three.
    pub const ALL: [Self; 3] = [
        Self::CanonicalObject,
        Self::AccessibleSummary,
        Self::AuditRecord,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalObject => "canonical_object",
            Self::AccessibleSummary => "accessible_summary",
            Self::AuditRecord => "audit_record",
        }
    }
}

/// Controlled operational-readiness event class an ORR-history entry archives for one supported-line identity,
/// so every active line's launch and servicing memory shares one typed registry rather than oral history and
/// archived meeting packets. Minted by this lane because the frozen matrix names the supported lines but not the
/// concrete ORR event classes a durable ORR-history archive must retain against exact build / release-line identity —
/// an archived ORR packet, a freeze exception, a rehearsal outcome, a cohort transition, a go/no-go decision, and a
/// post-review action-item closure. Every classified event class carries its canonical mode, and the cohort-transition
/// and go/no-go decision classes are public-facing (their history surfaces directly in the public-safe ORR-history
/// view partners and procurement read) so their published claim must stay matched to recorded decision history before
/// the line widens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OrrHistoryEventKind {
    /// The archived ORR packet: the recorded operational-readiness review packet, with its decision dates and the
    /// exact supported-line-matrix ref backing it.
    OrrPacketArchive,
    /// The freeze exception: a recorded exception to a release freeze, with the correction-train ref naming what it
    /// covered (internal-only).
    FreezeException,
    /// The rehearsal outcome: the recorded launch / recovery rehearsal result, with the line-history ref naming its
    /// rehearsal evidence (internal-only).
    RehearsalOutcome,
    /// The cohort transition (public-facing; a recorded change to which cohort the line serves, whose published claim
    /// must match recorded decision history).
    CohortTransition,
    /// The go/no-go decision (public-facing; the recorded go / no-go / conditional-go outcome, whose published claim
    /// must match recorded decision history so procurement sees the decision that actually shipped).
    GoNoGoDecision,
    /// The post-review action-item closure: whether a post-review follow-up action closed cleanly, linked to its
    /// closure target so an unclosed follow-up surfaces on the affected line rather than staying hidden (internal-only).
    ActionItemClosure,
    /// The ORR-history event class is unclassified, which is disallowed.
    EventClassUnclassified,
}

impl M5OrrHistoryEventKind {
    /// Every orr-history-event kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::OrrPacketArchive,
        Self::FreezeException,
        Self::RehearsalOutcome,
        Self::CohortTransition,
        Self::GoNoGoDecision,
        Self::ActionItemClosure,
        Self::EventClassUnclassified,
    ];

    /// The six canonical orr-history-event kinds every claimed M5 supported line archives for its readiness history.
    pub const CANONICAL_JOURNEYS: [Self; 6] = [
        Self::OrrPacketArchive,
        Self::FreezeException,
        Self::RehearsalOutcome,
        Self::CohortTransition,
        Self::GoNoGoDecision,
        Self::ActionItemClosure,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OrrPacketArchive => "orr_packet_archive",
            Self::FreezeException => "freeze_exception",
            Self::RehearsalOutcome => "rehearsal_outcome",
            Self::CohortTransition => "cohort_transition",
            Self::GoNoGoDecision => "go_no_go_decision",
            Self::ActionItemClosure => "action_item_closure",
            Self::EventClassUnclassified => "event_class_unclassified",
        }
    }

    /// Whether the item is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::EventClassUnclassified)
    }

    /// The canonical mode for this orr-history-event kind.
    pub const fn canonical_report_section_mode(self) -> &'static str {
        match self {
            Self::OrrPacketArchive => "orr_packet_archive_mode",
            Self::FreezeException => "freeze_exception_mode",
            Self::RehearsalOutcome => "rehearsal_outcome_mode",
            Self::CohortTransition => "cohort_transition_mode",
            Self::GoNoGoDecision => "go_no_go_decision_mode",
            Self::ActionItemClosure => "action_item_closure_mode",
            Self::EventClassUnclassified => "",
        }
    }

    /// Whether this ORR-history event class is public-facing and so must keep its published
    /// claim matched to recorded decision history before the line widens.
    pub const fn is_public_facing_line(self) -> bool {
        matches!(self, Self::CohortTransition | Self::GoNoGoDecision)
    }
}

/// Controlled follow-up-closure scope a line's ORR-history follow-up state sits in, so a change in an active line's
/// unclosed follow-up work or rehearsal evidence becomes a typed closure event against its archived ORR history
/// rather than a forgotten shiproom note, and shares one registry rather than a hand-copied per-record assumption.
/// Minted by this lane, tracking whether a post-review action item is still unclosed on the active line
/// (unclosed-action-item), the line's rehearsal evidence has gone stale (stale-rehearsal-evidence), or the line can
/// no longer be reconstructed from its archived ORR history (unreconstructable-line-history). Each scope maps directly
/// to the follow-up-closure visibility the implementation requirement names, so support, partner, procurement, and
/// governance reviews follow the same currentness rules as internal release control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FollowUpClosureScope {
    /// Unclosed-action-item: a post-review follow-up action is still unclosed on the active line rather than only in
    /// an archived meeting packet.
    UnclosedActionItem,
    /// Stale-rehearsal-evidence: the line's rehearsal evidence has gone stale versus its archived ORR history.
    StaleRehearsalEvidence,
    /// Unreconstructable-line-history: the current supported line can no longer be reconstructed from ORR history
    /// without separate shiproom notes or oral memory.
    UnreconstructableLineHistory,
    /// The follow-up-closure scope is unclassified, which is disallowed.
    ClosureScopeUnclassified,
}

impl M5FollowUpClosureScope {
    /// Every comparison scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::UnclosedActionItem,
        Self::StaleRehearsalEvidence,
        Self::UnreconstructableLineHistory,
        Self::ClosureScopeUnclassified,
    ];

    /// The three canonical comparison scopes every follow-up-closure report must stay distinct across.
    pub const CANONICAL_SCOPES: [Self; 3] = [
        Self::UnclosedActionItem,
        Self::StaleRehearsalEvidence,
        Self::UnreconstructableLineHistory,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnclosedActionItem => "unclosed_action_item",
            Self::StaleRehearsalEvidence => "stale_rehearsal_evidence",
            Self::UnreconstructableLineHistory => "unreconstructable_line_history",
            Self::ClosureScopeUnclassified => "closure_scope_unclassified",
        }
    }

    /// Whether the comparison scope is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ClosureScopeUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a line-orr_history_event or
/// line-downgrade-packet token's meaning stays stable whether it appears in the release-center, shiproom,
/// executive-steering, program-governance, or a support / export form. Minted by this lane, tracking the
/// first-consumer surfaces the implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportedLineOrrHistoryEventSurfaceContext {
    /// The release-center surface.
    ReleaseCenterSurface,
    /// The shiproom surface.
    ShiproomSurface,
    /// The executive-steering surface.
    ExecutiveSteeringSurface,
    /// The program-governance surface.
    ProgramGovernanceSurface,
    /// The support / export form surface.
    SupportOrExportForm,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5SupportedLineOrrHistoryEventSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReleaseCenterSurface,
        Self::ShiproomSurface,
        Self::ExecutiveSteeringSurface,
        Self::ProgramGovernanceSurface,
        Self::SupportOrExportForm,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::ReleaseCenterSurface,
        Self::ShiproomSurface,
        Self::ExecutiveSteeringSurface,
        Self::ProgramGovernanceSurface,
        Self::SupportOrExportForm,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenterSurface => "release_center_surface",
            Self::ShiproomSurface => "shiproom_surface",
            Self::ExecutiveSteeringSurface => "executive_steering_surface",
            Self::ProgramGovernanceSurface => "program_governance_surface",
            Self::SupportOrExportForm => "support_or_export_form",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part a line-orr_history_event or line-downgrade-packet entry must be able to show, so no
/// line journey, repo / bundle / toolchain / deployment row, known-limits packet, rollback target,
/// line-downgrade field, or registry fact is left implicit behind a hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportedLineOrrHistoryEventAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The line journey the entry classifies (line-orr_history_event entry).
    CohortArchetype,
    /// The exact repo / journey rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles the entry publishes (line-orr_history_event entry).
    RepoBundleToolchainAndDeploymentRows,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The known-limits packet and rollback target the entry preserves before widening (line-orr_history_event
    /// entry).
    KnownLimitsAndRollbackTarget,
    /// The line-downgrade fields (line identity, known-limits ledger, rollback target, rehearsal currency,
    /// readiness signoff, support language) the entry publishes (line-downgrade-packet entry).
    CohortEvidenceFields,
    /// The support-identity hint the entry publishes (line-downgrade-packet entry).
    SupportIdentityHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved line orr_history_event or line downgrade (both entries).
    PlainLanguageMeaning,
}

impl M5SupportedLineOrrHistoryEventAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::CohortArchetype,
        Self::RepoBundleToolchainAndDeploymentRows,
        Self::ResolutionFormCoverage,
        Self::KnownLimitsAndRollbackTarget,
        Self::CohortEvidenceFields,
        Self::SupportIdentityHint,
        Self::KeyboardRoute,
        Self::PlainLanguageMeaning,
    ];

    /// The three parts every claimed entry must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::RegistryReference];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::RegistryReference => "registry_reference",
            Self::CohortArchetype => "report_section",
            Self::RepoBundleToolchainAndDeploymentRows => {
                "repo_bundle_toolchain_and_deployment_rows"
            }
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::KnownLimitsAndRollbackTarget => "known_limits_and_rollback_target",
            Self::CohortEvidenceFields => "follow_up_closure_fields",
            Self::SupportIdentityHint => "support_identity_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// line orr_history_event, a line-downgrade packet, or a degraded line-orr_history_event / line-downgrade-packet entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportedLineOrrHistoryEventNextAction {
    /// Expand the resolved line orr_history_event's or line-downgrade packet's plain-language meaning.
    ExpandCohortMeaning,
    /// Inspect the line journey or downgrade scope the entry resolves.
    InspectArchetypeOrScope,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5SupportedLineOrrHistoryEventNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandCohortMeaning,
        Self::InspectArchetypeOrScope,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandCohortMeaning => "expand_line_meaning",
            Self::InspectArchetypeOrScope => "inspect_journey_or_scope",
            Self::CompleteResolutionFormCoverage => "complete_resolution_form_coverage",
            Self::TraceCanonicalRegistry => "trace_canonical_registry",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportedLineOrrHistoryEventExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The line families covered.
    CohortFamilies,
    /// The line journeys carried.
    CohortArchetypes,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The downgrade scopes carried.
    EvidenceScopes,
    /// The render / surface context.
    SurfaceContext,
    /// The line-journey modes carried.
    CohortArchetypeModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5SupportedLineOrrHistoryEventExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::CohortFamilies,
        Self::CohortArchetypes,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::EvidenceScopes,
        Self::SurfaceContext,
        Self::CohortArchetypeModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::CohortFamilies,
        Self::CohortArchetypes,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::CohortFamilies => "line_families",
            Self::CohortArchetypes => "report_sections",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::EvidenceScopes => "comparison_scopes",
            Self::SurfaceContext => "surface_context",
            Self::CohortArchetypeModes => "report_section_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a line-orr_history_event entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, widen-without-rollback, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OrrHistoryEventEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the orr_history_event means.
    DescriptorTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The line journey is unclassified (not in the resolved taxonomy).
    CohortEventClassUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    DescriptorNotBoundToRegistry,
    /// The resolved line-orr_history_event object is incomplete: the exact repo / journey rows, bundle IDs, install
    /// topology, toolchain envelope, known limits, rollback target, or diagnostics posture is unstated.
    CohortDescriptorObjectIncomplete,
    /// The line's rollback and diagnostics posture is not preserved before widening (a line widening without
    /// a rollback target and diagnostics posture), or a public-facing line ran its support language ahead of
    /// line proof.
    DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A public-facing line did not keep its support language matched to line proof before widening.
    RollbackOrDiagnosticsNotPreservedForPublicCohort,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5OrrHistoryEventEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::DescriptorTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::CohortEventClassUnclassified,
        Self::DescriptorNotBoundToRegistry,
        Self::CohortDescriptorObjectIncomplete,
        Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof,
        Self::ResolutionFormCoverageIncomplete,
        Self::RollbackOrDiagnosticsNotPreservedForPublicCohort,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DescriptorTokenUnstated => "orr_history_event_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::CohortEventClassUnclassified => "line_event_class_unclassified",
            Self::DescriptorNotBoundToRegistry => "orr_history_event_not_bound_to_registry",
            Self::CohortDescriptorObjectIncomplete => "orr_history_event_object_incomplete",
            Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof => {
                "orr_history_event_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::RollbackOrDiagnosticsNotPreservedForPublicCohort => {
                "rollback_or_diagnostics_not_preserved_for_public_line"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5SupportedLineOrrHistoryEventNextAction {
        match self {
            Self::DescriptorTokenUnstated | Self::DescriptorNotBoundToRegistry => {
                M5SupportedLineOrrHistoryEventNextAction::TraceCanonicalRegistry
            }
            Self::CohortEventClassUnclassified
            | Self::CohortDescriptorObjectIncomplete
            | Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof => {
                M5SupportedLineOrrHistoryEventNextAction::InspectArchetypeOrScope
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5SupportedLineOrrHistoryEventNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::RollbackOrDiagnosticsNotPreservedForPublicCohort
            | Self::ProofStale => M5SupportedLineOrrHistoryEventNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5SupportedLineTransparencyDowngradeTrigger {
        match self {
            Self::DescriptorTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::ResolutionFormCoverageIncomplete
            | Self::DescriptorNotBoundToRegistry => {
                M5SupportedLineTransparencyDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::CohortEventClassUnclassified | Self::CohortDescriptorObjectIncomplete => {
                M5SupportedLineTransparencyDowngradeTrigger::FreshnessWindowUnstated
            }
            Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof
            | Self::RollbackOrDiagnosticsNotPreservedForPublicCohort => {
                M5SupportedLineTransparencyDowngradeTrigger::WidenedClaimOnStalePublicProof
            }
            Self::ProofStale => M5SupportedLineTransparencyDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a line-downgrade-packet entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FollowUpClosureEntryDegradeReason {
    /// The canonical registry token name is unstated.
    EvidenceTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The downgrade scope is unclassified (not in the resolved taxonomy).
    EvidenceScopeUnclassified,
    /// The line downgrade would run partner / public support language ahead of line proof, hide the line
    /// downgrade, let a known-limits gap masquerade as covered, or it dropped one of the required line-downgrade
    /// fields (line identity, known-limits ledger, rollback target, rehearsal currency, readiness signoff,
    /// support language, last widening revision).
    CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence,
    /// The canonical / accessible / audit resolution-form coverage of the downgrade is incomplete.
    EvidenceFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5FollowUpClosureEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EvidenceTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::EvidenceScopeUnclassified,
        Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence,
        Self::EvidenceFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceTokenUnstated => "comparison_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::EvidenceScopeUnclassified => "comparison_closure_scope_unclassified",
            Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence => {
                "follow_up_closure_runs_support_ahead_of_proof_or_drops_follow_up_closure"
            }
            Self::EvidenceFormCoverageIncomplete => "comparison_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5SupportedLineOrrHistoryEventNextAction {
        match self {
            Self::EvidenceTokenUnstated => {
                M5SupportedLineOrrHistoryEventNextAction::TraceCanonicalRegistry
            }
            Self::EvidenceScopeUnclassified
            | Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence => {
                M5SupportedLineOrrHistoryEventNextAction::InspectArchetypeOrScope
            }
            Self::EvidenceFormCoverageIncomplete => {
                M5SupportedLineOrrHistoryEventNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5SupportedLineOrrHistoryEventNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5SupportedLineTransparencyDowngradeTrigger {
        match self {
            Self::EvidenceTokenUnstated => {
                M5SupportedLineTransparencyDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::EvidenceScopeUnclassified => {
                M5SupportedLineTransparencyDowngradeTrigger::ExportClassUnstated
            }
            Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence => {
                M5SupportedLineTransparencyDowngradeTrigger::RanSupportLanguageAheadOfPublicProof
            }
            Self::EvidenceFormCoverageIncomplete => {
                M5SupportedLineTransparencyDowngradeTrigger::ImpliedGreenWhileProofOrArchiveWasStale
            }
            Self::ProofStale => M5SupportedLineTransparencyDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_orr_history_event_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5OrrHistoryEventEntryResolutionInput {
    /// Stable identity of the line-orr_history_event-registry entry.
    pub entry_id: String,
    /// The stable line-binding ID this orr_history_event binds to (e.g. `launch.line.public-preview`); empty means
    /// unstated.
    pub line_binding_id: String,
    /// The canonical registry token name (e.g. `line.orr_history_event.go_no_go_decision`); empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5SupportedLineTransparencyRole,
    /// The line journey this entry classifies.
    pub report_section: M5OrrHistoryEventKind,
    /// The render / surface context.
    pub surface_context: M5SupportedLineOrrHistoryEventSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5SupportedLineOrrHistoryEventResolutionForm>,
    /// The published exact repo / journey rows; empty means unstated.
    pub exact_repo_journey_rows: String,
    /// The published bundle IDs; empty means unstated.
    pub bundle_ids: String,
    /// The published install topology; empty means unstated.
    pub install_topology: String,
    /// The published toolchain envelope; empty means unstated.
    pub toolchain_envelope: String,
    /// The published known limits; empty means unstated.
    pub known_limits: String,
    /// The published rollback target; empty means unstated.
    pub rollback_target: String,
    /// The published diagnostics posture; empty means unstated.
    pub diagnostics_posture: String,
    /// True when the behavior traces to the line-orr_history_event registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the line's rollback and diagnostics posture is preserved before widening (a hard invariant
    /// when `false`).
    pub rollback_and_diagnostics_bounded: bool,
    /// True when this line's journey is public-facing.
    pub is_public_facing_line: bool,
    /// True when partner / public support language is matched to line proof before a public-facing line
    /// widens.
    pub support_language_matches_line_proof: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe line-orr_history_event-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedOrrHistoryEventEntry {
    /// Stable identity of the line-orr_history_event-registry entry.
    pub entry_id: String,
    /// The stable line-binding ID this orr_history_event binds to.
    pub line_binding_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the downgrade snapshot and signoff before widening.
    pub semantic_role_must_preserve_downgrade_snapshot_and_signoff_before_widening: bool,
    /// The line-journey token named by the entry.
    pub report_section: String,
    /// Whether the line journey is classified into the resolved taxonomy.
    pub report_section_is_classified: bool,
    /// The canonical mode for the entry's line journey.
    pub canonical_report_section_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published exact repo / journey rows.
    pub exact_repo_journey_rows: String,
    /// The published bundle IDs.
    pub bundle_ids: String,
    /// The published install topology.
    pub install_topology: String,
    /// The published toolchain envelope.
    pub toolchain_envelope: String,
    /// The published known limits.
    pub known_limits: String,
    /// The published rollback target.
    pub rollback_target: String,
    /// The published diagnostics posture.
    pub diagnostics_posture: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved line-orr_history_event object publishes every required field.
    pub orr_history_event_object_complete: bool,
    /// Whether the entry traces to the line-orr_history_event registry.
    pub bound_to_registry: bool,
    /// Whether the line's rollback and diagnostics posture stays preserved before widening.
    pub rollback_and_diagnostics_bounded: bool,
    /// Whether this line's journey is public-facing.
    pub is_public_facing_line: bool,
    /// Whether partner / public support language is matched to line proof before widening.
    pub support_language_matches_line_proof: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5OrrHistoryEventEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5SupportedLineOrrHistoryEventNextAction,
    /// Whether the orr_history_event resolves to one typed object across every claimed line (clean entry naming every
    /// fact).
    pub orr_history_event_resolves_across_lines: bool,
}

impl M5ResolvedOrrHistoryEventEntry {
    /// Whether this line-orr_history_event entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_follow_up_closure_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5FollowUpClosureEntryResolutionInput {
    /// Stable identity of the line-downgrade-packet entry.
    pub entry_id: String,
    /// The stable downgrade-ref this record binds to; empty means unstated.
    pub comparison_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5SupportedLineTransparencyRole,
    /// The downgrade scope this record must resolve its line proof from.
    pub comparison_scope: M5FollowUpClosureScope,
    /// The render / surface context.
    pub surface_context: M5SupportedLineOrrHistoryEventSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5SupportedLineOrrHistoryEventResolutionForm>,
    /// The published resolved line identity; empty means missing.
    pub resolved_line_identity: String,
    /// The published known-limits ledger; empty means missing.
    pub known_limits_ledger: String,
    /// The published rollback-target reference; empty means missing.
    pub rollback_target_reference: String,
    /// The published rehearsal-currency state; empty means missing.
    pub rehearsal_currency_state: String,
    /// The published readiness-signoff state; empty means missing.
    pub readiness_signoff_state: String,
    /// The published line-bound support-language reference; empty means missing.
    pub support_language_reference: String,
    /// The published last widening revision; empty means missing.
    pub last_widening_revision: String,
    /// True when the record keeps the line downgrade visible.
    pub keeps_follow_up_closure_visible: bool,
    /// True when the downgrade is truthful (never claims a clean packet over hidden line downgrade).
    pub comparison_is_truthful: bool,
    /// True when partner / public support language is present on this record.
    pub support_language_present: bool,
    /// True when the support language is bound to line proof rather than running ahead of it.
    pub support_language_bound_to_proof: bool,
    /// True when a known-limits gap is present on this record.
    pub known_limits_gap_present: bool,
    /// True when a known-limits gap is flagged rather than masquerading as covered.
    pub known_limits_gap_flagged: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe line-downgrade-packet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedFollowUpClosureEntry {
    /// Stable identity of the line-downgrade-packet entry.
    pub entry_id: String,
    /// The stable downgrade-ref this record binds to.
    pub comparison_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the downgrade snapshot and signoff before widening.
    pub semantic_role_must_preserve_downgrade_snapshot_and_signoff_before_widening: bool,
    /// The downgrade-scope token named by the entry.
    pub comparison_scope: String,
    /// Whether the downgrade scope is classified into the resolved taxonomy.
    pub comparison_scope_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published resolved line identity.
    pub resolved_line_identity: String,
    /// The published known-limits ledger.
    pub known_limits_ledger: String,
    /// The published rollback-target reference.
    pub rollback_target_reference: String,
    /// The published rehearsal-currency state.
    pub rehearsal_currency_state: String,
    /// The published readiness-signoff state.
    pub readiness_signoff_state: String,
    /// The published line-bound support-language reference.
    pub support_language_reference: String,
    /// The published last widening revision.
    pub last_widening_revision: String,
    /// Whether the record keeps the line downgrade visible.
    pub keeps_follow_up_closure_visible: bool,
    /// Whether the downgrade is truthful.
    pub comparison_is_truthful: bool,
    /// Whether partner / public support language is present on this build.
    pub support_language_present: bool,
    /// Whether the support language is bound to line proof rather than running ahead of it.
    pub support_language_bound_to_proof: bool,
    /// Whether a known-limits gap is present on this record.
    pub known_limits_gap_present: bool,
    /// Whether a known-limits gap is flagged rather than masquerading as covered.
    pub known_limits_gap_flagged: bool,
    /// Whether the record stays honest (line downgrade visible, support language bound to proof, known-limits
    /// gap flagged).
    pub follow_up_closure_stays_honest: bool,
    /// Whether the entry provides the complete line-downgrade object (line identity, known-limits ledger,
    /// rollback target, rehearsal currency, readiness signoff, support language, last widening revision).
    pub provides_complete_follow_up_closure: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5FollowUpClosureEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5SupportedLineOrrHistoryEventNextAction,
    /// Whether the line downgrade is safe on every claimed line (clean entry naming every fact).
    pub comparison_safe_on_every_line: bool,
}

impl M5ResolvedFollowUpClosureEntry {
    /// Whether this line-downgrade-packet entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5SupportedLineOrrHistoryEventResolutionError {
    /// The line-orr_history_event-entry id was empty.
    EmptyCohortDescriptorEntryId,
    /// The line-downgrade-packet-entry id was empty.
    EmptyCohortEvidencePacketEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5SupportedLineOrrHistoryEventResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyCohortDescriptorEntryId => "empty_orr_history_event_entry_id",
            Self::EmptyCohortEvidencePacketEntryId => "empty_follow_up_closure_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5SupportedLineOrrHistoryEventResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 line-orr_history_event / line-downgrade-packet registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5SupportedLineOrrHistoryEventResolutionError {}

fn form_tokens(forms: &[M5SupportedLineOrrHistoryEventResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5SupportedLineOrrHistoryEventResolutionForm]) -> bool {
    let present: BTreeSet<M5SupportedLineOrrHistoryEventResolutionForm> =
        forms.iter().copied().collect();
    M5SupportedLineOrrHistoryEventResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved line-orr_history_event object publishes every required field: classified line journey,
/// exact repo / journey rows, bundle IDs, install topology, toolchain envelope, known limits, rollback target,
/// and diagnostics posture. An unclassified journey or any empty field never resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn orr_history_event_object_is_complete(
    journey: M5OrrHistoryEventKind,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> bool {
    journey.is_classified()
        && !exact_repo_journey_rows.trim().is_empty()
        && !bundle_ids.trim().is_empty()
        && !install_topology.trim().is_empty()
        && !toolchain_envelope.trim().is_empty()
        && !known_limits.trim().is_empty()
        && !rollback_target.trim().is_empty()
        && !diagnostics_posture.trim().is_empty()
}

/// Whether the line orr_history_event keeps a line from widening without preserving its rollback and diagnostics
/// posture: the journey must be classified, the rollback and diagnostics posture must be preserved before
/// widening, and a public-facing line must keep its support language matched to line proof. An unclassified
/// journey, an unpreserved rollback / diagnostics posture, or partner / public support language running ahead
/// of proof never matches.
pub fn line_preserves_rollback_and_diagnostics_before_widening(
    journey: M5OrrHistoryEventKind,
    rollback_and_diagnostics_bounded: bool,
    is_public_facing_line: bool,
    support_language_matches_line_proof: bool,
) -> bool {
    journey.is_classified()
        && rollback_and_diagnostics_bounded
        && (!is_public_facing_line || support_language_matches_line_proof)
}

/// Whether a line-downgrade packet stays honest: the scope must be classified, the downgrade must be truthful,
/// it must keep the line downgrade visible, any partner / public support language must be bound to line proof
/// rather than running ahead of it, and any known-limits gap must be flagged rather than masquerade as covered.
pub fn follow_up_closure_stays_honest(
    scope: M5FollowUpClosureScope,
    comparison_is_truthful: bool,
    keeps_follow_up_closure_visible: bool,
    support_language_present: bool,
    support_language_bound_to_proof: bool,
    known_limits_gap_present: bool,
    known_limits_gap_flagged: bool,
) -> bool {
    scope.is_classified()
        && comparison_is_truthful
        && keeps_follow_up_closure_visible
        && (!support_language_present || support_language_bound_to_proof)
        && (!known_limits_gap_present || known_limits_gap_flagged)
}

/// Resolves a line-orr_history_event-registry entry so it stays bound to the line-orr_history_event registry: the entry
/// names its canonical token, semantic role, and line journey, covers all three resolution forms, publishes
/// a complete orr_history_event object (exact repo / journey rows, bundle IDs, install topology, toolchain envelope,
/// known limits, rollback target, diagnostics posture), preserves its rollback and diagnostics posture before
/// widening so a line never widens without it, and keeps a public-facing line's support language matched to
/// line proof.
pub fn resolve_orr_history_event_entry(
    input: M5OrrHistoryEventEntryResolutionInput,
) -> Result<M5ResolvedOrrHistoryEventEntry, M5SupportedLineOrrHistoryEventResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5SupportedLineOrrHistoryEventResolutionError::EmptyCohortDescriptorEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.line_binding_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.exact_repo_journey_rows)
        || string_is_forbidden(&input.bundle_ids)
        || string_is_forbidden(&input.install_topology)
        || string_is_forbidden(&input.toolchain_envelope)
        || string_is_forbidden(&input.known_limits)
        || string_is_forbidden(&input.rollback_target)
        || string_is_forbidden(&input.diagnostics_posture)
    {
        return Err(M5SupportedLineOrrHistoryEventResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = orr_history_event_object_is_complete(
        input.report_section,
        &input.exact_repo_journey_rows,
        &input.bundle_ids,
        &input.install_topology,
        &input.toolchain_envelope,
        &input.known_limits,
        &input.rollback_target,
        &input.diagnostics_posture,
    );
    let preserve_ok = line_preserves_rollback_and_diagnostics_before_widening(
        input.report_section,
        input.rollback_and_diagnostics_bounded,
        input.is_public_facing_line,
        input.support_language_matches_line_proof,
    );
    let support_undisclosed =
        input.is_public_facing_line && !input.support_language_matches_line_proof;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5OrrHistoryEventEntryDegradeReason::DescriptorTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5OrrHistoryEventEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.report_section.is_classified() {
        Some(M5OrrHistoryEventEntryDegradeReason::CohortEventClassUnclassified)
    } else if !input.bound_to_registry {
        Some(M5OrrHistoryEventEntryDegradeReason::DescriptorNotBoundToRegistry)
    } else if !object_complete {
        Some(M5OrrHistoryEventEntryDegradeReason::CohortDescriptorObjectIncomplete)
    } else if !preserve_ok {
        Some(M5OrrHistoryEventEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof)
    } else if !all_forms {
        Some(M5OrrHistoryEventEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if support_undisclosed {
        Some(M5OrrHistoryEventEntryDegradeReason::RollbackOrDiagnosticsNotPreservedForPublicCohort)
    } else if !input.proof_fresh {
        Some(M5OrrHistoryEventEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5SupportedLineOrrHistoryEventNextAction::ExpandCohortMeaning,
    };

    Ok(M5ResolvedOrrHistoryEventEntry {
        entry_id: input.entry_id,
        line_binding_id: input.line_binding_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_downgrade_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_preserve_evidence_snapshot_and_signoff_before_widening(),
        report_section: input.report_section.as_str().to_owned(),
        report_section_is_classified: input.report_section.is_classified(),
        canonical_report_section_mode: input
            .report_section
            .canonical_report_section_mode()
            .to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        exact_repo_journey_rows: input.exact_repo_journey_rows,
        bundle_ids: input.bundle_ids,
        install_topology: input.install_topology,
        toolchain_envelope: input.toolchain_envelope,
        known_limits: input.known_limits,
        rollback_target: input.rollback_target,
        diagnostics_posture: input.diagnostics_posture,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        orr_history_event_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        rollback_and_diagnostics_bounded: input.rollback_and_diagnostics_bounded,
        is_public_facing_line: input.is_public_facing_line,
        support_language_matches_line_proof: input.support_language_matches_line_proof,
        degrade_reason,
        next_action,
        orr_history_event_resolves_across_lines: degrade_reason.is_none(),
    })
}

/// Resolves a line-downgrade-packet entry so its downgrade stays safe: the entry names its canonical token,
/// semantic role, and downgrade scope, covers all three resolution forms, provides the complete line-identity /
/// known-limits-ledger / rollback-target / rehearsal-currency / readiness-signoff / support-language /
/// last-widening-revision line-downgrade object, and degrades honestly when the downgrade would run partner /
/// public support language ahead of line proof, hide the line downgrade, or let a known-limits gap masquerade
/// as covered.
pub fn resolve_follow_up_closure_entry(
    input: M5FollowUpClosureEntryResolutionInput,
) -> Result<M5ResolvedFollowUpClosureEntry, M5SupportedLineOrrHistoryEventResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(
            M5SupportedLineOrrHistoryEventResolutionError::EmptyCohortEvidencePacketEntryId,
        );
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.comparison_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.resolved_line_identity)
        || string_is_forbidden(&input.known_limits_ledger)
        || string_is_forbidden(&input.rollback_target_reference)
        || string_is_forbidden(&input.rehearsal_currency_state)
        || string_is_forbidden(&input.readiness_signoff_state)
        || string_is_forbidden(&input.support_language_reference)
        || string_is_forbidden(&input.last_widening_revision)
    {
        return Err(M5SupportedLineOrrHistoryEventResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let record_stays_honest = follow_up_closure_stays_honest(
        input.comparison_scope,
        input.comparison_is_truthful,
        input.keeps_follow_up_closure_visible,
        input.support_language_present,
        input.support_language_bound_to_proof,
        input.known_limits_gap_present,
        input.known_limits_gap_flagged,
    );
    let provides_record = input.comparison_scope.is_classified()
        && !input.resolved_line_identity.trim().is_empty()
        && !input.known_limits_ledger.trim().is_empty()
        && !input.rollback_target_reference.trim().is_empty()
        && !input.rehearsal_currency_state.trim().is_empty()
        && !input.readiness_signoff_state.trim().is_empty()
        && !input.support_language_reference.trim().is_empty()
        && !input.last_widening_revision.trim().is_empty()
        && record_stays_honest;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5FollowUpClosureEntryDegradeReason::EvidenceTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5FollowUpClosureEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.comparison_scope.is_classified() {
        Some(M5FollowUpClosureEntryDegradeReason::EvidenceScopeUnclassified)
    } else if !provides_record {
        Some(M5FollowUpClosureEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence)
    } else if !all_forms {
        Some(M5FollowUpClosureEntryDegradeReason::EvidenceFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5FollowUpClosureEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5SupportedLineOrrHistoryEventNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedFollowUpClosureEntry {
        entry_id: input.entry_id,
        comparison_ref: input.comparison_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_downgrade_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_preserve_evidence_snapshot_and_signoff_before_widening(),
        comparison_scope: input.comparison_scope.as_str().to_owned(),
        comparison_scope_is_classified: input.comparison_scope.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        resolved_line_identity: input.resolved_line_identity,
        known_limits_ledger: input.known_limits_ledger,
        rollback_target_reference: input.rollback_target_reference,
        rehearsal_currency_state: input.rehearsal_currency_state,
        readiness_signoff_state: input.readiness_signoff_state,
        support_language_reference: input.support_language_reference,
        last_widening_revision: input.last_widening_revision,
        keeps_follow_up_closure_visible: input.keeps_follow_up_closure_visible,
        comparison_is_truthful: input.comparison_is_truthful,
        support_language_present: input.support_language_present,
        support_language_bound_to_proof: input.support_language_bound_to_proof,
        known_limits_gap_present: input.known_limits_gap_present,
        known_limits_gap_flagged: input.known_limits_gap_flagged,
        follow_up_closure_stays_honest: record_stays_honest,
        provides_complete_follow_up_closure: provides_record,
        degrade_reason,
        next_action,
        comparison_safe_on_every_line: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved line-orr_history_event and line-downgrade-packet
/// entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5SupportedLineTransparencyQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Widening stages this row keeps the same truth across.
    pub widening_stages: Vec<M5SupportedLineTransparencyWideningStage>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5SupportedLineTransparencyRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5SupportedLineTransparencyAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5SupportedLineOrrHistoryEventAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5SupportedLineOrrHistoryEventExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5SupportedLineTransparencyDowngradeTrigger>,
    /// Resolved line-orr_history_event-registry examples.
    pub orr_history_event_entries: Vec<M5ResolvedOrrHistoryEventEntry>,
    /// Resolved line-downgrade-packet examples.
    pub follow_up_closure_entries: Vec<M5ResolvedFollowUpClosureEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the line-orr_history_event and
    /// line-downgrade-packet domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never widens a line without current rollback and diagnostics downgrade. MUST be
    /// `false`.
    pub widens_a_line_without_current_rollback_and_diagnostics_downgrade: bool,
    /// Hard invariant: this row never runs partner or public support language ahead of line proof. MUST be
    /// `false`.
    pub runs_partner_or_public_support_language_ahead_of_line_proof: bool,
    /// Hard invariant: this row never hides the rollback target or diagnostics posture before widening. MUST be
    /// `false`.
    pub hides_the_rollback_target_or_diagnostics_posture_before_widening: bool,
    /// Hard invariant: this row never collapses distinct line downgrade classes into one lane. MUST be `false`.
    pub collapses_distinct_follow_up_closure_classes_into_one_lane: bool,
}

impl M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5SupportedLineOrrHistoryEventAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5SupportedLineOrrHistoryEventAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5SupportedLineOrrHistoryEventExportField> =
            self.export_fields.iter().copied().collect();
        M5SupportedLineOrrHistoryEventExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.widens_a_line_without_current_rollback_and_diagnostics_downgrade
            && !self.runs_partner_or_public_support_language_ahead_of_line_proof
            && !self.hides_the_rollback_target_or_diagnostics_posture_before_widening
            && !self.collapses_distinct_follow_up_closure_classes_into_one_lane
    }

    /// True when a clean line-orr_history_event entry preserves registry-bound truth: it traces to the registry,
    /// keeps a classified line journey, publishes a complete orr_history_event object, preserves its rollback and
    /// diagnostics posture, covers all three resolution forms, and keeps a public-facing line's support
    /// language matched to proof.
    fn orr_history_event_is_honest(ex: &M5ResolvedOrrHistoryEventEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.report_section_is_classified
                && ex.orr_history_event_object_complete
                && ex.rollback_and_diagnostics_bounded
                && ex.covers_all_resolution_forms
                && (!ex.is_public_facing_line || ex.support_language_matches_line_proof))
    }

    /// True when a clean line-downgrade-packet entry preserves a safe packet: it keeps a classified downgrade
    /// scope, provides the complete line-downgrade object, stays honest, and covers all three resolution forms.
    fn downgrade_is_honest(ex: &M5ResolvedFollowUpClosureEntry) -> bool {
        !ex.is_clean()
            || (ex.comparison_scope_is_classified
                && ex.provides_complete_follow_up_closure
                && ex.follow_up_closure_stays_honest
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.orr_history_event_entries
            .iter()
            .all(Self::orr_history_event_is_honest)
            && self
                .follow_up_closure_entries
                .iter()
                .all(Self::downgrade_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Cohort-journey tokens (minted by this lane).
    pub report_section_kinds: Vec<String>,
    /// Evidence-scope tokens (minted by this lane).
    pub comparison_scopes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Cohort-orr_history_event-entry degrade-reason tokens.
    pub orr_history_event_degrade_reasons: Vec<String>,
    /// Cohort-downgrade-packet-entry degrade-reason tokens.
    pub follow_up_closure_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5SupportedLineTransparencyRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5SupportedLineOrrHistoryEventResolutionForm::ALL, |v| {
                v.as_str()
            }),
            report_section_kinds: tokens(&M5OrrHistoryEventKind::ALL, |v| v.as_str()),
            comparison_scopes: tokens(&M5FollowUpClosureScope::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5SupportedLineOrrHistoryEventSurfaceContext::ALL, |v| {
                v.as_str()
            }),
            orr_history_event_degrade_reasons: tokens(
                &M5OrrHistoryEventEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            follow_up_closure_degrade_reasons: tokens(
                &M5FollowUpClosureEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5SupportedLineOrrHistoryEventAnatomyPart::ALL, |v| {
                v.as_str()
            }),
            next_actions: tokens(&M5SupportedLineOrrHistoryEventNextAction::ALL, |v| {
                v.as_str()
            }),
            export_fields: tokens(&M5SupportedLineOrrHistoryEventExportField::ALL, |v| {
                v.as_str()
            }),
            consumer_surfaces: tokens(&M5SupportedLineTransparencyConsumerSurface::ALL, |v| {
                v.as_str()
            }),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesGovernanceReview {
    /// The orr_history_event registry names a canonical token, semantic role, and line journey for every entry.
    pub orr_history_event_registry_names_token_role_and_journey: bool,
    /// Every claimed line resolves to one typed line-orr_history_event object from the shared registry, not
    /// per-entry reconstruction.
    pub line_resolves_to_typed_orr_history_event_from_shared_registry: bool,
    /// The exact repo / journey rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles are published for every resolved orr_history_event.
    pub repo_bundle_toolchain_and_deployment_rows_published: bool,
    /// Cohorts cannot widen without preserving rollback and diagnostics posture before widening.
    pub lines_cannot_widen_without_rollback_and_diagnostics: bool,
    /// The line downgrade keeps the line proof visible and binds partner / public support language to it.
    pub follow_up_closure_keeps_proof_visible_and_binds_support_language: bool,
    /// Partner / public support language stays matched to line proof for every public-facing line.
    pub support_language_matched_to_line_proof_for_public_lines: bool,
    /// Every line-orr_history_event and line-downgrade-packet entry covers the canonical / accessible / audit
    /// resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Cohort-orr_history_event and line-downgrade-packet behavior stay bound to the shared registries rather than
    /// hand-copied per line.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Shiproom, release center, executive steering, and program governance read a single line source.
    pub shiproom_release_center_executive_steering_and_program_governance_read_single_source: bool,
    /// A widen-without-rollback attempt, an incomplete object, or hidden line downgrade is caught by fixtures
    /// before release downgrade turns green.
    pub orr_history_event_or_downgrade_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesConsumerProjection {
    /// Shiproom and release center consume the shared line-orr_history_event registry.
    pub shiproom_and_release_center_consume_shared_registries: bool,
    /// Executive steering and program governance consume the shared line-downgrade registry.
    pub executive_steering_and_program_governance_consume_shared_registries: bool,
    /// Diagnostics and public proof consume the shared registries.
    pub diagnostics_and_public_proof_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical line-orr_history_event and line-downgrade-packet domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical line-orr_history_event / line-downgrade-packet registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting line audit for the lane.
    pub line_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection:
        M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 line-orr_history_event and line-downgrade-packet registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket {
    /// Record kind; must equal [`M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection:
        M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacketInput) -> Self {
        Self {
            record_kind:
                M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_RECORD_KIND
                    .to_owned(),
            schema_version:
                M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_SCHEMA_VERSION,
            packet_id: input.packet_id,
            registries_label: input.registries_label,
            registry_rows: input.registry_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the registries-packet invariants.
    pub fn validate(
        &self,
    ) -> Vec<M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind
            != M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_RECORD_KIND
        {
            violations.push(
                M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::WrongRecordKind,
            );
        }
        if self.schema_version
            != M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_SCHEMA_VERSION
        {
            violations.push(
                M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::WrongSchemaVersion,
            );
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(
                M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::MissingIdentity,
            );
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(
                M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::VocabularySetDrift,
            );
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(&serde_json::to_value(self).expect(
            "m5 line-orr_history_event / line-downgrade-packet registries packet serializes",
        )) {
            violations.push(
                M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::RawMaterialInExport,
            );
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect(
            "m5 line-orr_history_event / line-downgrade-packet registries packet serializes",
        )
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,orr_history_event_entries,follow_up_closure_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .orr_history_event_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.follow_up_closure_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.orr_history_event_entries.len(),
                row.follow_up_closure_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 ORR-History-Event and Follow-Up-Closure Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Event classes: {}\n",
            self.vocabulary_set.report_section_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Resolution forms: {}\n",
            self.vocabulary_set.resolution_forms.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last audit: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.registry_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - ORR-history-event entries: {} / follow-up-closure entries: {}\n",
                row.orr_history_event_entries.len(),
                row.follow_up_closure_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry line reference table generated from the registry, so docs and shiproom runbooks
    /// render the same journey-mode / repo-rows / bundle-ids / install-topology / toolchain-envelope /
    /// rollback-target truth the resolvers produced rather than a hand-copied line table. Only clean,
    /// registry-bound line-orr_history_event entries are listed.
    pub fn render_orr_history_event_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| line_binding_id | journey_mode | exact_repo_journey_rows | bundle_ids | install_topology | toolchain_envelope | rollback_target |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.orr_history_event_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.line_binding_id,
                    ex.canonical_report_section_mode,
                    ex.exact_repo_journey_rows,
                    ex.bundle_ids,
                    ex.install_topology,
                    ex.toolchain_envelope,
                    ex.rollback_target
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation>),
}

impl fmt::Display for M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 line-orr_history_event / line-downgrade-packet registries export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 line-orr_history_event / line-downgrade-packet registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesArtifactError {}

/// Validation failures emitted by [`M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// The registries packet declares no rows.
    NoRegistryRows,
    /// A registry row is incomplete.
    RegistryRowIncomplete,
    /// A registry row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A registry row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A registry row does not point at both the line-orr_history_event and line-downgrade-packet domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, widen-without-rollback, field-incomplete,
    /// form-incomplete, or a line-downgrade entry missing the complete downgrade object).
    DishonestExample,
    /// A registry row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Cohort-orr_history_event-resolution is not proven: clean orr_history_event entries do not cover the canonical line
    /// journeys or the first release-center / shiproom / executive-steering / program-governance / support
    /// surfaces, no object-incomplete example degrades, or a clean orr_history_event entry published an incomplete
    /// object.
    CohortDescriptorResolutionNotProven,
    /// Rollback-and-diagnostics-preservation is not proven: no widen-without-rollback example and no unbound
    /// example degrade, no clean bounded orr_history_event entry is present, or a clean orr_history_event entry is unbounded
    /// or unbound.
    RollbackAndDiagnosticsPreservationNotProven,
    /// Cohort-downgrade-integrity is not proven: clean downgrade entries do not cover the canonical Retest-pending /
    /// Evidence-stale / narrowed-support scopes with full resolution-form coverage while providing the
    /// complete downgrade object, no support-ahead or form-incomplete example degrades, or a clean downgrade entry
    /// is missing the complete downgrade object.
    CohortEvidenceIntegrityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoRegistryRows => "no_registry_rows",
            Self::RegistryRowIncomplete => "registry_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::CohortDescriptorResolutionNotProven => "orr_history_event_resolution_not_proven",
            Self::RollbackAndDiagnosticsPreservationNotProven => {
                "rollback_and_diagnostics_preservation_not_proven"
            }
            Self::CohortEvidenceIntegrityNotProven => "follow_up_closure_integrity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_supported_line_orr_history_and_follow_up_closure_registries_export(
) -> Result<
    M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket,
    M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesArtifactError,
> {
    let packet: M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-supported-line-orr-history-and-follow-up-closure-registries-proof/support_export.json"
        )
    ))
    .map_err(M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(
            M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesArtifactError::Validation(
                violations,
            ),
        )
    }
}

fn validate_source_contracts(
    packet: &M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket,
    violations: &mut Vec<M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_SCHEMA_REF,
        M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_DOC_REF,
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_REF,
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_DOC_REF,
        M5_SUPPORTED_LINE_ORR_HISTORY_DOMAIN_SCHEMA_REF,
        M5_FOLLOW_UP_CLOSURE_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(
                M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::MissingSourceContracts,
            );
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket,
    violations: &mut Vec<M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations
            .push(M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::NoRegistryRows);
        return;
    }
    for row in &packet.registry_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.widening_stages.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(
                M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::RegistryRowIncomplete,
            );
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(
                M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::MandatoryAnatomyMissing,
            );
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_SUPPORTED_LINE_ORR_HISTORY_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_FOLLOW_UP_CLOSURE_DOMAIN_SCHEMA_REF)
        {
            violations.push(
                M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::DomainSchemaRefMissing,
            );
        }
        if row.orr_history_event_entries.is_empty() || row.follow_up_closure_entries.is_empty() {
            violations.push(
                M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::ExamplesMissing,
            );
        }
        if !row.examples_are_honest() {
            violations.push(
                M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::DishonestExample,
            );
        }
        if !row.honours_invariants() {
            violations.push(
                M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::RowInvariantViolated,
            );
        }
    }
}

fn validate_governance_review(
    packet: &M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket,
    violations: &mut Vec<M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.orr_history_event_registry_names_token_role_and_journey,
        review.line_resolves_to_typed_orr_history_event_from_shared_registry,
        review.repo_bundle_toolchain_and_deployment_rows_published,
        review.lines_cannot_widen_without_rollback_and_diagnostics,
        review.follow_up_closure_keeps_proof_visible_and_binds_support_language,
        review.support_language_matched_to_line_proof_for_public_lines,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.shiproom_release_center_executive_steering_and_program_governance_read_single_source,
        review.orr_history_event_or_downgrade_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket,
    violations: &mut Vec<M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shiproom_and_release_center_consume_shared_registries,
        projection.executive_steering_and_program_governance_consume_shared_registries,
        projection.diagnostics_and_public_proof_consume_shared_registries,
        projection.docs_help_and_cli_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(
                M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket,
    violations: &mut Vec<M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(
            M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::ProofFreshnessIncomplete,
        );
    }
}

fn validate_release_posture(
    packet: &M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket,
    violations: &mut Vec<M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.line_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(
            M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::ReleasePostureIncomplete,
        );
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket,
    violations: &mut Vec<M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation>,
) {
    let orr_history_events = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.orr_history_event_entries.iter())
    };
    let downgrade = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.follow_up_closure_entries.iter())
    };

    // AC1: every active line can be inspected by exact rows, bundles, toolchains, and deployment profiles.
    // Clean orr_history_event entries cover the canonical line journeys and the first release-center / shiproom /
    // executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and no
    // clean orr_history_event entry published an incomplete object.
    let clean_journeys: BTreeSet<String> = orr_history_events()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.report_section.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = orr_history_events()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let journeys_covered = M5OrrHistoryEventKind::CANONICAL_JOURNEYS
        .iter()
        .all(|k| clean_journeys.contains(k.as_str()));
    let first_surfaces_covered = M5SupportedLineOrrHistoryEventSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = orr_history_events().any(|ex| {
        ex.degrade_reason
            == Some(M5OrrHistoryEventEntryDegradeReason::CohortDescriptorObjectIncomplete)
    });
    let no_clean_incomplete =
        !orr_history_events().any(|ex| ex.is_clean() && !ex.orr_history_event_object_complete);
    if !(journeys_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::CohortDescriptorResolutionNotProven,
        );
    }

    // AC2: line packets preserve rollback and diagnostics posture before widening. A widen-without-rollback
    // example degrades, an unbound example degrades, at least one clean bounded orr_history_event entry is present, and
    // no clean orr_history_event entry is unbounded or unbound.
    let widen_fold_degrades = orr_history_events().any(|ex| {
        ex.degrade_reason
            == Some(
                M5OrrHistoryEventEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof,
            )
    });
    let unbound_degrades = orr_history_events().any(|ex| {
        ex.degrade_reason == Some(M5OrrHistoryEventEntryDegradeReason::DescriptorNotBoundToRegistry)
    });
    let bounded_clean_orr_history_event =
        orr_history_events().any(|ex| ex.is_clean() && ex.rollback_and_diagnostics_bounded);
    let no_clean_unbound = !orr_history_events().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_unbounded =
        !orr_history_events().any(|ex| ex.is_clean() && !ex.rollback_and_diagnostics_bounded);
    if !(widen_fold_degrades
        && unbound_degrades
        && bounded_clean_orr_history_event
        && no_clean_unbound
        && no_clean_unbounded)
    {
        violations.push(
            M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::RollbackAndDiagnosticsPreservationNotProven,
        );
    }

    // AC3: claim publication can prove which line downgrade backs each launch-bearing lane. Clean downgrade
    // entries cover every canonical Retest-pending / Evidence-stale / narrowed-support scope with full
    // resolution-form coverage while providing the complete downgrade object, a support-ahead example degrades, a
    // form-incomplete example degrades, and no clean downgrade entry is missing the complete object.
    let clean_comparison_scopes: BTreeSet<String> = downgrade()
        .filter(|ex| {
            ex.is_clean()
                && ex.comparison_scope_is_classified
                && ex.provides_complete_follow_up_closure
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.comparison_scope.clone())
        .collect();
    let comparison_scopes_covered = M5FollowUpClosureScope::CANONICAL_SCOPES
        .iter()
        .all(|m| clean_comparison_scopes.contains(m.as_str()));
    let support_ahead_degrades = downgrade().any(|ex| {
        ex.degrade_reason
            == Some(
                M5FollowUpClosureEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence,
            )
    });
    let form_incomplete_degrades = downgrade().any(|ex| {
        ex.degrade_reason
            == Some(M5FollowUpClosureEntryDegradeReason::EvidenceFormCoverageIncomplete)
    });
    let no_clean_missing_downgrade =
        !downgrade().any(|ex| ex.is_clean() && !ex.provides_complete_follow_up_closure);
    if !(comparison_scopes_covered
        && support_ahead_degrades
        && form_incomplete_degrades
        && no_clean_missing_downgrade)
    {
        violations.push(
            M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesViolation::CohortEvidenceIntegrityNotProven,
        );
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The launch-bearing lines this lane implements, for downstream reference: the line-orr_history_event registry
/// covers the core-team canary, design-partner preview, extension-author, public preview, and certified-journey
/// lines the frozen matrix froze, and the line-downgrade-packet registry binds the downgrade that backs each.
pub const IMPLEMENTED_LINES: [M5SupportedLineTransparencyObject; 5] =
    M5SupportedLineTransparencyObject::ALL;
