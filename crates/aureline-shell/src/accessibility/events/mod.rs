//! Event-class non-visual coverage for the claimed M5 dynamic workflows.
//!
//! Where the live-announcement grammar
//! ([`crate::announcement_grammar`]) governs *how* a dynamic event is narrated —
//! its message template, channel, coalescing budget, and durable fallback — and the
//! frozen dynamic-surface matrix
//! ([`crate::freeze_the_m5_accessibility_bridge_live_announcement_focus_return_and_non_visual_dynamic_surface_matrix`])
//! governs *which* controlled vocabularies an accessibility object may carry, this
//! module materializes *which concrete dynamic events* each high-churn M5 workflow
//! must narrate. One [`M5EventFamilyCoverage`] row per governed event family —
//! diagnostics, completion/snippet/session changes, run/debug/test transitions,
//! terminal command boundaries, collaboration control/recording changes, AI
//! patch/review milestones, and stale/degraded-truth transitions — enumerates the
//! [`M5DynamicEventMapping`]s a professional user follows non-visually, and binds
//! each event to the announcement grammar class it narrates through, a stable
//! concise-identity message id, whether it discloses a blocked/degraded reason, and
//! the durable fallback surface that preserves the same identity and state labels the
//! user heard.
//!
//! The catalog is the single M5 source for assistive-tech *event* truth: editor,
//! terminal, debug, review, collaboration, AI, and notebook surfaces route their
//! dynamic events through these mappings rather than improvising per-surface
//! announcements; diagnostics, support exports, docs/help, and assistive-tech
//! conformance packets reuse the same mappings so a dynamic-narration regression is
//! debuggable from the support export alone. Only meaning-changing dynamic events
//! belong in the assistive channel: every covered event is `meaning_changing`, every
//! family can announce a concise identity plus a blocked/degraded reason, and each
//! event links to a reopenable durable fallback. When a family's bridge or proof
//! state goes stale the claimed coverage auto-narrows rather than implying silent
//! screen-reader completeness.
//!
//! The controlled event-class and durable-fallback-surface vocabularies are reused
//! verbatim from the announcement grammar, and the shared state vocabularies from the
//! frozen matrix, rather than minting parallel tokens. Only the coverage-shaped
//! vocabularies this lane adds (event family, event producer, and blocked/degraded
//! reason class) are minted here and frozen in a self-describing
//! [`M5EventCoverageVocabularySet`]. Raw provider payloads, credentials, secret
//! material, screenshots, and untranslated free-text prose stay outside the support
//! boundary.
//!
//! The boundary schema is
//! [`schemas/a11y/m5-event-coverage.schema.json`](../../../../../schemas/a11y/m5-event-coverage.schema.json).
//! The contract doc is
//! [`docs/a11y/m5-event-coverage.md`](../../../../../docs/a11y/m5-event-coverage.md).
//! The protected fixture directory is
//! [`fixtures/a11y/m5-event-coverage/`](../../../../../fixtures/a11y/m5-event-coverage/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_event_coverage_catalog, seeded_m5_event_coverage_catalog_bridge_unavailable_narrowed,
    seeded_m5_event_coverage_catalog_proof_stale_narrowed, M5_EVENT_COVERAGE_CATALOG_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The live-announcement grammar owns the canonical event-class and
// durable-fallback-surface vocabularies; route every covered event through them
// rather than minting parallel synonyms.
use crate::announcement_grammar as grammar;
// The frozen matrix owns the shared state vocabularies, qualification classes,
// downgrade triggers, consumer surfaces, and proof/release posture.
use crate::freeze_the_m5_accessibility_bridge_live_announcement_focus_return_and_non_visual_dynamic_surface_matrix as matrix;

pub use grammar::{
    M5AnnouncementEventClass, M5AnnouncementGrammarVocabularySet, M5DurableFallbackRef,
    M5DurableFallbackSurface,
};
pub use matrix::{
    A11yAnnouncementPoliteness, A11yNonVisualFidelity, M5DynamicSurfaceA11yConsumerSurface,
    M5DynamicSurfaceA11yDowngradeTrigger, M5DynamicSurfaceA11yProofFreshness,
    M5DynamicSurfaceA11yQualificationClass, M5DynamicSurfaceA11yReleasePosture,
    M5DynamicSurfaceA11yVocabularySet,
};

