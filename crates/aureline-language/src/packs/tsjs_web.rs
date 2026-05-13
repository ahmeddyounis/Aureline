use std::collections::BTreeSet;

use aureline_content_safety::{SurfaceFamily, SuspiciousContentClass, TrustClass};
use serde::{Deserialize, Serialize};

use crate::diagnostics::{
    DiagnosticFreshnessClass, DiagnosticSeverityClass, DiagnosticSourceFamily,
    DiagnosticSurfaceClass,
};
use crate::lsp_router::{
    CapabilityClass, FallbackClass, FaultDomainId, HealthState, LocalityClass, ProviderKind,
    RedactionClass, ScopeClaimClass, ScopeLimitClass, SupportClass, SurfaceClass,
};
use crate::tree_sitter::TreeSitterGrammarRegistry;
use crate::tsjs::{
    TsJsQualityActionClass, TsJsQualityPreviewRequirementClass, TsJsQualityRerunPostureClass,
    TsJsQualitySafetyClass, TsJsQualityToolKindClass, TsJsQualityTriggerClass,
};

/// Integer schema version for TS/JS web language-pack artifacts.
pub type TsJsWebLanguagePackSchemaVersion = u32;

/// Schema version used by TS/JS web language-pack artifacts and snapshots.
pub const TSJS_WEB_LANGUAGE_PACK_SCHEMA_VERSION: TsJsWebLanguagePackSchemaVersion = 1;

/// Claim depth represented by a TS/JS web language-pack artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TsJsWebClaimDepthClass {
    /// Alpha-limited launch wedge with explicit known gaps.
    AlphaLimited,
}

/// Support posture claimed for one language inside the web pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TsJsWebLanguageSupportClass {
    /// Syntax, LSP routing, diagnostics, completion, and previewable rename are represented.
    AlphaStandard,
    /// Syntax, diagnostics labels, icons, docs, and quality hooks are represented.
    AlphaBasic,
    /// Only syntax or text fallback is claimed.
    SyntaxFallbackOnly,
}

/// Enablement state emitted by the first language-pack consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TsJsWebLanguagePackEnablementStateClass {
    /// The artifact can enable its declared launch languages from one pack.
    Enabled,
    /// One or more declared grammars could not be resolved through the shared registry.
    DegradedMissingGrammar,
    /// The artifact is internally incomplete and should not enable.
    BlockedInvalidArtifact,
}

/// One launch language row bundled by the TS/JS web alpha pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsWebLanguageRow {
    /// Canonical language id resolved through the grammar registry.
    pub language_id: String,
    /// Human-readable language label.
    pub display_name: String,
    /// Grammar id expected in the shared Tree-sitter registry.
    pub grammar_id: String,
    /// Registry artifact that owns the grammar metadata.
    pub grammar_registry_ref: String,
    /// File extensions admitted by this language row.
    pub file_extensions: Vec<String>,
    /// Support class claimed for this language.
    pub support_class: TsJsWebLanguageSupportClass,
    /// Default provider route used for semantic or fallback answers.
    pub default_provider_ref: String,
    /// Diagnostic profile applied by default.
    pub diagnostics_profile_ref: String,
    /// Default formatter or formatting fallback hook.
    pub formatter_hook_ref: Option<String>,
    /// Icon id used by file trees, tabs, and quick-open rows.
    pub icon_ref: String,
    /// Docs pack refs attached to this row.
    pub docs_pack_refs: Vec<String>,
    /// Known-gap refs that narrow this row.
    pub known_gap_refs: Vec<String>,
    /// User-visible fallback label for degraded conditions.
    pub fallback_label: String,
}

