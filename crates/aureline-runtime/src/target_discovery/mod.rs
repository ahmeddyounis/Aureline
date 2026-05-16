//! Beta finalize layer for target-discovery confidence and explainability.
//!
//! The alpha `crates/aureline-runtime/src/targets` module projects the
//! canonical [`ExecutionContext`] into one [`TargetConfidenceCard`] with a
//! single `TargetDiscoveryConfidenceClass` token mixing two orthogonal axes:
//! where the target came from (native protocol vs structured adapter vs
//! heuristic parser) and how confident the resolver is in the result. The
//! beta layer promotes those axes into separate closed vocabularies and ships
//! the projection that downstream run / test / debug / build / support
//! consumers read so a heuristic or imported row cannot masquerade as exact
//! runnable truth on protected actions.
//!
//! Every beta row carries:
//!
//! - a [`DiscoverySourceClass`] (where the target came from);
//! - a [`DiscoveryFreshnessClass`] (how current the discovery is);
//! - a closed list of [`SupportedCapabilityClass`] tokens the target supports;
//! - one decision per [`ProtectedActionClass`] (allowed, requires review, or a
//!   typed blocked-reason).
//!
//! The machine-readable boundary lives at
//! [`/schemas/runtime/target_discovery_beta.schema.json`](../../../../schemas/runtime/target_discovery_beta.schema.json)
//! and the reviewer-facing companion doc at
//! [`/docs/runtime/m3/target_discovery_beta.md`](../../../../docs/runtime/m3/target_discovery_beta.md).

use serde::{Deserialize, Serialize};

use crate::execution_context::{ExecutionContext, MixedVersionDriftState, TargetClass, TrustState};
use crate::provenance::{
    dedupe_context_provenance, ExecutionEventProvenance, ExecutionProvenanceEvent,
    ExecutionProvenanceEventClass,
};
use crate::targets::{
    HostBoundaryCueClass, TargetConfidenceCard, TargetConfidenceLaneClass,
    TargetConfidenceReviewRow, TargetConfidenceSupportExport, TargetDiscoveryConfidenceClass,
    TargetHostBoundaryRow,
};

/// Schema version for the target-discovery beta records.
pub const TARGET_DISCOVERY_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for one beta target-discovery row.
pub const TARGET_DISCOVERY_BETA_ROW_RECORD_KIND: &str = "target_discovery_beta_row_record";

/// Stable record-kind tag for the beta projection bundle.
pub const TARGET_DISCOVERY_BETA_PROJECTION_RECORD_KIND: &str =
    "target_discovery_beta_projection_record";

/// Stable record-kind tag for the beta support-export packet.
pub const TARGET_DISCOVERY_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "target_discovery_beta_support_export_record";

/// Stable record-kind tag for the beta coverage manifest.
pub const TARGET_DISCOVERY_BETA_COVERAGE_MANIFEST_RECORD_KIND: &str =
    "target_discovery_beta_coverage_manifest_record";

/// Closed vocabulary for where a discovered target came from.
///
/// Adding a class is additive-minor and MUST update the schema, the coverage
/// manifest, and the reviewer doc together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoverySourceClass {
    /// Target was minted by a native runtime / language host / debug-adapter
    /// protocol (DAP launch metadata, LSP runnable, language-host attach).
    NativeProtocol,
    /// Target came from a typed adapter parsing a structured manifest
    /// (package.json scripts, pyproject `[tool.pytest]`, Cargo metadata,
    /// devcontainer manifest, launch.json).
    StructuredAdapter,
    /// Target was inferred by a heuristic / regex / fallback parser when no
    /// structured source produced an exact runnable.
    HeuristicParser,
    /// Target was lifted from imported CI / external metadata that the local
    /// resolver did not probe in this session.
    ImportedMetadata,
    /// Target was declared by the user (explicit override, saved profile, or
    /// pinned launch profile).
    UserDeclared,
    /// The discovery layer was unavailable; no target may dispatch protected
    /// work from this row.
    ResolverUnavailable,
}

impl DiscoverySourceClass {
    /// All beta discovery-source classes.
    pub const ALL: [Self; 6] = [
        Self::NativeProtocol,
        Self::StructuredAdapter,
        Self::HeuristicParser,
        Self::ImportedMetadata,
        Self::UserDeclared,
        Self::ResolverUnavailable,
    ];

    /// Stable token recorded in records, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NativeProtocol => "native_protocol",
            Self::StructuredAdapter => "structured_adapter",
            Self::HeuristicParser => "heuristic_parser",
            Self::ImportedMetadata => "imported_metadata",
            Self::UserDeclared => "user_declared",
            Self::ResolverUnavailable => "resolver_unavailable",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NativeProtocol => "Native protocol",
            Self::StructuredAdapter => "Structured adapter",
            Self::HeuristicParser => "Heuristic parser",
            Self::ImportedMetadata => "Imported metadata",
            Self::UserDeclared => "User declared",
            Self::ResolverUnavailable => "Resolver unavailable",
        }
    }

    /// True when this source MUST NOT dispatch protected run / test / debug
    /// work without typed review. Heuristic and resolver-unavailable rows
    /// always block; imported rows block only on protected dispatch.
    pub const fn blocks_protected_dispatch(self) -> bool {
        matches!(self, Self::HeuristicParser | Self::ResolverUnavailable)
    }
}

