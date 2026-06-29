//! Typed release-note evidence rows — the what's-new / release-note surface that separates marketing
//! prose from a controlled [change class](ChangeClass), the affected scope, support sensitivity, and
//! **direct action links** (evidence, migration docs, rollback controls, certification deltas,
//! setting / import surfaces) on every behavior-changing or security-sensitive M5 update.
//!
//! The [typed update-center summary objects](crate::m5_update_summary) answer "what is changing, and
//! did it verify"; the [change-impact cards](crate::m5_change_impact_card) answer "what will the change
//! do before restart"; this lane answers the exit-gate question: *does this release note actually
//! reduce risk* — and it does so only when it separates prose from evidence, change class, and a direct
//! link to the relevant setting, import, or rollback surface.
//!
//! Each [release-note evidence row](ReleaseNoteEvidenceRow) carries:
//!
//! - a controlled [change class](ChangeClass) (`breaking`, `behavioral`, `security`, `policy`,
//!   `compatibility`, `docs_only`, `admin_action_required`, `deprecated`, `migration_required`) so a
//!   behavior-changing note can never read like a routine docs touch-up;
//! - one or more [evidence links](EvidenceLink) — the lane's core invariant is that a behavior-changing
//!   or security-sensitive note links to *evidence, migration docs, rollback controls, or certification
//!   deltas* rather than prose alone, and a breaking or migration-relevant note links *directly* to the
//!   setting / import / rollback surface;
//! - a [what's-new card](WhatsNewCard) that is always dismissible and reopenable from the update center
//!   or Help, and that **never blocks typing, save, restore, or recovery-critical workflows**; and
//! - the affected artifact classes, profiles, and channels, with support sensitivity derived from the
//!   channels so a stable / LTS note is never overloaded with roadmap marketing.
//!
//! The [consumer surfaces](ReleaseNoteConsumer) — update center, what's-new panel, Help center,
//! docs/Help, release center, support export — bind the notes they read and *derive* their
//! [readiness](NoteReadiness) and gaps from the rows, so all of them read this one
//! [`ReleaseNoteEvidenceSet`] packet under one vocabulary and one schema rather than cloning a parallel
//! release-note stream.
//!
//! The packet is inspectable and serde-serializable; it carries metadata, refs, and message ids only —
//! no credential bodies or raw provider payloads, and no free-form prose — so the same set renders
//! byte-identically across the app, docs/Help, and exported summaries.
//!
//! - Packet schema:
//!   [`schemas/release/m5-release-note-evidence-row.schema.json`](../../../../../schemas/release/m5-release-note-evidence-row.schema.json)
//! - Vocabulary artifact:
//!   [`artifacts/release/m5-release-note-vocabulary.md`](../../../../../artifacts/release/m5-release-note-vocabulary.md)
//! - Contract doc:
//!   [`docs/release/m5-release-note-evidence-contract.md`](../../../../../docs/release/m5-release-note-evidence-contract.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_release_note_evidence_set, seeded_m5_release_note_evidence_set_dismissed,
    seeded_m5_release_note_evidence_set_docs_only,
    seeded_m5_release_note_evidence_set_security_and_migration,
    M5_RELEASE_NOTE_EVIDENCE_SET_PACKET_ID,
};

use serde::{Deserialize, Serialize};

// The release-note rows reuse the update / support-lifecycle vocabularies for artifact class, channel,
// and deployment profile, and the descriptor / badge runtime's gate / status / signal vocabulary, so
// this communication layer can never drift to a different vocabulary than the layers above it.
use crate::m5_descriptor_badge::{ConsumerStatus, DescriptorGate, DescriptorSignal};
use crate::m5_update_lifecycle::{ArtifactClass, ChannelScope, DeploymentProfile};

/// Record-kind tag carried by [`ReleaseNoteEvidenceSet`].
pub const M5_RELEASE_NOTE_EVIDENCE_SET_RECORD_KIND: &str = "m5_release_note_evidence_set";

/// Schema version for the release-note evidence-set packet.
pub const M5_RELEASE_NOTE_EVIDENCE_SET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the evidence-set packet schema.
pub const M5_RELEASE_NOTE_EVIDENCE_SCHEMA_REF: &str =
    "schemas/release/m5-release-note-evidence-row.schema.json";

/// Repo-relative path of the published evidence-set inventory.
pub const M5_RELEASE_NOTE_EVIDENCE_SET_REF: &str =
    "artifacts/release/m5-release-note-evidence.json";

/// Repo-relative path of the release-grade evidence-set parity proof.
pub const M5_RELEASE_NOTE_EVIDENCE_SET_PROOF_REF: &str =
    "artifacts/release/m5-release-note-proof/release-note-evidence.json";

/// Repo-relative path of the machine-readable per-note CSV export.
pub const M5_RELEASE_NOTE_EVIDENCE_SET_CSV_REF: &str =
    "artifacts/release/m5-release-note-evidence.csv";

/// Repo-relative path of the published change-class / link-kind vocabulary.
pub const M5_RELEASE_NOTE_EVIDENCE_VOCABULARY_REF: &str =
    "artifacts/release/m5-release-note-vocabulary.md";

/// Repo-relative path of the evidence-set contract doc.
pub const M5_RELEASE_NOTE_EVIDENCE_SET_DOC_REF: &str =
    "docs/release/m5-release-note-evidence-contract.md";

/// Repo-relative directory of the per-state evidence-set fixtures.
pub const M5_RELEASE_NOTE_EVIDENCE_SET_FIXTURE_DIR: &str =
    "fixtures/release/whats-new-and-migration/";

/// Prefix every release-note message id carries so consumers can route it.
pub const M5_RELEASE_NOTE_MESSAGE_ID_PREFIX: &str = "release_note_evidence.";

const REDACTION_CLASS: &str = "metadata_safe_default";

// ---------------------------------------------------------------------------
// Controlled vocabularies
// ---------------------------------------------------------------------------

/// The controlled change class a release note carries. The set is frozen so what's-new and release-note
/// surfaces speak one vocabulary across the app, docs/Help, and exported summaries; declaration order is
/// least→most action-demanding, and the vocabulary deliberately separates a routine
/// [`DocsOnly`](Self::DocsOnly) note from a [`Breaking`](Self::Breaking) or
/// [`Security`](Self::Security) one so the two can never read alike.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeClass {
    /// Docs / Help content only; no behavior change.
    DocsOnly,
    /// A compatibility-window change (e.g. a supported-range shift) with no behavior change.
    Compatibility,
    /// A user-facing behavior change.
    Behavioral,
    /// A policy / governance change the user should acknowledge.
    Policy,
    /// A feature or interface is deprecated ahead of removal.
    Deprecated,
    /// A migration is required before the change is complete.
    MigrationRequired,
    /// An administrator must take an action before or after the update.
    AdminActionRequired,
    /// A security-sensitive change (advisory, fix, or hardening).
    Security,
    /// A breaking change to a public interface or workflow.
    Breaking,
}

impl ChangeClass {
    /// Every change class, least→most action-demanding.
    pub const ALL: [Self; 9] = [
        Self::DocsOnly,
        Self::Compatibility,
        Self::Behavioral,
        Self::Policy,
        Self::Deprecated,
        Self::MigrationRequired,
        Self::AdminActionRequired,
        Self::Security,
        Self::Breaking,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsOnly => "docs_only",
            Self::Compatibility => "compatibility",
            Self::Behavioral => "behavioral",
            Self::Policy => "policy",
            Self::Deprecated => "deprecated",
            Self::MigrationRequired => "migration_required",
            Self::AdminActionRequired => "admin_action_required",
            Self::Security => "security",
            Self::Breaking => "breaking",
        }
    }

    /// Human-facing label for the change class.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DocsOnly => "Docs only",
            Self::Compatibility => "Compatibility",
            Self::Behavioral => "Behavioral",
            Self::Policy => "Policy",
            Self::Deprecated => "Deprecated",
            Self::MigrationRequired => "Migration required",
            Self::AdminActionRequired => "Admin action required",
            Self::Security => "Security",
            Self::Breaking => "Breaking",
        }
    }

    /// Accountable owner role for this change class.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::DocsOnly => "docs_owner",
            Self::Compatibility => "compatibility_owner",
            Self::Behavioral => "product_behavior_owner",
            Self::Policy => "policy_owner",
            Self::Deprecated => "deprecation_owner",
            Self::MigrationRequired => "migration_owner",
            Self::AdminActionRequired => "admin_owner",
            Self::Security => "security_response_owner",
            Self::Breaking => "public_interface_owner",
        }
    }

    /// True for every class except [`DocsOnly`](Self::DocsOnly): a note that changes behavior or is
    /// security-sensitive must link to evidence, migration docs, rollback controls, or a certification
    /// delta rather than prose alone.
    pub const fn is_behavior_changing_or_security_sensitive(self) -> bool {
        !matches!(self, Self::DocsOnly)
    }

    /// True for the classes that must link *directly* to the relevant setting, import, or rollback
    /// surface — a breaking, migration-required, or admin-action-required note.
    pub const fn requires_direct_action_link(self) -> bool {
        matches!(
            self,
            Self::Breaking | Self::MigrationRequired | Self::AdminActionRequired
        )
    }

    /// The communication-severity gate this change class implies. The gate maps one-to-one to a
    /// [readiness](NoteReadiness); it classifies how much action the note asks for and never blocks a
    /// workflow.
    pub const fn severity_gate(self) -> DescriptorGate {
        match self {
            Self::DocsOnly | Self::Compatibility => DescriptorGate::Governed,
            Self::Behavioral | Self::Policy | Self::Deprecated => DescriptorGate::Narrowed,
            Self::MigrationRequired
            | Self::AdminActionRequired
            | Self::Security
            | Self::Breaking => DescriptorGate::Blocked,
        }
    }
}

