//! Surface certification of package-explorer-row, manifest-scope-switcher,
//! install-review-sheet, registry-or-mirror-row, script-risk-notice,
//! lockfile-impact-card, grouped-update-planner, and rollback-checkpoint-strip
//! truth on every claimed M5 package-management surface.
//!
//! This module is the closing certification capstone over the eight shared
//! package-management components frozen in
//! [`crate::freeze_the_m5_package_management_component_matrix`], implemented by the
//! package-explorer-row, manifest-scope / registry-or-mirror, install-review /
//! lockfile-impact, and script-risk / grouped-update / rollback-checkpoint lanes,
//! adopted by the shared consumers in
//! [`crate::add_shared_package_explorer_search_detail_help_support_diagnostics_and_export_consumers_so_package_components_keep_scope_auth_and_lockfile_language_aligned`],
//! and proven across assistive, headless, and exported forms by
//! [`crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_manifest_scope_registry_freshness_auth_state_or_lockfile_impact_truth_is_stale_or_partial_across_claimed_m5_package_components`].
//!
//! Where the implement lanes ship the components and the consumer / accessibility
//! lanes prove scope / auth / lockfile parity, this lane certifies the release
//! claim: that on every claimed M5 package-management surface — package explorer,
//! dependency search detail, install-review sheet, help surface, support export,
//! exported package-review packet, headless CLI, and diagnostics — the same
//! controlled component truth is presented with no hidden manifest-scope,
//! registry-source, auth, script-risk, or lockfile-churn drift. Each certified
//! surface row scores six certification axes
//! ([`PackageComponentCertificationAxis`]): the visual, keyboard, screen-reader,
//! and CLI/export axes that every claim must always pass, the degraded-state axis
//! that narrows a claim when manifest scope, registry freshness, auth state,
//! lockfile impact, or rollback truth weakens, and the scope-and-source-provenance
//! axis that keeps the certification honest — a certified surface never implies its
//! manifest scope is full, its registry is fresh, that no scripts run, or that
//! lockfile churn is small.
//!
//! A surface earns [`PackageComponentSurfaceClaimStatus::CertifiedParity`] only when
//! its certified claim equals its claimed claim, no axis narrows, and component
//! truth is preserved. It narrows to
//! [`PackageComponentSurfaceClaimStatus::NarrowedParity`] the moment an axis narrows
//! or the certified claim drops below the claimed one, and it fails to
//! [`PackageComponentSurfaceClaimStatus::ParityBlocked`] whenever the target
//! manifest, direct/transitive relation, registry source, auth posture,
//! script/native-build risk, lockfile churn, grouped-update reason, or
//! rollback/checkpoint identity is flattened out of the export. That last rule is
//! the delta of this capstone: certification may narrow a claim but may never drop
//! the component's meaning.
//!
//! The packet references upstream component, consumer, and accessibility contracts
//! by id rather than embedding their content. Raw registry credentials, tokens, and
//! live manifest payloads stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-package-management-component-certification.schema.json`](../../../../schemas/ui/m5-package-management-component-certification.schema.json).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_package_management_component_matrix::M5PackageComponent;
use crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_manifest_scope_registry_freshness_auth_state_or_lockfile_impact_truth_is_stale_or_partial_across_claimed_m5_package_components::PackageComponentClaimTier;

/// Stable record-kind tag carried by [`PackageComponentCertificationPacket`].
pub const M5_PACKAGE_CERTIFICATION_RECORD_KIND: &str =
    "m5_package_management_component_surface_certification_truth";

/// Schema version for package-management component surface certification records.
pub const M5_PACKAGE_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_PACKAGE_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/ui/m5-package-management-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PACKAGE_CERTIFICATION_DOC_REF: &str =
    "docs/deps/m5/certify_package_management_component_truth_on_every_claimed_m5_package_management_surface.md";

/// Repo-relative path of the frozen component matrix this certification builds on.
pub const M5_PACKAGE_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-package-management-component-matrix.schema.json";

/// Repo-relative path of the shared-consumer parity contract this certification builds on.
pub const M5_PACKAGE_CERTIFICATION_CONSUMER_CONTRACT_REF: &str =
    "schemas/ui/m5-package-management-component-consumer.schema.json";

/// Repo-relative path of the accessibility / headless / export parity contract this certification builds on.
pub const M5_PACKAGE_CERTIFICATION_ACCESSIBILITY_CONTRACT_REF: &str =
    "schemas/ui/m5-package-management-component-accessibility-parity.schema.json";

/// Repo-relative path of the package-explorer-row controls contract.
pub const M5_PACKAGE_CERTIFICATION_EXPLORER_ROW_CONTRACT_REF: &str =
    "schemas/ui/m5-package-explorer-row.schema.json";

/// Repo-relative path of the manifest-scope / registry-or-mirror controls contract.
pub const M5_PACKAGE_CERTIFICATION_MANIFEST_SCOPE_REGISTRY_CONTRACT_REF: &str =
    "schemas/ui/m5-manifest-scope-registry-controls.schema.json";

/// Repo-relative path of the install-review / lockfile-impact controls contract.
pub const M5_PACKAGE_CERTIFICATION_INSTALL_REVIEW_LOCKFILE_CONTRACT_REF: &str =
    "schemas/ui/m5-install-review-lockfile-controls.schema.json";

