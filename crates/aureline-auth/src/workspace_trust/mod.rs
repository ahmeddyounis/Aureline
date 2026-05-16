//! Beta workspace-trust audit projection for claimed execution lanes.
//!
//! This module promotes the restricted-mode alpha packet into a page-level
//! beta audit over every claimed workspace-trust surface family. It keeps
//! open, run, debug, extension, AI, provider, review, support, and admin rows
//! on one vocabulary: the trust-state matrix authority, the explicit
//! trust-elevation audit event, the trust-loss fallback, profile-specific
//! connected / mirrored / offline / enterprise behavior, and the
//! support-export row all come from the same record.
//!
//! The shell, headless inspector, support bundle, and reviewer docs should
//! consume [`seeded_workspace_trust_beta_page`] rather than re-deriving local
//! `is_trusted` checks.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::trust::{
    authority_for_trust_state, CapabilityAuthorityClass, LaunchWedgeCapabilityFamily,
    RestrictedModeTrustStateClass, TrustAuditEventClass, TrustRecoveryActionClass,
};

/// Beta schema version exported with every workspace-trust record.
pub const WORKSPACE_TRUST_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every beta workspace-trust record.
pub const WORKSPACE_TRUST_BETA_SHARED_CONTRACT_REF: &str = "security:workspace_trust_beta:v1";

/// Stable record kind for [`WorkspaceTrustBetaPage`] payloads.
pub const WORKSPACE_TRUST_BETA_PAGE_RECORD_KIND: &str = "security_workspace_trust_beta_page_record";

/// Stable record kind for [`WorkspaceTrustBetaRow`] payloads.
pub const WORKSPACE_TRUST_BETA_ROW_RECORD_KIND: &str = "security_workspace_trust_beta_row_record";

/// Stable record kind for [`WorkspaceTrustBetaSupportRow`] payloads.
pub const WORKSPACE_TRUST_BETA_SUPPORT_ROW_RECORD_KIND: &str =
    "security_workspace_trust_beta_support_row_record";

/// Stable record kind for [`WorkspaceTrustBetaDefect`] payloads.
pub const WORKSPACE_TRUST_BETA_DEFECT_RECORD_KIND: &str =
    "security_workspace_trust_beta_defect_record";

/// Stable record kind for [`WorkspaceTrustBetaSupportExport`] payloads.
pub const WORKSPACE_TRUST_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "security_workspace_trust_beta_support_export_record";

/// Claimed beta surface families covered by the workspace-trust audit.
pub const WORKSPACE_TRUST_BETA_SURFACE_FAMILIES: [LaunchWedgeCapabilityFamily; 26] = [
    LaunchWedgeCapabilityFamily::WorkspaceOpenRestore,
    LaunchWedgeCapabilityFamily::EditorReadWrite,
    LaunchWedgeCapabilityFamily::SearchLocal,
    LaunchWedgeCapabilityFamily::LocalGitRead,
    LaunchWedgeCapabilityFamily::LocalGitWrite,
    LaunchWedgeCapabilityFamily::ShellCommandPalette,
    LaunchWedgeCapabilityFamily::TasksRun,
    LaunchWedgeCapabilityFamily::TerminalManualOpen,
    LaunchWedgeCapabilityFamily::TerminalRepoRecipeLaunch,
    LaunchWedgeCapabilityFamily::DebugLaunch,
    LaunchWedgeCapabilityFamily::NotebookKernelConnect,
    LaunchWedgeCapabilityFamily::NotebookCellExecute,
    LaunchWedgeCapabilityFamily::NotebookRichOutputRender,
    LaunchWedgeCapabilityFamily::AiContextRead,
    LaunchWedgeCapabilityFamily::AiApplyMutation,
    LaunchWedgeCapabilityFamily::AiToolCallMutating,
    LaunchWedgeCapabilityFamily::ExtensionActivation,
    LaunchWedgeCapabilityFamily::ExtensionInstall,
    LaunchWedgeCapabilityFamily::EnvironmentActivatorRun,
    LaunchWedgeCapabilityFamily::ScaffoldTemplateRun,
    LaunchWedgeCapabilityFamily::ConnectedProviderOpen,
    LaunchWedgeCapabilityFamily::ConnectedProviderToolCall,
    LaunchWedgeCapabilityFamily::RemoteAttach,
    LaunchWedgeCapabilityFamily::McpServerLaunch,
    LaunchWedgeCapabilityFamily::SupportBundleExport,
    LaunchWedgeCapabilityFamily::AdminPolicyRead,
];

/// Closed lane vocabulary used to group claimed beta rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceTrustBetaLaneClass {
    /// Workspace open, editor, search, and passive local inspection.
    Open,
    /// Shell, task, terminal, environment, and MCP launch paths.
    Run,
    /// Debugger and notebook execution/rendering paths.
    Debug,
    /// Extension activation and install/update paths.
    Extension,
    /// AI context, apply, and tool-call paths.
    Ai,
    /// Connected-provider and remote-attach paths.
    Provider,
    /// Git write and scaffold/template review paths.
    Review,
    /// Redacted support export path.
    Support,
    /// Effective admin-policy inspection path.
    Admin,
}

