//! Cross-surface beta projection of the three acquisition records.
//!
//! [`RepositoryAcquisitionBetaProjection`] is the single record Start
//! Center, the command palette, deep-link resolvers, and CLI/headless
//! acquisition paths read so they all agree, before any
//! hydrate/init/fetch path runs, on:
//!
//! - **what source is being acquired** (the acquisition verb, locator
//!   class, transport, and freshness/signer posture quoted from the
//!   [`SourceLocatorRecord`]),
//! - **what checkout shape and cost band will be used** (the
//!   [`CheckoutShape`] and [`ExpectedCostBand`] derived from the plan's
//!   topology markers, read-only partial roots, and bootstrap queue),
//! - **what credential posture applies** (the
//!   [`BootstrapCredentialPosture`] derived from the locator auth mode and
//!   the plan trust stage),
//! - **which follow-up bootstrap actions remain manual** (the
//!   [`ManualFollowup`] list derived from the queue items that have not yet
//!   succeeded), and
//! - **how to recover from interrupted acquisition** (the
//!   [`InterruptedRecovery`] branches derived from the plan's resumable
//!   state) without silently drifting into hidden setup or hidden trust
//!   elevation (the [`AcquisitionGuardrails`]).
//!
//! The projection never invents new vocabulary; it only quotes the three
//! boundary records and adds typed predicates the surface contract
//! requires.

use serde::{Deserialize, Serialize};

use super::descriptors::{
    AbsenceClass, AcquisitionPosture, AcquisitionResumeState, AttributableEvidenceClass,
    AuthModeClass, BootstrapExecutionClass, BootstrapItemClass, BootstrapItemState,
    BootstrapQueueItemRecord, CheckoutPlanRecord, CheckoutTrustStage, DeclaredFreshnessClass,
    DiscardPosture, LocatorClass, NextStepDecisionHook, ReadOnlyPartialRoot, SignerContinuityClass,
    SourceLocatorRecord, TopologyMarkerClass, TransportClass, UpstreamDeltaClass,
};
use super::shared::AcquisitionSurface;

/// Schema version for [`RepositoryAcquisitionBetaProjection`].
pub const REPOSITORY_ACQUISITION_SCHEMA_VERSION: u32 = 1;

/// Record-kind discriminator for the projection.
pub const REPOSITORY_ACQUISITION_RECORD_KIND: &str = "repository_acquisition_record";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryAcquisitionRecordKind {
    RepositoryAcquisitionRecord,
}

/// Distinct acquisition verb the projection resolves. Open folder, Clone
/// repository, Import bundle, Open archive, and Resume snapshot stay
/// distinct verbs with distinct review, trust, and recovery semantics; the
/// projection never collapses one into another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcquisitionVerb {
    /// Open an already-local folder, file, or workspace/workset manifest.
    OpenLocal,
    /// Clone a remote or mirror-served repository.
    Clone,
    /// Import a handoff packet, portable-state package, or competitor config.
    Import,
    /// Open a snapshot archive (read-only extraction).
    OpenArchive,
    /// Materialize a template or prebuild snapshot.
    OpenTemplateOrPrebuild,
    /// Resume a live session or restore a recovery checkpoint.
    Resume,
    /// Resolve a review / work-item / notification deep link.
    OpenDeepLink,
}

impl AcquisitionVerb {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenLocal => "open_local",
            Self::Clone => "clone",
            Self::Import => "import",
            Self::OpenArchive => "open_archive",
            Self::OpenTemplateOrPrebuild => "open_template_or_prebuild",
            Self::Resume => "resume",
            Self::OpenDeepLink => "open_deep_link",
        }
    }

    /// True when the verb materializes bytes from a remote / mirror /
    /// network source rather than reusing an on-disk copy.
    pub const fn is_remote_acquisition(self) -> bool {
        matches!(self, Self::Clone)
    }
}

/// Closed checkout-mode class derived from the plan's topology markers and
/// read-only partial roots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutModeClass {
    /// A full checkout / full clone with complete history and content.
    FullCheckout,
    /// A partial-clone filter or promisor root is in play.
    PartialClone,
    /// A sparse / workset-narrowed checkout.
    SparseCheckout,
    /// Shallow history is present.
    ShallowHistory,
    /// Content materialized by read-only archive / bundle extraction.
    ArchiveExtract,
    /// A live remote / managed attach with no local checkout.
    LiveAttach,
    /// Mode is not applicable (for example, a deep-link resolution).
    NotApplicable,
}

impl CheckoutModeClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullCheckout => "full_checkout",
            Self::PartialClone => "partial_clone",
            Self::SparseCheckout => "sparse_checkout",
            Self::ShallowHistory => "shallow_history",
            Self::ArchiveExtract => "archive_extract",
            Self::LiveAttach => "live_attach",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Submodule policy derived from the plan's topology markers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubmodulePolicyClass {
    NotPresent,
    InitPending,
    InitPartial,
    InitComplete,
    InitFailed,
}

impl SubmodulePolicyClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotPresent => "not_present",
            Self::InitPending => "init_pending",
            Self::InitPartial => "init_partial",
            Self::InitComplete => "init_complete",
            Self::InitFailed => "init_failed",
        }
    }
}

/// Git LFS policy derived from the plan's topology markers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LfsPolicyClass {
    NotPresent,
    PointerOnly,
    HydratePending,
    HydratePartial,
    HydrateComplete,
    HydrateFailed,
}

impl LfsPolicyClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotPresent => "not_present",
            Self::PointerOnly => "pointer_only",
            Self::HydratePending => "hydrate_pending",
            Self::HydratePartial => "hydrate_partial",
            Self::HydrateComplete => "hydrate_complete",
            Self::HydrateFailed => "hydrate_failed",
        }
    }
}

/// Checkout shape the plan discloses before any hydrate / init / fetch path
/// runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckoutShape {
    /// Full / partial / sparse / shallow / archive / live mode.
    pub mode: CheckoutModeClass,
    /// True when the checkout is partial or sparse and cannot claim full
    /// local truth.
    pub partial_or_sparse: bool,
    /// True when the plan exposes at least one read-only partial root.
    pub read_only_partial_present: bool,
    /// Submodule init policy derived from topology markers.
    pub submodule_policy: SubmodulePolicyClass,
    /// LFS hydrate policy derived from topology markers.
    pub lfs_policy: LfsPolicyClass,
    /// Read-only partial-root classes the plan made safe to inspect.
    pub read_only_partial_roots: Vec<ReadOnlyPartialRoot>,
}

/// Coarse, deterministic expected cost band for the acquisition. The seed
/// records carry no byte estimates, so the band is derived from transport,
/// posture, topology, and the bootstrap queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpectedCostBand {
    /// No fetch: the source is already on disk or a labelled local upload.
    LocalNoFetch,
    /// A metered fetch over the network with no large hydrate / partial set.
    MeteredFetch,
    /// A large fetch or LFS / partial-clone hydrate is expected.
    LargeFetchOrHydrate,
    /// A live remote attach (no local checkout cost).
    LiveAttach,
    /// Cost is not known yet.
    Unknown,
}

impl ExpectedCostBand {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalNoFetch => "local_no_fetch",
            Self::MeteredFetch => "metered_fetch",
            Self::LargeFetchOrHydrate => "large_fetch_or_hydrate",
            Self::LiveAttach => "live_attach",
            Self::Unknown => "unknown",
        }
    }
}

