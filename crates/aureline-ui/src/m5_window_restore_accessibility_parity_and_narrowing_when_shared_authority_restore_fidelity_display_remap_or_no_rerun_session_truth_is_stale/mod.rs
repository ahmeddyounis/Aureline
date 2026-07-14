//! Keyboard / screen-reader / high-zoom / high-contrast / localization / CLI / export parity, and honest
//! automatic claim narrowing for the M5 shared-workspace-authority / window-local-topology /
//! skeleton-first-restore / no-rerun-session-hydration / display-topology-recovery window-restore families.
//!
//! This module is the M05-1186 accessibility-localization-support-export parity and auto-narrowing capstone
//! over the frozen M5 window-restore matrix ([`crate::m5_window_restore_matrix`]). Where the freeze matrix
//! defines the five governed workspace-restore families, and the 1181-1184 implementation lanes resolve their
//! per-surface workspace-authority, window-topology, skeleton-first restore, no-rerun session-hydration, and
//! display-topology-recovery truth, this lane certifies — per workspace-restore family — that workspace
//! authority / window topology / restore-fidelity class / missing-dependency posture / display-remap history /
//! no-rerun session state claims stay **keyboard-reachable, screen-reader-announced, high-zoom-legible,
//! high-contrast-safe, localization-safe, CLI/export-safe, and self-narrowing** rather than presenting a shared
//! authority that only lives in a shell screenshot, a restore that is claimed exact without proof, a
//! session-scoped tool shown as reattached when it was never fenced, or a display-remap shown as recovered when
//! its evidence has aged out as still a stable, trusted restore surface:
//!
//! - **Keyboard / screen-reader / high-zoom / high-contrast / localization / CLI reach.** Every family
//!   exposes a keyboard-reachable, screen-reader-announced, high-zoom-reflowing, high-contrast-legible,
//!   localization-safe, and CLI/headless-reachable path into the same window-restore identity, semantic role,
//!   registry reference, workspace authority, restore-fidelity class, and display affinity the rendered surface
//!   shows — never a pointer-only affordance hidden in shell chrome, an unlabeled control, or a workspace
//!   authority / restore class that only lives in a screenshot and strands assistive-tech, localized, or
//!   headless-CLI users. Structure-heavy families (the skeleton-first restore-fidelity table, the no-rerun
//!   session-replay table, the display-topology remap-history table) additionally bind their structured layout
//!   to a flat list / textual / CLI path.
//! - **Export parity.** The support / release / CLI export reconstructs each family's meaning from typed
//!   tokens and opaque refs **without a raw payload**, preserving the same window-restore identity, semantic
//!   role, registry reference, workspace authority, restore-fidelity class, and display affinity shown
//!   in-product so support, help, and release proof can reconstruct which workspace-restore truth class was
//!   active without leaking a raw secret blob, a machine-specific sensitive path, or a shell-only screenshot.
//! - **Honest auto-narrowing.** When a skeleton-first family's layout-skeleton proof can only be partially
//!   disclosed, a no-rerun / display family's session-replay fence cannot be confirmed, or a family's
//!   display-remap recovery evidence has aged out or is policy-blocked, the family's claim auto-narrows from
//!   `trusted_restore_surface` / `reviewable_restore_surface` to a layout-skeleton-disclosed /
//!   session-replay-unverified / display-recovery-unverified projection, discloses the narrowing with a precise
//!   trigger and binding dimension, and preserves the canonical window-restore identity / last-known registry
//!   reference. The underlying workspace-authority / window-topology / restore-fidelity / session-replay /
//!   display-recovery truth is never dropped opaquely. A family with every dimension intact must NOT carry a
//!   spurious narrowing, and a fidelity-overclaimed / evidence-aged / policy-blocked state can never keep a
//!   trusted, stable restore claim — window-restore meaning is never conveyed by a shell-chrome-only affordance,
//!   a mislabeled screenshot, or an unlabeled control alone.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the restore coordinator, the shell UI,
//!   the workspace service, the session service, the diagnostics surface, the docs / help surface, the CLI
//!   export, the support export, and the product UI so product, help, and release publication stay aligned on
//!   downgrade behavior rather than drifting in copy — a trusted-looking restore surface can never outrun the
//!   workspace-authority / restore-fidelity / session-replay / display-recovery evidence it is being viewed
//!   away from.
//!
//! Each [`WindowRestoreAccessibilityRow`] keys on one
//! [`crate::m5_window_restore_matrix::M5WindowRestoreFamily`] and reuses that frozen family vocabulary plus
//! the frozen [`M5WindowRestoreRequiredLabel`], [`M5WindowRestoreDowngradeTrigger`], and shared
//! [`M5WindowRestoreConsumerSurface`] consumer surfaces rather than minting parallel synonyms, so the
//! certified labels stay byte-identical to the matrix and the sibling window-restore packets.
//!
//! The packet is metadata-only: raw secret blobs, machine-specific sensitive paths, plaintext payloads, and
//! endpoint refs never cross this boundary; the packet carries only typed class tokens, opaque window-restore
//! refs, booleans, and controlled labels so support, release, and diagnostics exports can reconstruct exactly
//! which workspace-restore truth class was active without leaking sensitive material or a raw payload.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen window-restore vocabulary — the capstone certifies the freeze matrix's families, required
// labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::m5_window_restore_matrix::{
    M5WindowRestoreConsumerSurface, M5WindowRestoreDowngradeTrigger, M5WindowRestoreFamily,
    M5WindowRestoreRequiredLabel, M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF,
};

/// Schema version stamped on the M05-1186 window-restore accessibility parity packet.
pub const WINDOW_RESTORE_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`WindowRestoreAccessibilityPacket`].
pub const WINDOW_RESTORE_A11Y_RECORD_KIND: &str = "m5_window_restore_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`WindowRestoreAccessibilityRow`].
pub const WINDOW_RESTORE_A11Y_ROW_RECORD_KIND: &str = "m5_window_restore_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const WINDOW_RESTORE_A11Y_SCHEMA_REF: &str =
    "schemas/shell/m5-window-restore-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const WINDOW_RESTORE_A11Y_DOC_REF: &str =
    "docs/recovery/m5_window_restore_accessibility_parity.md";

/// Repo-relative path of the frozen window-restore matrix this lane certifies.
pub const WINDOW_RESTORE_A11Y_MATRIX_REF: &str = M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const WINDOW_RESTORE_A11Y_FIXTURE_DIR: &str =
    "fixtures/ui/m5-window-restore-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const WINDOW_RESTORE_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-window-restore-accessibility-parity/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const WINDOW_RESTORE_A11Y_CSV_REF: &str =
    "artifacts/release/m5-window-restore-accessibility-parity/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const WINDOW_RESTORE_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-window-restore-accessibility-parity.md";

