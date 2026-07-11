//! Two reusable M5 notebook components — the kernel picker row and the kernel origin pill — so a
//! user can see, before acting on a runtime, what kind of kernel each candidate is and where the
//! current kernel physically runs: the kernel picker row names a candidate's kernel class (local
//! interpreter, virtual env, conda env, container, remote, or managed), its environment identity /
//! fingerprint, its locality, its compatibility state, its trust / policy limits, and its
//! last-seen availability, and offers first-class choose / inspect / view-compatibility actions so
//! a user can choose another kernel without losing sight of provenance, compatibility, or trust
//! limits; the kernel origin pill names whether the current kernel is local, SSH, container,
//! devcontainer, managed, or browser-bridge backed, how trusted that origin is, whether its
//! provenance is exact or degraded, and whether reattaching / rerunning after restore or handoff
//! would keep exact continuity — so a kernel change never silently implies exact continuity when
//! the environment fingerprint differs materially.
//!
//! Aureline's frozen notebook-kernel-output component matrix
//! ([`crate::freeze_the_m5_notebook_document_header_kernel_state_strip_kernel_picker_row_kernel_origin_pill_output_trust_banner_output_provenance_chip_group_restart_consequence_card_and_kernel_recovery_card_component_matrix`])
//! names the kernel picker row and the kernel origin pill as two governed component families and
//! freezes their controlled vocabulary — the kernel candidate kinds (`local_interpreter`,
//! `virtual_env`, `conda_env`, `container_kernel`, `remote_kernel`, `managed_kernel`) and kernel
//! selection states (`selected`, `available`, `recommended`, `incompatible`, `unavailable`,
//! `needs_install`) a picker row binds; the kernel origin classes (`local_host`, `ssh_remote`,
//! `container`, `devcontainer`, `managed_workspace`, `browser_bridge`) and kernel origin trust
//! states (`trusted_origin`, `first_party`, `third_party`, `unverified_origin`,
//! `restricted_origin`, `unknown_origin`) a pill binds; the one controlled disposition vocabulary;
//! the surface families; the deployment lines; the consumer surfaces; the accessibility routes;
//! the required labels; and the downgrade triggers. This module *implements* that contract as two
//! co-equal component vectors so a claimed M5 notebook, kernel-manager, debug, review, or CLI
//! surface can project a picker row and an origin pill that keep the same truth.
//!
//! The module has two derived resolvers:
//!
//! 1. [`resolve_kernel_picker_row`] — takes a picker row's candidate kind and selection state and
//!    derives its choice state (currently-selected, recommended, available, needs-setup-first,
//!    incompatible, or unavailable), whether the candidate can be chosen now, whether it is the
//!    current kernel, and which notes the row must carry — so an incompatible, unavailable, or
//!    install-first candidate can never read as a clean, selectable choice.
//! 2. [`resolve_kernel_origin_pill`] — takes a pill's kernel origin class, origin trust state, and
//!    environment fingerprint state and derives its provenance class (exact, degraded, restricted,
//!    or unknown), whether the origin is local, whether reattaching / rerunning would keep exact
//!    continuity, and which notes the pill must carry — so a third-party, unverified, restricted,
//!    or drifted kernel can never imply exact continuity and a local / SSH / container / managed /
//!    browser-bridge origin never collapses into one unlabeled badge.
//!
//! A single controls packet — [`KernelPickerRowKernelOriginPillControlsPacket`] — binds one vector
//! of kernel picker rows and one vector of kernel origin pills to the same candidate, selection,
//! origin, trust, fingerprint, deep-link, and non-visual accessibility vocabulary, so kernel
//! choice truth and kernel origin truth stay distinct and explicit across notebook,
//! kernel-manager, debug, review, headless / export, and support consumers.
//!
//! The component family ([`M5NotebookKernelOutputComponentFamily`]), kernel candidate kind
//! ([`M5KernelCandidateKind`]), kernel selection state ([`M5KernelSelectionState`]), kernel origin
//! class ([`M5KernelOriginClass`]), kernel origin trust state ([`M5KernelOriginTrustState`]),
//! disposition ([`M5NotebookKernelOutputDisposition`]), surface family
//! ([`M5NotebookKernelOutputSurfaceFamily`]), deployment line
//! ([`M5NotebookKernelOutputDeploymentLine`]), consumer surface
//! ([`M5NotebookKernelOutputConsumerSurface`]), accessibility route
//! ([`M5NotebookKernelOutputAccessibilityRoute`]), required label
//! ([`M5NotebookKernelOutputRequiredLabel`]), and downgrade trigger
//! ([`M5NotebookKernelOutputDowngradeTrigger`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the two components
//! themselves: the derived choice and provenance classes, the environment fingerprint state, the
//! bounded picker and pill actions, and the deep-link kinds. No M5 notebook surface invents a
//! second kernel-picker or kernel-origin grammar.
//!
//! Raw notebook payloads, pasted paths, credentials, and private endpoints stay outside the export
//! boundary; every context line, deep-link reference, and component identity is carried only as an
//! opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_kernel_picker_row_kernel_origin_pill_controls,
    seeded_kernel_picker_row_kernel_origin_pill_controls_kernel_origin_pill_degraded,
    seeded_kernel_picker_row_kernel_origin_pill_controls_kernel_picker_row_incompatible,
    KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_PACKET_ID,
};

// The kernel candidate kinds and selection states, the kernel origin classes and trust states, the
// disposition vocabulary, and the surface / deployment / consumer / accessibility / label /
// downgrade vocabularies are frozen once, in the notebook-kernel-output component matrix. This lane
// reuses them verbatim so it never invents a parallel kernel-picker or kernel-origin vocabulary.
pub use crate::freeze_the_m5_notebook_document_header_kernel_state_strip_kernel_picker_row_kernel_origin_pill_output_trust_banner_output_provenance_chip_group_restart_consequence_card_and_kernel_recovery_card_component_matrix::{
    M5KernelCandidateKind, M5KernelOriginClass, M5KernelOriginTrustState, M5KernelSelectionState,
    M5NotebookKernelOutputAccessibilityRoute, M5NotebookKernelOutputComponentFamily,
    M5NotebookKernelOutputConsumerSurface, M5NotebookKernelOutputDeploymentLine,
    M5NotebookKernelOutputDisposition, M5NotebookKernelOutputDowngradeTrigger,
    M5NotebookKernelOutputRequiredLabel, M5NotebookKernelOutputSurfaceFamily,
    M5_KERNEL_ORIGIN_PILL_SCHEMA_REF, M5_KERNEL_PICKER_ROW_SCHEMA_REF,
    M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_DOC_REF, M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`KernelPickerRowKernelOriginPillControlsPacket`].
pub const KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_RECORD_KIND: &str =
    "implement_kernel_picker_rows_and_kernel_origin_pills_with_kernel_class_environment_identity_locality_trust_limits_exact_or_degraded_provenance_and_rerun_reattach_continuity_across_claimed_m5_notebook_surfaces";

/// Schema version for M5 kernel-picker-row / kernel-origin-pill control records.
pub const KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_SCHEMA_REF: &str =
    "schemas/ui/m5-kernel-picker-row-kernel-origin-pill-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_DOC_REF: &str =
    "docs/notebooks/m5_kernel_picker_row_kernel_origin_pill_controls.md";

/// Repo-relative path of the protected fixture directory.
pub const KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_FIXTURE_DIR: &str =
    "fixtures/ui/m5-kernel-picker-row-kernel-origin-pill-controls";