/// Closed vocabulary for how current a target discovery is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryFreshnessClass {
    /// Target was probed in the current resolver session and the probe
    /// matched canonical expectations.
    FreshProbe,
    /// Target was probed earlier in this session and the resolver still trusts
    /// the binding (no drift signal observed).
    RecentWithinSession,
    /// Target was imported from an authoritative external source (CI metadata,
    /// signed manifest) that the resolver did not re-probe locally.
    ImportedAuthoritative,
    /// Target was imported but the resolver observed drift or staleness
    /// (mixed-version unchecked, capsule drift, conflicting sources).
    StaleImported,
    /// Freshness cannot be determined; treat as unsafe for protected dispatch.
    Unknown,
}

impl DiscoveryFreshnessClass {
    /// Stable token recorded in records, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshProbe => "fresh_probe",
            Self::RecentWithinSession => "recent_within_session",
            Self::ImportedAuthoritative => "imported_authoritative",
            Self::StaleImported => "stale_imported",
            Self::Unknown => "unknown",
        }
    }

    /// True when this freshness class is not strong enough to dispatch
    /// protected run / test / debug actions.
    pub const fn requires_refresh_before_dispatch(self) -> bool {
        matches!(self, Self::StaleImported | Self::Unknown)
    }
}

/// Closed vocabulary for action classes a target may advertise.
///
/// This is intentionally smaller than the full surface set: surfaces that do
/// not dispatch live target work (e.g. inspectors, support export) read these
/// rows without consuming the capability list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportedCapabilityClass {
    /// Target supports running a build / package / generic task.
    Run,
    /// Target supports running test attempts.
    Test,
    /// Target supports launching a debug session (start + spawn).
    DebugLaunch,
    /// Target supports attaching to an existing process for debugging.
    DebugAttach,
    /// Target supports producing a build artifact.
    Build,
    /// Target supports read-only inspection only (no dispatch).
    InspectOnly,
}

impl SupportedCapabilityClass {
    /// All beta capability classes.
    pub const ALL: [Self; 6] = [
        Self::Run,
        Self::Test,
        Self::DebugLaunch,
        Self::DebugAttach,
        Self::Build,
        Self::InspectOnly,
    ];

    /// Stable token recorded in records, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Run => "run",
            Self::Test => "test",
            Self::DebugLaunch => "debug_launch",
            Self::DebugAttach => "debug_attach",
            Self::Build => "build",
            Self::InspectOnly => "inspect_only",
        }
    }
}

/// Protected action class gated by the beta target-discovery layer.
///
/// Adding a class is additive-minor and MUST update the schema, the coverage
/// manifest, and the reviewer doc together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtectedActionClass {
    DispatchRun,
    DispatchTest,
    DispatchDebugLaunch,
    DispatchDebugAttach,
    DispatchBuild,
    ExportArtifact,
}

impl ProtectedActionClass {
    /// All beta protected-action classes.
    pub const ALL: [Self; 6] = [
        Self::DispatchRun,
        Self::DispatchTest,
        Self::DispatchDebugLaunch,
        Self::DispatchDebugAttach,
        Self::DispatchBuild,
        Self::ExportArtifact,
    ];

    /// Stable token recorded in records, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DispatchRun => "dispatch_run",
            Self::DispatchTest => "dispatch_test",
            Self::DispatchDebugLaunch => "dispatch_debug_launch",
            Self::DispatchDebugAttach => "dispatch_debug_attach",
            Self::DispatchBuild => "dispatch_build",
            Self::ExportArtifact => "export_artifact",
        }
    }

    /// Capability class this protected action requires the target to support.
    pub const fn required_capability(self) -> Option<SupportedCapabilityClass> {
        match self {
            Self::DispatchRun => Some(SupportedCapabilityClass::Run),
            Self::DispatchTest => Some(SupportedCapabilityClass::Test),
            Self::DispatchDebugLaunch => Some(SupportedCapabilityClass::DebugLaunch),
            Self::DispatchDebugAttach => Some(SupportedCapabilityClass::DebugAttach),
            Self::DispatchBuild => Some(SupportedCapabilityClass::Build),
            Self::ExportArtifact => None,
        }
    }
}

/// Closed decision vocabulary returned by the beta gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtectedActionDecisionClass {
    /// Action may proceed without further review.
    Allowed,
    /// Action may proceed only after reviewing the typed review packet
    /// (helper-backed or non-high-confidence row).
    RequiresReview,
    /// Heuristic-discovered target cannot dispatch this protected action.
    BlockedHeuristicTarget,
    /// Imported-metadata target cannot dispatch this live protected action.
    BlockedImportedTarget,
    /// Target advertises that it does not support this capability class.
    BlockedUnsupportedCapability,
    /// Resolver was unavailable for this row; no protected work permitted.
    BlockedResolverUnavailable,
    /// Discovery is stale; refresh required before dispatch.
    BlockedFreshnessStale,
}

impl ProtectedActionDecisionClass {
    /// Stable token recorded in records, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::RequiresReview => "requires_review",
            Self::BlockedHeuristicTarget => "blocked_heuristic_target",
            Self::BlockedImportedTarget => "blocked_imported_target",
            Self::BlockedUnsupportedCapability => "blocked_unsupported_capability",
            Self::BlockedResolverUnavailable => "blocked_resolver_unavailable",
            Self::BlockedFreshnessStale => "blocked_freshness_stale",
        }
    }

    /// True when this decision blocks the protected action entirely.
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::BlockedHeuristicTarget
                | Self::BlockedImportedTarget
                | Self::BlockedUnsupportedCapability
                | Self::BlockedResolverUnavailable
                | Self::BlockedFreshnessStale
        )
    }
}