/// The kind of direct link a release-note evidence row carries. The kinds are distinct so a note never
/// implies it links to a rollback control when it only links to a doc, and so a behavior-changing note
/// can be checked for *substantive* evidence rather than a bare prose pointer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceLinkKind {
    /// A release evidence / proof packet backing the claim.
    EvidencePacket,
    /// A published security advisory (CVE / GHSA).
    SecurityAdvisory,
    /// A migration guide / docs page describing the required migration.
    MigrationDoc,
    /// A certification / qualification delta.
    CertificationDelta,
    /// A direct link to the rollback / pin control.
    RollbackControl,
    /// A direct link to the relevant setting surface.
    SettingSurface,
    /// A direct link to the import / migration-assistant surface.
    ImportSurface,
    /// A docs / Help page (the lighter reference a docs-only note carries).
    DocsPage,
}

impl EvidenceLinkKind {
    /// Every link kind, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::EvidencePacket,
        Self::SecurityAdvisory,
        Self::MigrationDoc,
        Self::CertificationDelta,
        Self::RollbackControl,
        Self::SettingSurface,
        Self::ImportSurface,
        Self::DocsPage,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidencePacket => "evidence_packet",
            Self::SecurityAdvisory => "security_advisory",
            Self::MigrationDoc => "migration_doc",
            Self::CertificationDelta => "certification_delta",
            Self::RollbackControl => "rollback_control",
            Self::SettingSurface => "setting_surface",
            Self::ImportSurface => "import_surface",
            Self::DocsPage => "docs_page",
        }
    }

    /// Human-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::EvidencePacket => "Evidence packet",
            Self::SecurityAdvisory => "Security advisory",
            Self::MigrationDoc => "Migration doc",
            Self::CertificationDelta => "Certification delta",
            Self::RollbackControl => "Rollback control",
            Self::SettingSurface => "Setting surface",
            Self::ImportSurface => "Import surface",
            Self::DocsPage => "Docs page",
        }
    }

    /// True for the in-app surfaces a user acts on directly: the rollback control, the setting surface,
    /// or the import surface. A breaking or migration-relevant note must carry at least one of these.
    pub const fn is_direct_action(self) -> bool {
        matches!(
            self,
            Self::RollbackControl | Self::SettingSurface | Self::ImportSurface
        )
    }

    /// True for the link kinds that count as *substantive evidence* backing the claim — an evidence
    /// packet, security advisory, migration doc, certification delta, or rollback control — as opposed
    /// to a bare docs pointer. A behavior-changing or security-sensitive note must carry at least one.
    pub const fn is_substantive_evidence(self) -> bool {
        matches!(
            self,
            Self::EvidencePacket
                | Self::SecurityAdvisory
                | Self::MigrationDoc
                | Self::CertificationDelta
                | Self::RollbackControl
        )
    }
}

/// The readiness a release note or consumer resolves to: a direct reading of the
/// [communication-severity gate](ChangeClass::severity_gate) in user-facing language. It classifies how
/// much action a note asks for and never blocks a workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteReadiness {
    /// Informational: no action is required; read when convenient.
    Informational,
    /// Action recommended: review or migrate when ready.
    ActionRecommended,
    /// Action required: a setting / import / rollback action is called for.
    ActionRequired,
}

impl NoteReadiness {
    /// Every readiness, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::Informational,
        Self::ActionRecommended,
        Self::ActionRequired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Informational => "informational",
            Self::ActionRecommended => "action_recommended",
            Self::ActionRequired => "action_required",
        }
    }

    /// The readiness a gate resolves to.
    pub const fn from_gate(gate: DescriptorGate) -> Self {
        match gate {
            DescriptorGate::Governed => Self::Informational,
            DescriptorGate::Narrowed => Self::ActionRecommended,
            DescriptorGate::Blocked => Self::ActionRequired,
        }
    }
}

/// The dismiss state of a what's-new card. A card is always dismissible and reopenable; this records
/// whether it is currently showing or has been dismissed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WhatsNewDismissState {
    /// The card is currently shown.
    Active,
    /// The card has been dismissed; it remains reopenable from the update center or Help.
    Dismissed,
}

impl WhatsNewDismissState {
    /// Every dismiss state, in declaration order.
    pub const ALL: [Self; 2] = [Self::Active, Self::Dismissed];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Dismissed => "dismissed",
        }
    }
}

/// A surface a dismissed what's-new card can be reopened from. Every card is reopenable from both, so a
/// user can always recover release communication later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenSurface {
    /// The update center.
    UpdateCenter,
    /// The Help center / About surface.
    HelpCenter,
}

impl ReopenSurface {
    /// Every reopen surface, in declaration order.
    pub const ALL: [Self; 2] = [Self::UpdateCenter, Self::HelpCenter];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UpdateCenter => "update_center",
            Self::HelpCenter => "help_center",
        }
    }
}

/// The named cause of a consumer's readiness gap on one note it reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteGapKind {
    /// A read note recommends action (a behavioral / policy / deprecation change).
    ActionRecommended,
    /// A read note requires action (a breaking / migration / admin / security change).
    ActionRequired,
    /// A note the consumer reads is not published in the set.
    NoteNotPublished,
}

impl NoteGapKind {
    /// Every gap kind, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ActionRecommended,
        Self::ActionRequired,
        Self::NoteNotPublished,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActionRecommended => "action_recommended",
            Self::ActionRequired => "action_required",
            Self::NoteNotPublished => "note_not_published",
        }
    }

    /// The gate this gap forces.
    const fn gate(self) -> DescriptorGate {
        match self {
            Self::ActionRecommended => DescriptorGate::Narrowed,
            Self::ActionRequired | Self::NoteNotPublished => DescriptorGate::Blocked,
        }
    }
}

/// One claimed consumer surface that reads the release-note evidence rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseNoteConsumer {
    /// The in-product update center.
    UpdateCenter,
    /// The in-product what's-new panel.
    WhatsNewPanel,
    /// The Help center / About surface.
    HelpCenter,
    /// The published docs/Help release notes.
    DocsHelp,
    /// The release center / public-truth automation.
    ReleaseCenter,
    /// The support export.
    SupportExport,
}

impl ReleaseNoteConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::UpdateCenter,
        Self::WhatsNewPanel,
        Self::HelpCenter,
        Self::DocsHelp,
        Self::ReleaseCenter,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UpdateCenter => "update_center",
            Self::WhatsNewPanel => "whats_new_panel",
            Self::HelpCenter => "help_center",
            Self::DocsHelp => "docs_help",
            Self::ReleaseCenter => "release_center",
            Self::SupportExport => "support_export",
        }
    }

    /// Human-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::UpdateCenter => "Update center",
            Self::WhatsNewPanel => "What's-new panel",
            Self::HelpCenter => "Help center",
            Self::DocsHelp => "Docs/Help release notes",
            Self::ReleaseCenter => "Release center",
            Self::SupportExport => "Support export",
        }
    }

    /// Accountable owner role.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::UpdateCenter => "update_center_owner",
            Self::WhatsNewPanel => "whats_new_owner",
            Self::HelpCenter => "help_center_owner",
            Self::DocsHelp => "docs_help_owner",
            Self::ReleaseCenter => "release_center_owner",
            Self::SupportExport => "support_export_owner",
        }
    }
}

