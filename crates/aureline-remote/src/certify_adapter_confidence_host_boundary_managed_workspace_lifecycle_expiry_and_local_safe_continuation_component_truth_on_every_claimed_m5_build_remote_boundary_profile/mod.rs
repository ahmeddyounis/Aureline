//! M05-1083 profile certification over the frozen M5 build/remote-boundary
//! component matrix — the closing capstone of the B128 batch.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_adapter_confidence_chip_discovery_diff_card_host_boundary_strip_execution_origin_receipt_row_managed_workspace_lifecycle_card_suspend_resume_rebuild_review_sheet_workspace_expiry_banner_and_local_safe_continuation_card_component_matrix`])
//! defines the eight reusable adapter-confidence chip, discovery-diff card,
//! host-boundary strip, execution-origin receipt row, managed-workspace lifecycle
//! card, suspend/resume/rebuild review sheet, workspace-expiry banner, and
//! local-safe continuation card components, the four M05-1077..1080 implement lanes
//! narrow each one, the M05-1082 consumer lane
//! ([`crate::wire_run_test_debug_notebook_preview_ai_companion_and_support_consumers_so_build_and_remote_boundary_components_keep_one_vocabulary_across_claimed_m5_execution_and_export_surfaces`])
//! adopts them, and the M05-1081 accessibility lane
//! ([`crate::implement_keyboard_screen_reader_reduced_motion_high_contrast_cli_export_and_support_packet_parity_and_build_remote_boundary_component_claim_auto_narrowing`])
//! proves keyboard / screen-reader / reduced-motion / high-contrast / CLI-export
//! parity and per-family auto-narrowing, this closing capstone *certifies* that the
//! shared component truth holds on every claimed M5 build / remote / managed
//! execution profile — and auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **execution / deployment profile** a user, operator,
//! or support engineer reads build/remote-boundary truth through (local, SSH,
//! container, devcontainer, managed workspace, suspend/resume, rebuild/recreate, and
//! expiry / local-safe continuation), not on component family or implement lane.
//! Each [`BuildRemoteProfileCertificationRow`] certifies one profile across six truth
//! axes — visual, keyboard, screen-reader, CLI/export, degraded-state, and
//! boundary-truth behavior — and either passes (green), auto-narrows its
//! boundary-support claim to the weakest supported ceiling (yellow), or is blocked
//! (red) when a degraded axis is hidden behind a fresh first-party full-truth claim
//! inherited from a healthier profile, or a spec guardrail is violated.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A
//! profile that keeps a `FullTruth` / `ResolvedTruth` claim while one of its truth
//! axes is not current is over-claiming and blocks; a profile that discloses the
//! reduction by narrowing its boundary-support claim (with a bound reason and a
//! frozen downgrade trigger) is honestly yellow. The always-on CLI/export axis must
//! always stay certified, so support and automation can reconstruct the certified
//! adapter confidence, discovery drift, host ownership, execution origin, lifecycle
//! state, persistence class, continuity, expiry timing, and local-safe continuation
//! truth from the same object identity the user saw. A stale, unverified, or
//! unsupported profile can never keep a fresh first-party full-truth claim, and no
//! build/remote profile may imply exact continuity after a material target /
//! image / template / persistence-class change, hide local-safe or companion
//! handoff behind overflow-only affordances, or let lower-confidence discovery
//! overwrite higher-confidence resolved target truth without an explicit review
//! state.
//!
//! Every row cites exactly one canonical build/remote-boundary proof bundle
//! ([`BUILD_REMOTE_CERT_CANONICAL_BUNDLE_REF`]) — the frozen build/remote-boundary
//! component matrix proof — rather than cloning per-profile evidence. The packet is
//! metadata-only: raw provider tokens, credential material, and bearer secrets never
//! cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-build-remote-boundary-component-certification.schema.json`](../../../../schemas/ui/m5-build-remote-boundary-component-certification.schema.json).
//! The contract doc is
//! [`docs/remote/m5_build_remote_boundary_component_certification_contract.md`](../../../../docs/remote/m5_build_remote_boundary_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_adapter_confidence_chip_discovery_diff_card_host_boundary_strip_execution_origin_receipt_row_managed_workspace_lifecycle_card_suspend_resume_rebuild_review_sheet_workspace_expiry_banner_and_local_safe_continuation_card_component_matrix as matrix;
use crate::implement_keyboard_screen_reader_reduced_motion_high_contrast_cli_export_and_support_packet_parity_and_build_remote_boundary_component_claim_auto_narrowing as a11y;
use a11y::M5BuildRemoteAccessClaim;
use matrix::{M5BuildRemoteBoundaryComponentFamily, M5BuildRemoteDowngradeTrigger};

