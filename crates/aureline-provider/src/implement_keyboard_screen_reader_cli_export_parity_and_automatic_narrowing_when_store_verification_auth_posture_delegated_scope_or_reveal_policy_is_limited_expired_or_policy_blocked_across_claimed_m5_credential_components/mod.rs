//! Keyboard / screen-reader / CLI / export parity and honest automatic narrowing for the
//! M5 credential components (credential-state row, secret-access-prompt sheet, vault-or-keychain
//! picker, credential-store-capability row, browser/device-code handoff card, delegated-credential
//! row, rotation/revoke-event row, and export-safety banner).
//!
//! This module is the M05-994 accessibility-and-auto-narrowing capstone over the frozen M5
//! credential component matrix
//! ([`crate::freeze_the_m5_credential_component_matrix`]). Where the freeze matrix defines the
//! reusable credential component primitives, and the 989-993 implementation / consumer lanes
//! resolve their per-surface truth, this lane certifies — per component family — that
//! credential claims stay **keyboard-complete, assistive-tech-reachable, CLI/export-safe, and
//! self-narrowing** rather than presenting an unverified store, an expired auth posture, a drifted
//! delegated scope, or a policy-blocked reveal as a still verified, current, fully brokered
//! credential surface:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
//!   screen-reader-reachable, and CLI/headless-reachable path into the same canonical credential
//!   identity, storage mode, handle-only-versus-raw-reveal posture, local-versus-forwarded /
//!   delegated identity, expiry / lifecycle state, and raw-secret-excluded export boundary the
//!   rich component shows — never a hover-only chip that strands assistive-tech or headless users.
//!   Hierarchy-heavy families (the export-safety banner's nested export-surface / excluded-field /
//!   redaction-posture lineage) additionally bind their tree to a flat list / textual path.
//! - **Export parity.** The support / release / evaluation export reconstructs each component's
//!   meaning from typed tokens and opaque refs without a screenshot, preserving the same
//!   canonical IDs, storage modes, reveal postures, delegated-identity labels, expiry states,
//!   export-safety boundaries, and narrowing reasons shown in-product so credential truth can be
//!   reconstructed without screenshots or private team memory — and never a raw secret.
//! - **Honest auto-narrowing.** When store verification is missing, auth posture is expired,
//!   delegated scope has drifted, or reveal policy is blocked by deployment / profile policy, the
//!   component's credential claim auto-narrows from `VerifiedBrokered` / `HandleReadyProjection`
//!   to an unverified-store / expired-auth / drifted-delegation / reveal-blocked projection,
//!   discloses the narrowing with a precise trigger and binding dimension, and preserves the
//!   canonical identity / storage / delegation / expiry lineage — the underlying credential
//!   lineage is never dropped opaquely. A component with every dimension intact must NOT carry a
//!   spurious narrowing, and an unverified, expired, or reveal-blocked state can never keep a
//!   verified-brokered claim.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the credential-settings,
//!   secret-prompt, vault-picker, device-code-handoff, delegated-identity, status-bar, general
//!   product UI, headless CLI, and support / release exports so product, docs, and release
//!   publication stay aligned on credential-boundary downgrade behavior rather than drifting in
//!   copy — a verified-looking surface can never outrun the store-verification / auth / delegation /
//!   reveal proof it is being viewed away from.
//!
//! Each [`CredentialComponentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_credential_component_matrix::M5CredentialComponentFamily`] and reuses that
//! frozen family vocabulary plus the frozen [`M5CredentialRequiredLabel`] and
//! [`M5CredentialDowngradeTrigger`] and the shared [`M5CredentialConsumerSurface`] consumer
//! surfaces rather than minting parallel synonyms, so the certified labels stay byte-identical to
//! the matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw credentials, tokens, passwords, request bodies, and endpoint
//! secrets never cross this boundary; the packet carries only typed class tokens, opaque
//! credential / store / delegation refs, booleans, and redacted labels so support, release, and
//! diagnostics exports can reconstruct exactly what an accessible fallback would have shown
//! without leaking credential material.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's families,
// required labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::freeze_the_m5_credential_component_matrix::{
    M5CredentialComponentFamily, M5CredentialConsumerSurface, M5CredentialDowngradeTrigger,
    M5CredentialRequiredLabel,
};

/// Schema version stamped on the M05-994 credential component accessibility parity packet.
pub const CREDENTIAL_COMPONENT_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`CredentialComponentAccessibilityPacket`].
pub const CREDENTIAL_COMPONENT_A11Y_RECORD_KIND: &str =
    "m5_credential_component_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`CredentialComponentAccessibilityRow`].
pub const CREDENTIAL_COMPONENT_A11Y_ROW_RECORD_KIND: &str =
    "m5_credential_component_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const CREDENTIAL_COMPONENT_A11Y_SCHEMA_REF: &str =
    "schemas/ui/m5-credential-component-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const CREDENTIAL_COMPONENT_A11Y_DOC_REF: &str =
    "docs/security/m5_credential_component_accessibility_parity.md";

/// Repo-relative path of the frozen credential component matrix this lane certifies.
pub const CREDENTIAL_COMPONENT_A11Y_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-credential-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const CREDENTIAL_COMPONENT_A11Y_FIXTURE_DIR: &str =
    "fixtures/ui/m5-credential-component-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const CREDENTIAL_COMPONENT_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-credential-component-accessibility-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const CREDENTIAL_COMPONENT_A11Y_CSV_REF: &str =
    "artifacts/release/m5-credential-component-accessibility-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const CREDENTIAL_COMPONENT_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-credential-component-accessibility-proof.md";