// ---------------------------------------------------------------------------
// Ranking helpers for deterministic ordering
// ---------------------------------------------------------------------------

fn change_class_rank(c: ChangeClass) -> usize {
    ChangeClass::ALL
        .iter()
        .position(|x| *x == c)
        .unwrap_or(usize::MAX)
}

fn link_kind_rank(k: EvidenceLinkKind) -> usize {
    EvidenceLinkKind::ALL
        .iter()
        .position(|x| *x == k)
        .unwrap_or(usize::MAX)
}

fn artifact_rank(c: ArtifactClass) -> usize {
    ArtifactClass::ALL
        .iter()
        .position(|x| *x == c)
        .unwrap_or(usize::MAX)
}

fn profile_rank(p: DeploymentProfile) -> usize {
    DeploymentProfile::ALL
        .iter()
        .position(|x| *x == p)
        .unwrap_or(usize::MAX)
}

fn channel_rank(c: ChannelScope) -> usize {
    ChannelScope::ALL
        .iter()
        .position(|x| *x == c)
        .unwrap_or(usize::MAX)
}

fn consumer_rank(c: ReleaseNoteConsumer) -> usize {
    ReleaseNoteConsumer::ALL
        .iter()
        .position(|x| *x == c)
        .unwrap_or(usize::MAX)
}

fn gate_rank(g: DescriptorGate) -> u8 {
    match g {
        DescriptorGate::Governed => 0,
        DescriptorGate::Narrowed => 1,
        DescriptorGate::Blocked => 2,
    }
}

fn worst_gate(a: DescriptorGate, b: DescriptorGate) -> DescriptorGate {
    if gate_rank(a) >= gate_rank(b) {
        a
    } else {
        b
    }
}

fn status_for_gate(gate: DescriptorGate) -> ConsumerStatus {
    match gate {
        DescriptorGate::Governed => ConsumerStatus::Mapped,
        DescriptorGate::Narrowed => ConsumerStatus::Provisional,
        DescriptorGate::Blocked => ConsumerStatus::Unmapped,
    }
}

fn signal_for_gate(gate: DescriptorGate) -> DescriptorSignal {
    match gate {
        DescriptorGate::Governed => DescriptorSignal::Green,
        DescriptorGate::Narrowed => DescriptorSignal::Yellow,
        DescriptorGate::Blocked => DescriptorSignal::Red,
    }
}

/// True for a support-sensitive channel (general-availability or long-term-support), where a what's-new
/// card must never be overloaded with roadmap marketing.
fn channel_is_support_sensitive(channel: ChannelScope) -> bool {
    matches!(channel, ChannelScope::Stable | ChannelScope::Lts)
}

// ---------------------------------------------------------------------------
// Evidence link
// ---------------------------------------------------------------------------

/// One direct link a release-note evidence row carries: its [kind](EvidenceLinkKind), whether it is a
/// direct in-app action and substantive evidence (both derived from the kind), an opaque target ref,
/// and a routable message id. The target ref is a path or in-app route token — never a raw payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceLink {
    /// The kind of link.
    pub kind: EvidenceLinkKind,
    /// True when the link is a direct in-app action surface (rollback / setting / import).
    pub direct_action: bool,
    /// True when the link counts as substantive evidence (not a bare docs pointer).
    pub substantive_evidence: bool,
    /// Opaque target ref (a repo-relative path or in-app route token); never a raw payload.
    pub target_ref: String,
    /// Routable message id for the link's label.
    pub link_message_id: String,
}

impl EvidenceLink {
    /// Builds an evidence link, deriving the [direct-action](EvidenceLinkKind::is_direct_action) and
    /// [substantive-evidence](EvidenceLinkKind::is_substantive_evidence) flags from the kind.
    pub fn new(note_id: &str, kind: EvidenceLinkKind, target_ref: &str) -> Self {
        Self {
            kind,
            direct_action: kind.is_direct_action(),
            substantive_evidence: kind.is_substantive_evidence(),
            target_ref: target_ref.to_owned(),
            link_message_id: format!(
                "{}note.{}.link.{}",
                M5_RELEASE_NOTE_MESSAGE_ID_PREFIX,
                note_id,
                kind.as_str()
            ),
        }
    }

    /// Recomputes the derived flags from the kind.
    fn recompute(&mut self) {
        self.direct_action = self.kind.is_direct_action();
        self.substantive_evidence = self.kind.is_substantive_evidence();
    }
}

// ---------------------------------------------------------------------------
// What's-new card
// ---------------------------------------------------------------------------

/// The what's-new card backing a release-note evidence row. It is always dismissible and reopenable
/// from the update center or Help, and it **never** blocks typing, save, restore, or recovery-critical
/// workflows. The blocking flags are recorded explicitly so a tampered packet that flips one to `true`
/// fails [`ReleaseNoteEvidenceSet::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WhatsNewCard {
    /// The card can be dismissed.
    pub dismissible: bool,
    /// The card can be reopened after dismissal.
    pub reopenable: bool,
    /// The card never blocks typing.
    pub blocks_typing: bool,
    /// The card never blocks save.
    pub blocks_save: bool,
    /// The card never blocks restore.
    pub blocks_restore: bool,
    /// The card never blocks a recovery-critical workflow.
    pub blocks_recovery: bool,
    /// The current dismiss state.
    pub dismiss_state: WhatsNewDismissState,
    /// The surfaces the card can be reopened from (the update center and Help).
    pub reopen_surfaces: Vec<ReopenSurface>,
    /// Routable message id for the card.
    pub card_message_id: String,
    /// Routable message id for the reopen affordance.
    pub reopen_message_id: String,
}

impl WhatsNewCard {
    fn with_state(note_id: &str, dismiss_state: WhatsNewDismissState) -> Self {
        Self {
            dismissible: true,
            reopenable: true,
            blocks_typing: false,
            blocks_save: false,
            blocks_restore: false,
            blocks_recovery: false,
            dismiss_state,
            reopen_surfaces: ReopenSurface::ALL.to_vec(),
            card_message_id: format!(
                "{}note.{}.whats_new",
                M5_RELEASE_NOTE_MESSAGE_ID_PREFIX, note_id
            ),
            reopen_message_id: format!(
                "{}note.{}.reopen",
                M5_RELEASE_NOTE_MESSAGE_ID_PREFIX, note_id
            ),
        }
    }

    /// An active (currently shown) what's-new card.
    pub fn active(note_id: &str) -> Self {
        Self::with_state(note_id, WhatsNewDismissState::Active)
    }

    /// A dismissed what's-new card; it remains reopenable from the update center and Help.
    pub fn dismissed(note_id: &str) -> Self {
        Self::with_state(note_id, WhatsNewDismissState::Dismissed)
    }

    fn recompute_surfaces(&mut self) {
        self.reopen_surfaces.sort_by_key(|s| {
            ReopenSurface::ALL
                .iter()
                .position(|x| *x == *s)
                .unwrap_or(usize::MAX)
        });
        self.reopen_surfaces.dedup();
    }

    /// True when the card never blocks any workflow.
    pub const fn is_non_blocking(&self) -> bool {
        !self.blocks_typing && !self.blocks_save && !self.blocks_restore && !self.blocks_recovery
    }

    /// True when the card is dismissible and reopenable from both the update center and Help.
    pub fn is_reopenable_everywhere(&self) -> bool {
        self.dismissible
            && self.reopenable
            && self.reopen_surfaces.contains(&ReopenSurface::UpdateCenter)
            && self.reopen_surfaces.contains(&ReopenSurface::HelpCenter)
    }
}

// ---------------------------------------------------------------------------
// Release-note evidence row
// ---------------------------------------------------------------------------

/// Builder input for [`ReleaseNoteEvidenceRow::new`].
#[derive(Debug, Clone)]
pub struct ReleaseNoteEvidenceRowInput {
    /// Stable, slug-style note id (used to route message ids).
    pub note_id: String,
    /// The controlled change class.
    pub change_class: ChangeClass,
    /// The channels the note applies to.
    pub channels: Vec<ChannelScope>,
    /// Artifact classes the change affects.
    pub affected_artifact_classes: Vec<ArtifactClass>,
    /// Deployment profiles the change affects.
    pub affected_profiles: Vec<DeploymentProfile>,
    /// The version the note moves from (absent when not applicable).
    pub from_version: Option<String>,
    /// The version the note moves to (absent when not applicable).
    pub to_version: Option<String>,
    /// The direct links backing the note.
    pub evidence_links: Vec<EvidenceLink>,
    /// The what's-new card.
    pub whats_new_card: WhatsNewCard,
}

