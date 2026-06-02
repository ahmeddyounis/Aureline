//! Qualification audit for optional AI-adjacent surface families.
//!
//! Optional AI-adjacent surfaces — notebook, voice, browser companion,
//! preview/designer, background branch automation, and other adjacent lanes —
//! may not inherit Stable simply because the core composer and patch lanes are
//! green. Each exposed lane must carry its own qualification proof, or be
//! visibly labeled below Stable in product, docs/help, and release packets.
//!
//! This module owns the qualification matrix record type that enforces that
//! rule. The packet records, for each optional surface family:
//!
//! - whether the lane has its own current qualification packet (or is
//!   explicitly labeled below Stable);
//! - the family-specific boundary requirements (notebook kernel/output trust,
//!   voice consent and transcript posture, browser-companion scope honesty,
//!   preview/designer mutation boundary);
//! - downgrade automation state — what triggers a downgrade, what it targets,
//!   and where the downgrade state propagates; and
//! - propagation state — whether Help/About, docs, marketplace, CLI/headless
//!   inspect, and support-export surfaces consume family-specific qualification
//!   state instead of collapsing unqualified optional surfaces into an
//!   optimistic "available in build" stable label.
//!
//! The packet is export-safe. It carries refs, stable class tokens, booleans,
//! counts, and review labels only. Raw prompts, provider payloads, endpoint
//! URLs, credentials, and signing-key material stay outside the boundary.
//!
//! The boundary schema is
//! [`schemas/ai/optional-ai-surface-qualification.schema.json`](../../../../schemas/ai/optional-ai-surface-qualification.schema.json).
//! The contract doc is
//! [`docs/ai/m4/audit-optional-ai-adjacent-surfaces.md`](../../../../docs/ai/m4/audit-optional-ai-adjacent-surfaces.md).
//! The protected fixture directory is
//! [`fixtures/ai/m4/audit-optional-ai-adjacent-surfaces/`](../../../../fixtures/ai/m4/audit-optional-ai-adjacent-surfaces/).

#[cfg(test)]
mod tests;

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`OptionalAiAdjacentSurfaceAuditPacket`].
pub const OPTIONAL_AI_ADJACENT_SURFACE_AUDIT_RECORD_KIND: &str =
    "optional_ai_adjacent_surface_qualification";

/// Schema version for optional AI-adjacent surface audit records.
pub const OPTIONAL_AI_ADJACENT_SURFACE_AUDIT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const OPTIONAL_AI_ADJACENT_SURFACE_AUDIT_SCHEMA_REF: &str =
    "schemas/ai/optional-ai-surface-qualification.schema.json";

/// Repo-relative path of the contract doc.
pub const OPTIONAL_AI_ADJACENT_SURFACE_AUDIT_DOC_REF: &str =
    "docs/ai/m4/audit-optional-ai-adjacent-surfaces.md";

/// Repo-relative path of the protected fixture directory.
pub const OPTIONAL_AI_ADJACENT_SURFACE_AUDIT_FIXTURE_DIR: &str =
    "fixtures/ai/m4/audit-optional-ai-adjacent-surfaces";

/// Repo-relative path of the checked audit support-export artifact.
pub const OPTIONAL_AI_ADJACENT_SURFACE_AUDIT_ARTIFACT_REF: &str =
    "artifacts/ai/m4/audit-optional-ai-adjacent-surfaces/support_export.json";

/// Repo-relative path of the checked audit Markdown summary.
pub const OPTIONAL_AI_ADJACENT_SURFACE_AUDIT_SUMMARY_REF: &str =
    "artifacts/ai/m4/audit-optional-ai-adjacent-surfaces/summary.md";

/// Surface families covered by the optional AI-adjacent surface audit.
///
/// Each variant corresponds to one lane that requires its own qualification
/// proof before claiming Stable status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OptionalAiSurfaceFamily {
    /// AI affordances embedded in a notebook document/kernel/output surface.
    Notebook,
    /// Voice or dictation input lane feeding an AI surface.
    Voice,
    /// Browser extension or companion panel AI actions.
    BrowserCompanion,
    /// Preview, designer, or publish lane AI affordances.
    PreviewDesigner,
    /// Background branch automation or side-branch AI jobs.
    BackgroundBranchAutomation,
    /// Any other AI-adjacent surface not covered by the named families.
    OtherAdjacent,
}