/// The reusable window-restore families that render a dense, structured surface (the skeleton-first
/// restore-fidelity table, the no-rerun session-replay table, the display-topology remap-history table) and
/// therefore MUST bind their structured layout to an equivalent flat list / textual / CLI path so the
/// structure is navigable non-visually.
const fn family_is_structure_heavy(family: M5WindowRestoreFamily) -> bool {
    matches!(
        family,
        M5WindowRestoreFamily::SkeletonFirstRestore
            | M5WindowRestoreFamily::NoRerunSessionHydration
            | M5WindowRestoreFamily::DisplayTopologyRecovery
    )
}

/// The workspace-restore-truth dimension whose weakening a family primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(family: M5WindowRestoreFamily) -> M5WindowRestoreClaimDimension {
    match family {
        M5WindowRestoreFamily::SharedWorkspaceAuthority => {
            M5WindowRestoreClaimDimension::SharedAuthorityClarity
        }
        M5WindowRestoreFamily::WindowLocalTopology => {
            M5WindowRestoreClaimDimension::WindowLocalIsolationClarity
        }
        M5WindowRestoreFamily::SkeletonFirstRestore => {
            M5WindowRestoreClaimDimension::SkeletonFidelityClarity
        }
        M5WindowRestoreFamily::NoRerunSessionHydration => {
            M5WindowRestoreClaimDimension::SessionReplayFenceClarity
        }
        M5WindowRestoreFamily::DisplayTopologyRecovery => {
            M5WindowRestoreClaimDimension::DisplayRecoveryClarity
        }
    }
}

/// A rendered fallback modality for a window-restore family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowRestoreFallbackModality {
    /// A rich, structured (restore-fidelity / session-replay / remap-history table) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / label-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5WindowRestoreFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured surface
    /// (i.e. a keyboard / screen-reader / CLI path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Textual | Self::Cli)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::List => "list",
            Self::Textual => "textual",
            Self::Cli => "cli",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same window-restore
/// family may render at desktop-full capability or narrow to a companion, read-only browser, headless CLI,
/// docs export, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowRestoreRenderingSurface {
    /// The full-capability desktop surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A docs / help export projection.
    DocsExport,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5WindowRestoreRenderingSurface {
    /// Returns true when the surface narrows the window-restore family below the desktop full-capability
    /// baseline and therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::CliHeadless => "cli_headless",
            Self::DocsExport => "docs_export",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / high-zoom / high-contrast / localization / CLI reach for a window-restore
/// family's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowRestoreNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A shell-chrome-only / pointer-only / view-only surface that traps keyboard / assistive-tech /
    /// localized / headless-CLI users (red).
    ViewOnlyTrap,
}

impl WindowRestoreNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / localized / CLI users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndLabeled => "reachable_and_labeled",
            Self::DisclosedReducedButReachable => "disclosed_reduced_but_reachable",
            Self::ViewOnlyTrap => "view_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the window-restore meaning without leaking a raw payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowRestoreExportSummaryState {
    /// The window-restore meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl WindowRestoreExportSummaryState {
    /// Returns true when the export never falls back to leaking a raw payload.
    pub const fn never_requires_raw_payload(self) -> bool {
        !matches!(self, Self::RequiresRawPayload)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutRawPayload => "reconstructable_without_raw_payload",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::RequiresRawPayload => "requires_raw_payload",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced window-restore projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowRestoreNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced projection, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Window-restore state or tokens dropped without disclosure (red).
    SilentlyDropped,
}

impl WindowRestoreNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or tokens.
    pub const fn never_drops_silently(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityPreserved => "parity_preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::SilentlyDropped => "silently_dropped",
        }
    }
}

/// The window-restore claim ceiling a family asserts: how strong a trusted / stable posture it lets a surface
/// present. Auto-narrowing lowers this ceiling when a skeleton-fidelity / session-replay / display-recovery
/// dimension weakens so a partially-disclosed layout skeleton, an unconfirmed session-replay fence, or an
/// aged-out / policy-blocked display-remap recovery can never keep an old `TrustedRestoreSurface` or
/// `ReviewableRestoreSurface` label — window-restore meaning is never conveyed by a shell-chrome-only
/// affordance, a mislabeled screenshot, or an unlabeled control alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowRestoreA11yClaim {
    /// Trusted restore surface: a fully current, registry-bound, authority-inspectable, window-local-isolated,
    /// session-replay-fenced, display-recovered restore topology — the strongest claim, a window-restore
    /// surface Aureline can present as exactly trusted and stable right now.
    TrustedRestoreSurface,
    /// Reviewable restore surface: a self-sufficient, inspectable read-only window-restore projection (a
    /// static window-local pane topology / registry reference an operator can inspect) that is not itself an
    /// authoritative, live-resolving surface.
    ReviewableRestoreSurface,
    /// Layout-skeleton-disclosed projection: a skeleton-first restore's layout-skeleton proof can only be
    /// partially disclosed; the family stays a layout-skeleton-disclosed projection that discloses the partial
    /// skeleton proof alongside the last-known pane-role placeholders, never a hydrated layout shown as exact
    /// when its skeleton proof is incomplete.
    LayoutSkeletonDisclosedProjection,
    /// Session-replay-unverified projection: a no-rerun session-hydration family's session-replay fence cannot
    /// be confirmed; the family stays a session-replay-unverified projection that keeps the last-known
    /// reopened-versus-rerun posture explicit, never a session shown as reattached when it may have silently
    /// reran.
    SessionReplayUnverifiedProjection,
    /// Display-recovery-unverified projection: a family's display-topology remap / recovery evidence has aged
    /// out or is policy-blocked; the family stays a display-recovery-unverified projection that keeps the
    /// last-known monitor-affinity / bounds state explicit, never a window shown as recovered on-screen when
    /// its remap evidence has aged out or become policy-blocked.
    DisplayRecoveryUnverifiedProjection,
}

impl M5WindowRestoreA11yClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 5] = [
        Self::TrustedRestoreSurface,
        Self::ReviewableRestoreSurface,
        Self::LayoutSkeletonDisclosedProjection,
        Self::SessionReplayUnverifiedProjection,
        Self::DisplayRecoveryUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::TrustedRestoreSurface => 4,
            Self::ReviewableRestoreSurface => 3,
            Self::LayoutSkeletonDisclosedProjection => 2,
            Self::SessionReplayUnverifiedProjection => 1,
            Self::DisplayRecoveryUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully trusted, stable restore surface.
    pub const fn asserts_trusted_surface(self) -> bool {
        matches!(self, Self::TrustedRestoreSurface)
    }

    /// Returns true when this claim asserts a fully self-sufficient (trusted or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::TrustedRestoreSurface | Self::ReviewableRestoreSurface
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedRestoreSurface => "trusted_restore_surface",
            Self::ReviewableRestoreSurface => "reviewable_restore_surface",
            Self::LayoutSkeletonDisclosedProjection => "layout_skeleton_disclosed_projection",
            Self::SessionReplayUnverifiedProjection => "session_replay_unverified_projection",
            Self::DisplayRecoveryUnverifiedProjection => "display_recovery_unverified_projection",
        }
    }
}