/// Schema version stamped on the M05-1083 certification packet.
pub const BUILD_REMOTE_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`BuildRemoteProfileCertificationPacket`].
pub const BUILD_REMOTE_CERT_RECORD_KIND: &str =
    "m5_build_remote_boundary_component_certification_packet";

/// Stable record-kind tag carried by each [`BuildRemoteProfileCertificationRow`].
pub const BUILD_REMOTE_CERT_ROW_RECORD_KIND: &str =
    "m5_build_remote_boundary_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const BUILD_REMOTE_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-build-remote-boundary-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const BUILD_REMOTE_CERT_DOC_REF: &str =
    "docs/remote/m5_build_remote_boundary_component_certification_contract.md";

/// Repo-relative path of the frozen build/remote-boundary component matrix schema the
/// certified profiles render.
pub const BUILD_REMOTE_CERT_MATRIX_REF: &str =
    matrix::M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF;

/// The one canonical build/remote-boundary proof bundle every certified profile cites
/// as its first-resolved component truth. All eight profiles point back to it rather
/// than cloning per-profile evidence.
pub const BUILD_REMOTE_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_BUILD_REMOTE_BOUNDARY_COMPONENT_ARTIFACT_REF;

/// The M05-1081 accessibility support export the certification builds on. Recorded as
/// a supporting evidence ref on every row.
pub const BUILD_REMOTE_CERT_A11Y_BUNDLE_REF: &str = a11y::BUILD_REMOTE_BOUNDARY_A11Y_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const BUILD_REMOTE_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-build-remote-boundary-component-certification-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const BUILD_REMOTE_CERT_CSV_REF: &str =
    "artifacts/release/m5-build-remote-boundary-component-certification-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const BUILD_REMOTE_CERT_REPORT_REF: &str =
    "artifacts/release/m5-build-remote-boundary-component-certification-proof/report.md";

/// The eight claimed M5 build / remote / managed execution profiles this capstone
/// certifies. Keyed on the execution / deployment profile a user, operator, or support
/// engineer reads build/remote-boundary truth through, not on the reusable component
/// family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildRemoteCertifiedProfile {
    /// Local, first-party execution on the developer's own machine.
    LocalExecution,
    /// Execution over an SSH remote.
    SshExecution,
    /// Execution inside a container target.
    ContainerExecution,
    /// Execution inside a devcontainer target.
    DevcontainerExecution,
    /// Execution inside a provisioned, managed workspace.
    ManagedWorkspace,
    /// A managed workspace crossing a suspend / resume boundary.
    SuspendResume,
    /// A managed workspace crossing a rebuild / recreate boundary (image / template /
    /// persistence class may have changed materially).
    RebuildRecreate,
    /// A workspace crossing an expiry boundary and falling back to local-safe
    /// continuation.
    ExpiryLocalSafe,
}

impl M5BuildRemoteCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5BuildRemoteCertifiedProfile; 8] = [
        M5BuildRemoteCertifiedProfile::LocalExecution,
        M5BuildRemoteCertifiedProfile::SshExecution,
        M5BuildRemoteCertifiedProfile::ContainerExecution,
        M5BuildRemoteCertifiedProfile::DevcontainerExecution,
        M5BuildRemoteCertifiedProfile::ManagedWorkspace,
        M5BuildRemoteCertifiedProfile::SuspendResume,
        M5BuildRemoteCertifiedProfile::RebuildRecreate,
        M5BuildRemoteCertifiedProfile::ExpiryLocalSafe,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalExecution => "local_execution",
            Self::SshExecution => "ssh_execution",
            Self::ContainerExecution => "container_execution",
            Self::DevcontainerExecution => "devcontainer_execution",
            Self::ManagedWorkspace => "managed_workspace",
            Self::SuspendResume => "suspend_resume",
            Self::RebuildRecreate => "rebuild_recreate",
            Self::ExpiryLocalSafe => "expiry_local_safe",
        }
    }

    /// True for the single local, first-party-local profile — the only one that may
    /// certify a live `FullTruth` claim.
    pub const fn is_local_first_party(self) -> bool {
        matches!(self, Self::LocalExecution)
    }
}

/// The six truth axes a certified profile is scored on. These are exactly the parity
/// dimensions the spec requires verifying — visual, keyboard, screen-reader,
/// CLI/export, degraded-state, and boundary-truth behavior. The CLI/export axis is
/// always-on and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildRemoteCertificationAxis {
    /// Visual parity: adapter confidence, discovery drift, host ownership, execution
    /// origin, lifecycle state, persistence class, continuity, expiry timing, and
    /// local-safe continuation are shown on the primary surface.
    Visual,
    /// Keyboard-reach parity: the same boundary truth and its actions (inspect, review,
    /// reconnect, export-before-loss, renew) are reachable without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on
    /// color or a status glyph alone.
    ScreenReader,
    /// CLI / export parity (always-on): the certified profile state is reconstructable
    /// as text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a stale, unverified, or unsupported build/remote reading
    /// honestly downgrades a `FullTruth` / `ResolvedTruth` claim rather than reading as
    /// fresh first-party local truth.
    DegradedState,
    /// Boundary-truth parity: adapter confidence, discovery drift, host boundary,
    /// execution origin, lifecycle state, persistence class, continuity, expiry timing,
    /// and local-safe continuation stay explicit and never collapse into generic status
    /// wording, imply exact continuity after a material change, hide local-safe /
    /// companion handoff behind overflow only, or let lower-confidence discovery
    /// overwrite resolved target truth.
    BoundaryTruth,
}