impl OptionalAiSurfaceFamily {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::Voice => "voice",
            Self::BrowserCompanion => "browser_companion",
            Self::PreviewDesigner => "preview_designer",
            Self::BackgroundBranchAutomation => "background_branch_automation",
            Self::OtherAdjacent => "other_adjacent",
        }
    }

    /// Surface families that must be covered before the umbrella audit can
    /// close as a stable row.
    pub const fn required_families() -> [Self; 5] {
        [
            Self::Notebook,
            Self::Voice,
            Self::BrowserCompanion,
            Self::PreviewDesigner,
            Self::BackgroundBranchAutomation,
        ]
    }
}

/// Qualification label for an optional AI-adjacent surface.
///
/// A lane with no current qualification packet MUST carry a label narrower than
/// [`SurfaceQualificationLabel::Stable`] and propagate that label into
/// product copy, docs/help, and release packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceQualificationLabel {
    /// Lane has passed all stable-gate requirements under its own packet.
    Stable,
    /// Lane is production-ready but with reduced feature scope.
    Limited,
    /// Lane is available to early adopters with explicit opt-in.
    Preview,
    /// Lane is exploratory; not production-recommended.
    Experimental,
    /// Lane is not supported in the shipped build.
    Unsupported,
}

impl SurfaceQualificationLabel {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Limited => "limited",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unsupported => "unsupported",
        }
    }

    /// Returns `true` when the label is narrower than Stable and therefore
    /// requires a visible below-stable label in product and docs.
    pub const fn requires_visible_below_stable_label(self) -> bool {
        !matches!(self, Self::Stable)
    }
}

/// Notebook-family specific boundary requirements.
///
/// A notebook AI-adjacent surface must carry explicit trust labeling for the
/// document, kernel, and outputs, surface busy/queued/idle/interrupted kernel
/// state consistently, label debugger-bridge support, and provide output
/// sandbox/trust cues rather than relying on generic AI affordances.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookRequirements {
    /// Document-level trust is explicitly labeled.
    pub document_trust_labeled: bool,
    /// Kernel-level trust is explicitly labeled.
    pub kernel_trust_labeled: bool,
    /// Output-level trust is explicitly labeled.
    pub output_trust_labeled: bool,
    /// Kernel busy/queued/idle/interrupted state is consistently surfaced.
    pub kernel_state_consistent: bool,
    /// Debugger-bridge support level is explicitly labeled.
    pub debugger_support_labeled: bool,
    /// Output sandbox and trust cues are present.
    pub output_sandbox_cues_present: bool,
    /// No generic AI affordances that imply JupyterLab-class maturity through
    /// silence.
    pub no_generic_ai_affordances: bool,
}

impl NotebookRequirements {
    /// Returns `true` when all notebook requirements are satisfied.
    pub fn all_satisfied(&self) -> bool {
        self.document_trust_labeled
            && self.kernel_trust_labeled
            && self.output_trust_labeled
            && self.kernel_state_consistent
            && self.debugger_support_labeled
            && self.output_sandbox_cues_present
            && self.no_generic_ai_affordances
    }
}

/// Voice/dictation-family specific boundary requirements.
///
/// A voice or dictation AI surface must require explicit consent before
/// capture, declare its capture boundary, be explicit about local-versus-
/// retained transcript posture, expose a disable path, and provide an
/// accessibility-safe fallback rather than treating voice as just another
/// text input channel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceRequirements {
    /// Explicit consent is required before capture begins.
    pub explicit_consent_required: bool,
    /// The capture boundary is declared to the user.
    pub capture_boundary_declared: bool,
    /// Local-versus-retained transcript posture is explicit.
    pub local_vs_retained_transcript_explicit: bool,
    /// A disable path is present and reachable.
    pub disable_path_present: bool,
    /// An accessibility-safe fallback is present.
    pub accessibility_safe_fallback_present: bool,
    /// Voice is not silently treated as just another text input channel.
    pub not_treated_as_plain_text_input: bool,
}

impl VoiceRequirements {
    /// Returns `true` when all voice requirements are satisfied.
    pub fn all_satisfied(&self) -> bool {
        self.explicit_consent_required
            && self.capture_boundary_declared
            && self.local_vs_retained_transcript_explicit
            && self.disable_path_present
            && self.accessibility_safe_fallback_present
            && self.not_treated_as_plain_text_input
    }
}

