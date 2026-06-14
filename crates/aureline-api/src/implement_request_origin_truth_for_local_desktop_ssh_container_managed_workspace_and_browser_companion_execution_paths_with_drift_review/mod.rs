//! Request-origin truth and rerun drift-review qualification records.
//!
//! This module owns the typed records that make execution origin a first-class
//! fact everywhere a request can run. Each resolved-origin row keeps the
//! execution path (local desktop, SSH, container, managed workspace, or browser
//! companion), the canonical origin lane, the opaque target identity, the trust
//! boundary, and the drift state inspectable so `localhost`, container service
//! names, and private DNS never silently mean the same thing across lanes. The
//! companion rerun-review sheets distinguish *rerun exactly* from *rerun with
//! current context* and enumerate every origin change before dispatch, so a
//! saved request or rerun that would resolve through a different host, lane, or
//! trust boundary than before never slips past a green send button.
//!
//! These records reuse the canonical matrix vocabulary
//! ([`RequestOriginKind`], [`RequestOriginDriftState`], [`RetentionMode`]) and
//! reference the
//! [`freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix`](crate::freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix)
//! packet as a verified upstream truth rather than minting a local synonym set.
//! The finer [`OriginExecutionPath`] adds the explicit SSH and local-desktop
//! distinction the request-origin lane requires while mapping one-to-one onto
//! the frozen [`RequestOriginKind`] lanes.
//!
//! Raw endpoint URLs, raw secrets, raw request bodies, raw headers, and raw
//! cookie or token values do not belong in these records. Origins carry opaque,
//! non-secret target-identity labels, closed posture vocabularies, and
//! reviewable summaries. Managed and browser-companion origins never inherit
//! desktop-local trust or naming; silent retargeting between origins on reopen
//! or rerun is always blocked behind an acknowledgement; and rerun compare UX
//! never widens request-history retention toward unsafe body or header capture.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix::{
    RequestOriginDriftState, RequestOriginKind, RetentionMode,
    API_MATRIX_QUALIFICATION_RECORD_KIND,
};

/// Supported schema version for request-origin truth qualification packets.
pub const ORIGIN_TRUTH_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`OriginTruthQualificationPacket`].
pub const ORIGIN_TRUTH_QUALIFICATION_RECORD_KIND: &str =
    "implement_request_origin_truth_for_local_desktop_ssh_container_managed_workspace_and_browser_companion_execution_paths_with_drift_review";

/// Repo-relative path to the checked-in request-origin truth packet.
pub const ORIGIN_TRUTH_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/data/m5/implement-request-origin-truth-for-local-desktop-ssh-container-managed-workspace-and-browser-companion-execution-paths-with-drift-review.json";

/// Embedded checked-in packet JSON.
pub const ORIGIN_TRUTH_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/m5/implement-request-origin-truth-for-local-desktop-ssh-container-managed-workspace-and-browser-companion-execution-paths-with-drift-review.json"
));

/// Qualification label shown on promoted origin-truth surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginTruthQualificationLabel {
    /// Surface has current proof and may be called stable for its declared scope.
    Stable,
    /// Surface is visible but below stable.
    Preview,
    /// Surface is an experiment or internal lab.
    Labs,
    /// Surface may inspect metadata but must not execute or export live data.
    InspectOnly,
    /// Surface may import or view captured files only.
    ImportOnly,
}

impl OriginTruthQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Origin-truth consumer surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginTruthSurfaceKind {
    /// Request composer where the active origin is shown before send.
    RequestComposer,
    /// Rerun-review sheet shown before a saved request or rerun is dispatched.
    RerunReviewSheet,
    /// Request/collection list showing each request's origin class.
    RequestList,
    /// Browser-companion request surface that can drift from desktop-local state.
    BrowserCompanionSurface,
    /// CLI or headless request execution output.
    CliHeadlessOutput,
    /// Support-export bundle carrying origin truth.
    SupportExport,
    /// Help/About surface describing the origin and drift-review contract.
    HelpAbout,
}

/// Fine-grained execution path a request resolves through.
///
/// This vocabulary adds the explicit SSH and local-desktop distinction the
/// request-origin lane requires; it maps one-to-one onto the frozen
/// [`RequestOriginKind`] lanes via [`OriginExecutionPath::canonical_origin`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginExecutionPath {
    /// Local desktop process targeting localhost or a loopback service.
    LocalDesktop,
    /// Host reached over an SSH tunnel or remote shell session.
    Ssh,
    /// Container or compose service name resolved inside a runtime.
    Container,
    /// Managed-workspace or cloud-hosted execution target.
    ManagedWorkspace,
    /// Browser-companion runtime executing against page or private-DNS context.
    BrowserCompanion,
}