/// The typed evidence row for one release note: its [change class](ChangeClass), affected scope,
/// support sensitivity, [evidence links](EvidenceLink), [what's-new card](WhatsNewCard), and derived
/// readiness verdict. The packet carries no free-form prose — only the change class, controlled labels,
/// refs, and routable message ids — so the row separates marketing from evidence by construction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseNoteEvidenceRow {
    /// Stable note id.
    pub note_id: String,
    /// Routable message id for the note's headline (no prose stored in the packet).
    pub headline_message_id: String,
    /// The controlled change class.
    pub change_class: ChangeClass,
    /// Human-facing change-class label.
    pub change_class_label: String,
    /// Accountable owner role.
    pub owner_role: String,
    /// The channels the note applies to.
    pub channels: Vec<ChannelScope>,
    /// True when any channel is support-sensitive (stable / LTS).
    pub support_sensitive: bool,
    /// The artifact classes the change affects.
    pub affected_artifact_classes: Vec<ArtifactClass>,
    /// The deployment profiles the change affects.
    pub affected_profiles: Vec<DeploymentProfile>,
    /// The version the note moves from.
    pub from_version: Option<String>,
    /// The version the note moves to.
    pub to_version: Option<String>,
    /// The direct links backing the note.
    pub evidence_links: Vec<EvidenceLink>,
    /// The what's-new card.
    pub whats_new_card: WhatsNewCard,
    /// True when the note carries at least one substantive evidence link.
    pub has_substantive_evidence: bool,
    /// True when the note carries at least one direct-action link.
    pub has_direct_action_link: bool,
    /// True when the note asks the user to take an action (gate is blocked).
    pub requires_user_action: bool,
    /// Communication-severity gate derived from the change class.
    pub gate: DescriptorGate,
    /// Readiness mirroring [`gate`](Self::gate).
    pub note_readiness: NoteReadiness,
    /// Coverage status mirroring [`gate`](Self::gate).
    pub status: ConsumerStatus,
    /// Traffic-light signal mirroring [`gate`](Self::gate).
    pub signal: DescriptorSignal,
    /// Routable message id for the note's detail.
    pub detail_message_id: String,
}

impl ReleaseNoteEvidenceRow {
    /// Builds a row from its inputs, deriving the gate, readiness, support sensitivity, and link
    /// summaries.
    pub fn new(input: ReleaseNoteEvidenceRowInput) -> Self {
        let note_id = input.note_id;
        let change_class = input.change_class;
        let mut row = Self {
            headline_message_id: format!(
                "{}note.{}.headline",
                M5_RELEASE_NOTE_MESSAGE_ID_PREFIX, note_id
            ),
            detail_message_id: format!(
                "{}note.{}.detail",
                M5_RELEASE_NOTE_MESSAGE_ID_PREFIX, note_id
            ),
            note_id,
            change_class,
            change_class_label: change_class.label().to_owned(),
            owner_role: change_class.owner_role().to_owned(),
            channels: input.channels,
            support_sensitive: false,
            affected_artifact_classes: input.affected_artifact_classes,
            affected_profiles: input.affected_profiles,
            from_version: input.from_version,
            to_version: input.to_version,
            evidence_links: input.evidence_links,
            whats_new_card: input.whats_new_card,
            has_substantive_evidence: false,
            has_direct_action_link: false,
            requires_user_action: false,
            gate: DescriptorGate::Governed,
            note_readiness: NoteReadiness::Informational,
            status: ConsumerStatus::Mapped,
            signal: DescriptorSignal::Green,
        };
        row.recompute();
        row
    }

    /// Recomputes the derived scope, link summaries, and verdict from the row's inputs.
    pub fn recompute(&mut self) {
        let mut channels = self.channels.clone();
        channels.sort_by_key(|c| channel_rank(*c));
        channels.dedup();
        self.channels = channels;
        self.support_sensitive = self
            .channels
            .iter()
            .copied()
            .any(channel_is_support_sensitive);

        let mut classes = self.affected_artifact_classes.clone();
        classes.sort_by_key(|c| artifact_rank(*c));
        classes.dedup();
        self.affected_artifact_classes = classes;

        let mut profiles = self.affected_profiles.clone();
        profiles.sort_by_key(|p| profile_rank(*p));
        profiles.dedup();
        self.affected_profiles = profiles;

        for link in &mut self.evidence_links {
            link.recompute();
        }
        self.evidence_links.sort_by(|a, b| {
            link_kind_rank(a.kind)
                .cmp(&link_kind_rank(b.kind))
                .then(a.target_ref.cmp(&b.target_ref))
        });
        self.has_substantive_evidence = self.evidence_links.iter().any(|l| l.substantive_evidence);
        self.has_direct_action_link = self.evidence_links.iter().any(|l| l.direct_action);

        self.whats_new_card.recompute_surfaces();

        let gate = self.change_class.severity_gate();
        self.gate = gate;
        self.note_readiness = NoteReadiness::from_gate(gate);
        self.status = status_for_gate(gate);
        self.signal = signal_for_gate(gate);
        self.requires_user_action = gate == DescriptorGate::Blocked;
    }

    /// True when the note has a security advisory link.
    fn has_security_advisory(&self) -> bool {
        self.evidence_links
            .iter()
            .any(|l| l.kind == EvidenceLinkKind::SecurityAdvisory)
    }

    /// The gap kind this note contributes to a consumer that reads it, if any.
    fn gap_kind(&self) -> Option<NoteGapKind> {
        match self.gate {
            DescriptorGate::Governed => None,
            DescriptorGate::Narrowed => Some(NoteGapKind::ActionRecommended),
            DescriptorGate::Blocked => Some(NoteGapKind::ActionRequired),
        }
    }
}

// ---------------------------------------------------------------------------
// Consumer rows
// ---------------------------------------------------------------------------

/// A readiness gap a consumer carries for one note it reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoteGap {
    /// The consumer that carries the gap.
    pub consumer: ReleaseNoteConsumer,
    /// The note id that caused the gap.
    pub note_id: String,
    /// The note's change class (or the most severe class when the note is unpublished).
    pub change_class: Option<ChangeClass>,
    /// The named cause of the gap.
    pub gap_kind: NoteGapKind,
    /// Routable message id naming the cause.
    pub cause_message_id: String,
}

/// A consumer surface bound to the notes it reads, with its readiness, decision, and gaps derived from
/// those notes' rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseNoteConsumerRow {
    /// The consumer surface.
    pub consumer: ReleaseNoteConsumer,
    /// Human-facing label.
    pub consumer_label: String,
    /// Accountable owner role.
    pub owner_role: String,
    /// The note ids this consumer reads.
    pub read_note_ids: Vec<String>,
    /// The union of change classes across the read notes.
    pub disclosed_change_classes: Vec<ChangeClass>,
    /// The union of artifact classes disclosed across the read notes.
    pub disclosed_artifact_classes: Vec<ArtifactClass>,
    /// The union of channels across the read notes.
    pub channels: Vec<ChannelScope>,
    /// The derived readiness.
    pub note_readiness: NoteReadiness,
    /// Coverage status.
    pub status: ConsumerStatus,
    /// Traffic-light signal.
    pub signal: DescriptorSignal,
    /// Gate decision.
    pub gate_decision: DescriptorGate,
    /// True when at least one read note requires action.
    pub requires_user_action: bool,
    /// Readiness gaps, one per (note, cause).
    pub gaps: Vec<NoteGap>,
    /// Routable status message id.
    pub status_message_id: String,
    /// Routable decision message id.
    pub decision_message_id: String,
}

