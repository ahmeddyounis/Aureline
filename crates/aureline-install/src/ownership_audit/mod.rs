//! Desktop-entry ownership audit for deep-link, protocol-handler, and
//! file-association handoff surfaces across side-by-side channels and
//! portable installs.
//!
//! The audit consumes the already-seeded install-topology alpha packet
//! ([`crate::topology`]) by reference and projects one bounded packet that
//! answers three reviewer questions on every OS-level handoff surface:
//!
//! 1. **Which build owns this surface?** A typed [`OwnerVerdictClass`]
//!    pins the selected owner, candidate-only registration, an admin or
//!    managed-fleet owner, or a displaced owner that requires a
//!    diagnostic. Side-by-side rows must name the disclosure tokens that
//!    keep last-writer-wins out of the lane.
//! 2. **What happens when a side-by-side install coexists?** Each row
//!    enumerates the coexisting channels and the
//!    [`SideBySideDisclosureClass`] tokens (per-channel suffixed scheme,
//!    explicit selection, channel-owner summary in install review) so a
//!    reviewer can read which mitigation the surface uses.
//! 3. **Are deep links and file opens going through the same trust,
//!    preview, and scope checks as in-product invocation?** Each row
//!    that participates in route admission names the
//!    [`DeepLinkRouteCheckClass`] tokens the validator applies and
//!    asserts the in-product invocation path uses the same family.
//!
//! The packet does not implement an installer, register OS entry points,
//! or mutate desktop state. It is a typed projection used by the install
//! topology consumer surfaces (About, update, diagnostics, install
//! review, CLI, support export) and the shell ownership-audit headless
//! inspector.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::topology::{ChannelClass, InstallModeClass, PlatformClass};

/// Schema version exported with every ownership-audit record.
pub const OWNERSHIP_AUDIT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`OwnershipAuditPacket`].
pub const OWNERSHIP_AUDIT_PACKET_RECORD_KIND: &str = "install_ownership_audit_packet";

/// Stable record-kind tag for [`OwnershipAuditSupportExport`].
pub const OWNERSHIP_AUDIT_SUPPORT_EXPORT_RECORD_KIND: &str =
    "install_ownership_audit_support_export";

/// Stable shared contract ref consumed by every ownership-audit row.
pub const OWNERSHIP_AUDIT_SHARED_CONTRACT_REF: &str = "install:ownership_audit:v1";

/// OS-level handoff surface that exposes ownership to the desktop shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffSurfaceClass {
    /// File association registered against a file extension or content type.
    FileAssociation,
    /// Protocol or deep-link scheme handler.
    ProtocolHandler,
    /// Default-browser callback returning to the application.
    DefaultBrowserCallback,
    /// Deep-link intent dispatched through the application's entry-flow.
    DeepLinkIntent,
    /// Recent-item or jump-list registration shown by the OS shell.
    RecentItemRegistration,
    /// Default-open behavior for a file or scheme already registered.
    DefaultOpenBehavior,
}

impl HandoffSurfaceClass {
    /// Stable schema token for the surface class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileAssociation => "file_association",
            Self::ProtocolHandler => "protocol_handler",
            Self::DefaultBrowserCallback => "default_browser_callback",
            Self::DeepLinkIntent => "deep_link_intent",
            Self::RecentItemRegistration => "recent_item_registration",
            Self::DefaultOpenBehavior => "default_open_behavior",
        }
    }

    /// True when admission of an OS handoff to this surface must run the
    /// same trust, preview, and scope checks an in-product invocation
    /// uses. Recent-item registration does not dispatch and is exempt.
    pub const fn participates_in_route_admission(self) -> bool {
        !matches!(self, Self::RecentItemRegistration)
    }
}

