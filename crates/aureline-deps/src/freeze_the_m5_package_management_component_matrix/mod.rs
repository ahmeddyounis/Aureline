//! Frozen M5 package-management component matrix.
//!
//! This module locks the canonical M5 component truth for eight reusable
//! package-management surfaces — package explorer rows, manifest-scope
//! switchers, install-review sheets, registry/mirror rows, script-risk notices,
//! lockfile-impact cards, grouped-update planners, and rollback/checkpoint
//! strips — into one export-safe packet. Each
//! [`M5PackageComponentMatrixRow`] binds a component to its maturity class, the
//! exact manifest-scope disclosure it must preserve, its registry-source and
//! auth-posture disclosure, its script/native-build risk disclosure, its
//! lockfile-churn disclosure, its rollback/checkpoint disclosure, its
//! mirror/offline degradation-narrowing vocabulary, required evidence packet
//! refs, downgrade triggers, rollback posture, source contracts, and
//! consumer-surface parity.
//!
//! The matrix is the single source of truth for whether every claimed M5
//! package browse, review, mutate, export, or support surface may consume one
//! shared component family instead of private dialog text or generic
//! manage-package chrome. It references upstream dependency-row, manifest-scope,
//! install-review, mirror/offline, post-install-disclosure, lockfile-impact,
//! grouped-update, and rollback-primitive contracts by id rather than embedding
//! their content. Raw manifest bodies, raw lockfile bodies, registry
//! credentials, private registry URLs, and live registry responses stay outside
//! the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-package-management-component-matrix.schema.json`](../../../../schemas/ui/m5-package-management-component-matrix.schema.json).
//! The contract doc is
//! [`docs/deps/m5/freeze_the_m5_package_management_component_matrix.md`](../../../../docs/deps/m5/freeze_the_m5_package_management_component_matrix.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-package-management-components/`](../../../../fixtures/ui/m5-package-management-components/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5PackageComponentMatrixPacket`].
pub const M5_PACKAGE_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_package_management_component_matrix";

/// Schema version for M5 package-management component-matrix records.
pub const M5_PACKAGE_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_PACKAGE_COMPONENT_MATRIX_SCHEMA_REF: &str =
    "schemas/ui/m5-package-management-component-matrix.schema.json";

/// Repo-relative path of the M5 package-management component-matrix doc.
pub const M5_PACKAGE_COMPONENT_MATRIX_DOC_REF: &str =
    "docs/deps/m5/freeze_the_m5_package_management_component_matrix.md";

/// Repo-relative path of the frozen package-explorer-row (dependency row) contract.
pub const M5_PACKAGE_COMPONENT_MATRIX_EXPLORER_ROW_CONTRACT_REF: &str =
    "schemas/ui/m5-dependency-row.schema.json";

/// Repo-relative path of the frozen manifest-scope-switcher contract.
pub const M5_PACKAGE_COMPONENT_MATRIX_MANIFEST_SCOPE_CONTRACT_REF: &str =
    "schemas/runtime/manifest_scope_alpha.schema.json";

/// Repo-relative path of the frozen install-review-sheet contract.
pub const M5_PACKAGE_COMPONENT_MATRIX_INSTALL_REVIEW_CONTRACT_REF: &str =
    "schemas/ecosystem/m5-install-review.schema.json";

/// Repo-relative path of the frozen registry-or-mirror-row contract.
pub const M5_PACKAGE_COMPONENT_MATRIX_REGISTRY_MIRROR_CONTRACT_REF: &str =
    "schemas/ui/m5-mirror-offline-artifact-row.schema.json";

/// Repo-relative path of the frozen script-risk-notice contract.
pub const M5_PACKAGE_COMPONENT_MATRIX_SCRIPT_RISK_CONTRACT_REF: &str =
    "schemas/governance/post_install_disclosure.schema.json";