impl BuildRemoteCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [BuildRemoteCertificationAxis; 6] = [
        BuildRemoteCertificationAxis::Visual,
        BuildRemoteCertificationAxis::Keyboard,
        BuildRemoteCertificationAxis::ScreenReader,
        BuildRemoteCertificationAxis::CliExport,
        BuildRemoteCertificationAxis::DegradedState,
        BuildRemoteCertificationAxis::BoundaryTruth,
    ];

    /// The always-on CLI/export axis that must stay certified on every row.
    pub const fn is_always_on(self) -> bool {
        matches!(self, Self::CliExport)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::BoundaryTruth => "boundary_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildRemoteAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a
    /// visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a full-truth claim
    /// inherited from a healthier profile.
    UndisclosedDrift,
}

impl BuildRemoteAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole profile. Never asserted by the
/// author — always recomputed from the axis outcomes, guardrails, and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildRemoteProfileClaimStatus {
    /// Full standing: every axis certified, claimed boundary-support tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, CLI/export parity drops, a
    /// guardrail is violated, or the narrowing is inconsistent.
    Red,
}

impl BuildRemoteProfileClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the profile is publishable as certified (green or disclosed yellow);
    /// red profiles block the release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The three B128 spec guardrails, evaluated per certified profile. All must stay
/// false; any true blocks the profile (red) regardless of axis parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRemoteCertGuardrails {
    /// A reused card implies exact continuity after target identity, image, template,
    /// or persistence class changed materially.
    pub implies_exact_continuity_after_material_change: bool,
    /// Local-safe continuation or browser / companion handoff is hidden behind
    /// overflow-only affordances.
    pub hides_local_safe_or_companion_handoff_in_overflow_only: bool,
    /// Lower-confidence discovery overwrote a higher-confidence resolved target without
    /// an explicit review state.
    pub lower_confidence_overwrites_resolved_target_without_review: bool,
}

impl BuildRemoteCertGuardrails {
    /// A clean, all-false guardrail set.
    pub const CLEAN: BuildRemoteCertGuardrails = BuildRemoteCertGuardrails {
        implies_exact_continuity_after_material_change: false,
        hides_local_safe_or_companion_handoff_in_overflow_only: false,
        lower_confidence_overwrites_resolved_target_without_review: false,
    };

    /// True when every guardrail is held (all false).
    pub const fn all_held(self) -> bool {
        !self.implies_exact_continuity_after_material_change
            && !self.hides_local_safe_or_companion_handoff_in_overflow_only
            && !self.lower_confidence_overwrites_resolved_target_without_review
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis
/// certifies only when this offers text / JSON / Markdown reconstruction and prohibits
/// a screenshot-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRemoteCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The adapter-confidence / discovery-drift / host-boundary / execution-origin /
    /// lifecycle-state / persistence-class / continuity / expiry-timing /
    /// local-safe-continuation fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl BuildRemoteCertExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a
    /// screenshot-only export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.screenshot_only_prohibited
    }
}

/// One axis outcome on one certified profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRemoteAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: BuildRemoteCertificationAxis,
    /// The certification state of the axis.
    pub state: BuildRemoteAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5BuildRemoteDowngradeTrigger>,
}

impl BuildRemoteAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a frozen trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no visible
    ///   trigger (that is exactly what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            BuildRemoteAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            BuildRemoteAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            BuildRemoteAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a profile applies when a truth axis is not current.