/// Channel layout the audit row applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelLayoutClass {
    /// Stable channel installed alone.
    StableOnly,
    /// Stable and Preview installed side-by-side on the same host.
    StableAndPreviewSideBySide,
    /// Stable installed beside a Portable channel on the same host.
    StableAndPortableBeside,
    /// Stable installed beside a Managed deployment on the same host.
    StableAndManagedBeside,
    /// Portable channel installed alone (no installed channel present).
    PortableOnly,
    /// Air-gapped bundle deployment with no installed or portable peer.
    AirGappedBundleOnly,
    /// Stable, Preview, and Portable installed together on the same host.
    ThreeChannelMixed,
}

impl ChannelLayoutClass {
    /// Stable schema token for the layout.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableOnly => "stable_only",
            Self::StableAndPreviewSideBySide => "stable_and_preview_side_by_side",
            Self::StableAndPortableBeside => "stable_and_portable_beside",
            Self::StableAndManagedBeside => "stable_and_managed_beside",
            Self::PortableOnly => "portable_only",
            Self::AirGappedBundleOnly => "air_gapped_bundle_only",
            Self::ThreeChannelMixed => "three_channel_mixed",
        }
    }

    /// True when more than one channel coexists in the layout.
    pub const fn is_coexisting(self) -> bool {
        matches!(
            self,
            Self::StableAndPreviewSideBySide
                | Self::StableAndPortableBeside
                | Self::StableAndManagedBeside
                | Self::ThreeChannelMixed
        )
    }
}

/// Verdict the audit returns for a single handoff surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerVerdictClass {
    /// A specific channel is the user- or admin-selected owner of the surface.
    SelectedOwner,
    /// The build is registered as a candidate handler but is not the selected owner.
    CandidateOnly,
    /// The build does not register against the surface at all.
    NotRegistered,
    /// Admin policy pins the surface owner.
    AdminPolicyOwned,
    /// Managed fleet ring pins the surface owner.
    ManagedFleetOwned,
    /// Another channel displaced the expected owner; a diagnostic is required.
    DisplacedOwner,
}

impl OwnerVerdictClass {
    /// Stable schema token for the verdict.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedOwner => "selected_owner",
            Self::CandidateOnly => "candidate_only",
            Self::NotRegistered => "not_registered",
            Self::AdminPolicyOwned => "admin_policy_owned",
            Self::ManagedFleetOwned => "managed_fleet_owned",
            Self::DisplacedOwner => "displaced_owner",
        }
    }
}

/// Portable-mode ownership claim disclosed by the audit row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortableOwnershipClaim {
    /// Portable build never registers a host-global handler for the surface.
    NeverClaimsHostGlobalOwnership,
    /// Portable build does not apply (the audit row is not portable).
    NotApplicable,
}

impl PortableOwnershipClaim {
    /// Stable schema token for the claim.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NeverClaimsHostGlobalOwnership => "never_claims_host_global_ownership",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Managed-install ownership claim disclosed by the audit row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedOwnershipClaim {
    /// Admin policy directly owns the handler row.
    AdminPolicyOwnsHandler,
    /// Managed fleet ring owns the handler row.
    ManagedRingOwnsHandler,
    /// User surface shows the managed owner but cannot override it.
    UserVisibleNotOverrideable,
    /// Managed claim does not apply (the audit row is not managed).
    NotApplicable,
}

impl ManagedOwnershipClaim {
    /// Stable schema token for the claim.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdminPolicyOwnsHandler => "admin_policy_owns_handler",
            Self::ManagedRingOwnsHandler => "managed_ring_owns_handler",
            Self::UserVisibleNotOverrideable => "user_visible_not_overrideable",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Disclosure token surfaced on rows where channels coexist.
///
/// These tokens are the closed mitigations that keep side-by-side
/// installs from collapsing into last-writer-wins behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideBySideDisclosureClass {
    /// Each channel registers a suffixed scheme distinct from peers.
    PerChannelSuffixedScheme,
    /// Shared default-open behavior requires explicit user or admin selection.
    UserOrAdminSelectedDefaultNeverLastWriterWins,
    /// Channel-owner summary appears in install review before commit.
    ChannelOwnerSummaryInReview,
    /// Handler-owner change is staged through preview-before-commit.
    HandlerOwnerChangePreviewBeforeCommit,
    /// Portable build does not steal ownership from an installed channel.
    PortableDoesNotStealInstalledOwnership,
    /// Managed policy owner is shown to the user but is not overrideable.
    ManagedOwnerShownNotOverrideable,
    /// Disclosure does not apply (single-channel row).
    NotApplicable,
}

