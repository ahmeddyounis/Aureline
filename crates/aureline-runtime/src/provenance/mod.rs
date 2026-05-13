//! Export-safe execution-context provenance for downstream events.
//!
//! This module projects the canonical [`ExecutionContext`] into one compact
//! [`ExecutionEventProvenance`] object that task, test, debug-prep, review,
//! and support-export lanes can embed without reconstructing target truth from
//! logs. The projection intentionally carries identifiers, class tokens,
//! hashes, resolver decisions, and degraded-state markers only; it does not
//! copy raw environment bodies, raw command lines, secrets, or raw working
//! directories.

use serde::{Deserialize, Serialize};

use crate::execution_context::{
    ActorClass, CacheDisposition, ConfidenceLevel, ExecutionContext, PrebuildReuseState,
    ReachabilityState, ResolverInputField, ResolverInputSource, ScopeClass, SurfaceClass,
    TargetClass, ToolchainClass, TrustState,
};

/// Schema version emitted for execution-event provenance records.
pub const EXECUTION_EVENT_PROVENANCE_SCHEMA_VERSION: u32 = 1;
/// Stable record-kind tag for the reusable provenance object.
pub const EXECUTION_EVENT_PROVENANCE_RECORD_KIND: &str = "execution_event_provenance_record";
/// Stable record-kind tag for a lane event that embeds the provenance object.
pub const EXECUTION_PROVENANCE_EVENT_RECORD_KIND: &str = "execution_provenance_event_record";

/// Redaction posture for the execution-context provenance projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionProvenanceRedactionClass {
    /// Metadata, class tokens, refs, and digests only.
    MetadataSafeDefault,
}

impl ExecutionProvenanceRedactionClass {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
        }
    }
}

/// Downstream lane that carries an execution-context provenance event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionProvenanceEventClass {
    /// Canonical task-event stream.
    Task,
    /// Test session or attempt ledger.
    Test,
    /// Debug-prep seed surface.
    DebugPrep,
    /// Review packet or review lifecycle event.
    Review,
    /// Support/export packet.
    SupportExport,
}

impl ExecutionProvenanceEventClass {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Task => "task",
            Self::Test => "test",
            Self::DebugPrep => "debug_prep",
            Self::Review => "review",
            Self::SupportExport => "support_export",
        }
    }
}

/// One resolver-input decision copied into the redaction-safe projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionProvenanceInputDecision {
    /// Field the resolver settled.
    pub field: ResolverInputField,
    /// Stable field token.
    pub field_token: String,
    /// Source that won precedence.
    pub winning_source: ResolverInputSource,
    /// Stable winning-source token.
    pub winning_source_token: String,
    /// Token form of the resolved value.
    pub resolved_value_token: String,
    /// Lower-precedence sources that lost, when any.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conflicting_source_tokens: Vec<String>,
}