impl ReleaseNoteConsumerRow {
    /// Builds a consumer row; the resolved unions, gaps, and verdict are recomputed against the packet's
    /// notes when the packet is assembled.
    pub fn new(consumer: ReleaseNoteConsumer, read_note_ids: &[String]) -> Self {
        Self {
            consumer,
            consumer_label: consumer.label().to_owned(),
            owner_role: consumer.owner_role().to_owned(),
            read_note_ids: read_note_ids.to_vec(),
            disclosed_change_classes: Vec::new(),
            disclosed_artifact_classes: Vec::new(),
            channels: Vec::new(),
            note_readiness: NoteReadiness::Informational,
            status: ConsumerStatus::Mapped,
            signal: DescriptorSignal::Green,
            gate_decision: DescriptorGate::Governed,
            requires_user_action: false,
            gaps: Vec::new(),
            status_message_id: format!(
                "{}consumer.{}.status",
                M5_RELEASE_NOTE_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
            decision_message_id: format!(
                "{}consumer.{}.decision",
                M5_RELEASE_NOTE_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
        }
    }

    /// Recomputes the resolved unions, gaps, and verdict from the packet's notes, so a consumer's
    /// readiness is always generated from the same checked-in rows rather than a hand-maintained status.
    pub fn recompute(&mut self, notes: &[ReleaseNoteEvidenceRow]) {
        let mut read = self.read_note_ids.clone();
        read.sort();
        read.dedup();
        self.read_note_ids = read.clone();

        let note_for = |id: &str| -> Option<&ReleaseNoteEvidenceRow> {
            notes.iter().find(|n| n.note_id == id)
        };

        let mut change_classes: Vec<ChangeClass> = Vec::new();
        let mut artifact_classes: Vec<ArtifactClass> = Vec::new();
        let mut channels: Vec<ChannelScope> = Vec::new();
        let mut gaps: Vec<NoteGap> = Vec::new();
        let consumer = self.consumer;
        for id in &read {
            match note_for(id) {
                None => {
                    gaps.push(make_gap(consumer, id, None, NoteGapKind::NoteNotPublished));
                }
                Some(note) => {
                    change_classes.push(note.change_class);
                    artifact_classes.extend(note.affected_artifact_classes.iter().copied());
                    channels.extend(note.channels.iter().copied());
                    if let Some(kind) = note.gap_kind() {
                        gaps.push(make_gap(consumer, id, Some(note.change_class), kind));
                    }
                }
            }
        }
        change_classes.sort_by_key(|c| change_class_rank(*c));
        change_classes.dedup();
        artifact_classes.sort_by_key(|c| artifact_rank(*c));
        artifact_classes.dedup();
        channels.sort_by_key(|c| channel_rank(*c));
        channels.dedup();
        gaps.sort_by(|a, b| {
            a.note_id
                .cmp(&b.note_id)
                .then(a.gap_kind.as_str().cmp(b.gap_kind.as_str()))
        });

        self.disclosed_change_classes = change_classes;
        self.disclosed_artifact_classes = artifact_classes;
        self.channels = channels;
        self.gaps = gaps;

        let mut gate = DescriptorGate::Governed;
        for gap in &self.gaps {
            gate = worst_gate(gate, gap.gap_kind.gate());
        }
        self.gate_decision = gate;
        self.note_readiness = NoteReadiness::from_gate(gate);
        self.status = status_for_gate(gate);
        self.signal = signal_for_gate(gate);
        self.requires_user_action = gate == DescriptorGate::Blocked;
    }

    /// True when the consumer reads every note as informational.
    pub fn is_informational(&self) -> bool {
        self.gate_decision == DescriptorGate::Governed
    }

    /// True when at least one read note recommends action.
    pub fn is_action_recommended(&self) -> bool {
        self.gate_decision == DescriptorGate::Narrowed
    }

    /// True when at least one read note requires action.
    pub fn is_action_required(&self) -> bool {
        self.gate_decision == DescriptorGate::Blocked
    }
}

fn make_gap(
    consumer: ReleaseNoteConsumer,
    note_id: &str,
    change_class: Option<ChangeClass>,
    kind: NoteGapKind,
) -> NoteGap {
    NoteGap {
        consumer,
        note_id: note_id.to_owned(),
        change_class,
        gap_kind: kind,
        cause_message_id: format!(
            "{}consumer.{}.{}.{}.gap",
            M5_RELEASE_NOTE_MESSAGE_ID_PREFIX,
            consumer.as_str(),
            note_id,
            kind.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Aggregate sub-objects
// ---------------------------------------------------------------------------

/// The release the evidence rows describe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseNoteTarget {
    /// The channels the release publishes to.
    pub channels: Vec<ChannelScope>,
    /// The deployment profiles the release covers.
    pub profiles: Vec<DeploymentProfile>,
    /// The currently installed version.
    pub current_version: String,
    /// The version the release moves to.
    pub target_version: String,
}

/// Disclosure flags asserting every claimed consumer ingests this one evidence set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseNoteDisclosure {
    /// The update center consumes the evidence set.
    pub update_center_consumes_notes: bool,
    /// The what's-new panel consumes the evidence set.
    pub whats_new_panel_consumes_notes: bool,
    /// The Help center consumes the evidence set.
    pub help_center_consumes_notes: bool,
    /// The docs/Help release notes consume the evidence set.
    pub docs_help_consumes_notes: bool,
    /// The release center consumes the evidence set.
    pub release_center_consumes_notes: bool,
    /// The support export consumes the evidence set.
    pub support_export_consumes_notes: bool,
}

impl ReleaseNoteDisclosure {
    fn canonical() -> Self {
        Self {
            update_center_consumes_notes: true,
            whats_new_panel_consumes_notes: true,
            help_center_consumes_notes: true,
            docs_help_consumes_notes: true,
            release_center_consumes_notes: true,
            support_export_consumes_notes: true,
        }
    }

    /// True when every consumer is asserted to consume the evidence set.
    pub fn all_consume(&self) -> bool {
        self.update_center_consumes_notes
            && self.whats_new_panel_consumes_notes
            && self.help_center_consumes_notes
            && self.docs_help_consumes_notes
            && self.release_center_consumes_notes
            && self.support_export_consumes_notes
    }
}

/// Roll-up counts over the notes and consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseNoteCounts {
    /// Total notes.
    pub total_notes: u32,
    /// Informational notes (governed).
    pub informational_notes: u32,
    /// Action-recommended notes (narrowed).
    pub action_recommended_notes: u32,
    /// Action-required notes (blocked).
    pub action_required_notes: u32,
    /// Security notes.
    pub security_notes: u32,
    /// Breaking notes.
    pub breaking_notes: u32,
    /// Migration-required notes.
    pub migration_notes: u32,
    /// Deprecated notes.
    pub deprecated_notes: u32,
    /// Docs-only notes.
    pub docs_only_notes: u32,
    /// Notes carrying substantive evidence.
    pub evidence_backed_notes: u32,
    /// Notes carrying a direct-action link.
    pub direct_action_linked_notes: u32,
    /// Notes whose what's-new card is dismissed.
    pub dismissed_notes: u32,
    /// Notes whose what's-new card is reopenable.
    pub reopenable_notes: u32,
    /// Total consumers.
    pub total_consumers: u32,
    /// Consumers that read every note as informational.
    pub informational_consumers: u32,
    /// Consumers recommending action.
    pub action_recommended_consumers: u32,
    /// Consumers requiring action.
    pub action_required_consumers: u32,
    /// Whether any note requires user action.
    pub requires_user_action: bool,
}

/// The evidence-completeness honesty block: how many notes carry the required links, so a release can
/// disclose whether every behavior-changing note is evidence-backed and every breaking / migration note
/// links directly to an action surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceCoverage {
    /// Notes with at least one substantive evidence link.
    pub notes_with_substantive_evidence: u32,
    /// Notes with at least one direct-action link.
    pub notes_with_direct_action_link: u32,
    /// Notes whose change class requires a direct-action link.
    pub notes_requiring_direct_action_link: u32,
    /// Notes that are behavior-changing or security-sensitive (must be evidence-backed).
    pub notes_requiring_substantive_evidence: u32,
    /// True when every note that needs substantive evidence has it and every note that needs a
    /// direct-action link has it.
    pub all_required_links_present: bool,
    /// True when every what's-new card is dismissible and reopenable from the update center and Help.
    pub all_cards_reopenable: bool,
    /// True when every what's-new card is non-blocking.
    pub all_cards_non_blocking: bool,
}

/// The packet-level action gate aggregating the per-consumer decisions. This is the one place release /
/// shiproom tooling reads whether any published note asks the user to act — it never blocks a workflow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseNoteActionGate {
    /// Whether any consumer reads an action-required note.
    pub requires_user_action: bool,
    /// Tokens of the action-required consumers.
    pub action_required_consumers: Vec<String>,
    /// Tokens of the action-recommended consumers.
    pub action_recommended_consumers: Vec<String>,
    /// Tokens of the informational consumers.
    pub informational_consumers: Vec<String>,
    /// Ids of the notes that require action.
    pub action_required_notes: Vec<String>,
    /// Routable gate message id.
    pub gate_message_id: String,
}

