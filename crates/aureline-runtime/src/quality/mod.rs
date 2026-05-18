//! Governed quality-profile, action, session, suppression, and baseline records.
//!
//! This module owns the runtime record family that lets format, organize-imports,
//! fix-all, scanner, suppression, baseline, CLI, review, and support surfaces
//! describe one effective quality profile and one preview/apply/revert contract.
//! It composes the language and editor action contracts without replacing them:
//! language providers may still produce diagnostics and code-action summaries,
//! while this module records the profile, governance, and session truth that
//! every quality surface must share.
//!
//! The machine-readable boundaries live under
//! [`/schemas/quality/`](../../../../schemas/quality/).

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version shared by the runtime quality governance record family.
pub const QUALITY_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for effective quality profiles.
pub const EFFECTIVE_QUALITY_PROFILE_RECORD_KIND: &str = "effective_quality_profile_record";

/// Stable record-kind tag for normalized quality-action proposals.
pub const QUALITY_ACTION_PROPOSAL_RECORD_KIND: &str = "quality_action_proposal_record";

/// Stable record-kind tag for quality sessions.
pub const QUALITY_SESSION_RECORD_KIND: &str = "quality_session_record";

/// Stable record-kind tag for governed suppression records.
pub const SUPPRESSION_RECORD_KIND: &str = "suppression_record";

/// Stable record-kind tag for governed baseline records.
pub const BASELINE_RECORD_KIND: &str = "baseline_record";

/// Stable record-kind tag for support-export quality governance packets.
pub const QUALITY_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "quality_governance_support_export_record";

/// Error returned when a governed quality record would violate the contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QualityGovernanceError {
    /// Profile resolution was requested without any source candidate.
    EmptyProfileSourceChain,
    /// Every profile source was unavailable, incompatible, or rejected.
    NoAdmissibleProfileSource,
    /// A governed record omitted an owner.
    MissingOwner,
    /// A governed record omitted an actor.
    MissingActor,
    /// A governed record omitted its reason summary.
    MissingReason,
    /// A governed record omitted evidence refs required for audit.
    MissingEvidence,
    /// A suppression attempted to become a hidden permanent toggle.
    HiddenPermanentSuppressionDenied,
    /// A baseline was requested without accepted finding refs.
    EmptyBaseline,
}

impl fmt::Display for QualityGovernanceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyProfileSourceChain => {
                write!(formatter, "quality profile resolution needs at least one source")
            }
            Self::NoAdmissibleProfileSource => {
                write!(formatter, "no quality profile source could be admitted")
            }
            Self::MissingOwner => write!(formatter, "governed quality record requires an owner"),
            Self::MissingActor => write!(formatter, "governed quality record requires an actor"),
            Self::MissingReason => write!(formatter, "governed quality record requires a reason"),
            Self::MissingEvidence => {
                write!(formatter, "governed quality record requires evidence refs")
            }
            Self::HiddenPermanentSuppressionDenied => write!(
                formatter,
                "suppression without expiry or policy-managed review would be a hidden permanent toggle"
            ),
            Self::EmptyBaseline => write!(formatter, "baseline record requires accepted findings"),
        }
    }
}

impl Error for QualityGovernanceError {}

/// Product surface that renders the quality profile or action contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualitySurfaceClass {
    /// Save-participant inspector or save-status UI.
    SaveParticipantUi,
    /// Problems panel, inline diagnostic details, or diagnostic drawer.
    Problems,
    /// Review packet, review sheet, or batch preview.
    Review,
    /// CLI explain or headless JSON output.
    CliExplain,
    /// Support export or support-center packet.
    SupportExport,
    /// Local CI or headless local quality run.
    LocalCi,
    /// Managed CI or provider-authoritative quality run.
    ManagedCi,
}

impl QualitySurfaceClass {
    /// Returns every surface that must be able to inspect the winning profile.
    pub const fn required_profile_inspection_surfaces() -> [Self; 5] {
        [
            Self::SaveParticipantUi,
            Self::Problems,
            Self::Review,
            Self::CliExplain,
            Self::SupportExport,
        ]
    }

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SaveParticipantUi => "save_participant_ui",
            Self::Problems => "problems",
            Self::Review => "review",
            Self::CliExplain => "cli_explain",
            Self::SupportExport => "support_export",
            Self::LocalCi => "local_ci",
            Self::ManagedCi => "managed_ci",
        }
    }
}

/// Target scope a quality profile, action, session, suppression, or baseline names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityTargetScopeClass {
    /// Current file.
    CurrentFile,
    /// Current selection or span.
    CurrentSelection,
    /// Current workspace root.
    CurrentRoot,
    /// Selected workset.
    SelectedWorkset,
    /// Entire workspace.
    Workspace,
    /// Review diff.
    ReviewDiff,
    /// Baseline family or accepted debt set.
    BaselineFamily,
    /// Release candidate or release-bearing scope.
    ReleaseCandidate,
}

impl QualityTargetScopeClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentFile => "current_file",
            Self::CurrentSelection => "current_selection",
            Self::CurrentRoot => "current_root",
            Self::SelectedWorkset => "selected_workset",
            Self::Workspace => "workspace",
            Self::ReviewDiff => "review_diff",
            Self::BaselineFamily => "baseline_family",
            Self::ReleaseCandidate => "release_candidate",
        }
    }
}

/// Ordered source layer used by the effective quality-profile resolver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityProfileSourceLayer {
    /// Policy lock, managed profile, or regulated profile source.
    PolicyLockOrManagedProfile,
    /// Per-command or per-session override.
    PerCommandOverride,
    /// Repository-owned quality configuration.
    RepositoryQualityConfig,
    /// Workspace quality settings.
    WorkspaceQualitySettings,
    /// Active workspace profile or imported profile override.
    WorkspaceProfileOverride,
    /// Remote, container, or managed target default.
    RemoteOrContainerDefault,
    /// User or portable profile default.
    UserOrProfileDefault,
    /// Tool-native config such as formatter, linter, or editorconfig input.
    ImportedToolConfig,
    /// Built-in fallback default.
    FallbackDefault,
    /// Auto-detected source with no stronger authority.
    SystemAutoDetection,
}

impl QualityProfileSourceLayer {
    /// Precedence rank where smaller values win.
    pub const fn precedence_rank(self) -> u16 {
        match self {
            Self::PolicyLockOrManagedProfile => 0,
            Self::PerCommandOverride => 1,
            Self::RepositoryQualityConfig => 2,
            Self::WorkspaceQualitySettings => 3,
            Self::WorkspaceProfileOverride => 4,
            Self::RemoteOrContainerDefault => 5,
            Self::UserOrProfileDefault => 6,
            Self::ImportedToolConfig => 7,
            Self::FallbackDefault => 8,
            Self::SystemAutoDetection => 9,
        }
    }

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyLockOrManagedProfile => "policy_lock_or_managed_profile",
            Self::PerCommandOverride => "per_command_override",
            Self::RepositoryQualityConfig => "repository_quality_config",
            Self::WorkspaceQualitySettings => "workspace_quality_settings",
            Self::WorkspaceProfileOverride => "workspace_profile_override",
            Self::RemoteOrContainerDefault => "remote_or_container_default",
            Self::UserOrProfileDefault => "user_or_profile_default",
            Self::ImportedToolConfig => "imported_tool_config",
            Self::FallbackDefault => "fallback_default",
            Self::SystemAutoDetection => "system_auto_detection",
        }
    }
}

/// How a profile source participated in resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityProfileSourceStateClass {
    /// Source won resolution.
    SelectedWinner,
    /// Source was valid but lost to a higher-precedence source.
    ShadowedByHigherPrecedence,
    /// Source was valid but policy overrode it.
    PolicyOverridden,
    /// Source was incompatible and downgraded.
    DowngradedIncompatible,
    /// Source was unavailable in the current environment.
    UnavailableInEnvironment,
    /// Source was invalid and rejected.
    RejectedInvalid,
    /// Source is imported read-only evidence.
    ImportedReadOnly,
    /// Source was detected as a default.
    DetectedDefault,
}

impl QualityProfileSourceStateClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedWinner => "selected_winner",
            Self::ShadowedByHigherPrecedence => "shadowed_by_higher_precedence",
            Self::PolicyOverridden => "policy_overridden",
            Self::DowngradedIncompatible => "downgraded_incompatible",
            Self::UnavailableInEnvironment => "unavailable_in_environment",
            Self::RejectedInvalid => "rejected_invalid",
            Self::ImportedReadOnly => "imported_read_only",
            Self::DetectedDefault => "detected_default",
        }
    }
}