/// One decision row inside a beta target-discovery row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtectedActionDecisionRow {
    /// Protected action class.
    pub action: ProtectedActionClass,
    /// Stable action token.
    pub action_token: String,
    /// Decision class.
    pub decision: ProtectedActionDecisionClass,
    /// Stable decision token.
    pub decision_token: String,
    /// Export-safe summary that names the action and reason.
    pub summary: String,
}

/// One beta row preserving target-discovery explainability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetDiscoveryBetaRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Alpha target-confidence card this row refines.
    pub card_ref: String,
    /// Canonical execution-context id.
    pub execution_context_ref: String,
    /// Canonical target id.
    pub target_id: String,
    /// Stable target-class token.
    pub target_class_token: String,
    /// Local or helper-backed lane token.
    pub lane_token: String,
    /// Beta discovery-source classification.
    pub discovery_source: DiscoverySourceClass,
    /// Stable discovery-source token.
    pub discovery_source_token: String,
    /// Short discovery-source label.
    pub discovery_source_label: String,
    /// Beta freshness classification.
    pub discovery_freshness: DiscoveryFreshnessClass,
    /// Stable freshness token.
    pub discovery_freshness_token: String,
    /// Alpha discovery-confidence class quoted verbatim for cross-reference.
    pub alpha_discovery_confidence_token: String,
    /// Coarse resolver confidence token.
    pub target_confidence_level_token: String,
    /// Stable host-boundary cue token.
    pub host_boundary_cue_token: String,
    /// Short host-boundary label.
    pub host_boundary_label: String,
    /// Capability classes this target supports.
    pub supported_capabilities: Vec<SupportedCapabilityClass>,
    /// Stable capability tokens.
    pub supported_capability_tokens: Vec<String>,
    /// Per protected-action decisions.
    pub protected_action_decisions: Vec<ProtectedActionDecisionRow>,
    /// Explanation summary describing why this source + freshness was chosen.
    pub explanation_summary: String,
    /// Action ref opening the shared execution-context inspector.
    pub inspect_action_ref: String,
    /// Action ref switching the target before dispatch.
    pub change_target_action_ref: String,
    /// True because raw paths, command lines, env bodies, and secrets are
    /// excluded.
    pub redaction_safe: bool,
}

impl TargetDiscoveryBetaRow {
    /// Builds a beta row from an alpha [`TargetConfidenceCard`] and the
    /// canonical [`ExecutionContext`] that produced it.
    pub fn from_card_and_context(card: &TargetConfidenceCard, context: &ExecutionContext) -> Self {
        let discovery_source = discovery_source_for(card, context);
        let discovery_freshness = discovery_freshness_for(card, context, discovery_source);
        let supported_capabilities =
            supported_capabilities_for(context.target_identity.target_class);
        let supported_capability_tokens = supported_capabilities
            .iter()
            .map(|cap| cap.as_str().to_owned())
            .collect::<Vec<_>>();
        let protected_action_decisions = ProtectedActionClass::ALL
            .into_iter()
            .map(|action| {
                let decision = evaluate_protected_action(
                    action,
                    card,
                    discovery_source,
                    discovery_freshness,
                    &supported_capabilities,
                );
                let summary = decision_summary(action, decision, card.target_id.as_str());
                ProtectedActionDecisionRow {
                    action,
                    action_token: action.as_str().to_owned(),
                    decision,
                    decision_token: decision.as_str().to_owned(),
                    summary,
                }
            })
            .collect();
        let explanation_summary = format!(
            "Target {} discovered via {} ({}) with {} freshness; alpha confidence {} ({}).",
            card.target_id,
            discovery_source.label(),
            discovery_source.as_str(),
            discovery_freshness.as_str(),
            card.target_confidence_level_token,
            card.discovery_confidence_token,
        );
        Self {
            record_kind: TARGET_DISCOVERY_BETA_ROW_RECORD_KIND.to_owned(),
            schema_version: TARGET_DISCOVERY_BETA_SCHEMA_VERSION,
            row_id: format!("target-discovery-beta-row:{}", card.card_id),
            card_ref: card.card_id.clone(),
            execution_context_ref: card.execution_context_ref.clone(),
            target_id: card.target_id.clone(),
            target_class_token: card.target_class_token.clone(),
            lane_token: card.lane_token.clone(),
            discovery_source,
            discovery_source_token: discovery_source.as_str().to_owned(),
            discovery_source_label: discovery_source.label().to_owned(),
            discovery_freshness,
            discovery_freshness_token: discovery_freshness.as_str().to_owned(),
            alpha_discovery_confidence_token: card.discovery_confidence_token.clone(),
            target_confidence_level_token: card.target_confidence_level_token.clone(),
            host_boundary_cue_token: card.host_boundary_cue_token.clone(),
            host_boundary_label: card.host_boundary_label.clone(),
            supported_capabilities,
            supported_capability_tokens,
            protected_action_decisions,
            explanation_summary,
            inspect_action_ref: card.inspect_action_ref.clone(),
            change_target_action_ref: card.change_target_action_ref.clone(),
            redaction_safe: true,
        }
    }

    /// Returns the decision row for a specific protected-action class.
    pub fn decision_for(
        &self,
        action: ProtectedActionClass,
    ) -> Option<&ProtectedActionDecisionRow> {
        self.protected_action_decisions
            .iter()
            .find(|row| row.action == action)
    }