/// Repo-relative path of the script-risk / grouped-update / rollback controls contract.
pub const M5_PACKAGE_CERTIFICATION_SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_CONTRACT_REF: &str =
    "schemas/ui/m5-script-risk-grouped-update-rollback-controls.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_PACKAGE_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/ui/m5-package-management-component-certification";

/// Repo-relative path of the release-proof support export (read directly by tests).
pub const M5_PACKAGE_CERTIFICATION_RELEASE_PROOF_ARTIFACT_REF: &str =
    "artifacts/release/m5-package-management-certification-proof/support_export.json";

/// Repo-relative path of the release-proof certification matrix CSV.
pub const M5_PACKAGE_CERTIFICATION_RELEASE_PROOF_MATRIX_REF: &str =
    "artifacts/release/m5-package-management-certification-proof/matrix.csv";

/// Repo-relative path of the release-proof report.
pub const M5_PACKAGE_CERTIFICATION_RELEASE_PROOF_REPORT_REF: &str =
    "artifacts/release/m5-package-management-certification-proof/report.md";

/// Canonical component contract that a certified surface row must cite for a
/// component it presents.
///
/// Each of the eight shared components resolves to the checked-in controls schema of
/// the lane that implemented it: the package-explorer-row controls (explorer rows),
/// the manifest-scope / registry-or-mirror controls (manifest-scope switchers and
/// registry/mirror rows), the install-review / lockfile-impact controls
/// (install-review sheets and lockfile-impact cards), and the script-risk /
/// grouped-update / rollback-checkpoint controls (script-risk notices, grouped-update
/// planners, and rollback/checkpoint strips).
pub const fn certification_component_canonical_schema_ref(
    component: M5PackageComponent,
) -> &'static str {
    match component {
        M5PackageComponent::PackageExplorerRow => {
            M5_PACKAGE_CERTIFICATION_EXPLORER_ROW_CONTRACT_REF
        }
        M5PackageComponent::ManifestScopeSwitcher | M5PackageComponent::RegistryOrMirrorRow => {
            M5_PACKAGE_CERTIFICATION_MANIFEST_SCOPE_REGISTRY_CONTRACT_REF
        }
        M5PackageComponent::InstallReviewSheet | M5PackageComponent::LockfileImpactCard => {
            M5_PACKAGE_CERTIFICATION_INSTALL_REVIEW_LOCKFILE_CONTRACT_REF
        }
        M5PackageComponent::ScriptRiskNotice
        | M5PackageComponent::GroupedUpdatePlanner
        | M5PackageComponent::RollbackCheckpointStrip => {
            M5_PACKAGE_CERTIFICATION_SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_CONTRACT_REF
        }
    }
}

/// A claimed M5 package-management surface whose component truth this packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PackageComponentCertifiedSurface {
    /// Desktop package explorer surface.
    PackageExplorerSurface,
    /// Dependency search / detail surface.
    DependencySearchDetailSurface,
    /// Install / update / remove review sheet surface.
    InstallReviewSheetSurface,
    /// Help / About package-management surface.
    HelpPackageSurface,
    /// Support export bundle.
    SupportExport,
    /// Exported package-review packet (offline / publish-later review pack).
    ExportedPackageReviewPacket,
    /// Headless CLI package output.
    CliHeadless,
    /// Diagnostics package surface.
    Diagnostics,
}

impl M5PackageComponentCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::PackageExplorerSurface,
        Self::DependencySearchDetailSurface,
        Self::InstallReviewSheetSurface,
        Self::HelpPackageSurface,
        Self::SupportExport,
        Self::ExportedPackageReviewPacket,
        Self::CliHeadless,
        Self::Diagnostics,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackageExplorerSurface => "package_explorer_surface",
            Self::DependencySearchDetailSurface => "dependency_search_detail_surface",
            Self::InstallReviewSheetSurface => "install_review_sheet_surface",
            Self::HelpPackageSurface => "help_package_surface",
            Self::SupportExport => "support_export",
            Self::ExportedPackageReviewPacket => "exported_package_review_packet",
            Self::CliHeadless => "cli_headless",
            Self::Diagnostics => "diagnostics",
        }
    }
}

/// A certification axis scored on every certified surface row.
///
/// The first four axes are always-on: a claimed component must always pass them on
/// every surface. [`DegradedState`](Self::DegradedState) narrows a claim when
/// manifest scope, registry freshness, auth state, lockfile impact, or rollback
/// truth weakens. [`ScopeAndSourceProvenance`](Self::ScopeAndSourceProvenance) is the
/// certification-specific separation axis: it keeps the manifest-scope,
/// registry-source, script-risk, and lockfile-churn distinctions explicit so a
/// certified surface never implies its scope is full, its registry is fresh, that no
/// scripts run, or that lockfile churn is small.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageComponentCertificationAxis {
    /// Visual rendering carries the controlled component truth.
    Visual,
    /// Keyboard reach and operation carry the controlled component truth.
    Keyboard,
    /// Screen-reader labelling carries the controlled component truth.
    ScreenReader,
    /// CLI and export forms carry the controlled component truth.
    CliExport,
    /// Degraded manifest-scope, registry, auth, lockfile, or rollback state narrows the claim honestly.
    DegradedState,
    /// The manifest-scope / registry-source / script-risk / lockfile-churn distinction stays explicit; certified never implies safe one-click.
    ScopeAndSourceProvenance,
}