/// Browser-companion-family specific boundary requirements.
///
/// A browser companion or deep-link AI surface must limit its scope to
/// review/docs/light-handoff or source-first inspect modes. Native-depth
/// authority, silent write-back, and hidden runtime mutation must all be
/// absent unless separately proven.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserCompanionRequirements {
    /// Scope is limited to review/docs/light-handoff or source-first inspect
    /// modes.
    pub scope_limited_to_review_docs_or_inspect: bool,
    /// No native-depth authority is claimed.
    pub no_native_depth_authority: bool,
    /// No silent write-back is possible.
    pub no_silent_write_back: bool,
    /// No hidden runtime mutation is possible.
    pub no_hidden_runtime_mutation: bool,
    /// A scope label is present on the surface.
    pub scope_label_present: bool,
}

impl BrowserCompanionRequirements {
    /// Returns `true` when all browser-companion requirements are satisfied.
    pub fn all_satisfied(&self) -> bool {
        self.scope_limited_to_review_docs_or_inspect
            && self.no_native_depth_authority
            && self.no_silent_write_back
            && self.no_hidden_runtime_mutation
            && self.scope_label_present
    }
}

/// Preview/designer-family specific boundary requirements.
///
/// A preview, designer, or publish AI surface must keep its scope honest,
/// avoid claiming native-depth authority, avoid silent write-back, and avoid
/// hidden runtime mutation. Any write-capable sub-surface must be separately
/// proven rather than inheriting the preview lane's proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewDesignerRequirements {
    /// Scope representation is honest.
    pub scope_honest: bool,
    /// No native-depth authority is claimed.
    pub no_native_depth_authority: bool,
    /// No silent write-back is possible.
    pub no_silent_write_back: bool,
    /// No hidden runtime mutation is possible.
    pub no_hidden_runtime_mutation: bool,
    /// Any write-capable sub-surface carries its own separate proof.
    pub separately_proven_if_write_capable: bool,
}

impl PreviewDesignerRequirements {
    /// Returns `true` when all preview/designer requirements are satisfied.
    pub fn all_satisfied(&self) -> bool {
        self.scope_honest
            && self.no_native_depth_authority
            && self.no_silent_write_back
            && self.no_hidden_runtime_mutation
            && self.separately_proven_if_write_capable
    }
}

/// One row in the qualification matrix, covering a single surface family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceQualificationRow {
    /// The surface family this row covers.
    pub surface_family: OptionalAiSurfaceFamily,
    /// Human-readable label for the surface (e.g., `"Notebook AI affordances"`).
    pub surface_label: String,
    /// The current qualification label for this surface.
    pub qualification: SurfaceQualificationLabel,
    /// Whether this surface has its own current qualification packet.
    pub has_own_qualification_packet: bool,
    /// Ref to the qualification packet, if one exists.
    pub qualification_packet_ref: String,
    /// Trust boundary is explicit (not inherited from core AI graduation).
    pub trust_boundary_explicit: bool,
    /// Route and evidence posture is explicit.
    pub route_evidence_explicit: bool,
    /// Export/support parity is explicit.
    pub export_support_parity_explicit: bool,
    /// Downgrade rule is explicit.
    pub downgrade_rule_explicit: bool,
    /// This surface does not freeload on core AI graduation proof.
    pub no_stable_inheritance_from_core: bool,
    /// A visible below-stable label is shown in product, docs/help, and
    /// release packets. Required when `qualification != stable`.
    pub visible_below_stable_label: bool,
    /// Qualification state is propagated into Help/About surfaces.
    pub propagated_to_help_about: bool,
    /// Qualification state is propagated into docs.
    pub propagated_to_docs: bool,
    /// Qualification state is propagated into release packets.
    pub propagated_to_release_packets: bool,
    /// Qualification state is propagated into compatibility reports.
    pub propagated_to_compat_reports: bool,
    /// Notebook-specific requirements, when `surface_family == Notebook`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_requirements: Option<NotebookRequirements>,
    /// Voice-specific requirements, when `surface_family == Voice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_requirements: Option<VoiceRequirements>,
    /// Browser-companion-specific requirements, when `surface_family ==
    /// BrowserCompanion`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_companion_requirements: Option<BrowserCompanionRequirements>,
    /// Preview/designer-specific requirements, when `surface_family ==
    /// PreviewDesigner`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_designer_requirements: Option<PreviewDesignerRequirements>,
    /// Short label describing known gaps or open work items for this lane.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub known_gaps_label: Option<String>,
}