    /// True when every protected dispatch action (run / test / debug launch /
    /// debug attach / build) is blocked for this row.
    pub fn blocks_all_protected_dispatch(&self) -> bool {
        ProtectedActionClass::ALL.iter().all(|action| {
            if matches!(action, ProtectedActionClass::ExportArtifact) {
                return true;
            }
            self.decision_for(*action)
                .map(|row| row.decision.is_blocked())
                .unwrap_or(false)
        })
    }

    /// Returns one deterministic plaintext line for this row.
    pub fn summary_line(&self) -> String {
        format!(
            "row={}; target={}({}); source={}; freshness={}; alpha_confidence={}; confidence={}; boundary={}",
            self.row_id,
            self.target_id,
            self.target_class_token,
            self.discovery_source_token,
            self.discovery_freshness_token,
            self.alpha_discovery_confidence_token,
            self.target_confidence_level_token,
            self.host_boundary_cue_token,
        )
    }
}

/// One coverage-manifest row pinning the closed beta vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetDiscoveryBetaCoverageRow {
    /// Discovery source.
    pub source: DiscoverySourceClass,
    /// Stable source token.
    pub source_token: String,
    /// Short reviewer-facing label.
    pub source_label: String,
    /// Whether this source class blocks protected dispatch by default.
    pub blocks_protected_dispatch_by_default: bool,
    /// Protected actions evaluated for this source.
    pub gated_action_tokens: Vec<String>,
}

impl TargetDiscoveryBetaCoverageRow {
    fn canonical(source: DiscoverySourceClass) -> Self {
        Self {
            source,
            source_token: source.as_str().to_owned(),
            source_label: source.label().to_owned(),
            blocks_protected_dispatch_by_default: source.blocks_protected_dispatch(),
            gated_action_tokens: ProtectedActionClass::ALL
                .iter()
                .map(|action| action.as_str().to_owned())
                .collect(),
        }
    }
}

/// Coverage manifest pinning the canonical beta vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetDiscoveryBetaCoverageManifest {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version shared with the beta record set.
    pub schema_version: u32,
    /// Manifest id.
    pub manifest_id: String,
    /// Manifest timestamp.
    pub generated_at: String,
    /// One row per discovery-source class.
    pub source_rows: Vec<TargetDiscoveryBetaCoverageRow>,
    /// Closed list of freshness tokens.
    pub freshness_tokens: Vec<String>,
    /// Closed list of supported-capability tokens.
    pub supported_capability_tokens: Vec<String>,
    /// Closed list of protected-action tokens.
    pub protected_action_tokens: Vec<String>,
    /// Closed list of protected-action decision tokens.
    pub protected_action_decision_tokens: Vec<String>,
}

impl TargetDiscoveryBetaCoverageManifest {
    /// Builds the canonical coverage manifest.
    pub fn canonical(manifest_id: impl Into<String>, generated_at: impl Into<String>) -> Self {
        Self {
            record_kind: TARGET_DISCOVERY_BETA_COVERAGE_MANIFEST_RECORD_KIND.to_owned(),
            schema_version: TARGET_DISCOVERY_BETA_SCHEMA_VERSION,
            manifest_id: manifest_id.into(),
            generated_at: generated_at.into(),
            source_rows: DiscoverySourceClass::ALL
                .into_iter()
                .map(TargetDiscoveryBetaCoverageRow::canonical)
                .collect(),
            freshness_tokens: vec![
                DiscoveryFreshnessClass::FreshProbe.as_str().to_owned(),
                DiscoveryFreshnessClass::RecentWithinSession
                    .as_str()
                    .to_owned(),
                DiscoveryFreshnessClass::ImportedAuthoritative
                    .as_str()
                    .to_owned(),
                DiscoveryFreshnessClass::StaleImported.as_str().to_owned(),
                DiscoveryFreshnessClass::Unknown.as_str().to_owned(),
            ],
            supported_capability_tokens: SupportedCapabilityClass::ALL
                .iter()
                .map(|cap| cap.as_str().to_owned())
                .collect(),
            protected_action_tokens: ProtectedActionClass::ALL
                .iter()
                .map(|action| action.as_str().to_owned())
                .collect(),
            protected_action_decision_tokens: vec![
                ProtectedActionDecisionClass::Allowed.as_str().to_owned(),
                ProtectedActionDecisionClass::RequiresReview
                    .as_str()
                    .to_owned(),
                ProtectedActionDecisionClass::BlockedHeuristicTarget
                    .as_str()
                    .to_owned(),
                ProtectedActionDecisionClass::BlockedImportedTarget
                    .as_str()
                    .to_owned(),
                ProtectedActionDecisionClass::BlockedUnsupportedCapability
                    .as_str()
                    .to_owned(),
                ProtectedActionDecisionClass::BlockedResolverUnavailable
                    .as_str()
                    .to_owned(),
                ProtectedActionDecisionClass::BlockedFreshnessStale
                    .as_str()
                    .to_owned(),
            ],
        }
    }
}

/// Bundle of beta rows projected from one or more execution contexts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetDiscoveryBetaProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable projection id.
    pub projection_id: String,
    /// Workspace id shared by the projected contexts.
    pub workspace_id: String,
    /// Projection timestamp.
    pub generated_at: String,
    /// Beta rows.
    pub rows: Vec<TargetDiscoveryBetaRow>,
    /// True when at least one row blocks all protected dispatch.
    pub any_row_blocks_protected_dispatch: bool,
    /// True because raw paths, command lines, env bodies, and secrets are
    /// excluded.
    pub redaction_safe: bool,
}