/// Closed credential-posture class derived from the locator auth mode and
/// the plan trust stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialPostureClass {
    /// No credentials are needed (anonymous / none / local filesystem).
    NoCredentialsNeeded,
    /// The local user identity is inherited; no separate handle is used.
    LocalIdentityInherited,
    /// A typed credential handle is present and not yet exercised.
    HandlePresentNotYetUsed,
    /// A browser or device-code handoff is pending.
    BrowserOrDeviceHandoffPending,
    /// A managed-session or connected-provider ticket is required.
    ProviderTicketRequired,
    /// Re-authentication is required before acquisition can proceed.
    ReauthRequired,
    /// Posture is not yet known.
    Unknown,
}

impl CredentialPostureClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoCredentialsNeeded => "no_credentials_needed",
            Self::LocalIdentityInherited => "local_identity_inherited",
            Self::HandlePresentNotYetUsed => "handle_present_not_yet_used",
            Self::BrowserOrDeviceHandoffPending => "browser_or_device_handoff_pending",
            Self::ProviderTicketRequired => "provider_ticket_required",
            Self::ReauthRequired => "reauth_required",
            Self::Unknown => "unknown",
        }
    }
}

/// Typed bootstrap credential posture. Names the credential class without
/// ever carrying a raw secret.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BootstrapCredentialPosture {
    pub posture_class: CredentialPostureClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_mode: Option<AuthModeClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_handle_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ticket_ref: Option<String>,
    /// True when the plan or a queue item is blocked pending re-auth.
    pub reauth_required: bool,
}

/// One explicit branch an interrupted-acquisition card may offer. The card
/// never collapses these into a binary retry / cancel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterruptedRecoveryBranch {
    ResumeAcquisition,
    DiscardAndRestart,
    OpenReadOnlyPartial,
    RefreshMirror,
    SwitchToLiveOrigin,
}

impl InterruptedRecoveryBranch {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResumeAcquisition => "resume_acquisition",
            Self::DiscardAndRestart => "discard_and_restart",
            Self::OpenReadOnlyPartial => "open_read_only_partial",
            Self::RefreshMirror => "refresh_mirror",
            Self::SwitchToLiveOrigin => "switch_to_live_origin",
        }
    }
}

/// Typed interrupted-acquisition recovery card. Present only when the plan's
/// resume state is one of the explicit interrupted branches.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterruptedRecovery {
    pub resume_state: AcquisitionResumeState,
    pub discard_posture: DiscardPosture,
    /// True when at least one read-only partial root is inspectable now.
    pub open_read_only_available: bool,
    /// Explicit recovery branches the card offers, derived from the plan's
    /// next-step decision hooks and resume state.
    pub branches: Vec<InterruptedRecoveryBranch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resume_checkpoint_ref: Option<String>,
    /// True when the resulting state is export-safe for support and
    /// enterprise rollout (no raw paths / credentials / bytes).
    pub export_safe: bool,
}

/// One follow-up bootstrap action that remains manual after open. Surfaces
/// render the typed list rather than one opaque "setting up workspace"
/// spinner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManualFollowup {
    pub bootstrap_item_ref: String,
    pub item_class: BootstrapItemClass,
    pub state: BootstrapItemState,
    pub execution_class: BootstrapExecutionClass,
    pub absence_class: AbsenceClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_label: Option<String>,
}

/// Export-safe evidence packet that joins source identity, checkout plan,
/// and bootstrap-queue refs with the union of typed evidence classes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BootstrapEvidencePacket {
    pub source_locator_ref: String,
    pub checkout_plan_ref: String,
    pub bootstrap_item_refs: Vec<String>,
    /// Union of attributable-evidence classes across every queue item.
    pub evidence_classes: Vec<AttributableEvidenceClass>,
    /// True when every queue item carries at least one typed evidence entry.
    pub every_item_attributed: bool,
    /// Count of follow-up items that remain manual after open.
    pub manual_followup_count: u64,
    /// True when the packet carries only opaque refs and typed labels.
    pub export_safe: bool,
}

/// Closed honesty-label set the surface renders verbatim alongside an
/// acquisition row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcquisitionHonestyLabel {
    MirrorLagged,
    MirrorStale,
    UpstreamDeltaOutsideSkew,
    OfflineSnapshot,
    SignedOfflineBundle,
    SignerChangedReviewRequired,
    SignerFirstSeen,
    ReadOnlyPartial,
    ShallowHistory,
    PartialClone,
    SparseWorkset,
    SubmoduleInitPending,
    LfsPointerOnly,
    ReauthRequired,
    ReconnectRequired,
    PolicyNarrowed,
}

impl AcquisitionHonestyLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MirrorLagged => "mirror_lagged",
            Self::MirrorStale => "mirror_stale",
            Self::UpstreamDeltaOutsideSkew => "upstream_delta_outside_skew",
            Self::OfflineSnapshot => "offline_snapshot",
            Self::SignedOfflineBundle => "signed_offline_bundle",
            Self::SignerChangedReviewRequired => "signer_changed_review_required",
            Self::SignerFirstSeen => "signer_first_seen",
            Self::ReadOnlyPartial => "read_only_partial",
            Self::ShallowHistory => "shallow_history",
            Self::PartialClone => "partial_clone",
            Self::SparseWorkset => "sparse_workset",
            Self::SubmoduleInitPending => "submodule_init_pending",
            Self::LfsPointerOnly => "lfs_pointer_only",
            Self::ReauthRequired => "reauth_required",
            Self::ReconnectRequired => "reconnect_required",
            Self::PolicyNarrowed => "policy_narrowed",
        }
    }
}

/// Typed guardrail predicates the projection guarantees. Each maps to a
/// guardrail or acceptance criterion in the acquisition spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcquisitionGuardrails {
    /// The acquisition verb matches the locator: a clone verb only resolves
    /// for a remote / mirror locator, and an open-local verb never resolves
    /// for a remote one. Clone is never confused with open-local-copy.
    pub clone_not_confused_with_open: bool,
    /// No repository-owned code path (hooks, filters, generators, package
    /// scripts) runs implicitly: the plan blocks them and no side-effecting
    /// queue item has already run before trust admission.
    pub no_implicit_repo_code_execution: bool,
    /// Every enqueued bootstrap item carries at least one typed evidence
    /// entry.
    pub bootstrap_items_attributed: bool,
    /// The user can browse-safe inspect before execution / setup approval.
    pub browse_safe_inspection_available: bool,
    /// A mirror / proxy / lagged / stale / offline source never masquerades
    /// as a live upstream fetch.
    pub mirror_not_masquerading_as_live: bool,
    /// Trust is not silently elevated: a signer change routes to review and
    /// reauth / reconnect stages are surfaced rather than auto-admitted.
    pub no_hidden_trust_elevation: bool,
}

impl AcquisitionGuardrails {
    /// True when every guardrail holds.
    pub const fn all_hold(self) -> bool {
        self.clone_not_confused_with_open
            && self.no_implicit_repo_code_execution
            && self.bootstrap_items_attributed
            && self.browse_safe_inspection_available
            && self.mirror_not_masquerading_as_live
            && self.no_hidden_trust_elevation
    }
}