/// The frozen controlled vocabulary the rows draw from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseNoteVocabulary {
    /// Change-class tokens.
    pub change_classes: Vec<String>,
    /// Evidence-link-kind tokens.
    pub evidence_link_kinds: Vec<String>,
    /// Note-readiness tokens.
    pub note_readiness: Vec<String>,
    /// Dismiss-state tokens.
    pub dismiss_states: Vec<String>,
    /// Reopen-surface tokens.
    pub reopen_surfaces: Vec<String>,
    /// Artifact-class tokens.
    pub artifact_classes: Vec<String>,
    /// Profile tokens.
    pub profiles: Vec<String>,
    /// Channel tokens.
    pub channels: Vec<String>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
    /// Gap-kind tokens.
    pub gap_kinds: Vec<String>,
    /// Gate-decision tokens.
    pub gate_decisions: Vec<String>,
}

impl ReleaseNoteVocabulary {
    /// The canonical frozen vocabulary.
    pub fn canonical() -> Self {
        Self {
            change_classes: tokens(&ChangeClass::ALL, |x| x.as_str()),
            evidence_link_kinds: tokens(&EvidenceLinkKind::ALL, |x| x.as_str()),
            note_readiness: tokens(&NoteReadiness::ALL, |x| x.as_str()),
            dismiss_states: tokens(&WhatsNewDismissState::ALL, |x| x.as_str()),
            reopen_surfaces: tokens(&ReopenSurface::ALL, |x| x.as_str()),
            artifact_classes: tokens(&ArtifactClass::ALL, |x| x.as_str()),
            profiles: tokens(&DeploymentProfile::ALL, |x| x.as_str()),
            channels: tokens(&ChannelScope::ALL, |x| x.as_str()),
            consumers: tokens(&ReleaseNoteConsumer::ALL, |x| x.as_str()),
            gap_kinds: tokens(&NoteGapKind::ALL, |x| x.as_str()),
            gate_decisions: tokens(&DescriptorGate::ALL, |x| x.as_str()),
        }
    }

    /// True when this vocabulary equals the canonical frozen vocabulary.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Conformance flags every canonical evidence set asserts. They restate the acceptance bar so a tampered
/// packet that flips one to false fails [`ReleaseNoteEvidenceSet::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseNoteConformance {
    /// Every note carries a controlled change class.
    pub every_note_carries_change_class: bool,
    /// What's-new and release-note surfaces use one change-class vocabulary.
    pub change_classes_use_one_vocabulary: bool,
    /// Behavior-changing or security-sensitive notes are evidence-backed, not prose alone.
    pub behavior_or_security_notes_evidence_backed: bool,
    /// Breaking or migration-relevant notes link directly to a setting / import / rollback surface.
    pub breaking_or_migration_notes_link_directly: bool,
    /// Security notes link to an advisory.
    pub security_notes_link_to_advisory: bool,
    /// What's-new cards are dismissible and reopenable.
    pub whats_new_cards_dismissible_and_reopenable: bool,
    /// What's-new cards never block typing, save, restore, or recovery-critical workflows.
    pub whats_new_cards_never_block_workflows: bool,
    /// Dismissed cards are reopenable from the update center and Help.
    pub reopenable_from_update_center_and_help: bool,
    /// One evidence-row schema across app, docs/Help, and exported summaries.
    pub one_schema_across_app_docs_and_exports: bool,
    /// Every claimed consumer reads this one evidence set.
    pub consumers_read_one_note_set: bool,
    /// Every consumer verdict is derived from the notes, not hand-maintained.
    pub consumer_verdict_derived_from_notes: bool,
    /// The controlled enums are frozen.
    pub controlled_enums_frozen: bool,
    /// The export carries metadata and refs only — no credential bodies or raw payloads.
    pub export_carries_no_raw_material: bool,
    /// Marketing prose is separated from evidence; no roadmap overload on support-sensitive surfaces.
    pub marketing_separated_from_evidence: bool,
}

impl ReleaseNoteConformance {
    fn canonical() -> Self {
        Self {
            every_note_carries_change_class: true,
            change_classes_use_one_vocabulary: true,
            behavior_or_security_notes_evidence_backed: true,
            breaking_or_migration_notes_link_directly: true,
            security_notes_link_to_advisory: true,
            whats_new_cards_dismissible_and_reopenable: true,
            whats_new_cards_never_block_workflows: true,
            reopenable_from_update_center_and_help: true,
            one_schema_across_app_docs_and_exports: true,
            consumers_read_one_note_set: true,
            consumer_verdict_derived_from_notes: true,
            controlled_enums_frozen: true,
            export_carries_no_raw_material: true,
            marketing_separated_from_evidence: true,
        }
    }

    /// True when every conformance flag holds.
    pub fn all_hold(&self) -> bool {
        *self == Self::canonical()
    }
}

// ---------------------------------------------------------------------------
// Render channel
// ---------------------------------------------------------------------------

/// The render channels the packet must serialize identically across.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseNoteChannel {
    /// The desktop update center / what's-new panel.
    DesktopUi,
    /// The CLI / headless emitter.
    CliHeadless,
    /// The published docs/Help release notes.
    DocsHelp,
    /// The offline / exported summary.
    OfflineExport,
}

// ---------------------------------------------------------------------------
// Validation violations
// ---------------------------------------------------------------------------

/// A reason an evidence set failed [`ReleaseNoteEvidenceSet::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseNoteViolation {
    /// The record kind or schema version is wrong.
    HeaderDrift,
    /// Two notes share a note id.
    DuplicateNoteId,
    /// A note's derived gate / readiness / signal / scope / link summary drifted.
    NoteDerivationDrift,
    /// A behavior-changing or security-sensitive note lacks substantive evidence — prose alone.
    MissingEvidenceLink,
    /// A breaking or migration-relevant note lacks a direct-action link.
    MissingDirectActionLink,
    /// A security note lacks an advisory link.
    SecurityNoteMissingAdvisory,
    /// A what's-new card blocks a workflow — the lane's guardrail.
    WhatsNewCardBlocksWorkflow,
    /// A what's-new card is not dismissible / reopenable from the update center and Help.
    WhatsNewCardNotReopenable,
    /// A consumer's derived verdict, unions, or gaps drifted.
    ConsumerVerdictDrift,
    /// The summary counts, coverage, or action gate drifted.
    SummaryDrift,
    /// The disclosure flags do not all assert consumption of the one evidence set.
    DisclosureDrift,
    /// The controlled vocabulary drifted.
    VocabularyDrift,
    /// A conformance flag does not hold.
    ConformanceDrift,
    /// The export carried forbidden raw material.
    ForbiddenMaterial,
}

impl ReleaseNoteViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HeaderDrift => "header_drift",
            Self::DuplicateNoteId => "duplicate_note_id",
            Self::NoteDerivationDrift => "note_derivation_drift",
            Self::MissingEvidenceLink => "missing_evidence_link",
            Self::MissingDirectActionLink => "missing_direct_action_link",
            Self::SecurityNoteMissingAdvisory => "security_note_missing_advisory",
            Self::WhatsNewCardBlocksWorkflow => "whats_new_card_blocks_workflow",
            Self::WhatsNewCardNotReopenable => "whats_new_card_not_reopenable",
            Self::ConsumerVerdictDrift => "consumer_verdict_drift",
            Self::SummaryDrift => "summary_drift",
            Self::DisclosureDrift => "disclosure_drift",
            Self::VocabularyDrift => "vocabulary_drift",
            Self::ConformanceDrift => "conformance_drift",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

// ---------------------------------------------------------------------------
// Packet
// ---------------------------------------------------------------------------

/// Builder input for [`ReleaseNoteEvidenceSet::new`].
#[derive(Debug, Clone)]
pub struct ReleaseNoteEvidenceSetInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-facing report label.
    pub report_label: String,
    /// Evaluation timestamp.
    pub evaluated_at: String,
    /// The release the notes describe.
    pub target: ReleaseNoteTarget,
    /// The per-note evidence rows.
    pub notes: Vec<ReleaseNoteEvidenceRow>,
    /// The claimed consumer rows.
    pub consumers: Vec<ReleaseNoteConsumerRow>,
    /// Redaction-class token.
    pub redaction_class_token: String,
    /// Mint timestamp.
    pub minted_at: String,
}

