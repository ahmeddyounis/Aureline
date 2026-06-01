//! Hardened installation-topology audit: state-root audits, silent-deployment
//! posture, and fleet-rollout evidence for managed install lanes.
//!
//! This module closes the gap between the alpha install-topology rows and the
//! admin-facing accountability surface for managed, self-hosted, and
//! air-gapped deployments. It produces a single inspectable
//! [`HardenInstallTopologyPage`] that every diagnostics, Help/About,
//! support-export, and CLI surface can consume without reading localized prose
//! or external deployment notes.
//!
//! ## What the page asserts
//!
//! For each managed-fleet install row:
//!
//! - **Tenant identity** — an opaque `tenant_ref` anchors the row to an
//!   organization or tenant without exposing raw tenant credentials.
//! - **Rollout ring** — the [`RolloutRingClass`] (`canary`, `pilot`, `broad`,
//!   `lts`) is explicit on every managed row.
//! - **Updater owner** — the [`UpdaterOwnerClass`] (`managed_fleet`, etc.) is
//!   named on every row so the admin surface never has to infer ownership from
//!   install mode alone.
//! - **Binary root class** — the [`BinaryRootClass`] is captured per row,
//!   allowing support to explain whether the binary lives under a per-machine,
//!   package-manager, or portable area.
//! - **State-root audit** — every durable state root is itemized with its
//!   [`StateRootIsolationClass`], [`StateRootReviewClass`], and an explicit
//!   flag stating whether the root is visible in the admin/support view.
//! - **Policy source** — an opaque `policy_source_ref` names the GPO, MDM, or
//!   config-profile source without copying raw policy bodies.
//! - **Fleet evidence** — all required [`FleetRolloutEvidenceClass`] tokens
//!   (`ring_assignment`, `exact_build_inventory`, `managed_package_report`,
//!   `policy_root`, `rollback_target`, `verification_status`, `support_export`)
//!   must be present so inventory probes can identify the exact build and ring
//!   without host-log scraping.
//! - **Silent deployment posture** — every managed row must declare its
//!   [`SilentInstallSupportClass`], list its non-empty disclosed limits, and
//!   name its return-code families.
//!
//! ## Qualification narrowing
//!
//! | Condition | Narrowing |
//! |---|---|
//! | `admin_view_complete: false` on any managed row | `Withdrawn` |
//! | No managed-fleet rows present | `Preview` |
//! | Silent deployment limits not declared | `Beta` |
//! | Return-code families not named | `Beta` |
//! | All conditions met | `Stable` |
//!
//! ## Boundaries
//!
//! This module does not implement an installer, fleet-control service, or
//! rollout engine. All fields are typed tokens, opaque refs, counts, or
//! plain-language labels — no raw paths, credentials, policy bodies, or
//! secret material cross this boundary.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::topology::{
    BinaryRootClass, FleetRolloutEvidenceClass, RolloutRingClass, SilentInstallSupportClass,
    StateRootIsolationClass, StateRootReviewClass, UpdaterOwnerClass,
};

/// Schema version for harden-install-topology records.
pub const HARDEN_INSTALL_TOPOLOGY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`HardenInstallTopologyPage`].
pub const HARDEN_INSTALL_TOPOLOGY_PAGE_RECORD_KIND: &str =
    "harden_install_topology_page_record";

/// Stable record-kind tag for [`HardenInstallTopologySupportExport`].
pub const HARDEN_INSTALL_TOPOLOGY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "harden_install_topology_support_export_record";

/// Shared contract ref consumed by every harden-install-topology record.
pub const HARDEN_INSTALL_TOPOLOGY_SHARED_CONTRACT_REF: &str =
    "install:harden_installation_topology:v1";

/// Fleet evidence classes required for a fully inspectable managed row.
///
/// Every managed-fleet row must carry all seven evidence classes for a
/// `Stable` qualification claim.
pub const REQUIRED_FLEET_EVIDENCE: &[FleetRolloutEvidenceClass] = &[
    FleetRolloutEvidenceClass::RingAssignment,
    FleetRolloutEvidenceClass::ExactBuildInventory,
    FleetRolloutEvidenceClass::ManagedPackageReport,
    FleetRolloutEvidenceClass::PolicyRoot,
    FleetRolloutEvidenceClass::RollbackTarget,
    FleetRolloutEvidenceClass::VerificationStatus,
    FleetRolloutEvidenceClass::SupportExport,
];

