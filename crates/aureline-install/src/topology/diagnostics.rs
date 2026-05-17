//! Exact-build install diagnostics for product, CLI, support, and fleet views.
//!
//! The diagnostics packet is the beta-facing projection that sits on top of the
//! install-topology rows. It answers the field-support question "what exact
//! build is installed here, who owns updates, and which state roots belong to
//! this install?" without scraping host-specific installer logs or exposing raw
//! paths.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::{
    BinaryRootClass, ChannelClass, InstallModeClass, PlatformClass, RolloutRingClass,
    TopologySurfaceClass, UpdaterOwnerClass,
};

/// Schema version for exact-build install diagnostics packets.
pub const INSTALL_DIAGNOSTICS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`InstallDiagnosticsPacket`].
pub const INSTALL_DIAGNOSTICS_PACKET_RECORD_KIND: &str = "install_diagnostics_packet";

/// Stable record-kind tag for [`InstallDiagnosticsSupportExport`].
pub const INSTALL_DIAGNOSTICS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "install_diagnostics_support_export";

/// Exact-build manifest availability for one install row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExactBuildManifestState {
    /// The exact-build manifest is present and joinable from diagnostics.
    Present,
    /// The install row reserves the manifest slot but cannot claim a manifest yet.
    Reserved,
    /// The install row is blocked because exact-build identity cannot be resolved.
    Blocked,
}

/// Isolation posture for one durable state-root row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateRootIsolationClass {
    /// Root is owned by exactly one release channel.
    ChannelOwned,
    /// Root is colocated under a portable install directory.
    PortableColocated,
    /// Root is owned by administrator policy.
    AdminPolicyOwned,
    /// Root stores mirror or offline-bundle metadata.
    MirrorMetadataOwned,
    /// Root is shared read-only across channels.
    SharedReadOnly,
}

/// Review posture before an install row can use or import state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateRootReviewClass {
    /// No cross-install review is needed.
    NoReviewNeeded,
    /// Cross-channel state movement requires compare-before-commit review.
    ExplicitImportReviewRequired,
    /// Portable mode owns no host-global OS entry points.
    PortableNoOsOwnership,
    /// Administrator policy review owns this state boundary.
    AdminPolicyReviewRequired,
    /// Mirror or offline metadata verification review is required.
    MirrorVerificationReviewRequired,
}

/// Fleet evidence class required to identify an installed build remotely.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FleetRolloutEvidenceClass {
    /// Ring assignment is present in the diagnostic row.
    RingAssignment,
    /// Exact-build inventory identity is present.
    ExactBuildInventory,
    /// Managed-package report identity is present.
    ManagedPackageReport,
    /// Policy root identity is present.
    PolicyRoot,
    /// Rollback target identity is present.
    RollbackTarget,
    /// Last verification status is present.
    VerificationStatus,
    /// Support-export backreference is present.
    SupportExport,
}

/// Last verification status shown by diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallVerificationState {
    /// Install signature, exact-build manifest, and state-root checks passed.
    Verified,
    /// Verification slot exists but the row is not yet claimable.
    Reserved,
    /// Verification failed or stale evidence blocks the row.
    Blocked,
}

/// Upstream contracts consumed by an install diagnostics packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallDiagnosticsContractRefs {
    /// Install-topology packet or fixture ref.
    pub install_topology_packet_ref: String,
    /// Install-topology matrix ref.
    pub install_topology_matrix_ref: String,
    /// Durable state-root map ref.
    pub state_root_map_ref: String,
    /// Exact-build identity schema ref.
    pub exact_build_identity_schema_ref: String,
    /// Release artifact graph ref.
    pub artifact_graph_ref: String,
    /// Managed-package report seed ref.
    pub managed_package_report_seed_ref: String,
}

/// Exact-build identity captured for one installed row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExactBuildInstallIdentity {
    /// Exact-build identity ref resolved from the running binary or manifest.
    pub exact_build_identity_ref: String,
    /// Source record that minted the build identity.
    pub build_identity_source_ref: String,
    /// Release artifact-graph node or bundle that carries this identity.
    pub artifact_graph_ref: String,
    /// Release-channel class from the exact-build identity record.
    pub release_channel_class: String,
    /// Display version derived from the exact-build identity record.
    pub product_version: String,
    /// Source revision ref carried by the exact-build identity record.
    pub source_revision_ref: String,
    /// Exact-build manifest state for this install row.
    pub manifest_state: ExactBuildManifestState,
}

