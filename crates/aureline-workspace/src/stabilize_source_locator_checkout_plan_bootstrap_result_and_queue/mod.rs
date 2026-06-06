//! Stable source-locator, checkout-plan, bootstrap-result, and queue truth.
//!
//! This module projects the repository-acquisition seed records into one stable
//! project-entry packet shared by desktop, CLI/headless, deep-link, restore,
//! support-export, and policy-guided entry paths. The packet keeps source
//! provenance, checkout shape, credential posture, bootstrap result, and each
//! queued setup item inspectable without serializing raw secrets, raw repository
//! URLs with credentials, or raw local paths.

use serde::{Deserialize, Serialize};

use crate::acquisition::{
    AcquisitionHonestyLabel, AcquisitionPosture, AcquisitionSurface, AcquisitionVerb,
    AuthModeClass, BootstrapItemClass, BootstrapItemState, BootstrapQueueItemRecord,
    CheckoutModeClass, CheckoutPlanRecord, CheckoutTrustStage, CredentialPostureClass,
    DeclaredFreshnessClass, ExpectedCostBand, LocatorClass, ReadOnlyPartialRoot,
    ReadOnlyPartialRootClass, RepositoryAcquisitionBetaError, RepositoryAcquisitionBetaInputs,
    RepositoryAcquisitionBetaProjection, SignerContinuityClass, SourceLocatorRecord,
    SubmodulePolicyClass, TransportClass,
};

/// Schema version for the stable source-locator / checkout-plan bootstrap packet.
pub const SOURCE_LOCATOR_CHECKOUT_BOOTSTRAP_SCHEMA_VERSION: u32 = 1;

/// Record-kind discriminator for the stable packet.
pub const SOURCE_LOCATOR_CHECKOUT_BOOTSTRAP_RECORD_KIND: &str =
    "source_locator_checkout_plan_bootstrap_result_record";

/// Repo-relative schema reference for support exports and proof packets.
pub const SOURCE_LOCATOR_CHECKOUT_BOOTSTRAP_SCHEMA_REF: &str =
    "schemas/workspace/source-locator-checkout-plan-bootstrap-result.schema.json";

/// Inputs used to build the stable project-entry truth packet.
#[derive(Debug, Clone)]
pub struct StableProjectEntryTruthInput<'a> {
    /// The source locator produced by the entry path.
    pub locator: &'a SourceLocatorRecord,
    /// The checkout plan reviewed before acquisition.
    pub plan: &'a CheckoutPlanRecord,
    /// Individually attributable bootstrap queue items for the plan.
    pub bootstrap_items: &'a [BootstrapQueueItemRecord],
    /// The surface consuming the packet.
    pub surface: AcquisitionSurface,
}

/// Errors returned while building the stable project-entry truth packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableProjectEntryTruthError {
    /// The underlying acquisition projection rejected the input records.
    Acquisition(RepositoryAcquisitionBetaError),
}

impl std::fmt::Display for StableProjectEntryTruthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Acquisition(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for StableProjectEntryTruthError {}

impl From<RepositoryAcquisitionBetaError> for StableProjectEntryTruthError {
    fn from(value: RepositoryAcquisitionBetaError) -> Self {
        Self::Acquisition(value)
    }
}

/// Forge, mirror, import, or local class rendered with the source locator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForgeOrMirrorClass {
    /// Local filesystem source.
    Local,
    /// Live forge or generic remote repository source.
    Forge,
    /// Explicit mirror source.
    Mirror,
    /// Explicit proxy source.
    Proxy,
    /// Offline or air-gapped source media.
    OfflineMedia,
    /// Managed live workspace or remote attach target.
    ManagedWorkspace,
    /// Imported archive, bundle, handoff, or portable-state package.
    ImportedArtifact,
    /// Deep-link source that still needs an acquisition/open decision.
    DeepLink,
}

/// Stable source-locator truth quoted by entry, diagnostics, and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableSourceLocator {
    /// Reference to the seed source-locator record.
    pub source_locator_ref: String,
    /// Source locator kind.
    pub locator_kind: LocatorClass,
    /// Canonical URI/local-path/artifact reference after redaction.
    pub canonical_target_ref: String,
    /// Forge, mirror, proxy, import, managed, or local class.
    pub forge_or_mirror_class: ForgeOrMirrorClass,
    /// Transport protocol, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<TransportClass>,
    /// Branch/ref/work-item intent carried by the locator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_ref_intent: Option<String>,
    /// Auth-source class only; raw secrets never appear.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_source_class: Option<AuthModeClass>,
    /// Trust-seed hint carried into checkout review.
    pub trust_seed_hint: SignerContinuityClass,
}

