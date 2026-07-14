//! Shared shell, editor, review, notebook, debug, terminal, collaboration, companion-handoff, and
//! support / export consumers that keep the B141 workspace-window-restore families — shared workspace
//! authority, window-local topology, skeleton-first restore, no-rerun session hydration, and
//! display-topology recovery — at **one canonical registry** across every claimed M5 windowed surface.
//!
//! This module is the consumer-adoption lane for the five reusable workspace-restore families frozen in
//! [`crate::m5_window_restore_matrix`] and implemented by the workspace-authority / window-topology lane
//! ([`crate::m5_workspace_authority_and_window_topology_registries`]), the skeleton-first-restore /
//! session-hydration lane
//! ([`crate::m5_skeleton_first_restore_and_session_hydration_registries`]), the no-rerun
//! session-recovery / authority-replay-fence lane
//! ([`crate::m5_no_rerun_session_recovery_and_authority_replay_fence_registries`]), and the
//! display-topology-recovery / role-continuity lane
//! ([`crate::m5_display_topology_recovery_and_role_continuity_registries`]).
//!
//! It binds each shared window-restore family to the concrete restore-coordinator, shell, workspace,
//! session, diagnostics, docs / help, CLI / export, support-export, and general product consumers that
//! render it, and proves — by fixtures, not screenshots — that the same restore profile presents the same
//! window-restore-role, family, registry-reference, restore-context, surface-context, and
//! session-continuity grammar wherever it appears.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **Reuse.** Each of the five shared window-restore families must be adopted by at least two distinct
//!    consumers, so a family is proven to be shared restore-engine infrastructure rather than a
//!    one-surface, feature-local fork of workspace-authority, window-topology, restore-fidelity, or
//!    session-hydration copy.
//! 2. **One registry / no drift.** For a given restore profile every consumer surface must present
//!    identical [`WindowRestoreStateFacetValues`] — the same window-restore-role word, the same family
//!    word, the same registry-reference word, the same restore-context word, the same surface-context
//!    word, and the same session-continuity word. The window-restore-role word must be a token from the
//!    frozen [`M5WindowRestoreRole`] vocabulary, so no surface rewrites `workspace_authority`,
//!    `window_topology`, `pane_role`, `layout_skeleton`, `session_hydration`, `restore_fidelity`, or
//!    `display_affinity` in its own words. A surface may narrow *how much* it shows across desktop,
//!    compact, remote, and exported representations, but it may never reword the underlying grammar per
//!    surface, and a role that carries workspace-authority, session-hydration, restore-fidelity, or
//!    display-affinity meaning may never let a restore rerun or reattach session-scoped work implicitly,
//!    delete layout structure silently, strand a window or dialog off-screen after a display-topology
//!    remap, merge workspace-authority and window-topology into one opaque blob, or overclaim restore
//!    fidelity when only context or evidence reopened.
//! 3. **Map back to one family.** Support and CLI/export consumers must point at the canonical per-domain
//!    schema and the frozen matrix by id, so an exported packet can always map a shell / workspace /
//!    session / diagnostics window-restore surface back to one shared contract family.
//!
//! Narrowing is disclosed, never hidden: a compact, remote, or exported representation carries an
//! explicit [`WindowRestoreNarrowNote`] naming the reason, the preserved grammar, and the next action,
//! and an exported representation additionally names its export-safe detail boundary rather than
//! collapsing the profile out of view.
//!
//! The packet references upstream window-restore contracts by id rather than embedding their content.
//! Raw secret values, credentials, and private endpoints stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/shell/m5-window-restore-shared-consumers.schema.json`](../../../../schemas/shell/m5-window-restore-shared-consumers.schema.json).
//! The contract doc is
//! [`docs/recovery/m5_window_restore_shared_consumers_one_registry.md`](../../../../docs/recovery/m5_window_restore_shared_consumers_one_registry.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-window-restore-shared-consumers/`](../../../../fixtures/ui/m5-window-restore-shared-consumers/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_window_restore_shared_consumers,
    seeded_m5_window_restore_shared_consumers_compact_remote_narrowed,
    seeded_m5_window_restore_shared_consumers_exported_redaction_narrowed,
};

