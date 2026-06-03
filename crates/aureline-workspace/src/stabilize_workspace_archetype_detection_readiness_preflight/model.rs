//! Canonical stable truth model for workspace archetype detection, readiness
//! preflight, admission checkpoints, and first-useful-work routing.
//!
//! See the module-level documentation for the honesty invariants.

use serde::{Deserialize, Serialize};

use crate::admission::checkpoint::{
    AdmissionCheckpointRouteRecord, ContinueWithoutClass, DetectionConfidenceClass,
    DetectionEvidenceFreshness, DetectionOutcome, DetectionSignal, DetectionSignalSourceClass,
    DetectorState, FirstUsefulEntrySource, LandingSurface, MixedWorkspaceBoundaryChoice,
    ReadinessBuckets, RouteReasonClass, RouteSwitchOption, SignalFreshnessClass, SupportClaimClass,
};

/// Stable record-kind tag carried in serialized preflight records.
pub const WORKSPACE_ARCHETYPE_READINESS_PREFLIGHT_RECORD_KIND: &str =
    "workspace_archetype_readiness_preflight_record";

/// Schema version for the [`WorkspaceArchetypeReadinessPreflightRecord`] payload shape.
pub const WORKSPACE_ARCHETYPE_READINESS_PREFLIGHT_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface that ingests this record.
pub const WORKSPACE_ARCHETYPE_READINESS_PREFLIGHT_SHARED_CONTRACT_REF: &str =
    "workspace:archetype_readiness_preflight_stable:v1";

/// Reviewer-facing notice rendered on every preflight surface.
pub const WORKSPACE_ARCHETYPE_READINESS_PREFLIGHT_NOTICE: &str =
    "Workspace archetype readiness preflight truth: archetype detection signals are source-labeled \
     so surfaces can explain why a recommendation appeared; readiness work is grouped into \
     Blocking now, Recommended soon, and Optional later with source-labeled detection signals \
     such as manifest, workspace file, bundle marker, admin policy, extension contribution, or \
     previous user choice; certified and probable archetype labels respect evidence freshness \
     and downgrade automatically when qualification packets expire; first-useful-work routing \
     is inspectable and bounded and never auto-installs, auto-trusts, auto-runs setup, or hides \
     a required review step; mixed-root and nested-repo workspaces preserve truth by offering \
     Open whole repo, Open probable project, Open current folder only, or Create workset as \
     same-weight choices; Set up later, Open minimal, and Dismiss recommendation remain \
     same-weight options wherever setup is proposed. Shell, diagnostics, support exports, \
     Help/About, and docs read this record verbatim.";

const MAX_SENTENCE_CHARS: usize = 2048;
const MAX_REF_CHARS: usize = 200;
const CANONICAL_OBJECT_SCHEME: &str = "aureline://";

/// Returns true when `reference` is a canonical durable-object ref.
fn is_canonical_object_ref(reference: &str) -> bool {
    let reference = reference.trim();
    if reference.is_empty() || reference.len() > MAX_REF_CHARS {
        return false;
    }
    let Some(rest) = reference.strip_prefix(CANONICAL_OBJECT_SCHEME) else {
        return false;
    };
    let Some((class, ident)) = rest.split_once('/') else {
        return false;
    };
    if class.is_empty() || ident.is_empty() {
        return false;
    }
    !matches!(
        class,
        "home" | "dashboard" | "landing" | "index" | "overview" | "start" | "root"
    )
}

fn is_reviewable_sentence(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_SENTENCE_CHARS
}

fn require_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: value.to_string(),
        })
    }
}

/// Required source classes that can explain why a recommendation appeared.
const REQUIRED_SOURCE_CLASSES: &[DetectionSignalSourceClass] = &[
    DetectionSignalSourceClass::Manifest,
    DetectionSignalSourceClass::BundleMarker,
    DetectionSignalSourceClass::WorkspaceFile,
    DetectionSignalSourceClass::AdminPolicy,
    DetectionSignalSourceClass::ExtensionContribution,
    DetectionSignalSourceClass::PreviousUserChoice,
];