/// Lock posture applied to a profile source or effective profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityLockStateClass {
    /// No lock applies.
    Unlocked,
    /// Admin or managed policy locked the profile.
    PolicyLocked,
    /// Policy constrained the profile without fully locking it.
    PolicyConstrained,
    /// Provider owns the authoritative answer for this scope.
    ProviderAuthoritative,
    /// Imported evidence is read-only.
    ReadOnlyImported,
    /// Capability loss locks the action or profile.
    CapabilityLocked,
}

impl QualityLockStateClass {
    /// Returns true when this lock state is policy-bearing.
    pub const fn is_policy_bearing(self) -> bool {
        matches!(self, Self::PolicyLocked | Self::PolicyConstrained)
    }

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unlocked => "unlocked",
            Self::PolicyLocked => "policy_locked",
            Self::PolicyConstrained => "policy_constrained",
            Self::ProviderAuthoritative => "provider_authoritative",
            Self::ReadOnlyImported => "read_only_imported",
            Self::CapabilityLocked => "capability_locked",
        }
    }
}

/// Reason a quality profile or action is locked or constrained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityLockReasonClass {
    /// No lock reason applies.
    None,
    /// Admin policy pins the tool.
    AdminPolicyPinsTool,
    /// Admin policy pins the rule pack.
    AdminPolicyPinsRulePack,
    /// Admin policy disables mutation.
    AdminPolicyDisablesMutation,
    /// Regulated profile requires a scanner or rule family.
    RegulatedProfileRequiresScanner,
    /// Workspace trust restricts mutation.
    WorkspaceTrustRestricted,
    /// Imported evidence is read-only.
    ImportedEvidenceReadOnly,
    /// Tool is missing or outside the admitted version range.
    ToolMissingOrVersionMismatch,
    /// Host or target cannot support the profile.
    UnsupportedHostOrTarget,
}

impl QualityLockReasonClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::AdminPolicyPinsTool => "admin_policy_pins_tool",
            Self::AdminPolicyPinsRulePack => "admin_policy_pins_rule_pack",
            Self::AdminPolicyDisablesMutation => "admin_policy_disables_mutation",
            Self::RegulatedProfileRequiresScanner => "regulated_profile_requires_scanner",
            Self::WorkspaceTrustRestricted => "workspace_trust_restricted",
            Self::ImportedEvidenceReadOnly => "imported_evidence_read_only",
            Self::ToolMissingOrVersionMismatch => "tool_missing_or_version_mismatch",
            Self::UnsupportedHostOrTarget => "unsupported_host_or_target",
        }
    }
}

/// Quality tool family bound by the profile or proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityToolFamilyClass {
    /// Formatter tool.
    Formatter,
    /// Linter or lint autofix tool.
    Linter,
    /// Organize-imports provider.
    OrganizeImports,
    /// Style checker.
    StyleChecker,
    /// Type checker.
    TypeChecker,
    /// Security scanner.
    SecurityScanner,
    /// Secret scanner.
    SecretScanner,
    /// Dependency scanner.
    DependencyScanner,
    /// License scanner.
    LicenseScanner,
    /// Policy checker.
    PolicyChecker,
    /// Mixed quality pack.
    MixedQualityPack,
}

impl QualityToolFamilyClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Formatter => "formatter",
            Self::Linter => "linter",
            Self::OrganizeImports => "organize_imports",
            Self::StyleChecker => "style_checker",
            Self::TypeChecker => "type_checker",
            Self::SecurityScanner => "security_scanner",
            Self::SecretScanner => "secret_scanner",
            Self::DependencyScanner => "dependency_scanner",
            Self::LicenseScanner => "license_scanner",
            Self::PolicyChecker => "policy_checker",
            Self::MixedQualityPack => "mixed_quality_pack",
        }
    }
}

/// Source candidate supplied to the effective quality-profile resolver.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualityProfileSourceCandidate {
    /// Source layer that determines precedence.
    pub source_layer: QualityProfileSourceLayer,
    /// Opaque source ref, such as a settings, config, or policy ref.
    pub source_ref: String,
    /// Candidate profile ref supplied by this source.
    pub candidate_profile_ref: String,
    /// Tool family this source affects.
    pub tool_family_class: QualityToolFamilyClass,
    /// Lock state declared by the source.
    pub lock_state_class: QualityLockStateClass,
    /// Lock reason declared by the source.
    pub lock_reason_class: QualityLockReasonClass,
    /// True when this source can be read in the current environment.
    pub available: bool,
    /// True when this source maps to the current target and tool family.
    pub compatible: bool,
    /// True when the source is imported evidence that cannot mutate locally.
    pub imported_read_only: bool,
    /// Imported or native config keys that could not be mapped.
    pub unmapped_key_count: usize,
    /// Keys that policy overrode.
    pub policy_overridden_key_count: usize,
    /// Export-safe source summary.
    pub summary: String,
}

/// Resolved source row retained in the effective profile precedence chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualityProfileSourceRow {
    /// Numeric precedence rank where lower wins.
    pub precedence_rank: u16,
    /// Source layer.
    pub source_layer: QualityProfileSourceLayer,
    /// Stable source-layer token.
    pub source_layer_token: String,
    /// Source resolution state.
    pub source_state_class: QualityProfileSourceStateClass,
    /// Stable source-state token.
    pub source_state_token: String,
    /// Opaque source ref.
    pub source_ref: String,
    /// Candidate profile ref supplied by this source.
    pub candidate_profile_ref: String,
    /// Tool family this source affects.
    pub tool_family_class: QualityToolFamilyClass,
    /// Stable tool-family token.
    pub tool_family_token: String,
    /// Lock state declared by the source.
    pub lock_state_class: QualityLockStateClass,
    /// Stable lock-state token.
    pub lock_state_token: String,
    /// Lock reason declared by the source.
    pub lock_reason_class: QualityLockReasonClass,
    /// Stable lock-reason token.
    pub lock_reason_token: String,
    /// Imported or native config keys that could not be mapped.
    pub unmapped_key_count: usize,
    /// Keys that policy overrode.
    pub policy_overridden_key_count: usize,
    /// Export-safe source summary.
    pub summary: String,
}

impl QualityProfileSourceRow {
    /// Returns true when this source row won the effective profile.
    pub const fn is_winner(&self) -> bool {
        matches!(
            self.source_state_class,
            QualityProfileSourceStateClass::SelectedWinner
        )
    }
}

/// Request passed to [`QualityProfileResolver`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualityProfileResolutionRequest {
    /// Effective profile id to mint.
    pub effective_profile_id: String,
    /// Workspace ref the profile is resolved for.
    pub workspace_ref: String,
    /// Target scope the profile covers.
    pub target_scope_class: QualityTargetScopeClass,
    /// Surfaces that must render this profile.
    pub surface_classes: Vec<QualitySurfaceClass>,
    /// Resolution timestamp.
    pub resolved_at: String,
    /// Candidate source chain.
    pub source_candidates: Vec<QualityProfileSourceCandidate>,
    /// Selected tool identity refs after resolution.
    pub selected_tool_refs: Vec<String>,
    /// Suppression policy refs active under the profile.
    pub suppression_policy_refs: Vec<String>,
    /// Baseline refs active under the profile.
    pub baseline_refs: Vec<String>,
    /// Stable fingerprint for the resolved profile.
    pub profile_fingerprint: String,
    /// Export-safe resolution summary.
    pub resolution_summary: String,
}

/// Surface-specific projection of an effective quality profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualityProfileSurfaceProjection {
    /// Surface that receives the projection.
    pub surface_class: QualitySurfaceClass,
    /// Stable surface token.
    pub surface_token: String,
    /// Effective profile ref shown on the surface.
    pub effective_profile_ref: String,
    /// Winning source ref shown on the surface.
    pub winning_source_ref: String,
    /// True when policy lock or constraint state must be visible.
    pub policy_state_visible: bool,
    /// True when source precedence details are inspectable from this surface.
    pub source_chain_inspectable: bool,
    /// Export-safe explanation for this surface.
    pub explanation_summary: String,
}

