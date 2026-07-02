//! Durable progress-indicator and job-row actor, phase, action, and history
//! parity certified across every claimed M5 durable-work activity surface.
//!
//! The [frozen shell-primitives matrix][matrix] already freezes the durable-work
//! primitives — the ambient progress indicator and the durable job row — into one
//! export-safe packet: their progress states, source/provider/freshness labels,
//! accessibility routes, the mandatory labels every progress surface must be able to
//! show, and the downgrade triggers that narrow them below a claim. This lane is the
//! **durable-progress certification capstone** on top of that matrix: for every claimed
//! M5 durable-work job family — indexing, notebook/runtime work, requests/data loads,
//! downloads, updates, sync, branch-agent work, provider handoffs, and support/export
//! jobs — it certifies that durable work is never represented only by a transient spinner
//! or toast and stays reviewable after the user looks away; that every progress row
//! attributes its actor/subsystem, phase, current step, cancel/retry/open-details
//! actions, and a link back to the authoritative object or evidence packet; that grouped
//! completion/failure history and blocked/paused reasons are preserved in durable,
//! reopenable history; and that current progress and recent job history are
//! reconstructable from a support export without relying on transient toasts or a live
//! dashboard.
//!
//! Three records carry the truth:
//!
//! - the per-family **certification row** ([`DurableProgressCertificationRow`]): one row
//!   per [`M5DurableJobFamily`] naming the progress primitives it drives, the progress
//!   states / source-freshness labels / required labels / accessibility routes / consumer
//!   surfaces / downgrade triggers pulled from the frozen matrix, its durable-presence /
//!   progress-attribution / grouped-history / progress-export posture, any active waiver,
//!   and a derived green/yellow/red [`DurableProgressCertificationStatus`].
//! - the release **certification packet** ([`DurableProgressCertificationPacket`]): the
//!   full set of rows with derived per-row status, aggregate green/yellow/red counts, the
//!   active waivers, the exact certification causes ([`DurableProgressCertificationCause`]),
//!   and the blocking findings the lane refuses to ship with.
//! - the **certification dashboard** ([`DurableProgressCertificationDashboard`]): a light
//!   projection the shell / activity center / release automation reads to auto-narrow a
//!   claimed job family when its durable-progress proof falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow`
//! the moment it discloses a reduced history retention, a coarse attribution, a compacted
//! grouped history (backed by a waiver), or a partial support-export capture; it drops to
//! `red` if durable work is shown only through a transient spinner or toast, actor / phase
//! / action / authoritative-object attribution is missing, grouped history or a
//! blocked/paused reason is lost, the progress state is absent from the support-export
//! capture, a job is spinner-or-toast-only, or its progress states / required labels are
//! incomplete. That derivation is the auto-narrowing the acceptance criteria require.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw
//! URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials — only
//! stable ids, closed vocabulary, counts, refs, and short labels. The progress-state,
//! source-freshness, accessibility-route, required-label, consumer-surface,
//! downgrade-trigger, and qualification vocabulary is re-exported by reference from the
//! already frozen [matrix]; each row pulls its progress bindings straight from that
//! matrix's seeded progress-indicator and durable-job-row rows, so this lane mints no
//! parallel shell vocabulary and cannot certify a durable-progress posture the matrix does
//! not freeze. Only the certification-specific vocabulary ([`M5DurableJobFamily`],
//! [`M5DurableProgressProofDimension`], [`DurableProgressCertificationStatus`],
//! [`DurablePresenceState`], [`ProgressAttributionState`], [`GroupedHistoryState`],
//! [`ProgressExportState`], [`DurableProgressCertificationWaiver`],
//! [`DurableProgressCertificationCause`], [`DurableProgressCertificationFinding`]) is new.
//!
//! Unlike the pane controls, progress rows **do** carry source/provider/freshness truth —
//! a job row shows a provider-attributed handoff and labels sampled or in-flight values —
//! so this lane certifies the full six-label set ([`DURABLE_PROGRESS_REQUIRED_LABELS`]:
//! identity, state, keyboard-route, source-provider, freshness, and reopen-path) and
//! carries the frozen source-freshness labels on every row.
//!
//! [matrix]: crate::freeze_the_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix as matrix;