/// Clone depth/filter posture shown before acquisition starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CloneDepthFilterClass {
    /// Full history and content are expected.
    FullHistory,
    /// Shallow history is expected.
    ShallowDepth,
    /// Partial-clone filter or promisor posture is expected.
    PartialFilter,
    /// Sparse checkout/workset posture is expected.
    SparseProfile,
    /// Checkout depth/filter does not apply.
    NotApplicable,
}

/// Stable checkout-plan truth shown before heavy network or disk work begins.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableCheckoutPlan {
    /// Reference to the seed checkout-plan record.
    pub checkout_plan_ref: String,
    /// Opaque destination-path identity or durable destination ref.
    pub destination_ref: String,
    /// Full, partial, sparse, shallow, archive, live, or not-applicable mode.
    pub checkout_mode: CheckoutModeClass,
    /// Clone depth/filter posture.
    pub clone_depth_filter: CloneDepthFilterClass,
    /// Human-readable depth/filter label after redaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth_or_filter_label: Option<String>,
    /// Sparse/workset profile ref when the plan is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sparse_profile_ref: Option<String>,
    /// Submodule posture.
    pub submodule_mode: SubmodulePolicyClass,
    /// LFS posture as a stable string token.
    pub lfs_mode: String,
    /// Estimated cost band reviewed before acquisition.
    pub estimated_cost_band: ExpectedCostBand,
    /// Durable resumability token, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resumability_token: Option<String>,
    /// Policy or mirror constraints that shaped the plan.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_or_mirror_constraints: Vec<String>,
}

/// Host-key or TLS posture for the bootstrap credential descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostKeyOrTlsPosture {
    /// Host key/certificate is verified or continuous with previous acquisition.
    Verified,
    /// Local trust is inherited.
    InheritedLocal,
    /// Review is required before trust admission.
    ReviewRequired,
    /// The posture is not applicable to this source.
    NotApplicable,
    /// The posture is unknown.
    Unknown,
}

/// Offline fallback rule for acquisition and credential handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfflineFallbackRule {
    /// No offline fallback is needed.
    NotNeeded,
    /// Continue with the current signed/offline bundle.
    UseCurrentBundle,
    /// The mirror must be refreshed before the plan can claim live freshness.
    RefreshMirrorRequired,
    /// Fail closed rather than silently use stale or unauthenticated content.
    FailClosed,
}

/// Stable bootstrap credential descriptor with handle-only secret references.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BootstrapCredentialDescriptor {
    /// Credential posture class derived from locator and plan.
    pub delegated_auth_mode: CredentialPostureClass,
    /// Auth source class, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_source_class: Option<AuthModeClass>,
    /// Opaque credential handle. Raw secret material never appears.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_handle_ref: Option<String>,
    /// Opaque approval-ticket handle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ticket_ref: Option<String>,
    /// Proxy or mirror route ref, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_or_mirror_route_ref: Option<String>,
    /// Host-key or TLS posture.
    pub host_key_or_tls_posture: HostKeyOrTlsPosture,
    /// Offline fallback rule.
    pub offline_fallback_rule: OfflineFallbackRule,
    /// True when re-authentication is required.
    pub reauth_required: bool,
    /// Guardrail: this descriptor carries no raw secret payload.
    pub raw_secret_present: bool,
}

/// Outcome class preserved by restore, diagnostics, and support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcquisitionOutcomeClass {
    /// Existing local content was opened.
    Opened,
    /// A repository acquisition/clone path was selected.
    Acquired,
    /// A mirror/proxy/offline source shaped acquisition truth.
    Mirrored,
    /// An archive, handoff, or portable-state package was imported.
    Imported,
    /// A previous live session, snapshot, or recovery checkpoint was resumed.
    Resumed,
    /// Only partial authority/content is currently available.
    PartiallyAcquired,
    /// A deep link still requires source materialization or open review.
    DeepLinkPending,
    /// Policy blocks acquisition or setup.
    PolicyBlocked,
}

/// Completion state for bootstrap result truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BootstrapCompletionState {
    /// Acquisition has not started.
    NotStarted,
    /// Acquisition or setup is currently running.
    InProgress,
    /// Acquisition was interrupted and has explicit recovery branches.
    Interrupted,
    /// Acquisition completed with no remaining partial-authority caveat.
    Completed,
    /// Acquisition aborted or was cancelled.
    Aborted,
    /// Result is usable but still partial, sparse, mirrored, policy-shaped, or queued.
    PartialAuthority,
}