/// Repo-relative path of the checked support-export artifact.
pub const KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_ARTIFACT_REF: &str =
    "artifacts/release/m5-kernel-picker-row-kernel-origin-pill-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_CSV_REF: &str =
    "artifacts/release/m5-kernel-picker-row-kernel-origin-pill-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_REPORT_REF: &str =
    "artifacts/design/m5-kernel-picker-row-kernel-origin-pill.md";

// ---- shared deep-link vocabulary ----------------------------------------

/// The kind of stable deep link a notebook component binds its next step against, so a kernel
/// picker row or kernel origin pill never routes through an ephemeral overlay — every next step is
/// a stable kernel-manager, notebook location, docs, or support-bundle reference the user can
/// reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkKind {
    /// A stable notebook / cell location.
    NotebookLocation,
    /// A stable kernel-manager reference.
    KernelManager,
    /// A stable docs anchor.
    DocsAnchor,
    /// A stable support-bundle anchor.
    SupportBundle,
    /// No deep link is bound (the component names that it routes nowhere).
    NoDeepLink,
}

impl DeepLinkKind {
    /// Every deep-link kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NotebookLocation,
        Self::KernelManager,
        Self::DocsAnchor,
        Self::SupportBundle,
        Self::NoDeepLink,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookLocation => "notebook_location",
            Self::KernelManager => "kernel_manager",
            Self::DocsAnchor => "docs_anchor",
            Self::SupportBundle => "support_bundle",
            Self::NoDeepLink => "no_deep_link",
        }
    }

    /// True when this kind names a resolvable deep-link target.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoDeepLink)
    }
}

// ---- kernel-picker-row vocabulary ---------------------------------------

/// Derived choice state a kernel picker row may present.
///
/// This is the picker honesty axis: the class is derived from the frozen kernel selection state,
/// never asserted, so an incompatible, unavailable, or install-first candidate can never read as a
/// clean, immediately-selectable choice and a user can always tell whether a candidate is the
/// current kernel, a recommended or available choice, or blocked before acting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KernelChoiceState {
    /// The candidate is the currently selected kernel.
    CurrentlySelected,
    /// The candidate is recommended for this notebook.
    RecommendedChoice,
    /// The candidate is available to select now.
    AvailableChoice,
    /// The candidate needs an install / setup step before it can be selected.
    NeedsSetupFirst,
    /// The candidate is incompatible with this notebook.
    IncompatibleChoice,
    /// The candidate is currently unavailable / offline.
    UnavailableChoice,
}

impl KernelChoiceState {
    /// Every choice state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CurrentlySelected,
        Self::RecommendedChoice,
        Self::AvailableChoice,
        Self::NeedsSetupFirst,
        Self::IncompatibleChoice,
        Self::UnavailableChoice,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentlySelected => "currently_selected",
            Self::RecommendedChoice => "recommended_choice",
            Self::AvailableChoice => "available_choice",
            Self::NeedsSetupFirst => "needs_setup_first",
            Self::IncompatibleChoice => "incompatible_choice",
            Self::UnavailableChoice => "unavailable_choice",
        }
    }

    /// True when the candidate can be chosen immediately (already selected, recommended, or
    /// available), as opposed to needing install or being incompatible / unavailable.
    pub const fn is_selectable_now(self) -> bool {
        matches!(
            self,
            Self::CurrentlySelected | Self::RecommendedChoice | Self::AvailableChoice
        )
    }

    /// True when this candidate is the currently selected kernel.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::CurrentlySelected)
    }
}

/// One keyboard-complete default action a kernel picker row offers, so a row never hides its choose
/// / inspect / view-compatibility affordance behind a pointer-only gesture. `ChooseKernel`,
/// `InspectCandidate`, and `ViewCompatibility` are always offered so a user can choose another
/// kernel without losing sight of provenance, compatibility, or trust limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KernelPickerAction {
    /// Choose / attach this candidate kernel (always available).
    ChooseKernel,
    /// Inspect the candidate's environment identity / fingerprint (always available).
    InspectCandidate,
    /// View this candidate's compatibility and trust / policy limits (always available).
    ViewCompatibility,
    /// Keep the currently selected kernel.
    KeepCurrentKernel,
    /// Install / set up an install-first candidate.
    InstallKernel,
    /// Open the stable kernel-manager / notebook / docs / support deep link.
    OpenDeepLink,
}

impl KernelPickerAction {
    /// Every picker action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ChooseKernel,
        Self::InspectCandidate,
        Self::ViewCompatibility,
        Self::KeepCurrentKernel,
        Self::InstallKernel,
        Self::OpenDeepLink,
    ];

    /// The default actions every keyboard-complete picker row must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::ChooseKernel,
        Self::InspectCandidate,
        Self::ViewCompatibility,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChooseKernel => "choose_kernel",
            Self::InspectCandidate => "inspect_candidate",
            Self::ViewCompatibility => "view_compatibility",
            Self::KeepCurrentKernel => "keep_current_kernel",
            Self::InstallKernel => "install_kernel",
            Self::OpenDeepLink => "open_deep_link",
        }
    }
}

/// Disclosures a kernel picker row must carry, derived from the candidate kind and selection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KernelPickerRowDisclosure {
    /// The derived choice state this row may present.
    pub choice_state: KernelChoiceState,
    /// Whether the candidate can be chosen immediately.
    pub is_selectable_now: bool,
    /// Whether this candidate is the currently selected kernel.
    pub is_current: bool,
    /// Whether the row must carry an explicit incompatible note.
    pub needs_incompatible_note: bool,
    /// Whether the row must carry an explicit unavailable note.
    pub needs_unavailable_note: bool,
    /// Whether the row must carry an explicit install-first note.
    pub needs_install_note: bool,
}

/// Resolves the selection truth a kernel picker row may present.
///
/// A `selected` candidate is the currently selected kernel, a `recommended` candidate is a
/// recommended choice, and an `available` candidate is an available choice — all three can be
/// chosen now. A `needs_install` candidate needs setup first, an `incompatible` candidate is
/// incompatible, and an `unavailable` candidate is offline, so a candidate a user cannot cleanly
/// choose right now can never read as an immediately-selectable choice, and each blocked candidate
/// carries its own note.
pub fn resolve_kernel_picker_row(
    _candidate: M5KernelCandidateKind,
    selection: M5KernelSelectionState,
) -> KernelPickerRowDisclosure {
    use KernelChoiceState as Class;
    use M5KernelSelectionState as Sel;

    let choice_state = match selection {
        Sel::Selected => Class::CurrentlySelected,
        Sel::Recommended => Class::RecommendedChoice,
        Sel::Available => Class::AvailableChoice,
        Sel::NeedsInstall => Class::NeedsSetupFirst,
        Sel::Incompatible => Class::IncompatibleChoice,
        Sel::Unavailable => Class::UnavailableChoice,
    };

    KernelPickerRowDisclosure {
        choice_state,
        is_selectable_now: choice_state.is_selectable_now(),
        is_current: choice_state.is_current(),
        needs_incompatible_note: matches!(choice_state, Class::IncompatibleChoice),
        needs_unavailable_note: matches!(choice_state, Class::UnavailableChoice),
        needs_install_note: matches!(choice_state, Class::NeedsSetupFirst),
    }
}