/// Effective quality profile shared by UI, CLI, review, support, and release lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveQualityProfile {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Effective profile id.
    pub effective_profile_id: String,
    /// Workspace ref the profile is resolved for.
    pub workspace_ref: String,
    /// Target scope the profile covers.
    pub target_scope_class: QualityTargetScopeClass,
    /// Stable target-scope token.
    pub target_scope_token: String,
    /// Resolution timestamp.
    pub resolved_at: String,
    /// Stable fingerprint for the resolved profile.
    pub profile_fingerprint: String,
    /// Winning source ref.
    pub winning_source_ref: String,
    /// Winning candidate profile ref.
    pub winning_profile_ref: String,
    /// Effective lock state.
    pub lock_state_class: QualityLockStateClass,
    /// Stable lock-state token.
    pub lock_state_token: String,
    /// Effective lock reason.
    pub lock_reason_class: QualityLockReasonClass,
    /// Stable lock-reason token.
    pub lock_reason_token: String,
    /// Ordered source precedence chain.
    pub source_chain: Vec<QualityProfileSourceRow>,
    /// Selected tool identity refs after resolution.
    pub selected_tool_refs: Vec<String>,
    /// Suppression policy refs active under the profile.
    pub suppression_policy_refs: Vec<String>,
    /// Baseline refs active under the profile.
    pub baseline_refs: Vec<String>,
    /// Surface projections proving the same profile is inspectable everywhere.
    pub surface_projections: Vec<QualityProfileSurfaceProjection>,
    /// True when any source carried unmapped config keys.
    pub has_unmapped_imported_config: bool,
    /// True when policy overrode at least one lower-layer key.
    pub has_policy_overrides: bool,
    /// Export-safe resolution summary.
    pub resolution_summary: String,
}

impl EffectiveQualityProfile {
    /// Returns the winning source row.
    pub fn winning_source(&self) -> Option<&QualityProfileSourceRow> {
        self.source_chain.iter().find(|row| row.is_winner())
    }

    /// Returns true when the named surface has a projection for this profile.
    pub fn is_visible_on_surface(&self, surface: QualitySurfaceClass) -> bool {
        self.surface_projections
            .iter()
            .any(|projection| projection.surface_class == surface)
    }

    /// Returns stable source-state tokens in precedence order.
    pub fn source_state_tokens(&self) -> Vec<&str> {
        self.source_chain
            .iter()
            .map(|row| row.source_state_class.as_str())
            .collect()
    }
}

/// Resolver for effective quality-profile records.
#[derive(Debug, Clone, Copy, Default)]
pub struct QualityProfileResolver;

impl QualityProfileResolver {
    /// Resolves an effective profile and preserves every source candidate.
    ///
    /// # Errors
    ///
    /// Returns [`QualityGovernanceError::EmptyProfileSourceChain`] when no
    /// candidates are supplied, or [`QualityGovernanceError::NoAdmissibleProfileSource`]
    /// when every candidate is unavailable or incompatible.
    pub fn resolve(
        &self,
        request: QualityProfileResolutionRequest,
    ) -> Result<EffectiveQualityProfile, QualityGovernanceError> {
        if request.source_candidates.is_empty() {
            return Err(QualityGovernanceError::EmptyProfileSourceChain);
        }

        let mut indexed_candidates = request
            .source_candidates
            .iter()
            .enumerate()
            .collect::<Vec<_>>();
        indexed_candidates
            .sort_by_key(|(idx, candidate)| (candidate.source_layer.precedence_rank(), *idx));

        let winner_index = indexed_candidates
            .iter()
            .find(|(_, candidate)| candidate.available && candidate.compatible)
            .map(|(idx, _)| *idx)
            .ok_or(QualityGovernanceError::NoAdmissibleProfileSource)?;
        let winner = &request.source_candidates[winner_index];
        let winner_rank = winner.source_layer.precedence_rank();
        let winner_is_policy = winner.source_layer
            == QualityProfileSourceLayer::PolicyLockOrManagedProfile
            || winner.lock_state_class.is_policy_bearing();

        let source_chain = indexed_candidates
            .into_iter()
            .map(|(idx, candidate)| {
                let source_state_class = if idx == winner_index {
                    QualityProfileSourceStateClass::SelectedWinner
                } else if !candidate.available {
                    QualityProfileSourceStateClass::UnavailableInEnvironment
                } else if !candidate.compatible {
                    QualityProfileSourceStateClass::DowngradedIncompatible
                } else if candidate.imported_read_only {
                    QualityProfileSourceStateClass::ImportedReadOnly
                } else if winner_is_policy && winner_rank < candidate.source_layer.precedence_rank()
                {
                    QualityProfileSourceStateClass::PolicyOverridden
                } else {
                    QualityProfileSourceStateClass::ShadowedByHigherPrecedence
                };

                QualityProfileSourceRow {
                    precedence_rank: candidate.source_layer.precedence_rank(),
                    source_layer: candidate.source_layer,
                    source_layer_token: candidate.source_layer.as_str().to_owned(),
                    source_state_class,
                    source_state_token: source_state_class.as_str().to_owned(),
                    source_ref: candidate.source_ref.clone(),
                    candidate_profile_ref: candidate.candidate_profile_ref.clone(),
                    tool_family_class: candidate.tool_family_class,
                    tool_family_token: candidate.tool_family_class.as_str().to_owned(),
                    lock_state_class: candidate.lock_state_class,
                    lock_state_token: candidate.lock_state_class.as_str().to_owned(),
                    lock_reason_class: candidate.lock_reason_class,
                    lock_reason_token: candidate.lock_reason_class.as_str().to_owned(),
                    unmapped_key_count: candidate.unmapped_key_count,
                    policy_overridden_key_count: candidate.policy_overridden_key_count,
                    summary: candidate.summary.clone(),
                }
            })
            .collect::<Vec<_>>();

        let has_unmapped_imported_config =
            source_chain.iter().any(|row| row.unmapped_key_count > 0);
        let has_policy_overrides = source_chain
            .iter()
            .any(|row| row.policy_overridden_key_count > 0)
            || source_chain.iter().any(|row| {
                row.source_state_class == QualityProfileSourceStateClass::PolicyOverridden
            });

        let effective_profile_ref = request.effective_profile_id.clone();
        let surface_projections = request
            .surface_classes
            .into_iter()
            .map(|surface_class| QualityProfileSurfaceProjection {
                surface_class,
                surface_token: surface_class.as_str().to_owned(),
                effective_profile_ref: effective_profile_ref.clone(),
                winning_source_ref: winner.source_ref.clone(),
                policy_state_visible: winner.lock_state_class.is_policy_bearing(),
                source_chain_inspectable: true,
                explanation_summary: request.resolution_summary.clone(),
            })
            .collect();

        Ok(EffectiveQualityProfile {
            record_kind: EFFECTIVE_QUALITY_PROFILE_RECORD_KIND.to_owned(),
            schema_version: QUALITY_GOVERNANCE_SCHEMA_VERSION,
            effective_profile_id: request.effective_profile_id,
            workspace_ref: request.workspace_ref,
            target_scope_class: request.target_scope_class,
            target_scope_token: request.target_scope_class.as_str().to_owned(),
            resolved_at: request.resolved_at,
            profile_fingerprint: request.profile_fingerprint,
            winning_source_ref: winner.source_ref.clone(),
            winning_profile_ref: winner.candidate_profile_ref.clone(),
            lock_state_class: winner.lock_state_class,
            lock_state_token: winner.lock_state_class.as_str().to_owned(),
            lock_reason_class: winner.lock_reason_class,
            lock_reason_token: winner.lock_reason_class.as_str().to_owned(),
            source_chain,
            selected_tool_refs: request.selected_tool_refs,
            suppression_policy_refs: request.suppression_policy_refs,
            baseline_refs: request.baseline_refs,
            surface_projections,
            has_unmapped_imported_config,
            has_policy_overrides,
            resolution_summary: request.resolution_summary,
        })
    }
}

/// Normalized quality action family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityActionClass {
    /// Format a selected range.
    FormatRange,
    /// Format the whole document.
    FormatDocument,
    /// Organize imports.
    OrganizeImports,
    /// Quick fix for one finding.
    QuickFixSingle,
    /// Fix all diagnostics for one rule.
    FixAllRule,
    /// Batch linter autofix.
    LintAutofixBatch,
    /// Create or renew a governed suppression.
    SuppressionProposal,
    /// Accept or update a governed baseline.
    BaselineUpdate,
    /// Read-only scanner run.
    ScannerReadOnly,
    /// Read-only validation or recheck.
    ValidationRecheck,
}

impl QualityActionClass {
    /// Returns true when this action may mutate code or governance state.
    pub const fn is_mutating(self) -> bool {
        !matches!(self, Self::ScannerReadOnly | Self::ValidationRecheck)
    }

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FormatRange => "format_range",
            Self::FormatDocument => "format_document",
            Self::OrganizeImports => "organize_imports",
            Self::QuickFixSingle => "quick_fix_single",
            Self::FixAllRule => "fix_all_rule",
            Self::LintAutofixBatch => "lint_autofix_batch",
            Self::SuppressionProposal => "suppression_proposal",
            Self::BaselineUpdate => "baseline_update",
            Self::ScannerReadOnly => "scanner_read_only",
            Self::ValidationRecheck => "validation_recheck",
        }
    }
}