/// What can trigger an automatic downgrade of an optional AI-adjacent surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeTriggerClass {
    /// The qualification packet has passed its freshness deadline.
    PacketFreshnessExpired,
    /// Route truth has regressed (route is no longer verifiable).
    RouteTruthRegressed,
    /// Support/export parity is missing for this lane.
    SupportExportParityMissing,
    /// A trust boundary violation was detected.
    TrustBoundaryViolated,
    /// A required eval threshold was missed.
    EvalThresholdMissed,
}

impl DowngradeTriggerClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PacketFreshnessExpired => "packet_freshness_expired",
            Self::RouteTruthRegressed => "route_truth_regressed",
            Self::SupportExportParityMissing => "support_export_parity_missing",
            Self::TrustBoundaryViolated => "trust_boundary_violated",
            Self::EvalThresholdMissed => "eval_threshold_missed",
        }
    }
}

/// Downgrade automation configuration for optional AI-adjacent surfaces.
///
/// When any of the listed triggers fires, the surface is automatically
/// downgraded to the target label and the downgrade state is propagated into
/// the listed consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradeAutomationBlock {
    /// Trigger classes that activate the downgrade automation.
    pub triggers: Vec<DowngradeTriggerClass>,
    /// Downgrade fires when the qualification packet passes its freshness
    /// deadline.
    pub fires_on_packet_freshness_expiry: bool,
    /// Downgrade fires when route truth regresses.
    pub fires_on_route_truth_regression: bool,
    /// Downgrade fires when support/export parity is missing.
    pub fires_on_support_export_parity_missing: bool,
    /// Human-readable label for the downgrade target state.
    pub downgrade_target_label: String,
    /// Downgrade state is propagated into product copy.
    pub downgrade_propagates_to_product_copy: bool,
    /// Downgrade state is propagated into docs/help.
    pub downgrade_propagates_to_docs_help: bool,
    /// Downgrade state is propagated into release packets.
    pub downgrade_propagates_to_release_packets: bool,
    /// Downgrade state is propagated into compatibility reports.
    pub downgrade_propagates_to_compat_reports: bool,
}

/// Propagation state across all consumer surfaces.
///
/// Help/About, docs, marketplace, CLI/headless inspect, and support-export
/// must all consume family-specific qualification state and must not collapse
/// unqualified optional surfaces into an optimistic "available in build"
/// stable label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PropagationStateBlock {
    /// Help/About surfaces consume family-specific qualification state.
    pub help_about_surfaces_consume_family_specific_state: bool,
    /// Docs consume family-specific qualification state.
    pub docs_consume_family_specific_state: bool,
    /// Marketplace consumes family-specific qualification state.
    pub marketplace_consume_family_specific_state: bool,
    /// CLI/headless inspect consumes family-specific qualification state.
    pub cli_headless_inspect_consume_family_specific_state: bool,
    /// Support export consumes family-specific qualification state.
    pub support_export_consume_family_specific_state: bool,
    /// No consumer collapses unqualified optional surfaces back into one
    /// optimistic "available in build" stable label.
    pub no_optimistic_available_in_build_collapse: bool,
}

/// Root record for the optional AI-adjacent surface qualification audit.
///
/// Carries one [`SurfaceQualificationRow`] per exposed optional AI surface
/// family, the [`DowngradeAutomationBlock`], and the
/// [`PropagationStateBlock`]. Consumers validate the packet with
/// [`OptionalAiAdjacentSurfaceAuditPacket::validate`] before treating any
/// lane as Stable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionalAiAdjacentSurfaceAuditPacket {
    /// Record kind; must equal [`OPTIONAL_AI_ADJACENT_SURFACE_AUDIT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`OPTIONAL_AI_ADJACENT_SURFACE_AUDIT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Unique packet identifier.
    pub packet_id: String,
    /// Human-readable display label.
    pub display_label: String,
    /// Policy epoch this packet was evaluated against.
    pub policy_epoch_ref: String,
    /// Ref to the core AI graduation packet that this audit does not inherit
    /// from.
    pub core_ai_graduation_packet_ref: String,
    /// One row per exposed optional AI surface family.
    pub surface_rows: Vec<SurfaceQualificationRow>,
    /// Downgrade automation configuration.
    pub downgrade_automation: DowngradeAutomationBlock,
    /// Propagation state across all consumer surfaces.
    pub propagation_state: PropagationStateBlock,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Redaction class token (metadata-safe default for this export).
    pub redaction_class_token: String,
    /// RFC 3339 timestamp when this packet was minted.
    pub minted_at: String,
}