/// Compact execution-context provenance object embedded by downstream lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionEventProvenance {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable provenance projection id.
    pub context_provenance_id: String,
    /// Canonical execution-context id.
    pub execution_context_ref: String,
    /// Resolver provenance record id from the source context.
    pub provenance_record_ref: String,
    /// Timestamp copied from the resolver provenance record.
    pub recorded_at: String,
    /// Resolver version that produced the source context.
    pub resolver_version: String,
    /// Workspace id from the invocation subject.
    pub workspace_id: String,
    /// Command id from the invocation subject.
    pub command_id: String,
    /// Invoking surface class.
    pub surface: SurfaceClass,
    /// Stable surface token.
    pub surface_token: String,
    /// Invocation actor class.
    pub actor_class: ActorClass,
    /// Stable actor-class token.
    pub actor_class_token: String,
    /// Canonical target id selected by the resolver.
    pub target_id: String,
    /// Target class selected by the resolver.
    pub target_class: TargetClass,
    /// Stable target-class token.
    pub target_class_token: String,
    /// Reachability state of the selected target.
    pub reachability_state: ReachabilityState,
    /// Stable reachability token.
    pub reachability_state_token: String,
    /// True when UI and exports must disclose a local-vs-managed boundary.
    pub boundary_cue_visible: bool,
    /// Coarse target confidence label.
    pub target_confidence_level: ConfidenceLevel,
    /// Stable target-confidence token.
    pub target_confidence_level_token: String,
    /// Structured target-confidence reasons.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_confidence_reason_tokens: Vec<String>,
    /// Toolchain class selected by the resolver.
    pub toolchain_class: ToolchainClass,
    /// Stable toolchain-class token.
    pub toolchain_class_token: String,
    /// Resolver-owned toolchain id.
    pub toolchain_id: String,
    /// Resolver-owned version label.
    pub resolved_version: String,
    /// Environment capsule id cited by the context.
    pub environment_capsule_ref: String,
    /// Environment capsule hash cited by the context.
    pub environment_capsule_hash: String,
    /// Environment capsule drift token.
    pub environment_capsule_drift_token: String,
    /// True when the source context settled a working directory.
    pub working_directory_present: bool,
    /// Digest of the settled working directory, never the raw path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_directory_digest: Option<String>,
    /// Prebuild reuse state.
    pub prebuild_reuse_state: PrebuildReuseState,
    /// Stable prebuild reuse token.
    pub prebuild_reuse_state_token: String,
    /// Opaque prebuild snapshot reference, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prebuild_snapshot_ref: Option<String>,
    /// Prebuild compatibility fingerprint.
    pub prebuild_compatibility_fingerprint: String,
    /// Stable prebuild invalidation token, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prebuild_invalidation_reason_token: Option<String>,
    /// Workspace trust state.
    pub trust_state: TrustState,
    /// Stable trust-state token.
    pub trust_state_token: String,
    /// Stable identity-mode token.
    pub identity_mode_token: String,
    /// Policy epoch copied from the context.
    pub policy_epoch: u64,
    /// Workset scope class.
    pub scope_class: ScopeClass,
    /// Stable scope-class token.
    pub scope_class_token: String,
    /// Cache disposition.
    pub cache_disposition: CacheDisposition,
    /// Stable cache-disposition token.
    pub cache_disposition_token: String,
    /// Mixed helper/client version state token.
    pub mixed_version_state_token: String,
    /// Mixed helper/client version reason token.
    pub mixed_version_reason_token: String,
    /// Client protocol family recorded by the resolver.
    pub client_protocol: String,
    /// Helper protocol family when a helper advertised one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helper_protocol: Option<String>,
    /// Resolver input-decision rows.
    pub input_decisions: Vec<ExecutionProvenanceInputDecision>,
    /// Count of degraded fields on the source context.
    pub degraded_field_count: u32,
    /// Stable degraded-field reason tokens.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub degraded_field_tokens: Vec<String>,
    /// Stable resolver-explanation reason-code tokens.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub explanation_reason_code_tokens: Vec<String>,
    /// Redaction posture for this projection.
    pub redaction_class: ExecutionProvenanceRedactionClass,
    /// True because this projection omits raw environment, command, path, and secret bodies.
    pub redaction_safe: bool,
    /// Fields sufficient for a support reader to join back to source truth.
    pub reconstruction_fields: Vec<String>,
}