/// Provider route declared by the web language pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsWebProviderRoute {
    /// Stable provider reference.
    pub provider_ref: String,
    /// Provider implementation kind.
    pub provider_kind: ProviderKind,
    /// Human-readable provider label.
    pub provider_display_label: String,
    /// Languages served by this route.
    pub language_ids: Vec<String>,
    /// Capabilities exposed by this route.
    pub capability_classes: Vec<CapabilityClass>,
    /// Surfaces that may consume this route.
    pub surface_classes: Vec<SurfaceClass>,
    /// Authority class for the route.
    pub support_class: SupportClass,
    /// Expected activation health before live supervision narrows it.
    pub expected_health_state: HealthState,
    /// Locality where the provider is expected to run.
    pub locality_class: LocalityClass,
    /// Claimed scope for this route.
    pub scope_claim_class: ScopeClaimClass,
    /// Concrete scope limits that must remain visible.
    pub scope_limit_classes: Vec<ScopeLimitClass>,
    /// Fallback path available when this route cannot win.
    pub fallback_class: FallbackClass,
    /// Fault domain that owns restart or unavailability accounting.
    pub fault_domain_id: FaultDomainId,
    /// Export-safe route summary.
    pub summary: String,
}

/// Diagnostic default applied by the language pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsWebDiagnosticsDefault {
    /// Stable diagnostics profile reference.
    pub diagnostics_profile_ref: String,
    /// Languages covered by this profile.
    pub language_ids: Vec<String>,
    /// Diagnostic source family emitted by this profile.
    pub source_family: DiagnosticSourceFamily,
    /// Default severity for this profile.
    pub default_severity_class: DiagnosticSeverityClass,
    /// Default freshness for new findings.
    pub default_freshness_class: DiagnosticFreshnessClass,
    /// Surfaces that consume this profile.
    pub surface_classes: Vec<DiagnosticSurfaceClass>,
    /// Export-safe profile summary.
    pub summary: String,
}

/// Formatter, linter, or test hook declared by the language pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsWebToolHook {
    /// Stable tool-hook reference.
    pub tool_hook_ref: String,
    /// Tool lane covered by this hook.
    pub tool_kind_class: TsJsQualityToolKindClass,
    /// Action admitted by the hook.
    pub action_class: TsJsQualityActionClass,
    /// Trigger that admits the action.
    pub trigger_class: TsJsQualityTriggerClass,
    /// Provider route backing the hook.
    pub provider_ref: String,
    /// Canonical command id for UI, CLI, and support traces.
    pub canonical_command_id: String,
    /// Languages covered by this hook.
    pub language_ids: Vec<String>,
    /// Rerun posture exported to execution-plane consumers.
    pub rerun_posture_class: TsJsQualityRerunPostureClass,
    /// Preview requirement before mutation or rerun.
    pub preview_requirement_class: TsJsQualityPreviewRequirementClass,
    /// Fix-safety class for the hook.
    pub safety_class: TsJsQualitySafetyClass,
    /// Export-safe hook summary.
    pub summary: String,
}

/// Icon metadata bound to one or more launch languages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsWebIconRow {
    /// Stable icon reference.
    pub icon_ref: String,
    /// Languages covered by this icon.
    pub language_ids: Vec<String>,
    /// Symbol name used by theme/icon registries.
    pub icon_symbol: String,
    /// Theme token used when icons need a color cue.
    pub theme_token_ref: String,
    /// Export-safe icon summary.
    pub summary: String,
}

/// Docs or tour pack attached to the language pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsWebDocsPackRef {
    /// Stable docs pack reference.
    pub pack_ref: String,
    /// User-visible docs pack label.
    pub label: String,
    /// Source document or artifact.
    pub source_ref: String,
    /// Whether the docs pack can be mirrored for offline review.
    pub mirrorable: bool,
    /// Export-safe docs summary.
    pub summary: String,
}

