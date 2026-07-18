//! Shared shared-terminal / debug-view, collaboration-join-review, control-grant-prompt, presenter-handoff,
//! paste / secret guard, collaboration-retention, session-restore, and support / export consumers that keep the
//! B155 collaboration-control objects — the shared terminal / debug view, the control grant, the presenter
//! token, the consent envelope, the retention review, and the session-restore view — at **one canonical
//! vocabulary** across every claimed M5 desktop collaboration, browser / mobile companion, incident / support,
//! help / docs, and audit-export surface, and blocks any deferred-intent or outbox system from silently
//! queueing a control grant, presenter handoff, or terminal input across a reconnect or offline boundary.
//!
//! This module is the consumer-adoption capstone for the six governed collaboration-control object classes
//! frozen in
//! [`crate::m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix`]
//! and implemented by the session-policy-manifest / join-review lane
//! ([`crate::m5_session_policy_manifest_and_join_review_sheet_registries`]) and the five sibling
//! shared-terminal-debug-view, control-grant, paste / secret guard, retention-review, and session-restore-view
//! implement lanes that consume the same matrix.
//!
//! It binds each shared collaboration-control object to the concrete desktop-collaboration, browser / mobile
//! companion, incident / support, audit-export, and help / docs consumers — projected here through the
//! shared-terminal-debug-view, collaboration-join-review-sheet, control-grant-prompt, presenter-handoff-sheet,
//! paste / secret guard, collaboration-retention-sheet, session-restore-view, support-export, and help / docs
//! surfaces — that render it, and proves — by fixtures, not screenshots — that the same seeded collaboration
//! session presents the same collaboration-control-role, object, registry-reference, session-state,
//! surface-context, and authority-source vocabulary wherever it appears.
//!
//! The core honesty axes are four, mirroring the batch acceptance criteria.
//!
//! 1. **Reuse.** Each of the six shared collaboration-control objects must be adopted by at least two distinct
//!    consumers, so an object is proven to be shared collaboration-control infrastructure rather than a
//!    one-surface, feature-local fork of the shared terminal / debug view, control grant, presenter token,
//!    consent envelope, retention review, or session-restore view.
//! 2. **One vocabulary / no drift.** For a given seeded collaboration session every consumer surface must
//!    present identical [`CollaborationControlSharedStateFacetValues`] — the same collaboration-control-role
//!    word, the same object word, the same registry-reference word, the same session-state word, the same
//!    surface-context word, and the same authority-source word. The collaboration-control-role word must be a
//!    token from the frozen [`M5CollaborationControlRole`] vocabulary, so no surface rewrites
//!    `control_authority_disclosure`, `active_driver_disclosure`, `view_first_default_disclosure`,
//!    `consent_scope_disclosure`, `recording_retention_state_disclosure`, `paste_secret_guard_disclosure`, or
//!    `replay_free_restore_disclosure` in its own words. A surface may narrow *how much* it shows across
//!    desktop, compact, remote, and exported representations, but it may never reword the underlying vocabulary
//!    per surface, and no surface may acquire terminal / debug control from presence or follow without an
//!    explicit grant, allow more than one active driver on a sensitive surface, start recording / retention /
//!    guest-scope widening silently, replay prior terminal / debug input on join or restore, or reveal raw
//!    secrets, command text, variable bodies, or clipboard contents without a guard.
//! 3. **Map back to one object.** Support / export consumers must point at the canonical per-domain schema and
//!    the frozen matrix by id, so an exported packet — and every copy / export / open-in-provider action — can
//!    always map a desktop / companion / support surface back to one shared contract object rather than
//!    diverging into a surface-local payload or collapsing stable authority / session labels to generic prose.
//! 4. **No silent queued grants.** Deferred-intent and outbox systems can never queue a control grant, a
//!    presenter handoff, terminal input, or any other sensitive collaboration-control action across a reconnect
//!    or offline boundary; a refused control action explains why it was refused and demands a fresh live review
//!    rather than replaying later as if it were an idempotent background write.
//!
//! Narrowing is disclosed, never hidden: a compact, remote, or exported representation carries an explicit
//! [`CollaborationControlSharedNarrowNote`] naming the reason, the preserved vocabulary, and the next action,
//! and an exported representation additionally names its export-safe detail boundary rather than collapsing the
//! subject out of view.
//!
//! The packet references upstream collaboration-control contracts by id rather than embedding their content. Raw
//! secret values, command text, variable bodies, and clipboard contents stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/collaboration/m5-collaboration-control-shared-consumers.schema.json`](../../../../schemas/collaboration/m5-collaboration-control-shared-consumers.schema.json).
//! The contract doc is
//! [`docs/collaboration/m5_collaboration_control_shared_consumers_one_vocabulary.md`](../../../../docs/collaboration/m5_collaboration_control_shared_consumers_one_vocabulary.md).
//! The protected fixture directory is
//! [`fixtures/collaboration/m5-collaboration-control-shared-consumers/`](../../../../fixtures/collaboration/m5-collaboration-control-shared-consumers/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_collaboration_control_shared_consumers,
    seeded_m5_collaboration_control_shared_consumers_compact_remote_narrowed,
    seeded_m5_collaboration_control_shared_consumers_exported_redaction_narrowed,
};