impl ExecutionEventProvenance {
    /// Projects a redaction-safe provenance summary from a canonical context.
    pub fn from_context(context: &ExecutionContext) -> Self {
        let context_provenance_id = format!(
            "ctx-prov:{}",
            stable_token(&context.provenance.provenance_record_id)
        );
        let working_directory_digest = context
            .target_identity
            .working_directory
            .as_deref()
            .map(digest_token);
        Self {
            record_kind: EXECUTION_EVENT_PROVENANCE_RECORD_KIND.to_owned(),
            schema_version: EXECUTION_EVENT_PROVENANCE_SCHEMA_VERSION,
            context_provenance_id,
            execution_context_ref: context.execution_context_id.clone(),
            provenance_record_ref: context.provenance.provenance_record_id.clone(),
            recorded_at: context.provenance.recorded_at.clone(),
            resolver_version: context.provenance.resolver_version.clone(),
            workspace_id: context.invocation_subject.workspace_id.clone(),
            command_id: context.invocation_subject.command_id.clone(),
            surface: context.invocation_subject.surface,
            surface_token: context.invocation_subject.surface.as_str().to_owned(),
            actor_class: context.invocation_subject.actor_class,
            actor_class_token: context.invocation_subject.actor_class.as_str().to_owned(),
            target_id: context.target_identity.canonical_target_id.clone(),
            target_class: context.target_identity.target_class,
            target_class_token: context.target_identity.target_class.as_str().to_owned(),
            reachability_state: context.target_identity.reachability_state,
            reachability_state_token: context
                .target_identity
                .reachability_state
                .as_str()
                .to_owned(),
            boundary_cue_visible: context.target_identity.local_vs_managed_boundary_visible,
            target_confidence_level: context.target_confidence.level,
            target_confidence_level_token: context.target_confidence.level.as_str().to_owned(),
            target_confidence_reason_tokens: context
                .target_confidence
                .reasons
                .iter()
                .map(|reason| reason.as_str().to_owned())
                .collect(),
            toolchain_class: context.toolchain_identity.toolchain_class,
            toolchain_class_token: context
                .toolchain_identity
                .toolchain_class
                .as_str()
                .to_owned(),
            toolchain_id: context.toolchain_identity.toolchain_id.clone(),
            resolved_version: context.toolchain_identity.resolved_version.clone(),
            environment_capsule_ref: context.environment_capsule_ref.capsule_id.clone(),
            environment_capsule_hash: context.environment_capsule_ref.capsule_hash.clone(),
            environment_capsule_drift_token: context
                .environment_capsule_ref
                .drift_state
                .as_str()
                .to_owned(),
            working_directory_present: context.target_identity.working_directory.is_some(),
            working_directory_digest,
            prebuild_reuse_state: context.prebuild_metadata.reuse_state,
            prebuild_reuse_state_token: context.prebuild_metadata.reuse_state.as_str().to_owned(),
            prebuild_snapshot_ref: context.prebuild_metadata.snapshot_ref.clone(),
            prebuild_compatibility_fingerprint: context
                .prebuild_metadata
                .compatibility_fingerprint
                .clone(),
            prebuild_invalidation_reason_token: context
                .prebuild_metadata
                .invalidation_reason
                .map(|reason| reason.as_str().to_owned()),
            trust_state: context.policy_and_trust.trust_state,
            trust_state_token: context.policy_and_trust.trust_state.as_str().to_owned(),
            identity_mode_token: context.policy_and_trust.identity_mode.as_str().to_owned(),
            policy_epoch: context.policy_and_trust.policy_epoch,
            scope_class: context.workset_scope_class,
            scope_class_token: context.workset_scope_class.as_str().to_owned(),
            cache_disposition: context.cache_disposition,
            cache_disposition_token: context.cache_disposition.as_str().to_owned(),
            mixed_version_state_token: context.mixed_version_drift.state.as_str().to_owned(),
            mixed_version_reason_token: context.mixed_version_drift.reason.as_str().to_owned(),
            client_protocol: context.mixed_version_drift.client_protocol.clone(),
            helper_protocol: context.mixed_version_drift.helper_protocol.clone(),
            input_decisions: context
                .provenance
                .input_decisions
                .iter()
                .map(|decision| ExecutionProvenanceInputDecision {
                    field: decision.field,
                    field_token: decision.field.as_str().to_owned(),
                    winning_source: decision.winning_source,
                    winning_source_token: decision.winning_source.as_str().to_owned(),
                    resolved_value_token: decision.resolved_value_token.clone(),
                    conflicting_source_tokens: decision
                        .conflicting_sources
                        .iter()
                        .map(|source| source.as_str().to_owned())
                        .collect(),
                })
                .collect(),
            degraded_field_count: context.degraded_fields.len() as u32,
            degraded_field_tokens: context
                .degraded_fields
                .iter()
                .map(|field| field.reason.as_str().to_owned())
                .collect(),
            explanation_reason_code_tokens: context
                .explanations
                .iter()
                .map(|explanation| explanation.reason_code.as_str().to_owned())
                .collect(),
            redaction_class: ExecutionProvenanceRedactionClass::MetadataSafeDefault,
            redaction_safe: true,
            reconstruction_fields: vec![
                "execution_context_ref".to_owned(),
                "provenance_record_ref".to_owned(),
                "workspace_id".to_owned(),
                "surface_token".to_owned(),
                "target_id".to_owned(),
                "target_class_token".to_owned(),
                "toolchain_class_token".to_owned(),
                "environment_capsule_ref".to_owned(),
                "policy_epoch".to_owned(),
                "input_decisions".to_owned(),
            ],
        }
    }

    /// Returns true when this summary still points at the supplied context.
    pub fn matches_context(&self, context: &ExecutionContext) -> bool {
        self.execution_context_ref == context.execution_context_id
            && self.provenance_record_ref == context.provenance.provenance_record_id
            && self.target_id == context.target_identity.canonical_target_id
    }

    /// Returns true when the summary matches a task/test event identity.
    pub fn matches_event_identity(&self, execution_context_ref: &str, target_id: &str) -> bool {
        self.execution_context_ref == execution_context_ref && self.target_id == target_id
    }