/// Content-integrity posture applied by the language pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsWebTrustAndIntegrityPolicy {
    /// Default safe-preview trust class for source and manifest labels.
    pub default_trust_class: TrustClass,
    /// Surfaces that must preserve suspicious-content cues.
    pub surface_families: Vec<SurfaceFamily>,
    /// Suspicious-content classes this pack preserves.
    pub suspicious_content_classes: Vec<SuspiciousContentClass>,
    /// Copy/export actions that must distinguish source representation.
    pub copy_export_action_refs: Vec<String>,
    /// Whether support exports preserve trust and representation labels.
    pub support_export_preserves_labels: bool,
    /// Whether suspicious-content fixes require a previewable diff.
    pub suspicious_text_fix_requires_preview: bool,
    /// Export-safe policy summary.
    pub summary: String,
}

/// Known gap that narrows the alpha claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsWebKnownGapRow {
    /// Stable known-gap reference.
    pub gap_ref: String,
    /// Pack or language surface narrowed by the gap.
    pub applies_to_ref: String,
    /// Support effect of the gap.
    pub support_effect: String,
    /// Fallback label shown to users and support.
    pub fallback_label: String,
    /// Docs ref that explains the gap.
    pub docs_ref: String,
}

/// Protected flow that should enable from the pack rather than per-file setup.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsWebEnablementFlow {
    /// Stable flow reference.
    pub flow_ref: String,
    /// Human-readable flow label.
    pub title: String,
    /// Languages admitted by the flow.
    pub language_ids: Vec<String>,
    /// Provider routes required by the flow.
    pub required_provider_refs: Vec<String>,
    /// Tool hooks required by the flow.
    pub required_tool_hook_refs: Vec<String>,
    /// Docs packs required by the flow.
    pub required_docs_pack_refs: Vec<String>,
    /// Whether the flow still requires manual per-file assembly.
    pub manual_per_file_assembly_required: bool,
    /// Whether mutations require preview before apply.
    pub preview_required_before_mutation: bool,
    /// Export-safe flow summary.
    pub summary: String,
}

/// Canonical TS/JS web alpha language-pack artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsWebLanguagePackManifest {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: TsJsWebLanguagePackSchemaVersion,
    /// Stable pack id.
    pub pack_id: String,
    /// Monotonic pack revision.
    pub pack_revision: u32,
    /// Human-readable pack name.
    pub display_name: String,
    /// Release channel this artifact is scoped to.
    pub channel: String,
    /// Claim depth represented by this artifact.
    pub claim_depth_class: TsJsWebClaimDepthClass,
    /// Human-readable support claim.
    pub support_claim: String,
    /// Boundary text that prevents framework-depth overclaiming.
    pub claim_boundary: String,
    /// Source contracts and artifacts consumed by the pack.
    pub source_refs: Vec<String>,
    /// Launch language rows covered by the pack.
    pub language_rows: Vec<TsJsWebLanguageRow>,
    /// Provider routes exposed by the pack.
    pub provider_routes: Vec<TsJsWebProviderRoute>,
    /// Diagnostic defaults applied by the pack.
    pub diagnostics_defaults: Vec<TsJsWebDiagnosticsDefault>,
    /// Formatter, linter, and test hooks exposed by the pack.
    pub tool_hooks: Vec<TsJsWebToolHook>,
    /// Icon rows exposed by the pack.
    pub icon_rows: Vec<TsJsWebIconRow>,
    /// Docs and tour packs attached to the pack.
    pub docs_pack_refs: Vec<TsJsWebDocsPackRef>,
    /// Trust and content-integrity policy for pack surfaces.
    pub trust_and_integrity: TsJsWebTrustAndIntegrityPolicy,
    /// Known gaps that narrow the claim.
    pub known_gap_rows: Vec<TsJsWebKnownGapRow>,
    /// Protected enablement flows covered by this artifact.
    pub enablement_flows: Vec<TsJsWebEnablementFlow>,
    /// Export-safe manifest summary.
    pub export_safe_summary: String,
}

impl TsJsWebLanguagePackManifest {
    /// Stable record-kind tag carried in serialized language-pack artifacts.
    pub const RECORD_KIND: &'static str = "tsjs_web_language_pack_alpha";
}

