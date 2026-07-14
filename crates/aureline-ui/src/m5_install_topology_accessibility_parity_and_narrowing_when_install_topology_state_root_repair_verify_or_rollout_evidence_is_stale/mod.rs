//! Keyboard / screen-reader / high-zoom / high-contrast / localization / CLI / export parity, and honest
//! automatic claim narrowing for the M5 per-user-managed / per-machine-managed / side-by-side-stable-preview /
//! portable-mode / offline-air-gap-bundle install-topology families.
//!
//! This module is the M05-1178 accessibility-localization-support-export parity and auto-narrowing capstone
//! over the frozen M5 install-topology matrix ([`crate::m5_install_topology_matrix`]). Where the freeze matrix
//! defines the five governed delivery-topology families, and the 1173-1176 implementation lanes resolve their
//! per-surface install-mode, updater-ownership, state-root, channel-isolation, managed-operation, and
//! rollback truth, this lane certifies — per delivery-topology family — that install-mode / updater-owner /
//! state-root / repair-verify / rollout-ring / rollback claims stay **keyboard-reachable,
//! screen-reader-announced, high-zoom-legible, high-contrast-safe, localization-safe, CLI/export-safe, and
//! self-narrowing** rather than presenting an updater owner that only lives in an installer screenshot, a
//! state root that is claimed isolated without proof, a repair/verify posture shown as covered when it was
//! never run, or a rollout ring shown as promoted when its evidence has aged out as still a stable, trusted
//! delivery surface:
//!
//! - **Keyboard / screen-reader / high-zoom / high-contrast / localization / CLI reach.** Every family
//!   exposes a keyboard-reachable, screen-reader-announced, high-zoom-reflowing, high-contrast-legible,
//!   localization-safe, and CLI/headless-reachable path into the same install-topology identity, semantic
//!   role, registry reference, install mode, state root, and rollback target the rendered surface shows —
//!   never a pointer-only affordance hidden in installer chrome, an unlabeled control, or an updater owner /
//!   state root that only lives in a screenshot and strands assistive-tech, localized, or headless-CLI users.
//!   Structure-heavy families (the side-by-side channel-isolation table, the portable root-inventory table,
//!   the offline artifact-graph rollback table) additionally bind their structured layout to a flat list /
//!   textual / CLI path.
//! - **Export parity.** The support / release / CLI export reconstructs each family's meaning from typed
//!   tokens and opaque refs **without a raw payload**, preserving the same install-topology identity, semantic
//!   role, registry reference, install mode, state root, and rollback target shown in-product so support,
//!   help, and release proof can reconstruct which delivery-topology truth class was active without leaking a
//!   raw secret blob, a machine-specific sensitive path, or an installer-only screenshot.
//! - **Honest auto-narrowing.** When a side-by-side family's state-boundary proof can only be partially
//!   disclosed, a portable / offline family's repair/verify coverage cannot be confirmed, or a family's
//!   rollout-ring evidence has aged out or is policy-blocked, the family's claim auto-narrows from
//!   `trusted_delivery_surface` / `reviewable_delivery_surface` to a state-boundary-disclosed /
//!   repair-verify-unverified / rollout-evidence-unverified projection, discloses the narrowing with a precise
//!   trigger and binding dimension, and preserves the canonical install-topology identity / last-known
//!   registry reference. The underlying install-mode / state-root / repair-verify / rollout-ring / rollback
//!   truth is never dropped opaquely. A family with every dimension intact must NOT carry a spurious
//!   narrowing, and a state-spilling / evidence-aged / policy-blocked state can never keep a trusted, stable
//!   delivery claim — install-topology meaning is never conveyed by an installer-chrome-only affordance, a
//!   mislabeled screenshot, or an unlabeled control alone.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the updater service, the shell / About
//!   UI, the diagnostics surface, the admin surface, the installer, the docs / help surface, the CLI export,
//!   the support export, and the product UI so product, help, and release publication stay aligned on
//!   downgrade behavior rather than drifting in copy — a trusted-looking delivery surface can never outrun the
//!   install-mode / state-root / repair-verify / rollout-ring evidence it is being viewed away from.
//!
//! Each [`InstallTopologyAccessibilityRow`] keys on one
//! [`crate::m5_install_topology_matrix::M5InstallTopologyFamily`] and reuses that frozen family vocabulary plus
//! the frozen [`M5InstallTopologyRequiredLabel`], [`M5InstallTopologyDowngradeTrigger`], and shared
//! [`M5InstallTopologyConsumerSurface`] consumer surfaces rather than minting parallel synonyms, so the
//! certified labels stay byte-identical to the matrix and the sibling install-topology packets.
//!
//! The packet is metadata-only: raw secret blobs, machine-specific sensitive paths, plaintext payloads, and
//! endpoint refs never cross this boundary; the packet carries only typed class tokens, opaque install-topology
//! refs, booleans, and controlled labels so support, release, and diagnostics exports can reconstruct exactly
//! which delivery-topology truth class was active without leaking sensitive material or a raw payload.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen install-topology vocabulary — the capstone certifies the freeze matrix's families, required
// labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::m5_install_topology_matrix::{
    M5InstallTopologyConsumerSurface, M5InstallTopologyDowngradeTrigger, M5InstallTopologyFamily,
    M5InstallTopologyRequiredLabel, M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
};

/// Schema version stamped on the M05-1178 install-topology accessibility parity packet.
pub const INSTALL_TOPOLOGY_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`InstallTopologyAccessibilityPacket`].
pub const INSTALL_TOPOLOGY_A11Y_RECORD_KIND: &str =
    "m5_install_topology_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`InstallTopologyAccessibilityRow`].
pub const INSTALL_TOPOLOGY_A11Y_ROW_RECORD_KIND: &str =
    "m5_install_topology_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const INSTALL_TOPOLOGY_A11Y_SCHEMA_REF: &str =
    "schemas/install/m5-install-topology-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const INSTALL_TOPOLOGY_A11Y_DOC_REF: &str =
    "docs/install/m5_install_topology_accessibility_parity.md";