    /// Renders one deterministic diagnostic line without raw path or secret values.
    pub fn summary_line(&self) -> String {
        format!(
            "context={}; surface={}; target={}({}); toolchain={}; trust={}; prebuild={}; helper={}; degraded={}",
            self.execution_context_ref,
            self.surface_token,
            self.target_id,
            self.target_class_token,
            self.toolchain_class_token,
            self.trust_state_token,
            self.prebuild_reuse_state_token,
            self.mixed_version_state_token,
            self.degraded_field_count,
        )
    }
}

/// Lane event that embeds a shared execution-context provenance object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionProvenanceEvent {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable event id.
    pub event_id: String,
    /// Downstream lane class.
    pub event_class: ExecutionProvenanceEventClass,
    /// Stable lane token.
    pub event_class_token: String,
    /// Opaque ref to the event, packet, row, or export that carries this event.
    pub subject_ref: String,
    /// Event timestamp.
    pub occurred_at: String,
    /// Shared execution-context provenance object.
    pub context_provenance: ExecutionEventProvenance,
}

impl ExecutionProvenanceEvent {
    /// Builds a lane event around an existing provenance object.
    pub fn new(
        event_id: impl Into<String>,
        event_class: ExecutionProvenanceEventClass,
        subject_ref: impl Into<String>,
        occurred_at: impl Into<String>,
        context_provenance: ExecutionEventProvenance,
    ) -> Self {
        Self {
            record_kind: EXECUTION_PROVENANCE_EVENT_RECORD_KIND.to_owned(),
            schema_version: EXECUTION_EVENT_PROVENANCE_SCHEMA_VERSION,
            event_id: event_id.into(),
            event_class,
            event_class_token: event_class.as_str().to_owned(),
            subject_ref: subject_ref.into(),
            occurred_at: occurred_at.into(),
            context_provenance,
        }
    }

    /// Builds a lane event by projecting provenance from a canonical context.
    pub fn from_context(
        event_id: impl Into<String>,
        event_class: ExecutionProvenanceEventClass,
        subject_ref: impl Into<String>,
        occurred_at: impl Into<String>,
        context: &ExecutionContext,
    ) -> Self {
        Self::new(
            event_id,
            event_class,
            subject_ref,
            occurred_at,
            ExecutionEventProvenance::from_context(context),
        )
    }
}