/// Authority class for a resulting root.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultingRootAuthorityClass {
    /// Full local authority is available.
    FullAuthority,
    /// The root is browse-safe/read-only partial content.
    PartialReadOnly,
    /// The root is imported and requires review before trust/setup.
    ImportedReadOnly,
    /// The root is virtual/live-managed.
    VirtualAuthority,
}

/// One resulting root descriptor emitted by bootstrap result truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultingRootDescriptor {
    /// Opaque root reference.
    pub root_ref: String,
    /// Optional root class from the checkout plan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_class: Option<ReadOnlyPartialRootClass>,
    /// Resulting authority class.
    pub authority_class: ResultingRootAuthorityClass,
    /// Whether this root is browse-safe only.
    pub browse_safe_only: bool,
}

/// Stable bootstrap result object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableBootstrapResult {
    /// Outcome lineage; multi-class cases such as mirrored partial clones carry
    /// both `mirrored` and `partially_acquired`.
    pub outcome_lineage: Vec<AcquisitionOutcomeClass>,
    /// Completion state.
    pub completion_state: BootstrapCompletionState,
    /// Resulting roots.
    pub resulting_roots: Vec<ResultingRootDescriptor>,
    /// Honesty labels and warnings preserved for support export.
    pub warnings: Vec<AcquisitionHonestyLabel>,
    /// Evidence refs backing the result.
    pub evidence_refs: Vec<String>,
    /// Next-step queue refs.
    pub next_step_queue_refs: Vec<String>,
    /// Trust stage at result time.
    pub trust_stage: CheckoutTrustStage,
    /// True when the workspace may not claim full trusted authority.
    pub partial_authority: bool,
}

/// Approval or policy state for a stable queue item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueueApprovalState {
    /// Item is complete or already approved.
    ApprovedOrComplete,
    /// Item is waiting for trust admission.
    AwaitingTrustAdmission,
    /// Item is waiting for a policy decision or policy repair.
    AwaitingPolicy,
    /// Item is waiting for user action.
    AwaitingUserAction,
    /// Item is blocked by policy.
    PolicyBlocked,
    /// Item is blocked by network or credential availability.
    BlockedRecoverable,
    /// Item was cancelled.
    Cancelled,
    /// Item is reviewable but not currently blocked.
    Reviewable,
}

/// Individually attributable stable bootstrap queue item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableBootstrapQueueItem {
    /// Queue item ref.
    pub item_ref: String,
    /// Item kind.
    pub item_kind: BootstrapItemClass,
    /// Scope ref; currently the checkout-plan ref the item belongs to.
    pub scope_ref: String,
    /// Approval or policy state.
    pub approval_state: QueueApprovalState,
    /// Whether the item can be resumed after interruption.
    pub resumable: bool,
    /// Evidence ref or evidence summary backing attribution.
    pub evidence_ref: String,
    /// Optional expiry ref for policy/approval windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_ref: Option<String>,
    /// Whether the item can be cancelled.
    pub cancelable: bool,
    /// Whether the item should be reviewable as a separate row.
    pub reviewable: bool,
}

/// Stable project-entry truth record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableProjectEntryTruthRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Stable schema version.
    pub schema_version: u32,
    /// Schema ref for support and release proof packets.
    pub schema_ref: String,
    /// Surface that consumed the same packet.
    pub surface: AcquisitionSurface,
    /// Stable source-locator truth.
    pub source_locator: StableSourceLocator,
    /// Stable checkout-plan truth.
    pub checkout_plan: StableCheckoutPlan,
    /// Stable credential descriptor.
    pub credential_descriptor: BootstrapCredentialDescriptor,
    /// Stable bootstrap result.
    pub bootstrap_result: StableBootstrapResult,
    /// Stable, individually attributable bootstrap queue items.
    pub bootstrap_queue: Vec<StableBootstrapQueueItem>,
    /// True when review is required before network or disk-heavy acquisition.
    pub review_required_before_acquisition: bool,
    /// True when the packet is safe for default support export.
    pub default_support_export_safe: bool,
}

