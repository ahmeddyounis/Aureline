//! Keyboard / screen-reader / high-zoom / high-contrast / localization / CLI / export parity, and honest
//! automatic claim narrowing for the M5 open-local / clone-remote / open-archive / import-bundle /
//! resume-snapshot repository-bootstrap families.
//!
//! This module is the M05-1194 accessibility-localization-support-export parity and auto-narrowing capstone
//! over the frozen M5 repository-bootstrap matrix ([`crate::m5_repository_bootstrap_matrix`]). Where the freeze
//! matrix defines the five governed project-entry acquisition families, and the 1189-1192 implementation lanes
//! resolve their per-surface source-locator, checkout-plan, credential-posture, staged-trust, post-open-queue,
//! and bootstrap-evidence truth, this lane certifies — per acquisition family — that source-locator /
//! checkout-plan / credential-posture / staged-trust / bootstrap-evidence / mirror-signer-continuity /
//! partial-acquisition claims stay **keyboard-reachable, screen-reader-announced, high-zoom-legible,
//! high-contrast-safe, localization-safe, CLI/export-safe, and self-narrowing** rather than presenting a
//! checkout plan that only lives in an entry screenshot, a bootstrap shown as trusted without proof, a
//! repo-owned action shown as fenced when it was never staged, or a mirror shown as signer-continuous when its
//! evidence has aged out as still a stable, trusted acquisition surface:
//!
//! - **Keyboard / screen-reader / high-zoom / high-contrast / localization / CLI reach.** Every family
//!   exposes a keyboard-reachable, screen-reader-announced, high-zoom-reflowing, high-contrast-legible,
//!   localization-safe, and CLI/headless-reachable path into the same repository-bootstrap identity, semantic
//!   role, registry reference, source locator, checkout plan, and credential posture the rendered surface
//!   shows — never a pointer-only affordance hidden in entry chrome, an unlabeled control, or a checkout plan /
//!   credential posture that only lives in a screenshot and strands assistive-tech, localized, or headless-CLI
//!   users. Structure-heavy families (the open-archive extraction-plan table, the import-bundle staged-trust
//!   table, the resume-snapshot evidence-lineage table) additionally bind their structured layout to a flat
//!   list / textual / CLI path.
//! - **Export parity.** The support / release / CLI export reconstructs each family's meaning from typed
//!   tokens and opaque refs **without a raw payload**, preserving the same repository-bootstrap identity,
//!   semantic role, registry reference, source locator, checkout plan, and credential posture shown in-product
//!   so support, help, and release proof can reconstruct which acquisition truth class was active without
//!   leaking a raw secret blob, a machine-specific sensitive path, or an entry-only screenshot.
//! - **Honest auto-narrowing.** When an open-archive family's checkout-plan proof can only be partially
//!   disclosed, an import-bundle family's staged-trust fence cannot be confirmed, or a resume-snapshot family's
//!   bootstrap / mirror-signer evidence has aged out or is policy-blocked, the family's claim auto-narrows from
//!   `trusted_acquisition_surface` / `reviewable_acquisition_surface` to a checkout-plan-disclosed /
//!   trust-stage-unverified / bootstrap-evidence-unverified projection, discloses the narrowing with a precise
//!   trigger and binding dimension, and preserves the canonical repository-bootstrap identity / last-known
//!   registry reference. The underlying source-locator / checkout-plan / credential-posture / staged-trust /
//!   bootstrap-evidence truth is never dropped opaquely. A family with every dimension intact must NOT carry a
//!   spurious narrowing, and a trust-overclaimed / evidence-aged / policy-blocked state can never keep a
//!   trusted, stable acquisition claim — repository-bootstrap meaning is never conveyed by an entry-chrome-only
//!   affordance, a mislabeled screenshot, or an unlabeled control alone.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the acquisition engine, the shell UI,
//!   the workspace service, the git service, the trust service, the diagnostics surface, the docs / help
//!   surface, the CLI export, and the support export so product, help, and release publication stay aligned on
//!   downgrade behavior rather than drifting in copy — a trusted-looking acquisition surface can never outrun
//!   the source-locator / checkout-plan / staged-trust / bootstrap-evidence evidence it is being viewed away
//!   from.
//!
//! Each [`RepositoryBootstrapAccessibilityRow`] keys on one
//! [`crate::m5_repository_bootstrap_matrix::M5RepositoryBootstrapFamily`] and reuses that frozen family
//! vocabulary plus the frozen [`M5RepositoryBootstrapRequiredLabel`], [`M5RepositoryBootstrapDowngradeTrigger`],
//! and shared [`M5RepositoryBootstrapConsumerSurface`] consumer surfaces rather than minting parallel synonyms,
//! so the certified labels stay byte-identical to the matrix and the sibling repository-bootstrap packets.
//!
//! The packet is metadata-only: raw secret blobs, machine-specific sensitive paths, plaintext payloads, and
//! endpoint refs never cross this boundary; the packet carries only typed class tokens, opaque
//! repository-bootstrap refs, booleans, and controlled labels so support, release, and diagnostics exports can
//! reconstruct exactly which acquisition truth class was active without leaking sensitive material or a raw
//! payload.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen repository-bootstrap vocabulary — the capstone certifies the freeze matrix's families, required
// labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::m5_repository_bootstrap_matrix::{
    M5RepositoryBootstrapConsumerSurface, M5RepositoryBootstrapDowngradeTrigger,
    M5RepositoryBootstrapFamily, M5RepositoryBootstrapRequiredLabel,
    M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
};

/// Schema version stamped on the M05-1194 repository-bootstrap accessibility parity packet.
pub const REPOSITORY_BOOTSTRAP_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`RepositoryBootstrapAccessibilityPacket`].
pub const REPOSITORY_BOOTSTRAP_A11Y_RECORD_KIND: &str =
    "m5_repository_bootstrap_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`RepositoryBootstrapAccessibilityRow`].
pub const REPOSITORY_BOOTSTRAP_A11Y_ROW_RECORD_KIND: &str =
    "m5_repository_bootstrap_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const REPOSITORY_BOOTSTRAP_A11Y_SCHEMA_REF: &str =
    "schemas/workspaces/m5-repository-bootstrap-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const REPOSITORY_BOOTSTRAP_A11Y_DOC_REF: &str =
    "docs/workspaces/m5_repository_bootstrap_accessibility_parity.md";