/// Repo-relative path of the frozen install-topology matrix this lane certifies.
pub const INSTALL_TOPOLOGY_A11Y_MATRIX_REF: &str = M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const INSTALL_TOPOLOGY_A11Y_FIXTURE_DIR: &str =
    "fixtures/install/m5-install-topology-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const INSTALL_TOPOLOGY_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-install-topology-accessibility-parity/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const INSTALL_TOPOLOGY_A11Y_CSV_REF: &str =
    "artifacts/release/m5-install-topology-accessibility-parity/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const INSTALL_TOPOLOGY_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-install-topology-accessibility-parity.md";

/// The reusable install-topology families that render a dense, structured surface (the side-by-side
/// channel-isolation table, the portable root-inventory table, the offline artifact-graph rollback table) and
/// therefore MUST bind their structured layout to an equivalent flat list / textual / CLI path so the
/// structure is navigable non-visually.
const fn family_is_structure_heavy(family: M5InstallTopologyFamily) -> bool {
    matches!(
        family,
        M5InstallTopologyFamily::SideBySideStablePreview
            | M5InstallTopologyFamily::PortableMode
            | M5InstallTopologyFamily::OfflineAirgapBundle
    )
}

/// The delivery-topology-truth dimension whose weakening a family primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5InstallTopologyFamily,
) -> M5InstallTopologyClaimDimension {
    match family {
        M5InstallTopologyFamily::PerUserManaged => {
            M5InstallTopologyClaimDimension::InstallOwnershipClarity
        }
        M5InstallTopologyFamily::PerMachineManaged => {
            M5InstallTopologyClaimDimension::PolicyControlClarity
        }
        M5InstallTopologyFamily::SideBySideStablePreview => {
            M5InstallTopologyClaimDimension::StateBoundaryClarity
        }
        M5InstallTopologyFamily::PortableMode => {
            M5InstallTopologyClaimDimension::RepairVerifyClarity
        }
        M5InstallTopologyFamily::OfflineAirgapBundle => {
            M5InstallTopologyClaimDimension::RolloutEvidenceClarity
        }
    }
}

/// A rendered fallback modality for an install-topology family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallTopologyFallbackModality {
    /// A rich, structured (channel-isolation / root-inventory / rollback table) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / label-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5InstallTopologyFallbackModality {
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

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same install-topology
/// family may render at desktop-full capability or narrow to a companion, read-only browser, headless CLI,
/// docs export, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallTopologyRenderingSurface {
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

impl M5InstallTopologyRenderingSurface {
    /// Returns true when the surface narrows the install-topology family below the desktop full-capability
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

/// Keyboard / screen-reader / high-zoom / high-contrast / localization / CLI reach for an install-topology
/// family's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallTopologyNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// An installer-chrome-only / pointer-only / view-only surface that traps keyboard / assistive-tech /
    /// localized / headless-CLI users (red).
    ViewOnlyTrap,
}

impl InstallTopologyNonVisualReachState {
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

/// Whether an export-safe summary preserves the install-topology meaning without leaking a raw payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallTopologyExportSummaryState {
    /// The install-topology meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl InstallTopologyExportSummaryState {
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

/// Whether a narrower rendering surface discloses its reduced install-topology projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallTopologyNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced projection, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Install-topology state or tokens dropped without disclosure (red).
    SilentlyDropped,
}

impl InstallTopologyNarrowingDisclosureState {
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

/// The install-topology claim ceiling a family asserts: how strong a trusted / stable posture it lets a surface
/// present. Auto-narrowing lowers this ceiling when a state-boundary / repair-verify / rollout-evidence
/// dimension weakens so a partially-disclosed state boundary, an unconfirmed repair/verify coverage, or an
/// aged-out / policy-blocked rollout ring can never keep an old `TrustedDeliverySurface` or
/// `ReviewableDeliverySurface` label — install-topology meaning is never conveyed by an installer-chrome-only
/// affordance, a mislabeled screenshot, or an unlabeled control alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallTopologyA11yClaim {
    /// Trusted delivery surface: a fully current, registry-bound, ownership-inspectable, state-isolated,
    /// repair/verify-covered, rollout-evidenced delivery topology — the strongest claim, an install-topology
    /// surface Aureline can present as exactly trusted and stable right now.
    TrustedDeliverySurface,
    /// Reviewable delivery surface: a self-sufficient, inspectable read-only install-topology projection (a
    /// static per-machine policy-control / registry reference an admin can inspect) that is not itself an
    /// authoritative, live-resolving surface.
    ReviewableDeliverySurface,
    /// State-boundary-disclosed projection: a side-by-side stable/preview state-boundary proof can only be
    /// partially disclosed; the family stays a state-boundary-disclosed projection that discloses the partial
    /// boundary proof alongside the last-known isolated state root, never a shared namespace shown as isolated
    /// when its boundary proof is incomplete.
    StateBoundaryDisclosedProjection,
    /// Repair-verify-unverified projection: a portable / offline family's repair/verify coverage cannot be
    /// confirmed; the family stays a repair-verify-unverified projection that keeps the last-known repair/verify
    /// posture explicit, never a repair/verify shown as covered when it may never have run.
    RepairVerifyUnverifiedProjection,
    /// Rollout-evidence-unverified projection: a family's rollout-ring promotion / rollback evidence has aged
    /// out or is policy-blocked; the family stays a rollout-evidence-unverified projection that keeps the
    /// last-known ring state explicit, never a ring shown as promoted or a claim shown as published when its
    /// evidence has aged out or become policy-blocked.
    RolloutEvidenceUnverifiedProjection,
}