/// A kernel picker row naming a candidate's kernel class, environment identity / fingerprint,
/// locality, compatibility state, trust / policy limits, last-seen availability, its derived choice
/// state, bounded choose / inspect / view-compatibility actions, and a stable deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelPickerRow {
    /// Frozen component this control implements; must be `kernel_picker_row`.
    pub component: M5NotebookKernelOutputComponentFamily,
    /// Stable row id.
    pub row_id: String,
    /// Human-readable row label; required and non-empty.
    pub row_label: String,
    /// Kernel candidate kind, reused from the frozen matrix.
    pub candidate_kind: M5KernelCandidateKind,
    /// Kernel selection state, reused from the frozen matrix.
    pub selection_state: M5KernelSelectionState,
    /// Derived choice state (must equal the resolved state).
    pub choice_state: KernelChoiceState,
    /// Whether the row claims the candidate is selectable now (must equal the derived truth).
    pub claims_selectable_now: bool,
    /// Whether the row claims this candidate is the current kernel (must equal the derived truth).
    pub claims_current: bool,
    /// Incompatible note; required when the candidate is incompatible.
    pub incompatible_note: String,
    /// Unavailable note; required when the candidate is unavailable.
    pub unavailable_note: String,
    /// Install-first note; required when the candidate needs install.
    pub install_note: String,
    /// Kernel class label; always required so a candidate's kernel kind stays explicit and kernel
    /// kinds never collapse into one badge.
    pub kernel_class_label: String,
    /// Environment identity / fingerprint label; always required.
    pub environment_identity_label: String,
    /// Locality label; always required so where the candidate runs stays explicit.
    pub locality_label: String,
    /// Compatibility note; always required so the candidate's compatibility state stays explicit.
    pub compatibility_note: String,
    /// Trust / policy limit note; always required so trust and policy limits are never hover-only.
    pub trust_policy_limit_note: String,
    /// Last-seen availability label; always required.
    pub last_seen_label: String,
    /// Context note; always required so the row names what the choice truth means here.
    pub context_note: String,
    /// Kind of stable deep link this row binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include choose / inspect / view-compatibility).
    pub picker_actions: Vec<KernelPickerAction>,
    /// Dispositions this row binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5NotebookKernelOutputDisposition>,
    /// Downgrade triggers this row can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5NotebookKernelOutputDowngradeTrigger>,
    /// Mandatory labels this row can show (must include the mandatory labels).
    pub required_labels: Vec<M5NotebookKernelOutputRequiredLabel>,
    /// Claimed M5 surface families that render this row.
    pub surface_families: Vec<M5NotebookKernelOutputSurfaceFamily>,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5NotebookKernelOutputDeploymentLine>,
    /// Non-visual accessibility routes this row offers.
    pub accessibility_routes: Vec<M5NotebookKernelOutputAccessibilityRoute>,
    /// Notebook subsystems that consume this row's projection.
    pub consumer_surfaces: Vec<M5NotebookKernelOutputConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never collapses local / SSH / container / managed / browser-bridge kernels
    /// into one badge. MUST be `false`.
    pub collapses_kernel_origins_into_one_badge: bool,
    /// Hard invariant: never implies exact continuity when the environment fingerprint differs
    /// materially. MUST be `false`.
    pub implies_exact_continuity_on_material_drift: bool,
    /// Hard invariant: never hides trust or compatibility behind a hover-only affordance. MUST be
    /// `false`.
    pub hides_trust_or_compatibility_behind_hover_only: bool,
    /// Hard invariant: never overwrites resolved provenance with lower-confidence provenance
    /// without review. MUST be `false`.
    pub overwrites_provenance_without_review: bool,
}

impl KernelPickerRow {
    /// Selection disclosures this row must carry, derived from the frozen states.
    pub fn choice_disclosure(&self) -> KernelPickerRowDisclosure {
        resolve_kernel_picker_row(self.candidate_kind, self.selection_state)
    }

    /// Whether the row offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<KernelPickerAction> = self.picker_actions.iter().copied().collect();
        KernelPickerAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5NotebookKernelOutputRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5NotebookKernelOutputRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the row offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.picker_actions
            .contains(&KernelPickerAction::OpenDeepLink)
    }
}

// ---- kernel-origin-pill vocabulary --------------------------------------

/// Environment fingerprint state a kernel origin pill binds, so a pill can tell a user whether
/// reattaching or rerunning would keep the same environment or has materially drifted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KernelFingerprintState {
    /// The environment fingerprint matches the notebook's last run.
    FingerprintMatched,
    /// The environment fingerprint has drifted materially from the notebook's last run.
    FingerprintDrifted,
    /// The environment fingerprint could not be compared.
    FingerprintUnknown,
    /// No environment fingerprint has been evaluated yet.
    FingerprintNotEvaluated,
}

impl KernelFingerprintState {
    /// Every fingerprint state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FingerprintMatched,
        Self::FingerprintDrifted,
        Self::FingerprintUnknown,
        Self::FingerprintNotEvaluated,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FingerprintMatched => "fingerprint_matched",
            Self::FingerprintDrifted => "fingerprint_drifted",
            Self::FingerprintUnknown => "fingerprint_unknown",
            Self::FingerprintNotEvaluated => "fingerprint_not_evaluated",
        }
    }

    /// True only when the fingerprint matches the notebook's last run.
    pub const fn is_matched(self) -> bool {
        matches!(self, Self::FingerprintMatched)
    }
}

/// Derived provenance class a kernel origin pill may present.
///
/// This is the pill honesty axis: the class is derived from the frozen kernel origin trust state,
/// never asserted, so a third-party, unverified, restricted, or unknown origin can never read as
/// exact provenance and a user can always tell whether a kernel origin is exactly attributed or
/// only degraded before trusting exact continuity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KernelProvenanceClass {
    /// The origin is exactly attributed (trusted or first-party).
    ExactProvenance,
    /// The origin is attributed but degraded (third-party or unverified).
    DegradedProvenance,
    /// The origin is restricted by policy.
    RestrictedProvenance,
    /// The origin could not be attributed.
    UnknownProvenance,
}

impl KernelProvenanceClass {
    /// Every provenance class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ExactProvenance,
        Self::DegradedProvenance,
        Self::RestrictedProvenance,
        Self::UnknownProvenance,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactProvenance => "exact_provenance",
            Self::DegradedProvenance => "degraded_provenance",
            Self::RestrictedProvenance => "restricted_provenance",
            Self::UnknownProvenance => "unknown_provenance",
        }
    }

    /// True when the origin is exactly attributed.
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::ExactProvenance)
    }
}

/// One keyboard-complete default action a kernel origin pill offers, so a pill never hides its
/// inspect / view-provenance / copy affordance behind a pointer-only gesture. `InspectOrigin`,
/// `ViewProvenance`, and `CopyOriginIdentity` are always offered so a kernel origin stays visible
/// and copyable in notebook tabs, headers, debug bridges, and support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KernelPillAction {
    /// Inspect the kernel origin details (always available).
    InspectOrigin,
    /// View the origin's provenance and trust limits (always available).
    ViewProvenance,
    /// Copy the stable origin identity (always available).
    CopyOriginIdentity,
    /// Reattach the kernel from this origin.
    ReattachKernel,
    /// Review the rerun / reattach continuity consequences.
    ReviewContinuity,
    /// Open the stable kernel-manager / notebook / docs / support deep link.
    OpenDeepLink,
}

impl KernelPillAction {
    /// Every pill action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InspectOrigin,
        Self::ViewProvenance,
        Self::CopyOriginIdentity,
        Self::ReattachKernel,
        Self::ReviewContinuity,
        Self::OpenDeepLink,
    ];

    /// The default actions every keyboard-complete pill must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::InspectOrigin,
        Self::ViewProvenance,
        Self::CopyOriginIdentity,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectOrigin => "inspect_origin",
            Self::ViewProvenance => "view_provenance",
            Self::CopyOriginIdentity => "copy_origin_identity",
            Self::ReattachKernel => "reattach_kernel",
            Self::ReviewContinuity => "review_continuity",
            Self::OpenDeepLink => "open_deep_link",
        }
    }
}