/// Inputs used to assemble a beta projection.
#[derive(Debug, Clone)]
pub struct RepositoryAcquisitionBetaInputs<'a> {
    pub locator: &'a SourceLocatorRecord,
    pub plan: &'a CheckoutPlanRecord,
    pub bootstrap_items: &'a [BootstrapQueueItemRecord],
    pub surface: AcquisitionSurface,
}

/// Errors returned while assembling a beta projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryAcquisitionBetaError {
    /// The plan does not reference the supplied locator.
    PlanLocatorMismatch {
        expected_source_locator_id: String,
        observed: String,
    },
    /// A bootstrap item does not reference the supplied plan.
    BootstrapItemPlanMismatch {
        bootstrap_item_id: String,
        expected_checkout_plan_id: String,
        observed: String,
    },
    /// A bootstrap item does not reference the supplied locator.
    BootstrapItemLocatorMismatch {
        bootstrap_item_id: String,
        expected_source_locator_id: String,
        observed: String,
    },
    /// A bootstrap item carries no attributable evidence.
    BootstrapItemEvidenceMissing { bootstrap_item_id: String },
}

impl std::fmt::Display for RepositoryAcquisitionBetaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlanLocatorMismatch {
                expected_source_locator_id,
                observed,
            } => write!(
                f,
                "checkout plan references locator {observed}, expected {expected_source_locator_id}"
            ),
            Self::BootstrapItemPlanMismatch {
                bootstrap_item_id,
                expected_checkout_plan_id,
                observed,
            } => write!(
                f,
                "bootstrap item {bootstrap_item_id} references plan {observed}, expected {expected_checkout_plan_id}"
            ),
            Self::BootstrapItemLocatorMismatch {
                bootstrap_item_id,
                expected_source_locator_id,
                observed,
            } => write!(
                f,
                "bootstrap item {bootstrap_item_id} references locator {observed}, expected {expected_source_locator_id}"
            ),
            Self::BootstrapItemEvidenceMissing { bootstrap_item_id } => write!(
                f,
                "bootstrap item {bootstrap_item_id} carries no attributable evidence"
            ),
        }
    }
}

impl std::error::Error for RepositoryAcquisitionBetaError {}

/// Beta truth one acquisition surface reads so it agrees with every other
/// surface about source identity, checkout shape, credential posture,
/// remaining manual setup, and interrupted-acquisition recovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryAcquisitionBetaProjection {
    pub record_kind: RepositoryAcquisitionRecordKind,
    pub repository_acquisition_schema_version: u32,
    pub surface: AcquisitionSurface,

    pub source_locator_ref: String,
    pub checkout_plan_ref: String,
    pub bootstrap_queue_refs: Vec<String>,

    pub acquisition_verb: AcquisitionVerb,
    pub locator_class: LocatorClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport_class: Option<TransportClass>,
    pub acquisition_posture: AcquisitionPosture,
    pub declared_freshness_class: DeclaredFreshnessClass,
    pub signer_continuity_class: SignerContinuityClass,
    pub trust_stage: CheckoutTrustStage,

    pub checkout_shape: CheckoutShape,
    pub expected_cost_band: ExpectedCostBand,
    pub credential_posture: BootstrapCredentialPosture,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interrupted_recovery: Option<InterruptedRecovery>,

    pub manual_followups: Vec<ManualFollowup>,
    pub evidence_packet: BootstrapEvidencePacket,

    pub honesty_labels: Vec<AcquisitionHonestyLabel>,
    pub guardrails: AcquisitionGuardrails,
}

impl RepositoryAcquisitionBetaProjection {
    /// Builds a projection from a locator, plan, and bootstrap queue.
    /// Validates that the plan and every item bind the supplied locator and
    /// that every item carries attributable evidence.
    pub fn project(
        inputs: RepositoryAcquisitionBetaInputs<'_>,
    ) -> Result<Self, RepositoryAcquisitionBetaError> {
        let RepositoryAcquisitionBetaInputs {
            locator,
            plan,
            bootstrap_items,
            surface,
        } = inputs;

        let locator_id = locator.source_locator_id.as_str();
        let plan_id = plan.checkout_plan_id.as_str();

        if plan.source_locator_ref != locator_id {
            return Err(RepositoryAcquisitionBetaError::PlanLocatorMismatch {
                expected_source_locator_id: locator_id.to_owned(),
                observed: plan.source_locator_ref.clone(),
            });
        }

        for item in bootstrap_items {
            if item.checkout_plan_ref != plan_id {
                return Err(RepositoryAcquisitionBetaError::BootstrapItemPlanMismatch {
                    bootstrap_item_id: item.bootstrap_item_id.clone(),
                    expected_checkout_plan_id: plan_id.to_owned(),
                    observed: item.checkout_plan_ref.clone(),
                });
            }
            if item.source_locator_ref != locator_id {
                return Err(
                    RepositoryAcquisitionBetaError::BootstrapItemLocatorMismatch {
                        bootstrap_item_id: item.bootstrap_item_id.clone(),
                        expected_source_locator_id: locator_id.to_owned(),
                        observed: item.source_locator_ref.clone(),
                    },
                );
            }
            if item.attributable_evidence.is_empty() {
                return Err(
                    RepositoryAcquisitionBetaError::BootstrapItemEvidenceMissing {
                        bootstrap_item_id: item.bootstrap_item_id.clone(),
                    },
                );
            }
        }

        let acquisition_verb = resolve_acquisition_verb(locator);
        let transport_class = transport_of(locator);
        let checkout_shape = resolve_checkout_shape(locator, plan);
        let expected_cost_band =
            resolve_cost_band(locator, &checkout_shape, bootstrap_items, acquisition_verb);
        let credential_posture = resolve_credential_posture(locator, plan, bootstrap_items);
        let interrupted_recovery = resolve_interrupted_recovery(plan);
        let manual_followups = resolve_manual_followups(bootstrap_items);
        let evidence_packet = resolve_evidence_packet(
            locator,
            plan,
            bootstrap_items,
            manual_followups.len() as u64,
        );
        let honesty_labels = resolve_honesty_labels(locator, plan, &checkout_shape);
        let guardrails = resolve_guardrails(
            locator,
            plan,
            bootstrap_items,
            acquisition_verb,
            &evidence_packet,
        );

        let bootstrap_queue_refs = bootstrap_items
            .iter()
            .map(|item| item.bootstrap_item_id.clone())
            .collect();

        Ok(Self {
            record_kind: RepositoryAcquisitionRecordKind::RepositoryAcquisitionRecord,
            repository_acquisition_schema_version: REPOSITORY_ACQUISITION_SCHEMA_VERSION,
            surface,
            source_locator_ref: locator.source_locator_id.clone(),
            checkout_plan_ref: plan.checkout_plan_id.clone(),
            bootstrap_queue_refs,
            acquisition_verb,
            locator_class: locator.locator_class,
            transport_class,
            acquisition_posture: locator.acquisition_posture,
            declared_freshness_class: locator.declared_freshness_class,
            signer_continuity_class: locator.signer_continuity_class,
            trust_stage: plan.trust_stage,
            checkout_shape,
            expected_cost_band,
            credential_posture,
            interrupted_recovery,
            manual_followups,
            evidence_packet,
            honesty_labels,
            guardrails,
        })
    }