impl SideBySideDisclosureClass {
    /// Stable schema token for the disclosure class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PerChannelSuffixedScheme => "per_channel_suffixed_scheme",
            Self::UserOrAdminSelectedDefaultNeverLastWriterWins => {
                "user_or_admin_selected_default_never_last_writer_wins"
            }
            Self::ChannelOwnerSummaryInReview => "channel_owner_summary_in_review",
            Self::HandlerOwnerChangePreviewBeforeCommit => {
                "handler_owner_change_preview_before_commit"
            }
            Self::PortableDoesNotStealInstalledOwnership => {
                "portable_does_not_steal_installed_ownership"
            }
            Self::ManagedOwnerShownNotOverrideable => "managed_owner_shown_not_overrideable",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when the token is a real disclosure (not the placeholder
    /// `not_applicable`).
    pub const fn is_real_disclosure(self) -> bool {
        !matches!(self, Self::NotApplicable)
    }
}

/// Check applied by the deep-link route admission validator.
///
/// The tokens mirror the closed admission posture used by the live
/// deep-link validator in `aureline-shell` so the audit can claim
/// parity without re-deriving the rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkRouteCheckClass {
    /// Origin trust class check (no `unknown_untrusted` admission).
    OriginTrust,
    /// Reviewed-sheet preview required for boundary-raising routes.
    ReviewedSheetPreview,
    /// Target / workspace scope check before dispatch.
    TargetScope,
    /// Single-use replay posture (denies double-consumption).
    SingleUseReplay,
    /// Handler ownership verification before dispatch.
    HandlerOwnershipVerification,
}

impl DeepLinkRouteCheckClass {
    /// Stable schema token for the check.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OriginTrust => "origin_trust",
            Self::ReviewedSheetPreview => "reviewed_sheet_preview",
            Self::TargetScope => "target_scope",
            Self::SingleUseReplay => "single_use_replay",
            Self::HandlerOwnershipVerification => "handler_ownership_verification",
        }
    }
}

/// Audit row covering one (channel-layout, handoff-surface) pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipAuditRow {
    /// Stable audit row id.
    pub audit_row_id: String,
    /// Topology row this audit row references.
    pub topology_row_ref: String,
    /// Platform class for the row.
    pub platform_class: PlatformClass,
    /// Channel layout the row applies to.
    pub channel_layout: ChannelLayoutClass,
    /// Install mode class of the build that owns (or could own) the surface.
    pub owning_install_mode_class: InstallModeClass,
    /// Channel class of the build that owns (or could own) the surface.
    pub owning_channel_class: ChannelClass,
    /// Handoff surface this row covers.
    pub handoff_surface_class: HandoffSurfaceClass,
    /// Verdict for the row.
    pub owner_verdict: OwnerVerdictClass,
    /// Channel selected as owner (when applicable).
    pub selected_owner_channel: Option<ChannelClass>,
    /// Channels registered as candidate handlers on this surface.
    pub candidate_owner_channels: Vec<ChannelClass>,
    /// Other channels present in the layout (excludes the owning channel).
    pub coexisting_channels: Vec<ChannelClass>,
    /// Portable ownership claim disclosed by the row.
    pub portable_claim: PortableOwnershipClaim,
    /// Managed ownership claim disclosed by the row.
    pub managed_claim: ManagedOwnershipClaim,
    /// Side-by-side disclosure tokens (mitigations against last-writer-wins).
    pub side_by_side_disclosure: Vec<SideBySideDisclosureClass>,
    /// True when silent ownership steal is blocked for this row.
    pub silent_steal_blocked: bool,
    /// Deep-link route checks applied to this surface when it dispatches.
    pub deep_link_route_checks: Vec<DeepLinkRouteCheckClass>,
    /// True when the in-product invocation runs the same checks listed
    /// in [`Self::deep_link_route_checks`].
    pub in_product_invocation_uses_same_checks: bool,
    /// Optional ref to a topology stale-handler diagnostic that covers
    /// the displaced-owner case for this row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub displaced_owner_diagnostic_ref: Option<String>,
    /// Reviewer-facing rationale for the row.
    pub rationale: String,
}