/// Disclosures a kernel origin pill must carry, derived from the origin class, trust state, and
/// fingerprint state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KernelOriginPillDisclosure {
    /// The derived provenance class this pill may present.
    pub provenance_class: KernelProvenanceClass,
    /// Whether the origin is exactly attributed.
    pub is_exact_provenance: bool,
    /// Whether the origin is local (first-party host).
    pub is_local_origin: bool,
    /// Whether reattaching / rerunning may claim exact continuity (matched fingerprint AND exact
    /// provenance).
    pub may_claim_exact_continuity: bool,
    /// Whether the pill must carry an explicit degraded-provenance note.
    pub needs_degraded_note: bool,
    /// Whether the pill must carry an explicit restricted-origin note.
    pub needs_restricted_note: bool,
    /// Whether the pill must carry an explicit unknown-origin note.
    pub needs_unknown_origin_note: bool,
    /// Whether the pill must carry an explicit fingerprint-drift note.
    pub needs_drift_note: bool,
}

/// Resolves the origin and continuity truth a kernel origin pill may present.
///
/// A `trusted_origin` or `first_party` origin is exact provenance; a `third_party` or
/// `unverified_origin` is degraded; a `restricted_origin` is restricted; an `unknown_origin` is
/// unknown — so a kernel Aureline could not exactly attribute can never read as exact provenance.
/// Exact continuity may be claimed only when the environment fingerprint matches the last run and
/// the provenance is exact, so a kernel change never silently implies exact continuity when the
/// fingerprint differs materially. A drifted, unknown, or not-yet-evaluated fingerprint always
/// carries its own note.
pub fn resolve_kernel_origin_pill(
    origin: M5KernelOriginClass,
    trust: M5KernelOriginTrustState,
    fingerprint: KernelFingerprintState,
) -> KernelOriginPillDisclosure {
    use KernelProvenanceClass as Class;
    use M5KernelOriginTrustState as Trust;

    let provenance_class = match trust {
        Trust::TrustedOrigin | Trust::FirstParty => Class::ExactProvenance,
        Trust::ThirdParty | Trust::UnverifiedOrigin => Class::DegradedProvenance,
        Trust::RestrictedOrigin => Class::RestrictedProvenance,
        Trust::UnknownOrigin => Class::UnknownProvenance,
    };

    let is_exact_provenance = provenance_class.is_exact();

    KernelOriginPillDisclosure {
        provenance_class,
        is_exact_provenance,
        is_local_origin: matches!(origin, M5KernelOriginClass::LocalHost),
        may_claim_exact_continuity: fingerprint.is_matched() && is_exact_provenance,
        needs_degraded_note: matches!(provenance_class, Class::DegradedProvenance),
        needs_restricted_note: matches!(provenance_class, Class::RestrictedProvenance),
        needs_unknown_origin_note: matches!(provenance_class, Class::UnknownProvenance),
        needs_drift_note: !fingerprint.is_matched(),
    }
}

/// A kernel origin pill naming where the current kernel physically runs, how trusted that origin
/// is, its derived provenance class, its environment fingerprint state, whether reattaching /
/// rerunning keeps exact continuity, bounded inspect / view-provenance / copy actions, and a stable
/// deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelOriginPill {
    /// Frozen component this control implements; must be `kernel_origin_pill`.
    pub component: M5NotebookKernelOutputComponentFamily,
    /// Stable pill id.
    pub pill_id: String,
    /// Human-readable pill label; required and non-empty.
    pub pill_label: String,
    /// Kernel origin class, reused from the frozen matrix.
    pub origin_class: M5KernelOriginClass,
    /// Kernel origin trust state, reused from the frozen matrix.
    pub trust_state: M5KernelOriginTrustState,
    /// Environment fingerprint state.
    pub fingerprint_state: KernelFingerprintState,
    /// Derived provenance class (must equal the resolved class).
    pub provenance_class: KernelProvenanceClass,
    /// Whether the pill claims exact provenance (must equal the derived truth).
    pub claims_exact_provenance: bool,
    /// Whether the pill claims exact continuity across reattach / rerun. May be `true` only when
    /// the derived truth allows it.
    pub claims_exact_continuity: bool,
    /// Degraded-provenance note; required when the origin is degraded.
    pub degraded_note: String,
    /// Restricted-origin note; required when the origin is restricted.
    pub restricted_note: String,
    /// Unknown-origin note; required when the origin is unknown.
    pub unknown_origin_note: String,
    /// Fingerprint-drift note; required when the fingerprint does not match.
    pub drift_note: String,
    /// Kernel origin label; always required so where the kernel runs stays explicit and local /
    /// SSH / container / managed / browser-bridge kernels never collapse into one badge.
    pub origin_label: String,
    /// Provenance label; always required so exact-versus-degraded provenance stays explicit.
    pub provenance_label: String,
    /// Trust limit note; always required so trust and policy limits are never hover-only.
    pub trust_limit_note: String,
    /// Continuity note; always required so rerun / reattach continuity stays explicit before a user
    /// assumes exact continuity across restore or handoff.
    pub continuity_note: String,
    /// Context note; always required so the pill names what the origin truth means here.
    pub context_note: String,
    /// Kind of stable deep link this pill binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include inspect / view-provenance / copy).
    pub pill_actions: Vec<KernelPillAction>,
    /// Dispositions this pill binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5NotebookKernelOutputDisposition>,
    /// Downgrade triggers this pill can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5NotebookKernelOutputDowngradeTrigger>,
    /// Mandatory labels this pill can show (must include the mandatory labels).
    pub required_labels: Vec<M5NotebookKernelOutputRequiredLabel>,
    /// Claimed M5 surface families that render this pill.
    pub surface_families: Vec<M5NotebookKernelOutputSurfaceFamily>,
    /// Deployment lines this pill keeps the same truth across.
    pub deployment_lines: Vec<M5NotebookKernelOutputDeploymentLine>,
    /// Non-visual accessibility routes this pill offers.
    pub accessibility_routes: Vec<M5NotebookKernelOutputAccessibilityRoute>,
    /// Notebook subsystems that consume this pill's projection.
    pub consumer_surfaces: Vec<M5NotebookKernelOutputConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this pill.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never collapses local / SSH / container / managed / browser-bridge kernels
    /// into one badge. MUST be `false`.
    pub collapses_kernel_origins_into_one_badge: bool,
    /// Hard invariant: never implies exact continuity when the environment fingerprint differs
    /// materially. MUST be `false`.
    pub implies_exact_continuity_on_material_drift: bool,
    /// Hard invariant: never hides trust or compatibility behind a hover-only affordance. MUST be
    /// `false`.
    pub hides_trust_or_compatibility_behind_hover_only: bool,
    /// Hard invariant: never overwrites resolved provenance with lower-confidence provenance
    /// without review. MUST be `false`.
    pub overwrites_provenance_without_review: bool,
}

impl KernelOriginPill {
    /// Origin / continuity disclosures this pill must carry, derived from the frozen states.
    pub fn origin_disclosure(&self) -> KernelOriginPillDisclosure {
        resolve_kernel_origin_pill(self.origin_class, self.trust_state, self.fingerprint_state)
    }

    /// Whether the pill offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<KernelPillAction> = self.pill_actions.iter().copied().collect();
        KernelPillAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the pill declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5NotebookKernelOutputRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5NotebookKernelOutputRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the pill offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.pill_actions.contains(&KernelPillAction::OpenDeepLink)
    }
}

// ---- review blocks ------------------------------------------------------