    /// True when a surface MUST render this acquisition as something other
    /// than a plain live local copy: it is remote-acquired, partial/sparse,
    /// mirror-served, interrupted, or has remaining manual setup.
    pub fn surface_must_disclose_acquisition(&self) -> bool {
        self.acquisition_verb.is_remote_acquisition()
            || self.checkout_shape.partial_or_sparse
            || self.declared_freshness_class.is_not_live_origin()
            || self.interrupted_recovery.is_some()
            || !self.manual_followups.is_empty()
    }
}

fn transport_of(locator: &SourceLocatorRecord) -> Option<TransportClass> {
    if let Some(host) = locator.host_endpoint.as_ref() {
        return Some(host.transport_class);
    }
    if let Some(live) = locator.live_session_descriptor.as_ref() {
        if let Some(host) = live.host_endpoint.as_ref() {
            return Some(host.transport_class);
        }
    }
    if locator.locator_class.is_local_only() {
        return Some(TransportClass::LocalFilesystem);
    }
    None
}

fn resolve_acquisition_verb(locator: &SourceLocatorRecord) -> AcquisitionVerb {
    match locator.locator_class {
        LocatorClass::LocalFolder
        | LocatorClass::LocalFile
        | LocatorClass::WorkspaceFileManifest
        | LocatorClass::WorksetManifest => AcquisitionVerb::OpenLocal,
        LocatorClass::RepoUrl | LocatorClass::MirrorOrProxyRepo => AcquisitionVerb::Clone,
        LocatorClass::SnapshotArchive => AcquisitionVerb::OpenArchive,
        LocatorClass::HandoffPacket | LocatorClass::PortableStatePackage => AcquisitionVerb::Import,
        LocatorClass::Template | LocatorClass::PrebuildSnapshot => {
            AcquisitionVerb::OpenTemplateOrPrebuild
        }
        LocatorClass::LiveResumeTarget | LocatorClass::RecoveryCheckpoint => {
            AcquisitionVerb::Resume
        }
        LocatorClass::ReviewOrWorkItemDeepLink => AcquisitionVerb::OpenDeepLink,
    }
}

fn has_marker(plan: &CheckoutPlanRecord, marker: TopologyMarkerClass) -> bool {
    plan.topology_markers
        .iter()
        .any(|m| m.marker_class == marker)
}

fn resolve_submodule_policy(plan: &CheckoutPlanRecord) -> SubmodulePolicyClass {
    if has_marker(plan, TopologyMarkerClass::SubmoduleInitFailed) {
        SubmodulePolicyClass::InitFailed
    } else if has_marker(plan, TopologyMarkerClass::SubmoduleInitPending) {
        SubmodulePolicyClass::InitPending
    } else if has_marker(plan, TopologyMarkerClass::SubmoduleInitPartial) {
        SubmodulePolicyClass::InitPartial
    } else if has_marker(plan, TopologyMarkerClass::SubmoduleInitComplete) {
        SubmodulePolicyClass::InitComplete
    } else {
        SubmodulePolicyClass::NotPresent
    }
}

fn resolve_lfs_policy(plan: &CheckoutPlanRecord) -> LfsPolicyClass {
    if has_marker(plan, TopologyMarkerClass::LfsHydrateFailed) {
        LfsPolicyClass::HydrateFailed
    } else if has_marker(plan, TopologyMarkerClass::LfsPointerOnly) {
        LfsPolicyClass::PointerOnly
    } else if has_marker(plan, TopologyMarkerClass::LfsHydratePending) {
        LfsPolicyClass::HydratePending
    } else if has_marker(plan, TopologyMarkerClass::LfsHydratePartial) {
        LfsPolicyClass::HydratePartial
    } else if has_marker(plan, TopologyMarkerClass::LfsHydrateComplete) {
        LfsPolicyClass::HydrateComplete
    } else {
        LfsPolicyClass::NotPresent
    }
}

fn resolve_checkout_shape(
    locator: &SourceLocatorRecord,
    plan: &CheckoutPlanRecord,
) -> CheckoutShape {
    let partial_clone = has_marker(plan, TopologyMarkerClass::PartialCloneFilterPresent)
        || has_marker(plan, TopologyMarkerClass::PromisorRemoteRequired);
    let sparse = has_marker(plan, TopologyMarkerClass::SparseWorksetPresent);
    let shallow = has_marker(plan, TopologyMarkerClass::ShallowHistoryPresent);
    let read_only_partial_present = !plan.read_only_partial_roots.is_empty();

    let mode = if matches!(locator.locator_class, LocatorClass::LiveResumeTarget) {
        CheckoutModeClass::LiveAttach
    } else if matches!(
        locator.locator_class,
        LocatorClass::ReviewOrWorkItemDeepLink
    ) {
        CheckoutModeClass::NotApplicable
    } else if matches!(
        locator.locator_class,
        LocatorClass::SnapshotArchive
            | LocatorClass::HandoffPacket
            | LocatorClass::PortableStatePackage
    ) {
        CheckoutModeClass::ArchiveExtract
    } else if partial_clone {
        CheckoutModeClass::PartialClone
    } else if sparse {
        CheckoutModeClass::SparseCheckout
    } else if shallow {
        CheckoutModeClass::ShallowHistory
    } else {
        CheckoutModeClass::FullCheckout
    };

    let partial_or_sparse = partial_clone
        || sparse
        || shallow
        || matches!(
            locator.acquisition_posture,
            AcquisitionPosture::PartiallyAcquired | AcquisitionPosture::FilteredOrSparse
        )
        || read_only_partial_present;

    CheckoutShape {
        mode,
        partial_or_sparse,
        read_only_partial_present,
        submodule_policy: resolve_submodule_policy(plan),
        lfs_policy: resolve_lfs_policy(plan),
        read_only_partial_roots: plan.read_only_partial_roots.clone(),
    }
}

fn resolve_cost_band(
    locator: &SourceLocatorRecord,
    shape: &CheckoutShape,
    bootstrap_items: &[BootstrapQueueItemRecord],
    verb: AcquisitionVerb,
) -> ExpectedCostBand {
    if matches!(verb, AcquisitionVerb::Resume)
        && matches!(locator.locator_class, LocatorClass::LiveResumeTarget)
    {
        return ExpectedCostBand::LiveAttach;
    }

    let any_network_item = bootstrap_items.iter().any(|item| {
        matches!(
            item.execution_class,
            BootstrapExecutionClass::NetworkRequired
        ) && item.state.is_remaining()
    });
    let any_large_hydrate = matches!(
        shape.lfs_policy,
        LfsPolicyClass::PointerOnly
            | LfsPolicyClass::HydratePending
            | LfsPolicyClass::HydratePartial
    ) || matches!(shape.mode, CheckoutModeClass::PartialClone);

    if any_large_hydrate || (any_network_item && shape.partial_or_sparse) {
        return ExpectedCostBand::LargeFetchOrHydrate;
    }

    let transport = transport_of(locator);
    let non_network_transport = match transport {
        Some(t) => !t.is_network_bearing(),
        None => true,
    };
    // Snapshot archives, handoff packets, and portable-state packages arrive
    // as bytes (file upload / offline media), not as a network fetch.
    let on_disk_artifact = matches!(
        locator.locator_class,
        LocatorClass::SnapshotArchive
            | LocatorClass::HandoffPacket
            | LocatorClass::PortableStatePackage
    );
    let local_or_offline = locator.locator_class.is_local_only()
        || matches!(
            transport,
            Some(TransportClass::LocalFilesystem)
                | Some(TransportClass::FileUpload)
                | Some(TransportClass::AirGappedMedia)
        )
        || (on_disk_artifact && non_network_transport);

    if local_or_offline && !any_network_item {
        return ExpectedCostBand::LocalNoFetch;
    }

    if any_network_item
        || transport.is_some_and(TransportClass::is_network_bearing)
            && matches!(
                locator.acquisition_posture,
                AcquisitionPosture::NotYetAcquired | AcquisitionPosture::PartiallyAcquired
            )
    {
        return ExpectedCostBand::MeteredFetch;
    }

    if matches!(
        locator.acquisition_posture,
        AcquisitionPosture::AlreadyOnDisk
    ) {
        return ExpectedCostBand::LocalNoFetch;
    }

    ExpectedCostBand::Unknown
}