/// Qualification token for a harden-install-topology row or page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationToken {
    /// All conditions met; the row or page is stable.
    Stable,
    /// Non-critical conditions unmet; narrowed to beta.
    Beta,
    /// Structural coverage gap; narrowed to preview.
    Preview,
    /// Critical invariant violated; the page is withdrawn.
    Withdrawn,
}

/// Narrow reason token for a defect or row qualification narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowReasonToken {
    /// No narrowing applied.
    NotNarrowed,
    /// Rollout ring is not named on the managed row.
    RingNotNamed,
    /// Tenant identity reference is absent on the managed row.
    TenantNotNamed,
    /// Updater owner is not named on the managed row.
    UpdaterOwnerNotNamed,
    /// Binary root class is not named on the managed row.
    BinaryRootNotNamed,
    /// State-root audit entries are absent on the managed row.
    StateRootsNotAudited,
    /// Policy source reference is absent on the managed row.
    PolicySourceNotNamed,
    /// One or more required fleet-evidence classes are missing.
    FleetEvidenceIncomplete,
    /// The managed row cannot be fully inspected from the admin/support view.
    AdminViewIncomplete,
    /// Silent deployment limits are not declared on a managed row.
    SilentLimitsNotDeclared,
    /// Return-code families are not named on a managed row.
    ReturnCodesNotNamed,
    /// No managed-fleet rows are present in the page.
    NoManagedRows,
}

/// One audited durable state-root entry inside a managed-fleet row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateRootAuditEntry {
    /// Opaque state-root ref from the install topology or state-root map.
    pub state_root_ref: String,
    /// Isolation posture for the root.
    pub isolation_class: StateRootIsolationClass,
    /// Review posture for using or importing the root.
    pub review_class: StateRootReviewClass,
    /// True when this root is visible in the admin or support view.
    pub exposed_in_admin_view: bool,
    /// True when the root may contain secret material.
    ///
    /// When true the root appears in the admin view but its contents are
    /// never exported beyond the metadata token.
    pub contains_secret_material: bool,
}

impl StateRootAuditEntry {
    /// Returns true when the entry is auditable from the admin view.
    pub fn is_admin_visible(&self) -> bool {
        self.exposed_in_admin_view
    }
}

/// One managed-fleet install-topology audit row.
///
/// Captures the tenant identity, ring, updater owner, binary root, state-root
/// audit, policy source, fleet evidence, silent-deployment posture, and
/// whether the admin view is complete — all as typed tokens or opaque refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedFleetAuditRow {
    /// Stable row id for this audit entry.
    pub row_id: String,
    /// Install-topology row id this audit describes.
    pub topology_row_id: String,
    /// Platform token (`windows`, `macos`, `linux`, `air_gap_bundle_target`).
    pub platform_token: String,
    /// Opaque tenant or organization identity ref.
    ///
    /// Never contains raw tenant credentials, domain names, or email
    /// addresses — only an opaque identifier joinable from fleet inventory.
    pub tenant_ref: String,
    /// Rollout ring assigned to the install row.
    pub rollout_ring_class: RolloutRingClass,
    /// Actor class that owns updates for this row.
    pub updater_owner_class: UpdaterOwnerClass,
    /// Binary root placement class.
    pub binary_root_class: BinaryRootClass,
    /// Opaque policy source ref (GPO path token, MDM config profile id, etc.).
    ///
    /// Never contains raw policy bodies, ADMX content, or rule expressions.
    pub policy_source_ref: String,
    /// Audited durable state-root entries for this row.
    ///
    /// Must be non-empty for a `Stable` qualification claim.
    pub state_root_audit: Vec<StateRootAuditEntry>,
    /// Fleet-rollout evidence classes present on the row.
    pub fleet_evidence: Vec<FleetRolloutEvidenceClass>,
    /// Silent install support class.
    pub silent_deployment_class: SilentInstallSupportClass,
    /// True when at least one disclosed limit is named for silent deployment.
    pub silent_deployment_limits_declared: bool,
    /// True when the admin or support view can fully identify this install
    /// without reading localized prose or external deployment notes.
    pub admin_view_complete: bool,
    /// Row qualification token.
    pub qualification_token: QualificationToken,
    /// Narrow reason when the row is not `Stable`.
    pub narrow_reason_token: NarrowReasonToken,
    /// Export-safe plain-language summary for the admin/support view.
    pub plain_language_summary: String,
}