/// An invariant violation found by
/// [`OptionalAiAdjacentSurfaceAuditPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditViolation {
    /// The record_kind field does not match the expected constant.
    WrongRecordKind { found: String },
    /// The schema_version field does not match the expected constant.
    WrongSchemaVersion { found: u32 },
    /// The packet carries no surface rows.
    NoSurfaceRows,
    /// A required surface family is absent from the surface rows.
    MissingRequiredFamily { family: OptionalAiSurfaceFamily },
    /// A surface with qualification != Stable is missing a visible
    /// below-stable label.
    MissingBelowStableLabel { surface_label: String },
    /// A surface claims Stable but lacks its own qualification packet.
    StableWithoutOwnPacket { surface_label: String },
    /// A surface claims Stable inheritance from core AI graduation.
    StableInheritedFromCore { surface_label: String },
    /// A Notebook surface row is missing its notebook-specific requirements.
    NotebookRequirementsMissing { surface_label: String },
    /// A Notebook surface row has unsatisfied notebook requirements.
    NotebookRequirementsUnsatisfied { surface_label: String },
    /// A Voice surface row is missing its voice-specific requirements.
    VoiceRequirementsMissing { surface_label: String },
    /// A Voice surface row has unsatisfied voice requirements.
    VoiceRequirementsUnsatisfied { surface_label: String },
    /// A BrowserCompanion surface row is missing its requirements.
    BrowserCompanionRequirementsMissing { surface_label: String },
    /// A BrowserCompanion surface row has unsatisfied requirements.
    BrowserCompanionRequirementsUnsatisfied { surface_label: String },
    /// A PreviewDesigner surface row is missing its requirements.
    PreviewDesignerRequirementsMissing { surface_label: String },
    /// A PreviewDesigner surface row has unsatisfied requirements.
    PreviewDesignerRequirementsUnsatisfied { surface_label: String },
    /// Downgrade automation does not fire on packet freshness expiry.
    DowngradeNotFiringOnFreshnessExpiry,
    /// Downgrade automation does not fire on route truth regression.
    DowngradeNotFiringOnRouteTruthRegression,
    /// Downgrade automation does not fire on missing support/export parity.
    DowngradeNotFiringOnSupportExportParityMissing,
    /// Downgrade state does not propagate to at least one required consumer.
    DowngradeNotPropagating,
    /// Propagation state allows an optimistic "available in build" collapse.
    OptimisticAvailableInBuildCollapse,
    /// A consumer does not consume family-specific qualification state.
    ConsumerNotConsumingFamilySpecificState { consumer_label: String },
}