impl OwnershipAuditRow {
    /// True when the row covers a portable install mode.
    pub fn is_portable(&self) -> bool {
        matches!(self.owning_install_mode_class, InstallModeClass::Portable)
    }

    /// True when the row covers a managed deployment.
    pub fn is_managed(&self) -> bool {
        matches!(
            self.owning_install_mode_class,
            InstallModeClass::ManagedDeployed
        )
    }

    /// True when the row covers an offline / air-gapped bundle install.
    pub fn is_offline_bundle(&self) -> bool {
        matches!(
            self.owning_install_mode_class,
            InstallModeClass::OfflineBundle
        )
    }
}

/// Top-level ownership-audit packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipAuditPacket {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every audit row.
    pub shared_contract_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Ref to the install-topology alpha packet the audit projects from.
    pub topology_packet_ref: String,
    /// Audit rows.
    pub rows: Vec<OwnershipAuditRow>,
}

impl OwnershipAuditPacket {
    /// Returns the row count.
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Finds a row by id.
    pub fn row_by_id(&self, audit_row_id: &str) -> Option<&OwnershipAuditRow> {
        self.rows
            .iter()
            .find(|row| row.audit_row_id == audit_row_id)
    }

    /// Validates the audit packet.
    pub fn validate(&self) -> OwnershipAuditValidationReport {
        let mut validator = OwnershipAuditValidator::new(self);
        validator.validate();
        validator.finish()
    }

    /// Projects the audit into a surface row list.
    pub fn surface_projection(&self) -> OwnershipAuditSurfaceProjection {
        OwnershipAuditSurfaceProjection {
            packet_id: self.packet_id.clone(),
            shared_contract_ref: self.shared_contract_ref.clone(),
            rows: self
                .rows
                .iter()
                .map(OwnershipAuditSurfaceRow::from)
                .collect(),
        }
    }

    /// Returns a metadata-safe support-export projection.
    pub fn support_export_projection(&self) -> OwnershipAuditSupportExport {
        OwnershipAuditSupportExport {
            record_kind: OWNERSHIP_AUDIT_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: OWNERSHIP_AUDIT_SCHEMA_VERSION,
            shared_contract_ref: self.shared_contract_ref.clone(),
            packet_id: self.packet_id.clone(),
            projection: self.surface_projection(),
            redaction_class: "metadata_only_no_paths_or_secrets".to_string(),
        }
    }
}