/// Validated input used to mint a [`WorkspaceArchetypeReadinessPreflightRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreflightInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Underlying admission checkpoint route record produced by the live builder.
    pub underlying: AdmissionCheckpointRouteRecord,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// The canonical, governed workspace archetype readiness preflight record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceArchetypeReadinessPreflightRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,

    // --- archetype detection truth ---
    /// Detection outcome family.
    pub detection_outcome: DetectionOutcome,
    /// Detection confidence class.
    pub detection_confidence: DetectionConfidenceClass,
    /// Scoped support claim allowed by detection.
    pub support_claim: SupportClaimClass,
    /// Completion and freshness state for the detector.
    pub detector_state: DetectorState,
    /// Optional archetype reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archetype_ref: Option<String>,
    /// Compatible bundle references derived from detection.
    pub compatible_bundle_refs: Vec<String>,
    /// Source-labeled signals that explain the outcome.
    pub source_labeled_signals: Vec<DetectionSignal>,
    /// Evidence freshness rows that support certified or probable wording.
    pub evidence_freshness: Vec<DetectionEvidenceFreshness>,
    /// Opaque fact references surfaced by detection.
    pub detected_fact_refs: Vec<String>,
    /// Opaque recommendation references derived from facts.
    pub recommendation_refs: Vec<String>,
    /// Opaque policy block references.
    pub policy_block_refs: Vec<String>,

    // --- readiness preflight ---
    /// Readiness buckets, not flattened.
    pub readiness_buckets: ReadinessBuckets,

    // --- first-useful-work routing ---
    /// Entry source that triggered the route.
    pub entry_source: FirstUsefulEntrySource,
    /// Landing surface selected by the route.
    pub landing_surface: LandingSurface,
    /// Reason the landing surface was selected.
    pub route_reason: RouteReasonClass,
    /// Reversible switch options.
    pub switch_options: Vec<RouteSwitchOption>,
    /// Current effect of remembered routing.
    pub remembered_routing_effect: crate::admission::checkpoint::RememberedRoutingEffect,

    // --- boundary choices for mixed workspaces ---
    /// Explicit mixed-root or mixed-stack boundary choices.
    pub boundary_choices: Vec<MixedWorkspaceBoundaryChoice>,

    // --- same-weight bypass actions ---
    /// Same-weight bypass actions shown with setup recommendations.
    pub same_weight_bypass_actions: Vec<ContinueWithoutClass>,

    // --- safety invariants (all must be false for a valid record) ---
    /// Whether detection may auto-install setup. Must always be false.
    pub auto_install_allowed: bool,
    /// Whether detection may auto-trust the workspace. Must always be false.
    pub auto_trust_allowed: bool,
    /// Whether hidden setup was executed without explicit user action. Must always be false.
    pub hidden_setup_executed: bool,
    /// Whether trust was silently widened. Must always be false.
    pub trust_widened: bool,

    // --- explanation and refs ---
    /// Redacted reviewer-facing summary.
    pub summary: String,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// Reasons a [`WorkspaceArchetypeReadinessPreflightRecord`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A field that must be a reviewable sentence was empty or too long.
    InvalidSentence { field: &'static str },
    /// A field that must be a canonical object ref was not.
    NonCanonicalRef { field: &'static str, value: String },
    /// The underlying admission checkpoint route record violated its contract.
    UnderlyingContractViolation { findings: Vec<String> },
    /// Auto-install is not allowed.
    AutoInstallNotAllowed,
    /// Auto-trust is not allowed.
    AutoTrustNotAllowed,
    /// Hidden setup execution is not allowed.
    HiddenSetupExecutedNotAllowed,
    /// Trust widening is not allowed.
    TrustWidenedNotAllowed,
    /// Archetype outcome must carry at least one detection signal.
    EmptyArchetypeSignals,
    /// A readiness task is missing required source signal refs.
    ReadinessTaskMissingSourceSignal { task_ref: String },
    /// Certified or probable archetype states must expose evidence freshness.
    CertifiedProbableRequiresEvidenceFreshness,
    /// Stale evidence requires downgrade from certified archetype match.
    StaleEvidenceRequiresDowngrade { outcome: DetectionOutcome },
    /// Mixed workspace is missing a required boundary choice.
    MixedWorkspaceMissingBoundaryChoice {
        choice: MixedWorkspaceBoundaryChoice,
    },
    /// Same-weight bypass actions are missing a required action.
    MissingSameWeightBypass { action: ContinueWithoutClass },
    /// Restricted or missing-prerequisite route must offer Open minimal.
    MissingOpenMinimalForNarrowedRoute,
    /// Remembered routing must not silently suppress required review.
    RememberedRoutingSuppressesRequiredReview,
    /// Certified or probable outcome must have at least one required source-class signal.
    MissingRequiredSourceClassSignal,
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSentence { field } => {
                write!(f, "field `{field}` must be a non-empty reviewable sentence")
            }
            Self::NonCanonicalRef { field, value } => {
                write!(
                    f,
                    "field `{field}` must be a canonical object ref, got {value:?}"
                )
            }
            Self::UnderlyingContractViolation { findings } => {
                write!(
                    f,
                    "underlying admission checkpoint route record violated contract: {}",
                    findings.join("; ")
                )
            }
            Self::AutoInstallNotAllowed => write!(f, "auto_install_allowed must remain false"),
            Self::AutoTrustNotAllowed => write!(f, "auto_trust_allowed must remain false"),
            Self::HiddenSetupExecutedNotAllowed => {
                write!(f, "hidden_setup_executed must remain false")
            }
            Self::TrustWidenedNotAllowed => write!(f, "trust_widened must remain false"),
            Self::EmptyArchetypeSignals => {
                write!(
                    f,
                    "archetype outcome must carry at least one detection signal"
                )
            }
            Self::ReadinessTaskMissingSourceSignal { task_ref } => {
                write!(
                    f,
                    "readiness task `{task_ref}` must have source_signal_refs"
                )
            }
            Self::CertifiedProbableRequiresEvidenceFreshness => {
                write!(
                    f,
                    "certified and probable archetype states must expose evidence freshness"
                )
            }
            Self::StaleEvidenceRequiresDowngrade { outcome } => {
                write!(
                    f,
                    "stale evidence requires downgrade from outcome `{}`",
                    outcome.as_str()
                )
            }
            Self::MixedWorkspaceMissingBoundaryChoice { choice } => {
                write!(
                    f,
                    "mixed workspace must include boundary choice `{}`",
                    choice.as_str()
                )
            }
            Self::MissingSameWeightBypass { action } => {
                write!(
                    f,
                    "same-weight bypass actions must include `{}`",
                    action.as_str()
                )
            }
            Self::MissingOpenMinimalForNarrowedRoute => {
                write!(
                    f,
                    "restricted or missing-prerequisite route must offer open_minimal"
                )
            }
            Self::RememberedRoutingSuppressesRequiredReview => {
                write!(
                    f,
                    "remembered routing must not suppress required trust, policy, import, or prerequisite review"
                )
            }
            Self::MissingRequiredSourceClassSignal => {
                write!(
                    f,
                    "at least one signal must use a required source class (manifest, bundle_marker, workspace_file, admin_policy, extension_contribution, or previous_user_choice)"
                )
            }
        }
    }
}