impl fmt::Display for AuditViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongRecordKind { found } => write!(
                f,
                "record_kind is {found:?}, expected {:?}",
                OPTIONAL_AI_ADJACENT_SURFACE_AUDIT_RECORD_KIND
            ),
            Self::WrongSchemaVersion { found } => write!(
                f,
                "schema_version is {found}, expected {OPTIONAL_AI_ADJACENT_SURFACE_AUDIT_SCHEMA_VERSION}"
            ),
            Self::NoSurfaceRows => write!(f, "surface_rows is empty"),
            Self::MissingRequiredFamily { family } => {
                write!(f, "required surface family {:?} is absent", family.as_str())
            }
            Self::MissingBelowStableLabel { surface_label } => write!(
                f,
                "surface {surface_label:?} is below Stable but has no visible below-stable label"
            ),
            Self::StableWithoutOwnPacket { surface_label } => write!(
                f,
                "surface {surface_label:?} claims Stable but has no own qualification packet"
            ),
            Self::StableInheritedFromCore { surface_label } => write!(
                f,
                "surface {surface_label:?} inherits Stable from core AI graduation (freeloading)"
            ),
            Self::NotebookRequirementsMissing { surface_label } => write!(
                f,
                "notebook surface {surface_label:?} is missing notebook_requirements"
            ),
            Self::NotebookRequirementsUnsatisfied { surface_label } => write!(
                f,
                "notebook surface {surface_label:?} has unsatisfied notebook requirements"
            ),
            Self::VoiceRequirementsMissing { surface_label } => write!(
                f,
                "voice surface {surface_label:?} is missing voice_requirements"
            ),
            Self::VoiceRequirementsUnsatisfied { surface_label } => write!(
                f,
                "voice surface {surface_label:?} has unsatisfied voice requirements"
            ),
            Self::BrowserCompanionRequirementsMissing { surface_label } => write!(
                f,
                "browser companion surface {surface_label:?} is missing browser_companion_requirements"
            ),
            Self::BrowserCompanionRequirementsUnsatisfied { surface_label } => write!(
                f,
                "browser companion surface {surface_label:?} has unsatisfied browser companion requirements"
            ),
            Self::PreviewDesignerRequirementsMissing { surface_label } => write!(
                f,
                "preview/designer surface {surface_label:?} is missing preview_designer_requirements"
            ),
            Self::PreviewDesignerRequirementsUnsatisfied { surface_label } => write!(
                f,
                "preview/designer surface {surface_label:?} has unsatisfied preview/designer requirements"
            ),
            Self::DowngradeNotFiringOnFreshnessExpiry => write!(
                f,
                "downgrade automation does not fire on packet freshness expiry"
            ),
            Self::DowngradeNotFiringOnRouteTruthRegression => write!(
                f,
                "downgrade automation does not fire on route truth regression"
            ),
            Self::DowngradeNotFiringOnSupportExportParityMissing => write!(
                f,
                "downgrade automation does not fire when support/export parity is missing"
            ),
            Self::DowngradeNotPropagating => write!(
                f,
                "downgrade state does not propagate to any required consumer"
            ),
            Self::OptimisticAvailableInBuildCollapse => write!(
                f,
                "propagation state allows an optimistic 'available in build' stable collapse"
            ),
            Self::ConsumerNotConsumingFamilySpecificState { consumer_label } => write!(
                f,
                "consumer {consumer_label:?} does not consume family-specific qualification state"
            ),
        }
    }
}

impl Error for AuditViolation {}