impl WorkspaceTrustBetaLaneClass {
    /// Stable token recorded on beta rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Run => "run",
            Self::Debug => "debug",
            Self::Extension => "extension",
            Self::Ai => "ai",
            Self::Provider => "provider",
            Self::Review => "review",
            Self::Support => "support",
            Self::Admin => "admin",
        }
    }
}

/// Connectedness or enterprise profile under which a row is inspected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceTrustBetaProfileClass {
    /// Normal connected beta profile.
    Connected,
    /// Mirror-only profile where public endpoints are not fallback targets.
    MirrorOnly,
    /// Offline or air-gapped profile.
    Offline,
    /// Enterprise-managed profile with signed policy narrowing.
    EnterpriseManaged,
}

impl WorkspaceTrustBetaProfileClass {
    /// All required beta profiles in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Connected,
        Self::MirrorOnly,
        Self::Offline,
        Self::EnterpriseManaged,
    ];

    /// Stable token recorded on beta rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::MirrorOnly => "mirror_only",
            Self::Offline => "offline",
            Self::EnterpriseManaged => "enterprise_managed",
        }
    }
}

/// Typed defect emitted by the workspace-trust beta validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceTrustBetaDefectKind {
    /// A claimed row is absent from the page.
    MissingMatrixSurface,
    /// A row does not expose restricted mode before trust is granted.
    MissingRestrictedModeAvailability,
    /// A run-capable or mutation-capable row is allowed before trust.
    RunOrMutationAllowedBeforeTrust,
    /// Trust loss does not narrow a capability consistently.
    TrustLossNotPropagated,
    /// A row that needs elevation has no trust or approval cue.
    MissingEscalationCue,
    /// Connected, mirror, offline, or enterprise profile coverage is missing.
    ProfileCoverageMissing,
    /// A profile would silently fall back to an undeclared public endpoint.
    HiddenPublicEndpointFallback,
    /// A support/export row drifted from its live row.
    SupportRowVocabularyDrift,
    /// A trusted-policy-degraded row is wider than trusted.
    PolicyDegradedWidensTrusted,
    /// A row would expose raw private or secret material.
    RawPrivateMaterialExposed,
}

impl WorkspaceTrustBetaDefectKind {
    /// Stable token recorded on defect rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingMatrixSurface => "missing_matrix_surface",
            Self::MissingRestrictedModeAvailability => "missing_restricted_mode_availability",
            Self::RunOrMutationAllowedBeforeTrust => "run_or_mutation_allowed_before_trust",
            Self::TrustLossNotPropagated => "trust_loss_not_propagated",
            Self::MissingEscalationCue => "missing_escalation_cue",
            Self::ProfileCoverageMissing => "profile_coverage_missing",
            Self::HiddenPublicEndpointFallback => "hidden_public_endpoint_fallback",
            Self::SupportRowVocabularyDrift => "support_row_vocabulary_drift",
            Self::PolicyDegradedWidensTrusted => "policy_degraded_widens_trusted",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
        }
    }
}

/// Profile-specific authority projection for one beta row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTrustBetaProfileAuthority {
    /// Profile class.
    pub profile_class: WorkspaceTrustBetaProfileClass,
    /// Stable token for [`Self::profile_class`].
    pub profile_token: String,
    /// Effective authority in this profile.
    pub authority: CapabilityAuthorityClass,
    /// Stable token for [`Self::authority`].
    pub authority_token: String,
    /// Reviewable source label naming why the profile narrows or admits.
    pub source_label: String,
    /// True when the profile refuses undeclared public endpoint fallback.
    pub no_public_endpoint_fallback: bool,
}

/// One live beta row proving workspace-trust behavior for a surface family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTrustBetaRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Lane class.
    pub lane: WorkspaceTrustBetaLaneClass,
    /// Stable token for [`Self::lane`].
    pub lane_token: String,
    /// Surface family.
    pub surface_family: LaunchWedgeCapabilityFamily,
    /// Stable token for [`Self::surface_family`].
    pub surface_family_token: String,
    /// Matrix artifact this row implements.
    pub source_matrix_ref: String,
    /// Authority before trust is granted.
    pub restricted_authority: CapabilityAuthorityClass,
    /// Stable token for [`Self::restricted_authority`].
    pub restricted_authority_token: String,
    /// Authority after explicit trust is granted.
    pub trusted_authority: CapabilityAuthorityClass,
    /// Stable token for [`Self::trusted_authority`].
    pub trusted_authority_token: String,
    /// Authority when managed policy narrows a trusted workspace.
    pub trusted_policy_degraded_authority: CapabilityAuthorityClass,
    /// Stable token for [`Self::trusted_policy_degraded_authority`].
    pub trusted_policy_degraded_authority_token: String,
    /// Authority after trust is revoked or lost.
    pub trust_loss_authority: CapabilityAuthorityClass,
    /// Stable token for [`Self::trust_loss_authority`].
    pub trust_loss_authority_token: String,
    /// Profile-specific authority projection.
    pub profile_authorities: Vec<WorkspaceTrustBetaProfileAuthority>,
    /// True when the row has an explicit restricted-mode behavior.
    pub restricted_mode_available_before_trust: bool,
    /// Plain-language restricted-mode explanation.
    pub restricted_mode_explainer: String,
    /// True when this row belongs to the restricted-posture floor.
    pub floor_capability: bool,
    /// True when the row can execute code, mutate state, egress, or use
    /// identity authority when admitted.
    pub run_or_mutation_capable: bool,
    /// True when the row requires trust grant or a per-invocation approval.
    pub requires_explicit_trust_elevation: bool,
    /// Audit event token emitted by explicit trust elevation.
    pub trust_elevation_audit_event_token: String,
    /// Audit event token emitted by trust loss.
    pub trust_loss_audit_event_token: String,
    /// True when shell surfaces consume this trust row after state changes.
    pub trust_loss_propagates_to_shell: bool,
    /// True when runtime/execution surfaces consume this trust row.
    pub trust_loss_propagates_to_runtime: bool,
    /// True when extension hosts consume this trust row without re-deriving it.
    pub trust_loss_propagates_to_extension: bool,
    /// True when support/export surfaces consume this trust row.
    pub trust_loss_propagates_to_support: bool,
    /// Escalation or recovery cue tokens visible on blocked/review rows.
    pub escalation_cue_tokens: Vec<String>,
    /// Export-safe summary shown in support packets.
    pub support_export_summary: String,
    /// True when no undeclared public endpoint fallback is allowed.
    pub no_public_endpoint_fallback: bool,
    /// True when raw private/secret material is excluded from this record.
    pub raw_private_material_excluded: bool,
}