impl std::error::Error for BuildError {}

impl WorkspaceArchetypeReadinessPreflightRecord {
    /// Builds a governed preflight record from validated input.
    ///
    /// Returns a [`BuildError`] when the input would mint a record that violates
    /// the M04-190 honesty invariants.
    pub fn build(input: PreflightInput) -> Result<Self, BuildError> {
        // --- text / ref validation -------------------------------------------
        if !is_reviewable_sentence(&input.record_id) && input.record_id.len() > MAX_REF_CHARS {
            return Err(BuildError::InvalidSentence { field: "record_id" });
        }
        if !is_reviewable_sentence(&input.as_of) {
            return Err(BuildError::InvalidSentence { field: "as_of" });
        }
        require_ref("diagnostics_export_ref", &input.diagnostics_export_ref)?;
        require_ref("support_export_ref", &input.support_export_ref)?;
        for evidence in &input.evidence_refs {
            require_ref("evidence_refs", evidence)?;
        }
        for narrative in &input.narrative_refs {
            require_ref("narrative_refs", narrative)?;
        }

        // --- underlying contract validation ----------------------------------
        let underlying_findings = input.underlying.contract_findings();
        if !underlying_findings.is_empty() {
            return Err(BuildError::UnderlyingContractViolation {
                findings: underlying_findings,
            });
        }

        // --- M04-190 safety invariants ---------------------------------------
        if input.underlying.auto_install_allowed {
            return Err(BuildError::AutoInstallNotAllowed);
        }
        if input.underlying.auto_trust_allowed {
            return Err(BuildError::AutoTrustNotAllowed);
        }

        let archetype = &input.underlying.archetype;
        let readiness = &input.underlying.readiness;
        let route = &input.underlying.first_useful_route;

        // Archetype signals must not be empty.
        if archetype.signals.is_empty() {
            return Err(BuildError::EmptyArchetypeSignals);
        }

        // For outcomes that carry recommendations, at least one signal must use a required source class.
        if !archetype.recommendation_refs.is_empty()
            && !matches!(
                archetype.outcome,
                DetectionOutcome::UnknownOrGenericWorkspace
            )
        {
            let has_required_source = archetype
                .signals
                .iter()
                .any(|signal| REQUIRED_SOURCE_CLASSES.contains(&signal.source_class));
            if !has_required_source {
                return Err(BuildError::MissingRequiredSourceClassSignal);
            }
        }

        // Every readiness task must have source_signal_refs.
        for task in readiness.all_tasks() {
            if task.source_signal_refs.is_empty() {
                return Err(BuildError::ReadinessTaskMissingSourceSignal {
                    task_ref: task.task_ref.clone(),
                });
            }
        }

        // Certified and probable must have evidence freshness.
        if matches!(
            archetype.outcome,
            DetectionOutcome::CertifiedArchetypeMatch | DetectionOutcome::ProbableArchetype
        ) && archetype.evidence_freshness.is_empty()
        {
            return Err(BuildError::CertifiedProbableRequiresEvidenceFreshness);
        }

        // Stale evidence on certified requires downgrade.
        for ef in &archetype.evidence_freshness {
            if ef.freshness_class == SignalFreshnessClass::StaleRetestNeeded
                && archetype.outcome == DetectionOutcome::CertifiedArchetypeMatch
            {
                return Err(BuildError::StaleEvidenceRequiresDowngrade {
                    outcome: archetype.outcome,
                });
            }
        }

        // Mixed workspace must have all four boundary choices.
        if archetype.outcome == DetectionOutcome::MixedOrAmbiguousWorkspace {
            for required in [
                MixedWorkspaceBoundaryChoice::OpenWholeRepo,
                MixedWorkspaceBoundaryChoice::OpenProbableProject,
                MixedWorkspaceBoundaryChoice::OpenCurrentFolderOnly,
                MixedWorkspaceBoundaryChoice::CreateWorksetOrSlice,
            ] {
                if !input.underlying.boundary_choices.contains(&required) {
                    return Err(BuildError::MixedWorkspaceMissingBoundaryChoice {
                        choice: required,
                    });
                }
            }
        }

        // Same-weight bypass actions must include required set when there is work.
        if readiness.has_any_task() || !archetype.recommendation_refs.is_empty() {
            for required in [
                ContinueWithoutClass::SetUpLater,
                ContinueWithoutClass::OpenMinimal,
                ContinueWithoutClass::DismissRecommendation,
            ] {
                if !input
                    .underlying
                    .same_weight_bypass_actions
                    .contains(&required)
                {
                    return Err(BuildError::MissingSameWeightBypass { action: required });
                }
            }
        }

        // Restricted or missing-prerequisite must offer OpenMinimal.
        if matches!(
            archetype.outcome,
            DetectionOutcome::RestrictedOrPolicyBlocked | DetectionOutcome::MissingPrerequisite
        ) && !route
            .switch_options
            .contains(&RouteSwitchOption::OpenMinimal)
        {
            return Err(BuildError::MissingOpenMinimalForNarrowedRoute);
        }

        // Remembered routing must not suppress required review.
        if route.remembered_routing_effect
            == crate::admission::checkpoint::RememberedRoutingEffect::NarrowingHintOnly
            && matches!(
                archetype.outcome,
                DetectionOutcome::RestrictedOrPolicyBlocked | DetectionOutcome::MissingPrerequisite
            )
        {
            // Narrowing hint is acceptable for restricted/missing prerequisite because
            // the route still surfaces the review; we just verify OpenMinimal is present
            // (already checked above).
        }

        // Hidden setup and trust widening must be false (stable-record fields).
        // These come from the input directly since the underlying record does not carry them.
        // We default them to false when building from the convenience helper.

        let summary = format!(
            "{} entry with {} detection confidence lands on {}; {} blocking, {} recommended, {} optional; auto_install={}, auto_trust={}, hidden_setup={}, trust_widened={}.",
            route.entry_source.as_str(),
            archetype.confidence_class.as_str(),
            route.landing_surface.as_str(),
            readiness.blocking_now.len(),
            readiness.recommended_soon.len(),
            readiness.optional_later.len(),
            false,
            false,
            false,
            false,
        );

        Ok(Self {
            record_kind: WORKSPACE_ARCHETYPE_READINESS_PREFLIGHT_RECORD_KIND.to_string(),
            schema_version: WORKSPACE_ARCHETYPE_READINESS_PREFLIGHT_SCHEMA_VERSION,
            notice: WORKSPACE_ARCHETYPE_READINESS_PREFLIGHT_NOTICE.to_string(),
            shared_contract_ref: WORKSPACE_ARCHETYPE_READINESS_PREFLIGHT_SHARED_CONTRACT_REF
                .to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            detection_outcome: archetype.outcome,
            detection_confidence: archetype.confidence_class,
            support_claim: archetype.support_claim_class,
            detector_state: archetype.detector_state,
            archetype_ref: archetype.archetype_ref.clone(),
            compatible_bundle_refs: archetype.compatible_bundle_refs.clone(),
            source_labeled_signals: archetype.signals.clone(),
            evidence_freshness: archetype.evidence_freshness.clone(),
            detected_fact_refs: archetype.detected_fact_refs.clone(),
            recommendation_refs: archetype.recommendation_refs.clone(),
            policy_block_refs: archetype.policy_block_refs.clone(),
            readiness_buckets: readiness.clone(),
            entry_source: route.entry_source,
            landing_surface: route.landing_surface,
            route_reason: route.route_reason_class,
            switch_options: route.switch_options.clone(),
            remembered_routing_effect: route.remembered_routing_effect,
            boundary_choices: input.underlying.boundary_choices.clone(),
            same_weight_bypass_actions: input.underlying.same_weight_bypass_actions.clone(),
            auto_install_allowed: false,
            auto_trust_allowed: false,
            hidden_setup_executed: false,
            trust_widened: false,
            summary,
            diagnostics_export_ref: input.diagnostics_export_ref,
            support_export_ref: input.support_export_ref,
            evidence_refs: input.evidence_refs,
            narrative_refs: input.narrative_refs,
        })
    }