use crate::m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix::{
    M5CollaborationControlConsumerSurface, M5CollaborationControlObject, M5CollaborationControlRole,
    M5_COLLABORATION_CONTROL_MATRIX_DOC_REF, M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5CollaborationControlSharedConsumersPacket`].
pub const M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_RECORD_KIND: &str =
    "m5_collaboration_control_shared_consumer_registry_parity";

/// Schema version for collaboration-control shared-consumer parity records.
pub const M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_PACKET_ID: &str =
    "m5-collaboration-control-shared-consumers:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_SCHEMA_REF: &str =
    "schemas/collaboration/m5-collaboration-control-shared-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_DOC_REF: &str =
    "docs/collaboration/m5_collaboration_control_shared_consumers_one_vocabulary.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/release/m5-collaboration-control-shared-consumers-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_CSV_REF: &str =
    "artifacts/release/m5-collaboration-control-shared-consumers-proof/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_REPORT_REF: &str =
    "artifacts/release/m5-collaboration-control-shared-consumers-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/collaboration/m5-collaboration-control-shared-consumers";

/// Proof-freshness SLO in hours for this lane.
pub const M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_PROOF_SLO_HOURS: u32 = 720;

/// Authority-source sentinel words a control-authority / active-driver / view-first-default /
/// consent-scope gate role may never fall back to; a gate-carrying role that changes surface presentation
/// must always keep a real authority-source-disclosed-and-control-grant-bound continuity, never acquiring
/// terminal / debug control from presence alone, showing presence as control authority, showing a second
/// active driver on a sensitive surface, or letting prior input replay read as live control.
const AUTHORITY_SOURCE_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "control_acquired_from_presence_alone",
    "presence_shown_as_control_authority",
    "second_active_driver_shown_on_a_sensitive_surface",
    "prior_input_replayed_as_live_control",
];

/// Whether a consumer surface is an export / support path that must map an object back to its canonical
/// contract by id.
pub const fn consumer_must_reference_canonical(
    consumer: M5CollaborationControlConsumerSurface,
) -> bool {
    matches!(
        consumer,
        M5CollaborationControlConsumerSurface::SupportExportPacket
    )
}

/// Whether `token` is a member of the frozen [`M5CollaborationControlRole`] vocabulary.
///
/// This is the "one vocabulary" gate: a seeded subject's collaboration-control-role word must be a controlled role token
/// rather than a per-surface synonym.
pub fn is_known_collaboration_control_role_token(token: &str) -> bool {
    collaboration_control_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5CollaborationControlRole`], if it is one.
pub fn collaboration_control_role_from_token(token: &str) -> Option<M5CollaborationControlRole> {
    M5CollaborationControlRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// How much of a shared collaboration-control object a consumer renders for one representation.
///
/// Narrowing changes how much is shown, never the underlying vocabulary: a narrowed representation still
/// carries the same collaboration-control-role, object, registry-reference, session-state, surface-context, and
/// authority-source words, and discloses the narrowing through an explicit note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationControlSharedRepresentation {
    /// The full desktop representation; nothing is narrowed.
    DesktopFull,
    /// A compact representation that narrows disclosure depth.
    CompactNarrowed,
    /// A remote-projected representation backed by a remote source.
    RemoteProjected,
    /// An exported, export-safe-redacted representation.
    ExportedRedacted,
}

impl CollaborationControlSharedRepresentation {
    /// Every representation, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DesktopFull,
        Self::CompactNarrowed,
        Self::RemoteProjected,
        Self::ExportedRedacted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompactNarrowed => "compact_narrowed",
            Self::RemoteProjected => "remote_projected",
            Self::ExportedRedacted => "exported_redacted",
        }
    }

    /// Whether this representation narrows below full desktop disclosure.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }
}

/// A vocabulary axis whose word must stay identical across surfaces for one subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationControlSharedStateFacet {
    /// The frozen collaboration-control-role word.
    CollaborationControlRoleWord,
    /// The collaboration-control-object word.
    ObjectWord,
    /// The canonical registry-reference word the object points at.
    RegistryReferenceWord,
    /// The session-state word (viewer / driver / control-granted / recording-active / restore-view-only, etc.)
    /// the subject ships.
    SessionStateWord,
    /// The surface-context word.
    SurfaceContextWord,
    /// The authority-source word paired with a control-authority / active-driver / view-first-default /
    /// consent-scope gate role.
    AuthoritySourceWord,
}

impl CollaborationControlSharedStateFacet {
    /// Every state facet, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CollaborationControlRoleWord,
        Self::ObjectWord,
        Self::RegistryReferenceWord,
        Self::SessionStateWord,
        Self::SurfaceContextWord,
        Self::AuthoritySourceWord,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CollaborationControlRoleWord => "collaboration_control_role_word",
            Self::ObjectWord => "object_word",
            Self::RegistryReferenceWord => "registry_reference_word",
            Self::SessionStateWord => "session_state_word",
            Self::SurfaceContextWord => "surface_context_word",
            Self::AuthoritySourceWord => "authority_source_word",
        }
    }
}

/// Why a surface narrowed its rendering of a shared collaboration-control object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationControlSharedNarrowReason {
    /// A compact representation narrowed disclosure depth.
    CompactionNarrowed,
    /// A remote-projected representation narrowed to remote-backed truth.
    RemoteProjectionNarrowed,
    /// An exported representation narrowed to export-safe-redacted truth.
    ExportRedactionNarrowed,
}

impl CollaborationControlSharedNarrowReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompactionNarrowed => "compaction_narrowed",
            Self::RemoteProjectionNarrowed => "remote_projection_narrowed",
            Self::ExportRedactionNarrowed => "export_redaction_narrowed",
        }
    }
}

/// The next action a narrow note offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationControlSharedNarrowNextAction {
    /// Expand the object in the full desktop representation.
    ExpandInDesktop,
    /// Open the remote source backing the projection.
    OpenRemoteSource,
    /// Open the full detail behind the redacted export.
    OpenFullDetail,
}

impl CollaborationControlSharedNarrowNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandInDesktop => "expand_in_desktop",
            Self::OpenRemoteSource => "open_remote_source",
            Self::OpenFullDetail => "open_full_detail",
        }
    }
}

/// Whether a binding preserves full parity or discloses a narrowed representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationControlSharedVocabularyState {
    /// All vocabulary is preserved and shown in full.
    FacetsPreserved,
    /// All vocabulary is preserved and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl CollaborationControlSharedVocabularyState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetsPreserved => "facets_preserved",
            Self::FacetsDisclosedNarrowed => "facets_disclosed_narrowed",
        }
    }
}

/// Downgrade trigger that can narrow this consumer lane below its claimed collaboration-control parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlSharedConsumersDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Collaboration-control vocabulary drifted between surfaces for the same subject.
    CollaborationControlVocabularyDriftDetected,
    /// A gate-carrying role dropped its authority-source or session-state disclosure.
    AuthoritySourceOrSessionStateDisclosureDropped,
    /// A surface acquired terminal / debug control from presence or follow without an explicit grant.
    AcquiresTerminalOrDebugControlFromPresenceWithoutAnExplicitGrant,
    /// A surface allowed more than one active driver on a sensitive surface.
    AllowsMoreThanOneActiveDriverOnASensitiveSurface,
    /// A surface started recording, retention, or guest-scope widening silently.
    StartsRecordingRetentionOrGuestScopeWideningSilently,
    /// A surface replayed prior terminal / debug input on join or restore.
    ReplaysPriorTerminalOrDebugInputOnJoinOrRestore,
    /// A surface revealed raw secrets, command text, variable bodies, or clipboard contents without a guard.
    RevealsRawSecretsCommandTextVariableBodiesOrClipboardContentsWithoutAGuard,
    /// A deferred-intent or outbox system queued a sensitive collaboration-control action (a control grant,
    /// presenter handoff, or terminal input) across a reconnect or offline boundary without a fresh live review.
    DeferredIntentQueuedASensitiveControlActionWithoutAFreshLiveReview,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream shared collaboration-control object narrowed.
    UpstreamCollaborationControlNarrowed,
}

impl M5CollaborationControlSharedConsumersDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::CollaborationControlVocabularyDriftDetected,
        Self::AuthoritySourceOrSessionStateDisclosureDropped,
        Self::AcquiresTerminalOrDebugControlFromPresenceWithoutAnExplicitGrant,
        Self::AllowsMoreThanOneActiveDriverOnASensitiveSurface,
        Self::StartsRecordingRetentionOrGuestScopeWideningSilently,
        Self::ReplaysPriorTerminalOrDebugInputOnJoinOrRestore,
        Self::RevealsRawSecretsCommandTextVariableBodiesOrClipboardContentsWithoutAGuard,
        Self::DeferredIntentQueuedASensitiveControlActionWithoutAFreshLiveReview,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamCollaborationControlNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::CollaborationControlVocabularyDriftDetected => "collaboration_control_vocabulary_drift_detected",
            Self::AuthoritySourceOrSessionStateDisclosureDropped => {
                "authority_source_or_session_state_disclosure_dropped"
            }
            Self::AcquiresTerminalOrDebugControlFromPresenceWithoutAnExplicitGrant => {
                "acquires_terminal_or_debug_control_from_presence_without_an_explicit_grant"
            }
            Self::AllowsMoreThanOneActiveDriverOnASensitiveSurface => {
                "allows_more_than_one_active_driver_on_a_sensitive_surface"
            }
            Self::StartsRecordingRetentionOrGuestScopeWideningSilently => {
                "starts_recording_retention_or_guest_scope_widening_silently"
            }
            Self::ReplaysPriorTerminalOrDebugInputOnJoinOrRestore => {
                "replays_prior_terminal_or_debug_input_on_join_or_restore"
            }
            Self::RevealsRawSecretsCommandTextVariableBodiesOrClipboardContentsWithoutAGuard => {
                "reveals_raw_secrets_command_text_variable_bodies_or_clipboard_contents_without_a_guard"
            }
            Self::DeferredIntentQueuedASensitiveControlActionWithoutAFreshLiveReview => {
                "deferred_intent_queued_a_sensitive_control_action_without_a_fresh_live_review"
            }
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamCollaborationControlNarrowed => "upstream_collaboration_control_narrowed",
        }
    }
}

/// The controlled vocabulary a seeded collaboration-control subject presents.
///
/// These six words must be identical across every consumer surface that shows the same seeded subject. The
/// collaboration-control-role word must be a frozen role token; the rest are controlled words the subject's object
/// carries. A surface may narrow how much it renders, but it may never reword any of these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationControlSharedStateFacetValues {
    /// Collaboration-control-role word (must be a frozen [`M5CollaborationControlRole`] token).
    pub collaboration_control_role_word: String,
    /// Collaboration-control-object word.
    pub object_word: String,
    /// Canonical registry-reference word the object points at.
    pub registry_reference_word: String,
    /// Session-state word (viewer / driver / control-granted / recording-active / restore-view-only, etc.) the
    /// subject ships.
    pub session_state_word: String,
    /// Surface-context word.
    pub surface_context_word: String,
    /// Authority-source word paired with a control-authority / active-driver / view-first-default /
    /// consent-scope gate role.
    pub authority_source_word: String,
}

impl CollaborationControlSharedStateFacetValues {
    /// Whether every vocabulary word is present.
    pub fn all_present(&self) -> bool {
        !self.collaboration_control_role_word.trim().is_empty()
            && !self.object_word.trim().is_empty()
            && !self.registry_reference_word.trim().is_empty()
            && !self.session_state_word.trim().is_empty()
            && !self.surface_context_word.trim().is_empty()
            && !self.authority_source_word.trim().is_empty()
    }