/// Stable record-kind tag carried by [`M5EventCoverageCatalogPacket`].
pub const M5_EVENT_COVERAGE_RECORD_KIND: &str = "m5_event_class_coverage_catalog";

/// Schema version for M5 event-class coverage catalogs.
pub const M5_EVENT_COVERAGE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_EVENT_COVERAGE_SCHEMA_REF: &str = "schemas/a11y/m5-event-coverage.schema.json";

/// Repo-relative path of the M5 event-coverage contract doc.
pub const M5_EVENT_COVERAGE_DOC_REF: &str = "docs/a11y/m5-event-coverage.md";

/// Repo-relative path of the frozen dynamic-surface accessibility matrix that
/// governs this lane's shared controlled vocabularies and qualification classes.
pub const M5_EVENT_COVERAGE_MATRIX_REF: &str = "schemas/a11y/m5-dynamic-surface-a11y.schema.json";

/// Repo-relative path of the live-announcement grammar this lane routes through.
pub const M5_EVENT_COVERAGE_ANNOUNCEMENT_GRAMMAR_REF: &str =
    "schemas/a11y/m5-announcement-grammar.schema.json";

/// Repo-relative path of the per-surface accessibility descriptors this lane's
/// durable fallbacks resolve against.
pub const M5_EVENT_COVERAGE_SURFACE_DESCRIPTOR_REF: &str =
    "schemas/a11y/m5-surface-descriptors.schema.json";

/// Repo-relative path of the frozen screen-reader announcement / live-region
/// contract.
pub const M5_EVENT_COVERAGE_SCREEN_READER_CONTRACT_REF: &str =
    "docs/accessibility/screen_reader_and_live_region_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_EVENT_COVERAGE_FIXTURE_DIR: &str = "fixtures/a11y/m5-event-coverage";

/// Repo-relative path of the checked support-export artifact.
pub const M5_EVENT_COVERAGE_ARTIFACT_REF: &str =
    "artifacts/a11y/m5-event-coverage-proof/support_export.json";

/// Repo-relative path of the checked Markdown governance summary.
pub const M5_EVENT_COVERAGE_SUMMARY_REF: &str =
    "artifacts/a11y/m5-event-coverage-proof/event-coverage-proof.md";

/// Stable prefix every concise-identity event message id carries.
pub const M5_EVENT_IDENTITY_MESSAGE_ID_PREFIX: &str = "event.";

/// One governed family of M5 dynamic events the coverage catalog must narrate.
///
/// These are exactly the high-churn workflows whose dynamic state defines
/// professional work: diagnostics, editor assist (completion / snippet / session),
/// run/debug/test transitions, terminal command boundaries, collaboration
/// control/recording changes, AI patch/review milestones, and stale/degraded-truth
/// transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EventFamily {
    /// Diagnostics published, updated, blocking, or cleared.
    Diagnostics,
    /// Completion list, snippet session, and editor-assist session changes.
    CompletionAndSession,
    /// Run, debug, and test state transitions.
    RunDebugTest,
    /// Terminal command boundaries (start / exit), where shell integration allows.
    TerminalBoundary,
    /// Collaboration control and recording changes.
    CollaborationControl,
    /// AI patch and review milestone states.
    AiPatchReview,
    /// Stale and degraded truth transitions across surfaces.
    StaleDegradedTruth,
}

impl M5EventFamily {
    /// Every governed event family, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Diagnostics,
        Self::CompletionAndSession,
        Self::RunDebugTest,
        Self::TerminalBoundary,
        Self::CollaborationControl,
        Self::AiPatchReview,
        Self::StaleDegradedTruth,
    ];

    /// Stable token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Diagnostics => "diagnostics",
            Self::CompletionAndSession => "completion_and_session",
            Self::RunDebugTest => "run_debug_test",
            Self::TerminalBoundary => "terminal_boundary",
            Self::CollaborationControl => "collaboration_control",
            Self::AiPatchReview => "ai_patch_review",
            Self::StaleDegradedTruth => "stale_degraded_truth",
        }
    }
}