pub use matrix::{
    M5AccessibilityRoute, M5PrimitiveQualificationClass, M5PrimitiveRequiredLabel, M5ProgressState,
    M5ShellConsumerSurface, M5ShellPrimitiveDowngradeTrigger, M5ShellPrimitiveFamily,
    M5ShellZoneSlot, M5SourceFreshnessLabel,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_durable_progress_certification_packet,
    seeded_m5_durable_progress_certification_packet_branch_agent_spinner_or_toast_only_blocked,
    seeded_m5_durable_progress_certification_packet_indexing_transient_spinner_blocked,
    seeded_m5_durable_progress_certification_packet_notebook_attribution_missing_blocked,
    seeded_m5_durable_progress_certification_packet_request_history_lost_blocked,
    seeded_m5_durable_progress_certification_packet_update_progress_absent_from_capture_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_DURABLE_PROGRESS_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_DURABLE_PROGRESS_CERTIFICATION_SHARED_CONTRACT_REF: &str =
    "shell:m5_durable_progress_certification:v1";

/// Stable record kind for [`DurableProgressCertificationPacket`] payloads.
pub const M5_DURABLE_PROGRESS_CERTIFICATION_PACKET_RECORD_KIND: &str =
    "shell_m5_durable_progress_certification_packet_record";

/// Stable record kind for [`DurableProgressCertificationDashboard`] payloads.
pub const M5_DURABLE_PROGRESS_CERTIFICATION_DASHBOARD_RECORD_KIND: &str =
    "shell_m5_durable_progress_certification_dashboard_record";

/// Stable record kind for [`DurableProgressCertificationSupportExport`] payloads.
pub const M5_DURABLE_PROGRESS_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_durable_progress_certification_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_DURABLE_PROGRESS_CERTIFICATION_PACKET_ID: &str =
    "m5-durable-progress-certification:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_DURABLE_PROGRESS_CERTIFICATION_DASHBOARD_ID: &str =
    "m5-durable-progress-certification-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_DURABLE_PROGRESS_CERTIFICATION_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-durable-progress-certification:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_DURABLE_PROGRESS_CERTIFICATION_SOURCE_SCHEMA_REF: &str =
    "schemas/shell/m5-durable-progress-certification.schema.json";

/// Published markdown report ref reviewers reopen the certification proof from.
pub const M5_DURABLE_PROGRESS_CERTIFICATION_PUBLISHED_REPORT_REF: &str =
    "artifacts/shell/m5-durable-progress-certification.md";

/// Published certification-packet artifact ref.
pub const M5_DURABLE_PROGRESS_CERTIFICATION_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-durable-progress-certification-proof/packet.json";

/// Published certification-dashboard artifact ref.
pub const M5_DURABLE_PROGRESS_CERTIFICATION_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-durable-progress-certification-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_DURABLE_PROGRESS_CERTIFICATION_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-durable-progress-certification-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_DURABLE_PROGRESS_CERTIFICATION_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-durable-progress-certification-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_DURABLE_PROGRESS_CERTIFICATION_PUBLISHED_DOC_REF: &str =
    "docs/shell/m5_durable_progress_certification_contract.md";

/// Repo-relative ref to the frozen shell-primitives matrix schema.
pub const M5_DURABLE_PROGRESS_CERTIFICATION_MATRIX_SCHEMA_REF: &str =
    matrix::M5_SHELL_PRIMITIVES_SCHEMA_REF;

/// The six labels every durable-progress surface must be able to show. Progress rows
/// carry source/provider and freshness truth (a job row shows a provider-attributed
/// handoff and labels sampled or in-flight values), so this is the full
/// [`M5PrimitiveRequiredLabel::ALL`] set: identity, state, keyboard route, source
/// provider, freshness, and reopen path.
pub const DURABLE_PROGRESS_REQUIRED_LABELS: [M5PrimitiveRequiredLabel; 6] =
    M5PrimitiveRequiredLabel::ALL;

/// One of the claimed M5 durable-work job families the certification proof must cover, in
/// canonical order. Each family is a claimed M5 durable-work lane whose surfaces render
/// progress indicators and durable job rows; the lane certifies none beyond them and
/// refuses to ship if any is missing. Multi-job summaries, grouped completions, and
/// reopen-after-focus-loss are certified within each family's row rather than as separate
/// families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DurableJobFamily {
    /// Indexing / workspace-scan jobs.
    Indexing,
    /// Notebook / runtime execution jobs.
    NotebookRuntime,
    /// Request / data-load jobs.
    RequestDataLoad,
    /// Download jobs.
    Download,
    /// Update / install jobs.
    Update,
    /// Sync / replication jobs.
    Sync,
    /// Branch-agent / automation jobs.
    BranchAgent,
    /// Provider-handoff jobs.
    ProviderHandoff,
    /// Support / export jobs.
    SupportExport,
}

impl M5DurableJobFamily {
    /// Every governed job family, in canonical order.
    pub const ALL: [Self; 9] = [
        Self::Indexing,
        Self::NotebookRuntime,
        Self::RequestDataLoad,
        Self::Download,
        Self::Update,
        Self::Sync,
        Self::BranchAgent,
        Self::ProviderHandoff,
        Self::SupportExport,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Indexing => "indexing",
            Self::NotebookRuntime => "notebook_runtime",
            Self::RequestDataLoad => "request_data_load",
            Self::Download => "download",
            Self::Update => "update",
            Self::Sync => "sync",
            Self::BranchAgent => "branch_agent",
            Self::ProviderHandoff => "provider_handoff",
            Self::SupportExport => "support_export",
        }
    }

    /// Short, reviewer-facing label for the family's durable-work surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Indexing => "Indexing / workspace-scan progress",
            Self::NotebookRuntime => "Notebook / runtime execution progress",
            Self::RequestDataLoad => "Request / data-load progress",
            Self::Download => "Download progress",
            Self::Update => "Update / install progress",
            Self::Sync => "Sync / replication progress",
            Self::BranchAgent => "Branch-agent / automation progress",
            Self::ProviderHandoff => "Provider-handoff progress",
            Self::SupportExport => "Support / export job progress",
        }
    }
}

/// One of the four certification dimensions each job family is certified across.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DurableProgressProofDimension {
    /// Durable presence (reviewable after focus loss, never spinner/toast-only).
    DurablePresence,
    /// Progress attribution (actor/subsystem, phase, action, authoritative-object link).
    ProgressAttribution,
    /// Grouped history (grouped completion/failure + blocked/paused reasons preserved).
    GroupedHistory,
    /// Progress export (progress + recent history reconstructable from support export).
    ProgressExport,
}

impl M5DurableProgressProofDimension {
    /// Every certification dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DurablePresence,
        Self::ProgressAttribution,
        Self::GroupedHistory,
        Self::ProgressExport,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurablePresence => "durable_presence",
            Self::ProgressAttribution => "progress_attribution",
            Self::GroupedHistory => "grouped_history",
            Self::ProgressExport => "progress_export",
        }
    }
}

/// The derived certification light a governed job family carries.
///
/// `green` means the family's durable work stays reviewable after focus loss, attributes
/// its actor/phase/action/authoritative-object, preserves grouped completion/failure
/// history and blocked/paused reasons, and reconstructs progress and recent history from a
/// support export. `yellow` is a disclosed narrowing (a reduced history retention, a
/// coarse attribution, a waivered compacted grouped history, or a partial support-export
/// capture). `red` is blocked: durable work is spinner-or-toast-only, attribution or the
/// authoritative-object link is missing, grouped history or a blocked/paused reason is
/// lost, the progress state is absent from capture, a job is spinner-or-toast-only, or the
/// progress states / required labels are incomplete — and the family may not keep a
/// shell-maturity claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableProgressCertificationStatus {
    /// Full standing: durable, attributed, history-preserving, reconstructable.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl DurableProgressCertificationStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// `true` when the row keeps a publishable (green or yellow) claim.
    pub const fn is_publishable(self) -> bool {
        matches!(self, Self::Green | Self::Yellow)
    }
}

/// How the family's durable work stays present and reviewable after the user looks away.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurablePresenceState {
    /// Durable work is projected through a durable progress row that stays reviewable
    /// after focus loss — a completed, failed, or canceled job keeps its outcome in
    /// reopenable history rather than vanishing with a spinner or toast.
    DurableReviewableAfterFocusLoss,
    /// Under one surface the durable-history retention window is disclosedly reduced
    /// (older completed rows compact into a summary sooner) while every in-flight job and
    /// its recent history stay reviewable after focus loss and the reduction is disclosed.
    DisclosedReducedHistoryRetention,
    /// Durable work is represented only through a transient spinner or toast, so progress
    /// is lost the moment the user looks away — always a blocker.
    TransientSpinnerOrToastOnly,
}