/// Request for a deterministic language-pack enablement snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsWebLanguagePackEnablementRequest {
    /// Workspace id consuming the pack.
    pub workspace_id: String,
    /// Active workset id consuming the pack.
    pub workset_id: String,
    /// Subject root receiving pack activation.
    pub subject_root_ref: String,
    /// Execution context anchoring toolchain identity.
    pub execution_context_id: String,
    /// Capture timestamp.
    pub captured_at: String,
}

/// First-consumer projection proving the pack can enable as one artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsWebLanguagePackEnablementSnapshot {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: TsJsWebLanguagePackSchemaVersion,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Pack id consumed by this snapshot.
    pub pack_id: String,
    /// Workspace id consuming the pack.
    pub workspace_id: String,
    /// Active workset id consuming the pack.
    pub workset_id: String,
    /// Subject root receiving pack activation.
    pub subject_root_ref: String,
    /// Execution context anchoring toolchain identity.
    pub execution_context_id: String,
    /// Enablement state for this snapshot.
    pub enablement_state_class: TsJsWebLanguagePackEnablementStateClass,
    /// Languages enabled by the pack.
    pub enabled_language_ids: Vec<String>,
    /// File globs activated by the pack.
    pub activation_globs: Vec<String>,
    /// Grammar entries resolved through the shared registry.
    pub grammar_entry_refs: Vec<String>,
    /// Languages whose grammar could not be resolved or matched.
    pub missing_grammar_language_ids: Vec<String>,
    /// Provider routes exposed by this pack.
    pub provider_route_refs: Vec<String>,
    /// Diagnostic profiles exposed by this pack.
    pub diagnostics_profile_refs: Vec<String>,
    /// Tool hooks exposed by this pack.
    pub tool_hook_refs: Vec<String>,
    /// Icons exposed by this pack.
    pub icon_refs: Vec<String>,
    /// Docs packs exposed by this pack.
    pub docs_pack_refs: Vec<String>,
    /// Known gaps that narrow the pack.
    pub known_gap_refs: Vec<String>,
    /// Default safe-preview trust class.
    pub default_trust_class: TrustClass,
    /// Suspicious-content classes preserved by the pack.
    pub suspicious_content_classes: Vec<SuspiciousContentClass>,
    /// Whether protected flows avoid manual per-file assembly.
    pub can_enable_without_per_file_assembly: bool,
    /// Whether the artifact preserves an alpha-bounded claim.
    pub scope_is_bounded_alpha: bool,
    /// Whether downstream surfaces must show fallback or known-gap labels.
    pub fallback_label_required: bool,
    /// Redaction posture for support exports.
    pub redaction_class: RedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe snapshot summary.
    pub export_safe_summary: String,
}

impl TsJsWebLanguagePackEnablementSnapshot {
    /// Stable record-kind tag for enablement snapshots.
    pub const RECORD_KIND: &'static str = "tsjs_web_language_pack_enablement_snapshot";
}

/// Runtime consumer for the TS/JS web language-pack artifact.
#[derive(Debug, Clone)]
pub struct TsJsWebLanguagePack {
    manifest: TsJsWebLanguagePackManifest,
}

impl TsJsWebLanguagePack {
    /// Builds a runtime consumer from a checked-in pack manifest.
    pub fn new(manifest: TsJsWebLanguagePackManifest) -> Self {
        Self { manifest }
    }

    /// Returns the manifest backing this runtime consumer.
    pub const fn manifest(&self) -> &TsJsWebLanguagePackManifest {
        &self.manifest
    }