impl StableProjectEntryTruthRecord {
    /// Builds a stable packet from the existing acquisition records.
    ///
    /// # Errors
    ///
    /// Returns an error when the seed locator, checkout plan, and queue items do
    /// not agree on their refs or when any queue item lacks attributable evidence.
    pub fn project(
        input: StableProjectEntryTruthInput<'_>,
    ) -> Result<Self, StableProjectEntryTruthError> {
        let projection =
            RepositoryAcquisitionBetaProjection::project(RepositoryAcquisitionBetaInputs {
                locator: input.locator,
                plan: input.plan,
                bootstrap_items: input.bootstrap_items,
                surface: input.surface,
            })?;

        Ok(Self {
            record_kind: SOURCE_LOCATOR_CHECKOUT_BOOTSTRAP_RECORD_KIND.to_string(),
            schema_version: SOURCE_LOCATOR_CHECKOUT_BOOTSTRAP_SCHEMA_VERSION,
            schema_ref: SOURCE_LOCATOR_CHECKOUT_BOOTSTRAP_SCHEMA_REF.to_string(),
            surface: input.surface,
            source_locator: stable_source_locator(input.locator),
            checkout_plan: stable_checkout_plan(input.locator, input.plan, &projection),
            credential_descriptor: stable_credential_descriptor(input.locator, &projection),
            bootstrap_result: stable_bootstrap_result(
                input.plan,
                input.bootstrap_items,
                &projection,
            ),
            bootstrap_queue: input
                .bootstrap_items
                .iter()
                .map(stable_queue_item)
                .collect(),
            review_required_before_acquisition: review_required_before_acquisition(&projection),
            default_support_export_safe: true,
        })
    }

    /// Returns contract findings that would make the packet dishonest.
    pub fn contract_findings(&self) -> Vec<&'static str> {
        let mut findings = Vec::new();
        if self.record_kind != SOURCE_LOCATOR_CHECKOUT_BOOTSTRAP_RECORD_KIND {
            findings.push("record_kind");
        }
        if self.schema_version != SOURCE_LOCATOR_CHECKOUT_BOOTSTRAP_SCHEMA_VERSION {
            findings.push("schema_version");
        }
        if self.credential_descriptor.raw_secret_present {
            findings.push("raw_secret_present");
        }
        if self.bootstrap_result.partial_authority
            && self.bootstrap_result.completion_state == BootstrapCompletionState::Completed
        {
            findings.push("partial_authority_completed");
        }
        if self
            .bootstrap_queue
            .iter()
            .any(|item| item.evidence_ref.is_empty())
        {
            findings.push("queue_item_missing_evidence");
        }
        findings
    }

    /// True when the packet satisfies the stable honesty contract.
    pub fn is_contract_valid(&self) -> bool {
        self.contract_findings().is_empty()
    }
}

/// Convenience wrapper for [`StableProjectEntryTruthRecord::project`].
pub fn stabilize_source_locator_checkout_plan_bootstrap_result_and_queue(
    input: StableProjectEntryTruthInput<'_>,
) -> Result<StableProjectEntryTruthRecord, StableProjectEntryTruthError> {
    StableProjectEntryTruthRecord::project(input)
}

fn stable_source_locator(locator: &SourceLocatorRecord) -> StableSourceLocator {
    let protocol = locator
        .host_endpoint
        .as_ref()
        .map(|host| host.transport_class);
    let auth_source_class = locator.host_endpoint.as_ref().map(|host| host.auth_mode);
    let target_ref_intent = locator
        .host_endpoint
        .as_ref()
        .and_then(|host| host.branch_or_ref.clone())
        .or_else(|| {
            locator
                .deep_link_descriptor
                .as_ref()
                .and_then(|link| link.referenced_object_id.clone())
        });

    StableSourceLocator {
        source_locator_ref: locator.source_locator_id.clone(),
        locator_kind: locator.locator_class,
        canonical_target_ref: canonical_target_ref(locator),
        forge_or_mirror_class: forge_or_mirror_class(locator),
        protocol,
        target_ref_intent,
        auth_source_class,
        trust_seed_hint: locator.signer_continuity_class,
    }
}