/// Export-safe support row aligned one-to-one with a live beta row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTrustBetaSupportRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Live row id.
    pub row_id: String,
    /// Lane token copied from the live row.
    pub lane_token: String,
    /// Surface family token copied from the live row.
    pub surface_family_token: String,
    /// Restricted authority token copied from the live row.
    pub restricted_authority_token: String,
    /// Trust-loss authority token copied from the live row.
    pub trust_loss_authority_token: String,
    /// Profile authority tokens by profile token.
    pub profile_authority_tokens: BTreeMap<String, String>,
    /// Escalation cue tokens copied from the live row.
    pub escalation_cue_tokens: Vec<String>,
    /// Export-safe support summary.
    pub support_export_summary: String,
    /// True when no undeclared public endpoint fallback is allowed.
    pub no_public_endpoint_fallback: bool,
    /// True when raw private/secret material is excluded.
    pub raw_private_material_excluded: bool,
}

impl WorkspaceTrustBetaSupportRow {
    /// Builds an export-safe row from a live beta row.
    pub fn from_row(row: &WorkspaceTrustBetaRow) -> Self {
        let profile_authority_tokens = row
            .profile_authorities
            .iter()
            .map(|profile| {
                (
                    profile.profile_token.clone(),
                    profile.authority_token.clone(),
                )
            })
            .collect();
        Self {
            record_kind: WORKSPACE_TRUST_BETA_SUPPORT_ROW_RECORD_KIND.to_owned(),
            schema_version: WORKSPACE_TRUST_BETA_SCHEMA_VERSION,
            shared_contract_ref: WORKSPACE_TRUST_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_id: row.row_id.clone(),
            lane_token: row.lane_token.clone(),
            surface_family_token: row.surface_family_token.clone(),
            restricted_authority_token: row.restricted_authority_token.clone(),
            trust_loss_authority_token: row.trust_loss_authority_token.clone(),
            profile_authority_tokens,
            escalation_cue_tokens: row.escalation_cue_tokens.clone(),
            support_export_summary: row.support_export_summary.clone(),
            no_public_endpoint_fallback: row.no_public_endpoint_fallback,
            raw_private_material_excluded: row.raw_private_material_excluded,
        }
    }
}

/// Typed validation defect for the workspace-trust beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTrustBetaDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Defect kind.
    pub defect_kind: WorkspaceTrustBetaDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Row id, or `matrix` for missing-row defects.
    pub row_id: String,
    /// Surface family token.
    pub surface_family_token: String,
    /// Field that failed validation.
    pub field: String,
    /// Export-safe explanation.
    pub note: String,
}