impl M5InstallTopologyA11yClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 5] = [
        Self::TrustedDeliverySurface,
        Self::ReviewableDeliverySurface,
        Self::StateBoundaryDisclosedProjection,
        Self::RepairVerifyUnverifiedProjection,
        Self::RolloutEvidenceUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::TrustedDeliverySurface => 4,
            Self::ReviewableDeliverySurface => 3,
            Self::StateBoundaryDisclosedProjection => 2,
            Self::RepairVerifyUnverifiedProjection => 1,
            Self::RolloutEvidenceUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully trusted, stable delivery surface.
    pub const fn asserts_trusted_surface(self) -> bool {
        matches!(self, Self::TrustedDeliverySurface)
    }

    /// Returns true when this claim asserts a fully self-sufficient (trusted or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::TrustedDeliverySurface | Self::ReviewableDeliverySurface
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedDeliverySurface => "trusted_delivery_surface",
            Self::ReviewableDeliverySurface => "reviewable_delivery_surface",
            Self::StateBoundaryDisclosedProjection => "state_boundary_disclosed_projection",
            Self::RepairVerifyUnverifiedProjection => "repair_verify_unverified_projection",
            Self::RolloutEvidenceUnverifiedProjection => "rollout_evidence_unverified_projection",
        }
    }
}

/// The install-ownership / policy-control / state-boundary / repair-verify / rollout-evidence dimension whose
/// state governs how far an install-topology family may claim to be a fully trusted, stable delivery surface.
/// The dimensions map 1:1 to the five frozen delivery-topology families so every family carries an honest
/// narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallTopologyClaimDimension {
    /// Install-ownership clarity: is the install mode and per-user updater ownership inspectable rather than
    /// hidden (per-user-managed)?
    InstallOwnershipClarity,
    /// Policy-control clarity: is admin / machine policy control and updater ownership an inspectable
    /// reviewable surface rather than hidden in a managed flow (per-machine-managed)?
    PolicyControlClarity,
    /// State-boundary clarity: do side-by-side stable and preview channels prove isolated state roots rather
    /// than reusing a stable namespace without handoff (side-by-side-stable-preview)?
    StateBoundaryClarity,
    /// Repair-verify clarity: does the portable / offline repair/verify coverage stay confirmed rather than
    /// shown as covered when it never ran (portable-mode)?
    RepairVerifyClarity,
    /// Rollout-evidence clarity: does the rollout-ring promotion / rollback evidence stay current rather than
    /// aging out or becoming policy-blocked (offline-air-gap-bundle)?
    RolloutEvidenceClarity,
}

impl M5InstallTopologyClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::InstallOwnershipClarity,
        Self::PolicyControlClarity,
        Self::StateBoundaryClarity,
        Self::RepairVerifyClarity,
        Self::RolloutEvidenceClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstallOwnershipClarity => "install_ownership_clarity",
            Self::PolicyControlClarity => "policy_control_clarity",
            Self::StateBoundaryClarity => "state_boundary_clarity",
            Self::RepairVerifyClarity => "repair_verify_clarity",
            Self::RolloutEvidenceClarity => "rollout_evidence_clarity",
        }
    }
}

/// The observed condition of one delivery-topology dimension. Anything weaker than [`Self::FullyQualified`]
/// imposes a narrowing ceiling on the family's claim. The unconfirmed states the lane must auto-narrow on as
/// *weakened evidence* — an unconfirmed repair/verify coverage and an aged-out / policy-blocked rollout-ring
/// evidence — are the states that [`Self::cannot_be_shown_trusted`] flags. A partially-disclosed state boundary
/// is an honest disclosed-absence operation (a partial boundary proof shown honestly with the last-known
/// isolated state root), not a truth overstatement, so it is deliberately excluded there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallTopologyConditionState {
    /// Fully current, registry-bound, ownership-inspectable, state-isolated, repair/verify-covered,
    /// rollout-evidenced — imposes no ceiling.
    FullyQualified,
    /// The side-by-side stable/preview state-boundary proof can only be partially disclosed — claim drops to a
    /// state-boundary-disclosed projection.
    StateBoundaryDisclosedPartial,
    /// The portable / offline repair/verify coverage cannot be confirmed — claim drops to a
    /// repair-verify-unverified projection.
    RepairVerifyUnconfirmed,
    /// The rollout-ring promotion / rollback evidence has aged out or is policy-blocked — claim drops to a
    /// rollout-evidence-unverified projection.
    RolloutEvidenceUnconfirmed,
}

impl M5InstallTopologyConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullyQualified,
        Self::StateBoundaryDisclosedPartial,
        Self::RepairVerifyUnconfirmed,
        Self::RolloutEvidenceUnconfirmed,
    ];

    /// Returns true when the dimension is weaker than fully qualified and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FullyQualified)
    }

    /// Returns true when the condition reflects weakened evidence that cannot be shown as a fully trusted,
    /// stable delivery surface and must never be shown as such. A partially-disclosed state boundary is an
    /// honest disclosed-absence operation (a partial boundary proof shown honestly with the last-known isolated
    /// state root), not a truth overstatement, so it is deliberately excluded here.
    pub const fn cannot_be_shown_trusted(self) -> bool {
        matches!(
            self,
            Self::RepairVerifyUnconfirmed | Self::RolloutEvidenceUnconfirmed
        )
    }

    /// The strongest claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5InstallTopologyA11yClaim {
        match self {
            Self::FullyQualified => M5InstallTopologyA11yClaim::TrustedDeliverySurface,
            Self::StateBoundaryDisclosedPartial => {
                M5InstallTopologyA11yClaim::StateBoundaryDisclosedProjection
            }
            Self::RepairVerifyUnconfirmed => {
                M5InstallTopologyA11yClaim::RepairVerifyUnverifiedProjection
            }
            Self::RolloutEvidenceUnconfirmed => {
                M5InstallTopologyA11yClaim::RolloutEvidenceUnverifiedProjection
            }
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each state maps
    /// to the on-topic frozen trigger the freeze matrix already governs, so the certified reason stays
    /// byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5InstallTopologyDowngradeTrigger {
        match self {
            // The fully-qualified baseline never narrows; kept for exhaustiveness.
            Self::FullyQualified => M5InstallTopologyDowngradeTrigger::ProofStale,
            Self::StateBoundaryDisclosedPartial => M5InstallTopologyDowngradeTrigger::ProofStale,
            Self::RepairVerifyUnconfirmed => {
                M5InstallTopologyDowngradeTrigger::DeploymentClaimOutpacedRingOrRepairVerifyEvidence
            }
            Self::RolloutEvidenceUnconfirmed => {
                M5InstallTopologyDowngradeTrigger::DeploymentClaimOutpacedRingOrRepairVerifyEvidence
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyQualified => "fully_qualified",
            Self::StateBoundaryDisclosedPartial => "state_boundary_disclosed_partial",
            Self::RepairVerifyUnconfirmed => "repair_verify_unconfirmed",
            Self::RolloutEvidenceUnconfirmed => "rollout_evidence_unconfirmed",
        }
    }
}