/// The reusable component families that render a non-linear hierarchy (the export-safety banner's
/// nested export-surface / excluded-field / redaction-posture lineage) and therefore MUST bind
/// their tree to an equivalent flat list / textual path so the hierarchy is navigable
/// non-visually.
const fn family_is_hierarchy_heavy(family: M5CredentialComponentFamily) -> bool {
    matches!(family, M5CredentialComponentFamily::ExportSafetyBanner)
}

/// The credential dimension whose weakening a family primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5CredentialComponentFamily,
) -> M5CredentialComponentClaimDimension {
    match family {
        M5CredentialComponentFamily::CredentialStateRow => {
            M5CredentialComponentClaimDimension::AuthPosture
        }
        M5CredentialComponentFamily::SecretAccessPromptSheet => {
            M5CredentialComponentClaimDimension::RevealPolicy
        }
        M5CredentialComponentFamily::VaultOrKeychainPicker => {
            M5CredentialComponentClaimDimension::StoreVerification
        }
        M5CredentialComponentFamily::CredentialStoreCapabilityRow => {
            M5CredentialComponentClaimDimension::StoreVerification
        }
        M5CredentialComponentFamily::BrowserDeviceCodeHandoffCard => {
            M5CredentialComponentClaimDimension::AuthPosture
        }
        M5CredentialComponentFamily::DelegatedCredentialRow => {
            M5CredentialComponentClaimDimension::DelegatedScope
        }
        M5CredentialComponentFamily::RotationRevokeEventRow => {
            M5CredentialComponentClaimDimension::AuthPosture
        }
        M5CredentialComponentFamily::ExportSafetyBanner => {
            M5CredentialComponentClaimDimension::RevealPolicy
        }
    }
}

/// A rendered fallback modality for a credential component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialComponentFallbackModality {
    /// A rich, structured (nested export-surface / excluded-field / redaction-posture tree)
    /// projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5CredentialComponentFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured
    /// surface (i.e. a keyboard / screen-reader / headless path).
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
/// component may render at desktop-full capability or narrow to a companion, read-only browser,
/// headless CLI, offline handoff packet, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialComponentRenderingSurface {
    /// The full-capability desktop credential surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// An offline handoff packet.
    HandoffPacket,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5CredentialComponentRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop full-capability
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
            Self::HandoffPacket => "handoff_packet",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / CLI reach for a component's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialComponentNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / headless users
    /// (red).
    ViewOnlyTrap,
}

impl CredentialComponentNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / headless users.
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

/// Whether an export-safe summary preserves the component meaning without a screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialComponentExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl CredentialComponentExportSummaryState {
    /// Returns true when the export never falls back to a screenshot alone.
    pub const fn never_screenshot_only(self) -> bool {
        !matches!(self, Self::AbsentNeedsScreenshot)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutScreenshot => "reconstructable_without_screenshot",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::AbsentNeedsScreenshot => "absent_needs_screenshot",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialComponentNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl CredentialComponentNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or actions.
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

/// The credential claim ceiling a component asserts: how strong a credential-boundary posture it
/// lets a surface present. Auto-narrowing lowers this ceiling when a credential dimension weakens
/// so an unverified store, an expired auth posture, a drifted delegated scope, or a policy-blocked
/// reveal can never keep an old `VerifiedBrokered` or `HandleReadyProjection` label — an
/// unverified, expired, or reveal-blocked state never masquerades as verified-brokered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialComponentClaim {
    /// Verified-brokered: a verified store, a current auth posture, an in-scope delegation, and an
    /// allowed reveal / export policy — the strongest claim, a credential Aureline can broker,
    /// use, and (where permitted) reveal right now.
    VerifiedBrokered,
    /// Handle-ready projection: a self-sufficient handle-only projection (usable through its
    /// opaque handle) that is not itself a raw-revealable, fully brokered credential.
    HandleReadyProjection,
    /// Unverified-store projection: the credential store's verification is missing; the surface
    /// must not claim verified storage and stays an unverified-store projection until verified.
    UnverifiedStoreProjection,
    /// Expired-auth projection: the auth posture is expired; the surface cannot present as a
    /// current, usable credential and stays an expired-auth projection until re-authenticated.
    ExpiredAuthProjection,
    /// Drifted-delegation projection: the delegated scope has drifted from what was granted; the
    /// surface cannot present as an in-scope forwarded / delegated identity and stays a
    /// drifted-delegation projection until the scope is reconciled.
    DriftedDelegationProjection,
    /// Reveal-blocked projection: reveal / export policy is blocked by deployment or profile
    /// policy; the surface cannot claim allowed reveal / export and stays a reveal-blocked
    /// projection.
    RevealBlockedProjection,
}

impl M5CredentialComponentClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::VerifiedBrokered,
        Self::HandleReadyProjection,
        Self::UnverifiedStoreProjection,
        Self::ExpiredAuthProjection,
        Self::DriftedDelegationProjection,
        Self::RevealBlockedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger credential posture. Narrowing lowers
    /// rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::VerifiedBrokered => 5,
            Self::HandleReadyProjection => 4,
            Self::UnverifiedStoreProjection => 3,
            Self::ExpiredAuthProjection => 2,
            Self::DriftedDelegationProjection => 1,
            Self::RevealBlockedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully verified, current, brokered credential
    /// surface.
    pub const fn asserts_verified_brokered(self) -> bool {
        matches!(self, Self::VerifiedBrokered)
    }

    /// Returns true when this claim asserts a fully self-sufficient (verified-brokered or
    /// handle-ready) projection.
    pub const fn asserts_full_projection(self) -> bool {
        matches!(self, Self::VerifiedBrokered | Self::HandleReadyProjection)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifiedBrokered => "verified_brokered",
            Self::HandleReadyProjection => "handle_ready_projection",
            Self::UnverifiedStoreProjection => "unverified_store_projection",
            Self::ExpiredAuthProjection => "expired_auth_projection",
            Self::DriftedDelegationProjection => "drifted_delegation_projection",
            Self::RevealBlockedProjection => "reveal_blocked_projection",
        }
    }
}