impl ManagedFleetAuditRow {
    /// Returns true when the row carries all seven required fleet-evidence classes.
    pub fn fleet_evidence_complete(&self) -> bool {
        REQUIRED_FLEET_EVIDENCE
            .iter()
            .all(|required| self.fleet_evidence.contains(required))
    }

    /// Returns true when the row lacks any required fleet-evidence class.
    pub fn missing_fleet_evidence(&self) -> Vec<FleetRolloutEvidenceClass> {
        REQUIRED_FLEET_EVIDENCE
            .iter()
            .copied()
            .filter(|required| !self.fleet_evidence.contains(required))
            .collect()
    }
}

/// One silent-deployment audit row.
///
/// Verifies that managed rows declare their silent install support class,
/// disclosed limits, and return-code families.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SilentDeploymentAuditRow {
    /// Stable row id for this audit entry.
    pub row_id: String,
    /// Install-topology row id this audit describes.
    pub topology_row_id: String,
    /// Silent install support class.
    pub support_class: SilentInstallSupportClass,
    /// True when at least one non-empty limit is declared.
    pub limits_declared: bool,
    /// Non-empty plain-language limit labels.
    pub disclosed_limits: Vec<String>,
    /// True when at least one return-code family ref is named.
    pub return_code_families_named: bool,
    /// Return-code family refs.
    pub return_code_family_refs: Vec<String>,
    /// Row qualification token.
    pub qualification_token: QualificationToken,
    /// Narrow reason when the row is not `Stable`.
    pub narrow_reason_token: NarrowReasonToken,
}

/// Aggregate summary for a [`HardenInstallTopologyPage`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenInstallTopologySummary {
    /// Total number of managed-fleet audit rows.
    pub managed_fleet_row_count: u32,
    /// Number of rows qualified as `Stable`.
    pub stable_row_count: u32,
    /// Number of rows qualified as `Beta`.
    pub beta_row_count: u32,
    /// Number of rows qualified as `Preview`.
    pub preview_row_count: u32,
    /// Number of rows qualified as `Withdrawn`.
    pub withdrawn_row_count: u32,
    /// Rollout rings covered by managed-fleet rows.
    pub rings_covered: Vec<RolloutRingClass>,
    /// Fleet-evidence classes present across all managed-fleet rows.
    pub fleet_evidence_classes_present: Vec<FleetRolloutEvidenceClass>,
    /// Number of managed-fleet rows with silent deployment limits declared.
    pub silent_deployment_limits_declared_count: u32,
    /// Number of managed-fleet rows where the admin view is complete.
    pub admin_view_complete_count: u32,
    /// Overall page qualification token.
    pub overall_qualification_token: QualificationToken,
}

/// One defect emitted by the harden-install-topology auditor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenInstallTopologyDefect {
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for the defect.
    pub narrow_reason: NarrowReasonToken,
    /// Narrow reason token (same value, kept for schema symmetry with other modules).
    pub narrow_reason_token: NarrowReasonToken,
    /// Row or packet ref that produced the defect.
    pub source: String,
    /// Export-safe note explaining the defect.
    pub note: String,
}

/// Hardened install-topology proof page.
///
/// The single inspectable record for the managed-fleet installation topology
/// audit lane. Dashboards, Help/About surfaces, diagnostics views, and support
/// exports should ingest this page rather than maintaining parallel prose
/// about ring, tenant, owner, or state-root topology.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenInstallTopologyPage {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Human-readable page label.
    pub page_label: String,
    /// ISO 8601 timestamp (opaque string; not used for comparison).
    pub generated_at: String,
    /// Managed-fleet install-topology audit rows.
    pub managed_fleet_rows: Vec<ManagedFleetAuditRow>,
    /// Silent-deployment audit rows (one per topology row covering silent install).
    pub silent_deployment_rows: Vec<SilentDeploymentAuditRow>,
    /// Defects found by the auditor.
    pub defects: Vec<HardenInstallTopologyDefect>,
    /// Aggregate summary.
    pub summary: HardenInstallTopologySummary,
}

impl HardenInstallTopologyPage {
    /// Audits the page and returns all defects.
    ///
    /// This is the canonical audit entry point. The absence of defects is the
    /// `Stable` claim.
    pub fn audit(&self) -> Vec<HardenInstallTopologyDefect> {
        audit_harden_install_topology_page(self)
    }