impl TargetDiscoveryBetaProjection {
    /// Builds a projection from contexts + the cards already minted for them.
    pub fn from_cards_and_contexts<'a, I, J>(
        projection_id: impl Into<String>,
        generated_at: impl Into<String>,
        cards: I,
        contexts: J,
    ) -> Self
    where
        I: IntoIterator<Item = &'a TargetConfidenceCard>,
        J: IntoIterator<Item = &'a ExecutionContext>,
    {
        let cards: Vec<&TargetConfidenceCard> = cards.into_iter().collect();
        let contexts: Vec<&ExecutionContext> = contexts.into_iter().collect();
        assert_eq!(
            cards.len(),
            contexts.len(),
            "card and context counts must match"
        );
        let rows = cards
            .iter()
            .zip(contexts.iter())
            .map(|(card, context)| TargetDiscoveryBetaRow::from_card_and_context(card, context))
            .collect::<Vec<_>>();
        let any_row_blocks_protected_dispatch = rows.iter().any(|row| {
            row.protected_action_decisions
                .iter()
                .any(|d| d.decision.is_blocked())
        });
        let workspace_id = cards
            .first()
            .map(|card| card.workspace_id.clone())
            .unwrap_or_default();
        Self {
            record_kind: TARGET_DISCOVERY_BETA_PROJECTION_RECORD_KIND.to_owned(),
            schema_version: TARGET_DISCOVERY_BETA_SCHEMA_VERSION,
            projection_id: projection_id.into(),
            workspace_id,
            generated_at: generated_at.into(),
            rows,
            any_row_blocks_protected_dispatch,
            redaction_safe: true,
        }
    }

    /// Convenience: build directly from execution contexts (cards are minted
    /// from the contexts).
    pub fn from_contexts<'a>(
        projection_id: impl Into<String>,
        generated_at: impl Into<String>,
        contexts: impl IntoIterator<Item = &'a ExecutionContext>,
    ) -> Self {
        let contexts: Vec<&ExecutionContext> = contexts.into_iter().collect();
        let cards: Vec<TargetConfidenceCard> = contexts
            .iter()
            .map(|c| TargetConfidenceCard::from_context(c))
            .collect();
        let card_refs: Vec<&TargetConfidenceCard> = cards.iter().collect();
        Self::from_cards_and_contexts(projection_id, generated_at, card_refs, contexts)
    }

    /// Returns the row matching a given card id, if present.
    pub fn row_for_card(&self, card_id: &str) -> Option<&TargetDiscoveryBetaRow> {
        self.rows.iter().find(|row| row.card_ref == card_id)
    }
}

/// Support-export packet bundling alpha cards, host-boundary rows, beta rows,
/// review rows, the coverage manifest, and execution-context provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetDiscoveryBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Export timestamp.
    pub generated_at: String,
    /// Coverage manifest at export time.
    pub coverage_manifest: TargetDiscoveryBetaCoverageManifest,
    /// Beta projection.
    pub projection: TargetDiscoveryBetaProjection,
    /// Alpha target-confidence cards.
    pub cards: Vec<TargetConfidenceCard>,
    /// Alpha host-boundary rows.
    pub host_boundaries: Vec<TargetHostBoundaryRow>,
    /// Alpha review rows.
    pub review_rows: Vec<TargetConfidenceReviewRow>,
    /// Redaction-safe execution-context provenance.
    pub context_provenance: Vec<ExecutionEventProvenance>,
    /// Support-export provenance events carrying the same context objects.
    pub context_provenance_events: Vec<ExecutionProvenanceEvent>,
    /// True because raw paths, command lines, env bodies, and secrets are
    /// excluded.
    pub redaction_safe: bool,
}