/// Safety class assigned to a quality action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualitySafetyClass {
    /// Trivia-only change.
    TriviaSafe,
    /// Local syntax-level change.
    LocalSyntaxSafe,
    /// Semantic change within one file.
    SemanticLocal,
    /// Cross-file semantic change.
    CrossFileSemantic,
    /// Generated or protected path mutation.
    GeneratedOrProtected,
    /// Unknown, stale, or provider-ambiguous mutation.
    UnknownOrUnstable,
}

impl QualitySafetyClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TriviaSafe => "trivia_safe",
            Self::LocalSyntaxSafe => "local_syntax_safe",
            Self::SemanticLocal => "semantic_local",
            Self::CrossFileSemantic => "cross_file_semantic",
            Self::GeneratedOrProtected => "generated_or_protected",
            Self::UnknownOrUnstable => "unknown_or_unstable",
        }
    }
}

/// Coarse pre-run disclosure shown before a mutating quality action runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityActionDisclosureClass {
    /// Trivia-only action.
    TriviaOnly,
    /// Semantic action with bounded local scope.
    Semantic,
    /// Broad action such as multi-file, generated, or workspace mutation.
    Broad,
    /// Blocked action.
    Blocked,
    /// Policy-escalated action.
    PolicyEscalated,
    /// Read-only action with no mutation path.
    ReadOnly,
}

impl QualityActionDisclosureClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TriviaOnly => "trivia_only",
            Self::Semantic => "semantic",
            Self::Broad => "broad",
            Self::Blocked => "blocked",
            Self::PolicyEscalated => "policy_escalated",
            Self::ReadOnly => "read_only",
        }
    }
}

/// Mutation scope claimed by a quality action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityMutationScopeClass {
    /// No mutation; read-only action.
    NoMutationReadOnly,
    /// One anchor.
    SingleAnchor,
    /// Localized region in one file.
    SingleFileLocalized,
    /// Whole document in one file.
    SingleFileWholeDocument,
    /// Multiple files in one module.
    MultiFileSameModule,
    /// Multiple files across the workspace.
    MultiFileWorkspace,
    /// Generated family or generated companion.
    GeneratedFamily,
    /// Protected, policy-scoped, or repo-truth mutation.
    ProtectedOrPolicyScoped,
}

impl QualityMutationScopeClass {
    /// Returns true when this scope is broader than a local single-file edit.
    pub const fn is_broad(self) -> bool {
        matches!(
            self,
            Self::SingleFileWholeDocument
                | Self::MultiFileSameModule
                | Self::MultiFileWorkspace
                | Self::GeneratedFamily
                | Self::ProtectedOrPolicyScoped
        )
    }

    /// Returns true when this scope is policy-bearing.
    pub const fn is_policy_bearing(self) -> bool {
        matches!(self, Self::ProtectedOrPolicyScoped)
    }

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoMutationReadOnly => "no_mutation_read_only",
            Self::SingleAnchor => "single_anchor",
            Self::SingleFileLocalized => "single_file_localized",
            Self::SingleFileWholeDocument => "single_file_whole_document",
            Self::MultiFileSameModule => "multi_file_same_module",
            Self::MultiFileWorkspace => "multi_file_workspace",
            Self::GeneratedFamily => "generated_family",
            Self::ProtectedOrPolicyScoped => "protected_or_policy_scoped",
        }
    }
}

/// Preview requirement for a quality action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityPreviewRequirementClass {
    /// No preview required.
    NotRequired,
    /// Inline summary is enough.
    InlineSummary,
    /// Structured diff preview is required.
    StructuredDiff,
    /// Batch or scope preview is required.
    BatchScopePreview,
    /// Policy or repo-truth mutation preview is required.
    PolicyOrRepoMutationPreviewRequired,
    /// Issue link or typed review is required.
    IssueLinkOrTypedReviewRequired,
}

impl QualityPreviewRequirementClass {
    /// Returns true when apply must route through preview or review first.
    pub const fn requires_preview_first(self) -> bool {
        matches!(
            self,
            Self::StructuredDiff
                | Self::BatchScopePreview
                | Self::PolicyOrRepoMutationPreviewRequired
                | Self::IssueLinkOrTypedReviewRequired
        )
    }

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::InlineSummary => "inline_summary",
            Self::StructuredDiff => "structured_diff",
            Self::BatchScopePreview => "batch_scope_preview",
            Self::PolicyOrRepoMutationPreviewRequired => "policy_or_repo_mutation_preview_required",
            Self::IssueLinkOrTypedReviewRequired => "issue_link_or_typed_review_required",
        }
    }
}

/// Current apply posture for a quality action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityApplyPostureClass {
    /// Auto-apply is allowed.
    AutoApplyAllowed,
    /// Preview is required before apply.
    PreviewBeforeApply,
    /// User review blocks apply.
    BlockedPendingUserReview,
    /// Policy or trust review blocks apply.
    BlockedPendingPolicyOrTrust,
    /// Action is read-only.
    ReadOnlyAction,
}

impl QualityApplyPostureClass {
    /// Returns true when this posture blocks mutation.
    pub const fn blocks_apply(self) -> bool {
        matches!(
            self,
            Self::BlockedPendingUserReview | Self::BlockedPendingPolicyOrTrust
        )
    }

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AutoApplyAllowed => "auto_apply_allowed",
            Self::PreviewBeforeApply => "preview_before_apply",
            Self::BlockedPendingUserReview => "blocked_pending_user_review",
            Self::BlockedPendingPolicyOrTrust => "blocked_pending_policy_or_trust",
            Self::ReadOnlyAction => "read_only_action",
        }
    }
}

/// Rollback or revert boundary for a quality action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityRollbackBoundaryClass {
    /// No mutation occurs.
    NoMutation,
    /// Current buffer undo group can revert the action.
    CurrentBufferUndo,
    /// Single-file checkpoint can revert the action.
    SingleFileCheckpoint,
    /// Grouped workspace checkpoint can revert the action.
    GroupedWorkspaceCheckpoint,
    /// Policy audit trail is the only revert boundary.
    PolicyAuditOnly,
    /// Manual recovery is required.
    ManualRecoveryRequired,
}

impl QualityRollbackBoundaryClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoMutation => "no_mutation",
            Self::CurrentBufferUndo => "current_buffer_undo",
            Self::SingleFileCheckpoint => "single_file_checkpoint",
            Self::GroupedWorkspaceCheckpoint => "grouped_workspace_checkpoint",
            Self::PolicyAuditOnly => "policy_audit_only",
            Self::ManualRecoveryRequired => "manual_recovery_required",
        }
    }
}

/// Request used to normalize one quality-action proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualityActionProposalRequest {
    /// Proposal id to mint.
    pub proposal_id: String,
    /// Action class.
    pub action_class: QualityActionClass,
    /// Target scope.
    pub target_scope_class: QualityTargetScopeClass,
    /// Mutation scope.
    pub mutation_scope_class: QualityMutationScopeClass,
    /// Safety class.
    pub safety_class: QualitySafetyClass,
    /// Effective quality profile ref.
    pub effective_profile_ref: String,
    /// Triggering finding refs.
    pub triggering_finding_refs: Vec<String>,
    /// Rule refs behind the action.
    pub rule_refs: Vec<String>,
    /// Policy lock refs affecting the action.
    pub policy_lock_refs: Vec<String>,
    /// Affected file count.
    pub affected_file_count: usize,
    /// Affected anchor count.
    pub affected_anchor_count: usize,
    /// Generated path count.
    pub generated_path_count: usize,
    /// Protected path count.
    pub protected_path_count: usize,
    /// Blocked path count.
    pub blocked_path_count: usize,
    /// True when the semantic layer is current enough for mutation.
    pub semantic_current: bool,
    /// True when the effective profile is policy locked or constrained.
    pub profile_policy_locked: bool,
    /// Checkpoint ref, when one exists.
    pub checkpoint_ref: Option<String>,
    /// Preview ref, when one exists.
    pub preview_ref: Option<String>,
    /// Revert plan ref, when one exists.
    pub revert_plan_ref: Option<String>,
    /// Validation refs to run after apply.
    pub validation_refs: Vec<String>,
    /// Export-safe proposal summary.
    pub summary: String,
}