/// Repo-relative path of the frozen lockfile-impact-card contract.
pub const M5_PACKAGE_COMPONENT_MATRIX_LOCKFILE_IMPACT_CONTRACT_REF: &str =
    "schemas/runtime/lockfile_impact_alpha.schema.json";

/// Repo-relative path of the frozen grouped-update-planner contract.
pub const M5_PACKAGE_COMPONENT_MATRIX_GROUPED_UPDATE_CONTRACT_REF: &str =
    "schemas/deps/grouped-update-and-rollback-review.schema.json";

/// Repo-relative path of the frozen rollback-checkpoint-strip contract.
pub const M5_PACKAGE_COMPONENT_MATRIX_ROLLBACK_STRIP_CONTRACT_REF: &str =
    "schemas/ui/m5-bundle-rollback-remove-primitive.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_PACKAGE_COMPONENT_MATRIX_FIXTURE_DIR: &str =
    "fixtures/ui/m5-package-management-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PACKAGE_COMPONENT_MATRIX_ARTIFACT_REF: &str =
    "artifacts/release/m5-package-management-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_PACKAGE_COMPONENT_MATRIX_SUMMARY_REF: &str =
    "artifacts/release/m5-package-management-proof/summary.md";

/// One of the eight M5 reusable package-management components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PackageComponent {
    /// Package explorer row naming a package, its manifest, and its relation.
    PackageExplorerRow,
    /// Manifest-scope switcher selecting the target manifest for a mutation.
    ManifestScopeSwitcher,
    /// Install-review sheet previewing scope, script risk, and lockfile churn.
    InstallReviewSheet,
    /// Registry or mirror row naming the source and auth posture.
    RegistryOrMirrorRow,
    /// Script-risk notice naming script/native-build risk before install.
    ScriptRiskNotice,
    /// Lockfile-impact card quantifying lockfile churn without understating it.
    LockfileImpactCard,
    /// Grouped-update planner naming the grouped-update reason and constraints.
    GroupedUpdatePlanner,
    /// Rollback/checkpoint strip naming the checkpoint identity and recovery.
    RollbackCheckpointStrip,
}

impl M5PackageComponent {
    /// Every component, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::PackageExplorerRow,
        Self::ManifestScopeSwitcher,
        Self::InstallReviewSheet,
        Self::RegistryOrMirrorRow,
        Self::ScriptRiskNotice,
        Self::LockfileImpactCard,
        Self::GroupedUpdatePlanner,
        Self::RollbackCheckpointStrip,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackageExplorerRow => "package_explorer_row",
            Self::ManifestScopeSwitcher => "manifest_scope_switcher",
            Self::InstallReviewSheet => "install_review_sheet",
            Self::RegistryOrMirrorRow => "registry_or_mirror_row",
            Self::ScriptRiskNotice => "script_risk_notice",
            Self::LockfileImpactCard => "lockfile_impact_card",
            Self::GroupedUpdatePlanner => "grouped_update_planner",
            Self::RollbackCheckpointStrip => "rollback_checkpoint_strip",
        }
    }

    /// Canonical upstream source contract this component consumes.
    ///
    /// Every row must list this ref among its `source_contract_refs` so a
    /// component can never be re-homed onto generic manage-package chrome that
    /// hides its canonical source of truth.
    pub const fn canonical_source_contract_ref(self) -> &'static str {
        match self {
            Self::PackageExplorerRow => M5_PACKAGE_COMPONENT_MATRIX_EXPLORER_ROW_CONTRACT_REF,
            Self::ManifestScopeSwitcher => M5_PACKAGE_COMPONENT_MATRIX_MANIFEST_SCOPE_CONTRACT_REF,
            Self::InstallReviewSheet => M5_PACKAGE_COMPONENT_MATRIX_INSTALL_REVIEW_CONTRACT_REF,
            Self::RegistryOrMirrorRow => M5_PACKAGE_COMPONENT_MATRIX_REGISTRY_MIRROR_CONTRACT_REF,
            Self::ScriptRiskNotice => M5_PACKAGE_COMPONENT_MATRIX_SCRIPT_RISK_CONTRACT_REF,
            Self::LockfileImpactCard => M5_PACKAGE_COMPONENT_MATRIX_LOCKFILE_IMPACT_CONTRACT_REF,
            Self::GroupedUpdatePlanner => M5_PACKAGE_COMPONENT_MATRIX_GROUPED_UPDATE_CONTRACT_REF,
            Self::RollbackCheckpointStrip => {
                M5_PACKAGE_COMPONENT_MATRIX_ROLLBACK_STRIP_CONTRACT_REF
            }
        }
    }
}