/// One delivery-topology dimension's observed condition on a family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologyClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5InstallTopologyClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5InstallTopologyConditionState,
}

/// An honest claim auto-narrow block. When a delivery-topology dimension weakens, the family's claim lowers to
/// the permitted ceiling, names the binding dimension and frozen trigger, and preserves the canonical
/// install-topology identity / last-known registry reference rather than silently dropping it — the underlying
/// install-mode / state-root / repair-verify / rollout-ring / rollback truth is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologyClaimAutoNarrow {
    /// The claim the family is narrowed to.
    pub narrowed_to: M5InstallTopologyA11yClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling constraint).
    pub binding_dimension: M5InstallTopologyClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5InstallTopologyDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical install-topology identity and last-known registry reference are preserved rather than
    /// dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying install-mode / state-root / repair-verify / rollout-ring / rollback truth is preserved
    /// (never dropped) across the narrowing; must hold so state-boundary-disclosed,
    /// repair-verify-unverified, and rollout-evidence-unverified states never fail opaquely.
    pub preserves_truth_continuity: bool,
}

impl InstallTopologyClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and install-mode / state-root /
    /// repair-verify / rollout-ring / rollback truth and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_truth_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for an install-topology family's accessible fallback: the same truth must be copyable
/// as text / JSON / Markdown, and a raw payload is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologyCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl InstallTopologyCopyExportParity {
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
pub struct InstallTopologyRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5InstallTopologyRenderingSurface,
    /// How the surface discloses its reduced install-topology projection.
    pub state: InstallTopologyNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The install-topology affordances reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for an install-topology accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallTopologyAccessibilityStatus {
    /// Full keyboard / screen-reader / high-zoom / high-contrast / localization / CLI / export parity with no
    /// narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims trusted, or drops state silently (red).
    Stranded,
}

impl InstallTopologyAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one install-topology family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologyAccessibilityRow {
    /// Record kind; must equal [`INSTALL_TOPOLOGY_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`INSTALL_TOPOLOGY_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen delivery-topology family this row certifies.
    pub install_topology_family: M5InstallTopologyFamily,
    /// Ref to the frozen canonical per-domain schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the install-topology family this row represents; stays visible on every surface, so this
    /// is never empty.
    pub install_topology_context_ref: String,
    /// Rendered modalities offered; a structure-heavy family must also offer a non-visual (list / textual /
    /// CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5InstallTopologyFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical install-topology identity, semantic role, registry
    /// reference, install mode, state root, and rollback target as the rendered family; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: InstallTopologyNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: InstallTopologyNonVisualReachState,
    /// High-zoom (200–400% reflow / magnification) legibility of the non-visual path.
    pub high_zoom_reach: InstallTopologyNonVisualReachState,
    /// High-contrast / larger-text legibility of the non-visual path.
    pub high_contrast_reach: InstallTopologyNonVisualReachState,
    /// Localization (translated vocabulary / locale-specific labels) fidelity of the non-visual path.
    pub localization_reach: InstallTopologyNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: InstallTopologyNonVisualReachState,
    /// Whether the export-safe summary preserves install-topology meaning.
    pub export_summary: InstallTopologyExportSummaryState,
    /// Ref to the export-safe summary object for this family.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: InstallTopologyCopyExportParity,
    /// The full claim this family asserts when every dimension is intact.
    pub full_ready_claim: M5InstallTopologyA11yClaim,
    /// The observed condition of each modeled delivery-topology dimension.
    #[serde(default)]
    pub claim_conditions: Vec<InstallTopologyClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<InstallTopologyClaimAutoNarrow>,
    /// Whether the underlying install-mode / state-root / repair-verify / rollout-ring / rollback truth is
    /// preserved on this family regardless of narrowing; must hold so every unverified projection never fails
    /// opaquely.
    pub truth_preserved: bool,
    /// Rendering surfaces this family is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5InstallTopologyRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<InstallTopologyRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5InstallTopologyRequiredLabel>,
    /// Semantic consumer surfaces this family is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5InstallTopologyConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl InstallTopologyAccessibilityRow {
    /// Returns true when this family renders a dense, structured surface and must bind to a flat non-visual
    /// path.
    pub const fn is_structure_heavy(&self) -> bool {
        family_is_structure_heavy(self.install_topology_family)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `FullyQualified` when the row does not model that
    /// dimension.
    pub fn condition_for(
        &self,
        dimension: M5InstallTopologyClaimDimension,
    ) -> M5InstallTopologyConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5InstallTopologyConditionState::FullyQualified)
    }

    /// Whether any modeled dimension is weaker than fully qualified.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling, capped at the family's
    /// full claim.
    pub fn permitted_claim(&self) -> M5InstallTopologyA11yClaim {
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
    pub fn binding_condition(&self) -> Option<&InstallTopologyClaimConditionEntry> {
        let mut binding: Option<(&InstallTopologyClaimConditionEntry, u8)> = None;
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
    pub fn binding_dimension(&self) -> Option<M5InstallTopologyClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The claim this family effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5InstallTopologyA11yClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_ready_claim,
        }
    }

    /// AC / auto-narrowing honesty: a partially-disclosed state boundary, an unconfirmed repair/verify
    /// coverage, or an aged-out / policy-blocked rollout-ring evidence can no longer keep an old
    /// `TrustedDeliverySurface` / `ReviewableDeliverySurface` label. The effective claim never exceeds the
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

    /// AC / trusted honesty: an unconfirmed-repair-verify / aged-out-rollout-evidence state never keeps a
    /// trusted claim — install-topology meaning is never conveyed by an installer-chrome-only affordance, a
    /// mislabeled screenshot, or an unlabeled control alone. When such a state is modeled, the effective claim
    /// must not assert `TrustedDeliverySurface`.
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
            && !self.install_topology_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.high_zoom_reach.never_traps()
            && self.high_contrast_reach.never_traps()
            && self.localization_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_structure_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the install-topology meaning without leaking a raw payload.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_payload()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: every unverified projection preserves the underlying install-mode / state-root /
    /// repair-verify / rollout-ring / rollback truth. The row must assert `truth_preserved`, and any narrow
    /// block must preserve truth continuity too.
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

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced install-topology
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
        let primary = family_primary_dimension(self.install_topology_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5InstallTopologyRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> InstallTopologyAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.trusted_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_truth_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return InstallTopologyAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            InstallTopologyAccessibilityStatus::NarrowedDisclosed
        } else {
            InstallTopologyAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == INSTALL_TOPOLOGY_A11Y_ROW_RECORD_KIND
            && self.schema_version == INSTALL_TOPOLOGY_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.install_topology_context_ref.trim().is_empty()
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
            family = self.install_topology_family.as_str(),
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

/// Rolled-up summary of an M05-1178 install-topology accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologyAccessibilitySummary {
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

/// Constructor input for [`InstallTopologyAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallTopologyAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<InstallTopologyAccessibilityRow>,
}