use crate::m5_window_restore_matrix::{
    M5WindowRestoreConsumerSurface, M5WindowRestoreFamily, M5WindowRestoreRole,
    M5_WINDOW_RESTORE_MATRIX_DOC_REF, M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5WindowRestoreSharedConsumersPacket`].
pub const M5_WINDOW_RESTORE_SHARED_CONSUMERS_RECORD_KIND: &str =
    "m5_window_restore_shared_consumer_registry_parity";

/// Schema version for window-restore shared-consumer parity records.
pub const M5_WINDOW_RESTORE_SHARED_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_WINDOW_RESTORE_SHARED_CONSUMERS_PACKET_ID: &str =
    "m5-window-restore-shared-consumers:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_WINDOW_RESTORE_SHARED_CONSUMERS_SCHEMA_REF: &str =
    "schemas/shell/m5-window-restore-shared-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_WINDOW_RESTORE_SHARED_CONSUMERS_DOC_REF: &str =
    "docs/recovery/m5_window_restore_shared_consumers_one_registry.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_WINDOW_RESTORE_SHARED_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/release/m5-window-restore-shared-consumers-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_WINDOW_RESTORE_SHARED_CONSUMERS_CSV_REF: &str =
    "artifacts/release/m5-window-restore-shared-consumers-proof/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_WINDOW_RESTORE_SHARED_CONSUMERS_REPORT_REF: &str =
    "artifacts/release/m5-window-restore-shared-consumers-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_WINDOW_RESTORE_SHARED_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-window-restore-shared-consumers";

/// Proof-freshness SLO in hours for this lane.
pub const M5_WINDOW_RESTORE_SHARED_CONSUMERS_PROOF_SLO_HOURS: u32 = 720;

/// Session-continuity sentinel words a workspace-authority / session-hydration / restore-fidelity /
/// display-affinity role may never fall back to; an authority-carrying role that changes restore
/// presentation must always keep a real preserved window-local-selection-and-no-rerun continuity, never
/// rerunning session-scoped work, reattaching a privileged session implicitly, stranding a window
/// off-screen, or overclaiming restore fidelity.
const SESSION_CONTINUITY_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "reran_session_scoped_work",
    "reattached_privileged_session",
    "stranded_window_offscreen",
    "overclaimed_restore_fidelity",
];

/// Whether a consumer surface is an export / support path that must map a family back to its
/// canonical contract by id.
pub const fn consumer_must_reference_canonical(consumer: M5WindowRestoreConsumerSurface) -> bool {
    matches!(
        consumer,
        M5WindowRestoreConsumerSurface::SupportExport | M5WindowRestoreConsumerSurface::CliExport
    )
}

/// Whether `token` is a member of the frozen [`M5WindowRestoreRole`] vocabulary.
///
/// This is the "one registry" gate: a restore profile's window-restore-role word must be a controlled
/// role token rather than a per-surface synonym.
pub fn is_known_window_restore_role_token(token: &str) -> bool {
    window_restore_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5WindowRestoreRole`], if it is one.
pub fn window_restore_role_from_token(token: &str) -> Option<M5WindowRestoreRole> {
    M5WindowRestoreRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// How much of a shared window-restore family a consumer renders for one representation.
///
/// Narrowing changes how much is shown, never the underlying grammar: a narrowed representation still
/// carries the same window-restore-role, family, registry-reference, restore-context, surface-context,
/// and session-continuity words, and discloses the narrowing through an explicit note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowRestoreRepresentation {
    /// The full desktop representation; nothing is narrowed.
    DesktopFull,
    /// A compact representation that narrows disclosure depth.
    CompactNarrowed,
    /// A remote-projected representation backed by a remote source.
    RemoteProjected,
    /// An exported, export-safe-redacted representation.
    ExportedRedacted,
}

impl WindowRestoreRepresentation {
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

/// A grammar axis whose word must stay identical across surfaces for one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowRestoreParityFacet {
    /// The frozen window-restore-role word.
    WindowRestoreRoleWord,
    /// The window-restore-family word.
    FamilyWord,
    /// The canonical registry-reference word the family points at.
    RegistryReferenceWord,
    /// The restore-context word (cold start / warm restore / crash-loop recovery / multi-monitor /
    /// remote reconnect) the profile ships.
    RestoreContextWord,
    /// The surface-context word.
    SurfaceContextWord,
    /// The session-continuity word paired with a workspace-authority / session-hydration /
    /// restore-fidelity / display-affinity role.
    SessionContinuityWord,
}

impl WindowRestoreParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WindowRestoreRoleWord,
        Self::FamilyWord,
        Self::RegistryReferenceWord,
        Self::RestoreContextWord,
        Self::SurfaceContextWord,
        Self::SessionContinuityWord,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WindowRestoreRoleWord => "window_restore_role_word",
            Self::FamilyWord => "family_word",
            Self::RegistryReferenceWord => "registry_reference_word",
            Self::RestoreContextWord => "restore_context_word",
            Self::SurfaceContextWord => "surface_context_word",
            Self::SessionContinuityWord => "session_continuity_word",
        }
    }
}

/// Why a surface narrowed its rendering of a shared window-restore family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowRestoreNarrowReason {
    /// A compact representation narrowed disclosure depth.
    CompactionNarrowed,
    /// A remote-projected representation narrowed to remote-backed truth.
    RemoteProjectionNarrowed,
    /// An exported representation narrowed to export-safe-redacted truth.
    ExportRedactionNarrowed,
}

impl WindowRestoreNarrowReason {
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
pub enum WindowRestoreNarrowNextAction {
    /// Expand the family in the full desktop representation.
    ExpandInDesktop,
    /// Open the remote source backing the projection.
    OpenRemoteSource,
    /// Open the full detail behind the redacted export.
    OpenFullDetail,
}

impl WindowRestoreNarrowNextAction {
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
pub enum WindowRestoreParityState {
    /// All grammar is preserved and shown in full.
    FacetsPreserved,
    /// All grammar is preserved and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl WindowRestoreParityState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetsPreserved => "facets_preserved",
            Self::FacetsDisclosedNarrowed => "facets_disclosed_narrowed",
        }
    }
}