impl PackageComponentCertificationAxis {
    /// Every axis, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Visual,
        Self::Keyboard,
        Self::ScreenReader,
        Self::CliExport,
        Self::DegradedState,
        Self::ScopeAndSourceProvenance,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::ScopeAndSourceProvenance => "scope_and_source_provenance",
        }
    }

    /// Whether this axis must always be certified on every claimed surface.
    pub const fn is_always_on(self) -> bool {
        matches!(
            self,
            Self::Visual | Self::Keyboard | Self::ScreenReader | Self::CliExport
        )
    }
}

/// The certification state of a single axis on a surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageComponentAxisCertificationState {
    /// The axis is fully certified on this surface.
    Certified,
    /// The axis is certified but narrowed (an honest fallback is disclosed).
    NarrowedCertified,
    /// The axis is not certified on this surface (it is honestly out of scope here).
    NotCertifiedHere,
}

impl PackageComponentAxisCertificationState {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::NarrowedCertified => "narrowed_certified",
            Self::NotCertifiedHere => "not_certified_here",
        }
    }
}

/// The certification status a surface row earns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageComponentSurfaceClaimStatus {
    /// Green: certified claim equals claimed claim, no axis narrows, truth preserved.
    CertifiedParity,
    /// Yellow: certification is narrowed but component truth is preserved.
    NarrowedParity,
    /// Red: component truth was flattened out of this surface.
    ParityBlocked,
}

impl PackageComponentSurfaceClaimStatus {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedParity => "certified_parity",
            Self::NarrowedParity => "narrowed_parity",
            Self::ParityBlocked => "parity_blocked",
        }
    }

    /// Whether the surface is fully certified (green).
    pub const fn is_green(self) -> bool {
        matches!(self, Self::CertifiedParity)
    }

    /// Whether the surface is blocked (red).
    pub const fn is_red(self) -> bool {
        matches!(self, Self::ParityBlocked)
    }
}

/// Downgrade trigger that can narrow a certified surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageComponentCertificationDowngradeTrigger {
    /// Proof packet has gone stale relative to its freshness SLO.
    ProofStale,
    /// An upstream evidence packet failed validation or is missing.
    EvidencePacketInvalid,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// The target manifest scope is only partially known.
    ManifestScopePartial,
    /// Registry freshness is stale; the answer is mirror- or offline-sourced.
    RegistryFreshnessStale,
    /// Registry access requires authentication that is not satisfied.
    AuthStateUnsatisfied,
    /// The lockfile-impact / churn computation is unavailable or stale.
    LockfileImpactUnavailable,
    /// Script or native-build risk could not be established and is disclosed as unknown.
    ScriptRiskUndisclosed,
    /// The rollback / checkpoint truth is unavailable or policy-blocked.
    RollbackCheckpointUnavailable,
    /// An upstream dependency row narrowed.
    UpstreamDependencyNarrowed,
}

impl PackageComponentCertificationDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProofStale,
        Self::EvidencePacketInvalid,
        Self::PolicyBlocked,
        Self::ManifestScopePartial,
        Self::RegistryFreshnessStale,
        Self::AuthStateUnsatisfied,
        Self::LockfileImpactUnavailable,
        Self::ScriptRiskUndisclosed,
        Self::RollbackCheckpointUnavailable,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::EvidencePacketInvalid => "evidence_packet_invalid",
            Self::PolicyBlocked => "policy_blocked",
            Self::ManifestScopePartial => "manifest_scope_partial",
            Self::RegistryFreshnessStale => "registry_freshness_stale",
            Self::AuthStateUnsatisfied => "auth_state_unsatisfied",
            Self::LockfileImpactUnavailable => "lockfile_impact_unavailable",
            Self::ScriptRiskUndisclosed => "script_risk_undisclosed",
            Self::RollbackCheckpointUnavailable => "rollback_checkpoint_unavailable",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Derives the certification status of a surface from its claims and axis narrowing.
///
/// Component truth is the hard gate: if the target manifest, direct/transitive
/// relation, registry source, auth posture, script/native-build risk, lockfile churn,
/// grouped-update reason, or rollback/checkpoint identity is flattened, the surface is
/// [`PackageComponentSurfaceClaimStatus::ParityBlocked`] regardless of the claim
/// tiers. Otherwise a certified claim below the claimed one, or any narrowed axis,
/// narrows the surface to [`PackageComponentSurfaceClaimStatus::NarrowedParity`]; only
/// a full, un-narrowed claim earns
/// [`PackageComponentSurfaceClaimStatus::CertifiedParity`].
pub const fn derive_package_component_surface_claim_status(
    claimed: PackageComponentClaimTier,
    certified: PackageComponentClaimTier,
    component_truth_preserved: bool,
    has_narrowed_axes: bool,
) -> PackageComponentSurfaceClaimStatus {
    if !component_truth_preserved {
        PackageComponentSurfaceClaimStatus::ParityBlocked
    } else if certified.rank() < claimed.rank() || has_narrowed_axes {
        PackageComponentSurfaceClaimStatus::NarrowedParity
    } else {
        PackageComponentSurfaceClaimStatus::CertifiedParity
    }
}

/// One axis outcome on a certified surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentCertAxisOutcome {
    /// The certification axis scored.
    pub axis: PackageComponentCertificationAxis,
    /// The state the axis earned on this surface.
    pub state: PackageComponentAxisCertificationState,
    /// Human-readable note explaining the outcome (never empty).
    pub note: String,
}