/// Checked-in M05-1178 install-topology accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologyAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<InstallTopologyAccessibilityRow>,
    pub summary: InstallTopologyAccessibilitySummary,
}

impl InstallTopologyAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: InstallTopologyAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: INSTALL_TOPOLOGY_A11Y_SCHEMA_VERSION,
            record_kind: INSTALL_TOPOLOGY_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: InstallTopologyAccessibilitySummary {
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
    pub fn represented_families(&self) -> BTreeSet<M5InstallTopologyFamily> {
        self.rows
            .iter()
            .map(|r| r.install_topology_family)
            .collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5InstallTopologyClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5InstallTopologyConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5InstallTopologyA11yClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5InstallTopologyConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> InstallTopologyAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5InstallTopologyConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let structure_heavy: Vec<&InstallTopologyAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_structure_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                InstallTopologyAccessibilityStatus::Parity => green += 1,
                InstallTopologyAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                InstallTopologyAccessibilityStatus::Stranded => red += 1,
            }
        }

        InstallTopologyAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            structure_heavy_family_count: structure_heavy.len(),
            all_structure_heavy_have_non_visual_fallback: structure_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(InstallTopologyAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(InstallTopologyAccessibilityRow::claim_is_honest),
            all_trusted_honesty_holds: self
                .rows
                .iter()
                .all(InstallTopologyAccessibilityRow::trusted_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(InstallTopologyAccessibilityRow::export_preserves_meaning),
            all_truth_preserved: self
                .rows
                .iter()
                .all(InstallTopologyAccessibilityRow::preserves_truth_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(InstallTopologyAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<InstallTopologyAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != INSTALL_TOPOLOGY_A11Y_SCHEMA_VERSION {
            violations.push(InstallTopologyAccessibilityViolation::SchemaVersion {
                expected: INSTALL_TOPOLOGY_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != INSTALL_TOPOLOGY_A11Y_RECORD_KIND {
            violations.push(InstallTopologyAccessibilityViolation::RecordKind {
                expected: INSTALL_TOPOLOGY_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(InstallTopologyAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(InstallTopologyAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.install_topology_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_shown_trusted())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(InstallTopologyAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    InstallTopologyAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.install_topology_family),
                    },
                );
            }

            // Each row must preserve every mandatory install-topology label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    InstallTopologyAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A structure-heavy family must render a structured projection *and* a non-visual path.
            if row.is_structure_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5InstallTopologyFallbackModality::Structured)
            {
                violations.push(
                    InstallTopologyAccessibilityViolation::StructureHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: claim never over-asserts a trusted / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(InstallTopologyAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC / trusted honesty: an unconfirmed-repair-verify / aged-out-rollout-evidence state never keeps
            // a trusted claim.
            if !row.trusted_honesty_holds() {
                violations.push(
                    InstallTopologyAccessibilityViolation::WeakStateShownAsTrusted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    InstallTopologyAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(
                    InstallTopologyAccessibilityViolation::ExportRequiresRawPayload {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: weakened states preserve install-mode / state-root / repair-verify / rollout /
            // rollback truth.
            if !row.preserves_truth_continuity() {
                violations.push(InstallTopologyAccessibilityViolation::TruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    InstallTopologyAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    InstallTopologyAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == InstallTopologyAccessibilityStatus::Stranded {
                violations.push(InstallTopologyAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5InstallTopologyFamily::ALL {
            if !seen_families.contains(&family) {
                violations
                    .push(InstallTopologyAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5InstallTopologyClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    InstallTopologyAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every condition state (the fully-qualified baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5InstallTopologyConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    InstallTopologyAccessibilityViolation::MissingConditionStateCoverage { state },
                );
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing spectrum
        // (trusted → … → rollout-evidence-unverified) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5InstallTopologyA11yClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    InstallTopologyAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Trusted honesty must be proven with at least one unconfirmed-repair-verify / aged-out-rollout-evidence
        // row in the packet, so the "cannot-prove never shown as trusted" guarantee is exercised end-to-end.
        if !has_unprovable_row {
            violations.push(InstallTopologyAccessibilityViolation::TrustedHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the updater-service, shell / About, diagnostics,
        // admin, installer, docs/help, CLI-export, support-export, and product surfaces — so every consumer
        // surface is exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5InstallTopologyConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    InstallTopologyAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(InstallTopologyAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("install-topology accessibility parity packet serializes"),
        ) {
            violations
                .push(InstallTopologyAccessibilityViolation::RawInstallTopologyMaterialInExport);
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
            .expect("install-topology accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,install_topology_family,keyboard_reach,screen_reader_reach,high_zoom_reach,high_contrast_reach,localization_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{high_zoom},{high_contrast},{localization},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.install_topology_family.as_str(),
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
        out.push_str("# M5 Install-Topology Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5InstallTopologyFamily::ALL.len(),
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
                row.install_topology_family.as_str(),
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

/// Reads and validates the checked-in install-topology accessibility parity export.
pub fn current_m5_install_topology_a11y_export(
) -> Result<InstallTopologyAccessibilityPacket, InstallTopologyAccessibilityArtifactError> {
    let packet: InstallTopologyAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-install-topology-accessibility-parity/support_export.json"
    )))
    .map_err(InstallTopologyAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(InstallTopologyAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in install-topology accessibility parity export.
#[derive(Debug)]
pub enum InstallTopologyAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<InstallTopologyAccessibilityViolation>),
}

impl fmt::Display for InstallTopologyAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "install-topology accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "install-topology accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for InstallTopologyAccessibilityArtifactError {}

/// Validation failure for M05-1178 install-topology accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallTopologyAccessibilityViolation {
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
        dimension: M5InstallTopologyClaimDimension,
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
        family: M5InstallTopologyFamily,
    },
    MissingDimensionCoverage {
        dimension: M5InstallTopologyClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5InstallTopologyConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5InstallTopologyA11yClaim,
    },
    TrustedHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5InstallTopologyConsumerSurface,
    },
    SummaryMismatch,
    RawInstallTopologyMaterialInExport,
}

impl InstallTopologyAccessibilityViolation {
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
            Self::RawInstallTopologyMaterialInExport => "raw_install_topology_material_in_export",
        }
    }
}

impl fmt::Display for InstallTopologyAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory install-topology label")
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
                    "row {id} shows an unconfirmed-repair-verify / aged-out-rollout-evidence state as a trusted delivery surface"
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
                    "row {id} does not preserve install-mode / state-root / repair-verify / rollout-ring / rollback truth across narrowing"
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
                    "install-topology family {family:?} is not certified in the packet"
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
                    "no unconfirmed-repair-verify / aged-out-rollout-evidence row is present to prove the trusted-honesty guarantee"
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
            Self::RawInstallTopologyMaterialInExport => {
                write!(f, "export contains raw install-topology material")
            }
        }
    }
}

impl Error for InstallTopologyAccessibilityViolation {}

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

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Mirrors the frozen install-topology
/// matrix's own forbidden-material policy (see [`crate::m5_install_topology_matrix`]): install-topology grammar
/// legitimately uses the plain word `secrets` (a writable state-root class), so this heuristic never matches
/// on it and instead targets raw credential blobs, bearer tokens, key blocks, and endpoint URLs.
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
pub const INSTALL_TOPOLOGY_A11Y_PACKET_ID: &str =
    "m5-install-topology-accessibility-parity:stable:0001";

/// Builds the canonical, checked-in install-topology accessibility parity packet. This is the one source of
/// truth shared by the tests and the on-disk support export so both stay byte-aligned.
pub fn seeded_m5_install_topology_a11y_packet() -> InstallTopologyAccessibilityPacket {
    InstallTopologyAccessibilityPacket::new(InstallTopologyAccessibilityPacketInput {
        packet_id: INSTALL_TOPOLOGY_A11Y_PACKET_ID.to_owned(),
        as_of: "2026-07-14T00:00:00Z".to_owned(),
        matrix_ref: INSTALL_TOPOLOGY_A11Y_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:install-topology-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5InstallTopologyRequiredLabel> {
    M5InstallTopologyRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> InstallTopologyCopyExportParity {
    InstallTopologyCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5InstallTopologyClaimDimension,
    state: M5InstallTopologyConditionState,
) -> InstallTopologyClaimConditionEntry {
    InstallTopologyClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and the general product
/// UI — so the narrowed state always reaches headless field triage.
fn base_consumers(
    extra: &[M5InstallTopologyConsumerSurface],
) -> Vec<M5InstallTopologyConsumerSurface> {
    let mut out = vec![
        M5InstallTopologyConsumerSurface::SupportExport,
        M5InstallTopologyConsumerSurface::ProductUi,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps full label
/// and summary parity on the narrower surfaces; a narrowed row discloses the reduced projection it drops
/// there.
fn surface_disclosures(
    labels: &[&str],
    state: InstallTopologyNarrowingDisclosureState,
) -> Vec<InstallTopologyRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        InstallTopologyRenderingNarrowingDisclosure {
            rendering_surface: M5InstallTopologyRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["installer_chrome_pointer_affordance".to_owned()],
        },
        InstallTopologyRenderingNarrowingDisclosure {
            rendering_surface: M5InstallTopologyRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_rollout_ring_transition".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<InstallTopologyRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        InstallTopologyNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced projection while
/// preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<InstallTopologyRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        InstallTopologyNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5InstallTopologyRenderingSurface> {
    vec![
        M5InstallTopologyRenderingSurface::DesktopFull,
        M5InstallTopologyRenderingSurface::CliHeadless,
        M5InstallTopologyRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5InstallTopologyFallbackModality> {
    vec![
        M5InstallTopologyFallbackModality::List,
        M5InstallTopologyFallbackModality::Textual,
        M5InstallTopologyFallbackModality::Cli,
    ]
}

fn structured_modalities() -> Vec<M5InstallTopologyFallbackModality> {
    vec![
        M5InstallTopologyFallbackModality::Structured,
        M5InstallTopologyFallbackModality::List,
        M5InstallTopologyFallbackModality::Textual,
        M5InstallTopologyFallbackModality::Cli,
    ]
}

const REACHABLE: InstallTopologyNonVisualReachState =
    InstallTopologyNonVisualReachState::ReachableAndLabeled;
const REDUCED: InstallTopologyNonVisualReachState =
    InstallTopologyNonVisualReachState::DisclosedReducedButReachable;

fn seeded_rows() -> Vec<InstallTopologyAccessibilityRow> {
    vec![
        // Per-user managed install (install mode + per-user updater ownership inspectable) — the
        // per-user-managed family scopes its binary and durable state to the user profile with per-user updater
        // ownership, so it is a trusted delivery surface reachable on every surface with no narrowing (green).
        // Not structure-heavy: it exposes a flat list / textual / CLI path.
        InstallTopologyAccessibilityRow {
            record_kind: INSTALL_TOPOLOGY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: INSTALL_TOPOLOGY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:per-user-managed-install-ownership-inspectable".to_owned(),
            install_topology_family: M5InstallTopologyFamily::PerUserManaged,
            source_family_schema_ref: M5InstallTopologyFamily::PerUserManaged
                .canonical_domain_schema_ref()
                .to_owned(),
            install_topology_context_ref: "updater:per-user-managed:0001".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            localization_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: InstallTopologyExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:per-user-managed-install-ownership-inspectable:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "install_topology_identity",
                "semantic_role",
                "registry_reference",
                "install_mode",
            ]),
            full_ready_claim: M5InstallTopologyA11yClaim::TrustedDeliverySurface,
            claim_conditions: vec![condition(
                M5InstallTopologyClaimDimension::InstallOwnershipClarity,
                M5InstallTopologyConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "install_topology_identity",
                "semantic_role",
                "install_mode",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5InstallTopologyConsumerSurface::UpdaterService,
                M5InstallTopologyConsumerSurface::ShellUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.12 — Per-user managed install / updater ownership".to_owned(),
                INSTALL_TOPOLOGY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-14T00:00:00Z".to_owned(),
            evidence_refs: ev("per-user-managed-install-ownership-inspectable"),
        },
        // Per-machine managed install (admin / policy control inspectable) — the per-machine-managed family
        // exposes its admin / system updater ownership and machine-policy control as an inspectable read-only
        // reference an admin can review, so it is a self-sufficient reviewable delivery surface, but its
        // narrower non-visual traversal discloses a reduced high-zoom reflow walk of the dense policy-roots
        // table (yellow). Structure-heavy: no — it exposes a flat list / textual path.
        InstallTopologyAccessibilityRow {
            record_kind: INSTALL_TOPOLOGY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: INSTALL_TOPOLOGY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:per-machine-managed-policy-control-inspectable".to_owned(),
            install_topology_family: M5InstallTopologyFamily::PerMachineManaged,
            source_family_schema_ref: M5InstallTopologyFamily::PerMachineManaged
                .canonical_domain_schema_ref()
                .to_owned(),
            install_topology_context_ref: "admin:per-machine-managed:0002".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REDUCED,
            high_contrast_reach: REACHABLE,
            localization_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: InstallTopologyExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:per-machine-managed-policy-control-inspectable:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "install_topology_identity",
                "semantic_role",
                "registry_reference",
                "updater_owner",
            ]),
            full_ready_claim: M5InstallTopologyA11yClaim::ReviewableDeliverySurface,
            claim_conditions: vec![condition(
                M5InstallTopologyClaimDimension::PolicyControlClarity,
                M5InstallTopologyConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "install_topology_identity",
                "semantic_role",
                "updater_owner",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5InstallTopologyConsumerSurface::Admin,
                M5InstallTopologyConsumerSurface::Installer,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.12 — Per-machine managed install / admin policy control".to_owned(),
                INSTALL_TOPOLOGY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-14T00:00:00Z".to_owned(),
            evidence_refs: ev("per-machine-managed-policy-control-inspectable"),
        },
        // Side-by-side stable/preview (state-boundary proof partially disclosed) — the
        // side-by-side-stable-preview family's stable/preview state-boundary proof can only be partially
        // disclosed, so it auto-narrows to a state-boundary-disclosed projection that discloses the partial
        // boundary proof alongside the last-known isolated state root, never a shared namespace shown as
        // isolated when its boundary proof is incomplete (yellow). Its localized traversal narrows the
        // localization path to a disclosed reduction. Structure-heavy: its channel-isolation table binds to a
        // flat list / textual path. A partial boundary disclosure is an honest disclosed-absence operation, not
        // a trusted overstatement.
        InstallTopologyAccessibilityRow {
            record_kind: INSTALL_TOPOLOGY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: INSTALL_TOPOLOGY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:side-by-side-state-boundary-disclosed-partial".to_owned(),
            install_topology_family: M5InstallTopologyFamily::SideBySideStablePreview,
            source_family_schema_ref: M5InstallTopologyFamily::SideBySideStablePreview
                .canonical_domain_schema_ref()
                .to_owned(),
            install_topology_context_ref: "diagnostics:side-by-side-stable-preview:0003".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            localization_reach: REDUCED,
            cli_reach: REACHABLE,
            export_summary: InstallTopologyExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:side-by-side-state-boundary-disclosed-partial:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "install_topology_identity",
                "semantic_role",
                "registry_reference",
                "writable_state_roots",
            ]),
            full_ready_claim: M5InstallTopologyA11yClaim::TrustedDeliverySurface,
            claim_conditions: vec![condition(
                M5InstallTopologyClaimDimension::StateBoundaryClarity,
                M5InstallTopologyConditionState::StateBoundaryDisclosedPartial,
            )],
            claim_narrow: Some(InstallTopologyClaimAutoNarrow {
                narrowed_to: M5InstallTopologyA11yClaim::StateBoundaryDisclosedProjection,
                binding_dimension: M5InstallTopologyClaimDimension::StateBoundaryClarity,
                trigger: M5InstallTopologyDowngradeTrigger::ProofStale,
                narrowed_label:
                    "This side-by-side stable/preview family can only disclose a partial state-boundary proof — shown as a state-boundary-disclosed projection that discloses the partial boundary proof alongside the last-known isolated state root, never presenting a shared state namespace as isolated when its boundary proof is incomplete"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "install_topology_identity",
                "semantic_role",
                "writable_state_roots",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5InstallTopologyConsumerSurface::Diagnostics,
                M5InstallTopologyConsumerSurface::CliExport,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.12 — Side-by-side stable / preview channels".to_owned(),
                INSTALL_TOPOLOGY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-14T00:00:00Z".to_owned(),
            evidence_refs: ev("side-by-side-state-boundary-disclosed-partial"),
        },
        // Portable mode (repair/verify coverage unconfirmed) — the portable-mode family's colocated repair /
        // verify coverage cannot be confirmed, so it auto-narrows to a repair-verify-unverified projection that
        // keeps the last-known repair/verify posture explicit, never a repair/verify shown as covered when it
        // may never have run (yellow). Its forced-colors response narrows the high-contrast path to a disclosed
        // reduction. Structure-heavy: its portable root-inventory table binds to a flat list / textual path.
        InstallTopologyAccessibilityRow {
            record_kind: INSTALL_TOPOLOGY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: INSTALL_TOPOLOGY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:portable-mode-repair-verify-unconfirmed".to_owned(),
            install_topology_family: M5InstallTopologyFamily::PortableMode,
            source_family_schema_ref: M5InstallTopologyFamily::PortableMode
                .canonical_domain_schema_ref()
                .to_owned(),
            install_topology_context_ref: "diagnostics:portable-mode:0004".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REDUCED,
            localization_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: InstallTopologyExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:portable-mode-repair-verify-unconfirmed:a11y".to_owned(),
            copy_export: copy_export(&[
                "install_topology_identity",
                "semantic_role",
                "registry_reference",
                "repair_verify_posture",
            ]),
            full_ready_claim: M5InstallTopologyA11yClaim::TrustedDeliverySurface,
            claim_conditions: vec![condition(
                M5InstallTopologyClaimDimension::RepairVerifyClarity,
                M5InstallTopologyConditionState::RepairVerifyUnconfirmed,
            )],
            claim_narrow: Some(InstallTopologyClaimAutoNarrow {
                narrowed_to: M5InstallTopologyA11yClaim::RepairVerifyUnverifiedProjection,
                binding_dimension: M5InstallTopologyClaimDimension::RepairVerifyClarity,
                trigger:
                    M5InstallTopologyDowngradeTrigger::DeploymentClaimOutpacedRingOrRepairVerifyEvidence,
                narrowed_label:
                    "This portable-mode family cannot confirm that its repair / verify coverage was run — shown as a repair-verify-unverified projection that keeps the last-known repair/verify posture explicit, never presenting a repair/verify pass as covered when it may never have run"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "install_topology_identity",
                "semantic_role",
                "repair_verify_posture",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5InstallTopologyConsumerSurface::DocsHelp,
                M5InstallTopologyConsumerSurface::Diagnostics,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.12 — Portable mode / repair / verify".to_owned(),
                INSTALL_TOPOLOGY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-14T00:00:00Z".to_owned(),
            evidence_refs: ev("portable-mode-repair-verify-unconfirmed"),
        },
        // Offline / air-gap bundle (rollout-ring evidence aged out or policy-blocked) — the
        // offline-air-gap-bundle family's rollout-ring promotion / rollback evidence has aged out or is
        // policy-blocked, so it auto-narrows to a rollout-evidence-unverified projection that keeps the
        // last-known ring state explicit, never a ring shown as promoted or a claim shown as published when its
        // evidence has aged out or become policy-blocked (yellow). Structure-heavy: its artifact-graph rollback
        // table binds to a flat list / textual path.
        InstallTopologyAccessibilityRow {
            record_kind: INSTALL_TOPOLOGY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: INSTALL_TOPOLOGY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:offline-airgap-rollout-evidence-unconfirmed".to_owned(),
            install_topology_family: M5InstallTopologyFamily::OfflineAirgapBundle,
            source_family_schema_ref: M5InstallTopologyFamily::OfflineAirgapBundle
                .canonical_domain_schema_ref()
                .to_owned(),
            install_topology_context_ref: "admin:offline-airgap-bundle:0005".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            localization_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: InstallTopologyExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:offline-airgap-rollout-evidence-unconfirmed:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "install_topology_identity",
                "semantic_role",
                "registry_reference",
                "rollout_ring",
            ]),
            full_ready_claim: M5InstallTopologyA11yClaim::TrustedDeliverySurface,
            claim_conditions: vec![condition(
                M5InstallTopologyClaimDimension::RolloutEvidenceClarity,
                M5InstallTopologyConditionState::RolloutEvidenceUnconfirmed,
            )],
            claim_narrow: Some(InstallTopologyClaimAutoNarrow {
                narrowed_to: M5InstallTopologyA11yClaim::RolloutEvidenceUnverifiedProjection,
                binding_dimension: M5InstallTopologyClaimDimension::RolloutEvidenceClarity,
                trigger:
                    M5InstallTopologyDowngradeTrigger::DeploymentClaimOutpacedRingOrRepairVerifyEvidence,
                narrowed_label:
                    "This offline / air-gap bundle cannot confirm current rollout-ring promotion or rollback evidence — shown as a rollout-evidence-unverified projection that keeps the last-known ring state explicit, never presenting a ring as promoted or a deployment claim as published when its evidence has aged out or become policy-blocked"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "install_topology_identity",
                "semantic_role",
                "rollout_ring",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5InstallTopologyConsumerSurface::Admin,
                M5InstallTopologyConsumerSurface::Installer,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.12 — Offline / air-gap bundle / fleet rollout".to_owned(),
                INSTALL_TOPOLOGY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-14T00:00:00Z".to_owned(),
            evidence_refs: ev("offline-airgap-rollout-evidence-unconfirmed"),
        },
    ]
}