    /// Validates the page and returns a structured report.
    pub fn validate(&self) -> HardenInstallTopologyValidationReport {
        validate_harden_install_topology_page(self)
    }

    /// Returns a metadata-safe support-export projection.
    pub fn support_export_projection(&self) -> HardenInstallTopologySupportExport {
        let narrow_reasons: BTreeSet<NarrowReasonToken> = self
            .defects
            .iter()
            .map(|d| d.narrow_reason_token)
            .collect();
        let mut defect_counts: BTreeMap<String, u32> = BTreeMap::new();
        for defect in &self.defects {
            let key = format!("{:?}", defect.narrow_reason_token)
                .to_lowercase()
                .replace(' ', "_");
            *defect_counts.entry(key).or_insert(0) += 1;
        }
        HardenInstallTopologySupportExport {
            record_kind: HARDEN_INSTALL_TOPOLOGY_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: HARDEN_INSTALL_TOPOLOGY_SCHEMA_VERSION,
            shared_contract_ref: HARDEN_INSTALL_TOPOLOGY_SHARED_CONTRACT_REF.to_string(),
            export_id: format!("harden-install-topology:support-export:{}", self.page_id),
            generated_at: self.generated_at.clone(),
            page: self.clone(),
            narrow_reasons_present: narrow_reasons.into_iter().collect(),
            defect_counts_by_narrow_reason: defect_counts,
            raw_private_material_excluded: true,
        }
    }

    /// Returns the overall qualification token derived from the summary.
    pub fn overall_qualification(&self) -> QualificationToken {
        self.summary.overall_qualification_token
    }
}

/// Metadata-safe support-export for a harden-install-topology page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenInstallTopologySupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// ISO 8601 timestamp.
    pub generated_at: String,
    /// Source page.
    pub page: HardenInstallTopologyPage,
    /// Narrow reasons present across all defects.
    pub narrow_reasons_present: Vec<NarrowReasonToken>,
    /// Defect counts keyed by narrow-reason token string.
    pub defect_counts_by_narrow_reason: BTreeMap<String, u32>,
    /// Always `true`; no raw private material crosses this boundary.
    pub raw_private_material_excluded: bool,
}

/// Validation coverage collected from a harden-install-topology page.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenInstallTopologyCoverage {
    /// Rollout rings covered by managed-fleet rows.
    pub rings_covered: BTreeSet<RolloutRingClass>,
    /// Fleet-evidence classes present across all rows.
    pub fleet_evidence_classes: BTreeSet<FleetRolloutEvidenceClass>,
    /// Silent deployment support classes present.
    pub silent_deployment_classes: BTreeSet<SilentInstallSupportClass>,
    /// Binary root classes present.
    pub binary_root_classes: BTreeSet<BinaryRootClass>,
    /// Updater owner classes present.
    pub updater_owner_classes: BTreeSet<UpdaterOwnerClass>,
}

/// One validation finding from [`validate_harden_install_topology_page`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenInstallTopologyValidationFinding {
    /// Stable check id.
    pub check_id: String,
    /// Human-readable finding message.
    pub message: String,
    /// Row or packet ref that caused the finding.
    pub ref_id: String,
}

/// Validation report for a harden-install-topology page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenInstallTopologyValidationReport {
    /// True when validation found no errors.
    pub passed: bool,
    /// Coverage collected during validation.
    pub coverage: HardenInstallTopologyCoverage,
    /// Validation findings.
    pub findings: Vec<HardenInstallTopologyValidationFinding>,
}