/// Reviewer-facing row projected onto a consuming surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipAuditSurfaceRow {
    /// Stable audit row id.
    pub audit_row_id: String,
    /// Topology row ref.
    pub topology_row_ref: String,
    /// Platform class.
    pub platform_class: PlatformClass,
    /// Channel layout class.
    pub channel_layout: ChannelLayoutClass,
    /// Install mode class.
    pub owning_install_mode_class: InstallModeClass,
    /// Channel class.
    pub owning_channel_class: ChannelClass,
    /// Handoff surface class.
    pub handoff_surface_class: HandoffSurfaceClass,
    /// Verdict.
    pub owner_verdict: OwnerVerdictClass,
    /// Selected owner channel when applicable.
    pub selected_owner_channel: Option<ChannelClass>,
    /// Candidate owner channels.
    pub candidate_owner_channels: Vec<ChannelClass>,
    /// Coexisting channels.
    pub coexisting_channels: Vec<ChannelClass>,
    /// Portable claim.
    pub portable_claim: PortableOwnershipClaim,
    /// Managed claim.
    pub managed_claim: ManagedOwnershipClaim,
    /// Side-by-side disclosure tokens.
    pub side_by_side_disclosure: Vec<SideBySideDisclosureClass>,
    /// True when silent steal is blocked.
    pub silent_steal_blocked: bool,
    /// Deep-link route checks.
    pub deep_link_route_checks: Vec<DeepLinkRouteCheckClass>,
    /// True when in-product invocation uses the same checks.
    pub in_product_invocation_uses_same_checks: bool,
    /// Displaced-owner diagnostic ref when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub displaced_owner_diagnostic_ref: Option<String>,
    /// Reviewer-facing rationale.
    pub rationale: String,
}

impl From<&OwnershipAuditRow> for OwnershipAuditSurfaceRow {
    fn from(row: &OwnershipAuditRow) -> Self {
        Self {
            audit_row_id: row.audit_row_id.clone(),
            topology_row_ref: row.topology_row_ref.clone(),
            platform_class: row.platform_class,
            channel_layout: row.channel_layout,
            owning_install_mode_class: row.owning_install_mode_class,
            owning_channel_class: row.owning_channel_class,
            handoff_surface_class: row.handoff_surface_class,
            owner_verdict: row.owner_verdict,
            selected_owner_channel: row.selected_owner_channel,
            candidate_owner_channels: row.candidate_owner_channels.clone(),
            coexisting_channels: row.coexisting_channels.clone(),
            portable_claim: row.portable_claim,
            managed_claim: row.managed_claim,
            side_by_side_disclosure: row.side_by_side_disclosure.clone(),
            silent_steal_blocked: row.silent_steal_blocked,
            deep_link_route_checks: row.deep_link_route_checks.clone(),
            in_product_invocation_uses_same_checks: row.in_product_invocation_uses_same_checks,
            displaced_owner_diagnostic_ref: row.displaced_owner_diagnostic_ref.clone(),
            rationale: row.rationale.clone(),
        }
    }
}

/// Projection rendered on a consuming surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipAuditSurfaceProjection {
    /// Packet id.
    pub packet_id: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Audit rows.
    pub rows: Vec<OwnershipAuditSurfaceRow>,
}

/// Metadata-safe support-export wrapper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipAuditSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Packet id.
    pub packet_id: String,
    /// Audit surface projection.
    pub projection: OwnershipAuditSurfaceProjection,
    /// Redaction class for the projection.
    pub redaction_class: String,
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipAuditValidationFinding {
    /// Stable check id.
    pub check_id: String,
    /// Human-readable finding message.
    pub message: String,
    /// Row or packet ref associated with the finding.
    pub ref_id: String,
}

/// Validation report for an ownership-audit packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipAuditValidationReport {
    /// True when validation found no errors.
    pub passed: bool,
    /// Findings raised by the validator.
    pub findings: Vec<OwnershipAuditValidationFinding>,
    /// Coverage gathered while validating.
    pub coverage: OwnershipAuditCoverage,
}

/// Coverage gathered from the rows.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipAuditCoverage {
    /// Channel layouts covered.
    pub channel_layouts: BTreeSet<ChannelLayoutClass>,
    /// Handoff surfaces covered.
    pub handoff_surfaces: BTreeSet<HandoffSurfaceClass>,
    /// Install modes covered.
    pub install_modes: BTreeSet<InstallModeClass>,
    /// Deep-link route checks covered.
    pub deep_link_route_checks: BTreeSet<DeepLinkRouteCheckClass>,
    /// True when at least one row exposes a displaced-owner diagnostic ref.
    pub displaced_owner_diagnostic_covered: bool,
}