/// Producer crate that originates the events in a coverage family.
///
/// These are the first real event producers needed to prove end-to-end coverage;
/// each maps to a claimed M5 surface that emits meaning-changing dynamic state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EventProducer {
    /// Custom-rendered editor (`aureline-editor`).
    Editor,
    /// Terminal / log canvas (`aureline-terminal`).
    Terminal,
    /// Debugger host (`aureline-debug`).
    Debug,
    /// Review / diff surface (`aureline-review`).
    Review,
    /// Collaboration session (`aureline-collab`).
    Collab,
    /// AI assist / patch surface (`aureline-ai`).
    Ai,
    /// Notebook surface (`aureline-notebook`).
    Notebook,
    /// Shell host that aggregates cross-surface stale/degraded truth.
    Shell,
}

impl M5EventProducer {
    /// Every producer, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Editor,
        Self::Terminal,
        Self::Debug,
        Self::Review,
        Self::Collab,
        Self::Ai,
        Self::Notebook,
        Self::Shell,
    ];

    /// Stable token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Terminal => "terminal",
            Self::Debug => "debug",
            Self::Review => "review",
            Self::Collab => "collab",
            Self::Ai => "ai",
            Self::Notebook => "notebook",
            Self::Shell => "shell",
        }
    }
}

/// Class of blocked/degraded reason a dynamic event can disclose.
///
/// `not_applicable` is a normal meaning-changing transition (entered a mode, reached
/// a milestone); every other token names *why* the workflow state is blocked or
/// degraded so the announcement carries the reason rather than just the surface name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EventReasonClass {
    /// A normal transition; no blocked/degraded reason is disclosed.
    NotApplicable,
    /// The action is blocked and the reason is named (assertive interruption).
    Blocked,
    /// A capability is degraded but still partially available.
    Degraded,
    /// Truth has gone stale past its freshness floor.
    Stale,
    /// A capability is unavailable on this target / build.
    Unavailable,
    /// A policy or trust restriction blocks the state.
    PolicyRestricted,
}

impl M5EventReasonClass {
    /// Every reason class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotApplicable,
        Self::Blocked,
        Self::Degraded,
        Self::Stale,
        Self::Unavailable,
        Self::PolicyRestricted,
    ];

    /// Stable token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::Blocked => "blocked",
            Self::Degraded => "degraded",
            Self::Stale => "stale",
            Self::Unavailable => "unavailable",
            Self::PolicyRestricted => "policy_restricted",
        }
    }

    /// True when this reason class discloses a real blocked/degraded condition.
    pub const fn discloses_reason(self) -> bool {
        !matches!(self, Self::NotApplicable)
    }

    /// The announcement event class this reason must narrate through.
    ///
    /// A hard blocker interrupts assertively via
    /// [`M5AnnouncementEventClass::BlockerRaised`]; every degraded/stale/unavailable/
    /// policy reason narrates politely via
    /// [`M5AnnouncementEventClass::DegradedOrStaleTruth`]. `not_applicable` carries no
    /// required class because a normal transition may use any non-degraded class.
    pub const fn required_announcement_class(self) -> Option<M5AnnouncementEventClass> {
        match self {
            Self::NotApplicable => None,
            Self::Blocked => Some(M5AnnouncementEventClass::BlockerRaised),
            Self::Degraded | Self::Stale | Self::Unavailable | Self::PolicyRestricted => {
                Some(M5AnnouncementEventClass::DegradedOrStaleTruth)
            }
        }
    }
}

/// Concise-identity-plus-reason disclosure for one dynamic event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EventDegradedDisclosure {
    /// True when the announcement carries a blocked/degraded reason.
    pub announces_reason: bool,
    /// The blocked/degraded reason class disclosed.
    pub reason_class: M5EventReasonClass,
}