/// Downgrade trigger that can narrow this consumer lane below its claimed parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowRestoreSharedConsumersDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Window-restore grammar drifted between surfaces for the same profile.
    WindowRestoreGrammarDriftDetected,
    /// An authority-carrying role dropped its session-continuity or window-local-selection meaning.
    SessionContinuityOrWindowLocalSelectionDropped,
    /// A restore reran commands or reattached privileged sessions implicitly.
    RerunsCommandsOrReattachesPrivilegedSessionsImplicitlyDuringRestore,
    /// A restore deleted layout structure silently on a missing extension or remote target.
    DeletesLayoutStructureSilentlyOnMissingExtensionOrRemoteTarget,
    /// A display-topology remap left windows or dialogs unreachable.
    LeavesWindowsOrDialogsUnreachableAfterDisplayTopologyRemap,
    /// Workspace-authority and window-topology state were merged into one opaque blob.
    MergesWorkspaceAuthorityAndWindowTopologyIntoOneOpaqueBlob,
    /// A restore overclaimed fidelity when only context or evidence reopened.
    OverclaimsRestoreFidelityWhenOnlyContextOrEvidenceReopened,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream shared window-restore family narrowed.
    UpstreamWindowRestoreNarrowed,
}

impl WindowRestoreSharedConsumersDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::WindowRestoreGrammarDriftDetected,
        Self::SessionContinuityOrWindowLocalSelectionDropped,
        Self::RerunsCommandsOrReattachesPrivilegedSessionsImplicitlyDuringRestore,
        Self::DeletesLayoutStructureSilentlyOnMissingExtensionOrRemoteTarget,
        Self::LeavesWindowsOrDialogsUnreachableAfterDisplayTopologyRemap,
        Self::MergesWorkspaceAuthorityAndWindowTopologyIntoOneOpaqueBlob,
        Self::OverclaimsRestoreFidelityWhenOnlyContextOrEvidenceReopened,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamWindowRestoreNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::WindowRestoreGrammarDriftDetected => "window_restore_grammar_drift_detected",
            Self::SessionContinuityOrWindowLocalSelectionDropped => {
                "session_continuity_or_window_local_selection_dropped"
            }
            Self::RerunsCommandsOrReattachesPrivilegedSessionsImplicitlyDuringRestore => {
                "reruns_commands_or_reattaches_privileged_sessions_implicitly_during_restore"
            }
            Self::DeletesLayoutStructureSilentlyOnMissingExtensionOrRemoteTarget => {
                "deletes_layout_structure_silently_on_missing_extension_or_remote_target"
            }
            Self::LeavesWindowsOrDialogsUnreachableAfterDisplayTopologyRemap => {
                "leaves_windows_or_dialogs_unreachable_after_display_topology_remap"
            }
            Self::MergesWorkspaceAuthorityAndWindowTopologyIntoOneOpaqueBlob => {
                "merges_workspace_authority_and_window_topology_into_one_opaque_blob"
            }
            Self::OverclaimsRestoreFidelityWhenOnlyContextOrEvidenceReopened => {
                "overclaims_restore_fidelity_when_only_context_or_evidence_reopened"
            }
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamWindowRestoreNarrowed => "upstream_window_restore_narrowed",
        }
    }
}

/// The controlled grammar a restore profile presents.
///
/// These six words must be identical across every consumer surface that shows the same restore profile.
/// The window-restore-role word must be a frozen role token; the rest are controlled words the profile's
/// family carries. A surface may narrow how much it renders, but it may never reword any of these values
/// per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowRestoreStateFacetValues {
    /// Window-restore-role word (must be a frozen [`M5WindowRestoreRole`] token).
    pub window_restore_role_word: String,
    /// Window-restore-family word.
    pub family_word: String,
    /// Canonical registry-reference word the family points at.
    pub registry_reference_word: String,
    /// Restore-context word (cold start / warm restore / crash-loop recovery / multi-monitor /
    /// remote reconnect) the profile ships.
    pub restore_context_word: String,
    /// Surface-context word.
    pub surface_context_word: String,
    /// Session-continuity word paired with a workspace-authority / session-hydration /
    /// restore-fidelity / display-affinity role.
    pub session_continuity_word: String,
}

impl WindowRestoreStateFacetValues {
    /// Whether every grammar word is present.
    pub fn all_present(&self) -> bool {
        !self.window_restore_role_word.trim().is_empty()
            && !self.family_word.trim().is_empty()
            && !self.registry_reference_word.trim().is_empty()
            && !self.restore_context_word.trim().is_empty()
            && !self.surface_context_word.trim().is_empty()
            && !self.session_continuity_word.trim().is_empty()
    }