struct OwnershipAuditValidator<'a> {
    packet: &'a OwnershipAuditPacket,
    findings: Vec<OwnershipAuditValidationFinding>,
    coverage: OwnershipAuditCoverage,
    seen_ids: BTreeSet<String>,
}

impl<'a> OwnershipAuditValidator<'a> {
    fn new(packet: &'a OwnershipAuditPacket) -> Self {
        Self {
            packet,
            findings: Vec::new(),
            coverage: OwnershipAuditCoverage::default(),
            seen_ids: BTreeSet::new(),
        }
    }

    fn validate(&mut self) {
        self.validate_header();
        for row in &self.packet.rows {
            self.validate_row(row);
        }
        self.validate_required_coverage();
    }

    fn finish(self) -> OwnershipAuditValidationReport {
        OwnershipAuditValidationReport {
            passed: self.findings.is_empty(),
            findings: self.findings,
            coverage: self.coverage,
        }
    }

    fn push(&mut self, check_id: &str, message: impl Into<String>, ref_id: impl Into<String>) {
        self.findings.push(OwnershipAuditValidationFinding {
            check_id: check_id.to_string(),
            message: message.into(),
            ref_id: ref_id.into(),
        });
    }

    fn validate_header(&mut self) {
        if self.packet.record_kind != OWNERSHIP_AUDIT_PACKET_RECORD_KIND {
            self.push(
                "ownership_audit.packet.record_kind",
                "packet record_kind is not install_ownership_audit_packet",
                &self.packet.packet_id,
            );
        }
        if self.packet.schema_version != OWNERSHIP_AUDIT_SCHEMA_VERSION {
            self.push(
                "ownership_audit.packet.schema_version",
                "packet schema_version is unsupported",
                &self.packet.packet_id,
            );
        }
        if self.packet.shared_contract_ref != OWNERSHIP_AUDIT_SHARED_CONTRACT_REF {
            self.push(
                "ownership_audit.packet.shared_contract_ref",
                "packet shared_contract_ref is not install:ownership_audit:v1",
                &self.packet.packet_id,
            );
        }
        if self.packet.rows.is_empty() {
            self.push(
                "ownership_audit.packet.rows_empty",
                "packet must contain at least one audit row",
                &self.packet.packet_id,
            );
        }
        if self.packet.topology_packet_ref.trim().is_empty() {
            self.push(
                "ownership_audit.packet.topology_ref_missing",
                "packet must name a topology_packet_ref",
                &self.packet.packet_id,
            );
        }
    }