/// Repo-relative path of the frozen repository-bootstrap matrix this lane certifies.
pub const REPOSITORY_BOOTSTRAP_A11Y_MATRIX_REF: &str = M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const REPOSITORY_BOOTSTRAP_A11Y_FIXTURE_DIR: &str =
    "fixtures/workspaces/m5-repository-bootstrap-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const REPOSITORY_BOOTSTRAP_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-repository-bootstrap-accessibility-parity/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const REPOSITORY_BOOTSTRAP_A11Y_CSV_REF: &str =
    "artifacts/release/m5-repository-bootstrap-accessibility-parity/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const REPOSITORY_BOOTSTRAP_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-repository-bootstrap-accessibility-parity.md";

/// The reusable repository-bootstrap families that render a dense, structured surface (the open-archive
/// extraction-plan table, the import-bundle staged-trust table, the resume-snapshot evidence-lineage table) and
/// therefore MUST bind their structured layout to an equivalent flat list / textual / CLI path so the
/// structure is navigable non-visually.
const fn family_is_structure_heavy(family: M5RepositoryBootstrapFamily) -> bool {
    matches!(
        family,
        M5RepositoryBootstrapFamily::OpenArchive
            | M5RepositoryBootstrapFamily::ImportBundle
            | M5RepositoryBootstrapFamily::ResumeSnapshot
    )
}

/// The acquisition-truth dimension whose weakening a family primarily discloses. Every row must model at least
/// this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5RepositoryBootstrapFamily,
) -> M5RepositoryBootstrapClaimDimension {
    match family {
        M5RepositoryBootstrapFamily::OpenLocal => {
            M5RepositoryBootstrapClaimDimension::SourceLocatorClarity
        }
        M5RepositoryBootstrapFamily::CloneRemote => {
            M5RepositoryBootstrapClaimDimension::CredentialPostureClarity
        }
        M5RepositoryBootstrapFamily::OpenArchive => {
            M5RepositoryBootstrapClaimDimension::CheckoutPlanClarity
        }
        M5RepositoryBootstrapFamily::ImportBundle => {
            M5RepositoryBootstrapClaimDimension::TrustStageFenceClarity
        }
        M5RepositoryBootstrapFamily::ResumeSnapshot => {
            M5RepositoryBootstrapClaimDimension::BootstrapEvidenceClarity
        }
    }
}

/// A rendered fallback modality for a repository-bootstrap family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepositoryBootstrapFallbackModality {
    /// A rich, structured (extraction-plan / staged-trust / evidence-lineage table) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / label-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5RepositoryBootstrapFallbackModality {
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

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same
/// repository-bootstrap family may render at desktop-full capability or narrow to a companion, read-only
/// browser, headless CLI, docs export, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepositoryBootstrapRenderingSurface {
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

impl M5RepositoryBootstrapRenderingSurface {
    /// Returns true when the surface narrows the repository-bootstrap family below the desktop full-capability
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

/// Keyboard / screen-reader / high-zoom / high-contrast / localization / CLI reach for a repository-bootstrap
/// family's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryBootstrapNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// An entry-chrome-only / pointer-only / view-only surface that traps keyboard / assistive-tech /
    /// localized / headless-CLI users (red).
    ViewOnlyTrap,
}

impl RepositoryBootstrapNonVisualReachState {
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

/// Whether an export-safe summary preserves the repository-bootstrap meaning without leaking a raw payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryBootstrapExportSummaryState {
    /// The repository-bootstrap meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl RepositoryBootstrapExportSummaryState {
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

/// Whether a narrower rendering surface discloses its reduced repository-bootstrap projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryBootstrapNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced projection, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Repository-bootstrap state or tokens dropped without disclosure (red).
    SilentlyDropped,
}

impl RepositoryBootstrapNarrowingDisclosureState {
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

/// The repository-bootstrap claim ceiling a family asserts: how strong a trusted / stable posture it lets a
/// surface present. Auto-narrowing lowers this ceiling when a checkout-plan / staged-trust / bootstrap-evidence
/// dimension weakens so a partially-disclosed checkout plan, an unconfirmed staged-trust fence, or an aged-out
/// / policy-blocked bootstrap-evidence / mirror-signer continuity can never keep an old
/// `TrustedAcquisitionSurface` or `ReviewableAcquisitionSurface` label — repository-bootstrap meaning is never
/// conveyed by an entry-chrome-only affordance, a mislabeled screenshot, or an unlabeled control alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepositoryBootstrapA11yClaim {
    /// Trusted acquisition surface: a fully current, registry-bound, source-locator-resolved,
    /// checkout-plan-disclosed, credential-posture-disclosed, staged-trust-fenced, signer-continuous
    /// acquisition — the strongest claim, a repository-bootstrap surface Aureline can present as exactly trusted
    /// and stable right now.
    TrustedAcquisitionSurface,
    /// Reviewable acquisition surface: a self-sufficient, inspectable read-only repository-bootstrap projection
    /// (a static checkout-plan / credential-posture / registry reference an operator can inspect) that is not
    /// itself an authoritative, live-resolving surface.
    ReviewableAcquisitionSurface,
    /// Checkout-plan-disclosed projection: an open-archive's checkout / extraction-plan proof can only be
    /// partially disclosed; the family stays a checkout-plan-disclosed projection that discloses the partial
    /// checkout-plan proof alongside the last-known cost / topology placeholders, never a hydrated checkout
    /// shown as exact when its plan proof is incomplete.
    CheckoutPlanDisclosedProjection,
    /// Trust-stage-unverified projection: an import-bundle family's staged-trust / no-implicit-execution fence
    /// cannot be confirmed; the family stays a trust-stage-unverified projection that keeps the last-known
    /// deferred-repo-owned-action posture explicit, never a bundle shown as staged when a repo-owned action may
    /// have run implicitly.
    TrustStageUnverifiedProjection,
    /// Bootstrap-evidence-unverified projection: a resume-snapshot family's bootstrap / mirror-signer evidence
    /// has aged out or is policy-blocked; the family stays a bootstrap-evidence-unverified projection that keeps
    /// the last-known signer / digest / partial-root state explicit, never a mirror shown as signer-continuous
    /// when its evidence has aged out or become policy-blocked.
    BootstrapEvidenceUnverifiedProjection,
}