/// The shared-authority / window-local / skeleton-fidelity / session-replay / display-recovery dimension whose
/// state governs how far a window-restore family may claim to be a fully trusted, stable restore surface.
/// The dimensions map 1:1 to the five frozen workspace-restore families so every family carries an honest
/// narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowRestoreClaimDimension {
    /// Shared-authority clarity: is the shared workspace authority and its window binding inspectable rather
    /// than hidden (shared-workspace-authority)?
    SharedAuthorityClarity,
    /// Window-local-isolation clarity: is the window-local pane topology versioned and attributable rather
    /// than an opaque blob (window-local-topology)?
    WindowLocalIsolationClarity,
    /// Skeleton-fidelity clarity: does the skeleton-first restore prove a rebuilt layout skeleton and disclosed
    /// fidelity rather than silently deleting layout structure (skeleton-first-restore)?
    SkeletonFidelityClarity,
    /// Session-replay-fence clarity: does the no-rerun session hydration stay fenced rather than silently
    /// rerunning or reattaching a privileged session (no-rerun-session-hydration)?
    SessionReplayFenceClarity,
    /// Display-recovery clarity: does the display-topology remap / recovery evidence stay current rather than
    /// aging out or becoming policy-blocked (display-topology-recovery)?
    DisplayRecoveryClarity,
}

impl M5WindowRestoreClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SharedAuthorityClarity,
        Self::WindowLocalIsolationClarity,
        Self::SkeletonFidelityClarity,
        Self::SessionReplayFenceClarity,
        Self::DisplayRecoveryClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SharedAuthorityClarity => "shared_authority_clarity",
            Self::WindowLocalIsolationClarity => "window_local_isolation_clarity",
            Self::SkeletonFidelityClarity => "skeleton_fidelity_clarity",
            Self::SessionReplayFenceClarity => "session_replay_fence_clarity",
            Self::DisplayRecoveryClarity => "display_recovery_clarity",
        }
    }
}

/// The observed condition of one workspace-restore dimension. Anything weaker than [`Self::FullyQualified`]
/// imposes a narrowing ceiling on the family's claim. The unconfirmed states the lane must auto-narrow on as
/// *weakened evidence* — an unconfirmed session-replay fence and an aged-out / policy-blocked display-remap
/// recovery evidence — are the states that [`Self::cannot_be_shown_trusted`] flags. A partially-disclosed
/// layout skeleton is an honest disclosed-absence operation (a partial skeleton proof shown honestly with the
/// last-known pane-role placeholders), not a truth overstatement, so it is deliberately excluded there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowRestoreConditionState {
    /// Fully current, registry-bound, authority-inspectable, window-local-isolated, session-replay-fenced,
    /// display-recovered — imposes no ceiling.
    FullyQualified,
    /// The skeleton-first restore's layout-skeleton proof can only be partially disclosed — claim drops to a
    /// layout-skeleton-disclosed projection.
    LayoutSkeletonDisclosedPartial,
    /// The no-rerun session-replay fence cannot be confirmed — claim drops to a session-replay-unverified
    /// projection.
    SessionReplayUnconfirmed,
    /// The display-topology remap / recovery evidence has aged out or is policy-blocked — claim drops to a
    /// display-recovery-unverified projection.
    DisplayRecoveryUnconfirmed,
}

impl M5WindowRestoreConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullyQualified,
        Self::LayoutSkeletonDisclosedPartial,
        Self::SessionReplayUnconfirmed,
        Self::DisplayRecoveryUnconfirmed,
    ];

    /// Returns true when the dimension is weaker than fully qualified and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FullyQualified)
    }

    /// Returns true when the condition reflects weakened evidence that cannot be shown as a fully trusted,
    /// stable restore surface and must never be shown as such. A partially-disclosed layout skeleton is an
    /// honest disclosed-absence operation (a partial skeleton proof shown honestly with the last-known pane-role
    /// placeholders), not a truth overstatement, so it is deliberately excluded here.
    pub const fn cannot_be_shown_trusted(self) -> bool {
        matches!(
            self,
            Self::SessionReplayUnconfirmed | Self::DisplayRecoveryUnconfirmed
        )
    }

    /// The strongest claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5WindowRestoreA11yClaim {
        match self {
            Self::FullyQualified => M5WindowRestoreA11yClaim::TrustedRestoreSurface,
            Self::LayoutSkeletonDisclosedPartial => {
                M5WindowRestoreA11yClaim::LayoutSkeletonDisclosedProjection
            }
            Self::SessionReplayUnconfirmed => {
                M5WindowRestoreA11yClaim::SessionReplayUnverifiedProjection
            }
            Self::DisplayRecoveryUnconfirmed => {
                M5WindowRestoreA11yClaim::DisplayRecoveryUnverifiedProjection
            }
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each state maps
    /// to the on-topic frozen trigger the freeze matrix already governs, so the certified reason stays
    /// byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5WindowRestoreDowngradeTrigger {
        match self {
            // The fully-qualified baseline never narrows; kept for exhaustiveness.
            Self::FullyQualified => M5WindowRestoreDowngradeTrigger::ProofStale,
            Self::LayoutSkeletonDisclosedPartial => M5WindowRestoreDowngradeTrigger::ProofStale,
            Self::SessionReplayUnconfirmed => {
                M5WindowRestoreDowngradeTrigger::OverclaimedRestoreFidelityWhenOnlyContextOrEvidenceReopened
            }
            Self::DisplayRecoveryUnconfirmed => {
                M5WindowRestoreDowngradeTrigger::OverclaimedRestoreFidelityWhenOnlyContextOrEvidenceReopened
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyQualified => "fully_qualified",
            Self::LayoutSkeletonDisclosedPartial => "layout_skeleton_disclosed_partial",
            Self::SessionReplayUnconfirmed => "session_replay_unconfirmed",
            Self::DisplayRecoveryUnconfirmed => "display_recovery_unconfirmed",
        }
    }
}

/// One workspace-restore dimension's observed condition on a family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowRestoreClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5WindowRestoreClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5WindowRestoreConditionState,
}

/// An honest claim auto-narrow block. When a workspace-restore dimension weakens, the family's claim lowers to
/// the permitted ceiling, names the binding dimension and frozen trigger, and preserves the canonical
/// window-restore identity / last-known registry reference rather than silently dropping it — the underlying
/// workspace-authority / window-topology / restore-fidelity / session-replay / display-recovery truth is never
/// erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowRestoreClaimAutoNarrow {
    /// The claim the family is narrowed to.
    pub narrowed_to: M5WindowRestoreA11yClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling constraint).
    pub binding_dimension: M5WindowRestoreClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5WindowRestoreDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical window-restore identity and last-known registry reference are preserved rather than
    /// dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying workspace-authority / window-topology / restore-fidelity / session-replay /
    /// display-recovery truth is preserved (never dropped) across the narrowing; must hold so
    /// layout-skeleton-disclosed, session-replay-unverified, and display-recovery-unverified states never fail
    /// opaquely.
    pub preserves_truth_continuity: bool,
}