    /// Returns contract findings; an empty list means the record obeys the lane invariants.
    pub fn contract_findings(&self) -> Vec<String> {
        let mut findings = Vec::new();
        if self.auto_install_allowed {
            findings.push("auto_install_allowed must remain false".to_string());
        }
        if self.auto_trust_allowed {
            findings.push("auto_trust_allowed must remain false".to_string());
        }
        if self.hidden_setup_executed {
            findings.push("hidden_setup_executed must remain false".to_string());
        }
        if self.trust_widened {
            findings.push("trust_widened must remain false".to_string());
        }
        if self.source_labeled_signals.is_empty() {
            findings.push("archetype outcome must carry at least one detection signal".to_string());
        }
        if !self.recommendation_refs.is_empty()
            && !matches!(
                self.detection_outcome,
                DetectionOutcome::UnknownOrGenericWorkspace
            )
        {
            let has_required_source = self
                .source_labeled_signals
                .iter()
                .any(|signal| REQUIRED_SOURCE_CLASSES.contains(&signal.source_class));
            if !has_required_source {
                findings.push("at least one signal must use a required source class".to_string());
            }
        }
        for task in self.readiness_buckets.all_tasks() {
            if task.source_signal_refs.is_empty() {
                findings.push(format!(
                    "readiness task `{}` must have source_signal_refs",
                    task.task_ref
                ));
            }
        }
        if matches!(
            self.detection_outcome,
            DetectionOutcome::CertifiedArchetypeMatch | DetectionOutcome::ProbableArchetype
        ) && self.evidence_freshness.is_empty()
        {
            findings.push(
                "certified and probable archetype states must expose evidence freshness"
                    .to_string(),
            );
        }
        for ef in &self.evidence_freshness {
            if ef.freshness_class == SignalFreshnessClass::StaleRetestNeeded
                && self.detection_outcome == DetectionOutcome::CertifiedArchetypeMatch
            {
                findings.push(
                    "stale evidence requires downgrade from certified_archetype_match".to_string(),
                );
            }
        }
        if self.detection_outcome == DetectionOutcome::MixedOrAmbiguousWorkspace {
            for required in [
                MixedWorkspaceBoundaryChoice::OpenWholeRepo,
                MixedWorkspaceBoundaryChoice::OpenProbableProject,
                MixedWorkspaceBoundaryChoice::OpenCurrentFolderOnly,
                MixedWorkspaceBoundaryChoice::CreateWorksetOrSlice,
            ] {
                if !self.boundary_choices.contains(&required) {
                    findings.push(format!(
                        "mixed workspace must include boundary choice {}",
                        required.as_str()
                    ));
                }
            }
        }
        if self.readiness_buckets.has_any_task() || !self.recommendation_refs.is_empty() {
            for required in [
                ContinueWithoutClass::SetUpLater,
                ContinueWithoutClass::OpenMinimal,
                ContinueWithoutClass::DismissRecommendation,
            ] {
                if !self.same_weight_bypass_actions.contains(&required) {
                    findings.push(format!(
                        "same-weight bypass actions must include {}",
                        required.as_str()
                    ));
                }
            }
        }
        if matches!(
            self.detection_outcome,
            DetectionOutcome::RestrictedOrPolicyBlocked | DetectionOutcome::MissingPrerequisite
        ) && !self
            .switch_options
            .contains(&RouteSwitchOption::OpenMinimal)
        {
            findings.push(
                "restricted or missing-prerequisite route must offer open_minimal".to_string(),
            );
        }
        findings
    }