/// Maturity class for an M5 package-management component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PackageComponentMaturityClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5PackageComponentMaturityClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Evidence requirement level for a component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PackageComponentEvidenceRequirement {
    /// At least one evidence packet is required.
    Required,
    /// Evidence is recommended but not blocking.
    Recommended,
    /// Evidence is optional.
    Optional,
    /// Not applicable for this component's current maturity.
    NotApplicable,
}

impl M5PackageComponentEvidenceRequirement {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Recommended => "recommended",
            Self::Optional => "optional",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Registry/resolution degradation-narrowing vocabulary every component preserves.
///
/// This vocabulary names the package-resolution posture explicitly so a
/// mirror-backed, offline-snapshot, auth-required, or stale state is never
/// flattened into a generic "installed" or "not found" message, and a range-only
/// or unknown resolution is always labeled as such rather than presented as an
/// exact pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PackageComponentDegradationState {
    /// The resolution is an exact, authoritative pin from a reachable source.
    ResolvedExact,
    /// Only a manifest range governs; the exact resolution is not pinned here.
    ManifestRangeOnly,
    /// The answer came from an enterprise mirror rather than the upstream registry.
    MirrorBacked,
    /// Only an offline snapshot or local cache is available.
    OfflineSnapshotOnly,
    /// Registry access requires authentication that is not satisfied.
    AuthRequiredUnsatisfied,
    /// The package state could not be established or is stale.
    UnknownOrStale,
}

impl M5PackageComponentDegradationState {
    /// Every degradation state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ResolvedExact,
        Self::ManifestRangeOnly,
        Self::MirrorBacked,
        Self::OfflineSnapshotOnly,
        Self::AuthRequiredUnsatisfied,
        Self::UnknownOrStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolvedExact => "resolved_exact",
            Self::ManifestRangeOnly => "manifest_range_only",
            Self::MirrorBacked => "mirror_backed",
            Self::OfflineSnapshotOnly => "offline_snapshot_only",
            Self::AuthRequiredUnsatisfied => "auth_required_unsatisfied",
            Self::UnknownOrStale => "unknown_or_stale",
        }
    }
}

/// Downgrade trigger that can narrow a component below its claimed maturity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PackageComponentDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// The registry could not be reached.
    RegistryUnreachable,
    /// Registry access requires authentication that is not satisfied.
    AuthRequired,
    /// Only a mirror-backed answer is available.
    MirrorBackedOnly,
    /// Only an offline snapshot is available.
    OfflineSnapshotOnly,
    /// The lockfile and manifest disagree and mutation is blocked.
    LockfileDivergent,
    /// The mutation may run install scripts or a native build.
    ScriptOrNativeBuildRisk,
    /// The mutation would trigger broad lockfile regeneration.
    BroadLockfileRegeneration,
    /// No durable rollback checkpoint is available.
    RollbackUnavailable,
    /// Scope expanded beyond the qualified package-management boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency component narrowed.
    UpstreamDependencyNarrowed,
}