fn stable_checkout_plan(
    locator: &SourceLocatorRecord,
    plan: &CheckoutPlanRecord,
    projection: &RepositoryAcquisitionBetaProjection,
) -> StableCheckoutPlan {
    let clone_depth_filter = match projection.checkout_shape.mode {
        CheckoutModeClass::PartialClone => CloneDepthFilterClass::PartialFilter,
        CheckoutModeClass::SparseCheckout => CloneDepthFilterClass::SparseProfile,
        CheckoutModeClass::ShallowHistory => CloneDepthFilterClass::ShallowDepth,
        CheckoutModeClass::NotApplicable
        | CheckoutModeClass::ArchiveExtract
        | CheckoutModeClass::LiveAttach => CloneDepthFilterClass::NotApplicable,
        CheckoutModeClass::FullCheckout => CloneDepthFilterClass::FullHistory,
    };

    StableCheckoutPlan {
        checkout_plan_ref: plan.checkout_plan_id.clone(),
        destination_ref: destination_ref(locator, plan),
        checkout_mode: projection.checkout_shape.mode,
        clone_depth_filter,
        depth_or_filter_label: plan
            .topology_markers
            .iter()
            .find_map(|marker| marker.detail_label.clone()),
        sparse_profile_ref: plan
            .read_only_partial_roots
            .iter()
            .find(|root| root.root_class == ReadOnlyPartialRootClass::SparseCheckoutRoot)
            .map(|root| root.root_label.clone()),
        submodule_mode: projection.checkout_shape.submodule_policy,
        lfs_mode: projection.checkout_shape.lfs_policy.as_str().to_string(),
        estimated_cost_band: projection.expected_cost_band,
        resumability_token: plan
            .resumable_acquisition
            .resume_checkpoint_ref
            .clone()
            .or_else(|| Some(plan.resumable_acquisition.resume_state.as_str().to_string())),
        policy_or_mirror_constraints: policy_or_mirror_constraints(plan),
    }
}

fn stable_credential_descriptor(
    locator: &SourceLocatorRecord,
    projection: &RepositoryAcquisitionBetaProjection,
) -> BootstrapCredentialDescriptor {
    let proxy_or_mirror_route_ref = locator.host_endpoint.as_ref().and_then(|host| {
        host.mirror_label
            .clone()
            .or_else(|| host.upstream_origin_label.clone())
    });

    BootstrapCredentialDescriptor {
        delegated_auth_mode: projection.credential_posture.posture_class,
        auth_source_class: projection.credential_posture.auth_mode,
        credential_handle_ref: projection.credential_posture.credential_handle_ref.clone(),
        approval_ticket_ref: projection.credential_posture.approval_ticket_ref.clone(),
        proxy_or_mirror_route_ref,
        host_key_or_tls_posture: host_key_or_tls_posture(locator),
        offline_fallback_rule: offline_fallback_rule(locator),
        reauth_required: projection.credential_posture.reauth_required,
        raw_secret_present: false,
    }
}

fn stable_bootstrap_result(
    plan: &CheckoutPlanRecord,
    bootstrap_items: &[BootstrapQueueItemRecord],
    projection: &RepositoryAcquisitionBetaProjection,
) -> StableBootstrapResult {
    let partial_authority = projection.checkout_shape.partial_or_sparse
        || projection.interrupted_recovery.is_some()
        || !projection.manual_followups.is_empty()
        || projection.trust_stage != CheckoutTrustStage::AdmittedTrusted;

    StableBootstrapResult {
        outcome_lineage: outcome_lineage(projection),
        completion_state: completion_state(plan, projection, partial_authority),
        resulting_roots: resulting_roots(plan, projection),
        warnings: projection.honesty_labels.clone(),
        evidence_refs: evidence_refs(plan, bootstrap_items),
        next_step_queue_refs: projection.bootstrap_queue_refs.clone(),
        trust_stage: projection.trust_stage,
        partial_authority,
    }
}

fn stable_queue_item(item: &BootstrapQueueItemRecord) -> StableBootstrapQueueItem {
    let evidence_ref = item
        .attributable_evidence
        .first()
        .and_then(|evidence| {
            evidence
                .evidence_ref
                .clone()
                .or(evidence.summary_label.clone())
        })
        .unwrap_or_else(|| format!("evidence:{}", item.bootstrap_item_id));

    StableBootstrapQueueItem {
        item_ref: item.bootstrap_item_id.clone(),
        item_kind: item.item_class,
        scope_ref: item.checkout_plan_ref.clone(),
        approval_state: queue_approval_state(item),
        resumable: item.state.is_remaining() && !item.repair_hooks.is_empty(),
        evidence_ref,
        expiry_ref: None,
        cancelable: item.state.is_remaining(),
        reviewable: item.state.is_remaining() || item.state.requires_blocker_and_repair(),
    }
}