/// Normalized proposal used before applying any quality action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualityActionProposal {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Proposal id.
    pub proposal_id: String,
    /// Action class.
    pub action_class: QualityActionClass,
    /// Stable action token.
    pub action_token: String,
    /// Target scope.
    pub target_scope_class: QualityTargetScopeClass,
    /// Stable target-scope token.
    pub target_scope_token: String,
    /// Mutation scope.
    pub mutation_scope_class: QualityMutationScopeClass,
    /// Stable mutation-scope token.
    pub mutation_scope_token: String,
    /// Safety class.
    pub safety_class: QualitySafetyClass,
    /// Stable safety token.
    pub safety_token: String,
    /// Coarse pre-run disclosure class.
    pub disclosure_class: QualityActionDisclosureClass,
    /// Stable disclosure token.
    pub disclosure_token: String,
    /// Preview requirement.
    pub preview_requirement_class: QualityPreviewRequirementClass,
    /// Stable preview token.
    pub preview_requirement_token: String,
    /// Current apply posture.
    pub apply_posture_class: QualityApplyPostureClass,
    /// Stable apply-posture token.
    pub apply_posture_token: String,
    /// Rollback or revert boundary.
    pub rollback_boundary_class: QualityRollbackBoundaryClass,
    /// Stable rollback-boundary token.
    pub rollback_boundary_token: String,
    /// Effective quality profile ref.
    pub effective_profile_ref: String,
    /// Triggering finding refs.
    pub triggering_finding_refs: Vec<String>,
    /// Rule refs behind the action.
    pub rule_refs: Vec<String>,
    /// Policy lock refs affecting the action.
    pub policy_lock_refs: Vec<String>,
    /// Affected file count.
    pub affected_file_count: usize,
    /// Affected anchor count.
    pub affected_anchor_count: usize,
    /// Generated path count.
    pub generated_path_count: usize,
    /// Protected path count.
    pub protected_path_count: usize,
    /// Blocked path count.
    pub blocked_path_count: usize,
    /// Checkpoint ref, when one exists.
    pub checkpoint_ref: Option<String>,
    /// Preview ref, when one exists.
    pub preview_ref: Option<String>,
    /// Revert plan ref, when one exists.
    pub revert_plan_ref: Option<String>,
    /// Validation refs to run after apply.
    pub validation_refs: Vec<String>,
    /// True when preview or typed review must occur before apply.
    pub preview_first_required: bool,
    /// True when direct apply is blocked.
    pub apply_blocked: bool,
    /// Export-safe proposal summary.
    pub summary: String,
}

impl QualityActionProposal {
    /// Builds a normalized quality-action proposal from pre-run facts.
    pub fn from_request(request: QualityActionProposalRequest) -> Self {
        let disclosure_class = derive_disclosure_class(&request);
        let preview_requirement_class = derive_preview_requirement(&request, disclosure_class);
        let apply_posture_class = derive_apply_posture(&request, disclosure_class);
        let rollback_boundary_class = derive_rollback_boundary(&request, disclosure_class);

        Self {
            record_kind: QUALITY_ACTION_PROPOSAL_RECORD_KIND.to_owned(),
            schema_version: QUALITY_GOVERNANCE_SCHEMA_VERSION,
            proposal_id: request.proposal_id,
            action_class: request.action_class,
            action_token: request.action_class.as_str().to_owned(),
            target_scope_class: request.target_scope_class,
            target_scope_token: request.target_scope_class.as_str().to_owned(),
            mutation_scope_class: request.mutation_scope_class,
            mutation_scope_token: request.mutation_scope_class.as_str().to_owned(),
            safety_class: request.safety_class,
            safety_token: request.safety_class.as_str().to_owned(),
            disclosure_class,
            disclosure_token: disclosure_class.as_str().to_owned(),
            preview_requirement_class,
            preview_requirement_token: preview_requirement_class.as_str().to_owned(),
            apply_posture_class,
            apply_posture_token: apply_posture_class.as_str().to_owned(),
            rollback_boundary_class,
            rollback_boundary_token: rollback_boundary_class.as_str().to_owned(),
            effective_profile_ref: request.effective_profile_ref,
            triggering_finding_refs: request.triggering_finding_refs,
            rule_refs: request.rule_refs,
            policy_lock_refs: request.policy_lock_refs,
            affected_file_count: request.affected_file_count,
            affected_anchor_count: request.affected_anchor_count,
            generated_path_count: request.generated_path_count,
            protected_path_count: request.protected_path_count,
            blocked_path_count: request.blocked_path_count,
            checkpoint_ref: request.checkpoint_ref,
            preview_ref: request.preview_ref,
            revert_plan_ref: request.revert_plan_ref,
            validation_refs: request.validation_refs,
            preview_first_required: preview_requirement_class.requires_preview_first(),
            apply_blocked: apply_posture_class.blocks_apply(),
            summary: request.summary,
        }
    }

    /// Returns true when the proposal mutates code or governance truth.
    pub fn is_mutating(&self) -> bool {
        self.action_class.is_mutating()
    }
}

/// Trigger that opened a quality session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualitySessionTriggerClass {
    /// On-save participant pipeline.
    OnSave,
    /// Manual desktop command.
    ManualCommand,
    /// CLI or headless command.
    CliHeadless,
    /// Review packet or batch preview.
    Review,
    /// Local CI run.
    LocalCi,
    /// Managed CI or provider run.
    ManagedCi,
    /// Support replay.
    SupportReplay,
}

impl QualitySessionTriggerClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnSave => "on_save",
            Self::ManualCommand => "manual_command",
            Self::CliHeadless => "cli_headless",
            Self::Review => "review",
            Self::LocalCi => "local_ci",
            Self::ManagedCi => "managed_ci",
            Self::SupportReplay => "support_replay",
        }
    }
}

/// Outcome for a quality session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualitySessionOutcomeClass {
    /// Session applied every admitted mutation or completed read-only work.
    Applied,
    /// Preview is required before apply.
    PreviewRequired,
    /// Session skipped because no proposal could run.
    Skipped,
    /// Session timed out.
    TimedOut,
    /// Session requires rebase before apply.
    RebaseRequired,
    /// Policy blocked the session.
    BlockedByPolicy,
    /// Session failed.
    Failed,
    /// Session reverted a prior apply.
    Reverted,
}

impl QualitySessionOutcomeClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Applied => "applied",
            Self::PreviewRequired => "preview_required",
            Self::Skipped => "skipped",
            Self::TimedOut => "timed_out",
            Self::RebaseRequired => "rebase_required",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::Failed => "failed",
            Self::Reverted => "reverted",
        }
    }
}

/// Request used to build a quality session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualitySessionRequest {
    /// Session id to mint.
    pub session_id: String,
    /// Session trigger.
    pub trigger_class: QualitySessionTriggerClass,
    /// Target scope.
    pub target_scope_class: QualityTargetScopeClass,
    /// Effective profile ref used by the session.
    pub effective_profile_ref: String,
    /// Execution-context ref used by tools, when known.
    pub execution_context_ref: Option<String>,
    /// Session start timestamp.
    pub started_at: String,
    /// Session end timestamp.
    pub ended_at: Option<String>,
    /// Proposals considered in this session.
    pub proposals: Vec<QualityActionProposal>,
    /// Validation refs produced by the session.
    pub validation_refs: Vec<String>,
    /// Rollback or revert refs produced by the session.
    pub rollback_refs: Vec<String>,
    /// Export-safe session summary.
    pub summary: String,
}

/// Runtime quality session shared by UI, CLI, review, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualitySession {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Session id.
    pub session_id: String,
    /// Session trigger.
    pub trigger_class: QualitySessionTriggerClass,
    /// Stable trigger token.
    pub trigger_token: String,
    /// Target scope.
    pub target_scope_class: QualityTargetScopeClass,
    /// Stable target-scope token.
    pub target_scope_token: String,
    /// Effective profile ref used by the session.
    pub effective_profile_ref: String,
    /// Execution-context ref used by tools, when known.
    pub execution_context_ref: Option<String>,
    /// Session start timestamp.
    pub started_at: String,
    /// Session end timestamp.
    pub ended_at: Option<String>,
    /// Proposal refs considered in this session.
    pub proposal_refs: Vec<String>,
    /// Proposals considered in this session.
    pub proposals: Vec<QualityActionProposal>,
    /// Derived session outcome.
    pub outcome_class: QualitySessionOutcomeClass,
    /// Stable outcome token.
    pub outcome_token: String,
    /// Validation refs produced by the session.
    pub validation_refs: Vec<String>,
    /// Rollback or revert refs produced by the session.
    pub rollback_refs: Vec<String>,
    /// True when every surface must show preview before apply.
    pub any_preview_first_required: bool,
    /// True when any proposal blocks apply.
    pub any_apply_blocked: bool,
    /// Export-safe session summary.
    pub summary: String,
}