impl TargetDiscoveryBetaSupportExport {
    /// Builds the beta support-export packet from canonical execution
    /// contexts.
    pub fn from_contexts<'a>(
        support_export_id: impl Into<String>,
        generated_at: impl Into<String>,
        contexts: impl IntoIterator<Item = &'a ExecutionContext>,
    ) -> Self {
        let support_export_id = support_export_id.into();
        let generated_at = generated_at.into();
        let contexts: Vec<&ExecutionContext> = contexts.into_iter().collect();
        let alpha = TargetConfidenceSupportExport::from_contexts(
            format!("{support_export_id}:alpha"),
            generated_at.clone(),
            contexts.iter().copied(),
        );
        let card_refs: Vec<&TargetConfidenceCard> = alpha.cards.iter().collect();
        let projection = TargetDiscoveryBetaProjection::from_cards_and_contexts(
            format!("{support_export_id}:projection"),
            generated_at.clone(),
            card_refs,
            contexts.iter().copied(),
        );
        let review_rows = alpha
            .cards
            .iter()
            .map(TargetConfidenceReviewRow::from_card)
            .collect::<Vec<_>>();
        let context_provenance = dedupe_context_provenance(
            contexts
                .iter()
                .map(|context| ExecutionEventProvenance::from_context(context)),
        );
        let context_provenance_events = context_provenance
            .iter()
            .map(|provenance| {
                ExecutionProvenanceEvent::new(
                    format!(
                        "execution-provenance-event:target-discovery-beta-export:{}:{}",
                        stable_token(&support_export_id),
                        provenance.context_provenance_id
                    ),
                    ExecutionProvenanceEventClass::SupportExport,
                    support_export_id.clone(),
                    generated_at.clone(),
                    provenance.clone(),
                )
            })
            .collect();
        let workspace_id = alpha.workspace_id.clone();
        Self {
            record_kind: TARGET_DISCOVERY_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TARGET_DISCOVERY_BETA_SCHEMA_VERSION,
            support_export_id: support_export_id.clone(),
            workspace_id,
            generated_at: generated_at.clone(),
            coverage_manifest: TargetDiscoveryBetaCoverageManifest::canonical(
                format!("{support_export_id}:coverage"),
                generated_at.clone(),
            ),
            projection,
            cards: alpha.cards,
            host_boundaries: alpha.host_boundaries,
            review_rows,
            context_provenance,
            context_provenance_events,
            redaction_safe: true,
        }
    }

    /// Deterministic plaintext rendering for support / CLI surfaces.
    pub fn render_plaintext(&self) -> String {
        let mut out = format!(
            "Target discovery beta support export: {}\n",
            self.support_export_id
        );
        out.push_str(&format!("Workspace: {}\n", self.workspace_id));
        out.push_str(&format!("Generated: {}\n", self.generated_at));
        out.push_str(&format!(
            "Rows: {} (any-protected-dispatch-blocked: {})\n",
            self.projection.rows.len(),
            self.projection.any_row_blocks_protected_dispatch
        ));
        for row in &self.projection.rows {
            out.push_str(&row.summary_line());
            out.push('\n');
            for decision in &row.protected_action_decisions {
                out.push_str(&format!(
                    "  - action={}; decision={}; summary={}\n",
                    decision.action_token, decision.decision_token, decision.summary
                ));
            }
        }
        out
    }
}

fn discovery_source_for(
    card: &TargetConfidenceCard,
    context: &ExecutionContext,
) -> DiscoverySourceClass {
    let selected_by = card.selected_by_source_token.as_str();
    if selected_by == "explicit_override" {
        return DiscoverySourceClass::UserDeclared;
    }
    match card.discovery_confidence_class {
        TargetDiscoveryConfidenceClass::ResolverUnavailable => {
            DiscoverySourceClass::ResolverUnavailable
        }
        TargetDiscoveryConfidenceClass::UnresolvedRequiresUser => {
            DiscoverySourceClass::HeuristicParser
        }
        TargetDiscoveryConfidenceClass::InferredFromAmbient => {
            DiscoverySourceClass::HeuristicParser
        }
        TargetDiscoveryConfidenceClass::CanonicalDeclared => {
            // Prebuild runtimes / declared profiles came from a structured
            // adapter unless the surface explicitly overrode.
            DiscoverySourceClass::StructuredAdapter
        }
        TargetDiscoveryConfidenceClass::CanonicalMaterialised => {
            if context.target_identity.target_class == TargetClass::LocalHost {
                DiscoverySourceClass::NativeProtocol
            } else {
                DiscoverySourceClass::StructuredAdapter
            }
        }
        TargetDiscoveryConfidenceClass::ProbedConsistent => DiscoverySourceClass::StructuredAdapter,
        TargetDiscoveryConfidenceClass::ProbedDivergent => DiscoverySourceClass::StructuredAdapter,
    }
}

fn discovery_freshness_for(
    card: &TargetConfidenceCard,
    context: &ExecutionContext,
    source: DiscoverySourceClass,
) -> DiscoveryFreshnessClass {
    match source {
        DiscoverySourceClass::ResolverUnavailable => return DiscoveryFreshnessClass::Unknown,
        DiscoverySourceClass::HeuristicParser => return DiscoveryFreshnessClass::Unknown,
        DiscoverySourceClass::ImportedMetadata => return DiscoveryFreshnessClass::StaleImported,
        DiscoverySourceClass::UserDeclared => {
            if context.target_identity.target_class == TargetClass::LocalHost {
                return DiscoveryFreshnessClass::FreshProbe;
            }
            return DiscoveryFreshnessClass::RecentWithinSession;
        }
        DiscoverySourceClass::NativeProtocol | DiscoverySourceClass::StructuredAdapter => {}
    }
    if card.discovery_confidence_class == TargetDiscoveryConfidenceClass::ProbedDivergent {
        return DiscoveryFreshnessClass::StaleImported;
    }
    if context.mixed_version_drift.state == MixedVersionDriftState::NotNegotiated {
        return DiscoveryFreshnessClass::StaleImported;
    }
    if context.policy_and_trust.trust_state != TrustState::Trusted {
        return DiscoveryFreshnessClass::RecentWithinSession;
    }
    if card.lane == TargetConfidenceLaneClass::HelperBacked {
        return DiscoveryFreshnessClass::RecentWithinSession;
    }
    DiscoveryFreshnessClass::FreshProbe
}

fn supported_capabilities_for(target_class: TargetClass) -> Vec<SupportedCapabilityClass> {
    use SupportedCapabilityClass::*;
    match target_class {
        TargetClass::LocalHost => vec![Run, Test, DebugLaunch, DebugAttach, Build, InspectOnly],
        TargetClass::ContainerLocal | TargetClass::Devcontainer => {
            vec![Run, Test, DebugLaunch, DebugAttach, Build, InspectOnly]
        }
        TargetClass::SshRemote => vec![Run, Test, DebugAttach, Build, InspectOnly],
        TargetClass::RemoteWorkspaceVm => vec![Run, Test, DebugAttach, Build, InspectOnly],
        TargetClass::PrebuildRuntime => vec![Run, Build, InspectOnly],
        TargetClass::ManagedWorkspace => vec![Run, Test, Build, InspectOnly],
        TargetClass::NotebookKernelLocal => vec![Run, Test, DebugAttach, InspectOnly],
        TargetClass::NotebookKernelRemote => vec![Run, Test, InspectOnly],
        TargetClass::AiSandbox => vec![Run, Test, InspectOnly],
    }
}