    fn validate_row(&mut self, row: &OwnershipAuditRow) {
        if row.audit_row_id.trim().is_empty() {
            self.push(
                "ownership_audit.row.id_missing",
                "audit row id must not be empty",
                row.topology_row_ref.clone(),
            );
        }
        if !self.seen_ids.insert(row.audit_row_id.clone()) {
            self.push(
                "ownership_audit.row.id_duplicate",
                "audit row id must be unique",
                row.audit_row_id.clone(),
            );
        }
        if row.topology_row_ref.trim().is_empty() {
            self.push(
                "ownership_audit.row.topology_ref_missing",
                "row must name a topology_row_ref",
                row.audit_row_id.clone(),
            );
        }
        if row.rationale.trim().is_empty() {
            self.push(
                "ownership_audit.row.rationale_missing",
                "row must include a reviewer-facing rationale",
                row.audit_row_id.clone(),
            );
        }

        self.coverage.channel_layouts.insert(row.channel_layout);
        self.coverage
            .handoff_surfaces
            .insert(row.handoff_surface_class);
        self.coverage
            .install_modes
            .insert(row.owning_install_mode_class);
        for check in &row.deep_link_route_checks {
            self.coverage.deep_link_route_checks.insert(*check);
        }
        if row.displaced_owner_diagnostic_ref.is_some() {
            self.coverage.displaced_owner_diagnostic_covered = true;
        }

        if row.coexisting_channels.contains(&row.owning_channel_class) {
            self.push(
                "ownership_audit.row.coexisting_includes_owner",
                "coexisting_channels must not list the owning channel",
                row.audit_row_id.clone(),
            );
        }

        if row.channel_layout.is_coexisting() && row.coexisting_channels.is_empty() {
            self.push(
                "ownership_audit.row.coexisting_channels_missing",
                "coexisting layout must list at least one coexisting channel",
                row.audit_row_id.clone(),
            );
        }
        if !row.channel_layout.is_coexisting() && !row.coexisting_channels.is_empty() {
            self.push(
                "ownership_audit.row.coexisting_channels_unexpected",
                "single-channel layout must not list coexisting channels",
                row.audit_row_id.clone(),
            );
        }

        if row.channel_layout.is_coexisting()
            && !row
                .side_by_side_disclosure
                .iter()
                .any(|tok| tok.is_real_disclosure())
        {
            self.push(
                "ownership_audit.row.side_by_side_disclosure_missing",
                "coexisting layout must name at least one side-by-side disclosure token",
                row.audit_row_id.clone(),
            );
        }
        if row.channel_layout.is_coexisting() && !row.silent_steal_blocked {
            self.push(
                "ownership_audit.row.silent_steal_not_blocked",
                "coexisting layout must block silent ownership steal",
                row.audit_row_id.clone(),
            );
        }

        if row.handoff_surface_class.participates_in_route_admission() {
            if row.deep_link_route_checks.is_empty() {
                self.push(
                    "ownership_audit.row.route_checks_missing",
                    "row participating in route admission must list at least one deep-link route check",
                    row.audit_row_id.clone(),
                );
            }
            if !row.in_product_invocation_uses_same_checks {
                self.push(
                    "ownership_audit.row.in_product_check_parity_missing",
                    "deep links and file opens must run the same trust/preview/scope checks as in-product invocation",
                    row.audit_row_id.clone(),
                );
            }
        }

        if row.is_portable() {
            if row.portable_claim != PortableOwnershipClaim::NeverClaimsHostGlobalOwnership {
                self.push(
                    "ownership_audit.row.portable_claim_not_closed",
                    "portable row must disclose never_claims_host_global_ownership",
                    row.audit_row_id.clone(),
                );
            }
            if row.owner_verdict != OwnerVerdictClass::NotRegistered {
                self.push(
                    "ownership_audit.row.portable_owner_verdict",
                    "portable row owner_verdict must be not_registered",
                    row.audit_row_id.clone(),
                );
            }
            if !row.silent_steal_blocked {
                self.push(
                    "ownership_audit.row.portable_silent_steal",
                    "portable row must block silent ownership steal",
                    row.audit_row_id.clone(),
                );
            }
        } else if row.portable_claim != PortableOwnershipClaim::NotApplicable {
            self.push(
                "ownership_audit.row.portable_claim_unexpected",
                "non-portable row must mark portable_claim as not_applicable",
                row.audit_row_id.clone(),
            );
        }

        if row.is_managed() {
            if row.managed_claim == ManagedOwnershipClaim::NotApplicable {
                self.push(
                    "ownership_audit.row.managed_claim_missing",
                    "managed row must disclose a managed ownership claim",
                    row.audit_row_id.clone(),
                );
            }
            if !matches!(
                row.owner_verdict,
                OwnerVerdictClass::AdminPolicyOwned
                    | OwnerVerdictClass::ManagedFleetOwned
                    | OwnerVerdictClass::NotRegistered
            ) {
                self.push(
                    "ownership_audit.row.managed_owner_verdict",
                    "managed row owner_verdict must be admin_policy_owned, managed_fleet_owned, or not_registered",
                    row.audit_row_id.clone(),
                );
            }
        } else if row.managed_claim != ManagedOwnershipClaim::NotApplicable {
            self.push(
                "ownership_audit.row.managed_claim_unexpected",
                "non-managed row must mark managed_claim as not_applicable",
                row.audit_row_id.clone(),
            );
        }

        if row.is_offline_bundle() && row.owner_verdict != OwnerVerdictClass::NotRegistered {
            self.push(
                "ownership_audit.row.offline_bundle_owner_verdict",
                "offline bundle row owner_verdict must be not_registered",
                row.audit_row_id.clone(),
            );
        }

        if row.owner_verdict == OwnerVerdictClass::DisplacedOwner
            && row.displaced_owner_diagnostic_ref.is_none()
        {
            self.push(
                "ownership_audit.row.displaced_owner_diagnostic_missing",
                "displaced_owner row must reference a topology stale-handler diagnostic",
                row.audit_row_id.clone(),
            );
        }

        if row.owner_verdict == OwnerVerdictClass::SelectedOwner
            && row.selected_owner_channel.is_none()
        {
            self.push(
                "ownership_audit.row.selected_owner_channel_missing",
                "selected_owner row must name selected_owner_channel",
                row.audit_row_id.clone(),
            );
        }
        if matches!(
            row.owner_verdict,
            OwnerVerdictClass::NotRegistered | OwnerVerdictClass::CandidateOnly
        ) && row.selected_owner_channel.is_some()
        {
            self.push(
                "ownership_audit.row.selected_owner_channel_unexpected",
                "not_registered or candidate_only row must not name selected_owner_channel",
                row.audit_row_id.clone(),
            );
        }
    }