/// One durable state-root diagnostic entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateRootDiagnostic {
    /// Durable state-root ref from the release state-root map.
    pub state_root_ref: String,
    /// Isolation posture for the state root.
    pub isolation_class: StateRootIsolationClass,
    /// Owning channel when the root is channel-owned.
    pub owning_channel_class: Option<ChannelClass>,
    /// True when product diagnostics can show this root as metadata.
    pub exposed_in_product: bool,
    /// True when CLI diagnostics can show this root as metadata.
    pub exposed_in_cli: bool,
    /// True when support export can include this root as metadata.
    pub exposed_in_support_export: bool,
    /// True when the underlying root may contain secret material.
    pub contains_secret_material: bool,
    /// Review posture for using or importing this root.
    pub review_class: StateRootReviewClass,
}

impl StateRootDiagnostic {
    /// Returns true when the state-root entry is allowed to overlap across channels.
    pub fn is_shared_read_only(&self) -> bool {
        self.isolation_class == StateRootIsolationClass::SharedReadOnly
    }
}

/// Fleet-rollout diagnostics for a managed install row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FleetRolloutDiagnostic {
    /// Rollout ring assigned to the install row.
    pub rollout_ring_class: RolloutRingClass,
    /// Managed-package report ref used by fleet inventory.
    pub managed_package_report_ref: String,
    /// Fleet inventory probe ref that surfaces the row without host scraping.
    pub inventory_probe_ref: String,
    /// Policy roots that govern this fleet install.
    pub policy_root_refs: Vec<String>,
    /// Rollback target class available to the fleet owner.
    pub rollback_target_class: String,
    /// Exact-build identity observed by fleet inventory.
    pub exact_build_identity_ref: String,
    /// Evidence classes present on the diagnostic row.
    pub evidence: Vec<FleetRolloutEvidenceClass>,
    /// True when the inventory probe can run without interactive GUI launch.
    pub inventory_probe_available: bool,
    /// True when deprovision or rollback preserves local user work by contract.
    pub deprovision_preserves_local_work: bool,
}

/// One install diagnostics row consumed by product, CLI, support, and fleet surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallDiagnosticRow {
    /// Stable diagnostic row id.
    pub diagnostic_row_id: String,
    /// Install-topology row id this diagnostic describes.
    pub topology_row_id: String,
    /// Install-profile card ref this row resolves to.
    pub install_profile_card_ref: String,
    /// Platform class.
    pub platform_class: PlatformClass,
    /// Install mode class.
    pub install_mode_class: InstallModeClass,
    /// Channel class.
    pub channel_class: ChannelClass,
    /// Paired channel when side-by-side behavior is present.
    pub paired_channel_class: Option<ChannelClass>,
    /// Updater owner class.
    pub updater_owner_class: UpdaterOwnerClass,
    /// Binary root class.
    pub binary_root_class: BinaryRootClass,
    /// Exact-build identity diagnostic for the row.
    pub exact_build: ExactBuildInstallIdentity,
    /// Durable state-root diagnostics.
    pub durable_state_roots: Vec<StateRootDiagnostic>,
    /// Policy-root refs governing this install.
    pub policy_root_refs: Vec<String>,
    /// Peer topology rows that can coexist with this row.
    pub side_by_side_peer_row_ids: Vec<String>,
    /// State-root review class for cross-row movement or import.
    pub state_root_review_class: StateRootReviewClass,
    /// Fleet rollout diagnostic when this row is managed.
    pub fleet_rollout: Option<FleetRolloutDiagnostic>,
    /// Last install verification state.
    pub last_verification_state: InstallVerificationState,
    /// Rollback target class.
    pub rollback_target_class: String,
    /// Support bundle or support projection ref carrying this diagnostic.
    pub support_bundle_ref: String,
    /// Redaction class for diagnostics and support export.
    pub redaction_class: String,
    /// Product/support surfaces that must render the row.
    pub surface_claims: Vec<TopologySurfaceClass>,
}

impl InstallDiagnosticRow {
    /// Returns true when this row models side-by-side behavior.
    pub fn is_side_by_side(&self) -> bool {
        self.paired_channel_class.is_some() || !self.side_by_side_peer_row_ids.is_empty()
    }

    /// Returns the durable state-root refs carried by this row.
    pub fn state_root_refs(&self) -> Vec<String> {
        self.durable_state_roots
            .iter()
            .map(|root| root.state_root_ref.clone())
            .collect()
    }