fn evaluate_protected_action(
    action: ProtectedActionClass,
    card: &TargetConfidenceCard,
    source: DiscoverySourceClass,
    freshness: DiscoveryFreshnessClass,
    supported: &[SupportedCapabilityClass],
) -> ProtectedActionDecisionClass {
    if matches!(action, ProtectedActionClass::ExportArtifact) {
        return ProtectedActionDecisionClass::Allowed;
    }
    if matches!(source, DiscoverySourceClass::ResolverUnavailable) {
        return ProtectedActionDecisionClass::BlockedResolverUnavailable;
    }
    if matches!(source, DiscoverySourceClass::HeuristicParser) {
        return ProtectedActionDecisionClass::BlockedHeuristicTarget;
    }
    if let Some(required) = action.required_capability() {
        if !supported.contains(&required) {
            return ProtectedActionDecisionClass::BlockedUnsupportedCapability;
        }
    }
    if matches!(source, DiscoverySourceClass::ImportedMetadata) {
        return ProtectedActionDecisionClass::BlockedImportedTarget;
    }
    if freshness.requires_refresh_before_dispatch() {
        return ProtectedActionDecisionClass::BlockedFreshnessStale;
    }
    if card.lane == TargetConfidenceLaneClass::HelperBacked
        || card.target_confidence_level_token != "high"
        || !card.divergence_or_inference_reason_tokens.is_empty()
        || card.host_boundary_cue == HostBoundaryCueClass::ManagedWorkspaceBoundary
    {
        return ProtectedActionDecisionClass::RequiresReview;
    }
    ProtectedActionDecisionClass::Allowed
}

fn decision_summary(
    action: ProtectedActionClass,
    decision: ProtectedActionDecisionClass,
    target_id: &str,
) -> String {
    match decision {
        ProtectedActionDecisionClass::Allowed => format!(
            "Action {} is allowed on target {}.",
            action.as_str(),
            target_id
        ),
        ProtectedActionDecisionClass::RequiresReview => format!(
            "Action {} on target {} requires review before dispatch.",
            action.as_str(),
            target_id
        ),
        ProtectedActionDecisionClass::BlockedHeuristicTarget => format!(
            "Action {} on target {} is blocked: target was discovered by a heuristic parser.",
            action.as_str(),
            target_id
        ),
        ProtectedActionDecisionClass::BlockedImportedTarget => format!(
            "Action {} on target {} is blocked: target was imported and not probed locally.",
            action.as_str(),
            target_id
        ),
        ProtectedActionDecisionClass::BlockedUnsupportedCapability => format!(
            "Action {} on target {} is blocked: target does not support the required capability.",
            action.as_str(),
            target_id
        ),
        ProtectedActionDecisionClass::BlockedResolverUnavailable => format!(
            "Action {} on target {} is blocked: discovery resolver is unavailable.",
            action.as_str(),
            target_id
        ),
        ProtectedActionDecisionClass::BlockedFreshnessStale => format!(
            "Action {} on target {} is blocked: discovery is stale and must be refreshed.",
            action.as_str(),
            target_id
        ),
    }
}