fn canonical_target_ref(locator: &SourceLocatorRecord) -> String {
    if let Some(local) = locator.local_identity_ref.as_ref() {
        return local.clone();
    }
    if let Some(host) = locator.host_endpoint.as_ref() {
        return format!(
            "source:{}:{}",
            host.transport_class.as_str(),
            host.host_label
        );
    }
    if let Some(artifact) = locator.artifact_descriptor.as_ref() {
        return format!("artifact:{:?}", artifact.artifact_class).to_ascii_lowercase();
    }
    if let Some(live) = locator.live_session_descriptor.as_ref() {
        if let Some(handle) = live.session_handle_ref.as_ref() {
            return handle.clone();
        }
    }
    if let Some(link) = locator.deep_link_descriptor.as_ref() {
        return link
            .referenced_object_id
            .clone()
            .unwrap_or_else(|| format!("deep-link:{}", link.origin));
    }
    locator.source_locator_id.clone()
}

fn destination_ref(locator: &SourceLocatorRecord, plan: &CheckoutPlanRecord) -> String {
    plan.read_only_partial_roots
        .iter()
        .find_map(|root| root.local_identity_ref.clone())
        .or_else(|| locator.local_identity_ref.clone())
        .or_else(|| plan.entry_action_ref.clone())
        .unwrap_or_else(|| format!("destination-ref:{}", plan.checkout_plan_id))
}

fn forge_or_mirror_class(locator: &SourceLocatorRecord) -> ForgeOrMirrorClass {
    if locator.locator_class.is_local_only() {
        return ForgeOrMirrorClass::Local;
    }
    match locator.locator_class {
        LocatorClass::MirrorOrProxyRepo => ForgeOrMirrorClass::Mirror,
        LocatorClass::LiveResumeTarget => ForgeOrMirrorClass::ManagedWorkspace,
        LocatorClass::SnapshotArchive
        | LocatorClass::Template
        | LocatorClass::PrebuildSnapshot
        | LocatorClass::HandoffPacket
        | LocatorClass::PortableStatePackage => ForgeOrMirrorClass::ImportedArtifact,
        LocatorClass::ReviewOrWorkItemDeepLink => ForgeOrMirrorClass::DeepLink,
        LocatorClass::RepoUrl => match locator.host_endpoint.as_ref().map(|h| h.transport_class) {
            Some(TransportClass::Mirror) => ForgeOrMirrorClass::Mirror,
            Some(TransportClass::Proxy) => ForgeOrMirrorClass::Proxy,
            Some(TransportClass::AirGappedMedia) => ForgeOrMirrorClass::OfflineMedia,
            _ => ForgeOrMirrorClass::Forge,
        },
        _ => ForgeOrMirrorClass::Forge,
    }
}

fn policy_or_mirror_constraints(plan: &CheckoutPlanRecord) -> Vec<String> {
    let mut constraints = Vec::new();
    if let Some(mirror) = plan.mirror_freshness.as_ref() {
        constraints.push(format!(
            "mirror:{}",
            mirror_freshness_token(mirror.freshness_class)
        ));
        if let Some(delta) = mirror.upstream_delta_class {
            constraints.push(format!("upstream_delta:{}", delta.as_str()));
        }
    }
    for policy in &plan.policy_narrowing_refs {
        if let Some(policy_ref) = policy.policy_bundle_ref.as_ref() {
            constraints.push(policy_ref.clone());
        } else {
            constraints.push(format!("policy:{}", policy.policy_source_class.as_str()));
        }
    }
    constraints
}

fn host_key_or_tls_posture(locator: &SourceLocatorRecord) -> HostKeyOrTlsPosture {
    match locator.signer_continuity_class {
        SignerContinuityClass::ContinuousWithPreviousAcquisition
        | SignerContinuityClass::SignerRotationPreauthorized => HostKeyOrTlsPosture::Verified,
        SignerContinuityClass::NotApplicable => HostKeyOrTlsPosture::NotApplicable,
        SignerContinuityClass::SignerChangedReviewRequired
        | SignerContinuityClass::SignatureMismatch => HostKeyOrTlsPosture::ReviewRequired,
        SignerContinuityClass::Unsigned | SignerContinuityClass::SignatureMissing => {
            HostKeyOrTlsPosture::Unknown
        }
        SignerContinuityClass::NewSignerFirstSeen
        | SignerContinuityClass::SignerChangedTrustOnFirstUse => {
            HostKeyOrTlsPosture::InheritedLocal
        }
    }
}