impl M5RepositoryBootstrapA11yClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 5] = [
        Self::TrustedAcquisitionSurface,
        Self::ReviewableAcquisitionSurface,
        Self::CheckoutPlanDisclosedProjection,
        Self::TrustStageUnverifiedProjection,
        Self::BootstrapEvidenceUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::TrustedAcquisitionSurface => 4,
            Self::ReviewableAcquisitionSurface => 3,
            Self::CheckoutPlanDisclosedProjection => 2,
            Self::TrustStageUnverifiedProjection => 1,
            Self::BootstrapEvidenceUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully trusted, stable acquisition surface.
    pub const fn asserts_trusted_surface(self) -> bool {
        matches!(self, Self::TrustedAcquisitionSurface)
    }

    /// Returns true when this claim asserts a fully self-sufficient (trusted or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::TrustedAcquisitionSurface | Self::ReviewableAcquisitionSurface
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedAcquisitionSurface => "trusted_acquisition_surface",
            Self::ReviewableAcquisitionSurface => "reviewable_acquisition_surface",
            Self::CheckoutPlanDisclosedProjection => "checkout_plan_disclosed_projection",
            Self::TrustStageUnverifiedProjection => "trust_stage_unverified_projection",
            Self::BootstrapEvidenceUnverifiedProjection => {
                "bootstrap_evidence_unverified_projection"
            }
        }
    }
}

/// The source-locator / credential-posture / checkout-plan / staged-trust / bootstrap-evidence dimension whose
/// state governs how far a repository-bootstrap family may claim to be a fully trusted, stable acquisition
/// surface. The dimensions map 1:1 to the five frozen acquisition families so every family carries an honest
/// narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepositoryBootstrapClaimDimension {
    /// Source-locator clarity: is the source locator and its resolved checkout root inspectable rather than
    /// hidden or rewritten (open-local)?
    SourceLocatorClarity,
    /// Credential-posture clarity: is the bootstrap credential posture disclosed before network access rather
    /// than hidden behind generic connected-state copy (clone-remote)?
    CredentialPostureClarity,
    /// Checkout-plan clarity: does the open-archive prove a checkout / extraction plan and disclosed cost
    /// rather than silently mutating disk (open-archive)?
    CheckoutPlanClarity,
    /// Trust-stage-fence clarity: does the import-bundle stay staged-trust-fenced rather than silently running
    /// a repo-owned action implicitly during acquisition (import-bundle)?
    TrustStageFenceClarity,
    /// Bootstrap-evidence clarity: does the resume-snapshot bootstrap / mirror-signer evidence stay current
    /// rather than aging out or becoming policy-blocked (resume-snapshot)?
    BootstrapEvidenceClarity,
}

impl M5RepositoryBootstrapClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SourceLocatorClarity,
        Self::CredentialPostureClarity,
        Self::CheckoutPlanClarity,
        Self::TrustStageFenceClarity,
        Self::BootstrapEvidenceClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceLocatorClarity => "source_locator_clarity",
            Self::CredentialPostureClarity => "credential_posture_clarity",
            Self::CheckoutPlanClarity => "checkout_plan_clarity",
            Self::TrustStageFenceClarity => "trust_stage_fence_clarity",
            Self::BootstrapEvidenceClarity => "bootstrap_evidence_clarity",
        }
    }
}

/// The observed condition of one acquisition dimension. Anything weaker than [`Self::FullyQualified`] imposes a
/// narrowing ceiling on the family's claim. The unconfirmed states the lane must auto-narrow on as *weakened
/// evidence* — an unconfirmed staged-trust fence and an aged-out / policy-blocked bootstrap-evidence /
/// mirror-signer continuity — are the states that [`Self::cannot_be_shown_trusted`] flags. A partially-disclosed
/// checkout plan is an honest disclosed-absence operation (a partial checkout-plan proof shown honestly with
/// the last-known cost / topology placeholders), not a truth overstatement, so it is deliberately excluded
/// there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepositoryBootstrapConditionState {
    /// Fully current, registry-bound, source-locator-resolved, checkout-plan-disclosed,
    /// credential-posture-disclosed, staged-trust-fenced, signer-continuous — imposes no ceiling.
    FullyQualified,
    /// The open-archive's checkout / extraction-plan proof can only be partially disclosed — claim drops to a
    /// checkout-plan-disclosed projection.
    CheckoutPlanDisclosedPartial,
    /// The import-bundle staged-trust / no-implicit-execution fence cannot be confirmed — claim drops to a
    /// trust-stage-unverified projection.
    TrustStageUnconfirmed,
    /// The resume-snapshot bootstrap / mirror-signer evidence has aged out or is policy-blocked — claim drops
    /// to a bootstrap-evidence-unverified projection.
    BootstrapEvidenceUnconfirmed,
}

impl M5RepositoryBootstrapConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullyQualified,
        Self::CheckoutPlanDisclosedPartial,
        Self::TrustStageUnconfirmed,
        Self::BootstrapEvidenceUnconfirmed,
    ];

    /// Returns true when the dimension is weaker than fully qualified and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FullyQualified)
    }

    /// Returns true when the condition reflects weakened evidence that cannot be shown as a fully trusted,
    /// stable acquisition surface and must never be shown as such. A partially-disclosed checkout plan is an
    /// honest disclosed-absence operation (a partial checkout-plan proof shown honestly with the last-known
    /// cost / topology placeholders), not a truth overstatement, so it is deliberately excluded here.
    pub const fn cannot_be_shown_trusted(self) -> bool {
        matches!(
            self,
            Self::TrustStageUnconfirmed | Self::BootstrapEvidenceUnconfirmed
        )
    }

    /// The strongest claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5RepositoryBootstrapA11yClaim {
        match self {
            Self::FullyQualified => M5RepositoryBootstrapA11yClaim::TrustedAcquisitionSurface,
            Self::CheckoutPlanDisclosedPartial => {
                M5RepositoryBootstrapA11yClaim::CheckoutPlanDisclosedProjection
            }
            Self::TrustStageUnconfirmed => {
                M5RepositoryBootstrapA11yClaim::TrustStageUnverifiedProjection
            }
            Self::BootstrapEvidenceUnconfirmed => {
                M5RepositoryBootstrapA11yClaim::BootstrapEvidenceUnverifiedProjection
            }
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each state maps
    /// to the on-topic frozen trigger the freeze matrix already governs, so the certified reason stays
    /// byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5RepositoryBootstrapDowngradeTrigger {
        match self {
            // The fully-qualified baseline never narrows; kept for exhaustiveness.
            Self::FullyQualified => M5RepositoryBootstrapDowngradeTrigger::ProofStale,
            Self::CheckoutPlanDisclosedPartial => {
                M5RepositoryBootstrapDowngradeTrigger::ProofStale
            }
            Self::TrustStageUnconfirmed => {
                M5RepositoryBootstrapDowngradeTrigger::StagedTrustRuleUnstated
            }
            Self::BootstrapEvidenceUnconfirmed => {
                M5RepositoryBootstrapDowngradeTrigger::LostSignerOrMirrorProvenanceAcrossOfflineOrMirroredFetches
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyQualified => "fully_qualified",
            Self::CheckoutPlanDisclosedPartial => "checkout_plan_disclosed_partial",
            Self::TrustStageUnconfirmed => "trust_stage_unconfirmed",
            Self::BootstrapEvidenceUnconfirmed => "bootstrap_evidence_unconfirmed",
        }
    }
}