/// Audits a [`HardenInstallTopologyPage`] and returns all defects.
///
/// Returns an empty `Vec` when the page is fully `Stable`. Each defect
/// carries a closed `narrow_reason_token` and an export-safe `note`.
///
/// One condition forces `Withdrawn` immediately and skips all remaining
/// checks: any managed-fleet row with `admin_view_complete: false`.
pub fn audit_harden_install_topology_page(
    page: &HardenInstallTopologyPage,
) -> Vec<HardenInstallTopologyDefect> {
    let mut defects = Vec::new();
    let mut defect_idx = 0u32;

    let mut next_id = || {
        defect_idx += 1;
        format!("harden-topology-defect:{:04}", defect_idx)
    };

    if page.managed_fleet_rows.is_empty() {
        defects.push(HardenInstallTopologyDefect {
            defect_id: next_id(),
            narrow_reason: NarrowReasonToken::NoManagedRows,
            narrow_reason_token: NarrowReasonToken::NoManagedRows,
            source: page.page_id.clone(),
            note: "no managed-fleet audit rows are present; page is narrowed to preview".into(),
        });
        return defects;
    }

    for row in &page.managed_fleet_rows {
        // Critical: admin_view_complete is the withdrawal condition.
        if !row.admin_view_complete {
            defects.push(HardenInstallTopologyDefect {
                defect_id: next_id(),
                narrow_reason: NarrowReasonToken::AdminViewIncomplete,
                narrow_reason_token: NarrowReasonToken::AdminViewIncomplete,
                source: row.row_id.clone(),
                note: format!(
                    "managed row '{}' cannot be fully inspected from the admin view; \
                     ring, tenant, owner, binary-root, state-roots, and policy source \
                     must all be present",
                    row.topology_row_id
                ),
            });
            // Withdrawal — skip remaining checks for this page.
            return defects;
        }

        if row.tenant_ref.trim().is_empty() {
            defects.push(HardenInstallTopologyDefect {
                defect_id: next_id(),
                narrow_reason: NarrowReasonToken::TenantNotNamed,
                narrow_reason_token: NarrowReasonToken::TenantNotNamed,
                source: row.row_id.clone(),
                note: format!(
                    "managed row '{}' has no tenant_ref; \
                     organization identity must be explicit on every managed install",
                    row.topology_row_id
                ),
            });
        }

        if row.policy_source_ref.trim().is_empty() {
            defects.push(HardenInstallTopologyDefect {
                defect_id: next_id(),
                narrow_reason: NarrowReasonToken::PolicySourceNotNamed,
                narrow_reason_token: NarrowReasonToken::PolicySourceNotNamed,
                source: row.row_id.clone(),
                note: format!(
                    "managed row '{}' has no policy_source_ref; \
                     GPO, MDM, or config-profile source must be named",
                    row.topology_row_id
                ),
            });
        }

        if row.state_root_audit.is_empty() {
            defects.push(HardenInstallTopologyDefect {
                defect_id: next_id(),
                narrow_reason: NarrowReasonToken::StateRootsNotAudited,
                narrow_reason_token: NarrowReasonToken::StateRootsNotAudited,
                source: row.row_id.clone(),
                note: format!(
                    "managed row '{}' has no state-root audit entries; \
                     every durable state root must be itemized with isolation and review class",
                    row.topology_row_id
                ),
            });
        }

        let missing = row.missing_fleet_evidence();
        if !missing.is_empty() {
            defects.push(HardenInstallTopologyDefect {
                defect_id: next_id(),
                narrow_reason: NarrowReasonToken::FleetEvidenceIncomplete,
                narrow_reason_token: NarrowReasonToken::FleetEvidenceIncomplete,
                source: row.row_id.clone(),
                note: format!(
                    "managed row '{}' is missing fleet-evidence classes: {:?}",
                    row.topology_row_id, missing
                ),
            });
        }
    }

    for row in &page.silent_deployment_rows {
        if !row.limits_declared {
            defects.push(HardenInstallTopologyDefect {
                defect_id: next_id(),
                narrow_reason: NarrowReasonToken::SilentLimitsNotDeclared,
                narrow_reason_token: NarrowReasonToken::SilentLimitsNotDeclared,
                source: row.row_id.clone(),
                note: format!(
                    "silent-deployment row '{}' has no disclosed limits; \
                     silent or managed install must declare limits in the admin view",
                    row.topology_row_id
                ),
            });
        }

        if !row.return_code_families_named {
            defects.push(HardenInstallTopologyDefect {
                defect_id: next_id(),
                narrow_reason: NarrowReasonToken::ReturnCodesNotNamed,
                narrow_reason_token: NarrowReasonToken::ReturnCodesNotNamed,
                source: row.row_id.clone(),
                note: format!(
                    "silent-deployment row '{}' has no return-code families; \
                     unattended install outcomes must name their return codes",
                    row.topology_row_id
                ),
            });
        }
    }

    defects
}