/// First-glance kernel-picker / kernel-origin review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelPickerOriginReview {
    /// The picker row names each candidate's kernel class.
    pub picker_shows_kernel_class: bool,
    /// The picker row names each candidate's environment identity.
    pub picker_shows_environment_identity: bool,
    /// The picker row names each candidate's compatibility and trust / policy limits.
    pub picker_shows_compatibility_and_trust_limits: bool,
    /// The picker row offers choose, inspect, and view-compatibility.
    pub picker_offers_choose_inspect_compatibility: bool,
    /// The origin pill names its kernel origin class.
    pub pill_shows_kernel_origin_class: bool,
    /// The origin pill names its provenance confidence.
    pub pill_shows_provenance_confidence: bool,
    /// The origin pill offers inspect, view-provenance, and copy.
    pub pill_offers_inspect_provenance_copy: bool,
    /// Provenance and choice state are derived from state, never asserted.
    pub provenance_and_choice_derived_never_asserted: bool,
    /// A user can choose another kernel without losing sight of provenance, compatibility, or
    /// trust limits.
    pub choose_another_kernel_without_losing_provenance: bool,
    /// Kernel origin stays visible in notebook tabs, headers, debug bridges, and support exports.
    pub kernel_origin_visible_in_tabs_headers_debug_support: bool,
    /// Exact continuity is never implied when the environment fingerprint differs materially.
    pub exact_continuity_never_implied_on_material_drift: bool,
    /// Local / SSH / container / managed / browser-bridge kernels never collapse into one badge.
    pub no_kernel_origins_collapsed_into_one_badge: bool,
    /// Trust and compatibility are never hidden behind a hover-only affordance.
    pub trust_and_compatibility_never_hover_only: bool,
    /// Every next step names one stable kernel-manager / notebook / docs / support deep link.
    pub every_next_step_names_stable_deep_link: bool,
    /// Picker rows and origin pills stay consistent across edit, diff, debug, and support surfaces.
    pub picker_and_pill_consistent_across_surfaces: bool,
    /// No component widens export scope or exposes raw payloads by default.
    pub no_component_widens_export_scope_or_exposes_raw_by_default: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// The components stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl KernelPickerOriginReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.picker_shows_kernel_class
            && self.picker_shows_environment_identity
            && self.picker_shows_compatibility_and_trust_limits
            && self.picker_offers_choose_inspect_compatibility
            && self.pill_shows_kernel_origin_class
            && self.pill_shows_provenance_confidence
            && self.pill_offers_inspect_provenance_copy
            && self.provenance_and_choice_derived_never_asserted
            && self.choose_another_kernel_without_losing_provenance
            && self.kernel_origin_visible_in_tabs_headers_debug_support
            && self.exact_continuity_never_implied_on_material_drift
            && self.no_kernel_origins_collapsed_into_one_badge
            && self.trust_and_compatibility_never_hover_only
            && self.every_next_step_names_stable_deep_link
            && self.picker_and_pill_consistent_across_surfaces
            && self.no_component_widens_export_scope_or_exposes_raw_by_default
            && self.components_stable_across_deployment_lines
            && self.copy_and_export_safe
            && self.no_surface_invents_alternate_state_label
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelPickerOriginConsumerProjection {
    /// The kernel-manager surface reads a single canonical source.
    pub kernel_manager_surface_reads_single_source: bool,
    /// The notebook tab shows the kernel origin.
    pub notebook_tab_shows_kernel_origin: bool,
    /// The debug bridge shows the kernel origin.
    pub debug_bridge_shows_kernel_origin: bool,
    /// The support export shows the kernel origin.
    pub support_export_shows_kernel_origin: bool,
    /// The picker choice is visible before a run.
    pub picker_choice_visible_before_run: bool,
    /// Help / docs shows component truth.
    pub help_docs_shows_component_truth: bool,
}