impl WorkspaceTrustBetaDefect {
    fn new(
        defect_kind: WorkspaceTrustBetaDefectKind,
        row_id: impl Into<String>,
        surface_family_token: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: WORKSPACE_TRUST_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: WORKSPACE_TRUST_BETA_SCHEMA_VERSION,
            shared_contract_ref: WORKSPACE_TRUST_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            row_id: row_id.into(),
            surface_family_token: surface_family_token.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Aggregate summary for the workspace-trust beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTrustBetaSummary {
    /// Stable record kind for the parent page.
    pub page_record_kind: String,
    /// Number of live rows.
    pub row_count: usize,
    /// Number of support rows.
    pub support_row_count: usize,
    /// Number of rows in the restricted-posture floor.
    pub restricted_floor_row_count: usize,
    /// Number of rows blocked or review-gated before trust.
    pub blocked_or_review_before_trust_count: usize,
    /// Number of run-capable or mutation-capable rows.
    pub run_or_mutation_capable_count: usize,
    /// Lane tokens present on the page.
    pub lanes_present: Vec<String>,
    /// Profile tokens present on every valid row.
    pub profiles_present: Vec<String>,
    /// Defect count.
    pub defect_count: usize,
    /// Defect counts by defect kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl WorkspaceTrustBetaSummary {
    /// Builds a summary from live rows, support rows, and defects.
    pub fn from_rows(
        rows: &[WorkspaceTrustBetaRow],
        support_rows: &[WorkspaceTrustBetaSupportRow],
        defects: &[WorkspaceTrustBetaDefect],
    ) -> Self {
        let lanes_present: BTreeSet<String> =
            rows.iter().map(|row| row.lane_token.clone()).collect();
        let profiles_present: BTreeSet<String> = rows
            .iter()
            .flat_map(|row| {
                row.profile_authorities
                    .iter()
                    .map(|profile| profile.profile_token.clone())
            })
            .collect();
        let mut defect_counts_by_kind = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }

        Self {
            page_record_kind: WORKSPACE_TRUST_BETA_PAGE_RECORD_KIND.to_owned(),
            row_count: rows.len(),
            support_row_count: support_rows.len(),
            restricted_floor_row_count: rows.iter().filter(|row| row.floor_capability).count(),
            blocked_or_review_before_trust_count: rows
                .iter()
                .filter(|row| row.restricted_authority.requires_explanation())
                .count(),
            run_or_mutation_capable_count: rows
                .iter()
                .filter(|row| row.run_or_mutation_capable)
                .count(),
            lanes_present: lanes_present.into_iter().collect(),
            profiles_present: profiles_present.into_iter().collect(),
            defect_count: defects.len(),
            defect_counts_by_kind,
        }
    }
}

/// Top-level beta page consumed by shell, headless inspection, support export,
/// docs, and fixture replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTrustBetaPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source artifact matrix ref.
    pub source_matrix_ref: String,
    /// Live beta rows.
    pub rows: Vec<WorkspaceTrustBetaRow>,
    /// Support/export rows.
    pub support_rows: Vec<WorkspaceTrustBetaSupportRow>,
    /// Typed validation defects.
    pub defects: Vec<WorkspaceTrustBetaDefect>,
    /// Aggregate summary.
    pub summary: WorkspaceTrustBetaSummary,
}

/// Support-export wrapper for the workspace-trust beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTrustBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Exported page.
    pub page: WorkspaceTrustBetaPage,
    /// Defect kind tokens present.
    pub defect_kinds_present: Vec<String>,
    /// Defect counts by token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when raw private/secret material is excluded.
    pub raw_private_material_excluded: bool,
}

impl WorkspaceTrustBetaSupportExport {
    /// Builds a support-export wrapper from a beta page.
    pub fn from_page(
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
        page: WorkspaceTrustBetaPage,
    ) -> Self {
        let defect_counts_by_kind = page.summary.defect_counts_by_kind.clone();
        let defect_kinds_present = defect_counts_by_kind.keys().cloned().collect();
        Self {
            record_kind: WORKSPACE_TRUST_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: WORKSPACE_TRUST_BETA_SCHEMA_VERSION,
            shared_contract_ref: WORKSPACE_TRUST_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            page,
            defect_kinds_present,
            defect_counts_by_kind,
            raw_private_material_excluded: true,
        }
    }
}

/// Builds the seeded workspace-trust beta page.
pub fn seeded_workspace_trust_beta_page() -> WorkspaceTrustBetaPage {
    let rows: Vec<WorkspaceTrustBetaRow> = WORKSPACE_TRUST_BETA_SURFACE_FAMILIES
        .iter()
        .copied()
        .map(seed_row)
        .collect();
    let support_rows: Vec<WorkspaceTrustBetaSupportRow> = rows
        .iter()
        .map(WorkspaceTrustBetaSupportRow::from_row)
        .collect();
    let defects = audit_workspace_trust_beta_rows(&rows, &support_rows);
    let summary = WorkspaceTrustBetaSummary::from_rows(&rows, &support_rows, &defects);
    WorkspaceTrustBetaPage {
        record_kind: WORKSPACE_TRUST_BETA_PAGE_RECORD_KIND.to_owned(),
        schema_version: WORKSPACE_TRUST_BETA_SCHEMA_VERSION,
        shared_contract_ref: WORKSPACE_TRUST_BETA_SHARED_CONTRACT_REF.to_owned(),
        source_matrix_ref: "artifacts/security/trust_state_matrix.yaml".to_owned(),
        rows,
        support_rows,
        defects,
        summary,
    }
}