fn resolve_credential_posture(
    locator: &SourceLocatorRecord,
    plan: &CheckoutPlanRecord,
    bootstrap_items: &[BootstrapQueueItemRecord],
) -> BootstrapCredentialPosture {
    let host = locator.host_endpoint.as_ref().or_else(|| {
        locator
            .live_session_descriptor
            .as_ref()
            .and_then(|s| s.host_endpoint.as_ref())
    });
    let auth_mode = host.map(|h| h.auth_mode);
    let credential_handle_ref = host.and_then(|h| h.credential_handle_ref.clone());
    let approval_ticket_ref = host.and_then(|h| h.approval_ticket_ref.clone());

    let live_reauth = locator
        .live_session_descriptor
        .as_ref()
        .is_some_and(|s| s.attach_authority_class.requires_reauth());
    let item_reauth = bootstrap_items.iter().any(|item| {
        matches!(
            item.blocker,
            Some(super::descriptors::BlockerClass::AuthExpired)
                | Some(super::descriptors::BlockerClass::AuthorityRevoked)
        )
    });
    let reauth_required = matches!(plan.trust_stage, CheckoutTrustStage::ReauthRequired)
        || live_reauth
        || item_reauth;

    let posture_class = if reauth_required {
        CredentialPostureClass::ReauthRequired
    } else {
        match auth_mode {
            None => {
                if locator.locator_class.is_local_only() {
                    CredentialPostureClass::NoCredentialsNeeded
                } else {
                    CredentialPostureClass::Unknown
                }
            }
            Some(AuthModeClass::Anonymous) | Some(AuthModeClass::None) => {
                CredentialPostureClass::NoCredentialsNeeded
            }
            Some(AuthModeClass::InheritLocalIdentity) => {
                CredentialPostureClass::LocalIdentityInherited
            }
            Some(AuthModeClass::OauthHandle) | Some(AuthModeClass::DeviceCodeHandle) => {
                CredentialPostureClass::BrowserOrDeviceHandoffPending
            }
            Some(AuthModeClass::ManagedSessionTicket)
            | Some(AuthModeClass::ConnectedProviderTicket) => {
                CredentialPostureClass::ProviderTicketRequired
            }
            Some(AuthModeClass::SshAgent) | Some(AuthModeClass::PatHandle) => {
                CredentialPostureClass::HandlePresentNotYetUsed
            }
            Some(AuthModeClass::Other) => CredentialPostureClass::Unknown,
        }
    };

    BootstrapCredentialPosture {
        posture_class,
        auth_mode,
        credential_handle_ref,
        approval_ticket_ref,
        reauth_required,
    }
}

fn resolve_interrupted_recovery(plan: &CheckoutPlanRecord) -> Option<InterruptedRecovery> {
    let resume = &plan.resumable_acquisition;
    if !resume.resume_state.is_interrupted() {
        return None;
    }

    let open_read_only_available = resume.open_read_only_available.unwrap_or(false)
        || matches!(
            resume.resume_state,
            AcquisitionResumeState::InterruptedOpenReadOnlyAvailable
        )
        || !plan.read_only_partial_roots.is_empty();

    let mut branches: Vec<InterruptedRecoveryBranch> = Vec::new();
    for hook in &plan.next_step_decision_hooks {
        let branch = match hook {
            NextStepDecisionHook::ResumeAcquisition => {
                Some(InterruptedRecoveryBranch::ResumeAcquisition)
            }
            NextStepDecisionHook::DiscardAndRestart => {
                Some(InterruptedRecoveryBranch::DiscardAndRestart)
            }
            NextStepDecisionHook::OpenReadOnlyPartial => {
                Some(InterruptedRecoveryBranch::OpenReadOnlyPartial)
            }
            NextStepDecisionHook::RefreshMirror => Some(InterruptedRecoveryBranch::RefreshMirror),
            NextStepDecisionHook::SwitchToLiveOrigin => {
                Some(InterruptedRecoveryBranch::SwitchToLiveOrigin)
            }
            _ => None,
        };
        if let Some(branch) = branch {
            push_unique(&mut branches, branch);
        }
    }

    // Guarantee the contract minimum: an interrupted plan always exposes at
    // least one explicit branch even if the plan under-specified its hooks.
    if branches.is_empty() {
        match resume.resume_state {
            AcquisitionResumeState::InterruptedResumable => {
                branches.push(InterruptedRecoveryBranch::ResumeAcquisition)
            }
            AcquisitionResumeState::InterruptedDiscardRequired => {
                branches.push(InterruptedRecoveryBranch::DiscardAndRestart)
            }
            AcquisitionResumeState::InterruptedOpenReadOnlyAvailable => {
                branches.push(InterruptedRecoveryBranch::OpenReadOnlyPartial)
            }
            _ => {}
        }
    }
    if open_read_only_available {
        push_unique(
            &mut branches,
            InterruptedRecoveryBranch::OpenReadOnlyPartial,
        );
    }

    Some(InterruptedRecovery {
        resume_state: resume.resume_state,
        discard_posture: resume.discard_posture,
        open_read_only_available,
        branches,
        resume_checkpoint_ref: resume.resume_checkpoint_ref.clone(),
        export_safe: true,
    })
}

fn resolve_manual_followups(bootstrap_items: &[BootstrapQueueItemRecord]) -> Vec<ManualFollowup> {
    bootstrap_items
        .iter()
        .filter(|item| item.state.is_remaining())
        .map(|item| ManualFollowup {
            bootstrap_item_ref: item.bootstrap_item_id.clone(),
            item_class: item.item_class,
            state: item.state,
            execution_class: item.execution_class,
            absence_class: item.absence_class,
            presentation_label: item.presentation_label.clone(),
        })
        .collect()
}

fn resolve_evidence_packet(
    locator: &SourceLocatorRecord,
    plan: &CheckoutPlanRecord,
    bootstrap_items: &[BootstrapQueueItemRecord],
    manual_followup_count: u64,
) -> BootstrapEvidencePacket {
    let mut evidence_classes: Vec<AttributableEvidenceClass> = Vec::new();
    let mut every_item_attributed = true;
    for item in bootstrap_items {
        if item.attributable_evidence.is_empty() {
            every_item_attributed = false;
        }
        for evidence in &item.attributable_evidence {
            push_unique(&mut evidence_classes, evidence.evidence_class);
        }
    }

    BootstrapEvidencePacket {
        source_locator_ref: locator.source_locator_id.clone(),
        checkout_plan_ref: plan.checkout_plan_id.clone(),
        bootstrap_item_refs: bootstrap_items
            .iter()
            .map(|item| item.bootstrap_item_id.clone())
            .collect(),
        evidence_classes,
        every_item_attributed,
        manual_followup_count,
        export_safe: true,
    }
}