/// The credential dimension whose state governs how far a component may claim to be a verified,
/// current, brokered credential surface. The four dimensions map 1:1 to the four spec narrowing
/// axes — store verification, auth posture, delegated scope, and reveal policy — so every family
/// carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialComponentClaimDimension {
    /// Store verification: is the component's credential store verified, or is verification
    /// missing?
    StoreVerification,
    /// Auth posture: is the component's auth posture current, or has it expired?
    AuthPosture,
    /// Delegated scope: is the forwarded / delegated identity in scope, or has the delegated scope
    /// drifted?
    DelegatedScope,
    /// Reveal policy: does deployment / profile policy allow the component's reveal / export, or is
    /// it policy-blocked?
    RevealPolicy,
}

impl M5CredentialComponentClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::StoreVerification,
        Self::AuthPosture,
        Self::DelegatedScope,
        Self::RevealPolicy,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StoreVerification => "store_verification",
            Self::AuthPosture => "auth_posture",
            Self::DelegatedScope => "delegated_scope",
            Self::RevealPolicy => "reveal_policy",
        }
    }
}

/// The observed condition of one credential dimension. Anything weaker than
/// [`Self::VerifiedCurrent`] imposes a narrowing ceiling on the component's credential claim. The
/// four spec axes the lane must auto-narrow on — a missing store verification, an expired auth
/// posture, a drifted delegated scope, and a policy-blocked reveal — are [`Self::StoreUnverified`],
/// [`Self::AuthExpired`], [`Self::DelegatedScopeDrifted`], and [`Self::RevealPolicyBlocked`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialComponentConditionState {
    /// Verified, current, in-scope, and reveal-allowed — imposes no ceiling.
    VerifiedCurrent,
    /// The credential store's verification is missing — the surface cannot claim verified storage;
    /// credential claim drops to an unverified-store projection.
    StoreUnverified,
    /// The auth posture is expired — the surface cannot present as current; credential claim drops
    /// to an expired-auth projection.
    AuthExpired,
    /// The delegated scope has drifted from what was granted — the surface cannot present as
    /// in-scope; credential claim drops to a drifted-delegation projection.
    DelegatedScopeDrifted,
    /// Reveal / export policy is blocked by deployment or profile policy — the surface cannot claim
    /// allowed reveal / export; credential claim drops to a reveal-blocked projection.
    RevealPolicyBlocked,
}

impl M5CredentialComponentConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::VerifiedCurrent,
        Self::StoreUnverified,
        Self::AuthExpired,
        Self::DelegatedScopeDrifted,
        Self::RevealPolicyBlocked,
    ];

    /// Returns true when the dimension is weaker than verified-current and therefore imposes a
    /// narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::VerifiedCurrent)
    }

    /// Returns true when the condition reflects an unverified store, expired auth, or
    /// policy-blocked reveal — a state that must never be shown as verified-brokered because it
    /// would silently imply verified storage, current auth, or allowed reveal / export behavior.
    pub const fn is_unverified_expired_or_blocked(self) -> bool {
        matches!(
            self,
            Self::StoreUnverified | Self::AuthExpired | Self::RevealPolicyBlocked
        )
    }

    /// The strongest credential claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5CredentialComponentClaim {
        match self {
            Self::VerifiedCurrent => M5CredentialComponentClaim::VerifiedBrokered,
            Self::StoreUnverified => M5CredentialComponentClaim::UnverifiedStoreProjection,
            Self::AuthExpired => M5CredentialComponentClaim::ExpiredAuthProjection,
            Self::DelegatedScopeDrifted => M5CredentialComponentClaim::DriftedDelegationProjection,
            Self::RevealPolicyBlocked => M5CredentialComponentClaim::RevealBlockedProjection,
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing.
    /// Each state maps to the on-topic frozen trigger the freeze matrix already governs, so the
    /// certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5CredentialDowngradeTrigger {
        match self {
            // The verified baseline never narrows; kept for exhaustiveness.
            Self::VerifiedCurrent => M5CredentialDowngradeTrigger::StoreCapabilityUnstated,
            Self::StoreUnverified => M5CredentialDowngradeTrigger::StoreCapabilityUnstated,
            Self::AuthExpired => M5CredentialDowngradeTrigger::LifecycleStateHidden,
            Self::DelegatedScopeDrifted => M5CredentialDowngradeTrigger::DelegatedIdentityUnstated,
            Self::RevealPolicyBlocked => M5CredentialDowngradeTrigger::RevealPostureUnstated,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifiedCurrent => "verified_current",
            Self::StoreUnverified => "store_unverified",
            Self::AuthExpired => "auth_expired",
            Self::DelegatedScopeDrifted => "delegated_scope_drifted",
            Self::RevealPolicyBlocked => "reveal_policy_blocked",
        }
    }
}

/// One credential dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialComponentClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5CredentialComponentClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5CredentialComponentConditionState,
}

/// An honest credential-claim auto-narrow block. When a credential dimension weakens, the
/// component's credential claim lowers to the permitted ceiling, names the binding dimension and
/// frozen trigger, and preserves the canonical identity / storage / delegation / expiry lineage
/// rather than silently dropping it — the underlying credential lineage is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialComponentClaimAutoNarrow {
    /// The credential claim the component is narrowed to.
    pub narrowed_to: M5CredentialComponentClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5CredentialComponentClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5CredentialDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical credential identity, storage mode, delegated identity, and expiry state are
    /// preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying identity / storage / delegation / expiry lineage is preserved (never
    /// dropped) across the narrowing; must hold so unverified-store, expired-auth,
    /// drifted-delegation, and reveal-blocked states never fail opaquely.
    pub preserves_lineage_continuity: bool,
}