/// The one inspectable, serde-serializable release-note evidence set the update center, what's-new
/// panel, Help center, docs/Help, release center, and support export consume under one vocabulary and
/// one schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseNoteEvidenceSet {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-facing report label.
    pub report_label: String,
    /// Evaluation timestamp.
    pub evaluated_at: String,
    /// The release the notes describe.
    pub target: ReleaseNoteTarget,
    /// The per-note evidence rows.
    pub notes: Vec<ReleaseNoteEvidenceRow>,
    /// The note ids this packet covers.
    pub note_ids: Vec<String>,
    /// The consumer rows reading the notes.
    pub consumers: Vec<ReleaseNoteConsumerRow>,
    /// The consumer tokens, in canonical order.
    pub consumer_tokens: Vec<String>,
    /// Disclosure flags.
    pub disclosure: ReleaseNoteDisclosure,
    /// Roll-up counts.
    pub summary: ReleaseNoteCounts,
    /// Evidence-completeness honesty block.
    pub coverage: EvidenceCoverage,
    /// Packet-level action gate.
    pub action_gate: ReleaseNoteActionGate,
    /// Controlled vocabulary.
    pub vocabulary: ReleaseNoteVocabulary,
    /// Conformance flags.
    pub conformance: ReleaseNoteConformance,
    /// Redaction-class token.
    pub redaction_class_token: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl ReleaseNoteEvidenceSet {
    /// Builds a packet from the given notes and consumer rows, recomputing every derived field so the
    /// published packet is always generated from the same checked-in rows.
    pub fn new(input: ReleaseNoteEvidenceSetInput) -> Self {
        let mut notes = input.notes;
        for note in &mut notes {
            note.recompute();
        }
        notes.sort_by(|a, b| {
            change_class_rank(b.change_class)
                .cmp(&change_class_rank(a.change_class))
                .then(a.note_id.cmp(&b.note_id))
        });

        let mut consumers = input.consumers;
        for consumer in &mut consumers {
            consumer.recompute(&notes);
        }
        consumers.sort_by_key(|c| consumer_rank(c.consumer));

        let mut target = input.target;
        target.channels.sort_by_key(|c| channel_rank(*c));
        target.channels.dedup();
        target.profiles.sort_by_key(|p| profile_rank(*p));
        target.profiles.dedup();

        let mut note_ids: Vec<String> = notes.iter().map(|n| n.note_id.clone()).collect();
        note_ids.sort();
        note_ids.dedup();

        let summary = derive_counts(&notes, &consumers);
        let coverage = derive_coverage(&notes);
        let action_gate = derive_action_gate(&notes, &consumers);

        Self {
            record_kind: M5_RELEASE_NOTE_EVIDENCE_SET_RECORD_KIND.to_owned(),
            schema_version: M5_RELEASE_NOTE_EVIDENCE_SET_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            target,
            notes,
            note_ids,
            consumer_tokens: tokens(&ReleaseNoteConsumer::ALL, |x| x.as_str()),
            consumers,
            disclosure: ReleaseNoteDisclosure::canonical(),
            summary,
            coverage,
            action_gate,
            vocabulary: ReleaseNoteVocabulary::canonical(),
            conformance: ReleaseNoteConformance::canonical(),
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Looks up a note by id.
    pub fn note(&self, note_id: &str) -> Option<&ReleaseNoteEvidenceRow> {
        self.notes.iter().find(|n| n.note_id == note_id)
    }

    /// Looks up the consumer row for a consumer.
    pub fn consumer(&self, consumer: ReleaseNoteConsumer) -> Option<&ReleaseNoteConsumerRow> {
        self.consumers.iter().find(|c| c.consumer == consumer)
    }

    /// Whether any published note asks the user to take an action.
    pub fn requires_user_action(&self) -> bool {
        self.action_gate.requires_user_action
    }

    /// Validates every derived field by recomputing it from the notes and comparing, plus the lane's
    /// evidence / direct-link / reopenability guardrails. Returns an empty vector when the packet is
    /// internally consistent.
    pub fn validate(&self) -> Vec<ReleaseNoteViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_RELEASE_NOTE_EVIDENCE_SET_RECORD_KIND
            || self.schema_version != M5_RELEASE_NOTE_EVIDENCE_SET_SCHEMA_VERSION
        {
            violations.push(ReleaseNoteViolation::HeaderDrift);
        }

        // Note ids are unique.
        for (i, note) in self.notes.iter().enumerate() {
            if self.notes[i + 1..]
                .iter()
                .any(|n| n.note_id == note.note_id)
            {
                violations.push(ReleaseNoteViolation::DuplicateNoteId);
                break;
            }
        }

        for note in &self.notes {
            // Recompute the row from its inputs and compare the derived verdict.
            let mut fresh = note.clone();
            fresh.recompute();
            if fresh.gate != note.gate
                || fresh.note_readiness != note.note_readiness
                || fresh.status != note.status
                || fresh.signal != note.signal
                || fresh.support_sensitive != note.support_sensitive
                || fresh.has_substantive_evidence != note.has_substantive_evidence
                || fresh.has_direct_action_link != note.has_direct_action_link
                || fresh.requires_user_action != note.requires_user_action
                || fresh.affected_artifact_classes != note.affected_artifact_classes
                || fresh.affected_profiles != note.affected_profiles
                || fresh.channels != note.channels
                || fresh.evidence_links != note.evidence_links
            {
                violations.push(ReleaseNoteViolation::NoteDerivationDrift);
            }

            // Guardrail: a behavior-changing or security-sensitive note must be evidence-backed.
            if note
                .change_class
                .is_behavior_changing_or_security_sensitive()
                && !note.has_substantive_evidence
            {
                violations.push(ReleaseNoteViolation::MissingEvidenceLink);
            }
            // Guardrail: a breaking / migration / admin note must link directly to an action surface.
            if note.change_class.requires_direct_action_link() && !note.has_direct_action_link {
                violations.push(ReleaseNoteViolation::MissingDirectActionLink);
            }
            // Guardrail: a security note must link to an advisory.
            if note.change_class == ChangeClass::Security && !note.has_security_advisory() {
                violations.push(ReleaseNoteViolation::SecurityNoteMissingAdvisory);
            }
            // Guardrail: a what's-new card never blocks a workflow.
            if !note.whats_new_card.is_non_blocking() {
                violations.push(ReleaseNoteViolation::WhatsNewCardBlocksWorkflow);
            }
            // Guardrail: a what's-new card is dismissible and reopenable from the update center and Help.
            if !note.whats_new_card.is_reopenable_everywhere() {
                violations.push(ReleaseNoteViolation::WhatsNewCardNotReopenable);
            }
        }

        // Consumers: recompute and compare verdict, unions, and gaps.
        for consumer in &self.consumers {
            let mut fresh = ReleaseNoteConsumerRow::new(consumer.consumer, &consumer.read_note_ids);
            fresh.recompute(&self.notes);
            if fresh.gate_decision != consumer.gate_decision
                || fresh.note_readiness != consumer.note_readiness
                || fresh.status != consumer.status
                || fresh.signal != consumer.signal
                || fresh.requires_user_action != consumer.requires_user_action
                || fresh.disclosed_change_classes != consumer.disclosed_change_classes
                || fresh.disclosed_artifact_classes != consumer.disclosed_artifact_classes
                || fresh.channels != consumer.channels
                || fresh.gaps != consumer.gaps
            {
                violations.push(ReleaseNoteViolation::ConsumerVerdictDrift);
                break;
            }
        }

        if self.summary != derive_counts(&self.notes, &self.consumers)
            || self.coverage != derive_coverage(&self.notes)
            || self.action_gate != derive_action_gate(&self.notes, &self.consumers)
        {
            violations.push(ReleaseNoteViolation::SummaryDrift);
        }

        if !self.disclosure.all_consume()
            || self.consumer_tokens != tokens(&ReleaseNoteConsumer::ALL, |x| x.as_str())
        {
            violations.push(ReleaseNoteViolation::DisclosureDrift);
        }

        if !self.vocabulary.matches_canonical() {
            violations.push(ReleaseNoteViolation::VocabularyDrift);
        }

        if !self.conformance.all_hold() {
            violations.push(ReleaseNoteViolation::ConformanceDrift);
        }

        if contains_forbidden_material(self) {
            violations.push(ReleaseNoteViolation::ForbiddenMaterial);
        }

        violations
    }

    /// The canonical export form: pretty JSON, identical across every render channel.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("evidence set serializes")
    }

    /// Renders the packet for a channel. Every channel produces byte-identical output.
    pub fn render_for_channel(&self, _channel: ReleaseNoteChannel) -> String {
        self.export_safe_json()
    }

    /// A compact Markdown summary of the notes and consumer verdicts, for export and review outside the
    /// app.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# {}\n\n", self.report_label));
        out.push_str(&format!(
            "Release `{}` → `{}` — {} notes ({} action-required, {} action-recommended), {} consumers.\n\n",
            self.target.current_version,
            self.target.target_version,
            self.summary.total_notes,
            self.summary.action_required_notes,
            self.summary.action_recommended_notes,
            self.summary.total_consumers,
        ));
        if !self.coverage.all_required_links_present {
            out.push_str(
                "> Evidence incomplete: a behavior-changing / breaking / migration note is missing a required link.\n\n",
            );
        }
        out.push_str("## Release notes\n\n");
        out.push_str(
            "| Note | Change class | Readiness | Evidence | Direct link | What's-new | Scope |\n",
        );
        out.push_str("|---|---|---|---|---|---|---|\n");
        for n in &self.notes {
            let scope: Vec<&str> = n
                .affected_artifact_classes
                .iter()
                .map(|x| x.as_str())
                .collect();
            let links: Vec<&str> = n.evidence_links.iter().map(|l| l.kind.as_str()).collect();
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | {} | {} | `{}` | {} |\n",
                n.note_id,
                n.change_class.as_str(),
                n.note_readiness.as_str(),
                if n.has_substantive_evidence {
                    "yes"
                } else {
                    "no"
                },
                if n.has_direct_action_link {
                    links.join(", ")
                } else {
                    String::new()
                },
                n.whats_new_card.dismiss_state.as_str(),
                scope.join(", "),
            ));
        }
        out.push_str("\n## Consumers\n\n");
        for c in &self.consumers {
            out.push_str(&format!(
                "- `{}` → `{}` ({}",
                c.consumer.as_str(),
                c.note_readiness.as_str(),
                c.gate_decision.as_str(),
            ));
            if c.gaps.is_empty() {
                out.push_str(")\n");
            } else {
                let gaps: Vec<String> = c
                    .gaps
                    .iter()
                    .map(|g| format!("{}:{}", g.note_id, g.gap_kind.as_str()))
                    .collect();
                out.push_str(&format!("; gap: {})\n", gaps.join(", ")));
            }
        }
        out
    }

    /// A machine-readable CSV of every release-note evidence row, for export and review outside the app.
    pub fn render_note_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "note_id,change_class,note_readiness,support_sensitive,has_substantive_evidence,has_direct_action_link,dismiss_state,reopenable,from_version,to_version,gate\n",
        );
        for n in &self.notes {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                n.note_id,
                n.change_class.as_str(),
                n.note_readiness.as_str(),
                n.support_sensitive,
                n.has_substantive_evidence,
                n.has_direct_action_link,
                n.whats_new_card.dismiss_state.as_str(),
                n.whats_new_card.is_reopenable_everywhere(),
                n.from_version.as_deref().unwrap_or(""),
                n.to_version.as_deref().unwrap_or(""),
                n.gate.as_str(),
            ));
        }
        out
    }
}