fn resolve_honesty_labels(
    locator: &SourceLocatorRecord,
    plan: &CheckoutPlanRecord,
    shape: &CheckoutShape,
) -> Vec<AcquisitionHonestyLabel> {
    let mut labels: Vec<AcquisitionHonestyLabel> = Vec::new();

    match locator.declared_freshness_class {
        DeclaredFreshnessClass::MirrorLagged => {
            push_unique(&mut labels, AcquisitionHonestyLabel::MirrorLagged)
        }
        DeclaredFreshnessClass::MirrorStale => {
            push_unique(&mut labels, AcquisitionHonestyLabel::MirrorStale)
        }
        DeclaredFreshnessClass::OfflineSnapshot => {
            push_unique(&mut labels, AcquisitionHonestyLabel::OfflineSnapshot)
        }
        DeclaredFreshnessClass::SignedOfflineBundle => {
            push_unique(&mut labels, AcquisitionHonestyLabel::SignedOfflineBundle)
        }
        _ => {}
    }

    if plan
        .mirror_freshness
        .as_ref()
        .and_then(|m| m.upstream_delta_class)
        == Some(UpstreamDeltaClass::DeltaOutsideDeclaredSkew)
    {
        push_unique(
            &mut labels,
            AcquisitionHonestyLabel::UpstreamDeltaOutsideSkew,
        );
    }

    match locator.signer_continuity_class {
        SignerContinuityClass::SignerChangedReviewRequired
        | SignerContinuityClass::SignatureMismatch => push_unique(
            &mut labels,
            AcquisitionHonestyLabel::SignerChangedReviewRequired,
        ),
        SignerContinuityClass::NewSignerFirstSeen
        | SignerContinuityClass::SignerChangedTrustOnFirstUse => {
            push_unique(&mut labels, AcquisitionHonestyLabel::SignerFirstSeen)
        }
        _ => {}
    }

    if shape.read_only_partial_present {
        push_unique(&mut labels, AcquisitionHonestyLabel::ReadOnlyPartial);
    }
    if has_marker(plan, TopologyMarkerClass::ShallowHistoryPresent) {
        push_unique(&mut labels, AcquisitionHonestyLabel::ShallowHistory);
    }
    if matches!(shape.mode, CheckoutModeClass::PartialClone) {
        push_unique(&mut labels, AcquisitionHonestyLabel::PartialClone);
    }
    if has_marker(plan, TopologyMarkerClass::SparseWorksetPresent) {
        push_unique(&mut labels, AcquisitionHonestyLabel::SparseWorkset);
    }
    if has_marker(plan, TopologyMarkerClass::SubmoduleInitPending) {
        push_unique(&mut labels, AcquisitionHonestyLabel::SubmoduleInitPending);
    }
    if has_marker(plan, TopologyMarkerClass::LfsPointerOnly) {
        push_unique(&mut labels, AcquisitionHonestyLabel::LfsPointerOnly);
    }

    match plan.trust_stage {
        CheckoutTrustStage::ReauthRequired => {
            push_unique(&mut labels, AcquisitionHonestyLabel::ReauthRequired)
        }
        CheckoutTrustStage::ReconnectRequired => {
            push_unique(&mut labels, AcquisitionHonestyLabel::ReconnectRequired)
        }
        _ => {}
    }

    if !plan.policy_narrowing_refs.is_empty() {
        push_unique(&mut labels, AcquisitionHonestyLabel::PolicyNarrowed);
    }

    labels
}

fn resolve_guardrails(
    locator: &SourceLocatorRecord,
    plan: &CheckoutPlanRecord,
    bootstrap_items: &[BootstrapQueueItemRecord],
    verb: AcquisitionVerb,
    evidence_packet: &BootstrapEvidencePacket,
) -> AcquisitionGuardrails {
    // Clone is never confused with open-local-copy: the clone verb only
    // resolves for remote / mirror locators, and open-local never resolves
    // for a remote one.
    let clone_not_confused_with_open = match verb {
        AcquisitionVerb::Clone => matches!(
            locator.locator_class,
            LocatorClass::RepoUrl | LocatorClass::MirrorOrProxyRepo
        ),
        AcquisitionVerb::OpenLocal => locator.locator_class.is_local_only(),
        _ => true,
    };

    // No repository-owned code path runs implicitly: the plan blocks at
    // least one repo-owned code path while trust is unadmitted, and no
    // side-effecting queue item has already started or completed before
    // admission.
    let plan_blocks_repo_code = plan
        .blocked_execution_paths
        .iter()
        .any(|path| path.is_repo_owned_code());
    let no_premature_side_effect = if plan.trust_stage.is_admitted() {
        true
    } else {
        !bootstrap_items.iter().any(|item| {
            item.execution_class.runs_side_effects()
                && matches!(
                    item.state,
                    BootstrapItemState::Running | BootstrapItemState::Succeeded
                )
        })
    };
    let no_implicit_repo_code_execution = plan_blocks_repo_code
        && no_premature_side_effect
        && !plan.blocked_execution_paths.is_empty();

    let bootstrap_items_attributed =
        bootstrap_items.is_empty() || evidence_packet.every_item_attributed;

    let browse_safe_inspection_available = !plan.browse_safe_actions.is_empty();

    // Mirror / proxy / lagged / stale / offline source never masquerades as
    // a live upstream fetch: such a source must declare a non-live freshness
    // class and (for mirror/proxy transport) carry mirror-freshness
    // evidence.
    let mirror_not_masquerading_as_live = mirror_honesty_holds(locator, plan);

    // Trust is not silently elevated: a signer change that requires review
    // never lands on an admitted stage without a review hook, and reauth /
    // reconnect stages surface their typed hook.
    let no_hidden_trust_elevation = trust_honesty_holds(locator, plan);

    AcquisitionGuardrails {
        clone_not_confused_with_open,
        no_implicit_repo_code_execution,
        bootstrap_items_attributed,
        browse_safe_inspection_available,
        mirror_not_masquerading_as_live,
        no_hidden_trust_elevation,
    }
}

fn mirror_honesty_holds(locator: &SourceLocatorRecord, plan: &CheckoutPlanRecord) -> bool {
    let transport = transport_of(locator);
    let mirror_transport = transport.is_some_and(TransportClass::is_mirror_or_proxy)
        || matches!(locator.locator_class, LocatorClass::MirrorOrProxyRepo);

    if mirror_transport {
        // A mirror / proxy acquisition must not declare live origin and must
        // carry mirror-freshness evidence rather than pretend to be live.
        if matches!(
            locator.declared_freshness_class,
            DeclaredFreshnessClass::LiveOrigin
        ) {
            return false;
        }
        if plan.mirror_freshness.is_none() {
            return false;
        }
    }

    // A lagged / stale / offline freshness class must not be silently
    // rewritten to a fresh class on the plan's mirror-freshness evidence.
    if let Some(mirror) = plan.mirror_freshness.as_ref() {
        if locator.declared_freshness_class.is_lagged_or_stale()
            && matches!(
                mirror.freshness_class,
                super::descriptors::MirrorFreshnessClass::LiveOrigin
                    | super::descriptors::MirrorFreshnessClass::MirrorFresh
            )
        {
            return false;
        }
    }

    true
}