    /// Whether the window-restore-role word is a member of the frozen role vocabulary.
    pub fn window_restore_role_word_in_vocabulary(&self) -> bool {
        is_known_window_restore_role_token(self.window_restore_role_word.trim())
    }

    /// Whether the profile honours the session-continuity rule: a role that carries
    /// workspace-authority, session-hydration, restore-fidelity, or display-affinity meaning must pair
    /// its restore change with a real preserved window-local-selection-and-no-rerun continuity and never
    /// collapse to a reran-session-scoped-work, reattached-privileged-session, stranded-window-offscreen,
    /// or overclaimed-restore-fidelity sentinel.
    pub fn session_continuity_satisfied(&self) -> bool {
        match window_restore_role_from_token(self.window_restore_role_word.trim()) {
            Some(role)
                if role
                    .must_preserve_window_local_selection_and_no_rerun_under_shared_authority() =>
            {
                let continuity = self.session_continuity_word.trim().to_lowercase();
                !continuity.is_empty()
                    && !SESSION_CONTINUITY_ABSENT_SENTINELS.contains(&continuity.as_str())
            }
            _ => true,
        }
    }
}

/// The explicit note a narrowed representation shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowRestoreNarrowNote {
    /// Why the representation narrowed.
    pub reason: WindowRestoreNarrowReason,
    /// Note naming the preserved grammar (never omitted).
    pub preserved_grammar_note: String,
    /// The next action offered.
    pub next_action: WindowRestoreNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowRestoreRenderDisclosure {
    /// The parity state the representation requires.
    pub parity_state: WindowRestoreParityState,
    /// The narrow reason the representation requires, if any.
    pub narrow_reason: Option<WindowRestoreNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<WindowRestoreNarrowNextAction>,
    /// Whether the binding must carry an explicit narrow note.
    pub needs_narrow_note: bool,
    /// Whether the binding must carry an explicit remote-source note.
    pub needs_remote_source_note: bool,
    /// Whether the binding must carry an explicit export-safe-detail note.
    pub needs_export_detail_note: bool,
}