impl CredentialComponentClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and credential
    /// lineage and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_lineage_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be copyable
/// as text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialComponentCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl CredentialComponentCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at
    /// least one export field is named, and screenshots are prohibited as the sole export.
    pub fn is_complete(&self) -> bool {
        self.screenshot_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialComponentRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5CredentialComponentRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: CredentialComponentNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a credential component accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims verification, or drops state
    /// silently (red).
    Stranded,
}

impl CredentialComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one credential component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialComponentAccessibilityRow {
    /// Record kind; must equal [`CREDENTIAL_COMPONENT_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`CREDENTIAL_COMPONENT_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5CredentialComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the credential / store / delegation object this component acts on; stays
    /// visible on every surface, so this is never empty.
    pub credential_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual
    /// (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5CredentialComponentFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical identity, storage mode, reveal
    /// posture, delegated identity, expiry state, and export-safety boundary as the rich surface;
    /// must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: CredentialComponentNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: CredentialComponentNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: CredentialComponentNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: CredentialComponentExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: CredentialComponentCopyExportParity,
    /// The full credential claim this family asserts when every dimension is intact.
    pub full_credential_claim: M5CredentialComponentClaim,
    /// The observed condition of each modeled credential dimension.
    #[serde(default)]
    pub claim_conditions: Vec<CredentialComponentClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's
    /// full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<CredentialComponentClaimAutoNarrow>,
    /// Whether the underlying credential lineage is preserved on this component regardless of
    /// narrowing; must hold so unverified-store, expired-auth, drifted-delegation, and
    /// reveal-blocked states never fail opaquely.
    pub lineage_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5CredentialComponentRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<CredentialComponentRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5CredentialRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5CredentialConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl CredentialComponentAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a flat
    /// non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / cli) fallback modality is
    /// offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `VerifiedCurrent` when the row does not
    /// model that dimension.
    pub fn condition_for(
        &self,
        dimension: M5CredentialComponentClaimDimension,
    ) -> M5CredentialComponentConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5CredentialComponentConditionState::VerifiedCurrent)
    }

    /// Whether any modeled dimension is weaker than verified-current.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest credential claim permitted after applying every modeled dimension's ceiling,
    /// capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5CredentialComponentClaim {
        let mut permitted = self.full_credential_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension
    /// narrows below the family's full claim.
    pub fn binding_condition(&self) -> Option<&CredentialComponentClaimConditionEntry> {
        let mut binding: Option<(&CredentialComponentClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_credential_claim.capability_rank() {
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
    pub fn binding_dimension(&self) -> Option<M5CredentialComponentClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The credential claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5CredentialComponentClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_credential_claim,
        }
    }

    /// AC / auto-narrowing honesty: an unverified store, an expired auth posture, a drifted
    /// delegated scope, or a policy-blocked reveal can no longer keep an old `VerifiedBrokered` /
    /// `HandleReadyProjection` label. The effective claim never exceeds the permitted ceiling;
    /// when a dimension narrows below the full claim, an honest narrow block is present, narrows to
    /// exactly the permitted ceiling, binds to the ceiling-imposing dimension with its frozen
    /// trigger, and preserves canonical identity and credential lineage. When nothing narrows, no
    /// spurious narrow block is present.
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

    /// AC / broker honesty: an unverified, expired, or reveal-blocked state (which would silently
    /// imply verified storage, current auth, or allowed reveal / export) never keeps a
    /// verified-brokered claim. When such a state is modeled, the effective claim must not assert
    /// `VerifiedBrokered`.
    pub fn broker_honesty_holds(&self) -> bool {
        let has_unverified_expired_or_blocked = self
            .claim_conditions
            .iter()
            .any(|c| c.state.is_unverified_expired_or_blocked());
        !(has_unverified_expired_or_blocked && self.effective_claim().asserts_verified_brokered())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical
    /// truth — no keyboard / screen-reader / CLI trap, a hierarchy-heavy family offers a
    /// non-visual fallback, and the export reconstructs meaning without a screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.credential_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_hierarchy_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the component meaning without a screenshot.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_screenshot_only()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: unverified-store, expired-auth, drifted-delegation, and reveal-blocked states
    /// preserve the underlying credential lineage. The row must assert `lineage_preserved`, and
    /// any narrow block must preserve lineage continuity too.
    pub fn preserves_lineage_continuity(&self) -> bool {
        self.lineage_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_lineage_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the component carries an
    /// honest claim narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced
    /// interactivity and keeps its labels, so product / docs / release publication stay aligned
    /// on the same narrowed state.
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
        let primary = family_primary_dimension(self.component_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5CredentialRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> CredentialComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.broker_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_lineage_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return CredentialComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            CredentialComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            CredentialComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == CREDENTIAL_COMPONENT_A11Y_ROW_RECORD_KIND
            && self.schema_version == CREDENTIAL_COMPONENT_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.credential_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} cli={cli} \
export={export} full_claim={full} effective_claim={effective} status={status}",
            family = self.component_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_credential_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-994 credential component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialComponentAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_broker_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_lineage_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`CredentialComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<CredentialComponentAccessibilityRow>,
}

/// Checked-in M05-994 credential component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<CredentialComponentAccessibilityRow>,
    pub summary: CredentialComponentAccessibilitySummary,
}

impl CredentialComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: CredentialComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: CREDENTIAL_COMPONENT_A11Y_SCHEMA_VERSION,
            record_kind: CREDENTIAL_COMPONENT_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: CredentialComponentAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_broker_honesty_holds: false,
                all_export_summaries_preserve_meaning: false,
                all_lineage_preserved: false,
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
    pub fn represented_families(&self) -> BTreeSet<M5CredentialComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5CredentialComponentClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5CredentialComponentConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Credential claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5CredentialComponentClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5CredentialConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> CredentialComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5CredentialConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&CredentialComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                CredentialComponentAccessibilityStatus::Parity => green += 1,
                CredentialComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                CredentialComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        CredentialComponentAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(CredentialComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(CredentialComponentAccessibilityRow::claim_is_honest),
            all_broker_honesty_holds: self
                .rows
                .iter()
                .all(CredentialComponentAccessibilityRow::broker_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(CredentialComponentAccessibilityRow::export_preserves_meaning),
            all_lineage_preserved: self
                .rows
                .iter()
                .all(CredentialComponentAccessibilityRow::preserves_lineage_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(CredentialComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<CredentialComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != CREDENTIAL_COMPONENT_A11Y_SCHEMA_VERSION {
            violations.push(CredentialComponentAccessibilityViolation::SchemaVersion {
                expected: CREDENTIAL_COMPONENT_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != CREDENTIAL_COMPONENT_A11Y_RECORD_KIND {
            violations.push(CredentialComponentAccessibilityViolation::RecordKind {
                expected: CREDENTIAL_COMPONENT_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(CredentialComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_unverified_expired_or_blocked_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(CredentialComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.is_unverified_expired_or_blocked())
            {
                has_unverified_expired_or_blocked_row = true;
            }

            if !row.is_complete() {
                violations.push(CredentialComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    CredentialComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory credential label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    CredentialComponentAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5CredentialComponentFallbackModality::Structured)
            {
                violations.push(
                    CredentialComponentAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts a verified / handle-ready surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(
                    CredentialComponentAccessibilityViolation::ClaimOverAsserted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC2: an unverified / expired / reveal-blocked state never keeps a verified-brokered
            // claim.
            if !row.broker_honesty_holds() {
                violations.push(
                    CredentialComponentAccessibilityViolation::UnverifiedShownAsBrokered {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    CredentialComponentAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(
                    CredentialComponentAccessibilityViolation::ExportRequiresScreenshot {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: unverified-store, expired-auth, drifted-delegation, and reveal-blocked
            // states preserve credential lineage.
            if !row.preserves_lineage_continuity() {
                violations.push(CredentialComponentAccessibilityViolation::LineageDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    CredentialComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    CredentialComponentAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == CredentialComponentAccessibilityStatus::Stranded {
                violations.push(CredentialComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5CredentialComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    CredentialComponentAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5CredentialComponentClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    CredentialComponentAccessibilityViolation::MissingDimensionCoverage {
                        dimension,
                    },
                );
            }
        }

        // Coverage: every condition state (the verified baseline plus each spec narrowing axis)
        // is exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5CredentialComponentConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    CredentialComponentAccessibilityViolation::MissingConditionStateCoverage {
                        state,
                    },
                );
            }
        }

        // Coverage: every credential claim tier appears as an effective claim, so the full
        // narrowing spectrum (verified-brokered → … → reveal-blocked) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5CredentialComponentClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    CredentialComponentAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Broker honesty must be proven with at least one unverified / expired / reveal-blocked row
        // in the packet, so the "unverified / expired / blocked never shown as brokered" guarantee
        // is exercised end-to-end.
        if !has_unverified_expired_or_blocked_row {
            violations.push(CredentialComponentAccessibilityViolation::BrokerHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the credential-settings, secret-prompt,
        // vault-picker, device-code-handoff, delegated-identity, status-bar, product UI, CLI, and
        // support / release exports — so every consumer surface is exercised at least once across
        // the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5CredentialConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    CredentialComponentAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(CredentialComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("credential component accessibility parity packet serializes"),
        ) {
            violations
                .push(CredentialComponentAccessibilityViolation::RawCredentialMaterialInExport);
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
            .expect("credential component accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,keyboard_reach,screen_reader_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_credential_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Credential Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5CredentialComponentFamily::ALL.len(),
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
                row.component_family.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_credential_claim.as_str(),
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

/// Reads and validates the checked-in credential component accessibility parity export.
pub fn current_m5_credential_component_a11y_export(
) -> Result<CredentialComponentAccessibilityPacket, CredentialComponentAccessibilityArtifactError> {
    let packet: CredentialComponentAccessibilityPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-credential-component-accessibility-proof/support_export.json"
    )))
        .map_err(CredentialComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CredentialComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in credential component accessibility parity export.
#[derive(Debug)]
pub enum CredentialComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CredentialComponentAccessibilityViolation>),
}

impl fmt::Display for CredentialComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "credential component accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "credential component accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for CredentialComponentAccessibilityArtifactError {}

/// Validation failure for M05-994 credential component accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CredentialComponentAccessibilityViolation {
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
        dimension: M5CredentialComponentClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    HierarchyHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    UnverifiedShownAsBrokered {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresScreenshot {
        id: String,
    },
    LineageDropped {
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
        family: M5CredentialComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5CredentialComponentClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5CredentialComponentConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5CredentialComponentClaim,
    },
    BrokerHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5CredentialConsumerSurface,
    },
    SummaryMismatch,
    RawCredentialMaterialInExport,
}

impl fmt::Display for CredentialComponentAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory credential label")
            }
            Self::HierarchyHeavyMissingStructured { id } => {
                write!(
                    f,
                    "hierarchy-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts a verified / handle-ready surface for a weakened one, or narrows spuriously"
                )
            }
            Self::UnverifiedShownAsBrokered { id } => {
                write!(
                    f,
                    "row {id} shows an unverified, expired, or reveal-blocked state as verified-brokered"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresScreenshot { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without a screenshot"
                )
            }
            Self::LineageDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve credential lineage across narrowing"
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
                    "component family {family:?} is not certified in the packet"
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
                    "credential claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::BrokerHonestyUnproven => {
                write!(
                    f,
                    "no unverified / expired / reveal-blocked row is present to prove the broker-honesty guarantee"
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
            Self::RawCredentialMaterialInExport => {
                write!(f, "export contains raw credential material")
            }
        }
    }
}

impl Error for CredentialComponentAccessibilityViolation {}

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
            | "read only"
            | "unverified"
            | "expired"
            | "revoked"
            | "drifted"
            | "forwarded"
            | "session only"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Credential vocabulary
/// legitimately says "secret" (as in "raw-secret-excluded") and "api_key" (as a credential class),
/// so those tokens are NOT forbidden here; the check flags actual raw material — passwords,
/// passphrases, bearer tokens, PEM blocks, and embedded URLs.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
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

/// Builds the canonical, checked-in credential component accessibility parity packet. This is the
/// one source of truth shared by the tests and the on-disk support export so both stay
/// byte-aligned.
pub fn seeded_m5_credential_component_a11y_packet() -> CredentialComponentAccessibilityPacket {
    CredentialComponentAccessibilityPacket::new(CredentialComponentAccessibilityPacketInput {
        packet_id: "m5-credential-component-accessibility-parity:stable:0001".to_owned(),
        as_of: "2026-07-09T00:00:00Z".to_owned(),
        matrix_ref: CREDENTIAL_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:credential-component-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5CredentialRequiredLabel> {
    M5CredentialRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> CredentialComponentCopyExportParity {
    CredentialComponentCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5CredentialComponentClaimDimension,
    state: M5CredentialComponentConditionState,
) -> CredentialComponentClaimConditionEntry {
    CredentialComponentClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and CLI
/// inspect — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5CredentialConsumerSurface]) -> Vec<M5CredentialConsumerSurface> {
    let mut out = vec![
        M5CredentialConsumerSurface::SupportExport,
        M5CredentialConsumerSurface::CliInspect,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps
/// full label and summary parity on the narrower surfaces; a narrowed row discloses the reduced
/// interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: CredentialComponentNarrowingDisclosureState,
) -> Vec<CredentialComponentRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        CredentialComponentRenderingNarrowingDisclosure {
            rendering_surface: M5CredentialComponentRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        CredentialComponentRenderingNarrowingDisclosure {
            rendering_surface: M5CredentialComponentRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["raw_secret_reveal".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and
/// summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<CredentialComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        CredentialComponentNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<CredentialComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        CredentialComponentNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5CredentialComponentRenderingSurface> {
    vec![
        M5CredentialComponentRenderingSurface::DesktopFull,
        M5CredentialComponentRenderingSurface::CliHeadless,
        M5CredentialComponentRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<CredentialComponentAccessibilityRow> {
    vec![
        // Credential-state row (auth posture expired) — the credential's auth posture has expired,
        // so the row auto-narrows to an expired-auth projection rather than presenting a live,
        // usable credential, while keeping its canonical identity, storage mode, and reveal
        // posture visible (yellow).
        CredentialComponentAccessibilityRow {
            record_kind: CREDENTIAL_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CREDENTIAL_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:credential-state-row-auth-expired".to_owned(),
            component_family: M5CredentialComponentFamily::CredentialStateRow,
            source_family_schema_ref: CREDENTIAL_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            credential_context_ref: "credential:state-row:0001".to_owned(),
            fallback_modalities: vec![
                M5CredentialComponentFallbackModality::List,
                M5CredentialComponentFallbackModality::Textual,
                M5CredentialComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: CredentialComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:credential-state-row-auth-expired:a11y".to_owned(),
            copy_export: copy_export(&[
                "canonical_id",
                "storage_mode",
                "reveal_posture",
                "keyboard_route",
            ]),
            full_credential_claim: M5CredentialComponentClaim::VerifiedBrokered,
            claim_conditions: vec![condition(
                M5CredentialComponentClaimDimension::AuthPosture,
                M5CredentialComponentConditionState::AuthExpired,
            )],
            claim_narrow: Some(CredentialComponentClaimAutoNarrow {
                narrowed_to: M5CredentialComponentClaim::ExpiredAuthProjection,
                binding_dimension: M5CredentialComponentClaimDimension::AuthPosture,
                trigger: M5CredentialDowngradeTrigger::LifecycleStateHidden,
                narrowed_label:
                    "Auth posture has expired and this credential must be re-authenticated — shown as an expired-auth projection with its canonical ID, storage mode, and reveal posture still preserved, never as a current, usable credential"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "canonical_id",
                "storage_mode",
                "reveal_posture",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CredentialConsumerSurface::CredentialSettingsUi,
                M5CredentialConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §15.21 credential-state rows".to_owned(),
                CREDENTIAL_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("credential-state-row-auth-expired"),
        },
        // Secret-access-prompt sheet (reveal policy blocked) — deployment / profile policy blocks
        // the raw reveal, so the sheet auto-narrows to a reveal-blocked projection that keeps a
        // handle-only path rather than presenting an allowed raw reveal (yellow).
        CredentialComponentAccessibilityRow {
            record_kind: CREDENTIAL_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CREDENTIAL_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:secret-access-prompt-sheet-reveal-blocked".to_owned(),
            component_family: M5CredentialComponentFamily::SecretAccessPromptSheet,
            source_family_schema_ref: CREDENTIAL_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            credential_context_ref: "credential:secret-access-prompt-sheet:0002".to_owned(),
            fallback_modalities: vec![
                M5CredentialComponentFallbackModality::List,
                M5CredentialComponentFallbackModality::Textual,
                M5CredentialComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: CredentialComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:secret-access-prompt-sheet-reveal-blocked:a11y".to_owned(),
            copy_export: copy_export(&[
                "reveal_posture",
                "requested_scope",
                "policy_source",
                "keyboard_route",
            ]),
            full_credential_claim: M5CredentialComponentClaim::VerifiedBrokered,
            claim_conditions: vec![condition(
                M5CredentialComponentClaimDimension::RevealPolicy,
                M5CredentialComponentConditionState::RevealPolicyBlocked,
            )],
            claim_narrow: Some(CredentialComponentClaimAutoNarrow {
                narrowed_to: M5CredentialComponentClaim::RevealBlockedProjection,
                binding_dimension: M5CredentialComponentClaimDimension::RevealPolicy,
                trigger: M5CredentialDowngradeTrigger::RevealPostureUnstated,
                narrowed_label:
                    "Deployment policy blocks the raw reveal and only a handle-only path remains — shown as a reveal-blocked projection that names its reveal posture and policy source, never as an allowed raw reveal"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "reveal_posture",
                "requested_scope",
                "policy_source",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CredentialConsumerSurface::SecretPromptUi,
                M5CredentialConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UX Design System §16.25 secret-access prompts".to_owned(),
                CREDENTIAL_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("secret-access-prompt-sheet-reveal-blocked"),
        },
        // Vault-or-keychain picker (store verification missing) — the target store's verification
        // is missing, so the picker auto-narrows to an unverified-store projection rather than
        // presenting a store that is claimed to be securely verified, while naming the storage mode
        // and verification state (yellow).
        CredentialComponentAccessibilityRow {
            record_kind: CREDENTIAL_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CREDENTIAL_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:vault-or-keychain-picker-store-unverified".to_owned(),
            component_family: M5CredentialComponentFamily::VaultOrKeychainPicker,
            source_family_schema_ref: CREDENTIAL_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            credential_context_ref: "credential:vault-or-keychain-picker:0003".to_owned(),
            fallback_modalities: vec![
                M5CredentialComponentFallbackModality::List,
                M5CredentialComponentFallbackModality::Textual,
                M5CredentialComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: CredentialComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:vault-or-keychain-picker-store-unverified:a11y".to_owned(),
            copy_export: copy_export(&[
                "storage_mode",
                "store_verification_state",
                "portability_note",
                "keyboard_route",
            ]),
            full_credential_claim: M5CredentialComponentClaim::VerifiedBrokered,
            claim_conditions: vec![condition(
                M5CredentialComponentClaimDimension::StoreVerification,
                M5CredentialComponentConditionState::StoreUnverified,
            )],
            claim_narrow: Some(CredentialComponentClaimAutoNarrow {
                narrowed_to: M5CredentialComponentClaim::UnverifiedStoreProjection,
                binding_dimension: M5CredentialComponentClaimDimension::StoreVerification,
                trigger: M5CredentialDowngradeTrigger::StoreCapabilityUnstated,
                narrowed_label:
                    "Target store verification is missing and cannot be claimed as securely verified — shown as an unverified-store projection that names its storage mode and verification state, never as a saved-securely store"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "storage_mode",
                "store_verification_state",
                "portability_note",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CredentialConsumerSurface::VaultPickerUi,
                M5CredentialConsumerSurface::StatusBarUi,
            ]),
            source_refs: vec![
                "UX Design System §27.33 vault/keychain pickers".to_owned(),
                CREDENTIAL_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("vault-or-keychain-picker-store-unverified"),
        },
        // Credential-store-capability row — the store's verification and capabilities are stated
        // and the row is a self-sufficient handle-only projection (usable through its handle, not
        // itself a raw-revealable credential), reachable on every surface (green).
        CredentialComponentAccessibilityRow {
            record_kind: CREDENTIAL_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CREDENTIAL_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:credential-store-capability-row".to_owned(),
            component_family: M5CredentialComponentFamily::CredentialStoreCapabilityRow,
            source_family_schema_ref: CREDENTIAL_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            credential_context_ref: "credential:store-capability-row:0004".to_owned(),
            fallback_modalities: vec![
                M5CredentialComponentFallbackModality::List,
                M5CredentialComponentFallbackModality::Textual,
                M5CredentialComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: CredentialComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:credential-store-capability-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "store_capability",
                "store_verification_state",
                "portability_note",
                "keyboard_route",
            ]),
            full_credential_claim: M5CredentialComponentClaim::HandleReadyProjection,
            claim_conditions: vec![condition(
                M5CredentialComponentClaimDimension::StoreVerification,
                M5CredentialComponentConditionState::VerifiedCurrent,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "store_capability",
                "store_verification_state",
                "portability_note",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CredentialConsumerSurface::VaultPickerUi,
                M5CredentialConsumerSurface::CredentialSettingsUi,
            ]),
            source_refs: vec![
                "TDD §7.11.14 credential-store contracts".to_owned(),
                CREDENTIAL_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("credential-store-capability-row"),
        },
        // Browser/device-code handoff card — the auth posture is current and the handoff boundary
        // is stated, so the card is verified-brokered and reachable on every surface (green).
        CredentialComponentAccessibilityRow {
            record_kind: CREDENTIAL_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CREDENTIAL_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:browser-device-code-handoff-card".to_owned(),
            component_family: M5CredentialComponentFamily::BrowserDeviceCodeHandoffCard,
            source_family_schema_ref: CREDENTIAL_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            credential_context_ref: "credential:browser-device-code-handoff-card:0005".to_owned(),
            fallback_modalities: vec![
                M5CredentialComponentFallbackModality::List,
                M5CredentialComponentFallbackModality::Textual,
                M5CredentialComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: CredentialComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:browser-device-code-handoff-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "auth_handoff_boundary",
                "canonical_id",
                "expiry_state",
                "keyboard_route",
            ]),
            full_credential_claim: M5CredentialComponentClaim::VerifiedBrokered,
            claim_conditions: vec![condition(
                M5CredentialComponentClaimDimension::AuthPosture,
                M5CredentialComponentConditionState::VerifiedCurrent,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "auth_handoff_boundary",
                "canonical_id",
                "expiry_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CredentialConsumerSurface::DeviceCodeHandoffUi,
                M5CredentialConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §18.5 browser/device-code handoffs".to_owned(),
                CREDENTIAL_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("browser-device-code-handoff-card"),
        },
        // Delegated-credential row (delegated scope drifted) — the forwarded / delegated scope has
        // drifted from what was granted, so the row auto-narrows to a drifted-delegation projection
        // rather than presenting an in-scope delegated identity, while naming the source identity
        // and target scope (yellow).
        CredentialComponentAccessibilityRow {
            record_kind: CREDENTIAL_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CREDENTIAL_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:delegated-credential-row-scope-drifted".to_owned(),
            component_family: M5CredentialComponentFamily::DelegatedCredentialRow,
            source_family_schema_ref: CREDENTIAL_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            credential_context_ref: "credential:delegated-credential-row:0006".to_owned(),
            fallback_modalities: vec![
                M5CredentialComponentFallbackModality::List,
                M5CredentialComponentFallbackModality::Textual,
                M5CredentialComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: CredentialComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:delegated-credential-row-scope-drifted:a11y".to_owned(),
            copy_export: copy_export(&[
                "source_identity",
                "target_scope",
                "delegated_identity_origin",
                "keyboard_route",
            ]),
            full_credential_claim: M5CredentialComponentClaim::VerifiedBrokered,
            claim_conditions: vec![condition(
                M5CredentialComponentClaimDimension::DelegatedScope,
                M5CredentialComponentConditionState::DelegatedScopeDrifted,
            )],
            claim_narrow: Some(CredentialComponentClaimAutoNarrow {
                narrowed_to: M5CredentialComponentClaim::DriftedDelegationProjection,
                binding_dimension: M5CredentialComponentClaimDimension::DelegatedScope,
                trigger: M5CredentialDowngradeTrigger::DelegatedIdentityUnstated,
                narrowed_label:
                    "Delegated scope has drifted from what was granted and must be reconciled — shown as a drifted-delegation projection that names its source identity and target scope, never as an in-scope local identity"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "source_identity",
                "target_scope",
                "delegated_identity_origin",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CredentialConsumerSurface::DelegatedIdentityUi,
                M5CredentialConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UX Design System §27.33 delegated credential flows".to_owned(),
                CREDENTIAL_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("delegated-credential-row-scope-drifted"),
        },
        // Rotation/revoke-event row — the credential lifecycle is current and stated, so the row is
        // verified-brokered and reachable on every surface (green).
        CredentialComponentAccessibilityRow {
            record_kind: CREDENTIAL_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CREDENTIAL_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:rotation-revoke-event-row".to_owned(),
            component_family: M5CredentialComponentFamily::RotationRevokeEventRow,
            source_family_schema_ref: CREDENTIAL_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            credential_context_ref: "credential:rotation-revoke-event-row:0007".to_owned(),
            fallback_modalities: vec![
                M5CredentialComponentFallbackModality::List,
                M5CredentialComponentFallbackModality::Textual,
                M5CredentialComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: CredentialComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:rotation-revoke-event-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "canonical_id",
                "lifecycle_state",
                "impacted_workflow",
                "keyboard_route",
            ]),
            full_credential_claim: M5CredentialComponentClaim::VerifiedBrokered,
            claim_conditions: vec![condition(
                M5CredentialComponentClaimDimension::AuthPosture,
                M5CredentialComponentConditionState::VerifiedCurrent,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "canonical_id",
                "lifecycle_state",
                "impacted_workflow",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CredentialConsumerSurface::CredentialSettingsUi,
                M5CredentialConsumerSurface::StatusBarUi,
            ]),
            source_refs: vec![
                "TDD §9.29 rotation/revoke contracts".to_owned(),
                CREDENTIAL_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("rotation-revoke-event-row"),
        },
        // Export-safety banner — hierarchy-heavy (nested export-surface / excluded-field /
        // redaction-posture lineage); the reveal / export policy is current and the raw-secret
        // exclusion is stated, so the banner is a self-sufficient handle-only projection and binds
        // its nested lineage to a flat list / textual path (green).
        CredentialComponentAccessibilityRow {
            record_kind: CREDENTIAL_COMPONENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CREDENTIAL_COMPONENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:export-safety-banner".to_owned(),
            component_family: M5CredentialComponentFamily::ExportSafetyBanner,
            source_family_schema_ref: CREDENTIAL_COMPONENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            credential_context_ref: "credential:export-safety-banner:0008".to_owned(),
            fallback_modalities: vec![
                M5CredentialComponentFallbackModality::Structured,
                M5CredentialComponentFallbackModality::List,
                M5CredentialComponentFallbackModality::Textual,
                M5CredentialComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: CredentialComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: CredentialComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:export-safety-banner:a11y".to_owned(),
            copy_export: copy_export(&[
                "export_surface",
                "raw_secret_excluded",
                "redaction_posture",
                "reveal_posture",
            ]),
            full_credential_claim: M5CredentialComponentClaim::HandleReadyProjection,
            claim_conditions: vec![condition(
                M5CredentialComponentClaimDimension::RevealPolicy,
                M5CredentialComponentConditionState::VerifiedCurrent,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "export_surface",
                "raw_secret_excluded",
                "redaction_posture",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CredentialConsumerSurface::StatusBarUi,
                M5CredentialConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §18.5 export safety banners".to_owned(),
                CREDENTIAL_COMPONENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("export-safety-banner"),
        },
    ]
}