    /// Builds a cross-surface truth fingerprint for this diagnostic row.
    pub fn truth_fingerprint(&self) -> InstallDiagnosticsTruthFingerprint {
        InstallDiagnosticsTruthFingerprint {
            install_mode_class: self.install_mode_class,
            channel_class: self.channel_class,
            updater_owner_class: self.updater_owner_class,
            binary_root_class: self.binary_root_class,
            exact_build_identity_ref: self.exact_build.exact_build_identity_ref.clone(),
            durable_state_root_refs: self.state_root_refs(),
            policy_root_refs: self.policy_root_refs.clone(),
            state_root_review_class: self.state_root_review_class,
            last_verification_state: self.last_verification_state,
            fleet_rollout_ring: self
                .fleet_rollout
                .as_ref()
                .map(|fleet| fleet.rollout_ring_class),
            managed_package_report_ref: self
                .fleet_rollout
                .as_ref()
                .map(|fleet| fleet.managed_package_report_ref.clone()),
            rollback_target_class: self.rollback_target_class.clone(),
        }
    }
}

/// Exact-build install diagnostics packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallDiagnosticsPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Install-topology packet id this diagnostics packet was generated from.
    pub generated_from_topology_packet_id: String,
    /// Upstream contract refs.
    pub contract_refs: InstallDiagnosticsContractRefs,
    /// Exact-build identity refs known to this diagnostics packet.
    pub exact_build_identity_refs: Vec<String>,
    /// Install diagnostic rows.
    pub rows: Vec<InstallDiagnosticRow>,
}

impl InstallDiagnosticsPacket {
    /// Validates exact-build identity, state-root isolation, fleet inventory,
    /// surface parity, and support-export posture.
    pub fn validate(&self) -> InstallDiagnosticsValidationReport {
        let mut validator = InstallDiagnosticsValidator::new(self);
        validator.validate();
        validator.finish()
    }

    /// Returns a product or support surface projection from the packet.
    pub fn surface_projection(
        &self,
        surface_class: TopologySurfaceClass,
    ) -> InstallDiagnosticsSurfaceProjection {
        let rows = self
            .rows
            .iter()
            .filter(|row| row.surface_claims.contains(&surface_class))
            .map(InstallDiagnosticsSurfaceRow::from)
            .collect();
        InstallDiagnosticsSurfaceProjection {
            surface_class,
            packet_id: self.packet_id.clone(),
            rows,
        }
    }

    /// Returns a metadata-safe support-export projection.
    pub fn support_export_projection(&self) -> InstallDiagnosticsSupportExport {
        InstallDiagnosticsSupportExport {
            record_kind: INSTALL_DIAGNOSTICS_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: INSTALL_DIAGNOSTICS_SCHEMA_VERSION,
            packet_id: self.packet_id.clone(),
            projection: self.surface_projection(TopologySurfaceClass::SupportExport),
            exact_build_identity_refs: self.exact_build_identity_refs.clone(),
            redaction_class: "metadata_only_no_paths_or_secrets".to_string(),
        }
    }

    /// Finds a diagnostics row by install-topology row id.
    pub fn row_by_topology_id(&self, topology_row_id: &str) -> Option<&InstallDiagnosticRow> {
        self.rows
            .iter()
            .find(|row| row.topology_row_id == topology_row_id)
    }

    /// Returns true when two diagnostics rows expose disjoint mutable roots.
    pub fn state_roots_disjoint_for_rows(&self, left_row_id: &str, right_row_id: &str) -> bool {
        let Some(left) = self.row_by_topology_id(left_row_id) else {
            return false;
        };
        let Some(right) = self.row_by_topology_id(right_row_id) else {
            return false;
        };
        roots_disjoint_or_shared_read_only(left, right)
    }
}

/// One row rendered on a product, CLI, or support surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallDiagnosticsSurfaceRow {
    /// Stable diagnostic row id.
    pub diagnostic_row_id: String,
    /// Install-topology row id this diagnostic describes.
    pub topology_row_id: String,
    /// Platform class.
    pub platform_class: PlatformClass,
    /// Install mode class.
    pub install_mode_class: InstallModeClass,
    /// Channel class.
    pub channel_class: ChannelClass,
    /// Updater owner class.
    pub updater_owner_class: UpdaterOwnerClass,
    /// Binary root class.
    pub binary_root_class: BinaryRootClass,
    /// Exact-build identity ref.
    pub exact_build_identity_ref: String,
    /// Durable state-root refs.
    pub durable_state_root_refs: Vec<String>,
    /// Policy-root refs.
    pub policy_root_refs: Vec<String>,
    /// State-root review class.
    pub state_root_review_class: StateRootReviewClass,
    /// Last install verification state.
    pub last_verification_state: InstallVerificationState,
    /// Managed-package report ref when present.
    pub managed_package_report_ref: Option<String>,
    /// Surface-stable truth fingerprint.
    pub truth_fingerprint: InstallDiagnosticsTruthFingerprint,
}