/// Resolves the render disclosures a consumer binding must carry from its representation.
///
/// The full desktop representation renders at full parity. A compact representation narrows disclosure
/// depth, a remote-projected representation names its remote source, and an exported representation names
/// its export-safe-detail boundary — but all three keep every grammar word and disclose the narrowing
/// through an explicit note.
pub const fn resolve_window_restore_render_disclosure(
    representation: WindowRestoreRepresentation,
) -> WindowRestoreRenderDisclosure {
    match representation {
        WindowRestoreRepresentation::DesktopFull => WindowRestoreRenderDisclosure {
            parity_state: WindowRestoreParityState::FacetsPreserved,
            narrow_reason: None,
            narrow_next_action: None,
            needs_narrow_note: false,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        WindowRestoreRepresentation::CompactNarrowed => WindowRestoreRenderDisclosure {
            parity_state: WindowRestoreParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(WindowRestoreNarrowReason::CompactionNarrowed),
            narrow_next_action: Some(WindowRestoreNarrowNextAction::ExpandInDesktop),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        WindowRestoreRepresentation::RemoteProjected => WindowRestoreRenderDisclosure {
            parity_state: WindowRestoreParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(WindowRestoreNarrowReason::RemoteProjectionNarrowed),
            narrow_next_action: Some(WindowRestoreNarrowNextAction::OpenRemoteSource),
            needs_narrow_note: true,
            needs_remote_source_note: true,
            needs_export_detail_note: false,
        },
        WindowRestoreRepresentation::ExportedRedacted => WindowRestoreRenderDisclosure {
            parity_state: WindowRestoreParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(WindowRestoreNarrowReason::ExportRedactionNarrowed),
            narrow_next_action: Some(WindowRestoreNarrowNextAction::OpenFullDetail),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: true,
        },
    }
}

/// One consumer binding: a shared window-restore family rendered on one consumer surface in one
/// representation for one restore profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowRestoreConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable restore-profile id (shared across surfaces that show the same profile).
    pub restore_profile_id: String,
    /// Human-readable restore-profile identity.
    pub restore_profile_label: String,
    /// Which shared window-restore family this binding renders.
    pub family: M5WindowRestoreFamily,
    /// Which consumer surface renders it.
    pub consumer: M5WindowRestoreConsumerSurface,
    /// Which representation this surface renders.
    pub representation: WindowRestoreRepresentation,
    /// The controlled grammar presented (identical across surfaces for one profile).
    pub state_facets: WindowRestoreStateFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub parity_state: WindowRestoreParityState,
    /// The explicit narrow note; required and complete when the binding narrows.
    pub narrow_note: Option<WindowRestoreNarrowNote>,
    /// Remote-source note; required and non-empty when the disclosure demands it.
    pub remote_source_note: String,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Guardrail: this surface lets a restore rerun commands or reattach privileged sessions implicitly
    /// during restore. MUST be `false`.
    pub reruns_commands_or_reattaches_privileged_sessions_implicitly_during_restore: bool,
    /// Guardrail: this surface lets a restore delete layout structure silently on a missing extension or
    /// remote target. MUST be `false`.
    pub deletes_layout_structure_silently_on_missing_extension_or_remote_target: bool,
    /// Guardrail: this surface leaves windows or dialogs unreachable after a display-topology remap. MUST
    /// be `false`.
    pub leaves_windows_or_dialogs_unreachable_after_display_topology_remap: bool,
    /// Guardrail: this surface merges workspace-authority and window-topology state into one opaque blob.
    /// MUST be `false`.
    pub merges_workspace_authority_and_window_topology_into_one_opaque_blob: bool,
    /// Guardrail: this surface overclaims restore fidelity when only context or evidence reopened. MUST
    /// be `false`.
    pub overclaims_restore_fidelity_when_only_context_or_evidence_reopened: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl WindowRestoreConsumerBinding {
    /// Disclosures this binding must carry, derived from its representation.
    pub const fn disclosure(&self) -> WindowRestoreRenderDisclosure {
        resolve_window_restore_render_disclosure(self.representation)
    }

    /// Whether this binding renders below full parity.
    pub const fn is_narrowed(&self) -> bool {
        self.representation.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.reruns_commands_or_reattaches_privileged_sessions_implicitly_during_restore
            && !self.deletes_layout_structure_silently_on_missing_extension_or_remote_target
            && !self.leaves_windows_or_dialogs_unreachable_after_display_topology_remap
            && !self.merges_workspace_authority_and_window_topology_into_one_opaque_blob
            && !self.overclaims_restore_fidelity_when_only_context_or_evidence_reopened
    }

    /// Whether this binding points at the canonical per-domain schema and the matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let domain_ref = self.family.canonical_domain_schema_ref();
        self.source_contract_refs
            .iter()
            .any(|reference| reference == domain_ref)
            && self
                .source_contract_refs
                .iter()
                .any(|reference| reference == M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowRestoreSharedConsumersTrustReview {
    /// Family reuse is proven by fixtures rather than inferred from screenshots.
    pub family_reuse_proven_by_fixtures: bool,
    /// The same restore profile presents the same grammar across surfaces.
    pub same_profile_same_window_restore_across_surfaces: bool,
    /// Every window-restore-role word is a frozen role token.
    pub window_restore_role_words_stay_in_frozen_vocabulary: bool,
    /// Authority-carrying roles never clobber a window-local selection or rerun session-scoped work.
    pub authority_roles_never_clobber_selection_or_rerun_session_work: bool,
    /// A restore never reruns commands or reattaches a privileged session implicitly.
    pub restore_never_reruns_or_reattaches_session_scoped_work: bool,
    /// A restore never deletes layout structure silently on a missing extension or remote target.
    pub restore_never_deletes_layout_structure_silently: bool,
    /// A display-topology remap never leaves a window or dialog unreachable.
    pub display_remap_never_leaves_window_or_dialog_unreachable: bool,
    /// Workspace-authority and window-topology state are never merged into one opaque blob.
    pub workspace_authority_and_window_topology_never_merged_into_blob: bool,
    /// A restore never overclaims fidelity when only context or evidence reopened.
    pub restore_never_overclaims_fidelity_when_only_context_or_evidence_reopened: bool,
    /// Narrowing is disclosed across desktop, compact, remote, and exported forms.
    pub narrowing_disclosed_across_representations: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the family.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl WindowRestoreSharedConsumersTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.family_reuse_proven_by_fixtures
            && self.same_profile_same_window_restore_across_surfaces
            && self.window_restore_role_words_stay_in_frozen_vocabulary
            && self.authority_roles_never_clobber_selection_or_rerun_session_work
            && self.restore_never_reruns_or_reattaches_session_scoped_work
            && self.restore_never_deletes_layout_structure_silently
            && self.display_remap_never_leaves_window_or_dialog_unreachable
            && self.workspace_authority_and_window_topology_never_merged_into_blob
            && self.restore_never_overclaims_fidelity_when_only_context_or_evidence_reopened
            && self.narrowing_disclosed_across_representations
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowRestoreSharedConsumersProjection {
    /// The restore coordinator consumes the shared window-restore grammar.
    pub restore_coordinator_consumes_shared_window_restore: bool,
    /// The shell UI consumes the shared window-restore grammar.
    pub shell_ui_consumes_shared_window_restore: bool,
    /// The workspace service consumes the shared window-restore grammar.
    pub workspace_service_consumes_shared_window_restore: bool,
    /// The session service consumes the shared window-restore grammar.
    pub session_service_consumes_shared_window_restore: bool,
    /// The diagnostics surface consumes the shared window-restore grammar.
    pub diagnostics_consumes_shared_window_restore: bool,
    /// The docs / help surface consumes the shared window-restore grammar.
    pub docs_help_consumes_shared_window_restore: bool,
    /// The CLI / export path consumes the shared window-restore grammar.
    pub cli_export_consumes_shared_window_restore: bool,
    /// The support / export path consumes the shared window-restore grammar.
    pub support_export_consumes_shared_window_restore: bool,
    /// The general product UI surface consumes the shared window-restore grammar.
    pub product_ui_consumes_shared_window_restore: bool,
    /// Every family is adopted by two or more consumers.
    pub every_family_adopted_by_two_or_more_consumers: bool,
    /// Grammar is identical for the same restore profile.
    pub window_restore_identical_for_same_profile: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps a family back to one shared contract family.
    pub export_maps_back_to_one_window_restore_family: bool,
}

impl WindowRestoreSharedConsumersProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.restore_coordinator_consumes_shared_window_restore
            && self.shell_ui_consumes_shared_window_restore
            && self.workspace_service_consumes_shared_window_restore
            && self.session_service_consumes_shared_window_restore
            && self.diagnostics_consumes_shared_window_restore
            && self.docs_help_consumes_shared_window_restore
            && self.cli_export_consumes_shared_window_restore
            && self.support_export_consumes_shared_window_restore
            && self.product_ui_consumes_shared_window_restore
            && self.every_family_adopted_by_two_or_more_consumers
            && self.window_restore_identical_for_same_profile
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_window_restore_family
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowRestoreSharedConsumersProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5WindowRestoreSharedConsumersPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5WindowRestoreSharedConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<WindowRestoreConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<WindowRestoreSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5WindowRestoreConsumerSurface>,
    /// Trust review block.
    pub trust_review: WindowRestoreSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: WindowRestoreSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: WindowRestoreSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe window-restore shared-consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WindowRestoreSharedConsumersPacket {
    /// Record kind; must equal [`M5_WINDOW_RESTORE_SHARED_CONSUMERS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_WINDOW_RESTORE_SHARED_CONSUMERS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<WindowRestoreConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<WindowRestoreSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5WindowRestoreConsumerSurface>,
    /// Trust review block.
    pub trust_review: WindowRestoreSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: WindowRestoreSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: WindowRestoreSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5WindowRestoreSharedConsumersPacket {
    /// Builds a window-restore shared-consumer packet from stable-lane input.
    pub fn new(input: M5WindowRestoreSharedConsumersPacketInput) -> Self {
        Self {
            record_kind: M5_WINDOW_RESTORE_SHARED_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: M5_WINDOW_RESTORE_SHARED_CONSUMERS_SCHEMA_VERSION,
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

    /// Validates the window-restore shared-consumer parity invariants.
    pub fn validate(&self) -> Vec<M5WindowRestoreSharedConsumersViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_WINDOW_RESTORE_SHARED_CONSUMERS_RECORD_KIND {
            violations.push(M5WindowRestoreSharedConsumersViolation::WrongRecordKind);
        }
        if self.schema_version != M5_WINDOW_RESTORE_SHARED_CONSUMERS_SCHEMA_VERSION {
            violations.push(M5WindowRestoreSharedConsumersViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5WindowRestoreSharedConsumersViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5WindowRestoreSharedConsumersViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5WindowRestoreSharedConsumersViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5WindowRestoreSharedConsumersViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(M5WindowRestoreSharedConsumersViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5WindowRestoreSharedConsumersViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("window-restore shared-consumer packet serializes"),
        ) {
            violations.push(M5WindowRestoreSharedConsumersViolation::RawBoundaryMaterialInExport);
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
            .expect("window-restore shared-consumer packet serializes")
    }

    /// Deterministic matrix CSV, one row per consumer binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out =
            String::from("family,consumer,representation,window_restore_role_word,parity_state\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                binding.family.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.window_restore_role_word,
                binding.parity_state.as_str(),
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
        out.push_str("# Shared Window-Restore Consumers: One Registry Across Surfaces\n\n");
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
                "- **{}** [`{}`]: family `{}` on `{}`, representation `{}`, role `{}`\n",
                binding.restore_profile_label,
                binding.binding_id,
                binding.family.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.window_restore_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in window-restore shared-consumer export.
#[derive(Debug)]
pub enum M5WindowRestoreSharedConsumersArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5WindowRestoreSharedConsumersViolation>),
}

impl fmt::Display for M5WindowRestoreSharedConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "window-restore shared-consumer export parse failed: {error}"
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
                    "window-restore shared-consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5WindowRestoreSharedConsumersArtifactError {}

/// Validation failures emitted by [`M5WindowRestoreSharedConsumersPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5WindowRestoreSharedConsumersViolation {
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
    /// A binding's grammar values are incomplete.
    GrammarFacetIncomplete,
    /// A binding's window-restore-role word is not a frozen role token.
    WindowRestoreRoleWordOutsideVocabulary,
    /// A binding's authority-carrying role dropped its session continuity.
    SessionContinuityMissingForAuthorityRole,
    /// A binding's parity state does not match its representation.
    ParityStateMismatch,
    /// Two surfaces show the same restore profile with different grammar.
    WindowRestoreGrammarDriftAcrossSurfaces,
    /// A shared family is not adopted by at least two distinct consumers.
    FamilyReuseUnproven,
    /// A support / export binding does not point at the canonical contracts.
    SupportExportReferenceMissing,
    /// A narrowed binding is missing its explicit narrow note.
    NarrowNoteMissing,
    /// A narrow note's reason does not match the required narrow reason.
    NarrowReasonMismatch,
    /// A narrow note's next action does not match the required next action.
    NarrowNextActionMismatch,
    /// A narrow note is missing its preserved-grammar note.
    NarrowNotePreservedGrammarMissing,
    /// A narrow note is missing its next-action copy.
    NarrowNextActionLabelMissing,
    /// A full-desktop binding carries a narrow note it must not.
    UnexpectedNarrowNote,
    /// A binding that needs an explicit remote-source note is missing it.
    RemoteSourceNoteMissing,
    /// A binding that needs an explicit export-detail note is missing it.
    ExportDetailNoteMissing,
    /// A binding lets a restore rerun commands or reattach privileged sessions implicitly.
    RerunsCommandsOrReattachesPrivilegedSessionsImplicitlyDuringRestore,
    /// A binding lets a restore delete layout structure silently on a missing extension or remote target.
    DeletesLayoutStructureSilentlyOnMissingExtensionOrRemoteTarget,
    /// A binding leaves windows or dialogs unreachable after a display-topology remap.
    LeavesWindowsOrDialogsUnreachableAfterDisplayTopologyRemap,
    /// A binding merges workspace-authority and window-topology state into one opaque blob.
    MergesWorkspaceAuthorityAndWindowTopologyIntoOneOpaqueBlob,
    /// A binding overclaims restore fidelity when only context or evidence reopened.
    OverclaimsRestoreFidelityWhenOnlyContextOrEvidenceReopened,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared family appears among the bindings.
    FamilyCoverageMissing,
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

impl M5WindowRestoreSharedConsumersViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ConsumerBindingsMissing => "consumer_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::GrammarFacetIncomplete => "grammar_facet_incomplete",
            Self::WindowRestoreRoleWordOutsideVocabulary => {
                "window_restore_role_word_outside_vocabulary"
            }
            Self::SessionContinuityMissingForAuthorityRole => {
                "session_continuity_missing_for_authority_role"
            }
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::WindowRestoreGrammarDriftAcrossSurfaces => {
                "window_restore_grammar_drift_across_surfaces"
            }
            Self::FamilyReuseUnproven => "family_reuse_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::NarrowNoteMissing => "narrow_note_missing",
            Self::NarrowReasonMismatch => "narrow_reason_mismatch",
            Self::NarrowNextActionMismatch => "narrow_next_action_mismatch",
            Self::NarrowNotePreservedGrammarMissing => "narrow_note_preserved_grammar_missing",
            Self::NarrowNextActionLabelMissing => "narrow_next_action_label_missing",
            Self::UnexpectedNarrowNote => "unexpected_narrow_note",
            Self::RemoteSourceNoteMissing => "remote_source_note_missing",
            Self::ExportDetailNoteMissing => "export_detail_note_missing",
            Self::RerunsCommandsOrReattachesPrivilegedSessionsImplicitlyDuringRestore => {
                "reruns_commands_or_reattaches_privileged_sessions_implicitly_during_restore"
            }
            Self::DeletesLayoutStructureSilentlyOnMissingExtensionOrRemoteTarget => {
                "deletes_layout_structure_silently_on_missing_extension_or_remote_target"
            }
            Self::LeavesWindowsOrDialogsUnreachableAfterDisplayTopologyRemap => {
                "leaves_windows_or_dialogs_unreachable_after_display_topology_remap"
            }
            Self::MergesWorkspaceAuthorityAndWindowTopologyIntoOneOpaqueBlob => {
                "merges_workspace_authority_and_window_topology_into_one_opaque_blob"
            }
            Self::OverclaimsRestoreFidelityWhenOnlyContextOrEvidenceReopened => {
                "overclaims_restore_fidelity_when_only_context_or_evidence_reopened"
            }
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::FamilyCoverageMissing => "family_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable window-restore shared-consumer export.
pub fn current_stable_m5_window_restore_shared_consumers_export(
) -> Result<M5WindowRestoreSharedConsumersPacket, M5WindowRestoreSharedConsumersArtifactError> {
    let packet: M5WindowRestoreSharedConsumersPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-window-restore-shared-consumers-proof/support_export.json"
    )))
    .map_err(M5WindowRestoreSharedConsumersArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5WindowRestoreSharedConsumersArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5WindowRestoreSharedConsumersPacket,
    violations: &mut Vec<M5WindowRestoreSharedConsumersViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_WINDOW_RESTORE_SHARED_CONSUMERS_SCHEMA_REF,
        M5_WINDOW_RESTORE_SHARED_CONSUMERS_DOC_REF,
        M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF,
        M5_WINDOW_RESTORE_MATRIX_DOC_REF,
    ];
    // The five families map to two canonical domain schemas; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for family in M5WindowRestoreFamily::ALL {
        domains.insert(family.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5WindowRestoreSharedConsumersViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5WindowRestoreSharedConsumersPacket,
    violations: &mut Vec<M5WindowRestoreSharedConsumersViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(M5WindowRestoreSharedConsumersViolation::ConsumerBindingsMissing);
        return;
    }

    // One registry: the facet values must be identical for every binding that renders the same
    // restore profile.
    let mut profile_facets: BTreeMap<&str, &WindowRestoreStateFacetValues> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each family must be adopted by at least two distinct consumers.
    let mut family_consumers: BTreeMap<
        M5WindowRestoreFamily,
        BTreeSet<M5WindowRestoreConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5WindowRestoreConsumerSurface> = BTreeSet::new();
    let mut seen_families: BTreeSet<M5WindowRestoreFamily> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.restore_profile_id.trim().is_empty()
            || binding.restore_profile_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5WindowRestoreSharedConsumersViolation::BindingIncomplete);
        }
        if !binding.state_facets.all_present() {
            violations.push(M5WindowRestoreSharedConsumersViolation::GrammarFacetIncomplete);
        }
        if !binding
            .state_facets
            .window_restore_role_word_in_vocabulary()
        {
            violations.push(
                M5WindowRestoreSharedConsumersViolation::WindowRestoreRoleWordOutsideVocabulary,
            );
        }
        if !binding.state_facets.session_continuity_satisfied() {
            violations.push(
                M5WindowRestoreSharedConsumersViolation::SessionContinuityMissingForAuthorityRole,
            );
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5WindowRestoreSharedConsumersViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations.push(M5WindowRestoreSharedConsumersViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations
                            .push(M5WindowRestoreSharedConsumersViolation::NarrowReasonMismatch);
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations.push(
                            M5WindowRestoreSharedConsumersViolation::NarrowNextActionMismatch,
                        );
                    }
                    if note.preserved_grammar_note.trim().is_empty() {
                        violations.push(
                            M5WindowRestoreSharedConsumersViolation::NarrowNotePreservedGrammarMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5WindowRestoreSharedConsumersViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5WindowRestoreSharedConsumersViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_remote_source_note && binding.remote_source_note.trim().is_empty() {
            violations.push(M5WindowRestoreSharedConsumersViolation::RemoteSourceNoteMissing);
        }
        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations.push(M5WindowRestoreSharedConsumersViolation::ExportDetailNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.reruns_commands_or_reattaches_privileged_sessions_implicitly_during_restore {
            violations.push(
                M5WindowRestoreSharedConsumersViolation::RerunsCommandsOrReattachesPrivilegedSessionsImplicitlyDuringRestore,
            );
        }
        if binding.deletes_layout_structure_silently_on_missing_extension_or_remote_target {
            violations.push(
                M5WindowRestoreSharedConsumersViolation::DeletesLayoutStructureSilentlyOnMissingExtensionOrRemoteTarget,
            );
        }
        if binding.leaves_windows_or_dialogs_unreachable_after_display_topology_remap {
            violations.push(
                M5WindowRestoreSharedConsumersViolation::LeavesWindowsOrDialogsUnreachableAfterDisplayTopologyRemap,
            );
        }
        if binding.merges_workspace_authority_and_window_topology_into_one_opaque_blob {
            violations.push(
                M5WindowRestoreSharedConsumersViolation::MergesWorkspaceAuthorityAndWindowTopologyIntoOneOpaqueBlob,
            );
        }
        if binding.overclaims_restore_fidelity_when_only_context_or_evidence_reopened {
            violations.push(
                M5WindowRestoreSharedConsumersViolation::OverclaimsRestoreFidelityWhenOnlyContextOrEvidenceReopened,
            );
        }

        // Support / export consumers must map a family back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations.push(M5WindowRestoreSharedConsumersViolation::SupportExportReferenceMissing);
        }

        // Grammar-drift accumulation.
        match profile_facets.get(binding.restore_profile_id.as_str()) {
            None => {
                profile_facets.insert(binding.restore_profile_id.as_str(), &binding.state_facets);
            }
            Some(existing) => {
                if **existing != binding.state_facets && !drift_reported {
                    violations.push(
                        M5WindowRestoreSharedConsumersViolation::WindowRestoreGrammarDriftAcrossSurfaces,
                    );
                    drift_reported = true;
                }
            }
        }

        family_consumers
            .entry(binding.family)
            .or_default()
            .insert(binding.consumer);
        seen_consumers.insert(binding.consumer);
        seen_families.insert(binding.family);
    }

    // Coverage: every consumer surface and every family must appear.
    for consumer in M5WindowRestoreConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5WindowRestoreSharedConsumersViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for family in M5WindowRestoreFamily::ALL {
        if !seen_families.contains(&family) {
            violations.push(M5WindowRestoreSharedConsumersViolation::FamilyCoverageMissing);
            break;
        }
    }

    // Reuse: every present family must be adopted by two or more distinct consumers.
    for consumers in family_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5WindowRestoreSharedConsumersViolation::FamilyReuseUnproven);
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