impl QualitySession {
    /// Builds a quality session and derives its outcome from proposal posture.
    pub fn from_request(request: QualitySessionRequest) -> Self {
        let proposal_refs = request
            .proposals
            .iter()
            .map(|proposal| proposal.proposal_id.clone())
            .collect::<Vec<_>>();
        let any_preview_first_required = request
            .proposals
            .iter()
            .any(|proposal| proposal.preview_first_required);
        let any_apply_blocked = request
            .proposals
            .iter()
            .any(|proposal| proposal.apply_blocked);
        let policy_blocked = request.proposals.iter().any(|proposal| {
            proposal.apply_posture_class == QualityApplyPostureClass::BlockedPendingPolicyOrTrust
        });
        let outcome_class = if request.proposals.is_empty() {
            QualitySessionOutcomeClass::Skipped
        } else if policy_blocked {
            QualitySessionOutcomeClass::BlockedByPolicy
        } else if any_apply_blocked {
            QualitySessionOutcomeClass::Failed
        } else if any_preview_first_required {
            QualitySessionOutcomeClass::PreviewRequired
        } else {
            QualitySessionOutcomeClass::Applied
        };

        Self {
            record_kind: QUALITY_SESSION_RECORD_KIND.to_owned(),
            schema_version: QUALITY_GOVERNANCE_SCHEMA_VERSION,
            session_id: request.session_id,
            trigger_class: request.trigger_class,
            trigger_token: request.trigger_class.as_str().to_owned(),
            target_scope_class: request.target_scope_class,
            target_scope_token: request.target_scope_class.as_str().to_owned(),
            effective_profile_ref: request.effective_profile_ref,
            execution_context_ref: request.execution_context_ref,
            started_at: request.started_at,
            ended_at: request.ended_at,
            proposal_refs,
            proposals: request.proposals,
            outcome_class,
            outcome_token: outcome_class.as_str().to_owned(),
            validation_refs: request.validation_refs,
            rollback_refs: request.rollback_refs,
            any_preview_first_required,
            any_apply_blocked,
            summary: request.summary,
        }
    }
}

/// Governance boundary that a suppression or baseline mutates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityTruthMutationClass {
    /// Local session visibility only.
    LocalSessionVisibilityOnly,
    /// Workspace repository artifact.
    WorkspaceRepoArtifact,
    /// Managed policy artifact.
    ManagedPolicyArtifact,
    /// Imported baseline artifact.
    ImportedBaselineArtifact,
}

impl QualityTruthMutationClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalSessionVisibilityOnly => "local_session_visibility_only",
            Self::WorkspaceRepoArtifact => "workspace_repo_artifact",
            Self::ManagedPolicyArtifact => "managed_policy_artifact",
            Self::ImportedBaselineArtifact => "imported_baseline_artifact",
        }
    }
}

/// Policy edit state for suppression and baseline records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityPolicyLockStateClass {
    /// Local users may edit the record.
    EditableLocal,
    /// Record may be edited only through review.
    EditableWithReview,
    /// Policy makes the record read-only.
    ReadOnlyPolicyLocked,
    /// Policy owns expiry or review timing.
    ExpiryManagedByPolicy,
}

impl QualityPolicyLockStateClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditableLocal => "editable_local",
            Self::EditableWithReview => "editable_with_review",
            Self::ReadOnlyPolicyLocked => "read_only_policy_locked",
            Self::ExpiryManagedByPolicy => "expiry_managed_by_policy",
        }
    }
}

/// Reopen behavior for governed debt records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityReopenRuleClass {
    /// Reopen when expiry passes.
    ReopenOnExpiry,
    /// Reopen when rule metadata changes.
    ReopenOnRuleChange,
    /// Reopen when anchor remap fails.
    ReopenOnAnchorRemapFailure,
    /// Reopen when profile or target drifts.
    ReopenOnProfileOrTargetDrift,
    /// Reopen only through manual review.
    ManualReviewOnly,
}

impl QualityReopenRuleClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReopenOnExpiry => "reopen_on_expiry",
            Self::ReopenOnRuleChange => "reopen_on_rule_change",
            Self::ReopenOnAnchorRemapFailure => "reopen_on_anchor_remap_failure",
            Self::ReopenOnProfileOrTargetDrift => "reopen_on_profile_or_target_drift",
            Self::ManualReviewOnly => "manual_review_only",
        }
    }
}

/// Owner class for governed quality debt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityOwnerClass {
    /// Workspace owner.
    WorkspaceOwner,
    /// Codebase team owner.
    CodebaseTeamOwner,
    /// Security owner.
    SecurityOwner,
    /// Admin policy owner.
    AdminPolicyOwner,
    /// Imported baseline owner.
    ImportedBaselineOwner,
}

impl QualityOwnerClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceOwner => "workspace_owner",
            Self::CodebaseTeamOwner => "codebase_team_owner",
            Self::SecurityOwner => "security_owner",
            Self::AdminPolicyOwner => "admin_policy_owner",
            Self::ImportedBaselineOwner => "imported_baseline_owner",
        }
    }
}

/// Actor class for governed quality debt mutations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityActorClass {
    /// Local user.
    LocalUser,
    /// Collaboration remote user.
    CollaborationRemoteUser,
    /// Automation recipe.
    AutomationRecipe,
    /// Admin policy service.
    AdminPolicyService,
    /// Import session actor.
    ImportSessionActor,
}

impl QualityActorClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalUser => "local_user",
            Self::CollaborationRemoteUser => "collaboration_remote_user",
            Self::AutomationRecipe => "automation_recipe",
            Self::AdminPolicyService => "admin_policy_service",
            Self::ImportSessionActor => "import_session_actor",
        }
    }
}

/// Reopen state computed for a governed debt record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityDebtReopenStateClass {
    /// Record remains active.
    Active,
    /// Record has expired and must reopen.
    ExpiredReopened,
    /// Profile or target drift requires reopen.
    ReopenedForProfileOrTargetDrift,
    /// Manual review is required before reopening.
    ManualReviewRequired,
}

impl QualityDebtReopenStateClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::ExpiredReopened => "expired_reopened",
            Self::ReopenedForProfileOrTargetDrift => "reopened_for_profile_or_target_drift",
            Self::ManualReviewRequired => "manual_review_required",
        }
    }
}

/// Request used to mint a governed suppression record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuppressionRecordRequest {
    /// Suppression id.
    pub suppression_id: String,
    /// Target scope.
    pub scope_class: QualityTargetScopeClass,
    /// Rule refs covered by the suppression.
    pub rule_refs: Vec<String>,
    /// Finding refs covered by the suppression.
    pub finding_refs: Vec<String>,
    /// Governance boundary this suppression mutates.
    pub truth_mutation_class: QualityTruthMutationClass,
    /// Policy edit state.
    pub policy_lock_state_class: QualityPolicyLockStateClass,
    /// Owner class.
    pub owner_class: QualityOwnerClass,
    /// Owner ref.
    pub owner_ref: String,
    /// Actor class.
    pub actor_class: QualityActorClass,
    /// Actor ref.
    pub actor_ref: String,
    /// Created-at timestamp.
    pub created_at: String,
    /// Expiry timestamp, unless policy owns expiry.
    pub expires_at: Option<String>,
    /// Reason summary.
    pub reason: String,
    /// Evidence refs.
    pub evidence_refs: Vec<String>,
    /// Reopen rule.
    pub reopen_rule_class: QualityReopenRuleClass,
    /// Release-visible debt flag.
    pub release_visible: bool,
    /// Export-safe summary.
    pub summary: String,
}

/// Governed suppression record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuppressionRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Suppression id.
    pub suppression_id: String,
    /// Target scope.
    pub scope_class: QualityTargetScopeClass,
    /// Stable scope token.
    pub scope_token: String,
    /// Rule refs covered by the suppression.
    pub rule_refs: Vec<String>,
    /// Finding refs covered by the suppression.
    pub finding_refs: Vec<String>,
    /// Governance boundary this suppression mutates.
    pub truth_mutation_class: QualityTruthMutationClass,
    /// Stable truth-mutation token.
    pub truth_mutation_token: String,
    /// Policy edit state.
    pub policy_lock_state_class: QualityPolicyLockStateClass,
    /// Stable policy-lock token.
    pub policy_lock_state_token: String,
    /// Owner class.
    pub owner_class: QualityOwnerClass,
    /// Stable owner token.
    pub owner_token: String,
    /// Owner ref.
    pub owner_ref: String,
    /// Actor class.
    pub actor_class: QualityActorClass,
    /// Stable actor token.
    pub actor_token: String,
    /// Actor ref.
    pub actor_ref: String,
    /// Created-at timestamp.
    pub created_at: String,
    /// Expiry timestamp, unless policy owns expiry.
    pub expires_at: Option<String>,
    /// Reason summary.
    pub reason: String,
    /// Evidence refs.
    pub evidence_refs: Vec<String>,
    /// Reopen rule.
    pub reopen_rule_class: QualityReopenRuleClass,
    /// Stable reopen-rule token.
    pub reopen_rule_token: String,
    /// Release-visible debt flag.
    pub release_visible: bool,
    /// True because hidden permanent toggles are denied by construction.
    pub hidden_permanent_toggle_denied: bool,
    /// Export-safe summary.
    pub summary: String,
}