impl From<&InstallDiagnosticRow> for InstallDiagnosticsSurfaceRow {
    fn from(row: &InstallDiagnosticRow) -> Self {
        Self {
            diagnostic_row_id: row.diagnostic_row_id.clone(),
            topology_row_id: row.topology_row_id.clone(),
            platform_class: row.platform_class,
            install_mode_class: row.install_mode_class,
            channel_class: row.channel_class,
            updater_owner_class: row.updater_owner_class,
            binary_root_class: row.binary_root_class,
            exact_build_identity_ref: row.exact_build.exact_build_identity_ref.clone(),
            durable_state_root_refs: row.state_root_refs(),
            policy_root_refs: row.policy_root_refs.clone(),
            state_root_review_class: row.state_root_review_class,
            last_verification_state: row.last_verification_state,
            managed_package_report_ref: row
                .fleet_rollout
                .as_ref()
                .map(|fleet| fleet.managed_package_report_ref.clone()),
            truth_fingerprint: row.truth_fingerprint(),
        }
    }
}

/// Projection for one install diagnostics surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallDiagnosticsSurfaceProjection {
    /// Surface class.
    pub surface_class: TopologySurfaceClass,
    /// Source packet id.
    pub packet_id: String,
    /// Rows rendered on the surface.
    pub rows: Vec<InstallDiagnosticsSurfaceRow>,
}

impl InstallDiagnosticsSurfaceProjection {
    /// Returns row truth fingerprints keyed by topology row id.
    pub fn truth_fingerprints(&self) -> BTreeMap<String, InstallDiagnosticsTruthFingerprint> {
        self.rows
            .iter()
            .map(|row| (row.topology_row_id.clone(), row.truth_fingerprint.clone()))
            .collect()
    }
}

/// Metadata-safe support-export projection for install diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallDiagnosticsSupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Source packet id.
    pub packet_id: String,
    /// Support-export surface projection.
    pub projection: InstallDiagnosticsSurfaceProjection,
    /// Exact-build identity refs carried in the support export.
    pub exact_build_identity_refs: Vec<String>,
    /// Redaction class for the projection.
    pub redaction_class: String,
}

/// Cross-surface fingerprint for fields that must never contradict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallDiagnosticsTruthFingerprint {
    /// Install mode class.
    pub install_mode_class: InstallModeClass,
    /// Channel class.
    pub channel_class: ChannelClass,
    /// Updater owner class.
    pub updater_owner_class: UpdaterOwnerClass,
    /// Binary root class.
    pub binary_root_class: BinaryRootClass,
    /// Exact-build identity ref.
    pub exact_build_identity_ref: String,
    /// Durable state-root refs.
    pub durable_state_root_refs: Vec<String>,
    /// Policy-root refs.
    pub policy_root_refs: Vec<String>,
    /// State-root review class.
    pub state_root_review_class: StateRootReviewClass,
    /// Last install verification state.
    pub last_verification_state: InstallVerificationState,
    /// Fleet rollout ring when present.
    pub fleet_rollout_ring: Option<RolloutRingClass>,
    /// Managed-package report ref when present.
    pub managed_package_report_ref: Option<String>,
    /// Rollback target class.
    pub rollback_target_class: String,
}

/// Validation coverage collected from an install diagnostics packet.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallDiagnosticsCoverage {
    /// Install modes covered by diagnostics rows.
    pub install_modes: BTreeSet<InstallModeClass>,
    /// Channels covered by diagnostics rows.
    pub channels: BTreeSet<ChannelClass>,
    /// Product and support surfaces covered by diagnostics rows.
    pub surfaces: BTreeSet<TopologySurfaceClass>,
    /// Exact-build identity refs covered by diagnostics rows.
    pub exact_build_identity_refs: BTreeSet<String>,
    /// Fleet rollout rings covered by diagnostics rows.
    pub fleet_rollout_rings: BTreeSet<RolloutRingClass>,
    /// State-root review classes covered by diagnostics rows.
    pub state_root_review_classes: BTreeSet<StateRootReviewClass>,
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallDiagnosticsValidationFinding {
    /// Stable check id.
    pub check_id: String,
    /// Human-readable finding message.
    pub message: String,
    /// Row or packet ref associated with the finding.
    pub ref_id: String,
}