/// One certified surface row: a claimed M5 package-management surface and the
/// component truth it presents, scored across the six certification axes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentCertifiedSurfaceRow {
    /// Stable row id.
    pub row_id: String,
    /// The claimed M5 package-management surface.
    pub surface: M5PackageComponentCertifiedSurface,
    /// The shared components this surface presents (non-empty).
    pub components_present: Vec<M5PackageComponent>,
    /// The claim tier the surface claims for its components.
    pub claimed_claim: PackageComponentClaimTier,
    /// The claim tier the certification actually earns.
    pub certified_claim: PackageComponentClaimTier,
    /// The certification status the surface earns.
    pub status: PackageComponentSurfaceClaimStatus,
    /// Per-axis outcomes; must cover all six axes.
    pub axis_outcomes: Vec<PackageComponentCertAxisOutcome>,
    /// The axes that narrowed on this surface (subset of the axis outcomes).
    pub narrowed_axes: Vec<PackageComponentCertificationAxis>,
    /// The downgrade trigger disclosed when the surface narrows.
    pub downgrade_trigger: Option<PackageComponentCertificationDowngradeTrigger>,
    /// Delta invariant: the component's manifest scope, direct/transitive relation,
    /// registry source, auth posture, script/native-build risk, lockfile churn,
    /// grouped-update reason, and rollback/checkpoint truth is preserved (never
    /// flattened).
    pub component_truth_preserved: bool,
    /// Keyboard reach / operation label (never empty).
    pub keyboard_label: String,
    /// Screen-reader label (never empty).
    pub screen_reader_label: String,
    /// CLI enum token (never empty).
    pub cli_enum_token: String,
    /// Export enum token (never empty).
    pub export_enum_token: String,
    /// Human-readable explanation field (never empty).
    pub explanation_field: String,
    /// Source contract refs this row points at.
    pub source_contract_refs: Vec<String>,
}

impl PackageComponentCertifiedSurfaceRow {
    /// The status this row should carry, derived from its claims and narrowing.
    pub fn derived_status(&self) -> PackageComponentSurfaceClaimStatus {
        derive_package_component_surface_claim_status(
            self.claimed_claim,
            self.certified_claim,
            self.component_truth_preserved,
            !self.narrowed_axes.is_empty(),
        )
    }

    /// Whether the recorded status matches the derived one.
    pub fn status_is_consistent(&self) -> bool {
        self.status == self.derived_status()
    }

    /// Whether every axis is scored on this row.
    pub fn covers_all_axes(&self) -> bool {
        PackageComponentCertificationAxis::ALL.iter().all(|axis| {
            self.axis_outcomes
                .iter()
                .any(|outcome| outcome.axis == *axis)
        })
    }

    /// Whether every parity / export field is present.
    pub fn parity_fields_present(&self) -> bool {
        !self.keyboard_label.trim().is_empty()
            && !self.screen_reader_label.trim().is_empty()
            && !self.cli_enum_token.trim().is_empty()
            && !self.export_enum_token.trim().is_empty()
            && !self.explanation_field.trim().is_empty()
    }

    /// Whether the certified claim stays at or below the claimed one.
    pub fn certified_claim_within_claimed(&self) -> bool {
        self.certified_claim.rank() <= self.claimed_claim.rank()
    }

    /// Whether the narrowed axes agree with the axis outcomes marked narrowed.
    pub fn narrowed_axes_consistent(&self) -> bool {
        let narrowed: BTreeSet<PackageComponentCertificationAxis> =
            self.narrowed_axes.iter().copied().collect();
        for outcome in &self.axis_outcomes {
            let marked_narrowed =
                outcome.state == PackageComponentAxisCertificationState::NarrowedCertified;
            if marked_narrowed != narrowed.contains(&outcome.axis) {
                return false;
            }
        }
        true
    }

    /// Whether this row cites the canonical matrix and each present component's schema.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let refs: BTreeSet<&str> = self
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_PACKAGE_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF) {
            return false;
        }
        self.components_present.iter().all(|component| {
            refs.contains(certification_component_canonical_schema_ref(*component))
        })
    }
}

/// Aggregate certification summary across all surface rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentCertificationSummary {
    /// Total certified surface rows.
    pub total_rows: u32,
    /// Count of green (fully certified) surfaces.
    pub certified_count: u32,
    /// Count of yellow (narrowed) surfaces.
    pub narrowed_count: u32,
    /// Count of red (blocked) surfaces.
    pub blocked_count: u32,
    /// True when every surface preserves component truth (no red).
    pub all_rows_preserve_component_truth: bool,
    /// True when all eight claimed surfaces are covered.
    pub all_surfaces_covered: bool,
    /// True when all eight shared components appear across the surfaces.
    pub all_components_covered: bool,
    /// Human-readable certification note.
    pub certification_note: String,
}