fn derive_counts(
    notes: &[ReleaseNoteEvidenceRow],
    consumers: &[ReleaseNoteConsumerRow],
) -> ReleaseNoteCounts {
    let count_class =
        |class: ChangeClass| notes.iter().filter(|n| n.change_class == class).count() as u32;
    ReleaseNoteCounts {
        total_notes: notes.len() as u32,
        informational_notes: notes
            .iter()
            .filter(|n| n.gate == DescriptorGate::Governed)
            .count() as u32,
        action_recommended_notes: notes
            .iter()
            .filter(|n| n.gate == DescriptorGate::Narrowed)
            .count() as u32,
        action_required_notes: notes
            .iter()
            .filter(|n| n.gate == DescriptorGate::Blocked)
            .count() as u32,
        security_notes: count_class(ChangeClass::Security),
        breaking_notes: count_class(ChangeClass::Breaking),
        migration_notes: count_class(ChangeClass::MigrationRequired),
        deprecated_notes: count_class(ChangeClass::Deprecated),
        docs_only_notes: count_class(ChangeClass::DocsOnly),
        evidence_backed_notes: notes.iter().filter(|n| n.has_substantive_evidence).count() as u32,
        direct_action_linked_notes: notes.iter().filter(|n| n.has_direct_action_link).count()
            as u32,
        dismissed_notes: notes
            .iter()
            .filter(|n| n.whats_new_card.dismiss_state == WhatsNewDismissState::Dismissed)
            .count() as u32,
        reopenable_notes: notes
            .iter()
            .filter(|n| n.whats_new_card.is_reopenable_everywhere())
            .count() as u32,
        total_consumers: consumers.len() as u32,
        informational_consumers: consumers.iter().filter(|c| c.is_informational()).count() as u32,
        action_recommended_consumers: consumers
            .iter()
            .filter(|c| c.is_action_recommended())
            .count() as u32,
        action_required_consumers: consumers.iter().filter(|c| c.is_action_required()).count()
            as u32,
        requires_user_action: notes.iter().any(|n| n.requires_user_action),
    }
}

fn derive_coverage(notes: &[ReleaseNoteEvidenceRow]) -> EvidenceCoverage {
    let requiring_evidence: Vec<&ReleaseNoteEvidenceRow> = notes
        .iter()
        .filter(|n| n.change_class.is_behavior_changing_or_security_sensitive())
        .collect();
    let requiring_direct: Vec<&ReleaseNoteEvidenceRow> = notes
        .iter()
        .filter(|n| n.change_class.requires_direct_action_link())
        .collect();
    let all_required_links_present = requiring_evidence
        .iter()
        .all(|n| n.has_substantive_evidence)
        && requiring_direct.iter().all(|n| n.has_direct_action_link)
        && notes
            .iter()
            .filter(|n| n.change_class == ChangeClass::Security)
            .all(|n| n.has_security_advisory());
    EvidenceCoverage {
        notes_with_substantive_evidence: notes.iter().filter(|n| n.has_substantive_evidence).count()
            as u32,
        notes_with_direct_action_link: notes.iter().filter(|n| n.has_direct_action_link).count()
            as u32,
        notes_requiring_direct_action_link: requiring_direct.len() as u32,
        notes_requiring_substantive_evidence: requiring_evidence.len() as u32,
        all_required_links_present,
        all_cards_reopenable: notes
            .iter()
            .all(|n| n.whats_new_card.is_reopenable_everywhere()),
        all_cards_non_blocking: notes.iter().all(|n| n.whats_new_card.is_non_blocking()),
    }
}

fn derive_action_gate(
    notes: &[ReleaseNoteEvidenceRow],
    consumers: &[ReleaseNoteConsumerRow],
) -> ReleaseNoteActionGate {
    let collect = |pred: fn(&ReleaseNoteConsumerRow) -> bool| -> Vec<String> {
        consumers
            .iter()
            .filter(|c| pred(c))
            .map(|c| c.consumer.as_str().to_owned())
            .collect()
    };
    let mut action_notes: Vec<String> = notes
        .iter()
        .filter(|n| n.requires_user_action)
        .map(|n| n.note_id.clone())
        .collect();
    action_notes.sort();
    let required = collect(ReleaseNoteConsumerRow::is_action_required);
    ReleaseNoteActionGate {
        requires_user_action: !required.is_empty(),
        action_required_consumers: required,
        action_recommended_consumers: collect(ReleaseNoteConsumerRow::is_action_recommended),
        informational_consumers: collect(ReleaseNoteConsumerRow::is_informational),
        action_required_notes: action_notes,
        gate_message_id: format!("{}action_gate", M5_RELEASE_NOTE_MESSAGE_ID_PREFIX),
    }
}

/// Scans the export for forbidden raw material (credential bodies / raw provider payloads).
fn contains_forbidden_material(packet: &ReleaseNoteEvidenceSet) -> bool {
    let json = serde_json::to_string(packet)
        .unwrap_or_default()
        .to_ascii_lowercase();
    const FORBIDDEN: [&str; 6] = [
        "bearer_token",
        "authorization:",
        "private_key",
        "begin rsa",
        "set-cookie",
        "client_secret",
    ];
    FORBIDDEN.iter().any(|needle| json.contains(needle))
}

/// Maps each variant of an `as_str`-bearing enum to its token, in declaration order.
fn tokens<T: Copy, const N: usize>(all: &[T; N], f: impl Fn(&T) -> &'static str) -> Vec<String> {
    all.iter().map(|x| f(x).to_owned()).collect()
}