/// One concrete dynamic event a workflow narrates non-visually.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DynamicEventMapping {
    /// Stable event id, unique within the catalog.
    pub event_id: String,
    /// Human-readable event label.
    pub label: String,
    /// True when this event changes meaning (guardrail: low-value ticks are barred).
    pub meaning_changing: bool,
    /// Announcement grammar class this event narrates through (grammar-owned).
    pub announcement_event_class: M5AnnouncementEventClass,
    /// Stable concise-identity message id; carries the
    /// [`M5_EVENT_IDENTITY_MESSAGE_ID_PREFIX`].
    pub identity_message_id: String,
    /// Concise-identity-plus-reason disclosure.
    pub degraded_disclosure: M5EventDegradedDisclosure,
    /// Durable fallback surface that preserves this event's identity (grammar-owned).
    pub durable_fallback: M5DurableFallbackRef,
}

impl M5DynamicEventMapping {
    /// The live-region channel this event speaks on, derived from its grammar class.
    pub fn channel(&self) -> A11yAnnouncementPoliteness {
        if self
            .announcement_event_class
            .required_channel_is_assertive()
        {
            A11yAnnouncementPoliteness::Assertive
        } else {
            A11yAnnouncementPoliteness::Polite
        }
    }
}

/// One event-family coverage row for a claimed M5 workflow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EventFamilyCoverage {
    /// Stable family id, unique within the catalog.
    pub family_id: String,
    /// Governed event family.
    pub family: M5EventFamily,
    /// Human-readable family label.
    pub label: String,
    /// Owner role accountable for keeping this family's coverage current.
    pub owner_role: String,
    /// Qualification class earned by this family's coverage.
    pub qualification: M5DynamicSurfaceA11yQualificationClass,
    /// Non-visual fidelity the family's events currently deliver (matrix-owned).
    pub non_visual_fidelity: A11yNonVisualFidelity,
    /// Producer crates that originate these events.
    pub producers: Vec<M5EventProducer>,
    /// Concrete dynamic events covered by this family.
    pub events: Vec<M5DynamicEventMapping>,
    /// Downgrade triggers that can narrow this family below its claim.
    pub downgrade_triggers: Vec<M5DynamicSurfaceA11yDowngradeTrigger>,
    /// Assistive-tech proof packet refs that keep this family current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this family.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that project this family's coverage truth.
    pub consumer_surfaces: Vec<M5DynamicSurfaceA11yConsumerSurface>,
}

impl M5EventFamilyCoverage {
    /// True when at least one event in the family discloses a blocked/degraded reason.
    fn can_announce_degraded_reason(&self) -> bool {
        self.events
            .iter()
            .any(|event| event.degraded_disclosure.announces_reason)
    }
}

/// Self-describing controlled-vocabulary set for the coverage-shaped tokens this lane
/// mints (the event-class and durable-fallback tokens live in the grammar; the shared
/// state tokens live in the matrix).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EventCoverageVocabularySet {
    /// Event-family tokens.
    pub event_families: Vec<String>,
    /// Event-producer tokens.
    pub event_producers: Vec<String>,
    /// Blocked/degraded reason-class tokens.
    pub reason_classes: Vec<String>,
}

impl M5EventCoverageVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            event_families: M5EventFamily::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            event_producers: M5EventProducer::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            reason_classes: M5EventReasonClass::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Assistive-technology conformance review block for the event-coverage lane.
///
/// Every flag is a hard invariant; all must hold for the catalog to validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EventCoverageConformanceReview {
    /// Claimed workflows narrate their state transitions without visual-only cues.
    pub workflows_narrate_transitions_without_visual_only_cues: bool,
    /// Each family can announce a concise identity plus a blocked/degraded reason.
    pub each_family_announces_identity_plus_blocked_or_degraded_reason: bool,
    /// Events route through one announcement grammar, not per-surface prose.
    pub events_route_through_one_announcement_grammar_not_per_surface_prose: bool,
    /// Durable fallbacks preserve the event identity and state labels heard.
    pub durable_fallback_preserves_event_identity_and_state_labels: bool,
    /// Only meaning-changing dynamic events enter the assistive channel.
    pub only_meaning_changing_events_enter_assistive_channel: bool,
    /// Support exports can reconstruct what the user should have been told.
    pub support_export_can_reconstruct_what_user_should_have_been_told: bool,
    /// Claimed families auto-narrow when bridge or proof state goes stale.
    pub claimed_families_auto_narrow_when_bridge_or_proof_stale: bool,
    /// Downgrade narrows the claim rather than hiding the family.
    pub downgrade_narrows_instead_of_hides: bool,
    /// No event sources its truth from a pixel-only render or pointer-only cue.
    pub no_visual_only_or_pointer_only_event_source: bool,
}