impl OptionalAiAdjacentSurfaceAuditPacket {
    /// Validates the packet against the qualification invariants.
    ///
    /// Returns all violations found. An empty `Vec` means the packet is valid.
    /// Consumers must call this before treating any optional lane as Stable.
    ///
    /// # Errors
    ///
    /// Returns one [`AuditViolation`] per rule that is broken; callers should
    /// treat a non-empty return as a blocking failure.
    pub fn validate(&self) -> Vec<AuditViolation> {
        let mut violations = Vec::new();

        if self.record_kind != OPTIONAL_AI_ADJACENT_SURFACE_AUDIT_RECORD_KIND {
            violations.push(AuditViolation::WrongRecordKind {
                found: self.record_kind.clone(),
            });
        }

        if self.schema_version != OPTIONAL_AI_ADJACENT_SURFACE_AUDIT_SCHEMA_VERSION {
            violations.push(AuditViolation::WrongSchemaVersion {
                found: self.schema_version,
            });
        }

        if self.surface_rows.is_empty() {
            violations.push(AuditViolation::NoSurfaceRows);
            return violations;
        }

        // Every required family must be present.
        for required in OptionalAiSurfaceFamily::required_families() {
            if !self
                .surface_rows
                .iter()
                .any(|r| r.surface_family == required)
            {
                violations.push(AuditViolation::MissingRequiredFamily { family: required });
            }
        }

        for row in &self.surface_rows {
            // Below-Stable surfaces must carry a visible label.
            if row.qualification.requires_visible_below_stable_label()
                && !row.visible_below_stable_label
            {
                violations.push(AuditViolation::MissingBelowStableLabel {
                    surface_label: row.surface_label.clone(),
                });
            }

            // Stable surfaces must have their own qualification packet.
            if row.qualification == SurfaceQualificationLabel::Stable
                && !row.has_own_qualification_packet
            {
                violations.push(AuditViolation::StableWithoutOwnPacket {
                    surface_label: row.surface_label.clone(),
                });
            }

            // No surface may freeload on core AI graduation.
            if !row.no_stable_inheritance_from_core {
                violations.push(AuditViolation::StableInheritedFromCore {
                    surface_label: row.surface_label.clone(),
                });
            }

            // Family-specific requirement checks.
            match row.surface_family {
                OptionalAiSurfaceFamily::Notebook => match &row.notebook_requirements {
                    None => violations.push(AuditViolation::NotebookRequirementsMissing {
                        surface_label: row.surface_label.clone(),
                    }),
                    Some(req) if !req.all_satisfied() => {
                        violations.push(AuditViolation::NotebookRequirementsUnsatisfied {
                            surface_label: row.surface_label.clone(),
                        });
                    }
                    _ => {}
                },
                OptionalAiSurfaceFamily::Voice => match &row.voice_requirements {
                    None => violations.push(AuditViolation::VoiceRequirementsMissing {
                        surface_label: row.surface_label.clone(),
                    }),
                    Some(req) if !req.all_satisfied() => {
                        violations.push(AuditViolation::VoiceRequirementsUnsatisfied {
                            surface_label: row.surface_label.clone(),
                        });
                    }
                    _ => {}
                },
                OptionalAiSurfaceFamily::BrowserCompanion => {
                    match &row.browser_companion_requirements {
                        None => {
                            violations.push(AuditViolation::BrowserCompanionRequirementsMissing {
                                surface_label: row.surface_label.clone(),
                            });
                        }
                        Some(req) if !req.all_satisfied() => {
                            violations.push(
                                AuditViolation::BrowserCompanionRequirementsUnsatisfied {
                                    surface_label: row.surface_label.clone(),
                                },
                            );
                        }
                        _ => {}
                    }
                }
                OptionalAiSurfaceFamily::PreviewDesigner => {
                    match &row.preview_designer_requirements {
                        None => {
                            violations.push(AuditViolation::PreviewDesignerRequirementsMissing {
                                surface_label: row.surface_label.clone(),
                            });
                        }
                        Some(req) if !req.all_satisfied() => {
                            violations.push(
                                AuditViolation::PreviewDesignerRequirementsUnsatisfied {
                                    surface_label: row.surface_label.clone(),
                                },
                            );
                        }
                        _ => {}
                    }
                }
                OptionalAiSurfaceFamily::BackgroundBranchAutomation
                | OptionalAiSurfaceFamily::OtherAdjacent => {}
            }
        }

        // Downgrade automation must fire on all three required triggers.
        if !self.downgrade_automation.fires_on_packet_freshness_expiry {
            violations.push(AuditViolation::DowngradeNotFiringOnFreshnessExpiry);
        }
        if !self.downgrade_automation.fires_on_route_truth_regression {
            violations.push(AuditViolation::DowngradeNotFiringOnRouteTruthRegression);
        }
        if !self
            .downgrade_automation
            .fires_on_support_export_parity_missing
        {
            violations.push(AuditViolation::DowngradeNotFiringOnSupportExportParityMissing);
        }

        // Downgrade state must propagate to at least one consumer.
        let any_propagation = self
            .downgrade_automation
            .downgrade_propagates_to_product_copy
            || self.downgrade_automation.downgrade_propagates_to_docs_help
            || self
                .downgrade_automation
                .downgrade_propagates_to_release_packets
            || self
                .downgrade_automation
                .downgrade_propagates_to_compat_reports;
        if !any_propagation {
            violations.push(AuditViolation::DowngradeNotPropagating);
        }

        // No optimistic "available in build" collapse is allowed.
        if !self
            .propagation_state
            .no_optimistic_available_in_build_collapse
        {
            violations.push(AuditViolation::OptimisticAvailableInBuildCollapse);
        }

        // All consumers must consume family-specific state.
        if !self
            .propagation_state
            .help_about_surfaces_consume_family_specific_state
        {
            violations.push(AuditViolation::ConsumerNotConsumingFamilySpecificState {
                consumer_label: "help_about".to_owned(),
            });
        }
        if !self.propagation_state.docs_consume_family_specific_state {
            violations.push(AuditViolation::ConsumerNotConsumingFamilySpecificState {
                consumer_label: "docs".to_owned(),
            });
        }
        if !self
            .propagation_state
            .support_export_consume_family_specific_state
        {
            violations.push(AuditViolation::ConsumerNotConsumingFamilySpecificState {
                consumer_label: "support_export".to_owned(),
            });
        }

        violations
    }

    /// Loads and validates the checked-in support-export artifact.
    ///
    /// # Errors
    ///
    /// Returns a JSON parse error if the checked-in artifact is malformed.
    pub fn from_checked_artifact() -> Result<Self, serde_json::Error> {
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/ai/m4/audit-optional-ai-adjacent-surfaces/support_export.json"
        )))
    }
}