/// Present iff the certified claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRemoteClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: BuildRemoteCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5BuildRemoteAccessClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5BuildRemoteAccessClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 build / remote / managed execution profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRemoteProfileCertificationRow {
    /// Record kind; must equal [`BUILD_REMOTE_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`BUILD_REMOTE_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5BuildRemoteCertifiedProfile,
    /// The boundary-support claim ceiling the profile asserts.
    pub claimed_claim: M5BuildRemoteAccessClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger
    /// than `claimed_claim`.
    pub certified_claim: M5BuildRemoteAccessClaim,
    /// The frozen component families this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5BuildRemoteBoundaryComponentFamily>,
    /// One outcome per [`BuildRemoteCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<BuildRemoteAxisOutcome>,
    /// The three spec guardrails for this profile; all must be held (false).
    pub guardrails: BuildRemoteCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than
    /// `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<BuildRemoteClaimAutoNarrow>,
    /// The one canonical build/remote-boundary proof bundle this profile cites. Must
    /// equal [`BUILD_REMOTE_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: BuildRemoteProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: BuildRemoteCertExportParity,
    /// The compatibility notes captured for this profile.
    #[serde(default)]
    pub compatibility_notes: Vec<String>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the certification was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl BuildRemoteProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: BuildRemoteCertificationAxis) -> Option<&BuildRemoteAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<BuildRemoteCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && BuildRemoteCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(BuildRemoteAxisOutcome::well_formed)
    }

    /// True when the profile narrows its boundary-support claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<BuildRemoteCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == BuildRemoteAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, guardrails, and claim narrowing. This
    /// is the heart of the capstone: a guardrail breach blocks, a degraded axis must
    /// produce a visible claim narrowing, CLI/export parity must always certify, and the
    /// narrowing must be consistent.
    pub fn derive_status(&self) -> BuildRemoteProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != BUILD_REMOTE_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return BuildRemoteProfileClaimStatus::Red;
        }

        // Any spec guardrail breach blocks outright.
        if !self.guardrails.all_held() {
            return BuildRemoteProfileClaimStatus::Red;
        }

        // A live full-truth claim may only stand on the local, first-party-local profile.
        if self.certified_claim.asserts_live_truth() && !self.profile.is_local_first_party() {
            return BuildRemoteProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return BuildRemoteProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(BuildRemoteCertificationAxis::CliExport) {
            Some(o) if o.state == BuildRemoteAxisCertificationState::Certified => {}
            _ => return BuildRemoteProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == BuildRemoteAxisCertificationState::UndisclosedDrift)
        {
            return BuildRemoteProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return BuildRemoteProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return BuildRemoteProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return BuildRemoteProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return BuildRemoteProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden
        // overclaim inheriting a healthier profile's truth.
        if !narrowed.is_empty() {
            return BuildRemoteProfileClaimStatus::Red;
        }

        BuildRemoteProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == BUILD_REMOTE_CERT_ROW_RECORD_KIND
            && self.schema_version == BUILD_REMOTE_CERT_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.canonical_bundle_ref.trim().is_empty()
            && !self.consumed_families.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.compatibility_notes.is_empty()
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "profile={profile} claimed={claimed} certified={certified} status={status} \
narrowed_axes={narrowed}",
            profile = self.profile.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
        )
    }
}