impl OriginExecutionPath {
    /// Returns the canonical [`RequestOriginKind`] lane this execution path
    /// resolves under, keeping the finer path aligned with the frozen matrix.
    pub const fn canonical_origin(self) -> RequestOriginKind {
        match self {
            Self::LocalDesktop => RequestOriginKind::LocalHost,
            Self::Ssh => RequestOriginKind::Remote,
            Self::Container => RequestOriginKind::Container,
            Self::ManagedWorkspace => RequestOriginKind::Managed,
            Self::BrowserCompanion => RequestOriginKind::BrowserCompanion,
        }
    }

    /// Returns the canonical trust boundary this execution path is scoped to.
    pub const fn canonical_trust_boundary(self) -> OriginTrustBoundaryClass {
        match self {
            Self::LocalDesktop => OriginTrustBoundaryClass::DesktopLocalTrust,
            Self::Ssh => OriginTrustBoundaryClass::RemoteHostTrust,
            Self::Container => OriginTrustBoundaryClass::ContainerScopedTrust,
            Self::ManagedWorkspace => OriginTrustBoundaryClass::ManagedTenantTrust,
            Self::BrowserCompanion => OriginTrustBoundaryClass::BrowserCompanionTrust,
        }
    }

    /// Returns true only for the local-desktop path; every other path is remote
    /// to the desktop and must never inherit desktop-local trust or naming.
    pub const fn may_inherit_local_trust(self) -> bool {
        matches!(self, Self::LocalDesktop)
    }
}

/// Trust boundary an execution path is scoped to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginTrustBoundaryClass {
    /// Desktop-local trust; only the local-desktop path may claim it.
    DesktopLocalTrust,
    /// Trust scoped to a remote host reached over the network or SSH.
    RemoteHostTrust,
    /// Trust scoped to a container or compose network.
    ContainerScopedTrust,
    /// Trust scoped to a managed-workspace tenant.
    ManagedTenantTrust,
    /// Trust scoped to a browser-companion runtime.
    BrowserCompanionTrust,
}

/// How a rerun re-resolves the request's execution origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunReviewMode {
    /// Re-dispatch against the exact origin and snapshot recorded last time.
    RerunExactly,
    /// Re-resolve the origin through the current environment and context, which
    /// can drift from the recorded target.
    RerunWithCurrentContext,
}

/// A single kind of origin change a rerun review can enumerate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginChangeKind {
    /// The resolved host identity changed.
    HostIdentityChanged,
    /// The origin lane changed (for example local to container).
    OriginLaneChanged,
    /// The trust boundary changed.
    TrustBoundaryChanged,
    /// The resolved port or container service changed.
    PortOrServiceChanged,
    /// A private-DNS name rebound to a different target.
    PrivateDnsRebound,
}

impl OriginChangeKind {
    /// Returns true when the change crosses a host, lane, or trust boundary and
    /// therefore must be acknowledged before dispatch.
    pub const fn crosses_boundary(self) -> bool {
        matches!(
            self,
            Self::HostIdentityChanged
                | Self::OriginLaneChanged
                | Self::TrustBoundaryChanged
                | Self::PrivateDnsRebound
        )
    }
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginTruthQualificationProof {
    /// Stable proof packet id.
    pub packet_id: String,
    /// Repo-relative proof artifact reference.
    pub packet_ref: String,
    /// Proof-index reference.
    pub proof_index_ref: String,
    /// UTC capture date.
    pub captured_at: String,
    /// Evidence artifact references.
    pub evidence_refs: Vec<String>,
}

/// Boolean guard set that keeps stable origin-truth surfaces honest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginTruthSurfaceGuardSet {
    /// The origin class is visible.
    pub origin_class_visible: bool,
    /// The target identity is visible.
    pub target_identity_visible: bool,
    /// Origin-changed warnings are visible when the origin drifted.
    pub origin_changed_warning_visible: bool,
    /// The trust boundary is visible.
    pub trust_boundary_visible: bool,
    /// Rerun-exactly and rerun-with-current-context are distinguished.
    pub rerun_modes_distinguished: bool,
    /// Origin changes are enumerated before dispatch.
    pub changes_enumerated_before_dispatch: bool,
    /// Origins never silently retarget on reopen or rerun.
    pub no_silent_retarget: bool,
    /// Managed and companion origins never inherit desktop-local trust.
    pub trust_isolated: bool,
    /// Local request context is preserved across review and rerun.
    pub local_request_context_preserved: bool,
    /// Compare and rerun UX never widens history toward unsafe body/header retention.
    pub no_unsafe_retention_for_compare: bool,
}