/// One acquisition dimension's observed condition on a family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBootstrapClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5RepositoryBootstrapClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5RepositoryBootstrapConditionState,
}

/// An honest claim auto-narrow block. When an acquisition dimension weakens, the family's claim lowers to the
/// permitted ceiling, names the binding dimension and frozen trigger, and preserves the canonical
/// repository-bootstrap identity / last-known registry reference rather than silently dropping it — the
/// underlying source-locator / checkout-plan / credential-posture / staged-trust / bootstrap-evidence truth is
/// never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBootstrapClaimAutoNarrow {
    /// The claim the family is narrowed to.
    pub narrowed_to: M5RepositoryBootstrapA11yClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling constraint).
    pub binding_dimension: M5RepositoryBootstrapClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5RepositoryBootstrapDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical repository-bootstrap identity and last-known registry reference are preserved rather than
    /// dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying source-locator / checkout-plan / credential-posture / staged-trust / bootstrap-evidence
    /// truth is preserved (never dropped) across the narrowing; must hold so checkout-plan-disclosed,
    /// trust-stage-unverified, and bootstrap-evidence-unverified states never fail opaquely.
    pub preserves_truth_continuity: bool,
}

impl RepositoryBootstrapClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and source-locator /
    /// checkout-plan / credential-posture / staged-trust / bootstrap-evidence truth and carries a precise,
    /// non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_truth_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a repository-bootstrap family's accessible fallback: the same truth must be copyable
/// as text / JSON / Markdown, and a raw payload is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBootstrapCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl RepositoryBootstrapCopyExportParity {
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
pub struct RepositoryBootstrapRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5RepositoryBootstrapRenderingSurface,
    /// How the surface discloses its reduced repository-bootstrap projection.
    pub state: RepositoryBootstrapNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The repository-bootstrap affordances reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a repository-bootstrap accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryBootstrapAccessibilityStatus {
    /// Full keyboard / screen-reader / high-zoom / high-contrast / localization / CLI / export parity with no
    /// narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims trusted, or drops state silently (red).
    Stranded,
}

impl RepositoryBootstrapAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one repository-bootstrap family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBootstrapAccessibilityRow {
    /// Record kind; must equal [`REPOSITORY_BOOTSTRAP_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`REPOSITORY_BOOTSTRAP_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen acquisition family this row certifies.
    pub repository_bootstrap_family: M5RepositoryBootstrapFamily,
    /// Ref to the frozen canonical per-domain schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the repository-bootstrap family this row represents; stays visible on every surface, so
    /// this is never empty.
    pub repository_bootstrap_context_ref: String,
    /// Rendered modalities offered; a structure-heavy family must also offer a non-visual (list / textual /
    /// CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5RepositoryBootstrapFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical repository-bootstrap identity, semantic role,
    /// registry reference, source locator, checkout plan, and credential posture as the rendered family; must
    /// hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: RepositoryBootstrapNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: RepositoryBootstrapNonVisualReachState,
    /// High-zoom (200–400% reflow / magnification) legibility of the non-visual path.
    pub high_zoom_reach: RepositoryBootstrapNonVisualReachState,
    /// High-contrast / larger-text legibility of the non-visual path.
    pub high_contrast_reach: RepositoryBootstrapNonVisualReachState,
    /// Localization (translated vocabulary / locale-specific labels) fidelity of the non-visual path.
    pub localization_reach: RepositoryBootstrapNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: RepositoryBootstrapNonVisualReachState,
    /// Whether the export-safe summary preserves repository-bootstrap meaning.
    pub export_summary: RepositoryBootstrapExportSummaryState,
    /// Ref to the export-safe summary object for this family.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: RepositoryBootstrapCopyExportParity,
    /// The full claim this family asserts when every dimension is intact.
    pub full_ready_claim: M5RepositoryBootstrapA11yClaim,
    /// The observed condition of each modeled acquisition dimension.
    #[serde(default)]
    pub claim_conditions: Vec<RepositoryBootstrapClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<RepositoryBootstrapClaimAutoNarrow>,
    /// Whether the underlying source-locator / checkout-plan / credential-posture / staged-trust /
    /// bootstrap-evidence truth is preserved on this family regardless of narrowing; must hold so every
    /// unverified projection never fails opaquely.
    pub truth_preserved: bool,
    /// Rendering surfaces this family is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5RepositoryBootstrapRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<RepositoryBootstrapRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5RepositoryBootstrapRequiredLabel>,
    /// Semantic consumer surfaces this family is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5RepositoryBootstrapConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl RepositoryBootstrapAccessibilityRow {
    /// Returns true when this family renders a dense, structured surface and must bind to a flat non-visual
    /// path.
    pub const fn is_structure_heavy(&self) -> bool {
        family_is_structure_heavy(self.repository_bootstrap_family)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `FullyQualified` when the row does not model that
    /// dimension.
    pub fn condition_for(
        &self,
        dimension: M5RepositoryBootstrapClaimDimension,
    ) -> M5RepositoryBootstrapConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5RepositoryBootstrapConditionState::FullyQualified)
    }

    /// Whether any modeled dimension is weaker than fully qualified.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling, capped at the family's
    /// full claim.
    pub fn permitted_claim(&self) -> M5RepositoryBootstrapA11yClaim {
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
    pub fn binding_condition(&self) -> Option<&RepositoryBootstrapClaimConditionEntry> {
        let mut binding: Option<(&RepositoryBootstrapClaimConditionEntry, u8)> = None;
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
    pub fn binding_dimension(&self) -> Option<M5RepositoryBootstrapClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The claim this family effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5RepositoryBootstrapA11yClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_ready_claim,
        }
    }

    /// AC / auto-narrowing honesty: a partially-disclosed checkout plan, an unconfirmed staged-trust fence, or
    /// an aged-out / policy-blocked bootstrap-evidence / mirror-signer continuity can no longer keep an old
    /// `TrustedAcquisitionSurface` / `ReviewableAcquisitionSurface` label. The effective claim never exceeds the
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

    /// AC / trusted honesty: an unconfirmed-staged-trust / aged-out-bootstrap-evidence state never keeps a
    /// trusted claim — repository-bootstrap meaning is never conveyed by an entry-chrome-only affordance, a
    /// mislabeled screenshot, or an unlabeled control alone. When such a state is modeled, the effective claim
    /// must not assert `TrustedAcquisitionSurface`.
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
            && !self.repository_bootstrap_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.high_zoom_reach.never_traps()
            && self.high_contrast_reach.never_traps()
            && self.localization_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_structure_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the repository-bootstrap meaning without leaking a raw payload.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_payload()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: every unverified projection preserves the underlying source-locator / checkout-plan /
    /// credential-posture / staged-trust / bootstrap-evidence truth. The row must assert `truth_preserved`, and
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

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced
    /// repository-bootstrap projection and keeps its labels, so product / help / release publication stay
    /// aligned on the same narrowed state.
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
        let primary = family_primary_dimension(self.repository_bootstrap_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5RepositoryBootstrapRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> RepositoryBootstrapAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.trusted_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_truth_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return RepositoryBootstrapAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            RepositoryBootstrapAccessibilityStatus::NarrowedDisclosed
        } else {
            RepositoryBootstrapAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == REPOSITORY_BOOTSTRAP_A11Y_ROW_RECORD_KIND
            && self.schema_version == REPOSITORY_BOOTSTRAP_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.repository_bootstrap_context_ref.trim().is_empty()
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
            family = self.repository_bootstrap_family.as_str(),
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

/// Rolled-up summary of an M05-1194 repository-bootstrap accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBootstrapAccessibilitySummary {
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

/// Constructor input for [`RepositoryBootstrapAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryBootstrapAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<RepositoryBootstrapAccessibilityRow>,
}