/// Rolled-up summary of an M05-1083 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRemoteProfileCertificationSummary {
    pub row_count: usize,
    pub profile_count: usize,
    pub green_row_count: usize,
    pub yellow_row_count: usize,
    pub red_row_count: usize,
    pub all_profiles_present: bool,
    pub all_families_covered: bool,
    pub all_rows_publishable: bool,
    pub all_status_fresh: bool,
    pub all_rows_cite_canonical_bundle: bool,
    pub all_rows_export_parity_certified: bool,
    pub all_guardrails_held: bool,
    pub every_axis_covered_on_every_row: bool,
    pub narrowed_profile_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`BuildRemoteProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildRemoteProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<BuildRemoteProfileCertificationRow>,
}

/// Checked-in M05-1083 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRemoteProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<BuildRemoteProfileCertificationRow>,
    pub summary: BuildRemoteProfileCertificationSummary,
}

impl BuildRemoteProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: BuildRemoteProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: BUILD_REMOTE_CERT_SCHEMA_VERSION,
            record_kind: BUILD_REMOTE_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: BuildRemoteProfileCertificationSummary {
                row_count: 0,
                profile_count: 0,
                green_row_count: 0,
                yellow_row_count: 0,
                red_row_count: 0,
                all_profiles_present: false,
                all_families_covered: false,
                all_rows_publishable: false,
                all_status_fresh: false,
                all_rows_cite_canonical_bundle: false,
                all_rows_export_parity_certified: false,
                all_guardrails_held: false,
                every_axis_covered_on_every_row: false,
                narrowed_profile_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Profiles represented by some row in this packet.
    pub fn represented_profiles(&self) -> BTreeSet<M5BuildRemoteCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Component families rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5BuildRemoteBoundaryComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5BuildRemoteCertifiedProfile::ALL
                .iter()
                .all(|p| profiles.contains(p))
    }

    /// Whether every frozen component family is certified on at least one profile —
    /// proof the full matrix runs across the claimed profiles.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5BuildRemoteBoundaryComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(BuildRemoteCertificationAxis::CliExport)
                .is_some_and(|o| o.state == BuildRemoteAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Whether every row holds all three spec guardrails.
    pub fn all_guardrails_held(&self) -> bool {
        self.rows.iter().all(|r| r.guardrails.all_held())
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> BuildRemoteProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == BuildRemoteProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == BuildRemoteProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == BuildRemoteProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(BuildRemoteProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();
        let all_guardrails = self.all_guardrails_held();

        BuildRemoteProfileCertificationSummary {
            row_count: self.rows.len(),
            profile_count: profiles.len(),
            green_row_count: green,
            yellow_row_count: yellow,
            red_row_count: red,
            all_profiles_present: all_profiles,
            all_families_covered: all_families,
            all_rows_publishable: all_publishable,
            all_status_fresh: all_fresh,
            all_rows_cite_canonical_bundle: self
                .rows
                .iter()
                .all(|r| r.canonical_bundle_ref == BUILD_REMOTE_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: all_guardrails,
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(BuildRemoteProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable
                && all_fresh
                && all_profiles
                && all_families
                && all_guardrails,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<BuildRemoteCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != BUILD_REMOTE_CERT_SCHEMA_VERSION {
            violations.push(BuildRemoteCertificationViolation::SchemaVersion {
                expected: BUILD_REMOTE_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != BUILD_REMOTE_CERT_RECORD_KIND {
            violations.push(BuildRemoteCertificationViolation::RecordKind {
                expected: BUILD_REMOTE_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(BuildRemoteCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != BUILD_REMOTE_CERT_CANONICAL_BUNDLE_REF {
            violations.push(BuildRemoteCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(BuildRemoteCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(BuildRemoteCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(BuildRemoteCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(BuildRemoteCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != BUILD_REMOTE_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    BuildRemoteCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The three spec guardrails must be held.
            if !row.guardrails.all_held() {
                violations.push(BuildRemoteCertificationViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                });
            }

            // A live full-truth claim may only stand on the local first-party profile.
            if row.certified_claim.asserts_live_truth() && !row.profile.is_local_first_party() {
                violations.push(
                    BuildRemoteCertificationViolation::NonLocalProfileClaimsLiveTruth {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(BuildRemoteCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    BuildRemoteCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    BuildRemoteCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(BuildRemoteCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == BuildRemoteProfileClaimStatus::Red {
                violations.push(BuildRemoteCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(BuildRemoteCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen component family must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(BuildRemoteCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(BuildRemoteCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(BuildRemoteCertificationViolation::RawRemoteMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("certification packet serializes")
    }

    /// Deterministic CSV of the certification rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,profile,claimed_claim,certified_claim,status,narrowed_axes,binding_axis\n",
        );
        for row in &self.rows {
            let binding = row
                .claim_auto_narrow
                .as_ref()
                .map(|n| n.binding_axis.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{id},{profile},{claimed},{certified},{status},{narrowed},{binding}\n",
                id = row.row_id,
                profile = row.profile.as_str(),
                claimed = row.claimed_claim.as_str(),
                certified = row.certified_claim.as_str(),
                status = row.derived_status.as_str(),
                narrowed = row.narrowed_axes().len(),
                binding = binding,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Build/Remote-Boundary Component Profile Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5BuildRemoteCertifiedProfile::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- Guardrails held: {}\n",
            self.summary.all_guardrails_held
        ));
        out.push_str(&format!(
            "- Auto-narrowed profiles: {}\n",
            self.summary.narrowed_profile_count,
        ));
        out.push_str(&format!("- Report clean: {}\n", self.summary.report_clean));
        out.push_str("\n## Profiles\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in certification export.
pub fn current_m5_build_remote_boundary_component_certification_export(
) -> Result<BuildRemoteProfileCertificationPacket, BuildRemoteCertificationArtifactError> {
    let packet: BuildRemoteProfileCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-build-remote-boundary-component-certification-proof/support_export.json"
    )))
    .map_err(BuildRemoteCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(BuildRemoteCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum BuildRemoteCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<BuildRemoteCertificationViolation>),
}

impl fmt::Display for BuildRemoteCertificationArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "certification export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "certification export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for BuildRemoteCertificationArtifactError {}

/// Validation failure for M05-1083 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildRemoteCertificationViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    MissingIdentity,
    WrongCanonicalBundle,
    DuplicateId { id: String },
    IncompleteRow { id: String },
    AxisCoverageIncomplete { id: String },
    MalformedAxisOutcome { id: String },
    RowMissingCanonicalBundle { id: String },
    GuardrailViolated { id: String },
    NonLocalProfileClaimsLiveTruth { id: String },
    ExportParityNotCertified { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    ProfileBlocked { id: String },
    ProfileCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawRemoteMaterialInExport,
}

impl fmt::Display for BuildRemoteCertificationViolation {
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
            Self::WrongCanonicalBundle => {
                write!(
                    f,
                    "packet does not cite the canonical build/remote-boundary proof bundle"
                )
            }
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete certification row: {id}"),
            Self::AxisCoverageIncomplete { id } => {
                write!(
                    f,
                    "row {id} does not score every certification axis exactly once"
                )
            }
            Self::MalformedAxisOutcome { id } => {
                write!(
                    f,
                    "row {id} has an axis outcome whose disclosure fields disagree with its state"
                )
            }
            Self::RowMissingCanonicalBundle { id } => {
                write!(
                    f,
                    "row {id} does not cite the one canonical build/remote-boundary proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaches a B128 spec guardrail: it implies exact continuity after a \
material change, hides local-safe / companion handoff in overflow only, or lets \
lower-confidence discovery overwrite a resolved target without review"
                )
            }
            Self::NonLocalProfileClaimsLiveTruth { id } => {
                write!(
                    f,
                    "row {id} certifies a live first-party full-truth claim on a non-local profile"
                )
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on CLI/export parity (text / JSON / Markdown reconstruction)"
                )
            }
            Self::CertifiedClaimExceedsClaim { id } => {
                write!(
                    f,
                    "row {id} certifies a claim stronger than the claimed one"
                )
            }
            Self::StatusDerivationStale { id } => {
                write!(
                    f,
                    "row {id} stored status disagrees with a fresh derivation"
                )
            }
            Self::ProfileBlocked { id } => {
                write!(
                    f,
                    "row {id} is blocked (red): a degraded axis is hidden behind a fresh first-party \
claim, a guardrail is breached, CLI/export parity dropped, or the narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 build / remote / managed profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen build/remote-boundary component family is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawRemoteMaterialInExport => {
                write!(f, "export contains raw remote / credential material")
            }
        }
    }
}

impl Error for BuildRemoteCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&BuildRemoteAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != BuildRemoteAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes
/// the build/remote-boundary generics the spec forbids collapsing distinct adapter
/// confidence, discovery drift, host boundary, execution origin, lifecycle, continuity,
/// and expiry truth into (whole-label matches so a full sentence naming a concrete host,
/// target, or lifecycle state is not flagged).
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
            | "stale"
            | "unverified"
            | "offline"
            | "warning"
            | "blocked"
            | "loading"
            | "content"
            | "expired"
            | "rebuilt"
            | "remote"
            | "managed"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

// --------------------------------------------------------------------------
// Seed builder — the one source of truth shared by the tests and the on-disk
// support export so both stay byte-aligned.
// --------------------------------------------------------------------------

/// Builds the canonical, checked-in M05-1083 certification packet. Certifies all eight
/// claimed M5 build / remote / managed profiles: four deliver their claim (green) and
/// four auto-narrow a not-current truth axis to a weaker boundary-support ceiling
/// (yellow). No profile hides drift or breaches a guardrail (red).
pub fn seeded_m5_build_remote_boundary_component_certification_packet(
) -> BuildRemoteProfileCertificationPacket {
    BuildRemoteProfileCertificationPacket::new(BuildRemoteProfileCertificationPacketInput {
        packet_id: "m5-build-remote-boundary-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-11T00:00:00Z".to_owned(),
        matrix_ref: BUILD_REMOTE_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: BUILD_REMOTE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:build-remote-boundary-component-certification:{id}"),
        BUILD_REMOTE_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> BuildRemoteCertExportParity {
    BuildRemoteCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: BuildRemoteCertificationAxis) -> &'static str {
    match axis {
        BuildRemoteCertificationAxis::Visual => {
            "adapter confidence, discovery drift, host ownership, execution origin, lifecycle state, persistence class, continuity, expiry timing, and local-safe continuation shown on-surface"
        }
        BuildRemoteCertificationAxis::Keyboard => {
            "the same inspect / review / reconnect / export-before-loss / renew actions are keyboard-reachable"
        }
        BuildRemoteCertificationAxis::ScreenReader => {
            "the same boundary truth is announced non-visually, never color/glyph-only"
        }
        BuildRemoteCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        BuildRemoteCertificationAxis::DegradedState => {
            "a stale, unverified, or unsupported reading honestly downgrades the FullTruth/ResolvedTruth claim rather than reading as fresh first-party local truth"
        }
        BuildRemoteCertificationAxis::BoundaryTruth => {
            "host ownership, execution origin, lifecycle, continuity, and expiry stay explicit and never collapse into generic status wording, imply exact continuity after a material change, hide local-safe / companion handoff in overflow only, or let lower-confidence discovery overwrite a resolved target"
        }
    }
}

fn seed_certified(axis: BuildRemoteCertificationAxis) -> BuildRemoteAxisOutcome {
    BuildRemoteAxisOutcome {
        axis,
        state: BuildRemoteAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: BuildRemoteCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5BuildRemoteDowngradeTrigger,
) -> BuildRemoteAxisOutcome {
    BuildRemoteAxisOutcome {
        axis,
        state: BuildRemoteAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<BuildRemoteAxisOutcome> {
    BuildRemoteCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: BuildRemoteCertificationAxis,
    outcome: BuildRemoteAxisOutcome,
) -> Vec<BuildRemoteAxisOutcome> {
    BuildRemoteCertificationAxis::ALL
        .iter()
        .copied()
        .map(|a| {
            if a == axis {
                outcome.clone()
            } else {
                seed_certified(a)
            }
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn seed_row(
    row_id: &str,
    profile: M5BuildRemoteCertifiedProfile,
    claimed_claim: M5BuildRemoteAccessClaim,
    certified_claim: M5BuildRemoteAccessClaim,
    consumed_families: &[M5BuildRemoteBoundaryComponentFamily],
    axis_outcomes: Vec<BuildRemoteAxisOutcome>,
    claim_auto_narrow: Option<BuildRemoteClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> BuildRemoteProfileCertificationRow {
    let mut row = BuildRemoteProfileCertificationRow {
        record_kind: BUILD_REMOTE_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: BUILD_REMOTE_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: BuildRemoteCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: BUILD_REMOTE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: BuildRemoteProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            BUILD_REMOTE_CERT_MATRIX_REF.to_owned(),
            BUILD_REMOTE_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-11T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: BuildRemoteCertificationAxis,
    from_claim: M5BuildRemoteAccessClaim,
    to_claim: M5BuildRemoteAccessClaim,
    label: &str,
) -> BuildRemoteClaimAutoNarrow {
    BuildRemoteClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<BuildRemoteProfileCertificationRow> {
    use BuildRemoteCertificationAxis as Ax;
    use M5BuildRemoteAccessClaim::*;
    use M5BuildRemoteBoundaryComponentFamily::*;
    use M5BuildRemoteCertifiedProfile as P;
    use M5BuildRemoteDowngradeTrigger as Trig;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:local-execution",
            P::LocalExecution,
            FullTruth,
            FullTruth,
            &[HostBoundaryStrip, ExecutionOriginReceiptRow],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "host_boundary"],
            &[
                "host-boundary strip names the local, first-party-local host and its owning runtime",
                "execution-origin receipt row records the resolved target identity and export-safe lineage",
                "keyboard/screen-reader reach preserved for the host strip and the receipt row",
                "boundary-truth: local execution is the only profile that certifies a live first-party full-truth claim",
            ],
        ),
        seed_row(
            "cert:ssh-execution",
            P::SshExecution,
            ResolvedTruth,
            ResolvedTruth,
            &[HostBoundaryStrip, AdapterConfidenceChip],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "execution_origin"],
            &[
                "host-boundary strip names the SSH remote host and never reads as local first-party truth",
                "adapter-confidence chip names the resolved adapter source and confidence band for the remote",
                "text / JSON / Markdown reconstruction certified for support replay",
                "boundary-truth: the SSH host ownership stays explicit through reconnect and degraded state",
            ],
        ),
        seed_row(
            "cert:container-execution",
            P::ContainerExecution,
            ResolvedTruth,
            ResolvedTruth,
            &[AdapterConfidenceChip, DiscoveryDiffCard],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "adapter_confidence"],
            &[
                "adapter-confidence chip names the container adapter source, confidence band, and discovery mode",
                "discovery-diff card names any target-identity drift and requires review before switch",
                "export preserves the adapter-confidence and discovery-drift truth",
                "boundary-truth: lower-confidence discovery never overwrites the resolved container target without review",
            ],
        ),
        seed_row(
            "cert:devcontainer-execution",
            P::DevcontainerExecution,
            ResolvedTruth,
            ResolvedTruth,
            &[DiscoveryDiffCard, ExecutionOriginReceiptRow],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "discovery_drift"],
            &[
                "discovery-diff card names the devcontainer image/template provenance and changed certainty",
                "execution-origin receipt row records the resolved devcontainer target identity and provenance",
                "text / JSON / Markdown reconstruction certified so support can replay the boundary story",
                "boundary-truth: devcontainer origin and provenance stay explicit, never generic status wording",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:managed-workspace",
            P::ManagedWorkspace,
            ResolvedTruth,
            Degraded,
            &[ManagedWorkspaceLifecycleCard, HostBoundaryStrip],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the managed workspace's live host / lifecycle proof aged out and is re-establishing",
                    "The managed workspace's live host and lifecycle proof has gone stale and is re-establishing, so the ResolvedTruth claim narrows to degraded and the lifecycle card shows a last-known reading rather than presenting it as current first-party truth",
                    Trig::ProofStale,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ResolvedTruth,
                Degraded,
                "Degraded managed workspace: the live host/lifecycle proof is stale and re-establishing; the lifecycle state shown is last-known, not a live current reading",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "managed-workspace lifecycle card keeps the lifecycle state, persistence class, and continuity visible through the stale window",
                "host-boundary strip names the managed host and never reads as local first-party truth",
                "degraded-state: ResolvedTruth narrows to degraded (auto-narrowed)",
                "boundary-truth: host ownership stays explicit while the proof re-establishes",
            ],
        ),
        seed_row(
            "cert:suspend-resume",
            P::SuspendResume,
            ResolvedTruth,
            Unverified,
            &[SuspendResumeRebuildReviewSheet, ManagedWorkspaceLifecycleCard],
            seed_certified_except(
                Ax::BoundaryTruth,
                seed_narrowed(
                    Ax::BoundaryTruth,
                    "continuity relative to the pre-suspend runtime cannot be verified after resume",
                    "The workspace resumed from suspend but continuity relative to the pre-suspend runtime cannot be verified, so the ResolvedTruth claim narrows to unverified and the review sheet names the preserved-vs-lost state rather than implying exact continuity across the suspend boundary",
                    Trig::ExactContinuityOverclaimed,
                ),
            ),
            Some(seed_narrow(
                Ax::BoundaryTruth,
                ResolvedTruth,
                Unverified,
                "Unverified resume continuity: the workspace resumed from suspend but continuity relative to the pre-suspend runtime cannot be verified; preserved and lost state are named and exact continuity is not implied",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "suspend/resume/rebuild review sheet names the lifecycle state, preserved-vs-lost state, and continuity class",
                "managed-workspace lifecycle card names the persistence class the resumed workspace kept",
                "boundary-truth: ResolvedTruth narrows to unverified (auto-narrowed)",
                "boundary-truth: the review sheet never implies exact continuity after the suspend/resume boundary",
            ],
        ),
        seed_row(
            "cert:rebuild-recreate",
            P::RebuildRecreate,
            ResolvedTruth,
            Unverified,
            &[SuspendResumeRebuildReviewSheet, ManagedWorkspaceLifecycleCard],
            seed_certified_except(
                Ax::BoundaryTruth,
                seed_narrowed(
                    Ax::BoundaryTruth,
                    "the workspace image / template / persistence class changed materially on rebuild / recreate",
                    "The workspace was rebuilt / recreated and its image, template, or persistence class changed materially, so the ResolvedTruth claim narrows to unverified and the review sheet names the changed provenance and preserved-vs-lost state before commit rather than implying exact continuity with the prior workspace",
                    Trig::PersistenceChangeHidden,
                ),
            ),
            Some(seed_narrow(
                Ax::BoundaryTruth,
                ResolvedTruth,
                Unverified,
                "Unverified rebuild continuity: the image/template/persistence class changed materially on rebuild/recreate; changed provenance and preserved-vs-lost state are named before commit and exact continuity is not implied",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "suspend/resume/rebuild review sheet names the changed template/image provenance and requires review before commit",
                "managed-workspace lifecycle card names the changed persistence class after rebuild/recreate",
                "boundary-truth: ResolvedTruth narrows to unverified (auto-narrowed)",
                "boundary-truth: a material image/template/persistence change never implies exact continuity",
            ],
        ),
        seed_row(
            "cert:expiry-local-safe",
            P::ExpiryLocalSafe,
            ResolvedTruth,
            Unsupported,
            &[WorkspaceExpiryBanner, LocalSafeContinuationCard],
            seed_certified_except(
                Ax::BoundaryTruth,
                seed_narrowed(
                    Ax::BoundaryTruth,
                    "the workspace expired so live remote state is unsupported and continuation falls back to local-safe",
                    "The workspace crossed its expiry boundary, so live remote state is unsupported and the ResolvedTruth claim narrows to unsupported; the expiry banner names the expiry timing and the local-safe continuation card names the preserved files, the lost live state, and the next safe actions rather than implying the expired workspace still offers exact continuity",
                    Trig::ExpiryTimingUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::BoundaryTruth,
                ResolvedTruth,
                Unsupported,
                "Expired workspace, local-safe continuation: live remote state is unsupported after expiry; the expiry timing, preserved files, lost live state, and next safe actions are named and no exact continuity is claimed",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "workspace-expiry banner names the expiry timing, the triggering owner/source, and the affected capabilities",
                "local-safe continuation card names the preserved files/context, the lost live state, and the next safe actions and is never hidden behind overflow only",
                "boundary-truth: ResolvedTruth narrows to unsupported (auto-narrowed)",
                "CLI/export parity certified so automation can replay the expiry and local-safe continuation truth",
            ],
        ),
    ]
}