    /// Builds an enablement snapshot by resolving declared grammars and pack refs.
    pub fn enablement_snapshot(
        &self,
        registry: &TreeSitterGrammarRegistry,
        request: TsJsWebLanguagePackEnablementRequest,
    ) -> TsJsWebLanguagePackEnablementSnapshot {
        let (grammar_entry_refs, missing_grammar_language_ids) = self.resolve_grammars(registry);
        let can_enable_without_per_file_assembly = self.can_enable_without_per_file_assembly();
        let scope_is_bounded_alpha = self.scope_is_bounded_alpha();
        let enablement_state_class = if !missing_grammar_language_ids.is_empty() {
            TsJsWebLanguagePackEnablementStateClass::DegradedMissingGrammar
        } else if can_enable_without_per_file_assembly && scope_is_bounded_alpha {
            TsJsWebLanguagePackEnablementStateClass::Enabled
        } else {
            TsJsWebLanguagePackEnablementStateClass::BlockedInvalidArtifact
        };

        TsJsWebLanguagePackEnablementSnapshot {
            record_kind: TsJsWebLanguagePackEnablementSnapshot::RECORD_KIND.into(),
            schema_version: TSJS_WEB_LANGUAGE_PACK_SCHEMA_VERSION,
            snapshot_id: format!(
                "language_pack_snapshot:{}:{}",
                sanitize_id(&self.manifest.pack_id),
                sanitize_id(&request.subject_root_ref)
            ),
            pack_id: self.manifest.pack_id.clone(),
            workspace_id: request.workspace_id,
            workset_id: request.workset_id,
            subject_root_ref: request.subject_root_ref,
            execution_context_id: request.execution_context_id,
            enablement_state_class,
            enabled_language_ids: self.language_ids(),
            activation_globs: self.activation_globs(),
            grammar_entry_refs,
            missing_grammar_language_ids,
            provider_route_refs: self.provider_route_refs(),
            diagnostics_profile_refs: self.diagnostics_profile_refs(),
            tool_hook_refs: self.tool_hook_refs(),
            icon_refs: self.icon_refs(),
            docs_pack_refs: self.docs_pack_refs(),
            known_gap_refs: self.known_gap_refs(),
            default_trust_class: self.manifest.trust_and_integrity.default_trust_class,
            suspicious_content_classes: self
                .manifest
                .trust_and_integrity
                .suspicious_content_classes
                .clone(),
            can_enable_without_per_file_assembly,
            scope_is_bounded_alpha,
            fallback_label_required: self.fallback_label_required(),
            redaction_class: RedactionClass::MetadataSafeDefault,
            captured_at: request.captured_at,
            export_safe_summary: format!(
                "{} resolves {} launch languages, {} provider routes, and {} tool hooks.",
                self.manifest.display_name,
                self.manifest.language_rows.len(),
                self.manifest.provider_routes.len(),
                self.manifest.tool_hooks.len()
            ),
        }
    }

    fn resolve_grammars(&self, registry: &TreeSitterGrammarRegistry) -> (Vec<String>, Vec<String>) {
        let mut grammar_entry_refs = Vec::new();
        let mut missing_grammar_language_ids = Vec::new();

        for row in &self.manifest.language_rows {
            match registry.resolve_language_id(&row.language_id) {
                Some(descriptor) if descriptor.grammar_id == row.grammar_id => {
                    grammar_entry_refs
                        .push(format!("{}#{}", row.grammar_registry_ref, row.grammar_id));
                }
                _ => missing_grammar_language_ids.push(row.language_id.clone()),
            }
        }

        (grammar_entry_refs, missing_grammar_language_ids)
    }