impl DurablePresenceState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurableReviewableAfterFocusLoss => "durable_reviewable_after_focus_loss",
            Self::DisclosedReducedHistoryRetention => "disclosed_reduced_history_retention",
            Self::TransientSpinnerOrToastOnly => "transient_spinner_or_toast_only",
        }
    }

    /// `true` when durable work is reviewable after focus loss.
    pub const fn is_durable(self) -> bool {
        matches!(self, Self::DurableReviewableAfterFocusLoss)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedReducedHistoryRetention)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::TransientSpinnerOrToastOnly)
    }
}

/// How the family attributes actor/subsystem, phase, action, and authoritative-object
/// links on every progress row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProgressAttributionState {
    /// Every progress row shows its actor/subsystem, current phase and step, cancel /
    /// retry / open-details actions, and a link back to the authoritative object or
    /// evidence packet.
    ActorPhaseActionObjectAttributed,
    /// Under one surface the attribution is disclosedly coarse (a grouped batch shows the
    /// subsystem but folds per-job phase into a summary) while the actor, action
    /// affordances, and authoritative-object link stay present and the reduction is
    /// disclosed.
    DisclosedCoarseAttribution,
    /// A progress row hides its actor/phase attribution or drops the link to the
    /// authoritative object, so a grouped batch reads as an anonymous spinner — always a
    /// blocker.
    AttributionOrObjectLinkMissing,
}

impl ProgressAttributionState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActorPhaseActionObjectAttributed => "actor_phase_action_object_attributed",
            Self::DisclosedCoarseAttribution => "disclosed_coarse_attribution",
            Self::AttributionOrObjectLinkMissing => "attribution_or_object_link_missing",
        }
    }

    /// `true` when actor/phase/action/object are attributed.
    pub const fn is_attributed(self) -> bool {
        matches!(self, Self::ActorPhaseActionObjectAttributed)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedCoarseAttribution)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::AttributionOrObjectLinkMissing)
    }
}

/// How the family preserves grouped completion/failure history and blocked/paused reasons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupedHistoryState {
    /// Grouped completions and failure digests stay in durable, reopenable history, and a
    /// blocked/paused row explains why (cost, policy, network, or trust) it slowed,
    /// paused, was blocked, or needs approval.
    GroupedHistoryAndBlockedReasonsPreserved,
    /// The grouped history is disclosedly compacted (older grouped batches roll up into a
    /// digest with a reopen path) while each blocked/paused reason stays reconstructable;
    /// the compaction is disclosed and waivered.
    DisclosedCompactedHistory,
    /// Grouped completion/failure history or a blocked/paused reason is lost — a failed
    /// batch vanishes with its reason or a paused job gives no reason — always a blocker.
    HistoryOrBlockedReasonLost,
}

impl GroupedHistoryState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GroupedHistoryAndBlockedReasonsPreserved => {
                "grouped_history_and_blocked_reasons_preserved"
            }
            Self::DisclosedCompactedHistory => "disclosed_compacted_history",
            Self::HistoryOrBlockedReasonLost => "history_or_blocked_reason_lost",
        }
    }

    /// `true` when grouped history and blocked reasons are preserved.
    pub const fn is_preserved(self) -> bool {
        matches!(self, Self::GroupedHistoryAndBlockedReasonsPreserved)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedCompactedHistory)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::HistoryOrBlockedReasonLost)
    }
}

/// How current progress and recent job history are reconstructable from a support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProgressExportState {
    /// The support export reconstructs current progress and the recent job-history
    /// chronology (with actor, phase, and outcome) so a stuck or failed job can be
    /// diagnosed without relying on a transient toast or a live dashboard.
    ProgressAndHistoryReconstructable,
    /// The support export reconstructs current progress and discloses a partial capture of
    /// the recent job-history chronology while it is still being trimmed.
    DisclosedPartialCapture,
    /// Current progress or the recent job-history chronology is absent from the
    /// support-export capture, so a job bug cannot be explained without a live dashboard —
    /// always a blocker.
    ProgressStateAbsentFromCapture,
}

impl ProgressExportState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProgressAndHistoryReconstructable => "progress_and_history_reconstructable",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::ProgressStateAbsentFromCapture => "progress_state_absent_from_capture",
        }
    }

    /// `true` when the export reconstructs progress and history.
    pub const fn is_reconstructable(self) -> bool {
        matches!(self, Self::ProgressAndHistoryReconstructable)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::ProgressStateAbsentFromCapture)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red grouped-history narrowing
/// stay yellow rather than blocked — never lets a spinner-only job, a missing attribution,
/// a lost history, or a missing export hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableProgressCertificationWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The governed job family the waiver applies to.
    pub family: M5DurableJobFamily,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl DurableProgressCertificationWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a governed family's certification.
///
/// The trigger token mirrors the frozen [`M5ShellPrimitiveDowngradeTrigger`] vocabulary
/// so a cause never mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableProgressCertificationCause {
    /// The governed job family the cause applies to.
    pub family: M5DurableJobFamily,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5ShellPrimitiveDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a
    /// non-disclosed cause is a blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl DurableProgressCertificationCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One governed job family, certified across durable presence, progress attribution,