fn stable_token(raw: &str) -> String {
    let mut token = String::new();
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            token.push(ch.to_ascii_lowercase());
        } else if !token.ends_with('_') {
            token.push('_');
        }
    }
    let token = token.trim_matches('_').to_owned();
    if token.is_empty() {
        "unnamed".to_owned()
    } else {
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution_context::{
        CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest,
        ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode, ScopeClass,
        ToolchainClass,
    };

    fn resolver() -> ExecutionContextResolver {
        ExecutionContextResolver::new(ExecutionContextResolverConfig {
            workspace_id: "workspace:target-discovery-beta".to_owned(),
            profile_id: Some("profile:default".to_owned()),
            identity_mode: IdentityMode::AccountFreeLocal,
            policy_epoch: 7,
            workspace_default_target_class: TargetClass::LocalHost,
            workspace_default_working_directory: Some("/Users/example/private/project".to_owned()),
            workspace_default_scope_class: ScopeClass::CurrentRoot,
            local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
            environment_capsule_ref: EnvironmentCapsuleRef {
                capsule_id: "capsule:target-discovery-beta".to_owned(),
                capsule_hash: "sha256:target-discovery-beta".to_owned(),
                resolved_schema_version: "1".to_owned(),
                drift_state: CapsuleDriftState::InSync,
            },
            resolver_version: "target-discovery-beta-test".to_owned(),
        })
    }

    fn local_and_helper_contexts() -> (ExecutionContext, ExecutionContext) {
        let mut resolver = resolver();
        let local = resolver.resolve(ExecutionContextRequest::task_seed(
            "task.run.local",
            TrustState::Trusted,
            "2026-05-15T19:40:00Z",
        ));
        let mut helper_request = ExecutionContextRequest::task_seed(
            "task.run.helper",
            TrustState::Restricted,
            "2026-05-15T19:41:00Z",
        );
        helper_request.requested_target_class = Some(TargetClass::ManagedWorkspace);
        helper_request.requested_toolchain_class = Some(ToolchainClass::BuildDriverRuntime);
        let helper = resolver.resolve(helper_request);
        (local, helper)
    }

    #[test]
    fn local_native_row_allows_run_test_and_debug() {
        let (local, _) = local_and_helper_contexts();
        let card = TargetConfidenceCard::from_context(&local);
        let row = TargetDiscoveryBetaRow::from_card_and_context(&card, &local);

        assert_eq!(row.discovery_source, DiscoverySourceClass::NativeProtocol);
        assert_eq!(row.discovery_freshness, DiscoveryFreshnessClass::FreshProbe);
        assert!(row
            .supported_capabilities
            .contains(&SupportedCapabilityClass::DebugLaunch));
        let run = row
            .decision_for(ProtectedActionClass::DispatchRun)
            .expect("dispatch_run row");
        assert_eq!(run.decision, ProtectedActionDecisionClass::Allowed);
        let debug = row
            .decision_for(ProtectedActionClass::DispatchDebugLaunch)
            .expect("dispatch_debug_launch row");
        assert_eq!(debug.decision, ProtectedActionDecisionClass::Allowed);
    }

    #[test]
    fn helper_backed_managed_row_requires_review_or_freshness_refresh() {
        let (_, helper) = local_and_helper_contexts();
        let card = TargetConfidenceCard::from_context(&helper);
        let row = TargetDiscoveryBetaRow::from_card_and_context(&card, &helper);

        // helper has mixed_version_unchecked → stale_imported freshness
        assert_eq!(
            row.discovery_freshness,
            DiscoveryFreshnessClass::StaleImported
        );
        let run = row
            .decision_for(ProtectedActionClass::DispatchRun)
            .expect("dispatch_run row");
        assert_eq!(
            run.decision,
            ProtectedActionDecisionClass::BlockedFreshnessStale
        );
        let export = row
            .decision_for(ProtectedActionClass::ExportArtifact)
            .expect("export row");
        assert_eq!(export.decision, ProtectedActionDecisionClass::Allowed);
    }

    #[test]
    fn projection_and_support_export_round_trip() {
        let (local, helper) = local_and_helper_contexts();
        let export = TargetDiscoveryBetaSupportExport::from_contexts(
            "support-export:target-discovery-beta",
            "2026-05-15T19:42:00Z",
            [&local, &helper],
        );

        assert_eq!(
            export.record_kind,
            TARGET_DISCOVERY_BETA_SUPPORT_EXPORT_RECORD_KIND
        );
        assert_eq!(export.projection.rows.len(), 2);
        assert!(export.projection.any_row_blocks_protected_dispatch);
        let plaintext = export.render_plaintext();
        assert!(plaintext.contains("source=native_protocol"));
        assert!(plaintext.contains("decision=blocked_freshness_stale"));
        assert!(!plaintext.contains("/Users/example/private/project"));

        assert!(export.coverage_manifest.source_rows.len() == DiscoverySourceClass::ALL.len());
        assert!(export
            .coverage_manifest
            .protected_action_tokens
            .contains(&"dispatch_debug_launch".to_owned()));
        assert_eq!(export.cards.len(), 2);
        assert_eq!(export.review_rows.len(), 2);
        assert!(!export.context_provenance.is_empty());
    }

    #[test]
    fn coverage_manifest_pins_canonical_vocabulary() {
        let manifest = TargetDiscoveryBetaCoverageManifest::canonical(
            "target-discovery-beta:canonical",
            "2026-05-15T19:50:00Z",
        );
        assert_eq!(manifest.source_rows.len(), DiscoverySourceClass::ALL.len());
        assert!(manifest
            .source_rows
            .iter()
            .any(|row| row.source == DiscoverySourceClass::HeuristicParser
                && row.blocks_protected_dispatch_by_default));
        assert!(manifest
            .protected_action_decision_tokens
            .contains(&"blocked_heuristic_target".to_owned()));
        assert!(manifest
            .freshness_tokens
            .contains(&"imported_authoritative".to_owned()));
    }

    #[test]
    fn explicit_override_marks_source_as_user_declared() {
        let mut resolver = resolver();
        let mut request = ExecutionContextRequest::local_terminal_seed(
            "terminal.open.override",
            TrustState::Trusted,
            "2026-05-15T19:43:00Z",
        );
        request.override_target_class = Some(TargetClass::SshRemote);
        request.override_working_directory = Some("/srv/code");
        let context = resolver.resolve(request);
        let card = TargetConfidenceCard::from_context(&context);
        let row = TargetDiscoveryBetaRow::from_card_and_context(&card, &context);

        assert_eq!(row.discovery_source, DiscoverySourceClass::UserDeclared);
        let debug_launch = row
            .decision_for(ProtectedActionClass::DispatchDebugLaunch)
            .expect("debug_launch decision row");
        // SshRemote does not advertise DebugLaunch in supported_capabilities.
        assert_eq!(
            debug_launch.decision,
            ProtectedActionDecisionClass::BlockedUnsupportedCapability
        );
        let attach = row
            .decision_for(ProtectedActionClass::DispatchDebugAttach)
            .expect("debug_attach decision row");
        // SshRemote supports DebugAttach, but it is helper-backed so the
        // decision should require review.
        assert_eq!(
            attach.decision,
            ProtectedActionDecisionClass::RequiresReview
        );
    }
}