impl OriginTruthSurfaceGuardSet {
    /// Returns true when every required guard is present.
    pub const fn all_visible(&self) -> bool {
        self.origin_class_visible
            && self.target_identity_visible
            && self.origin_changed_warning_visible
            && self.trust_boundary_visible
            && self.rerun_modes_distinguished
            && self.changes_enumerated_before_dispatch
            && self.no_silent_retarget
            && self.trust_isolated
            && self.local_request_context_preserved
            && self.no_unsafe_retention_for_compare
    }
}

/// One governed origin-truth-consumer surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginTruthSurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: OriginTruthSurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: OriginTruthQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: OriginTruthQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<OriginTruthQualificationProof>,
    /// Visible guard set.
    pub guards: OriginTruthSurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// One resolved-origin truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedOriginRow {
    /// Stable origin id.
    pub origin_id: String,
    /// Owning surface ref.
    pub surface_ref: String,
    /// Fine-grained execution path.
    pub execution_path: OriginExecutionPath,
    /// Canonical origin lane (must equal the execution path's canonical lane).
    pub origin_kind: RequestOriginKind,
    /// Matrix origin row this truth reflects.
    pub matrix_origin_ref: String,
    /// Opaque, non-secret target identity label.
    pub target_identity_label: String,
    /// Trust boundary the origin is scoped to.
    pub trust_boundary: OriginTrustBoundaryClass,
    /// Drift state since the request was last resolved.
    pub drift_state: RequestOriginDriftState,
    /// Whether the origin inherits desktop-local trust.
    pub inherits_local_trust: bool,
    /// Whether the origin keeps an explicit, named target identity.
    pub explicit_target_identity: bool,
    /// Whether retargeting requires an explicit acknowledgement.
    pub retarget_requires_ack: bool,
    /// Whether silent retargeting on reopen or rerun is blocked.
    pub silent_retarget_blocked: bool,
    /// Whether the origin class is visible.
    pub origin_class_visible: bool,
    /// Whether the target identity is visible.
    pub target_identity_visible: bool,
    /// Whether an origin-changed warning is visible when drifted.
    pub drift_warning_visible: bool,
    /// Whether the trust boundary is visible.
    pub trust_boundary_visible: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// One rerun-review sheet row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RerunReviewSheetRow {
    /// Stable sheet id.
    pub sheet_id: String,
    /// Resolved-origin row this sheet reviews.
    pub origin_ref: String,
    /// How this rerun re-resolves the origin.
    pub rerun_mode: RerunReviewMode,
    /// Opaque label of the previously resolved target.
    pub prior_origin_label: String,
    /// Opaque label of the now-resolved target.
    pub resolved_origin_label: String,
    /// Drift state the sheet reflects.
    pub drift_state: RequestOriginDriftState,
    /// Whether the sheet distinguishes rerun-exactly from rerun-with-current-context.
    pub distinguishes_rerun_modes: bool,
    /// Whether the sheet enumerates origin changes before dispatch.
    pub enumerates_changes_before_dispatch: bool,
    /// Whether review is required before dispatch.
    pub requires_review_before_dispatch: bool,
    /// Whether dispatch is blocked until the changes are reviewed.
    pub dispatch_blocked_until_reviewed: bool,
    /// History retention mode the rerun relies on.
    pub history_retention_mode: RetentionMode,
    /// Whether the sheet forces unsafe body/header retention to support compare.
    pub forces_unsafe_body_header_retention: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// One enumerated origin-change row attached to a rerun-review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginChangeRow {
    /// Stable change id.
    pub change_id: String,
    /// Rerun-review sheet this change belongs to.
    pub sheet_ref: String,
    /// What changed.
    pub change_kind: OriginChangeKind,
    /// Opaque label of the prior value.
    pub from_label: String,
    /// Opaque label of the new value.
    pub to_label: String,
    /// Whether the change must be acknowledged before dispatch.
    pub requires_ack: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// Reference to an upstream M5 packet this row consumes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginTruthUpstreamRefRow {
    /// Stable reference id.
    pub ref_id: String,
    /// Upstream record kind.
    pub upstream_record_kind: String,
    /// Repo-relative path to the upstream packet.
    pub upstream_packet_path: String,
    /// Repo-relative path to the upstream schema.
    pub upstream_schema_path: String,
    /// Whether integration has been verified.
    pub integration_verified: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// Summary counts for a request-origin truth qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginTruthQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of resolved-origin rows.
    pub origin_count: usize,
    /// Number of origins whose drift state is changed.
    pub changed_origin_count: usize,
    /// Number of rerun-review sheet rows.
    pub rerun_sheet_count: usize,
    /// Number of origin-change rows.
    pub origin_change_count: usize,
    /// Number of upstream reference rows.
    pub upstream_ref_count: usize,
    /// Number of upstream integrations that passed verification.
    pub integration_pass_count: usize,
}