/// Checked-in M05-1194 repository-bootstrap accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBootstrapAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<RepositoryBootstrapAccessibilityRow>,
    pub summary: RepositoryBootstrapAccessibilitySummary,
}

impl RepositoryBootstrapAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: RepositoryBootstrapAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: REPOSITORY_BOOTSTRAP_A11Y_SCHEMA_VERSION,
            record_kind: REPOSITORY_BOOTSTRAP_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: RepositoryBootstrapAccessibilitySummary {
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
    pub fn represented_families(&self) -> BTreeSet<M5RepositoryBootstrapFamily> {
        self.rows
            .iter()
            .map(|r| r.repository_bootstrap_family)
            .collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5RepositoryBootstrapClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5RepositoryBootstrapConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5RepositoryBootstrapA11yClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5RepositoryBootstrapConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> RepositoryBootstrapAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5RepositoryBootstrapConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let structure_heavy: Vec<&RepositoryBootstrapAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_structure_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                RepositoryBootstrapAccessibilityStatus::Parity => green += 1,
                RepositoryBootstrapAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                RepositoryBootstrapAccessibilityStatus::Stranded => red += 1,
            }
        }

        RepositoryBootstrapAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            structure_heavy_family_count: structure_heavy.len(),
            all_structure_heavy_have_non_visual_fallback: structure_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(RepositoryBootstrapAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(RepositoryBootstrapAccessibilityRow::claim_is_honest),
            all_trusted_honesty_holds: self
                .rows
                .iter()
                .all(RepositoryBootstrapAccessibilityRow::trusted_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(RepositoryBootstrapAccessibilityRow::export_preserves_meaning),
            all_truth_preserved: self
                .rows
                .iter()
                .all(RepositoryBootstrapAccessibilityRow::preserves_truth_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(RepositoryBootstrapAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<RepositoryBootstrapAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != REPOSITORY_BOOTSTRAP_A11Y_SCHEMA_VERSION {
            violations.push(RepositoryBootstrapAccessibilityViolation::SchemaVersion {
                expected: REPOSITORY_BOOTSTRAP_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != REPOSITORY_BOOTSTRAP_A11Y_RECORD_KIND {
            violations.push(RepositoryBootstrapAccessibilityViolation::RecordKind {
                expected: REPOSITORY_BOOTSTRAP_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(RepositoryBootstrapAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(RepositoryBootstrapAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.repository_bootstrap_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_shown_trusted())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(RepositoryBootstrapAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    RepositoryBootstrapAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.repository_bootstrap_family),
                    },
                );
            }

            // Each row must preserve every mandatory repository-bootstrap label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    RepositoryBootstrapAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A structure-heavy family must render a structured projection *and* a non-visual path.
            if row.is_structure_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5RepositoryBootstrapFallbackModality::Structured)
            {
                violations.push(
                    RepositoryBootstrapAccessibilityViolation::StructureHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: claim never over-asserts a trusted / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(
                    RepositoryBootstrapAccessibilityViolation::ClaimOverAsserted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / trusted honesty: an unconfirmed-staged-trust / aged-out-bootstrap-evidence state never keeps
            // a trusted claim.
            if !row.trusted_honesty_holds() {
                violations.push(
                    RepositoryBootstrapAccessibilityViolation::WeakStateShownAsTrusted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    RepositoryBootstrapAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(
                    RepositoryBootstrapAccessibilityViolation::ExportRequiresRawPayload {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: weakened states preserve source-locator / checkout-plan / credential-posture /
            // staged-trust / bootstrap-evidence truth.
            if !row.preserves_truth_continuity() {
                violations.push(RepositoryBootstrapAccessibilityViolation::TruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    RepositoryBootstrapAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    RepositoryBootstrapAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == RepositoryBootstrapAccessibilityStatus::Stranded {
                violations.push(RepositoryBootstrapAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5RepositoryBootstrapFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    RepositoryBootstrapAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5RepositoryBootstrapClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    RepositoryBootstrapAccessibilityViolation::MissingDimensionCoverage {
                        dimension,
                    },
                );
            }
        }

        // Coverage: every condition state (the fully-qualified baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5RepositoryBootstrapConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    RepositoryBootstrapAccessibilityViolation::MissingConditionStateCoverage {
                        state,
                    },
                );
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing spectrum
        // (trusted → … → bootstrap-evidence-unverified) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5RepositoryBootstrapA11yClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    RepositoryBootstrapAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Trusted honesty must be proven with at least one unconfirmed-staged-trust / aged-out-bootstrap-evidence
        // row in the packet, so the "cannot-prove never shown as trusted" guarantee is exercised end-to-end.
        if !has_unprovable_row {
            violations.push(RepositoryBootstrapAccessibilityViolation::TrustedHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the acquisition-engine, shell, workspace-service,
        // git-service, trust-service, diagnostics, docs/help, CLI-export, and support-export surfaces — so every
        // consumer surface is exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5RepositoryBootstrapConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    RepositoryBootstrapAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(RepositoryBootstrapAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("repository-bootstrap accessibility parity packet serializes"),
        ) {
            violations.push(
                RepositoryBootstrapAccessibilityViolation::RawRepositoryBootstrapMaterialInExport,
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
        serde_json::to_string_pretty(self)
            .expect("repository-bootstrap accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,repository_bootstrap_family,keyboard_reach,screen_reader_reach,high_zoom_reach,high_contrast_reach,localization_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{high_zoom},{high_contrast},{localization},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.repository_bootstrap_family.as_str(),
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
        out.push_str("# M5 Repository-Bootstrap Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5RepositoryBootstrapFamily::ALL.len(),
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
                row.repository_bootstrap_family.as_str(),
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

/// Reads and validates the checked-in repository-bootstrap accessibility parity export.
pub fn current_m5_repository_bootstrap_a11y_export(
) -> Result<RepositoryBootstrapAccessibilityPacket, RepositoryBootstrapAccessibilityArtifactError> {
    let packet: RepositoryBootstrapAccessibilityPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-repository-bootstrap-accessibility-parity/support_export.json"
    )))
        .map_err(RepositoryBootstrapAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RepositoryBootstrapAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in repository-bootstrap accessibility parity export.
#[derive(Debug)]
pub enum RepositoryBootstrapAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RepositoryBootstrapAccessibilityViolation>),
}

impl fmt::Display for RepositoryBootstrapAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "repository-bootstrap accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "repository-bootstrap accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for RepositoryBootstrapAccessibilityArtifactError {}

/// Validation failure for M05-1194 repository-bootstrap accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryBootstrapAccessibilityViolation {
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
        dimension: M5RepositoryBootstrapClaimDimension,
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
        family: M5RepositoryBootstrapFamily,
    },
    MissingDimensionCoverage {
        dimension: M5RepositoryBootstrapClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5RepositoryBootstrapConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5RepositoryBootstrapA11yClaim,
    },
    TrustedHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5RepositoryBootstrapConsumerSurface,
    },
    SummaryMismatch,
    RawRepositoryBootstrapMaterialInExport,
}

impl RepositoryBootstrapAccessibilityViolation {
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
            Self::RawRepositoryBootstrapMaterialInExport => {
                "raw_repository_bootstrap_material_in_export"
            }
        }
    }
}

impl fmt::Display for RepositoryBootstrapAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory repository-bootstrap label")
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
                    "row {id} shows an unconfirmed-staged-trust / aged-out-bootstrap-evidence state as a trusted acquisition surface"
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
                    "row {id} does not preserve source-locator / checkout-plan / credential-posture / staged-trust / bootstrap-evidence truth across narrowing"
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
                    "repository-bootstrap family {family:?} is not certified in the packet"
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
                    "no unconfirmed-staged-trust / aged-out-bootstrap-evidence row is present to prove the trusted-honesty guarantee"
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
            Self::RawRepositoryBootstrapMaterialInExport => {
                write!(f, "export contains raw repository-bootstrap material")
            }
        }
    }
}

impl Error for RepositoryBootstrapAccessibilityViolation {}

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

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Mirrors the frozen
/// repository-bootstrap matrix's own boundary policy (see [`crate::m5_repository_bootstrap_matrix`]): raw secret
/// values and private endpoints stay outside the export boundary, so this heuristic targets raw credential
/// blobs, bearer tokens, key blocks, and endpoint URLs while the repository-bootstrap grammar carries only typed
/// class tokens and opaque refs.
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
pub const REPOSITORY_BOOTSTRAP_A11Y_PACKET_ID: &str =
    "m5-repository-bootstrap-accessibility-parity:stable:0001";

/// Builds the canonical, checked-in repository-bootstrap accessibility parity packet. This is the one source of
/// truth shared by the tests and the on-disk support export so both stay byte-aligned.
pub fn seeded_m5_repository_bootstrap_a11y_packet() -> RepositoryBootstrapAccessibilityPacket {
    RepositoryBootstrapAccessibilityPacket::new(RepositoryBootstrapAccessibilityPacketInput {
        packet_id: REPOSITORY_BOOTSTRAP_A11Y_PACKET_ID.to_owned(),
        as_of: "2026-07-14T00:00:00Z".to_owned(),
        matrix_ref: REPOSITORY_BOOTSTRAP_A11Y_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:repository-bootstrap-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5RepositoryBootstrapRequiredLabel> {
    M5RepositoryBootstrapRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> RepositoryBootstrapCopyExportParity {
    RepositoryBootstrapCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5RepositoryBootstrapClaimDimension,
    state: M5RepositoryBootstrapConditionState,
) -> RepositoryBootstrapClaimConditionEntry {
    RepositoryBootstrapClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and the acquisition
/// engine — so the narrowed state always reaches headless field triage.
fn base_consumers(
    extra: &[M5RepositoryBootstrapConsumerSurface],
) -> Vec<M5RepositoryBootstrapConsumerSurface> {
    let mut out = vec![
        M5RepositoryBootstrapConsumerSurface::SupportExport,
        M5RepositoryBootstrapConsumerSurface::AcquisitionEngine,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps full label
/// and summary parity on the narrower surfaces; a narrowed row discloses the reduced projection it drops
/// there.
fn surface_disclosures(
    labels: &[&str],
    state: RepositoryBootstrapNarrowingDisclosureState,
) -> Vec<RepositoryBootstrapRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        RepositoryBootstrapRenderingNarrowingDisclosure {
            rendering_surface: M5RepositoryBootstrapRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["entry_chrome_pointer_affordance".to_owned()],
        },
        RepositoryBootstrapRenderingNarrowingDisclosure {
            rendering_surface: M5RepositoryBootstrapRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_repository_bootstrap_fetch_transition".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<RepositoryBootstrapRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        RepositoryBootstrapNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced projection while
/// preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<RepositoryBootstrapRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        RepositoryBootstrapNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5RepositoryBootstrapRenderingSurface> {
    vec![
        M5RepositoryBootstrapRenderingSurface::DesktopFull,
        M5RepositoryBootstrapRenderingSurface::CliHeadless,
        M5RepositoryBootstrapRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5RepositoryBootstrapFallbackModality> {
    vec![
        M5RepositoryBootstrapFallbackModality::List,
        M5RepositoryBootstrapFallbackModality::Textual,
        M5RepositoryBootstrapFallbackModality::Cli,
    ]
}

fn structured_modalities() -> Vec<M5RepositoryBootstrapFallbackModality> {
    vec![
        M5RepositoryBootstrapFallbackModality::Structured,
        M5RepositoryBootstrapFallbackModality::List,
        M5RepositoryBootstrapFallbackModality::Textual,
        M5RepositoryBootstrapFallbackModality::Cli,
    ]
}

const REACHABLE: RepositoryBootstrapNonVisualReachState =
    RepositoryBootstrapNonVisualReachState::ReachableAndLabeled;
const REDUCED: RepositoryBootstrapNonVisualReachState =
    RepositoryBootstrapNonVisualReachState::DisclosedReducedButReachable;

fn seeded_rows() -> Vec<RepositoryBootstrapAccessibilityRow> {
    vec![
        // Open local (a local checkout already on disk is located and opened, never recloned over) — the
        // open-local family locates its source and resolves the existing checkout root, so it is a trusted
        // acquisition surface reachable on every surface with no narrowing (green). Not structure-heavy: it
        // exposes a flat list / textual / CLI path.
        RepositoryBootstrapAccessibilityRow {
            record_kind: REPOSITORY_BOOTSTRAP_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: REPOSITORY_BOOTSTRAP_A11Y_SCHEMA_VERSION,
            row_id: "a11y:open-local-source-locator-resolved".to_owned(),
            repository_bootstrap_family: M5RepositoryBootstrapFamily::OpenLocal,
            source_family_schema_ref: M5RepositoryBootstrapFamily::OpenLocal
                .canonical_domain_schema_ref()
                .to_owned(),
            repository_bootstrap_context_ref: "acquisition-engine:open-local:0001".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            localization_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: RepositoryBootstrapExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:open-local-source-locator-resolved:a11y".to_owned(),
            copy_export: copy_export(&[
                "repository_bootstrap_identity",
                "semantic_role",
                "registry_reference",
                "source_locator",
            ]),
            full_ready_claim: M5RepositoryBootstrapA11yClaim::TrustedAcquisitionSurface,
            claim_conditions: vec![condition(
                M5RepositoryBootstrapClaimDimension::SourceLocatorClarity,
                M5RepositoryBootstrapConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "repository_bootstrap_identity",
                "semantic_role",
                "source_locator",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5RepositoryBootstrapConsumerSurface::ShellUi,
                M5RepositoryBootstrapConsumerSurface::WorkspaceService,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.16 — Open a local checkout".to_owned(),
                REPOSITORY_BOOTSTRAP_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-14T00:00:00Z".to_owned(),
            evidence_refs: ev("open-local-source-locator-resolved"),
        },
        // Clone remote (checkout cost, topology, and credential posture shown before the fetch) — the
        // clone-remote family exposes its checkout plan and credential posture as an inspectable read-only
        // reference an operator can review before any network access, so it is a self-sufficient reviewable
        // acquisition surface, but its narrower non-visual traversal discloses a reduced high-zoom reflow walk
        // of the dense checkout-cost table (yellow). Structure-heavy: no — it exposes a flat list / textual
        // path.
        RepositoryBootstrapAccessibilityRow {
            record_kind: REPOSITORY_BOOTSTRAP_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: REPOSITORY_BOOTSTRAP_A11Y_SCHEMA_VERSION,
            row_id: "a11y:clone-remote-credential-posture-disclosed".to_owned(),
            repository_bootstrap_family: M5RepositoryBootstrapFamily::CloneRemote,
            source_family_schema_ref: M5RepositoryBootstrapFamily::CloneRemote
                .canonical_domain_schema_ref()
                .to_owned(),
            repository_bootstrap_context_ref: "git-service:clone-remote:0002".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REDUCED,
            high_contrast_reach: REACHABLE,
            localization_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: RepositoryBootstrapExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:clone-remote-credential-posture-disclosed:a11y".to_owned(),
            copy_export: copy_export(&[
                "repository_bootstrap_identity",
                "semantic_role",
                "registry_reference",
                "credential_posture",
            ]),
            full_ready_claim: M5RepositoryBootstrapA11yClaim::ReviewableAcquisitionSurface,
            claim_conditions: vec![condition(
                M5RepositoryBootstrapClaimDimension::CredentialPostureClarity,
                M5RepositoryBootstrapConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "repository_bootstrap_identity",
                "semantic_role",
                "credential_posture",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5RepositoryBootstrapConsumerSurface::GitService,
                M5RepositoryBootstrapConsumerSurface::ShellUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.16 — Clone a remote source".to_owned(),
                REPOSITORY_BOOTSTRAP_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-14T00:00:00Z".to_owned(),
            evidence_refs: ev("clone-remote-credential-posture-disclosed"),
        },
        // Open archive (checkout / extraction-plan proof partially disclosed) — the open-archive family's
        // checkout / extraction-plan proof can only be partially disclosed, so it auto-narrows to a
        // checkout-plan-disclosed projection that discloses the partial checkout-plan proof alongside the
        // last-known cost / topology placeholders, never a hydrated checkout shown as exact when its plan proof
        // is incomplete (yellow). Its localized traversal narrows the localization path to a disclosed
        // reduction. Structure-heavy: its extraction-plan table binds to a flat list / textual path. A partial
        // checkout-plan disclosure is an honest disclosed-absence operation, not a trusted overstatement.
        RepositoryBootstrapAccessibilityRow {
            record_kind: REPOSITORY_BOOTSTRAP_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: REPOSITORY_BOOTSTRAP_A11Y_SCHEMA_VERSION,
            row_id: "a11y:open-archive-checkout-plan-disclosed-partial".to_owned(),
            repository_bootstrap_family: M5RepositoryBootstrapFamily::OpenArchive,
            source_family_schema_ref: M5RepositoryBootstrapFamily::OpenArchive
                .canonical_domain_schema_ref()
                .to_owned(),
            repository_bootstrap_context_ref: "diagnostics:open-archive:0003".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            localization_reach: REDUCED,
            cli_reach: REACHABLE,
            export_summary: RepositoryBootstrapExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:open-archive-checkout-plan-disclosed-partial:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "repository_bootstrap_identity",
                "semantic_role",
                "registry_reference",
                "checkout_plan",
            ]),
            full_ready_claim: M5RepositoryBootstrapA11yClaim::TrustedAcquisitionSurface,
            claim_conditions: vec![condition(
                M5RepositoryBootstrapClaimDimension::CheckoutPlanClarity,
                M5RepositoryBootstrapConditionState::CheckoutPlanDisclosedPartial,
            )],
            claim_narrow: Some(RepositoryBootstrapClaimAutoNarrow {
                narrowed_to: M5RepositoryBootstrapA11yClaim::CheckoutPlanDisclosedProjection,
                binding_dimension: M5RepositoryBootstrapClaimDimension::CheckoutPlanClarity,
                trigger: M5RepositoryBootstrapDowngradeTrigger::ProofStale,
                narrowed_label:
                    "This open-archive acquisition can only disclose a partial checkout / extraction-plan proof — shown as a checkout-plan-disclosed projection that discloses the partial checkout-plan proof alongside the last-known cost and topology placeholders, never presenting a hydrated checkout as exact when its plan proof is incomplete"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "repository_bootstrap_identity",
                "semantic_role",
                "checkout_plan",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5RepositoryBootstrapConsumerSurface::Diagnostics,
                M5RepositoryBootstrapConsumerSurface::CliExport,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.17 — Open an archive container".to_owned(),
                REPOSITORY_BOOTSTRAP_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-14T00:00:00Z".to_owned(),
            evidence_refs: ev("open-archive-checkout-plan-disclosed-partial"),
        },
        // Import bundle (staged-trust / no-implicit-execution fence unconfirmed) — the import-bundle family's
        // staged-trust fence cannot be confirmed, so it auto-narrows to a trust-stage-unverified projection that
        // keeps the last-known deferred-repo-owned-action posture explicit, never a bundle shown as staged when
        // a repo-owned action may have run implicitly (yellow). Its forced-colors response narrows the
        // high-contrast path to a disclosed reduction. Structure-heavy: its staged-trust table binds to a flat
        // list / textual path.
        RepositoryBootstrapAccessibilityRow {
            record_kind: REPOSITORY_BOOTSTRAP_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: REPOSITORY_BOOTSTRAP_A11Y_SCHEMA_VERSION,
            row_id: "a11y:import-bundle-trust-stage-unconfirmed".to_owned(),
            repository_bootstrap_family: M5RepositoryBootstrapFamily::ImportBundle,
            source_family_schema_ref: M5RepositoryBootstrapFamily::ImportBundle
                .canonical_domain_schema_ref()
                .to_owned(),
            repository_bootstrap_context_ref: "trust-service:import-bundle:0004".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REDUCED,
            localization_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: RepositoryBootstrapExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:import-bundle-trust-stage-unconfirmed:a11y".to_owned(),
            copy_export: copy_export(&[
                "repository_bootstrap_identity",
                "semantic_role",
                "registry_reference",
                "staged_trust",
            ]),
            full_ready_claim: M5RepositoryBootstrapA11yClaim::TrustedAcquisitionSurface,
            claim_conditions: vec![condition(
                M5RepositoryBootstrapClaimDimension::TrustStageFenceClarity,
                M5RepositoryBootstrapConditionState::TrustStageUnconfirmed,
            )],
            claim_narrow: Some(RepositoryBootstrapClaimAutoNarrow {
                narrowed_to: M5RepositoryBootstrapA11yClaim::TrustStageUnverifiedProjection,
                binding_dimension: M5RepositoryBootstrapClaimDimension::TrustStageFenceClarity,
                trigger: M5RepositoryBootstrapDowngradeTrigger::StagedTrustRuleUnstated,
                narrowed_label:
                    "This import-bundle acquisition cannot confirm that its staged-trust fence held — shown as a trust-stage-unverified projection that keeps the last-known deferred-repo-owned-action posture explicit, never presenting a bundle as staged when a repo hook, task, extension, restore, or generator may have run implicitly during acquisition"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "repository_bootstrap_identity",
                "semantic_role",
                "staged_trust",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5RepositoryBootstrapConsumerSurface::TrustService,
                M5RepositoryBootstrapConsumerSurface::Diagnostics,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.17 — Import a bundle with staged trust".to_owned(),
                REPOSITORY_BOOTSTRAP_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-14T00:00:00Z".to_owned(),
            evidence_refs: ev("import-bundle-trust-stage-unconfirmed"),
        },
        // Resume snapshot (bootstrap / mirror-signer evidence aged out or policy-blocked) — the resume-snapshot
        // family's bootstrap / mirror-signer evidence has aged out or is policy-blocked, so it auto-narrows to a
        // bootstrap-evidence-unverified projection that keeps the last-known signer / digest / partial-root
        // state explicit, never a mirror shown as signer-continuous when its evidence has aged out or become
        // policy-blocked (yellow). Structure-heavy: its evidence-lineage table binds to a flat list / textual
        // path.
        RepositoryBootstrapAccessibilityRow {
            record_kind: REPOSITORY_BOOTSTRAP_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: REPOSITORY_BOOTSTRAP_A11Y_SCHEMA_VERSION,
            row_id: "a11y:resume-snapshot-bootstrap-evidence-unconfirmed".to_owned(),
            repository_bootstrap_family: M5RepositoryBootstrapFamily::ResumeSnapshot,
            source_family_schema_ref: M5RepositoryBootstrapFamily::ResumeSnapshot
                .canonical_domain_schema_ref()
                .to_owned(),
            repository_bootstrap_context_ref: "acquisition-engine:resume-snapshot:0005".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            localization_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: RepositoryBootstrapExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:resume-snapshot-bootstrap-evidence-unconfirmed:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "repository_bootstrap_identity",
                "semantic_role",
                "registry_reference",
                "bootstrap_evidence",
            ]),
            full_ready_claim: M5RepositoryBootstrapA11yClaim::TrustedAcquisitionSurface,
            claim_conditions: vec![condition(
                M5RepositoryBootstrapClaimDimension::BootstrapEvidenceClarity,
                M5RepositoryBootstrapConditionState::BootstrapEvidenceUnconfirmed,
            )],
            claim_narrow: Some(RepositoryBootstrapClaimAutoNarrow {
                narrowed_to: M5RepositoryBootstrapA11yClaim::BootstrapEvidenceUnverifiedProjection,
                binding_dimension: M5RepositoryBootstrapClaimDimension::BootstrapEvidenceClarity,
                trigger:
                    M5RepositoryBootstrapDowngradeTrigger::LostSignerOrMirrorProvenanceAcrossOfflineOrMirroredFetches,
                narrowed_label:
                    "This resume-snapshot acquisition cannot confirm current bootstrap or mirror-signer evidence — shown as a bootstrap-evidence-unverified projection that keeps the last-known signer, digest, and partial-root state explicit, never presenting a mirror as signer-continuous when its provenance evidence has aged out or become policy-blocked"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "repository_bootstrap_identity",
                "semantic_role",
                "bootstrap_evidence",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5RepositoryBootstrapConsumerSurface::DocsHelp,
                M5RepositoryBootstrapConsumerSurface::GitService,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.17 — Resume a partial-acquisition snapshot".to_owned(),
                REPOSITORY_BOOTSTRAP_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-14T00:00:00Z".to_owned(),
            evidence_refs: ev("resume-snapshot-bootstrap-evidence-unconfirmed"),
        },
    ]
}