/// Validates a beta page and returns typed defects on failure.
pub fn validate_workspace_trust_beta_page(
    page: &WorkspaceTrustBetaPage,
) -> Result<(), Vec<WorkspaceTrustBetaDefect>> {
    let defects = audit_workspace_trust_beta_rows(&page.rows, &page.support_rows);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Recomputes defects for workspace-trust beta rows and support rows.
pub fn audit_workspace_trust_beta_rows(
    rows: &[WorkspaceTrustBetaRow],
    support_rows: &[WorkspaceTrustBetaSupportRow],
) -> Vec<WorkspaceTrustBetaDefect> {
    let mut defects = Vec::new();
    let expected: BTreeSet<&str> = WORKSPACE_TRUST_BETA_SURFACE_FAMILIES
        .iter()
        .map(|family| family.as_str())
        .collect();
    let observed: BTreeSet<&str> = rows
        .iter()
        .map(|row| row.surface_family_token.as_str())
        .collect();

    for missing in expected.difference(&observed) {
        defects.push(WorkspaceTrustBetaDefect::new(
            WorkspaceTrustBetaDefectKind::MissingMatrixSurface,
            "matrix",
            *missing,
            "surface_family",
            "claimed beta surface family is missing from the workspace-trust page",
        ));
    }

    let support_by_row: BTreeMap<&str, &WorkspaceTrustBetaSupportRow> = support_rows
        .iter()
        .map(|support| (support.row_id.as_str(), support))
        .collect();

    for row in rows {
        if !row.restricted_mode_available_before_trust {
            defects.push(row_defect(
                row,
                WorkspaceTrustBetaDefectKind::MissingRestrictedModeAvailability,
                "restricted_mode_available_before_trust",
                "row does not expose restricted mode before trust is granted",
            ));
        }

        if row.run_or_mutation_capable
            && row.restricted_authority == CapabilityAuthorityClass::Allowed
        {
            defects.push(row_defect(
                row,
                WorkspaceTrustBetaDefectKind::RunOrMutationAllowedBeforeTrust,
                "restricted_authority",
                "run-capable or mutation-capable row is allowed before trust",
            ));
        }

        if !row.floor_capability
            && row.surface_family != LaunchWedgeCapabilityFamily::AiContextRead
            && row.trust_loss_authority == CapabilityAuthorityClass::Allowed
        {
            defects.push(row_defect(
                row,
                WorkspaceTrustBetaDefectKind::TrustLossNotPropagated,
                "trust_loss_authority",
                "trust loss must narrow non-floor rows",
            ));
        }

        if !row.trust_loss_propagates_to_shell
            || !row.trust_loss_propagates_to_runtime
            || !row.trust_loss_propagates_to_extension
            || !row.trust_loss_propagates_to_support
        {
            defects.push(row_defect(
                row,
                WorkspaceTrustBetaDefectKind::TrustLossNotPropagated,
                "trust_loss_propagation",
                "trust loss must propagate to shell, runtime, extension, and support surfaces",
            ));
        }

        if row.requires_explicit_trust_elevation
            && !row.escalation_cue_tokens.iter().any(|cue| {
                cue == "request_trust_grant_session_only" || cue == "request_approval_ticket"
            })
        {
            defects.push(row_defect(
                row,
                WorkspaceTrustBetaDefectKind::MissingEscalationCue,
                "escalation_cue_tokens",
                "trust-gated rows must expose a trust or approval cue",
            ));
        }

        let profile_tokens: BTreeSet<&str> = row
            .profile_authorities
            .iter()
            .map(|profile| profile.profile_token.as_str())
            .collect();
        for expected_profile in WorkspaceTrustBetaProfileClass::ALL {
            if !profile_tokens.contains(expected_profile.as_str()) {
                defects.push(row_defect(
                    row,
                    WorkspaceTrustBetaDefectKind::ProfileCoverageMissing,
                    "profile_authorities",
                    format!("missing {} profile coverage", expected_profile.as_str()),
                ));
            }
        }

        if !row.no_public_endpoint_fallback
            || row
                .profile_authorities
                .iter()
                .any(|profile| !profile.no_public_endpoint_fallback)
        {
            defects.push(row_defect(
                row,
                WorkspaceTrustBetaDefectKind::HiddenPublicEndpointFallback,
                "no_public_endpoint_fallback",
                "profile row permits undeclared public endpoint fallback",
            ));
        }

        if !row.raw_private_material_excluded {
            defects.push(row_defect(
                row,
                WorkspaceTrustBetaDefectKind::RawPrivateMaterialExposed,
                "raw_private_material_excluded",
                "workspace-trust rows must be export-safe metadata",
            ));
        }

        if authority_width(row.trusted_policy_degraded_authority)
            > authority_width(row.trusted_authority)
        {
            defects.push(row_defect(
                row,
                WorkspaceTrustBetaDefectKind::PolicyDegradedWidensTrusted,
                "trusted_policy_degraded_authority",
                "trusted_policy_degraded must be equal-or-narrower than trusted",
            ));
        }

        match support_by_row.get(row.row_id.as_str()) {
            Some(support) => compare_support_row(row, support, &mut defects),
            None => defects.push(row_defect(
                row,
                WorkspaceTrustBetaDefectKind::SupportRowVocabularyDrift,
                "support_rows",
                "missing support row for live workspace-trust row",
            )),
        }
    }

    defects
}

fn compare_support_row(
    row: &WorkspaceTrustBetaRow,
    support: &WorkspaceTrustBetaSupportRow,
    defects: &mut Vec<WorkspaceTrustBetaDefect>,
) {
    let profile_authority_tokens: BTreeMap<String, String> = row
        .profile_authorities
        .iter()
        .map(|profile| {
            (
                profile.profile_token.clone(),
                profile.authority_token.clone(),
            )
        })
        .collect();
    if support.lane_token != row.lane_token
        || support.surface_family_token != row.surface_family_token
        || support.restricted_authority_token != row.restricted_authority_token
        || support.trust_loss_authority_token != row.trust_loss_authority_token
        || support.profile_authority_tokens != profile_authority_tokens
        || support.escalation_cue_tokens != row.escalation_cue_tokens
        || support.support_export_summary != row.support_export_summary
        || support.no_public_endpoint_fallback != row.no_public_endpoint_fallback
        || support.raw_private_material_excluded != row.raw_private_material_excluded
    {
        defects.push(row_defect(
            row,
            WorkspaceTrustBetaDefectKind::SupportRowVocabularyDrift,
            "support_row",
            "support/export row drifted from live row vocabulary",
        ));
    }
}

fn row_defect(
    row: &WorkspaceTrustBetaRow,
    kind: WorkspaceTrustBetaDefectKind,
    field: impl Into<String>,
    note: impl Into<String>,
) -> WorkspaceTrustBetaDefect {
    WorkspaceTrustBetaDefect::new(
        kind,
        row.row_id.clone(),
        row.surface_family_token.clone(),
        field,
        note,
    )
}

fn seed_row(family: LaunchWedgeCapabilityFamily) -> WorkspaceTrustBetaRow {
    let restricted_authority =
        authority_for_trust_state(RestrictedModeTrustStateClass::Restricted, family);
    let trusted_authority =
        authority_for_trust_state(RestrictedModeTrustStateClass::Trusted, family);
    let trusted_policy_degraded_authority =
        authority_for_trust_state(RestrictedModeTrustStateClass::TrustedPolicyDegraded, family);
    let trust_loss_authority =
        authority_for_trust_state(RestrictedModeTrustStateClass::TrustRevoked, family);
    let lane = lane_for(family);
    let floor_capability = family.restricted_floor_family();
    let run_or_mutation_capable = run_or_mutation_capable(family);
    let requires_explicit_trust_elevation =
        !floor_capability && restricted_authority.requires_explanation();
    let surface_family_token = family.as_str().to_owned();
    let escalation_cue_tokens = escalation_cues(restricted_authority)
        .into_iter()
        .map(|cue| cue.as_str().to_owned())
        .collect();
    let support_export_summary = format!(
        "{}: restricted={}, trusted={}, policy_degraded={}, trust_loss={}; no public fallback; raw private material excluded.",
        surface_family_token,
        restricted_authority.as_str(),
        trusted_authority.as_str(),
        trusted_policy_degraded_authority.as_str(),
        trust_loss_authority.as_str(),
    );

    WorkspaceTrustBetaRow {
        record_kind: WORKSPACE_TRUST_BETA_ROW_RECORD_KIND.to_owned(),
        schema_version: WORKSPACE_TRUST_BETA_SCHEMA_VERSION,
        shared_contract_ref: WORKSPACE_TRUST_BETA_SHARED_CONTRACT_REF.to_owned(),
        row_id: format!("workspace_trust_beta:{}", family.as_str()),
        lane,
        lane_token: lane.as_str().to_owned(),
        surface_family: family,
        surface_family_token: surface_family_token.clone(),
        source_matrix_ref: format!(
            "artifacts/security/trust_state_matrix.yaml#{}",
            family.as_str()
        ),
        restricted_authority,
        restricted_authority_token: restricted_authority.as_str().to_owned(),
        trusted_authority,
        trusted_authority_token: trusted_authority.as_str().to_owned(),
        trusted_policy_degraded_authority,
        trusted_policy_degraded_authority_token: trusted_policy_degraded_authority
            .as_str()
            .to_owned(),
        trust_loss_authority,
        trust_loss_authority_token: trust_loss_authority.as_str().to_owned(),
        profile_authorities: profile_authorities_for(
            family,
            restricted_authority,
            trusted_policy_degraded_authority,
        ),
        restricted_mode_available_before_trust: true,
        restricted_mode_explainer: restricted_explainer(family, restricted_authority),
        floor_capability,
        run_or_mutation_capable,
        requires_explicit_trust_elevation,
        trust_elevation_audit_event_token: TrustAuditEventClass::WorkspaceTrustGranted
            .as_str()
            .to_owned(),
        trust_loss_audit_event_token: TrustAuditEventClass::WorkspaceTrustRevoked
            .as_str()
            .to_owned(),
        trust_loss_propagates_to_shell: true,
        trust_loss_propagates_to_runtime: true,
        trust_loss_propagates_to_extension: true,
        trust_loss_propagates_to_support: true,
        escalation_cue_tokens,
        support_export_summary,
        no_public_endpoint_fallback: true,
        raw_private_material_excluded: true,
    }
}

fn profile_authorities_for(
    family: LaunchWedgeCapabilityFamily,
    restricted_authority: CapabilityAuthorityClass,
    trusted_policy_degraded_authority: CapabilityAuthorityClass,
) -> Vec<WorkspaceTrustBetaProfileAuthority> {
    WorkspaceTrustBetaProfileClass::ALL
        .into_iter()
        .map(|profile| {
            let authority = match profile {
                WorkspaceTrustBetaProfileClass::Connected => restricted_authority,
                WorkspaceTrustBetaProfileClass::MirrorOnly => {
                    mirror_or_offline_authority(family, restricted_authority)
                }
                WorkspaceTrustBetaProfileClass::Offline => {
                    mirror_or_offline_authority(family, restricted_authority)
                }
                WorkspaceTrustBetaProfileClass::EnterpriseManaged => {
                    trusted_policy_degraded_authority
                }
            };
            WorkspaceTrustBetaProfileAuthority {
                profile_class: profile,
                profile_token: profile.as_str().to_owned(),
                authority,
                authority_token: authority.as_str().to_owned(),
                source_label: profile_source_label(profile, family),
                no_public_endpoint_fallback: true,
            }
        })
        .collect()
}

fn mirror_or_offline_authority(
    family: LaunchWedgeCapabilityFamily,
    restricted_authority: CapabilityAuthorityClass,
) -> CapabilityAuthorityClass {
    match family {
        LaunchWedgeCapabilityFamily::ConnectedProviderOpen => {
            CapabilityAuthorityClass::DegradedPreviewOnly
        }
        LaunchWedgeCapabilityFamily::ConnectedProviderToolCall
        | LaunchWedgeCapabilityFamily::RemoteAttach
        | LaunchWedgeCapabilityFamily::McpServerLaunch => {
            CapabilityAuthorityClass::BlockedPendingTrust
        }
        _ => restricted_authority,
    }
}

fn profile_source_label(
    profile: WorkspaceTrustBetaProfileClass,
    family: LaunchWedgeCapabilityFamily,
) -> String {
    match profile {
        WorkspaceTrustBetaProfileClass::Connected => {
            format!(
                "Connected profile uses the matrix authority for {}.",
                family.as_str()
            )
        }
        WorkspaceTrustBetaProfileClass::MirrorOnly => {
            "Mirror-only profile uses declared mirrors and refuses public fallback.".to_owned()
        }
        WorkspaceTrustBetaProfileClass::Offline => {
            "Offline profile preserves local floor rows and keeps network/provider rows gated."
                .to_owned()
        }
        WorkspaceTrustBetaProfileClass::EnterpriseManaged => {
            "Enterprise-managed profile applies signed policy narrowing before dispatch.".to_owned()
        }
    }
}

fn lane_for(family: LaunchWedgeCapabilityFamily) -> WorkspaceTrustBetaLaneClass {
    match family {
        LaunchWedgeCapabilityFamily::WorkspaceOpenRestore
        | LaunchWedgeCapabilityFamily::EditorReadWrite
        | LaunchWedgeCapabilityFamily::SearchLocal
        | LaunchWedgeCapabilityFamily::LocalGitRead => WorkspaceTrustBetaLaneClass::Open,
        LaunchWedgeCapabilityFamily::ShellCommandPalette
        | LaunchWedgeCapabilityFamily::TasksRun
        | LaunchWedgeCapabilityFamily::TerminalManualOpen
        | LaunchWedgeCapabilityFamily::TerminalRepoRecipeLaunch
        | LaunchWedgeCapabilityFamily::EnvironmentActivatorRun
        | LaunchWedgeCapabilityFamily::McpServerLaunch => WorkspaceTrustBetaLaneClass::Run,
        LaunchWedgeCapabilityFamily::DebugLaunch
        | LaunchWedgeCapabilityFamily::NotebookKernelConnect
        | LaunchWedgeCapabilityFamily::NotebookCellExecute
        | LaunchWedgeCapabilityFamily::NotebookRichOutputRender => {
            WorkspaceTrustBetaLaneClass::Debug
        }
        LaunchWedgeCapabilityFamily::ExtensionActivation
        | LaunchWedgeCapabilityFamily::ExtensionInstall => WorkspaceTrustBetaLaneClass::Extension,
        LaunchWedgeCapabilityFamily::AiContextRead
        | LaunchWedgeCapabilityFamily::AiApplyMutation
        | LaunchWedgeCapabilityFamily::AiToolCallMutating => WorkspaceTrustBetaLaneClass::Ai,
        LaunchWedgeCapabilityFamily::ConnectedProviderOpen
        | LaunchWedgeCapabilityFamily::ConnectedProviderToolCall
        | LaunchWedgeCapabilityFamily::RemoteAttach => WorkspaceTrustBetaLaneClass::Provider,
        LaunchWedgeCapabilityFamily::LocalGitWrite
        | LaunchWedgeCapabilityFamily::ScaffoldTemplateRun
        | LaunchWedgeCapabilityFamily::PackageInstallHelper => WorkspaceTrustBetaLaneClass::Review,
        LaunchWedgeCapabilityFamily::SupportBundleExport => WorkspaceTrustBetaLaneClass::Support,
        LaunchWedgeCapabilityFamily::AdminPolicyRead => WorkspaceTrustBetaLaneClass::Admin,
    }
}

fn run_or_mutation_capable(family: LaunchWedgeCapabilityFamily) -> bool {
    matches!(
        family,
        LaunchWedgeCapabilityFamily::LocalGitWrite
            | LaunchWedgeCapabilityFamily::ShellCommandPalette
            | LaunchWedgeCapabilityFamily::TasksRun
            | LaunchWedgeCapabilityFamily::TerminalManualOpen
            | LaunchWedgeCapabilityFamily::TerminalRepoRecipeLaunch
            | LaunchWedgeCapabilityFamily::DebugLaunch
            | LaunchWedgeCapabilityFamily::NotebookKernelConnect
            | LaunchWedgeCapabilityFamily::NotebookCellExecute
            | LaunchWedgeCapabilityFamily::NotebookRichOutputRender
            | LaunchWedgeCapabilityFamily::AiApplyMutation
            | LaunchWedgeCapabilityFamily::AiToolCallMutating
            | LaunchWedgeCapabilityFamily::ExtensionActivation
            | LaunchWedgeCapabilityFamily::ExtensionInstall
            | LaunchWedgeCapabilityFamily::EnvironmentActivatorRun
            | LaunchWedgeCapabilityFamily::PackageInstallHelper
            | LaunchWedgeCapabilityFamily::ScaffoldTemplateRun
            | LaunchWedgeCapabilityFamily::ConnectedProviderOpen
            | LaunchWedgeCapabilityFamily::ConnectedProviderToolCall
            | LaunchWedgeCapabilityFamily::RemoteAttach
            | LaunchWedgeCapabilityFamily::McpServerLaunch
    )
}

fn escalation_cues(authority: CapabilityAuthorityClass) -> Vec<TrustRecoveryActionClass> {
    match authority {
        CapabilityAuthorityClass::Allowed | CapabilityAuthorityClass::ReadOnly => {
            vec![TrustRecoveryActionClass::ContinueRestrictedNoElevation]
        }
        CapabilityAuthorityClass::DegradedPreviewOnly => vec![
            TrustRecoveryActionClass::UseLocalReadOnlyAlternative,
            TrustRecoveryActionClass::RequestTrustGrantSessionOnly,
            TrustRecoveryActionClass::RouteToSupportBundleExport,
        ],
        CapabilityAuthorityClass::BlockedPendingApproval
        | CapabilityAuthorityClass::ApprovalRequiredPerInvocation => vec![
            TrustRecoveryActionClass::RequestApprovalTicket,
            TrustRecoveryActionClass::RouteToSupportBundleExport,
        ],
        CapabilityAuthorityClass::PolicyDenied => vec![
            TrustRecoveryActionClass::RequestAdminPolicyChange,
            TrustRecoveryActionClass::RouteToSupportBundleExport,
        ],
        CapabilityAuthorityClass::QuarantineDenied => {
            vec![TrustRecoveryActionClass::RouteToSupportBundleExport]
        }
        CapabilityAuthorityClass::BlockedPendingTrust => vec![
            TrustRecoveryActionClass::RequestTrustGrantSessionOnly,
            TrustRecoveryActionClass::ContinueRestrictedNoElevation,
        ],
        CapabilityAuthorityClass::NotApplicable => Vec::new(),
    }
}

fn restricted_explainer(
    family: LaunchWedgeCapabilityFamily,
    restricted_authority: CapabilityAuthorityClass,
) -> String {
    if family.restricted_floor_family() {
        return format!(
            "{} stays available in restricted mode under {} authority.",
            family.label(),
            restricted_authority.as_str()
        );
    }
    if family == LaunchWedgeCapabilityFamily::AiContextRead {
        return "AI context assembly is local and inspectable; provider dispatch stays on provider/tool-call rows.".to_owned();
    }
    format!(
        "{} is {} before trust; safe local work remains available.",
        family.label(),
        restricted_authority.as_str()
    )
}

fn authority_width(authority: CapabilityAuthorityClass) -> u8 {
    match authority {
        CapabilityAuthorityClass::NotApplicable
        | CapabilityAuthorityClass::PolicyDenied
        | CapabilityAuthorityClass::QuarantineDenied => 0,
        CapabilityAuthorityClass::BlockedPendingTrust => 1,
        CapabilityAuthorityClass::BlockedPendingApproval => 2,
        CapabilityAuthorityClass::DegradedPreviewOnly => 3,
        CapabilityAuthorityClass::ReadOnly => 4,
        CapabilityAuthorityClass::ApprovalRequiredPerInvocation => 5,
        CapabilityAuthorityClass::Allowed => 6,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_covers_every_claimed_surface_with_zero_defects() {
        let page = seeded_workspace_trust_beta_page();
        assert_eq!(page.rows.len(), WORKSPACE_TRUST_BETA_SURFACE_FAMILIES.len());
        assert_eq!(page.support_rows.len(), page.rows.len());
        assert!(page.defects.is_empty());
        validate_workspace_trust_beta_page(&page).expect("seeded page validates");
        assert!(page
            .rows
            .iter()
            .any(|row| row.surface_family == LaunchWedgeCapabilityFamily::TasksRun));
        assert!(page
            .rows
            .iter()
            .any(|row| row.surface_family == LaunchWedgeCapabilityFamily::AiApplyMutation));
        assert!(page.summary.lanes_present.contains(&"provider".to_owned()));
        assert!(page
            .summary
            .profiles_present
            .contains(&"offline".to_owned()));
    }

    #[test]
    fn validator_rejects_run_capable_allowed_before_trust() {
        let mut page = seeded_workspace_trust_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|row| row.surface_family == LaunchWedgeCapabilityFamily::TasksRun)
            .expect("tasks row");
        row.restricted_authority = CapabilityAuthorityClass::Allowed;
        row.restricted_authority_token = CapabilityAuthorityClass::Allowed.as_str().to_owned();
        let defects = audit_workspace_trust_beta_rows(&page.rows, &page.support_rows);
        assert!(defects.iter().any(|defect| defect.defect_kind
            == WorkspaceTrustBetaDefectKind::RunOrMutationAllowedBeforeTrust));
    }

    #[test]
    fn validator_rejects_support_row_drift() {
        let mut page = seeded_workspace_trust_beta_page();
        let support = page
            .support_rows
            .iter_mut()
            .find(|row| row.surface_family_token == "ai_apply_mutation")
            .expect("support row");
        support.restricted_authority_token = "allowed".to_owned();
        let defects = audit_workspace_trust_beta_rows(&page.rows, &page.support_rows);
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind
                == WorkspaceTrustBetaDefectKind::SupportRowVocabularyDrift));
    }
}