impl PackageComponentCertificationSummary {
    /// Recomputes the summary from a surface row set.
    pub fn from_rows(rows: &[PackageComponentCertifiedSurfaceRow]) -> Self {
        let mut certified = 0u32;
        let mut narrowed = 0u32;
        let mut blocked = 0u32;
        let mut seen_surfaces: BTreeSet<M5PackageComponentCertifiedSurface> = BTreeSet::new();
        let mut seen_components: BTreeSet<M5PackageComponent> = BTreeSet::new();
        for row in rows {
            match row.status {
                PackageComponentSurfaceClaimStatus::CertifiedParity => certified += 1,
                PackageComponentSurfaceClaimStatus::NarrowedParity => narrowed += 1,
                PackageComponentSurfaceClaimStatus::ParityBlocked => blocked += 1,
            }
            seen_surfaces.insert(row.surface);
            for component in &row.components_present {
                seen_components.insert(*component);
            }
        }
        let all_surfaces_covered = M5PackageComponentCertifiedSurface::ALL
            .iter()
            .all(|surface| seen_surfaces.contains(surface));
        let all_components_covered = M5PackageComponent::ALL
            .iter()
            .all(|component| seen_components.contains(component));
        let all_preserve = blocked == 0;
        let certification_note = if all_preserve {
            format!(
                "{certified} surface(s) certified, {narrowed} narrowed; all preserve component truth"
            )
        } else {
            format!("{blocked} surface(s) blocked: component truth was flattened")
        };
        Self {
            total_rows: rows.len() as u32,
            certified_count: certified,
            narrowed_count: narrowed,
            blocked_count: blocked,
            all_rows_preserve_component_truth: all_preserve,
            all_surfaces_covered,
            all_components_covered,
            certification_note,
        }
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentCertificationTrustReview {
    /// Every claimed surface presents the same controlled component truth.
    pub same_component_truth_on_every_surface: bool,
    /// Target manifest and direct/transitive relation stay explicit, never generic package chrome.
    pub manifest_scope_and_relation_explicit: bool,
    /// Registry source and freshness stay explicit, never flattened to a bare "installed".
    pub registry_source_and_freshness_explicit: bool,
    /// Auth posture stays explicit, never hidden behind a one-click affordance.
    pub auth_posture_explicit: bool,
    /// Script and native-build risk stays explicit before any install runs.
    pub script_and_native_build_risk_explicit: bool,
    /// Lockfile churn is quantified and never understated.
    pub lockfile_churn_quantified_not_understated: bool,
    /// Grouped-update reason and rollback/checkpoint identity stay explicit before write.
    pub grouped_update_reason_and_rollback_identity_explicit: bool,
    /// Certified never implies safe one-click: scope, registry, script, and churn stay explicit.
    pub certified_never_implies_safe_one_click: bool,
    /// Mirror/offline continuity stays explicit when the registry answer is not upstream-fresh.
    pub mirror_or_offline_continuity_explicit: bool,
    /// Certification narrows a claim rather than dropping the component's meaning.
    pub narrows_instead_of_dropping_meaning: bool,
    /// A surface that flattens component truth blocks its certification.
    pub flattened_truth_blocks_certification: bool,
}

impl PackageComponentCertificationTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.same_component_truth_on_every_surface
            && self.manifest_scope_and_relation_explicit
            && self.registry_source_and_freshness_explicit
            && self.auth_posture_explicit
            && self.script_and_native_build_risk_explicit
            && self.lockfile_churn_quantified_not_understated
            && self.grouped_update_reason_and_rollback_identity_explicit
            && self.certified_never_implies_safe_one_click
            && self.mirror_or_offline_continuity_explicit
            && self.narrows_instead_of_dropping_meaning
            && self.flattened_truth_blocks_certification
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentCertificationConsumerProjection {
    /// Package explorer surface shows the certified component truth.
    pub package_explorer_surface_shows_certification: bool,
    /// Dependency search / detail surface shows the certified component truth.
    pub dependency_search_detail_surface_shows_certification: bool,
    /// Install-review sheet surface shows the certified component truth.
    pub install_review_sheet_surface_shows_certification: bool,
    /// Help / About package surface shows the certified component truth.
    pub help_package_surface_shows_certification: bool,
    /// Support export shows the certified component truth.
    pub support_export_shows_certification: bool,
    /// Exported package-review packet shows the certified component truth.
    pub exported_package_review_packet_shows_certification: bool,
    /// CLI / headless shows the certified component truth.
    pub cli_headless_shows_certification: bool,
    /// Diagnostics shows the certified component truth.
    pub diagnostics_shows_certification: bool,
    /// Narrowed surfaces are visibly labelled rather than silently downgraded.
    pub narrowed_surfaces_visibly_labelled: bool,
}

impl PackageComponentCertificationConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.package_explorer_surface_shows_certification
            && self.dependency_search_detail_surface_shows_certification
            && self.install_review_sheet_surface_shows_certification
            && self.help_package_surface_shows_certification
            && self.support_export_shows_certification
            && self.exported_package_review_packet_shows_certification
            && self.cli_headless_shows_certification
            && self.diagnostics_shows_certification
            && self.narrowed_surfaces_visibly_labelled
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentCertificationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the certification.
    pub auto_narrow_on_stale: bool,
}

/// Per-surface observation fed to [`PackageComponentCertificationPacket::apply_downgrade_automation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageComponentCertObservation {
    /// Surface the observation applies to.
    pub surface: M5PackageComponentCertifiedSurface,
    /// True when the surface's manifest scope and registry source are currently fresh.
    pub scope_and_registry_fresh: bool,
    /// True when the surface still preserves component truth.
    pub component_truth_preserved: bool,
}