impl WindowRestoreClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and workspace-authority /
    /// window-topology / restore-fidelity / session-replay / display-recovery truth and carries a precise,
    /// non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_truth_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a window-restore family's accessible fallback: the same truth must be copyable
/// as text / JSON / Markdown, and a raw payload is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowRestoreCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl WindowRestoreCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at least one
    /// export field is named, and a raw-payload-only export is prohibited.
    pub fn is_complete(&self) -> bool {
        self.raw_payload_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowRestoreRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5WindowRestoreRenderingSurface,
    /// How the surface discloses its reduced window-restore projection.
    pub state: WindowRestoreNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The window-restore affordances reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a window-restore accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowRestoreAccessibilityStatus {
    /// Full keyboard / screen-reader / high-zoom / high-contrast / localization / CLI / export parity with no
    /// narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims trusted, or drops state silently (red).
    Stranded,
}

impl WindowRestoreAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one window-restore family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowRestoreAccessibilityRow {
    /// Record kind; must equal [`WINDOW_RESTORE_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`WINDOW_RESTORE_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen workspace-restore family this row certifies.
    pub window_restore_family: M5WindowRestoreFamily,
    /// Ref to the frozen canonical per-domain schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the window-restore family this row represents; stays visible on every surface, so this
    /// is never empty.
    pub window_restore_context_ref: String,
    /// Rendered modalities offered; a structure-heavy family must also offer a non-visual (list / textual /
    /// CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5WindowRestoreFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical window-restore identity, semantic role, registry
    /// reference, workspace authority, restore-fidelity class, and display affinity as the rendered family;
    /// must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: WindowRestoreNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: WindowRestoreNonVisualReachState,
    /// High-zoom (200–400% reflow / magnification) legibility of the non-visual path.
    pub high_zoom_reach: WindowRestoreNonVisualReachState,
    /// High-contrast / larger-text legibility of the non-visual path.
    pub high_contrast_reach: WindowRestoreNonVisualReachState,
    /// Localization (translated vocabulary / locale-specific labels) fidelity of the non-visual path.
    pub localization_reach: WindowRestoreNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: WindowRestoreNonVisualReachState,
    /// Whether the export-safe summary preserves window-restore meaning.
    pub export_summary: WindowRestoreExportSummaryState,
    /// Ref to the export-safe summary object for this family.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: WindowRestoreCopyExportParity,
    /// The full claim this family asserts when every dimension is intact.
    pub full_ready_claim: M5WindowRestoreA11yClaim,
    /// The observed condition of each modeled workspace-restore dimension.
    #[serde(default)]
    pub claim_conditions: Vec<WindowRestoreClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<WindowRestoreClaimAutoNarrow>,
    /// Whether the underlying workspace-authority / window-topology / restore-fidelity / session-replay /
    /// display-recovery truth is preserved on this family regardless of narrowing; must hold so every
    /// unverified projection never fails opaquely.
    pub truth_preserved: bool,
    /// Rendering surfaces this family is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5WindowRestoreRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<WindowRestoreRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5WindowRestoreRequiredLabel>,
    /// Semantic consumer surfaces this family is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5WindowRestoreConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl WindowRestoreAccessibilityRow {
    /// Returns true when this family renders a dense, structured surface and must bind to a flat non-visual
    /// path.
    pub const fn is_structure_heavy(&self) -> bool {
        family_is_structure_heavy(self.window_restore_family)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `FullyQualified` when the row does not model that
    /// dimension.
    pub fn condition_for(
        &self,
        dimension: M5WindowRestoreClaimDimension,
    ) -> M5WindowRestoreConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5WindowRestoreConditionState::FullyQualified)
    }

    /// Whether any modeled dimension is weaker than fully qualified.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling, capped at the family's
    /// full claim.
    pub fn permitted_claim(&self) -> M5WindowRestoreA11yClaim {
        let mut permitted = self.full_ready_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension narrows below
    /// the family's full claim.
    pub fn binding_condition(&self) -> Option<&WindowRestoreClaimConditionEntry> {
        let mut binding: Option<(&WindowRestoreClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_ready_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition, rank)),
            }
        }
        binding.map(|(condition, _)| condition)
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any.
    pub fn binding_dimension(&self) -> Option<M5WindowRestoreClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The claim this family effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5WindowRestoreA11yClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_ready_claim,
        }
    }

    /// AC / auto-narrowing honesty: a partially-disclosed layout skeleton, an unconfirmed session-replay
    /// fence, or an aged-out / policy-blocked display-remap recovery can no longer keep an old
    /// `TrustedRestoreSurface` / `ReviewableRestoreSurface` label. The effective claim never exceeds the
    /// permitted ceiling; when a dimension narrows below the full claim, an honest narrow block is present,
    /// narrows to exactly the permitted ceiling, binds to the ceiling-imposing dimension with its frozen
    /// trigger, and preserves canonical identity and truth. When nothing narrows, no spurious narrow block is
    /// present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_condition()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding.dimension
                    && narrow.trigger == binding.state.default_trigger()
                    && binding.state.is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / trusted honesty: an unconfirmed-session-replay / aged-out-display-recovery state never keeps a
    /// trusted claim — window-restore meaning is never conveyed by a shell-chrome-only affordance, a
    /// mislabeled screenshot, or an unlabeled control alone. When such a state is modeled, the effective claim
    /// must not assert `TrustedRestoreSurface`.
    pub fn trusted_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_shown_trusted());
        !(has_unprovable_state && self.effective_claim().asserts_trusted_surface())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical truth — no
    /// keyboard / screen-reader / high-zoom / high-contrast / localization / CLI trap, a structure-heavy
    /// family offers a non-visual fallback, and the export reconstructs meaning without a raw payload.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.window_restore_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.high_zoom_reach.never_traps()
            && self.high_contrast_reach.never_traps()
            && self.localization_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_structure_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the window-restore meaning without leaking a raw payload.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_payload()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: every unverified projection preserves the underlying workspace-authority / window-topology
    /// / restore-fidelity / session-replay / display-recovery truth. The row must assert `truth_preserved`, and
    /// any narrow block must preserve truth continuity too.
    pub fn preserves_truth_continuity(&self) -> bool {
        self.truth_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_truth_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the family carries an honest claim
    /// narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.high_zoom_reach.is_disclosed_reduction()
            || self.high_contrast_reach.is_disclosed_reduction()
            || self.localization_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced window-restore
    /// projection and keeps its labels, so product / help / release publication stay aligned on the same
    /// narrowed state.
    pub fn narrowing_disclosed(&self) -> bool {
        // Every declared narrowed rendering surface has a disclosure entry.
        for surface in &self.rendering_surfaces {
            if surface.is_narrowed()
                && !self
                    .narrowing_disclosures
                    .iter()
                    .any(|d| d.rendering_surface == *surface)
            {
                return false;
            }
        }
        // Every disclosure never silently drops and preserves labels on a narrowed surface.
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Whether the row models its family's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = family_primary_dimension(self.window_restore_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5WindowRestoreRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> WindowRestoreAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.trusted_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_truth_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return WindowRestoreAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            WindowRestoreAccessibilityStatus::NarrowedDisclosed
        } else {
            WindowRestoreAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == WINDOW_RESTORE_A11Y_ROW_RECORD_KIND
            && self.schema_version == WINDOW_RESTORE_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.window_restore_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} \
high_zoom={high_zoom} high_contrast={high_contrast} localization={localization} cli={cli} \
export={export} full_claim={full} effective_claim={effective} status={status}",
            family = self.window_restore_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            high_zoom = self.high_zoom_reach.as_str(),
            high_contrast = self.high_contrast_reach.as_str(),
            localization = self.localization_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_ready_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1186 window-restore accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowRestoreAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub structure_heavy_family_count: usize,
    pub all_structure_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_trusted_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_truth_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`WindowRestoreAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowRestoreAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<WindowRestoreAccessibilityRow>,
}

/// Checked-in M05-1186 window-restore accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowRestoreAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<WindowRestoreAccessibilityRow>,
    pub summary: WindowRestoreAccessibilitySummary,
}

impl WindowRestoreAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: WindowRestoreAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: WINDOW_RESTORE_A11Y_SCHEMA_VERSION,
            record_kind: WINDOW_RESTORE_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: WindowRestoreAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                structure_heavy_family_count: 0,
                all_structure_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_trusted_honesty_holds: false,
                all_export_summaries_preserve_meaning: false,
                all_truth_preserved: false,
                all_narrowing_disclosed: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                rendering_surface_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5WindowRestoreFamily> {
        self.rows.iter().map(|r| r.window_restore_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5WindowRestoreClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5WindowRestoreConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5WindowRestoreA11yClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5WindowRestoreConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> WindowRestoreAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5WindowRestoreConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let structure_heavy: Vec<&WindowRestoreAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_structure_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                WindowRestoreAccessibilityStatus::Parity => green += 1,
                WindowRestoreAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                WindowRestoreAccessibilityStatus::Stranded => red += 1,
            }
        }

        WindowRestoreAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            structure_heavy_family_count: structure_heavy.len(),
            all_structure_heavy_have_non_visual_fallback: structure_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(WindowRestoreAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(WindowRestoreAccessibilityRow::claim_is_honest),
            all_trusted_honesty_holds: self
                .rows
                .iter()
                .all(WindowRestoreAccessibilityRow::trusted_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(WindowRestoreAccessibilityRow::export_preserves_meaning),
            all_truth_preserved: self
                .rows
                .iter()
                .all(WindowRestoreAccessibilityRow::preserves_truth_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(WindowRestoreAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<WindowRestoreAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != WINDOW_RESTORE_A11Y_SCHEMA_VERSION {
            violations.push(WindowRestoreAccessibilityViolation::SchemaVersion {
                expected: WINDOW_RESTORE_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != WINDOW_RESTORE_A11Y_RECORD_KIND {
            violations.push(WindowRestoreAccessibilityViolation::RecordKind {
                expected: WINDOW_RESTORE_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(WindowRestoreAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(WindowRestoreAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.window_restore_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_shown_trusted())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(WindowRestoreAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    WindowRestoreAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.window_restore_family),
                    },
                );
            }

            // Each row must preserve every mandatory window-restore label.
            if !row.preserves_mandatory_labels() {
                violations.push(WindowRestoreAccessibilityViolation::MissingMandatoryLabel {
                    id: row.row_id.clone(),
                });
            }

            // A structure-heavy family must render a structured projection *and* a non-visual path.
            if row.is_structure_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5WindowRestoreFallbackModality::Structured)
            {
                violations.push(
                    WindowRestoreAccessibilityViolation::StructureHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: claim never over-asserts a trusted / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(WindowRestoreAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC / trusted honesty: an unconfirmed-session-replay / aged-out-display-recovery state never keeps
            // a trusted claim.
            if !row.trusted_honesty_holds() {
                violations.push(
                    WindowRestoreAccessibilityViolation::WeakStateShownAsTrusted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(WindowRestoreAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // AC: export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(
                    WindowRestoreAccessibilityViolation::ExportRequiresRawPayload {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: weakened states preserve workspace-authority / window-topology / restore-fidelity /
            // session-replay / display-recovery truth.
            if !row.preserves_truth_continuity() {
                violations.push(WindowRestoreAccessibilityViolation::TruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    WindowRestoreAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(WindowRestoreAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == WindowRestoreAccessibilityStatus::Stranded {
                violations.push(WindowRestoreAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5WindowRestoreFamily::ALL {
            if !seen_families.contains(&family) {
                violations
                    .push(WindowRestoreAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5WindowRestoreClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    WindowRestoreAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every condition state (the fully-qualified baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5WindowRestoreConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    WindowRestoreAccessibilityViolation::MissingConditionStateCoverage { state },
                );
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing spectrum
        // (trusted → … → display-recovery-unverified) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5WindowRestoreA11yClaim::ALL {
            if !effective.contains(&claim) {
                violations
                    .push(WindowRestoreAccessibilityViolation::MissingClaimTierCoverage { claim });
            }
        }

        // Trusted honesty must be proven with at least one unconfirmed-session-replay / aged-out-display-recovery
        // row in the packet, so the "cannot-prove never shown as trusted" guarantee is exercised end-to-end.
        if !has_unprovable_row {
            violations.push(WindowRestoreAccessibilityViolation::TrustedHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the restore-coordinator, shell, workspace-service,
        // session-service, diagnostics, docs/help, CLI-export, support-export, and product surfaces — so every
        // consumer surface is exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5WindowRestoreConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    WindowRestoreAccessibilityViolation::MissingConsumerSurfaceCoverage { surface },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(WindowRestoreAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("window-restore accessibility parity packet serializes"),
        ) {
            violations.push(WindowRestoreAccessibilityViolation::RawWindowRestoreMaterialInExport);
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
            .expect("window-restore accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,window_restore_family,keyboard_reach,screen_reader_reach,high_zoom_reach,high_contrast_reach,localization_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{high_zoom},{high_contrast},{localization},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.window_restore_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                high_zoom = row.high_zoom_reach.as_str(),
                high_contrast = row.high_contrast_reach.as_str(),
                localization = row.localization_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_ready_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, help, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Window-Restore Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5WindowRestoreFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.window_restore_family.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_ready_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in window-restore accessibility parity export.
pub fn current_m5_window_restore_a11y_export(
) -> Result<WindowRestoreAccessibilityPacket, WindowRestoreAccessibilityArtifactError> {
    let packet: WindowRestoreAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-window-restore-accessibility-parity/support_export.json"
    )))
    .map_err(WindowRestoreAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(WindowRestoreAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in window-restore accessibility parity export.
#[derive(Debug)]
pub enum WindowRestoreAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<WindowRestoreAccessibilityViolation>),
}

impl fmt::Display for WindowRestoreAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "window-restore accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "window-restore accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for WindowRestoreAccessibilityArtifactError {}

/// Validation failure for M05-1186 window-restore accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowRestoreAccessibilityViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    MissingPrimaryDimension {
        id: String,
        dimension: M5WindowRestoreClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    StructureHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    WeakStateShownAsTrusted {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresRawPayload {
        id: String,
    },
    TruthDropped {
        id: String,
    },
    NarrowingDropsContextSilently {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    StrandedRow {
        id: String,
    },
    MissingFamilyCoverage {
        family: M5WindowRestoreFamily,
    },
    MissingDimensionCoverage {
        dimension: M5WindowRestoreClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5WindowRestoreConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5WindowRestoreA11yClaim,
    },
    TrustedHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5WindowRestoreConsumerSurface,
    },
    SummaryMismatch,
    RawWindowRestoreMaterialInExport,
}

impl WindowRestoreAccessibilityViolation {
    /// Stable token for CLI / support handoff.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaVersion { .. } => "schema_version",
            Self::RecordKind { .. } => "record_kind",
            Self::MissingIdentity => "missing_identity",
            Self::DuplicateId { .. } => "duplicate_id",
            Self::IncompleteRow { .. } => "incomplete_row",
            Self::MissingPrimaryDimension { .. } => "missing_primary_dimension",
            Self::MissingMandatoryLabel { .. } => "missing_mandatory_label",
            Self::StructureHeavyMissingStructured { .. } => "structure_heavy_missing_structured",
            Self::ClaimOverAsserted { .. } => "claim_over_asserted",
            Self::WeakStateShownAsTrusted { .. } => "weak_state_shown_as_trusted",
            Self::AssistiveTechStranded { .. } => "assistive_tech_stranded",
            Self::ExportRequiresRawPayload { .. } => "export_requires_raw_payload",
            Self::TruthDropped { .. } => "truth_dropped",
            Self::NarrowingDropsContextSilently { .. } => "narrowing_drops_context_silently",
            Self::MissingConsumerParity { .. } => "missing_consumer_parity",
            Self::StrandedRow { .. } => "stranded_row",
            Self::MissingFamilyCoverage { .. } => "missing_family_coverage",
            Self::MissingDimensionCoverage { .. } => "missing_dimension_coverage",
            Self::MissingConditionStateCoverage { .. } => "missing_condition_state_coverage",
            Self::MissingClaimTierCoverage { .. } => "missing_claim_tier_coverage",
            Self::TrustedHonestyUnproven => "trusted_honesty_unproven",
            Self::MissingConsumerSurfaceCoverage { .. } => "missing_consumer_surface_coverage",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawWindowRestoreMaterialInExport => "raw_window_restore_material_in_export",
        }
    }
}

impl fmt::Display for WindowRestoreAccessibilityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete accessibility row: {id}"),
            Self::MissingPrimaryDimension { id, dimension } => {
                write!(
                    f,
                    "row {id} does not model its family's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory window-restore label")
            }
            Self::StructureHeavyMissingStructured { id } => {
                write!(
                    f,
                    "structure-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts a trusted / reviewable surface for a weakened one, or narrows spuriously"
                )
            }
            Self::WeakStateShownAsTrusted { id } => {
                write!(
                    f,
                    "row {id} shows an unconfirmed-session-replay / aged-out-display-recovery state as a trusted restore surface"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / high-zoom / high-contrast / localization / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresRawPayload { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without leaking a raw payload"
                )
            }
            Self::TruthDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve workspace-authority / window-topology / restore-fidelity / session-replay / display-recovery truth across narrowing"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing it"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "window-restore family {family:?} is not certified in the packet"
                )
            }
            Self::MissingDimensionCoverage { dimension } => {
                write!(
                    f,
                    "claim dimension {} is not exercised in the packet",
                    dimension.as_str()
                )
            }
            Self::MissingConditionStateCoverage { state } => {
                write!(
                    f,
                    "condition state {} is not exercised in the packet",
                    state.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::TrustedHonestyUnproven => {
                write!(
                    f,
                    "no unconfirmed-session-replay / aged-out-display-recovery row is present to prove the trusted-honesty guarantee"
                )
            }
            Self::MissingConsumerSurfaceCoverage { surface } => {
                write!(
                    f,
                    "consumer surface {} does not ingest any row in the packet",
                    surface.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawWindowRestoreMaterialInExport => {
                write!(f, "export contains raw window-restore material")
            }
        }
    }
}

impl Error for WindowRestoreAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "blocked"
            | "unresolved"
            | "partial"
            | "stale"
            | "incomplete"
            | "not comparable"
            | "restricted"
            | "collapsed"
            | "ellipsis"
            | "mixed"
            | "expired"
            | "inferred"
            | "unverified"
            | "trusted"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Mirrors the frozen window-restore
/// matrix's own boundary policy (see [`crate::m5_window_restore_matrix`]): raw secret values and private
/// endpoints stay outside the export boundary, so this heuristic targets raw credential blobs, bearer tokens,
/// key blocks, and endpoint URLs while the window-restore grammar carries only typed class tokens and opaque
/// refs.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The canonical packet id for the checked-in stable export.
pub const WINDOW_RESTORE_A11Y_PACKET_ID: &str =
    "m5-window-restore-accessibility-parity:stable:0001";

/// Builds the canonical, checked-in window-restore accessibility parity packet. This is the one source of
/// truth shared by the tests and the on-disk support export so both stay byte-aligned.
pub fn seeded_m5_window_restore_a11y_packet() -> WindowRestoreAccessibilityPacket {
    WindowRestoreAccessibilityPacket::new(WindowRestoreAccessibilityPacketInput {
        packet_id: WINDOW_RESTORE_A11Y_PACKET_ID.to_owned(),
        as_of: "2026-07-14T00:00:00Z".to_owned(),
        matrix_ref: WINDOW_RESTORE_A11Y_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:window-restore-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5WindowRestoreRequiredLabel> {
    M5WindowRestoreRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> WindowRestoreCopyExportParity {
    WindowRestoreCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5WindowRestoreClaimDimension,
    state: M5WindowRestoreConditionState,
) -> WindowRestoreClaimConditionEntry {
    WindowRestoreClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and the general product
/// UI — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5WindowRestoreConsumerSurface]) -> Vec<M5WindowRestoreConsumerSurface> {
    let mut out = vec![
        M5WindowRestoreConsumerSurface::SupportExport,
        M5WindowRestoreConsumerSurface::ProductUi,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps full label
/// and summary parity on the narrower surfaces; a narrowed row discloses the reduced projection it drops
/// there.
fn surface_disclosures(
    labels: &[&str],
    state: WindowRestoreNarrowingDisclosureState,
) -> Vec<WindowRestoreRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        WindowRestoreRenderingNarrowingDisclosure {
            rendering_surface: M5WindowRestoreRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["shell_chrome_pointer_affordance".to_owned()],
        },
        WindowRestoreRenderingNarrowingDisclosure {
            rendering_surface: M5WindowRestoreRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_display_topology_remap_transition".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<WindowRestoreRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        WindowRestoreNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced projection while
/// preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<WindowRestoreRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        WindowRestoreNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5WindowRestoreRenderingSurface> {
    vec![
        M5WindowRestoreRenderingSurface::DesktopFull,
        M5WindowRestoreRenderingSurface::CliHeadless,
        M5WindowRestoreRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5WindowRestoreFallbackModality> {
    vec![
        M5WindowRestoreFallbackModality::List,
        M5WindowRestoreFallbackModality::Textual,
        M5WindowRestoreFallbackModality::Cli,
    ]
}

fn structured_modalities() -> Vec<M5WindowRestoreFallbackModality> {
    vec![
        M5WindowRestoreFallbackModality::Structured,
        M5WindowRestoreFallbackModality::List,
        M5WindowRestoreFallbackModality::Textual,
        M5WindowRestoreFallbackModality::Cli,
    ]
}

const REACHABLE: WindowRestoreNonVisualReachState =
    WindowRestoreNonVisualReachState::ReachableAndLabeled;
const REDUCED: WindowRestoreNonVisualReachState =
    WindowRestoreNonVisualReachState::DisclosedReducedButReachable;

fn seeded_rows() -> Vec<WindowRestoreAccessibilityRow> {
    vec![
        // Shared workspace authority (one authority backs multiple windows, selections and focus stay
        // window-local) — the shared-workspace-authority family keeps one authority backing many windows while
        // window-local selection and focus never clobber shared state, so it is a trusted restore surface
        // reachable on every surface with no narrowing (green). Not structure-heavy: it exposes a flat list /
        // textual / CLI path.
        WindowRestoreAccessibilityRow {
            record_kind: WINDOW_RESTORE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WINDOW_RESTORE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:shared-workspace-authority-window-local-preserved".to_owned(),
            window_restore_family: M5WindowRestoreFamily::SharedWorkspaceAuthority,
            source_family_schema_ref: M5WindowRestoreFamily::SharedWorkspaceAuthority
                .canonical_domain_schema_ref()
                .to_owned(),
            window_restore_context_ref: "restore-coordinator:shared-workspace-authority:0001"
                .to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            localization_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: WindowRestoreExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:shared-workspace-authority-window-local-preserved:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "window_restore_identity",
                "semantic_role",
                "registry_reference",
                "workspace_authority",
            ]),
            full_ready_claim: M5WindowRestoreA11yClaim::TrustedRestoreSurface,
            claim_conditions: vec![condition(
                M5WindowRestoreClaimDimension::SharedAuthorityClarity,
                M5WindowRestoreConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "window_restore_identity",
                "semantic_role",
                "workspace_authority",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WindowRestoreConsumerSurface::RestoreCoordinator,
                M5WindowRestoreConsumerSurface::ShellUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.6 — Multi-window shared workspace authority".to_owned(),
                WINDOW_RESTORE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-14T00:00:00Z".to_owned(),
            evidence_refs: ev("shared-workspace-authority-window-local-preserved"),
        },
        // Window-local topology (versioned, attributable pane trees scoped to their window) — the
        // window-local-topology family exposes its versioned, attributable pane topology as an inspectable
        // read-only reference an operator can review, so it is a self-sufficient reviewable restore surface, but
        // its narrower non-visual traversal discloses a reduced high-zoom reflow walk of the dense pane-tree
        // table (yellow). Structure-heavy: no — it exposes a flat list / textual path.
        WindowRestoreAccessibilityRow {
            record_kind: WINDOW_RESTORE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WINDOW_RESTORE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:window-local-topology-versioned-attributable".to_owned(),
            window_restore_family: M5WindowRestoreFamily::WindowLocalTopology,
            source_family_schema_ref: M5WindowRestoreFamily::WindowLocalTopology
                .canonical_domain_schema_ref()
                .to_owned(),
            window_restore_context_ref: "workspace-service:window-local-topology:0002".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REDUCED,
            high_contrast_reach: REACHABLE,
            localization_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: WindowRestoreExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:window-local-topology-versioned-attributable:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "window_restore_identity",
                "semantic_role",
                "registry_reference",
                "window_topology",
            ]),
            full_ready_claim: M5WindowRestoreA11yClaim::ReviewableRestoreSurface,
            claim_conditions: vec![condition(
                M5WindowRestoreClaimDimension::WindowLocalIsolationClarity,
                M5WindowRestoreConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "window_restore_identity",
                "semantic_role",
                "window_topology",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WindowRestoreConsumerSurface::WorkspaceService,
                M5WindowRestoreConsumerSurface::SessionService,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.6 — Window-local pane topology".to_owned(),
                WINDOW_RESTORE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-14T00:00:00Z".to_owned(),
            evidence_refs: ev("window-local-topology-versioned-attributable"),
        },
        // Skeleton-first restore (layout-skeleton proof partially disclosed) — the skeleton-first-restore
        // family's layout-skeleton proof can only be partially disclosed, so it auto-narrows to a
        // layout-skeleton-disclosed projection that discloses the partial skeleton proof alongside the
        // last-known pane-role placeholders, never a hydrated layout shown as exact when its skeleton proof is
        // incomplete (yellow). Its localized traversal narrows the localization path to a disclosed reduction.
        // Structure-heavy: its restore-fidelity table binds to a flat list / textual path. A partial skeleton
        // disclosure is an honest disclosed-absence operation, not a trusted overstatement.
        WindowRestoreAccessibilityRow {
            record_kind: WINDOW_RESTORE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WINDOW_RESTORE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:skeleton-first-restore-layout-skeleton-disclosed-partial".to_owned(),
            window_restore_family: M5WindowRestoreFamily::SkeletonFirstRestore,
            source_family_schema_ref: M5WindowRestoreFamily::SkeletonFirstRestore
                .canonical_domain_schema_ref()
                .to_owned(),
            window_restore_context_ref: "diagnostics:skeleton-first-restore:0003".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            localization_reach: REDUCED,
            cli_reach: REACHABLE,
            export_summary: WindowRestoreExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref:
                "summary:skeleton-first-restore-layout-skeleton-disclosed-partial:a11y".to_owned(),
            copy_export: copy_export(&[
                "window_restore_identity",
                "semantic_role",
                "registry_reference",
                "restore_fidelity_class",
            ]),
            full_ready_claim: M5WindowRestoreA11yClaim::TrustedRestoreSurface,
            claim_conditions: vec![condition(
                M5WindowRestoreClaimDimension::SkeletonFidelityClarity,
                M5WindowRestoreConditionState::LayoutSkeletonDisclosedPartial,
            )],
            claim_narrow: Some(WindowRestoreClaimAutoNarrow {
                narrowed_to: M5WindowRestoreA11yClaim::LayoutSkeletonDisclosedProjection,
                binding_dimension: M5WindowRestoreClaimDimension::SkeletonFidelityClarity,
                trigger: M5WindowRestoreDowngradeTrigger::ProofStale,
                narrowed_label:
                    "This skeleton-first restore can only disclose a partial layout-skeleton proof — shown as a layout-skeleton-disclosed projection that discloses the partial skeleton proof alongside the last-known pane-role placeholders, never presenting a hydrated layout as exact when its skeleton proof is incomplete"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "window_restore_identity",
                "semantic_role",
                "restore_fidelity_class",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WindowRestoreConsumerSurface::Diagnostics,
                M5WindowRestoreConsumerSurface::CliExport,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.10 — Skeleton-first / hydrate-second session restore".to_owned(),
                WINDOW_RESTORE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-14T00:00:00Z".to_owned(),
            evidence_refs: ev("skeleton-first-restore-layout-skeleton-disclosed-partial"),
        },
        // No-rerun session hydration (session-replay fence unconfirmed) — the no-rerun-session-hydration
        // family's session-replay fence cannot be confirmed, so it auto-narrows to a session-replay-unverified
        // projection that keeps the last-known reopened-versus-rerun posture explicit, never a session shown as
        // reattached when it may have silently reran (yellow). Its forced-colors response narrows the
        // high-contrast path to a disclosed reduction. Structure-heavy: its session-replay table binds to a flat
        // list / textual path.
        WindowRestoreAccessibilityRow {
            record_kind: WINDOW_RESTORE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WINDOW_RESTORE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:no-rerun-session-replay-unconfirmed".to_owned(),
            window_restore_family: M5WindowRestoreFamily::NoRerunSessionHydration,
            source_family_schema_ref: M5WindowRestoreFamily::NoRerunSessionHydration
                .canonical_domain_schema_ref()
                .to_owned(),
            window_restore_context_ref: "session-service:no-rerun-session-hydration:0004".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REDUCED,
            localization_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: WindowRestoreExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:no-rerun-session-replay-unconfirmed:a11y".to_owned(),
            copy_export: copy_export(&[
                "window_restore_identity",
                "semantic_role",
                "registry_reference",
                "session_replay_fence",
            ]),
            full_ready_claim: M5WindowRestoreA11yClaim::TrustedRestoreSurface,
            claim_conditions: vec![condition(
                M5WindowRestoreClaimDimension::SessionReplayFenceClarity,
                M5WindowRestoreConditionState::SessionReplayUnconfirmed,
            )],
            claim_narrow: Some(WindowRestoreClaimAutoNarrow {
                narrowed_to: M5WindowRestoreA11yClaim::SessionReplayUnverifiedProjection,
                binding_dimension: M5WindowRestoreClaimDimension::SessionReplayFenceClarity,
                trigger:
                    M5WindowRestoreDowngradeTrigger::OverclaimedRestoreFidelityWhenOnlyContextOrEvidenceReopened,
                narrowed_label:
                    "This no-rerun session hydration cannot confirm that its session-replay fence held — shown as a session-replay-unverified projection that keeps the last-known reopened-versus-rerun posture explicit, never presenting a session-scoped tool as reattached when it may have silently reran"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "window_restore_identity",
                "semantic_role",
                "session_replay_fence",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WindowRestoreConsumerSurface::DocsHelp,
                M5WindowRestoreConsumerSurface::Diagnostics,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.10 — No-rerun session hydration".to_owned(),
                WINDOW_RESTORE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-14T00:00:00Z".to_owned(),
            evidence_refs: ev("no-rerun-session-replay-unconfirmed"),
        },
        // Display-topology recovery (remap / recovery evidence aged out or policy-blocked) — the
        // display-topology-recovery family's remap / recovery evidence has aged out or is policy-blocked, so it
        // auto-narrows to a display-recovery-unverified projection that keeps the last-known monitor-affinity /
        // bounds state explicit, never a window shown as recovered on-screen when its remap evidence has aged
        // out or become policy-blocked (yellow). Structure-heavy: its remap-history table binds to a flat list /
        // textual path.
        WindowRestoreAccessibilityRow {
            record_kind: WINDOW_RESTORE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WINDOW_RESTORE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:display-topology-recovery-unconfirmed".to_owned(),
            window_restore_family: M5WindowRestoreFamily::DisplayTopologyRecovery,
            source_family_schema_ref: M5WindowRestoreFamily::DisplayTopologyRecovery
                .canonical_domain_schema_ref()
                .to_owned(),
            window_restore_context_ref: "restore-coordinator:display-topology-recovery:0005"
                .to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            localization_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: WindowRestoreExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:display-topology-recovery-unconfirmed:a11y".to_owned(),
            copy_export: copy_export(&[
                "window_restore_identity",
                "semantic_role",
                "registry_reference",
                "display_affinity",
            ]),
            full_ready_claim: M5WindowRestoreA11yClaim::TrustedRestoreSurface,
            claim_conditions: vec![condition(
                M5WindowRestoreClaimDimension::DisplayRecoveryClarity,
                M5WindowRestoreConditionState::DisplayRecoveryUnconfirmed,
            )],
            claim_narrow: Some(WindowRestoreClaimAutoNarrow {
                narrowed_to: M5WindowRestoreA11yClaim::DisplayRecoveryUnverifiedProjection,
                binding_dimension: M5WindowRestoreClaimDimension::DisplayRecoveryClarity,
                trigger:
                    M5WindowRestoreDowngradeTrigger::OverclaimedRestoreFidelityWhenOnlyContextOrEvidenceReopened,
                narrowed_label:
                    "This display-topology recovery cannot confirm current remap or recovery evidence — shown as a display-recovery-unverified projection that keeps the last-known monitor-affinity and bounds state explicit, never presenting a window as recovered on-screen when its remap evidence has aged out or become policy-blocked"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "window_restore_identity",
                "semantic_role",
                "display_affinity",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WindowRestoreConsumerSurface::RestoreCoordinator,
                M5WindowRestoreConsumerSurface::ShellUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.14 — Display-topology recovery / restore provenance".to_owned(),
                WINDOW_RESTORE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-14T00:00:00Z".to_owned(),
            evidence_refs: ev("display-topology-recovery-unconfirmed"),
        },
    ]
}