/// Validates a [`HardenInstallTopologyPage`] and returns a structured report.
pub fn validate_harden_install_topology_page(
    page: &HardenInstallTopologyPage,
) -> HardenInstallTopologyValidationReport {
    let mut findings = Vec::new();
    let mut coverage = HardenInstallTopologyCoverage::default();
    let mut finding_idx = 0u32;

    let mut push = |findings: &mut Vec<HardenInstallTopologyValidationFinding>,
                    check_id: &str,
                    message: String,
                    ref_id: String| {
        finding_idx += 1;
        findings.push(HardenInstallTopologyValidationFinding {
            check_id: check_id.to_string(),
            message,
            ref_id,
        });
    };

    if page.record_kind != HARDEN_INSTALL_TOPOLOGY_PAGE_RECORD_KIND {
        push(
            &mut findings,
            "harden_topology.page.record_kind",
            "page record_kind is not harden_install_topology_page_record".into(),
            page.page_id.clone(),
        );
    }
    if page.schema_version != HARDEN_INSTALL_TOPOLOGY_SCHEMA_VERSION {
        push(
            &mut findings,
            "harden_topology.page.schema_version",
            "page schema_version is unsupported".into(),
            page.page_id.clone(),
        );
    }
    if page.shared_contract_ref != HARDEN_INSTALL_TOPOLOGY_SHARED_CONTRACT_REF {
        push(
            &mut findings,
            "harden_topology.page.shared_contract_ref",
            "page shared_contract_ref does not match expected value".into(),
            page.page_id.clone(),
        );
    }
    if page.page_id.trim().is_empty() {
        push(
            &mut findings,
            "harden_topology.page.page_id_empty",
            "page_id must be non-empty".into(),
            page.page_id.clone(),
        );
    }

    for row in &page.managed_fleet_rows {
        coverage.rings_covered.insert(row.rollout_ring_class);
        coverage.binary_root_classes.insert(row.binary_root_class);
        coverage.updater_owner_classes.insert(row.updater_owner_class);
        for ev in &row.fleet_evidence {
            coverage.fleet_evidence_classes.insert(*ev);
        }
        coverage
            .silent_deployment_classes
            .insert(row.silent_deployment_class);

        if row.row_id.trim().is_empty() {
            push(
                &mut findings,
                "harden_topology.managed.row_id_empty",
                "managed-fleet row has empty row_id".into(),
                row.topology_row_id.clone(),
            );
        }
        if row.topology_row_id.trim().is_empty() {
            push(
                &mut findings,
                "harden_topology.managed.topology_row_id_empty",
                "managed-fleet row has empty topology_row_id".into(),
                row.row_id.clone(),
            );
        }
        if row.tenant_ref.trim().is_empty() {
            push(
                &mut findings,
                "harden_topology.managed.tenant_not_named",
                "managed-fleet row has empty tenant_ref".into(),
                row.row_id.clone(),
            );
        }
        if row.policy_source_ref.trim().is_empty() {
            push(
                &mut findings,
                "harden_topology.managed.policy_source_not_named",
                "managed-fleet row has empty policy_source_ref".into(),
                row.row_id.clone(),
            );
        }
        if row.state_root_audit.is_empty() {
            push(
                &mut findings,
                "harden_topology.managed.state_roots_not_audited",
                "managed-fleet row has no state-root audit entries".into(),
                row.row_id.clone(),
            );
        }
        if !row.fleet_evidence_complete() {
            push(
                &mut findings,
                "harden_topology.managed.fleet_evidence_incomplete",
                format!(
                    "managed-fleet row is missing fleet-evidence classes: {:?}",
                    row.missing_fleet_evidence()
                ),
                row.row_id.clone(),
            );
        }
        if !row.admin_view_complete {
            push(
                &mut findings,
                "harden_topology.managed.admin_view_incomplete",
                "managed-fleet row admin_view_complete is false — page cannot be stable".into(),
                row.row_id.clone(),
            );
        }
        if row.plain_language_summary.trim().is_empty() {
            push(
                &mut findings,
                "harden_topology.managed.summary_empty",
                "managed-fleet row has empty plain_language_summary".into(),
                row.row_id.clone(),
            );
        }
    }

    for row in &page.silent_deployment_rows {
        coverage
            .silent_deployment_classes
            .insert(row.support_class);

        if row.row_id.trim().is_empty() {
            push(
                &mut findings,
                "harden_topology.silent.row_id_empty",
                "silent-deployment row has empty row_id".into(),
                row.topology_row_id.clone(),
            );
        }
        if !row.limits_declared || row.disclosed_limits.is_empty() {
            push(
                &mut findings,
                "harden_topology.silent.limits_not_declared",
                "silent-deployment row has no disclosed limits".into(),
                row.row_id.clone(),
            );
        }
        if !row.return_code_families_named || row.return_code_family_refs.is_empty() {
            push(
                &mut findings,
                "harden_topology.silent.return_codes_not_named",
                "silent-deployment row has no return-code family refs".into(),
                row.row_id.clone(),
            );
        }
    }

    HardenInstallTopologyValidationReport {
        passed: findings.is_empty(),
        coverage,
        findings,
    }
}