/// Constructor input for [`PackageComponentCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageComponentCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified surface rows.
    pub surface_rows: Vec<PackageComponentCertifiedSurfaceRow>,
    /// Aggregate certification summary.
    pub summary: PackageComponentCertificationSummary,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<PackageComponentCertificationDowngradeTrigger>,
    /// Trust review block.
    pub trust_review: PackageComponentCertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: PackageComponentCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: PackageComponentCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe package-management component surface certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentCertificationPacket {
    /// Record kind; must equal [`M5_PACKAGE_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PACKAGE_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified surface rows.
    pub surface_rows: Vec<PackageComponentCertifiedSurfaceRow>,
    /// Aggregate certification summary.
    pub summary: PackageComponentCertificationSummary,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<PackageComponentCertificationDowngradeTrigger>,
    /// Trust review block.
    pub trust_review: PackageComponentCertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: PackageComponentCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: PackageComponentCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl PackageComponentCertificationPacket {
    /// Builds a package-management component surface certification packet from stable-lane input.
    pub fn new(input: PackageComponentCertificationPacketInput) -> Self {
        Self {
            record_kind: M5_PACKAGE_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_PACKAGE_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            certification_label: input.certification_label,
            surface_rows: input.surface_rows,
            summary: input.summary,
            downgrade_triggers: input.downgrade_triggers,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Narrows surfaces whose manifest scope and registry source are no longer fresh
    /// and blocks surfaces that flatten component truth, then recomputes the summary.
    ///
    /// This is the downgrade automation: a surface reported with a flattened component
    /// truth blocks (red); a still-green surface whose manifest scope / registry source
    /// went stale narrows its full-management claim to a disclosed mirror-or-offline
    /// source, marks the scope-and-source-provenance axis narrowed, and discloses the
    /// registry-freshness trigger. Observations for surfaces not present in the packet
    /// are ignored; surfaces without an observation are left unchanged.
    pub fn apply_downgrade_automation(&mut self, observations: &[PackageComponentCertObservation]) {
        for row in &mut self.surface_rows {
            let Some(observation) = observations.iter().find(|obs| obs.surface == row.surface)
            else {
                continue;
            };
            if !observation.component_truth_preserved {
                row.component_truth_preserved = false;
            } else if !observation.scope_and_registry_fresh
                && row.status == PackageComponentSurfaceClaimStatus::CertifiedParity
            {
                if row.certified_claim.rank()
                    > PackageComponentClaimTier::MirrorOrOfflineSourced.rank()
                {
                    row.certified_claim = PackageComponentClaimTier::MirrorOrOfflineSourced;
                }
                if !row
                    .narrowed_axes
                    .contains(&PackageComponentCertificationAxis::ScopeAndSourceProvenance)
                {
                    row.narrowed_axes
                        .push(PackageComponentCertificationAxis::ScopeAndSourceProvenance);
                }
                for outcome in &mut row.axis_outcomes {
                    if outcome.axis == PackageComponentCertificationAxis::ScopeAndSourceProvenance {
                        outcome.state = PackageComponentAxisCertificationState::NarrowedCertified;
                        outcome.note =
                            "Manifest scope and registry freshness went stale; the claim narrows to a disclosed mirror-or-offline source and the scope-and-source provenance stays explicit"
                                .to_owned();
                    }
                }
                row.downgrade_trigger =
                    Some(PackageComponentCertificationDowngradeTrigger::RegistryFreshnessStale);
            }
            row.status = row.derived_status();
        }
        self.summary = PackageComponentCertificationSummary::from_rows(&self.surface_rows);
    }

    /// Validates the package-management component surface certification invariants.
    pub fn validate(&self) -> Vec<PackageComponentCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PACKAGE_CERTIFICATION_RECORD_KIND {
            violations.push(PackageComponentCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PACKAGE_CERTIFICATION_SCHEMA_VERSION {
            violations.push(PackageComponentCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(PackageComponentCertificationViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(PackageComponentCertificationViolation::DowngradeTriggersMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_summary(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(PackageComponentCertificationViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(PackageComponentCertificationViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(PackageComponentCertificationViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("package certification packet serializes"),
        ) {
            violations.push(PackageComponentCertificationViolation::RawPackageMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("package certification packet serializes")
    }

    /// Deterministic certification matrix CSV for release proof.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,component_truth_preserved\n",
        );
        for row in &self.surface_rows {
            let narrowed = row
                .narrowed_axes
                .iter()
                .map(|axis| axis.as_str())
                .collect::<Vec<_>>()
                .join("|");
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.row_id,
                row.surface.as_str(),
                row.claimed_claim.as_str(),
                row.certified_claim.as_str(),
                row.status.as_str(),
                narrowed,
                row.component_truth_preserved,
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Package-Management Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.certification_label));
        out.push_str(&format!(
            "- Surfaces: {} ({} certified, {} narrowed, {} blocked)\n",
            self.summary.total_rows,
            self.summary.certified_count,
            self.summary.narrowed_count,
            self.summary.blocked_count,
        ));
        out.push_str(&format!(
            "- All surfaces preserve component truth: {}\n",
            self.summary.all_rows_preserve_component_truth
        ));
        out.push_str(&format!("- Note: {}\n", self.summary.certification_note));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Certified surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "- **{}** [`{}`]: `{}` (claimed `{}`, certified `{}`)\n",
                row.surface.as_str(),
                row.row_id,
                row.status.as_str(),
                row.claimed_claim.as_str(),
                row.certified_claim.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in package certification export.
#[derive(Debug)]
pub enum PackageComponentCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PackageComponentCertificationViolation>),
}

impl fmt::Display for PackageComponentCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "package certification export parse failed: {error}"
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
                    "package certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for PackageComponentCertificationArtifactError {}

/// Validation failures emitted by [`PackageComponentCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageComponentCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No surface rows are present.
    SurfaceRowsMissing,
    /// A surface row is incomplete.
    RowIncomplete,
    /// A surface row lists no components.
    ComponentsMissingOnRow,
    /// A row is missing its keyboard label.
    KeyboardLabelMissing,
    /// A row is missing its screen-reader label.
    ScreenReaderLabelMissing,
    /// A row is missing its CLI enum token.
    CliEnumTokenMissing,
    /// A row is missing its export enum token.
    ExportEnumTokenMissing,
    /// A row is missing its explanation field.
    ExplanationFieldMissing,
    /// A row does not score all six certification axes.
    AxisCoverageMissing,
    /// An axis outcome is missing its explanatory note.
    AxisNoteMissing,
    /// A certified claim exceeds the claimed claim it certifies.
    CertifiedClaimExceedsClaimed,
    /// The recorded status does not agree with the derived one.
    StatusMismatch,
    /// The narrowed-axis list disagrees with the axis outcomes marked narrowed.
    NarrowedAxesInconsistent,
    /// A narrowed surface is missing its disclosed downgrade trigger.
    NarrowingWithoutTrigger,
    /// A surface flattened the component's scope / source / auth / script / lockfile / rollback truth.
    PackageComponentTruthDropped,
    /// A row does not cite the canonical matrix and component contracts.
    CanonicalContractReferenceMissing,
    /// Not every claimed surface appears among the rows.
    SurfaceCoverageMissing,
    /// Not every shared component appears across the surfaces.
    ComponentCoverageMissing,
    /// The summary does not agree with the surface rows.
    SummaryMismatch,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// Export contains raw package boundary material.
    RawPackageMaterialInExport,
}

impl PackageComponentCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::SurfaceRowsMissing => "surface_rows_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::ComponentsMissingOnRow => "components_missing_on_row",
            Self::KeyboardLabelMissing => "keyboard_label_missing",
            Self::ScreenReaderLabelMissing => "screen_reader_label_missing",
            Self::CliEnumTokenMissing => "cli_enum_token_missing",
            Self::ExportEnumTokenMissing => "export_enum_token_missing",
            Self::ExplanationFieldMissing => "explanation_field_missing",
            Self::AxisCoverageMissing => "axis_coverage_missing",
            Self::AxisNoteMissing => "axis_note_missing",
            Self::CertifiedClaimExceedsClaimed => "certified_claim_exceeds_claimed",
            Self::StatusMismatch => "status_mismatch",
            Self::NarrowedAxesInconsistent => "narrowed_axes_inconsistent",
            Self::NarrowingWithoutTrigger => "narrowing_without_trigger",
            Self::PackageComponentTruthDropped => "package_component_truth_dropped",
            Self::CanonicalContractReferenceMissing => "canonical_contract_reference_missing",
            Self::SurfaceCoverageMissing => "surface_coverage_missing",
            Self::ComponentCoverageMissing => "component_coverage_missing",
            Self::SummaryMismatch => "summary_mismatch",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RawPackageMaterialInExport => "raw_package_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable package certification export.
pub fn current_package_component_certification_export(
) -> Result<PackageComponentCertificationPacket, PackageComponentCertificationArtifactError> {
    let packet: PackageComponentCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-package-management-certification-proof/support_export.json"
    )))
    .map_err(PackageComponentCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PackageComponentCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Canonical trust review block with every invariant satisfied.
pub fn canonical_trust_review() -> PackageComponentCertificationTrustReview {
    PackageComponentCertificationTrustReview {
        same_component_truth_on_every_surface: true,
        manifest_scope_and_relation_explicit: true,
        registry_source_and_freshness_explicit: true,
        auth_posture_explicit: true,
        script_and_native_build_risk_explicit: true,
        lockfile_churn_quantified_not_understated: true,
        grouped_update_reason_and_rollback_identity_explicit: true,
        certified_never_implies_safe_one_click: true,
        mirror_or_offline_continuity_explicit: true,
        narrows_instead_of_dropping_meaning: true,
        flattened_truth_blocks_certification: true,
    }
}

/// Canonical consumer projection block with every surface projecting certification truth.
pub fn canonical_consumer_projection() -> PackageComponentCertificationConsumerProjection {
    PackageComponentCertificationConsumerProjection {
        package_explorer_surface_shows_certification: true,
        dependency_search_detail_surface_shows_certification: true,
        install_review_sheet_surface_shows_certification: true,
        help_package_surface_shows_certification: true,
        support_export_shows_certification: true,
        exported_package_review_packet_shows_certification: true,
        cli_headless_shows_certification: true,
        diagnostics_shows_certification: true,
        narrowed_surfaces_visibly_labelled: true,
    }
}

/// Canonical source contract refs that every certification export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        M5_PACKAGE_CERTIFICATION_SCHEMA_REF.to_owned(),
        M5_PACKAGE_CERTIFICATION_DOC_REF.to_owned(),
        M5_PACKAGE_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        M5_PACKAGE_CERTIFICATION_CONSUMER_CONTRACT_REF.to_owned(),
        M5_PACKAGE_CERTIFICATION_ACCESSIBILITY_CONTRACT_REF.to_owned(),
        M5_PACKAGE_CERTIFICATION_EXPLORER_ROW_CONTRACT_REF.to_owned(),
        M5_PACKAGE_CERTIFICATION_MANIFEST_SCOPE_REGISTRY_CONTRACT_REF.to_owned(),
        M5_PACKAGE_CERTIFICATION_INSTALL_REVIEW_LOCKFILE_CONTRACT_REF.to_owned(),
        M5_PACKAGE_CERTIFICATION_SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_CONTRACT_REF.to_owned(),
    ]
}

fn validate_source_contracts(
    packet: &PackageComponentCertificationPacket,
    violations: &mut Vec<PackageComponentCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PACKAGE_CERTIFICATION_SCHEMA_REF,
        M5_PACKAGE_CERTIFICATION_DOC_REF,
        M5_PACKAGE_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF,
        M5_PACKAGE_CERTIFICATION_CONSUMER_CONTRACT_REF,
        M5_PACKAGE_CERTIFICATION_ACCESSIBILITY_CONTRACT_REF,
        M5_PACKAGE_CERTIFICATION_EXPLORER_ROW_CONTRACT_REF,
        M5_PACKAGE_CERTIFICATION_MANIFEST_SCOPE_REGISTRY_CONTRACT_REF,
        M5_PACKAGE_CERTIFICATION_INSTALL_REVIEW_LOCKFILE_CONTRACT_REF,
        M5_PACKAGE_CERTIFICATION_SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(PackageComponentCertificationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(
    packet: &PackageComponentCertificationPacket,
    violations: &mut Vec<PackageComponentCertificationViolation>,
) {
    if packet.surface_rows.is_empty() {
        violations.push(PackageComponentCertificationViolation::SurfaceRowsMissing);
        return;
    }

    let mut seen_surfaces: BTreeSet<M5PackageComponentCertifiedSurface> = BTreeSet::new();
    let mut seen_components: BTreeSet<M5PackageComponent> = BTreeSet::new();

    for row in &packet.surface_rows {
        if row.row_id.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(PackageComponentCertificationViolation::RowIncomplete);
        }
        if row.components_present.is_empty() {
            violations.push(PackageComponentCertificationViolation::ComponentsMissingOnRow);
        }

        if row.keyboard_label.trim().is_empty() {
            violations.push(PackageComponentCertificationViolation::KeyboardLabelMissing);
        }
        if row.screen_reader_label.trim().is_empty() {
            violations.push(PackageComponentCertificationViolation::ScreenReaderLabelMissing);
        }
        if row.cli_enum_token.trim().is_empty() {
            violations.push(PackageComponentCertificationViolation::CliEnumTokenMissing);
        }
        if row.export_enum_token.trim().is_empty() {
            violations.push(PackageComponentCertificationViolation::ExportEnumTokenMissing);
        }
        if row.explanation_field.trim().is_empty() {
            violations.push(PackageComponentCertificationViolation::ExplanationFieldMissing);
        }

        if !row.covers_all_axes() {
            violations.push(PackageComponentCertificationViolation::AxisCoverageMissing);
        }
        if row
            .axis_outcomes
            .iter()
            .any(|outcome| outcome.note.trim().is_empty())
        {
            violations.push(PackageComponentCertificationViolation::AxisNoteMissing);
        }

        // AC2 core: a certified claim may never exceed the claim it certifies.
        if !row.certified_claim_within_claimed() {
            violations.push(PackageComponentCertificationViolation::CertifiedClaimExceedsClaimed);
        }

        if !row.narrowed_axes_consistent() {
            violations.push(PackageComponentCertificationViolation::NarrowedAxesInconsistent);
        }

        // A narrowed surface must disclose its downgrade trigger.
        if !row.narrowed_axes.is_empty() && row.downgrade_trigger.is_none() {
            violations.push(PackageComponentCertificationViolation::NarrowingWithoutTrigger);
        }

        // Delta: certification may narrow a claim but never drop component truth.
        if !row.component_truth_preserved {
            violations.push(PackageComponentCertificationViolation::PackageComponentTruthDropped);
        }

        // The recorded status must agree with the derived one.
        if !row.status_is_consistent() {
            violations.push(PackageComponentCertificationViolation::StatusMismatch);
        }

        if !row.points_at_canonical_contracts() {
            violations
                .push(PackageComponentCertificationViolation::CanonicalContractReferenceMissing);
        }

        seen_surfaces.insert(row.surface);
        for component in &row.components_present {
            seen_components.insert(*component);
        }
    }

    for surface in M5PackageComponentCertifiedSurface::ALL {
        if !seen_surfaces.contains(&surface) {
            violations.push(PackageComponentCertificationViolation::SurfaceCoverageMissing);
            break;
        }
    }
    for component in M5PackageComponent::ALL {
        if !seen_components.contains(&component) {
            violations.push(PackageComponentCertificationViolation::ComponentCoverageMissing);
            break;
        }
    }
}

fn validate_summary(
    packet: &PackageComponentCertificationPacket,
    violations: &mut Vec<PackageComponentCertificationViolation>,
) {
    let recomputed = PackageComponentCertificationSummary::from_rows(&packet.surface_rows);
    if recomputed != packet.summary {
        violations.push(PackageComponentCertificationViolation::SummaryMismatch);
    }
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
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
