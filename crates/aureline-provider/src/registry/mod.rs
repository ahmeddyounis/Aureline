//! Registry records for provider descriptors, surface claims, and CI overlays.
//!
//! The registry module is the typed consumer for
//! `/schemas/providers/connected_provider_registry.schema.json`. It validates
//! descriptor coverage, mutation-mode disclosure, CI overlay truth, and
//! run-control review posture before any UI, CLI, support, or automation surface
//! can claim a provider lane.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::publish_later::{PublishLaterQueueAlphaItem, QueueState};

/// Schema version for the connected-provider registry alpha packet.
pub const CONNECTED_PROVIDER_REGISTRY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ConnectedProviderAlphaPacket`].
pub const CONNECTED_PROVIDER_ALPHA_PACKET_RECORD_KIND: &str = "connected_provider_alpha_packet";

/// Stable record-kind tag for [`ConnectedProviderDescriptor`].
pub const CONNECTED_PROVIDER_DESCRIPTOR_RECORD_KIND: &str = "connected_provider_descriptor_record";

/// Stable record-kind tag for [`ClaimedProviderSurface`].
pub const PROVIDER_SURFACE_CLAIM_RECORD_KIND: &str = "claimed_provider_surface_record";

/// Stable record-kind tag for [`PipelineOverlayDescriptor`].
pub const PIPELINE_OVERLAY_DESCRIPTOR_RECORD_KIND: &str = "pipeline_overlay_descriptor_record";

/// Stable record-kind tag for [`RunControlDescriptor`].
pub const RUN_CONTROL_DESCRIPTOR_RECORD_KIND: &str = "run_control_descriptor_record";

/// Stable record-kind tag for [`ProviderAlphaValidationReport`].
pub const PROVIDER_ALPHA_VALIDATION_REPORT_RECORD_KIND: &str = "provider_alpha_validation_report";

/// Stable record-kind tag for [`ProviderAlphaSupportExport`].
pub const PROVIDER_ALPHA_SUPPORT_EXPORT_RECORD_KIND: &str = "provider_alpha_support_export";

/// Fixture metadata used by protected cases and validation captures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderFixtureMetadata {
    /// Short fixture name.
    pub name: String,
    /// Human-readable scenario summary that is safe for support exports.
    pub scenario: String,
    /// Closed axes the fixture intends to exercise.
    #[serde(default)]
    pub exercised_axes: BTreeMap<String, Vec<String>>,
}

/// One fixture packet containing registry records and publish-later rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectedProviderAlphaPacket {
    /// Optional fixture metadata for validation lanes.
    #[serde(default, rename = "__fixture__")]
    pub fixture_metadata: Option<ProviderFixtureMetadata>,
    /// Registry records for descriptors, surface claims, overlays, and controls.
    pub registry: ConnectedProviderRegistryPacket,
    /// Publish-later queue rows the claimed surfaces reference.
    #[serde(default)]
    pub publish_later_queue: Vec<PublishLaterQueueAlphaItem>,
}

impl ConnectedProviderAlphaPacket {
    /// Validates provider registry and queue invariants for alpha surfaces.
    pub fn validate(&self) -> ProviderAlphaValidationReport {
        let mut validator = ProviderAlphaValidator::new(self);
        validator.validate();
        validator.finish()
    }

    /// Builds an export-safe projection for support and reviewer surfaces.
    pub fn support_export_projection(&self) -> ProviderAlphaSupportExport {
        let descriptor_summaries = self
            .registry
            .descriptors
            .iter()
            .map(|descriptor| ProviderDescriptorSummary {
                descriptor_id: descriptor.descriptor_id.clone(),
                provider_family: descriptor.provider_family,
                surface_class: descriptor.surface_class,
                source_class: descriptor.source.source_class,
                actor_class: descriptor.actor_scope.primary_actor_class,
                freshness_class: descriptor.freshness.freshness_class,
                supported_states: descriptor.supported_surface_states.clone(),
            })
            .collect();

        let queue_summaries = self
            .publish_later_queue
            .iter()
            .map(|item| ProviderQueueSummary {
                queue_item_id: item.queue_item_id.clone(),
                provider_descriptor_ref: item.provider_descriptor_ref.clone(),
                target_ref: item.target_ref.clone(),
                action_kind: item.action_kind,
                queue_state: item.queue_state,
                stale_target_risk_class: item.stale_target_risk_class,
                reauth_requirement: item.reauth_requirement,
                rescope_requirement: item.rescope_requirement,
                next_safe_action: item.next_safe_action,
                support_export_summary: item.support_export_summary.clone(),
            })
            .collect();

        let run_control_summaries = self
            .registry
            .run_controls
            .iter()
            .map(|control| ProviderRunControlSummary {
                run_control_id: control.run_control_id.clone(),
                control_class: control.control_class,
                mutation_mode: control.mutation_mode,
                upstream_mutation_scope: control.upstream_mutation_scope,
                auth_source_class: control.auth_source_class,
                target_ref: control.target_ref.clone(),
                stale_target_risk_class: control.stale_target_risk_class,
                disclosure_summary: control.disclosure_summary.clone(),
            })
            .collect();

        ProviderAlphaSupportExport {
            record_kind: PROVIDER_ALPHA_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: CONNECTED_PROVIDER_REGISTRY_SCHEMA_VERSION,
            packet_id: self.registry.packet_id.clone(),
            descriptor_summaries,
            queue_summaries,
            run_control_summaries,
            redaction_class: RedactionClass::MetadataSafeDefault,
        }
    }
}

/// Registry packet containing all descriptor records for one alpha slice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectedProviderRegistryPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this packet.
    pub connected_provider_registry_schema_version: u32,
    /// Opaque packet id.
    pub packet_id: String,
    /// Existing upstream contracts this packet consumes by reference.
    pub contract_refs: ContractRefs,
    /// Provider descriptors claimed by this packet.
    #[serde(default)]
    pub descriptors: Vec<ConnectedProviderDescriptor>,
    /// Surface claims projected from the descriptors.
    #[serde(default)]
    pub surface_claims: Vec<ClaimedProviderSurface>,
    /// Pipeline overlay descriptors projected from CI/check providers.
    #[serde(default)]
    pub pipeline_overlays: Vec<PipelineOverlayDescriptor>,
    /// Rerun, cancel, and retry control descriptors.
    #[serde(default)]
    pub run_controls: Vec<RunControlDescriptor>,
}

/// References to upstream schema and contract files consumed by this packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractRefs {
    /// Existing connected-account registry schema reference.
    pub connected_account_registry_schema_ref: String,
    /// Existing publish-later record schema reference.
    pub publish_later_record_schema_ref: String,
    /// Existing deferred-publish queue item schema reference.
    pub deferred_publish_queue_schema_ref: String,
    /// Existing pipeline run-row schema reference.
    pub pipeline_run_row_schema_ref: String,
    /// Existing pipeline log-view schema reference.
    pub pipeline_log_view_schema_ref: String,
    /// Existing pipeline artifact-card schema reference.
    pub pipeline_artifact_card_schema_ref: String,
    /// Existing pipeline annotation-row schema reference.
    pub pipeline_annotation_row_schema_ref: String,
    /// Existing run-control review schema reference.
    pub run_control_review_schema_ref: String,
}

impl ContractRefs {
    fn all_refs(&self) -> [&str; 8] {
        [
            &self.connected_account_registry_schema_ref,
            &self.publish_later_record_schema_ref,
            &self.deferred_publish_queue_schema_ref,
            &self.pipeline_run_row_schema_ref,
            &self.pipeline_log_view_schema_ref,
            &self.pipeline_artifact_card_schema_ref,
            &self.pipeline_annotation_row_schema_ref,
            &self.run_control_review_schema_ref,
        ]
    }
}

/// Descriptor for one connected provider family claimed by alpha surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectedProviderDescriptor {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this descriptor.
    pub connected_provider_registry_schema_version: u32,
    /// Opaque descriptor id.
    pub descriptor_id: String,
    /// Opaque connected provider record reference.
    pub connected_provider_record_ref: String,
    /// Provider family covered by this descriptor.
    pub provider_family: ProviderFamily,
    /// Provider-linked surface class for this descriptor.
    pub surface_class: ProviderSurfaceClass,
    /// Redaction-safe source metadata.
    pub source: ProviderSource,
    /// Actor and authority scope used by actions on this descriptor.
    pub actor_scope: ActorScope,
    /// Freshness truth for the descriptor.
    pub freshness: FreshnessTruth,
    /// Provider-side object kinds supported by this descriptor.
    pub supported_object_kinds: Vec<ProviderObjectKind>,
    /// Mutation states this descriptor may render.
    pub supported_surface_states: Vec<MutationSurfaceState>,
    /// Fallback modes offered when mutation cannot proceed in product.
    pub fallback_modes: Vec<ProviderFallbackMode>,
    /// Redaction posture for descriptor exports.
    pub redaction_class: RedactionClass,
}

/// Claimed provider surface with explicit action-state distinctions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimedProviderSurface {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this surface claim.
    pub connected_provider_registry_schema_version: u32,
    /// Opaque surface id.
    pub surface_id: String,
    /// Descriptor this surface reads from.
    pub provider_descriptor_ref: String,
    /// Surface class rendered by the consumer.
    pub surface_class: ProviderSurfaceClass,
    /// Actions and states rendered by this surface.
    pub actions: Vec<SurfaceActionDescriptor>,
    /// Export-safe summary of what the surface claims.
    pub surface_summary: String,
}

/// One action state exposed by a claimed provider surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceActionDescriptor {
    /// Opaque action id.
    pub action_id: String,
    /// Redaction-safe action label.
    pub action_label: String,
    /// Mutation state shown at the point of intent.
    pub mutation_state: MutationSurfaceState,
    /// Local draft reference required for local-draft actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_draft_ref: Option<String>,
    /// Approval-ticket reference required for publish-now actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ticket_ref: Option<String>,
    /// Browser-handoff packet reference required for open-in-provider actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Publish-later queue item reference required for queue actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_later_queue_item_ref: Option<String>,
    /// Inspect-only snapshot reference for read-only provider imports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_snapshot_ref: Option<String>,
}

/// CI pipeline overlay descriptor consumed by launch-wedge provider surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PipelineOverlayDescriptor {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this overlay descriptor.
    pub connected_provider_registry_schema_version: u32,
    /// Opaque overlay id.
    pub overlay_id: String,
    /// Descriptor this overlay reads from.
    pub provider_descriptor_ref: String,
    /// Kind of pipeline overlay represented.
    pub overlay_kind: PipelineOverlayKind,
    /// Source of truth for this overlay row.
    pub truth_source_class: ProviderTruthSourceClass,
    /// Target ref the overlay is bound to.
    pub target_ref: TargetRef,
    /// Freshness truth for the overlay.
    pub freshness: FreshnessTruth,
    /// Actor scope associated with the provider import or live overlay.
    pub actor_scope: ActorScope,
    /// Local-vs-provider authority binding.
    pub local_truth_authority_class: LocalTruthAuthorityClass,
    /// Artifact trust class for artifact overlays.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_trust_class: Option<ArtifactTrustClass>,
    /// Existing CI schema record this overlay projects.
    pub schema_ref: String,
    /// Export-safe summary.
    pub overlay_summary: String,
}

/// Run-control descriptor for auditable upstream CI/check mutations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunControlDescriptor {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this control descriptor.
    pub connected_provider_registry_schema_version: u32,
    /// Opaque run-control id.
    pub run_control_id: String,
    /// Descriptor this control reads from.
    pub provider_descriptor_ref: String,
    /// Pipeline overlay this control acts on.
    pub pipeline_overlay_ref: String,
    /// Rerun, cancel, or retry control class.
    pub control_class: RunControlClass,
    /// Mutation mode disclosed before invocation.
    pub mutation_mode: RunControlMutationMode,
    /// Upstream scope affected by the control.
    pub upstream_mutation_scope: UpstreamMutationScopeClass,
    /// Auth source used for the upstream mutation.
    pub auth_source_class: ProviderAuthSourceClass,
    /// Target ref the upstream provider will mutate.
    pub target_ref: TargetRef,
    /// Freshness truth at review time.
    pub freshness: FreshnessTruth,
    /// Actor scope used by the upstream mutation.
    pub actor_scope: ActorScope,
    /// Stale-target risk disclosed before execution.
    pub stale_target_risk_class: StaleTargetRiskClass,
    /// Fallback mode if in-product invocation is not admitted.
    pub fallback_mode: ProviderFallbackMode,
    /// Approval-ticket ref required for in-product publish-now controls.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ticket_ref: Option<String>,
    /// Browser-handoff packet ref required for open-in-provider controls.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Publish-later queue item ref required for deferred controls.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_later_queue_item_ref: Option<String>,
    /// Export-safe review text shown before invocation.
    pub disclosure_summary: String,
    /// Audit event refs minted or expected by this control.
    pub audit_event_refs: Vec<String>,
}

/// Redaction-safe provider source metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderSource {
    /// Class of provider source.
    pub source_class: ProviderSourceClass,
    /// Opaque canonical host reference.
    pub canonical_host_ref: String,
    /// Opaque tenant, org, or project scope reference.
    pub tenant_or_org_scope_ref: String,
    /// Opaque environment reference when relevant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_ref: Option<String>,
}

/// Actor scope and authority used by provider actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorScope {
    /// Primary actor class that provider actions execute as.
    pub primary_actor_class: ProviderActorClass,
    /// Opaque actor subject reference.
    pub actor_subject_ref: String,
    /// Scope refs granted to the actor.
    #[serde(default)]
    pub granted_scope_refs: Vec<String>,
    /// Auth source behind the actor class.
    pub auth_source_class: ProviderAuthSourceClass,
}

/// Freshness truth shared by descriptors, overlays, and controls.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessTruth {
    /// Freshness label rendered by the surface.
    pub freshness_class: FreshnessLabel,
    /// Observation time for the provider truth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_at: Option<String>,
    /// Opaque freshness-floor reference.
    pub freshness_floor_ref: String,
    /// Time or duration after which the observation becomes stale.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stale_after: Option<String>,
    /// Degraded or stale reason shown when freshness is not fresh.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_reason: Option<String>,
    /// Import-session ref when the truth came from a provider import.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_session_ref: Option<String>,
}

/// Target ref identity for provider objects and run controls.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetRef {
    /// Target-ref class.
    pub target_ref_class: String,
    /// Opaque target-ref id.
    pub target_ref: String,
    /// Redaction-safe display label.
    pub target_label: String,
    /// Route-origin label when a provider target is reached through a
    /// route-sensitive handoff such as a tunnel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_origin: Option<ProviderRouteOriginLabel>,
}

/// Route-origin label attached to a provider target ref.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRouteOriginLabel {
    /// Route-choice token from the shared route-origin vocabulary.
    pub route_choice: String,
    /// Target class token associated with the route.
    pub target_class: String,
    /// Redaction-safe route label.
    pub route_label: String,
    /// Transport label such as `SSH tunnel`.
    pub transport_label: String,
    /// Opaque target identity ref preserved for support reconstruction.
    pub target_identity_ref: String,
    /// Tunnel session ref when `route_choice` is `tunnel_exposed_route`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tunnel_session_ref: Option<String>,
    /// Exposure posture associated with the route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exposure_posture: Option<String>,
}

/// Provider family covered by one descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderFamily {
    /// Code-host or review provider.
    CodeHost,
    /// Issue or planning tracker provider.
    IssueTracker,
    /// CI or checks provider.
    CiChecks,
}

/// Provider-linked surface class reused from the provider-mode contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderSurfaceClass {
    /// Code-host surface such as pull requests and branch review.
    CodeHostSurface,
    /// Issue or planning surface.
    IssueOrPlanningSurface,
    /// CI or checks surface.
    CiOrChecksSurface,
}

/// Provider-side object kind named by a descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderObjectKind {
    /// Pull-request or hosted review object.
    PullRequest,
    /// Branch or ref object.
    Branch,
    /// Issue, ticket, incident, or work item.
    IssueOrWorkItem,
    /// CI check run.
    CheckRun,
    /// Pipeline or workflow run.
    PipelineRun,
    /// Pipeline log surface.
    PipelineLog,
    /// Pipeline artifact card.
    PipelineArtifact,
    /// Pipeline annotation row.
    PipelineAnnotation,
}

/// Provider source class for descriptor origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderSourceClass {
    /// Live provider connection.
    LiveProvider,
    /// Cached provider overlay.
    CachedProviderOverlay,
    /// Imported provider snapshot.
    ImportedSnapshot,
    /// Mirrored or self-hosted provider route.
    MirroredOrSelfHosted,
}

/// Provider actor class reused from connected-account records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderActorClass {
    /// Human account.
    HumanAccount,
    /// Installation, app, or bot grant.
    InstallationOrAppGrant,
    /// Delegated user token.
    DelegatedUserToken,
    /// Project-scoped grant.
    ProjectScopedGrant,
    /// Policy-injected service identity.
    PolicyInjectedServiceIdentity,
    /// Unknown actor class requiring repair.
    UnknownActorClass,
}

/// Auth source for a provider actor or run control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderAuthSourceClass {
    /// Signed-in human session.
    HumanSession,
    /// Provider installation grant.
    InstallationGrant,
    /// Delegated credential.
    DelegatedCredential,
    /// Project-scoped grant.
    ProjectScopedGrant,
    /// Policy-injected service identity.
    PolicyInjectedService,
    /// Browser-only authentication source.
    BrowserOnly,
    /// Unknown auth source requiring repair.
    UnknownAuthSource,
}

/// Freshness label rendered by provider surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessLabel {
    /// Provider truth is current enough for the descriptor's freshness floor.
    Fresh,
    /// Provider truth is stale but still inside a bounded review window.
    StaleWithinWindow,
    /// Provider truth is beyond the freshness window and requires re-observe.
    ExpiredBeyondWindow,
    /// Provider has never been observed.
    NeverObserved,
    /// Provider grant or connection was revoked or disconnected.
    RevokedOrDisconnected,
}

/// Mutation state shown by claimed provider surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationSurfaceState {
    /// Action remains a local draft.
    LocalDraft,
    /// Action publishes immediately through reviewed in-product authority.
    PublishNow,
    /// Action opens a typed provider handoff.
    OpenInProvider,
    /// Action enters the publish-later queue.
    PublishLaterQueue,
    /// Action reads provider truth without mutation.
    InspectOnly,
}

/// Fallback mode offered when direct mutation is unavailable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderFallbackMode {
    /// Export or copy an evidence-safe summary.
    CopyOrExport,
    /// Open a typed browser handoff.
    OpenInProvider,
    /// Queue the mutation for later.
    PublishLaterQueue,
    /// Keep the surface inspect-only.
    InspectOnly,
}

/// Pipeline overlay kind represented in the registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PipelineOverlayKind {
    /// Pipeline run row.
    Run,
    /// Pipeline log view.
    Log,
    /// Pipeline artifact card.
    Artifact,
    /// Pipeline annotation row.
    Annotation,
}

/// Source of truth for a provider overlay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderTruthSourceClass {
    /// Current local task or debug truth.
    CurrentLocalTaskTruth,
    /// Imported provider truth.
    ImportedProviderTruth,
    /// Live provider overlay truth.
    LiveProviderTruth,
    /// Cached provider overlay truth.
    CachedProviderTruth,
}

/// Local-vs-provider authority class for pipeline overlays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalTruthAuthorityClass {
    /// Local task truth is authoritative.
    LocalTruthIsAuthoritative,
    /// Provider overlay is authoritative.
    ProviderOverlayIsAuthoritative,
    /// Local and provider truth disagree and require review.
    LocalAndProviderDisagreeReviewRequired,
    /// No local correspondent exists.
    NoLocalCorrespondent,
}

/// Artifact trust class for pipeline artifact overlays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactTrustClass {
    /// Metadata-only artifact row.
    MetadataOnly,
    /// Sanitized rich artifact.
    SanitizedRich,
    /// Isolated remote-active artifact.
    IsolatedRemoteActive,
    /// Raw text artifact.
    RawText,
    /// Download-only artifact.
    DownloadOnly,
}

/// Auditable CI/check run-control class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunControlClass {
    /// Rerun a workflow or failed jobs.
    Rerun,
    /// Cancel an upstream workflow or job.
    Cancel,
    /// Retry a failed step or job.
    Retry,
}

/// Mutation scope affected by a run control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpstreamMutationScopeClass {
    /// Entire workflow run.
    EntireWorkflowRun,
    /// Failed jobs only.
    FailedJobsOnly,
    /// Single job only.
    SingleJobOnly,
    /// Single step only.
    SingleStepOnly,
}

/// Stale-target risk disclosed before provider mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StaleTargetRiskClass {
    /// Target is unchanged relative to the freshness floor.
    TargetUnchanged,
    /// Target may be stale and requires review.
    StaleTargetReviewRequired,
    /// Target identity changed and requires re-targeting.
    TargetIdentityChanged,
    /// Actor scope changed and requires reauth or rescope.
    ActorScopeChanged,
}

/// Mutation mode for run controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunControlMutationMode {
    /// In-product invocation through reviewed authority.
    PublishNow,
    /// Typed browser handoff to provider.
    OpenInProvider,
    /// Deferred publish-later queue item.
    DeferredPublish,
    /// Preview-only or unavailable control.
    InspectOnly,
}

/// Redaction posture shared by registry and queue exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    /// Metadata-safe default export posture.
    MetadataSafeDefault,
    /// Operator-only restricted export posture.
    OperatorOnlyRestricted,
    /// Internal-support restricted export posture.
    InternalSupportRestricted,
    /// Signing evidence only.
    SigningEvidenceOnly,
}

/// Validation report emitted by the first registry consumer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderAlphaValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version validated by this report.
    pub schema_version: u32,
    /// Packet id under validation.
    pub packet_id: String,
    /// Whether all checks passed.
    pub passed: bool,
    /// Coverage observed while validating the packet.
    pub coverage: ProviderAlphaCoverage,
    /// Findings emitted by failed checks.
    pub findings: Vec<ProviderAlphaValidationFinding>,
}

/// Coverage observed during provider alpha validation.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ProviderAlphaCoverage {
    /// Provider families covered by descriptors.
    pub provider_families: BTreeSet<ProviderFamily>,
    /// Mutation surface states rendered by surface claims.
    pub mutation_surface_states: BTreeSet<MutationSurfaceState>,
    /// Pipeline overlay kinds covered by CI descriptors.
    pub pipeline_overlay_kinds: BTreeSet<PipelineOverlayKind>,
    /// Run-control classes covered by descriptors.
    pub run_control_classes: BTreeSet<RunControlClass>,
    /// Queue item states covered by publish-later rows.
    pub queue_states: BTreeSet<QueueState>,
}

/// Validation finding emitted by the first registry consumer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderAlphaValidationFinding {
    /// Severity of the finding.
    pub severity: FindingSeverity,
    /// Stable check id.
    pub check_id: String,
    /// Redaction-safe finding message.
    pub message: String,
}

/// Validation finding severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    /// Error that blocks the packet.
    Error,
}

/// Export-safe provider alpha support projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderAlphaSupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for the projection.
    pub schema_version: u32,
    /// Packet id projected into support export.
    pub packet_id: String,
    /// Descriptor summaries safe for support bundles.
    pub descriptor_summaries: Vec<ProviderDescriptorSummary>,
    /// Queue summaries safe for support bundles.
    pub queue_summaries: Vec<ProviderQueueSummary>,
    /// Run-control summaries safe for support bundles.
    pub run_control_summaries: Vec<ProviderRunControlSummary>,
    /// Redaction posture for the projection.
    pub redaction_class: RedactionClass,
}

/// Export-safe summary for one provider descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderDescriptorSummary {
    /// Opaque descriptor id.
    pub descriptor_id: String,
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Surface class.
    pub surface_class: ProviderSurfaceClass,
    /// Provider source class.
    pub source_class: ProviderSourceClass,
    /// Actor class.
    pub actor_class: ProviderActorClass,
    /// Freshness class.
    pub freshness_class: FreshnessLabel,
    /// Mutation states supported by the descriptor.
    pub supported_states: Vec<MutationSurfaceState>,
}

/// Export-safe summary for one publish-later queue item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderQueueSummary {
    /// Opaque queue item id.
    pub queue_item_id: String,
    /// Provider descriptor ref.
    pub provider_descriptor_ref: String,
    /// Provider target ref including route-origin labels when present.
    pub target_ref: TargetRef,
    /// Queue action kind.
    pub action_kind: crate::publish_later::QueueActionKind,
    /// Queue state.
    pub queue_state: QueueState,
    /// Stale-target risk.
    pub stale_target_risk_class: StaleTargetRiskClass,
    /// Reauth requirement.
    pub reauth_requirement: crate::publish_later::ReauthRequirementClass,
    /// Rescope requirement.
    pub rescope_requirement: crate::publish_later::RescopeRequirementClass,
    /// Next safe action.
    pub next_safe_action: crate::publish_later::QueueNextSafeActionClass,
    /// Redaction-safe support summary.
    pub support_export_summary: String,
}

/// Export-safe summary for one run control.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRunControlSummary {
    /// Opaque run-control id.
    pub run_control_id: String,
    /// Control class.
    pub control_class: RunControlClass,
    /// Mutation mode.
    pub mutation_mode: RunControlMutationMode,
    /// Upstream mutation scope.
    pub upstream_mutation_scope: UpstreamMutationScopeClass,
    /// Auth source class.
    pub auth_source_class: ProviderAuthSourceClass,
    /// Provider target ref including route-origin labels when present.
    pub target_ref: TargetRef,
    /// Stale-target risk class.
    pub stale_target_risk_class: StaleTargetRiskClass,
    /// Redaction-safe disclosure summary.
    pub disclosure_summary: String,
}

struct ProviderAlphaValidator<'a> {
    packet: &'a ConnectedProviderAlphaPacket,
    descriptor_ids: BTreeSet<&'a str>,
    surface_ids: BTreeSet<&'a str>,
    overlay_ids: BTreeSet<&'a str>,
    queue_ids: BTreeSet<&'a str>,
    findings: Vec<ProviderAlphaValidationFinding>,
    coverage: ProviderAlphaCoverage,
}

impl<'a> ProviderAlphaValidator<'a> {
    fn new(packet: &'a ConnectedProviderAlphaPacket) -> Self {
        Self {
            packet,
            descriptor_ids: BTreeSet::new(),
            surface_ids: BTreeSet::new(),
            overlay_ids: BTreeSet::new(),
            queue_ids: BTreeSet::new(),
            findings: Vec::new(),
            coverage: ProviderAlphaCoverage::default(),
        }
    }

    fn validate(&mut self) {
        self.validate_registry_header();
        self.validate_descriptors();
        self.validate_surface_claims();
        self.validate_pipeline_overlays();
        self.validate_run_controls();
        self.validate_publish_later_queue();
        self.validate_required_coverage();
    }

    fn finish(self) -> ProviderAlphaValidationReport {
        ProviderAlphaValidationReport {
            record_kind: PROVIDER_ALPHA_VALIDATION_REPORT_RECORD_KIND.to_string(),
            schema_version: CONNECTED_PROVIDER_REGISTRY_SCHEMA_VERSION,
            packet_id: self.packet.registry.packet_id.clone(),
            passed: self.findings.is_empty(),
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn validate_registry_header(&mut self) {
        let registry = &self.packet.registry;
        self.expect(
            registry.record_kind == CONNECTED_PROVIDER_ALPHA_PACKET_RECORD_KIND,
            "provider_alpha.registry_record_kind",
            "registry.record_kind must be connected_provider_alpha_packet",
        );
        self.expect(
            registry.connected_provider_registry_schema_version
                == CONNECTED_PROVIDER_REGISTRY_SCHEMA_VERSION,
            "provider_alpha.registry_schema_version",
            "registry schema version must match the crate constant",
        );
        for contract_ref in registry.contract_refs.all_refs() {
            self.expect(
                !contract_ref.trim().is_empty(),
                "provider_alpha.contract_ref_missing",
                "every consumed upstream contract ref must be non-empty",
            );
        }
    }

    fn validate_descriptors(&mut self) {
        let descriptors = &self.packet.registry.descriptors;
        self.expect(
            !descriptors.is_empty(),
            "provider_alpha.descriptors_missing",
            "at least one provider descriptor is required",
        );

        for descriptor in descriptors {
            self.expect(
                descriptor.record_kind == CONNECTED_PROVIDER_DESCRIPTOR_RECORD_KIND,
                "provider_alpha.descriptor_record_kind",
                "provider descriptor record_kind is wrong",
            );
            self.expect(
                descriptor.connected_provider_registry_schema_version
                    == CONNECTED_PROVIDER_REGISTRY_SCHEMA_VERSION,
                "provider_alpha.descriptor_schema_version",
                "provider descriptor schema version is wrong",
            );
            let descriptor_id_is_unique = self.descriptor_ids.insert(&descriptor.descriptor_id);
            self.expect(
                descriptor_id_is_unique,
                "provider_alpha.descriptor_duplicate",
                "provider descriptor ids must be unique",
            );
            self.coverage
                .provider_families
                .insert(descriptor.provider_family);
            self.expect(
                !descriptor.connected_provider_record_ref.trim().is_empty(),
                "provider_alpha.connected_provider_ref_missing",
                "descriptor must cite a connected provider record",
            );
            self.expect(
                !descriptor.source.canonical_host_ref.trim().is_empty(),
                "provider_alpha.source_host_missing",
                "descriptor source must cite a canonical host ref",
            );
            self.expect(
                descriptor.actor_scope.primary_actor_class != ProviderActorClass::UnknownActorClass,
                "provider_alpha.actor_unknown",
                "claimed provider descriptors cannot act under unknown actor scope",
            );
            self.expect(
                !descriptor.supported_object_kinds.is_empty(),
                "provider_alpha.supported_objects_missing",
                "descriptor must name supported object kinds",
            );
            self.expect(
                !descriptor.supported_surface_states.is_empty(),
                "provider_alpha.supported_states_missing",
                "descriptor must name supported surface states",
            );
            self.expect(
                !descriptor.fallback_modes.is_empty(),
                "provider_alpha.fallback_modes_missing",
                "descriptor must name fallback modes",
            );
            self.validate_freshness(&descriptor.freshness, "descriptor");
        }
    }

    fn validate_surface_claims(&mut self) {
        let claims = &self.packet.registry.surface_claims;
        self.expect(
            !claims.is_empty(),
            "provider_alpha.surface_claims_missing",
            "at least one claimed provider surface is required",
        );

        for claim in claims {
            self.expect(
                claim.record_kind == PROVIDER_SURFACE_CLAIM_RECORD_KIND,
                "provider_alpha.surface_record_kind",
                "provider surface claim record_kind is wrong",
            );
            self.expect(
                claim.connected_provider_registry_schema_version
                    == CONNECTED_PROVIDER_REGISTRY_SCHEMA_VERSION,
                "provider_alpha.surface_schema_version",
                "provider surface claim schema version is wrong",
            );
            let surface_id_is_unique = self.surface_ids.insert(&claim.surface_id);
            self.expect(
                surface_id_is_unique,
                "provider_alpha.surface_duplicate",
                "provider surface ids must be unique",
            );
            self.expect(
                self.descriptor_ids
                    .contains(claim.provider_descriptor_ref.as_str()),
                "provider_alpha.surface_descriptor_missing",
                "surface claims must cite a known descriptor",
            );
            self.expect(
                !claim.actions.is_empty(),
                "provider_alpha.surface_actions_missing",
                "surface claim must contain at least one action",
            );
            let mut states = BTreeSet::new();
            for action in &claim.actions {
                self.coverage
                    .mutation_surface_states
                    .insert(action.mutation_state);
                self.expect(
                    states.insert(action.mutation_state),
                    "provider_alpha.surface_state_collapsed",
                    "surface action states must not collapse into duplicate controls",
                );
                self.validate_surface_action(action);
            }
        }
    }

    fn validate_surface_action(&mut self, action: &SurfaceActionDescriptor) {
        match action.mutation_state {
            MutationSurfaceState::LocalDraft => self.expect(
                action.local_draft_ref.as_deref().is_some_and(non_empty),
                "provider_alpha.local_draft_ref_missing",
                "local_draft actions must cite a local draft ref",
            ),
            MutationSurfaceState::PublishNow => self.expect(
                action.approval_ticket_ref.as_deref().is_some_and(non_empty),
                "provider_alpha.publish_now_ticket_missing",
                "publish_now actions must cite an approval ticket ref",
            ),
            MutationSurfaceState::OpenInProvider => self.expect(
                action
                    .browser_handoff_packet_ref
                    .as_deref()
                    .is_some_and(non_empty),
                "provider_alpha.open_in_provider_packet_missing",
                "open_in_provider actions must cite a browser handoff packet ref",
            ),
            MutationSurfaceState::PublishLaterQueue => self.expect(
                action
                    .publish_later_queue_item_ref
                    .as_deref()
                    .is_some_and(non_empty),
                "provider_alpha.publish_later_ref_missing",
                "publish_later_queue actions must cite a queue item ref",
            ),
            MutationSurfaceState::InspectOnly => self.expect(
                action
                    .imported_snapshot_ref
                    .as_deref()
                    .is_some_and(non_empty),
                "provider_alpha.inspect_snapshot_missing",
                "inspect_only actions must cite an imported snapshot ref",
            ),
        }
    }

    fn validate_pipeline_overlays(&mut self) {
        let overlays = &self.packet.registry.pipeline_overlays;
        self.expect(
            !overlays.is_empty(),
            "provider_alpha.pipeline_overlays_missing",
            "pipeline overlays are required for CI descriptors",
        );

        for overlay in overlays {
            self.expect(
                overlay.record_kind == PIPELINE_OVERLAY_DESCRIPTOR_RECORD_KIND,
                "provider_alpha.pipeline_overlay_record_kind",
                "pipeline overlay record_kind is wrong",
            );
            self.expect(
                overlay.connected_provider_registry_schema_version
                    == CONNECTED_PROVIDER_REGISTRY_SCHEMA_VERSION,
                "provider_alpha.pipeline_overlay_schema_version",
                "pipeline overlay schema version is wrong",
            );
            let overlay_id_is_unique = self.overlay_ids.insert(&overlay.overlay_id);
            self.expect(
                overlay_id_is_unique,
                "provider_alpha.pipeline_overlay_duplicate",
                "pipeline overlay ids must be unique",
            );
            self.expect(
                self.descriptor_ids
                    .contains(overlay.provider_descriptor_ref.as_str()),
                "provider_alpha.pipeline_overlay_descriptor_missing",
                "pipeline overlays must cite a known descriptor",
            );
            self.coverage
                .pipeline_overlay_kinds
                .insert(overlay.overlay_kind);
            self.expect(
                !overlay.schema_ref.trim().is_empty(),
                "provider_alpha.pipeline_overlay_schema_ref_missing",
                "pipeline overlays must cite the upstream CI schema they project",
            );
            self.expect(
                !overlay.target_ref.target_ref.trim().is_empty(),
                "provider_alpha.pipeline_overlay_target_missing",
                "pipeline overlays must disclose the target ref",
            );
            self.validate_target_ref(&overlay.target_ref, "pipeline overlay");
            if overlay.overlay_kind == PipelineOverlayKind::Artifact {
                self.expect(
                    overlay.artifact_trust_class.is_some(),
                    "provider_alpha.artifact_trust_class_missing",
                    "artifact overlays must disclose an artifact trust class",
                );
            }
            self.validate_freshness(&overlay.freshness, "pipeline overlay");
        }

        let distinguishes_local_from_provider = overlays.iter().any(|overlay| {
            matches!(
                overlay.local_truth_authority_class,
                LocalTruthAuthorityClass::LocalTruthIsAuthoritative
                    | LocalTruthAuthorityClass::LocalAndProviderDisagreeReviewRequired
            ) && matches!(
                overlay.truth_source_class,
                ProviderTruthSourceClass::ImportedProviderTruth
                    | ProviderTruthSourceClass::LiveProviderTruth
                    | ProviderTruthSourceClass::CachedProviderTruth
            )
        });
        self.expect(
            distinguishes_local_from_provider,
            "provider_alpha.pipeline_local_provider_truth_not_distinguished",
            "at least one pipeline overlay must distinguish local task truth from imported provider truth",
        );
    }

    fn validate_run_controls(&mut self) {
        let controls = &self.packet.registry.run_controls;
        self.expect(
            !controls.is_empty(),
            "provider_alpha.run_controls_missing",
            "run-control descriptors are required for CI overlays",
        );

        for control in controls {
            self.expect(
                control.record_kind == RUN_CONTROL_DESCRIPTOR_RECORD_KIND,
                "provider_alpha.run_control_record_kind",
                "run-control descriptor record_kind is wrong",
            );
            self.expect(
                control.connected_provider_registry_schema_version
                    == CONNECTED_PROVIDER_REGISTRY_SCHEMA_VERSION,
                "provider_alpha.run_control_schema_version",
                "run-control descriptor schema version is wrong",
            );
            self.expect(
                self.descriptor_ids
                    .contains(control.provider_descriptor_ref.as_str()),
                "provider_alpha.run_control_descriptor_missing",
                "run controls must cite a known descriptor",
            );
            self.expect(
                self.overlay_ids
                    .contains(control.pipeline_overlay_ref.as_str()),
                "provider_alpha.run_control_overlay_missing",
                "run controls must cite a known pipeline overlay",
            );
            self.coverage
                .run_control_classes
                .insert(control.control_class);
            self.expect(
                !control.target_ref.target_ref.trim().is_empty(),
                "provider_alpha.run_control_target_missing",
                "run controls must disclose target ref",
            );
            self.validate_target_ref(&control.target_ref, "run control");
            self.expect(
                !control.audit_event_refs.is_empty(),
                "provider_alpha.run_control_audit_missing",
                "run controls must disclose audit event refs",
            );
            self.expect(
                !control.disclosure_summary.trim().is_empty(),
                "provider_alpha.run_control_disclosure_missing",
                "run controls must disclose upstream mutation scope, auth source, and stale-target risk",
            );
            self.validate_freshness(&control.freshness, "run control");
            match control.mutation_mode {
                RunControlMutationMode::PublishNow => self.expect(
                    control
                        .approval_ticket_ref
                        .as_deref()
                        .is_some_and(non_empty),
                    "provider_alpha.run_control_publish_now_ticket_missing",
                    "publish_now run controls must cite an approval ticket",
                ),
                RunControlMutationMode::OpenInProvider => self.expect(
                    control
                        .browser_handoff_packet_ref
                        .as_deref()
                        .is_some_and(non_empty),
                    "provider_alpha.run_control_browser_handoff_missing",
                    "open_in_provider run controls must cite a browser handoff packet",
                ),
                RunControlMutationMode::DeferredPublish => self.expect(
                    control
                        .publish_later_queue_item_ref
                        .as_deref()
                        .is_some_and(non_empty),
                    "provider_alpha.run_control_queue_ref_missing",
                    "deferred_publish run controls must cite a publish-later queue item",
                ),
                RunControlMutationMode::InspectOnly => {}
            }
        }
    }

    fn validate_publish_later_queue(&mut self) {
        let queue = &self.packet.publish_later_queue;
        self.expect(
            !queue.is_empty(),
            "provider_alpha.queue_missing",
            "publish-later queue rows are required",
        );

        for item in queue {
            self.expect(
                item.record_kind
                    == crate::publish_later::PUBLISH_LATER_QUEUE_ALPHA_ITEM_RECORD_KIND,
                "provider_alpha.queue_record_kind",
                "publish-later queue record_kind is wrong",
            );
            self.expect(
                item.schema_version
                    == crate::publish_later::PUBLISH_LATER_QUEUE_ALPHA_SCHEMA_VERSION,
                "provider_alpha.queue_schema_version",
                "publish-later queue schema version is wrong",
            );
            let queue_id_is_unique = self.queue_ids.insert(&item.queue_item_id);
            self.expect(
                queue_id_is_unique,
                "provider_alpha.queue_duplicate",
                "publish-later queue item ids must be unique",
            );
            self.coverage.queue_states.insert(item.queue_state);
            self.expect(
                self.descriptor_ids
                    .contains(item.provider_descriptor_ref.as_str()),
                "provider_alpha.queue_descriptor_missing",
                "publish-later queue items must cite a known descriptor",
            );
            self.expect(
                !item.linked_publish_later_queue_item_ref.trim().is_empty(),
                "provider_alpha.queue_canonical_ref_missing",
                "publish-later queue items must cite the canonical publish_later_queue_item_record",
            );
            self.expect(
                item.dependency_order_is_strict(),
                "provider_alpha.queue_dependency_order_invalid",
                "publish-later queue dependency order must be zero-based and contiguous",
            );
            self.expect(
                item.is_export_safe(),
                "provider_alpha.queue_export_not_safe",
                "publish-later queue items must remain export-safe",
            );
            self.validate_target_ref(&item.target_ref, "queue item");
            if item.queue_state == QueueState::PendingReauth {
                self.expect(
                    item.reauth_requirement
                        != crate::publish_later::ReauthRequirementClass::NotRequired,
                    "provider_alpha.queue_reauth_requirement_missing",
                    "pending_reauth queue items must carry a reauth requirement",
                );
            }
            if item.queue_state == QueueState::PendingRescope {
                self.expect(
                    item.rescope_requirement
                        != crate::publish_later::RescopeRequirementClass::NotRequired,
                    "provider_alpha.queue_rescope_requirement_missing",
                    "pending_rescope queue items must carry a rescope requirement",
                );
            }
            if item.stale_target_risk_class != StaleTargetRiskClass::TargetUnchanged {
                self.expect(
                    item.next_safe_action
                        != crate::publish_later::QueueNextSafeActionClass::DrainNow,
                    "provider_alpha.queue_stale_target_drain_now",
                    "queue items with stale-target risk cannot drain immediately",
                );
            }
            self.validate_freshness(&item.freshness, "queue item");
        }

        for claim in &self.packet.registry.surface_claims {
            for action in &claim.actions {
                if let Some(queue_ref) = &action.publish_later_queue_item_ref {
                    self.expect(
                        self.queue_ids.contains(queue_ref.as_str()),
                        "provider_alpha.surface_queue_ref_unknown",
                        "surface publish-later actions must cite an existing queue row",
                    );
                }
            }
        }

        for control in &self.packet.registry.run_controls {
            if let Some(queue_ref) = &control.publish_later_queue_item_ref {
                self.expect(
                    self.queue_ids.contains(queue_ref.as_str()),
                    "provider_alpha.run_control_queue_ref_unknown",
                    "run controls must cite an existing queue row",
                );
            }
        }
    }

    fn validate_required_coverage(&mut self) {
        self.expect(
            self.coverage
                .provider_families
                .contains(&ProviderFamily::CodeHost)
                && self
                    .coverage
                    .provider_families
                    .contains(&ProviderFamily::IssueTracker)
                && self
                    .coverage
                    .provider_families
                    .contains(&ProviderFamily::CiChecks),
            "provider_alpha.provider_family_coverage_missing",
            "registry must cover code-host, issue, and CI provider families",
        );

        for state in [
            MutationSurfaceState::LocalDraft,
            MutationSurfaceState::PublishNow,
            MutationSurfaceState::OpenInProvider,
            MutationSurfaceState::PublishLaterQueue,
        ] {
            self.expect(
                self.coverage.mutation_surface_states.contains(&state),
                "provider_alpha.surface_state_coverage_missing",
                "registry surfaces must distinguish local draft, publish now, open in provider, and publish-later queue states",
            );
        }

        for overlay_kind in [
            PipelineOverlayKind::Run,
            PipelineOverlayKind::Log,
            PipelineOverlayKind::Artifact,
            PipelineOverlayKind::Annotation,
        ] {
            self.expect(
                self.coverage.pipeline_overlay_kinds.contains(&overlay_kind),
                "provider_alpha.pipeline_overlay_coverage_missing",
                "registry must cover run, log, artifact, and annotation overlays",
            );
        }

        for control_class in [
            RunControlClass::Rerun,
            RunControlClass::Cancel,
            RunControlClass::Retry,
        ] {
            self.expect(
                self.coverage.run_control_classes.contains(&control_class),
                "provider_alpha.run_control_coverage_missing",
                "registry must cover the auditable rerun, cancel, and retry control subset",
            );
        }
    }

    fn validate_freshness(&mut self, freshness: &FreshnessTruth, owner: &str) {
        self.expect(
            !freshness.freshness_floor_ref.trim().is_empty(),
            "provider_alpha.freshness_floor_missing",
            &format!("{owner} freshness must cite a freshness floor"),
        );
        if matches!(
            freshness.freshness_class,
            FreshnessLabel::StaleWithinWindow
                | FreshnessLabel::ExpiredBeyondWindow
                | FreshnessLabel::RevokedOrDisconnected
        ) {
            self.expect(
                freshness.degraded_reason.as_deref().is_some_and(non_empty),
                "provider_alpha.freshness_degraded_reason_missing",
                &format!("{owner} degraded freshness must name the reason"),
            );
        }
    }

    fn validate_target_ref(&mut self, target_ref: &TargetRef, owner: &str) {
        self.expect(
            !target_ref.target_ref_class.trim().is_empty()
                && !target_ref.target_ref.trim().is_empty()
                && !target_ref.target_label.trim().is_empty(),
            "provider_alpha.target_ref_missing",
            &format!("{owner} target ref must carry class, id, and label"),
        );
        let Some(route_origin) = &target_ref.route_origin else {
            return;
        };
        self.expect(
            !route_origin.route_choice.trim().is_empty()
                && !route_origin.target_class.trim().is_empty()
                && !route_origin.route_label.trim().is_empty()
                && !route_origin.transport_label.trim().is_empty()
                && !route_origin.target_identity_ref.trim().is_empty(),
            "provider_alpha.route_origin_label_missing",
            &format!(
                "{owner} route-origin label must carry route, target, transport, and identity refs"
            ),
        );
        if route_origin.route_choice == "tunnel_exposed_route" {
            self.expect(
                route_origin.target_class == "tunnel_exposed_target",
                "provider_alpha.tunnel_route_target_class",
                "tunnel route-origin labels must use tunnel_exposed_target",
            );
            self.expect(
                route_origin
                    .tunnel_session_ref
                    .as_deref()
                    .is_some_and(non_empty),
                "provider_alpha.tunnel_session_ref_missing",
                "tunnel route-origin labels must cite a tunnel session ref",
            );
        }
    }

    fn expect(&mut self, passed: bool, check_id: &str, message: &str) {
        if !passed {
            self.findings.push(ProviderAlphaValidationFinding {
                severity: FindingSeverity::Error,
                check_id: check_id.to_string(),
                message: message.to_string(),
            });
        }
    }
}

fn non_empty(value: &str) -> bool {
    !value.trim().is_empty()
}