    /// Whether the collaboration-control-role word is a member of the frozen role vocabulary.
    pub fn collaboration_control_role_word_in_vocabulary(&self) -> bool {
        is_known_collaboration_control_role_token(self.collaboration_control_role_word.trim())
    }

    /// Whether the subject honours the authority-source rule: a role that carries control-authority,
    /// active-driver, view-first-default, or consent-scope meaning must pair its surface change with a real
    /// authority-source-disclosed-and-control-grant-bound continuity and never collapse to a
    /// control-acquired-from-presence-alone, presence-shown-as-control-authority,
    /// second-active-driver-shown-on-a-sensitive-surface, or prior-input-replayed-as-live-control sentinel.
    pub fn authority_source_satisfied(&self) -> bool {
        match collaboration_control_role_from_token(self.collaboration_control_role_word.trim()) {
            Some(role)
                if role.must_be_present_before_surfacing_as_a_collaboration_control_result() =>
            {
                let membership = self.authority_source_word.trim().to_lowercase();
                !membership.is_empty()
                    && !AUTHORITY_SOURCE_ABSENT_SENTINELS.contains(&membership.as_str())
            }
            _ => true,
        }
    }
}

/// The explicit note a narrowed representation shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationControlSharedNarrowNote {
    /// Why the representation narrowed.
    pub reason: CollaborationControlSharedNarrowReason,
    /// Note naming the preserved vocabulary (never omitted).
    pub preserved_vocabulary_note: String,
    /// The next action offered.
    pub next_action: CollaborationControlSharedNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollaborationControlSharedRenderDisclosure {
    /// The vocabulary state the representation requires.
    pub vocabulary_state: CollaborationControlSharedVocabularyState,
    /// The narrow reason the representation requires, if any.
    pub narrow_reason: Option<CollaborationControlSharedNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<CollaborationControlSharedNarrowNextAction>,
    /// Whether the binding must carry an explicit narrow note.
    pub needs_narrow_note: bool,
    /// Whether the binding must carry an explicit remote-source note.
    pub needs_remote_source_note: bool,
    /// Whether the binding must carry an explicit export-safe-detail note.
    pub needs_export_detail_note: bool,
}

/// Resolves the render disclosures a consumer binding must carry from its representation.
///
/// The full desktop representation renders at full parity. A compact representation narrows disclosure depth, a
/// remote-projected representation names its remote source, and an exported representation names its
/// export-safe-detail boundary — but all three keep every vocabulary word and disclose the narrowing through an
/// explicit note.
pub const fn resolve_collaboration_control_shared_render_disclosure(
    representation: CollaborationControlSharedRepresentation,
) -> CollaborationControlSharedRenderDisclosure {
    match representation {
        CollaborationControlSharedRepresentation::DesktopFull => {
            CollaborationControlSharedRenderDisclosure {
                vocabulary_state: CollaborationControlSharedVocabularyState::FacetsPreserved,
                narrow_reason: None,
                narrow_next_action: None,
                needs_narrow_note: false,
                needs_remote_source_note: false,
                needs_export_detail_note: false,
            }
        }
        CollaborationControlSharedRepresentation::CompactNarrowed => {
            CollaborationControlSharedRenderDisclosure {
                vocabulary_state:
                    CollaborationControlSharedVocabularyState::FacetsDisclosedNarrowed,
                narrow_reason: Some(CollaborationControlSharedNarrowReason::CompactionNarrowed),
                narrow_next_action: Some(
                    CollaborationControlSharedNarrowNextAction::ExpandInDesktop,
                ),
                needs_narrow_note: true,
                needs_remote_source_note: false,
                needs_export_detail_note: false,
            }
        }
        CollaborationControlSharedRepresentation::RemoteProjected => {
            CollaborationControlSharedRenderDisclosure {
                vocabulary_state:
                    CollaborationControlSharedVocabularyState::FacetsDisclosedNarrowed,
                narrow_reason: Some(
                    CollaborationControlSharedNarrowReason::RemoteProjectionNarrowed,
                ),
                narrow_next_action: Some(
                    CollaborationControlSharedNarrowNextAction::OpenRemoteSource,
                ),
                needs_narrow_note: true,
                needs_remote_source_note: true,
                needs_export_detail_note: false,
            }
        }
        CollaborationControlSharedRepresentation::ExportedRedacted => {
            CollaborationControlSharedRenderDisclosure {
                vocabulary_state:
                    CollaborationControlSharedVocabularyState::FacetsDisclosedNarrowed,
                narrow_reason: Some(
                    CollaborationControlSharedNarrowReason::ExportRedactionNarrowed,
                ),
                narrow_next_action: Some(
                    CollaborationControlSharedNarrowNextAction::OpenFullDetail,
                ),
                needs_narrow_note: true,
                needs_remote_source_note: false,
                needs_export_detail_note: true,
            }
        }
    }
}