impl KernelPickerOriginConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.kernel_manager_surface_reads_single_source
            && self.notebook_tab_shows_kernel_origin
            && self.debug_bridge_shows_kernel_origin
            && self.support_export_shows_kernel_origin
            && self.picker_choice_visible_before_run
            && self.help_docs_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelPickerOriginProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`KernelPickerRowKernelOriginPillControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelPickerRowKernelOriginPillControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Kernel picker rows.
    pub picker_rows: Vec<KernelPickerRow>,
    /// Kernel origin pills.
    pub origin_pills: Vec<KernelOriginPill>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5NotebookKernelOutputDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5NotebookKernelOutputConsumerSurface>,
    /// Kernel review block.
    pub kernel_review: KernelPickerOriginReview,
    /// Consumer projection block.
    pub consumer_projection: KernelPickerOriginConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: KernelPickerOriginProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe kernel-picker-row / kernel-origin-pill controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelPickerRowKernelOriginPillControlsPacket {
    /// Record kind; must equal [`KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Kernel picker rows.
    pub picker_rows: Vec<KernelPickerRow>,
    /// Kernel origin pills.
    pub origin_pills: Vec<KernelOriginPill>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5NotebookKernelOutputDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5NotebookKernelOutputConsumerSurface>,
    /// Kernel review block.
    pub kernel_review: KernelPickerOriginReview,
    /// Consumer projection block.
    pub consumer_projection: KernelPickerOriginConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: KernelPickerOriginProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl KernelPickerRowKernelOriginPillControlsPacket {
    /// Builds a kernel-picker-row / kernel-origin-pill controls packet from stable-lane input.
    pub fn new(input: KernelPickerRowKernelOriginPillControlsPacketInput) -> Self {
        Self {
            record_kind: KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_RECORD_KIND.to_owned(),
            schema_version: KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            picker_rows: input.picker_rows,
            origin_pills: input.origin_pills,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            kernel_review: input.kernel_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the kernel-picker-row / kernel-origin-pill control invariants.
    pub fn validate(&self) -> Vec<KernelPickerRowKernelOriginPillViolation> {
        let mut violations = Vec::new();

        if self.record_kind != KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_RECORD_KIND {
            violations.push(KernelPickerRowKernelOriginPillViolation::WrongRecordKind);
        }
        if self.schema_version != KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_SCHEMA_VERSION {
            violations.push(KernelPickerRowKernelOriginPillViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(KernelPickerRowKernelOriginPillViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(KernelPickerRowKernelOriginPillViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(KernelPickerRowKernelOriginPillViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_picker_rows(self, &mut violations);
        validate_origin_pills(self, &mut violations);

        if !self.kernel_review.all_hold() {
            violations.push(KernelPickerRowKernelOriginPillViolation::KernelReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(KernelPickerRowKernelOriginPillViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(KernelPickerRowKernelOriginPillViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("kernel picker row kernel origin pill packet serializes"),
        ) {
            violations.push(KernelPickerRowKernelOriginPillViolation::RawMaterialInExport);
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
            .expect("kernel picker row kernel origin pill packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str("component,id,kind_or_origin,selection_or_trust,derived,selectable_or_exact,deep_link_kind\n");
        for row in &self.picker_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "kernel_picker_row",
                csv_field(&row.row_id),
                row.candidate_kind.as_str(),
                row.selection_state.as_str(),
                row.choice_disclosure().choice_state.as_str(),
                row.choice_disclosure().is_selectable_now,
                row.deep_link_kind.as_str(),
            ));
        }
        for pill in &self.origin_pills {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "kernel_origin_pill",
                csv_field(&pill.pill_id),
                pill.origin_class.as_str(),
                pill.trust_state.as_str(),
                pill.origin_disclosure().provenance_class.as_str(),
                pill.origin_disclosure().is_exact_provenance,
                pill.deep_link_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let not_selectable = self
            .picker_rows
            .iter()
            .filter(|row| !row.choice_disclosure().is_selectable_now)
            .count();
        let not_exact = self
            .origin_pills
            .iter()
            .filter(|pill| !pill.origin_disclosure().is_exact_provenance)
            .count();

        let mut out = String::new();
        out.push_str("# Kernel picker rows and kernel origin pills\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Kernel picker rows: {} ({} not selectable right now)\n",
            self.picker_rows.len(),
            not_selectable
        ));
        out.push_str(&format!(
            "- Kernel origin pills: {} ({} not exact provenance)\n",
            self.origin_pills.len(),
            not_exact
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Kernel picker rows\n\n");
        for row in &self.picker_rows {
            out.push_str(&format!(
                "- **{}** — kind `{}`, selection `{}` → `{}`, last seen `{}`, deep link `{}`\n",
                row.row_label,
                row.candidate_kind.as_str(),
                row.selection_state.as_str(),
                row.choice_disclosure().choice_state.as_str(),
                row.last_seen_label,
                row.deep_link_kind.as_str(),
            ));
        }

        out.push_str("\n## Kernel origin pills\n\n");
        for pill in &self.origin_pills {
            out.push_str(&format!(
                "- **{}** — origin `{}`, trust `{}` → `{}`, fingerprint `{}`, deep link `{}`\n",
                pill.pill_label,
                pill.origin_class.as_str(),
                pill.trust_state.as_str(),
                pill.origin_disclosure().provenance_class.as_str(),
                pill.fingerprint_state.as_str(),
                pill.deep_link_kind.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in kernel-picker-row / kernel-origin-pill export.
#[derive(Debug)]
pub enum KernelPickerRowKernelOriginPillArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<KernelPickerRowKernelOriginPillViolation>),
}

impl fmt::Display for KernelPickerRowKernelOriginPillArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "kernel picker row kernel origin pill export parse failed: {error}"
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
                    "kernel picker row kernel origin pill export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for KernelPickerRowKernelOriginPillArtifactError {}

/// Validation failures emitted by [`KernelPickerRowKernelOriginPillControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KernelPickerRowKernelOriginPillViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No kernel picker rows are present.
    PickerRowsMissing,
    /// A kernel picker row is incomplete.
    PickerRowIncomplete,
    /// A kernel picker row carries the wrong frozen component class.
    PickerRowWrongComponentClass,
    /// A picker row misrepresents its derived choice state.
    ChoiceStateMisrepresented,
    /// An incompatible candidate does not name its incompatibility.
    IncompatibleNoteMissing,
    /// An unavailable candidate does not name its unavailability.
    UnavailableNoteMissing,
    /// An install-first candidate does not name its install step.
    InstallNoteMissing,
    /// A picker row does not name its kernel class.
    KernelClassLabelMissing,
    /// A picker row does not name its environment identity.
    EnvironmentIdentityMissing,
    /// A picker row does not name its locality.
    LocalityLabelMissing,
    /// A picker row does not name its compatibility state.
    CompatibilityNoteMissing,
    /// A picker row does not name its trust / policy limits.
    TrustPolicyLimitNoteMissing,
    /// A picker row does not name its last-seen availability.
    LastSeenLabelMissing,
    /// A picker row omits a mandatory choose / inspect / view-compatibility action.
    PickerActionsIncomplete,
    /// The picker rows do not cover every kernel candidate kind.
    KernelCandidateKindCoverageMissing,
    /// The picker rows do not cover every kernel selection state.
    KernelSelectionStateCoverageMissing,
    /// The picker rows do not cover every derived choice state.
    KernelChoiceStateCoverageMissing,
    /// No kernel origin pills are present.
    OriginPillsMissing,
    /// A kernel origin pill is incomplete.
    OriginPillIncomplete,
    /// A kernel origin pill carries the wrong frozen component class.
    OriginPillWrongComponentClass,
    /// A pill misrepresents its derived provenance class.
    ProvenanceMisrepresented,
    /// A pill claims exact continuity when the fingerprint or provenance does not allow it.
    ExactContinuityOverclaimed,
    /// A degraded-provenance pill does not name its degraded provenance.
    DegradedNoteMissing,
    /// A restricted-origin pill does not name its restriction.
    RestrictedNoteMissing,
    /// An unknown-origin pill does not name its unknown origin.
    UnknownOriginNoteMissing,
    /// A drifted / unverified fingerprint pill does not name its drift.
    DriftNoteMissing,
    /// A pill does not name its kernel origin.
    OriginLabelMissing,
    /// A pill does not name its provenance.
    ProvenanceLabelMissing,
    /// A pill does not name its trust limits.
    TrustLimitNoteMissing,
    /// A pill does not name its rerun / reattach continuity.
    ContinuityNoteMissing,
    /// A pill omits a mandatory inspect / view-provenance / copy action.
    PillActionsIncomplete,
    /// The pills do not cover every kernel origin class.
    KernelOriginClassCoverageMissing,
    /// The pills do not cover every kernel origin trust state.
    KernelOriginTrustStateCoverageMissing,
    /// The pills do not cover every derived provenance class.
    KernelProvenanceClassCoverageMissing,
    /// The pills do not cover every environment fingerprint state.
    KernelFingerprintStateCoverageMissing,
    /// A component does not name its context.
    ContextNoteMissing,
    /// A component offers a deep-link action but its deep link does not resolve exactly.
    DeepLinkUnresolved,
    /// A component names a deep-link kind but not its stable reference.
    DeepLinkRefMissing,
    /// A component does not bind any disposition.
    DispositionsMissing,
    /// A component does not declare its downgrade triggers.
    DowngradeTriggersMissing,
    /// A component does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A component does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A component collapses local / SSH / container / managed / browser-bridge kernels into one
    /// badge.
    KernelOriginsCollapsed,
    /// A component implies exact continuity when the environment fingerprint differs materially.
    ExactContinuityImpliedOnDrift,
    /// A component hides trust or compatibility behind a hover-only affordance.
    TrustOrCompatibilityHoverOnly,
    /// A component overwrites resolved provenance with lower-confidence provenance without review.
    ProvenanceOverwrittenWithoutReview,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Kernel review does not satisfy required invariants.
    KernelReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl KernelPickerRowKernelOriginPillViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::PickerRowsMissing => "picker_rows_missing",
            Self::PickerRowIncomplete => "picker_row_incomplete",
            Self::PickerRowWrongComponentClass => "picker_row_wrong_component_class",
            Self::ChoiceStateMisrepresented => "choice_state_misrepresented",
            Self::IncompatibleNoteMissing => "incompatible_note_missing",
            Self::UnavailableNoteMissing => "unavailable_note_missing",
            Self::InstallNoteMissing => "install_note_missing",
            Self::KernelClassLabelMissing => "kernel_class_label_missing",
            Self::EnvironmentIdentityMissing => "environment_identity_missing",
            Self::LocalityLabelMissing => "locality_label_missing",
            Self::CompatibilityNoteMissing => "compatibility_note_missing",
            Self::TrustPolicyLimitNoteMissing => "trust_policy_limit_note_missing",
            Self::LastSeenLabelMissing => "last_seen_label_missing",
            Self::PickerActionsIncomplete => "picker_actions_incomplete",
            Self::KernelCandidateKindCoverageMissing => "kernel_candidate_kind_coverage_missing",
            Self::KernelSelectionStateCoverageMissing => "kernel_selection_state_coverage_missing",
            Self::KernelChoiceStateCoverageMissing => "kernel_choice_state_coverage_missing",
            Self::OriginPillsMissing => "origin_pills_missing",
            Self::OriginPillIncomplete => "origin_pill_incomplete",
            Self::OriginPillWrongComponentClass => "origin_pill_wrong_component_class",
            Self::ProvenanceMisrepresented => "provenance_misrepresented",
            Self::ExactContinuityOverclaimed => "exact_continuity_overclaimed",
            Self::DegradedNoteMissing => "degraded_note_missing",
            Self::RestrictedNoteMissing => "restricted_note_missing",
            Self::UnknownOriginNoteMissing => "unknown_origin_note_missing",
            Self::DriftNoteMissing => "drift_note_missing",
            Self::OriginLabelMissing => "origin_label_missing",
            Self::ProvenanceLabelMissing => "provenance_label_missing",
            Self::TrustLimitNoteMissing => "trust_limit_note_missing",
            Self::ContinuityNoteMissing => "continuity_note_missing",
            Self::PillActionsIncomplete => "pill_actions_incomplete",
            Self::KernelOriginClassCoverageMissing => "kernel_origin_class_coverage_missing",
            Self::KernelOriginTrustStateCoverageMissing => {
                "kernel_origin_trust_state_coverage_missing"
            }
            Self::KernelProvenanceClassCoverageMissing => {
                "kernel_provenance_class_coverage_missing"
            }
            Self::KernelFingerprintStateCoverageMissing => {
                "kernel_fingerprint_state_coverage_missing"
            }
            Self::ContextNoteMissing => "context_note_missing",
            Self::DeepLinkUnresolved => "deep_link_unresolved",
            Self::DeepLinkRefMissing => "deep_link_ref_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::KernelOriginsCollapsed => "kernel_origins_collapsed",
            Self::ExactContinuityImpliedOnDrift => "exact_continuity_implied_on_drift",
            Self::TrustOrCompatibilityHoverOnly => "trust_or_compatibility_hover_only",
            Self::ProvenanceOverwrittenWithoutReview => "provenance_overwritten_without_review",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::KernelReviewIncomplete => "kernel_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable kernel-picker-row / kernel-origin-pill export.
pub fn current_kernel_picker_row_kernel_origin_pill_export() -> Result<
    KernelPickerRowKernelOriginPillControlsPacket,
    KernelPickerRowKernelOriginPillArtifactError,
> {
    let packet: KernelPickerRowKernelOriginPillControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-kernel-picker-row-kernel-origin-pill-proof/support_export.json"
        )))
        .map_err(KernelPickerRowKernelOriginPillArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(KernelPickerRowKernelOriginPillArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &KernelPickerRowKernelOriginPillControlsPacket,
    violations: &mut Vec<KernelPickerRowKernelOriginPillViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_SCHEMA_REF,
        KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_DOC_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_DOC_REF,
        M5_KERNEL_PICKER_ROW_SCHEMA_REF,
        M5_KERNEL_ORIGIN_PILL_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(KernelPickerRowKernelOriginPillViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_picker_rows(
    packet: &KernelPickerRowKernelOriginPillControlsPacket,
    violations: &mut Vec<KernelPickerRowKernelOriginPillViolation>,
) {
    if packet.picker_rows.is_empty() {
        violations.push(KernelPickerRowKernelOriginPillViolation::PickerRowsMissing);
        return;
    }

    let mut choice_states: BTreeSet<KernelChoiceState> = BTreeSet::new();
    let mut kinds: BTreeSet<M5KernelCandidateKind> = BTreeSet::new();
    let mut selections: BTreeSet<M5KernelSelectionState> = BTreeSet::new();

    for row in &packet.picker_rows {
        let disclosure = row.choice_disclosure();
        choice_states.insert(disclosure.choice_state);
        kinds.insert(row.candidate_kind);
        selections.insert(row.selection_state);

        if row.row_id.trim().is_empty()
            || row.row_label.trim().is_empty()
            || row.fields_shown.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.consumer_surfaces.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(KernelPickerRowKernelOriginPillViolation::PickerRowIncomplete);
        }
        if row.component != M5NotebookKernelOutputComponentFamily::KernelPickerRow {
            violations.push(KernelPickerRowKernelOriginPillViolation::PickerRowWrongComponentClass);
        }
        if row.choice_state != disclosure.choice_state
            || row.claims_selectable_now != disclosure.is_selectable_now
            || row.claims_current != disclosure.is_current
        {
            violations.push(KernelPickerRowKernelOriginPillViolation::ChoiceStateMisrepresented);
        }
        if disclosure.needs_incompatible_note && row.incompatible_note.trim().is_empty() {
            violations.push(KernelPickerRowKernelOriginPillViolation::IncompatibleNoteMissing);
        }
        if disclosure.needs_unavailable_note && row.unavailable_note.trim().is_empty() {
            violations.push(KernelPickerRowKernelOriginPillViolation::UnavailableNoteMissing);
        }
        if disclosure.needs_install_note && row.install_note.trim().is_empty() {
            violations.push(KernelPickerRowKernelOriginPillViolation::InstallNoteMissing);
        }
        if row.kernel_class_label.trim().is_empty() {
            violations.push(KernelPickerRowKernelOriginPillViolation::KernelClassLabelMissing);
        }
        if row.environment_identity_label.trim().is_empty() {
            violations.push(KernelPickerRowKernelOriginPillViolation::EnvironmentIdentityMissing);
        }
        if row.locality_label.trim().is_empty() {
            violations.push(KernelPickerRowKernelOriginPillViolation::LocalityLabelMissing);
        }
        if row.compatibility_note.trim().is_empty() {
            violations.push(KernelPickerRowKernelOriginPillViolation::CompatibilityNoteMissing);
        }
        if row.trust_policy_limit_note.trim().is_empty() {
            violations.push(KernelPickerRowKernelOriginPillViolation::TrustPolicyLimitNoteMissing);
        }
        if row.last_seen_label.trim().is_empty() {
            violations.push(KernelPickerRowKernelOriginPillViolation::LastSeenLabelMissing);
        }
        if !row.declares_mandatory_actions() {
            violations.push(KernelPickerRowKernelOriginPillViolation::PickerActionsIncomplete);
        }
        validate_deep_link(
            row.offers_deep_link_action(),
            row.deep_link_kind,
            &row.deep_link_ref,
            &row.context_note,
            violations,
        );
        validate_common_control(
            &row.dispositions,
            &row.downgrade_triggers,
            row.declares_mandatory_labels(),
            &row.accessibility_routes,
            ControlInvariants {
                collapses_kernel_origins_into_one_badge: row
                    .collapses_kernel_origins_into_one_badge,
                implies_exact_continuity_on_material_drift: row
                    .implies_exact_continuity_on_material_drift,
                hides_trust_or_compatibility_behind_hover_only: row
                    .hides_trust_or_compatibility_behind_hover_only,
                overwrites_provenance_without_review: row.overwrites_provenance_without_review,
            },
            violations,
        );
    }

    for required in KernelChoiceState::ALL {
        if !choice_states.contains(&required) {
            violations
                .push(KernelPickerRowKernelOriginPillViolation::KernelChoiceStateCoverageMissing);
            break;
        }
    }
    for required in M5KernelCandidateKind::ALL {
        if !kinds.contains(&required) {
            violations
                .push(KernelPickerRowKernelOriginPillViolation::KernelCandidateKindCoverageMissing);
            break;
        }
    }
    for required in M5KernelSelectionState::ALL {
        if !selections.contains(&required) {
            violations.push(
                KernelPickerRowKernelOriginPillViolation::KernelSelectionStateCoverageMissing,
            );
            break;
        }
    }
}

fn validate_origin_pills(
    packet: &KernelPickerRowKernelOriginPillControlsPacket,
    violations: &mut Vec<KernelPickerRowKernelOriginPillViolation>,
) {
    if packet.origin_pills.is_empty() {
        violations.push(KernelPickerRowKernelOriginPillViolation::OriginPillsMissing);
        return;
    }

    let mut provenance_classes: BTreeSet<KernelProvenanceClass> = BTreeSet::new();
    let mut origins: BTreeSet<M5KernelOriginClass> = BTreeSet::new();
    let mut trusts: BTreeSet<M5KernelOriginTrustState> = BTreeSet::new();
    let mut fingerprints: BTreeSet<KernelFingerprintState> = BTreeSet::new();

    for pill in &packet.origin_pills {
        let disclosure = pill.origin_disclosure();
        provenance_classes.insert(disclosure.provenance_class);
        origins.insert(pill.origin_class);
        trusts.insert(pill.trust_state);
        fingerprints.insert(pill.fingerprint_state);

        if pill.pill_id.trim().is_empty()
            || pill.pill_label.trim().is_empty()
            || pill.fields_shown.is_empty()
            || pill.surface_families.is_empty()
            || pill.deployment_lines.is_empty()
            || pill.consumer_surfaces.is_empty()
            || pill.source_contract_refs.is_empty()
        {
            violations.push(KernelPickerRowKernelOriginPillViolation::OriginPillIncomplete);
        }
        if pill.component != M5NotebookKernelOutputComponentFamily::KernelOriginPill {
            violations
                .push(KernelPickerRowKernelOriginPillViolation::OriginPillWrongComponentClass);
        }
        if pill.provenance_class != disclosure.provenance_class
            || pill.claims_exact_provenance != disclosure.is_exact_provenance
        {
            violations.push(KernelPickerRowKernelOriginPillViolation::ProvenanceMisrepresented);
        }
        if pill.claims_exact_continuity && !disclosure.may_claim_exact_continuity {
            violations.push(KernelPickerRowKernelOriginPillViolation::ExactContinuityOverclaimed);
        }
        if disclosure.needs_degraded_note && pill.degraded_note.trim().is_empty() {
            violations.push(KernelPickerRowKernelOriginPillViolation::DegradedNoteMissing);
        }
        if disclosure.needs_restricted_note && pill.restricted_note.trim().is_empty() {
            violations.push(KernelPickerRowKernelOriginPillViolation::RestrictedNoteMissing);
        }
        if disclosure.needs_unknown_origin_note && pill.unknown_origin_note.trim().is_empty() {
            violations.push(KernelPickerRowKernelOriginPillViolation::UnknownOriginNoteMissing);
        }
        if disclosure.needs_drift_note && pill.drift_note.trim().is_empty() {
            violations.push(KernelPickerRowKernelOriginPillViolation::DriftNoteMissing);
        }
        if pill.origin_label.trim().is_empty() {
            violations.push(KernelPickerRowKernelOriginPillViolation::OriginLabelMissing);
        }
        if pill.provenance_label.trim().is_empty() {
            violations.push(KernelPickerRowKernelOriginPillViolation::ProvenanceLabelMissing);
        }
        if pill.trust_limit_note.trim().is_empty() {
            violations.push(KernelPickerRowKernelOriginPillViolation::TrustLimitNoteMissing);
        }
        if pill.continuity_note.trim().is_empty() {
            violations.push(KernelPickerRowKernelOriginPillViolation::ContinuityNoteMissing);
        }
        if !pill.declares_mandatory_actions() {
            violations.push(KernelPickerRowKernelOriginPillViolation::PillActionsIncomplete);
        }
        validate_deep_link(
            pill.offers_deep_link_action(),
            pill.deep_link_kind,
            &pill.deep_link_ref,
            &pill.context_note,
            violations,
        );
        validate_common_control(
            &pill.dispositions,
            &pill.downgrade_triggers,
            pill.declares_mandatory_labels(),
            &pill.accessibility_routes,
            ControlInvariants {
                collapses_kernel_origins_into_one_badge: pill
                    .collapses_kernel_origins_into_one_badge,
                implies_exact_continuity_on_material_drift: pill
                    .implies_exact_continuity_on_material_drift,
                hides_trust_or_compatibility_behind_hover_only: pill
                    .hides_trust_or_compatibility_behind_hover_only,
                overwrites_provenance_without_review: pill.overwrites_provenance_without_review,
            },
            violations,
        );
    }

    for required in KernelProvenanceClass::ALL {
        if !provenance_classes.contains(&required) {
            violations.push(
                KernelPickerRowKernelOriginPillViolation::KernelProvenanceClassCoverageMissing,
            );
            break;
        }
    }
    for required in M5KernelOriginClass::ALL {
        if !origins.contains(&required) {
            violations
                .push(KernelPickerRowKernelOriginPillViolation::KernelOriginClassCoverageMissing);
            break;
        }
    }
    for required in M5KernelOriginTrustState::ALL {
        if !trusts.contains(&required) {
            violations.push(
                KernelPickerRowKernelOriginPillViolation::KernelOriginTrustStateCoverageMissing,
            );
            break;
        }
    }
    for required in KernelFingerprintState::ALL {
        if !fingerprints.contains(&required) {
            violations.push(
                KernelPickerRowKernelOriginPillViolation::KernelFingerprintStateCoverageMissing,
            );
            break;
        }
    }
}

/// Validates the context and stable deep-link truth shared by both component vectors.
///
/// A component that offers a deep-link action must name a resolvable deep-link kind, a component
/// that names a resolvable kind must carry its stable reference, and every component must name its
/// context — so a next step is never an ephemeral overlay or hidden route.
fn validate_deep_link(
    offers_deep_link_action: bool,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    context_note: &str,
    violations: &mut Vec<KernelPickerRowKernelOriginPillViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(KernelPickerRowKernelOriginPillViolation::ContextNoteMissing);
    }
    if offers_deep_link_action && !deep_link_kind.is_resolvable() {
        violations.push(KernelPickerRowKernelOriginPillViolation::DeepLinkUnresolved);
    }
    if deep_link_kind.is_resolvable() && deep_link_ref.trim().is_empty() {
        violations.push(KernelPickerRowKernelOriginPillViolation::DeepLinkRefMissing);
    }
}

/// The four hard-invariant bools every component must keep `false`.
struct ControlInvariants {
    collapses_kernel_origins_into_one_badge: bool,
    implies_exact_continuity_on_material_drift: bool,
    hides_trust_or_compatibility_behind_hover_only: bool,
    overwrites_provenance_without_review: bool,
}

/// Validates the axes shared by both component vectors.
fn validate_common_control(
    dispositions: &[M5NotebookKernelOutputDisposition],
    downgrade_triggers: &[M5NotebookKernelOutputDowngradeTrigger],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5NotebookKernelOutputAccessibilityRoute],
    invariants: ControlInvariants,
    violations: &mut Vec<KernelPickerRowKernelOriginPillViolation>,
) {
    if dispositions.is_empty() {
        violations.push(KernelPickerRowKernelOriginPillViolation::DispositionsMissing);
    }
    if downgrade_triggers.is_empty() {
        violations.push(KernelPickerRowKernelOriginPillViolation::DowngradeTriggersMissing);
    }
    if !declares_mandatory_labels {
        violations.push(KernelPickerRowKernelOriginPillViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes
            .contains(&M5NotebookKernelOutputAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(KernelPickerRowKernelOriginPillViolation::AccessibilityRouteMissing);
    }
    if invariants.collapses_kernel_origins_into_one_badge {
        violations.push(KernelPickerRowKernelOriginPillViolation::KernelOriginsCollapsed);
    }
    if invariants.implies_exact_continuity_on_material_drift {
        violations.push(KernelPickerRowKernelOriginPillViolation::ExactContinuityImpliedOnDrift);
    }
    if invariants.hides_trust_or_compatibility_behind_hover_only {
        violations.push(KernelPickerRowKernelOriginPillViolation::TrustOrCompatibilityHoverOnly);
    }
    if invariants.overwrites_provenance_without_review {
        violations
            .push(KernelPickerRowKernelOriginPillViolation::ProvenanceOverwrittenWithoutReview);
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