/// Canonical request-origin truth qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginTruthQualificationPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Release document reference.
    pub release_doc_ref: String,
    /// Help document reference.
    pub help_doc_ref: String,
    /// JSON Schema path.
    pub schema_ref: String,
    /// Surface rows.
    pub surfaces: Vec<OriginTruthSurfaceQualificationRow>,
    /// Resolved-origin rows.
    pub origins: Vec<ResolvedOriginRow>,
    /// Rerun-review sheet rows.
    pub rerun_sheets: Vec<RerunReviewSheetRow>,
    /// Origin-change rows.
    pub origin_changes: Vec<OriginChangeRow>,
    /// Upstream reference rows.
    pub upstream_refs: Vec<OriginTruthUpstreamRefRow>,
    /// Summary counts.
    pub summary: OriginTruthQualificationSummary,
}

impl OriginTruthQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> OriginTruthQualificationSummary {
        let promoted_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.promoted_build_surface)
            .count();
        let stable_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.displayed_label.is_stable())
            .count();
        let changed_origin_count = self
            .origins
            .iter()
            .filter(|row| row.drift_state == RequestOriginDriftState::OriginChanged)
            .count();
        let integration_pass_count = self
            .upstream_refs
            .iter()
            .filter(|ref_row| ref_row.integration_verified)
            .count();
        OriginTruthQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            origin_count: self.origins.len(),
            changed_origin_count,
            rerun_sheet_count: self.rerun_sheets.len(),
            origin_change_count: self.origin_changes.len(),
            upstream_ref_count: self.upstream_refs.len(),
            integration_pass_count,
        }
    }

    /// Returns the ids of origins whose origin drifted and must warn on rerun.
    pub fn changed_origin_ids(&self) -> Vec<String> {
        self.origins
            .iter()
            .filter(|row| row.drift_state == RequestOriginDriftState::OriginChanged)
            .map(|row| row.origin_id.clone())
            .collect()
    }

    /// Returns the ids of origins that must isolate desktop-local trust (managed
    /// and browser-companion lanes).
    pub fn trust_isolated_origin_ids(&self) -> Vec<String> {
        self.origins
            .iter()
            .filter(|row| row.origin_kind.must_isolate_local_trust())
            .map(|row| row.origin_id.clone())
            .collect()
    }

    /// Returns the ids of rerun sheets that block dispatch until origin changes
    /// are reviewed.
    pub fn dispatch_blocked_sheet_ids(&self) -> Vec<String> {
        self.rerun_sheets
            .iter()
            .filter(|row| row.dispatch_blocked_until_reviewed)
            .map(|row| row.sheet_id.clone())
            .collect()
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<OriginTruthQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != ORIGIN_TRUTH_QUALIFICATION_SCHEMA_VERSION {
            violations.push(OriginTruthQualificationViolation::SchemaVersion {
                expected: ORIGIN_TRUTH_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != ORIGIN_TRUTH_QUALIFICATION_RECORD_KIND {
            violations.push(OriginTruthQualificationViolation::RecordKind {
                expected: ORIGIN_TRUTH_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let surface_ids = collect_ids(
            self.surfaces.iter().map(|row| row.surface_id.as_str()),
            &mut violations,
            OriginTruthViolationKind::Surface,
        );
        let origin_ids = collect_ids(
            self.origins.iter().map(|row| row.origin_id.as_str()),
            &mut violations,
            OriginTruthViolationKind::Origin,
        );
        let sheet_ids = collect_ids(
            self.rerun_sheets.iter().map(|row| row.sheet_id.as_str()),
            &mut violations,
            OriginTruthViolationKind::RerunSheet,
        );
        collect_ids(
            self.origin_changes.iter().map(|row| row.change_id.as_str()),
            &mut violations,
            OriginTruthViolationKind::OriginChange,
        );
        collect_ids(
            self.upstream_refs.iter().map(|row| row.ref_id.as_str()),
            &mut violations,
            OriginTruthViolationKind::UpstreamRef,
        );

        self.validate_surfaces(&mut violations);
        self.validate_origins(&mut violations, &surface_ids);
        self.validate_rerun_sheets(&mut violations, &origin_ids, &sheet_ids);
        self.validate_origin_changes(&mut violations, &sheet_ids);
        self.validate_upstream_refs(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(OriginTruthQualificationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_surfaces(&self, violations: &mut Vec<OriginTruthQualificationViolation>) {
        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(
                        OriginTruthQualificationViolation::StableSurfaceMissingProof {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
                if !surface.guards.all_visible() {
                    violations.push(
                        OriginTruthQualificationViolation::StableSurfaceMissingGuard {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
            }
            if !surface.displayed_label.is_stable()
                && surface.claim_label.is_stable()
                && !surface.downgrade_if_missing
            {
                violations.push(
                    OriginTruthQualificationViolation::NarrowedSurfaceLacksDowngradeRule {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let surface_kinds: BTreeSet<_> = self.surfaces.iter().map(|row| row.surface_kind).collect();
        for required_kind in [
            OriginTruthSurfaceKind::RequestComposer,
            OriginTruthSurfaceKind::RerunReviewSheet,
            OriginTruthSurfaceKind::RequestList,
            OriginTruthSurfaceKind::BrowserCompanionSurface,
            OriginTruthSurfaceKind::CliHeadlessOutput,
            OriginTruthSurfaceKind::SupportExport,
            OriginTruthSurfaceKind::HelpAbout,
        ] {
            if !surface_kinds.contains(&required_kind) {
                violations.push(OriginTruthQualificationViolation::MissingSurfaceKind {
                    surface_kind: required_kind,
                });
            }
        }
    }

    fn validate_origins(
        &self,
        violations: &mut Vec<OriginTruthQualificationViolation>,
        surface_ids: &BTreeSet<String>,
    ) {
        for row in &self.origins {
            if !surface_ids.contains(&row.surface_ref)
                || row.matrix_origin_ref.is_empty()
                || row.target_identity_label.is_empty()
                || !row.origin_class_visible
                || !row.target_identity_visible
                || !row.trust_boundary_visible
                || !row.explicit_target_identity
            {
                violations.push(OriginTruthQualificationViolation::IncompleteOrigin {
                    origin_id: row.origin_id.clone(),
                });
            }
            // The fine execution path must resolve under its canonical lane and
            // trust boundary so the finer vocabulary never diverges from the
            // frozen matrix lanes.
            if row.origin_kind != row.execution_path.canonical_origin()
                || row.trust_boundary != row.execution_path.canonical_trust_boundary()
            {
                violations.push(OriginTruthQualificationViolation::OriginPathMismatch {
                    origin_id: row.origin_id.clone(),
                });
            }
            // Only the local-desktop path may inherit desktop-local trust; every
            // other lane is remote to the desktop.
            if row.inherits_local_trust && !row.execution_path.may_inherit_local_trust() {
                violations.push(
                    OriginTruthQualificationViolation::OriginInheritsLocalTrust {
                        origin_id: row.origin_id.clone(),
                    },
                );
            }
            // Silent retargeting on reopen or rerun is always blocked behind an
            // acknowledgement so origins never quietly change targets.
            if !row.retarget_requires_ack || !row.silent_retarget_blocked {
                violations.push(
                    OriginTruthQualificationViolation::OriginAllowsSilentRetarget {
                        origin_id: row.origin_id.clone(),
                    },
                );
            }
            // A changed origin must surface an origin-changed warning and be
            // reviewed through a current-context rerun sheet before dispatch.
            if row.drift_state == RequestOriginDriftState::OriginChanged {
                if !row.drift_warning_visible {
                    violations.push(
                        OriginTruthQualificationViolation::ChangedOriginWarningHidden {
                            origin_id: row.origin_id.clone(),
                        },
                    );
                }
                let reviewed = self.rerun_sheets.iter().any(|sheet| {
                    sheet.origin_ref == row.origin_id
                        && sheet.rerun_mode == RerunReviewMode::RerunWithCurrentContext
                        && sheet.requires_review_before_dispatch
                        && sheet.dispatch_blocked_until_reviewed
                });
                if !reviewed {
                    violations.push(
                        OriginTruthQualificationViolation::ChangedOriginLacksReview {
                            origin_id: row.origin_id.clone(),
                        },
                    );
                }
            }
        }

        let paths: BTreeSet<_> = self.origins.iter().map(|row| row.execution_path).collect();
        for required_path in [
            OriginExecutionPath::LocalDesktop,
            OriginExecutionPath::Ssh,
            OriginExecutionPath::Container,
            OriginExecutionPath::ManagedWorkspace,
            OriginExecutionPath::BrowserCompanion,
        ] {
            if !paths.contains(&required_path) {
                violations.push(OriginTruthQualificationViolation::MissingExecutionPath {
                    execution_path: required_path,
                });
            }
        }

        // At least one origin must have drifted so drift review is exercised
        // rather than asserted as theoretically possible.
        if !self
            .origins
            .iter()
            .any(|row| row.drift_state == RequestOriginDriftState::OriginChanged)
        {
            violations.push(OriginTruthQualificationViolation::NoChangedOriginCovered);
        }
    }

    fn validate_rerun_sheets(
        &self,
        violations: &mut Vec<OriginTruthQualificationViolation>,
        origin_ids: &BTreeSet<String>,
        sheet_ids: &BTreeSet<String>,
    ) {
        let _ = sheet_ids;
        for row in &self.rerun_sheets {
            if !origin_ids.contains(&row.origin_ref)
                || row.prior_origin_label.is_empty()
                || row.resolved_origin_label.is_empty()
                || !row.distinguishes_rerun_modes
            {
                violations.push(OriginTruthQualificationViolation::IncompleteRerunSheet {
                    sheet_id: row.sheet_id.clone(),
                });
            }
            // Rerun compare and review UX must never widen request-history
            // retention toward unsafe body or header capture.
            if row.forces_unsafe_body_header_retention
                || row.history_retention_mode == RetentionMode::OptInFullCapture
            {
                violations.push(
                    OriginTruthQualificationViolation::RerunForcesUnsafeRetention {
                        sheet_id: row.sheet_id.clone(),
                    },
                );
            }
            match row.rerun_mode {
                // Rerun-exactly pins the recorded target, so the resolved origin
                // equals the prior origin and never drifts.
                RerunReviewMode::RerunExactly => {
                    if row.drift_state != RequestOriginDriftState::OriginStable
                        || row.prior_origin_label != row.resolved_origin_label
                    {
                        violations.push(OriginTruthQualificationViolation::RerunExactlyDrifts {
                            sheet_id: row.sheet_id.clone(),
                        });
                    }
                }
                // Rerun-with-current-context that drifted must require review,
                // block dispatch, and enumerate at least one origin change.
                RerunReviewMode::RerunWithCurrentContext => {
                    if row.drift_state == RequestOriginDriftState::OriginChanged {
                        let enumerated = self
                            .origin_changes
                            .iter()
                            .any(|change| change.sheet_ref == row.sheet_id);
                        if !row.requires_review_before_dispatch
                            || !row.dispatch_blocked_until_reviewed
                            || !row.enumerates_changes_before_dispatch
                            || !enumerated
                        {
                            violations.push(
                                OriginTruthQualificationViolation::DriftedRerunLacksReview {
                                    sheet_id: row.sheet_id.clone(),
                                },
                            );
                        }
                    }
                }
            }
        }

        let modes: BTreeSet<_> = self.rerun_sheets.iter().map(|row| row.rerun_mode).collect();
        for required_mode in [
            RerunReviewMode::RerunExactly,
            RerunReviewMode::RerunWithCurrentContext,
        ] {
            if !modes.contains(&required_mode) {
                violations.push(OriginTruthQualificationViolation::MissingRerunMode {
                    rerun_mode: required_mode,
                });
            }
        }
    }

    fn validate_origin_changes(
        &self,
        violations: &mut Vec<OriginTruthQualificationViolation>,
        sheet_ids: &BTreeSet<String>,
    ) {
        for row in &self.origin_changes {
            if !sheet_ids.contains(&row.sheet_ref)
                || row.from_label.is_empty()
                || row.to_label.is_empty()
                || row.from_label == row.to_label
            {
                violations.push(OriginTruthQualificationViolation::IncompleteOriginChange {
                    change_id: row.change_id.clone(),
                });
            }
            // A boundary-crossing change must be acknowledged before dispatch.
            if row.change_kind.crosses_boundary() && !row.requires_ack {
                violations.push(OriginTruthQualificationViolation::OriginChangeMissingAck {
                    change_id: row.change_id.clone(),
                });
            }
            // Changes are only enumerated for current-context reruns, never for
            // a rerun-exactly sheet.
            let on_current_context = self.rerun_sheets.iter().any(|sheet| {
                sheet.sheet_id == row.sheet_ref
                    && sheet.rerun_mode == RerunReviewMode::RerunWithCurrentContext
            });
            if !on_current_context {
                violations.push(
                    OriginTruthQualificationViolation::OriginChangeOnExactRerun {
                        change_id: row.change_id.clone(),
                    },
                );
            }
        }

        let change_kinds: BTreeSet<_> = self
            .origin_changes
            .iter()
            .map(|row| row.change_kind)
            .collect();
        for required_kind in [
            OriginChangeKind::HostIdentityChanged,
            OriginChangeKind::OriginLaneChanged,
            OriginChangeKind::TrustBoundaryChanged,
            OriginChangeKind::PortOrServiceChanged,
            OriginChangeKind::PrivateDnsRebound,
        ] {
            if !change_kinds.contains(&required_kind) {
                violations.push(OriginTruthQualificationViolation::MissingOriginChangeKind {
                    change_kind: required_kind,
                });
            }
        }
    }

    fn validate_upstream_refs(&self, violations: &mut Vec<OriginTruthQualificationViolation>) {
        for row in &self.upstream_refs {
            if row.upstream_record_kind.is_empty()
                || row.upstream_packet_path.is_empty()
                || row.upstream_schema_path.is_empty()
            {
                violations.push(OriginTruthQualificationViolation::IncompleteUpstreamRef {
                    ref_id: row.ref_id.clone(),
                });
            }
        }
        // The origin truth must consume the frozen API-collection matrix as a
        // verified upstream packet so the origin and drift lanes stay aligned.
        let consumes_matrix = self.upstream_refs.iter().any(|row| {
            row.upstream_record_kind == API_MATRIX_QUALIFICATION_RECORD_KIND
                && row.integration_verified
        });
        if !consumes_matrix {
            violations.push(OriginTruthQualificationViolation::MatrixUpstreamNotIntegrated);
        }
    }
}

/// Loads the checked-in request-origin truth qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_origin_truth_qualification(
) -> Result<OriginTruthQualificationPacket, serde_json::Error> {
    serde_json::from_str(ORIGIN_TRUTH_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OriginTruthViolationKind {
    /// Surface rows.
    Surface,
    /// Resolved-origin rows.
    Origin,
    /// Rerun-review sheet rows.
    RerunSheet,
    /// Origin-change rows.
    OriginChange,
    /// Upstream reference rows.
    UpstreamRef,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<OriginTruthQualificationViolation>,
    kind: OriginTruthViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(OriginTruthQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for request-origin truth qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OriginTruthQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: OriginTruthViolationKind,
        id: String,
    },
    /// Stable row has no proof packet.
    StableSurfaceMissingProof { surface_id: String },
    /// Stable row is missing one or more visible guards.
    StableSurfaceMissingGuard { surface_id: String },
    /// Narrowed stable claim lacks an explicit downgrade rule.
    NarrowedSurfaceLacksDowngradeRule { surface_id: String },
    /// Required consumer surface kind is missing.
    MissingSurfaceKind {
        surface_kind: OriginTruthSurfaceKind,
    },
    /// Origin row does not project origin/target/trust truth everywhere.
    IncompleteOrigin { origin_id: String },
    /// Origin's execution path does not resolve under its canonical lane/boundary.
    OriginPathMismatch { origin_id: String },
    /// A non-local-desktop origin inherits desktop-local trust.
    OriginInheritsLocalTrust { origin_id: String },
    /// Origin allows silent retargeting on reopen or rerun.
    OriginAllowsSilentRetarget { origin_id: String },
    /// Changed origin hides its origin-changed warning.
    ChangedOriginWarningHidden { origin_id: String },
    /// Changed origin has no current-context rerun review blocking dispatch.
    ChangedOriginLacksReview { origin_id: String },
    /// Required execution path is missing.
    MissingExecutionPath { execution_path: OriginExecutionPath },
    /// No origin drifted, so drift review is not exercised.
    NoChangedOriginCovered,
    /// Rerun sheet does not resolve its origin or distinguish rerun modes.
    IncompleteRerunSheet { sheet_id: String },
    /// Rerun sheet forces unsafe retention to support compare.
    RerunForcesUnsafeRetention { sheet_id: String },
    /// A rerun-exactly sheet drifts from its recorded target.
    RerunExactlyDrifts { sheet_id: String },
    /// A drifted current-context rerun does not require review or enumerate changes.
    DriftedRerunLacksReview { sheet_id: String },
    /// Required rerun mode is missing.
    MissingRerunMode { rerun_mode: RerunReviewMode },
    /// Origin-change row is incomplete.
    IncompleteOriginChange { change_id: String },
    /// Boundary-crossing change is not acknowledged before dispatch.
    OriginChangeMissingAck { change_id: String },
    /// Origin-change row is attached to a rerun-exactly sheet.
    OriginChangeOnExactRerun { change_id: String },
    /// Required origin-change kind is missing.
    MissingOriginChangeKind { change_kind: OriginChangeKind },
    /// Upstream reference is incomplete.
    IncompleteUpstreamRef { ref_id: String },
    /// The origin truth does not consume the API-collection matrix as a verified upstream packet.
    MatrixUpstreamNotIntegrated,
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for OriginTruthQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema_version expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record_kind expected {expected}, got {actual}")
            }
            Self::DuplicateId { kind, id } => write!(f, "{kind:?} id {id} is duplicated"),
            Self::StableSurfaceMissingProof { surface_id } => {
                write!(f, "{surface_id} is stable without a proof packet")
            }
            Self::StableSurfaceMissingGuard { surface_id } => {
                write!(f, "{surface_id} is stable without complete guard truth")
            }
            Self::NarrowedSurfaceLacksDowngradeRule { surface_id } => {
                write!(f, "{surface_id} is narrowed without a downgrade rule")
            }
            Self::MissingSurfaceKind { surface_kind } => {
                write!(f, "consumer surface kind {surface_kind:?} is not covered")
            }
            Self::IncompleteOrigin { origin_id } => {
                write!(f, "{origin_id} does not project origin truth everywhere")
            }
            Self::OriginPathMismatch { origin_id } => {
                write!(
                    f,
                    "{origin_id} execution path does not match its canonical lane or trust boundary"
                )
            }
            Self::OriginInheritsLocalTrust { origin_id } => {
                write!(
                    f,
                    "{origin_id} inherits desktop-local trust it must not have"
                )
            }
            Self::OriginAllowsSilentRetarget { origin_id } => {
                write!(
                    f,
                    "{origin_id} allows silent retargeting on reopen or rerun"
                )
            }
            Self::ChangedOriginWarningHidden { origin_id } => {
                write!(f, "{origin_id} hides its origin-changed warning")
            }
            Self::ChangedOriginLacksReview { origin_id } => {
                write!(
                    f,
                    "{origin_id} drifted without a current-context rerun review blocking dispatch"
                )
            }
            Self::MissingExecutionPath { execution_path } => {
                write!(f, "execution path {execution_path:?} is not covered")
            }
            Self::NoChangedOriginCovered => {
                write!(f, "no origin drifted, so drift review is not exercised")
            }
            Self::IncompleteRerunSheet { sheet_id } => {
                write!(
                    f,
                    "{sheet_id} does not resolve its origin or distinguish rerun modes"
                )
            }
            Self::RerunForcesUnsafeRetention { sheet_id } => {
                write!(
                    f,
                    "{sheet_id} forces unsafe body/header retention to compare"
                )
            }
            Self::RerunExactlyDrifts { sheet_id } => {
                write!(
                    f,
                    "{sheet_id} is rerun-exactly yet drifts from its recorded target"
                )
            }
            Self::DriftedRerunLacksReview { sheet_id } => {
                write!(
                    f,
                    "{sheet_id} drifted under current context without review or enumerated changes"
                )
            }
            Self::MissingRerunMode { rerun_mode } => {
                write!(f, "rerun mode {rerun_mode:?} is not covered")
            }
            Self::IncompleteOriginChange { change_id } => {
                write!(f, "{change_id} does not project change truth everywhere")
            }
            Self::OriginChangeMissingAck { change_id } => {
                write!(
                    f,
                    "{change_id} crosses a boundary without an acknowledgement"
                )
            }
            Self::OriginChangeOnExactRerun { change_id } => {
                write!(f, "{change_id} is attached to a rerun-exactly sheet")
            }
            Self::MissingOriginChangeKind { change_kind } => {
                write!(f, "origin-change kind {change_kind:?} is not covered")
            }
            Self::IncompleteUpstreamRef { ref_id } => {
                write!(
                    f,
                    "{ref_id} does not project upstream reference truth everywhere"
                )
            }
            Self::MatrixUpstreamNotIntegrated => {
                write!(
                    f,
                    "origin truth does not consume the API-collection matrix as a verified upstream packet"
                )
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for OriginTruthQualificationViolation {}