fn mirror_freshness_token(freshness: crate::acquisition::MirrorFreshnessClass) -> &'static str {
    match freshness {
        crate::acquisition::MirrorFreshnessClass::LiveOrigin => "live_origin",
        crate::acquisition::MirrorFreshnessClass::MirrorFresh => "mirror_fresh",
        crate::acquisition::MirrorFreshnessClass::MirrorLagged => "mirror_lagged",
        crate::acquisition::MirrorFreshnessClass::MirrorStale => "mirror_stale",
        crate::acquisition::MirrorFreshnessClass::OfflineSnapshot => "offline_snapshot",
        crate::acquisition::MirrorFreshnessClass::SignedOfflineBundle => "signed_offline_bundle",
        crate::acquisition::MirrorFreshnessClass::UnknownFreshness => "unknown_freshness",
    }
}

fn offline_fallback_rule(locator: &SourceLocatorRecord) -> OfflineFallbackRule {
    match locator.declared_freshness_class {
        DeclaredFreshnessClass::OfflineSnapshot | DeclaredFreshnessClass::SignedOfflineBundle => {
            OfflineFallbackRule::UseCurrentBundle
        }
        DeclaredFreshnessClass::MirrorLagged | DeclaredFreshnessClass::MirrorStale => {
            OfflineFallbackRule::RefreshMirrorRequired
        }
        DeclaredFreshnessClass::LiveOrigin | DeclaredFreshnessClass::MirrorFresh => {
            if locator.locator_class.is_local_only() {
                OfflineFallbackRule::NotNeeded
            } else {
                OfflineFallbackRule::FailClosed
            }
        }
        DeclaredFreshnessClass::UnknownFreshness => OfflineFallbackRule::FailClosed,
    }
}

fn outcome_lineage(
    projection: &RepositoryAcquisitionBetaProjection,
) -> Vec<AcquisitionOutcomeClass> {
    let mut out = Vec::new();
    match projection.acquisition_verb {
        AcquisitionVerb::OpenLocal => push_unique(&mut out, AcquisitionOutcomeClass::Opened),
        AcquisitionVerb::Clone => push_unique(&mut out, AcquisitionOutcomeClass::Acquired),
        AcquisitionVerb::Import
        | AcquisitionVerb::OpenArchive
        | AcquisitionVerb::OpenTemplateOrPrebuild => {
            push_unique(&mut out, AcquisitionOutcomeClass::Imported)
        }
        AcquisitionVerb::Resume => push_unique(&mut out, AcquisitionOutcomeClass::Resumed),
        AcquisitionVerb::OpenDeepLink => {
            push_unique(&mut out, AcquisitionOutcomeClass::DeepLinkPending)
        }
    }
    if matches!(
        projection.declared_freshness_class,
        DeclaredFreshnessClass::MirrorFresh
            | DeclaredFreshnessClass::MirrorLagged
            | DeclaredFreshnessClass::MirrorStale
            | DeclaredFreshnessClass::OfflineSnapshot
            | DeclaredFreshnessClass::SignedOfflineBundle
    ) || matches!(
        projection.transport_class,
        Some(TransportClass::Mirror | TransportClass::Proxy)
    ) {
        push_unique(&mut out, AcquisitionOutcomeClass::Mirrored);
    }
    if projection.checkout_shape.partial_or_sparse
        || projection.interrupted_recovery.is_some()
        || matches!(
            projection.acquisition_posture,
            AcquisitionPosture::PartiallyAcquired | AcquisitionPosture::FilteredOrSparse
        )
    {
        push_unique(&mut out, AcquisitionOutcomeClass::PartiallyAcquired);
    }
    if matches!(
        projection.acquisition_posture,
        AcquisitionPosture::PolicyBlocked
    ) {
        push_unique(&mut out, AcquisitionOutcomeClass::PolicyBlocked);
    }
    out
}

fn completion_state(
    plan: &CheckoutPlanRecord,
    projection: &RepositoryAcquisitionBetaProjection,
    partial_authority: bool,
) -> BootstrapCompletionState {
    if projection.interrupted_recovery.is_some() {
        return BootstrapCompletionState::Interrupted;
    }
    match plan.resumable_acquisition.resume_state {
        crate::acquisition::AcquisitionResumeState::NeverStarted => {
            BootstrapCompletionState::NotStarted
        }
        crate::acquisition::AcquisitionResumeState::InProgress => {
            BootstrapCompletionState::InProgress
        }
        crate::acquisition::AcquisitionResumeState::Completed => {
            if partial_authority {
                BootstrapCompletionState::PartialAuthority
            } else {
                BootstrapCompletionState::Completed
            }
        }
        crate::acquisition::AcquisitionResumeState::Aborted => BootstrapCompletionState::Aborted,
        _ => BootstrapCompletionState::Interrupted,
    }
}