impl M5PackageComponentDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::RegistryUnreachable,
        Self::AuthRequired,
        Self::MirrorBackedOnly,
        Self::OfflineSnapshotOnly,
        Self::LockfileDivergent,
        Self::ScriptOrNativeBuildRisk,
        Self::BroadLockfileRegeneration,
        Self::RollbackUnavailable,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::RegistryUnreachable => "registry_unreachable",
            Self::AuthRequired => "auth_required",
            Self::MirrorBackedOnly => "mirror_backed_only",
            Self::OfflineSnapshotOnly => "offline_snapshot_only",
            Self::LockfileDivergent => "lockfile_divergent",
            Self::ScriptOrNativeBuildRisk => "script_or_native_build_risk",
            Self::BroadLockfileRegeneration => "broad_lockfile_regeneration",
            Self::RollbackUnavailable => "rollback_unavailable",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Rollback / write-back posture for a component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PackageComponentRollbackPosture {
    /// Read-only component that never mutates the manifest, lockfile, or repo.
    ReadOnlyNoMutation,
    /// Staged-review component that never writes back until an explicit apply.
    StagedReviewNoWrite,
    /// Write-back stays individually attributable behind a durable checkpoint.
    WriteBackCheckpointed,
    /// Generated lockfiles regenerate from source rather than accept manual edits.
    RegenerateOnlyNoManualEdit,
    /// Only a compensating cleanup exists; there is no clean automatic revert.
    CompensatingOnlyNoCleanRevert,
    /// Not applicable for the component's current maturity.
    NotApplicable,
}

impl M5PackageComponentRollbackPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyNoMutation => "read_only_no_mutation",
            Self::StagedReviewNoWrite => "staged_review_no_write",
            Self::WriteBackCheckpointed => "write_back_checkpointed",
            Self::RegenerateOnlyNoManualEdit => "regenerate_only_no_manual_edit",
            Self::CompensatingOnlyNoCleanRevert => "compensating_only_no_clean_revert",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Consumer surface that must project this component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PackageComponentConsumerSurface {
    /// Desktop package workspace.
    PackageWorkspace,
    /// Dependency explorer / browse surface.
    DependencyExplorer,
    /// Install / update review surface.
    InstallUpdateReview,
    /// Registry / auth workspace.
    RegistryAuthWorkspace,
    /// Rollback / recovery surface.
    RollbackRecovery,
    /// Browser companion / handoff follow-up.
    BrowserCompanion,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Help / About surface.
    HelpAbout,
}

impl M5PackageComponentConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::PackageWorkspace,
        Self::DependencyExplorer,
        Self::InstallUpdateReview,
        Self::RegistryAuthWorkspace,
        Self::RollbackRecovery,
        Self::BrowserCompanion,
        Self::CliHeadless,
        Self::SupportExport,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackageWorkspace => "package_workspace",
            Self::DependencyExplorer => "dependency_explorer",
            Self::InstallUpdateReview => "install_update_review",
            Self::RegistryAuthWorkspace => "registry_auth_workspace",
            Self::RollbackRecovery => "rollback_recovery",
            Self::BrowserCompanion => "browser_companion",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::HelpAbout => "help_about",
        }
    }
}