/// One consumer binding: a shared collaboration-control object rendered on one consumer surface in one representation for
/// one seeded collaboration-control subject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationControlSharedConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable subject id (shared across surfaces that show the same subject).
    pub subject_id: String,
    /// Human-readable subject identity.
    pub subject_label: String,
    /// Which shared collaboration-control object this binding renders.
    pub object: M5CollaborationControlObject,
    /// Which consumer surface renders it.
    pub consumer: M5CollaborationControlConsumerSurface,
    /// Which representation this surface renders.
    pub representation: CollaborationControlSharedRepresentation,
    /// The controlled vocabulary presented (identical across surfaces for one subject).
    pub state_facets: CollaborationControlSharedStateFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub vocabulary_state: CollaborationControlSharedVocabularyState,
    /// The explicit narrow note; required and complete when the binding narrows.
    pub narrow_note: Option<CollaborationControlSharedNarrowNote>,
    /// Remote-source note; required and non-empty when the disclosure demands it.
    pub remote_source_note: String,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Guardrail: this surface acquires terminal / debug control from presence or follow without an explicit
    /// grant. MUST be `false`.
    pub acquires_terminal_or_debug_control_from_presence_without_an_explicit_grant: bool,
    /// Guardrail: this surface allows more than one active driver on a sensitive surface. MUST be `false`.
    pub allows_more_than_one_active_driver_on_a_sensitive_surface: bool,
    /// Guardrail: this surface starts recording, retention, or guest-scope widening silently. MUST be `false`.
    pub starts_recording_retention_or_guest_scope_widening_silently: bool,
    /// Guardrail: this surface replays prior terminal / debug input on join or restore. MUST be `false`.
    pub replays_prior_terminal_or_debug_input_on_join_or_restore: bool,
    /// Guardrail: this surface reveals raw secrets, command text, variable bodies, or clipboard contents
    /// without a guard. MUST be `false`.
    pub reveals_raw_secrets_command_text_variable_bodies_or_clipboard_contents_without_a_guard:
        bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl CollaborationControlSharedConsumerBinding {
    /// Disclosures this binding must carry, derived from its representation.
    pub const fn disclosure(&self) -> CollaborationControlSharedRenderDisclosure {
        resolve_collaboration_control_shared_render_disclosure(self.representation)
    }

    /// Whether this binding renders below full parity.
    pub const fn is_narrowed(&self) -> bool {
        self.representation.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.acquires_terminal_or_debug_control_from_presence_without_an_explicit_grant
            && !self.allows_more_than_one_active_driver_on_a_sensitive_surface
            && !self.starts_recording_retention_or_guest_scope_widening_silently
            && !self.replays_prior_terminal_or_debug_input_on_join_or_restore
            && !self
                .reveals_raw_secrets_command_text_variable_bodies_or_clipboard_contents_without_a_guard
    }

    /// Whether this binding points at the canonical per-domain schema and the matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let domain_ref = self.object.canonical_domain_schema_ref();
        self.source_contract_refs
            .iter()
            .any(|reference| reference == domain_ref)
            && self
                .source_contract_refs
                .iter()
                .any(|reference| reference == M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollaborationControlSharedConsumersTrustReview {
    /// Object reuse is proven by fixtures rather than inferred from screenshots.
    pub object_reuse_proven_by_fixtures: bool,
    /// The same seeded subject presents the same vocabulary across surfaces.
    pub same_subject_same_collaboration_control_vocabulary_across_surfaces: bool,
    /// Every collaboration-control-role word is a frozen role token.
    pub collaboration_control_role_words_stay_in_frozen_vocabulary: bool,
    /// Gate-carrying roles never let presence or follow read as terminal / debug control authority.
    pub gate_roles_never_let_presence_read_as_control_authority: bool,
    /// A sensitive surface never allows more than one active driver at a time.
    pub never_more_than_one_active_driver_on_a_sensitive_surface: bool,
    /// Recording, retention, and guest scope are never widened silently.
    pub recording_retention_and_guest_scope_never_widened_silently: bool,
    /// Prior terminal / debug input is never replayed on join or restore.
    pub prior_terminal_debug_input_never_replayed_on_join_or_restore: bool,
    /// Raw secrets, command text, variable bodies, and clipboard contents are never revealed without a guard.
    pub raw_secrets_command_text_and_clipboard_never_revealed_without_a_guard: bool,
    /// Deferred-intent and outbox systems never queue a control grant, presenter handoff, or terminal input
    /// across a reconnect or offline boundary.
    pub deferred_intent_never_queues_control_grants_presenter_handoffs_or_terminal_input: bool,
    /// A refused control action explains why it was refused and demands a fresh live review rather than
    /// replaying later as if it were an idempotent background write.
    pub refused_control_actions_explain_instead_of_replaying_as_idempotent_background_writes: bool,
    /// Narrowing is disclosed across desktop, compact, remote, and exported forms.
    pub narrowing_disclosed_across_representations: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Copy / export / open-in-provider actions preserve one canonical payload rather than diverging.
    pub copy_export_open_provider_preserve_one_payload: bool,
    /// Downgrade narrows the claim rather than hiding the object.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl M5CollaborationControlSharedConsumersTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.object_reuse_proven_by_fixtures
            && self.same_subject_same_collaboration_control_vocabulary_across_surfaces
            && self.collaboration_control_role_words_stay_in_frozen_vocabulary
            && self.gate_roles_never_let_presence_read_as_control_authority
            && self.never_more_than_one_active_driver_on_a_sensitive_surface
            && self.recording_retention_and_guest_scope_never_widened_silently
            && self.prior_terminal_debug_input_never_replayed_on_join_or_restore
            && self.raw_secrets_command_text_and_clipboard_never_revealed_without_a_guard
            && self.deferred_intent_never_queues_control_grants_presenter_handoffs_or_terminal_input
            && self
                .refused_control_actions_explain_instead_of_replaying_as_idempotent_background_writes
            && self.narrowing_disclosed_across_representations
            && self.support_export_point_canonical_contracts
            && self.copy_export_open_provider_preserve_one_payload
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollaborationControlSharedConsumersProjection {
    /// The shared terminal / debug view consumes the shared collaboration-control vocabulary.
    pub shared_terminal_debug_view_consumes_shared_collaboration_control_vocabulary: bool,
    /// The collaboration join-review sheet consumes the shared collaboration-control vocabulary.
    pub collaboration_join_review_sheet_consumes_shared_collaboration_control_vocabulary: bool,
    /// The control-grant prompt consumes the shared collaboration-control vocabulary.
    pub control_grant_prompt_consumes_shared_collaboration_control_vocabulary: bool,
    /// The presenter-handoff sheet consumes the shared collaboration-control vocabulary.
    pub presenter_handoff_sheet_consumes_shared_collaboration_control_vocabulary: bool,
    /// The paste / secret guard consumes the shared collaboration-control vocabulary.
    pub paste_secret_guard_consumes_shared_collaboration_control_vocabulary: bool,
    /// The collaboration retention sheet consumes the shared collaboration-control vocabulary.
    pub collaboration_retention_sheet_consumes_shared_collaboration_control_vocabulary: bool,
    /// The session-restore view consumes the shared collaboration-control vocabulary.
    pub session_restore_view_consumes_shared_collaboration_control_vocabulary: bool,
    /// The support / export packet consumes the shared collaboration-control vocabulary.
    pub support_export_packet_consumes_shared_collaboration_control_vocabulary: bool,
    /// The help / docs surface consumes the shared collaboration-control vocabulary.
    pub help_docs_consumes_shared_collaboration_control_vocabulary: bool,
    /// Every object is adopted by two or more consumers.
    pub every_object_adopted_by_two_or_more_consumers: bool,
    /// Vocabulary is identical for the same seeded subject.
    pub collaboration_control_vocabulary_identical_for_same_subject: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps an object back to one shared contract object.
    pub export_maps_back_to_one_collaboration_control_object: bool,
    /// Deferred-intent and outbox systems are blocked from queueing sensitive collaboration-control actions.
    pub deferred_intent_and_outbox_systems_blocked_from_queueing_sensitive_control_actions: bool,
}

impl M5CollaborationControlSharedConsumersProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.shared_terminal_debug_view_consumes_shared_collaboration_control_vocabulary
            && self.collaboration_join_review_sheet_consumes_shared_collaboration_control_vocabulary
            && self.control_grant_prompt_consumes_shared_collaboration_control_vocabulary
            && self.presenter_handoff_sheet_consumes_shared_collaboration_control_vocabulary
            && self.paste_secret_guard_consumes_shared_collaboration_control_vocabulary
            && self.collaboration_retention_sheet_consumes_shared_collaboration_control_vocabulary
            && self.session_restore_view_consumes_shared_collaboration_control_vocabulary
            && self.support_export_packet_consumes_shared_collaboration_control_vocabulary
            && self.help_docs_consumes_shared_collaboration_control_vocabulary
            && self.every_object_adopted_by_two_or_more_consumers
            && self.collaboration_control_vocabulary_identical_for_same_subject
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_collaboration_control_object
            && self
                .deferred_intent_and_outbox_systems_blocked_from_queueing_sensitive_control_actions
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollaborationControlSharedConsumersProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5CollaborationControlSharedConsumersPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CollaborationControlSharedConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<CollaborationControlSharedConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CollaborationControlSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5CollaborationControlConsumerSurface>,
    /// Trust review block.
    pub trust_review: M5CollaborationControlSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5CollaborationControlSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CollaborationControlSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe collaboration-control shared-consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollaborationControlSharedConsumersPacket {
    /// Record kind; must equal [`M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<CollaborationControlSharedConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CollaborationControlSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5CollaborationControlConsumerSurface>,
    /// Trust review block.
    pub trust_review: M5CollaborationControlSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5CollaborationControlSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CollaborationControlSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CollaborationControlSharedConsumersPacket {
    /// Builds a collaboration-control shared-consumer packet from stable-lane input.
    pub fn new(input: M5CollaborationControlSharedConsumersPacketInput) -> Self {
        Self {
            record_kind: M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            consumer_bindings: input.consumer_bindings,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the collaboration-control shared-consumer parity invariants.
    pub fn validate(&self) -> Vec<M5CollaborationControlSharedConsumersViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_RECORD_KIND {
            violations.push(M5CollaborationControlSharedConsumersViolation::WrongRecordKind);
        }
        if self.schema_version != M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_SCHEMA_VERSION {
            violations.push(M5CollaborationControlSharedConsumersViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CollaborationControlSharedConsumersViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations
                .push(M5CollaborationControlSharedConsumersViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations
                .push(M5CollaborationControlSharedConsumersViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5CollaborationControlSharedConsumersViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(M5CollaborationControlSharedConsumersViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations
                .push(M5CollaborationControlSharedConsumersViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("collaboration-control shared-consumer packet serializes"),
        ) {
            violations
                .push(M5CollaborationControlSharedConsumersViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("collaboration-control shared-consumer packet serializes")
    }

    /// Deterministic matrix CSV, one row per consumer binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "object,consumer,representation,collaboration_control_role_word,vocabulary_state\n",
        );
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                binding.object.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.collaboration_control_role_word,
                binding.vocabulary_state.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .consumer_bindings
            .iter()
            .filter(|binding| binding.is_narrowed())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Shared Collaboration-Control Consumers: One Vocabulary Across Surfaces\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Consumer bindings: {} ({} narrowed)\n",
            self.consumer_bindings.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Consumer bindings\n\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "- **{}** [`{}`]: object `{}` on `{}`, representation `{}`, role `{}`\n",
                binding.subject_label,
                binding.binding_id,
                binding.object.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.collaboration_control_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in collaboration-control shared-consumer export.
#[derive(Debug)]
pub enum M5CollaborationControlSharedConsumersArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CollaborationControlSharedConsumersViolation>),
}

impl fmt::Display for M5CollaborationControlSharedConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "collaboration-control shared-consumer export parse failed: {error}"
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
                    "collaboration-control shared-consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CollaborationControlSharedConsumersArtifactError {}

/// Validation failures emitted by [`M5CollaborationControlSharedConsumersPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CollaborationControlSharedConsumersViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No consumer bindings are present.
    ConsumerBindingsMissing,
    /// A consumer binding is incomplete.
    BindingIncomplete,
    /// A binding's vocabulary values are incomplete.
    VocabularyFacetIncomplete,
    /// A binding's collaboration-control-role word is not a frozen role token.
    CollaborationControlRoleWordOutsideVocabulary,
    /// A binding's gate-carrying role dropped its authority source.
    AuthoritySourceMissingForGateRole,
    /// A binding's vocabulary state does not match its representation.
    VocabularyStateMismatch,
    /// Two surfaces show the same seeded subject with different vocabulary.
    CollaborationControlVocabularyDriftAcrossSurfaces,
    /// A shared object is not adopted by at least two distinct consumers.
    ObjectReuseUnproven,
    /// A support / export binding does not point at the canonical contracts.
    SupportExportReferenceMissing,
    /// A narrowed binding is missing its explicit narrow note.
    NarrowNoteMissing,
    /// A narrow note's reason does not match the required narrow reason.
    NarrowReasonMismatch,
    /// A narrow note's next action does not match the required next action.
    NarrowNextActionMismatch,
    /// A narrow note is missing its preserved-vocabulary note.
    NarrowNotePreservedVocabularyMissing,
    /// A narrow note is missing its next-action copy.
    NarrowNextActionLabelMissing,
    /// A full-desktop binding carries a narrow note it must not.
    UnexpectedNarrowNote,
    /// A binding that needs an explicit remote-source note is missing it.
    RemoteSourceNoteMissing,
    /// A binding that needs an explicit export-detail note is missing it.
    ExportDetailNoteMissing,
    /// A binding acquires terminal / debug control from presence or follow without an explicit grant.
    AcquiresTerminalOrDebugControlFromPresenceWithoutAnExplicitGrant,
    /// A binding allows more than one active driver on a sensitive surface.
    AllowsMoreThanOneActiveDriverOnASensitiveSurface,
    /// A binding starts recording, retention, or guest-scope widening silently.
    StartsRecordingRetentionOrGuestScopeWideningSilently,
    /// A binding replays prior terminal / debug input on join or restore.
    ReplaysPriorTerminalOrDebugInputOnJoinOrRestore,
    /// A binding reveals raw secrets, command text, variable bodies, or clipboard contents without a guard.
    RevealsRawSecretsCommandTextVariableBodiesOrClipboardContentsWithoutAGuard,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared object appears among the bindings.
    ObjectCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5CollaborationControlSharedConsumersViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ConsumerBindingsMissing => "consumer_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::VocabularyFacetIncomplete => "vocabulary_facet_incomplete",
            Self::CollaborationControlRoleWordOutsideVocabulary => "collaboration_control_role_word_outside_vocabulary",
            Self::AuthoritySourceMissingForGateRole => "authority_source_missing_for_gate_role",
            Self::VocabularyStateMismatch => "vocabulary_state_mismatch",
            Self::CollaborationControlVocabularyDriftAcrossSurfaces => {
                "collaboration_control_vocabulary_drift_across_surfaces"
            }
            Self::ObjectReuseUnproven => "object_reuse_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::NarrowNoteMissing => "narrow_note_missing",
            Self::NarrowReasonMismatch => "narrow_reason_mismatch",
            Self::NarrowNextActionMismatch => "narrow_next_action_mismatch",
            Self::NarrowNotePreservedVocabularyMissing => "narrow_note_preserved_vocabulary_missing",
            Self::NarrowNextActionLabelMissing => "narrow_next_action_label_missing",
            Self::UnexpectedNarrowNote => "unexpected_narrow_note",
            Self::RemoteSourceNoteMissing => "remote_source_note_missing",
            Self::ExportDetailNoteMissing => "export_detail_note_missing",
            Self::AcquiresTerminalOrDebugControlFromPresenceWithoutAnExplicitGrant => {
                "acquires_terminal_or_debug_control_from_presence_without_an_explicit_grant"
            }
            Self::AllowsMoreThanOneActiveDriverOnASensitiveSurface => {
                "allows_more_than_one_active_driver_on_a_sensitive_surface"
            }
            Self::StartsRecordingRetentionOrGuestScopeWideningSilently => {
                "starts_recording_retention_or_guest_scope_widening_silently"
            }
            Self::ReplaysPriorTerminalOrDebugInputOnJoinOrRestore => {
                "replays_prior_terminal_or_debug_input_on_join_or_restore"
            }
            Self::RevealsRawSecretsCommandTextVariableBodiesOrClipboardContentsWithoutAGuard => {
                "reveals_raw_secrets_command_text_variable_bodies_or_clipboard_contents_without_a_guard"
            }
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::ObjectCoverageMissing => "object_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable collaboration-control shared-consumer export.
pub fn current_stable_m5_collaboration_control_shared_consumers_export() -> Result<
    M5CollaborationControlSharedConsumersPacket,
    M5CollaborationControlSharedConsumersArtifactError,
> {
    let packet: M5CollaborationControlSharedConsumersPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-collaboration-control-shared-consumers-proof/support_export.json"
    )))
    .map_err(M5CollaborationControlSharedConsumersArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CollaborationControlSharedConsumersArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5CollaborationControlSharedConsumersPacket,
    violations: &mut Vec<M5CollaborationControlSharedConsumersViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_SCHEMA_REF,
        M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_DOC_REF,
        M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_REF,
        M5_COLLABORATION_CONTROL_MATRIX_DOC_REF,
    ];
    // The six objects each map to their own canonical domain schema; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object in M5CollaborationControlObject::ALL {
        domains.insert(object.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5CollaborationControlSharedConsumersViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5CollaborationControlSharedConsumersPacket,
    violations: &mut Vec<M5CollaborationControlSharedConsumersViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(M5CollaborationControlSharedConsumersViolation::ConsumerBindingsMissing);
        return;
    }

    // One vocabulary: the facet values must be identical for every binding that renders the same seeded
    // subject.
    let mut subject_facets: BTreeMap<&str, &CollaborationControlSharedStateFacetValues> =
        BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each object must be adopted by at least two distinct consumers.
    let mut object_consumers: BTreeMap<
        M5CollaborationControlObject,
        BTreeSet<M5CollaborationControlConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5CollaborationControlConsumerSurface> = BTreeSet::new();
    let mut seen_objects: BTreeSet<M5CollaborationControlObject> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.subject_id.trim().is_empty()
            || binding.subject_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5CollaborationControlSharedConsumersViolation::BindingIncomplete);
        }
        if !binding.state_facets.all_present() {
            violations
                .push(M5CollaborationControlSharedConsumersViolation::VocabularyFacetIncomplete);
        }
        if !binding
            .state_facets
            .collaboration_control_role_word_in_vocabulary()
        {
            violations.push(
                M5CollaborationControlSharedConsumersViolation::CollaborationControlRoleWordOutsideVocabulary,
            );
        }
        if !binding.state_facets.authority_source_satisfied() {
            violations.push(
                M5CollaborationControlSharedConsumersViolation::AuthoritySourceMissingForGateRole,
            );
        }

        let disclosure = binding.disclosure();

        if binding.vocabulary_state != disclosure.vocabulary_state {
            violations
                .push(M5CollaborationControlSharedConsumersViolation::VocabularyStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations
                        .push(M5CollaborationControlSharedConsumersViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations.push(
                            M5CollaborationControlSharedConsumersViolation::NarrowReasonMismatch,
                        );
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations.push(
                            M5CollaborationControlSharedConsumersViolation::NarrowNextActionMismatch,
                        );
                    }
                    if note.preserved_vocabulary_note.trim().is_empty() {
                        violations.push(
                            M5CollaborationControlSharedConsumersViolation::NarrowNotePreservedVocabularyMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5CollaborationControlSharedConsumersViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5CollaborationControlSharedConsumersViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_remote_source_note && binding.remote_source_note.trim().is_empty() {
            violations
                .push(M5CollaborationControlSharedConsumersViolation::RemoteSourceNoteMissing);
        }
        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations
                .push(M5CollaborationControlSharedConsumersViolation::ExportDetailNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.acquires_terminal_or_debug_control_from_presence_without_an_explicit_grant {
            violations.push(
                M5CollaborationControlSharedConsumersViolation::AcquiresTerminalOrDebugControlFromPresenceWithoutAnExplicitGrant,
            );
        }
        if binding.allows_more_than_one_active_driver_on_a_sensitive_surface {
            violations.push(
                M5CollaborationControlSharedConsumersViolation::AllowsMoreThanOneActiveDriverOnASensitiveSurface,
            );
        }
        if binding.starts_recording_retention_or_guest_scope_widening_silently {
            violations.push(
                M5CollaborationControlSharedConsumersViolation::StartsRecordingRetentionOrGuestScopeWideningSilently,
            );
        }
        if binding.replays_prior_terminal_or_debug_input_on_join_or_restore {
            violations.push(
                M5CollaborationControlSharedConsumersViolation::ReplaysPriorTerminalOrDebugInputOnJoinOrRestore,
            );
        }
        if binding
            .reveals_raw_secrets_command_text_variable_bodies_or_clipboard_contents_without_a_guard
        {
            violations.push(
                M5CollaborationControlSharedConsumersViolation::RevealsRawSecretsCommandTextVariableBodiesOrClipboardContentsWithoutAGuard,
            );
        }

        // Support / export consumers must map an object back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations.push(
                M5CollaborationControlSharedConsumersViolation::SupportExportReferenceMissing,
            );
        }

        // Vocabulary-drift accumulation.
        match subject_facets.get(binding.subject_id.as_str()) {
            None => {
                subject_facets.insert(binding.subject_id.as_str(), &binding.state_facets);
            }
            Some(existing) => {
                if **existing != binding.state_facets && !drift_reported {
                    violations.push(
                        M5CollaborationControlSharedConsumersViolation::CollaborationControlVocabularyDriftAcrossSurfaces,
                    );
                    drift_reported = true;
                }
            }
        }

        object_consumers
            .entry(binding.object)
            .or_default()
            .insert(binding.consumer);
        seen_consumers.insert(binding.consumer);
        seen_objects.insert(binding.object);
    }

    // Coverage: every consumer surface and every object must appear.
    for consumer in M5CollaborationControlConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations
                .push(M5CollaborationControlSharedConsumersViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for object in M5CollaborationControlObject::ALL {
        if !seen_objects.contains(&object) {
            violations.push(M5CollaborationControlSharedConsumersViolation::ObjectCoverageMissing);
            break;
        }
    }

    // Reuse: every present object must be adopted by two or more distinct consumers.
    for consumers in object_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5CollaborationControlSharedConsumersViolation::ObjectReuseUnproven);
            break;
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