fn resulting_roots(
    plan: &CheckoutPlanRecord,
    projection: &RepositoryAcquisitionBetaProjection,
) -> Vec<ResultingRootDescriptor> {
    if !plan.read_only_partial_roots.is_empty() {
        return plan
            .read_only_partial_roots
            .iter()
            .map(resulting_partial_root)
            .collect();
    }

    let authority_class = match projection.checkout_shape.mode {
        CheckoutModeClass::LiveAttach => ResultingRootAuthorityClass::VirtualAuthority,
        CheckoutModeClass::ArchiveExtract => ResultingRootAuthorityClass::ImportedReadOnly,
        _ if projection.checkout_shape.partial_or_sparse => {
            ResultingRootAuthorityClass::PartialReadOnly
        }
        _ => ResultingRootAuthorityClass::FullAuthority,
    };

    vec![ResultingRootDescriptor {
        root_ref: format!("root:{}", projection.checkout_plan_ref),
        root_class: None,
        browse_safe_only: authority_class != ResultingRootAuthorityClass::FullAuthority,
        authority_class,
    }]
}

fn resulting_partial_root(root: &ReadOnlyPartialRoot) -> ResultingRootDescriptor {
    ResultingRootDescriptor {
        root_ref: root
            .local_identity_ref
            .clone()
            .unwrap_or_else(|| root.root_label.clone()),
        root_class: Some(root.root_class),
        authority_class: ResultingRootAuthorityClass::PartialReadOnly,
        browse_safe_only: root.browse_safe_only.unwrap_or(true),
    }
}

fn evidence_refs(
    plan: &CheckoutPlanRecord,
    bootstrap_items: &[BootstrapQueueItemRecord],
) -> Vec<String> {
    let mut refs = Vec::new();
    if let Some(mirror) = plan.mirror_freshness.as_ref() {
        if let Some(evidence_ref) = mirror.mirror_attestation_ref.as_ref() {
            push_unique(&mut refs, evidence_ref.clone());
        }
    }
    if let Some(review_ref) = plan.signer_continuity.review_ticket_ref.as_ref() {
        push_unique(&mut refs, review_ref.clone());
    }
    for item in bootstrap_items {
        for evidence in &item.attributable_evidence {
            if let Some(evidence_ref) = evidence.evidence_ref.as_ref() {
                push_unique(&mut refs, evidence_ref.clone());
            }
        }
    }
    if refs.is_empty() {
        refs.push(plan.checkout_plan_id.clone());
    }
    refs
}

fn queue_approval_state(item: &BootstrapQueueItemRecord) -> QueueApprovalState {
    match item.state {
        BootstrapItemState::Succeeded | BootstrapItemState::Skipped => {
            QueueApprovalState::ApprovedOrComplete
        }
        BootstrapItemState::Cancelled => QueueApprovalState::Cancelled,
        BootstrapItemState::AwaitingAdmission => QueueApprovalState::AwaitingTrustAdmission,
        BootstrapItemState::AwaitingPolicyDecision => QueueApprovalState::AwaitingPolicy,
        BootstrapItemState::AwaitingUserAction => QueueApprovalState::AwaitingUserAction,
        BootstrapItemState::FailedBlocking
            if matches!(
                item.blocker,
                Some(crate::acquisition::BlockerClass::PolicyBlocked)
            ) =>
        {
            QueueApprovalState::PolicyBlocked
        }
        BootstrapItemState::FailedRecoverable
        | BootstrapItemState::FailedBlocking
        | BootstrapItemState::AwaitingNetwork => QueueApprovalState::BlockedRecoverable,
        _ => QueueApprovalState::Reviewable,
    }
}

fn review_required_before_acquisition(projection: &RepositoryAcquisitionBetaProjection) -> bool {
    matches!(
        projection.acquisition_verb,
        AcquisitionVerb::Clone
            | AcquisitionVerb::Import
            | AcquisitionVerb::OpenArchive
            | AcquisitionVerb::OpenTemplateOrPrebuild
            | AcquisitionVerb::Resume
    ) || projection.expected_cost_band != ExpectedCostBand::LocalNoFetch
        || projection.checkout_shape.partial_or_sparse
        || !projection.manual_followups.is_empty()
}

fn push_unique<T: PartialEq>(slot: &mut Vec<T>, value: T) {
    if !slot.contains(&value) {
        slot.push(value);
    }
}