/// One row in the M5 package-management component matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PackageComponentMatrixRow {
    /// Package-management component.
    pub component: M5PackageComponent,
    /// Maturity class earned by this component.
    pub maturity: M5PackageComponentMaturityClass,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Exact target-manifest and direct/transitive scope disclosure this keeps.
    pub manifest_scope_disclosure: String,
    /// Registry-source (public/private/mirror/offline) disclosure this keeps.
    pub registry_source_disclosure: String,
    /// Auth-posture disclosure this component keeps explicit.
    pub auth_posture_disclosure: String,
    /// Script / native-build risk disclosure this component keeps explicit.
    pub script_native_build_disclosure: String,
    /// Lockfile-churn / blast-radius disclosure this component keeps explicit.
    pub lockfile_churn_disclosure: String,
    /// Rollback / checkpoint identity disclosure this component keeps explicit.
    pub rollback_checkpoint_disclosure: String,
    /// Registry/resolution degradation-narrowing vocabulary this must preserve.
    pub degradation_narrowing_vocab: Vec<M5PackageComponentDegradationState>,
    /// Evidence requirement level.
    pub evidence_requirement: M5PackageComponentEvidenceRequirement,
    /// Required evidence packet refs for this maturity.
    pub required_evidence_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5PackageComponentDowngradeTrigger>,
    /// Rollback / write-back posture.
    pub rollback_posture: M5PackageComponentRollbackPosture,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this component.
    pub consumer_surfaces: Vec<M5PackageComponentConsumerSurface>,
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PackageComponentMatrixTrustReview {
    /// Target manifest and scope are always explicit.
    pub manifest_scope_always_explicit: bool,
    /// Direct/transitive and workspace-local state stays explicit.
    pub direct_transitive_state_explicit: bool,
    /// Registry source (public/private/mirror/offline) is always explicit.
    pub registry_source_always_explicit: bool,
    /// Auth posture is never hidden behind a generic prompt.
    pub auth_posture_never_hidden: bool,
    /// Script / native-build risk stays explicit before any install.
    pub script_native_build_risk_explicit: bool,
    /// Lockfile churn is never understated.
    pub lockfile_churn_never_understated: bool,
    /// Grouped-update reason and constraints stay explicit.
    pub grouped_update_reason_explicit: bool,
    /// Rollback / checkpoint identity stays explicit.
    pub rollback_checkpoint_identity_explicit: bool,
    /// Mirror / offline continuity stays explicit rather than reading as clean.
    pub mirror_offline_continuity_explicit: bool,
    /// Generic one-click language never conceals scope, auth, or risk.
    pub one_click_never_conceals_scope_or_risk: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PackageComponentMatrixConsumerProjection {
    /// Package explorer row shows the manifest and direct/transitive relation.
    pub package_explorer_row_shows_manifest_and_relation: bool,
    /// Manifest-scope switcher shows the target and required confirmation.
    pub manifest_scope_switcher_shows_target_and_confirmation: bool,
    /// Install-review sheet shows scope, script risk, and lockfile churn.
    pub install_review_sheet_shows_scope_script_and_lockfile: bool,
    /// Registry / mirror row shows the source and auth posture.
    pub registry_or_mirror_row_shows_source_and_auth: bool,
    /// Script-risk notice shows script / native-build risk.
    pub script_risk_notice_shows_script_native_build_risk: bool,
    /// Lockfile-impact card shows churn without understating it.
    pub lockfile_impact_card_shows_churn_without_understating: bool,
    /// Grouped-update planner shows the reason and constraints.
    pub grouped_update_planner_shows_reason_and_constraints: bool,
    /// Rollback / checkpoint strip shows the checkpoint identity.
    pub rollback_checkpoint_strip_shows_checkpoint_identity: bool,
    /// CLI / headless shows component truth.
    pub cli_headless_shows_component_truth: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PackageComponentMatrixProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5PackageComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PackageComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5PackageComponentMatrixRow>,
    /// Trust review block.
    pub trust_review: M5PackageComponentMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5PackageComponentMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PackageComponentMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 package-management component-matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PackageComponentMatrixPacket {
    /// Record kind; must equal [`M5_PACKAGE_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PACKAGE_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5PackageComponentMatrixRow>,
    /// Trust review block.
    pub trust_review: M5PackageComponentMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5PackageComponentMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PackageComponentMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5PackageComponentMatrixPacket {
    /// Builds an M5 package-management component-matrix packet.
    pub fn new(input: M5PackageComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_PACKAGE_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_PACKAGE_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 package-management component-matrix invariants.
    pub fn validate(&self) -> Vec<M5PackageComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PACKAGE_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5PackageComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PACKAGE_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5PackageComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5PackageComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 package-component matrix packet serializes"),
        ) {
            violations.push(M5PackageComponentMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 package-component matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.maturity.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Package-Management Component Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Components: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Components\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component.as_str(),
                row.maturity.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Manifest scope: {}\n",
                row.manifest_scope_disclosure
            ));
            out.push_str(&format!(
                "  - Registry source: {}\n",
                row.registry_source_disclosure
            ));
            out.push_str(&format!(
                "  - Auth posture: {}\n",
                row.auth_posture_disclosure
            ));
            out.push_str(&format!(
                "  - Script/native-build risk: {}\n",
                row.script_native_build_disclosure
            ));
            out.push_str(&format!(
                "  - Lockfile churn: {}\n",
                row.lockfile_churn_disclosure
            ));
            out.push_str(&format!(
                "  - Rollback/checkpoint: {}\n",
                row.rollback_checkpoint_disclosure
            ));
            out.push_str(&format!(
                "  - Rollback posture: {}\n",
                row.rollback_posture.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 package-component matrix export.
#[derive(Debug)]
pub enum M5PackageComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5PackageComponentMatrixViolation>),
}

impl fmt::Display for M5PackageComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 package-component matrix export parse failed: {error}"
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
                    "m5 package-component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5PackageComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5PackageComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5PackageComponentMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required component is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component's row does not list its canonical source contract ref.
    ComponentSourceContractMismatch,
    /// A component claiming Stable is missing required evidence packet refs.
    StableComponentMissingEvidence,
    /// A component has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component does not name its target-manifest / scope disclosure.
    ManifestScopeDisclosureMissing,
    /// A component does not name its registry-source disclosure.
    RegistrySourceDisclosureMissing,
    /// A component does not name its auth-posture disclosure.
    AuthPostureDisclosureMissing,
    /// A component does not name its script / native-build risk disclosure.
    ScriptNativeBuildDisclosureMissing,
    /// A component does not name its lockfile-churn disclosure.
    LockfileChurnDisclosureMissing,
    /// A component does not name its rollback / checkpoint disclosure.
    RollbackCheckpointDisclosureMissing,
    /// A component does not carry a degradation-narrowing vocabulary.
    DegradationNarrowingVocabMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5PackageComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::ComponentSourceContractMismatch => "component_source_contract_mismatch",
            Self::StableComponentMissingEvidence => "stable_component_missing_evidence",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ManifestScopeDisclosureMissing => "manifest_scope_disclosure_missing",
            Self::RegistrySourceDisclosureMissing => "registry_source_disclosure_missing",
            Self::AuthPostureDisclosureMissing => "auth_posture_disclosure_missing",
            Self::ScriptNativeBuildDisclosureMissing => "script_native_build_disclosure_missing",
            Self::LockfileChurnDisclosureMissing => "lockfile_churn_disclosure_missing",
            Self::RollbackCheckpointDisclosureMissing => "rollback_checkpoint_disclosure_missing",
            Self::DegradationNarrowingVocabMissing => "degradation_narrowing_vocab_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 package-component matrix export.
pub fn current_stable_m5_package_component_matrix_export(
) -> Result<M5PackageComponentMatrixPacket, M5PackageComponentMatrixArtifactError> {
    let packet: M5PackageComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-package-management-proof/support_export.json"
    )))
    .map_err(M5PackageComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5PackageComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5PackageComponentMatrixPacket,
    violations: &mut Vec<M5PackageComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PACKAGE_COMPONENT_MATRIX_SCHEMA_REF,
        M5_PACKAGE_COMPONENT_MATRIX_DOC_REF,
        M5_PACKAGE_COMPONENT_MATRIX_EXPLORER_ROW_CONTRACT_REF,
        M5_PACKAGE_COMPONENT_MATRIX_MANIFEST_SCOPE_CONTRACT_REF,
        M5_PACKAGE_COMPONENT_MATRIX_INSTALL_REVIEW_CONTRACT_REF,
        M5_PACKAGE_COMPONENT_MATRIX_REGISTRY_MIRROR_CONTRACT_REF,
        M5_PACKAGE_COMPONENT_MATRIX_SCRIPT_RISK_CONTRACT_REF,
        M5_PACKAGE_COMPONENT_MATRIX_LOCKFILE_IMPACT_CONTRACT_REF,
        M5_PACKAGE_COMPONENT_MATRIX_GROUPED_UPDATE_CONTRACT_REF,
        M5_PACKAGE_COMPONENT_MATRIX_ROLLBACK_STRIP_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5PackageComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_component_rows(
    packet: &M5PackageComponentMatrixPacket,
    violations: &mut Vec<M5PackageComponentMatrixViolation>,
) {
    let present: BTreeSet<M5PackageComponent> = packet
        .component_rows
        .iter()
        .map(|row| row.component)
        .collect();
    for required in M5PackageComponent::ALL {
        if !present.contains(&required) {
            violations.push(M5PackageComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        if row.scope_summary.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(M5PackageComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|contract| contract == row.component.canonical_source_contract_ref())
        {
            violations.push(M5PackageComponentMatrixViolation::ComponentSourceContractMismatch);
        }
        if row.maturity.is_stable() && row.required_evidence_packet_refs.is_empty() {
            violations.push(M5PackageComponentMatrixViolation::StableComponentMissingEvidence);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5PackageComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5PackageComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.manifest_scope_disclosure.trim().is_empty() {
            violations.push(M5PackageComponentMatrixViolation::ManifestScopeDisclosureMissing);
        }
        if row.registry_source_disclosure.trim().is_empty() {
            violations.push(M5PackageComponentMatrixViolation::RegistrySourceDisclosureMissing);
        }
        if row.auth_posture_disclosure.trim().is_empty() {
            violations.push(M5PackageComponentMatrixViolation::AuthPostureDisclosureMissing);
        }
        if row.script_native_build_disclosure.trim().is_empty() {
            violations.push(M5PackageComponentMatrixViolation::ScriptNativeBuildDisclosureMissing);
        }
        if row.lockfile_churn_disclosure.trim().is_empty() {
            violations.push(M5PackageComponentMatrixViolation::LockfileChurnDisclosureMissing);
        }
        if row.rollback_checkpoint_disclosure.trim().is_empty() {
            violations.push(M5PackageComponentMatrixViolation::RollbackCheckpointDisclosureMissing);
        }
        if row.degradation_narrowing_vocab.is_empty() {
            violations.push(M5PackageComponentMatrixViolation::DegradationNarrowingVocabMissing);
        }
    }
}

fn validate_trust_review(
    packet: &M5PackageComponentMatrixPacket,
    violations: &mut Vec<M5PackageComponentMatrixViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.manifest_scope_always_explicit,
        review.direct_transitive_state_explicit,
        review.registry_source_always_explicit,
        review.auth_posture_never_hidden,
        review.script_native_build_risk_explicit,
        review.lockfile_churn_never_understated,
        review.grouped_update_reason_explicit,
        review.rollback_checkpoint_identity_explicit,
        review.mirror_offline_continuity_explicit,
        review.one_click_never_conceals_scope_or_risk,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(M5PackageComponentMatrixViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5PackageComponentMatrixPacket,
    violations: &mut Vec<M5PackageComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.package_explorer_row_shows_manifest_and_relation,
        projection.manifest_scope_switcher_shows_target_and_confirmation,
        projection.install_review_sheet_shows_scope_script_and_lockfile,
        projection.registry_or_mirror_row_shows_source_and_auth,
        projection.script_risk_notice_shows_script_native_build_risk,
        projection.lockfile_impact_card_shows_churn_without_understating,
        projection.grouped_update_planner_shows_reason_and_constraints,
        projection.rollback_checkpoint_strip_shows_checkpoint_identity,
        projection.cli_headless_shows_component_truth,
        projection.support_export_shows_component_truth,
    ] {
        if !ok {
            violations.push(M5PackageComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5PackageComponentMatrixPacket,
    violations: &mut Vec<M5PackageComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5PackageComponentMatrixViolation::ProofFreshnessIncomplete);
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