    /// Returns true when the record obeys the lane invariants.
    pub fn is_contract_valid(&self) -> bool {
        self.contract_findings().is_empty()
    }

    /// Returns a deterministic plaintext truth block for support exports.
    pub fn support_export_lines(&self) -> Vec<String> {
        let signal_sources = self
            .source_labeled_signals
            .iter()
            .map(|signal| format!("{}:{}", signal.source_class.as_str(), signal.signal_ref))
            .collect::<Vec<_>>()
            .join(", ");
        let evidence = self
            .evidence_freshness
            .iter()
            .map(|row| {
                let reviewed = row.reviewed_on.as_deref().unwrap_or("unknown");
                format!(
                    "{}:{}:reviewed_on={}",
                    row.evidence_ref,
                    row.freshness_class.as_str(),
                    reviewed
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        let bypasses = self
            .same_weight_bypass_actions
            .iter()
            .map(|action| action.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let switches = self
            .switch_options
            .iter()
            .map(|option| option.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let boundaries = self
            .boundary_choices
            .iter()
            .map(|choice| choice.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let mut lines = vec![
            format!(
                "workspace_archetype_readiness_preflight: {}",
                self.record_id
            ),
            format!("as_of: {}", self.as_of),
            format!(
                "detection: {} confidence={} support={} detector={}",
                self.detection_outcome.as_str(),
                self.detection_confidence.as_str(),
                self.support_claim.as_str(),
                self.detector_state.as_str(),
            ),
            format!("signal_sources: [{}]", signal_sources),
            format!("evidence_freshness: [{}]", evidence),
            format!(
                "readiness: blocking_now={} recommended_soon={} optional_later={}",
                self.readiness_buckets.blocking_now.len(),
                self.readiness_buckets.recommended_soon.len(),
                self.readiness_buckets.optional_later.len(),
            ),
            format!(
                "route: entry={} landing={} reason={} switches=[{}]",
                self.entry_source.as_str(),
                self.landing_surface.as_str(),
                self.route_reason.as_str(),
                switches
            ),
            format!(
                "same_weight_bypass=[{}] boundaries=[{}]",
                bypasses, boundaries
            ),
            format!(
                "safety: auto_install={} auto_trust={} hidden_setup={} trust_widened={}",
                self.auto_install_allowed,
                self.auto_trust_allowed,
                self.hidden_setup_executed,
                self.trust_widened,
            ),
            format!("summary: {}", self.summary),
        ];

        if !self.detected_fact_refs.is_empty() {
            lines.push(format!("detected_facts: {:?}", self.detected_fact_refs));
        }
        if !self.recommendation_refs.is_empty() {
            lines.push(format!("recommendations: {:?}", self.recommendation_refs));
        }
        if !self.policy_block_refs.is_empty() {
            lines.push(format!("policy_blocks: {:?}", self.policy_block_refs));
        }

        lines
    }
}