/// Validation report for an install diagnostics packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallDiagnosticsValidationReport {
    /// True when validation found no errors.
    pub passed: bool,
    /// Coverage collected while validating.
    pub coverage: InstallDiagnosticsCoverage,
    /// Validation findings.
    pub findings: Vec<InstallDiagnosticsValidationFinding>,
}

struct InstallDiagnosticsValidator<'a> {
    packet: &'a InstallDiagnosticsPacket,
    coverage: InstallDiagnosticsCoverage,
    findings: Vec<InstallDiagnosticsValidationFinding>,
    seen_row_ids: BTreeSet<String>,
    seen_diagnostic_ids: BTreeSet<String>,
}

impl<'a> InstallDiagnosticsValidator<'a> {
    fn new(packet: &'a InstallDiagnosticsPacket) -> Self {
        Self {
            packet,
            coverage: InstallDiagnosticsCoverage::default(),
            findings: Vec::new(),
            seen_row_ids: BTreeSet::new(),
            seen_diagnostic_ids: BTreeSet::new(),
        }
    }

    fn validate(&mut self) {
        self.validate_header();
        for row in &self.packet.rows {
            self.validate_row(row);
        }
        self.validate_required_coverage();
        self.validate_cross_row_state_boundaries();
    }

    fn finish(self) -> InstallDiagnosticsValidationReport {
        InstallDiagnosticsValidationReport {
            passed: self.findings.is_empty(),
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn push(&mut self, check_id: &str, message: impl Into<String>, ref_id: impl Into<String>) {
        self.findings.push(InstallDiagnosticsValidationFinding {
            check_id: check_id.to_string(),
            message: message.into(),
            ref_id: ref_id.into(),
        });
    }

    fn validate_header(&mut self) {
        if self.packet.record_kind != INSTALL_DIAGNOSTICS_PACKET_RECORD_KIND {
            self.push(
                "install_diagnostics.packet.record_kind",
                "packet record_kind is not install_diagnostics_packet",
                &self.packet.packet_id,
            );
        }
        if self.packet.schema_version != INSTALL_DIAGNOSTICS_SCHEMA_VERSION {
            self.push(
                "install_diagnostics.packet.schema_version",
                "packet schema_version is unsupported",
                &self.packet.packet_id,
            );
        }
        if self
            .packet
            .generated_from_topology_packet_id
            .trim()
            .is_empty()
        {
            self.push(
                "install_diagnostics.packet.topology_ref_missing",
                "packet must name the install-topology packet it projects",
                &self.packet.packet_id,
            );
        }
        if self.packet.exact_build_identity_refs.is_empty() {
            self.push(
                "install_diagnostics.packet.exact_build_refs_missing",
                "packet must carry at least one exact-build identity ref",
                &self.packet.packet_id,
            );
        }
        if self.packet.rows.is_empty() {
            self.push(
                "install_diagnostics.packet.rows_empty",
                "packet must contain at least one diagnostics row",
                &self.packet.packet_id,
            );
        }
    }

    fn validate_row(&mut self, row: &InstallDiagnosticRow) {
        if row.diagnostic_row_id.trim().is_empty() {
            self.push(
                "install_diagnostics.row.diagnostic_id_missing",
                "diagnostic row id must not be empty",
                row.topology_row_id.clone(),
            );
        }
        if !self
            .seen_diagnostic_ids
            .insert(row.diagnostic_row_id.clone())
        {
            self.push(
                "install_diagnostics.row.diagnostic_id_duplicate",
                "diagnostic row id must be unique",
                row.diagnostic_row_id.clone(),
            );
        }
        if row.topology_row_id.trim().is_empty() {
            self.push(
                "install_diagnostics.row.topology_id_missing",
                "topology row id must not be empty",
                row.diagnostic_row_id.clone(),
            );
        }
        if !self.seen_row_ids.insert(row.topology_row_id.clone()) {
            self.push(
                "install_diagnostics.row.topology_id_duplicate",
                "topology row id must be unique in diagnostics",
                row.topology_row_id.clone(),
            );
        }

        self.coverage.install_modes.insert(row.install_mode_class);
        self.coverage.channels.insert(row.channel_class);
        self.coverage
            .exact_build_identity_refs
            .insert(row.exact_build.exact_build_identity_ref.clone());
        self.coverage
            .state_root_review_classes
            .insert(row.state_root_review_class);
        for surface in &row.surface_claims {
            self.coverage.surfaces.insert(*surface);
        }
        if let Some(fleet) = &row.fleet_rollout {
            self.coverage
                .fleet_rollout_rings
                .insert(fleet.rollout_ring_class);
        }

        self.validate_surfaces(row);
        self.validate_exact_build(row);
        self.validate_state_roots(row);
        self.validate_portable(row);
        self.validate_side_by_side(row);
        self.validate_fleet(row);
    }

    fn validate_surfaces(&mut self, row: &InstallDiagnosticRow) {
        let actual: BTreeSet<TopologySurfaceClass> = row.surface_claims.iter().copied().collect();
        for required in [
            TopologySurfaceClass::About,
            TopologySurfaceClass::Diagnostics,
            TopologySurfaceClass::Cli,
            TopologySurfaceClass::SupportExport,
        ] {
            if !actual.contains(&required) {
                self.push(
                    "install_diagnostics.row.surface_missing",
                    format!("row is missing required diagnostics surface: {required:?}"),
                    row.topology_row_id.clone(),
                );
            }
        }
        if row.support_bundle_ref.trim().is_empty() {
            self.push(
                "install_diagnostics.row.support_ref_missing",
                "diagnostic row must carry a support bundle or projection ref",
                row.topology_row_id.clone(),
            );
        }
        if row.redaction_class != "metadata_only_no_paths_or_secrets" {
            self.push(
                "install_diagnostics.row.redaction_class",
                "install diagnostics must be metadata-only and path/secret safe",
                row.topology_row_id.clone(),
            );
        }
    }

    fn validate_exact_build(&mut self, row: &InstallDiagnosticRow) {
        let identity = &row.exact_build;
        if !identity
            .exact_build_identity_ref
            .starts_with("build-id:aureline:")
        {
            self.push(
                "install_diagnostics.exact_build.ref_invalid",
                "exact-build identity ref must use the build-id:aureline namespace",
                row.topology_row_id.clone(),
            );
        }
        if !self
            .packet
            .exact_build_identity_refs
            .contains(&identity.exact_build_identity_ref)
        {
            self.push(
                "install_diagnostics.exact_build.ref_not_listed",
                "row exact-build identity ref must appear in packet exact_build_identity_refs",
                row.topology_row_id.clone(),
            );
        }
        if identity.manifest_state != ExactBuildManifestState::Present {
            self.push(
                "install_diagnostics.exact_build.manifest_not_present",
                "claimed beta diagnostics rows must carry a present exact-build manifest",
                row.topology_row_id.clone(),
            );
        }
        for (field, value) in [
            (
                "build_identity_source_ref",
                &identity.build_identity_source_ref,
            ),
            ("artifact_graph_ref", &identity.artifact_graph_ref),
            ("release_channel_class", &identity.release_channel_class),
            ("product_version", &identity.product_version),
            ("source_revision_ref", &identity.source_revision_ref),
        ] {
            if value.trim().is_empty() {
                self.push(
                    "install_diagnostics.exact_build.field_missing",
                    format!("exact-build {field} must not be empty"),
                    row.topology_row_id.clone(),
                );
            }
        }
    }

    fn validate_state_roots(&mut self, row: &InstallDiagnosticRow) {
        if row.durable_state_roots.is_empty() {
            self.push(
                "install_diagnostics.row.state_roots_missing",
                "diagnostic row must disclose at least one durable state root",
                row.topology_row_id.clone(),
            );
        }
        let mut seen_roots = BTreeSet::new();
        for root in &row.durable_state_roots {
            if root.state_root_ref.trim().is_empty() {
                self.push(
                    "install_diagnostics.state_root.ref_missing",
                    "state root ref must not be empty",
                    row.topology_row_id.clone(),
                );
            }
            if !seen_roots.insert(root.state_root_ref.clone()) {
                self.push(
                    "install_diagnostics.state_root.duplicate",
                    "state root ref must be unique within a diagnostics row",
                    row.topology_row_id.clone(),
                );
            }
            if !root.exposed_in_product || !root.exposed_in_cli || !root.exposed_in_support_export {
                self.push(
                    "install_diagnostics.state_root.not_exported",
                    "state roots must be visible in product, CLI, and support diagnostics",
                    root.state_root_ref.clone(),
                );
            }
            if root.isolation_class == StateRootIsolationClass::ChannelOwned
                && root.owning_channel_class != Some(row.channel_class)
            {
                self.push(
                    "install_diagnostics.state_root.channel_owner_mismatch",
                    "channel-owned state root must name the row channel as owner",
                    root.state_root_ref.clone(),
                );
            }
            if root.contains_secret_material
                && row.redaction_class != "metadata_only_no_paths_or_secrets"
            {
                self.push(
                    "install_diagnostics.state_root.secret_redaction",
                    "secret-bearing roots require metadata-only diagnostics redaction",
                    root.state_root_ref.clone(),
                );
            }
        }
    }

    fn validate_portable(&mut self, row: &InstallDiagnosticRow) {
        if row.install_mode_class != InstallModeClass::Portable {
            return;
        }
        if row.state_root_review_class != StateRootReviewClass::PortableNoOsOwnership {
            self.push(
                "install_diagnostics.portable.review_class",
                "portable diagnostics must disclose portable no-OS-ownership posture",
                row.topology_row_id.clone(),
            );
        }
        for root in &row.durable_state_roots {
            if root.isolation_class != StateRootIsolationClass::PortableColocated {
                self.push(
                    "install_diagnostics.portable.root_not_colocated",
                    "portable diagnostics must use portable-colocated state roots only",
                    root.state_root_ref.clone(),
                );
            }
            if !root.state_root_ref.contains("portable_colocated_root") {
                self.push(
                    "install_diagnostics.portable.root_ref_not_portable",
                    "portable diagnostics root ref must resolve to the portable colocated root",
                    root.state_root_ref.clone(),
                );
            }
        }
        if !row.policy_root_refs.is_empty() {
            self.push(
                "install_diagnostics.portable.policy_root_claimed",
                "portable diagnostics must not claim admin policy roots",
                row.topology_row_id.clone(),
            );
        }
    }

    fn validate_side_by_side(&mut self, row: &InstallDiagnosticRow) {
        if !row.is_side_by_side() {
            return;
        }
        if row.paired_channel_class.is_none() {
            self.push(
                "install_diagnostics.side_by_side.paired_channel_missing",
                "side-by-side diagnostics must name a paired channel",
                row.topology_row_id.clone(),
            );
        }
        if row.side_by_side_peer_row_ids.is_empty() {
            self.push(
                "install_diagnostics.side_by_side.peer_missing",
                "side-by-side diagnostics must name peer rows",
                row.topology_row_id.clone(),
            );
        }
        if matches!(row.install_mode_class, InstallModeClass::Portable) {
            return;
        }
        if row.state_root_review_class != StateRootReviewClass::ExplicitImportReviewRequired {
            self.push(
                "install_diagnostics.side_by_side.import_review_missing",
                "side-by-side installed rows must require explicit import review",
                row.topology_row_id.clone(),
            );
        }
    }

    fn validate_fleet(&mut self, row: &InstallDiagnosticRow) {
        if !matches!(
            row.install_mode_class,
            InstallModeClass::ManagedDeployed | InstallModeClass::PerMachineInstalled
        ) && row.updater_owner_class != UpdaterOwnerClass::ManagedFleet
        {
            return;
        }
        if row.updater_owner_class != UpdaterOwnerClass::ManagedFleet {
            return;
        }
        let Some(fleet) = &row.fleet_rollout else {
            self.push(
                "install_diagnostics.fleet.missing",
                "managed-fleet rows must include fleet rollout diagnostics",
                row.topology_row_id.clone(),
            );
            return;
        };
        if fleet.exact_build_identity_ref != row.exact_build.exact_build_identity_ref {
            self.push(
                "install_diagnostics.fleet.exact_build_mismatch",
                "fleet inventory exact-build ref must match the row exact-build ref",
                row.topology_row_id.clone(),
            );
        }
        if fleet.managed_package_report_ref.trim().is_empty() {
            self.push(
                "install_diagnostics.fleet.report_missing",
                "fleet diagnostics must name a managed-package report ref",
                row.topology_row_id.clone(),
            );
        }
        if fleet.inventory_probe_ref.trim().is_empty() || !fleet.inventory_probe_available {
            self.push(
                "install_diagnostics.fleet.inventory_probe_missing",
                "fleet diagnostics must expose an inventory probe without GUI launch",
                row.topology_row_id.clone(),
            );
        }
        if fleet.policy_root_refs.is_empty() || row.policy_root_refs.is_empty() {
            self.push(
                "install_diagnostics.fleet.policy_roots_missing",
                "fleet diagnostics must disclose policy roots",
                row.topology_row_id.clone(),
            );
        }
        if fleet.rollback_target_class.trim().is_empty()
            || fleet.rollback_target_class != row.rollback_target_class
        {
            self.push(
                "install_diagnostics.fleet.rollback_target_mismatch",
                "fleet rollback target must match the diagnostics row",
                row.topology_row_id.clone(),
            );
        }
        if !fleet.deprovision_preserves_local_work {
            self.push(
                "install_diagnostics.fleet.deprovision_preservation_missing",
                "fleet diagnostics must state that deprovision preserves local work",
                row.topology_row_id.clone(),
            );
        }
        let evidence: BTreeSet<FleetRolloutEvidenceClass> =
            fleet.evidence.iter().copied().collect();
        for required in [
            FleetRolloutEvidenceClass::RingAssignment,
            FleetRolloutEvidenceClass::ExactBuildInventory,
            FleetRolloutEvidenceClass::ManagedPackageReport,
            FleetRolloutEvidenceClass::RollbackTarget,
            FleetRolloutEvidenceClass::VerificationStatus,
        ] {
            if !evidence.contains(&required) {
                self.push(
                    "install_diagnostics.fleet.evidence_missing",
                    format!("fleet diagnostics must include {required:?} evidence"),
                    row.topology_row_id.clone(),
                );
            }
        }
    }

    fn validate_required_coverage(&mut self) {
        for mode in [
            InstallModeClass::Portable,
            InstallModeClass::SideBySidePreview,
            InstallModeClass::ManagedDeployed,
        ] {
            if !self.coverage.install_modes.contains(&mode) {
                self.push(
                    "install_diagnostics.coverage.install_mode_missing",
                    format!("required diagnostics install mode is not covered: {mode:?}"),
                    self.packet.packet_id.clone(),
                );
            }
        }
        for surface in [
            TopologySurfaceClass::About,
            TopologySurfaceClass::Diagnostics,
            TopologySurfaceClass::Cli,
            TopologySurfaceClass::SupportExport,
        ] {
            if !self.coverage.surfaces.contains(&surface) {
                self.push(
                    "install_diagnostics.coverage.surface_missing",
                    format!("required diagnostics surface is not covered: {surface:?}"),
                    self.packet.packet_id.clone(),
                );
            }
        }
        if self.coverage.fleet_rollout_rings.is_empty() {
            self.push(
                "install_diagnostics.coverage.fleet_ring_missing",
                "packet must cover at least one fleet rollout ring",
                self.packet.packet_id.clone(),
            );
        }
    }

    fn validate_cross_row_state_boundaries(&mut self) {
        for row in &self.packet.rows {
            for peer_id in &row.side_by_side_peer_row_ids {
                let Some(peer) = self.packet.row_by_topology_id(peer_id) else {
                    self.push(
                        "install_diagnostics.side_by_side.peer_unknown",
                        "side-by-side peer row id must resolve",
                        format!("{} -> {peer_id}", row.topology_row_id),
                    );
                    continue;
                };
                if !roots_disjoint_or_shared_read_only(row, peer) {
                    self.push(
                        "install_diagnostics.side_by_side.state_roots_overlap",
                        "side-by-side peers must not share mutable durable state roots",
                        format!("{} -> {peer_id}", row.topology_row_id),
                    );
                }
            }
        }

        for (left_idx, left) in self.packet.rows.iter().enumerate() {
            for right in self.packet.rows.iter().skip(left_idx + 1) {
                if left.channel_class == right.channel_class {
                    continue;
                }
                if !roots_disjoint_or_shared_read_only(left, right) {
                    self.push(
                        "install_diagnostics.cross_channel.state_roots_overlap",
                        "diagnostics rows for different channels must not share mutable state roots",
                        format!("{} <-> {}", left.topology_row_id, right.topology_row_id),
                    );
                }
            }
        }
    }
}

fn roots_disjoint_or_shared_read_only(
    left: &InstallDiagnosticRow,
    right: &InstallDiagnosticRow,
) -> bool {
    let right_roots: BTreeMap<&str, &StateRootDiagnostic> = right
        .durable_state_roots
        .iter()
        .map(|root| (root.state_root_ref.as_str(), root))
        .collect();
    for left_root in &left.durable_state_roots {
        let Some(right_root) = right_roots.get(left_root.state_root_ref.as_str()) else {
            continue;
        };
        if !left_root.is_shared_read_only() || !right_root.is_shared_read_only() {
            return false;
        }
    }
    true
}