impl SuppressionRecord {
    /// Builds a suppression record after governance validation.
    ///
    /// # Errors
    ///
    /// Returns a [`QualityGovernanceError`] when owner, actor, reason, evidence,
    /// or expiry governance is missing.
    pub fn from_request(request: SuppressionRecordRequest) -> Result<Self, QualityGovernanceError> {
        validate_non_empty_ref(&request.owner_ref, QualityGovernanceError::MissingOwner)?;
        validate_non_empty_ref(&request.actor_ref, QualityGovernanceError::MissingActor)?;
        validate_non_empty_ref(&request.reason, QualityGovernanceError::MissingReason)?;
        if request.evidence_refs.is_empty() {
            return Err(QualityGovernanceError::MissingEvidence);
        }
        if request.expires_at.is_none()
            && request.policy_lock_state_class != QualityPolicyLockStateClass::ExpiryManagedByPolicy
        {
            return Err(QualityGovernanceError::HiddenPermanentSuppressionDenied);
        }

        Ok(Self {
            record_kind: SUPPRESSION_RECORD_KIND.to_owned(),
            schema_version: QUALITY_GOVERNANCE_SCHEMA_VERSION,
            suppression_id: request.suppression_id,
            scope_class: request.scope_class,
            scope_token: request.scope_class.as_str().to_owned(),
            rule_refs: request.rule_refs,
            finding_refs: request.finding_refs,
            truth_mutation_class: request.truth_mutation_class,
            truth_mutation_token: request.truth_mutation_class.as_str().to_owned(),
            policy_lock_state_class: request.policy_lock_state_class,
            policy_lock_state_token: request.policy_lock_state_class.as_str().to_owned(),
            owner_class: request.owner_class,
            owner_token: request.owner_class.as_str().to_owned(),
            owner_ref: request.owner_ref,
            actor_class: request.actor_class,
            actor_token: request.actor_class.as_str().to_owned(),
            actor_ref: request.actor_ref,
            created_at: request.created_at,
            expires_at: request.expires_at,
            reason: request.reason,
            evidence_refs: request.evidence_refs,
            reopen_rule_class: request.reopen_rule_class,
            reopen_rule_token: request.reopen_rule_class.as_str().to_owned(),
            release_visible: request.release_visible,
            hidden_permanent_toggle_denied: true,
            summary: request.summary,
        })
    }

    /// Computes whether this suppression reopens at an ISO-8601 timestamp.
    pub fn reopen_state_at(&self, observed_at: &str) -> QualityDebtReopenStateClass {
        match (&self.expires_at, self.reopen_rule_class) {
            (Some(expires_at), QualityReopenRuleClass::ReopenOnExpiry)
                if expires_at.as_str() <= observed_at =>
            {
                QualityDebtReopenStateClass::ExpiredReopened
            }
            (_, QualityReopenRuleClass::ManualReviewOnly) => {
                QualityDebtReopenStateClass::ManualReviewRequired
            }
            _ => QualityDebtReopenStateClass::Active,
        }
    }
}

/// Compatibility state for a baseline record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BaselineCompatibilityStateClass {
    /// Baseline is compatible with the current profile and target.
    Compatible,
    /// Tool or rule-pack drift blocks comparison.
    ToolOrRulePackDriftBlocked,
    /// Effective profile drift blocks comparison.
    ProfileDriftBlocked,
    /// Target drift blocks comparison.
    TargetDriftBlocked,
    /// Compatibility has not been checked.
    UnknownRequiresReview,
}

impl BaselineCompatibilityStateClass {
    /// Returns true when comparison must be blocked.
    pub const fn blocks_comparison(self) -> bool {
        !matches!(self, Self::Compatible)
    }

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compatible => "compatible",
            Self::ToolOrRulePackDriftBlocked => "tool_or_rule_pack_drift_blocked",
            Self::ProfileDriftBlocked => "profile_drift_blocked",
            Self::TargetDriftBlocked => "target_drift_blocked",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }
}

/// Request used to mint a governed baseline record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaselineRecordRequest {
    /// Baseline id.
    pub baseline_id: String,
    /// Compatible profile family ref.
    pub compatible_profile_family_ref: String,
    /// Target scope ref.
    pub target_scope_ref: String,
    /// Target scope class.
    pub target_scope_class: QualityTargetScopeClass,
    /// Accepted finding refs.
    pub accepted_finding_refs: Vec<String>,
    /// Created-at timestamp.
    pub created_at: String,
    /// Actor class.
    pub actor_class: QualityActorClass,
    /// Actor ref.
    pub actor_ref: String,
    /// Owner class.
    pub owner_class: QualityOwnerClass,
    /// Owner ref.
    pub owner_ref: String,
    /// Evidence refs.
    pub evidence_refs: Vec<String>,
    /// Review refs.
    pub review_refs: Vec<String>,
    /// Baseline refs superseded by this record.
    pub supersedes_refs: Vec<String>,
    /// Compatibility state.
    pub compatibility_state_class: BaselineCompatibilityStateClass,
    /// Policy edit state.
    pub policy_lock_state_class: QualityPolicyLockStateClass,
    /// Reopen rule.
    pub reopen_rule_class: QualityReopenRuleClass,
    /// Release-visible debt flag.
    pub release_visible: bool,
    /// Export-safe summary.
    pub summary: String,
}

/// Governed baseline record used for compatible debt comparison.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaselineRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Baseline id.
    pub baseline_id: String,
    /// Compatible profile family ref.
    pub compatible_profile_family_ref: String,
    /// Target scope ref.
    pub target_scope_ref: String,
    /// Target scope class.
    pub target_scope_class: QualityTargetScopeClass,
    /// Stable target-scope token.
    pub target_scope_token: String,
    /// Accepted finding refs.
    pub accepted_finding_refs: Vec<String>,
    /// Created-at timestamp.
    pub created_at: String,
    /// Actor class.
    pub actor_class: QualityActorClass,
    /// Stable actor token.
    pub actor_token: String,
    /// Actor ref.
    pub actor_ref: String,
    /// Owner class.
    pub owner_class: QualityOwnerClass,
    /// Stable owner token.
    pub owner_token: String,
    /// Owner ref.
    pub owner_ref: String,
    /// Evidence refs.
    pub evidence_refs: Vec<String>,
    /// Review refs.
    pub review_refs: Vec<String>,
    /// Baseline refs superseded by this record.
    pub supersedes_refs: Vec<String>,
    /// Compatibility state.
    pub compatibility_state_class: BaselineCompatibilityStateClass,
    /// Stable compatibility token.
    pub compatibility_state_token: String,
    /// Policy edit state.
    pub policy_lock_state_class: QualityPolicyLockStateClass,
    /// Stable policy-lock token.
    pub policy_lock_state_token: String,
    /// Reopen rule.
    pub reopen_rule_class: QualityReopenRuleClass,
    /// Stable reopen-rule token.
    pub reopen_rule_token: String,
    /// Release-visible debt flag.
    pub release_visible: bool,
    /// Export-safe summary.
    pub summary: String,
}

impl BaselineRecord {
    /// Builds a baseline record after governance validation.
    ///
    /// # Errors
    ///
    /// Returns a [`QualityGovernanceError`] when owner, actor, evidence, or
    /// accepted findings are missing.
    pub fn from_request(request: BaselineRecordRequest) -> Result<Self, QualityGovernanceError> {
        validate_non_empty_ref(&request.owner_ref, QualityGovernanceError::MissingOwner)?;
        validate_non_empty_ref(&request.actor_ref, QualityGovernanceError::MissingActor)?;
        if request.evidence_refs.is_empty() {
            return Err(QualityGovernanceError::MissingEvidence);
        }
        if request.accepted_finding_refs.is_empty() {
            return Err(QualityGovernanceError::EmptyBaseline);
        }

        Ok(Self {
            record_kind: BASELINE_RECORD_KIND.to_owned(),
            schema_version: QUALITY_GOVERNANCE_SCHEMA_VERSION,
            baseline_id: request.baseline_id,
            compatible_profile_family_ref: request.compatible_profile_family_ref,
            target_scope_ref: request.target_scope_ref,
            target_scope_class: request.target_scope_class,
            target_scope_token: request.target_scope_class.as_str().to_owned(),
            accepted_finding_refs: request.accepted_finding_refs,
            created_at: request.created_at,
            actor_class: request.actor_class,
            actor_token: request.actor_class.as_str().to_owned(),
            actor_ref: request.actor_ref,
            owner_class: request.owner_class,
            owner_token: request.owner_class.as_str().to_owned(),
            owner_ref: request.owner_ref,
            evidence_refs: request.evidence_refs,
            review_refs: request.review_refs,
            supersedes_refs: request.supersedes_refs,
            compatibility_state_class: request.compatibility_state_class,
            compatibility_state_token: request.compatibility_state_class.as_str().to_owned(),
            policy_lock_state_class: request.policy_lock_state_class,
            policy_lock_state_token: request.policy_lock_state_class.as_str().to_owned(),
            reopen_rule_class: request.reopen_rule_class,
            reopen_rule_token: request.reopen_rule_class.as_str().to_owned(),
            release_visible: request.release_visible,
            summary: request.summary,
        })
    }