/// Consumer projection block: who routes events through the coverage catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EventCoverageConsumerProjection {
    /// Editor routes diagnostics and assist events through the coverage catalog.
    pub editor_routes_diagnostics_and_assist: bool,
    /// Terminal routes command-boundary events through the coverage catalog.
    pub terminal_routes_command_boundaries: bool,
    /// Debug and test transitions route through the coverage catalog.
    pub debug_and_test_route_transitions: bool,
    /// Review and AI milestones route through the coverage catalog.
    pub review_and_ai_route_milestones: bool,
    /// Collaboration control / recording changes route through the catalog.
    pub collaboration_routes_control_changes: bool,
    /// Notebook session changes route through the coverage catalog.
    pub notebook_routes_session_changes: bool,
    /// Shell narrates cross-surface stale/degraded truth through the catalog.
    pub shell_routes_stale_degraded_truth: bool,
    /// Support export reuses the coverage catalog.
    pub support_export_reuses_coverage: bool,
    /// Docs / help reuse the coverage catalog.
    pub docs_help_reuse_coverage: bool,
    /// Assistive-tech conformance packets reuse the coverage catalog.
    pub at_conformance_packets_reuse_coverage: bool,
}

/// Constructor input for [`M5EventCoverageCatalogPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5EventCoverageCatalogPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Event-family coverage rows.
    pub families: Vec<M5EventFamilyCoverage>,
    /// Shared (matrix-owned) controlled-vocabulary set.
    pub shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet,
    /// Announcement-grammar (grammar-owned) controlled-vocabulary set, reused to prove
    /// events route through the same governed grammar.
    pub announcement_vocabulary_set: M5AnnouncementGrammarVocabularySet,
    /// Coverage-shaped controlled-vocabulary set.
    pub coverage_vocabulary_set: M5EventCoverageVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5EventCoverageConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5EventCoverageConsumerProjection,
    /// Proof freshness block (reused from the matrix lane).
    pub proof_freshness: M5DynamicSurfaceA11yProofFreshness,
    /// Release and mirror/offline parity posture (reused from the matrix lane).
    pub release_posture: M5DynamicSurfaceA11yReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 event-class coverage catalog packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EventCoverageCatalogPacket {
    /// Record kind; must equal [`M5_EVENT_COVERAGE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_EVENT_COVERAGE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Event-family coverage rows.
    pub families: Vec<M5EventFamilyCoverage>,
    /// Shared (matrix-owned) controlled-vocabulary set.
    pub shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet,
    /// Announcement-grammar (grammar-owned) controlled-vocabulary set.
    pub announcement_vocabulary_set: M5AnnouncementGrammarVocabularySet,
    /// Coverage-shaped controlled-vocabulary set.
    pub coverage_vocabulary_set: M5EventCoverageVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5EventCoverageConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5EventCoverageConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DynamicSurfaceA11yProofFreshness,
    /// Release and mirror/offline parity posture.
    pub release_posture: M5DynamicSurfaceA11yReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5EventCoverageCatalogPacket {
    /// Builds an event-coverage catalog packet from seed input.
    pub fn new(input: M5EventCoverageCatalogPacketInput) -> Self {
        Self {
            record_kind: M5_EVENT_COVERAGE_RECORD_KIND.to_owned(),
            schema_version: M5_EVENT_COVERAGE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            catalog_label: input.catalog_label,
            families: input.families,
            shared_vocabulary_set: input.shared_vocabulary_set,
            announcement_vocabulary_set: input.announcement_vocabulary_set,
            coverage_vocabulary_set: input.coverage_vocabulary_set,
            conformance_review: input.conformance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Total number of covered dynamic events across every family.
    pub fn event_count(&self) -> usize {
        self.families.iter().map(|family| family.events.len()).sum()
    }

    /// Validates the event-coverage catalog invariants.
    pub fn validate(&self) -> Vec<M5EventCoverageViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_EVENT_COVERAGE_RECORD_KIND {
            violations.push(M5EventCoverageViolation::WrongRecordKind);
        }
        if self.schema_version != M5_EVENT_COVERAGE_SCHEMA_VERSION {
            violations.push(M5EventCoverageViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.catalog_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5EventCoverageViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_sets(self, &mut violations);
        validate_families(self, &mut violations);
        validate_conformance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 event coverage catalog serializes"),
        ) {
            violations.push(M5EventCoverageViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 event coverage catalog serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable = self
            .families
            .iter()
            .filter(|f| f.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Event-Class Non-Visual Coverage\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalog_label));
        out.push_str(&format!(
            "- Families: {} ({} stable), {} events\n",
            self.families.len(),
            stable,
            self.event_count()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Event families\n\n");
        for family in &self.families {
            out.push_str(&format!(
                "- **{}** (`{}`): `{}`, fidelity `{}`\n",
                family.family_id,
                family.family.as_str(),
                family.qualification.as_str(),
                family.non_visual_fidelity.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", family.owner_role));
            out.push_str(&format!(
                "  - Producers: {}\n",
                family
                    .producers
                    .iter()
                    .map(|p| p.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            for event in &family.events {
                out.push_str(&format!(
                    "  - `{}` -> {} / {} ({}); identity `{}`; fallback {} (`{}`)\n",
                    event.event_id,
                    event.announcement_event_class.as_str(),
                    event.channel().as_str(),
                    event.degraded_disclosure.reason_class.as_str(),
                    event.identity_message_id,
                    event.durable_fallback.surface.as_str(),
                    event.durable_fallback.surface_ref
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in event-coverage export.
#[derive(Debug)]
pub enum M5EventCoverageArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5EventCoverageViolation>),
}

impl fmt::Display for M5EventCoverageArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "m5 event coverage export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 event coverage export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5EventCoverageArtifactError {}

/// Validation failures emitted by [`M5EventCoverageCatalogPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5EventCoverageViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A governed event family has no coverage row.
    RequiredFamilyMissing,
    /// Two rows cover the same event family.
    DuplicateFamily,
    /// Two rows share a family id.
    DuplicateFamilyId,
    /// A family row is incomplete.
    FamilyIncomplete,
    /// Two events share an event id.
    DuplicateEventId,
    /// An event row is incomplete.
    EventIncomplete,
    /// An event identity message id is missing the governed prefix.
    EventIdentityPrefixMissing,
    /// A low-value (non-meaning-changing) event entered the assistive channel.
    LowValueEventInChannel,
    /// An event's reason disclosure flag disagrees with its reason class.
    ReasonDisclosureInconsistent,
    /// An event's announcement class does not match its blocked/degraded reason.
    AnnouncementClassReasonMismatch,
    /// A family cannot announce any blocked/degraded reason.
    FamilyCannotAnnounceDegradedReason,
    /// An event has no reopenable durable fallback surface.
    EventDurableFallbackMissing,
    /// A family's non-visual fidelity is not an accessible class.
    FamilyNonVisualFidelityInvalid,
    /// A family claiming Stable is missing required proof packet refs.
    StableFamilyMissingProof,
    /// A family has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A family has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Conformance review does not satisfy required invariants.
    ConformanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/mirror-offline parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5EventCoverageViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredFamilyMissing => "required_family_missing",
            Self::DuplicateFamily => "duplicate_family",
            Self::DuplicateFamilyId => "duplicate_family_id",
            Self::FamilyIncomplete => "family_incomplete",
            Self::DuplicateEventId => "duplicate_event_id",
            Self::EventIncomplete => "event_incomplete",
            Self::EventIdentityPrefixMissing => "event_identity_prefix_missing",
            Self::LowValueEventInChannel => "low_value_event_in_channel",
            Self::ReasonDisclosureInconsistent => "reason_disclosure_inconsistent",
            Self::AnnouncementClassReasonMismatch => "announcement_class_reason_mismatch",
            Self::FamilyCannotAnnounceDegradedReason => "family_cannot_announce_degraded_reason",
            Self::EventDurableFallbackMissing => "event_durable_fallback_missing",
            Self::FamilyNonVisualFidelityInvalid => "family_non_visual_fidelity_invalid",
            Self::StableFamilyMissingProof => "stable_family_missing_proof",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ConformanceReviewIncomplete => "conformance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable event-coverage export.
pub fn current_stable_m5_event_coverage_export(
) -> Result<M5EventCoverageCatalogPacket, M5EventCoverageArtifactError> {
    let packet: M5EventCoverageCatalogPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/a11y/m5-event-coverage-proof/support_export.json"
    )))
    .map_err(M5EventCoverageArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5EventCoverageArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5EventCoverageCatalogPacket,
    violations: &mut Vec<M5EventCoverageViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_EVENT_COVERAGE_SCHEMA_REF,
        M5_EVENT_COVERAGE_DOC_REF,
        M5_EVENT_COVERAGE_MATRIX_REF,
        M5_EVENT_COVERAGE_ANNOUNCEMENT_GRAMMAR_REF,
        M5_EVENT_COVERAGE_SURFACE_DESCRIPTOR_REF,
        M5_EVENT_COVERAGE_SCREEN_READER_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5EventCoverageViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_sets(
    packet: &M5EventCoverageCatalogPacket,
    violations: &mut Vec<M5EventCoverageViolation>,
) {
    if !packet.shared_vocabulary_set.matches_canonical()
        || !packet.announcement_vocabulary_set.matches_canonical()
        || !packet.coverage_vocabulary_set.matches_canonical()
    {
        violations.push(M5EventCoverageViolation::VocabularySetDrift);
    }
}

fn validate_families(
    packet: &M5EventCoverageCatalogPacket,
    violations: &mut Vec<M5EventCoverageViolation>,
) {
    let present: BTreeSet<M5EventFamily> = packet.families.iter().map(|f| f.family).collect();
    for required in M5EventFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5EventCoverageViolation::RequiredFamilyMissing);
            break;
        }
    }

    let mut seen_family_ids: BTreeSet<&str> = BTreeSet::new();
    let mut seen_families: BTreeSet<M5EventFamily> = BTreeSet::new();
    let mut seen_event_ids: BTreeSet<&str> = BTreeSet::new();
    for family in &packet.families {
        if !seen_family_ids.insert(family.family_id.as_str()) {
            violations.push(M5EventCoverageViolation::DuplicateFamilyId);
        }
        if !seen_families.insert(family.family) {
            violations.push(M5EventCoverageViolation::DuplicateFamily);
        }

        if family.family_id.trim().is_empty()
            || family.label.trim().is_empty()
            || family.owner_role.trim().is_empty()
            || family.producers.is_empty()
            || family.events.is_empty()
            || family.source_contract_refs.is_empty()
        {
            violations.push(M5EventCoverageViolation::FamilyIncomplete);
        }

        if !is_accessible_fidelity(family.non_visual_fidelity) {
            violations.push(M5EventCoverageViolation::FamilyNonVisualFidelityInvalid);
        }
        if !family.can_announce_degraded_reason() {
            violations.push(M5EventCoverageViolation::FamilyCannotAnnounceDegradedReason);
        }
        if family.qualification.is_stable() && family.required_proof_packet_refs.is_empty() {
            violations.push(M5EventCoverageViolation::StableFamilyMissingProof);
        }
        if family.downgrade_triggers.is_empty() {
            violations.push(M5EventCoverageViolation::DowngradeTriggersMissing);
        }
        if family.consumer_surfaces.is_empty() {
            violations.push(M5EventCoverageViolation::ConsumerSurfacesMissing);
        }

        for event in &family.events {
            if !seen_event_ids.insert(event.event_id.as_str()) {
                violations.push(M5EventCoverageViolation::DuplicateEventId);
            }
            validate_event(event, violations);
        }
    }
}

fn validate_event(event: &M5DynamicEventMapping, violations: &mut Vec<M5EventCoverageViolation>) {
    if event.event_id.trim().is_empty()
        || event.label.trim().is_empty()
        || event.identity_message_id.trim().is_empty()
    {
        violations.push(M5EventCoverageViolation::EventIncomplete);
    }

    if !event
        .identity_message_id
        .starts_with(M5_EVENT_IDENTITY_MESSAGE_ID_PREFIX)
    {
        violations.push(M5EventCoverageViolation::EventIdentityPrefixMissing);
    }

    // Guardrail: only meaning-changing dynamic events belong in the assistive
    // channel; a producer that can emit a low-value tick must not seed it here.
    if !event.meaning_changing {
        violations.push(M5EventCoverageViolation::LowValueEventInChannel);
    }

    // The disclosure flag and the reason class must agree: a normal transition
    // discloses no reason; any blocked/degraded reason must be flagged.
    let reason = event.degraded_disclosure.reason_class;
    if event.degraded_disclosure.announces_reason != reason.discloses_reason() {
        violations.push(M5EventCoverageViolation::ReasonDisclosureInconsistent);
    }

    // The announcement class must match the reason: a blocker interrupts assertively,
    // a degraded/stale/unavailable/policy reason narrates through degraded-or-stale
    // truth, and a normal transition never claims either reserved class.
    let class = event.announcement_event_class;
    let class_ok = match reason.required_announcement_class() {
        Some(required) => class == required,
        None => !matches!(
            class,
            M5AnnouncementEventClass::BlockerRaised
                | M5AnnouncementEventClass::DegradedOrStaleTruth
        ),
    };
    if !class_ok {
        violations.push(M5EventCoverageViolation::AnnouncementClassReasonMismatch);
    }

    // Every covered event must preserve its identity on a reopenable durable surface.
    if event.durable_fallback.surface_ref.trim().is_empty() || !event.durable_fallback.reopenable {
        violations.push(M5EventCoverageViolation::EventDurableFallbackMissing);
    }
}

/// True when a fidelity class still conveys non-visual truth for a covered event.
fn is_accessible_fidelity(fidelity: A11yNonVisualFidelity) -> bool {
    matches!(
        fidelity,
        A11yNonVisualFidelity::FullAccessible
            | A11yNonVisualFidelity::DegradedAccessible
            | A11yNonVisualFidelity::SummaryOnly
    )
}

fn validate_conformance_review(
    packet: &M5EventCoverageCatalogPacket,
    violations: &mut Vec<M5EventCoverageViolation>,
) {
    let review = &packet.conformance_review;
    for ok in [
        review.workflows_narrate_transitions_without_visual_only_cues,
        review.each_family_announces_identity_plus_blocked_or_degraded_reason,
        review.events_route_through_one_announcement_grammar_not_per_surface_prose,
        review.durable_fallback_preserves_event_identity_and_state_labels,
        review.only_meaning_changing_events_enter_assistive_channel,
        review.support_export_can_reconstruct_what_user_should_have_been_told,
        review.claimed_families_auto_narrow_when_bridge_or_proof_stale,
        review.downgrade_narrows_instead_of_hides,
        review.no_visual_only_or_pointer_only_event_source,
    ] {
        if !ok {
            violations.push(M5EventCoverageViolation::ConformanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5EventCoverageCatalogPacket,
    violations: &mut Vec<M5EventCoverageViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.editor_routes_diagnostics_and_assist,
        projection.terminal_routes_command_boundaries,
        projection.debug_and_test_route_transitions,
        projection.review_and_ai_route_milestones,
        projection.collaboration_routes_control_changes,
        projection.notebook_routes_session_changes,
        projection.shell_routes_stale_degraded_truth,
        projection.support_export_reuses_coverage,
        projection.docs_help_reuse_coverage,
        projection.at_conformance_packets_reuse_coverage,
    ] {
        if !ok {
            violations.push(M5EventCoverageViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5EventCoverageCatalogPacket,
    violations: &mut Vec<M5EventCoverageViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5EventCoverageViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5EventCoverageCatalogPacket,
    violations: &mut Vec<M5EventCoverageViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.mirror_offline_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.mirror_offline_parity_required
        || !posture.stable_promotion_blocks_without_mapped_proof
    {
        violations.push(M5EventCoverageViolation::ReleasePostureIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