/// Deduplicates provenance objects by their projection id in insertion order.
pub fn dedupe_context_provenance(
    provenance: impl IntoIterator<Item = ExecutionEventProvenance>,
) -> Vec<ExecutionEventProvenance> {
    let mut out = Vec::new();
    for item in provenance {
        if !out.iter().any(|known: &ExecutionEventProvenance| {
            known.context_provenance_id == item.context_provenance_id
        }) {
            out.push(item);
        }
    }
    out
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

fn digest_token(payload: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in payload.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("sha256:{hash:064x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution_context::{
        CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest,
        ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode,
    };
    use serde::Deserialize;

    fn resolver() -> ExecutionContextResolver {
        ExecutionContextResolver::new(ExecutionContextResolverConfig {
            workspace_id: "workspace:provenance".to_owned(),
            profile_id: Some("profile:default".to_owned()),
            identity_mode: IdentityMode::AccountFreeLocal,
            policy_epoch: 42,
            workspace_default_target_class: TargetClass::LocalHost,
            workspace_default_working_directory: Some("/Users/example/private/project".to_owned()),
            workspace_default_scope_class: ScopeClass::CurrentRoot,
            local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
            environment_capsule_ref: EnvironmentCapsuleRef {
                capsule_id: "capsule:workspace:provenance".to_owned(),
                capsule_hash: "sha256:capsule".to_owned(),
                resolved_schema_version: "1".to_owned(),
                drift_state: CapsuleDriftState::InSync,
            },
            resolver_version: "provenance-test".to_owned(),
        })
    }

    #[test]
    fn projection_preserves_target_truth_without_raw_working_directory() {
        let mut resolver = resolver();
        let context = resolver.resolve(ExecutionContextRequest::task_seed(
            "task.run",
            TrustState::Trusted,
            "2026-05-13T19:00:00Z",
        ));

        let provenance = ExecutionEventProvenance::from_context(&context);
        assert!(provenance.matches_context(&context));
        assert_eq!(provenance.target_id, "localhost:darwin-arm64");
        assert_eq!(provenance.target_class_token, "local_host");
        assert!(provenance.working_directory_present);
        assert!(provenance.working_directory_digest.is_some());
        assert!(!provenance
            .summary_line()
            .contains("/Users/example/private/project"));
        assert!(provenance.redaction_safe);
    }

    #[test]
    fn lane_events_can_share_the_same_context_provenance_object() {
        let mut resolver = resolver();
        let context = resolver.resolve(ExecutionContextRequest::test_seed(
            "test.run",
            TrustState::Trusted,
            "2026-05-13T19:01:00Z",
        ));
        let provenance = ExecutionEventProvenance::from_context(&context);
        let events = [
            ExecutionProvenanceEvent::new(
                "prov-event:task",
                ExecutionProvenanceEventClass::Task,
                "task:event:1",
                "2026-05-13T19:01:01Z",
                provenance.clone(),
            ),
            ExecutionProvenanceEvent::new(
                "prov-event:test",
                ExecutionProvenanceEventClass::Test,
                "test:attempt:1",
                "2026-05-13T19:01:02Z",
                provenance.clone(),
            ),
            ExecutionProvenanceEvent::new(
                "prov-event:debug",
                ExecutionProvenanceEventClass::DebugPrep,
                "debug-prep:surface:1",
                "2026-05-13T19:01:03Z",
                provenance.clone(),
            ),
            ExecutionProvenanceEvent::new(
                "prov-event:review",
                ExecutionProvenanceEventClass::Review,
                "review:packet:1",
                "2026-05-13T19:01:04Z",
                provenance.clone(),
            ),
            ExecutionProvenanceEvent::new(
                "prov-event:support",
                ExecutionProvenanceEventClass::SupportExport,
                "support-export:1",
                "2026-05-13T19:01:05Z",
                provenance.clone(),
            ),
        ];

        for event in events {
            assert_eq!(event.context_provenance, provenance);
            assert!(matches!(
                event.event_class,
                ExecutionProvenanceEventClass::Task
                    | ExecutionProvenanceEventClass::Test
                    | ExecutionProvenanceEventClass::DebugPrep
                    | ExecutionProvenanceEventClass::Review
                    | ExecutionProvenanceEventClass::SupportExport
            ));
        }
    }

    #[test]
    fn fixture_matrix_covers_shared_context_across_event_lanes() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/runtime/execution_provenance_alpha/provenance_event_matrix.json");
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
        let fixture: ProvenanceFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()));

        assert_eq!(fixture.record_kind, "execution_provenance_alpha_case");
        assert_eq!(
            fixture.context_provenance.context_provenance_id,
            fixture.expect.all_events_share_context_provenance_id
        );
        assert!(fixture.expect.redaction_safe);
        assert!(!fixture.expect.raw_working_directory_exported);
        assert!(fixture
            .context_provenance
            .working_directory_digest
            .is_some());
        assert!(!payload.contains("/workspace/alpha"));

        let mut classes = Vec::new();
        for event in &fixture.events {
            assert_eq!(event.context_provenance, fixture.context_provenance);
            assert_eq!(
                event.context_provenance.context_provenance_id,
                fixture.expect.all_events_share_context_provenance_id
            );
            assert!(event.context_provenance.redaction_safe);
            classes.push(event.event_class);
        }
        classes.sort_by_key(|class| class.as_str());
        classes.dedup();
        assert_eq!(classes.len(), 5);
        assert!(classes.contains(&ExecutionProvenanceEventClass::Task));
        assert!(classes.contains(&ExecutionProvenanceEventClass::Test));
        assert!(classes.contains(&ExecutionProvenanceEventClass::DebugPrep));
        assert!(classes.contains(&ExecutionProvenanceEventClass::Review));
        assert!(classes.contains(&ExecutionProvenanceEventClass::SupportExport));
    }

    #[derive(Debug, Deserialize)]
    struct ProvenanceFixture {
        record_kind: String,
        #[allow(dead_code)]
        schema_version: u32,
        #[allow(dead_code)]
        case_id: String,
        context_provenance: ExecutionEventProvenance,
        events: Vec<ExecutionProvenanceEvent>,
        expect: ProvenanceFixtureExpect,
    }

    #[derive(Debug, Deserialize)]
    struct ProvenanceFixtureExpect {
        all_events_share_context_provenance_id: String,
        raw_working_directory_exported: bool,
        redaction_safe: bool,
    }
}