    /// Returns true when this baseline cannot compare in the current state.
    pub fn blocks_comparison(&self) -> bool {
        self.compatibility_state_class.blocks_comparison()
    }

    /// Computes reopen state from compatibility drift.
    pub fn reopen_state_for_comparison(&self) -> QualityDebtReopenStateClass {
        match (self.compatibility_state_class, self.reopen_rule_class) {
            (
                BaselineCompatibilityStateClass::ProfileDriftBlocked
                | BaselineCompatibilityStateClass::TargetDriftBlocked,
                QualityReopenRuleClass::ReopenOnProfileOrTargetDrift,
            ) => QualityDebtReopenStateClass::ReopenedForProfileOrTargetDrift,
            (_, QualityReopenRuleClass::ManualReviewOnly) => {
                QualityDebtReopenStateClass::ManualReviewRequired
            }
            _ => QualityDebtReopenStateClass::Active,
        }
    }
}

/// Support-export packet for quality governance records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualityGovernanceSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Support export id.
    pub support_export_id: String,
    /// Export timestamp.
    pub generated_at: String,
    /// Effective profile refs included in the export.
    pub effective_profile_refs: Vec<String>,
    /// Quality session refs included in the export.
    pub quality_session_refs: Vec<String>,
    /// Suppression refs included in the export.
    pub suppression_refs: Vec<String>,
    /// Baseline refs included in the export.
    pub baseline_refs: Vec<String>,
    /// Effective profile records included in the export.
    pub effective_profiles: Vec<EffectiveQualityProfile>,
    /// Quality sessions included in the export.
    pub quality_sessions: Vec<QualitySession>,
    /// Suppression records included in the export.
    pub suppressions: Vec<SuppressionRecord>,
    /// Baseline records included in the export.
    pub baselines: Vec<BaselineRecord>,
    /// True because raw code, raw logs, raw tool args, and raw paths are absent.
    pub redaction_safe: bool,
    /// Export-safe summary.
    pub summary: String,
}

impl QualityGovernanceSupportExport {
    /// Builds a support-export packet from governed quality records.
    pub fn from_records(
        support_export_id: impl Into<String>,
        generated_at: impl Into<String>,
        effective_profiles: Vec<EffectiveQualityProfile>,
        quality_sessions: Vec<QualitySession>,
        suppressions: Vec<SuppressionRecord>,
        baselines: Vec<BaselineRecord>,
        summary: impl Into<String>,
    ) -> Self {
        let effective_profile_refs = effective_profiles
            .iter()
            .map(|profile| profile.effective_profile_id.clone())
            .collect();
        let quality_session_refs = quality_sessions
            .iter()
            .map(|session| session.session_id.clone())
            .collect();
        let suppression_refs = suppressions
            .iter()
            .map(|suppression| suppression.suppression_id.clone())
            .collect();
        let baseline_refs = baselines
            .iter()
            .map(|baseline| baseline.baseline_id.clone())
            .collect();

        Self {
            record_kind: QUALITY_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: QUALITY_GOVERNANCE_SCHEMA_VERSION,
            support_export_id: support_export_id.into(),
            generated_at: generated_at.into(),
            effective_profile_refs,
            quality_session_refs,
            suppression_refs,
            baseline_refs,
            effective_profiles,
            quality_sessions,
            suppressions,
            baselines,
            redaction_safe: true,
            summary: summary.into(),
        }
    }
}

fn derive_disclosure_class(request: &QualityActionProposalRequest) -> QualityActionDisclosureClass {
    if !request.action_class.is_mutating()
        || request.mutation_scope_class == QualityMutationScopeClass::NoMutationReadOnly
    {
        return QualityActionDisclosureClass::ReadOnly;
    }
    if request.safety_class == QualitySafetyClass::UnknownOrUnstable
        || !request.semantic_current
        || request.blocked_path_count > 0
    {
        return QualityActionDisclosureClass::Blocked;
    }
    if !request.policy_lock_refs.is_empty()
        || request.profile_policy_locked
        || request.mutation_scope_class.is_policy_bearing()
    {
        return QualityActionDisclosureClass::PolicyEscalated;
    }
    if request.affected_file_count > 1
        || request.generated_path_count > 0
        || request.protected_path_count > 0
        || request.mutation_scope_class.is_broad()
        || request.safety_class == QualitySafetyClass::CrossFileSemantic
        || request.safety_class == QualitySafetyClass::GeneratedOrProtected
    {
        return QualityActionDisclosureClass::Broad;
    }
    if request.safety_class == QualitySafetyClass::SemanticLocal {
        return QualityActionDisclosureClass::Semantic;
    }
    QualityActionDisclosureClass::TriviaOnly
}

fn derive_preview_requirement(
    request: &QualityActionProposalRequest,
    disclosure_class: QualityActionDisclosureClass,
) -> QualityPreviewRequirementClass {
    match disclosure_class {
        QualityActionDisclosureClass::ReadOnly => QualityPreviewRequirementClass::NotRequired,
        QualityActionDisclosureClass::TriviaOnly => QualityPreviewRequirementClass::NotRequired,
        QualityActionDisclosureClass::Semantic => QualityPreviewRequirementClass::StructuredDiff,
        QualityActionDisclosureClass::Broad => QualityPreviewRequirementClass::BatchScopePreview,
        QualityActionDisclosureClass::PolicyEscalated => {
            QualityPreviewRequirementClass::PolicyOrRepoMutationPreviewRequired
        }
        QualityActionDisclosureClass::Blocked => {
            if !request.policy_lock_refs.is_empty() || request.profile_policy_locked {
                QualityPreviewRequirementClass::PolicyOrRepoMutationPreviewRequired
            } else {
                QualityPreviewRequirementClass::IssueLinkOrTypedReviewRequired
            }
        }
    }
}

fn derive_apply_posture(
    request: &QualityActionProposalRequest,
    disclosure_class: QualityActionDisclosureClass,
) -> QualityApplyPostureClass {
    match disclosure_class {
        QualityActionDisclosureClass::ReadOnly => QualityApplyPostureClass::ReadOnlyAction,
        QualityActionDisclosureClass::TriviaOnly => QualityApplyPostureClass::AutoApplyAllowed,
        QualityActionDisclosureClass::Semantic | QualityActionDisclosureClass::Broad => {
            QualityApplyPostureClass::PreviewBeforeApply
        }
        QualityActionDisclosureClass::PolicyEscalated => {
            QualityApplyPostureClass::BlockedPendingPolicyOrTrust
        }
        QualityActionDisclosureClass::Blocked => {
            if !request.policy_lock_refs.is_empty() || request.profile_policy_locked {
                QualityApplyPostureClass::BlockedPendingPolicyOrTrust
            } else {
                QualityApplyPostureClass::BlockedPendingUserReview
            }
        }
    }
}

fn derive_rollback_boundary(
    request: &QualityActionProposalRequest,
    disclosure_class: QualityActionDisclosureClass,
) -> QualityRollbackBoundaryClass {
    match disclosure_class {
        QualityActionDisclosureClass::ReadOnly => QualityRollbackBoundaryClass::NoMutation,
        QualityActionDisclosureClass::TriviaOnly => {
            if request.checkpoint_ref.is_some() {
                QualityRollbackBoundaryClass::SingleFileCheckpoint
            } else {
                QualityRollbackBoundaryClass::CurrentBufferUndo
            }
        }
        QualityActionDisclosureClass::Semantic => {
            QualityRollbackBoundaryClass::SingleFileCheckpoint
        }
        QualityActionDisclosureClass::Broad => {
            QualityRollbackBoundaryClass::GroupedWorkspaceCheckpoint
        }
        QualityActionDisclosureClass::PolicyEscalated => {
            QualityRollbackBoundaryClass::PolicyAuditOnly
        }
        QualityActionDisclosureClass::Blocked => {
            QualityRollbackBoundaryClass::ManualRecoveryRequired
        }
    }
}

fn validate_non_empty_ref(
    value: &str,
    error: QualityGovernanceError,
) -> Result<(), QualityGovernanceError> {
    if value.trim().is_empty() {
        Err(error)
    } else {
        Ok(())
    }
}