    fn can_enable_without_per_file_assembly(&self) -> bool {
        if self.manifest.language_rows.is_empty()
            || self.manifest.provider_routes.is_empty()
            || self.manifest.diagnostics_defaults.is_empty()
            || self.manifest.tool_hooks.is_empty()
            || self.manifest.docs_pack_refs.is_empty()
        {
            return false;
        }

        let provider_refs = self
            .manifest
            .provider_routes
            .iter()
            .map(|route| route.provider_ref.as_str())
            .collect::<BTreeSet<_>>();
        let tool_hook_refs = self
            .manifest
            .tool_hooks
            .iter()
            .map(|hook| hook.tool_hook_ref.as_str())
            .collect::<BTreeSet<_>>();
        let docs_pack_refs = self
            .manifest
            .docs_pack_refs
            .iter()
            .map(|doc| doc.pack_ref.as_str())
            .collect::<BTreeSet<_>>();

        let language_rows_complete = self.manifest.language_rows.iter().all(|row| {
            !row.language_id.is_empty()
                && !row.grammar_id.is_empty()
                && !row.file_extensions.is_empty()
                && !row.default_provider_ref.is_empty()
                && !row.diagnostics_profile_ref.is_empty()
                && !row.icon_ref.is_empty()
        });
        let flows_complete = self.manifest.enablement_flows.iter().all(|flow| {
            !flow.manual_per_file_assembly_required
                && flow
                    .required_provider_refs
                    .iter()
                    .all(|provider_ref| provider_refs.contains(provider_ref.as_str()))
                && flow
                    .required_tool_hook_refs
                    .iter()
                    .all(|hook_ref| tool_hook_refs.contains(hook_ref.as_str()))
                && flow
                    .required_docs_pack_refs
                    .iter()
                    .all(|doc_ref| docs_pack_refs.contains(doc_ref.as_str()))
        });

        language_rows_complete && flows_complete
    }

    fn scope_is_bounded_alpha(&self) -> bool {
        self.manifest.claim_depth_class == TsJsWebClaimDepthClass::AlphaLimited
            && self
                .manifest
                .known_gap_rows
                .iter()
                .any(|gap| gap.gap_ref == "known_gap:tsjs_web.framework_expert_depth_not_claimed")
            && self.manifest.language_rows.iter().all(|row| {
                !matches!(
                    row.support_class,
                    TsJsWebLanguageSupportClass::SyntaxFallbackOnly
                )
            })
    }

    fn fallback_label_required(&self) -> bool {
        self.manifest
            .language_rows
            .iter()
            .any(|row| !row.known_gap_refs.is_empty() || !row.fallback_label.is_empty())
            || self
                .manifest
                .provider_routes
                .iter()
                .any(|route| route.fallback_class != FallbackClass::NoFallback)
            || !self.manifest.known_gap_rows.is_empty()
    }

    fn language_ids(&self) -> Vec<String> {
        self.manifest
            .language_rows
            .iter()
            .map(|row| row.language_id.clone())
            .collect()
    }

    fn activation_globs(&self) -> Vec<String> {
        self.manifest
            .language_rows
            .iter()
            .flat_map(|row| row.file_extensions.iter())
            .map(|extension| format!("**/*.{}", extension.trim_start_matches('.')))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    fn provider_route_refs(&self) -> Vec<String> {
        self.manifest
            .provider_routes
            .iter()
            .map(|route| route.provider_ref.clone())
            .collect()
    }

    fn diagnostics_profile_refs(&self) -> Vec<String> {
        self.manifest
            .diagnostics_defaults
            .iter()
            .map(|profile| profile.diagnostics_profile_ref.clone())
            .collect()
    }

    fn tool_hook_refs(&self) -> Vec<String> {
        self.manifest
            .tool_hooks
            .iter()
            .map(|hook| hook.tool_hook_ref.clone())
            .collect()
    }

    fn icon_refs(&self) -> Vec<String> {
        self.manifest
            .icon_rows
            .iter()
            .map(|icon| icon.icon_ref.clone())
            .collect()
    }

    fn docs_pack_refs(&self) -> Vec<String> {
        self.manifest
            .docs_pack_refs
            .iter()
            .map(|doc| doc.pack_ref.clone())
            .collect()
    }

    fn known_gap_refs(&self) -> Vec<String> {
        self.manifest
            .known_gap_rows
            .iter()
            .map(|gap| gap.gap_ref.clone())
            .collect()
    }
}

fn sanitize_id(value: &str) -> String {
    value
        .trim()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect()
}