    fn validate_required_coverage(&mut self) {
        for required in [
            ChannelLayoutClass::StableAndPreviewSideBySide,
            ChannelLayoutClass::StableAndPortableBeside,
            ChannelLayoutClass::StableAndManagedBeside,
        ] {
            if !self.coverage.channel_layouts.contains(&required) {
                self.push(
                    "ownership_audit.coverage.channel_layout_missing",
                    format!("required channel layout is not covered: {required:?}"),
                    self.packet.packet_id.clone(),
                );
            }
        }
        for required in [
            HandoffSurfaceClass::FileAssociation,
            HandoffSurfaceClass::ProtocolHandler,
            HandoffSurfaceClass::DefaultBrowserCallback,
            HandoffSurfaceClass::DeepLinkIntent,
        ] {
            if !self.coverage.handoff_surfaces.contains(&required) {
                self.push(
                    "ownership_audit.coverage.handoff_surface_missing",
                    format!("required handoff surface is not covered: {required:?}"),
                    self.packet.packet_id.clone(),
                );
            }
        }
        for required in [
            InstallModeClass::Portable,
            InstallModeClass::ManagedDeployed,
            InstallModeClass::SideBySidePreview,
        ] {
            if !self.coverage.install_modes.contains(&required) {
                self.push(
                    "ownership_audit.coverage.install_mode_missing",
                    format!("required install mode is not covered: {required:?}"),
                    self.packet.packet_id.clone(),
                );
            }
        }
        for required in [
            DeepLinkRouteCheckClass::OriginTrust,
            DeepLinkRouteCheckClass::ReviewedSheetPreview,
            DeepLinkRouteCheckClass::TargetScope,
        ] {
            if !self.coverage.deep_link_route_checks.contains(&required) {
                self.push(
                    "ownership_audit.coverage.deep_link_check_missing",
                    format!("required deep-link route check is not covered: {required:?}"),
                    self.packet.packet_id.clone(),
                );
            }
        }
        if !self.coverage.displaced_owner_diagnostic_covered {
            self.push(
                "ownership_audit.coverage.displaced_owner_diagnostic_missing",
                "audit must cover at least one displaced-owner diagnostic row",
                self.packet.packet_id.clone(),
            );
        }
    }
}