fn trust_honesty_holds(locator: &SourceLocatorRecord, plan: &CheckoutPlanRecord) -> bool {
    // A signer change that requires review must not sit on an admitted stage
    // without routing to the review hook.
    if locator.signer_continuity_class.requires_signer_review() && plan.trust_stage.is_admitted() {
        let has_review_hook = plan
            .next_step_decision_hooks
            .iter()
            .any(|h| matches!(h, NextStepDecisionHook::ReviewSignerChange));
        if !has_review_hook {
            return false;
        }
    }

    // Reauth / reconnect stages must surface their typed next-step hook
    // rather than silently admit.
    match plan.trust_stage {
        CheckoutTrustStage::ReauthRequired => plan
            .next_step_decision_hooks
            .iter()
            .any(|h| matches!(h, NextStepDecisionHook::ReauthRequired)),
        CheckoutTrustStage::ReconnectRequired => plan
            .next_step_decision_hooks
            .iter()
            .any(|h| matches!(h, NextStepDecisionHook::ReconnectRequired)),
        CheckoutTrustStage::SignerReviewRequired => plan
            .next_step_decision_hooks
            .iter()
            .any(|h| matches!(h, NextStepDecisionHook::ReviewSignerChange)),
        _ => true,
    }
}

fn push_unique<T: PartialEq>(slot: &mut Vec<T>, value: T) {
    if !slot.contains(&value) {
        slot.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::super::descriptors::{
        AbsenceClass, AcquisitionResumeState, AttributableEvidence, AttributableEvidenceClass,
        AuthModeClass, BlockedExecutionPathClass, BootstrapExecutionClass, BootstrapItemClass,
        BootstrapItemState, BootstrapQueueItemRecord, BootstrapQueueItemRecordKind,
        BrowseSafeActionClass, CheckoutPlanRecord, CheckoutPlanRecordKind, CheckoutTrustStage,
        CheckoutTrustState, DeclaredFreshnessClass, DiscardPosture, HostEndpointDescriptor,
        LocatorEntryVerbHint, MirrorFreshnessClass, MirrorFreshnessEvidence, NextStepDecisionHook,
        ResumableAcquisitionState, SignerContinuityClass, SignerContinuityEvidence,
        SourceLocatorRecord, SourceLocatorRecordKind, TopologyMarker, TopologyMarkerClass,
        TransportClass, UpstreamDeltaClass, BOOTSTRAP_QUEUE_ITEM_SCHEMA_VERSION,
        CHECKOUT_PLAN_SCHEMA_VERSION, SOURCE_LOCATOR_SCHEMA_VERSION,
    };
    use super::*;

    fn now() -> String {
        "mono:1:00:00:00.0000".to_string()
    }

    fn clone_locator() -> SourceLocatorRecord {
        SourceLocatorRecord {
            schema: None,
            fixture: None,
            record_kind: SourceLocatorRecordKind::SourceLocatorRecord,
            source_locator_schema_version: SOURCE_LOCATOR_SCHEMA_VERSION,
            source_locator_id: "locator:clone".to_string(),
            locator_class: LocatorClass::RepoUrl,
            acquisition_posture: AcquisitionPosture::NotYetAcquired,
            declared_freshness_class: DeclaredFreshnessClass::LiveOrigin,
            signer_continuity_class: SignerContinuityClass::ContinuousWithPreviousAcquisition,
            presentation_label: None,
            presentation_subtitle: None,
            local_identity_ref: None,
            host_endpoint: Some(HostEndpointDescriptor {
                host_label: "github.com".to_string(),
                transport_class: TransportClass::Ssh,
                auth_mode: AuthModeClass::SshAgent,
                credential_handle_ref: Some("cred:ssh".to_string()),
                approval_ticket_ref: None,
                branch_or_ref: None,
                mirror_label: None,
                upstream_origin_label: None,
            }),
            artifact_descriptor: None,
            live_session_descriptor: None,
            deep_link_descriptor: None,
            entry_verb_hint: Some(LocatorEntryVerbHint::Clone),
            target_kind_hint: None,
            previous_acquisition_ref: None,
            observed_at: now(),
        }
    }

    fn clone_plan() -> CheckoutPlanRecord {
        CheckoutPlanRecord {
            schema: None,
            fixture: None,
            record_kind: CheckoutPlanRecordKind::CheckoutPlanRecord,
            checkout_plan_schema_version: CHECKOUT_PLAN_SCHEMA_VERSION,
            checkout_plan_id: "plan:clone".to_string(),
            source_locator_ref: "locator:clone".to_string(),
            entry_action_ref: None,
            trust_state: CheckoutTrustState::PendingEvaluation,
            trust_stage: CheckoutTrustStage::PreFetchInspection,
            browse_safe_actions: vec![BrowseSafeActionClass::InspectManifest],
            blocked_execution_paths: vec![
                BlockedExecutionPathClass::PostCheckoutHook,
                BlockedExecutionPathClass::SubmoduleRecursiveInit,
            ],
            resumable_acquisition: ResumableAcquisitionState {
                resume_state: AcquisitionResumeState::NeverStarted,
                discard_posture: DiscardPosture::NoDiscardRequired,
                resume_checkpoint_ref: None,
                open_read_only_available: Some(false),
                failure_reason_class: None,
                user_visible_reason: None,
            },
            mirror_freshness: None,
            signer_continuity: SignerContinuityEvidence {
                continuity_class: SignerContinuityClass::ContinuousWithPreviousAcquisition,
                signer_identity_label: None,
                previous_signer_identity_label: None,
                previous_acquisition_ref: None,
                rotation_policy_ref: None,
                review_ticket_ref: None,
            },
            read_only_partial_roots: Vec::new(),
            topology_markers: vec![
                TopologyMarker {
                    marker_class: TopologyMarkerClass::SubmoduleInitPending,
                    detail_label: None,
                    pending_count: Some(3),
                    completed_count: None,
                    failed_count: None,
                    evidence_ref: None,
                },
                TopologyMarker {
                    marker_class: TopologyMarkerClass::LfsPointerOnly,
                    detail_label: None,
                    pending_count: None,
                    completed_count: None,
                    failed_count: None,
                    evidence_ref: None,
                },
            ],
            policy_narrowing_refs: Vec::new(),
            bootstrap_queue_ref: Some("queue:clone".to_string()),
            next_step_decision_hooks: vec![NextStepDecisionHook::ReviewTrustAndOpen],
            presentation_label: None,
            presentation_subtitle: None,
            emitted_at: now(),
        }
    }

    fn item(
        id: &str,
        class: BootstrapItemClass,
        exec: BootstrapExecutionClass,
        state: BootstrapItemState,
        absence: AbsenceClass,
    ) -> BootstrapQueueItemRecord {
        BootstrapQueueItemRecord {
            schema: None,
            fixture: None,
            record_kind: BootstrapQueueItemRecordKind::BootstrapQueueItemRecord,
            bootstrap_queue_item_schema_version: BOOTSTRAP_QUEUE_ITEM_SCHEMA_VERSION,
            bootstrap_item_id: id.to_string(),
            checkout_plan_ref: "plan:clone".to_string(),
            source_locator_ref: "locator:clone".to_string(),
            queue_position: 0,
            parallel_group_id: None,
            depends_on_item_ids: Vec::new(),
            item_class: class,
            execution_class: exec,
            state,
            absence_class: absence,
            skip_reason: None,
            blocker: None,
            side_effect_envelope: None,
            attributable_evidence: vec![AttributableEvidence {
                evidence_class: AttributableEvidenceClass::CheckoutPlanReference,
                evidence_ref: Some("plan:clone".to_string()),
                summary_label: None,
            }],
            topology_marker_refs: Vec::new(),
            repair_hooks: Vec::new(),
            user_visible_reason: None,
            presentation_label: None,
            started_at: None,
            ended_at: None,
            emitted_at: now(),
        }
    }

    #[test]
    fn clone_projects_distinct_verb_and_shape() {
        let locator = clone_locator();
        let plan = clone_plan();
        let items = vec![item(
            "boot:submodule",
            BootstrapItemClass::SubmoduleInit,
            BootstrapExecutionClass::DeferredUntilTrustAdmitted,
            BootstrapItemState::AwaitingAdmission,
            AbsenceClass::NotYetFetched,
        )];

        let projection =
            RepositoryAcquisitionBetaProjection::project(RepositoryAcquisitionBetaInputs {
                locator: &locator,
                plan: &plan,
                bootstrap_items: &items,
                surface: AcquisitionSurface::StartCenter,
            })
            .expect("must project");

        assert_eq!(projection.acquisition_verb, AcquisitionVerb::Clone);
        assert_eq!(
            projection.checkout_shape.submodule_policy,
            SubmodulePolicyClass::InitPending
        );
        assert_eq!(
            projection.checkout_shape.lfs_policy,
            LfsPolicyClass::PointerOnly
        );
        assert_eq!(
            projection.expected_cost_band,
            ExpectedCostBand::LargeFetchOrHydrate
        );
        assert_eq!(projection.manual_followups.len(), 1);
        assert!(projection.guardrails.all_hold());
        assert!(projection.surface_must_disclose_acquisition());
        assert!(projection
            .honesty_labels
            .contains(&AcquisitionHonestyLabel::SubmoduleInitPending));
    }

    #[test]
    fn plan_locator_mismatch_is_rejected() {
        let locator = clone_locator();
        let mut plan = clone_plan();
        plan.source_locator_ref = "locator:other".to_string();
        let err = RepositoryAcquisitionBetaProjection::project(RepositoryAcquisitionBetaInputs {
            locator: &locator,
            plan: &plan,
            bootstrap_items: &[],
            surface: AcquisitionSurface::CliHeadless,
        })
        .expect_err("mismatch must be rejected");
        assert!(matches!(
            err,
            RepositoryAcquisitionBetaError::PlanLocatorMismatch { .. }
        ));
    }

    #[test]
    fn mirror_clone_declaring_live_origin_fails_guardrail() {
        let mut locator = clone_locator();
        locator.locator_class = LocatorClass::MirrorOrProxyRepo;
        if let Some(host) = locator.host_endpoint.as_mut() {
            host.transport_class = TransportClass::Mirror;
        }
        // Declares live origin while served by a mirror: dishonest.
        locator.declared_freshness_class = DeclaredFreshnessClass::LiveOrigin;
        let plan = clone_plan();
        let projection =
            RepositoryAcquisitionBetaProjection::project(RepositoryAcquisitionBetaInputs {
                locator: &locator,
                plan: &plan,
                bootstrap_items: &[],
                surface: AcquisitionSurface::StartCenter,
            })
            .expect("must project");
        assert!(!projection.guardrails.mirror_not_masquerading_as_live);
    }

    #[test]
    fn lagged_mirror_with_delta_emits_honesty_labels() {
        let mut locator = clone_locator();
        locator.locator_class = LocatorClass::MirrorOrProxyRepo;
        if let Some(host) = locator.host_endpoint.as_mut() {
            host.transport_class = TransportClass::Mirror;
        }
        locator.declared_freshness_class = DeclaredFreshnessClass::MirrorLagged;
        let mut plan = clone_plan();
        plan.mirror_freshness = Some(MirrorFreshnessEvidence {
            freshness_class: MirrorFreshnessClass::MirrorLagged,
            upstream_delta_class: Some(UpstreamDeltaClass::DeltaOutsideDeclaredSkew),
            upstream_delta_summary: None,
            mirror_attestation_ref: None,
            upstream_origin_label: None,
            mirror_label: None,
            measured_at: None,
        });
        let projection =
            RepositoryAcquisitionBetaProjection::project(RepositoryAcquisitionBetaInputs {
                locator: &locator,
                plan: &plan,
                bootstrap_items: &[],
                surface: AcquisitionSurface::StartCenter,
            })
            .expect("must project");
        assert!(projection.guardrails.mirror_not_masquerading_as_live);
        assert!(projection
            .honesty_labels
            .contains(&AcquisitionHonestyLabel::MirrorLagged));
        assert!(projection
            .honesty_labels
            .contains(&AcquisitionHonestyLabel::UpstreamDeltaOutsideSkew));
    }

    #[test]
    fn interrupted_plan_exposes_explicit_branches() {
        let locator = clone_locator();
        let mut plan = clone_plan();
        plan.trust_stage = CheckoutTrustStage::PostFetchContentReview;
        plan.resumable_acquisition = ResumableAcquisitionState {
            resume_state: AcquisitionResumeState::InterruptedResumable,
            discard_posture: DiscardPosture::DiscardStagingOnly,
            resume_checkpoint_ref: Some("ckpt:1".to_string()),
            open_read_only_available: Some(true),
            failure_reason_class: Some(
                super::super::descriptors::AcquisitionFailureReasonClass::NetworkInterruption,
            ),
            user_visible_reason: None,
        };
        plan.next_step_decision_hooks = vec![
            NextStepDecisionHook::ResumeAcquisition,
            NextStepDecisionHook::OpenReadOnlyPartial,
            NextStepDecisionHook::DiscardAndRestart,
        ];
        let projection =
            RepositoryAcquisitionBetaProjection::project(RepositoryAcquisitionBetaInputs {
                locator: &locator,
                plan: &plan,
                bootstrap_items: &[],
                surface: AcquisitionSurface::StartCenter,
            })
            .expect("must project");
        let recovery = projection
            .interrupted_recovery
            .expect("must be interrupted");
        assert!(recovery.open_read_only_available);
        assert!(recovery
            .branches
            .contains(&InterruptedRecoveryBranch::ResumeAcquisition));
        assert!(recovery
            .branches
            .contains(&InterruptedRecoveryBranch::DiscardAndRestart));
        assert!(recovery
            .branches
            .contains(&InterruptedRecoveryBranch::OpenReadOnlyPartial));
        assert!(recovery.export_safe);
    }

    #[test]
    fn missing_evidence_is_rejected() {
        let locator = clone_locator();
        let plan = clone_plan();
        let mut bad = item(
            "boot:bad",
            BootstrapItemClass::PackageRestore,
            BootstrapExecutionClass::NetworkRequired,
            BootstrapItemState::Pending,
            AbsenceClass::NotYetFetched,
        );
        bad.attributable_evidence.clear();
        let err = RepositoryAcquisitionBetaProjection::project(RepositoryAcquisitionBetaInputs {
            locator: &locator,
            plan: &plan,
            bootstrap_items: std::slice::from_ref(&bad),
            surface: AcquisitionSurface::Support,
        })
        .expect_err("missing evidence must be rejected");
        assert!(matches!(
            err,
            RepositoryAcquisitionBetaError::BootstrapItemEvidenceMissing { .. }
        ));
    }
}