/// grouped history, and progress export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableProgressCertificationRow {
    /// The governed job family being certified.
    pub family: M5DurableJobFamily,
    /// The progress primitives this family drives. Pulled from the matrix.
    pub driven_primitive_families: Vec<M5ShellPrimitiveFamily>,
    /// The frozen qualification class of the driven progress primitives (the most-narrowed
    /// of the two). Pulled from the matrix.
    pub matrix_qualification: M5PrimitiveQualificationClass,
    /// Owner role accountable for keeping this family certified.
    pub owner_role: String,
    /// Short family-surface label.
    pub family_label: String,
    /// The canonical shell zone the durable job-row surfaces attach to. Pulled from the
    /// matrix.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Progress states these surfaces honour (union across the two progress families).
    /// Pulled from the matrix.
    pub certified_progress_states: Vec<M5ProgressState>,
    /// Source/provider/freshness labels these surfaces can show (union across the two
    /// progress families). Pulled from the matrix.
    pub certified_source_freshness_labels: Vec<M5SourceFreshnessLabel>,
    /// Non-visual accessibility routes. Pulled from the matrix.
    pub accessibility_routes: Vec<M5AccessibilityRoute>,
    /// Mandatory labels every progress surface must be able to show. Pulled from the
    /// matrix.
    pub required_labels: Vec<M5PrimitiveRequiredLabel>,
    /// Shell subsystems this family stays aligned across. Pulled from the matrix.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this family. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5ShellPrimitiveDowngradeTrigger>,
    /// Durable-presence posture.
    pub durable_presence: DurablePresenceState,
    /// Progress-attribution posture.
    pub progress_attribution: ProgressAttributionState,
    /// Grouped-history posture.
    pub grouped_history: GroupedHistoryState,
    /// Progress-export posture.
    pub progress_export: ProgressExportState,
    /// Hard invariant: durable work is never spinner-or-toast-only. `false` is a blocker.
    pub never_spinner_or_toast_only: bool,
    /// Active waiver, when a disclosed grouped-history compaction is in force.
    pub active_waiver: Option<DurableProgressCertificationWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: DurableProgressCertificationStatus,
    /// The exact certification causes that narrowed or blocked this row.
    pub certification_causes: Vec<DurableProgressCertificationCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl DurableProgressCertificationRow {
    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when every progress state the matrix freezes is certified — the lint that
    /// prevents a progress surface from shipping without its full queued / running /
    /// grouped-batch / paused / succeeded / failed / canceled / reopenable-history
    /// transition set.
    pub fn progress_states_complete(&self) -> bool {
        let present: BTreeSet<M5ProgressState> =
            self.certified_progress_states.iter().copied().collect();
        M5ProgressState::ALL
            .iter()
            .all(|state| present.contains(state))
    }

    /// `true` when every durable-progress required label is certified — the lint that
    /// prevents a progress surface from shipping without identity, state, keyboard-route,
    /// source-provider, freshness, and reopen-path labels. Progress rows carry
    /// source/provider and freshness truth, so the required set is the full six
    /// [`DURABLE_PROGRESS_REQUIRED_LABELS`].
    pub fn required_labels_complete(&self) -> bool {
        let present: BTreeSet<M5PrimitiveRequiredLabel> =
            self.required_labels.iter().copied().collect();
        DURABLE_PROGRESS_REQUIRED_LABELS
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        self.durable_presence.is_blocked()
            || self.progress_attribution.is_blocked()
            || self.grouped_history.is_blocked()
            || self.progress_export.is_blocked()
            || !self.never_spinner_or_toast_only
            || !self.progress_states_complete()
            || !self.required_labels_complete()
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.durable_presence.is_disclosed()
            || self.progress_attribution.is_disclosed()
            || self.grouped_history.is_disclosed()
            || self.progress_export.is_disclosed()
    }

    /// Recomputes the derived status from the four axes and the spinner-only invariant.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest
    /// narrowing forces `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> DurableProgressCertificationStatus {
        if self.has_hard_blocker() {
            DurableProgressCertificationStatus::Red
        } else if self.has_narrowing() {
            DurableProgressCertificationStatus::Yellow
        } else {
            DurableProgressCertificationStatus::Green
        }
    }

    /// Recomputes the exact certification causes for the row, in deterministic order
    /// (presence, attribution, grouped history, export, spinner-only invariant).
    pub fn recompute_causes(&self) -> Vec<DurableProgressCertificationCause> {
        let mut causes = Vec::new();
        if !self.durable_presence.is_durable() {
            causes.push(DurableProgressCertificationCause {
                family: self.family,
                trigger: M5ShellPrimitiveDowngradeTrigger::SpinnerOnlyState,
                disclosed: self.durable_presence.is_disclosed(),
                detail: if self.durable_presence.is_disclosed() {
                    "Under one surface the durable-history retention window is disclosedly reduced \
                     (older completed rows compact into a summary sooner) while every in-flight job \
                     and its recent history stay reviewable after focus loss; the reduction is \
                     disclosed and the row is narrowed below green."
                        .to_owned()
                } else {
                    "Durable work is represented only through a transient spinner or toast, so \
                     progress is lost the moment the user looks away."
                        .to_owned()
                },
            });
        }
        if !self.progress_attribution.is_attributed() {
            causes.push(DurableProgressCertificationCause {
                family: self.family,
                trigger: M5ShellPrimitiveDowngradeTrigger::GroupedProgressUnattributed,
                disclosed: self.progress_attribution.is_disclosed(),
                detail: if self.progress_attribution.is_disclosed() {
                    "Under one surface the attribution is disclosedly coarse (a grouped batch shows \
                     the subsystem but folds per-job phase into a summary) while the actor, action \
                     affordances, and authoritative-object link stay present; the reduction is \
                     disclosed and the row is narrowed below green."
                        .to_owned()
                } else {
                    "A progress row hides its actor/phase attribution or drops the link to the \
                     authoritative object, so a grouped batch reads as an anonymous spinner."
                        .to_owned()
                },
            });
        }
        if !self.grouped_history.is_preserved() {
            causes.push(DurableProgressCertificationCause {
                family: self.family,
                trigger: M5ShellPrimitiveDowngradeTrigger::ProgressLostOnLookAway,
                disclosed: self.grouped_history.is_disclosed(),
                detail: if self.grouped_history.is_disclosed() {
                    "The grouped history is disclosedly compacted (older grouped batches roll up \
                     into a digest with a reopen path) while each blocked/paused reason stays \
                     reconstructable; the compaction is disclosed and waivered, and the row is \
                     narrowed below green."
                        .to_owned()
                } else {
                    "Grouped completion/failure history or a blocked/paused reason is lost — a \
                     failed batch vanishes with its reason or a paused job gives no reason."
                        .to_owned()
                },
            });
        }
        if !self.progress_export.is_reconstructable() {
            causes.push(DurableProgressCertificationCause {
                family: self.family,
                trigger: M5ShellPrimitiveDowngradeTrigger::ProofStale,
                disclosed: self.progress_export.is_disclosed(),
                detail: if self.progress_export.is_disclosed() {
                    "The support export reconstructs current progress and discloses a partial \
                     capture of the recent job-history chronology while it is still being trimmed; \
                     the partial capture is disclosed and the row is narrowed below green."
                        .to_owned()
                } else {
                    "Current progress or the recent job-history chronology is absent from the \
                     support-export capture, so a job bug cannot be explained without a live \
                     dashboard."
                        .to_owned()
                },
            });
        }
        if !self.never_spinner_or_toast_only {
            causes.push(DurableProgressCertificationCause {
                family: self.family,
                trigger: M5ShellPrimitiveDowngradeTrigger::SpinnerOnlyState,
                disclosed: false,
                detail:
                    "A job is represented only by a transient spinner or toast, with no durable \
                         reopenable row, so its progress is spinner-or-toast-only."
                        .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed compacted grouped history may only stay yellow (rather than red) when a
    /// waiver discloses it.
    pub fn requires_waiver(&self) -> bool {
        self.grouped_history.is_disclosed()
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<DurableProgressCertificationFinding> {
        let mut findings = Vec::new();
        let family = self.family.as_str().to_owned();

        if self.durable_presence.is_blocked() {
            findings.push(DurableProgressCertificationFinding::ProgressNotDurable {
                family: family.clone(),
            });
        }
        if self.progress_attribution.is_blocked() {
            findings.push(DurableProgressCertificationFinding::AttributionMissing {
                family: family.clone(),
            });
        }
        if self.grouped_history.is_blocked() {
            findings.push(DurableProgressCertificationFinding::GroupedHistoryLost {
                family: family.clone(),
            });
        }
        if self.progress_export.is_blocked() {
            findings.push(
                DurableProgressCertificationFinding::ProgressStateAbsentFromCapture {
                    family: family.clone(),
                },
            );
        }
        if !self.never_spinner_or_toast_only {
            findings.push(DurableProgressCertificationFinding::JobSpinnerOrToastOnly {
                family: family.clone(),
            });
        }
        if !self.progress_states_complete() {
            findings.push(
                DurableProgressCertificationFinding::ProgressStatesIncomplete {
                    family: family.clone(),
                },
            );
        }
        if !self.required_labels_complete() {
            findings.push(
                DurableProgressCertificationFinding::RequiredLabelsIncomplete {
                    family: family.clone(),
                },
            );
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, DurableProgressCertificationStatus::Green) && !self.has_reason() {
            findings.push(
                DurableProgressCertificationFinding::NarrowedRowWithoutReason {
                    family: family.clone(),
                },
            );
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an
        // active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(
                DurableProgressCertificationFinding::NarrowedRowWithoutWaiver {
                    family: family.clone(),
                },
            );
        }
        // An attached waiver must still be active and must point at this family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.family != self.family {
                findings.push(DurableProgressCertificationFinding::WaiverFamilyMismatch {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(DurableProgressCertificationFinding::WaiverExpired {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(DurableProgressCertificationFinding::RowStatusStale {
                family: family.clone(),
            });
        }
        if self.certification_causes != self.recompute_causes() {
            findings.push(DurableProgressCertificationFinding::RowCausesStale { family });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} qual={} presence={} attribution={} history={} export={} durable_row={} waiver={}",
            self.family.as_str(),
            self.derived_status.as_str(),
            self.matrix_qualification.as_str(),
            self.durable_presence.as_str(),
            self.progress_attribution.as_str(),
            self.grouped_history.as_str(),
            self.progress_export.as_str(),
            self.never_spinner_or_toast_only,
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the durable-progress certification proof refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum DurableProgressCertificationFinding {
    /// A governed job family has no certification row.
    FamilyMissing {
        /// The missing family token.
        family: String,
    },
    /// A family's durable work is spinner-or-toast-only.
    ProgressNotDurable {
        /// The family token.
        family: String,
    },
    /// A family's progress row hides attribution or drops the authoritative-object link.
    AttributionMissing {
        /// The family token.
        family: String,
    },
    /// A family's grouped history or a blocked/paused reason is lost.
    GroupedHistoryLost {
        /// The family token.
        family: String,
    },
    /// A family's progress state is absent from the support-export capture.
    ProgressStateAbsentFromCapture {
        /// The family token.
        family: String,
    },
    /// A family has a job represented only by a transient spinner or toast.
    JobSpinnerOrToastOnly {
        /// The family token.
        family: String,
    },
    /// A family does not certify every frozen progress state.
    ProgressStatesIncomplete {
        /// The family token.
        family: String,
    },
    /// A family does not certify every durable-progress required label.
    RequiredLabelsIncomplete {
        /// The family token.
        family: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The family token.
        family: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The family token.
        family: String,
    },
    /// An attached waiver does not point at the row's family.
    WaiverFamilyMismatch {
        /// The family token.
        family: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The family token.
        family: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The family token.
        family: String,
    },
    /// The declared certification causes do not match the recomputed causes.
    RowCausesStale {
        /// The family token.
        family: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl DurableProgressCertificationFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::FamilyMissing { .. } => "family_missing",
            Self::ProgressNotDurable { .. } => "progress_not_durable",
            Self::AttributionMissing { .. } => "attribution_missing",
            Self::GroupedHistoryLost { .. } => "grouped_history_lost",
            Self::ProgressStateAbsentFromCapture { .. } => "progress_state_absent_from_capture",
            Self::JobSpinnerOrToastOnly { .. } => "job_spinner_or_toast_only",
            Self::ProgressStatesIncomplete { .. } => "progress_states_incomplete",
            Self::RequiredLabelsIncomplete { .. } => "required_labels_incomplete",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverFamilyMismatch { .. } => "waiver_family_mismatch",
            Self::WaiverExpired { .. } => "waiver_expired",
            Self::RowStatusStale { .. } => "row_status_stale",
            Self::RowCausesStale { .. } => "row_causes_stale",
            Self::StatusCountsStale => "status_counts_stale",
            Self::CoverageStale => "coverage_stale",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }

    /// The owning subject ref the finding points at.
    pub fn subject_ref(&self) -> &str {
        match self {
            Self::FamilyMissing { family }
            | Self::ProgressNotDurable { family }
            | Self::AttributionMissing { family }
            | Self::GroupedHistoryLost { family }
            | Self::ProgressStateAbsentFromCapture { family }
            | Self::JobSpinnerOrToastOnly { family }
            | Self::ProgressStatesIncomplete { family }
            | Self::RequiredLabelsIncomplete { family }
            | Self::NarrowedRowWithoutReason { family }
            | Self::NarrowedRowWithoutWaiver { family }
            | Self::WaiverFamilyMismatch { family, .. }
            | Self::WaiverExpired { family, .. }
            | Self::RowStatusStale { family }
            | Self::RowCausesStale { family } => family,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The release certification packet shared by the shell / activity center / release
/// automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableProgressCertificationPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the packet.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable packet id used to pivot across surfaces.
    pub packet_id: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Reviewer-facing summary line printed above the rows.
    pub headline: String,
    /// The frozen shell-primitives matrix packet id this proof certifies.
    pub matrix_packet_ref: String,
    /// Repo-relative ref to the frozen shell-primitives matrix schema.
    pub matrix_schema_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The certification dimensions every family is certified across.
    pub required_proof_dimensions: Vec<M5DurableProgressProofDimension>,
    /// The progress states every family must certify.
    pub required_progress_states: Vec<M5ProgressState>,
    /// The required labels every family must certify.
    pub required_labels: Vec<M5PrimitiveRequiredLabel>,
    /// Per-family certification rows, in canonical order.
    pub rows: Vec<DurableProgressCertificationRow>,
    /// Governed families certified, in canonical (sorted) order.
    pub covered_families: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<DurableProgressCertificationWaiver>,
    /// Every exact certification cause, in row then cause order.
    pub certification_causes: Vec<DurableProgressCertificationCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<DurableProgressCertificationFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Shell / release automation refs that consume this packet to auto-narrow claimed
    /// families.
    pub shell_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Docs / help refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published certification-packet ref.
    pub published_packet_ref: String,
    /// Published certification-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl DurableProgressCertificationPacket {
    /// Returns the certification row for `family`, if present.
    pub fn row(&self, family: M5DurableJobFamily) -> Option<&DurableProgressCertificationRow> {
        self.rows.iter().find(|row| row.family == family)
    }

    /// Returns compact text lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!(
                "packet: id={}, rows={}, green={}, yellow={}, red={}, clean={}",
                self.packet_id,
                self.row_count,
                self.green_row_count,
                self.yellow_row_count,
                self.red_row_count,
                self.report_clean,
            ),
            format!(
                "matrix={} build={} channel={} publishable={}",
                self.matrix_packet_ref,
                self.build_identity_ref,
                self.release_channel_class,
                self.all_rows_publishable,
            ),
        ];
        for row in &self.rows {
            lines.push(row.compact_line());
        }
        for waiver in &self.active_waivers {
            lines.push(format!(
                "  waiver {} -> {} (expires {})",
                waiver.waiver_id,
                waiver.family.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.certification_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.family.as_str(),
                cause.cause_token(),
                cause.disclosed
            ));
        }
        for finding in &self.blocking_findings {
            lines.push(format!(
                "  blocker: {} -- {}",
                finding.class_token(),
                finding.subject_ref()
            ));
        }
        lines
    }

    /// Projects the light certification dashboard the shell automation consumes.
    pub fn dashboard(&self) -> DurableProgressCertificationDashboard {
        DurableProgressCertificationDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 durable-progress certification packet serializes")
    }

    /// Deterministic, machine-readable certification CSV: one row per family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "family,status,qualification,shell_zone_slot,durable_presence,progress_attribution,grouped_history,progress_export,never_spinner_or_toast_only,progress_states,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.family.as_str(),
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.shell_zone_slot.as_str(),
                row.durable_presence.as_str(),
                row.progress_attribution.as_str(),
                row.grouped_history.as_str(),
                row.progress_export.as_str(),
                row.never_spinner_or_toast_only,
                join_tokens(&row.certified_progress_states, |s| s.as_str()),
                row.active_waiver
                    .as_ref()
                    .map(|w| w.waiver_id.as_str())
                    .unwrap_or("none"),
            ));
        }
        out
    }

    /// Renders the markdown report for the lane.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 durable progress-indicator & job-row actor, phase, action & history parity\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_durable_progress_certification`](../../crates/aureline-shell/src/m5_durable_progress_certification/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_durable_progress_certification -- markdown > \\\n  artifacts/shell/m5-durable-progress-certification.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Packet id: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Certifies matrix packet: `{}`\n",
            self.matrix_packet_ref
        ));
        out.push_str(&format!("- Exact build: `{}`\n", self.build_identity_ref));
        out.push_str(&format!(
            "- Release channel: `{}`\n",
            self.release_channel_class
        ));
        out.push_str(&format!("- Rows certified: {}\n", self.row_count));
        out.push_str(&format!("- Green: {}\n", self.green_row_count));
        out.push_str(&format!(
            "- Yellow (auto-narrowed): {}\n",
            self.yellow_row_count
        ));
        out.push_str(&format!("- Red (blocked): {}\n", self.red_row_count));
        out.push_str(&format!(
            "- All rows publishable: `{}`\n",
            self.all_rows_publishable
        ));
        out.push_str(&format!(
            "- Blocking findings: {}\n",
            self.blocking_findings.len()
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Certification dimensions\n\n");
        for dimension in &self.required_proof_dimensions {
            out.push_str(&format!("- `{}`\n", dimension.as_str()));
        }
        out.push('\n');

        out.push_str("## Certification rows\n\n");
        out.push_str(
            "| Family | Status | Qualification | Presence | Attribution | History | Export | Durable-row | Waiver |\n\
             | ------ | ------ | ------------- | -------- | ----------- | ------- | ------ | ----------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.family_label,
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.durable_presence.as_str(),
                row.progress_attribution.as_str(),
                row.grouped_history.as_str(),
                row.progress_export.as_str(),
                row.never_spinner_or_toast_only,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&DurableProgressCertificationRow> = self
            .rows
            .iter()
            .filter(|row| {
                !matches!(
                    row.derived_status,
                    DurableProgressCertificationStatus::Green
                )
            })
            .collect();
        if narrowed.is_empty() {
            out.push_str("None — every governed job family is certified at full standing.\n\n");
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.family.as_str(),
                    row.derived_status.as_str(),
                    row.narrowing_reason.as_deref().unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Exact certification causes\n\n");
        if self.certification_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.certification_causes {
                out.push_str(&format!(
                    "- `{}` — `{}` (disclosed: `{}`) — {}\n",
                    cause.family.as_str(),
                    cause.cause_token(),
                    cause.disclosed,
                    cause.detail,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Active waivers\n\n");
        if self.active_waivers.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for waiver in &self.active_waivers {
                out.push_str(&format!(
                    "- `{}` (`{}`, owner: {}, expires `{}`) — {}\n",
                    waiver.waiver_id,
                    waiver.family.as_str(),
                    waiver.owner_role,
                    waiver.expires_at,
                    waiver.reason,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Findings\n\n");
        if self.blocking_findings.is_empty() {
            out.push_str("Findings: none.\n\n");
        } else {
            for finding in &self.blocking_findings {
                out.push_str(&format!(
                    "- `{}` — `{}`\n",
                    finding.class_token(),
                    finding.subject_ref()
                ));
            }
            out.push('\n');
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_durable_progress_certification -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_durable_progress_certification_fixtures\n",
        );
        out.push_str("```\n");
        out
    }
}

/// One row of the light certification dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableProgressCertificationDashboardRow {
    /// The governed family.
    pub family: M5DurableJobFamily,
    /// Short family-surface label.
    pub family_label: String,
    /// Derived green/yellow/red status.
    pub status: DurableProgressCertificationStatus,
    /// Frozen qualification class.
    pub matrix_qualification: M5PrimitiveQualificationClass,
    /// Durable-presence posture.
    pub durable_presence: DurablePresenceState,
    /// Progress-attribution posture.
    pub progress_attribution: ProgressAttributionState,
    /// Grouped-history posture.
    pub grouped_history: GroupedHistoryState,
    /// Progress-export posture.
    pub progress_export: ProgressExportState,
    /// `true` when durable work is never spinner-or-toast-only.
    pub never_spinner_or_toast_only: bool,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light certification dashboard the shell / activity center / release automation
/// reads to auto-narrow claimed job families.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableProgressCertificationDashboard {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the dashboard.
    pub schema_version: u32,
    /// Stable dashboard id.
    pub dashboard_id: String,
    /// The packet id this dashboard projects.
    pub source_packet_ref: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Dashboard rows, in canonical order.
    pub rows: Vec<DurableProgressCertificationDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Shell / release automation refs that consume the dashboard.
    pub shell_automation_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl DurableProgressCertificationDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &DurableProgressCertificationPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| DurableProgressCertificationDashboardRow {
                family: row.family,
                family_label: row.family_label.clone(),
                status: row.derived_status,
                matrix_qualification: row.matrix_qualification,
                durable_presence: row.durable_presence,
                progress_attribution: row.progress_attribution,
                grouped_history: row.grouped_history,
                progress_export: row.progress_export,
                never_spinner_or_toast_only: row.never_spinner_or_toast_only,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .certification_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_DURABLE_PROGRESS_CERTIFICATION_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_DURABLE_PROGRESS_CERTIFICATION_SCHEMA_VERSION,
            dashboard_id: M5_DURABLE_PROGRESS_CERTIFICATION_DASHBOARD_ID.to_owned(),
            source_packet_ref: packet.packet_id.clone(),
            source_schema_ref: packet.source_schema_ref.clone(),
            rows,
            green_row_count: packet.green_row_count,
            yellow_row_count: packet.yellow_row_count,
            red_row_count: packet.red_row_count,
            all_rows_publishable: packet.all_rows_publishable,
            shell_automation_refs: packet.shell_automation_refs.clone(),
            generated_at: packet.generated_at.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 durable-progress certification dashboard serializes")
    }
}

/// Support-export wrapper for the durable-progress certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableProgressCertificationSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: DurableProgressCertificationPacket,
    /// Dashboard quoted in full.
    pub dashboard: DurableProgressCertificationDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl DurableProgressCertificationSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each family, and each
    /// active waiver id is quoted as a case id so a support reviewer — or the shell
    /// automation — can name the same family and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: DurableProgressCertificationPacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.family.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_DURABLE_PROGRESS_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_DURABLE_PROGRESS_CERTIFICATION_SCHEMA_VERSION,
            shared_contract_ref: M5_DURABLE_PROGRESS_CERTIFICATION_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_durable_progress_certification_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableProgressCertificationInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen shell-primitives matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-family certification rows.
    pub rows: Vec<DurableProgressCertificationRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items
        .iter()
        .map(|item| to_token(item))
        .collect::<Vec<_>>()
        .join("|")
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
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Builds a [`DurableProgressCertificationPacket`] from the exact build identity, the
/// frozen matrix ref, and the per-family certification rows.
///
/// Each row's derived status and certification causes, the aggregate counts, the active
/// waivers, and the blocking findings are recomputed here so the packet is the single
/// source of truth and the auto-narrowing cannot be asserted.
pub fn build_m5_durable_progress_certification_packet(
    input: DurableProgressCertificationInput,
) -> DurableProgressCertificationPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent
    // and the auto-narrowing is the single source of truth.
    let rows: Vec<DurableProgressCertificationRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.certification_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<DurableProgressCertificationFinding> = Vec::new();

    // Every governed family must carry a certification row.
    let present: BTreeSet<M5DurableJobFamily> = rows.iter().map(|row| row.family).collect();
    for family in M5DurableJobFamily::ALL {
        if !present.contains(&family) {
            blocking_findings.push(DurableProgressCertificationFinding::FamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_families: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|family| family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| {
            matches!(
                row.derived_status,
                DurableProgressCertificationStatus::Green
            )
        })
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| {
            matches!(
                row.derived_status,
                DurableProgressCertificationStatus::Yellow
            )
        })
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, DurableProgressCertificationStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(DurableProgressCertificationFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<DurableProgressCertificationWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let certification_causes: Vec<DurableProgressCertificationCause> = rows
        .iter()
        .flat_map(|row| row.certification_causes.clone())
        .collect();

    let mut packet = DurableProgressCertificationPacket {
        record_kind: M5_DURABLE_PROGRESS_CERTIFICATION_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_DURABLE_PROGRESS_CERTIFICATION_SCHEMA_VERSION,
        shared_contract_ref: M5_DURABLE_PROGRESS_CERTIFICATION_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_DURABLE_PROGRESS_CERTIFICATION_PACKET_ID.to_owned(),
        source_schema_ref: M5_DURABLE_PROGRESS_CERTIFICATION_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Durable progress-indicator and job-row actor, phase, action, and history \
                   parity certified across every claimed M5 durable-work job family: indexing, \
                   notebook/runtime, request/data-load, download, update, sync, branch-agent, \
                   provider-handoff, and support/export each stay reviewable after focus loss \
                   rather than as a transient spinner or toast, attribute their actor/subsystem, \
                   phase, action affordances, and authoritative-object links, preserve grouped \
                   completion/failure history and blocked/paused reasons in reopenable history, \
                   and reconstruct current progress and recent job history from a support export — \
                   with each row's green/yellow/red claim auto-narrowed from its durable-presence, \
                   attribution, grouped-history, and export posture."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_DURABLE_PROGRESS_CERTIFICATION_MATRIX_SCHEMA_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_proof_dimensions: M5DurableProgressProofDimension::ALL.to_vec(),
        required_progress_states: M5ProgressState::ALL.to_vec(),
        required_labels: DURABLE_PROGRESS_REQUIRED_LABELS.to_vec(),
        rows,
        covered_families,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        certification_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        shell_automation_refs: vec![
            "shell_frame.activity_center.durable_progress_registry".to_owned(),
            "release_automation.auto_narrow.durable_progress_certification_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.durable_progress_certification".to_owned(),
            "artifacts/release/m5-durable-progress-certification-proof/packet.json".to_owned(),
        ],
        help_docs_refs: vec![M5_DURABLE_PROGRESS_CERTIFICATION_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-durable-progress-certification".to_owned()],
        published_report_ref: M5_DURABLE_PROGRESS_CERTIFICATION_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_DURABLE_PROGRESS_CERTIFICATION_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_DURABLE_PROGRESS_CERTIFICATION_PUBLISHED_DASHBOARD_REF
            .to_owned(),
        published_doc_ref: M5_DURABLE_PROGRESS_CERTIFICATION_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("certification packet serializes"),
    ) {
        blocking_findings.push(DurableProgressCertificationFinding::RawBoundaryMaterialInExport);
    }

    blocking_findings.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    packet.report_clean = blocking_findings.is_empty();
    packet.blocking_findings = blocking_findings;

    packet
}

/// Validation error produced by [`validate_m5_durable_progress_certification_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum DurableProgressCertificationValidationError {
    /// The packet has no rows.
    NoRows,
    /// The packet's record kind is wrong.
    WrongRecordKind,
    /// The packet's schema version is wrong.
    WrongSchemaVersion,
    /// The packet's exact-build identity ref is empty.
    BuildIdentityRefMissing,
    /// The packet does not certify a frozen matrix packet.
    MatrixPacketRefMissing,
    /// The rows do not cover all nine governed families.
    CoverageIncomplete,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// The declared required proof dimensions are not the canonical set.
    RequiredDimensionsStale,
    /// The declared required progress states are not the canonical set.
    RequiredProgressStatesStale,
    /// The declared required labels are not the durable-progress set.
    RequiredLabelsStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared certification causes do not match the recomputed causes.
    CertificationCausesStale,
    /// The declared blocking findings do not match the recomputed findings.
    BlockingFindingsStale,
    /// A blocking finding remains in the packet.
    BlockingFindingPresent {
        /// Finding class.
        class: String,
        /// Owning subject ref.
        subject_ref: String,
    },
    /// The published report ref is empty.
    PublishedReportRefMissing,
    /// The published packet ref is empty.
    PublishedPacketRefMissing,
    /// The published dashboard ref is empty.
    PublishedDashboardRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
}

/// Validates a packet against the durable-progress certification invariants.
///
/// The checks encode the track invariant and acceptance criteria: every governed job
/// family carries a current certification row; each row's status is the derived
/// auto-narrowed value, never asserted; a green row cannot keep a claim while durable work
/// is spinner-or-toast-only, attribution or the authoritative-object link is missing,
/// grouped history or a blocked/paused reason is lost, the progress state is dropped from
/// capture, a job is spinner-or-toast-only, or its progress states / required labels are
/// incomplete; and a disclosed narrowing is backed by a reason and, where required, an
/// active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_durable_progress_certification_packet(
    packet: &DurableProgressCertificationPacket,
) -> Result<(), Vec<DurableProgressCertificationValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(DurableProgressCertificationValidationError::NoRows);
    }
    if packet.record_kind != M5_DURABLE_PROGRESS_CERTIFICATION_PACKET_RECORD_KIND {
        errors.push(DurableProgressCertificationValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_DURABLE_PROGRESS_CERTIFICATION_SCHEMA_VERSION {
        errors.push(DurableProgressCertificationValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(DurableProgressCertificationValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(DurableProgressCertificationValidationError::MatrixPacketRefMissing);
    }
    if packet.required_proof_dimensions != M5DurableProgressProofDimension::ALL {
        errors.push(DurableProgressCertificationValidationError::RequiredDimensionsStale);
    }
    if packet.required_progress_states != M5ProgressState::ALL {
        errors.push(DurableProgressCertificationValidationError::RequiredProgressStatesStale);
    }
    if packet.required_labels != DURABLE_PROGRESS_REQUIRED_LABELS {
        errors.push(DurableProgressCertificationValidationError::RequiredLabelsStale);
    }

    let present: BTreeSet<M5DurableJobFamily> = packet.rows.iter().map(|row| row.family).collect();
    let coverage_complete = M5DurableJobFamily::ALL
        .iter()
        .all(|family| present.contains(family));
    if !coverage_complete || packet.rows.len() != M5DurableJobFamily::ALL.len() {
        errors.push(DurableProgressCertificationValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|family| family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_families {
        errors.push(DurableProgressCertificationValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| {
            matches!(
                row.recompute_status(),
                DurableProgressCertificationStatus::Green
            )
        })
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| {
            matches!(
                row.recompute_status(),
                DurableProgressCertificationStatus::Yellow
            )
        })
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| {
            matches!(
                row.recompute_status(),
                DurableProgressCertificationStatus::Red
            )
        })
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(DurableProgressCertificationValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<DurableProgressCertificationWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(DurableProgressCertificationValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<DurableProgressCertificationCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.certification_causes {
        errors.push(DurableProgressCertificationValidationError::CertificationCausesStale);
    }

    let mut recomputed: Vec<DurableProgressCertificationFinding> = Vec::new();
    for family in M5DurableJobFamily::ALL {
        if !present.contains(&family) {
            recomputed.push(DurableProgressCertificationFinding::FamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(DurableProgressCertificationFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("certification packet serializes"),
    ) {
        recomputed.push(DurableProgressCertificationFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(DurableProgressCertificationValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(
            DurableProgressCertificationValidationError::BlockingFindingPresent {
                class: finding.class_token().to_owned(),
                subject_ref: finding.subject_ref().to_owned(),
            },
        );
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(DurableProgressCertificationValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(DurableProgressCertificationValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(DurableProgressCertificationValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(DurableProgressCertificationValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