/// Returns a fully populated, valid [`HardenInstallTopologyPage`] seeded from
/// the managed stable deployment lane.
///
/// This is the single inspectable record for the lane. Dashboards, Help/About
/// surfaces, and support exports should ingest it rather than cloning status
/// text or maintaining parallel deployment notes.
pub fn seeded_harden_install_topology_page() -> HardenInstallTopologyPage {
    let managed_windows_row = ManagedFleetAuditRow {
        row_id: "harden-topology.managed.windows.stable".into(),
        topology_row_id: "install.topology.windows.managed.stable".into(),
        platform_token: "windows".into(),
        tenant_ref: "fleet.tenant.windows.managed.stable.org_ref".into(),
        rollout_ring_class: RolloutRingClass::Pilot,
        updater_owner_class: UpdaterOwnerClass::ManagedFleet,
        binary_root_class: BinaryRootClass::PerMachineProgramArea,
        policy_source_ref: "fleet.policy_source.windows.gpo.managed.stable".into(),
        state_root_audit: vec![
            StateRootAuditEntry {
                state_root_ref: "state.per_user_configuration_root.stable".into(),
                isolation_class: StateRootIsolationClass::ChannelOwned,
                review_class: StateRootReviewClass::AdminPolicyReviewRequired,
                exposed_in_admin_view: true,
                contains_secret_material: false,
            },
            StateRootAuditEntry {
                state_root_ref: "state.per_user_recovery_root.stable".into(),
                isolation_class: StateRootIsolationClass::ChannelOwned,
                review_class: StateRootReviewClass::AdminPolicyReviewRequired,
                exposed_in_admin_view: true,
                contains_secret_material: false,
            },
            StateRootAuditEntry {
                state_root_ref: "state.per_user_derived_cache_root.stable".into(),
                isolation_class: StateRootIsolationClass::ChannelOwned,
                review_class: StateRootReviewClass::AdminPolicyReviewRequired,
                exposed_in_admin_view: true,
                contains_secret_material: false,
            },
            StateRootAuditEntry {
                state_root_ref: "state.per_machine_shared_data_root.stable".into(),
                isolation_class: StateRootIsolationClass::AdminPolicyOwned,
                review_class: StateRootReviewClass::AdminPolicyReviewRequired,
                exposed_in_admin_view: true,
                contains_secret_material: false,
            },
            StateRootAuditEntry {
                state_root_ref: "state.per_machine_admin_policy_root.stable".into(),
                isolation_class: StateRootIsolationClass::AdminPolicyOwned,
                review_class: StateRootReviewClass::AdminPolicyReviewRequired,
                exposed_in_admin_view: true,
                contains_secret_material: false,
            },
        ],
        fleet_evidence: REQUIRED_FLEET_EVIDENCE.to_vec(),
        silent_deployment_class: SilentInstallSupportClass::ManagedOnly,
        silent_deployment_limits_declared: true,
        admin_view_complete: true,
        qualification_token: QualificationToken::Stable,
        narrow_reason_token: NarrowReasonToken::NotNarrowed,
        plain_language_summary: "Windows managed-fleet pilot ring: tenant identity, ring, \
            GPO policy source, machine and user state-root topology, and full fleet \
            evidence are visible from the admin support view without prose."
            .into(),
    };

    let airgap_row = ManagedFleetAuditRow {
        row_id: "harden-topology.managed.airgap.stable".into(),
        topology_row_id: "install.topology.airgap.bundle.stable".into(),
        platform_token: "air_gap_bundle_target".into(),
        tenant_ref: "fleet.tenant.airgap.managed.stable.org_ref".into(),
        rollout_ring_class: RolloutRingClass::Broad,
        updater_owner_class: UpdaterOwnerClass::Admin,
        binary_root_class: BinaryRootClass::OfflineBundleExtractedProgramArea,
        policy_source_ref: "fleet.policy_source.airgap.offline_bundle.stable".into(),
        state_root_audit: vec![
            StateRootAuditEntry {
                state_root_ref: "state.per_user_configuration_root.stable".into(),
                isolation_class: StateRootIsolationClass::ChannelOwned,
                review_class: StateRootReviewClass::MirrorVerificationReviewRequired,
                exposed_in_admin_view: true,
                contains_secret_material: false,
            },
            StateRootAuditEntry {
                state_root_ref: "state.offline_bundle_mirror_metadata_root.stable".into(),
                isolation_class: StateRootIsolationClass::MirrorMetadataOwned,
                review_class: StateRootReviewClass::MirrorVerificationReviewRequired,
                exposed_in_admin_view: true,
                contains_secret_material: false,
            },
        ],
        fleet_evidence: REQUIRED_FLEET_EVIDENCE.to_vec(),
        silent_deployment_class: SilentInstallSupportClass::Full,
        silent_deployment_limits_declared: true,
        admin_view_complete: true,
        qualification_token: QualificationToken::Stable,
        narrow_reason_token: NarrowReasonToken::NotNarrowed,
        plain_language_summary: "Air-gapped bundle broad ring: tenant identity, offline-bundle \
            binary root, mirror-metadata state root, and full fleet evidence are visible \
            from the admin support view without reading external deployment notes."
            .into(),
    };

    let silent_windows_row = SilentDeploymentAuditRow {
        row_id: "harden-topology.silent.windows.stable".into(),
        topology_row_id: "install.topology.windows.managed.stable".into(),
        support_class: SilentInstallSupportClass::ManagedOnly,
        limits_declared: true,
        disclosed_limits: vec![
            "Managed silent deployment is owned by the fleet lane and visible from diagnostics."
                .into(),
            "Rollback is a ring action, not a user-owned update button.".into(),
            "Silent uninstall preserves declared user state roots by contract.".into(),
        ],
        return_code_families_named: true,
        return_code_family_refs: vec![
            "success".into(),
            "partial_success".into(),
            "trust_policy_denial".into(),
            "rollback_required".into(),
            "admin_required".into(),
        ],
        qualification_token: QualificationToken::Stable,
        narrow_reason_token: NarrowReasonToken::NotNarrowed,
    };

    let silent_airgap_row = SilentDeploymentAuditRow {
        row_id: "harden-topology.silent.airgap.stable".into(),
        topology_row_id: "install.topology.airgap.bundle.stable".into(),
        support_class: SilentInstallSupportClass::Full,
        limits_declared: true,
        disclosed_limits: vec![
            "Full silent install, update, rollback, and uninstall are claimed for the air-gap lane."
                .into(),
            "Mirror-bundle verification must pass before any silent install proceeds.".into(),
            "Local-core continuity is maintained throughout silent deployment.".into(),
        ],
        return_code_families_named: true,
        return_code_family_refs: vec![
            "success".into(),
            "partial_success".into(),
            "mirror_verification_failed".into(),
            "rollback_required".into(),
            "offline_policy_denial".into(),
        ],
        qualification_token: QualificationToken::Stable,
        narrow_reason_token: NarrowReasonToken::NotNarrowed,
    };

    let rings_covered: Vec<RolloutRingClass> =
        vec![RolloutRingClass::Pilot, RolloutRingClass::Broad];
    let fleet_evidence_classes_present: Vec<FleetRolloutEvidenceClass> =
        REQUIRED_FLEET_EVIDENCE.to_vec();

    let summary = HardenInstallTopologySummary {
        managed_fleet_row_count: 2,
        stable_row_count: 2,
        beta_row_count: 0,
        preview_row_count: 0,
        withdrawn_row_count: 0,
        rings_covered,
        fleet_evidence_classes_present,
        silent_deployment_limits_declared_count: 2,
        admin_view_complete_count: 2,
        overall_qualification_token: QualificationToken::Stable,
    };

    HardenInstallTopologyPage {
        record_kind: HARDEN_INSTALL_TOPOLOGY_PAGE_RECORD_KIND.to_string(),
        schema_version: HARDEN_INSTALL_TOPOLOGY_SCHEMA_VERSION,
        shared_contract_ref: HARDEN_INSTALL_TOPOLOGY_SHARED_CONTRACT_REF.to_string(),
        page_id: "harden-install-topology:managed-fleet:seeded:0001".into(),
        page_label: "Hardened Install Topology — Managed Fleet Audit".into(),
        generated_at: "2026-06-01T00:00:00Z".into(),
        managed_fleet_rows: vec![managed_windows_row, airgap_row],
        silent_deployment_rows: vec![silent_windows_row, silent_airgap_row],
        defects: vec![],
        summary,
    }
}
